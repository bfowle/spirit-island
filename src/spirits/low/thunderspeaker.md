# Thunderspeaker

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Thunderspeaker Openings 1–3](https://latentoctopus.github.io/guide/thunderspeaker-opening1/) + [Phantaskippy's Wiki Guide](https://spiritislandwiki.com/index.php?title=Thunderspeaker/Phantaskippy%27s_Guide) + [BGG thread 1966212](https://boardgamegeek.com/thread/1966212/openings-thunderspeaker).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Moderate                                           |
| Play Difficulty       | 3 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn (bundled effects) |
| Power summary (1–5)   | **Offense 4** · **Control 5** · Fear 3 · Defense 2 · Utility 1 |
| Primary Elements      | **Sun** (Lead the Furious Assault) · **Fire** (Lead) · Air (Gather the Warriors + innate-fast) · Animal (Gather) |
| Special Rules         | Ally of the Dahan (Presence moves with Dahan) + Sworn to Victory (destroyed Dahan destroy Presence) |
| Aspects (JE)          | Tactician · Warrior `[VERIFY]`                     |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Openings 1–3](https://latentoctopus.github.io/guide/thunderspeaker-opening1/) |
| Phantaskippy's Guide  | [Wiki page](https://spiritislandwiki.com/index.php?title=Thunderspeaker/Phantaskippy%27s_Guide) |
```

## Spirit Overview — Framing

Thunderspeaker is the game's strongest **single-land "hammer"** — a mobile, Dahan-leveraging spirit that marches a concentrated war-band across the island rather than holding a home territory. Community consensus (BGG 375755): *"Thunderspeaker is probably the strongest 'Hammer' spirit in the game… delivers the strongest blow to single lands… faster than most other spirits and not dependent on RNG Major power draw."*

**Wiki-printed playstyle note**:

> Dahan-centric; requires Dahan density to do most of its work. Mobile — Ally of the Dahan moves Presence with Dahan for free.

**Identity capture** (Phantaskippy, Wiki):

> You are not built for holding a small territory, you are built to march into battle and drive out the invaders.

**The non-obvious trait**: Manifestation of Power and Glory scales with `presence × Dahan` — your real growth curve is measured not in presence count but in **how often you can fire Manifestation**. Every drafting and placement decision is downstream of that.

**Opening cost math**: Starting hand costs ~6 Energy total. Starting income is 1E. You cannot plush-start; T1 demands compressing plays or deferring one.

## Starting Setup

> Put **2 Presence** on your starting board: **1 in each of the 2 lands with the most Dahan**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                       | Best when                                              |
|--------|---------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + Gain 1 Power Card               | Hand depleted + draft two Minors in one turn           |
| **G2** | Thunder2 (add 2 Thunder tokens) + Thunder1 (add 1 Thunder)    | Dahan-positioning turn; Thunder is the "march" token   |
| **G3** | Add Presence (Range 1) + +4 Energy                            | Opening-track acceleration + big energy spike          |

G3 is the default opening move: **+4E is the biggest single-turn energy gain of any base-game spirit**, which is why Phantaskippy and latentoctopus both open with it.

## Presence Tracks

- **Energy track** (6 slots): `energy1 → air → energy2 → fire → sun → energy3`
  - 1E → +Air → 2E → +Fire → +Sun → 3E
- **Card-play track** (7 slots): `card1 → card2 → card2 → card3 → reclaim1 → card3 → card4`
  - 1 CP → 2 CP → 2 CP → 3 CP → Reclaim 1 → 3 CP → 4 CP

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Ally of the Dahan

> Your Presence may move with Dahan. (Whenever a Dahan moves from 1 of your lands to another land, you may move 1 Presence along with it.)

Free presence mobility tied to Dahan movement. Voice of Thunder's Push-4-Dahan moves your presence too — the core of the "march" identity.

### Special Rule: Sworn to Victory

> After a Ravage Action destroys 1 or more Dahan, for each Dahan Destroyed, Destroy 1 of your Presence within Range 1.

Every Dahan lost during Ravage = 1 Presence lost. Preserving Dahan is *existential* — Thunderspeaker games end when your presence runs out.

### Innate: Gather the Warriors

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power may be Fast. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Gather up to 1 Dahan per Air you have. Push up to 1 Dahan per Sun you have. |

Dahan herder. Animal 1 (trivial) + Air-scaling gather.

### Innate: Lead the Furious Assault

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power may be Fast. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Destroy 1 Town for every 2 Dahan in target land. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | Destroy 1 City for every 3 Dahan in target land. |

The hammer innate. L2 at 2 Sun + 1 Fire is reachable from starting hand (starters carry 3 Sun + 2 Fire — Lennert's thread math). L3 requires +2 Sun from track/Minors.

## Unique Cards (all 4, Wiki-verified)

### Manifestation of Power and Glory
- **3 Energy · Slow · Range 0 · Land with Dahan · Sun, Fire, Air**
- *1 Fear. Each Dahan deals Damage equal to the number of your Presence in target land.*

The big hammer. Scales with `Dahan × Presence`. Classic play: 2 Dahan, 2 Presence → 4 damage. 3E is steep — this card requires G3 energy or mid-game accumulation. Lennert (BGG): *"even on beginner, there's a 5/6 chance per board of a city ravage before the slow phase of turn 3"* — don't hold Manifestation; fire it early and often.

### Sudden Ambush
- **2 Energy · Fast · Range 1 · Any Land · Fire, Air, Animal**
- *You may Gather 1 Dahan. Each Dahan Destroys 1 Explorer.*

Explorer-killer scaling with Dahan. 2E Fast = pair with Manifestation on the same land for double-impact turn. Phantaskippy warns: high cost — forget after 1–2 reclaims.

### Voice of Thunder
- **0 Energy · Slow · Range 1 · Any Land · Sun, Air**
- *Push up to 4 Dahan.* **OR** *If Invaders are present, 2 Fear.*

0-cost Dahan-push. Combined with Ally of the Dahan, this is also up to 4 Presence-moves per play — the "march" primitive.

### Words of Warning
- **1 Energy · Fast · Range 1 · Land with Dahan · Sun, Air, Animal**
- *Defend 3. During Ravage, Dahan in target land deal Damage simultaneously with Invaders.*

Defend + Dahan-fight-first. The Dahan-simultaneous clause is huge — Dahan damage lands *before* Invader damage, potentially killing attackers before they harm Dahan/land.

## Key Strategic Principles

1. **Manifestation every turn from T3 onward is the goal** — your fear/damage ceiling.
2. **March, don't hold.** Ally of the Dahan makes your presence mobile — concentrate force, don't defend territory.
3. **Sun + Fire = load-bearing.** Lead the Furious Assault L2 opens the destroyer toolkit. 4 Air (both innates Fast) is valuable but not at cost-of-effect.
4. **Preserve Dahan aggressively.** Sworn to Victory punishes Dahan loss with Presence loss — Words of Warning + Defend-minor stacking is existential.
5. **Minors > Majors by default** (Phantaskippy). Focus on push/gather/defense minors that don't kill Dahan.
6. **Sudden Ambush is high-cost** — forget after 1–2 reclaims.

```admonish tip title="Phantaskippy's timing rule"
*"Making your innates fast isn't so great that you should sacrifice effects for it."* — don't over-draft Air at the cost of Sun+Fire damage thresholds.
```

## Possible Openings

### Shared starting state

- **2 Presence** on the 2 Dahan-densest starting-board lands.
- **4 Uniques in hand**: Manifestation of Power and Glory (3E Slow, Sun/Fire/Air), Sudden Ambush (2E Fast, Fire/Air/Animal), Voice of Thunder (0E Slow, Sun/Air), Words of Warning (1E Fast, Sun/Air/Animal).
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Hybrid / Minor-focused 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/thunderspeaker-opening1/).

**T1 · Growth**: G3 bottom (+4E, place presence from plays). Income: 1E (track) + 4E (G3) = 5E. 4 CP from starters.

**T1 · Play** (5E, 1 CP): **Sudden Ambush + Manifestation** (if vs. BP or strong Dahan-dense start).
- Alternative: Manifestation + Voice of Thunder (unlock Lead the Furious Assault L2 with 2 Sun + 1 Fire from starters).

**T2 · Growth**: G2 top. Income: 1E (track) + base = 2E.

**T2 · Play** (2E, 4 CP available): **Words of Warning + Voice of Thunder**.

**T3 · Growth**: G1 — Reclaim + Gain 1 Minor + Gain 1 Minor.

**T3 · Play** (income + remaining, 4 CP): Manifestation + Sudden Ambush repeat; start drafted Minor integration.

**T4 end state**:
- 12E accumulated (CE).
- 3 plays baseline.
- Both innates reliably active.
- **Lead the Furious Assault L2** online on most turns.

### Opening B — Full Bottom Track 🟨

From [Opening 2](https://latentoctopus.github.io/guide/thunderspeaker-opening2/).

**T1**: G3 bottom; play 2 starters.
**T2**: G3 bottom again (Words of Warning + 1 card).
**T3**: Reclaim + 2 minors.
**T4**: G2 bottom, 3 plays.

Rationale (latentoctopus): "Highest CP while still having the Energy to play all the Uniques." Pays in late Sun/Air element spikes.

### Opening C — Top Track 🟥

From [Opening 3](https://latentoctopus.github.io/guide/thunderspeaker-opening3/).

**T1**: G2 top; play Sudden Ambush *or* Voice of Thunder. 2E / 4 cards / 1 Air unlocked.
**T2**: G2 top again; Words of Warning. 4E, 1 Air/1 Fire/1 Sun.
**T3**: G3 bottom; Voice of Thunder or Ambush + Manifestation. 10E, 2 Air/1 Fire/1 Sun — right innate live.
**T4**: Reclaim + 2 minors. 12E, ~6 cards.

Latentoctopus: "Drop Sudden Ambush vs England (cost vs. value)."

### Opening Decision

- **Default Opening A** for Prussia, Sweden, France, Scotland, Russia.
- **Opening B** when card-volume matters more than energy spikes.
- **Opening C** top-track only when scenario/adversary demands late-turn Sun thresholds.

## Card Priority Ratings

### Uniques — Thunderspeaker-specific ranking

1. **Manifestation of Power and Glory** — the hammer; preserve reclaim cycles for this.
2. **Words of Warning** — Defend 3 + Dahan-first fight; existential for Dahan preservation.
3. **Voice of Thunder** — 0-cost Push 4 Dahan + Presence movement.
4. **Sudden Ambush** — situational; forget after 1–2 reclaims.

### Top 10 Minor Draft Picks (Sun/Fire/Air-prime)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Dahan-scaling Push |
| 2 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Sun + Animal |
| 3 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | Four-element flex 0-cost |
| 4 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Animal (Gather) |
| 5 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap Major-adjacent; Fire + Animal |
| 6 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun + Air |
| 7 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire-feeder for Lead |
| 8 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air flex |
| 9 | **Gift of Power** | 1 | Fast | Moon | Moon utility |
| 10 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Air-feeder |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Powerstorm** | 3 | Fast | Sun, Fire, Air | All three Thunderspeaker-prime elements |
| 2 | **Instruments of Their Own Ruin** | 3 | Fast | Fire, Air, Animal | Multi-land Dahan synergy |
| 3 | **Vigor of the Breaking Dawn** | 4 | Fast | Sun, Plant | Cheap multi-land offense |
| 4 | **Wrap in Wings of Sunlight** | 2 | Slow | Sun, Air | Sun + defensive utility |
| 5 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Board-wide Defend + Sun |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Dahan-destroying Minors | Sworn to Victory compounds loss |
| Blight-adding Majors | Dahan proximity = Presence risk |
| Single-land single-target Majors without Dahan scaling | Manifestation already fills this slot |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Sudden Ambush + Manifestation is the Prussia-opener sigil    |
| Sweden                  | A / B   | ★★★☆☆  | Build-spam punishes Dahan-dense lands                        |
| France-Plantation       | A       | ★★★☆☆  | Dahan attract — Sworn to Victory risk elevated               |
| Scotland                | A       | ★★★☆☆  | Coastal Cities are hard Dahan-counter targets                |
| Russia                  | A       | ★★☆☆☆  | Pogrom events destroy Dahan → destroy Presence               |
| England                 | B / C   | ★★★☆☆  | Drop Sudden Ambush; Opening C top-track                      |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Prefer boards with **dense starting Dahan placement** (natural 2+ Dahan lands at start). Phantaskippy's board ratings not published; heuristic is "follow the Dahan."

## Game-Phase Strategy

### Early (T1–3)
- G3 → G2 → G1 cadence. Manifestation on T1 if energy allows.
- Words of Warning every Ravage turn.

### Mid (T4–6)
- Lead the Furious Assault L2 reliable.
- Minor-draft integration; drift toward Powerstorm or Instruments Major.

### Late (T7+)
- L3 Lead (4 Sun + 3 Fire) for City-destruction.
- Manifestation multi-cast via Reclaim cycles.

## Synergy Partners (Multiplayer)

Per BGG "Favorite Hammer" opening — **River preferred** (energy boon enables Manifestation every turn from T3), **Ocean secondary**.

Also strong: **any energy-donor spirit** (Thunderspeaker is tight on energy through T2).

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Trying to stop every Build.** Thunderspeaker is a hammer, not a wall — concentrate force.
2. **Rooting out small threats instead of concentrating force.** 2 Dahan × 2 Presence = Manifestation for 4 damage; split into two 1-Presence lands and you get 2 × 1 = 2.
3. **Prioritizing innate-speed over positioning.** 4 Air is nice-to-have, not a priority draft.
4. **Sloppy Dahan preservation.** Sworn to Victory cascades.
5. **Treating Sudden Ambush as a keeper.** High cost, weed out after 1–2 reclaims.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 + Sudden Ambush + Manifestation (BP) or + Voice of Thunder |
| 2    | G2 top + Words of Warning + Voice of Thunder              |
| 3    | Reclaim + 2 Minors; Manifestation on best target          |
| 4    | G2 + 3 plays; Lead the Furious Assault L2 reliable        |
| 5–7  | Multi-Manifestation cycles via Reclaim + G1 bundle         |
| 8+   | Lead L3 vs. City-heavy boards                              |

## Source Notes

- **Mechanics**: `data/references/wiki/thunderspeaker.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus Opening 1/2/3 + Phantaskippy's Wiki guide.
- **BGG**: [thread 1966212](https://boardgamegeek.com/thread/1966212/openings-thunderspeaker).
