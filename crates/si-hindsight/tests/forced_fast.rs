//! An innate printed Slow is taken Fast only through its "may be Fast" tier: `F:` in a
//! typed line says so, the enumerator offers it both ways when the tier is reached, and
//! a line that does not reach the tier is refused with the reason.

use std::path::Path;

use si_defs::Definitions;
use si_model::element::{Element, ElementSet};

fn defs() -> Option<Definitions> {
    let d = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
    d.is_dir().then(|| Definitions::from_dir(&d).expect("the engine definitions load"))
}

fn air(n: usize) -> ElementSet {
    ElementSet::from_list(std::iter::repeat(Element::parse("Air").unwrap()).take(n))
}

#[test]
fn thunderspeakers_assault_is_fast_only_with_four_air() {
    let Some(defs) = defs() else { return };
    let plain = si_hindsight::parse_plays(&defs, "LFA>A4").unwrap();
    assert!(!plain[0].fast && !plain[0].forced && plain[0].innate, "{plain:?}");
    let forced = si_hindsight::parse_plays(&defs, "F:LFA>A4").unwrap();
    assert!(forced[0].fast && forced[0].forced && forced[0].innate, "{forced:?}");
    // a Slow card taken Fast reads; the line must hold something that makes it Fast
    let vt = si_hindsight::parse_plays(&defs, "F:VT>A5").unwrap();
    assert!(vt[0].fast && vt[0].forced, "{vt:?}");
    let refused = si_hindsight::forced_speed_error(&defs, "Thunderspeaker", &vt, &air(4)).expect("nothing makes Voice of Thunder Fast");
    assert!(refused.contains("nothing in this line makes a Power Fast"), "{refused}");
    assert!(si_hindsight::parse_plays(&defs, "F:SA>A5").is_ok(), "a Fast card with F: is only said out loud");
    let lfa = defs.spirits["Thunderspeaker"].innates.iter().find(|i| i.id == "LeadTheFuriousAssault").unwrap();
    assert!(si_hindsight::may_be_fast(lfa, &air(4)));
    assert!(!si_hindsight::may_be_fast(lfa, &air(3)));
    let refused = si_hindsight::forced_speed_error(&defs, "Thunderspeaker", &forced, &air(3)).expect("three Air is not enough");
    assert!(refused.contains("Lead the Furious Assault") && refused.contains("Slow as printed"), "{refused}");
    assert!(si_hindsight::forced_speed_error(&defs, "Thunderspeaker", &forced, &air(4)).is_none());
    // plain text carries the mark; the screen leaves it to color
    let code = |id: &str| if id == "LeadTheFuriousAssault" { "LFA".to_string() } else { id.to_string() };
    assert_eq!(si_hindsight::Line::play_word(&forced[0], &[], &code, true), "F:LFA>A4");
    assert_eq!(si_hindsight::Line::play_word(&forced[0], &[], &code, false), "LFA>A4", "on screen the color says it");
    assert_eq!(si_hindsight::Line::play_word(&plain[0], &[], &code, true), "LFA>A4");
}

#[test]
fn a_gather_and_push_bracket_reads_both_ways() {
    let Some(defs) = defs() else { return };
    let plays = si_hindsight::parse_plays(&defs, "GW>A1[<A3 <A5 >A4 >A4] WW>A7").unwrap();
    assert_eq!(plays.len(), 2);
    let gw = &plays[0];
    assert_eq!(gw.target, Some("A1".parse().unwrap()));
    // a bare move of a Power that moves Dahan is a Dahan's
    let dahan = Some(si_model::power::PieceFilter::Piece(si_model::island::PieceKind::Dahan));
    let m = |from: &str, to: &str, n: u32| si_hindsight::Move { from: from.parse().unwrap(), to: to.parse().unwrap(), piece: dahan.clone(), n };
    assert_eq!(gw.moves, vec![m("A3", "A1", 1), m("A5", "A1", 1), m("A1", "A4", 2)]);
    let code = |id: &str| if id == "GatherTheWarriors" { "GW".to_string() } else { id.to_string() };
    assert_eq!(si_hindsight::Line::play_word(gw, &gw.moves, &code, false), "GW>A1[<A3,<A5,>A4x2]", "a count says how many");
    let commas = si_hindsight::parse_plays(&defs, "GW>A1[<A3,<A5,>A4x2]").unwrap();
    assert_eq!(commas[0].moves, gw.moves, "commas and spaces both separate the moves");
    assert_eq!(si_hindsight::Line::play_word(gw, &[m("A3", "A1", 4)], &code, false), "GW>A1[<A3x4]");
    assert!(si_hindsight::parse_plays(&defs, "GW[<A3]").is_err(), "a bracket needs the Power's land");
    assert!(si_hindsight::parse_plays(&defs, "GW>A1[A3]").is_err(), "a move is < or >");
}

#[test]
fn a_bracket_names_kinds_kills_and_nothing() {
    let Some(defs) = defs() else { return };
    let code = |id: &str| id.to_string();
    // a kind named after a colon; a Dahan bare
    let p = si_hindsight::parse_plays(&defs, "SuddenAmbush>A5[<A4,<A6:E]").unwrap();
    let kinds: Vec<Option<String>> = p[0].moves.iter().map(|m| m.piece.as_ref().and_then(si_rules::effects::piece_code).map(String::from)).collect();
    assert_eq!(kinds, vec![Some("Dh".to_string()), Some("E".to_string())]);
    // kills: in its land, in another, counted; `-` alone says none
    let k = si_hindsight::parse_plays(&defs, "VoiceOfThunder>A5[-T,-A6:Ex2]").unwrap();
    let kills = k[0].kills.clone().unwrap();
    assert_eq!(kills.len(), 3);
    assert_eq!(si_hindsight::Line::play_word(&k[0], &[], &code, true), "VoiceOfThunder>A5[-T,-A6:Ex2]");
    let none = si_hindsight::parse_plays(&defs, "VoiceOfThunder>A5[-]").unwrap();
    assert_eq!(none[0].kills, Some(vec![]));
    assert_eq!(si_hindsight::Line::play_word(&none[0], &[], &code, true), "VoiceOfThunder>A5[-]");
    // `[]`: asked, and nothing moved
    let empty = si_hindsight::parse_plays(&defs, "GatherTheWarriors>A1[]").unwrap();
    assert!(empty[0].exact && empty[0].moves.is_empty());
    assert_eq!(si_hindsight::Line::play_word(&empty[0], &[], &code, true), "GatherTheWarriors>A1[]");
    assert!(si_hindsight::parse_plays(&defs, "VoiceOfThunder>A5[-Q]").is_err(), "Q is no piece");
}
