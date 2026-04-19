# Thunderspeaker

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 5 · Fear 3 · Defense 2 · Utility 1             |
| Primary Elements      | Air, Sun, Fire, Animal (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Has a keen interest in where the Dahan are - partly because so many of its starting powers work through them partly because its Presence can move along with them. When picking new Power Cards, it will often want to take good Dahan-centric Powers, but it can also branch out into other areas.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board: 1 in each of the 2 lands with the most Dahan.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=gain1p |
| G2 | first=Thunder2, second=Thunder1 |
| G3 | first=addpresence1, second=energy4 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, air, energy2, fire, sun, energy3
- **Card-play track**: card1, card2, card2, card3, reclaim1, card3, card4

## Core Mechanics & Special Rules

### Special Rule

ALLY OF THE DAHAN Your Presence may move with Dahan. (Whenever a Dahan moves from 1 of your lands to another land, you may move 1 Presence along with it.) SWORN TO VICTORY After a Ravage Action destroys 1 or more Dahan, for each Dahan Destroyed, Destroy 1 of your Presence within Range 1.

### Innate: GATHER THE WARRIORS

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 Air | This Power may be Fast. |
| 2 | 1 Animal | Gather up to 1 Dahan per Air you have. Push up to 1 Dahan per Sun you have. |


### Innate: LEAD THE FURIOUS ASSAULT

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 Air | This Power may be Fast. |
| 2 | 2 Sun + 1 Fire | Destroy 1 Town for every 2 Dahan in target land. |
| 3 | 4 Sun + 3 Fire | Destroy 1 City for every 3 Dahan in target land. |


## Unique Cards (all, Wiki-verified)

#### Manifestation of Power and Glory

- **3 Energy · Slow · Range 0 · Land with Dahan · Sun, Fire, Air**
- *1 Fear. Each Dahan deals Damage equal to the number of your Presence in target land.*

#### Sudden Ambush

- **2 Energy · Fast · Range 1 · Any Land · Fire, Air, Animal**
- *You may Gather 1 Dahan. Each Dahan Destroys 1 Explorer.*

#### Voice of Thunder

- **0 Energy · Slow · Range 1 · Any Land · Sun, Air**
- *Push up to 4 Dahan. **OR** If Invaders are present, 2 Fear.*

#### Words of Warning

- **1 Energy · Fast · Range 1 · Land with Dahan · Sun, Air, Animal**
- *Defend 3. During Ravage, Dahan in target land deal Damage simultaneously with Invaders.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Thunderspeaker's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/thunderspeaker.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/thunderspeaker.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 7.2), **Sun** (wt 2.4), **Fire** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 4 · Control 5 · Fear 3 · Defense 2 · Utility 1
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Manifestation of Power and Glory**
- **Sudden Ambush**
- **Voice of Thunder**
- **Words of Warning**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire+sun → 11.1 |
| 2 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements air+sun → 9.6 |
| 3 | **Birds Cry Warning** | 1 | Fast | Sun, Air, Animal | The next time Dahan would be Destroyed in target land, Destroy 2 fewer Dahan. **OR** Push… | elements air+animal+sun → 10.2 |
| 4 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements air+sun → 9.6 |
| 5 | **Call to Isolation** | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. | elements air+animal+sun → 10.2 |
| 6 | **Fire in the Sky** | 1 | Fast | Sun, Fire, Air | 2 Fear. Add 1 Strife. | elements air+fire+sun → 11.1 |
| 7 | **Spur on with Words of Fire** | 1 | Fast | Sun, Fire, Air | If you target a Spirit other than yourself, they gain +1 Energy. Target Spirit may immedi… | elements air+fire+sun → 11.1 |
| 8 | **Enticing Splendor** | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer/Town. **OR** Gather up to 2 Dahan. | elements air+sun → 9.6 |
| 9 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. | elements air+sun → 9.6 |
| 10 | **Call to Migrate** | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. | elements air+animal+fire → 9.3 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Instruments of Their Own Ruin** | 4 | Fast | Sun, Fire, Air, Animal | Add 1 Strife. Each Invader with Strife deals Damage to other Invaders in target land. | elements air+animal+fire+sun → 11.7 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+fire+sun → 11.7 |
| 3 | **Sweep into the Sea** | 4 | Slow | Sun, Air, Water | Push all Explorers and Towns one land towards the nearest Ocean. **OR** If target land is… | elements air+sun → 9.6 |
| 4 | **Powerstorm** | 3 | Fast | Sun, Fire, Air | Target Spirit gains 3 Energy. Once this turn, target Spirit may Repeat a Power Card by pa… | elements air+fire+sun → 11.1 |
| 5 | **Voice of Command** | 3 | Fast | Sun, Air | 1 Damage per Dahan/Explorer, to Towns/Cities only. Defend 2. During Ravage Actions, Explo… | elements air+sun → 9.6 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Thunderspeaker in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Renewing Boon** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Tsunami** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/thunderspeaker.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Thunderspeaker](https://spiritislandwiki.com/index.php?title=Thunderspeaker).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
