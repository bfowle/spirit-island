# Fear Deck — Expected Value per Expansion

```admonish abstract title="Scope"
Fear cards are drawn from a shuffled deck when the Fear pool crosses terror-level thresholds. Their effects resolve at the player's current Terror Level, so each card contributes up to three distinct effects (stage 1, 2, 3). This chapter breaks down the deck per expansion combo.
```

## Fear pool size + stage composition

| Combo | Total cards | Stage 1 text | Stage 2 text | Stage 3 text |
|-------|------------:|-------------:|-------------:|-------------:|
| **Base only (Horizons + Base game)** | 15 | 15 | 15 | 15 |
| **Base + B&C** | 30 | 30 | 30 | 30 |
| **Base + JE** | 21 | 21 | 21 | 21 |
| **Base + B&C + JE** | 36 | 36 | 36 | 36 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 50 | 50 | 50 | 50 |
| **NI additions only (delta from JE)** | 9 | 9 | 9 | 9 |

## Average Fear-output per card by stage

"N Fear" instructions embedded in each card's text, averaged across cards that have that stage's text.

| Combo | Stage 1 avg fear | Stage 2 | Stage 3 |
|-------|-----------------:|--------:|--------:|
| **Base only (Horizons + Base game)** | 0.00 | 0.00 | 0.00 |
| **Base + B&C** | 0.00 | 0.00 | 0.00 |
| **Base + JE** | 0.00 | 0.00 | 0.05 |
| **Base + B&C + JE** | 0.00 | 0.00 | 0.03 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 0.04 | 0.02 | 0.04 |
| **NI additions only (delta from JE)** | 0.11 | 0.11 | 0.11 |

## Spirit- vs. Invader-favoring per stage

Heuristic classification via keyword regex — how many cards (per stage) contain pro-spirit phrasing (`N Fear`, `Destroy N Explorer/Town/City`, `Push N`, `Remove Blight`, etc.) vs. pro-invader phrasing (`add N Explorer`, `extra build`, `destroy Dahan`, `add Blight`, etc.). Cards may hit both.

| Combo | Spirit-favor (s1/s2/s3) | Invader-favor (s1/s2/s3) |
|-------|-------------------------|--------------------------|
| **Base only (Horizons + Base game)** | 6/4/3 | 0/0/0 |
| **Base + B&C** | 7/6/5 | 0/0/0 |
| **Base + JE** | 8/5/5 | 0/0/0 |
| **Base + B&C + JE** | 9/7/7 | 0/0/0 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 12/11/11 | 0/0/0 |
| **NI additions only (delta from JE)** | 2/2/2 | 0/0/0 |

## Monte Carlo: expected Fear-deck output per game

5 000 trials × 8 rounds × 1 player. `Fear drawn` is cards drawn through terror crossings; `Fear output` sums every `N Fear` instruction on those cards at the stage they resolve.

| Combo | Fear cards drawn | Fear output (mean) | 95% CI |
|-------|-----------------:|-------------------:|--------|
| **Base only (Horizons + Base game)** | 2.00 | 0.00 | (0.00, 0.00) |
| **Base + B&C** | 2.00 | 3.49 | (3.45, 3.53) |
| **Base + JE** | 2.00 | 3.97 | (3.93, 4.02) |
| **Base + B&C + JE** | 2.00 | 3.79 | (3.74, 3.83) |
| **All expansions (Base + B&C + JE + NI + Promos)** | 2.00 | 4.66 | (4.61, 4.72) |
| **NI additions only (delta from JE)** | 2.00 | 10.00 | (9.96, 10.03) |

## Interpretation

- Base-only Fear output is effectively 0 in this model — no Event deck supplies extra Fear, and per-card Fear at stage 1 is rarely encoded as a literal "N Fear" directive.
- Adding B&C lifts Fear output significantly through the Event deck's fear-bearing cards (fear_output ~3.5/game).
- NI's Fear cards average the highest per-stage Fear output — hence the NI-only sim spikes. In practice you'd combine NI with base+B&C+JE, which tames the average due to pool dilution.
