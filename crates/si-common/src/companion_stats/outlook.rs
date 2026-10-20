//! Lumia's timelines (reports/Spirit Island companion stat models.md,
//! "Timelines" and the race section's "invaders-only projection, enumerated
//! exactly over the next few cards"): the next Invader Phases played forward
//! for every order the unseen Invader cards can come in, with no Spirit
//! acting. The deck is a shuffle of each stage's unseen cards, so every
//! order is equally likely and a count of orders is a natural frequency.
//!
//! A floor on the pressure, not a prediction of the game: Spirits act,
//! cascades add their fewest Blight, and Events and Fear Cards are not drawn.
//! Each path records the first deterministic ending it reaches (an
//! adversary's loss condition at its check step, a Spirit with no Presence,
//! the deck running out); `race.rs` turns that into the competing-risks
//! incidence per ending.

use super::{adversary, invaders, phase_blight};
use crate::public::{PublicInvaderCard, PublicSnapshot};
use crate::rules;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Invader Phases played forward, this turn's (or next turn's) first.
pub const PHASES: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Outlook {
    /// The turns played forward.
    pub turns: Vec<u32>,
    /// Equally likely card orders in all: the natural-frequency base.
    pub orders: u64,
    /// Distinct outcomes, each with the number of orders that lead to it.
    pub paths: Vec<OutlookPath>,
    /// One row per kind of card the next unseen Explore card can be.
    pub next_cards: Vec<NextCard>,
    /// Per turn: orders in which the Blight Card has flipped by then (0s
    /// once it is already flipped).
    pub flipped_by: Vec<u64>,
    /// The turn whose Explore finds the Invader deck empty, if it comes
    /// within the turns played forward.
    pub deck_out: Option<u32>,
    pub blight_left: i64,
    pub flipped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutlookPath {
    pub orders: u64,
    /// Which `next_cards` row these orders start with.
    pub first: Option<usize>,
    /// Per turn: Blight the Invader Phase adds (fewest cascades).
    pub blight: Vec<i64>,
    /// Per turn, after the Build: Explorers, Towns, Cities on the island.
    pub invaders: Vec<[u32; 3]>,
    /// Per turn: lands that Ravage, Build, Explore.
    pub lands: Vec<[u32; 3]>,
    /// The first deterministic ending this path reaches, if any.
    #[serde(default)]
    pub end: Option<PathEnd>,
    /// Per turn: Dahan the Ravages destroy.
    #[serde(default)]
    pub dahan_lost: Vec<u32>,
    /// Per turn, after the Build: lands with a Town or City (Explore sources).
    #[serde(default)]
    pub sources: Vec<u32>,
    /// Per turn, after the Build: lands with any Invader.
    #[serde(default)]
    pub invaded: Vec<u32>,
    /// Per turn: Ravages that add Blight.
    #[serde(default)]
    pub blighting: Vec<u32>,
}

/// One Invader action stopped, for a counterfactual walk (`chain.rs`): the
/// Ravage in a land does no damage, the piece a Build adds is gone before
/// it acts, the Explorer an Explore places is gone before it Builds.
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub land: String,
    /// "Explore", "Build", "Ravage".
    pub step: String,
    pub turn: u32,
}

/// An ending a path reaches: which, and when.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PathEnd {
    /// "clock:<adversary id>", "presence:<spirit id>", "deck".
    pub key: String,
    pub title: String,
    pub turn: u32,
    /// "Fast", "Ravage", "Build", "Explore".
    pub step: String,
}

impl OutlookPath {
    /// Keep the earliest ending.
    fn note_end(&mut self, e: PathEnd) {
        let rank = |x: &PathEnd| (x.turn, super::race::step_rank(&x.step));
        if self.end.as_ref().is_none_or(|cur| rank(&e) < rank(cur)) {
            self.end = Some(e);
        }
    }

    /// Blight added through each turn.
    pub fn cumulative(&self) -> Vec<i64> {
        self.blight.iter().scan(0, |t, b| {
            *t += b;
            Some(*t)
        }).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NextCard {
    pub card: PublicInvaderCard,
    /// Unseen cards of this kind, out of the stage's unseen cards.
    pub count: u32,
    pub of: u32,
    /// The turn it Explores.
    pub turn: u32,
    pub explores: Vec<String>,
    /// Per turn played forward: fewest and most Blight added, across the
    /// cards that can follow it.
    pub blight: Vec<(i64, i64)>,
    /// Orders that start with this card.
    pub orders: u64,
}

struct Leaf {
    weight: u64,
    first: Option<usize>,
    trail: OutlookPath,
}

struct Walk<'a> {
    nbrs: BTreeMap<&'a str, BTreeSet<&'a str>>,
    leaves: Vec<Leaf>,
    firsts: Vec<NextCard>,
    turns: Vec<u32>,
    deck_out: Option<u32>,
    stop: Option<&'a Stop>,
}

/// Cards that play the same: same stage, terrains and Escalation flag.
fn kind(c: &PublicInvaderCard) -> (u8, Vec<String>, bool) {
    let mut t = c.terrains.clone();
    t.sort();
    (c.stage, t, c.escalation)
}

pub fn outlook(s: &PublicSnapshot) -> Outlook {
    outlook_with(s, None)
}

/// The outlook with one action stopped on every path it happens on.
pub fn outlook_with(s: &PublicSnapshot, stop: Option<&Stop>) -> Outlook {
    let mut root = if invaders::phase_position(&s.step) == "after" { invaders::next_turn_start(s) } else { s.clone() };
    let stages = invaders::forecast_at(&root).stages;
    let pools: [Vec<PublicInvaderCard>; 3] = [0usize, 1, 2].map(|i| stages.get(i).map(|st| st.unseen.clone()).unwrap_or_default());
    let mut w = Walk { nbrs: rules::neighbors(s), leaves: Vec::new(), firsts: Vec::new(), turns: Vec::new(), deck_out: None, stop };
    w.stop_ravage(&mut root);
    w.walk(root, 0, pools, 1, None, OutlookPath::default());

    let orders: u64 = w.leaves.iter().map(|l| l.weight).sum();
    let n = w.turns.len();
    let b = &s.blight;
    let mut flipped_by = vec![0u64; n];
    let mut merged: Vec<OutlookPath> = Vec::new();
    for leaf in &w.leaves {
        let cum = leaf.trail.cumulative();
        for j in 0..n {
            // A path cut short by an empty deck keeps its last count.
            let k = cum.get(j).or(cum.last()).copied().unwrap_or(0);
            if !b.flipped && b.remaining > 0 && k >= b.remaining {
                flipped_by[j] += leaf.weight;
            }
        }
        let same = |p: &&mut OutlookPath| {
            p.first == leaf.first
                && p.end == leaf.trail.end
                && p.blight == leaf.trail.blight
                && p.invaders == leaf.trail.invaders
                && p.lands == leaf.trail.lands
                && p.dahan_lost == leaf.trail.dahan_lost
                && p.sources == leaf.trail.sources
                && p.invaded == leaf.trail.invaded
                && p.blighting == leaf.trail.blighting
        };
        match merged.iter_mut().find(same) {
            Some(p) => p.orders += leaf.weight,
            None => merged.push(OutlookPath { orders: leaf.weight, first: leaf.first, ..leaf.trail.clone() }),
        }
        if let Some(f) = leaf.first.and_then(|i| w.firsts.get_mut(i)) {
            f.orders += leaf.weight;
            for (j, add) in leaf.trail.blight.iter().enumerate() {
                match f.blight.get_mut(j) {
                    Some(r) => *r = (r.0.min(*add), r.1.max(*add)),
                    None => f.blight.push((*add, *add)),
                }
            }
        }
    }
    merged.sort_by_key(|p| std::cmp::Reverse(p.orders));
    Outlook {
        turns: w.turns,
        orders,
        paths: merged,
        next_cards: w.firsts,
        flipped_by,
        deck_out: w.deck_out,
        blight_left: b.remaining,
        flipped: b.flipped,
    }
}

impl<'a> Walk<'a> {
    /// A stopped Ravage: Defend beyond any damage, in the turn it is checked.
    fn stop_ravage(&self, s: &mut PublicSnapshot) {
        if let Some(st) = self.stop.filter(|st| st.step == "Ravage" && st.turn == s.turn) {
            if let Some(l) = s.lands.iter_mut().find(|l| l.id == st.land) {
                l.defend = 999;
            }
        }
    }

    /// A stopped Build or Explore: the piece it added this turn is gone from
    /// next turn's board.
    fn stop_piece(&self, view: &invaders::InvaderView, next: &mut PublicSnapshot) {
        let Some(st) = self.stop.filter(|st| st.turn == view.phase.turn) else { return };
        let kind = match st.step.as_str() {
            "Build" => view.phase.builds.iter().find(|b| b.land == st.land && !b.done).map(|b| b.adds.clone()),
            "Explore" => view
                .phase
                .explore
                .candidates
                .iter()
                .find(|_| view.phase.explore.revealed.is_some())
                .filter(|c| c.lands.contains(&st.land))
                .map(|_| "Explorer".to_string()),
            _ => None,
        };
        let (Some(kind), Some(l)) = (kind, next.lands.iter_mut().find(|l| l.id == st.land)) else { return };
        match kind.as_str() {
            "Explorer" => l.explorers = l.explorers.saturating_sub(1),
            "Town" => l.towns = l.towns.saturating_sub(1),
            "City" => l.cities = l.cities.saturating_sub(1),
            _ => return,
        }
        if let Some(i) = l.pieces.iter().position(|p| p.kind == kind) {
            l.pieces.remove(i);
        }
    }

    fn walk(&mut self, s: PublicSnapshot, phase: usize, pools: [Vec<PublicInvaderCard>; 3], weight: u64, first: Option<usize>, trail: OutlookPath) {
        if phase == PHASES {
            self.leaves.push(Leaf { weight, first, trail });
            return;
        }
        let inv = &s.invaders;
        // The last phase's Explore card adds no Blight within the turns
        // played forward, so it isn't branched on.
        let branch = inv.explore.is_empty() && phase + 1 < PHASES;
        let stage = inv.upcoming_stages.first().map(|st| (*st as usize).clamp(1, 3) - 1);
        if let (true, Some(st)) = (branch, stage) {
            let pool = &pools[st];
            if !pool.is_empty() {
                let mut kinds: Vec<(PublicInvaderCard, u64)> = Vec::new();
                for c in pool {
                    match kinds.iter_mut().find(|(k, _)| kind(k) == kind(c)) {
                        Some((_, n)) => *n += 1,
                        None => kinds.push((c.clone(), 1)),
                    }
                }
                for (card, n) in kinds {
                    let mut next = s.clone();
                    next.invaders.explore = vec![card.clone()];
                    next.invaders.upcoming_stages.remove(0);
                    let mut rest = pools.clone();
                    if let Some(i) = rest[st].iter().position(|c| kind(c) == kind(&card)) {
                        rest[st].remove(i);
                    }
                    let first = first.or_else(|| {
                        self.firsts.push(NextCard { card: card.clone(), count: n as u32, of: pool.len() as u32, turn: s.turn, ..Default::default() });
                        Some(self.firsts.len() - 1)
                    });
                    self.step(next, phase, rest, weight * n, first, trail.clone());
                }
                return;
            }
        }
        self.step(s, phase, pools, weight, first, trail);
    }

    fn step(&mut self, s: PublicSnapshot, phase: usize, pools: [Vec<PublicInvaderCard>; 3], weight: u64, first: Option<usize>, mut trail: OutlookPath) {
        let view = invaders::forecast_at(&s);
        if self.turns.len() == phase {
            self.turns.push(view.phase.turn);
        }
        let blight = phase_blight(&view.phase, &s.lands, &self.nbrs).0;
        let pieces = view.after_build.iter().filter(|l| !l.is_ocean).fold([0u32; 3], |a, l| [a[0] + l.explorers, a[1] + l.towns, a[2] + l.cities]);
        let explored = view.phase.explore.revealed.as_ref().and(view.phase.explore.candidates.first()).map(|c| c.lands.clone()).unwrap_or_default();
        let ravaging = view.phase.ravages.iter().chain(&view.phase.build_ravages).filter(|r| !r.done).count() as u32;
        let building = view.phase.builds.iter().filter(|b| !b.done).count() as u32;
        let dahan_lost = view.phase.ravages.iter().chain(&view.phase.build_ravages).filter(|r| !r.done).map(|r| r.ledger.dahan_destroyed).sum();
        trail.blight.push(blight);
        trail.invaders.push(pieces);
        trail.lands.push([ravaging, building, explored.len() as u32]);
        trail.dahan_lost.push(dahan_lost);
        trail.sources.push(view.after_build.iter().filter(|l| !l.is_ocean && l.towns + l.cities > 0).count() as u32);
        trail.invaded.push(view.after_build.iter().filter(|l| !l.is_ocean && rules::has_invaders(l)).count() as u32);
        trail.blighting.push(view.phase.ravages.iter().chain(&view.phase.build_ravages).filter(|r| !r.done && r.ledger.adds_blight).count() as u32);
        // The adversary's loss conditions on this path, at their check step.
        // A path's snapshot sits at the Spirit Phase, before the Fast check;
        // the root may sit later in its turn.
        let turn = view.phase.turn;
        let before_fast_check = matches!(s.step.as_str(), "PlayerSetup" | "InvadersStartingAction" | "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower" | "FastPower");
        for c in adversary::loss_clocks(&adversary::Adv::new(&s), &view.after_build, &view.phase.ravages, &view.phase.build_ravages) {
            let fires = |v: i64| if c.trigger == "over" { v > c.limit } else { v >= c.limit };
            let after = c.after.unwrap_or(c.now);
            let end = if c.deferred {
                if fires(c.now) && before_fast_check {
                    Some((turn, "Fast"))
                } else if fires(after) {
                    Some((turn + 1, "Fast"))
                } else {
                    None
                }
            } else if fires(after) {
                Some((turn, c.check_step.as_str()))
            } else {
                None
            };
            if let Some((t, step)) = end {
                trail.note_end(PathEnd { key: format!("clock:{}", c.id), title: c.title.clone(), turn: t, step: step.into() });
            }
        }
        // The first unseen card: which lands it Explores.
        if let Some(f) = first.and_then(|i| self.firsts.get_mut(i)) {
            if f.turn == view.phase.turn && f.explores.is_empty() {
                f.explores = explored;
            }
        }
        if s.invaders.explore.is_empty() && s.invaders.upcoming_stages.is_empty() {
            self.deck_out = Some(turn);
            trail.note_end(PathEnd { key: "deck".into(), title: "Invader deck runs out".into(), turn, step: "Explore".into() });
            self.leaves.push(Leaf { weight, first, trail });
            return;
        }
        match invaders::advance(&s, &view) {
            Some(mut next) => {
                self.stop_piece(&view, &mut next);
                self.stop_ravage(&mut next);
                // A Spirit whose last Presence this turn's Blight destroyed.
                for sp in &next.spirits {
                    let had: u32 = s.lands.iter().map(|l| l.presence.get(&sp.id).copied().unwrap_or(0)).sum();
                    let has: u32 = next.lands.iter().map(|l| l.presence.get(&sp.id).copied().unwrap_or(0)).sum();
                    if had > 0 && has == 0 {
                        trail.note_end(PathEnd { key: format!("presence:{}", sp.id), title: format!("{} has no Presence", sp.title), turn, step: "Ravage".into() });
                    }
                }
                self.walk(next, phase + 1, pools, weight, first, trail)
            }
            None => self.leaves.push(Leaf { weight, first, trail }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{PublicBoard, PublicInvaderDeck, PublicLand, PublicSpirit};

    fn card(stage: u8, t: &str) -> PublicInvaderCard {
        PublicInvaderCard { id: format!("{stage}{t}"), stage, terrains: vec![t.into()], ..Default::default() }
    }

    /// One coastal board, a Town in each Jungle: the Stage I cards that can
    /// come next (Wetland, Sands, Mountain unseen; Jungle in Build).
    fn snap() -> PublicSnapshot {
        let terrains = ["Jungle", "Wetland", "Sands", "Mountain", "Jungle", "Wetland", "Sands", "Mountain"];
        let lands: Vec<PublicLand> = terrains
            .iter()
            .enumerate()
            .map(|(i, t)| PublicLand {
                id: format!("A{}", i + 1),
                board: "A".into(),
                number: i as i64 + 1,
                terrain: t.to_string(),
                coastal: i < 3,
                towns: u32::from(*t == "Jungle"),
                ..Default::default()
            })
            .collect();
        let ids: Vec<String> = lands.iter().map(|l| l.id.clone()).collect();
        let adjacency = ids.windows(2).map(|w| (w[0].clone(), w[1].clone())).collect();
        let mut s = PublicSnapshot {
            game_id: "g".into(),
            turn: 2,
            step: "SpiritPhase".into(),
            lands,
            adjacency,
            boards: vec![PublicBoard { id: "A".into(), lands: ids }],
            spirits: vec![PublicSpirit::default()],
            ..Default::default()
        };
        s.blight.remaining = 3;
        s.invaders = PublicInvaderDeck {
            upcoming_stages: vec![1, 1, 2, 2],
            build: vec![card(1, "Jungle")],
            pool: ["Jungle", "Wetland", "Sands", "Mountain"].iter().map(|t| card(1, t)).chain(["Jungle", "Wetland"].iter().map(|t| card(2, t))).collect(),
            ..Default::default()
        };
        s
    }

    #[test]
    fn every_order_counted_once() {
        let o = outlook(&snap());
        // Turn 2 draws one of 3 unseen Stage I cards, Turn 3 one of the
        // other 2, Turn 4 one of 2 Stage II: 3 × 2 × 2 orders.
        assert_eq!(o.orders, 12);
        assert_eq!(o.turns, vec![2, 3, 4, 5]);
        assert_eq!(o.next_cards.len(), 3);
        assert!(o.next_cards.iter().all(|c| (c.count, c.of, c.orders) == (1, 3, 4)));
        assert_eq!(o.paths.iter().map(|p| p.orders).sum::<u64>(), 12);
        // Turn 3 Ravages the Jungles (a Town each, 2 Damage): 2 Blight in every order.
        assert!(o.paths.iter().all(|p| p.blight[1] == 2));
        assert!(o.flipped_by.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn deck_out_ends_the_walk() {
        let mut s = snap();
        s.invaders.upcoming_stages = vec![1];
        let o = outlook(&s);
        assert_eq!(o.deck_out, Some(3));
        assert_eq!(o.orders, 3);
    }
}
