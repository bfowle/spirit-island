## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/heart-of-the-wildfire.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence and 2 Blight on your starting board in the highest-numbered Sands. (Blight comes from the box, not the Blight Card)
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); energy2 ((track slot showing 2 Energy)); Wildfire ((spirit-specific: `Wildfire` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · fire · energy1 · energy2 · fireplant · energy3` — income as slots reveal: 0 → fire → 1 → 2 → fireplant → 3

### Card-play track

`card1 · fireX · card2 · card3 · fireX · card4` — CP as slots reveal: 1 → fireX → 2 → 3 → fireX → 4

### Innate Powers

- **FIRESTORM** (Speed: Fast · Range: 0 · Target: blight)
  - **L1** — 1 Plant: 1 Damage per 2 Fire you have.
  - **L2** — 3 Plant: Instead, 1 Damage per Fire you have.
  - **L3** — 4 Fire + 2 Air: Split this Power's Damage however desired between target land and any number of your lands with Blight.
  - **L4** — 7 Fire: In a land with Blight where you have Presence, Push all Dahan. Destroy all Invaders and Beast. Add 1 Blight.
- **THE BURNED LAND REGROWS** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 4 Fire + 1 Plant: If target land has 2 Blight or more, remove 1 Blight
  - **L2** — 4 Fire + 2 Plant: Instead, remove 1 Blight.
  - **L3** — 5 Fire + 2 Earth + 2 Plant: Remove another Blight.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Fire** ×2, **Plant** ×2
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Asphyxiating Smoke** | 2 | Slow | 2, from your Sacred Site | Any Land | fire, air, plant | 1 Fear. Destroy 1 Town. Push 1 Dahan. |
| **Flame's Fury** | 0 | Fast | No Range | Any Spirit | sun, fire, plant | Target Spirit gains 1 Energy. Target Spirit does +1 Damage with each Damage dealing Power they use … |
| **Flash-Fires** | 2 | Slow | 1 | Any Land | fire, air | 1 Fear. 1 Damage. |
| **Threatening Flames** | 0 | Fast | 0 | Land with 1 or more Blight and 1 or more Invaders | fire, plant | 2 Fear. Push 1 Explorer/Town per Terror Level from target land to adjacent lands without your Prese… |

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
