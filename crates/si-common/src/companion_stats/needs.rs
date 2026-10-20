//! Need finder (spec 7.4): which unseen Minors and Majors could meet a need
//! now, as natural frequencies and draw-of-4 odds.
//!
//! Effects are tagged from the engine's card text, one tag set per `{Or}`
//! option. Lines with a condition ("If …", "For each …") are tagged but
//! flagged `conditional`. "Saves the land" means any effect that brings
//! this Ravage's damage below the land's health, not one keyword (Brett,
//! Test 1 Turn 3: a Defend-only count missed Lure of the Unknown).

use crate::probability::hypergeom_at_least;
use crate::public::{PublicLand, PublicPowerCard, PublicSnapshot, PublicSpirit};
use crate::rules::{self, RavageLedger};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, VecDeque};

/// Effects of one card option.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EffectTags {
    pub defend: i64,
    pub damage: i64,
    /// Damage is per Dahan in the target land ("For each {Dahan}, 1 Damage").
    pub damage_per_dahan: bool,
    /// Damage only to these kinds (e.g. Town/City); empty = any Invader.
    pub damage_to: Vec<String>,
    pub fear: i64,
    pub push: Vec<(String, i64)>,
    pub gather: Vec<(String, i64)>,
    pub destroy: Vec<(String, i64)>,
    pub strife: i64,
    pub downgrade: bool,
    pub skip: Vec<String>,
    pub isolate: bool,
    pub remove_blight: i64,
    pub conditional: bool,
}

const PIECES: [&str; 8] = ["Explorer", "Town", "City", "Dahan", "Blight", "Beast", "Wilds", "Disease"];

fn num_before(s: &str, idx: usize) -> Option<i64> {
    let head = s[..idx].trim_end();
    let digits: String = head.chars().rev().take_while(|c| c.is_ascii_digit()).collect::<Vec<_>>().into_iter().rev().collect();
    digits.parse().ok()
}

/// Pieces named right after a verb: "Push up to 2 {Explorer}/{Town}".
fn verb_pieces(line: &str, verb: &str) -> Vec<(String, i64)> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(i) = rest.find(verb) {
        let after = &rest[i + verb.len()..];
        let after = after.trim_start().trim_start_matches("up to").trim_start();
        let n: i64 = if after.starts_with("all") {
            99
        } else {
            after.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(1)
        };
        let tail: String = after.chars().take(60).collect();
        for p in PIECES {
            if tail.split('.').next().unwrap_or("").contains(&format!("{{{p}}}")) {
                out.push((p.to_string(), n));
            }
        }
        rest = &rest[i + verb.len()..];
    }
    out
}

pub fn tag_options(card: &PublicPowerCard) -> Vec<EffectTags> {
    let mut options = vec![EffectTags::default()];
    for line in &card.body {
        if line.trim() == "{Or}" {
            options.push(EffectTags::default());
            continue;
        }
        let t = options.last_mut().unwrap();
        let lower = line.to_lowercase();
        if lower.starts_with("if ") || lower.contains(" if ") || lower.contains("for each") || lower.contains("instead") {
            t.conditional = true;
        }
        if let Some(i) = line.find("Defend") {
            if let Some(n) = line[i + 6..].trim_start().split(|c: char| !c.is_ascii_digit()).next().and_then(|d| d.parse::<i64>().ok()) {
                t.defend += n;
            }
        }
        for (i, _) in line.match_indices(" Damage").chain(line.match_indices("Damage").filter(|(i, _)| *i == 0)) {
            if let Some(n) = num_before(line, i) {
                t.damage += n;
            }
        }
        if t.damage > 0 && lower.contains("for each {dahan}") {
            t.damage_per_dahan = true;
        }
        if line.contains("Damage to {Town}/{City}") || line.contains("Damage to each {Town}/{City}") {
            t.damage_to = vec!["Town".into(), "City".into()];
        }
        for (i, _) in line.match_indices(" {Fear}") {
            if let Some(n) = num_before(line, i) {
                t.fear += n;
            }
        }
        t.push.extend(verb_pieces(line, "Push"));
        t.gather.extend(verb_pieces(line, "Gather"));
        t.destroy.extend(verb_pieces(line, "Destroy").into_iter().filter(|(p, _)| p != "Dahan"));
        if line.contains("{Strife}") && (line.contains("Add") || line.contains("Strife.")) {
            t.strife += line.split("{Strife}").count() as i64 - 1;
        }
        if lower.contains("downgrade") {
            t.downgrade = true;
        }
        if lower.contains("skip") {
            for a in ["Ravage", "Build", "Explore"] {
                if line.contains(a) || lower.contains("invader actions") {
                    t.skip.push(a.to_string());
                }
            }
        }
        if line.contains("Isolate") {
            t.isolate = true;
        }
        if let Some(i) = line.find("remove") .or_else(|| line.find("Remove")) {
            if line[i..].contains("{Blight}") {
                t.remove_blight += line[i + 6..].trim_start().split(|c: char| !c.is_ascii_digit()).next().and_then(|d| d.parse().ok()).unwrap_or(1);
            }
        }
    }
    options
}

/// Damage this option removes from a Ravage in `l` (or `i64::MAX` if it skips it).
pub(crate) fn reduction(t: &EffectTags, l: &PublicLand) -> i64 {
    if t.skip.iter().any(|s| s == "Ravage") {
        return i64::MAX;
    }
    let invaders: Vec<(&str, i64, i64)> = l
        .pieces
        .iter()
        .filter(|p| rules::is_invader(&p.kind) && p.strife == 0)
        .map(|p| (p.kind.as_str(), rules::invader_damage(&p.kind), (p.max_health - p.damage).max(1)))
        .collect();
    let mut cut = t.defend;
    // Damage: the most Ravage damage removable by destroying pieces within it.
    let dmg = if t.damage_per_dahan { t.damage * l.dahan as i64 } else { t.damage };
    if dmg > 0 {
        let pool: Vec<&(&str, i64, i64)> =
            invaders.iter().filter(|(k, _, _)| t.damage_to.is_empty() || t.damage_to.iter().any(|x| x == k)).collect();
        let n = pool.len().min(12);
        let mut best = 0;
        for mask in 1u32..(1 << n) {
            let (hp, d) = (0..n).filter(|i| mask & (1 << i) != 0).fold((0, 0), |(h, d), i| (h + pool[i].2, d + pool[i].1));
            if hp <= dmg {
                best = best.max(d);
            }
        }
        cut += best;
    }
    // Destroy / Push / Strife: remove the most damaging pieces they can name.
    let mut by_damage: Vec<(&str, i64)> = invaders.iter().map(|(k, d, _)| (*k, *d)).collect();
    by_damage.sort_by_key(|(_, d)| std::cmp::Reverse(*d));
    for (kind, n) in t.destroy.iter().chain(&t.push) {
        let mut left = *n;
        by_damage.retain(|(k, d)| {
            if left > 0 && k == kind {
                left -= 1;
                cut += d;
                false
            } else {
                true
            }
        });
    }
    for (_, d) in by_damage.iter().take(t.strife.max(0) as usize) {
        cut += d;
    }
    if t.downgrade && invaders.iter().any(|(k, _, _)| *k != "Explorer") {
        cut += 1;
    }
    cut
}

fn target_ok(target: Option<&str>, l: &PublicLand) -> Option<bool> {
    let t = target?;
    let any_inv = rules::has_invaders(l);
    Some(match t {
        "AnyLand" => true,
        "LandWithInvaders" => any_inv,
        "LandWithDahan" => l.dahan > 0,
        "LandWithTwoDahan" => l.dahan >= 2,
        "LandWithNoBlight" => l.blight == 0,
        "LandWithBlight" => l.blight > 0,
        "LandWithTownCity" => l.towns + l.cities > 0,
        "Coastal" => l.coastal,
        "Inland" => !l.coastal,
        "LandWithDisease" => l.tokens.get("Disease").copied().unwrap_or(0) > 0,
        "LandWithStrife" => l.pieces.iter().any(|p| p.strife > 0),
        "NonWetland" => l.terrain != "Wetland",
        other if other.contains(l.terrain.as_str()) && ["Jungle", "Mountain", "Sands", "Wetland"].iter().any(|x| other.starts_with(x)) => true,
        other if ["Jungle", "Mountain", "Sands", "Wetland"].iter().any(|x| other.starts_with(x)) => false,
        _ => return None,
    })
}

/// Shortest land distance from any of `from` (BFS over adjacency). "The
/// Ocean itself is not a land, and is not in play" (core rulebook), so Range
/// never passes through an Ocean, except that with Ocean's Hungry Grasp in
/// play Powers may measure Range through Oceans (FAQ; open-questions Q29).
fn distance(s: &PublicSnapshot, from: &[&str], to: &str) -> Option<i64> {
    let nbrs = rules::neighbors(s);
    let ocean_in_play = s.spirits.iter().any(|sp| sp.id == "OceansHungryGrasp");
    let ocean = |id: &str| s.lands.iter().any(|l| l.id == id && l.is_ocean);
    let mut seen: BTreeSet<&str> = from.iter().copied().collect();
    let mut q: VecDeque<(&str, i64)> = from.iter().map(|f| (*f, 0)).collect();
    while let Some((id, d)) = q.pop_front() {
        if id == to {
            return Some(d);
        }
        for n in nbrs.get(id).into_iter().flatten() {
            if (ocean_in_play || !ocean(n) || *n == to) && seen.insert(n) {
                q.push_back((n, d + 1));
            }
        }
    }
    None
}

/// Can this spirit target `l` with the card (range from presence or a
/// Sacred Site)? `None` when the card's reach rule is not modeled.
pub(crate) fn reaches(card: &PublicPowerCard, sp: &PublicSpirit, s: &PublicSnapshot, l: &PublicLand) -> Option<bool> {
    let range = card.range?;
    let ok = target_ok(card.target.as_deref(), l)?;
    let mine: Vec<&str> = s
        .lands
        .iter()
        .filter(|x| x.presence.get(&sp.id).copied().unwrap_or(0) >= if card.from_sacred_site { 2 } else { 1 })
        .map(|x| x.id.as_str())
        .collect();
    if mine.is_empty() {
        return Some(false);
    }
    Some(ok && distance(s, &mine, &l.id).is_some_and(|d| d <= range))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NeedCount {
    /// Cards whose effect meets the need (any option, conditions allowed).
    pub effect: u32,
    /// …and can target the land from this spirit's presence now.
    pub reach: u32,
    /// …and are Fast (act before this turn's Invader Phase).
    pub fast_reach: u32,
    pub pool: u32,
    /// P(≥1 in a draw of 4) for effect / reach / fast_reach.
    pub p_effect: f64,
    pub p_reach: f64,
    pub p_fast_reach: f64,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SaveLandNeed {
    pub land: String,
    /// Damage to remove for the land to take no blight.
    pub margin: i64,
    pub minor: NeedCount,
    pub major: NeedCount,
    /// Cards in hand that could do it, with reach and speed.
    pub in_hand: Vec<String>,
    pub pocket_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpiritNeeds {
    pub spirit: String,
    pub save_land: Vec<SaveLandNeed>,
    /// Effect families in the unseen Minor pool (any option): (need, cards, pool, P≥1 in 4).
    pub minor_families: Vec<(String, u32, u32, f64)>,
    pub major_families: Vec<(String, u32, u32, f64)>,
    pub basis: Vec<String>,
}

fn count(pool: &[PublicPowerCard], pred_effect: impl Fn(&PublicPowerCard) -> bool, pred_reach: impl Fn(&PublicPowerCard) -> bool) -> NeedCount {
    let n = pool.len() as u32;
    let eff: Vec<&PublicPowerCard> = pool.iter().filter(|c| pred_effect(c)).collect();
    let reach: Vec<&&PublicPowerCard> = eff.iter().filter(|c| pred_reach(c)).collect();
    let fast = reach.iter().filter(|c| c.speed == "Fast").count() as u32;
    let p = |k: u32| if n == 0 { 0.0 } else { hypergeom_at_least(n, k, 4.min(n), 1) };
    NeedCount {
        effect: eff.len() as u32,
        reach: reach.len() as u32,
        fast_reach: fast,
        pool: n,
        p_effect: p(eff.len() as u32),
        p_reach: p(reach.len() as u32),
        p_fast_reach: p(fast),
        examples: reach.iter().take(8).map(|c| c.title.clone()).collect(),
    }
}

fn families(pool: &[PublicPowerCard]) -> Vec<(String, u32, u32, f64)> {
    let tags: Vec<Vec<EffectTags>> = pool.iter().map(tag_options).collect();
    let n = pool.len() as u32;
    let fam = |name: &str, f: &dyn Fn(&EffectTags) -> bool| {
        let k = tags.iter().filter(|opts| opts.iter().any(f)).count() as u32;
        (name.to_string(), k, n, if n == 0 { 0.0 } else { hypergeom_at_least(n, k, 4.min(n), 1) })
    };
    let invader = |v: &[(String, i64)]| v.iter().any(|(p, _)| rules::is_invader(p));
    vec![
        fam("Defend 1+", &|t| t.defend >= 1),
        fam("Defend 2+", &|t| t.defend >= 2),
        fam("Damage 1+", &|t| t.damage >= 1),
        fam("Damage 2+", &|t| t.damage >= 2 && !t.damage_per_dahan),
        fam("Fear", &|t| t.fear >= 1),
        fam("Push Invaders", &|t| invader(&t.push)),
        fam("Gather Invaders", &|t| invader(&t.gather)),
        fam("Push or Gather Dahan", &|t| t.push.iter().chain(&t.gather).any(|(p, _)| p == "Dahan")),
        fam("Destroy Invaders", &|t| invader(&t.destroy)),
        fam("Strife", &|t| t.strife > 0),
        fam("Skip an Invader action", &|t| !t.skip.is_empty()),
        fam("Isolate", &|t| t.isolate),
        fam("Remove Blight", &|t| t.remove_blight > 0),
    ]
}

pub fn spirit_needs(sp: &PublicSpirit, s: &PublicSnapshot, ravages: &[(String, RavageLedger)]) -> SpiritNeeds {
    let mut save_land = Vec::new();
    for (id, ledger) in ravages.iter().filter(|(_, l)| l.adds_blight) {
        let Some(l) = s.lands.iter().find(|x| x.id == *id) else { continue };
        let margin = ledger.damage_after_defend - (ledger.land_health - 1);
        let saves = |c: &PublicPowerCard| tag_options(c).iter().any(|t| reduction(t, l) >= margin);
        let reach = |c: &PublicPowerCard| reaches(c, sp, s, l).unwrap_or(false);
        let in_hand = sp
            .hand
            .iter()
            .filter(|c| saves(c))
            .map(|c| {
                let r = match reaches(c, sp, s, l) {
                    Some(true) => "reaches",
                    Some(false) => "out of reach",
                    None => "reach not modeled",
                };
                format!("{} ({}, {}, {} Energy)", c.title, c.speed, r, c.cost)
            })
            .collect();
        let minor = count(&s.powers.minor_unseen, saves, reach);
        let major = count(&s.powers.major_unseen, saves, reach);
        save_land.push(SaveLandNeed {
            land: id.clone(),
            margin,
            pocket_rule: format!(
                "{} of {} unseen Minors could save {id}; P(at least one in a draw of 4) ≈ 1 − (1 − {}/{})⁴.",
                minor.effect, minor.pool, minor.effect, minor.pool
            ),
            minor,
            major,
            in_hand,
        });
    }
    SpiritNeeds {
        spirit: sp.title.clone(),
        save_land,
        minor_families: families(&s.powers.minor_unseen),
        major_families: families(&s.powers.major_unseen),
        basis: vec![
            "Effects are read from the card text, one set per {Or} option; conditional lines count.".into(),
            "Reach uses range from presence (Sacred Site when the card says so) and the target type; spirit special rules (e.g. Shadows paying 1 Energy to target a land with Dahan) are not applied.".into(),
            "Range is counted over lands only; Oceans count only with Ocean's Hungry Grasp in play.".into(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(body: &[&str]) -> PublicPowerCard {
        PublicPowerCard { title: "T".into(), body: body.iter().map(|s| s.to_string()).collect(), ..Default::default() }
    }

    #[test]
    fn tags_or_options() {
        let t = tag_options(&card(&["For each {Dahan}, 1 Damage to {Town}/{City}.", "{Or}", "1 {Fear}. Gather up to 2 {Dahan}."]));
        assert_eq!(t.len(), 2);
        assert!(t[0].damage_per_dahan && t[0].conditional && t[0].damage == 1);
        assert_eq!(t[0].damage_to, vec!["Town", "City"]);
        assert_eq!(t[1].fear, 1);
        assert_eq!(t[1].gather, vec![("Dahan".to_string(), 2)]);
    }

    #[test]
    fn tags_defend_push_strife() {
        let t = tag_options(&card(&["Defend 2.", "Push up to 2 {Explorer}/{Town}.", "Add 1 {Strife}."]));
        assert_eq!(t[0].defend, 2);
        assert_eq!(t[0].push, vec![("Explorer".to_string(), 2), ("Town".to_string(), 2)]);
        assert_eq!(t[0].strife, 1);
    }

    #[test]
    fn range_skips_oceans_unless_ocean_in_play() {
        // C1 - Ocean - C3, and C1 - C2 - C4 - C3 over land.
        let land = |id: &str, ocean: bool| PublicLand { id: id.into(), is_ocean: ocean, ..Default::default() };
        let mut s = PublicSnapshot {
            lands: vec![land("C1", false), land("O", true), land("C3", false), land("C2", false), land("C4", false)],
            adjacency: [("C1", "O"), ("O", "C3"), ("C1", "C2"), ("C2", "C4"), ("C4", "C3")].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(),
            ..Default::default()
        };
        assert_eq!(distance(&s, &["C1"], "C3"), Some(3));
        s.spirits.push(PublicSpirit { id: "OceansHungryGrasp".into(), ..Default::default() });
        assert_eq!(distance(&s, &["C1"], "C3"), Some(2));
    }
}
