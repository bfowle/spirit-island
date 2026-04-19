# Towering Roots of the Jungle

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 3 · Fear 2 · Defense 5 · Utility 2             |
| Primary Elements      | Sun, Plant, Earth, Moon (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Incredibly good at protecting everything at its {{incarna|roots}} - and can draw Invaders towards there - but is constrained in when and where it can move its {{incarna|roots}}. Has some ability to Remove Invaders (driving them from the island), but starts off vastly better at guarding the land than at smashing things.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the highest-numbered Jungle without Blight, 1 in the highest-numbered Mountain, and 1 in the highest-numbered Wetland. Put {{incarna|roots}}, Unempowered ({{incarna|unempowered}}) side up, in the Jungle with your Presence. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence1, third=addvitalityrootsincarna |
| G3 | first=gain1p, second=addpresence3, third=replacepresencerootsincarna |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, earth, energy4, plant, energy6
- **Card-play track**: card1, card2, sunX, card3, plantX, card4

## Core Mechanics & Special Rules

### Special Rule

ENDURING VITALITY</br>Some of your Actions Add Vitality Tokens ({{vitality}}). (Each {{vitality}} in a land with no Blight prevents 1 Blight from being added, then is Removed.) HEART-TREE GUARDS THE LAND</br>You have an Incarna ({{incarna|roots}}). * Your Powers get Range +1 if {{incarna|roots}} is in the origin land. * Invaders/Dahan/Beast can't be damaged or destroyed at {{incarna|roots}}. * Empower {{incarna|roots}} the first time it's in a land with 3 or more {{vitality}}. * Skip all Build Actions at empowered {{incarna|roots}}.

### Innate: SHELTER UNDER TOWERING BRANCHES

- **Speed**: slow · **Range**: 0 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 1 Plant | Gather up to 1 Dahan. |
| 2 | 1 Sun + 1 Earth + 2 Plant | Gather up to 1 Explorer. |
| 3 | 2 Sun + 1 Earth + 3 Plant | Gather up to 1 Town. |
| 4 | 3 Sun + 2 Earth + 4 Plant | Gather up to 1 City. |


### Innate: REVOKE SANCTUARY AND CAST OUT

- **Speed**: slow · **Range**: 0 (optionally from a sacred site) · **Target**: rootsincarnainvaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 1 Moon + 2 Plant | 1 Fear. Remove 1 Explorer/Town. |
| 2 | 2 Sun + 1 Moon + 3 Plant | 1 Fear. Remove 1 Explorer/Town. |
| 3 | 2 Sun + 2 Moon + 4 Plant | 1 Fear. Remove 1 Invader. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Towering Roots of the Jungle's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/towering-roots-of-the-jungle.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/towering-roots-of-the-jungle.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Plant** (wt 9.0), **Sun** (wt 5.7), **Moon** (wt 2.1)
- **Mid-game energy estimate (T3–T5 avg)**: 5.0E
- **Power summary**: Offense 1 · Control 3 · Fear 2 · Defense 5 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Favor of the Sun and Star-lit Dark** | 1 | Fast | Sun, Moon, Plant | Defend 4. Push up to 1 Blight. | elements moon+plant+sun → 16.8 |
| 2 | **Absorb Corruption** | 1 | Slow | Sun, Earth, Plant | Gather 1 Blight. **OR** Pay 1 Energy to Remove 1 Blight. | elements earth+plant+sun → 16.2 |
| 3 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear and 1 Damage. | elements plant+sun → 14.7 |
| 4 | **Enticing Splendor** | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer/Town. **OR** Gather up to 2 Dahan. | elements plant+sun → 14.7 |
| 5 | **Like Calls to Like** | 1 | Slow | Sun, Water, Plant | If target land has Explorer, Gather up to 1 Explorer. Do likewise for Town, Dahan, Blight… | elements plant+sun → 14.7 |
| 6 | **Song of Sanctity** | 1 | Slow | Sun, Water, Plant | If Explorer(s) are present, Push all Explorers. Otherwise, Remove 1 Blight. | elements plant+sun → 14.7 |
| 7 | **Gift of Living Energy** | 0 | Fast | Sun, Fire, Plant | Target Spirit gains 1 Energy. If you have at least 2 Sacred Sites, target Spirit gains 1 … | elements plant+sun → 14.7 |
| 8 | **Pact of the Joined Hunt** | 1 | Slow | Sun, Plant, Animal | Target Spirit Gathers 1 Dahan into one of their lands. 1 Damage in that land per Dahan pr… | elements plant+sun → 14.7 |
| 9 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements plant+sun → 14.7 |
| 10 | **Teeming Rivers** | 1 | Slow | Sun, Water, Plant, Animal | If target land has no Blight, add 1 Beasts. If target land has exactly 1 Blight, Remove i… | elements plant+sun → 14.7 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements earth+moon+plant+sun → 18.3 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements earth+moon+plant+sun → 18.3 |
| 3 | **Trees Radiate Celestial Brilliance** | 3 | Fast | Sun, Moon, Plant | 3 Fear. Defend 6. This turn, Invaders in target land skip the next Build Action. | elements moon+plant+sun → 16.8 |
| 4 | **The Trees and Stones Speak of War** | 2 | Fast | Sun, Earth, Plant | For each Dahan, 1 Damage and Defend 2. | elements earth+plant+sun → 16.2 |
| 5 | **Walls of Rock and Thorn** | 4 | Fast | Sun, Earth, Plant | 2 Damage. Defend 8. Add 1 Wilds. Isolate target land. | elements earth+plant+sun → 16.2 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Towering Roots of the Jungle in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opening Strategy

`[VERIFY: needs play data]` — opening variants should be rehearsed turn-by-turn per the [SPIRIT_TEMPLATE.md](../../../templates/SPIRIT_TEMPLATE.md) opener format.

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


### Strategy Cliffs — per-adversary-level shifts that change Towering Roots of the Jungle's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Towering Roots of the Jungle's profile (Fear 2, Offense 1, Control 3, Defense 5, Utility 2).
```

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Towering Roots of the Jungle**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/towering-roots-of-the-jungle.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Towering Roots of the Jungle](https://spiritislandwiki.com/index.php?title=Towering_Roots_of_the_Jungle).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
