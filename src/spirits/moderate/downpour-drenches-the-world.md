# Downpour Drenches the World

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 2                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3             |
| Primary Elements      | Water, Earth, Plant, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Cares about the question "How useful is this Power in the current context?" even more than most Spirits; it rarely plays all its Power Cards in any given Reclaim cycle (some get discarded to Growth), and for those it does play, it often has the option of using them multiple times. Benefits even more than most Spirits from having lots of Presence on the board, both for Rain and Mud Suppress Conflict and to facilitate its Unique Powers (by making more lands Wetlands).

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the lowest-numbered Wetlands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=movepresence2 |
| G2 | first=addpresence2, second=addpresence2, third=gain2water |
| G3 | first=gain1p, second=addpresence3, third=energy1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, water, plant, water, energy2air, water, earth, doublewater
- **Card-play track**: card1, movepres, waterX, card2, movepres, card3

## Core Mechanics & Special Rules

### Special Rule

DRENCH THE LANDSCAPE Spirit Actions and Special Rules treat your Sacred Site as Wetlands in addition to the printed terrain. POUR DOWN POWER ACROSS THE ISLAND For each 2 Water you have, during the Fast/Slow phase you may either: * Gain 1 Energy; or * Repeat a land-targeting Power Card by paying its cost again. (It need not target the same land.) Use scenario markers or spare game pieces to track uses of this rule. (Max 5 times per turn, no matter how much Water you have.)

### Innate: RAIN AND MUD SUPPRESS CONFLICT

- **Speed**: fast · **Range**: None · **Target**: you

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Air + 3 Water | Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.) |
| 2 | 5 Water + 1 Earth | Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. |
| 3 | 3 Air + 9 Water + 2 Earth | 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1). |


### Innate: WATER NOURISHES LIFE'S GROWTH

- **Speed**: fast · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Water + 2 Plant | Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (From target land). |
| 2 | 5 Water + 1 Earth + 2 Plant | Gain +1 Energy. Gather up to 1 Dahan. |
| 3 | 7 Water + 2 Earth + 3 Plant | When Blight would be added to target land, instead leave it on the card. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Downpour Drenches the World's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/downpour-drenches-the-world.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/downpour-drenches-the-world.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Water** (wt 16.2), **Plant** (wt 3.9), **Earth** (wt 2.4)
- **Mid-game energy estimate (T3–T5 avg)**: 2.0E
- **Power summary**: Offense 2 · Control 3 · Fear 1 · Defense 5 · Utility 3
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Call to Trade** | 1 | Fast | Air, Water, Earth, Plant | You may Gather 1 Dahan. If the Terror Level is 2 or lower, Gather 1 Town and the first Ra… | elements air+earth+plant+water → 24.3 |
| 2 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+plant+water → 22.5 |
| 3 | **Flow Downriver, Blow Downwind** | 0 | Slow | Air, Water, Plant | Push up to 1 Blight/Explorer/Town. | elements air+plant+water → 21.9 |
| 4 | **Renewing Rain** | 1 | Slow | Water, Earth, Plant | If target land is a Jungle or Sands, Remove 1 Blight. | elements earth+plant+water → 22.5 |
| 5 | **The Shore Seethes with Hatred** | 1 | Slow | Fire, Water, Earth, Plant | 1 Fear. Add 1 Badlands and 1 Wilds. | elements earth+plant+water → 22.5 |
| 6 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements plant+water → 20.1 |
| 7 | **Sky Stretches to Shore** | 1 | Fast | Sun, Air, Water, Earth | This turn, target Spirit may use 1 Slow Power as if it were Fast, or vice versa. Target S… | elements air+earth+water → 20.4 |
| 8 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer. | elements air+earth+water → 20.4 |
| 9 | **Mesmerized Tranquility** | 0 | Fast | Water, Earth, Animal | Isolate target land. Each Invader does -1 Damage. | elements earth+water → 18.6 |
| 10 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear if Invaders are present. Isolate target land. | elements earth+water → 18.6 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+plant+water → 24.3 |
| 2 | **Flocking Red-Talons** | 3 | Fast | Air, Water, Plant, Animal | Add 1 Beasts. Move up to 2 Beasts within 3 Range to target land. For each Beasts present,… | elements air+plant+water → 21.9 |
| 3 | **Dream of the Untouched Land** | 6 | Fast | Moon, Water, Earth, Plant, Animal | Remove up to 3 Blight and up to 3 Health worth of Invaders. | elements earth+plant+water → 22.5 |
| 4 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+earth+water → 20.4 |
| 5 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+earth+water → 20.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Downpour Drenches the World in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Renewing Boon** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Land of Haunts and Embers** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Tsunami** | destroys Dahan |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/downpour-drenches-the-world.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in the lowest-numbered Wetlands.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); movepresence2 (Move 1 Presence (Range 2))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2)); addpresence2 (Place 1 Presence from a track (Range 2)); gain2water ((spirit-specific: `gain2water` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); energy1 ((track slot showing 1 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · water · plant · water · energy2air · water · earth · doublewater` — income as slots reveal: 1 → water → plant → water → 2 → water → earth → doublewater

### Card-play track

`card1 · movepres · waterX · card2 · movepres · card3` — CP as slots reveal: 1 → movepres → waterX → 2 → movepres → 3

### Innate Powers

- **RAIN AND MUD SUPPRESS CONFLICT** (Speed: Fast · Range: ? · Target: you)
  - **L1** — 1 Air + 3 Water: Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.)
  - **L2** — 5 Water + 1 Earth: Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1.
  - **L3** — 3 Air + 9 Water + 2 Earth: 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1).
- **WATER NOURISHES LIFE'S GROWTH** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — 3 Water + 2 Plant: Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (From target land).
  - **L2** — 5 Water + 1 Earth + 2 Plant: Gain +1 Energy. Gather up to 1 Dahan.
  - **L3** — 7 Water + 2 Earth + 3 Plant: When Blight would be added to target land, instead leave it on the card.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Air, Uniques give 0; need 3 Water, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

*No Unique cards parsed.*

### Invader phase by turn (base deck)

| Turn | Explore | Build | Ravage | Notes |
|------|---------|-------|--------|-------|
| 1 | ✓ | — | — | Ravage-protection effects are **dormant T1**. |
| 2 | ✓ | ✓ | — | First Build; Ravage-protection still dormant. |
| 3 | ✓ | ✓ | ✓ | First Ravage; Ravage-protection becomes material. |
| 4+ | ✓ | ✓ | ✓ | Full cycle continues. |

Adversary escalation can shift this — check the adversary JSON for deviations (Sweden front-loads a Build; some Habsburg levels add early Builds).

### Pause-point before writing T1 prose

```admonish warning title="Before claiming what T1 does"
1. **Compute post-growth E/CP** for every growth × track-choice branch. Don't assume both tracks reveal simultaneously.
2. **Enumerate legal T1 plays** — subsets of hand with sum(costs) ≤ E and count ≤ CP.
3. **Separate Fast vs. Slow elements** — when claiming an innate fires, verify the threshold is met using only elements from its resolution phase (Fast sees Fast; Slow sees Fast + Slow).
4. **Flag dormant effects** — Ravage-protection, Defend N, etc. are **null T1/T2** in base play. Only cite them as opener value when the trigger actually occurs that turn.
5. **State per-turn material effect** for every card play: Fear generated, units pushed/gathered/destroyed, elements contributed. Never narrate dormant effects as if they were active.
```

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


### Strategy Cliffs — per-adversary-level shifts that change Downpour Drenches the World's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Downpour Drenches the World's profile (Fear 1, Offense 2, Control 3, Defense 5, Utility 3).
```

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Downpour Drenches the World**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Downpour Drenches the World**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/downpour-drenches-the-world.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Downpour Drenches the World](https://spiritislandwiki.com/index.php?title=Downpour_Drenches_the_World).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
