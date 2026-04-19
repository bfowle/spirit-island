# Grinning Trickster Stirs Up Trouble

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "two" — see Growth Options below         |
| Power summary (1–5)   | Offense 4 · Control 3 · Fear 2 · Defense 5 · Utility 4             |
| Primary Elements      | Fire, Air, Moon, Animal (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Requires some comfort with risk: both Overenthusiastic Arson and Let's See What Will Happen involve uncertainty about how the Fast Powers phase will pan out. Can be effective from the get-go, but benefits greatly from not working too hard, instead improving its capacity for mischief by adding Presence and gaining Power Cards. Bonus Energy from Let's See What Will Happen can be extremely helpful in avoiding the distraction of gaining Energy elsewhere.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board: 1 in the highest-numbered land with Dahan, and 1 in land #4.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=Sharp1, second=movepresence1 |
| G2 | first=addpresence2 |
| G3 | first=gain1p |
| G4 | first=energycardplays |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, moon, energy2, any, fire, energy3
- **Card-play track**: card2, pushdahan, card3, card3, card4, airX, card5

## Core Mechanics & Special Rules

### Special Rule

A REAL FLAIR FOR DISCORD After one of your Powers adds Strife in a land, you may pay 1 Energy to add 1 Strife within Range 1 of that land. CLEANING UP MESSES IS A DRAG After one of your Powers Removes Blight, Destroy 1 of your Presence. Ignore this rule for Let's See What Happens.

### Innate: LET'S SEE WHAT HAPPENS

- **Speed**: fast · **Range**: 1 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Fire + 2 Air | Discard Minor Powers from the deck until you get one that targets a land. Use its text effects on target land immediately, ignoring normal Range/Targeting restrictions. All "up to" instructions must be used at max. value. Treat all "OR"s as "AND"s. (It is not considered a card of yours or a card in play. Its effects are treated as performed by this Power, as if its text were copied here.) |
| 2 | 2 Moon + 1 Fire + 2 Air | You may Forget a Power Card to gain the just-used Power Card (to hand) and 1 Energy. |


### Innate: WHY DON'T YOU AND THEM FIGHT

- **Speed**: fast · **Range**: 0 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 3 Moon | This Power may be Slow. |
| 2 | 3 Air | Add 1 Strife. |
| 3 | 3 Sun + 3 Fire | 1 Invader and 1 Dahan deal Damage to each other. |
| 4 | 3 Animal | If target land has Beast, 2 Damage. Otherwise, you may Gather 1 Beast. |


## Unique Cards (all, Wiki-verified)

#### Impersonate Authority

- **0 Energy · Slow · Range 1 · Any Land · Sun, Air, Animal**
- *Add 1 Strife.*

#### Incite the Mob

- **1 Energy · Slow · Range 1 · Land with 1 or more Invaders · Moon, Fire, Air, Animal**
- *1 Invader with Strife deals Damage to other Invaders (not to each Invader). 1 Fear per Invader this Power Destroyed.*

#### Overenthusiastic Arson

- **1 Energy · Fast · Range 1 · Any Land · Fire, Air**
- *Destroy 1 Town. Discard the top card of the Minor Power Deck. If it provides Fire: 1 Fear, 2 Damage, and add 1 Blight.*

#### Unexpected Tigers

- **0 Energy · Slow · Range 1 · Any Land · Moon, Fire, Animal**
- *1 Fear if Invaders are present. If you can gather 1 Beasts, do so, then push 1 Explorer. Otherwise, add 1 Beasts.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Grinning Trickster Stirs Up Trouble's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/grinning-trickster.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/grinning-trickster.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Air** (wt 4.8), **Moon** (wt 4.8), **Fire** (wt 2.4)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 4 · Control 3 · Fear 2 · Defense 5 · Utility 4
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Impersonate Authority**
- **Incite the Mob**
- **Overenthusiastic Arson**
- **Unexpected Tigers**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+fire+moon → 12.0 |
| 2 | **Land of Haunts and Embers** | 0 | Fast | Moon, Fire, Air | 2 Fear. Push up to 2 Explorers/Towns. If Blight is present, 2 Fear and Push up to 2 Explo… | elements air+fire+moon → 12.0 |
| 3 | **Twilight Fog Brings Madness** | 0 | Slow | Sun, Moon, Air, Water | Add 1 Strife. Push 1 Dahan. Each remaining Dahan takes 1 Damage. | elements air+moon+sun → 10.5 |
| 4 | **Veil the Night's Hunt** | 1 | Fast | Moon, Air, Animal | For each Dahan present, choose a different Invader. 1 Damage to each of those Invaders. *… | elements air+animal+moon → 10.5 |
| 5 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear. The next time an Invader is Destroyed in target land this turn, 1 Fear. | elements air+moon+sun → 10.5 |
| 6 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | elements air+animal+moon → 10.5 |
| 7 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. | elements air+moon+sun → 10.5 |
| 8 | **Entrancing Apparitions** | 1 | Fast | Moon, Air, Water | Defend 2. If no Invaders are present, Gather up to 2 Explorers. | elements air+moon → 9.6 |
| 9 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer/Town/Dahan. 2 Fear. If target land has any Beasts, 1 Fear. | elements air+animal+moon → 10.5 |
| 10 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | elements air+animal+moon → 10.5 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+fire+moon+sun → 13.8 |
| 2 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+fire+moon → 12.0 |
| 3 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon+sun → 10.5 |
| 4 | **Twisted Flowers Murmur Ultimatums** | 5 | Slow | Sun, Moon, Air, Earth, Plant | 4 Fear. Add 1 Strife. If the Terror Level is 2 or higher, Remove 2 Invaders. | elements air+moon+sun → 10.5 |
| 5 | **Sleep and Never Waken** | 3 | Fast | Moon, Air, Earth, Animal | Invaders skip all Actions in target land. 1 Fear per 2 Explorers this Power Removes. Remo… | elements air+animal+moon → 10.5 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Grinning Trickster Stirs Up Trouble in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Pyroclastic Flow** | adds Blight |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/grinning-trickster.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board: 1 in the highest-numbered land with Dahan, and 1 in land #4.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card2`): **1 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `two` — (see spirit panel)

### Growth options

- **G1**: Sharp1 ((spirit-specific: `Sharp1` — consult spirit panel)); movepresence1 (Move 1 Presence (Range 1))
- **G2**: addpresence2 (Place 1 Presence from a track (Range 2))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G4**: energycardplays ((spirit-specific: `energycardplays` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · moon · energy2 · any · fire · energy3` — income as slots reveal: 1 → moon → 2 → any → fire → 3

### Card-play track

`card2 · pushdahan · card3 · card3 · card4 · airX · card5` — CP as slots reveal: 2 → pushdahan → 3 → 3 → 4 → airX → 5

### Innate Powers

- **LET'S SEE WHAT HAPPENS** (Speed: Fast · Range: 1 · Target: invaders)
  - **L1** — 1 Moon + 1 Fire + 2 Air: Discard Minor Powers from the deck until you get one that targets a land. Use its text effects on target land immediately, ignoring normal Range/Targeting restrictions. All "up to" instructions must be used at max. value. Treat all "OR"s as "AND"s. (It is not considered a card of yours or a card in play. Its effects are treated as performed by this Power, as if its text were copied here.)
  - **L2** — 2 Moon + 1 Fire + 2 Air: You may Forget a Power Card to gain the just-used Power Card (to hand) and 1 Energy.
- **WHY DON'T YOU AND THEM FIGHT** (Speed: Fast · Range: 0 · Target: invaders)
  - **L1** — 3 Moon: This Power may be Slow.
  - **L2** — 3 Air: Add 1 Strife.
  - **L3** — 3 Sun + 3 Fire: 1 Invader and 1 Dahan deal Damage to each other.
  - **L4** — 3 Animal: If target land has Beast, 2 Damage. Otherwise, you may Gather 1 Beast.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Fire** ×1, **Air** ×1
- **Fast-phase L1 ceiling from Uniques alone is insufficient** — need 1 Moon, Uniques give 0; need 2 Air, Uniques give 1. L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Impersonate Authority** | 0 | Slow | 1 | Any Land | sun, air, animal | Add 1 Strife. |
| **Incite the Mob** | 1 | Slow | 1 | Land with 1 or more Invaders | moon, fire, air, animal | 1 Invader with Strife deals Damage to other Invaders (not to each Invader). 1 Fear per Invader this… |
| **Overenthusiastic Arson** | 1 | Fast | 1 | Any Land | fire, air | Destroy 1 Town. Discard the top card of the Minor Power Deck. If it provides Fire: 1 Fear, 2 Damage… |
| **Unexpected Tigers** | 0 | Slow | 1 | Any Land | moon, fire, animal | 1 Fear if Invaders are present. If you can gather 1 Beasts, do so, then push 1 Explorer. Otherwise,… |

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


### Strategy Cliffs — per-adversary-level shifts that change Grinning Trickster Stirs Up Trouble's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Grinning Trickster Stirs Up Trouble's profile (Fear 2, Offense 4, Control 3, Defense 5, Utility 4).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Grinning Trickster Stirs Up Trouble**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Grinning Trickster Stirs Up Trouble**: Front-load coastal defense or disruption before T3's first Ravage.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Grinning Trickster Stirs Up Trouble**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Grinning Trickster Stirs Up Trouble**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/grinning-trickster-stirs-up-trouble.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Grinning Trickster Stirs Up Trouble](https://spiritislandwiki.com/index.php?title=Grinning_Trickster_Stirs_Up_Trouble).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
