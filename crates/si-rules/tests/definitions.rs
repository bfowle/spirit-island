//! Every Power Card, innate and adversary level in the engine definitions, on random states
//! of every board, held to the rules' own laws (`si_rules::fuzz`). Skipped when the
//! definitions are not here (`si-steam-dump defs data/engine-defs` writes them). Every
//! printed sentence is held to the parser's law too: read as what it says.
//! `SI_FUZZ_CASES=3000` runs it deeper.

use std::path::Path;

#[test]
fn everything_in_the_definitions_keeps_its_laws() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
    if !dir.is_dir() {
        eprintln!("skipping: no engine definitions at {}", dir.display());
        return;
    }
    let defs = si_defs::Definitions::from_dir(&dir).expect("the engine definitions load");
    let cases = std::env::var("SI_FUZZ_CASES").ok().and_then(|v| v.parse().ok()).unwrap_or(60);
    let seed = std::env::var("SI_FUZZ_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    let r = si_rules::fuzz::fuzz_definitions(&defs, cases, seed);
    assert!(r.powers.len() > 300, "every Power Card and innate: {}", r.powers.len());
    assert!(r.adversaries.len() > 40, "every adversary at every level: {}", r.adversaries.len());
    let broken: Vec<String> = r
        .powers
        .iter()
        .flat_map(|p| p.broken.iter().map(move |(s, law, d)| format!("{} ({}): {law}: {d} [seed {s}]", p.title, p.id)))
        .chain(r.adversaries.iter().flat_map(|a| a.broken.iter().map(move |(s, law, d)| format!("{:?} {}: {law}: {d} [seed {s}]", a.adversary, a.level))))
        .collect();
    assert!(broken.is_empty(), "{}", broken.join("\n"));
    // every printed sentence read as what it says (si_defs::effects::sentence_holds)
    let misread: Vec<String> = r.stated.iter().map(|(who, law, d)| format!("{who}: {law}: {d}")).collect();
    assert!(misread.is_empty(), "printed sentences read as something else:\n{}", misread.join("\n"));
}

#[test]
fn an_innate_tier_that_says_instead_replaces_the_ones_before() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
    if !dir.is_dir() {
        return;
    }
    let defs = si_defs::Definitions::from_dir(&dir).expect("the engine definitions load");
    let beset = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == "BesetAndConfoundTheInvaders").expect("Many Minds' innate");
    use si_model::power::Effect;
    // past the tier's cost markers (Beset needs Beasts in the land), the Instead marker
    let instead = |t: &si_model::spirit::Tier| matches!(t.effects.iter().find(|e| !matches!(e, Effect::Requires { .. })), Some(Effect::Instead));
    assert!(!instead(&beset.tiers[0]), "the first tier stands alone");
    assert!(beset.tiers[1..].iter().all(instead), "\"Instead, 3 Fear and Defend 4\" and the rest replace");
    assert!(beset.tiers.iter().all(|t| t.effects.iter().any(|e| matches!(e, Effect::Requires { what, .. } if what == "Beast"))), "each tier needs its Beasts");
    let n = defs.spirits.values().flat_map(|s| s.innates.iter()).flat_map(|i| i.tiers.iter()).filter(|t| instead(t)).count();
    assert_eq!(n, 21, "every innate tier printed \"Instead\"");
}
