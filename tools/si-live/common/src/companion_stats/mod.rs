//! Companion stats: pure functions of `PublicSnapshot` plus the game's
//! history. Nothing here can see hidden information; odds come from public
//! pools (Section 5 of the Steam companion spec).
//!
//! Posture (spec 7.6, "augment, don't decide"): every output answers a named
//! question with its parts and a pocket rule. Nothing ranks options or names
//! a best move.

pub mod adversary;
pub mod backtest;
pub mod changes;
pub mod invaders;
pub mod needs;
pub mod review;
pub mod spirits;

use crate::public::{PublicLand, PublicSnapshot};
use crate::rules;
use invaders::{InvaderPhaseForecast, LandOutlook, StageCount};
use serde::{Deserialize, Serialize};
use spirits::SpiritStats;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanionStats {
    pub invader_phase: InvaderPhaseForecast,
    pub lands: Vec<LandOutlook>,
    pub stages: Vec<StageCount>,
    pub fear: FearStats,
    pub blight: BlightStats,
    pub spirits: Vec<SpiritStats>,
    pub needs: Vec<needs::SpiritNeeds>,
    /// Adversary, spirit, and aspect rules in play that touch the invader
    /// pipeline (rules register), and whether the forecast applies them.
    pub rules_in_play: Vec<crate::rules_register::RuleInPlay>,
    pub prompts: Vec<Prompt>,
    /// Each adversary's additional loss condition as a count against its limit.
    #[serde(default)]
    pub loss_clocks: Vec<adversary::LossClock>,
    /// Next turn's Invader Phase, played forward from this turn's forecast
    /// (`None` once this turn's Invader Phase is over: then `invader_phase`
    /// is next turn's).
    #[serde(default)]
    pub next_phase: Option<InvaderPhaseForecast>,
    /// Assumptions every number above rests on.
    pub basis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Prompt {
    pub question: String,
    /// Which panel answers it.
    pub panel: String,
    /// The facts from this game that make it relevant now.
    pub facts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FearStats {
    pub generated: i64,
    pub required_per_card: i64,
    pub to_next_card: i64,
    pub terror_level: u8,
    pub cards_left_by_level: [u32; 3],
    /// Fear to reach the next Terror Level; `None` at Terror Level III.
    pub to_next_level: Option<i64>,
    pub to_clear_deck: i64,
    pub earned_banked: u32,
    pub victory_condition: String,
    pub victory_blockers: u32,
    /// Fear from destroying invaders in this phase's counterattacks, best case.
    pub counterattack_fear_best: i64,
    pub per_turn: Vec<(u32, i64)>,
    pub pocket_rule: String,
}

/// Next turn's Ravage against the Blight card, before anyone acts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NextRavage {
    /// The Ravage card(s) next turn (this turn's Build cards).
    pub cards: Vec<String>,
    /// Lands that add Blight next turn if nothing changes.
    pub lands: Vec<String>,
    /// Fewest Blight they add (cascades to the nearest unblighted land).
    pub adds_min: i64,
    /// Blight left on the card after this turn's forecast; `None` when this
    /// turn flips the card (the new side's count isn't known yet).
    pub remaining_after_this: Option<i64>,
    /// "card holds", "card flips", "card runs out (loss)",
    /// "card runs out (draw a new Blight Card)".
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BlightStats {
    pub remaining: i64,
    pub flipped: bool,
    pub on_island: u32,
    /// Blight this phase's Ravages add if nothing changes: fewest (each
    /// cascade sent to the nearest unblighted land) and with every cascade.
    pub adds_this_phase_min: i64,
    pub ravage_lands_adding: Vec<String>,
    /// "card holds", "card flips", "card runs out (loss)", from the minimum.
    pub outcome_this_phase: String,
    /// Before the flip: (blight the flipped side would add, how many unseen cards show it).
    pub flipped_side: Vec<(i64, u32)>,
    /// Before the flip: how many of those cards are Still-Healthy.
    pub flipped_still_healthy: u32,
    /// Chance of losing to Blight if this many more Blight come off the card
    /// this phase: (Blight, probability), over the cards a draw could bring,
    /// through Still-Healthy chains. Immediately effects are not counted.
    pub loss_risk: Vec<(i64, f64)>,
    /// `loss_risk` at this phase's forecast (`adds_this_phase_min`).
    pub loss_risk_now: f64,
    pub cascade_chains: Vec<CascadeChain>,
    /// Next turn's Ravage, played forward from this turn's forecast.
    #[serde(default)]
    pub next_turn: Option<NextRavage>,
    pub per_turn: Vec<(u32, u32)>,
    pub pocket_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CascadeChain {
    pub land: String,
    /// Fewest extra blight if a blight lands here (0 = no cascade).
    pub min_extra: i64,
    /// Neighbors without blight (where a cascade can stop).
    pub safe_neighbors: Vec<String>,
    /// The connected cluster of blighted lands this land belongs to: how far
    /// a cascade can travel through already-blighted lands (spec 7.9).
    pub cluster: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RoundSummary {
    pub turn: u32,
    pub last_step: String,
    pub fear_total: i64,
    pub fear_cards_earned: u32,
    pub terror_level: u8,
    pub blight_on_island: u32,
    pub blight_remaining: i64,
    pub blight_flipped: bool,
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    pub dahan: u32,
    pub invaded_lands: u32,
    pub presence_on_island: u32,
    pub presence_destroyed: u32,
    pub energy: i64,
    pub cards_in_hand: u32,
    pub snapshots: u32,
}

pub fn compute(s: &PublicSnapshot, history: &[RoundSummary]) -> CompanionStats {
    let view = invaders::forecast(s);
    let fear = fear_stats(s, &view.phase, history);
    let mut blight = blight_stats(s, &view.phase, history);
    let next = invaders::next_turn(s, &view);
    // Moot when this turn already empties the card.
    if !blight.outcome_this_phase.starts_with("card runs out (loss") {
        blight.next_turn = next.as_ref().map(|n| next_ravage(s, n, &blight));
    }
    let spirits = s.spirits.iter().map(|sp| spirits::spirit_stats(sp, s)).collect();
    let open_ravages: Vec<(String, rules::RavageLedger)> =
        view.phase.ravages.iter().filter(|r| !r.done).map(|r| (r.land.clone(), r.ledger.clone())).collect();
    let needs = s.spirits.iter().map(|sp| needs::spirit_needs(sp, s, &open_ravages)).collect();
    let mut rules_in_play: Vec<crate::rules_register::RuleInPlay> =
        s.adversaries.iter().flat_map(|a| crate::rules_register::adversary_rules(&a.id, a.level)).collect();
    rules_in_play.extend(s.spirits.iter().flat_map(|sp| crate::rules_register::spirit_rules(&sp.id, &sp.title, sp.aspect.as_deref())));
    let prompts = prompts(s, &view.phase, &view.lands, &fear, &blight);
    let loss_clocks = adversary::loss_clocks(&adversary::Adv::new(s), &view.after_build, &view.phase.ravages, &view.phase.build_ravages);
    CompanionStats {
        invader_phase: view.phase,
        lands: view.lands,
        stages: view.stages,
        fear,
        blight,
        spirits,
        needs,
        rules_in_play,
        prompts,
        loss_clocks,
        next_phase: next.map(|n| n.phase),
        basis: vec![
            "Adversary and scenario rules are applied where the forecast models them; rules the players resolve by choice are listed as options, not applied.".into(),
            "Explore odds treat each unseen card of the next stage as equally likely (pool − seen); the deck is never read.".into(),
            "Explore sources are counted after this phase's Builds; Ravage counterattack kills are not assumed.".into(),
        ],
    }
}

fn fear_stats(s: &PublicSnapshot, phase: &InvaderPhaseForecast, history: &[RoundSummary]) -> FearStats {
    let f = &s.fear;
    let req = f.required_per_card.max(1);
    let lvl = f.terror_level.clamp(1, 3);
    let at_level = f.deck_by_level[(lvl - 1) as usize] as i64;
    let total_left: i64 = f.deck_by_level.iter().map(|n| *n as i64).sum();
    let to_next_level = (lvl < 3).then(|| (at_level * req - f.generated).max(0));
    let (cond, blockers) = rules::terror_victory_blockers(lvl, s);
    let counter: i64 = phase.ravages.iter().filter(|r| !r.done).map(|r| r.ledger.counterattack_fear).sum();
    let mut per_turn = Vec::new();
    let mut prev = 0;
    for h in history {
        per_turn.push((h.turn, h.fear_total - prev));
        prev = h.fear_total;
    }
    FearStats {
        generated: f.generated,
        required_per_card: req,
        to_next_card: (req - f.generated).max(0),
        terror_level: lvl,
        cards_left_by_level: f.deck_by_level,
        to_next_level,
        to_clear_deck: (total_left * req - f.generated).max(0),
        earned_banked: f.earned_count,
        victory_condition: cond,
        victory_blockers: blockers,
        counterattack_fear_best: counter,
        per_turn,
        pocket_rule: if lvl < 3 {
            format!("Fear to the next Terror Level = cards left at this level × {req} − fear on the pool ({at_level} × {req} − {}).", f.generated)
        } else {
            format!("Fear to clear the deck = cards left × {req} − fear on the pool ({total_left} × {req} − {}).", f.generated)
        },
    }
}

/// Fewest extra blight a cascade from `start` causes: the distance to the
/// nearest land without blight (Oceans excluded), 0 if `start` has none.
fn cascade_min_extra(start: &str, lands: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> (i64, Vec<String>) {
    let blighted = |id: &str| lands.iter().find(|l| l.id == id).map(|l| l.blight > 0).unwrap_or(false);
    let ocean = |id: &str| lands.iter().find(|l| l.id == id).map(|l| l.is_ocean).unwrap_or(true);
    if !blighted(start) {
        return (0, Vec::new());
    }
    let safe: Vec<String> = nbrs.get(start).into_iter().flatten().filter(|n| !ocean(n) && !blighted(n)).map(|n| n.to_string()).collect();
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([(start, 0i64)]);
    while let Some((id, d)) = queue.pop_front() {
        for n in nbrs.get(id).into_iter().flatten() {
            if ocean(n) || !seen.insert(n) {
                continue;
            }
            if !blighted(n) {
                return (d + 1, safe);
            }
            queue.push_back((n, d + 1));
        }
    }
    (seen.len() as i64, safe)
}

fn blighted_cluster(start: &str, lands: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> Vec<String> {
    let blighted = |id: &str| lands.iter().any(|l| l.id == id && l.blight > 0 && !l.is_ocean);
    let mut seen = BTreeSet::from([start]);
    let mut stack = vec![start];
    while let Some(id) = stack.pop() {
        for n in nbrs.get(id).into_iter().flatten() {
            if blighted(n) && seen.insert(n) {
                stack.push(n);
            }
        }
    }
    seen.into_iter().map(String::from).collect()
}

/// Fewest Blight a phase's Ravages add on `lands`, and the lands adding it.
fn phase_blight(phase: &InvaderPhaseForecast, lands: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> (i64, Vec<String>) {
    let mut adds = 0;
    let mut adding = Vec::new();
    for r in phase.ravages.iter().chain(&phase.build_ravages).filter(|r| !r.done && r.ledger.adds_blight) {
        let extra = if r.ledger.cascades { cascade_min_extra(&r.land, lands, nbrs).0 } else { 0 };
        adds += 1 + extra + r.ledger.extra_blight;
        adding.push(r.land.clone());
    }
    (adds, adding)
}

fn next_ravage(s: &PublicSnapshot, next: &invaders::InvaderView, b: &BlightStats) -> NextRavage {
    let nbrs = rules::neighbors(s);
    let (adds_min, lands) = phase_blight(&next.phase, &next.after_build, &nbrs);
    // This turn flips the card, or empties a Still-Healthy side (a new Blight
    // Card is drawn): the next count isn't known yet.
    let changes_now = b.adds_this_phase_min > 0 && b.adds_this_phase_min >= b.remaining;
    let remaining_after_this = (!changes_now).then(|| b.remaining - b.adds_this_phase_min);
    let still_healthy = s.blight.blighted_text.iter().any(|t| t.contains("draw a new Blight Card"));
    let outcome = match remaining_after_this {
        Some(left) if adds_min > 0 && adds_min >= left => {
            if !b.flipped {
                "card flips"
            } else if still_healthy {
                "card runs out (draw a new Blight Card)"
            } else {
                "card runs out (loss)"
            }
        }
        Some(_) => "card holds",
        None if adds_min > 0 => "card changes this turn (it flips, or a new Blight Card is drawn); next turn's Ravage goes on the new side",
        None => "card holds",
    };
    NextRavage { cards: next.phase.ravage_cards.iter().map(|c| c.id.clone()).collect(), lands, adds_min, remaining_after_this, outcome: outcome.into() }
}

fn blight_stats(s: &PublicSnapshot, phase: &InvaderPhaseForecast, history: &[RoundSummary]) -> BlightStats {
    let b = &s.blight;
    let nbrs = rules::neighbors(s);
    let (adds, adding) = phase_blight(phase, &s.lands, &nbrs);
    // A Still-Healthy flipped side draws a new Blight Card when it empties
    // instead of losing (JE rulebook; open-questions Q11).
    let still_healthy = b.blighted_text.iter().any(|t| t.contains("draw a new Blight Card"));
    let outcome = if b.flipped {
        if adds >= b.remaining && adds > 0 {
            if still_healthy { "card runs out (draw a new Blight Card)" } else { "card runs out (loss)" }
        } else {
            "card holds"
        }
    } else if adds >= b.remaining && adds > 0 {
        "card flips"
    } else {
        "card holds"
    };
    let mut dist: BTreeMap<i64, u32> = BTreeMap::new();
    for v in b.unseen_flipped.iter().flatten() {
        *dist.entry(*v).or_default() += 1;
    }
    let cascade_chains = s
        .lands
        .iter()
        .filter(|l| l.blight > 0 && !l.is_ocean)
        .map(|l| {
            let (min_extra, safe_neighbors) = cascade_min_extra(&l.id, &s.lands, &nbrs);
            CascadeChain { land: l.id.clone(), min_extra, safe_neighbors, cluster: blighted_cluster(&l.id, &s.lands, &nbrs) }
        })
        .collect();
    BlightStats {
        remaining: b.remaining,
        flipped: b.flipped,
        on_island: b.on_island,
        adds_this_phase_min: adds,
        ravage_lands_adding: adding,
        outcome_this_phase: outcome.into(),
        flipped_side: dist.into_iter().collect(),
        flipped_still_healthy: b.unseen_still_healthy,
        loss_risk: (0..=(adds + 4).max(6)).map(|k| (k, p_loss(k, b))).collect(),
        loss_risk_now: p_loss(adds, b),
        cascade_chains,
        next_turn: None,
        per_turn: history.iter().map(|h| (h.turn, h.blight_on_island)).collect(),
        pocket_rule: "Each blight added comes off the card; a blight added to a land that already has blight cascades to an adjacent land.".into(),
    }
}

/// Chance the island is lost to Blight if `k` more Blight come off the card
/// (playtest U16). A Blighted side that empties loses; a Still-Healthy side
/// that empties draws a new card, already flipped, from the cards not yet
/// seen; before the flip the current card is one of the cards sharing its
/// healthy side. Built from the public pool only; Immediately effects that
/// add Blight are not counted, so this is a floor.
pub fn p_loss(k: i64, b: &crate::public::PublicBlight) -> f64 {
    if k < b.remaining || b.remaining <= 0 && b.pool.is_empty() {
        return 0.0;
    }
    let pool: Vec<(i64, bool)> = b.pool.iter().filter_map(|o| o.flipped_total.map(|x| (x, o.still_healthy))).collect();
    let left = k - b.remaining;
    if b.flipped {
        return if b.current_still_healthy { draw_loss(left, &pool) } else { 1.0 };
    }
    // Before the flip: the current card is one of the same-healthy-side cards.
    let group: Vec<usize> = b
        .pool
        .iter()
        .filter(|o| o.flipped_total.is_some())
        .enumerate()
        .filter(|(_, o)| o.same_healthy_side)
        .map(|(i, _)| i)
        .collect();
    if group.is_empty() {
        return 0.0;
    }
    let total: f64 = group
        .iter()
        .map(|&i| {
            let (x, still_healthy) = pool[i];
            let rest: Vec<(i64, bool)> = pool.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, c)| *c).collect();
            match (left >= x, still_healthy) {
                (false, _) => 0.0,
                (true, true) => draw_loss(left - x, &rest),
                (true, false) => 1.0,
            }
        })
        .sum();
    total / group.len() as f64
}

/// Loss chance when `k` Blight remain to place and a new card (already
/// flipped) is drawn from `pool` (Blight on its flipped side, Still-Healthy).
fn draw_loss(k: i64, pool: &[(i64, bool)]) -> f64 {
    if k <= 0 {
        return 0.0;
    }
    if pool.is_empty() {
        return 1.0;
    }
    let total: f64 = pool
        .iter()
        .enumerate()
        .map(|(i, &(x, still_healthy))| match (k >= x, still_healthy) {
            (false, _) => 0.0,
            (true, true) => {
                let rest: Vec<(i64, bool)> = pool.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, c)| *c).collect();
                draw_loss(k - x, &rest)
            }
            (true, false) => 1.0,
        })
        .sum();
    total / pool.len() as f64
}

fn prompts(s: &PublicSnapshot, phase: &InvaderPhaseForecast, lands: &[LandOutlook], fear: &FearStats, blight: &BlightStats) -> Vec<Prompt> {
    let mut out = Vec::new();
    let ravage_lands: Vec<&str> = phase.ravages.iter().filter(|r| !r.done).map(|r| r.land.as_str()).collect();
    if !ravage_lands.is_empty() {
        let mut facts: Vec<String> = phase
            .ravages
            .iter()
            .filter(|r| !r.done)
            .map(|r| {
                let l = &r.ledger;
                let margin = l.damage_after_defend - (l.land_health - 1);
                format!(
                    "{}: {} damage vs health {} → {}{}",
                    r.land,
                    l.damage_after_defend,
                    l.land_health,
                    if l.adds_blight { format!("blight (reduce damage by {margin} to save)") } else { "no blight".into() },
                    if l.cascades { ", cascades" } else { "" }
                )
            })
            .collect();
        facts.push(format!("Blight card: {} left, {} this phase at least → {}", blight.remaining, blight.adds_this_phase_min, blight.outcome_this_phase));
        out.push(Prompt { question: "Which Ravage land is one action from safe, and which is not worth saving?".into(), panel: "Ravage ledger".into(), facts });
    }
    let builds: Vec<String> = phase.builds.iter().filter(|b| !b.done).map(|b| format!("{} +{}", b.land, b.adds)).collect();
    if !builds.is_empty() {
        let next: Vec<&str> = lands.iter().filter(|l| l.badges.iter().any(|b| b == "ravage-next")).map(|l| l.id.as_str()).collect();
        out.push(Prompt {
            question: "Stop a Build now, or meet it as a Ravage next turn?".into(),
            panel: "Pipeline".into(),
            facts: vec![format!("Building this turn: {}", builds.join(", ")), format!("These lands Ravage next turn: {}", next.join(", "))],
        });
    }
    let sole: Vec<String> = lands
        .iter()
        .filter(|l| !l.sole_source_for.is_empty())
        .map(|l| format!("{} feeds {}", l.id, l.sole_source_for.join(", ")))
        .collect();
    if !sole.is_empty() {
        out.push(Prompt {
            question: "Which inland lands can you make unexplorable by clearing one source?".into(),
            panel: "Explore access".into(),
            facts: sole,
        });
    }
    let coastal_hits: Vec<&str> = lands.iter().filter(|l| l.coastal && l.explore_chance.0 > 0).map(|l| l.id.as_str()).collect();
    if !coastal_hits.is_empty() {
        out.push(Prompt {
            question: "Coastal lands can always be explored: is clearing them buying a turn or a permanent change?".into(),
            panel: "Explore odds".into(),
            facts: vec![format!("Coastal lands the next card may hit: {}", coastal_hits.join(", "))],
        });
    }
    if fear.earned_banked > 0 || fear.to_next_level.is_some_and(|f| f <= fear.required_per_card) {
        out.push(Prompt {
            question: "Can you cross the next Terror Level before the Fear phase?".into(),
            panel: "Fear".into(),
            facts: vec![
                format!("{} fear card(s) banked, face down", fear.earned_banked),
                format!("{} fear to the next card; {} to the next Terror Level", fear.to_next_card, fear.to_next_level.map(|v| v.to_string()).unwrap_or("—".into())),
            ],
        });
    }
    if matches!(s.step.as_str(), "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower") {
        out.push(Prompt {
            question: "What is certain this turn, and what is still hidden?".into(),
            panel: "Pipeline".into(),
            facts: vec![
                format!("Certain: Ravage {:?}, Build {:?}", phase.ravage_cards.iter().map(|c| &c.id).collect::<Vec<_>>(), phase.build_cards.iter().map(|c| &c.id).collect::<Vec<_>>()),
                format!("Hidden: next Explore card ({}), Event, power draws", phase.explore.pocket_rule),
            ],
        });
    }
    out
}

/// One summary per turn: the last snapshot of each turn in the timeline.
pub fn round_summaries(timeline: &[PublicSnapshot]) -> Vec<RoundSummary> {
    let mut out: Vec<RoundSummary> = Vec::new();
    for s in timeline {
        let sum = |f: fn(&PublicLand) -> u32| s.lands.iter().map(f).sum::<u32>();
        let row = RoundSummary {
            turn: s.turn,
            last_step: s.step.clone(),
            fear_total: (s.fear.earned_count as i64 + s.fear.discard.len() as i64) * s.fear.required_per_card + s.fear.generated,
            fear_cards_earned: s.fear.earned_count + s.fear.discard.len() as u32,
            terror_level: s.fear.terror_level,
            blight_on_island: s.blight.on_island,
            blight_remaining: s.blight.remaining,
            blight_flipped: s.blight.flipped,
            explorers: sum(|l| l.explorers),
            towns: sum(|l| l.towns),
            cities: sum(|l| l.cities),
            dahan: sum(|l| l.dahan),
            invaded_lands: s.lands.iter().filter(|l| rules::has_invaders(l)).count() as u32,
            presence_on_island: s.spirits.iter().map(|x| x.presence_on_island).sum(),
            presence_destroyed: s.spirits.iter().map(|x| x.presence_destroyed).sum(),
            energy: s.spirits.iter().map(|x| x.energy).sum(),
            cards_in_hand: s.spirits.iter().map(|x| x.hand.len() as u32).sum(),
            snapshots: 1,
        };
        match out.last_mut() {
            Some(last) if last.turn == row.turn => {
                let n = last.snapshots + 1;
                *last = RoundSummary { snapshots: n, ..row };
            }
            _ => out.push(row),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{BlightOption, PublicBlight};

    /// Playtest 2026-10-19 Turn 5: Strong Earth Shatters Slowly (Still-Healthy,
    /// 1 left) with 22 unseen cards; 5 Blight forecast.
    fn turn5() -> PublicBlight {
        let mut pool = Vec::new();
        let mut add = |x: i64, n: usize, still_healthy: bool| {
            for _ in 0..n {
                pool.push(BlightOption { flipped_total: Some(x), still_healthy, same_healthy_side: false });
            }
        };
        add(1, 1, true);
        add(2, 4, false);
        add(2, 2, true);
        add(3, 5, false);
        add(4, 7, false);
        add(5, 3, false);
        PublicBlight { flipped: true, remaining: 1, current_still_healthy: true, pool, ..Default::default() }
    }

    #[test]
    fn loss_risk_through_still_healthy_chains() {
        let b = turn5();
        let pct = |k| (p_loss(k, &b) * 100.0).round() as i64;
        assert_eq!((pct(5), pct(4), pct(3), pct(2), pct(1)), (76, 42, 18, 0, 0));
        // A Blighted side loses as soon as it empties.
        let blighted = PublicBlight { flipped: true, remaining: 3, ..Default::default() };
        assert_eq!((p_loss(2, &blighted), p_loss(3, &blighted)), (0.0, 1.0));
    }
}
