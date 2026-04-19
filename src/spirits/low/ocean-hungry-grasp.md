# Ocean's Hungry Grasp

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low (official) — Moderate in multiplayer           |
| Play Difficulty       | 1                                                  |
| Archetypes            | Energy Denial (coastal-specific) · Fear-Rush       |
| Primary Elements      | Water, Moon, Air                                   |
| Typical Opening       | Full bottom, Minor-heavy                           |
| Typical Draft Bias    | Mixed (Minors early, Major T5+)                    |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | Deeps, Shoreline, Haven                            |
```

## Spirit Overview — Framing

Ocean is the only spirit that treats the ocean itself as presence-territory. Every coastal land is part of Ocean's reach; every invader that touches the coast is a candidate for drowning. Unique placement mechanics + drowning = a spirit that plays nothing like the inland spirits and everything like a coastal-pressure denial engine.

**One-line fantasy**: the sea rises, drags invaders into the depths, and leaves the coasts empty. Inland invaders never even see Ocean's power — they just see their supply chains drowning.

**The honest complexity signal**: Ocean is printed as Low, and for solo coastal-focused games is genuinely accessible. In multiplayer with inland-focused partners, Ocean becomes Moderate because your partner is playing a different *kind* of Spirit Island — they handle the inland; you handle the seam. Coordination required.

## Core Mechanics & Special Rules

### Special Rule: Ocean's presence on the ocean

Ocean's presence lives on the *ocean*, not on specific land terrains. The ocean is Ocean's sacred site; all coastal lands are "adjacent." This changes range/targeting: range-1 from your presence reaches any land adjacent to the ocean.

### Special Rule: Drowning

Pushing an invader or dahan from a coastal land into the ocean removes them from the game (dahan return to supply; invaders destroyed). This is Ocean's signature move — permanent removal without kill-damage.

### Innate: Deeps Swallow the Unworthy

Scales drowning + fear with Water/Moon elements. High-threshold levels drown high-cost invaders (Cities).

### Consuming Depths (Unique)

Core drowning card. Fast power; plays every turn you can afford.

### Tsunami (Unique)

Heavy damage + drowning on a coastal land. Late-game closer.

### Gift of Power (Unique)

Energy-generation unique; fuels the drowning loop.

### Call of the Sea (Unique)

Push + gather power; supporting tool for coastal manipulation.

## Key Strategic Principles

1. **Coastal presence is the engine.** Ocean's primary power lands must touch coastal lands. Growth choices prioritize coastal coverage; inland plays are incidental.
2. **Drown Cities first.** A drowned City = 2 fear + permanent removal. Drowning an Explorer is cheap; drowning a City is game-ending. When both are available, prioritize the City.
3. **Water + Moon elements are load-bearing.** Deeps Swallow the Unworthy innate requires Water threshold; Moon unlocks level 2. Draft element-heavy Minors.
4. **Don't overgrow inland.** Inland presence is wasted — Ocean's cards can't target inland without coastal-adjacency. Keep 80%+ of presence on/adjacent to ocean.
5. **Partner coordination matters in multiplayer.** Coastal/inland territory split must be agreed upfront. Inland-only partners shouldn't expect Ocean to help inland; Ocean shouldn't expect inland help with coasts.
6. **Tsunami sequencing.** Save Tsunami for a turn where it drowns multiple Cities simultaneously. Single-target Tsunami is overpaying for the effect.
7. **Coast-to-coast is the win path.** Ocean's Terror-3 win is "all coastal lands cleared of invaders" effectively; most inland lands become partner-problems.

```admonish tip title="Pro Tip"
When Invader Deck cards say "Explore: Coast," prepare drown stacks on lands about to receive invaders. Drowning an Explorer the turn it arrives = no Town next turn = chain disruption.
```

## Opening Strategy

### Opening A — Full bottom, Minor-heavy 🟨

**Source**: community consensus; BGG strategy threads.

**When to pick this**: most matchups. Default opening.

**Target arc**: 3 CP by T4 · Water + Moon thresholds T4 · Terror 2 flip by T6.

#### Turn 1

- **Growth**: **G2 bottom** (place presence on ocean + card play).
- **Cards played**: **Consuming Depths** (drown 1 Explorer on coast) + **Call of the Sea** (push for positioning) OR a 0-cost Water Minor.
- **Presence placement**: ocean adjacent to a coastal land with the closest invader pressure.
- **Elements by end**: 2 Water, 1 Moon.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: 1 coastal invader drowned; 1 fear from the removal.

#### Turn 2

- **Growth**: **G3 bottom** (Minor gain + presence).
- **Cards played**: **Consuming Depths reclaimed + new Minor**.
- **Presence placement**: second ocean adjacency.
- **Elements by end**: 2 Water, 2 Moon, 1 Air.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: 2 invaders drowned across T1–T2; fear pool at 3/4.

#### Turn 3

- **Growth**: **G1 Reclaim + G3 bottom** (refresh + Minor).
- **Cards played**: **Consuming Depths + 2 Minors**.
- **Presence placement**: third ocean adjacency.
- **Elements by end**: 3 Water, 2 Moon.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Level 2 innate threshold reached; 3 drownings total.

#### Turn 4 — state audit

After T3:

- **Presence**: 5–6 on ocean spaces; coverage of 3+ coastal lands.
- **Energy / CP**: 1E / 2–3CP.
- **Engine state**: Level 2 Deeps Swallow firing; drowning 1–2 invaders per turn.
- **Fear pool**: 4–5 of 8.
- **Blight**: 0–1 (coastal ravages prevented via drowning).

Pivot advice:
- **Inland pressure rising** → accept it's your partner's problem OR gain a Major with inland reach (Sea Monsters, Fiery Power).
- **Missing Moon element** → delay Level 2 to T5.
- **Coastal density too low to target** → use Call of the Sea to push invaders onto coasts you can drown.

### Opening B — Hybrid with Early Major 🟨

**When to pick this**: late-scaling adversaries (Habsburg Mining, some Russia matchups).

- **T1**: G2 bottom + Gift of Power + Consuming Depths.
- **T2**: G4 (Major gain — forget Call of the Sea for Sea Monsters or similar). Play Gift of Power + new Major threshold prep.
- **T3**: Reclaim + play Sea Monsters (or Major) + Consuming Depths. Fire high-threshold drownings.
- **T4 audit**: Major firing; Deeps Swallow level 1–2; 1E bank; 3–4 drownings total.

Opening B trades early scaling for a T4 Major spike. Higher variance.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Ocean] --> Adv{Adversary?}
  Adv -->|England| A[Opening A - standard]
  Adv -->|Brandenburg-Prussia| A
  Adv -->|Sweden| A
  Adv -->|France| A
  Adv -->|Habsburg Mining| B[Opening B - Early Major]
  Adv -->|Russia| B
  Adv -->|Multi-handed with inland partner| A
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Water (primary), Moon (secondary), Air (tertiary).

**Aspects**:

- **Deeps** — extends ocean reach; drowning scales more aggressively.
- **Shoreline** — coastal-focused; better against inland-heavy adversaries.
- **Haven** — support-oriented; dahan protection + coastal defense. Multiplayer pick.

**Pick default**: base Ocean for first games; Deeps for solo optimization; Haven in multiplayer.

## Card Priority Ratings

Ocean is **Mixed** draft-bias.

### Uniques

| Card                            | Grade | Notes                                                |
|---------------------------------|-------|------------------------------------------------------|
| Consuming Depths                | A+    | Core drown; play every turn.                         |
| Tsunami                         | A     | Multi-target drown; save for Cities.                 |
| Gift of Power                   | A-    | Energy generation; fuels majors.                     |
| Call of the Sea                 | B+    | Positional; draft-context dependent.                 |

### Minors to Target

| Card                            | Grade | Why                                                  |
|---------------------------------|-------|------------------------------------------------------|
| Any 0-cost Water Minor          | A+    | Essential for Level 2 innate.                        |
| Dissolving Vapors               | A     | Water + fear.                                        |
| Powerstorm                      | A-    | Air + energy sustain.                                |
| Call of the Dahan               | B+    | Mobility + fear.                                     |

### Majors that Over-perform on Ocean

| Card                            | Grade | Why                                                  |
|---------------------------------|-------|------------------------------------------------------|
| Sea Monsters                    | A+    | Water-major; coastal killer.                         |
| Fiery Power                     | A     | Water-Fire cross; high-threshold damage.             |
| Rouse the Trees and Stones      | A-    | Terrain-agnostic damage.                             |
| Vigor of the Breaking Dawn      | B+    | Off-element; depends on draft.                       |

### Avoid drafting

- **Inland-only Majors** (jungle-gated, mountain-gated). Ocean can't reach them.
- **Sacred-site-only cards**. Ocean's sacred site is the ocean itself; most cards don't target it the way they target land-sacred-sites.

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes                                                |
|----------------------|----|----|----|----|-----------------------------------------------------|
| England              | A+ | A  | A- | B+ | Stage III coastal invasion = Ocean's perfect storm.  |
| Brandenburg-Prussia  | A  | A- | B+ | B  | Coastal cities drown well.                           |
| Sweden               | A  | A- | B+ | B  | Coastal pressure favors Ocean.                       |
| France (Plantation)  | B+ | B  | B- | C+ | Inland dahan-capture is out of Ocean's reach.        |
| Habsburg Mining      | B+ | B  | B- | C+ | Inland scaling; Ocean less relevant mid-game.        |
| Russia               | B  | B- | C+ | C  | Settler mechanics inland; Ocean secondary.           |
| Scotland             | A  | A- | B+ | B  | Decent matchup.                                      |
| Habsburg Livestock   | A  | B+ | B  | B  | Mixed pressure; fine.                                |

## Board Position Evaluation

- **Favorable**: Boards with high coastal density (A, B, E — most boards have 4+ coastal lands).
- **Neutral**: Standard boards.
- **Unfavorable**: Inland-mostly layouts; Ocean under-utilized.

## Game-Phase Strategy

### Early (T1–3)
- Drown 1–2 invaders/turn via Consuming Depths.
- Place presence on ocean adjacent to coastal invader concentrations.
- Draft Water/Moon Minors aggressively.

### Mid (T4–6)
- Level 2 Deeps Swallow firing; 2–3 drownings/turn.
- Gain a Major T5 if coastal threat scaling.
- Fear pool at Terror 2.

### Late (T7+)
- Tsunami for City sweeps.
- Coast cleared; inland partner closes or Ocean closes via Terror 3.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Thunderspeaker** — Thunderspeaker handles inland dahan; Ocean handles coast. Clean territory split.
- **Vital Strength of the Earth** — Earth defends inland; Ocean drowns coast. Durable partnership.
- **Shadows Flicker Like Flame** — fear-rush combination; Ocean drowns generate fear for Shadows innate.
- **Keeper of the Forbidden Wilds** — Keeper handles inland Majors; Ocean handles coast.
```

```admonish warning title="Anti-Synergy"
- **Another coastal-heavy spirit** — Ocean + Serpent = both want coastal cities. Territory conflict.
- **Inland-zero spirits** (none exist in modern meta, but some aspect variants are very inland-focused).
```

## Common Mistakes

```admonish failure title="Common Mistake"
Growing inland presence when coastal threats dominate. Ocean's cards rarely reach inland; inland presence is usually wasted.
```

```admonish failure title="Common Mistake"
Drowning Explorers when Cities are drownable. Cities = 2 fear + 4-damage prevented; Explorers = 1 fear. Prioritize City drownings.
```

```admonish failure title="Common Mistake"
Playing Tsunami on a single target. Save for multi-City coastal turns; otherwise it's overpriced damage.
```

```admonish failure title="Common Mistake"
Ignoring coordination in multiplayer. Ocean + inland partner without territory split = Ocean watching partner handle inland coasts.
```

## Tempo Profile

| Round | Energy | CP | Presence on Ocean | Coast Lands Covered | Key Play                        |
|-------|--------|----|---------------------|----------------------|---------------------------------|
| 1     | 1E     | 2  | 4 (of 13)           | 2–3                  | Consuming Depths + Gift        |
| 2     | 1E     | 2  | 5                   | 3–4                  | Consuming Depths + Minor        |
| 3     | 1–2E   | 2  | 6                   | 4–5                  | Reclaim + 2 Minors              |
| 4     | 2E     | 3  | 7                   | 5                    | Level 2 innate; Major gain prep |
| 5     | 2–3E   | 3  | 7                   | 5                    | Gain Major (Sea Monsters)       |
| 6     | 3E     | 3  | 6                   | 5                    | Tsunami turn; fear spike        |
| 7     | 3E     | 3  | 5                   | 5                    | Terror 2 flip                   |
| 8     | 3E     | 3  | 5                   | 5                    | Terror 3 → close                |

Cliff turn: **T4**. Level 2 innate must fire; coast coverage must be 4+ lands.

## Major vs. Minor — Ocean Specifically

**Draft bias**: Mixed (Minor-heavy early, 1 Major T5).

- T1–T3: Water/Moon Minors.
- T4–T5: Sea Monsters or Fiery Power if offered.
- T6+: sometimes a second Major if banking permits.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1 digital estimates):

- Solo win rate at L6: ~55%.
- Best adversary: England L6 at ~68% (Stage III coastal = perfect matchup).
- Worst adversary: France L6 at ~35%.
- Strong 2-handed: Ocean + Thunderspeaker at ~70% L6.
```

## Source Notes

```admonish abstract title="Sources"
- Community BGG strategy threads on Ocean openings
- [Cardboard Crew tier](https://thecardboardcrew.com/spirit-island-spirits/)
- [Spirit Island Wiki — Ocean](https://spiritislandwiki.com/)
- Cross-reference: [Energy Denial archetype](../../combos/energy-denial.md), [Terrain fundamentals](../../fundamentals/terrain.md)
```

---

*Last revised: 2026-04-19*
