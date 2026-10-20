//! `si replay <log|capture> [--defs <dir>] [--json]`
//!
//! Replays a digital-edition log into a record and prints it: turns, spirits,
//! lands, coverage. Definitions come from `--defs` or `SI_DEFS_DIR` (the
//! directory `si-steam-dump defs` writes).

mod color;

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

/// Print one painted line (see `color`).
macro_rules! out {
    () => { println!() };
    ($($arg:tt)*) => { crate::color::emit(&format!($($arg)*)) };
}
use si_defs::Definitions;
use si_record::{GameRecord, Replay, TouchKind};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    color::init(&args);
    let Some(cmd) = args.first() else { return usage() };
    match cmd.as_str() {
        "replay" => replay(&args[1..]),
        "report" => report(&args[1..]),
        "corpus" => corpus(&args[1..]),
        "cards" => cards(&args[1..]),
        "verify" => verify(&args[1..]),
        "notate" => notate(&args[1..]),
        "checkpoint" => checkpoint(&args[1..]),
        "inject" => inject(&args[1..]),
        "hindsight" => hindsight(&args[1..]),
        _ => usage(),
    }
}

fn usage() -> Result<()> {
    eprintln!("usage: si replay <EndGame.txt | capture dir> [--defs <dir>] [--json] [--lands]");
    eprintln!("       si report <EndGame.txt | capture dir> [--defs <dir>]        land tempo, track tempo, build profile");
    eprintln!("       si corpus <path>... [--defs <dir>] [--md]                  one row per game over captures and EndGame logs, then groupings");
    eprintln!("       si hindsight <EndGame.txt | capture dir> [--defs <dir>] [--turn n] [--top n] [--fixed-growth] [--try \"SLSF>C8 MI\"]   each turn's growth and plays against every legal line, scored on the Invader steps that followed; Event and Fear choices against their alternatives");
    eprintln!("       si notate <EndGame.txt | capture dir> [--defs <dir>] [--canonical]   the game in Spirit Island notation (moves, reveals, results)");
    eprintln!("       si checkpoint <capture dir…> [--turn n] [--phase SpiritPhase|FastPower|SlowPower|…] [--export <dir>] [--graph [--dot]]   the app's saves kept with a capture, by turn and phase; --export writes the chosen .dat; --graph draws the checkpoint DAG (undo branches as forks)");
    eprintln!("       si inject <capture dir> --turn n [--phase SpiritPhase] [--save <file part>] [--any-branch] [--name save-Undo-990.dat] [--timeout 90] [--fetch]   load that checkpoint (on the path the game finally took) into the RUNNING Steam app from WSL (frida-inject.exe, no restart; main menu or in-game)");
    eprintln!("       si verify <EndGame.txt | capture dir> [--defs <dir>] [--level n] [--all]   simulate each turn's Invader phase and compare with the log");
    eprintln!("       si cards [--defs <dir>] [--unparsed [n]] [--card <title>] [--features] [--fit]   effect-parser coverage; one card's parse; the feature table; the pricing fit");
    Ok(())
}

/// The Steam folder that holds the `EndGame--<id>` logs and saves.
fn steam_dir() -> PathBuf {
    std::env::var("SI_STEAM_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/mnt/c/Users/brett/AppData/LocalLow/Handelabra Games Inc_/Spirit Island"))
}

fn decode_save(dat: &Path) -> Option<si_steam::engine_map::EngineSnapshot> {
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
        let id = path.file_name()?.to_string_lossy().strip_prefix("live-")?.to_string();
        return save_index().get(&id).and_then(|p| decode_save(p));
    }
    let dat = path.with_extension("dat");
    if !dat.exists() {
        return None;
    }
    decode_save(&dat)
}

/// Fill the record's metadata from the save beside the log: the adversary
/// (when the log never named it), its level, the game difficulty, the scenario.
fn apply_save(rec: &mut GameRecord, path: &Path) -> Option<si_steam::engine_map::EngineSnapshot> {
    let snap = sibling_save(path)?;
    if let Some((id, level)) = snap.adversaries.first() {
        if rec.meta.adversary.is_none() {
            rec.meta.adversary = Some(id.clone());
        }
        rec.meta.adversary_level = level.map(|l| l as u8);
    }
    rec.meta.difficulty = snap.difficulty;
    rec.meta.scenario = snap.scenario.clone();
    Some(snap)
}

fn defs_from(args: &[String]) -> Result<Option<Definitions>> {
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

fn replay(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("replay needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
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
        let Some(c) = defs.cards.get(&name).or_else(|| defs.card_by_title(&name)) else { bail!("no card {name}") };
        let elements = if c.elements.is_empty() { "no elements".to_string() } else { c.elements.to_string() };
        out!("{}  ({})", c.title, c.id);
        out!("  {:?} Power · {} Energy · {:?} · {} · target {} · {}", c.kind, c.cost, c.speed, c.range.as_ref().map(|r| r.words()).unwrap_or("no Range".into()), c.target.words(), elements);
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
            out!("  threshold {} {}", t.elements, if t.title.is_empty() { String::new() } else { format!("— {}", t.title) });
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
    out!("cards that can act on a slot: explore {} · build {} · ravage {}", slots[0], slots[1], slots[2]);
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
    out!("PRICING FIT ({kind}) — cost ≈ Σ coefficient × feature, over {} fully parsed cards (R² {:.2})", cards.len(), si_metrics::fit::r_squared(&y, &res));
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

fn hindsight(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("hindsight needs a log file or capture directory") };
    let Some(defs) = defs_from(args)? else { bail!("hindsight needs --defs <dir> or SI_DEFS_DIR") };
    let only_turn: Option<u32> = args.iter().position(|a| a == "--turn").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok());
    let top: usize = args.iter().position(|a| a == "--top").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(3);
    let try_line: Option<String> = args.iter().position(|a| a == "--try").and_then(|i| args.get(i + 1)).cloned();
    let fixed_growth = args.iter().any(|a| a == "--fixed-growth");
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let mut rec = Replay::new(Some(&defs)).run(&lines, undo.as_deref(), &source);
    apply_save(&mut rec, Path::new(path));
    let codes = std::cell::RefCell::new(si_record::notation::Codes::new(Some(&defs), false));
    let progress = std::io::IsTerminal::is_terminal(&std::io::stderr());
    let opts = si_hindsight::ReviewOptions { growth: !fixed_growth, progress, ..Default::default() };
    if progress {
        eprintln!("hindsight: {} turns to review; each dot is one growth position tried with every play set (--fixed-growth keeps the growth as played)", rec.turns.iter().filter(|t| t.turn >= 1 && !t.before_fast.is_empty()).count());
    }
    let review = si_hindsight::review_with(&rec, &defs, &opts);
    let pool_at = blight_pool_by_turn(Path::new(path));
    out!("{}", rec.meta.source);
    out!("spirits {} | boards {} | adversary {} level {} | {}", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.adversary.clone().unwrap_or("none".into()), rec.meta.adversary_level.map(|l| l.to_string()).unwrap_or("?".into()), rec.meta.ending.map(|e| e.words()).unwrap_or("no ending"));
    if rec.coverage.undo_unresolved > 0 {
        out!("caution: bare log, {} undo reloads placed one save back; a capture would be exact", rec.coverage.undo_reloads);
    }
    out!("HINDSIGHT — each turn's growth and plays against every legal line from the same position, each followed by the turn's real Invader steps and the next turn's at half weight.");
    out!("Growth: every group, top or bottom slot, every legal placement, each offered card, the Reclaim. Plays: every Fast and Slow card and innate at every legal target.");
    out!("The Ravage and Build cards were face up when the choice was made: this is what was knowable, not luck. Event and Fear piece effects are applied to every line as the log shows them.");
    if fixed_growth {
        out!("--fixed-growth: the growth is taken as played; only the plays vary.");
    }
    out!("ledger: {}", review.values.source);
    out!("glyphs: {}", si_hindsight::GLYPH_LEGEND);
    for n in &review.notes {
        out!("note: {n}");
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
        out!("════ T{} ════  hand {}  ·  {} plays  ·  {} Energy", t.turn, hand.join(" "), t.plays, t.energy);
        // the Blight pool at the turn's start, when the capture kept that save: a line whose
        // Blight reaches it ends the game
        let pool = pool_at.get(&t.turn).copied();
        let pool_note = match pool {
            Some((p, true)) => format!("; Blight card flipped, {p} left: Bl{p} or more LOSES the game"),
            Some((p, false)) => format!("; Blight card not yet flipped, {p} left on it: Bl{p} or more FLIPS it"),
            None => String::new(),
        };
        out!("  LINES  ({} play sets over {} growth position{}; ranked by the informed score{})", t.candidates, t.positions, if t.positions == 1 { "" } else { "s" }, pool_note);
        out!("    {:<6} {:<36} {:<42} {:>8} {:>8} {:>6} {:>5}  {:<23} {:<16} {}", "", "growth", "plays (cards | innate …)", "informed", "hindsight", "luck", "conf", "elements S M F A W E P A", "innate tiers", "produced (hindsight)");
        // a column never grows past its width, so the table stays aligned; the full notation is in si notate
        let fit = |s: &str, w: usize| -> String {
            if s.chars().count() <= w { s.to_string() } else { format!("{}…", s.chars().take(w - 1).collect::<String>()) }
        };
        let row = |tag: &str, l: &si_hindsight::Line| {
            let growth = l.growth_notation(&|id| code(id));
            let growth = if l.growth_actual && !l.actual { format!("{growth} (as played)") } else { growth };
            let growth = if l.ties > 0 { format!("{growth} +{} alike", l.ties) } else { growth };
            let conf = l.confidence.map(|c| format!("{:.0}%", c * 100.0)).unwrap_or_else(|| "-".into());
            let tiers: Vec<String> = l.tiers.iter().map(|(id, reached, of)| format!("{}{}", code(id), (0..*of).map(|i| if i < *reached { '●' } else { '○' }).collect::<String>())).collect();
            let mark = match pool {
                Some((p, true)) if p > 0 && l.outcome.blight as i64 >= p => "  LOSES",
                Some((p, false)) if p > 0 && l.outcome.blight as i64 >= p => "  FLIPS",
                _ => "",
            };
            out!("    {tag} {:<36} {:<42} {:>+8.2} {:>+8.2} {:>+6.2} {:>5}  {:<23} {:<16} {}{}", fit(&growth, 36), fit(&l.notation(&|id| code(id)), 42), l.informed, l.score, l.luck, conf, l.elements, fit(&tiers.join(" "), 16), l.outcome.summary(), mark);
        };
        row("actual", &t.actual);
        let mut shown = 0;
        for l in t.ranked.iter() {
            if l.actual {
                continue;
            }
            if shown >= top {
                break;
            }
            row(if shown == 0 { "best  " } else { "then  " }, l);
            shown += 1;
        }
        if t.gap_growth > 0.25 {
            if let Some(l) = t.ranked.iter().find(|l| l.growth_actual && !l.actual) {
                row("same G", l);
            }
        }
        out!("  READING  {}", t.reading);
        out!("           informed = expected on what was knowable (the Explore cards still in the stage, Event and Fear effects unknown, a later draft's card not played); hindsight = on the cards that came; luck = hindsight − informed; conf = share of possible Explore cards under which the line beats the actual");
        if !t.choices.is_empty() {
            let scored: Vec<&si_hindsight::ChoiceReview> = t.choices.iter().filter(|c| c.gap.is_some()).collect();
            let unscored: Vec<&si_hindsight::ChoiceReview> = t.choices.iter().filter(|c| c.gap.is_none()).collect();
            out!("  CHOICES THE GAME ASKED FOR  (Event and Fear steps; each land scored with your growth and plays kept)");
            for c in &scored {
                let opts: Vec<String> = c.options.iter().map(|(o, sc)| match sc {
                    Some(v) if *o == c.selected => format!("{o} {v:+.2} ←chosen"),
                    Some(v) => format!("{o} {v:+.2}"),
                    None => o.clone(),
                }).collect();
                let verdict = match c.gap {
                    Some(g) if g <= 0.01 => "best option".to_string(),
                    Some(g) if g <= 0.5 => format!("within {g:.2} of best"),
                    Some(g) => format!("better land open, by {g:.2}"),
                    None => String::new(),
                };
                let effect = c.note.split(" placed in each land").next().unwrap_or("").to_string();
                out!("    {:<5} {:<34} {:<26} {:<28} {}", c.source_kind, c.source, effect, verdict, opts.join("  "));
            }
            if !unscored.is_empty() {
                out!("    also asked (not scored yet):");
                for c in &unscored {
                    let opts: Vec<String> = c.options.iter().map(|(o, _)| if *o == c.selected { format!("{o} ←chosen") } else { o.clone() }).collect();
                    out!("      {:<5} {:<34} {:<26} {}", c.source_kind, c.source, c.reason, opts.join("  "));
                }
            }
        }
        out!("  HIDDEN WHEN YOU CHOSE");
        for h in &t.hidden {
            let h = if let Some(rest) = h.strip_prefix("the gain offered ") {
                let (cards, tail) = rest.split_once(';').unwrap_or((rest, ""));
                let coded: Vec<String> = cards.split(", ").map(|c| code(c.trim())).collect();
                format!("gain offered {}{}", coded.join(" "), if tail.is_empty() { String::new() } else { format!(";{tail}") })
            } else if let Some(rest) = h.strip_prefix("the Event (") {
                let (id, tail) = rest.split_once(')').unwrap_or((rest, ""));
                format!("Event: {}{}", spaced(id), tail)
            } else {
                h.clone()
            };
            out!("    {h}");
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
    let legend = codes.borrow().legend();
    if !legend.is_empty() {
        out!();
        out!("{{{legend}}}");
    }
    Ok(())
}

/// One of the app's saves kept with a capture, decoded.
struct Checkpoint {
    path: PathBuf,
    /// The file name the app wrote (`save-Feedback-002-0047-SpiritPhase.dat`).
    name: String,
    snap: si_steam::engine_map::EngineSnapshot,
    /// sha256 of the save bytes: the checkpoint's id. Copies share it; a reload that
    /// lands on an existing id is a fork in the game's DAG.
    id: String,
    /// On the path the game finally took (an undone attempt is off it).
    on_final_path: bool,
}

fn checkpoints_of(dir: &Path) -> Result<Vec<Checkpoint>> {
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

fn checkpoint_matches(c: &Checkpoint, turn: Option<i64>, phase: Option<&str>) -> bool {
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
    out!("CHECKPOINT DAG — {} capture(s), {} distinct saves, {} root(s), {} fork(s), {} leaf/leaves", dirs.len(), nodes.len(), roots, forks.len(), leaves.len());
    out!("ids are the first 12 hex digits of the save's sha256; a fork is a save the game came back to (undo, or a live reload) and left again");
    for id in &order {
        let n = &nodes[id];
        let mark = if n.children.len() > 1 { "⑂" } else if n.children.is_empty() { "●" } else { "│" };
        out!("  {mark} {id}  T{:<2} {:<22} {:<48} ← {}  {}", n.turn, n.state, n.label, n.parents.join(","), if n.seen_in.len() > 1 { format!("in {}", n.seen_in.join(" + ")) } else { String::new() });
    }
    if !forks.is_empty() {
        out!("forks:");
        for f in forks {
            let n = &nodes[f];
            out!("  {f} T{} {} → {} branches: {}", n.turn, n.state, n.children.len(), n.children.join(" "));
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
    out!("attaching frida-inject to SpiritIsland.exe (up to {timeout}s). Keep the game window focused: Unity stops running its frames in the background.");
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
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let mut rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
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
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let mut rec = Replay::new(Some(&defs)).run(&lines, undo.as_deref(), &source);
    apply_save(&mut rec, Path::new(path));
    // the save beside the log states the level; else the lowest consistent
    // candidate (level rules are cumulative, so its rules hold regardless)
    let from_save = rec.meta.adversary_level;
    let est = si_metrics::adversary::estimate(&rec, &defs);
    let level = level.or(from_save).or_else(|| est.as_ref().and_then(|e| e.candidates.first().copied()));
    let r = si_rules::verify::verify(&rec, &defs, level);
    out!("{}", rec.meta.source);
    out!("spirits {} | boards {} | adversary {} level {} | {} turns simulated", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.adversary.clone().unwrap_or("none".into()), match (from_save, &est, level) { (Some(l), _, _) => format!("{l} (from the save)"), (None, Some(e), Some(l)) if e.candidates.len() > 1 => format!("{l} (lowest of {})", e.label()), (_, _, Some(l)) => l.to_string(), _ => "?".into() }, r.turns);
    out!("INVADER PHASE and SPIRIT ACTIONS — simulated from the recorded state at each phase start, compared with the log");
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
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let mut rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
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
            "adversary level {} (difficulty {}) — {}",
            est.label(),
            est.difficulty.map(|(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).unwrap_or("?".into()),
            est.evidence.join("; ")
        );
    }
    out!();
    out!("LAND TEMPO — each Invader card's pass over a land: Explore t → Build t+1 → Ravage t+2");
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
                si_metrics::Clock::AfterExplore => "X→B",
                si_metrics::Clock::AfterBuild => "B→R",
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
    out!("LEDGER — per turn: lands due, how the Ravage-due ones ended, and what the spirits paid (all spirits)");
    out!("turn  due B/R   above  on  gambled  failed  untouched  sacrificed  blight   cards  energy   hand→discard  runway");
    let mut all_rows: Vec<si_metrics::TrackRow> = Vec::new();
    for sp in &rec.meta.spirits {
        all_rows.extend(si_metrics::track::rows(&rec, defs.as_ref(), sp));
    }
    let paid = |turn: u32| rec.spirits.iter().filter(|s| s.turn == turn).map(|s| s.energy_paid).sum::<i64>();
    let ledger = si_metrics::ledger::ledger(&chains, &all_rows, &paid);
    for l in &ledger {
        out!(
            "T{:<4} {:>2}/{:<3}   {:>5} {:>3} {:>8} {:>7} {:>10} {:>11} {:>7}   {:>5} {:>7}   {:>4}→{:<9} {}",
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
        out!("handled per card over the game: {handled} lands / {spent} cards = {:.2} · cards aimed at a chain's land: {price_cards} ({price_energy} Energy)", handled as f64 / spent as f64);
    }
    out!();
    out!("by Ravage turn:");
    for (turn, classes) in si_metrics::tempo::classes_by_turn(&chains) {
        out!("  T{:<3} {}", turn, classes.iter().map(|(c, n)| format!("{} {}", c.name(), n)).collect::<Vec<_>>().join(" · "));
    }
    let total = chains.iter().filter(|c| c.class != si_metrics::Class::Open).count().max(1);
    let count = |cl: si_metrics::Class| chains.iter().filter(|c| c.class == cl).count();
    out!(
        "  game  above {} · on {} · gambled {} · failed {} · sacrificed {} · untouched {} · late {}  (of {} chains)",
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
        out!("TRACK TEMPO — {title}");
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
                Some(t) => out!("spike {} reached T{}{}  — {}", spike.label, t, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
                None => out!("spike {} not reached{}  — {}", spike.label, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
            }
        }
        let weak = si_metrics::track::weak_turns(&rows);
        out!("weak turns {:?} · reclaim turns {:?} · reclaim loops {}", weak, si_metrics::track::reclaim_turns(&rows), si_metrics::track::reclaim_loops(&rows));
        let blight_by_t4: u32 = rec.turns.iter().filter(|t| t.turn >= 1 && t.turn <= 4).map(|t| t.blight_added + t.blight_cascaded).sum();
        let p = si_metrics::profile::profile(defs.as_ref(), sp, &rows, &chains, blight_by_t4);
        out!(
            "PROFILE  scaling {:?} (presence by T3 {}/{}, cards gained {}, first reclaim {}) · stance {:?} (above share {}) · blight by T4 {}",
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
    out!("GROWTH SHAPES — each series over the turns, binned by the simplest order that explains it (Lumia)");
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
    // A bare EndGame log of a game that a capture also holds is the same game
    // twice: the capture (exact undo clock) wins. Captures first.
    paths.sort_by_key(|p| !p.is_dir());
    let mut capture_lines: Vec<std::collections::HashSet<String>> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    for p in &paths {
        let (lines, undo, source) = match si_record::replay::load(p) {
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
        let mut rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
        apply_save(&mut rec, p);
        if !rec.meta.from_setup || rec.meta.turns < 2 {
            eprintln!("skip {} (joined in progress or too short: {} turns)", p.display(), rec.meta.turns);
            continue;
        }
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

fn print_record(r: &GameRecord, defs: Option<&Definitions>, lands: bool) {
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
            let mut placed: Vec<String> = st.presence_placed.iter().map(|(n, l)| format!("{}→{}", n.as_deref().map(slot_short).unwrap_or("board"), l.as_ref().map(|l| l.to_string()).unwrap_or("?".into()))).collect();
            placed.extend(st.presence_moved.iter().map(|(f, t)| format!("{f}⇢{t}")));
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
        out!("lands: visits (turn step card: +added blight/cascade dmg def dahan-lost inv-destroyed → E/T/C/D after) and touches (turn phase by)");
        for (id, l) in &r.lands {
            if l.visits.is_empty() && l.touches.is_empty() && l.blight.is_empty() {
                continue;
            }
            out!("{}{}  end E/T/C/D/B {}/{}/{}/{}/{}", id, l.terrain.map(|t| format!(" {t}")).unwrap_or_default(), l.end.explorers, l.end.towns, l.end.cities, l.end.dahan, l.end.blight);
            for v in &l.visits {
                out!(
                    "   T{} {:<7} {:<10} +{:<8} blight {}/{} dmg {} def {} dahan-lost {} destroyed {} → {}/{}/{}/{}",
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

/// `ShroudOfSilentMistCardPlays3` → `P3`, `…Energy2` → `E2`.
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
