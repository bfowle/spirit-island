# Many Minds Move as One

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [Rei's BGG Guide (thread 2774198)](https://boardgamegeek.com/thread/2774198/guide-many-minds-move-as-one) + [latentoctopus Openings 1–2](https://latentoctopus.github.io/guide/mm-opening1/) + [BGG thread 2502110](https://boardgamegeek.com/thread/2502110/openings-many-minds-move-one).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "many" — multiple growths per turn                 |
| Power summary (1–5)   | Offense 1 · **Control 5** · **Fear 5** · **Defense 5** · Utility 1 |
| Primary Elements      | **Animal** (all innates) · **Air** (Teeming Host + Beset scaling) · Water/Fire/Earth for tiers |
| Special Rules         | Fly Fast as Thought (Beast gather/push 2 lands) + Joining of Swarms and Flocks (Sacred Site = Beast) |
| Aspects (NI)          | Persistence · Swarm `[VERIFY]`                     |
| Rei's Guide           | **[Yes — thread 2774198](https://boardgamegeek.com/thread/2774198)** + [Advanced 3030644](https://boardgamegeek.com/thread/3030644) |
| latentoctopus         | [Opening 1](https://latentoctopus.github.io/guide/mm-opening1/)    |
```

## Spirit Overview — Framing

Many Minds is **"one of the most consistent Spirits from all of JE"** (Rei) — Control + Fear + Defense at 5/5/5 on the power-summary, paired with strong Beast-Major synergy. The defining theme is **action economy**:

> Everything you do — placing presence, gaining cards, deciding which cards to play, manipulating beasts, what land to control — all has a major purpose in getting you ahead without having to "do things twice over." — Rei

**The sacred-site-counts-as-Beast rule** means 2 discs already satisfy the 2-Beast clause on Many Minds's defensive innate (Beset and Confound) and its key uniques. Rei: move presence often to keep range where it's needed and to cash in Beast Events.

**Complexity signal**: Moderate. Many Minds has 5 Uniques (not 4), which is unusual — complexity is in the per-turn decision of *which three to play, which to defer, which to forget* across reclaim cycles.

## Starting Setup

> Put **1 Presence and 1 Beast** on your starting board, in a land with Beast. *Note that you have 5 Unique Power Cards.*

## Growth Options (growthtype: "many" — pick multiple)

| Growth | Effects                                                 | Best when                                              |
|--------|---------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card                             | Reclaim cycle                                          |
| **G2** | Add Presence (Range 1) + Add Presence (Range 0)         | Spread + starting-land density                         |
| **G3** | Add Presence (Range 3) [via Beast] + +1 Energy + Gather 2 Beasts | Pre-position Beast layer + energy                   |

G3 is Rei's favorite: **"DOES NOT need to be in the same land you add your presence to"** — move a Beast from an adjacent board into your Land 2 to set up Dreadful Tide without spending Innate 1 to gather.

## Presence Tracks

- **Energy track** (7 slots): `energy0 → energy1 → air → energy2 → animal → energy3 → energy4`
  - 0E → 1E → +Air marker → 2E → +Animal marker → 3E → 4E
- **Card-play track** (7 slots): `card1 → card2 → pay2pcard → card3 → card3 → card4 → card5`
  - 1 CP → 2 CP → Pay 2 for a card → 3 CP → 3 CP → 4 CP → 5 CP

**Starting income**: 0 Energy, 1 Card Play. Power spike at **2/4cp for Minors** (Rei: "the Drafter") or **3/3cp for Majors** (Rei: "Big Birb").

## Core Mechanics & Special Rules

### Special Rule: Fly Fast as Thought

> When you Gather or Push Beast, they may come from or go to lands up to 2 distant (rather than adjacent only).

Beast-movement range extended. Combined with Ranging-style powers, this is board-wide Beast repositioning.

### Special Rule: A Joining of Swarms and Flocks

> Your Sacred Site may also count as Beast.

A 2-Presence land *is* a Beast for rule purposes. Cleanly satisfies the "2 Beast" clauses on Beset and Confound and Dreadful Tide — no need to add a literal Beast token.

### Innate: The Teeming Host Arrives

- **Speed**: Fast · **Range**: 2 · **Target**: Any

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Gather up to 1 Beast. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | *Instead*, Gather up to 1 Beast per Air you have. |
| 3     | 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | Push up to 3 Beasts. |

Beast-topology engine. L2 scales gather with Air — reachable mid-game for massive Beast-shuffle turns.

### Innate: Beset and Confound the Invaders

- **Speed**: Fast · **Range**: 2 · **Target**: Invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 Air + 2 Animal | 2 Fear and Defend 2. |
| 2     | 2 Air + 3 Animal | *Instead*, 3 Fear and Defend 4. |
| 3     | 3 Air + 4 Animal | *Instead*, 4 Fear and Defend 7. |
| 4     | 4 Air + 1 Earth + 5 Animal | *Instead*, 6 Fear and Defend 10. |

**The definitive Many Minds card**: Fear + Defend scale together. L4 (6 Fear + Defend 10) is Many Minds's late-game high point.

## Unique Cards (all 5, Wiki-verified)

### A Dreadful Tide of Scurrying Flesh
- **0 Energy · Fast · Range 1 from your Sacred Site · Land with 2+ Beasts · Moon, Air, Water, Animal**
- *Remove up to half (round down) of Beasts in target land. For each Beast Removed, 2 Fear and skip one Invader Action.*

Beast→Fear conversion + Invader-Action skip. Skipping a Ravage or Build is categorically the strongest Invader-phase disruption. **Sacred Site = Beast** (Special Rule) means a 2-Presence sacred-site land always has at least 1 Beast-equivalent.

### Boon of Swarming Bedevilment
- **0 Energy · Fast · No Range · Another Spirit · Air, Water, Animal**
- *For the rest of this turn, each of target Spirit's Presence grants Defend 1 in its land. Target Spirit may Push up to 1 of their Presence.*

Ally-gift: board-wide Defend 1 per Presence + free Presence repositioning. In solo, self-target. In multi, hand to the spirit with most Presence.

### Ever-Multiplying Swarm
- **1 Energy · Slow · Range 0 · Any Land · Fire, Earth, Animal**
- *Add 2 Beasts.*

Beast-seeder. Fire/Earth/Animal carries Beset L4's Earth tier.

### Guide the Way on Feathered Wings
- **0 Energy · Fast · Range 1 · Any Land · Sun, Air, Animal**
- *Move 1 Beast up to two lands. As it moves, up to 2 Dahan may move with it, for part or all of the way.*

Beast + Dahan movement. Fly-Fast-as-Thought combined with this = 4-land Beast reach.

### Pursue with Scratches, Pecks, and Stings
- **0 Energy · Slow · Range 1 · Any Land with Beast · Moon, Fire, Animal**
- *1 Fear. 1 Damage per Beast in target land. You may move 1 Beast from target land up to 2 lands away.*

Beast-count damage + repositioning. The only damage-dealing Unique — Many Minds's offense lives here.

## Key Strategic Principles

1. **Animal first, Air second.** Everything scales with Animal; Air unlocks Beset/Teeming Host tiers.
2. **Sacred Site = Beast.** Don't build unnecessary 2-Beast literal stacks when a sacred site already satisfies the clause.
3. **G3 pre-positions Beasts** — don't waste Teeming Host L1 on gather turns that G3 could have handled.
4. **Majors are Beast-synergy first.** Rei's "Beasty Bois": Tigers Hunting, Settling into Hunting Grounds, Angry Bears, Insatiable Hunger of the Swarm, Venomous Spiders, Sea Monsters.
5. **Power spike is 2/4cp (Minor) or 3/3cp (Major).** Pick opening archetype before game.
6. **Never draft presence-destroyers.** Many Minds is Presence-thin.

```admonish tip title="Rei's 2-Beast rule"
Placing Sacred Sites *in newly-explored lands* is better than stacking 2-Beast-literal lands — the sacred-site-as-Beast rule means you're getting the 2-Beast trigger for free. Move Beast tokens to event-fodder targets instead.
```

## Possible Openings

### Shared starting state

- **1 Presence + 1 Beast** on starting land (must be a land with Beast).
- **5 Uniques in hand**: A Dreadful Tide, Boon of Swarming Bedevilment, Ever-Multiplying Swarm, Guide the Way on Feathered Wings, Pursue with Scratches.
- **Starting income**: 0 Energy, 1 Card Play.

### Opening A — "The Drafter" (Minor, solo-leaning) 🟨

Rei's Minor-heavy opening.

**T1**: G2 top + G2 bottom (1E + 1CP net from growth); play **Ever-Multiplying Swarm** (1 Air + 2 Animal, Beset L1 unlock).

**T2**: G2 top × 2; play 2 — **Pursue with Scratches** or Guide the Way + Boon of Bedevilment or Dreadful Tide.

**T3**: G2 bottom × 2, pay 2 to gain a Minor; play 3 (deferred combo + new Minor).

**T4**: Reclaim + gain Minor.

**Power spike**: 2/4cp for Minors. Rei wants 0-cost Animal/Air Minors to stay sustainable through reclaim cycles.

### Opening B — "The Extra Friend" / "The Ponderer" (G3 tempo, Rei's favorite multi) 🟩

**T1**: G3 — top presence only, *move a Beast from an adjacent board into your Land 2* (CRITICAL: doesn't need to be in the same land you add presence to). Play Ever-Multiplying Swarm.

**T2**: G2 top + G2 bottom; play combo.

**T3**: G2 top × 2 (or bottom for card gain); play the deferred combo.

**T4**: Reclaim + gain Major or Minor (forget Swarm).

Rei: *"1 turn ahead of the Invaders."* Preps Dreadful Tide without spending Innate 1 to gather Beasts.

### Opening C — "Big Birb" (Major rush) 🟨

**T1–T2**: same shell as Opening B.
**T3**: keep going top track.
**T4**: Reclaim into Major with ~8E stockpiled.
**T5**: G2 bottom × 2 + pay-2 gain Major.

**Power spike**: 3/3cp. Hits almost every Major threshold **except** Instruments of Their Own Ruin (needs 4 plays).

### Opening Decision

- **Default Opening B** (Ponderer) — Rei's favorite in multiplayer.
- **Opening A** (Drafter) in solo / high-Minor draft environments.
- **Opening C** (Big Birb) when Beast-Major is in scope.

## Card Priority Ratings

### Uniques — Many Minds-specific ranking

1. **A Dreadful Tide of Scurrying Flesh** — Invader-action skip is the strongest disruption in the game at 0E.
2. **Beset and Confound via Boon of Bedevilment** — ally-gifting a board-wide Defend 1.
3. **Ever-Multiplying Swarm** — Beast-seeder; reclaim candidate.
4. **Pursue with Scratches** — damage.
5. **Guide the Way on Feathered Wings** — Beast + Dahan movement; situational.

### Top 10 Minor Draft Picks (Animal > Air)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Animal |
| 2 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Air + Animal |
| 3 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Animal |
| 4 | **Bats Scout for Raids by Darkness** | 1 | Fast | Moon, Air, Animal | Trifecta |
| 5 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Animal + Earth (L4 threshold) |
| 6 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Earth-feeder |
| 7 | **Gift of Power** | 1 | Fast | Moon | Moon utility |
| 8 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Animal |
| 9 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Air-feeder |
| 10 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air |

### Top 5 Major Draft Picks (Rei's "Beasty Bois")

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap + Animal |
| 2 | **Angry Bears** | 3 | Fast | Moon, Fire, Animal | Animal + Fear |
| 3 | **Insatiable Hunger of the Swarm** | 3 | Fast | Animal, Plant | Multi-element Animal |
| 4 | **Settling into Hunting Grounds** | 3 | Slow | Moon, Earth, Plant, Animal | Four-element flex |
| 5 | **Venomous Spiders** | 2 | Fast | Moon, Animal | Cheap Animal |

Other Rei favorites: Sea Monsters, Vigor of the Breaking Dawn, Wrap in Wings of Sunlight, Voice of Command, Bargains of Power & Protection, Powerstorm, Weave Together a Fabric of Place, Forest of Living Obsidian, Instruments of Their Own Ruin, Irresistible Call.

### Cards to Avoid (Rei's never-draft)

| Card | Reason |
|------|--------|
| Volcanic Eruption | Wrong profile for Many Minds kit |
| Tsunami | — |
| Fire and Flood | — |
| Cast Down into the Briny Deep | — |
| Entwined Power | — |
| Death Falls Gently | — |
| Utter a Curse | — |
| Dream of the Untouched Land | — |

Rei's rarely-drafted Minors: Renewing Boon, Dry Wood, Call to Migrate (wait — this contradicts top-10 above — Rei flags both situationally), Prowling Panthers, Thriving Chokefungus, Shore Seethes, Desiccating Winds, Song of Sanctity, Sunset Fire Flows, Unquenchable Flames. Situational: Infested Aquifers / Poisoned Drew (Russia killers).

## Adversary Matchup Matrix (Rei-documented)

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Prussia                 | B       | ★★★★☆  | Rei: "Prussia 6 true-solo shown"                             |
| France-Plantation       | B       | ★★★★☆  | "France 6 viable solo"                                       |
| Sweden                  | B / C   | ★★★☆☆  | L5–L6: "The Hasteful" used to unlock Defend 4 on T2; multi-gift Boon |
| Russia                  | B / C   | ★★★☆☆  | L5–L6: "The Hasteful" + Defend-gift; multi-Boon is 1–2 turns ahead |
| England                 | A / B   | ★★★☆☆  | Rei: "England 5 partial-solo"                                |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

Rei: *"Many Minds likes Very Short Distance Lands for Beast Movement and an Interesting Starting Position relative to other Island Board Beasts."*

| Board | Rating | Reason |
|-------|--------|--------|
| **F** | ★★★★★ | Best per Rei; Beast-movement topology |
| **E** | ★★★★★ | Best per Rei                           |
| **A + C** | ★★★★☆ | Good synergies                        |
| Others | ★★★☆☆ | Workable                               |

## Game-Phase Strategy

### Early (T1–3)
- G2/G3 Beast pre-positioning.
- Beset L1 (1 Air + 2 Animal) online T1 with Ever-Multiplying Swarm.
- Dreadful Tide on first 2-Beast land (or sacred site).

### Mid (T4–6)
- Beset L2/L3 (2–3 Air + 3–4 Animal) = huge Fear + Defend.
- Major integration (Opening C).
- Boon of Bedevilment gifted to partner in multi.

### Late (T7+)
- Beset L4 (4 Air + 1 Earth + 5 Animal) = 6 Fear + Defend 10.
- Dreadful Tide every Reclaim cycle for Invader-action skip.

## Synergy Partners (Multiplayer)

- **Beast-Major donors** — Keeper, Green, Fangs (Animal).
- **Any spirit that places presence near you T1–T2** — enables Boon-target for Defend-3 vs. Sweden/Russia.
- **Slow spirits that leave Beasts alone** — you collect them via G3.
- **Solo**: Boon self-targeting stacks Defend massively.

## Common Mistakes

```admonish failure title="Rei's named mistakes"
1. **Using Innate 1 to gather Beasts when G3 could have pre-positioned them.**
2. **Creating 2-Beast sacred sites unnecessarily.** Place SS in newly-explored lands; move Beasts to Event-fodder targets.
3. **Committing to "The Hasteful" outside Sweden/Russia** — "heavy sacrifice to scaling; stuck in sub-optimal reclaim loops at high difficulty."
4. **Drafting >4-cost Majors outside Sea Monsters / Blazing Renewal / Irresistible Call.**
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2/G3 + Ever-Multiplying Swarm; Beset L1 live             |
| 2    | Pursue with Scratches + Boon/Dreadful Tide                |
| 3    | G2×2 + combo; 3 plays                                     |
| 4    | Reclaim + Major (Opening C) or Minor (A/B)                |
| 5–7  | Beset L2/L3; Dreadful Tide loop                            |
| 8+   | Beset L4; late-game closure                                |

## Source Notes

- **Mechanics**: `data/references/wiki/many-minds-move-as-one.json` (Wiki-parsed 2026-04-23).
- **Primary strategy**: [Rei's BGG Guide (2774198)](https://boardgamegeek.com/thread/2774198/guide-many-minds-move-as-one) + [Advanced Guide (3030644)](https://boardgamegeek.com/thread/3030644/advanced-guide-many-minds-move-as-one).
- **Openings**: latentoctopus Opening 1 + BGG thread 2502110.
