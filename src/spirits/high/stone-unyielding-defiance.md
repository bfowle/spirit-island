# Stone's Unyielding Defiance

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [jyonker13's BGG openings thread 2512536](https://boardgamegeek.com/thread/2512536/openings-stones-unyielding-defiance) + aviator13 / Sh0rtz variants.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Moderate (Wiki) / High (BGG playtest)              |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | **Offense 4** · Control 2 · Fear 1 · **Defense 5** · Utility 2 |
| Primary Elements      | **Earth** (everything) · Sun (Bulwark L2/L3) · Plant (Bulwark L3) |
| Special Rules         | Bestow the Endurance of Bedrock (Blight doesn't cascade unless it outnumbers Presence) + Deep Layers Exposed (first +1 CP uncover → gain Minor) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed (Kalen Noreth / ZenithPriest videos)    |
| BGG                   | [jyonker13 thread 2512536](https://boardgamegeek.com/thread/2512536) |
```

## Spirit Overview — Framing

Stone is the **Blight-nonchalant lockdown defender** — plants presence in doomed lands and converts Card-Blight to Box-Blight via Special Rule. jyonker13's brutally concise framing:

> You're Stone, and you don't give a shit about Blight.

> The game plan is relatively linear in execution: you want to occupy lands that are Ravaging, so as to add their Blight from the box instead of the card, and possibly rebound some of that damage upon the Invaders who dealt it.

**Wiki-printed playstyle note**:

> Lockdown defender that punishes invaders for ravaging on top of it.

**Complexity signal**: BGG discussion leans Moderate/High — execution is linear but board-coverage planning is dense.

## Starting Setup

> Put **2 Presence** on your starting board: **1 in the lowest-numbered Mountain without Dahan; 1 in an adjacent land that has Blight** (if possible) or is Sands (if not).

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                  | Best when                                              |
|--------|----------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 2 Earth markers + Stone (special token)   | Reclaim turn + Earth element scaling                   |
| **G2** | Add Presence (Range 2) + +3 Energy                       | Spread + energy                                        |
| **G3** | Gain 1 Power Card + Add Presence (Range 1)               | Card + close placement                                 |

## Presence Tracks

- **Energy track** (7 slots): `energy2 → energy3 → card+1stone → energy4 → card+1stone → energy6 → card+1stone`
- **Card-play track** (6 slots): `card1 → earthX → earthX → earthreclaimX → earthanyX → card2earth`

**Starting income**: 2 Energy, 1 Card Play. Energy-rich; the +1 CP / Stone slots give free Minors on each uncover.

## Core Mechanics & Special Rules

### Special Rule: Bestow the Endurance of Bedrock

> When Blight is added to one of your lands, unless the Blight then outnumbers your Presence, it does not cascade or destroy Presence (yours or others').

**Blight from Box, not Card.** As long as Presence > Blight in the land, Stone's lands are Blight-cascade-immune.

### Special Rule: Deep Layers Exposed to the Surface

> The first time you uncover each of your "+1 Card Play" Presence spaces, gain a Minor Power.

Free Minor on each +1 CP uncover.

### Innate: Hold the Island Fast with a Bulwark of Will

- **Speed**: Fast · **Target**: You (passive)

| Level | Thresholds                              | Effect                                                                      |
|-------|-----------------------------------------|------------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | When Blight is added to one of your lands, you may pay 2 Energy per Blight to take it from the box instead of the Blight card. |
| 2     | 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | The cost is 1 Energy instead of 2. |
| 3     | 6 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | When an Event or Blight card directly destroys Presence (yours or others'), you may prevent any number of Presence from being destroyed. |

Blight-mitigation economy; L3 prevents Event-card Presence-destruction.

### Innate: Let Them Break Themselves Against the Stone

- **Speed**: Fast · **Range**: 0 · **Target**: Any

`[VERIFY thresholds — fetch parse incomplete]`. Scales with Ravage damage.

## Unique Cards (all 4, Wiki-verified)

### Jagged Shards Push from the Earth
- **[VERIFY]** — Add 1 Badlands + Push 2 Dahan.

### Plows Shatter on Rocky Ground
- **[VERIFY]** — Damage each Town/City then Push 1 Town OR Destroy 1 Town.
- **Important (mat9h correction)**: *"damage each town/city"* fires *regardless* of which OR branch you pick. It is *not* a mode switch.

### Scarred and Stony Land
- **[VERIFY]** — Badlands-adder + defensive utility.

### Stubborn Solidity
- **[VERIFY]** — Defend / Blight-interaction.

## Key Strategic Principles

1. **Stone doesn't care about Blight.** The Special Rule is the whole identity — plant presence in doomed lands.
2. **Earth is everything.** Gain 2 via bottom Growth + starters.
3. **Spread to all terrains** by T2 (janes85 → jyonker13): *"Adding presence to land types you aren't already in will allow you to negate Blight regardless of which Stage II cards come up."*
4. **Distribute Badlands liberally.** jyonker13: *"A good investment if you're not sure which of your Starters to play in a given turn."*
5. **Minors > Majors.** Sh0rtz: *"Indomitable Claim and Unrelenting Growth are your best hits by far."*
6. **Gain 2 Earth ≠ bankable.** Elements clear at Time Passes.

## Possible Openings

### Shared starting state

- **2 Presence** on Mountain + adjacent Blight-land or Sands.
- **4 Uniques in hand**: Jagged Shards, Plows Shatter, Scarred and Stony Land, Stubborn Solidity.
- **Starting income**: 2 Energy, 1 Card Play.

### Opening A — jyonker13 mixed-track 🟨 (default)

**T1 · Growth**: G2 (+3E, +Presence top).
**T1 · Play**: **Jagged Shards Push from the Earth** (1 Badlands + push 2 Dahan) **OR** **Plows Shatter on Rocky Ground** (damage each Town/City + push 1 Town).
- Place range-extended presence into a land type you don't yet occupy — ideally one Ravaging next turn.

**T2 · Growth**: G2 (+3E, +Presence top).
**T2 · Play**: **Stubborn Solidity + whichever of Jagged/Plows you didn't use**.
- The +1 Play spot ideally uncovers Earth+Sun on the Minor gain; otherwise take best-effect Minor.

**T3 · Growth**: G1 Reclaim All + Add Presence (bottom) + 2 Earth.
- Abuse Bulwark of Will L1 to mitigate extra Ravage for 2E.

### Opening B — aviator13 bottom-track (Earth-rush) 🟥

Full bottom track builds Earth fastest → powers Let Them Break Themselves L2 one turn earlier. aviator13:

> Stone isn't the most helpful for other people, but you can absolutely lock your own board down and always have it covered. And you can cover it using only your starting cards, so it's not reliant on getting elements from the deck or certain types of abilities.

Sh0rtz prefers Opening A's 2nd-Minor T2 for Earth+Sun consistency.

### Opening Decision

- **Default Opening A** in most games.
- **Opening B** in solo / when board-lock is the singular plan.

## Card Priority Ratings

### Uniques — Stone-specific ranking

1. **Stubborn Solidity** — Defend + Blight interaction.
2. **Plows Shatter on Rocky Ground** — multi-Town damage.
3. **Jagged Shards Push from the Earth** — Badlands + push.
4. **Scarred and Stony Land** — Badlands flex.

### Top 10 Minor Draft Picks (Earth + Sun)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Indomitable Claim** | — | — | Earth | Sh0rtz: *"best hits by far"* |
| 2 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | Sh0rtz: *"best hits by far"* |
| 3 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Earth |
| 4 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 5 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Earth |
| 6 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire + Earth |
| 7 | **Drift Down to Rest** | 0 | Slow | Sun, Air, Plant | 0-cost Sun |
| 8 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Sun |
| 9 | **Gift of Constancy** | 0 | Fast | Sun, Plant, Animal | 0-cost Sun |
| 10 | **Sap Their Strength** | 1 | Fast | Moon, Earth, Plant | Earth + Plant |

### Top 5 Major Draft Picks (only when home-board locked)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Forest of Living Obsidian** | — | Slow | Earth, Plant | Earth + Plant |
| 2 | **Land Thrashes in Anger** | 4 | Slow | Fire, Earth | Fire + Earth |
| 3 | **Infinite Vitality** | 4 | Fast | Fire, Water, Plant | Fire + Plant |
| 4 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Earth + late-game |
| 5 | **Vigor of the Breaking Dawn** | 4 | Fast | Sun, Plant | Sun + Plant |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Air-heavy Minors | Off-axis |
| Single-land damage without Earth scaling | Stone's damage is Earth-elemental |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| **Brandenburg-Prussia** | A       | ★★★★★  | Heavy-damage Ravage — Stone's perfect matchup               |
| **England (high-level)**| A       | ★★★★☆  | Coastal City cascade absorption                              |
| Sweden                  | A       | ★★★★☆  | Plantation chains compress into Stone's tanking              |
| France-Plantation       | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Russia                  | A       | ★★★☆☆  | Dahan-destroy events stress                                  |
| Scotland                | A       | ★★★☆☆  | Multi-adversary hardness                                     |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY]`. Stone's Mountain-start + adjacent-Blight placement makes Mountain-dense boards preferable.

## Game-Phase Strategy

### Early (T1–3)
- Spread to all 4 terrains by T2 (janes85 heuristic).
- Badlands distributed liberally.
- Bulwark L1 online by T2 (2 Earth).

### Mid (T4–6)
- Bulwark L2 (4 Earth) — 1E per Blight absorb.
- Reclaim cycles with Earth-element stacking.

### Late (T7+)
- Bulwark L3 (6 Earth + 1 Plant) prevents Event-destroy-Presence.
- Board-lock stable; Major integration optional.

## Synergy Partners (Multiplayer)

- **Offensive partners** who can't afford to tank — Fractured Days, Volcano, Ocean.
- **Partners who need defensive cover** — gift Bulwark zones.
- **Avoid**: other spirits needing your turn for offense (Stone can't carry damage).

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Plows Shatter OR branch misread.** "Damage each Town/City" fires regardless.
2. **Adding all T1/T2 presence to lands you already occupy.** Spread across terrains for Stage-II coverage.
3. **Treating "gain 2 Stone/Earth" as bankable.** Elements clear at Time Passes.
4. **Going full bottom track in multi-spirit games.** Loses 2nd CP and becomes board-locked.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Jagged/Plows; spread across terrain                  |
| 2    | G2 + Stubborn + remaining starter; all 4 terrains covered |
| 3    | G1 Reclaim + Presence + Earth; Bulwark L1 online          |
| 4–6  | Bulwark L2 cycle; Reclaim every 3 turns                   |
| 7+   | Bulwark L3; major integration                              |

## Source Notes

- **Mechanics**: `data/references/wiki/stone-unyielding-defiance.json` (Wiki-parsed 2026-04-23).
- **Openings**: [jyonker13 BGG 2512536](https://boardgamegeek.com/thread/2512536/openings-stones-unyielding-defiance).
