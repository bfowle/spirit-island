# Major vs. Minor Draft Rates

Most spirit chapters rate a preferred draft bias: **Minor-heavy**, **Major-heavy**, or **Mixed**. Under the hood, that reflects expected value under the spirit's energy curve, innate element needs, and play-count-per-turn budget. This chapter extracts the drivers.

## The arithmetic

Drafting a Major: 3-card-view-pick-1, costs ≥3 Energy to play, gates on elemental threshold. Drafting a Minor: 4-card-view-pick-1 (or 2-pick-1 per growth), costs 0-2 Energy, no elemental gate.

**EV per draft:**

- Major: P(useful Major in view) × E(payoff | played within 3 turns)
- Minor: P(useful Minor in view) × E(payoff) — typically higher P(hit), lower E(payoff).

Low-income spirits (most base-complexity) struggle to play Majors fast enough to amortize the draft opportunity cost. Minor-heavy wins for them.

High-income spirits (Stone's Unyielding Defiance, Starlight, Serpent) or energy-banking growths (Shadows G3, Ocean G3) can afford 3-6E Majors and have the CP to play them alongside cheap Minors.

## Draft bias by archetype

| Archetype | Typical income | Typical CP | Draft bias |
|-----------|---------------:|-----------:|------------|
| Fear-rush low-complexity | 1-2E | 1-2 | **Minor-heavy** |
| Fear-rush + presence (Shadows) | 1-3E | 1-2 | **Mixed** (Minor T1-3, 1 Major T5+) |
| Damage specialist (Lightning, Heart) | 1-3E | 2-3 | **Minor-heavy early, Major T4+** |
| Energy-bank late-ramp (Starlight, Stone) | 2-5E | 2-3 | **Major-heavy** |
| Dahan-coordinator (Thunderspeaker, Hearth-Vigil) | 1-2E | 1-2 | **Minor-heavy** |
| Control/presence (Serpent, Lure) | 2-4E | 2-3 | **Mixed** |
| Complex late-game (Fractured Days, Wandering Voice) | 2-4E | 2-3 | **Major-heavy** |

## Pool-dilution effect

Per [Expansion Dilution Claims](expansion-dilution-claims.md):

- Base Major pool mean: **1.05 fear/card**
- B&C Majors: **2.00** (peak)
- JE Majors: **1.04**
- **NI Majors: 0.83** (lowest)

**Implication**: more expansions → *less* raw Fear per average Major draft. This partially compensates the Minor pool expansion (B&C Minors 0.67 vs. base 0.58).

So all-expansions games have a subtly different Minor-vs-Major balance than base-only. In base, Major drafts are more rewarding on average. In all-expansions, Minor advantage grows.

## When Minor-heavy wins decisively

- Innate depends on specific-element Minors you cheaply chain.
- Energy income caps at 2-3E through mid-game.
- Adversary rewards board-positioning over damage (England early, Sweden, Russia).

## When Major-shopping wins decisively

- Innate doesn't need much elemental help (Bringer — only Moon).
- Adversary has escalation cliffs you pre-empt with a single big play (France L5 plantation removal; Mining L5 scaling).
- You have 4-5E regular income by T5.

## Gain-1-Major growth option?

Starlight, Fractured Days, Lure of the Deep Wilderness have growths that gain a Major. Taking one costs:

- 1 presence not placed
- 1 CP slot on a card you may not afford for 2-3 more turns

It's correct when the Major carries a game-phase AND your expected income crosses the Major's cost within 2 turns of drafting.

## See also

- [Deck Expansion Impact](deck-expansion-impact.md) — pool sizes per combo.
- [Fear Card Expected Value](fear-card-expected-value.md) — Fear-output calibration.
- [Card Draft Theory](../fundamentals/card-draft-theory.md) — fundamentals-level treatment.
- Each spirit chapter's **Card Priority Ratings** — full-pool picks.
