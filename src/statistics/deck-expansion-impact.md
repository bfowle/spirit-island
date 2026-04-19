# Deck Expansion Impact — headline statistics

```admonish abstract title="Summary"
Each Spirit Island expansion adds cards to the Minor, Major, Fear, Event, and Blight decks. Those additions **dilute** the base pool — drawing a card from the Fear deck at Base-only is a different probability distribution than drawing one from Base+B&C+JE+NI. This chapter renders the per-expansion-combo impact across all five decks, using deck data deterministically fetched from [spiritislandwiki.com](https://spiritislandwiki.com) via `scripts/wiki-fetch.py` and analyzed by `scripts/deck-stats.py` + `scripts/deck-simulate.py`.
```

## Pool sizes per expansion combo

| Combo | Minor | Major | Fear | Event | Blight |
|-------|-------|-------|------|-------|--------|
| **Base only (Horizons + Base game)** | 36 | 22 | 15 | 0 | 2 |
| **Base + B&C** | 66 | 43 | 30 | 23 | 8 |
| **Base + JE** | 69 | 46 | 21 | 30 | 9 |
| **Base + B&C + JE** | 99 | 66 | 36 | 53 | 15 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 100 | 78 | 50 | 62 | 23 |
| **NI additions only (delta from JE)** | 1 | 13 | 9 | 9 | 8 |

## Expected per-game outcomes (Monte Carlo, 5 000 trials × 8 rounds × 1 player)

Each trial simulates 8 invader rounds using a Gaussian fear-generation model (mean 2.5 fear/turn, σ=1) and sequential draws from the Fear/Event/Blight decks per expansion combo. Fear threshold transitions at TL1→TL2 (4 fear) and TL2→TL3 (8 fear) trigger Fear-card draws. Events draw once per turn (when an Event deck is in play). Blight cards draw at game start + on track-fill. Values are means; the 95% confidence intervals are reported in the JSON data, but omitted here for readability.

| Combo | Fear drawn | Fear output | Events | Blight cards | Spirit favor | Invader favor | Net favor |
|-------|-----------:|------------:|-------:|-------------:|-------------:|--------------:|----------:|
| **Base only (Horizons + Base game)** | 2.00 | 0.00 | 0.0 | 0.00 | 0.65 | 0.00 | **+0.65** |
| **Base + B&C** | 2.00 | 3.49 | 8.0 | 1.89 | 7.03 | 5.94 | **+1.09** |
| **Base + JE** | 2.00 | 3.97 | 8.0 | 1.89 | 10.72 | 5.86 | **+4.87** |
| **Base + B&C + JE** | 2.00 | 3.79 | 8.0 | 1.89 | 9.05 | 5.88 | **+3.17** |
| **All expansions (Base + B&C + JE + NI + Promos)** | 2.00 | 4.66 | 8.0 | 1.90 | 9.20 | 5.55 | **+3.65** |
| **NI additions only (delta from JE)** | 2.00 | 10.00 | 8.0 | 1.90 | 10.32 | 3.56 | **+6.76** |

## Key readings

- **Base-only is un-eventful by design.** No Event deck, 2 Blight cards only. Games are deterministic up to the spirits' own plays.
- **+B&C adds the Event deck** (23 cards) and the full Blight deck (8 cards). That's ~+8 events drawn per game — significant stochastic variance injection.
- **+JE events skew slightly more spirit-favorable** (net +4.87 hits/game vs. +1.09 for B&C alone). JE Minor/Major pools also dilute the base element distribution — see per-deck chapters.
- **NI events are the most spirit-favoring** when counted alone (+6.76), but NI adds only 9 events (small pool, high variance per game).
- **Fear drawn stabilizes at ~2 cards/game** across combos (2 threshold crossings × 1 draw each at 1-player scale). At higher player counts the thresholds scale linearly — expected fear drawn drops per player.

## Cross-references

- [Fear Deck Expected Value](fear-deck-expected-value.md) — per-expansion Fear-card breakdown + stage distribution.
- [Event Deck Risk Profiles](event-deck-risk-profiles.md) — balanced vs. skewed events by expansion.
- [Blight Deck Severity](blight-deck-severity.md) — how catastrophic each expansion's Blight additions are.
- [Token Dilution](token-dilution.md) — per-token-type frequency across pools.
- [Minor vs. Major Draft Rates](major-vs-minor-rates.md) — how Minor/Major draft composition shifts.
