# Lightning's Swift Strike

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
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 3 · Defense 1 · Utility 2             |
| Primary Elements      | Fire, Air, Water (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Virtually all offense to start with: without a more defensive teammate, Blight may become a problem. Excellent at destroying buildings, less good at containing Explorers. Using Thundering Destruction tends to be a burst affair: a turn or two of position and build up Energy, followed by a really big turn. Starting Powers are extremely focused on Air and Fire: good for Thundering Destruction, bad for Major Power versatility.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board in the highest-numbered Sands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence2, second=addpresence0 |
| G3 | first=addpresence1, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy1, energy2, energy2, energy3, energy4, energy4, energy5
- **Card-play track**: card2, card3, card4, card5, card6

## Core Mechanics & Special Rules

### Special Rule

SWIFTNESS OF LIGHTNING For every Air you have, you may use 1 Slow Power as if it were Fast. (Power Cards or your Innate Powers.)

### Innate: THUNDERING DESTRUCTION

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Fire + 2 Air | Destroy 1 Town. |
| 2 | 4 Fire + 3 Air | You may instead destroy 1 City. |
| 3 | 5 Fire + 4 Air + 1 Water | Also, Destroy 1 Town / City. |
| 4 | 5 Fire + 5 Air + 2 Water | Also, Destroy 1 Town / City. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Lightning's Swift Strike's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/lightning-swift-strike.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/lightning-swift-strike.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Fire** (wt 8.1), **Air** (wt 6.3), **Water** (wt 0.9)
- **Mid-game energy estimate (T3–T5 avg)**: 2.33E
- **Power summary**: Offense 5 · Control 2 · Fear 3 · Defense 1 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer. | elements air+fire+water → 15.3 |
| 2 | **Dry Wood Explodes in Smoldering Splinters** | 1 | Slow | Fire, Air, Plant | You may spend 1 Energy to make this Power Fast. 2 Fear. 1 Damage. | elements air+fire → 14.4 |
| 3 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear. Add 1 Disease. | elements air+fire+water → 15.3 |
| 4 | **Desiccating Winds** | 1 | Slow | Fire, Air, Earth | If target land has Badlands, 1 Damage. Add 1 Badlands. | elements air+fire → 14.4 |
| 5 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire → 14.4 |
| 6 | **Call to Migrate** | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. | elements air+fire → 14.4 |
| 7 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+fire → 14.4 |
| 8 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+fire → 14.4 |
| 9 | **Swarming Wasps** | 0 | Fast | Fire, Air, Animal | Add 1 Beasts. **OR** If target land has Beasts, Push up to 2 Explorers. | elements air+fire → 14.4 |
| 10 | **Fire in the Sky** | 1 | Fast | Sun, Fire, Air | 2 Fear. Add 1 Strife. | elements air+fire → 14.4 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Storm-Swath** | 3 | Slow | Fire, Air, Water | 2 Fear. In both origin land and target land: 1 Damage to each Invader. | elements air+fire+water → 15.3 |
| 2 | **Plague Ships Sail to Distant Ports** | 4 | Fast | Fire, Air, Water, Animal | 1 Fear. Add 4 Disease among Coastal lands (on any boards) other than target land. | elements air+fire+water → 15.3 |
| 3 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+fire+water → 15.3 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+fire+water → 15.3 |
| 5 | **Exaltation of the Incandescent Sky** | 7 | Fast | Sun, Fire, Air, Water | Target Spirit may play 1 Power Card by paying its cost, make up to 2 of their Powers Fast… | elements air+fire+water → 15.3 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Lightning's Swift Strike in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Delusions of Danger** | Minor | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. |
| **Call to Bloodshed** | Minor | 1 | Slow | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan. |
| **Powerstorm** | Major | 3 | Fast | Sun, Fire, Air | Target Spirit gains 3 Energy. Once this turn, target Spirit may Repeat a Power Card by pa… |
| **Purifying Flame** | Minor | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… |
| **Pillar of Living Flame** | Major | 5 | Slow | Fire | 3 Fear. 5 Damage. If target land is a Jungle or Wetland, add 1 Blight. |
| **Entrancing Apparitions** | Minor | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers. |
| **Call to Isolation** | Minor | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Renewing Boon** | destroys Presence |
| **Skies Herald the Season of Return** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Tsunami** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/lightnings-swift-strike.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Lightning's Swift Strike](https://spiritislandwiki.com/index.php?title=Lightning's_Swift_Strike).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
