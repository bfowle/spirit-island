# si-live — interactive Spirit Island board-state tool

An always-on local web app for tracking a live Spirit Island game. Replaces typed shorthand in the `si-at-the-table` workflow: click to edit board state, and Claude reads the same `data/current-game.json` file the UI writes.

## Architecture

- **`backend/`** — axum HTTP server on `127.0.0.1:7777`. GET/PUT `/api/state`, persists to `data/current-game.json`.
- **`mcp/`** — MCP stdio server exposing `get_game_state`. Registered with Claude Code as a local MCP server.
- **`common/`** — shared rule engine + schema. Imported by both backend and MCP server so they cannot diverge.
- **`frontend/`** — Vue 3 + Vite SPA. Proxies `/api` to the backend.

All three Rust crates live in one workspace (`tools/si-live/Cargo.toml`). Single `cargo build` compiles everything.

## Quick start

```bash
# first time only — install JS deps
cd tools/si-live/frontend && npm install && cd -

# launch dev stack (hot-reload frontend, cargo run backend)
./tools/si-live/start.sh

# open http://127.0.0.1:5173 in a browser
```

`start.sh` starts both processes and forwards Ctrl-C to both. State is read/written from `data/current-game.json`.

## Production mode

```bash
SI_LIVE_BUILD=1 ./tools/si-live/start.sh
```

Builds the frontend (`dist/`), then launches the backend which serves the built assets directly — no Vite, single port.

## MCP server registration (Phase B)

```bash
# Build the MCP binary
(cd tools/si-live && cargo build --release -p si-mcp)

# Register with Claude Code (exact command TBD — verify current MCP config syntax)
claude mcp add si-live -- $PWD/tools/si-live/target/release/si-mcp
```

## Steam companion (`#/companion`)

A read-only, second-monitor dashboard for the Steam (Handelabra) version. The
backend decodes the game's save files (`steam/`, crate `si-steam`), passes them
through an information firewall (only what a player at the table could know),
and serves forecasts: the coming Invader Phase in resolution order (Ravage →
Build → Explore), per-land Explore odds and access, Fast vs Slow windows, fear
and blight runways, innate thresholds, draw odds, a play-by-play feed, and a
post-game review. Design: `docs/superpowers/specs/2026-10-18-steam-companion-design.md`.

It never reads or writes `data/current-game.json`. It writes only
`data/steam-game.json` and `data/steam-history/<game-id>.jsonl` (gitignored).

```bash
# Live (default): watches the game's save folder while you play on Steam.
./tools/si-live/start.sh                      # then open http://127.0.0.1:5173/#/companion

# Replay a captured game, with play / pause / step / seek in the status strip.
SI_STEAM_MODE=replay SI_STEAM_CAPTURE=data/steam-capture/2026-10-18-test1 ./tools/si-live/start.sh
```

| Variable | Default | Meaning |
|---|---|---|
| `SI_STEAM_MODE` | `live` | `live`, `replay`, or `off` |
| `SI_STEAM_DIR` | `%USERPROFILE%\AppData\LocalLow\Handelabra Games Inc_\Spirit Island` (via `cmd.exe` + `wslpath`) | live save folder |
| `SI_STEAM_CAPTURE` | `data/steam-capture/2026-10-18-test1` | replay folder (`saves/<HHMMSS_mmm>__<name>.dat`) |
| `SI_STEAM_DLL` | Steam install's `SpiritIslandEngine.dll` | engine card, board, and spirit definitions (read at startup, never copied) |
| `SI_STEAM_POLL_MS` | `250` | live poll interval |

Endpoints: `GET /api/companion/state`, `GET /api/companion/stream` (SSE),
`GET /api/companion/health`, `POST /api/companion/replay`
(`{"action":"play","speed":8}`, `pause`, `{"action":"step","by":-1}`,
`{"action":"seek","index":134}`; replay mode only).

Dev tool: `cargo run -p si-steam --bin si-steam-dump -- <command>`:
`check <saves…>` (decode), `sum <save>` (engine summary, hidden data included,
for debugging only), `pub <save>` (public snapshot JSON), `stats|review|backtest
<capture-dir> [index]`, `defs <out-dir>` (export engine definitions).

Tests that need real data (`steam/tests/firewall.rs`, `backtest.rs`) read the
local capture and the installed DLL and skip when either is absent; this repo
is public, so captures are not checked in.

## Phase status

- [x] Phase 0: rules-check skill + Shadows opener correction
- [x] Phase A scaffolding: Rust workspace compiles, Vue frontend builds
- [ ] Phase A MVP play-test: Shadows vs. England L3, 8-round game
- [ ] Phase B: MCP server wired into Claude Code + `si-at-the-table` uses it
- [ ] Phase C: probability / chart.js visualizations
- [ ] Phase D: all boards/spirits/adversaries/scenarios
