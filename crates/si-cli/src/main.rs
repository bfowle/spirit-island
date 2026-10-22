//! `si replay <log|capture> [--defs <dir>] [--json]`
//!
//! Replays a digital-edition log into a record and prints it: turns, spirits,
//! lands, coverage. Definitions come from `--defs` or `SI_DEFS_DIR` (the
//! directory `si-steam-dump defs` writes).

mod color;
mod tui;

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

/// Print one painted line (see `color`).
macro_rules! out {
    () => { println!() };
    ($($arg:tt)*) => { crate::color::emit(&format!($($arg)*)) };
}
mod journal;
use si_defs::Definitions;
use si_record::{GameRecord, Replay, TouchKind};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    color::init(&args);
    let Some(cmd) = args.first() else {
        eprint!("{}", usage_text());
        std::process::exit(2);
    };
    if matches!(cmd.as_str(), "--help" | "-h" | "help") {
        match args.get(1).and_then(|c| command(c)) {
            Some(c) => print!("{}", help_text(c)),
            None => print!("{}", usage_text()),
        }
        return Ok(());
    }
    let Some(c) = command(cmd) else {
        eprintln!("si: no command {cmd:?}\n");
        eprint!("{}", usage_text());
        std::process::exit(2);
    };
    if args[1..].iter().any(|a| a == "--help" || a == "-h") {
        print!("{}", help_text(c));
        return Ok(());
    }
    match c.name {
        "replay" => replay(&args[1..]),
        "report" => report(&args[1..]),
        "corpus" => corpus(&args[1..]),
        "cards" => cards(&args[1..]),
        "verify" => verify(&args[1..]),
        "notate" => notate(&args[1..]),
        "checkpoint" => checkpoint(&args[1..]),
        "inject" => inject(&args[1..]),
        "hindsight" => hindsight(&args[1..]),
        "import" => import(&args[1..]),
        "calibrate" => calibrate(&args[1..]),
        "fuzz" => fuzz(&args[1..]),
        "study" => tui::run(&args[1..]),
        "journal" => journal::run(&args[1..]),
        _ => unreachable!(),
    }
}

/// One command: its name, its arguments on the usage line, one line on what it does,
/// and its options, each with a line of its own.
struct CommandHelp {
    name: &'static str,
    args: &'static str,
    what: &'static str,
    options: &'static [(&'static str, &'static str)],
}

const DEFS: (&str, &str) = ("--defs <dir>", "the engine definitions (else $SI_DEFS_DIR; `si-steam-dump defs <dir>` writes them)");
const GAME: &str = "<EndGame.txt | capture dir>";

const COMMANDS: &[CommandHelp] = &[
    CommandHelp { name: "replay", args: "<EndGame.txt | capture dir> [--json] [--lands]", what: "replay a game log into a record and print it: turns, spirits, lands, coverage", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--json", "the record as JSON"), ("--lands", "every land's ledger")] },
    CommandHelp { name: "journal", args: "<capture dir | save.dat[.gz]> [--entries] [--check] [--against-log] [--json] [--lands]", what: "the record from a save's own journal (every decision save carries the whole game's), no log needed; --check: its end against the save's board; --against-log: its differences from the capture's log-built record", options: &[DEFS, ("--entries", "the journal's entries, one per line, resolved to the log's names"), ("--check", "replay the journal and compare the record's end with the board the save holds, land by land"), ("--against-log", "compare with the record replayed from the capture's log-stream.txt"), ("--json", "the record as JSON"), ("--lands", "every land's ledger")] },
    CommandHelp { name: "report", args: "<EndGame.txt | capture dir>", what: "land tempo with price and reach, the ledger, track tempo, spikes, the build profile, growth shapes, the adversary level", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS] },
    CommandHelp { name: "corpus", args: "<path>... [--md]", what: "one row per game over captures and EndGame logs, then the groupings", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--md", "Markdown tables")] },
    CommandHelp { name: "verify", args: "<EndGame.txt | capture dir> [--level n] [--all]", what: "simulate each turn's Invader phase and the Power uses, and compare them with what the log records", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--level n", "the adversary level when the save beside the log does not say"), ("--all", "every land, not only the ones that differ")] },
    CommandHelp { name: "notate", args: "<EndGame.txt | capture dir> [--canonical]", what: "the game in Spirit Island notation: decisions, reveals and results kept apart, Power>land[results]", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--canonical", "full ids instead of the short codes")] },
    CommandHelp { name: "cards", args: "[--unparsed [n]] [--card <title>] [--innates] [--features] [--fit minor|major|unique|all]", what: "the effect grammar's coverage of the Power Cards and innates; one card's parse; the feature table; the pricing fit", options: &[DEFS, ("--unparsed [n]", "the cards the grammar does not read in full, n of them"), ("--card <title>", "one card or innate, sentence by sentence (read, typed, simulated)"), ("--innates", "the innate tiers not read"), ("--features", "the feature table over the cards"), ("--fit <kind>", "the least-squares price fit over that kind")] },
    CommandHelp { name: "hindsight", args: "<EndGame.txt | capture dir> [--turn n] [--top n] [--fixed-growth] [--seed <save>] [--calibration <json>] [--try \"SLSF>C8 MI\"]", what: "each turn's growth and plays against every legal line from the same position: informed, hindsight, luck, confidence, the readings", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--turn n", "one turn only"), ("--top n", "how many alternatives to print"), ("--fixed-growth", "the growth as played; only the plays vary (much faster)"), ("--seed <save.dat|.gz>", "the save a journal that starts mid-game was reloaded from (si inject prints it)"), ("--calibration <json>", "the measured constants (else data/calibration.json beside the defs, else built in)"), ("--try \"<line>\"", "score a line you write, beside the actual one (with --turn)"), ("--actual", "the actual line only, no alternatives (seconds for a whole game)"), ("--json", "the review as JSON instead of tables: every turn's lines and outcomes, losses, clocks, the simulator's error")] },
    CommandHelp { name: "study", args: "<EndGame.txt | capture dir> | --live [--steam <dir>] [--replay <capture> --at Tn/Phase [--save <part>]]  [--fixed-growth] [--seed <save>] [--calibration <json>]", what: "the hindsight review as screens, turn by turn (a capture, its newest save; one of its saves, the game as that save holds it); --live: the game being played, the open turn scoring the plans you type beside doing nothing", options: &[("--log", "the record from the log (the EndGame log, a capture's log-stream.txt) instead of the capture's newest save journal"), DEFS, ("--live", "read the Steam folder as it stands and follow it; every save seen is recorded into $SI_CAPTURE_DIR/live-<id>/"), ("--steam <dir>", "another folder than $SI_STEAM_DIR or the Handelabra path"), ("--replay <capture> --at Tn/Phase", "the live screen on a capture cut at that save, no game needed (the earliest save of that turn and phase on the path the game took)"), ("--attempt <save>", "with --live: an earlier attempt of this game (a save of the branch rolled back from); its review of the turn being played seeds the plans"), ("--save <part>", "with --replay: a save by a part of its name instead of --at"), ("--fixed-growth", "review the growth as played; only the plays vary (seconds, not minutes)"), ("--seed <save.dat|.gz>", "a file's journal that starts mid-game: the save it was reloaded from"), ("--calibration <json>", "the measured constants"), ("keys", "h/l turns, j/k rows, Enter opens a group, e/c all, Tab pacing, t a line or a plan, d delete it, i load this turn's start into the running game, ? the rest")] },
    CommandHelp { name: "checkpoint", args: "<capture dir>... [--turn n] [--phase <Phase>] [--any-branch] [--export <dir>] [--graph [--dot]]", what: "the app's saves kept with a capture, by turn and phase, on the path the game finally took", options: &[("--turn n", "that turn's saves"), ("--phase <Phase>", "SpiritPhase, FastPower, SlowPower, Ravage, ... (the save's state name)"), ("--any-branch", "undone attempts too"), ("--export <dir>", "write the matching saves there as plain .dat"), ("--graph [--dot]", "the checkpoints as a DAG (forks where the game came back to a save); --dot for Graphviz")] },
    CommandHelp { name: "inject", args: "<capture dir> --turn n [--phase SpiritPhase] [--save <part>] [--any-branch] [--name save-Undo-990.dat] [--timeout 90] [--fetch]", what: "load that checkpoint into the RUNNING Steam app from WSL (frida-inject.exe, no restart): the open game reloads in place, or Continue opens it from the main menu", options: &[("--turn n", "the turn (its earliest save of the phase on the path the game took)"), ("--phase <Phase>", "default SpiritPhase, the turn's start"), ("--save <part>", "a save by a part of its name"), ("--any-branch", "undone attempts too"), ("--name <file>", "the file name the checkpoint is placed under in the save folder"), ("--timeout <s>", "how long to wait for the app's frame (default 90)"), ("--fetch", "download frida-inject.exe for Windows into the tools folder first")] },
    CommandHelp { name: "import", args: "<EndGame--n.txt> [--out <steam-capture dir>]", what: "build a live-<game id>/ capture from an EndGame log and the saves the app still has (its final .dat, any save-Feedback/Undo of that game)", options: &[("--out <dir>", "where the capture goes (default $SI_CAPTURE_DIR)")] },
    CommandHelp { name: "fuzz", args: "[<path>...] [--rules | --live] [--lines n] [--seed n] [--json]", what: "random legal lines on every closed turn of every game (the captures and the Steam folder), the game logic's invariants checked on each: no panic, the same result twice, finite scores, no refusal of a legal line, the notation read back the same, moves between adjacent lands, presence never from nowhere, no win and loss together; the actual line not ending a game that went on", options: &[DEFS, ("--use <game> <turn> <Power>", "one recorded use of a Power played alone and explained beside the game's result"), ("--case <Power> <seed>", "with --rules: the case a violation named, built again from its seed and explained"), ("--rules", "the rules themselves instead: every Power Card and innate in the definitions on random states of every board, every adversary at every level through Explore, Build and Ravage, each held to its laws (--lines: cases each, default 40)"), ("--live", "the turn being played instead: at the first and last save of every open phase on the path each capture's game took, doing nothing and random plans scored as the live screen scores them"), ("--lines n", "random lines per turn (default 30; with --live, plans per save, default 8)"), ("--seed n", "the seed (a finding prints the one that builds its line again)"), ("--boards", "each turn where the actual line in notation ends otherwise than the record, land by land"), ("--powers", "every Power the games used, each use played alone from the board as it began: how many leave the board as the game did, and the first that does not"), ("--json", "the reports as JSON")] },
    CommandHelp { name: "calibrate", args: "[<path>...] [--out <json>] [--dry-run] [--turns]", what: "measure over the corpus what hindsight has no rule for: the Event and Fear cards' mean worth, the next-turn weight, a turn's luck sd, the simulator's error", options: &[DEFS, ("--out <json>", "where to write (default data/calibration.json beside the defs)"), ("--dry-run", "measure and print, write nothing"), ("--turns", "also print each turn's simulator error (game, turn, ledger units), to see what a change moved")] },
];

fn command(name: &str) -> Option<&'static CommandHelp> {
    COMMANDS.iter().find(|c| c.name == name)
}

/// Every command on one line each.
fn usage_text() -> String {
    let mut s = String::from("usage: si <command> [args]   (si <command> --help for its options; $SI_DEFS_DIR names the engine definitions)\n\n");
    let w = COMMANDS.iter().map(|c| c.name.len()).max().unwrap_or(10);
    for c in COMMANDS {
        s.push_str(&format!("  {:w$}  {}\n", c.name, c.what, w = w));
    }
    s.push_str(&format!("\nGames: {GAME} is a Steam EndGame log (with its .dat beside it) or a recorder capture (live-<id>/ with saves/).\n"));
    s
}

/// One command: its usage line, what it does, its options.
fn help_text(c: &CommandHelp) -> String {
    let mut s = format!("usage: si {} {}\n\n{}\n", c.name, c.args, c.what);
    if !c.options.is_empty() {
        s.push('\n');
        let w = c.options.iter().map(|(o, _)| o.len()).max().unwrap_or(10);
        for (o, what) in c.options {
            s.push_str(&format!("  {:w$}  {}\n", o, what, w = w));
        }
    }
    s
}

/// The Steam folder that holds the `EndGame--<id>` logs and saves.
pub(crate) fn steam_dir() -> PathBuf {
    std::env::var("SI_STEAM_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/mnt/c/Users/brett/AppData/LocalLow/Handelabra Games Inc_/Spirit Island"))
}

/// Where the recorder keeps captures (`live-<game id>/`), for `si import`.
pub(crate) fn capture_root() -> PathBuf {
    std::env::var("SI_CAPTURE_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/home/brett/repos/spirit-island/data/steam-capture"))
}

/// `WetlandA5` -> `A5`: the engine names a land by terrain and id.
/// `--seed <save.dat|.gz>`: the save a mid-game log was reloaded from.
fn seed_from_args(args: &[String]) -> Result<Option<si_record::replay::Seed>> {
    let Some(p) = args.iter().position(|a| a == "--seed").and_then(|i| args.get(i + 1)) else { return Ok(None) };
    let snap = decode_save(Path::new(p)).ok_or_else(|| anyhow::anyhow!("--seed: {p} is not a readable save"))?;
    let name = Path::new(p).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    Ok(Some(si_steam::seed::from_snapshot(&snap, &name)))
}

pub(crate) fn decode_save(dat: &Path) -> Option<si_steam::engine_map::EngineSnapshot> {
    let bytes = si_steam::source::read_save(dat).ok()?;
    let graph = si_steam::nrbf::decode(&bytes).ok()?;
    Some(si_steam::engine_map::map(&graph))
}

/// Every `EndGame` save in the Steam folder by its game id (the save's
/// `game_id` is also the recorder's `live-<id>` capture name), decoded once.
fn save_index() -> &'static std::collections::BTreeMap<String, PathBuf> {
    static INDEX: std::sync::OnceLock<std::collections::BTreeMap<String, PathBuf>> = std::sync::OnceLock::new();
    INDEX.get_or_init(|| {
        let mut m = std::collections::BTreeMap::new();
        if let Ok(rd) = std::fs::read_dir(steam_dir()) {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().is_some_and(|x| x == "dat") && p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("EndGame--")) {
                    if let Some(snap) = decode_save(&p) {
                        m.insert(snap.game_id.clone(), p);
                    }
                }
            }
        }
        m
    })
}

/// The engine save of this game: beside an `EndGame--<id>.txt` log (same id,
/// `.dat`), or the `EndGame` save whose game id matches a capture's
/// `live-<id>` name. The save is the full serialized game state, which carries
/// what the log never says (adversary level, scenario, the hidden decks in
/// order, the engine's journal).
fn sibling_save(path: &Path) -> Option<si_steam::engine_map::EngineSnapshot> {
    if path.is_dir() {
        let id: String = path.file_name()?.to_string_lossy().strip_prefix("live-")?.chars().take(12).collect();
        return save_index().get(&id).and_then(|p| decode_save(p));
    }
    let dat = path.with_extension("dat");
    if !dat.exists() {
        return None;
    }
    decode_save(&dat)
}

/// The record of a game: from the newest save's journal when `path` is a capture that
/// holds saves (every decision save carries the engine's typed journal of the whole
/// game; docs/superpowers/specs/2026-10-21-save-journal-reader.md), else from the log,
/// which `--log` forces. The source names which.
pub(crate) fn load_record(path: &Path, defs: Option<&Definitions>, seed: Option<si_record::replay::Seed>, force_log: bool) -> Result<(GameRecord, String)> {
    // a save itself: the game as it holds it, whatever branch the capture's newest save stands on
    if path.is_file() && path.file_name().is_some_and(|n| n.to_string_lossy().contains(".dat")) {
        let g = si_steam::nrbf::decode(&si_steam::source::read_save(path)?)?;
        let source = format!("{} (save journal)", path.display());
        return Ok((si_steam::journal::record(&g, defs, &source), source));
    }
    if !force_log && path.is_dir() {
        if let Some(save) = journal::newest_save(path) {
            if let Some(g) = si_steam::source::read_save(&save).ok().and_then(|b| si_steam::nrbf::decode(&b).ok()) {
                let source = format!("{} (save journal: {})", path.display(), save.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
                let rec = si_steam::journal::record(&g, defs, &source);
                if rec.meta.from_setup && rec.meta.turns > 0 {
                    return Ok((rec, source));
                }
            }
        }
    }
    let (lines, undo, source) = si_record::replay::load(path)?;
    let mut rec = Replay::new(defs).with_seed(seed).run(&lines, undo.as_deref(), &source);
    // the save beside a log names what the log does not: the level, the lands across boards
    apply_save(&mut rec, path);
    Ok((rec, source))
}

fn force_log(args: &[String]) -> bool {
    args.iter().any(|a| a == "--log")
}

/// Fill the record's metadata from the save beside the log: the adversary
/// (when the log never named it), its level, the game difficulty, the scenario.
pub(crate) fn apply_save(rec: &mut GameRecord, path: &Path) -> Option<si_steam::engine_map::EngineSnapshot> {
    let snap = sibling_save(path)?;
    if let Some((id, level)) = snap.adversaries.first() {
        if rec.meta.adversary.is_none() {
            rec.meta.adversary = Some(id.clone());
        }
        rec.meta.adversary_level = level.map(|l| l as u8);
    }
    rec.meta.difficulty = snap.difficulty;
    rec.meta.scenario = snap.scenario.clone();
    if rec.meta.links.is_empty() {
        rec.meta.links = si_steam::engine_map::cross_board_links(&snap);
    }
    Some(snap)
}

pub(crate) fn defs_from(args: &[String]) -> Result<Option<Definitions>> {
    let dir = args.iter().position(|a| a == "--defs").and_then(|i| args.get(i + 1)).cloned().or_else(|| std::env::var("SI_DEFS_DIR").ok());
    match dir {
        Some(d) => {
            let defs = Definitions::from_dir(Path::new(&d))?;
            color::register(&defs);
            Ok(Some(defs))
        }
        None => Ok(None),
    }
}

/// One progress line per turn on stderr: a 40-cell bar filled as growth positions
/// complete (from any thread), the counts, `ranking`, then the turn's time.
fn progress_bars(colored: bool) -> impl Fn(si_hindsight::Progress) + Sync {
    use std::io::Write;
    const BAR: usize = 40;
    let filled = std::sync::Mutex::new(0usize);
    move |p: si_hindsight::Progress| {
        let (bold, cyan, dim, reset) = if colored { ("\x1b[1m", "\x1b[36m", "\x1b[2m", "\x1b[0m") } else { ("", "", "", "") };
        let mut err = std::io::stderr().lock();
        match p {
            si_hindsight::Progress::Start { turn, .. } => {
                *filled.lock().unwrap() = 0;
                let _ = write!(err, "  {bold}T{turn:<2}{reset} {dim}[{reset}{cyan}");
            }
            si_hindsight::Progress::Positions { done, total, .. } => {
                let after = (done * BAR / total.max(1)).min(BAR);
                let mut f = filled.lock().unwrap();
                if after > *f {
                    let _ = write!(err, "{}", "#".repeat(after - *f));
                    *f = after;
                }
            }
            si_hindsight::Progress::Ranking { play_sets, lines, .. } => {
                let f = *filled.lock().unwrap();
                let _ = write!(err, "{}{reset}{dim}]{reset} {play_sets:>6} play sets {lines:>5} lines  {dim}ranking{reset}", " ".repeat(BAR.saturating_sub(f)));
            }
            si_hindsight::Progress::Done { lines, explore_cards, seconds, .. } => {
                let _ = writeln!(err, " {lines} lines x {explore_cards} Explore card{}  {dim}{seconds:.1}s{reset}", if explore_cards == 1 { "" } else { "s" });
            }
        }
        let _ = err.flush();
    }
}

/// The definitions folder named on the command line or in `SI_DEFS_DIR`.
fn defs_dir_from(args: &[String]) -> Option<PathBuf> {
    args.iter().position(|a| a == "--defs").and_then(|i| args.get(i + 1)).cloned().or_else(|| std::env::var("SI_DEFS_DIR").ok()).map(PathBuf::from)
}

/// Where `si calibrate` writes and `si hindsight` reads the measured constants:
/// `--calibration <json>`, else `calibration.json` beside the definitions folder.
fn calibration_path(args: &[String]) -> PathBuf {
    if let Some(p) = args.iter().position(|a| a == "--calibration").and_then(|i| args.get(i + 1)) {
        return PathBuf::from(p);
    }
    defs_dir_from(args).and_then(|d| d.parent().map(|p| p.join("calibration.json"))).unwrap_or_else(|| PathBuf::from("calibration.json"))
}

fn calibration_from(args: &[String]) -> Option<si_hindsight::Calibration> {
    let p = calibration_path(args);
    let text = std::fs::read_to_string(&p).ok()?;
    match serde_json::from_str::<si_hindsight::Calibration>(&text) {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("calibration {} not read ({e}); built-in values used", p.display());
            None
        }
    }
}

fn replay(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("replay needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (rec, _) = load_record(Path::new(path), defs.as_ref(), seed_from_args(args)?, force_log(args))?;
    if args.iter().any(|a| a == "--json") {
        out!("{}", serde_json::to_string_pretty(&rec)?);
        return Ok(());
    }
    print_record(&rec, defs.as_ref(), args.iter().any(|a| a == "--lands" || a == "--all"));
    Ok(())
}

fn cards(args: &[String]) -> Result<()> {
    let Some(defs) = defs_from(args)? else { bail!("cards needs --defs <dir> or SI_DEFS_DIR") };
    let pos = |flag: &str| args.iter().position(|a| a == flag);
    if let Some(i) = pos("--card") {
        let name = args.get(i + 1).cloned().unwrap_or_default();
        let norm = |x: &str| x.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
        let innate = defs.spirits.values().flat_map(|sp| sp.innates.iter().map(move |i| (sp, i))).find(|(_, i)| i.id == name || norm(&i.title) == norm(&name));
        if let (None, Some((sp, inn))) = (defs.cards.get(&name).or_else(|| defs.card_by_title(&name)), innate) {
            // an innate: its rider and each tier, printed and as read
            out!("{}  ({}) innate of {}", inn.title, inn.id, sp.title);
            out!("  {:?} | {} | target {}", inn.speed, inn.range.as_ref().map(|r| r.words()).unwrap_or("no Range".into()), inn.target.words());
            for r in &inn.rider {
                out!("  rider: {}", si_defs::text::plain(r));
            }
            for t in &inn.tiers {
                out!("  {}{}: {}", t.elements, if t.modifier { " (modifier)" } else { "" }, si_defs::text::plain(&t.body));
                for e in &t.effects {
                    out!("      • {}", e.words());
                }
            }
            return Ok(());
        }
        let Some(c) = defs.cards.get(&name).or_else(|| defs.card_by_title(&name)) else { bail!("no card or innate {name}") };
        let elements = if c.elements.is_empty() { "no elements".to_string() } else { c.elements.to_string() };
        out!("{}  ({})", c.title, c.id);
        out!("  {:?} Power | {} Energy | {:?} | {} | target {} | {}", c.kind, c.cost, c.speed, c.range.as_ref().map(|r| r.words()).unwrap_or("no Range".into()), c.target.words(), elements);
        let print_block = |indent: &str, body: &[String], effects: &[si_model::power::Effect]| {
            out!("{indent}printed:");
            for b in body {
                let text = si_defs::text::plain(b).replace("<i>", "").replace("</i>", "").replace("<nobr>", "").replace("</nobr>", "").replace("<b>", "").replace("</b>", "");
                for line in text.lines() {
                    for sentence in line.split_inclusive(". ") {
                        let sentence = sentence.trim();
                        if !sentence.is_empty() {
                            out!("{indent}  {sentence}");
                        }
                    }
                }
            }
            out!("{indent}read as:");
            if effects.is_empty() {
                out!("{indent}  (nothing)");
            }
            let mut unread = 0usize;
            for e in effects {
                let w = e.words();
                let bad = e.is_other() || w.contains("NOT READ") || w.contains('"');
                if bad {
                    unread += 1;
                }
                out!("{indent}{}{w}", if bad { "  ✗ " } else if !e.simulated() { "  ◦ " } else { "  • " });
            }
            unread
        };
        let mut unread = print_block("  ", &c.body, &c.effects);
        for t in &c.thresholds {
            out!("  threshold {} {}", t.elements, if t.title.is_empty() { String::new() } else { format!("- {}", t.title) });
            unread += print_block("    ", &t.body, &t.effects);
        }
        let mut others = Vec::new();
        for e in c.all_effects() {
            e.others(&mut others);
        }
        let unread = unread.max(others.len());
        let slots: Vec<&str> = [(si_model::power::Slot::Explore, "Explore"), (si_model::power::Slot::Build, "Build"), (si_model::power::Slot::Ravage, "Ravage")].iter().filter(|(s, _)| c.acts_on(*s)).map(|(_, n)| *n).collect();
        out!("  can act on: {}", if slots.is_empty() { "no Invader step directly".to_string() } else { slots.join(", ") });
        out!("  for the price fit: {}", c.features().words());
        let typed_only = c.not_simulated();
        if unread == 0 && typed_only.is_empty() {
            out!("  reading: fully read and simulated");
        } else if unread == 0 {
            out!("  reading: read in full; {} effect(s) typed but not simulated yet (◦): hindsight judges the turn with that caveat", typed_only.len());
        } else {
            out!("  reading: {unread} sentence(s) or piece word(s) NOT READ (✗); hindsight will not judge a turn that plays this card");
        }
        return Ok(());
    }
    if let Some(i) = pos("--fit") {
        let kind = args.get(i + 1).filter(|k| !k.starts_with("--")).cloned().unwrap_or("all".into());
        return pricing_fit(&defs, &kind);
    }
    if args.iter().any(|a| a == "--features") {
        out!("id\ttitle\tkind\tcost\tspeed\trange\tsacred\ttarget\telements\t{}\tother", si_model::power::Features::NAMES.join("\t"));
        for c in defs.cards.values() {
            let f = c.features();
            let v: Vec<String> = f.vector().iter().map(|x| format!("{x}")).collect();
            out!("{}\t{}\t{:?}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}", c.id, c.title, c.kind, c.cost, c.speed, c.range.as_ref().map(|r| r.distance.to_string()).unwrap_or("-".into()), c.range.as_ref().is_some_and(|r| r.from_sacred_site), c.target.0, c.elements.total(), v.join("\t"), f.other);
        }
        return Ok(());
    }
    let total = defs.cards.len();
    let full = defs.cards.values().filter(|c| c.fully_parsed()).count();
    let mut by_kind: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    let mut slots = [0usize; 3];
    for c in defs.cards.values() {
        let e = by_kind.entry(format!("{:?}", c.kind)).or_default();
        e.0 += 1;
        if c.fully_parsed() {
            e.1 += 1;
        }
        for (i, s) in [si_model::power::Slot::Explore, si_model::power::Slot::Build, si_model::power::Slot::Ravage].into_iter().enumerate() {
            if c.acts_on(s) {
                slots[i] += 1;
            }
        }
    }
    out!("{full} of {total} Power Cards fully parsed ({:.0}%)", 100.0 * full as f64 / total.max(1) as f64);
    let simulated = defs.cards.values().filter(|c| c.fully_parsed() && c.not_simulated().is_empty()).count();
    out!("{simulated} of {total} read and simulated in full; {} read with typed effects the simulator does not execute yet", full - simulated);
    for (k, (n, f)) in &by_kind {
        out!("  {k}: {f}/{n}");
    }
    out!("cards that can act on a slot: explore {} | build {} | ravage {}", slots[0], slots[1], slots[2]);
    let tiers: Vec<&si_model::spirit::Tier> = defs.spirits.values().flat_map(|s| s.innates.iter().flat_map(|i| i.tiers.iter())).collect();
    let tiers_full = tiers.iter().filter(|t| !t.effects.is_empty() && !t.effects.iter().any(|e| e.is_other())).count();
    out!("{tiers_full} of {} innate tiers fully parsed", tiers.len());
    if args.iter().any(|a| a == "--innates") {
        out!("innate tiers the grammar does not read in full:");
        for sp in defs.spirits.values() {
            for inn in &sp.innates {
                for t in &inn.tiers {
                    let mut others = Vec::new();
                    t.effects.iter().for_each(|e| e.others(&mut others));
                    if t.effects.is_empty() || !others.is_empty() {
                        out!("  {} / {} [{}]: {}", sp.short_title, inn.title, t.elements, si_defs::text::plain(&t.body));
                        for o in others {
                            out!("      ✗ {o}");
                        }
                    }
                }
            }
        }
        return Ok(());
    }
    if let Some(i) = pos("--unparsed") {
        let n: usize = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(40);
        let mut counts: std::collections::BTreeMap<String, (u32, String)> = Default::default();
        for c in defs.cards.values() {
            for s in c.unparsed() {
                let key: String = s.chars().map(|ch| if ch.is_ascii_digit() { 'N' } else { ch }).collect();
                let e = counts.entry(key).or_insert((0, c.title.clone()));
                e.0 += 1;
            }
        }
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
        out!("unparsed sentences ({} distinct):", v.len());
        for (s, (n_, ex)) in v.into_iter().take(n) {
            out!("  {n_:>3}  {s}   [{ex}]");
        }
    }
    Ok(())
}

/// Section 4 of the tempo spec: cost regressed on what the card does, over
/// the fully parsed cards. Coefficients are the implied Energy price of one
/// unit of each feature; residuals name the cards priced under or over
/// their effect.
fn pricing_fit(defs: &Definitions, kind: &str) -> Result<()> {
    use si_model::power::{Features, PowerKind, Speed};
    let want = |c: &si_model::PowerCard| match kind {
        "minor" => c.kind == PowerKind::Minor,
        "major" => c.kind == PowerKind::Major,
        "unique" => c.kind == PowerKind::Unique,
        _ => true,
    };
    let cards: Vec<&si_model::PowerCard> = defs.cards.values().filter(|c| c.fully_parsed() && want(c)).collect();
    let names: Vec<String> = ["intercept"].iter().map(|s| s.to_string()).chain(Features::NAMES.iter().map(|s| s.to_string())).chain(["slow", "range", "from sacred site", "major", "unique", "elements"].iter().map(|s| s.to_string())).collect();
    let row = |c: &si_model::PowerCard| -> Vec<f64> {
        let mut v = vec![1.0];
        v.extend(c.features().vector());
        v.push(if c.speed == Speed::Slow { 1.0 } else { 0.0 });
        v.push(c.range.as_ref().map(|r| r.distance as f64).unwrap_or(0.0));
        v.push(if c.range.as_ref().is_some_and(|r| r.from_sacred_site) { 1.0 } else { 0.0 });
        v.push(if c.kind == PowerKind::Major { 1.0 } else { 0.0 });
        v.push(if c.kind == PowerKind::Unique { 1.0 } else { 0.0 });
        v.push(c.elements.total() as f64);
        v
    };
    let x: Vec<Vec<f64>> = cards.iter().map(|c| row(c)).collect();
    let y: Vec<f64> = cards.iter().map(|c| c.cost as f64).collect();
    let Some(beta) = si_metrics::fit::least_squares(&x, &y) else { bail!("no cards to fit") };
    let res = si_metrics::fit::residuals(&x, &y, &beta);
    out!("PRICING FIT ({kind}) - cost ≈ Σ coefficient × feature, over {} fully parsed cards (R² {:.2})", cards.len(), si_metrics::fit::r_squared(&y, &res));
    out!("Energy per unit of:");
    for (i, (n, b)) in names.iter().zip(&beta).enumerate() {
        let used = x.iter().any(|r| r[i] != 0.0) && (i == 0 || x.iter().any(|r| r[i] != x[0][i]));
        if used {
            out!("  {:<18} {:>6.2}", n, b);
        }
    }
    let mut order: Vec<usize> = (0..cards.len()).collect();
    order.sort_by(|a, b| res[*b].partial_cmp(&res[*a]).unwrap());
    out!("priced above their effect (cost − fit, largest first):");
    for i in order.iter().take(10) {
        out!("  {:>+5.1}  {} (cost {}, {:?})", res[*i], cards[*i].title, cards[*i].cost, cards[*i].kind);
    }
    out!("priced below their effect:");
    for i in order.iter().rev().take(10) {
        out!("  {:>+5.1}  {} (cost {}, {:?})", res[*i], cards[*i].title, cards[*i].cost, cards[*i].kind);
    }
    Ok(())
}

/// A table in the house style (talaria-cli): UTF-8 box borders with round corners,
/// content arranged to the width, a bold cyan header; styling off when color is off.
/// `condensed` drops the lines between rows (short tables).
fn styled_table(width: u16, condensed: bool, header: &[&str]) -> comfy_table::Table {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets, Attribute, Cell, Color, ContentArrangement, Table};
    let mut t = Table::new();
    t.load_preset(if condensed { presets::UTF8_FULL_CONDENSED } else { presets::UTF8_FULL }).apply_modifier(UTF8_ROUND_CORNERS).set_content_arrangement(ContentArrangement::Dynamic).set_width(width);
    if !color::enabled() {
        t.force_no_tty();
    }
    if !header.is_empty() {
        t.set_header(header.iter().map(|h| Cell::new(h).add_attribute(Attribute::Bold).fg(Color::Cyan)).collect::<Vec<_>>());
    }
    t
}

/// The terminal's width, or `COLUMNS` when piped, else 170.
fn terminal_width() -> u16 {
    terminal_size::terminal_size().map(|(terminal_size::Width(w), _)| w).or_else(|| std::env::var("COLUMNS").ok().and_then(|c| c.parse().ok())).unwrap_or(170)
}

/// `HHMMSS_mmm` of a file's write time (UTC clock), the recorder's save-name stamp, and the same as a log clock.
fn stamp_of(path: &Path) -> (String, String) {
    let ms = std::fs::metadata(path).and_then(|m| m.modified()).ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64 % 86_400_000).unwrap_or(0);
    let (h, m, s, mil) = (ms / 3_600_000, ms / 60_000 % 60, ms / 1000 % 60, ms % 1000);
    (format!("{h:02}{m:02}{s:02}_{mil:03}"), format!("{h:02}:{m:02}:{s:02}.{mil:03}"))
}

/// Build a `live-<game id>/` capture from an EndGame log after the fact: the log as
/// `log-stream.txt`, the game's final save and any `save-Feedback`/`save-Undo` of the
/// same game still in the app's folder as `saves/`, their write times as `events.txt`.
/// The app keeps only the current game's step saves, so an old game gets its final
/// state alone; a game the recorder watched has every step.
fn import(args: &[String]) -> Result<()> {
    let Some(log) = args.first() else { bail!("import needs an EndGame log (EndGame--<n>.txt)") };
    let log = PathBuf::from(log);
    let dat = log.with_extension("dat");
    let snap = decode_save(&dat).ok_or_else(|| anyhow::anyhow!("no readable save beside the log ({}): the game id comes from it", dat.display()))?;
    let root = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(capture_root);
    let id = snap.game_id.clone();
    let mut dir = root.join(format!("live-{id}"));
    if dir.exists() {
        // the same game id: a branch of a recorded game (a reloaded checkpoint played on)
        let n: String = log.file_stem().map(|s| s.to_string_lossy().chars().filter(char::is_ascii_digit).collect()).unwrap_or_default();
        let tail: String = n.chars().rev().take(6).collect::<Vec<_>>().into_iter().rev().collect();
        dir = root.join(format!("live-{id}-{tail}"));
        if dir.exists() {
            bail!("{} already exists", dir.display());
        }
    }
    std::fs::create_dir_all(dir.join("saves"))?;
    std::fs::copy(&log, dir.join("log-stream.txt"))?;
    let mut events: Vec<(String, String)> = Vec::new();
    let mut copied = 0;
    let steam = log.parent().map(Path::to_path_buf).unwrap_or_else(steam_dir);
    let mut saves: Vec<PathBuf> = vec![dat.clone()];
    if let Ok(rd) = std::fs::read_dir(&steam) {
        saves.extend(rd.flatten().map(|e| e.path()).filter(|p| {
            p.file_name().is_some_and(|n| {
                let n = n.to_string_lossy();
                n.starts_with("save-") && n.ends_with(".dat")
            })
        }));
    }
    for p in saves {
        let Some(s) = decode_save(&p) else { continue };
        if s.game_id != id {
            continue;
        }
        let name = if p == dat { "save-EndGame.dat".to_string() } else { p.file_name().unwrap().to_string_lossy().to_string() };
        let (stamp, clock) = stamp_of(&p);
        std::fs::copy(&p, dir.join("saves").join(format!("{stamp}__{name}")))?;
        events.push((clock.clone(), format!("{clock}  NEW            {name}")));
        copied += 1;
    }
    events.sort();
    std::fs::write(dir.join("events.txt"), events.iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>().join("\n") + "\n")?;
    out!("{}: log-stream.txt from {}, {} save(s) (turn {} at {} in the final one)", dir.display(), log.display(), copied, snap.turn_index + 1, snap.state);
    out!("review: si hindsight {}   checkpoints: si checkpoint {}", dir.display(), dir.display());
    Ok(())
}

fn hindsight(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("hindsight needs a log file or capture directory") };
    let Some(defs) = defs_from(args)? else { bail!("hindsight needs --defs <dir> or SI_DEFS_DIR") };
    let only_turn: Option<u32> = args.iter().position(|a| a == "--turn").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok());
    let top: usize = args.iter().position(|a| a == "--top").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(3);
    let try_line: Option<String> = args.iter().position(|a| a == "--try").and_then(|i| args.get(i + 1)).cloned();
    let fixed_growth = args.iter().any(|a| a == "--fixed-growth");
    let (mut rec, _) = load_record(Path::new(path), Some(&defs), seed_from_args(args)?, force_log(args))?;
    apply_save(&mut rec, Path::new(path));
    let codes = std::cell::RefCell::new(si_record::notation::Codes::new(Some(&defs), false));
    color::set_boards(&rec.meta.boards);
    let progress = std::io::IsTerminal::is_terminal(&std::io::stderr());
    let calibration = calibration_from(args);
    // the Blight Card per turn from the capture's saves (a save under saves/ names its capture)
    let home = {
        let p = Path::new(path);
        if p.is_file() { p.parent().filter(|d| d.file_name().is_some_and(|n| n == "saves")).and_then(|d| d.parent()).map(Path::to_path_buf).unwrap_or_else(|| p.to_path_buf()) } else { p.to_path_buf() }
    };
    let actual_only = args.iter().any(|a| a == "--actual");
    let json = args.iter().any(|a| a == "--json");
    let opts = si_hindsight::ReviewOptions { growth: !fixed_growth && !actual_only, calibration, blight_pool: blight_pool_by_turn(&home), actual_only, ..Default::default() };
    let progress = progress && !json;
    if progress {
        eprintln!("hindsight  {} turns; each bar fills as the turn's growth positions are tried with every play set, then the ranking runs{}", rec.turns.iter().filter(|t| t.turn >= 1 && !t.before_fast.is_empty()).count(), if fixed_growth { " (--fixed-growth: the growth as played)" } else { " (--fixed-growth keeps the growth as played and is much faster)" });
    }
    // --try with --json and --turn: the written line's full result, nothing else
    if let (true, Some(text), Some(turn)) = (args.iter().any(|a| a == "--json"), &try_line, only_turn) {
        match si_hindsight::try_line(&rec, &defs, turn, text) {
            Ok(l) => println!("{}", serde_json::to_string(&l)?),
            Err(e) => bail!("{e}"),
        }
        return Ok(());
    }
    let only: Option<Vec<u32>> = only_turn.map(|t| vec![t]);
    let review = if progress { si_hindsight::review_turns(&rec, &defs, &opts, only.as_deref(), &progress_bars(color::enabled())) } else { si_hindsight::review_turns(&rec, &defs, &opts, only.as_deref(), &|_| {}) };
    // --json: the review itself (turns, lines, outcomes, losses, clocks, the simulator's error)
    if json {
        println!("{}", serde_json::to_string(&review)?);
        return Ok(());
    }
    let pool_at = opts.blight_pool.clone();
    // the game in a small table; the key to every column and term is a table at the end
    use comfy_table::{ColumnConstraint, Width};
    let width = terminal_width();
    // GAME and KEY are reading tables: capped at 120 columns like CODES, rows wrap
    let two_column = |rows: Vec<(String, String)>| -> comfy_table::Table {
        let mut t = styled_table(width.min(120), true, &[]);
        if let Some(c) = t.column_mut(0) {
            c.set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(11)));
        }
        for (k, v) in rows {
            t.add_row(vec![k, v]);
        }
        t
    };
    let spirit_name = |id: &str| defs.spirits.get(id).map(|p| p.title.clone()).filter(|t| !t.is_empty()).unwrap_or_else(|| id.to_string());
    let adversary = match (&rec.meta.adversary, rec.meta.adversary_level) {
        (Some(id), level) => format!("{}{}", defs.adversaries.get(id).map(|a| a.title.clone()).unwrap_or_else(|| id.clone()), level.map(|l| format!(", level {l}")).unwrap_or_else(|| ", level unknown".into())),
        (None, _) => "none".into(),
    };
    let mut game: Vec<(String, String)> = vec![
        ("source".into(), rec.meta.source.clone()),
        ("spirit".into(), rec.meta.spirits.iter().map(|s| spirit_name(s)).collect::<Vec<_>>().join(" + ")),
        ("board".into(), rec.meta.boards.join(" ")),
        ("adversary".into(), adversary),
        ("deck".into(), format!("{} by stage, the first card the setup Explore", review.deck.source.rsplit(": ").next().unwrap_or(&review.deck.source))),
        ("result".into(), rec.meta.ending.map(|e| e.words().to_string()).unwrap_or_else(|| format!("in progress: {} turns in the log", rec.meta.turns))),
    ];
    if rec.coverage.undo_unresolved > 0 {
        game.push(("record".into(), format!("bare log: {} undo reloads placed one save back; a capture would be exact", rec.coverage.undo_reloads)));
    }
    if fixed_growth {
        game.push(("growth".into(), "as played (--fixed-growth): only the plays vary".into()));
    }
    for n in &review.notes {
        game.push(("note".into(), n.clone()));
    }
    out!("GAME");
    for line in two_column(game).lines() {
        out!("  {line}");
    }
    let code = |id: &str| codes.borrow_mut().code(id);
    let spaced = |id: &str| -> String {
        let mut w = String::new();
        for (i, ch) in id.chars().enumerate() {
            if ch.is_ascii_uppercase() && i > 0 {
                w.push(' ');
            }
            w.push(ch);
        }
        w
    };
    for t in &review.turns {
        if only_turn.is_some_and(|n| n != t.turn) {
            continue;
        }
        out!();
        let hand: Vec<String> = t.hand.iter().map(|c| code(c)).collect();
        out!("==== T{} ====  hand {}  |  {} plays  |  {} Energy", t.turn, hand.join(" "), t.plays, t.energy);
        // the Blight pool at the turn's start, when the capture kept that save: a line whose
        // Blight reaches it ends the game
        let pool = pool_at.get(&t.turn).copied();
        let pool_note = match pool {
            Some((p, true)) => format!(" | Blight card flipped, {p} left: Bl{p} or more LOSES the game"),
            Some((p, false)) => format!(" | Blight card not yet flipped, {p} left on it: Bl{p} or more FLIPS it"),
            None => String::new(),
        };
        out!("  LINES  {} play sets over {} growth position{}, ranked by informed{}", t.candidates, t.positions, if t.positions == 1 { "" } else { "s" }, pool_note);
        // a wrapping table: long cells fold onto more lines instead of being cut. The tag,
        // the numbers, the eight element letters (the painter needs them on one line) and
        // the tiers have one width each; the text columns have the width of their header
        // at least, so no header ever wraps; the table is as wide as the terminal, or as
        // wide as those minimums need
        use comfy_table::{Cell, CellAlignment};
        let headers = ["", "growth", "plays", "informed", "hindsight", "conf", "elements", "innate tiers", "produced"];
        let tiers_w: u16 = (t.actual.tiers.iter().map(|(id, _, of)| code(id).len() + 3 + of.to_string().len()).sum::<usize>() + t.actual.tiers.len()).max(12) as u16 + 2;
        let fixed: [(usize, u16); 6] = [(0, 8), (3, 11), (4, 11), (5, 6), (6, 25), (7, tiers_w)];
        let floors: [(usize, u16); 3] = [(1, 16), (2, 28), (8, 16)];
        let min_total: u16 = fixed.iter().map(|(_, w)| *w).sum::<u16>() + floors.iter().map(|(_, w)| *w).sum::<u16>() + headers.len() as u16 + 1;
        let width = width.max(min_total);
        let mut table = styled_table(width, false, &headers);
        for (i, w) in fixed {
            if let Some(c) = table.column_mut(i) {
                c.set_constraint(ColumnConstraint::Absolute(Width::Fixed(w)));
            }
        }
        for (i, w) in floors {
            if let Some(c) = table.column_mut(i) {
                c.set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(w)));
            }
        }
        let row = |tag: &str, l: &si_hindsight::Line| -> Vec<Cell> {
            let growth = l.growth_notation(&|id| code(id));
            let growth = if l.growth_actual && !l.actual { format!("{growth} (as played)") } else { growth };
            let growth = if l.ties > 0 { format!("{growth} +{} alike", l.ties) } else { growth };
            let conf = l.confidence.map(|c| format!("{:.0}%", c * 100.0)).unwrap_or_else(|| "-".into());
            let tiers: Vec<String> = l.tiers.iter().map(|(id, reached, of)| format!("{} {reached}/{of}", code(id))).collect();
            let mark = match pool {
                Some((p, true)) if p > 0 && l.outcome.blight as i64 >= p && l.outcome.loses.is_none() => " LOSES",
                Some((p, false)) if p > 0 && l.outcome.blight as i64 >= p => " FLIPS",
                _ => "",
            };
            vec![
                Cell::new(tag),
                Cell::new(growth),
                // every play in the order it acts: innates in braces, F: an innate taken Fast,
                // (WW) a card played for its elements only
                Cell::new(l.plays_notation(&|id| code(id), true, true)),
                Cell::new(format!("{:+.2}", l.informed)).set_alignment(CellAlignment::Right),
                Cell::new(format!("{:+.2}", l.score)).set_alignment(CellAlignment::Right),
                Cell::new(conf).set_alignment(CellAlignment::Right),
                Cell::new(l.elements.clone()),
                Cell::new(tiers.join(" ")),
                Cell::new(format!("{}{mark}", l.outcome.summary())),
            ]
        };
        table.add_row(row("actual", &t.actual));
        let mut shown = 0;
        for l in t.ranked.iter() {
            if l.actual {
                continue;
            }
            if shown >= top {
                break;
            }
            table.add_row(row(if shown == 0 { "best" } else { "then" }, l));
            shown += 1;
        }
        if t.gap_growth > 0.25 {
            if let Some(l) = t.ranked.iter().find(|l| l.growth_actual && !l.actual) {
                table.add_row(row("same G", l));
            }
        }
        for line in table.lines() {
            out!("    {line}");
        }
        out!("  READING  {}", t.reading);
        if !t.clocks.is_empty() {
            out!("  CLOCKS   {}", t.clocks.join("; "));
        }
        // a loss the actual line meets, this turn or the next with no Spirit action
        if let Some(r) = &t.actual.outcome.loses {
            out!("  LOSS     your line: {r}");
        } else if let Some(r) = &t.actual.outcome.next_loses {
            out!("  LOSS     next turn unless something stops it: {r}");
        }
        if !t.choices.is_empty() {
            // one row per prompt: what asked, what it placed, the verdict, every option with its
            // score where the simulator could place the effect, the chosen one marked
            out!("  CHOICES  what the Event, Fear and Blight steps asked; each land scored with your growth and plays kept");
            let mut ct = styled_table(width, true, &["step", "card", "asked", "verdict", "options (<- chosen)"]);
            for (i, w) in [(0usize, 7u16), (1, 26), (3, 20)] {
                if let Some(c) = ct.column_mut(i) {
                    c.set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(w)));
                }
            }
            for c in &t.choices {
                let opts: Vec<String> = c.options.iter().map(|(o, sc)| match sc {
                    Some(v) if *o == c.selected => format!("{o} {v:+.2} <-"),
                    Some(v) => format!("{o} {v:+.2}"),
                    None if *o == c.selected => format!("{o} <-"),
                    None => o.clone(),
                }).collect();
                let (asked, verdict) = match c.gap {
                    Some(g) => (c.note.split(" placed in each land").next().unwrap_or("").to_string(), if g <= 0.01 { "best option".to_string() } else if g <= 0.5 { format!("within {g:.2} of best") } else { format!("better land open, by {g:.2}") }),
                    None => (spaced(&c.reason), "not scored".to_string()),
                };
                ct.add_row(vec![c.source_kind.clone(), c.source.clone(), asked, verdict, opts.join("  ")]);
            }
            for line in ct.lines() {
                out!("    {line}");
            }
        }
        out!("  HIDDEN   face down when you chose");
        for h in &t.hidden {
            let (label, value) = h.split_once(": ").unwrap_or(("", h.as_str()));
            let value = match label {
                "Event" => value.split(", ").map(spaced).collect::<Vec<_>>().join(", "),
                "Draft" => {
                    let (cards, tail) = value.split_once(';').unwrap_or((value, ""));
                    format!("{}{}", cards.split(", ").map(|c| code(c.trim())).collect::<Vec<_>>().join(" "), if tail.is_empty() { String::new() } else { format!(";{tail}") })
                }
                _ => value.to_string(),
            };
            out!("    {label:<8} {value}");
        }
        if !t.forecast.is_empty() {
            // the deck from this Fast phase: which stage each coming turn draws from, which cards
            // are still possible, and each land's chance of an Explore, re-hits apart from fresh
            out!("  FORECAST the Invader deck from this Fast phase: each land's chance of an Explore; re-hit = the land already holds Invaders or Blight (board as this turn's face-up Ravage and Build leave it)");
            let mut ft = styled_table(width, true, &["turn", "stage", "cards possible", "re-hit", "fresh", "expected re-hits / fresh"]);
            for (i, w) in [(0usize, 6u16), (1, 7), (5, 14)] {
                if let Some(c) = ft.column_mut(i) {
                    c.set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(w)));
                }
            }
            let chance = |l: &si_hindsight::deck::LandForecast| format!("{} {:.0}%", l.land, l.p * 100.0);
            for f in &t.forecast {
                let rehit: Vec<String> = f.lands.iter().filter(|l| l.rehit).map(chance).collect();
                let fresh: Vec<String> = f.lands.iter().filter(|l| !l.rehit).map(chance).collect();
                let cards: Vec<String> = f.cards.iter().map(|c| c.clone()).collect();
                ft.add_row(vec![format!("T{}", f.turn), format!("{}", f.stage), cards.join(" "), if rehit.is_empty() { "-".into() } else { rehit.join("  ") }, if fresh.is_empty() { "-".into() } else { fresh.join("  ") }, format!("{:.1} / {:.1}", f.rehits, f.fresh)]);
            }
            for line in ft.lines() {
                out!("    {line}");
            }
        }
        if !t.notes.is_empty() {
            out!("  NOTES");
            for n in &t.notes {
                out!("    {n}");
            }
        }
        if let Some(line) = &try_line {
            if only_turn == Some(t.turn) {
                match si_hindsight::try_line(&rec, &defs, t.turn, line) {
                    Ok(l) => out!("  TRY    {:<36} {:<44} {:>8} {:>+8.2}  {}", "(growth as played)", l.notation(&|id| code(id)), "-", l.score, l.outcome.summary()),
                    Err(e) => out!("  TRY: {e}"),
                }
            }
        }
    }
    // the key: one short row per column and term, the numbers from the review itself
    let v = &review.values;
    let cal = &review.calibration;
    let w = v.next_weight;
    let mut key: Vec<(String, String)> = vec![
        ("lines".into(), "each turn's growth and plays against every legal line from the same position; every line then runs the turn's real Invader steps and the next turn's. actual = yours; best, then = the alternatives, ranked by informed; same G = the best line from your growth when another growth ranks first".into()),
        ("growth".into(), "every group, top or bottom slot, every legal placement, each card of the offered draft, the Reclaim. (as played) keeps yours; +N alike folds rows with the same plays and score from another placement or slot".into()),
        ("plays".into(), "every Fast and Slow card and innate at every legal target. A Slow card takes its land after the Invader steps, as you do; the land shown is the one chosen under the Explore card that came".into()),
        ("notation".into(), "CARD>A2 targets the land; CARD@A2 places presence there; CARD alone acts on the Spirit or in all its lands at once; innates are underlined".into()),
        ("elements".into(), "S M F A W E P A = Sun Moon Fire Air Water Earth Plant Animal, each with its count. CTM 2/3 = innate tier 2 of 3 reached".into()),
        ("produced".into(), "Bl Blight, Dh Dahan lost, Pr presence lost, Pr+1 presence added, kill 2E 1T destroyed, push N pushed out, Fr Fear, built 1T the Build's Towns and Cities; | next = the next turn's Invader steps, | then = the turn after's expected Ravage".into()),
        ("informed".into(), "the line's worth on what was knowable: averaged over the Explore cards still in the stage, the face-down Event and Fear cards at their measured average worth, a card from a later draft not played. Lines rank by it".into()),
        ("hindsight".into(), "the line's worth on the cards that came. hindsight minus informed is luck".into()),
        ("conf".into(), format!("how sure the line beats yours: under each possible Explore card, the chance its lead outweighs the simulator's own error ({:.2}), averaged over the cards. A lead of 1 reads about 70%, of 3 about 93%", cal.model_sd)),
        ("reading".into(), format!("the gap to the best informed line against a turn's usual luck ({:.2}): Sound under a fifth of it, Better available up to four fifths, Mistake above. Unlucky when the hidden cards cost more than a whole swing; Gamble paid when they made your line the best", cal.luck_sd)),
        ("hidden".into(), "the Ravage and Build cards were face up when you chose: knowable, not luck. The Explore, Event and Fear cards were face down; what they did is applied from the log to every line in the hindsight score".into()),
        ("ledger".into(), format!("Blight {:+.2}; Dahan {:+.2}; presence {:+.2}; Explorer/Town/City destroyed {:+.2}/{:+.2}/{:+.2} and built the same against; pushed out {:+.2}; Fear {:+.2}; track slot revealed {:+.2}; card gained {:+.2}; Energy kept {:+.2}", v.blight, v.dahan, v.presence, v.explorer, v.town, v.city, v.explorer, v.fear, v.slot, v.card, v.energy)),
    ];
    if !v.reasons.is_empty() {
        key.push(("spirit".into(), v.reasons.join(". ")));
    }
    key.push(("horizon".into(), format!("this turn in full; the next turn's Invader steps at {w:.2} (the measured share of an unattended threat that materializes once the Spirit plays its turn); the turn after's expected Ravage at {:.2}", w * w)));
    key.push(("calibrated".into(), format!("Event {:+.2}, Fear card {:+.2}/{:+.2}/{:+.2} by Terror Level, a turn's luck sd {:.2}, simulator error {:.2}, all in ledger units; {}", cal.event_mean, cal.fear_card_mean[0], cal.fear_card_mean[1], cal.fear_card_mean[2], cal.luck_sd, cal.model_sd, cal.source.split(" under ").next().unwrap_or(&cal.source))));
    out!();
    out!("KEY");
    for line in two_column(key).lines() {
        out!("  {line}");
    }
    let entries = codes.borrow().entries();
    if !entries.is_empty() {
        out!();
        out!("CODES");
        let mut lt = styled_table(width.max(120), true, &[]);
        for chunk in entries.chunks(3) {
            let mut row: Vec<String> = Vec::new();
            for (c, title) in chunk {
                row.push(c.clone());
                row.push(title.clone());
            }
            while row.len() < 6 {
                row.push(String::new());
            }
            lt.add_row(row);
        }
        for line in lt.lines() {
            out!("  {line}");
        }
    }
    Ok(())
}

/// One of the app's saves kept with a capture, decoded.
pub(crate) struct Checkpoint {
    pub(crate) path: PathBuf,
    /// The file name the app wrote (`save-Feedback-002-0047-SpiritPhase.dat`).
    pub(crate) name: String,
    snap: si_steam::engine_map::EngineSnapshot,
    /// sha256 of the save bytes: the checkpoint's id. Copies share it; a reload that
    /// lands on an existing id is a fork in the game's DAG.
    id: String,
    /// On the path the game finally took (an undone attempt is off it).
    pub(crate) on_final_path: bool,
}

pub(crate) fn checkpoints_of(dir: &Path) -> Result<Vec<Checkpoint>> {
    let saves = dir.join("saves");
    if !saves.is_dir() {
        bail!("{} has no saves/ folder: only recorder captures keep the app's saves (an EndGame log has only its final .dat)", dir.display());
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(&saves)?.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().contains("save-"))).collect();
    files.sort();
    let mut out = Vec::new();
    for p in files {
        let Ok(bytes) = si_steam::source::read_save(&p) else { continue };
        let Ok(graph) = si_steam::nrbf::decode(&bytes) else { continue };
        let snap = si_steam::engine_map::map(&graph);
        use sha2::Digest;
        let id = format!("{:x}", sha2::Sha256::digest(&bytes));
        let full = p.file_name().unwrap().to_string_lossy().trim_end_matches(".gz").to_string();
        let name = full.split("__").last().unwrap_or(&full).to_string();
        out.push(Checkpoint { path: p, name, snap, id, on_final_path: false });
    }
    // the final path: walk the saves on the capture's clock; a save whose id was seen
    // before (an undo copy, a reload) moves the tip back to it, so what follows hangs
    // off that node; the ancestors of the last save are the path the game took
    let mut parent: std::collections::BTreeMap<String, Option<String>> = std::collections::BTreeMap::new();
    let mut tip: Option<String> = None;
    for c in &out {
        if !parent.contains_key(&c.id) {
            parent.insert(c.id.clone(), tip.clone());
        }
        tip = Some(c.id.clone());
    }
    let mut on_path: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut cur = tip;
    while let Some(id) = cur {
        if !on_path.insert(id.clone()) {
            break;
        }
        cur = parent.get(&id).cloned().flatten();
    }
    for c in out.iter_mut() {
        c.on_final_path = on_path.contains(&c.id);
    }
    Ok(out)
}

pub(crate) fn checkpoint_matches(c: &Checkpoint, turn: Option<i64>, phase: Option<&str>) -> bool {
    turn.is_none_or(|n| n == c.snap.turn_index + 1) && phase.is_none_or(|ph| c.snap.state.eq_ignore_ascii_case(ph) || c.snap.phase.eq_ignore_ascii_case(ph) || c.name.contains(ph))
}

/// The checkpoint saves a capture kept (`saves/*.dat.gz`, the app's own
/// save at every decision point), decoded for their turn and phase.
fn checkpoint(args: &[String]) -> Result<()> {
    let dirs: Vec<PathBuf> = args.iter().take_while(|a| !a.starts_with("--")).map(PathBuf::from).collect();
    if dirs.is_empty() {
        bail!("checkpoint needs a capture directory");
    }
    let turn: Option<i64> = args.iter().position(|a| a == "--turn").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok());
    let phase: Option<String> = args.iter().position(|a| a == "--phase").and_then(|i| args.get(i + 1)).cloned();
    let export: Option<PathBuf> = args.iter().position(|a| a == "--export").and_then(|i| args.get(i + 1)).map(PathBuf::from);
    if args.iter().any(|a| a == "--graph") {
        return checkpoint_graph(&dirs, args.iter().any(|a| a == "--dot"));
    }
    let dir = &dirs[0];
    let any_branch = args.iter().any(|a| a == "--any-branch");
    let all = checkpoints_of(dir)?;
    out!("{} save(s) kept with {} (save-Feedback-<turn index>-<journal index>-<state>; save-Undo-<n> are the app's undo copies); {} on the path the game finally took{}", all.len(), dir.display(), all.iter().filter(|c| c.on_final_path).count(), if any_branch { "" } else { " (shown; --any-branch shows undone attempts too)" });
    out!("{:<4} {:<58} {:>4}  {:<28} {:<12} {:<7} {}", "#", "file", "turn", "state", "id", "path", "game");
    let chosen: Vec<(usize, &Checkpoint)> = all.iter().enumerate().filter(|(_, c)| checkpoint_matches(c, turn, phase.as_deref()) && (any_branch || c.on_final_path)).collect();
    for (i, c) in &chosen {
        out!("{:<4} {:<58} {:>4}  {:<28} {:<12} {:<7} {}", i, c.path.file_name().unwrap().to_string_lossy(), c.snap.turn_index + 1, format!("{}/{}", c.snap.phase, c.snap.state), &c.id[..12], if c.on_final_path { "final" } else { "undone" }, c.snap.game_id);
    }
    if chosen.is_empty() {
        out!("no save matches the filter");
    }
    if let Some(dir) = export {
        let Some((_, c)) = chosen.first() else { bail!("nothing to export") };
        if chosen.len() > 1 {
            out!("{} saves match; exporting the first (narrow with --turn and --phase)", chosen.len());
        }
        let target = export_checkpoint(c, &dir)?;
        out!("exported {} ({} bytes; turn {} {}/{}; id {}) to {}", c.name, std::fs::metadata(&target)?.len(), c.snap.turn_index + 1, c.snap.phase, c.snap.state, &c.id[..12], target.display());
        out!("load it into the running app: si inject {} --turn {} --phase {}", dirs[0].display(), c.snap.turn_index + 1, c.snap.state);
    }
    Ok(())
}

/// Write the save as the file the app wrote (a capture's .dat.gz gunzipped once).
fn export_checkpoint(c: &Checkpoint, dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let bytes = si_steam::source::read_save(&c.path)?;
    let target = dir.join(&c.name);
    std::fs::write(&target, &bytes)?;
    Ok(target)
}

/// The checkpoints of one or more captures as a DAG: nodes are save ids (content
/// hashes), each save's parent is the save before it in the capture's clock, and
/// a save whose id already exists (an undo copy, a reload) moves the tip back to
/// that node, so the next save forks from it. Captures of the same game join
/// where they share an id.
fn checkpoint_graph(dirs: &[PathBuf], dot: bool) -> Result<()> {
    use std::collections::BTreeMap;
    struct Node {
        label: String,
        turn: i64,
        state: String,
        parents: Vec<String>,
        children: Vec<String>,
        seen_in: Vec<String>,
    }
    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for dir in dirs {
        let cap = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let saves = checkpoints_of(dir)?;
        let mut tip: Option<String> = None;
        for c in &saves {
            let id = c.id[..12].to_string();
            let existed = nodes.contains_key(&id);
            let node = nodes.entry(id.clone()).or_insert_with(|| Node { label: format!("{} ({})", c.name, c.snap.game_id), turn: c.snap.turn_index + 1, state: c.snap.state.clone(), parents: vec![], children: vec![], seen_in: vec![] });
            if !node.seen_in.contains(&cap) {
                node.seen_in.push(cap.clone());
            }
            if !existed {
                order.push(id.clone());
                if let Some(t) = &tip {
                    if t != &id {
                        node.parents.push(t.clone());
                        nodes.get_mut(t).unwrap().children.push(id.clone());
                    }
                }
            }
            tip = Some(id);
        }
    }
    let forks: Vec<&String> = nodes.iter().filter(|(_, n)| n.children.len() > 1).map(|(id, _)| id).collect();
    let roots = nodes.values().filter(|n| n.parents.is_empty()).count();
    let leaves: Vec<&String> = nodes.iter().filter(|(_, n)| n.children.is_empty()).map(|(id, _)| id).collect();
    if dot {
        out!("digraph checkpoints {{");
        out!("  rankdir=LR; node [shape=box, fontsize=9];");
        for (id, n) in &nodes {
            let color = if n.children.len() > 1 { ", color=red, penwidth=2" } else if n.children.is_empty() { ", color=darkgreen" } else { "" };
            out!("  \"{id}\" [label=\"T{} {}\\n{}\\n{}\"{color}];", n.turn, n.state, n.label, id);
        }
        for (id, n) in &nodes {
            for ch in &n.children {
                out!("  \"{id}\" -> \"{ch}\";");
            }
        }
        out!("}}");
        return Ok(());
    }
    out!("CHECKPOINT DAG - {} capture(s), {} distinct saves, {} root(s), {} fork(s), {} leaf/leaves", dirs.len(), nodes.len(), roots, forks.len(), leaves.len());
    out!("ids are the first 12 hex digits of the save's sha256; a fork is a save the game came back to (undo, or a live reload) and left again");
    for id in &order {
        let n = &nodes[id];
        let mark = if n.children.len() > 1 { "+" } else if n.children.is_empty() { "*" } else { "|" };
        out!("  {mark} {id}  T{:<2} {:<22} {:<48} <- {}  {}", n.turn, n.state, n.label, n.parents.join(","), if n.seen_in.len() > 1 { format!("in {}", n.seen_in.join(" + ")) } else { String::new() });
    }
    if !forks.is_empty() {
        out!("forks:");
        for f in forks {
            let n = &nodes[f];
            out!("  {f} T{} {} -> {} branches: {}", n.turn, n.state, n.children.len(), n.children.join(" "));
        }
    }
    Ok(())
}

/// Load a checkpoint into the running Steam app from WSL: export the save next to
/// Frida's standalone injector on the Windows side and run it with the reload
/// script; the app reloads in place (its own Undo path), no restart.
fn inject(args: &[String]) -> Result<()> {
    use std::io::BufRead;
    use std::process::{Command, Stdio};
    let Some(path) = args.first() else { bail!("inject needs a capture directory") };
    let turn: Option<i64> = args.iter().position(|a| a == "--turn").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok());
    let phase: Option<String> = args.iter().position(|a| a == "--phase").and_then(|i| args.get(i + 1)).cloned();
    let name: String = args.iter().position(|a| a == "--name").and_then(|i| args.get(i + 1)).cloned().unwrap_or("save-Undo-990.dat".into());
    let timeout: u64 = args.iter().position(|a| a == "--timeout").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(90);
    let pick: Option<String> = args.iter().position(|a| a == "--save").and_then(|i| args.get(i + 1)).cloned();
    let tools = win_tools_dir();
    std::fs::create_dir_all(tools.join("checkpoints"))?;
    let injector = tools.join("frida-inject.exe");
    if !injector.exists() {
        if args.iter().any(|a| a == "--fetch") {
            out!("fetching frida-inject {FRIDA_VERSION} for Windows into {}", tools.display());
            let url = format!("https://github.com/frida/frida/releases/download/{FRIDA_VERSION}/frida-inject-{FRIDA_VERSION}-windows-x86_64.exe.xz");
            let xz = tools.join("frida-inject.exe.xz");
            let st = Command::new("curl").args(["-sL", "-o"]).arg(&xz).arg(&url).status()?;
            if !st.success() {
                bail!("download failed: {url}");
            }
            let st = Command::new("xz").arg("-d").arg("-f").arg(&xz).status()?;
            if !st.success() {
                bail!("xz -d failed on {}", xz.display());
            }
        } else {
            bail!("{} is missing: run with --fetch to download frida-inject {FRIDA_VERSION} (Windows x64) there", injector.display());
        }
    }
    if !game_running() {
        bail!("SpiritIsland.exe is not running: open the game (any game, past setup) and run this again");
    }
    let any_branch = args.iter().any(|a| a == "--any-branch");
    let all = checkpoints_of(Path::new(path))?;
    let chosen: Vec<&Checkpoint> = all.iter().filter(|c| checkpoint_matches(c, turn, phase.as_deref()) && (any_branch || c.on_final_path) && pick.as_ref().is_none_or(|p| c.name.contains(p.as_str()))).collect();
    let Some(c) = chosen.first() else { bail!("no checkpoint on the final path matches --turn/--phase/--save (si checkpoint {} lists them; --any-branch includes undone attempts)", path) };
    if chosen.len() > 1 {
        out!("{} checkpoints on the final path match; taking the earliest, {} (the start of that phase as the game actually went; --save <file name part> picks another)", chosen.len(), c.name);
    }
    let dat = export_checkpoint(c, &tools.join("checkpoints"))?;
    out!("seed for the branch's review once it is played out: si hindsight <its EndGame log> --seed {}", c.path.display());
    // the app's loader reads disk files as <save folder>/<file name>. The undo name serves
    // both routes: in a game, ReloadFromFile reads it; at the main menu, Continue falls back
    // to the newest undo file on disk when the continue file is not found (verified
    // 2026-10-20 with Steam Cloud on: "Continue file missing but found undo").
    let save_dir = steam_dir();
    let continue_name = "save-ContinueGame.dat".to_string();
    std::fs::copy(&dat, save_dir.join(&name)).map_err(|e| anyhow::anyhow!("could not place {} in the save folder {}: {e}", name, save_dir.display()))?;
    let script = live_replay_script()?;
    let dat_win = wslpath_w(&save_dir.join(&name))?;
    let script_win = wslpath_w(&script)?;
    let log = tools.join("checkpoints").join("inject.log");
    let _ = std::fs::remove_file(&log);
    let log_win = wslpath_w(&tools.join("checkpoints"))? + "\\inject.log";
    let params = serde_json::json!({ "path": dat_win, "name": name, "continue_name": continue_name, "log": log_win }).to_string();
    let player_log = save_dir.join("Player.log");
    let log_mark = std::fs::metadata(&player_log).map(|m| m.len()).unwrap_or(0);
    out!("checkpoint {} (turn {} {}/{}, id {}) placed in {} as {}", c.name, c.snap.turn_index + 1, c.snap.phase, c.snap.state, &c.id[..12], save_dir.display(), name);
    // Unity runs no frames while its window is in the background, and the hook waits for a
    // frame: bring the game to the front first (a focus change on the desktop, nothing more)
    let focused = Command::new("powershell.exe").args(["-NoProfile", "-Command", "(New-Object -ComObject WScript.Shell).AppActivate('Spirit Island')"]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim() == "True").unwrap_or(false);
    if focused {
        out!("the game window is in front; keep it there until the reload (Unity stops running its frames in the background)");
    } else {
        out!("could not bring the game window to the front: click into it now and keep it there (Unity stops running its frames in the background)");
    }
    out!("attaching frida-inject to SpiritIsland.exe (up to {timeout}s)");
    out!("inside a game: the open game reloads in place to the checkpoint (its own Undo path); at the main menu: the checkpoint replaces the app's continue game and Continue opens it. Either way the previous state stays in its capture.");
    // -i keeps frida-inject attached while the script works; closing its stdin ends it.
    let mut child = Command::new(&injector).args(["-n", "SpiritIsland.exe", "-s", &script_win, "-P", &params, "-i"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    for reader in [Box::new(stdout) as Box<dyn std::io::Read + Send>, Box::new(stderr)] {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for line in std::io::BufReader::new(reader).lines().map_while(Result::ok) {
                let _ = tx.send(line);
            }
        });
    }
    drop(tx);
    let started = std::time::Instant::now();
    let mut done = false;
    let mut failed = false;
    let mut seen_log = 0usize;
    let handle = |line: &str, done: &mut bool, failed: &mut bool| {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            return;
        }
        out!("  app: {line}");
        if line.contains("\"step\":\"error\"") {
            *failed = true;
        }
        if line.contains("\"step\":\"done\"") {
            *done = true;
        }
    };
    while started.elapsed().as_secs() < timeout && !done {
        // the script's own log file is the reliable channel; stdout arrives when it can
        if let Ok(text) = std::fs::read_to_string(&log) {
            let lines: Vec<&str> = text.lines().collect();
            for l in &lines[seen_log.min(lines.len())..] {
                handle(l, &mut done, &mut failed);
            }
            seen_log = lines.len();
        }
        match rx.recv_timeout(std::time::Duration::from_millis(200)) {
            Ok(line) => {
                if !line.contains("\"type\":\"send\"") {
                    handle(&line, &mut done, &mut failed);
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    // closing stdin ends frida-inject's interactive loop; it detaches on exit
    drop(child.stdin.take());
    std::thread::sleep(std::time::Duration::from_millis(500));
    let _ = child.kill();
    let _ = Command::new("taskkill.exe").args(["/F", "/IM", "frida-inject.exe"]).stdout(Stdio::null()).stderr(Stdio::null()).status();
    // what the app itself logged during the run
    if let Ok(mut f) = std::fs::File::open(&player_log) {
        use std::io::{Read, Seek, SeekFrom};
        if f.seek(SeekFrom::Start(log_mark)).is_ok() {
            let mut tail = String::new();
            if f.read_to_string(&mut tail).is_ok() {
                let interesting: Vec<&str> = tail.lines().filter(|l| ["ReloadFromFile", "Failed to load", "Continue file", "Has continue", "Successfully loaded", "LoadContinue", "Loading", "Exception", "undo", "SaveIfNecessary", "SaveForContinue", "continue file", "Saving feedback", "Written to Steam Cloud"].iter().any(|k| l.contains(k))).collect();
                for l in interesting.iter().take(20) {
                    out!("  app log: {}", l.trim());
                }
            }
        }
    }
    if !done {
        bail!("no completion from the injector within {timeout}s; the app's Player.log (next to the saves) has its ReloadFromFile lines");
    }
    if failed {
        bail!("the reload failed inside the app; see the error line above");
    }
    // the app has loaded the file (twice on the menu route: the validity check and the load);
    // left on disk it would be the "last undo file" the Undo button finds, so take it away
    std::thread::sleep(std::time::Duration::from_secs(2));
    match std::fs::remove_file(save_dir.join(&name)) {
        Ok(()) => out!("removed {} from the save folder: the app holds the state now and writes its own continue save at its next decision", name),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => out!("{} is gone from the save folder already: the app's reload deleted its undo files, as it does", name),
        Err(e) => out!("could not remove {} from the save folder ({e}); delete it by hand so the Undo button does not find it", name),
    }
    out!("reloaded: the app now shows {} at turn {} ({}). Play the branch; its saves continue under the same game id, so the checkpoint DAG shows the fork. The hindsight block for this turn: si hindsight {} --turn {}", c.name, c.snap.turn_index + 1, c.snap.state, path, c.snap.turn_index + 1);
    Ok(())
}

const FRIDA_VERSION: &str = "17.24.3";

/// The Blight pool at each turn's start, from the earliest Spirit-phase save the capture
/// kept for that turn (`save-Feedback-<turn index>-<journal>-SpiritPhase`); empty for a bare log.
fn blight_pool_by_turn(path: &Path) -> std::collections::BTreeMap<u32, (i64, bool)> {
    let mut out = std::collections::BTreeMap::new();
    let saves = path.join("saves");
    let Ok(rd) = std::fs::read_dir(&saves) else { return out };
    let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().contains("-SpiritPhase.dat"))).collect();
    files.sort();
    let mut seen: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    for p in files {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        // save-Feedback-002-0047-SpiritPhase.dat(.gz): the first number is the turn index
        let Some(idx) = name.split("save-Feedback-").nth(1).and_then(|r| r.split('-').next()).and_then(|n| n.parse::<u32>().ok()) else { continue };
        if !seen.insert(idx) {
            continue;
        }
        if let Some(snap) = decode_save(&p) {
            if let Some(b) = snap.blight_card.as_ref() {
                out.insert(idx + 1, (b.blight_remaining, b.flipped));
            }
        }
    }
    out
}

/// `/mnt/c/Users/<user>/AppData/Local/si-companion/tools`: Windows-visible, next to the recorder's captures.
fn win_tools_dir() -> PathBuf {
    if let Ok(d) = std::env::var("SI_WIN_TOOLS_DIR") {
        return PathBuf::from(d);
    }
    let user = std::env::var("SI_WIN_USER").ok().or_else(|| {
        std::process::Command::new("cmd.exe").args(["/c", "echo %USERNAME%"]).output().ok().and_then(|o| String::from_utf8(o.stdout).ok()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty() && !s.contains('%'))
    }).unwrap_or_else(|| "brett".into());
    PathBuf::from(format!("/mnt/c/Users/{user}/AppData/Local/si-companion/tools"))
}

fn wslpath_w(p: &Path) -> Result<String> {
    let o = std::process::Command::new("wslpath").arg("-w").arg(p).output()?;
    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
    if s.is_empty() {
        bail!("wslpath -w failed for {}", p.display());
    }
    Ok(s)
}

fn game_running() -> bool {
    std::process::Command::new("tasklist.exe").args(["/FI", "IMAGENAME eq SpiritIsland.exe", "/NH"]).output().map(|o| String::from_utf8_lossy(&o.stdout).contains("SpiritIsland.exe")).unwrap_or(false)
}

/// The reload script shipped with the repo (`tools/live-replay/reload_checkpoint.js`), or `SI_LIVE_REPLAY_SCRIPT`.
fn live_replay_script() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("SI_LIVE_REPLAY_SCRIPT") {
        return Ok(PathBuf::from(p));
    }
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tools/live-replay/reload_checkpoint.js");
    let p = repo.canonicalize().map_err(|e| anyhow::anyhow!("reload script not found at {} ({e}); set SI_LIVE_REPLAY_SCRIPT", repo.display()))?;
    Ok(p)
}

fn notate(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("notate needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (mut rec, _) = load_record(Path::new(path), defs.as_ref(), None, force_log(args))?;
    apply_save(&mut rec, Path::new(path));
    for line in si_record::notation::notate(&rec, defs.as_ref(), args.iter().any(|a| a == "--canonical")).lines() {
        out!("{line}");
    }
    Ok(())
}

fn verify(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("verify needs a log file or capture directory") };
    let Some(defs) = defs_from(args)? else { bail!("verify needs --defs <dir> or SI_DEFS_DIR") };
    let level: Option<u8> = args.iter().position(|a| a == "--level").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok());
    let all = args.iter().any(|a| a == "--all");
    let (mut rec, _) = load_record(Path::new(path), Some(&defs), None, force_log(args))?;
    apply_save(&mut rec, Path::new(path));
    // the save beside the log states the level; else the lowest consistent
    // candidate (level rules are cumulative, so its rules hold regardless)
    let from_save = rec.meta.adversary_level;
    let est = si_metrics::adversary::estimate(&rec, &defs);
    let level = level.or(from_save).or_else(|| est.as_ref().and_then(|e| e.candidates.first().copied()));
    let r = si_rules::verify::verify(&rec, &defs, level);
    out!("{}", rec.meta.source);
    out!("spirits {} | boards {} | adversary {} level {} | {} turns simulated", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.adversary.clone().unwrap_or("none".into()), match (from_save, &est, level) { (Some(l), _, _) => format!("{l} (from the save)"), (None, Some(e), Some(l)) if e.candidates.len() > 1 => format!("{l} (lowest of {})", e.label()), (_, _, Some(l)) => l.to_string(), _ => "?".into() }, r.turns);
    out!("INVADER PHASE and SPIRIT ACTIONS - simulated from the recorded state at each phase start, compared with the log");
    out!("{:<30} {:>8} {:>9}", "field", "matched", "compared");
    for (f, (m, c)) in &r.fields {
        out!("{:<30} {:>8} {:>9}", f, m, c);
    }
    out!("{:<30} {:>8} {:>9}   ({:.0}%)", "all", r.matched(), r.compared(), 100.0 * r.matched() as f64 / r.compared().max(1) as f64);
    for n in &r.notes {
        out!("note: {n}");
    }
    let shown = if all { r.mismatches.len() } else { r.mismatches.len().min(40) };
    if !r.mismatches.is_empty() {
        out!();
        out!("mismatches ({} of {}):", shown, r.mismatches.len());
        out!("turn  step            card        land  field               simulated  logged      notes");
        for c in r.mismatches.iter().take(shown) {
            out!("T{:<4} {:<15} {:<11} {:<5} {:<19} {:<10} {:<11} {}", c.turn, c.step, c.card, c.land, c.field, c.simulated, c.logged, c.notes.join("; "));
        }
    }
    Ok(())
}

fn report(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("report needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (mut rec, _) = load_record(Path::new(path), defs.as_ref(), None, force_log(args))?;
    let snap = apply_save(&mut rec, Path::new(path));
    let chains = si_metrics::tempo::chains_with(&rec, defs.as_ref());
    out!("{}", rec.meta.source);
    out!("spirits {} | boards {} | adversary {} | turns {} | {}", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.adversary.clone().unwrap_or("none".into()), rec.meta.turns, rec.meta.ending.map(|e| e.words()).unwrap_or("no ending"));
    if rec.coverage.undo_unresolved > 0 {
        out!("caution: {} of {} undo reloads had no clock; placed one save back, {} repaired from repeated facts", rec.coverage.undo_unresolved, rec.coverage.undo_reloads, rec.coverage.undo_repairs);
    }
    if let Some(snap) = snap {
        out!(
            "save beside the log: adversary {} | difficulty {} | scenario {} | Blight card {} | Invader deck left {} | Fear deck left {} | Event deck left {}",
            snap.adversaries.iter().map(|(a, l)| format!("{a} level {}", l.map(|l| l.to_string()).unwrap_or("?".into()))).collect::<Vec<_>>().join(", "),
            snap.difficulty.map(|d| d.to_string()).unwrap_or("?".into()),
            snap.scenario.clone().unwrap_or("none".into()),
            snap.blight_card.as_ref().map(|b| format!("{}{}", b.id, if b.flipped { " (flipped)" } else { "" })).unwrap_or("?".into()),
            snap.invader_deck.len(),
            snap.fear_deck.len(),
            snap.event_deck.len()
        );
    }
    if let Some(est) = defs.as_ref().and_then(|d| si_metrics::adversary::estimate(&rec, d)) {
        out!(
            "adversary level {} (difficulty {}) - {}",
            est.label(),
            est.difficulty.map(|(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).unwrap_or("?".into()),
            est.evidence.join("; ")
        );
    }
    out!();
    out!("LAND TEMPO - each Invader card's pass over a land: Explore t -> Build t+1 -> Ravage t+2");
    out!("land terrain   card       t0   E  B  R   outcome    blight  class       price    reach                      touches by clock position");
    for c in &chains {
        // one entry per clock position: the powers used, then what else happened
        let mut touches: Vec<String> = Vec::new();
        for clock in [si_metrics::Clock::BeforeExplore, si_metrics::Clock::AfterExplore, si_metrics::Clock::AfterBuild, si_metrics::Clock::AfterRavage] {
            let here: Vec<&si_metrics::tempo::ChainTouch> = c.touches.iter().filter(|t| t.clock == clock).collect();
            if here.is_empty() {
                continue;
            }
            let mut powers: Vec<String> = Vec::new();
            let (mut defend, mut dmg, mut out, mut into, mut presence, mut fate) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
            for t in &here {
                if !t.touch.by_spirit() {
                    fate += 1;
                    continue;
                }
                match &t.touch.kind {
                    TouchKind::Power => {
                        let p = short(&t.touch.by);
                        if !powers.contains(&p) {
                            powers.push(p);
                        }
                    }
                    TouchKind::Defend(n) => defend += n,
                    TouchKind::Damage(n) => dmg += n,
                    TouchKind::Presence => presence += 1,
                    TouchKind::Moved { out: o, into: i } => {
                        out += o;
                        into += i;
                    }
                    TouchKind::Recover(_) => {}
                }
            }
            let mut parts = powers;
            if defend > 0 {
                parts.push(format!("def{defend}"));
            }
            if dmg > 0 {
                parts.push(format!("dmg{dmg}"));
            }
            if out + into > 0 {
                parts.push(format!("mv-{out}+{into}"));
            }
            if presence > 0 {
                parts.push(format!("+{presence}pres"));
            }
            if fate > 0 {
                parts.push(format!("event/fear×{fate}"));
            }
            let name = match clock {
                si_metrics::Clock::BeforeExplore => "pre-X",
                si_metrics::Clock::AfterExplore => "X->B",
                si_metrics::Clock::AfterBuild => "B->R",
                si_metrics::Clock::AfterRavage => "post-R",
            };
            touches.push(format!("{name}[{}]", parts.join(" ")));
        }
        let opt = |t: Option<u32>| t.map(|t| t.to_string()).unwrap_or("-".into());
        let price = if c.price_cards > 0 { format!("{}c {}E", c.price_cards, c.price_energy) } else { String::new() };
        let reach = match &c.reach {
            Some(r) if c.left_to_blight() => match r.first_ravage() {
                Some(card) if card.sure() => format!("could: {} T{}", short(&card.card), card.turn),
                Some(card) => format!("maybe: {} (range?)", short(&card.card)),
                None => "no reach".to_string(),
            },
            Some(r) if c.class == si_metrics::Class::On && r.could_build() => "above was open".to_string(),
            _ => String::new(),
        };
        out!(
            "{:<4} {:<9} {:<10} {:>2}  {:>2} {:>2} {:>2}   {:<10} {:>3}{}  {:<11} {:<8} {:<26} {}",
            c.land,
            c.terrain.map(|t| t.name()).unwrap_or(""),
            c.card.clone().unwrap_or_default(),
            c.t0,
            opt(c.explore),
            opt(c.build),
            opt(c.ravage),
            format!("{:?}", c.outcome).to_lowercase(),
            c.blight,
            if c.cascade > 0 { format!("+{}", c.cascade) } else { "  ".into() },
            c.class.name(),
            price,
            reach,
            touches.join("  ")
        );
    }
    out!();
    out!("LEDGER - per turn: lands due, how the Ravage-due ones ended, and what the spirits paid (all spirits)");
    out!("turn  due B/R   above  on  gambled  failed  untouched  sacrificed  blight   cards  energy   hand->discard  runway");
    let mut all_rows: Vec<si_metrics::TrackRow> = Vec::new();
    for sp in &rec.meta.spirits {
        all_rows.extend(si_metrics::track::rows(&rec, defs.as_ref(), sp));
    }
    let paid = |turn: u32| rec.spirits.iter().filter(|s| s.turn == turn).map(|s| s.energy_paid).sum::<i64>();
    let ledger = si_metrics::ledger::ledger(&chains, &all_rows, &paid);
    for l in &ledger {
        out!(
            "T{:<4} {:>2}/{:<3}   {:>5} {:>3} {:>8} {:>7} {:>10} {:>11} {:>7}   {:>5} {:>7}   {:>4}->{:<9} {}",
            l.turn,
            l.due_build,
            l.due_ravage,
            l.above,
            l.on,
            l.gambled,
            l.failed,
            l.untouched,
            l.sacrificed,
            l.blight,
            l.cards_spent,
            l.energy_spent,
            l.hand_after,
            l.discard_after,
            match (l.reclaimed, l.runway) {
                (true, _) => "reclaim".to_string(),
                (false, Some(r)) => format!("{r} turn{}", if r == 1 { "" } else { "s" }),
                (false, None) => "-".to_string(),
            }
        );
    }
    let handled: u32 = ledger.iter().map(si_metrics::LedgerTurn::handled).sum();
    let spent: u32 = ledger.iter().map(|l| l.cards_spent).sum();
    let (price_cards, price_energy): (u32, i64) = chains.iter().map(|c| (c.price_cards, c.price_energy)).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    if spent > 0 {
        out!("handled per card over the game: {handled} lands / {spent} cards = {:.2} | cards aimed at a chain's land: {price_cards} ({price_energy} Energy)", handled as f64 / spent as f64);
    }
    out!();
    out!("by Ravage turn:");
    for (turn, classes) in si_metrics::tempo::classes_by_turn(&chains) {
        out!("  T{:<3} {}", turn, classes.iter().map(|(c, n)| format!("{} {}", c.name(), n)).collect::<Vec<_>>().join(" | "));
    }
    let total = chains.iter().filter(|c| c.class != si_metrics::Class::Open).count().max(1);
    let count = |cl: si_metrics::Class| chains.iter().filter(|c| c.class == cl).count();
    out!(
        "  game  above {} | on {} | gambled {} | failed {} | sacrificed {} | untouched {} | late {}  (of {} chains)",
        count(si_metrics::Class::Above),
        count(si_metrics::Class::On),
        count(si_metrics::Class::Gambled),
        count(si_metrics::Class::Failed),
        count(si_metrics::Class::Sacrificed),
        count(si_metrics::Class::Untouched),
        count(si_metrics::Class::Late),
        total
    );
    out!();
    for sp in &rec.meta.spirits {
        let title = defs.as_ref().and_then(|d| d.spirits.get(sp)).map(|p| p.title.clone()).unwrap_or(sp.clone());
        let rows = si_metrics::track::rows(&rec, defs.as_ref(), sp);
        out!("TRACK TEMPO - {title}");
        out!("turn  growth   x/ycp   presence  cards  reclaim  played/plays  energy  hand  discard");
        for r in &rows {
            out!(
                "T{:<4} {:<8} {:<7} +{:<2}{:<5} +{:<4} {:<8} {}/{:<10} {:>4}  {:>4}  {:>7}",
                r.turn,
                r.growth,
                match (r.income, r.plays) {
                    (Some(i), Some(p)) => format!("{i}/{p}cp"),
                    _ => "?".into(),
                },
                r.presence_placed,
                format!("{}{}", if r.presence_other > 0 { format!("+{}b", r.presence_other) } else { String::new() }, if r.presence_destroyed > 0 { format!("(-{})", r.presence_destroyed) } else { String::new() }),
                r.cards_gained,
                if r.reclaimed { "reclaim" } else { "" },
                r.cards_played,
                r.plays.map(|p| p.to_string()).unwrap_or("?".into()),
                r.energy_after,
                r.hand,
                r.discard
            );
        }
        for spike in si_metrics::spikes::spikes_for(sp) {
            match si_metrics::track::spike_turn(&rows, &spike) {
                Some(t) => out!("spike {} reached T{}{}  - {}", spike.label, t, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
                None => out!("spike {} not reached{}  - {}", spike.label, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
            }
        }
        let weak = si_metrics::track::weak_turns(&rows);
        out!("weak turns {:?} | reclaim turns {:?} | reclaim loops {}", weak, si_metrics::track::reclaim_turns(&rows), si_metrics::track::reclaim_loops(&rows));
        let blight_by_t4: u32 = rec.turns.iter().filter(|t| t.turn >= 1 && t.turn <= 4).map(|t| t.blight_added + t.blight_cascaded).sum();
        let p = si_metrics::profile::profile(defs.as_ref(), sp, &rows, &chains, blight_by_t4);
        out!(
            "PROFILE  scaling {:?} (presence by T3 {}/{}, cards gained {}, first reclaim {}) | stance {:?} (above share {}) | blight by T4 {}",
            p.scaling,
            p.presence_by_t3.0,
            p.presence_by_t3.1,
            p.cards_gained_by_t3,
            p.first_reclaim.map(|t| format!("T{t}")).unwrap_or("none".into()),
            p.stance,
            p.above_share.map(|s| format!("{:.0}%", s * 100.0)).unwrap_or("-".into()),
            p.blight_by_t4
        );
        out!();
    }
    out!("GROWTH SHAPES - each series over the turns, binned by the simplest order that explains it (Lumia)");
    for (name, series) in growth_series(&rec) {
        match si_metrics::growth::fit(&series) {
            Some(f) => out!("  {:<28} {:<12} {}", name, f.order.name(), series.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(" ")),
            None => out!("  {:<28} {:<12} {}", name, "too short", series.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(" ")),
        }
    }
    Ok(())
}

/// The per-turn series a game record gives, for growth fitting.
fn growth_series(rec: &GameRecord) -> Vec<(String, Vec<f64>)> {
    let mut out = Vec::new();
    let turns: Vec<&si_record::TurnRecord> = rec.turns.iter().filter(|t| t.turn >= 1).collect();
    let mut blight = 0.0;
    out.push(("Blight on the island".into(), turns.iter().map(|t| { blight += (t.blight_added + t.blight_cascaded) as f64; blight }).collect()));
    out.push(("Invaders on the board".into(), turns.iter().map(|t| t.board_after.invaders() as f64).collect()));
    out.push(("lands with Invaders".into(), turns.iter().map(|t| t.lands_with_invaders_after as f64).collect()));
    let mut fear = 0.0;
    out.push(("Fear cards earned".into(), turns.iter().map(|t| { fear += t.fear_cards_earned as f64; fear }).collect()));
    for sp in &rec.meta.spirits {
        let rows: Vec<&si_record::SpiritTurn> = rec.spirits.iter().filter(|s| s.spirit == *sp && s.turn >= 1).collect();
        out.push((format!("{} Energy banked", short(sp)), rows.iter().map(|s| s.energy_after as f64).collect()));
        out.push((format!("{} presence on island", short(sp)), rows.iter().map(|s| s.presence_after.values().sum::<u32>() as f64).collect()));
        out.push((format!("{} income", short(sp)), rows.iter().map(|s| s.income.unwrap_or(0) as f64).collect()));
        out.push((format!("{} cards in hand", short(sp)), rows.iter().map(|s| s.hand_after.len() as f64).collect()));
    }
    out
}

/// One game, measured.
struct Row {
    source: String,
    exact: bool,
    adversary: Option<String>,
    level: String,
    difficulty: Option<(u8, u8)>,
    blight_shape: String,
    spirits: Vec<String>,
    boards: String,
    turns: u32,
    ending: Option<si_model::Ending>,
    scaling: si_metrics::Scaling,
    stance: si_metrics::Stance,
    presence_t3: (u32, u32),
    cards_t3: u32,
    first_reclaim: Option<u32>,
    spike: Option<(String, Option<u32>, Option<u32>)>,
    classes: std::collections::BTreeMap<si_metrics::Class, u32>,
    blight_t4: u32,
    blight_end: u32,
    fear_cards_t4: u32,
    weak: u32,
    signature: String,
}

fn games_under(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }
    if path.join("log-stream.txt").exists() {
        return vec![path.to_path_buf()];
    }
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(path) {
        let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        entries.sort();
        for e in entries {
            if e.is_dir() && e.join("log-stream.txt").exists() {
                out.push(e);
            } else if e.is_file() && e.file_name().is_some_and(|n| n.to_string_lossy().starts_with("EndGame--") && n.to_string_lossy().ends_with(".txt")) {
                out.push(e);
            }
        }
    }
    out
}

fn measure(rec: &GameRecord, defs: Option<&Definitions>, source: &str) -> Row {
    let chains = si_metrics::tempo::chains_with(rec, defs);
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let rows = si_metrics::track::rows(rec, defs, &spirit);
    let blight_t4: u32 = rec.turns.iter().filter(|t| (1..=4).contains(&t.turn)).map(|t| t.blight_added + t.blight_cascaded).sum();
    let blight_end: u32 = rec.turns.iter().filter(|t| t.turn >= 1).map(|t| t.blight_added + t.blight_cascaded).sum();
    let fear_cards_t4: u32 = rec.turns.iter().filter(|t| (1..=4).contains(&t.turn)).map(|t| t.fear_cards_earned).sum();
    let p = si_metrics::profile::profile(defs, &spirit, &rows, &chains, blight_t4);
    let spike = si_metrics::spikes::spikes_for(&spirit).first().map(|sp| (sp.label.clone(), sp.by_turn, si_metrics::track::spike_turn(&rows, sp)));
    let mut classes = std::collections::BTreeMap::new();
    for c in &chains {
        if c.class != si_metrics::Class::Open {
            *classes.entry(c.class).or_insert(0u32) += 1;
        }
    }
    // the same game recorded twice (a capture and its EndGame log) shares its
    // first revealed Invader cards and its ending; distinct cards, so an undo
    // re-reveal in a bare log does not split the pair
    let mut explores: Vec<String> = Vec::new();
    for c in rec.turns.iter().flat_map(|t| t.explore.iter()) {
        if !explores.contains(c) {
            explores.push(c.clone());
        }
        if explores.len() == 4 {
            break;
        }
    }
    let signature = format!("{}|{}|{:?}|{}", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.ending, explores.join(","));
    let est = defs.and_then(|d| si_metrics::adversary::estimate(rec, d));
    let from_save = rec.meta.adversary_level.map(|l| (l, rec.meta.difficulty));
    let blight_series = growth_series(rec).into_iter().find(|(n, _)| n == "Blight on the island").map(|(_, s)| s).unwrap_or_default();
    Row {
        source: source.to_string(),
        exact: rec.coverage.undo_unresolved == 0,
        adversary: rec.meta.adversary.clone(),
        level: match (&from_save, &est) {
            (Some((l, _)), _) => l.to_string(),
            (None, Some(e)) => e.label(),
            (None, None) => if rec.meta.adversary.is_some() { "?".into() } else { "-".into() },
        },
        difficulty: match (&from_save, &est) {
            (_, Some(e)) if e.difficulty.is_some() => e.difficulty,
            (Some((_, Some(d))), _) => Some((*d as u8, *d as u8)),
            _ => None,
        },
        blight_shape: si_metrics::growth::fit(&blight_series).map(|f| f.order.name()).unwrap_or("-".into()),
        spirits: rec.meta.spirits.clone(),
        boards: rec.meta.boards.join(""),
        turns: rec.meta.turns,
        ending: rec.meta.ending,
        scaling: p.scaling,
        stance: p.stance,
        presence_t3: p.presence_by_t3,
        cards_t3: p.cards_gained_by_t3,
        first_reclaim: p.first_reclaim,
        spike,
        classes,
        blight_t4,
        blight_end,
        fear_cards_t4,
        weak: si_metrics::track::weak_turns(&rows).len() as u32,
        signature,
    }
}

/// The games under `paths`, replayed: captures first, then the bare EndGame logs no
/// capture already holds (a log whose last actions a capture shares is the same game
/// twice; the capture, with its exact undo clock, wins). Games joined in progress or
/// shorter than two turns are skipped with a note on stderr.
/// `si fuzz`: the game logic's invariants on random legal lines over the corpus.
fn fuzz(args: &[String]) -> Result<()> {
    let Some(defs) = defs_from(args)? else { bail!("fuzz needs --defs <dir> or SI_DEFS_DIR") };
    let after = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let live = args.iter().any(|a| a == "--live");
    let opts = si_hindsight::fuzz::FuzzOptions { lines: after("--lines").and_then(|s| s.parse().ok()).unwrap_or(if live { 8 } else { 30 }), seed: after("--seed").and_then(|s| s.parse().ok()).unwrap_or(1) };
    let mut paths: Vec<PathBuf> = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if a.starts_with("--") || matches!(args.get(i.wrapping_sub(1)).map(|s| s.as_str()), Some("--defs" | "--lines" | "--seed")) {
            continue;
        }
        paths.extend(games_under(Path::new(a)));
    }
    if paths.is_empty() {
        paths.extend(games_under(&capture_root()));
        if !live {
            paths.extend(games_under(&steam_dir()));
        }
    }
    let json = args.iter().any(|a| a == "--json");
    if let Some(i) = args.iter().position(|a| a == "--case") {
        // a Power's case built again from the seed a violation named
        let power = args.get(i + 1).cloned().unwrap_or_default();
        let seed: u64 = args.get(i + 2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let id = defs.cards.get(&power).map(|c| c.id.clone()).or_else(|| defs.card_by_title(&power).map(|c| c.id.clone())).or_else(|| defs.spirits.values().flat_map(|s| s.innates.iter()).find(|x| x.id == power || x.title == power).map(|x| x.id.clone())).unwrap_or(power);
        out!("{}", si_rules::fuzz::explain_power_case(&defs, &id, seed).map_err(|e| anyhow::anyhow!(e))?);
        return Ok(());
    }
    if args.iter().any(|a| a == "--rules") {
        let cases = after("--lines").and_then(|s| s.parse().ok()).unwrap_or(40);
        return fuzz_rules(&defs, cases, opts.seed, json);
    }
    if live {
        let cal = calibration_from(args).unwrap_or_default();
        return fuzz_live(&defs, &paths, &opts, &cal, json);
    }
    if let Some(i) = args.iter().position(|a| a == "--use") {
        // one recorded use of a Power, played alone and explained
        let game = args.get(i + 1).cloned().unwrap_or_default();
        let turn: u32 = args.get(i + 2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let power = args.get(i + 3).cloned().unwrap_or_default();
        let (mut rec, _) = load_record(Path::new(&game), Some(&defs), None, false)?;
        apply_save(&mut rec, Path::new(&game));
        let id = defs.cards.get(&power).map(|c| c.id.clone()).or_else(|| defs.card_by_title(&power).map(|c| c.id.clone())).or_else(|| defs.spirits.values().flat_map(|s| s.innates.iter()).find(|x| x.id == power || x.title == power).map(|x| x.id.clone())).unwrap_or(power);
        for d in si_hindsight::fuzz::explain_use(&rec, &defs, turn, &id) {
            out!("{d}\n");
        }
        return Ok(());
    }
    let games = load_corpus(&paths, Some(&defs));
    let mut total = 0usize;
    let mut reports = Vec::new();
    // every Power use the games recorded, each played alone against the game's own result
    let mut uses: Vec<(String, si_hindsight::fuzz::UseCheck)> = Vec::new();
    for (p, _, rec) in &games {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        uses.extend(si_hindsight::fuzz::power_uses(rec, &defs).into_iter().map(|u| (name.clone(), u)));
    }
    for (p, _, rec) in &games {
        let r = si_hindsight::fuzz::fuzz_game(rec, &defs, &opts);
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let who = format!("{} vs {}", rec.meta.spirits.join("+"), rec.meta.adversary.as_deref().map(|a| format!("{a} {}", rec.meta.adversary_level.map(|l| l.to_string()).unwrap_or("?".into()))).unwrap_or("no adversary".into()));
        total += r.findings.len();
        if !json {
            let checks = si_hindsight::fuzz::by_check(&r);
            out!("{name:<28} {who:<60} {:>2} turns {:>5} lines  exact {}/{}  {}", r.turns, r.lines, r.exact.len(), r.exact.len() + r.inexact.len(), if checks.is_empty() { "clean".to_string() } else { checks.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ") });
            if args.iter().any(|a| a == "--boards") {
                for (t, d) in &r.inexact {
                    out!("    T{t} the actual line's notation ends the turn otherwise than the record (sim/record): {d}");
                }
            }
            for f in r.findings.iter().take(6) {
                out!("    T{} {}: {}  [{}]  seed {}", f.turn, f.check, f.detail, f.line, f.seed);
            }
            if r.findings.len() > 6 {
                out!("    ... {} more", r.findings.len() - 6);
            }
        }
        reports.push((name, who, r));
    }
    if json {
        let v: Vec<serde_json::Value> = reports.iter().map(|(n, w, r)| serde_json::json!({ "game": n, "who": w, "report": r })).collect();
        println!("{}", serde_json::to_string(&v)?);
    } else {
        let exact: usize = reports.iter().map(|(_, _, r)| r.exact.len()).sum();
        let measured: usize = reports.iter().map(|(_, _, r)| r.exact.len() + r.inexact.len()).sum();
        out!("{} games, {} turns, {} lines: {} finding(s); the actual line in notation ends {exact} of {measured} turns exactly as the record does (--boards lists the rest)", reports.len(), reports.iter().map(|(_, _, r)| r.turns).sum::<usize>(), reports.iter().map(|(_, _, r)| r.lines).sum::<usize>(), total);
        let exact_uses = uses.iter().filter(|(_, u)| u.diff.is_none()).count();
        out!("{exact_uses} of {} recorded Power uses, each played alone, leave the board as the game did (--powers: by Power)", uses.len());
        if args.iter().any(|a| a == "--powers") {
            let mut by: std::collections::BTreeMap<String, (usize, usize, bool, Option<String>)> = std::collections::BTreeMap::new();
            for (g, u) in &uses {
                let e = by.entry(u.power.clone()).or_insert((0, 0, u.simulated, None));
                e.0 += 1;
                match &u.diff {
                    None => e.1 += 1,
                    Some(d) if e.3.is_none() => e.3 = Some(format!("{g} T{} on {}: {d}", u.turn, u.target.as_ref().map(|l| l.to_string()).unwrap_or("-".into()))),
                    Some(_) => {}
                }
            }
            let mut rows: Vec<(&String, &(usize, usize, bool, Option<String>))> = by.iter().collect();
            rows.sort_by_key(|(_, (n, ok, _, _))| (*ok as i64 - *n as i64, -(*n as i64)));
            let mut tb = styled_table(terminal_width(), false, &["Power", "uses", "exact", "simulated", "first difference (simulated/game)"]);
            for (pw, (n, ok, sim, d)) in rows {
                let title = defs.cards.get(pw.as_str()).map(|c| c.title.clone()).or_else(|| defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == *pw).map(|i| format!("{} (innate)", i.title))).unwrap_or(pw.clone());
                tb.add_row(vec![title, n.to_string(), ok.to_string(), if *sim { "in full".into() } else { "in part".into() }, d.clone().unwrap_or_default()]);
            }
            out!("{tb}");
        }
    }
    if total > 0 {
        bail!("{total} finding(s)");
    }
    Ok(())
}

/// `si fuzz --rules`: every Power and every adversary level in the definitions, on random
/// states of the real boards, held to the rules' own laws.
fn args_stated() -> bool {
    std::env::args().any(|a| a == "--stated")
}

fn fuzz_rules(defs: &Definitions, cases: usize, seed: u64, json: bool) -> Result<()> {
    let r = si_rules::fuzz::fuzz_definitions(defs, cases, seed);
    if json {
        println!("{}", serde_json::to_string(&r)?);
    }
    let broken_powers: Vec<&si_rules::fuzz::PowerRun> = r.powers.iter().filter(|p| !p.broken.is_empty()).collect();
    let broken_levels: Vec<&si_rules::fuzz::AdversaryRun> = r.adversaries.iter().filter(|a| !a.broken.is_empty()).collect();
    if !json {
        let cards = r.powers.iter().filter(|p| p.spirit.is_none()).count();
        let simulated = r.powers.iter().filter(|p| p.simulated).count();
        out!("boards: {}", r.boards.join(" "));
        out!("{} Powers ({cards} Power Cards, {} innates), {} cases each: {} broke a law; {simulated} of them every effect simulated, {} in part", r.powers.len(), r.powers.len() - cards, cases, broken_powers.len(), r.powers.len() - simulated);
        out!("{} adversary levels and the base rules, {} cases each: {} broke a law", r.adversaries.len() - 1, cases, broken_levels.len());
        out!("{} printed sentences read as something other than what they say (--stated lists them)", r.stated.len());
        if args_stated() {
            let mut tb = styled_table(terminal_width(), false, &["Power", "law", "what was seen"]);
            for (who, law, detail) in &r.stated {
                tb.add_row(vec![who.clone(), law.clone(), detail.clone()]);
            }
            out!("{tb}");
        }
        if !broken_powers.is_empty() {
            let mut tb = styled_table(terminal_width(), false, &["Power", "law", "what was seen", "seed"]);
            for p in &broken_powers {
                let (seed, law, detail) = &p.broken[0];
                let who = match &p.spirit {
                    Some(sp) => format!("{} (innate, {sp})", p.title),
                    None => p.title.clone(),
                };
                tb.add_row(vec![who, law.clone(), detail.clone(), seed.to_string()]);
            }
            out!("{tb}");
        }
        if !broken_levels.is_empty() {
            let mut tb = styled_table(terminal_width(), false, &["adversary", "level", "law", "what was seen", "seed"]);
            for a in &broken_levels {
                let (seed, law, detail) = &a.broken[0];
                tb.add_row(vec![a.adversary.clone().unwrap_or("none".into()), a.level.to_string(), law.clone(), detail.clone(), seed.to_string()]);
            }
            out!("{tb}");
        }
    }
    let n = broken_powers.len() + broken_levels.len();
    if n > 0 {
        bail!("{n} broke a law");
    }
    Ok(())
}

/// `si fuzz --live`: the turn being played, cut at the first and last save of every open
/// phase on the path each capture's game took (the record from that save's own journal, the
/// seed from the save, as the live screen builds them), fuzzed as the screen scores plans.
fn fuzz_live(defs: &Definitions, dirs: &[PathBuf], opts: &si_hindsight::fuzz::FuzzOptions, cal: &si_hindsight::Calibration, json: bool) -> Result<()> {
    let mut total = 0usize;
    let mut reports = Vec::new();
    for dir in dirs.iter().filter(|d| d.join("saves").is_dir()) {
        let cps = checkpoints_of(dir)?;
        let mut cuts: std::collections::BTreeMap<(i64, String), (usize, usize)> = std::collections::BTreeMap::new();
        for (i, c) in cps.iter().enumerate().filter(|(_, c)| c.on_final_path) {
            cuts.entry((c.snap.turn_index, c.snap.phase.clone())).and_modify(|e| e.1 = i).or_insert((i, i));
        }
        let mut r = si_hindsight::fuzz::FuzzReport::default();
        let mut who = String::new();
        for (a, b) in cuts.into_values() {
            for i in if a == b { vec![a] } else { vec![a, b] } {
                let c = &cps[i];
                let Some(g) = si_steam::source::read_save(&c.path).ok().and_then(|b| si_steam::nrbf::decode(&b).ok()) else { continue };
                let mut rec = si_steam::journal::record(&g, Some(defs), &format!("{} (save journal)", c.path.display()));
                let seed = si_steam::seed::from_snapshot(&c.snap, &c.name);
                seed.apply_meta(&mut rec.meta);
                if !rec.meta.from_setup {
                    continue;
                }
                who = format!("{} vs {}", rec.meta.spirits.join("+"), rec.meta.adversary.as_deref().map(|a| format!("{a} {}", rec.meta.adversary_level.map(|l| l.to_string()).unwrap_or("?".into()))).unwrap_or("no adversary".into()));
                let Some(open) = si_hindsight::live::open_turn(std::sync::Arc::new(rec), defs, &seed, cal) else { continue };
                let at = format!("T{} {}, {}", open.turn, si_hindsight::live::phase_word(open.phase), c.name);
                let one = si_hindsight::fuzz::fuzz_open(&open, defs, opts, &at);
                r.turns += one.turns;
                r.lines += one.lines;
                r.findings.extend(one.findings);
            }
        }
        let name = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        total += r.findings.len();
        if !json {
            let checks = si_hindsight::fuzz::by_check(&r);
            out!("{name:<28} {who:<60} {:>3} saves {:>5} plans  {}", r.turns, r.lines, if checks.is_empty() { "clean".to_string() } else { checks.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ") });
            for f in r.findings.iter().take(6) {
                out!("    {} {}: {}  [{}]  seed {}", f.at, f.check, f.detail, f.line, f.seed);
            }
            if r.findings.len() > 6 {
                out!("    ... {} more", r.findings.len() - 6);
            }
        }
        reports.push((name, who, r));
    }
    if json {
        let v: Vec<serde_json::Value> = reports.iter().map(|(n, w, r)| serde_json::json!({ "game": n, "who": w, "report": r })).collect();
        println!("{}", serde_json::to_string(&v)?);
    } else {
        out!("{} captures, {} saves, {} plans: {} finding(s)", reports.len(), reports.iter().map(|(_, _, r)| r.turns).sum::<usize>(), reports.iter().map(|(_, _, r)| r.lines).sum::<usize>(), total);
    }
    if total > 0 {
        bail!("{total} finding(s)");
    }
    Ok(())
}

fn load_corpus(paths: &[PathBuf], defs: Option<&Definitions>) -> Vec<(PathBuf, String, GameRecord)> {
    let mut paths: Vec<PathBuf> = paths.to_vec();
    paths.sort_by_key(|p| !p.is_dir());
    let mut capture_lines: Vec<std::collections::HashSet<String>> = Vec::new();
    let mut out = Vec::new();
    for p in &paths {
        let (lines, undo, _) = match si_record::replay::load(p) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("skip {}: {e}", p.display());
                continue;
            }
        };
        let actions: Vec<&str> = lines.iter().map(|l| l.text.trim()).filter(|t| t.starts_with("DidPerformAction")).collect();
        if p.is_dir() {
            capture_lines.push(actions.iter().map(|t| t.to_string()).collect());
        } else {
            let tail: Vec<&str> = actions.iter().rev().take(40).copied().collect();
            if !tail.is_empty() && capture_lines.iter().any(|set| tail.iter().filter(|t| set.contains(**t)).count() * 10 >= tail.len() * 9) {
                eprintln!("skip {} (same game as a capture)", p.display());
                continue;
            }
        }
        let (mut rec, source) = match load_record(p, defs, None, false) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("skip {}: {e}", p.display());
                continue;
            }
        };
        let _ = undo;
        apply_save(&mut rec, p);
        if !rec.meta.from_setup || rec.meta.turns < 2 {
            eprintln!("skip {} (joined in progress or too short: {} turns)", p.display(), rec.meta.turns);
            continue;
        }
        out.push((p.clone(), source, rec));
    }
    out
}

/// Measure over the corpus what hindsight has no rule for, and write `calibration.json`:
/// the Event's and the Fear cards' mean ledger effect (the informed score's expectation
/// of the face-down cards), the next-turn weight (how much of an unattended threat
/// materializes once the Spirit plays), the spread of a turn's luck (the reading bands'
/// unit), and the ledger's separation of wins from losses. Two passes over the actual
/// lines only: the weight first, then the luck spread with it.
fn calibrate(args: &[String]) -> Result<()> {
    let Some(defs) = defs_from(args)? else { bail!("calibrate needs --defs <dir> or SI_DEFS_DIR") };
    let out_path = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(|| calibration_path(args));
    let dry = args.iter().any(|a| a == "--dry-run");
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if matches!(a.as_str(), "--defs" | "--out" | "--calibration") {
            skip = true;
            continue;
        }
        if a.starts_with("--") {
            continue;
        }
        paths.extend(games_under(Path::new(a)));
    }
    let mut roots: Vec<String> = args.iter().enumerate().filter(|(i, a)| !a.starts_with("--") && !matches!(args.get(i.wrapping_sub(1)).map(|s| s.as_str()), Some("--defs" | "--out" | "--calibration"))).map(|(_, a)| a.clone()).collect();
    if paths.is_empty() {
        paths.extend(games_under(&capture_root()));
        paths.extend(games_under(&steam_dir()));
        roots = vec![capture_root().display().to_string(), steam_dir().display().to_string()];
    }
    let games = load_corpus(&paths, Some(&defs));
    if games.is_empty() {
        bail!("no games found under {}", paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "));
    }
    let values = si_hindsight::Values::generic();
    // the Blight Card per turn for each capture (its saves), so a Blight Card loss counts too
    let pools: Vec<std::collections::BTreeMap<u32, (i64, bool)>> = games.iter().map(|(p, _, _)| if p.is_dir() { blight_pool_by_turn(p) } else { Default::default() }).collect();
    // `losses`: score a lost game; the constants are measured without (pieces only), the
    // ledger against results with
    let review_all = |cal: &si_hindsight::Calibration, losses: bool| -> Vec<(String, Option<si_model::game::Ending>, Vec<si_hindsight::TurnReview>)> {
        games
            .iter()
            .zip(&pools)
            .map(|((p, _, rec), pool)| {
                let opts = si_hindsight::ReviewOptions { growth: false, actual_only: true, calibration: Some(cal.clone()), blight_pool: pool.clone(), score_losses: losses, ..Default::default() };
                let r = si_hindsight::review_with(rec, &defs, &opts);
                (p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), rec.meta.ending, r.turns)
            })
            .collect()
    };
    let mean = |v: &[f64]| -> f64 { if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 } };
    let sd = |v: &[f64]| -> f64 {
        if v.len() < 2 {
            return 0.0;
        }
        let m = mean(v);
        (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
    };
    // pass 1: the next-turn weight, then the Event and Fear means fitted to the hidden effect
    // on each turn's actual line (least squares: Event indicator and Fear cards per Terror
    // Level, no intercept: a turn with neither has no hidden effect by definition)
    let base = si_hindsight::Calibration::default();
    let pass1 = review_all(&base, false);
    let samples: Vec<&si_hindsight::Sample> = pass1.iter().flat_map(|(_, _, ts)| ts.iter().map(|t| &t.sample)).collect();
    let ledger = |(b, d): (u32, u32)| -> f64 { b as f64 * values.blight.abs() + d as f64 * values.dahan.abs() };
    let (mut threatened, mut realized, mut next_n) = (0.0, 0.0, 0u32);
    for s in &samples {
        if let (Some(th), Some(ac)) = (s.next_threat, s.next_actual) {
            if ledger(th) > 0.0 {
                threatened += ledger(th);
                realized += ledger(ac);
                next_n += 1;
            }
        }
    }
    let next_weight = if threatened > 0.0 { (realized / threatened).clamp(0.05, 1.0) } else { base.next_weight };
    // the hidden effects are scored with the weight above, so fit them on a second pass
    let mut cal = si_hindsight::Calibration { source: String::new(), event_mean: 0.0, event_n: 0, fear_card_mean: [0.0; 3], fear_card_n: [0; 3], next_weight, next_n, luck_sd: base.luck_sd, luck_n: 0, model_sd: 0.0, model_n: 0 };
    let pass2 = review_all(&cal, false);
    let fit_rows: Vec<([f64; 4], f64)> = pass2
        .iter()
        .flat_map(|(_, _, ts)| ts.iter())
        .filter_map(|t| {
            let y = t.sample.hidden_effect?;
            let mut x = [0.0; 4];
            x[0] = if t.sample.event { 1.0 } else { 0.0 };
            x[t.sample.terror.clamp(1, 3) as usize] = t.sample.fear_cards as f64;
            Some((x, y))
        })
        .collect();
    let beta = least_squares(&fit_rows);
    cal.event_mean = beta[0];
    cal.event_n = fit_rows.iter().filter(|(x, _)| x[0] > 0.0).count() as u32;
    for tl in 1..=3usize {
        cal.fear_card_mean[tl - 1] = beta[tl];
        cal.fear_card_n[tl - 1] = fit_rows.iter().filter(|(x, _)| x[tl] > 0.0).count() as u32;
    }
    // pass 3: a turn's luck as the review defines it, with the means above
    let pass2 = review_all(&cal, false);
    let lucks: Vec<f64> = pass2.iter().flat_map(|(_, _, ts)| ts.iter().filter(|t| t.sample.hidden_effect.is_some()).map(|t| t.luck)).collect();
    cal.luck_sd = if lucks.len() >= 2 { sd(&lucks) } else { base.luck_sd };
    cal.luck_n = lucks.len() as u32;
    let errors: Vec<f64> = pass2.iter().flat_map(|(_, _, ts)| ts.iter().filter_map(|t| t.sample.model_error)).collect();
    // --turns: each turn's simulator error, to see which games and turns a change moved
    if args.iter().any(|a| a == "--turns") {
        for (name, _, ts) in &pass2 {
            for t in ts {
                if let Some(e) = t.sample.model_error {
                    out!("  model error  {name} T{} {e:+.2}", t.turn);
                }
            }
        }
    }
    cal.model_sd = if errors.is_empty() { 0.0 } else { (errors.iter().map(|e| e * e).sum::<f64>() / errors.len() as f64).sqrt() };
    cal.model_n = errors.len() as u32;
    let turns_total: usize = pass2.iter().map(|(_, _, ts)| ts.len()).sum();
    cal.source = format!("si calibrate {} (UTC): {} games, {turns_total} turns under {}", chrono_date(), games.len(), roots.join(", "));
    // the ledger against results: each game's actual lines summed, a lost game scored
    let pass_results = review_all(&cal, true);
    let mut wins: Vec<(String, f64)> = Vec::new();
    let mut losses: Vec<(String, f64)> = Vec::new();
    for (name, ending, ts) in &pass_results {
        let total: f64 = ts.iter().map(|t| t.actual.score).sum();
        let per_turn = if ts.is_empty() { 0.0 } else { total / ts.len() as f64 };
        match ending {
            Some(e) if e.won() => wins.push((name.clone(), per_turn)),
            Some(_) => losses.push((name.clone(), per_turn)),
            None => {}
        }
    }
    let pairs = wins.len() * losses.len();
    let concordant = wins.iter().flat_map(|w| losses.iter().map(move |l| (w.1 > l.1) as usize)).sum::<usize>();
    out!("CALIBRATION  {} games, {turns_total} turns", games.len());
    out!("  hidden effect fit         Event {:+.2} ({} turns with one); Fear card TL1 {:+.2} ({} turns)  TL2 {:+.2} ({})  TL3 {:+.2} ({}); least squares over {} turns of what the face-down cards did to the actual line", cal.event_mean, cal.event_n, cal.fear_card_mean[0], cal.fear_card_n[0], cal.fear_card_mean[1], cal.fear_card_n[1], cal.fear_card_mean[2], cal.fear_card_n[2], fit_rows.len());
    out!("  next-turn weight          {:.2} = {realized:.1} realized / {threatened:.1} threatened (Blight and Dahan in ledger units) over {next_n} turns", cal.next_weight);
    out!("  a turn's luck             sd {:.2}, mean {:+.2}, over {} turns (hindsight minus informed on the actual line)", cal.luck_sd, mean(&lucks), cal.luck_n);
    out!("  simulator error           rms {:.2}, mean {:+.2}, over {} turns (recorded state at the Slow phase minus the simulated Invader phase of the actual line, in ledger units)", cal.model_sd, mean(&errors), cal.model_n);
    out!("  ledger vs result          wins {} (mean {:+.2} per turn), losses {} (mean {:+.2} per turn); {} of {} win/loss pairs ordered right ({:.0}%)", wins.len(), mean(&wins.iter().map(|w| w.1).collect::<Vec<_>>()), losses.len(), mean(&losses.iter().map(|l| l.1).collect::<Vec<_>>()), concordant, pairs, if pairs > 0 { concordant as f64 / pairs as f64 * 100.0 } else { 0.0 });
    out!("  per game (ledger per turn): {}", wins.iter().map(|(n, v)| format!("W {n} {v:+.2}")).chain(losses.iter().map(|(n, v)| format!("L {n} {v:+.2}"))).collect::<Vec<_>>().join("; "));
    if dry {
        out!("--dry-run: {} not written", out_path.display());
    } else {
        std::fs::write(&out_path, serde_json::to_string_pretty(&cal)?)?;
        out!("written: {}", out_path.display());
    }
    Ok(())
}

/// Ordinary least squares for four coefficients (normal equations with a hair of ridge,
/// so a feature no turn has gets zero rather than a singular matrix).
fn least_squares(rows: &[([f64; 4], f64)]) -> [f64; 4] {
    let mut a = [[0.0f64; 4]; 4];
    let mut b = [0.0f64; 4];
    for (x, y) in rows {
        for i in 0..4 {
            b[i] += x[i] * y;
            for j in 0..4 {
                a[i][j] += x[i] * x[j];
            }
        }
    }
    for i in 0..4 {
        a[i][i] += 1e-6;
    }
    // Gaussian elimination with partial pivoting
    let mut m = [[0.0f64; 5]; 4];
    for i in 0..4 {
        m[i][..4].copy_from_slice(&a[i]);
        m[i][4] = b[i];
    }
    for col in 0..4 {
        let pivot = (col..4).max_by(|&i, &j| m[i][col].abs().partial_cmp(&m[j][col].abs()).unwrap()).unwrap();
        m.swap(col, pivot);
        let d = m[col][col];
        if d.abs() < 1e-12 {
            continue;
        }
        for j in col..5 {
            m[col][j] /= d;
        }
        for i in 0..4 {
            if i != col {
                let f = m[i][col];
                for j in col..5 {
                    m[i][j] -= f * m[col][j];
                }
            }
        }
    }
    [m[0][4], m[1][4], m[2][4], m[3][4]]
}

/// Today's date (UTC) as YYYY-MM-DD, without a date crate.
fn chrono_date() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let days = secs / 86400;
    // civil-from-days (Howard Hinnant)
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

fn corpus(args: &[String]) -> Result<()> {
    let defs = defs_from(args)?;
    let md = args.iter().any(|a| a == "--md");
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut skip = false;
    for (i, a) in args.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if a == "--defs" {
            skip = true;
            continue;
        }
        if a.starts_with("--") {
            continue;
        }
        let _ = i;
        paths.extend(games_under(Path::new(a)));
    }
    let mut rows: Vec<Row> = Vec::new();
    for (_, source, rec) in load_corpus(&paths, defs.as_ref()) {
        rows.push(measure(&rec, defs.as_ref(), &source));
    }
    // dedupe: keep the exact (capture) record of a game over its bare log
    rows.sort_by(|a, b| a.signature.cmp(&b.signature).then(b.exact.cmp(&a.exact)));
    rows.dedup_by(|a, b| a.signature == b.signature);
    rows.sort_by(|a, b| a.spirits.cmp(&b.spirits).then(a.source.cmp(&b.source)));
    let sep = if md { " | " } else { "  " };
    let name = |s: &str| defs.as_ref().and_then(|d| d.spirits.get(s)).map(|p| p.short_title.clone()).filter(|t| !t.is_empty()).unwrap_or(s.to_string());
    let header = ["spirit", "board", "adversary", "level", "diff", "result", "turns", "undo", "scaling", "stance", "pres T3", "cards T3", "reclaim", "spike", "above", "on", "gambled", "failed", "sacrificed", "untouched", "late", "blight T4", "blight end", "blight shape", "fear cards T4", "weak turns"];
    if md {
        out!("| {} |", header.join(" | "));
        out!("|{}|", header.iter().map(|_| "---").collect::<Vec<_>>().join("|"));
    } else {
        out!("{}", header.join(sep));
    }
    let count = |r: &Row, c: si_metrics::Class| r.classes.get(&c).copied().unwrap_or(0);
    for r in &rows {
        let cells = vec![
            r.spirits.iter().map(|s| name(s)).collect::<Vec<_>>().join("+"),
            r.boards.clone(),
            r.adversary.clone().map(|a| a.trim_start_matches("The").replace("KingdomOf", "").replace("TsardomOf", "").replace("Expedition", "").replace("Monarchy", "")).unwrap_or("-".into()),
            r.level.clone(),
            r.difficulty.map(|(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).unwrap_or("-".into()),
            r.ending.map(|e| if e.won() { "won" } else { "lost" }).unwrap_or("open").to_string(),
            r.turns.to_string(),
            if r.exact { "exact" } else { "heuristic" }.to_string(),
            format!("{:?}", r.scaling),
            format!("{:?}", r.stance),
            format!("{}/{}", r.presence_t3.0, r.presence_t3.1),
            r.cards_t3.to_string(),
            r.first_reclaim.map(|t| format!("T{t}")).unwrap_or("-".into()),
            r.spike.as_ref().map(|(l, by, got)| format!("{l} {}{}", got.map(|t| format!("T{t}")).unwrap_or("never".into()), by.map(|b| format!(" (target T{b})")).unwrap_or_default())).unwrap_or("-".into()),
            count(r, si_metrics::Class::Above).to_string(),
            count(r, si_metrics::Class::On).to_string(),
            count(r, si_metrics::Class::Gambled).to_string(),
            count(r, si_metrics::Class::Failed).to_string(),
            count(r, si_metrics::Class::Sacrificed).to_string(),
            count(r, si_metrics::Class::Untouched).to_string(),
            count(r, si_metrics::Class::Late).to_string(),
            r.blight_t4.to_string(),
            r.blight_end.to_string(),
            r.blight_shape.clone(),
            r.fear_cards_t4.to_string(),
            r.weak.to_string(),
        ];
        if md {
            out!("| {} |", cells.join(" | "));
        } else {
            out!("{}", cells.join(sep));
        }
    }
    out!();
    // groupings: wins by scaling, by stance, by blight at T4, by spike reached
    let finished: Vec<&Row> = rows.iter().filter(|r| r.ending.is_some()).collect();
    let won = |rs: &[&Row]| rs.iter().filter(|r| r.ending.is_some_and(|e| e.won())).count();
    out!("GROUPINGS ({} finished games; counts, not significance)", finished.len());
    for sc in [si_metrics::Scaling::Fast, si_metrics::Scaling::Minus, si_metrics::Scaling::Problematic, si_metrics::Scaling::Unknown] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| r.scaling == sc).collect();
        if !g.is_empty() {
            out!("  scaling {:?}: {} games, {} won, blight by T4 avg {:.1}", sc, g.len(), won(&g), g.iter().map(|r| r.blight_t4 as f64).sum::<f64>() / g.len() as f64);
        }
    }
    for st in [si_metrics::Stance::Above, si_metrics::Stance::On, si_metrics::Stance::Unknown] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| r.stance == st).collect();
        if !g.is_empty() {
            out!("  stance {:?}: {} games, {} won, blight by T4 avg {:.1}", st, g.len(), won(&g), g.iter().map(|r| r.blight_t4 as f64).sum::<f64>() / g.len() as f64);
        }
    }
    for (label, pred) in [("blight by T4 ≤ 2", Box::new(|r: &Row| r.blight_t4 <= 2) as Box<dyn Fn(&Row) -> bool>), ("blight by T4 3–4", Box::new(|r: &Row| (3..=4).contains(&r.blight_t4))), ("blight by T4 ≥ 5", Box::new(|r: &Row| r.blight_t4 >= 5))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            out!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    for (label, pred) in [("adversary difficulty ≤ 4 (lowest candidate)", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| a <= 4)) as Box<dyn Fn(&Row) -> bool>), ("adversary difficulty 5–7", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| (5..=7).contains(&a)))), ("adversary difficulty ≥ 8", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| a >= 8))), ("no adversary", Box::new(|r: &Row| r.adversary.is_none()))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            out!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    let mut shapes: std::collections::BTreeMap<String, Vec<&Row>> = Default::default();
    for r in &finished {
        shapes.entry(r.blight_shape.clone()).or_default().push(r);
    }
    for (shape, g) in &shapes {
        out!("  blight grows {shape}: {} games, {} won", g.len(), won(g));
    }
    let sacrificed = |r: &Row| count(r, si_metrics::Class::Sacrificed) + count(r, si_metrics::Class::Untouched);
    for (label, pred) in [("no sacrificed/untouched chains", Box::new(|r: &Row| sacrificed(r) == 0) as Box<dyn Fn(&Row) -> bool>), ("1–2 sacrificed/untouched", Box::new(|r: &Row| (1..=2).contains(&sacrificed(r)))), ("3+ sacrificed/untouched", Box::new(|r: &Row| sacrificed(r) >= 3))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            out!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    Ok(())
}

pub(crate) fn print_record(r: &GameRecord, defs: Option<&Definitions>, lands: bool) {
    let m = &r.meta;
    out!("{}", m.source);
    out!(
        "spirits {} | boards {} | turns {} | {} | {}",
        m.spirits.join("+"),
        m.boards.join(""),
        m.turns,
        m.ending.map(|e| e.words()).unwrap_or("no ending"),
        if m.from_setup { "from setup" } else { "joined in progress" }
    );
    let c = &r.coverage;
    out!(
        "coverage: {} typed, {} skipped ({}), {} noise | undo reloads {} ({} events dropped, {} unresolved, {} repaired) | log copies {}",
        c.typed_total(),
        c.skipped_total(),
        c.skipped.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "),
        c.noise,
        c.undo_reloads,
        c.undo_dropped,
        c.undo_unresolved,
        c.undo_repairs,
        c.copies
    );
    out!();
    out!("turn  ravage      build       explore     fear  cards  terror  blight(+casc)  E/T/C/D after  invaded");
    for t in &r.turns {
        out!(
            "{:<5} {:<10}  {:<10}  {:<10}  {:>4}  {:>5}  {:>6}  {:>5}(+{})      {:>2}/{:>2}/{:>2}/{:>2}       {}",
            if t.turn == 0 { "setup".to_string() } else { format!("T{}", t.turn) },
            t.ravage.join("+"),
            t.build.join("+"),
            t.explore.join("+"),
            t.fear_generated,
            t.fear_cards_earned,
            t.terror_after,
            t.blight_added,
            t.blight_cascaded,
            t.board_after.explorers,
            t.board_after.towns,
            t.board_after.cities,
            t.board_after.dahan,
            t.lands_with_invaders_after
        );
    }
    out!();
    for sp in &m.spirits {
        let title = defs.and_then(|d| d.spirits.get(sp)).map(|p| p.title.clone()).unwrap_or(sp.clone());
        out!("{title}");
        out!("turn  growth           x/ycp   presence placed            E +gain -paid =after  played                               gained/reclaimed/forgotten          hand  discard  powers (phase target)");
        for st in r.spirits.iter().filter(|s| s.spirit == *sp) {
            let growth = if st.growth_groups.is_empty() { "-".to_string() } else { st.growth_groups.iter().map(|g| format!("G{g}")).collect::<Vec<_>>().join("+") };
            let xy = match (st.income, st.plays) {
                (Some(i), Some(p)) => format!("{i}/{p}cp"),
                _ => "?".into(),
            };
            let changes = format!(
                "{}{}{}",
                if st.cards_gained.is_empty() { String::new() } else { format!("+{} ", st.cards_gained.join(",")) },
                if st.reclaimed { format!("reclaim({}) ", st.reclaimed_cards.len()) } else { String::new() },
                if st.cards_forgotten.is_empty() { String::new() } else { format!("forget {} ", st.cards_forgotten.join(",")) }
            );
            let powers: Vec<String> = st.powers_used.iter().map(|(ph, p, l)| format!("{}:{}{}", &ph.name()[..1], short(p), l.as_ref().map(|l| format!("@{l}")).unwrap_or_default())).collect();
            let mut placed: Vec<String> = st.presence_placed.iter().map(|(n, l)| format!("{}->{}", n.as_deref().map(slot_short).unwrap_or("board"), l.as_ref().map(|l| l.to_string()).unwrap_or("?".into()))).collect();
            placed.extend(st.presence_moved.iter().map(|(f, t)| format!("{f}>{t}")));
            out!(
                "{:<5} {:<16} {:<7} {:<26} {:>2} +{:<3} -{:<3} ={:<4}  {:<36} {:<35} {:>4}  {:>7}  {}",
                if st.turn == 0 { "setup".to_string() } else { format!("T{}", st.turn) },
                growth,
                xy,
                placed.join(" "),
                st.energy_after - st.energy_gained + st.energy_paid,
                st.energy_gained,
                st.energy_paid,
                st.energy_after,
                st.cards_played.iter().map(|c| short(c)).collect::<Vec<_>>().join(","),
                changes,
                st.hand_after.len(),
                st.discard_after.len(),
                powers.join(" ")
            );
        }
        out!();
    }
    if lands {
        out!("lands: visits (turn step card: +added blight/cascade dmg def dahan-lost inv-destroyed -> E/T/C/D after) and touches (turn phase by)");
        for (id, l) in &r.lands {
            if l.visits.is_empty() && l.touches.is_empty() && l.blight.is_empty() {
                continue;
            }
            out!("{}{}  end E/T/C/D/B {}/{}/{}/{}/{}", id, l.terrain.map(|t| format!(" {t}")).unwrap_or_default(), l.end.explorers, l.end.towns, l.end.cities, l.end.dahan, l.end.blight);
            for v in &l.visits {
                out!(
                    "   T{} {:<7} {:<10} +{:<8} blight {}/{} dmg {} def {} dahan-lost {} destroyed {} -> {}/{}/{}/{}",
                    v.turn,
                    v.step.map(|s| format!("{s:?}")).unwrap_or_default(),
                    v.card.clone().unwrap_or_default(),
                    v.added.iter().map(|k| format!("{k:?}")).collect::<Vec<_>>().join("+"),
                    v.blight,
                    v.cascade,
                    v.damage,
                    v.defend,
                    v.dahan_lost,
                    v.invaders_destroyed,
                    v.after.explorers,
                    v.after.towns,
                    v.after.cities,
                    v.after.dahan
                );
            }
            for t in &l.touches {
                let kind = match &t.kind {
                    TouchKind::Power => "power".to_string(),
                    TouchKind::Defend(n) => format!("defend {n}"),
                    TouchKind::Damage(n) => format!("damage {n}"),
                    TouchKind::Presence => "presence".to_string(),
                    TouchKind::Moved { out, into } => format!("moved out {out} in {into}"),
                    TouchKind::Recover(n) => format!("blight removed {n}"),
                };
                out!("   T{} {:<7} {:<22} {}", t.turn, t.phase.name(), kind, short(&t.by));
            }
        }
    }
}

/// `ShroudOfSilentMistCardPlays3` -> `P3`, `...Energy2` -> `E2`.
fn slot_short(id: &str) -> &str {
    if let Some(i) = id.find("CardPlays") {
        return &id[i + 4..];
    }
    if let Some(i) = id.find("Energy") {
        return &id[i..];
    }
    id
}

fn short(s: &str) -> String {
    let caps: String = s.chars().filter(|c| c.is_ascii_uppercase() || c.is_ascii_digit()).collect();
    if caps.len() >= 2 && s.chars().all(|c| c.is_ascii_alphanumeric()) { caps } else { s.chars().take(18).collect() }
}
