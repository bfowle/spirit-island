# Sharp Fangs Behind the Leaves

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch and Claw                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "two" — see Growth Options below         |
| Power summary (1–5)   | Offense 3 · Control 3 · Fear 4 · Defense 2 · Utility 1             |
| Primary Elements      | Animal, Moon, Plant, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> All about Beasts and Jungles. Can be very fast out of the gate, but doesn't have the late-game power that some spirits do, and is likely to have some difficulty with Blighted areas. "Ranging Hunt" is a critical Innate ability, particularly in early-game: it simultaneously gives Beasts mobility and permits picking off a stray Explorers or Towns on most turns.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Beast on your starting board in the highest-numbered Jungle. Put 1 Presence in a land of your choice with Beast anywhere on the island.

## Growth Options (two)

| Growth | Effects |
|--------|---------|
| G1 | first=Sharp1, second=gain1p |
| G2 | first=Sharp |
| G3 | first=gain1p, second=energy1 |
| G4 | first=energy3 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy1, animal, plant, energy2, animal, energy3, energy4
- **Card-play track**: card2, card2, card3, reclaim1, card4, card5reclaim1

## Core Mechanics & Special Rules

### Special Rule

ALLY OF THE BEASTS Your Presence may move with Beast. (Whenever a Beast moves from 1 of your lands to another land, you may move 1 Presence along with it.) CALL FORTH PREDATORS During each Spirit Phase, you may replace 1 of your Presence with 1 Beast. The replaced Presence leaves the game. (It was not destroyed, so things which return destroyed Presence cannot bring it back.)

### Innate: RANGING HUNT

- **Speed**: fast · **Range**: 1 · **Target**: noblight

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Animal | You may Gather 1 Beast. |
| 2 | 2 Plant + 3 Animal | 1 Damage per Beast. |
| 3 | 2 Animal | You may Push up to 2 Beast. |


### Innate: FRENZIED ASSAULT

- **Speed**: slow · **Range**: 1 · **Target**: beast

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Moon + 1 Fire + 4 Animal | 1 Fear and 2 Damage. Remove 1 Beast. |
| 2 | 1 Moon + 2 Fire + 5 Animal | +1 Fear and +1 Damage. |


## Unique Cards (all, Wiki-verified)

#### Prey on the Builders

- **1 Energy · Fast · Range 0 · Any Land · Moon, Fire, Animal**
- *You may Gather 1 Beasts. If target land has Beasts, Invaders do not Build there this turn.*

#### Teeth Gleam from Darkness

- **1 Energy · Slow · Range 1, from a Jungle · Land with no Blight · Moon, Plant, Animal**
- *1 Fear. Add 1 Beasts. **OR** If target land has both Beasts and Invaders: 3 Fear.*

#### Terrifying Chase

- **1 Energy · Slow · Range 0 · Any Land · Sun, Animal**
- *Push 2 Explorers/Towns/Dahan. Push another 2 Explorers/Towns/Dahan per Beasts in target land. If you Pushed any Invaders, 2 Fear.*

#### Too Near the Jungle

- **0 Energy · Slow · Range 1, from a Jungle · Any Land · Plant, Animal**
- *1 Fear. Destroy 1 Explorer.*

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Sharp Fangs Behind the Leaves's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/sharp-fangs-behind-the-leaves.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/sharp-fangs-behind-the-leaves.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Animal** (wt 10.8), **Fire** (wt 2.1), **Moon** (wt 1.5)
- **Mid-game energy estimate (T3–T5 avg)**: 3.5E
- **Power summary**: Offense 3 · Control 3 · Fear 4 · Defense 2 · Utility 1
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Prey on the Builders**
- **Teeth Gleam from Darkness**
- **Terrifying Chase**
- **Too Near the Jungle**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Inflame the Fires of Life** | 1 | Slow | Moon, Fire, Plant, Animal | Add 1 Disease. **OR** 1 Fear. Add 1 Strife. | elements animal+fire+moon+plant → 15.6 |
| 2 | **Domesticated Animals Go Berserk** | 1 | Fast | Moon, Fire, Animal | 1 Fear. Defend 5. | elements animal+fire+moon → 14.4 |
| 3 | **Prowling Panthers** | 1 | Slow | Moon, Fire, Animal | 1 Fear. Add 1 Beasts. **OR** If target land has Beasts, Destroy 1 Explorer/Town. | elements animal+fire+moon → 14.4 |
| 4 | **Quicken the Earth's Struggles** | 1 | Fast | Moon, Fire, Earth, Animal | 1 Damage to each Town/City. **OR** Defend 10. | elements animal+fire+moon → 14.4 |
| 5 | **Savage Mawbeasts** | 0 | Slow | Fire, Animal | If target land is a Jungle or Wetland, 1 Fear and 1 Damage. | elements animal+fire → 12.9 |
| 6 | **Weep for What is Lost** | 0 | Slow | Fire, Water, Animal | 1 Fear per type of Invader present. Push up to 1 Explorer/Town per Blight. | elements animal+fire → 12.9 |
| 7 | **Swarming Wasps** | 0 | Fast | Fire, Air, Animal | Add 1 Beasts. **OR** If target land has Beasts, Push up to 2 Explorers. | elements animal+fire → 12.9 |
| 8 | **Blood Draws Predators** | 1 | Fast | Sun, Fire, Water, Animal | After the next time Invaders are Destroyed in target land: Add 1 Beasts, then 1 Damage pe… | elements animal+fire → 12.9 |
| 9 | **Gold's Allure** | 0 | Slow | Fire, Earth, Animal | Gather 1 Explorer and 1 Town. Add 1 Strife. | elements animal+fire → 12.9 |
| 10 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer/Town/Dahan. 2 Fear. If target land has any Beasts, 1 Fear. | elements animal+moon → 12.3 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Settle Into Hunting-Grounds** | 3 | Fast | Moon, Fire, Plant, Animal | Your Presence may count as Badlands and Beasts. (Decide per Presence, per Action.) Your P… | elements animal+fire+moon+plant → 15.6 |
| 2 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements animal+fire+moon+plant → 15.6 |
| 3 | **Unearth a Beast of Wrathful Stone** | 5 | Fast | Moon, Fire, Earth, Animal | After the next Invader Phase (on any turn) with no Ravage/Build Actions in target land:</… | elements animal+fire+moon → 14.4 |
| 4 | **Unlock the Gates of Deepest Power** | 4 | Fast | Sun, Moon, Fire, Air, Water, Earth, Plant, Animal | Target Spirit gains a Major Power by drawing 2 and keeping 1, without having to Forget an… | elements animal+fire+moon+plant → 15.6 |
| 5 | **Vengeance of the Dead** | 3 | Fast | Moon, Fire, Animal | 3 Fear. After Towns/Cities/Dahan are Destroyed in target land, 1 Damage per Town/City/Dah… | elements animal+fire+moon → 14.4 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Sharp Fangs Behind the Leaves in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Pillar of Living Flame** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **The Jungle Hungers** | destroys Dahan |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Tsunami** | destroys Dahan |

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


### Strategy Cliffs — per-adversary-level shifts that change Sharp Fangs Behind the Leaves's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Sharp Fangs Behind the Leaves's profile (Fear 4, Offense 3, Control 3, Defense 2, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Sharp Fangs Behind the Leaves**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### England L3 — Coastal Lands build faster

**What changes**: England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**

**Mitigation for Sharp Fangs Behind the Leaves**: Front-load coastal defense or disruption before T3's first Ravage.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Sharp Fangs Behind the Leaves**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Sharp Fangs Behind the Leaves**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Sharp Fangs Behind the Leaves**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Sharp Fangs Behind the Leaves**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Sharp Fangs Behind the Leaves**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/sharp-fangs-behind-the-leaves.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Sharp Fangs Behind the Leaves](https://spiritislandwiki.com/index.php?title=Sharp_Fangs_Behind_the_Leaves).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
