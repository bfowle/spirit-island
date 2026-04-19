# Many Minds Move as One

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Moderate                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 5 · Fear 5 · Defense 5 · Utility 1             |
| Primary Elements      | Animal, Air, Water, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Requires heavy spatial thought for Beast movement, due to its improved Push/Gather and large numbers of Beast. Has no offense to start with, but an excellent stalling defense combined with Fear generation; outright Fear victories may be plausible in smaller games. Both Fear Cards and Beast events are unpredictable, however, so swings of fortune are apt to be more relevant than usual.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Beast on your starting board, in a land with Beast. Note that you have 5 Unique Power Cards.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=addpresence1, second=addpresence0 |
| G3 | first=addpresence3beast, second=energy1, third=gatherbeast2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, air, energy2, animal, energy3, energy4
- **Card-play track**: card1, card2, pay2pcard, card3, card3, card4, card5

## Core Mechanics & Special Rules

### Special Rule

FLY FAST AS THOUGHT When you Gather or Push Beast, they may come from or go to lands up to 2 distant (rather than adjacent only). A JOINING OF SWARMS AND FLOCKS Your {{sacredsite}} may also count as Beast. (Note: You never have more than 1 {{sacredsite}} in a land, no matter how many Presence you have there.) (If something changes a Beast that is your {{sacredsite}}, it affects 2 of your Presence there - e.g., Push 1 Beast will Push 2 of your Presence together.)

### Innate: THE TEEMING HOST ARRIVES

- **Speed**: fast · **Range**: 2 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Air + 1 Animal | Gather up to 1 Beasts. |
| 2 | 3 Air + 1 Water + 2 Animal | Instead, Gather up to 1 Beasts per {{air}} you have. |
| 3 | 1 Fire + 4 Air + 2 Animal | Push up to 3 Beasts. |


### Innate: BESET AND CONFOUND THE INVADERS

- **Speed**: fast · **Range**: 2 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Air + 2 Animal | 2 Fear and Defend 2. |
| 2 | 2 Air + 3 Animal | Instead, 3 Fear and Defend 4. |
| 3 | 3 Air + 4 Animal | Instead, 4 Fear and Defend 7. |
| 4 | 4 Air + 1 Earth + 5 Animal | Instead, 6 Fear and Defend 10. |


## Unique Cards (all, Wiki-verified)

#### None

- **0 Energy · Fast · Range Range 1, from your Sacred Site · Land with 2 or more Beasts tokens · Moon, Air, Water, Animal**
- *Remove up to half (round down) of Beasts in target land. For each Beasts Removed, 2 Fear and skip one Invader Action.*

#### None

- **0 Energy · Fast · Range No Range · Another Spirit · Air, Water, Animal**
- *For the rest of this turn, each of target Spirit's Presence grants Defend 1 in its land. Target Spirit may Push up to 1 of their Presence.*

#### None

- **1 Energy · Slow · Range 0 · Any Land · Fire, Earth, Animal**
- *Add 2 Beasts.*

#### None

- **0 Energy · Fast · Range 1 · Any Land · Sun, Air, Animal**
- *Move 1 Beasts up to two lands. As it moves, up to 2 Dahan may move with it, for part or all of the way. (The Beasts/Dahan may move to an adjacent land and then back.)*

- **Pursue with Scratches$ Pecks$ and Stings** — `[VERIFY]` (Wiki fetch error: Wiki API error for 'Pursue_with_Scratches$_Pecks$_and_Stings': {'code': 'missingtitle', 'info': "The page you specified doesn't exist.", 'docref': 'See https://spiritislandwiki.com/api.php for API usage. Subscribe to the mediawiki-api-announce mailing list at &lt;https://lists.wikimedia.org/mailman/listinfo/mediawiki-api-announce&gt; for notice of API deprecations and breaking changes.'})


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/many-minds-move-as-one.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Many Minds Move as One](https://spiritislandwiki.com/index.php?title=Many_Minds_Move_as_One).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
