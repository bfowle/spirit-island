//! Golden game: Brett's Shroud of Silent Mist win on board D (capture
//! live-e392726411c9, 2026-10-20). Every number here was checked against the
//! game's own saves in docs/superpowers/reviews/2026-10-19/shroud-game-e392726411c9.md.

use std::path::Path;

use si_defs::Definitions;
use si_model::Ending;
use si_record::Replay;

fn record() -> si_record::GameRecord {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"));
    let defs = Definitions::from_dir(&root.join("engine")).unwrap();
    let (lines, undo, source) = si_record::replay::load(&root.join("shroud-e392726411c9")).unwrap();
    Replay::new(Some(&defs)).run(&lines, undo.as_deref(), &source)
}

#[test]
fn the_game_its_turns_and_its_ending() {
    let r = record();
    assert!(r.meta.from_setup);
    assert_eq!(r.meta.spirits, vec!["ShroudOfSilentMist"]);
    assert_eq!(r.meta.boards, vec!["D"]);
    assert_eq!(r.meta.turns, 6);
    assert_eq!(r.meta.ending, Some(Ending::WinInvadersDestroyed));
    // 20 undo reloads, all placed from the capture's save-write clock
    assert_eq!(r.coverage.undo_reloads, 20);
    assert_eq!(r.coverage.undo_unresolved, 0);
    assert_eq!(r.coverage.copies, 6);
}

#[test]
fn blight_fear_and_invader_cards_per_turn() {
    let r = record();
    let blight: Vec<(u32, u32)> = (1..=6).map(|t| r.turn(t).map(|x| (x.blight_added, x.blight_cascaded)).unwrap()).collect();
    assert_eq!(blight, vec![(0, 0), (1, 0), (2, 0), (1, 0), (1, 1), (0, 0)]);
    let cards: Vec<u32> = (1..=6).map(|t| r.turn(t).unwrap().fear_cards_earned).collect();
    assert_eq!(cards.iter().sum::<u32>(), 6);
    assert_eq!(r.turn(6).unwrap().terror_after, 3);
    let t3 = r.turn(3).unwrap();
    assert_eq!((t3.ravage.as_slice(), t3.build.as_slice(), t3.explore.as_slice()), (&["Jungle1".to_string()][..], &["Wetland1".to_string()][..], &["Coastal2".to_string()][..]));
    assert_eq!(r.turn(1).unwrap().board_after.towns, 3, "board D's printed Town in land 7 plus two Towns built on turn 1");
    assert_eq!(r.setup.get(&"D2".parse().unwrap()).map(|c| (c.cities, c.dahan)), Some((1, 1)), "board D land 2 starts with a City and a Dahan");
}

#[test]
fn shrouds_growth_track_and_cards() {
    let r = record();
    let st = |t| r.spirit_turn("ShroudOfSilentMist", t).unwrap();
    let groups: Vec<Vec<u8>> = (1..=6).map(|t| st(t).growth_groups.clone()).collect();
    assert_eq!(groups, vec![vec![2], vec![2], vec![3], vec![1], vec![3], vec![3]]);
    assert_eq!((st(1).income, st(1).plays), (Some(1), Some(1)), "double top on turn 1: 1/1cp");
    assert_eq!((st(2).income, st(2).plays), (Some(2), Some(2)));
    assert_eq!(st(3).cards_gained, vec!["AnimatedWrackroot"]);
    assert!(st(4).reclaimed);
    assert_eq!(st(6).cards_gained, vec!["Tsunami"]);
    assert_eq!(st(6).cards_forgotten, vec!["SavageTransformation"]);
    assert_eq!(st(6).cards_played, vec!["Tsunami"]);
    assert_eq!(st(1).cards_played.len() + st(2).cards_played.len(), 3);
    assert_eq!(st(1).presence_placed.len(), 2);
}

#[test]
fn lands_carry_visits_and_touches() {
    let r = record();
    let d2 = r.lands.get(&"D2".parse().unwrap()).expect("Jungle D2 ledger");
    assert!(d2.blight.iter().any(|(t, _, _, _)| *t == 3), "D2 blighted on turn 3");
    assert!(d2.blight.iter().any(|(t, _, _, c)| *t == 5 && !*c), "D2 blighted again on turn 5");
    assert!(d2.touches.iter().any(|t| t.turn == 6 && t.by == "Tsunami"), "Tsunami on D2 on turn 6");
    let d4 = r.lands.get(&"D4".parse().unwrap()).unwrap();
    assert!(d4.blight.iter().any(|(t, _, _, c)| *t == 5 && *c), "the turn-5 cascade reached D4");
    let ravages: usize = r.lands.values().flat_map(|l| l.visits.iter()).filter(|v| v.step == Some(si_model::event::InvaderStep::Ravage)).count();
    assert!(ravages >= 5, "{ravages}");
}
