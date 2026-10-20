//! The chain (Brett, 2026-10-19: "if you prevent a Build that clears up
//! stuff for Ravage next phase; if you prevent an Explore it prevents a
//! Build and Ravage the next turns. It's exponential"). One Invader card
//! walks Explore → Build → Ravage over three turns on the same lands, and
//! every Town it builds is a source that seeds Explorers into its
//! neighbors. So the weight of an Invader action is the chain it starts,
//! not a constant. Here the chain is measured by the engine: the outlook
//! walked again with that one action stopped, and the difference counted
//! in actions, Blight and Dahan over the turns played forward, plus the
//! chains a source would still seed after them.
//!
//! Pace in one unit on both sides: the Invaders' pressure a turn is their
//! actions weighted by the chains they start; the Spirits' capacity a turn
//! is the chain value of the actions their plays can stop. The backlog is
//! what was started and not stopped; the crossing is the turn capacity
//! first meets pressure.

use super::curve::Curve;
use super::outlook::{self, Outlook, Stop};
use super::{invaders, needs, RoundSummary};
use crate::public::{PublicInnate, PublicLand, PublicPowerCard, PublicSnapshot, PublicSpirit};
use crate::rules;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A Blight's worth in actions, and a Dahan's.
pub const BLIGHT_WORTH: f64 = 1.0;
pub const DAHAN_WORTH: f64 = 0.5;
/// A point of Invader health left on the board: a Town (2) is about one
/// play to clear, an Explorer (1) half of one, a City (3) one and a half.
pub const PIECE_WORTH: f64 = 0.5;
/// A chain a source seeds after the turns played forward: its Explore,
/// Build, Ravage and Blight (depth one; what that chain seeds is not counted).
const SEEDED_CHAIN: f64 = 3.0 + BLIGHT_WORTH;
/// Chain values when this turn has no action of a step to measure:
/// Explore → Build → Ravage → Blight; Build → Ravage → Blight; Ravage → Blight.
const DEFAULT_WEIGHTS: [f64; 3] = [3.0 + BLIGHT_WORTH, 2.0 + BLIGHT_WORTH, 1.0 + BLIGHT_WORTH];

pub const POLICY: &str = "an action's value = the Explores and Builds, Blight-adding Ravages (1 each, cascades not counted), Dahan (½) and Invader health (½ a point) it plants over the next 4 Invader Phases with no Spirit action, measured by stopping it alone, plus the chains (3 actions + 1 Blight each) a Town it leaves would seed into empty neighbors by the end of the deck; a stopped Ravage is one Defended past its damage; pressure beyond the turns played forward uses the stage averages; capacity = the best legal plays this turn against this turn's Ravages and Builds and next turn's Ravages, then plays a turn along the Card Plays track at this turn's value per play, a Reclaim turn gaining no card";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Chain {
    pub turn: u32,
    pub orders: u64,
    /// This turn's Invader actions, each with the chain it starts.
    pub actions: Vec<ActionChain>,
    /// Mean chain value of an Explore, Build, Ravage on this board.
    pub weights: [f64; 3],
    /// Turn 1 through the turn the deck runs out.
    pub turns: Vec<PaceTurn>,
    pub sources: Sources,
    /// What this turn's plays can stop, per Spirit.
    pub capacity_now: Vec<CapacityNow>,
    /// The first turn, this one or later, whose capacity meets its pressure.
    pub crossing: Option<u32>,
    /// The win from the board: what still stands against a board clear at
    /// this Terror Level, and how fast pieces have been leaving.
    pub clear: ClearPace,
    /// The earliest way the Spirits win at their pace: ("outpace" | "board
    /// clear" | "Fear victory", turn).
    pub win: Option<(String, u32)>,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClearPace {
    pub level: u8,
    /// Invaders that block the board clear at this Terror Level.
    pub standing: u32,
    /// Pieces removed a turn, and pieces arriving a turn (Explores and
    /// Builds), over the last three finished turns.
    pub removed_per_turn: f64,
    pub arrived_per_turn: f64,
    /// The turn the standing pieces are gone at the net pace, if ever.
    pub arrives: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActionChain {
    pub land: String,
    pub terrain: String,
    /// "Explore", "Build", "Ravage".
    pub step: String,
    pub turn: u32,
    /// Cards that make it happen, of the candidates ((1, 1) when certain).
    pub chance: (u32, u32),
    /// The chain it starts, given it happens.
    pub value: f64,
    pub actions: f64,
    pub blight: f64,
    pub dahan: f64,
    /// Invader health left on the board at the end of the turns played forward.
    pub pieces: f64,
    /// Explore sources left at the end of the turns played forward.
    pub sources: f64,
    pub tail: f64,
    /// What it plants, by turn: "T4 Build 1.0", "T5 Ravage 1.0", "T5 Explore 0.4".
    pub plants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaceTurn {
    pub turn: u32,
    /// Lands acted in: Explore, Build, Ravage (a mean where cards are unseen).
    pub lands: [f64; 3],
    pub pressure: f64,
    pub capacity: f64,
    pub plays: i64,
    pub reclaim: bool,
    /// Pressure started and not stopped, after this turn.
    pub backlog: f64,
    /// pressure ÷ capacity.
    pub ratio: Option<f64>,
    /// "played", "over N orders", "stage average".
    pub basis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Sources {
    /// Lands with a Town or City now.
    pub now: u32,
    /// Lands (Oceans excluded).
    pub lands: u32,
    /// Lands with no Invaders.
    pub empty: u32,
    /// New sources one source makes per cycle: the chance the next card
    /// names each empty neighbor, summed, averaged over the sources.
    pub r: f64,
    /// 1 + r.
    pub factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapacityNow {
    pub spirit: String,
    pub plays: i64,
    pub energy: i64,
    /// Chain value the best legal set of plays stops.
    pub best: f64,
    /// (card or innate, "Ravage C4" / "Build E1" / "Ravage C7 next turn", value).
    pub cuts: Vec<(String, String, f64)>,
    pub per_play: f64,
}

/// Orders-weighted totals over an outlook.
#[derive(Default, Clone)]
struct Agg {
    actions: f64,
    blight: f64,
    dahan: f64,
    sources_end: f64,
    /// Invader health on the board at the end: Explorers 1, Towns 2, Cities 3.
    health_end: f64,
    /// Per outlook turn: Explore, Build, Ravage lands.
    per_turn: Vec<[f64; 3]>,
}

/// Orders-weighted totals over an outlook. Blight counts the Ravages that
/// add it, not the cascades they set off: on a Blighted island every
/// cascade runs through the whole island, and that is the Blight card's
/// race to tell, not a weight on one action.
fn agg(o: &Outlook) -> Agg {
    let total = o.orders.max(1) as f64;
    let mut a = Agg { per_turn: vec![[0.0; 3]; o.turns.len()], ..Default::default() };
    for p in &o.paths {
        let w = p.orders as f64 / total;
        for (j, l) in p.lands.iter().enumerate() {
            // Path arrays are [Ravage, Build, Explore]; the Explores and
            // Builds are the actions planted (a Ravage's part is its Blight).
            a.per_turn[j][0] += w * l[2] as f64;
            a.per_turn[j][1] += w * l[1] as f64;
            a.per_turn[j][2] += w * l[0] as f64;
            a.actions += w * (l[1] + l[2]) as f64;
        }
        a.blight += w * p.blighting.iter().sum::<u32>() as f64;
        a.dahan += w * p.dahan_lost.iter().sum::<u32>() as f64;
        a.sources_end += w * p.sources.last().copied().unwrap_or(0) as f64;
        a.health_end += w * p.invaders.last().map(|[x, t, c]| (x + 2 * t + 3 * c) as f64).unwrap_or(0.0);
    }
    a
}

/// The chance the next card of `stage` names this land.
fn names(stage: u8, l: &PublicLand, stages: &[invaders::StageCount]) -> f64 {
    let Some(st) = stages.iter().find(|x| x.stage == stage) else { return 0.0 };
    let pool = if st.unseen.is_empty() { return 0.0 } else { &st.unseen };
    pool.iter().filter(|c| rules::card_matches(c, l)).count() as f64 / pool.len() as f64
}

pub fn chain(s: &PublicSnapshot, history: &[RoundSummary], curve: &Curve, base: &Outlook, fear: &super::pace::FearPace) -> Chain {
    let view = invaders::forecast(s);
    let turn = view.phase.turn;
    let nbrs = rules::neighbors(s);
    let land = |id: &str| s.lands.iter().find(|l| l.id == id);
    let base_agg = agg(base);

    // The stages the outlook did not draw: what a source this action leaves
    // would still seed after the turns played forward. Each empty neighbor
    // no other source feeds, Explored at least once by the later cards.
    let drawn = (outlook::PHASES - 1).min(s.invaders.upcoming_stages.len());
    let later: Vec<u8> = s.invaders.upcoming_stages[drawn..].to_vec();
    let seeds_after = |l: &PublicLand| -> f64 {
        nbrs.get(l.id.as_str())
            .into_iter()
            .flatten()
            .filter_map(|n| land(n))
            .filter(|x| !x.is_ocean && !rules::has_invaders(x))
            .filter(|x| view.lands.iter().find(|v| v.id == x.id).is_none_or(|v| v.fed_by.iter().all(|f| *f == l.id)))
            .map(|x| 1.0 - later.iter().map(|st| 1.0 - names(*st, x, &view.stages)).product::<f64>())
            .sum::<f64>()
            * SEEDED_CHAIN
    };

    // This turn's actions.
    let mut todo: Vec<(String, &str, (u32, u32))> = Vec::new();
    for r in view.phase.ravages.iter().chain(&view.phase.build_ravages).filter(|r| !r.done) {
        todo.push((r.land.clone(), "Ravage", (1, 1)));
    }
    for b in view.phase.builds.iter().filter(|b| !b.done) {
        todo.push((b.land.clone(), "Build", (1, 1)));
    }
    for (id, (n, d)) in &view.phase.explore.per_land {
        if *n > 0 {
            todo.push((id.clone(), "Explore", (*n, *d)));
        }
    }
    let mut actions = Vec::new();
    for (id, step, chance) in todo {
        let Some(l) = land(&id) else { continue };
        let cf = agg(&outlook::outlook_with(s, Some(&Stop { land: id.clone(), step: step.into(), turn })));
        let q = if chance.1 == 0 { 1.0 } else { chance.0 as f64 / chance.1 as f64 };
        let d = |a: f64, b: f64| ((a - b) / q).max(0.0);
        let (da, db, dd, ds, dp) = (
            d(base_agg.actions, cf.actions),
            d(base_agg.blight, cf.blight),
            d(base_agg.dahan, cf.dahan),
            d(base_agg.sources_end, cf.sources_end),
            d(base_agg.health_end, cf.health_end),
        );
        let tail = ds * seeds_after(l);
        let mut plants = Vec::new();
        for (j, t) in base.turns.iter().enumerate() {
            for (k, name) in ["Explore", "Build"].iter().enumerate() {
                let x = (base_agg.per_turn[j][k] - cf.per_turn.get(j).map(|p| p[k]).unwrap_or(0.0)) / q;
                if x > 0.05 {
                    plants.push(format!("T{t} {name} {x:.1}"));
                }
            }
        }
        if db > 0.05 {
            plants.push(format!("Blight {db:.1}"));
        }
        if dd > 0.05 {
            plants.push(format!("Dahan {dd:.1}"));
        }
        if dp > 0.05 {
            plants.push(format!("Invader health {dp:.1}"));
        }
        if tail > 0.05 {
            plants.push(format!("seeds after T{} {tail:.1}", base.turns.last().copied().unwrap_or(turn)));
        }
        actions.push(ActionChain {
            land: id,
            terrain: l.terrain.clone(),
            step: step.into(),
            turn,
            chance,
            value: da + BLIGHT_WORTH * db + DAHAN_WORTH * dd + PIECE_WORTH * dp + tail,
            actions: da,
            blight: db,
            dahan: dd,
            pieces: dp,
            sources: ds,
            tail,
            plants,
        });
    }
    actions.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal));

    // The derived weights: the mean chain value per step on this board.
    let mut weights = DEFAULT_WEIGHTS;
    for (k, name) in ["Explore", "Build", "Ravage"].iter().enumerate() {
        let of: Vec<&ActionChain> = actions.iter().filter(|a| a.step == *name).collect();
        let w: f64 = of.iter().map(|a| a.chance.0 as f64 / a.chance.1.max(1) as f64).sum();
        if w > 0.0 {
            weights[k] = of.iter().map(|a| a.value * a.chance.0 as f64 / a.chance.1.max(1) as f64).sum::<f64>() / w;
        }
    }

    // Sources and their reproduction.
    let island: Vec<&PublicLand> = s.lands.iter().filter(|l| !l.is_ocean).collect();
    let src: Vec<&PublicLand> = island.iter().copied().filter(|l| l.towns + l.cities > 0).collect();
    let next_stage = s.invaders.upcoming_stages.first().copied().unwrap_or(0);
    let r = if src.is_empty() {
        0.0
    } else {
        src.iter()
            .map(|l| {
                nbrs.get(l.id.as_str())
                    .into_iter()
                    .flatten()
                    .filter_map(|n| land(n))
                    .filter(|x| !x.is_ocean && !rules::has_invaders(x))
                    .map(|x| names(next_stage, x, &view.stages))
                    .sum::<f64>()
            })
            .sum::<f64>()
            / src.len() as f64
    };
    let sources = Sources {
        now: src.len() as u32,
        lands: island.len() as u32,
        empty: island.iter().filter(|l| !rules::has_invaders(l)).count() as u32,
        r,
        factor: 1.0 + r,
    };

    // Capacity now: the best legal plays against the actions on the table.
    let next = invaders::next_turn(s, &view);
    let capacity_now: Vec<CapacityNow> = s.spirits.iter().map(|sp| capacity_now(sp, s, &view, next.as_ref(), &actions, &weights)).collect();
    let weighted = |l: &[f64; 3]| l[0] * weights[0] + l[1] * weights[1] + l[2] * weights[2];
    // What the Spirits stopped each finished turn: the Fast Phase (set to
    // act at the turn's start, less what happened) and the Slow Phase (next
    // turn as the cards advanced, less next turn's start).
    // Explores and Builds count when they did not happen; a Ravage counts
    // as stopped when it no longer adds Blight (a Defend or a kill).
    let stopped = |t: u32| -> Option<f64> {
        let h = history.iter().find(|h| h.turn == t)?;
        let fast: f64 = (0..2).map(|k| (h.forecast_start[k] - h.actual[k]).max(0.0) * weights[k]).sum::<f64>() + (h.blighting[0] - h.blighting[1]).max(0.0) * weights[2];
        let slow: f64 = history
            .iter()
            .find(|n| n.turn == t + 1)
            .map(|n| (0..2).map(|k| (h.forecast_next[k] - n.forecast_start[k]).max(0.0) * weights[k]).sum::<f64>() + (h.blighting[2] - n.blighting[0]).max(0.0) * weights[2])
            .unwrap_or(0.0);
        Some(fast + slow)
    };
    let finished: Vec<(f64, i64)> = history.iter().filter(|h| h.turn < turn).filter_map(|h| stopped(h.turn).map(|v| (v, h.plays))).collect();
    // Value a play: the last three finished turns, else this turn's best.
    let per_play: f64 = {
        let recent: Vec<&(f64, i64)> = finished.iter().rev().take(3).collect();
        let plays: i64 = recent.iter().map(|x| x.1).sum();
        let so_far = recent.iter().map(|x| x.0).sum::<f64>();
        if plays > 0 && so_far > 0.0 {
            so_far / plays as f64
        } else {
            capacity_now.iter().map(|c| c.best).sum::<f64>() / capacity_now.iter().map(|c| c.plays as f64).sum::<f64>().max(1.0)
        }
    };

    // Beyond the turns played forward the cards name lands; only those with
    // Invaders Build or Ravage, only fed ones Explore.
    let invaded_end: f64 = base.paths.iter().map(|p| p.orders as f64 * p.invaded.last().copied().unwrap_or(0) as f64).sum::<f64>() / base.orders.max(1) as f64;
    let share_invaded = (invaded_end / island.len().max(1) as f64).clamp(0.0, 1.0);
    let share_fed = view.lands.iter().filter(|l| matches!(l.explore_access.as_str(), "coastal" | "fed")).count() as f64 / island.len().max(1) as f64;

    // The pace, turn by turn: pressure from the cards, capacity from the
    // tracks. Each Spirit's cards follow the Card Plays track, one presence
    // a turn, with a Reclaim when the hand is short of the plays (no card
    // gained that turn).
    let before_plays = matches!(s.step.as_str(), "PlayerSetup" | "InvadersStartingAction" | "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower");
    let plays_now: i64 = capacity_now.iter().map(|c| c.plays).sum();
    struct Deck {
        hand: i64,
        discard: i64,
    }
    let mut decks: Vec<Deck> = s
        .spirits
        .iter()
        .zip(&capacity_now)
        .map(|(sp, c)| {
            let now = if before_plays { c.plays } else { 0 };
            Deck { hand: sp.hand.len() as i64 - now, discard: (sp.discard.len() + sp.play_area.len()) as i64 + now }
        })
        .collect();
    let mut backlog = 0.0;
    let mut turns = Vec::new();
    for ct in &curve.turns {
        let t = ct.turn;
        let played = history.iter().find(|h| h.turn == t).filter(|h| t < turn && h.actual != [0.0; 3]);
        let (lands, basis) = match (played, base.turns.iter().position(|x| *x == t)) {
            (Some(h), _) => (h.actual, "played".to_string()),
            (None, Some(j)) => {
                // The outlook does not draw its last phase's Explore card (it
                // adds no Blight within the turns played forward): that
                // turn's Explore is the stage average, like the turns after.
                let mut lands = base_agg.per_turn[j];
                if j + 1 == base.turns.len() && lands[0] == 0.0 && base.deck_out.is_none_or(|d| d > t) {
                    lands[0] = ct.explore.lands * share_fed;
                }
                (lands, format!("over {} orders", base.orders))
            }
            (None, None) if t < turn => ([ct.explore.lands, ct.build.lands, ct.ravage.lands], "cards".to_string()),
            (None, None) => ([ct.explore.lands * share_fed, ct.build.lands * share_invaded, ct.ravage.lands * share_invaded], "stage average".to_string()),
        };
        let pressure = weighted(&lands);
        let (plays, reclaim) = if t < turn {
            (history.iter().find(|h| h.turn == t).map(|h| h.plays).unwrap_or(0), false)
        } else if t == turn {
            (plays_now, false)
        } else {
            let k = (t - turn) as usize;
            let mut total = 0;
            let mut any_reclaim = false;
            for (sp, d) in s.spirits.iter().zip(decks.iter_mut()) {
                let p = super::curve::projected(sp, "CardPlaysPerTurn", k).1;
                d.hand += 1;
                let reclaim = d.hand < p;
                if reclaim {
                    d.hand += d.discard - 1;
                    d.discard = 0;
                }
                let played = p.min(d.hand).max(0);
                d.hand -= played;
                d.discard += played;
                total += played;
                any_reclaim |= reclaim;
            }
            (total, any_reclaim)
        };
        let capacity = if t < turn {
            stopped(t).unwrap_or(0.0)
        } else if t == turn {
            capacity_now.iter().map(|c| c.best).sum::<f64>().max(plays as f64 * per_play)
        } else {
            plays as f64 * per_play
        };
        backlog = (backlog + pressure - capacity).max(0.0);
        turns.push(PaceTurn {
            turn: t,
            lands,
            pressure,
            capacity,
            plays,
            reclaim,
            backlog,
            ratio: (capacity > 0.0).then(|| pressure / capacity),
            basis,
        });
    }
    let crossing = turns.iter().find(|x| x.turn >= turn && x.capacity >= x.pressure).map(|x| x.turn);

    // The board-clear win: pieces leaving against pieces arriving, over the
    // finished turns, against what still stands at this Terror Level.
    let level = s.fear.terror_level.max(1);
    let (x, t, c) = island.iter().fold((0, 0, 0), |(x, t, c), l| (x + l.explorers, t + l.towns, c + l.cities));
    let standing = super::pace::blockers(level, x, t, c);
    let mut removed = Vec::new();
    let mut arrived = Vec::new();
    for w in history.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        if b.turn >= turn {
            break;
        }
        let before = (a.explorers + a.towns + a.cities) as f64;
        let after = (b.explorers + b.towns + b.cities) as f64;
        let arr = b.actual[0] + b.actual[1];
        removed.push((before + arr - after).max(0.0));
        arrived.push(arr);
    }
    let recent = |v: &[f64]| {
        let r: Vec<f64> = v.iter().rev().take(3).copied().collect();
        if r.is_empty() { 0.0 } else { r.iter().sum::<f64>() / r.len() as f64 }
    };
    let (removed_per_turn, arrived_per_turn) = (recent(&removed), recent(&arrived));
    let net = removed_per_turn - arrived_per_turn;
    let arrives = if standing == 0 {
        Some(turn)
    } else if net > 0.05 {
        Some(turn + (standing as f64 / net).ceil() as u32)
    } else {
        None
    };
    let clear = ClearPace { level, standing, removed_per_turn, arrived_per_turn, arrives };
    let fear_win = fear.reach.iter().find(|(m, _)| m == "Victory").and_then(|(_, t)| *t);
    let win = [crossing.map(|t| ("outpace".to_string(), t)), arrives.map(|t| ("board clear".to_string(), t)), fear_win.map(|t| ("Fear victory".to_string(), t))]
        .into_iter()
        .flatten()
        .min_by_key(|(_, t)| *t);

    Chain { turn, orders: base.orders, actions, weights, turns, sources, capacity_now, crossing, clear, win, policy: POLICY.into() }
}

/// An innate's level as a card: its text, speed, range and target.
fn innate_card(i: &PublicInnate, level: usize) -> PublicPowerCard {
    PublicPowerCard {
        id: format!("{}:{level}", i.id),
        title: i.title.clone(),
        cost: 0,
        speed: i.speed.clone(),
        kind: "Innate".into(),
        elements: Vec::new(),
        range: i.range,
        from_sacred_site: i.from_sacred_site,
        target: i.target.clone(),
        body: i.levels[level].body.lines().map(String::from).collect(),
    }
}

/// An action a play can stop: its land, label and value, and what it takes.
struct Target<'a> {
    land: &'a PublicLand,
    label: String,
    value: f64,
    /// Ravage damage to remove, or Invader damage to clear for a Build.
    needed: i64,
    fast_only: bool,
}

fn can_cut(card: &PublicPowerCard, sp: &PublicSpirit, s: &PublicSnapshot, t: &Target) -> bool {
    if t.fast_only && card.speed != "Fast" {
        return false;
    }
    if !needs::reaches(card, sp, s, t.land).unwrap_or(false) {
        return false;
    }
    needs::tag_options(card).iter().any(|o| needs::reduction(o, t.land) >= t.needed)
}

fn capacity_now(sp: &PublicSpirit, s: &PublicSnapshot, view: &invaders::InvaderView, next: Option<&invaders::InvaderView>, actions: &[ActionChain], weights: &[f64; 3]) -> CapacityNow {
    let plays = super::spirits::track_value(sp, "CardPlaysPerTurn", true).unwrap_or(0).max(sp.card_plays);
    // Before the Gain Energy step, this turn's income is still to come.
    let before_income = matches!(s.step.as_str(), "PlayerSetup" | "InvadersStartingAction" | "SpiritPhase" | "Grow");
    let energy = sp.energy + if before_income { super::spirits::track_value(sp, "EnergyPerTurn", true).unwrap_or(0) } else { 0 };
    let land = |id: &str| s.lands.iter().find(|l| l.id == id);
    let value_of = |id: &str, step: &str| actions.iter().find(|a| a.land == id && a.step == step).map(|a| a.value);
    let mut targets: Vec<Target> = Vec::new();
    for r in view.phase.ravages.iter().chain(&view.phase.build_ravages).filter(|r| !r.done) {
        let (Some(l), Some(v)) = (land(&r.land), value_of(&r.land, "Ravage")) else { continue };
        if v <= 0.0 {
            continue;
        }
        let needed = if r.ledger.adds_blight { r.ledger.damage_after_defend - (r.ledger.land_health - 1) } else { r.ledger.damage_after_defend };
        targets.push(Target { land: l, label: format!("Ravage {}", r.land), value: v, needed: needed.max(1), fast_only: true });
    }
    for b in view.phase.builds.iter().filter(|b| !b.done) {
        let (Some(l), Some(v)) = (land(&b.land), value_of(&b.land, "Build")) else { continue };
        let clear = l.explorers as i64 + 2 * l.towns as i64 + 3 * l.cities as i64;
        targets.push(Target { land: l, label: format!("Build {}", b.land), value: v, needed: clear.max(1), fast_only: true });
    }
    if let Some(n) = next {
        for r in n.phase.ravages.iter().filter(|r| r.ledger.adds_blight) {
            let Some(l) = land(&r.land) else { continue };
            let needed = r.ledger.damage_after_defend - (r.ledger.land_health - 1);
            targets.push(Target { land: l, label: format!("Ravage {} next turn", r.land), value: weights[2], needed: needed.max(1), fast_only: false });
        }
    }

    // Every legal set of plays, with the innate levels its elements reach.
    let hand: Vec<&PublicPowerCard> = sp.hand.iter().collect();
    let n = hand.len().min(10);
    let mut best = 0.0;
    let mut best_cuts: Vec<(String, String, f64)> = Vec::new();
    for mask in 0u32..(1 << n) {
        let set: Vec<&PublicPowerCard> = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| hand[i]).collect();
        if set.len() as i64 > plays || set.iter().map(|c| c.cost).sum::<i64>() > energy {
            continue;
        }
        let mut elements: BTreeMap<String, i64> = sp.elements.clone();
        for c in &set {
            for e in &c.elements {
                *elements.entry(e.clone()).or_default() += 1;
            }
        }
        let mut plays_now: Vec<PublicPowerCard> = set.iter().map(|c| (*c).clone()).collect();
        for i in &sp.innate_powers {
            let level = i.levels.iter().rposition(|lv| lv.elements.iter().all(|(e, n)| elements.get(e).copied().unwrap_or(0) >= *n));
            if let Some(lv) = level {
                plays_now.push(innate_card(i, lv));
            }
        }
        // Each play stops at most one action; each action is stopped once.
        let can: Vec<Vec<usize>> = plays_now.iter().map(|c| targets.iter().enumerate().filter(|(_, t)| can_cut(c, sp, s, t)).map(|(j, _)| j).collect()).collect();
        let mut taken = vec![false; targets.len()];
        let mut cuts = Vec::new();
        let total = assign(0, &plays_now, &can, &targets, &mut taken, &mut cuts);
        if total > best + 1e-9 {
            best = total;
            best_cuts = cuts;
        }
    }
    CapacityNow { spirit: sp.title.clone(), plays, energy, best, cuts: best_cuts, per_play: best / plays.max(1) as f64 }
}

/// The most value `plays[i..]` can stop, one action each.
fn assign(i: usize, plays: &[PublicPowerCard], can: &[Vec<usize>], targets: &[Target], taken: &mut [bool], cuts: &mut Vec<(String, String, f64)>) -> f64 {
    if i == plays.len() {
        return 0.0;
    }
    let mut best = assign(i + 1, plays, can, targets, taken, cuts);
    let mut best_cuts = cuts.clone();
    for &j in &can[i] {
        if taken[j] {
            continue;
        }
        taken[j] = true;
        let mut sub = cuts.clone();
        sub.push((plays[i].title.clone(), targets[j].label.clone(), targets[j].value));
        let v = targets[j].value + assign(i + 1, plays, can, targets, taken, &mut sub);
        taken[j] = false;
        if v > best + 1e-9 {
            best = v;
            best_cuts = sub;
        }
    }
    *cuts = best_cuts;
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{PublicBoard, PublicInvaderCard, PublicInvaderDeck, PublicPiece};

    fn card(stage: u8, t: &str) -> PublicInvaderCard {
        PublicInvaderCard { id: format!("{stage}{t}"), stage, terrains: vec![t.into()], ..Default::default() }
    }

    /// One board, a Town in Jungle A1 (coastal), the Jungle card in Build,
    /// Sands in Ravage with nothing there; Stage I Wetland/Sands/Mountain unseen.
    fn snap() -> PublicSnapshot {
        let terrains = ["Jungle", "Wetland", "Sands", "Mountain", "Jungle", "Wetland"];
        let mut lands: Vec<PublicLand> = terrains
            .iter()
            .enumerate()
            .map(|(i, t)| PublicLand { id: format!("A{}", i + 1), board: "A".into(), number: i as i64 + 1, terrain: t.to_string(), coastal: i < 2, ..Default::default() })
            .collect();
        lands[0].towns = 1;
        lands[0].pieces.push(PublicPiece { kind: "Town".into(), max_health: 2, ..Default::default() });
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
        s.blight.remaining = 5;
        s.invaders = PublicInvaderDeck {
            upcoming_stages: vec![1, 1, 1, 2, 2],
            ravage: vec![card(1, "Sands")],
            build: vec![card(1, "Jungle")],
            pool: ["Jungle", "Wetland", "Sands", "Mountain"].iter().map(|t| card(1, t)).chain(["Jungle", "Wetland"].iter().map(|t| card(2, t))).collect(),
            ..Default::default()
        };
        s
    }

    #[test]
    fn a_build_plants_a_ravage_and_the_explore_chain_is_longest() {
        let s = snap();
        let history = Vec::new();
        let o = outlook::outlook(&s);
        let c = chain(&s, &history, &super::super::curve::curve(&s, &history), &o, &super::super::pace::FearPace::default());
        let build = c.actions.iter().find(|a| a.step == "Build" && a.land == "A1").expect("the Jungle Build");
        // The Town in A1 Blights on T3 with or without the City the Build
        // adds; what the Build plants is the City itself (3 health).
        assert_eq!(build.blight, 0.0, "{build:?}");
        assert!((build.pieces - 3.0).abs() < 1e-9, "{build:?}");
        assert!((build.value - 1.5).abs() < 1e-9, "{build:?}");
        // The Wetland next door Explores on 1 of 2 unseen Stage I cards
        // (Jungle and Sands are face up); left alone it Builds on T3 and
        // Ravages for a Blight on T4, and leaves an Explorer and a Town.
        // (The Explore itself still happens; the Explorer is gone before it Builds.)
        let explore = c.actions.iter().find(|a| a.step == "Explore" && a.land == "A2").expect("the Wetland Explore");
        assert_eq!(explore.chance, (1, 2));
        assert!((explore.actions - 1.0).abs() < 1e-9, "{explore:?}");
        assert!((explore.blight - 1.0).abs() < 1e-9, "{explore:?}");
        assert!((explore.pieces - 3.0).abs() < 1e-9, "{explore:?}");
        assert!((explore.value - 3.5).abs() < 1e-9, "{explore:?}");
        assert!(c.weights[0] > c.weights[2], "{:?}", c.weights);
        assert_eq!(c.turns.first().map(|t| t.turn), Some(1));
        assert!(c.turns.iter().all(|t| t.pressure >= 0.0 && t.backlog >= 0.0));
    }

    #[test]
    fn sources_count_empty_neighbors() {
        let s = snap();
        let history = Vec::new();
        let o = outlook::outlook(&s);
        let c = chain(&s, &history, &super::super::curve::curve(&s, &history), &o, &super::super::pace::FearPace::default());
        assert_eq!((c.sources.now, c.sources.lands, c.sources.empty), (1, 6, 5));
        // A1's empty neighbor A2 is a Wetland: 1 of the 2 unseen Stage I cards.
        assert!((c.sources.r - 0.5).abs() < 1e-9, "{}", c.sources.r);
    }
}
