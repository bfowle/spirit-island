# Breath of Darkness Down Your Spine

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from **[Sarusta's advanced guide](https://docs.google.com/document/u/0/d/1r2K_cjZ64vCszBI72bO0XYFnkyPLnPTuVYtrLPhidOc)** + [Reddit builds thread](https://www.reddit.com/r/spiritisland/comments/14vzyjp/). Best-covered of the NI Incarna spirits.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 2 · Control 4 · **Fear 5** · Defense 1 · Utility 2 |
| Primary Elements      | **Moon** (everything) · **Air** (Trail + Lost) · **Animal** (Trail) |
| Special Rules         | Terror Stalks the Land (Incarna Abducts 1 Explorer/Town/turn to The Endless Dark) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| Sarusta Guide         | [Google Doc](https://docs.google.com/document/u/0/d/1r2K_cjZ64vCszBI72bO0XYFnkyPLnPTuVYtrLPhidOc) |
```

## Spirit Overview — Framing

Breath of Darkness is a **High-complexity Incarna spirit** with the unique **Abduction** mechanic — damage a lone Invader, it goes to The Endless Dark (ED) instead of being destroyed. Sarusta:

> An incredible Control-Fear Spirit, with a control game that functions very similarly to Finder. Abductions are a powerful Control mechanic that can render Invaders unable to function normally.

**Central puzzle**: keep the ED full while keeping the board empty. Every non-Reclaim Growth **escapes** Invaders back to the board.

## Starting Setup

> Put **2 Presence and your Incarna** (Unempowered) on your starting board: 1 Presence + Incarna in the lowest-numbered Jungle, 1 Presence in the highest-numbered Jungle. You start with your **4 Unique Power Cards**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                  | Best when                                              |
|--------|----------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + Move Incarna               | Reclaim + reposition                                   |
| **G2** | Gain 1 Power Card + Add Presence (R3) + Darkness Escape 2 | Card + spread (escapes 2 Invaders!)                    |
| **G3** | Add Presence (R1) + Add/Move Incarna + Darkness Escape 1  | Signature Incarna-mobility growth                      |

**Tempo cost**: G2/G3 both escape pieces from ED back to the board. Only G1 Reclaim holds the ED fully.

## Presence Tracks

- **Energy**: `energy1 → energy2 → moon → energy3 → empowerincarna → energy4animal → energy5air`
- **CP**: `card2 → movepres → card3 → moonX → reclaim1 → card4air`

**Starting income**: 1 Energy, 2 Card Plays.

## Core Mechanics & Special Rules

### Special Rule: Terror Stalks the Land

> You have an Incarna. Empower Incarna after uncovering [Empower]. You may Abduct 1 Explorer/Town at empowered Incarna each Fast phase. To Abduct a piece, Move it to The Endless Dark. When pieces Escape, Move them to non-Ocean lands with your Presence/Incarna; if they have no legal destination, they stay in the ED.

**Abduction = Remove to ED.** Escape = return to a land with your Presence.

### Innate: Leave a Trail of Deathly Silence

- **Speed**: Fast · **Target**: Yourself

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage at Incarna. Push Incarna. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +1 Damage at Incarna. Push Incarna. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +1 Damage at Incarna. Push Incarna. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Move Incarna to ED; it brings 1 Invader. |

Trail is the Incarna's mobility + abduction engine.

### Innate: Lost in the Endless Dark

- **Speed**: Slow · **Target**: ED

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear per Invader (max 4). Downgrade up to 1 Invader. |
| 2     | 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear per Invader (max 4). Downgrade any number of Invaders. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Add 1 Beast. |

Abducted Invaders → Fear + Downgrade. ED is the payoff.

## Unique Cards (4, Wiki-verified names)

- **Reach from the Infinite Darkness**
- **Swallowed by the Endless Dark**
- **Emerge from the Dread Night Wind**
- **Terror of the Hunted**

`[VERIFY exact costs/text.]`

## Key Strategic Principles

1. **Minor drafts**: Sarusta — *"Elements. Elements. Elements."* Triple on-element (Moon/Air/Animal).
2. **Dire Metamorphosis is the top Minor** — dumps 4 tokens + Blight into the ED.
3. **Sleep and Never Waken is a godsend Major.**
4. **Skip Majors create free dumpsites.**
5. **Don't Empower mid-game just to have it** — Empower is expensive and often a trap.
6. **The Endless Dark is targetable** for Powers with terrain-origin requirements — Jungle-Hungers it from your Jungle presence.
7. **Don't Escape Invaders to illegal terrain** (Ocean/Volcano presence can brick you into instant-loss).

## Possible Openings

### Shared starting state

- **2 Presence + Incarna** on starting Jungles.
- **4 Uniques** in hand.
- **1 Energy**, 2 Card Plays.

### Opening A — Sarusta's Mixed (default) 🟨

Start Top Track to Breakpoint 1 (after Moon node = 2 plays + 2E), then drop to Bottom. L0cksmash's solo line vs. Scotland 6:

**G2 → G3 → G1 Reclaim → G3 → G2 → G2/G3 → G1 Reclaim**

T1 play-pairing rule (Sarusta): pair one **Moon+Air** card with one **Animal+Air** card so both innates' first thresholds fire. *"Do not play Terror + Reach, as you'll be left without an Air on subsequent turns."*

T1 Swallowed + Trail: abducts an Explorer then the City (if L2 explored T1).

T4 target: 4 Moon / 3 Animal for Lost's big threshold + 3-piece ED for recurring fear.

### Opening B — Top-track Empower/Major 🟥

G2 exclusively. Reclaims rarely, late Empower, plays Majors. Struggles to threshold Trail.

### Opening C — Bottom-track Trail/Minor 🟥

G2/G3 mix for plays; hits Trail's 3/1/1 threshold consistently via early Moon.

### Opening Decision

- **Default Opening A** (Mixed) — Sarusta's recommendation.
- **Opening B** when Major-draft is strong.
- **Opening C** when card plays > Empowerment.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Moon + Air + Animal — triple on-element)

| # | Card | Why |
|---|------|-----|
| 1 | **Dire Metamorphosis** | Sarusta: top pick — dumps tokens + Blight into ED |
| 2 | **Veil the Night's Hunt** | Sarusta pick |
| 3 | **Here There Be Monsters** | Moon + Animal |
| 4 | **Bats Scout for Raids by Darkness** | Moon + Air + Animal |
| 5 | **Predatory Nightmares** | 0-cost Moon + Animal |
| 6 | **Call to Migrate** | 0-cost Air + Animal |
| 7 | **Call to Bloodshed** | 0-cost Moon + Animal |
| 8 | **Strange Tales of the Sky** | Moon + Air |
| 9 | **Pull Beneath the Hungry Earth** | Moon + Earth |
| 10 | **Entrancing Apparitions** | Moon + Air |

### Top 5 Major Draft Picks

| # | Card | Why |
|---|------|-----|
| 1 | **Sleep and Never Waken** | Sarusta: "a godsend" |
| 2 | **Savage Transformation** | Sarusta: punches above cost |
| 3 | **Tigers Hunting** | Cheap damage |
| 4 | **Indomitable Claim** (or other Skip major) | Creates free dumpsite |
| 5 | **The Jungle Hungers** | Plant + Moon; ED-targetable |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                     |
|-------------------------|--------|----------------------------------------------------------|
| Scotland                | ★★★★☆  | Wide adversary — Darkness strong                         |
| Brandenburg-Prussia     | ★★★★☆  | Wide                                                     |
| Sweden                  | ★★★☆☆  | `[VERIFY]`                                               |
| France-Plantation       | ★★★☆☆  | `[VERIFY]`                                               |
| **England**             | ★★☆☆☆  | **Weak** — tall-building adversary                        |
| **Habsburg Mining L5+** | ★★☆☆☆  | **Weak** — tall                                          |
| Russia                  | ★★★☆☆  | `[VERIFY]`                                               |
| Habsburg Livestock      | ★★★☆☆  | `[VERIFY]`                                               |

## Board / Map Configuration

**Minimise adjacencies** is the most important board-pick criterion (fewer escape-land options = more stable ED).

## Common Mistakes

```admonish failure title="Sarusta named mistakes"
1. **Forgetting the ED is targetable** for terrain-origin Powers (Jungle Hungers from Jungle presence).
2. **Letting a land reach 2 Cities before you have a Major.**
3. **Escaping Invaders to illegal terrain** (Ocean/Volcano presence = instant-loss risk).
4. **Empowering mid-game just to have it.**
5. **Playing Terror + Reach T1** — loses Air for subsequent turns.
```

## Synergy Partners (Multiplayer)

Sarusta: *"Darkness pairs incredibly well with 'dumpsite' Spirits… Stone, Vengeance, Roots, Lure, Gaze, DUE and Volcano."*

- **Wildfire, Ocean, Volcano** benefit uniquely from Reach's presence-abduction + range boost.

## Source Notes

- **Mechanics**: `data/references/wiki/breath-of-darkness.json` (Wiki-parsed 2026-04-23).
- **Primary guide**: [Sarusta's Advanced Guide](https://docs.google.com/document/u/0/d/1r2K_cjZ64vCszBI72bO0XYFnkyPLnPTuVYtrLPhidOc).
- **Reddit**: [builds thread](https://www.reddit.com/r/spiritisland/comments/14vzyjp/).
