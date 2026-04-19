## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/towering-roots-of-the-jungle.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the highest-numbered Jungle without Blight, 1 in the highest-numbered Mountain, and 1 in the highest-numbered Wetland. Put {{incarna|roots}}, Unempowered ({{incarna|unempowered}}) side up, in the Jungle with your Presence. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence0 (Place 1 Presence from a track (Range 0))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1)); addvitalityrootsincarna ((spirit-specific: `addvitalityrootsincarna` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); replacepresencerootsincarna ((spirit-specific: `replacepresencerootsincarna` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · earth · energy4 · plant · energy6` — income as slots reveal: 1 → 2 → earth → 4 → plant → 6

### Card-play track

`card1 · card2 · sunX · card3 · plantX · card4` — CP as slots reveal: 1 → 2 → sunX → 3 → plantX → 4

### Innate Powers

- **SHELTER UNDER TOWERING BRANCHES** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 Sun + 1 Plant: Gather up to 1 Dahan.
  - **L2** — 1 Sun + 1 Earth + 2 Plant: Gather up to 1 Explorer.
  - **L3** — 2 Sun + 1 Earth + 3 Plant: Gather up to 1 Town.
  - **L4** — 3 Sun + 2 Earth + 4 Plant: Gather up to 1 City.
- **REVOKE SANCTUARY AND CAST OUT** (Speed: Slow · Range: 0 · Target: rootsincarnainvaders)
  - **L1** — 1 Sun + 1 Moon + 2 Plant: 1 Fear. Remove 1 Explorer/Town.
  - **L2** — 2 Sun + 1 Moon + 3 Plant: 1 Fear. Remove 1 Explorer/Town.
  - **L3** — 2 Sun + 2 Moon + 4 Plant: 1 Fear. Remove 1 Invader.

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
