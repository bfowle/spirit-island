# Eyes Watch from the Trees

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
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 4 · Defense 5 · Utility 1             |
| Primary Elements      | Moon, Plant, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Good at Defending against Ravages, and at steadily earning Fear. Its ability to Gather Dahan to fight back when Defending can make a huge difference, changing a stalling tactic into a blow against the Invaders.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board, in the highest-numbered Jungle. You start with your 4 Unique Powers and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=energy1 |
| G2 | first=addpresence2, second=addpresence0 |
| G3 | first=gain1p, second=addpresence3, third=energy1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy1, energy2, plant, energy3, moon, energy4
- **Card-play track**: card1, card2, airX, card3, plantX, card4

## Core Mechanics & Special Rules

### Special Rule

Dahan Trust the Watchers After one of your Powers adds Defend to a single land, Gather up to 1 Dahan into that land. ("Power" includes both your Innate Power and your Power Cards. Can be used with any number of Defend Powers each turn.)

### Innate: Mischief and Sabotage

- **Speed**: fast · **Range**: 1 (optionally from a sacred site) · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 2 Plant | 1 Fear and Defend 2. |
| 2 | 2 Moon + 3 Plant | Instead, 1 Fear and Defend 4. |
| 3 | 2 Moon + 2 Air + 4 Plant | Instead, 3 Fear and Defend 6. |
| 4 | 3 Moon + 3 Air + 5 Plant | Instead, 5 Fear and Defend 12. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Eyes Watch from the Trees's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/eyes-watch-from-the-trees.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/eyes-watch-from-the-trees.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Plant** (wt 6.3), **Moon** (wt 3.6), **Air** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 2 · Control 3 · Fear 4 · Defense 5 · Utility 1
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+moon+plant → 11.4 |
| 2 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear. Isolate. Defend 2.</br>1 Dahan does not participate in Ravage.</br>(Check when ra… | elements moon+plant → 9.9 |
| 3 | **Disorienting Landscape** | 1 | Fast | Moon, Air, Plant | Push 1 [[Explorer]]. If target land is a Mountain or Jungle, add 1 [[Wilds]]. | elements air+moon+plant → 11.4 |
| 4 | **Favor of the Sun and Star-lit Dark** | 1 | Fast | Sun, Moon, Plant | Defend 4. Push up to 1 Blight. | elements moon+plant → 9.9 |
| 5 | **Dark and Tangled Woods** | 1 | Fast | Moon, Earth, Plant | 2 Fear. If target land is a Mountain or Jungle, Defend 3. | elements moon+plant → 9.9 |
| 6 | **Razor-Sharp Undergrowth** | 1 | Fast | Moon, Plant | Destroy 1 Explorer and 1 Dahan. Add 1 Wilds. Defend 2. | elements moon+plant → 9.9 |
| 7 | **Shadows of the Burning Forest** | 0 | Slow | Moon, Fire, Plant | 2 Fear. If target land is a Mountain or Jungle, Push 1 Explorer and 1 Town. | elements moon+plant → 9.9 |
| 8 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear. Destroy 1 Explorer. **OR** Add 1 Wilds. | elements moon+plant → 9.9 |
| 9 | **Inflame the Fires of Life** | 1 | Slow | Moon, Fire, Plant, Animal | Add 1 Disease. **OR** 1 Fear. Add 1 Strife. | elements moon+plant → 9.9 |
| 10 | **Gift of Power** | 0 | Slow | Moon, Water, Earth, Plant | Target Spirit gains a Minor Power Card. | elements moon+plant → 9.9 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Death Falls Gently from Open Blossoms** | 4 | Slow | Moon, Air, Plant | 4 Damage. If any Invaders remain, add 1 Disease. | elements air+moon+plant → 11.4 |
| 2 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+moon+plant → 11.4 |
| 3 | **Trees Radiate Celestial Brilliance** | 3 | Fast | Sun, Moon, Plant | 3 Fear. Defend 6. This turn, Invaders in target land skip the next Build Action. | elements moon+plant → 9.9 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+moon+plant → 11.4 |
| 5 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+moon+plant → 11.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Eyes Watch from the Trees in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Lure of the Unknown** | Minor | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. |
| Drift Down Into Slumber | — | — | — | — | (fetch error: No {{PowerCardArticle|...}} template found on page) |
| **Delusions of Danger** | Minor | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. |
| **Terrifying Nightmares** | Major | 4 | Fast | Moon, Air | 2 Fear. Push up to 4 Explorers/Towns. |
| **Dark and Tangled Woods** | Minor | 1 | Fast | Moon, Earth, Plant | 2 Fear. If target land is a Mountain or Jungle, Defend 3. |
| **The Jungle Hungers** | Major | 3 | Slow | Moon, Plant | Destroy all Explorers and all Towns. Destroy all Dahan. |
| Veil the Night\'s Hunt | — | — | — | — | (fetch error: Wiki API error for 'Veil_the_Night\'s_Hunt': {'code': 'missingtitle', 'info': "The page you specified doesn't exist.", 'docref': 'See https://spiritislandwiki.com/api.php for API usage. Subscribe to the mediawiki-api-announce mailing list at &lt;https://lists.wikimedia.org/mailman/listinfo/mediawiki-api-announce&gt; for notice of API deprecations and breaking changes.'}) |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Skies Herald the Season of Return** | destroys Presence |
| **Land of Haunts and Embers** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Blazing Renewal** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/eyes-watch-from-the-trees.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board, in the highest-numbered Jungle. You start with your 4 Unique Powers and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy1 ((track slot showing 1 Energy))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); energy1 ((track slot showing 1 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy1 · energy2 · plant · energy3 · moon · energy4` — income as slots reveal: 1 → 1 → 2 → plant → 3 → moon → 4

### Card-play track

`card1 · card2 · airX · card3 · plantX · card4` — CP as slots reveal: 1 → 2 → airX → 3 → plantX → 4

### Innate Powers

- **Mischief and Sabotage** (Speed: Fast · Range: 1 · Target: invaders)
  - **L1** — 1 Moon + 2 Plant: 1 Fear and Defend 2.
  - **L2** — 2 Moon + 3 Plant: Instead, 1 Fear and Defend 4.
  - **L3** — 2 Moon + 2 Air + 4 Plant: Instead, 3 Fear and Defend 6.
  - **L4** — 3 Moon + 3 Air + 5 Plant: Instead, 5 Fear and Defend 12.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Moon, Uniques give 0; need 2 Plant, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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


### Strategy Cliffs — per-adversary-level shifts that change Eyes Watch from the Trees's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Eyes Watch from the Trees's profile (Fear 4, Offense 2, Control 3, Defense 5, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Eyes Watch from the Trees**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Eyes Watch from the Trees**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Eyes Watch from the Trees**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Eyes Watch from the Trees**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Eyes Watch from the Trees**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Eyes Watch from the Trees**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Eyes Watch from the Trees**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/eyes-watch-from-the-trees.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Eyes Watch from the Trees](https://spiritislandwiki.com/index.php?title=Eyes_Watch_from_the_Trees).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
