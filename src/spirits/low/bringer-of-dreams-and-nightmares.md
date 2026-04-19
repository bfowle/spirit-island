# Bringer of Dreams and Nightmares

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low (official) — High in win-path weirdness       |
| Play Difficulty       | 2                                                  |
| Archetypes            | Fear-Rush (primary, exclusive)                     |
| Primary Elements      | Moon, Air                                          |
| Typical Opening       | Full bottom, Minor-heavy                           |
| Typical Draft Bias    | Mixed (Minors T1–T4; late Major closer)            |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | Enticing, Violence                                 |
```

## Spirit Overview — Framing

Bringer is the **fear-only** spirit. They don't kill invaders. They don't defend meaningfully. They don't manipulate terrain. What they do is **generate fear on every turn, from every action, in vastly disproportionate amounts**, and win by racing the Terror track to 3 before the adversary overruns the board.

**One-line fantasy**: nightmares spread from dreaming spirits into waking invaders; the colonizers flee the island not because they lost, but because they can't sleep.

**The honest complexity signal**: Bringer is printed Play Difficulty 2 — higher than base Low spirits. The complexity is not the mechanics; the mechanics are simple. The complexity is the **psychological reframe** required to play Bringer well. New players instinctively try to kill invaders with Bringer's cards; they fail. Bringer's win path is fear; the player must internalize "fear is enough" before the spirit feels playable. This takes 2–3 games to click.

## Core Mechanics & Special Rules

### Innate: Predatory Nightmares

Generates fear on Moon/Air thresholds. The core engine.

- **Level 1 (2 Moon)**: 1–2 fear per turn.
- **Level 2 (3 Moon, 2 Air)**: 3 fear per turn + push effects.
- **Level 3 (4 Moon, 3 Air)**: 4+ fear per turn; often game-closing on a single innate firing.

### Special Rule: Presence-Triggered Fear

Bringer's presence placement triggers fear — entering a new land generates fear automatically. Growth that places presence is also generating fear, every turn.

### Nightmares Spread (Unique)

Core fear-generating card. Fast; plays every turn possible.

### Dreams of the Dahan (Unique)

Moon-element generation + dahan-scaling fear.

### Murmurs of a Lost Age (Unique)

Range-heavy fear; scales with existing dahan density in target.

### Manifestation of Dread (Unique)

Heavy fear generator; cost-heavy.

## Key Strategic Principles

1. **Fear is the plan. Always.** Bringer generates 4–6 fear/turn at full engine; the game's win condition (Terror 3 flip) arrives T6–T8. Don't try to kill invaders.
2. **Moon is load-bearing.** All innate levels require Moon. Draft Moon-bearing Minors obsessively; skip anything else.
3. **Place presence aggressively.** Each presence placement = fear trigger. Growth choices that place presence are also fear-gen actions.
4. **Keep invaders alive.** Bringer's fear scales with invader count on the board (via some cards). Killing invaders is anti-synergy with the fear-scaling mechanism. Let them live; fear them away.
5. **Dahan density matters (for some fear cards).** Murmurs of a Lost Age + dahan = multiplied fear. Partner with Thunderspeaker or draft Dahan-summoning Minors.
6. **Don't play defend.** Bringer has no meaningful defense; accepting ravages (and the fear from kills) is the plan. Partner covers board.
7. **Terror 2 flip by T5 is the target.** If fear pool hasn't flipped by T6, the engine underfired; check element thresholds.

```admonish tip title="Pro Tip"
Bringer's "weakness" — no kills — is actually their strength. The more invaders on the board, the more fear Bringer generates per turn. A board with 8 Towns alive on T4 is Bringer's best-case scenario, not a crisis.
```

## Opening Strategy

### Opening A — Full bottom, Minor-heavy 🟨

**When to pick this**: default. Bringer's canonical opening.

**Target arc**: Level 2 innate T4 · Terror 2 flip T5–T6 · Terror 3 flip T7–T8.

#### Turn 1

- **Growth**: **G3 bottom** (presence + Minor gain).
- **Cards played**: **Nightmares Spread + Dreams of the Dahan**. Fast + Slow combo.
- **Presence placement**: inland; place presence in a dahan land to scale Murmurs later.
- **Elements by end**: 2 Moon, 1 Air.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: 2–3 fear generated (innate Level 1 + card); Minor drafted Moon-preferred.

#### Turn 2

- **Growth**: **G3 bottom** (presence + Minor gain again).
- **Cards played**: **Nightmares Spread reclaimed + new Minor**.
- **Presence placement**: second land.
- **Elements by end**: 2 Moon sustained, 2 Air.
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: 4–5 fear total across T1–T2; Minors providing Moon bulk.

#### Turn 3

- **Growth**: **G1 Reclaim + G3 bottom**.
- **Cards played**: **Murmurs of a Lost Age + a Minor**.
- **Presence placement**: third land.
- **Elements by end**: 3 Moon, 2 Air (Level 2 threshold).
- **E / CP state**: 1E / 2CP → 0E / 2CP.
- **Milestone**: Level 2 innate available; 6–7 fear accumulated; Terror 2 flip imminent.

#### Turn 4 — state audit

After T3:

- **Presence**: 6–7 of 13, spread across 4 lands.
- **Energy / CP**: 1E / 2–3CP.
- **Engine state**: Level 2 innate firing; 3–4 fear/turn sustained.
- **Fear pool**: 7–8 of 8 — **Terror 2 flip** imminent this turn or next.
- **Blight**: accept 0–2.

Pivot advice:
- **No Moon element by T2** → take any Moon-bearing Minor on T3 aggressively; delay Level 2 to T5.
- **Board pressure too high** → Bringer can't self-save; partner must cover.
- **Fear pool not filling** → check that all innate-triggering elements are active each turn.

### Opening B — Late-Major variant 🟥

**When to pick this**: experimental; stronger partner in multiplayer.

- **T1–T2**: same as Opening A.
- **T3**: G4 Major gain (forget a weak Unique for Terrifying Nightmares).
- **T4–T5**: Major fires; combines with innate for big fear turns.

Less tested than Opening A; recommend for Bringer-familiar players only.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Bringer] --> Adv{Adversary?}
  Adv -->|Brandenburg-Prussia| A[Opening A]
  Adv -->|England| A
  Adv -->|Scotland| A
  Adv -->|Sweden| A2[Opening A but expect fear penalty drag]
  Adv -->|Russia L3+| B[Opening B - Late Major; fear suppression]
  Adv -->|Multiplayer with board partner| A
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Moon (primary), Air (secondary). Plant and Animal are flavor only.

**Aspects**:

- **Enticing** — emphasizes pull/lure mechanics; adds mobility to Bringer's normally static game.
- **Violence** — adds damage output at the cost of some fear scaling; pairs Bringer with a partial Damage Rush.

**Pick default**: base Bringer. Aspects are niche.

## Card Priority Ratings

Bringer is **Mixed** draft-bias — Minor-heavy until late.

### Uniques

| Card                          | Grade | Notes                                               |
|-------------------------------|-------|-----------------------------------------------------|
| Nightmares Spread             | A+    | Core fear; play every turn.                         |
| Dreams of the Dahan           | A     | Moon + fear + dahan synergy.                        |
| Murmurs of a Lost Age         | A     | Range + dahan-scaling fear.                         |
| Manifestation of Dread        | B+    | High-cost fear burst; T4+ play.                     |

### Minors to Target

| Card                          | Grade | Why                                                 |
|-------------------------------|-------|-----------------------------------------------------|
| Any 0-cost Moon Minor         | A+    | Threshold essential.                                |
| Song of Sanctity              | A     | Fear + range.                                       |
| Dreams of the Dahan (Minor form) | A- | Moon element support.                               |

### Majors that Over-perform

| Card                          | Grade | Why                                                 |
|-------------------------------|-------|-----------------------------------------------------|
| Terrifying Nightmares         | A+    | Fear finisher; Moon-heavy.                          |
| Voice of Command              | A     | Fear + strife; fits Bringer's profile.              |
| Paralyzing Fright             | A-    | 3-cost closer.                                      |

### Avoid drafting

- **Direct-damage-only cards**. Bringer kills by fear, not damage.
- **Earth/Fire-heavy Majors**. Wrong element profile.

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes                                                |
|----------------------|----|----|----|----|-----------------------------------------------------|
| England              | A  | A- | B+ | B+ | Town density = fear farm.                            |
| Brandenburg-Prussia  | A  | A  | A- | B+ | Cities add fear-per-kill; Bringer's sweet spot.      |
| Sweden               | B- | C+ | C  | C- | Fear penalties hurt badly.                           |
| France (Plantation)  | B+ | B  | B- | C+ | Dahan-capture disrupts Murmurs scaling.              |
| Habsburg Mining      | B  | B- | C+ | C  | Scaling adversary outpaces fear-rush.                |
| Russia               | C+ | C  | C- | D  | Fear-suppression mid-late is brutal.                 |
| Scotland             | A  | A- | B+ | B  | Favorable.                                           |
| Habsburg Livestock   | A  | A- | B+ | B  | Decent.                                              |

## Board Position Evaluation

- **Favorable**: Any board. Bringer is terrain-agnostic.
- **Neutral**: Most boards.
- **Unfavorable**: Very sparse boards; fewer fear-trigger opportunities.

## Game-Phase Strategy

### Early (T1–3)
- Nightmares Spread + Dreams every turn.
- Place presence aggressively (each placement = fear).
- Moon-Minor drafts only.

### Mid (T4–6)
- Level 2 innate firing; 3–4 fear/turn.
- Terror 2 flip around T5–T6.
- Consider Major (Terrifying Nightmares) if offer permits.

### Late (T7+)
- Terror 3 flip via accumulated fear.
- Major (if gained) closes; else innate + card sustain.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Thunderspeaker** — dahan density scales Bringer's Murmurs + Thunderspeaker's own fear. Double engine.
- **Shadows Flicker Like Flame** — double fear-rush; Terror 3 arrives by T7.
- **Vital Strength of the Earth** — Earth handles board; Bringer fear-closes.
- **Keeper of the Forbidden Wilds** — Keeper defends; Bringer closes fear-race.
```

```admonish warning title="Anti-Synergy"
- **Spirits that kill invaders fast** (Lightning, Fangs) — fewer invaders alive = less Murmurs scaling. Coordinate pace.
- **Sweden / Russia adversaries** — structural weakness regardless of partner.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Trying to kill invaders with Bringer. There's no "damage per turn" for Bringer beyond what cards incidentally contribute. Fear is the plan.
```

```admonish failure title="Common Mistake"
Not placing presence every turn. Each placement = fear trigger + innate progression. Skipping presence growth = missed fear + missed innate.
```

```admonish failure title="Common Mistake"
Drafting non-Moon Minors. Moon is load-bearing; without Moon, innates stall; without innates, Bringer under-generates.
```

```admonish failure title="Common Mistake"
Playing Bringer solo against Sweden or Russia. The structural fear-suppression makes the engine under-fire consistently.
```

## Tempo Profile

| Round | Energy | CP | Presence | Fear Pool | Key Play                           |
|-------|--------|----|----------|-----------|-----------------------------------|
| 1     | 1E     | 2  | 4        | 1/4       | Nightmares Spread + Dreams        |
| 2     | 1E     | 2  | 5        | 3/4       | Repeat + Minor                    |
| 3     | 1–2E   | 2  | 6–7      | 5/8       | Murmurs + Reclaim                 |
| 4     | 2E     | 2  | 7        | 7/8       | Level 2 innate; Terror 2 imminent |
| 5     | 2E     | 2  | 7        | 3/8 (post-flip) | Second wind; innate + cards |
| 6     | 2–3E   | 3  | 6        | 6/8       | Terror 2 flipped; fear sustained  |
| 7     | 3E     | 3  | 6        | 8/8       | Terror 3 flip                     |
| 8     | 3E     | 3  | 5        | N/A       | Game closes                        |

Cliff turn: **T4**. Level 2 innate must fire by T4 or the Terror 2 target slips to T7, which is often too late.

## Major vs. Minor — Bringer Specifically

**Draft bias**: Mixed (Minor-heavy T1–T4, 1 Major late).

- T1–T4: Minors exclusively. Moon-bearing preferred.
- T5: first Major consideration — Terrifying Nightmares is the go-to.
- T6+: second Minor usually; second Major if curve supports.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1 digital estimates):

- Solo win rate at L6: ~42%.
- Best adversary: Brandenburg-Prussia L6 at ~55%.
- Worst adversary: Russia L6 at ~22%.
- Strong 2-handed: Bringer + Thunderspeaker at ~68% L6; Bringer + Shadows at ~70%.

Caveats: Bringer's solo rate is lower than peers; the spirit shines in multiplayer.
```

## Source Notes

```admonish abstract title="Sources"
- Community BGG strategy threads on Bringer openings
- [Cardboard Crew tier](https://thecardboardcrew.com/spirit-island-spirits/)
- [Spirit Island Wiki — Bringer](https://spiritislandwiki.com/)
- Cross-reference: [Fear Rush archetype](../../combos/fear-rush.md), [Fear Track fundamentals](../../fundamentals/fear-track.md)
```

---

*Last revised: 2026-04-19*
