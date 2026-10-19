# Blight Deck — Severity per Expansion

```admonish abstract title="Scope"
One Blight Card is drawn at setup and placed Healthy Island side up without looking at the back (core rulebook setup step 5; with Jagged Earth, the whole shuffled stack sits on the Blight space and you use the top card). Setup puts **2 Blight per player + 1** on it, or 5 per player + 1 with no Blight Card: a 1-player game starts with 3, a 4-player game with 9 (errata, NI rulebook p.5 and the FAQ; the Horizons of Spirit Island board already prints the +1, so don't add another). The +1 is setup only. When the last Blight leaves the Healthy side, flip the card and add the number per player printed on the other side, with no +1. With an extra board, count boards instead of players, at setup and on the flip (Jagged Earth rulebook). **Still-Healthy** cards don't end the healthy phase: when their pool runs out you draw a new Blight Card and go straight to its reverse side (Jagged Earth rulebook). In solo play you may redraw a Blighted card that has only 2 Blight per player, but not a Still-Healthy card (Branch & Claw rulebook, Solo Play; NI rulebook p.14).

Severity varies hugely — some cards permanently cripple your damage math (`All Things Weaken`) while others are one-time effects (`A Pall Upon the Land`). We use a severity proxy: count of `Destroy`, `Add Blight`, and `Ongoing` keywords in the flipped-side text (the Ongoing weight is doubled since it compounds across turns). It is a keyword count, not a measure of difficulty.
```

## Blight deck per expansion

| Combo | Pool | Mean severity proxy |
|-------|-----:|--------------------:|
| **Base only (Horizons + Base game)** | 2 | 1.00 |
| **Base + B&C** | 8 | 0.75 |
| **Base + JE** | 9 | 1.00 |
| **Base + B&C + JE** | 15 | 0.87 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 23 | 0.87 |
| **NI additions only (delta from JE)** | 8 | 0.88 |

The 2 base cards are **Downward Spiral** and **Memory Fades to Dust**. Branch & Claw adds 6 (Tipping Point, its seventh, is dropped by Nature Incarnate's "Dropped Cards" list, NI rulebook p.5), Jagged Earth 7 and Nature Incarnate 8, for 23 active cards. This matches the engine's Blight Card list (23 cards plus the retired Tipping Point and the no-card "Blight" option) and the Steam game's allowed pool (`data/references/rules/open-questions.md` Q15).

## Top-5 worst-draw Blight cards (all expansions)

- **All Things Weaken** (Jagged Earth - Plain) — severity 4, blight/player 3
- **Disintegrating Ecosystem** (Branch and Claw) — severity 2, blight/player 5
- **Intensifying Exploitation** (Nature Incarnate) — severity 2, blight/player 5
- **Untended Land Crumbles** (Jagged Earth - Plain) — severity 2, blight/player 4
- **A Pall Upon the Land** (Branch and Claw) — severity 1, blight/player 3

The fifth slot is a ten-way tie at severity 1; A Pall Upon the Land is listed only because it sorts first. The proxy also misses repetition and some wording: Downward Spiral destroys Presence every Invader Phase but scores 1 (no "Ongoing" keyword), and Strong Earth Shatters Slowly adds Blight but scores 0 (the pattern matches "add", not "adds").

## Blight Cards drawn per game

A game draws 1 Blight Card at setup and another only when a Still-Healthy card's pool runs out, so the count depends on how many Still-Healthy cards sit above the first Blighted card in the shuffled stack. If the island goes all the way to a Blighted side, the expected number of cards drawn is the expected position of the first Blighted card: (N + 1) ÷ (K + 1) for a pool of N cards with K Blighted (negative hypergeometric).

| Combo | Pool N | Still-Healthy | Blighted K | Cards drawn by the time the island is Blighted |
|-------|-----:|-----:|-----:|-----:|
| **Base only (Horizons + Base game)** | 2 | 0 | 2 | 1 |
| **Base + B&C** | 8 | 0 | 8 | 1 |
| **Base + JE** | 9 | 2 | 7 | 10 ÷ 8 = 1.25 |
| **Base + B&C + JE** | 15 | 2 | 13 | 16 ÷ 14 ≈ 1.14 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 23 | 4 | 19 | 24 ÷ 20 = 1.20 |
| **NI additions only (delta from JE)** | 8 | 2 | 6 | 9 ÷ 7 ≈ 1.29 |

The Still-Healthy cards are Invaders Find the Land to Their Liking and Strong Earth Shatters Slowly (Jagged Earth), and The Border of Life and Death and Thriving Crops (Nature Incarnate). With all 23 cards, the number of Blight per player printed on the flipped side runs from 1 to 5 and averages 73 ÷ 23 ≈ 3.17.

```admonish note title="Stat Insight"
`scripts/deck-simulate.py` (5 000 trials × 8 turns, solo, seed 42, run 2026-10-19) follows the rules above. Its Blight inflow is an **unsourced placeholder** (net mean 0.5 Blight per turn from Turn 2), so only the cards-drawn column is driven by the rules; the Blighted share is illustrative. The simulated means sit below the exact column because not every trial reaches a Blighted side within 8 turns.

| Combo | Blight cards drawn (mean, 95% CI) | Trials Blighted by turn 8 |
|-------|-----------------------------------|-----:|
| Base only | 1.00 (1.00, 1.00) | 70.3% |
| Base + B&C | 1.00 (1.00, 1.00) | 69.9% |
| Base + JE | 1.09 (1.08, 1.10) | 61.5% |
| Base + B&C + JE | 1.05 (1.05, 1.06) | 65.7% |
| All expansions | 1.08 (1.07, 1.09) | 65.6% |
| NI additions only | 1.14 (1.13, 1.15) | 63.1% |
```
