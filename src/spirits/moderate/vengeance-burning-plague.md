# Vengeance as a Burning Plague

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
| Power summary (1–5)   | Offense 5 · Control 2 · Fear 3 · Defense 1 · Utility 1             |
| Primary Elements      | Fire, Animal, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Not so powerful early, but can be a late-game juggernaut, especially if things are going badly: Blight adds to its Damage and its Presence being Destroyed adds Disease. It may even want to engineer these situations, which can make other Spirits nervous (and be risky if things go poorly).

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> 1 of your Presence starts the game already Destroyed. Put 2 Presence on your starting board: 1 in a land with Blight, 1 in a Wetland without Dahan.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=Vengeance, second=Vengeance |
| G3 | first=gain1p, second=addpresencedisease1, third=energy1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, animal, energy3, energy4
- **Card-play track**: card1, card2, fireX, card2, card3, card3, card4

## Core Mechanics & Special Rules

### Special Rule

THE TERROR OF A SLOWLY UNFOLDING PLAGUE When Disease would prevent a Build on a board with your Presence, you may let the Build happen (removing no Disease). If you do, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. LINGERING PESTILENCE When your Presence is destroyed by anything except a Spirit action, add 1 Disease where each destroyed Presence was. WREAK VENGEANCE FOR THE LAND'S CORRUPTION Your actions treat Blight on the island as also being Badlands.

### Innate: EPIDEMICS RUN RAMPANT

- **Speed**: fast · **Range**: 1 · **Target**: disease

_(no thresholds listed in Wiki)_


### Innate: SAVAGE REVENGE

- **Speed**: slow · **Range**: 0 · **Target**: building

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power has Range +1. |
| 2 | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage. |
| 3 | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +2 Damage. |
| 4 | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +3 Damage. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Vengeance as a Burning Plague's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/vengeance-burning-plague.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/vengeance-burning-plague.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Fire** (wt 4.5), **Air** (wt 3.3), **Animal** (wt 1.8)
- **Mid-game energy estimate (T3–T5 avg)**: 3.5E
- **Power summary**: Offense 5 · Control 2 · Fear 3 · Defense 1 · Utility 1
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Swarming Wasps** | 0 | Fast | Fire, Air, Animal | Add 1 Beasts. **OR** If target land has Beasts, Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+animal+fire → 9.6 |
| 2 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Disease. | elements air+animal+fire → 9.6 |
| 3 | **Call to Migrate** | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Push up to 3 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. | elements air+animal+fire → 9.6 |
| 4 | **Steam Vents** | 1 | Fast | Fire, Air, Water, Earth | Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+fire → 7.8 |
| 5 | **Dry Wood Explodes in Smoldering Splinters** | 1 | Slow | Fire, Air, Plant | You may spend 1 Energy to make this Power Fast. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. | elements air+fire → 7.8 |
| 6 | **Hazards Spread Across the Island** | 0 | Fast | Fire, Air, Earth, Plant | Choose a type of token from Badlands/Beasts/Disease/Strife/Wilds that exists in an adjace… | elements air+fire → 7.8 |
| 7 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+fire → 7.8 |
| 8 | **Desiccating Winds** | 1 | Slow | Fire, Air, Earth | If target land has Badlands, 1 Damage. Add 1 Badlands. | elements air+fire → 7.8 |
| 9 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+fire → 7.8 |
| 10 | **Fire in the Sky** | 1 | Fast | Sun, Fire, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. | elements air+fire → 7.8 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Plague Ships Sail to Distant Ports** | 4 | Fast | Fire, Air, Water, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 4 Disease among Coastal lands (on any boards) other than target land. | elements air+animal+fire → 9.6 |
| 2 | **Instruments of Their Own Ruin** | 4 | Fast | Sun, Fire, Air, Animal | Add 1 Strife. Each Invader with Strife deals Damage to other Invaders in target land. | elements air+animal+fire → 9.6 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+fire → 9.6 |
| 4 | **Talons of Lightning** | 6 | Fast | Fire, Air | 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 5 Damage. | elements air+fire → 7.8 |
| 5 | **Storm-Swath** | 3 | Slow | Fire, Air, Water | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. In both origin land and target land: 1 Damage to each Invader. | elements air+fire → 7.8 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Vengeance as a Burning Plague in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Pyroclastic Flow** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/vengeance-burning-plague.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: 1 of your Presence starts the game already Destroyed. Put 2 Presence on your starting board: 1 in a land with Blight, 1 in a Wetland without Dahan.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: Vengeance ((spirit-specific: `Vengeance` — consult spirit panel)); Vengeance ((spirit-specific: `Vengeance` — consult spirit panel))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresencedisease1 ((spirit-specific: `addpresencedisease1` — consult spirit panel)); energy1 ((track slot showing 1 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · animal · energy3 · energy4` — income as slots reveal: 1 → 2 → animal → 3 → 4

### Card-play track

`card1 · card2 · fireX · card2 · card3 · card3 · card4` — CP as slots reveal: 1 → 2 → fireX → 2 → 3 → 3 → 4

### Innate Powers

- **EPIDEMICS RUN RAMPANT** (Speed: Fast · Range: 1 · Target: disease)
- **SAVAGE REVENGE** (Speed: Slow · Range: 0 · Target: building)
  - **L1** — 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: This Power has Range +1.
  - **L2** — 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal: 1 Damage.
  - **L3** — 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal: +2 Damage.
  - **L4** — 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal: +3 Damage.

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


### Strategy Cliffs — per-adversary-level shifts that change Vengeance as a Burning Plague's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Vengeance as a Burning Plague's profile (Fear 3, Offense 5, Control 2, Defense 1, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Vengeance as a Burning Plague**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Vengeance as a Burning Plague**: Front-load coastal defense or disruption before T3's first Ravage.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Vengeance as a Burning Plague**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Vengeance as a Burning Plague**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Vengeance as a Burning Plague**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Vengeance as a Burning Plague**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/vengeance-as-a-burning-plague.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Vengeance as a Burning Plague](https://spiritislandwiki.com/index.php?title=Vengeance_as_a_Burning_Plague).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
