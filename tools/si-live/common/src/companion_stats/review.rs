//! Post-game review, from the game's timeline of public snapshots.
//!
//! Every change between two snapshots is attributed to the step it happened
//! in (the earlier snapshot's step: a save is written on entering a step, so
//! blight from a Ravage first shows in the Build save). From that:
//! - **Blight attribution**: each blight by turn, land, and cause step.
//! - **Invader chains**: for blight from a Ravage, the same card's Explore
//!   and Build in that land, and the Invaders standing there at each point
//!   where a Fast or Slow action could have acted.
//! - **Flow**: Invaders added (Explore, Build, Event, …) vs removed (Fast,
//!   Slow, Fear, Ravage counterattack) per turn.
//! - **Pressure**: total Ravage damage the island's Invaders could deal at
//!   the end of each turn, and where it jumped.
//!
//! Output is facts and reflection questions, never a grade (spec 7.6).

use crate::public::{PublicDecision, PublicLand, PublicSnapshot};
use crate::rules;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GameReview {
    pub game_id: String,
    pub ending: Option<String>,
    pub turns: u32,
    pub blight: Vec<BlightEvent>,
    pub chains: Vec<InvaderChain>,
    pub flow: Vec<TurnFlow>,
    pub pressure: Vec<(u32, i64)>,
    pub largest_jump: Option<(u32, i64)>,
    pub turn_notes: Vec<TurnNote>,
    pub reflections: Vec<Reflection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BlightEvent {
    pub turn: u32,
    pub step: String,
    pub land: String,
    pub added: u32,
    /// "Ravage" when the land matched the Ravage card and had Invaders;
    /// "cascade" for other blight in the Ravage step; otherwise the step.
    pub cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvaderChain {
    pub land: String,
    pub card: String,
    pub ravaged_turn: u32,
    pub points: Vec<ChainPoint>,
    /// The player's decisions that touched this land while the chain built
    /// (two turns before the Ravage through the Ravage turn).
    pub actions_here: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChainPoint {
    /// e.g. "Turn 3, after Explore (Slow phase)".
    pub when: String,
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    /// Ravage damage these Invaders would deal (before Defend).
    pub damage: i64,
    /// Health of all Invaders here: what clearing the land would take.
    pub health_to_clear: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurnFlow {
    pub turn: u32,
    /// Invader pieces added, by cause step (Explore, Build, Event, Escalation, …).
    pub added: BTreeMap<String, i64>,
    /// Invader pieces removed, by cause step (FastPower, SlowPower, Fear, Ravage, …).
    pub removed: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurnNote {
    pub turn: u32,
    pub played: Vec<String>,
    pub energy_left: i64,
    pub plays_left: i64,
    /// The turn's decisions in order ("Turn 3 FastPower: Used Lure of the Unknown on MountainC3").
    pub decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Reflection {
    pub question: String,
    pub facts: Vec<String>,
}

fn land<'a>(s: &'a PublicSnapshot, id: &str) -> Option<&'a PublicLand> {
    s.lands.iter().find(|l| l.id == id)
}

fn invader_damage(l: &PublicLand) -> i64 {
    l.explorers as i64 + 2 * l.towns as i64 + 3 * l.cities as i64
}

fn invader_health(l: &PublicLand) -> i64 {
    l.pieces.iter().filter(|p| rules::is_invader(&p.kind)).map(|p| (p.max_health - p.damage).max(1)).sum()
}

fn cause(step: &str) -> String {
    match step {
        "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower" => "Spirit phase".into(),
        "InvadersStartingAction" | "PlayerSetup" => "Setup".into(),
        other => other.to_string(),
    }
}

/// Last snapshot of `turn` whose step satisfies `pred`.
fn last_at(timeline: &[PublicSnapshot], turn: u32, pred: impl Fn(&str) -> bool) -> Option<&PublicSnapshot> {
    timeline.iter().filter(|s| s.turn == turn && pred(&s.step)).last()
}

fn point(when: String, s: Option<&PublicSnapshot>, id: &str) -> Option<ChainPoint> {
    let l = land(s?, id)?;
    Some(ChainPoint {
        when,
        explorers: l.explorers,
        towns: l.towns,
        cities: l.cities,
        damage: invader_damage(l),
        health_to_clear: invader_health(l),
    })
}

const BEFORE_INVADERS: &[&str] = &["SpiritPhase", "Grow", "GainEnergy", "PlayPayPower", "FastPower", "BlightedIsland", "Event", "Fear", "HighImmigration"];
const AFTER_INVADERS: &[&str] = &["AdvanceInvaderCards", "SlowPower", "TimePasses"];

pub fn review(timeline: &[PublicSnapshot], decisions: &[PublicDecision]) -> GameReview {
    let Some(last) = timeline.last() else { return GameReview::default() };
    let mut blight = Vec::new();
    let mut flow: BTreeMap<u32, TurnFlow> = BTreeMap::new();

    for w in timeline.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        let step = cause(&a.step);
        for lb in &b.lands {
            let Some(la) = land(a, &lb.id) else { continue };
            if lb.blight > la.blight {
                let ravaged = step == "Ravage" && rules::has_invaders(la) && a.invaders.ravage.iter().any(|c| rules::card_matches(c, la));
                let cause = if ravaged { "Ravage" } else if step == "Ravage" { "cascade" } else { step.as_str() };
                blight.push(BlightEvent { turn: a.turn, step: step.clone(), land: lb.id.clone(), added: lb.blight - la.blight, cause: cause.into() });
            }
        }
        // Island totals per piece kind, so pushes and gathers cancel out; a
        // City downgraded to a Town shows as one of each.
        let f = flow.entry(a.turn).or_insert_with(|| TurnFlow { turn: a.turn, ..Default::default() });
        let totals = |s: &PublicSnapshot| -> [i64; 3] {
            s.lands.iter().fold([0; 3], |t, l| [t[0] + l.explorers as i64, t[1] + l.towns as i64, t[2] + l.cities as i64])
        };
        let (ta, tb) = (totals(a), totals(b));
        for k in 0..3 {
            let d = tb[k] - ta[k];
            if d > 0 {
                *f.added.entry(step.clone()).or_default() += d;
            } else if d < 0 {
                *f.removed.entry(step.clone()).or_default() += -d;
            }
        }
    }

    // Invader chains for blight that came from a Ravage.
    let mut chains = Vec::new();
    for ev in blight.iter().filter(|e| e.cause == "Ravage") {
        let t = ev.turn;
        let before_ravage = last_at(timeline, t, |s| BEFORE_INVADERS.contains(&s) || s == "Ravage");
        let card = before_ravage
            .and_then(|s| s.invaders.ravage.iter().find(|c| land(s, &ev.land).is_some_and(|l| rules::card_matches(c, l))))
            .map(|c| c.id.clone())
            .unwrap_or_default();
        let mut points = Vec::new();
        if t >= 2 {
            points.extend(point(format!("Turn {}, after Explore (Slow phase)", t - 2), last_at(timeline, t - 2, |s| AFTER_INVADERS.contains(&s)), &ev.land));
        }
        if t >= 1 {
            points.extend(point(format!("Turn {}, before Build (Fast phase)", t - 1), last_at(timeline, t - 1, |s| BEFORE_INVADERS.contains(&s)), &ev.land));
            points.extend(point(format!("Turn {}, after Build (Slow phase)", t - 1), last_at(timeline, t - 1, |s| AFTER_INVADERS.contains(&s)), &ev.land));
        }
        points.extend(point(format!("Turn {t}, before Ravage (Fast phase)"), before_ravage, &ev.land));
        let actions_here = decisions
            .iter()
            .filter(|d| d.land.as_deref() == Some(ev.land.as_str()) && d.turn + 2 >= t && d.turn <= t)
            .map(|d| format!("Turn {} {}: {}", d.turn, d.step, d.text))
            .collect();
        chains.push(InvaderChain { land: ev.land.clone(), card, ravaged_turn: t, points, actions_here });
    }

    // Pressure at the end of each turn.
    let mut pressure: Vec<(u32, i64)> = Vec::new();
    for s in timeline {
        let p: i64 = s.lands.iter().map(invader_damage).sum();
        match pressure.last_mut() {
            Some(last) if last.0 == s.turn => last.1 = p,
            _ => pressure.push((s.turn, p)),
        }
    }
    let largest_jump = pressure.windows(2).map(|w| (w[1].0, w[1].1 - w[0].1)).max_by_key(|(_, d)| *d).filter(|(_, d)| *d > 0);

    // What was played each turn, and what was left.
    let turns = last.turn;
    let turn_notes: Vec<TurnNote> = (1..=turns)
        .filter_map(|t| {
            let s = last_at(timeline, t, |s| s == "SlowPower" || s == "TimePasses").or_else(|| last_at(timeline, t, |_| true))?;
            Some(TurnNote {
                turn: t,
                played: s
                    .spirits
                    .iter()
                    .flat_map(|sp| sp.play_area.iter().map(|c| format!("{} ({})", c.title, if c.speed.is_empty() { c.kind.as_str() } else { c.speed.as_str() })))
                    .collect(),
                energy_left: s.spirits.iter().map(|sp| sp.energy).sum(),
                plays_left: s.spirits.iter().map(|sp| sp.card_plays).sum(),
                decisions: decisions.iter().filter(|d| d.turn == t).map(|d| format!("{}: {}", d.step, d.text)).collect(),
            })
        })
        .collect();

    let flow: Vec<TurnFlow> = flow.into_values().collect();
    let reflections = reflections(&blight, &chains, &flow, &turn_notes, largest_jump);

    GameReview {
        game_id: last.game_id.clone(),
        ending: last.ending.clone(),
        turns,
        blight,
        chains,
        flow,
        pressure,
        largest_jump,
        turn_notes,
        reflections,
    }
}

fn reflections(
    blight: &[BlightEvent],
    chains: &[InvaderChain],
    flow: &[TurnFlow],
    notes: &[TurnNote],
    jump: Option<(u32, i64)>,
) -> Vec<Reflection> {
    let mut out = Vec::new();
    let mut by_cause: BTreeMap<&str, u32> = BTreeMap::new();
    for b in blight {
        *by_cause.entry(b.cause.as_str()).or_default() += b.added;
    }
    if !blight.is_empty() {
        out.push(Reflection {
            question: "Where did the blight come from?".into(),
            facts: by_cause.iter().map(|(k, v)| format!("{v} from {k}")).collect(),
        });
    }
    let early: Vec<String> = chains
        .iter()
        .filter_map(|c| {
            let first = c.points.first()?;
            let lastp = c.points.last()?;
            (first.towns + first.cities == 0 && first.explorers > 0 && lastp.damage > first.damage).then(|| {
                format!(
                    "{} (Turn {} Ravage, {}): {} Explorer(s) at \"{}\" vs {} damage at the Ravage",
                    c.land, c.ravaged_turn, c.card, first.explorers, first.when, lastp.damage
                )
            })
        })
        .collect();
    if !early.is_empty() {
        out.push(Reflection {
            question: "Which Ravage blights started as a lone Explorer, when one Slow action could have ended the chain?".into(),
            facts: early,
        });
    }
    let total = |f: &dyn Fn(&TurnFlow) -> &BTreeMap<String, i64>, key: &str| -> i64 { flow.iter().map(|t| f(t).get(key).copied().unwrap_or(0)).sum() };
    let added_explore = total(&|t| &t.added, "Explore");
    let added_build = total(&|t| &t.added, "Build");
    let removed: Vec<String> = ["FastPower", "SlowPower", "Fear", "Ravage", "Event"]
        .iter()
        .map(|k| (k, total(&|t| &t.removed, k)))
        .filter(|(_, n)| *n > 0)
        .map(|(k, n)| format!("removed in {k}: {n}"))
        .collect();
    out.push(Reflection {
        question: "Did removal keep pace with arrival, and in which phase did it happen?".into(),
        facts: std::iter::once(format!("added by Explore: {added_explore}, by Build: {added_build}")).chain(removed).collect(),
    });
    if let Some((t, d)) = jump {
        out.push(Reflection {
            question: format!("What happened on Turn {t}, the largest jump in Invader pressure (+{d} damage)?"),
            facts: flow
                .iter()
                .filter(|f| f.turn == t)
                .flat_map(|f| f.added.iter().map(|(k, v)| format!("+{v} by {k}")).chain(f.removed.iter().map(|(k, v)| format!("−{v} in {k}"))))
                .collect(),
        });
    }
    let idle: Vec<String> = notes
        .iter()
        .filter(|n| n.plays_left > 0 || n.energy_left >= 3)
        .map(|n| format!("Turn {}: {} card play(s) and {} Energy left", n.turn, n.plays_left, n.energy_left))
        .collect();
    if !idle.is_empty() {
        out.push(Reflection { question: "Were unused plays or banked Energy deliberate?".into(), facts: idle });
    }
    out
}
