# Wandering Voice Keens Delirium

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from the [Wiki Builds section](https://spiritislandwiki.com/index.php?title=Wandering_Voice_Keens_Delirium) + [Reddit Spotlight thread](https://www.reddit.com/r/spiritisland/comments/1ceduyf/). High-complexity Incarna + Strife engine.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | **Very High**                                      |
| Play Difficulty       | `[VERIFY]`                                         |
| Growth type           | "one" — pick one growth per turn                   |
| Power summary (1–5)   | Offense 2 · **Control 5** · Fear 3 · Defense 1 · Utility 2 |
| Primary Elements      | **Air** (mandatory Mind-Shattering) · **Moon** + **Sun** (paired tiers) |
| Special Rules         | Clarion Voice Given Form (Incarna Isolates if Empowered) + Spread Tumult & Delusion (Incarna adds Strife + Dahan can't Ravage near Incarna) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| Wiki Builds           | Wiki [Wandering Voice](https://spiritislandwiki.com/index.php?title=Wandering_Voice_Keens_Delirium) Builds section |
```

## Spirit Overview — Framing

Wandering Voice is a **Very-High-complexity Incarna spirit weaponising Strife**. Spread Tumult and Delusion adds 1 Strife when the Incarna arrives at an Invader land AND prevents Dahan in that land + adjacent from participating in Ravage.

**The twist** (TheLordSet, Reddit):

> Strife stops being a defensive/counter-offensive tool, and turns into a control/offense tool.

Mind-Shattering Song's top tier destroys 1 Invader with Strife per Sun+Moon pair.

**echowing442**:

> It flips Strife to being more of a defensive tool, at least at first… as you gain more card plays and can use your innate powers to boost your incarna around the island, those setups become more viable.

**Complexity signal**: Very High. Wandering Voice is the one spirit Brett's memory explicitly flags as tough.

## Starting Setup

> Put **2 Presence** on your starting board: 1 in land #6 and 1 in land #7. Put **Incarna (Unempowered)** in land #6. You start with your **4 Unique Power Cards**.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                          | Best when                                              |
|--------|------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Add/Move Incarna + +1 Energy                           | Reclaim All cycle                                      |
| **G2** | Add Presence (R3) + Add Presence (R1)                            | Double-spread                                          |
| **G3** | Gain 1 Power Card + Add Presence (R2) + +1 Energy                | Card + energy                                          |

## Presence Tracks

- **Energy**: `energy0 → energy1 → sunormoon → energy2 → air → energy4 → pushvoiceincarna`
- **CP**: `card1 → card2 → card2 → card3 → reclaim1 → card4`

**Starting income**: 0 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: A Clarion Voice Given Form

> You have an Incarna. If Empowered, it Isolates its land.

Empowered Incarna = full land Isolation. Paired with Strife + Dahan-suppression, a very tight control lock.

### Special Rule: Spread Tumult and Delusion

> When your Actions add/move Incarna to a land with Invaders, Add 1 Strife in the destination land. In lands with or adjacent to Incarna: if Strife is present, Dahan do not participate in Ravage.

Incarna = mobile Strife + Dahan-silencer.

### Innate: Inscrutable Journeying

- **Speed**: Fast · **Target**: Yourself

Incarna movement utility. `[VERIFY thresholds]`.

### Innate: Mind-Shattering Song

- **Speed**: Slow · **Range**: 1 · **Target**: Strife

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Fear per Moon you have. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | 1 Damage per Sun you have, to Invaders with Strife only. |
| 3     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | For each Sun–Moon pair you have, Destroy 1 Invader with Strife. |

Top tier is the closer — destroy Strifed Invaders per Sun+Moon pair.

## Unique Cards (4, Wiki-verified names)

- **Frightful Keening**
- **Turmoil's Touch**
- **Exhale Confusion and Delirium**
- **Twist Perceptions**

`[VERIFY exact costs/text.]`

## Key Strategic Principles

1. **Air is mandatory** — every Mind-Shattering tier.
2. **Moon and Sun are paired** — each pair destroys one more Strifed Invader at L3.
3. **Reclaim Loop build** (Wiki) wants to stay at 4 starter cards — intentionally tight.
4. **Top Track** (TheLordSet) broadens card choice; Sun/Moon track node is pivotal.
5. **Without Air, "you really don't deal any damage"** (BobLoblawsLab).

## Possible Openings

### Shared starting state

- **2 Presence + Incarna** on land #6 + #7.
- **4 Uniques** in hand.
- **0 Energy**, 1 Card Play.

### Opening A — Reclaim Loop build (Wiki canonical) 🟨

**T1 · Growth**: G2 top/bottom (play 1 one-cost unique; push Incarna ×1).

**T2 · Growth**: G2 bottom/bottom (play 0 cards OR play Exhale Confusion; push Incarna ×1).

**T3 · Growth**: G3 bottom/bottom → reclaim played card via **Turmoil's Touch**; play full hand; empower Incarna; max-tier Mind-Shattering Song (**destroy 3 Strifed Invaders + 3 Fear + 3 Damage**).

**T4+ · Growth**: G1 Reclaim All; max innate every turn.

### Opening B — Top-track / Mixed (TheLordSet) 🟥

**T1 · Growth**: G2 top/bottom.
**T2 · Growth**: G2 top/top.
**T3 · Growth**: G3 top — Mind-Shattering Song tier 2.

Broader card choice; less reclaim-dependent. TheLordSet: *"top track has Sun/Moon and Air — Sun/Moon + Air gives you the elements you would need from one more card play."*

### Opening C — BobLoblawsLab G3 T1 🟥

G3 T1, then let draws dictate track.

### Opening Decision

- **Default Opening A** (Reclaim Loop) — Wiki canonical; intentionally tight.
- **Opening B** (Mixed) for more card variety.
- **Opening C** for experimental play.

## Card Priority Ratings

### Top 10 Minor Draft Picks (Air primary)

For the Reclaim Loop, **don't draft** — stay at 4 starter cards. For Top Track / Mixed:

| # | Card | Why |
|---|------|-----|
| 1 | **Call to Migrate** | 0-cost Air |
| 2 | **Strange Tales of the Sky** | Moon + Air |
| 3 | **Drift Down to Rest** | Sun + Air + Plant |
| 4 | **Travel Unsuspected** | Air + Water |
| 5 | **Bats Scout for Raids** | Moon + Air + Animal |
| 6 | **Entrancing Apparitions** | Moon + Air |
| 7 | **Gift of Power** | Moon |
| 8 | **Elemental Boon** | 0-cost four-element |
| 9 | **Predatory Nightmares** | 0-cost Moon |
| 10 | **Song of Sanctity** | 0-cost Sun |

### Top 5 Major Draft Picks

| # | Card | Why |
|---|------|-----|
| 1 | **Terrifying Nightmares** | Moon + Air |
| 2 | **Bargains of Power and Protection** | Sun + Moon + Fire + Air |
| 3 | **Sky Stretches to Shore** | Sun + Moon + Air |
| 4 | **Instruments of Their Own Ruin** | Air + Fire + Animal |
| 5 | **Wrap in Wings of Sunlight** | Sun + Air |

## Adversary Matchup Matrix

| Adversary               | Rating | Note                                                     |
|-------------------------|--------|----------------------------------------------------------|
| Habsburg Livestock      | ★★★★☆  | Reclaim Loop friendly                                     |
| Russia                  | ★★★★☆  | Reclaim Loop friendly                                     |
| **France-Plantation**   | ★★☆☆☆  | Early-pressure hurts Reclaim Loop                         |
| **Brandenburg-Prussia** | ★★☆☆☆  | Early-pressure                                            |
| **England (bot track)** | ★★☆☆☆  | Push-blocks-build interactions wasted                     |
| Others                  | ★★★☆☆  | `[VERIFY]`                                               |

## Board / Map Configuration

`[VERIFY]`. Land #6 + #7 starting placement — boards where these lands are well-positioned.

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Treating Strife as defensive.** Dahan-suppression means *your* Dahan can't retaliate near Incarna.
2. **Bouncing the Incarna back and forth on your own board.** Leaves Dahan permanently sidelined.
3. **Missing the right-innate tempo.** Without Air, you deal zero damage.
4. **Attempting the Reclaim Loop without committing.** Underplaying T1–T2 while also drafting cards wastes the plan.
```

## Synergy Partners (Multiplayer)

- **Ocean combos strongly** (Fotsalot): combo with Ocean to send endless waves of Invaders off to drown.
- **Dahan-protectors elsewhere** — compensate for Voice's Dahan-silencing near Incarna.
- **Air donors** (Lightning, Sun-Bright Whirlwind) — extend Mind-Shattering Song range.
- **Greedy / aggressive partners** — Voice contributes high fear + positional control.

## Source Notes

- **Mechanics**: `data/references/wiki/wandering-voice.json` (Wiki-parsed 2026-04-23).
- **Openings**: Wiki Builds section + [Reddit Spotlight](https://www.reddit.com/r/spiritisland/comments/1ceduyf/).
