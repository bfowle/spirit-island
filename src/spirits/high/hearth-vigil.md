# Hearth-Vigil

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 3 · Control 1 · Fear 2 · Defense 4 · Utility 4             |
| Primary Elements      | Sun, Earth, Animal, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very good at protecting Dahan in its lands, not so great at stopping Blight. In keeping with its nature, largely brings Dahan to its Presence (or vice versa); getting Dahan elsewhere may require a bit of forethought with Keep Watch for New Incursions. Very reactive, with reliable ways to deal with Invaders as they Ravage and Build, but has trouble handling established City that aren't Ravaging.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the highest-numbered land with Dahan and 2 in the lowest-numbered land with at least 2 Dahan. Add 1 Dahan in each of those lands (additional survivors of the Invaders' diseases). You start with your 4 Unique Power Cards and 1 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence3dahan |
| G3 | first=addpresence2, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: gather1dahan1land
- **Card-play track**: energy0, energy1sun, energy2, energy3animal, energy4, energy5sun

## Core Mechanics & Special Rules

### Special Rule

ROOTED IN THE COMMUNITY Blight added in your lands does not Destroy your Presence if Dahan are present. (Ravage Actions Destroy Dahan before added Blight destroys Presence and cascades.) FORTIFY HEART AND HEARTH Dahan have +4 Health (each) while in your lands. Event and Blight Card Actions don't damage, destroy, or replace Dahan in your lands. (Ravages are not Event Actions even if caused by Events.) LOYAL GUARDIAN When all Dahan leave one of your lands, your Presence may Move with those Dahan. (Each Dahan can Bring any number of Presence.)

### Innate: WARN OF IMPENDING CONFLICT

- **Speed**: fast · **Range**:  · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Sun + 1 Earth | In one of your lands, 1 Dahan deals Damage before Invaders during Ravages. (Choose a land when Invaders Ravage there.) |
| 2 | 3 Sun + 1 Earth | In that land, another Dahan deals Damage before Invaders during Ravages. |
| 3 | 4 Sun + 2 Earth | In that land, all Dahan deal Damage before Invaders during Ravages. |
| 4 | 5 Sun + 3 Earth | Instead, all Dahan in all of your lands deal Damage before Invaders during Ravages. |


### Innate: KEEP WATCH FOR NEW INCURSIONS

- **Speed**: fast · **Range**: 1 (optionally from a sacredsitedahan site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Animal | Gather up to 2 Dahan, from your lands only. |
| 2 | 1 Sun + 2 Air + 3 Animal | Once this turn after Invaders are added or moved into target land, 1 Damage per Dahan in target land, to those added/moved Invaders only. |
| 3 | 2 Sun + 3 Air + 4 Animal | Repeat this Power. |


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

```admonish note title="Stat Insight `[VERIFY from mindwanderer]`"
Pending re-scrape of mindwanderer current data. Historical directional figures unavailable in this template draft.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/hearth-vigil.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Hearth-Vigil](https://spiritislandwiki.com/index.php?title=Hearth-Vigil).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
