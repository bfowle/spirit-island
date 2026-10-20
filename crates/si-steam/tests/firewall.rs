//! Firewall tests (spec Section 5, Enforcement 2 and 4).
//!
//! Run against Brett's local Test 1 capture and installed engine DLL. Both
//! are personal or proprietary data and are not in this (public) repo, so
//! each test returns early when they are absent.
//!
//! Invariance: permuting hidden card identities within groups that look the
//! same to a player must not change a single byte of the public output.

use si_steam::engine_defs::{DEFAULT_DLL, EngineDefs};
use si_steam::engine_map::{self, EngineCard, EngineSnapshot};
use si_steam::{nrbf, projection};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn capture_dir() -> PathBuf {
    std::env::var_os("SI_STEAM_CAPTURE").map(PathBuf::from).unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/steam-capture/2026-10-18-test1/saves")
    })
}

fn load() -> Option<(EngineDefs, Vec<(String, EngineSnapshot)>)> {
    let dir = capture_dir();
    let dll = Path::new(DEFAULT_DLL);
    if !dir.is_dir() || !dll.exists() {
        eprintln!("skipping: capture or engine DLL not present");
        return None;
    }
    let defs = EngineDefs::load(dll).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path()).collect();
    files.sort();
    let snaps = files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "dat"))
        .map(|p| {
            let g = nrbf::decode(&std::fs::read(p).unwrap()).unwrap();
            (p.file_name().unwrap().to_string_lossy().to_string(), engine_map::map(&g))
        })
        .collect();
    Some((defs, snaps))
}

/// Small deterministic generator so a failure reproduces.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self, n: usize) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n.max(1)
    }
}

/// Replace hidden identities within each indistinguishable group:
/// invader cards within the same stage (including the never-seen removed
/// card), Fear cards (deck + earned) from the unseen Fear pool, Events and
/// Powers within their decks' unseen pools, the blight card with another of
/// the same healthy side and the same pool size.
fn permute_hidden(s: &EngineSnapshot, defs: &EngineDefs, rng: &mut Lcg) -> EngineSnapshot {
    let mut p = s.clone();

    // Invader deck: candidates per stage = rules pool − face-up cards.
    let face_up: BTreeSet<String> = s
        .explore_slot_revealed
        .iter()
        .chain(&s.build_slot)
        .chain(&s.ravage_slot)
        .chain(&s.invader_discard)
        .map(|c| c.id.clone())
        .collect();
    for stage in 1..=3u8 {
        let mut pool: Vec<String> = defs
            .invader_cards
            .keys()
            .filter(|id| defs.invader_stage(id) == Some(stage) && !face_up.contains(*id) && *id != "SaltDeposits2")
            .cloned()
            .collect();
        pool.sort();
        let slots: Vec<usize> = p
            .invader_deck
            .iter()
            .enumerate()
            .filter(|(_, c)| c.class == "InvaderCard" && defs.invader_stage(&c.id) == Some(stage))
            .map(|(i, _)| i)
            .collect();
        shuffle_into(&mut p.invader_deck, &slots, &mut pool, rng);
    }

    // Fear: deck + earned share one unseen pool.
    let seen_fear: BTreeSet<&str> = s.fear_discard.iter().map(|c| c.id.as_str()).collect();
    let mut fear_pool: Vec<String> = s.allowed_fear.iter().filter(|id| !seen_fear.contains(id.as_str())).cloned().collect();
    let mut all: Vec<EngineCard> = p.fear_deck.iter().chain(&p.fear_earned).cloned().collect();
    let slots: Vec<usize> = all.iter().enumerate().filter(|(_, c)| c.class == "FearCard").map(|(i, _)| i).collect();
    shuffle_into(&mut all, &slots, &mut fear_pool, rng);
    let (deck, earned) = all.split_at(p.fear_deck.len());
    p.fear_deck = deck.to_vec();
    p.fear_earned = earned.to_vec();

    // Events.
    let seen_ev: BTreeSet<&str> = s.event_discard.iter().map(|c| c.id.as_str()).collect();
    let mut ev_pool: Vec<String> = s.allowed_event.iter().filter(|id| !seen_ev.contains(id.as_str())).cloned().collect();
    let slots: Vec<usize> = (0..p.event_deck.len()).collect();
    shuffle_into(&mut p.event_deck, &slots, &mut ev_pool, rng);

    // Power decks: permute within the deck (the unseen pool is the deck's set).
    for deck in [&mut p.minor_deck, &mut p.major_deck] {
        let mut ids: Vec<String> = deck.iter().map(|c| c.id.clone()).collect();
        let slots: Vec<usize> = (0..deck.len()).collect();
        shuffle_into(deck, &slots, &mut ids, rng);
    }

    // Blight card, before the flip: same healthy side, same healthy count.
    if let Some(b) = p.blight_card.as_mut().filter(|b| !b.flipped) {
        let key = |id: &str| defs.blight_cards.get(id).map(|d| (d["healthyIslandBody"].to_string(), d["healthyIslandNumberPerStartingBoard"].to_string()));
        let mine = key(&b.id);
        let mut group: Vec<&String> = s.allowed_blight.iter().filter(|id| key(id) == mine).collect();
        group.sort();
        b.id = group[rng.next(group.len())].clone();
    }
    p
}

/// Assign identities to `slots` drawn without replacement from `pool`.
fn shuffle_into(cards: &mut [EngineCard], slots: &[usize], pool: &mut Vec<String>, rng: &mut Lcg) {
    for &i in slots {
        if pool.is_empty() {
            return;
        }
        let k = rng.next(pool.len());
        cards[i].id = pool.swap_remove(k);
    }
}

#[test]
fn every_capture_save_projects() {
    let Some((defs, snaps)) = load() else { return };
    assert!(snaps.len() > 300, "expected the full Test 1 capture, got {}", snaps.len());
    for (name, s) in &snaps {
        let p = projection::project(s, &defs);
        assert!(!p.lands.is_empty(), "{name}: no lands");
        assert!(p.warnings.iter().all(|w| !w.contains("mismatch")), "{name}: {:?}", p.warnings);
    }
}

#[test]
fn public_output_is_invariant_to_hidden_permutations() {
    let Some((defs, snaps)) = load() else { return };
    let mut rng = Lcg(0x5eed);
    let ids = |v: &[EngineCard]| v.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
    let (mut moved_invader, mut moved_fear, mut moved_blight) = (0, 0, 0);
    for (name, s) in &snaps {
        let base_value = serde_json::to_value(projection::project(s, &defs)).unwrap();
        let base = base_value.to_string();
        for round in 0..3 {
            let permuted = permute_hidden(s, &defs, &mut rng);
            moved_invader += (ids(&permuted.invader_deck) != ids(&s.invader_deck)) as usize;
            moved_fear += (ids(&permuted.fear_deck) != ids(&s.fear_deck) || ids(&permuted.fear_earned) != ids(&s.fear_earned)) as usize;
            moved_blight += (permuted.blight_card.as_ref().map(|b| &b.id) != s.blight_card.as_ref().map(|b| &b.id)) as usize;
            let out_value = serde_json::to_value(projection::project(&permuted, &defs)).unwrap();
            if base != out_value.to_string() {
                panic!("{name}: permutation {round} changed the public output at {}", first_diff(&base_value, &out_value, String::new()));
            }
        }
    }
    // The test only means something if the permutation actually moved hidden data.
    let n = snaps.len() * 3;
    assert!(moved_invader > n / 2 && moved_fear > n / 2 && moved_blight > n / 4, "permutation too weak: invader {moved_invader}, fear {moved_fear}, blight {moved_blight} of {n}");
}

#[test]
fn no_hidden_names_or_rng_fields_in_output() {
    let Some((defs, snaps)) = load() else { return };
    for (name, s) in &snaps {
        let value = serde_json::to_value(projection::project(s, &defs)).unwrap();
        let mut keys = Vec::new();
        collect_keys(&value, &mut keys);
        for key in &keys {
            let k = key.to_lowercase();
            assert!(!["rng", "seed", "inext", "deck_order"].iter().any(|b| k.contains(b)), "{name}: output field {key}");
        }
        // The rules pool lists every card by design (public card list).
        let mut value = value;
        value["invaders"]["pool"] = serde_json::Value::Null;
        let json = value.to_string();
        let face_up: BTreeSet<&str> = s
            .explore_slot_revealed
            .iter()
            .chain(&s.build_slot)
            .chain(&s.ravage_slot)
            .chain(&s.invader_discard)
            .chain(&s.fear_discard)
            .chain(&s.event_discard)
            .map(|c| c.id.as_str())
            .collect();
        let hidden = s.invader_deck.iter().chain(&s.fear_deck).chain(&s.fear_earned).chain(&s.event_deck);
        for c in hidden.filter(|c| c.class == "InvaderCard" || c.class == "FearCard" || c.class == "EventCard") {
            if !face_up.contains(c.id.as_str()) {
                assert!(!json.contains(&format!("\"{}\"", c.id)), "{name}: hidden card {} leaked", c.id);
            }
        }
        if let Some(b) = s.blight_card.as_ref().filter(|b| !b.flipped) {
            assert!(!json.contains(&b.id), "{name}: unflipped blight card leaked");
        }
    }
}

fn collect_keys(v: &serde_json::Value, out: &mut Vec<String>) {
    match v {
        serde_json::Value::Object(m) => {
            for (k, v) in m {
                out.push(k.clone());
                collect_keys(v, out);
            }
        }
        serde_json::Value::Array(a) => a.iter().for_each(|v| collect_keys(v, out)),
        _ => {}
    }
}

fn first_diff(a: &serde_json::Value, b: &serde_json::Value, path: String) -> String {
    use serde_json::Value::*;
    match (a, b) {
        (Object(x), Object(y)) => {
            for (k, v) in x {
                if y.get(k) != Some(v) {
                    return first_diff(v, y.get(k).unwrap_or(&Null), format!("{path}/{k}"));
                }
            }
            path
        }
        (Array(x), Array(y)) if x.len() == y.len() => {
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                if v != w {
                    return first_diff(v, w, format!("{path}/{i}"));
                }
            }
            path
        }
        _ => format!("{path}: {a} vs {b}"),
    }
}
