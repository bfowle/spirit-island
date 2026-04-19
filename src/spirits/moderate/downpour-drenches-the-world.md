# Downpour Drenches the World

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: "Pour Down" labeled as Unique card (it's an innate); 3 of 4 Unique names were wrong; missed the Drench the Landscape special rule and the 3-innate structure.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promo Pack 2 `[VERIFY expansion]`                  |
| Complexity            | High                                               |
| Play Difficulty       | 2                                                  |
| Archetypes            | Defend · Blight removal · Energy sustain           |
| Primary Elements      | Water (primary, heavy), Air, Earth, Plant          |
| Typical Opening       | Water-element accumulation; defend-build           |
| Typical Draft Bias    | Minor-heavy (complementarity drafting)             |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | [Downpour concepts + opening 1](https://latentoctopus.github.io/guide/downpour-concepts/) |
| Aspects               | `[VERIFY]`                                         |
```

## Spirit Overview — Framing

Downpour is the **triple-innate Water spirit**. Three innates (Pour Down, Rain and Mud Suppress Conflict, Water Nourishes Life's Growth) plus a special rule (Drench the Landscape) mean the spirit has lots of simultaneous scaling — Water thresholds unlock multiple effects per turn.

**One-line fantasy**: rain so constant it drowns the invaders' plans before they act. Every land becomes wetland; every Ravage becomes muted.

**The honest complexity signal**: High per Wiki. Three innates to track; multi-effect scaling based on Water element count. Players learning Downpour often under-leverage the simultaneous firing.

## Core Mechanics & Special Rules

### Special Rule: Drench the Landscape

Spirit Actions and Special Rules treat your Sacred Site as Wetlands in addition to the printed terrain.

**Strategic implication**: Sacred Site lands count as Wetlands for targeting purposes. This matters for terrain-gated cards — "target Wetland" Minors/Majors can always hit Downpour's Sacred Sites.

### Innate: Pour Down Power Across the Island

For each 2 Water you have: gain 1 Energy OR repeat a land-targeting Power Card by paying its cost again (max 5 times per turn).

**Strategic implication**: Water scales into energy OR power-card repetition. A card-repeat innate is extremely powerful — effectively doubles or triples the output of a single card in one turn.

### Innate: Rain and Mud Suppress Conflict

- **L1** (1 Air, 3 Water): Each Presence grants Defend 1; lowers Dahan counterattack damage by 1.
- **L2** (5 Water, 1 Earth): Each Presence grants Defend 1; lowers Dahan counterattack damage by 1.
- **L3** (3 Air, 9 Water, 2 Earth): 2 Fear; Invaders and Dahan have -1 Health (min 1) in your lands.

### Innate: Water Nourishes Life's Growth

- **L1** (3 Water, 2 Plant): Gain 1 Energy; remove 1 Blight by removing one Presence.
- **L2** (5 Water, 1 Earth, 2 Plant): Gain 1 Energy; gather up to 1 Dahan.
- **L3** (7 Water, 2 Earth, 3 Plant): Prevent Blight addition to target land.

### Unique Cards

Per Wiki; costs/speeds/elements `[VERIFY]`:

- **Dark Skies Loose a Stinging Rain**
- **Foundations Sink into Mud**
- **Gift of Abundance**
- **Unbearable Deluge**

## Key Strategic Principles

1. **Water threshold-stacking is the engine.** Pour Down at 2 Water = 1 Energy OR repeat; at 4 Water = 2 Energy OR 2 repeats. Water-heavy drafts compound.
2. **Three innates fire in parallel.** A single turn can trigger Pour Down + Rain and Mud + Water Nourishes if Water threshold is high enough.
3. **Card repetition via Pour Down is the killer app.** A 1-cost Minor that repeats 3 times is effectively 3 plays for 3 energy.
4. **Defend engine scales with Presence count.** Rain and Mud L1+ gives Defend 1 per Presence — a 7-presence spirit defends 7 per turn via innate alone.
5. **Level 3 Water Nourishes prevents blight.** Late-game insurance.
6. **Plant is secondary.** Water Nourishes requires Plant for scaling.
7. **Minor-heavy draft.** Cost-1 repeatable Minors are Downpour's ideal draft.

## Opening Strategy

### Opening A — Water accumulation 🟥

**When to pick this**: default.

**Target arc**: 4 Water by T3 · Pour Down repeating by T3 · Level 1 of all three innates firing T4.

#### Turn 1

- **Growth**: **G3 bottom** (Minor gain + presence).
- **Cards played**: Unique + 0-cost Water Minor.
- **Presence placement**: coastal/wetland.
- **Elements by end**: 2 Water.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Pour Down L1 firing (2 Water = 1 Energy or 1 repeat).

#### Turn 2

- **Growth**: **G2 bottom**.
- **Cards played**: Unique + Minor.
- **Presence placement**: adjacent to T1.
- **Elements by end**: 3 Water, 1 Plant.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Water Nourishes L1 threshold (3 Water + 2 Plant).

#### Turn 3

- **Growth**: **G1 Reclaim + G3 Minor**.
- **Cards played**: 3 cards.
- **Elements by end**: 4 Water, 1 Air, 2 Plant.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Pour Down 2-repeat capability; Rain and Mud L1 threshold (1 Air + 3 Water).

#### Turn 4 — state audit

- **Presence**: 6 of 13.
- **Energy / CP**: 1–2E / 2–3CP.
- **Engine**: Pour Down producing 2 energy or 2 card-repeats; Rain and Mud firing; Water Nourishes firing.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Downpour] --> Adv{Adversary?}
  Adv -->|Default| A[Opening A - Water accumulation]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Water (heavy), Plant, Air, Earth.

**Aspects**: `[VERIFY]`.

## Card Priority Ratings

Downpour is **Minor-heavy**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Dark Skies Loose a Stinging Rain | `[VERIFY]` | Wiki didn't surface effect; check copy. |
| Foundations Sink into Mud | `[VERIFY]` | Same. |
| Gift of Abundance | `[VERIFY]` | Scaling card; per latentoctopus concepts "recommended for scaling improvement." |
| Unbearable Deluge | `[VERIFY]` | Same. |

### Minors (complementarity-drafting per latentoctopus)

| Card | Grade | Why |
|---|---|---|
| Water-bearing 1-cost Minors | A+ | Pour Down repeatable fuel. |
| 0-cost Water Minors | A+ | Threshold + free. |
| Explorer-removal Minors | A+ | Fills gap per latentoctopus. |
| Defend-without-suppression Minors | A | Fills gap. |

### Majors

Rare for Downpour (Minor-heavy archetype). Water-heavy Majors if any.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Slow build matches. |
| Brandenburg-Prussia | A- | B+ | B | B- | Fast cities. |
| Sweden | A | A- | B+ | B | Fear-agnostic. |
| France | A+ | A | A- | B+ | Dahan-capture; Downpour's Dahan-protect helps. |
| Habsburg Mining | A- | B+ | B | B- | Scaling. |
| Russia | A | A- | B+ | B | Explorer-removal Minor essential. |
| Scotland | A- | B+ | B | B- | Decent. |
| Habsburg Livestock | A | A- | B+ | B | Terrain focus. |

## Synergy Partners

```admonish tip title="Best Partners"
- **Thunderspeaker** — Downpour defends; Thunderspeaker's dahan-fear closes.
- **Earth** — Downpour buys time; Earth's Majors close.
- **Shadows** — denial + fear-rush pairing.
- **Fangs** — Downpour defends; Fangs damages.
```

```admonish warning title="Anti-Synergy"
- **Vengeance** — blight-positive vs. Downpour's blight-removal.
- **Wildfire** — same blight-conflict.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Drafting Minors that duplicate Downpour's existing strengths. Draft for gaps (Explorer-removal, Defend-without-suppression) per latentoctopus complementarity.
```

```admonish failure title="Common Mistake"
Not chaining Pour Down repeats. A 1-cost Minor + 4 Water = 2 repeats (3 total plays). Under-using this wastes Downpour's core mechanic.
```

```admonish failure title="Common Mistake"
Chasing Majors. Downpour's engine is Minor-heavy; Majors over-extend.
```

## Tempo Profile

| Round | Energy | CP | Presence | Water | Key Play |
|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 2 | Unique + Minor |
| 2 | 1E | 2 | 5 | 3 | Unique + Minor |
| 3 | 1–2E | 2 | 6 | 4 | Reclaim + 3 cards |
| 4 | 2E | 3 | 7 | 4–5 | All 3 innates active |
| 5 | 2E | 3 | 7 | 5 | Sustained |
| 6 | 2–3E | 3 | 7 | 5–6 | Terror 2 approaches |
| 7 | 3E | 3 | 6 | 6+ | Partner closes. |
| 8 | 3E | 3 | 6 | 7+ | Level 3 innate possible. |

## Expansion Sensitivity

- **Promo Pack 2 minimum** (Downpour from this pack).
- **+ Jagged Earth / Nature Incarnate**: deeper Minor pool; complementarity-drafting has more gap-fillers.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~48%.
- Best vs. France L6 (~65%).
- Small sample (promo spirit).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- [latentoctopus — Downpour concepts](https://latentoctopus.github.io/guide/downpour-concepts/).
- [latentoctopus — Downpour Opening 1](https://latentoctopus.github.io/guide/downpour-opening1/).
- Spirit Island Wiki — Downpour page.
- Cross-reference: [Energy Denial archetype](../../combos/energy-denial.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
