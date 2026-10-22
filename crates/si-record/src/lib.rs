//! From the game's own log to a `GameRecord`.
//!
//! `log` turns each line into an `si_model::event::Event`; `replay` walks the
//! events with the turn and phase tracked, applies the undo reloads the log
//! records, and builds the record: what every land went through, what every
//! spirit did and held, and what the board looked like after each turn.

pub mod log;
pub mod notation;
pub mod record;
pub mod replay;

pub use record::{Choice, Counts, Coverage, GameMeta, GameRecord, InvaderVisit, LandLedger, PowerUse, SpiritTurn, Touch, TouchKind, TurnRecord};
pub use replay::Replay;
