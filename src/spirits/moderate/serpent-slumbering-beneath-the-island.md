# Serpent Slumbering Beneath the Island

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 1                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "two" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 2 · Fear 2 · Defense 4 · Utility 5             |
| Primary Elements      | Earth, Fire, Water, Plant (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> There are several ways to play the Serpent, but all require patience: early game involves slowly building up Powers and Presence. It's not helpless during this time, but it isn't as effective as anyone else. It becomes incredibly powerful after awakening, but getting there requires a lot of time. Make sure to Absorb Essence before you run up against your Presence cap - and to get other players' buy-in before using Absorb Essence on their Presence.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the land #5.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=movepresence1 |
| G2 | first=gain1p, second=energy1 |
| G3 | first=energy4 |
| G4 | first=noblight |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, fire, any, reclaim1E, earth, energy6, any, energy12
- **Card-play track**: card1, moonX, card2, waterX, serpent, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

DEEP SLUMBER You start off limited to 5 Presence on the island. Raise this with your "Absorb Essence" Power Card. Each use covers the lowest revealed number; your Presence limit is the lowest uncovered number. <div id="presence"><div class="option" style="line-height: 95%; font-size:150%;"> 5</div><div class="option" style="line-height: 95%; font-size:150%;"> 7</div><div class="option" style="line-height: 95%; font-size:150%;"> 8</div> <div class="option" style="line-height: 95%; font-size:150%;"> 10</div><div class="option" style="line-height: 95%; font-size:150%;"> 11</div><div class="option" style="line-height: 95%; font-size:150%;"> 12</div><div class="option" style="border: 0px !important; line-height: 95%; font-size:150%;"> 13</div></div>

### Innate: SERPENT WAKES IN POWER

- **Speed**: slow · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Fire + 1 Water + 1 Plant | Gain 1 Energy. Other spirits with any Absorbed Presence also gain 1 Energy. |
| 2 | 2 Water + 3 Earth + 2 Plant | Add 1 Presence to Range 1. Other spirits with 2 or more Absorbed Presence may do likewise. |
| 3 | 3 Fire + 3 Water + 3 Earth + 3 Plant | Gain a Major Power without Forgetting. Other Spirits with 3 or more Absorbed Presence may do likewise. |


### Innate: SERPENT ROUSES IN ANGER

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Fire + 1 Earth | For each Fire Earth you have, 1 Damage to 1 Town / City. |
| 2 | 2 Moon + 2 Earth | For each 2 Moon 2 Earth you have, 2 Fear and you may Push 1 Town from target land. |
| 3 | 5 Moon + 6 Fire + 6 Earth | Cost 7 In every land in the game: X Damage, where X is the number of Presence you have in and adjacent to that land. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Serpent Slumbering Beneath the Island's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/serpent-slumbering-beneath-the-island.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/serpent-slumbering-beneath-the-island.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Earth** (wt 6.6), **Fire** (wt 5.4), **Water** (wt 3.0)
- **Mid-game energy estimate (T3–T5 avg)**: 12.0E
- **Power summary**: Offense 2 · Control 2 · Fear 2 · Defense 4 · Utility 5
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **The Shore Seethes with Hatred** | 1 | Slow | Fire, Water, Earth, Plant | 1 Fear. Add 1 Badlands and 1 Wilds. | elements earth+fire+plant+water → 18.0 |
| 2 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon+plant+water → 15.3 |
| 3 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements earth+fire+plant → 15.0 |
| 4 | **Rouse the Trees and Stones** | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. | elements earth+fire+plant → 15.0 |
| 5 | **Treacherous Waterways** | 0 | Fast | Fire, Water, Earth | Add 1 Wilds. **OR** Push 1 Explorer. | elements earth+fire+water → 15.0 |
| 6 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements earth+fire+moon → 14.7 |
| 7 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements fire+moon+plant+water → 14.1 |
| 8 | **Unquenchable Flames** | 1 | Slow | Moon, Fire, Earth | 1 Fear. 1 Damage to Towns/Cities. Invaders do not heal Damage at end of turn. | elements earth+fire+moon → 14.7 |
| 9 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer. | elements earth+fire+water → 15.0 |
| 10 | **Rites of the Land's Rejection** | 1 | Fast | Moon, Fire, Earth | Invaders do not Build in target land this turn. 1 Fear per Town/City or 1 Fear per Dahan,… | elements earth+fire+moon → 14.7 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements earth+fire+moon+plant+water → 20.7 |
| 2 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements earth+fire+moon+plant → 17.7 |
| 3 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements earth+fire+plant → 15.0 |
| 4 | **The Land Thrashes in Furious Pain** | 4 | Slow | Moon, Fire, Earth | 2 Damage per Blight. For each Blight in adjacent lands, 1 Damage (in target land). | elements earth+fire+moon → 14.7 |
| 5 | **Dream of the Untouched Land** | 6 | Fast | Moon, Water, Earth, Plant, Animal | Remove up to 3 Blight and up to 3 Health worth of Invaders. | elements earth+moon+plant+water → 15.3 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Serpent Slumbering Beneath the Island in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Renewing Boon** | destroys Presence |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Blazing Renewal** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Insatiable Hunger of the Swarm** | adds Blight |
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


### Strategy Cliffs — per-adversary-level shifts that change Serpent Slumbering Beneath the Island's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Serpent Slumbering Beneath the Island's profile (Fear 2, Offense 2, Control 2, Defense 4, Utility 5).
```

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Serpent Slumbering Beneath the Island**: Front-load coastal defense or disruption before T3's first Ravage.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/serpent-slumbering-beneath-the-island.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Serpent Slumbering Beneath the Island](https://spiritislandwiki.com/index.php?title=Serpent_Slumbering_Beneath_the_Island).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
