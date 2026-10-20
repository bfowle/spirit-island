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
    /// Forecast actions that didn't happen because the player used an
    /// available skip (Choke the Land with Green, an optional Skip): choices,
    /// not forecast errors.
    pub skipped_by_choice: Vec<String>,
    /// England L3+: the High Immigration Build before the Ravage, forecast on
    /// entering that step.
    #[serde(default)]
    pub high_immigration_predicted: Vec<String>,
    #[serde(default)]
    pub high_immigration_actual: Vec<String>,
    /// A Building Escalation (England): the tied lands it may pick, and where
    /// it built.
    #[serde(default)]
    pub escalation_predicted: Vec<String>,
    #[serde(default)]
    pub escalation_actual: Vec<String>,
    /// Adversary actions after the Build Step the players pick (Mining
    /// Boom): where they happened. Not Builds.
    #[serde(default)]
    pub rule_actions_actual: Vec<String>,
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

/// Lands that gained a Town or City from `a` to `b`, as "Land +Piece".
fn built(a: &PublicSnapshot, b: &PublicSnapshot) -> Vec<String> {
    let mut out = Vec::new();
    for lb in &b.lands {
        let Some(la) = land(a, &lb.id) else { continue };
        if lb.cities > la.cities {
            out.push(format!("{} +City", lb.id));
        } else if lb.towns > la.towns {
            out.push(format!("{} +Town", lb.id));
        }
    }
    out
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

        if let Some(pre_hi) = first(timeline, t, "HighImmigration") {
            let fh = invaders::forecast(pre_hi);
            check.high_immigration_predicted = fh.phase.high_immigration.iter().map(|b| format!("{} +{}", b.land, b.adds)).collect();
            check.high_immigration_actual = built(pre_hi, pre);
            diff("High Immigration", &check.high_immigration_predicted, &check.high_immigration_actual, &mut check.mismatches);
        }

        check.ravage_blight_predicted = f.phase.ravages.iter().filter(|r| r.ledger.adds_blight).map(|r| r.land.clone()).collect();
        let ravaged: BTreeSet<&str> = f.phase.ravages.iter().map(|r| r.land.as_str()).collect();
        check.ravage_blight_actual = changed(pre, after_r, |l| l.blight).into_iter().filter(|id| ravaged.contains(id.as_str())).collect();
        // A cascade from a predicted cascading Ravage can land in another
        // Ravage land (the player picks the target): that is the cascade, not
        // a missed Ravage Blight (playtest Turn 5: B2's cascade into B1).
        let nbrs = crate::rules::neighbors(pre);
        let cascade_targets: BTreeSet<&str> = f
            .phase
            .ravages
            .iter()
            .filter(|r| r.ledger.cascades)
            .flat_map(|r| nbrs.get(r.land.as_str()).into_iter().flatten().copied())
            .collect();
        check.ravage_blight_actual.retain(|id| {
            check.ravage_blight_predicted.contains(id) || !cascade_targets.contains(id.as_str())
        });
        for r in f.phase.ravages.iter().filter(|r| r.ledger.adds_blight && r.ledger.skip_available.is_some()) {
            if !check.ravage_blight_actual.contains(&r.land) {
                check.skipped_by_choice.push(format!("{} Ravage ({})", r.land, r.ledger.skip_available.clone().unwrap_or_default()));
                check.ravage_blight_predicted.retain(|l| *l != r.land);
            }
        }
        diff("Ravage blight", &check.ravage_blight_predicted, &check.ravage_blight_actual, &mut check.mismatches);
        // How much Blight each predicted land took (Heavy Mining adds 2).
        for r in f.phase.ravages.iter().filter(|r| r.ledger.adds_blight && check.ravage_blight_actual.contains(&r.land)) {
            let got = land(after_r, &r.land).map(|l| l.blight).unwrap_or(0) as i64 - land(pre, &r.land).map(|l| l.blight).unwrap_or(0) as i64;
            let want = 1 + r.ledger.extra_blight;
            if got != want {
                check.mismatches.push(format!("Blight amount in {}: predicted {want}, actual {got}", r.land));
            }
        }

        check.builds_predicted = f.phase.builds.iter().map(|b| format!("{} +{}", b.land, b.adds)).collect();
        check.builds_actual = built(after_r, after_b);
        // Mining Boom (Habsburg Mining L3/L5) upgrades 1 Explorer per board
        // after the Build Step, before the save this compares: an Explorer
        // fewer and a Town more. Where that's all that changed it isn't a
        // Build (Habsburg game: Wetland NW3 and Mountain NE7).
        if f.phase.rule_actions.iter().any(|a| a.when == "after the Build" && a.rule.starts_with("Mining Boom")) {
            for lb in &after_b.lands {
                let Some(la) = land(after_r, &lb.id) else { continue };
                if lb.explorers < la.explorers && lb.towns > la.towns {
                    check.rule_actions_actual.push(format!("{} Explorer → Town", lb.id));
                    let entry = format!("{} +Town", lb.id);
                    let built_too = lb.towns >= la.towns + 2 || check.builds_predicted.contains(&entry) && lb.towns + lb.cities > la.towns + la.cities + 1;
                    if !built_too {
                        check.builds_actual.retain(|b| *b != entry);
                    }
                }
            }
        }
        for b in f.phase.builds.iter().filter(|b| b.notes.iter().any(|n| n.starts_with("Can skip") || n.starts_with("Skip available"))) {
            let entry = format!("{} +{}", b.land, b.adds);
            if !check.builds_actual.contains(&entry) {
                check.skipped_by_choice.push(format!("{entry} Build"));
                check.builds_predicted.retain(|x| *x != entry);
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
        // A Building Escalation on a flagged card builds in one of the tied
        // lands per board before the Explore. Its Town/City is not an Explore.
        let mut escalated: BTreeSet<String> = BTreeSet::new();
        let explored_card = after_e.invaders.build.first().filter(|c| c.escalation);
        if let (Some(e), Some(c)) = (f.phase.escalation.as_ref().filter(|e| e.each_land), explored_card) {
            // Sweden: every predicted land for the drawn card swaps a Dahan
            // for a Town.
            check.escalation_predicted =
                e.builds.iter().filter(|b| b.card.as_deref().is_none_or(|id| id == c.id)).map(|b| format!("{} +{}", b.land, b.adds)).collect();
            check.escalation_actual = after_e
                .lands
                .iter()
                .filter(|lb| land(after_b, &lb.id).is_some_and(|la| lb.towns > la.towns && lb.dahan < la.dahan))
                .map(|l| format!("{} +Town", l.id))
                .collect();
            escalated = check.escalation_actual.iter().filter_map(|b| b.split(' ').next()).map(String::from).collect();
            diff("Escalation", &check.escalation_predicted, &check.escalation_actual, &mut check.mismatches);
        } else if let Some(e) = f.phase.escalation.as_ref().filter(|e| explored_card.is_some() && e.builds.iter().all(|b| b.adds == "Explorer")) {
            // Explorers placed by the Escalation (Russia: among lands with
            // Beasts; the players split them), not by the Explore card.
            let drawn = explored_card.map(|c| c.id.as_str());
            let mine: Vec<&super::invaders::BuildForecast> = e.builds.iter().filter(|b| b.card.is_none() || b.card.as_deref() == drawn).collect();
            let candidates: BTreeSet<&str> = mine.iter().map(|b| b.land.as_str()).collect();
            check.escalation_predicted = candidates.iter().map(|l| format!("{l} +Explorer")).collect();
            for id in changed(after_b, after_e, |l| l.explorers) {
                if candidates.contains(id.as_str()) && !check.explores_predicted.contains(&id) {
                    check.escalation_actual.push(format!("{id} +Explorer"));
                    escalated.insert(id);
                }
            }
        } else if let Some(e) = f.phase.escalation.as_ref().filter(|_| explored_card.is_some()) {
            check.escalation_predicted = e.builds.iter().map(|b| format!("{} +{}", b.land, b.adds)).collect();
            check.escalation_actual = built(after_b, after_e).into_iter().filter(|b| check.escalation_predicted.contains(b)).collect();
            escalated = check.escalation_actual.iter().filter_map(|b| b.split(' ').next()).map(String::from).collect();
            let boards: BTreeSet<&str> = e.builds.iter().filter_map(|b| land(pre, &b.land)).map(|l| l.board.as_str()).collect();
            if check.escalation_actual.len() != boards.len() {
                check.mismatches.push(format!("Escalation: expected one Build per board among {:?}, saw {:?}", check.escalation_predicted, built(after_b, after_e)));
            }
        }
        check.explores_actual.retain(|id| !escalated.contains(id) || check.explores_predicted.contains(id));
        // Promising Farmland places a Town instead of an Explorer in one
        // explored land per board; a Town added to a land matching the Explore
        // card with no new Explorer is that (Escalation Towns usually go to
        // non-matching lands). Playtest Turn 3, Mountain B2.
        if let Some(c) = after_e.invaders.build.first() {
            for id in changed(after_b, after_e, |l| l.towns) {
                if !escalated.contains(&id) && !check.explores_actual.contains(&id) && land(after_e, &id).is_some_and(|l| crate::rules::card_matches(c, l)) {
                    check.explores_actual.push(id);
                }
            }
            check.explores_actual.sort();
        }
        check.explore_card = card;
        diff("Explore", &check.explores_predicted, &check.explores_actual, &mut check.mismatches);
        out.push(check);
    }
    out
}

/// The Runway against the game: on entering each Ravage, the row for the
/// card that was then explored. Its Explore lands are exact (they don't
/// depend on the players), so they must match; its next-turn Blight is the
/// unanswered figure, set beside the Blight the next Invader Phase actually
/// added (the gap is what the spirits prevented, or a model error).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutlookCheck {
    pub turn: u32,
    pub card: Option<String>,
    /// The card was among the rows (the unseen pool held it).
    pub in_rows: bool,
    pub explores_row: Vec<String>,
    pub explores_actual: Vec<String>,
    /// Next turn's Blight, unanswered (fewest, most across the later cards).
    pub next_unanswered: Option<(i64, i64)>,
    /// Blight next turn's Invader Phase added (Ravage, cascades).
    pub next_actual: Option<i64>,
}

fn blight_added(a: &PublicSnapshot, b: &PublicSnapshot) -> i64 {
    b.lands.iter().map(|lb| (lb.blight as i64 - land(a, &lb.id).map(|l| l.blight as i64).unwrap_or(0)).max(0)).sum()
}

pub fn outlook_check(timeline: &[PublicSnapshot]) -> Vec<OutlookCheck> {
    let checks = backtest(timeline);
    let mut out = Vec::new();
    for c in &checks {
        let Some(pre) = first(timeline, c.turn, "Ravage") else { continue };
        let o = super::outlook::outlook(pre);
        let actual = c.explore_card.as_ref().and_then(|id| pre.invaders.pool.iter().find(|x| &x.id == id));
        let row = actual.and_then(|card| {
            let mut t = card.terrains.clone();
            t.sort();
            o.next_cards.iter().position(|r| {
                let mut rt = r.card.terrains.clone();
                rt.sort();
                r.card.stage == card.stage && rt == t && r.card.escalation == card.escalation
            })
        });
        let mut explores_row = row.map(|i| o.next_cards[i].explores.clone()).unwrap_or_default();
        explores_row.sort();
        let next_actual = match (first(timeline, c.turn + 1, "Ravage"), first(timeline, c.turn + 1, "SlowPower").or_else(|| first(timeline, c.turn + 1, "TimePasses"))) {
            (Some(a), Some(b)) => Some(blight_added(a, b)),
            _ => None,
        };
        out.push(OutlookCheck {
            turn: c.turn,
            card: c.explore_card.clone(),
            in_rows: row.is_some(),
            explores_row,
            explores_actual: c.explores_actual.clone(),
            next_unanswered: row.and_then(|i| o.next_cards[i].blight.get(1).copied()),
            next_actual,
        });
    }
    out
}

/// The race against the game: at the last decision save of a finished
/// game (the first save of its final turn), the ending the race names and
/// its incidence at that turn, beside the ending the engine recorded. For
/// a deterministic ending the incidence must be 1 in every order; for a
/// Blight loss it is the hidden side's share.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RaceCheck {
    pub ending: Option<String>,
    pub end_turn: u32,
    /// The save the race was computed at: (turn, step).
    pub at: (u32, String),
    pub turns: Vec<u32>,
    /// Each ending's title and incidence per turn.
    pub rows: Vec<(String, String, Vec<f64>)>,
    pub realized_key: Option<String>,
    /// The realized ending's incidence at the final turn.
    pub realized_cif: Option<f64>,
    /// The binding ending the race named, and its median turn.
    pub binding: Option<(String, Option<u32>)>,
    /// The largest incidence any other Invader-side ending had at the final turn.
    pub others_max: f64,
}

/// Map the engine's ending to a race key.
pub fn ending_key(ending: &str, race: &super::race::Race) -> Option<String> {
    let starts = |p: &str| race.endings.iter().find(|e| e.key.starts_with(p)).map(|e| e.key.clone());
    match ending {
        "LoseTooMuchBlight" => Some("blight".into()),
        "LoseAdversary" => starts("clock:"),
        "LoseTimeRunsOut" => Some("deck".into()),
        "LoseSpiritDestroyed" => starts("presence:"),
        "WinTerrorVictory" => Some("fear".into()),
        "WinInvadersDestroyed" => Some("board".into()),
        _ => None,
    }
}

pub fn race_at(timeline: &[PublicSnapshot], idx: usize) -> super::CompanionStats {
    let history = super::round_summaries(&timeline[..=idx]);
    super::compute(&timeline[idx], &history)
}

pub fn race_check(timeline: &[PublicSnapshot]) -> Option<RaceCheck> {
    let last = timeline.last()?;
    let end_turn = last.turn;
    let idx = timeline.iter().position(|s| s.turn == end_turn)?;
    let stats = race_at(timeline, idx);
    let race = &stats.race;
    let realized_key = last.ending.as_deref().and_then(|e| ending_key(e, race));
    let realized_cif = realized_key.as_deref().and_then(|k| race.at(k, end_turn));
    let others_max = race
        .endings
        .iter()
        .filter(|e| e.side == "invaders" && Some(&e.key) != realized_key.as_ref())
        .filter_map(|e| race.at(&e.key, end_turn))
        .fold(0.0, f64::max);
    Some(RaceCheck {
        ending: last.ending.clone(),
        end_turn,
        at: (timeline[idx].turn, timeline[idx].step.clone()),
        turns: race.turns.clone(),
        rows: race.endings.iter().map(|e| (e.key.clone(), e.title.clone(), e.cif.clone())).collect(),
        realized_key,
        realized_cif,
        binding: race.binding.map(|i| (race.endings[i].title.clone(), race.endings[i].median_turn)),
        others_max,
    })
}
