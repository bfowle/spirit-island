# Major vs. Minor Draft Rates

Most spirit chapters rate a preferred draft bias: **Minor-heavy**, **Major-heavy**, or **Mixed**. Under the hood, that reflects expected value under the spirit's energy curve, innate element needs, and play-count-per-turn budget. This chapter extracts the drivers.

## The arithmetic

Gaining either kind is the same draw: draw 4 from the chosen deck, keep 1, and put the other 3 in that deck's discard (core rulebook, "Gaining Power Cards"; `data/references/rules/open-questions.md` Q9). A Major also makes you Forget a Power Card unless the effect waives it.

- **Major cost:** 2 to 9 Energy. Nine of the 78 Majors cost 2, among them Entwined Power and The Trees and Stones Speak of War (base game) and Tigers Hunting (Branch & Claw); 22 cost 3 (engine card list `power_cards.json`, `powerType` "Major", `energyCost`; the wiki card data agrees).
- **Minor cost:** 0 or 1 Energy (36 cost 0 and 64 cost 1 in the active pool of 100, per the wiki card data; the engine list agrees once the dropped Growth Through Sacrifice is set aside).
- **Thresholds are a bonus, not a gate.** You can play a Major for its base effect without meeting its Elemental Threshold; the threshold only adds its extra effect when you have the Elements (core rulebook, "Elemental Thresholds": optional effects you may use only with those Elements). Most Majors print a threshold; a few Minors do too (the engine list encodes one on, for example, Drought and Renewing Rain).

**EV per draft:**

- Major: P(useful Major in view) × E(payoff | played within 3 turns)
- Minor: P(useful Minor in view) × E(payoff) — typically higher P(hit), lower E(payoff).

Low-income spirits (most base-complexity) struggle to play Majors fast enough to amortize the draft opportunity cost. Minor-heavy wins for them.

High-income spirits (Stone's Unyielding Defiance, Starlight, Serpent) or energy-banking growths (Shadows G3, Ocean G3) can afford the 4+ Energy Majors and have the CP to play them alongside cheap Minors. A 2-cost Major is within reach of almost anyone, so its cost is rarely the reason to skip it.

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

The income and CP ranges are rough authoring estimates, not computed from presence-track data; check the spirit's chapter for its actual tracks.

## Pool-dilution effect

Mean "N Fear" printed in each card's base effect (threshold text excluded), from `scripts/expansion-deltas.py` run on the current wiki card data (see also [Expansion Dilution Claims](expansion-dilution-claims.md), whose base-game rows, n = 41 Majors and 50 Minors, don't match the current 22- and 36-card base pools):

- Base Majors (22 cards): **1.18 fear/card**
- B&C Majors (21): **2.00** (peak)
- JE Majors (23): **1.04**
- **NI Majors (12): 0.83** (lowest)

**Implication**: across all expansions, Majors print 102 Fear over 78 cards (102 ÷ 78 ≈ 1.31 per card) against 26 ÷ 22 ≈ 1.18 in base alone. B&C's Majors pull the average up and NI's pull it down (base + B&C + JE: 92 ÷ 66 ≈ 1.39). Minors go from 18 ÷ 36 = 0.50 in base to 58 ÷ 100 = 0.58 with everything; B&C Minors average 20 ÷ 30 ≈ 0.67.

So all-expansions games shift both decks a little toward printed Fear, Majors more than Minors. Printed Fear is one narrow measure of a card's value, so treat this as a nudge, not a draft rule.

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
