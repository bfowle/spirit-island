//! The capture corpus as tests: every game recorded so far, across spirits (Thunderspeaker,
//! Shroud, Rampant Green, Many Minds, Shifting Memory, Volcano, Ocean, Lightning and River,
//! Vengeance, Lure, Starlight, Trickster, Fractured Days) and adversaries at their levels
//! (none, England 3 and 4, Russia 5, Brandenburg-Prussia 2, Sweden 4, Habsburg Mining 4), run
//! through the commands the screens use. Skipped when SI_DEFS_DIR or the captures are absent.
//!
//! What they hold the simulator to:
//! - every capture's save journal replays to the save's own board (`si journal --check`);
//! - a lost game loses on its last turn for the reason it was lost, a won game never loses;
//! - the simulator does not get worse on any capture: `si verify` mismatches and each turn's
//!   simulator error (the actual line against the record at the Slow phase) stay at or under
//!   the blessed baseline, and a turn replayed exactly stays exact
//!   (`SI_CORPUS_BLESS=1` rewrites `tests/fixtures/corpus/baseline.json` after a change that
//!   is meant to move them);
//! - Thunderspeaker's T5 against England 4 replays exactly as the journal has it;
//! - England's capital: the review finds the T5 line that does not lose, and a line that
//!   walks into the seventh building reads LOSES.

use std::path::{Path, PathBuf};
use std::process::Command;

fn capture_root() -> PathBuf {
    PathBuf::from(std::env::var("SI_CAPTURE_DIR").unwrap_or_else(|_| "/home/brett/repos/spirit-island/data/steam-capture".into()))
}

fn ready() -> bool {
    if std::env::var_os("SI_DEFS_DIR").is_none() {
        eprintln!("skipping: SI_DEFS_DIR not set");
        return false;
    }
    if !capture_root().join("live-3a72b583b5da").join("saves").is_dir() {
        eprintln!("skipping: the captures are not present");
        return false;
    }
    true
}

/// Every capture with saves, by name.
fn captures() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(capture_root())
        .map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("live-")) && p.join("saves").is_dir()).collect())
        .unwrap_or_default();
    out.sort();
    out
}

fn name(p: &Path) -> String {
    p.file_name().unwrap().to_string_lossy().to_string()
}

fn si(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_si")).args(args).env("NO_COLOR", "1").output().expect("si runs");
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn json(args: &[&str]) -> serde_json::Value {
    let text = si(args);
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("si {} gave no JSON ({e}): {}", args.join(" "), &text[..text.len().min(400)]))
}

/// The actual lines of every closed turn of a capture: (turn, outcome, model error).
fn actual_turns(cap: &Path) -> Vec<(u64, serde_json::Value, Option<f64>)> {
    let r = json(&["hindsight", cap.to_str().unwrap(), "--actual", "--json"]);
    r["turns"].as_array().cloned().unwrap_or_default().into_iter().map(|t| (t["turn"].as_u64().unwrap_or(0), t["actual"]["outcome"].clone(), t["sample"]["model_error"].as_f64())).collect()
}

fn ending(cap: &Path) -> Option<String> {
    let r = json(&["replay", cap.to_str().unwrap(), "--json"]);
    r["meta"]["ending"].as_str().map(String::from)
}

#[test]
fn every_capture_journal_matches_its_save() {
    if !ready() {
        return;
    }
    let bad: Vec<String> = captures().iter().filter(|c| !si(&["journal", c.to_str().unwrap(), "--check"]).contains("matches the save's board on every land")).map(|c| name(c)).collect();
    assert!(bad.is_empty(), "the save journal does not replay to the save's board: {bad:?}");
}

/// Known simulator gaps that make a won game's actual line read as lost, each with the
/// reason; a new one fails the test.
const KNOWN_FALSE_LOSSES: &[(&str, u64, &str)] = &[];

#[test]
fn lost_games_lose_on_their_last_turn_and_won_games_never_do() {
    if !ready() {
        return;
    }
    let mut problems: Vec<String> = Vec::new();
    for c in captures() {
        let Some(end) = ending(&c) else { continue };
        let turns = actual_turns(&c);
        let loses: Vec<(u64, String)> = turns.iter().filter_map(|(t, o, _)| o["loses"].as_str().map(|r| (*t, r.to_string()))).collect();
        if end.starts_with("Win") {
            let unknown: Vec<&(u64, String)> = loses.iter().filter(|(t, _)| !KNOWN_FALSE_LOSSES.iter().any(|(n, kt, _)| *n == name(&c) && kt == t)).collect();
            if !unknown.is_empty() {
                problems.push(format!("{} won, yet the review says it lost: {unknown:?}", name(&c)));
            }
            continue;
        }
        let last = turns.iter().map(|(t, _, _)| *t).max().unwrap_or(0);
        match loses.iter().find(|(t, _)| *t == last) {
            None => problems.push(format!("{} ended {end}, but its last turn (T{last}) does not lose", name(&c))),
            Some((_, r)) => {
                // the reason it was lost; a Blight loss shows as the Blight Card when the
                // capture kept that turn's Blight pool (Russia's T8 met its Hunters too)
                let fits = match end.as_str() {
                    "LoseAdversary" => !r.contains("Blight Card"),
                    "LoseTooMuchBlight" => r.contains("Blight Card") || name(&c) == "live-27696743c529",
                    _ => true,
                };
                if !fits {
                    problems.push(format!("{} ended {end}, the review says: {r}", name(&c)));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The baseline: per capture, `si verify`'s mismatch count and each turn's simulator error
/// (`{"live-...": {"verify": 4, "errors": {"T1": 0.0, ...}}}`).
fn baseline_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/corpus/baseline.json")
}

#[test]
fn the_simulator_does_not_get_worse_on_any_capture() {
    if !ready() {
        return;
    }
    let mut now = serde_json::Map::new();
    for c in captures() {
        let verify = si(&["verify", c.to_str().unwrap()]).lines().filter(|l| l.starts_with('T') && l.chars().nth(1).is_some_and(|d| d.is_ascii_digit())).count();
        let turns = actual_turns(&c);
        let errors: serde_json::Map<String, serde_json::Value> = turns.iter().filter_map(|(t, _, e)| e.map(|e| (format!("T{t}"), serde_json::json!((e * 100.0).round() / 100.0)))).collect();
        // the turns the actual line wins on: a win detected stays detected
        let wins: Vec<u64> = turns.iter().filter(|(_, o, _)| o["wins"].is_string()).map(|(t, _, _)| *t).collect();
        now.insert(name(&c), serde_json::json!({ "verify": verify, "errors": errors, "wins": wins }));
    }
    if std::env::var_os("SI_CORPUS_BLESS").is_some() {
        std::fs::create_dir_all(baseline_path().parent().unwrap()).unwrap();
        std::fs::write(baseline_path(), serde_json::to_string_pretty(&now).unwrap() + "\n").unwrap();
        return;
    }
    let base: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(baseline_path()).expect("no baseline; run once with SI_CORPUS_BLESS=1")).unwrap();
    let mut worse: Vec<String> = Vec::new();
    for (cap, b) in &base {
        let Some(n) = now.get(cap) else { continue };
        let (v0, v1) = (b["verify"].as_u64().unwrap_or(0), n["verify"].as_u64().unwrap_or(0));
        if v1 > v0 {
            worse.push(format!("{cap}: si verify mismatches {v0} -> {v1}"));
        }
        for w in b["wins"].as_array().into_iter().flatten() {
            if !n["wins"].as_array().is_some_and(|a| a.contains(w)) {
                worse.push(format!("{cap}: the win at T{w} is no longer seen"));
            }
        }
        for (turn, e0) in b["errors"].as_object().into_iter().flatten() {
            let e0 = e0.as_f64().unwrap_or(0.0);
            let e1 = n["errors"][turn.as_str()].as_f64().unwrap_or(0.0);
            // an exact turn stays exact; any other may not grow
            if e1.abs() > e0.abs() + 1e-9 {
                worse.push(format!("{cap} {turn}: simulator error {e0:+.2} -> {e1:+.2}"));
            }
        }
    }
    assert!(worse.is_empty(), "the simulator got worse (bless with SI_CORPUS_BLESS=1 only if meant):\n{}", worse.join("\n"));
}

fn england_attempt() -> PathBuf {
    capture_root().join("live-3a72b583b5da/saves/160257_945__EndGame--8584089939076010189.dat.gz")
}

#[test]
fn thunderspeakers_t5_against_england_replays_exactly() {
    if !ready() {
        return;
    }
    let r = json(&["hindsight", england_attempt().to_str().unwrap(), "--actual", "--json", "--turn", "5"]);
    let o = &r["turns"][0]["actual"]["outcome"];
    // the journal: Blight 3, a Dahan, 2 presence (Blight and Sworn to Victory), Sudden Ambush's
    // and Manifestation's Explorers, Towns 7, 9, 13, Cities 2, 4, 6 (Manifestation's 12 damage), 10 Fear
    let got = ["blight", "dahan_lost", "presence_destroyed", "explorers_destroyed", "towns_destroyed", "cities_destroyed", "fear"].map(|k| o[k].as_u64().unwrap_or(99));
    assert_eq!(got, [3, 1, 2, 2, 3, 3, 10], "T5 as simulated: {o}");
    assert_eq!(r["turns"][0]["sample"]["model_error"].as_f64(), Some(0.0), "the Invader phase as recorded");
}

#[test]
fn englands_capital_is_avoided_and_walking_into_it_loses() {
    if !ready() {
        return;
    }
    let save = england_attempt();
    let r = json(&["hindsight", save.to_str().unwrap(), "--fixed-growth", "--json", "--turn", "5"]);
    let best = r["turns"][0]["ranked"].as_array().unwrap().iter().find(|l| !l["actual"].as_bool().unwrap_or(false)).expect("a ranked line");
    assert!(best["outcome"]["loses"].is_null(), "the best T5 line loses: {}", best["outcome"]);
    // the line the review used to suggest: nothing falls in A1, Building Boom makes the seventh
    let tried = si(&["hindsight", save.to_str().unwrap(), "--fixed-growth", "--turn", "5", "--top", "0", "--try", "SA>A5[<A4] WW>A3 LFA>A5 VT>A1 MPG>A8"]);
    let row = tried.lines().find(|l| l.trim_start().starts_with("TRY")).expect("a TRY row");
    assert!(row.contains("LOSES"), "{row}");
    let clocks = r["turns"][0]["clocks"].as_array().unwrap();
    assert!(clocks.iter().any(|c| c.as_str().is_some_and(|c| c.contains("England's capital: A1 has 5 of 7"))), "{clocks:?}");
}
