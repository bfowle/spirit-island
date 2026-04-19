# Serpent Slumbering Beneath the Island

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: all 4 Unique card names were wrong; "Slumber" mechanism mis-described (actual: 5-Presence limit that raises via Absorb Essence — not a "sleep" state but a presence-cap progression).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promo — Feather & Flame                            |
| Complexity            | High                                               |
| Play Difficulty       | 3                                                  |
| Archetypes            | Late-Game Juggernaut · Support via absorbed Presence |
| Primary Elements      | Fire, Water, Earth, Plant, Moon                    |
| Typical Opening       | Slow Absorb-Essence ramp                           |
| Typical Draft Bias    | Mixed                                              |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | `[VERIFY — Locus may exist]`                       |
```

## Spirit Overview — Framing

Serpent is the **late-game presence-absorber**. Starts with a **5-Presence-on-island limit**; uses Absorb Essence to take Presence from partners and expand the cap (5→7→8→10→11→12→13). The absorbed partners become tied to Serpent's plays — Serpent's innates trigger effects across all Spirits with absorbed Presence.

**One-line fantasy**: the land is a body; the body dreams; the dreamer wakes when enough essence is gathered.

**The honest complexity signal**: High per Wiki. Serpent is multiplayer-native — Absorb Essence literally requires absorbing from another Spirit. Solo Serpent is possible but plays differently. Decision depth: when to absorb, how to balance partner's tempo hit vs. Serpent's long-term scaling.

## Core Mechanics & Special Rules

### Special Rule: Deep Slumber

Spirit starts limited to **5 Presence on island**. "Absorb Essence" raises the limit via progression: **5 → 7 → 8 → 10 → 11 → 12 → 13**.

**Strategic implication**: Serpent cannot expand normally. Every Absorb Essence play is a presence-cap raise + presence relocation from another spirit. Multiplayer-essential for full scaling.

### Innate: The Serpent Wakes in Power — Slow, 0 Range, Self

- **L1** (2 Fire, 1 Water, 1 Plant): Gain 1 Energy; other Spirits with absorbed Presence gain 1 Energy.
- **L2** (2 Water, 3 Earth, 2 Plant): Add 1 Presence to 1 Range; others with 2+ absorbed Presence may do likewise.
- **L3** (3 Fire, 3 Water, 3 Earth, 3 Plant): Gain a Major Power without Forgetting; others with 3+ absorbed Presence may do likewise.

### Innate: The Serpent Rouses in Anger — Slow, 0 Range, Any target land

- **L1** (1 Fire, 1 Earth): 1 Damage per Fire/Earth pair to Towns/Cities.
- **L2** (2 Moon, 2 Earth): 2 Fear per Moon/Earth pair; may Push 1 Town from target land.
- **L3** (5 Moon, 6 Fire, 6 Earth): X Damage across all lands (X = Presence in/adjacent to each land).

### Unique Cards

All four starting Uniques per Wiki:

- **Absorb Essence** — 2 Energy, Fast, Self-target, 2 Earth + 3 Water + 3 Fire. Removes another Spirit's Presence; they gain Energy + elements; benefits Wakes in Power innate.
- **Elemental Aegis** — Slow, Self-target, 3 Earth + 3 Water + 3 Fire. Defends multiple lands; strength scales with absorbed Presence.
- **Gift of Flowing Power** — `[VERIFY cost/speed/elements]`. Affects other Spirits; provides Power Card play opportunity.
- **Gift of the Primordial Deeps** — `[VERIFY cost/speed/elements]`. Affects other Spirits; provides elemental/play benefits.

## Key Strategic Principles

1. **Absorb Essence is the engine.** Without Absorb, Serpent can't scale past 5 Presence. Multiplayer only (or self-targeted in solo? `[VERIFY solo rules]`).
2. **Wakes in Power cascades.** Triggers effects on all Spirits with absorbed Presence. Multiplayer compounding.
3. **Rouses in Anger is damage.** Level 3 is X damage across all lands — a board wipe at high-threshold.
4. **Multi-element thresholds.** Fire, Water, Earth, Plant, Moon are all required at various levels. Broad drafting.
5. **Elemental Aegis scales with absorbed Presence.** More absorbed = more defense.
6. **Solo Serpent underperforms.** The spirit is multiplayer-native by design.

## Opening Strategy

### Opening A — Multiplayer Slumber 🟥

**When to pick this**: 2+ player; partner agrees to Absorb Essence coordination.

- **T1**: Slow ramp; low activity. Establish 1–2 presence from starting allocation.
- **T2**: Coordinate with partner — where will Serpent absorb?
- **T3**: First Absorb Essence (presence limit 5→7).
- **T4**: Second Absorb (7→8); Wakes in Power L1 firing.

### Opening B — Solo 🟥 (discouraged)

**When to pick this**: solo practice or partner unavailability.

- Serpent without Absorb Essence is severely limited. Plays as a mid-tier Major-shopper with a 5-Presence hard cap.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Serpent] --> MP{Multiplayer?}
  MP -->|Yes, partner coordinating| A[Opening A - Slumber]
  MP -->|No, solo| B[Opening B - discouraged]
</pre>

## Element & Aspect Preferences

**Preferred elements**: multi (Fire, Water, Earth, Plant, Moon).

**Aspects**: `[VERIFY — Locus may exist]`.

## Card Priority Ratings

Serpent is **Mixed**; absorbed-Presence-dependent.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Absorb Essence | A+ | Core scaling; multiplayer engine. |
| Elemental Aegis | A | Defend + absorbed-Presence scaling. |
| Gift of Flowing Power | A (multiplayer) | Partner support. |
| Gift of the Primordial Deeps | A (multiplayer) | Partner support. |

### Majors

Late-game high-cost Majors fit (Sea Monsters, Fire and Flood, Vigor of the Breaking Dawn).

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | B+ | B | B- | C+ | Fast cities outpace slumber. |
| Brandenburg-Prussia | B+ | B | B- | C+ | Same. |
| Sweden | A | A- | B+ | B | Slow-ish; matches scaling. |
| France | B+ | B | B- | C+ | Complex. |
| Habsburg Mining | A | A- | B+ | B | Best matchup. |
| Russia | A- | B+ | B | B- | Long game. |
| Scotland | B+ | B | B- | C | Faster than ideal. |
| Habsburg Livestock | A | A- | B+ | B | Slow pace fine. |

## Synergy Partners

```admonish tip title="Best Partners"
- **Any Major-shopper partner** — Wakes in Power L3 gives both Serpent and partners with 3+ absorbed Presence a free Major. Compounds.
- **Thunderspeaker** — Gift of Flowing Power + Absorb Essence supports Thunderspeaker's expensive Major turns.
- **Lightning** — Lightning handles early; Serpent closes.
```

```admonish warning title="Anti-Synergy"
- **Another slow spirit** — both scale late; neither handles T1–T3.
- **Solo play** — Serpent is not designed for solo.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Absorbing presence that the partner critically needs. Coordinate before absorbing.
```

```admonish failure title="Common Mistake"
Ignoring Serpent's 5-Presence limit. It's not just a flavor rule; it's a scaling cap.
```

```admonish failure title="Common Mistake"
Drafting single-element Majors. Serpent wants multi-element to hit multiple innate thresholds.
```

## Tempo Profile

Multiplayer-dependent; solo profile is degraded.

## Expansion Sensitivity

- **+ Feather & Flame Promo**: spirit itself is an expansion addition.
- **+ other expansions**: deeper Major pool helps Wakes in Power L3.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~40% (variance-heavy).
- 2-handed with Lightning partner: ~65%.
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- Spirit Island Wiki — Serpent page.
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
