# Lightning's Swift Strike

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low                                                |
| Play Difficulty       | 0                                                  |
| Archetypes            | Direct Damage (primary) · Fast-Energy              |
| Primary Elements      | Air, Fire                                          |
| Typical Opening       | Full top, energy-heavy                             |
| Typical Draft Bias    | Minor-heavy                                        |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | Wind, Pandemonium, Immense, Sparking               |
```

## Spirit Overview — Framing

Lightning is the **fast, direct-damage spirit**. Fastest to start, fastest to scale, fastest to close. Every turn is simple: play a fast-damage card, kill an invader, feel accomplished. Lightning is the game's easiest spirit and arguably its most satisfying for new players.

**One-line fantasy**: the sky cracks open; an Explorer dies instantly; the island's fury turns electric.

**The honest complexity signal**: Low is correct. Lightning is the game's teach-spirit default along with River. The depth is in multiplayer — Lightning's mobility + low-cost cards let them clean up partner-spirits' leftover invaders, making the combined board control stronger than either solo.

## Core Mechanics & Special Rules

### Special Rule: Fast Cards

Lightning's Uniques are all Fast-timed; they play *before* the Invader Phase. Lightning can kill an Explorer before it becomes a Town.

### Innate: Thundering Destruction

Air/Fire-scaling damage. Modest at Level 1; strong at Level 2; game-closing at Level 3.

### Shatter Homesteads (Unique)

Fast damage. Core kill card.

### Lightning's Boon (Unique)

Energy-gain unique; fuels the Fast-energy curve.

### Delusions of Danger (Unique)

Fear + damage combo.

### Raging Storm (Unique)

High-cost, high-damage closer.

## Key Strategic Principles

1. **Kill early, kill often.** Every Explorer killed T1–T2 is a Town not built T3.
2. **Energy first.** Lightning's curve demands 2E by T3 to afford multi-card Fast turns. G1/G2 top-track energy is standard.
3. **Air is primary.** Innate thresholds want Air heavy; Fire secondary. Draft Air-bearing Minors.
4. **Mobility matters.** Lightning can target Range-2 lands early; don't restrict to Range-0 placement.
5. **Don't over-defend.** Lightning has no real defend; let the adversary ravage occasionally; compensate with kills.
6. **Draft 0-cost Minors aggressively.** Hand rotation benefits from cheap Minors stacking onto Fast-phase turns.
7. **Majors are the finisher, not the opener.** T5+.

```admonish tip title="Pro Tip"
"Kill the Explorer before it builds" is the highest-leverage play in Spirit Island. Lightning is the spirit best-positioned to do this; never hold Shatter Homesteads for "a better target."
```

## Opening Strategy

### Opening A — Full top, energy-focused 🟨

**When to pick this**: default for Lightning.

**Target arc**: 2E by T3 · Level 1 innate by T3 · Level 2 innate T5 · Terror 2 flip by T6 via kill-fear.

#### Turn 1

- **Growth**: **G2 top** (energy + card play).
- **Cards played**: **Shatter Homesteads + Lightning's Boon** — kill an Explorer; gain energy for T2.
- **Presence placement**: adjacent to first Ravage target.
- **Elements by end**: 1 Air, 1 Fire.
- **E / CP state**: 1E / 2CP → 0E / 2CP (Boon generates energy forward).
- **Milestone**: 1 Explorer killed; 1 fear.

#### Turn 2

- **Growth**: **G2 top** (second energy).
- **Cards played**: **Shatter Homesteads (reclaimed if offered) + Delusions of Danger + 0-cost Minor if drafted**.
- **Presence placement**: second land; mobility focus.
- **Elements by end**: 2 Air, 2 Fire.
- **E / CP state**: 2E / 2CP → 0–1E / 2CP.
- **Milestone**: 2 kills total; Level 1 innate threshold; fear at 3/4.

#### Turn 3

- **Growth**: **G1 Reclaim + G3** (Minor gain + refresh).
- **Cards played**: **2 Uniques + Minor** — 3 cards total.
- **Presence placement**: third land.
- **Elements by end**: 3 Air, 2 Fire.
- **E / CP state**: 2E / 2CP → 0E / 2CP.
- **Milestone**: Level 1 innate firing reliably; 3 kills total.

#### Turn 4 — state audit

After T3:
- **Presence**: 6 on board; spread for mobility.
- **Energy / CP**: 2E / 3CP.
- **Engine state**: Level 1 innate firing; 3 cards/turn.
- **Fear pool**: 4–5 of 8.
- **Blight**: 1–2 (acceptable — Lightning lets some ravages land).

Pivot advice:
- **Energy starved** → skip Minor draft; take G1 (reclaim) instead.
- **Missing Air** → draft Air-bearing Minor immediately.
- **Adversary scaling faster than kills** → Major T4 to spike damage.

### Opening B — Major-early hybrid 🟨

**When to pick this**: adversaries where Uniques aren't enough (Brandenburg-Prussia L5+, Habsburg Mining late).

- **T1**: G2 top + Shatter + Lightning's Boon.
- **T2**: G4 (Major gain, forget Raging Storm for Fiery Power). Play Shatter + new Major.
- **T3**: Reclaim; Major fires.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Lightning] --> Adv{Adversary?}
  Adv -->|Any base/B&C| A[Opening A]
  Adv -->|Brandenburg-Prussia L5+| B[Opening B - Major early]
  Adv -->|Habsburg Mining| B
  Adv -->|Teaching a new player| A
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Air (primary), Fire (secondary).

**Aspects**:

- **Wind** — mobility emphasis.
- **Pandemonium** — strife + fear hybrid.
- **Immense** — single-target annihilation.
- **Sparking** — chain-damage multi-target.

## Card Priority Ratings

Lightning is **Minor-heavy**.

### Uniques
| Card | Grade | Notes |
|---|---|---|
| Shatter Homesteads | A+ | Core kill |
| Lightning's Boon | A | Energy sustain |
| Delusions of Danger | A- | Fear + damage |
| Raging Storm | B+ | Cost-heavy; forget T5 |

### Minors to Target
| Card | Grade | Why |
|---|---|---|
| 0-cost Air Minors | A+ | Threshold + free plays |
| Strengthen the Gifts | A | Threshold |
| Dissolve Flash of Lightning | A- | Direct damage + Air |

### Majors that Over-perform
| Card | Grade | Why |
|---|---|---|
| Fiery Power | A+ | Air-Fire; massive damage |
| Tigers Hunting | A | Multi-target kill |
| Paralyzing Fright | A- | 3-cost closer |

### Avoid drafting
- Defend-heavy cards (Lightning doesn't defend).
- Slow-only cards that duplicate Uniques.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Town-dense; good kill-fear |
| Brandenburg-Prussia | A+ | A | A- | B+ | Explorer kill prevents cascading builds |
| Sweden | A | A- | B+ | B | Fear-agnostic; kills still work |
| France | A- | B+ | B | B- | Dahan safe under kills |
| Habsburg Mining | B+ | B | B- | C+ | Scaling adversary; runs out of gas |
| Russia | B+ | B | B | B- | Settlers killable |
| Scotland | A | A- | B+ | B | Favorable |
| Habsburg Livestock | A- | B+ | B | B | Decent |

## Board Position Evaluation

- **Favorable**: Any board (terrain-agnostic).
- **Neutral**: All boards.
- **Unfavorable**: None.

## Game-Phase Strategy

- **Early (T1–3)**: Shatter + Boon; kill 1–2 invaders/turn; place presence for mobility.
- **Mid (T4–6)**: Level 1 innate; 3 cards/turn; consider Major T4–5.
- **Late (T7+)**: Major + Shatter combos; Terror 2 flipped.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Vital Strength of the Earth** — Earth defends; Lightning kills.
- **Thunderspeaker** — Lightning kills Explorers; dahan handles Towns.
- **Shadows** — kill-fear amplifies Shadows's innate.
- **Green** — Green defends + Majors; Lightning provides early tempo.
```

```admonish warning title="Anti-Synergy"
- Other pure-damage spirits (Fangs, Wildfire) — redundant.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Holding Shatter Homesteads for "a better target." Shatter is for Explorers and early Towns; playing it late when Cities dominate is under-using the card.
```

```admonish failure title="Common Mistake"
Over-drafting Slow cards. Lightning's identity is Fast-phase kills.
```

```admonish failure title="Common Mistake"
Trying to defend with Lightning. Lacks defend Uniques.
```

```admonish failure title="Common Mistake"
Picking Lightning for an L5+ Habsburg Mining game. The scaling outpaces Lightning's ceiling.
```

## Tempo Profile

| Round | Energy | CP | Presence | Kills Total | Key Play |
|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 1 | Shatter + Boon |
| 2 | 1–2E | 2 | 5 | 2–3 | Shatter + Delusions |
| 3 | 2E | 2 | 6 | 3–4 | Reclaim + 2 cards |
| 4 | 2E | 3 | 6 | 5–6 | Level 1 innate; 3 cards |
| 5 | 2–3E | 3 | 6 | 7+ | Gain Major |
| 6 | 3E | 3 | 6 | 9+ | Major + Shatter |
| 7 | 3–4E | 3 | 6 | 11+ | Terror 2 flip |
| 8 | 4E | 3 | 5 | — | Terror 3 close |

Cliff turn: **T3**. Must reclaim or energy-saturate.

## Major vs. Minor — Lightning Specifically

**Draft bias**: Minor-heavy.

- T1–T3: 0-cost + 1-cost Minors.
- T4–T5: Major consideration; Fiery Power is the standout.
- T6+: more Minors or second Major.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1 digital estimates):
- Solo L6 win rate: ~58%.
- Best vs. Brandenburg-Prussia L6 at ~68%.
- Worst vs. Habsburg Mining L6 at ~42%.
- Popular teach-spirit; data skews newer players.
```

## Source Notes

```admonish abstract title="Sources"
- Community BGG Lightning strategy threads
- [Cardboard Crew tier](https://thecardboardcrew.com/spirit-island-spirits/)
- [Spirit Island Wiki — Lightning](https://spiritislandwiki.com/)
- Cross-reference: [Teaching Methods](../../social/teaching-methods.md)
```

---

*Last revised: 2026-04-19*
