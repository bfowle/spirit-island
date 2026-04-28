# Sharp Fangs Behind the Leaves

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Fangs concepts](https://latentoctopus.github.io/guide/fangs-concepts/) + [4 Opening pages](https://latentoctopus.github.io/guide/fangs-opening1/) + [jyonker13's BGG openings thread 1978097](https://boardgamegeek.com/thread/1978097/openings-sharp-fangs-behind-leaves).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch & Claw                                      |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "many" — pick **multiple** growth options per turn (see table) |
| Power summary (1–5)   | **Offense 3** · Control 3 · **Fear 4** · Defense 2 · Utility 1 |
| Primary Elements      | **Animal** (Ranging Hunt all tiers) · Plant (L2 damage) · Moon/Fire (Frenzied Assault) |
| Special Rules         | Ally of the Beasts (Presence moves with Beast) + Call Forth Predators (replace Presence with Beast during Spirit Phase) |
| Aspects (JE)          | Encircle · Unconstrained `[VERIFY]`                |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Concepts](https://latentoctopus.github.io/guide/fangs-concepts/) + [Openings 1–4](https://latentoctopus.github.io/guide/fangs-opening1/) |
```

## Spirit Overview — Framing

Sharp Fangs is the **Beast-centric board-control spirit** — its entire game is *where are the Beasts, how many, and can Ranging Hunt damage them into action this turn*. Both Special Rules tie presence-movement to Beasts; both Uniques key off Beast-presence; the primary innate (Ranging Hunt) gates damage on Beasts-in-target-land. Drafts, placements, and opening lines are all downstream of one question: **Beast topology**.

**Wiki-printed playstyle note**:

> Relies on Beasts to do its damage; good at picking off single Invaders, but may need to team up with other Spirits to handle whole lands at once.

**Identity capture** (latentoctopus concepts, paraphrased):

> *Beasts placement is the single most important aspect of playing Fangs.*

The non-obvious twist: the Ranging Hunt *movement* threshold (L3, Push 2 Beast) is as strategically important as the damage thresholds — because it sets next turn's Beast topology. Don't treat Push-tier as a filler; it's the positioning lever.

**Complexity signal**: Moderate. 4 uniques, 2 innates, 2 special rules, but the mental load is high because every decision (place presence, gain card, play card) interacts with a mobile Beast layer.

## Starting Setup

> Put **1 Presence and 1 Beast** on your starting board in the **highest-numbered Jungle**.
> Put **1 Presence** in a land of your choice with Beast anywhere on the island.

Two starting presences, one starting Beast on your board, placement flexibility on the second presence (place adjacent to an existing cross-board Beast for coverage).

## Growth Options (growthtype: "many" — pick multiple)

Fangs's "many" growth is unusual: the guide reference *three equivalent two-turn bundles*:

| Turn pair | Bundle                                      | Net effect                                      |
|-----------|---------------------------------------------|-------------------------------------------------|
| G3 → G3   | Gain card×2, +2 Energy                      | 2 cards + 2E                                    |
| G4 → G1   | +3E (G4), Sharp1 + Gain card (G1)           | 1 card + 3E + Reclaim + Sharp-token placement   |
| G3 → G1   | Gain card + energy1 (G3), Sharp1 + Gain card (G1) | 2 cards + 1E + Reclaim + Sharp placement  |

| Growth | Effects (per Wiki)                          |
|--------|--------------------------------------------|
| **G1** | Sharp1 (add 1 Sharp token) + Gain 1 Power Card |
| **G2** | Sharp (add 1 Sharp token)                  |
| **G3** | Gain 1 Power Card + +1 Energy              |
| **G4** | +3 Energy                                  |

("Sharp" tokens here are Fangs's Beast-spawning special token. In-game iconography replaces the "Beast" label.)

## Presence Tracks

- **Energy track** (7 slots): `energy1 → animal → plant → energy2 → animal → energy3 → energy4`
  - 1E → +Animal marker → +Plant marker → 2E → +Animal → 3E → 4E
- **Card-play track** (6 slots): `card2 → card2 → card3 → reclaim1 → card4 → card5reclaim1`
  - 2 CP → 2 CP → 3 CP → Reclaim 1 → 4 CP → 5 CP + Reclaim 1

**Starting income**: 1 Energy, **2 Card Plays** (unusually high starting CP — Fangs expects 2 plays from T1).

## Core Mechanics & Special Rules

### Special Rule: Ally of the Beasts

> Your Presence may move with Beast. (Whenever a Beast moves from 1 of your lands to another land, you may move 1 Presence along with it.)

Free presence repositioning whenever Beasts move. Ranging Hunt L3 (Push 2 Beasts) is *also* a Push 2 Presence if those Beasts come from your lands.

### Special Rule: Call Forth Predators

> During each Spirit Phase, you may replace 1 of your Presence with 1 Beast. The replaced Presence leaves the game.

Exchange 1 Presence for 1 Beast per turn. The exchange is permanent — Presence is gone, Beast stays.

### Innate: Ranging Hunt

- **Speed**: Fast · **Range**: 1 · **Target**: No Blight

| Level | Thresholds                              | Effect                                  |
|-------|-----------------------------------------|-----------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | You may Gather 1 Beast. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant + 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage per Beast. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | You may Push up to 2 Beast. |

Note L3's threshold is only **2 Animal** — the same as L1. The L3 effect is a *choice* at that threshold, not an extra-element unlock. jyonker13 on BGG: *"Ranging Hunt has no extra-element scaling."* — the damage ceiling is fixed by L2.

### Innate: Frenzied Assault

- **Speed**: Slow · **Range**: 1 · **Target**: Beast

| Level | Thresholds                              | Effect                                                       |
|-------|-----------------------------------------|--------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Fear and 2 Damage. Remove 1 Beast. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 5 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +1 Fear and +1 Damage. |

Mid-game finisher. 4 Animal is a big ask; reached by T4+ once Minor drafts stack Animal.

## Unique Cards (all 4, Wiki-verified)

### Prey on the Builders
- **1 Energy · Fast · Range 0 · Any Land · Moon, Fire, Animal**
- *You may Gather 1 Beasts. If target land has Beasts, Invaders do not Build there this turn.*

Build-prevention + Beast-gather. Carries Moon/Fire → feeds Frenzied Assault thresholds. The Build-cancel is Fangs's signature [Prevent-Build play](../../fundamentals/invader-lifecycle.md).

### Teeth Gleam from Darkness
- **1 Energy · Slow · Range 1 from a Jungle · Land with no Blight · Moon, Plant, Animal**
- *1 Fear. Add 1 Beasts.* **OR** *If target land has both Beasts and Invaders: 3 Fear.*

Beast-seeding or fear-spike. Moon/Plant/Animal — all Fangs primary elements. "No Blight" clause is the only friction.

### Terrifying Chase
- **1 Energy · Slow · Range 0 · Any Land · Sun, Animal**
- *Push 2 Explorers/Towns/Dahan. Push another 2 Explorers/Towns/Dahan per Beasts in target land. If you Pushed any Invaders, 2 Fear.*

Beast-scaling Push. 2-Beast land = Push 6 pieces. Useful for both board-shaping and Isolation setup via partner powers.

### Too Near the Jungle
- **0 Energy · Slow · Range 1 from a Jungle · Any Land · Plant, Animal**
- *1 Fear. Destroy 1 Explorer.*

**The workhorse Reclaim target** — latentoctopus: *"Reclaim 1 Too Near the Jungle should probably be the Power you reclaim most often after you uncover the Reclaim 1 spot."* 0-cost, kills Explorers (Fear trigger), carries Fangs's primary elements.

## Key Strategic Principles

1. **Beasts placement is strategic output, not input.** Every play should end in a better Beast topology for next turn's Ranging Hunt.
2. **Ranging Hunt has no extra-element scaling.** Over-drafting Animal past 3 is wasted element-count. Draft Animal to 3, then pivot.
3. **Plant is the L2 lever.** 2 Plant + 3 Animal = damage. One Plant Minor unlocks the damage tier.
4. **Reclaim Too Near the Jungle for free 0-cost Explorer kills.** Bottom-track Reclaim 1 pattern.
5. **Majors are optional — latentoctopus's Op 2 / Op 4 are "no Majors" designs.** Default Minor; Majors only when adversary pressure demands.
6. **Drafting too many 1-cost Minors is a trap without G4.** latentoctopus: *"Can handle 1 or 2, but more than that will cause issues."* Prefer 0-cost.
7. **Ally of the Beasts = free relocation.** Every Beast push via Ranging Hunt L3 is a free Presence repositioning.

## Possible Openings

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

### Shared starting state

- **2 Presence on board** (1 on starting Jungle + 1 with a cross-board Beast), **1 Beast on starting land**.
- **4 Uniques in hand**.
- **Starting income**: 1 Energy, 2 Card Plays.

### Opening A — Top-Track Hybrid (Minors) 🟨 — vs Prussia / Sweden / France / Scotland / Russia

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/fangs-opening1/).

**T1**: G2 (Sharp token) + G3 (gain Minor); play 2 (both starting Uniques if possible, for Ranging Hunt damage).
- *Pause-point*: Ranging Hunt L2 (2 Plant + 3 Animal) is only reachable T1 if drafted Minor carries Plant + Animal.

**T2**: G2 + G3 Minor; play 2.

**T3**: **Reclaim + G2 bottom**; play 1 Animal + 1 Plant card for Ranging Hunt.

**T4**: G2 bottom to 3 CP + G4; play 1 Animal + 1 Plant card.

**Target state**: Ranging Hunt firing every turn from T2 onward.

### Opening B — Full Bottom (Minors, 3 CP rush) 🟨 — vs England / Sweden / Habsburg Livestock

From [latentoctopus Opening 2](https://latentoctopus.github.io/guide/fangs-opening2/).

**T1**: G2 bottom + G3 Minor; play 2.

**T2**: G2 bottom + G4; play 3 (high-CP burst).

**T3**: Reclaim + G2 top; play 3.

**T4**: G2 bottom + G3; play 3.

**Target state**: 3 plays T2, 5 plays T6. Raw card-volume approach when Ranging Hunt damage isn't reliable (damage-resistant adversaries).

### Opening C — Top-Track Hybrid with Major 🟥 — vs damage-resilient adversaries

From [latentoctopus Opening 3](https://latentoctopus.github.io/guide/fangs-opening3/).

**T1–T2**: same as Opening A.

**T3**: Reclaim **with Major gain** (cost 2–5).

**T4**: G2 bottom + G4; play the Major + 1 other.

Reserved for England, Sweden, Habsburg Livestock, Habsburg Mining — where Prey on the Builders + raw Ranging Hunt damage isn't enough.

### Opening D — Reclaim-Heavy Bottom (Ranging Hunt every turn) 🟥 — vs BP / Scotland / Russia

From [latentoctopus Opening 4](https://latentoctopus.github.io/guide/fangs-opening4/).

**T1**: G2 top + G4; play both starting Uniques.
**T2**: Reclaim + G2 bottom.
**T3**: Reclaim + G2 bottom.
**T5**: Reclaim + G2 bottom; play 4.

Very Energy-starved. Goal: Ranging Hunt every turn including T1. Maximum Beast seeding early.

### Opening Decision

- **Default Opening A** for most Prussia/Sweden/France/Scotland/Russia matchups.
- **Opening B** for England / tough Sweden — when damage-dealing must scale via cards, not innate.
- **Opening C** only when Ranging Hunt damage is degraded (England L4+, high-Health NI scenarios).
- **Opening D** only for BP / Scotland / Russia where maximum Beast seeding matters early.

## Card Priority Ratings

### Uniques — Fangs-specific ranking

1. **Too Near the Jungle** — reclaim-every-cycle 0-cost Explorer-kill.
2. **Prey on the Builders** — Build-cancel, Beast-gather, Moon/Fire.
3. **Teeth Gleam from Darkness** — Beast-seeder + fear spike.
4. **Terrifying Chase** — Push scaling with Beasts; situational.

### Top 10 Minor Draft Picks (Animal + Plant prime)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | Plant + Animal feeder 0-cost |
| 2 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Animal |
| 3 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Moon/Earth for Frenzied |
| 4 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Animal + Air utility |
| 5 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant for L2 |
| 6 | **Absorb Essence** | 0 | Fast | Water | Flex-element filler |
| 7 | **Bats Scout for Raids by Darkness** | 1 | Fast | Moon, Air, Animal | Moon + Animal |
| 8 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | Plant + Animal 0-cost |
| 9 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Moon + Animal |
| 10 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Moon-utility |

### Top 5 Major Draft Picks (only for Opening C)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Instruments of Their Own Ruin** | 3 | Fast | Fire, Air, Animal | Animal + multi-land |
| 2 | **Venomous Spiders** | 2 | Fast | Moon, Animal | Cheap + Animal |
| 3 | **Angry Bears** | 3 | Fast | Moon, Fire, Animal | Animal + Fear |
| 4 | **Insatiable Hunger of the Swarm** | 3 | Fast | Animal, Plant | Multi-element Fangs-prime |
| 5 | **Settling into Hunting Grounds** | 3 | Slow | Moon, Earth, Plant, Animal | Multi-element utility |

### Cards to Avoid

| Card | Reason |
|------|--------|
| 1-cost Minors past 2 in hand | Without G4-every-turn, cost-stack breaks sustainability |
| Teeth Gleam early reclaim | Costs 1E and redundant with drafted 0-cost kills |
| Non-Jungle-targetable Slow | R1-from-Jungle constraint on 2 Uniques limits geometry |

## Adversary Matchup Matrix

Per latentoctopus Opening mapping:

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A / D   | ★★★★☆  | Beast-seeding + Build-cancel are Prussia-perfect             |
| Sweden                  | A / B   | ★★★☆☆  | Bottom-track card-volume when Sweden Build spam compounds    |
| France-Plantation       | A       | ★★★☆☆  | Top-track hybrid                                             |
| Scotland                | A / D   | ★★★★☆  | Opening D for max Beast-seed                                 |
| Russia                  | A / D   | ★★★☆☆  | Dahan-destruction events hurt Beast kit                      |
| England                 | B / C   | ★★★☆☆  | Opening C (Major) for damage-resilient                       |
| Habsburg Livestock      | B / C   | ★★☆☆☆  | Animal-token interaction `[VERIFY]`                          |
| Habsburg Mining         | C       | ★★★☆☆  | Opening C against mine-token grind                           |

## Board / Map Configuration

`[VERIFY — latentoctopus does not provide ratings]`. Heuristic: favor boards with **dense Jungle clusters** (2+ Uniques are R1-from-Jungle). Board A / B have concentrated Jungle; E / F spread Jungle out which slightly weakens the kit.

## Game-Phase Strategy

### Early (T1–3)
- Beast seeding + Ranging Hunt L2 damage.
- Prey on the Builders on any Build-threat land.
- Too Near the Jungle on first Reclaim.

### Mid (T4–6)
- 3 CP unlocked. Frenzied Assault L1 reachable once 4 Animal is stacked.
- Beast-push via Ranging Hunt L3 repositions presence for next-turn setup.

### Late (T7+)
- Frenzied Assault L2 reliable. Multi-land damage loops via Too Near the Jungle reclaim.

## Synergy Partners (Multiplayer)

- **Energy-donor partners** — Fangs is Energy-starved; 1–2 E/turn gift helps.
- **Card-draw partners** — bottom-track openings reward extra draws.
- **Isolation partners** — Isolated lands are ideal Beast-concentration targets.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Over-drafting Animal past 3.** Ranging Hunt has no extra-element scaling.
2. **Too many 1-cost Minors without G4.** latentoctopus: "more than 2 will cause issues."
3. **Wasting Reclaims on Teeth Gleam instead of 0-cost.** Prefer Too Near the Jungle reclaim.
4. **Ignoring Beast-in-blighted-land placements.** Gives you Prey-on-the-Builders fallback targets.
5. **Treating Ranging Hunt L3 as filler.** Push 2 Beast *is* the positioning lever.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | Both starting Uniques + Ranging Hunt L1 live             |
| 2    | Ranging Hunt L2 (2P + 3A) online; 3+ Beasts on board     |
| 3    | Reclaim + re-position via Beast-move Special Rule        |
| 4    | 3 CP; Frenzied Assault L1 reachable                       |
| 5+   | Frenzied Assault L1/L2; Too Near the Jungle loop          |

## Source Notes

- **Mechanics**: `data/references/wiki/sharp-fangs-behind-the-leaves.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus concepts + Opening 1–4 pages.
- **BGG**: [jyonker13 thread 1978097](https://boardgamegeek.com/thread/1978097/openings-sharp-fangs-behind-leaves).
