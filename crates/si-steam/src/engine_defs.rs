//! Engine definitions: the JSON resources embedded in `SpiritIslandEngine.dll`
//! (power cards, invader cards, boards, adversaries, spirits, …).
//!
//! These are read at runtime from Brett's own game install, read-only, and
//! never checked in. They are public card data (cost, speed, elements, text,
//! board adjacency), so the firewall does not apply to them.
//!
//! The DLL is a .NET PE file. Its CLI header (data directory 14) points at a
//! resources blob: a run of `[u32 length][bytes]` records. Resources are
//! identified by content rather than by parsing the ManifestResource table.

use anyhow::{Context, Result, bail};
use serde_json::Value as Json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct EngineDefs {
    pub engine_sha256: String,
    pub power_cards: HashMap<String, Json>,
    pub invader_cards: HashMap<String, Json>,
    pub boards: HashMap<String, Json>,
    pub adversaries: HashMap<String, Json>,
    pub spirits: HashMap<String, Json>,
    pub fear_cards: HashMap<String, Json>,
    pub event_cards: HashMap<String, Json>,
    pub blight_cards: HashMap<String, Json>,
    pub scenarios: HashMap<String, Json>,
    pub reminder_cards: HashMap<String, Json>,
}

pub const DEFAULT_DLL: &str =
    "/mnt/c/Program Files (x86)/Steam/steamapps/common/Spirit Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll";

impl EngineDefs {
    pub fn load(dll: &Path) -> Result<Self> {
        let bytes = std::fs::read(dll).with_context(|| format!("reading {}", dll.display()))?;
        let mut defs = EngineDefs { engine_sha256: hex(&Sha256::digest(&bytes)), ..Default::default() };
        for blob in manifest_resources(&bytes)? {
            let text = String::from_utf8_lossy(blob);
            let text = text.trim_start_matches('\u{feff}');
            let Ok(json) = serde_json::from_str::<Json>(text) else { continue };
            defs.absorb(json);
        }
        if defs.power_cards.is_empty() || defs.boards.is_empty() {
            bail!("engine definitions not found in {}", dll.display());
        }
        Ok(defs)
    }

    /// Sort one resource into its table by its shape.
    fn absorb(&mut self, json: Json) {
        let index = |arr: &Json| -> HashMap<String, Json> {
            arr.as_array()
                .into_iter()
                .flatten()
                .filter_map(|c| Some((c.get("identifier")?.as_str()?.to_string(), c.clone())))
                .collect()
        };
        if let Some(a) = json.get("adversaries") {
            self.adversaries = index(a);
        } else if let Some(b) = json.get("boards") {
            self.boards = index(b);
        } else if let Some(s) = json.get("spirits") {
            self.spirits = index(s);
        } else if let Some(s) = json.get("scenarios") {
            self.scenarios = index(s);
        } else if let Some(cards) = json.get("cards") {
            let first = cards.get(0).cloned().unwrap_or_default();
            let table = if first.get("energyCost").is_some() {
                &mut self.power_cards
            } else if first.get("landTypes").is_some() {
                &mut self.invader_cards
            } else if first.get("eventSections").is_some() {
                &mut self.event_cards
            } else if first.get("healthyIslandNumberPerStartingBoard").is_some() {
                &mut self.blight_cards
            } else if first.get("footer").is_some() {
                &mut self.reminder_cards
            } else if first.get("expansion").is_some()
                && first.get("body").and_then(|b| b.as_array()).is_some_and(|b| b.len() == 3)
            {
                // Fear cards: one body line per Terror Level. Reminder cards
                // also have title + body but carry a `footer` and no expansion.
                &mut self.fear_cards
            } else {
                return;
            };
            *table = index(cards);
        }
    }

    pub fn invader_stage(&self, id: &str) -> Option<u8> {
        self.invader_cards.get(id)?.get("stage")?.as_u64().map(|s| s as u8)
    }

    pub fn invader_land_types(&self, id: &str) -> Vec<String> {
        self.invader_cards
            .get(id)
            .and_then(|c| c.get("landTypes"))
            .and_then(|l| l.as_array())
            .map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }

    pub fn title_of(&self, id: &str) -> Option<String> {
        [&self.power_cards, &self.fear_cards, &self.event_cards, &self.blight_cards, &self.adversaries, &self.spirits, &self.reminder_cards]
            .iter()
            .find_map(|t| t.get(id))
            .and_then(|c| c.get("title").or_else(|| c.pointer("/eventSections/0/eventSectionTitle")))
            .and_then(|t| t.as_str())
            .map(String::from)
            .or_else(|| self.innate_title(id))
    }

    /// Innate powers live inside each spirit's definition.
    fn innate_title(&self, id: &str) -> Option<String> {
        self.spirits.values().find_map(|s| {
            s.get("innatePowers")?
                .as_array()?
                .iter()
                .find(|p| p.get("identifier").and_then(|i| i.as_str()) == Some(id))?
                .get("title")?
                .as_str()
                .map(String::from)
        })
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn u16_at(b: &[u8], o: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(o..o + 2)?.try_into().ok()?))
}
fn u32_at(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}

/// The managed resources of a .NET PE image, in blob order.
fn manifest_resources(pe: &[u8]) -> Result<Vec<&[u8]>> {
    let bad = || anyhow::anyhow!("not a .NET PE image");
    let lfanew = u32_at(pe, 0x3c).ok_or_else(bad)? as usize;
    if pe.get(lfanew..lfanew + 4) != Some(b"PE\0\0") {
        bail!(bad());
    }
    let coff = lfanew + 4;
    let n_sections = u16_at(pe, coff + 2).ok_or_else(bad)? as usize;
    let opt_size = u16_at(pe, coff + 16).ok_or_else(bad)? as usize;
    let opt = coff + 20;
    let dirs = match u16_at(pe, opt).ok_or_else(bad)? {
        0x10b => opt + 96,
        0x20b => opt + 112,
        _ => bail!(bad()),
    };
    let clr_rva = u32_at(pe, dirs + 14 * 8).ok_or_else(bad)?;
    let sections = opt + opt_size;
    let to_offset = |rva: u32| -> Option<usize> {
        (0..n_sections).find_map(|i| {
            let s = sections + i * 40;
            let vsize = u32_at(pe, s + 8)?;
            let va = u32_at(pe, s + 12)?;
            let raw_size = u32_at(pe, s + 16)?;
            let raw = u32_at(pe, s + 20)?;
            (rva >= va && rva < va + vsize.max(raw_size)).then(|| (rva - va + raw) as usize)
        })
    };
    let clr = to_offset(clr_rva).ok_or_else(bad)?;
    let res_rva = u32_at(pe, clr + 24).ok_or_else(bad)?;
    let res_size = u32_at(pe, clr + 28).ok_or_else(bad)? as usize;
    let base = to_offset(res_rva).ok_or_else(bad)?;
    let blob = pe.get(base..base + res_size).ok_or_else(bad)?;

    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos + 4 <= blob.len() {
        let len = u32_at(blob, pos).unwrap_or(0) as usize;
        if len == 0 || pos + 4 + len > blob.len() {
            break;
        }
        out.push(&blob[pos + 4..pos + 4 + len]);
        pos += 4 + len;
        // Compilers pad each resource to 8 bytes; skip zero padding.
        while pos < blob.len() && !pos.is_multiple_of(8) && blob[pos] == 0 {
            pos += 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs against the local game install; skipped when it is absent.
    #[test]
    fn loads_installed_engine_defs() {
        let dll = Path::new(DEFAULT_DLL);
        if !dll.exists() {
            return;
        }
        let d = EngineDefs::load(dll).unwrap();
        assert!(d.power_cards.len() > 300, "power cards: {}", d.power_cards.len());
        assert_eq!(d.invader_stage("Coastal2"), Some(2));
        assert_eq!(d.invader_land_types("SandsWetland3"), vec!["Sands", "Wetland"]);
        assert!(d.boards.contains_key("C"));
        assert!(d.adversaries.contains_key("TheKingdomOfBrandenburgPrussia"));
        assert!(d.spirits.contains_key("ShadowsFlickerLikeFlame"));
        assert!(!d.fear_cards.is_empty() && !d.event_cards.is_empty() && !d.blight_cards.is_empty());
    }
}
