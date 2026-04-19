# Shadows Flicker Like Flame

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Low (official) / deceptively Moderate once optimizing |
| Play Difficulty       | 1                                                  |
| Archetypes            | Fear-farm · Terror-3 closer · Stealth / dodge      |
| Primary Elements      | Moon, Fire, Air                                    |
| Typical Opening       | Hybrid or Full bottom, Minor-heavy                 |
| Typical Draft Bias    | Mixed (Minors early; 1 Major late T5–T7 as closer) |
| Rei's Guide           | Not covered by Rei specifically; see latentoctopus |
| latentoctopus         | Not currently listed; use Shadows's community threads |
| Aspects               | Madness, Reach, Amorphous, Foreboding              |
```

## Spirit Overview — Framing

Shadows is the game's clearest "fear-rush with board avoidance" archetype. You hide in terrain, accumulate moonlight in your presence tracks, and chain innate fear + strife until Terror 3 flips the win condition wide open.

**One-line fantasy**: you're the unseen thing at the edge of the firelight — invaders feel dread before they even see a town burn down.

**The honest complexity signal**: Shadows *looks* Low-complexity because the cards read simply, but optimizing her is Moderate. The decision depth is in *when to hold back* (keep invaders alive to fear-harvest) versus *when to strike* (kill an invader to cross a fear threshold). New players play her too aggressively and fail to leverage fear generation. You're about to make that mistake if you don't know it's a mistake.

## Core Mechanics & Special Rules

### Innate: Dark and Tangled Wood

Generates fear + strife when thresholds hit. The central engine.

- **Level 1 (1 Moon)**: 1 fear. Almost free; usually active T1–T2.
- **Level 2 (2 Moon, 1 Fire)**: 2 fear + strife. The bread-and-butter state by T3–T4.
- **Level 3 (3 Moon, 2 Fire, 2 Air)**: 3 fear + more strife. Requires element loading; a T5+ payoff most games.

### Innate: Favors Called Due

Targets strife-ed invaders; damage-via-fear-conversion. The finisher.

### Offering of Fear and Flame

Your signature Unique: generates fear on-demand. Keep it in rotation.

### Mantle of Dread

Fast defend + fear. Use sparingly — Mantle is card economy pressure.

### Memory of Fire Now Forgotten

Targeting flexibility; often forgotten for a Major late-game.

## Key Strategic Principles

1. **Fear first, kills second.** Shadows wins by Terror 3 flipping the win condition, not by board-clearing. Kill rate is secondary — accept that invaders survive longer than with Lightning.
2. **Moon is your primary element.** Getting to 3 Moon by T4 is the floor; 3 Moon + 2 Fire + 2 Air unlocks the full innate. Draft element-rich Minors.
3. **Strife is a soft-kill.** A strife-ed invader deals half damage next ravage. Use strife to defuse ravages you can't prevent, not (just) to set up the innate.
4. **Avoid sacred-site density.** Shadows doesn't use sacred-site triggers — spread presence instead. 2 presence per land = wasted placement.
5. **Hold cards for reclaim cycles.** Shadows has 2CP openings. If you blow the hand on T1–T2, T3 is terrible. Plan your reclaim cycle.
6. **The "Memory of Fire" forget.** When a Major comes that fits (Moon/Fire/Air-biased), forget Memory of Fire Now Forgotten. It's your weakest Unique by late game.
7. **Avoid the "over-kill" temptation.** You'll sometimes be able to destroy a Town that you could instead leave alive for 2 more turns of fear generation. Count fear math: leaving a Town alive for 2 turns of level-2 innate = +4 fear; killing it = +1 fear. Leave it alive if fear is on the edge.

```admonish tip title="Pro Tip"
Before Slow powers, count fear on the pool: if you're within 2 fear of a Terror flip, bias every decision toward generating that fear, even at cost of board control. Once you flip, the next several rounds' win conditions loosen dramatically.
```

## Opening Strategy

Shadows doesn't have a fully-dissected latentoctopus opening yet (as of 2026-04-19). Here's the community-consensus distillation:

### Opening A — Hybrid, Minor-heavy 🟨

Canonical opening against Brandenburg-Prussia and England.

- **T1**: G3 bottom (gain Minor + place presence) → play Offering of Fear and Flame + any Minor that hits Moon.
- **T2**: G1 or G2 (energy bias) → play Mantle of Dread if the ravage is bad; else Offering again + a Minor.
- **T3**: Reclaim → 3 cards in hand; pre-threshold Level 2.
- **T4**: Level 2 innate active. Play 3 cards/turn. Strife-heavy.
- **T5+**: Transition to Major consideration. Forget Memory of Fire. Keep the engine running.

**Target milestones**: 2 Moon by T2, 3 Moon + 1 Fire by T4, Level 2 innate firing reliably by T4.

### Opening B — Full bottom, heavier Minor draft 🟨

For matchups where you need to scale into T6+ (e.g., Habsburg Mining, Russia).

- **T1–2**: G3 twice (presence + Minors).
- **T3**: Reclaim + gain Minor.
- **T4+**: Fire up the engine and scale fear.

Slightly slower than Opening A but more resilient against mid-game adversary spikes.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Shadows] --> Adv{Adversary?}
  Adv -->|Brandenburg-Prussia| A[Opening A - Hybrid]
  Adv -->|England| A
  Adv -->|Sweden| A2[Opening A, but prioritize early fear-kills]
  Adv -->|Russia L3+| B[Opening B - Full bottom]
  Adv -->|Habsburg Mining| B
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Moon (primary), Fire (secondary), Air (tertiary). Secondary — Plant and Animal are mostly flavor on Shadows; don't draft for them.

**Aspects**:

- **Madness** (Jagged Earth) — converts spirit mechanics toward fear-heavy play. Strong on Shadows; arguably the best aspect. Leans deeper into strife + fear generation.
- **Reach** (Jagged Earth) — extends range on powers. Useful vs. terrain-sparse layouts; less essential than Madness.
- **Amorphous** (Promo Pack 2) — unusual mobility; experimental but niche.
- **Foreboding** (Promo Pack 2) — additional fear mechanics. Good, but Madness usually preferred.

**Pick default**: Madness on any matchup you want more fear generation. Base Shadows otherwise.

## Card Priority Ratings

Shadows is Mixed draft-bias — Minors-heavy early, 1 Major late.

### Unique (Signature) Powers

| Card                            | Grade | Notes                                                   |
|---------------------------------|-------|---------------------------------------------------------|
| Offering of Fear and Flame      | A+    | Core engine card. Play most turns.                      |
| Mantle of Dread                 | B+    | Cost-heavy; use when ravage is unavoidable.             |
| Dark and Tangled Wood (innate)  | A     | Your engine; track thresholds each turn.                |
| Memory of Fire Now Forgotten    | B     | Weakest Unique; usually forget T5+.                     |
| Favors Called Due (innate)      | A-    | Closer; requires strife setup.                          |

### Minors to Target

| Card                            | Grade | Why with Shadows                                        |
|---------------------------------|-------|---------------------------------------------------------|
| Strengthen the Gifts of the Earth | A   | Moon-heavy; threshold-fit.                              |
| Song of Sanctity                | A-    | Fear generation + range.                                |
| Dreams of the Dahan             | A-    | Low-cost Moon source.                                   |
| Call of the Dahan               | B+    | Opportunistic; helps when dahan are thin.               |
| Any 0-cost Moon card            | A     | Auto-take.                                              |

### Majors that Over-perform on Shadows

| Card                            | Grade | Why                                                     |
|---------------------------------|-------|---------------------------------------------------------|
| Terrifying Nightmares           | A+    | Fear finisher; Moon-fit.                                |
| Voice of Command                | A     | Generates fear + strife combined.                       |
| Paralyzing Fright               | A     | 3-cost; fits Shadows's energy by T5.                    |
| Pent-Up Calamity                | A-    | Conditional but explodes with strife pre-staged.        |
| Call of the Dahan's Blood       | B+    | Situational; lower-tier.                                |

### Cards to AVOID drafting

- **Strong-damage Earth/Stone Majors** — no element synergy; Shadows can't reliably threshold them.
- **Moon+Plant only cards** — wasted if Plant is never a useful element for you.

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes                                                      |
|----------------------|----|----|----|----|-----------------------------------------------------------|
| England              | A  | A- | B+ | B  | Slow builds; fear-rush works. Stage III bites coast-heavy. |
| Brandenburg-Prussia  | A  | A- | B+ | B  | Cities add fear-per-kill. Favorable.                       |
| Sweden               | A- | B+ | B  | B- | Fear penalties reduce the engine's output; play carefully. |
| France (Plantation)  | B+ | B  | B  | C+ | Dahan pressure hurts fear conversion. Avoid.               |
| Habsburg Mining      | A  | B+ | B  | C+ | Scaling issue late.                                        |
| Russia               | B+ | B  | B- | C  | Fear-suppression mid-late cripples Shadows.                |
| Scotland             | A  | B+ | B+ | B  | Generally favorable.                                       |
| Habsburg Livestock   | A  | A- | B  | B  | Favorable; decent matchup.                                  |

*Grades are directional; see per-adversary chapter rows for reasoning.*

## Board Position Evaluation

- **Favorable**: Boards A, D, F. Jungle-mountain density favors Moon-fed innate.
- **Neutral**: Boards B, C, E.
- **Unfavorable**: Boards with very sparse terrain layouts reduce fear-harvest opportunities.

## Game-Phase Strategy

### Early (T1–3)

- Place presence on 3–4 lands (spread, not stacked).
- Play Offering + 1 Minor per turn.
- Hit 2 Moon + 1 Fire by T3.
- **Don't** kill invaders you could leave for fear farming.

### Mid (T4–6)

- Level 2 innate firing every turn.
- 3 CP, 2E.
- Transition: forget Memory of Fire for a Major around T4–5.
- Strife accumulates across 4–6 lands.

### Late (T7+)

- Terror 2 is the goal by T6; Terror 3 by T8.
- Closer cards (Terrifying Nightmares, Voice of Command) fire here.
- Don't over-extend presence — ravages are harder, and you want to finish fear, not outlast blight.

## Synergy Partners (Multiplayer)

```admonish tip title="Best Partners"
- **Bringer of Dreams and Nightmares** — double fear-engine. Bringer draws the fear card at Time Passes, then Shadows triggers innate fear for the next turn.
- **Thunderspeaker** — Thunderspeaker handles board pressure while Shadows fear-rushes. Clean territory split.
- **Ocean's Hungry Grasp** — Ocean drowns coasts, Shadows fear-feeds off Inland Cities. Both win independently.
```

```admonish warning title="Anti-Synergy"
- **Wildfire** — territory conflict (both want Moon/Fire and invader-alive-for-scaling in overlapping ways). Works but crowds the board.
- **Vengeance** — both fear-heavy; you'll step on each other's generation. Not bad but suboptimal.
```

## Common Mistakes

```admonish failure title="Common Mistake — Over-killing"
Killing a Town on T2 that would have given you 4 more fear over T3–T4 if left alive. Track fear math deliberately.
```

```admonish failure title="Common Mistake — Sacred-site hoarding"
Stacking 2 presence in a land hoping for synergy. Shadows has no sacred-site triggers. Spread presence across 5+ lands instead.
```

```admonish failure title="Common Mistake — Missing Moon element"
Drafting a no-Moon Minor because it has "cool fear effect." Without Moon, your Level 2 innate doesn't fire, and you lose ~3 fear/turn.
```

```admonish failure title="Common Mistake — Mantle of Dread on T1"
Mantle is a 2-energy card. On T1, you can't afford it. Save it for T3+ when a ravage would hurt.
```

```admonish failure title="Common Mistake — Trying to board-clear"
Shadows isn't a clear-all-invaders spirit. If you're trying to out-damage Fangs or Lightning, you're playing the wrong spirit.
```

## Tempo Profile

Target round-by-round state (varies with opening variant):

| Round | Energy | CP | Presence on Board | Key Play / Milestone                      |
|-------|--------|----|--------------------|-------------------------------------------|
| 1     | 1E     | 2  | 4 (of 13)          | Offering + Minor; place 1 presence        |
| 2     | 1E     | 2  | 5                  | Offering + Minor; hit 2 Moon              |
| 3     | 2E     | 2  | 6                  | Reclaim + Minor; Level 2 innate available |
| 4     | 2E     | 3  | 7                  | Engine ON: 3 cards, Level 2 firing         |
| 5     | 2E     | 3  | 7                  | Gain Major; forget Memory of Fire          |
| 6     | 3E     | 3  | 6                  | Fear-rush push; Terror 2 flip              |
| 7     | 3E     | 3  | 6                  | Closer Major lands                         |
| 8     | 3E     | 3  | 5                  | Terror 3 → game                            |

Cliff turn: **T4**. If Level 2 isn't firing by T4, you're behind and the late-game won't recover.

## Major vs. Minor — Shadows Specifically

**Draft bias**: Mixed (Minors-heavy early, 1 Major T5 as closer).

- **T1–3**: Minors only. 0-cost Moon-bearing Minors are auto-take.
- **T4–5**: First Major consideration. Forget Memory of Fire. Prefer Terrifying Nightmares, Voice of Command, Paralyzing Fright.
- **T6+**: Second Major optional. Usually more Minors if the curve supports it.

See [Major vs. Minor Fundamentals](../../fundamentals/major-vs-minor.md).

## Stat Snapshot

```admonish note title="Stat Insight"
Per [mindwanderer](https://mindwanderer.net/si/stats.html) (digital data, 2026-Q1 estimates):

- Solo win rate at L6: approximately 52% (95% CI wide; sample ~400)
- Best adversary: Brandenburg-Prussia L6 at ~62%
- Worst adversary: Russia L6 at ~38%
- Strong 2-handed combo: Shadows + Bringer wins at ~70% at L6

**Caveats**: digital-only data; Madness aspect not always filtered distinctly. Expect tabletop variance.
```

## Source Notes

```admonish abstract title="Sources"
- latentoctopus glossary (notation): https://latentoctopus.github.io/glossary/
- Cardboard Crew tier list: https://thecardboardcrew.com/spirit-island-spirits/
- mindwanderer stats: https://mindwanderer.net/si/stats.html
- BGG Spirit Island general strategy forum: https://boardgamegeek.com/forum/1543090/spirit-island/strategy
- Spirit Island Wiki — Shadows page: https://spiritislandwiki.com/
- Spirited Discussion podcast: https://spiriteddiscussion.substack.com/
```

---

*Last revised: 2026-04-19 — v0.1 (MVP authored, to be play-tested and revised)*
