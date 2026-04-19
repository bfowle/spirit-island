# Fathomless Mud of the Swamp

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
| Power summary (1–5)   | Offense 3 · Control 2 · Fear 3 · Defense 3 · Utility 2             |
| Primary Elements      | Water, Moon, Earth, Plant (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Likes having Sacred Site where Invaders will Build, but may need to re-create those Sacred Site after oozing outwards with its Innate Power. In smaller games, might be able to cut off the most Inland lands from Explore actions by Destroying Inland Town/City and stopping new ones from being built. Causes a fair bit of Fear, much of which represents unpleasantness, hardship, and disgust.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board, in the lowest-numbered Wetland. You start with your 4 Unique Powers and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=addpresence1, second=addpresence1 |
| G3 | first=gain1p, second=addpresence2, third=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, plant, energy2, energy3, water, energy4, energy5
- **Card-play track**: card1, card2, earthX, card3, moonX, card4

## Core Mechanics & Special Rules

### Special Rule

OFFER NO FIRM FOUNDATIONS At your Sacred Site, Build actions add Explorer instead of Town/City.

### Innate: SPREADING AND DREADFUL MIRE

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Water | Move 1 Presence from the origin Sacred Site to target land. (This is required.) |
| 2 | 1 Moon + 2 Water + 1 Earth | 1 Fear. 1 Damage. Push 1 Dahan. |
| 3 | 2 Moon + 3 Water + 2 Earth | 1 Fear. 1 Damage. Push 1 Dahan. |
| 4 | 3 Moon + 4 Water + 3 Earth + 2 Plant | 2 Damage. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Fathomless Mud of the Swamp's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/fathomless-mud-of-the-swamp.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/fathomless-mud-of-the-swamp.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Water** (wt 4.2), **Moon** (wt 2.1), **Earth** (wt 2.1)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 3 · Control 2 · Fear 3 · Defense 3 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon+plant+water → 9.0 |
| 2 | **Infested Aquifers** | 1 | Slow | Moon, Water, Earth, Animal | If target land has any Disease, 1 Damage to each Invader. **OR** If target land is a Moun… | elements earth+moon+water → 8.4 |
| 3 | **Pull Beneath the Hungry Earth** | 1 | Slow | Moon, Water, Earth | If your Presence is present, 1 Fear and 1 Damage. If target land is a Sands or Wetland, 1… | elements earth+moon+water → 8.4 |
| 4 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear if Invaders are present. Isolate target land. | elements earth+moon+water → 8.4 |
| 5 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements moon+plant+water → 6.9 |
| 6 | **The Shore Seethes with Hatred** | 1 | Slow | Fire, Water, Earth, Plant | 1 Fear. Add 1 Badlands and 1 Wilds. | elements earth+plant+water → 6.9 |
| 7 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear. Terror Level 2: 2 Fear or add 1 Strife.… | elements moon+water → 6.3 |
| 8 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements moon+water → 6.3 |
| 9 | **Mesmerized Tranquility** | 0 | Fast | Water, Earth, Animal | Isolate target land. Each Invader does -1 Damage. | elements earth+water → 6.3 |
| 10 | **Renewing Rain** | 1 | Slow | Water, Earth, Plant | If target land is a Jungle or Sands, Remove 1 Blight. | elements earth+plant+water → 6.9 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Melt Earth Into Quicksand** | 4 | Fast | Moon, Water, Earth | 1 Fear. 2 Damage. Isolate target land. After Invaders/Dahan are Moved into target land, D… | elements earth+moon+water → 8.4 |
| 2 | **Dream of the Untouched Land** | 6 | Fast | Moon, Water, Earth, Plant, Animal | Remove up to 3 Blight and up to 3 Health worth of Invaders. | elements earth+moon+plant+water → 9.0 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements earth+moon+plant+water → 9.0 |
| 4 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements earth+moon+water → 8.4 |
| 5 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements earth+moon+water → 8.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Fathomless Mud of the Swamp in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Steam Vents** | Minor | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer. |
| **Entrancing Apparitions** | Minor | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers. |
| **Uncanny Melting** | Minor | 1 | Slow | Sun, Moon, Water | If Invaders are present, 1 Fear. If target land is a Sands or Wetland, Remove 1 Blight. |
| **Cleansing Floods** | Major | 5 | Slow | Sun, Water | 4 Damage. Remove 1 Blight. |
| **Pull Beneath the Hungry Earth** | Minor | 1 | Slow | Moon, Water, Earth | If your Presence is present, 1 Fear and 1 Damage. If target land is a Sands or Wetland, 1… |
| **The Land Thrashes in Furious Pain** | Major | 4 | Slow | Moon, Fire, Earth | 2 Damage per Blight. For each Blight in adjacent lands, 1 Damage (in target land). |
| **Gift of Power** | Minor | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Tsunami** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |

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


### Strategy Cliffs — per-adversary-level shifts that change Fathomless Mud of the Swamp's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Fathomless Mud of the Swamp's profile (Fear 3, Offense 3, Control 2, Defense 3, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Fathomless Mud of the Swamp**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Fathomless Mud of the Swamp**: Front-load coastal defense or disruption before T3's first Ravage.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Fathomless Mud of the Swamp**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Fathomless Mud of the Swamp**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Fathomless Mud of the Swamp**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Fathomless Mud of the Swamp**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/fathomless-mud-of-the-swamp.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Fathomless Mud of the Swamp](https://spiritislandwiki.com/index.php?title=Fathomless_Mud_of_the_Swamp).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
