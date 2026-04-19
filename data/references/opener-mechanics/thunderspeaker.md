## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/thunderspeaker.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in each of the 2 lands with the most Dahan.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: Thunder2 ((spirit-specific: `Thunder2` — consult spirit panel)); Thunder1 ((spirit-specific: `Thunder1` — consult spirit panel))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); energy4 (+4 Energy this turn (growth effect))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · air · energy2 · fire · sun · energy3` — income as slots reveal: 1 → air → 2 → fire → sun → 3

### Card-play track

`card1 · card2 · card2 · card3 · reclaim1 · card3 · card4` — CP as slots reveal: 1 → 2 → 2 → 3 → reclaim1 → 3 → 4

### Innate Powers

- **GATHER THE WARRIORS** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 4 Air: This Power may be Fast.
  - **L2** — 1 Animal: Gather up to 1 Dahan per Air you have. Push up to 1 Dahan per Sun you have.
- **LEAD THE FURIOUS ASSAULT** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 4 Air: This Power may be Fast.
  - **L2** — 2 Sun + 1 Fire: Destroy 1 Town for every 2 Dahan in target land.
  - **L3** — 4 Sun + 3 Fire: Destroy 1 City for every 3 Dahan in target land.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Fire** ×1, **Air** ×2, **Animal** ×2

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Manifestation of Power and Glory** | 3 | Slow | 0 | Land with Dahan | sun, fire, air | 1 Fear. Each Dahan deals Damage equal to the number of your Presence in target land. |
| **Sudden Ambush** | 2 | Fast | 1 | Any Land | fire, air, animal | You may Gather 1 Dahan. Each Dahan Destroys 1 Explorer. |
| **Voice of Thunder** | 0 | Slow | 1 | Any Land | sun, air | Push up to 4 Dahan. **OR** If Invaders are present, 2 Fear. |
| **Words of Warning** | 1 | Fast | 1 | Land with Dahan | sun, air, animal | Defend 3. During Ravage, Dahan in target land deal Damage simultaneously with Invaders. |

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
