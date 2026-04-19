# Vital Strength of the Earth

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3             |
| Primary Elements      | Earth, Plant, Sun, Animal (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Powerful but slow: has potent Power Cards and an excellent Energy income, but starts wtih only one card play per turn, and Growth is limited to adding one Presence per turn. Also slow to change: learning new Powers carries slightly more cost than reclaiming played Power Cards

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 2 in the highest-numbered Mountain, 1 in the highest-numbered Jungle.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence2 |
| G2 | first=gain1p, second=addpresence0 |
| G3 | first=addpresence1, second=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, energy3, energy4, energy6, energy7, energy8
- **Card-play track**: card1, card1, card2, card2, card3, card4

## Core Mechanics & Special Rules

### Special Rule

EARTH'S VITALITY Defend 3 in every land where you have Sacred Site.

### Innate: GIFT OF STRENGTH

- **Speed**: fast · **Range**: None · **Target**: anyspirit

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 2 Earth + 2 Plant | Once this turn, Target Spirit may Repeat 1 Power Card with Energy cost of 1 or less. |
| 2 | 2 Sun + 3 Earth + 2 Plant | Instead, the Energy cost limit is 3 or less. |
| 3 | 2 Sun + 4 Earth + 3 Plant | Instead, the Energy cost limit is 6 or less. |


## Unique Cards (all, Wiki-verified)

#### A Year of Perfect Stillness

- **3 Energy · Fast · Range 1 · Any Land · Sun, Earth**
- *Invaders skip all Actions in target land this turn.*

#### Draw of the Fruitful Earth

- **1 Energy · Slow · Range 1 · Any Land · Earth, Plant, Animal**
- *Gather up to 2 [[Explorers]]. Gather up to 2 [[Dahan]].*

#### Guard the Healing Land

- **3 Energy · Fast · Range 1, from your Sacred Site · Any Land · Water, Earth, Plant**
- *Remove 1 Blight. Defend 4.*

#### Rituals of Destruction

- **3 Energy · Slow · Range 1, from your Sacred Site · Land with Dahan · Sun, Moon, Fire, Earth, Plant**
- *2 Damage. If target land has at least 3 Dahan, +3 Damage and 2 Fear.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Vital Strength of the Earth's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/vital-strength-of-the-earth.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/vital-strength-of-the-earth.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Earth** (wt 4.8), **Plant** (wt 3.9), **Sun** (wt 2.7)
- **Mid-game energy estimate (T3–T5 avg)**: 5.67E
- **Power summary**: Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **A Year of Perfect Stillness**
- **Draw of the Fruitful Earth**
- **Guard the Healing Land**
- **Rituals of Destruction**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Absorb Corruption** | 1 | Slow | Sun, Earth, Plant | Gather 1 Blight. **OR** Pay 1 Energy to Remove 1 Blight. | elements earth+plant+sun → 11.4 |
| 2 | **Carapaced Land** | 0 | Fast | Earth, Plant, Animal | If targeting a land with Beasts, this Power has +1 Range. Defend 3. | elements earth+plant → 8.7 |
| 3 | **Drift Down into Slumber** | 0 | Fast | Air, Earth, Plant | Defend 1. If target land is a Jungle or Sands, instead Defend 4. | elements earth+plant → 8.7 |
| 4 | **Entrap the Forces of Corruption** | 1 | Fast | Earth, Plant, Animal | Gather up to 1 Blight. Isolate target land. When Blight is added to target land, it doesn… | elements earth+plant → 8.7 |
| 5 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear. If target land is a Mountain or Jungle, Defend 3. | elements earth+plant → 8.7 |
| 6 | **Nature's Resilience** | 1 | Fast | Earth, Plant, Animal | Defend 6. | elements earth+plant → 8.7 |
| 7 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements earth+sun → 7.5 |
| 8 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements earth+plant → 8.7 |
| 9 | **Call to Trade** | 1 | Fast | Air, Water, Earth, Plant | You may Gather 1 Dahan. If the Terror Level is 2 or lower, Gather 1 Town and the first Ra… | elements earth+plant → 8.7 |
| 10 | **Renewing Boon** | 1 | Slow | Sun, Earth, Plant | Choose a land where you and target Spirit both have Presence. In that land: Remove 1 Blig… | elements earth+plant+sun → 11.4 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **The Trees and Stones Speak of War** | 2 | Fast | Sun, Earth, Plant | For each Dahan, 1 Damage and Defend 2. | elements earth+plant+sun → 11.4 |
| 2 | **Walls of Rock and Thorn** | 4 | Fast | Sun, Earth, Plant | 2 Damage. Defend 8. Add 1 Wilds. Isolate target land. | elements earth+plant+sun → 11.4 |
| 3 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements earth+plant+sun → 11.4 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements earth+plant+sun → 11.4 |
| 5 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements earth+plant+sun → 11.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Vital Strength of the Earth in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Rouse the Trees and Stones** | Minor | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. |
| **Call to Migrate** | Minor | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. |
| **Poisoned Land** | Major | 3 | Slow | Earth, Plant, Animal | 1 Fear. 7 Damage. Add 1 Blight. Destroy all Dahan. |
| **Devouring Ants** | Minor | 1 | Slow | Sun, Earth, Animal | 1 Fear. 1 Damage. Destroy 1 Dahan. If target land is a Jungle or Sands, +1 Damage. |
| **Vigor of the Breaking Dawn** | Major | 3 | Fast | Sun, Animal | 2 Damage per Dahan. |
| **Voracious Growth** | Minor | 1 | Slow | Water, Plant | 2 Damage. **OR** Remove 1 Blight. |
| **Savage Mawbeasts** | Minor | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Renewing Boon** | destroys Presence |
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Scour the Land** | adds Blight |
| **Land of Haunts and Embers** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Tsunami** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
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


### Strategy Cliffs — per-adversary-level shifts that change Vital Strength of the Earth's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Vital Strength of the Earth's profile (Fear 1, Offense 2, Control 3, Defense 5, Utility 3).
```

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Vital Strength of the Earth**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Vital Strength of the Earth**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/vital-strength-of-the-earth.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Vital Strength of the Earth](https://spiritislandwiki.com/index.php?title=Vital_Strength_of_the_Earth).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
