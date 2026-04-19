# Wounded Waters Bleeding

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 5 · Fear 2 · Defense 1 · Utility 1             |
| Primary Elements      | Water, Animal, Plant, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Starts off wounded, losing a Presence or a Power Card every turn. Heals over the course of the game, finding a new nature - while some choices may be a touch better or worse due to Adversary, Power Card picks, teammates, etc., most combinations are viable in most games. Benefits from careful Presence placement, both due to losing Presence and because some if its Unique Powers must target from lands with Blight.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> On your starting board, put 2 Presence in a land with Blight, then put 2 Presence and 1 Blight (from the box) in the highest-numbered land with a Town Setup Symbol. You start with your 4 Unique Power Cards and 4 Energy.</br>Set your 4 Healing Cards nearby.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=gain1p, second=addpresence2 |
| G3 | first=addpresence3, second=energy3, third=adddestroyedpresence1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: blank, blank, blank, blank, energy3, energy4fireorplant, energy5any
- **Card-play track**: energy0card1, wateroranimal, gatherblight, energy1card2

## Core Mechanics & Special Rules

### Special Rule

SEEKING A PATH TOWARDS HEALING</br>After playing cards each Spirit Phase: * Claim a Healing Marker (Element Marker) matching whichever of Water or Simpleanimal you have more of. (You break ties.) * You may then Claim a Healing Card if you meet its requirements. (You can claim your first Healing Card on Turn 3.) * Then Destroy 1 Presence or Forget a Power Card (unless a Healing Card just removed this rule).

### Innate: SWIRL AND SPILL

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Water | Push up to 2 Explorer/Dahan/Blight. |
| 2 | 3 Water + 1 Animal | 1 Fear. Push up to 2 Town/Presence/Beasts. |
| 3 | 5 Water + 2 Plant + 2 Animal | In one land pushed into, Downgrade all Town and all City. |


### Innate: SANGUINARY TAINT

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Animal | 1 Fear. 1 Damage. Push 1 Dahan. |
| 2 | 1 Water + 3 Animal | 1 Damage. Add 1 Beasts. |
| 3 | 2 Fire + 2 Water + 5 Animal | 1 Fear. 4 Damage. Add 1 Disease. |


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/wounded-waters-bleeding.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Wounded Waters Bleeding](https://spiritislandwiki.com/index.php?title=Wounded_Waters_Bleeding).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
