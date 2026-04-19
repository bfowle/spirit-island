## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/a-spread-of-rampant-green.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board; 1 in the highest-numbered Wetland, and 1 in the Jungle without any Dahan. (If there is more than 1 such Jungle, you may choose)
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `always` — (see spirit panel)

### Growth options

- **G1**: Spread ((spirit-specific: `Spread` — consult spirit panel))
- **G2**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); old+1cardplay ((spirit-specific: `old+1cardplay` — consult spirit panel))
- **G4**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · plant · energy2 · energy2 · plant · energy3` — income as slots reveal: 0 → 1 → plant → 2 → 2 → plant → 3

### Card-play track

`card1 · card1 · card2 · card2 · card3 · card4` — CP as slots reveal: 1 → 1 → 2 → 2 → 3 → 4

### Innate Powers

- **CREEPERS TEAR INTO MORTAR** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 Moon + 2 Plant: 1 Damage to 1 Town / City.
  - **L2** — 2 Moon + 3 Plant: Repeat this Power.
  - **L3** — 3 Moon + 4 Plant: Repeat this Power again.
- **ALL-ENVELOPING GREEN** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 1 Water + 3 Plant: Defend 2.
  - **L2** — 2 Water + 4 Plant: Instead, Defend 4.
  - **L3** — 3 Water + 1 Earth + 5 Plant: Also, remove 1 Blight.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×2, **Plant** ×2

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Fields Choked with Growth** | 0 | Slow | 1 | Any Land | sun, water, plant | Push 1 Town. **OR** Push 3 Dahan. |
| **Gift of Proliferation** | 1 | Fast | No Range | Another Spirit | moon, plant | Target Spirit adds 1 Presence up to 1 Range from their Presence. |
| **Overgrow in a Night** | 2 | Fast | 1 | Any Land | moon, plant | Add 1 Presence. **OR** If target land has your Presence and Invaders, 3 Fear. |
| **Stem the Flow of Fresh Water** | 0 | Slow | 1, from your Sacred Site | Any Land | water, plant | 1 Damage to 1 Town/City. If target land is a Mountain or Sands, instead, 1 Damage to each Town/City. |

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
