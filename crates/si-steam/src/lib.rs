//! si-steam — read-only companion source for the Steam (Handelabra) build of
//! Spirit Island.
//!
//! Pipeline: save files and `log.txt` → `EngineSnapshot` / `RawLogEvent`
//! (hidden information inside) → `projection` (the firewall) → public types
//! from `si-common` → `CompanionState`. Nothing outside `projection` reads
//! hidden fields.

pub mod backfill;
pub mod engine_defs;
pub mod companion;
pub mod engine_map;
pub mod log_parse;
pub mod nrbf;
pub mod projection;
pub mod seed;
pub mod source;
