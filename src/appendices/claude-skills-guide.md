# Claude Skills Companion

This book is static. The Claude skills that ship with it are how it becomes an active learning loop.

This appendix documents the six skills, their invocation patterns, the data they read/write, and how they work together.

## Install

From the repo root:

```bash
./scripts/install-skills.sh
```

This symlinks the three MVP skills (`si-daily-challenge`, `si-post-game`, `si-at-the-table`) into `~/.claude/skills/`, so Claude Code recognizes them by name.

Uninstall:

```bash
./scripts/install-skills.sh --uninstall
```

## The six skills

### MVP (shipped now)

#### `si-daily-challenge` — today's drill

**When**: "daily Spirit Island challenge", "SI drill today", `/si-daily-challenge`.

**What it does**: picks today's spirit × adversary × level × learning goal based on `data/playlog.csv`. Anti-repeats recent spirits; progresses difficulty by 2-wins-in-a-row.

**Output**: a compact markdown block with the challenge, links to chapters to read first, a "write on paper: T3 milestone" prompt.

**Depends on**: `data/playlog.csv`, `data/spirits.json`, `data/adversaries.json`.

#### `si-post-game` — log + reflect

**When**: "SI post-game", "log my game", any sentence describing a just-finished game.

**What it does**: normalizes Brett's shorthand into a CSV row, appends to `data/playlog.csv`, then surfaces 1–2 likely learning points by diffing his notes against the spirit chapter's Common Mistakes.

**Output**: logged-confirmation line, 1–2 takeaways with chapter links, rolling stats, next-session suggestion.

**Depends on**: `data/playlog.csv`, spirit chapters.

#### `si-at-the-table` — live tactical lookup

**When**: Brett mid-game describing a board state + asking a question.

**What it does**: reads the spirit + adversary chapter, returns 2–3 options with tradeoffs. Never solves the turn.

**Output**: terse options-with-tradeoffs block. Designed to be read in 10 seconds at the table.

**Hard rule**: anti-alpha. Never prescribes specific plays.

**Depends on**: chapter content.

### Deferred (M2–M4)

#### `si-combo-drill` — 2-spirit pairing drill

**Status**: skeleton stub only; full logic lands with Part VI in M3.

#### `si-matchup-drill` — adversary-focused drill

**Status**: skeleton stub only; full logic lands with the adversary chapters in M2.

#### `si-aspect-explorer` — aspect recommendation

**Status**: skeleton stub only; full logic lands with `spirits/aspects.md` in M4.

## The loop

<pre class="mermaid">
graph LR
  A[si-daily-challenge] --> B[Read chapter 15min]
  B --> C[Play the challenge]
  C --> D[si-post-game]
  D --> A
  C -.->|mid-game| E[si-at-the-table]
</pre>

1. Morning: `si-daily-challenge` picks today's drill.
2. Pre-game: 15 min with the spirit + adversary chapter.
3. Play.
4. Mid-game if needed: `si-at-the-table` for tactical signal.
5. Post-game: `si-post-game` logs + surfaces 1–2 learnings.
6. Tomorrow's challenge shifts based on today's result.

After ~20–30 loops, you'll notice you're reaching for chapter less — patterns are in your head.

## The living-document feedback path

Running the skills changes the data, which changes the book:

- `data/playlog.csv` grows. The [Self-Tracking](../statistics/self-tracking.md) chapter teaches you to read your own data.
- `si-post-game` outputs become revision tickets. If a chapter's common-mistake list doesn't predict what actually cost you games, the chapter needs updating.
- New chapters get added as your ladder progresses — stub → full as you learn that spirit.

## Data schema

### `data/playlog.csv`

```
date,player_count,spirits,aspects,adversary,level,scenario,board,result,rounds,fear_level_end,blight_end,notes
```

- `date`: ISO YYYY-MM-DD
- `player_count`: integer
- `spirits`: comma-separated slug list (for multi-spirit games)
- `aspects`: aspect names, comma-separated
- `adversary`: slug from `data/adversaries.json`
- `level`: integer 0–6
- `scenario`: slug from `data/scenarios.json` or empty
- `board`: letter A–F, letter-list, or empty
- `result`: `win` | `loss`
- `rounds`: integer
- `fear_level_end`: integer (fear pool count at game end)
- `blight_end`: integer
- `notes`: freeform, Brett's verbatim commentary

### `data/spirits.json`

Schema at file top. Key fields used by skills:

- `brett_priority`: prioritize in daily-challenge rotation
- `chapter_status`: `full` vs `stub`; only `full` chapters are linked as "read first"
- `archetypes`: used by `si-combo-drill` once shipped

### `data/adversaries.json`

Per-adversary: `slug`, `name`, `expansion`, `difficulty_levels`, `pressure_shape`, `chapter_path`, `chapter_status`.

### `data/scenarios.json`

Per-scenario: `slug`, `name`, `expansion`, `difficulty_mod`, `pressure_shape`, `chapter_path`.

## Extending the skills

The skills are plain markdown with YAML frontmatter (per Claude's `writing-skills` conventions). To add a new skill or change behavior:

1. Edit `skills/{skill}/SKILL.md`.
2. Re-run `./scripts/install-skills.sh --force` to refresh symlinks.

## Privacy

All skills write to `data/playlog.csv` locally. Nothing is sent anywhere except what Claude Code normally sends when you invoke a skill in-session.

---

*Last revised: 2026-04-19*
