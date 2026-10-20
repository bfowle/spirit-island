//! Steam companion routes. Read-only with respect to every game: they serve
//! `CompanionState` built by `si-steam` (public information only) and never
//! read or write the tabletop `data/current-game.json`.
//!
//! - `GET  /api/companion/state`   current `CompanionState`
//! - `GET  /api/companion/stream`  SSE: one `state` event per change
//! - `GET  /api/companion/health`  source status and decoder diagnostics
//! - `POST /api/companion/replay`  play / pause / step / seek (replay mode only;
//!   moves a cursor over local capture files)
//!
//! Configuration: `SI_STEAM_MODE` = `live` (default) | `replay` | `off`;
//! `SI_STEAM_DIR` (live save folder), `SI_STEAM_CAPTURE` (replay folder),
//! `SI_STEAM_DLL` (engine definitions), `SI_STEAM_POLL_MS` (default 250).

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use si_steam::companion::CompanionState;
use si_steam::engine_defs::{DEFAULT_DLL, EngineDefs};
use si_steam::source::{self, ReplayCommand, SourceConfig, SourceHandle};
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use tokio::sync::watch;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::WatchStream;
use tracing::{info, warn};

#[derive(Clone)]
pub struct CompanionApp {
    state: watch::Receiver<Arc<CompanionState>>,
    control: Option<Arc<Mutex<mpsc::Sender<ReplayCommand>>>>,
}

/// The Windows user's LocalLow game folder, via `cmd.exe` + `wslpath`.
fn default_steam_dir() -> Option<PathBuf> {
    let out = std::process::Command::new("cmd.exe").args(["/c", "echo %USERPROFILE%"]).output().ok()?;
    let win = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let out = std::process::Command::new("wslpath").args(["-u", &win]).output().ok()?;
    let home = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    Some(home.join("AppData/LocalLow/Handelabra Games Inc_/Spirit Island"))
}

pub fn start(data_dir: &Path) -> CompanionApp {
    let mode = std::env::var("SI_STEAM_MODE").unwrap_or_else(|_| "live".into());
    let dll = std::env::var("SI_STEAM_DLL").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(DEFAULT_DLL));
    let defs = match EngineDefs::load(&dll) {
        Ok(d) => Arc::new(d),
        Err(e) => {
            warn!("companion off: {e:#}");
            return from_handle(source::spawn(SourceConfig::Off, Arc::new(EngineDefs::default())));
        }
    };
    let config = match mode.as_str() {
        "off" => SourceConfig::Off,
        "replay" => {
            let capture = std::env::var("SI_STEAM_CAPTURE")
                .map(PathBuf::from)
                .unwrap_or_else(|_| data_dir.join("steam-capture/2026-10-18-test1"));
            info!("companion: replay {}", capture.display());
            SourceConfig::Replay { capture }
        }
        _ => {
            let Some(dir) = std::env::var("SI_STEAM_DIR").map(PathBuf::from).ok().or_else(default_steam_dir) else {
                warn!("companion off: cannot resolve the Steam save folder; set SI_STEAM_DIR");
                return from_handle(source::spawn(SourceConfig::Off, defs));
            };
            let poll = std::env::var("SI_STEAM_POLL_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(250);
            SourceConfig::Live { dir, poll: Duration::from_millis(poll), data_dir: data_dir.to_path_buf() }
        }
    };
    from_handle(source::spawn(config, defs))
}

fn from_handle(h: SourceHandle) -> CompanionApp {
    CompanionApp { state: h.state, control: h.control.map(|c| Arc::new(Mutex::new(c))) }
}

pub fn routes(app: CompanionApp) -> Router {
    Router::new()
        .route("/api/companion/state", get(get_state))
        .route("/api/companion/stream", get(stream))
        .route("/api/companion/health", get(health))
        .route("/api/companion/replay", post(replay))
        .with_state(app)
}

async fn get_state(State(app): State<CompanionApp>) -> Json<serde_json::Value> {
    let s = app.state.borrow().clone();
    Json(serde_json::to_value(&*s).unwrap_or_default())
}

async fn stream(State(app): State<CompanionApp>) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let events = WatchStream::new(app.state.clone())
        .map(|s| Ok(Event::default().event("state").data(serde_json::to_string(&*s).unwrap_or_default())));
    Sse::new(events).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
}

async fn health(State(app): State<CompanionApp>) -> Json<serde_json::Value> {
    let s = app.state.borrow().clone();
    Json(serde_json::json!({
        "source": s.source,
        "seq": s.seq,
        "has_state": s.public.is_some(),
        "warnings": s.public.as_ref().map(|p| p.warnings.clone()).unwrap_or_default(),
    }))
}

#[derive(Deserialize)]
struct ReplayRequest {
    action: String,
    speed: Option<f64>,
    index: Option<usize>,
    by: Option<i64>,
}

async fn replay(State(app): State<CompanionApp>, Json(req): Json<ReplayRequest>) -> impl IntoResponse {
    let Some(ctl) = &app.control else {
        return (StatusCode::CONFLICT, "not in replay mode").into_response();
    };
    let cmd = match req.action.as_str() {
        "play" => ReplayCommand::Play { speed: req.speed.unwrap_or(8.0) },
        "pause" => ReplayCommand::Pause,
        "step" => ReplayCommand::Step(req.by.unwrap_or(1)),
        "seek" => ReplayCommand::Seek(req.index.unwrap_or(0)),
        _ => return (StatusCode::BAD_REQUEST, "action: play | pause | step | seek").into_response(),
    };
    match ctl.lock().map(|c| c.send(cmd)) {
        Ok(Ok(())) => StatusCode::NO_CONTENT.into_response(),
        _ => (StatusCode::SERVICE_UNAVAILABLE, "replay source stopped").into_response(),
    }
}
