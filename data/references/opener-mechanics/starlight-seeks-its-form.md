## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/starlight-seeks-its-form.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board, in a land with Blight.
- **Starting income** (from `presence_energy_track[0]` = `empty`, `presence_cardplay_track[0]` = `empty`): **0 Energy · 0 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `three` — (see spirit panel)

### Growth options

- **G1**: reclaim1 (Reclaim 1 Power Card (of your choice))
- **G2**: addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: energy1 ((track slot showing 1 Energy))
- **G4**: movepresence3 (Move 1 Presence (Range 3))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`empty · growthreclaimhalf · growthdivider · growthgainpowercard · growthmovepresence1` — income as slots reveal: empty → growthreclaimhalf → growthdivider → growthgainpowercard → growthmovepresence1

### Card-play track

`empty · growth3energy · growthdivider · growthplusonecardplay · growthmovepresence2` — CP as slots reveal: empty → growth3energy → growthdivider → growthplusonecardplay → growthmovepresence2

### Innate Powers

- **AIR MOVES, EARTH ENDURES** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 3 Air: Push up to 2 Explorer or 1 Town.
  - **L2** — 3 Earth: Defend 5.
- **FIRE BURNS, WATER SOOTHES** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 3 Fire: 1 Fear. 2 Damage.
  - **L2** — 3 Water: Remove 1 Blight.
- **WOOD SEEKS GROWTH, HUMANS SEEK FREEDOM** (Speed: Slow · Range: 2 · Target: any)
  - **L1** — 3 Plant: Choose a Spirit with Presence in target land. They gain a Power Card.
  - **L2** — 3 Animal: 1 Damage per Dahan OR Push up to 3 Dahan.
- **SIDEREAL GUIDANCE** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Moon: Gather up to 1 Explorer/Dahan.
  - **L2** — 3 Moon: Instead, Gather up to 3 Explorer.
- **STARS BLAZE IN THE DAYTIME SKY** (Speed: Slow · Range: ? · Target: yourself)
  - **L1** — 4 Sun: 3 Fear. Gain 1 Energy. Reclaim up to 1 Power Card from play or your discard pile.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 3 Air, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
