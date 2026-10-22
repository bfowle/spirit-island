//! The turn being played, cut from the Shroud capture at T3 Growth (before any growth
//! decision) and at T3 Fast, each with the save the app wrote at that moment.

use std::path::Path;
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::live::{open_turn, score_plan};
use si_hindsight::{explore_candidates, Calibration};
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::{GameRecord, Replay};

fn defs() -> Definitions {
    Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/engine"))).unwrap()
}

fn cut(defs: &Definitions, name: &str) -> (Arc<GameRecord>, Seed) {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/open-turn")).join(name);
    let (lines, undo, source) = si_record::replay::load(&root).unwrap();
    let mut rec = Replay::new(Some(defs)).run(&lines, undo.as_deref(), &source);
    let save = std::fs::read_dir(root.join("saves")).unwrap().next().unwrap().unwrap().path();
    let bytes = si_steam::source::read_save(&save).unwrap();
    let snap = si_steam::engine_map::map(&si_steam::nrbf::decode(&bytes).unwrap());
    let seed = si_steam::seed::from_snapshot(&snap, &save.file_name().unwrap().to_string_lossy());
    seed.apply_meta(&mut rec.meta);
    (Arc::new(rec), seed)
}

fn full(defs: &Definitions) -> GameRecord {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/shroud-e392726411c9"));
    let (lines, undo, source) = si_record::replay::load(root).unwrap();
    Replay::new(Some(defs)).run(&lines, undo.as_deref(), &source)
}

#[test]
fn the_growth_cut_opens_turn_three_before_any_growth() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    assert_eq!(rec.meta.turns, 3);
    assert_eq!(seed.turn_index, 2);
    assert_eq!(seed.phase, Phase::SpiritPhase);
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).expect("an open turn");
    assert_eq!((open.turn, open.phase), (3, Phase::SpiritPhase));
    assert!(open.t.explore.is_empty(), "no Explore yet");
    assert_eq!(open.t.ravage, seed.ravage_slot);
    assert_eq!(open.t.build, seed.build_slot);
    assert!(!open.t.ravage.is_empty() && !open.t.build.is_empty(), "slots from the save");
    assert!(!open.t.before_fast.is_empty(), "the board now, from the save");
    assert!(open.s.growth_groups.is_empty(), "nothing taken");
    assert!(!open.s.hand_after.is_empty(), "the hand from the save");
}

#[test]
fn the_fast_cut_opens_turn_three_with_the_growth_taken() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "fast");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).expect("an open turn");
    assert_eq!((open.turn, open.phase), (3, Phase::FastPower));
    assert!(!open.s.growth_groups.is_empty(), "the growth from the journal");
    assert!(!open.t.before_fast.is_empty());
    assert!(open.t.explore.is_empty());
}

#[test]
fn an_open_turn_averages_over_the_stages_unseen_cards() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "fast");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let cands = explore_candidates(&rec, &defs, &open.deck, &open.t);
    assert!(cands.len() > 1, "a pool, not one card: {cands:?}");
    for c in &cands {
        assert!(!seed.invader_cards_seen.contains(&c[0]), "{} is face up already", c[0]);
        assert!(!open.t.ravage.contains(&c[0]) && !open.t.build.contains(&c[0]));
    }
}

#[test]
fn a_finished_turn_is_not_open() {
    let defs = defs();
    let (rec, mut seed) = cut(&defs, "fast");
    seed.turn_index = 0; // a save from another turn
    assert!(open_turn(rec, &defs, &seed, &Calibration::default()).is_none());
}

#[test]
fn a_growth_token_the_panel_does_not_offer_is_refused_with_the_groups() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec, &defs, &seed, &Calibration::default()).unwrap();
    let err = score_plan(&defs, &open, None, "G9 | ").unwrap_err();
    assert!(err.contains("G1") && err.contains("no growth position matches"), "{err}");
}

#[test]
fn doing_nothing_scores_on_every_open_tab() {
    let defs = defs();
    for name in ["growth", "fast"] {
        let (rec, seed) = cut(&defs, name);
        let open = open_turn(rec, &defs, &seed, &Calibration::default()).unwrap();
        let none = score_plan(&defs, &open, None, "").unwrap();
        // nothing beyond what the journal shows made: at Growth nothing, at Fast the plays so far
        for p in &none.plays {
            assert!(p.innate || open.s.cards_played.contains(&p.power), "{name}: {} is not a play the journal records", p.power);
        }
        if name == "growth" {
            assert!(none.plays.is_empty(), "{name}: nothing played");
        }
        assert!(!none.per_card.is_empty(), "{name}: scored under each Explore card");
        assert!(none.confidence.is_none());
        assert!(!none.outcome.after.is_empty(), "{name}: the board after");
    }
}

#[test]
fn a_growth_plan_is_scored_from_its_position_and_against_none() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let positions = si_hindsight::growth_positions(&rec, &defs, &open.s, &open.t, &si_metrics::reach::island_for(&rec, &defs));
    let first = positions.first().expect("the panel offers growth");
    let text = format!("{} |", first.growth.join(" "));
    let none = score_plan(&defs, &open, None, "").unwrap();
    let plan = score_plan(&defs, &open, Some(&none), &text).unwrap();
    assert_eq!(plan.growth, first.growth);
    assert!(plan.growth_value > 0.0, "slots, cards or Energy kept count");
    assert!(plan.confidence.is_some(), "vs none");
}

#[test]
fn the_fast_plan_of_the_actual_plays_agrees_with_the_review() {
    let defs = defs();
    let cal = Calibration::default();
    let mut full = full(&defs);
    let (rec, seed) = cut(&defs, "fast");
    // the full record has no sibling save in the fixture: take the level from the same save
    seed.apply_meta(&mut full.meta);
    let open = open_turn(rec.clone(), &defs, &seed, &cal).unwrap();
    // the same adversary level and deck offset on both sides
    let level_full = full.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(&full, &defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    assert_eq!(open.rules.level, level_full, "adversary level");
    assert_eq!(open.deck.offset, si_hindsight::deck::Deck::for_game(&full, &defs, level_full).offset, "deck offset");
    let opts = si_hindsight::ReviewOptions { actual_only: true, calibration: Some(cal.clone()), ..Default::default() };
    let review = si_hindsight::review_turns(&full, &defs, &opts, Some(&[3]), &|_| {});
    let actual = &review.turns[0].actual;
    // the actual plays as a plan writes them: in the order played, with their gathers and
    // pushes, Slow ones without a land so both paths choose them after the steps
    let text: String = actual.plays_text(&|id| id.to_string());
    let plan = score_plan(&defs, &open, None, &text).unwrap();
    assert!((plan.expected - actual.expected).abs() < 0.01, "the expectation terms: plan {} (Event and Fear cards as the open turn counts them) vs review {}", plan.expected, actual.expected);
    let (a, b) = (plan.informed - plan.expected, actual.informed - actual.expected);
    let words = |l: &si_hindsight::Line| l.plays_notation(&|id| id.to_string(), true, true);
    assert!((a - b).abs() < 0.01, "informed less the expectation: plan {a:.3} vs review {b:.3}\nplan {:?}\nreview {:?}\ntext {text}\nplan {}\nreview {}\nplan notes {:?}", plan.per_card, actual.per_card, words(&plan), words(actual), plan.outcome.notes);
}

#[test]
fn the_review_skips_the_open_turn() {
    let defs = defs();
    let (rec, _) = cut(&defs, "fast");
    assert_eq!(si_hindsight::closed_turns(&rec), vec![1, 2]);
    let opts = si_hindsight::ReviewOptions { growth: false, ..Default::default() };
    let review = si_hindsight::review_turns(&rec, &defs, &opts, Some(&[3]), &|_| {});
    assert!(review.turns.is_empty(), "turn 3 has no Invader steps yet");
}

#[test]
fn unseen_minors_exclude_what_the_journal_has_shown() {
    use si_model::power::PowerKind;
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let unseen = si_hindsight::live::unseen_cards(&rec, &defs, PowerKind::Minor);
    let total = defs.cards.values().filter(|c| c.kind == PowerKind::Minor).count();
    // the engine fixture carries a handful of Minors; none of them show in the Shroud journal
    assert!(!unseen.is_empty() && unseen.len() <= total, "{} of {total}", unseen.len());
    for s in &rec.spirits {
        for c in s.hand_after.iter().chain(s.cards_gained.iter()).chain(s.offered.iter()) {
            assert!(!unseen.iter().any(|u| u.id == *c), "{c} has been seen");
        }
    }
    let land: si_model::island::LandId = open.t.before_fast.keys().next().unwrap().clone();
    let count = si_hindsight::live::draw_count(&defs, &open, PowerKind::Minor, &land);
    assert!(count.playable <= count.unseen);
}

#[test]
fn the_ravage_thresholds_name_the_damage_and_the_defend_that_saves() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "fast");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let island = si_rules::verify::island_for(&rec, &defs, &open.t.before_fast);
    let mut seen_damage = false;
    for l in island.lands.values().filter(|l| l.c.invaders() > 0) {
        let hit = open.t.ravage.iter().filter_map(|c| defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|t| c.matches(t, l.coastal)));
        if !hit {
            continue;
        }
        let th = si_hindsight::live::ravage_thresholds(&island, &defs, &open.t.ravage, &open.rules, &l.id);
        assert!(th.damage > 0, "{}: Invaders ravage for something", l.id);
        if let Some(d) = th.defend_keeps_land {
            assert!(d <= th.damage, "{}: Defend {d} against {} damage", l.id, th.damage);
        }
        seen_damage = true;
    }
    assert!(seen_damage, "the T3 Ravage card names a land with Invaders");
}
