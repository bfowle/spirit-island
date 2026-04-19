## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/lure-of-the-deep-wilderness.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 2 in land #8, and 1 in land #7. Add 1 Beast to land #8.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `oneandone` — (see spirit panel)

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); energy1 ((track slot showing 1 Energy))
- **G2**: addpresence4inland ((spirit-specific: `addpresence4inland` — consult spirit panel))
- **G3**: moonairplant ((spirit-specific: `moonairplant` — consult spirit panel)); energy2 ((track slot showing 2 Energy))
- **G4**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · moon · energy3plant · energy4air · energy5reclaim` — income as slots reveal: 1 → 2 → moon → 3 → 4 → 5

### Card-play track

`card1 · card2 · animalX · card3 · card4 · card5reclaim1` — CP as slots reveal: 1 → 2 → animalX → 3 → 4 → 5

### Innate Powers

- **FORSAKE SOCIETY TO CHASE AFTER DREAMS** (Speed: Slow · Range: 1 · Target: invaders)
- **NEVER HEARD FROM AGAIN** (Speed: Slow · Range: 0 · Target: inland)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Fire** ×1, **Air** ×1, **Plant** ×1, **Animal** ×1

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Gift of the Untamed Wild** | 0 | Slow | No Range | Any Spirit | moon, fire, air, plant | Target Spirit chooses to either: Add 1 Wilds to one of their lands. **OR** Replace 1 of their Prese… |
| **Perils of the Deepest Island** | 1 | Slow | 0 | Inland Land | moon, plant, animal | 1 Fear. Add 1 Badlands. Add 1 Beasts within 1 Range. Push up to 2 Dahan. |
| **Softly Beckon Ever Inward** | 2 | Slow | 0 | Inland Land | moon, air | Gather up to 2 Explorers. Gather up to 2 Towns. Gather up to 2 Beasts. Gather up to 2 Dahan. |
| **Swallowed by the Wilderness** | 1 | Fast | 0 | Inland Land | fire, air, plant, animal | 2 Fear. 1 Damage per Beasts/Disease/Wilds/Badlands. (Count max. 5 tokens.) |

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
