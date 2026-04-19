## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/hearth-vigil.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the highest-numbered land with Dahan and 2 in the lowest-numbered land with at least 2 Dahan. Add 1 Dahan in each of those lands (additional survivors of the Invaders' diseases). You start with your 4 Unique Power Cards and 1 Energy.
- **Starting income** (from `presence_energy_track[0]` = `gather1dahan1land`, `presence_cardplay_track[0]` = `energy0`): **0 Energy · 0 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence0 (Place 1 Presence from a track (Range 0))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3dahan ((spirit-specific: `addpresence3dahan` — consult spirit panel))
- **G3**: addpresence2 (Place 1 Presence from a track (Range 2)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`gather1dahan1land` — income as slots reveal: gather1dahan1land

### Card-play track

`energy0 · energy1sun · energy2 · energy3animal · energy4 · energy5sun` — CP as slots reveal: energy0 → energy1sun → energy2 → energy3animal → energy4 → energy5sun

### Innate Powers

- **WARN OF IMPENDING CONFLICT** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 2 Sun + 1 Earth: In one of your lands, 1 Dahan deals Damage before Invaders during Ravages. (Choose a land when Invaders Ravage there.)
  - **L2** — 3 Sun + 1 Earth: In that land, another Dahan deals Damage before Invaders during Ravages.
  - **L3** — 4 Sun + 2 Earth: In that land, all Dahan deal Damage before Invaders during Ravages.
  - **L4** — 5 Sun + 3 Earth: Instead, all Dahan in all of your lands deal Damage before Invaders during Ravages.
- **KEEP WATCH FOR NEW INCURSIONS** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 1 Animal: Gather up to 2 Dahan, from your lands only.
  - **L2** — 1 Sun + 2 Air + 3 Animal: Once this turn after Invaders are added or moved into target land, 1 Damage per Dahan in target land, to those added/moved Invaders only.
  - **L3** — 2 Sun + 3 Air + 4 Animal: Repeat this Power.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Sun, Uniques give 0; need 1 Earth, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
