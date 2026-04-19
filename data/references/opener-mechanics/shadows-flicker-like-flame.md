## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/shadows-flicker-like-flame.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 2 in the highest-numbered Jungle and 1 in land #5
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G3**: addpresence3 (Place 1 Presence from a track (Range 3)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · energy3 · energy4 · energy5 · energy6` — income as slots reveal: 0 → 1 → 3 → 4 → 5 → 6

### Card-play track

`card1 · card2 · card3 · card3 · card4 · card5` — CP as slots reveal: 1 → 2 → 3 → 3 → 4 → 5

### Innate Powers

- **DARKNESS SWALLOWS THE UNWARY** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 2 Moon + 1 Fire: Gather 1 Explorer.
  - **L2** — 3 Moon + 2 Fire: Destroy up to 2 Explorer. 1 Fear per Explorer destroyed.
  - **L3** — 4 Moon + 3 Fire + 2 Air: 3 Damage. 1 Fear per Invaders destroyed by this Damage.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×1, **Air** ×1
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Moon, Uniques give 1; need 1 Fire, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Concealing Shadows** | 0 | Fast | 0 | Any Land | moon, air | 1 Fear. Dahan take no Damage from Ravaging Invaders this turn. |
| **Crops Wither and Fade** | 1 | Slow | 0 | Any Land | moon, fire, plant | 2 Fear. Replace 1 Town with 1 Explorer. **OR** Replace 1 City with 1 Town. |
| **Favors Called Due** | 1 | Slow | 1 | Any Land | moon, air, animal | Gather up to 4 Dahan. If Invaders are present and Dahan now outnumber them, 3 Fear. |
| **Mantle of Dread** | 1 | Slow | No Range | Any Spirit | moon, fire, air | 2 Fear. Target Spirit may Push 1 Explorer and 1 Town from a land where it has Presence. |

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
