# Expansion Dilution — Is B&C Really Swingier? Does NI Dilute?

```admonish abstract title="The claim under test"
> "B&C alone felt swingy; JE balanced that; NI diluted things."

This chapter tests the claim with per-expansion marginal statistics. For each expansion (Base, B&C, JE, NI) we assign every card to its **home** expansion (the earliest/most-specific set tag) and compute mean, standard deviation, and **coefficient of variation (CV)** — CV = σ/μ, the standard measure of swinginess independent of scale. A higher CV = wider spread of outcomes relative to the average = swingier.

Data pipeline: `scripts/expansion-deltas.py` → `data/references/deck-stats/per_expansion_deltas.json`.
```

## Headline result

**B&C is the swingiest expansion for Fear + Event decks. JE is the most balanced. NI does NOT dilute Fear — it actually brings the heaviest-hitting Fear cards per card added — but NI DOES dilute Major Powers.** Full breakdown below.

## Fear deck: swinginess per expansion (net-favor)

Net-favor = (#spirit-favoring keyword hits) − (#invader-favoring keyword hits) per card, summed across all three terror-level stages. Higher CV means the card-to-card swing is wider.

| Expansion | n | mean net-favor | σ | **CV** | median |
|-----------|---|---------------:|--:|-------:|-------:|
| Base game | 15 | 0.40 | 0.51 | 1.27 | 0 |
| **Branch & Claw** | 15 | 0.13 | 0.35 | **2.64** ← swingiest | 0 |
| Jagged Earth | 6 | 0.50 | 0.55 | **1.09** ← most balanced | 0 |
| Nature Incarnate | 9 | 0.33 | 0.71 | 2.12 | 0 |

**Reading**: B&C's Fear-card net-favor has the highest CV (2.64) — individual draws are nearly 3× as variable relative to the mean as JE's (1.09). Your intuition is measurably correct: **within the same Terror level, B&C Fear cards are the most unpredictable swing-cards in the game.**

## Fear deck: dilution analysis

The "dilution" claim is about **cumulative mean** — when you add a later expansion's Fear cards to the running pool, how does the combined average shift? Negative delta = new expansion is weaker (dilutes the pool); positive delta = stronger (concentrates variance).

| Added | +cards | Mean per added card (fear_amount) | Cumulative-pool mean | Δ vs. previous |
|-------|-------:|----------------------------------:|---------------------:|---------------:|
| Base game | +15 | 0.00 | 0.00 | (baseline) |
| Branch & Claw | +15 | 0.00 | 0.00 | +0.00 |
| Jagged Earth | +6 | 0.17 | 0.03 | **+0.03** |
| Nature Incarnate | +9 | 0.33 | 0.09 | **+0.06** |

**NI does NOT dilute the Fear deck** — NI's 9 Fear cards carry the highest per-card embedded "N Fear" directives (mean 0.33), so adding NI *concentrates* Fear-generation in the pool. What NI does change is the **mix**: more Fear cards that explicitly generate Fear on draw (via event-style triggers), vs. base/B&C Fear cards that mostly change game rules.

Where's the "dilution" feeling coming from then? Probably from the **Event deck**, which is larger and shuffled every game — see below.

## Event deck: the real swingy one

| Expansion | n | mean net-favor | σ | **CV** |
|-----------|---|---------------:|--:|-------:|
| **Branch & Claw** | 23 | 0.09 | 0.95 | **10.92** ← extreme swing |
| Jagged Earth | 30 | 0.53 | 1.17 | 2.19 |
| Nature Incarnate | 9 | 0.78 | 0.83 | **1.07** ← tight |

**B&C's Event deck is enormously swingy** — CV of **10.92** because the mean is near-zero (0.09) but the stdev is almost 1.0. That means B&C Events individually swing wildly positive or negative but **cancel out on average**. JE adds 30 more Events with a consistently pro-spirit lean (mean +0.53, CV 2.19). NI's 9 Events are the most tightly clustered (mean +0.78, CV 1.07) — NI Events lean pro-spirit with low variance.

### Interpretation

- **B&C Events feel random** because they are: each card is either a big boon or a big bust with few middle-ground cards.
- **JE Events balance** by adding many consistently-themed cards that pull the mean pro-spirit.
- **NI Events concentrate** — few cards, all pro-spirit.

## Minor deck: mean Fear output per card

| Expansion | n | mean fear_amount | CV |
|-----------|---|-----------------:|---:|
| Base | 50 | 0.58 | 1.81 |
| B&C | 30 | 0.67 | 1.64 |
| JE | 33 | 0.58 | 1.89 |
| NI | 1 | 1.00 | 0.00 |

**B&C Minors generate the most Fear per card** (mean 0.67 and lower CV — more reliable). JE Minors match base in mean Fear but have slightly higher variance. NI only adds 1 new Minor (most NI content is Uniques + Majors).

## Major deck: **NI does dilute the Major pool**

| Expansion | n | mean fear_amount | CV |
|-----------|---|-----------------:|---:|
| Base | 41 | 1.05 | 1.36 |
| **B&C** | 22 | **2.00** | 1.41 |
| JE | 23 | 1.04 | 1.43 |
| **NI** | 12 | **0.83** | 1.60 |

**B&C Majors are the peak** — 2.00 mean Fear per Major card (double any other expansion). NI's 12 new Majors average 0.83 (lowest), so drafting from an all-expansions Major deck *does* pull your expected Fear-output down compared to drafting from just base+B&C. **The NI-dilutes-Majors claim is measurably true.**

### Rough per-card spread for Majors

- **B&C Majors**: highest fear + widest range of big effects. The 2.00 mean comes from power-level-5 Majors like `Vengeance of the Dead` family and `Cast Down into the Briny Deep`.
- **NI Majors**: trend toward non-Fear effects (Beast-summoning, land-transformation, presence-redistribution). Different strategic axis, lower direct Fear output.

## So what's the full verdict?

| Claim | Verdict | Evidence |
|-------|---------|----------|
| "B&C alone feels swingy" | ✅ **Confirmed** | B&C Fear CV 2.64 + Event CV 10.92 — both highest |
| "JE balances" | ✅ **Confirmed** (for Fear + Events) | JE Fear CV 1.09 (lowest); JE Events CV 2.19 (lower than B&C) |
| "NI dilutes" | ⚠️ **Partial** | NI concentrates Fear cards (mean 0.33 > JE 0.17); NI dilutes Majors (mean 0.83 < B&C 2.00) |

## Strategic implications

- **Playing B&C-only**: expect high variance. Plan for worst-case Event draws; don't build strategies that depend on a specific Event card appearing.
- **Playing B&C + JE**: JE's 30 more Events pull the combined Event-pool mean pro-spirit and reduce CV from 10.92 → somewhere around 4-5. Less swingy; tactical planning becomes more reliable.
- **Playing all expansions**: Fear drawn stays roughly constant per-game (threshold-driven), but the Major deck's mean Fear output drops. Pivot Major-shopping toward JE-era Majors (still 1.04 mean, lower variance than NI) when the draft offers cross-expansion Majors.

## Source + methodology

- Every card classified by its `set` field on the Wiki, assigned to the earliest matching expansion for marginal analysis.
- Fear-amount: sum of `N Fear` regex matches across all three stages (Fear cards) or the full text (Events/Minors/Majors).
- Net-favor: regex-based keyword count of pro-spirit (`Destroy N Explorer`, `Push N`, `N Fear`, `Remove Blight`, etc.) minus pro-invader (`add N Explorer`, `extra build`, `destroy Dahan`, etc.) matches per card.
- Scripts: [`scripts/expansion-deltas.py`](https://github.com/bfowle/spirit-island/blob/main/scripts/expansion-deltas.py) computes per-expansion moments; output at [`data/references/deck-stats/per_expansion_deltas.json`](https://github.com/bfowle/spirit-island/blob/main/data/references/deck-stats/per_expansion_deltas.json).
- Cross-reference: [Deck Expansion Impact](deck-expansion-impact.md) for the combo-pool view and Monte Carlo per-game outcomes.
