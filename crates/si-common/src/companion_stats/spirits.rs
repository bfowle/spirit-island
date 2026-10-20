//! Spirit economy and powers: Energy and card-play curves from the presence
//! track, innate thresholds (met now / reachable with this hand), and power
//! draw odds from the unseen pools.

use crate::probability::hypergeom_at_least;
use crate::public::{PublicPowerCard, PublicSnapshot, PublicSpirit};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const ELEMENTS: [&str; 8] = ["Sun", "Moon", "Fire", "Air", "Water", "Earth", "Plant", "Animal"];

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpiritStats {
    pub id: String,
    pub title: String,
    pub energy: i64,
    pub card_plays: i64,
    pub energy_per_turn: Option<i64>,
    pub plays_per_turn: Option<i64>,
    /// Value revealed by the next presence taken off each track.
    pub next_energy: Option<i64>,
    pub next_plays: Option<i64>,
    pub elements: BTreeMap<String, i64>,
    pub innates: Vec<InnateStatus>,
    pub draw_odds: Vec<DrawOdds>,
    /// Hand cards by speed: what can still act this turn.
    pub hand_fast: Vec<String>,
    pub hand_slow: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InnateStatus {
    pub title: String,
    pub speed: String,
    pub levels: Vec<LevelStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LevelStatus {
    pub threshold: BTreeMap<String, i64>,
    pub met_now: bool,
    /// Elements still missing with no more cards played.
    pub missing: BTreeMap<String, i64>,
    /// Cheapest affordable set of hand cards that reaches it (titles), if any.
    pub reachable_with: Option<Vec<String>>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DrawOdds {
    pub element: String,
    /// (cards with the element, unseen pool, P(≥1 in a draw of 4)).
    pub minor: (u32, u32, f64),
    pub major: (u32, u32, f64),
}

pub fn track_value(sp: &PublicSpirit, prefix: &str, revealed: bool) -> Option<i64> {
    let nodes = sp.track.iter().filter(|n| n.kind.starts_with(prefix) && n.value.is_some());
    if revealed {
        nodes.filter(|n| n.revealed).filter_map(|n| n.value).max()
    } else {
        nodes.filter(|n| !n.revealed).filter_map(|n| n.value).next()
    }
}

fn meets(have: &BTreeMap<String, i64>, need: &BTreeMap<String, i64>) -> BTreeMap<String, i64> {
    need.iter()
        .filter_map(|(e, n)| {
            let short = n - have.get(e).copied().unwrap_or(0);
            (short > 0).then(|| (e.clone(), short))
        })
        .collect()
}

pub fn spirit_stats(sp: &PublicSpirit, s: &PublicSnapshot) -> SpiritStats {
    // Blitz: "All Powers are Fast" (scenario rules; Prussia + Blitz game: every
    // removal happened in the Fast phase).
    let blitz = s.scenario.as_deref() == Some("Blitz");
    let speed = |printed: &str| if blitz { "Fast".to_string() } else { printed.to_string() };
    let hand: Vec<&PublicPowerCard> = sp.hand.iter().collect();
    let affordable_sets = affordable_subsets(&hand, sp.energy, sp.card_plays);
    let innates = sp
        .innate_powers
        .iter()
        .map(|inn| InnateStatus {
            title: inn.title.clone(),
            speed: speed(&inn.speed),
            levels: inn
                .levels
                .iter()
                .map(|lvl| {
                    let missing = meets(&sp.elements, &lvl.elements);
                    let reachable_with = if missing.is_empty() {
                        Some(Vec::new())
                    } else {
                        affordable_sets
                            .iter()
                            .filter(|(set, _)| {
                                let mut have = sp.elements.clone();
                                for c in set {
                                    for e in &c.elements {
                                        *have.entry(e.clone()).or_default() += 1;
                                    }
                                }
                                meets(&have, &lvl.elements).is_empty()
                            })
                            .min_by_key(|(set, cost)| (*cost, set.len()))
                            .map(|(set, _)| set.iter().map(|c| c.title.clone()).collect())
                    };
                    LevelStatus { threshold: lvl.elements.clone(), met_now: missing.is_empty(), missing, reachable_with, body: lvl.body.clone() }
                })
                .collect(),
        })
        .collect();

    let odds = |pool: &[PublicPowerCard], e: &str| -> (u32, u32, f64) {
        let n = pool.len() as u32;
        let k = pool.iter().filter(|c| c.elements.iter().any(|x| x == e)).count() as u32;
        (k, n, if n == 0 { 0.0 } else { hypergeom_at_least(n, k, 4.min(n), 1) })
    };
    let draw_odds = ELEMENTS
        .iter()
        .map(|e| DrawOdds { element: e.to_string(), minor: odds(&s.powers.minor_unseen, e), major: odds(&s.powers.major_unseen, e) })
        .collect();

    SpiritStats {
        id: sp.id.clone(),
        title: sp.title.clone(),
        energy: sp.energy,
        card_plays: sp.card_plays,
        energy_per_turn: track_value(sp, "EnergyPerTurn", true),
        plays_per_turn: track_value(sp, "CardPlaysPerTurn", true),
        next_energy: track_value(sp, "EnergyPerTurn", false),
        next_plays: track_value(sp, "CardPlaysPerTurn", false),
        elements: sp.elements.clone(),
        innates,
        draw_odds,
        hand_fast: sp.hand.iter().filter(|c| speed(&c.speed) == "Fast").map(|c| c.title.clone()).collect(),
        hand_slow: sp.hand.iter().filter(|c| speed(&c.speed) == "Slow").map(|c| c.title.clone()).collect(),
    }
}

/// Every set of hand cards within the card plays left and Energy, with its cost.
fn affordable_subsets<'a>(hand: &[&'a PublicPowerCard], energy: i64, plays: i64) -> Vec<(Vec<&'a PublicPowerCard>, i64)> {
    let n = hand.len().min(12);
    let mut out = Vec::new();
    for mask in 1u32..(1 << n) {
        if mask.count_ones() as i64 > plays {
            continue;
        }
        let set: Vec<&PublicPowerCard> = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| hand[i]).collect();
        let cost: i64 = set.iter().map(|c| c.cost).sum();
        if cost <= energy {
            out.push((set, cost));
        }
    }
    out
}
