# Fractured Days Split the Sky

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Very High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 2 · Fear 1 · Defense 3 · Utility 5             |
| Primary Elements      | Sun, Moon, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Excellent at support and sweeping indirect effects, but starts off very limited otherwise. Several of its Unique Powers need setup to use well; it's entirely possible 1 or 2 of them may see no play in a given game. Has a hard time getting lots of Presence onto the board. This can make targeting tricky, and may be quite dangerous if a Blighted Island effect Destroys Presence.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 3 Presence on your starting board: 1 in the lowest-numbered land with 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">, and 2 in the highest-numbered land without Dahan. Deal 4 Minor and Major Powers face-up as your initial Days That Never Were cards; in a 1 or 2-player game, instead deal 6 of each. In a 1-board game, gain 1 Time.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=gainair, second=reclaim, third=gain2time |
| G2 | first=gainmoon, second=gain1p, third=addpresence2 |
| G3 | first=gainsun, second=movepresence4, third=daysthatneverwerecard |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy1, energy2, energy2, energy2, energy3
- **Card-play track**: card1, card1, card1, card2, card2, card3

## Core Mechanics & Special Rules

### Special Rule

FRAGMENTS OF SHATTERED TIME Each Presence on this ability represents 1 Time. Many of your Powers require Time as an additional cost. Spend it when you Resolve the Power. (Not when you play it.) When you Gain 1 Time, put 1 of your Presence here from your Presence track (or, optionally, the island). When you Spend 1 Time, return it to a Presence track - or if you have no free spaces, Destroy it. DAYS THAT NEVER WERE Your 3rd Growth option lets you gain any one Power Card from a special set you create during Setup. When you gain a Power Card any other way, you may add one unchosen card to this set.

### Innate: SLIP THE FLOW OF TIME

- **Speed**: fast · **Range**: None · **Target**: anyspirit

_(no thresholds listed in Wiki)_


### Innate: VISIONS OF A SHIFTING FUTURE

- **Speed**: slow · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.) |
| 2 | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Fractured Days Split the Sky's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/fractured-days-split-the-sky.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/fractured-days-split-the-sky.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Moon** (wt 3.6), **Air** (wt 3.0), **Sun** (wt 2.1)
- **Mid-game energy estimate (T3–T5 avg)**: 2.0E
- **Power summary**: Offense 1 · Control 2 · Fear 1 · Defense 3 · Utility 5
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. The next time an Invader is Destroyed in target land this turn, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 8.7 |
| 2 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon+sun → 8.7 |
| 3 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. Each remaining Dahan takes 1 Damage. | elements air+moon+sun → 8.7 |
| 4 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+moon → 6.6 |
| 5 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. | elements air+moon → 6.6 |
| 6 | **Haunted by Primal Memories** | 1 | Fast | Moon, Air, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Defend 3. If Beasts are present, +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon → 6.6 |
| 7 | **Disorienting Landscape** | 1 | Fast | Moon, Air, Plant | Push 1 [[Explorer]]. If target land is a Mountain or Jungle, add 1 [[Wilds]]. | elements air+moon → 6.6 |
| 8 | **Gift of Twinned Days** | 1 | Fast | Sun, Moon | Once this turn, target Spirit may Repeat the lowest-cost Power Card they have in play by … | elements moon+sun → 5.7 |
| 9 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town/Dahan. 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land has any Beasts, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. | elements air+moon → 6.6 |
| 10 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Terror Level 2: 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> or add 1 Strife.… | elements air+moon → 6.6 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon+sun → 8.7 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+moon+sun → 8.7 |
| 3 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+moon+sun → 8.7 |
| 4 | **Solidify Echoes of Majesty Past** | 4 | Fast | Sun, Moon, Air, Earth | Choose one of target Spirit's lands. In that land and each adjacent land, Defend 3. They … | elements air+moon+sun → 8.7 |
| 5 | **Bargain of Coursing Paths** | 2 | Fast | Moon, Air, Water, Earth | Bargain: 1 Presence now and -1 Energy/turn. Now: Mark both target land and another land w… | elements air+moon → 6.6 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Fractured Days Split the Sky in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Renewing Boon** | destroys Presence |
| **Devouring Ants** | destroys Dahan |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Tsunami** | destroys Dahan |
| **Volcanic Eruption** | destroys Dahan, adds Blight |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/fractured-days-split-the-sky.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 3 Presence on your starting board: 1 in the lowest-numbered land with 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">, and 2 in the highest-numbered land without Dahan. Deal 4 Minor and Major Powers face-up as your initial Days That Never Were cards; in a 1 or 2-player game, instead deal 6 of each. In a 1-board game, gain 1 Time.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: gainair ((spirit-specific: `gainair` — consult spirit panel)); reclaim (Reclaim all discarded+played Power Cards); gain2time ((spirit-specific: `gain2time` — consult spirit panel))
- **G2**: gainmoon ((spirit-specific: `gainmoon` — consult spirit panel)); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: gainsun ((spirit-specific: `gainsun` — consult spirit panel)); movepresence4 (Move 1 Presence (Range 4)); daysthatneverwerecard ((spirit-specific: `daysthatneverwerecard` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy1 · energy2 · energy2 · energy2 · energy3` — income as slots reveal: 1 → 1 → 2 → 2 → 2 → 3

### Card-play track

`card1 · card1 · card1 · card2 · card2 · card3` — CP as slots reveal: 1 → 1 → 1 → 2 → 2 → 3

### Innate Powers

- **SLIP THE FLOW OF TIME** (Speed: Fast · Range: ? · Target: anyspirit)
- **VISIONS OF A SHIFTING FUTURE** (Speed: Slow · Range: ? · Target: yourself)
  - **L1** — 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.)
  - **L2** — 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air: Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/fractured-days-split-the-sky.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Fractured Days Split the Sky](https://spiritislandwiki.com/index.php?title=Fractured_Days_Split_the_Sky).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
