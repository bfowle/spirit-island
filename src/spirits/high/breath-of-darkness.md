# Breath of Darkness Down Your Spine

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
| Power summary (1–5)   | Offense 2 · Control 4 · Fear 5 · Defense 1 · Utility 2             |
| Primary Elements      | Moon, Animal, Air (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Abducts lone Invaders to gain Fear and keep them off the board for a time; its mobile Incarna is particularly useful for this. Reclaiming permits the Invaders to escape its void en masse, so can be quite painful. Has trouble with built-up lands, and may need to scatter Invaders or take a Major Power to deal with them.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence and your Incarna ({{incarna|breath}}), Unempowered ({{incarna|unempowered}}), on your starting board: 1 Presence and {{incarna|breath}} in the lowest-numbered Jungle and 1 in the highest-numbered Jungle. Set [[The Endless Dark]] ({{endlessdark}}) tile next to the island with 1 Explorer on it. You start with your 4 Unique Power Cards and 0 Energy.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p, third=movebreathincarna |
| G2 | first=gain1p, second=addpresence3, third=darknessescape2 |
| G3 | first=addpresence1, second=addmovebreathincarna, third=darknessescape1 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, energy2, moon, energy3, empowerincarna, energy4animal, energy5air
- **Card-play track**: card2, movepres, card3, moonX, reclaim1, card4air

## Core Mechanics & Special Rules

### Special Rule

TERROR STALKS THE LAND</br>You have an Incarna ({{incarna|breath}}). Empower {{incarna|breath}} after uncovering {{empower}}. You may Abduct 1 Explorer/Town at empowered {{incarna|breath}} each Fast phase. To Abduct a piece, Move it to [[The Endless Dark]] ({{endlessdark}}). When pieces Escape, Move them to non-Ocean lands with your Presence/{{incarna|breath}}; if they have no legal land to move to, you lose. When your Powers would directly damage or directly destroy the only Invader in a land, instead Abduct it. (Check if it is alone at the start of the damage/destroy instruction. {{endlessdark}} is not a land.) SHADOW-TOUCHED REALM</br>Your land-targeting Powers can target {{endlessdark}} as if it were a land, ignoring Range. ({{endlessdark}} is Inland and has no terrain.) Rules for The Endless Dark</br>{{#lst:The Endless Dark|ted}}

### Innate: LEAVE A TRAIL OF DEATHLY SILENCE

- **Speed**: fast · **Range**: None · **Target**: yourself

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 1 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 2 | 3 Moon + 1 Air + 1 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 3 | 4 Moon + 2 Air + 2 Animal | 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}. |
| 4 | 5 Moon + 2 Air + 3 Animal | Move {{incarna|breath}} to {{endlessdark}}. It Brings 1 Invader (from its land). |


### Innate: LOST IN THE ENDLESS DARK

- **Speed**: slow · **Range**: None · **Target**: endlessdark

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 1 Air | 1 Fear per Invader (max. 4). Downgrade up to 1 Invader. (Downgrading Removes Explorer.) |
| 2 | 4 Moon + 3 Air | 1 Fear per Invader (max. 4). Downgrade any number of Invaders. |
| 3 | 3 Moon + 2 Animal | Add 1 Beast. |


## Unique Cards (all, Wiki-verified)

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Breath of Darkness Down Your Spine's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/breath-of-darkness.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/breath-of-darkness.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Moon** (wt 11.4), **Air** (wt 4.5), **Animal** (wt 3.6)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 2 · Control 4 · Fear 5 · Defense 1 · Utility 2
```

### Uniques

*No Unique cards listed.*

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer/Town/Dahan. 2 Fear. If target land has any Beasts, 1 Fear. | elements air+animal+moon → 19.5 |
| 2 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | elements air+animal+moon → 19.5 |
| 3 | **Veil the Night's Hunt** | 1 | Fast | Moon, Air, Animal | For each Dahan present, choose a different Invader. 1 Damage to each of those Invaders. *… | elements air+animal+moon → 19.5 |
| 4 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | elements air+animal+moon → 19.5 |
| 5 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. | elements air+moon → 15.9 |
| 6 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear. The next time an Invader is Destroyed in target land this turn, 1 Fear. | elements air+moon → 15.9 |
| 7 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+moon → 15.9 |
| 8 | **Land of Haunts and Embers** | 0 | Fast | Moon, Fire, Air | 2 Fear. Push up to 2 Explorers/Towns. If Blight is present, 2 Fear and Push up to 2 Explo… | elements air+moon → 15.9 |
| 9 | **Haunted by Primal Memories** | 1 | Fast | Moon, Air, Earth | 1 Fear. Defend 3. If Beasts are present, +2 Fear. | elements air+moon → 15.9 |
| 10 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear. Terror Level 2: 2 Fear or add 1 Strife.… | elements air+moon → 15.9 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sleep and Never Waken** | 3 | Fast | Moon, Air, Earth, Animal | Invaders skip all Actions in target land. 1 Fear per 2 Explorers this Power Removes. Remo… | elements air+animal+moon → 19.5 |
| 2 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+moon → 19.5 |
| 3 | **Terrifying Nightmares** | 4 | Fast | Moon, Air | 2 Fear. Push up to 4 Explorers/Towns. | elements air+moon → 15.9 |
| 4 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+moon → 15.9 |
| 5 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon → 15.9 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Breath of Darkness Down Your Spine in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```


*No HoSI beginner-deck bundle for this spirit.*


### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Renewing Boon** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pyroclastic Flow** | adds Blight |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
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
Auto-derived from `data/references/wiki/breath-of-darkness.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence and your Incarna ({{incarna|breath}}), Unempowered ({{incarna|unempowered}}), on your starting board: 1 Presence and {{incarna|breath}} in the lowest-numbered Jungle and 1 in the highest-numbered Jungle. Set [[The Endless Dark]] ({{endlessdark}}) tile next to the island with 1 Explorer on it. You start with your 4 Unique Power Cards and 0 Energy.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); movebreathincarna ((spirit-specific: `movebreathincarna` — consult spirit panel))
- **G2**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence3 (Place 1 Presence from a track (Range 3)); darknessescape2 ((spirit-specific: `darknessescape2` — consult spirit panel))
- **G3**: addpresence1 (Place 1 Presence from a track (Range 1)); addmovebreathincarna ((spirit-specific: `addmovebreathincarna` — consult spirit panel)); darknessescape1 ((spirit-specific: `darknessescape1` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · moon · energy3 · empowerincarna · energy4animal · energy5air` — income as slots reveal: 1 → 2 → moon → 3 → empowerincarna → 4 → 5

### Card-play track

`card2 · movepres · card3 · moonX · reclaim1 · card4air` — CP as slots reveal: 2 → movepres → 3 → moonX → reclaim1 → 4

### Innate Powers

- **LEAVE A TRAIL OF DEATHLY SILENCE** (Speed: Fast · Range: ? · Target: yourself)
  - **L1** — 2 Moon + 1 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L2** — 3 Moon + 1 Air + 1 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L3** — 4 Moon + 2 Air + 2 Animal: 1 Damage at {{incarna|breath}}. You may Push {{incarna|breath}}.
  - **L4** — 5 Moon + 2 Air + 3 Animal: Move {{incarna|breath}} to {{endlessdark}}. It Brings 1 Invader (from its land).
- **LOST IN THE ENDLESS DARK** (Speed: Slow · Range: ? · Target: endlessdark)
  - **L1** — 2 Moon + 1 Air: 1 Fear per Invader (max. 4). Downgrade up to 1 Invader. (Downgrading Removes Explorer.)
  - **L2** — 4 Moon + 3 Air: 1 Fear per Invader (max. 4). Downgrade any number of Invaders.
  - **L3** — 3 Moon + 2 Animal: Add 1 Beast.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 2 Moon, Uniques give 0; need 1 Animal, Uniques give 0. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

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


### Strategy Cliffs — per-adversary-level shifts that change Breath of Darkness Down Your Spine's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Breath of Darkness Down Your Spine's profile (Fear 5, Offense 2, Control 4, Defense 1, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Breath of Darkness Down Your Spine**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Breath of Darkness Down Your Spine**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Breath of Darkness Down Your Spine**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Breath of Darkness Down Your Spine**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Breath of Darkness Down Your Spine**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Breath of Darkness Down Your Spine**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Breath of Darkness Down Your Spine**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/breath-of-darkness-down-your-spine.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Breath of Darkness Down Your Spine](https://spiritislandwiki.com/index.php?title=Breath_of_Darkness_Down_Your_Spine).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
