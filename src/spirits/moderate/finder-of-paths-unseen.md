# Finder of Paths Unseen

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 2                                        |
| Complexity            | Very High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense -1 · Control 5 · Fear 1 · Defense 2 · Utility 2             |
| Primary Elements      | Air, Moon, Sun, Water (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> All about moving the Invaders - and Dahan/Presence/Beast from time to time. Good at creating Invader-free "safe-zones," due to its many movement Powers and its capacity to Isolate. Can't afford to Destroy Invaders too often without a way to re-add Destroyed Presence, so either needs a big-hammer Major Power or to rely on its teammates for offense. Changes the topology of the board, which increases complexity for all players - particularly in larger games!

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in land #3. Put 1 Presence on any board in land #1. Note that you have 6 Unique Power Cards.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=ignorerange |
| G2 | first=addpresence1, second=new+1cardplay |
| G3 | first=gain1p, second=addpresence2 |
| G4 | first=addpresence, second=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: _(unknown)_
- **Card-play track**: blank, blank, energy1moon, blank, movepresair, blank, energy+1range+1, energy+1range+1text

## Core Mechanics & Special Rules

### Special Rule

RESPONSIBILITIES TO THE DEAD After one of your Actions Destroys 1 or more Dahan/Invaders, or directly triggers their Destruction by moving them, Destroy 1 of your Presence and lose 1 Energy. If you have no Energy to lose, Destroy another Presence. OPEN THE WAYS You may make up to two of your lands adjacent at a time. You may change which lands are adjacent once between Actions.

### Innate: LAY PATHS THEY CANNOT HELP BUT WALK

- **Speed**: fast · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Push up to half (rounded down) of Invaders from target land. Do likewise for Dahan, Presence, and Beast (each separately). |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Push up to 1 Invader/Dahan/Presence/Beast. |
| 3 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Repeat this Power. |


### Innate: CLOSE THE WAYS

- **Speed**: fast · **Range**: 1 · **Target**: any

_(no thresholds listed in Wiki)_


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Finder of Paths Unseen's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/finder-of-paths-unseen.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/finder-of-paths-unseen.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 4.2), **Moon** (wt 2.4), **Sun** (wt 1.2)
- **Mid-game energy estimate (T3–T5 avg)**: 0.0E
- **Power summary**: Offense -1 · Control 5 · Fear 1 · Defense 2 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+sun+water → 8.7 |
| 2 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. The next time an Invader is Destroyed in target land this turn, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 7.8 |
| 3 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 7.8 |
| 4 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+moon → 6.6 |
| 5 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+moon+water → 7.5 |
| 6 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town/Dahan. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land has any Beasts, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon → 6.6 |
| 7 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Terror Level 2: 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> or add 1 Strife.… | elements air+moon+water → 7.5 |
| 8 | **Disorienting Landscape** | 1 | Fast | Moon, Air, Plant | Push 1 [[Explorer]]. If target land is a Mountain or Jungle, add 1 [[Wilds]]. | elements air+moon → 6.6 |
| 9 | **Land of Haunts and Embers** | 0 | Fast | Moon, Fire, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. If Blight is present, 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push up to 2 Explo… | elements air+moon → 6.6 |
| 10 | **Veil the Night's Hunt** | 1 | Fast | Moon, Air, Animal | For each Dahan present, choose a different Invader. 1 Damage to each of those Invaders. *… | elements air+moon → 6.6 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+moon+water → 7.5 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+moon+sun+water → 8.7 |
| 3 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon+sun+water → 8.7 |
| 4 | **Terrifying Nightmares** | 4 | Fast | Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 4 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. | elements air+moon → 6.6 |
| 5 | **Sleep and Never Waken** | 3 | Fast | Moon, Air, Earth, Animal | Invaders skip all Actions in target land. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> this Power Removes. Remo… | elements air+moon → 6.6 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Finder of Paths Unseen in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/finder-of-paths-unseen.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence on your starting board in land #3. Put 1 Presence on any board in land #1. Note that you have 6 Unique Power Cards.
- **Starting income** (from `presence_energy_track[0]` = `—`, `presence_cardplay_track[0]` = `blank`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); ignorerange ((spirit-specific: `ignorerange` — consult spirit panel))
- **G2**: addpresence1 (Place 1 Presence from a track (Range 1)); new+1cardplay ((spirit-specific: `new+1cardplay` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G4**: addpresence ((spirit-specific: `addpresence` — consult spirit panel)); energy2 ((track slot showing 2 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

(not parsed)

### Card-play track

`blank · blank · energy1moon · blank · movepresair · blank · energy+1range+1 · energy+1range+1text` — CP as slots reveal: blank → blank → energy1moon → blank → movepresair → blank → energy+1range+1 → energy+1range+1text

### Innate Powers

- **LAY PATHS THEY CANNOT HELP BUT WALK** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Push up to half (rounded down) of Invaders from target land. Do likewise for Dahan, Presence, and Beast (each separately).
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Push up to 1 Invader/Dahan/Presence/Beast.
  - **L3** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: Repeat this Power.
- **CLOSE THE WAYS** (Speed: Fast · Range: 1 · Target: any)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon, Uniques give 0; need 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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


### Strategy Cliffs — per-adversary-level shifts that change Finder of Paths Unseen's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Finder of Paths Unseen's profile (Fear 1, Offense -1, Control 5, Defense 2, Utility 2).
```

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Finder of Paths Unseen**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/finder-of-paths-unseen.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Finder of Paths Unseen](https://spiritislandwiki.com/index.php?title=Finder_of_Paths_Unseen).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
