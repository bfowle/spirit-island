# Fathomless Mud of the Swamp

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
| Power summary (1–5)   | Offense 3 · Control 2 · Fear 3 · Defense 3 · Utility 2             |
| Primary Elements      | Water, Moon, Earth, Plant (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Likes having Sacred Site where Invaders will Build, but may need to re-create those Sacred Site after oozing outwards with its Innate Power. In smaller games, might be able to cut off the most Inland lands from Explore actions by Destroying Inland Town/City and stopping new ones from being built. Causes a fair bit of Fear, much of which represents unpleasantness, hardship, and disgust.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board, in the lowest-numbered Wetland. You start with your 4 Unique Powers and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=addpresence1, second=addpresence1 |
| G3 | first=gain1p, second=addpresence2, third=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, plant, energy2, energy3, water, energy4, energy5
- **Card-play track**: card1, card2, earthX, card3, moonX, card4

## Core Mechanics & Special Rules

### Special Rule

OFFER NO FIRM FOUNDATIONS At your Sacred Site, Build actions add Explorer instead of Town/City.

### Innate: SPREADING AND DREADFUL MIRE

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Water | Move 1 Presence from the origin Sacred Site to target land. (This is required.) |
| 2 | 1 Moon + 2 Water + 1 Earth | 1 Fear. 1 Damage. Push 1 Dahan. |
| 3 | 2 Moon + 3 Water + 2 Earth | 1 Fear. 1 Damage. Push 1 Dahan. |
| 4 | 3 Moon + 4 Water + 3 Earth + 2 Plant | 2 Damage. |


## Unique Cards (all, Wiki-verified)



## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Steam Vents** | 1 | Fast | 0 | Any Land | Fire, Air, Water, Earth | Destroy 1 Explorer. |
| **Entrancing Apparitions** | 1 | Fast | 1 | Any Land | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers. |
| **Uncanny Melting** | 1 | Slow | 1, from your Sacred Site | Any Land | Sun, Moon, Water | If Invaders are present, 1 Fear. If target land is a Sands or Wetland, Remove 1 Blight. |
| **Pull Beneath the Hungry Earth** | 1 | Slow | 1 | Any Land | Moon, Water, Earth | If your Presence is present, 1 Fear and 1 Damage. If target land is a Sands or Wetland, 1 Damage. |
| **Gift of Power** | 0 | Slow | No Range | Any Spirit | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. |

### Major Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Cleansing Floods** | 5 | Slow | 1, from a Wetland | Any Land | Sun, Water | 4 Damage. Remove 1 Blight. |
| **The Land Thrashes in Furious Pain** | 4 | Slow | 2 | Land with Dahan | Moon, Fire, Earth | 2 Damage per Blight. For each Blight in adjacent lands, 1 Damage (in target land). |



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
- **Authoritative mechanics** (this chapter): `data/references/wiki/fathomless-mud-of-the-swamp.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Fathomless Mud of the Swamp](https://spiritislandwiki.com/index.php?title=Fathomless_Mud_of_the_Swamp).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
