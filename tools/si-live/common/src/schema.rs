//! Game-state schema. Mirrors `data/current-game.json`.
//!
//! The schema is intentionally lenient: unknown fields from the Wiki-parsed
//! spirit JSONs pass through as `serde_json::Value` so the parser can evolve
//! without breaking the rule engine.

use crate::elements::Element;
use crate::growth::GrowthOption;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameState {
    #[serde(default = "default_version")]
    pub version: String,
    pub round: u8,
    pub phase: Phase,
    pub setup: Setup,
    pub pools: Pools,
    pub spirits: HashMap<String, Spirit>,
    pub board_state: HashMap<String, Board>,
    #[serde(default)]
    pub log: Vec<LogEntry>,
}

fn default_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    #[default]
    Setup,
    Growth,
    Fast,
    Invader,
    Slow,
    TimePasses,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Setup {
    pub adversary: Option<String>,
    pub level: Option<u8>,
    pub scenario: Option<String>,
    #[serde(default)]
    pub spirits: Vec<String>,
    #[serde(default)]
    pub boards: Vec<String>,
    #[serde(default)]
    pub expansions_active: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pools {
    pub fear_current: u8,
    pub fear_threshold: u8,
    pub terror_level: u8,
    pub blight_current: u8,
    pub blight_cap: u8,
    #[serde(default)]
    pub island_blighted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Spirit {
    pub energy: i32,
    pub card_plays: u8,
    #[serde(default)]
    pub presence_on_board: HashMap<String, u8>,
    /// Remaining (covered) slots on the energy track, in order.
    #[serde(default)]
    pub presence_on_track_energy: Vec<String>,
    /// Remaining (covered) slots on the card-play track, in order.
    #[serde(default)]
    pub presence_on_track_cardplay: Vec<String>,
    #[serde(default)]
    pub elements_this_turn: HashMap<Element, u8>,
    #[serde(default)]
    pub hand: Vec<String>,
    #[serde(default)]
    pub discard: Vec<String>,
    #[serde(default)]
    pub forgotten: Vec<String>,
    #[serde(default)]
    pub played_this_turn: Vec<String>,
    #[serde(default)]
    pub growth_options: Vec<GrowthOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Board {
    pub lands: HashMap<String, Land>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Land {
    pub terrain: String,
    #[serde(default)]
    pub coastal: bool,
    #[serde(default)]
    pub explorers: u8,
    #[serde(default)]
    pub towns: u8,
    #[serde(default)]
    pub cities: u8,
    #[serde(default)]
    pub dahan: u8,
    #[serde(default)]
    pub blight: u8,
    #[serde(default)]
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub slug: String,
    pub name: String,
    pub cost: u8,
    pub speed: String,
    pub range: String,
    pub target: String,
    #[serde(default)]
    pub elements: Vec<Element>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub round: u8,
    pub event: String,
    #[serde(flatten)]
    pub details: serde_json::Value,
}
