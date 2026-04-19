# River Surges in Sunlight

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: "Flood the Hills" was hallucinated (actual: **Flash Floods**); "Flow Through the Powers" not in Wiki's 4 Uniques (actual 4th: **Boon of Vigor**); missed the **River's Domain** special rule (Wetlands as Sacred Sites).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low                                                |
| Play Difficulty       | 0                                                  |
| Archetypes            | Terrain Control · Push · Fear generation           |
| Primary Elements      | Sun, Water, Earth                                  |
| Typical Opening       | Bottom-track Bounty loop                           |
| Typical Draft Bias    | Mixed                                              |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | [River opening 1](https://latentoctopus.github.io/guide/river-opening1/) |
| Aspects               | Sunshine, Travel, Haven `[VERIFY]`                 |
```

## Spirit Overview — Framing

River is the **flexible push-gather spirit** — the most forgiving in the game. Wetlands automatically count as Sacred Sites. Massive Flooding pushes or damages invaders based on threshold.

**One-line fantasy**: the river rises, sweeps invaders out of their build targets, feeds the dahan.

## Core Mechanics & Special Rules

### Special Rule: River's Domain

Presence in Wetlands automatically counts as Sacred Sites.

**Strategic implication**: Wetland presence is double-duty — 1 presence placement gets Sacred Site effects free. Wetland-heavy boards are River's favorite.

### Innate: Massive Flooding — Slow, 1 Range from Sacred Site, Any land

- **L1** (1 Sun, 2 Water): Push 1 Explorer/Town from target land.
- **L2** (2 Sun, 3 Water): Deal 2 Damage instead; push up to 3 Explorers/Towns.
- **L3** (3 Sun, 4 Water, 1 Earth): Deal 2 Damage to each Invader in target land.

### Unique Cards

Per Wiki:

- **Boon of Vigor** — `[VERIFY cost/speed/elements]`. Cooperative support card providing vigor to allies.
- **Flash Floods** — Fast speed, `[VERIFY cost/elements]`. Effective against Explorers and Towns.
- **River's Bounty** — `[VERIFY cost/speed/elements]`. Provides blessings; generates additional Dahan population.
- **Wash Away** — `[VERIFY cost/speed/elements]`. Displaces Explorers and Towns through flooding effects.

## Key Strategic Principles

1. **Wetlands = free Sacred Sites.** Prioritize wetland presence for Massive Flooding range.
2. **Water is load-bearing.** Level 2+ Massive Flooding needs 3+ Water.
3. **Push > kill for early game.** Flash Floods + Wash Away push invaders out of builds.
4. **Reclaim cycles matter.** 2-turn cycles with Bounty + refreshing Minors.
5. **Boon of Vigor is multiplayer support.** Solo use more limited.
6. **River's Bounty scales dahan** `[VERIFY exact effect]`.

## Opening Strategy

### Opening A — Bottom track, Bounty-focus 🟩

**Source**: [latentoctopus Opening 1](https://latentoctopus.github.io/guide/river-opening1/).

#### Turn 1

- **Growth**: G3 bottom.
- **Cards played**: River's Bounty + Wash Away (or Flash Floods).
- **Presence placement**: wetland if possible.
- **Elements by end**: 2 Water, 1 Sun.

#### Turn 2

- **Growth**: G2 bottom.
- **Cards played**: Flash Floods + Minor.
- **Elements by end**: 2 Water, 2 Sun.

#### Turn 3

- **Growth**: G1 Reclaim + G3 bottom.
- **Cards played**: River's Bounty + Minor + Unique — 3 cards.
- **Elements by end**: 3 Water, 2 Sun.

#### Turn 4 — state audit

- Presence 6–7 with wetland density; Massive Flooding Level 2 firing.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — River] --> Adv{Adversary?}
  Adv -->|Default| A[Opening A]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Sun, Water (primary), Earth (Level 3).

**Aspects**: Sunshine, Travel, Haven `[VERIFY]`.

## Card Priority Ratings

River is **Mixed**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| River's Bounty | A+ | Core engine. |
| Flash Floods | A | Fast push + damage. |
| Wash Away | A | Push + damage. |
| Boon of Vigor | A (multiplayer) / B+ (solo) | Ally support. |

### Majors

Vigor of the Breaking Dawn, Sea Monsters, Fiery Power — Water/Sun alignment.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Coastal push prevents builds. **L5 cliff**: +1 HP buildings; Level 1 Massive Flooding's push still works (push, not damage). Level 2+ damage becomes insufficient without high Water threshold. |
| Brandenburg-Prussia | A | B+ | B | B- | Fast cities outpace. |
| Sweden | A- | B+ | B | B | Fear-agnostic. |
| France | B+ | B | B- | C+ | Dahan capture. |
| Habsburg Mining | A- | B+ | B | B- | Level 2 keeps up. |
| Russia | B+ | B | B | B- | Push vs settlers. |
| Scotland | A | A- | B+ | B | Favorable. |
| Habsburg Livestock | A- | B+ | B | B | Decent. |

### Strategy Cliff — England L5

```admonish info title="Strategy Cliff — England L5"
**What changes**: buildings gain +1 HP.

**Impact on River**: Massive Flooding Level 2 deals 2 damage, no longer killing Towns alone (now 3 HP). Level 3 scales better. Push-only strategy (Level 1) remains fully functional — pushed invaders don't build.

**Mitigation**: bias toward Level 2+ Water threshold; consider Vigor of the Breaking Dawn or Tigers Hunting Major for direct Town/City kills.
```

## Synergy Partners

```admonish tip title="Best Partners"
- **Thunderspeaker** — River pushes; dahan retaliate.
- **Earth** — Earth inland defense; River wetland/coastal.
- **Sharp Fangs** — Fangs beasts + River push.
- **Bringer** — dahan preservation for Bringer's scaling.
```

```admonish warning title="Anti-Synergy"
- **Ocean** — both coastal; territorial conflict.
- **Wildfire** — blight-conflict.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Not reclaiming on T3. River's engine relies on 2-turn reclaim cycles.
```

```admonish failure title="Common Mistake"
Drafting non-Water Minors. Water is threshold-essential.
```

```admonish failure title="Common Mistake"
Ignoring Wetland = Sacred Site. Wetland presence is high-value.
```

## Tempo Profile

Per Opening A.

## Major vs. Minor

**Draft bias**: Mixed.

## Expansion Sensitivity

- **Base only**: fully functional.
- **+ later expansions**: deeper Water-Sun Major pool.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~54%.
- Best vs. England L6 (~65%).
- High sample size.
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- [latentoctopus River Opening 1](https://latentoctopus.github.io/guide/river-opening1/).
- Cross-reference: [Terrain Control archetype](../../combos/terrain-control.md), [Teaching Methods](../../social/teaching-methods.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
