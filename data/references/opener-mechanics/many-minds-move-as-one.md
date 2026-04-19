## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/many-minds-move-as-one.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Beast on your starting board, in a land with Beast. Note that you have 5 Unique Power Cards.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: addpresence1 (Place 1 Presence from a track (Range 1)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: addpresence3beast ((spirit-specific: `addpresence3beast` — consult spirit panel)); energy1 ((track slot showing 1 Energy)); gatherbeast2 ((spirit-specific: `gatherbeast2` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · air · energy2 · animal · energy3 · energy4` — income as slots reveal: 0 → 1 → air → 2 → animal → 3 → 4

### Card-play track

`card1 · card2 · pay2pcard · card3 · card3 · card4 · card5` — CP as slots reveal: 1 → 2 → pay2pcard → 3 → 3 → 4 → 5

### Innate Powers

- **THE TEEMING HOST ARRIVES** (Speed: Fast · Range: 2 · Target: any)
  - **L1** — 2 Air + 1 Animal: Gather up to 1 Beasts.
  - **L2** — 3 Air + 1 Water + 2 Animal: Instead, Gather up to 1 Beasts per Air you have.
  - **L3** — 1 Fire + 4 Air + 2 Animal: Push up to 3 Beasts.
- **BESET AND CONFOUND THE INVADERS** (Speed: Fast · Range: 2 · Target: invaders)
  - **L1** — 1 Air + 2 Animal: 2 Fear and Defend 2.
  - **L2** — 2 Air + 3 Animal: Instead, 3 Fear and Defend 4.
  - **L3** — 3 Air + 4 Animal: Instead, 4 Fear and Defend 7.
  - **L4** — 4 Air + 1 Earth + 5 Animal: Instead, 6 Fear and Defend 10.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Moon** ×1, **Air** ×3, **Water** ×2, **Animal** ×3
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **A Dreadful Tide of Scurrying Flesh** | 0 | Fast | Range 1, from your Sacred Site | Land with 2 or more Beasts tokens | moon, air, water, animal | Remove up to half (round down) of Beasts in target land. For each Beasts Removed, 2 Fear and skip o… |
| **Boon of Swarming Bedevilment** | 0 | Fast | No Range | Another Spirit | air, water, animal | For the rest of this turn, each of target Spirit's Presence grants Defend 1 in its land. Target Spi… |
| **Ever-Multiplying Swarm** | 1 | Slow | 0 | Any Land | fire, earth, animal | Add 2 Beasts. |
| **Guide the Way on Feathered Wings** | 0 | Fast | 1 | Any Land | sun, air, animal | Move 1 Beasts up to two lands. As it moves, up to 2 Dahan may move with it, for part or all of the … |

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
