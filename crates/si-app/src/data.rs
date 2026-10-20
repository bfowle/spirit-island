//! The data side: the Steam source (live or a replayed capture) running
//! in-process on its own thread, and the Windows paths it needs.

use si_steam::companion::CompanionState;
use si_steam::engine_defs::EngineDefs;
use si_steam::source::{self, ReplayCommand, SourceConfig, SourceHandle};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

/// The game's engine DLL (card, spirit and adversary definitions).
pub fn engine_dll() -> PathBuf {
    if let Ok(p) = std::env::var("SI_STEAM_DLL") {
        return PathBuf::from(p);
    }
    if cfg!(windows) {
        PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\Spirit Island\SpiritIsland_Data\Managed\SpiritIslandEngine.dll")
    } else {
        PathBuf::from(si_steam::engine_defs::DEFAULT_DLL)
    }
}

/// The game's save folder (LocalLow) on Windows.
fn steam_dir() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SI_STEAM_DIR") {
        return Some(PathBuf::from(p));
    }
    let home = std::env::var("USERPROFILE").ok()?;
    Some(Path::new(&home).join(r"AppData\LocalLow\Handelabra Games Inc_\Spirit Island"))
}

/// Where the app records live games: `SI_DATA_DIR`, else
/// `%LOCALAPPDATA%\si-companion` (never the repo's tabletop data).
fn data_dir() -> PathBuf {
    if let Ok(p) = std::env::var("SI_DATA_DIR") {
        return PathBuf::from(p);
    }
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".into());
    Path::new(&base).join("si-companion")
}

/// The game's art for the versus screen: `SI_ART_DIR`, else `<data dir>/art`.
pub fn art_dir() -> PathBuf {
    if let Ok(p) = std::env::var("SI_ART_DIR") {
        return PathBuf::from(p);
    }
    data_dir().join("art")
}

pub struct Source {
    handle: SourceHandle,
    /// "Replay · live-0d0b…" or "Live".
    pub label: String,
}

impl Source {
    pub fn start(capture: Option<&Path>) -> anyhow::Result<Source> {
        let defs = Arc::new(EngineDefs::load(&engine_dll())?);
        let (config, label) = match capture {
            Some(c) => {
                let name = c.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                (SourceConfig::Replay { capture: c.to_path_buf() }, format!("Replay · {name}"))
            }
            None => match steam_dir() {
                Some(dir) => (SourceConfig::Live { dir, poll: Duration::from_millis(250), data_dir: data_dir() }, "Live".to_string()),
                None => (SourceConfig::Off, "Off".to_string()),
            },
        };
        Ok(Source { handle: source::spawn(config, defs), label })
    }

    pub fn changed(&self) -> bool {
        self.handle.state.has_changed().unwrap_or(false)
    }

    /// The latest state, marking it seen.
    pub fn latest(&mut self) -> Arc<CompanionState> {
        self.handle.state.borrow_and_update().clone()
    }

    pub fn peek(&self) -> Arc<CompanionState> {
        self.handle.state.borrow().clone()
    }

    pub fn send(&self, cmd: ReplayCommand) {
        if let Some(c) = &self.handle.control {
            let _ = c.send(cmd);
        }
    }
}

/// Resolve `--at` for a capture: a save index, `end`, or `T<turn>/<Step>`
/// (the first save of that step, from the save file names, which carry the
/// zero-based turn and the step: `save-Feedback-004-0118-FastPower.dat`).
pub fn resolve_at(capture: &Path, at: &str) -> Option<usize> {
    let saves = if capture.join("saves").is_dir() { capture.join("saves") } else { capture.to_path_buf() };
    let mut names: Vec<String> = std::fs::read_dir(&saves)
        .ok()?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|f| {
            let n = f.split_once("__").map(|(_, n)| n).unwrap_or(f).trim_end_matches(".gz");
            n.ends_with(".dat") && (n.starts_with("save-") || n.starts_with("EndGame--"))
        })
        .collect();
    names.sort();
    if at == "end" {
        return names.len().checked_sub(1);
    }
    if let Ok(i) = at.parse::<usize>() {
        return Some(i.min(names.len().saturating_sub(1)));
    }
    let (turn, step) = at.trim_start_matches(['T', 't']).split_once('/')?;
    let turn: u32 = turn.parse().ok()?;
    let needle = format!("-{:03}-", turn.checked_sub(1)?);
    names.iter().position(|n| n.contains(&needle) && n.trim_end_matches(".gz").ends_with(&format!("-{step}.dat")))
}
