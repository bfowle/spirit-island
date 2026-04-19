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

## Phase status

- [x] Phase 0: rules-check skill + Shadows opener correction
- [x] Phase A scaffolding: Rust workspace compiles, Vue frontend builds
- [ ] Phase A MVP play-test: Shadows vs. England L3, 8-round game
- [ ] Phase B: MCP server wired into Claude Code + `si-at-the-table` uses it
- [ ] Phase C: probability / chart.js visualizations
- [ ] Phase D: all boards/spirits/adversaries/scenarios
