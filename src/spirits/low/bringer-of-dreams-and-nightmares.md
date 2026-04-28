# Bringer of Dreams and Nightmares

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Phantaskippy's Wiki Guide](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares/Phantaskippy%27s_Guide) (canonical) + [BGG openings thread 1971193](https://boardgamegeek.com/thread/1971193/openings-bringer-dreams-and-nightmares).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | High                                               |
| Play Difficulty       | 3 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 1 · Control 2 · **Fear 5** · Defense 2 · Utility 2 |
| Primary Elements      | **Moon** (everything) · **Air** (Night Terrors + Spirits May Yet Dream) · Animal (Night Terrors L2/L3) |
| Special Rules         | To Dream a Thousand Deaths (can't destroy Invaders — converts would-be-destroys to 0/2/5 Fear + Push) |
| Aspects (JE)          | Enticing · Violence `[VERIFY]`                     |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| Phantaskippy's Guide  | [Wiki page](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares/Phantaskippy%27s_Guide) |
```

## Spirit Overview — Framing

Bringer is the **terror-race driver** — cannot destroy Invaders (except its own presence), and exists to hasten easier win conditions. Phantaskippy:

> Your job on a team is to drive the terror level advance. Make up for your lack of destruction by hastening the easier win conditions.

**Wiki-printed playstyle note**:

> Fear-generation engine that replaces would-be-destruction damage with Fear + Push. Dahan are the only source of Destroy on Bringer's board — protect them.

**Identity in one line**: fear-engine + terror-race + fear-card-preview via Spirits May Yet Dream.

**Complexity signal**: High is correct. The damage-to-fear conversion rule inverts normal combat intuition; Dahan-preservation is existential; power-ordering matters (Call on Midnight's Dreams Forget interaction).

## Starting Setup

> Put **2 Presence** on your starting board in the **highest-numbered Sands**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                             | Best when                                              |
|--------|-----------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card                         | Reclaim cycle                                          |
| **G2** | Reclaim 1 + Add Presence (Range 0)                  | Partial-reclaim + density                              |
| **G3** | Gain 1 Power Card + Add Presence (Range 1)          | Card + spread                                          |
| **G4** | Night Effect + +2 Energy                            | Energy spike                                           |

## Presence Tracks

- **Energy track** (7 slots): `energy2 → air → energy3 → moon → energy4 → any → energy5`
- **Card-play track** (6 slots): `card2 → card2 → card2 → card3 → card3 → any`

**Starting income**: 2 Energy, 2 Card Plays. Unusual — Bringer starts with *2 plays* but generates no damage, so those plays feed Fear instead.

## Core Mechanics & Special Rules

### Special Rule: To Dream a Thousand Deaths

> Your Powers never cause Damage, nor can they Destroy anything other than your own Presence. When your Powers would Destroy (or deal enough Damage to Destroy) Explorer/Town/City, instead generate 0/2/5 Fear. The Power Pushes all Explorer/Town it would Destroy.

**Every damage instance converts to Fear + Push**. A Power that would destroy a City generates 5 Fear and pushes; one that would destroy a Town generates 2 Fear and pushes; one that would destroy an Explorer generates 0 Fear and pushes (no fear on Explorer, because Destroy-Explorer is 0 Fear baseline).

### Innate: Spirits May Yet Dream

- **Speed**: Fast · **Target**: Any Spirit

| Level | Thresholds                              | Effect                                                |
|-------|-----------------------------------------|--------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Turn any face-down Fear Card face-up. (Earned/resolved normally, but players can see what's coming.) |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | Target Spirit gains an element that they have at least 1 of. |

Fear-card-preview + ally element-gift. L1 is Bringer's **information weapon** — in 4P it can turn a panic-turn into a planned one.

### Innate: Night Terrors

- **Speed**: Fast · **Range**: 0 · **Target**: Invaders

| Level | Thresholds                              | Effect                |
|-------|-----------------------------------------|------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +1 Fear. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +1 Fear. |

## Unique Cards (all 4, Wiki-verified)

### Call on Midnight's Dream
- **0 Energy · Fast · Range 0 · Any Land · Moon, Animal**
- *If target land has Dahan, gain a Major Power. If you Forget this Power, gain Energy equal to Dahan and you may play the Major Power immediately, paying its cost.* **OR** *If Invaders are present, 2 Fear.*

**The Major-gain engine** — tied to Dahan count. Forgetting it converts Dahan to Energy for immediate Major-cast.

### Dread Apparitions
- **2 Energy · Fast · Range 1 · Land with 1+ Invaders · Moon, Air**
- *When Powers generate Fear in target land, Defend 1 per Fear. 1 Fear.*

Fear-into-Defend converter. Night Terrors generates Fear → Defend stacks. 1 Fear generated in target = Defend 1.

### Dreams of the Dahan
- **0 Energy · Fast · Range 2 · Any Land · Moon, Air**
- *Gather up to 2 Dahan.* **OR** *If target land has Towns/Cities, 1 Fear for each Dahan, to a maximum of 3 Fear.*

0-cost Dahan-Gather or Dahan-scaled Fear.

### Predatory Nightmares
- **2 Energy · Slow · Range 1 from Sacred Site · Land with 1+ Invaders · Moon, Fire, Earth, Animal**
- *2 Damage. Push up to 2 Dahan.*

**"2 Damage"** — but via To Dream: 2 damage to Explorer/Town converts to Fear + Push. A Town at 1 Dmg becomes 2 Fear + Push.

## Key Strategic Principles

1. **Moon is king.** Reach Moon on the energy track by T3 — the opener's only hard requirement.
2. **Major-heavy drafts.** Phantaskippy: good energy generation + easy Major access + reclaim-1 = Major-friendly. Inverts usual advice.
3. **1-damage Minors are trash.** Bringer doesn't accumulate damage across Powers.
4. **Dahan preservation is existential.** "Don't lose your Dahan, even if you have to slow your advance."
5. **Preview Fear cards strategically.** Spirits May Yet Dream at 4P = "extra action per player/board" value.
6. **Call on Midnight's Dreams — spend elements BEFORE forgetting.** Forgetting Call removes its element contribution.
7. **Reclaim-1 cadence** matters — Bringer has no 2-presence growth option, so reclaim-alls cost tempo.

## Possible Openings

### Shared starting state

- **2 Presence** on highest Sands.
- **4 Uniques in hand**: Call on Midnight's Dream (0E Fast, Moon/Animal), Dread Apparitions (2E Fast, Moon/Air), Dreams of the Dahan (0E Fast, Moon/Air), Predatory Nightmares (2E Slow, Moon/Fire/Earth/Animal).
- **Starting income**: 2 Energy, 2 Card Plays.

### Opening A — Phantaskippy's canonical 🟨 (default)

**Goal**: unlock **Moon on energy track by T3** (T4 acceptable with poor card luck). G3 every turn until Moon is unlocked.

**T1 · Growth**: G3 (+1 Minor + Presence top-track, place in land with Invaders/Dahan).
- Top-track placement reveals **Air**.

**T1 · Play** (2E, 2 CP): **Dreams of the Dahan + Predatory Nightmares**.
- Triggers Spirits May Yet Dream L1 (2 Moon + 2 Air via starters) + Night Terrors L1 (1 Moon + 1 Air).
- Predatory Nightmares: push a Town about to Ravage; ideally leave 1 Dahan to kill the leftover Explorer via counter.
- Dreams of the Dahan: gather Dahan for T2.

**T2 · Growth**: G3 (+2E, +1 Presence top, place in land with Invaders/Dahan).

**T2 · Play** (4E, 2 CP): **Dread Apparitions + Call on Midnight's Dreams**.
- **Resolution order matters:**
  1. Dread Apparitions at Range 0 (not Range 1 — you want Range 0 for Defend stacking).
  2. Innate Night Terrors for +2 Fear in that land → Defend 3.
  3. Spirits May Yet Dream (flip another Fear card).
  4. Call on Midnight's Dreams LAST to learn a Major (if you Forget Call to cast the Major, you lose Call's elements — so spend elements first).
- With 2 Dahan gathered for Energy, ~5E is typically available — *"enough to cast 86% of Major powers."*

**T3 · Growth**:
- If T2 Major was castable and good: **Reclaim 1** (G2) + cast it again alongside a Minor.
- If both have Moon: top-track placement brings a 3rd Moon → Spirits May Yet Dream L2 (element-gift).
- If T2 Major wasn't castable: take presence+card growth again for another Major shot.

**T4 target**: Moon unlocked on energy track, one cast-Major in hand, 2+ face-up Fear cards known, Dahan still alive.

**Past T4**: Phantaskippy prefers the "any" element space over the 3rd card play — widens Major-threshold access.

### Opening Decision

- **Default Opening A**. Phantaskippy flagged Bringer as *the exception* to his "two good starts" rule — Bringer has one clearly better line.

## Card Priority Ratings

### Uniques — Bringer-specific ranking

1. **Dread Apparitions** — Fear-into-Defend stacking.
2. **Call on Midnight's Dream** — Major-gain engine.
3. **Dreams of the Dahan** — 0-cost Gather + Dahan-scaled Fear.
4. **Predatory Nightmares** — Fast-pushing Fear generation.

### Top 10 Minor Draft Picks (Moon/Air, **elements > damage**)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Dahan-scaling + Moon (Phantaskippy's top pick) |
| 2 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Moon + Air prime |
| 3 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air |
| 4 | **Predatory Nightmares** (extra copy) | 0 | Fast | Moon, Animal | Moon + Animal |
| 5 | **Gift of Power** | 1 | Fast | Moon | Moon-feeder |
| 6 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Moon |
| 7 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Moon + Animal |
| 8 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Moon + Fire |
| 9 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Moon + utility |
| 10 | **Bats Scout for Raids** | 1 | Fast | Moon, Air, Animal | Three-prime |

### Top 5 Major Draft Picks (Phantaskippy's perfect-pick list)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Volcanic Eruption** | 8 | Slow | Moon, Fire, Earth | Phantaskippy: *"who cares if you can't meet the threshold"* — damage becomes Fear |
| 2 | **Cast Down into the Briny Deep** | 5 | Slow | Sun, Moon, Water | Destroy → massive Fear via To Dream |
| 3 | **Poisoned Land** | 4 | Slow | Moon, Fire, Water | Destroy-scale → Fear-scale |
| 4 | **Entrancing Apparitions** | — | — | — | (if Major exists in pool — check Wiki) |
| 5 | **Utter a Curse of Dread and Bone** | 3 | Slow | Sun, Moon, Air | Fear-scaling Major |

### Cards to Avoid

| Card | Reason |
|------|--------|
| 1-damage Minors | Useless — Bringer doesn't accumulate damage per Power |
| Fear-heavy Minors | Low priority — Bringer generates Fear natively |
| Single-land destroy-no-damage cards | Destroy → Fear conversion only works when *Damage* converts |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| England (L3–L4)         | A       | ★★★★★  | Terror-2 win adversary — Bringer is elite                    |
| Sweden                  | A       | ★★★★☆  | Terror-2 win friendly                                        |
| Brandenburg-Prussia     | A       | ★★★☆☆  | Build-pressure fights Fear tempo                             |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Scotland                | A       | ★★☆☆☆  | Coastal Cities hardens against Terror win                    |
| **Habsburg Mining**     | A       | ★★☆☆☆  | **Weak** — hardens against fear win; needs real destroyer    |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Boards with **dense starting Dahan** are best — Dahan preservation is non-negotiable. `[VERIFY per-board ratings]`.

## Game-Phase Strategy

### Early (T1–3)
- G3 every turn until Moon unlocked.
- Spirits May Yet Dream L1 on every turn with 2 Moon + 2 Air.
- Predatory Nightmares + Dreams of the Dahan cycle.

### Mid (T4–6)
- Major-integration (Volcanic Eruption, Cast Down, Poisoned Land).
- Call on Midnight's Dreams → Forget for instant Major-cast.

### Late (T7+)
- Terror 2/3 race.
- Dread Apparitions board-wide Defend stack.

## Synergy Partners (Multiplayer)

- **Damage-dealers who wound** — VSE (Rituals half-kills), River (push+damage), Lightning (city softener). Bringer pushes wounded Towns *without resetting* damage.
- **Presence-granters** — Green's Gift of Proliferation, Keeper — accelerate Bringer's slow growth.
- **Avoid doubling on pure fear** — two Bringers / Bringer+Shadows-Madness = redundant tempo.

## Common Mistakes

```admonish failure title="Phantaskippy named mistakes"
1. **Letting Dahan die early.** "Don't lose your Dahan, even if you have to slow your advance to reclaim Dread Apparitions."
2. **Flipping Fear cards you can't use.** Check if the card resolves this turn — sometimes skip flipping for T+1 planning.
3. **Drafting 1-damage Minors.** Wasted on Bringer.
4. **Casting Call on Midnight's Dreams before spending its elements.** Forgetting Call removes its element contribution.
5. **Going wide too fast without Moon.** Unlocking Moon T3 is the opener's only hard requirement.
6. **Over-using Reclaim All.** Bringer has no 2-presence growth + no reclaim-with-presence option, so Reclaim-Alls compound tempo loss.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 + Dreams of the Dahan + Predatory Nightmares; Night Terrors L1 + Spirits L1 |
| 2    | G3 + Dread Apparitions + Call on Midnight; Major gained + castable |
| 3    | G2 Reclaim-1 + cast Major; Moon unlocks                   |
| 4    | G3 + Minor; Moon on track + Major in hand                  |
| 5–7  | Major-cycle via Reclaim-1; Fear-race to T2/T3             |
| 8+   | Major board-wide closes                                    |

## Source Notes

- **Mechanics**: `data/references/wiki/bringer-of-dreams-and-nightmares.json` (Wiki-parsed 2026-04-23).
- **Primary strategy**: [Phantaskippy's Wiki Guide](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares/Phantaskippy%27s_Guide).
- **BGG openings**: [thread 1971193](https://boardgamegeek.com/thread/1971193/openings-bringer-dreams-and-nightmares).
