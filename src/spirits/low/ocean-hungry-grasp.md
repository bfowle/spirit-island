# Ocean's Hungry Grasp

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | High                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 4 · Fear 4 · Defense 3 · Utility 2             |
| Primary Elements      | Water, Moon, Earth, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Extremely good at assaulting the coasts where the Invaders start out strong, but quite weak inland - the ocean is not accustomed to affecting events so far ashore. Its Presence shifts in and out like the tide, which can be tricky to manage, but permits re-positioning and tactical retreats or offensives in the hands of a skillful player. Has fairly inexpensive Unique Powers, but the energy gained from drowning Invaders can be necessary in stepping up to more potent Powers.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence onto your starting board: 1 in the Ocean, and 1 in a Coastal land of your choice.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=OceanG |
| G2 | first=OceanA, second=OceanA, third=energy1 |
| G3 | first=gain1p, second=Ocean, third=OceanP |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, moon, water, energy1, earth, water, energy2
- **Card-play track**: card1, card2, card2, card3, card4, card5

## Core Mechanics & Special Rules

### Special Rule

OCEAN IN PLAY You may add/move Presence into Oceans, but may not add/move Presence into Inland lands. On boards where you have 1 or more Presence, Oceans are treated as Coastal Wetlands for Spirit Powers and Blight. You Drown any Invaders or Dahan moved to those Oceans. DROWNING Destroy Drowned pieces, placing Drowned Invaders here. At any time you may exchange (X) Health of these Invaders for 1 Energy, where X = number of players. (Ignore modifiers to Invader Health.)

### Innate: POUND SHIPS TO SPLINTERS

- **Speed**: fast · **Range**: 0 · **Target**: coastal

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Air + 2 Water | 1 Fear. |
| 2 | 2 Moon + 1 Air + 3 Water | +1 Fear. |
| 3 | 3 Moon + 2 Air + 4 Water | +2 Fear. |


### Innate: OCEAN BREAKS THE SHORE

- **Speed**: slow · **Range**: 0 · **Target**: coastal

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Water + 1 Earth | Drown 1 Town. |
| 2 | 3 Water + 2 Earth | You may instead Drown 1 City. |
| 3 | 4 Water + 3 Earth | Also, Drown 1 Town / City. |


## Unique Cards (all, Wiki-verified)

#### Call of the Deeps

- **0 Energy · Fast · Range 0 · Coastal Land · Moon, Air, Water**
- *Gather 1 Explorer. If target land is the Ocean, you may Gather another Explorer.*

#### Grasping Tide

- **1 Energy · Fast · Range 1 · Coastal Land · Moon, Water**
- *2 Fear. Defend 4.*

#### Swallow the Land-Dwellers

- **0 Energy · Slow · Range 0 · Coastal Land · Water, Earth**
- *Drown 1 Explorer, 1 Town, and 1 Dahan.*

#### Tidal Boon

- **1 Energy · Slow · Range No Range · Another Spirit · Moon, Water, Earth**
- *Target Spirit gains 2 Energy and may Push 1 Town and up to 2 Dahan from one of their lands. If Dahan are pushed to your Ocean, you may move them to any Coastal land instead of Drowning them.*


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/oceans-hungry-grasp.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Ocean's Hungry Grasp](https://spiritislandwiki.com/index.php?title=Ocean's_Hungry_Grasp).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
