//! The race: the competing goals that end the game (the direction document,
//! §3). Several endings race to arrive first and only the first counts
//! (competing risks: Putter, Fiocco & Geskus 2007; the stat-models report,
//! "Simulation is for races…"). Over every order of the unseen Invader
//! cards (`outlook.rs`, no Spirit action), each path records the first
//! deterministic ending it reaches: an adversary's loss condition at its
//! check step, a Spirit with no Presence, the Invader deck running out.
//! Blight loss is the one ending with a hidden part, the Blight card's
//! other side, so on each path it is a cumulative probability
//! (`p_loss`) of the Blight taken off the card, cut off at the path's
//! first deterministic ending. The cumulative incidence per ending is the
//! orders-weighted mean; the incidences never sum past one.
//!
//! Player-driven endings (Fear victory, board clear) cannot come from the
//! Invaders' side: they are shown as scenarios and counts, never as a
//! probability.

use super::outlook::{Outlook, PathEnd};
use super::p_loss;
use super::pace::FearPace;
use crate::public::PublicSnapshot;
use serde::{Deserialize, Serialize};

/// The policy every number here assumes, printed on the panel.
pub const POLICY: &str = "no Spirit action; cascades placed on the nearest unblighted land; Events and Fear Cards not drawn; the Blight card's hidden side counted over the unseen cards";

/// Where in a turn an ending is checked, for ordering.
pub fn step_rank(step: &str) -> u8 {
    match step {
        "Fast" => 1,
        "Ravage" => 2,
        "Build" => 3,
        "Explore" => 4,
        _ => 0,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Race {
    /// The turns the race is drawn over (the outlook's, extended to the
    /// latest deterministic ending any path reaches).
    pub turns: Vec<u32>,
    pub orders: u64,
    /// Invader-side endings first, then the Spirits'.
    pub endings: Vec<Ending>,
    /// The ending whose arrival comes first (by median turn, then first
    /// arrival), among those with an incidence.
    pub binding: Option<usize>,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Ending {
    /// "blight", "deck", "clock:<adversary id>", "presence:<spirit id>", "fear", "board".
    pub key: String,
    pub title: String,
    /// "invaders" or "spirits".
    pub side: String,
    /// When it is checked: "the Ravage", "the Build", "the end of the Fast Phase", …
    pub checked: String,
    /// Cumulative incidence per turn of `Race.turns`: the share of futures
    /// (orders × the hidden side) in which this ending has arrived, first,
    /// by that turn. Empty for a scenario.
    pub cif: Vec<f64>,
    /// First turn with any incidence; the turn it reaches half.
    pub first_turn: Option<u32>,
    pub median_turn: Option<u32>,
    /// Scenario lines for player-driven endings, and notes.
    pub scenario: Vec<String>,
}

impl Race {
    pub fn get(&self, key: &str) -> Option<&Ending> {
        self.endings.iter().find(|e| e.key == key)
    }

    pub fn cif(&self, key: &str) -> Option<&[f64]> {
        self.get(key).map(|e| e.cif.as_slice())
    }

    /// The incidence of `key` at turn `t`, if the race covers it.
    pub fn at(&self, key: &str, t: u32) -> Option<f64> {
        let i = self.turns.iter().position(|x| *x == t)?;
        self.cif(key)?.get(i).copied()
    }
}

/// `true` when the Ravage of turn `t` comes after ending `e`.
fn ravage_after(e: &PathEnd, t: u32) -> bool {
    e.turn < t || (e.turn == t && step_rank(&e.step) < step_rank("Ravage"))
}

pub fn race(s: &PublicSnapshot, o: &Outlook, pace: &FearPace) -> Race {
    let mut turns = o.turns.clone();
    if turns.is_empty() {
        return Race { policy: POLICY.into(), ..Default::default() };
    }
    let last_end = o.paths.iter().filter_map(|p| p.end.as_ref().map(|e| e.turn)).max().unwrap_or(0);
    while *turns.last().unwrap() < last_end {
        turns.push(turns.last().unwrap() + 1);
    }
    let first = o.turns[0];
    let b = &s.blight;

    // The deterministic endings the paths reach, in order of appearance.
    let mut keys: Vec<(String, String, String)> = Vec::new();
    for p in &o.paths {
        if let Some(e) = &p.end {
            if !keys.iter().any(|k| k.0 == e.key) {
                keys.push((e.key.clone(), e.title.clone(), e.step.clone()));
            }
        }
    }
    let n = turns.len();
    let mut blight_cif = vec![0f64; n];
    let mut det_cif: Vec<Vec<f64>> = vec![vec![0f64; n]; keys.len()];
    for p in &o.paths {
        let w = p.orders as f64;
        let cum = p.cumulative();
        let cum_at = |t: u32| -> i64 {
            match o.turns.iter().position(|x| *x == t) {
                Some(i) => cum.get(i).or(cum.last()).copied().unwrap_or(0),
                None if t > *o.turns.last().unwrap() => cum.last().copied().unwrap_or(0),
                None => 0,
            }
        };
        let cum_before = |t: u32| if t > first { cum_at(t - 1) } else { 0 };
        // Blight taken off the card by the last Ravage before the path's ending.
        let cut = p.end.as_ref().map(|e| if step_rank(&e.step) <= step_rank("Fast") { cum_before(e.turn) } else { cum_at(e.turn) });
        for (j, t) in turns.iter().enumerate() {
            let k = match (&p.end, cut) {
                (Some(e), Some(c)) if ravage_after(e, *t) => c,
                _ => cum_at(*t),
            };
            blight_cif[j] += w * p_loss(k, b);
            if let (Some(e), Some(c)) = (&p.end, cut) {
                if *t >= e.turn {
                    let i = keys.iter().position(|x| x.0 == e.key).unwrap();
                    det_cif[i][j] += w * (1.0 - p_loss(c, b));
                }
            }
        }
    }
    let total = o.orders.max(1) as f64;
    let norm = |v: Vec<f64>| v.into_iter().map(|x| x / total).collect::<Vec<f64>>();

    let mut endings = Vec::new();
    let clock_checked = |step: &str| match step {
        "Fast" => "the end of the Fast Phase".to_string(),
        "Ravage" => "the Ravage".to_string(),
        "Build" => "the Build".to_string(),
        "Explore" => "the Explore".to_string(),
        other => other.to_string(),
    };
    for (i, (key, title, step)) in keys.iter().enumerate() {
        // A path can reach an ending only after Blight has already taken
        // every hidden side; such an ending has no incidence and is not a row.
        let cif = norm(det_cif[i].clone());
        if cif.iter().any(|x| *x > 1e-9) {
            endings.push(ending(key, title, "invaders", &clock_checked(step), cif, &turns));
        }
    }
    let blight_title = if b.flipped { "Blight card empties" } else { "Blight card flips, then empties" };
    endings.push(ending("blight", blight_title, "invaders", "the Ravage", norm(blight_cif), &turns));
    if !keys.iter().any(|k| k.0 == "deck") {
        if let Some(d) = pace.deck_out {
            let mut e = ending("deck", "Invader deck runs out", "invaders", "the Explore", Vec::new(), &turns);
            e.first_turn = Some(d);
            e.median_turn = Some(d);
            e.scenario.push(format!("T{d}'s Explore, in every order that gets there"));
            endings.push(e);
        }
    }
    // The Spirits' side: scenarios and counts, never a probability.
    let victory = pace.marks.last().map(|m| m.1).unwrap_or(pace.total);
    let left = (victory - pace.total).max(0);
    let mut fear = ending("fear", "Fear victory", "spirits", "the Fear step", Vec::new(), &turns);
    fear.scenario.push(format!("{} of {victory} Fear", pace.total));
    if let (Some(need), Some(d)) = (pace.need, pace.deck_out) {
        fear.scenario.push(format!("at {need:.1} a turn: T{d}"));
    }
    if let Some((bt, bv)) = pace.per_turn.iter().filter(|(t, _)| *t < s.turn).max_by_key(|(_, v)| *v) {
        if *bv > 0 {
            let t = s.turn + (left as f64 / *bv as f64).ceil() as u32;
            fear.scenario.push(format!("at {bv} a turn (your T{bt}): T{t}"));
        }
    }
    endings.push(fear);
    let mut board = ending("board", "Board clear", "spirits", "any destroy", Vec::new(), &turns);
    for r in &pace.board {
        board.scenario.push(format!("Terror {}: {} ({})", ["I", "II", "III"][(r.level as usize).clamp(1, 3) - 1], r.now, r.condition));
    }
    endings.push(board);

    // The binding ending: earliest median, then earliest first arrival.
    let binding = endings
        .iter()
        .enumerate()
        .filter(|(_, e)| e.side == "invaders" && (e.median_turn.is_some() || e.first_turn.is_some()))
        .min_by_key(|(i, e)| (e.median_turn.unwrap_or(u32::MAX), e.first_turn.unwrap_or(u32::MAX), *i))
        .map(|(i, _)| i);
    Race { turns, orders: o.orders, endings, binding, policy: POLICY.into() }
}

fn ending(key: &str, title: &str, side: &str, checked: &str, cif: Vec<f64>, turns: &[u32]) -> Ending {
    let first_turn = cif.iter().position(|x| *x > 1e-9).map(|i| turns[i]);
    let median_turn = cif.iter().position(|x| *x >= 0.5).map(|i| turns[i]);
    Ending { key: key.into(), title: title.into(), side: side.into(), checked: checked.into(), cif, first_turn, median_turn, scenario: Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::companion_stats::outlook::OutlookPath;

    fn blight_sure() -> PublicSnapshot {
        let mut s = PublicSnapshot { turn: 4, step: "SpiritPhase".into(), ..Default::default() };
        // A Blighted side with 2 left: the card empties at the 2nd Blight.
        s.blight.flipped = true;
        s.blight.remaining = 2;
        s
    }

    #[test]
    fn blight_only_is_the_card_arithmetic() {
        let s = blight_sure();
        let o = Outlook { turns: vec![4, 5, 6], orders: 2, paths: vec![OutlookPath { orders: 2, blight: vec![1, 1, 1], ..Default::default() }], ..Default::default() };
        let r = race(&s, &o, &FearPace::default());
        assert_eq!(r.cif("blight").unwrap(), &[0.0, 1.0, 1.0]);
        assert_eq!(r.get("blight").unwrap().median_turn, Some(5));
        assert_eq!(r.binding.map(|i| r.endings[i].key.clone()), Some("blight".into()));
    }

    #[test]
    fn a_clock_that_fires_first_censors_blight() {
        let s = blight_sure();
        // One order ends by a clock at T5's Fast (before T5's Ravage); the other does not.
        let ended = OutlookPath { orders: 1, blight: vec![1, 1, 1], end: Some(PathEnd { key: "clock:X".into(), title: "X".into(), turn: 5, step: "Fast".into() }), ..Default::default() };
        let open = OutlookPath { orders: 1, blight: vec![1, 1, 1], ..Default::default() };
        let o = Outlook { turns: vec![4, 5, 6], orders: 2, paths: vec![ended, open], ..Default::default() };
        let r = race(&s, &o, &FearPace::default());
        // The clock path: Blight stays at 1 (T4's Ravage only), so the clock takes it all at T5.
        assert_eq!(r.cif("clock:X").unwrap(), &[0.0, 0.5, 0.5]);
        assert_eq!(r.cif("blight").unwrap(), &[0.0, 0.5, 0.5]);
        let sum: f64 = r.endings.iter().filter(|e| !e.cif.is_empty()).map(|e| e.cif[2]).sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert_eq!(r.binding.map(|i| r.endings[i].key.clone()), Some("clock:X".into()));
    }
}
