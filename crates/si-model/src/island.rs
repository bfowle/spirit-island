use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::str::FromStr;

use crate::SpiritId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Terrain {
    Ocean,
    Jungle,
    Mountain,
    Sands,
    Wetland,
}

impl Terrain {
    pub fn parse(s: &str) -> Option<Terrain> {
        Some(match s.trim() {
            "Ocean" => Terrain::Ocean,
            "Jungle" => Terrain::Jungle,
            "Mountain" => Terrain::Mountain,
            "Sands" => Terrain::Sands,
            "Wetland" => Terrain::Wetland,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Terrain::Ocean => "Ocean",
            Terrain::Jungle => "Jungle",
            Terrain::Mountain => "Mountain",
            Terrain::Sands => "Sands",
            Terrain::Wetland => "Wetland",
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A land on the island: board letter plus land number (`A3`). Land 0 is the
/// board's ocean. Parses the engine's `JungleA3`, `Jungle A3` and bare `A3`.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct LandId {
    pub board: String,
    pub number: u8,
}

impl From<LandId> for String {
    fn from(l: LandId) -> String {
        l.to_string()
    }
}

impl TryFrom<String> for LandId {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl LandId {
    pub fn new(board: &str, number: u8) -> Self {
        Self { board: board.to_string(), number }
    }

    /// The engine's compact form, `JungleA3`, needs the terrain.
    pub fn engine(&self, terrain: Terrain) -> String {
        format!("{}{}{}", terrain.name(), self.board, self.number)
    }
}

impl LandId {
    /// The short board code the log uses in land ids: `D`, or `NW` for the
    /// thematic "North West" board (the definitions spell it out).
    pub fn board_code(name: &str) -> String {
        let t: String = name.chars().filter(|c| !c.is_whitespace()).collect();
        match t.as_str() {
            "NorthWest" => "NW".into(),
            "NorthEast" => "NE".into(),
            "West" => "W".into(),
            "East" => "E".into(),
            "SouthWest" => "SW".into(),
            "SouthEast" => "SE".into(),
            _ => t,
        }
    }
}

impl FromStr for LandId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        // strip a leading terrain name if present
        let rest = ["Ocean", "Jungle", "Mountain", "Sands", "Wetland"]
            .iter()
            .find_map(|p| t.strip_prefix(p))
            .unwrap_or(&t);
        let mut board: String = rest.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        let digits: String = rest.chars().skip(board.len()).collect();
        // thematic boards are named in full; keep them short
        board = match board.as_str() {
            "NorthWest" => "NW".into(),
            "NorthEast" => "NE".into(),
            "West" => "W".into(),
            "East" => "E".into(),
            "SouthWest" => "SW".into(),
            "SouthEast" => "SE".into(),
            _ => board,
        };
        // boards are one or two capital letters (A–H, thematic NE/NW/E/W/SE/SW)
        if board.is_empty() || board.len() > 2 || !board.chars().all(|c| c.is_ascii_uppercase()) || digits.is_empty() || digits.len() > 2 {
            return Err(format!("not a land id: {s:?}"));
        }
        let number = digits.parse::<u8>().map_err(|_| format!("not a land id: {s:?}"))?;
        Ok(LandId { board, number })
    }
}

impl fmt::Display for LandId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.board, self.number)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceKind {
    Explorer,
    Town,
    City,
    Dahan,
}

impl PieceKind {
    pub fn parse(s: &str) -> Option<PieceKind> {
        let t = s.trim_end_matches(|c: char| c.is_ascii_digit());
        Some(match t {
            "Explorer" => PieceKind::Explorer,
            "Town" => PieceKind::Town,
            "City" => PieceKind::City,
            "Dahan" => PieceKind::Dahan,
            _ => return None,
        })
    }

    /// Printed health (core rulebook): Explorer 1, Town 2, City 3, Dahan 2.
    pub fn health(self) -> u8 {
        match self {
            PieceKind::Explorer => 1,
            PieceKind::Town => 2,
            PieceKind::City => 3,
            PieceKind::Dahan => 2,
        }
    }

    /// Damage dealt in a Ravage (Invaders) or counterattack (Dahan).
    pub fn damage(self) -> u8 {
        match self {
            PieceKind::Explorer => 1,
            PieceKind::Town => 2,
            PieceKind::City => 3,
            PieceKind::Dahan => 2,
        }
    }

    /// Fear generated when destroyed: Town 1, City 2.
    pub fn fear_on_destroy(self) -> u8 {
        match self {
            PieceKind::Town => 1,
            PieceKind::City => 2,
            _ => 0,
        }
    }

    pub fn is_invader(self) -> bool {
        !matches!(self, PieceKind::Dahan)
    }
}

/// One physical piece. The engine names pieces (`Town7`) and tracks damage
/// per piece, so the model does too.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub id: String,
    pub kind: PieceKind,
    #[serde(default)]
    pub damage: u8,
}

impl Piece {
    pub fn new(id: &str, kind: PieceKind) -> Self {
        Self { id: id.to_string(), kind, damage: 0 }
    }

    pub fn health_left(&self) -> u8 {
        self.kind.health().saturating_sub(self.damage)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    Beast,
    Wilds,
    Disease,
    Strife,
    Badlands,
    Vitality,
    Isolate,
}

impl TokenKind {
    pub fn parse(s: &str) -> Option<TokenKind> {
        let t = s.trim().trim_matches(|c| c == '{' || c == '}').trim_end_matches(|c: char| c.is_ascii_digit());
        Some(match t {
            "Beast" | "Beasts" => TokenKind::Beast,
            "Wilds" => TokenKind::Wilds,
            "Disease" => TokenKind::Disease,
            "Strife" => TokenKind::Strife,
            "Badlands" => TokenKind::Badlands,
            "Vitality" => TokenKind::Vitality,
            "Isolate" | "Isolated" => TokenKind::Isolate,
            _ => return None,
        })
    }
}

/// A land's printed setup.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitialPieces {
    pub explorers: u8,
    pub towns: u8,
    pub cities: u8,
    pub dahan: u8,
    pub blight: u8,
    #[serde(default)]
    pub beasts: u8,
    #[serde(default)]
    pub wilds: u8,
    #[serde(default)]
    pub disease: u8,
    #[serde(default)]
    pub strife: u8,
    #[serde(default)]
    pub badlands: u8,
}

/// A land as printed on a board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandDef {
    pub id: LandId,
    pub terrain: Terrain,
    /// Adjacent land numbers on the same board.
    pub adjacent: Vec<u8>,
    pub initial: InitialPieces,
}

impl LandDef {
    /// Coastal = adjacent to the board's ocean (land 0).
    pub fn coastal(&self) -> bool {
        self.terrain != Terrain::Ocean && self.adjacent.contains(&0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardDef {
    pub id: String,
    pub title: String,
    pub lands: Vec<LandDef>,
}

/// A land in play.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Land {
    pub id: LandId,
    pub terrain: Terrain,
    pub coastal: bool,
    pub adjacent: BTreeSet<LandId>,
    #[serde(default)]
    pub pieces: Vec<Piece>,
    #[serde(default)]
    pub blight: u32,
    #[serde(default)]
    pub tokens: BTreeMap<TokenKind, u32>,
    #[serde(default)]
    pub presence: BTreeMap<SpiritId, u32>,
}

impl Land {
    pub fn from_def(d: &LandDef) -> Self {
        let mut pieces = Vec::new();
        let mut n = 0u32;
        let mut push = |kind: PieceKind, count: u8| {
            for _ in 0..count {
                n += 1;
                pieces.push(Piece::new(&format!("{:?}{}", kind, n), kind));
            }
        };
        push(PieceKind::Explorer, d.initial.explorers);
        push(PieceKind::Town, d.initial.towns);
        push(PieceKind::City, d.initial.cities);
        push(PieceKind::Dahan, d.initial.dahan);
        let mut tokens = BTreeMap::new();
        for (k, v) in [
            (TokenKind::Beast, d.initial.beasts),
            (TokenKind::Wilds, d.initial.wilds),
            (TokenKind::Disease, d.initial.disease),
            (TokenKind::Strife, d.initial.strife),
            (TokenKind::Badlands, d.initial.badlands),
        ] {
            if v > 0 {
                tokens.insert(k, v as u32);
            }
        }
        Land {
            id: d.id.clone(),
            terrain: d.terrain,
            coastal: d.coastal(),
            adjacent: d.adjacent.iter().map(|n| LandId::new(&d.id.board, *n)).collect(),
            pieces,
            blight: d.initial.blight as u32,
            tokens,
            presence: BTreeMap::new(),
        }
    }

    pub fn count(&self, kind: PieceKind) -> u32 {
        self.pieces.iter().filter(|p| p.kind == kind).count() as u32
    }

    pub fn invaders(&self) -> u32 {
        self.pieces.iter().filter(|p| p.kind.is_invader()).count() as u32
    }

    pub fn has_invaders(&self) -> bool {
        self.invaders() > 0
    }

    pub fn token(&self, kind: TokenKind) -> u32 {
        self.tokens.get(&kind).copied().unwrap_or(0)
    }

    pub fn presence_of(&self, spirit: &str) -> u32 {
        self.presence.get(spirit).copied().unwrap_or(0)
    }

    pub fn total_presence(&self) -> u32 {
        self.presence.values().sum()
    }

    /// Ravage damage the Invaders here would deal, before Defend.
    pub fn invader_damage(&self) -> u32 {
        self.pieces.iter().filter(|p| p.kind.is_invader()).map(|p| p.kind.damage() as u32).sum()
    }
}

/// Every land in play, across boards, with adjacency including the
/// cross-board edges (which only a built game knows).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Island {
    pub lands: BTreeMap<LandId, Land>,
}

impl Island {
    pub fn from_boards(boards: &[BoardDef]) -> Self {
        let mut lands = BTreeMap::new();
        for b in boards {
            for d in &b.lands {
                lands.insert(d.id.clone(), Land::from_def(d));
            }
        }
        Island { lands }
    }

    pub fn get(&self, id: &LandId) -> Option<&Land> {
        self.lands.get(id)
    }

    pub fn get_mut(&mut self, id: &LandId) -> Option<&mut Land> {
        self.lands.get_mut(id)
    }

    /// Add a two-way adjacency (cross-board edges).
    pub fn connect(&mut self, a: &LandId, b: &LandId) {
        if let Some(l) = self.lands.get_mut(a) {
            l.adjacent.insert(b.clone());
        }
        if let Some(l) = self.lands.get_mut(b) {
            l.adjacent.insert(a.clone());
        }
    }

    /// Shortest path length in lands, not crossing oceans unless `through_ocean`.
    pub fn distance(&self, from: &LandId, to: &LandId, through_ocean: bool) -> Option<u32> {
        if from == to {
            return Some(0);
        }
        let mut seen: BTreeSet<&LandId> = BTreeSet::new();
        let mut q: VecDeque<(&LandId, u32)> = VecDeque::new();
        seen.insert(from);
        q.push_back((from, 0));
        while let Some((cur, d)) = q.pop_front() {
            let Some(land) = self.lands.get(cur) else { continue };
            for n in &land.adjacent {
                if n == to {
                    return Some(d + 1);
                }
                let Some(nl) = self.lands.get(n) else { continue };
                if nl.terrain == Terrain::Ocean && !through_ocean {
                    continue;
                }
                if seen.insert(n) {
                    q.push_back((n, d + 1));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn land_ids_parse_every_engine_spelling() {
        for s in ["JungleA3", "Jungle A3", "A3"] {
            let id: LandId = s.parse().unwrap();
            assert_eq!(id, LandId::new("A", 3));
        }
        assert_eq!(LandId::new("D", 7).to_string(), "D7");
        assert_eq!(LandId::new("D", 7).engine(Terrain::Sands), "SandsD7");
        assert!("Jungle".parse::<LandId>().is_err());
        assert!("StarlightSeeksItsFormCardPlays2".parse::<LandId>().is_err(), "a track slot is not a land");
        assert_eq!("WetlandNorthEast9".parse::<LandId>().unwrap(), LandId::new("NE", 9));
        assert_eq!("SandsNorthWest2".parse::<LandId>().unwrap().to_string(), "NW2");
        assert!("Volcano Looming High's Forgotten Powers".parse::<LandId>().is_err());
    }

    #[test]
    fn a_land_from_its_definition_has_its_setup_pieces() {
        let d = LandDef {
            id: LandId::new("A", 2),
            terrain: Terrain::Wetland,
            adjacent: vec![0, 1, 3, 4],
            initial: InitialPieces { cities: 1, dahan: 1, ..Default::default() },
        };
        let l = Land::from_def(&d);
        assert!(l.coastal);
        assert_eq!(l.count(PieceKind::City), 1);
        assert_eq!(l.count(PieceKind::Dahan), 1);
        assert_eq!(l.invader_damage(), 3);
        assert!(l.adjacent.contains(&LandId::new("A", 4)));
    }

    #[test]
    fn distance_walks_adjacency_and_skips_oceans() {
        let mk = |n: u8, t: Terrain, adj: &[u8]| LandDef {
            id: LandId::new("A", n),
            terrain: t,
            adjacent: adj.to_vec(),
            initial: InitialPieces::default(),
        };
        let b = BoardDef {
            id: "A".into(),
            title: "A".into(),
            lands: vec![
                mk(0, Terrain::Ocean, &[1, 2]),
                mk(1, Terrain::Mountain, &[0, 3]),
                mk(2, Terrain::Wetland, &[0]),
                mk(3, Terrain::Jungle, &[1]),
            ],
        };
        let island = Island::from_boards(&[b]);
        let a = |n| LandId::new("A", n);
        assert_eq!(island.distance(&a(3), &a(1), false), Some(1));
        assert_eq!(island.distance(&a(1), &a(2), false), None);
        assert_eq!(island.distance(&a(1), &a(2), true), Some(2));
    }
}
