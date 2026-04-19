# Spirit Island Mastery

A stats-driven strategy and learning guide for [Spirit Island](https://www.greaterthangames.com/products/spirit-island), paired with active-learning Claude skills for daily challenges, post-game tracking, and at-the-table tactical lookups.

## What's here

- **`src/`** — mdbook source for the full mastery guide (9 Parts, 100+ chapters at stub or full state).
- **`skills/`** — Claude skills for the active-learning loop (3 MVP + 3 deferred).
- **`data/`** — playlog + reference JSON/CSV that the skills read/write.
- **`templates/`** — canonical chapter templates (Rei's BGG format for spirits).
- **`scripts/`** — install + scaffold helpers.
- **`theme/`** — vendored Gruvbox mdbook theme.

## Build

Requirements: Rust toolchain, `mdbook` 0.4.52, `mdbook-admonish` preprocessor.

```bash
# One-time setup
cargo install mdbook --version 0.4.52
cargo install mdbook-admonish
mdbook-admonish install --css-dir theme .

# Build
mdbook build

# Serve locally with live reload
mdbook serve --open
```

Output goes to `./book/`.

## Install the Claude skills

```bash
./scripts/install-skills.sh
```

This symlinks the MVP skills into `~/.claude/skills/` so Claude recognizes them by name. Uninstall with `--uninstall`.

## The active-learning loop

1. `si-daily-challenge` — generates today's spirit × adversary × level × learning goal.
2. Read the chapter (15 min).
3. Play.
4. `si-at-the-table` mid-game if you need tactical signal.
5. `si-post-game` to log + surface 1–2 learnings.
6. Tomorrow's challenge shifts based on today's result.

## Deployment

Planned: GitHub Pages at `https://bfowle.github.io/spirit-island/` via the `.github/workflows/build-docs.yml` action. Not yet live.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for tone, citation rules, and ethical sourcing.

This book is a synthesis of the community's public deep-dives — Rei's BGG guides, latentoctopus's openings, mindwanderer's stats, Spirited Discussion podcast analysis — formatted in Rei's canonical spirit-chapter spine. Every pattern claim is credited; no verbatim copying.

## License

- **Prose & strategy content**: CC BY-SA 4.0 (see [LICENSE-CONTENT](./LICENSE-CONTENT)).
- **Scripts & skill code**: MIT (see [LICENSE-CODE](./LICENSE-CODE)).

## Status

v0.1 — MVP. 18 chapters authored; 80+ chapters at stub state pending future milestones. See [Meta Changelog](./src/appendices/changelog-of-meta.md) for the running log.
