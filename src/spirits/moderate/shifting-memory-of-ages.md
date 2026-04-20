# Shifting Memory of Ages

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 2 · Fear 2 · Defense 4 · Utility 5             |
| Primary Elements      | Earth, Air, Moon (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Starts with little ability to influence the board - most of what it does in that regard will come from new Power Cards. Extremely good with Major Powers and usually wants to take them early and often. Can either try sprinting towards victory with its phenomenal Energy Growth or build up towards becoming a late-game powerhouse.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board in the highest-numbered land that is Sands or Mountain. Prepare 1 Simplemoon, 1 Simpleair, and 1 Simpleearth marker (put them by your Special Rules).

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence2 |
| G3 | first=addpresence1, second=energy2 |
| G4 | first=energy9 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, energy2, energy3marker, energy4, reclaim1E, energy5, energy6marker
- **Card-play track**: card1, card2, card2, markersforplay, card3

## Core Mechanics & Special Rules

### Special Rule

LONG AGES OF KNOWLEDGE AND FORGETFULNESS When you would Forget a Power Card from your hand, you may instead discard it. (Max. once per Action.) INSIGHTS INTO THE WORLD'S NATURE Some of your Actions let you Prepare Element Markers, which are kept here until used. Choose the Elements freely. (I.e., you are not limited to Elements you have at the time.) Each Element Marker spent grants 1 of that Element for a single Action. (E.g., one Power use.)

### Innate: LEARN THE INVADERS' TACTICS

- **Speed**: fast · **Range**: 1 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Defend 2. |
| 2 | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Instead, Defend 3. |
| 3 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Instead, Defend 2 per card in the Invader discard pile. |


### Innate: OBSERVE THE EVER-CHANGING WORLD

- **Speed**: fast · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | Prepare 1 Element Marker. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Instead, after each of the next three Actions that change which pieces are in target land, Prepare 1 Element Marker. (You can stack the markers you intend to take here or on the target land so you don't need to ponder on what to take mid-turn. Any Action can trigger Preparing a marker, not just your own.) |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Shifting Memory of Ages's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/shifting-memory-of-ages.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/shifting-memory-of-ages.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Earth** (wt 4.2), **Moon** (wt 2.7), **Air** (wt 2.1)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 1 · Control 2 · Fear 2 · Defense 4 · Utility 5
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Haunted by Primal Memories** | 1 | Fast | Moon, Air, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Defend 3. If Beasts are present, +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+earth+moon → 9.0 |
| 2 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | elements air+earth+moon → 9.0 |
| 3 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if Invaders are present. Isolate target land. | elements earth+moon → 6.9 |
| 4 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Defend 3. | elements earth+moon → 6.9 |
| 5 | **Guardian Serpents** | 1 | Fast | Sun, Moon, Earth, Animal | Add 1 Beasts in one of target Spirit's lands. If target Spirit has a Sacred Site in that … | elements earth+moon → 6.9 |
| 6 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements earth+moon → 6.9 |
| 7 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements air+earth → 6.3 |
| 8 | **Drift Down into Slumber** | 0 | Fast | Air, Earth, Plant | Defend 1. If target land is a Jungle or Sands, instead Defend 4. | elements air+earth → 6.3 |
| 9 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon → 6.9 |
| 10 | **Rites of the Land's Rejection** | 1 | Fast | Moon, Fire, Earth | Invaders do not Build in target land this turn. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Town/City or 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Dahan,… | elements earth+moon → 6.9 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+earth+moon → 9.0 |
| 2 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+earth+moon → 9.0 |
| 3 | **Sleep and Never Waken** | 3 | Fast | Moon, Air, Earth, Animal | Invaders skip all Actions in target land. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> this Power Removes. Remo… | elements air+earth+moon → 9.0 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+moon → 9.0 |
| 5 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+earth+moon → 9.0 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Shifting Memory of Ages in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Scour the Land** | adds Blight |
| **Land of Haunts and Embers** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/shifting-memory-of-ages.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board in the highest-numbered land that is Sands or Mountain. Prepare 1 Simplemoon, 1 Simpleair, and 1 Simpleearth marker (put them by your Special Rules).
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence0 (Place 1 Presence from a track (Range 0))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); energy2 ((track slot showing 2 Energy))
- **G4**: energy9 (+9 Energy this turn (growth effect))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · energy2 · energy3marker · energy4 · reclaim1E · energy5 · energy6marker` — income as slots reveal: 0 → 1 → 2 → 3 → 4 → reclaim1E → 5 → 6

### Card-play track

`card1 · card2 · card2 · markersforplay · card3` — CP as slots reveal: 1 → 2 → 2 → markersforplay → 3

### Innate Powers

- **LEARN THE INVADERS' TACTICS** (Speed: Fast · Range: 1 · Target: invaders)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Defend 2.
  - **L2** — 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Instead, Defend 3.
  - **L3** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Instead, Defend 2 per card in the Invader discard pile.
- **OBSERVE THE EVER-CHANGING WORLD** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon: Prepare 1 Element Marker.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Instead, after each of the next three Actions that change which pieces are in target land, Prepare 1 Element Marker. (You can stack the markers you intend to take here or on the target land so you don't need to ponder on what to take mid-turn. Any Action can trigger Preparing a marker, not just your own.)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/shifting-memory-of-ages.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Shifting Memory of Ages](https://spiritislandwiki.com/index.php?title=Shifting_Memory_of_Ages).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
