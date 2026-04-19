# Ocean's Hungry Grasp

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: all 4 Unique names were wrong (Consuming Depths, Tsunami, Gift of Power, Call of the Sea — none of these are Ocean's actual Uniques per Wiki). Missed the 2nd innate (Ocean Breaks the Shore). The special rule description was partially correct but missing the Energy-exchange mechanic.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | High                                               |
| Play Difficulty       | 1–2                                                |
| Archetypes            | Coastal Drowning · Fear generation                 |
| Primary Elements      | Moon, Air, Water, Earth                            |
| Typical Opening       | Ocean-presence + coastal drowning                  |
| Typical Draft Bias    | Mixed                                              |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | `[VERIFY]`                                         |
```

## Spirit Overview — Framing

Ocean is the **ocean-dwelling drowning spirit**. Presence lives in Oceans (not Inland lands). Invaders/Dahan moved to oceans are drowned — removed from play, and Ocean exchanges drowned invader health for Energy.

**One-line fantasy**: the sea rises, drags invaders into the depths, leaves the coasts empty.

## Core Mechanics & Special Rules

### Special Rule: Ocean Presence + Drowning

You may add/move Presence into Oceans, but may not add/move Presence into Inland lands. Invaders or Dahan moved to oceans are drowned; **spirit exchanges drowned invader health for energy (X health = 1 energy, where X equals player count)**.

**Strategic implication**: Ocean is **coastal-dependent by design**. The energy economy is tied to drowning invaders — Ocean gains Energy per drowned invader's HP, scaled by player count. Solo, a drowned City = 3 Energy (3 HP ÷ 1 player); 4-player, a drowned City = 0.75 Energy rounded.

### Innate: Pound Ships to Splinters — Fast, 0 Range, Coastal

- **L1** (1 Moon, 1 Air, 2 Water): 1 Fear.
- **L2** (2 Moon, 1 Air, 3 Water): +1 Fear (total 2).
- **L3** (3 Moon, 2 Air, 4 Water): +2 Fear (total 3).

### Innate: Ocean Breaks the Shore — Slow, 0 Range, Coastal

- **L1** (2 Water, 1 Earth): Drown 1 Town.
- **L2** (3 Water, 2 Earth): May Drown 1 City instead.
- **L3** (4 Water, 3 Earth): Also Drown 1 Town/City.

### Unique Cards

Per Wiki:

- **Call of the Deeps** — 0 Energy, Fast, 0 Range Coastal (requires Sacred Site). Moves Explorers; enables consumption via drowning.
- **Grasping Tide** — `[VERIFY cost/speed/elements]`. Defensive ability protecting against Invader damage on shores.
- **Swallow the Land-Dwellers** — `[VERIFY cost/speed/elements]`. Causes drowning effects; harms Dahan.
- **Tidal Boon** — `[VERIFY cost/speed/elements]`. Allows other spirits to move Invaders/Dahan; provides safe passage for allied Dahan.

## Key Strategic Principles

1. **Ocean presence only goes in ocean.** Don't try to place inland.
2. **Drowning = Energy.** The energy economy is drown-dependent. Target Cities for best energy exchange in solo; balance in multiplayer.
3. **Water + Moon + Earth threshold spread.** Two innates cover Moon/Air/Water and Water/Earth.
4. **Call of the Deeps requires Sacred Site.** A 2-Presence-ocean-space gets sacred-site targeting for the 0-cost drowning enabler.
5. **Tidal Boon is a multiplayer lever.** Moves invaders/Dahan for partners — high-value in multi-spirit games.
6. **Swallow the Land-Dwellers harms Dahan.** Be careful in dahan-dense multiplayer scenarios.

## Opening Strategy

### Opening A — Coastal setup 🟥

- **T1**: G2 or G3; play 1 Unique. Place presence in ocean adjacent to invader-heavy coast.
- **T2**: G3 Minor; play Unique.
- **T3**: Reclaim; 2–3 cards.
- **T4 audit**: 5–6 presence in ocean; coastal coverage 3–4 lands; Level 1 innates firing.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Ocean] --> Adv{Adversary?}
  Adv -->|England (esp L5+ coastal)| A[Opening A]
  Adv -->|Habsburg Mining/Russia| A2[Opening A + Early Major]
  Adv -->|Multi-handed with inland partner| A
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Moon, Air, Water (all needed for Pound Ships), Earth (for Ocean Breaks the Shore).

**Aspects**: `[VERIFY]`.

## Card Priority Ratings

Ocean is **Mixed**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Call of the Deeps | A+ | Core drown enabler. |
| Ocean Breaks the Shore (innate) — not a card | — | Drown engine. |
| Swallow the Land-Dwellers | A | Drowning, but hurts Dahan — coordinate. |
| Tidal Boon | A (multiplayer) | Partner support. |
| Grasping Tide | A- | Coastal defense. |

### Majors

Sea Monsters, Fiery Power (Water-element) — coastal targets.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A+ | A | A- | B+ | Coastal invasion = drown-fodder. |
| Brandenburg-Prussia | A | A- | B+ | B | Coastal cities drown. |
| Sweden | A | A- | B+ | B | Favorable. |
| France | B+ | B | B- | C+ | Inland dahan-capture is out of reach. |
| Habsburg Mining | B+ | B | B- | C+ | Inland scaling. |
| Russia | B | B- | C+ | C | Inland-heavy. |
| Scotland | A | A- | B+ | B | Decent. |
| Habsburg Livestock | A | B+ | B | B | Mixed. |

## Synergy Partners

```admonish tip title="Best Partners"
- **Thunderspeaker** — dahan inland; Ocean coastal.
- **Earth** — Earth defends inland; Ocean drowns.
- **Shadows** — drowning generates fear for Shadows's innate.
- **Keeper** — Keeper inland Majors; Ocean coastal.
```

```admonish warning title="Anti-Synergy"
- **Another coastal spirit** — territory conflict.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Growing inland presence — Ocean can't. Re-read the special rule.
```

```admonish failure title="Common Mistake"
Drowning Explorers when Cities are drownable. City drowning = 3 Energy (solo); Explorer drowning = 1.
```

```admonish failure title="Common Mistake"
Ignoring coordination in multiplayer. Territory split matters.
```

## Tempo Profile

| Round | Energy | CP | Ocean Presence | Coast Lands Covered | Key Play |
|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 2–3 | Unique + Minor |
| 2 | 1E | 2 | 5 | 3–4 | Unique + Minor |
| 3 | 1–2E | 2 | 6 | 4–5 | Reclaim + 2 Minors |
| 4 | 2E | 3 | 7 | 5 | Level 1 innates; 1st drown |
| 5 | 2–3E | 3 | 7 | 5 | Drown City (+3E); Major gain prep |
| 6 | 3E | 3 | 6 | 5 | Ocean Breaks the Shore L2 |
| 7 | 3E | 3 | 5 | 5 | Close coastal |
| 8 | 3E | 3 | 5 | 5 | Terror close |

## Major vs. Minor

**Draft bias**: Mixed.

## Expansion Sensitivity

- **Base only**: fully functional.
- **+ Branch & Claw**: coastal event cards interact; check each.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~55%.
- Best vs. England L6 (~68%).
- Worst vs. France L6 (~35%).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- Cross-reference: [Terrain fundamentals](../../fundamentals/terrain.md), [Energy Denial archetype](../../combos/energy-denial.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
