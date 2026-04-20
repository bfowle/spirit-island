# Shadows Flicker Like Flame

```admonish success title="Mechanics Wiki-verified 2026-04-19"
Card data, innate text, special rules, growth options, presence track, power-summary ratings, suggested-draft card text, **and aspect mechanics** below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Only these remain `[VERIFY]`: Play Difficulty (not on Wiki spirit-template), current mindwanderer stats, and board ratings (require play experience).

**⚠️ Expansion-source discrepancy**: Brett initially said Amorphous + Foreboding are from B&C; Wiki clearly lists them as **Promo Pack 2 (Feather and Flame)**. Using Wiki. Please confirm.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Low                                                |
| Play Difficulty       | 1 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick **one** growth option per turn        |
| Power summary (1–5)   | Offense 4 · Control 3 · **Fear 5** · Defense 1 · Utility 1 |
| Primary Elements      | Moon (all innate levels) · Fire (L2+) · Air (L3)   |
| Special Rule          | Shadows of the Dahan — pay 1 Energy to target any Dahan land regardless of Range |
| Aspects               | **Promo Pack 2**: Amorphous, Foreboding · **JE**: Madness, Reach · **NI**: Dark Fire `[VERIFY Amorphous/Foreboding expansion — Brett said B&C, Wiki says Promo Pack 2]` |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
```

## Spirit Overview — Framing

Shadows is a **base-game Fear-5 spirit with low Defense** (per the Wiki's own power-summary ratings). The design-space is: generate maximum Fear per turn, control Explorer movement via Gather/Push, leverage Dahan adjacency for spatial flexibility — and accept that board-clearing damage is someone else's job.

**Wiki-printed playstyle note** (verbatim for accuracy):

> Good at causing Fear and picking off lone Explorers and Towns, containing the Invaders. Not so good at massive damage — may need to rely on allies to handle thoroughly colonized lands. The ability to boost Range gives more flexibility to Range 0 Powers, and can be important in larger games.

**The honest complexity signal**: Low is correct. 4 Uniques, 1 Innate, 1 Special Rule, 3 single-choice growth options. Strategic depth is in *when* to trigger Innate Level 1 (Gather) vs. Level 2 (Destroy) + which Unique to play per turn.

## Starting Setup

> Put 3 Presence on your starting board: **2 in the highest-numbered Jungle and 1 in land #5**.

## Growth Options (growthtype: "one" — pick one per turn)

Each growth option has a "first" and "second" effect, both resolved together when chosen:

| Growth | Effects                                    | Best when                                          |
|--------|--------------------------------------------|----------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card                | Hand is depleted; drafting a Minor this turn       |
| **G2** | Gain 1 Power Card + Add 1 Presence (Range 1) | Want a card *and* placement on nearby land        |
| **G3** | Add 1 Presence (Range 3) + +3 Energy       | Need spatial reach + energy bank                   |

**Note**: Shadows picks *one* of these per turn — unlike multi-growth spirits that pick multiple. This is a tight constraint. Most turns: G2 early (card-heavy); G1 when hand needs refresh; G3 for energy spikes.

## Presence Tracks

As presence leaves each track, these values are revealed (cumulative per-turn gain):

- **Energy track** (6 slots): **0 → 1 → 3 → 4 → 5 → 6 Energy** per turn as track fills.
- **Card-play track** (6 slots): **1 → 2 → 3 → 3 → 4 → 5** cards per turn.

Starting: 1 Energy, 1 CP (implied from track-0 values). Opening the track quickly unlocks 3 Energy (significant for the 1E-per-turn Shadows-of-the-Dahan rule).

## Core Mechanics & Special Rules

### Special Rule: Shadows of the Dahan

> Whenever you use a Power, you may pay 1 Energy to target a land with Dahan regardless of the Power's Range. *(Power Cards or your Innate Powers.)*

**Strategic implication**: the most flexible targeting rule of any base spirit. Budget 1 Energy/turn for this; Dahan preservation directly expands Shadows's reach.

### Innate: Darkness Swallows the Unwary

- **Speed**: Fast · **Range**: 1 (optionally from a Sacred Site) · **Target**: Any land

| Level | Thresholds               | Effect                                                      |
|-------|--------------------------|-------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire          | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">.                                          |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire          | Destroy up to 2 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Explorer destroyed.    |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air  | 3 Damage. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Invader destroyed by this Damage.      |

Key decision: Level 1 *gathers* (repositions) an Explorer without killing — useful to move Explorers out of upcoming Build lands. Level 2+ destroys for fear. Don't over-rush Level 2 if Level 1 solves the land.

## Unique Cards (all 4, Wiki-verified)

#### Concealing Shadows

- **0 Energy · Fast · Range 0 · Any Land · Moon, Air**
- *1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Dahan take no Damage from Ravaging Invaders this turn.*

Free-to-play defensive + 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> every turn. Pairs with Favors Called Due (preserves Dahan to outnumber Invaders).

#### Crops Wither and Fade

- **1 Energy · Slow · Range 0 · Any Land · Moon, Fire, Plant**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Replace 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> with 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** Replace 1 City <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City"> <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City"> with 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">.*

**Downgrade**, not destroy. Softens Ravages + 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per play. A City → Town reduces that land's Ravage damage by 1 and removes a Build upgrade path.

#### Favors Called Due

- **1 Energy · Slow · Range 1 · Any Land · Moon, Air, Animal**
- *Gather up to 4 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. If Invaders are present and Dahan now outnumber them, 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.*

Massive conditional fear spike (3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">). Needs gatherable Dahan + Invader-present target + post-gather Dahan > Invader count.

#### Mantle of Dread

- **1 Energy · Slow · No Range · Any Spirit · Moon, Fire, Air**
- *2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Target Spirit may Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> from a land where it has Presence.*

**Partner-support** (target Any Spirit). In multiplayer, hands a partner a free push on one of their lands. In solo, self-target.

## Key Strategic Principles

1. **Fear output is huge.** Concealing (1) + Crops (2) + Favors (3 conditional) + Mantle (2) + Innate L2 (up to 2) = **up to 10 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> in one turn** at the peak. Shadows has the base game's strongest per-turn fear ceiling by raw card output.
2. **Shadows of the Dahan = spatial superpower.** Budget 1E/turn for range extension. Dahan density across the board directly expands Shadows's reach.
3. **Crops Wither is downgrade-not-destroy.** Use to soften Ravages + bank Fear, not to kill. The "replaced" Invader remains in the land.
4. **Innate Level 1 is positional.** Gather an Explorer out of a Build target → no Town next turn. A single growth option can solve a land via innate L1 without spending cards.
5. **Moon + Fire is load-bearing.** L1 = 2M+1F, L2 = 3M+2F. Draft Moon + Fire Minors aggressively.
6. **Favors Called Due wants Dahan density.** Preserve Dahan (Concealing) so gather + outnumber triggers.
7. **Mantle of Dread targets a Spirit.** In multiplayer, always a partner-help card if their turn is tight.

```admonish tip title="Pro Tip — Favors Called Due math"
Before Slow powers, count: can Favors gather ≥ 4 Dahan into a target land with ≤ 3 Invaders? If yes, that 3 Fear is triggerable and is Shadows's biggest single-card fear spike.
```

## Possible Openings

Three Rei-format opening variants. Each is a turn-by-turn rehearsal for the first 3 turns + a T4 state audit. Pick the variant that matches your adversary + scenario before game start — don't improvise T1. And don't execute the opener on auto-pilot: each turn block below has a **Pause-point** callout flagging the decision that most often gets mis-made on that turn. Cross-reference [Deliberate Play](../../fundamentals/deliberate-play.md) for the universal per-phase checklist.

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

### Shared starting state (all openings)

Verified per `si-rules-check shadows-flicker-like-flame`:

- 3 Presence on board: 2 in highest-numbered Jungle, 1 in land #5.
- 4 Uniques in hand: Concealing Shadows (0E Fast, 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air), Crops Wither and Fade (1E Slow, 1M+1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire+1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant), Favors Called Due (1E Slow, 1M+1A+1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal), Mantle of Dread (1E Slow, 1M+1F+1A).
- **Starting income: 0 Energy, 1 Card Play** (first-uncovered Energy track slot `energy0`, first-uncovered CP track slot `card1`).
- Growth type **"one"**: pick one of G1/G2/G3 per turn. Presence placement reveals **one** track slot; it doesn't reveal both.

### Base-deck invader phase by turn (affects Ravage-protection cards)

| Turn | Explore | Build | Ravage | Consequence for opener prose                                      |
|------|---------|-------|--------|-------------------------------------------------------------------|
| 1    | ✓       | —     | —      | **Ravage-protection is dormant T1.** Concealing's "Dahan take 0 Damage from Ravaging" = null effect this turn. Its Fear + elements are the only T1 material value. |
| 2    | ✓       | ✓     | —      | **Still dormant** for Ravage-protection. First Build triggers; Mantle/Crops pushes matter for Build prevention.  |
| 3    | ✓       | ✓     | ✓      | **First Ravage.** Concealing's protection now material IF played on the Ravage target land. |

### Growth-option decoder (T1 income branches)

| Growth | Effects | Post-growth T1 state (from 0E/1CP) |
|--------|---------|------------------------------------|
| **G1** | Reclaim + Gain 1 Minor (no presence placed) | 0E / 1CP (unchanged); 1 Minor gained |
| **G2, reveal CP track (card2)** | Gain 1 Minor + Place 1 Presence Range 1 | 0E / **2CP**; 1 Minor gained |
| **G2, reveal Energy track (energy1)** | Gain 1 Minor + Place 1 Presence Range 1 | **1E** / 1CP; 1 Minor gained |
| **G3, reveal CP track (card2)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **3E / 2CP** |
| **G3, reveal Energy track (energy1)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **4E** / 1CP (1 slot + 3 growth effect) |

### Fast-phase element ceiling — load-bearing constraint

Shadows's innate Darkness Swallows the Unwary is **Fast**. Fast innates check thresholds using only elements from Fast cards played before the innate resolves. Mantle/Crops/Favors are all Slow — their elements **arrive after the Fast innate's resolution window** and cannot trigger it.

**Fast-phase Moon/Fire available from Uniques alone:**

- Concealing Shadows — the only Fast Unique — gives 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air.
- Fast-phase Moon ceiling from Uniques = **1**. Fast-phase Fire from Uniques = **0**.
- L1 needs 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire. **Shadows cannot fire L1 in Fast phase from Uniques alone.**

**Minimum requirement for L1 to fire**: a drafted Fast Moon+Fire Minor. Wiki-suggested candidates: **Land of Haunts and Embers** (0E Fast, 1M+1F+1A — drafts + plays in the same turn) or **Visions of Fiery Doom** (1E Fast, 1M+1F — needs energy). Hitting L1 T1 is entirely draft-dependent and not guaranteed.

### Opening A — G3-via-CP Double-Card Fear Opener 🟨 (default)

**When to pick this**: default against **Brandenburg-Prussia, England, Scotland, Habsburg Livestock** — fear-friendly adversaries. G3-via-CP is the only T1 growth choice that affords a 0E + 1E two-card play from a 0E start, giving the highest T1 Fear + material-effect output.

**Target arc**: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> T1 · L1 fires first at T3 after reclaim + Fast Moon+Fire Minor · Terror 2 flip T5–T6.

#### Turn 1

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G3</strong> revealing CP track — gain <strong>+3 Energy</strong> + 1 Presence Range 3</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>3E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> <img class="si" src="/spirit-island/theme/icons/speed-fast.svg" alt="Fast"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Mantle of Dread</strong> (self) <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> · 1E spent</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear generated</span> <span class="si-pool"><strong>2E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I — Explore only</span></dd>
</dl>


- **Growth**: **G3, revealing CP track** (uncover `card2`). Post-growth: **3E / 2CP**. 1 Presence placed Range 3; "+3 Energy" is a this-turn-only effect.
- **Fast phase plays**: **Concealing Shadows** (0E, target one of your Presence lands — jungle #6 or wetland #5).
  - **T1 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Elements 1M+1A into Fast pool. *"Dahan take 0 Damage from Ravaging"* clause is **dormant this turn** (no Ravage until T3). Concealing discards.
- **Fast innate check**: pool = 1M+1A. L1 needs 2M+1F → **does NOT fire T1**.
- **Invader phase**: Explore (Card 1 = Stage I). Explorers arrive in any land adjacent to a City/Town/ocean (coastal). No Build, no Ravage.
- **Slow phase plays**: **Mantle of Dread** (1E, target self).
  - **T1 gain from Mantle**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> from one of your Presence lands — removes 2 Invader units from the board. Elements 1M+1F+1A into Slow pool. Mantle discards.
- **End of T1**: **3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> generated** · 1E spent (Mantle) · **2E banked** · 0 CP left · 4 Presence on board (3 starting + 1 G3) · Hand: 3 Uniques (Concealing + Mantle in discard) · Tracks: CP at card2 (2 CP sustained from now), Energy still at energy0.
- **Pause-point (pre-Mantle-target)**: self-target is the default, but Mantle's target is "Any Spirit" — in multiplayer, check whether pushing from a partner's Presence land solves a bigger T2-Build threat than pushing from yours. Solo: choose the Presence land whose Explorer+Town pair creates the largest T2 Invader-count reduction in the land most likely to Ravage at T3.

#### Turn 2

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing Energy track (<code>energy1</code>) — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">2E banked + 1E income = <strong>3E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Crops Wither</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span> <span class="si-land"><strong>Favors Called Due</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">0 — no Fast cards played · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T2</dt>
  <dd><span class="si-pool">2–5 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 City <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City"> → Town (if targeted)</span> <span class="si-pool">Dahan gathered</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I+II — Build + Explore (first <strong>Build</strong>)</span></dd>
</dl>


- **Growth**: **G2, revealing Energy track** (uncover `energy1` → 1E permanent income from now). Post-growth: **2E banked + 1E income = 3E / 2CP**, +1 Minor drafted.
- **Draft priority for the T2 Minor**: Fast Moon+Fire (Land of Haunts and Embers 0E, Visions of Fiery Doom 1E). Enables T2 Fast innate firing.
- **Fast phase plays (if Fast M+F Minor drafted)**: drafted Minor alone (no Concealing in hand — it's in discard since T1). Fast elements = Minor's contribution (e.g., Haunts 1M+1F+1A).
- **Fast innate check (with Haunts drafted)**: 1M+1F → still short of 2M+1F. **L1 still does NOT fire**. Needs Concealing back in hand (T3 reclaim) plus the Fast M+F Minor to combine for 2M+1F.
- **Slow phase plays**: **Crops Wither and Fade** (1E, Range 0 — target one of your Presence lands with a Town or City) + **Favors Called Due** (1E, Range 1 — target a Dahan-dense Invader-present land).
  - **T2 gain from Crops**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Immediate replace: City→Town OR Town→Explorer in target land. *Material this turn*: the replaced unit is gone; *downstream*: when T3 Ravage hits that land, Damage is 1 lower than it would have been.
  - **T2 gain from Favors**: Gather up to 4 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> into target. *Material this turn*: Dahan positioning for T3+ counterattacks. If post-gather Dahan > Invaders in target, **+3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> conditional**.
- **Slow elements (Fast innate already resolved)**: Crops 1M+1F+1P + Favors 1M+1A+1An = 2M+1F+1A+1P+1An. Darkness Swallows can't use these (Fast innate only checks in Fast phase).
- **End of T2**: **2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> generated** (2 Crops + 0–3 Favors conditional) · 2E spent · **1E banked** · 0 CP left · 5 Presence on board · Hand: 1 Unique (Concealing in discard, Crops+Favors now also in discard; Mantle discarded T1) · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-Favors-target)**: Favors's 3-Fear conditional is **not free** — it needs post-gather Dahan to strictly outnumber Invaders. Before committing, count: Invaders in target + Dahan in R1 of target = enough to gather past Invader count? If no, change target or save Favors for T3.

#### Turn 3

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G1</strong> — Reclaim (all played back to hand) + Gain 1 Minor</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">1E banked + 1E income = <strong>2E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> (target Ravage land w/ Dahan) <img class="si" src="/spirit-island/theme/icons/speed-fast.svg" alt="Fast"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"></span> <span class="si-land">+ drafted Fast Moon+Fire Minor (if in hand) → L1 may fire</span></dd>

  <dt>Key T3 note</dt>
  <dd><span class="si-pool"><strong>First Ravage turn</strong> — Concealing's protection now <em>material</em></span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I+II+III — first <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <strong>Ravage</strong></span></dd>
</dl>


- **Growth**: **G1** — Reclaim (all 3 discarded Uniques back to hand) + Gain 1 Minor. No presence placed. State: **1E banked + 1E income = 2E / 2CP**, hand = 4 Uniques + 2 Minors.
- **Invader state**: **T3 is the first Ravage turn.** Identify the Ravage-target land from the current Invader card's Ravage column (the card that was in Build slot last turn has advanced).
- **Fast phase plays**: **Concealing Shadows** (target the Ravage-incoming land, with Dahan in it, if any) + **drafted Fast Moon+Fire Minor if in hand** (e.g., Land of Haunts).
  - **T3 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + **Ravage-protection now material** — Dahan in target land take 0 Damage from this turn's Ravage. Fast elements 1M+1A.
  - **T3 gain from Haunts (if played)**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns from R2 target. Fast elements 1M+1F+1A.
- **Fast innate check (Concealing + Haunts)**: pool = 2M+1F+2A → **L1 fires for the first time this game**. Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into a land at R1 of your presence or Sacred Site. Fast, so the gather happens **before** the Ravage — can pull an Explorer out of the Ravage target (one fewer Invader taking the Ravage).
- **Slow phase plays**: 1 more Slow card if CP remains — e.g., **Crops Wither** (another City→Town downgrade). Depending on hand: **Mantle of Dread** for +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + another push.
- **End of T3**: **5+ Fear generated** (1 Concealing + 2 Haunts + 2 Crops/Mantle + possible 1-Fear L1 if destroy-variant) · **1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered (L1 innate)** · **Ravage Damage in one land reduced or neutralized** (Concealing or Haunts push) · ~0E / ~0CP depending on plays · 5 Presence on board · Hand: mixed reclaims + drafts · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-Fast-plays)**: Concealing's Ravage-protection is material T3+ **only if the target land actually Ravages this turn AND has Dahan to protect**. If the Ravage target has 0 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">, Concealing's protection clause is still dormant — consider targeting a Dahan-adjacent Ravage land (use Shadows of the Dahan's 1E range extension) or playing Concealing elsewhere for its Fear + elements alone.

#### Turn 4 — state audit

After T3, you should have:

- **Presence**: 5 on board (3 starting + 1 T1 G3 + 1 T2 G2).
- **Energy / CP**: 1E income (from energy1 slot) / 2CP.
- **Engine**: All 4 Uniques available + 2 Minors drafted. L1 firing conditional on Fast M+F Minor in hand.
- **Fear pool**: ~8–12 (solo — Terror 2 should flip around end-T4 or T5, not T3 as the prior opener claimed).
- **Blight**: 0–1 (depends on T3 Ravage outcomes; Concealing + L1 Gather mitigation should keep this low).

**Pivot triggers** (if any of these, deviate from the opener):

- **No Fast Moon+Fire Minor drafted by T3** — L1 firing stays blocked. Keep leaning on Crops + Favors for Fear; L1 is "nice to have" not "required".
- **Ravage target T3 has no Dahan** — Concealing's protection is still dormant; use Concealing purely for Fear + elements, and save the real defense for T4+ when Dahan density increases.
- **Favors never triggers 3-Fear conditional** — Fear pool is running 3–4 under target. Prioritize Minor picks that generate Fear directly (e.g., 2-Fear Moon Minors from the full pool).

### Opening B — G2-via-CP Minor-Draft-Dependent 🟥

**When to pick this**: **Habsburg Mining L5+, Russia L5+**, or long-game matchups where Minor-draft density pays off. Trades T1 Fear output for hand depth. **Gated on drafting a 0-cost Fast Moon+Fire Minor (Land of Haunts and Embers) by T1** — otherwise this opener under-delivers.

**Target arc**: If Haunts drafted T1 → **L1 fires T1** (unique to Opening B among the three) · 2 Minors drafted by T2 · Reclaim T3 → 4 Uniques + 3 Minors in hand · matchup-specific Minor picks scale T4+.

#### Turn 1

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing CP track — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>0E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Best-case Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> + <strong>Land of Haunts and Embers</strong> (drafted 0-cost Minor)</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> · <strong>L1 FIRES T1</strong> → Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered by L1</span> <span class="si-pool">up to 2 pushed by Haunts</span></dd>
</dl>


- **Growth**: **G2, revealing CP track** (uncover `card2`). Post-growth: **0E / 2CP**, 1 Minor drafted.
- **Draft priority T1 Minor**: absolute top pick **Land of Haunts and Embers** (0E Fast, 1M+1F+1A). Second pick: Shadows of the Burning Forest (0E Slow, 1M+1F+1P). If neither appears: Opening B is underpowered T1 — consider pivoting to C (see Opening Decision tree).
- **Fast phase plays (with Haunts drafted)**: **Concealing Shadows** (0E) + **Land of Haunts and Embers** (0E, target a Mountain or Jungle with Invaders for the push rider, else any Invader land for the 2-Fear base).
  - **T1 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Dormant Ravage-protection (no Ravage T1). 1M+1A Fast elements.
  - **T1 gain from Haunts**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns from target land. 1M+1F+1A Fast elements. (Haunts "Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight">" clause is card-text — read the full card; if playing the 0-cost base version, blight consequences apply.)
- **Fast innate check**: pool = 2M+1F+2A → **L1 fires T1**. Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> to a R1 land. The only Opening that gets L1 T1.
- **Slow phase plays**: none (0 CP remaining; both CP used on Fast cards).
- **Invader phase**: Explore only.
- **End of T1**: **3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered (L1)** + up-to-2 units pushed by Haunts · 0E banked · 0 CP left · 4 Presence on board · Hand: 3 Uniques + 0 Minors (Haunts played) · Tracks: CP at card2, Energy still at energy0.
- **Pause-point (pre-Haunts-target)**: Haunts's push clause triggers only on Mountain or Jungle; its base 2-Fear + 2-push works on any Invader land. If target isn't Mountain/Jungle, the push-up-to-2 doesn't fire — pick a Mountain/Jungle Invader land if you have range, else accept the 2-Fear-only output. Also check Haunts's "Add 1 Blight <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> if Blight present" clause on the physical card — some versions add Blight unconditionally.

**Fallback T1 (Haunts NOT drafted)**: Play **Concealing alone**. Fear = 1. L1 does NOT fire (Fast phase elements = 1M+1A). 1 CP unused. This is Opening B underperforming — expect T2–T3 Minor draft to compensate.

#### Turn 2

- **Growth**: **G2, revealing Energy track** (uncover `energy1` → 1E income). Post-growth: **1E / 2CP**, +1 Minor drafted (2 total).
- **Draft priority T2 Minor**: if Haunts was T1, look for Moon+Fire next (Visions of Fiery Doom 1E Fast) to chain L1 firings; otherwise Haunts.
- **Fast phase plays**: Fast cards in hand (Concealing discarded T1 if played; Fast Minors drafted).
  - If drafted Fast M+F Minor this T2 and have 1E to spare: play it. Elements contribute to L1 threshold (but Concealing is in discard — only 1M from the Fast play typically, unless a 2M-on-one-card Minor was drafted).
  - Typically: play nothing Fast T2; L1 defers to T3 reclaim + Fast M+F Minor combo.
- **Slow phase plays**: 1 Slow Unique at 1E — **Crops Wither** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, downgrade) OR **Mantle** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, self-push) OR **Favors** (+0–3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, Dahan gather).
- **T2 material gain**: typically 2–3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + one material board effect (downgrade / push / gather). **L1 does not fire T2 reliably** — Fast pool depends on which Minors were drafted and playable.
- **End of T2**: 2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> · ~0E banked · 0 CP left · 5 Presence · hand typically 2 Uniques + 2 Minors.
- **Pause-point (pre-T2-plays)**: T2 introduces first Build. Check the Invader card's Build column — which terrain Builds? Any land with Town/City on that terrain gets a new Town/City. Crops'ing a Town on that terrain T2 Slow phase (or Mantle pushing from it) pre-empts the Build's upgrade path.

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. State: **~0E + 1E income = 1E / 2CP**, hand = 4 Uniques + 3 Minors.
- **Fast phase plays**: **Concealing + Fast M+F Minor** (Haunts if reclaimed, or Visions if drafted). Elements = 2M+1F+... → **L1 fires T3**.
- **Slow phase plays**: best Fear/material combo from hand. Typically Crops + Mantle, or Crops + Favors if Dahan-outnumber is triggerable.
- **T3 material gain**: 3–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + L1 Gather + 2× Slow-card effects. First turn with Ravage + Concealing protection becomes material.
- **Pause-point (pre-Concealing-target)**: same as Opening A T3 — Ravage-protection is material only on the Ravage target with Dahan. If Ravage lands don't have Dahan, use Concealing purely for Fear + elements.

#### Turn 4 — state audit

- **Presence**: 5 on board.
- **Energy/CP**: 1E income / 2CP.
- **Engine**: L1 firing reliably if Fast M+F Minor in hand; L2 conditional on additional Moon or Fire Minors.
- **Fear pool**: 6–10 (slower than Opening A if Haunts missed T1; comparable if Haunts hit T1).
- **Hand depth**: 7 cards (4 Uniques + 3 Minors) — deepest of the three openers.
- **Strategy note**: Opening B's payoff is **late** — T5+ benefits from Minor-draft breadth for matchup-specific picks (e.g., Dahan-summoning, Blight-removing, Strife-adding). Fear-rush numbers lag Opening A by ~3–4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> through T4.

### Opening C — G3-via-Energy Energy-Bank (Fear-Suppression) 🟥

**When to pick this**: **Sweden L3+, Russia L3+** — where fear-card penalties reduce the value of a Fear-rush opener. Opening C leans on energy depth for (a) aggressive Shadows-of-the-Dahan range extension (1E/use budget), (b) 1E Slow Uniques played every turn, (c) potential Major acquisition via fear-card Events when they do fire.

**Target arc**: Mantle T1 (2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">) · double-Slow T2 · L1 first fires T3–T4 via reclaim + drafted Fast M+F Minor · Terror 2 flip delayed to T6–T7 (accepted).

#### Turn 1

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G3</strong> revealing Energy track — 1E income perm + 3E growth effect</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>4E</strong></span> <span class="si-pool"><strong>1CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Mantle of Dread</strong> (self) <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E · 2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear + push</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">0 — no Fast card · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool"><strong>3E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>
</dl>


- **Growth**: **G3, revealing Energy track** (uncover `energy1` → 1E permanent income). Post-growth: **1E income + 3E growth-effect = 4E / 1CP**.
- **Fast phase plays**: none (CP budget of 1 is best spent on a higher-material-effect Slow card).
- **Fast innate check**: pool = 0 elements. L1 does NOT fire.
- **Invader phase**: Explore.
- **Slow phase plays**: **Mantle of Dread** (1E, self-target).
  - **T1 gain from Mantle**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> from one of your Presence lands. 1M+1F+1A Slow elements (irrelevant for Darkness Swallows since it's Fast).
- **End of T1**: **2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">** · 1E spent · **3E banked** · 0 CP left · 4 Presence on board · Hand: 3 Uniques (Mantle in discard) · Tracks: CP at card1 (still 1 CP), Energy at energy1 (1E sustained).
- **Pause-point (growth choice)**: G3-via-Energy gives deeper Energy income long-term (energy1 permanent = +1E/turn from T2 onward) vs. G3-via-CP (card2 permanent = +1 CP/turn). Over 6 rounds that's 6E vs. 6 extra card plays. Pick based on matchup: fear-suppression adversaries benefit from energy depth (more Shadows-of-the-Dahan range extensions; more Minor cost-curve flexibility); fear-friendly adversaries benefit from CP depth (more Fear-generating card plays per turn). If unsure, G3-via-CP (Opening A) is the tighter fear-rush; G3-via-Energy (Opening C) is the slower, more flexible play.

#### Turn 2

- **Growth**: **G2, revealing CP track** (uncover `card2` → 2 CP from now). Post-growth: **3E banked + 1E income = 4E / 2CP**, +1 Minor drafted.
- **Fast phase plays**: none likely (Concealing in hand but saving for T3 Ravage turn; or play Concealing for 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + element cycle).
  - Optional Concealing T1-style 1-Fear + element-pool play: 1M+1A. Doesn't fire innate alone.
- **Slow phase plays**: **Crops Wither** (1E, Presence-land target with a Town or City) + **Favors Called Due** (1E, R1 Dahan-dense target).
  - **T2 gain from Crops**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + City→Town OR Town→Explorer. (Material THIS turn; reduces next-turn Ravage damage in that land.)
  - **T2 gain from Favors**: Gather up to 4 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> into R1 target; conditional +3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if post-gather Dahan > Invaders.
- **Slow elements**: Crops 1M+1F+1P + Favors 1M+1A+1An = 2M+1F+1A+1P+1An. (Fast innate already resolved with 0 elements; Slow elements not usable here.)
- **End of T2**: **2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">** · 2E spent · **2E banked** · 0 CP left · 5 Presence on board · Hand: 1 Unique (Concealing) + 1 Minor · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-T2-plays)**: T2 is first Build. Same check as Opening A/B — Crops + Favors together handle one Build-threat land (downgrade + gather counter-Dahan). If two lands are about to Build-into-City, one of them is **not handled** — prioritize the worst one; accept the other.

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. State: **2E banked + 1E income = 3E / 2CP**, hand = 4 Uniques + 2 Minors.
- **Fast phase plays**: **Concealing + drafted Fast M+F Minor** if in hand → L1 fires. Otherwise Concealing alone (1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, elements pool 1M+1A, L1 does not fire).
- **Slow phase plays**: 1–2 cards from 3E budget. Typical pick: **Crops + Mantle** for 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + downgrade + push; OR **Crops + Favors** if Dahan-outnumber still triggerable somewhere.
- **T3 material gain**: 4–6 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + material board effects + Ravage-protection on Concealing's target (material T3+).
- **End of T3**: ~0E banked · energy bank depleted for the big-swing turn.
- **Pause-point (T3 Ravage targeting)**: same Ravage-protection check as Openings A/B. Additionally, Opening C's energy-bank advantage lets you spend 1E on Shadows-of-the-Dahan range extension to place a Slow-effect card (Crops / Favors / Mantle) on a non-Presence Dahan land — consider if the highest-leverage target is outside R0.

#### Turn 4 — state audit

- **Presence**: 5 on board.
- **Energy/CP**: ~1E income / 2CP.
- **Engine**: L1 firing conditional on Minor draft (same as Openings A/B); L2 element threshold 3M+2F rarely reached without multiple Moon-heavy Minors.
- **Fear pool**: 4–8 (lowest of three openers; accepted for fear-suppression matchups).
- **Blight**: 0–2.
- **Strategy note**: energy depth will continue paying dividends T5+ — Shadows-of-the-Dahan range extensions multiply over the game. Accept slower Terror; target Terror 2 T6–T7.

### Opening Decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Shadows] --> Adv{Adversary?}
  Adv -->|Brandenburg-Prussia| A[Opening A - Standard Fear]
  Adv -->|England| A
  Adv -->|Scotland| A
  Adv -->|Habsburg Livestock| A
  Adv -->|Sweden L0-L2| A
  Adv -->|Sweden L3+| C[Opening C - Fear-Suppression Resistant]
  Adv -->|Russia| C
  Adv -->|Habsburg Mining L5+| B[Opening B - Minor-Draft Dependent]
  Adv -->|Multi-handed with board-carry partner| A
  Adv -->|Other| A
</pre>

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across all 114 Minor + 98 Major cards in the full deck (Base + B&C + JE + NI), weighted by Shadows Flicker Like Flame's innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/shadows-flicker-like-flame.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/shadows-flicker-like-flame.json) for full scoring + reasons.

- **Primary elements (innate-weighted)**: **Moon** (wt 4.8), **Fire** (wt 3.0), **Air** (wt 0.6)
- **Mid-game energy estimate (T3–T5 avg)**: 4.0E
- **Power summary**: Offense 4 · Control 3 · Fear 5 · Defense 1 · Utility 1
```

### Uniques

The spirit's own 4 Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):

- **Concealing Shadows**
- **Crops Wither and Fade**
- **Favors Called Due**
- **Mantle of Dread**

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Animated Wrackroot** | 0 | Slow | Moon, Fire, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">. **OR** Add 1 Wilds. | elements fire+moon → 7.8 |
| 2 | **Lure of the Unknown** | 0 | Fast | Moon, Fire, Air, Plant | Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements air+fire+moon → 8.4 |
| 3 | **Roiling Bog and Snagging Thorn** | 0 | Fast | Moon, Fire, Water, Plant | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Isolate. Defend 2.</br>1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> does not participate in Ravage.</br>(Check when ra… | elements fire+moon → 7.8 |
| 4 | **Land of Haunts and Embers** | 0 | Fast | Moon, Fire, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. If Blight is present, 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push up to 2 Explo… | elements air+fire+moon → 8.4 |
| 5 | **Rites of the Land's Rejection** | 1 | Fast | Moon, Fire, Earth | Invaders do not Build in target land this turn. 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Town/City or 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Dahan,… | elements fire+moon → 7.8 |
| 6 | **Shadows of the Burning Forest** | 0 | Slow | Moon, Fire, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. | elements fire+moon → 7.8 |
| 7 | **Visions of Fiery Doom** | 1 | Fast | Moon, Fire | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements fire+moon → 7.8 |
| 8 | **Prowling Panthers** | 1 | Slow | Moon, Fire, Animal | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Add 1 Beasts. **OR** If target land has Beasts, Destroy 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. | elements fire+moon → 7.8 |
| 9 | **Sunset's Fire Flows Across the Land** | 1 | Slow | Sun, Moon, Fire, Water | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage. You may pay 1 Energy to deal 1 Damage in an adjacent land. | elements fire+moon → 7.8 |
| 10 | **Unquenchable Flames** | 1 | Slow | Moon, Fire, Earth | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. 1 Damage to Towns/Cities. Invaders do not heal Damage at end of turn. | elements fire+moon → 7.8 |

### Top 5 Major Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |
|---|------|------|-------|----------|--------------------|---------------|
| 1 | **Transform to a Murderous Darkness** | 6 | Slow | Moon, Fire, Air, Water, Plant | Target Spirit may choose one of their Sacred Site. In that land: Replace all their Presen… | elements air+fire+moon → 8.4 |
| 2 | **Settle Into Hunting-Grounds** | 3 | Fast | Moon, Fire, Plant, Animal | Your Presence may count as Badlands and Beasts. (Decide per Presence, per Action.) Your P… | elements fire+moon → 7.8 |
| 3 | **Unearth a Beast of Wrathful Stone** | 5 | Fast | Moon, Fire, Earth, Animal | After the next Invader Phase (on any turn) with no Ravage/Build Actions in target land:</… | elements fire+moon → 7.8 |
| 4 | **Pent-Up Calamity** | 3 | Fast | Moon, Fire, Earth, Plant, Animal | Add 1 Disease and 1 Strife. **OR** Remove any number of Beasts/Disease/Strife/Wilds. For … | elements fire+moon → 7.8 |
| 5 | **Vengeance of the Dead** | 3 | Fast | Moon, Fire, Animal | 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. After Towns/Cities/Dahan are Destroyed in target land, 1 Damage per Town/City/Dah… | elements fire+moon → 7.8 |

### HoSI Beginner Deck Bundle — for reference only

```admonish note title="Not a draft-priority list"
These are the cards shipped with Shadows Flicker Like Flame in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.
```

| Card | Type | Cost | Speed | Elements | Effect (truncated) |
|------|------|------|-------|----------|--------------------|
| **Dark and Tangled Woods** | Minor | 1 | Fast | Moon, Earth, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Defend 3. |
| **Shadows of the Burning Forest** | Minor | 0 | Slow | Moon, Fire, Plant | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. If target land is a Mountain or Jungle, Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town">. |
| **The Jungle Hungers** | Major | 3 | Slow | Moon, Plant | Destroy all Explorers and all Towns. Destroy all Dahan. |
| **Land of Haunts and Embers** | Minor | 0 | Fast | Moon, Fire, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. If Blight is present, 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> and Push up to 2 Explo… |
| **Terrifying Nightmares** | Major | 4 | Fast | Moon, Air | 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 4 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns. |
| **Call of the Dahan Ways** | Minor | 1 | Slow | Moon, Water, Animal | Replace 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> with 1 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">. |
| **Visions of Fiery Doom** | Minor | 1 | Fast | Moon, Fire | 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Town. |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason(s) |
|------|-----------|
| **Land of Haunts and Embers** | adds Blight |
| **Skies Herald the Season of Return** | destroys Presence |
| **Scour the Land** | adds Blight |
| **Devouring Ants** | destroys Dahan |
| **Renewing Boon** | destroys Presence |
| **Solidify Echoes of Majesty Past** | destroys Presence |
| **Pyroclastic Flow** | adds Blight |
| **Pillar of Living Flame** | adds Blight |
| **The Jungle Hungers** | destroys Dahan |
| **Blazing Renewal** | destroys Presence |
| **Insatiable Hunger of the Swarm** | adds Blight |
| **Volcanic Eruption** | destroys Dahan, adds Blight |
| **Draw Towards a Consuming Void** | destroys Presence |
| **Tsunami** | destroys Dahan |
| **Poisoned Land** | destroys Dahan, adds Blight |

## Aspects (all 5, Wiki-verified mechanics)

All 5 Shadows aspects **replace** the Shadows of the Dahan special rule. Four raise complexity; one lowers it.

### Amorphous (Promo Pack 2 — Feather and Flame) — Higher Complexity

**Replaces**: Shadows of the Dahan.

> **Shadows Partake of Amorphous Space**: During each Fast phase, you may move 1 of your Presence to an adjacent land, or to a land with Dahan anywhere on the island. During each Slow phase, you may move 1 of your Presence to an adjacent land, or to a land with Dahan anywhere on the island.

**Strategic shift**: converts the 1-Energy-per-range-extension tax into 2 *free* presence-moves per turn. Plays as a mobility-specialist variant — physical presence motion instead of virtual range-extension.

### Foreboding (Promo Pack 2 — Feather and Flame) — Higher Complexity

**Replaces**: Shadows of the Dahan with a **new Innate**, *Stretch Out Coils of Foreboding Dread* (Fast · Range 2 · Any Land):

- **2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air**: Your other Powers may ignore Range when targeting the target land.
- **1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon**: After an Action generates Fear in target land (incl. Town/City destruction): Push up to 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> per Fear / 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> per 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.
- **2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire**: 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.
- **2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air**: 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">.

**Strategic shift**: swaps a per-Power spatial rule for a land-targeted innate. Range-ignoring is gated to one targeted land (rather than all Dahan lands). Converts fear-on-destruction into pushes for control value.

### Madness (Jagged Earth) — Higher Complexity

**Replaces**: Shadows of the Dahan. Adds 2 new rules:

> **Shadows Cast a Subtle Madness**: When you add Presence during Growth, you may also add 1 Strife in that land.

> **Glimpse of the Shadowed Void**: When your Presence is Destroyed, if Invaders are present, 1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per Presence Destroyed there.

**Strategic shift**: this is where **strife enters Shadows** (previous chapter revisions wrongly put strife on base Shadows). Madness monetizes presence-destruction into fear, incentivizing aggressive presence deployment + Choke-style self-sacrifice plays.

### Reach (Jagged Earth) — **Lower** Complexity

**Replaces**: Shadows of the Dahan with a simpler, single rule:

> **Reach Through Ephemeral Distance**: Once per turn, you may ignore Range. (Anything for which there's a Range arrow or the word "Range" is used. Affects a single Action.)

**Strategic shift**: energy-free range ignore, capped at once per turn. The beginner-friendly aspect — arguably the first aspect to try on Shadows for newer players.

### Dark Fire (Nature Incarnate) — Higher Complexity

**Replaces**: Shadows of the Dahan. Adds 2 new rules:

> **Dark and Fire as One**: You may treat each Moon available to you as being Fire, or vice versa. (Choose during each Action for each Moon/Fire you have.) You may discard or Forget Powers that grant Moon to pay for Fire Choice Events, and vice versa.

> **Frightful Shadows Elude Destruction**: The first time each Action would destroy your Presence, you may Push 1 of those Presence instead of destroying it.

**Strategic shift**: Moon↔Fire interchangeability is exceptional on a spirit whose innate requires *both* elements heavily. Plus presence-push-instead-of-destroy offers a Madness-style defensive safety net. Arguably the most powerful Shadows aspect.

### Aspect selection heuristic

```admonish tip title="Which aspect when?"
- **First few Shadows games** → base (no aspect).
- **Want simpler** → **Reach** (lower complexity; one free range ignore/turn).
- **Want more fear output** → **Madness** (strife + fear-from-presence-destroy).
- **Want mobility specialist** → **Amorphous** (2 free presence-moves/turn).
- **Want element flexibility** → **Dark Fire** (Moon↔Fire swap is huge here).
- **Want experimental** → **Foreboding** (new innate replaces the special rule).
```

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes                                                                        |
|----------------------|----|----|----|----|------------------------------------------------------------------------------|
| England              | A  | A- | B+ | B  | Slow builds; fear-rush works. **L5 cliff**: buildings +1 HP — Innate L3 (3 damage) insufficient for 4-HP Cities without Crops Wither's City→Town downgrade first. |
| Brandenburg-Prussia  | A  | A- | B+ | B  | Cities add fear-per-kill; favorable.                                         |
| Sweden               | A- | B+ | B  | B- | Fear penalties reduce engine output at L2+.                                  |
| France (Plantation)  | B+ | B  | B  | C+ | Dahan capture threatens Shadows of the Dahan targeting pool.                 |
| Habsburg Mining      | A  | B+ | B  | C+ | Scaling outpaces fear-rush late.                                             |
| Russia               | B+ | B  | B- | C  | Fear-suppression mid-late.                                                   |
| Scotland             | A  | B+ | B+ | B  | Favorable.                                                                   |
| Habsburg Livestock   | A  | A- | B  | B  | Favorable.                                                                   |

Grades directional `[VERIFY]` — individual cell confirmations pending playtest.

### Strategy Cliff — England L5

```admonish info title="Strategy Cliff — England L5"
**What changes**: buildings gain +1 HP (Town = 3 HP, City = 4 HP).

**Impact on Shadows**: Innate Level 3 (3 Damage) kills 3-HP Towns but not 4-HP Cities. Crops Wither and Fade's City → Town replacement becomes the pre-softener; Terrifying Nightmares (Major) pushes rather than kills — still valuable but doesn't solve the HP-math alone.

**Mitigation sequence**: (turn N) Crops Wither downgrades City → Town; (turn N+1) Innate L3 or Major finishes.
```

### Strategy Cliff — Sweden L2+ / Russia L3+

```admonish info title="Strategy Cliff — Fear-suppression adversaries"
**What changes**: Sweden L2+ and Russia L3+ have fear-suppression rules.

**Impact on Shadows**: Terror flips slower; fear-card effects reduced.

**Mitigation**: shift toward Crops Wither + Innate L2 Explorer-destruction as board-pressure tools; accept later Terror timeline.
```

## Board / Map Configuration

`[VERIFY via play — pending]` — no community-consensus board ratings for Shadows surfaced from the Wiki spirit-template. Directional hypotheses:

- **Likely favorable**: jungle-dense boards (starting setup puts presence in jungle; innate L3 Moon+Fire+Air threshold matches Shadows's element profile without requiring specific terrain).
- **Likely favorable**: boards with dense starting Dahan clusters (Shadows of the Dahan targeting benefits).
- **Likely neutral**: coastal-heavy boards (Shadows is terrain-flexible but coast-agnostic).
- **Likely unfavorable**: sparse-Dahan layouts.

Ratings per board letter (A–H) deferred to physical-play data.

## Game-Phase Strategy

### Early (T1–3)
- Grow with G2 or G3 to extend card pool and energy.
- Play Concealing Shadows every turn (0 Energy is free value).
- Use Innate L1 (Gather) to reposition Explorers away from Build targets.
- Draft Moon + Fire Minors.

### Mid (T4–6)
- Innate L2 firing (destroy 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> per turn).
- Crops Wither softens City turns.
- Favors Called Due's 3-Fear trigger becomes available.
- Fear pool heading toward Terror 2 flip.

### Late (T7+)
- Terror 2 → Terror 3 transition.
- Gain Terrifying Nightmares Major if offered; forget Mantle of Dread (weakest in solo) or the weakest-matchup Unique.
- Innate L3 available if 4 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air reliably on track.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Bringer of Dreams and Nightmares** — double fear engine; damage → Fear conversion on both sides.
- **Thunderspeaker** — Thunderspeaker grows Dahan density, feeding Favors Called Due + Shadows of the Dahan targeting.
- **Ocean's Hungry Grasp** — Ocean drowns coasts; Shadows handles inland Dahan lands.
- **Any partner** — Mantle of Dread's partner-target push is universally useful.
```

```admonish warning title="Anti-Synergy"
- **Heart of the Wildfire** — destroys Dahan via blight.
- **Vengeance as a Burning Plague** — blight-heavy.
- **Volcano Looming High** — destruction kills Dahan unconditionally.
- **The Jungle Hungers** (your own Major!) — destroys all Dahan in target. Don't draft in a dahan-reliant multiplayer table.
```

## Common Mistakes

```admonish failure title="Common Mistake — Running at 0 Energy"
Shadows of the Dahan costs 1 Energy per range extension. Budget 1E/turn or you lose spatial flexibility.
```

```admonish failure title="Common Mistake — Using Crops Wither as a kill card"
Crops Wither *replaces*, not destroys. A replaced City is a Town in the same land, still Ravages next turn.
```

```admonish failure title="Common Mistake — Missing Favors Called Due conditions"
3 Fear only triggers if Invaders are present AND Dahan (after gather) outnumber them. Count before committing.
```

```admonish failure title="Common Mistake — Drafting Jungle Hungers in multiplayer"
Destroys all Dahan in target. Kills Thunderspeaker's engine. Draft only in solo or with dahan-agnostic partners.
```

```admonish failure title="Common Mistake — Drafting non-Moon Minors"
Every innate level wants Moon. Non-Moon Minors stall the engine.
```

## Tempo Profile

Round-by-round targets, with per-turn Fear contribution `[VERIFY against typical play]`:

| Round | Energy | CP | Presence Placed | Moon | Fire | Per-Turn Fear | Key Play                                |
|-------|--------|----|------------------|------|------|---------------|-----------------------------------------|
| 1     | 1E     | 2  | 4 (of 13)        | 1–2  | 1    | 1–2           | Concealing + Unique/Minor               |
| 2     | 1E     | 2  | 5                | 2    | 1–2  | 2–3           | Concealing + Crops or Favors            |
| 3     | 1E–2E  | 2  | 6                | 2–3  | 2    | 3–5           | Reclaim (G1) or draft (G2); L2 prep     |
| 4     | 2E     | 3  | 7 (track opens)  | 3+   | 2+   | 4–6           | Innate L2 firing; Favors conditional    |
| 5     | 2E–3E  | 3  | 7                | 3+   | 2+   | 4–6           | Major gain (Terrifying Nightmares)      |
| 6     | 3E     | 3  | 7                | 3+   | 3    | 5–7           | Terror 2 flip                           |
| 7     | 3E     | 3  | 6                | 4+   | 3    | 6–8           | Major fires; possible L3 innate prep    |
| 8     | 3E–4E  | 3  | 5                | 4+   | 3+   | 7–10          | Terror 3 close                          |

Cliff turn: **T4**. Innate L2 must fire; Fear pool at 4+/8 (solo).

## Major vs. Minor

**Draft bias**: Mixed (Minor-heavy T1–T3; 1 Major T5+).

## Expansion Sensitivity

- **Base only**: fully functional — 4 Uniques + Innate engine + 1 Special Rule.
- **+ Branch & Claw**: Amorphous + Foreboding aspects unlock; event + blight decks add turn-by-turn variance.
- **+ Jagged Earth**: Madness + Reach aspects unlock; Minor/Major pool deepens.
- **+ Nature Incarnate**: Dark Fire aspect unlocks.

Aspect mechanics now Wiki-verified (see Aspects section above); parser extended 2026-04-19.

## Stat Snapshot

```admonish note title="Stat Insight"
`[VERIFY current numbers from mindwanderer]` — directional figures from community tier lists + older mindwanderer snapshots:
- Solo L6 win rate: approximately 50–55%.
- Best adversary: Brandenburg-Prussia L6 (fear-dense; favorable).
- Worst adversary: Russia L6 (fear-suppression).
- 2-handed: Shadows + Bringer often cited as top fear-rush pair.

Live stats require re-scraping mindwanderer's current page.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/shadows-flicker-like-flame.json` — parsed deterministically via `scripts/wiki-fetch.py` 2026-04-19.
- **Aspect list**: Brett physical-copy verification 2026-04-19.
- Spirit Island Wiki — [Shadows](https://spiritislandwiki.com/index.php?title=Shadows_Flicker_Like_Flame), [Concealing Shadows](https://spiritislandwiki.com/index.php?title=Concealing_Shadows), [Crops Wither and Fade](https://spiritislandwiki.com/index.php?title=Crops_Wither_and_Fade), [Favors Called Due](https://spiritislandwiki.com/index.php?title=Favors_Called_Due), [Mantle of Dread](https://spiritislandwiki.com/index.php?title=Mantle_of_Dread).
- Cross-reference: [Dahan fundamentals](../../fundamentals/dahan.md), [Fear Rush archetype](../../combos/fear-rush.md), [si-wiki-fetch skill](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Physical-copy verified: Uniques, innate thresholds, aspects (2026-04-19). Remaining `[VERIFY]`: Play Difficulty (spirit panel), aspect mechanics (pending parser extension), board ratings (physical play), live mindwanderer stats.*

*Last revised: 2026-04-19 — v0.2.4 (full Wiki-scripted rewrite with suggested-cards, growth, presence tracks, power summary)*
