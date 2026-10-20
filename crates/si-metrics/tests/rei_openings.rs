//! Rei's openings, written as growth and play sequences, run through the
//! track rows and the build profile. Expected labels are Rei's own
//! (research_notes/rei/*.md, read 2026-10-20). Slot ids and starting hands
//! come from the engine definitions, so the x/ycp rows are the real tracks.

use std::path::Path;

use si_defs::Definitions;
use si_metrics::profile::{profile, Scaling};
use si_metrics::track::{rows, spike_turn};
use si_model::island::LandId;
use si_record::{GameRecord, SpiritTurn};

const V: &str = "VengeanceAsABurningPlague";
const MM: &str = "ManyMindsMoveAsOne";
const SH: &str = "ShroudOfSilentMist";

fn defs() -> Definitions {
    Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-defs/tests/fixtures/engine"))).unwrap()
}

/// One opening turn: growth group(s), track slots revealed (suffixes such as
/// `Energy2`), cards gained, cards played, whether it was a Reclaim.
struct T(&'static [u8], &'static [&'static str], u32, u32, bool);

fn opening(defs: &Definitions, spirit: &str, turns: &[T]) -> GameRecord {
    let panel = &defs.spirits[spirit];
    let mut rec = GameRecord::default();
    rec.meta.spirits = vec![spirit.to_string()];
    rec.meta.turns = turns.len() as u32;
    let mut hand: Vec<String> = panel.uniques.clone();
    let mut discard: Vec<String> = vec![];
    let mut revealed = panel.starting_revealed();
    let mut gained_n = 0;
    for (i, t) in turns.iter().enumerate() {
        let turn = i as u32 + 1;
        if t.4 {
            hand.append(&mut discard);
        }
        let mut gained = vec![];
        for _ in 0..t.2 {
            gained_n += 1;
            let c = format!("Minor{gained_n}");
            hand.push(c.clone());
            gained.push(c);
        }
        let mut placed = vec![];
        for slot in t.1 {
            let id = format!("{spirit}{slot}");
            assert!(panel.tracks.iter().any(|n| n.id == id), "no slot {id}");
            revealed.insert(id.clone());
            placed.push((Some(id), Some(LandId::new("A", 1))));
        }
        let st = panel.track_state(&revealed);
        let played: Vec<String> = hand.drain(..(t.3 as usize).min(hand.len())).collect();
        discard.extend(played.iter().cloned());
        rec.spirits.push(SpiritTurn {
            turn,
            spirit: spirit.to_string(),
            growth_groups: t.0.to_vec(),
            presence_placed: placed,
            income: Some(st.income),
            plays: Some(st.plays),
            cards_played: played,
            cards_gained: gained,
            reclaimed: t.4,
            hand_after: hand.clone(),
            discard_after: discard.clone(),
            ..Default::default()
        });
    }
    rec
}

fn label(defs: &Definitions, spirit: &str, turns: &[T]) -> (Scaling, Vec<String>, u32, u32) {
    let rec = opening(defs, spirit, turns);
    let rows = rows(&rec, Some(defs), spirit);
    let p = profile(Some(defs), spirit, &rows, &[], 0);
    let xy: Vec<String> = rows.iter().map(|r| format!("{}/{}cp", r.income.unwrap(), r.plays.unwrap())).collect();
    (p.scaling, xy, p.presence_by_t3.0, p.growth_cards_by_t3)
}

#[test]
fn vengeance_builds_carry_reis_labels() {
    let d = defs();
    // Fast Minors: G3 (minor + top), G2 (bottom ×2), G2 (bottom ×2); Reclaim T4.
    let fast_minors = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["CardPlays2", "CardPlays3"], 0, 2, false), T(&[2], &["CardPlays4", "CardPlays5"], 0, 2, false), T(&[1], &[], 1, 2, true)];
    let (s, xy, placed, cards) = label(&d, V, &fast_minors);
    assert_eq!((s, placed, cards), (Scaling::Fast, 5, 1));
    assert_eq!(xy[..3], ["2/1cp", "2/2cp", "2/3cp"], "3 plays unlocked on turn 3, before the reclaim");
    let three_plays = si_metrics::spikes::spikes_for(V).into_iter().find(|s| s.plays == 3 && s.income == 0).unwrap();
    assert_eq!(spike_turn(&rows(&opening(&d, V, &fast_minors), Some(&d), V), &three_plays), Some(3));

    // Minus Minors (1): keep G2 on T2, take a second minor with G3 on T3, 3 plays on T4.
    let minus_minors = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["CardPlays2", "CardPlays3"], 0, 2, false), T(&[3], &["CardPlays4"], 1, 2, false), T(&[2], &["CardPlays5", "CardPlays6"], 0, 3, false)];
    let (s, xy, placed, cards) = label(&d, V, &minus_minors);
    assert_eq!((s, placed, cards), (Scaling::Minus, 4, 2));
    assert_eq!(xy[3], "2/3cp", "3 plays one turn later than the Fast path");

    // Minus Minors (2), the "unfortunate" path: Animal on T3, then G2 on T4.
    let unfortunate = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["CardPlays2", "CardPlays3"], 0, 2, false), T(&[3], &["Energy3"], 1, 2, false), T(&[2], &["CardPlays4", "CardPlays5"], 0, 3, false)];
    assert_eq!(label(&d, V, &unfortunate).0, Scaling::Minus);

    // Fast Majors: G2 (top ×2), G2 (top + bottom), G3 (major + top).
    let fast_majors = [T(&[2], &["Energy2", "Energy3"], 0, 1, false), T(&[2], &["Energy4", "CardPlays2"], 0, 2, false), T(&[3], &["Energy5"], 1, 2, false)];
    let (s, xy, _, _) = label(&d, V, &fast_majors);
    assert_eq!(s, Scaling::Fast);
    assert_eq!(xy, ["2/1cp", "3/2cp", "4/2cp"], "the 4 Energy unlock on turn 3");

    // Minus Majors: G3 (minor + top), G2 (Animal + 2 plays), G3 (major + 3 Energy).
    let minus_majors = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["Energy3", "CardPlays2"], 0, 2, false), T(&[3], &["Energy4"], 1, 2, false)];
    let (s, xy, _, _) = label(&d, V, &minus_majors);
    assert_eq!(s, Scaling::Minus);
    assert_eq!(xy[2], "3/2cp");
}

#[test]
fn many_minds_builds_carry_reis_labels() {
    let d = defs();
    // The Drafter: G2 (top + bottom), G2 (top ×2), G2 (bottom ×2 + pay 2 for a minor); Reclaim T4.
    let drafter = [T(&[2], &["Energy2", "CardPlays2"], 0, 1, false), T(&[2], &["Energy3", "Energy4"], 0, 2, false), T(&[2], &["CardPlays3", "CardPlays4"], 1, 2, false), T(&[1], &[], 1, 3, true)];
    let (s, xy, placed, cards) = label(&d, MM, &drafter);
    assert_eq!((s, placed, cards), (Scaling::Fast, 6, 0), "the pay-2 minor is a slot, not a growth trade");
    assert_eq!(xy[..3], ["1/2cp", "2/2cp", "2/3cp"], "Rei: 2e/3 card plays by Turn 4");

    // The Extra Friend: G3 (top + Beast), G2 (top + bottom), G2 (bottom ×2 + pay 2); Reclaim T4.
    let extra_friend = [T(&[3], &["Energy2"], 0, 1, false), T(&[2], &["Energy3", "CardPlays2"], 0, 2, false), T(&[2], &["CardPlays3", "CardPlays4"], 1, 2, false), T(&[1], &[], 1, 3, true)];
    let (s, xy, placed, _) = label(&d, MM, &extra_friend);
    assert_eq!((s, placed), (Scaling::Minus, 5), "one growth took a Beast and a single presence");
    assert_eq!(xy[..3], ["1/1cp", "1/2cp", "1/3cp"], "Energy a turn behind the Drafter");

    // The Hasteful: G2, G2 (bottom ×2 → 3 plays on T2), Reclaim T3 with the hand run dry.
    let hasteful = [T(&[2], &["Energy2", "CardPlays2"], 0, 2, false), T(&[2], &["CardPlays3", "CardPlays4"], 0, 3, false), T(&[1], &[], 1, 3, true), T(&[2], &["Energy3", "Energy4"], 1, 3, false)];
    let (s, xy, _, _) = label(&d, MM, &hasteful);
    assert_eq!(s, Scaling::Problematic, "the T3 Reclaim was forced: 0 cards in hand against 3 plays");
    assert_eq!(xy[1], "1/3cp");

    // Big Birb: G2 ×3 up the Energy track; Reclaim T4.
    let big_birb = [T(&[2], &["Energy2", "CardPlays2"], 0, 1, false), T(&[2], &["Energy3", "Energy4"], 0, 2, false), T(&[2], &["Energy5", "Energy6"], 0, 2, false), T(&[1], &[], 1, 2, true)];
    let (s, xy, _, _) = label(&d, MM, &big_birb);
    assert_eq!(s, Scaling::Fast);
    assert_eq!(xy[2], "3/2cp", "the 3 Energy rush");

    // The Ponderer: G3 (top + Beast), G2 (top + bottom), G2 (top ×2); Reclaim T4.
    let ponderer = [T(&[3], &["Energy2"], 0, 1, false), T(&[2], &["Energy3", "CardPlays2"], 0, 2, false), T(&[2], &["Energy4", "Energy5"], 0, 2, false), T(&[1], &[], 1, 2, true)];
    let (s, xy, _, _) = label(&d, MM, &ponderer);
    assert_eq!(s, Scaling::Minus);
    assert_eq!(xy[2], "2/2cp");
}

#[test]
fn shroud_builds_carry_reis_labels() {
    let d = defs();
    // The Unleashed: G2 (top + bottom), G3 (minor + bottom), then G2 (bottom ×2) …
    let unleashed = [T(&[2], &["Energy2", "CardPlays2"], 0, 2, false), T(&[3], &["CardPlays3"], 1, 1, false), T(&[2], &["CardPlays4", "CardPlays5"], 0, 2, false)];
    let (s, xy, placed, cards) = label(&d, SH, &unleashed);
    assert_eq!((s, placed, cards), (Scaling::Fast, 5, 1));
    assert_eq!(xy, ["1/2cp", "1/2cp", "1/3cp"], "Rei: your power spike at 1/3cp, pre-reclaim");
    let spike = si_metrics::spikes::spikes_for(SH).into_iter().find(|s| s.plays == 3).unwrap();
    assert_eq!(spike_turn(&rows(&opening(&d, SH, &unleashed), Some(&d), SH), &spike), Some(3));

    // … or its Reclaim fallback on T3. Not forced (2 cards in hand against 2
    // plays), so it is measured on the three slots it revealed: Minus. Rei
    // files the build as Fast on the strength of the G2 path.
    let unleashed_reclaim = [T(&[2], &["Energy2", "CardPlays2"], 0, 2, false), T(&[3], &["CardPlays3"], 1, 1, false), T(&[1], &[], 1, 2, true), T(&[2], &["CardPlays4", "CardPlays5"], 0, 2, false)];
    let (s, xy, _, _) = label(&d, SH, &unleashed_reclaim);
    assert_eq!(s, Scaling::Minus);
    assert_eq!(xy[3], "1/3cp", "3 plays right after the reclaim");

    // The Gambler: G3 (minor + top), G2 (top + bottom), G3 (minor + top), G3 …
    let gambler = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["Energy3", "CardPlays2"], 0, 2, false), T(&[3], &["Energy4"], 1, 2, false), T(&[3], &["CardPlays3"], 1, 2, false)];
    let (s, xy, placed, cards) = label(&d, SH, &gambler);
    assert_eq!((s, placed, cards), (Scaling::Minus, 4, 2));
    assert_eq!(xy[2], "2/2cp", "the 2 Energy unlock before 3 plays");

    // The Slow and Silent: G3, G2, G3 (major).
    let slow_and_silent = [T(&[3], &["Energy2"], 1, 1, false), T(&[2], &["Energy3", "CardPlays2"], 0, 2, false), T(&[3], &["Energy4"], 1, 2, false)];
    assert_eq!(label(&d, SH, &slow_and_silent).0, Scaling::Minus);
}

#[test]
fn a_forced_reclaim_is_read_from_the_hand() {
    let d = defs();
    // Shroud G2 ×2 then Reclaim with an empty hand: Problematic, not Minus.
    let dry = [T(&[2], &["Energy2", "CardPlays2"], 0, 2, false), T(&[2], &["CardPlays3", "CardPlays4"], 0, 2, false), T(&[1], &[], 1, 2, true)];
    let rec = opening(&d, SH, &dry);
    let rows = rows(&rec, Some(&d), SH);
    assert_eq!(rows[2].hand_before, 0);
    assert!(rows[2].forced_reclaim);
    assert_eq!(profile(Some(&d), SH, &rows, &[], 0).scaling, Scaling::Problematic);
}

#[test]
fn track_steps_read_the_printed_tracks() {
    use si_metrics::growth::{Growth, TrackSteps};
    let d = defs();
    let v = TrackSteps::income(&d.spirits[V]);
    assert_eq!(v.values, vec![1, 2, 2, 3, 4], "Vengeance: 1, 2, Animal, 3, 4");
    let p = TrackSteps::plays(&d.spirits[V]);
    assert_eq!(p.values, vec![1, 2, 2, 2, 3, 3, 4], "Vengeance: 1, 2, Fire, 2, 3, 3, 4");
    assert_eq!(TrackSteps::plays(&d.spirits[MM]).values, vec![1, 2, 2, 3, 3, 4, 5]);
    assert_eq!(v.value(10), 4.0, "past the end the track stays at its last value");
}
