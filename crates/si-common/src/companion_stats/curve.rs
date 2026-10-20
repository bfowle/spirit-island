//! Rei's curve (reports/Spirit Island companion stat models.md, "Economy
//! momentum is a deterministic projection against a step-function curve"):
//! the Invader side as the lands each turn's cards name in Explore, Build and
//! Ravage, a step that climbs with the stages (darker: Stage I, II, III
//! cards hit about 2, 3, 4 lands per board); the Spirit side as Energy and
//! card plays a turn from the presence tracks, so far and projected along
//! each track. Natural units only: no exchange rate between the two.

use super::RoundSummary;
use crate::public::{PublicInvaderCard, PublicSnapshot, PublicSpirit};
use crate::rules;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Curve {
    pub boards: u32,
    /// Lands a card of each stage names on this island, on average over
    /// the stage's cards still unseen (all of them once none are).
    pub stage_reach: [f64; 3],
    /// Turn 1 through the turn the deck runs out.
    pub turns: Vec<CurveTurn>,
    /// Per turn so far: Energy and card plays a turn, presence on the
    /// island, Invaders (Explorers, Towns, Cities).
    pub economy: Vec<Economy>,
    /// Projections from next turn on, one presence a turn from one track.
    pub growth: Vec<GrowthLine>,
    pub turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurveTurn {
    pub turn: u32,
    pub ravage: Step,
    pub build: Step,
    pub explore: Step,
}

impl CurveTurn {
    /// Lands named across the three steps.
    pub fn lands(&self) -> f64 {
        self.ravage.lands + self.build.lands + self.explore.lands
    }
}

/// One step's cards: known (seen, or face up) or only the stage on the back.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Step {
    /// 0: no card.
    pub stage: u8,
    pub terrains: Vec<String>,
    pub lands: f64,
    pub known: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Economy {
    pub turn: u32,
    pub energy: i64,
    pub plays: i64,
    pub presence: u32,
    pub invaders: [u32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GrowthLine {
    /// "Energy track" or "Card Plays track".
    pub label: String,
    /// (turn, Energy a turn, card plays a turn).
    pub points: Vec<(u32, i64, i64)>,
}

fn matched(c: &PublicInvaderCard, s: &PublicSnapshot) -> f64 {
    s.lands.iter().filter(|l| !l.is_ocean && rules::card_matches(c, l)).count() as f64
}

fn known(cards: &[PublicInvaderCard], s: &PublicSnapshot) -> Step {
    if cards.is_empty() {
        return Step::default();
    }
    let mut terrains: Vec<String> = cards.iter().flat_map(|c| c.terrains.clone()).collect();
    terrains.dedup();
    Step { stage: cards.iter().map(|c| c.stage).max().unwrap_or(0), terrains, lands: cards.iter().map(|c| matched(c, s)).sum(), known: true }
}

pub fn curve(s: &PublicSnapshot, history: &[RoundSummary]) -> Curve {
    let inv = &s.invaders;
    let boards = s.boards.len().max(1) as u32;
    let seen: Vec<&str> = inv.explore.iter().chain(&inv.build).chain(&inv.ravage).chain(&inv.discard).map(|c| c.id.as_str()).collect();
    let mut stage_reach = [0.0; 3];
    for (i, reach) in stage_reach.iter_mut().enumerate() {
        let of_stage: Vec<&PublicInvaderCard> = inv.pool.iter().filter(|c| c.stage as usize == i + 1).collect();
        let unseen: Vec<&&PublicInvaderCard> = of_stage.iter().filter(|c| !seen.contains(&c.id.as_str())).collect();
        let cards: Vec<&PublicInvaderCard> = if unseen.is_empty() { of_stage.clone() } else { unseen.into_iter().copied().collect() };
        *reach = if cards.is_empty() {
            // No pool: darker's rule of thumb.
            (i as f64 + 2.0) * boards as f64
        } else {
            cards.iter().map(|c| matched(c, s)).sum::<f64>() / cards.len() as f64
        };
    }
    let by_stage = |st: u8| Step { stage: st, lands: stage_reach[(st as usize).clamp(1, 3) - 1], known: false, ..Default::default() };

    // The card each turn Explores: seen ones from the history, then the
    // stages on the backs. Turn 0 is setup's Explore.
    let mut explores: BTreeMap<u32, Step> = BTreeMap::new();
    let rows: BTreeMap<u32, &RoundSummary> = history.iter().map(|h| (h.turn, h)).collect();
    if let Some(first) = history.first().filter(|h| h.turn == 1) {
        explores.insert(0, known(&first.built, s));
    }
    for h in history {
        if !h.explored.is_empty() {
            explores.insert(h.turn, known(&h.explored, s));
        }
    }
    // Turns the companion did not see (it joined mid-game): the cards drawn
    // so far are public (the discard, newest first, then the slots), and the
    // k-th card drawn Explored on turn k−1, Built on k, Ravaged on k+1.
    let mut drawn: Vec<&PublicInvaderCard> = inv.discard.iter().rev().collect();
    drawn.extend(inv.ravage.iter().chain(&inv.build).chain(&inv.explore));
    for (k, c) in drawn.iter().enumerate() {
        explores.entry(k as u32).or_insert_with(|| known(std::slice::from_ref(*c), s));
    }
    let advanced = matches!(s.step.as_str(), "AdvanceInvaderCards" | "SlowPower" | "TimePasses");
    let next_explore = s.turn + u32::from(advanced);
    for (k, st) in inv.upcoming_stages.iter().enumerate() {
        explores.entry(next_explore + k as u32).or_insert_with(|| by_stage(*st));
    }
    let deck_out = next_explore + inv.upcoming_stages.len() as u32;

    let turns = (1..=deck_out)
        .map(|t| {
            let row = rows.get(&t);
            let slot = |cards: Option<&Vec<PublicInvaderCard>>, back: u32| match cards.filter(|c| !c.is_empty()) {
                Some(c) => known(c, s),
                None => t.checked_sub(back).and_then(|e| explores.get(&e)).cloned().unwrap_or_default(),
            };
            CurveTurn {
                turn: t,
                ravage: slot(row.map(|r| &r.ravaged), 2),
                build: slot(row.map(|r| &r.built), 1),
                explore: if t == deck_out { Step::default() } else { explores.get(&t).cloned().unwrap_or_default() },
            }
        })
        .collect();

    let economy = history
        .iter()
        .map(|h| Economy { turn: h.turn, energy: h.energy_income, plays: h.plays, presence: h.presence_on_island, invaders: [h.explorers, h.towns, h.cities] })
        .collect();
    let horizon: Vec<u32> = (s.turn + 1..=deck_out.max(s.turn + 1)).collect();
    let growth = [("Energy track", "EnergyPerTurn"), ("Card Plays track", "CardPlaysPerTurn")]
        .iter()
        .map(|(label, kind)| {
            let points = horizon
                .iter()
                .enumerate()
                .map(|(k, t)| {
                    let (e, p) = s.spirits.iter().fold((0, 0), |(e, p), sp| {
                        let (de, dp) = projected(sp, kind, k as usize + 1);
                        (e + de, p + dp)
                    });
                    (*t, e, p)
                })
                .collect();
            GrowthLine { label: label.to_string(), points }
        })
        .collect();
    Curve { boards, stage_reach, turns, economy, growth, turn: s.turn }
}

/// A spirit's Energy and card plays a turn after `n` more presence come off
/// the track that holds `kind` nodes (the other track unchanged).
pub fn projected(sp: &PublicSpirit, kind: &str, n: usize) -> (i64, i64) {
    // Node ids are "<spirit><Track><n>": "ASpreadOfRampantGreenEnergy7".
    let mut tracks: BTreeMap<String, Vec<(u32, &crate::public::TrackNode)>> = BTreeMap::new();
    for node in &sp.track {
        let rest = node.id.strip_prefix(sp.id.as_str()).unwrap_or(&node.id);
        let name: String = rest.chars().filter(|c| !c.is_ascii_digit()).collect();
        let num: u32 = rest.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
        tracks.entry(name).or_default().push((num, node));
    }
    let track = tracks.values().max_by_key(|nodes| nodes.iter().filter(|(_, x)| x.kind == kind).count());
    let mut taken: Vec<&str> = Vec::new();
    if let Some(nodes) = track {
        let mut nodes = nodes.clone();
        nodes.sort_by_key(|(num, _)| *num);
        taken = nodes.iter().filter(|(_, x)| !x.revealed).take(n).map(|(_, x)| x.id.as_str()).collect();
    }
    let best = |k: &str| sp.track.iter().filter(|x| x.kind == k && (x.revealed || taken.contains(&x.id.as_str()))).filter_map(|x| x.value).max().unwrap_or(0);
    (best("EnergyPerTurn"), best("CardPlaysPerTurn"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::TrackNode;

    fn node(id: &str, revealed: bool, kind: &str, value: Option<i64>) -> TrackNode {
        TrackNode { id: id.into(), revealed, kind: kind.into(), value, elements: vec![] }
    }

    #[test]
    fn projects_along_each_track() {
        // Rampant Green, Russia game Turn 6: Energy 2 (next 3), plays 4.
        let sp = PublicSpirit {
            id: "G".into(),
            track: vec![
                node("GEnergy1", true, "EnergyPerTurn", Some(0)),
                node("GEnergy5", true, "EnergyPerTurn", Some(2)),
                node("GEnergy6", false, "Element", None),
                node("GEnergy7", false, "EnergyPerTurn", Some(3)),
                node("GCardPlays1", true, "CardPlaysPerTurn", Some(1)),
                node("GCardPlays2", false, "CardPlaysPerTurn", Some(2)),
            ],
            ..Default::default()
        };
        assert_eq!(projected(&sp, "EnergyPerTurn", 1), (2, 1));
        assert_eq!(projected(&sp, "EnergyPerTurn", 2), (3, 1));
        assert_eq!(projected(&sp, "CardPlaysPerTurn", 1), (2, 2));
    }

    #[test]
    fn step_curve_from_history_then_stages() {
        let card = |stage: u8, t: &str| PublicInvaderCard { id: format!("{stage}{t}"), stage, terrains: vec![t.into()], ..Default::default() };
        let lands = ["Jungle", "Wetland", "Jungle", "Sands"]
            .iter()
            .enumerate()
            .map(|(i, t)| crate::public::PublicLand { id: format!("A{i}"), terrain: t.to_string(), ..Default::default() })
            .collect();
        let mut s = PublicSnapshot { turn: 2, step: "Ravage".into(), lands, ..Default::default() };
        s.invaders.ravage = vec![card(1, "Jungle")];
        s.invaders.build = vec![card(1, "Wetland")];
        s.invaders.upcoming_stages = vec![1, 2];
        s.invaders.pool = vec![card(1, "Jungle"), card(1, "Wetland"), card(1, "Sands"), card(2, "Jungle")];
        let row = |turn, ravaged: Vec<PublicInvaderCard>, built: Vec<PublicInvaderCard>, explored: Vec<PublicInvaderCard>| RoundSummary { turn, ravaged, built, explored, ..Default::default() };
        let history = vec![row(1, vec![], vec![card(1, "Jungle")], vec![card(1, "Wetland")]), row(2, vec![card(1, "Jungle")], vec![card(1, "Wetland")], vec![])];
        let c = curve(&s, &history);
        // Unseen Stage I: only Sands (1 land).
        assert_eq!(c.stage_reach[0], 1.0);
        assert_eq!(c.turns.len(), 4);
        // Turn 2: Ravage Jungle (2), Build Wetland (1), Explore a Stage I back (1).
        let t2 = &c.turns[1];
        assert_eq!((t2.ravage.lands, t2.build.lands, t2.explore.lands, t2.explore.known), (2.0, 1.0, 1.0, false));
        // Turn 4: the deck is out; Ravage the Turn 2 card, Build the Stage II back.
        let t4 = &c.turns[3];
        assert_eq!((t4.ravage.stage, t4.build.stage, t4.explore.stage), (1, 2, 0));
    }
}

#[cfg(test)]
mod joined_late {
    use super::*;

    /// No history (the companion joined mid-game): the past turns' cards
    /// come from the discard (newest first) and the slots.
    #[test]
    fn past_turns_from_the_discard_and_slots() {
        let card = |stage: u8, t: &str| PublicInvaderCard { id: format!("{stage}{t}"), stage, terrains: vec![t.into()], ..Default::default() };
        let lands = ["Jungle", "Wetland", "Sands", "Mountain"]
            .iter()
            .enumerate()
            .map(|(i, t)| crate::public::PublicLand { id: format!("A{i}"), terrain: t.to_string(), ..Default::default() })
            .collect();
        let mut s = PublicSnapshot { turn: 4, step: "SpiritPhase".into(), lands, ..Default::default() };
        // Drawn in order: Jungle (setup), Wetland (T1), Sands (T2), Mountain (T3).
        s.invaders.discard = vec![card(1, "Wetland"), card(1, "Jungle")];
        s.invaders.ravage = vec![card(1, "Sands")];
        s.invaders.build = vec![card(1, "Mountain")];
        s.invaders.upcoming_stages = vec![2, 2];
        s.invaders.pool = vec![card(1, "Jungle"), card(1, "Wetland"), card(1, "Sands"), card(1, "Mountain"), card(2, "Jungle"), card(2, "Sands")];
        let c = curve(&s, &[]);
        let t = |n: usize| &c.turns[n - 1];
        assert_eq!((t(1).ravage.lands, t(1).build.terrains.clone(), t(1).explore.terrains.clone()), (0.0, vec!["Jungle".to_string()], vec!["Wetland".to_string()]));
        assert_eq!((t(2).ravage.terrains.clone(), t(2).build.terrains.clone(), t(2).explore.terrains.clone()), (vec!["Jungle".to_string()], vec!["Wetland".to_string()], vec!["Sands".to_string()]));
        assert_eq!((t(3).ravage.terrains.clone(), t(3).build.terrains.clone(), t(3).explore.terrains.clone()), (vec!["Wetland".to_string()], vec!["Sands".to_string()], vec!["Mountain".to_string()]));
        // T4: the slots as they stand; the Explore is a Stage II back.
        assert_eq!((t(4).ravage.terrains.clone(), t(4).build.terrains.clone(), t(4).explore.stage, t(4).explore.known), (vec!["Sands".to_string()], vec!["Mountain".to_string()], 2, false));
    }
}
