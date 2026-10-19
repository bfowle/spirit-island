//! Backtest: the forecast made on entering Ravage (after the Fast phase)
//! against what the engine then did, turn by turn. A mismatch points at a
//! rule the companion does not model yet.

use super::invaders;
use crate::public::{PublicLand, PublicSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurnCheck {
    pub turn: u32,
    pub ravage_blight_predicted: Vec<String>,
    pub ravage_blight_actual: Vec<String>,
    pub builds_predicted: Vec<String>,
    pub builds_actual: Vec<String>,
    pub explore_card: Option<String>,
    pub explores_predicted: Vec<String>,
    pub explores_actual: Vec<String>,
    pub mismatches: Vec<String>,
}

fn first<'a>(timeline: &'a [PublicSnapshot], turn: u32, step: &str) -> Option<&'a PublicSnapshot> {
    timeline.iter().find(|s| s.turn == turn && s.step == step)
}

fn land<'a>(s: &'a PublicSnapshot, id: &str) -> Option<&'a PublicLand> {
    s.lands.iter().find(|l| l.id == id)
}

fn changed(a: &PublicSnapshot, b: &PublicSnapshot, f: fn(&PublicLand) -> u32) -> Vec<String> {
    b.lands.iter().filter(|lb| land(a, &lb.id).is_some_and(|la| f(lb) > f(la))).map(|l| l.id.clone()).collect()
}

fn diff(label: &str, predicted: &[String], actual: &[String], out: &mut Vec<String>) {
    let p: BTreeSet<&String> = predicted.iter().collect();
    let a: BTreeSet<&String> = actual.iter().collect();
    let missing: Vec<_> = a.difference(&p).collect();
    let extra: Vec<_> = p.difference(&a).collect();
    if !missing.is_empty() {
        out.push(format!("{label}: happened but not predicted {missing:?}"));
    }
    if !extra.is_empty() {
        out.push(format!("{label}: predicted but did not happen {extra:?}"));
    }
}

pub fn backtest(timeline: &[PublicSnapshot]) -> Vec<TurnCheck> {
    let turns: BTreeSet<u32> = timeline.iter().map(|s| s.turn).collect();
    let mut out = Vec::new();
    for t in turns {
        let (Some(pre), Some(after_r), Some(after_b), Some(after_e)) = (
            first(timeline, t, "Ravage"),
            first(timeline, t, "Build"),
            first(timeline, t, "Explore"),
            // The AdvanceInvaderCards save is written before the cards move.
            first(timeline, t, "SlowPower").or_else(|| first(timeline, t, "TimePasses")),
        ) else {
            continue;
        };
        let f = invaders::forecast(pre);
        let mut check = TurnCheck { turn: t, ..Default::default() };

        check.ravage_blight_predicted = f.phase.ravages.iter().filter(|r| r.ledger.adds_blight).map(|r| r.land.clone()).collect();
        let ravaged: BTreeSet<&str> = f.phase.ravages.iter().map(|r| r.land.as_str()).collect();
        check.ravage_blight_actual = changed(pre, after_r, |l| l.blight).into_iter().filter(|id| ravaged.contains(id.as_str())).collect();
        diff("Ravage blight", &check.ravage_blight_predicted, &check.ravage_blight_actual, &mut check.mismatches);

        check.builds_predicted = f.phase.builds.iter().map(|b| format!("{} +{}", b.land, b.adds)).collect();
        for lb in &after_b.lands {
            let Some(la) = land(after_r, &lb.id) else { continue };
            if lb.cities > la.cities {
                check.builds_actual.push(format!("{} +City", lb.id));
            } else if lb.towns > la.towns {
                check.builds_actual.push(format!("{} +Town", lb.id));
            }
        }
        diff("Build", &check.builds_predicted, &check.builds_actual, &mut check.mismatches);

        // After Advance, the card just explored sits in the Build slot.
        let card = after_e.invaders.build.first().map(|c| c.id.clone());
        check.explores_predicted = f
            .phase
            .explore
            .candidates
            .iter()
            .find(|c| Some(&c.card.id) == card.as_ref())
            .map(|c| c.lands.clone())
            .unwrap_or_default();
        check.explores_actual = changed(after_b, after_e, |l| l.explorers);
        check.explore_card = card;
        diff("Explore", &check.explores_predicted, &check.explores_actual, &mut check.mismatches);
        out.push(check);
    }
    out
}
