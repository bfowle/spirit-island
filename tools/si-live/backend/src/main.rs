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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state_path = std::env::var("SI_STATE_PATH")
        .unwrap_or_else(|_| "data/current-game.json".to_string())
        .into();
    let decks_dir = std::env::var("SI_DECKS_DIR")
        .unwrap_or_else(|_| "data/decks".to_string())
        .into();
    let state = load_or_default(&state_path)?;

    let app_state = AppState {
        state: Arc::new(RwLock::new(state)),
        state_path,
        decks_dir,
    };

    let frontend_dist = PathBuf::from("tools/si-live/frontend/dist");

    let app = Router::new()
        .route("/api/state", get(get_state).put(put_state))
        .route("/api/health", get(health))
        .route("/api/stats", get(get_stats))
        .route("/api/draw-probability", get(get_draw_probability))
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
