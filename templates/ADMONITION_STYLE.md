# Admonition Style Guide

Only **six** admonition classes in this book. Resist scope creep. Each has a specific semantic role.

## 1. `tip` — Pro Tip

For actionable, stat-backed advice a reader could apply next session.

\```admonish tip title="Pro Tip"
Keep a fire+plant element (e.g., Grasping Roots) in hand on T1 — you unlock level-2 Innate on T2.
\```

## 2. `failure` — Common Mistake

Frequent trap + why it traps you. Be specific; avoid generic "don't do X" without the reasoning.

\```admonish failure title="Common Mistake"
Drafting Major Powers before T3 on Wildfire — Flash-Fires eats your forget capacity and you can't replace it without sacrificing tempo.
\```

## 3. `note` — Stat Insight

Data-backed claim. Always include: source, sample size, date, caveats.

\```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1, N = 2,413 digital games):
- River L6 solo win rate: 62% (95% CI ±4%)
- Best vs. Brandenburg-Prussia; worst vs. Russia L5.
Digital-only data; see [Digital vs. Tabletop](../../statistics/digital-vs-tabletop.md) for bias notes.
\```

## 4. `info` — Rules Clarification

Points from the Querki Living FAQ or official rulings. Never opinion.

\```admonish info title="Rules Clarification"
Per Querki FAQ: Gather moves the target token, even if it's a Sacred Site. Sacred Sites are not Beasts for Shroud's purposes — only for Many Minds.
[Querki source](https://querki.net/raw/darker/spirit-island-faq/Rules-semi-commonly-misplayed)
\```

## 5. `cite` (using `abstract` visually) — Source Note

Block at the end of a section or chapter. Credits community contributors; never copy prose verbatim.

\```admonish abstract title="Sources"
- Rei's BGG guide for this spirit: [link](URL)
- latentoctopus opening: [link](URL)
- Stats: [mindwanderer](https://mindwanderer.net/si/stats.html), as of 2026-04-19
\```

## 6. `example` — Math / Worked Example

For probability calculations, expected-value tables, or a turn-by-turn worked example.

\```admonish example title="Math"
For a sample of n games with observed win rate p̂, a 95% Wilson interval is approximately:

p̂ ± 1.96 × √(p̂(1−p̂)/n)

E.g., 50 games at 60% win rate → 60% ± 13%. Not precise.
\```

---

## Style rules

1. **One admonition per ~300 words of prose, max.** Dense admonition pages become unreadable.
2. **No nested admonitions.** If an idea needs a subcallout, flatten to bullets.
3. **Title every admonition.** The type icon alone isn't enough context.
4. **Don't restate the admonition body in the title.** "Pro Tip: Keep fire+plant in hand" is redundant with a body that says the same.
5. **Never editorialize in a Rules Clarification.** If it's opinion, it's a Pro Tip or Common Mistake instead.
