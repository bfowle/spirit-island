# Shroud of Silent Mist

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Rei's BGG Guide (thread 2722115)](https://boardgamegeek.com/thread/2722115/guide-shroud-of-silent-mist) + [latentoctopus Openings 1–2](https://latentoctopus.github.io/guide/mist-opening1/) + [BGG thread 2488254](https://boardgamegeek.com/thread/2488254/openings-shroud-silent-mist).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | High                                               |
| Play Difficulty       | 4 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick one growth per turn (bundled)         |
| Power summary (1–5)   | **Offense 4** · **Control 4** · **Fear 5** · Defense 2 · Utility 1 |
| Primary Elements      | **Air** (Suffocating Shroud + Swirling Haze all tiers) · **Moon** (Suffocating Shroud + Cool-and-Dark Energy) · **Water** (both innates) · Earth (late) |
| Special Rules         | Gather Power from the Cool and Dark (Minor gain w/o Fire = +1E) + Mists Shift and Flow (target-land Gather presence) + Slow and Silent Death (no heal + Fear drip on damaged Invaders) |
| Aspects               | None                                               |
| Rei's Guide           | **[Yes — thread 2722115](https://boardgamegeek.com/thread/2722115)** |
| latentoctopus         | [Openings 1–2](https://latentoctopus.github.io/guide/mist-opening1/) |
```

## Spirit Overview — Framing

Shroud is a **damaged-Invader farmer** — its win condition is *letting damaged Invaders live*, dripping Fear each turn via Slow and Silent Death until the killbox matures. Rei calls it *"a very, very complicated Spirit. I genuinely feel that it's so mechanically intense to play well, that many people are probably going to find it off-putting."*

**Wiki-printed playstyle note**:

> Damage + Fear generator with unusual mechanics. Damaged Invaders in your lands don't heal; Slow and Silent Death generates per-turn Fear on them. The spirit *wants* damage without destruction.

**Identity capture** (latentoctopus):

> Playing Mist is very much about flirting with disaster. It wants damaged Invaders to hang about in its lands for as long as it can manage, providing a slow drip of Fear until it finally gobbles them all up.

**Identity capture** (Rei):

> Shroud wants to sense not only the dangers of the event deck, but also think about all the ways each potential invader card can affect them while picking up bits and scraps of generosity from Fear Cards + other Spirits.

**Power spike**: **1/3cp** (Rei: *"1/4cp + reclaim is when you Transcend."*)

**Complexity signal**: High is correct. Shroud is widely regarded as one of the hardest JE spirits to pilot. Decision-load is high across all three phases: Event-card reads, Invader-card anticipation, multi-turn damaged-Invader farming, and Mists-Shift-and-Flow presence gather mid-turn.

## Starting Setup

> Put **2 Presence** on your starting board: **1 in the highest-numbered Wetland and 1 in the highest-numbered Mountain**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                           | Best when                                                   |
|--------|---------------------------------------------------|-------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card                       | Hand depleted + Minor-draft on Reclaim turn                 |
| **G2** | Add Presence (Range 0) + Add Presence (Range 0)   | Stack starting lands (2x R0)                                |
| **G3** | Gain 1 Power Card + Add Presence (Range 3, Mountain/Wetland) | Spread + card                                            |

G2's double-R0 is unusual — you're forced to stack presence on existing lands, not spread. Combines with Mists Shift and Flow to create movable presence clusters.

## Presence Tracks

- **Energy track** (5 slots): `energy0 → energy1 → water → energy2 → air`
  - 0E → 1E → +Water marker → 2E → +Air marker
- **Card-play track** (8 slots): `card1 → card2 → movepres → moonX → card3 → card4 → reclaim1 → card5`
  - 1 CP → 2 CP → Move Presence → Moon scaling → 3 CP → 4 CP → Reclaim 1 → 5 CP

**Starting income**: 0 Energy, 1 Card Play. Starting-Energy-0 is the hallmark Shroud problem — compounded by Cool-and-Dark Energy which only triggers on Minor gain without Fire.

## Core Mechanics & Special Rules

### Special Rule: Gather Power from the Cool and Dark

> Once a turn, when you Gain a Power Card without Fire, gain 1 Energy.

Every Minor (non-Fire) gain = +1 Energy. Rei explicitly: *drafting without Fire is preferable* — Fire-drafts forfeit the Energy and break the Cool-and-Dark economy.

### Special Rule: Mists Shift and Flow

> When targeting a land with a Power, you may Gather 1 of your Presence into the target or an adjacent land. This can enable you to meet Range and targeting requirements.

**Shroud's signature**: per-Power free presence-gather. This lets you count land-targeting Powers at turn-start as a **movement budget** — every card is also a free presence-move.

### Special Rule: Slow and Silent Death

> Invaders and Dahan in your lands don't heal Damage. During Time Passes: 1 Fear per damaged Invader in your lands.

The farm. Every damaged Invader alive at Time Passes = 1 Fear. Rei: **"schmoney" over kills** until 1/3cp.

### Innate: Suffocating Shroud

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | For each adjacent land with your Presence, 1 Damage to a different Invader. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 6 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | 1 Damage to each Invader. |

L2 is the *adjacent-presence scaler* — the core tactical lever. L4 is a board-wide 1-damage per Invader (late-game).

### Innate: Lost in the Swirling Haze

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 1 Dahan. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 2 Explorer/Dahan. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 2 Explorer/Dahan. |

Positioning innate. Push-Explorer keeps Invaders from land-exits so damaged Invaders stay *in* your lands for the Slow-and-Silent-Death Fear drip.

## Unique Cards (all 4, Wiki-verified)

### Dissolving Vapors
- **2 Energy · Slow · Range 0 · Any Land · Air, Water**
- *1 Fear. 1 Damage to each Invader. 1 Damage to each Dahan.*

The AOE. 2E Slow hits every Invader *and* every Dahan — usable on low-Dahan or Dahan-absent lands. Under Slow-and-Silent-Death, damaged-Invader land = next-turn Fear factory.

### Unnerving Pall
- **1 Energy · Fast · Range 0 · Land with 1+ Invaders · Moon, Air, Animal**
- *1 Fear. Up to 3 Damaged Invaders do not participate in Ravage.* **OR** *1 Fear. Defend 1 per Presence you have in target land (when this Power is used).*

Dual-mode Fast: *damaged-Invader skip-Ravage* (synergy with Dissolving Vapors setups) **OR** *presence-scaled Defend*. The Damaged-Invader skip is the signature Shroud tempo-preservation play.

### Flowing and Silent Forms Dart By
- **0 Energy · Fast · Range 0 · Any Land · Moon, Air, Water**
- *2 Fear if Invaders are present. When Presence in target land would be Destroyed, its owner may, if possible, instead Push that Presence. You may Gather 1 Presence/Sacred Site of another Spirit (with their permission).*

0E Fast Fear + Presence-destroy-cancel + ally-presence-gather. The Presence-cancel clause covers all spirits in the target land — massive ally-protection.

### The Fog Closes In
- **1 Energy · Slow · Range 0 · Any Land · Moon, Air, Water**
- *For each adjacent land with your Presence, 1 Damage to a different Invader. Push 2 Dahan.*

Adjacent-presence-scaled damage + Dahan push. Pairs with Suffocating Shroud L2 for *double* adjacent-scaling turns.

## Key Strategic Principles

1. **Damage, don't destroy — until 1/3cp.** Slow-and-Silent-Death pays Fear per damaged Invader at Time Passes.
2. **Air > Moon > Water > Earth.** Rei: *"the jump from the first tier of Suffocating Shroud to the second is the most dramatic"* — Air is the highest-leverage element.
3. **Avoid Fire drafts.** Cool-and-Dark gives +1E on non-Fire Minor gain.
4. **Mists Shift and Flow = movement budget.** Count land-targeting Powers at turn-start.
5. **Minors > Majors by default.** Rei: *"I HIGHLY recommend the Minor builds to get a better feel for Shroud before attempting Majors."*
6. **Shroud gathers ally Presence.** Flowing and Silent Forms lets allies move presence through Shroud's lands for otherwise-impossible placement.
7. **Don't over-greed Air.** Rei: *"The Air is very tempting, but it very much HALTS your scaling for multiple turns."*

## Possible Openings

### Shared starting state

- **2 Presence on board**: highest-numbered Wetland + highest-numbered Mountain on starting board.
- **4 Uniques in hand**: Dissolving Vapors (2E Slow, Air/Water), Unnerving Pall (1E Fast, Moon/Air/Animal), Flowing and Silent Forms Dart By (0E Fast, Moon/Air/Water), The Fog Closes In (1E Slow, Moon/Air/Water).
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — The Unleashed (Standard Minor) 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/mist-opening1/).

**T1 · Growth**: G2 top + G2 bottom (1E + 2 CP net).
**T1 · Play** (1E, 2 CP): **Flowing and Silent Forms Dart By + The Fog Closes In**.
- Elements: 1 Moon + 2 Air + 2 Water. Hits Suffocating Shroud L1 (1M+2A+1W) and Swirling Haze L1 (1A+2W).
- *Pause-point*: The Fog Closes In's adjacent-presence clause needs placements to have fired — verify target land has your-presence-adjacent.

**T2 · Growth**: G3 — Gain Minor + bottom-track presence.
**T2 · Play**: Unnerving Pall (and maybe drafted Minor).

**T3 · Growth**: Split:
- **Safe branch**: Reclaim (G1) + Gain Minor.
- **Aggressive branch**: G2 bottom × 2 + play Dissolving Vapors + Minor (tolerates some blight).

**T4 · Target state**: 3 plays pre-reclaim → 1/4cp + Reclaim (the "Transcend" state per Rei).

**Confidence**: 🟨 latentoctopus primary + Rei-aligned.

### Opening B — The Gambler (Minus-Tempo Minor) 🟥

From [latentoctopus Opening 2](https://latentoctopus.github.io/guide/mist-opening2/).

**T1 · Growth**: G3 — Gain Minor + top-track presence.
**T1 · Play**: Dissolving Vapors on a big/City land (takes some blight).

**T2 · Growth**: G2 top + G2 bottom.
**T2 · Play**: Unnerving Pall + Flowing Forms (Water unlock) **OR** Fog Closes In + Flowing Forms (Movement unlock).

**T3**: Branch:
- **Back to fast tempo**: G2 bottom × 2 + Unnerving Pall + flex Minor.
- **Stay Gambler**: G3 gain-Minor.

**T4**: G3 with 2E unlocked — gain Minor or Major.

Rei's warning: *"The Air is very tempting, but it very much HALTS your scaling for multiple turns."* Don't over-greed 3-Air thresholds at T2 cost.

### Opening C — Solo (jyonker13 BGG follow-up) 🟥

T2 adds Minor via Plays track. Flipping blight T2 is *"really sucky in solo"* — race for Fear rather than board removal.

### Opening Decision

- **Default Opening A** — balanced, Rei-and-latentoctopus aligned.
- **Opening B** when T1 Dissolving Vapors target presents itself cleanly.
- **Opening C** in solo with Fear-rush plan.

## Card Priority Ratings

### Uniques — Shroud-specific ranking

1. **Flowing and Silent Forms Dart By** — 0E Fast; Presence-destroy-cancel; ally-gather.
2. **Unnerving Pall** — damaged-Invader skip-Ravage is the signature tempo card.
3. **The Fog Closes In** — adjacent-presence-scaled damage.
4. **Dissolving Vapors** — the AOE; 2E Slow, setup-dependent.

### Top 10 Minor Draft Picks (Air > Moon > Water, **avoid Fire**)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Travel Unsuspected** | 1 | Fast | Air, Water | Air + Water dual-prime |
| 2 | **Absorb Essence** | 0 | Fast | Water | 0-cost Water |
| 3 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air |
| 4 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Air |
| 5 | **Call of the Deeps** | 1 | Slow | Water, Animal | Water-feeder |
| 6 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |
| 7 | **Gift of Power** | 1 | Fast | Moon | Moon utility |
| 8 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Air |
| 9 | **Call to Isolation** | 0 | Slow | Water, Animal | Isolate utility |
| 10 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Moon + Water (expensive) |

### Top 5 Major Draft Picks (advanced — "The Confounding")

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Dissolve Into Mist** | 4 | Fast | Air, Water | Air + Water double-prime |
| 2 | **Flow Like Water, Reach Like Air** | 2 | Fast | Sun, Air, Water | Cheap + Air-Water |
| 3 | **Weave Together a Fabric of Place** | 4 | Fast | Sun, Moon, Air, Water, Earth | Five-element flex |
| 4 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Late-game Water closer |
| 5 | **Veil of Shimmering Darkness** | 3 | Fast | Moon, Water | Moon + Water |

Rei: *"I HIGHLY recommend the Minor builds to get a better feel for Shroud before attempting Majors."*

### Cards to Avoid

| Card | Reason |
|------|--------|
| Fire Minors | Cool-and-Dark forfeits Energy on Fire gain |
| Single-land destroy-no-damage cards | Kills Shroud's farm before Slow-and-Silent pays off |
| Blight-adding Majors | Shroud's low Defense can't absorb |

## Adversary Matchup Matrix (Rei-documented)

| Adversary               | Opening | Rating | Matchup note (Rei level cap: 5)                              |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Rei: "Prussia 5" validated                                   |
| England                 | A       | ★★★★☆  | Rei: "England 5" validated                                   |
| France-Plantation       | A       | ★★★☆☆  | Rei: "France 4" validated                                    |
| Sweden                  | A       | ★★★☆☆  | Rei: "Sweden 5" validated                                    |
| Russia                  | A       | ★★★☆☆  | `[VERIFY]` — Rei avoids L6 generally                         |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

Rei: *"I avoid Level 6 because level 6 vastly warps & changes builds/strategies."*

## Board / Map Configuration

Rei: *"Shroud likes Range, Easy Movement between lands, & Favorable Dahan setup close to each other."*

| Board | Rating | Reason |
|-------|--------|--------|
| **C** | ★★★★★ | Good centralized Dahan |
| **D** | ★★★★★ | Best quad adjacencies ("cursed with double wetland coast") |
| **F** | ★★★★☆ | Strong starting placement |
| Others | ★★★☆☆ | Workable |

## Game-Phase Strategy

### Early (T1–3)
- G2 top + G2 bottom; Flowing Forms + Fog Closes In.
- Damage-without-destroy approach — Slow-and-Silent-Death begins paying T1.

### Mid (T4–6)
- 1/3cp Power Spike. Dissolving Vapors every reclaim cycle.
- Suffocating Shroud L2 (adjacent-presence scaling) online.
- Mists Shift and Flow used as movement budget for Range-0 targeting.

### Late (T7+)
- Suffocating Shroud L3/L4 board-wide damage.
- Major integration (Dissolve Into Mist, Weave Together).

## Synergy Partners (Multiplayer)

- **Damage-dealing partners** — let Shroud's lands stay "damaged-Invaders-present" for Fear drip.
- **Beast/Strife/Disease suppliers** — Event save-rate lifts from 40.7% → 48%+.
- **Presence-destruction-vulnerable partners** — Flowing Forms's destroy-cancel covers Serpent in Blighted lands, Fangs in non-Jungles.
- **Fear-reward-dependent partners** — benefit from Shroud's Fear drip.

## Common Mistakes

```admonish failure title="Rei's named mistakes"
1. **Greeding Air early.** "HALTS your scaling for multiple turns."
2. **Firing Suffocating Shroud to destroy Invaders when the farm was the plan.** "Schmoney over kills" until 1/3cp.
3. **Letting Dahan die in lands you needed for Fog Closes In push.**
4. **Forgetting Shroud can gather presence into lands adjacent to the target.** Mists Shift and Flow enables Range break-ups.
5. **Not counting land-targeting Powers as movement budget.**
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Flowing Forms + Fog Closes In; Swirling Haze L1      |
| 2    | G3 + Unnerving Pall; damaged-Invader farm starts          |
| 3    | Reclaim + Gain Minor; 2 plays                             |
| 4    | 1/4cp + Reclaim ("Transcend" per Rei)                     |
| 5–7  | Suffocating Shroud L2 + Slow-and-Silent Fear drip         |
| 8+   | L3/L4 board-wide damage; Major integration                |

## Source Notes

- **Mechanics**: `data/references/wiki/shroud-of-silent-mist.json` (Wiki-parsed 2026-04-23).
- **Primary strategy**: [Rei's BGG Guide (2722115)](https://boardgamegeek.com/thread/2722115/guide-shroud-of-silent-mist).
- **Openings**: latentoctopus Opening 1 + Opening 2.
- **BGG**: [thread 2488254](https://boardgamegeek.com/thread/2488254/openings-shroud-silent-mist).
