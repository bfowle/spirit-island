//! Derived statistics on top of GameState.
//!
//! The backend computes these on demand for its `/api/stats` endpoint;
//! the MCP server can also expose them via `compute_*` tools in Phase C+.

use crate::elements::Element;
use crate::schema::{GameState, LogEntry};
use serde::Serialize;
use std::collections::HashMap;

/// Innate threshold proximity summary for a spirit.
#[derive(Debug, Clone, Serialize)]
pub struct InnateThresholdStatus {
    pub innate_name: String,
    pub speed: String,
    pub levels: Vec<InnateLevelStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InnateLevelStatus {
    pub level: u8,
    /// Elements the level requires, e.g. {"moon": 2, "fire": 1}.
    pub required: HashMap<String, u8>,
    /// Elements present in the spirit's elements_this_turn pool.
    pub have: HashMap<String, u8>,
    /// Shortfalls: required - have, per element, non-zero entries only.
    pub shortfall: HashMap<String, u8>,
    /// True when every required element is satisfied (shortfall empty).
    pub fires: bool,
}

/// Compute innate-threshold status for each (spirit, innate, level) tuple
/// given the current GameState's elements_this_turn on each spirit.
///
/// **Note**: this is a naive whole-turn check — it doesn't differentiate
/// Fast-phase elements from Slow-phase. Phase C+ will track elements by
/// phase to accurately model Fast-innate firing. For now, this treats the
/// full elements_this_turn pool as available.
pub fn compute_innate_status(
    state: &GameState,
    spirit_innates: &HashMap<String, Vec<SpiritInnateSpec>>,
) -> HashMap<String, Vec<InnateThresholdStatus>> {
    let mut out: HashMap<String, Vec<InnateThresholdStatus>> = HashMap::new();
    for (slug, spirit) in &state.spirits {
        let Some(innates) = spirit_innates.get(slug) else {
            continue;
        };
        let mut per_spirit = Vec::new();
        for innate in innates {
            let levels = innate
                .thresholds
                .iter()
                .enumerate()
                .map(|(i, thr)| {
                    let have: HashMap<String, u8> = spirit
                        .elements_this_turn
                        .iter()
                        .map(|(k, v)| (element_key(*k).to_string(), *v))
                        .collect();
                    let mut shortfall = HashMap::new();
                    for (elem, need) in &thr.required {
                        let got = have.get(elem).copied().unwrap_or(0);
                        if got < *need {
                            shortfall.insert(elem.clone(), need - got);
                        }
                    }
                    InnateLevelStatus {
                        level: (i + 1) as u8,
                        required: thr.required.clone(),
                        have,
                        shortfall: shortfall.clone(),
                        fires: shortfall.is_empty(),
                    }
                })
                .collect();
            per_spirit.push(InnateThresholdStatus {
                innate_name: innate.name.clone(),
                speed: innate.speed.clone(),
                levels,
            });
        }
        out.insert(slug.clone(), per_spirit);
    }
    out
}

/// Representation the stats module needs for each innate — callers build this
/// from the parsed Wiki JSON once at startup.
#[derive(Debug, Clone)]
pub struct SpiritInnateSpec {
    pub name: String,
    pub speed: String,
    pub thresholds: Vec<InnateThresholdSpec>,
}

#[derive(Debug, Clone)]
pub struct InnateThresholdSpec {
    pub required: HashMap<String, u8>,
    pub effect: String,
}

fn element_key(e: Element) -> &'static str {
    match e {
        Element::Sun => "sun",
        Element::Moon => "moon",
        Element::Fire => "fire",
        Element::Air => "air",
        Element::Water => "water",
        Element::Earth => "earth",
        Element::Plant => "plant",
        Element::Animal => "animal",
    }
}

/// Fear pool history per round, derived from log entries.
#[derive(Debug, Clone, Serialize)]
pub struct FearOverRounds {
    pub round: u8,
    pub fear_generated: u32,
}

pub fn fear_by_round(log: &[LogEntry]) -> Vec<FearOverRounds> {
    let mut by_round: HashMap<u8, u32> = HashMap::new();
    for entry in log {
        if entry.event == "fear_generated" {
            if let Some(n) = entry.details.get("amount").and_then(|v| v.as_u64()) {
                *by_round.entry(entry.round).or_insert(0) += n as u32;
            }
        }
    }
    let mut out: Vec<FearOverRounds> = by_round
        .into_iter()
        .map(|(round, fear_generated)| FearOverRounds { round, fear_generated })
        .collect();
    out.sort_by_key(|r| r.round);
    out
}
