//! Generic MS-NRBF (.NET BinaryFormatter) decoder.
//!
//! Reads a stream into an object graph keyed by object id, with class and
//! member names preserved. It knows nothing about Spirit Island: `engine_map`
//! walks the graph and picks out the fields it needs.
//!
//! Spec: [MS-NRBF] .NET Remoting: Binary Format Data Structure, sections
//! 2.1–2.7. Only the record types BinaryFormatter emits for plain object
//! graphs are supported; MethodCall/MethodReturn (remoting) are rejected.

use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NrbfError {
    #[error("unexpected end of stream at offset {0}")]
    Eof(usize),
    #[error("unsupported record type {0} at offset {1}")]
    UnsupportedRecord(u8, usize),
    #[error("unsupported primitive type {0} at offset {1}")]
    UnsupportedPrimitive(u8, usize),
    #[error("ClassWithId references unknown metadata id {0}")]
    UnknownMetadata(i32),
    #[error("invalid utf-8 string at offset {0}")]
    Utf8(usize),
    #[error("stream does not start with a SerializedStreamHeader")]
    NoHeader,
}

/// A member or array element value. Objects are always `Ref`s into the graph.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    U8(u8),
    I8(i8),
    Char(char),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Decimal(Rc<str>),
    TimeSpan(i64),
    DateTime(u64),
    Ref(i32),
}

impl Value {
    pub fn as_i64(&self) -> Option<i64> {
        Some(match *self {
            Value::U8(v) => v as i64,
            Value::I8(v) => v as i64,
            Value::I16(v) => v as i64,
            Value::U16(v) => v as i64,
            Value::I32(v) => v as i64,
            Value::U32(v) => v as i64,
            Value::I64(v) => v,
            Value::U64(v) => v as i64,
            _ => return None,
        })
    }
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
    pub fn as_ref_id(&self) -> Option<i32> {
        match *self {
            Value::Ref(id) => Some(id),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ClassMeta {
    pub name: String,
    pub library: Option<i32>,
    pub members: Vec<String>,
    member_types: Vec<MemberType>,
}

#[derive(Debug, Clone)]
enum MemberType {
    Primitive(u8),
    String,
    Object,
    SystemClass,
    Class,
    ObjectArray,
    StringArray,
    PrimitiveArray,
}

#[derive(Debug)]
pub enum Object {
    Class {
        meta: Rc<ClassMeta>,
        values: Vec<Value>,
    },
    String(Rc<str>),
    Array {
        /// Product of all dimension lengths; rank kept for completeness.
        lengths: Vec<i32>,
        values: Vec<Value>,
    },
}

/// The decoded stream: every object by id, plus the root id.
#[derive(Debug)]
pub struct Graph {
    pub root: i32,
    pub objects: HashMap<i32, Object>,
    pub libraries: HashMap<i32, String>,
}

impl Graph {
    pub fn get(&self, id: i32) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn root(&self) -> Node<'_> {
        Node { graph: self, id: self.root }
    }

    pub fn node(&self, id: i32) -> Node<'_> {
        Node { graph: self, id }
    }

    /// Count objects by class name. Diagnostic only.
    pub fn class_histogram(&self) -> Vec<(String, usize)> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for o in self.objects.values() {
            if let Object::Class { meta, .. } = o {
                *counts.entry(meta.name.as_str()).or_default() += 1;
            }
        }
        let mut out: Vec<_> = counts.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        out
    }
}

/// A cursor on one object of the graph, with name-tolerant field access.
#[derive(Clone, Copy)]
pub struct Node<'g> {
    pub graph: &'g Graph,
    pub id: i32,
}

impl<'g> Node<'g> {
    pub fn object(&self) -> Option<&'g Object> {
        self.graph.get(self.id)
    }

    /// Short class name: `Handelabra.SpiritIsland.Engine.Model.Land` → `Land`,
    /// generic arguments stripped.
    pub fn class_name(&self) -> Option<&'g str> {
        match self.object()? {
            Object::Class { meta, .. } => Some(short_name(&meta.name)),
            _ => None,
        }
    }

    pub fn full_class_name(&self) -> Option<&'g str> {
        match self.object()? {
            Object::Class { meta, .. } => Some(meta.name.as_str()),
            _ => None,
        }
    }

    /// Raw member value. Matches `name`, `<name>k__BackingField`, `_name`,
    /// and `Base+<name>k__BackingField` (inherited members are prefixed with
    /// the declaring class). A subclass can re-declare a member it inherits
    /// (`Land._defense` and `IslandLocation+_defense`); the first non-null
    /// match wins.
    pub fn raw(&self, name: &str) -> Option<&'g Value> {
        let Object::Class { meta, values } = self.object()? else { return None };
        let backing = format!("<{name}>k__BackingField");
        let underscore = format!("_{name}");
        let mut first = None;
        for (m, v) in meta.members.iter().zip(values) {
            let m = m.rsplit('+').next().unwrap_or(m);
            if m == name || m == backing || m == underscore {
                if *v != Value::Null {
                    return Some(v);
                }
                first.get_or_insert(v);
            }
        }
        first
    }

    pub fn has(&self, name: &str) -> bool {
        self.raw(name).is_some()
    }

    pub fn child(&self, name: &str) -> Option<Node<'g>> {
        let id = self.raw(name)?.as_ref_id()?;
        Some(self.graph.node(id))
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        match self.raw(name)? {
            Value::Ref(id) => self.graph.node(*id).boxed_int(),
            v => v.as_i64(),
        }
    }

    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.raw(name)? {
            Value::Ref(id) => self.graph.node(*id).raw("m_value")?.as_bool(),
            v => v.as_bool(),
        }
    }

    pub fn str(&self, name: &str) -> Option<&'g str> {
        self.child(name)?.as_str()
    }

    pub fn as_str(&self) -> Option<&'g str> {
        match self.object()? {
            Object::String(s) => Some(s),
            _ => None,
        }
    }

    /// Boxed primitives (`System.Int32` with `m_value`) and enums boxed as
    /// objects (`value__`).
    pub fn boxed_int(&self) -> Option<i64> {
        self.raw("m_value").or_else(|| self.raw("value__"))?.as_i64()
    }

    /// Elements of a `List<T>`, `Stack<T>`, `HashSet<T>`, or a raw array.
    /// Unused capacity slots past `_size` are dropped.
    pub fn items(&self) -> Vec<Value> {
        match self.object() {
            Some(Object::Array { values, .. }) => values.clone(),
            Some(Object::Class { .. }) => {
                let arr = self.child("_items").or_else(|| self.child("_array")).or_else(|| self.child("Elements"));
                let Some(arr) = arr else { return Vec::new() };
                let mut vals = arr.items();
                if let Some(size) = self.raw("_size").and_then(|v| v.as_i64()) {
                    vals.truncate(size.max(0) as usize);
                }
                vals
            }
            _ => Vec::new(),
        }
    }

    pub fn item_nodes(&self) -> Vec<Node<'g>> {
        self.items().into_iter().filter_map(|v| v.as_ref_id()).map(|id| self.graph.node(id)).collect()
    }

    /// `Dictionary<K,V>` entries as (key, value) pairs.
    pub fn dict_entries(&self) -> Vec<(Value, Value)> {
        let Some(kvs) = self.child("KeyValuePairs") else { return Vec::new() };
        kvs.item_nodes()
            .into_iter()
            .filter_map(|kv| Some((kv.raw("key")?.clone(), kv.raw("value")?.clone())))
            .collect()
    }

    /// Member names of this class, in stream order.
    pub fn member_names(&self) -> Vec<&'g str> {
        match self.object() {
            Some(Object::Class { meta, .. }) => meta.members.iter().map(|s| s.as_str()).collect(),
            _ => Vec::new(),
        }
    }
}

pub fn short_name(full: &str) -> &str {
    let no_generic = full.split('`').next().unwrap_or(full);
    no_generic.rsplit(['.', '+']).next().unwrap_or(no_generic)
}

// ---------------------------------------------------------------- decoding

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], NrbfError> {
        if self.pos + n > self.buf.len() {
            return Err(NrbfError::Eof(self.pos));
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, NrbfError> {
        Ok(self.take(1)?[0])
    }
    fn i32(&mut self) -> Result<i32, NrbfError> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn arr<const N: usize>(&mut self) -> Result<[u8; N], NrbfError> {
        Ok(self.take(N)?.try_into().unwrap())
    }
    /// LengthPrefixedString: 7-bit varint length, then UTF-8.
    fn lps(&mut self) -> Result<String, NrbfError> {
        let mut len: usize = 0;
        let mut shift = 0;
        loop {
            let b = self.u8()?;
            len |= ((b & 0x7f) as usize) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        let at = self.pos;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| NrbfError::Utf8(at))
    }
    fn primitive(&mut self, ty: u8) -> Result<Value, NrbfError> {
        let at = self.pos;
        Ok(match ty {
            1 => Value::Bool(self.u8()? != 0),
            2 => Value::U8(self.u8()?),
            3 => {
                // One UTF-8 encoded character.
                let first = self.u8()?;
                let extra = match first {
                    0x00..=0x7f => 0,
                    0xc0..=0xdf => 1,
                    0xe0..=0xef => 2,
                    _ => 3,
                };
                let mut bytes = vec![first];
                bytes.extend_from_slice(self.take(extra)?);
                let s = std::str::from_utf8(&bytes).map_err(|_| NrbfError::Utf8(at))?;
                Value::Char(s.chars().next().unwrap_or('\0'))
            }
            5 => Value::Decimal(self.lps()?.into()),
            6 => Value::F64(f64::from_le_bytes(self.arr()?)),
            7 => Value::I16(i16::from_le_bytes(self.arr()?)),
            8 => Value::I32(self.i32()?),
            9 => Value::I64(i64::from_le_bytes(self.arr()?)),
            10 => Value::I8(self.u8()? as i8),
            11 => Value::F32(f32::from_le_bytes(self.arr()?)),
            12 => Value::TimeSpan(i64::from_le_bytes(self.arr()?)),
            13 => Value::DateTime(u64::from_le_bytes(self.arr()?)),
            14 => Value::U16(u16::from_le_bytes(self.arr()?)),
            15 => Value::U32(u32::from_le_bytes(self.arr()?)),
            16 => Value::U64(u64::from_le_bytes(self.arr()?)),
            17 => Value::Null,
            other => return Err(NrbfError::UnsupportedPrimitive(other, at)),
        })
    }
}

/// What one call to `record` produced.
enum Rec {
    Value(Value),
    /// ObjectNullMultiple*: this many consecutive nulls.
    Nulls(usize),
    End,
}

struct Decoder<'a> {
    r: Reader<'a>,
    objects: HashMap<i32, Object>,
    metas: HashMap<i32, Rc<ClassMeta>>,
    libraries: HashMap<i32, String>,
}

pub fn decode(buf: &[u8]) -> Result<Graph, NrbfError> {
    let mut d = Decoder {
        r: Reader { buf, pos: 0 },
        objects: HashMap::new(),
        metas: HashMap::new(),
        libraries: HashMap::new(),
    };
    if d.r.u8()? != 0 {
        return Err(NrbfError::NoHeader);
    }
    let root = d.r.i32()?;
    let _header_id = d.r.i32()?;
    let _major = d.r.i32()?;
    let _minor = d.r.i32()?;
    loop {
        match d.record()? {
            Rec::End => break,
            Rec::Value(_) | Rec::Nulls(_) => {}
        }
    }
    Ok(Graph { root, objects: d.objects, libraries: d.libraries })
}

impl<'a> Decoder<'a> {
    fn record(&mut self) -> Result<Rec, NrbfError> {
        loop {
            let at = self.r.pos;
            let ty = self.r.u8()?;
            return Ok(match ty {
                1 => {
                    let id = self.r.i32()?;
                    let meta_id = self.r.i32()?;
                    let meta = self.metas.get(&meta_id).cloned().ok_or(NrbfError::UnknownMetadata(meta_id))?;
                    self.metas.insert(id, meta.clone());
                    self.class_values(id, meta)?;
                    Rec::Value(Value::Ref(id))
                }
                4 | 5 => {
                    let (id, name, members) = self.class_info()?;
                    let mut kinds = Vec::with_capacity(members.len());
                    for _ in 0..members.len() {
                        kinds.push(self.r.u8()?);
                    }
                    let mut member_types = Vec::with_capacity(kinds.len());
                    for k in kinds {
                        member_types.push(match k {
                            0 => MemberType::Primitive(self.r.u8()?),
                            1 => MemberType::String,
                            2 => MemberType::Object,
                            3 => {
                                self.r.lps()?;
                                MemberType::SystemClass
                            }
                            4 => {
                                self.r.lps()?;
                                self.r.i32()?;
                                MemberType::Class
                            }
                            5 => MemberType::ObjectArray,
                            6 => MemberType::StringArray,
                            7 => {
                                self.r.u8()?;
                                MemberType::PrimitiveArray
                            }
                            other => return Err(NrbfError::UnsupportedPrimitive(other, self.r.pos)),
                        });
                    }
                    let library = if ty == 5 { Some(self.r.i32()?) } else { None };
                    let meta = Rc::new(ClassMeta { name, library, members, member_types });
                    self.metas.insert(id, meta.clone());
                    self.class_values(id, meta)?;
                    Rec::Value(Value::Ref(id))
                }
                6 => {
                    let id = self.r.i32()?;
                    let s = self.r.lps()?;
                    self.objects.insert(id, Object::String(s.into()));
                    Rec::Value(Value::Ref(id))
                }
                7 => {
                    let id = self.r.i32()?;
                    let array_type = self.r.u8()?;
                    let rank = self.r.i32()?.max(0) as usize;
                    let mut lengths = Vec::with_capacity(rank);
                    for _ in 0..rank {
                        lengths.push(self.r.i32()?);
                    }
                    if matches!(array_type, 3..=5) {
                        for _ in 0..rank {
                            self.r.i32()?;
                        }
                    }
                    let elem = self.r.u8()?;
                    let prim = match elem {
                        0 | 7 => Some(self.r.u8()?),
                        3 => {
                            self.r.lps()?;
                            None
                        }
                        4 => {
                            self.r.lps()?;
                            self.r.i32()?;
                            None
                        }
                        _ => None,
                    };
                    let count: usize = lengths.iter().map(|&l| l.max(0) as usize).product();
                    let values = if elem == 0 {
                        let p = prim.unwrap_or(8);
                        (0..count).map(|_| self.r.primitive(p)).collect::<Result<_, _>>()?
                    } else {
                        self.element_records(count)?
                    };
                    self.objects.insert(id, Object::Array { lengths, values });
                    Rec::Value(Value::Ref(id))
                }
                8 => {
                    let p = self.r.u8()?;
                    Rec::Value(self.r.primitive(p)?)
                }
                9 => Rec::Value(Value::Ref(self.r.i32()?)),
                10 => Rec::Value(Value::Null),
                11 => Rec::End,
                12 => {
                    let id = self.r.i32()?;
                    let name = self.r.lps()?;
                    self.libraries.insert(id, name);
                    continue;
                }
                13 => Rec::Nulls(self.r.u8()? as usize),
                14 => Rec::Nulls(self.r.i32()?.max(0) as usize),
                15 => {
                    let id = self.r.i32()?;
                    let len = self.r.i32()?.max(0) as usize;
                    let p = self.r.u8()?;
                    let values = if p == 2 {
                        // byte[]: keep as U8 values; small enough here.
                        self.r.take(len)?.iter().map(|&b| Value::U8(b)).collect()
                    } else {
                        (0..len).map(|_| self.r.primitive(p)).collect::<Result<_, _>>()?
                    };
                    self.objects.insert(id, Object::Array { lengths: vec![len as i32], values });
                    Rec::Value(Value::Ref(id))
                }
                16 | 17 => {
                    let id = self.r.i32()?;
                    let len = self.r.i32()?.max(0) as usize;
                    let values = self.element_records(len)?;
                    self.objects.insert(id, Object::Array { lengths: vec![len as i32], values });
                    Rec::Value(Value::Ref(id))
                }
                other => return Err(NrbfError::UnsupportedRecord(other, at)),
            });
        }
    }

    fn class_info(&mut self) -> Result<(i32, String, Vec<String>), NrbfError> {
        let id = self.r.i32()?;
        let name = self.r.lps()?;
        let n = self.r.i32()?.max(0) as usize;
        let mut members = Vec::with_capacity(n);
        for _ in 0..n {
            members.push(self.r.lps()?);
        }
        Ok((id, name, members))
    }

    fn class_values(&mut self, id: i32, meta: Rc<ClassMeta>) -> Result<(), NrbfError> {
        let mut values = Vec::with_capacity(meta.members.len());
        for mt in meta.member_types.iter() {
            let v = match mt {
                MemberType::Primitive(p) => self.r.primitive(*p)?,
                _ => match self.record()? {
                    Rec::Value(v) => v,
                    Rec::Nulls(_) => Value::Null,
                    Rec::End => return Err(NrbfError::Eof(self.r.pos)),
                },
            };
            values.push(v);
        }
        self.objects.insert(id, Object::Class { meta, values });
        Ok(())
    }

    fn element_records(&mut self, count: usize) -> Result<Vec<Value>, NrbfError> {
        let mut out = Vec::with_capacity(count);
        while out.len() < count {
            match self.record()? {
                Rec::Value(v) => out.push(v),
                Rec::Nulls(n) => out.extend(std::iter::repeat_n(Value::Null, n)),
                Rec::End => return Err(NrbfError::Eof(self.r.pos)),
            }
        }
        out.truncate(count);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_names() {
        assert_eq!(short_name("Handelabra.SpiritIsland.Engine.Model.Land"), "Land");
        assert_eq!(
            short_name("System.Collections.Generic.List`1[[Handelabra.SpiritIsland.Engine.Model.Card, SpiritIslandEngine]]"),
            "List"
        );
        assert_eq!(short_name("Outer+Inner"), "Inner");
    }

    #[test]
    fn lps_multi_byte_length() {
        let mut buf = vec![0x80 | 0x05, 0x01];
        buf.extend(std::iter::repeat_n(b'a', 133));
        let mut r = Reader { buf: &buf, pos: 0 };
        assert_eq!(r.lps().unwrap().len(), 133);
    }
}
