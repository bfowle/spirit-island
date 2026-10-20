//! The game as data. Every type here is plain, serializable and free of
//! rules: `si-rules` decides what happens, `si-record` reads what happened,
//! `si-metrics` measures it. The one organizing idea is the list of forces in
//! [`force::Force`]: each force that decides a turn has a type here.

pub mod decks;
pub mod element;
pub mod event;
pub mod force;
pub mod game;
pub mod island;
pub mod modifier;
pub mod power;
pub mod setup;
pub mod spirit;

pub use element::{Element, ElementSet};
pub use game::{Ending, GameState, Phase, Step};
pub use island::{Island, Land, LandId, Piece, PieceKind, Terrain, TokenKind};
pub use power::{PowerCard, Range, Speed, TargetKind};
pub use spirit::{SpiritPanel, SpiritState, TrackNode};

/// Engine identifiers are used as ids throughout (e.g. `Thunderspeaker`,
/// `ManifestationOfPowerAndGlory`, `Mountain1`).
pub type SpiritId = String;
pub type CardId = String;
pub type InvaderCardId = String;
