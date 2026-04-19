# Keeper of the Forbidden Wilds

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch and Claw                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "two" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 1 · Defense 4 · Utility 3             |
| Primary Elements      | Plant, Sun, Fire, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> A slowly growing wall - expanding can sometimes be difficult, but the Invaders will have an equally difficult time penetrating wherever the Keeper plants itself. In larger games, it may be useful to spread to one of the two far-distant lands early on, to have multiple points from which to slowly grow.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Wilds on your starting board in the highest-numbered Jungle.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=energy1 |
| G2 | first=gain1p |
| G3 | first=Keeper, second=energy1 |
| G4 | first=Keeper3, second=noblight |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, sun, energy4, energy5, plant, energy7, energy8, energy9
- **Card-play track**: card1, card2, card2, card3, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

FORBIDDEN GROUND After you create a Sacred Site, Push all Dahan from that land. Dahan Events never move Dahan to your Sacred Site, but Powers can do so.

### Innate: PUNISH THOSE WHO TRESPASS

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Sun + 1 Fire + 2 Plant | 2 Damage. Destroy 1 Dahan. |
| 2 | 2 Sun + 2 Fire + 3 Plant | +1 Damage per SunPlant you have. |
| 3 | 4 Plant | Split this Power's Damage however desired between target land and another 1 of your lands. |


### Innate: SPREADING WILDS

- **Speed**: slow · **Range**: 1 · **Target**: noblight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Sun | Push 1 Explorer from target land per 2 Sun you have. |
| 2 | 1 Plant | If target land has no Explorer, add 1 Wilds. |
| 3 | 3 Plant | This Power has Range +1. |
| 4 | 1 Air | This Power has Range +1. |


## Unique Cards (all, Wiki-verified)

#### Boon of Growing Power

- **1 Energy · Slow · Range No Range · Any Spirit · Sun, Moon, Plant**
- *Target Spirit gains a Power Card. If you target another Spirit, they also gain 1 Energy.*

#### Regrow from Roots

- **1 Energy · Slow · Range 1 · Jungle or Wetland · Water, Earth, Plant**
- *If there are 2 Blight or fewer in target land, Remove 1 Blight.*

#### Sacrosanct Wilderness

- **2 Energy · Fast · Range 1 · Land with no Blight · Sun, Earth, Plant**
- *Push 2 Dahan. 2 Damage per Wilds in target land. **OR** Add 1 Wilds.*

#### Towering Wrath

- **3 Energy · Slow · Range 1, from your Sacred Site · Any Land · Sun, Fire, Plant**
- *2 Fear. For each of your Sacred Site in/adjacent to target land, 2 Damage. Destroy all Dahan.*


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/keeper-of-the-forbidden-wilds.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Keeper of the Forbidden Wilds](https://spiritislandwiki.com/index.php?title=Keeper_of_the_Forbidden_Wilds).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
