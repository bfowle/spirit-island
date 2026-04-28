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
    let mut state = load_or_default(&state_path)?;
    migrate_invader_deck(&mut state, &data_dir);

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
        .route("/api/save-board-geometry", axum::routing::post(post_save_board_geometry))
        .route("/api/saved-games", get(get_saved_games))
        .route("/api/saved-games/archive", axum::routing::post(post_archive_current_game))
        .route("/api/saved-games/:id/load", axum::routing::post(post_load_saved_game))
        .route("/api/saved-games/:id", axum::routing::delete(delete_saved_game))
        .route("/api/spirit/:slug", get(get_spirit_wiki))
        .route("/api/deck/:kind", get(get_deck))
        .route("/api/invader-stack", get(get_invader_stack))
        .route("/api/spirit-affinity", get(get_spirit_affinity))
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

/// Look up the canonical invader stack for a given adversary+level from
/// `data/adversary-stacks.json`. Falls back to base 3·4·5 if not found.
fn canonical_stack(
    data_dir: &PathBuf,
    adversary: Option<&str>,
    level: Option<u8>,
) -> (Vec<u8>, String) {
    let path = data_dir.join("adversary-stacks.json");
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let root: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::json!({}));
    let base_default = root.get("base").cloned().unwrap_or_else(|| {
        serde_json::json!({
            "stack_sequence": [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3],
            "notation": "3 · 4 · 5"
        })
    });
    let picked = adversary
        .and_then(|adv| {
            let lvl = level.unwrap_or(0).to_string();
            root.get("adversaries")
                .and_then(|v| v.get(adv))
                .and_then(|v| v.get("levels"))
                .and_then(|v| v.get(&lvl))
                .cloned()
        })
        .unwrap_or(base_default);
    let seq: Vec<u8> = picked
        .get("stack_sequence")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
        .unwrap_or_default();
    let notation = picked
        .get("notation")
        .and_then(|v| v.as_str())
        .unwrap_or("3 · 4 · 5")
        .to_string();
    (seq, notation)
}

/// Ensure the saved game's invader_deck matches the canonical sequence for its
/// adversary/level. If the notation differs (old saves pre-Prussia-6 fix), rebuild
/// the upcoming stack from canonical while preserving exposed + discarded cards.
///
/// This auto-migration is safe because:
///   - Stage is locked per position (revealed cards don't change identity)
///   - Discarded cards preserve play history
///   - Only the unrevealed tail of the stack is replaced, so no flipped progress is lost
fn migrate_invader_deck(state: &mut GameState, data_dir: &PathBuf) {
    let adversary = state.setup.adversary.as_deref();
    let level = state.setup.level;
    let (canonical_seq, canonical_notation) = canonical_stack(data_dir, adversary, level);
    if canonical_seq.is_empty() {
        return;
    }

    let Some(deck) = state.invader_deck.as_mut() else {
        // Saved game predates the invader_deck schema entirely; seed fresh.
        state.invader_deck = Some(si_common::InvaderDeckState {
            ravage: None,
            build: None,
            explore: None,
            upcoming: canonical_seq
                .iter()
                .map(|&s| si_common::InvaderCard {
                    stage: s,
                    terrain: None,
                    notes: None,
                })
                .collect(),
            discarded: Vec::new(),
            notation: Some(canonical_notation),
        });
        return;
    };

    // Already aligned? skip.
    if deck.notation.as_deref() == Some(canonical_notation.as_str()) {
        return;
    }

    // Migrate: keep discarded + exposed; rebuild upcoming from canonical[revealed_count..]
    let exposed_count = usize::from(deck.ravage.is_some())
        + usize::from(deck.build.is_some())
        + usize::from(deck.explore.is_some());
    let revealed_count = deck.discarded.len() + exposed_count;
    let tail = canonical_seq.iter().skip(revealed_count).copied();
    deck.upcoming = tail
        .map(|stage| si_common::InvaderCard {
            stage,
            terrain: None,
            notes: None,
        })
        .collect();
    deck.notation = Some(canonical_notation);
    tracing::info!(
        adversary = ?adversary,
        level = ?level,
        revealed_count,
        new_upcoming = deck.upcoming.len(),
        "auto-migrated invader_deck to canonical stack"
    );
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn get_state(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    // Take a write lock so we can migrate stale invader decks in place before
    // returning. Old saves (pre-Prussia-6 fix) get auto-corrected as soon as
    // the frontend asks for state.
    let snapshot = {
        let mut guard = state.state.write().await;
        let pre = guard.invader_deck.as_ref().and_then(|d| d.notation.clone());
        migrate_invader_deck(&mut guard, &state.data_dir);
        let post = guard.invader_deck.as_ref().and_then(|d| d.notation.clone());
        let cloned = guard.clone();
        drop(guard);
        if pre != post {
            if let Err(e) = persist(&state.state_path, &cloned).await {
                tracing::error!(?e, "failed to persist migrated state");
            }
        }
        cloned
    };
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

    // List all board files. Expose only the summary needed by the UI:
    // board_id, expansion, available variants.
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
                let board_id = json
                    .get("board_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("?")
                    .to_string();
                let expansion = json
                    .get("expansion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("?")
                    .to_string();
                let variants: Vec<serde_json::Value> = json
                    .get("variants")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| {
                                serde_json::json!({
                                    "key": k,
                                    "name": v.get("name").cloned().unwrap_or_default(),
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                boards.push(serde_json::json!({
                    "file": name,
                    "board_id": board_id,
                    "expansion": expansion,
                    "variants": variants,
                }));
            }
        }
    }

    // Aspects: read every `references/wiki/aspects/*.json`, group by spirit slug.
    // Each aspect JSON carries a `spirit` field with the human-readable name; we
    // resolve that to a slug via spirits.json so the frontend can key aspects by
    // the same slug it uses for spirit selection.
    let aspects_dir = state.data_dir.join("references/wiki/aspects");
    let mut aspects_by_spirit: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    if let Ok(entries) = std::fs::read_dir(&aspects_dir) {
        // Build a name→slug index from the spirits registry so we can key aspects
        // by slug (matching the rest of the API surface).
        let name_to_slug: HashMap<String, String> = spirits
            .as_ref()
            .and_then(|s| s.get("spirits"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| {
                        let name = s.get("name").and_then(|v| v.as_str())?.to_string();
                        let slug = s.get("slug").and_then(|v| v.as_str())?.to_string();
                        Some((name, slug))
                    })
                    .collect()
            })
            .unwrap_or_default();

        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let Some(aspect_key) = p.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Ok(raw) = std::fs::read_to_string(&p) else { continue };
            let Ok(json): Result<serde_json::Value, _> = serde_json::from_str(&raw) else { continue };
            let spirit_name = json.get("spirit").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let Some(spirit_slug) = name_to_slug.get(&spirit_name) else {
                // Orphan aspect — the wiki's spirit name doesn't match our registry.
                // Skip rather than guess; authoring can fix the JSON if needed.
                continue;
            };
            let aspect_name = json
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(aspect_key)
                .to_string();
            let expansion = json.get("expansion").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let complexity_change = json
                .get("complexity_change")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            aspects_by_spirit
                .entry(spirit_slug.clone())
                .or_default()
                .push(serde_json::json!({
                    "key": aspect_key,
                    "name": aspect_name,
                    "expansion": expansion,
                    "complexity_change": complexity_change,
                }));
        }
        // Stable sort for deterministic UI ordering.
        for list in aspects_by_spirit.values_mut() {
            list.sort_by(|a, b| {
                let ak = a.get("key").and_then(|v| v.as_str()).unwrap_or("");
                let bk = b.get("key").and_then(|v| v.as_str()).unwrap_or("");
                ak.cmp(bk)
            });
        }
    }

    Json(serde_json::json!({
        "spirits": spirits,
        "adversaries": adversaries,
        "scenarios": scenarios,
        "boards": boards,
        "aspects": aspects_by_spirit,
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
    /// "balanced" or "thematic" — which side of the physical board to use.
    /// Applied globally to all selected boards. Defaults to "balanced".
    #[serde(default = "default_variant")]
    board_variant: String,
    /// Optional map from spirit slug → aspect slug. An empty or missing value
    /// for a spirit means the base (no-aspect) version.
    #[serde(default)]
    aspects: HashMap<String, String>,
}

fn default_variant() -> String {
    "balanced".to_string()
}

async fn post_new_game(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<NewGameRequest>,
) -> impl IntoResponse {
    // Load each board file and seed board_state + initial setup per the
    // selected variant (balanced or thematic).
    let boards_dir = state.data_dir.join("boards");
    let variant_key = req.board_variant.as_str();
    let mut board_state: std::collections::HashMap<String, serde_json::Value> =
        std::collections::HashMap::new();
    for board_letter in &req.boards {
        let file_candidates = [
            format!("base_{board_letter}.json"),
            format!("je_{board_letter}.json"),
            format!("hosi_{board_letter}.json"),
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

        // Pick the variant. Fall back to "balanced" if the requested one
        // doesn't exist (e.g., HoSI boards only have one side).
        let variant = board_json
            .get("variants")
            .and_then(|v| v.get(variant_key))
            .or_else(|| {
                board_json
                    .get("variants")
                    .and_then(|v| v.get("balanced"))
            });
        let Some(variant) = variant else {
            return (
                StatusCode::BAD_REQUEST,
                format!("board {board_letter}: no usable variant"),
            )
                .into_response();
        };

        let mut lands_out = serde_json::Map::new();
        if let Some(lands) = variant.get("lands").and_then(|v| v.as_object()) {
            for (land_id, meta) in lands {
                if land_id == "0" {
                    continue;
                }
                let terrain = meta.get("terrain").and_then(|v| v.as_str()).unwrap_or("?");
                let coastal = meta.get("coastal").and_then(|v| v.as_bool()).unwrap_or(false);
                let starting_dahan = meta.get("starting_dahan").and_then(|v| v.as_u64()).unwrap_or(0);
                let starting_town = meta.get("starting_town").and_then(|v| v.as_u64()).unwrap_or(0);
                let starting_city = meta.get("starting_city").and_then(|v| v.as_u64()).unwrap_or(0);
                let starting_blight = meta.get("starting_blight").and_then(|v| v.as_u64()).unwrap_or(0);
                let starting_explorer = meta.get("starting_explorer").and_then(|v| v.as_u64()).unwrap_or(0);
                let tokens = meta.get("tokens").cloned().unwrap_or_else(|| serde_json::json!([]));

                lands_out.insert(
                    land_id.clone(),
                    serde_json::json!({
                        "terrain": terrain,
                        "coastal": coastal,
                        "explorers": starting_explorer,
                        "towns": starting_town,
                        "cities": starting_city,
                        "dahan": starting_dahan,
                        "blight": starting_blight,
                        "tokens": tokens,
                    }),
                );
            }
        }
        board_state.insert(
            board_letter.clone(),
            serde_json::json!({
                "lands": serde_json::Value::Object(lands_out),
                "variant": variant_key,
                "variant_name": variant.get("name").cloned().unwrap_or_default(),
            }),
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

        // Extract starting income from the first slot of each track. That slot
        // is already uncovered at setup (its value is visible without removing
        // a disc — it's where the spirit's starting income comes from).
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

        // The "covered" array is slots [1..] — slot 0 starts revealed.
        let covered_energy: Vec<serde_json::Value> = energy_track
            .as_array()
            .map(|a| a.iter().skip(1).cloned().collect())
            .unwrap_or_default();
        let covered_cp: Vec<serde_json::Value> = cp_track
            .as_array()
            .map(|a| a.iter().skip(1).cloned().collect())
            .unwrap_or_default();

        let aspect_key = req.aspects.get(slug).cloned().filter(|s| !s.is_empty());

        // Aspect-driven hand + special-rule overrides. Some aspects (e.g.,
        // Dark Fire / Shadows) ship with a starting-card gain; others remove
        // or replace Uniques; most just swap the spirit's Special Rule.
        // We parse the aspect JSON's `other_row_fields.Setup` field for
        // "Gain {{Card|NAME}}" style additions and append to the hand.
        let mut hand = uniques.as_array().cloned().unwrap_or_default();
        let mut aspect_special_rules: serde_json::Value = serde_json::Value::Null;
        let mut aspect_setup_note: Option<String> = None;
        let mut aspect_name: Option<String> = None;
        let mut aspect_innate_override: serde_json::Value = serde_json::Value::Null;
        if let Some(key) = aspect_key.as_deref() {
            let aspect_path = wiki_dir.join(format!("aspects/{key}.json"));
            if let Ok(raw) = std::fs::read_to_string(&aspect_path) {
                if let Ok(aspect_json) = serde_json::from_str::<serde_json::Value>(&raw) {
                    aspect_name = aspect_json
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                    aspect_special_rules = aspect_json
                        .get("special_rules")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);
                    aspect_innate_override = aspect_json
                        .get("innate_override")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);
                    // "Gain {{Card|NAME}}" — add NAME to hand.
                    if let Some(setup_str) = aspect_json
                        .get("other_row_fields")
                        .and_then(|v| v.get("Setup"))
                        .and_then(|v| v.as_str())
                    {
                        aspect_setup_note = Some(setup_str.to_string());
                        for cap in setup_str.split("Gain ").skip(1) {
                            // cap might look like "{{Card|Unquenchable Flames}} (Minor Power) ..."
                            if let Some(start) = cap.find("{{Card|") {
                                let rest = &cap[start + 7..];
                                if let Some(end) = rest.find("}}") {
                                    let name = rest[..end].trim();
                                    if !name.is_empty() && !hand.iter().any(|c| c.as_str() == Some(name)) {
                                        hand.push(serde_json::Value::String(name.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        spirits_out.insert(
            slug.clone(),
            serde_json::json!({
                "energy": start_e,
                "card_plays": start_cp,
                "presence_on_board": {},
                "presence_on_track_energy": covered_energy,
                "presence_on_track_cardplay": covered_cp,
                "elements_this_turn": {},
                "hand": hand,
                "discard": [],
                "forgotten": [],
                "played_this_turn": [],
                "growth_options": [],
                "aspect": aspect_key,
                "aspect_name": aspect_name,
                "aspect_special_rules": aspect_special_rules,
                "aspect_setup_note": aspect_setup_note,
                "aspect_innate_override": aspect_innate_override,
            }),
        );
    }

    // Pick adversary stack sequence from data/adversary-stacks.json.
    // Fall back to the base 3·4·5 default when the adversary/level combo isn't known.
    let stacks_path = state.data_dir.join("adversary-stacks.json");
    let stacks_json: serde_json::Value = std::fs::read_to_string(&stacks_path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    let base_default = stacks_json
        .get("base")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({
            "stack_sequence": [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3],
            "notation": "3 · 4 · 5"
        }));
    let picked = req
        .adversary
        .as_ref()
        .and_then(|adv| {
            let level = req.level.unwrap_or(0).to_string();
            stacks_json
                .get("adversaries")
                .and_then(|v| v.get(adv))
                .and_then(|v| v.get("levels"))
                .and_then(|v| v.get(&level))
                .cloned()
        })
        .unwrap_or(base_default);
    let stack_seq = picked
        .get("stack_sequence")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let notation = picked
        .get("notation")
        .and_then(|v| v.as_str())
        .unwrap_or("3 · 4 · 5")
        .to_string();
    let upcoming_cards: Vec<serde_json::Value> = stack_seq
        .into_iter()
        .filter_map(|v| v.as_u64())
        .map(|s| serde_json::json!({ "stage": s, "terrain": null }))
        .collect();

    let player_count = req.spirits.len().max(1) as u8;
    let fear_threshold = match player_count {
        1 => 4,
        2 => 4,
        3 => 4,
        4 => 4,
        _ => 4,
    };
    // Fear deck size = 3 × player count by default (one section per terror tier
    // per player). Adversary data may override via `fear_split` on the level
    // entry — e.g., some adversaries thicken a specific terror tier.
    let adv_fear_split: Option<[u32; 3]> = picked
        .get("fear_split")
        .and_then(|v| v.as_array())
        .and_then(|a| {
            if a.len() == 3 {
                Some([
                    a[0].as_u64().unwrap_or(0) as u32,
                    a[1].as_u64().unwrap_or(0) as u32,
                    a[2].as_u64().unwrap_or(0) as u32,
                ])
            } else {
                None
            }
        });
    let default_tier = u32::from(player_count);
    let tier_counts: [u32; 3] = adv_fear_split.unwrap_or([default_tier, default_tier, default_tier]);
    let fear_deck_size: u32 = tier_counts.iter().sum();

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
            "aspects": req.aspects,
        },
        "pools": {
            "fear_current": 0,
            "fear_threshold": fear_threshold,
            "terror_level": 1,
            "blight_current": 0,
            "blight_cap": 3,
            "island_blighted": false,
        },
        "spirits": spirits_out,
        "board_state": board_state,
        "log": [],
        "invader_deck": {
            "ravage": null,
            "build": null,
            "explore": null,
            "upcoming": upcoming_cards,
            "discarded": [],
            "notation": notation,
        },
        "fear_deck": {
            "deck_size": fear_deck_size,
            "tier_counts": tier_counts,
            "earned": [],
            "resolved": [],
            "unseen": fear_deck_size,
        },
        // Event deck — starts empty. Players will mark the face-up previewed
        // card at start of Setup; per rules, the first previewed Event resolves
        // on Turn 2 (not Turn 1). `unseen` is event-pool-size approximate; tools
        // don't need it exact since we don't draw programmatically.
        "event_deck": {
            "previewed": [],
            "resolved": [],
            "unseen": 62,
        },
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

// --- Saved Games --- -------------------------------------------------------

fn saved_games_dir(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("games")
}

#[derive(serde::Serialize)]
struct SavedGameSummary {
    id: String,
    filename: String,
    created_at: u64,
    round: u8,
    phase: String,
    adversary: Option<String>,
    level: Option<u8>,
    scenario: Option<String>,
    spirits: Vec<String>,
    boards: Vec<String>,
}

fn build_summary(id: &str, filename: &str, path: &PathBuf) -> Option<SavedGameSummary> {
    let raw = std::fs::read_to_string(path).ok()?;
    let state: GameState = serde_json::from_str(&raw).ok()?;
    let metadata = std::fs::metadata(path).ok()?;
    let created = metadata
        .created()
        .or_else(|_| metadata.modified())
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(SavedGameSummary {
        id: id.to_string(),
        filename: filename.to_string(),
        created_at: created,
        round: state.round,
        phase: format!("{:?}", state.phase).to_lowercase(),
        adversary: state.setup.adversary,
        level: state.setup.level,
        scenario: state.setup.scenario,
        spirits: state.setup.spirits,
        boards: state.setup.boards,
    })
}

async fn get_saved_games(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let dir = saved_games_dir(&state.data_dir);
    let mut out: Vec<SavedGameSummary> = Vec::new();
    if dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) != Some("json") {
                    continue;
                }
                let Some(filename) = p.file_name().and_then(|s| s.to_str()).map(String::from)
                else {
                    continue;
                };
                let id = filename.trim_end_matches(".json").to_string();
                if let Some(sum) = build_summary(&id, &filename, &p) {
                    out.push(sum);
                }
            }
        }
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Json(serde_json::json!({ "games": out }))
}

async fn post_archive_current_game(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let snapshot = state.state.read().await.clone();
    let dir = saved_games_dir(&state.data_dir);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("mkdir {dir:?}: {e}")).into_response();
    }
    // Build filename: {timestamp}-r{round}-{adversary-level}-{spirits-joined}.json
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let adv = snapshot.setup.adversary.as_deref().unwrap_or("solo");
    let lvl = snapshot
        .setup
        .level
        .map(|l| format!("-l{l}"))
        .unwrap_or_default();
    let spirits = snapshot
        .setup
        .spirits
        .iter()
        .map(|s| s.split('-').next().unwrap_or(s).to_string())
        .collect::<Vec<_>>()
        .join(",");
    let spirits = if spirits.is_empty() { "solo".to_string() } else { spirits };
    let id = format!("{ts}-r{}-{adv}{lvl}-{spirits}", snapshot.round);
    let filename = format!("{id}.json");
    let path = dir.join(&filename);
    let json = match serde_json::to_string_pretty(&snapshot) {
        Ok(s) => s,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("serialize: {e}")).into_response()
        }
    };
    if let Err(e) = std::fs::write(&path, json) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("write {path:?}: {e}")).into_response();
    }
    Json(serde_json::json!({ "status": "archived", "id": id, "filename": filename })).into_response()
}

async fn post_load_saved_game(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let dir = saved_games_dir(&state.data_dir);
    let path = dir.join(format!("{id}.json"));
    if !path.exists() {
        return (StatusCode::NOT_FOUND, format!("no saved game {id}")).into_response();
    }
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("read: {e}")).into_response(),
    };
    let mut parsed: GameState = match serde_json::from_str(&raw) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("parse: {e}")).into_response(),
    };
    // Auto-migrate invader deck from canonical adversary-stacks.json. Old saves
    // (pre-Prussia-6 fix) get their upcoming stack replaced with the current
    // canonical sequence; exposed + discarded cards are preserved.
    migrate_invader_deck(&mut parsed, &state.data_dir);
    {
        let mut guard = state.state.write().await;
        *guard = parsed.clone();
    }
    if let Err(e) = persist(&state.state_path, &parsed).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("persist: {e}")).into_response();
    }
    Json(parsed).into_response()
}

async fn delete_saved_game(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let dir = saved_games_dir(&state.data_dir);
    let path = dir.join(format!("{id}.json"));
    if !path.exists() {
        return (StatusCode::NOT_FOUND, format!("no saved game {id}")).into_response();
    }
    if let Err(e) = std::fs::remove_file(&path) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("remove: {e}")).into_response();
    }
    Json(serde_json::json!({ "status": "deleted", "id": id })).into_response()
}

async fn get_spirit_wiki(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Guard against path traversal
    if slug.contains('/') || slug.contains("..") {
        return (StatusCode::BAD_REQUEST, "invalid slug".to_string()).into_response();
    }
    let path = state.data_dir.join("references/wiki").join(format!("{slug}.json"));
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::NOT_FOUND, format!("no wiki data for {slug}")).into_response();
    };
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("parse: {e}")).into_response(),
    }
}

// --- Deck lookup -----------------------------------------------------------

/// Serves baked deck JSON (fear/event/minor/major/unique/blight) from data/decks/.
async fn get_deck(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(kind): axum::extract::Path<String>,
) -> impl IntoResponse {
    let allowed = ["fear", "event", "minor", "major", "unique", "blight"];
    if !allowed.contains(&kind.as_str()) {
        return (StatusCode::BAD_REQUEST, format!("unknown deck kind: {kind}")).into_response();
    }
    let path = state.decks_dir.join(format!("{kind}.json"));
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::NOT_FOUND, format!("deck file not found: {kind}")).into_response();
    };
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("parse: {e}")).into_response(),
    }
}

// --- Invader stack lookup --------------------------------------------------

#[derive(Deserialize)]
struct InvaderStackQuery {
    adversary: Option<String>,
    level: Option<u8>,
}

async fn get_invader_stack(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(q): Query<InvaderStackQuery>,
) -> impl IntoResponse {
    let path = state.data_dir.join("adversary-stacks.json");
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::NOT_FOUND, "adversary-stacks.json not found".to_string()).into_response();
    };
    let root: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("parse: {e}")).into_response(),
    };
    let base_default = root.get("base").cloned().unwrap_or_else(|| serde_json::json!({
        "stack_sequence": [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3],
        "notation": "3 · 4 · 5"
    }));
    let picked = q
        .adversary
        .as_ref()
        .and_then(|adv| {
            let level = q.level.unwrap_or(0).to_string();
            root.get("adversaries")
                .and_then(|v| v.get(adv))
                .and_then(|v| v.get("levels"))
                .and_then(|v| v.get(&level))
                .cloned()
        })
        .unwrap_or(base_default);
    Json(picked).into_response()
}

// --- Spirit-terrain affinity lookup ----------------------------------------

async fn get_spirit_affinity(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let path = state.data_dir.join("spirit-terrain-affinity.json");
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::NOT_FOUND, "spirit-terrain-affinity.json not found".to_string())
            .into_response();
    };
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("parse: {e}")).into_response(),
    }
}

// --- Board geometry --------------------------------------------------------

#[derive(Deserialize)]
struct SaveBoardGeometryRequest {
    board_id: String,
    /// Map of land_id → {terrain, coastal, …}. Only `terrain` and `coastal`
    /// are persisted back to the static geometry; unit counts stay in live state.
    lands: serde_json::Map<String, serde_json::Value>,
}

async fn post_save_board_geometry(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<SaveBoardGeometryRequest>,
) -> impl IntoResponse {
    // Map board_id (e.g., "A", "B", "E") to the on-disk file (base_A.json / je_E.json).
    let boards_dir = state.data_dir.join("boards");
    let candidates = [
        format!("base_{}.json", req.board_id),
        format!("je_{}.json", req.board_id),
    ];
    let mut found: Option<(PathBuf, serde_json::Value)> = None;
    for cand in &candidates {
        let p = boards_dir.join(cand);
        if p.exists() {
            let raw = match std::fs::read_to_string(&p) {
                Ok(s) => s,
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("read error {p:?}: {e}")).into_response()
                }
            };
            let json: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("parse error {p:?}: {e}")).into_response()
                }
            };
            found = Some((p, json));
            break;
        }
    }
    let Some((path, mut board_json)) = found else {
        return (
            StatusCode::NOT_FOUND,
            format!("board {} not found in {boards_dir:?}", req.board_id),
        )
            .into_response();
    };

    // Update each matching land's `terrain` + `coastal`. Keep `starting_dahan`
    // and any other metadata already on disk. Preserve the ocean (land 0)
    // untouched.
    if let Some(lands_obj) = board_json
        .get_mut("lands")
        .and_then(|v| v.as_object_mut())
    {
        for (land_id, incoming) in &req.lands {
            if land_id == "0" {
                continue;
            }
            let Some(existing) = lands_obj.get_mut(land_id) else {
                continue;
            };
            let Some(existing_obj) = existing.as_object_mut() else {
                continue;
            };
            if let Some(terrain) = incoming.get("terrain").and_then(|v| v.as_str()) {
                existing_obj.insert(
                    "terrain".to_string(),
                    serde_json::Value::String(terrain.to_string()),
                );
            }
            if let Some(coastal) = incoming.get("coastal").and_then(|v| v.as_bool()) {
                existing_obj.insert(
                    "coastal".to_string(),
                    serde_json::Value::Bool(coastal),
                );
            }
        }
    }

    // Mark the source field to show it's been corrected by the user
    board_json["source"] = serde_json::Value::String(format!(
        "corrected via si-live UI {}",
        chrono_like_ts()
    ));

    let json_out = match serde_json::to_string_pretty(&board_json) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("serialize error: {e}")).into_response(),
    };
    if let Err(e) = std::fs::write(&path, json_out + "\n") {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("write error {path:?}: {e}")).into_response();
    }
    Json(serde_json::json!({ "status": "saved", "path": path.display().to_string() })).into_response()
}

fn chrono_like_ts() -> String {
    // Avoid pulling chrono for one timestamp. Use SystemTime + manual format.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("(epoch {secs})")
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
