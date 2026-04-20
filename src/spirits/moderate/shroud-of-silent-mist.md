# Shroud of Silent Mist

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 4 · Fear 5 · Defense 2 · Utility 1             |
| Primary Elements      | Air, Water, Moon (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Constantly shifting and moving its Presence around the board. Hurt more than most by Presence loss due to its desire to surround and envelop the Invaders. Can (slowly) clear the most built-up of lands, but its real strength is the free Fear from Slow and Silent Death. Extremely limited Energy income, but can stretch to Major Powers if it manages to gather enough Energy from its Special Rules.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board: 1 in the highest-numbered Wetland and 1 in the highest-numbered Mountain.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=addpresence0, second=addpresence0 |
| G3 | first=gain1p, second=addpresence3mw |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, water, energy2, air
- **Card-play track**: card1, card2, movepres, moonX, card3, card4, reclaim1, card5

## Core Mechanics & Special Rules

### Special Rule

GATHER POWER FROM THE COOL AND DARK Once a turn, when you Gain a Power Card without Simplefire, gain 1 Energy. MISTS SHIFT AND FLOW When targeting a land with a Power, you may Gather 1 of your Presence into the target or an adjacent land. This can enable you to meet Range and targeting requirements. SLOW AND SILENT DEATH Invaders and Dahan in your lands don't heal Damage. During Time Passes: 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> (max. 5) per land of yours with Damaged Invaders. Gain 1 Energy per 3 lands of yours with Damaged Invaders.

### Innate: SUFFOCATING SHROUD

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | For each adjacent land with your Presence, 1 Damage to a different Invader. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage. |
| 4 | 5 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 6 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage to each Invader. |


### Innate: LOST IN THE SWIRLING HAZE

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Dahan. |
| 3 | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Dahan. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Shroud of Silent Mist's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/shroud-of-silent-mist.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/shroud-of-silent-mist.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 9.6), **Water** (wt 9.0), **Moon** (wt 4.8)
- **Mid-game energy estimate (T3–T5 avg)**: 2.0E
- **Power summary**: Offense 4 · Control 4 · Fear 5 · Defense 2 · Utility 1
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+water → 23.4 |
| 2 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Terror Level 2: 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> or add 1 Strife.… | elements air+moon+water → 23.4 |
| 3 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+moon+water → 23.4 |
| 4 | **Rain of Blood** | 0 | Slow | Air, Water, Animal | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land has at least 2 Towns <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">/Cities, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+water → 18.6 |
| 5 | **Flow Downriver, Blow Downwind** | 0 | Slow | Air, Water, Plant | Push up to 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">/Explorer/Town. | elements air+water → 18.6 |
| 6 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Disease. | elements air+water → 18.6 |
| 7 | **Call to Trade** | 1 | Fast | Air, Water, Earth, Plant | You may Gather 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. If the Terror Level is 2 or lower, Gather 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> and the first Ra… | elements air+water → 18.6 |
| 8 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+water → 18.6 |
| 9 | **Reaching Grasp** | 0 | Fast | Sun, Air, Water | Target Spirit gets +2 Range with all their Powers. | elements air+water → 18.6 |
| 10 | **Confounding Mists** | 1 | Fast | Air, Water | Defend 4. **OR** Each Invader added to target land this turn may be immediately Pushed to… | elements air+water → 18.6 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Mists of Oblivion** | 4 | Slow | Moon, Air, Water | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Town/City this Power Destroys (max. 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">). 1 Damage to each Invader. | elements air+moon+water → 23.4 |
| 2 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+moon+water → 23.4 |
| 3 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon+water → 23.4 |
| 4 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+moon+water → 23.4 |
| 5 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+moon+water → 23.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Shroud of Silent Mist in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Tsunami** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Blazing Renewal** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/shroud-of-silent-mist.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in the highest-numbered Wetland and 1 in the highest-numbered Mountain.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: addpresence0 (Place 1 Presence from a track (Range 0)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3mw ((spirit-specific: `addpresence3mw` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · water · energy2 · air` — income as slots reveal: 0 → 1 → water → 2 → air

### Card-play track

`card1 · card2 · movepres · moonX · card3 · card4 · reclaim1 · card5` — CP as slots reveal: 1 → 2 → movepres → moonX → 3 → 4 → reclaim1 → 5

### Innate Powers

- **SUFFOCATING SHROUD** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: 1 Damage.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: For each adjacent land with your Presence, 1 Damage to a different Invader.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: 1 Damage.
  - **L4** — 5 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 6 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: 1 Damage to each Invader.
- **LOST IN THE SWIRLING HAZE** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: Push up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: Push up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Dahan.
  - **L3** — 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: Push up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Dahan.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_

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


### Strategy Cliffs — per-adversary-level shifts that change Shroud of Silent Mist's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Shroud of Silent Mist's profile (Fear 5, Offense 4, Control 4, Defense 2, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Shroud of Silent Mist**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Shroud of Silent Mist**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Shroud of Silent Mist**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Shroud of Silent Mist**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Shroud of Silent Mist**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Shroud of Silent Mist**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Shroud of Silent Mist**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/shroud-of-silent-mist.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Shroud of Silent Mist](https://spiritislandwiki.com/index.php?title=Shroud_of_Silent_Mist).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
