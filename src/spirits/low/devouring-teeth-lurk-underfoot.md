# Devouring Teeth Lurk Underfoot

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
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 2 · Defense 1 · Utility 1             |
| Primary Elements      | Fire, Animal, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Likes being in the same lands as Invaders, so it can use Range 0 offensive and defensive Powers. The first of its Innate Power can give some mobility, if needed. Has a poor Plays track and potent but expensive Unique Powers, so can be better at handling fewer large threats than lots of little ones.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board, in land #5. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence1 |
| G3 | first=addpresence2, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, fire, energy3, energy4, animal, energy6, energy7
- **Card-play track**: card1, card2, animalX, fireX, card3, earthX, card4

## Core Mechanics & Special Rules

### Special Rule

TERRITORIAL AGGRESSION Your Damage-Dealing Powers do +1 Damage. (This adds +1 Damage total for the Power, even if the Power Damages multiple Invaders or "each Invader". It can boost Minor/Major Power Cards, too, not just your Uniques + Innate.)

### Innate: DEATH APPROACHES FROM BENEATH THE SURFACE

- **Speed**: slow · **Range**: 1 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Fire + 1 Animal | If you don't have Presence in target land, Gather 1 of your Presence. (This is required.) |
| 2 | 2 Fire + 1 Earth + 2 Animal | 1 Damage. (+1 for your "Territorial Aggression" Special Rule) |
| 3 | 3 Fire + 1 Earth + 3 Animal | 2 Damage. |
| 4 | 4 Fire + 2 Earth + 5 Animal | 2 Fear. 4 Damage. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Devouring Teeth Lurk Underfoot's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/devouring-teeth-lurk-underfoot.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/devouring-teeth-lurk-underfoot.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Animal** (wt 4.5), **Fire** (wt 4.2), **Earth** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 5.67E
- **Power summary**: Offense 5 · Control 2 · Fear 2 · Defense 1 · Utility 1
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Gold's Allure** | 0 | Slow | Fire, Earth, Animal | Gather 1 Explorer and 1 Town. Add 1 Strife. | elements animal+earth+fire → 10.2 |
| 2 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements animal+earth+fire → 10.2 |
| 3 | **Savage Mawbeasts** | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. | elements animal+fire → 8.7 |
| 4 | **Territorial Strife** | 0 | Slow | Sun, Fire, Animal | 3 Damage to Explorers/Towns. **OR** Add 1 Strife. | elements animal+fire → 8.7 |
| 5 | **Call to Bloodshed** | 1 | Slow | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan. | elements animal+fire → 8.7 |
| 6 | **Prowling Panthers** | 1 | Slow | Moon, Fire, Animal | 1 Fear. Add 1 Beasts. **OR** If target land has Beasts, Destroy 1 Explorer/Town. | elements animal+fire → 8.7 |
| 7 | **Weep for What is Lost** | 0 | Slow | Fire, Water, Animal | 1 Fear per type of Invader present. Push up to 1 Explorer/Town per Blight. | elements animal+fire → 8.7 |
| 8 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements animal+fire → 8.7 |
| 9 | **Call to Migrate** | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. | elements animal+fire → 8.7 |
| 10 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear. Add 1 Disease. | elements animal+fire → 8.7 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements animal+earth+fire → 10.2 |
| 2 | **Unearth a Beast of Wrathful Stone** | 5 | Fast | Moon, Fire, Earth, Animal | After the next Invader Phase (on any turn) with no Ravage/Build Actions in target land:</… | elements animal+earth+fire → 10.2 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements animal+earth+fire → 10.2 |
| 4 | **Angry Bears** | 3 | Slow | Sun, Fire, Animal | 2 Fear. 2 Damage. If no Beasts are present, add 1 Beasts. Otherwise, +2 Damage, and Push … | elements animal+fire → 8.7 |
| 5 | **The Wounded Wild Turns on its Assailants** | 4 | Slow | Fire, Plant, Animal | Add 2 Badlands. Gather up to 2 Beasts. 1 Damage per Blight/Beasts/Wilds. | elements animal+fire → 8.7 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Devouring Teeth Lurk Underfoot in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Devouring Ants** | Minor | 1 | Slow | Sun, Earth, Animal | 1 Fear. 1 Damage. Destroy 1 Dahan. If target land is a Jungle or Sands, +1 Damage. |
| **Gnawing Rootbiters** | Minor | 0 | Slow | Earth, Animal | Push up to 2 Towns. |
| Quicken the Earth\'s Struggles | — | — | — | — | (fetch error: Wiki API error for 'Quicken_the_Earth\'s_Struggles': {'code': 'missingtitle', 'info': "The page you specified doesn't exist.", 'docref': 'See https://spiritislandwiki.com/api.php for API usage. Subscribe to the mediawiki-api-announce mailing list at &lt;https://lists.wikimedia.org/mailman/listinfo/mediawiki-api-announce&gt; for notice of API deprecations and breaking changes.'}) |
| **Pillar of Living Flame** | Major | 5 | Slow | Fire | 3 Fear. 5 Damage. If target land is a Jungle or Wetland, add 1 Blight. |
| **Rouse the Trees and Stones** | Minor | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. |
| **Poisoned Land** | Major | 3 | Slow | Earth, Plant, Animal | 1 Fear. 7 Damage. Add 1 Blight. Destroy all Dahan. |
| **Savage Mawbeasts** | Minor | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Pillar of Living Flame** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Tsunami** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
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


### Strategy Cliffs — per-adversary-level shifts that change Devouring Teeth Lurk Underfoot's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Devouring Teeth Lurk Underfoot's profile (Fear 2, Offense 5, Control 2, Defense 1, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Devouring Teeth Lurk Underfoot**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Devouring Teeth Lurk Underfoot**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Devouring Teeth Lurk Underfoot**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/devouring-teeth-lurk-underfoot.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Devouring Teeth Lurk Underfoot](https://spiritislandwiki.com/index.php?title=Devouring_Teeth_Lurk_Underfoot).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
