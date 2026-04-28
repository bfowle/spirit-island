# Sun-Bright Whirlwind

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from [taurunti's notes](https://thetaurunti.github.io/obsidian-spiritisland/Spirits/Sun-Bright-Whirlwind) + [BGG thread 2907137](https://boardgamegeek.com/thread/2907137). No Rei / latentoctopus / Phantaskippy.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Horizons of Spirit Island                          |
| Complexity            | Low                                                |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 3 · **Control 5** · Fear 1 · Defense 1 · Utility 3 |
| Primary Elements      | **Air** (all tiers) · **Sun** (scaling) · Minor Moon/Earth |
| Special Rules         | A Stiff Wind at Their Backs (Growth-placement Pushes 1 Explorer/Dahan) |
| Aspects               | None                                               |
| HoSI                  | Yes                                                |
| Wiki Guides           | [taurunti Standard](https://thetaurunti.github.io/obsidian-spiritisland/Guides/Whirlwind---Standard) + [BGG 2907137](https://boardgamegeek.com/thread/2907137) |
```

## Spirit Overview — Framing

Sun-Bright Whirlwind is an **Explorer-control and tempo-utility spirit** — the ur-anti-Explore build. A Stiff Wind at Their Backs pushes 1 Explorer/Dahan from any land where you add presence during Growth — a free board-wipe each turn.

**Wiki-printed playstyle note**:

> Explorer-control specialist. Weakness: struggles vs. Towns/Cities.

**Teaching role**: placement-as-attack; "prevent the Build" timing.

## Starting Setup

> Put **3 Presence** on your starting board: **1 in the highest-numbered Sands, 2 in the lowest-numbered Mountain**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                  | Best when                                              |
|--------|----------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy                  | Reclaim cycle                                          |
| **G2** | Add Presence (R1) + +4 Energy                            | Energy spike                                           |
| **G3** | Gain 1 Power Card + Add Presence (R4)                    | Card + far-reach spread                                |

## Presence Tracks

- **Energy**: `energy1 → energy2 → sun → energy3 → energy4air → energy6`
- **CP**: `card1 → card2 → card3 → airX → card4 → card5sun`

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: A Stiff Wind at Their Backs

> After you Add Presence during Growth, Push up to 1 Explorer/Dahan from that land.

**Free Push per Growth-placement**. The Wiki literally warns players to announce this (fires in Spirit Phase, not as a Fast power).

### Innate: Violent Windstorms

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Push up to 1 Explorer. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear. Push up to 2 Explorer/Town. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | For each Invader Pushed by this Power, 1 Damage in the land it was Pushed to. |
| 4     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 5 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 4 Damage (in target land). |

L3 is the payoff — cascades into T2 pushes for damage.

## Unique Cards (all 4, Wiki-verified names)

- **Tempest of Leaves and Branches**
- **Scatter to the Winds**
- **Exaltation of the Incandescent Sky**
- **Gift of the Sunlit Air** (shared HoSI unique) — ally Fast-conversion utility.

## Key Strategic Principles

1. **Air at 2× the rate of Sun.** Cheapest Air-element Minor beats situationally-better non-Air Minors.
2. **Stiff Wind fires in Spirit Phase, not as Fast power.** Announce to table.
3. **Underplay T3/T4** — plan Energy forward for T5 power spike.
4. **T5 is the big turn** — need 2 Sun + 3 Air in card-plays to reach L3 innate.
5. **Don't try to solo kill Towns/Cities** — Whirlwind is an Explorer specialist.

## Possible Openings

### Shared starting state

- **3 Presence** (1 Sands + 2 Mountain).
- **4 Uniques** in hand.
- **0 Energy**, 1 Card Play.

### Opening A — taurunti Standard 🟨

**T1 · Growth**: G3 bot + cheapest Air Minor → Gift of Sunlit Air target.
- Every Growth-add also Pushes.

**T2 · Growth**: G2 bot.
**T2 · Play**: Drop **Gift of Wind + Scatter to the Winds + Tempest**.

**T3 · Growth**: Reclaim.
**T3 · Play**: Underplay — save plays for T5.

**T4 · Growth**: G3 bot + Minor.
**T4 · Play**: 2 additional cards.

**T5 · Growth**: G3 if enough energy (tier-4 goal: 4 Damage in target land); G2 for max plays.
**T5 · Play**: **Big turn**. 2 Sun + 3 Air in card-plays → tier-3 innate.

**T6+ · Growth**: Reclaim + adapt.

### Opening Decision

- **Default Opening A** — consistent T5 power spike.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Air > Sun)

| # | Card | Why |
|---|------|-----|
| 1 | **Call to Migrate** | 0-cost Air + Animal |
| 2 | **Drift Down to Rest** | 0-cost Air + Sun + Plant |
| 3 | **Strange Tales of the Sky** | Air + Moon |
| 4 | **Travel Unsuspected** | Air + Water |
| 5 | **Bats Scout for Raids** | Air + Moon + Animal |
| 6 | **Entrancing Apparitions** | Moon + Air |
| 7 | **Gift of Constancy** | Sun + Plant + Animal |
| 8 | **Song of Sanctity** | 0-cost Sun + Plant |
| 9 | **Unrelenting Growth** | 0-cost Sun + Plant |
| 10 | **Elemental Boon** | 0-cost four-element flex |

### Top 5 Major Draft Picks

| # | Card | Why |
|---|------|-----|
| 1 | **Wrap in Wings of Sunlight** | Sun + Air |
| 2 | **Sky Stretches to Shore** | Sun + Moon + Air |
| 3 | **Terrifying Nightmares** | Moon + Air |
| 4 | **Instruments of Their Own Ruin** | Fire + Air + Animal |
| 5 | **Trees Radiate Ancient Sanctity** | Sun + multi-land Defend |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                     |
|-------------------------|--------|----------------------------------------------------------|
| Brandenburg-Prussia     | ★★★★☆  | Explore-heavy — Whirlwind shines                          |
| Russia                  | ★★★☆☆  | Explore-heavy                                             |
| Sweden                  | ★★☆☆☆  | Coast-heavy — fewer inland Explores to push              |
| France-Plantation       | ★★★☆☆  | Coast-heavy                                               |
| Others                  | ★★★☆☆  | `[VERIFY]`                                               |

## Board / Map Configuration

Boards with connected Sands/Mountain clusters. **G excellent**, **H workable**.

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Using Stiff Wind as a Fast Push power.** Fires in Spirit Phase; new players mis-time it.
2. **Over-spending T3/T4 instead of underplaying toward T5.** Plan energy forward.
3. **Drafting Sun-heavy cards ignoring Air.** Air is the scarce resource.
4. **Trying to solo kill Towns/Cities.** Partner with a heavy-hitter.
```

## Synergy Partners (Multiplayer)

- **Volcano** — Whirlwind Push-into-land + Volcano damage ticks.
- **Ocean's Hungry Grasp** — Push Explorers into coastline for drowning.
- **Lightning** — shared Air, speeds up Slow innate.
- **Any Major-reliant spirit** — Gift of Sunlit Air / Wind-Sped Steps ally amps.
- **Avoid pairing with another Explorer-killer** — redundant.

## Source Notes

- **Mechanics**: `data/references/wiki/sun-bright-whirlwind.json` (Wiki-parsed 2026-04-23).
- **Openings**: [taurunti Standard](https://thetaurunti.github.io/obsidian-spiritisland/Guides/Whirlwind---Standard) + [BGG 2907137](https://boardgamegeek.com/thread/2907137).
