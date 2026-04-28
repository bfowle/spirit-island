# River Surges in Sunlight

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus River Opening 1](https://latentoctopus.github.io/guide/river-opening1/) + [Jeremy Lennert's BGG openings thread 1967085](https://boardgamegeek.com/thread/1967085/openings-river-surges-sunlight).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Low                                                |
| Play Difficulty       | 0 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn (bundled)         |
| Power summary (1–5)   | **Offense 4** · **Control 5** · Fear 1 · Defense 1 · **Utility 4** |
| Primary Elements      | **Water** (all Massive Flooding tiers + special rule) · **Sun** (all tiers) · Earth (L3) |
| Special Rules         | River's Domain (Presence in Wetlands count as Sacred Site) |
| Aspects (JE)          | Sunshine · Travel `[VERIFY]`                       |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Opening 1](https://latentoctopus.github.io/guide/river-opening1/) |
| BGG                   | [Jeremy Lennert thread 1967085](https://boardgamegeek.com/thread/1967085) |
```

## Spirit Overview — Framing

River is a **base-game Sun/Water scaling spirit** whose win plan is to hit max-tier Massive Flooding every turn from T4 onward. It's one of the two lowest-complexity spirits (Play Difficulty 0) — a teaching spirit that's also surprisingly effective at high difficulty via its reclaim-loop innate.

**Wiki-printed playstyle note**:

> Lots of sacred sites for targeting your innate. The best way to get them is generally to focus on expanding to wetlands, where your special rule (River's Domain) gives you a sacred site with only 1 presence.

**Identity capture** (latentoctopus):

> River is an innate-focused scaling Spirit whose win plan is to hit max-tier Massive Flooding every turn from T4/T5 onward.

**The Sacred-Site shortcut**: River's Domain is unique — 1 Presence in a Wetland = a sacred site. Most spirits need 2 presence to make a sacred site; River needs 1. This effectively doubles River's effective presence count for targeting purposes when Wetlands are available.

**Complexity signal**: Low is correct — the simplest "Scale the Innate" spirit. One innate. One Special Rule. Linear energy track (1→2→2→3→4→4→5). Decision-load is *when* to Reclaim and *which Minor* to draft — not much else.

## Starting Setup

> Put **1 Presence** on your starting board in the **highest-numbered Wetlands**.

Only 1 starting presence, which immediately counts as a sacred site via River's Domain.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                             | Best when                                                  |
|--------|-----------------------------------------------------|------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy             | Default Reclaim turn — full bundle                         |
| **G2** | Add Presence (Range 1) + Add Presence (Range 1)     | Rapid spread turn                                          |
| **G3** | Gain 1 Power Card + Add Presence (Range 2)          | Card + placement                                           |

River's growth is benign — all three options are usable. G1 bundles Reclaim + card + energy and is used every 3–4 turns in the standard opening.

## Presence Tracks

- **Energy track** (7 slots): `energy1 → energy2 → energy2 → energy3 → energy4 → energy4 → energy5`
  - 1E → 2E → 2E → 3E → 4E → 4E → 5E
- **Card-play track** (7 slots): `card1 → card2 → card2 → card3 → reclaim1 → card4 → card5`
  - 1 CP → 2 CP → 2 CP → 3 CP → Reclaim 1 → 4 CP → 5 CP

**Starting income**: 1 Energy, 1 Card Play. Linear energy scaling — no marker complexity.

## Core Mechanics & Special Rules

### Special Rule: River's Domain

> Your Presence in Wetlands counts as Sacred Site.

1-Presence Wetland = sacred site. Targeting from sacred sites matters for Range-from-Sacred-Site abilities (Wash Away, Flash Floods).

### Innate: Massive Flooding

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push 1 Explorer/Town. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | *Instead*, 2 Damage. Push up to 3 Explorers/Towns. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | *Instead*, 2 Damage to each Invader. |

**L3 (Max-tier Massive Flooding)** is River's endgame — 2 damage per Invader in a populated land destroys everything a T+ ravage land. Played every turn from T4/T5 onward, this is the whole spirit.

## Unique Cards (all 4, Wiki-verified)

### Wash Away
- **1 Energy · Slow · Range 1 · Any Land · Water, Earth**
- *Push 1 Explorer/Town/City.* *(+1 damage per Water if text says so — verify from physical card.)*

1E push for any Invader type. Lennert: *"ideal target is a land where the Invaders just explored but which started with a Town — stopping the T2 city-build. Otherwise push towards the coast to create inland dead zones."*

### River's Bounty
- **0 Energy · Slow · Range 0 · Any Land · Sun, Water, Animal**
- *Gather 1 Dahan. Gather 1 Explorer. Each Dahan in target land destroys 1 Explorer.*

0-cost Dahan-vs-Explorer. The key reclaim-engine card — played and reclaimed every cycle. Sun+Water double-prime.

### Flash Floods
- **2 Energy · Fast · Range 1 · Any Land · Sun, Water**
- *1 Damage. +1 Damage per Water you have.*

Water-scaling damage at 2E Fast. Key to Massive Flooding threshold-feeding during non-Reclaim turns.

### Boon of Vigor
- **0 Energy · Fast · No Range · Any Spirit · Sun, Water, Plant**
- *Target Spirit gains +1 Energy this turn and +1 Energy at end of turn (total +2).*

Ally gift — **Lennert: "Playing Boon of Vigor on yourself is always wrong. It generates more energy for an ally."** Gift to a partner who needs Energy for Majors or plays.

## Key Strategic Principles

1. **Hit max-tier Massive Flooding every turn from T4/T5.** This is the whole win condition.
2. **Water + Sun are co-primary.** Both unlock Flooding tiers; both sit in starter elements.
3. **Minors > Majors by default.** Lennert: *"The main advantage of major powers is dealing with built-up lands that simply have too many invaders to efficiently handle with minor powers. River is able to handle those problem lands using the highest level of Massive Flooding, so I think it's generally better to just get a ton of minor powers."*
4. **Wetlands = free sacred sites.** Target Wetland-expansion over other terrain.
5. **Boon of Vigor never self-targeted.** Always gift to ally.
6. **Wash Away targets the pre-build land, not the city land.** Lennert heuristic.
7. **Reclaim cycle**: River's Bounty is reclaimed every loop — 0-cost Sun/Water card always in rotation.

## Possible Openings

### Shared starting state

- **1 Presence on board**: highest-numbered Wetland (= sacred site via River's Domain).
- **4 Uniques in hand**: Wash Away (1E Slow, Water/Earth), River's Bounty (0E Slow, Sun/Water/Animal), Flash Floods (2E Fast, Sun/Water), Boon of Vigor (0E Fast, Sun/Water/Plant).
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Consistent Reclaim Loop 🟨 (default, latentoctopus)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/river-opening1/).

**T1 · Growth**: G2 bottom.
**T1 · Play** (1E, 1 CP): **River's Bounty + one card**.
- Elements hit: 1 Sun + 2 Water (+ whatever else). Massive Flooding L1 (1 Sun + 2 Water) live.

**T2 · Growth**: G2 bottom. **Reclaim River's Bounty** (via Reclaim-1 track slot — *note: you may need G1 for full Reclaim at this stage, depending on track state*).
- Play 3 cards → 2 Sun + 3 Water elements → Massive Flooding **L2**.

**T3 · Growth**: Reclaim (G1) + Gain Minor.
**T3 · Play**: River's Bounty + new Minor. Hold 2 Sun / 3 Water / 1 Earth.

**T4 · Growth**: G2 bottom.
**T4 · Play**: Reclaim River's Bounty + play all 4 starters → **3 Sun / 4 Water / 1 Earth, unlocking max-level Massive Flooding**.

**Core constraint**: "Little presence placement past T4 requires careful planning."

### Opening B — Lennert's 5-turn arc (more detailed) 🟨

From [BGG 1967085](https://boardgamegeek.com/thread/1967085/openings-river-surges-sunlight).

**T1 · Growth**: G2 bottom.
**T1 · Play**: **Wash Away + River's Bounty**.
- Wash Away target: "a land where the Invaders just explored but which started with a Town — stopping the T2 city-build."
- Alternative: push toward coast to create inland dead zones.

**T2 · Growth**: G2 bottom. Reclaim River's Bounty.
**T2 · Play**: Flash Floods + Boon of Vigor + River's Bounty (3 plays, **Massive Flooding L2**).

**T3 · Growth**: Reclaim + Gain Minor (+1E).
**T3 · Play**: Bounty + Flash Floods + Wash Away, hit **Massive Flooding L2** again.

**T4 · Growth**: G2 bottom + Gain second Minor.
**T4 · Play**: all 4 starting cards if draws allow.

**T5 · Growth**: Reclaim + Gain Minor (+1E).
**T5 · Play**: "You can now definitely activate the highest tier of your innate no matter what minors you drew" by playing all 4 starters.

**Critical note** (Lennert): *"Make sure you have enough energy for this turn — if both drafted Minors cost energy, River's Bounty must be played both T3 and T4."*

### Opening C — Top-Track Variant (optional Major) 🟥

**T1 · Growth**: G3 top instead of G2 bottom.
- Rationale: top-track gives +5E by T3, enough to plausibly grab a Major T3–T4 if cheap and reclaim-loop it.

Trade-off: loses the T2 Reclaim; card plays off-curve for L2 Flooding.

### Opening Decision

- **Default Opening A or B** (both very similar). Lennert's variant is more explicit.
- **Opening C** only when Major-rushing is the plan.

## Card Priority Ratings

### Uniques — River-specific ranking

1. **River's Bounty** — 0-cost Sun/Water + reclaim-every-cycle engine.
2. **Flash Floods** — Water-scaling damage; innate-threshold feeder.
3. **Wash Away** — surgical Push for Build-prevention.
4. **Boon of Vigor** — ally-gift; never self-target.

### Top 10 Minor Draft Picks (Water + Sun priority)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Gift of Proliferation** | 1 | Fast | Plant, Water | Water-feeder |
| 2 | **Absorb Essence** | 0 | Fast | Water | 0-cost Water |
| 3 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun |
| 4 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Sun |
| 5 | **Call to Isolation** | 0 | Slow | Water, Animal | 0-cost Water |
| 6 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Sun |
| 7 | **Flow Like Water, Reach Like Air** | 2 | Fast | Sun, Air, Water | Sun + Water |
| 8 | **Purify the Land** | 0 | Slow | Moon, Water, Plant | 0-cost Water + blight |
| 9 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Water + 2E |
| 10 | **Gift of Power** | 1 | Fast | Moon | Utility flex |

### Top 5 Major Draft Picks (Opening C only)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Water-rich late-game closer |
| 2 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Cheap Water |
| 3 | **Sky Stretches to Shore** | 2 | Fast | Sun, Moon, Air | 2-cost Sun |
| 4 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Sun + board-control |
| 5 | **Manifest Incarnation** | 3 | Fast | Moon, Plant, Animal, Water | Water-rich utility |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Single-land non-Sun-Water Majors | Don't compete with max-tier Flooding |
| Presence-destroyers | River's Domain Wetlands are foundational |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A / B   | ★★★★☆  | Push-based disruption matches Prussia tempo                  |
| England                 | A / B   | ★★★☆☆  | Needs Max-tier Flooding against coastal Cities               |
| Sweden                  | A / B   | ★★★☆☆  | Push-kit works against Sweden                                |
| France-Plantation       | A / B   | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A / B   | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A / B   | ★★★☆☆  | Dahan-destruction doesn't hurt River much                    |
| Habsburg Mining         | A / B   | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A / B   | ★★★☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Wetland-heavy boards give free sacred sites via River's Domain. Prioritize **Board A / B / D orientations** that push river lands toward buildable targets. No adversary matrix explicit in Lennert — core advice applies broadly.

## Game-Phase Strategy

### Early (T1–3)
- G2 bottom → reclaim → G2 bottom cadence.
- Massive Flooding L1 on T1, L2 on T2.
- River's Bounty reclaimed every cycle.

### Mid (T4–6)
- Max-tier Massive Flooding (L3) active.
- Minor-integration; Water + Sun drafts.
- Boon of Vigor ally-gifting.

### Late (T7+)
- Reclaim loops stabilize; Massive Flooding every turn.
- Optional Major (Tsunami, Manifest).

## Synergy Partners (Multiplayer)

- **Energy-hungry allies** for Boon of Vigor — Lightning, Thunderspeaker, Major-users.
- **Defending partners** — cover River's Bounty-targeted Dahan lands so Dahan actually kill.
- **Grouped-Invader partners** — Massive Flooding L3 destroys pods; partners who concentrate Invaders multiply River's output.

## Common Mistakes

```admonish failure title="Lennert's named mistakes"
1. **Taking top-track T1 "for the energy"** — loses T2 Reclaim; off-curve for L2 Flooding.
2. **Playing Boon of Vigor on yourself.** Always gift.
3. **Reclaiming wrong card T4 and losing T5 max-innate combo** — must be able to play all 4 starters.
4. **Wash-Away-ing a coastal Town** instead of the land-that-will-build-a-City next turn.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Wash Away + River's Bounty; Flooding L1 live         |
| 2    | G2 + Reclaim River's Bounty + Flash Floods; Flooding L2   |
| 3    | Reclaim + Gain Minor; Flooding L2                         |
| 4    | G2 + second Minor; all 4 starters available                |
| 5    | Reclaim + Gain Minor; Flooding L3 **reliable every turn**  |
| 6+   | Max-tier Flooding cycle; optional Major integration        |

## Source Notes

- **Mechanics**: `data/references/wiki/river-surges-in-sunlight.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus Opening 1 + Lennert BGG 1967085.
