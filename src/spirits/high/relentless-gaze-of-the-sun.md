# Relentless Gaze of the Sun

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from three Phantaskippy-style Wiki subpages (Top Track, Bottom Track, Contextual Builds) + [BGG thread 3140073](https://boardgamegeek.com/thread/3140073). Best-covered of the NI spirits.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | **Offense 5** · Control 1 · Fear 3 · Defense 2 · Utility 3 |
| Primary Elements      | **Sun** (everything) · **Moon** (Harmonious Nature L1 — blight-to-destroy-presence) · Fire (Scorching) |
| Special Rules         | Relentless Punishment (Repeat a Power at escalating Energy cost if 3+ Presence in origin land) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| Wiki Guides           | [Top Track](https://spiritislandwiki.com/index.php?title=Relentless_Gaze_of_the_Sun/Top_Track_Build) + [Bottom Track](https://spiritislandwiki.com/index.php?title=Relentless_Gaze_of_the_Sun/Bottom_Track_Build) + [Contextual](https://spiritislandwiki.com/index.php?title=Relentless_Gaze_of_the_Sun/Contextual_Build) |
```

## Spirit Overview — Framing

Relentless Gaze is a **concentrated-repeat cannon** — uses triple-Presence stacks + high Energy to **repeat Power cards on the same land** (cost 0→1→3→6 cumulative). Unique in the game: the only innate that lets you **destroy your own Presence instead of adding Blight**, given 3 Sun + 1 Moon.

**Wiki-printed playstyle note**:

> Concentrated-stack repeat-damage spirit. Every turn the question is "what power am I repeating, and how many times?"

**Best-covered of NI** — three Phantaskippy-style Wiki subpages + 12 BGG posts + YouTube walkthroughs.

## Starting Setup

> Put **2 Presence and 1 Badlands** on your starting board, in the lowest-numbered Sands. You start with your **4 Unique Power Cards and 0 Energy**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                 | Best when                                              |
|--------|---------------------------------------------------------|--------------------------------------------------------|
| **G1** | Gaze (add presence + other effect — Gaze-specific bundle) | Signature spread                                       |
| **G2** | Reclaim + Add 3 Destroyed Presence Together             | Reset + rebuild                                        |
| **G3** | Gain 1 Power Card                                       | Card-gain                                              |
| **G4** | Gain Double Energy + Move 3 Presence Together           | Energy spike + reposition                              |

## Presence Tracks

- **Energy**: `energy1 → energy2sun → energy3fire → sun → energy4any → energy5 → (last slot empty)`
- **CP**: `card1 → card1 → card2 → sunX → card3 → reclaim1 → card4`

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Relentless Punishment

> After using a Power Card, if you had at least 3 Presence in the origin land, you may Repeat it any number of times on the same target land(s) (ignoring origin, Range, and target requirements) by paying:
> - the Energy cost of the Power, and
> - 1 Energy per previous use of the Power this turn each time you Repeat the Power.

Repeat-cascade with escalating cost: a 0-cost Power repeats at 0/1/3/6 Energy (0 initial + 0+1 first repeat + 0+2 second + 0+3 third = 0→1→3→6). 3+ Presence in origin land is the gate.

### Innate: Scorching Convergence

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds                                       | Effect                                                                    |
|-------|--------------------------------------------------|----------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun | Move all of your Presence from origin land directly to target land. 1 Damage to Town/City only. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | 3 Damage to Invaders. 3 Damage to Dahan. Add 1 Blight without cascading. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 3 Fear if this Power destroyed any Invaders. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Damage per remaining Presence of yours in target land. |

### Innate: Consider a Harmonious Nature

- **Speed**: Fast · **Target**: Yourself (passive)

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun | When your Powers would Add Blight, you may Destroy 1 Presence instead. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Your Powers don't damage or destroy Dahan. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Choose another Spirit. They add 1 Destroyed Presence to one of your lands. |
| 4     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Give up to 3 of your Energy to the chosen Spirit. |

L1 is critical — converts Scorching L2's Blight into Presence-destroy, feeding Gaze's Presence-recycling.

## Unique Cards (all 4, Wiki-verified names)

- **Focus the Sun's Rays**
- **Unbearable Gaze**
- **Blinding Glare**
- **Wither Bodies, Scar Stones**

`[VERIFY exact costs/text from physical copy.]`

## Key Strategic Principles

1. **Moon is critical, not optional.** 335907: *"my biggest pitfall originally was not grabbing power cards with moon element... Getting cards with moon is my top priority for the first few turns."*
2. **Top Track is default** (rchandra: *"My standard opener is G3 top then G2 bottom x2."*). Bottom Track for Russia.
3. **Repeat across multiple Powers, not one Power 3×.** 814764: *"repeating 3 minors each once is more efficient then repeating one of them 3 times."*
4. **Don't auto-take Scorching L2.** Dahan-kill + Blight-add is often worse than stopping at L1.
5. **Blight-adders are valuable** because you can convert Blight → Destroy-Presence at 3 Sun + 1 Moon.
6. **Support cards are worse than usual.** Gaze is plays-constrained.

## Possible Openings

### Shared starting state

- **2 Presence + 1 Badlands** on lowest-Sands.
- **4 Uniques** in hand.
- **0 Energy**, 1 Card Play.

### Opening A — Top Track 🟨 (default, rchandra)

**T1 · Growth**: G3 top (+1 presence, +1 card, +1 Energy).
**T1 · Play**: Draft Minor (**Moon priority — critical**). Play **Focus the Sun's Rays + left innate** to kill a Town, OR **Unbearable Gaze** to clear 2 lands.

**T2 · Growth**: G2 bottom.
**T2 · Play**: Draft another Minor. Play the other opener. Keep drafting Moon.

**T3 · Growth**: G2 bottom.
**T3 · Play**: Build toward 3 plays / 3 Energy.

**T4+ · Growth**: G1/G3 for repeat Energy. **Blinding Glare** becomes Ravage-skip + fear engine.

### Opening B — Bottom Track (vs. Russia) 🟥

Wiki: *"Place a presence from bottom track, draft a card, create a triple stack, then grow to 2 plays next turn."* Preserves left innate for later; Russia-specific (Beasts/Wilderness interaction).

### Opening C — Contextual triggers 🟥

Wiki third subpage — when to skew top T1:
- Multi-land solution available via **Unbearable Gaze**.
- Sands-explore letting you clear starting land with innate.
- Immediate skip-land needed vs. cascade threat.

### Opening Decision

- **Default Opening A** — balanced Top Track.
- **Opening B** vs Russia.
- **Opening C** in context-specific T1 situations.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Moon > Sun > Fire)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |
| 2 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Moon |
| 3 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Moon |
| 4 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air |
| 5 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun |
| 6 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Sun |
| 7 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Sun |
| 8 | **Gift of Power** | 1 | Fast | Moon | Moon utility |
| 9 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire-feeder |
| 10 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | 0-cost quadruple |

### Top 5 Major Draft Picks

Blight-adders + Sun/Fire/Air Majors.

| # | Card | Cost | Why |
|---|------|------|-----|
| 1 | **Focus the Land's Anguish** | 5 | Sun-scaling |
| 2 | **Poisoned Land** | 4 | Blight-add (convertible) |
| 3 | **Scour the Land** | 4 | Blight-add multi-land |
| 4 | **Pillar of Living Flame** | 4 | Fire + Sun |
| 5 | **Vigor of the Breaking Dawn** | 4 | Sun + multi-land |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Note                                                     |
|-------------------------|---------|--------|----------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Single-land focused                                      |
| Scotland                | A       | ★★★★☆  | Board-wipe wins (18 fear reports at L5)                  |
| England                 | A       | ★★★☆☆  | Coastal scaling                                           |
| Sweden                  | A       | ★★★☆☆  | `[VERIFY]`                                               |
| **France-Plantation**   | A       | ★★☆☆☆  | Building-limit adversaries block dump-and-skip            |
| Russia                  | B       | ★★★☆☆  | **Bottom Track** variant                                  |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                               |
| Habsburg Livestock      | A       | ★★★☆☆  | `[VERIFY]`                                               |

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Skipping Moon** — forces blight-add; eats your board.
2. **Replacing Withered Bodies T2 without stockpiling energy** (Top Track wiki).
3. **Over-repeating single powers** — spread across 3 Minors beats 1× three times.
4. **Auto-taking Scorching L2** — Dahan-kill + Blight is often worse than L1.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 top + Moon Minor + Focus the Sun's Rays / Unbearable  |
| 2    | G2 bottom + 2nd Minor + remaining opener                  |
| 3    | G2 bottom + 3 plays / 3 Energy target                     |
| 4+   | Blinding Glare + repeat cycles                             |

## Source Notes

- **Mechanics**: `data/references/wiki/relentless-gaze-of-the-sun.json` (Wiki-parsed 2026-04-23).
- **Wiki guides**: Top Track / Bottom Track / Contextual subpages.
- **BGG**: [thread 3140073](https://boardgamegeek.com/thread/3140073).
