//! Forecast vs engine, turn by turn, over Brett's local Test 1 capture (not
//! in this public repo; skipped when absent). Every rule the forecast uses
//! was found or confirmed by a mismatch here: Strife, Disease, Wilds, Defend
//! per Dahan, −1 damage per Invader, and counterattack clears removing an
//! Explore source.

use si_steam::companion::Companion;
use si_steam::engine_defs::{DEFAULT_DLL, EngineDefs};
use si_steam::{engine_map, nrbf, projection};
use std::path::Path;

fn replay() -> Option<Companion> {
    let dir = std::env::var_os("SI_STEAM_CAPTURE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/steam-capture/2026-10-18-test1/saves"));
    let dll = Path::new(DEFAULT_DLL);
    if !dir.is_dir() || !dll.exists() {
        eprintln!("skipping: capture or engine DLL not present");
        return None;
    }
    let defs = EngineDefs::load(dll).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path()).collect();
    files.sort();
    let mut c = Companion::default();
    for f in &files {
        let g = nrbf::decode(&std::fs::read(f).unwrap()).unwrap();
        c.push(projection::project(&engine_map::map(&g), &defs));
    }
    Some(c)
}

#[test]
fn review_attributes_all_test1_blight() {
    let Some(c) = replay() else { return };
    let r = si_common::companion_stats::review::review(c.timeline(), &[]);
    assert_eq!(r.ending.as_deref(), Some("LoseTooMuchBlight"));
    let by = |cause: &str| r.blight.iter().filter(|b| b.cause == cause).map(|b| b.added).sum::<u32>();
    assert_eq!((by("Ravage"), by("cascade"), by("Event")), (6, 2, 5));
    assert_eq!(r.chains.len(), 6, "one invader chain per Ravage blight");
    assert_eq!(r.largest_jump.map(|j| j.0), Some(4));
}

#[test]
fn forecast_matches_engine_on_every_turn() {
    let Some(c) = replay() else { return };
    let checks = si_common::companion_stats::backtest::backtest(c.timeline());
    assert_eq!(checks.len(), 6, "Test 1 has six complete Invader Phases");
    let mismatches: Vec<String> = checks.iter().flat_map(|t| t.mismatches.iter().map(move |m| format!("Turn {}: {m}", t.turn))).collect();
    assert!(mismatches.is_empty(), "{mismatches:#?}");
    // Not vacuous: the game had Ravage blight, Builds, and Explores every turn.
    assert!(checks.iter().map(|t| t.ravage_blight_actual.len()).sum::<usize>() >= 6);
    assert!(checks.iter().all(|t| !t.builds_actual.is_empty() && !t.explores_actual.is_empty()));
}
