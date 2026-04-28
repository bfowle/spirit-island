# Dances Up Earthquakes

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from **Steve Haas's "Turn 4 Megaquake" BGG guide (thread 3107804)** — dominant canonical opening.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | Very High                                          |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | **Offense 5** · Control 2 · Fear 2 · Defense 3 · Utility 4 |
| Primary Elements      | **Earth** (both innates) · **Fire** (Earth Shudders) · Moon (Land Creaks) · Air (expensive) |
| Special Rules         | Begin a Dance of Decades (impend Power Cards by paying Energy; cards sit aside for future turns) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG Guide             | [Steve Haas T4 Megaquake — thread 3107804](https://boardgamegeek.com/thread/3107804) |
```

## Spirit Overview — Framing

Dances Up Earthquakes is the **tempo/impending spirit** — banks cards into a future turn via Begin a Dance of Decades, then detonates them simultaneously. The community meta is dominated by Steve Haas's **Turn 4 Megaquake opening** — so dominant one commenter admitted:

> Hardly feels like we're playing a game.

**Wiki-printed playstyle note**:

> Impend cards for a future crescendo. Quake tokens + Fire + Earth scaling for board-wide damage + fear.

**Complexity signal**: Very High is correct — impending-energy timing, quake-token management, and T4 threshold-stacking all need rehearsal.

## Starting Setup

> Put **1 Presence** on your starting board in the highest-numbered land with Dahan. You start with your **6 Unique Power Cards and 0 Energy**. Set the Quake Tokens nearby.

**6 Uniques in hand** (not 4) — unusually deep starting kit.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                          | Best when                                              |
|--------|------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Add Presence (R2) OR Gain Major without Forgetting     | Mid-game Reclaim                                       |
| **G2** | Gain 1 Power Card + Add Presence (R1)                            | Default opening move                                   |
| **G3** | Add Presence (R3) + Impend Energy + Reclaim-1                    | Energy-spike into impending                            |

## Presence Tracks

- **Energy**: `energy1+impendenergy1 → movepresence1 → energy2 → impend1 → energy3 → impendenergy2 → energy4any`
- **CP**: `card2 → gather1dahan1land → moonfire → impend1 → earthX → card3 → card4`

**Starting income**: 1 Energy (+ 1 Impending Energy — bonus for impended cards), 2 Card Plays.

## Core Mechanics & Special Rules

### Special Rule: Begin a Dance of Decades

> Whenever you would play a Power Card, you may instead pay any amount of Energy onto the card to make it an impending card, setting it aside out of play for use on a future turn. (It doesn't provide Elements. It's still your Power Card, so it can be forgotten while it's impending.)

**Impend = bank the card for later**. Energy paid sits on the card until resolution; multiple cards can impend in parallel.

### Innate: Land Creaks With Tension

- **Speed**: Fast · **Target**: You

| Level | Thresholds                              | Effect                                                                |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | If you have ≥1 impending card, add 1 Quake in one of your lands. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In one of your lands, Defend 1 per impending card (max 3). |
| 3     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | If ≥3 impending cards, add 1 Quake. |
| 4     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In one of your lands, Defend 1 per impending card (max 3). |

### Innate: Earth Shudders, Buildings Fall

- **Speed**: Slow · **Range**: 0 · **Target**: Quake

Text: X Damage (where X = Quake tokens), plus element-scaling Fear and fire damage. `[VERIFY exact threshold math]`. T4 fully-threshold state: **4 Fire + 5 Earth + 7 cards played = 2-damage-per-quake + Invader damage + 2 Fear**.

## Unique Cards (6, Wiki-verified names)

- Inspire a Winding Dance
- Resounding Footfalls Sow Dismay
- Radiating Tremors
- Rumblings Portend a Greater Quake
- Gift of Seismic Energy
- Exaltation of Echoed Steps

`[VERIFY exact costs/text — fetch parse incomplete; cross-check physical copy before opener play.]`

## Key Strategic Principles (Steve Haas's T4 Megaquake framework)

1. **T4 is the target turn** — Earth Shudders fully threshold with 7 cards played.
2. **Impend expensive Uniques on T1–T2.** Gift of Constancy (3-cost), Resounding Footfalls (3-cost) are the high-cost candidates.
3. **Two cheaper Majors > one expensive Major.** Steve: *"two cheaper majors is going to be better than one major costing 5 or 6, since you can so easily threshold majors."*
4. **Minor drafts must secure Fire+Earth.** Left innate demands both.
5. **Don't impend 0-cost Minors** — no Energy-discount benefit.
6. **Don't impend T3 and do nothing real** — still place Quakes + Defend in the interim.

## Possible Openings

### Shared starting state

- **1 Presence** on highest-Dahan land.
- **6 Uniques** in hand.
- **0 Energy**, 2 Card Plays.

### Opening A — Steve Haas T4 Megaquake 🟨🟩 (dominant)

**T1 · Growth**: G2 (top: +2 presence).
**T1 · Play**: Draft Minor (Fire or Earth priority). **Impend most expensive Uniques** (Gift of Constancy + Resounding Footfalls — both 3-cost). Pay 0 or small energy onto them.

**T2 · Growth**: G2.
**T2 · Play**: Draft second Minor. Impend **Radiating Tremors** (2-cost). Actually play **Rumblings of Discontent** (places quake + defends). Hits Land Creaks L1 → +1 Quake.

**T3 · Growth**: G2.
**T3 · Play**: Draft **Major**; Forget Rumblings. Impend two Minors, play Exaltation or Inspire. One Quake from innate.

**T4 · Growth**: G2.
**T4 · Play**: Draft **second Major**, Forget unique played T3. All impended cards crash in: **Gift + Resounding + Radiating + 2 Minors + both Majors**. **7+ cards in play, 5 Quake tokens, right innate fully thresholded.**

Steve: *"Clear your board on turn 4 straight up wins"* solo.

### Opening B — Slow-burn / multiplayer (940202 variant) 🟥

**T5 megaquake with ravage-skip looping** for multi-board or England.

### Opening C — G3 early (vs. France 6) 🟥

Switch to **G3** (+1 impending energy) to fire the quake *one turn earlier* vs France 6 when town-limit pressure is too high.

### Opening Decision

- **Default Opening A** (T4 Megaquake) — solo, Prussia, Scotland, Russia, HME.
- **Opening B** (T5 variant) — England, Sweden L5+, multi-board.
- **Opening C** (G3 early) — France 6 only.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Fire + Earth)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth prime |
| 2 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Moon + Earth |
| 3 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire + Moon |
| 4 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Multi-element Fire |
| 5 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth |
| 6 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |
| 7 | **Gift of Power** | 1 | Fast | Moon | Moon-feeder |
| 8 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | 0-cost Fire |
| 9 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |
| 10 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Earth + Moon |

### Top 5 Major Draft Picks

| # | Card | Cost | Why |
|---|------|------|-----|
| 1 | **Cast Down into the Briny Deep** | 5 | Fire + Earth threshold |
| 2 | **Pyroclastic Flow** | 6 | Fire + Earth multi-land |
| 3 | **Volcanic Eruption** | 8 | Late-game closer |
| 4 | **Infinite Vitality** | 4 | Fire + Plant |
| 5 | **Tigers Hunting** | 3 | Cheap Fire |

## Adversary Matchup Matrix (Steve Haas's solo L6)

| Adversary               | Rating | Note                                           |
|-------------------------|--------|------------------------------------------------|
| Brandenburg-Prussia     | ★★★★★  | 100% (Steve's data)                             |
| Scotland                | ★★★★★  | 100%                                           |
| Russia                  | ★★★★★  | 100%                                           |
| Habsburg Mining         | ★★★★★  | 100%                                           |
| Sweden                  | ★★★★☆  | 90–95%                                         |
| England                 | ★★★☆☆  | 80%+ (needs T5 variant)                         |
| France-Plantation       | ★★★☆☆  | 80% (town-limit risk; needs G3 early)          |
| Habsburg Livestock      | ★★★★☆  | 80%                                            |

## Board / Map Configuration

Board D + Scotland + Wetlands early is Steve's named edge-case (extra attention required).

## Game-Phase Strategy

### Early (T1–T3)
- Impend cadence per Opening A.
- Still place Quake tokens + Defend in interim.

### Mid (T4–T5)
- **T4 Megaquake**.
- Board-clear or significant damage.

### Late (T6+)
- Reclaim cycle with impending-again.
- Second Megaquake around T8.

## Synergy Partners (Multiplayer)

- **Consistent early-presence partners** (Ocean, Shadows, Lightning) while DUE ramps.
- **Set-piece-Turn-4 partners** (Behemoth, Stone).
- **Avoid** partners who need T1–T3 help (Shadows vs England).

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Impending every play** — still need Quake placement + Defend in interim.
2. **Impending 0-cost Minors** — no Energy-discount benefit.
3. **Drafting expensive T3 Major (5–6 cost)** — blocks second Major T4.
4. **Not securing Fire+Earth** on T1–T2 Minor drafts.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Impend Gift of Constancy + Resounding Footfalls; draft Fire/Earth Minor |
| 2    | G2 + Impend Radiating; play Rumblings; Land Creaks L1     |
| 3    | G2 + Draft Major; impend 2 Minors; play Exaltation/Inspire |
| 4    | **MEGAQUAKE**: 7+ cards + 5 Quakes + Earth Shudders full threshold |
| 5+   | Reclaim cycle; Quake Megaquake cadence every 4 turns      |

## Source Notes

- **Mechanics**: `data/references/wiki/dances-up-earthquakes.json` (Wiki-parsed 2026-04-23).
- **Primary guide**: [Steve Haas BGG 3107804](https://boardgamegeek.com/thread/3107804).
