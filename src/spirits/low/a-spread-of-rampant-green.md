# A Spread of Rampant Green

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Moderate                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "always" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 3 · Fear 2 · Defense 5 · Utility 4             |
| Primary Elements      | Plant, Moon, Water, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Fairly good at dealing with Towns, but terrible at handling Explorers (who are unfazed by prolific foliage). Can get Presence onto the board faster than most other Spirits. Extra Presence is good for targeting and especially for 'Choke the Land with Green", which can be extremely effective at slowing down invaders. Just be careful not to destroy Sacred Sites needed for Power use.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board; 1 in the highest-numbered Wetland, and 1 in the Jungle without any Dahan. (If there is more than 1 such Jungle, you may choose)

## Growth Options (always)

| Growth | Effects |
|--------|---------|
| G1 | first=Spread |
| G2 | first=reclaim, second=gain1p |
| G3 | first=addpresence1, second=old+1cardplay |
| G4 | first=gain1p, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, plant, energy2, energy2, plant, energy3
- **Card-play track**: card1, card1, card2, card2, card3, card4

## Core Mechanics & Special Rules

### Special Rule

CHOKE THE LAND WITH GREEN Whenever Invaders would Ravage or Build in a land with your {{sacredsite}}, you may prevent it by destroying one of your Presence in that land. STEADY REGENERATION When adding Presence to the board via Growth, you may optionally use your destroyed Presence. If the island is Healthy, do so freely. If the island is Blighted, doing so costs 1 Energy per destroyed Presence you add.

### Innate: CREEPERS TEAR INTO MORTAR

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 2 Plant | 1 Damage to 1 Town / City. |
| 2 | 2 Moon + 3 Plant | Repeat this Power. |
| 3 | 3 Moon + 4 Plant | Repeat this Power again. |


### Innate: ALL-ENVELOPING GREEN

- **Speed**: fast · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Water + 3 Plant | Defend 2. |
| 2 | 2 Water + 4 Plant | Instead, Defend 4. |
| 3 | 3 Water + 1 Earth + 5 Plant | Also, remove 1 Blight. |


## Unique Cards (all, Wiki-verified)

#### Fields Choked with Growth

- **0 Energy · Slow · Range 1 · Any Land · Sun, Water, Plant**
- *Push 1 Town. **OR** Push 3 Dahan.*

#### Gift of Proliferation

- **1 Energy · Fast · Range No Range · Another Spirit · Moon, Plant**
- *Target Spirit adds 1 Presence up to 1 Range from their Presence.*

#### Overgrow in a Night

- **2 Energy · Fast · Range 1 · Any Land · Moon, Plant**
- *Add 1 Presence. **OR** If target land has your Presence and Invaders, 3 Fear.*

#### Stem the Flow of Fresh Water

- **0 Energy · Slow · Range 1, from your Sacred Site · Any Land · Water, Plant**
- *1 Damage to 1 Town/City. If target land is a Mountain or Sands, instead, 1 Damage to each Town/City.*


## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

_(none in Wiki's suggested list)_

### Major Powers

_(none in Wiki's suggested list)_



## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opening Strategy

`[VERIFY: needs play data]` — opening variants should be rehearsed turn-by-turn per the [SPIRIT_TEMPLATE.md](../../../templates/SPIRIT_TEMPLATE.md) opener format.

## Card Priority Ratings

**Uniques**: see above, all 4 cards are starting-deck and usually all grade A-tier for the spirit's intended playstyle.

**Suggested Minors + Majors**: see tables above. Wiki's suggestions reflect community-recommended drafts.

`[VERIFY: ratings per matchup pending]`.

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

```admonish note title="Stat Insight `[VERIFY from mindwanderer]`"
Pending re-scrape of mindwanderer current data. Historical directional figures unavailable in this template draft.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/a-spread-of-rampant-green.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [A Spread of Rampant Green](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
