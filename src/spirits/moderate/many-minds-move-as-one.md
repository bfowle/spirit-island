# Many Minds Move as One

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                        |
| Complexity            | Moderate                                       |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "one" — see Growth Options below         |
| Power summary (1–5)   | Offense 1 · Control 5 · Fear 5 · Defense 5 · Utility 1             |
| Primary Elements      | Animal, Air, Water, Fire (derived from innates + uniques)|
| Aspects               | `[VERIFY from physical aspect panels]` |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> Requires heavy spatial thought for Beast movement, due to its improved Push/Gather and large numbers of Beast. Has no offense to start with, but an excellent stalling defense combined with Fear generation; outright Fear victories may be plausible in smaller games. Both Fear Cards and Beast events are unpredictable, however, so swings of fortune are apt to be more relevant than usual.

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> Put 1 Presence and 1 Beast on your starting board, in a land with Beast. Note that you have 5 Unique Power Cards.

## Growth Options (one)

| Growth | Effects |
|--------|---------|
| G1 | first=reclaim, second=gain1p |
| G2 | first=addpresence1, second=addpresence0 |
| G3 | first=addpresence3beast, second=energy1, third=gatherbeast2 |

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: energy0, energy1, air, energy2, animal, energy3, energy4
- **Card-play track**: card1, card2, pay2pcard, card3, card3, card4, card5

## Core Mechanics & Special Rules

### Special Rule

FLY FAST AS THOUGHT When you Gather or Push Beast, they may come from or go to lands up to 2 distant (rather than adjacent only). A JOINING OF SWARMS AND FLOCKS Your Sacred Site may also count as Beast. (Note: You never have more than 1 Sacred Site in a land, no matter how many Presence you have there.) (If something changes a Beast that is your Sacred Site, it affects 2 of your Presence there - e.g., Push 1 Beast will Push 2 of your Presence together.)

### Innate: THE TEEMING HOST ARRIVES

- **Speed**: fast · **Range**: 2 · **Target**: any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 2 Air + 1 Animal | Gather up to 1 Beasts. |
| 2 | 3 Air + 1 Water + 2 Animal | Instead, Gather up to 1 Beasts per Air you have. |
| 3 | 1 Fire + 4 Air + 2 Animal | Push up to 3 Beasts. |


### Innate: BESET AND CONFOUND THE INVADERS

- **Speed**: fast · **Range**: 2 · **Target**: invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1 | 1 Air + 2 Animal | 2 Fear and Defend 2. |
| 2 | 2 Air + 3 Animal | Instead, 3 Fear and Defend 4. |
| 3 | 3 Air + 4 Animal | Instead, 4 Fear and Defend 7. |
| 4 | 4 Air + 1 Earth + 5 Animal | Instead, 6 Fear and Defend 10. |


## Unique Cards (all, Wiki-verified)

#### A Dreadful Tide of Scurrying Flesh

- **0 Energy · Fast · Range Range 1, from your Sacred Site · Land with 2 or more Beasts tokens · Moon, Air, Water, Animal**
- *Remove up to half (round down) of Beasts in target land. For each Beasts Removed, 2 Fear and skip one Invader Action.*

#### Boon of Swarming Bedevilment

- **0 Energy · Fast · Range No Range · Another Spirit · Air, Water, Animal**
- *For the rest of this turn, each of target Spirit's Presence grants Defend 1 in its land. Target Spirit may Push up to 1 of their Presence.*

#### Ever-Multiplying Swarm

- **1 Energy · Slow · Range 0 · Any Land · Fire, Earth, Animal**
- *Add 2 Beasts.*

#### Guide the Way on Feathered Wings

- **0 Energy · Fast · Range 1 · Any Land · Sun, Air, Animal**
- *Move 1 Beasts up to two lands. As it moves, up to 2 Dahan may move with it, for part or all of the way. (The Beasts/Dahan may move to an adjacent land and then back.)*

- **Pursue with Scratches$ Pecks$ and Stings** — `[VERIFY]` (Wiki fetch error: Wiki API error for 'Pursue_with_Scratches$_Pecks$_and_Stings': {'code': 'missingtitle', 'info': "The page you specified doesn't exist.", 'docref': 'See https://spiritislandwiki.com/api.php for API usage. Subscribe to the mediawiki-api-announce mailing list at &lt;https://lists.wikimedia.org/mailman/listinfo/mediawiki-api-announce&gt; for notice of API deprecations and breaking changes.'})

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Many Minds Move as One's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/many-minds-move-as-one.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/many-minds-move-as-one.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Animal** (wt 9.0), **Air** (wt 9.0), **Water** (wt 0.6)
- **Mid-game energy estimate (T3–T5 avg)**: 3.0E
- **Power summary**: Offense 1 · Control 5 · Fear 5 · Defense 5 · Utility 1
```

### Uniques

The spirit's own 5 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **A Dreadful Tide of Scurrying Flesh**
- **Boon of Swarming Bedevilment**
- **Ever-Multiplying Swarm**
- **Guide the Way on Feathered Wings**
- **Pursue with Scratches$ Pecks$ and Stings**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Here There Be Monsters** | 0 | Slow | Moon, Air, Animal | You may Push 1 Explorer/Town/Dahan. 2 Fear. If target land has any Beasts, 1 Fear. | elements air+animal → 18.0 |
| 2 | **Set Them on an Ever-Twisting Trail** | 1 | Fast | Air, Plant, Animal | Gather or Push 1 Explorer. Isolate target land. | elements air+animal → 18.0 |
| 3 | **Swarming Wasps** | 0 | Fast | Fire, Air, Animal | Add 1 Beasts. **OR** If target land has Beasts, Push up to 2 Explorers. | elements air+animal+fire → 18.3 |
| 4 | **Rain of Blood** | 0 | Slow | Air, Water, Animal | 2 Fear. If target land has at least 2 Towns/Cities, 1 Fear. | elements air+animal+water → 18.6 |
| 5 | **Call to Isolation** | 0 | Fast | Sun, Air, Animal | Push 1 Explorer/Town per Dahan. **OR** Push 1 Dahan. | elements air+animal → 18.0 |
| 6 | **Bats Scout for Raids by Darkness** | 1 | Slow | Moon, Air, Animal | For each [[Dahan]], 1 Damage to [[Towns]]/[[Cities]]. **OR** 1 [[Fear]]. [[Gather]] up to… | elements air+animal → 18.0 |
| 7 | **Fleshrot Fever** | 1 | Slow | Fire, Air, Water, Animal | 1 Fear. Add 1 Disease. | elements air+animal+fire+water → 18.9 |
| 8 | **Birds Cry Warning** | 1 | Fast | Sun, Air, Animal | The next time Dahan would be Destroyed in target land, Destroy 2 fewer Dahan. **OR** Push… | elements air+animal → 18.0 |
| 9 | **Veil the Night's Hunt** | 1 | Fast | Moon, Air, Animal | For each Dahan present, choose a different Invader. 1 Damage to each of those Invaders. *… | elements air+animal → 18.0 |
| 10 | **Call to Migrate** | 1 | Slow | Fire, Air, Animal | Gather up to 3 Dahan. Push up to 3 Dahan. | elements air+animal+fire → 18.3 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Winds of Rust and Atrophy** | 3 | Fast | Air, Water, Animal | 1 Fear and Defend 6. Downgrade 1 Town/City. | elements air+animal+water → 18.6 |
| 2 | **Infestation of Venomous Spiders** | 4 | Fast | Air, Earth, Plant, Animal | Add 1 Beasts. Gather up to 1 Beasts. For each Beasts, 1 Fear (max. 4) and Invaders skip o… | elements air+animal+earth → 18.3 |
| 3 | **Plague Ships Sail to Distant Ports** | 4 | Fast | Fire, Air, Water, Animal | 1 Fear. Add 4 Disease among Coastal lands (on any boards) other than target land. | elements air+animal+fire+water → 18.9 |
| 4 | **Wrap in Wings of Sunlight** | 3 | Fast | Sun, Air, Animal | Move up to 5 Dahan to any land (including back into target land). If you moved at least 1… | elements air+animal → 18.0 |
| 5 | **Flocking Red-Talons** | 3 | Fast | Air, Water, Plant, Animal | Add 1 Beasts. Move up to 2 Beasts within 3 Range to target land. For each Beasts present,… | elements air+animal+water → 18.6 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Many Minds Move as One in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
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
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Pyroclastic Flow** | adds Blight |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Poisoned Land** | destroys Dahan, adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **Blazing Renewal** | destroys Presence |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **The Jungle Hungers** | destroys Dahan |
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


### Strategy Cliffs — per-adversary-level shifts that change Many Minds Move as One's math

```admonish warning title="Cliffs to watch"
Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to Many Minds Move as One's profile (Fear 5, Offense 1, Control 5, Defense 5, Utility 1).
```

#### England L5 — Buildings +1 HP

**What changes**: Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**

**Mitigation for Many Minds Move as One**: Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.

#### Sweden L2+ — Fear-card effects reduced

**What changes**: Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**

**Mitigation for Many Minds Move as One**: Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.

#### Russia L3+ — Dahan under pressure + fear suppression

**What changes**: Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**

**Mitigation for Many Minds Move as One**: Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.

#### Habsburg Mining L5+ — Explorer/Town scaling

**What changes**: Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**

**Mitigation for Many Minds Move as One**: Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.

#### France (Plantation) — Dahan capture threatens your Dahan engine

**What changes**: France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**

**Mitigation for Many Minds Move as One**: Play Defend Powers on Dahan lands; accept loss of range-extension budget.

#### Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)

**What changes**: BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**

**Mitigation for Many Minds Move as One**: Aim at City-dense lands with your highest-damage plays for outsized Fear returns.

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
- **Authoritative mechanics** (this chapter): `data/references/wiki/many-minds-move-as-one.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [Many Minds Move as One](https://spiritislandwiki.com/index.php?title=Many_Minds_Move_as_One).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data 2026-04-19. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
