# Lightning's Swift Strike

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
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 3 · Defense 1 · Utility 2             |
| Primary Elements      | Fire, Air, Water (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Virtually all offense to start with: without a more defensive teammate, Blight may become a problem. Excellent at destroying buildings, less good at containing Explorers. Using Thundering Destruction tends to be a burst affair: a turn or two of position and build up Energy, followed by a really big turn. Starting Powers are extremely focused on Air and Fire: good for Thundering Destruction, bad for Major Power versatility.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board in the highest-numbered Sands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence2, second=addpresence0 |
| G3 | first=addpresence1, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy1, energy2, energy2, energy3, energy4, energy4, energy5
- **Card-play track**: card2, card3, card4, card5, card6

## Core Mechanics & Special Rules

### Special Rule

SWIFTNESS OF LIGHTNING For every Air you have, you may use 1 Slow Power as if it were Fast. (Power Cards or your Innate Powers.)

### Innate: THUNDERING DESTRUCTION

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Fire + 2 Air | Destroy 1 Town. |
| 2 | 4 Fire + 3 Air | You may instead destroy 1 City. |
| 3 | 5 Fire + 4 Air + 1 Water | Also, Destroy 1 Town / City. |
| 4 | 5 Fire + 5 Air + 2 Water | Also, Destroy 1 Town / City. |


## Unique Cards (all, Wiki-verified)



## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Delusions of Danger** | 1 | Fast | 1 | Any Land | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. |
| **Call to Bloodshed** | 1 | Slow | 1 | Land with Dahan | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan. |
| **Purifying Flame** | 1 | Slow | 1, from your Sacred Site | Any Land | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blight. |
| **Entrancing Apparitions** | 1 | Fast | 1 | Any Land | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers. |
| **Call to Isolation** | 0 | Fast | 1 | Land with Dahan | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. |

### Major Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Powerstorm** | 3 | Fast | No Range | Any Spirit | Sun, Fire, Air | Target Spirit gains 3 Energy. Once this turn, target Spirit may Repeat a Power Card by paying its cost again. |
| **Pillar of Living Flame** | 5 | Slow | 2, from your Sacred Site | Any Land | Fire | 3 Fear. 5 Damage. If target land is a Jungle or Wetland, add 1 Blight. |



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
- **Authoritative mechanics** (this chapter): `data/references/wiki/lightnings-swift-strike.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Lightning's Swift Strike](https://spiritislandwiki.com/index.php?title=Lightning's_Swift_Strike).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
