# Lightning's Swift Strike

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Antistone / Jeremy Lennert's BGG openings thread 1969985](https://boardgamegeek.com/thread/1969985/openings-lightnings-swift-strike).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Low                                                |
| Play Difficulty       | 1 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn (bundled)         |
| Power summary (1–5)   | **Offense 5** · Control 2 · Fear 3 · Defense 1 · Utility 2 |
| Primary Elements      | **Fire** · **Air** (Thundering Destruction all tiers + Swiftness) · Water (L3/L4) |
| Special Rules         | Swiftness of Lightning (1 Air → 1 Slow used as Fast) |
| Aspects (JE)          | Pandemonium · Immense · Wind `[VERIFY]`            |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [Antistone thread 1969985](https://boardgamegeek.com/thread/1969985) |
```

## Spirit Overview — Framing

Lightning is a **fast-burst damage spirit** whose identity is inverted from most spirits — abundant card plays and scarce cards. Antistone:

> Lightning has a very easy time getting lots of card plays and a hard time getting lots of cards to spend them on.

**Wiki-printed playstyle note**:

> Deals damage quickly with support from allies. Air-innate amplifier for the whole team via Lightning's Boon.

**Identity in one line**: fast Town-killer on T1–T2, team amplifier via Air, card-play surplus looking for cards.

**Complexity signal**: Low is correct. 4 Uniques, 1 innate, 1 Special Rule (Air → Slow-as-Fast). Decision-load: *when to Reclaim* and *which Minor* to draft.

## Starting Setup

> Put **2 Presence** on your starting board in the **highest-numbered Sands**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                              | Best when                                                 |
|--------|------------------------------------------------------|-----------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy              | Default Reclaim turn                                      |
| **G2** | Add Presence (R2) + Add Presence (R0)                | Spread + starting-land density                            |
| **G3** | Add Presence (R1) + +3 Energy                        | Big energy spike turn                                     |

## Presence Tracks

- **Energy track** (8 slots): `energy1 → energy1 → energy2 → energy2 → energy3 → energy4 → energy4 → energy5`
- **Card-play track** (5 slots): `card2 → card3 → card4 → card5 → card6`

**Starting income**: 1 Energy, 2 Card Plays. The CP ceiling (up to 6) is huge — the problem is finding enough cards to spend them on.

## Core Mechanics & Special Rules

### Special Rule: Swiftness of Lightning

> For every Air you have, you may use 1 Slow Power as if it were Fast. (Power Cards or your Innate Powers.)

**Air-driven Slow→Fast conversion**. 1 Air = 1 Slow-as-Fast. 2 Air = 2 Slow-as-Fast. The identity-shaping rule.

### Innate: Thundering Destruction

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds                                     | Effect                              |
|-------|------------------------------------------------|-------------------------------------|
| 1     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Destroy 1 Town. |
| 2     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | You may instead destroy 1 City. |
| 3     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Also, Destroy 1 Town / City. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 5 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Also, Destroy 1 Town / City. |

Town-killer innate. L1 (3 Fire + 2 Air) reachable T1–T2. L2 upgrades to City-kill.

## Unique Cards (all 4, Wiki-verified)

### Raging Storm
- **3 Energy · Slow · Range 1 · Any Land · Fire, Air, Water**
- *1 Damage to each Invader.*

AOE. Antistone: inefficient; better as Explore-prevention than raw damage.

### Shatter Homesteads
- **2 Energy · Slow · Range 2 from Sacred Site · Any Land · Fire, Air**
- *1 Fear. Destroy 1 Town.*

The signature Town-killer. Range 2 from sacred site + Swiftness makes it a Fast Town-destroy.

### Lightning's Boon
- **1 Energy · Fast · No Range · Any Spirit · Fire, Air**
- *Target Spirit may use up to 2 Slow Powers as if they were Fast Powers this turn.*

**Team amplifier**. Gift to any Slow-heavy partner. Antistone: in solo, *"play for elements only"* — the Slow→Fast effect doubles on Lightning itself (with Swiftness native).

### Harbingers of the Lightning
- **0 Energy · Slow · Range 1 · Any Land · Fire, Air**
- *Push up to 2 Dahan. 1 Fear if you pushed any Dahan into a land with Towns/Cities.*

0-cost Fire/Air — the threshold-filler. Used every turn to stack innate elements.

## Key Strategic Principles

1. **Air = Slow-as-Fast.** Draft Air minors aggressively.
2. **Fire for Thundering Destruction.** Every turn wants Fire + Air in hand.
3. **Card-plays exceed cards.** Minor-draft over Major.
4. **Swiftness timing is not reflexive.** Antistone: if destroying a starting Town creates out-of-range lands after future Builds, make Shatter *Fast*; otherwise play Slow to hit a Town about to Ravage.
5. **Reclaim every turn is a tempo-loss** — it costs presence growth.
6. **Lightning's Boon solo = elements only.** The ally-benefit wastes.

## Possible Openings

### Shared starting state

- **2 Presence** on highest-numbered Sands.
- **4 Uniques in hand**: Raging Storm (3E Slow, Fire/Air/Water), Shatter Homesteads (2E Slow, Fire/Air), Lightning's Boon (1E Fast, Fire/Air), Harbingers of the Lightning (0E Slow, Fire/Air).
- **Starting income**: 1 Energy, 2 Card Plays.

### Opening A — Antistone's Presence-first (slow-ramp, multiplayer) 🟨 (default)

**T1 · Growth**: G2 (2 presence from top-track). Extra presence = range + 2nd sacred site.
**T1 · Play** (1E, 2 CP): **Shatter Homesteads only**. Skip 2nd card play — save for T2 innate trigger.

**T2 · Growth**: G2 (+3E from G3 misread — actually G3: R1 presence + 3E). Correction: T2 G3 for +3E + presence.
**T2 · Play** (4E, 2 CP): **Lightning's Boon + Raging Storm + Harbingers** — triggers Thundering Destruction L1 (3 Fire + 2 Air), destroys 2nd Town.

**T3 · Growth**: G1 (Reclaim + Gain Minor + 1E). Card-starved relief.

**T4 target**: sustained 3 card plays per turn with innate fires; Reclaim loops every 3 turns.

### Opening B — Ruduen's Aggressive (solo / fast adversaries) 🟨

**T1 · Growth**: G1 (+1E + Reclaim, but only empty to reclaim) — **Variant**: G3 (R1 presence + 3E) = 4E.
**T1 · Play** (4E): **Shatter Homesteads + Lightning's Boon + Harbingers** → innate L1 T1, up to 4 Fear immediately.

**T2 · Growth**: G1 Reclaim + Minor + 1E.
**T2 · Play**: Repeat Town-clear if needed.

**T3 · Growth**: G2 or G1 for 3E.
**T3 · Play**: Everything unplayed except maybe Raging Storm.

**T4 · Growth**: G1 Reclaim + Minor + 1E.

**Trade-off**: destroys fewer total Towns across T1–T3 but earlier; prevents inland explores Opening A can't. Higher-risk, higher-reward vs. aggressive adversaries.

### Opening C — Ocean-partner Variant 🟥

With Ocean's Tidal Boon (+2E gift): play 0 cards T1, G2 bottom T2, play all 4 starters T2 — but lack Air to Fast everything. Niche.

### Opening Decision

- **Default Opening A** in 2P+ multiplayer.
- **Opening B** solo or vs. fast-cliff adversaries.
- **Opening C** when paired with Ocean.

## Card Priority Ratings

### Uniques — Lightning-specific ranking

1. **Shatter Homesteads** — the Town-killer; 2E Fire/Air.
2. **Lightning's Boon** — team amplifier (or solo elements).
3. **Harbingers of the Lightning** — 0-cost Fire/Air threshold-filler.
4. **Raging Storm** — AOE; situational Explore-prevention.

### Top 10 Minor Draft Picks (Air > Fire)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Air |
| 2 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Air-feeder |
| 3 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Air |
| 4 | **Bats Scout for Raids by Darkness** | 1 | Fast | Moon, Air, Animal | Air |
| 5 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire-feeder |
| 6 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth |
| 7 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire + multi-element |
| 8 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Air-feeder |
| 9 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | Four-element flex |
| 10 | **Travel Unsuspected** | 1 | Fast | Air, Water | Air + Water (L3/L4) |

### Top 5 Major Draft Picks

Rare for Lightning — drafts only situationally. Prefer:

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Powerstorm** | 3 | Fast | Sun, Fire, Air | Fire+Air double-prime |
| 2 | **Instruments of Their Own Ruin** | 3 | Fast | Fire, Air, Animal | Fire+Air multi-land |
| 3 | **Pillar of Living Flame** | 4 | Fast | Sun, Fire, Air | Fire+Air element-wide |
| 4 | **Entwined Power** | 4 | Slow | Fire, Air | Expensive but dual-prime |
| 5 | **Bargains of Power and Protection** | 3 | Fast | Sun, Moon, Fire, Air | Four-element utility |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Defense-heavy Minors | Lightning is offense, not defense |
| Earth-only Minors | Off-axis elements |
| Single-land non-Town-destroy Majors | Shatter Homesteads already fills slot |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Town-kill pace matches Prussia ramp                          |
| England                 | A       | ★★★☆☆  | Late-stage Cities need L2+ innate                            |
| Sweden                  | A / B   | ★★★☆☆  | Build-spam fights Lightning's card economy                   |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Board A matters most (Ruduen): 2 coastal lands unreachable without presence-heavy start — use Opening A variant with top-track. Boards B/C/D let Shatter Homesteads reach all but one land, so the G3 energy-start is viable.

## Game-Phase Strategy

### Early (T1–3)
- Shatter Homesteads T1 + T2 Town-kills.
- Thundering Destruction L1 online T2.
- Card-draft Air/Fire aggressively.

### Mid (T4–6)
- Thundering Destruction L2 (4 Fire + 3 Air) for City-kills.
- Reclaim cycles every 3 turns.

### Late (T7+)
- Thundering Destruction L3/L4 for multi-Town/City destroy.
- Major integration (if drafted).

## Synergy Partners (Multiplayer)

- **Keeper of the Forbidden Wilds** — slow card economy, benefits from Lightning's Boon Fast conversion.
- **Serpent Slumbering Beneath the Island** — Gift of Flowing Power card-play acceleration.
- **Ocean** — Tidal Boon energy gift; shared coastal presence.
- **Green** — Gift of Proliferation for early presence acceleration.

## Common Mistakes

```admonish failure title="Antistone's named mistakes"
1. **Trying to play all 4 starters T1 without Reclaim planning.** Hand runs out fast.
2. **Lightning's Boon solo for stated effect.** Only take for elements.
3. **Reflexive Swiftness.** Some turns Shatter should be Slow to hit a Town *about to Ravage*.
4. **Raging Storm as a premium pick.** It's inefficient; better as Explore-prevention.
5. **Reclaim every turn.** Costs presence growth.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2/G3 + Shatter Homesteads; save plays                   |
| 2    | G3 + Boon + Raging Storm + Harbingers; Thundering L1 fires |
| 3    | G1 Reclaim + Minor; card relief                           |
| 4    | Sustained 3 plays; Thundering L2 reachable                |
| 5+   | Thundering L2/L3/L4 cycles                                |

## Source Notes

- **Mechanics**: `data/references/wiki/lightning-swift-strike.json` (Wiki-parsed 2026-04-23).
- **Openings**: [Antistone BGG 1969985](https://boardgamegeek.com/thread/1969985/openings-lightnings-swift-strike).
