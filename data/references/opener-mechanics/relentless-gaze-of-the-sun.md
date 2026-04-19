## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/relentless-gaze-of-the-sun.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence and 1 Badlands on your starting board, in the lowest-numbered Sands. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `always` — (see spirit panel)

### Growth options

- **G1**: Gaze ((spirit-specific: `Gaze` — consult spirit panel))
- **G2**: reclaim (Reclaim all discarded+played Power Cards); add3destroyedpresencetogether ((spirit-specific: `add3destroyedpresencetogether` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G4**: gaindoubleenergy ((spirit-specific: `gaindoubleenergy` — consult spirit panel)); move3presencetogether ((spirit-specific: `move3presencetogether` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2sun · energy3fire · sun · energy4any · energy5 · ` — income as slots reveal: 1 → 2 → 3 → sun → 4 → 5 → 

### Card-play track

`card1 · card1 · card2 · sunX · card3 · reclaim1 · card4` — CP as slots reveal: 1 → 1 → 2 → sunX → 3 → reclaim1 → 4

### Innate Powers

- **SCORCHING CONVERGENCE** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Sun: Move all of your Presence from origin land directly to target land. 1 Damage, to Town/City only.
  - **L2** — 3 Sun + 1 Fire: 3 Damage to Invaders. 3 Damage to Dahan. Add 1 Blight without cascading.
  - **L3** — 4 Sun + 2 Fire + 1 Air: 3 Fear if this Power destroyed any Invaders.
  - **L4** — 5 Sun + 3 Fire + 2 Air: 1 Damage per remaining Presence of yours in target land.
- **CONSIDER A HARMONIOUS NATURE** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 3 Sun + 1 Moon: When your Powers would Add Blight, you may Destroy 1 Presence instead (there or elsewhere).
  - **L2** — 3 Sun + 1 Water: Your Powers don't damage or destroy Dahan.
  - **L3** — 3 Sun + 1 Plant: Choose another Spirit. They Add 1 {{destroyedpresence}} to one of your lands.
  - **L4** — 3 Sun + 1 Water + 1 Plant: Give up to 3 of your Energy to the chosen Spirit.

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
