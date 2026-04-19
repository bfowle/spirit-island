# Sun-Bright Whirlwind

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
| Power summary (1–5)   | Offense 3 · Control 5 · Fear 1 · Defense 1 · Utility 3             |
| Primary Elements      | Sun, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Incredibly good at handling Explorer, clearing newly-Explored lands of Invaders so they don't Build there. Not nearly so good at dealing with Town/City. Can focus on Energy and largely forego its Innate Power, focus on Plays to aim for mid-to-high Innate thresholds, or strike a more balanced path. Adds at most 1 Presence per turn, so there won't be time to do it all.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the highest-numbered Sands, 2 in the lowest-numbered Mountain.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence1, second=energy4 |
| G3 | first=gain1p, second=addpresence4 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, sun, energy3, energy4air, energy6
- **Card-play track**: card1, card2, card3, airX, card4, card5sun

## Core Mechanics & Special Rules

### Special Rule

A STIFF WIND AT THEIR BACKS After you Add Presence during Growth, Push up to 1 Explorer/Dahan from that land. (Let other players know this is due to your Special Rule, so they know you're still in the Spirit Phase and not using a Fast power.)

### Innate: VIOLENT WINDSTORMS

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 2 Air | Push up to 1 Explorer. |
| 2 | 2 Sun + 3 Air | 1 Fear. Push up to 2 Explorer/Town. |
| 3 | 2 Sun + 4 Air | For each Invader Pushed by this Power, 1 Damage in the land it was Pushed to. |
| 4 | 3 Sun + 5 Air | 4 Damage (in target land). |


## Unique Cards (all, Wiki-verified)



## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Gift of Living Energy** | 0 | Fast | No Range | Any Spirit | Sun, Fire, Plant | Target Spirit gains 1 Energy. If you have at least 2 Sacred Sites, target Spirit gains 1 Energy. If you target another Spirit, they gain 1 Energy. |
| **Elemental Boon** | 1 | Fast | No Range | Any Spirit | — | Target Spirit gains 3 different Elements of their choice. If you target another Spirit, you also gain the chosen Elements. |
| **Reaching Grasp** | 0 | Fast | No Range | Any Spirit | Sun, Air, Water | Target Spirit gets +2 Range with all their Powers. |
| **Enticing Splendor** | 0 | Fast | 0 | Land with no Blight | Sun, Air, Plant | Gather 1 Explorer/Town. **OR** Gather up to 2 Dahan. |
| **Call to Isolation** | 0 | Fast | 1 | Land with Dahan | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. |

### Major Powers

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Powerstorm** | 3 | Fast | No Range | Any Spirit | Sun, Fire, Air | Target Spirit gains 3 Energy. Once this turn, target Spirit may Repeat a Power Card by paying its cost again. |
| **Wrap in Wings of Sunlight** | 3 | Fast | 0 | Any Land | Sun, Air, Animal | Move up to 5 Dahan to any land (including back into target land). If you moved at least 1 Dahan, Defend 5 in that land. |



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
- **Authoritative mechanics** (this chapter): `data/references/wiki/sun-bright-whirlwind.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Sun-Bright Whirlwind](https://spiritislandwiki.com/index.php?title=Sun-Bright_Whirlwind).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
