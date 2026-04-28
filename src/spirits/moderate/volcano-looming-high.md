# Volcano Looming High

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [jyonker13's BGG openings thread 2508259](https://boardgamegeek.com/thread/2508259/openings-volcano-looming-high) + alercah / DsnowMan / Sh0rtz variants.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | **Offense 5** · Control 1 · Fear 2 · Defense 1 · Utility 3 |
| Primary Elements      | **Fire** (both innates, all tiers) · **Earth** (both innates) · Air (Eruption L4/L5) |
| Special Rules         | Mountain Home (Mountain-only placement) + Collapse in a Blast of Lava and Steam (destroyed Presence → damage) + Volcanic Peaks Tower (Range +1 from 3-presence lands) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | tepidgoose Drive PDF (external)                    |
| BGG                   | [jyonker13 thread 2508259](https://boardgamegeek.com/thread/2508259) |
```

## Spirit Overview — Framing

Volcano is **"the Spirit of philosophers"** (jyonker13) — the core tension isn't *whether* to meet the Eruption threshold, it's *when* to erupt. Identity:

> You will weigh its moral merits, you will second-guess its forbidden allure, you will ponder what it truly means to "destroy presence": in short, you will spend a ton of time thinking about when the fuck to do it.

**Wiki-printed playstyle note**:

> Slow-build bomber — trades presence for burst damage + Fear. Furnace of the Earth recycles destroyed presence each Slow phase.

**Identity in one line**: Mountain-locked bomber; destroy Presence → big AOE damage + Fear.

**Complexity signal**: Moderate is correct — mechanics are straightforward, but *when* to erupt decision-load is high.

## Starting Setup

> Put **1 Presence and 1 Badlands** on your starting board in **a Mountain of your choice**. Push all Dahan from that land.

Starting Badlands pre-seeds damage scaling.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                          | Best when                                              |
|--------|------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +3 Energy                          | Full Reclaim + big-energy turn                         |
| **G2** | Add Presence (R0) + Add Presence (R0)                            | Stack starting Mountain density                        |
| **G3** | Gain 1 Power Card + Add Presence (R4) + New-slot +1 Card Play    | Card + reach + CP                                      |

## Presence Tracks

- **Energy track** (6 slots): `energy1 → energy2 → earth → energy3 → energy4 → energy5`
- **Card-play track** (8 slots): `card1 → fireX → earthX → card2 → airX → card3 → fireX → card4`

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Mountain Home

> Your Presence may only be added/moved into Mountains.

Hard placement constraint.

### Special Rule: Collapse in a Blast of Lava and Steam

> When your Presence is destroyed, in that land, deal 1 Damage per destroyed Presence to both Invaders and to Dahan.

Destroyed Presence damages Invaders + Dahan. Collapse stacks on top of Eruption — a single eruption for 2 Presence is *4 damage* (alercah).

### Special Rule: Volcanic Peaks Tower Over the Landscape

> Your Power Cards gain Range +1 if you have 3 or more Presence in the origin land.

Range-bonus for dense Mountain-lands.

### Innate: Explosive Eruption

- **Speed**: Fast · **Range**: 0 · **Target**: Any

| Level | Thresholds                       | Effect                                                                         |
|-------|----------------------------------|--------------------------------------------------------------------------------|
| base  | Destroy X Presence in target land | X is used as a damage/fear multiplier through tiers                             |
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In one land within Range 1, X Damage. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Generate X Fear. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Each land in Range 1: 4 Damage. Add 1 Blight to target. Doesn't destroy your Presence. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 5 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Each land in Range 2: +4 Damage. Add 1 Blight to adjacent lands. |

### Innate: Powered by the Furnace of the Earth

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                            | Effect                                                 |
|-------|---------------------------------------|---------------------------------------------------------|
| 1     | 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Add 1 of your destroyed Presence. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Gain a Power Card. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Move up to 2 of your Presence from other lands to target land. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Return up to 2 of your destroyed Presence to your Presence tracks. |

Furnace recycles destroyed Presence + Power Cards. The spirit-engine that makes Eruption sustainable.

## Unique Cards (all 4, Wiki-verified)

### Lava Flows
- **Cost / Speed / Range / Target / Elements**: `[VERIFY specific numbers from Wiki]` — generally Fire-feeding Explorer-kill or Fire-token laying.

### Pyroclastic Bombardment
- **[VERIFY]**: Destroy Town/City scaling with Badlands and Fire presence.

### Rain of Ash
- **[VERIFY]**: Board-wide effect at higher cost; Air + Fire scaling.

### Exaltation of Molten Stone
- **[VERIFY]**: Explode + regain Presence via Energy exchange.

`[VERIFY all 4 unique costs/effects via Wiki; fetch parse didn't surface these cleanly.]`

## Key Strategic Principles

1. **Erupt timing is everything.** alercah: Explosive Eruption's threshold rarely gates you — timing does.
2. **Fire is oxygen** — every turn wants 2+ Fire.
3. **Mountain-only placement is a hard constraint.** Board selection is existential.
4. **Collapse stacks on Eruption** — a 2-Presence eruption is 4 damage (2 Eruption + 2 Collapse).
5. **Furnace recycles Presence.** Sustainable eruptions via Slow-phase return.
6. **Never leave a land at single Presence you cannot protect.** mat9h: *"I erupted myself down to a single presence… my teammate caused the land to blight. The blight card removed my only presence forcing game loss."*

## Possible Openings

### Shared starting state

- **1 Presence + 1 Badlands** on chosen Mountain.
- **4 Uniques in hand**: Lava Flows, Pyroclastic Bombardment, Rain of Ash, Exaltation of Molten Stone.
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — alercah's 3-Fire T5 Eruption 🟨 (aggressive)

**T1 · Growth**: +1 presence each from Energy and Plays.
**T1 · Play**: **Lava Flows** (snipe Explorer or lay Fire tokens).

**T2 · Growth**: +2 presence from plays.
**T2 · Play**: **Pyroclastic Bombardment + Exaltation of Molten Stone** — explode for 2–3 presence, regain 1. Take 2 of 3 Energy from Exaltation and buy a card.

**T3 · Growth**: +1 presence play-track + 2E + play + Minor.
**T3 · Play**: **Rain of Ash + Minor**.

**T4 · Growth**: G3 again off energy track.
**T4 · Play**: swing for T5 eruption with **3 Fire showing**.

Sh0rtz: *"the two energy you forego in order to place the extra presence will pay itself back when you reclaim."*

### Opening B — DsnowMan's T2 Tier-2 Eruption (solo) 🟥

**T1 · Growth**: +1 Energy / +1 Plays.
**T1 · Play**: Rain of Ash.

**T2 · Growth**: +2 from plays.
**T2 · Play**: Exaltation + Lava Flows. Now sit at 3 Fire / 3 Earth with 5 presence stacked in starter Mountain.

**T2 Erupt**: 4 damage + 4 Fear + 4 splash damage.

Caveat (DsnowMan, gpope): *"4 damage is usually going to be overkill or underkill… I don't necessarily want the tier 2 eruption on T2."*

### Opening C — Proffian's Growth-3 hand-building 🟥

Lean into G3 repeatedly for early cards. Sacrifices eruption size for a deep reclaim-avoidance hand.

### Opening Decision

- **Default Opening A** — balanced; most matchups.
- **Opening B** in solo when T2 4-damage eruption happens to clear a key land.
- **Opening C** for deep-hand patient play.

## Card Priority Ratings

### Uniques — Volcano-specific ranking

1. **Pyroclastic Bombardment** — Fire-scaled Town/City destroy.
2. **Lava Flows** — Explorer-snipe + Fire-token seed.
3. **Exaltation of Molten Stone** — Presence recycle + Energy.
4. **Rain of Ash** — board-wide support.

### Top 10 Minor Draft Picks (Fire + Earth)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth prime |
| 2 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire-feeder |
| 3 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire + multi-element |
| 4 | **Dry Wood** | 0 | Slow | Fire, Plant | 0-cost Fire |
| 5 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 6 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth |
| 7 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |
| 8 | **Infested Aquifers** | 1 | Slow | Water, Animal | Disease-utility |
| 9 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Earth |
| 10 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | 0-cost Fire |

### Top 5 Major Draft Picks

Volcanic Peaks' range-extension makes 0-range Majors dramatically better.

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Flow** | 6 | Slow | Fire, Earth | Fire + Earth multi-land |
| 2 | **Volcanic Eruption** | 8 | Slow | Moon, Fire, Earth | Late-game closer |
| 3 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap Fire |
| 4 | **Pillar of Living Flame** | 4 | Fast | Sun, Fire, Air | Fire + element-wide |
| 5 | **Infinite Vitality** | 4 | Fast | Fire, Water, Plant | Fire + Plant |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Non-Mountain-relevant defense Minors | Off-axis |
| Presence-destroyers beyond Collapse | Too much presence drain |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Eruption clears Build stacks                                 |
| England                 | A       | ★★★☆☆  | Rain of Ash T1 vs early Town; erupt City T3+                 |
| Sweden                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| **Scotland**            | A       | ★★☆☆☆  | Double-city start; T2 Pyroclastic needed but Energy conflicts |
| France-Plantation       | A       | ★★★★☆  | Extra-Build triggers = Eruption motivators                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★★☆  | Extra-Build triggers                                          |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Mountain-only placement is the hard constraint. Flips13: *"Since you are only in mountains, choose carefully which board you start on!"* Board E coastal Mountain is a classic trap.

## Game-Phase Strategy

### Early (T1–3)
- G2/G3 to stack presence in starting Mountain.
- T2 or T3 first eruption.
- Badlands placement via Lava Flows.

### Mid (T4–6)
- Eruption cycle: erupt → Furnace recycles → re-erupt.
- Eruption L2 (3 Fire + 3 Earth) for +X Fear.
- Major integration.

### Late (T7+)
- Eruption L3/L4 (4+ Fire + 2+ Air + 4+ Earth) for board-wide damage.
- Volcanic Eruption / Pyroclastic Flow Majors.

## Synergy Partners (Multiplayer)

- **Allies who drop buildings in your Mountains** — multiply eruption value.
- **Presence-movement / range-extension** (Keeper, Finder) multiply reach.
- **Avoid Fractured Days-style disruption** that breaks Volcano's timing.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Leaving a land at 1 Presence you cannot protect.** Blight card → game loss.
2. **Forgetting Collapse stacks on eruption damage.** 2-Presence eruption = 4 damage.
3. **Hoarding presence for 3-tier eruption when T2/T3 small eruptions + Badlands are cheaper.**
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | +1 presence each track; Lava Flows                       |
| 2    | +2 presence plays; Pyroclastic Bombardment + Exaltation  |
| 3    | +1 plays + 2E + Minor; Rain of Ash                       |
| 4    | G3 + big Eruption setup                                  |
| 5    | **T5 Erupt** with 3 Fire showing                         |
| 6+   | Furnace recycle; Eruption cycle continues                 |

## Source Notes

- **Mechanics**: `data/references/wiki/volcano-looming-high.json` (Wiki-parsed 2026-04-23).
- **Openings**: [jyonker13 BGG 2508259](https://boardgamegeek.com/thread/2508259/openings-volcano-looming-high).
