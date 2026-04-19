# Dances Up Earthquakes

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | Very High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 2 · Defense 3 · Utility 4             |
| Primary Elements      | Earth, Moon (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very much about tempo and timing: Can play high-cost Powers extremely easily, but they won't take effect until later in the game. Faces a constant tension between solving problems now and carefully planning ahead for big turns in the future - neglecting either one can be disastrous. Despite starting with 6 Unique Powers, benefits greatly from gaining more.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the highest-numbered land with Dahan. You start with your 6 Unique Power Cards and 0 Energy. Set the Quake Tokens ({{quake}}) nearby.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence2orgainmajorwoforgetting |
| G2 | first=gain1p, second=addpresence1 |
| G3 | first=addpresence3, second=energyimpend, third=reclaim1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1impendenergy1, movepresence1energytrack, energy2, impend1, energy3, impendenergy2, energy4any
- **Card-play track**: card2, gather1dahan1land, moonfire, impend1, earthX, card3, card4

## Core Mechanics & Special Rules

### Special Rule

BEGIN A DANCE OF DECADES</br>Whenever you would play a Power Card, you may instead pay any amount of Energy onto the card to make it an impending card ({{impendingcard}}), setting it aside out of play for use on a future turn. (It doesn't provide Elements. It's still your Power Card, so it can be forgotten while it's impending.) RHYTHMIC POWER BUILDS TO A CATACLYSMIC CRESCENDO</br>When you gain Energy from your Presence Track, also gain {{impendingenergyblank}} Energy onto each Power Card made {{impendingcard}} on a previous turn. If any {{impendingcard}} now have Energy on them at least equal to their cost, discard that Energy and play them. (This costs no card plays.)

### Innate: LAND CREAKS WITH TENSION

- **Speed**: fast · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Earth | If you have at least 1 {{impendingcard}}, Add 1 {{quake}} in one of your lands. |
| 2 | 1 Moon + 1 Earth | In one of your lands, Defend 1 per {{impendingcard}} (max. 3). |
| 3 | 1 Moon + 2 Earth | If you have at least 3 {{impendingcard}}, Add 1 {{quake}} in one of your lands. |
| 4 | 2 Moon + 3 Earth | In one of your lands, Defend 1 per {{impendingcard}} (max. 3). |


### Innate: EARTH SHUDDERS, BUILDINGS FALL

- **Speed**: slow · **Range**: 0 · **Target**: quake

_(no thresholds listed in Wiki)_


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Dances Up Earthquakes's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/dances-up-earthquakes.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/dances-up-earthquakes.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Earth** (wt 3.0), **Moon** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 3.5E
- **Power summary**: Offense 5 · Control 2 · Fear 2 · Defense 3 · Utility 4
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements earth+moon → 4.5 |
| 2 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear if Invaders are present. Isolate target land. | elements earth+moon → 4.5 |
| 3 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | elements earth+moon → 4.5 |
| 4 | **Infested Aquifers** | 1 | Slow | Moon, Water, Earth, Animal | If target land has any Disease, 1 Damage to each Invader. **OR** If target land is a Moun… | elements earth+moon → 4.5 |
| 5 | **Pull Beneath the Hungry Earth** | 1 | Slow | Moon, Water, Earth | If your Presence is present, 1 Fear and 1 Damage. If target land is a Sands or Wetland, 1… | elements earth+moon → 4.5 |
| 6 | **Unquenchable Flames** | 1 | Slow | Moon, Fire, Earth | 1 Fear. 1 Damage to Towns/Cities. Invaders do not heal Damage at end of turn. | elements earth+moon → 4.5 |
| 7 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear. If target land is a Mountain or Jungle, Defend 3. | elements earth+moon → 4.5 |
| 8 | **Guardian Serpents** | 1 | Fast | Sun, Moon, Earth, Animal | Add 1 Beasts in one of target Spirit's lands. If target Spirit has a Sacred Site in that … | elements earth+moon → 4.5 |
| 9 | **Haunted by Primal Memories** | 1 | Fast | Moon, Air, Earth | 1 Fear. Defend 3. If Beasts are present, +2 Fear. | elements earth+moon → 4.5 |
| 10 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements earth → 3.0 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Melt Earth Into Quicksand** | 4 | Fast | Moon, Water, Earth | 1 Fear. 2 Damage. Isolate target land. After Invaders/Dahan are Moved into target land, D… | elements earth+moon → 4.5 |
| 2 | **The Land Thrashes in Furious Pain** | 4 | Slow | Moon, Fire, Earth | 2 Damage per Blight. For each Blight in adjacent lands, 1 Damage (in target land). | elements earth+moon → 4.5 |
| 3 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements earth+moon → 4.5 |
| 4 | **Unearth a Beast of Wrathful Stone** | 5 | Fast | Moon, Fire, Earth, Animal | After the next Invader Phase (on any turn) with no Ravage/Build Actions in target land:</… | elements earth+moon → 4.5 |
| 5 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements earth+moon → 4.5 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Dances Up Earthquakes in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Pyroclastic Flow** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **The Jungle Hungers** | destroys Dahan |
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


### Strategy Cliffs — per-adversary-level shifts that change Dances Up Earthquakes's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Dances Up Earthquakes's profile (Fear 2, Offense 5, Control 2, Defense 3, Utility 4).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Dances Up Earthquakes**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Dances Up Earthquakes**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Dances Up Earthquakes**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/dances-up-earthquakes.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Dances Up Earthquakes](https://spiritislandwiki.com/index.php?title=Dances_Up_Earthquakes).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
