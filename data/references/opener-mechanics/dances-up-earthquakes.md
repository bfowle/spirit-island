## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/dances-up-earthquakes.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in the highest-numbered land with Dahan. You start with your 6 Unique Power Cards and 0 Energy. Set the Quake Tokens ({{quake}}) nearby.
- **Starting income** (from `presence_energy_track[0]` = `energy1impendenergy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence2orgainmajorwoforgetting ((spirit-specific: `addpresence2orgainmajorwoforgetting` — consult spirit panel))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G3**: addpresence3 (Place 1 Presence from a track (Range 3)); energyimpend ((spirit-specific: `energyimpend` — consult spirit panel)); reclaim1 (Reclaim 1 Power Card (of your choice))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1impendenergy1 · movepresence1energytrack · energy2 · impend1 · energy3 · impendenergy2 · energy4any` — income as slots reveal: 1 → movepresence1energytrack → 2 → impend1 → 3 → impendenergy2 → 4

### Card-play track

`card2 · gather1dahan1land · moonfire · impend1 · earthX · card3 · card4` — CP as slots reveal: 2 → gather1dahan1land → moonfire → impend1 → earthX → 3 → 4

### Innate Powers

- **LAND CREAKS WITH TENSION** (Speed: Fast · Range: ? · Target: you)
  - **L1** — 1 Earth: If you have at least 1 {{impendingcard}}, Add 1 {{quake}} in one of your lands.
  - **L2** — 1 Moon + 1 Earth: In one of your lands, Defend 1 per {{impendingcard}} (max. 3).
  - **L3** — 1 Moon + 2 Earth: If you have at least 3 {{impendingcard}}, Add 1 {{quake}} in one of your lands.
  - **L4** — 2 Moon + 3 Earth: In one of your lands, Defend 1 per {{impendingcard}} (max. 3).
- **EARTH SHUDDERS, BUILDINGS FALL** (Speed: Slow · Range: 0 · Target: quake)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Earth, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
