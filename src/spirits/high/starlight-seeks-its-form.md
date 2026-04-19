# Starlight Seeks Its Form

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Very High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "three" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 1 · Fear 1 · Defense 2 · Utility 2             |
| Primary Elements      | Moon, Air, Earth, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> A build-your-own-Spirit, capable of going in many different directions based on Elements picked, Growth choices selected, and Power Cards kept. Has a very high personal/visual complexity and a huge number of early-game options, but doesn't alter play much for other players at the table. As it commits to choices, it loses versatility - not all paths will be good (or even possible) at all things. It especially wants a measure of adaptation to early Power Cards, rather than trying to pre-select a strategy.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board, in a land with Blight.

## Growth Options (three)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim1 |
| G2 | first=addpresence0 |
| G3 | first=energy1 |
| G4 | first=movepresence3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: empty, growthreclaimhalf, growthdivider, growthgainpowercard, growthmovepresence1
- **Card-play track**: empty, growth3energy, growthdivider, growthplusonecardplay, growthmovepresence2

## Core Mechanics & Special Rules

### Special Rule

GROWTH BEGETS GROWTH You have 6 Presence tracks. (As usual, you may add Presence from any track.) 4 of the Presence tracks are next to rows of Growth choices: these choices start unavailable. Upon emptying a Growth track, pick one of its two Growth choices to be immediately available. The other stays unavailable for the rest of the game (cover with a spare piece). After you add Presence from a space marked {{energyplus1}}, gain 1 Energy. SLOWLY COALESCING NATURE After revealing an {{gainelement}}, place 1 Element Marker of your choice on it. That element is permanent and is constantly available (As if pre-printed on the Presence track.)

### Innate: AIR MOVES, EARTH ENDURES

- **Speed**: fast · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Air | Push up to 2 Explorer or 1 Town. |
| 2 | 3 Earth | Defend 5. |


### Innate: FIRE BURNS, WATER SOOTHES

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Fire | 1 Fear. 2 Damage. |
| 2 | 3 Water | Remove 1 Blight. |


### Innate: WOOD SEEKS GROWTH, HUMANS SEEK FREEDOM

- **Speed**: slow · **Range**: 2 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Plant | Choose a Spirit with Presence in target land. They gain a Power Card. |
| 2 | 3 Animal | 1 Damage per Dahan OR Push up to 3 Dahan. |


### Innate: SIDEREAL GUIDANCE

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon | Gather up to 1 Explorer/Dahan. |
| 2 | 3 Moon | Instead, Gather up to 3 Explorer. |


### Innate: STARS BLAZE IN THE DAYTIME SKY

- **Speed**: slow · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 Sun | 3 Fear. Gain 1 Energy. Reclaim up to 1 Power Card from play or your discard pile. |


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/starlight-seeks-its-form.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Starlight Seeks Its Form](https://spiritislandwiki.com/index.php?title=Starlight_Seeks_Its_Form).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
