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
| 1     | 2 Moon + 1 Fire          | Gather 1 Explorer.                                          |
| 2     | 3 Moon + 2 Fire          | Destroy up to 2 Explorer. 1 Fear per Explorer destroyed.    |
| 3     | 4 Moon + 3 Fire + 2 Air  | 3 Damage. 1 Fear per Invader destroyed by this Damage.      |

Key decision: Level 1 *gathers* (repositions) an Explorer without killing — useful to move Explorers out of upcoming Build lands. Level 2+ destroys for fear. Don't over-rush Level 2 if Level 1 solves the land.

## Unique Cards (all 4, Wiki-verified)

#### Concealing Shadows

- **0 Energy · Fast · Range 0 · Any Land · Moon, Air**
- *1 Fear. Dahan take no Damage from Ravaging Invaders this turn.*

Free-to-play defensive + 1 Fear every turn. Pairs with Favors Called Due (preserves Dahan to outnumber Invaders).

#### Crops Wither and Fade

- **1 Energy · Slow · Range 0 · Any Land · Moon, Fire, Plant**
- *2 Fear. Replace 1 Town with 1 Explorer. **OR** Replace 1 City with 1 Town.*

**Downgrade**, not destroy. Softens Ravages + 2 Fear per play. A City → Town reduces that land's Ravage damage by 1 and removes a Build upgrade path.

#### Favors Called Due

- **1 Energy · Slow · Range 1 · Any Land · Moon, Air, Animal**
- *Gather up to 4 Dahan. If Invaders are present and Dahan now outnumber them, 3 Fear.*

Massive conditional fear spike (3 Fear). Needs gatherable Dahan + Invader-present target + post-gather Dahan > Invader count.

#### Mantle of Dread

- **1 Energy · Slow · No Range · Any Spirit · Moon, Fire, Air**
- *2 Fear. Target Spirit may Push 1 Explorer and 1 Town from a land where it has Presence.*

**Partner-support** (target Any Spirit). In multiplayer, hands a partner a free push on one of their lands. In solo, self-target.

## Key Strategic Principles

1. **Fear output is huge.** Concealing (1) + Crops (2) + Favors (3 conditional) + Mantle (2) + Innate L2 (up to 2) = **up to 10 Fear in one turn** at the peak. Shadows has the base game's strongest per-turn fear ceiling by raw card output.
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

Three Rei-format opening variants. Each is a turn-by-turn rehearsal for the first 3 turns + a T4 state audit. Pick the variant that matches your adversary + scenario before game start — don't improvise T1.

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

**Starting state** (all openings share this — verified per `si-rules-check`):

- 3 Presence on board: 2 in highest-numbered Jungle, 1 in land #5.
- 4 Uniques in hand: Concealing Shadows (0E Fast, Moon+Air), Crops Wither and Fade (1E Slow, Moon+Fire+Plant), Favors Called Due (1E Slow, Moon+Air+Animal), Mantle of Dread (1E Slow, Moon+Fire+Air).
- **Starting income: 0 Energy, 1 Card Play** (first-uncovered Energy track slot `energy0`, first-uncovered CP track slot `card1`).
- Growth type: **pick one** of G1/G2/G3 per turn. Presence placement reveals **one** track slot; it doesn't reveal both.

**Growth reference**:

| Growth | Effects | Post-growth T1 state (from 0E/1CP) |
|--------|---------|------------------------------------|
| **G1** | Reclaim + Gain 1 Minor (no presence placed) | 0E / 1CP (unchanged); 1 Minor gained |
| **G2, reveal CP track (card2)** | Gain 1 Minor + Place 1 Presence Range 1 | 0E / **2CP**; 1 Minor gained |
| **G2, reveal Energy track (energy1)** | Gain 1 Minor + Place 1 Presence Range 1 | **1E** / 1CP; 1 Minor gained |
| **G3, reveal CP track (card2)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **3E / 2CP** |
| **G3, reveal Energy track (energy1)** | Place 1 Presence Range 3 + "+3 Energy" growth effect | **4E** / 1CP (1 from slot + 3 from growth effect) |

**T1 legal play enumeration**: given T1 starting 0E/1CP, only G3-via-CP (3E/2CP) supports playing both a 0E card and a 1E card. G2 branches give either 2-card-but-0-energy or 1-energy-but-1-card. G1 is 0E/1CP.

### Opening A — Fear Opener via G3 🟨 (default)

**When to pick this**: default against **Brandenburg-Prussia, England, Scotland, Habsburg Livestock** — fear-friendly adversaries. The G3-via-CP-track T1 is the *only* growth choice that affords a 0E+1E two-card T1 from 0E start, making this the tightest path to Innate L1 firing T1.

**Target arc**: Innate L1 fires T1 · Innate L2 reachable T3 after Reclaim · Terror 2 flip T5–T6.

#### Turn 1

- **Growth**: **G3, revealing CP track** (uncover `card2`). Post-growth state: **3E / 2CP** (1 Presence placed Range 3; "+3 Energy" growth effect).
- **Cards played**: **Concealing Shadows** (0E Fast, Moon+Air) + **Mantle of Dread** (1E Slow, target self, Moon+Fire+Air). Total **1E spent**, both Fast+Slow phases used.
  - Concealing Shadows: 1 Fear + Dahan in target land take 0 Damage from Ravaging this turn.
  - Mantle of Dread (self-target): 2 Fear + you may Push 1 Explorer and 1 Town from one of your Presence lands.
- **Elements played this turn**: 2 Moon + 1 Fire + 2 Air.
- **Innate trigger**: **Darkness Swallows the Unwary L1** (2M+1F) fires → **Gather 1 Explorer** into the innate's target land (Fast, Range 1 from a Sacred Site if you want the optional range-boost; else Range 1 from your presence).
- **E/CP state**: 0E / 1CP entering → G3 makes it 3E / 2CP → spend 1E (Mantle) → end T1 with **2E banked / 0 CP remaining**.
- **Presence placement**: 1 Presence placed Range 3 (from CP track — reveals card2 so from T2 on you have 2 CP sustained).
- **Fear contribution**: 3 Fear (1 Concealing + 2 Mantle).
- **Milestone**: 1 Explorer gathered (positional); 1 Explorer pushed via Mantle; 1 Town pushed via Mantle; Dahan in Concealing's land protected; 2E banked carries to T2.

#### Turn 2

- **Growth**: **G2, revealing Energy track** (uncover `energy1`). Post-growth state: **2E banked + 1E income = 3E / 2CP** (CP track already at card2 from T1; Energy track now at energy1; +1 Minor drafted).
- **Cards played**: **Crops Wither and Fade** (1E Slow, Range 0, Moon+Fire+Plant) on a City-present land (replace City→Town) + **Favors Called Due** (1E Slow, Range 1, Moon+Air+Animal) on a Dahan-dense Invader-present land. Total **2E spent**.
  - Crops Wither: 2 Fear + City→Town downgrade (or Town→Explorer).
  - Favors Called Due: Gather up to 4 Dahan; if Invaders present and Dahan outnumber them, **3 Fear**.
- **Elements played this turn**: 2 Moon + 1 Fire + 1 Plant + 1 Air + 1 Animal.
- **Innate trigger**: L1 fires again (2M+1F) → gather another Explorer. L2 (3M+2F) **not** reached this turn (only 1 Fire).
- **E/CP state**: 3E entering → 1E after (2E spent) → end T2 with **1E banked / 0 CP remaining**.
- **Fear contribution**: 2 (Crops) + 3 (Favors conditional if triggered; 0 if not) = 2–5 Fear.
- **Milestone**: 1 City softened; Dahan-outnumber fear spike (conditional).

#### Turn 3

- **Growth**: **G1** — Reclaim all played + Gain 1 Minor. Hand now has all 4 Uniques + 2 Minors = 6 cards. No presence placed. State: **1E banked + 1E income (energy1 slot) = 2E / 2CP**.
- **Cards played**: 2 cards (2 CP available). Target innate L2 threshold. Play **Crops Wither + Mantle of Dread** — elements 2M+2F+1P+1A. Still 1 Moon short of L2 (need 3M+2F).
  - Alternative: play a Moon-heavy Minor if drafted (e.g., Land of Haunts and Embers brings M+F+Air; adds M). With 2 Moon-heavy cards you can hit 3M.
- **Innate trigger**: L1 fires; L2 conditional on Minor elements.
- **E/CP state**: 2E → 0E after 2 cards played (Crops+Mantle = 2E) → end T3 at 0E.
- **Fear contribution**: 2 (Crops) + 2 (Mantle) = 4 Fear.
- **Milestone**: Fresh full hand for T4+; T4 can play 2 cards aimed directly at L2 threshold once a Fire-heavy Minor is in play.

#### Turn 4 — state audit

After T3, you should have:

- **Presence**: 5 on board (3 starting + 1 from T1 G3 + 1 from T2 G2).
- **Energy / CP**: 1E income (from energy1 slot) + 0E banked = 1E / 2CP.
- **Engine**: All 4 Uniques reclaimed + 2 Minors drafted. Innate L1 firing reliably; L2 (3M+2F) reachable when Minor draft supports.
- **Fear pool**: 9–12 of ~8 (solo — Terror 2 should have flipped by end T3 or early T4).
- **Blight**: 0–1 (Concealing + Favors-gather prevented most ravage damage).

Pivot advice (via `si-rules-check`):

- **Missing Moon+Fire T1** — if starting hand somehow lacks Concealing or Mantle, fallback: G2 via Energy track → 1E/1CP, play Concealing alone; defer L1 firing to T2 Reclaim + Mantle play.
- **Cannot afford double-card T1** — check legal-play enumeration above; G3-via-CP is the enabler.
- **Fear pool stalling** — Favors's 3-Fear conditional needs Dahan-outnumber state; verify Dahan count exceeds Invader count post-gather.
- **No Invaders in Range 1 from starting presence** — Concealing Shadows targets Any Land (0 Range), so it's restricted to lands where you have presence; Mantle targets Any Spirit. The innate needs a land at Range 1 of presence or Sacred Site — if no such land has Invaders, L1 gathers from the nearest legal target.

### Opening B — Card Depth (Minor-heavy draft) 🟥

**When to pick this**: **Habsburg Mining L5+, Russia L5+**, or any long-game matchup where Minor draft density pays off via Fear-card Events or matchup-specific Minors. Trades T1 innate-firing for a larger T3+ hand.

**Target arc**: 2 Minors drafted by T2 · Reclaim T3 → 6+ cards in hand · innate firings cap at L1 until Minor-draft supplies Fire for L2.

#### Turn 1

- **Growth**: **G2, revealing CP track** (uncover `card2`). Post-growth: **0E / 2CP**, 1 Minor drafted.
- **Cards played**: **Concealing Shadows** (0E Fast). If a 0-cost Minor was also drafted (e.g., Land of Haunts and Embers which is 0E Fast M+F+Air, or Shadows of the Burning Forest which is 0E Slow M+F+P), play that as the second card.
  - If no 0-cost Minor offered: play Concealing alone; banked 2nd card-play slot unused.
- **Elements (Concealing alone)**: 1 Moon + 1 Air. Innate L1 **does not fire** (needs 2M+1F).
- **Elements (Concealing + Moon+Fire 0-cost Minor)**: 2 Moon + 1 Fire + additional → **innate L1 fires**, conditional on Minor draft.
- **E/CP state**: 0E / 1CP entering → G2 makes it 0E / 2CP → 0E / remaining CP depending on plays.
- **Fear contribution**: 1 (Concealing) + 0–2 (from Minor if fear-bearing) = 1–3 Fear.
- **Milestone**: 1 Minor drafted (prioritize Moon+Fire dual-element to enable innate); broad hand for T3 reclaim value.

#### Turn 2

- **Growth**: **G2, revealing Energy track** (uncover `energy1`). Post-growth: **1E income / 2CP** (CP already at card2), 1 Minor drafted.
- **Cards played**: up to 2 cards at cost ≤ 1E total. Options:
  - **Crops Wither** (1E) alone — elements M+F+P.
  - **Mantle of Dread** (1E) alone — elements M+F+Air.
  - **Favors Called Due** (1E) alone — elements M+Air+Animal (no Fire).
  - **Concealing Shadows** (reclaimed... wait, not reclaimed, in discard) unless drafted Minor is 0-cost: a 0E Minor + 1E Unique.
- **Innate trigger**: L1 fires if elements sum to 2M+1F.
- **E/CP state**: 1E income + 0E banked = 1E → spend 1E → 0E.
- **Fear contribution**: 2 (Crops or Mantle) + optional from Minor.
- **Milestone**: 2 Minors total drafted; hand pool at 5 Uniques (4 original - 2 played + reclaim doesn't help yet) — wait actually only 2 Uniques played (T1 Concealing + T2 one Unique), so hand = 4 - 2 = 2 Uniques + 2 Minors = 4 cards.

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. Reclaim brings back Concealing + T2's played Unique. State: **1E / 2CP**, hand = 4 Uniques + 3 Minors = 7 cards.
- **Cards played**: 2 cards (CP limit), cost ≤ 1E. Aim for element mix 3M+2F for L2 innate. Example: Concealing (0E, 1M+1A) + Crops Wither (1E, 1M+1F+1P) = 2M+1F+1A+1P. Still 1 Moon short of L2.
- **Innate**: L1 fires (2M+1F reached).
- **Milestone**: card depth ready for T4–T5 push toward L2.

#### Turn 4 — state audit

After T3:

- **Presence**: 5 on board (3 starting + 1 T1 G2 + 1 T2 G2).
- **Energy/CP**: 1E / 2CP sustained.
- **Engine**: L1 firing reliably; L2 conditional on Minor draft containing Fire.
- **Fear pool**: 6–9 (slower than Opening A).
- **Milestone**: `[VERIFY mid-game]` — Major-acquisition path for Shadows is via fear-card Events (e.g., "Gain a Major Power") rather than a direct growth option; Brett should play-test to see how reliably Major-shop materializes.

`[VERIFY]` — this opening is tentative. Fewer T1 fear, but deeper hand. Better for games that go long.

### Opening C — Heavy Energy Bank (Fear-suppression matchups) 🟥

**When to pick this**: **Sweden L3+, Russia L3+** — where fear-card penalties make direct fear-rush weaker. Energy-bank strategy leans on Mantle's partner-push + Innate L2 Explorer-destroy (not fear) as the late-game output.

**Target arc**: Deep energy bank T1–T2 · Innate L2 firing T3 via Reclaim-reconstructed hand · Explorer-destroy as the alt-fear-source.

#### Turn 1

- **Growth**: **G3, revealing Energy track** (uncover `energy1`). Post-growth: **1E income + 3E growth-effect = 4E / 1CP** (CP track unrevealed, still at card1).
- **Cards played**: 1 card only (1CP limit). **Mantle of Dread** (1E Slow, Moon+Fire+Air) — best single-card T1 output (2 Fear + self-push).
  - Alternative: Crops Wither (1E) if a City/Town is in range 0 of your presence for the replace effect.
  - Concealing Shadows (0E) is OK too but lower fear ceiling; Mantle is the higher-fear pick here.
- **Elements played**: 1 Moon + 1 Fire + 1 Air (Mantle only). Innate L1 (2M+1F) **does not fire** — only 1 Moon.
- **E/CP state**: 0E / 1CP entering → G3 makes it 4E / 1CP → spend 1E → **3E banked / 0CP**.
- **Fear contribution**: 2 (Mantle).
- **Milestone**: 1 Explorer + 1 Town pushed (via Mantle self-target); 3E banked for T2.

#### Turn 2

- **Growth**: **G3, revealing CP track** (uncover `card2`). Post-growth: **3E banked + 1E income + 3E growth-effect = 7E / 2CP**.
- **Cards played**: 2 cards. Crops Wither (1E) + Favors Called Due (1E) = 2E spent. Elements 2M+1F+1P+1A+1An → L1 fires; L2 (3M+2F) still short.
- **E/CP state**: 7E → 5E after 2 cards → **5E banked entering T3** (massive).
- **Fear contribution**: 2 (Crops) + 3 (Favors conditional) = 2–5 Fear.
- **Milestone**: 5E bank carries into T3 — enables heavy T3–T4 plays including multiple 1E cards or draft-shopping.

#### Turn 3

- **Growth**: **G1** — Reclaim + Gain 1 Minor. State: **5E banked + 1E income = 6E / 2CP**. Hand = 4 Uniques + 2 Minors.
- **Cards played**: 2 cards, up to 6E budget. Play the two cards that best fit the L2 threshold (3M+2F): e.g., Crops Wither + Mantle of Dread = 2M+2F+1A+1P. Still need 1 Moon more for L2.
  - If a Moon-bearing Minor is in hand, play it instead of Mantle: Crops + Minor (M-heavy) could clear L2.
- **Innate**: L1 fires; L2 conditional.
- **E/CP state**: 6E → 4E after 2 cards → 4E banked entering T4.
- **Milestone**: deep energy bank sustains; even on fear-suppressed adversaries, Mantle's pushes + L2 destroys keep board pressure manageable.

#### Turn 4 — state audit

- **Presence**: 5 on board.
- **Energy/CP**: ~4E banked + 1E income = 5E / 2CP.
- **Engine**: L1 reliable, L2 element-draft-dependent.
- **Fear pool**: lower than Opening A due to fewer Fear-generating cards per turn + adversary fear suppression. Target: 5–7 by T4.
- **Strategy**: accept slow Terror progression; rely on Mantle pushes + innate L2 destroys for damage/control.

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
  Adv -->|Habsburg Mining L5+| B[Opening B - Major Shop]
  Adv -->|Multi-handed with board-carry partner| A
  Adv -->|Other| A
</pre>

## Suggested Draft Cards (Wiki-recommended, verified)

The Wiki's `suggestedcard` field lists 7 community-recommended draft picks for Shadows — all full text parsed:

### Minor Powers (5)

| Card                        | Cost | Speed | Range | Target              | Elements            | Effect                                                                 |
|-----------------------------|------|-------|-------|---------------------|---------------------|------------------------------------------------------------------------|
| **Dark and Tangled Woods**  | 1    | Fast  | 1     | Any Land            | Moon, Earth, Plant  | 2 Fear. If target land is a Mountain or Jungle, Defend 3.              |
| **Shadows of the Burning Forest** | 0 | Slow | 0    | Land with 1+ Invaders | Moon, Fire, Plant | 2 Fear. If Mountain or Jungle, Push 1 Explorer and 1 Town.             |
| **Land of Haunts and Embers** | 0  | Fast  | 2     | Any Land            | Moon, Fire, Air     | 2 Fear. Push up to 2 Explorers/Towns. If Blight is present, 2 Fear and Push up to 2 Explorers/Towns. Add 1 Blight. |
| **Call of the Dahan Ways**  | 1    | Slow  | 1     | Land with Dahan     | Moon, Water, Animal | Replace 1 Explorer with 1 Dahan.                                       |
| **Visions of Fiery Doom**   | 1    | Fast  | 0     | Any Land            | Moon, Fire          | 1 Fear. Push 1 Explorer/Town.                                          |

**Historical note**: "Dark and Tangled Woods" is a Minor Power card — an earlier revision of this chapter mistakenly labeled a corrupted version of this name as a Shadows innate. Resolved.

### Major Powers (2)

| Card                      | Cost | Speed | Range        | Target   | Elements    | Effect                                                                 |
|---------------------------|------|-------|--------------|----------|-------------|------------------------------------------------------------------------|
| **The Jungle Hungers**    | 3    | Slow  | 1 (Jungle)   | Any Land | Moon, Plant | Destroy all Explorers and all Towns. Destroy all Dahan.                |
| **Terrifying Nightmares** | 4    | Fast  | 2            | Any Land | Moon, Air   | 2 Fear. Push up to 4 Explorers/Towns.                                  |

**⚠️ Jungle Hungers caveat**: *Destroys all Dahan* in the target land. Anti-synergy with dahan-dependent partners (Thunderspeaker, Hearth-Vigil) and with Shadows's own Favors Called Due / Shadows of the Dahan reliance. Draft carefully in multiplayer.

**Terrifying Nightmares** is the Shadows Major of choice — fast, 2 Fear + 4 pushes, Moon+Air alignment.

## Aspects (all 5, Wiki-verified mechanics)

All 5 Shadows aspects **replace** the Shadows of the Dahan special rule. Four raise complexity; one lowers it.

### Amorphous (Promo Pack 2 — Feather and Flame) — Higher Complexity

**Replaces**: Shadows of the Dahan.

> **Shadows Partake of Amorphous Space**: During each Fast phase, you may move 1 of your Presence to an adjacent land, or to a land with Dahan anywhere on the island. During each Slow phase, you may move 1 of your Presence to an adjacent land, or to a land with Dahan anywhere on the island.

**Strategic shift**: converts the 1-Energy-per-range-extension tax into 2 *free* presence-moves per turn. Plays as a mobility-specialist variant — physical presence motion instead of virtual range-extension.

### Foreboding (Promo Pack 2 — Feather and Flame) — Higher Complexity

**Replaces**: Shadows of the Dahan with a **new Innate**, *Stretch Out Coils of Foreboding Dread* (Fast · Range 2 · Any Land):

- **2 Air**: Your other Powers may ignore Range when targeting the target land.
- **1 Moon**: After an Action generates Fear in target land (incl. Town/City destruction): Push up to 1 Explorer per Fear / 1 Town per 2 Fear.
- **2 Fire**: 1 Fear.
- **2 Moon + 4 Air**: 2 Fear.

**Strategic shift**: swaps a per-Power spatial rule for a land-targeted innate. Range-ignoring is gated to one targeted land (rather than all Dahan lands). Converts fear-on-destruction into pushes for control value.

### Madness (Jagged Earth) — Higher Complexity

**Replaces**: Shadows of the Dahan. Adds 2 new rules:

> **Shadows Cast a Subtle Madness**: When you add Presence during Growth, you may also add 1 Strife in that land.

> **Glimpse of the Shadowed Void**: When your Presence is Destroyed, if Invaders are present, 1 Fear per Presence Destroyed there.

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

## Card Priority Ratings

Shadows is **Mixed** draft-bias — Minor-heavy T1–T3, 1 Major T5+ as closer.

### Uniques (all A-tier)

| Card                   | Grade | Notes                                                   |
|------------------------|-------|---------------------------------------------------------|
| Concealing Shadows     | A+    | 0 Energy; plays every turn.                             |
| Crops Wither and Fade  | A     | Downgrade + 2 Fear; City/Town-dense targets.            |
| Favors Called Due      | A     | Conditional 3 Fear; needs Dahan-dense targets.          |
| Mantle of Dread        | A- (solo) / A+ (MP) | Partner-support utility.                  |

### Minors to Target (Wiki-suggested + general drafts)

Top priority: the 5 Minors listed above. Beyond those:

- Any 0-cost Moon Minor (threshold essential).
- Any Moon+Fire dual-element Minor (Innate L2 unlock).
- Dahan-preserving or Dahan-summoning Minors (Call of the Dahan Ways is the archetype).

### Majors (Wiki-suggested)

- **Terrifying Nightmares** (A+) — Shadows's premier Major.
- **The Jungle Hungers** (A, with caveat) — board wipe; only in solo or when dahan loss is acceptable.

### Cards to AVOID drafting

- Majors requiring Earth/Plant-only thresholds (Shadows has no reliable Earth or Plant without specific Minor drafts).
- Pure-damage Majors that don't pay fear (Shadows has innate L3 damage already).

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
- Innate L2 firing (destroy 2 Explorers + 2 Fear per turn).
- Crops Wither softens City turns.
- Favors Called Due's 3-Fear trigger becomes available.
- Fear pool heading toward Terror 2 flip.

### Late (T7+)
- Terror 2 → Terror 3 transition.
- Gain Terrifying Nightmares Major if offered; forget Mantle of Dread (weakest in solo) or the weakest-matchup Unique.
- Innate L3 available if 4 Moon + 3 Fire + 2 Air reliably on track.

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
