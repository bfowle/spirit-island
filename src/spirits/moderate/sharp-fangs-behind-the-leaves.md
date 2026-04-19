# Sharp Fangs Behind the Leaves

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: "Elusive Ways of the Predator" and "Stir the Trees and Stones" were hallucinated Uniques (actual: **Prey on the Builders**, **Terrifying Chase**); missed 2nd innate **Frenzied Assault**.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base `[VERIFY — may be Branch & Claw; Wiki was unclear]` |
| Complexity            | Moderate                                           |
| Play Difficulty       | 1                                                  |
| Archetypes            | Direct Damage · Beast Scaling · Minor-heavy        |
| Primary Elements      | Plant, Animal, Moon, Fire                          |
| Typical Opening       | Beast-build + Ranging Hunt                         |
| Typical Draft Bias    | Minor-heavy                                        |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | [Fangs concepts + 4 opening variants](https://latentoctopus.github.io/guide/fangs-concepts/) |
| Aspects               | Encircle, Unconstrained                            |
```

## Spirit Overview — Framing

Fangs is the **beast-scaling damage spirit**. Presence converts to Beasts via the Call Forth Predators special rule; Beasts retaliate via Ranging Hunt and concentrate damage via Frenzied Assault.

**One-line fantasy**: the jungle hunts.

## Core Mechanics & Special Rules

### Special Rule: Ally of the Beasts

Your Presence may move with Beasts.

### Special Rule: Call Forth Predators

During each Spirit Phase, you may replace 1 of your Presence with 1 Beasts.

**Strategic implication**: Fangs's engine converts Presence → Beasts each turn. The spirit's "presence count" on the spirit panel depletes turn over turn; Beasts on the board accumulate.

### Innate: Ranging Hunt — Fast, 1 Range, non-Blighted lands

- **2 Animal, 2 Plant**: You may Gather 1 Beasts.
- **2 Plant, 3 Animal**: 1 Damage per Beasts.
- **2 Animal**: You may Push up to 2 Beasts.

### Innate: Frenzied Assault — Slow, 1 Range, lands with Beasts

- **1 Moon, 1 Fire, 4 Animal**: 1 Fear and 2 Damage; Remove 1 Beasts.
- **1 Moon, 2 Fire, 5 Animal**: +1 Fear and +1 Damage.

### Unique Cards

Per Wiki:

- **Prey on the Builders** — 1 Energy, Fast, 0 Range.
- **Teeth Gleam from Darkness** — 1 Energy, `[VERIFY speed]`, targets non-Blighted lands.
- **Terrifying Chase** — 1 Energy.
- **Too Near the Jungle** — 1 Energy, Slow, 0 Range.

## Key Strategic Principles

1. **Convert presence → beasts every turn.** Call Forth Predators is the core.
2. **Plant-Animal dual element is the Ranging Hunt trigger.**
3. **Beast positioning > quantity.** 3 beasts in 1 land > 3 beasts in 3 lands for damage.
4. **Frenzied Assault is the City-killer.** Needs 4+ Animal + Moon + Fire.
5. **Energy-tight.** Cost-1 Uniques + 0-cost Minors preferred.
6. **Non-Blighted land targeting** for Ranging Hunt — adversaries that blight fast reduce Fangs's effectiveness.

## Opening Strategy

Per latentoctopus's 4 openings:

### Opening 1 — Top Track Hybrid 🟨

G2 top + G3 Minor T1–T2; T3 reclaim + Minor. Ranging Hunt active T2+.

### Opening 2 — Full Bottom 🟨

Bottom-track CP focus; 3 CP by T2; energy-tighter.

### Opening 3 — Hybrid with Majors 🟨

Major T3 (forget a Unique); Plant-Animal threshold Majors.

### Opening 4 — Reclaim-Heavy Beast Scaling 🟥

Constant reclaim; energy-starved but Beast-rich. Strong vs. BP and Scotland.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Fangs] --> Adv{Adversary?}
  Adv -->|Default| A[Opening 1 - Top hybrid]
  Adv -->|Build-heavy| B[Opening 2 - Full bottom]
  Adv -->|Health-increase adversaries| C[Opening 3 - Majors]
  Adv -->|BP/Scotland| D[Opening 4 - Beast scaling]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Plant-Animal dual (essential), Moon + Fire (Frenzied Assault).

**Aspects**: Encircle, Unconstrained.

## Card Priority Ratings

Fangs is **Minor-heavy**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Teeth Gleam from Darkness | A+ | Core. |
| Too Near the Jungle | A | Push. |
| Prey on the Builders | A | Build-prevention (Fast). |
| Terrifying Chase | A- | `[VERIFY effect]` |

### Minors

Plant-Animal duals + blight-handling Minors (per latentoctopus).

### Majors (Opening 3 only)

Tigers Hunting, Rouse the Trees and Stones.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Town density favors Ranging Hunt. **L5 cliff**: +1 HP buildings; Ranging Hunt's 1-damage-per-Beasts at 3 Animal threshold still effective if 3+ beasts in target. |
| Brandenburg-Prussia | A+ | A | A- | B+ | Opening 4 specifically shines. |
| Sweden | A- | B+ | B | B- | Fear penalties. |
| France | A- | B+ | B | B- | Blight. |
| Habsburg Mining | B+ | B | B- | C+ | Scaling. |
| Russia | B+ | B | B- | C+ | Settlers tough. |
| Scotland | A+ | A | A- | B+ | Opening 4 strong. |
| Habsburg Livestock | A- | B+ | B | B | Decent. |

## Synergy Partners

```admonish tip title="Best Partners"
- **Thunderspeaker** — dahan + beasts share battlefield.
- **River** — push + beasts kill.
- **Earth** — Earth defends; Fangs damages.
- **Bringer** — Bringer fear-rushes; Fangs kills for fear.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Not converting presence to beasts. Presence on panel is inert.
```

```admonish failure title="Common Mistake"
Spreading beasts thinly. Concentrate for Frenzied Assault scaling.
```

```admonish failure title="Common Mistake"
Drafting 1-cost Minors on Opening 2.
```

## Tempo Profile

Per openings above.

## Major vs. Minor

**Draft bias**: Minor-heavy. Opening 3 deviates.

## Expansion Sensitivity

- **Base/B&C minimum**.
- **+ Jagged Earth / later**: deeper Animal-Plant Major pool.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~52%.
- Best vs. Scotland L6 (~65% — Opening 4).
- Worst vs. Habsburg Mining L6 (~40%).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- [latentoctopus Fangs concepts + 4 openings](https://latentoctopus.github.io/guide/fangs-concepts/).
- Cross-reference: [Dahan Rush archetype](../../combos/dahan-rush.md) (Beast proxy-dahan section).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
