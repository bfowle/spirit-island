//! `si journal`: the record from a save's own journal, and its differences from the
//! log-built record of the same capture (the acceptance test of the journal reader).

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use si_record::record::GameRecord;
use si_record::replay::Replay;

use crate::{defs_from, print_record};

pub(crate) fn run(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("journal needs a capture directory or a save (.dat, .dat.gz)") };
    let path = PathBuf::from(path);
    let defs = defs_from(args)?;
    let (save, capture) = if path.is_dir() {
        let Some(s) = newest_save(&path) else { bail!("no saves under {}", path.join("saves").display()) };
        (s, Some(path.clone()))
    } else {
        (path.clone(), None)
    };
    let g = si_steam::nrbf::decode(&si_steam::source::read_save(&save)?)?;
    if args.iter().any(|a| a == "--entries") {
        for e in si_steam::journal::entries(&g) {
            out!("{}", e.words());
        }
        return Ok(());
    }
    let source = format!("{} (save journal)", save.display());
    let rec = si_steam::journal::record(&g, defs.as_ref(), &source);
    if args.iter().any(|a| a == "--json") {
        out!("{}", serde_json::to_string_pretty(&rec)?);
        return Ok(());
    }
    if args.iter().any(|a| a == "--check") {
        let snap = si_steam::engine_map::map(&g);
        let seed = si_steam::seed::from_snapshot(&snap, &save.file_name().unwrap_or_default().to_string_lossy());
        let diffs = si_steam::journal::check(&rec, &seed);
        out!("{}: {} turns, {}", source, rec.meta.turns, rec.meta.ending.map(|e| e.words().to_string()).unwrap_or("no ending".into()));
        if diffs.is_empty() {
            out!("the record's end matches the save's board on every land ({} lands)", seed.board().len());
        } else {
            for d in &diffs {
                out!("{d}");
            }
            out!("{} lands differ", diffs.len());
        }
        return Ok(());
    }
    if args.iter().any(|a| a == "--against-log") {
        let Some(c) = capture else { bail!("--against-log needs a capture directory (its log-stream.txt is the other record)") };
        let (lines, undo, log_source) = si_record::replay::load(&c)?;
        let from_log = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &log_source);
        let diffs = diff(&from_log, &rec);
        out!("{} vs {}", log_source, source);
        if diffs.is_empty() {
            out!("no differences: turns, ending, each turn's cards, Fear, Blight, board, each spirit's turn, each land's end");
        } else {
            for d in &diffs {
                out!("{d}");
            }
            out!("{} differences", diffs.len());
        }
        return Ok(());
    }
    print_record(&rec, defs.as_ref(), args.iter().any(|a| a == "--lands" || a == "--all"));
    Ok(())
}

/// The capture's last-written save (the recorder stamps names in write order).
pub(crate) fn newest_save(capture: &Path) -> Option<PathBuf> {
    // by write time, not name: the stamp on the name is a clock of the day and a game can cross midnight
    let mut saves: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(capture.join("saves"))
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains(".dat"))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    saves.sort();
    saves.pop().map(|(_, p)| p)
}

/// What differs between two records of the same game, field by field: `log` is the
/// record from log.txt, `journal` the one from the save.
pub(crate) fn diff(log: &GameRecord, journal: &GameRecord) -> Vec<String> {
    let mut out = Vec::new();
    fn field(out: &mut Vec<String>, name: &str, a: String, b: String) {
        if a != b {
            out.push(format!("{name}: log {a} | journal {b}"));
        }
    }
    field(&mut out, "turns", log.meta.turns.to_string(), journal.meta.turns.to_string());
    field(&mut out, "ending", format!("{:?}", log.meta.ending), format!("{:?}", journal.meta.ending));
    field(&mut out, "spirits", log.meta.spirits.join("+"), journal.meta.spirits.join("+"));
    field(&mut out, "boards", log.meta.boards.join(""), journal.meta.boards.join(""));
    field(&mut out, "adversary", format!("{:?}", log.meta.adversary), format!("{:?}", journal.meta.adversary));
    field(&mut out, "from_setup", log.meta.from_setup.to_string(), journal.meta.from_setup.to_string());
    let turns = log.meta.turns.max(journal.meta.turns);
    for t in 1..=turns {
        let (a, b) = (log.turn(t), journal.turn(t));
        let (Some(a), Some(b)) = (a, b) else {
            out.push(format!("T{t}: present in log {} | in journal {}", a.is_some(), b.is_some()));
            continue;
        };
        let v = |x: &[String]| x.join("+");
        field(&mut out, &format!("T{t} ravage"), v(&a.ravage), v(&b.ravage));
        field(&mut out, &format!("T{t} build"), v(&a.build), v(&b.build));
        field(&mut out, &format!("T{t} explore"), v(&a.explore), v(&b.explore));
        field(&mut out, &format!("T{t} high_immigration"), v(&a.high_immigration), v(&b.high_immigration));
        field(&mut out, &format!("T{t} fear_generated"), a.fear_generated.to_string(), b.fear_generated.to_string());
        field(&mut out, &format!("T{t} fear_cards_earned"), a.fear_cards_earned.to_string(), b.fear_cards_earned.to_string());
        field(&mut out, &format!("T{t} fear_cards"), v(&a.fear_cards), v(&b.fear_cards));
        field(&mut out, &format!("T{t} events"), v(&a.events), v(&b.events));
        field(&mut out, &format!("T{t} terror_after"), a.terror_after.to_string(), b.terror_after.to_string());
        field(&mut out, &format!("T{t} blight_added"), a.blight_added.to_string(), b.blight_added.to_string());
        field(&mut out, &format!("T{t} blight_cascaded"), a.blight_cascaded.to_string(), b.blight_cascaded.to_string());
        field(&mut out, &format!("T{t} board_after"), format!("{:?}", a.board_after), format!("{:?}", b.board_after));
        field(&mut out, &format!("T{t} lands_with_invaders_after"), a.lands_with_invaders_after.to_string(), b.lands_with_invaders_after.to_string());
        for (name, x, y) in [("before_fast", &a.before_fast, &b.before_fast), ("before_slow", &a.before_slow, &b.before_slow), ("before_invaders", &a.before_invaders, &b.before_invaders)] {
            for land in x.keys().chain(y.keys()).collect::<std::collections::BTreeSet<_>>() {
                let (p, q) = (x.get(land), y.get(land));
                if p != q {
                    out.push(format!("T{t} {name} {land}: log {} | journal {}", p.map(|c| format!("{c:?}")).unwrap_or("-".into()), q.map(|c| format!("{c:?}")).unwrap_or("-".into())));
                }
            }
        }
        field(&mut out, &format!("T{t} choices"), a.choices.len().to_string(), b.choices.len().to_string());
    }
    let spirits: std::collections::BTreeSet<(String, u32)> = log.spirits.iter().chain(journal.spirits.iter()).map(|s| (s.spirit.clone(), s.turn)).collect();
    for (sp, t) in spirits {
        let (a, b) = (log.spirit_turn(&sp, t), journal.spirit_turn(&sp, t));
        let (Some(a), Some(b)) = (a, b) else {
            out.push(format!("{sp} T{t}: present in log {} | in journal {}", a.is_some(), b.is_some()));
            continue;
        };
        let v = |x: &[String]| x.join("+");
        let pre = format!("{sp} T{t}");
        field(&mut out, &format!("{pre} growth_groups"), format!("{:?}", a.growth_groups), format!("{:?}", b.growth_groups));
        field(&mut out, &format!("{pre} presence_placed"), format!("{:?}", a.presence_placed), format!("{:?}", b.presence_placed));
        field(&mut out, &format!("{pre} presence_moved"), format!("{:?}", a.presence_moved), format!("{:?}", b.presence_moved));
        field(&mut out, &format!("{pre} presence_destroyed"), a.presence_destroyed.to_string(), b.presence_destroyed.to_string());
        field(&mut out, &format!("{pre} revealed_after"), format!("{:?}", a.revealed_after), format!("{:?}", b.revealed_after));
        field(&mut out, &format!("{pre} energy"), format!("+{} -{} = {}", a.energy_gained, a.energy_paid, a.energy_after), format!("+{} -{} = {}", b.energy_gained, b.energy_paid, b.energy_after));
        field(&mut out, &format!("{pre} cards_played"), v(&a.cards_played), v(&b.cards_played));
        field(&mut out, &format!("{pre} cards_gained"), v(&a.cards_gained), v(&b.cards_gained));
        field(&mut out, &format!("{pre} cards_forgotten"), v(&a.cards_forgotten), v(&b.cards_forgotten));
        field(&mut out, &format!("{pre} reclaimed"), format!("{} {:?}", a.reclaimed, a.reclaimed_cards), format!("{} {:?}", b.reclaimed, b.reclaimed_cards));
        field(&mut out, &format!("{pre} hand_after"), v(&a.hand_after), v(&b.hand_after));
        field(&mut out, &format!("{pre} discard_after"), v(&a.discard_after), v(&b.discard_after));
        field(&mut out, &format!("{pre} powers_used"), format!("{:?}", a.powers_used), format!("{:?}", b.powers_used));
        field(&mut out, &format!("{pre} presence_after"), format!("{:?}", a.presence_after), format!("{:?}", b.presence_after));
        field(&mut out, &format!("{pre} offered"), v(&a.offered), v(&b.offered));
    }
    for land in log.lands.keys().chain(journal.lands.keys()).collect::<std::collections::BTreeSet<_>>() {
        let (a, b) = (log.lands.get(land), journal.lands.get(land));
        match (a, b) {
            (Some(a), Some(b)) => {
                if a.end != b.end {
                    out.push(format!("{land} end: log {:?} | journal {:?}", a.end, b.end));
                }
            }
            _ => out.push(format!("{land}: in log {} | in journal {}", a.is_some(), b.is_some())),
        }
    }
    out
}
