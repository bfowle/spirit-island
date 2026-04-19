## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/grinning-trickster.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in the highest-numbered land with Dahan, and 1 in land #4.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: Sharp1 ((spirit-specific: `Sharp1` — consult spirit panel)); movepresence1 (Move 1 Presence (Range 1))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G4**: energycardplays ((spirit-specific: `energycardplays` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · moon · energy2 · any · fire · energy3` — income as slots reveal: 1 → moon → 2 → any → fire → 3

### Card-play track

`card2 · pushdahan · card3 · card3 · card4 · airX · card5` — CP as slots reveal: 2 → pushdahan → 3 → 3 → 4 → airX → 5

### Innate Powers

- **LET'S SEE WHAT HAPPENS** (Speed: Fast · Range: 1 · Target: invaders)
  - **L1** — 1 Moon + 1 Fire + 2 Air: Discard Minor Powers from the deck until you get one that targets a land. Use its text effects on target land immediately, ignoring normal Range/Targeting restrictions. All "up to" instructions must be used at max. value. Treat all "OR"s as "AND"s. (It is not considered a card of yours or a card in play. Its effects are treated as performed by this Power, as if its text were copied here.)
  - **L2** — 2 Moon + 1 Fire + 2 Air: You may Forget a Power Card to gain the just-used Power Card (to hand) and 1 Energy.
- **WHY DON'T YOU AND THEM FIGHT** (Speed: Fast · Range: 0 · Target: invaders)
  - **L1** — 3 Moon: This Power may be Slow.
  - **L2** — 3 Air: Add 1 Strife.
  - **L3** — 3 Sun + 3 Fire: 1 Invader and 1 Dahan deal Damage to each other.
  - **L4** — 3 Animal: If target land has Beast, 2 Damage. Otherwise, you may Gather 1 Beast.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Fire** ×1, **Air** ×1
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Moon, Uniques give 0; need 2 Air, Uniques give 1. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Impersonate Authority** | 0 | Slow | 1 | Any Land | sun, air, animal | Add 1 Strife. |
| **Incite the Mob** | 1 | Slow | 1 | Land with 1 or more Invaders | moon, fire, air, animal | 1 Invader with Strife deals Damage to other Invaders (not to each Invader). 1 Fear per Invader this… |
| **Overenthusiastic Arson** | 1 | Fast | 1 | Any Land | fire, air | Destroy 1 Town. Discard the top card of the Minor Power Deck. If it provides Fire: 1 Fear, 2 Damage… |
| **Unexpected Tigers** | 0 | Slow | 1 | Any Land | moon, fire, animal | 1 Fear if Invaders are present. If you can gather 1 Beasts, do so, then push 1 Explorer. Otherwise,… |

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
