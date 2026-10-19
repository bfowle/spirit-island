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

Pool sizes count only cards that are still in use. Nature Incarnate's "Dropped Cards" list (NI rulebook p.5) removes Outpaced, War Touches the Island's Shores and A Strange Madness Among the Beasts (Events), Growth Through Sacrifice (Minor) and Tipping Point (Blight); Far-off Wars Touch the Island and Roiling Bog and Snagging Thorn replace two of them. Jagged Earth replaced the Branch & Claw Major Sea Monsters with an updated card. The Event counts include Slave Rebellion, which is added only against France, so without France each pool that includes Branch & Claw is one Event smaller (all expansions: 61 + Slave Rebellion). These totals match the Steam game's allowed pools (Minors 100, Majors 78, Fear 50, Events 61 + Slave Rebellion, Blight 23; `data/references/rules/open-questions.md`, Q15).

## Expected per-game outcomes (Monte Carlo, 5 000 trials × 8 rounds × 1 player)

Each trial plays 8 turns with no Adversary or Scenario (`scripts/deck-simulate.py`, seed 42). The rules it follows:

- **Fear.** The Fear Pool holds 4 Fear per player, so solo each Fear Card costs 4 Fear; leftover Fear carries over. The Fear Deck is 9 cards split 3/3/3 by the Terror Level dividers. Earning the 3rd card reveals the Terror Level II divider and earning the 6th reveals Terror Level III, at once; earning the 9th is a Fear victory. Earned cards resolve in the next Fear step at the Terror Level current then (core rulebook setup and "Fear"; `open-questions.md` Q6, Q7).
- **Events.** One per turn. The Turn-1 Event is drawn and discarded without resolving (Jagged Earth rulebook p.12), so 8 turns draw 8 Events and resolve 7. Slave Rebellion is left out (no France).
- **Blight.** One Blight Card at setup with 2 per player + 1 Blight. A second card is drawn only when a Still-Healthy card's pool runs out (Jagged Earth rulebook).

Two inputs are **unsourced placeholders**, not measured rates: Fear generated per turn (Gaussian, mean 2.5, σ = 1) and net Blight per turn from Turn 2 (Gaussian, mean 0.5, σ = 1). Treat every figure that depends on them as illustrative. "Spirit favor" and "Invader favor" are keyword-phrase hits on resolved Fear and Event texts (see [Event Deck Risk Profiles](event-deck-risk-profiles.md)). Values are means; the 95% confidence intervals are in the JSON output.

| Combo | Fear cards earned | Resolved at TL I / II / III | "N Fear" text | Events drawn / resolved | Blight cards | Spirit favor | Invader favor | Net favor |
|-------|-----------:|------:|------------:|-------:|-------------:|-------------:|--------------:|----------:|
| **Base only (Horizons + Base game)** | 4.64 | 1.99 / 2.51 / 0.13 | 0.00 | 0 / 0 | 1.00 | 1.50 | 0.00 | **+1.50** |
| **Base + B&C** | 4.62 | 1.99 / 2.51 / 0.12 | 3.19 | 8 / 7 | 1.00 | 6.69 | 5.44 | **+1.25** |
| **Base + JE** | 4.64 | 1.99 / 2.52 / 0.13 | 3.50 | 8 / 7 | 1.09 | 10.18 | 5.11 | **+5.07** |
| **Base + B&C + JE** | 4.61 | 1.99 / 2.50 / 0.12 | 3.40 | 8 / 7 | 1.05 | 8.59 | 5.27 | **+3.33** |
| **All expansions (Base + B&C + JE + NI + Promos)** | 4.63 | 1.99 / 2.51 / 0.13 | 4.29 | 8 / 7 | 1.08 | 8.79 | 4.96 | **+3.83** |
| **NI additions only (delta from JE)** | 4.62 | 1.99 / 2.51 / 0.13 | 9.08 | 8 / 7 | 1.14 | 9.89 | 3.10 | **+6.79** |

## Key readings

- **Base-only has no Event deck.** A Blight Card, if used, comes from the 2 base cards (the core game also allows the Invader Board's printed Blight space instead), and the Invader, Fear and Power decks are still hidden, shuffled orders.
- **+B&C adds the Event deck** (23 cards, 22 without France) and 6 Blight Cards (8 with the base 2), and makes a Blight Card required (Branch & Claw rulebook). An 8-turn game draws 8 Events and resolves 7.
- **The keyword counts come out net positive in every combo** (+1.25 with B&C, +5.07 with JE, +6.79 for NI's cards alone). They sum resolved Fear and Event texts; for scale, the base-only row, which has Fear Cards only, nets +1.50. Read the Event share as a limit of the phrase classifier rather than a finding: the designer intended Events to be slightly negative on average (darker, [BGG 2506650](https://boardgamegeek.com/thread/2506650)). See [Event Deck Risk Profiles](event-deck-risk-profiles.md).
- **Fear cards earned depend only on the Fear rate, not the expansion.** At the placeholder rate, 8 turns generate about 8 × 2.5 = 20 Fear; leftover Fear short of a full card averages about 1.5, so (20 − 1.5) ÷ 4 ≈ 4.6 cards, as simulated. Cards 1–2 resolve at Terror Level I and cards 3–5 at II; Terror Level III needs 6 cards (24 Fear solo), reached in about 1 trial in 9. Each card costs 4 Fear per player, so at the same total rate a 2-player game earns half as many (2.07 cards in a 2 000-trial run with `--players 2`).
- **Blight cards drawn is 1 plus the Still-Healthy draws.** Base and B&C have no Still-Healthy cards, so exactly 1.

## Cross-references

- [Fear Deck Expected Value](fear-card-expected-value.md) — per-expansion Fear-card breakdown + Terror Level distribution.
- [Event Deck Risk Profiles](event-deck-risk-profiles.md) — balanced vs. skewed events by expansion.
- [Blight Deck Severity](blight-deck-severity.md) — how catastrophic each expansion's Blight additions are.
- [Token Dilution](token-dilution.md) — per-token-type frequency across pools.
- [Minor vs. Major Draft Rates](major-vs-minor-rates.md) — how Minor/Major draft composition shifts.
