//! The per-turn ledger (spec 2.0): what was due, what was handled, what it
//! cost in cards and Energy, and where the hand stood afterwards.

use serde::{Deserialize, Serialize};

use crate::tempo::{Chain, Class};
use crate::track::TrackRow;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LedgerTurn {
    pub turn: u32,
    /// Lands whose Build fell on this turn (an Above action this turn or last
    /// turn's Slow phase prevents it).
    pub due_build: u32,
    /// Lands whose Ravage fell on this turn.
    pub due_ravage: u32,
    /// Of the Ravage-due lands: handled Above, handled On, saved by fate,
    /// touched and still Blighted, left to Blight with reach, left without.
    pub above: u32,
    pub on: u32,
    pub gambled: u32,
    pub failed: u32,
    pub untouched: u32,
    pub sacrificed: u32,
    pub blight: u32,
    /// Cards every spirit played this turn and the Energy they paid.
    pub cards_spent: u32,
    pub energy_spent: i64,
    /// Cards in hand and in the discard after the turn, all spirits.
    pub hand_after: u32,
    pub discard_after: u32,
    /// Shortest hand runway among the spirits (turns before a forced Reclaim).
    pub runway: Option<u32>,
    pub reclaimed: bool,
}

impl LedgerTurn {
    pub fn handled(&self) -> u32 {
        self.above + self.on
    }
}

/// One ledger line per turn, from the chains and every spirit's track rows.
pub fn ledger(chains: &[Chain], rows: &[TrackRow], energy_paid: &dyn Fn(u32) -> i64) -> Vec<LedgerTurn> {
    let last = rows.iter().map(|r| r.turn).max().unwrap_or(0).max(chains.iter().map(Chain::ravage_turn).max().unwrap_or(0));
    (1..=last)
        .map(|turn| {
            let mut l = LedgerTurn { turn, ..Default::default() };
            for c in chains {
                if c.build == Some(turn) || (c.build.is_none() && c.t0 + 1 == turn && c.explore.is_some()) {
                    l.due_build += 1;
                }
                if c.ravage_turn() == turn && c.class != Class::Open {
                    l.due_ravage += 1;
                    l.blight += c.blight;
                    match c.class {
                        Class::Above => l.above += 1,
                        Class::On => l.on += 1,
                        Class::Gambled => l.gambled += 1,
                        Class::Failed | Class::Late => l.failed += 1,
                        Class::Untouched => l.untouched += 1,
                        Class::Sacrificed => l.sacrificed += 1,
                        Class::Open => {}
                    }
                }
            }
            for r in rows.iter().filter(|r| r.turn == turn) {
                l.cards_spent += r.cards_played;
                l.hand_after += r.hand;
                l.discard_after += r.discard;
                l.reclaimed |= r.reclaimed;
                l.runway = match (l.runway, r.runway) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (None, b) => b,
                    (a, None) => a,
                };
            }
            l.energy_spent = energy_paid(turn);
            l
        })
        .collect()
}
