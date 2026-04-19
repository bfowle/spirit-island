# Bringer of Dreams and Nightmares

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | High                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 2 · Fear 5 · Defense 2 · Utility 2             |
| Primary Elements      | Moon, Air, Animal, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> With most Spirits, Terror Victories are a backup plan if the main push against the Invaders stalls out for too long, but Bringer turns Fear into a more viable primary strategy. Its transformation of damage & destruction into Fear can turn Major Powers into tremendous sources of terror and panic. However, the only real offense Bringer has is the Dahan fighting back. While it does have some defensive ability, it is fundamentally poor at clearing areas of Invaders.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board in the highest-numbered Sands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=reclaim1, second=addpresence0 |
| G3 | first=gain1p, second=addpresence1 |
| G4 | first=Night, second=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, air, energy3, moon, energy4, any, energy5
- **Card-play track**: card2, card2, card2, card3, card3, any

## Core Mechanics & Special Rules

### Special Rule

TO DREAM A THOUSAND DEATHS Your Powers never cause Damage, nor can they Destroy anything other than your own Presence. When your Powers would Destroy (or deal enough Damage to Destroy) Explorer/Town/City, instead generate 0/2/5 Fear. The Power Pushes all Explorer/Town it would Destroy. Notes: A single Power cannot Destroy a given Invader more than once. Powers that cause Damage via Dahan are affected just like all others. All effects other than Damage/Destroy work as usual.

### Innate: SPIRITS MAY YET DREAM

- **Speed**: fast · **Range**: None · **Target**: anyspirit

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 2 Air | Turn any face down Fear Card face-up. (It's earned/resolved normally, but players can see what's coming) |
| 2 | 3 Moon | Target Spirit gains an element that they have at least 1 of. |


### Innate: NIGHT TERRORS

- **Speed**: fast · **Range**: 0 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Air | 1 Fear. |
| 2 | 2 Moon + 1 Air + 1 Animal | +1 Fear. |
| 3 | 3 Moon + 2 Air + 1 Animal | +1 Fear. |


## Unique Cards (all, Wiki-verified)

#### Call on Midnight's Dream

- **0 Energy · Fast · Range 0 · Any Land · Moon, Animal**
- *If target land has Dahan, gain a Major Power. If you Forget this Power, gain Energy equal to Dahan and you may play the Major Power immediately, paying its cost. **OR** If Invaders are present, 2 Fear.*

#### Dread Apparitions

- **2 Energy · Fast · Range 1 · Land with 1 or more Invaders · Moon, Air**
- *When Powers generate Fear in target land, Defend 1 per Fear. 1 Fear. (Fear from To Dream a Thousand Deaths counts. Fear from Destroying Towns/Cities does not.)*

#### Dreams of the Dahan

- **0 Energy · Fast · Range 2 · Any Land · Moon, Air**
- *Gather up to 2 [[Dahan]]. **OR** If target land has [[Towns]]/[[Cities]], 1 [[Fear]] for each [[Dahan]], to a maximum of 3 [[Fear]].*

#### Predatory Nightmares

- **2 Energy · Slow · Range 1, from your Sacred Site · Land with 1 or more Invaders · Moon, Fire, Earth, Animal**
- *2 Damage. Push up to 2 Dahan. (When your Powers would Destroy Invaders, instead they generate Fear and/or Push those Invaders.)*


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
- **Authoritative mechanics** (this chapter): `data/references/wiki/bringer-of-dreams-and-nightmares.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Bringer of Dreams and Nightmares](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
