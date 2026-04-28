# Keeper of the Forbidden Wilds

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Jeremy Lennert's BGG openings thread 1978655](https://boardgamegeek.com/thread/1978655/openings-keeper-forbidden-wilds).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch & Claw                                      |
| Complexity            | Moderate                                           |
| Play Difficulty       | 3 `[VERIFY physical spirit panel]`                 |
| Growth type           | "many" — pick multiple growths per turn            |
| Power summary (1–5)   | **Offense 5** · Control 2 · Fear 1 · **Defense 4** · Utility 3 |
| Primary Elements      | **Sun** (both innates) · **Plant** (both innates) · Fire (Punish L1/L2) · Air (Spreading Wilds range) |
| Special Rules         | Forbidden Ground (Sacred Sites Push Dahan; Dahan Events don't move Dahan to Sacred Sites) |
| Aspects (JE)          | Spreading Hostility `[VERIFY]`                     |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [Lennert thread 1978655](https://boardgamegeek.com/thread/1978655) |
```

## Spirit Overview — Framing

Keeper is a **slow-starting, cluster-building sacred-site spirit** — falls behind in Stage 1, catches up in Stage 2 when its Wilds tokens start choking Explores. Lennert:

> Keeper tends to fall behind the invaders in the early game and then catch up in stage 2 when all those wilds you've been placing start blocking explores.

**Wiki-printed playstyle note**:

> Wilds-placing, range-limited damage spirit. Punish Those Who Trespass is a zero-range damage innate; Spreading Wilds converts land-type into defensive terrain.

**Non-obvious constraint**: **No way to place presence into blighted land.** All three add-presence modes require no-blight, existing wilds, or existing presence. Shapes opening play more than damage numbers do.

**Solo-vs-multi inversion on Towering Wrath**: Wrath scales with sacred-site clustering, which fights against multi-board coverage needs. Community consensus: Wrath is phenomenal solo, weaker at 3–4p.

**Complexity signal**: Moderate is correct.

## Starting Setup

> Put **1 Presence and 1 Wilds** on your starting board in the **highest-numbered Jungle**.

## Growth Options (growthtype: "many" — pick multiple)

| Growth | Effects                                                          | Best when                                              |
|--------|------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + +1 Energy                                              | Reclaim cycle + energy bank                            |
| **G2** | Gain 1 Power Card                                                | Card-gain only                                         |
| **G3** | Keeper-add (Presence) + +1 Energy                                | Spread + energy                                        |
| **G4** | Keeper3 (3-presence bundle) + no-Blight constraint               | Big spread into safe lands                             |

## Presence Tracks

- **Energy track** (8 slots): `energy2 → sun → energy4 → energy5 → plant → energy7 → energy8 → energy9`
- **Card-play track** (6 slots): `card1 → card2 → card2 → card3 → card4 → card5reclaim1`

**Starting income**: 2 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Forbidden Ground

> After you create a Sacred Site, Push all Dahan from that land. Dahan Events never move Dahan to your Sacred Site, but Powers can do so.

Sacred sites repel Dahan. Keeper's lands are *forbidden* to Dahan — makes Towering Wrath's Destroy-all-Dahan clause mostly a non-event.

### Innate: Punish Those Who Trespass

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                                       | Effect                           |
|-------|--------------------------------------------------|-----------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 2 Damage. Destroy 1 Dahan. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | +1 Damage per Sun/Plant you have. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Split this Power's Damage between target land and another of your lands. |

Range-0 damage scales with Sun+Plant. L2 at 2S/2F/3P can reach 4–6 damage.

### Innate: Spreading Wilds

- **Speed**: Slow · **Range**: 1 · **Target**: No-Blight

| Level | Thresholds                                       | Effect                                 |
|-------|--------------------------------------------------|-----------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun | Push 1 Explorer per 2 Sun you have. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | If target land has no Explorer, add 1 Wilds. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | This Power has Range +1. |
| 4     | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power has Range +1. |

Explorer-push + Wilds-seeding in empty lands. Wilds cancels Explores (see [Token Economies](../../fundamentals/token-economies.md)).

## Unique Cards (all 4, Wiki-verified)

### Boon of Growing Power
- **1 Energy · Slow · No Range · Any Spirit · Sun, Moon, Plant**
- *Target Spirit gains a Power Card. If you target another Spirit, they also gain 1 Energy.*

Card-gain + partner amp. Self-target fine in solo.

### Regrow from Roots
- **1 Energy · Slow · Range 1 · Jungle or Wetland · Water, Earth, Plant**
- *If there are 2 Blight or fewer in target land, Remove 1 Blight.*

Blight-remove with a cap — won't fix cascaded lands but handles 1–2 Blight.

### Sacrosanct Wilderness
- **2 Energy · Fast · Range 1 · Land with no Blight · Sun, Earth, Plant**
- *Push 2 Dahan. 2 Damage per Wilds in target land.* **OR** *Add 1 Wilds.*

Dual-mode: Wilds-scaled damage (in already-wilded lands) or Wilds-seeder.

### Towering Wrath
- **3 Energy · Slow · Range 1 from Sacred Site · Any Land · Sun, Fire, Plant**
- *2 Fear. For each of your Sacred Site in/adjacent to target land, 2 Damage. Destroy all Dahan.*

Cluster-scaled damage. 3 sacred sites adjacent = 6 damage + 2 Fear. The "Destroy all Dahan" clause is mostly irrelevant (Forbidden Ground repels Dahan anyway).

## Key Strategic Principles

1. **Cluster sacred sites.** Every additional site adjacent = +2 Wrath damage.
2. **Wilds tokens compound across turns.** T1 Wilds = T4 Explore-cancel.
3. **Draft range-extenders.** Zero-range Punish desperately wants Reaching Grasp, Sky Stretches to Shore.
4. **Keep 2+ energy banked T1–T2** — the +1 E / +presence growth is conditional.
5. **Solo vs multi inverts Wrath priority.** Solo → cluster. 3–4p → spread.
6. **Strangling Firevine is the Major-draft crown jewel** (Lennert).

## Possible Openings

### Shared starting state

- **1 Presence + 1 Wilds** on highest Jungle.
- **4 Uniques in hand**: Boon of Growing Power (1E Slow, Sun/Moon/Plant), Regrow from Roots (1E Slow, Water/Earth/Plant), Sacrosanct Wilderness (2E Fast, Sun/Earth/Plant), Towering Wrath (3E Slow, Sun/Fire/Plant).
- **Starting income**: 2 Energy, 1 Card Play.

### Opening A — Lennert canonical 🟨 (default)

Saves 2E per early turn; reclaim on T4.

**T1 · Growth**: G1 (Minor) + G2 (+1E, place presence). Presence in starting land.
- Target elements: 2 Sun + 1 Plant; save 2E. If 0-cost Sun Minor drops, place from bottom track; play Boon of Growing Power + that Minor.
- Otherwise: place from top track (expose Sun); play only Boon.
- Hits Spreading Wilds L1 + L2 (2 Sun + 1 Plant); clear lone-Explorer land, seed Wilds.

**T2 · Growth**: G1 (+1E, presence) + G3 (pay 3E, presence, Minor).
- Income: 4E / 2 plays.
- Target: 2 Sun + 1 Plant again; save 2E. Legal combos:
  - **Sacrosanct Wilderness + any 0-cost Minor**, OR
  - **Regrow from Roots + any Sun Minor**.

**T3 · Growth**: G1 + G3 again (both bottom-track).
- Cards: **Towering Wrath + Plant/Fire Minor + Plant Minor (e.g., Regrow from Roots)**.
- Hits Punish Those Who Trespass L2 (4 damage at Range 0, 5–6 with planning) + Spreading Wilds L3 (+1 Range).
- Energy tight — Wrath costs 3 of your 4E income; one companion must be 0-cost.

**T4 · Growth**: Reclaim. Repeat roughly T3 shape.

### Opening B — T2 Wrath (damage-rush) 🟥

Play Wrath T2 for earlier burst. Lennert: typically a damage loss (fewer sites; only L1 Punish) and slows development — T3 can only place 1 presence.

### Opening C — brunoxv's "save 3E" 🟥

Save 3 rather than 2 on early turns so 3E growth option converts an unblighted presence into a sacred site same turn. Pushes Reclaim to T5.

### Opening D — Major-rush (497328 variant) 🟥

**T3 · Growth**: G3 + Major (5+ Plant); forget Towering Wrath if better Major dropped.

The Jungle Hungers and Strangling Firevine are flagged as better 3-energy effects in games where site-clustering isn't viable.

### Opening Decision

- **Default Opening A** — Lennert's canonical balanced line.
- **Opening B/C** in niche acceleration contexts.
- **Opening D** in 3–4p where cluster-scaling loses value.

## Card Priority Ratings

### Uniques — Keeper-specific ranking

1. **Towering Wrath** — site-cluster-scaled damage.
2. **Sacrosanct Wilderness** — Wilds-scaled Fast damage.
3. **Boon of Growing Power** — card-gain + partner amp.
4. **Regrow from Roots** — situational Blight-cleanse.

### Top 10 Minor Draft Picks (Sun + Plant + Fire)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Reaching Grasp** | 1 | Fast | Sun, Water, Plant | Range-extender for Punish |
| 2 | **Sky Stretches to Shore** (if Major, skip — else sub) | — | — | — | — |
| 3 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun + Plant |
| 4 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost double-prime |
| 5 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost triple-prime |
| 6 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Sun + Plant |
| 7 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Plant + Moon |
| 8 | **Gift of Power** | 1 | Fast | Moon | Utility |
| 9 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire-feeder |
| 10 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Plant |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Strangling Firevine** | 3 | Slow | Fire, Plant | Lennert: *"amazing for Keeper"* |
| 2 | **The Jungle Hungers** | 6 | Slow | Moon, Plant | Plant + multi-land |
| 3 | **Sky Stretches to Shore** | 2 | Fast | Sun, Moon, Air | Range-extender |
| 4 | **Reaching Grasp** | — | — | — | (if Major pool has Grasp — check) |
| 5 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Sun + Plant multi-land |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Single-land non-Plant Majors | Off-axis |
| Presence-destroyers | Keeper is presence-thin |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★☆☆  | Wilds-cancel Explores; Wrath clears mid-game                 |
| England                 | A       | ★★★☆☆  | Coastal Cities bypass Jungle-start range                     |
| Sweden                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

| Board / Start | Rating | Reason |
|---------------|--------|--------|
| **Board B + Sands first-explore** | ★☆☆☆☆ | *"Keeper nightmare"* (Stixidor) — no starting card solves a Sands; Sands at Range 2+ |
| Others | ★★★☆☆ | Workable |

Antistone: Across 12 draws, ~98.8% chance of drawing a useful land-clear. New blight rules mitigate the Board B trap further.

## Game-Phase Strategy

### Early (T1–3)
- Save 2E per turn; cluster sacred sites.
- Seed Wilds T1–T3 for Stage-2 Explore cancel.
- T3 Towering Wrath for first burst.

### Mid (T4–6)
- Reclaim cycle every 3 turns.
- Punish Those Who Trespass L2 for 4–6 damage.
- Major integration (Strangling Firevine).

### Late (T7+)
- Wrath + Wilds cycle with 4+ sacred sites adjacent.
- Punish L3 (4 Plant) for split-damage.

## Synergy Partners (Multiplayer)

- **A Spread of Rampant Green** — Gift of Proliferation jumps Keeper to 5E T2 / 7E T3.
- **Early-tempo partners** (Sharp Fangs, Lightning) — pick up the slack T1–T2 (brunoxv).
- **Avoid**: other slow spirits without external acceleration.

## Common Mistakes

```admonish failure title="Lennert named mistakes"
1. **Placing both presence from top track T1 without a plan.** Stalls economy.
2. **Playing Towering Wrath T2 with no sacred-site cluster yet.**
3. **3–4p cluster-focused + zero late-game reach to other boards** (nobody82b).
4. **Ignoring range-extenders in the Minor draft.**
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G1+G2 + Boon; Spreading Wilds L1/L2; seed Wilds           |
| 2    | G1+G3 + Sacrosanct / Regrow + Minor; 4E income             |
| 3    | G1+G3 + Wrath + Minors; Punish L2 hits 4–6 damage          |
| 4    | Reclaim; reset                                             |
| 5+   | Wrath-every-Reclaim cycle; Major integration               |

## Source Notes

- **Mechanics**: `data/references/wiki/keeper-of-the-forbidden-wilds.json` (Wiki-parsed 2026-04-23).
- **Openings**: [Lennert BGG 1978655](https://boardgamegeek.com/thread/1978655/openings-keeper-forbidden-wilds).
