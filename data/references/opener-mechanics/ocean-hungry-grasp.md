## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/ocean-hungry-grasp.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence onto your starting board: 1 in the Ocean, and 1 in a Coastal land of your choice.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); OceanG ((spirit-specific: `OceanG` — consult spirit panel))
- **G2**: OceanA ((spirit-specific: `OceanA` — consult spirit panel)); OceanA ((spirit-specific: `OceanA` — consult spirit panel)); energy1 ((track slot showing 1 Energy))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); Ocean ((spirit-specific: `Ocean` — consult spirit panel)); OceanP ((spirit-specific: `OceanP` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · moon · water · energy1 · earth · water · energy2` — income as slots reveal: 0 → moon → water → 1 → earth → water → 2

### Card-play track

`card1 · card2 · card2 · card3 · card4 · card5` — CP as slots reveal: 1 → 2 → 2 → 3 → 4 → 5

### Innate Powers

- **POUND SHIPS TO SPLINTERS** (Speed: Fast · Range: 0 · Target: coastal)
  - **L1** — 1 Moon + 1 Air + 2 Water: 1 Fear.
  - **L2** — 2 Moon + 1 Air + 3 Water: +1 Fear.
  - **L3** — 3 Moon + 2 Air + 4 Water: +2 Fear.
- **OCEAN BREAKS THE SHORE** (Speed: Slow · Range: 0 · Target: coastal)
  - **L1** — 2 Water + 1 Earth: Drown 1 Town.
  - **L2** — 3 Water + 2 Earth: You may instead Drown 1 City.
  - **L3** — 4 Water + 3 Earth: Also, Drown 1 Town / City.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×2, **Air** ×1, **Water** ×2
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Call of the Deeps** | 0 | Fast | 0 | Coastal Land | moon, air, water | Gather 1 Explorer. If target land is the Ocean, you may Gather another Explorer. |
| **Grasping Tide** | 1 | Fast | 1 | Coastal Land | moon, water | 2 Fear. Defend 4. |
| **Swallow the Land-Dwellers** | 0 | Slow | 0 | Coastal Land | water, earth | Drown 1 Explorer, 1 Town, and 1 Dahan. |
| **Tidal Boon** | 1 | Slow | No Range | Another Spirit | moon, water, earth | Target Spirit gains 2 Energy and may Push 1 Town and up to 2 Dahan from one of their lands. If Daha… |

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
