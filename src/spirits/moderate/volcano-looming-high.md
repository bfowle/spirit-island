# Volcano Looming High

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
| Power summary (1–5)   | Offense 5 · Control 1 · Fear 2 · Defense 1 · Utility 3             |
| Primary Elements      | Fire, Earth, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Benefits more than most Spirits from getting Presence onto the board; in addition to the usual benefits, it can fuel an Explosive Eruption. This can result in a huge turn, but if overdone the following turn or two may be very constrained. Bigger eruptions are extremely powerful, but cause Blight, and the Invaders may not provide the luxury of enough time to build up the desired pressure - judging the timing of when to erupt and for how much is a key part of playing this Spirit.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Badlands on your starting board in a mountain of your choice. Push all Dahan from that land.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy3 |
| G2 | first=addpresence0, second=addpresence0 |
| G3 | first=gain1p, second=addpresence4, third=new+1cardplay |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, earth, energy3, energy4, energy5
- **Card-play track**: card1, fireX, earthX, card2, airX, card3, fireX, card4

## Core Mechanics & Special Rules

### Special Rule

MOUNTAIN HOME Your Presence may only be added/moved into Mountains. COLLAPSE IN A BLAST OF LAVA AND STEAM When your Presence is destroyed, in that land, deal 1 Damage per destroyed Presence to both Invaders and to Dahan. VOLCANIC PEAKS TOWER OVER THE LANDSCAPE Your Power Cards gain Range +1 if you have 3 or more Presence in the origin land.

### Innate: EXPLOSIVE ERUPTION

- **Speed**: fast · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | — | Destroy X (1 or more) of your Presence in target land; {{volcanodestroyedpresence}} checks how many you destroyed. This Power does Damage (separately and equally) to both Invaders and Dahan. Ranges below can't be increased. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In one land within Range 1, X Damage. |
| 3 | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Generate X Fear. |
| 4 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In each land within Range 1, 4 Damage. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> to target land; doing so does not Destroy your Presence. |
| 5 | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 5 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In each land within Range 2, +4 Damage. In each land adjacent to the target, add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> if it doesn't have any. |


### Innate: POWERED BY THE FURNACE OF THE EARTH

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Add 1 of your destroyed Presence. |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Gain a Power Card. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Move up to 2 of your Presence from other lands to target land. |
| 4 | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Return up to 2 of your destroyed Presence to your Presence tracks. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Volcano Looming High's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/volcano-looming-high.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/volcano-looming-high.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Fire** (wt 9.3), **Earth** (wt 8.7), **Air** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 5 · Control 1 · Fear 2 · Defense 1 · Utility 3
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+earth+fire → 19.5 |
| 2 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+earth+fire → 19.5 |
| 3 | **Desiccating Winds** | 1 | Slow | Fire, Air, Earth | If target land has Badlands, 1 Damage. Add 1 Badlands. | elements air+earth+fire → 19.5 |
| 4 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements earth+fire → 18.0 |
| 5 | **Treacherous Waterways** | 0 | Fast | Fire, Water, Earth | Add 1 Wilds. **OR** Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements earth+fire → 18.0 |
| 6 | **Drought** | 1 | Slow | Sun, Fire, Earth | Destroy 3 [[Towns]]. 1 Damage to each [[Town]]/[[City]]. Add 1 [[Blight]]. | elements earth+fire → 18.0 |
| 7 | **Rouse the Trees and Stones** | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements earth+fire → 18.0 |
| 8 | **Unquenchable Flames** | 1 | Slow | Moon, Fire, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage to Towns/Cities. Invaders do not heal Damage at end of turn. | elements earth+fire → 18.0 |
| 9 | **Call to Ferocity** | 0 | Slow | Sun, Fire, Earth | Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. **OR** If target land has Dahan, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 T… | elements earth+fire → 18.0 |
| 10 | **Gold's Allure** | 0 | Slow | Fire, Earth, Animal | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. Add 1 Strife. | elements earth+fire → 18.0 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+fire → 19.5 |
| 2 | **Pyroclastic Flow** | 3 | Fast | Fire, Air, Earth | 2 Damage. Destroy all Explorers. If target land is a Jungle or Wetland, add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. | elements air+earth+fire → 19.5 |
| 3 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements earth+fire → 18.0 |
| 4 | **Spill Bitterness Into the Earth** | 5 | Fast | Fire, Water, Earth | 6 Damage. Add 2 Badlands/Strife and 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. In up to 3 adjacent lands with Blight, add … | elements earth+fire → 18.0 |
| 5 | **Unearth a Beast of Wrathful Stone** | 5 | Fast | Moon, Fire, Earth, Animal | After the next Invader Phase (on any turn) with no Ravage/Build Actions in target land:</… | elements earth+fire → 18.0 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Volcano Looming High in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/volcano-looming-high.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Badlands on your starting board in a mountain of your choice. Push all Dahan from that land.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy3 ((+3 Energy this turn — growth effect, not track reveal))
- **G2**: addpresence0 (Place 1 Presence from a track (Range 0)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence4 (Place 1 Presence from a track (Range 4)); new+1cardplay ((spirit-specific: `new+1cardplay` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · earth · energy3 · energy4 · energy5` — income as slots reveal: 1 → 2 → earth → 3 → 4 → 5

### Card-play track

`card1 · fireX · earthX · card2 · airX · card3 · fireX · card4` — CP as slots reveal: 1 → fireX → earthX → 2 → airX → 3 → fireX → 4

### Innate Powers

- **EXPLOSIVE ERUPTION** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — (no threshold): Destroy X (1 or more) of your Presence in target land; {{volcanodestroyedpresence}} checks how many you destroyed. This Power does Damage (separately and equally) to both Invaders and Dahan. Ranges below can't be increased.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: In one land within Range 1, X Damage.
  - **L3** — 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Generate X Fear.
  - **L4** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: In each land within Range 1, 4 Damage. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> to target land; doing so does not Destroy your Presence.
  - **L5** — 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 5 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: In each land within Range 2, +4 Damage. In each land adjacent to the target, add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> if it doesn't have any.
- **POWERED BY THE FURNACE OF THE EARTH** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Add 1 of your destroyed Presence.
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire: Gain a Power Card.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Move up to 2 of your Presence from other lands to target land.
  - **L4** — 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire: Return up to 2 of your destroyed Presence to your Presence tracks.

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


### Strategy Cliffs — per-adversary-level shifts that change Volcano Looming High's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Volcano Looming High's profile (Fear 2, Offense 5, Control 1, Defense 1, Utility 3).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Volcano Looming High**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Volcano Looming High**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Volcano Looming High**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/volcano-looming-high.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Volcano Looming High](https://spiritislandwiki.com/index.php?title=Volcano_Looming_High).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
