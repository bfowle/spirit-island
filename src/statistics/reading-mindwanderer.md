# Reading mindwanderer

[mindwanderer](https://mindwanderer.net/si/stats.html) analyzes Spirit Island Digital game data into win-rate and difficulty tables per (spirit × adversary × level × player-count). It's the best public signal for relative spirit strength — but it's **not** ground truth, and reading it well takes a few heuristics.

```admonish tip title="Beyond the stats dashboard"
The headline dashboard at `/stats.html` is only the entry point. [`mindwanderer.net/si/`](https://mindwanderer.net/si/) has **100+ detail sub-pages** — per-spirit (`LightningsSwiftStrike.html`), per-adversary (`TheHabsburgMonarchy.html`), per-scenario, and per-board-layout (`layout_T4.html`) — each with its own blight-flip rate, average score, and difficulty model. If you're prepping a specific matchup, open the spirit and adversary detail pages side by side; the dashboard only surfaces aggregate cells.

When you cite mindwanderer numbers, also read [BGG thread 3555502 — *An Update on the SI Digital Statistics Project*](https://boardgamegeek.com/thread/3555502): the Handelabra analytics migration changed collection methodology mid-stream, so pre- and post-migration numbers aren't strictly comparable.

For broader empirical context, the community's [SI Digital Data Analysis Series](https://boardgamegeek.com/thread/3617445/si-digital-data-analysis-series-5-how-does-each-sp) (Jonathan4290, 26 entries) mines Spirit Island Digital data covering per-spirit-per-adversary results, scenario difficulty, and aspect performance with more narrative than mindwanderer alone. Its "2.27M" figure counts log lines, not games ([BGG 3569421](https://boardgamegeek.com/thread/3569421)).
```

## What it actually is

- **Spirit Island Digital only**: every game in it was played in the digital app. There is no tabletop data, so see [Digital vs. Tabletop](digital-vs-tabletop.md) before applying it to your table.
- **Regression models**: the site's figures come from regression models fit to that data, not only from raw wins ÷ games. A model estimate adjusts for the other factors in the model, so a Wilson interval computed from a raw count is not its uncertainty. Use the Wilson interval below only where the page gives a raw wins-and-games count.
- **Skill-heterogeneous**: a player's 1st Shadows game and their 50th are both in the data.
- **Selection bias**: it covers the people who play the digital game, at the difficulties they pick.

Where a page does give a raw count, say a hypothetical `Shadows vs. England L6: 52% (n=180)`, treat that as:
*"The true underlying win rate for this population is probably in (44.7%, 59.2%) at 95% confidence."*

## The Wilson 95% CI

Win-rate without a CI is just a point estimate. With n=180 and p=0.52 the Wilson 95% interval is approximately **(44.7%, 59.2%)**. At n=25 that same 52% point estimate broadens to roughly (33.5%, 70.0%) — a much weaker claim.

Mental shortcut (near 50%, the 95% half-width of one rate is 1.96 × √(0.25/n) ≈ 1/√n; the gap between two rates with n games each must exceed about 1.96 × √(0.5/n) ≈ 1.4/√n to clear the noise):

- **n < 20**: "we don't know." Each rate is ± 22 points or more. Treat as a rumor.
- **n = 20–100**: directional but weak. At n = 100 each rate is ± 10 points, and gaps under about 14 points are noise.
- **n = 500**: each rate is ± 4.4 points; gaps under about 6 points are noise.

## Comparing two spirits

Don't subtract point estimates. Compute each's Wilson CI; if intervals overlap, you can't confidently say one is stronger.

Example (hypothetical numbers, for the arithmetic only): Shadows 52% (n=180) vs. River 48% (n=95):
- Shadows: (44.7%, 59.2%)
- River: (38.2%, 57.9%)

Intervals overlap heavily → "maybe Shadows is stronger but it's within error bars."

Compare (also hypothetical): Bringer 61% (n=220) vs. Thunderspeaker 47% (n=200):
- Bringer: (54.4%, 67.2%)
- Thunderspeaker: (40.2%, 53.9%)

The intervals just miss each other (53.9% vs. 54.4%) → Bringer is plausibly stronger (narrow margin, still reasonable doubt).

## What mindwanderer doesn't capture

- **Scenario effects**: scenario cells hold fewer games than no-scenario cells; check n before trusting one.
- **Expansion mix**: check whether the page filters by the expansions in use. A cell that pools games played with different decks mixes different Event, Fear, Blight and Power pools.
- **Player-count interaction**: 2-handed solo ≠ true 2-player. Filter by player count.
- **Adversary-within-level variation**: L3 England ≠ L3 Scotland. Blended top-lines lose this.

## Workflow

1. Pick the spirit × adversary combo.
2. Filter mindwanderer to your player count.
3. Read the point estimate + n. Compute Wilson bounds (or eyeball: 95% half-width ≈ 1/√n near 50%).
4. Compare against the closest alternative spirit via overlapping-intervals test.
5. Cross-reference [Expansion Dilution Claims](expansion-dilution-claims.md) for how your expansion mix shifts the baseline.

## When to ignore mindwanderer entirely

- **You've played <10 games total** — the community's average skill is above yours. Their win rates overstate what you'll experience.
- **You're doing a specific scenario or aspect** — data is thin. Use the spirit chapter's hand-authored analysis instead.
- **You're playing for learning** — highest win rate ≠ most learning-value. A below-average spirit can teach more if it's a different archetype than your comfort pick.

## See also

- [Digital vs. Tabletop](digital-vs-tabletop.md) — why digital-only data may not transfer to your table.
- [Aspect Power Deltas](aspect-power-deltas.md) — when an Aspect meaningfully changes a spirit's rate.
- [Self-Tracking Your Games](self-tracking.md) — your own log as a calibration layer.
