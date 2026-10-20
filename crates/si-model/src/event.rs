//! What happened: the vocabulary shared by the log reader (`si-record`) and
//! the rules (`si-rules`). A replay feeds these in; a simulation produces them.

use serde::{Deserialize, Serialize};

use crate::game::{Ending, Phase};
use crate::island::{LandId, PieceKind, TokenKind};
use crate::power::Speed;
use crate::{CardId, InvaderCardId, SpiritId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvaderStep {
    Explore,
    Build,
    Ravage,
}

impl InvaderStep {
    pub fn parse(s: &str) -> Option<InvaderStep> {
        Some(match s {
            "Explore" => InvaderStep::Explore,
            "Build" => InvaderStep::Build,
            "Ravage" => InvaderStep::Ravage,
            _ => return None,
        })
    }

    pub fn phase(self) -> Phase {
        match self {
            InvaderStep::Explore => Phase::Explore,
            InvaderStep::Build => Phase::Build,
            InvaderStep::Ravage => Phase::Ravage,
        }
    }
}

/// A piece moving: `from`/`to` are lands, or `None` for out of game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PieceMove {
    pub piece: String,
    pub kind: Option<PieceKind>,
    pub from: Option<LandId>,
    pub to: Option<LandId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    PhaseChanged { from: Option<Phase>, to: Phase },
    /// A Ravage/Build/Explore of one Invader card finished.
    InvaderCardResolved { step: InvaderStep, card: InvaderCardId },
    /// The Explore card was flipped (becomes public).
    InvaderCardRevealed { card: InvaderCardId },
    SlotsAdvanced { moves: Vec<(InvaderCardId, String, String)> },
    /// A Ravage action in one land began (the engine logs the land).
    Ravaged { land: LandId },
    PiecesAdded { moves: Vec<PieceMove> },
    PiecesMoved { moves: Vec<PieceMove> },
    PiecesDestroyed { pieces: Vec<(String, Option<LandId>)>, source: String },
    Damage { source: String, land: Option<LandId>, amount: u32, distribution: Vec<(String, u32)> },
    Defend { lands: Vec<LandId>, amount: u32, source: String },
    BlightAdded { land: Option<LandId>, amount: u32, cascade: bool },
    BlightRemoved { land: Option<LandId>, amount: u32 },
    Fear { amount: i32, land: Option<LandId>, source: String },
    FearCardEarned,
    FearCardResolved { card: String },
    TerrorLevel { level: u8 },
    EventRevealed { card: String },
    EventResolved { card: String },
    /// A Power (card or innate) used by a spirit at a land, with the phase.
    PowerUsed { spirit: Option<SpiritId>, power: String, target: Option<LandId>, speed: Option<Speed> },
    CardPlayed { spirit: Option<SpiritId>, card: CardId },
    CardUnplayed { spirit: Option<SpiritId>, card: CardId },
    EnergyPaid { spirit: SpiritId, amount: i64 },
    EnergyGained { spirit: SpiritId, amount: i64 },
    ElementsGained { spirit: Option<SpiritId>, source: String },
    PlaysAdjusted { source: String },
    /// Presence from a track slot (`node`) to a land, land to land, or land to
    /// destroyed (`to` None).
    PresenceMoved { spirit: SpiritId, node: Option<String>, from: Option<LandId>, to: Option<LandId>, destroyed: bool },
    GrowthChosen { spirit: SpiritId, group: u8 },
    GrowthOptionTaken { spirit: SpiritId, option: String },
    CardGained { spirit: Option<SpiritId>, card: CardId, deck: String },
    CardForgotten { spirit: Option<SpiritId>, card: CardId },
    CardsDiscarded { spirit: Option<SpiritId>, cards: Vec<CardId> },
    TokensAdded { land: Option<LandId>, kind: Option<TokenKind>, count: u32 },
    StatusAdded { text: String },
    TimePassed,
    GameOver { ending: Ending },
    Undo { file: String },
    Setup { text: String },
    /// An action line the reader recognized by name but did not type.
    Unparsed { action: String },
}

impl Event {
    /// The engine action name this event came from, for coverage counts.
    pub fn kind(&self) -> &'static str {
        match self {
            Event::PhaseChanged { .. } => "PhaseChanged",
            Event::InvaderCardResolved { .. } => "InvaderCardResolved",
            Event::InvaderCardRevealed { .. } => "InvaderCardRevealed",
            Event::SlotsAdvanced { .. } => "SlotsAdvanced",
            Event::Ravaged { .. } => "Ravaged",
            Event::PiecesAdded { .. } => "PiecesAdded",
            Event::PiecesMoved { .. } => "PiecesMoved",
            Event::PiecesDestroyed { .. } => "PiecesDestroyed",
            Event::Damage { .. } => "Damage",
            Event::Defend { .. } => "Defend",
            Event::BlightAdded { .. } => "BlightAdded",
            Event::BlightRemoved { .. } => "BlightRemoved",
            Event::Fear { .. } => "Fear",
            Event::FearCardEarned => "FearCardEarned",
            Event::FearCardResolved { .. } => "FearCardResolved",
            Event::TerrorLevel { .. } => "TerrorLevel",
            Event::EventRevealed { .. } => "EventRevealed",
            Event::EventResolved { .. } => "EventResolved",
            Event::PowerUsed { .. } => "PowerUsed",
            Event::CardPlayed { .. } => "CardPlayed",
            Event::CardUnplayed { .. } => "CardUnplayed",
            Event::EnergyPaid { .. } => "EnergyPaid",
            Event::EnergyGained { .. } => "EnergyGained",
            Event::ElementsGained { .. } => "ElementsGained",
            Event::PlaysAdjusted { .. } => "PlaysAdjusted",
            Event::PresenceMoved { .. } => "PresenceMoved",
            Event::GrowthChosen { .. } => "GrowthChosen",
            Event::GrowthOptionTaken { .. } => "GrowthOptionTaken",
            Event::CardGained { .. } => "CardGained",
            Event::CardForgotten { .. } => "CardForgotten",
            Event::CardsDiscarded { .. } => "CardsDiscarded",
            Event::TokensAdded { .. } => "TokensAdded",
            Event::StatusAdded { .. } => "StatusAdded",
            Event::TimePassed => "TimePassed",
            Event::GameOver { .. } => "GameOver",
            Event::Undo { .. } => "Undo",
            Event::Setup { .. } => "Setup",
            Event::Unparsed { .. } => "Unparsed",
        }
    }
}
