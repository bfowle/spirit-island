# Starlight Seeks Its Form

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [jyonker13's BGG openings thread 2518429](https://boardgamegeek.com/thread/2518429/openings-starlight-seeks-its-form) + Carlo Gon's "Casino Starlight" (BGG 2624676) + Zubon's three-form framework.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Very High                                          |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn; **6 Presence tracks** (!) |
| Power summary (1–5)   | Offense 1 · Control 1 · Fear 1 · Defense 2 · Utility 2 |
| Primary Elements      | **All eight** — Starlight is element-flexible by design |
| Special Rules         | Growth Begets Growth (6 tracks; emptying a track unlocks a Growth choice permanently; the alternative is locked-out forever) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [jyonker13 thread 2518429](https://boardgamegeek.com/thread/2518429) + [Carlo Gon's Casino Starlight 2624676](https://boardgamegeek.com/thread/2624676) |
```

## Spirit Overview — Framing

Starlight is the **shape-shifter** — its identity emerges mid-game from which tracks you unlock and which innates your draft supports. jyonker13:

> Starlight is a Spirit that can truly be anything… its arsenal of 3-Element innates ensures that you'll find something to do with anything that you draft. That being said, your ability to pursue these various avenues is tempered by what comes your way, in a 'wand-chooses-the-wizard-mister-potter' kind of situation.

**jhaelen**:

> It's like a 'mystery box': you never know what kind of spirit you'll end up playing.

**Wiki-printed playstyle note**:

> Six Presence tracks. Emptying a track unlocks one of two Growth choices permanently; the other stays locked for the game.

**Complexity signal**: Very High. Not only does every game play differently, but each opening commits you to half the track-choices for the rest of the game. Planning Growth-choice selections is a 4-dimensional optimization.

## Starting Setup

> Put **1 Presence** on your starting board, in a land with Blight.

## Growth Options (growthtype: "one" — pick one per turn; 6-track model)

Starlight has **6 Presence tracks**, each with a Growth-unlock. Emptying a track picks one of two locked choices:

| Track | Choices (pick 1, other locked forever) |
|-------|------------------------------------------|
| **1** | Reclaim Half **OR** Reclaim Full         |
| **2** | +3 Energy **OR** +1 Card Play           |
| **3** | Gain Power Card **OR** Move Presence 1  |
| **4** | +1 Extra Card Play **OR** Move Presence 2 |
| **5** | (energy/element spaces — see track)      |
| **6** | (energy/element spaces — see track)      |

Each Presence add from a spaces-track unlocks its associated element.

## Presence Tracks (partial Wiki parse — structure)

- **Track 1 (energy)**: empty → growthreclaimhalf → growthdivider → growthgainpowercard → growthmovepresence1
- **Track 2 (CP)**: empty → growth3energy → growthdivider → growthplusonecardplay → growthmovepresence2

Full 6-track mapping `[VERIFY from physical spirit panel]`.

**Starting income**: 0 Energy, 1 Card Play. Grows dramatically via track-element spaces.

## Core Mechanics & Special Rules

### Special Rule: Growth Begets Growth

> You have 6 Presence tracks. (As usual, you may add Presence from any track.) 4 of the Presence tracks are next to rows of Growth choices: these choices start unavailable. Upon emptying a Growth track, pick one of its two Growth choices to be immediately available. The other stays unavailable for the rest of the game.

The spirit-defining rule — **each track pick is a game-long commitment**. Jonah's heuristic:

> **Tracks 1 and 3 = Reclaim-vs-Gain-Power**: pick opposite halves of each so you always have one of each mechanism.
> **Tracks 2 and 4 = Majors-vs-Plays**: left = money/damage with fewer cards; right = extra Play.

## Innates (5 total — Starlight has more innates than any other spirit)

### Air Moves, Earth Endures (Fast, Range 1)
| Threshold | Effect |
|-----------|--------|
| 3 Air     | Push up to 2 Explorer or 1 Town |
| 3 Earth   | Defend 5 |

### Fire Burns, Water Soothes (Slow, Range 1)
| Threshold | Effect |
|-----------|--------|
| 3 Fire    | 1 Fear. 2 Damage |
| 3 Water   | Remove 1 Blight |

### Wood Seeks Growth, Humans Seek Freedom (Slow, Range 2)
| Threshold | Effect |
|-----------|--------|
| 3 Plant   | Target Spirit with Presence in target land gains a Power Card |
| 3 Animal  | 1 Damage per Dahan OR Push up to 3 Dahan |

### Sidereal Guidance (Slow, Range 1)
| Threshold | Effect |
|-----------|--------|
| 2 Moon    | Gather up to 1 Explorer/Dahan |
| 3 Moon    | Instead, Gather up to 3 Explorer |

### Stars Blaze in the Daytime Sky (Slow, No Range, Yourself)
| Threshold | Effect |
|-----------|--------|
| 4 Sun     | 3 Fear. Gain 1 Energy. Reclaim up to 1 Power Card from play or discard. |

Five innates — one for each element-pair plus Moon. Starlight's draft commits you to *which* innate you'll fire each turn.

## Unique Cards (all 4, Wiki-verified)

### Gather the Scattered Light of Stars
- **[VERIFY]** — Rolling reclaim + Presence gather.

### Shape the Self Anew
- **[VERIFY]** — Form-adoption card; key T1 play.

### Peace of the Nighttime Sky
- **[VERIFY]** — Moon-scaling defense / fear-card manipulation.

### Boon of Reimagining
- **[VERIFY]** — Card-replacement for partner.

`[VERIFY all 4 unique costs/effects — Wiki fetch parse incomplete.]`

## Key Strategic Principles

1. **Form emerges from draft.** Decide form *after* seeing your opening hand (jyonker13).
2. **Moon is Starlight's signature** but never over-spec.
3. **Tracks 1+3 opposite halves = always have both Reclaim + Gain-Power access.** Tracks 2+4 similar for Majors-vs-Plays.
4. **Take pairs of Elements.** Don't force an innate the draft didn't offer.
5. **Hold off first track-element** until you've seen Powers and know what you're building.
6. **Pair-heavy Moon Minors → Plays form.** Dense-element Minor + easy-threshold Major → Majors form. Memory teammate → Memory-pair opener.

## Possible Openings (Zubon's three forms)

### Shared starting state

- **1 Presence** on a Blighted land.
- **4 Uniques in hand**: Gather the Scattered Light of Stars, Shape the Self Anew, Peace of the Nighttime Sky, Boon of Reimagining.
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — Majors Form (Zubon #1) 🟨

**T1 · Growth**: Track 1 — gain Major (forget Gather); Forget Peace for Boon.
**T2 · Growth**: Track 2 — uncover +3 Energy.
**T3+ · Growth**: Track 6 — elements.

Targets thresholded beatstick reliance.

### Opening B — Plays Form (Zubon #2) 🟨

**T1 · Growth**: Track 2 first — +1 CP.
**T1 · Play**: Play everything except Peace of the Nighttime Sky; reclaim the two power-generators.
**T2+**: Uncover Track 1 for Power-card option next.

Joy is stacking innates every turn.

### Opening C — No-reclaim Form (jhaelen commenter) 🟥

Uses **Gather the Scattered Light of Stars** as a rolling Reclaim; never takes a Reclaim Growth.

T1–T5 track-pick cadence: +1 Power → Move 1 → +3E → +1 Power + 1E twice → Majors T3–T4. Never touch Track 4.

### Opening D — jyonker13's Majors-lean canonical 🟨

**T1 · Growth**: Track 1 (+Minor, +Move 1, +1E).
**T1 · Play**: Boon of Reimagining + Shape the Self Anew (or strong Moon Minor if drafted; forget Shape for +3E).

**T2 · Growth**: Track 3 (+Major, +Move 1, +1E).
**T2 · Play**: **Peace of the Nighttime Sky** (forget for double-cast + Moon) + Moon Minor OR Shape. Forget Boon when gaining Major.

**T3 · Growth**: Track 3 again (+Major + element + energy).
**T3 · Play**: Major + Minor.

**T4**: Depends on Major combo — if panned out, add Track 5 (income) or Track 6 (element); Reclaim All; flog combo.

### Opening E — Memory-pair (Zubon #3) 🟥

Partner: Shifting Memory of Ages.

**T1 · Track 2 (+3E)**: play **Boon of Reimagining** on Memory (Memory's special rule lets them Discard instead of Forget) → Memory plays Boon of Ancient Memories on you → both spirits enter T2 with a Major.

### Opening Decision

- **Decide form after seeing T1 hand** — form is drafted, not pre-planned.
- **Opening A (Majors)** when opening Major pool is threshold-friendly.
- **Opening B (Plays)** when Moon-heavy Minors dropped.
- **Opening C (No-reclaim)** when Gather the Scattered Light seems to solve all your needs.
- **Opening E (Memory-pair)** with Shifting Memory partner.

## Card Priority Ratings

### Uniques — Starlight-specific ranking

`[VERIFY exact card ranking from community]`. Generally:

1. **Peace of the Nighttime Sky** — **do not Forget without Major replacement** per jyonker13.
2. **Shape the Self Anew** — T1 form-adoption.
3. **Gather the Scattered Light** — rolling reclaim.
4. **Boon of Reimagining** — partner-amp + Memory-pair enabler.

### Top 10 Minor Draft Picks (element pairs)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon+Air pair |
| 2 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire+Earth pair |
| 3 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Moon+Earth |
| 4 | **Drifting Into Stillness** | 1 | Slow | Moon, Plant | Moon+Plant |
| 5 | **Travel Unsuspected** | 1 | Fast | Air, Water | Air+Water |
| 6 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | Four-element 0-cost |
| 7 | **Purify the Land** | 0 | Slow | Moon, Water, Plant | Three-element 0-cost |
| 8 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost three-pair |
| 9 | **Bats Scout for Raids** | 1 | Fast | Moon, Air, Animal | Triple-pair |
| 10 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |

### Top 5 Major Draft Picks (by element synergy)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Unlock the Gates of Deepest Power** | 4 | Fast | All | Universal threshold-hit |
| 2 | **Bargains of Power and Protection** | 3 | Fast | Sun, Moon, Fire, Air | Four-element flex |
| 3 | **Weave Together a Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Five-element flex |
| 4 | **Terrifying Nightmares** | 4 | Fast | Moon, Air | Moon+Air |
| 5 | **Vigor of the Breaking Dawn** | 4 | Fast | Sun, Plant | Sun+Plant |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Single-element specialized Majors | Starlight thrives on pair-flexibility |
| 6+ cost Majors without Sun-Reclaim access | Hard to play twice without Stars Blaze innate |

## Adversary Matchup Matrix

Per jyonker13's innate-matchup notes:

| Adversary               | Innate fit | Rating | Note                                                     |
|-------------------------|-----------|--------|----------------------------------------------------------|
| France-Plantation       | Moon (Sidereal) | ★★★★★ | Sidereal Guidance *"absurd"* (jyonker13)             |
| Brandenburg-Prussia     | Moon      | ★★★★★ | Sidereal absurd                                           |
| Sweden                  | Moon      | ★★★★☆ | Sidereal absurd                                           |
| England                 | Water     | ★★★☆☆ | Moon *"mediocre"* — Water innate conditional            |
| Habsburg Mining         | Water     | ★★★☆☆ | *"Mediocre"* Moon innate                                 |
| Russia                  | Animal    | ★★★☆☆ | Dahan innate combo                                       |
| Scotland                | Water     | ★★★☆☆ | `[VERIFY]`                                               |
| Habsburg Livestock      | Moon/Water | ★★★☆☆ | `[VERIFY]`                                               |

## Board / Map Configuration

`[VERIFY]`. Starlight works on any board — form-flexibility makes terrain-adaptation the norm.

## Game-Phase Strategy

### Early (T1–3)
- Draft-driven form decision.
- T1: Track unlock (usually 1 or 2).
- T2: commit to form; Peace/Shape/Gather interactions.

### Mid (T4–6)
- Major integration (if Majors form).
- Chosen innates firing reliably.

### Late (T7+)
- Innate stacking; Stars Blaze Reclaim-self.
- Form-specific closes.

## Synergy Partners (Multiplayer)

- **Shifting Memory of Ages** — Memory-pair T1 (both spirits T2 Major).
- **Thunderspeaker** or **Stone** — "clearly defined roles" that Starlight supports post-hoc (jyonker13).
- **Vengeance**, **Green** — "malleable partners" that Starlight adapts around.

jyonker13's open question: *"I'm still unsure if Starlight is better with Spirits who have clearly defined roles… or those that are a little more malleable."*

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Forcing an innate the draft doesn't offer.**
2. **Taking Reclaim Half (Track 1) *and* neglecting Gain Power (Track 3).** Always have one of each reclaim method + one gain-power method.
3. **Taking Track 4 blindly.** Jonah only loves it as a mid-game damage/fastifier supplement, not an opener staple.
4. **Not forgetting Peace of the Nighttime Sky for its repeat.** "Once you get rolling with your chosen Major it'll be very hard to work it back in again."
5. **Locking out both reclaim options accidentally** — one commenter played a whole game without reclaim.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | Track 1 or 2 uncovered; form-commit in hand               |
| 2    | Track 3 uncovered (+Major/+Move); Peace/Shape deployed    |
| 3    | Track 3 or 5; elements online; innate reliably firing     |
| 4    | Reclaim All; combo flog                                    |
| 5+   | Form-specific late game                                    |

## Source Notes

- **Mechanics**: `data/references/wiki/starlight-seeks-its-form.json` (Wiki-parsed 2026-04-23).
- **Openings**: [jyonker13 BGG 2518429](https://boardgamegeek.com/thread/2518429/openings-starlight-seeks-its-form) + [Carlo Gon "Casino Starlight" 2624676](https://boardgamegeek.com/thread/2624676) + Zubon three-form framework.
