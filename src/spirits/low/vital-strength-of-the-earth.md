# Vital Strength of the Earth

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3             |
| Primary Elements      | Earth, Plant, Sun, Animal (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Powerful but slow: has potent Power Cards and an excellent Energy income, but starts wtih only one card play per turn, and Growth is limited to adding one Presence per turn. Also slow to change: learning new Powers carries slightly more cost than reclaiming played Power Cards

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 2 in the highest-numbered Mountain, 1 in the highest-numbered Jungle.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence2 |
| G2 | first=gain1p, second=addpresence0 |
| G3 | first=addpresence1, second=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, energy3, energy4, energy6, energy7, energy8
- **Card-play track**: card1, card1, card2, card2, card3, card4

## Core Mechanics & Special Rules

### Special Rule

EARTH'S VITALITY Defend 3 in every land where you have Sacred Site.

### Innate: GIFT OF STRENGTH

- **Speed**: fast · **Range**: None · **Target**: anyspirit

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 2 Earth + 2 Plant | Once this turn, Target Spirit may Repeat 1 Power Card with Energy cost of 1 or less. |
| 2 | 2 Sun + 3 Earth + 2 Plant | Instead, the Energy cost limit is 3 or less. |
| 3 | 2 Sun + 4 Earth + 3 Plant | Instead, the Energy cost limit is 6 or less. |


## Unique Cards (all, Wiki-verified)

#### A Year of Perfect Stillness

- **3 Energy · Fast · Range 1 · Any Land · Sun, Earth**
- *Invaders skip all Actions in target land this turn.*

#### Draw of the Fruitful Earth

- **1 Energy · Slow · Range 1 · Any Land · Earth, Plant, Animal**
- *Gather up to 2 [[Explorers]]. Gather up to 2 [[Dahan]].*

#### Guard the Healing Land

- **3 Energy · Fast · Range 1, from your Sacred Site · Any Land · Water, Earth, Plant**
- *Remove 1 Blight. Defend 4.*

#### Rituals of Destruction

- **3 Energy · Slow · Range 1, from your Sacred Site · Land with Dahan · Sun, Moon, Fire, Earth, Plant**
- *2 Damage. If target land has at least 3 Dahan, +3 Damage and 2 Fear.*


## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Rouse the Trees and Stones** | 1 | Slow | 1, from your Sacred Site | Land with no Blight | Fire, Earth, Plant | 2 Damage. Push 1 Explorer. |
| **Call to Migrate** | 1 | Slow | 1 | Any Land | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. |
| **Devouring Ants** | 1 | Slow | 1, from your Sacred Site | Any Land | Sun, Earth, Animal | 1 Fear. 1 Damage. Destroy 1 Dahan. If target land is a Jungle or Sands, +1 Damage. |
| **Voracious Growth** | 1 | Slow | 1, from your Sacred Site | Jungle or Wetland | Water, Plant | 2 Damage. **OR** Remove 1 Blight. |
| **Savage Mawbeasts** | 0 | Slow | 1, from your Sacred Site | Any Land | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. |

### Major Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Poisoned Land** | 3 | Slow | 1 | Any Land | Earth, Plant, Animal | 1 Fear. 7 Damage. Add 1 Blight. Destroy all Dahan. |
| **Vigor of the Breaking Dawn** | 3 | Fast | 2 | Land with Dahan | Sun, Animal | 2 Damage per Dahan. |



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
- **Authoritative mechanics** (this chapter): `data/references/wiki/vital-strength-of-the-earth.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Vital Strength of the Earth](https://spiritislandwiki.com/index.php?title=Vital_Strength_of_the_Earth).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
