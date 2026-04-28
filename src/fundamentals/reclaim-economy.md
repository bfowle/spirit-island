# Reclaim Economy

Reclaim is the quietest Growth option in Spirit Island and the one most often discussed by its community-canonical terms — **reclaim cycle**, **reclaim loop**, **cumulative energy**. When you internalize Reclaim as an economy rather than a button you press when you run out of cards, whole spirit archetypes open up.

This chapter codifies that economy.

## The rule

**Reclaim** (Growth option on most spirits): retrieve all your played Power Cards back into your hand.

That's the whole rule text. The strategic depth is in *when* you Reclaim — which determines how often each Unique can fire across a game.

## Key terms (community notation)

From latentoctopus's canonical glossary, lifted into this book:

- **Reclaim cycle**: the turns between one Reclaim (inclusive) and the next Reclaim (exclusive). A 4-turn reclaim cycle means you Reclaim on T1, play cards T1–T4, Reclaim again T5.
- **Reclaim loop**: a reclaim cycle of length 1 — you Reclaim every turn. Lightning and Grinning Trickster can sustain this; most spirits cannot.
- **Cumulative Energy (CE)**: total energy you've gained via Growth across the game. Track this — it's the denominator for every Reclaim-amortization calculation.

## Why Reclaim is an economy, not a button

Each card you play is "spent" — it lives in your discard until Reclaim brings it back. So the number of times you can fire a given Unique over a game is bounded by:

**Plays per game = (total turns) ÷ (reclaim cycle length) × (CP available those turns, capped at hand size)**

Working example — Shadows on an 8-turn game:

| Reclaim cycle | Plays of Concealing Shadows |
|---|---|
| 4 turns (reclaim T1, T5) | 2 full Reclaims × 4 CP ≈ 6–7 plays (capped by CP each turn) |
| 2 turns (reclaim T1, T3, T5, T7) | 4 Reclaims × 2 CP each ≈ 8 plays, but you gave up a Growth slot 3 extra times |
| 1 turn (loop) | 8 plays, but every turn burned on Reclaim Growth |

The faster you Reclaim, the more times each card fires — but the fewer Growth slots you have for energy, presence placement, and new card gains. **Reclaim cycle is a resource allocation, not a convenience.**

## The four reclaim archetypes

Spirits cluster by how they approach Reclaim economy:

### 1. Reclaim-loopers

Every turn is Reclaim + 1 CP (or 2 CP if the spirit has a Growth branch that stacks Reclaim with other gains). The Uniques fire every single turn.

**Spirits**: Lightning's Swift Strike (classic — 1-CP plays, all Fast, fires every turn), Grinning Trickster (loop opening variant), Shadows on specific openings.

**Draft consequence**: Majors that you play once per loop cycle are devastating. Powerstorm's "Repeat a card you played this turn" with a 1-turn loop means *every turn* Powerstorm doubles another card. Minors are underdrafted — you don't need volume when each card fires 6–8 times.

### 2. Short-cycle reclaim (2–3 turns)

Reclaim every 2nd or 3rd turn. The dominant pattern for moderate-complexity spirits.

**Spirits**: Downpour Drenches the World (opening 1), many Shroud openings, Keeper variants.

**Draft consequence**: Minors that add elements matter, because you play them multiple times per cycle. Majors are viable but you need ≥2 cycles left to amortize them (see Forget Economy in [Major vs. Minor](major-vs-minor.md)).

### 3. Long-cycle reclaim (4+ turns)

Reclaim once or twice in the entire game. You're playing a slow, card-accumulation strategy — the Reclaim happens only after the hand is fully depleted.

**Spirits**: Vital Strength of the Earth, Stone's Unyielding Defiance, most heavy-Major shoppers.

**Draft consequence**: Each card played matters a lot — maybe it fires twice the whole game. Forget cost is enormous (you'll rarely re-play a forgotten Unique). Majors here need to be game-closing; a Major played once and never again is exactly the "just take the shiny" trap the Major vs. Minor chapter warns against.

### 4. No-reclaim (or reclaim-irrelevant)

Some spirits rarely Reclaim because their card economy is structured elsewhere. Shifting Memory of Ages can re-gain Forgotten cards (a Reclaim analogue); Fractured Days can replay other spirits' cards; Serpent has its own card-slot math around being Dormant.

**Draft consequence**: the Reclaim cycle math doesn't apply — use the spirit's specific card-economy chapter.

## When to Reclaim

The decision usually reduces to three questions, answered in order:

1. **Am I about to be handlocked?** If you've played 3 of 4 cards and the one in hand doesn't match next turn's needs, Reclaim now rather than wasting a turn drawing air.
2. **Does the Growth slot have a better alternative this turn?** Energy gains, presence placement, and card gains all compete with Reclaim. If the board state or adversary cliff demands one of the others, Reclaim later.
3. **Am I at the end of my reclaim cycle anyway?** If your cycle is 4 turns and you're on turn 4, Reclaim and get on with it. Don't squeeze an extra turn out of a cycle that's already paid.

```admonish tip title="The +1 rule"
You should *almost never* Reclaim with more than 1 card still in hand. If you have 2 cards in hand and you Reclaim, you paid a Growth slot to "refresh" a hand that wasn't empty — a ~50% Reclaim inefficiency on that cycle. Exceptions: the Growth branch bundles Reclaim with another gain that was worth taking on its own; you're setting up a specific card-combination next turn.
```

## Reclaim + Repeat stacking

Spirits with native Repeat access (Lightning's Powerstorm, Gift of Strength in hand, Keeper's Innate) see Reclaim's effective card count multiply. A Lightning loop with Powerstorm means:

- Reclaim (every turn) → 2 CP → Powerstorm + one target → target plays twice.

That's effectively 3 card-firings per turn from a 2-CP slot. No spirit without this combination can match that per-turn throughput.

Draft consequence for loopers: Repeat-target cards are premium picks. A cheap Minor that enables Powerstorm's Repeat is worth more than an expensive unique-effect Minor — because the Repeat doubles whatever you put on the other slot.

## Common mistakes

```admonish failure title="Three patterns"
1. **Reclaiming on autopilot when the Growth menu had a better option.** Energy gains compound over the rest of the game; a Reclaim you didn't need wastes the whole slot.
2. **Extending the cycle too long to "save" Growth slots.** If your hand is empty and you skip Reclaim because you're banking Growth slots for later, you wasted a turn on one card play. That lost play compounds worse than the Growth slot you saved.
3. **Ignoring the cycle when drafting.** Drafting 3 Minors on a 4-turn-cycle spirit is fine — they each fire 2× per game. Drafting 3 Minors on a long-cycle spirit is bloated; you'll only play each 1× and the slot pressure eats into your Uniques.
```

## Further reading

- [Energy Curves](energy-curves.md) — Reclaim cycle interacts with energy via the "Reclaim + gain 1 Energy" Growth branches.
- [Major vs. Minor](major-vs-minor.md) — Forget Economy amortization depends on cycle length.
- [Card Draft Theory](card-draft-theory.md) — draft cadence aligns with Reclaim cadence.
- [latentoctopus Glossary](https://latentoctopus.github.io/glossary/) — canonical terms (Gn, nE, nCP, CE, cycle, loop).
- [BGG thread 2652960 — Lightning Reclaim Loop](https://boardgamegeek.com/thread/2652960) — deepest community discussion of the 1-turn-loop case.
