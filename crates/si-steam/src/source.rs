//! Snapshot sources. Both run on their own thread and publish
//! `CompanionState` through a `watch` channel.
//!
//! - **Live**: polls the game's save folder (WSL gets no inotify events for
//!   Windows writes on `/mnt/c`). The most recently written save, main or
//!   undo, is the truth. Polls `tasklist.exe` for the process.
//! - **Replay**: steps through a capture folder (`saves/<HHMMSS_mmm>__<name>.dat`)
//!   in capture order, with play / pause / step / seek. Reads local files
//!   only; it never touches the game or tabletop state.

use crate::companion::{Companion, CompanionState, ReplayStatus, SourceStatus};
use crate::engine_defs::EngineDefs;
use crate::{engine_map, log_parse, nrbf, projection};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::Read;
use si_common::public::PublicSnapshot;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::watch;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub enum ReplayCommand {
    Play { speed: f64 },
    Pause,
    Step(i64),
    Seek(usize),
}

pub struct SourceHandle {
    pub state: watch::Receiver<Arc<CompanionState>>,
    pub control: Option<mpsc::Sender<ReplayCommand>>,
}

pub enum SourceConfig {
    Live { dir: PathBuf, poll: Duration, data_dir: PathBuf },
    Replay { capture: PathBuf },
    Off,
}

pub fn spawn(config: SourceConfig, defs: Arc<EngineDefs>) -> SourceHandle {
    let (tx, rx) = watch::channel(Arc::new(CompanionState::default()));
    match config {
        SourceConfig::Off => {
            let status = SourceStatus { mode: "off".into(), process: "unknown".into(), ..Default::default() };
            tx.send_replace(Arc::new(CompanionState { source: status, ..Default::default() }));
            SourceHandle { state: rx, control: None }
        }
        SourceConfig::Live { dir, poll, data_dir } => {
            std::thread::Builder::new()
                .name("si-steam-live".into())
                .spawn(move || live_loop(dir, poll, data_dir, defs, tx))
                .expect("spawn live source");
            SourceHandle { state: rx, control: None }
        }
        SourceConfig::Replay { capture } => {
            let (ctl_tx, ctl_rx) = mpsc::channel();
            std::thread::Builder::new()
                .name("si-steam-replay".into())
                .spawn(move || replay_loop(capture, defs, tx, ctl_rx))
                .expect("spawn replay source");
            SourceHandle { state: rx, control: Some(ctl_tx) }
        }
    }
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

/// A save's bytes, gunzipped when the recorder stored it as `.gz`.
pub fn read_save(path: &Path) -> anyhow::Result<Vec<u8>> {
    let raw = std::fs::read(path)?;
    if path.extension().is_some_and(|e| e == "gz") {
        let mut out = Vec::new();
        GzDecoder::new(&raw[..]).read_to_end(&mut out)?;
        Ok(out)
    } else {
        Ok(raw)
    }
}

fn decode_file(path: &Path, defs: &EngineDefs) -> anyhow::Result<PublicSnapshot> {
    let bytes = read_save(path)?;
    decode_bytes(&bytes, defs)
}

fn decode_bytes(bytes: &[u8], defs: &EngineDefs) -> anyhow::Result<PublicSnapshot> {
    let graph = nrbf::decode(bytes)?;
    Ok(projection::project(&engine_map::map(&graph), defs))
}

// ------------------------------------------------------------------ live

/// Records a live game as a capture (`data/steam-capture/live-<game-id>/`):
/// `saves/<HHMMSS_mmm>__<name>.dat.gz` at each save's write time,
/// `events.txt` (`NEW <name>` at write time, which the decision parser uses
/// for undo), and `log-stream.txt` (each `log.txt` line stamped when read).
/// The same files a replay reads, so every live game can be replayed and
/// reviewed. Personal data: gitignored.
struct Recorder {
    root: PathBuf,
    dir: Option<PathBuf>,
    game: Option<String>,
    log_offset: u64,
    pending_log: Vec<String>,
    utc_offset_ms: i64,
}

impl Recorder {
    fn new(data_dir: &Path) -> Self {
        // Local wall clock for stamps, like the original capture tool.
        let utc_offset_ms = std::process::Command::new("date")
            .arg("+%z")
            .output()
            .ok()
            .and_then(|o| {
                let z = String::from_utf8_lossy(&o.stdout).trim().to_string();
                let sign = if z.starts_with('-') { -1 } else { 1 };
                let d = z.trim_start_matches(['+', '-']);
                let (h, m) = (d.get(0..2)?.parse::<i64>().ok()?, d.get(2..4)?.parse::<i64>().ok()?);
                Some(sign * (h * 60 + m) * 60_000)
            })
            .unwrap_or(0);
        Recorder { root: data_dir.join("steam-capture"), dir: None, game: None, log_offset: 0, pending_log: Vec::new(), utc_offset_ms }
    }

    fn local_ms(&self, t: SystemTime) -> u64 {
        let ms = t.duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0) + self.utc_offset_ms;
        ms.rem_euclid(86_400_000) as u64
    }

    fn stamp(&self, t: SystemTime) -> String {
        let ms = self.local_ms(t);
        format!("{}.{:03}", clock(ms), ms % 1000)
    }

    fn append(&self, file: &str, line: &str) {
        let Some(dir) = &self.dir else { return };
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join(file)) {
            let _ = writeln!(f, "{line}");
        }
    }

    fn set_game(&mut self, game_id: &str) {
        if self.game.as_deref() == Some(game_id) {
            return;
        }
        let dir = self.root.join(format!("live-{game_id}"));
        if std::fs::create_dir_all(dir.join("saves")).is_err() {
            return;
        }
        self.dir = Some(dir);
        self.game = Some(game_id.to_string());
        for line in std::mem::take(&mut self.pending_log) {
            self.append("log-stream.txt", &line);
        }
    }

    fn save(&self, name: &str, mtime: SystemTime, bytes: &[u8]) {
        let Some(dir) = &self.dir else { return };
        let ms = self.local_ms(mtime);
        let stamp = format!("{:02}{:02}{:02}_{:03}", ms / 3_600_000, (ms / 60_000) % 60, (ms / 1000) % 60, ms % 1000);
        let path = dir.join("saves").join(format!("{stamp}__{name}.gz"));
        if let Ok(f) = std::fs::File::create(&path) {
            let mut gz = GzEncoder::new(f, Compression::fast());
            let _ = gz.write_all(bytes).and_then(|_| gz.finish().map(|_| ()));
        }
        self.append("events.txt", &format!("{}  NEW            {name}", self.stamp(mtime)));
    }

    /// Read lines appended to `log.txt`. A shorter file means a new game:
    /// lines wait until its first save names the game.
    fn tail(&mut self, log: &Path) {
        let Ok(meta) = std::fs::metadata(log) else { return };
        if meta.len() < self.log_offset {
            self.log_offset = 0;
            self.game = None;
            self.dir = None;
        }
        if meta.len() == self.log_offset {
            return;
        }
        let Ok(mut f) = std::fs::File::open(log) else { return };
        use std::io::{Seek, SeekFrom};
        if f.seek(SeekFrom::Start(self.log_offset)).is_err() {
            return;
        }
        let mut buf = Vec::new();
        if f.read_to_end(&mut buf).is_err() {
            return;
        }
        // Keep a partial last line for the next read.
        let end = buf.iter().rposition(|b| *b == b'\n').map(|i| i + 1).unwrap_or(0);
        self.log_offset += end as u64;
        let stamp = self.stamp(SystemTime::now());
        for line in String::from_utf8_lossy(&buf[..end]).lines() {
            let line = format!("{stamp}  {line}");
            if self.dir.is_some() {
                self.append("log-stream.txt", &line);
            } else {
                self.pending_log.push(line);
            }
        }
    }
}

fn is_save(name: &str) -> bool {
    name.ends_with(".dat") && (name.starts_with("save-") || name.starts_with("EndGame--"))
}

fn game_running() -> Option<bool> {
    let out = std::process::Command::new("tasklist.exe")
        .args(["/FI", "IMAGENAME eq SpiritIsland.exe", "/NH"])
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&out.stdout).contains("SpiritIsland.exe"))
}

fn live_loop(dir: PathBuf, poll: Duration, data_dir: PathBuf, defs: Arc<EngineDefs>, tx: watch::Sender<Arc<CompanionState>>) {
    info!("steam live source watching {}", dir.display());
    let mut companion = Companion::default();
    let mut seen: HashMap<String, (SystemTime, u64)> = HashMap::new();
    let mut pending: HashMap<String, (SystemTime, u64)> = HashMap::new();
    let mut status = SourceStatus {
        mode: "live".into(),
        process: "unknown".into(),
        engine_build: String::new(),
        ..Default::default()
    };
    let mut last_proc_check = Instant::now() - Duration::from_secs(60);
    let mut last_update = Instant::now();
    let mut restored_game: Option<String> = None;
    let history_dir = data_dir.join("steam-history");
    // The truth is the most recently written save. Older files (rolling
    // saves, months of EndGame files) are never replayed as current.
    let mut truth_mtime = UNIX_EPOCH;
    let mut recorder = Recorder::new(&data_dir);
    let record = std::env::var("SI_STEAM_RECORD").map(|v| v != "0").unwrap_or(true);
    let started = SystemTime::now();

    loop {
        let mut changed = false;
        if last_proc_check.elapsed() >= Duration::from_secs(5) {
            last_proc_check = Instant::now();
            let p = match game_running() {
                Some(true) => "running",
                Some(false) => "not-running",
                None => "unknown",
            };
            if status.process != p {
                status.process = p.into();
                changed = true;
            }
        }

        if record {
            recorder.tail(&dir.join("log.txt"));
        }
        // Every save that is new or changed and complete (it decodes: a
        // complete stream ends in MessageEnd; a partial write fails and is
        // retried next poll). All are recorded (a main save and an undo
        // checkpoint often land within one poll); the newest is the truth.
        let mut stable: Vec<(SystemTime, String, u64, Vec<u8>)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if !is_save(&name) {
                    continue;
                }
                let Ok(meta) = e.metadata() else { continue };
                let (mtime, size) = (meta.modified().unwrap_or(UNIX_EPOCH), meta.len());
                if seen.get(&name) == Some(&(mtime, size)) || (mtime < truth_mtime && mtime < started) {
                    continue;
                }
                match std::fs::read(e.path()) {
                    Ok(bytes) if nrbf::decode(&bytes).is_ok() => stable.push((mtime, name, size, bytes)),
                    _ => {
                        pending.insert(name, (mtime, size));
                    }
                }
            }
        }
        stable.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
        if let Some((mtime, name, size, bytes)) = stable.last().filter(|(m, ..)| *m >= truth_mtime) {
            let (mtime, name, size) = (*mtime, name.clone(), *size);
            match decode_bytes(bytes, &defs) {
                Ok(snap) => {
                    truth_mtime = mtime;
                    if record && mtime >= started {
                        recorder.set_game(&snap.game_id);
                    }
                    if restored_game.as_deref() != Some(snap.game_id.as_str()) {
                        restored_game = Some(snap.game_id.clone());
                        for old in load_history(&history_dir, &snap.game_id) {
                            companion.push(old);
                        }
                    }
                    status.engine_build = snap.engine_build.clone();
                    status.last_save = Some(name.clone());
                    if companion.push(snap.clone()) {
                        append_history(&history_dir, &snap);
                        changed = true;
                    }
                    last_update = Instant::now();
                    status.stale = false;
                }
                Err(err) => {
                    // Usually a save caught mid-write; retried when it changes again.
                    status.decode_errors += 1;
                    warn!("decode {name} ({size} bytes): {err}");
                }
            }
        }
        for (mtime, name, size, bytes) in &stable {
            seen.insert(name.clone(), (*mtime, *size));
            pending.remove(name);
            // Only saves written while the companion runs are recorded, not old files.
            if record && *mtime >= started {
                recorder.save(name, *mtime, bytes);
            }
        }

        if !status.stale && last_update.elapsed() > Duration::from_secs(2) && status.process != "running" && !companion.timeline().is_empty() {
            status.stale = true;
            changed = true;
        }
        if record {
            if let Some(dir) = &recorder.dir {
                let raw = log_parse::capture_decisions(dir);
                let before = companion.timeline().len();
                companion.set_decisions(projection::decisions(&raw, &defs));
                changed |= before > 0 && changed;
                // Joined after Turn 1: the turns before the first save, from
                // the log. The first snapshot is the newest save at the
                // recorder's start, so the log read then is the log up to it.
                if companion.needs_backfill().is_some() {
                    if let Some(anchor_ms) = log_start_ms(dir) {
                        companion.backfill_from(dir, anchor_ms);
                        changed = true;
                    }
                }
            }
        }
        if changed {
            status.updated_at_ms = now_ms();
            let state = Arc::new(companion.state(status.clone()));
            write_latest(&data_dir, &state);
            tx.send_replace(state);
        }
        std::thread::sleep(poll);
    }
}

fn load_history(dir: &Path, game_id: &str) -> Vec<PublicSnapshot> {
    let Ok(text) = std::fs::read_to_string(dir.join(format!("{game_id}.jsonl"))) else { return Vec::new() };
    text.lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

fn append_history(dir: &Path, snap: &PublicSnapshot) {
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let path = dir.join(format!("{}.jsonl", snap.game_id));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path)
        && let Ok(line) = serde_json::to_string(snap) {
            let _ = writeln!(f, "{line}");
        }
}

/// `data/steam-game.json`: the latest state, for restarts and (later) MCP.
fn write_latest(data_dir: &Path, state: &CompanionState) {
    let tmp = data_dir.join("steam-game.json.tmp");
    if let Ok(json) = serde_json::to_vec(state)
        && std::fs::write(&tmp, json).is_ok() {
            let _ = std::fs::rename(&tmp, data_dir.join("steam-game.json"));
        }
}

// ---------------------------------------------------------------- replay

struct Frame {
    clock_ms: u64,
    name: String,
    snap: Option<PublicSnapshot>,
}

/// The wall clock of the first line of a capture's log stream: when the
/// recorder first read the game's log.
pub fn log_start_ms(capture: &Path) -> Option<u64> {
    let text = std::fs::read_to_string(capture.join("log-stream.txt")).ok()?;
    let first = text.lines().next()?;
    let (t, _) = first.split_once("  ")?;
    let (hms, ms) = t.split_once('.')?;
    let mut parts = hms.split(':').map(|p| p.parse::<u64>().ok());
    let (h, m, s) = (parts.next()??, parts.next()??, parts.next()??);
    Some(((h * 60 + m) * 60 + s) * 1000 + ms.parse::<u64>().ok()?)
}

/// `HHMMSS_mmm__save-….dat` → (ms since midnight, original save name).
fn parse_capture_name(file: &str) -> Option<(u64, String)> {
    let (stamp, name) = file.split_once("__")?;
    let (hms, ms) = stamp.split_once('_')?;
    if hms.len() != 6 {
        return None;
    }
    let n = |r: std::ops::Range<usize>| hms.get(r)?.parse::<u64>().ok();
    let t = ((n(0..2)? * 60 + n(2..4)?) * 60 + n(4..6)?) * 1000 + ms.parse::<u64>().ok()?;
    Some((t, name.to_string()))
}

fn clock(ms: u64) -> String {
    let s = ms / 1000;
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

fn replay_loop(capture: PathBuf, defs: Arc<EngineDefs>, tx: watch::Sender<Arc<CompanionState>>, ctl: mpsc::Receiver<ReplayCommand>) {
    let saves = if capture.join("saves").is_dir() { capture.join("saves") } else { capture.clone() };
    let mut files: Vec<(u64, String, PathBuf)> = std::fs::read_dir(&saves)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let file = e.file_name().to_string_lossy().to_string();
            let (t, name) = parse_capture_name(&file)?;
            let name = name.trim_end_matches(".gz").to_string();
            is_save(&name).then(|| (t, name, e.path()))
        })
        .collect();
    files.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    info!("steam replay: {} saves from {}", files.len(), saves.display());
    tx.send_replace(Arc::new(CompanionState {
        source: SourceStatus {
            mode: "replay".into(),
            process: "replay".into(),
            note: Some(format!("decoding {} saves", files.len())),
            updated_at_ms: now_ms(),
            ..Default::default()
        },
        ..Default::default()
    }));

    let mut frames: Vec<Frame> = Vec::new();
    let mut decode_errors = 0;
    for (t, name, path) in &files {
        let snap = match decode_file(path, &defs) {
            Ok(s) => Some(s),
            Err(e) => {
                decode_errors += 1;
                warn!("replay decode {name}: {e}");
                None
            }
        };
        frames.push(Frame { clock_ms: *t, name: name.clone(), snap });
    }
    let all_decisions = projection::decisions(&log_parse::capture_decisions(&capture), &defs);
    let total = frames.len();
    let mut index = 0usize;
    let mut playing = false;
    let mut speed = 8.0;
    let capture_name = capture.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

    let mut seq = 0u64;
    let mut publish = |index: usize, playing: bool, speed: f64| {
        seq += 1;
        let mut companion = Companion::default();
        for f in frames.iter().take(index + 1) {
            if let Some(s) = &f.snap {
                companion.push(s.clone());
            }
        }
        let frame = frames.get(index);
        // No look-ahead: only decisions made before this save was written.
        let upto = frame.map(|f| f.clock_ms).unwrap_or(0);
        companion.set_decisions(all_decisions.iter().filter(|d| d.ms <= upto).cloned().collect());
        // A capture that began after Turn 1: the turns before it, from the log.
        if companion.needs_backfill().is_some() {
            if let Some(anchor_ms) = frames.first().map(|f| f.clock_ms) {
                companion.backfill_from(&capture, anchor_ms);
            }
        }
        let status = SourceStatus {
            mode: "replay".into(),
            process: "replay".into(),
            last_save: frame.map(|f| f.name.clone()),
            updated_at_ms: now_ms(),
            stale: false,
            engine_build: frame.and_then(|f| f.snap.as_ref()).map(|s| s.engine_build.clone()).unwrap_or_default(),
            decode_errors,
            replay: Some(ReplayStatus {
                capture: capture_name.clone(),
                index,
                total,
                playing,
                speed,
                clock: frame.map(|f| clock(f.clock_ms)).unwrap_or_default(),
            }),
            note: None,
        };
        let mut state = companion.state(status);
        // Monotonic across seeks, so clients can drop stale states.
        state.seq = seq;
        tx.send_replace(Arc::new(state));
    };
    publish(index, playing, speed);

    loop {
        // While playing, wait for the real gap between saves ÷ speed, capped.
        let wait = if playing && index + 1 < total {
            let gap = frames[index + 1].clock_ms.saturating_sub(frames[index].clock_ms);
            Duration::from_millis(((gap as f64 / speed) as u64).clamp(60, 3000))
        } else {
            Duration::from_secs(3600)
        };
        match ctl.recv_timeout(wait) {
            Ok(cmd) => {
                match cmd {
                    ReplayCommand::Play { speed: s } => {
                        playing = true;
                        speed = s.clamp(0.25, 500.0);
                    }
                    ReplayCommand::Pause => playing = false,
                    ReplayCommand::Step(d) => {
                        playing = false;
                        index = (index as i64 + d).clamp(0, total.saturating_sub(1) as i64) as usize;
                    }
                    ReplayCommand::Seek(i) => index = i.min(total.saturating_sub(1)),
                }
                publish(index, playing, speed);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if playing && index + 1 < total {
                    index += 1;
                    if index + 1 == total {
                        playing = false;
                    }
                    publish(index, playing, speed);
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_names() {
        let (t, name) = parse_capture_name("132416_424__save-Feedback-003-0085-FastPower.dat").unwrap();
        assert_eq!(name, "save-Feedback-003-0085-FastPower.dat");
        assert_eq!(t, ((13 * 60 + 24) * 60 + 16) * 1000 + 424);
        assert_eq!(clock(t), "13:24:16");
        assert!(parse_capture_name("log.txt").is_none());
    }
}
