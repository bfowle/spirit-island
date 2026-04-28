# Ocean's Hungry Grasp

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Antistone BGG openings thread 1970506](https://boardgamegeek.com/thread/1970506/openings-oceans-hungry-grasp).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | High                                               |
| Play Difficulty       | 3 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn (bundled)         |
| Power summary (1–5)   | **Offense 5** · Control 4 · Fear 4 · Defense 3 · Utility 2 |
| Primary Elements      | **Water** (both innates) · **Earth** (Ocean Breaks the Shore) · Moon (Pound Ships) · Air (Pound Ships L3) |
| Special Rules         | Ocean in Play (inland-forbidden; oceans = Coastal Wetlands on your boards; Drown pieces) + Drowning (exchange Invader HP for Energy) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [Antistone thread 1970506](https://boardgamegeek.com/thread/1970506) |
```

## Spirit Overview — Framing

Ocean is the **coastal-only drown machine** — presence lives in oceans + coastal lands, and every Invader pushed into your ocean converts to Energy via the Drowning rule. Ocean's special rules restructure the board for your spirit: on boards where you have Presence, oceans are treated as Coastal Wetlands.

**Wiki-printed playstyle note**:

> Inland-forbidden spirit with ocean-based presence. Drown Invaders for Energy; multi-board spread via shared oceans.

**Identity in one line**: coast-locker, drown-fueler, multi-board presence via shared oceans in 3–4p.

**Complexity signal**: High is correct. Inland-forbidden is a hard constraint; multi-board spread via oceans is a mid-game pivot that needs rehearsal; Drowning-to-Energy conversion is its own sub-economy.

## Starting Setup

> Put **2 Presence** onto your starting board: **1 in the Ocean, and 1 in a Coastal land of your choice**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                          | Best when                                                      |
|--------|------------------------------------------------------------------|----------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + Ocean-Gather (forced 1 presence into each ocean) | Default Reclaim turn                                      |
| **G2** | Ocean-add × 2 + +1 Energy                                        | Spread into 2 oceans; mid-game + energy                       |
| **G3** | Gain 1 Power Card + Ocean-add + Ocean-push (multi-board spread)  | Card + 4th-board push                                          |

## Presence Tracks

- **Energy track** (7 slots): `energy0 → moon → water → energy1 → earth → water → energy2`
  - 0E → +Moon marker → +Water marker → 1E → +Earth marker → +Water marker → 2E
- **Card-play track** (6 slots): `card1 → card2 → card2 → card3 → card4 → card5`
  - 1 CP → 2 CP → 2 CP → 3 CP → 4 CP → 5 CP

**Starting income**: 0 Energy, 1 Card Play. Ocean is deeply energy-starved early — the Drowning-to-Energy conversion is how you stay afloat.

## Core Mechanics & Special Rules

### Special Rule: Ocean in Play

> You may add/move Presence into Oceans, but may not add/move Presence into Inland lands. On boards where you have 1 or more Presence, Oceans are treated as Coastal Wetlands for Spirit Powers and Blight. You Drown any Invaders or Dahan moved to those Oceans.

Hard placement constraint + ocean-as-Wetland clause.

### Special Rule: Drowning

> Destroy Drowned pieces, placing Drowned Invaders here. At any time you may exchange (X) Health of these Invaders for 1 Energy. (1 Explorer = 1 HP; 1 Town = 2 HP; 1 City = 3 HP.)

Invaders pushed into your oceans → Drowned pile → exchange HP for Energy at any time. Partner-pushed Invaders count (tell allies!).

### Innate: Pound Ships to Splinters

- **Speed**: Fast · **Range**: 0 · **Target**: Coastal

| Level | Thresholds                              | Effect                |
|-------|-----------------------------------------|------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Fear. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | +1 Fear. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | +2 Fear. |

### Innate: Ocean Breaks the Shore

- **Speed**: Slow · **Range**: 0 · **Target**: Coastal

| Level | Thresholds                              | Effect                |
|-------|-----------------------------------------|------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Drown 1 Town. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | You may instead Drown 1 City. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Also, Drown 1 Town / City. |

L1 requires a **coastal Town** in range 0 — only your starting coastal land. Adversary setups (Prussia, France 3, England) provide one; Sweden does not.

## Unique Cards (all 4, Wiki-verified)

### Call of the Deeps
- **0 Energy · Fast · Range 0 · Coastal Land · Moon, Air, Water**
- *Gather 1 Explorer. If target land is the Ocean, you may Gather another Explorer.*

0-cost Fast Explorer-drain into your ocean. Triggers Pound Ships L1 + Drowning-Energy gain.

### Grasping Tide
- **1 Energy · Fast · Range 1 · Coastal Land · Moon, Water**
- *2 Fear. Defend 4.*

Fast Defend with Fear. Moon + Water — element-primes Pound Ships.

### Swallow the Land-Dwellers
- **0 Energy · Slow · Range 0 · Coastal Land · Water, Earth**
- *Drown 1 Explorer, 1 Town, and 1 Dahan.*

0-cost triple-Drown. Water + Earth primes Ocean Breaks the Shore L1. **Watch out**: Drowns your own Dahan too.

### Tidal Boon
- **1 Energy · Slow · No Range · Another Spirit · Moon, Water, Earth**
- *Target Spirit gains 2 Energy and may Push 1 Town and up to 2 Dahan from one of their lands. If Dahan are pushed to your Ocean, you may move them to any Coastal land instead of Drowning.*

Ally gift: +2 Energy + Push-for-partner + Dahan redirect (avoid Drowning own Dahan). **The signature Ocean ally card.**

## Key Strategic Principles

1. **Water + Earth are load-bearing.** Both innates key off them.
2. **Drowning = energy economy.** Every pushed Invader becomes fuel.
3. **Tell allies to push Invaders into your oceans.** Free Energy + free Drowning.
4. **Multi-board via oceans in 3–4p.** Use G3 to spread presence across boards.
5. **Ocean Breaks the Shore L1 is a T1 target** if adversary provides coastal Town.
6. **Sweden is the worst matchup** — no coastal Town at most levels.
7. **Swallow the Land-Dwellers Drowns Dahan.** Don't play on Dahan-present lands.

## Possible Openings

### Shared starting state

- **2 Presence** — 1 Ocean, 1 Coastal land.
- **4 Uniques in hand**: Call of the Deeps (0E Fast, Moon/Air/Water), Grasping Tide (1E Fast, Moon/Water), Swallow the Land-Dwellers (0E Slow, Water/Earth), Tidal Boon (1E Slow, Moon/Water/Earth).
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — Antistone's Standard 🟨 (default)

**T1 · Growth**: G2 bottom (2 presence in oceans, +1 Energy).
**T1 · Play** (1E, 1 CP): **Call of the Deeps + Tidal Boon**.
- Triggers Pound Ships L1 (1 Moon + 1 Air + 2 Water) + Ocean Breaks the Shore L1 (2 Water + 1 Earth) if coastal Town present.
- *Pause-point*: Ocean Breaks the Shore is Range 0 — only targetable in your starting coastal land. Requires a coastal Town there. If Sweden or similar: skip.

**T2 · Growth**: Push presence from ocean into a coastal land; place 1 bottom; gain Minor.
**T2 · Play**: **Grasping Tide + Swallow the Land-Dwellers + new Minor**.
- In 4P, push into coastal land adjacent to 4th board, then *place* onto 4th board — presence on all boards.

**T3 · Growth**: G1 Reclaim (forced Ocean-Gather into each ocean where possible).
**T3 · Play**: New hand from Reclaim.

**T4 target**: Settled into the high-tide/low-tide cycle; Reclaim cycles with ocean-gather; presence on multiple boards; innate triggers sustained.

### Opening B — Growth-stacking (497328) 🟥

G2/G2/G3 sequence — energy-energy, plays-plays, plays. Makes elements easier (free moon + water) + realistic 4-fear Pound Ships L3. But T1–T3 card-play economy is awkward (Boon alone T1 → Call+Tide T2 → Swallow+Minor T3). Best in 4p where you want presence on all boards fast.

### Opening Decision

- **Default Opening A** vs. Prussia, France 3, England (coastal-Town adversaries).
- **Opening B** in 4P when spread matters more than T1 Drown.
- **Don't pick Ocean** vs. Sweden at most levels.

## Card Priority Ratings

### Uniques — Ocean-specific ranking

1. **Tidal Boon** — ally-amp + Dahan-redirect.
2. **Call of the Deeps** — 0-cost Fast Drown-fuel.
3. **Swallow the Land-Dwellers** — triple-Drown 0-cost.
4. **Grasping Tide** — Fast Defend + Fear.

### Top 10 Minor Draft Picks (Water > Earth > Moon)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Absorb Essence** | 0 | Fast | Water | 0-cost Water |
| 2 | **Call to Isolation** | 0 | Slow | Water, Animal | 0-cost Water |
| 3 | **Gift of Proliferation** | 1 | Fast | Plant, Water | Water + utility |
| 4 | **Purify the Land** | 0 | Slow | Moon, Water, Plant | 0-cost Water + Blight |
| 5 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Water + Moon |
| 6 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |
| 7 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth |
| 8 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Moon |
| 9 | **Travel Unsuspected** | 1 | Fast | Air, Water | Water + Air |
| 10 | **Ravaged Undergrowth Slithers Back** | 0 | Slow | Water, Earth, Plant | 0-cost Water + Earth |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Water + Earth triple-prime |
| 2 | **Dissolve Into Mist** | 4 | Fast | Air, Water | Water-feeder |
| 3 | **Manifest Incarnation** | 3 | Fast | Moon, Plant, Animal, Water | Water + utility |
| 4 | **Flow Like Water, Reach Like Air** | 2 | Fast | Sun, Air, Water | Water + cheap |
| 5 | **Cast Down into the Briny Deep** | 5 | Slow | Sun, Moon, Water | Water + Moon |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Inland-only effects | Placement constraint |
| Pure-Animal Minors | Off-axis |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Extra coastal Town land 3 — perfect T1 Ocean Breaks setup     |
| England                 | A       | ★★★☆☆  | Coastal Cities scale with Ocean's drown path                 |
| France-Plantation       | A       | ★★★☆☆  | Coastal Town at L3+                                           |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]` — L1 Trading Port errata interaction              |
| Russia                  | A       | ★★☆☆☆  | `[VERIFY]`                                                   |
| **Sweden**              | —       | ★☆☆☆☆  | **Worst matchup** — no coastal Town at most levels; avoid    |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Board D bonus: if Wetlands get explored during setup, Call of the Deeps can Gather **2** Explorers into the ocean T1 instead of 1.

Otherwise board-selection priority: boards with **coastal Towns at setup** + adjacent-to-partner-board oceans (for multi-board spread in 3–4p).

## Game-Phase Strategy

### Early (T1–3)
- Call of the Deeps every turn; drown Explorers.
- Ocean Breaks the Shore L1 on coastal Towns.
- Tidal Boon gift to partner for energy.

### Mid (T4–6)
- Multi-board spread via G3 ocean-push.
- Drowning-Energy conversion funds Majors.
- Pound Ships L2 reliable.

### Late (T7+)
- Ocean Breaks the Shore L2/L3 — City drown.
- Pound Ships L3 — big Fear per turn.
- Tsunami / Cast Down Majors.

## Synergy Partners (Multiplayer)

- **Push-based partners** — Thunderspeaker, Lightning (Harbingers Push), Keeper (Spreading Wilds Push) — funnel Invaders into your oceans.
- **Tidal Boon targets** — Lightning, Major-drafters need the Energy gift.
- **Coastal-adjacent boards** — board select for shared ocean access.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Forgetting Ocean Breaks the Shore is Range 0.** Common T1 trap — no coastal Town in setup = no target.
2. **Pushing presence before placing in 4P Growth.** Steps are ordered.
3. **Swallow the Land-Dwellers on Dahan lands.** Drowns your own.
4. **Not telling allies** — moving Invaders into your oceans = free Energy.
5. **Reclaiming without planning forced ocean-gather.** Can strand presence.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Call + Tidal Boon; both innates L1 fire               |
| 2    | G2/G3 + Grasping Tide + Swallow + Minor; multi-board spread |
| 3    | G1 Reclaim; Ocean-Gather mandatory                        |
| 4+   | High-tide/low-tide cycle; Drowning-Energy feeds Majors     |

## Source Notes

- **Mechanics**: `data/references/wiki/ocean-hungry-grasp.json` (Wiki-parsed 2026-04-23).
- **Openings**: [Antistone BGG 1970506](https://boardgamegeek.com/thread/1970506/openings-oceans-hungry-grasp).
