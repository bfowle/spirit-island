# River Surges in Sunlight

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low                                                |
| Play Difficulty       | 0                                                  |
| Archetypes            | Terrain Control (primary) · Fear-Rush (secondary)  |
| Primary Elements      | Water, Sun, Earth                                  |
| Typical Opening       | Full bottom, Minor-heavy                           |
| Typical Draft Bias    | Mixed (Minors early, Major T6+)                    |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | [River opening 1](https://latentoctopus.github.io/guide/river-opening1/) |
| Aspects               | Sunshine, Travel, Haven                            |
```

## Spirit Overview — Framing

River is the **flexible push-gather spirit** — most forgiving in the game, easiest teach, viable at all difficulty levels. River does many things adequately while scaling into a Massive Flooding closer.

**One-line fantasy**: the river rises, sweeps invaders out of their build targets, feeds the dahan, cleans the land.

**The honest complexity signal**: Low is right. Default teach-spirit because mechanics are intuitive, win path is clear, every turn produces visible effect. Depth is in *when to push vs. gather vs. Bounty* — a subtle optimization that separates casual play from L6 play.

## Core Mechanics & Special Rules

### Innate: Massive Flooding

Water-Sun-Earth scaling damage. Level 3 wipes a coastal land.

- **Level 1 (2 Water)**: modest damage.
- **Level 2 (3 Water, 2 Sun)**: significant damage.
- **Level 3 (3 Water, 2 Sun, 1 Earth)**: massive damage.

### River's Bounty (Unique)

Dahan-heal + Water element generator. Reclaim-loop cornerstone.

### Flood the Hills (Unique)

Push + damage.

### Wash Away (Unique)

Fast damage + push.

### Flow Through the Powers (Unique)

Energy sustain + scaling.

## Key Strategic Principles

1. **River's Bounty every turn.** The reclaim-engine; cards + water elements + dahan-heal incrementally.
2. **Push > kill for early game.** Pushing invaders out of build targets = no build = more time.
3. **Water is load-bearing.** Level 2+ Massive Flooding needs 3 Water.
4. **Reclaim cycles matter.** Bounty + refreshing Minors = Reclaim every 2 turns.
5. **Place presence on coastal/wetland.** Matches element bias; Massive Flooding reaches coastal hubs.
6. **Don't over-commit to Majors.** River scales through Uniques + innate; Majors T6+ closers.
7. **Forgiving spirit = forgiving play.** Experiment.

```admonish tip title="Pro Tip"
You need to play your full starting hand (including River's Bounty) for max-level Massive Flooding to trigger via the reclaim cycle. Plan hand rotation deliberately starting T2.
```

## Opening Strategy

### Opening A — Bottom track, Bounty-focus 🟩

**Source**: [latentoctopus — River opening 1](https://latentoctopus.github.io/guide/river-opening1/)

**When to pick this**: default.

**Target arc**: 3 Water by T3 · 3 Water + 2 Sun by T4 · Max-level Massive Flooding T5–T6 · Terror 2 via kill-fear T6–T7.

#### Turn 1

- **Growth**: **G3 bottom** (presence + Minor gain).
- **Cards played**: **River's Bounty + Wash Away**.
- **Presence placement**: wetland or coastal land adjacent to invaders.
- **Elements by end**: 2 Water, 1 Sun.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Bounty played; 1 Water-damage.

#### Turn 2

- **Growth**: **G2 bottom** (presence + card play).
- **Cards played**: **Flood the Hills + new Minor**.
- **Presence placement**: adjacent to T1 land.
- **Elements by end**: 2 Water, 2 Sun.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: push + damage on second land.

#### Turn 3

- **Growth**: **G1 Reclaim + G3 bottom**.
- **Cards played**: **River's Bounty + a Minor + a Unique (Wash Away reclaimed)** — 3 cards.
- **Presence placement**: third land.
- **Elements by end**: 3 Water, 2 Sun.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Level 2 Massive Flooding threshold; 1 kill.

#### Turn 4 — state audit

After T3:
- **Presence**: 6–7 of 13, clustered wetland/coastal.
- **Energy / CP**: 1E / 2–3CP.
- **Engine**: Level 2 Massive Flooding available; Bounty rotating.
- **Fear pool**: 3–4 of 8.
- **Blight**: 0–1.

Pivot advice:
- **Missing Sun** → Sun-bearing Minor T4.
- **Bounty not reclaimed** → play fresh from hand T4.
- **Adversary scaling too fast** → Level 2 Massive Flooding is enough; don't chase Level 3.

### Opening B — Hybrid with Early Major 🟨

- **T1–T2**: same as Opening A.
- **T3**: G4 Major gain (forget Flow Through the Powers for Vigor or Fiery Power).
- **T4**: Major fires.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — River] --> Adv{Adversary?}
  Adv -->|Any adversary default| A[Opening A]
  Adv -->|Blitz scenario| A
  Adv -->|Teaching a new player| A
  Adv -->|Want a closer Major T4| B[Opening B - Early Major]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Water (primary), Sun (secondary), Earth (Level 3).

**Aspects**: Sunshine (Sun-heavier; earlier Level 2) · Travel (mobility + range) · Haven (dahan-support multiplayer pick).

## Card Priority Ratings

River is **Mixed** draft-bias.

### Uniques
| Card | Grade | Notes |
|---|---|---|
| River's Bounty | A+ | Core engine |
| Wash Away | A | Fast push + damage |
| Flood the Hills | A- | Slow push + damage |
| Flow Through the Powers | B+ | Forget T6 candidate |

### Minors
| Card | Grade | Why |
|---|---|---|
| 0-cost Water Minors | A+ | Element + cheap |
| Dissolving Vapors | A- | Water + fear |
| Draining Swiftness | A | Water-Sun + reclaim |

### Majors
| Card | Grade | Why |
|---|---|---|
| Vigor of the Breaking Dawn | A | Water-Sun-Earth threshold matches |
| Fiery Power | A- | Water-Fire |
| Sea Monsters | A- | Water-heavy |

### Avoid
- Fire/Moon-only Majors.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Coastal town pressure matches strengths |
| Brandenburg-Prussia | A | B+ | B | B- | Fast cities outpace scaling |
| Sweden | A- | B+ | B | B | Fear-agnostic; push prevents builds |
| France | B+ | B | B- | C+ | Dahan capture despite heal |
| Habsburg Mining | A- | B+ | B | B- | Level 2 keeps up |
| Russia | B+ | B | B | B- | Settler push = River's playstyle |
| Scotland | A | A- | B+ | B | Favorable |
| Habsburg Livestock | A- | B+ | B | B | Decent |

## Board Position Evaluation

- **Favorable**: Wetland + jungle + coastal boards.
- **Neutral**: Most boards.
- **Unfavorable**: Mountain-heavy layouts.

## Game-Phase Strategy

- **Early (T1–3)**: Bounty + push plays; Water element accumulation; dahan preservation.
- **Mid (T4–6)**: Level 2 Massive Flooding firing; reclaim cycle stable.
- **Late (T7+)**: Level 3 Massive Flooding if elements align; Terror 2 flip.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Thunderspeaker** — River pushes; dahan retaliate.
- **Earth** — Earth inland; River coastal + wetland.
- **Sharp Fangs** — Fangs beasts + River push = kill-chains.
- **Bringer** — River preserves dahan for Bringer's scaling.
```

```admonish warning title="Anti-Synergy"
- **Ocean's Hungry Grasp** — both coastal; territorial conflict.
- **Heart of the Wildfire** — blight-conflict.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Playing Bounty Slow when Fast would have reclaimed next turn. Bounty is Fast.
```

```admonish failure title="Common Mistake"
Killing Explorers on T2 when pushing prevents the build. Push > kill early.
```

```admonish failure title="Common Mistake"
Not reclaiming on T3. Engine relies on 2-turn reclaim cycles.
```

```admonish failure title="Common Mistake"
Drafting non-Water Minors. Water is threshold-essential.
```

## Tempo Profile

| Round | Energy | CP | Presence | Water | Key Play |
|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 2 | Bounty + Wash Away |
| 2 | 1E | 2 | 5 | 2 | Flood + Minor |
| 3 | 1–2E | 2 | 6 | 3 | Reclaim + 3 cards; Level 2 prep |
| 4 | 2E | 3 | 7 | 3 | Level 2 Massive Flooding |
| 5 | 2E | 3 | 7 | 3+ | Bounty cycle + Major gain |
| 6 | 2–3E | 3 | 6–7 | 3+ | Level 3 if aligned |
| 7 | 3E | 3 | 6 | 3+ | Massive Flooding closer |
| 8 | 3E | 3 | 5 | 3+ | Terror 2-3 close |

Cliff turn: **T3**. Reclaim must happen; Level 2 on track.

## Major vs. Minor — River Specifically

**Draft bias**: Mixed (Minors T1–T4, Major T6+).

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1):
- Solo L6: ~54%.
- Best vs. England L6 at ~65%.
- Worst vs. Brandenburg-Prussia L6 at ~40%.
- High sample size (popular teaching spirit).
```

## Source Notes

```admonish abstract title="Sources"
- [latentoctopus — River opening 1](https://latentoctopus.github.io/guide/river-opening1/)
- [Cardboard Crew tier](https://thecardboardcrew.com/spirit-island-spirits/)
- [Spirit Island Wiki — River](https://spiritislandwiki.com/)
- Cross-reference: [Terrain Control archetype](../../combos/terrain-control.md), [Teaching Methods](../../social/teaching-methods.md)
```

---

*Last revised: 2026-04-19*
