# Self-Tracking Your Games

Community stats are a starting point; personal stats are ground truth for calibrating strategies against **your** play. This chapter describes what to log, how, and what to do with the log.

## What to capture

MVP schema for `data/playlog.csv`:

| Field | Example | Notes |
|-------|---------|-------|
| `game_id` | `2026-04-19-001` | Unique per game (date + sequence) |
| `date` | `2026-04-19` | ISO date |
| `spirits` | `shadows;thunderspeaker` | Semicolon-separated slugs |
| `adversary` | `england` | slug |
| `level` | `3` | int |
| `scenario` | `blitz` or blank | slug, optional |
| `boards` | `A;B` | Semicolon-separated |
| `expansions` | `base;bnc;je;ni` | Semicolon-separated |
| `result` | `win` / `loss` / `abandoned` | — |
| `final_round` | `7` | int |
| `final_terror` | `3` | 1/2/3 |
| `fear_generated` | `12` | total Fear generated |
| `blight_placed` | `4` | total Blight |
| `notes` | free text | surprises, pivots, mistakes |

## Workflow

1. After every game, append a row to `data/playlog.csv`.
2. Invoke `si-post-game` (Claude skill) to parse the row + prompt for learning points. Surfaces unverified items in played-spirit chapters.
3. Weekly, run a win-rate rollup. Wilson 95% CIs on anything with n ≥ 10.
4. Monthly, compare your rates to mindwanderer for the same (spirit × adversary). Gaps indicate playstyle divergence worth investigating.

## What to do with the log

### Find your strongest matchups

Group by `(spirit × adversary × level)`. For n ≥ 5, compute win rate + Wilson. Your highest-rate tight-CI groups are your practiced edges.

### Find your blind spots

Groups where your rate is 10%+ below community AND your n ≥ 10 → you're under-performing and the signal is real (not tiny-sample noise). Start there.

### Calibrate tempo guesses

`fear_generated` / `final_round` = average Fear/round. Compare to [Tempo](../fundamentals/tempo.md) expected curves — divergence indicates pace issues.

### Detect pivots that worked

In `notes`, record "pivoted from G2 to G3 at T3 — ended T4 at 6E" observations. Across 10+ games, patterns emerge: which pivots reliably work vs. which fix symptoms but lose the game.

## Existing tooling

- **`si-post-game` skill** — accepts game facts, appends to playlog, surfaces unverified items from played-spirit chapters.
- **`si-daily-challenge`** — rotates spirits by priority list + your recent log (less-played spirits get priority).
- **si-live tool** — every state edit persists to `data/current-game.json`; on game end, export the final state into a playlog row.

## What *not* to over-track

Don't log every decision. Signal-to-noise is bad and you'll stop within a week. Track **end state + 2-3 surprising moments per game** — enough to compound over 50 games.

## Sample starter CSV

```csv
game_id,date,spirits,adversary,level,scenario,boards,expansions,result,final_round,final_terror,fear_generated,blight_placed,notes
2026-04-19-001,2026-04-19,shadows-flicker-like-flame,england,3,,A,base,win,7,3,14,1,"L1 innate fired T3 via Haunts draft; no cliff hit; fear-rush cleaned up T6-7"
```

## See also

- [Reading mindwanderer](reading-mindwanderer.md) — Wilson CI + interpretation.
- [Digital vs. Tabletop](digital-vs-tabletop.md) — keep separate logs per mode.
- Skills: `si-post-game`, `si-daily-challenge` in `skills/`.
