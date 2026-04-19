# Sun-Bright Whirlwind

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Horizons of Spirit Island                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 3 · Control 5 · Fear 1 · Defense 1 · Utility 3             |
| Primary Elements      | Sun, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Incredibly good at handling Explorer, clearing newly-Explored lands of Invaders so they don't Build there. Not nearly so good at dealing with Town/City. Can focus on Energy and largely forego its Innate Power, focus on Plays to aim for mid-to-high Innate thresholds, or strike a more balanced path. Adds at most 1 Presence per turn, so there won't be time to do it all.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the highest-numbered Sands, 2 in the lowest-numbered Mountain.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence1, second=energy4 |
| G3 | first=gain1p, second=addpresence4 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, sun, energy3, energy4air, energy6
- **Card-play track**: card1, card2, card3, airX, card4, card5sun

## Core Mechanics & Special Rules

### Special Rule

A STIFF WIND AT THEIR BACKS After you Add Presence during Growth, Push up to 1 Explorer/Dahan from that land. (Let other players know this is due to your Special Rule, so they know you're still in the Spirit Phase and not using a Fast power.)

### Innate: VIOLENT WINDSTORMS

- **Speed**: slow · **Range**: 1 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 2 Air | Push up to 1 Explorer. |
| 2 | 2 Sun + 3 Air | 1 Fear. Push up to 2 Explorer/Town. |
| 3 | 2 Sun + 4 Air | For each Invader Pushed by this Power, 1 Damage in the land it was Pushed to. |
| 4 | 3 Sun + 5 Air | 4 Damage (in target land). |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Sun-Bright Whirlwind's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/sun-bright-whirlwind.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/sun-bright-whirlwind.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 6.3), **Sun** (wt 3.6)
- **Mid-game energy estimate (T3–T5 avg)**: 4.33E
- **Power summary**: Offense 3 · Control 5 · Fear 1 · Defense 1 · Utility 3
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements air+sun → 9.9 |
| 2 | **Call to Guard** | 0 | Fast | Sun, Air, Earth | Gather up to 1 Dahan. Then, if Dahan are present, either: Defend 1 per Dahan. **OR** Afte… | elements air+sun → 9.9 |
| 3 | **Call to Isolation** | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. | elements air+sun → 9.9 |
| 4 | **Enticing Splendor** | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer/Town. **OR** Gather up to 2 Dahan. | elements air+sun → 9.9 |
| 5 | **Birds Cry Warning** | 1 | Fast | Sun, Air, Animal | The next time Dahan would be Destroyed in target land, Destroy 2 fewer Dahan. **OR** Push… | elements air+sun → 9.9 |
| 6 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. | elements air+sun → 9.9 |
| 7 | **Purifying Flame** | 1 | Slow | Sun, Fire, Air, Plant | 1 Damage per Blight. If target land is a Mountain or Sands, you may instead Remove 1 Blig… | elements air+sun → 9.9 |
| 8 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear. The next time an Invader is Destroyed in target land this turn, 1 Fear. | elements air+sun → 9.9 |
| 9 | **Reaching Grasp** | 0 | Fast | Sun, Air, Water | Target Spirit gets +2 Range with all their Powers. | elements air+sun → 9.9 |
| 10 | **Spur on with Words of Fire** | 1 | Fast | Sun, Fire, Air | If you target a Spirit other than yourself, they gain +1 Energy. Target Spirit may immedi… | elements air+sun → 9.9 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sweep into the Sea** | 4 | Slow | Sun, Air, Water | Push all Explorers and Towns one land towards the nearest Ocean. **OR** If target land is… | elements air+sun → 9.9 |
| 2 | **Voice of Command** | 3 | Fast | Sun, Air | 1 Damage per Dahan/Explorer, to Towns/Cities only. Defend 2. During Ravage Actions, Explo… | elements air+sun → 9.9 |
| 3 | **Irresistible Call** | 6 | Fast | Sun, Air, Plant | Gather 5 Towns, 5 Dahan, 5 Beasts, and 15 Explorers. | elements air+sun → 9.9 |
| 4 | **Wrap in Wings of Sunlight** | 3 | Fast | Sun, Air, Animal | Move up to 5 Dahan to any land (including back into target land). If you moved at least 1… | elements air+sun → 9.9 |
| 5 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+sun → 9.9 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Sun-Bright Whirlwind in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Gift of Living Energy** | Minor | 0 | Fast | Sun, Fire, Plant | Target Spirit gains 1 Energy. If you have at least 2 Sacred Sites, target Spirit gains 1 … |
| **Elemental Boon** | Minor | 1 | Fast |  | Target Spirit gains 3 different Elements of their choice. If you target another Spirit, y… |
| **Reaching Grasp** | Minor | 0 | Fast | Sun, Air, Water | Target Spirit gets +2 Range with all their Powers. |
| **Powerstorm** | Major | 3 | Fast | Sun, Fire, Air | Target Spirit gains 3 Energy. Once this turn, target Spirit may Repeat a Power Card by pa… |
| **Enticing Splendor** | Minor | 0 | Fast | Sun, Air, Plant | Gather 1 Explorer/Town. **OR** Gather up to 2 Dahan. |
| **Wrap in Wings of Sunlight** | Major | 3 | Fast | Sun, Air, Animal | Move up to 5 Dahan to any land (including back into target land). If you moved at least 1… |
| **Call to Isolation** | Minor | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Blazing Renewal** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
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
Auto-derived from `data/references/wiki/sun-bright-whirlwind.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the highest-numbered Sands, 2 in the lowest-numbered Mountain.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: addpresence1 (Place 1 Presence from a track (Range 1)); energy4 (+4 Energy this turn (growth effect))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence4 (Place 1 Presence from a track (Range 4))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · sun · energy3 · energy4air · energy6` — income as slots reveal: 1 → 2 → sun → 3 → 4 → 6

### Card-play track

`card1 · card2 · card3 · airX · card4 · card5sun` — CP as slots reveal: 1 → 2 → 3 → airX → 4 → 5

### Innate Powers

- **VIOLENT WINDSTORMS** (Speed: Slow · Range: 1 · Target: any)
  - **L1** — 1 Sun + 2 Air: Push up to 1 Explorer.
  - **L2** — 2 Sun + 3 Air: 1 Fear. Push up to 2 Explorer/Town.
  - **L3** — 2 Sun + 4 Air: For each Invader Pushed by this Power, 1 Damage in the land it was Pushed to.
  - **L4** — 3 Sun + 5 Air: 4 Damage (in target land).

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


### Strategy Cliffs — per-adversary-level shifts that change Sun-Bright Whirlwind's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Sun-Bright Whirlwind's profile (Fear 1, Offense 3, Control 5, Defense 1, Utility 3).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Sun-Bright Whirlwind**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Sun-Bright Whirlwind**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Sun-Bright Whirlwind**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Sun-Bright Whirlwind**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/sun-bright-whirlwind.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Sun-Bright Whirlwind](https://spiritislandwiki.com/index.php?title=Sun-Bright_Whirlwind).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
