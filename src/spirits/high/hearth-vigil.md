# Hearth-Vigil

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
| Power summary (1–5)   | Offense 3 · Control 1 · Fear 2 · Defense 4 · Utility 4             |
| Primary Elements      | Sun, Earth, Animal, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very good at protecting Dahan in its lands, not so great at stopping Blight. In keeping with its nature, largely brings Dahan to its Presence (or vice versa); getting Dahan elsewhere may require a bit of forethought with Keep Watch for New Incursions. Very reactive, with reliable ways to deal with Invaders as they Ravage and Build, but has trouble handling established City that aren't Ravaging.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the highest-numbered land with Dahan and 2 in the lowest-numbered land with at least 2 Dahan. Add 1 Dahan in each of those lands (additional survivors of the Invaders' diseases). You start with your 4 Unique Power Cards and 1 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addpresence0 |
| G2 | first=gain1p, second=addpresence3dahan |
| G3 | first=addpresence2, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: gather1dahan1land
- **Card-play track**: energy0, energy1sun, energy2, energy3animal, energy4, energy5sun

## Core Mechanics & Special Rules

### Special Rule

ROOTED IN THE COMMUNITY Blight added in your lands does not Destroy your Presence if Dahan are present. (Ravage Actions Destroy Dahan before added Blight destroys Presence and cascades.) FORTIFY HEART AND HEARTH Dahan have +4 Health (each) while in your lands. Event and Blight Card Actions don't damage, destroy, or replace Dahan in your lands. (Ravages are not Event Actions even if caused by Events.) LOYAL GUARDIAN When all Dahan leave one of your lands, your Presence may Move with those Dahan. (Each Dahan can Bring any number of Presence.)

### Innate: WARN OF IMPENDING CONFLICT

- **Speed**: fast · **Range**:  · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Sun + 1 Earth | In one of your lands, 1 Dahan deals Damage before Invaders during Ravages. (Choose a land when Invaders Ravage there.) |
| 2 | 3 Sun + 1 Earth | In that land, another Dahan deals Damage before Invaders during Ravages. |
| 3 | 4 Sun + 2 Earth | In that land, all Dahan deal Damage before Invaders during Ravages. |
| 4 | 5 Sun + 3 Earth | Instead, all Dahan in all of your lands deal Damage before Invaders during Ravages. |


### Innate: KEEP WATCH FOR NEW INCURSIONS

- **Speed**: fast · **Range**: 1 (optionally from a sacredsitedahan site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Animal | Gather up to 2 Dahan, from your lands only. |
| 2 | 1 Sun + 2 Air + 3 Animal | Once this turn after Invaders are added or moved into target land, 1 Damage per Dahan in target land, to those added/moved Invaders only. |
| 3 | 2 Sun + 3 Air + 4 Animal | Repeat this Power. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Hearth-Vigil's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/hearth-vigil.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/hearth-vigil.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Sun** (wt 7.5), **Animal** (wt 3.9), **Earth** (wt 3.0)
- **Mid-game energy estimate (T3–T5 avg)**: 0.0E
- **Power summary**: Offense 3 · Control 1 · Fear 2 · Defense 4 · Utility 4
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Promises of Protection** | 0 | Fast | Sun, Earth, Animal | Gather up to 2 Dahan. Dahan have +2 Health while in target land. | elements animal+earth+sun → 14.4 |
| 2 | **Guardian Serpents** | 1 | Fast | Sun, Moon, Earth, Animal | Add 1 Beasts in one of target Spirit's lands. If target Spirit has a Sacred Site in that … | elements animal+earth+sun → 14.4 |
| 3 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements air+earth+sun → 12.6 |
| 4 | **Call to Isolation** | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. | elements air+animal+sun → 13.5 |
| 5 | **Birds Cry Warning** | 1 | Fast | Sun, Air, Animal | The next time Dahan would be Destroyed in target land, Destroy 2 fewer Dahan. **OR** Push… | elements air+animal+sun → 13.5 |
| 6 | **Sky Stretches to Shore** | 1 | Fast | Sun, Air, Water, Earth | This turn, target Spirit may use 1 Slow Power as if it were Fast, or vice versa. Target S… | elements air+earth+sun → 12.6 |
| 7 | **Territorial Strife** | 0 | Slow | Sun, Fire, Animal | 3 Damage to Explorers/Towns. **OR** Add 1 Strife. | elements animal+sun → 11.4 |
| 8 | **Gift of Constancy** | 0 | Fast | Sun, Earth | Target Spirit gains 2 Energy. At end of turn, target Spirit may Reclaim 1 Power Card inst… | elements earth+sun → 10.5 |
| 9 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements animal+sun → 11.4 |
| 10 | **Strong and Constant Currents** | 0 | Fast | Sun, Water, Earth | Push 1 Explorer/Town to an adjacent Coastal land. **OR** Move up to 2 Dahan between targe… | elements earth+sun → 10.5 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+earth+sun → 16.5 |
| 2 | **Bargains of Power and Protection** | 2 | Fast | Sun, Water, Earth, Animal | Remove 1 of your Presence on the island from the game, setting it on the Reminder Card. F… | elements animal+earth+sun → 14.4 |
| 3 | **Wrap in Wings of Sunlight** | 3 | Fast | Sun, Air, Animal | Move up to 5 Dahan to any land (including back into target land). If you moved at least 1… | elements air+animal+sun → 13.5 |
| 4 | **Instruments of Their Own Ruin** | 4 | Fast | Sun, Fire, Air, Animal | Add 1 Strife. Each Invader with Strife deals Damage to other Invaders in target land. | elements air+animal+sun → 13.5 |
| 5 | **Manifest Incarnation** | 6 | Slow | Sun, Moon, Earth, Animal | 6 Fear. +1 Fear for each Town/City and for each of your Presence in target land. Remove 1… | elements animal+earth+sun → 14.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Hearth-Vigil in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Land of Haunts and Embers** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/hearth-vigil.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the highest-numbered land with Dahan and 2 in the lowest-numbered land with at least 2 Dahan. Add 1 Dahan in each of those lands (additional survivors of the Invaders' diseases). You start with your 4 Unique Power Cards and 1 Energy.
- **Starting income** (from `presence_energy_track[0]` = `gather1dahan1land`, `presence_cardplay_track[0]` = `energy0`): **0 Energy · 0 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addpresence0 (Place 1 Presence from a track (Range 0))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3dahan ((spirit-specific: `addpresence3dahan` — consult spirit panel))
- **G3**: addpresence2 (Place 1 Presence from a track (Range 2)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`gather1dahan1land` — income as slots reveal: gather1dahan1land

### Card-play track

`energy0 · energy1sun · energy2 · energy3animal · energy4 · energy5sun` — CP as slots reveal: energy0 → energy1sun → energy2 → energy3animal → energy4 → energy5sun

### Innate Powers

- **WARN OF IMPENDING CONFLICT** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 2 Sun + 1 Earth: In one of your lands, 1 Dahan deals Damage before Invaders during Ravages. (Choose a land when Invaders Ravage there.)
  - **L2** — 3 Sun + 1 Earth: In that land, another Dahan deals Damage before Invaders during Ravages.
  - **L3** — 4 Sun + 2 Earth: In that land, all Dahan deal Damage before Invaders during Ravages.
  - **L4** — 5 Sun + 3 Earth: Instead, all Dahan in all of your lands deal Damage before Invaders during Ravages.
- **KEEP WATCH FOR NEW INCURSIONS** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 1 Animal: Gather up to 2 Dahan, from your lands only.
  - **L2** — 1 Sun + 2 Air + 3 Animal: Once this turn after Invaders are added or moved into target land, 1 Damage per Dahan in target land, to those added/moved Invaders only.
  - **L3** — 2 Sun + 3 Air + 4 Animal: Repeat this Power.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Sun, Uniques give 0; need 1 Earth, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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


### Strategy Cliffs — per-adversary-level shifts that change Hearth-Vigil's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Hearth-Vigil's profile (Fear 2, Offense 3, Control 1, Defense 4, Utility 4).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Hearth-Vigil**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Hearth-Vigil**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Hearth-Vigil**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/hearth-vigil.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Hearth-Vigil](https://spiritislandwiki.com/index.php?title=Hearth-Vigil).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
