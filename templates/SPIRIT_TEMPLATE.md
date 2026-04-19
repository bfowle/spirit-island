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

Link to latentoctopus openings first. Summarize the variant(s) in your own words. If Rei has an opening pattern, describe it.

### Opening A — {latentoctopus name + confidence}
Source: [latentoctopus](URL)

- **T1**: {growth} + {card plays} + {goals}
- **T2**: ...
- **T3**: ...
- **Presence arc**: 10 → 6 by T5 (or spirit-specific)
- **Target milestones**: 3 CP by T4; unlock {innate threshold} by T5

### Opening B — {variant}
As above.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Turn 1] --> Adv{Adversary?}
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

Grades are directional. Letter grade + one-sentence reasoning per cell.

| Adversary               | L0 | L3 | L5 | L6 | Notes                                        |
|-------------------------|----|----|----|----|----------------------------------------------|
| England                 | A  | A- | B+ | B  | Slow; favors gradual setup.                  |
| Brandenburg-Prussia     | B+ | B  | B- | C+ | Rapid builds punish our slow scaling.        |
| Sweden                  | A  | A  | A- | B+ | Coastal pressure plays to our strengths.     |
| France (Plantation)     |    |    |    |    |                                              |
| Habsburg Mining         |    |    |    |    |                                              |
| Russia                  |    |    |    |    |                                              |
| Scotland                |    |    |    |    |                                              |
| Habsburg Livestock      |    |    |    |    |                                              |

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
