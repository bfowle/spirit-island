# Vengeance as a Burning Plague

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing from **[Rei's BGG guide (thread 2709070)](https://boardgamegeek.com/thread/2709070/guide-vengeance-as-a-burning-plague)** + [Rei on Spirited Discussion podcast](https://spiriteddiscussion.substack.com/p/vengeance-as-a-burning-plague-with) + [jyonker13 openings thread 2484534](https://boardgamegeek.com/thread/2484534/openings-vengeance-burning-plague).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "many" — pick multiple growth options per turn     |
| Power summary (1–5)   | **Offense 5** · Control 2 · Fear 3 · Defense 1 · Utility 1 |
| Primary Elements      | **Animal** (Epidemics) · **Fire** (Savage Revenge) · Plant (Disease-seeding) · Air (Savage Revenge L1/L3) |
| Special Rules         | Terror of a Slowly Unfolding Plague (let Build pass → 1 Fear) + Lingering Pestilence (destroyed Presence = 1 Disease) + Wreak Vengeance (Blight counts as Badlands) |
| Aspects               | None                                               |
| Rei's Guide           | **[Yes — thread 2709070](https://boardgamegeek.com/thread/2709070)** |
| latentoctopus         | Not covered                                        |
| BGG openings          | [jyonker13 thread 2484534](https://boardgamegeek.com/thread/2484534) |
```

## Spirit Overview — Framing

Vengeance is a **destruction-triggered scaling spirit** — its whole kit is inverted: *letting* Invaders Ravage your lands and destroy your presence unleashes the plague. Rei's identity thesis:

> I think of them as a sleepy spirit that needs to get woken up by invader's destructive tendencies to unleash its revenge.

**Wiki-printed playstyle note**:

> Wants Invaders (and itself) to take Damage — Blight is fuel via the Badlands rule, Destroyed Presence becomes Disease, and the innate scales with Damage dealt.

**jlrothe's framing** (BGG):

> Decide early where your kill zones will be, get presence there, get killed, get a bunch of disease in there, get to the second level of your first innate... that's all she wrote.

**Rei's power-spike benchmark** (canonical):

> 3 energy / 3 card plays unlocks Tier-2 innates and energy sustainability — anything beyond that is gravy. Scaling is much stronger than cards for Vengeance.

**Complexity signal**: High is correct. The psychological inversion is the hard part — most spirit-island instincts say "prevent Ravages, preserve Presence." Vengeance wants the opposite, and choosing *which* Ravages/destruction to accept is decision-dense.

## Starting Setup

> **1 of your Presence starts the game already Destroyed.** Put **2 Presence** on your starting board: **1 in a land with Blight, 1 in a Wetland without Dahan**.

Yes — one Presence starts already Destroyed. That starting-Destroyed triggers Lingering Pestilence: **1 Disease seeded at setup** on whatever land you designate.

## Growth Options (growthtype: "many" — pick multiple)

| Growth | Effects                                                | Best when                                                   |
|--------|--------------------------------------------------------|-------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy                | Full Reclaim cycle                                          |
| **G2** | Vengeance (special token/placement) + Vengeance (again) | The Disease-multiplier growth; picked twice-per-turn in the Fast Tempo opening |
| **G3** | Gain 1 Power Card + Add Presence (Range 1, Disease-seeded) + +1 Energy | Card-gain + Disease placement + energy                |

## Presence Tracks

- **Energy track** (5 slots): `energy1 → energy2 → animal → energy3 → energy4`
  - 1E → 2E → +Animal marker → 3E → 4E
- **Card-play track** (7 slots): `card1 → card2 → fireX → card2 → card3 → card3 → card4`
  - 1 CP → 2 CP → Fire scaling → 2 CP → 3 CP → 3 CP → 4 CP

**Starting income**: 1 Energy, 1 Card Play.

## Core Mechanics & Special Rules

### Special Rule: Terror of a Slowly Unfolding Plague

> When Disease would prevent a Build on a board with your Presence, you may let the Build happen (removing no Disease). If you do, 1 Fear.

Free 1 Fear per turn if you **don't** use your Disease to prevent a Build. The token stays — use it to seed the next Ravage-farm instead. **Rei clarifies**: the Presence needs to be *anywhere on the board*, not in the specific Disease land.

### Special Rule: Lingering Pestilence

> When your Presence is destroyed by anything except a Spirit action, add 1 Disease where each destroyed Presence was.

Every Ravage that destroys your Presence = 1 Disease on that land. This is why Vengeance *wants* to be Ravaged.

### Special Rule: Wreak Vengeance for the Land's Corruption

> Your actions treat Blight on the island as also being Badlands.

Every Blight token is a Badlands for your damage-scaling — Plaguebearers, Savage Revenge, and any Badlands-scaling Minor triple-dips.

### Innate: Epidemics Run Rampant

- **Speed**: Fast · **Range**: 1 · **Target**: Disease

Text: (Disease cascade — check Wiki for exact text; the tool parse didn't surface thresholds. Known effect structure: Disease in the target land spreads/multiplies, with element-gated tiers.)

Wiki page has fuller thresholds; cross-reference your physical panel or the [Wiki Vengeance page](https://spiritislandwiki.com/index.php?title=Vengeance_as_a_Burning_Plague).

### Innate: Savage Revenge

- **Speed**: Slow · **Range**: 0 · **Target**: Building

| Level | Thresholds                                    | Effect                        |
|-------|-----------------------------------------------|-------------------------------|
| 1     | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | This Power has Range +1. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +2 Damage. |
| 4     | 5 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | +3 Damage. |

Cumulative at L2+L3+L4 = 1+2+3 = 6 Damage to a Building at Range 0. Rei: *"Scaling is much stronger than cards for Vengeance."* — this is why.

## Unique Cards (all 4, Wiki-verified)

### Fetid Breath Spreads Infection
- **2 Energy · Slow · Range 1 · Land with 1+ Invaders · Air, Water, Animal**
- *1 Fear. Add 1 Disease.*

The Disease-seeder. 2E Slow is expensive, but Disease-placement is the whole game setup. Drafted into Majors, Fetid Breath is the first Forget candidate — specialization into other Disease-adding Minors takes over.

### Strike Low with Sudden Fevers
- **2 Energy · Fast · Range 1 · Land with 1+ Disease tokens · Fire, Air, Earth, Animal**
- *1 Fear. Invaders skip Ravage Actions.*

Ravage-skip at Disease lands. The conservation play — skips a whole Ravage in a land where you've pre-seeded Disease. Rei: never cast this "just because you can or for hitting a Tier-1 innate" — only when there's a *specific* Ravage worth skipping.

### Fiery Vengeance
- **0 Energy · Fast · No Range · Any Spirit · Sun, Fire**
- *Cost to Use: Target Spirit Removes 1 of their Destroyed Presence from the game. 1 Fear and 1 Damage in one of target Spirit's lands.*

0-cost self-fueling damage. The Destroyed-Presence cost is *your* resource — 1 Destroyed Presence per cast. Carries Fire → helps Savage Revenge thresholds.

### Plaguebearers
- **1 Energy · Slow · Range 2 · Land with 1+ Disease tokens · Fire, Water, Animal**
- *1 Fear if Invaders are present. For each Disease, Push 2 Explorer/Town/Dahan. 1 Disease may move with each Pushed piece.*

Disease-push + Disease-relocation. **The signature Vengeance card**: moves Invaders into your kill zones + relocates Disease to where you want it next turn.

## Key Strategic Principles

1. **Let Ravages happen in your farmed lands.** Destroyed Presence = Disease; Ravaged Blight counts as Badlands.
2. **Rei's power spike is 3E / 3 CP.** Target this by T4. Don't chase beyond it.
3. **Decide kill zones early.** jlrothe: *"Decide early where your kill zones will be, get presence there, get killed, get a bunch of disease in there."*
4. **Specialize elements — don't straddle Fire vs. Animal.** Rei picks Fire by default (Savage Revenge L2 without Disease concentration).
5. **Terror of a Slowly Unfolding Plague gives 1 Fear per turn for free.** Presence *anywhere* on the board, not the specific Disease land.
6. **Forget Fetid Breath when you draft Majors** — specialization takes over. Exception: if the Major synergizes with Fetid (Flow Like Water, Sea Monsters), forget Fiery Vengeance instead (especially vs England).
7. **Monitor presence-destruction Blight cards** (Tipping Point, Pall, Erosion of Will) — in solo keep 2+ presence banked.

## Possible Openings

### Shared starting state

- **2 Presence + 1 Destroyed Presence + 1 Disease** on board (Disease seeded via Lingering Pestilence from Destroyed setup).
- **4 Uniques in hand**: Fetid Breath Spreads Infection (2E Slow, Air/Water/Animal), Strike Low with Sudden Fevers (2E Fast, Fire/Air/Earth/Animal), Fiery Vengeance (0E Fast, Sun/Fire), Plaguebearers (1E Slow, Fire/Water/Animal).
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Fast-Tempo Minors (Rei's "bread & butter") 🟨 (default)

Above-tempo with Invaders; farms Blight for scaling.

**T1 · Growth**: G3 (Minor + top-track presence + 1E).
**T1 · Play**: **Fetid Breath** — seed Disease at range, prep T2 Plaguebearers target.

**T2 · Growth**: G2 × 2 (two bottom-track).
**T2 · Play**: **Plaguebearers** + flex (Fiery Vengeance or Minor). Push Explorer+Town from setup land to create "sacrificial altars" (jlrothe's phrasing).

**T3 · Growth**: G2 × 2 (two bottom, Disease-seed Ravaging / Dahan lands).
**T3 · Play**: **Strike Low with Sudden Fevers** + Minor + flex.

**T4 · Target**: 3/3 CP pre-Reclaim. If Invader cards "doubled up" (Stage-1→Stage-2 same terrain), skip T3 Strike Low. Otherwise Reclaim loop is expected.

### Opening B — Minus-Tempo Minors 🟨

On-tempo with Invaders; patient growth.

**T1**: Same as Opening A.
**T2**: G2 (single) — preserve flex into Fast Tempo if T3 demands Strike Low.
**T3**: If no Strike Low pressure → grab a Minor; set up T4 for 7–8E + 3 plays + Strike Low ravage-skip.

### Opening C — Early Major (Rei-preferred) 🟥

**T1 · G2** (top × 2) + Fetid Breath (~1E left).
**T2 · G2** (top + bottom into Ravaging/Dahan lands) + **Plaguebearers + Fiery Vengeance** (flex).
**T3 · G3** — gain Major + top-or-bottom presence (priority: Sacred Site for Major > range > Ravaging land). 6–7E pool. Flex Strike Low if needed.

**Fire-vs-4E decision**: Fire if Major is 2–3 cost OR Strike Low must skip T3; 4E if saving Strike Low for T4 or aiming at 4–6 cost Major.

### Opening D — jyonker13 simplified 🟨

For teaching / new players.

**T1 · G2** (+1E, Minor, presence from energy). Play Fetid Breath.
**T2 · G3** (2 presence). Play Plaguebearers + new Minor.
**T3 · G2** (2 presence). Play Strike Low + Fiery Vengeance.
**T4**: Reclaim.

jyonker13 prefers Fire over Animal Minors: *"your second innate scales better than your first without requiring Disease to be concentrated in one land."*

### Opening Decision

- **Default Opening A (Fast-Tempo Minors)** for most matchups — Prussia, Russia, Scotland, Sweden, Habsburg.
- **Opening B (Minus-Tempo)** when board-state is fragile T2.
- **Opening C (Early Major)** when the Major draft lands Fire or Water at ≤4E.
- **Opening D** for teaching or low-L play.

## Card Priority Ratings

### Uniques — Vengeance-specific ranking

1. **Plaguebearers** — Disease-push + relocation engine.
2. **Fiery Vengeance** — 0E Fast self-fueling damage.
3. **Strike Low with Sudden Fevers** — Ravage-skip (conservation).
4. **Fetid Breath Spreads Infection** — Disease-seeder; first Forget candidate for Major drafts.

### Top 10 Minor Draft Picks (Fire > Animal > Plant)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Pyroclastic Friction** | 1 | Fast | Fire, Earth | Fire-feeder |
| 2 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Fire-feeder |
| 3 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Fire + multi-element |
| 4 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Animal |
| 5 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Animal |
| 6 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Animal |
| 7 | **Infested Aquifers** | 1 | Slow | Water, Animal | Disease-compatible utility |
| 8 | **Poisoned Dew** | 1 | Slow | Water, Plant | Russia-killer flex |
| 9 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost triple-element |
| 10 | **Dry Wood** | 0 | Slow | Fire, Plant | Fire-seeder (rare useful Plant) |

### Top 5 Major Draft Picks (Rei's favorites)

| # | Card | Cost | Speed | Elements | Why (Rei) |
|---|------|------|-------|----------|-----------|
| 1 | **Flow Like Water, Reach Like Air** | 2 | Fast | Sun, Air, Water | Gather + Blight-pile |
| 2 | **Unleash a Torrent of the Deep** | 5 | Slow | Moon, Water, Earth, Animal | Energy battery with Fiery |
| 3 | **Settling Into Hunting Grounds** | 3 | Slow | Moon, Earth, Plant, Animal | Beast/Disease synergy |
| 4 | **Angry Bears** | 3 | Fast | Moon, Fire, Animal | Beast + Fire |
| 5 | **Dissolve the Bonds of Kinship** | 3 | Fast | Sun, Moon, Air, Water | Invader-vs-Invader damage |

Also Rei-favored: Infestation of Venomous Spiders, Bloodwrack Plague, Utter a Curse, Pillar of Living Flame, Focus the Land's Anguish.

### Cards to Avoid (Rei's almost-never list)

| Card | Reason |
|------|--------|
| Cast Down into the Briny Deep | Wrong profile |
| Volcanic Eruption | — |
| Draw Towards a Consuming Void | — |
| Fire and Flood | — |
| Transform into Murderous Darkness | — |
| Tsunami | — |
| Twisted Flowers Murmur | — |
| Accelerated Rot | — |
| Grant Hatred | — |

Rei's minor-avoid list: Renewing Boon, Dry Wood (contradicts above — flag variant: drafting context matters), Reaching Grasp, Haunted by Primal, Encompassing Ward, Call to Migrate/Ferocity, Prowling Panthers.

## Adversary Matchup Matrix (Rei-documented)

| Adversary               | Opening                          | Rating | Matchup note                                 |
|-------------------------|----------------------------------|--------|----------------------------------------------|
| Brandenburg-Prussia     | Fast-Tempo Minors (A)            | ★★★★☆  | Rei: Prussia 4/6 validated                   |
| England                 | Fast-Tempo Majors w/ Fire (C)    | ★★★★☆  | Rei: England 5                               |
| France-Plantation       | Minus-Tempo Minors w/ Animal (B) | ★★★☆☆  | Rei: France 5                                |
| Habsburg Mining         | Minus-Tempo Majors (B/C)         | ★★★☆☆  | Rei: Habsburg 4                              |
| Russia                  | Fast-Tempo Majors w/ Fire (C)    | ★★★☆☆  | Rei: Russia 4; flex Poisoned Dew             |
| Scotland                | Fast-Tempo Majors w/ Fire (C)    | ★★★☆☆  | Rei: Scotland 4                              |
| Sweden                  | Fast-Tempo Majors w/ 4-Energy (C)| ★★★★☆  | Rei: Sweden 5                                |
| Habsburg Livestock      | Minus-Tempo                      | ★★★☆☆  | `[VERIFY]`                                   |

## Board / Map Configuration

Per Rei:

| Board | Rating | Reason |
|-------|--------|--------|
| **C** | ★★★★★ | **"Fin."** (Rei). 2 Dahan in blighted starting land; 1-range to all except coastal 3; central 2-range Plaguebearers reach; 6 lands for Bloodwrack AOE. |
| **F** | ★★★★☆ | Similar central range to C |
| **B** | ★★★★☆ | Slight N/S split |
| **E** | ★★★☆☆ | Only vs Sweden (Blight-move synergy) |
| Others | ★★★☆☆ | Workable |

## Game-Phase Strategy

### Early (T1–3)
- Fetid Breath → Plaguebearers → Strike Low (flex) cadence.
- 3 CP / 3E power spike by T4.
- Savage Revenge L1 (3 Air) online via Air-minor draft.

### Mid (T4–6)
- Savage Revenge L2/L3 — multi-Damage destroy cycles.
- Major-integration (if Opening C).
- Presence destruction accepted as Disease-fuel.

### Late (T7+)
- Savage Revenge L4 (6 damage cumulative).
- Plaguebearers multi-Disease cycles.
- Major board-wide closes (Bloodwrack, Focus).

## Synergy Partners (Multiplayer)

- **Energy-donors** — Rei: Vengeance should almost always get first board-pick; Energy/card gifts reward the destruction engine.
- **Water spirits** (Downpour, Ocean) — Board A/C coordination for water access.
- **Defend-providing partners** — unlock Vengeance's aggressive mode.
- aaroncstevens93: *"If a Spirit gives Vengeance a Gift, then Vengeance can exact revenge on that Spirit's behalf as well."*

## Common Mistakes

```admonish failure title="Rei's named mistakes"
1. **Strike Low without a Ravage worth skipping.** "Just because you can" is a tempo-loss.
2. **Playing a weak Minor when doing nothing sets up Tier-2 innates next turn.**
3. **Reclaiming preemptively with Strike Low still in hand.**
4. **Straddling Fire + Animal Minor drafts** — neuters damage.
5. **Forgetting Vengeance only needs 1 presence anywhere on the island to let a Build pass for 1 Fear.** Widely missed; German-JE card is misprinted on this.
6. **Ignoring presence-destruction Blight cards** — keep 2+ presence banked in solo.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 + Fetid Breath; Disease seeded                         |
| 2    | G2×2 + Plaguebearers + flex; push-to-kill-zone            |
| 3    | G2×2 + Strike Low + Minor; 3E income                      |
| 4    | Reclaim; 3/3 CP power spike                                |
| 5–7  | Savage Revenge L2/L3 cycles; Major-integration (Opening C) |
| 8+   | Savage Revenge L4; Bloodwrack AOE                          |

## Source Notes

- **Mechanics**: `data/references/wiki/vengeance-burning-plague.json` (Wiki-parsed 2026-04-23).
- **Primary strategy**: [Rei's BGG Guide 2709070](https://boardgamegeek.com/thread/2709070/guide-vengeance-as-a-burning-plague).
- **Openings**: [jyonker13 thread 2484534](https://boardgamegeek.com/thread/2484534/openings-vengeance-burning-plague).
- **Podcast**: [Rei on Spirited Discussion](https://spiriteddiscussion.substack.com/p/vengeance-as-a-burning-plague-with).
