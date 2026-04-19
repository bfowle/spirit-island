## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/breath-of-darkness.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence and your Incarna ({{incarna|breath}}), Unempowered ({{incarna|unempowered}}), on your starting board: 1 Presence and {{incarna|breath}} in the lowest-numbered Jungle and 1 in the highest-numbered Jungle. Set [[The Endless Dark]] ({{endlessdark}}) tile next to the island with 1 Explorer on it. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); movebreathincarna ((spirit-specific: `movebreathincarna` — consult spirit panel))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); darknessescape2 ((spirit-specific: `darknessescape2` — consult spirit panel))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); addmovebreathincarna ((spirit-specific: `addmovebreathincarna` — consult spirit panel)); darknessescape1 ((spirit-specific: `darknessescape1` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · moon · energy3 · empowerincarna · energy4animal · energy5air` — income as slots reveal: 1 → 2 → moon → 3 → empowerincarna → 4 → 5

### Card-play track

`card2 · movepres · card3 · moonX · reclaim1 · card4air` — CP as slots reveal: 2 → movepres → 3 → moonX → reclaim1 → 4

### Innate Powers

- **LEAVE A TRAIL OF DEATHLY SILENCE** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 2 Moon + 1 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L2** — 3 Moon + 1 Air + 1 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L3** — 4 Moon + 2 Air + 2 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L4** — 5 Moon + 2 Air + 3 Animal: Move {{incarna|breath}} to {{endlessdark}}. It Brings 1 Invader (from its land).
- **LOST IN THE ENDLESS DARK** (Speed: Slow · Range: ? · Target: endlessdark)
  - **L1** — 2 Moon + 1 Air: 1 Fear per Invader (max. 4). Downgrade up to 1 Invader. (Downgrading Removes Explorer.)
  - **L2** — 4 Moon + 3 Air: 1 Fear per Invader (max. 4). Downgrade any number of Invaders.
  - **L3** — 3 Moon + 2 Animal: Add 1 Beast.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Moon, Uniques give 0; need 1 Animal, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
