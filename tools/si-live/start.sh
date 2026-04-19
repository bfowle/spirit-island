#!/usr/bin/env bash
#
# Launch the si-live dev stack: axum backend (port 7777) + Vite dev server
# (port 5173). Vite proxies /api to the backend; `data/current-game.json`
# is the shared source of truth for game state.
#
# Usage:
#   ./tools/si-live/start.sh           # dev mode, hot-reload frontend
#   SI_LIVE_BUILD=1 ./tools/si-live/start.sh  # prebuild frontend, serve via axum

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"

export SI_STATE_PATH="${SI_STATE_PATH:-$REPO_ROOT/data/current-game.json}"
export SI_BIND="${SI_BIND:-127.0.0.1:7777}"

cleanup() {
  [[ -n "${BACKEND_PID:-}" ]] && kill "$BACKEND_PID" 2>/dev/null || true
  [[ -n "${FRONTEND_PID:-}" ]] && kill "$FRONTEND_PID" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

MANIFEST="tools/si-live/Cargo.toml"

if [[ -n "${SI_LIVE_BUILD:-}" ]]; then
  echo "==> building frontend (production)..."
  (cd tools/si-live/frontend && npm run build)
  echo "==> starting axum (serves prebuilt dist + API)..."
  cargo run --release --manifest-path "$MANIFEST" -p si-backend &
  BACKEND_PID=$!
  wait "$BACKEND_PID"
else
  echo "==> starting axum backend on $SI_BIND ..."
  cargo run --manifest-path "$MANIFEST" -p si-backend &
  BACKEND_PID=$!
  sleep 1

  echo "==> starting Vite dev server (frontend at http://127.0.0.1:5173) ..."
  (cd tools/si-live/frontend && npm run dev) &
  FRONTEND_PID=$!

  echo "==> si-live running."
  echo "    Frontend: http://127.0.0.1:5173"
  echo "    Backend:  http://$SI_BIND"
  echo "    State:    $SI_STATE_PATH"
  echo "    Press Ctrl-C to stop."
  wait
fi
