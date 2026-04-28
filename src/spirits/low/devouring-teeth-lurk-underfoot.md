# Devouring Teeth Lurk Underfoot

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from [taurunti's obsidian-spiritisland notes](https://thetaurunti.github.io/obsidian-spiritisland/Spirits/Devouring-Teeth-Lurk-Underfoot) + [BGG thread 2924043](https://boardgamegeek.com/thread/2924043). No Rei / latentoctopus / Phantaskippy coverage.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Horizons of Spirit Island                          |
| Complexity            | Low                                                |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | **Offense 5** · Control 2 · Fear 2 · Defense 1 · Utility 1 |
| Primary Elements      | **Fire** · **Animal** (both innate all tiers) · Earth (L2+) |
| Special Rules         | Territorial Aggression (+1 Damage to every damage-dealing Power — including Minors/Majors) |
| Aspects               | None                                               |
| HoSI                  | Yes — pairs with Base Minors + Majors              |
| Wiki Guides           | None official; [taurunti community notes](https://thetaurunti.github.io/obsidian-spiritisland/Guides/Teeth---Standard) |
```

## Spirit Overview — Framing

Devouring Teeth is a **territorial ambush predator** — Range-0 offensive spirit that walks itself to threats via its innate's Gather threshold. Territorial Aggression gives +1 Damage to every damage-dealing Power, weaponizing the *whole draft pool*.

**Wiki-printed playstyle note**:

> Range-0 aggressor. Innate Gathers the spirit into Invader lands; Territorial Aggression amplifies all damage.

**Teaching role**: demonstrates range-0 + mobility; small damage bumps compound.

**Complexity signal**: Low — appropriate for an onboarding spirit.

## Starting Setup

> Put **1 Presence** on your starting board, in **land #5**. You start with your **4 Unique Power Cards and 0 Energy**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                       | Best when                                              |
|--------|-----------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Add Presence (R0)                   | Reclaim + stack                                        |
| **G2** | Gain 1 Power Card + Add Presence (R1)         | Card + close spread                                    |
| **G3** | Add Presence (R2) + +3 Energy                 | Spread + energy                                        |

## Presence Tracks

- **Energy**: `energy2 → fire → energy3 → energy4 → animal → energy6 → energy7`
- **CP**: `card1 → card2 → animalX → fireX → card3 → earthX → card4`

**Starting income**: 2 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Territorial Aggression

> Your Damage-Dealing Powers do +1 Damage. (This adds +1 Damage total for the Power, even if the Power Damages multiple Invaders or "each Invader". It can boost Minor/Major Power Cards, too, not just your Uniques + Innate.)

**+1 Damage to every damage Power**. A 1-damage Minor becomes 2-damage; a 3-damage City-killer becomes 4.

### Innate: Death Approaches from Beneath the Surface

- **Speed**: Slow · **Range**: 1 · **Target**: Invaders

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | If you don't have Presence in target land, Gather 1 of your Presence. (Required.) |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage. (+1 from Territorial Aggression) |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 2 Damage. |
| 4     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 5 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 2 Fear. 4 Damage. |

L1 is the mobility lever: Gather moves Teeth into the target land (required). L2+ adds damage with the +1 Territorial bonus.

## Unique Cards (all 4, Wiki-verified)

`[VERIFY cost/text via physical copy]`:

- **Herd Towards the Lurking Maw** — push Dahan; Defend utility.
- **Ferocious Rampage** — multi-Invader damage.
- **Mark Territory with Scars and Teeth** — Fast multi-Invader.
- **Gift of Furious Might** (shared HoSI unique) — partner-amp or self-cast for easy City-kill.

## Key Strategic Principles

1. **Fire + Animal** — draft priority.
2. **Territorial Aggression multiplies drafts.** Gold's Allure, Quicken the Earth's Struggles, Call to Guard are standouts per taurunti.
3. **Gather tier is required**, not optional — this is the whole point of the innate.
4. **Gift of Furious Might** is arguably the easiest City-killer in HoSI thanks to Territorial Aggression turning "3 Damage" into 4.
5. **Don't draft Moon/Water out of excitement** — they don't fire the innate.

## Possible Openings

### Shared starting state

- **1 Presence** on land #5.
- **4 Uniques** in hand.
- **2 Energy**, 1 Card Play.

### Opening A — taurunti Standard 🟨

**T1 · Growth**: G3 bot (+Presence 0, +Presence 1, +3E → 3E).
**T1 · Play**: **Herd Towards the Lurking Maw + Ferocious Rampage**. Clears a freshly-explored land; Herd's Defend-9-style utility is massive on a board with a Disease land or early T1 Build risk.

**T2 · Growth**: G2 top (Reclaim one + Gain Minor).
**T2 · Play**: **Mark Territory + Furious Rampage** — hits L2 innate (2F+1E+2A) for 2 Damage + Gather 1, and Rampage deletes a Town.

**T3+**: Aim for 3rd card play (bottom-track presence removal). **Gift of Furious Might** for City-kill.

### Opening B — Reclaim Loop 🟥

Cycle Herd+Rampage every 2 turns — simple but repetitive.

### Opening Decision

- **Default Opening A** — broad draft pool.
- **Opening B** when learning the spirit; more predictable.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Fire + Animal)

| # | Card | Why |
|---|------|-----|
| 1 | **Gold's Allure** | taurunti's top flag |
| 2 | **Quicken the Earth's Struggles** | Fast, Sacred-Site, destroys a Town before Ravage (amplified by Territorial Aggression) |
| 3 | **Call to Guard** | cancels Build AND Escalation Town (BGG 2924043) |
| 4 | **Predatory Nightmares** | 0-cost Animal |
| 5 | **Call to Bloodshed** | 0-cost Animal |
| 6 | **Pyroclastic Friction** | Fire + Earth |
| 7 | **Visions of Fiery Doom** | Fire-feeder |
| 8 | **Call to Migrate** | Air + Animal |
| 9 | **Rain of Blood** | Fire + multi-element |
| 10 | **Dry Wood** | 0-cost Fire |

### Top 5 Major Draft Picks

| # | Card | Why |
|---|------|-----|
| 1 | **Tigers Hunting** | Cheap Fire + Animal |
| 2 | **Angry Bears** | Fire + Animal + Fear |
| 3 | **Pillar of Living Flame** | Fire + board-wide |
| 4 | **Insatiable Hunger of the Swarm** | Animal + multi-land |
| 5 | **Instruments of Their Own Ruin** | Fire + Animal multi-land |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                     |
|-------------------------|--------|----------------------------------------------------------|
| Brandenburg-Prussia     | ★★★★☆  | +1 Damage helps kill extra T1 Town                       |
| England                 | ★★★☆☆  | Gather chases Coastal Land 1 Builds without range crutch |
| Sweden                  | ★★★☆☆  | `[VERIFY]`                                               |
| Habsburg                | ★★★☆☆  | `[VERIFY]`                                               |

## Board / Map Configuration

Land #5 placement is fine on boards G and H (both have usable interior #5s).

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Treating Herd as pure Defend** — it's push-Dahan, closer to Year of Perfect Stillness than real Defend.
2. **Ignoring the Gather-tier clause** ("if you don't have presence in target land") — the whole point of the innate.
3. **Drafting Moon/Water Minors** — don't fire the innate.
```

## Synergy Partners (Multiplayer)

- **Rising Heat of Stone and Sand** — shared Fire (BGG reports T1 interior wipe via Gift of Furious Might + Blistering Heat -1 HP).
- **Lightning** — grants Fast so Teeth's Slow innate fires pre-Ravage.
- **Shadows Flicker Like Flame** — shared Fire, quiet fear synergies.
- **Avoid Volcano** if boards are tight (both want to be in Invader lands).

## Source Notes

- **Mechanics**: `data/references/wiki/devouring-teeth-lurk-underfoot.json` (Wiki-parsed 2026-04-23).
- **Openings**: [taurunti Guides](https://thetaurunti.github.io/obsidian-spiritisland/Guides/Teeth---Standard) + [BGG 2924043](https://boardgamegeek.com/thread/2924043).
