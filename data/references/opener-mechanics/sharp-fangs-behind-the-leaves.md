## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/sharp-fangs-behind-the-leaves.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Beast on your starting board in the highest-numbered Jungle. Put 1 Presence in a land of your choice with Beast anywhere on the island.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: Sharp1 ((spirit-specific: `Sharp1` — consult spirit panel)); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: Sharp ((spirit-specific: `Sharp` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G4**: energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · animal · plant · energy2 · animal · energy3 · energy4` — income as slots reveal: 1 → animal → plant → 2 → animal → 3 → 4

### Card-play track

`card2 · card2 · card3 · reclaim1 · card4 · card5reclaim1` — CP as slots reveal: 2 → 2 → 3 → reclaim1 → 4 → 5

### Innate Powers

- **RANGING HUNT** (Speed: Fast · Range: 1 · Target: noblight)
  - **L1** — 2 Animal: You may Gather 1 Beast.
  - **L2** — 2 Plant + 3 Animal: 1 Damage per Beast.
  - **L3** — 2 Animal: You may Push up to 2 Beast.
- **FRENZIED ASSAULT** (Speed: Slow · Range: 1 · Target: beast)
  - **L1** — 1 Moon + 1 Fire + 4 Animal: 1 Fear and 2 Damage. Remove 1 Beast.
  - **L2** — 1 Moon + 2 Fire + 5 Animal: +1 Fear and +1 Damage.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×1, **Fire** ×1, **Animal** ×1
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Animal, Uniques give 1. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Prey on the Builders** | 1 | Fast | 0 | Any Land | moon, fire, animal | You may Gather 1 Beasts. If target land has Beasts, Invaders do not Build there this turn. |
| **Teeth Gleam from Darkness** | 1 | Slow | 1, from a Jungle | Land with no Blight | moon, plant, animal | 1 Fear. Add 1 Beasts. **OR** If target land has both Beasts and Invaders: 3 Fear. |
| **Terrifying Chase** | 1 | Slow | 0 | Any Land | sun, animal | Push 2 Explorers/Towns/Dahan. Push another 2 Explorers/Towns/Dahan per Beasts in target land. If yo… |
| **Too Near the Jungle** | 0 | Slow | 1, from a Jungle | Any Land | plant, animal | 1 Fear. Destroy 1 Explorer. |

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
