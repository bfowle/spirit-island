# Hearth-Vigil

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from [BGG thread 3137164](https://boardgamegeek.com/thread/3137164). Thinnest NI coverage — physical-copy verify is especially important.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 3 · Control 1 · Fear 2 · **Defense 4** · **Utility 4** |
| Primary Elements      | **Sun** (both innates backbone) · **Animal** (Keep Watch) · Earth (Warn L2) · Air (Keep Watch L3) |
| Special Rules         | Rooted in the Community (Blight doesn't destroy Presence when Dahan present) + Fortify Heart and Hearth (Dahan +4 HP in your lands; Event/Blight immunity) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [thread 3137164](https://boardgamegeek.com/thread/3137164) |
```

## Spirit Overview — Framing

Hearth-Vigil is a **reactive Dahan-protector** — Dahan in your lands get +4 HP and Event/Blight-Card immunity. Not a Blight-stopper — very good *during* Ravages, poor against established Cities that aren't Ravaging.

**Wiki-printed playstyle note**:

> Dahan-protection spirit. Reactive defense; Dahan first-strike on Ravages.

**Identity in one line** (Steve496 BGG — different user):

> I've found it very strong (broken, really) to just loop Favors of Story and Season. That's all you need to hit the first tier of your innates, move a ton of Dahan around, and probably clear both ravaging lands without losing any Dahan.

**Complexity signal**: Moderate — easiest of the NI Incarna spirits.

## Starting Setup

> Put **3 Presence** on your starting board: 1 in the highest-numbered land with Dahan and 2 in the lowest-numbered land with at least 2 Dahan. Add **1 Dahan** in each of those lands (additional survivors of the Invaders' diseases).

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                   | Best when                                              |
|--------|-----------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Add Presence (R0)                               | Reclaim + density                                      |
| **G2** | Gain 1 Power Card + Add Presence (R3 Dahan-land)          | Card + Dahan-adjacent spread                           |
| **G3** | Add Presence (R2) + +3 Energy                             | Spread + energy                                        |

## Presence Tracks

- **Energy**: `gather1dahan1land → ...` (energy track is unusual; specifics `[VERIFY]`)
- **CP**: `energy0 → energy1sun → energy2 → energy3animal → energy4 → energy5sun`

**Starting income**: 0 Energy, 1 Card Play (with Dahan-gather on first track-reveal).

## Core Mechanics & Special Rules

### Special Rule: Rooted in the Community

> Blight added in your lands does not Destroy your Presence if Dahan are present. (Ravage Actions Destroy Dahan before added Blight destroys Presence and cascades.)

Dahan-presence pairing = Blight-cascade immunity. The identity-shaping rule.

### Special Rule: Fortify Heart and Hearth

> Dahan have +4 Health (each) while in your lands. Event and Blight Card Actions don't damage, destroy, or replace Dahan in your lands.

**Dahan tanks**. Event/Blight-Card immunity covers half the punishing events for free.

### Innate: Warn of Impending Conflict

- **Speed**: Fast · **Target**: Yourself

| Level | Thresholds                            | Effect                                                                |
|-------|---------------------------------------|------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In one of your lands, 1 Dahan deals Damage before Invaders during Ravages. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In that land, another Dahan deals Damage before Invaders. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | In that land, all Dahan deal Damage before Invaders. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Instead, all Dahan in all of your lands deal Damage before Invaders. |

Dahan first-strike escalating from 1 Dahan → all Dahan in 1 land → all Dahan in all lands.

### Innate: Keep Watch for New Incursions

- **Speed**: Fast · **Range**: 1 · **Target**: Any

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Gather up to 2 Dahan, from your lands only. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Once this turn after Invaders are added/moved into target land, 1 Damage per Dahan. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Repeat this Power. |

## Unique Cards (all 4, Wiki-verified names)

- **Coordinated Raid**
- **Surrounded by the Dahan**
- **Favors of Story and Season** (the signature reclaim-loop card per Steve496)
- **Call to Vigilance**

`[VERIFY exact costs/text.]`

## Key Strategic Principles

1. **Loop Favors of Story and Season** — Steve496's core strat. Clears both Ravages without losing Dahan.
2. **Rush the 1/Sun top-track spot.** kyren_vos: *"my most important takeaway was that I should have gone for the 1/Sun top track spot earlier. I waited to take it until turn 4... which meant I wasn't reliably hitting the second level of my left innate early on, resulting in taking some blight I shouldn't have."*
3. **Sun is the backbone.** Both innates key off Sun.
4. **Dahan preservation is non-negotiable.** Rooted only fires when Dahan are present.
5. **Watch for Loyal Guardian** — your anti-cascade relocation tool when Dahan leave a land.

## Possible Openings

### Shared starting state

- **3 Presence + 2 Dahan added** on starting board.
- **4 Uniques** in hand.
- **0 Energy**, 1 Card Play.

### Opening A — Community-default (Favors loop) 🟨

**T1 · Growth**: G2 top (+1 presence, +Gain Power Card).
**T1 · Play**: Draft Minor. Play **Favors of Story and Season** (reclaim-looped every turn if solo — can't target self in team games).

**T2 · Growth**: Move to bottom track — 1/Sun spot is critical (kyren_vos).

**T3 · Growth**: G2 bottom for continued presence placement into Ravage-threatened lands with Dahan. Play Favors again if reclaimed.
- **First Ravage under innate** should have Dahan striking first.

**T4+ · Growth**: G2 bottom for reclaim/energy. Draft Air and Animal for L3 of both innates.

### Opening Decision

- **Default Opening A** — Favors loop + Sun-rush.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Sun + Animal + Air + Earth)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Air + Animal |
| 2 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Animal |
| 3 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Sun + Animal |
| 4 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Sun + Animal |
| 5 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun + Air |
| 6 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 7 | **Bats Scout for Raids** | 1 | Fast | Moon, Air, Animal | Air + Animal |
| 8 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth + Animal |
| 9 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Sun |
| 10 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |

### Top 5 Major Draft Picks

| # | Card | Why |
|---|------|-----|
| 1 | **Settling into Hunting Grounds** | Earth + Animal + Plant |
| 2 | **Instruments of Their Own Ruin** | Animal + multi-land |
| 3 | **Tigers Hunting** | Cheap Animal |
| 4 | **Angry Bears** | Animal + Fear |
| 5 | **Vigor of the Breaking Dawn** | Sun + Plant |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                        |
|-------------------------|--------|-------------------------------------------------------------|
| Brandenburg-Prussia     | ★★★★☆  | Dahan first-strike destroys Prussia Build stacks             |
| Sweden                  | ★★★★☆  | `[VERIFY]`                                                  |
| France-Plantation       | ★★★★☆  | Dahan-attract synergy; Dahan tanks Ravages                   |
| **England**             | ★★☆☆☆  | **Weak** — established non-Ravaging Cities bypass Rooted     |
| **Habsburg Mining**     | ★★☆☆☆  | **Weak** — Blight-without-Ravaging                           |
| Russia                  | ★★★☆☆  | Pogrom events break Dahan pockets                             |
| Scotland                | ★★★☆☆  | `[VERIFY]`                                                  |
| Habsburg Livestock      | ★★★☆☆  | `[VERIFY]`                                                  |

kyren_vos's Prussia 5 loss report: *"events that add building HP neuter first-strike; events that force Dahan movement break your pockets."*

## Board / Map Configuration

Boards with **dense starting Dahan clusters** — standard Dahan-scalers prefer B, C, F.

## Game-Phase Strategy

### Early (T1–T3)
- Favors of Story and Season reclaim loop.
- Rush 1/Sun top-track spot (kyren_vos).
- Warn of Impending Conflict L1 online by T2–T3.

### Mid (T4–T6)
- Warn L2/L3 — multi-Dahan first-strike.
- Keep Watch L1 for Gather-2-Dahan per cast.

### Late (T7+)
- Warn L4 — all Dahan in all your lands first-strike.
- Keep Watch L3 — Repeat for double-damage.

## Synergy Partners (Multiplayer)

- **Dahan-scalers**: Sharp Fangs, River Surges, Lure, Finder.
- **Avoid**: Dahan-destroyer spirits (Bringer of Dreams, Volcano, Wounded Waters Roiling).

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Delaying the 1/Sun spot.** kyren_vos's named lesson.
2. **Abandoning lands** — Hearth-Vigil's protections are lands-with-your-presence-scoped.
3. **Forgetting Loyal Guardian.** Your anti-cascade relocation tool.
4. **Ignoring Event-Card Dahan immunity.** Fortify Heart eats half the punishing events for free.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 top + Favors + Minor; Sun track begun                  |
| 2    | Bottom-track 1/Sun spot critical                          |
| 3    | G2 + Favors reclaim + Minor; Warn L1 online               |
| 4+   | Warn L2 reliably; Keep Watch Gather-2-Dahan               |
| 7+   | Warn L4 board-wide first-strike                            |

## Source Notes

- **Mechanics**: `data/references/wiki/hearth-vigil.json` (Wiki-parsed 2026-04-23).
- **Openings**: [BGG thread 3137164](https://boardgamegeek.com/thread/3137164).
- **Coverage note**: Only 3 BGG posts in the analysis thread; latentoctopus + Dahan-Codex no content. **Thinnest coverage of the NI spirits — physical-copy playtesting is especially important.**
