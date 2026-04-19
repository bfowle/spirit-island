//! si-backend — axum HTTP + WebSocket server for the interactive board tool.
//!
//! Serves the Vue frontend's built assets; exposes REST `/api/state` for
//! reads + writes; broadcasts state-changes over a WebSocket so multiple
//! browser tabs stay in sync. Persists state to `data/current-game.json`.

use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use si_common::{
    GameState,
    probability::{hypergeom_at_least, wilson_95},
    stats::{fear_by_round, FearOverRounds},
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Clone)]
struct AppState {
    state: Arc<RwLock<GameState>>,
    state_path: PathBuf,
    decks_dir: PathBuf,
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state_path: PathBuf = std::env::var("SI_STATE_PATH")
        .unwrap_or_else(|_| "data/current-game.json".to_string())
        .into();
    let decks_dir: PathBuf = std::env::var("SI_DECKS_DIR")
        .unwrap_or_else(|_| "data/decks".to_string())
        .into();
    let data_dir: PathBuf = std::env::var("SI_DATA_DIR")
        .unwrap_or_else(|_| "data".to_string())
        .into();
    let state = load_or_default(&state_path)?;

    let app_state = AppState {
        state: Arc::new(RwLock::new(state)),
        state_path,
        decks_dir,
        data_dir,
    };

    let frontend_dist = PathBuf::from("tools/si-live/frontend/dist");

    let app = Router::new()
        .route("/api/state", get(get_state).put(put_state))
        .route("/api/health", get(health))
        .route("/api/stats", get(get_stats))
        .route("/api/draw-probability", get(get_draw_probability))
        .route("/api/registry", get(get_registry))
        .route("/api/new-game", axum::routing::post(post_new_game))
        .fallback_service(ServeDir::new(&frontend_dist).append_index_html_on_directories(true))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(app_state);

    let addr: SocketAddr = std::env::var("SI_BIND")
        .unwrap_or_else(|_| "127.0.0.1:7777".to_string())
        .parse()?;
    info!("si-live backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn load_or_default(path: &PathBuf) -> anyhow::Result<GameState> {
    if path.exists() {
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw)?)
    } else {
        Ok(GameState::default())
    }
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn get_state(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let snapshot = state.state.read().await.clone();
    Json(snapshot)
}

async fn put_state(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(new_state): Json<GameState>,
) -> impl IntoResponse {
    {
        let mut guard = state.state.write().await;
        *guard = new_state.clone();
    }
    if let Err(e) = persist(&state.state_path, &new_state).await {
        tracing::error!(?e, "failed to persist state");
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("persist error: {e}")).into_response();
    }
    (StatusCode::OK, Json(new_state)).into_response()
}

async fn persist(path: &PathBuf, state: &GameState) -> anyhow::Result<()> {
    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }
    let json = serde_json::to_string_pretty(state)?;
    tokio::fs::write(path, json).await?;
    Ok(())
}

#[derive(serde::Serialize)]
struct StatsResponse {
    round: u8,
    fear_current: u8,
    fear_threshold: u8,
    terror_level: u8,
    blight_current: u8,
    blight_cap: u8,
    /// Per-spirit element pool this turn.
    elements_per_spirit: HashMap<String, HashMap<String, u8>>,
    /// Per-spirit hand / discard / played counts.
    pile_counts: HashMap<String, PileCounts>,
    /// Fear generated per round, as logged.
    fear_by_round: Vec<FearOverRounds>,
}

#[derive(serde::Serialize)]
struct PileCounts {
    hand: u32,
    discard: u32,
    played_this_turn: u32,
    forgotten: u32,
    presence_placed: u32,
}

async fn get_stats(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let guard = state.state.read().await;
    let mut elements_per_spirit = HashMap::new();
    let mut pile_counts = HashMap::new();
    for (slug, spirit) in &guard.spirits {
        let elems: HashMap<String, u8> = spirit
            .elements_this_turn
            .iter()
            .map(|(k, v)| {
                let key = serde_json::to_value(k)
                    .ok()
                    .and_then(|v| v.as_str().map(String::from))
                    .unwrap_or_default();
                (key, *v)
            })
            .collect();
        elements_per_spirit.insert(slug.clone(), elems);
        pile_counts.insert(
            slug.clone(),
            PileCounts {
                hand: spirit.hand.len() as u32,
                discard: spirit.discard.len() as u32,
                played_this_turn: spirit.played_this_turn.len() as u32,
                forgotten: spirit.forgotten.len() as u32,
                presence_placed: spirit.presence_on_board.values().map(|n| *n as u32).sum(),
            },
        );
    }
    let body = StatsResponse {
        round: guard.round,
        fear_current: guard.pools.fear_current,
        fear_threshold: guard.pools.fear_threshold,
        terror_level: guard.pools.terror_level,
        blight_current: guard.pools.blight_current,
        blight_cap: guard.pools.blight_cap,
        elements_per_spirit,
        pile_counts,
        fear_by_round: fear_by_round(&guard.log),
    };
    Json(body)
}

#[derive(Deserialize)]
struct DrawProbQuery {
    /// Deck to sample from: "minor" | "major" | "unique".
    deck: String,
    /// Element to check for (lowercase — moon, fire, air, ...).
    element: String,
    /// Number of cards already drawn / removed from the deck (defaults 0).
    #[serde(default)]
    drawn: u32,
    /// Number of draws to evaluate. Defaults 1.
    #[serde(default = "one")]
    draws: u32,
    /// At-least-k successes. Defaults 1.
    #[serde(default = "one")]
    at_least: u32,
}

fn one() -> u32 {
    1
}

#[derive(serde::Serialize)]
struct DrawProbResponse {
    deck: String,
    element: String,
    population: u32,
    successes: u32,
    draws: u32,
    at_least: u32,
    probability: f64,
    wilson_95: (f64, f64),
}

async fn get_registry(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    // Aggregate spirits, adversaries, scenarios, boards registries into one response.
    let read_json = |path: PathBuf| -> Option<serde_json::Value> {
        std::fs::read_to_string(&path).ok().and_then(|s| serde_json::from_str(&s).ok())
    };
    let spirits = read_json(state.data_dir.join("spirits.json"));
    let adversaries = read_json(state.data_dir.join("adversaries.json"));
    let scenarios = read_json(state.data_dir.join("scenarios.json"));

    // List all board files
    let boards_dir = state.data_dir.join("boards");
    let mut boards = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&boards_dir) {
        let mut names: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                let p = e.path();
                if p.extension().and_then(|s| s.to_str()) == Some("json") {
                    p.file_stem().and_then(|s| s.to_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect();
        names.sort();
        for name in names {
            if let Some(json) = read_json(boards_dir.join(format!("{name}.json"))) {
                boards.push(serde_json::json!({ "file": name, "data": json }));
            }
        }
    }

    Json(serde_json::json!({
        "spirits": spirits,
        "adversaries": adversaries,
        "scenarios": scenarios,
        "boards": boards,
    }))
}

#[derive(Deserialize)]
struct NewGameRequest {
    adversary: Option<String>,
    level: Option<u8>,
    scenario: Option<String>,
    spirits: Vec<String>,
    boards: Vec<String>,
    expansions_active: Vec<String>,
}

async fn post_new_game(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<NewGameRequest>,
) -> impl IntoResponse {
    // Load each board file and seed board_state + initial invader placement.
    let boards_dir = state.data_dir.join("boards");
    let mut board_state: std::collections::HashMap<String, serde_json::Value> =
        std::collections::HashMap::new();
    for board_letter in &req.boards {
        // Try matching base_<L> or je_<L> file by searching for _{letter}.json
        let file_candidates = [
            format!("base_{board_letter}.json"),
            format!("je_{board_letter}.json"),
        ];
        let mut found = None;
        for cand in &file_candidates {
            let p = boards_dir.join(cand);
            if p.exists() {
                found = std::fs::read_to_string(&p).ok();
                break;
            }
        }
        let Some(raw) = found else {
            return (
                StatusCode::BAD_REQUEST,
                format!("board {board_letter} not found in {boards_dir:?}"),
            )
                .into_response();
        };
        let board_json: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("parse error on board {board_letter}: {e}"),
                )
                    .into_response();
            }
        };
        // Build lands: take land.terrain/coastal + base_invader_setup for units
        let setup = board_json.get("base_invader_setup").cloned().unwrap_or(serde_json::json!({}));
        let mut lands_out = serde_json::Map::new();
        if let Some(lands) = board_json.get("lands").and_then(|v| v.as_object()) {
            for (land_id, meta) in lands {
                if land_id == "0" {
                    continue; // skip ocean
                }
                let terrain = meta.get("terrain").and_then(|v| v.as_str()).unwrap_or("?");
                let coastal = meta.get("coastal").and_then(|v| v.as_bool()).unwrap_or(false);
                let starting_dahan = meta.get("starting_dahan").and_then(|v| v.as_u64()).unwrap_or(0);
                let mut explorers = 0u64;
                let mut towns = 0u64;
                let mut cities = 0u64;
                if setup.get("1_explorer").and_then(|v| v.as_str()) == Some(land_id.as_str()) {
                    explorers = 1;
                }
                if setup.get("2_town").and_then(|v| v.as_str()) == Some(land_id.as_str()) {
                    towns = 1;
                }
                if setup.get("3_city").and_then(|v| v.as_str()) == Some(land_id.as_str()) {
                    cities = 1;
                }
                lands_out.insert(
                    land_id.clone(),
                    serde_json::json!({
                        "terrain": terrain,
                        "coastal": coastal,
                        "explorers": explorers,
                        "towns": towns,
                        "cities": cities,
                        "dahan": starting_dahan,
                        "blight": 0,
                        "tokens": [],
                    }),
                );
            }
        }
        board_state.insert(
            board_letter.clone(),
            serde_json::json!({ "lands": serde_json::Value::Object(lands_out) }),
        );
    }

    // Seed spirits with their Wiki-parsed starting state
    let wiki_dir = state.data_dir.join("references/wiki");
    let mut spirits_out: std::collections::HashMap<String, serde_json::Value> =
        std::collections::HashMap::new();
    for slug in &req.spirits {
        let wiki_path = wiki_dir.join(format!("{slug}.json"));
        let Ok(raw) = std::fs::read_to_string(&wiki_path) else {
            return (StatusCode::BAD_REQUEST, format!("spirit wiki data not found: {slug}")).into_response();
        };
        let wiki: serde_json::Value = serde_json::from_str(&raw).unwrap_or_default();
        let energy_track = wiki
            .get("presence_energy_track")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        let cp_track = wiki
            .get("presence_cardplay_track")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        let uniques = wiki
            .get("unique_cards")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));

        // Extract starting income from first slot of each track
        let start_e = energy_track
            .get(0)
            .and_then(|v| v.as_str())
            .and_then(|s| s.strip_prefix("energy"))
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        let start_cp = cp_track
            .get(0)
            .and_then(|v| v.as_str())
            .and_then(|s| s.strip_prefix("card"))
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(1);

        spirits_out.insert(
            slug.clone(),
            serde_json::json!({
                "energy": start_e,
                "card_plays": start_cp,
                "presence_on_board": {},
                "presence_on_track_energy": energy_track,
                "presence_on_track_cardplay": cp_track,
                "elements_this_turn": {},
                "hand": uniques,
                "discard": [],
                "forgotten": [],
                "played_this_turn": [],
                "growth_options": [],
            }),
        );
    }

    let new_state = serde_json::json!({
        "version": "1.0",
        "round": 1,
        "phase": "setup",
        "setup": {
            "adversary": req.adversary,
            "level": req.level,
            "scenario": req.scenario,
            "spirits": req.spirits,
            "boards": req.boards,
            "expansions_active": req.expansions_active,
        },
        "pools": {
            "fear_current": 0,
            "fear_threshold": 4,
            "terror_level": 1,
            "blight_current": 0,
            "blight_cap": 3,
            "island_blighted": false,
        },
        "spirits": spirits_out,
        "board_state": board_state,
        "log": [],
    });

    // Parse into GameState; reject if the schema doesn't match
    let parsed: GameState = match serde_json::from_value(new_state.clone()) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(?e, "new-game schema validation failed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("schema error: {e}"),
            )
                .into_response();
        }
    };

    {
        let mut guard = state.state.write().await;
        *guard = parsed.clone();
    }
    if let Err(e) = persist(&state.state_path, &parsed).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("persist error: {e}")).into_response();
    }
    Json(parsed).into_response()
}

async fn get_draw_probability(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(q): Query<DrawProbQuery>,
) -> impl IntoResponse {
    let deck_path = state.decks_dir.join(format!("{}.json", q.deck));
    let raw = match std::fs::read_to_string(&deck_path) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                format!("deck '{}' not found ({}): {}", q.deck, deck_path.display(), e),
            )
                .into_response();
        }
    };
    let deck: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("parse error on deck '{}': {}", q.deck, e),
            )
                .into_response();
        }
    };
    let cards = deck
        .get("cards")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let total = cards.len() as u32;
    let element_l = q.element.to_lowercase();
    let successes = cards
        .iter()
        .filter(|c| {
            c.get("elements")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().any(|e| e.as_str() == Some(element_l.as_str())))
                .unwrap_or(false)
        })
        .count() as u32;

    let population = total.saturating_sub(q.drawn);
    let prob = hypergeom_at_least(population, successes, q.draws.min(population), q.at_least);
    // Wilson bound on the underlying element-prevalence (Laplace-style sample):
    let (lo, hi) = wilson_95(successes, total);

    Json(DrawProbResponse {
        deck: q.deck,
        element: element_l,
        population,
        successes,
        draws: q.draws,
        at_least: q.at_least,
        probability: prob,
        wilson_95: (lo, hi),
    })
    .into_response()
}
