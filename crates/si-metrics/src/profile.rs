//! Rei's three build axes over the opening (turns 1–3 for scaling, 1–4 for
//! stance and Blight).
//!
//! Scaling follows the labels Rei puts on his own builds (Vengeance, Many
//! Minds, Shroud guides; fixtures in tests/rei_openings.rs):
//!
//! * **Fast** – every growth through turn 3 took the full presence the panel
//!   allows, except at most one that took a card instead (Vengeance G3,
//!   Shroud G3). "The fast scaling path to unlocking 3 card plays before
//!   reclaim."
//! * **Minus** – a growth gave up presence for something else a second time,
//!   or took less than the full presence (Many Minds G3's single presence):
//!   "either your Energy gain or Card Plays on your Presence Track becomes
//!   slightly reduced for 1 turn" for more cards or position.
//! * **Problematic** – a Reclaim by turn 3 that the hand forced (hand < plays
//!   at the start of the turn): the Hasteful, "too sacrificial to your tempo".
//!
//! A Reclaim by turn 3 that was not forced (Shroud's Unleashed fallback) is
//! measured on the slots it revealed, so it reads Minus.

use serde::{Deserialize, Serialize};
use si_defs::Definitions;

use crate::tempo::{Chain, Class};
use crate::track::{self, TrackRow};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scaling {
    Fast,
    Minus,
    Problematic,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stance {
    Above,
    On,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildProfile {
    pub spirit: String,
    pub scaling: Scaling,
    pub stance: Stance,
    /// Presence placed from the track by the end of turn 3, against the ceiling.
    pub presence_by_t3: (u32, u32),
    /// Cards taken by growth choices instead of presence, turns 1–3.
    pub growth_cards_by_t3: u32,
    /// All cards gained by turn 3 (growth, pay-to-gain slots, reclaim gains).
    pub cards_gained_by_t3: u32,
    pub first_reclaim: Option<u32>,
    pub forced_reclaim_by_t3: bool,
    /// Above / (Above + On) over chains whose Explore fell on turns 1–4.
    pub above_share: Option<f64>,
    pub blight_by_t4: u32,
}

pub fn profile(defs: Option<&Definitions>, spirit: &str, rows: &[TrackRow], chains: &[Chain], blight_by_t4: u32) -> BuildProfile {
    let by_t3: Vec<&TrackRow> = rows.iter().filter(|r| r.turn <= 3).collect();
    let placed: u32 = by_t3.iter().map(|r| r.presence_placed).sum();
    let gained: u32 = by_t3.iter().map(|r| r.cards_gained).sum();
    let growth_cards: u32 = by_t3.iter().map(|r| r.growth_cards).sum();
    let per_growth = defs.map(|d| track::max_presence_per_growth(d, spirit) as u32).unwrap_or(0);
    let ceiling = per_growth * 3;
    let first_reclaim = track::reclaim_turns(rows).first().copied();
    let forced = by_t3.iter().any(|r| r.forced_reclaim);
    let scaling = if rows.is_empty() || ceiling == 0 {
        Scaling::Unknown
    } else if forced || (first_reclaim.is_some_and(|t| t <= 3) && defs.is_none()) {
        Scaling::Problematic
    } else if first_reclaim.is_none_or(|t| t > 3) && growth_cards <= 1 && placed + growth_cards >= ceiling {
        // every growth revealed the full presence, or traded exactly one
        // presence for one card, at most once
        Scaling::Fast
    } else {
        Scaling::Minus
    };
    let opening: Vec<&Chain> = chains.iter().filter(|c| (1..=4).contains(&c.t0) && c.class.handled()).collect();
    let above = opening.iter().filter(|c| c.class == Class::Above).count();
    let above_share = (!opening.is_empty()).then(|| above as f64 / opening.len() as f64);
    let stance = match above_share {
        Some(s) if s >= 0.5 => Stance::Above,
        Some(_) => Stance::On,
        None => Stance::Unknown,
    };
    BuildProfile { spirit: spirit.to_string(), scaling, stance, presence_by_t3: (placed, ceiling), growth_cards_by_t3: growth_cards, cards_gained_by_t3: gained, first_reclaim, forced_reclaim_by_t3: forced, above_share, blight_by_t4 }
}
