# Starlight Seeks Its Form

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Very High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "three" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 1 · Fear 1 · Defense 2 · Utility 2             |
| Primary Elements      | Moon, Air, Earth, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> A build-your-own-Spirit, capable of going in many different directions based on Elements picked, Growth choices selected, and Power Cards kept. Has a very high personal/visual complexity and a huge number of early-game options, but doesn't alter play much for other players at the table. As it commits to choices, it loses versatility - not all paths will be good (or even possible) at all things. It especially wants a measure of adaptation to early Power Cards, rather than trying to pre-select a strategy.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board, in a land with Blight.

## Growth Options (three)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim1 |
| G2 | first=addpresence0 |
| G3 | first=energy1 |
| G4 | first=movepresence3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: empty, growthreclaimhalf, growthdivider, growthgainpowercard, growthmovepresence1
- **Card-play track**: empty, growth3energy, growthdivider, growthplusonecardplay, growthmovepresence2

## Core Mechanics & Special Rules

### Special Rule

GROWTH BEGETS GROWTH You have 6 Presence tracks. (As usual, you may add Presence from any track.) 4 of the Presence tracks are next to rows of Growth choices: these choices start unavailable. Upon emptying a Growth track, pick one of its two Growth choices to be immediately available. The other stays unavailable for the rest of the game (cover with a spare piece). After you add Presence from a space marked {{energyplus1}}, gain 1 Energy. SLOWLY COALESCING NATURE After revealing an {{gainelement}}, place 1 Element Marker of your choice on it. That element is permanent and is constantly available (As if pre-printed on the Presence track.)

### Innate: AIR MOVES, EARTH ENDURES

- **Speed**: fast · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Air | Push up to 2 Explorer or 1 Town. |
| 2 | 3 Earth | Defend 5. |


### Innate: FIRE BURNS, WATER SOOTHES

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Fire | 1 Fear. 2 Damage. |
| 2 | 3 Water | Remove 1 Blight. |


### Innate: WOOD SEEKS GROWTH, HUMANS SEEK FREEDOM

- **Speed**: slow · **Range**: 2 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Plant | Choose a Spirit with Presence in target land. They gain a Power Card. |
| 2 | 3 Animal | 1 Damage per Dahan OR Push up to 3 Dahan. |


### Innate: SIDEREAL GUIDANCE

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon | Gather up to 1 Explorer/Dahan. |
| 2 | 3 Moon | Instead, Gather up to 3 Explorer. |


### Innate: STARS BLAZE IN THE DAYTIME SKY

- **Speed**: slow · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 Sun | 3 Fear. Gain 1 Energy. Reclaim up to 1 Power Card from play or your discard pile. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Starlight Seeks Its Form's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/starlight-seeks-its-form.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/starlight-seeks-its-form.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Moon** (wt 3.6), **Sun** (wt 3.6), **Air** (wt 2.7)
- **Mid-game energy estimate (T3–T5 avg)**: 0.0E
- **Power summary**: Offense 1 · Control 1 · Fear 1 · Defense 2 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+fire+moon+plant → 11.7 |
| 2 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements air+moon+sun+water → 11.7 |
| 3 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements fire+moon+plant+water → 10.8 |
| 4 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire+plant+sun → 11.7 |
| 5 | **Sunset's Fire Flows Across the Land** | 1 | Slow | Sun, Moon, Fire, Water | 1 Fear. 1 Damage. You may pay 1 Energy to deal 1 Damage in an adjacent land. | elements fire+moon+sun+water → 11.7 |
| 6 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+earth+fire+plant → 9.9 |
| 7 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear. The next time an Invader is Destroyed in target land this turn, 1 Fear. | elements air+moon+sun → 9.9 |
| 8 | **Guardian Serpents** | 1 | Fast | Sun, Moon, Earth, Animal | Add 1 Beasts in one of target Spirit's lands. If target Spirit has a Sacred Site in that … | elements animal+earth+moon+sun → 10.8 |
| 9 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon+plant+water → 9.9 |
| 10 | **Inflame the Fires of Life** | 1 | Slow | Moon, Fire, Plant, Animal | Add 1 Disease. **OR** 1 Fear. Add 1 Strife. | elements animal+fire+moon+plant → 10.8 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+earth+fire+moon+plant+sun+water → 20.7 |
| 2 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+earth+moon+sun+water → 13.5 |
| 3 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+earth+moon+plant+sun → 14.4 |
| 4 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements animal+earth+fire+moon+plant → 12.6 |
| 5 | **Unleash a Torrent of the Self's Own Essence** | 2 | Fast | Sun, Moon, Fire, Water | Gain 4 Energy. You may forget a Power card to gain 4 more Energy. **OR** Pay X Energy (mi… | elements fire+moon+sun+water → 11.7 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Starlight Seeks Its Form in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Land of Haunts and Embers** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Scour the Land** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

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

## Adversary Matchup Matrix

`[VERIFY all grades]` — template only; fill in per-adversary notes after play.

| Adversary            | L0 | L3 | L5 | L6 | Notes `[VERIFY]`     |
|----------------------|----|----|----|----|----------------------|
| England              | ?  | ?  | ?  | ?  |                      |
| Brandenburg-Prussia  | ?  | ?  | ?  | ?  |                      |
| Sweden               | ?  | ?  | ?  | ?  |                      |
| France (Plantation)  | ?  | ?  | ?  | ?  |                      |
| Habsburg Mining      | ?  | ?  | ?  | ?  |                      |
| Russia               | ?  | ?  | ?  | ?  |                      |
| Scotland             | ?  | ?  | ?  | ?  |                      |
| Habsburg Livestock   | ?  | ?  | ?  | ?  |                      |

## Board / Map Configuration

`[VERIFY via play]` — base boards A–D, Jagged Earth E–H, and thematic ratings pending per-spirit play experience.

## Game-Phase Strategy

`[VERIFY: needs play data]`.

## Synergy Partners (Multiplayer)

`[VERIFY: needs multi-spirit play data]` — archetype-based hints from [Archetype Index](../../combos/archetype-index.md) are the starting point.

## Common Mistakes

`[VERIFY: collect from play]`.

## Tempo Profile

`[VERIFY: per-round targets need playtest]`.

## Expansion Sensitivity

- **Base only**: core Uniques + Innate + Special Rule functional if expansion = Base.
- **+ Branch & Claw**: events + blight deck introduce variance.
- **+ Jagged Earth**: Major/Minor pool deepens.
- **+ Nature Incarnate**: additional aspects may unlock; check the aspect column above.

Per-expansion specifics `[VERIFY]`.

## Stat Snapshot

```admonish note title="Stat Insight"
`[VERIFY from mindwanderer]` — pending re-scrape of mindwanderer current data. Historical directional figures unavailable in this template draft.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/starlight-seeks-its-form.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Starlight Seeks Its Form](https://spiritislandwiki.com/index.php?title=Starlight_Seeks_Its_Form).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
