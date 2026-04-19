# Relentless Gaze of the Sun

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "always" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 1 · Fear 3 · Defense 2 · Utility 3             |
| Primary Elements      | Sun, Fire, Air, Water (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Uses stacks of 3 Presence and high Energy income to hammer the same lands with repeated Power Cards. Because of its single-mindedness, is better at dealing with large problems than smaller ones. Harms the land and Dahan while smiting Invaders unless it successfully changes its nature (by expanding into new Elements), which may mean partially foregoing the Sun and Fire that form the core of its initial strength.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence and 1 Badlands on your starting board, in the lowest-numbered Sands. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (always)

| Growth | Effects |
|--------|---------|
| G1 | first=Gaze |
| G2 | first=reclaim, second=add3destroyedpresencetogether |
| G3 | first=gain1p |
| G4 | first=gaindoubleenergy, second=move3presencetogether |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2sun, energy3fire, sun, energy4any, energy5, 
- **Card-play track**: card1, card1, card2, sunX, card3, reclaim1, card4

## Core Mechanics & Special Rules

### Special Rule

RELENTLESS PUNISHMENT After using a Power Card, if you had at least 3 Presence in the origin land, you may Repeat it any number of times on the same target land(s) (ignoring origin, Range, and target requirements) by paying both: * the Energy cost of the Power, and * 1 Energy per previous use of the Power this turn each time you Repeat the Power. (You may Repeat each Power Card multiple times, but its 1st Repeat costs 1 extra, its 2nd Repeat costs 2 extra, etc. Check if the origin has at least 3 Presence when you target it the first time.)

### Innate: SCORCHING CONVERGENCE

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Sun | Move all of your Presence from origin land directly to target land. 1 Damage, to Town/City only. |
| 2 | 3 Sun + 1 Fire | 3 Damage to Invaders. 3 Damage to Dahan. Add 1 Blight without cascading. |
| 3 | 4 Sun + 2 Fire + 1 Air | 3 Fear if this Power destroyed any Invaders. |
| 4 | 5 Sun + 3 Fire + 2 Air | 1 Damage per remaining Presence of yours in target land. |


### Innate: CONSIDER A HARMONIOUS NATURE

- **Speed**: fast · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Sun + 1 Moon | When your Powers would Add Blight, you may Destroy 1 Presence instead (there or elsewhere). |
| 2 | 3 Sun + 1 Water | Your Powers don't damage or destroy Dahan. |
| 3 | 3 Sun + 1 Plant | Choose another Spirit. They Add 1 {{destroyedpresence}} to one of your lands. |
| 4 | 3 Sun + 1 Water + 1 Plant | Give up to 3 of your Energy to the chosen Spirit. |


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/relentless-gaze-of-the-sun.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Relentless Gaze of the Sun](https://spiritislandwiki.com/index.php?title=Relentless_Gaze_of_the_Sun).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
