# A Spread of Rampant Green

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "always" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 3 · Fear 2 · Defense 5 · Utility 4             |
| Primary Elements      | Plant, Moon, Water, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Fairly good at dealing with Towns, but terrible at handling Explorers (who are unfazed by prolific foliage). Can get Presence onto the board faster than most other Spirits. Extra Presence is good for targeting and especially for 'Choke the Land with Green", which can be extremely effective at slowing down invaders. Just be careful not to destroy Sacred Sites needed for Power use.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board; 1 in the highest-numbered Wetland, and 1 in the Jungle without any Dahan. (If there is more than 1 such Jungle, you may choose)

## Growth Options (always)

| Growth | Effects |
|--------|---------|
| G1 | first=Spread |
| G2 | first=reclaim, second=gain1p |
| G3 | first=addpresence1, second=old+1cardplay |
| G4 | first=gain1p, second=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, plant, energy2, energy2, plant, energy3
- **Card-play track**: card1, card1, card2, card2, card3, card4

## Core Mechanics & Special Rules

### Special Rule

CHOKE THE LAND WITH GREEN Whenever Invaders would Ravage or Build in a land with your Sacred Site, you may prevent it by destroying one of your Presence in that land. STEADY REGENERATION When adding Presence to the board via Growth, you may optionally use your destroyed Presence. If the island is Healthy, do so freely. If the island is Blighted, doing so costs 1 Energy per destroyed Presence you add.

### Innate: CREEPERS TEAR INTO MORTAR

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 2 Plant | 1 Damage to 1 Town / City. |
| 2 | 2 Moon + 3 Plant | Repeat this Power. |
| 3 | 3 Moon + 4 Plant | Repeat this Power again. |


### Innate: ALL-ENVELOPING GREEN

- **Speed**: fast · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Water + 3 Plant | Defend 2. |
| 2 | 2 Water + 4 Plant | Instead, Defend 4. |
| 3 | 3 Water + 1 Earth + 5 Plant | Also, remove 1 Blight. |


## Unique Cards (all, Wiki-verified)

#### Fields Choked with Growth

- **0 Energy · Slow · Range 1 · Any Land · Sun, Water, Plant**
- *Push 1 Town. **OR** Push 3 Dahan.*

#### Gift of Proliferation

- **1 Energy · Fast · Range No Range · Another Spirit · Moon, Plant**
- *Target Spirit adds 1 Presence up to 1 Range from their Presence.*

#### Overgrow in a Night

- **2 Energy · Fast · Range 1 · Any Land · Moon, Plant**
- *Add 1 Presence. **OR** If target land has your Presence and Invaders, 3 Fear.*

#### Stem the Flow of Fresh Water

- **0 Energy · Slow · Range 1, from your Sacred Site · Any Land · Water, Plant**
- *1 Damage to 1 Town/City. If target land is a Mountain or Sands, instead, 1 Damage to each Town/City.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by A Spread of Rampant Green's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/a-spread-of-rampant-green.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/a-spread-of-rampant-green.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Plant** (wt 11.4), **Moon** (wt 3.0), **Water** (wt 3.0)
- **Mid-game energy estimate (T3–T5 avg)**: 2.33E
- **Power summary**: Offense 4 · Control 3 · Fear 2 · Defense 5 · Utility 4
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Fields Choked with Growth**
- **Gift of Proliferation**
- **Overgrow in a Night**
- **Stem the Flow of Fresh Water**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements moon+plant+water → 17.4 |
| 2 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements earth+moon+plant+water → 17.7 |
| 3 | **Thriving Chokefungus** | 1 | Slow | Moon, Water, Plant | Add 1 Disease and 1 Badlands. | elements moon+plant+water → 17.4 |
| 4 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear. Destroy 1 Explorer. **OR** Add 1 Wilds. | elements moon+plant → 14.4 |
| 5 | **Razor-Sharp Undergrowth** | 1 | Fast | Moon, Plant | Destroy 1 Explorer and 1 Dahan. Add 1 Wilds. Defend 2. | elements moon+plant → 14.4 |
| 6 | **Flow Downriver, Blow Downwind** | 0 | Slow | Air, Water, Plant | Push up to 1 Blight/Explorer/Town. | elements plant+water → 14.4 |
| 7 | **Shadows of the Burning Forest** | 0 | Slow | Moon, Fire, Plant | 2 Fear. If target land is a Mountain or Jungle, Push 1 Explorer and 1 Town. | elements moon+plant → 14.4 |
| 8 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear. If target land is a Mountain or Jungle, Defend 3. | elements earth+moon+plant → 14.7 |
| 9 | **Favor of the Sun and Star-lit Dark** | 1 | Fast | Sun, Moon, Plant | Defend 4. Push up to 1 Blight. | elements moon+plant → 14.4 |
| 10 | **Poisoned Dew** | 1 | Slow | Fire, Water, Plant | Destroy 1 Explorer. If target land is a Jungle or a Wetland, Destroy all Explorer. | elements plant+water → 14.4 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements moon+plant+water → 17.4 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements earth+moon+plant+water → 17.7 |
| 3 | **Entwined Power** | 2 | Fast | Moon, Water, Plant | You and target Spirit may use each other's Presence to target Powers (only). Target Spiri… | elements moon+plant+water → 17.4 |
| 4 | **Transformative Sacrifice** | 3 | Fast | Moon, Fire, Water, Plant | Target Spirit may Remove up to 3 Presence (from anywhere on the island). Then for each re… | elements moon+plant+water → 17.4 |
| 5 | **Dream of the Untouched Land** | 6 | Fast | Moon, Water, Earth, Plant, Animal | Remove up to 3 Blight and up to 3 Health worth of Invaders. | elements earth+moon+plant+water → 17.7 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with A Spread of Rampant Green in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/a-spread-of-rampant-green.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board; 1 in the highest-numbered Wetland, and 1 in the Jungle without any Dahan. (If there is more than 1 such Jungle, you may choose)
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `always` — (see spirit panel)

### Growth options

- **G1**: Spread ((spirit-specific: `Spread` — consult spirit panel))
- **G2**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); old+1cardplay ((spirit-specific: `old+1cardplay` — consult spirit panel))
- **G4**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy3 ((+3 Energy this turn — growth effect, not track reveal))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · plant · energy2 · energy2 · plant · energy3` — income as slots reveal: 0 → 1 → plant → 2 → 2 → plant → 3

### Card-play track

`card1 · card1 · card2 · card2 · card3 · card4` — CP as slots reveal: 1 → 1 → 2 → 2 → 3 → 4

### Innate Powers

- **CREEPERS TEAR INTO MORTAR** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 1 Moon + 2 Plant: 1 Damage to 1 Town / City.
  - **L2** — 2 Moon + 3 Plant: Repeat this Power.
  - **L3** — 3 Moon + 4 Plant: Repeat this Power again.
- **ALL-ENVELOPING GREEN** (Speed: Fast · Range: 1 · Target: any)
  - **L1** — 1 Water + 3 Plant: Defend 2.
  - **L2** — 2 Water + 4 Plant: Instead, Defend 4.
  - **L3** — 3 Water + 1 Earth + 5 Plant: Also, remove 1 Blight.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×2, **Plant** ×2

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Fields Choked with Growth** | 0 | Slow | 1 | Any Land | sun, water, plant | Push 1 Town. **OR** Push 3 Dahan. |
| **Gift of Proliferation** | 1 | Fast | No Range | Another Spirit | moon, plant | Target Spirit adds 1 Presence up to 1 Range from their Presence. |
| **Overgrow in a Night** | 2 | Fast | 1 | Any Land | moon, plant | Add 1 Presence. **OR** If target land has your Presence and Invaders, 3 Fear. |
| **Stem the Flow of Fresh Water** | 0 | Slow | 1, from your Sacred Site | Any Land | water, plant | 1 Damage to 1 Town/City. If target land is a Mountain or Sands, instead, 1 Damage to each Town/City. |

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


### Strategy Cliffs — per-adversary-level shifts that change A Spread of Rampant Green's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to A Spread of Rampant Green's profile (Fear 2, Offense 4, Control 3, Defense 5, Utility 4).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for A Spread of Rampant Green**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for A Spread of Rampant Green**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for A Spread of Rampant Green**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for A Spread of Rampant Green**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/a-spread-of-rampant-green.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [A Spread of Rampant Green](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
