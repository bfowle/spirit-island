# Towering Roots of the Jungle

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from [Sipricy's Reddit Spotlight](https://www.reddit.com/r/spiritisland/comments/1brgxwu/) + [RedReVenge BGG thread 3167402](https://boardgamegeek.com/thread/3167402). Two competing schools — empowered-stall vs. G3-tempo.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 1 · Control 3 · Fear 2 · **Defense 5** · Utility 2 |
| Primary Elements      | **Plant** (everything) · **Sun** (both innates) · Earth (Shelter L2+; Moon L1) |
| Special Rules         | Enduring Vitality (Vitality tokens prevent Blight) + Heart-Tree Guards the Land (Incarna grants Range +1; Incarna's land is damage-immune) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [RedReVenge thread 3167402](https://boardgamegeek.com/thread/3167402) + [Sipricy Reddit 1brgxwu](https://www.reddit.com/r/spiritisland/comments/1brgxwu/) |
```

## Spirit Overview — Framing

Towering Roots is a **Moderate-complexity Incarna spirit** built around a single massive Heart-Tree and Vitality tokens. Wiki:

> Incredibly good at protecting everything at its Incarna — and can draw Invaders towards there — but is constrained in when and where it can move its Incarna… starts off vastly better at guarding the land than at smashing things.

**Community split on play style**:

- **Empowered-stall** (canonical): slow the game, stack Vitality, protect the land.
- **G3-tempo** (Sipricy's contrarian read): **"Forget Growth 2. Prioritise G3 throughout the game for the energy, the range on presence placement, and optional Incarna move."**

**Complexity signal**: Moderate mechanically but conceptually split — two competing schools with different macro play.

## Starting Setup

> Put **3 Presence** on your starting board: 1 in the highest-numbered Jungle without Blight, 1 in the highest-numbered Mountain, 1 in the highest-numbered Wetland. Put Incarna (Unempowered) side up in the Jungle.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                  | Best when                                              |
|--------|----------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Add Presence (R0)                              | Reclaim cycle                                          |
| **G2** | Gain 1 Power Card + Add Presence (R1) + Add Vitality    | Vitality-bank                                          |
| **G3** | Gain 1 Power Card + Add Presence (R3) + Move/Replace Incarna | **Sipricy's recommended default**                    |

## Presence Tracks

- **Energy**: `energy1 → energy2 → earth → energy4 → plant → energy6`
- **CP**: `card1 → card2 → sunX → card3 → plantX → card4`

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Enduring Vitality

> Some of your Actions Add Vitality Tokens. (Each Vitality in a land with no Blight prevents 1 Blight from being added, then is Removed.)

Vitality tokens = preventive Blight-absorbers. **But Sipricy flags**: *"If you're not skipping Builds for a specific, particular reason, you need to ask yourself why you're playing toward Empowering."* Vitality often sits unused.

### Special Rule: Heart-Tree Guards the Land

> You have an Incarna (Heart-Tree).
> - Your Powers get Range +1 if Incarna is in the origin land.
> - Invaders/Dahan/Beast can't be damaged or destroyed at Incarna's land.

The Incarna's land is a **protected dumpsite** — but damage-immune also means *you can't Dahan-counter* there. Sipricy warns:

> Essentially, your Incarna is Concealing Shadows in disguise.

### Innate: Shelter Under Towering Branches

- **Speed**: Slow · **Range**: 0 · **Target**: Any

| Level | Thresholds                              | Effect                             |
|-------|-----------------------------------------|-------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gather up to 1 Dahan. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gather up to 1 Explorer. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gather up to 1 Town. |
| 4     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gather up to 1 City. |

### Innate: Revoke Sanctuary and Cast Out

- **Speed**: Slow · **Range**: 0 from Incarna · **Target**: Invaders

| Level | Thresholds                              | Effect                                     |
|-------|-----------------------------------------|---------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Fear. Remove 1 Explorer/Town. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Fear. Remove 1 Explorer/Town. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | 1 Fear. Remove 1 Invader. |

## Unique Cards (4, Wiki-verified names)

- **Radiant and Hallowed Grove**
- **Blooming of the Rocks and Trees**
- **Boon of Resilient Power** (shared HoSI unique)
- **Entwine the Fates of All** (shared HoSI unique)

## Key Strategic Principles

1. **Sipricy's default: G3 throughout the game.** Energy + Range + optional Incarna move.
2. **Plant > Sun > Earth/Moon.** Sipricy: *"primarily for Minor/Major Powers with Plant, and secondarily for cards with Sun/Moon/Earth, with Sun being the most important of these three."*
3. **Stick to 4–5 Energy Majors.** Pair with 0-cost Plant Minors.
4. **Don't Empower for its own sake.** Vitality often sits unused.
5. **Sometimes let the Incarna be destroyed** to unlock Dahan retaliation. Sipricy: *"Your Incarna is Concealing Shadows in disguise."*
6. **Fear output is anaemic** — don't plan on it.

## Possible Openings

### Shared starting state

- **3 Presence + Incarna** on starting board (Jungle, Mountain, Wetland spread).
- **4 Uniques** in hand.
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Sipricy G3-tempo 🟨 (contrarian default)

**T1 · Growth**: G3. Play **Radiant and Hallowed Grove** (remove Explorers) or **Blooming of the Rocks and Trees** as tempo Wilds-drop.

**T2–T3**: Keep pulsing G3 to scale Energy.

**T4 target**: 4 Energy/turn + 3 card plays + a Plant Major (Walls of Rock and Thorn, Trees Radiate Celestial Brilliance).

### Opening B — Fast Empower (n0radrenaline) 🟥

G2 T1, G2 T2 while playing the Vitality-adding card, so the Incarna empowers early; then stack Invaders in the Incarna's land and keep it Vitality-topped-up. Roots becomes invulnerable but needs a partner to actually kill things.

### Opening Decision

- **Default Opening A** (Sipricy G3) in most matchups.
- **Opening B** (Fast Empower) with a damage-partner who closes the kill.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Plant + Sun)

| # | Card | Why |
|---|------|-----|
| 1 | **Unrelenting Growth** | 0-cost Sun + Plant |
| 2 | **Song of Sanctity** | 0-cost Sun + Plant |
| 3 | **Drift Down to Rest** | 0-cost Sun + Plant |
| 4 | **Gift of Constancy** | 0-cost Sun + Plant |
| 5 | **Quicken the Earth's Struggles** | 0-cost Earth + Plant |
| 6 | **Sap Their Strength** | Moon + Earth + Plant |
| 7 | **Drifting Into Stillness** | Moon + Plant |
| 8 | **Purify the Land** | Moon + Water + Plant |
| 9 | **Dry Wood** | Fire + Plant |
| 10 | **Call of the Dahan Ways** | Moon + Earth |

### Top 5 Major Draft Picks

| # | Card | Cost | Why |
|---|------|------|-----|
| 1 | **Walls of Rock and Thorn** | 4 | Plant + Earth |
| 2 | **Trees Radiate Celestial Brilliance** | 4 | Plant + multi-element |
| 3 | **Vigor of the Breaking Dawn** | 4 | Sun + Plant |
| 4 | **Dream of the Untouched Land** | 4 | Sun + Moon + Plant |
| 5 | **Trees Radiate Ancient Sanctity** | 3 | Moon + Sun + Plant + Earth |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                     |
|-------------------------|--------|----------------------------------------------------------|
| England                 | ★★★★☆  | Build-heavy — Vitality + protected land shine            |
| Scotland                | ★★★★☆  | Build volume                                             |
| Brandenburg-Prussia     | ★★★☆☆  | `[VERIFY]`                                               |
| Sweden                  | ★★★☆☆  | `[VERIFY]`                                               |
| **Habsburg Mining**     | ★★☆☆☆  | **Worst matchup** per BGG thread consensus                |
| Others                  | ★★★☆☆  | `[VERIFY]`                                               |

## Board / Map Configuration

Roots needs **Jungle access** for the Incarna start. `[VERIFY board-specific ratings]`.

## Common Mistakes

```admonish failure title="Sipricy's named mistakes"
1. **Empowering for its own sake.** Ask "why am I playing toward Empowering?"
2. **Defaulting to G2 for Vitality every turn.** Vitality often sits unused.
3. **Putting Vitality in Dahan-counterattack lands.** Dahan can't counter at the Incarna's land.
4. **Relying on Roots to generate fear.** Fear output is anaemic.
```

## Synergy Partners (Multiplayer)

- **Dumpsite partners**: Breath of Darkness, Vengeance, Volcano, Stone, Lure.
- **Aggressive partners who kill while Roots stalls**.

## Source Notes

- **Mechanics**: `data/references/wiki/towering-roots-of-the-jungle.json` (Wiki-parsed 2026-04-23).
- **Openings**: [RedReVenge BGG 3167402](https://boardgamegeek.com/thread/3167402) + [Sipricy Reddit](https://www.reddit.com/r/spiritisland/comments/1brgxwu/).
- **Coverage note**: Two competing schools; physical-copy verify before definitive play.
