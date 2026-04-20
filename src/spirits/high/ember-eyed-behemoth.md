# Ember-Eyed Behemoth

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
| Power summary (1–5)   | Offense 5 · Control 1 · Fear 1 · Defense 1 · Utility 2             |
| Primary Elements      | _(unknown)_ (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Slowly but consistently stomps its Incarna around the island, smashing Invaders. (Dahan can keep clear, unless it really gets going.) Benefits from spread-out Sacred Site, both for targeting Powers and for moving its Incarna long distances. Adding Presence at its Incarna (which can count as Presence) can make it easier to get Presence into new lands, particularly non-Jungles.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence and {{incarna|behemoth}}, Unempowered ({{incarna|unempowered}}) side up, in the highest-numbered Wetland on your starting board that is adjacent to any Jungle. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=addpresence3junglepresence, second=addpresence0 |
| G3 | first=gain1p, second=addpresence1, third=energy3 |
| G4 | first=reclaimallfire, second=empowerbehemoth, third=moveincarnabehemoth |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, energy2fire, energy3, earth, energy4plant, energy5fire
- **Card-play track**: card1, card2, card2, card3, fireX, card4

## Core Mechanics & Special Rules

### Special Rule

THE BEHEMOTH RISES You have an Incarna ({{incarna|behemoth}}). Once per turn, during the Spirit, Fast, or Slow phase, you may either: * Push {{incarna|behemoth}}; or * Add or Move {{incarna|behemoth}} to any of your Sacred Site on the island. UNRELENTING STRIDES On any turn that you don't use Innate Powers, you may use The Behemoth Rises an additional time. (When you use an Innate Power, cover this Special Rule with a Reminder Marker; when you use this Special Rule, cover your Innate Power.)

### Innate: SMASH, STOMP, AND FLATTEN

- **Speed**: slow · **Range**: None · **Target**: behemoth

_(no thresholds listed in Wiki)_


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Ember-Eyed Behemoth's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/ember-eyed-behemoth.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/ember-eyed-behemoth.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: —
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 5 · Control 1 · Fear 1 · Defense 1 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** Add 1 Wilds. | 0-cost (always affordable) |
| 2 | **Savage Mawbeasts** | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | 0-cost (always affordable) |
| 3 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | 0-cost (always affordable) |
| 4 | **Territorial Strife** | 0 | Slow | Sun, Fire, Animal | 3 Damage to Explorers/Towns. **OR** Add 1 Strife. | 0-cost (always affordable) |
| 5 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | 0-cost (always affordable) |
| 6 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | Slow speed matches innate |
| 7 | **Call to Bloodshed** | 1 | Slow | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | Slow speed matches innate |
| 8 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | 0-cost (always affordable) |
| 9 | **Desiccating Winds** | 1 | Slow | Fire, Air, Earth | If target land has Badlands, 1 Damage. Add 1 Badlands. | Slow speed matches innate |
| 10 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | Slow speed matches innate |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Accelerated Rot** | 4 | Slow | Sun, Water, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 4 Damage. | Slow speed matches innate |
| 2 | **Angry Bears** | 3 | Slow | Sun, Fire, Animal | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 2 Damage. If no Beasts are present, add 1 Beasts. Otherwise, +2 Damage, and Push … | Slow speed matches innate |
| 3 | **Bombard with Boulders and Stinging Seeds** | 2 | Slow | Air, Earth, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 2 Damage.</br>Add 1 Badlands. | Slow speed matches innate |
| 4 | **Cleansing Floods** | 5 | Slow | Sun, Water | 4 Damage. Remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. | Slow speed matches innate |
| 5 | **Death Falls Gently from Open Blossoms** | 4 | Slow | Moon, Air, Plant | 4 Damage. If any Invaders remain, add 1 Disease. | Slow speed matches innate |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Ember-Eyed Behemoth in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Scour the Land** | adds Blight |
| **Land of Haunts and Embers** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Pillar of Living Flame** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
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
Auto-derived from `data/references/wiki/ember-eyed-behemoth.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence and {{incarna|behemoth}}, Unempowered ({{incarna|unempowered}}) side up, in the highest-numbered Wetland on your starting board that is adjacent to any Jungle. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: addpresence3junglepresence ((spirit-specific: `addpresence3junglepresence` — consult spirit panel)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1)); energy3 ((+3 Energy this turn — growth effect, not track reveal))
- **G4**: reclaimallfire ((spirit-specific: `reclaimallfire` — consult spirit panel)); empowerbehemoth ((spirit-specific: `empowerbehemoth` — consult spirit panel)); moveincarnabehemoth ((spirit-specific: `moveincarnabehemoth` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · energy2fire · energy3 · earth · energy4plant · energy5fire` — income as slots reveal: 0 → 1 → 2 → 3 → earth → 4 → 5

### Card-play track

`card1 · card2 · card2 · card3 · fireX · card4` — CP as slots reveal: 1 → 2 → 2 → 3 → fireX → 4

### Innate Powers

- **SMASH, STOMP, AND FLATTEN** (Speed: Slow · Range: ? · Target: behemoth)

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


### Strategy Cliffs — per-adversary-level shifts that change Ember-Eyed Behemoth's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Ember-Eyed Behemoth's profile (Fear 1, Offense 5, Control 1, Defense 1, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Ember-Eyed Behemoth**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Ember-Eyed Behemoth**: Front-load coastal defense or disruption before T3's first Ravage.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Ember-Eyed Behemoth**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/ember-eyed-behemoth.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Ember-Eyed Behemoth](https://spiritislandwiki.com/index.php?title=Ember-Eyed_Behemoth).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
