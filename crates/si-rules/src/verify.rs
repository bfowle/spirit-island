//! Replay every recorded turn's Invader phase through the simulator and
//! compare with what the log says happened. The log is the oracle: a mismatch
//! is a rule gap, a player's choice the simulation cannot know, or an
//! exogenous effect the record carries but the simulation does not model yet.
//! The engine's own status effects and `CancelAction` lines are read from the
//! record, so skips, cancels, extra Explorers and Defend grants from Fear and
//! Event cards are applied before the comparison.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::event::InvaderStep;
use si_model::game::Phase;
use si_model::island::{LandId, PieceKind};
use si_model::power::Effect;
use si_record::{GameRecord, InvaderVisit, TurnRecord};

use crate::effects;
use crate::invaders::{self, Island, Land, Outcome, Rules};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Check {
    pub turn: u32,
    pub step: String,
    pub card: String,
    pub land: String,
    pub field: String,
    pub simulated: String,
    pub logged: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Report {
    pub turns: u32,
    /// field → (matched, compared)
    pub fields: BTreeMap<String, (u32, u32)>,
    pub mismatches: Vec<Check>,
    /// Outcomes the log explains by something outside the step's rules (an
    /// Event or Fear card cancel, a Spirit rule), with the source.
    pub explained: Vec<Check>,
    pub notes: Vec<String>,
}

impl Report {
    pub fn matched(&self) -> u32 {
        self.fields.values().map(|(m, _)| m).sum()
    }
    pub fn compared(&self) -> u32 {
        self.fields.values().map(|(_, c)| c).sum()
    }
}

pub fn island_for(rec: &GameRecord, defs: &Definitions, before: &BTreeMap<LandId, si_record::Counts>) -> Island {
    let mut lands: BTreeMap<LandId, Land> = BTreeMap::new();
    for code in &rec.meta.boards {
        let Some(b) = defs.board_def(code) else { continue };
        for d in &b.lands {
            lands.insert(
                d.id.clone(),
                Land {
                    id: d.id.clone(),
                    terrain: Some(d.terrain),
                    coastal: d.coastal(),
                    adjacent: d.adjacent.iter().map(|n| LandId::new(&d.id.board, *n)).collect(),
                    c: before.get(&d.id).copied().unwrap_or_default(),
                    ..Default::default()
                },
            );
        }
    }
    for (id, c) in before {
        lands.entry(id.clone()).or_insert_with(|| Land { id: id.clone(), c: *c, ..Default::default() });
    }
    // the lands that touch across boards
    for (a, b) in &rec.meta.links {
        if lands.contains_key(a) && lands.contains_key(b) {
            for (x, y) in [(a, b), (b, a)] {
                if let Some(l) = lands.get_mut(x) {
                    if !l.adjacent.contains(y) {
                        l.adjacent.push(y.clone());
                    }
                }
            }
        }
    }
    Island { lands }
}

/// The island at one of a turn's snapshots (`before_fast`, `after_fast`, `before_invaders`,
/// `before_slow`, `after_slow`), with the Damage the pieces carried then.
pub fn island_at(rec: &GameRecord, defs: &Definitions, t: &TurnRecord, at: &str) -> Island {
    let counts = match at {
        "after_fast" => &t.after_fast,
        "before_invaders" => &t.before_invaders,
        "before_slow" => &t.before_slow,
        "after_slow" => &t.after_slow,
        _ => &t.before_fast,
    };
    with_wounds(island_for(rec, defs, counts), t.wounds.get(at))
}

/// The Damage pieces carry, put on the island's lands.
pub fn with_wounds(mut island: Island, wounds: Option<&BTreeMap<LandId, Vec<(PieceKind, u32)>>>) -> Island {
    for (id, w) in wounds.into_iter().flatten() {
        if let Some(l) = island.lands.get_mut(id) {
            l.wounds = w.clone();
            l.trim_wounds();
        }
    }
    island
}

/// Spirit Powers used in this turn's Fast phase whose printed effect skips an
/// Invader step in their target land. Not used by `verify` (the engine logs
/// the skip itself); kept for the simulate mode, where no log exists.
#[allow(dead_code)]
pub fn apply_fast_effects(island: &mut Island, rec: &GameRecord, defs: &Definitions, turn: u32) {
    for s in rec.spirits.iter().filter(|s| s.turn == turn) {
        for (phase, power, target) in &s.powers_used {
            if *phase != Phase::FastPower && *phase != Phase::SpiritPhase {
                continue;
            }
            let Some(target) = target else { continue };
            let effects: Vec<&Effect> = match defs.cards.get(power) {
                Some(c) => c.all_effects().collect(),
                None => defs.spirits.values().flat_map(|p| p.innates.iter()).filter(|i| i.id == *power).flat_map(|i| i.tiers.iter().flat_map(|t| t.effects.iter())).collect(),
            };
            let Some(l) = island.lands.get_mut(target) else { continue };
            for e in effects {
                flag(l, e);
            }
        }
    }
}

pub fn flag(l: &mut Land, e: &Effect) {
    match e {
        Effect::SkipRavage | Effect::SkipAll => l.skip_ravage = true,
        Effect::SkipBuild => l.skip_build = true,
        Effect::SkipExplore => l.skip_explore = true,
        Effect::NoParticipation(n) if *n < 255 => l.no_participation += *n as u32,
        Effect::InvaderDamageMinus(n) => l.damage_minus += *n as u32,
        Effect::Isolate => l.isolated = true,
        Effect::NoCascade => l.no_cascade = true,
        Effect::NoBlight => l.no_blight = true,
        Effect::DahanHealth(n) => l.dahan_health += *n as u32,
        Effect::DahanSimultaneous => l.dahan_simultaneous = true,
        Effect::Trigger { when, then, .. } if when.to_ascii_lowercase().contains("{blight} is added") => then.iter().for_each(|e| flag(l, e)),
        Effect::Conditional { then, .. } | Effect::Per { then, .. } => then.iter().for_each(|e| flag(l, e)),
        _ => {}
    }
}

/// Where the player put a Power's spread pieces this turn, as the log
/// recorded the choice: (the engine's reason, land), for one Power.
pub fn placements_of(defs: &Definitions, t: &TurnRecord, power: &str) -> Vec<(String, LandId)> {
    let norm = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let mut names = vec![norm(power)];
    if let Some(c) = defs.cards.get(power) {
        names.push(norm(&c.title));
    }
    if let Some(i) = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == power) {
        names.push(norm(&i.title));
    }
    t.choices.iter().filter(|c| c.kind == "Location" && names.contains(&norm(&c.source))).filter_map(|c| c.selected.parse::<LandId>().ok().map(|l| (c.reason.clone(), l))).collect()
}

/// Where the record has the spirit gather a presence as this Power targeted its land
/// (Shroud's Mists Shift and Flow): `Some(Some(land))` gathered into it, `Some(None)` the
/// player declined, None no such choice this turn.
pub fn presence_gather_of(defs: &Definitions, t: &TurnRecord, power: &str) -> Option<Option<LandId>> {
    let norm = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let mut names = vec![norm(power)];
    if let Some(c) = defs.cards.get(power) {
        names.push(norm(&c.title));
    }
    if let Some(i) = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == power) {
        names.push(norm(&i.title));
    }
    t.choices.iter().find(|c| c.kind == "Location" && c.reason == "Gather" && names.contains(&norm(&c.source))).map(|c| c.selected.parse::<LandId>().ok())
}

/// The piece and branch choices the log recorded for one Power this turn: (the engine's reason, what was selected).
pub fn picks_of(defs: &Definitions, t: &TurnRecord, power: &str) -> Vec<(String, String)> {
    let norm = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let mut names = vec![norm(power)];
    if let Some(c) = defs.cards.get(power) {
        names.push(norm(&c.title));
    }
    if let Some(i) = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == power) {
        names.push(norm(&i.title));
    }
    t.choices.iter().filter(|c| (c.kind == "Piece" || c.kind == "Effect") && names.contains(&norm(&c.source))).map(|c| (c.reason.clone(), c.selected.clone())).collect()
}

/// The "or" branches the log recorded this Power taking, as indexes into the card's printed
/// order (the engine lists the branches in that order).
pub fn branches_of(defs: &Definitions, t: &TurnRecord, power: &str) -> Vec<usize> {
    let norm = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let mut names = vec![norm(power)];
    if let Some(c) = defs.cards.get(power) {
        names.push(norm(&c.title));
    }
    if let Some(i) = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == power) {
        names.push(norm(&i.title));
    }
    t.choices.iter().filter(|c| c.kind == "Effect" && names.contains(&norm(&c.source))).filter_map(|c| c.choices.iter().position(|x| *x == c.selected)).collect()
}

fn list(v: &str) -> Vec<String> {
    v.trim().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

fn step_of(name: &str) -> Option<InvaderStep> {
    InvaderStep::parse(name)
}

/// The Ravages and Builds the player paid a presence to skip this turn (Rampant Green's Choke
/// the Land with Green), as the record has them: the cancels a Spirit paid for.
pub fn chokes_of(rec: &GameRecord, t: &TurnRecord) -> Vec<(InvaderStep, LandId)> {
    t.canceled
        .iter()
        .filter(|c| matches!(c.step, InvaderStep::Ravage | InvaderStep::Build) && (rec.meta.spirits.iter().any(|s| c.source.contains(s.as_str())) || c.source.contains("Choke")))
        .filter_map(|c| Some((c.step, c.land.clone()?)))
        .collect()
}

/// The `nth` use of a Power this turn, as the record has it.
pub fn use_of<'a>(t: &'a TurnRecord, power: &str, nth: usize) -> Option<&'a si_record::PowerUse> {
    t.power_uses.iter().filter(|u| u.power == power).nth(nth)
}

/// What a use destroyed, by land, as the effects take a recorded choice.
pub fn kills_of(u: &si_record::PowerUse) -> Vec<(LandId, si_model::power::PieceFilter)> {
    u.destroyed.iter().map(|(l, k)| (l.clone(), si_model::power::PieceFilter::parse(k))).collect()
}

/// What a use moved, each kind named: the gathers into its land, the pushes out of it, and
/// any other move it made.
pub fn moves_of_use(u: &si_record::PowerUse) -> Vec<crate::effects::Move> {
    let mut out: Vec<crate::effects::Move> = Vec::new();
    for (k, from, to) in &u.moves {
        let piece = Some(si_model::power::PieceFilter::parse(k));
        match out.iter_mut().find(|m| m.from == *from && m.to == *to && m.piece == piece) {
            Some(m) => m.n += 1,
            None => out.push(crate::effects::Move { from: from.clone(), to: to.clone(), piece, n: 1 }),
        }
    }
    out
}

/// The engine's status effects of the turn, read into rules and land flags.
pub fn apply_status_effects(island: &mut Island, rules: &mut Rules, t: &TurnRecord, notes: &mut Vec<String>) {
    for se in &t.status_effects {
        let p = |k: &str| se.params.get(k).cloned().unwrap_or_default();
        match se.kind.as_str() {
            "SkipInvaderActionsStatusEffect" => {
                let steps: Vec<InvaderStep> = list(&p("StatesToSkip")).iter().filter_map(|s| step_of(s)).collect();
                match p("LocationToSkip").parse::<LandId>() {
                    Ok(land) => {
                        if let Some(l) = island.lands.get_mut(&land) {
                            let all = steps.is_empty();
                            l.skip_ravage |= all || steps.contains(&InvaderStep::Ravage);
                            l.skip_build |= all || steps.contains(&InvaderStep::Build);
                            l.skip_explore |= all || steps.contains(&InvaderStep::Explore);
                        }
                    }
                    Err(_) => notes.push(format!("T{} {}: skip {:?} in a land of the player's choice", t.turn, se.source, steps)),
                }
            }
            "CancelActionStatusEffect" => {
                let ty = p("CancelActionType");
                let pieces = list(&p("IslandPieceTypes"));
                let targets = list(&p("TargetTypes"));
                if ty == "Build" && pieces.iter().any(|x| x == "City") && targets.iter().any(|x| x == "Coastal") {
                    rules.no_coastal_city = true;
                } else if let Ok(land) = p("TargetLocation").parse::<LandId>() {
                    if let Some(l) = island.lands.get_mut(&land) {
                        match ty.as_str() {
                            "Build" => l.skip_build = true,
                            "Ravage" => l.skip_ravage = true,
                            "Explore" => l.skip_explore = true,
                            _ => {}
                        }
                    }
                } else {
                    notes.push(format!("T{} {}: cancels a {ty} (which one is the player's choice; the log's CancelAction lines settle it)", t.turn, se.source));
                }
            }
            "ModifyInvaderActionStatusEffect" => {
                if list(&p("InvaderActionTypesToModify")).iter().any(|x| x == "Explore") {
                    if let Ok(n) = p("NumberOfExplorersModifier").parse::<u32>() {
                        rules.explore_bonus.push((n, p("TargetType")));
                    }
                }
            }
            "IsolateStatusEffect" => {
                for l in list(&p("LocationsToIsolate")) {
                    if let Ok(id) = l.parse::<LandId>() {
                        rules.isolated.push(id);
                    }
                }
            }
            "AdjustRavageDamageStatusEffect" => {
                if let Ok(a) = p("DamageAdjustment").parse::<i32>() {
                    let kinds: Vec<PieceKind> = list(&p("PerInvaderOfType")).iter().filter_map(|k| PieceKind::parse(k)).collect();
                    if p("PerInvaderOfType").trim().is_empty() {
                        notes.push(format!("T{} {}: {a:+} Ravage damage (condition {} not modeled)", t.turn, se.source, p("IfLandHasPresence")));
                    } else {
                        // the lands it applies to: the effect's own list, else the land the player
                        // chose for the same card this turn (the engine logs the effect before the
                        // choice, with an empty list), else every land
                        let norm = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
                        let mut lands: Vec<LandId> = list(&p("AffectedLands")).iter().filter_map(|l| l.parse::<LandId>().ok()).collect();
                        if lands.is_empty() {
                            lands = t.choices.iter().filter(|c| c.kind == "Location" && norm(&c.source) == norm(&se.source)).filter_map(|c| c.selected.parse::<LandId>().ok()).collect();
                        }
                        if lands.is_empty() {
                            rules.damage_adjust.push((a, kinds));
                        } else {
                            for land in lands {
                                if let Some(l) = island.lands.get_mut(&land) {
                                    l.damage_adjust.push((a, kinds.clone()));
                                }
                            }
                        }
                    }
                }
            }
            "ModifyPieceDamageStatusEffect" => {
                if let Ok(m) = p("DamageModifier").parse::<i32>() {
                    rules.piece_damage_mod += m;
                }
            }
            "AdjustHealthStatusEffect" => {
                let kinds = p("PieceTypesToAffect");
                if kinds.contains("Dahan") {
                    if let Ok(h) = p("HealthModifier").parse::<i32>() {
                        for l in list(&p("IslandLocationsToAffect")) {
                            if let Ok(id) = l.parse::<LandId>() {
                                *rules.dahan_health_bonus.entry(id).or_insert(0) += h;
                            }
                        }
                    }
                } else {
                    notes.push(format!("T{} {}: Invader health {} (per Strife: {}) not modeled", t.turn, se.source, p("HealthModifier"), p("PerStrife")));
                }
            }
            "DefendStatusEffect" => {
                // "Presence grant 1 Defense per piece", "Dahan grant 2 Defense"
                let summary = p("Summary");
                let n: u32 = summary.split_whitespace().find_map(|w| w.parse().ok()).unwrap_or(0);
                let per_piece = summary.contains("per piece");
                for l in island.lands.values_mut() {
                    let count = if summary.starts_with("Presence") { l.c.presence } else if summary.starts_with("Dahan") { l.c.dahan } else { 0 };
                    if count > 0 && l.defend == 0 {
                        l.defend += if per_piece { n * count } else { n };
                    }
                }
                notes.push(format!("T{} {}: {summary} (applied where the land had no logged Defend)", t.turn, se.source));
            }
            "IgnoreTokenStatusEffect" => {
                if list(&p("TokenTypesToIgnore")).iter().any(|x| x == "Disease") && p("DuringInvaderPhase") == "True" {
                    rules.ignore_disease = true;
                }
            }
            "PiecesCountAsTypeStatusEffect" => {
                if p("GetSourcePieceTypeString()") == "Beast" && p("GetOtherPieceTypeString()") == "Badlands" {
                    rules.beasts_as_badlands = true;
                }
            }
            "ChangeRavageParticipationStatusEffect" | "ModifyDahanCounterattackStatusEffect" => {
                notes.push(format!("T{} {}: {} not modeled", t.turn, se.source, se.kind));
            }
            _ => {}
        }
    }
}

fn kinds(v: &[PieceKind]) -> String {
    let mut m: BTreeMap<&str, u32> = BTreeMap::new();
    for k in v {
        *m.entry(match k {
            PieceKind::Explorer => "E",
            PieceKind::Town => "T",
            PieceKind::City => "C",
            PieceKind::Dahan => "D",
        })
        .or_insert(0) += 1;
    }
    if m.is_empty() { "-".into() } else { m.iter().map(|(k, n)| format!("{n}{k}")).collect::<Vec<_>>().join("") }
}

struct Ctx<'a> {
    turn: u32,
    step: &'a str,
    card: &'a str,
}

fn tally(report: &mut Report, ctx: &Ctx, land: &LandId, field: &str, sim: String, logged: String, notes: &[String]) {
    let e = report.fields.entry(field.to_string()).or_insert((0, 0));
    e.1 += 1;
    if sim == logged {
        e.0 += 1;
    } else {
        report.mismatches.push(Check { turn: ctx.turn, step: ctx.step.into(), card: ctx.card.into(), land: land.to_string(), field: field.into(), simulated: sim, logged, notes: notes.to_vec() });
    }
}

fn explain(report: &mut Report, ctx: &Ctx, land: &LandId, what: &str, source: &str) {
    let e = report.fields.entry("explained by the log".into()).or_insert((0, 0));
    e.0 += 1;
    e.1 += 1;
    report.explained.push(Check { turn: ctx.turn, step: ctx.step.into(), card: ctx.card.into(), land: land.to_string(), field: what.into(), simulated: String::new(), logged: String::new(), notes: vec![source.to_string()] });
}

fn did_nothing(o: &Outcome) -> bool {
    o.added.is_empty() && o.damage == 0 && o.blight == 0 && o.dahan_lost == 0 && o.invaders_destroyed == 0
}

/// A Power's effects and element thresholds, from the definitions: a card's
/// printed effects plus its element thresholds, or an innate's tiers.
pub fn power_effects(defs: &Definitions, power: &str) -> Option<(Vec<Effect>, Vec<(si_model::ElementSet, Vec<Effect>)>)> {
    if let Some(c) = defs.cards.get(power) {
        let thresholds = c.thresholds.iter().filter(|t| !t.elements.is_empty()).map(|t| (t.elements.clone(), t.effects.clone())).collect();
        return Some((c.effects.clone(), thresholds));
    }
    let innate = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == power)?;
    Some((vec![], innate.tiers.iter().filter(|t| !t.modifier).map(|t| (t.elements.clone(), t.effects.clone())).collect()))
}

/// The spirit's elements this turn: revealed track slots plus every card played.
pub fn elements_of(defs: &Definitions, s: &si_record::SpiritTurn) -> si_model::ElementSet {
    let mut set = defs.spirits.get(&s.spirit).map(|p| p.track_state(&s.revealed_after).elements).unwrap_or_default();
    for c in &s.cards_played {
        if let Some(card) = defs.cards.get(c) {
            set.extend(&card.elements);
        }
    }
    // a growth option's, a Power's, a wildcard slot's: what the turn gained beyond those
    for (_, els) in &s.elements_gained {
        for e in els.iter().filter_map(|e| si_model::element::Element::parse(e)) {
            set.add(e, 1);
        }
    }
    set
}

/// The gathers and pushes the record shows for one use of a Power: Dahan and Invaders the
/// Power moved into or out of its land this turn, by the land at the other end.
pub fn recorded_moves(rec: &GameRecord, turn: u32, power: &str, target: Option<&LandId>) -> Vec<crate::effects::Move> {
    let Some(tg) = target else { return Vec::new() };
    let mut out = Vec::new();
    for l in rec.lands.values() {
        if l.land == *tg {
            continue;
        }
        for x in l.touches.iter().filter(|x| x.turn == turn && x.by == power) {
            if let si_record::TouchKind::Moved { out: o, into } = x.kind {
                for (from, to, n) in [(l.land.clone(), tg.clone(), o), (tg.clone(), l.land.clone(), into)] {
                    if n == 0 {
                        continue;
                    }
                    match out.iter_mut().find(|m: &&mut crate::effects::Move| m.from == from && m.to == to) {
                        Some(m) => m.n += n,
                        None => out.push(crate::effects::Move { from, to, piece: None, n }),
                    }
                }
            }
        }
    }
    out
}

/// Spirit actions: every logged Power use in the Fast and Slow phases,
/// executed on the recorded state at the phase's start, compared per land
/// with what the log attributes to Powers there (Damage, Defend, pieces
/// pushed, Blight removed).
pub fn verify_powers(rec: &GameRecord, defs: &Definitions, rules: &Rules, report: &mut Report) {
    for t in &rec.turns {
        if t.turn == 0 {
            continue;
        }
        for (phase, before) in [(Phase::FastPower, &t.before_fast), (Phase::SlowPower, &t.before_slow)] {
            if before.is_empty() {
                continue;
            }
            let mut island = island_for(rec, defs, before);
            // simulated totals per land
            let mut sim: BTreeMap<LandId, (u32, u32, u32, u32)> = BTreeMap::new(); // damage, defend, pushed, blight removed
            let mut powers_on: BTreeMap<LandId, Vec<String>> = BTreeMap::new();
            let mut notes_on: BTreeMap<LandId, Vec<String>> = BTreeMap::new();
            for s in rec.spirits.iter().filter(|s| s.turn == t.turn) {
                let elements = elements_of(defs, s);
                for (p, power, target) in &s.powers_used {
                    if *p != phase {
                        continue;
                    }
                    let Some(target) = target else { continue };
                    let ctx = effects::Ctx { health_bonus: rules.invader_health_bonus(), elements: elements.clone(), blight_as_badlands: rules.blight_as_badlands_for(&s.spirit), rules: Some(rules.clone()), placements: placements_of(defs, t, power), picks: picks_of(defs, t, power), moves: recorded_moves(rec, t.turn, power, Some(target)), moves_exact: true, branches: branches_of(defs, t, power), presence_with_dahan: rules.presence_with_dahan_for(&s.spirit), kills: None, site_beast: rules.site_beast_for(&s.spirit), beasts_far: rules.beasts_far_for(&s.spirit) };
                    let Some((base, thresholds)) = power_effects(defs, power) else {
                        notes_on.entry(target.clone()).or_default().push(format!("{power}: no definition"));
                        continue;
                    };
                    // Shroud's Mists Shift and Flow: the presence the record gathered toward the land
                    if rules.gathers_presence_on_targeting(&s.spirit) {
                        if let Some((_, into)) = ctx.placements.iter().find(|(r, _)| r == "Gather") {
                            effects::gather_presence(&mut island, into);
                        }
                    }
                    let a = effects::apply(&base, &thresholds, &mut island, target, &ctx);
                    let e = sim.entry(target.clone()).or_insert((0, 0, 0, 0));
                    e.0 += a.damage;
                    e.1 += a.defend;
                    e.2 += a.pushed;
                    e.3 += a.blight_removed;
                    powers_on.entry(target.clone()).or_default().push(power.clone());
                    notes_on.entry(target.clone()).or_default().extend(a.notes);
                }
            }
            for (land, (dmg, def, pushed, removed)) in &sim {
                let powers = powers_on.get(land).cloned().unwrap_or_default();
                let (mut l_dmg, mut l_def, mut l_out, mut l_rec) = (0u32, 0u32, 0u32, 0u32);
                if let Some(ledger) = rec.lands.get(land) {
                    for touch in ledger.touches.iter().filter(|x| x.turn == t.turn && x.phase == phase) {
                        match touch.kind {
                            si_record::TouchKind::Damage(n) if powers.contains(&touch.by) => l_dmg += n,
                            si_record::TouchKind::Defend(n) => l_def += n,
                            si_record::TouchKind::Moved { out, .. } if powers.contains(&touch.by) => l_out += out,
                            si_record::TouchKind::Recover(n) => l_rec += n,
                            _ => {}
                        }
                    }
                }
                let step = if phase == Phase::FastPower { "Fast" } else { "Slow" };
                let card = powers.join("+");
                let ctx = Ctx { turn: t.turn, step, card: &card };
                let notes = notes_on.get(land).cloned().unwrap_or_default();
                let choice = notes.iter().any(|n| n.starts_with("'or'") || n.starts_with("push: which") || n.starts_with("gather:"));
                let suffix = if choice { " (player's choice)" } else { "" };
                tally(report, &ctx, land, &format!("power damage{suffix}"), dmg.to_string(), l_dmg.to_string(), &notes);
                if *def > 0 || l_def > 0 {
                    tally(report, &ctx, land, &format!("power defend{suffix}"), def.to_string(), l_def.to_string(), &notes);
                }
                if *pushed > 0 || l_out > 0 {
                    tally(report, &ctx, land, &format!("pieces pushed{suffix}"), pushed.to_string(), l_out.to_string(), &notes);
                }
                if *removed > 0 || l_rec > 0 {
                    tally(report, &ctx, land, "blight removed", removed.to_string(), l_rec.to_string(), &notes);
                }
            }
        }
    }
}

/// Every turn with a recorded pre-Invader state, simulated step by step
/// (High Immigration, Ravage, Build, Explore) and compared land by land.
/// After each step the simulated lands are reset to the logged state, so one
/// gap does not cascade into the next step's comparison.
pub fn verify(rec: &GameRecord, defs: &Definitions, level: Option<u8>) -> Report {
    let mut report = Report::default();
    let base_rules = Rules { scenario: rec.meta.scenario.clone(), adversary: rec.meta.adversary.clone(), level: level.unwrap_or(0), spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), ..Default::default() };
    if rec.meta.adversary.is_some() && level.is_none() {
        report.notes.push("adversary level unknown: level rules not applied (pass --level)".into());
    }
    if base_rules.adversary.as_deref() == Some("HabsburgMiningExpedition") {
        report.notes.push("Habsburg Mining's Mining lands are not modeled".into());
    }
    verify_powers(rec, defs, &base_rules, &mut report);
    for t in &rec.turns {
        if t.turn == 0 || t.before_invaders.is_empty() {
            continue;
        }
        report.turns += 1;
        let mut rules = base_rules.clone();
        let mut island = island_at(rec, defs, t, "before_invaders");
        for (id, d) in &t.defend {
            if let Some(l) = island.lands.get_mut(id) {
                l.defend = *d;
            }
        }
        // Skips and cancels come from the engine's own status effects and
        // CancelAction lines (the oracle), not from reading card text.
        apply_status_effects(&mut island, &mut rules, t, &mut report.notes);
        rules.chokes = chokes_of(rec, t);
        let logged_cascades: u32 = rec.lands.values().flat_map(|l| l.blight.iter()).filter(|(turn, phase, _, cascade)| *turn == t.turn && *phase == Phase::Ravage && *cascade).map(|(_, _, n, _)| n).sum();
        let mut sim_cascades = 0u32;
        let steps: Vec<(&str, &Vec<String>, InvaderStep)> = vec![("HighImmigration", &t.high_immigration, InvaderStep::Build), ("Ravage", &t.ravage, InvaderStep::Ravage), ("Build", &t.build, InvaderStep::Build), ("Explore", &t.explore, InvaderStep::Explore)];
        for (name, cards, step) in steps {
            for card_id in cards {
                let Some(card) = defs.invader_cards.get(card_id) else {
                    report.notes.push(format!("T{} {name}: unknown Invader card {card_id}", t.turn));
                    continue;
                };
                let ctx = Ctx { turn: t.turn, step: name, card: card_id };
                let outcomes = match step {
                    InvaderStep::Ravage => invaders::ravage(&mut island, card, &rules),
                    InvaderStep::Build => invaders::build(&mut island, card, &rules),
                    InvaderStep::Explore => invaders::explore(&mut island, card, &rules),
                };
                let visits: Vec<(LandId, &InvaderVisit)> = rec.lands.values().flat_map(|l| l.visits.iter().map(move |v| (l.land.clone(), v))).filter(|(_, v)| v.turn == t.turn && v.step == Some(step) && v.card.as_deref() == Some(card_id.as_str())).collect();
                let whole_card_cancel = t.canceled.iter().find(|c| c.step == step && c.land.is_none()).map(|c| c.source.clone());
                let canceled_in = |land: &LandId| t.canceled.iter().find(|c| c.step == step && c.land.as_ref() == Some(land)).map(|c| c.source.clone());
                for o in &outcomes {
                    if o.land.board == "?" {
                        report.notes.push(format!("T{} {name} {card_id}: {}", t.turn, o.notes.join("; ")));
                        continue;
                    }
                    if step == InvaderStep::Ravage {
                        sim_cascades += o.cascade;
                    }
                    let logged = visits.iter().find(|(l, _)| *l == o.land).map(|(_, v)| *v);
                    let cancel = canceled_in(&o.land).or_else(|| whole_card_cancel.clone());
                    match (logged, cancel) {
                        (None, Some(_)) if did_nothing(o) => {
                            // the simulation predicted the cancel (Disease, a skip) and the log agrees
                            tally(&mut report, &ctx, &o.land, "land acted on", "no".into(), "no".into(), &o.notes);
                        }
                        (None, Some(source)) => explain(&mut report, &ctx, &o.land, &format!("{name} canceled"), &format!("canceled by {source}")),
                        (None, None) if did_nothing(o) => tally(&mut report, &ctx, &o.land, "land acted on", "no".into(), "no".into(), &o.notes),
                        (None, None) => tally(&mut report, &ctx, &o.land, "land acted on", "yes".into(), "no".into(), &o.notes),
                        (Some(v), _) => {
                            if did_nothing(o) && !o.alternative.is_empty() && v.added == o.alternative {
                                tally(&mut report, &ctx, &o.land, "pieces added", kinds(&v.added), kinds(&v.added), &["Vengeance let the Build happen through Disease".into()]);
                                continue;
                            }
                            tally(&mut report, &ctx, &o.land, "land acted on", "yes".into(), "yes".into(), &[]);
                            if step != InvaderStep::Ravage {
                                tally(&mut report, &ctx, &o.land, "pieces added", kinds(&o.added), kinds(&v.added), &o.notes);
                            } else {
                                tally(&mut report, &ctx, &o.land, "blight", o.blight.to_string(), v.blight.to_string(), &o.notes);
                                tally(&mut report, &ctx, &o.land, "dahan lost", o.dahan_lost.to_string(), v.dahan_lost.to_string(), &o.notes);
                                let field = if o.notes.iter().any(|n| n.starts_with("counterattack distribution")) { "invaders destroyed (player's distribution)" } else { "invaders destroyed" };
                                tally(&mut report, &ctx, &o.land, field, o.invaders_destroyed.to_string(), v.invaders_destroyed.to_string(), &o.notes);
                            }
                        }
                    }
                }
                for (l, v) in &visits {
                    if !outcomes.iter().any(|o| o.land == *l) {
                        tally(&mut report, &ctx, l, "land acted on", "no".into(), format!("yes ({} added, {} blight)", kinds(&v.added), v.blight), &[]);
                    }
                }
                // resync the touched lands to the log before the next step
                for (l, v) in &visits {
                    if let Some(land) = island.lands.get_mut(l) {
                        land.c = v.after;
                    }
                }
            }
        }
        if !t.ravage.is_empty() {
            let ctx = Ctx { turn: t.turn, step: "Ravage", card: "(turn)" };
            tally(&mut report, &ctx, &LandId::new("-", 0), "cascades (turn total)", sim_cascades.to_string(), logged_cascades.to_string(), &[]);
        }
    }
    report
}
