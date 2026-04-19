## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/serpent-slumbering-beneath-the-island.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in the land #5.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); movepresence1 (Move 1 Presence (Range 1))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G3**: energy4 (+4 Energy this turn (growth effect))
- **G4**: noblight ((spirit-specific: `noblight` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · fire · any · reclaim1E · earth · energy6 · any · energy12` — income as slots reveal: 1 → fire → any → reclaim1E → earth → 6 → any → 12

### Card-play track

`card1 · moonX · card2 · waterX · serpent · card4 · card5reclaim1` — CP as slots reveal: 1 → moonX → 2 → waterX → serpent → 4 → 5

### Innate Powers

- **SERPENT WAKES IN POWER** (Speed: Slow · Range: ? · Target: you)
  - **L1** — 2 Fire + 1 Water + 1 Plant: Gain 1 Energy. Other spirits with any Absorbed Presence also gain 1 Energy.
  - **L2** — 2 Water + 3 Earth + 2 Plant: Add 1 Presence to Range 1. Other spirits with 2 or more Absorbed Presence may do likewise.
  - **L3** — 3 Fire + 3 Water + 3 Earth + 3 Plant: Gain a Major Power without Forgetting. Other Spirits with 3 or more Absorbed Presence may do likewise.
- **SERPENT ROUSES IN ANGER** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 Fire + 1 Earth: For each Fire Earth you have, 1 Damage to 1 Town / City.
  - **L2** — 2 Moon + 2 Earth: For each 2 Moon 2 Earth you have, 2 Fear and you may Push 1 Town from target land.
  - **L3** — 5 Moon + 6 Fire + 6 Earth: Cost 7 In every land in the game: X Damage, where X is the number of Presence you have in and adjacent to that land.

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
