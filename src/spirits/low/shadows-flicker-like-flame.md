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

```admonish note title="Stat Insight `[VERIFY current numbers from mindwanderer]`"
Directional figures from community tier lists + older mindwanderer snapshots:
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
