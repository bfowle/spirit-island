# Reading mindwanderer

[mindwanderer](https://mindwanderer.net/si/stats.html) aggregates player-reported Spirit Island game outcomes into win-rate tables per (spirit × adversary × level × player-count). It's the best public signal for relative spirit strength — but it's **not** ground truth, and reading it well takes a few heuristics.

```admonish tip title="Beyond the stats dashboard"
The headline dashboard at `/stats.html` is only the entry point. [`mindwanderer.net/si/`](https://mindwanderer.net/si/) has **100+ detail sub-pages** — per-spirit (`LightningsSwiftStrike.html`), per-adversary (`TheHabsburgMonarchy.html`), per-scenario, and per-board-layout (`layout_T4.html`) — each with its own blight-flip rate, average score, and difficulty model. If you're prepping a specific matchup, open the spirit and adversary detail pages side by side; the dashboard only surfaces aggregate cells.

When you cite mindwanderer numbers, also read [BGG thread 3555502 — *An Update on the SI Digital Statistics Project*](https://boardgamegeek.com/thread/3555502): the Handelabra analytics migration changed collection methodology mid-stream, so pre- and post-migration numbers aren't strictly comparable.

For broader empirical context, the community's [SI Digital Data Analysis Series](https://boardgamegeek.com/thread/3617445/si-digital-data-analysis-series-5-how-does-each-sp) mines 2.27M games across 13 entries — entries 5–13 cover per-spirit-per-adversary, scenario difficulty, and aspect performance with more narrative than mindwanderer alone.
```

## What it actually is

- **Self-reported**: players submit after their game. Selection bias is real.
- **Cross-table**: digital (Steam + Asmodee Online) + tabletop, blended.
- **Skill-heterogeneous**: a player's 1st Shadows game and their 50th are both in the denominator.

When you see `Shadows vs. England L6: 52% (n=180)`, treat that as:
*"Across a self-selected mix of players, the true underlying win-rate is probably in (44%, 60%) at 95% confidence."*

## The Wilson 95% CI

Win-rate without a CI is just a point estimate. With n=180 and p=0.52 the Wilson 95% interval is approximately **(44.6%, 59.1%)**. At n=25 that same 52% point estimate broadens to roughly (33%, 71%) — a much weaker claim.

Mental shortcut:

- **n < 20**: "we don't know." Treat as a rumor.
- **n = 20–100**: directional but weak. Don't rank-order by 2-3% gaps.
- **n > 100**: gap-meaningful at 5-10% level.
- **n > 500**: gap-meaningful at 2-3% level.

## Comparing two spirits

Don't subtract point estimates. Compute each's Wilson CI; if intervals overlap, you can't confidently say one is stronger.

Example: Shadows 52% (n=180) vs. River 48% (n=95):
- Shadows: (44.6%, 59.1%)
- River: (38.1%, 58.0%)

Intervals overlap heavily → "maybe Shadows is stronger but it's within error bars."

Compare: Bringer 61% (n=220) vs. Thunderspeaker 47% (n=200):
- Bringer: (54.4%, 67.2%)
- Thunderspeaker: (40.2%, 54.0%)

Overlap is tiny → Bringer is plausibly stronger (narrow margin, still reasonable doubt).

## What mindwanderer doesn't capture

- **Scenario effects**: most reports are no-scenario; scenario stats are typically too thin.
- **Expansion mix**: `base+JE` and `base+B&C+JE+NI` often go in the same bucket. Expansion-column data is approximate.
- **Player-count interaction**: 2-handed solo ≠ true 2-player. Filter by player count.
- **Adversary-within-level variation**: L3 England ≠ L3 Scotland. Blended top-lines lose this.

## Workflow

1. Pick the spirit × adversary combo.
2. Filter mindwanderer to your player count.
3. Read the point estimate + n. Compute Wilson bounds (or eyeball: CI width ≈ 1/√n).
4. Compare against the closest alternative spirit via overlapping-intervals test.
5. Cross-reference [Expansion Dilution Claims](expansion-dilution-claims.md) for how your expansion mix shifts the baseline.

## When to ignore mindwanderer entirely

- **You've played <10 games total** — the community's average skill is above yours. Their win rates overstate what you'll experience.
- **You're doing a specific scenario or aspect** — data is thin. Use the spirit chapter's hand-authored analysis instead.
- **You're playing for learning** — highest win rate ≠ most learning-value. A 5%-below-mean spirit can teach more if it's a different archetype than your comfort pick.

## See also

- [Digital vs. Tabletop](digital-vs-tabletop.md) — reasons to discount digital-heavy data.
- [Aspect Power Deltas](aspect-power-deltas.md) — when an Aspect meaningfully changes a spirit's rate.
- [Self-Tracking Your Games](self-tracking.md) — your own log as a calibration layer.
