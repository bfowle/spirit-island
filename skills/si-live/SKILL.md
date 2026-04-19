---
name: si-live
description: Launch + interact with the interactive Spirit Island board-state tool. State lives at data/current-game.json and is shared between the web UI, this skill, and (Phase B onward) the MCP server.
user-invocable: true
---

# si-live

Interactive Spirit Island board-state tool. Eliminates typed shorthand from the table workflow: the web UI writes `data/current-game.json`; skills (`si-at-the-table`, `si-post-game`) read the same file.

## Prerequisites

- Rust toolchain (for backend + MCP server).
- Node 18+ / npm (for Vue 3 frontend).
- One-time frontend install: `cd tools/si-live/frontend && npm install`.

## How to launch (manual)

The server needs to run outside the Claude Code process — Claude's skills are stdin/stdout and can't host a long-lived web server. Brett launches it once per session:

```bash
./tools/si-live/start.sh
```

Opens two processes:
- **axum backend** on `http://127.0.0.1:7777` — owns `data/current-game.json` writes.
- **Vite dev server** on `http://127.0.0.1:5173` — proxies `/api` to the backend.

Brett opens the frontend URL in a browser, sets up the game (adversary, spirits, board), and clicks through state updates. The backend persists to `data/current-game.json` debounced.

## How Claude uses this skill

**Phase A (current — file read):** every skill that needs live board state (`si-at-the-table`, `si-post-game`) reads `data/current-game.json` directly. Schema is documented at `tools/si-live/common/src/schema.rs`.

**Phase B (planned — MCP):** the `si-mcp` binary exposes `get_game_state`, `compute_innate_threshold`, `compute_draw_probability`, etc. as MCP tools. Skills switch to MCP calls instead of file reads. File-read remains the fallback.

## When to invoke

- User is about to play a game and asks Claude to help track / analyze in real time.
- User says "launch the tool" / "start si-live" / "open the board".
- `si-at-the-table` detects the state file is stale or missing and wants to prompt launch.

Do not auto-launch on every Claude invocation — Brett runs the tool when he wants it up. If the state file is missing or stale, tell Brett to run `./tools/si-live/start.sh`; don't try to spawn it yourself.

## State schema summary

See `tools/si-live/common/src/schema.rs` for the authoritative types. Key shapes:

- `GameState`: `{ version, round, phase, setup, pools, spirits, board_state, log }`
- `Spirit`: `{ energy, card_plays, presence_on_board, presence_on_track_energy, presence_on_track_cardplay, elements_this_turn, hand, discard, forgotten, played_this_turn, growth_options }`
- `Board`: `{ lands: { "<id>": Land } }`
- `Land`: `{ terrain, coastal, explorers, towns, cities, dahan, blight, tokens }`

Tracks are represented as **remaining-covered-slot** arrays. The head of the array is the next slot that will be revealed. Starting Energy/CP come from `presence_energy_track[0]` / `presence_cardplay_track[0]` — this is the invariant `si-rules-check` relies on.

## Related skills

- `si-rules-check` — validate proposed plays against starting state + growth options.
- `si-at-the-table` — read the live state + return Rei-format option trade analysis.
- `si-post-game` — pull end-state from `data/current-game.json` and append to playlog.
- `si-wiki-fetch` — authoritative source for spirit/card/adversary data.
