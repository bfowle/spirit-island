## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/fractured-days-split-the-sky.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the lowest-numbered land with 1 Dahan, and 2 in the highest-numbered land without Dahan. Deal 4 Minor and Major Powers face-up as your initial Days That Never Were cards; in a 1 or 2-player game, instead deal 6 of each. In a 1-board game, gain 1 Time.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: gainair ((spirit-specific: `gainair` — consult spirit panel)); reclaim (Reclaim all discarded+played Power Cards); gain2time ((spirit-specific: `gain2time` — consult spirit panel))
- **G2**: gainmoon ((spirit-specific: `gainmoon` — consult spirit panel)); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: gainsun ((spirit-specific: `gainsun` — consult spirit panel)); movepresence4 (Move 1 Presence (Range 4)); daysthatneverwerecard ((spirit-specific: `daysthatneverwerecard` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy1 · energy2 · energy2 · energy2 · energy3` — income as slots reveal: 1 → 1 → 2 → 2 → 2 → 3

### Card-play track

`card1 · card1 · card1 · card2 · card2 · card3` — CP as slots reveal: 1 → 1 → 1 → 2 → 2 → 3

### Innate Powers

- **SLIP THE FLOW OF TIME** (Speed: Fast · Range: ? · Target: anyspirit)
- **VISIONS OF A SHIFTING FUTURE** (Speed: Slow · Range: ? · Target: yourself)
  - **L1** — 1 Sun + 2 Moon + 2 Air: Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.)
  - **L2** — 2 Sun + 3 Moon + 2 Air: Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

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
