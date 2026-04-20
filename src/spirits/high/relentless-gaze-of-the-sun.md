# Relentless Gaze of the Sun

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "always" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 1 · Fear 3 · Defense 2 · Utility 3             |
| Primary Elements      | Sun, Fire, Air, Water (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Uses stacks of 3 Presence and high Energy income to hammer the same lands with repeated Power Cards. Because of its single-mindedness, is better at dealing with large problems than smaller ones. Harms the land and Dahan while smiting Invaders unless it successfully changes its nature (by expanding into new Elements), which may mean partially foregoing the Sun and Fire that form the core of its initial strength.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence and 1 Badlands on your starting board, in the lowest-numbered Sands. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (always)

| Growth | Effects |
|--------|---------|
| G1 | first=Gaze |
| G2 | first=reclaim, second=add3destroyedpresencetogether |
| G3 | first=gain1p |
| G4 | first=gaindoubleenergy, second=move3presencetogether |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2sun, energy3fire, sun, energy4any, energy5, 
- **Card-play track**: card1, card1, card2, sunX, card3, reclaim1, card4

## Core Mechanics & Special Rules

### Special Rule

RELENTLESS PUNISHMENT After using a Power Card, if you had at least 3 Presence in the origin land, you may Repeat it any number of times on the same target land(s) (ignoring origin, Range, and target requirements) by paying both: * the Energy cost of the Power, and * 1 Energy per previous use of the Power this turn each time you Repeat the Power. (You may Repeat each Power Card multiple times, but its 1st Repeat costs 1 extra, its 2nd Repeat costs 2 extra, etc. Check if the origin has at least 3 Presence when you target it the first time.)

### Innate: SCORCHING CONVERGENCE

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun | Move all of your Presence from origin land directly to target land. 1 Damage, to Town/City only. |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | 3 Damage to Invaders. 3 Damage to Dahan. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> without cascading. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if this Power destroyed any Invaders. |
| 4 | 5 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Damage per remaining Presence of yours in target land. |


### Innate: CONSIDER A HARMONIOUS NATURE

- **Speed**: fast · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | When your Powers would Add Blight, you may Destroy 1 Presence instead (there or elsewhere). |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Your Powers don't damage or destroy Dahan. |
| 3 | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Choose another Spirit. They Add 1 {{destroyedpresence}} to one of your lands. |
| 4 | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Give up to 3 of your Energy to the chosen Spirit. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Relentless Gaze of the Sun's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/relentless-gaze-of-the-sun.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/relentless-gaze-of-the-sun.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Sun** (wt 12.6), **Fire** (wt 2.1), **Air** (wt 0.9)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 5 · Control 1 · Fear 3 · Defense 2 · Utility 3
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sunset's Fire Flows Across the Land** | 1 | Slow | Sun, Moon, Fire, Water | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. You may pay 1 Energy to deal 1 Damage in an adjacent land. | elements fire+moon+sun+water → 16.5 |
| 2 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire+plant+sun → 16.2 |
| 3 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | elements fire+plant+sun → 15.3 |
| 4 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+sun+water → 15.3 |
| 5 | **Territorial Strife** | 0 | Slow | Sun, Fire, Animal | 3 Damage to Explorers/Towns. **OR** Add 1 Strife. | elements fire+sun → 14.7 |
| 6 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements fire+sun+water → 15.6 |
| 7 | **Elusive Ambushes** | 1 | Fast | Sun, Fire, Water | 1 Damage. **OR** Defend 4. | elements fire+sun+water → 15.6 |
| 8 | **Call to Bloodshed** | 1 | Slow | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | elements fire+sun → 14.7 |
| 9 | **Drought** | 1 | Slow | Sun, Fire, Earth | Destroy 3 [[Towns]]. 1 Damage to each [[Town]]/[[City]]. Add 1 [[Blight]]. | elements fire+sun → 14.7 |
| 10 | **Call to Ferocity** | 0 | Slow | Sun, Fire, Earth | Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. **OR** If target land has Dahan, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 T… | elements fire+sun → 14.7 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+fire+moon+plant+sun+water → 18.0 |
| 2 | **Exaltation of the Incandescent Sky** | 7 | Fast | Sun, Fire, Air, Water | Target Spirit may play 1 Power Card by paying its cost, make up to 2 of their Powers Fast… | elements air+fire+sun+water → 16.5 |
| 3 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements fire+plant+sun → 15.3 |
| 4 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+moon+plant+sun → 15.0 |
| 5 | **Unrelenting Growth** | 4 | Slow | Sun, Fire, Water, Plant | Target Spirit adds 2 Presence and 1 Wilds to a land at Range 1 of their Presence. | elements fire+plant+sun+water → 16.2 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Relentless Gaze of the Sun in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Tsunami** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/relentless-gaze-of-the-sun.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence and 1 Badlands on your starting board, in the lowest-numbered Sands. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `always` — (see spirit panel)

### Growth options

- **G1**: Gaze ((spirit-specific: `Gaze` — consult spirit panel))
- **G2**: reclaim (Reclaim all discarded+played Power Cards); add3destroyedpresencetogether ((spirit-specific: `add3destroyedpresencetogether` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G4**: gaindoubleenergy ((spirit-specific: `gaindoubleenergy` — consult spirit panel)); move3presencetogether ((spirit-specific: `move3presencetogether` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2sun · energy3fire · sun · energy4any · energy5 · ` — income as slots reveal: 1 → 2 → 3 → sun → 4 → 5 → 

### Card-play track

`card1 · card1 · card2 · sunX · card3 · reclaim1 · card4` — CP as slots reveal: 1 → 1 → 2 → sunX → 3 → reclaim1 → 4

### Innate Powers

- **SCORCHING CONVERGENCE** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun: Move all of your Presence from origin land directly to target land. 1 Damage, to Town/City only.
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire: 3 Damage to Invaders. 3 Damage to Dahan. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> without cascading.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if this Power destroyed any Invaders.
  - **L4** — 5 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: 1 Damage per remaining Presence of yours in target land.
- **CONSIDER A HARMONIOUS NATURE** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon: When your Powers would Add Blight, you may Destroy 1 Presence instead (there or elsewhere).
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water: Your Powers don't damage or destroy Dahan.
  - **L3** — 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Choose another Spirit. They Add 1 {{destroyedpresence}} to one of your lands.
  - **L4** — 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Give up to 3 of your Energy to the chosen Spirit.

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


### Strategy Cliffs — per-adversary-level shifts that change Relentless Gaze of the Sun's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Relentless Gaze of the Sun's profile (Fear 3, Offense 5, Control 1, Defense 2, Utility 3).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Relentless Gaze of the Sun**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Relentless Gaze of the Sun**: Front-load coastal defense or disruption before T3's first Ravage.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Relentless Gaze of the Sun**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Relentless Gaze of the Sun**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Relentless Gaze of the Sun**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Relentless Gaze of the Sun**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/relentless-gaze-of-the-sun.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Relentless Gaze of the Sun](https://spiritislandwiki.com/index.php?title=Relentless_Gaze_of_the_Sun).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
