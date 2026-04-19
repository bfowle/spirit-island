# Card Draft Theory

Every Minor or Major gain pops a 4-card offer. You pick one, the other three shuffle back. That's the mechanic. The *theory* — when to take, when to pass, how to coordinate drafts with a partner — is what separates drafting by feel from drafting for leverage.

Read this after [Major vs. Minor](./major-vs-minor.md). That chapter sets the archetype-level bias; this one operates below it.

## The 4-card offer

When you gain a Minor Power, 4 Minor Power cards reveal from the top of the Minor deck. You take 1; the other 3 return to the bottom of the deck.

When you gain a Major Power, 4 Major Power cards reveal from the top of the Major deck. You take 1 (and pay the Forget cost); the other 3 return.

In multiplayer, the offer is **pooled** by default — any spirit can take any revealed card, but only the acting spirit pays the gain cost. This creates coordination opportunities (and alpha-gaming traps — see [Alpha-Player Problem](../social/alpha-player-problem.md)).

## The two axes of card evaluation

Every offer card is evaluated on two dimensions, simultaneously:

### 1. Spirit fit

Does the card's **elements**, **cost**, and **effect type** fit your spirit?

- **Elements**: the card adds elements to your per-turn total; you care about elements your innates threshold.
- **Cost**: the card must play within your energy curve.
- **Effect type**: damage, fear, defend, push, gather, etc. — matching your spirit's needs.

A 0-cost Moon Minor for Shadows is A+. A 3-cost Fire-only Minor for Shadows is D because the cost breaks the curve.

### 2. Offer variance

What are the *other* 3 cards? If all 4 are weak fits, the best of 4 may still be worth taking. If 1 is a perfect fit and 3 are irrelevant, take the fit.

Experienced drafters think about expected-next-offer too: if you pass this mediocre card, the next gain-growth produces a fresh 4-offer. Sometimes the draft is a ratchet — take the decent card now because variance may hand you a worse offer next turn.

## The decision tree

For each gain:

<pre class="mermaid">
graph TD
  Q1{Is any offer a top-tier fit?} -->|Yes| Take[Take it]
  Q1 -->|No| Q2{Is the best-of-4 at least B-tier?}
  Q2 -->|Yes| Q3{Do I have energy + CP for another gain soon?}
  Q3 -->|Yes| Consider[Consider passing; next gain may be better]
  Q3 -->|No| Take
  Q2 -->|No| Q4{Am I critically short on this effect type?}
  Q4 -->|Yes| Take
  Q4 -->|No| Pass[Take the least-bad or pass if possible]
</pre>

"Pass" here means: take a weak card to clear the offer, or take a gain-option that doesn't advance card count (rarely available).

## Multiplayer draft coordination

The 4-offer is pooled. Any spirit can claim a revealed card. This means:

### Pre-gain discussion

Before Spirit A's growth triggers a gain, a quick table check:
- "Anyone want a gain this turn? I could pick different growth."
- "If the offer has X, call dibs."

### During-gain claim

When 4 cards reveal:
- Spirit A (gainer) announces their top pick.
- If Spirit B or C wants a different revealed card MORE than their own upcoming gain, they may swap — but only the *gainer* executes the gain mechanically.

### Gain-pooling patterns

- **Spirit A has a Major gain but nothing fits**. Spirit B needs a Minor but doesn't have a gain this turn. Spirit A can "waste" their Major gain on a Minor-shape offer for Spirit B's benefit — *if* the table agreed.
- **Multiple claims on one card**: communicate before the reveal.

The alpha-gaming risk is real here. See [Alpha-Player Problem](../social/alpha-player-problem.md) for how to keep this coordination fair.

## Forget math

Majors require a Forget. Evaluate:

- **Which card am I forgetting?** The weakest Unique, typically. Check the spirit chapter for your "forget-candidate."
- **Is the forget candidate still useful?** Some Uniques degrade in the matchup (Wildfire's Flash-Fires vs. England), making them free Forgets. Others are load-bearing (Earth's Guard the Healing Land), making the Forget painful.
- **Net value**: Major gained − forgotten card utility. If negative, the Major isn't worth it.

## Draft cadence

Most spirits gain 3–5 cards over a game (excluding Major Power Shoppers who gain 5–7). Your draft cadence is:

- **T1**: if a gain-growth option exists, usually take it.
- **T2–T3**: 1–2 more gains, depending on spirit's draft-bias.
- **T4–T6**: Major consideration (if Mixed or Major-heavy bias); more Minors (if Minor-heavy).
- **T7+**: closer draft — the one that ends the game.

Over-gaining (5+ cards on a Minor-heavy spirit) produces a bloated hand you can't play. Under-gaining (2 cards by T6) leaves you without options. Target 3–4 total for most spirits.

## Pass to shuffle

If an offer is uniformly bad AND you have future gains coming, refusing this one shuffles the 4 back to deck bottom — improving the next gain's offer statistically. Rarely a primary strategy but valid as a tie-breaker.

## Pool-size awareness

The Minor deck has ~101 cards; Major ~80. Your offers draw from this pool. Late-game, the deck shrinks as cards get into hands and stay there. Card variance changes — the 4-card offer late in a long game is often worse than a T2 offer because the "best" cards are already in play.

## Card-specific grades

Each spirit chapter has a **Card Priority Ratings** table (e.g., [Shadows's table](../spirits/low/shadows-flicker-like-flame.md#card-priority-ratings)) listing the top Minors, Majors, and Uniques for that spirit. Refer to those when drafting at the table.

## Stat insight

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1):
- Spirits that pass Major offers more than 30% of the time have higher win rates than spirits that take the best-of-4 every time.
- Over-drafting Majors (4+ total) correlates with loss rates above 60%.
- Sample variance is high; treat as directional, not prescriptive.
```

## Common mistakes

```admonish failure title="Common Mistake"
Taking "the cool Major" without threshold-fit. Vigor of the Breaking Dawn is a beautiful card; with 0 Sun elements, it's dead in hand.
```

```admonish failure title="Common Mistake"
Passing a decent Minor "because the next offer might be better." Variance makes next offers often worse. Take the B+ you have.
```

```admonish failure title="Common Mistake"
Drafting a Major that perfectly fits a *future* state you'll reach T6 — but you're on T2 with 1E. You paid the Forget for a 4-turn-delayed card.
```

```admonish failure title="Common Mistake"
Ignoring the offer-pool in multiplayer. Your Major gain might be better used to let a partner grab a better card. Coordinate.
```

## Cross-references

- [Major vs. Minor](./major-vs-minor.md) — archetype bias.
- [Energy Curves](./energy-curves.md) — cost fit.
- [Alpha-Player Problem](../social/alpha-player-problem.md) — draft-coordination etiquette.
- [SICK Card Database](https://sick.oberien.de/) — browse + filter all power cards.

---

*Last revised: 2026-04-19*
