# Rising Heat of Stone and Sand

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Horizons of Spirit Island                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 3 · Fear 1 · Defense 2 · Utility 3             |
| Primary Elements      | Fire, Air, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Weaken-and-Destroy. Likes having Sacred Site where there's Town/City, as it makes all Spirits' Damage more effective there. Has an easier time setting up Sacred Site in Sands and Mountains, but can do so in any terrain with a bit more time.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board, in the highest-numbered Sands. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence3ms, second=addpresence3ms |
| G3 | first=gain1p, second=addpresence1, third=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, earth, energy2, energy3, fire, energy4, energy5
- **Card-play track**: card1, card2, card2, card3, card4, card5fire

## Core Mechanics & Special Rules

### Special Rule

BLISTERING HEAT At your Sacred Site, Invaders have -1 Health (min. 1).

### Innate: SCORCH WITH WAVES OF HEAT

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Fire + 2 Air | 2 Damage to Explorer only. |
| 2 | 3 Fire + 2 Earth | 2 Damage. |
| 3 | 4 Fire + 1 Air + 3 Earth | 2 Damage. |
| 4 | 5 Fire + 2 Air + 3 Earth | 1 Damage to each Invader. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Rising Heat of Stone and Sand's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/rising-heat-of-stone-and-sand.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/rising-heat-of-stone-and-sand.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Fire** (wt 6.3), **Earth** (wt 3.0), **Air** (wt 2.7)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 5 · Control 3 · Fear 1 · Defense 2 · Utility 3
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Desiccating Winds** | 1 | Slow | Fire, Air, Earth | If target land has Badlands, 1 Damage. Add 1 Badlands. | elements air+earth+fire → 12.0 |
| 2 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer. | elements air+earth+fire → 12.0 |
| 3 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+earth+fire → 12.0 |
| 4 | **Rouse the Trees and Stones** | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. | elements earth+fire → 9.3 |
| 5 | **Drought** | 1 | Slow | Sun, Fire, Earth | Destroy 3 [[Towns]]. 1 Damage to each [[Town]]/[[City]]. Add 1 [[Blight]]. | elements earth+fire → 9.3 |
| 6 | **Unquenchable Flames** | 1 | Slow | Moon, Fire, Earth | 1 Fear. 1 Damage to Towns/Cities. Invaders do not heal Damage at end of turn. | elements earth+fire → 9.3 |
| 7 | **Call to Ferocity** | 0 | Slow | Sun, Fire, Earth | Gather up to 3 Dahan. **OR** If target land has Dahan, 1 Fear and Push 1 Explorer and 1 T… | elements earth+fire → 9.3 |
| 8 | **Gold's Allure** | 0 | Slow | Fire, Earth, Animal | Gather 1 Explorer and 1 Town. Add 1 Strife. | elements earth+fire → 9.3 |
| 9 | **Dry Wood Explodes in Smoldering Splinters** | 1 | Slow | Fire, Air, Plant | You may spend 1 Energy to make this Power Fast. 2 Fear. 1 Damage. | elements air+fire → 9.0 |
| 10 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire → 9.0 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+fire → 12.0 |
| 2 | **Pyroclastic Flow** | 3 | Fast | Fire, Air, Earth | 2 Damage. Destroy all Explorers. If target land is a Jungle or Wetland, add 1 Blight. | elements air+earth+fire → 12.0 |
| 3 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements earth+fire → 9.3 |
| 4 | **The Land Thrashes in Furious Pain** | 4 | Slow | Moon, Fire, Earth | 2 Damage per Blight. For each Blight in adjacent lands, 1 Damage (in target land). | elements earth+fire → 9.3 |
| 5 | **Rumbling Earthquakes** | 6 | Slow | Fire, Earth | This Power ignores Health bonuses.</br>3 Fear. 6 Damage, to Towns/Cities only.</br>6 Dama… | elements earth+fire → 9.3 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Rising Heat of Stone and Sand in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Visions of Fiery Doom** | Minor | 1 | Fast | Moon, Fire | 1 Fear. Push 1 Explorer/Town. |
| **Drought** | Minor | 1 | Slow | Sun, Fire, Earth | Destroy 3 [[Towns]]. 1 Damage to each [[Town]]/[[City]]. Add 1 [[Blight]]. |
| **Purifying Flame** | Minor | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… |
| **Indomitable Claim** | Major | 4 | Fast | Sun, Earth | Add 1 Presence in target land even if you normally could not due to land type. Defend 20. |
| **Encompassing Ward** | Minor | 1 | Fast | Sun, Water, Earth | Target Spirit provides Defend 2 in each of its lands. |
| **Talons of Lightning** | Major | 6 | Fast | Fire, Air | 3 Fear. 5 Damage. |
| **Land of Haunts and Embers** | Minor | 0 | Fast | Moon, Fire, Air | 2 Fear. Push up to 2 Explorers/Towns. If Blight is present, 2 Fear and Push up to 2 Explo… |

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
| **Pillar of Living Flame** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opening Strategy

`[VERIFY: needs play data]` — opening variants should be rehearsed turn-by-turn per the [SPIRIT_TEMPLATE.md](../../../templates/SPIRIT_TEMPLATE.md) opener format.

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


### Strategy Cliffs — per-adversary-level shifts that change Rising Heat of Stone and Sand's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Rising Heat of Stone and Sand's profile (Fear 1, Offense 5, Control 3, Defense 2, Utility 3).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Rising Heat of Stone and Sand**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Rising Heat of Stone and Sand**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Rising Heat of Stone and Sand**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Rising Heat of Stone and Sand**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/rising-heat-of-stone-and-sand.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Rising Heat of Stone and Sand](https://spiritislandwiki.com/index.php?title=Rising_Heat_of_Stone_and_Sand).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
