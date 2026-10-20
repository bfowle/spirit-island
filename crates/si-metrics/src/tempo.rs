//! Land tempo: every Invader card's pass over a land is a chain
//! Explore (t) → Build (t+1) → Ravage (t+2). Each Spirit touch sits on that
//! clock; the Ravage slot decides the outcome; the two together give Rei's
//! class.

use serde::{Deserialize, Serialize};
use si_model::event::InvaderStep;
use si_model::game::Phase;
use si_model::island::{LandId, Terrain};
use si_defs::Definitions;
use si_record::{GameRecord, Touch, TouchKind};

use crate::reach::{self, ChainReach};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Clock {
    BeforeExplore,
    AfterExplore,
    AfterBuild,
    AfterRavage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// No Ravage happened in the land on the chain's Ravage turn.
    Prevented,
    /// Ravaged, no Blight, no Town or City left.
    Cleared,
    /// Ravaged, no Blight, buildings remain.
    Defended,
    /// Blight added by the Ravage.
    Blighted,
    /// The game ended before the Ravage slot.
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Class {
    Above,
    On,
    Gambled,
    Failed,
    Sacrificed,
    Untouched,
    Late,
    Open,
}

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Above => "above",
            Class::On => "on",
            Class::Gambled => "gambled",
            Class::Failed => "failed",
            Class::Sacrificed => "sacrificed",
            Class::Untouched => "untouched",
            Class::Late => "late",
            Class::Open => "open",
        }
    }

    pub fn handled(self) -> bool {
        matches!(self, Class::Above | Class::On)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainTouch {
    pub clock: Clock,
    pub touch: Touch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chain {
    pub land: LandId,
    pub terrain: Option<Terrain>,
    pub card: Option<String>,
    pub stage: Option<u8>,
    /// The chain's clock origin: the Explore turn (or Build − 1, Ravage − 2).
    pub t0: u32,
    pub explore: Option<u32>,
    pub build: Option<u32>,
    pub ravage: Option<u32>,
    pub outcome: Outcome,
    pub blight: u32,
    pub cascade: u32,
    pub dahan_lost: u32,
    pub defend: u32,
    pub damage: u32,
    pub touches: Vec<ChainTouch>,
    pub class: Class,
    /// Index into `touches` of the touch that decided the outcome.
    pub deciding: Option<usize>,
    /// Could a held card have acted here (needs definitions)?
    #[serde(default)]
    pub reach: Option<ChainReach>,
    /// Cards the spirits played at this land over the chain, and their Energy.
    #[serde(default)]
    pub price_cards: u32,
    #[serde(default)]
    pub price_energy: i64,
}

impl Chain {
    /// The Ravage turn on the chain's clock.
    pub fn ravage_turn(&self) -> u32 {
        self.ravage.unwrap_or(self.t0 + 2)
    }

    /// Blight landed with no Spirit action before the Ravage slot.
    pub fn left_to_blight(&self) -> bool {
        matches!(self.class, Class::Sacrificed | Class::Untouched)
    }
}

fn phase_order(p: Phase) -> u8 {
    match p {
        Phase::PlayerSetup | Phase::InvadersStartingAction => 0,
        Phase::SpiritPhase => 1,
        Phase::FastPower => 2,
        Phase::BlightedIsland => 3,
        Phase::Event => 4,
        Phase::Fear => 5,
        Phase::Ravage => 6,
        Phase::Build => 7,
        Phase::Explore => 8,
        Phase::AdvanceInvaderCards => 9,
        Phase::SlowPower => 10,
        Phase::TimePasses => 11,
        Phase::GameOver | Phase::Other => 12,
    }
}

/// Where a touch at (turn, phase) sits on a chain whose Explore was turn `t0`.
pub fn clock(t0: u32, turn: u32, phase: Phase) -> Clock {
    let p = phase_order(phase);
    if turn < t0 || (turn == t0 && p < phase_order(Phase::Explore)) {
        Clock::BeforeExplore
    } else if turn == t0 || (turn == t0 + 1 && p < phase_order(Phase::Build)) {
        Clock::AfterExplore
    } else if turn == t0 + 1 || (turn == t0 + 2 && p < phase_order(Phase::Ravage)) {
        Clock::AfterBuild
    } else {
        Clock::AfterRavage
    }
}

fn stage_of(card: &str) -> Option<u8> {
    card.chars().last().and_then(|c| c.to_digit(10)).map(|d| d as u8)
}

/// Spare card plays any spirit still had on `turn` (plays minus cards played).
fn plays_left(rec: &GameRecord, turn: u32) -> Option<i64> {
    let rows: Vec<_> = rec.spirits.iter().filter(|s| s.turn == turn).collect();
    if rows.is_empty() {
        return None;
    }
    let mut left = 0i64;
    for s in &rows {
        left += s.plays? - s.cards_played.len() as i64;
    }
    Some(left)
}

pub fn chains(rec: &GameRecord) -> Vec<Chain> {
    let last_turn = rec.meta.turns;
    let mut out = Vec::new();
    for (land, ledger) in &rec.lands {
        // group the land's visits by Invader card, in order of first visit
        let mut groups: Vec<(Option<String>, Vec<&si_record::InvaderVisit>)> = Vec::new();
        for v in &ledger.visits {
            match groups.iter_mut().find(|(c, vs)| *c == v.card && vs.last().is_some_and(|l| v.turn <= l.turn + 2)) {
                Some((_, vs)) => vs.push(v),
                None => groups.push((v.card.clone(), vec![v])),
            }
        }
        for (card, visits) in groups {
            let turn_of = |s: InvaderStep| visits.iter().find(|v| v.step == Some(s)).map(|v| v.turn);
            let (explore, build, ravage) = (turn_of(InvaderStep::Explore), turn_of(InvaderStep::Build), turn_of(InvaderStep::Ravage));
            let t0 = explore.or_else(|| build.map(|b| b.saturating_sub(1))).or_else(|| ravage.map(|r| r.saturating_sub(2))).unwrap_or(0);
            let rv = visits.iter().find(|v| v.step == Some(InvaderStep::Ravage));
            let outcome = match rv {
                Some(v) if v.blight > 0 => Outcome::Blighted,
                Some(v) if v.after.buildings() == 0 => Outcome::Cleared,
                Some(_) => Outcome::Defended,
                None if last_turn >= t0 + 2 && rec.meta.ending.is_some() || last_turn > t0 + 2 => Outcome::Prevented,
                None => Outcome::Open,
            };
            let touches: Vec<ChainTouch> = ledger
                .touches
                .iter()
                .filter(|t| t.turn >= t0 && t.turn <= t0 + 2 && !matches!(t.kind, TouchKind::Recover(_)))
                .map(|t| ChainTouch { clock: clock(t0, t.turn, t.phase), touch: t.clone() })
                .collect();
            // Only a Spirit's own solving action can decide a chain; presence
            // placements and Event/Fear effects are context.
            let pre: Vec<(usize, &ChainTouch)> = touches.iter().enumerate().filter(|(_, t)| t.clock != Clock::AfterRavage && t.touch.by_spirit() && t.touch.solving()).collect();
            let deciding = pre.last().map(|(i, _)| *i);
            let earliest = pre.iter().map(|(_, t)| t.clock).min();
            let late_only = touches.iter().any(|t| t.touch.by_spirit() && t.touch.solving()) && pre.is_empty();
            let class = match outcome {
                Outcome::Open => Class::Open,
                Outcome::Prevented => match earliest {
                    Some(Clock::BeforeExplore) | Some(Clock::AfterExplore) => Class::Above,
                    Some(_) => Class::On,
                    None => Class::Gambled,
                },
                Outcome::Cleared | Outcome::Defended => match deciding.map(|i| touches[i].clock) {
                    Some(Clock::AfterBuild) => Class::On,
                    Some(_) => Class::Above,
                    None => Class::Gambled,
                },
                Outcome::Blighted => {
                    if deciding.is_some() {
                        Class::Failed
                    } else if late_only {
                        Class::Late
                    } else {
                        match plays_left(rec, ravage.unwrap_or(t0 + 2)) {
                            Some(left) if left > 0 => Class::Untouched,
                            _ => Class::Sacrificed,
                        }
                    }
                }
            };
            out.push(Chain {
                land: land.clone(),
                terrain: ledger.terrain,
                stage: card.as_deref().and_then(stage_of),
                card,
                t0,
                explore,
                build,
                ravage,
                outcome,
                blight: rv.map(|v| v.blight).unwrap_or(0),
                cascade: rv.map(|v| v.cascade).unwrap_or(0),
                dahan_lost: rv.map(|v| v.dahan_lost).unwrap_or(0),
                defend: rv.map(|v| v.defend).unwrap_or(0),
                damage: rv.map(|v| v.damage).unwrap_or(0),
                touches,
                class,
                deciding,
                reach: None,
                price_cards: 0,
                price_energy: 0,
            });
        }
    }
    out.sort_by_key(|c| (c.t0, c.land.clone()));
    out
}

/// Chains with `reach` and price filled in from the definitions, and the
/// Sacrificed / Untouched split decided by what the hand could do (D2):
/// Untouched when a held card surely reached the Ravage slot with a play to
/// spare; Sacrificed when no card could; the plays-left reading stays when
/// range could not be measured.
pub fn chains_with(rec: &GameRecord, defs: Option<&Definitions>) -> Vec<Chain> {
    let mut out = chains(rec);
    let Some(defs) = defs else { return out };
    let island = reach::island_for(rec, defs);
    for c in &mut out {
        let r = reach::chain_reach(rec, defs, &island, c);
        if c.left_to_blight() {
            if r.could_ravage() {
                c.class = Class::Untouched;
            } else if !r.ravage_uncertain() {
                c.class = Class::Sacrificed;
            }
        }
        let (cards, energy) = reach::price(c, defs);
        c.price_cards = cards;
        c.price_energy = energy;
        c.reach = Some(r);
    }
    out
}

/// Chains by class for the turns whose Ravage slot falls on `turn`.
pub fn classes_by_turn(chains: &[Chain]) -> Vec<(u32, Vec<(Class, u32)>)> {
    let mut turns: std::collections::BTreeMap<u32, std::collections::BTreeMap<Class, u32>> = Default::default();
    for c in chains {
        *turns.entry(c.t0 + 2).or_default().entry(c.class).or_insert(0) += 1;
    }
    turns.into_iter().map(|(t, m)| (t, m.into_iter().collect())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clock_follows_the_turn_structure() {
        assert_eq!(clock(3, 3, Phase::SpiritPhase), Clock::BeforeExplore);
        assert_eq!(clock(3, 3, Phase::SlowPower), Clock::AfterExplore);
        assert_eq!(clock(3, 4, Phase::FastPower), Clock::AfterExplore);
        assert_eq!(clock(3, 4, Phase::SlowPower), Clock::AfterBuild);
        assert_eq!(clock(3, 5, Phase::FastPower), Clock::AfterBuild);
        assert_eq!(clock(3, 5, Phase::SlowPower), Clock::AfterRavage);
        assert_eq!(clock(3, 6, Phase::FastPower), Clock::AfterRavage);
    }
}
