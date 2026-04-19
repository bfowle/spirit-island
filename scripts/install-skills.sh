#!/usr/bin/env bash
# Symlink the Spirit Island skills into ~/.claude/skills/ so they're invokable by name.
#
# Usage:
#   ./scripts/install-skills.sh             # install all MVP skills
#   ./scripts/install-skills.sh --uninstall # remove symlinks
#   ./scripts/install-skills.sh --force     # overwrite any existing symlink/file

set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TARGET_DIR="$HOME/.claude/skills"

SKILLS=(
  "si-daily-challenge"
  "si-post-game"
  "si-at-the-table"
)

uninstall=0
force=0

for arg in "$@"; do
  case "$arg" in
    --uninstall) uninstall=1 ;;
    --force) force=1 ;;
    *) echo "unknown arg: $arg" >&2; exit 2 ;;
  esac
done

mkdir -p "$TARGET_DIR"

for skill in "${SKILLS[@]}"; do
  src="$REPO_DIR/skills/$skill"
  dst="$TARGET_DIR/$skill"

  if [ ! -d "$src" ]; then
    echo "skip: $skill (source dir missing)" >&2
    continue
  fi

  if [ "$uninstall" -eq 1 ]; then
    if [ -L "$dst" ] || [ -e "$dst" ]; then
      rm "$dst"
      echo "removed: $dst"
    fi
    continue
  fi

  if [ -e "$dst" ] || [ -L "$dst" ]; then
    if [ "$force" -eq 1 ]; then
      rm "$dst"
    else
      echo "exists (use --force to overwrite): $dst" >&2
      continue
    fi
  fi

  ln -s "$src" "$dst"
  echo "installed: $dst -> $src"
done

echo "Done. Skills live in $TARGET_DIR/"
