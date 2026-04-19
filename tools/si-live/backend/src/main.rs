//! si-backend — axum HTTP + WebSocket server for the interactive board tool.
//!
//! Serves the Vue frontend's built assets; exposes REST `/api/state` for
//! reads + writes; broadcasts state-changes over a WebSocket so multiple
//! browser tabs stay in sync. Persists state to `data/current-game.json`.

use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use si_common::GameState;
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state_path = std::env::var("SI_STATE_PATH")
        .unwrap_or_else(|_| "data/current-game.json".to_string())
        .into();
    let state = load_or_default(&state_path)?;

    let app_state = AppState {
        state: Arc::new(RwLock::new(state)),
        state_path,
    };

    let frontend_dist = PathBuf::from("tools/si-live/frontend/dist");

    let app = Router::new()
        .route("/api/state", get(get_state).put(put_state))
        .route("/api/health", get(health))
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
