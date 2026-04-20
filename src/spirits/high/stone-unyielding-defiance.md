# Stone's Unyielding Defiance

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
| Power summary (1–5)   | Offense 4 · Control 2 · Fear 1 · Defense 5 · Utility 2             |
| Primary Elements      | Earth, Plant (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Most of its special rules and innates require being where the Invaders are - particularly in the worst, most-overrun lands, so it can mitigate incoming Blight and (eventually) destroy the Invaders with their own Ravages. Does best with the patience to build up a position over time, and the temperance to hold some Energy in reserve so it can take advantage of Hold the Island Fast With a Bulwark of Will.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board: 1 in the lowest-numbered Mountain without Dahan; 1 in an adjacent land that has Blight (if possible) or is Sands (if not).

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain2earth, third=Stone |
| G2 | first=addpresence2, second=energy3 |
| G3 | first=gain1p, second=addpresence1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, energy3, card+1stone, energy4, card+1stone, energy6, card+1stone
- **Card-play track**: card1, earthX, earthX, earthreclaimX, earthanyX, card2earth

## Core Mechanics & Special Rules

### Special Rule

BESTOW THE ENDURANCE OF BEDROCK When Blight is added to one of your lands, unless the Blight then outnumbers your Presence, it does not cascade or destroy Presence (yours or others'). DEEP LAYERS EXPOSED TO THE SURFACE The first time you uncover each of your "+1 Card Play" Presence spaces, gain a Minor Power. (They're marked with [[Image:Minorsymbol.png|15px]] as a reminder.)

### Innate: HOLD THE ISLAND FAST WITH A BULWARK OF WILL

- **Speed**: fast · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | When Blight is added to one of your lands, you may pay 2 Energy per Blight to take it from the box instead of the Blight Card. (Handle any cascade separately.) |
| 2 | 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | The cost is 1 Energy instead of 2. |
| 3 | 6 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | When an Event or Blight card directly destroys Presence (yours or others'), you may prevent any number of Presence from being destroyed by paying 1 Energy each. ("Directly" means "not by adding Blight".) |


### Innate: LET THEM BREAK THEMSELVES AGAINST THE STONE

- **Speed**: fast · **Range**: 0 · **Target**: any

_(no thresholds listed in Wiki)_


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Stone's Unyielding Defiance's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/stone-unyielding-defiance.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/stone-unyielding-defiance.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Earth** (wt 6.0), **Plant** (wt 0.3)
- **Mid-game energy estimate (T3–T5 avg)**: 5.0E
- **Power summary**: Offense 4 · Control 2 · Fear 1 · Defense 5 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements earth → 6.0 |
| 2 | **Mesmerized Tranquility** | 0 | Fast | Water, Earth, Animal | Isolate target land. Each Invader does -1 Damage. | elements earth → 6.0 |
| 3 | **Carapaced Land** | 0 | Fast | Earth, Plant, Animal | If targeting a land with Beasts, this Power has +1 Range. Defend 3. | elements earth+plant → 6.3 |
| 4 | **Drift Down into Slumber** | 0 | Fast | Air, Earth, Plant | Defend 1. If target land is a Jungle or Sands, instead Defend 4. | elements earth+plant → 6.3 |
| 5 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements earth → 6.0 |
| 6 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if Invaders are present. Isolate target land. | elements earth → 6.0 |
| 7 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Defend 3. | elements earth+plant → 6.3 |
| 8 | **Entrap the Forces of Corruption** | 1 | Fast | Earth, Plant, Animal | Gather up to 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. Isolate target land. When Blight is added to target land, it doesn… | elements earth+plant → 6.3 |
| 9 | **Nature's Resilience** | 1 | Fast | Earth, Plant, Animal | Defend 6. | elements earth+plant → 6.3 |
| 10 | **Encompassing Ward** | 1 | Fast | Sun, Water, Earth | Target Spirit provides Defend 2 in each of its lands. | elements earth → 6.0 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **The Trees and Stones Speak of War** | 2 | Fast | Sun, Earth, Plant | For each Dahan, 1 Damage and Defend 2. | elements earth+plant → 6.3 |
| 2 | **Walls of Rock and Thorn** | 4 | Fast | Sun, Earth, Plant | 2 Damage. Defend 8. Add 1 Wilds. Isolate target land. | elements earth+plant → 6.3 |
| 3 | **Bloodwrack Plague** | 4 | Fast | Water, Earth, Animal | Add 2 Disease. For each Disease in target land, Defend 1 in target and all adjacent lands. | elements earth → 6.0 |
| 4 | **Melt Earth Into Quicksand** | 4 | Fast | Moon, Water, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 2 Damage. Isolate target land. After Invaders/Dahan are Moved into target land, D… | elements earth → 6.0 |
| 5 | **Bargains of Power and Protection** | 2 | Fast | Sun, Water, Earth, Animal | Remove 1 of your Presence on the island from the game, setting it on the Reminder Card. F… | elements earth → 6.0 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Stone's Unyielding Defiance in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/stone-unyielding-defiance.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in the lowest-numbered Mountain without Dahan; 1 in an adjacent land that has Blight (if possible) or is Sands (if not).
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain2earth ((spirit-specific: `gain2earth` — consult spirit panel)); Stone ((spirit-specific: `Stone` — consult spirit panel))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2)); energy3 ((+3 Energy this turn — growth effect, not track reveal))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · energy3 · card+1stone · energy4 · card+1stone · energy6 · card+1stone` — income as slots reveal: 2 → 3 → card+1stone → 4 → card+1stone → 6 → card+1stone

### Card-play track

`card1 · earthX · earthX · earthreclaimX · earthanyX · card2earth` — CP as slots reveal: 1 → earthX → earthX → earthreclaimX → earthanyX → 2

### Innate Powers

- **HOLD THE ISLAND FAST WITH A BULWARK OF WILL** (Speed: Fast · Range: ? · Target: you)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: When Blight is added to one of your lands, you may pay 2 Energy per Blight to take it from the box instead of the Blight Card. (Handle any cascade separately.)
  - **L2** — 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: The cost is 1 Energy instead of 2.
  - **L3** — 6 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: When an Event or Blight card directly destroys Presence (yours or others'), you may prevent any number of Presence from being destroyed by paying 1 Energy each. ("Directly" means "not by adding Blight".)
- **LET THEM BREAK THEMSELVES AGAINST THE STONE** (Speed: Fast · Range: 0 · Target: any)

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


### Strategy Cliffs — per-adversary-level shifts that change Stone's Unyielding Defiance's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Stone's Unyielding Defiance's profile (Fear 1, Offense 4, Control 2, Defense 5, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Stone's Unyielding Defiance**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Stone's Unyielding Defiance**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Stone's Unyielding Defiance**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/stones-unyielding-defiance.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Stone's Unyielding Defiance](https://spiritislandwiki.com/index.php?title=Stone's_Unyielding_Defiance).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
