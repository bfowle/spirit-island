# Ember-Eyed Behemoth

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Behemoth Openings 1–2](https://latentoctopus.github.io/guide/behemoth-opening1/). No Rei guide. No BGG-canonical opener thread.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "one" — pick one growth per turn (bundled)         |
| Power summary (1–5)   | **Offense 5** · Control 1 · Fear 1 · Defense 1 · **Utility 2** |
| Primary Elements      | **Fire** (everything — Special Rule + innate all tiers) · **Earth** (Smash L1/L2/L3/L4) · Plant (L2/L3/L4) |
| Special Rules         | The Behemoth Rises (Incarna push/move) + Unrelenting Strides (use Behemoth Rises twice on non-innate turns) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Opening 1](https://latentoctopus.github.io/guide/behemoth-opening1/) + [Opening 2](https://latentoctopus.github.io/guide/behemoth-opening2/) |
```

## Spirit Overview — Framing

Behemoth is a **Fire-locked Incarna spirit** — one of four Nature Incarnate spirits that field a named physical piece (the Behemoth) that walks the island delivering damage on the Smash, Stomp, and Flatten innate. Everything keys off Fire + Earth thresholds, and the Empowerment mechanic (G4, max once per game) is the central power spike.

**Wiki-printed playstyle note**:

> Aggro Incarna spirit — flexible placement, Fire-scaling damage, Incarna movement via sacred-site teleport. High offense, low defense.

**Identity capture** (jyonker13, BGG):

> Perhaps the most flexible and challenging Aggro Spirit the game has to offer.

**The Fire-lock**: Behemoth's opening lives or dies on hitting **2 Fire + 1 Earth** by end of T1 to unlock Smash L1 (2 Damage). Without a Fire Minor drafted or a Fire-Unique played, the innate lies dormant and Behemoth falls a turn behind.

**Complexity signal**: Moderate — 4 Uniques, 1 innate with 4 tiers, 2 Special Rules. But the Incarna-movement planning adds decision-load: every sacred site is a Behemoth-teleport target, so sacred-site placement is a damage-targeting decision.

## Starting Setup

> Put **2 Presence and {Behemoth, Unempowered side up}** in the **highest-numbered Wetland on your starting board that is adjacent to any Jungle**. You start with your 4 Unique Power Cards.

Behemoth the incarna piece sits on that starting Wetland. Empowerment happens later via G4.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                                  | Best when                                                          |
|--------|--------------------------------------------------------------------------|--------------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card                                              | Hand depleted; mid-game Reclaim cycle                              |
| **G2** | Add Presence (R3 Jungle-presence path) + Add Presence (Range 0)          | Spread reach to Jungle-accessible lands                            |
| **G3** | Gain 1 Power Card + Add Presence (Range 1) + +3 Energy + discard-fire-power | Default opening — biggest energy jump + card draw (cost: discard Fire card) |
| **G4** | **MAX 1/GAME** — Reclaim-all-Fire + Empower Behemoth + Move Behemoth   | The single biggest power spike in Behemoth's arc                    |

**G4 is the transformational turn.** Empowers Behemoth (unlocks the "you may Repeat Smash once each turn" clause) + reclaims all Fire cards + moves Behemoth to any sacred site.

G3's "discard-fire-power" cost: you must discard one Fire-element Power card from hand to take the +3E. Plan Fire-card selection T1–T2 around this constraint.

## Presence Tracks

- **Energy track** (7 slots): `energy0 → energy1 → energy2fire → energy3 → earth → energy4plant → energy5fire`
  - 0E → 1E → 2E + Fire marker → 3E → +Earth marker → 4E + Plant marker → 5E + Fire marker
- **Card-play track** (6 slots): `card1 → card2 → card2 → card3 → fireX → card4`
  - 1 CP → 2 CP → 2 CP → 3 CP → Fire scaling → 4 CP

**Starting income**: 0 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: The Behemoth Rises

> You have an Incarna (Behemoth). Once per turn, during the Spirit, Fast, or Slow phase, you may either:
> - Push Behemoth; or
> - Add or Move Behemoth to any of your Sacred Sites on the island.

**Free sacred-site teleport per turn.** Every sacred site is a valid Smash target as long as you can move Behemoth there.

### Special Rule: Unrelenting Strides

> On any turn that you don't use Innate Powers, you may use The Behemoth Rises an additional time.

On innate-idle turns, double the Incarna movement. Useful for T1–T2 pre-positioning before the Fire-element threshold is online.

### Innate: Smash, Stomp, and Flatten

- **Speed**: Slow · **Range**: — · **Target**: Behemoth (the incarna land)

Text: *If Behemoth is Empowered, you may Repeat this Power once each turn.*

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | 2 Damage. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Damage. Push 1 Dahan. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Fear. 1 Damage. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 2 Damage. 2 Damage to Dahan. |

Cumulative: L1+L2+L3+L4 = 2+1+1+2 = 6 Damage + 1 Fear + Push Dahan + 2 Dahan damage. Empowered Repeat = 12 Damage per round of play. The Repeat clause makes G4 the single biggest turn of a Behemoth game.

## Unique Cards (all 4, Wiki-verified)

### Blazing Intimidation
- **2 Energy · Fast · Range 1 · Any Land · Fire, Plant, Animal**
- *1 Fear. Push up to 2 Invaders.*

Fast push + Fear; Fire + Plant for innate thresholds. Repositioning tool before Smash cycles.

### Surging Lahar
- **2 Energy · Slow · Range 1 from your Sacred Site · Any Land · Fire, Water, Earth**
- *Add 1 Badlands. 1 Damage to each Invader per Badlands in target land.*

Badlands-adder + Badlands-scaling damage. 1 Badlands → 1 damage per Invader; 2 Badlands → 2 damage per Invader. Pairs with Behemoth's Smash-scaling pattern.

### Exaltation of Grasping Roots
- **0 Energy · Slow · No Range · Any Spirit · Moon, Fire, Earth, Plant**
- *Target Spirit gains 1 Power Card. Forget a Power Card. If it's a Major Power, target Spirit may pay 2 Energy instead of Forgetting a Power Card.*

Ally-gift; 0E Slow. Self-target gives +1 card gain (useful on card-light turns). Elements cover Earth + Plant for innate.

### Terrifying Rampage
- **1 Energy · Fast · Range 1 · Any Land · Moon, Fire, Earth**
- *Push 1 Explorer/Town. 1 Fear per Presence of yours adjacent to target land.*

Presence-scaled Fear. Fire + Earth for innate thresholds. Note: *"useless T1 — no triggers and no Energy"* (latentoctopus warning).

## Key Strategic Principles

1. **Fire > Earth > Plant in draft priority.** Every Smash tier needs Fire first.
2. **Don't play Terrifying Rampage T1.** Useless without adjacent presence.
3. **G4 is the power spike.** Plan the Empowerment turn on T3–T4 — that's the spike moment.
4. **Behemoth teleports to sacred sites.** Sacred-site placement = Smash target selection.
5. **Bottom track first.** Skip top-track Energy — 3 CP is the goal.
6. **Keep Fire+Plant cards in hand T2** for next-turn L2 threshold.
7. **Hold Major in hand** on non-Fire turns for the right Fire-innate-unlock turn.

## Possible Openings

### Shared starting state

- **2 Presence + Behemoth (Unempowered)** on starting Wetland adjacent to Jungle.
- **4 Uniques in hand**: Blazing Intimidation (2E Fast, Fire/Plant/Animal), Surging Lahar (2E Slow, Fire/Water/Earth), Exaltation of Grasping Roots (0E Slow, Moon/Fire/Earth/Plant), Terrifying Rampage (1E Fast, Moon/Fire/Earth).
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — Standard (Early Badlands + Innate lock-on) 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/behemoth-opening1/).

**T1 · Growth**: G3 bottom — Gain 1 Minor (Fire/Earth priority) + Presence (R1) + +3E (discard a Fire card as cost).
- Income: 3E from G3.

**T1 · Play** (3E, 1 CP — but 4 plays avail from track): unlock Smash L1 via **2 Fire + 1 Earth**.
- **Preferred**: Surging Lahar + Exaltation of Grasping Roots (if a valid Lahar target exists). Covers Fire+Earth+Moon+Plant.
- **Alternative**: Blazing Intimidation + Exaltation of Grasping Roots. Covers Fire+Earth+Plant.
- *Pause-point*: Terrifying Rampage is useless T1; don't pick.

**T2 · Growth**: G2 top — Add Presence (R3 Jungle-presence) + R0.
**T2 · Play**: 2 cards — may unlock Innate L2 (3 Fire + 1 Earth + 1 Plant) depending on drafts.

**T3 · Growth**: **G4** — Reclaim-all-Fire + Empower Behemoth + Move Behemoth.
- **This is the transformational turn.** Every Fire card returns to hand; Behemoth becomes Empowered (Smash now Repeats).

**T3 · Play** (income + G4-reclaimed hand): Play 2 cards. Smash fires once + Repeats once = 4 Damage baseline.

**T4 · Growth**: G2 bottom — reach **3 CP** for future turns.
- Even without a Fire-element card this turn, prioritize 3 CP.

**T4 end state** (audit):
- Behemoth Empowered; Smash firing twice per turn.
- 3 CP baseline.
- Fire + Plant cards staged in hand for L2+ thresholds.

**Confidence**: 🟨 latentoctopus primary.

### Opening B — Early Major 🟥

From [Opening 2](https://latentoctopus.github.io/guide/behemoth-opening2/).

**T1 · Growth**: G2 top — Add Presence; start 2E / 4 cards / 4 plays.
**T1 · Play**: Surging Lahar (unlocks Innate via Fire+Earth).

**T2 · Growth**: G3 bottom (discard/gain Minor or Major).
**T2 · Play**: 2 cards. *"If the Major does not have Fire element, keep it in hand and play the other two cards to unlock your Innate level 2."*

**T3 · Branch**:
- **Fire-Major route**: G4, Empower Behemoth, play 2.
- **Non-Fire-Major route**: G3 top, gain Major from discard, play both Majors.

**T4 · Branch**:
- Fire route: G3 top.
- Non-Fire route: G4 Empower.

The Fire-element check on the drafted Major is the central pivot.

### Opening Decision

- **Default Opening A** — reliable across most matchups.
- **Opening B** when the Major draft lands with Fire or Moon elements; riskier.

## Card Priority Ratings

### Uniques — Behemoth-specific ranking

1. **Surging Lahar** — Badlands engine + Fire/Earth threshold.
2. **Exaltation of Grasping Roots** — 0E flex + ally-gift + four primary elements.
3. **Blazing Intimidation** — Fast push; Fire/Plant.
4. **Terrifying Rampage** — situational; presence-scaled Fear.

### Top 10 Minor Draft Picks (Fire > Earth > Plant)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth double-prime |
| 2 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 3 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost flex |
| 4 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire-feeder |
| 5 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth + Plant |
| 6 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire + Moon |
| 7 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant |
| 8 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Plant |
| 9 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Earth + Plant |
| 10 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Plant |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Flow** | 6 | Slow | Fire, Earth | Fire-Earth multi-land |
| 2 | **Volcanic Eruption** | 8 | Slow | Moon, Fire, Earth | Late-game closer |
| 3 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap Fire |
| 4 | **Infinite Vitality** | 4 | Fast | Fire, Water, Plant | Fire + Plant |
| 5 | **Pillar of Living Flame** | 4 | Fast | Sun, Fire, Air | Fire-element flex |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Non-Fire 2-cost Minors | Can't amortize through G3 discard-Fire |
| Defense-only Powers | Behemoth kit is aggro-only |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Direct damage fits Prussia ravages                           |
| Sweden                  | A       | ★★★☆☆  | Build-spam can outpace Behemoth clears                       |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| England                 | B       | ★★☆☆☆  | Damage-resistant; Major route only                           |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY — latentoctopus no per-board ratings]`. Heuristic: boards with **Mountains/Sands adjacent to Wetlands** reward Lahar's Badlands scaling. Board E/F have good terrain mix.

## Game-Phase Strategy

### Early (T1–3)
- T1: Surging Lahar / Blazing Intimidation to unlock Smash L1.
- T2: stack Fire + Plant cards in hand for L2.
- T3: G4 Empowerment — the power spike.

### Mid (T4–6)
- Empowered Smash firing twice per turn.
- 3 CP baseline.
- Major integration (Opening B variant).

### Late (T7+)
- Smash L2/L3/L4 cycle (6+ damage per round).
- Volcanic/Pyroclastic Major for board-wide closes.

## Synergy Partners (Multiplayer)

- **Badlands/Blight-tolerant partners** — Wildfire, Volcano.
- **Defense partners** — cover Behemoth's ravage lands.
- **Fire-element donors** — Lightning, Wildfire (via repeat of Fire Minors).

## Common Mistakes

```admonish failure title="latentoctopus named mistakes"
1. **Playing Terrifying Rampage T1.** Useless without triggers and no Energy.
2. **Dropping a non-Fire Major T2 instead of holding it to unlock Innate L2 via starting Uniques.**
3. **Skipping Incarna empowerment on T3.** The G4 option IS the build curve.
4. **Top-track chase.** 3 CP is the real priority.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 bottom + Surging Lahar/Exaltation; Smash L1 live       |
| 2    | G2 top + 2 cards; L2 reachable with Plant draft           |
| 3    | **G4 — Empower Behemoth**; Smash Repeat active            |
| 4    | G2 bottom → 3 CP; Empowered Smash firing twice per turn   |
| 5–7  | Smash L2/L3 cycle; Major integration (if Opening B)       |
| 8+   | Smash L4 + Major board-wide closes                         |

## Source Notes

- **Mechanics**: `data/references/wiki/ember-eyed-behemoth.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus Opening 1 + Opening 2.
- **Note**: No Rei guide; no BGG-canonical opener thread. Strategic framing thin compared to older spirits.
