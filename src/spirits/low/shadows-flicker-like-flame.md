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

Starting: 0 Energy, 1 CP (track-0 values). Opening the track quickly unlocks 3 Energy (significant for the 1E-per-turn Shadows-of-the-Dahan rule).

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
| 1    | ✓       | ✓     | —      | **Ravage-protection is dormant T1** (no Ravage yet). Concealing's "Dahan take no damage from Ravaging Invaders" = null effect this turn; its Fear + elements are the only T1 material value. **First Build**: Setup's Explore card sits in the Build slot, so its terrain Builds T1. |
| 2    | ✓       | ✓     | ✓      | **First Ravage** (Setup's card, now in the Ravage slot). Concealing's protection is **live** IF played on a Ravage land with Dahan. |
| 3    | ✓       | ✓     | ✓      | Ravage, Build, and Explore every turn from here. |

### Growth-option decoder (T1 income branches)

| Growth | Effects | Post-growth T1 state (from 0E/1CP) |
|--------|---------|------------------------------------|
| **G1** | Reclaim + Gain 1 Minor (no presence placed) | 0E / 1CP (unchanged); 1 Minor gained |
| **G2, reveal CP track (card2)** | Gain 1 Minor + Place 1 Presence Range 1 | 0E / **2CP**; 1 Minor gained |
| **G2, reveal Energy track (energy1)** | Gain 1 Minor + Place 1 Presence Range 1 | **1E** / 1CP; 1 Minor gained |
| **G3, reveal CP track (card2)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **3E / 2CP** |
| **G3, reveal Energy track (energy1)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **4E** / 1CP (1 slot + 3 growth effect) |

### Element math — every played card counts

Shadows's innate Darkness Swallows the Unwary is **Fast**: it resolves in the Fast Power Phase, before the Invader Phase. Its thresholds check **every Element you have this turn**. Each card you play in the Spirit Phase grants its Elements from the moment you play it, whether the card is Fast or Slow (Core rulebook p.16; FAQ "Rules semi-commonly misplayed"; confirmed in `data/references/rules/open-questions.md` Q10). So Mantle, Crops, and Favors feed the Fast innate on the turn you play them, even though their own effects wait for the Slow phase. Shadows's Presence tracks have no Element spaces, so cards are the only source.

**Element reach from Uniques alone** (each Unique gives exactly 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon; only Crops and Mantle give <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire):

- **L1 (2 Moon + 1 Fire)**: any two Uniques that include Crops or Mantle. Cheapest: Concealing + Crops, or Concealing + Mantle → **1E, 2 Card Plays**. Concealing + Favors gives 2 Moon but no Fire → no L1.
- **L2 (3 Moon + 2 Fire)**: Crops + Mantle + any third Moon card (Concealing is cheapest) → **2E, 3 Card Plays**. Reachable from Uniques alone once the CP track shows 3.
- **L3 (4 Moon + 3 Fire + 2 Air)**: the Uniques hold only 2 Fire between them, so L3 needs Fire from a drafted card plus at least four Moon cards → 4+ Card Plays.
- **1-Card-Play turns**: no Unique gives 2 Moon, so L1 can't fire on a 1-CP turn of Uniques.

**What speed still decides**: when each card's *effect* lands. A Fast card's effect, and the innate's Gather or Destroy, acts before this turn's Ravage, Build, and Explore. A Slow card's effect acts after the Invader Phase and sets up next turn. Moon+Fire Minors feed the innate at either speed, so weigh a Minor's cost and its effect's timing, not its Elements' timing.

**Pause-point (card plays)**: before committing cards, total Moon and Fire across *all* planned plays, Slow ones included. One Fire short? Swapping Favors for Crops or Mantle may fix it without drafting anything.

### Opening A — G3-via-CP Double-Card Fear Opener 🟨 (default)

**When to pick this**: default against **Brandenburg-Prussia, England, Scotland, Habsburg Livestock** — fear-friendly adversaries. G3-via-CP is the only T1 growth choice that affords a 0E + 1E two-card play from a 0E start, giving the highest T1 Fear + material-effect output — and two Uniques with Moon + Fire, so L1 fires T1.

**Target arc**: 3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + L1 Gather T1 · L1 again T2 (Crops + Favors) and T3 (Concealing + a Fire card) · Terror 2 flip T5–T6.

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

  <dt>Turn elements</dt>
  <dd><span class="si-land">2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air (Concealing + Mantle, both counted from the moment played) · <strong>L1 FIRES T1</strong> → Gather 1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear generated</span> <span class="si-pool"><strong>2E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered by L1</span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I — Build + Explore (no Ravage yet)</span></dd>
</dl>


- **Growth**: **G3, revealing CP track** (uncover `card2`). Post-growth: **3E / 2CP**. 1 Presence placed Range 3; "+3 Energy" is a this-turn-only effect.
- **Card plays (Spirit Phase)**: **Concealing Shadows** (0E, Fast) + **Mantle of Dread** (1E, Slow) — 1E spent, both CP used. **Elements this turn: 2M+1F+2A.** Both cards' Elements count from the moment they're played, Mantle's included, though Mantle's effect waits for the Slow phase.
- **Fast phase plays**: **Concealing Shadows** (target one of your Presence lands — jungle #6 or wetland #5).
  - **T1 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Its 1M+1A supply half the innate's Moon this turn. *"Dahan take 0 Damage from Ravaging"* clause is **dormant this turn** (no Ravage until T2). Concealing discards — so it is also out of hand for T2's first Ravage, until the T3 Reclaim.
- **Innate check (Fast phase)**: total 2M+1F+2A → **L1 fires T1** (needs 2M+1F). L2 (3M+2F) does not.
  - **T1 gain from L1**: Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into the target land, before the Invader Phase. Material this turn if it pulls the only Invader out of a land that Builds this turn (lands without Invaders don't Build). Check the reverse too: gathering into an empty land of the Build terrain gives it a Build.
- **Invader phase**: Build in the lands matching Setup's Explore card, then Explore (Stage I): one Explorer in each land of the shown terrain that has, or is adjacent to, a Town/City or the Ocean. No Ravage yet.
- **Slow phase plays**: **Mantle of Dread** (target self).
  - **T1 gain from Mantle**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> and 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> from one of your Presence lands — moves 2 Invader units out of that land. Its 1M+1F+1A already counted toward L1 this turn. Mantle discards.
- **End of T1**: **3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> generated** · **1 Explorer gathered (L1)** · 1E spent (Mantle) · **2E banked** · 0 CP left · 4 Presence on board (3 starting + 1 G3) · Hand: 2 Uniques (Concealing + Mantle in discard) · Tracks: CP at card2 (2 CP sustained from now), Energy still at energy0.
- **Pause-point (card choice)**: Concealing + Mantle is the cheapest L1 pair (1E). Crops + Mantle (2E) also fires L1 (2M+2F), for 4 Fear instead of 3, and keeps Concealing in hand for T2's first Ravage, at the cost of 1E banked instead of 2E and a different T2 hand. Crops' downgrade needs a Town or City in the target land to do more than its 2 Fear. Which is worth more here: the extra Energy, or Dahan protection on T2's Ravage land?
- **Pause-point (pre-Mantle-target)**: self-target is the default, but Mantle's target is "Any Spirit" — in multiplayer, check whether pushing from a partner's Presence land solves a bigger threat than pushing from yours. Solo: T2's Ravage hits Setup's Explore terrain — the lands that Build this turn — so choose the Presence land whose Explorer+Town push most shrinks that Ravage (or a later one).

#### Turn 2

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing Energy track (<code>energy1</code>) — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">2E banked + 1E income = <strong>3E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Crops Wither</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span> <span class="si-land"><strong>Favors Called Due</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span></dd>

  <dt>Turn elements</dt>
  <dd><span class="si-land">2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal (Crops + Favors) · <strong>L1 FIRES T2</strong> → Gather 1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> before the first Ravage</span></dd>

  <dt>End of T2</dt>
  <dd><span class="si-pool">2–5 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered by L1</span> <span class="si-pool">1 City <img class="si" src="/spirit-island/theme/icons/unit-city.svg" alt="City"> → Town (if targeted)</span> <span class="si-pool">Dahan gathered</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I — Ravage + Build + Explore (first <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <strong>Ravage</strong>)</span></dd>
</dl>


- **Growth**: **G2, revealing Energy track** (uncover `energy1` → 1E permanent income from now). Post-growth: **2E banked + 1E income = 3E / 2CP**, +1 Minor drafted.
- **Draft consideration for the T2 Minor**: L1 already fires T2 from Crops + Favors (below), and 2 CP fills this turn, so this Minor is for T3+. Moon+Fire Minors feed the innate at either speed. 0-cost ones (e.g., Land of Haunts and Embers, Fast, adds 1 Blight; Shadows of the Burning Forest, Slow) fit low-Energy turns. A Fast one's effect also lands before the Invaders act.
- **Card plays (Spirit Phase)**: **Crops Wither and Fade** (1E) + **Favors Called Due** (1E) — 2E spent, both CP used. **Elements this turn: 2M+1F+1A+1P+1An**, counted from the moment played.
- **Fast phase**: no Fast cards. **Innate check**: 2M+1F → **L1 fires T2**; L2 (3M+2F) does not.
  - **T2 gain from L1**: Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into the target land before the Invader Phase. T2 is the first Ravage: pulling an Explorer out of the Ravage land (or out of a Build land where it's the only Invader) means one fewer Invader there when the Invaders act. It's the only part of this turn that acts before the Ravage.
- **Slow phase plays**: **Crops Wither and Fade** (1E, Range 0 — target one of your Presence lands with a Town or City) + **Favors Called Due** (1E, Range 1 — target a Dahan-dense Invader-present land).
  - **T2 gain from Crops**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Immediate replace: City→Town OR Town→Explorer in target land. *Material this turn*: the replaced unit is gone; *downstream*: when T3 Ravage hits that land, Damage is 1 lower than it would have been.
  - **T2 gain from Favors**: Gather up to 4 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> into target. *Material this turn*: Dahan positioning for T3+ counterattacks. If post-gather Dahan > Invaders in target, **+3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> conditional**.
- **End of T2**: **2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> generated** (2 Crops + 0–3 Favors conditional) · **1 Explorer gathered (L1)** · 2E spent · **1E banked** · 0 CP left · 5 Presence on board · Hand: 0 Uniques + 1 Minor (all four Uniques in discard: Concealing + Mantle since T1, Crops + Favors now) · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-Favors-target)**: Favors's 3-Fear conditional is **not free** — it needs post-gather Dahan to strictly outnumber Invaders. Before committing, count: Invaders in target + Dahan in R1 of target = enough to gather past Invader count? If no, change target or save Favors for T3. If you swap a card for the drafted Minor, check L1 still holds: Crops is the only Fire Unique in hand (Mantle is in discard), and every pair needs 2 Moon. Crops + a Moonless Minor, or Favors + a Fireless Minor, leaves L1 unfired.

#### Turn 3

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G1</strong> — Reclaim (all played back to hand) + Gain 1 Minor</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">1E banked + 1E income = <strong>2E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Card plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> (target Ravage land w/ Dahan) <img class="si" src="/spirit-island/theme/icons/speed-fast.svg" alt="Fast"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"></span> <span class="si-land">+ one Moon+Fire card (Crops, Mantle, or a drafted Minor, either speed) → <strong>L1 FIRES T3</strong></span></dd>

  <dt>Key T3 note</dt>
  <dd><span class="si-pool"><strong>Second Ravage turn</strong> (first was T2) — Concealing is back in hand, its protection <em>material</em></span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I Ravage + Build · Stage II Explore</span></dd>
</dl>


- **Growth**: **G1** — Reclaim (all 4 discarded Uniques back to hand) + Gain 1 Minor. No presence placed. State: **1E banked + 1E income = 2E / 2CP**, hand = 4 Uniques + 2 Minors.
- **Invader state**: **T3 is the second Ravage turn** (the first was T2). Identify the Ravage-target land from the current Invader card's Ravage column (the card that was in Build slot last turn has advanced).
- **Card plays (Spirit Phase)**: **Concealing Shadows** (0E) + one card with Moon + Fire costing ≤1E: **Crops** (1E), **Mantle** (1E), or a drafted Moon+Fire Minor of either speed (e.g., Land of Haunts and Embers, 0E). Any of these pairs totals 2M+1F or more. 2 CP caps the turn at two cards, so L2 (3M+2F) is out of reach.
- **Fast phase plays**: **Concealing Shadows** (target the Ravage-incoming land, with Dahan in it, if any), plus the second card if it's Fast.
  - **T3 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + **Ravage-protection now material** — Dahan in target land take 0 Damage from this turn's Ravage. Elements 1M+1A.
  - **T3 gain from Haunts (if it's the second card)**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns from its Range-2 target, before the Ravage. If Blight is already present there: +2 Fear and Push up to 2 more. Then **Add 1 Blight** <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> to the target land (its own line on the card, so it always happens). Elements 1M+1F+1A.
- **Innate check (Fast phase)**: 2M+1F or more → **L1 fires T3** (its third straight turn in this opener). Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into the target land. Fast, so the gather happens **before** the Ravage — can pull an Explorer out of the Ravage target (one fewer Invader taking the Ravage).
- **Slow phase plays**: the second card, if you chose **Crops** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + City→Town or Town→Explorer downgrade) or **Mantle** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + push). Either resolves after this turn's Ravage. No CP remains for a third card.
- **End of T3**: **3 Fear generated** (1 Concealing + 2 from Crops, Mantle, or Haunts; L1 makes no Fear) · **1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered (L1 innate)** · **Ravage Damage in one land reduced** (Concealing protects Dahan; the L1 Gather, or a Haunts push, thins the Ravage land first) · +1 Blight if Haunts was played · 1–2E banked (2E if the second card cost 0) · 0 CP left · 5 Presence on board · Hand: 4 cards (2 Uniques + 2 Minors, or 3 Uniques + 1 Minor if a Minor was the second card) · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-Fast-plays)**: Concealing's Ravage-protection is material from T2 on **only if the target land actually Ravages this turn AND has Dahan to protect**. If the Ravage target has 0 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan">, Concealing's protection clause is still dormant — consider targeting a Dahan-adjacent Ravage land (use Shadows of the Dahan's 1E range extension) or playing Concealing elsewhere for its Fear + elements alone.

#### Turn 4 — state audit

After T3, you should have:

- **Presence**: 5 on board (3 starting + 1 T1 G3 + 1 T2 G2).
- **Energy / CP**: 1E income (from energy1 slot) / 2CP.
- **Engine**: All 4 Uniques available + 2 Minors drafted. L1 fires any turn your plays total 2M+1F — from Uniques alone, Concealing + Crops or Mantle does it. L2 (3M+2F) needs a third Card Play (Crops + Mantle + one more Moon card), so it waits on the CP track's `card3` slot.
- **Fear pool**: ~8–12 (solo — Terror 2 should flip around end-T4 or T5, not T3 as the prior opener claimed).
- **Blight**: 0–1 (depends on the T2 and T3 Ravage outcomes; Concealing + L1 Gather mitigation should keep this low), +1 if you played Land of Haunts and Embers.

**Pivot triggers** (if any of these, deviate from the opener):

- **Fire runs out** — Crops and Mantle are the only Fire Uniques. On a turn both sit in discard and no Fire Minor is in hand, L1 can't fire. Time the G1 Reclaim around the turns you need the Gather.
- **Ravage target T3 has no Dahan** — Concealing's protection is still dormant; use Concealing purely for Fear + elements, and save the real defense for T4+ when Dahan density increases.
- **Favors never triggers 3-Fear conditional** — Fear pool is running 3–4 under target. Prioritize Minor picks that generate Fear directly (e.g., 2-Fear Moon Minors from the full pool).

### Opening B — G2-via-CP Minor-Draft-Dependent 🟥

**When to pick this**: **Habsburg Mining L5+, Russia L5+**, or long-game matchups where Minor-draft density pays off. Trades T1 Fear output for hand depth. **Gated on drafting a 0-cost Moon+Fire Minor on T1, Fast or Slow** (e.g., Land of Haunts and Embers or Shadows of the Burning Forest) — otherwise this opener under-delivers.

**Target arc**: If a 0-cost Moon+Fire Minor is drafted T1 → **L1 fires T1** (as in Opening A, here via Concealing + the Minor) · 2 Minors drafted by T2 · Reclaim T3 → 4 Uniques + 3 Minors in hand · matchup-specific Minor picks scale T4+.

#### Turn 1

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing CP track — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>0E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Best-case plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> + a drafted 0-cost Moon+Fire Minor (e.g., <strong>Land of Haunts and Embers</strong>)</span></dd>

  <dt>Turn elements</dt>
  <dd><span class="si-land">2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> (with Haunts) · <strong>L1 FIRES T1</strong> → Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered by L1</span> <span class="si-pool">up to 2 pushed by Haunts</span> <span class="si-pool">+1 <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> Blight (Haunts)</span></dd>
</dl>


- **Growth**: **G2, revealing CP track** (uncover `card2`). Post-growth: **0E / 2CP**, 1 Minor drafted.
- **Draft priority T1 Minor**: any **0-cost Minor with Moon + Fire** fires L1 alongside Concealing. Slow ones count too, since a card's Elements count from the moment it's played. Candidates from the full-pool table below (engine card text):
  - **Land of Haunts and Embers** (Fast; Moon, Fire, Air) — 2 Fear, Push up to 2 Explorers/Towns; more if Blight is present; **Adds 1 Blight**.
  - **Shadows of the Burning Forest** (Slow; Moon, Fire, Plant) — 2 Fear; if the target is a Mountain or Jungle, Push 1 Explorer and 1 Town.
  - **Animated Wrackroot** (Slow; Moon, Fire, Plant) — 1 Fear + Destroy 1 Explorer, OR Add 1 Wilds.
  - **Lure of the Unknown** (Fast; Moon, Fire, Air, Plant) — Gather 1 Explorer/Town.
  - **Roiling Bog and Snagging Thorn** (Fast; Moon, Fire, Water, Plant) — 1 Fear, Isolate, Defend 2 (Defend dormant T1).
  - **Pause**: speed now decides only *when the Minor's effect lands*. A Fast push or gather acts before T1's Build; a Slow one acts after the Invader Phase. Weigh that, and Haunts' Blight, against each other. If no 0-cost Moon+Fire Minor appears: Opening B is underpowered T1 — consider pivoting to C (see Opening Decision tree).
- **Card plays (Spirit Phase, with Haunts drafted)**: **Concealing Shadows** (0E) + **Land of Haunts and Embers** (0E) — 0E spent, both CP used. **Elements this turn: 2M+1F+2A.**
- **Fast phase plays**: both cards. Haunts is Range 2; target an Invader land (its push works on any terrain).
  - **T1 gain from Concealing**: +1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Dormant Ravage-protection (no Ravage T1). Elements 1M+1A.
  - **T1 gain from Haunts**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">. Push up to 2 Explorers <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer">/Towns from target land, before T1's Build — material if it empties a land that Builds this turn. If Blight is already present: +2 more Fear and Push up to 2 more. Then **Add 1 Blight** <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> to the target land (its own line on the card, so it always happens). Elements 1M+1F+1A.
- **Innate check (Fast phase)**: total 2M+1F+2A → **L1 fires T1** (L2 does not). Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into the target land, before the Invader Phase. With a Slow 0-cost Moon+Fire Minor instead of Haunts, L1 fires just the same.
- **Slow phase plays**: none with Haunts (0 CP remain). If the drafted Minor was Slow (e.g., Shadows of the Burning Forest), its effect resolves here.
- **Invader phase**: Build (Setup's Explore terrain) + Explore. No Ravage yet.
- **End of T1**: **3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered (L1)** + up-to-2 units pushed by Haunts + **1 Blight added (Haunts)** · 0E banked · 0 CP left · 4 Presence on board · Hand: 3 Uniques + 0 Minors (Haunts played) · Tracks: CP at card2, Energy still at energy0.
- **Pause-point (pre-Haunts-target)**: Haunts' push works on any terrain (the Mountain/Jungle condition belongs to Shadows of the Burning Forest), but it Adds 1 Blight to the target land every time. Is +2 Fear and up to 2 pushed before the Build worth a Blight now? Would a Slow 0-cost Moon+Fire Minor, if offered, give the same T1 L1 without it?

**Fallback T1 (no 0-cost Moon+Fire Minor drafted)**: at 0E only 0-cost cards are playable — Concealing, plus the drafted Minor if it costs 0. **Concealing alone**: Fear = 1; elements 1M+1A → L1 does NOT fire (needs 2M+1F). A 0-cost Minor with Moon but no Fire also leaves L1 short. This is Opening B underperforming — expect T2–T3 Minor draft to compensate.

#### Turn 2

- **Growth**: **G2, revealing Energy track** (uncover `energy1` → 1E income). Post-growth: **1E / 2CP**, +1 Minor drafted (2 total).
- **Draft priority T2 Minor**: another Moon+Fire Minor (either speed) widens which pairs fire L1. At 1E this turn, only a 0-cost one can be paired with a 1E Unique now.
- **Card plays (Spirit Phase)**: 1E / 2 CP. Hand = Crops, Favors, Mantle + the T2 Minor (Concealing and the T1 Minor are in discard). Every Unique in hand costs 1E, so the most you can play is one Unique + one 0-cost Minor.
- **Innate check**: one Unique gives 1 Moon; L1 needs 2 Moon + 1 Fire. So **L1 fires T2 only if the T2 Minor costs 0 and carries Moon** (and Fire too, if the Unique is Favors). Otherwise L1 does not fire T2.
- **Slow phase plays**: 1 Slow Unique at 1E — **Crops Wither** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, downgrade) OR **Mantle** (+2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, self-push) OR **Favors** (+0–3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, Dahan gather).
- **T2 material gain**: typically 2–3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + one material board effect (downgrade / push / gather), + the L1 Gather if a 0-cost Moon Minor was drafted (see innate check).
- **End of T2**: 2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> · ~0E banked · 0–1 CP left · 5 Presence · hand typically 2 Uniques + 0–1 Minor.
- **Pause-point (pre-T2-plays)**: T2 is the first Ravage (and the second Build). Check the Invader card's Build column — which terrain Builds? Every land of that terrain with any Invaders gets a Town (or a City, if it has more Towns than Cities). Slow cards resolve after the Invader Phase, so a T2 Slow Crops (or Mantle push) can't stop this turn's Build; it shrinks that land's Ravage next turn instead.

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. State: **~0E + 1E income = 1E / 2CP**, hand = 4 Uniques + 3 Minors.
- **Card plays (Spirit Phase)**: 1E / 2 CP — two cards. **Concealing** (0E) + one card costing ≤1E with Moon + Fire: Crops, Mantle, Haunts (if reclaimed), Visions of Fiery Doom (1E Fast), or another Moon+Fire Minor. Elements = 2M+1F+... → **L1 fires T3**. 2 CP caps the turn there; L2 (3M+2F) needs a third card.
- **Fast phase plays**: Concealing, plus the second card if it's Fast. L1 Gather resolves here, before the Ravage.
- **Slow phase plays**: the second card, if it's Slow (Crops or Mantle). No CP remains for more.
- **T3 material gain**: 2–3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> (Concealing 1 + Crops, Mantle, or Haunts 2, or Visions 1) + L1 Gather + the second card's board effect. First Ravage turn in this opener with Concealing back in hand (the first Ravage was T2), so its protection becomes material.
- **Pause-point (pre-Concealing-target)**: same as Opening A T3 — Ravage-protection is material only on the Ravage target with Dahan. If Ravage lands don't have Dahan, use Concealing purely for Fear + elements.

#### Turn 4 — state audit

- **Presence**: 5 on board.
- **Energy/CP**: 1E income / 2CP.
- **Engine**: L1 fires whenever two plays total 2M+1F — from Uniques alone, Concealing + Crops or Mantle does it. L2 (3M+2F) needs a third Card Play (Crops + Mantle + one more Moon card).
- **Fear pool**: 6–10 (slower than Opening A if the T1 Moon+Fire Minor missed; comparable if it hit).
- **Hand depth**: 7 cards (4 Uniques + 3 Minors) — deepest of the three openers.
- **Strategy note**: Opening B's payoff is **late** — T5+ benefits from Minor-draft breadth for matchup-specific picks (e.g., Dahan-summoning, Blight-removing, Strife-adding). Fear-rush numbers lag Opening A by ~3–4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> through T4.

### Opening C — G3-via-Energy Energy-Bank 🟥

**When to pick this**: when you want Energy depth over early Card Plays. Opening C leans on energy for (a) aggressive Shadows-of-the-Dahan range extension (1E/use budget), (b) 1E Slow Uniques played every turn, (c) room for a costlier Minor or Major later. The trade: 1 CP on T1, so **L1 can't fire T1** (no Unique gives 2 Moon), and 2 T1 Fear against Opening A's 3.

An earlier version of this chapter sent Sweden L3+ and Russia here because of "fear-card penalties". Neither adversary has one (`data/references/rules/register.md` §4.3, §4.6). What they do change that touches Shadows is listed under [Strategy Cliff — Sweden / Russia](#strategy-cliff--sweden--russia-no-fear-penalty). Whether any of it favors this opener over Opening A is untested `[VERIFY via play]`.

**Target arc**: Mantle T1 (2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">; L1 can't fire on 1 CP) · Crops + Favors T2 → L1 fires T2 · Terror 2 flip later (T6–T7).

#### Turn 1

<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G3</strong> revealing Energy track — 1E income perm + 3E growth effect</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>4E</strong></span> <span class="si-pool"><strong>1CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Mantle of Dread</strong> (self) <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E · 2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear + push</span></dd>

  <dt>Turn elements</dt>
  <dd><span class="si-land">1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air (Mantle) · L1 <strong class="subtle">does NOT fire</strong> — 1 Moon short</span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool"><strong>3E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>
</dl>


- **Growth**: **G3, revealing Energy track** (uncover `energy1` → 1E permanent income). Post-growth: **1E income + 3E growth-effect = 4E / 1CP**.
- **Card plays (Spirit Phase)**: **Mantle of Dread** (1E) — 1E spent, the only CP used. **Elements this turn: 1M+1F+1A**, counted from the moment it's played.
- **Fast phase plays**: none (CP budget of 1 is best spent on a higher-material-effect Slow card).
- **Innate check (Fast phase)**: 1M+1F+1A → L1 needs 2 Moon → **does NOT fire**. With 1 CP and no Unique carrying 2 Moon, L1 can't fire T1 in this opener whichever card you play.
- **Invader phase**: Build (Setup's Explore terrain) + Explore. No Ravage yet.
- **Slow phase plays**: **Mantle of Dread** (self-target).
  - **T1 gain from Mantle**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + Push 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 Town <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> from one of your Presence lands. Its 1M+1F+1A counted all turn, one Moon short of L1.
- **End of T1**: **2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">** · 1E spent · **3E banked** · 0 CP left · 4 Presence on board · Hand: 3 Uniques (Mantle in discard) · Tracks: CP at card1 (still 1 CP), Energy at energy1 (1E sustained).
- **Pause-point (growth choice)**: G3-via-Energy gives deeper Energy income long-term (energy1 permanent = +1E/turn from T2 onward) vs. G3-via-CP (card2 permanent = +1 CP/turn). Over 6 rounds that's 6E vs. 6 extra card plays. Energy depth buys Shadows-of-the-Dahan range extensions and Minor cost-curve flexibility. CP depth buys more card plays per turn, and with them more Fear and more innate Elements (L1 needs 2 plays of Uniques, L2 needs 3). If unsure, G3-via-CP (Opening A) is the tighter fear-rush; G3-via-Energy (Opening C) is the slower, more flexible play.

#### Turn 2

- **Growth**: **G2, revealing CP track** (uncover `card2` → 2 CP from now). Post-growth: **3E banked + 1E income = 4E / 2CP**, +1 Minor drafted.
- **Card plays (Spirit Phase)**: **Crops Wither** (1E) + **Favors Called Due** (1E) — 2E spent, both CP used. **Elements this turn: 2M+1F+1A+1P+1An**, counted from the moment played.
- **Fast phase plays**: none in this line. Concealing is in hand and T2 is the first Ravage, so its protection would be live this turn — but the 2 CP go to Crops + Favors. Pause: is protecting Dahan in T2's Ravage land worth more than one of those Slow plays? Concealing + Crops also fires L1 (2M+1F); Concealing + Favors does not (2 Moon, no Fire — Mantle is in discard).
- **Innate check (Fast phase)**: 2M+1F → **L1 fires T2** (L2 does not). Gather 1 Explorer <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> into the target land before the first Ravage — can pull an Explorer out of the Ravage land.
- **Slow phase plays**: **Crops Wither** (1E, Presence-land target with a Town or City) + **Favors Called Due** (1E, R1 Dahan-dense target).
  - **T2 gain from Crops**: +2 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + City→Town OR Town→Explorer. (Material THIS turn; reduces next-turn Ravage damage in that land.)
  - **T2 gain from Favors**: Gather up to 4 Dahan <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> <img class="si" src="/spirit-island/theme/icons/unit-dahan.svg" alt="Dahan"> into R1 target; conditional +3 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> if post-gather Dahan > Invaders.
- **End of T2**: **2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">** · **1 Explorer gathered (L1)** · 2E spent · **2E banked** · 0 CP left · 5 Presence on board · Hand: 1 Unique (Concealing) + 1 Minor · Tracks: CP at card2, Energy at energy1.
- **Pause-point (pre-T2-plays)**: T2 is the first Ravage and the second Build. Same check as Opening A/B — Crops + Favors are Slow, so they resolve after this turn's Ravage and Build and together handle one land that Ravages next turn (downgrade + gather counter-Dahan). The L1 Gather is the only piece that acts before this turn's Ravage. If two lands are headed for a big Ravage, one of them is **not handled** — which one costs you more?

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. State: **2E banked + 1E income = 3E / 2CP**, hand = 4 Uniques + 2 Minors.
- **Card plays (Spirit Phase)**: 3E / 2 CP — two cards. Any pair totaling 2M+1F fires L1: **Concealing + Crops or Mantle** (1E; Concealing's Ravage-protection live on a Ravage land with Dahan), **Crops + Mantle** (2E; 4 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> + downgrade + push), **Crops + Favors** (2E; if Dahan-outnumber is still triggerable somewhere), or Concealing/Favors + a drafted Moon+Fire Minor. Concealing + Favors has no Fire → no L1. L2 (3M+2F) needs a third card, so not on 2 CP.
- **Fast phase plays**: Concealing, if in the pair (1 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear">, elements 1M+1A), and the **L1 Gather** whenever the pair totals 2M+1F — before the Ravage.
- **Slow phase plays**: the Slow card(s) from the pair.
- **T3 material gain**: 2–5 Fear <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> depending on the pair (Concealing 1; Crops, Mantle 2 each; Favors 0–3) + L1 Gather + the pair's board effects + Ravage-protection on Concealing's target if played.
- **End of T3**: 1–2E banked before any Shadows-of-the-Dahan range extensions (1E each).
- **Pause-point (T3 Ravage targeting)**: same Ravage-protection check as Openings A/B. Additionally, Opening C's energy-bank advantage lets you spend 1E on Shadows-of-the-Dahan range extension to place a Slow-effect card (Crops / Favors / Mantle) on a non-Presence Dahan land — consider if the highest-leverage target is outside R0.

#### Turn 4 — state audit

- **Presence**: 5 on board.
- **Energy/CP**: ~1E income / 2CP.
- **Engine**: L1 fires from any two-card play with Crops or Mantle plus another Moon card (same as Openings A/B). L2 (3M+2F) needs three plays (Crops + Mantle + one more Moon card), so it waits on a third Card Play.
- **Fear pool**: ~6–12 (T1 2 + T2 2–5 + T3 2–5).
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
  Adv -->|Sweden L3+| C[Opening C - Energy Bank]
  Adv -->|Russia| C
  Adv -->|Habsburg Mining L5+| B[Opening B - Minor-Draft Dependent]
  Adv -->|Multi-handed with board-carry partner| A
  Adv -->|Other| A
</pre>

The Sweden L3+ and Russia → C routes are untested 🟥 suggestions. Their original reason ("fear-card penalties") was wrong: neither adversary has one (see [Strategy Cliff — Sweden / Russia](#strategy-cliff--sweden--russia-no-fear-penalty)).

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
| Sweden               | A- | B+ | B  | B- | No Fear penalty. Escalation turns Dahan into Towns after Explore; L3 Towns deal 3 Damage, Cities 5. |
| France (Plantation)  | B+ | B  | B  | C+ | Dahan capture threatens Shadows of the Dahan targeting pool.                 |
| Habsburg Mining      | A  | B+ | B  | C+ | Scaling outpaces fear-rush late.                                             |
| Russia               | B+ | B  | B- | C  | No Fear penalty. L2 Pushes the first Explorer each Action would Destroy (+1 Fear); L5 Fear cards #3/#7 send Invader cards to Build. |
| Scotland             | A  | B+ | B+ | B  | Favorable.                                                                   |
| Habsburg Livestock   | A  | A- | B  | B  | Favorable.                                                                   |

Grades directional `[VERIFY]` — individual cell confirmations pending playtest.

### Strategy Cliff — England L5

```admonish info title="Strategy Cliff — England L5"
**What changes**: buildings gain +1 HP (Town = 3 HP, City = 4 HP).

**Impact on Shadows**: Innate Level 3 (3 Damage) kills 3-HP Towns but not 4-HP Cities. Crops Wither and Fade's City → Town replacement becomes the pre-softener; Terrifying Nightmares (Major) pushes rather than kills — still valuable but doesn't solve the HP-math alone.

**Mitigation sequence**: (turn N) Crops Wither downgrades City → Town; (turn N+1) Innate L3 or Major finishes.
```

### Strategy Cliff — Sweden / Russia (no Fear penalty)

```admonish info title="Strategy Cliff — Sweden / Russia"
**What changes**: neither adversary penalizes Fear (`data/references/rules/register.md` §4.3, §4.6). The rules that touch Shadows:

- **Sweden Escalation** (Stage II): after Invaders Explore into a land, if it has at least as many Invaders as Dahan, replace 1 Dahan with 1 Town — fewer Dahan for Favors Called Due and Shadows of the Dahan.
- **Sweden L3**: Towns deal 3 Damage, Cities 5 — Ravages kill more Dahan, so Concealing's protection and Crops' downgrade save more.
- **Russia L2**: the first time each Action would Destroy Explorers, 1 of them is Pushed instead (if possible), +1 Fear. Innate L2 destroys 1 and pushes 1 of its 2 Explorers; Russia's Fear makes up the lost Fear, but the pushed Explorer stays on the island.
- **Russia L3**: Ravage cards also match lands with 3+ Explorers — an L1 Gather that brings a land to 3 Explorers adds a Ravage there.
- **Russia L5**: an Invader card sits under Fear cards #3 and #7 and goes straight to the Build space when earned — Shadows' Fear pace decides when those extra Builds arrive.

**Pause-points**: before an L1 Gather vs. Russia L3+, count Explorers in the target land. Before a Fear-heavy turn vs. Russia L5+, count Fear cards earned: is the 3rd or 7th about to flip an Invader card into Build?
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

Round-by-round targets, with per-turn Fear contribution confirmed by one authored playthrough (solo, Brandenburg-Prussia Level 6, Board A balanced, 8-round win). Early rounds may hit the low end; the R4 major-draft (Moon-aligned) is the inflection point where per-turn fear jumps from ~2 to 5+.

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
- Worst adversary: Russia L6.
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
