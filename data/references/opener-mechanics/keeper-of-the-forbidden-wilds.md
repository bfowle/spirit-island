## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/keeper-of-the-forbidden-wilds.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Wilds on your starting board in the highest-numbered Jungle.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G3**: Keeper ((spirit-specific: `Keeper` — consult spirit panel)); energy1 ((track slot showing 1 Energy))
- **G4**: Keeper3 ((spirit-specific: `Keeper3` — consult spirit panel)); noblight ((spirit-specific: `noblight` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · sun · energy4 · energy5 · plant · energy7 · energy8 · energy9` — income as slots reveal: 2 → sun → 4 → 5 → plant → 7 → 8 → 9

### Card-play track

`card1 · card2 · card2 · card3 · card4 · card5reclaim1` — CP as slots reveal: 1 → 2 → 2 → 3 → 4 → 5

### Innate Powers

- **PUNISH THOSE WHO TRESPASS** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 2 Sun + 1 Fire + 2 Plant: 2 Damage. Destroy 1 Dahan.
  - **L2** — 2 Sun + 2 Fire + 3 Plant: +1 Damage per SunPlant you have.
  - **L3** — 4 Plant: Split this Power's Damage however desired between target land and another 1 of your lands.
- **SPREADING WILDS** (Speed: Slow · Range: 1 · Target: noblight)
  - **L1** — 2 Sun: Push 1 Explorer from target land per 2 Sun you have.
  - **L2** — 1 Plant: If target land has no Explorer, add 1 Wilds.
  - **L3** — 3 Plant: This Power has Range +1.
  - **L4** — 1 Air: This Power has Range +1.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Earth** ×1, **Plant** ×1

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Boon of Growing Power** | 1 | Slow | No Range | Any Spirit | sun, moon, plant | Target Spirit gains a Power Card. If you target another Spirit, they also gain 1 Energy. |
| **Regrow from Roots** | 1 | Slow | 1 | Jungle or Wetland | water, earth, plant | If there are 2 Blight or fewer in target land, Remove 1 Blight. |
| **Sacrosanct Wilderness** | 2 | Fast | 1 | Land with no Blight | sun, earth, plant | Push 2 Dahan. 2 Damage per Wilds in target land. **OR** Add 1 Wilds. |
| **Towering Wrath** | 3 | Slow | 1, from your Sacred Site | Any Land | sun, fire, plant | 2 Fear. For each of your Sacred Site in/adjacent to target land, 2 Damage. Destroy all Dahan. |

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
