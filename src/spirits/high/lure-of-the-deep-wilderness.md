# Lure of the Deep Wilderness

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "oneandone" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 4 · Fear 4 · Defense 2 · Utility 1             |
| Primary Elements      | Moon, Air, Plant, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Very focused on the interior - its best options for coastal lands are "draw the Invaders inland" or "turn Town/City into Explorer, then draw them inland". Likes the interior to be dangerous, full of Badlands, Beast, Disease, and Wilds, ideally where its Presence is. Has better-than-average potential for containing Invaders and setting up a zone safe from Explores, but the coasts may get messy while doing so.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 2 in land #8, and 1 in land #7. Add 1 Beast to land #8.

## Growth Options (oneandone)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=energy1 |
| G2 | first=addpresence4inland |
| G3 | first=moonairplant, second=energy2 |
| G4 | first=gain1p |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, moon, energy3plant, energy4air, energy5reclaim
- **Card-play track**: card1, card2, animalX, card3, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

HOME OF THE ISLAND'S HEART Your Presence may only be added/moved to lands that are Inland. ENTHRALL THE FOREIGN EXPLORERS For each of your Presence in a land, up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> do not participate in Ravage.

### Innate: FORSAKE SOCIETY TO CHASE AFTER DREAMS

- **Speed**: slow · **Range**: 1 · **Target**: invaders

_(no thresholds listed in Wiki)_


### Innate: NEVER HEARD FROM AGAIN

- **Speed**: slow · **Range**: 0 · **Target**: inland

_(no thresholds listed in Wiki)_


## Unique Cards (all, Wiki-verified)

#### Gift of the Untamed Wild

- **0 Energy · Slow · Range No Range · Any Spirit · Moon, Fire, Air, Plant**
- *Target Spirit chooses to either: Add 1 Wilds to one of their lands. **OR** Replace 1 of their Presence with 1 Disease.*

#### Perils of the Deepest Island

- **1 Energy · Slow · Range 0 · Inland Land · Moon, Plant, Animal**
- *1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Badlands. Add 1 Beasts within 1 Range. Push up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.*

#### Softly Beckon Ever Inward

- **2 Energy · Slow · Range 0 · Inland Land · Moon, Air**
- *Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. Gather up to 2 Towns <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. Gather up to 2 Beasts. Gather up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.*

#### Swallowed by the Wilderness

- **1 Energy · Fast · Range 0 · Inland Land · Fire, Air, Plant, Animal**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage per Beasts/Disease/Wilds/Badlands. (Count max. 5 tokens.)*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Lure of the Deep Wilderness's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/lure-of-the-deep-wilderness.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/lure-of-the-deep-wilderness.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: —
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 4 · Control 4 · Fear 4 · Defense 2 · Utility 1
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Gift of the Untamed Wild**
- **Perils of the Deepest Island**
- **Softly Beckon Ever Inward**
- **Swallowed by the Wilderness**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | Slow speed matches innate |
| 2 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** Add 1 Wilds. | 0-cost (always affordable) |
| 3 | **Call to Ferocity** | 0 | Slow | Sun, Fire, Earth | Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. **OR** If target land has Dahan, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 T… | 0-cost (always affordable) |
| 4 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town/Dahan. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land has any Beasts, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | 0-cost (always affordable) |
| 5 | **Savage Mawbeasts** | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | 0-cost (always affordable) |
| 6 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | 0-cost (always affordable) |
| 7 | **Shadows of the Burning Forest** | 0 | Slow | Moon, Fire, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. | 0-cost (always affordable) |
| 8 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | 0-cost (always affordable) |
| 9 | **Weep for What is Lost** | 0 | Slow | Fire, Water, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per type of Invader present. Push up to 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town per Blight. | 0-cost (always affordable) |
| 10 | **Call to Bloodshed** | 1 | Slow | Sun, Fire, Animal | 1 Damage per Dahan. **OR** Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | Slow speed matches innate |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Angry Bears** | 3 | Slow | Sun, Fire, Animal | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 2 Damage. If no Beasts are present, add 1 Beasts. Otherwise, +2 Damage, and Push … | Slow speed matches innate |
| 2 | **Focus the Land's Anguish** | 5 | Slow | Sun | If this Power Destroys any Towns/Cities, 5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Gather up to 5 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. 1 Damage per Blig… | Slow speed matches innate |
| 3 | **Ravaged Undergrowth Slithers Back to Life** | 3 | Slow | Water, Plant, Animal | Replace 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> with 1 Wilds.</br>1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 3 Damage.</br>Push that Wilds. | Slow speed matches innate |
| 4 | **The Wounded Wild Turns on its Assailants** | 4 | Slow | Fire, Plant, Animal | Add 2 Badlands. Gather up to 2 Beasts. 1 Damage per Blight/Beasts/Wilds. | Slow speed matches innate |
| 5 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | Slow speed matches innate |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Lure of the Deep Wilderness in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/lure-of-the-deep-wilderness.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 2 in land #8, and 1 in land #7. Add 1 Beast to land #8.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `oneandone` — (see spirit panel)

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); energy1 ((track slot showing 1 Energy))
- **G2**: addpresence4inland ((spirit-specific: `addpresence4inland` — consult spirit panel))
- **G3**: moonairplant ((spirit-specific: `moonairplant` — consult spirit panel)); energy2 ((track slot showing 2 Energy))
- **G4**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · moon · energy3plant · energy4air · energy5reclaim` — income as slots reveal: 1 → 2 → moon → 3 → 4 → 5

### Card-play track

`card1 · card2 · animalX · card3 · card4 · card5reclaim1` — CP as slots reveal: 1 → 2 → animalX → 3 → 4 → 5

### Innate Powers

- **FORSAKE SOCIETY TO CHASE AFTER DREAMS** (Speed: Slow · Range: 1 · Target: invaders)
- **NEVER HEARD FROM AGAIN** (Speed: Slow · Range: 0 · Target: inland)

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Fire** ×1, **Air** ×1, **Plant** ×1, **Animal** ×1

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Gift of the Untamed Wild** | 0 | Slow | No Range | Any Spirit | moon, fire, air, plant | Target Spirit chooses to either: Add 1 Wilds to one of their lands. **OR** Replace 1 of their Prese… |
| **Perils of the Deepest Island** | 1 | Slow | 0 | Inland Land | moon, plant, animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Badlands. Add 1 Beasts within 1 Range. Push up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| **Softly Beckon Ever Inward** | 2 | Slow | 0 | Inland Land | moon, air | Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. Gather up to 2 Towns <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. Gather up to 2 Beasts. Gather up to 2 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| **Swallowed by the Wilderness** | 1 | Fast | 0 | Inland Land | fire, air, plant, animal | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage per Beasts/Disease/Wilds/Badlands. (Count max. 5 tokens.) |

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


### Strategy Cliffs — per-adversary-level shifts that change Lure of the Deep Wilderness's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Lure of the Deep Wilderness's profile (Fear 4, Offense 4, Control 4, Defense 2, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Lure of the Deep Wilderness**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Lure of the Deep Wilderness**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Lure of the Deep Wilderness**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Lure of the Deep Wilderness**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Lure of the Deep Wilderness**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Lure of the Deep Wilderness**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Lure of the Deep Wilderness**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/lure-of-the-deep-wilderness.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Lure of the Deep Wilderness](https://spiritislandwiki.com/index.php?title=Lure_of_the_Deep_Wilderness).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
