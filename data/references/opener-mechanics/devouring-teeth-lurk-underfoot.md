## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/devouring-teeth-lurk-underfoot.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board, in land #5. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence0 (Place 1 Presence from a track (Range 0))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G3**: addpresence2 (Place 1 Presence from a track (Range 2)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · fire · energy3 · energy4 · animal · energy6 · energy7` — income as slots reveal: 2 → fire → 3 → 4 → animal → 6 → 7

### Card-play track

`card1 · card2 · animalX · fireX · card3 · earthX · card4` — CP as slots reveal: 1 → 2 → animalX → fireX → 3 → earthX → 4

### Innate Powers

- **DEATH APPROACHES FROM BENEATH THE SURFACE** (Speed: Slow · Range: 1 · Target: invaders)
  - **L1** — 1 Fire + 1 Animal: If you don't have Presence in target land, Gather 1 of your Presence. (This is required.)
  - **L2** — 2 Fire + 1 Earth + 2 Animal: 1 Damage. (+1 for your "Territorial Aggression" Special Rule)
  - **L3** — 3 Fire + 1 Earth + 3 Animal: 2 Damage.
  - **L4** — 4 Fire + 2 Earth + 5 Animal: 2 Fear. 4 Damage.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_

### Unique Power Cards

*No Unique cards parsed.*

### Invader phase by turn (base deck)

| Turn | Explore | Build | Ravage | Notes |
|------|---------|-------|--------|-------|
| 1 | ✓ | — | — | Ravage-protection effects are **dormant T1**. |
| 2 | ✓ | ✓ | — | First Build; Ravage-protection still dormant. |
| 3 | ✓ | ✓ | ✓ | First Ravage; Ravage-protection becomes material. |
| 4+ | ✓ | ✓ | ✓ | Full cycle continues. |

Adversary escalation can shift this — check the adversary JSON for deviations (Sweden front-loads a Build; some Habsburg levels add early Builds).

### Pause-point before writing T1 prose

```admonish warning title="Before claiming what T1 does"
1. **Compute post-growth E/CP** for every growth × track-choice branch. Don't assume both tracks reveal simultaneously.
2. **Enumerate legal T1 plays** — subsets of hand with sum(costs) ≤ E and count ≤ CP.
3. **Separate Fast vs. Slow elements** — when claiming an innate fires, verify the threshold is met using only elements from its resolution phase (Fast sees Fast; Slow sees Fast + Slow).
4. **Flag dormant effects** — Ravage-protection, Defend N, etc. are **null T1/T2** in base play. Only cite them as opener value when the trigger actually occurs that turn.
5. **State per-turn material effect** for every card play: Fear generated, units pushed/gathered/destroyed, elements contributed. Never narrate dormant effects as if they were active.
```
