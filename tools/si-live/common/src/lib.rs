//! si-common — shared game-state schema and rule engine for Spirit Island.
//!
//! Both the axum backend and the MCP stdio server depend on this crate, so
//! the rules live in exactly one place. The guide authoring workflow also
//! references these types when writing opener/tempo sections (via the
//! `si-rules-check` skill), guaranteeing that chapter prose cannot make
//! claims the rule engine would reject.

pub mod elements;
pub mod growth;
pub mod probability;
pub mod schema;
pub mod stats;

pub use elements::Element;
pub use schema::{Card, GameState, Phase, Spirit};
