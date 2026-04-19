# Blight Deck — Severity per Expansion

```admonish abstract title="Scope"
Blight cards are drawn at game start + whenever the Blight track fills. Their severity varies hugely — some permanently cripple your damage math (`All Things Weaken`) while others are mild (`Wildfires`, `A Pall Upon the Land`). We use a severity proxy: count of `Destroy`, `Add Blight`, and `Ongoing` keywords in the text (the Ongoing weight is doubled since it compounds across turns).
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

## Top-5 worst-draw Blight cards (all expansions)

- **All Things Weaken** (Jagged Earth - Plain) — severity 4, blight/player 3
- **Disintegrating Ecosystem** (Branch and Claw) — severity 2, blight/player 5
- **Intensifying Exploitation** (Nature Incarnate) — severity 2, blight/player 5
- **Untended Land Crumbles** (Jagged Earth - Plain) — severity 2, blight/player 4
- **A Pall Upon the Land** (Branch and Claw) — severity 1, blight/player 3

## Monte Carlo: expected Blight cards drawn per 8-round game

| Combo | Pool | Blight cards drawn (mean, 95% CI) |
|-------|-----:|-----------------------------------|
| **Base + B&C** | 8 | 1.89 (1.88, 1.91) |
| **Base + JE** | 9 | 1.89 (1.87, 1.91) |
| **Base + B&C + JE** | 15 | 1.89 (1.87, 1.91) |
| **All expansions (Base + B&C + JE + NI + Promos)** | 23 | 1.90 (1.88, 1.92) |
| **NI additions only (delta from JE)** | 8 | 1.90 (1.88, 1.92) |
