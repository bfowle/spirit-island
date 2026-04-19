# Shadows Flicker Like Flame

```admonish success title="Mechanics verified 2026-04-19"
Card data, innate text, and special rules below were parsed deterministically from the Spirit Island Wiki using `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template-field extraction). No LLM summarization in that pipeline. Strategic prose is opinion.

Aspects verified against physical copy per Brett 2026-04-19: Amorphous + Foreboding (B&C), Madness + Reach (JE), Dark Fire (NI).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base Game                                          |
| Complexity            | Low                                                |
| Play Difficulty       | 1 `[VERIFY]`                                       |
| Archetypes            | Fear generation · Explorer control · Dahan-leverage |
| Primary Elements      | Moon (primary), Fire, Air (Animal/Plant on 2 Uniques) |
| Special Rule          | Shadows of the Dahan (1 Energy extends any Power's Range to Dahan lands) |
| Aspects               | **B&C**: Amorphous, Foreboding · **JE**: Madness, Reach · **NI**: Dark Fire |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
```

## Spirit Overview — Framing

Shadows is the **fear-generating Explorer controller**. Three of four Uniques generate Fear directly (1, 2, 2 respectively + a conditional 3), and the innate chains Explorer-gathering into Explorer-destruction into multi-Invader damage as element thresholds rise. The Shadows-of-the-Dahan special rule effectively removes range as a constraint — 1 Energy lets any Power hit any land with Dahan.

**One-line fantasy**: the unseen hand. Where Dahan walk, Shadows walks alongside them; where invaders step, shadows gather and fear rises.

**The honest complexity signal**: Low is correct. Shadows has a small toolkit and a clear archetype. Strategic depth comes from **when to trigger which level of the innate**:
- Level 1 is positional (gather an Explorer — reposition without killing).
- Level 2 is lethal (destroy up to 2 Explorers + fear).
- Level 3 is multi-target (3 damage across any invaders).

Patience on Level 2+ pays: gathering Explorers first, then destroying 2 at a time, is higher fear output than picking off Explorers one-by-one.

## Core Mechanics & Special Rules

### Special Rule: Shadows of the Dahan

> Whenever you use a Power, you may pay 1 Energy to target a land with Dahan regardless of the Power's Range. *(Power Cards or your Innate Powers.)*

**Strategic implication**: the most flexible targeting special rule of any base spirit. Every Power — card or innate — can reach any Dahan-occupied land for 1 Energy. This means:

- Range 0 Power Cards effectively get unlimited range to Dahan lands.
- The Innate's "Range 1 from Sacred Site" constraint is bypassed entirely for Dahan lands.
- Dahan preservation across the board directly expands Shadows's reach.

**Budget 1 Energy per turn** for this rule's use. Running at 0 Energy eliminates your spatial flexibility.

### Innate: Darkness Swallows the Unwary

- **Speed**: Fast
- **Range**: 1, optionally from a Sacred Site
- **Target**: Any land

| Level | Thresholds         | Effect                                                         |
|-------|--------------------|----------------------------------------------------------------|
| 1     | 2 Moon + 1 Fire    | Gather 1 Explorer.                                             |
| 2     | 3 Moon + 2 Fire    | Destroy up to 2 Explorer. 1 Fear per Explorer destroyed.       |
| 3     | 4 Moon + 3 Fire + 2 Air | 3 Damage. 1 Fear per Invader destroyed by this Damage.    |

**Strategic implication**: Level 1 *repositions* an Explorer (gather into the target); Level 2+ destroys. The Level 1 "gather" alone can prevent a Build by moving an Explorer out of the target land, without burning a Unique. Level 2 is the fear engine — destroy 2 Explorers = 2 Fear per turn, sustained.

### Unique Cards

#### Concealing Shadows

- **Cost**: 0 Energy
- **Speed**: Fast
- **Range**: 0 · **Target**: Any Land
- **Elements**: Moon, Air
- **Effect**: *1 Fear. Dahan take no Damage from Ravaging Invaders this turn.*

**Strategic use**: defensive + fear. Dahan-protection on a Ravage land prevents Dahan loss entirely for that turn. Pairs well with Favors Called Due (which wants Dahan-outnumber-Invaders).

#### Crops Wither and Fade

- **Cost**: 1 Energy
- **Speed**: Slow
- **Range**: 0 · **Target**: Any Land
- **Elements**: Moon, Fire, Plant
- **Effect**: *2 Fear. Replace 1 Town with 1 Explorer. **OR** Replace 1 City with 1 Town.*

**Strategic use**: this is a **downgrade**, not a destroy. A City becomes a Town (still present but less threatening); a Town becomes an Explorer. The replaced invader remains in the land — land pressure stays the same, but Ravage damage drops and future Build behavior softens. Plus 2 Fear.

Combined with the innate: replace a City → Town (Crops Wither), then gather/destroy the Explorers that were also in that land next turn.

#### Favors Called Due

- **Cost**: 1 Energy
- **Speed**: Slow
- **Range**: 1 · **Target**: Any Land
- **Elements**: Moon, Air, Animal
- **Effect**: *Gather up to 4 Dahan. If Invaders are present and Dahan now outnumber them, 3 Fear.*

**Strategic use**: Dahan reinforcement into a land where Invaders are present. Outnumbering triggers 3 Fear — a massive fear spike. Requires 4+ Dahan in the gather-radius and an Invader-present target with fewer Invaders than the gathered Dahan count.

#### Mantle of Dread

- **Cost**: 1 Energy
- **Speed**: Slow
- **Range**: N/A · **Target**: Any Spirit
- **Elements**: Moon, Fire, Air
- **Effect**: *2 Fear. Target Spirit may Push 1 Explorer and 1 Town from a land where it has Presence.*

**Strategic use**: **partner-support card**. Target another Spirit (or self); that Spirit pushes 1 Explorer + 1 Town out of one of their Presence lands. In multiplayer, this is Shadows helping a partner defuse a threatening land. In solo, Shadows targets self.

## Key Strategic Principles

1. **Fear output per turn can be huge.** Full engine: 1 (Concealing) + 2 (Crops) + 3 (Favors, conditional) + 2 (Mantle) + 1–2 (innate L2) = up to 10+ Fear in a single turn. Shadows is *the* fear spirit of the base game in raw output.
2. **Shadows of the Dahan is the spatial engine.** Budget 1 Energy per turn for range-extension. Don't let Energy hit 0.
3. **Crops Wither is downgrade, not destroy.** Use to soften Ravages, not to kill. City→Town cuts Ravage damage by 1 and removes a future Build threat.
4. **Favors Called Due wants Dahan clusters.** 4+ Dahan in the radius + Invader-present target = 3 Fear. Preserve Dahan (Concealing Shadows) so Favors fires.
5. **Innate Level 1 is positional, not lethal.** Gather an Explorer out of a Build target = no Town next turn. You don't always need Level 2 to solve a land.
6. **Moon is primary; Fire is secondary.** Every innate level wants Moon + Fire. Air opens at Level 3 and on Mantle. Animal only appears on Favors.
7. **Mantle of Dread is a partner-help tool.** In solo, still useful (target self). In multiplayer, unique utility.

```admonish tip title="Pro Tip"
Before Slow powers, count: is a Favors Called Due 3-Fear trigger possible? If Dahan count (including gatherable) would exceed Invader count in a land with Invaders, Favors is live. This is the single biggest fear-rush turn Shadows has.
```

## Opening Strategy

### Opening A — Hybrid, Minor-heavy 🟥 (needs playtest)

**When to pick this**: default vs. most base/B&C adversaries.

**Target arc**: Level 1 innate T1 · Level 2 innate T3–T4 · Terror 2 flip by T6.

#### Turn 1

- **Growth**: 1 of the 3 growth options (see [VERIFY growth options against spirit panel]).
- **Cards played**: depends on opener; typically Concealing Shadows (0 Energy — free fear + dahan protection) + 1 more Unique or drafted Minor.
- **Presence placement**: Dahan-adjacent inland land.
- **Elements by end**: 2 Moon + 1 Fire (Level 1 innate threshold).
- **Milestone**: Innate L1 gathers an Explorer; 1 Fear via Concealing.

#### Turn 2

- **Growth**: gain energy or card as curve demands.
- **Cards played**: Crops Wither and Fade (if a Town/City is placed) or another Unique.
- **Elements by end**: 3 Moon + 2 Fire (Level 2 innate threshold).
- **Milestone**: 2 Explorers destroyed via innate L2; 2 Fear from innate + 2 Fear from Crops.

#### Turn 3

- **Growth**: Reclaim if available.
- **Cards played**: 2–3 cards.
- **Milestone**: fear pool accumulating toward Terror 2.

#### Turn 4 — state audit

After T3:
- Presence 5–6 (total placed; Shadows's presence track limits exact count — `[VERIFY presence cap from track]`).
- Fear pool: 5–7 of 8 (solo).
- Innate firing Level 2+ reliably.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Shadows] --> Adv{Adversary?}
  Adv -->|Brandenburg-Prussia| A[Opening A - Hybrid]
  Adv -->|England| A
  Adv -->|Scotland| A
  Adv -->|Sweden L0-L2| A
  Adv -->|Sweden L3+| B[Opening variant - slower-scaling]
  Adv -->|Russia| B
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Moon (primary — all innate levels), Fire (secondary — Level 2+), Air (Level 3 + Concealing + Mantle), Animal (Favors only), Plant (Crops only).

**Aspects** (verified per Brett 2026-04-19):

- **Amorphous** (Branch & Claw) — `[VERIFY aspect mechanics]`.
- **Foreboding** (Branch & Claw) — `[VERIFY aspect mechanics]`.
- **Madness** (Jagged Earth) — `[VERIFY aspect mechanics]`.
- **Reach** (Jagged Earth) — `[VERIFY aspect mechanics]`.
- **Dark Fire** (Nature Incarnate) — `[VERIFY aspect mechanics]`.

Aspect mechanics will be scraped in a follow-up pass once the parser supports aspect-page templates.

## Card Priority Ratings

Shadows is **Mixed** draft-bias — Minor-heavy early, 1 Major late.

### Unique Cards

All 4 Uniques grade in-game:

| Card                  | Grade | Notes                                                   |
|-----------------------|-------|---------------------------------------------------------|
| Concealing Shadows    | A+    | 0 Energy; plays every turn; defensive + fear.           |
| Crops Wither and Fade | A     | Downgrade + 2 Fear; City/Town-dense targets.            |
| Favors Called Due     | A     | Conditional 3 Fear; needs Dahan-dense lands + Invaders. |
| Mantle of Dread       | A- (solo) / A+ (multiplayer) | Partner-support utility.             |

### Minors to Target

| Card type                             | Grade | Why                                                |
|---------------------------------------|-------|----------------------------------------------------|
| 0-cost Moon Minor                     | A+    | Every threshold wants Moon.                        |
| Moon + Fire dual Minor                | A+    | Innate Level 2 unlock.                             |
| Animal-bearing Minor                  | A     | Favors Called Due synergy.                         |
| Dahan-preserving / Dahan-gathering Minor | A  | Feeds Favors + Shadows of the Dahan targeting.    |

Specific Minor card names `[VERIFY against your Minor deck]` — the parser doesn't yet list Minor cards per-draft.

### Majors that Over-perform on Shadows

| Card                    | Grade | Why                                                 |
|-------------------------|-------|-----------------------------------------------------|
| Terrifying Nightmares   | A+    | Fear finisher; Moon-element alignment. `[VERIFY]`   |
| Paralyzing Fright       | A     | 3-cost closer. `[VERIFY]`                           |

`[VERIFY Major card text against your Major deck]` — existing ratings are inherited from the pre-audit chapter and should be cross-checked.

### Cards to AVOID drafting

- Majors that require Earth or Plant thresholds Shadows can't hit.
- Pure-damage Majors — Shadows generates damage via innate L3 without extra Majors.

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes                                                      |
|----------------------|----|----|----|----|-----------------------------------------------------------|
| England              | A  | A- | B+ | B  | Slow builds; fear-rush works. **L5 cliff**: buildings +1 HP — innate Level 3 (3 damage) insufficient for 4-HP Cities without Crops Wither's City→Town downgrade first. |
| Brandenburg-Prussia  | A  | A- | B+ | B  | Cities = 5 fear per kill (high fear-per-kill). Favorable. |
| Sweden               | A- | B+ | B  | B- | Fear penalties reduce engine output at L2+.                |
| France (Plantation)  | B+ | B  | B  | C+ | Dahan capture threatens the Shadows of the Dahan targeting pool. |
| Habsburg Mining      | A  | B+ | B  | C+ | Scaling outpaces fear-rush late.                           |
| Russia               | B+ | B  | B- | C  | Fear-suppression mid-late.                                 |
| Scotland             | A  | B+ | B+ | B  | Favorable.                                                 |
| Habsburg Livestock   | A  | A- | B  | B  | Favorable.                                                 |

### Strategy Cliff — England L5

```admonish info title="Strategy Cliff — England L5"
**What changes at L5**: buildings gain +1 HP (Town = 3 HP, City = 4 HP).

**Impact on Shadows**: innate Level 3 does 3 Damage — kills a 3-HP Town outright, but not a 4-HP City. Crops Wither and Fade's City→Town replacement becomes the way to pre-soften Cities for innate kill.

**Mitigation**: sequence Crops Wither (turn N) → innate L2/L3 or Major (turn N+1) for City clears. Build fear pool for Terror 2 flip before T6.
```

### Strategy Cliff — Sweden L2+ / Russia L3+

```admonish info title="Strategy Cliff — Fear-suppression adversaries"
**What changes**: Sweden L2+ and Russia L3+ have fear-suppression rules.

**Impact on Shadows**: Terror flips slower. Fear-rush timing slips by 1–2 rounds.

**Mitigation**: shift to Crops Wither as board-pressure tool rather than fear-spike. Innate L3 + Major-closer becomes the kill path.
```

## Board / Map Configuration

`[VERIFY all ratings]` — ratings below are directional placeholders until Brett can play + rate; community data on Shadows board preferences is not strongly sourced.

### Base game boards (A–D)

- **Likely favorable**: boards with high Dahan starting count and multiple Dahan-adjacent lands (leverages Shadows of the Dahan).
- **Likely neutral**: most base boards.
- **Likely unfavorable**: sparse-Dahan boards.

Specific A/B/C/D ratings `[VERIFY during play]`.

### Jagged Earth boards (E–H)

`[VERIFY]` — ratings to come.

### Thematic map

Shadows works on thematic but the spatial range-extension benefit diminishes on the tighter thematic connectivity.

### Scenario-forced maps

- **Dahan Insurrection**: strongly favorable for Shadows (extra Dahan + scenario rewards fear-heavy spirits).
- **Blitz**: moderate — Shadows's T1 output is fine, but Crops Wither's long-term engine needs T4–T6.
- **Rituals of Terror**: very favorable — fear-race + Shadows's native fear-rush.

## Game-Phase Strategy

### Early (T1–3)
- Place presence in Dahan-adjacent lands.
- Innate L1 to reposition Explorers away from Build targets.
- Concealing Shadows every turn (0 Energy is free value).
- Draft Moon + Fire Minors.

### Mid (T4–6)
- Innate L2 firing — destroy 2 Explorers per turn.
- Crops Wither on City-present lands.
- Favors Called Due with conditional 3 Fear triggering.

### Late (T7+)
- Terror 2 → Terror 3 transition.
- Mantle of Dread for partner support if multiplayer.
- Major closer if gained.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Bringer of Dreams and Nightmares** — double fear-engine; Bringer's To Dream a Thousand Deaths converts damage to fear; compounding.
- **Thunderspeaker** — Thunderspeaker grows Dahan density, feeding both Favors Called Due and Shadows of the Dahan targeting.
- **Ocean's Hungry Grasp** — Ocean drowns coasts; Shadows handles inland Dahan lands.
- **Any partner** — Mantle of Dread's partner-target push is useful on almost any ally's turn.
```

```admonish warning title="Anti-Synergy"
- **Heart of the Wildfire** — destroys Dahan via blight; shrinks Shadows's Dahan-targeting pool.
- **Vengeance as a Burning Plague** — blight-heavy; Dahan attrition.
- **Volcano Looming High** — destruction kills Dahan unconditionally.
```

## Common Mistakes

```admonish failure title="Common Mistake — Running at 0 Energy"
Shadows of the Dahan costs 1 Energy per range extension. If you're at 0 Energy, you can't target Dahan lands beyond base range. Budget 1E float every turn.
```

```admonish failure title="Common Mistake — Using Crops Wither as a kill card"
Crops Wither *replaces*, not destroys. A replaced City becomes a Town (still present, still Ravages next turn). Plan around the downgrade, not the kill.
```

```admonish failure title="Common Mistake — Drafting non-Moon Minors"
Every innate level wants Moon. Non-Moon Minors stall Level 1–2 innate firing.
```

```admonish failure title="Common Mistake — Missing Favors Called Due conditions"
3 Fear only triggers if Invaders are present AND Dahan (after gather) outnumber them. Counting is required before committing the play.
```

```admonish failure title="Common Mistake — Sacred-site hoarding"
Innate targets 'optionally from a Sacred Site.' Shadows's power is spread + Dahan-extension, not sacred-site density. Don't stack 2 presence in one land when 2 adjacent lands with Dahan are both reachable.
```

## Tempo Profile `[VERIFY against physical play]`

| Round | Energy | CP | Presence | Moon | Fire | Fear Contribution Per Turn | Key Play                                |
|-------|--------|----|----------|------|------|----------------------------|-----------------------------------------|
| 1     | 1E     | 2  | 4        | 2    | 1    | 1–2 (Concealing + innate L1)| Concealing + 0-cost Minor              |
| 2     | 1E     | 2  | 5        | 2–3  | 1–2  | 2–3 (Concealing reclaimed + Minor)| Reclaim + Minor                   |
| 3     | 1E     | 2  | 6        | 3    | 2    | 3–5 (Crops Wither + innate L2) | 3-card turn: Concealing + Crops + Minor |
| 4     | 1–2E   | 3  | 7        | 3+   | 2+   | 4–6 (innate L2 sustained + Favors conditional) | Favors Called Due if Dahan ready |
| 5     | 2E     | 3  | 7        | 3+   | 2+   | 4–6                        | Major gain prep                         |
| 6     | 2E     | 3  | 7        | 3+   | 3    | 5–7                        | Terror 2 flip                           |
| 7     | 2–3E   | 3  | 6        | 3+   | 3    | 6–8                        | Major fires (if gained)                 |
| 8     | 3E     | 3  | 5        | 4+   | 3    | 7–10 with L3 innate        | Terror 3 close                          |

Cliff turn: **T3**. Innate Level 2 must fire, and Crops Wither must land on a meaningful target.

## Major vs. Minor

**Draft bias**: Mixed (Minor-heavy T1–T3; 1 Major T5+ as closer).

## Expansion Sensitivity

- **Base only**: fully functional; 4 Uniques + innate engine.
- **+ Branch & Claw**: Amorphous + Foreboding aspects unlock; events + blight deck add variance.
- **+ Jagged Earth**: Madness + Reach aspects unlock; deeper Minor/Major pool.
- **+ Nature Incarnate**: Dark Fire aspect unlocks.

Aspect-specific strategic notes pending aspect-page scraping.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY current numbers]`:
- Solo L6 win rate: approximately 50–55% (rough directional).
- Best adversary: Brandenburg-Prussia L6 (fear-dense, favorable).
- Worst adversary: Russia L6 (fear suppression).
- 2-handed: Shadows + Bringer commonly cited as top fear-rush pair.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/shadows-flicker-like-flame.json` — parsed deterministically from Spirit Island Wiki 2026-04-19 via `scripts/wiki-fetch.py`.
- **Aspect list**: Brett physical-copy verification 2026-04-19.
- Spirit Island Wiki — [Shadows page](https://spiritislandwiki.com/index.php?title=Shadows_Flicker_Like_Flame).
- Cross-reference: [Dahan fundamentals](../../fundamentals/dahan.md), [Fear Rush archetype](../../combos/fear-rush.md), [si-wiki-fetch skill](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Physical-copy verified: partial (Uniques, innate, aspects 2026-04-19). Board ratings, draft-specific card names, Major card text still pending verification.*

*Last revised: 2026-04-19 — v0.2.3 (scripted-parser rewrite)*
