## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/finder-of-paths-unseen.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in land #3. Put 1 Presence on any board in land #1. Note that you have 6 Unique Power Cards.
- **Starting income** (from `presence_energy_track[0]` = `—`, `presence_cardplay_track[0]` = `blank`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); ignorerange ((spirit-specific: `ignorerange` — consult spirit panel))
- **G2**: addpresence1 (Place 1 Presence from a track (Range 1)); new+1cardplay ((spirit-specific: `new+1cardplay` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G4**: addpresence ((spirit-specific: `addpresence` — consult spirit panel)); energy2 ((track slot showing 2 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

(not parsed)

### Card-play track

`blank · blank · energy1moon · blank · movepresair · blank · energy+1range+1 · energy+1range+1text` — CP as slots reveal: blank → blank → energy1moon → blank → movepresair → blank → energy+1range+1 → energy+1range+1text

### Innate Powers

- **LAY PATHS THEY CANNOT HELP BUT WALK** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — 2 Moon + 2 Air: Push up to half (rounded down) of Invaders from target land. Do likewise for Dahan, Presence, and Beast (each separately).
  - **L2** — 2 Sun + 2 Air: Push up to 1 Invader/Dahan/Presence/Beast.
  - **L3** — 2 Moon + 4 Air + 3 Water: Repeat this Power.
- **CLOSE THE WAYS** (Speed: Fast · Range: 1 · Target: any)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Moon, Uniques give 0; need 2 Air, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
