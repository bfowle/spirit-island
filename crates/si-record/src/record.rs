//! What a replay produces.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use si_model::event::InvaderStep;
use si_model::game::{Ending, Phase};
use si_model::island::{LandId, PieceKind, Terrain};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameMeta {
    pub source: String,
    /// Engine spirit ids when definitions were available, else the log's short names.
    pub spirits: Vec<String>,
    pub boards: Vec<String>,
    pub ending: Option<Ending>,
    pub turns: u32,
    /// The log started at setup (else it joined a game in progress).
    pub from_setup: bool,
    /// The adversary named in the log (level is not logged).
    #[serde(default)]
    pub adversary: Option<String>,
    /// From the engine save beside the log, when one was read.
    #[serde(default)]
    pub adversary_level: Option<u8>,
    #[serde(default)]
    pub difficulty: Option<i64>,
    #[serde(default)]
    pub scenario: Option<String>,
}

/// One Invader step in one land.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvaderVisit {
    pub turn: u32,
    pub step: Option<InvaderStep>,
    pub card: Option<String>,
    /// Pieces the step added here.
    pub added: Vec<PieceKind>,
    /// Blight this step added here (direct) and by cascade.
    pub blight: u32,
    pub cascade: u32,
    /// Damage the Invaders dealt here (a Ravage), Defend in force, Dahan lost,
    /// Invaders destroyed by the counterattack.
    pub damage: u32,
    pub defend: u32,
    pub dahan_lost: u32,
    pub invaders_destroyed: u32,
    /// Pieces standing here after the step.
    pub after: Counts,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    pub dahan: u32,
    pub blight: u32,
    pub presence: u32,
    #[serde(default)]
    pub beasts: u32,
    #[serde(default)]
    pub disease: u32,
    #[serde(default)]
    pub strife: u32,
    #[serde(default)]
    pub wilds: u32,
    #[serde(default)]
    pub badlands: u32,
}

impl Counts {
    pub fn invaders(&self) -> u32 {
        self.explorers + self.towns + self.cities
    }

    pub fn buildings(&self) -> u32 {
        self.towns + self.cities
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchKind {
    /// A Power or innate used with this land as its target.
    Power,
    Defend(u32),
    /// Damage dealt here by a Spirit effect.
    Damage(u32),
    /// Presence placed or moved here during Growth.
    Presence,
    /// Pieces pushed out of or gathered into this land by a Spirit effect.
    Moved { out: u32, into: u32 },
    /// Blight removed here.
    Recover(u32),
}

/// A decision the engine asked the player to make and what was on offer:
/// a land for an Event or Fear effect, an "or" branch of a Power, a piece to
/// Strife, a card to gain or forget. `source` is the card or rule that asked;
/// `source_kind` is Event, Fear, Power, Spirit, Invader, Blight or Rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub phase: Phase,
    /// Location, Effect, Piece, Spirit, Board, Number, Card.
    pub kind: String,
    pub reason: String,
    pub selected: String,
    pub choices: Vec<String>,
    pub source: String,
    pub source_kind: String,
    pub optional: bool,
    pub spirit: Option<String>,
    /// The engine's activation number, for telling repeats apart.
    pub instance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touch {
    pub turn: u32,
    pub phase: Phase,
    /// Power id, or what else did it.
    pub by: String,
    pub spirit: Option<String>,
    pub kind: TouchKind,
}

impl Touch {
    /// A Spirit's own action (Growth, Fast or Slow phase), as opposed to an
    /// Event, Fear card or Invader-phase effect.
    pub fn by_spirit(&self) -> bool {
        matches!(self.phase, Phase::SpiritPhase | Phase::FastPower | Phase::SlowPower)
    }

    /// A touch that works on the land's Invaders or Blight (not a presence placement).
    pub fn solving(&self) -> bool {
        !matches!(self.kind, TouchKind::Presence)
    }
}

/// Everything that happened in one land, in order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandLedger {
    pub land: LandId,
    pub terrain: Option<Terrain>,
    pub visits: Vec<InvaderVisit>,
    pub touches: Vec<Touch>,
    /// (turn, phase, amount, cascade) for every Blight added here.
    pub blight: Vec<(u32, Phase, u32, bool)>,
    pub end: Counts,
}

/// One spirit's turn.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpiritTurn {
    pub turn: u32,
    pub spirit: String,
    pub growth_groups: Vec<u8>,
    pub growth_options: Vec<String>,
    /// (track slot revealed, land) for presence placed this turn (slot None:
    /// from off the board, e.g. setup).
    pub presence_placed: Vec<(Option<String>, Option<LandId>)>,
    /// Presence moved land to land (growth options, special rules).
    #[serde(default)]
    pub presence_moved: Vec<(LandId, LandId)>,
    pub presence_destroyed: u32,
    pub revealed_after: BTreeSet<String>,
    pub income: Option<i64>,
    pub plays: Option<i64>,
    pub energy_gained: i64,
    pub energy_paid: i64,
    pub energy_after: i64,
    pub cards_played: Vec<String>,
    pub cards_gained: Vec<String>,
    pub cards_forgotten: Vec<String>,
    pub reclaimed: bool,
    pub reclaimed_cards: Vec<String>,
    pub hand_after: Vec<String>,
    pub discard_after: Vec<String>,
    /// (phase, power, target)
    pub powers_used: Vec<(Phase, String, Option<LandId>)>,
    /// Where this spirit's presence stood at the end of the turn (for range).
    #[serde(default)]
    pub presence_after: BTreeMap<LandId, u32>,
    /// The Power Cards offered at this turn's gain (the draft the player chose from).
    #[serde(default)]
    pub offered: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnRecord {
    pub turn: u32,
    pub explore: Vec<String>,
    pub build: Vec<String>,
    pub ravage: Vec<String>,
    /// Cards that built from England's High Immigration slot (before the Ravage).
    #[serde(default)]
    pub high_immigration: Vec<String>,
    /// Every land as it stood when the first Invader step of the turn began
    /// (after the Event and Fear cards), for the rules simulator.
    #[serde(default)]
    pub before_invaders: BTreeMap<LandId, Counts>,
    /// Defend in force per land at that moment.
    #[serde(default)]
    pub defend: BTreeMap<LandId, u32>,
    /// Every land as it stood when the Fast phase began (after Growth) and
    /// when the Slow phase began (after the Invader steps), for verifying
    /// Spirit actions.
    #[serde(default)]
    pub before_fast: BTreeMap<LandId, Counts>,
    #[serde(default)]
    pub before_slow: BTreeMap<LandId, Counts>,
    /// Rules the engine put in force this turn (skips, cancels, Defend
    /// grants, extra Explorers, health and damage changes), as logged.
    #[serde(default)]
    pub status_effects: Vec<StatusEffect>,
    /// Every choice the player was asked to make this turn, with what was offered.
    #[serde(default)]
    pub choices: Vec<Choice>,
    /// Invader steps the engine canceled this turn, with the source.
    #[serde(default)]
    pub canceled: Vec<Canceled>,
    pub fear_generated: i32,
    pub fear_cards_earned: u32,
    pub terror_after: u8,
    pub blight_added: u32,
    pub blight_cascaded: u32,
    pub events: Vec<String>,
    pub fear_cards: Vec<String>,
    pub board_after: Counts,
    pub lands_with_invaders_after: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    pub kind: String,
    pub source: String,
    pub params: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Canceled {
    pub step: InvaderStep,
    pub land: Option<LandId>,
    pub source: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coverage {
    pub typed: BTreeMap<String, u32>,
    pub skipped: BTreeMap<String, u32>,
    pub noise: u32,
    pub undo_reloads: u32,
    pub undo_dropped: u32,
    pub undo_unresolved: u32,
    /// Rollbacks inferred from a fact that cannot happen twice.
    #[serde(default)]
    pub undo_repairs: u32,
    pub copies: u32,
    pub notes: Vec<String>,
}

impl Coverage {
    pub fn typed_total(&self) -> u32 {
        self.typed.values().sum()
    }

    pub fn skipped_total(&self) -> u32 {
        self.skipped.values().sum()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameRecord {
    pub meta: GameMeta,
    /// Pieces on each land when turn 1 began (setup, adversary additions included).
    #[serde(default)]
    pub setup: BTreeMap<LandId, Counts>,
    /// Fear cards earned while the Terror Level was I, II, III (the Fear
    /// deck's layout, hence the adversary level, shows through these).
    #[serde(default)]
    pub fear_earned_by_level: [u32; 3],
    pub turns: Vec<TurnRecord>,
    pub lands: BTreeMap<LandId, LandLedger>,
    pub spirits: Vec<SpiritTurn>,
    pub coverage: Coverage,
}

impl GameRecord {
    pub fn turn(&self, n: u32) -> Option<&TurnRecord> {
        self.turns.iter().find(|t| t.turn == n)
    }

    pub fn spirit_turn(&self, spirit: &str, n: u32) -> Option<&SpiritTurn> {
        self.spirits.iter().find(|s| s.spirit == spirit && s.turn == n)
    }
}
