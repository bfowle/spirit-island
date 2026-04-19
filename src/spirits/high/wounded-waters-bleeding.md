# Wounded Waters Bleeding

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
| Power summary (1–5)   | Offense 4 · Control 5 · Fear 2 · Defense 1 · Utility 1             |
| Primary Elements      | Water, Animal, Plant, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Starts off wounded, losing a Presence or a Power Card every turn. Heals over the course of the game, finding a new nature - while some choices may be a touch better or worse due to Adversary, Power Card picks, teammates, etc., most combinations are viable in most games. Benefits from careful Presence placement, both due to losing Presence and because some if its Unique Powers must target from lands with Blight.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> On your starting board, put 2 Presence in a land with Blight, then put 2 Presence and 1 Blight (from the box) in the highest-numbered land with a Town Setup Symbol. You start with your 4 Unique Power Cards and 4 Energy.</br>Set your 4 Healing Cards nearby.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=gain1p, second=addpresence2 |
| G3 | first=addpresence3, second=energy3, third=adddestroyedpresence1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: blank, blank, blank, blank, energy3, energy4fireorplant, energy5any
- **Card-play track**: energy0card1, wateroranimal, gatherblight, energy1card2

## Core Mechanics & Special Rules

### Special Rule

SEEKING A PATH TOWARDS HEALING</br>After playing cards each Spirit Phase: * Claim a Healing Marker (Element Marker) matching whichever of Water or Simpleanimal you have more of. (You break ties.) * You may then Claim a Healing Card if you meet its requirements. (You can claim your first Healing Card on Turn 3.) * Then Destroy 1 Presence or Forget a Power Card (unless a Healing Card just removed this rule).

### Innate: SWIRL AND SPILL

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Water | Push up to 2 Explorer/Dahan/Blight. |
| 2 | 3 Water + 1 Animal | 1 Fear. Push up to 2 Town/Presence/Beasts. |
| 3 | 5 Water + 2 Plant + 2 Animal | In one land pushed into, Downgrade all Town and all City. |


### Innate: SANGUINARY TAINT

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Animal | 1 Fear. 1 Damage. Push 1 Dahan. |
| 2 | 1 Water + 3 Animal | 1 Damage. Add 1 Beasts. |
| 3 | 2 Fire + 2 Water + 5 Animal | 1 Fear. 4 Damage. Add 1 Disease. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Wounded Waters Bleeding's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/wounded-waters-bleeding.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/wounded-waters-bleeding.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Animal** (wt 6.3), **Water** (wt 6.3), **Plant** (wt 0.6)
- **Mid-game energy estimate (T3–T5 avg)**: 5.0E
- **Power summary**: Offense 4 · Control 5 · Fear 2 · Defense 1 · Utility 1
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Weep for What is Lost** | 0 | Slow | Fire, Water, Animal | 1 Fear per type of Invader present. Push up to 1 Explorer/Town per Blight. | elements animal+fire+water → 13.2 |
| 2 | **Call to Tend** | 1 | Slow | Water, Plant, Animal | Remove 1 Blight. **OR** Push up to 3 Dahan. | elements animal+plant+water → 13.2 |
| 3 | **Call of the Dahan Ways** | 1 | Slow | Moon, Water, Animal | Replace 1 Explorer with 1 Dahan. | elements animal+water → 12.6 |
| 4 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements animal+fire+water → 13.2 |
| 5 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear. Add 1 Disease. | elements animal+fire+water → 13.2 |
| 6 | **Infested Aquifers** | 1 | Slow | Moon, Water, Earth, Animal | If target land has any Disease, 1 Damage to each Invader. **OR** If target land is a Moun… | elements animal+water → 12.6 |
| 7 | **Mesmerized Tranquility** | 0 | Fast | Water, Earth, Animal | Isolate target land. Each Invader does -1 Damage. | elements animal+water → 12.6 |
| 8 | **Teeming Rivers** | 1 | Slow | Sun, Water, Plant, Animal | If target land has no Blight, add 1 Beasts. If target land has exactly 1 Blight, Remove i… | elements animal+plant+water → 13.2 |
| 9 | **Rain of Blood** | 0 | Slow | Air, Water, Animal | 2 Fear. If target land has at least 2 Towns/Cities, 1 Fear. | elements animal+water → 12.6 |
| 10 | **Sap the Strength of Multitudes** | 0 | Fast | Water, Animal | Defend 5. | elements animal+water → 12.6 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Ravaged Undergrowth Slithers Back to Life** | 3 | Slow | Water, Plant, Animal | Replace 1 Blight with 1 Wilds.</br>1 Fear. 3 Damage.</br>Push that Wilds. | elements animal+plant+water → 13.2 |
| 2 | **Flocking Red-Talons** | 3 | Fast | Air, Water, Plant, Animal | Add 1 Beasts. Move up to 2 Beasts within 3 Range to target land. For each Beasts present,… | elements animal+plant+water → 13.2 |
| 3 | **Dissolve the Bonds of Kinship** | 4 | Slow | Fire, Water, Animal | Replace 1 City with 2 Explorers. Replace 1 Town with 1 Explorer. Replace 1 Dahan with 1 E… | elements animal+fire+water → 13.2 |
| 4 | **Inspire the Release of Stolen Lands** | 4 | Slow | Sun, Water, Plant, Animal | Gather up to 3 Dahan. Remove up to 3 Health worth of Invaders per Dahan. | elements animal+plant+water → 13.2 |
| 5 | **Plague Ships Sail to Distant Ports** | 4 | Fast | Fire, Air, Water, Animal | 1 Fear. Add 4 Disease among Coastal lands (on any boards) other than target land. | elements animal+fire+water → 13.2 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Wounded Waters Bleeding in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/wounded-waters-bleeding.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: On your starting board, put 2 Presence in a land with Blight, then put 2 Presence and 1 Blight (from the box) in the highest-numbered land with a Town Setup Symbol. You start with your 4 Unique Power Cards and 4 Energy.</br>Set your 4 Healing Cards nearby.
- **Starting income** (from `presence_energy_track[0]` = `blank`, `presence_cardplay_track[0]` = `energy0card1`): **3 Energy · 0 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: addpresence3 (Place 1 Presence from a track (Range 3)); energy3 ((+3 Energy this turn — growth effect, not track reveal)); adddestroyedpresence1 ((spirit-specific: `adddestroyedpresence1` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`blank · blank · blank · blank · energy3 · energy4fireorplant · energy5any` — income as slots reveal: blank → blank → blank → blank → 3 → 4 → 5

### Card-play track

`energy0card1 · wateroranimal · gatherblight · energy1card2` — CP as slots reveal: energy0card1 → wateroranimal → gatherblight → energy1card2

### Innate Powers

- **SWIRL AND SPILL** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Water: Push up to 2 Explorer/Dahan/Blight.
  - **L2** — 3 Water + 1 Animal: 1 Fear. Push up to 2 Town/Presence/Beasts.
  - **L3** — 5 Water + 2 Plant + 2 Animal: In one land pushed into, Downgrade all Town and all City.
- **SANGUINARY TAINT** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 2 Animal: 1 Fear. 1 Damage. Push 1 Dahan.
  - **L2** — 1 Water + 3 Animal: 1 Damage. Add 1 Beasts.
  - **L3** — 2 Fire + 2 Water + 5 Animal: 1 Fear. 4 Damage. Add 1 Disease.

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


### Strategy Cliffs — per-adversary-level shifts that change Wounded Waters Bleeding's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Wounded Waters Bleeding's profile (Fear 2, Offense 4, Control 5, Defense 1, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Wounded Waters Bleeding**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Wounded Waters Bleeding**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Wounded Waters Bleeding**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Wounded Waters Bleeding**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/wounded-waters-bleeding.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Wounded Waters Bleeding](https://spiritislandwiki.com/index.php?title=Wounded_Waters_Bleeding).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
