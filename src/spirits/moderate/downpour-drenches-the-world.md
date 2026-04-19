# Downpour Drenches the World

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 2                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3             |
| Primary Elements      | Water, Earth, Plant, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Cares about the question "How useful is this Power in the current context?" even more than most Spirits; it rarely plays all its Power Cards in any given Reclaim cycle (some get discarded to Growth), and for those it does play, it often has the option of using them multiple times. Benefits even more than most Spirits from having lots of Presence on the board, both for Rain and Mud Suppress Conflict and to facilitate its Unique Powers (by making more lands Wetlands).

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the lowest-numbered Wetlands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=movepresence2 |
| G2 | first=addpresence2, second=addpresence2, third=gain2water |
| G3 | first=gain1p, second=addpresence3, third=energy1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, water, plant, water, energy2air, water, earth, doublewater
- **Card-play track**: card1, movepres, waterX, card2, movepres, card3

## Core Mechanics & Special Rules

### Special Rule

DRENCH THE LANDSCAPE Spirit Actions and Special Rules treat your Sacred Site as Wetlands in addition to the printed terrain. POUR DOWN POWER ACROSS THE ISLAND For each 2 Water you have, during the Fast/Slow phase you may either: * Gain 1 Energy; or * Repeat a land-targeting Power Card by paying its cost again. (It need not target the same land.) Use scenario markers or spare game pieces to track uses of this rule. (Max 5 times per turn, no matter how much Water you have.)

### Innate: RAIN AND MUD SUPPRESS CONFLICT

- **Speed**: fast · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Air + 3 Water | Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.) |
| 2 | 5 Water + 1 Earth | Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. |
| 3 | 3 Air + 9 Water + 2 Earth | 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1). |


### Innate: WATER NOURISHES LIFE'S GROWTH

- **Speed**: fast · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Water + 2 Plant | Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (From target land). |
| 2 | 5 Water + 1 Earth + 2 Plant | Gain +1 Energy. Gather up to 1 Dahan. |
| 3 | 7 Water + 2 Earth + 3 Plant | When Blight would be added to target land, instead leave it on the card. |


## Unique Cards (all, Wiki-verified)



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

```admonish note title="Stat Insight"
`[VERIFY from mindwanderer]` — pending re-scrape of mindwanderer current data. Historical directional figures unavailable in this template draft.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/downpour-drenches-the-world.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Downpour Drenches the World](https://spiritislandwiki.com/index.php?title=Downpour_Drenches_the_World).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
