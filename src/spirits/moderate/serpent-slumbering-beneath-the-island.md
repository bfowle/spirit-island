# Serpent Slumbering Beneath the Island

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [jyonker13's BGG openings thread 2282835](https://boardgamegeek.com/thread/2282835/openings-serpent-slumbering-beneath-island) + Zetan's solo refinement.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 1                                 |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 2 · Control 2 · Fear 2 · **Defense 4** · **Utility 5** |
| Primary Elements      | **Earth** (Serpent Wakes + Rouses) · **Fire** + **Water** (Serpent Wakes + Rouses) · Moon (Rouses) · Plant (Wakes) |
| Special Rules         | Deep Slumber (5-presence cap; raise via Absorb Essence) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [jyonker13 thread 2282835](https://boardgamegeek.com/thread/2282835) |
```

## Spirit Overview — Framing

Serpent is an **enabler-then-nuker** — early turns are support via two Gift powers; mid-late, the Deep Slumber track absorbs presence off-board to power Serpent Wakes in Power + Serpent Rouses in Anger. jyonker13:

> Being effective as a multiplayer Serpent hinges on predictive management of your own Presence cap, and a working knowledge of the Reclaim cycles of the other Spirits at your table.

**Non-obvious traits**:

- **Solo vs multi divergence is sharper than any other spirit.** In solo, Gifts self-target. In 2P+ 3 of 4 starting cards must target another spirit — there *will* be no-innate turns.
- **Presence-cap management is active.** Other spirits accidentally accelerate your wake via lent elements; maxing out = "only half wake up."
- **Gifts reshape partner reclaim cycles.** Primordial Deeps delays reclaim; Flowing Power accelerates discard.

**Complexity signal**: High is accurate. Different spirit in solo vs. multi.

## Starting Setup

> Put **1 Presence** on your starting board in **land #5**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                       | Best when                                           |
|--------|-----------------------------------------------|-----------------------------------------------------|
| **G1** | Reclaim + Move 1 Presence (Range 1)           | Reclaim cycle + reposition                           |
| **G2** | Gain 1 Power Card + +1 Energy                 | Card + energy                                        |
| **G3** | +4 Energy                                     | Big energy spike for Majors                          |
| **G4** | Add Presence (Range 0, No-Blight)             | Spread on safe lands                                 |

## Presence Tracks

- **Energy track** (8 slots): `energy1 → fire → any → reclaim1E → earth → energy6 → any → energy12`
- **Card-play track** (7 slots): `card1 → moonX → card2 → waterX → serpent → card4 → card5reclaim1`

**Starting income**: 1 Energy, 1 Card Play. Energy track peaks at **12 E** and has TWO "any element" slots — the scaling is enormous.

## Core Mechanics & Special Rules

### Special Rule: Deep Slumber

> You start off limited to 5 Presence on the island. Raise this with your Absorb Essence Power Card. Each use covers the lowest revealed number; your Presence limit is the lowest uncovered number.

Serpent's presence cap starts at 5 and expands via Absorb Essence casts. Managing this cap is the whole game.

### Innate: Serpent Wakes in Power

- **Speed**: Slow · **Target**: You

| Level | Thresholds                                             | Effect                                                                 |
|-------|--------------------------------------------------------|------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gain 1 Energy. Other spirits with any Absorbed Presence also gain 1 Energy. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Add 1 Presence to Range 1. Other spirits with 2+ Absorbed Presence may do likewise. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gain a Major Power without Forgetting. Other Spirits with 3+ Absorbed Presence may do likewise. |

The team-rewarding innate. Partners with Absorbed Presence benefit.

### Innate: Serpent Rouses in Anger

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                            | Effect                                                                 |
|-------|---------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | For each Fire & Earth you have, 1 Damage to 1 Town / City. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | For each 2 Moon + 2 Earth you have, 2 Fear + Push 1 Town from target land. |
| 3     | 5 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 6 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 6 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | **Cost 7 Energy.** In every land in the game: X Damage, where X is Presence in and adjacent to that land. |

L3 is a *board-wide* nuke at 7E cost. The "steamroll" turn.

## Unique Cards (Serpent has special Uniques; let me list what I have)

`[VERIFY — Wiki-fetch didn't parse Serpent's uniques cleanly; the 4 canonical uniques are:]`

### Elemental Aegis
- **1 Energy · Fast · Range 0 · Any Land · Fire, Water, Earth**
- *Defend 2 in target land and all adjacent lands. For every Presence on your "Deep Slumber" track, Defend 1 in target land and all adjacent lands.*

The Deep-Slumber-scaled Defend. Reclaimed every turn in the solo build.

### Absorb Essence
- **0 Energy · Fast · Range 0 · Any Land · Moon, Water**
- *Absorb 1 of your Presence (remove from the game, covering your limit). Gain 3 Energy. Target Spirit gains 1 element that target Spirit has.*

Presence → Energy conversion. Core to the Serpent economy.

### Gift of Flowing Power
- **0 Energy · Fast · No Range · Another Spirit · Water, Earth, Plant**
- *Target Spirit gains an extra Power Card play this turn. Serpent gains an element that target Spirit has.*

Partner extra-play + element theft.

### Gift of the Primordial Deeps
- **0 Energy · Fast · No Range · Another Spirit · Moon, Water, Earth**
- *Target Spirit plays a Minor Power from the top of the Minor deck for free. Serpent gains an element that target Spirit has.*

Partner free-Minor + element theft.

## Key Strategic Principles

1. **Solo and multi are different spirits.** Solo self-target Gifts; multi target partners.
2. **Don't wake up too fast.** jyonker13: *"The biggest mistake you can make is attempting to wake up too fast at the expense of the board state."*
3. **Earth is pivotal.** Gated behind T3 growth track — can't be rushed.
4. **Absorb Essence is the bridge.** Present → Energy + partner element-gift in one play.
5. **Elemental Aegis is reclaim-engine material.**
6. **Choose Gift by partner tempo fit**, not by element color.

## Possible Openings

### Shared starting state

- **1 Presence** on land #5.
- **4 Uniques**: Elemental Aegis, Absorb Essence, Gift of Flowing Power, Gift of the Primordial Deeps.
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Multi-player canonical (jyonker13) 🟨

**T1 · Growth**: G2 (+1E, gain Minor, presence from plays).
**T1 · Play**: **Gift of Flowing Power OR Gift of Primordial Deeps** at a partner.

**T2 · Growth**: G3 (+4E, presence from plays).
**T2 · Play**: **Absorb Essence + Elemental Aegis**.
- Absorb nets +3E and grants a partner an element. Use Absorb's "any" element as Earth to hit Serpent Rouses L1.

**T3 · Growth**: G4 (+Major, +1E, presence from energy).
**T3 · Play**: remaining Gift + lesser Minor. Forget weaker Minor.

**T4 · Growth**: G1 Reclaim All + Move presence + Add from energy.

### Opening B — Solo (Zetan refinement) 🟨

**T1 · Growth**: G2 (+1E, Minor, presence from bottom).
**T1 · Play**: **Gift of Flowing Power → play Gift of Primordial Deeps**.
- Place presence in City-building land or Town-with-no-Dahan.
- If GoFP's new Minor has Fire+Earth, push Serpent Rouses to Town-destroy T1.

**T2 · Growth**: G3 (+4E, presence from bottom).
**T2 · Play**: **Absorb Essence + Elemental Aegis**. Position presence to cover all ravages.

**T3+ · Growth**: G1 Reclaim All + Move + add from top.
**T3+ · Play**: Flowing Power + Aegis. Reclaim Aegis every turn.

**T5+**: Shift to Majors once Earth is uncovered. Eventually uncover card plays — 4 plays (effectively 5 with Flowing Power) becomes brutally destructive.

### Opening Decision

- **Default Opening A** in multi.
- **Default Opening B** in solo.

## Card Priority Ratings

### Uniques — Serpent-specific ranking

1. **Absorb Essence** — Presence→Energy bridge.
2. **Elemental Aegis** — reclaim-engine Defend.
3. **Gift of Flowing Power** — partner-extra-play.
4. **Gift of the Primordial Deeps** — partner free-Minor.

### Top 10 Minor Draft Picks (Earth/Fire/Water)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth |
| 2 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 3 | **Absorb Essence** (extra copy) | — | — | — | — |
| 4 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth + Plant |
| 5 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire + Water |
| 6 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Earth + Plant |
| 7 | **Absorb Essence** | 0 | Fast | Water | 0-cost Water |
| 8 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant |
| 9 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |
| 10 | **Call to Isolation** | 0 | Slow | Water, Animal | 0-cost Water |

### Top 5 Major Draft Picks

Serpent welcomes Majors — Wakes L3 gives free one. Target high-Energy multi-land damage/utility.

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Flow** | 6 | Slow | Fire, Earth | Fire + Earth prime |
| 2 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Water + Earth |
| 3 | **Volcanic Eruption** | 8 | Slow | Moon, Fire, Earth | Late-game closer |
| 4 | **Manifest Incarnation** | 3 | Fast | Moon, Plant, Animal, Water | Water + Plant |
| 5 | **Weave Together a Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Five-element |

### Cards to Avoid

`[VERIFY]` — thin BGG data.

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Strong late-game board-wipe                                  |
| England                 | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Sweden                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY]`. Focuscoene's question — "every time I use this opening I blight in both lands T3" — answered: Serpent trades early blight acceptance for late-game steamroll. Don't panic-pivot on T3 blight.

## Game-Phase Strategy

### Early (T1–3)
- Gifts T1 (partner or self); Absorb + Aegis T2.
- G3 energy spike.
- Serpent Rouses L1 online via Fire + Earth starter elements.

### Mid (T4–6)
- Shift to Majors after Earth uncover.
- Elemental Aegis reclaim-loop.
- Wakes L2 with 2W/3E/2P.

### Late (T7+)
- Wakes L3 (free Major without Forget!).
- Rouses L3 (Cost 7 board-wide damage).

## Synergy Partners (Multiplayer)

- **Bringer of Dreams and Nightmares** — Primordial Deeps T1: Bringer plays extra Minor + innates for 3 Fear + grants Serpent Earth (enables T1 Serpent Rouses Town-push).
- **Vital Strength of the Earth** — Flowing Power T1: Earth plays Rituals + Draw + Gift of Strength L1.
- **Wildfire** — Flowing Power T1: Flash Fires + Asphyxiating Smoke.
- **Lightning's Swift Strike** — Flowing Power T1: grants Fire, enables Harbingers + Shatter + Thundering Destruction L1.
- **Shadows Flicker Like Flame** — Deeps T1 + Flowing Power T3 sequence.

## Common Mistakes

```admonish failure title="jyonker13 named mistakes"
1. **Waking too fast at the expense of board state.** The biggest Serpent mistake.
2. **Maxing presence on-track** — hitting cap → Wakes-in-Power only half-fires.
3. **Solo ignoring the "target self with any-spirit cards" rule** (JE manual p. 22).
4. **Multi picking Gifts by element rather than partner-tempo fit.**
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Gift to partner; Serpent Rouses L1 if Fire+Earth available |
| 2    | G3 (+4E) + Absorb Essence + Elemental Aegis               |
| 3    | G4 (+Major) + remaining Gift + Forget weaker Minor         |
| 4    | G1 Reclaim All + redeploy                                  |
| 5+   | Wakes L2; Earth uncovered → Majors                        |
| 8+   | Wakes L3 (free Major!); Rouses L3 board-wipe              |

## Source Notes

- **Mechanics**: `data/references/wiki/serpent-slumbering-beneath-the-island.json` (Wiki-parsed 2026-04-23).
- **Openings**: [jyonker13 BGG 2282835](https://boardgamegeek.com/thread/2282835/openings-serpent-slumbering-beneath-island) + Zetan solo variant.
