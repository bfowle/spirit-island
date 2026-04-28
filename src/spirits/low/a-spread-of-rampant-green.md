# A Spread of Rampant Green

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Phantaskippy's Wiki Guide](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide) (canonical) + [BGG openings thread 1965751](https://boardgamegeek.com/thread/1965751/openings-spread-rampant-green).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Moderate                                           |
| Play Difficulty       | 2 `[VERIFY physical spirit panel]`                 |
| Growth type           | "many" — pick multiple growth options per turn     |
| Power summary (1–5)   | **Offense 4** · Control 3 · Fear 2 · **Defense 5** · Utility 4 |
| Primary Elements      | **Plant** (both innates all tiers) · **Moon** (Creepers) · **Water** (All-Enveloping) |
| Special Rules         | Choke the Land with Green (destroy presence to prevent Ravage/Build) + Steady Regeneration (destroyed Presence returns via Growth) |
| Aspects (JE)          | Tangles · Regrowth `[VERIFY]`                      |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| Phantaskippy's Guide  | [Wiki page](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide) |
```

## Spirit Overview — Framing

Green is the **multi-tasker** — built to affect 3+ lands every turn rather than clear one. Phantaskippy:

> You can frequently deal with 3 or more lands every turn… when that is done well, Green is one of the most effective spirits at contributing to a victory.

**Wiki-printed playstyle note**:

> Multi-land support via Choke the Land with Green (sacrifice presence to cancel a Ravage or Build at a sacred site) + element-greedy innates.

**Identity in one line**: blight control + multi-land juggling, not direct clearing.

**Complexity signal**: Moderate. Per-turn decision load is high (which land to choke, which Ravages to eat, which Minors to draft) but mechanics are clean.

## Starting Setup

> Put **2 Presence** on your starting board: **1 in the highest-numbered Wetland, and 1 in the Jungle without any Dahan**. (If there is more than 1 such Jungle, you may choose.)

## Growth Options (growthtype: "many" — pick multiple per turn)

| Growth | Effects                                                          | Best when                                              |
|--------|------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Spread (Green-specific bundle — place presence + other effects)  | Early-game spread                                      |
| **G2** | Reclaim + Gain 1 Power Card                                      | Reclaim cycle turn                                     |
| **G3** | Add Presence (Range 1) + Old-slot +1 Card Play                   | Spread + play-track advance                            |
| **G4** | Gain 1 Power Card + +3 Energy                                    | Energy spike + card                                    |

## Presence Tracks

- **Energy track** (7 slots): `energy0 → energy1 → plant → energy2 → energy2 → plant → energy3`
- **Card-play track** (6 slots): `card1 → card1 → card2 → card2 → card3 → card4`

**Starting income**: 0 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Choke the Land with Green

> Whenever Invaders would Ravage or Build in a land with your Sacred Site, you may prevent it by destroying one of your Presence in that land.

The signature rule: **sacrifice 1 Presence → cancel 1 Ravage or Build** at a sacred site.

### Special Rule: Steady Regeneration

> When adding Presence to the board via Growth, you may optionally use your destroyed Presence. If the island is Healthy, do so freely. If the island is Blighted, do so at an Energy cost.

Destroyed Presence recoverable at Growth. Green's Choke economy only works because of this loop.

### Innate: Creepers Tear Into Mortar

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                              | Effect                       |
|-------|-----------------------------------------|-------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Damage to 1 Town / City. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Repeat this Power. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Repeat this Power again. |

### Innate: All-Enveloping Green

- **Speed**: Fast · **Range**: 1 · **Target**: Any

| Level | Thresholds                              | Effect                                          |
|-------|-----------------------------------------|-------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Defend 2. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Instead, Defend 4. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 5 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Also, remove 1 Blight. |

## Unique Cards (all 4, Wiki-verified)

### Fields Choked with Growth
- **0 Energy · Slow · Range 1 · Any Land · Sun, Water, Plant**
- *Push 1 Town.* **OR** *Push 3 Dahan.*

0-cost push. Dahan push sets up counterattacks.

### Gift of Proliferation
- **1 Energy · Fast · No Range · Another Spirit · Moon, Plant**
- *Target Spirit adds 1 Presence up to 1 Range from their Presence.*

Ally-presence gift. Accelerates slow-ramping partners — Keeper is canonical target.

### Overgrow in a Night
- **2 Energy · Fast · Range 1 · Any Land · Moon, Plant**
- *Add 1 Presence.* **OR** *If target land has your Presence and Invaders, 3 Fear.*

Fast presence-add — combined with Choke's destroy-to-cancel, this is a cycle engine.

### Stem the Flow of Fresh Water
- **0 Energy · Slow · Range 1 from Sacred Site · Any Land · Water, Plant**
- *1 Damage to 1 Town/City. If target land is a Mountain or Sands, instead, 1 Damage to each Town/City.*

0-cost Town-damage + Mountain/Sands AOE clause.

## Key Strategic Principles

1. **Minor-first, elements-first.** Phantaskippy priority:
   1. Gift of Power / Growth Through Sacrifice (0-cost, all elements)
   2. Moon+Water (both innates)
   3. Moon+Plant (Creepers)
   4. Water+Plant (All-Enveloping)
   5. Damage adders
   6. Explorer-destroyers (weakest gap)
2. **Multi-task, don't clear.** 3+ lands per turn is the win condition.
3. **Never Choke a land that needs other Powers.** Phantaskippy: *"Never use Choke the Land on a land that will require you to then use other powers to keep it from Blighting."*
4. **Clear tracks before placing destroyed Presence.** Late-game flex only.
5. **Green + Lightning T1 combo** is the canonical "devastating start."

## Possible Openings

### Shared starting state

- **2 Presence** on starting Wetland + Jungle.
- **4 Uniques in hand**: Fields Choked with Growth (0E Slow, Sun/Water/Plant), Gift of Proliferation (1E Fast, Moon/Plant), Overgrow in a Night (2E Fast, Moon/Plant), Stem the Flow of Fresh Water (0E Slow, Water/Plant).
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — Phantaskippy's canonical 🟨 (default)

**T1 · Growth**: G2 (top + bottom + Always).
- Place 1 from top (uncover +1 Energy).
- Place 1 from bottom (progress toward 2 plays sustained).
- Plus "Always" presence.

**T1 · Play** (0E + G2 energy; 2 CP via track): **Gift of Proliferation + Stem the Flow of Fresh Water**.
- Stem + Creepers L1 = destroy a Town.

**T2 · Growth**: G3 (+3 energy, +1 Minor, +1 Presence).

**T2 · Play**: **Overgrow in a Night** (place presence; uncover free Plant → All-Enveloping L1 Defend 2), then **Fields Choked with Growth** (push 1 Town or 3 Dahan).
- Phantaskippy: *"That's 6 presence on the map before the invaders ravage."*
- Choke a Ravage if geometry demands.

**T2 variant**: If T2 Minor has Plant+Moon+Water (Gift of Power, Elemental Boon, Growth through Sacrifice), play it instead of Fields to hit **Creepers L2** — destroys a Town without combo.

**T3 · Growth**: G2 Reclaim All. Pick up everything for T4 re-deploy.

**T4 target**: 2 CP sustained, 6+ presence, comfortable with one Choke per round without panic. Major-power pivot via G4 (+3 Energy).

### Opening Decision

- **Default Opening A** — Phantaskippy canonical, works across matchups.
- **+ Lightning T1 combo**: Gift to Lightning + Stem; Lightning plays Lightning's Boon on you + Shatter Homesteads. Both back-Towns destroyed T1.

## Card Priority Ratings

### Uniques — Green-specific ranking

1. **Gift of Proliferation** — ally amplifier.
2. **Overgrow in a Night** — Fast Presence-add = Choke-cycle engine.
3. **Stem the Flow of Fresh Water** — 0-cost Town-damage.
4. **Fields Choked with Growth** — 0-cost push.

### Top 10 Minor Draft Picks (Moon+Plant, Water+Plant)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Gift of Power** | 1 | Fast | Moon | Phantaskippy: "dream minor" |
| 2 | **Growth Through Sacrifice** | 0 | Slow | Moon, Fire, Plant | "Dream minor" |
| 3 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | Four-element 0-cost |
| 4 | **Drifting Into Stillness** | 1 | Slow | Moon, Plant | Moon + Plant |
| 5 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon-feeder |
| 6 | **Absorb Essence** | 0 | Fast | Water | 0-cost Water |
| 7 | **Purify the Land** | 0 | Slow | Moon, Water, Plant | 0-cost triple-prime |
| 8 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Plant + Moon |
| 9 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Plant + Earth |
| 10 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Plant + multi-element |
| 2 | **Vigor of the Breaking Dawn** | 4 | Fast | Sun, Plant | Plant + multi-land |
| 3 | **Insatiable Hunger of the Swarm** | 3 | Fast | Animal, Plant | Plant + multi-land |
| 4 | **Dream of the Untouched Land** | 4 | Fast | Moon, Sun, Plant | Moon + Plant |
| 5 | **The Jungle Hungers** | 6 | Slow | Moon, Plant | Plant late-game closer |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Fire-heavy Minors | Off-axis |
| Pure-damage Majors | Green clears via Creepers-Repeat |
| Presence-destroyers | Compounds Choke losses |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★☆☆  | Build-pressure — Choke Builds, not Ravages                    |
| England                 | A       | ★★★☆☆  | Town/City win-conditions limit juggling                      |
| Sweden                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Green shines on maps where one "juggled" land can be safely let-go. **Land 2 (coast)** is Phantaskippy's default juggle target.

## Game-Phase Strategy

### Early (T1–3)
- G2 + G3 cadence. Overgrow + Stem + Fields.
- Creepers L1 T1 via Stem.

### Mid (T4–6)
- 2 CP sustained; Creepers L2 (Repeat) reliable.
- All-Enveloping L2 (2 Water + 4 Plant) Defend 4.

### Late (T7+)
- Creepers L3 (Repeat-again).
- All-Enveloping L3 Defend + Blight-remove.

## Synergy Partners (Multiplayer)

- **Lightning's Swift Strike** — Phantaskippy's canonical T1 combo.
- **Keeper of the Forbidden Wilds** — Gift of Proliferation accelerates Keeper's slow ramp.
- **Shadows** — push + damage combo.
- **Explorer-destroyers** (Lightning, Sharp Fangs) — cover Green's worst gap.

## Common Mistakes

```admonish failure title="Phantaskippy named mistakes"
1. **Using 2 powers on one land.** Never Choke a land that needs other Powers to prevent Blight.
2. **Sacrificing a sacred site you needed for a Slow Power.** Stack to 3 presence in trouble lands.
3. **Placing destroyed Presence while tracks still have Presence.** "Never ever." Clear tracks first.
4. **Trying to clear every land.** Green juggles.
5. **Not spreading wide.** 3 concentrated presence = 60% of real Green.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Gift + Stem; Creepers L1 destroys a Town            |
| 2    | G3 + Overgrow + Fields; 6 presence + All-Enveloping L1   |
| 3    | G2 Reclaim All; redeploy                                  |
| 4+   | 2 CP sustained; Creepers L2 Repeat; one Choke/round      |

## Source Notes

- **Mechanics**: `data/references/wiki/a-spread-of-rampant-green.json` (Wiki-parsed 2026-04-23).
- **Primary strategy**: [Phantaskippy's Wiki Guide](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide).
- **BGG openings**: [thread 1965751](https://boardgamegeek.com/thread/1965751/openings-spread-rampant-green).
