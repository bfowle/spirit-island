//! The Invader deck as a chance node. The deck's composition is known (the stage of
//! every position from the adversary's deck order; the cards of each stage from the
//! definitions) and it is drawn without replacement, so a card still in a stage is at
//! any of that stage's future draw positions with probability one over the cards left.
//! From that: which cards each future turn's Explore can be, and each land's chance of
//! an Explore, split into re-hits (the land already holds Invaders or Blight) and fresh
//! openings. Spec: docs/superpowers/specs/2026-10-20-decision-structure.md section 4.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::island::{LandId, Terrain};
use si_record::{GameRecord, TurnRecord};
use si_rules::invaders::{Island, Rules};

/// The stage each deck position draws from (position 0: the setup Explore) and the
/// offset that lines the record's turns up with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    pub stages: Vec<u8>,
    pub offset: i32,
    pub source: String,
}

fn parse_order(order: &str) -> Vec<u8> {
    order
        .chars()
        .filter_map(|c| match c {
            '1' => Some(1),
            '2' | 'C' | 'S' => Some(2),
            '3' => Some(3),
            _ => None,
        })
        .collect()
}

impl Deck {
    /// The deck for the game's adversary at `level` (the base deck without one), lined up
    /// with the turns whose Explore card the record shows: turn `u` draws position
    /// `u + offset`, the offset that matches the most recorded stages (0 when none match).
    pub fn for_game(rec: &GameRecord, defs: &Definitions, level: u8) -> Deck {
        let (stages, source) = match rec.meta.adversary.as_ref().and_then(|a| defs.adversaries.get(a)) {
            Some(a) => (a.stages_at(level), format!("{} level {level}: {}", a.title, a.deck_order_at(level))),
            None => (parse_order(si_model::setup::Adversary::BASE_DECK), format!("base deck {}", si_model::setup::Adversary::BASE_DECK)),
        };
        let recorded: Vec<(u32, u8)> = rec
            .turns
            .iter()
            .filter(|t| t.turn >= 1)
            .filter_map(|t| t.explore.first().and_then(|c| defs.invader_cards.get(c)).map(|c| (t.turn, c.stage)))
            .collect();
        let mut best = (0i32, 0usize);
        for off in [0i32, 1, -1, 2] {
            let hits = recorded.iter().filter(|(u, s)| stages.get((*u as i32 + off).max(0) as usize) == Some(s)).count();
            if hits > best.1 {
                best = (off, hits);
            }
        }
        Deck { stages, offset: best.0, source }
    }

    /// The stage turn `u`'s Explore draws from; None once the deck is out.
    pub fn stage_at(&self, turn: u32) -> Option<u8> {
        let i = turn as i32 + self.offset;
        if i < 0 {
            return None;
        }
        self.stages.get(i as usize).copied()
    }

    /// Cards left in the deck after turn `u`'s Explore.
    pub fn left_after(&self, turn: u32) -> usize {
        let i = (turn as i32 + self.offset + 1).max(0) as usize;
        self.stages.len().saturating_sub(i)
    }
}

/// Every Invader card the record shows drawn before turn `t`'s Explore: the earlier
/// turns' cards and this turn's face-up Ravage and Build (High Immigration included).
pub fn drawn_before_explore(rec: &GameRecord, t: &TurnRecord) -> BTreeSet<String> {
    let mut drawn: BTreeSet<String> = BTreeSet::new();
    for u in rec.turns.iter().filter(|u| u.turn < t.turn) {
        drawn.extend(u.explore.iter().cloned());
        drawn.extend(u.build.iter().cloned());
        drawn.extend(u.ravage.iter().cloned());
        drawn.extend(u.high_immigration.iter().cloned());
    }
    drawn.extend(t.build.iter().cloned());
    drawn.extend(t.ravage.iter().cloned());
    drawn.extend(t.high_immigration.iter().cloned());
    drawn
}

/// The cards turn `turn`'s Explore can be, as known at turn `t`'s Fast phase: the
/// cards of that turn's stage not drawn yet. Each is equally likely (`1 / len`), for
/// this turn and for any later turn of the same stage alike.
pub fn pool(rec: &GameRecord, defs: &Definitions, deck: &Deck, t: &TurnRecord, turn: u32) -> Option<(u8, Vec<String>)> {
    let stage = deck.stage_at(turn)?;
    let drawn = drawn_before_explore(rec, t);
    let mut v: Vec<String> = defs.invader_cards.iter().filter(|(id, c)| c.stage == stage && !drawn.contains(*id)).map(|(id, _)| id.clone()).collect();
    v.sort();
    Some((stage, v))
}

/// One land's chance of an Explore on one turn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandForecast {
    pub land: LandId,
    pub p: f64,
    /// The land already holds Invaders or Blight: a second wave on the first.
    pub rehit: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnForecast {
    pub turn: u32,
    pub stage: u8,
    /// The cards the Explore can be.
    pub cards: Vec<String>,
    /// Lands with any chance, most likely first.
    pub lands: Vec<LandForecast>,
    /// Expected lands explored a second time while still holding Invaders or Blight.
    pub rehits: f64,
    /// Expected lands opened fresh.
    pub fresh: f64,
}

/// The next `horizon` turns' Explores from turn `t`'s Fast phase, on `island` as the
/// board is projected to stand (this turn's face-up Ravage and Build run, no Explore).
/// A land can be explored when it is coastal or holds or neighbors a Town or City, is
/// not Isolated and holds no Wilds; which lands qualify is read off the projected board
/// for every turn (later Builds and Explores would widen it: an undercount for the
/// later turns). `this_turn`: the cards this turn's Explore can be, when the caller has
/// them (the record's own candidate list); else the deck's pool.
pub fn forecast(rec: &GameRecord, defs: &Definitions, deck: &Deck, t: &TurnRecord, island: &Island, rules: &Rules, horizon: u32, this_turn: Option<&[String]>) -> Vec<TurnForecast> {
    let mut out = Vec::new();
    for turn in t.turn..t.turn + horizon {
        let (stage, cards) = match (turn == t.turn, this_turn) {
            (true, Some(c)) if !c.is_empty() => (c.first().and_then(|id| defs.invader_cards.get(id)).map(|x| x.stage).or_else(|| deck.stage_at(turn)).unwrap_or(0), c.to_vec()),
            _ => match pool(rec, defs, deck, t, turn) {
                Some(p) => p,
                None => break,
            },
        };
        if cards.is_empty() {
            continue;
        }
        let p_card = 1.0 / cards.len() as f64;
        let mut lands: Vec<LandForecast> = Vec::new();
        for l in island.lands.values() {
            let Some(terrain) = l.terrain else { continue };
            if terrain == Terrain::Ocean {
                continue;
            }
            let reached = l.coastal || l.c.buildings() > 0 || island.buildings_adjacent(&l.id) > 0;
            if !reached || l.isolated || rules.isolated.contains(&l.id) || l.c.wilds > 0 || l.skip_explore {
                continue;
            }
            let p: f64 = cards.iter().filter_map(|id| defs.invader_cards.get(id)).filter(|c| c.matches(terrain, l.coastal)).count() as f64 * p_card;
            if p > 0.0 {
                lands.push(LandForecast { land: l.id.clone(), p, rehit: l.c.invaders() > 0 || l.c.blight > 0 });
            }
        }
        lands.sort_by(|a, b| b.p.partial_cmp(&a.p).unwrap().then(a.land.cmp(&b.land)));
        let rehits = lands.iter().filter(|l| l.rehit).map(|l| l.p).sum();
        let fresh = lands.iter().filter(|l| !l.rehit).map(|l| l.p).sum();
        out.push(TurnForecast { turn, stage, cards, lands, rehits, fresh });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_orders_parse_to_stages() {
        assert_eq!(parse_order("111-2222-33333"), vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3]);
        // Scotland: the Coastal Lands card (C) is a Stage II card at position 6
        assert_eq!(parse_order("11-22-1-C2-33333"), vec![1, 1, 2, 2, 1, 2, 2, 3, 3, 3, 3, 3]);
        // Russia 4+: a Stage III card after each Stage II card
        assert_eq!(parse_order("111-2-3-2-3-2-3-2-33"), vec![1, 1, 1, 2, 3, 2, 3, 2, 3, 2, 3, 3]);
    }

    #[test]
    fn turn_draws_its_position() {
        let d = Deck { stages: parse_order("111-2222-33333"), offset: 0, source: String::new() };
        // position 0 is the setup Explore; turn 1 draws position 1
        assert_eq!(d.stage_at(1), Some(1));
        assert_eq!(d.stage_at(2), Some(1));
        assert_eq!(d.stage_at(3), Some(2));
        assert_eq!(d.stage_at(7), Some(3));
        assert_eq!(d.stage_at(12), None);
        assert_eq!(d.left_after(1), 10);
        assert_eq!(d.left_after(11), 0);
        let r = Deck { stages: parse_order("111-2-3-2-3-2-3-2-33"), offset: 0, source: String::new() };
        assert_eq!((r.stage_at(3), r.stage_at(4), r.stage_at(5)), (Some(2), Some(3), Some(2)));
    }
}
