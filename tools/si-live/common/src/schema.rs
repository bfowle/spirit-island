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
    /// Invader deck state. Ordered upcoming stack (stage-known; terrain revealed on flip)
    /// + three exposed positions (ravage/build/explore) + discard pile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invader_deck: Option<InvaderDeckState>,
    /// Fear deck state. 9-slot (3·L1 + 3·L2 + 3·L3) model — earned (drawn, awaiting Fear
    /// phase) vs resolved (played). Ordered revealed names when looked up from fear.json.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fear_deck: Option<FearDeckState>,
    /// Event deck state. Rule: the first Event card is flipped face-up during Setup
    /// (previewed for Turn 1 planning) but doesn't resolve until Turn 2. Previewed
    /// and resolved are separate lists so the tool correctly models the one-turn lead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_deck: Option<EventDeckState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvaderCard {
    /// 1, 2, or 3 — locked when the stack is built (known at setup).
    pub stage: u8,
    /// Revealed terrain when the card is flipped (e.g., "Jungle", "Mountain + Wetland").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terrain: Option<String>,
    /// Optional notes (e.g., adversary escalation applied this flip).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvaderDeckState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ravage: Option<InvaderCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<InvaderCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explore: Option<InvaderCard>,
    /// Upcoming stack: [0] = next to flip into Explore.
    #[serde(default)]
    pub upcoming: Vec<InvaderCard>,
    /// Discarded (post-Ravage) cards — kept ordered, newest last.
    #[serde(default)]
    pub discarded: Vec<InvaderCard>,
    /// Human-readable notation of the stack shape (e.g., "3 · 4 · 5").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FearDeckState {
    /// Total size of the Fear deck. Default = 3 × player count (3 players → 9).
    /// Adversaries/scenarios can add/subtract from specific terror levels.
    pub deck_size: u32,
    /// Card count at each terror level: [T1, T2, T3]. When absent, the UI
    /// treats it as evenly split from `deck_size`. Explicit counts let
    /// adversaries with asymmetric splits be represented faithfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier_counts: Option<[u32; 3]>,
    /// Fear-generated thresholds crossed so far (cards earned but not yet resolved).
    /// Each entry is the fear-card name when looked up from `data/decks/fear.json`.
    #[serde(default)]
    pub earned: Vec<FearCardEntry>,
    /// Fear cards already resolved in prior Fear phases.
    #[serde(default)]
    pub resolved: Vec<FearCardEntry>,
    /// Unseen count remaining in the deck.
    pub unseen: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FearCardEntry {
    /// Card name (from data/decks/fear.json). Empty if not yet identified.
    pub name: String,
    /// Terror level at which the card was drawn (1/2/3).
    pub terror_level: u8,
    /// Round the card was earned.
    pub round: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventCardEntry {
    /// Event name as it appears in data/decks/event.json. Empty until identified.
    pub name: String,
    /// The turn this card was previewed (face-up, visible).
    pub previewed_on_turn: u8,
    /// The turn the card resolved (usually `previewed_on_turn + 1`).
    /// None while the card is still in the "previewed" bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_on_turn: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventDeckState {
    /// Face-up card(s) previewed for next turn. Typically exactly one at a time.
    #[serde(default)]
    pub previewed: Vec<EventCardEntry>,
    /// Cards that have resolved in prior turns, preserved for retrospective review.
    #[serde(default)]
    pub resolved: Vec<EventCardEntry>,
    /// Cards remaining in the Event deck (face-down, unknown).
    pub unseen: u32,
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
    /// Sub-phase of the Invader Phase — event card resolves here.
    Event,
    /// Sub-phase of the Invader Phase — earned fear cards resolve here.
    Fear,
    /// Sub-phase of the Invader Phase — Ravage / Build / Explore actions.
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
    /// Selected Blight card name (matches data/decks/blight.json).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blight_card: Option<String>,
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

    /// Per-spirit disc color (hex) — purely cosmetic, drives the presence-disc
    /// rendering across the app. Persisted with the rest of game state so the
    /// UI's color/style customization survives refresh + save/load.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disc_color: Option<String>,
    /// Per-spirit disc style: "glass" | "wood" | "solid".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disc_style: Option<String>,

    /// Selected aspect slug (e.g., "dark-fire"). None = base spirit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect: Option<String>,
    /// Aspect display name (e.g., "Dark Fire").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect_name: Option<String>,
    /// Aspect special-rule overrides. Array of `{name, text}` pairs — kept as
    /// raw JSON so schema evolution on the Wiki side doesn't need code changes.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub aspect_special_rules: serde_json::Value,
    /// Raw setup-note string from the aspect (e.g., "Gain Unquenchable Flames (Minor Power)").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect_setup_note: Option<String>,
    /// Replacement innate (when an aspect overrides the spirit's base innate).
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub aspect_innate_override: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Board {
    pub lands: HashMap<String, Land>,
    /// Variant key chosen at New Game (e.g., "balanced" or "thematic").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Human-readable variant name (e.g., "Board A" or "North East").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_name: Option<String>,
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
    /// Event-specific payload (e.g., `{"amount": 2}` for fear_generated).
    /// Kept as a named field (not flattened) so the frontend's `{details:{…}}`
    /// shape round-trips cleanly and `fear_by_round` can read `details.amount`.
    #[serde(default)]
    pub details: serde_json::Value,
}
