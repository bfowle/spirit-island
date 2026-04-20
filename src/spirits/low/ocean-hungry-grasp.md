# Ocean's Hungry Grasp

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 4 · Fear 4 · Defense 3 · Utility 2             |
| Primary Elements      | Water, Moon, Earth, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Extremely good at assaulting the coasts where the Invaders start out strong, but quite weak inland - the ocean is not accustomed to affecting events so far ashore. Its Presence shifts in and out like the tide, which can be tricky to manage, but permits re-positioning and tactical retreats or offensives in the hands of a skillful player. Has fairly inexpensive Unique Powers, but the energy gained from drowning Invaders can be necessary in stepping up to more potent Powers.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence onto your starting board: 1 in the Ocean, and 1 in a Coastal land of your choice.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=OceanG |
| G2 | first=OceanA, second=OceanA, third=energy1 |
| G3 | first=gain1p, second=Ocean, third=OceanP |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, moon, water, energy1, earth, water, energy2
- **Card-play track**: card1, card2, card2, card3, card4, card5

## Core Mechanics & Special Rules

### Special Rule

OCEAN IN PLAY You may add/move Presence into Oceans, but may not add/move Presence into Inland lands. On boards where you have 1 or more Presence, Oceans are treated as Coastal Wetlands for Spirit Powers and Blight. You Drown any Invaders or Dahan moved to those Oceans. DROWNING Destroy Drowned pieces, placing Drowned Invaders here. At any time you may exchange (X) Health of these Invaders for 1 Energy, where X = number of players. (Ignore modifiers to Invader Health.)

### Innate: POUND SHIPS TO SPLINTERS

- **Speed**: fast · **Range**: 0 · **Target**: coastal

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. |
| 3 | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. |


### Innate: OCEAN BREAKS THE SHORE

- **Speed**: slow · **Range**: 0 · **Target**: coastal

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Drown 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | You may instead Drown 1 City <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City">. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Also, Drown 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> / City. |


## Unique Cards (all, Wiki-verified)

#### Call of the Deeps

- **0 Energy · Fast · Range 0 · Coastal Land · Moon, Air, Water**
- *Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. If target land is the Ocean, you may Gather another Explorer.*

#### Grasping Tide

- **1 Energy · Fast · Range 1 · Coastal Land · Moon, Water**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Defend 4.*

#### Swallow the Land-Dwellers

- **0 Energy · Slow · Range 0 · Coastal Land · Water, Earth**
- *Drown 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">, 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">, and 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.*

#### Tidal Boon

- **1 Energy · Slow · Range No Range · Another Spirit · Moon, Water, Earth**
- *Target Spirit gains 2 Energy and may Push 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> and up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> from one of their lands. If Dahan are pushed to your Ocean, you may move them to any Coastal land instead of Drowning them.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Ocean's Hungry Grasp's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/ocean-hungry-grasp.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/ocean-hungry-grasp.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Water** (wt 9.6), **Moon** (wt 3.0), **Earth** (wt 3.0)
- **Mid-game energy estimate (T3–T5 avg)**: 2.0E
- **Power summary**: Offense 5 · Control 4 · Fear 4 · Defense 3 · Utility 2
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Call of the Deeps**
- **Grasping Tide**
- **Swallow the Land-Dwellers**
- **Tidal Boon**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sucking Ooze** | 0 | Fast | Moon, Water, Earth | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if Invaders are present. Isolate target land. | elements earth+moon+water → 15.6 |
| 2 | **Infested Aquifers** | 1 | Slow | Moon, Water, Earth, Animal | If target land has any Disease, 1 Damage to each Invader. **OR** If target land is a Moun… | elements earth+moon+water → 15.6 |
| 3 | **Pull Beneath the Hungry Earth** | 1 | Slow | Moon, Water, Earth | If your Presence is present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. If target land is a Sands or Wetland, 1… | elements earth+moon+water → 15.6 |
| 4 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+water → 14.7 |
| 5 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+moon+water → 14.7 |
| 6 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon+water → 15.6 |
| 7 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+earth+water → 14.7 |
| 8 | **Call to Trade** | 1 | Fast | Air, Water, Earth, Plant | You may Gather 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. If the Terror Level is 2 or lower, Gather 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> and the first Ra… | elements air+earth+water → 14.7 |
| 9 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Terror Level 2: 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> or add 1 Strife.… | elements air+moon+water → 14.7 |
| 10 | **Sky Stretches to Shore** | 1 | Fast | Sun, Air, Water, Earth | This turn, target Spirit may use 1 Slow Power as if it were Fast, or vice versa. Target S… | elements air+earth+water → 14.7 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+earth+moon+water → 17.7 |
| 2 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+earth+moon+water → 17.7 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+moon+water → 17.7 |
| 4 | **Melt Earth Into Quicksand** | 4 | Fast | Moon, Water, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 2 Damage. Isolate target land. After Invaders/Dahan are Moved into target land, D… | elements earth+moon+water → 15.6 |
| 5 | **Mists of Oblivion** | 4 | Slow | Moon, Air, Water | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Town/City this Power Destroys (max. 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">). 1 Damage to each Invader. | elements air+moon+water → 14.7 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Ocean's Hungry Grasp in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Renewing Boon** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
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
Auto-derived from `data/references/wiki/ocean-hungry-grasp.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence onto your starting board: 1 in the Ocean, and 1 in a Coastal land of your choice.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); OceanG ((spirit-specific: `OceanG` — consult spirit panel))
- **G2**: OceanA ((spirit-specific: `OceanA` — consult spirit panel)); OceanA ((spirit-specific: `OceanA` — consult spirit panel)); energy1 ((track slot showing 1 Energy))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); Ocean ((spirit-specific: `Ocean` — consult spirit panel)); OceanP ((spirit-specific: `OceanP` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · moon · water · energy1 · earth · water · energy2` — income as slots reveal: 0 → moon → water → 1 → earth → water → 2

### Card-play track

`card1 · card2 · card2 · card3 · card4 · card5` — CP as slots reveal: 1 → 2 → 2 → 3 → 4 → 5

### Innate Powers

- **POUND SHIPS TO SPLINTERS** (Speed: Fast · Range: 0 · Target: coastal)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.
  - **L3** — 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.
- **OCEAN BREAKS THE SHORE** (Speed: Slow · Range: 0 · Target: coastal)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Drown 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">.
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: You may instead Drown 1 City <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City">.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth: Also, Drown 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> / City.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×2, **Air** ×1, **Water** ×2
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Call of the Deeps** | 0 | Fast | 0 | Coastal Land | moon, air, water | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. If target land is the Ocean, you may Gather another Explorer. |
| **Grasping Tide** | 1 | Fast | 1 | Coastal Land | moon, water | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Defend 4. |
| **Swallow the Land-Dwellers** | 0 | Slow | 0 | Coastal Land | water, earth | Drown 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">, 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">, and 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| **Tidal Boon** | 1 | Slow | No Range | Another Spirit | moon, water, earth | Target Spirit gains 2 Energy and may Push 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> and up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> from one of their lands. If Daha… |

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


### Strategy Cliffs — per-adversary-level shifts that change Ocean's Hungry Grasp's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Ocean's Hungry Grasp's profile (Fear 4, Offense 5, Control 4, Defense 3, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Ocean's Hungry Grasp**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Ocean's Hungry Grasp**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Ocean's Hungry Grasp**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Ocean's Hungry Grasp**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Ocean's Hungry Grasp**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Ocean's Hungry Grasp**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Ocean's Hungry Grasp**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/oceans-hungry-grasp.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Ocean's Hungry Grasp](https://spiritislandwiki.com/index.php?title=Ocean's_Hungry_Grasp).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
