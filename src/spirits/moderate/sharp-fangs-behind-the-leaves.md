# Sharp Fangs Behind the Leaves

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
| Power summary (1–5)   | Offense 3 · Control 3 · Fear 4 · Defense 2 · Utility 1             |
| Primary Elements      | Animal, Moon, Plant, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> All about Beasts and Jungles. Can be very fast out of the gate, but doesn't have the late-game power that some spirits do, and is likely to have some difficulty with Blighted areas. "Ranging Hunt" is a critical Innate ability, particularly in early-game: it simultaneously gives Beasts mobility and permits picking off a stray Explorers or Towns on most turns.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Beast on your starting board in the highest-numbered Jungle. Put 1 Presence in a land of your choice with Beast anywhere on the island.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=Sharp1, second=gain1p |
| G2 | first=Sharp |
| G3 | first=gain1p, second=energy1 |
| G4 | first=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, animal, plant, energy2, animal, energy3, energy4
- **Card-play track**: card2, card2, card3, reclaim1, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

ALLY OF THE BEASTS Your Presence may move with Beast. (Whenever a Beast moves from 1 of your lands to another land, you may move 1 Presence along with it.) CALL FORTH PREDATORS During each Spirit Phase, you may replace 1 of your Presence with 1 Beast. The replaced Presence leaves the game. (It was not destroyed, so things which return destroyed Presence cannot bring it back.)

### Innate: RANGING HUNT

- **Speed**: fast · **Range**: 1 · **Target**: noblight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Animal | You may Gather 1 Beast. |
| 2 | 2 Plant + 3 Animal | 1 Damage per Beast. |
| 3 | 2 Animal | You may Push up to 2 Beast. |


### Innate: FRENZIED ASSAULT

- **Speed**: slow · **Range**: 1 · **Target**: beast

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Fire + 4 Animal | 1 Fear and 2 Damage. Remove 1 Beast. |
| 2 | 1 Moon + 2 Fire + 5 Animal | +1 Fear and +1 Damage. |


## Unique Cards (all, Wiki-verified)

#### Prey on the Builders

- **1 Energy · Fast · Range 0 · Any Land · Moon, Fire, Animal**
- *You may Gather 1 Beasts. If target land has Beasts, Invaders do not Build there this turn.*

#### Teeth Gleam from Darkness

- **1 Energy · Slow · Range 1, from a Jungle · Land with no Blight · Moon, Plant, Animal**
- *1 Fear. Add 1 Beasts. **OR** If target land has both Beasts and Invaders: 3 Fear.*

#### Terrifying Chase

- **1 Energy · Slow · Range 0 · Any Land · Sun, Animal**
- *Push 2 Explorers/Towns/Dahan. Push another 2 Explorers/Towns/Dahan per Beasts in target land. If you Pushed any Invaders, 2 Fear.*

#### Too Near the Jungle

- **0 Energy · Slow · Range 1, from a Jungle · Any Land · Plant, Animal**
- *1 Fear. Destroy 1 Explorer.*


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/sharp-fangs-behind-the-leaves.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Sharp Fangs Behind the Leaves](https://spiritislandwiki.com/index.php?title=Sharp_Fangs_Behind_the_Leaves).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
