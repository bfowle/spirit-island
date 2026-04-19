## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/stone-unyielding-defiance.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in the lowest-numbered Mountain without Dahan; 1 in an adjacent land that has Blight (if possible) or is Sands (if not).
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain2earth ((spirit-specific: `gain2earth` — consult spirit panel)); Stone ((spirit-specific: `Stone` — consult spirit panel))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2)); energy3 ((+3 Energy this turn — growth effect, not track reveal))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · energy3 · card+1stone · energy4 · card+1stone · energy6 · card+1stone` — income as slots reveal: 2 → 3 → card+1stone → 4 → card+1stone → 6 → card+1stone

### Card-play track

`card1 · earthX · earthX · earthreclaimX · earthanyX · card2earth` — CP as slots reveal: 1 → earthX → earthX → earthreclaimX → earthanyX → 2

### Innate Powers

- **HOLD THE ISLAND FAST WITH A BULWARK OF WILL** (Speed: Fast · Range: ? · Target: you)
  - **L1** — 2 Earth: When Blight is added to one of your lands, you may pay 2 Energy per Blight to take it from the box instead of the Blight Card. (Handle any cascade separately.)
  - **L2** — 4 Earth: The cost is 1 Energy instead of 2.
  - **L3** — 6 Earth + 1 Plant: When an Event or Blight card directly destroys Presence (yours or others'), you may prevent any number of Presence from being destroyed by paying 1 Energy each. ("Directly" means "not by adding Blight".)
- **LET THEM BREAK THEMSELVES AGAINST THE STONE** (Speed: Fast · Range: 0 · Target: any)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Earth, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
