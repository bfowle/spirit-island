//! Game rules used by the companion's stats.
//!
//! Every rule names its evidence. "Engine" means the Steam engine's own
//! code or definitions (see `research_notes/…/engine_state_model.md` Q3);
//! `Q#` refers to `data/references/rules/open-questions.md`, where Brett
//! confirmed the rule against the physical copy or delegated it to the
//! rulebooks and FAQ (2026-10-19); the rules register
//! (`data/references/rules/register.md`) records each rule's review status.

use crate::public::{PublicInvaderCard, PublicLand, PublicSnapshot, PublicStatusEffect, StatusParam};
use std::collections::{BTreeMap, BTreeSet};

/// Land health before modifiers. Engine: `Land::get_MaxHealth` returns 2.
pub const LAND_HEALTH: i64 = 2;
/// Damage each invader deals during Ravage. Engine: `IslandPiece::get_DamageToDeal`
/// returns the piece's normal max health (Explorer 1, Town 2, City 3).
pub fn invader_damage(kind: &str) -> i64 {
    match kind {
        "Explorer" => 1,
        "Town" => 2,
        "City" => 3,
        _ => 0,
    }
}
/// Each surviving Dahan deals 2 damage when it counterattacks (engine log:
/// `DamageAmount=4` from two Dahan; Dahan health 2).
pub const DAHAN_COUNTERATTACK: i64 = 2;
/// Fear for destroying a Town / City. Engine constants
/// `FearGeneratedFromDestroyingTowns=1`, `…Cities=2`; Explorers give none.
pub fn destroy_fear(kind: &str) -> i64 {
    match kind {
        "Town" => 1,
        "City" => 2,
        _ => 0,
    }
}

pub fn is_invader(kind: &str) -> bool {
    matches!(kind, "Explorer" | "Town" | "City")
}

pub fn has_invaders(l: &PublicLand) -> bool {
    l.explorers + l.towns + l.cities > 0
}

/// Does an invader card's land type match this land? Coastal cards match
/// coastal lands; Oceans never match. Engine: `LandMatchesInvaderCard`;
/// card land types from the engine's InvaderCardList.
pub fn card_matches(card: &PublicInvaderCard, land: &PublicLand) -> bool {
    if land.is_ocean {
        return false;
    }
    card.terrains.iter().any(|t| t == &land.terrain || (t == "Coastal" && land.coastal))
}

/// Neighbors of each land.
pub fn neighbors(s: &PublicSnapshot) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut out: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (a, b) in &s.adjacency {
        out.entry(a.as_str()).or_default().insert(b.as_str());
        out.entry(b.as_str()).or_default().insert(a.as_str());
    }
    out
}

/// Explore sources: Oceans and lands with a Town or City. Explorers are not
/// a source. Engine: `GameController::GetLandsToExploreFrom` (base predicate:
/// not isolated AND (Ocean OR has Town/City)); Wiki *Sequence of Play* 3c,
/// rev 12144; `data/references/rules/explore-phase.md`; confirmed by Brett (Q3).
pub fn is_explore_source(l: &PublicLand) -> bool {
    l.is_ocean || l.towns + l.cities > 0
}

/// A land can be explored if it is a source or is adjacent to one (explore
/// range 1). One Explorer per land however many sources. Engine:
/// `IsSourceLandInExploreRange`, `GetExploreRange = 1 + AskForExploreRange`.
pub fn explore_sources_for<'a>(land: &PublicLand, lands: &'a [PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> Vec<&'a str> {
    let mut out = Vec::new();
    if let Some(me) = lands.iter().find(|l| l.id == land.id)
        && is_explore_source(me) && !me.is_ocean {
            out.push(me.id.as_str());
        }
    if let Some(ns) = nbrs.get(land.id.as_str()) {
        for n in ns {
            if let Some(nl) = lands.iter().find(|l| l.id == *n)
                && is_explore_source(nl) {
                    out.push(nl.id.as_str());
                }
        }
    }
    out
}

/// What Build adds. Engine `GetIslandPieceTypeToBuildInLand`: if the land has
/// more Towns than Cities, a City; otherwise a Town. Confirmed by Brett (Q2).
pub fn build_piece(l: &PublicLand) -> &'static str {
    if l.towns > l.cities { "City" } else { "Town" }
}

/// Build happens only in matching lands that have Invaders (Wiki *Sequence of
/// Play* 3b; confirmed by Brett, Q2; England L1 also builds in empty lands).
pub fn builds_in(card: &PublicInvaderCard, l: &PublicLand) -> bool {
    card_matches(card, l) && has_invaders(l)
}

/// Ravage happens only in matching lands that have Invaders.
pub fn ravages_in(card: &PublicInvaderCard, l: &PublicLand) -> bool {
    card_matches(card, l) && has_invaders(l)
}

/// The deterministic part of one Ravage, as signed terms.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RavageLedger {
    /// Damage by invader kind before Defend, e.g. [("City", 3), ("Explorer", 2)].
    pub damage_terms: Vec<(String, i64)>,
    pub invader_damage: i64,
    pub defend: i64,
    pub damage_after_defend: i64,
    pub land_health: i64,
    pub adds_blight: bool,
    /// The land already has blight, so the added blight cascades (Q5).
    pub cascades: bool,
    pub dahan_before: u32,
    pub dahan_destroyed: u32,
    pub counterattack_damage: i64,
    /// Best case for the counterattack: the most invaders it can destroy
    /// (by Fear, then count), e.g. ["Town", "Explorer"].
    pub counterattack_can_destroy: Vec<String>,
    pub counterattack_fear: i64,
    /// The counterattack destroys every Invader whatever the damage
    /// assignment (its damage ≥ their total health).
    pub counterattack_clears: bool,
    pub presence_at_risk: BTreeMap<String, u32>,
    /// Rule effects the ledger applied, with their evidence.
    pub applied: Vec<String>,
    /// An optional Skip that can cancel this Ravage (source, uses left).
    pub skip_available: Option<String>,
    /// Tokens or effects present that the base ledger does not model.
    pub unmodeled: Vec<String>,
}

/// Base-rules Ravage ledger for one land.
///
/// - Invaders deal their damage to the land and to the Dahan; Defend
///   reduces both (engine: `GetInvaderDamageForLand` subtracts the defense
///   value; Test 1 Turn 3 log, Jungle E3 with Defend 2). Confirmed by Brett (Q4).
/// - One blight if the damage after Defend reaches the land's health
///   (engine `DealDamageAction.AfterAction`).
/// - Damage to Dahan destroys as many as it can; excess is not carried over
///   (Test 1 log: `Dahan1 => 2, JungleC1 => 3`). Q4.
/// - Surviving Dahan counterattack after the invaders by default (engine
///   `DahanCounterattackTiming.AfterInvaders`).
/// - An Invader with Strife deals no Ravage damage and loses 1 Strife
///   (engine, Test 1 Turn 4 log: `Strife1 ( City 1's Token Location =>
///   OutOfGame)` then `DamageAmountsBySource=[Explorer6-1]` in Sands C2;
///   JE rulebook, Q25).
/// - Each Invader's damage after its own adjustments is floored at 0; per-land
///   bonuses apply only when that total is at least 1; Defend comes last
///   (engine `RavageAction` coroutine; NQ-L).
pub fn ravage_ledger(l: &PublicLand, effects: &[PublicStatusEffect]) -> RavageLedger {
    let mut applied = Vec::new();
    let mut unmodeled = Vec::new();
    let mut dealing = Vec::new();
    for p in l.pieces.iter().filter(|p| is_invader(&p.kind)) {
        if p.strife > 0 {
            applied.push(format!("{} with Strife deals no damage (loses 1 Strife)", p.kind));
            continue;
        }
        dealing.push(p);
    }
    let mut per_piece: Vec<i64> = dealing.iter().map(|p| invader_damage(&p.kind)).collect();
    let mut land_terms: Vec<(String, i64)> = Vec::new();
    let mut defend = l.defend;
    let mut skip_available = None;
    let mut no_cascade = false;
    for e in effects {
        let src = e.source.clone().unwrap_or_else(|| e.kind.clone());
        match e.kind.as_str() {
            "Defend" => match defend_from(e, l) {
                Some(d) if d != 0 => {
                    defend += d;
                    applied.push(format!("Defend {d} ({src})"));
                }
                Some(_) => {}
                None => unmodeled.push(format!("Defend from {src}: scope not modeled")),
            },
            "AdjustRavageDamage" => match ravage_adjustment(e, l) {
                Some(RavageAdjust::PerInvader { kinds, amount, per_damage_taken }) => {
                    let mut total = 0;
                    for (p, d) in dealing.iter().zip(per_piece.iter_mut()) {
                        if kinds.contains(&p.kind) {
                            let adj = if per_damage_taken { amount * p.damage } else { amount };
                            *d += adj;
                            total += adj;
                        }
                    }
                    if total != 0 {
                        let per = if per_damage_taken { " per Damage each Invader has taken" } else { " per Invader" };
                        applied.push(format!("{amount:+}{per}: {total:+} in all ({src})"));
                    }
                }
                Some(RavageAdjust::PerLand(d)) if d != 0 => land_terms.push((src, d)),
                Some(RavageAdjust::PerLand(_)) => {}
                None => unmodeled.push(format!("Ravage damage change from {src}: conditions not modeled")),
            },
            "SkipInvaderActions" if targets_land(e, l, "islandLocationToSkip") && skips_step(e, "Ravage") => {
                let uses = int(e, "NumberOfUses").map(|u| format!(", {u} uses")).unwrap_or_default();
                let optional = if flag(e, "IsOptional") { "optional" } else { "automatic" };
                skip_available = Some(format!("{src} ({optional}{uses})"));
            }
            "PreventBlightCascade" if targets_land(e, l, "AffectedLands") => {
                no_cascade = true;
                applied.push(format!("no cascade ({src})"));
            }
            "AdjustHealth" => {
                if let Some(h) = int(e, "HealthModifier") {
                    let who = list(e, "PieceTypesToAffect").join("/");
                    let per = if flag(e, "PerStrife") { " per Strife" } else { "" };
                    applied.push(format!("{who} health {h:+}{per} ({src}), already on the pieces"));
                }
            }
            _ => {}
        }
    }
    for (token, n) in &l.tokens {
        if *n > 0 && matches!(token.as_str(), "Badlands" | "Beast" | "Wilds" | "Disease" | "Vitality") {
            unmodeled.push(format!("{n} {token}"));
        }
    }
    let mut terms: BTreeMap<String, i64> = BTreeMap::new();
    for (p, d) in dealing.iter().zip(&per_piece) {
        *terms.entry(p.kind.clone()).or_default() += (*d).max(0);
    }
    let own: i64 = terms.values().sum();
    if own > 0 {
        for (src, d) in land_terms {
            applied.push(format!("{d:+} damage ({src})"));
            terms.insert(src, d);
        }
    } else if !land_terms.is_empty() {
        applied.push("Per-land damage bonuses don't apply: the Invaders deal 0 themselves (JE p.7)".into());
    }
    let invader_damage: i64 = terms.values().sum::<i64>().max(0);
    let after = (invader_damage - defend).max(0);
    let land_health = (LAND_HEALTH + l.health_modifier).max(1) - l.damage_taken;
    let adds_blight = invader_damage > 0 && after >= land_health;

    let mut dahan: Vec<i64> = l.pieces.iter().filter(|p| p.kind == "Dahan").map(|p| (p.max_health - p.damage).max(1)).collect();
    dahan.sort();
    let mut left = after;
    let mut destroyed = 0u32;
    for h in &dahan {
        if left >= *h {
            left -= h;
            destroyed += 1;
        } else {
            break;
        }
    }
    let survivors = dahan.len() as u32 - destroyed;
    let counter = survivors as i64 * DAHAN_COUNTERATTACK;
    let (can_destroy, fear) = best_destroy(l, counter);
    let invader_health: i64 = l.pieces.iter().filter(|p| is_invader(&p.kind)).map(|p| (p.max_health - p.damage).max(1)).sum();

    RavageLedger {
        damage_terms: terms.into_iter().collect(),
        invader_damage,
        defend,
        damage_after_defend: after,
        land_health,
        adds_blight,
        cascades: adds_blight && l.blight > 0 && !no_cascade,
        dahan_before: dahan.len() as u32,
        dahan_destroyed: destroyed,
        counterattack_damage: counter,
        counterattack_can_destroy: can_destroy,
        counterattack_fear: fear,
        counterattack_clears: counter > 0 && invader_health > 0 && counter >= invader_health,
        presence_at_risk: if adds_blight { l.presence.keys().map(|k| (k.clone(), 1)).collect() } else { BTreeMap::new() },
        applied,
        skip_available,
        unmodeled,
    }
}

fn int(e: &PublicStatusEffect, k: &str) -> Option<i64> {
    match e.params.get(k)? {
        StatusParam::Int(i) => Some(*i),
        _ => None,
    }
}

fn flag(e: &PublicStatusEffect, k: &str) -> bool {
    matches!(e.params.get(k), Some(StatusParam::Bool(true)))
}

fn list<'a>(e: &'a PublicStatusEffect, k: &str) -> &'a [String] {
    match e.params.get(k) {
        Some(StatusParam::List(l)) => l,
        _ => &[],
    }
}

/// An effect with no land list applies everywhere; with one, only there.
fn targets_land(e: &PublicStatusEffect, l: &PublicLand, key: &str) -> bool {
    let lands = list(e, key);
    lands.is_empty() || lands.contains(&l.id)
}

/// `statesToSkip` empty means the next Invader action of any kind.
fn skips_step(e: &PublicStatusEffect, step: &str) -> bool {
    let states = list(e, "statesToSkip");
    states.is_empty() || states.iter().any(|s| s == step)
}

/// Defend granted by a `DefendStatusEffect` in this land. Engine members:
/// `DefendAmount`, `DefendPerPiece` + `IslandPieceTypes`, `AffectsAllLands`.
/// Test 1 Turn 5 (Years of Little Rain, "Dahan grant 1 Defense per piece"):
/// Mountain C7 with 2 Dahan took 1 of 3 damage. `None` = scope not modeled.
fn defend_from(e: &PublicStatusEffect, l: &PublicLand) -> Option<i64> {
    let scoped = ["WithinRange", "WithinRangeOfPieceCount", "TokenTypes", "SpiritsWithPiecesThatDefend", "AdjacentOnlyDefendPerPiece", "RequireAllPieceTypes"];
    if scoped.iter().any(|k| e.params.contains_key(*k)) {
        return None;
    }
    let per_piece = int(e, "DefendPerPiece").unwrap_or(0);
    let amount = int(e, "DefendAmount").unwrap_or(0);
    if per_piece != 0 {
        let kinds = list(e, "IslandPieceTypes");
        let n = l.pieces.iter().filter(|p| kinds.contains(&p.kind)).count() as i64;
        return Some(per_piece * n + amount);
    }
    if flag(e, "AffectsAllLands") {
        return Some(amount);
    }
    None
}

/// An `AdjustRavageDamageStatusEffect`, split the way the engine's
/// `RavageAction` applies it (engine read 2026-10-19; open-questions NQ-L,
/// Q32). With `PerInvaderOfType` it adjusts each matching Invader's own
/// damage (times the Damage that Invader has taken when `PerDamageTaken`),
/// and each Invader's damage is floored at 0. Without it, it is a per-land
/// bonus, added only when the Invaders' own damage is at least 1 (JE p.7).
enum RavageAdjust {
    PerInvader { kinds: Vec<String>, amount: i64, per_damage_taken: bool },
    PerLand(i64),
}

/// Engine members: `DamageAdjustment`, `PerInvaderOfType`, `PerDamageTaken`,
/// `AffectedLands`, plus about 15 conditions; any condition present makes it
/// `None` (not modeled). Distracted by Local Troubles (Terror Level II/III)
/// is -1 per Damage each Invader has taken (Q32).
fn ravage_adjustment(e: &PublicStatusEffect, l: &PublicLand) -> Option<RavageAdjust> {
    let conditions = [
        "IfLandHasPresence", "IfLandHasMinimumInvaders", "IfLandHasTownOrCity", "IfLandHasNoStrife", "AffectedBoard",
        "IfLandHasDahan", "OnlyDamageToLand", "OnlyDamageToDahan", "OnlyLowestNumberedMatchingLand",
        "PerPresenceOfSpiritInLand", "OnlyDamageFromDahan", "AddInvaderStageAsDamage", "IfLandHasBeast",
        "NumberOfTimesPerBoard", "CheckConditionFromSource", "NormalRavageOnly",
    ];
    if conditions.iter().any(|k| e.params.contains_key(*k)) {
        return None;
    }
    if !targets_land(e, l, "AffectedLands") {
        return Some(RavageAdjust::PerLand(0));
    }
    let amount = int(e, "DamageAdjustment")?;
    let per = list(e, "PerInvaderOfType");
    if per.is_empty() {
        return Some(RavageAdjust::PerLand(amount));
    }
    Some(RavageAdjust::PerInvader { kinds: per.to_vec(), amount, per_damage_taken: flag(e, "PerDamageTaken") })
}

/// Does a Wilds token stop an Explore here? Engine, Test 1 Turn 2:
/// `Wilds2 ( Sands C6 => OutOfGame)` then `CancelAction: ExploreAction …
/// Sands C6` (the Explore is cancelled and 1 Wilds removed), unless an
/// `IgnoreToken` effect ignores Wilds.
pub fn wilds_stop_explore(l: &PublicLand, effects: &[PublicStatusEffect]) -> bool {
    l.tokens.get("Wilds").copied().unwrap_or(0) > 0
        && !effects.iter().any(|e| e.kind == "IgnoreToken" && list(e, "TokenTypesToIgnore").iter().any(|t| t == "Wilds"))
}

/// Lands Isolated this turn: every land listed by an `IsolateStatusEffect`
/// (engine members `IslandLocationsToIsolate` / `LandsToIsolate`). An Isolated
/// land is not explored and is not an Explore source; Isolating an Ocean stops
/// it being a source but its lands stay Coastal; Isolate does not affect
/// cascades; it lasts one turn (JE rulebook, Isolate; open-questions Q3).
pub fn isolated_lands(effects: &[PublicStatusEffect]) -> BTreeSet<String> {
    effects
        .iter()
        .filter(|e| e.kind.contains("Isolate"))
        .flat_map(|e| e.params.iter().filter(|(k, _)| k.to_ascii_lowercase().contains("isolate")))
        .filter_map(|(_, v)| match v {
            StatusParam::List(l) => Some(l.iter().cloned()),
            _ => None,
        })
        .flatten()
        .collect()
}

/// Build outcome in a land, after tokens and effects. Engine, Test 1 Turn 5:
/// `Disease1 ( Jungle C1 => OutOfGame)` then `CancelAction: BuildAction`
/// (a Disease stops the Build and is removed), unless an `IgnoreToken`
/// effect ignores Disease during the Invader Phase.
pub fn build_outcome(l: &PublicLand, effects: &[PublicStatusEffect]) -> (Option<&'static str>, Vec<String>) {
    let mut notes = Vec::new();
    for e in effects {
        if e.kind == "SkipInvaderActions" && targets_land(e, l, "islandLocationToSkip") && skips_step(e, "Build") {
            notes.push(format!("Skip available: {}", e.source.clone().unwrap_or_default()));
        }
    }
    let disease = l.tokens.get("Disease").copied().unwrap_or(0);
    let ignored = effects.iter().any(|e| e.kind == "IgnoreToken" && list(e, "TokenTypesToIgnore").iter().any(|t| t == "Disease"));
    if disease > 0 && !ignored {
        notes.push("Disease stops this Build (1 Disease removed)".into());
        return (None, notes);
    }
    if disease > 0 {
        notes.push("Disease ignored this Invader Phase".into());
    }
    (Some(build_piece(l)), notes)
}

/// The set of invaders `damage` can destroy that gives the most Fear, then
/// the most pieces. Exhaustive over the land's invaders (a handful).
pub fn best_destroy(l: &PublicLand, damage: i64) -> (Vec<String>, i64) {
    let inv: Vec<(&str, i64)> = l
        .pieces
        .iter()
        .filter(|p| is_invader(&p.kind))
        .map(|p| (p.kind.as_str(), (p.max_health - p.damage).max(1)))
        .collect();
    if damage <= 0 || inv.is_empty() {
        return (Vec::new(), 0);
    }
    let n = inv.len().min(16);
    let mut best: (i64, usize, u32) = (0, 0, 0);
    for mask in 1u32..(1 << n) {
        let (mut hp, mut fear, mut count) = (0, 0, 0);
        for (i, (kind, h)) in inv.iter().take(n).enumerate() {
            if mask & (1 << i) != 0 {
                hp += h;
                fear += destroy_fear(kind);
                count += 1;
            }
        }
        if hp <= damage && (fear, count) > (best.0, best.1) {
            best = (fear, count, mask);
        }
    }
    let mut kinds: Vec<String> = (0..n).filter(|i| best.2 & (1 << i) != 0).map(|i| inv[i].0.to_string()).collect();
    kinds.sort_by_key(|k| std::cmp::Reverse(invader_damage(k)));
    (kinds, best.0)
}

/// Victory condition by Terror Level (base rulebook, Fear: "Terror Level
/// I: no Invaders; II: no Towns or Cities; III: no Cities"). Confirmed by Brett (Q6).
pub fn terror_victory_blockers(level: u8, s: &PublicSnapshot) -> (String, u32) {
    let sum = |f: fn(&PublicLand) -> u32| s.lands.iter().map(f).sum::<u32>();
    match level {
        1 => ("no Invaders on the island".into(), sum(|l| l.explorers + l.towns + l.cities)),
        2 => ("no Towns or Cities".into(), sum(|l| l.towns + l.cities)),
        _ => ("no Cities".into(), sum(|l| l.cities)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::PublicPiece;

    fn land(pieces: &[(&str, i64)], blight: u32, defend: i64) -> PublicLand {
        let mut l = PublicLand { id: "L".into(), terrain: "Jungle".into(), blight, defend, ..Default::default() };
        for (kind, hp) in pieces {
            l.pieces.push(PublicPiece { kind: kind.to_string(), max_health: *hp, damage: 0, strife: 0 });
            match *kind {
                "Explorer" => l.explorers += 1,
                "Town" => l.towns += 1,
                "City" => l.cities += 1,
                "Dahan" => l.dahan += 1,
                _ => {}
            }
        }
        l
    }

    #[test]
    fn ravage_town_explorer_vs_one_dahan() {
        // Town 2 + Explorer 1 = 3 damage: blight, the Dahan dies, no counterattack.
        let r = ravage_ledger(&land(&[("Town", 2), ("Explorer", 1), ("Dahan", 2)], 0, 0), &[]);
        assert_eq!(r.invader_damage, 3);
        assert!(r.adds_blight && !r.cascades);
        assert_eq!((r.dahan_destroyed, r.counterattack_damage), (1, 0));
    }

    #[test]
    fn defend_two_saves_the_land_and_dahan_counterattack() {
        // Test 1 Turn 3, Jungle E3 shape: 3 damage, Defend 2 → 1 damage, no blight;
        // 2 Dahan survive and counterattack for 4, enough for the Town and the Explorer.
        let r = ravage_ledger(&land(&[("Town", 2), ("Explorer", 1), ("Dahan", 2), ("Dahan", 2)], 1, 2), &[]);
        assert_eq!(r.damage_after_defend, 1);
        assert!(!r.adds_blight);
        assert_eq!(r.dahan_destroyed, 0);
        assert_eq!(r.counterattack_damage, 4);
        assert_eq!(r.counterattack_can_destroy, vec!["Town", "Explorer"]);
        assert_eq!(r.counterattack_fear, 1);
    }

    #[test]
    fn strife_city_deals_no_damage() {
        // Test 1 Turn 4, Sands C2: City with Strife + Explorer → only 1 damage, no blight.
        let mut l = land(&[("City", 3), ("Explorer", 1)], 0, 0);
        l.pieces[0].strife = 1;
        let r = ravage_ledger(&l, &[]);
        assert_eq!(r.invader_damage, 1);
        assert!(!r.adds_blight);
        assert_eq!(r.applied.len(), 1);
    }

    fn effect(kind: &str, params: &[(&str, StatusParam)]) -> PublicStatusEffect {
        PublicStatusEffect {
            kind: kind.into(),
            source: Some("Test".into()),
            params: params.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
        }
    }

    #[test]
    fn defend_per_dahan_saves_the_land() {
        // Test 1 Turn 5, Mountain C7: Town + Explorer = 3, 2 Dahan × Defend 1 → 1 damage.
        let l = land(&[("Town", 2), ("Explorer", 1), ("Dahan", 2), ("Dahan", 2)], 1, 0);
        let e = effect("Defend", &[("DefendPerPiece", StatusParam::Int(1)), ("IslandPieceTypes", StatusParam::List(vec!["Dahan".into()]))]);
        let r = ravage_ledger(&l, &[e]);
        assert_eq!((r.defend, r.damage_after_defend, r.adds_blight), (2, 1, false));
    }

    #[test]
    fn minus_one_per_invader() {
        // Two Explorers, −1 per Invader → 0 damage.
        let l = land(&[("Explorer", 1), ("Explorer", 1)], 0, 0);
        let e = effect(
            "AdjustRavageDamage",
            &[("DamageAdjustment", StatusParam::Int(-1)), ("PerInvaderOfType", StatusParam::List(vec!["Explorer".into(), "Town".into(), "City".into()]))],
        );
        let r = ravage_ledger(&l, &[e]);
        assert_eq!((r.invader_damage, r.adds_blight), (0, false));
    }

    #[test]
    fn minus_one_per_damage_taken_floors_each_invader() {
        // Distracted by Local Troubles (Q32): -1 per Damage each Invader has
        // taken, floored at 0 per Invader. Town (2 dmg, took 1) deals 1; City
        // (3, took 0) deals 3; Explorer (1, took 1 of 2 health) deals 0.
        let mut l = land(&[("Town", 3), ("City", 4), ("Explorer", 2)], 0, 0);
        l.pieces[0].damage = 1;
        l.pieces[2].damage = 1;
        let kinds = StatusParam::List(vec!["Explorer".into(), "Town".into(), "City".into()]);
        let e = effect("AdjustRavageDamage", &[("DamageAdjustment", StatusParam::Int(-1)), ("PerInvaderOfType", kinds), ("PerDamageTaken", StatusParam::Bool(true))]);
        let r = ravage_ledger(&l, &[e]);
        assert_eq!(r.invader_damage, 4);
    }

    #[test]
    fn per_land_bonus_needs_invader_damage() {
        // NQ-L: +2 per land does nothing when every Invader has Strife.
        let mut l = land(&[("Town", 2)], 0, 0);
        l.pieces[0].strife = 1;
        let e = effect("AdjustRavageDamage", &[("DamageAdjustment", StatusParam::Int(2))]);
        assert_eq!(ravage_ledger(&l, &[e.clone()]).invader_damage, 0);
        l.pieces[0].strife = 0;
        assert_eq!(ravage_ledger(&l, &[e]).invader_damage, 4);
    }

    #[test]
    fn disease_stops_build() {
        let mut l = land(&[("Explorer", 1)], 0, 0);
        l.tokens.insert("Disease".into(), 1);
        assert_eq!(build_outcome(&l, &[]).0, None);
        let ignore = effect("IgnoreToken", &[("TokenTypesToIgnore", StatusParam::List(vec!["Disease".into()]))]);
        assert_eq!(build_outcome(&l, &[ignore]).0, Some("Town"));
    }

    #[test]
    fn blight_on_blight_cascades() {
        let r = ravage_ledger(&land(&[("City", 3)], 1, 0), &[]);
        assert!(r.adds_blight && r.cascades);
    }

    #[test]
    fn build_town_then_city() {
        assert_eq!(build_piece(&land(&[("Explorer", 1)], 0, 0)), "Town");
        assert_eq!(build_piece(&land(&[("Town", 2)], 0, 0)), "City");
        assert_eq!(build_piece(&land(&[("Town", 2), ("City", 3)], 0, 0)), "Town");
    }

    #[test]
    fn isolate_lists_its_lands() {
        let iso = effect("Isolate", &[("IslandLocationsToIsolate", StatusParam::List(vec!["JungleC4".into(), "OceanC0".into()]))]);
        let defend = effect("Defend", &[("AffectedLands", StatusParam::List(vec!["SandsC2".into()]))]);
        let got = isolated_lands(&[iso, defend]);
        assert_eq!(got.into_iter().collect::<Vec<_>>(), vec!["JungleC4".to_string(), "OceanC0".to_string()]);
    }
}
