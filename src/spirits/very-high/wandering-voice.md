# Wandering Voice Keens Delirium

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 2 · Control 5 · Fear 3 · Defense 1 · Utility 2             |
| Primary Elements      | Air, Moon, Sun (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very positional; has a highly mobile Incarna (particularly with lots of Simpleair) that adds Strife and chases Explorer/Town around as it roams the island. Has a harder time setting up Dahan counterattacks, but can use Mind-Shattering Song to more directly harm Invaders with Strife - and earn a fair bit of Fear in the process.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board: 1 in land #6 and 1 in land #7. Put {{incarna|voice}}, Unempowered ({{incarna|unempowered}}) side up, on your starting board in land #6. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=addmovevoiceincarna, third=energy1 |
| G2 | first=addpresence3, second=addpresence1 |
| G3 | first=gain1p, second=addpresence2, third=energy1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, sunormoon, energy2, air, energy4, pushvoiceincarna
- **Card-play track**: card1, card2, card2, card3, reclaim1, card4

## Core Mechanics & Special Rules

### Special Rule

A CLARION VOICE GIVEN FORM</br>You have an Incarna ({{incarna|voice}}). If Empowered, it Isolates its land. SPREAD TUMULT AND DELUSION</br>When your Actions add/move {{incarna|voice}} to a land with Invaders, Add 1 Strife in the destination land. In lands with or adjacent to {{incarna|voice}}: if Strife is present, Dahan do not participate in Ravage. (They do not take Damage or counterattack. Isolate has no effect on {{incarna|voice}} and Dahan being adjacent.) SENSELESS ROAMING</br>When your Actions add Strife to an Explorer/Town, you may Push it.

### Innate: INSCRUTABLE JOURNEYING

- **Speed**: fast · **Range**:  · **Target**: yourself

_(no thresholds listed in Wiki)_


### Innate: MIND-SHATTERING SONG

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: strife

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Moon you have. |
| 2 | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Damage per Sun you have, to Invaders with Strife only. |
| 3 | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | For each Sun Moon pair you have, Destroy 1 Invader with Strife. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Wandering Voice Keens Delirium's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/wandering-voice.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/wandering-voice.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 4.2), **Moon** (wt 1.2), **Sun** (wt 0.9)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 2 · Control 5 · Fear 3 · Defense 1 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 6.3 |
| 2 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+sun → 6.3 |
| 3 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. The next time an Invader is Destroyed in target land this turn, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 6.3 |
| 4 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+moon → 5.4 |
| 5 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town/Dahan. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land has any Beasts, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon → 5.4 |
| 6 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements air+sun → 5.1 |
| 7 | **Call to Isolation** | 0 | Fast | Sun, Air, Animal | Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town per Dahan. **OR** Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | elements air+sun → 5.1 |
| 8 | **Enticing Splendor** | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. **OR** Gather up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | elements air+sun → 5.1 |
| 9 | **Disorienting Landscape** | 1 | Fast | Moon, Air, Plant | Push 1 [[Explorer]]. If target land is a Mountain or Jungle, add 1 [[Wilds]]. | elements air+moon → 5.4 |
| 10 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+moon → 5.4 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Terrifying Nightmares** | 4 | Fast | Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 4 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. | elements air+moon → 5.4 |
| 2 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon+sun → 6.3 |
| 3 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+moon → 5.4 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+moon+sun → 6.3 |
| 5 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+moon+sun → 6.3 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Wandering Voice Keens Delirium in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
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
Auto-derived from `data/references/wiki/wandering-voice.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in land #6 and 1 in land #7. Put {{incarna|voice}}, Unempowered ({{incarna|unempowered}}) side up, on your starting board in land #6. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); addmovevoiceincarna ((spirit-specific: `addmovevoiceincarna` — consult spirit panel)); energy1 ((track slot showing 1 Energy))
- **G2**: addpresence3 (Place 1 Presence from a track (Range 3)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2)); energy1 ((track slot showing 1 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · energy1 · sunormoon · energy2 · air · energy4 · pushvoiceincarna` — income as slots reveal: 0 → 1 → sunormoon → 2 → air → 4 → pushvoiceincarna

### Card-play track

`card1 · card2 · card2 · card3 · reclaim1 · card4` — CP as slots reveal: 1 → 2 → 2 → 3 → reclaim1 → 4

### Innate Powers

- **INSCRUTABLE JOURNEYING** (Speed: Fast · Range: ? · Target: yourself)
- **MIND-SHATTERING SONG** (Speed: Slow · Range: 1 · Target: strife)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Moon you have.
  - **L2** — 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: 1 Damage per Sun you have, to Invaders with Strife only.
  - **L3** — 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: For each Sun Moon pair you have, Destroy 1 Invader with Strife.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

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


### Strategy Cliffs — per-adversary-level shifts that change Wandering Voice Keens Delirium's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Wandering Voice Keens Delirium's profile (Fear 3, Offense 2, Control 5, Defense 1, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Wandering Voice Keens Delirium**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Wandering Voice Keens Delirium**: Front-load coastal defense or disruption before T3's first Ravage.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Wandering Voice Keens Delirium**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Wandering Voice Keens Delirium**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Wandering Voice Keens Delirium**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/wandering-voice-keens-delirium.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Wandering Voice Keens Delirium](https://spiritislandwiki.com/index.php?title=Wandering_Voice_Keens_Delirium).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
