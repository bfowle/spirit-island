//! The Invader deck and slots, the Fear deck and Terror level, Events and the
//! Blight card.

use serde::{Deserialize, Serialize};

use crate::island::Terrain;
use crate::InvaderCardId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvaderTarget {
    Terrain(Terrain),
    Coastal,
    Other(String),
}

impl InvaderTarget {
    pub fn parse(s: &str) -> InvaderTarget {
        match s {
            "Coastal" => InvaderTarget::Coastal,
            t => Terrain::parse(t).map(InvaderTarget::Terrain).unwrap_or_else(|| InvaderTarget::Other(t.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvaderCard {
    pub id: InvaderCardId,
    pub stage: u8,
    pub targets: Vec<InvaderTarget>,
    #[serde(default)]
    pub escalation: bool,
}

impl InvaderCard {
    /// Does this card act in a land of `terrain` (coastal flag for Stage II Coastal)?
    pub fn matches(&self, terrain: Terrain, coastal: bool) -> bool {
        self.targets.iter().any(|t| match t {
            InvaderTarget::Terrain(x) => *x == terrain,
            InvaderTarget::Coastal => coastal && terrain != Terrain::Ocean,
            InvaderTarget::Other(_) => false,
        })
    }
}

/// The three slots plus what is left and what has been discarded. Cards still
/// in the deck are known only by stage (public information: the deck order
/// by stage is printed; the terrains are not).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvaderDeck {
    pub remaining_stages: Vec<u8>,
    pub explore: Vec<InvaderCard>,
    pub build: Vec<InvaderCard>,
    pub ravage: Vec<InvaderCard>,
    pub discard: Vec<InvaderCard>,
}

impl InvaderDeck {
    pub fn remaining(&self) -> usize {
        self.remaining_stages.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FearCard {
    pub id: String,
    pub title: String,
    /// Terror Level I, II, III texts.
    pub body: Vec<String>,
    pub expansion: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FearState {
    /// Fear needed per card (4 per player).
    pub per_card: u32,
    /// Fear generated toward the next card.
    pub generated: u32,
    pub earned: Vec<String>,
    /// Cards left in the deck per Terror Level (I, II, III).
    pub remaining: [u8; 3],
    pub terror: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSection {
    pub title: String,
    pub body: Vec<String>,
    /// Engine kinds: HealthyIsland, BlightedIsland, Dahan, Beast, Token, …
    pub kinds: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCard {
    pub id: String,
    pub sections: Vec<EventSection>,
    pub expansion: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlightCard {
    pub id: String,
    pub title: String,
    pub healthy_per_board: u8,
    pub flipped_per_board: u8,
    pub flipped_effect_title: String,
    pub flipped_effect: String,
    pub expansion: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlightState {
    pub card: Option<String>,
    pub flipped: bool,
    pub remaining: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coastal_cards_match_coastal_lands_only() {
        let c = InvaderCard { id: "Coastal2".into(), stage: 2, targets: vec![InvaderTarget::Coastal], escalation: false };
        assert!(c.matches(Terrain::Sands, true));
        assert!(!c.matches(Terrain::Sands, false));
        assert!(!c.matches(Terrain::Ocean, true));
        let j = InvaderCard { id: "Jungle1".into(), stage: 1, targets: vec![InvaderTarget::Terrain(Terrain::Jungle)], escalation: false };
        assert!(j.matches(Terrain::Jungle, false));
        assert!(!j.matches(Terrain::Sands, true));
    }
}
