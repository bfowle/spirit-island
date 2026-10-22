//! The rules against their laws over everything the definitions hold: every Power Card and
//! every innate (its tiers reached, or none) on every board's real layout, held to the
//! effects' laws (`effects::oracle`); every adversary at every level, and no adversary,
//! through Explore, Build and Ravage on every board with the definitions' Invader cards,
//! held to the Invader steps' laws (`invaders::oracle`). `si fuzz --rules` runs it; a
//! violation names the seed that builds its case again.

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::element::{Element, ElementSet};
use si_model::event::InvaderStep;
use si_model::island::{LandId, Terrain};
use si_model::oracle::{case_seed, Rng, Violation};

use crate::effects::{self, oracle as eo};
use crate::invaders::{self, oracle as io, Rules};
use crate::verify;

/// One Power's run: how many cases, which broke a law (seed, law, what was seen), and whether
/// every effect it has is one the simulator executes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PowerRun {
    pub id: String,
    pub title: String,
    pub spirit: Option<String>,
    pub cases: usize,
    pub broken: Vec<(u64, String, String)>,
    pub simulated: bool,
}

/// One adversary at one level (None: the base rules).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdversaryRun {
    pub adversary: Option<String>,
    pub level: u8,
    pub cases: usize,
    pub broken: Vec<(u64, String, String)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RulesReport {
    pub boards: Vec<String>,
    pub powers: Vec<PowerRun>,
    pub adversaries: Vec<AdversaryRun>,
    /// Printed sentences read as something other than what they say: (Power, the law, what
    /// was seen), from `si_defs::effects::sentence_holds`.
    #[serde(default)]
    pub stated: Vec<(String, String, String)>,
}

/// Every printed sentence of every Power Card (its thresholds too) and every innate (its
/// tiers and rider) held to the parser's law: read as what it says.
pub fn stated(defs: &Definitions) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut check = |who: &str, lines: &[String]| {
        for line in lines {
            for sentence in si_defs::effects::sentences(line) {
                let effects = si_defs::effects::parse_sentence(&sentence);
                for v in si_defs::effects::sentence_holds(&sentence, &effects) {
                    out.push((who.to_string(), v.law.to_string(), v.detail));
                }
            }
        }
    };
    let mut cards: Vec<&si_model::power::PowerCard> = defs.cards.values().collect();
    cards.sort_by(|a, b| a.title.cmp(&b.title));
    for c in cards {
        check(&c.title, &c.body);
        for t in &c.thresholds {
            check(&c.title, &t.body);
        }
    }
    for sp in defs.spirits.values() {
        for i in &sp.innates {
            let who = format!("{} (innate)", i.title);
            check(&who, &i.rider);
            for t in &i.tiers {
                check(&who, std::slice::from_ref(&t.body));
            }
        }
    }
    out
}

fn note(broken: &mut Vec<(u64, String, String)>, seed: u64, v: Vec<Violation>) {
    for x in v {
        if broken.len() < 5 {
            broken.push((seed, x.law.to_string(), x.detail));
        }
    }
}

fn every_element() -> ElementSet {
    let mut e = ElementSet::new();
    for x in Element::ALL {
        e.add(x, 9);
    }
    e
}

fn boards_of(defs: &Definitions) -> Vec<&si_model::island::BoardDef> {
    let mut boards: Vec<&si_model::island::BoardDef> = defs.boards.values().filter(|b| b.lands.iter().any(|l| l.terrain != Terrain::Ocean)).collect();
    boards.sort_by(|a, b| a.id.cmp(&b.id));
    boards
}

/// One board, or two of a kind (balanced with balanced, thematic with thematic) under
/// different land codes, as a game would set them out.
fn pick_boards<'a>(rng: &mut Rng, boards: &[&'a si_model::island::BoardDef]) -> Vec<&'a si_model::island::BoardDef> {
    let code = |b: &si_model::island::BoardDef| b.lands.first().map(|l| l.id.board.clone()).unwrap_or_default();
    let thematic = |b: &si_model::island::BoardDef| b.id.chars().count() > 1;
    let a = boards[rng.below(boards.len())];
    if rng.one_in(4) && boards.len() > 1 {
        let b = boards[rng.below(boards.len())];
        if code(b) != code(a) && thematic(a) == thematic(b) {
            return vec![a, b];
        }
    }
    vec![a]
}

/// The case a seed builds for a Power: the island, the land it targets, what the line says.
fn power_case(rng: &mut Rng, boards: &[&si_model::island::BoardDef]) -> Option<(effects::Ctx, crate::invaders::Island, LandId)> {
    let on = pick_boards(rng, boards);
    let island = io::arbitrary_island_on(rng, &on);
    let lands: Vec<LandId> = island.lands.values().filter(|l| l.terrain != Some(Terrain::Ocean) || rng.one_in(8)).map(|l| l.id.clone()).collect();
    let target = rng.pick(&lands).cloned()?;
    let mut ctx = eo::arbitrary_ctx(rng, &island, &target);
    ctx.elements = if rng.coin() { every_element() } else { ElementSet::new() };
    Some((ctx, island, target))
}

/// A Power's case built again from the seed a violation named, played and explained: the
/// target, what the line said, every land that changed, the notes, the laws it broke.
pub fn explain_power_case(defs: &Definitions, power: &str, seed: u64) -> Result<String, String> {
    let (base, thresholds) = verify::power_effects(defs, power).ok_or(format!("no Power {power}"))?;
    let boards = boards_of(defs);
    let (ctx, island, target) = power_case(&mut Rng::new(seed), &boards).ok_or("no case for that seed")?;
    let mut after = island.clone();
    let a = effects::apply(&base, &thresholds, &mut after, &target, &ctx);
    let mut out = vec![format!("{power} on {target}"), format!("asked {:?} exact {} kills {:?} site-beast {} beasts-far {} elements {}", ctx.moves, ctx.moves_exact, ctx.kills, ctx.site_beast, ctx.beasts_far, ctx.elements)];
    out.push(format!("effects run: {:?}", effects::reached_effects_in(&base, &thresholds, &ctx.elements, island.lands.get(&target), Some(&ctx))));
    for (id, l) in &after.lands {
        let b = &island.lands[id];
        if b.c != l.c || b.wounds != l.wounds {
            out.push(format!("{id}: {:?} {:?} -> {:?} {:?}", b.c, b.wounds, l.c, l.wounds));
        }
    }
    out.push(format!("moves {:?} killed {:?} destroyed {:?} notes {:?}", a.moves, a.killed, a.destroyed, a.notes));
    for v in eo::apply_holds(&base, &thresholds, &target, &ctx, &island, &after, &a) {
        out.push(format!("BROKEN {v}"));
    }
    Ok(out.join("\n"))
}

/// Every Power and adversary in the definitions, `cases` random states each.
pub fn fuzz_definitions(defs: &Definitions, cases: usize, seed: u64) -> RulesReport {
    let boards = boards_of(defs);
    let mut report = RulesReport { boards: boards.iter().map(|b| b.id.clone()).collect(), ..Default::default() };
    if boards.is_empty() {
        return report;
    }
    let pick_boards = |rng: &mut Rng| pick_boards(rng, &boards);
    // every Power Card and innate
    let mut powers: Vec<(String, String, Option<String>, bool)> = defs.cards.values().map(|c| (c.id.clone(), c.title.clone(), None, c.not_simulated().is_empty())).collect();
    for sp in defs.spirits.values() {
        for i in &sp.innates {
            let simulated = i.tiers.iter().flat_map(|t| t.effects.iter()).all(|e| e.is_other() || e.simulated());
            powers.push((i.id.clone(), i.title.clone(), Some(sp.id.clone()), simulated));
        }
    }
    powers.sort();
    for (k, (id, title, spirit, simulated)) in powers.into_iter().enumerate() {
        let Some((base, thresholds)) = verify::power_effects(defs, &id) else { continue };
        let mut run = PowerRun { id: id.clone(), title, spirit, cases: 0, broken: Vec::new(), simulated };
        for i in 0..cases {
            let s = case_seed(seed.wrapping_add((k as u64) << 24), i);
            let mut rng = Rng::new(s);
            let Some((ctx, island, target)) = power_case(&mut rng, &boards) else { continue };
            let mut after = island.clone();
            run.cases += 1;
            let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| effects::apply(&base, &thresholds, &mut after, &target, &ctx)));
            match ran {
                Err(_) => note(&mut run.broken, s, vec![Violation::new("no panic", "apply panicked")]),
                Ok(a) => note(&mut run.broken, s, eo::apply_holds(&base, &thresholds, &target, &ctx, &island, &after, &a)),
            }
        }
        report.powers.push(run);
    }
    // every adversary at every level, and the base rules
    let cards: Vec<&si_model::decks::InvaderCard> = defs.invader_cards.values().collect();
    let spirit_ids: Vec<String> = defs.spirits.keys().cloned().collect();
    let scenarios: Vec<String> = defs.scenarios.keys().cloned().collect();
    let mut runs: Vec<(Option<String>, u8)> = vec![(None, 0)];
    for a in defs.adversaries.values() {
        let top = a.levels.iter().map(|l| l.number).max().unwrap_or(6);
        for level in 0..=top {
            runs.push((Some(a.id.clone()), level));
        }
    }
    for (k, (adversary, level)) in runs.into_iter().enumerate() {
        let mut run = AdversaryRun { adversary: adversary.clone(), level, cases: 0, broken: Vec::new() };
        for i in 0..cases {
            let s = case_seed(seed.wrapping_add(0x5EED_0000 + ((k as u64) << 24)), i);
            let mut rng = Rng::new(s);
            let on = pick_boards(&mut rng);
            let island = io::arbitrary_island_on(&mut rng, &on);
            let Some(card) = rng.pick(&cards).map(|c| (*c).clone()) else { continue };
            // a spirit or two in play, their special rules with them, and now and then a scenario
            let mut spirits: Vec<String> = Vec::new();
            for _ in 0..rng.between(0, 2) {
                if let Some(sp) = rng.pick(&spirit_ids) {
                    spirits.push(sp.clone());
                }
            }
            let scenario = if rng.one_in(6) { rng.pick(&scenarios).cloned() } else { None };
            let rules = Rules { adversary: adversary.clone(), level, terror: rng.between(1, 3) as u8, hooks: Rules::hooks_of(defs, &spirits), spirits, scenario, ..Default::default() };
            let step = [InvaderStep::Explore, InvaderStep::Build, InvaderStep::Ravage][rng.below(3)];
            let mut after = island.clone();
            run.cases += 1;
            let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match step {
                InvaderStep::Explore => invaders::explore(&mut after, &card, &rules),
                InvaderStep::Build => invaders::build(&mut after, &card, &rules),
                InvaderStep::Ravage => invaders::ravage(&mut after, &card, &rules),
            }));
            let Ok(outcomes) = ran else {
                note(&mut run.broken, s, vec![Violation::new("no panic", format!("{step:?} panicked"))]);
                continue;
            };
            let mut v = io::adversary_holds(step, &island, &after, &rules);
            v.extend(io::victory_holds(&after, &rules));
            // the base rules, exactly (an escalation is an adversary's)
            if adversary.is_none() && !card.escalation && rules.hooks.is_empty() && rules.scenario.is_none() {
                v.extend(match step {
                    InvaderStep::Explore => io::explore_holds(&card, &island, &after),
                    InvaderStep::Build => io::build_holds(&card, &island, &after),
                    InvaderStep::Ravage => io::ravage_holds(&card, &island, &after, &outcomes),
                });
            }
            note(&mut run.broken, s, v);
        }
        report.adversaries.push(run);
    }
    report.stated = stated(defs);
    report
}
