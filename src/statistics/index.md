# Statistics: Why and How

Spirit Island is a game of probabilities layered under strict mechanics. You can win by feel — most experienced players do — but at higher difficulty levels the feel-based approach hits a ceiling. Beyond that ceiling, counting starts paying.

This part gives you the mental models, the stat sources, and the reusable reasoning tools for playing the **underlying math** as a first-class part of your game.

## What's in this part

- **[Reading mindwanderer](reading-mindwanderer.md)** — how to interpret the community win-rate dashboard without misreading aggregate data. Includes Wilson-CI primer.
- **[Digital vs. Tabletop](digital-vs-tabletop.md)** — why the Steam app's win-rate distributions don't map 1:1 to your physical table.
- **[Fear Card Expected Value](fear-card-expected-value.md)** — per-expansion Fear pool distribution + Monte Carlo per-game output.
- **[Event Deck Risk Profiles](event-deck-risk-profiles.md)** — balance + top spirit/invader-favoring events.
- **[Deck Expansion Impact](deck-expansion-impact.md)** — headline pool sizes + simulated per-game outcomes across all 5 decks × 6 expansion combos.
- **[Expansion Dilution Claims](expansion-dilution-claims.md)** — variance analysis testing "B&C swingy, JE balances, NI dilutes" hypothesis.
- **[Blight Deck Severity](blight-deck-severity.md)** — severity-proxy distribution.
- **[Token-Effect Dilution](token-dilution.md)** — per-token × per-deck × per-combo frequency.
- **[Aspect Power Deltas](aspect-power-deltas.md)** — how much each Aspect shifts its base spirit's win rate.
- **[Major vs. Minor Draft Rates](major-vs-minor-rates.md)** — when Minor-heavy beats Major-shopping.
- **[Self-Tracking Your Games](self-tracking.md)** — what to log, how to use `si-post-game`, the basics of a personal win-rate history.

## The core statistical lenses

Four lenses run through every chapter:

1. **Confidence, not point estimates.** A win rate of 60% across 10 games isn't 60% ± 0 — it's closer to (26%, 88%) via Wilson 95%. Every rate here reports a CI where data supports one.
2. **Conditional on expansion pool.** The Minor deck at base-only is not the Minor deck at all-expansions. A "33% chance of drawing Moon-heavy" means different things in each pool.
3. **Per-turn distributions, not per-game averages.** "Averages 6 fear/game" hides whether that's 6 evenly or 2+0+0+4+0. Shape is the signal.
4. **Correlated vs. independent events.** Fear-card draws depend on fear-pool threshold crossings; Event draws are independent per turn. The math differs.

## Data provenance

- **Wiki-parsed card data** — `data/decks/{minor,major,fear,event,blight}.json`, parsed by `scripts/wiki-fetch.py`.
- **Monte Carlo simulations** — `scripts/deck-simulate.py` runs 5 000+ trials per expansion combo; outputs at `data/references/deck-stats/sim_*.json`.
- **Community win-rate data** — [mindwanderer](https://mindwanderer.net/si/stats.html). Directional; subject to selection bias. Treated accordingly.
