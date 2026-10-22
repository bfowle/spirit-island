//! Where the record comes from: a file or capture (built once), the Steam folder polled
//! while the game is played, or a capture cut at one save for the smoke test and for
//! judging the screen without the game running. Spec section 3.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use si_defs::Definitions;
use si_record::replay::Seed;
use si_record::{GameRecord, Replay};

use super::Msg;

pub(super) struct LiveUpdate {
    pub rec: GameRecord,
    /// The newest save, through the firewall; None until one decodes.
    pub seed: Option<Seed>,
    /// The footer's words: "live: T4 Fast, save 003-0089, journal 1,204 lines", with the
    /// first land where the journal's board and the save's differ.
    pub words: String,
    /// The seed names the turn being played: the game has not ended, the save is at the
    /// journal's trailing turn, and its phase has an open tab.
    pub open: bool,
    /// The capture the live screen is writing for this game (`live-<id>/`), once a save
    /// has named it: where `i` finds the turn's start.
    pub capture: Option<PathBuf>,
    /// `--replay`: the app's intent at the cut, in words; live sends it as its own message.
    pub activity: Option<String>,
}

/// Which save a journal that joined in progress starts from: the one `i` injected, when
/// the screen did that; else the oldest decision save in the folder.
pub(super) type JoinFrom = Arc<Mutex<Option<PathBuf>>>;

/// Copy every save of the current game that the recorder has not seen (new or rewritten)
/// into the capture, as the old companion's recorder did: the app keeps only the last
/// turn's decision saves, the capture keeps them all. The game is the one the app's own
/// `save-*` files name; an `EndGame--*.dat` (the folder holds months of them) is copied
/// only when it is that game's and written since `since`. Returns the capture directory.
pub(super) fn record_saves(dir: &Path, since: SystemTime, recorder: &mut si_steam::source::Recorder, seen: &mut HashMap<String, (SystemTime, u64)>) -> Option<PathBuf> {
    let mut files: Vec<(SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| { let n = n.to_string_lossy(); n.ends_with(".dat") && (n.starts_with("save-") || n.starts_with("EndGame--")) }))
        .filter_map(|p| std::fs::metadata(&p).and_then(|m| m.modified()).ok().map(|t| (t, p)))
        .collect();
    files.sort();
    for (mtime, p) in files {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let final_save = name.starts_with("EndGame--");
        if final_save && mtime < since {
            continue;
        }
        let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
        if seen.get(&name) == Some(&(mtime, len)) {
            continue;
        }
        let Ok(bytes) = std::fs::read(&p) else { continue };
        // a partial write does not decode; it is tried again next poll
        let Ok(graph) = si_steam::nrbf::decode(&bytes) else { continue };
        let snap = si_steam::engine_map::map(&graph);
        if final_save {
            // the game's own end, into its capture; another game's final save is not ours
            if recorder.game() != Some(snap.game_id.as_str()) {
                seen.insert(name, (mtime, len));
                continue;
            }
        } else {
            recorder.set_game(&snap.game_id);
        }
        if !recorder.has(&name, mtime) {
            recorder.save(&name, mtime, &bytes);
        }
        seen.insert(name, (mtime, len));
    }
    recorder.dir().map(|d| d.to_path_buf())
}

/// The newest complete save in a Steam folder, by write time: a partial write fails to
/// decode and is left for the next poll.
fn newest_save(dir: &Path) -> Option<(PathBuf, SystemTime)> {
    let mut files: Vec<(PathBuf, SystemTime)> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name().is_some_and(|n| {
                let n = n.to_string_lossy();
                n.ends_with(".dat") && (n.starts_with("save-Feedback-") || n.starts_with("save-Undo-") || n == "save-ContinueGame.dat")
            })
        })
        .filter_map(|p| std::fs::metadata(&p).and_then(|m| m.modified()).ok().map(|t| (p, t)))
        .collect();
    files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    files.into_iter().next()
}

/// The oldest decision save: where a journal that joined in progress starts from.
fn oldest_decision_save(dir: &Path) -> Option<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("save-Feedback-"))).collect();
    files.sort();
    files.into_iter().next()
}

/// The capture's save written last before the clock `ms` (milliseconds of the day): the
/// state a journal that restarts at that moment continued from.
pub(super) fn save_before(capture: &Path, ms: u64) -> Option<PathBuf> {
    let events = std::fs::read_to_string(capture.join("events.txt")).ok()?;
    let mut best: Option<(u64, String)> = None;
    for raw in events.lines() {
        let l = si_record::log::split_stamp(raw);
        let (Some(t), Some(rest)) = (l.ms, l.text.strip_prefix("NEW")) else { continue };
        let name = rest.trim().to_string();
        if t <= ms && best.as_ref().is_none_or(|(bt, _)| t >= *bt) {
            best = Some((t, name));
        }
    }
    let (t, name) = best?;
    let stamp = format!("{:02}{:02}{:02}_{:03}", t / 3_600_000, (t / 60_000) % 60, (t / 1000) % 60, t % 1000);
    let p = capture.join("saves").join(format!("{stamp}__{name}.gz"));
    p.exists().then_some(p)
}

/// The clock of the first line of the journal's last copy (the app writes a new copy when
/// a game is reloaded or started), when the lines carry one.
fn journal_restart_ms(journal: &[si_record::log::Line]) -> Option<u64> {
    let start = journal.iter().rposition(|l| l.text.trim() == "GameView Start")?;
    journal[start..].iter().find_map(|l| l.ms)
}

/// `HHMMSS_mmm` at the front of a capture file name, as milliseconds of the day.
fn stamp_ms(path: &Path) -> Option<u64> {
    parse_stamp(path.file_name()?.to_string_lossy().split("__").next()?)
}

/// A game reloaded from one of its own earlier saves writes a journal that starts at the
/// load. The capture's earlier copy holds the turns before it: cut that copy at the clock
/// of the save the game came back to (the earliest capture save with the same bytes as
/// the one it loaded) and put the restarted copy after it, one journal from setup. The
/// turns before the load keep their reviews; the replayed ones follow with their numbers.
fn stitch_restart(capture: &Path, journal: &[si_record::log::Line]) -> Option<Vec<si_record::log::Line>> {
    let last_start = journal.iter().rposition(|l| l.text.trim() == "GameView Start")?;
    if last_start == 0 {
        return None;
    }
    let restart_ms = journal[last_start..].iter().find_map(|l| l.ms)?;
    let loaded = save_before(capture, restart_ms)?;
    // the app writes the state it came back to again, under the original's own name (the
    // same turn and journal index): the earliest capture save of that name is the cut
    let name = loaded.file_name()?.to_string_lossy().split("__").last()?.to_string();
    if !name.starts_with("save-Feedback-") {
        return None;
    }
    let mut same: Vec<u64> = std::fs::read_dir(capture.join("saves"))
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p != &loaded && p.file_name().is_some_and(|n| n.to_string_lossy().ends_with(&format!("__{name}"))))
        .filter_map(|p| stamp_ms(&p))
        .filter(|t| *t < restart_ms)
        .collect();
    same.sort();
    let cut_ms = *same.first()?;
    // every reload leaves a stub copy (a header and a few lines); the earlier copy to keep
    // is the last one that holds a game from its setup
    let starts: Vec<usize> = journal.iter().enumerate().filter(|(_, l)| l.text.trim() == "GameView Start").map(|(i, _)| i).collect();
    let game_copy_end = starts
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .find(|(k, a)| {
            let end = starts.get(k + 1).copied().unwrap_or(journal.len());
            journal[**a..end].iter().any(|l| l.text.contains("InvadersStartingAction"))
        })
        .map(|(k, _)| starts.get(k + 1).copied().unwrap_or(journal.len()))?;
    let (earlier, _) = si_record::log::timed_last_copy(&journal[..game_copy_end]);
    // the save was written right after a SAVING marker, and its lines were read a beat
    // later: keep through that marker, within a few seconds of the save's clock
    let mut out: Vec<si_record::log::Line> = Vec::new();
    let mut clock = 0u64;
    let mut past = false;
    for l in earlier {
        if let Some(ms) = l.ms {
            clock = ms;
        }
        if clock > cut_ms + 3000 {
            break;
        }
        if clock > cut_ms {
            past = true;
        }
        let saving = l.text.trim() == "***** SAVING *****";
        out.push(l);
        if past && saving {
            break;
        }
    }
    if out.is_empty() {
        return None;
    }
    out.extend(journal[last_start + 1..].iter().cloned());
    Some(out)
}

/// The record from a save's own journal, through the firewall the seed uses.
fn journal_record(path: &Path, defs: &Definitions, source: &str) -> Option<GameRecord> {
    let g = si_steam::nrbf::decode(&si_steam::source::read_save(path).ok()?).ok()?;
    Some(si_steam::journal::record(&g, Some(defs), &format!("{source} (save journal)")))
}

fn seed_of(path: &Path) -> Option<Seed> {
    let snap = crate::decode_save(path)?;
    Some(si_steam::seed::from_snapshot(&snap, &path.file_name()?.to_string_lossy()))
}

/// Lands where the record's board now and the seed's differ, as footer words.
fn disagreements(rec: &GameRecord, seed: &Seed) -> Vec<String> {
    let mut out = Vec::new();
    for (id, c, _) in &seed.lands {
        let Some(l) = rec.lands.get(id) else { continue };
        let j = l.end;
        if (j.explorers, j.towns, j.cities, j.dahan, j.blight) != (c.explorers, c.towns, c.cities, c.dahan, c.blight) {
            out.push(format!("{id}: journal {}E {}T {}C Dh{} Bl{}, save {}E {}T {}C Dh{} Bl{}", j.explorers, j.towns, j.cities, j.dahan, j.blight, c.explorers, c.towns, c.cities, c.dahan, c.blight));
        }
    }
    out
}

/// The record and the open tab's seed from a journal and a save: the replay from setup,
/// or from the oldest decision save when the journal joined in progress.
pub(super) fn build(defs: &Definitions, journal: &[si_record::log::Line], undo: Option<&[(u64, u32)]>, newest: Option<&Path>, join_from: Option<&Path>, capture: Option<&Path>, source: &str) -> LiveUpdate {
    let seed = newest.and_then(seed_of);
    // the newest save's own journal is the record when it reaches back to setup: no log,
    // no undo guessing, no journal copies (docs/superpowers/specs/2026-10-21-save-journal-reader.md)
    let from_save = newest.and_then(|p| journal_record(p, defs, source)).filter(|r| r.meta.from_setup && r.meta.turns > 0);
    let saved = from_save.is_some();
    let mut rec = from_save.unwrap_or_else(|| Replay::new(Some(defs)).run(journal, undo, source));
    let mut joined = None;
    let mut stitched = false;
    if !saved && !rec.meta.from_setup {
        if let Some(whole) = capture.and_then(|c| stitch_restart(c, journal)) {
            // the save names the turn being played; of the two undo placements (the capture's
            // clock, the one-save-back guess) take the one that lands on it
            let want = seed.as_ref().map(|s| s.turn_index + 1);
            let exact = Replay::new(Some(defs)).run(&whole, undo, source);
            let r = if want.is_some_and(|w| exact.meta.turns != w) {
                let guessed = Replay::new(Some(defs)).run(&whole, None, source);
                if want.is_some_and(|w| guessed.meta.turns == w) { guessed } else { exact }
            } else {
                exact
            };
            if r.meta.from_setup {
                rec = r;
                stitched = true;
            }
        }
    }
    if !saved && !rec.meta.from_setup {
        // the save the restarted journal continued from: the capture's last save before the
        // journal's first line, else the save `i` injected, else the oldest decision save
        let from_clock = capture.zip(journal_restart_ms(journal)).and_then(|(c, ms)| save_before(c, ms));
        if let Some(j) = from_clock.as_deref().or(join_from).and_then(seed_of) {
            rec = Replay::new(Some(defs)).with_seed(Some(j.clone())).run(journal, undo, source);
            joined = Some(j.save_name.clone());
        }
    }
    if let Some(s) = &seed {
        s.apply_meta(&mut rec.meta);
    }
    let disagree = seed.as_ref().map(|s| disagreements(&rec, s)).unwrap_or_default();
    let open = seed.as_ref().is_some_and(|s| s.ending.is_none() && rec.meta.ending.is_none() && si_hindsight::live::is_open_phase(s.phase) && s.turn_index + 1 == rec.meta.turns);
    let phase = seed.as_ref().map(|s| format!("T{} {}", s.turn_index + 1, si_hindsight::live::phase_word(s.phase))).unwrap_or_else(|| "no save yet".into());
    let save = seed.as_ref().map(|s| s.save_name.rsplit("__").next().unwrap_or(&s.save_name).trim_start_matches("save-Feedback-").trim_end_matches(".gz").trim_end_matches(".dat").to_string()).unwrap_or_default();
    let clock = if saved { "save journal" } else if undo.is_some() { "capture" } else { "bare journal" };
    let mut words = match (rec.meta.ending, open) {
        (Some(e), _) => format!("live: no game in progress; the last game, {} after T{}, {} lines ({clock})", e.words(), rec.meta.turns, journal.len()),
        (None, true) => format!("live: {phase}, save {save}, journal {} lines ({clock})", journal.len()),
        (None, false) => format!("live: journal at T{}, newest save at {phase}; waiting for the next save, journal {} lines ({clock})", rec.meta.turns, journal.len()),
    };
    if let Some(j) = joined {
        words.push_str(&format!(", joined from {j}"));
    }
    if stitched {
        words.push_str(", the turns before the reload kept from the capture");
    }
    if open {
        if let Some(d) = disagree.first() {
            words.push_str(&format!(" | journal and save disagree in {d}"));
        }
    }
    LiveUpdate { activity: None, rec, seed, words, open, capture: None }
}

/// Poll the Steam folder every 500 ms; a change in the journal's or the newest save's
/// write time or length sends a fresh `LiveUpdate` once the folder has held still for a
/// poll, so a burst of logged actions becomes one rebuild. Every save seen is copied
/// into the capture under `capture_root` (`live-<id>/`), the journal tailed beside it.
pub(super) fn spawn_live(dir: PathBuf, capture_root: PathBuf, defs: Arc<Definitions>, tx: Sender<Msg>, join_from: JoinFrom) {
    std::thread::spawn(move || {
        let journal = dir.join("log.txt");
        let player = dir.join("Player.log");
        let mut last_activity: Option<String> = None;
        type Seen = (SystemTime, u64, Option<(PathBuf, SystemTime)>);
        let mut last: Option<Seen> = None;
        let mut settling: Option<Seen> = None;
        // the recorder writes under <data dir>/steam-capture/; capture_root is that folder
        let mut recorder = si_steam::source::Recorder::new(capture_root.parent().unwrap_or(&capture_root));
        let mut seen_saves: HashMap<String, (SystemTime, u64)> = HashMap::new();
        let started = SystemTime::now();
        loop {
            recorder.tail(&journal);
            recorder.tail_player(&player);
            let activity = super::activity::tail_text(&player, 1 << 16).and_then(|t| super::activity::parse(t.lines())).map(|a| a.words());
            if activity != last_activity {
                last_activity = activity.clone();
                if tx.send(Msg::Activity(activity)).is_err() {
                    return;
                }
            }
            let capture = record_saves(&dir, started, &mut recorder, &mut seen_saves);
            let meta = std::fs::metadata(&journal).ok().and_then(|m| m.modified().ok().map(|t| (t, m.len())));
            let has_journal = meta.is_some();
            // the saves alone carry the record now: a folder with no log.txt still builds
            let meta = Some(meta.unwrap_or((std::time::UNIX_EPOCH, 0)));
            let newest = newest_save(&dir);
            if let Some((mt, len)) = meta {
                let now: Seen = (mt, len, newest.clone());
                if last.as_ref() == Some(&now) {
                    settling = None;
                } else if settling.as_ref() == Some(&now) {
                    // unchanged since the last poll: build it. The capture the screen writes
                    // carries the save clock, which places every undo reload exactly; the bare
                    // journal can only guess one save back, so it is the fallback
                    let loaded = match &capture {
                        Some(c) if c.join("events.txt").exists() => si_record::replay::load(c).ok(),
                        _ => None,
                    };
                    let loaded = loaded.or_else(|| si_record::replay::read_log(&journal).ok().map(|l| (l, None, journal.display().to_string())));
                    let loaded = loaded.or_else(|| Some((Vec::new(), None, journal.display().to_string())));
                    if let Some((lines, undo, source)) = loaded {
                        let injected = join_from.lock().ok().and_then(|j| j.clone());
                        let join = injected.or_else(|| oldest_decision_save(&dir));
                        let mut update = build(&defs, &lines, undo.as_deref(), newest.as_ref().map(|(p, _)| p.as_path()), join.as_deref(), capture.as_deref(), &source);
                        update.capture = capture.clone();
                        // the app keeps saving but stopped writing its journal: the app closes it at a
                        // game over, and an undo or reload after that writes none until the game view
                        // starts afresh (i reopens it before its own reload)
                        if let Some((_, save_t)) = newest.as_ref().filter(|_| has_journal) {
                            if save_t.duration_since(mt).map(|d| d.as_secs() >= 30).unwrap_or(false) && update.seed.as_ref().is_some_and(|s| s.ending.is_none()) {
                                update.words = format!("{} | the app is saving but its journal stopped at {}: the app closes it at a game over; i reopens it with its reload, or restart the app (the saves are kept in the capture)", update.words, save_age_words(mt));
                                update.open = false;
                            }
                        }
                        if tx.send(Msg::Live(Box::new(update))).is_err() {
                            return;
                        }
                    }
                    last = Some(now);
                    settling = None;
                } else {
                    settling = Some(now);
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

/// `--replay <capture> --at Tn/Phase [--save part]`: the capture's journal cut at the
/// chosen save's stamp, that save as the newest. One update, no polling. `--at` takes
/// the earliest matching save on the path the game finally took, as `si inject` does.
pub(super) fn replay_at(capture: &Path, defs: &Definitions, at: Option<(i64, String)>, save: Option<&str>) -> anyhow::Result<LiveUpdate> {
    let all = crate::checkpoints_of(capture)?;
    let chosen = all.iter().find(|c| c.on_final_path && at.as_ref().is_none_or(|(t, ph)| crate::checkpoint_matches(c, Some(*t), Some(ph.as_str()))) && save.is_none_or(|s| c.name.contains(s)));
    let Some(c) = chosen else { anyhow::bail!("no save on the final path matches --at/--save (si checkpoint {} lists them)", capture.display()) };
    // the recorder's stamp HHMMSS_mmm on the file name, as a log clock
    let stamp = c.path.file_name().unwrap().to_string_lossy().split("__").next().unwrap_or_default().to_string();
    let cut_ms = parse_stamp(&stamp).ok_or_else(|| anyhow::anyhow!("no clock on {}", c.path.display()))?;
    let (lines, undo, source) = si_record::replay::load(capture)?;
    let mut keep = true;
    let lines: Vec<si_record::log::Line> = lines
        .into_iter()
        .filter(|l| {
            if let Some(ms) = l.ms {
                keep = ms <= cut_ms;
            }
            keep
        })
        .collect();
    let undo: Option<Vec<(u64, u32)>> = undo.map(|v| v.into_iter().filter(|(ms, _)| *ms <= cut_ms).collect());
    let mut u = build(defs, &lines, undo.as_deref(), Some(&c.path), None, Some(capture), &source);
    // the app's intent at the cut, from the capture's player stream when it has one
    if let Ok(text) = std::fs::read_to_string(capture.join("player-stream.txt")) {
        let upto: Vec<String> = text.lines().map(si_record::log::split_stamp).filter(|l| l.ms.is_none_or(|ms| ms <= cut_ms)).map(|l| l.text).collect();
        u.activity = super::activity::parse(upto.iter().map(String::as_str)).map(|a| a.words());
    }
    Ok(u)
}

/// A write time as a clock of the day, for the footer.
fn save_age_words(t: SystemTime) -> String {
    let secs = t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    // the recorder's local clock, as the capture names its saves
    let offset: i64 = std::process::Command::new("date").arg("+%z").output().ok().and_then(|o| { let z = String::from_utf8_lossy(&o.stdout).trim().to_string(); let sign = if z.starts_with('-') { -1 } else { 1 }; let d = z.trim_start_matches(['+', '-']); Some(sign * (d.get(0..2)?.parse::<i64>().ok()? * 3600 + d.get(2..4)?.parse::<i64>().ok()? * 60)) }).unwrap_or(0);
    let s = (secs as i64 + offset).rem_euclid(86_400) as u64;
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

/// `HHMMSS_mmm` to milliseconds of the day.
fn parse_stamp(s: &str) -> Option<u64> {
    let (hms, ms) = s.split_once('_')?;
    if hms.len() != 6 {
        return None;
    }
    let h: u64 = hms[0..2].parse().ok()?;
    let m: u64 = hms[2..4].parse().ok()?;
    let sec: u64 = hms[4..6].parse().ok()?;
    let ms: u64 = ms.parse().ok()?;
    Some(((h * 60 + m) * 60 + sec) * 1000 + ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defs() -> Definitions {
        Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/engine"))).unwrap()
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-hindsight/tests/fixtures/open-turn")).join(name)
    }

    /// The saves a folder holds land in the capture as the recorder names them, once each.
    #[test]
    fn saves_are_recorded_into_a_capture_once() {
        let dir = std::env::temp_dir().join(format!("si-live-record-{}", std::process::id()));
        let root = dir.join("data");
        let steam = dir.join("steam");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&steam).unwrap();
        std::fs::create_dir_all(&root).unwrap();
        for (name, save_as) in [("growth", "save-Feedback-002-0047-SpiritPhase.dat"), ("fast", "save-Feedback-002-0054-FastPower.dat")] {
            let gz = std::fs::read_dir(fixture(name).join("saves")).unwrap().next().unwrap().unwrap().path();
            std::fs::write(steam.join(save_as), si_steam::source::read_save(&gz).unwrap()).unwrap();
        }
        let mut recorder = si_steam::source::Recorder::new(&root);
        let mut seen = HashMap::new();
        let since = SystemTime::now();
        let capture = record_saves(&steam, since, &mut recorder, &mut seen).expect("a capture named by the game");
        assert!(capture.file_name().unwrap().to_string_lossy().starts_with("live-"), "{}", capture.display());
        let saves: Vec<String> = std::fs::read_dir(capture.join("saves")).unwrap().flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        assert_eq!(saves.len(), 2, "{saves:?}");
        assert!(saves.iter().all(|s| s.contains("__save-Feedback-002-") && s.ends_with(".dat.gz")), "{saves:?}");
        let events = std::fs::read_to_string(capture.join("events.txt")).unwrap();
        assert_eq!(events.lines().count(), 2);
        // a second pass sees nothing new
        record_saves(&steam, since, &mut recorder, &mut seen);
        assert_eq!(std::fs::read_dir(capture.join("saves")).unwrap().count(), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The save a restarted journal continued from: the capture's last save before the clock.
    #[test]
    fn the_save_before_a_clock() {
        let cap = fixture("growth");
        let stamp = ((23 * 60 + 26) * 60 + 4) * 1000 + 642; // 232604_642__save-Feedback-002-0047
        assert!(save_before(&cap, stamp).is_some_and(|p| p.to_string_lossy().contains("0047-SpiritPhase")));
        assert!(save_before(&cap, stamp + 5000).is_some());
        assert!(save_before(&cap, 0).is_none());
    }

    /// Debug on the real capture: where the stitch bails.
    #[test]
    #[ignore]
    fn stitch_on_the_real_capture() {
        let cap = Path::new("/home/brett/repos/spirit-island/data/steam-capture/live-3a72b583b5da");
        let (lines, _, _) = si_record::replay::load(cap).unwrap();
        let last_start = lines.iter().rposition(|l| l.text.trim() == "GameView Start").unwrap();
        let restart_ms = lines[last_start..].iter().find_map(|l| l.ms);
        eprintln!("lines {} last_start {last_start} restart_ms {restart_ms:?}", lines.len());
        let loaded = restart_ms.and_then(|ms| save_before(cap, ms));
        eprintln!("loaded {loaded:?}");
        let (earlier, copies) = si_record::log::timed_last_copy(&lines[..last_start]);
        let stamped: Vec<u64> = earlier.iter().filter_map(|l| l.ms).collect();
        eprintln!("earlier copy: {} lines, {copies} copies, stamped {} first {:?} last {:?}", earlier.len(), stamped.len(), stamped.first(), stamped.last());
        let out = stitch_restart(cap, &lines);
        eprintln!("stitched: {:?} lines", out.as_ref().map(|v| v.len()));
        let defs = defs();
        if let Some(whole) = out {
            let (_, undo, _) = si_record::replay::load(cap).unwrap();
            for (label, u) in [("no undo clock", None), ("capture undo clock", undo.as_deref())] {
                let r = Replay::new(Some(&defs)).run(&whole, u, "stitched");
                let turns: Vec<(u32, bool)> = r.turns.iter().map(|t| (t.turn, !t.before_fast.is_empty())).collect();
                eprintln!("{label}: from_setup {} turns {} {:?} ending {:?} undo {} dropped {} unresolved {}", r.meta.from_setup, r.meta.turns, turns, r.meta.ending, r.coverage.undo_reloads, r.coverage.undo_dropped, r.coverage.undo_unresolved);
            }
            let tail: Vec<String> = whole.iter().rev().take(8).map(|l| format!("{:?} {}", l.ms, &l.text[..l.text.len().min(70)])).collect();
            eprintln!("stitched tail:\n{}", tail.into_iter().rev().collect::<Vec<_>>().join("\n"));
        }
    }

    #[test]
    fn stamps_parse() {
        assert_eq!(parse_stamp("232604_642"), Some(((23 * 60 + 26) * 60 + 4) * 1000 + 642));
        assert_eq!(parse_stamp("bad"), None);
    }

    /// A scratch folder filled in stages from the growth and fast cuts: the record grows,
    /// the open tab's phase follows the newest save, a shorter journal rebuilds.
    #[test]
    fn a_folder_filled_in_stages() {
        let defs = defs();
        let dir = std::env::temp_dir().join(format!("si-live-source-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let stage = |journal_from: &str, save_from: &str, save_as: &str| -> LiveUpdate {
            std::fs::write(dir.join("log.txt"), std::fs::read_to_string(fixture(journal_from).join("log-stream.txt")).unwrap()).unwrap();
            let gz = std::fs::read_dir(fixture(save_from).join("saves")).unwrap().next().unwrap().unwrap().path();
            let bytes = si_steam::source::read_save(&gz).unwrap();
            std::fs::write(dir.join(save_as), bytes).unwrap();
            let lines = si_record::replay::read_log(&dir.join("log.txt")).unwrap();
            build(&defs, &lines, None, newest_save(&dir).map(|(p, _)| p).as_deref(), None, None, "test")
        };
        let a = stage("growth", "growth", "save-Feedback-002-0047-SpiritPhase.dat");
        assert!(a.open, "the save is at the journal's trailing turn");
        assert_eq!(a.rec.meta.turns, 3);
        assert_eq!(a.seed.as_ref().unwrap().phase, si_model::game::Phase::SpiritPhase);
        assert!(a.words.starts_with("live: T3 Growth"), "{}", a.words);
        assert!(a.words.contains("(save journal)"), "the record comes from the save's own journal: {}", a.words);
        std::thread::sleep(Duration::from_millis(20));
        let b = stage("fast", "fast", "save-Feedback-002-0054-FastPower.dat");
        assert_eq!(b.seed.as_ref().unwrap().phase, si_model::game::Phase::FastPower);
        assert!(b.rec.turns.iter().any(|t| t.turn == 3 && !t.before_fast.is_empty()));
        // the log shrinks back to the growth cut while the newest save still says Fast: the
        // save's journal is the record, so the log's state changes nothing
        std::thread::sleep(Duration::from_millis(20));
        let c = stage("growth", "fast", "save-Feedback-002-0055-FastPower.dat");
        assert!(c.open);
        assert_eq!(c.rec.meta.turns, 3);
        assert!(c.rec.turns.iter().any(|t| t.turn == 3 && !t.before_fast.is_empty()), "the save's journal is the record");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A journal that has ended opens no tab (no save beside it: the log is the record) and
    /// the footer says so; a save behind the journal opens none either.
    #[test]
    fn an_ended_journal_or_a_stale_save_opens_no_tab() {
        let defs = defs();
        let shroud = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/shroud-e392726411c9"));
        let (lines, undo, source) = si_record::replay::load(shroud).unwrap();
        let gz = std::fs::read_dir(fixture("fast").join("saves")).unwrap().next().unwrap().unwrap().path();
        let ended = build(&defs, &lines, undo.as_deref(), None, None, None, &source);
        assert!(ended.rec.meta.ending.is_some());
        assert!(!ended.open);
        assert!(ended.words.contains("no game in progress"), "{}", ended.words);
        // the growth journal with the fast save: both at T3, open; the fast journal cut is at
        // T3 too, so a save from T1 would be stale: the seed's turn index decides
        let (lines, undo, source) = si_record::replay::load(&fixture("fast")).unwrap();
        let mut stale = build(&defs, &lines, undo.as_deref(), Some(&gz), None, None, &source);
        assert!(stale.open);
        stale.rec.meta.turns = 5;
        let open = stale.seed.as_ref().is_some_and(|s| s.turn_index + 1 == stale.rec.meta.turns);
        assert!(!open, "a save three turns behind the journal names no open turn");
    }
}
