# Breath of Darkness Down Your Spine

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
| Power summary (1–5)   | Offense 2 · Control 4 · Fear 5 · Defense 1 · Utility 2             |
| Primary Elements      | Moon, Animal, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Abducts lone Invaders to gain Fear and keep them off the board for a time; its mobile Incarna is particularly useful for this. Reclaiming permits the Invaders to escape its void en masse, so can be quite painful. Has trouble with built-up lands, and may need to scatter Invaders or take a Major Power to deal with them.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence and your Incarna ({{incarna|breath}}), Unempowered ({{incarna|unempowered}}), on your starting board: 1 Presence and {{incarna|breath}} in the lowest-numbered Jungle and 1 in the highest-numbered Jungle. Set [[The Endless Dark]] ({{endlessdark}}) tile next to the island with 1 Explorer on it. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=movebreathincarna |
| G2 | first=gain1p, second=addpresence3, third=darknessescape2 |
| G3 | first=addpresence1, second=addmovebreathincarna, third=darknessescape1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, moon, energy3, empowerincarna, energy4animal, energy5air
- **Card-play track**: card2, movepres, card3, moonX, reclaim1, card4air

## Core Mechanics & Special Rules

### Special Rule

TERROR STALKS THE LAND</br>You have an Incarna ({{incarna|breath}}). Empower {{incarna|breath}} after uncovering {{empower}}. You may Abduct 1 Explorer/Town at empowered {{incarna|breath}} each Fast phase. To Abduct a piece, Move it to [[The Endless Dark]] ({{endlessdark}}). When pieces Escape, Move them to non-Ocean lands with your Presence/{{incarna|breath}}; if they have no legal land to move to, you lose. When your Powers would directly damage or directly destroy the only Invader in a land, instead Abduct it. (Check if it is alone at the start of the damage/destroy instruction. {{endlessdark}} is not a land.) SHADOW-TOUCHED REALM</br>Your land-targeting Powers can target {{endlessdark}} as if it were a land, ignoring Range. ({{endlessdark}} is Inland and has no terrain.) Rules for The Endless Dark</br>{{#lst:The Endless Dark|ted}}

### Innate: LEAVE A TRAIL OF DEATHLY SILENCE

- **Speed**: fast · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 1 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 2 | 3 Moon + 1 Air + 1 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 3 | 4 Moon + 2 Air + 2 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 4 | 5 Moon + 2 Air + 3 Animal | Move {{incarna|breath}} to {{endlessdark}}. It Brings 1 Invader (from its land). |


### Innate: LOST IN THE ENDLESS DARK

- **Speed**: slow · **Range**: None · **Target**: endlessdark

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 1 Air | 1 Fear per Invader (max. 4). Downgrade up to 1 Invader. (Downgrading Removes Explorer.) |
| 2 | 4 Moon + 3 Air | 1 Fear per Invader (max. 4). Downgrade any number of Invaders. |
| 3 | 3 Moon + 2 Animal | Add 1 Beast. |


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/breath-of-darkness-down-your-spine.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Breath of Darkness Down Your Spine](https://spiritislandwiki.com/index.php?title=Breath_of_Darkness_Down_Your_Spine).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
