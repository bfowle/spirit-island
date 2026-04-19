# Bringer of Dreams and Nightmares

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                        |
| Complexity            | High                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 2 · Fear 5 · Defense 2 · Utility 2             |
| Primary Elements      | Moon, Air, Animal, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> With most Spirits, Terror Victories are a backup plan if the main push against the Invaders stalls out for too long, but Bringer turns Fear into a more viable primary strategy. Its transformation of damage & destruction into Fear can turn Major Powers into tremendous sources of terror and panic. However, the only real offense Bringer has is the Dahan fighting back. While it does have some defensive ability, it is fundamentally poor at clearing areas of Invaders.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 2 Presence on your starting board in the highest-numbered Sands.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=reclaim1, second=addpresence0 |
| G3 | first=gain1p, second=addpresence1 |
| G4 | first=Night, second=energy2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy2, air, energy3, moon, energy4, any, energy5
- **Card-play track**: card2, card2, card2, card3, card3, any

## Core Mechanics & Special Rules

### Special Rule

TO DREAM A THOUSAND DEATHS Your Powers never cause Damage, nor can they Destroy anything other than your own Presence. When your Powers would Destroy (or deal enough Damage to Destroy) Explorer/Town/City, instead generate 0/2/5 Fear. The Power Pushes all Explorer/Town it would Destroy. Notes: A single Power cannot Destroy a given Invader more than once. Powers that cause Damage via Dahan are affected just like all others. All effects other than Damage/Destroy work as usual.

### Innate: SPIRITS MAY YET DREAM

- **Speed**: fast · **Range**: None · **Target**: anyspirit

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Moon + 2 Air | Turn any face down Fear Card face-up. (It's earned/resolved normally, but players can see what's coming) |
| 2 | 3 Moon | Target Spirit gains an element that they have at least 1 of. |


### Innate: NIGHT TERRORS

- **Speed**: fast · **Range**: 0 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Air | 1 Fear. |
| 2 | 2 Moon + 1 Air + 1 Animal | +1 Fear. |
| 3 | 3 Moon + 2 Air + 1 Animal | +1 Fear. |


## Unique Cards (all, Wiki-verified)

#### Call on Midnight's Dream

- **0 Energy · Fast · Range 0 · Any Land · Moon, Animal**
- *If target land has Dahan, gain a Major Power. If you Forget this Power, gain Energy equal to Dahan and you may play the Major Power immediately, paying its cost. **OR** If Invaders are present, 2 Fear.*

#### Dread Apparitions

- **2 Energy · Fast · Range 1 · Land with 1 or more Invaders · Moon, Air**
- *When Powers generate Fear in target land, Defend 1 per Fear. 1 Fear. (Fear from To Dream a Thousand Deaths counts. Fear from Destroying Towns/Cities does not.)*

#### Dreams of the Dahan

- **0 Energy · Fast · Range 2 · Any Land · Moon, Air**
- *Gather up to 2 [[Dahan]]. **OR** If target land has [[Towns]]/[[Cities]], 1 [[Fear]] for each [[Dahan]], to a maximum of 3 [[Fear]].*

#### Predatory Nightmares

- **2 Energy · Slow · Range 1, from your Sacred Site · Land with 1 or more Invaders · Moon, Fire, Earth, Animal**
- *2 Damage. Push up to 2 Dahan. (When your Powers would Destroy Invaders, instead they generate Fear and/or Push those Invaders.)*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Bringer of Dreams and Nightmares's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/bringer-of-dreams-and-nightmares.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/bringer-of-dreams-and-nightmares.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Moon** (wt 6.6), **Air** (wt 3.9), **Animal** (wt 0.9)
- **Mid-game energy estimate (T3–T5 avg)**: 4.5E
- **Power summary**: Offense 1 · Control 2 · Fear 5 · Defense 2 · Utility 2
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Call on Midnight's Dream**
- **Dread Apparitions**
- **Dreams of the Dahan**
- **Predatory Nightmares**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer/Town/Dahan. 2 Fear. If target land has any Beasts, 1 Fear. | elements air+animal+moon → 11.4 |
| 2 | **Portents of Disaster** | 0 | Fast | Sun, Moon, Air | 2 Fear. The next time an Invader is Destroyed in target land this turn, 1 Fear. | elements air+moon → 10.5 |
| 3 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | elements air+animal+moon → 11.4 |
| 4 | **Dire Metamorphosis** | 1 | Slow | Moon, Air, Earth, Animal | 1 Fear. 1 Damage. 1 Damage to [[Dahan]]. Add 1 [[Badlands]], 1 [[Beasts]], 1 [[Disease]],… | elements air+animal+moon → 11.4 |
| 5 | **Delusions of Danger** | 1 | Fast | Sun, Moon, Air | Push 1 Explorer. **OR** 2 Fear. | elements air+moon → 10.5 |
| 6 | **Haunted by Primal Memories** | 1 | Fast | Moon, Air, Earth | 1 Fear. Defend 3. If Beasts are present, +2 Fear. | elements air+moon → 10.5 |
| 7 | **Terror Turns to Madness** | 0 | Slow | Moon, Air, Water | If the Terror Level is... Terror Level 1: 3 Fear. Terror Level 2: 2 Fear or add 1 Strife.… | elements air+moon → 10.5 |
| 8 | **Veil the Night's Hunt** | 1 | Fast | Moon, Air, Animal | For each Dahan present, choose a different Invader. 1 Damage to each of those Invaders. *… | elements air+animal+moon → 11.4 |
| 9 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer/Town. | elements air+moon → 10.5 |
| 10 | **Land of Haunts and Embers** | 0 | Fast | Moon, Fire, Air | 2 Fear. Push up to 2 Explorers/Towns. If Blight is present, 2 Fear and Push up to 2 Explo… | elements air+moon → 10.5 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Sleep and Never Waken** | 3 | Fast | Moon, Air, Earth, Animal | Invaders skip all Actions in target land. 1 Fear per 2 Explorers this Power Removes. Remo… | elements air+animal+moon → 11.4 |
| 2 | **Terrifying Nightmares** | 4 | Fast | Moon, Air | 2 Fear. Push up to 4 Explorers/Towns. | elements air+moon → 10.5 |
| 3 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements air+animal+moon → 11.4 |
| 4 | **Weave Together the Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Target land and a land adjacent to it become a single land for this turn. (It has the ter… | elements air+moon → 10.5 |
| 5 | **Death Falls Gently from Open Blossoms** | 4 | Slow | Moon, Air, Plant | 4 Damage. If any Invaders remain, add 1 Disease. | elements air+moon → 10.5 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Bringer of Dreams and Nightmares in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
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
Auto-derived from `data/references/wiki/bringer-of-dreams-and-nightmares.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 2 Presence on your starting board in the highest-numbered Sands.
- **Starting income** (from `presence_energy_track[0]` = `energy2`, `presence_cardplay_track[0]` = `card2`): **2 Energy · 2 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted))
- **G2**: reclaim1 (Reclaim 1 Power Card (of your choice)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence1 (Place 1 Presence from a track (Range 1))
- **G4**: Night ((spirit-specific: `Night` — consult spirit panel)); energy2 ((track slot showing 2 Energy))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy2 · air · energy3 · moon · energy4 · any · energy5` — income as slots reveal: 2 → air → 3 → moon → 4 → any → 5

### Card-play track

`card2 · card2 · card2 · card3 · card3 · any` — CP as slots reveal: 2 → 2 → 2 → 3 → 3 → any

### Innate Powers

- **SPIRITS MAY YET DREAM** (Speed: Fast · Range: ? · Target: anyspirit)
  - **L1** — 2 Moon + 2 Air: Turn any face down Fear Card face-up. (It's earned/resolved normally, but players can see what's coming)
  - **L2** — 3 Moon: Target Spirit gains an element that they have at least 1 of.
- **NIGHT TERRORS** (Speed: Fast · Range: 0 · Target: invaders)
  - **L1** — 1 Moon + 1 Air: 1 Fear.
  - **L2** — 2 Moon + 1 Air + 1 Animal: +1 Fear.
  - **L3** — 3 Moon + 2 Air + 1 Animal: +1 Fear.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: **Moon** ×3, **Air** ×2, **Animal** ×1
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

| Card | Cost | Speed | Range | Target | Elements | Effect |
|------|------|-------|-------|--------|----------|--------|
| **Call on Midnight's Dream** | 0 | Fast | 0 | Any Land | moon, animal | If target land has Dahan, gain a Major Power. If you Forget this Power, gain Energy equal to Dahan … |
| **Dread Apparitions** | 2 | Fast | 1 | Land with 1 or more Invaders | moon, air | When Powers generate Fear in target land, Defend 1 per Fear. 1 Fear. (Fear from To Dream a Thousand… |
| **Dreams of the Dahan** | 0 | Fast | 2 | Any Land | moon, air | Gather up to 2 [[Dahan]]. **OR** If target land has [[Towns]]/[[Cities]], 1 [[Fear]] for each [[Dah… |
| **Predatory Nightmares** | 2 | Slow | 1, from your Sacred Site | Land with 1 or more Invaders | moon, fire, earth, animal | 2 Damage. Push up to 2 Dahan. (When your Powers would Destroy Invaders, instead they generate Fear … |

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


### Strategy Cliffs — per-adversary-level shifts that change Bringer of Dreams and Nightmares's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Bringer of Dreams and Nightmares's profile (Fear 5, Offense 1, Control 2, Defense 2, Utility 2).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Bringer of Dreams and Nightmares**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Bringer of Dreams and Nightmares**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Bringer of Dreams and Nightmares**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Bringer of Dreams and Nightmares**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Bringer of Dreams and Nightmares**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Bringer of Dreams and Nightmares**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/bringer-of-dreams-and-nightmares.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Bringer of Dreams and Nightmares](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
