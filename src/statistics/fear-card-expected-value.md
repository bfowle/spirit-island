# Fear Deck — Expected Value per Expansion

```admonish abstract title="Scope"
At setup, 9 Fear Cards are dealt from the shuffled pool and split 3/3/3 by the Terror Level II and III dividers (Adversaries change the split). The Fear Pool holds 4 Fear Markers per player (5 with England L6 or an extra board). Each time all of them have been generated you earn the top Fear Card, and leftover Fear carries over. Earning the card that reveals a divider raises the Terror Level at once; earning the last card is a Fear victory. Earned cards stay face down until the next Invader Phase's Fear step, then resolve one at a time at the Terror Level current at that moment, whatever the level was when they were earned (core rulebook setup and "Fear"; FAQ, Fear and Fear Cards; `data/references/rules/open-questions.md` Q6–Q8). Each card prints three texts, one per Terror Level, and only the current level's text resolves. This chapter breaks down the card pool per expansion combo.
```

## Fear card pool size + Terror Level texts

| Combo | Total cards | TL I text | TL II text | TL III text |
|-------|------------:|-------------:|-------------:|-------------:|
| **Base only (Horizons + Base game)** | 15 | 15 | 15 | 15 |
| **Base + B&C** | 30 | 30 | 30 | 30 |
| **Base + JE** | 21 | 21 | 21 | 21 |
| **Base + B&C + JE** | 36 | 36 | 36 | 36 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 50 | 50 | 50 | 50 |
| **NI additions only (delta from JE)** | 9 | 9 | 9 | 9 |

## Average Fear-output per card by Terror Level

"N Fear" instructions embedded in each card's text, averaged across cards that have that Terror Level's text.

| Combo | TL I avg fear | TL II | TL III |
|-------|-----------------:|--------:|--------:|
| **Base only (Horizons + Base game)** | 0.00 | 0.00 | 0.00 |
| **Base + B&C** | 0.00 | 0.00 | 0.00 |
| **Base + JE** | 0.00 | 0.00 | 0.05 |
| **Base + B&C + JE** | 0.00 | 0.00 | 0.03 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 0.04 | 0.02 | 0.04 |
| **NI additions only (delta from JE)** | 0.11 | 0.11 | 0.11 |

## Spirit- vs. Invader-favoring per Terror Level

Heuristic classification via keyword regex — how many cards (per Terror Level) contain pro-spirit phrasing (`N Fear`, `Destroy N Explorer/Town/City`, `Push N`, `Remove Blight`, etc.) vs. pro-invader phrasing (`add N Explorer`, `extra build`, `destroy Dahan`, `add Blight`, etc.). Cards may hit both.

| Combo | Spirit-favor (TL I/II/III) | Invader-favor (TL I/II/III) |
|-------|-------------------------|--------------------------|
| **Base only (Horizons + Base game)** | 6/4/3 | 0/0/0 |
| **Base + B&C** | 7/6/5 | 0/0/0 |
| **Base + JE** | 8/5/5 | 0/0/0 |
| **Base + B&C + JE** | 9/7/7 | 0/0/0 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 12/11/11 | 0/0/0 |
| **NI additions only (delta from JE)** | 2/2/2 | 0/0/0 |

## Monte Carlo: expected Fear-deck output per game

5 000 trials × 8 rounds × 1 player, no Adversary (`scripts/deck-simulate.py`, seed 42). Fear arrives at an **unsourced placeholder rate** (Gaussian, mean 2.5 per turn, σ = 1), so the card counts illustrate the rules rather than predict a real game. Each card costs 4 Fear solo. `Cards earned` counts Fear Cards earned; the TL columns split them by the Terror Level they resolved at. `Fear output` sums every `N Fear` phrase on the resolved Fear texts and on the 7 resolved Events (the Turn-1 Event is discarded unresolved); conditional text counts as if it always fires, and that Fear is not fed back into the Fear Pool.

| Combo | Cards earned | Resolved at TL I / II / III | Fear output (mean) | 95% CI |
|-------|-----------------:|------:|-------------------:|--------|
| **Base only (Horizons + Base game)** | 4.64 | 1.99 / 2.51 / 0.13 | 0.00 | (0.00, 0.00) |
| **Base + B&C** | 4.62 | 1.99 / 2.51 / 0.12 | 3.19 | (3.16, 3.23) |
| **Base + JE** | 4.64 | 1.99 / 2.52 / 0.13 | 3.50 | (3.46, 3.54) |
| **Base + B&C + JE** | 4.61 | 1.99 / 2.50 / 0.12 | 3.40 | (3.35, 3.44) |
| **All expansions (Base + B&C + JE + NI + Promos)** | 4.63 | 1.99 / 2.51 / 0.13 | 4.29 | (4.23, 4.34) |
| **NI additions only (delta from JE)** | 4.62 | 1.99 / 2.51 / 0.13 | 9.08 | (9.03, 9.12) |

```admonish example title="Math"
Cards earned is a budget, not a draw: 8 turns × 2.5 Fear = 20 Fear. Leftover Fear short of a full card averages about 1.5, so (20 − 1.5) ÷ 4 ≈ 4.6 cards. The 3rd card reveals the Terror Level II divider, so cards 1–2 resolve at TL I and card 3 already resolves at TL II. Terror Level III takes 6 cards, 6 × 4 = 24 Fear solo, which this rate reaches in about 1 trial in 9. With N players each card costs 4N Fear, so the same total rate earns 1/N as many cards.
```

## Interpretation

- Base-only Fear output is effectively 0 in this model — no Event deck supplies extra Fear, and base Fear Cards print no literal "N Fear" directive (0.00 per card in the table above).
- Adding B&C lifts Fear output through the Event deck's fear-bearing texts (about 3.2 per game).
- NI's Fear Cards average the highest per-level Fear output — hence the NI-only spike. In practice you'd combine NI with base+B&C+JE, which tames the average through pool dilution.
- The number of Fear Cards earned is the same in every combo because it depends only on the Fear rate and the 4-per-player cost; expansions change which cards you earn, not how many.
