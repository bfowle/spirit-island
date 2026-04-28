# Heart of the Wildfire

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Wildfire concepts](https://latentoctopus.github.io/guide/wildfire-concepts/) + [Openings 1–2](https://latentoctopus.github.io/guide/wildfire-opening1/) + [jyonker13's BGG openings thread 2313791](https://boardgamegeek.com/thread/2313791/openings-heart-wildfire).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 1                                 |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "one" — pick **one** growth option per turn (bundled effects) |
| Power summary (1–5)   | **Offense 5** · Control 3 · **Fear 4** · Defense 1 · Utility 2 |
| Primary Elements      | **Fire** (Special Rule + both innates) · Plant (Firestorm L2) · Air (Firestorm L3 + L4) · Earth (Regrows L3) |
| Special Rules         | Blazing Presence (Presence-placement = damage + Blight) + Flame's Fury (the incarna-style Fire card) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Concepts](https://latentoctopus.github.io/guide/wildfire-concepts/) + [Opening 1](https://latentoctopus.github.io/guide/wildfire-opening1/) + [Opening 2](https://latentoctopus.github.io/guide/wildfire-opening2/) |
```

## Spirit Overview — Framing

Wildfire is "perhaps the most flexible and challenging Aggro Spirit the game has to offer" (jyonker13). Its identity is *Blazing Presence* — every Presence placement damages the land and (if 2+ Fire showing) adds Blight. You're **actively progressing the loss condition** (latentoctopus) just by expanding, which means placement decisions are commitments, not free gains. Reclaim turns are structurally weak because placement *is* Wildfire's damage, and a Reclaim turn is a no-placement turn.

**Wiki-printed playstyle note**:

> Damage and fear machine extraordinaire, but also spreading blight at the same time. Can self-sustain and reposition with Regrows innate, but there's a window before the innate is online where presence placement is a one-way commitment.

**Identity capture** (jyonker13, BGG):

> Perhaps the most flexible and challenging Aggro Spirit the game has to offer.

**The top track trap** (latentoctopus): Wildfire's top track is *terrible* — rushing 1 Energy delays 2 CP to T4 and 3 CP to T6, and the cumulative Energy gain vs. ignoring the top track is only ~+4 by T5 (= one G3). **Default bottom track.**

**Complexity signal**: High is accurate. The spirit has few decision axes but each is irreversible — placement, blight, reclaim timing. Losing a game to cascade-blight 3 turns ago is the signature Wildfire fail state.

## Starting Setup

> Put **3 Presence and 2 Blight** on your starting board in the **highest-numbered Sands**. *(Blight comes from the box, not the Blight Card.)*

Three starting presences — highest starting count in the game — all pre-loaded on a **2-Blight land**. Your starting board is a Wildfire kill zone out of the gate.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                                       | Best when                                                         |
|--------|------------------------------------------------------------------------------|-------------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy                                       | Hand depleted; Boon-of-Abundance-equivalent turn                  |
| **G2** | Gain 1 Power Card + Add Presence (Range 3)                                    | Card-gain + placement; Major draft                                |
| **G3** | Add Presence (Range 1) + +2 Energy + Wildfire (special damage)                | **Default** — placement + energy + free damage                    |

Note G3 has a **Wildfire effect** bundled — the placement triggers Blazing Presence, adding damage to the destination land automatically. G3 is your signature opening Growth.

## Presence Tracks

- **Energy track** (6 slots): `energy0 → fire → energy1 → energy2 → fireplant → energy3`
  - 0E → +Fire marker → 1E → 2E → +Fire + Plant marker → 3E
- **Card-play track** (6 slots): `card1 → fireX → card2 → card3 → fireX → card4`
  - 1 CP → Fire scaling → 2 CP → 3 CP → Fire scaling → 4 CP

**Starting income**: 0 Energy, 1 Card Play. Energy-light and CP-tight early. Top track reveals 1 Energy after first presence placed — enough to explain why latentoctopus calls it "terrible" (one slot for one Energy, late).

## Core Mechanics & Special Rules

### Special Rule: Blazing Presence

> Post-Setup, after your Presence is added/moved, in the land it goes to:
> - For each Simplefire showing on your Presence Tracks, do 1 Damage.
> - If 2 Simplefire or more are showing on your Presence Tracks, add 1 Blight.
> - Push all Beast and any number of Dahan. If you add multiple Presence to the land, resolve effects once.

Placement = Damage. With 0 Fire markers revealed: 0 damage. With 1 Fire revealed: 1 damage on placement. With 2+ Fire: 1 damage + Blight on placement. **Your Growth is a damage engine**, not a neutral card-economy move.

### Special Rule: Flame's Fury (printed as a Unique)

See Unique card section below — Flame's Fury is printed as a card but functions as a pseudo-special-rule in most Wildfire analysis.

### Innate: Firestorm

- **Speed**: Fast · **Range**: 0 · **Target**: Blight

| Level | Thresholds                              | Effect                                                         |
|-------|-----------------------------------------|----------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Damage per 2 Fire you have. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | *Instead*, 1 Damage per Fire you have. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Split this Power's Damage however desired between target land and any number of your lands with Blight. |
| 4     | 7 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | In a land with Blight where you have Presence, Push all Dahan. Destroy all Invaders and Beast. Add 1 Blight. |

**Plant doubles the damage output** — 1 damage per 2 Fire (L1) → 1 damage per 1 Fire (L2). This is why latentoctopus says *"a Power that only grants Plant is better than one that only grants Fire"* — hitting 3 Plant doubles every Firestorm's output.

### Innate: The Burned Land Regrows

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                              | Effect                                                         |
|-------|-----------------------------------------|----------------------------------------------------------------|
| 1     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | If target land has 2 Blight or more, remove 1 Blight. |
| 2     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | *Instead*, remove 1 Blight. |
| 3     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Remove another Blight. |

Self-healing. High Fire-threshold (4+) means this unlocks mid-game when Fire markers stack, not T1.

## Unique Cards (all 4, Wiki-verified)

### Asphyxiating Smoke
- **2 Energy · Slow · Range 2 from your Sacred Site · Any Land · Fire, Air, Plant**
- *1 Fear. Destroy 1 Town. Push 1 Dahan.*

Town-killer at R2 (sacred-site extended). Fire/Air/Plant carries all three Wildfire-primary elements.

### Flame's Fury
- **0 Energy · Fast · No Range · Any Spirit · Sun, Fire, Plant**
- *Target Spirit gains 1 Energy. Target Spirit does +1 Damage with each Damage dealing Power they use this turn. (Powers which Damage multiple lands or each Invader only get 1 extra Damage total. Repeated Powers keep the +1 per resolution.)*

Damage-buff + 1 Energy gift. In solo, self-target for +1 damage on every damaging Power this turn. Fire/Plant carries threshold-critical elements.

### Flash-Fires
- **2 Energy · Slow · Range 1 · Any Land · Fire, Air**
- *1 Fear. 1 Damage.*

**The pivot card.** latentoctopus: *"Flash-fires should only be played if really needed, because it's a terrible card: weak effect, high cost, bad elements (no Fire)."* Actually has Fire per the Wiki parse — revisit: Fire, Air in its elements row. The card's weakness is the *ratio* — 2E + Slow + 1 Damage + 1 Fear is dreadful on its face; its value is the Fire+Air element combo. The Major-draft opening exists largely to Forget this.

### Threatening Flames
- **0 Energy · Fast · Range 0 · Land with 1+ Blight and 1+ Invaders · Fire, Plant**
- *2 Fear. Push 1 Explorer/Town per Terror Level from target land to adjacent lands without your Presence. If there are no such adjacent lands, +2 Fear.*

The starter-Ravage-mitigation card. 0-cost Fast, Fire+Plant. Push-scaling with Terror Level means T1 = 1 push, T3 = 3 push. Primarily used T1 on the starting-Sands land to push Explorers out before ravage.

## Key Strategic Principles

1. **Bottom track by default.** Top track is "terrible" per latentoctopus. G3 is the workhorse.
2. **Plant > Fire in draft priority.** 3 Plant doubles Firestorm output; a Plant-only Minor outranks a Fire-only Minor.
3. **Placement is damage.** Every Growth-triggered Presence-add does Blazing Presence damage. Plan placements into Ravage lands or pre-Ravage Build-threat lands.
4. **Reclaim is weak.** Reclaim turns have no placement = no Wildfire damage. Minimize Reclaim cycles.
5. **Entrap the Forces of Corruption is game-changing.** Blight-management Minors outrank raw-element picks.
6. **Flash-Fires is a forget candidate.** Take a Major early only when you have a clean Forget target (Flash-Fires).
7. **Don't Full Fuego when Invaders just explored into a blighted land** — cascading blight will flip the card before you can recover.

## Possible Openings

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

### Shared starting state

- **3 Presence on starting Sands land**; **2 Blight on that land**.
- **4 Uniques in hand**: Asphyxiating Smoke (2E Slow, Fire/Air/Plant), Flame's Fury (0E Fast, Sun/Fire/Plant), Flash-Fires (2E Slow, Fire/Air), Threatening Flames (0E Fast, Fire/Plant).
- **Starting income**: 0 Energy, 1 Card Play.
- Growth type **"one"**: pick G1/G2/G3 per turn.

### Opening A — Full Bottom Track (Minor/Mixed) 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/wildfire-opening1/).

**T1 · Growth**: G3 bottom — Add Presence (R1) + 2E + Wildfire-damage.
- Income: 2E from G3. Place presence into a Ravage land or high-threat land.

**T1 · Play** (2E, 1 CP): **Asphyxiating Smoke** — 2E Town-kill.
- *Pause-point*: Threatening Flames is 0E — consider swapping if Asphyxiating target isn't prime.

**T2 · Growth**: G2 top — Gain 1 Minor + Presence (R3).
- Card-gain turn. Draft Plant/Fire priority.

**T2 · Play** (0E + drafted Minor energy, 1 CP): **Threatening Flames** to push Explorers from starting land.

**T3 · Growth**: G3 bottom — +Presence + 2E + Wildfire.
- Firestorm L1 live if 1 Plant threshold hit (native elements from played cards).

**T3 · Play** (2E, 2 CP): **Flame's Fury + Flash-Fires/Minor**.

**T4 · Growth**: G1 — Reclaim + Gain Minor (or Major if ready).

**T5 · Growth**: G3 bottom.

**T4 end state** (audit):
- 5–6 presence on board, 3–4 lands with Blight.
- 2 CP baseline.
- Firestorm L1 reliable; L2 reachable if 3 Plant drafted.
- Blight track has likely flipped — Regrows innate becomes critical.

**Confidence**: 🟨 latentoctopus's primary opening. Works across most matchups except damage-resistant (England).

### Opening A-Variant — Delayed 2-Fire (lower blight) 🟥

**T1**: G2 bottom + drafted Minor; play Flame's Fury (3 damage via Special Rule + Firestorm).
**T2**: G3 bottom; play Asphyxiating Smoke + Flash-Fires.

Trade-off: earliest 3 CP, lowest blight — but late 2-Fire means fewer Firestorm targets.

### Opening B — Early Major (vs. damage-resilient adversaries, esp. England) 🟥

From [latentoctopus Opening 2](https://latentoctopus.github.io/guide/wildfire-opening2/).

**T1 · Growth**: G2 bottom + **Major gain (Forget Flash-Fires)**.
**T1 · Play**: Threatening Flames.

**T2 · Growth**: G3 top (+1E + Presence).
**T2 · Play** (3E, 2 CP): the Major (if cost ≤ 3E) **or** Asphyxiating Smoke.

**T3 · Growth**: G2 bottom + Minor (or second Major forgetting from discard).
**T3 · Play**: Flame's Fury + remaining Power.

**T4 · Growth**: G1 Reclaim + Gain.

**Target**: damage-resilient matchups where Firestorm alone can't clear. latentoctopus: *"Early Majors are often a gamble, but they can give Wildfire a (slim) fighting chance against damage-resilient adversaries, most notably England."*

### Opening Decision

- **Default Opening A** (Full Bottom) for Prussia, Sweden, France, Scotland, Russia, Habsburg at most levels.
- **Opening A-Variant** when starting conditions demand low-blight T1–T2.
- **Opening B** (Early Major) only vs. England or high-Health NI scenarios.

## Card Priority Ratings

### Uniques — Wildfire-specific ranking

1. **Threatening Flames** — 0-cost Fast, starter-Ravage-management.
2. **Flame's Fury** — damage buff + Energy gift; fits every turn.
3. **Asphyxiating Smoke** — Town-kill 2E.
4. **Flash-Fires** — pivot card; forget on Opening B.

### Top 10 Minor Draft Picks (Plant > Fire > Air)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Entrap the Forces of Corruption** | 1 | Fast | Moon, Plant | Blight-management + Plant |
| 2 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | Plant + Earth + 0-cost |
| 3 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire + multi-element |
| 4 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant |
| 5 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Plant + Earth |
| 6 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Plant |
| 7 | **Fire and Flood** | 2 | Slow | Fire, Water | Fire-Water utility |
| 8 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth |
| 9 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Plant flex |
| 10 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Air + Plant |

### Top 5 Major Draft Picks (for Opening B)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Flow** | 6 | Slow | Fire, Earth | Multi-land damage + Fire |
| 2 | **Infinite Vitality** | 4 | Fast | Fire, Water, Plant | Fire + Plant |
| 3 | **Volcanic Eruption** | 8 | Slow | Moon, Fire, Earth | Late-game closer |
| 4 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap + Fire |
| 5 | **Pillar of Living Flame** | 4 | Fast | Sun, Fire, Air | Fire + Air + element-wide |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Flash-Fires (reclaim target) | Forget priority, not reclaim priority |
| Blight-adding Majors without Regrows-L2 online | Compound loss-condition risk |
| Pure Defend-heavy cards | Wildfire wants Offense / damage, not Defense |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| England                 | B       | ★★☆☆☆  | Damage-resistant; Early Major the only line                  |
| Brandenburg-Prussia     | A       | ★★★★☆  | Placement damage + Build-prevention fits Prussia tempo       |
| Sweden                  | A       | ★★★☆☆  | Build-spam compounds blight risk                             |
| Scotland                | A       | ★★★☆☆  | Coastal Cities are hard to Wildfire-clear                    |
| Russia                  | A       | ★★★☆☆  | Dahan-destruction events don't hurt Wildfire much            |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★☆☆☆  | Mine tokens don't Destroy — blight risk compounds            |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY — latentoctopus does not provide per-board ratings]`. Heuristic: favor boards with **central Sands/Mountains** and **isolated starting land** so initial 2-Blight doesn't cascade via adjacency.

## Game-Phase Strategy

### Early (T1–3)
- Bottom-track G3 cadence. Asphyxiating Smoke + Threatening Flames to clear starting-Sands pre-ravage.
- Firestorm L1 online by T2 with 1 Plant drafted.

### Mid (T4–6)
- Firestorm L2 (3 Plant) = 1 damage per Fire. With 4 Fire markers, that's 4 damage per cast.
- Regrows L1/L2 unlocks; self-heal cycle begins.
- Flame's Fury on self = +1 damage per damaging Power.

### Late (T7+)
- Firestorm L3 (4 Fire + 2 Air) — split damage across multiple Blighted lands.
- Firestorm L4 (7 Fire) — board-nuke in a Blighted-Presence land.

## Synergy Partners (Multiplayer)

- **Blight-removal partners** — cover Wildfire's structural weakness (Water Nourishes-level blight removal).
- **Defend partners** — free Wildfire to place aggressively without Ravage-mitigation duty.
- **Fear-rush partners** — Wildfire's high Fear per kill snowballs with Fear-rush kits.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Top-track rush.** Latentoctopus: terrible. Delays CP scaling by 1–2 turns.
2. **Full-Fuego T2 on already-blighted land.** Causes cascades; blight track flips prematurely.
3. **De-blighting right before a Reclaim turn.** gpope on BGG: *"you'll feel extremely stupid if you had one of those lands lined up and manageable and then removed blight before it got re-explored."*
4. **Reclaim without pre-positioning presence.** Wildfire's Reclaim is weak specifically because placement IS damage.
5. **Over-valuing the top track.** Bottom-track G3 is the correct default.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 bottom + Asphyxiating Smoke + 2E banked for Flash/Minor |
| 2    | G2 top + Threatening Flames push + Minor drafted           |
| 3    | G3 bottom + Flame's Fury + Minor; Firestorm L1 online      |
| 4    | G1 Reclaim + Gain; Firestorm L2 if 3 Plant drafted         |
| 5–6  | G3 bottom; Firestorm L2 + Regrows L1 cycle                 |
| 7+   | Firestorm L3/L4; late-game board-nuke                       |

## Source Notes

- **Mechanics**: `data/references/wiki/heart-of-the-wildfire.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus concepts + Opening 1 / Opening 2 pages.
- **BGG**: [jyonker13 thread 2313791](https://boardgamegeek.com/thread/2313791/openings-heart-wildfire).
