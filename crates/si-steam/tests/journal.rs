//! The save-journal reader on the fixture saves kept in the repo: every entry typed, the
//! record from setup, and its end on every land equal to the board the save holds.

use std::path::Path;

fn fixture(name: &str) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../si-hindsight/tests/fixtures/open-turn").join(name).join("saves");
    std::fs::read_dir(&dir).unwrap().next().unwrap().unwrap().path()
}

#[test]
fn the_growth_cut_replays_from_setup_and_lands_on_the_saved_board() {
    for name in ["growth", "fast"] {
        let save = fixture(name);
        let g = si_steam::nrbf::decode(&si_steam::source::read_save(&save).unwrap()).unwrap();
        let entries = si_steam::journal::entries(&g);
        assert!(entries.len() > 200, "{name}: {} entries", entries.len());
        let untyped: Vec<String> = entries.iter().filter_map(|e| match &e.kind { si_steam::journal::Kind::Other(c) => Some(c.clone()), _ => None }).collect();
        assert!(untyped.is_empty(), "{name}: entry classes not typed: {untyped:?}");
        // the printed pieces come from the board definitions; without them the end differs by exactly those
        let defs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
        let defs = defs_dir.is_dir().then(|| si_defs::Definitions::from_dir(&defs_dir).expect("the engine definitions load"));
        let rec = si_steam::journal::record(&g, defs.as_ref(), name);
        assert!(rec.meta.from_setup, "{name}: the journal starts at setup");
        assert_eq!(rec.meta.turns, 3, "{name}: the cut is in turn 3");
        assert_eq!(rec.meta.spirits, vec!["ShroudOfSilentMist".to_string()]);
        assert_eq!(rec.meta.boards, vec!["D".to_string()]);
        let snap = si_steam::engine_map::map(&g);
        let seed = si_steam::seed::from_snapshot(&snap, "fixture");
        let diffs = si_steam::journal::check(&rec, &seed);
        if defs.is_some() {
            assert!(diffs.is_empty(), "{name}: the record's end differs from the save's board:\n{}", diffs.join("\n"));
        }
    }
}
