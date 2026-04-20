# Keeper of the Forbidden Wilds

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch and Claw                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "two" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 1 · Defense 4 · Utility 3             |
| Primary Elements      | Plant, Sun, Fire, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> A slowly growing wall - expanding can sometimes be difficult, but the Invaders will have an equally difficult time penetrating wherever the Keeper plants itself. In larger games, it may be useful to spread to one of the two far-distant lands early on, to have multiple points from which to slowly grow.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Wilds on your starting board in the highest-numbered Jungle.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=energy1 |
| G2 | first=gain1p |
| G3 | first=Keeper, second=energy1 |
| G4 | first=Keeper3, second=noblight |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, sun, energy4, energy5, plant, energy7, energy8, energy9
- **Card-play track**: card1, card2, card2, card3, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

FORBIDDEN GROUND After you create a Sacred Site, Push all Dahan from that land. Dahan Events never move Dahan to your Sacred Site, but Powers can do so.

### Innate: PUNISH THOSE WHO TRESPASS

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 2 Damage. Destroy 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | +1 Damage per SunPlant you have. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Split this Power's Damage however desired between target land and another 1 of your lands. |


### Innate: SPREADING WILDS

- **Speed**: slow · **Range**: 1 · **Target**: noblight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun | Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> from target land per 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun you have. |
| 2 | 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | If target land has no Explorer, add 1 Wilds. |
| 3 | 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | This Power has Range +1. |
| 4 | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power has Range +1. |


## Unique Cards (all, Wiki-verified)

#### Boon of Growing Power

- **1 Energy · Slow · Range No Range · Any Spirit · Sun, Moon, Plant**
- *Target Spirit gains a Power Card. If you target another Spirit, they also gain 1 Energy.*

#### Regrow from Roots

- **1 Energy · Slow · Range 1 · Jungle or Wetland · Water, Earth, Plant**
- *If there are 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> or fewer in target land, Remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">.*

#### Sacrosanct Wilderness

- **2 Energy · Fast · Range 1 · Land with no Blight · Sun, Earth, Plant**
- *Push 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. 2 Damage per Wilds in target land. **OR** Add 1 Wilds.*

#### Towering Wrath

- **3 Energy · Slow · Range 1, from your Sacred Site · Any Land · Sun, Fire, Plant**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. For each of your Sacred Site in/adjacent to target land, 2 Damage. Destroy all Dahan.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Keeper of the Forbidden Wilds's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/keeper-of-the-forbidden-wilds.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/keeper-of-the-forbidden-wilds.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Plant** (wt 6.3), **Sun** (wt 4.8), **Fire** (wt 2.1)
- **Mid-game energy estimate (T3–T5 avg)**: 6.67E
- **Power summary**: Offense 5 · Control 2 · Fear 1 · Defense 4 · Utility 3
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Boon of Growing Power**
- **Regrow from Roots**
- **Sacrosanct Wilderness**
- **Towering Wrath**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | elements fire+plant+sun → 13.2 |
| 2 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire+plant+sun → 13.5 |
| 3 | **Gift of Living Energy** | 0 | Fast | Sun, Fire, Plant | Target Spirit gains 1 Energy. If you have at least 2 Sacred Sites, target Spirit gains 1 … | elements fire+plant+sun → 13.2 |
| 4 | **Pact of the Joined Hunt** | 1 | Slow | Sun, Plant, Animal | Target Spirit Gathers 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> into one of their lands. 1 Damage in that land per Dahan pr… | elements plant+sun → 11.1 |
| 5 | **Enticing Splendor** | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. **OR** Gather up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | elements air+plant+sun → 11.4 |
| 6 | **Favor of the Sun and Star-lit Dark** | 1 | Fast | Sun, Moon, Plant | Defend 4. Push up to 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. | elements plant+sun → 11.1 |
| 7 | **Absorb Corruption** | 1 | Slow | Sun, Earth, Plant | Gather 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. **OR** Pay 1 Energy to Remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. | elements plant+sun → 11.1 |
| 8 | **Like Calls to Like** | 1 | Slow | Sun, Water, Plant | If target land has Explorer, Gather up to 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. Do likewise for Town, Dahan, Blight… | elements plant+sun → 11.1 |
| 9 | **Song of Sanctity** | 1 | Slow | Sun, Water, Plant | If Explorer(s) are present, Push all Explorers. Otherwise, Remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. | elements plant+sun → 11.1 |
| 10 | **Teeming Rivers** | 1 | Slow | Sun, Water, Plant, Animal | If target land has no Blight, add 1 Beasts. If target land has exactly 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">, Remove i… | elements plant+sun → 11.1 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements fire+plant+sun → 13.2 |
| 2 | **Unrelenting Growth** | 4 | Slow | Sun, Fire, Water, Plant | Target Spirit adds 2 Presence and 1 Wilds to a land at Range 1 of their Presence. | elements fire+plant+sun → 13.2 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+fire+plant+sun → 13.5 |
| 4 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+plant+sun → 11.4 |
| 5 | **The Trees and Stones Speak of War** | 2 | Fast | Sun, Earth, Plant | For each Dahan, 1 Damage and Defend 2. | elements plant+sun → 11.1 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Keeper of the Forbidden Wilds in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Renewing Boon** | destroys Presence |
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pillar of Living Flame** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/keeper-of-the-forbidden-wilds.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Wilds on your starting board in the highest-numbered Jungle.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card1`): **2 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G3**: Keeper ((spirit-specific: `Keeper` — consult spirit panel)); energy1 ((track slot showing 1 Energy))
- **G4**: Keeper3 ((spirit-specific: `Keeper3` — consult spirit panel)); noblight ((spirit-specific: `noblight` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · sun · energy4 · energy5 · plant · energy7 · energy8 · energy9` — income as slots reveal: 2 → sun → 4 → 5 → plant → 7 → 8 → 9

### Card-play track

`card1 · card2 · card2 · card3 · card4 · card5reclaim1` — CP as slots reveal: 1 → 2 → 2 → 3 → 4 → 5

### Innate Powers

- **PUNISH THOSE WHO TRESPASS** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: 2 Damage. Destroy 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: +1 Damage per SunPlant you have.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Split this Power's Damage however desired between target land and another 1 of your lands.
- **SPREADING WILDS** (Speed: Slow · Range: 1 · Target: noblight)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun: Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> from target land per 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun you have.
  - **L2** — 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: If target land has no Explorer, add 1 Wilds.
  - **L3** — 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: This Power has Range +1.
  - **L4** — 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: This Power has Range +1.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Earth** ×1, **Plant** ×1

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Boon of Growing Power** | 1 | Slow | No Range | Any Spirit | sun, moon, plant | Target Spirit gains a Power Card. If you target another Spirit, they also gain 1 Energy. |
| **Regrow from Roots** | 1 | Slow | 1 | Jungle or Wetland | water, earth, plant | If there are 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> or fewer in target land, Remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. |
| **Sacrosanct Wilderness** | 2 | Fast | 1 | Land with no Blight | sun, earth, plant | Push 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. 2 Damage per Wilds in target land. **OR** Add 1 Wilds. |
| **Towering Wrath** | 3 | Slow | 1, from your Sacred Site | Any Land | sun, fire, plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. For each of your Sacred Site in/adjacent to target land, 2 Damage. Destroy all Dahan. |

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


### Strategy Cliffs — per-adversary-level shifts that change Keeper of the Forbidden Wilds's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Keeper of the Forbidden Wilds's profile (Fear 1, Offense 5, Control 2, Defense 4, Utility 3).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Keeper of the Forbidden Wilds**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Keeper of the Forbidden Wilds**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Keeper of the Forbidden Wilds**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/keeper-of-the-forbidden-wilds.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Keeper of the Forbidden Wilds](https://spiritislandwiki.com/index.php?title=Keeper_of_the_Forbidden_Wilds).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
