//! Public game state: only what a player at a physical table could know.
//!
//! These types are the far side of the Steam companion's information
//! firewall (`si-steam/src/projection.rs`). No field can hold a deck order,
//! an RNG seed, or the name of a face-down card: decks are counts plus the
//! stages printed on card backs, earned Fear cards are a count, and the
//! blight card is its generic healthy side until it flips.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicSnapshot {
    pub game_id: String,
    pub engine_build: String,
    /// Human turn number (engine `TurnIndex` + 1; setup belongs to Turn 1).
    pub turn: u32,
    /// Engine phase: StartOfGame, Spirit, FastPower, Invader, SlowPower, TimePasses.
    pub phase: String,
    /// Engine step within the phase: Grow, FastPower, Ravage, Build, Explore, …
    pub step: String,
    pub difficulty: Option<i64>,
    pub adversaries: Vec<AdversaryRef>,
    pub scenario: Option<String>,
    pub multiplayer: bool,
    pub boards: Vec<PublicBoard>,
    pub lands: Vec<PublicLand>,
    /// Undirected adjacency by land id, Ocean lands included.
    pub adjacency: Vec<(String, String)>,
    pub spirits: Vec<PublicSpirit>,
    pub invaders: PublicInvaderDeck,
    pub fear: PublicFear,
    pub events: PublicEvents,
    pub powers: PublicPowers,
    pub blight: PublicBlight,
    pub status_effects: Vec<PublicStatusEffect>,
    pub ending: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdversaryRef {
    pub id: String,
    pub title: String,
    pub level: i64,
    /// Pieces on the adversary's panel by kind, which its rules track
    /// (Russia: Beasts destroyed by adversary rules).
    #[serde(default)]
    pub panel: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicBoard {
    pub id: String,
    pub lands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicLand {
    pub id: String,
    pub board: String,
    pub number: i64,
    /// Jungle, Mountain, Sands, Wetland, Ocean.
    pub terrain: String,
    pub is_ocean: bool,
    /// Adjacent to an Ocean land.
    pub coastal: bool,
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    pub dahan: u32,
    pub blight: u32,
    /// Presence count by spirit id.
    pub presence: BTreeMap<String, u32>,
    /// Token count by type (Beast, Wilds, Disease, Badlands, …).
    pub tokens: BTreeMap<String, u32>,
    /// Invaders and Dahan with their current health (damage this turn is public).
    pub pieces: Vec<PublicPiece>,
    pub damage_taken: i64,
    pub defend: i64,
    pub health_modifier: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicPiece {
    /// Explorer, Town, City, Dahan, Thief.
    pub kind: String,
    pub max_health: i64,
    pub damage: i64,
    pub strife: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicSpirit {
    pub id: String,
    pub title: String,
    pub aspect: Option<String>,
    pub energy: i64,
    pub card_plays: i64,
    pub phase_status: Option<String>,
    /// Element counts by name (Sun, Moon, …), cards in play plus tracks.
    pub elements: BTreeMap<String, i64>,
    pub element_markers: BTreeMap<String, i64>,
    pub hand: Vec<PublicPowerCard>,
    pub play_area: Vec<PublicPowerCard>,
    pub discard: Vec<PublicPowerCard>,
    pub forgotten: Vec<PublicPowerCard>,
    /// Cards revealed while gaining a power (public while the choice is open).
    pub revealed: Vec<PublicPowerCard>,
    /// Presence-track nodes in engine order; `revealed` = no presence covers it.
    pub track: Vec<TrackNode>,
    pub presence_on_island: u32,
    pub presence_destroyed: u32,
    pub innates: Vec<String>,
    pub innate_powers: Vec<PublicInnate>,
    pub special_rules: Vec<NamedText>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackNode {
    pub id: String,
    pub revealed: bool,
    /// Engine node type: EnergyPerTurn, CardPlaysPerTurn, Element, ReclaimCards, …
    pub kind: String,
    /// The printed number (Energy or card plays per turn) when the node has one.
    pub value: Option<i64>,
    pub elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicInnate {
    pub id: String,
    pub title: String,
    pub speed: String,
    pub range: Option<i64>,
    pub from_sacred_site: bool,
    pub target: Option<String>,
    pub levels: Vec<InnateLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InnateLevel {
    /// Element threshold, e.g. {"Moon": 2, "Fire": 1}.
    pub elements: BTreeMap<String, i64>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NamedText {
    pub id: String,
    pub title: String,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicPowerCard {
    pub id: String,
    pub title: String,
    pub cost: i64,
    /// "Fast" or "Slow".
    pub speed: String,
    /// "Minor", "Major", "Unique".
    pub kind: String,
    pub elements: Vec<String>,
    pub range: Option<i64>,
    pub from_sacred_site: bool,
    pub target: Option<String>,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicInvaderCard {
    pub id: String,
    pub stage: u8,
    /// The card carries the Escalation flag (Stage II terrain cards).
    #[serde(default)]
    pub escalation: bool,
    /// Land types the card matches: Jungle, Mountain, Sands, Wetland, Coastal, SaltDeposits.
    pub terrains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicInvaderDeck {
    /// Stage printed on the back of each face-down card, next card first.
    pub upcoming_stages: Vec<u8>,
    /// Face-up in the Explore slot (only during Explore).
    pub explore: Vec<PublicInvaderCard>,
    pub build: Vec<PublicInvaderCard>,
    pub ravage: Vec<PublicInvaderCard>,
    pub discard: Vec<PublicInvaderCard>,
    /// Adversary slots beyond the base three (e.g. England's High Immigration).
    pub extra_slots: Vec<(String, Vec<PublicInvaderCard>)>,
    pub highest_stage_reached: i64,
    /// Every invader card this game's rules can use (the public pool, from
    /// the card list), so odds come from *pool − seen*, never the deck.
    pub pool: Vec<PublicInvaderCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicFear {
    pub generated: i64,
    pub required_per_card: i64,
    pub terror_level: u8,
    /// Earned and still face down. Count only: names stay hidden until resolved.
    pub earned_count: u32,
    /// Fear cards left in the deck at each Terror Level section (divider
    /// positions are public): `[TL I, TL II, TL III]`.
    pub deck_by_level: [u32; 3],
    pub discard: Vec<PublicFearCard>,
    /// The Fear Card being resolved now, if any.
    #[serde(default)]
    pub resolving: Option<PublicFearCard>,
    /// Fear cards removed without being seen (count only).
    pub removed_unseen: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicFearCard {
    pub id: String,
    pub title: String,
    pub resolved_at_terror_level: Option<i64>,
    /// The card's text at Terror Level I, II, III (public once revealed).
    #[serde(default)]
    pub body: Vec<String>,
}

/// A revealed Event Card with its sections (public once revealed).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicEventCard {
    pub id: String,
    pub title: String,
    pub sections: Vec<PublicEventSection>,
    /// Being resolved now (in the Play Area), not yet discarded.
    pub in_play: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicEventSection {
    pub title: String,
    /// When it applies: HealthyIsland, BlightedIsland, TerrorLevelI…, StageI…,
    /// or a piece/token section (Dahan, Beast, Disease, Strife, …).
    pub types: Vec<String>,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicEvents {
    pub deck_count: u32,
    /// The Event being resolved, else the last one resolved.
    #[serde(default)]
    pub shown: Option<PublicEventCard>,
    /// Newest last.
    pub discard: Vec<NamedCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NamedCard {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicPowers {
    pub minor_deck_count: u32,
    pub major_deck_count: u32,
    pub minor_discard: Vec<String>,
    pub major_discard: Vec<String>,
    /// Allowed pool − every card seen outside the deck. These are the cards
    /// that could be drawn; computed from public zones, never from the deck.
    pub minor_unseen: Vec<PublicPowerCard>,
    pub major_unseen: Vec<PublicPowerCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicBlight {
    pub flipped: bool,
    /// Blight left on the card.
    pub remaining: i64,
    pub on_island: u32,
    pub healthy_side: Vec<String>,
    /// Only set once the card has flipped.
    pub blighted_card: Option<NamedCard>,
    pub blighted_text: Vec<String>,
    /// Before the flip: for each blight card that could be face down here,
    /// the blight it would put on the card when flipped (per board × boards).
    /// `None` entries are cards with no printed number.
    pub unseen_flipped: Vec<Option<i64>>,
    /// Before the flip: how many of those cards are Still-Healthy (emptying
    /// one draws a new Blight Card instead of losing).
    #[serde(default)]
    pub unseen_still_healthy: u32,
    /// The current card's flipped side is Still-Healthy (emptying it draws a
    /// new card, already flipped, instead of losing).
    #[serde(default)]
    pub current_still_healthy: bool,
    /// Every Blight Card not seen yet (allowed minus the current card when
    /// face up, minus cards already used): what a draw could bring. Public:
    /// built from the allowed list and seen cards only.
    #[serde(default)]
    pub pool: Vec<BlightOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BlightOption {
    /// Blight its flipped side puts on the card (per board × boards).
    pub flipped_total: Option<i64>,
    pub still_healthy: bool,
    /// Shares the face-down current card's healthy side (before the flip,
    /// the current card is one of these).
    pub same_healthy_side: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicStatusEffect {
    pub kind: String,
    /// Title of the public card or rule that created it, when known.
    pub source: Option<String>,
    /// event, fear, power, blight, adversary, scenario, spirit or other.
    /// Event, Fear and Power effects last only this turn.
    #[serde(default)]
    pub source_kind: String,
    /// Engine parameters (non-default only), when the source is public.
    pub params: BTreeMap<String, StatusParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StatusParam {
    Int(i64),
    Bool(bool),
    List(Vec<String>),
}

/// One of the player's decisions, from the game log (whitelisted kinds only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicDecision {
    pub turn: u32,
    pub step: String,
    /// growth, gain-power, play, power, effect, land-choice, presence, damage, forget.
    pub kind: String,
    pub text: String,
    pub land: Option<String>,
    pub card: Option<String>,
    /// Capture wall clock, ms since midnight (orders decisions against saves).
    pub ms: u64,
    /// An Event revealed in the log, with its sections (the save shows it
    /// only at the end of the Event step).
    #[serde(default)]
    pub event: Option<PublicEventCard>,
}
