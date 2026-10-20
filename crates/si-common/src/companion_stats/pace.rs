//! Lumia's Fear arithmetic over time: the Fear the deck still holds, the
//! turns the Invader deck leaves to earn it, and the pace that takes,
//! against the pace so far. Totals count from the start of the game
//! (earned cards × Fear per card + Fear on the pool), so the marks ahead
//! (each Terror Level, victory) are fixed lines the running total climbs to.
//!
//! The other way to win is the board: at each Terror Level the spirits win
//! once no Invader of the kinds it names is left (Terror I: every Invader;
//! II: Towns and Cities; III: Cities). Fear lowers that bar; removal clears
//! it. `board` tracks what blocks each level still ahead.

use super::RoundSummary;
use crate::public::PublicSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FearPace {
    /// Running Fear total at the end of each turn so far (the current turn
    /// last, still running).
    pub totals: Vec<(u32, i64)>,
    /// Fear generated each turn so far.
    pub per_turn: Vec<(u32, i64)>,
    pub total: i64,
    /// Totals still ahead: ("Terror Level III", 36), ("Victory", 52).
    pub marks: Vec<(String, i64)>,
    /// Turns the Terror Level went up: (turn, new level).
    pub level_ups: Vec<(u32, u8)>,
    /// The turn whose Explore finds the Invader deck empty.
    pub deck_out: Option<u32>,
    /// Turns left to generate Fear in, this one and the deck-out turn
    /// included (its Fear, Ravage and Build come before its Explore).
    pub turns_left: u32,
    /// Fear a turn that clears the deck by the deck-out turn.
    pub need: Option<f64>,
    /// Mean Fear a turn over the last three finished turns, and the game.
    pub recent: Option<f64>,
    pub game: Option<f64>,
    /// At the recent pace, the turn each mark is reached (None: not before
    /// the deck runs out).
    pub reach: Vec<(String, Option<u32>)>,
    /// The board route, the current Terror Level first.
    #[serde(default)]
    pub board: Vec<BoardRoute>,
    pub pocket_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BoardRoute {
    pub level: u8,
    /// "no Towns or Cities".
    pub condition: String,
    /// Invaders that block it now.
    pub now: u32,
    /// At the end of each turn so far.
    pub history: Vec<(u32, u32)>,
    /// The level is the current one.
    pub current: bool,
    /// Otherwise, the turn it arrives at the recent Fear pace.
    pub arrives: Option<u32>,
}

/// Invaders that block a Terror Level's victory, from piece counts.
pub fn blockers(level: u8, explorers: u32, towns: u32, cities: u32) -> u32 {
    match level {
        1 => explorers + towns + cities,
        2 => towns + cities,
        _ => cities,
    }
}

pub fn fear_pace(s: &PublicSnapshot, history: &[RoundSummary]) -> FearPace {
    let f = &s.fear;
    let req = f.required_per_card.max(1);
    let out = (f.earned_count as i64) + f.discard.len() as i64;
    let total = out * req + f.generated;
    let deck = f.deck_by_level.map(|n| n as i64);
    let lvl = f.terror_level.clamp(1, 3);
    let mut marks = Vec::new();
    if lvl == 1 {
        marks.push(("Terror Level II".to_string(), (out + deck[0]) * req));
    }
    if lvl <= 2 {
        marks.push(("Terror Level III".to_string(), (out + deck[0] + deck[1]) * req));
    }
    marks.push(("Victory".to_string(), (out + deck.iter().sum::<i64>()) * req));

    let mut totals: Vec<(u32, i64)> = history.iter().map(|h| (h.turn, h.fear_total)).collect();
    match totals.last_mut() {
        Some(last) if last.0 == s.turn => last.1 = total,
        _ => totals.push((s.turn, total)),
    }
    // A history that starts after Turn 1 (joined mid-game) has no first
    // difference.
    let mut per_turn = Vec::new();
    let mut prev = None;
    for (t, v) in &totals {
        match prev {
            Some(p) => per_turn.push((*t, v - p)),
            None if *t == 1 => per_turn.push((*t, *v)),
            None => {}
        }
        prev = Some(*v);
    }
    let mut level_ups = Vec::new();
    for w in history.windows(2) {
        if w[1].terror_level > w[0].terror_level {
            level_ups.push((w[1].turn, w[1].terror_level));
        }
    }

    // The deck clock: one card per Explore; this turn's is still to come
    // unless the cards have advanced.
    let left = s.invaders.upcoming_stages.len() as u32;
    let explored = matches!(s.step.as_str(), "AdvanceInvaderCards" | "SlowPower" | "TimePasses");
    let deck_out = s.turn + left + u32::from(explored);
    let turns_left = deck_out - s.turn + 1;
    let victory = marks.last().map(|m| m.1).unwrap_or(total);
    let need = (turns_left > 0).then(|| (victory - total).max(0) as f64 / turns_left as f64);

    // Finished turns only: the current one is still running.
    let done: Vec<i64> = per_turn.iter().filter(|(t, _)| *t < s.turn).map(|(_, v)| *v).collect();
    let mean = |v: &[i64]| (!v.is_empty()).then(|| v.iter().sum::<i64>() as f64 / v.len() as f64);
    let recent = mean(&done[done.len().saturating_sub(3)..]);
    let game = mean(&done);
    let reach: Vec<(String, Option<u32>)> = marks
        .iter()
        .map(|(name, at)| {
            let turn = recent.filter(|p| *p > 0.0).map(|p| s.turn + ((at - total).max(0) as f64 / p).ceil() as u32);
            (name.clone(), turn.filter(|t| *t <= deck_out))
        })
        .collect();
    let board = (lvl..=3)
        .map(|level| {
            let (condition, now) = crate::rules::terror_victory_blockers(level, s);
            let mark = format!("Terror Level {}", ["I", "II", "III"][level as usize - 1]);
            BoardRoute {
                level,
                condition,
                now,
                history: history.iter().map(|h| (h.turn, blockers(level, h.explorers, h.towns, h.cities))).collect(),
                current: level == lvl,
                arrives: reach.iter().find(|(n, _)| *n == mark).and_then(|(_, t)| *t),
            }
        })
        .collect();
    FearPace {
        board,
        totals,
        per_turn,
        total,
        marks,
        level_ups,
        deck_out: Some(deck_out),
        turns_left,
        need,
        recent,
        game,
        reach,
        pocket_rule: format!("Fear a turn to win in time = Fear left in the deck ÷ turns of Invader cards left ({} ÷ {turns_left}).", (victory - total).max(0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn need_and_pace() {
        // Russia game, Turn 6: 5 cards out at 4 Fear, 3 on the pool, 0/4/4
        // left; 7 Invader cards left before this turn's Explore.
        let mut s = PublicSnapshot { turn: 6, step: "Ravage".into(), ..Default::default() };
        s.fear.required_per_card = 4;
        s.fear.generated = 3;
        s.fear.terror_level = 2;
        s.fear.deck_by_level = [0, 4, 4];
        s.fear.discard = vec![Default::default(); 5];
        s.invaders.upcoming_stages = vec![3, 2, 3, 2, 3, 2, 3];
        let row = |turn, fear_total| RoundSummary { turn, fear_total, terror_level: 2, ..Default::default() };
        let history = vec![row(1, 2), row(2, 5), row(3, 8), row(4, 14), row(5, 20), row(6, 23)];
        let p = fear_pace(&s, &history);
        assert_eq!(p.total, 23);
        assert_eq!(p.marks, vec![("Terror Level III".to_string(), 36), ("Victory".to_string(), 52)]);
        assert_eq!((p.deck_out, p.turns_left), (Some(13), 8));
        assert_eq!(p.need, Some(29.0 / 8.0));
        // Turns 3–5 gave 3, 6, 6; the game 2, 3, 3, 6, 6.
        assert_eq!((p.recent, p.game), (Some(5.0), Some(4.0)));
        // 13 Fear to Terror Level III at 5 a turn: 3 turns.
        assert_eq!(p.reach[0], ("Terror Level III".to_string(), Some(9)));
        // The board route: Terror II now, Terror III on Turn 9.
        assert_eq!(p.board.iter().map(|b| (b.level, b.current, b.arrives)).collect::<Vec<_>>(), vec![(2, true, None), (3, false, Some(9))]);
    }
}
