## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/wounded-waters-bleeding.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: On your starting board, put 2 Presence in a land with Blight, then put 2 Presence and 1 Blight (from the box) in the highest-numbered land with a Town Setup Symbol. You start with your 4 Unique Power Cards and 4 Energy.</br>Set your 4 Healing Cards nearby.
- **Starting income** (from `presence_energy_track[0]` = `blank`, `presence_cardplay_track[0]` = `energy0card1`): **3 Energy · 0 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: addpresence3 (Place 1 Presence from a track (Range 3)); energy3 ((+3 Energy this turn — growth effect, not track reveal)); adddestroyedpresence1 ((spirit-specific: `adddestroyedpresence1` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`blank · blank · blank · blank · energy3 · energy4fireorplant · energy5any` — income as slots reveal: blank → blank → blank → blank → 3 → 4 → 5

### Card-play track

`energy0card1 · wateroranimal · gatherblight · energy1card2` — CP as slots reveal: energy0card1 → wateroranimal → gatherblight → energy1card2

### Innate Powers

- **SWIRL AND SPILL** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Water: Push up to 2 Explorer/Dahan/Blight.
  - **L2** — 3 Water + 1 Animal: 1 Fear. Push up to 2 Town/Presence/Beasts.
  - **L3** — 5 Water + 2 Plant + 2 Animal: In one land pushed into, Downgrade all Town and all City.
- **SANGUINARY TAINT** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Animal: 1 Fear. 1 Damage. Push 1 Dahan.
  - **L2** — 1 Water + 3 Animal: 1 Damage. Add 1 Beasts.
  - **L3** — 2 Fire + 2 Water + 5 Animal: 1 Fear. 4 Damage. Add 1 Disease.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_

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
