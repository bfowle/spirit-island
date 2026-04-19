# Devouring Teeth Lurk Underfoot

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Horizons of Spirit Island                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 2 · Defense 1 · Utility 1             |
| Primary Elements      | Fire, Animal, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Likes being in the same lands as Invaders, so it can use Range 0 offensive and defensive Powers. The first of its Innate Power can give some mobility, if needed. Has a poor Plays track and potent but expensive Unique Powers, so can be better at handling fewer large threats than lots of little ones.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board, in land #5. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence1 |
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

- **Energy track**: energy2, fire, energy3, energy4, animal, energy6, energy7
- **Card-play track**: card1, card2, animalX, fireX, card3, earthX, card4

## Core Mechanics & Special Rules

### Special Rule

TERRITORIAL AGGRESSION Your Damage-Dealing Powers do +1 Damage. (This adds +1 Damage total for the Power, even if the Power Damages multiple Invaders or "each Invader". It can boost Minor/Major Power Cards, too, not just your Uniques + Innate.)

### Innate: DEATH APPROACHES FROM BENEATH THE SURFACE

- **Speed**: slow · **Range**: 1 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Fire + 1 Animal | If you don't have Presence in target land, Gather 1 of your Presence. (This is required.) |
| 2 | 2 Fire + 1 Earth + 2 Animal | 1 Damage. (+1 for your "Territorial Aggression" Special Rule) |
| 3 | 3 Fire + 1 Earth + 3 Animal | 2 Damage. |
| 4 | 4 Fire + 2 Earth + 5 Animal | 2 Fear. 4 Damage. |


## Unique Cards (all, Wiki-verified)



## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Devouring Ants** | 1 | Slow | 1, from your Sacred Site | Any Land | Sun, Earth, Animal | 1 Fear. 1 Damage. Destroy 1 Dahan. If target land is a Jungle or Sands, +1 Damage. |
| **Gnawing Rootbiters** | 0 | Slow | 1 | Any Land | Earth, Animal | Push up to 2 Towns. |
| **Rouse the Trees and Stones** | 1 | Slow | 1, from your Sacred Site | Land with no Blight | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. |
| **Savage Mawbeasts** | 0 | Slow | 1, from your Sacred Site | Any Land | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. |

### Major Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Pillar of Living Flame** | 5 | Slow | 2, from your Sacred Site | Any Land | Fire | 3 Fear. 5 Damage. If target land is a Jungle or Wetland, add 1 Blight. |
| **Poisoned Land** | 3 | Slow | 1 | Any Land | Earth, Plant, Animal | 1 Fear. 7 Damage. Add 1 Blight. Destroy all Dahan. |

### Other (error or unclassified)\n\n| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| Quicken the Earth\'s Struggles | — | — | — | — | — | _Wiki error_ |

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/devouring-teeth-lurk-underfoot.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Devouring Teeth Lurk Underfoot](https://spiritislandwiki.com/index.php?title=Devouring_Teeth_Lurk_Underfoot).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
