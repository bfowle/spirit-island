//! Pure functions over a `GameRecord`. Definitions in
//! docs/superpowers/specs/2026-10-20-tempo-and-growth-model.md.

pub mod adversary;
pub mod fit;
pub mod growth;
pub mod ledger;
pub mod profile;
pub mod reach;
pub mod spikes;
pub mod tempo;
pub mod track;

pub use profile::{BuildProfile, Scaling, Stance};
pub use adversary::LevelEstimate;
pub use growth::{Growth, GrowthOrder};
pub use ledger::LedgerTurn;
pub use reach::{ChainReach, ReachCard};
pub use tempo::{Chain, Class, Clock, Outcome};
pub use track::TrackRow;
