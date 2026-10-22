//! The game's rules as code, trusted only where the log agrees. `invaders`
//! simulates the Invader phase (Ravage, Build, Explore, with the adversary
//! rules Brett's games use); `verify` replays every recorded turn through it
//! and reports where the simulation and the log differ. Design:
//! docs/superpowers/specs/2026-10-20-hindsight-analysis.md, section 3.

pub mod effects;
pub mod fuzz;
pub mod invaders;
pub mod verify;

pub use invaders::{Island, Land, Outcome, Rules};
pub use verify::{Check, Report};
