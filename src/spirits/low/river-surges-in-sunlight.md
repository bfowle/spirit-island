# River Surges in Sunlight

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | Low                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 5 · Fear 1 · Defense 1 · Utility 4             |
| Primary Elements      | Sun, Water, Earth (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> While capable of some direct offense, River Surges in Sunlight is best at flooding out Explorers and Towns, displacing them from lands where they might Build or Ravage. The ability to get free Sacred Sites makes a wide range of Powers more useful.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence on your starting board in the highest-numbered Wetlands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence1, second=addpresence1 |
| G3 | first=gain1p, second=addpresence2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, energy2, energy3, energy4, energy4, energy5
- **Card-play track**: card1, card2, card2, card3, reclaim1, card4, card5

## Core Mechanics & Special Rules

### Special Rule

RIVER'S DOMAIN Your Presence in Wetlands counts as Sacred Site.

### Innate: MASSIVE FLOODING

- **Speed**: slow · **Range**: 1 (optionally from a sacred site) · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Sun + 2 Water | Push 1 Explorer / Town. |
| 2 | 2 Sun + 3 Water | Instead, 2 Damage. Push up to 3 Explorer / Town. |
| 3 | 3 Sun + 4 Water + 1 Earth | Instead, 2 Damage to each Invader. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by River Surges in Sunlight's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/river-surges-in-sunlight.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/river-surges-in-sunlight.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Water** (wt 4.8), **Sun** (wt 3.0), **Earth** (wt 0.3)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 4 · Control 5 · Fear 1 · Defense 1 · Utility 4
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements sun+water → 7.8 |
| 2 | **Strong and Constant Currents** | 0 | Fast | Sun, Water, Earth | Push 1 Explorer/Town to an adjacent Coastal land. **OR** Move up to 2 Dahan between targe… | elements earth+sun+water → 8.1 |
| 3 | **Like Calls to Like** | 1 | Slow | Sun, Water, Plant | If target land has Explorer, Gather up to 1 Explorer. Do likewise for Town, Dahan, Blight… | elements sun+water → 7.8 |
| 4 | **Song of Sanctity** | 1 | Slow | Sun, Water, Plant | If Explorer(s) are present, Push all Explorers. Otherwise, Remove 1 Blight. | elements sun+water → 7.8 |
| 5 | **Sunset's Fire Flows Across the Land** | 1 | Slow | Sun, Moon, Fire, Water | 1 Fear. 1 Damage. You may pay 1 Energy to deal 1 Damage in an adjacent land. | elements sun+water → 7.8 |
| 6 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements sun+water → 7.8 |
| 7 | **Elusive Ambushes** | 1 | Fast | Sun, Fire, Water | 1 Damage. **OR** Defend 4. | elements sun+water → 7.8 |
| 8 | **Reaching Grasp** | 0 | Fast | Sun, Air, Water | Target Spirit gets +2 Range with all their Powers. | elements sun+water → 7.8 |
| 9 | **Teeming Rivers** | 1 | Slow | Sun, Water, Plant, Animal | If target land has no Blight, add 1 Beasts. If target land has exactly 1 Blight, Remove i… | elements sun+water → 7.8 |
| 10 | **Uncanny Melting** | 1 | Slow | Sun, Moon, Water | If Invaders are present, 1 Fear. If target land is a Sands or Wetland, Remove 1 Blight. | elements sun+water → 7.8 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sweep into the Sea** | 4 | Slow | Sun, Air, Water | Push all Explorers and Towns one land towards the nearest Ocean. **OR** If target land is… | elements sun+water → 7.8 |
| 2 | **Inspire the Release of Stolen Lands** | 4 | Slow | Sun, Water, Plant, Animal | Gather up to 3 Dahan. Remove up to 3 Health worth of Invaders per Dahan. | elements sun+water → 7.8 |
| 3 | **Accelerated Rot** | 4 | Slow | Sun, Water, Plant | 2 Fear. 4 Damage. | elements sun+water → 7.8 |
| 4 | **Cleansing Floods** | 5 | Slow | Sun, Water | 4 Damage. Remove 1 Blight. | elements sun+water → 7.8 |
| 5 | **Unrelenting Growth** | 4 | Slow | Sun, Fire, Water, Plant | Target Spirit adds 2 Presence and 1 Wilds to a land at Range 1 of their Presence. | elements sun+water → 7.8 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with River Surges in Sunlight in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Uncanny Melting** | Minor | 1 | Slow | Sun, Moon, Water | If Invaders are present, 1 Fear. If target land is a Sands or Wetland, Remove 1 Blight. |
| Nature\'s Resilience | — | — | — | — | (fetch error: Wiki API error for 'Nature\'s_Resilience': {'code': 'missingtitle', 'info': "The page you specified doesn't exist.", 'docref': 'See https://spiritislandwiki.com/api.php for API usage. Subscribe to the mediawiki-api-announce mailing list at &lt;https://lists.wikimedia.org/mailman/listinfo/mediawiki-api-announce&gt; for notice of API deprecations and breaking changes.'}) |
| **Pull Beneath the Hungry Earth** | Minor | 1 | Slow | Moon, Water, Earth | If your Presence is present, 1 Fear and 1 Damage. If target land is a Sands or Wetland, 1… |
| **Accelerated Rot** | Major | 4 | Slow | Sun, Water, Plant | 2 Fear. 4 Damage. |
| **Song of Sanctity** | Minor | 1 | Slow | Sun, Water, Plant | If Explorer(s) are present, Push all Explorers. Otherwise, Remove 1 Blight. |
| **Tsunami** | Major | 6 | Slow | Water, Earth | 2 Fear. 8 Damage. Destroy 2 Dahan. |
| **Encompassing Ward** | Minor | 1 | Fast | Sun, Water, Earth | Target Spirit provides Defend 2 in each of its lands. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Land of Haunts and Embers** | adds Blight |
| **Scour the Land** | adds Blight |
| **Tsunami** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opening Strategy

`[VERIFY: needs play data]` — opening variants should be rehearsed turn-by-turn per the [SPIRIT_TEMPLATE.md](../../../templates/SPIRIT_TEMPLATE.md) opener format.

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


### Strategy Cliffs — per-adversary-level shifts that change River Surges in Sunlight's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to River Surges in Sunlight's profile (Fear 1, Offense 4, Control 5, Defense 1, Utility 4).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for River Surges in Sunlight**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for River Surges in Sunlight**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for River Surges in Sunlight**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for River Surges in Sunlight**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/river-surges-in-sunlight.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [River Surges in Sunlight](https://spiritislandwiki.com/index.php?title=River_Surges_in_Sunlight).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
