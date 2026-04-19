## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/bringer-of-dreams-and-nightmares.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board in the highest-numbered Sands.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card2`): **2 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: reclaim1 (Reclaim 1 Power Card (of your choice)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G4**: Night ((spirit-specific: `Night` — consult spirit panel)); energy2 ((track slot showing 2 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · air · energy3 · moon · energy4 · any · energy5` — income as slots reveal: 2 → air → 3 → moon → 4 → any → 5

### Card-play track

`card2 · card2 · card2 · card3 · card3 · any` — CP as slots reveal: 2 → 2 → 2 → 3 → 3 → any

### Innate Powers

- **SPIRITS MAY YET DREAM** (Speed: Fast · Range: ? · Target: anyspirit)
  - **L1** — 2 Moon + 2 Air: Turn any face down Fear Card face-up. (It's earned/resolved normally, but players can see what's coming)
  - **L2** — 3 Moon: Target Spirit gains an element that they have at least 1 of.
- **NIGHT TERRORS** (Speed: Fast · Range: 0 · Target: invaders)
  - **L1** — 1 Moon + 1 Air: 1 Fear.
  - **L2** — 2 Moon + 1 Air + 1 Animal: +1 Fear.
  - **L3** — 3 Moon + 2 Air + 1 Animal: +1 Fear.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×3, **Air** ×2, **Animal** ×1
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Call on Midnight's Dream** | 0 | Fast | 0 | Any Land | moon, animal | If target land has Dahan, gain a Major Power. If you Forget this Power, gain Energy equal to Dahan … |
| **Dread Apparitions** | 2 | Fast | 1 | Land with 1 or more Invaders | moon, air | When Powers generate Fear in target land, Defend 1 per Fear. 1 Fear. (Fear from To Dream a Thousand… |
| **Dreams of the Dahan** | 0 | Fast | 2 | Any Land | moon, air | Gather up to 2 [[Dahan]]. **OR** If target land has [[Towns]]/[[Cities]], 1 [[Fear]] for each [[Dah… |
| **Predatory Nightmares** | 2 | Slow | 1, from your Sacred Site | Land with 1 or more Invaders | moon, fire, earth, animal | 2 Damage. Push up to 2 Dahan. (When your Powers would Destroy Invaders, instead they generate Fear … |

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
