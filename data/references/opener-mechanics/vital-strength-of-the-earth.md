## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/vital-strength-of-the-earth.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 2 in the highest-numbered Mountain, 1 in the highest-numbered Jungle.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence2 (Place 1 Presence from a track (Range 2))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); energy2 ((track slot showing 2 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · energy3 · energy4 · energy6 · energy7 · energy8` — income as slots reveal: 2 → 3 → 4 → 6 → 7 → 8

### Card-play track

`card1 · card1 · card2 · card2 · card3 · card4` — CP as slots reveal: 1 → 1 → 2 → 2 → 3 → 4

### Innate Powers

- **GIFT OF STRENGTH** (Speed: Fast · Range: ? · Target: anyspirit)
  - **L1** — 1 Sun + 2 Earth + 2 Plant: Once this turn, Target Spirit may Repeat 1 Power Card with Energy cost of 1 or less.
  - **L2** — 2 Sun + 3 Earth + 2 Plant: Instead, the Energy cost limit is 3 or less.
  - **L3** — 2 Sun + 4 Earth + 3 Plant: Instead, the Energy cost limit is 6 or less.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Water** ×1, **Earth** ×2, **Plant** ×1
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Plant, Uniques give 1. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **A Year of Perfect Stillness** | 3 | Fast | 1 | Any Land | sun, earth | Invaders skip all Actions in target land this turn. |
| **Draw of the Fruitful Earth** | 1 | Slow | 1 | Any Land | earth, plant, animal | Gather up to 2 [[Explorers]]. Gather up to 2 [[Dahan]]. |
| **Guard the Healing Land** | 3 | Fast | 1, from your Sacred Site | Any Land | water, earth, plant | Remove 1 Blight. Defend 4. |
| **Rituals of Destruction** | 3 | Slow | 1, from your Sacred Site | Land with Dahan | sun, moon, fire, earth, plant | 2 Damage. If target land has at least 3 Dahan, +3 Damage and 2 Fear. |

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
