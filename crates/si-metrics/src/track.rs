//! Track tempo: one row per spirit per turn, in Rei's x/ycp terms, with the
//! hand-and-discard state Brett counts as part of the ramp.

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::spirit::{Spike, SpiritPanel};
use si_record::{GameRecord, SpiritTurn};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackRow {
    pub turn: u32,
    pub spirit: String,
    pub growth: String,
    pub income: Option<i64>,
    pub plays: Option<i64>,
    /// Presence placed from a track slot this turn (what scaling counts).
    pub presence_placed: u32,
    /// Presence that arrived from elsewhere (setup, cards, special rules).
    pub presence_other: u32,
    pub presence_destroyed: u32,
    /// All cards gained this turn (growth, pay-to-gain slots, reclaim gains).
    pub cards_gained: u32,
    /// Cards the chosen growth group(s) hand out by their printed yield; a
    /// growth that took a card instead of presence (Vengeance G3, Shroud G3).
    /// Reclaim groups are not counted here.
    pub growth_cards: u32,
    /// The most presence the panel's growth could have placed this turn.
    pub presence_ceiling: u32,
    pub reclaimed: bool,
    /// The Reclaim was forced: the hand at the start of the turn could not
    /// fill the plays the track already gave (hand < plays).
    pub forced_reclaim: bool,
    pub cards_played: u32,
    pub energy_after: i64,
    /// Cards in hand at the start of the turn (before Growth).
    pub hand_before: u32,
    pub hand: u32,
    pub discard: u32,
    /// Plays the spirit did not use (plays − cards played), when plays are known.
    pub plays_unused: Option<i64>,
    /// Turns the hand lasts at this turn's spend rate before a Reclaim is
    /// forced (hand < plays): floor(hand / cards played), None when nothing
    /// was played.
    pub runway: Option<u32>,
}

impl TrackRow {
    /// One row from a spirit's turn, given the previous row (for the hand at
    /// the start of the turn) and the panel (for yields and the starting hand).
    pub fn from_turn(s: &SpiritTurn, prev: Option<&TrackRow>, panel: Option<&SpiritPanel>) -> TrackRow {
        let hand_before = match prev {
            Some(p) => p.hand,
            None => panel.map(|p| p.uniques.len() as u32).unwrap_or(0),
        };
        let plays_before = prev.and_then(|p| p.plays).or_else(|| panel.map(|p| p.track_state(&p.starting_revealed()).plays));
        let groups: Vec<&si_model::spirit::GrowthGroup> = panel.map(|p| s.growth_groups.iter().filter_map(|g| (*g as usize).checked_sub(1).and_then(|i| p.growth.groups.get(i))).collect()).unwrap_or_default();
        let growth_cards: u32 = if s.reclaimed { 0 } else { groups.iter().map(|g| g.yield_().cards as u32).sum() };
        let presence_ceiling = panel.map(|p| max_presence_per_growth_panel(p) as u32).unwrap_or(0);
        let cards_played = s.cards_played.len() as u32;
        let hand = s.hand_after.len() as u32;
        TrackRow {
            turn: s.turn,
            spirit: s.spirit.clone(),
            growth: if s.growth_groups.is_empty() { "-".into() } else { s.growth_groups.iter().map(|g| format!("G{g}")).collect::<Vec<_>>().join("+") },
            income: s.income,
            plays: s.plays,
            presence_placed: s.presence_placed.iter().filter(|(n, _)| n.is_some()).count() as u32,
            presence_other: s.presence_placed.iter().filter(|(n, _)| n.is_none()).count() as u32,
            presence_destroyed: s.presence_destroyed,
            cards_gained: s.cards_gained.len() as u32,
            growth_cards,
            presence_ceiling,
            reclaimed: s.reclaimed,
            forced_reclaim: s.reclaimed && plays_before.is_some_and(|p| (hand_before as i64) < p),
            cards_played,
            energy_after: s.energy_after,
            hand_before,
            hand,
            discard: s.discard_after.len() as u32,
            plays_unused: s.plays.map(|p| p - cards_played as i64),
            runway: (cards_played > 0).then(|| hand / cards_played),
        }
    }
}

pub fn rows(rec: &GameRecord, defs: Option<&Definitions>, spirit: &str) -> Vec<TrackRow> {
    let panel = defs.and_then(|d| d.spirits.get(spirit));
    let mut out: Vec<TrackRow> = Vec::new();
    for s in rec.spirits.iter().filter(|s| s.spirit == spirit && s.turn > 0) {
        let row = TrackRow::from_turn(s, out.last(), panel);
        out.push(row);
    }
    out
}

/// First turn whose track state meets the spike.
pub fn spike_turn(rows: &[TrackRow], spike: &Spike) -> Option<u32> {
    rows.iter().find(|r| r.income.is_some_and(|i| i >= spike.income) && r.plays.is_some_and(|p| p >= spike.plays)).map(|r| r.turn)
}

/// Turns on which a Reclaim growth was taken.
pub fn reclaim_turns(rows: &[TrackRow]) -> Vec<u32> {
    rows.iter().filter(|r| r.reclaimed).map(|r| r.turn).collect()
}

/// Reclaims two turns apart: Rei's reclaim loop.
pub fn reclaim_loops(rows: &[TrackRow]) -> u32 {
    let t = reclaim_turns(rows);
    t.windows(2).filter(|w| w[1] == w[0] + 2).count() as u32
}

/// Weak turns: plays left unused, or nothing played.
pub fn weak_turns(rows: &[TrackRow]) -> Vec<u32> {
    rows.iter().filter(|r| r.cards_played == 0 || r.plays_unused.is_some_and(|u| u > 0)).map(|r| r.turn).collect()
}

/// Turns on which banked Energy had risen for two turns running (Rei's
/// stockpiling), reported as the turn the second rise landed on.
pub fn stockpile_turns(rows: &[TrackRow]) -> Vec<u32> {
    rows.windows(3).filter(|w| w[1].energy_after > w[0].energy_after && w[2].energy_after > w[1].energy_after).map(|w| w[2].turn).collect()
}

/// The panel's largest presence yield from one growth choice (the Fast
/// scaling ceiling per turn).
pub fn max_presence_per_growth(defs: &Definitions, spirit: &str) -> u8 {
    defs.spirits.get(spirit).map(max_presence_per_growth_panel).unwrap_or(0)
}

fn max_presence_per_growth_panel(p: &SpiritPanel) -> u8 {
    p.growth.groups.iter().map(|g| g.yield_().presence).max().unwrap_or(0) * p.growth.pick
}
