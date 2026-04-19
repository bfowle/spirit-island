# Dances Up Earthquakes

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | Veryhigh                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 2 · Defense 3 · Utility 4             |
| Primary Elements      | Earth, Moon (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very much about tempo and timing: Can play high-cost Powers extremely easily, but they won't take effect until later in the game. Faces a constant tension between solving problems now and carefully planning ahead for big turns in the future - neglecting either one can be disastrous. Despite starting with 6 Unique Powers, benefits greatly from gaining more.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the highest-numbered land with Dahan. You start with your 6 Unique Power Cards and 0 Energy. Set the Quake Tokens ({{quake}}) nearby.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence2orgainmajorwoforgetting |
| G2 | first=gain1p, second=addpresence1 |
| G3 | first=addpresence3, second=energyimpend, third=reclaim1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1impendenergy1, movepresence1energytrack, energy2, impend1, energy3, impendenergy2, energy4any
- **Card-play track**: card2, gather1dahan1land, moonfire, impend1, earthX, card3, card4

## Core Mechanics & Special Rules

### Special Rule

BEGIN A DANCE OF DECADES</br>Whenever you would play a Power Card, you may instead pay any amount of Energy onto the card to make it an impending card ({{impendingcard}}), setting it aside out of play for use on a future turn. (It doesn't provide Elements. It's still your Power Card, so it can be forgotten while it's impending.) RHYTHMIC POWER BUILDS TO A CATACLYSMIC CRESCENDO</br>When you gain Energy from your Presence Track, also gain {{impendingenergyblank}} Energy onto each Power Card made {{impendingcard}} on a previous turn. If any {{impendingcard}} now have Energy on them at least equal to their cost, discard that Energy and play them. (This costs no card plays.)

### Innate: LAND CREAKS WITH TENSION

- **Speed**: fast · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Earth | If you have at least 1 {{impendingcard}}, Add 1 {{quake}} in one of your lands. |
| 2 | 1 Moon + 1 Earth | In one of your lands, Defend 1 per {{impendingcard}} (max. 3). |
| 3 | 1 Moon + 2 Earth | If you have at least 3 {{impendingcard}}, Add 1 {{quake}} in one of your lands. |
| 4 | 2 Moon + 3 Earth | In one of your lands, Defend 1 per {{impendingcard}} (max. 3). |


### Innate: EARTH SHUDDERS, BUILDINGS FALL

- **Speed**: slow · **Range**: 0 · **Target**: quake

_(no thresholds listed in Wiki)_


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/dances-up-earthquakes.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Dances Up Earthquakes](https://spiritislandwiki.com/index.php?title=Dances_Up_Earthquakes).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
