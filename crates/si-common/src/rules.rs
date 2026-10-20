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

/// `Engine.TargetType`, from the DLL's enum constants (static metadata read,
/// 2026-10-19). Status effects use it to restrict lands, e.g. Overseas Trade
/// Seems Safer's Build cancel targets `Coastal`.
pub const TARGET_TYPES: &[&str] = &[
    "AnyLand", "Coastal", "Inland", "LandWithInvaders", "LandWithNoInvaders", "Jungle", "Mountain", "Ocean", "Sands",
    "Wetland", "AnySpirit", "AnotherSpirit", "LandWithDahan", "LandWithBlight", "LandWithNoBlight", "JungleWetland",
    "MountainJungle", "LandWithCity", "MountainWetland", "LandWithBlightAndInvaders", "You", "JungleSands", "TwoLands",
    "CoastalOrWetland", "SandsWetland", "LandWithBeast", "LandWithDisease", "LandWithoutTown", "LandWithTownCity",
    "NonWetland", "JungleOrNoBlight", "LandWhereDahanOutnumberTownCity", "LandWithoutTownCity", "LandWithoutPresence",
    "MountainSands", "None", "LandWithTwoBeast", "LandWithTwoDahan", "CoastalLandWithCity", "LandWithBehemothIncarna",
    "LandWithStrife", "LandWithDeliriumIncarna", "LandWithRootsIncarna", "LandWithRootsIncarnaAndInvaders",
    "LandWithQuake", "LandWithDeeps", "LandWithLocusIncarna", "TheEndlessDark",
];

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

/// Explore range: 1, plus `ModifyExploreRange` effects (the Event Distant
/// Exploration: +1; playtest 2026-10-19 Russia game, Mountain A1 explored from
/// 2 lands away).
pub fn explore_range(effects: &[PublicStatusEffect]) -> i64 {
    1 + effects.iter().filter(|e| e.kind == "ModifyExploreRange").filter_map(|e| int(e, "ExploreRangeModifier")).sum::<i64>()
}

/// Explore sources within `range` of a land: the land itself when it is a
/// source, then lands outward over adjacency (an Ocean is a source but is not
/// passed through). Range 1 is `explore_sources_for`.
pub fn explore_sources_within<'a>(land: &PublicLand, lands: &'a [PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>, range: i64) -> Vec<&'a str> {
    if range <= 1 {
        return explore_sources_for(land, lands, nbrs);
    }
    let by_id = |id: &str| lands.iter().find(|l| l.id == id);
    let mut out = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::from([land.id.clone()]);
    let mut frontier = vec![land.id.clone()];
    if let Some(me) = by_id(&land.id).filter(|l| is_explore_source(l) && !l.is_ocean) {
        out.push(me.id.as_str());
    }
    for _ in 0..range {
        let mut next = Vec::new();
        for id in &frontier {
            for n in nbrs.get(id.as_str()).into_iter().flatten() {
                if !seen.insert(n.to_string()) {
                    continue;
                }
                if let Some(nl) = by_id(n) {
                    if is_explore_source(nl) {
                        out.push(nl.id.as_str());
                    }
                    if !nl.is_ocean {
                        next.push(nl.id.clone());
                    }
                }
            }
        }
        frontier = next;
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

/// Town and City pieces in the lands adjacent to `l`, on `lands`.
pub fn adjacent_towns_cities(l: &PublicLand, lands: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> u32 {
    nbrs.get(l.id.as_str())
        .into_iter()
        .flatten()
        .filter_map(|n| lands.iter().find(|x| x.id == *n))
        .map(|x| x.towns + x.cities)
        .sum()
}

/// England L1, Indentured Servants Earn Land: "Invader Build actions affect
/// lands without Invaders, if they are adjacent to at least 2 Town/City before
/// the Build Action" (engine text). Only lands the card matches (FAQ, "Which
/// lands are affected…"). `before` is the board before this Build card: the
/// pieces it adds don't count for its other lands, but an earlier card's
/// (High Immigration's) do (FAQ, "Does the order of Builds matter…").
/// Playtest 2026-10-19 England game: Mountain D5 (4 adjacent), Sands D7 (4)
/// and Wetland D3 (5) built empty; Wetland D3 with 1 adjacent did not.
pub fn builds_without_invaders(card: &PublicInvaderCard, l: &PublicLand, before: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> bool {
    card_matches(card, l) && !has_invaders(l) && adjacent_towns_cities(l, before, nbrs) >= 2
}

/// Add a built piece to a land, with the Health the island's pieces of that
/// kind have now (Local Autonomy and Health effects included).
pub fn add_built(l: &mut PublicLand, piece: &str, health: i64) {
    match piece {
        "City" => l.cities += 1,
        "Explorer" => l.explorers += 1,
        _ => l.towns += 1,
    }
    l.pieces.push(crate::public::PublicPiece { kind: piece.to_string(), max_health: health, damage: 0, strife: 0 });
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
    /// Blight added beyond the first, without its own cascade or Presence
    /// loss (Sweden L1, Heavy Mining).
    #[serde(default)]
    pub extra_blight: i64,
    /// The Ravage does not happen here (source): no damage, no Blight.
    #[serde(default)]
    pub cancelled: Option<String>,
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
    let mut dahan_protected = false;
    let mut dahan_sitting_out = 0usize;
    let mut invaders_sitting_out: Vec<(Vec<String>, usize, String)> = Vec::new();
    let mut cancelled = None;
    for e in effects {
        let src = e.source.clone().unwrap_or_else(|| e.kind.clone());
        match e.kind.as_str() {
            // "Dahan do not participate in Ravage" (Roiling Bog and Snagging
            // Thorn): they neither take Damage nor counterattack.
            // Temporary Truce (TL I): "In lands with Dahan: 1 Dahan and 1
            // Town/City do not participate in Ravages" (`TargetType`
            // LandWithDahan; playtest 2026-10-19 Sweden game, Sands F8).
            "ChangeRavageParticipation"
                if targets_land(e, l, "SpecificLand")
                    && int(e, "TargetType").is_none_or(|t| TARGET_TYPES.get(t as usize).is_some_and(|name| land_matches_target(l, name))) =>
            {
                let n = int(e, "NumberOfPiecesToRemove").map(|n| n.max(0) as usize).unwrap_or(usize::MAX);
                let types = list(e, "IslandPieceTypesToBeRemoved");
                if types.iter().any(|t| t == "Dahan") {
                    dahan_sitting_out = dahan_sitting_out.saturating_add(n);
                    applied.push(format!("{} Dahan sit out the Ravage ({src})", if n == usize::MAX { "All".to_string() } else { n.to_string() }));
                }
                let invader_types: Vec<String> = types.iter().filter(|t| is_invader(t)).cloned().collect();
                if !invader_types.is_empty() {
                    invaders_sitting_out.push((invader_types, n, src.clone()));
                }
            }
            // "Invaders do not Ravage in lands with Disease/Strife" (Wave of
            // Reconnaissance, Putting Down Roots; playtest 2026-10-19 Sweden
            // game: Wetland F7's City had Strife and its Ravage was cancelled).
            "CancelAction" if cancels(e, CANCEL_RAVAGE) && targets_land(e, l, "TargetIslandLocation") && has_any_token(l, list(e, "ifTokenTypes")) => {
                cancelled = Some(src.clone());
            }
            // Concealing Shadows and similar: "Dahan take no damage from
            // Ravaging Invaders" (`CancelAction`, DamageToPiece, playtest R4).
            "CancelAction"
                if cancels(e, CANCEL_DAMAGE_TO_PIECE)
                    && flag(e, "DamageFromRavage")
                    && list(e, "islandPieceTypes").iter().any(|t| t == "Dahan")
                    && targets_land(e, l, "TargetIslandLocation")
                    && cancel_conditions_met(e, l) =>
            {
                dahan_protected = true;
                applied.push(format!("Dahan take no Ravage damage ({src})"));
            }
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
                        let per = if per_damage_taken { format!(" ({amount:+} per Damage each Invader has taken)") } else { String::new() };
                        applied.push(format!("{src}{per} → {total:+} damage"));
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
    // Invaders sitting out: the players pick, so the forecast takes the
    // pieces that would deal the most damage (rules register: players choose
    // what is best for them, Q21).
    for (types, n, src) in &invaders_sitting_out {
        let mut out = Vec::new();
        for _ in 0..(*n).min(dealing.len()) {
            let best = dealing
                .iter()
                .zip(&per_piece)
                .enumerate()
                .filter(|(i, (p, _))| !out.contains(i) && types.contains(&p.kind))
                .max_by_key(|(_, (_, d))| **d)
                .map(|(i, _)| i);
            match best {
                Some(i) => out.push(i),
                None => break,
            }
        }
        for i in &out {
            applied.push(format!("{} sits out the Ravage ({src}; your pick, best case)", dealing[*i].kind));
        }
        out.sort();
        for i in out.into_iter().rev() {
            dealing.remove(i);
            per_piece.remove(i);
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
    let adds_blight = invader_damage > 0 && after >= land_health && cancelled.is_none();
    let after = if cancelled.is_some() { 0 } else { after };
    if let Some(src) = &cancelled {
        applied.push(format!("No Ravage here ({src})"));
    }

    let mut dahan: Vec<i64> = l.pieces.iter().filter(|p| p.kind == "Dahan").map(|p| (p.max_health - p.damage).max(1)).collect();
    dahan.sort();
    dahan.truncate(dahan.len().saturating_sub(dahan_sitting_out));
    let mut left = if dahan_protected { 0 } else { after };
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
    let counter = if cancelled.is_some() { 0 } else { survivors as i64 * DAHAN_COUNTERATTACK };
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
        extra_blight: 0,
        cancelled,
    }
}

/// The land has a token of one of `types` (Strife is on the pieces); an
/// empty list is no condition.
fn has_any_token(l: &PublicLand, types: &[String]) -> bool {
    types.is_empty() || types.iter().any(|t| token_count(l, t) > 0)
}

fn token_count(l: &PublicLand, t: &str) -> i64 {
    if t == "Strife" {
        l.pieces.iter().map(|p| p.strife).sum()
    } else {
        l.tokens.get(t).copied().unwrap_or(0) as i64
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
    // "Target Spirit provides Defend 2 in each of its lands" (Encompassing
    // Ward): `SpiritsWithPiecesThatDefend` + `PieceTypes: [Presence]`
    // (playtest 2026-10-19 Russia game, Jungle A8 took 0).
    let spirits = list(e, "SpiritsWithPiecesThatDefend");
    let only_presence = list(e, "PieceTypes").iter().all(|p| p == "Presence");
    let other_scope = ["WithinRange", "WithinRangeOfPieceCount", "TokenTypes", "AdjacentOnlyDefendPerPiece", "RequireAllPieceTypes", "DefendPerPiece"];
    if !spirits.is_empty() && only_presence && !other_scope.iter().any(|k| e.params.contains_key(*k)) {
        let present = spirits.iter().any(|sp| l.presence.get(sp).copied().unwrap_or(0) > 0);
        return Some(if present { int(e, "DefendAmount").unwrap_or(0) } else { 0 });
    }
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

/// Effects whose amount depends on the neighboring lands, resolved for one
/// land as a flat effect the ledger reads. Network of Support and Trade
/// (Influx of Settlers, Dahan): "When Invaders Ravage, if the land has Dahan,
/// Defend 1 + Defend 1 per adjacent Dahan" (`AdjacentOnlyDefendPerPiece`,
/// `DefendAmount`, `IslandPieceTypes`). Playtest 2026-10-19 England game,
/// Turn 2: Mountain D5 (1 Dahan, 5 adjacent) took no Blight; Mountain D8
/// (no Dahan, 4 adjacent) did.
///
/// Beset by Many Troubles (TL I/II): "In each land with Badlands/Beasts/
/// Disease/Wilds/Strife, or adjacent to 3 or more such tokens, Defend 5"
/// (`TokenTypes`, `WithinRange` 1, `WithinRangeOfPieceCount`).
pub fn for_land(effects: &[PublicStatusEffect], l: &PublicLand, lands: &[PublicLand], nbrs: &BTreeMap<&str, BTreeSet<&str>>) -> Vec<PublicStatusEffect> {
    let adjacent = || nbrs.get(l.id.as_str()).into_iter().flatten().filter_map(|n| lands.iter().find(|x| x.id == *n));
    let token_scope = ["DefendAmount", "PieceTypes", "TokenTypes", "WithinRange", "WithinRangeOfPieceCount"];
    effects
        .iter()
        .map(|e| {
            let amount = if let Some(per) = int(e, "AdjacentOnlyDefendPerPiece").filter(|_| e.kind == "Defend") {
                let kinds = list(e, "IslandPieceTypes");
                let count = |x: &PublicLand| x.pieces.iter().filter(|p| kinds.contains(&p.kind)).count() as i64;
                if count(l) > 0 { int(e, "DefendAmount").unwrap_or(0) + per * adjacent().map(count).sum::<i64>() } else { 0 }
            } else if e.kind == "Defend"
                && e.params.contains_key("TokenTypes")
                && e.params.keys().all(|k| token_scope.contains(&k.as_str()))
                && int(e, "WithinRange").unwrap_or(1) <= 1
            {
                let types = list(e, "TokenTypes");
                let tokens = |x: &PublicLand| types.iter().map(|t| token_count(x, t)).sum::<i64>();
                let own = tokens(l);
                let near = int(e, "WithinRangeOfPieceCount").is_some_and(|n| own + adjacent().map(tokens).sum::<i64>() >= n);
                if own > 0 || near { int(e, "DefendAmount").unwrap_or(0) } else { 0 }
            } else {
                return e.clone();
            };
            let mut flat = e.clone();
            flat.params = [
                ("DefendAmount".to_string(), StatusParam::Int(amount)),
                ("AffectsAllLands".to_string(), StatusParam::Bool(true)),
            ]
            .into_iter()
            .collect();
            flat
        })
        .collect()
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
        "IfLandHasMinimumInvaders", "IfLandHasTownOrCity", "IfLandHasNoStrife", "AffectedBoard",
        "IfLandHasDahan", "OnlyDamageToLand", "OnlyDamageToDahan", "OnlyLowestNumberedMatchingLand",
        "PerPresenceOfSpiritInLand", "OnlyDamageFromDahan", "AddInvaderStageAsDamage", "IfLandHasBeast",
        "NumberOfTimesPerBoard", "CheckConditionFromSource", "NormalRavageOnly",
    ];
    if conditions.iter().any(|k| e.params.contains_key(*k)) {
        return None;
    }
    // Investigation of Dangers (TL II/III): "Invaders do +3 Damage (per
    // land) when Ravaging in lands with Presence".
    let no_presence = flag(e, "IfLandHasPresence") && l.presence.values().all(|n| *n == 0);
    if !targets_land(e, l, "AffectedLands") || no_presence {
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

/// `Engine.CancelActionType` values (DLL enum constants, 2026-10-19).
const CANCEL_BUILD: i64 = 1;
const CANCEL_DAMAGE_TO_PIECE: i64 = 2;
const CANCEL_RAVAGE: i64 = 5;

/// A `CancelAction` status effect of this type. The type is an enum, so the
/// projection carries it as an integer; Explore (0) is the default value and
/// may be absent.
fn cancels(e: &PublicStatusEffect, kind: i64) -> bool {
    int(e, "CancelActionType").unwrap_or(0) == kind
}

/// Conditions a `CancelAction` can carry that the companion checks.
fn cancel_conditions_met(e: &PublicStatusEffect, l: &PublicLand) -> bool {
    !flag(e, "DahanOutnumberTownsAndCities") || l.dahan > l.towns + l.cities
}

/// Does a land match an engine `TargetType` used to restrict an effect?
/// Unknown types don't match, so the effect isn't applied rather than guessed.
fn land_matches_target(l: &PublicLand, target: &str) -> bool {
    match target {
        "AnyLand" => true,
        "Coastal" => l.coastal,
        "Inland" => !l.coastal && !l.is_ocean,
        "Jungle" | "Mountain" | "Sands" | "Wetland" | "Ocean" => l.terrain == target,
        "LandWithInvaders" => has_invaders(l),
        "LandWithNoInvaders" => !has_invaders(l),
        "LandWithDahan" => l.dahan > 0,
        "LandWithBlight" => l.blight > 0,
        "LandWithNoBlight" => l.blight == 0,
        "LandWithCity" => l.cities > 0,
        "LandWithTownCity" => l.towns + l.cities > 0,
        "LandWithoutTownCity" => l.towns + l.cities == 0,
        _ => false,
    }
}

/// Build outcome in a land, after tokens and effects. Engine, Test 1 Turn 5:
/// `Disease1 ( Jungle C1 => OutOfGame)` then `CancelAction: BuildAction`
/// (a Disease stops the Build and is removed), unless an `IgnoreToken`
/// effect ignores Disease during the Invader Phase.
pub fn build_outcome(l: &PublicLand, effects: &[PublicStatusEffect]) -> (Option<&'static str>, Vec<String>) {
    build_outcome_with(l, effects, &mut BuildCancels::new(effects))
}

/// Build-cancelling effects (`CancelAction`, Build) with their uses left, so
/// one-use cancels apply once across the Build cards and lands of a phase.
#[derive(Clone)]
pub struct BuildCancels<'a> {
    items: Vec<(&'a PublicStatusEffect, i64)>,
}

impl<'a> BuildCancels<'a> {
    pub fn new(effects: &'a [PublicStatusEffect]) -> Self {
        let items = effects
            .iter()
            .filter(|e| e.kind == "CancelAction" && cancels(e, CANCEL_BUILD))
            .map(|e| (e, int(e, "NumberOfUses").unwrap_or(i64::MAX)))
            .collect();
        BuildCancels { items }
    }

    fn whole_card(e: &PublicStatusEffect) -> bool {
        (flag(e, "SkipStateEntirely") || flag(e, "NextInvaderState"))
            && list(e, "TargetIslandLocation").is_empty()
            && !flag(e, "LowestNumberMatchingLand")
    }

    /// Immigration Slows (TL I): "During the next normal Build, skip the
    /// lowest-numbered land matching the Invader Card on each board"
    /// (`LowestNumberMatchingLand`, one use; playtest 2026-10-19 Sweden game:
    /// Wetland F3 skipped, Wetland F7 still built).
    pub fn lowest(&mut self, normal: bool) -> Option<String> {
        let (e, uses) = self
            .items
            .iter_mut()
            .find(|(e, uses)| *uses > 0 && flag(e, "LowestNumberMatchingLand") && (normal || !flag(e, "NormalActionOnly")))?;
        *uses -= 1;
        Some(e.source.clone().unwrap_or_else(|| "an effect".into()))
    }

    /// "Skip the next normal Build" (Immigration Slows: `SkipStateEntirely`,
    /// `NextInvaderState`, one use): a whole Build card's resolution.
    /// `normal`: the card reached Build by normal advancement (playtest
    /// 2026-10-19 Russia game: the cancel skipped Jungle+Wetland, and the
    /// Jungle card Entrench in the Face of Fear put there still built).
    pub fn card(&mut self, normal: bool) -> Option<String> {
        let (e, uses) = self
            .items
            .iter_mut()
            .find(|(e, uses)| *uses > 0 && Self::whole_card(e) && (normal || !flag(e, "NormalActionOnly")))?;
        *uses -= 1;
        Some(e.source.clone().unwrap_or_else(|| "an effect".into()))
    }

    /// One Build in a land: Overseas Trade Seems Safer (TL II, "Skip Build
    /// Actions in Coastal lands that would add a City": islandPieceTypes,
    /// targetTypes; playtest R6) or Unsettled (one land, one use).
    pub fn land(&mut self, l: &PublicLand, piece: &str) -> Option<String> {
        let (e, uses) = self.items.iter_mut().find(|(e, uses)| {
            let pieces = list(e, "islandPieceTypes");
            *uses > 0
                && !Self::whole_card(e)
                && !flag(e, "LowestNumberMatchingLand")
                && (pieces.is_empty() || pieces.iter().any(|p| p == piece))
                && targets_land(e, l, "TargetIslandLocation")
                && list(e, "targetTypes").iter().all(|t| land_matches_target(l, t))
                && cancel_conditions_met(e, l)
        })?;
        *uses -= 1;
        Some(e.source.clone().unwrap_or_else(|| "an effect".into()))
    }
}

pub fn build_outcome_with(l: &PublicLand, effects: &[PublicStatusEffect], cancels: &mut BuildCancels) -> (Option<&'static str>, Vec<String>) {
    let mut notes = Vec::new();
    for e in effects {
        if e.kind == "SkipInvaderActions" && targets_land(e, l, "islandLocationToSkip") && skips_step(e, "Build") {
            notes.push(format!("Skip available: {}", e.source.clone().unwrap_or_default()));
        }
    }
    if let Some(src) = cancels.land(l, build_piece(l)) {
        notes.push(format!("Build skipped ({src})"));
        return (None, notes);
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
            source_kind: "power".into(),
            params: params.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
        }
    }

    #[test]
    fn concealing_shadows_spares_dahan() {
        // R4: 3 damage would kill the 1 Dahan; Concealing Shadows' cancel
        // keeps it alive to counterattack (the land still Blights).
        let mut l = land(&[("Town", 2), ("Explorer", 1), ("Dahan", 2)], 0, 0);
        l.id = "WetlandB1".into();
        let e = effect(
            "CancelAction",
            &[
                ("CancelActionType", StatusParam::Int(2)),
                ("DamageFromRavage", StatusParam::Bool(true)),
                ("islandPieceTypes", StatusParam::List(vec!["Dahan".into()])),
                ("TargetIslandLocation", StatusParam::List(vec!["WetlandB1".into()])),
            ],
        );
        let r = ravage_ledger(&l, &[e]);
        assert_eq!((r.adds_blight, r.dahan_destroyed, r.counterattack_damage), (true, 0, 2));
    }

    #[test]
    fn coastal_city_build_skipped() {
        // R6: Overseas Trade Seems Safer (TL II) skips Coastal Builds that would add a City.
        let e = effect(
            "CancelAction",
            &[
                ("CancelActionType", StatusParam::Int(1)),
                ("islandPieceTypes", StatusParam::List(vec!["City".into()])),
                ("targetTypes", StatusParam::List(vec!["Coastal".into()])),
            ],
        );
        let mut coastal = land(&[("Town", 2), ("Explorer", 1)], 0, 0);
        coastal.coastal = true;
        assert_eq!(build_outcome(&coastal, &[e.clone()]).0, None);
        let inland = land(&[("Town", 2), ("Explorer", 1)], 0, 0);
        assert_eq!(build_outcome(&inland, &[e.clone()]).0, Some("City"));
        // A Coastal land that would add a Town still Builds.
        let mut coastal_town = land(&[("Explorer", 1)], 0, 0);
        coastal_town.coastal = true;
        assert_eq!(build_outcome(&coastal_town, &[e]).0, Some("Town"));
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

    /// Lands `ids[i]` with `pieces[i]`, all adjacent to the first.
    fn star(ids: &[&str], pieces: &[&[(&str, i64)]]) -> (Vec<PublicLand>, Vec<(String, String)>) {
        let lands: Vec<PublicLand> = ids
            .iter()
            .zip(pieces)
            .map(|(id, p)| PublicLand { id: id.to_string(), terrain: id.trim_end_matches(char::is_numeric).into(), ..land(p, 0, 0) })
            .collect();
        let adjacency = ids[1..].iter().map(|id| (ids[0].to_string(), id.to_string())).collect();
        (lands, adjacency)
    }

    #[test]
    fn england_l1_builds_in_empty_land_beside_two_towns_or_cities() {
        // England game, Turn 1: Mountain D5 had only Dahan; its neighbors had
        // a City, a Town and a City, and a Town (4 in all). Turn 4: Wetland D3
        // beside a single City did not build.
        let (lands, adjacency) = star(&["Mountain5", "Wetland1", "Jungle2", "Sands7"], &[&[("Dahan", 2)], &[("City", 3)], &[("Town", 2), ("City", 3)], &[("Town", 2)]]);
        let s = PublicSnapshot { adjacency, ..Default::default() };
        let nbrs = neighbors(&s);
        let mountain = PublicInvaderCard { id: "Mountain1".into(), stage: 1, terrains: vec!["Mountain".into()], ..Default::default() };
        assert!(!builds_in(&mountain, &lands[0]));
        assert_eq!(adjacent_towns_cities(&lands[0], &lands, &nbrs), 4);
        assert!(builds_without_invaders(&mountain, &lands[0], &lands, &nbrs));
        let (lands, adjacency) = star(&["Wetland3", "Jungle2"], &[&[], &[("City", 3)]]);
        let s = PublicSnapshot { adjacency, ..Default::default() };
        let wetland = PublicInvaderCard { id: "Wetland1".into(), stage: 1, terrains: vec!["Wetland".into()], ..Default::default() };
        assert!(!builds_without_invaders(&wetland, &lands[0], &lands, &neighbors(&s)));
    }

    #[test]
    fn network_of_support_defends_by_adjacent_dahan() {
        // England game, Turn 2: "if the land has Dahan, Defend 1 + Defend 1 per
        // adjacent Dahan". Mountain D5 (Town, 1 Dahan, 5 adjacent Dahan) took
        // no Blight; Mountain D8 (Town, Explorer, no Dahan) did.
        let e = effect(
            "Defend",
            &[
                ("AdjacentOnlyDefendPerPiece", StatusParam::Int(1)),
                ("DefendAmount", StatusParam::Int(1)),
                ("IslandPieceTypes", StatusParam::List(vec!["Dahan".into()])),
                ("PieceTypes", StatusParam::List(vec!["IslandPiece".into()])),
            ],
        );
        let (lands, adjacency) = star(
            &["Mountain5", "Wetland1", "Jungle2", "Sands7"],
            &[&[("Town", 2), ("Dahan", 2)], &[("Dahan", 2), ("Dahan", 2)], &[("Dahan", 2)], &[("Dahan", 2), ("Dahan", 2)]],
        );
        let s = PublicSnapshot { adjacency, ..Default::default() };
        let nbrs = neighbors(&s);
        let r = ravage_ledger(&lands[0], &for_land(std::slice::from_ref(&e), &lands[0], &lands, &nbrs));
        assert_eq!((r.defend, r.adds_blight), (6, false));
        let (lands, adjacency) = star(&["Mountain8", "Wetland1"], &[&[("Town", 2), ("Explorer", 1)], &[("Dahan", 2), ("Dahan", 2)]]);
        let s = PublicSnapshot { adjacency, ..Default::default() };
        let r = ravage_ledger(&lands[0], &for_land(&[e], &lands[0], &lands, &neighbors(&s)));
        assert_eq!((r.defend, r.adds_blight), (0, true));
    }

    #[test]
    fn temporary_truce_sits_out_the_city_in_lands_with_dahan() {
        // Sweden game, Turn 4, Sands F8: 2 Explorers, Town (3), City (5), 1
        // Dahan, Defend 4. The City sat out: 5 damage − 4 = 1, no Blight.
        let truce = |types: &[&str]| {
            effect(
                "ChangeRavageParticipation",
                &[
                    ("IslandPieceTypesToBeRemoved", StatusParam::List(types.iter().map(|t| t.to_string()).collect())),
                    ("NumberOfPiecesToRemove", StatusParam::Int(1)),
                    ("TargetType", StatusParam::Int(12)),
                ],
            )
        };
        let steel = |kind: &str, d: i64| {
            effect("AdjustRavageDamage", &[("DamageAdjustment", StatusParam::Int(d)), ("PerInvaderOfType", StatusParam::List(vec![kind.into()]))])
        };
        let effects = [truce(&["Dahan"]), truce(&["Town", "City"]), steel("Town", 1), steel("City", 2)];
        let l = land(&[("Explorer", 1), ("Explorer", 1), ("Town", 2), ("City", 3), ("Dahan", 2)], 2, 4);
        let r = ravage_ledger(&l, &effects);
        assert_eq!((r.invader_damage, r.damage_after_defend, r.adds_blight), (5, 1, false));
        // Without Dahan the Truce doesn't apply.
        let l = land(&[("Explorer", 1), ("Town", 2)], 0, 0);
        assert_eq!(ravage_ledger(&l, &effects).invader_damage, 4);
    }

    #[test]
    fn no_ravage_in_lands_with_strife() {
        // Wave of Reconnaissance: "Invaders do not Ravage in lands with
        // Disease/Strife" (Sweden game, Turn 6, Wetland F7's City had Strife).
        let e = effect("CancelAction", &[("CancelActionType", StatusParam::Int(5)), ("ifTokenTypes", StatusParam::List(vec!["Disease".into(), "Strife".into()]))]);
        let mut l = land(&[("Town", 2), ("City", 3)], 0, 0);
        assert!(ravage_ledger(&l, std::slice::from_ref(&e)).adds_blight);
        l.pieces[1].strife = 1;
        let r = ravage_ledger(&l, &[e]);
        assert!(!r.adds_blight && r.cancelled.is_some());
    }

    #[test]
    fn beset_by_many_troubles_defends_near_tokens() {
        // TL II: Defend 5 in lands with a token, or with 3+ tokens within 1.
        let e = effect(
            "Defend",
            &[
                ("DefendAmount", StatusParam::Int(5)),
                ("TokenTypes", StatusParam::List(vec!["Beast".into(), "Disease".into(), "Strife".into()])),
                ("WithinRange", StatusParam::Int(1)),
                ("WithinRangeOfPieceCount", StatusParam::Int(3)),
            ],
        );
        let (mut lands, adjacency) = star(&["Sands1", "Jungle2", "Wetland3"], &[&[("Town", 2)], &[], &[]]);
        lands[1].tokens.insert("Beast".into(), 2);
        let s = PublicSnapshot { adjacency, ..Default::default() };
        let nbrs = neighbors(&s);
        let d = |lands: &[PublicLand]| ravage_ledger(&lands[0], &for_land(std::slice::from_ref(&e), &lands[0], lands, &nbrs)).defend;
        assert_eq!(d(&lands), 0);
        lands[2].tokens.insert("Disease".into(), 1);
        assert_eq!(d(&lands), 5);
    }

    #[test]
    fn immigration_slows_one_skips_only_the_lowest_land() {
        let e = effect(
            "CancelAction",
            &[
                ("CancelActionType", StatusParam::Int(1)),
                ("LowestNumberMatchingLand", StatusParam::Bool(true)),
                ("NextInvaderState", StatusParam::Bool(true)),
                ("NormalActionOnly", StatusParam::Bool(true)),
                ("NumberOfUses", StatusParam::Int(1)),
            ],
        );
        let effects = [e];
        let mut c = BuildCancels::new(&effects);
        assert_eq!(c.card(true), None, "not a whole-card skip");
        assert_eq!(c.land(&land(&[("Town", 2)], 0, 0), "City"), None, "not a per-land cancel either");
        assert!(c.lowest(true).is_some());
        assert!(c.lowest(true).is_none(), "one use");
    }

    #[test]
    fn plus_three_damage_only_in_lands_with_presence() {
        // Investigation of Dangers (TL II/III): +3 per land with Presence.
        let e = effect("AdjustRavageDamage", &[("DamageAdjustment", StatusParam::Int(3)), ("IfLandHasPresence", StatusParam::Bool(true))]);
        let mut l = land(&[("Explorer", 1)], 0, 0);
        assert_eq!(ravage_ledger(&l, std::slice::from_ref(&e)).damage_after_defend, 1);
        l.presence.insert("OceansHungryGrasp".into(), 1);
        assert_eq!(ravage_ledger(&l, &[e]).damage_after_defend, 4);
    }
}
