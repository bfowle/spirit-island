//! Adversaries, scenarios and the setup of one game.

use serde::{Deserialize, Serialize};

use crate::SpiritId;

/// Pieces a level adds during setup, on each board.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupAdditions {
    pub towns_in_land: Vec<u8>,
    pub cities_in_land: Vec<u8>,
    pub blight_in_land: Vec<u8>,
    /// Sweden 2: Blight starting in land #4 goes to land #5 instead.
    pub move_blight: Option<(u8, u8)>,
    pub explorer_highest_without_town_city: bool,
    pub beast_highest_without_town_city: bool,
    pub town_highest_without_town: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdversaryLevel {
    pub number: u8,
    pub difficulty: u8,
    /// Fear deck composition at this level: cards added to the base 3/3/3
    /// per Terror Level (I/II/III). Not cumulative; each level states its own.
    pub fear_modifiers: [u8; 3],
    pub title: String,
    pub body: String,
    /// Extra effects some levels carry (Habsburg Mining's Ceaseless Mining).
    #[serde(default)]
    pub extra: Vec<(String, String)>,
    /// The Invader deck order this level's text prints ("111-3-2222-3333"),
    /// with the special cards kept as letters (C, S). Later levels inherit it.
    #[serde(default)]
    pub deck_order: Option<String>,
    /// Sweden 4: "Accelerate the Invader Deck" (one Stage I card fewer after setup).
    #[serde(default)]
    pub accelerate: bool,
    #[serde(default)]
    pub setup: SetupAdditions,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adversary {
    pub id: String,
    pub title: String,
    pub base_difficulty: u8,
    pub levels: Vec<AdversaryLevel>,
    pub escalation: (String, String),
    pub loss_condition: (String, String),
    pub expansion: String,
}

impl Adversary {
    /// The base Invader deck: three Stage I, four Stage II, five Stage III.
    pub const BASE_DECK: &'static str = "111-2222-33333";

    pub fn level(&self, n: u8) -> Option<&AdversaryLevel> {
        self.levels.iter().find(|l| l.number == n)
    }

    /// The deck order in force at level `n` (level 0: the base deck).
    pub fn deck_order_at(&self, n: u8) -> String {
        let mut order = Self::BASE_DECK.to_string();
        let mut levels: Vec<&AdversaryLevel> = self.levels.iter().filter(|l| l.number <= n).collect();
        levels.sort_by_key(|l| l.number);
        for l in levels {
            if let Some(o) = &l.deck_order {
                order = o.clone();
            }
            if l.accelerate {
                if let Some(i) = order.find('1') {
                    order.remove(i);
                }
            }
        }
        order
    }

    /// The deck order as stages, in draw order (special cards count as Stage II).
    pub fn stages_at(&self, n: u8) -> Vec<u8> {
        self.deck_order_at(n)
            .chars()
            .filter_map(|c| match c {
                '1' => Some(1),
                '2' | 'C' | 'S' => Some(2),
                '3' => Some(3),
                _ => None,
            })
            .collect()
    }

    /// Fear cards per Terror Level at level `n` (level 0: 3/3/3).
    pub fn fear_deck_at(&self, n: u8) -> [u8; 3] {
        let m = self.level(n).map(|l| l.fear_modifiers).unwrap_or([0; 3]);
        [3 + m[0], 3 + m[1], 3 + m[2]]
    }

    /// Every level effect in force at level `n` (levels are cumulative).
    pub fn effects_through(&self, n: u8) -> Vec<&AdversaryLevel> {
        self.levels.iter().filter(|l| l.number <= n).collect()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub difficulty: i8,
    pub expansion: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seat {
    pub spirit: SpiritId,
    pub aspect: Option<String>,
    pub board: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSetup {
    pub seats: Vec<Seat>,
    pub boards: Vec<String>,
    pub adversary: Option<(String, u8)>,
    pub scenario: Option<String>,
    #[serde(default)]
    pub thematic: bool,
}

impl GameSetup {
    pub fn players(&self) -> usize {
        self.seats.len()
    }
}
