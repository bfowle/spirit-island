# Heart of the Wildfire

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 1                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 3 · Fear 4 · Defense 1 · Utility 2             |
| Primary Elements      | Fire, Plant, Air, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Starts with good offense and gets better from there, but lays down Blight as it grows. The smaller the game, the more restraint is needed to prevent tipping the island over into being completely Blighted. The Wildfire can heal the land where it is, but may benefit from other Blight removal Powers so it can add Presence to problem lands without triggering Blight cascade. Removing Blight from its own lands limits its "Firestorm" innate power, however. In the Reprint, the complexity was changed from Moderate to High

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence and 2 Blight on your starting board in the highest-numbered Sands. (Blight comes from the box, not the Blight Card)

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=gain1p, second=addpresence3 |
| G3 | first=addpresence1, second=energy2, third=Wildfire |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, fire, energy1, energy2, fireplant, energy3
- **Card-play track**: card1, fireX, card2, card3, fireX, card4

## Core Mechanics & Special Rules

### Special Rule

BLAZING PRESENCE Post-Setup, after your Presence is added/moved, in the land it goes to: * For each Simplefire showing on your Presence Tracks, do 1 Damage. * If 2 Simplefire or more are showing on your Presence Tracks, add 1 Blight. * Push all Beast and any number of Dahan. If you add multiple Presence into a land at the same time, only do the above effects once. DESTRUCTIVE NATURE Blight added due to Spirit effects (Powers, Special Rules, Scenario-based Rituals, etc) does not destroy your Presence. (This includes cascades.)

### Innate: FIRESTORM

- **Speed**: fast · **Range**: 0 · **Target**: blight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Plant | 1 Damage per 2 Fire you have. |
| 2 | 3 Plant | Instead, 1 Damage per Fire you have. |
| 3 | 4 Fire + 2 Air | Split this Power's Damage however desired between target land and any number of your lands with Blight. |
| 4 | 7 Fire | In a land with Blight where you have Presence, Push all Dahan. Destroy all Invaders and Beast. Add 1 Blight. |


### Innate: THE BURNED LAND REGROWS

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 Fire + 1 Plant | If target land has 2 Blight or more, remove 1 Blight |
| 2 | 4 Fire + 2 Plant | Instead, remove 1 Blight. |
| 3 | 5 Fire + 2 Earth + 2 Plant | Remove another Blight. |


## Unique Cards (all, Wiki-verified)

#### Asphyxiating Smoke

- **2 Energy · Slow · Range 2, from your Sacred Site · Any Land · Fire, Air, Plant**
- *1 Fear. Destroy 1 Town. Push 1 Dahan.*

#### Flame's Fury

- **0 Energy · Fast · Range No Range · Any Spirit · Sun, Fire, Plant**
- *Target Spirit gains 1 Energy. Target Spirit does +1 Damage with each Damage dealing Power they use this turn. (Powers which Damage multiple lands or each Invader only get 1 extra Damage total. Repeated Powers keep the +1 boost. Destroy effects don't get any bonus.)*

#### Flash-Fires

- **2 Energy · Slow · Range 1 · Any Land · Fire, Air**
- *1 Fear. 1 Damage.*

- **Threshold**: 2 Air — 2 Air: This Power is Fast.

#### Threatening Flames

- **0 Energy · Fast · Range 0 · Land with 1 or more Blight and 1 or more Invaders · Fire, Plant**
- *2 Fear. Push 1 Explorer/Town per Terror Level from target land to adjacent lands without your Presence. If there are no such adjacent lands, +2 Fear.*


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/heart-of-the-wildfire.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Heart of the Wildfire](https://spiritislandwiki.com/index.php?title=Heart_of_the_Wildfire).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
