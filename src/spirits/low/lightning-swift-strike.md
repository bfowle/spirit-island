# Lightning's Swift Strike

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: missed the "Swiftness of Lightning" special rule (makes Slow Powers Fast via Air); innate threshold details corrected; "Delusions of Danger" Unique was likely hallucinated (not in the Wiki-surfaced 4-card list).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low                                                |
| Play Difficulty       | 0                                                  |
| Archetypes            | Direct Damage · Fast-Phase Speed                   |
| Primary Elements      | Air, Fire, Water (Level 3 of innate)               |
| Typical Opening       | Fast + Air-accumulation                            |
| Typical Draft Bias    | Minor-heavy                                        |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | Wind, Pandemonium, Immense, Sparking `[VERIFY]`    |
```

## Spirit Overview — Framing

Lightning is the **fast-phase damage spirit**. "Swiftness of Lightning" converts Slow Powers to Fast Powers based on Air element — Lightning's effective damage window is *before* the Invader Phase, preventing builds and ravages before they happen.

**One-line fantasy**: the sky cracks open; an Explorer dies before it becomes a Town.

## Core Mechanics & Special Rules

### Special Rule: Swiftness of Lightning

For every Air element you have, you may use **1 Slow Power as if it were Fast**.

**Strategic implication**: this is the core Lightning trick. With 3 Air, Lightning can play 3 Slow cards in the Fast phase — wiping the board pre-ravage. Drafting Air is essential not just for innate threshold but for timing conversion.

### Innate: Thundering Destruction — Slow, 1 Range from Sacred Site, Any land

- **3 Fire, 2 Air**: Destroy 1 Town.
- **4 Fire, 3 Air**: Destroy 1 City instead.
- **5 Fire, 4 Air, 1 Water**: Also destroy 1 Town/City.
- **5 Fire, 5 Air, 2 Water**: Also destroy 1 Town/City.

**Strategic implication**: multi-Town/City destruction at Level 3+. With Swiftness of Lightning, this Slow-phase innate can fire in Fast phase if Air is high enough.

### Unique Cards

Per Wiki:

- **Harbingers of the Lightning** — `[VERIFY cost/speed/elements]`.
- **Lightning's Boon** — `[VERIFY cost/speed/elements]`.
- **Raging Storm** — `[VERIFY cost/speed/elements]`.
- **Shatter Homesteads** — `[VERIFY cost/speed/elements]`.

## Key Strategic Principles

1. **Swiftness of Lightning = timing conversion.** Every Air element = 1 Slow-to-Fast conversion. Air is the single most important element for Lightning.
2. **Fire + Air is the damage engine.** Thundering Destruction requires both heavily; drafts prioritize both.
3. **Destroy Explorers pre-Build.** Lightning's Fast-phase kills prevent the Town-upgrade chain.
4. **Sacred Site for Thundering Destruction's 1 Range.** 2 Presence in one land unlocks Sacred Site targeting.
5. **Majors are optional finishers.** Lightning's Uniques + innate are usually sufficient; a Major T5+ helps close.

## Opening Strategy

### Opening A — Air accumulation 🟥

#### Turn 1

- **Growth**: G2 top (energy + CP) or G3 (Minor gain).
- **Cards played**: Shatter Homesteads + Lightning's Boon.
- **Presence placement**: adjacent to first invader-dense land.
- **Elements by end**: 1 Air, 1 Fire.

#### Turn 2

- **Growth**: G2 top.
- **Cards played**: reclaim + Minor.
- **Elements by end**: 2 Air, 2 Fire.

#### Turn 3

- **Growth**: G1 Reclaim + G3 Minor.
- **Cards played**: 3 cards.
- **Elements by end**: 3 Air, 2 Fire (Thundering Destruction Level 1 threshold).

#### Turn 4 — state audit

- Presence 6, Swiftness of Lightning converting 3 Slows to Fast, Thundering Destruction firing.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Lightning] --> Adv{Adversary?}
  Adv -->|Any base/B&C| A[Opening A]
  Adv -->|Brandenburg-Prussia L5+| A2[Opening A + Major T4]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Air (primary; Swiftness of Lightning), Fire (secondary), Water (Level 3 innate).

**Aspects**: Wind, Pandemonium, Immense, Sparking `[VERIFY]`.

## Card Priority Ratings

Lightning is **Minor-heavy**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Shatter Homesteads | A+ | Core kill. `[VERIFY speed]` |
| Lightning's Boon | A | `[VERIFY effect]` |
| Raging Storm | A- | `[VERIFY]` |
| Harbingers of the Lightning | `[VERIFY]` | Wiki didn't surface effect. |

### Majors

Fiery Power, Tigers Hunting (Air-Fire threshold preferred).

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Town kills prevent Stage III. **L5 cliff**: +1 HP buildings; Thundering Destruction Level 1 still destroys Towns (effect-based, not damage-based). |
| Brandenburg-Prussia | A+ | A | A- | B+ | Fast-phase Explorer kills prevent City-upgrades. |
| Sweden | A | A- | B+ | B | Kills still work; fear bonus. |
| France | A- | B+ | B | B- | Dahan-safe. |
| Habsburg Mining | B+ | B | B- | C+ | Scaling runs out. |
| Russia | B+ | B | B | B- | Settlers killable. |
| Scotland | A | A- | B+ | B | Favorable. |
| Habsburg Livestock | A- | B+ | B | B | Decent. |

## Synergy Partners

```admonish tip title="Best Partners"
- **Earth** — Earth defends + repeats Lightning's Majors via Gift of Strength.
- **Thunderspeaker** — Lightning kills Explorers; Thunderspeaker's dahan handle Towns.
- **Shadows** — kill-fear compounds.
- **Green** — Green defends; Lightning damages.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Holding Shatter Homesteads for "a better target." Shatter is for Explorers and early Towns.
```

```admonish failure title="Common Mistake"
Ignoring Air element. Swiftness of Lightning + Thundering Destruction both need Air heavily.
```

```admonish failure title="Common Mistake"
Trying to defend. Lightning has no defend.
```

## Tempo Profile

Typical arc; see Opening Strategy above.

## Major vs. Minor

**Draft bias**: Minor-heavy.

## Expansion Sensitivity

- **Base only**: fully functional.
- **+ expansions**: deeper Air-Fire Major pool.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~58%.
- Best vs. Brandenburg-Prussia L6 (~68%).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- Cross-reference: [Teaching Methods](../../social/teaching-methods.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
