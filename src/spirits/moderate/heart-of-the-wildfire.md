# Heart of the Wildfire

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 1                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 5 · Control 3 · Fear 4 · Defense 1 · Utility 2             |
| Primary Elements      | Fire, Plant, Air, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Starts with good offense and gets better from there, but lays down Blight as it grows. The smaller the game, the more restraint is needed to prevent tipping the island over into being completely Blighted. The Wildfire can heal the land where it is, but may benefit from other Blight removal Powers so it can add Presence to problem lands without triggering Blight cascade. Removing Blight from its own lands limits its "Firestorm" innate power, however. In the Reprint, the complexity was changed from Moderate to High

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence and 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> on your starting board in the highest-numbered Sands. (Blight comes from the box, not the Blight Card)

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=gain1p, second=addpresence3 |
| G3 | first=addpresence1, second=energy2, third=Wildfire |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, fire, energy1, energy2, fireplant, energy3
- **Card-play track**: card1, fireX, card2, card3, fireX, card4

## Core Mechanics & Special Rules

### Special Rule

BLAZING PRESENCE Post-Setup, after your Presence is added/moved, in the land it goes to: * For each Simplefire showing on your Presence Tracks, do 1 Damage. * If 2 Simplefire or more are showing on your Presence Tracks, add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. * Push all Beast and any number of Dahan. If you add multiple Presence into a land at the same time, only do the above effects once. DESTRUCTIVE NATURE Blight added due to Spirit effects (Powers, Special Rules, Scenario-based Rituals, etc) does not destroy your Presence. (This includes cascades.)

### Innate: FIRESTORM

- **Speed**: fast · **Range**: 0 · **Target**: blight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Damage per 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire you have. |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Instead, 1 Damage per Fire you have. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Split this Power's Damage however desired between target land and any number of your lands with Blight. |
| 4 | 7 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | In a land with Blight where you have Presence, Push all Dahan. Destroy all Invaders and Beast. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. |


### Innate: THE BURNED LAND REGROWS

- **Speed**: slow · **Range**: 0 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | If target land has 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> or more, remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> |
| 2 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Instead, remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">. |
| 3 | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Remove another Blight. |


## Unique Cards (all, Wiki-verified)

#### Asphyxiating Smoke

- **2 Energy · Slow · Range 2, from your Sacred Site · Any Land · Fire, Air, Plant**
- *1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">.*

#### Flame's Fury

- **0 Energy · Fast · Range No Range · Any Spirit · Sun, Fire, Plant**
- *Target Spirit gains 1 Energy. Target Spirit does +1 Damage with each Damage dealing Power they use this turn. (Powers which Damage multiple lands or each Invader only get 1 extra Damage total. Repeated Powers keep the +1 boost. Destroy effects don't get any bonus.)*

#### Flash-Fires

- **2 Energy · Slow · Range 1 · Any Land · Fire, Air**
- *1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage.*

- **Threshold**: 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air — 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: This Power is Fast.

#### Threatening Flames

- **0 Energy · Fast · Range 0 · Land with 1 or more Blight and 1 or more Invaders · Fire, Plant**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town per Terror Level from target land to adjacent lands without your Presence. If there are no such adjacent lands, +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Heart of the Wildfire's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/heart-of-the-wildfire.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/heart-of-the-wildfire.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Fire** (wt 10.8), **Plant** (wt 5.4), **Air** (wt 0.6)
- **Mid-game energy estimate (T3–T5 avg)**: 2.5E
- **Power summary**: Offense 5 · Control 3 · Fear 4 · Defense 1 · Utility 2
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Asphyxiating Smoke**
- **Flame's Fury**
- **Flash-Fires**
- **Threatening Flames**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+earth+fire+plant → 17.4 |
| 2 | **Dry Wood Explodes in Smoldering Splinters** | 1 | Slow | Fire, Air, Plant | You may spend 1 Energy to make this Power Fast. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. | elements air+fire+plant → 16.8 |
| 3 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** Add 1 Wilds. | elements fire+plant → 16.2 |
| 4 | **Sear Anger Into the Wild Lands** | 0 | Slow | Sun, Fire, Plant | Add 1 Badlands. **OR** If Wilds and Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and 1 Damage. | elements fire+plant → 16.2 |
| 5 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+fire+plant → 16.8 |
| 6 | **Rouse the Trees and Stones** | 1 | Slow | Fire, Earth, Plant | 2 Damage. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements earth+fire+plant → 16.8 |
| 7 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Isolate. Defend 2.</br>1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> does not participate in Ravage.</br>(Check when ra… | elements fire+plant → 16.2 |
| 8 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire+plant → 16.8 |
| 9 | **Shadows of the Burning Forest** | 0 | Slow | Moon, Fire, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. | elements fire+plant → 16.2 |
| 10 | **The Shore Seethes with Hatred** | 1 | Slow | Fire, Water, Earth, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Badlands and 1 Wilds. | elements earth+fire+plant → 16.8 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements earth+fire+plant → 16.8 |
| 2 | **Settle Into Hunting-Grounds** | 3 | Fast | Moon, Fire, Plant, Animal | Your Presence may count as Badlands and Beasts. (Decide per Presence, per Action.) Your P… | elements fire+plant → 16.2 |
| 3 | **Forests of Living Obsidian** | 4 | Slow | Sun, Fire, Earth, Plant | Add 1 Badlands. Push all Dahan. 1 Damage to each Invader. If the origin land is your Sacr… | elements earth+fire+plant → 16.8 |
| 4 | **The Wounded Wild Turns on its Assailants** | 4 | Slow | Fire, Plant, Animal | Add 2 Badlands. Gather up to 2 Beasts. 1 Damage per Blight/Beasts/Wilds. | elements fire+plant → 16.2 |
| 5 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+earth+fire+plant → 17.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Heart of the Wildfire in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Blazing Renewal** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Draw Towards a Consuming Void** | destroys Presence |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/heart-of-the-wildfire.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence and 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> on your starting board in the highest-numbered Sands. (Blight comes from the box, not the Blight Card)
- **Starting income** (from `presence_energy_track[0]` = `energy0`, `presence_cardplay_track[0]` = `card1`): **0 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); energy2 ((track slot showing 2 Energy)); Wildfire ((spirit-specific: `Wildfire` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy0 · fire · energy1 · energy2 · fireplant · energy3` — income as slots reveal: 0 → fire → 1 → 2 → fireplant → 3

### Card-play track

`card1 · fireX · card2 · card3 · fireX · card4` — CP as slots reveal: 1 → fireX → 2 → 3 → fireX → 4

### Innate Powers

- **FIRESTORM** (Speed: Fast · Range: 0 · Target: blight)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: 1 Damage per 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire you have.
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Instead, 1 Damage per Fire you have.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Split this Power's Damage however desired between target land and any number of your lands with Blight.
  - **L4** — 7 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire: In a land with Blight where you have Presence, Push all Dahan. Destroy all Invaders and Beast. Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">.
- **THE BURNED LAND REGROWS** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: If target land has 2 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> or more, remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">
  - **L2** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Instead, remove 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">.
  - **L3** — 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant: Remove another Blight.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Sun** ×1, **Fire** ×2, **Plant** ×2
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Asphyxiating Smoke** | 2 | Slow | 2, from your Sacred Site | Any Land | fire, air, plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| **Flame's Fury** | 0 | Fast | No Range | Any Spirit | sun, fire, plant | Target Spirit gains 1 Energy. Target Spirit does +1 Damage with each Damage dealing Power they use … |
| **Flash-Fires** | 2 | Slow | 1 | Any Land | fire, air | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. |
| **Threatening Flames** | 0 | Fast | 0 | Land with 1 or more Blight and 1 or more Invaders | fire, plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town per Terror Level from target land to adjacent lands without your Prese… |

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


### Strategy Cliffs — per-adversary-level shifts that change Heart of the Wildfire's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Heart of the Wildfire's profile (Fear 4, Offense 5, Control 3, Defense 1, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Heart of the Wildfire**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Heart of the Wildfire**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Heart of the Wildfire**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Heart of the Wildfire**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Heart of the Wildfire**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Heart of the Wildfire**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Heart of the Wildfire**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/heart-of-the-wildfire.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Heart of the Wildfire](https://spiritislandwiki.com/index.php?title=Heart_of_the_Wildfire).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
