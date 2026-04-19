## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/downpour-drenches-the-world.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in the lowest-numbered Wetlands.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); movepresence2 (Move 1 Presence (Range 2))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2)); addpresence2 (Place 1 Presence from a track (Range 2)); gain2water ((spirit-specific: `gain2water` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); energy1 ((track slot showing 1 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · water · plant · water · energy2air · water · earth · doublewater` — income as slots reveal: 1 → water → plant → water → 2 → water → earth → doublewater

### Card-play track

`card1 · movepres · waterX · card2 · movepres · card3` — CP as slots reveal: 1 → movepres → waterX → 2 → movepres → 3

### Innate Powers

- **RAIN AND MUD SUPPRESS CONFLICT** (Speed: Fast · Range: ? · Target: you)
  - **L1** — 1 Air + 3 Water: Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.)
  - **L2** — 5 Water + 1 Earth: Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1.
  - **L3** — 3 Air + 9 Water + 2 Earth: 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1).
- **WATER NOURISHES LIFE'S GROWTH** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — 3 Water + 2 Plant: Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (From target land).
  - **L2** — 5 Water + 1 Earth + 2 Plant: Gain +1 Energy. Gather up to 1 Dahan.
  - **L3** — 7 Water + 2 Earth + 3 Plant: When Blight would be added to target land, instead leave it on the card.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Air, Uniques give 0; need 3 Water, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
