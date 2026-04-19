# {Spirit Name}

<!--
CANONICAL FORMAT — derived from Rei's BGG guides (Many Minds, Shroud, Vengeance).
Every spirit chapter must conform to this spine. Deviate only where the spirit's
design genuinely lacks a section (e.g., a spirit with no special rules can say so
in one line rather than inventing content).

Notation (from latentoctopus.github.io/glossary/):
  - Gn     : n-th growth option, left to right (1-indexed)
  - nE     : energy per turn
  - nCP    : card plays per turn
  - x/y    : energy/CP combined
  - CE     : cumulative energy over the game
  - Hybrid : presence taken from both tracks roughly equally
  - Full top / Full bottom : one track emptied, minimal from the other
-->

```admonish abstract title="At a Glance"
| Field                 | Value                                           |
|-----------------------|-------------------------------------------------|
| Expansion             | Base / B&C / JE / F&F / Horizons / NI / Promo   |
| Complexity            | Low / Moderate / High / Very High               |
| Play Difficulty       | 0 / 1 / 2 / 3 (per official rating)             |
| Archetypes            | (e.g., fear-farm, terrain-denial, dahan-rush)   |
| Primary Elements      | (e.g., Air, Animal)                             |
| Typical Opening       | Full bottom / Hybrid / Full top                 |
| Typical Draft Bias    | Minor / Mixed / Major                           |
| Rei's Guide           | [BGG thread](URL) — or: "not covered by Rei"    |
| latentoctopus Opening | [name](URL) + confidence badge (red/yellow/green)|
```

## Spirit Overview — Framing

One or two paragraphs. What does this spirit feel like to play? What is its core fantasy? What's the one-line answer to "what does this spirit do"?

Keep it honest about complexity: don't oversell an easy spirit as strategic depth, don't hide complexity in a "High" spirit behind vibes.

## Core Mechanics & Special Rules

Each special rule and key innate gets a strategic, not rules-text, treatment. Show what the rule *enables* and what decisions it shapes.

### {Special Rule 1 Name}
Explain the strategic implication. Example (Shroud's Mists Shift and Flow): "gather presence into target or adjacent land to meet targeting — makes the spirit's effective range 1 larger than it looks."

### {Innate 1 Name}
Thresholds, typical unlock round, engineering cost, failure modes.

### {Innate 2 Name}
Same.

## Key Strategic Principles

Numbered list of 5–10 principles, Rei-style. Each is a decision-shaping claim, not generic advice.

1. **{Principle name}** — Why this is true + how it shows up in play.
2. **{Principle name}** — ...
3. **{Principle name}** — ...

```admonish tip title="Pro Tip"
If a principle has a specific in-game heuristic (e.g., "Keep a fire-plant element in hand on T1"), pull it out into a Pro Tip admonition.
```

## Opening Strategy

Each opening variant below is a full T1–T4 rehearsal. Pick the variant that matches your adversary + draft plan, then execute turn-by-turn. These are the reps you'd do if you were prepping for a tournament game — don't improvise T1.

Every variant carries a confidence badge (🟥 tentative · 🟨 somewhat tested · 🟩 well-tested).

### Opening A — {latentoctopus or Rei variant name} {🟥/🟨/🟩}

**Source**: [latentoctopus: {variant name}](URL) / Rei BGG thread {URL}

**When to pick this**: {1–2 sentences — which adversaries, which draft plan, which matchups.}

**Target arc**: {3 CP by T4, Level 2 innate active by T3, Terror 2 flip by T6, …}

#### Turn 1

- **Growth**: G{n} — {what this unlocks: presence placement / energy / CP / element / reclaim}
- **Cards played**: {Unique 1 (fast)}, {Unique or Minor 2 (slow)} — total energy {X}
- **Presence placement**: {land X} (jungle, coastal — note terrain type if load-bearing)
- **Elements by end**: {e.g., 2 Moon, 1 Fire}
- **E / CP state**: enter {1E / 2CP} → exit {0E / 2CP}
- **Milestone**: {one-sentence check — "one invader in target land strife-ed"}

#### Turn 2

- **Growth**: G{n} — {unlock}
- **Cards played**: {card list}, fast/slow annotated
- **Presence placement**: {land Y}, reasoning
- **Elements by end**: {running total}
- **E / CP state**: {X / Y} → {X / Y}
- **Milestone**: {check}

#### Turn 3

- **Growth**: G{n} — typically reclaim or card gain here
- **Cards played**: {list, including reclaimed hand if applicable}
- **Presence placement**: {land}
- **Elements by end**: {total — usually the first level-2 innate threshold hits here}
- **E / CP state**: {X / Y} → {X / Y}
- **Milestone**: {"Level 2 innate firing reliably from next turn"}

#### Turn 4 (state audit, not a full plan)

After T3 you should have:

- {Presence count on board}, spread across {N lands}
- {Energy, CP}
- Engine state: {"Innate level 2 firing; 3 CP; hand rotation stable"}
- Fear pool: {range, e.g., "3–4 of 8 — on track for Terror 2 by T6"}
- Blight pool: {range, e.g., "0–1 — comfortable"}

If you're off by more than one step, you've diverged. Pivot advice:

- **Missing element X by T2** → {pivot plan, often "take a different Minor next gain and delay the innate threshold one turn"}
- **Energy-starved** → {"Skip a card play this turn; bank 1E for T4"}
- **Presence too sparse** → {"Shift next growth to G{n} even if you planned G{m}"}

### Opening B — {variant name} {🟥/🟨/🟩}

(Same structure as Opening A.)

**When to pick this**: ...

#### Turn 1 / Turn 2 / Turn 3 / Turn 4 audit
...

### Opening C (optional) — {e.g., Early Major variant}

If the spirit has 3+ distinct openings documented, include them here. If not, two openings is plenty.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Turn 1 — {Spirit}] --> Adv{Adversary?}
  Adv -->|England| A[Opening A]
  Adv -->|Sweden| B[Opening B]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: (e.g., Air, Animal)  
**Why**: what thresholds they unlock and how easy they are to reach.

**Aspects** (if applicable):
- **{Aspect name}** — what changes strategically, when to pick this aspect, win-rate delta (if known). Cross-link to [aspects chapter](../aspects.md).

## Card Priority Ratings

Letter grades A+ through F for the cards most relevant to this spirit. Unique powers get explicit coverage; majors/minors are the high-impact picks.

### Unique (Signature) Powers

| Card                  | Grade | Notes                                             |
|-----------------------|-------|---------------------------------------------------|
| {Unique Card 1}       | A     | Core engine card — play turn 1 most openings     |
| {Unique Card 2}       | A-    | Strong but sometimes forget for a Major         |
| {Unique Card 3}       | B+    | Situational                                       |

### Minors to Target

| Card               | Grade | Why with this spirit                              |
|--------------------|-------|---------------------------------------------------|
| {Minor}            | A     | ...                                               |

### Majors that Over-perform

| Card               | Grade | Why with this spirit                              |
|--------------------|-------|---------------------------------------------------|
| {Major}            | A     | ...                                               |

### Cards to AVOID drafting

- **{Card}** — why it misleads on this spirit.

## Adversary Matchup Matrix

Grades are directional (L0/L3/L5/L6 columns). **Always annotate per-level strategy cliffs** — specific levels where an adversary rule fundamentally changes this spirit's playstyle.

| Adversary               | L0 | L3 | L5 | L6 | Notes (call out level cliffs)                                           |
|-------------------------|----|----|----|----|-------------------------------------------------------------------------|
| England                 | A  | A- | B+ | B  | Slow; favors gradual setup. **L5 cliff**: buildings +1 HP changes kill-math for push/damage spirits. |
| Brandenburg-Prussia     | B+ | B  | B- | C+ | Rapid builds punish slow scaling. **L4 cliff**: Town-to-City accelerates mid-game. |
| Sweden                  | A  | A  | A- | B+ | Fear penalties active from L2+; fear-rush spirits collapse here.        |
| France (Plantation)     |    |    |    |    | **L3 cliff**: Plantation capture active; dahan-centric spirits struggle. |
| Habsburg Mining         |    |    |    |    |                                                                         |
| Russia                  |    |    |    |    | **L3 cliff**: fear suppression active; **L5 cliff**: Settler mechanics compound. |
| Scotland                |    |    |    |    |                                                                         |
| Habsburg Livestock      |    |    |    |    |                                                                         |

### Strategy cliff callouts

For each adversary with a level where strategy materially changes, add a short footnote:

```admonish info title="Strategy Cliff — {Adversary} L{N}"
**What changes at L{N}**: {specific rule text}.
**Impact on {spirit}**: {what tactic stops working / what new approach is needed}.
**Mitigation**: {how to adapt — draft different cards, open differently, partner with X}.
```

Example (River vs England L5):

```admonish info title="Strategy Cliff — England L5"
**What changes at L5**: buildings gain +1 HP.
**Impact on River**: push-and-1-damage kills now leave 1-HP survivors. Massive Flooding still kills Towns but requires Level 2+ threshold reliability.
**Mitigation**: draft a 2-damage Minor earlier; bias toward Flood the Hills (2 damage) over Wash Away (1 damage).
```

## Board Position Evaluation

Which boards favor or punish this spirit, using Rei's pattern.

- **Favorable**: Boards {A/B/C/D/E/F}. Why: {e.g., coastal density, inland isolation, specific terrain distributions}.
- **Neutral**: ...
- **Unfavorable**: ...

## Game-Phase Strategy

### Early (T1–3)
Goals, presence footprint, cards to have played, common panic moments.

### Mid (T4–6)
The "engine must be on" checkpoint. Key element thresholds. Card-play target.

### Late (T7+)
Win-condition plays. When to stop growing presence. Reclaim-cycle optimization.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **{Spirit A}** — why they complement (element gifting, territory split, fear share)
- **{Spirit B}** — why
- **{Spirit C}** — why
```

```admonish warning title="Anti-Synergy / Territory Conflicts"
- **{Spirit X}** — conflict reason (e.g., blight-sensitive partner when we intentionally blight)
```

## Common Mistakes

```admonish failure title="Common Mistake"
Playing X on round 1 — you sink tempo you need for Y and can't recover until T4.
```

```admonish failure title="Common Mistake"
Drafting the "obvious" element card when you actually need Z. The spirit rewards patience on element accumulation.
```

(5–8 of these, each concrete enough to recognize at the table.)

## Tempo Profile

Round-by-round expected energy/CP/presence state, and the "cliff" rounds where your tempo could collapse.

| Round | Energy | CP | Presence on Board | Key Play                      |
|-------|--------|----|--------------------|-------------------------------|
| 1     | 1E     | 2  | 10 of 13           | {what you must accomplish}    |
| 2     | 1E     | 2  | 9                  | ...                           |
| 3     | 2E     | 3  | 8                  | Unlock innate threshold X     |
| 4     | 2E     | 3  | 7                  | Engine fully running          |
| 5     | 3E     | 3  | 6                  | ...                           |

## Major vs. Minor — This Spirit Specifically

**Draft bias**: Minor-heavy / Mixed / Major-heavy / Flexible

Per the stats + the spirit's energy curve:

- {Rule 1 — e.g., "Don't gain a Major before T3 unless you're forgetting Flash-Fires."}
- {Rule 2}
- {Rule 3}

Cross-reference: [Major vs. Minor Fundamentals](../../fundamentals/major-vs-minor.md)

## Expansion Sensitivity

Main guide assumes all expansions in play. Call out where advice changes by expansion combination.

### What changes per expansion

- **Base only**: no events, no blight deck, thinner Minor/Major pool. Specific implications for {spirit}: {e.g., "Bringer's fear-card variance is lower without B&C fear cards; strategy shifts to T1 aggression"}.
- **+ Branch & Claw**: events introduce per-turn variance; blight deck adds blight-management complexity. For {spirit}: {e.g., "Wildfire's blight-positive strategy accelerates; Keeper's blight-removal gains blight-deck-specific targets"}.
- **+ Jagged Earth**: event deck replaced/expanded; Minor + Major pools deepened; aspects introduced. For {spirit}: {e.g., "fear-rush is harder — more fear cards exist but pool is deeper"}.
- **+ Feather & Flame / Promo Pack 2**: minor pool additions. For {spirit}: {e.g., "a specific 0-cost Water Minor from Promo 2 changes River's opening"}.
- **+ Horizons**: low-complexity spirit variants; doesn't affect most established spirits' strategy.
- **+ Nature Incarnate**: deepest pool impact; Incarna mechanic; new aspects. For {spirit}: {spirit-specific changes}.

### Expansion decision guide

<pre class="mermaid">
graph TD
  Start[What expansions are active?] --> Base[Base only]
  Start --> BC[+ Branch & Claw]
  Start --> JE[+ Jagged Earth]
  Start --> All[All expansions]
  Base --> BaseAdvice[Simpler pool; T1 aggression prioritized]
  BC --> BCAdvice[Events add variance; account for blight deck]
  JE --> JEAdvice[Pool depth — fear-rush harder; aspects available]
  All --> AllAdvice[Guide's default assumption; all advice above applies]
</pre>

## Stat Snapshot

```admonish note title="Stat Insight"
Per [mindwanderer](https://mindwanderer.net/si/stats.html) (Spirit Island Digital, data as of YYYY-MM-DD, N = XXXX games):
- Overall win rate: XX% (95% CI ±Y%)
- L6 win rate: XX%
- Best adversary: {Adv} at XX%
- Worst adversary: {Adv} at XX%

Caveats: digital-only data; biases discussed in [Digital vs. Tabletop](../../statistics/digital-vs-tabletop.md).
```

## Source Notes

```admonish cite title="Sources"
- **Rei's BGG guide**: [link](URL) — canonical community reference for this spirit (if covered)
- **latentoctopus openings**: [opening 1](URL), [opening 2](URL)
- **Cardboard Crew tier**: [link](https://thecardboardcrew.com/spirit-island-spirits/)
- **Stats**: [mindwanderer](https://mindwanderer.net/si/stats.html)
- **Querki FAQ**: [rulings relevant to this spirit](URL)
- **Spirited Discussion podcast**: episode #{N} "{Spirit} deep dive" (if exists)
- **Community discussion**: BGG thread #{id}
```

---

*Last revised: YYYY-MM-DD — meta version X.Y*
