# Card Draft Theory

Every Minor or Major gain pops a 4-card offer. You keep one, the other three go to that deck's discard pile. That's the mechanic. The *theory* — when to gain at all, which deck to draw from, which card to keep, how to coordinate gains with a partner — is what separates drafting by feel from drafting for leverage.

Read this after [Major vs. Minor](./major-vs-minor.md). That chapter sets the archetype-level bias; this one operates below it.

## The 4-card offer

When you gain a Power Card, you first choose Minor or Major, then draw 4 cards from that deck. You keep 1 in your hand; the other 3 go to that deck's discard pile (not back into the deck). Only when the deck runs out do you reshuffle its discard pile.

When you gain a Major Power, you must also Forget one of your Power Cards, unless an effect says you don't have to (e.g., Unlock the Gates of Deepest Power). You may Forget any of your cards — in hand, in your discard, or in play — including the Major you just kept.

You can't pass a gain once the cards are drawn: you always keep one. The real "pass" happens earlier, at Growth, by choosing an option without a gain.

In multiplayer, each gain belongs to the gaining spirit: they draw, keep, and discard. What the table shares is the conversation before Growth (see below; for keeping that conversation fair, see [Alpha-Player Problem](../social/alpha-player-problem.md)).

## The two axes of card evaluation

Every offer card is evaluated on two dimensions, simultaneously:

### 1. Spirit fit

Does the card's **elements**, **cost**, and **effect type** fit your spirit?

- **Elements**: the card adds elements to your per-turn total; you care about elements your innates threshold.
- **Cost**: the card must play within your energy curve.
- **Effect type**: damage, fear, defend, push, gather, etc. — matching your spirit's needs.

A 0-cost Moon Minor for Shadows is A+. A 3-cost Fire-only Minor for Shadows is D because the cost breaks the curve.

### Minors-feed-innates — invert the draft framing

A Phantaskippy-coined re-framing: the best Minors for most spirits aren't "the strongest Minors" — they're **the Minors that unlock your Innate thresholds**. Shadows wants Moon/Fire-feeders more than it wants "good Minors in general"; Lightning wants Air/Fire feeders; Keeper wants Plant/Sun/Earth/Animal feeders because its Innate wants four elements simultaneously.

The framing inversion:

- **Wrong question**: "Is this the best Minor in the offer?"
- **Right question**: "Does this Minor give me an element I need to unlock an Innate I actually plan to fire?"

The right question often downgrades a "stronger" card and upgrades a "weak" utility card. A 0-cost Minor that adds Moon to Shadows's T2 is a 10/10 pick even if the card's base effect is unremarkable — the Innate it unlocks does the real work.

**When the inversion doesn't apply**: spirits whose Innates are damage-incidental (Ocean, Vital Strength early-game) or whose closing win condition is Fear-card effects. There, the Minor's own effect dominates.

### 2. Offer variance

What are the *other* 3 cards? If all 4 are weak fits, you still keep one: pick the least-bad. If 1 is a perfect fit and 3 are irrelevant, take the fit.

Experienced drafters think about expected-next-offer too, but at Growth, before drawing: a later gain draws a fresh 4 from the same shuffled deck, so it is not better on average. The 3 cards you don't keep go to the discard and won't reappear until that deck runs out and is reshuffled.

## The decision tree

Before Growth, ask whether you'll have the Energy and Card Plays to use a new card soon; if not, a growth option without a gain may serve you better. Once you've drawn, for each gain:

<pre class="mermaid">
graph TD
  Q1{Is any offer a top-tier fit?} -->|Yes| Take[Take it]
  Q1 -->|No| Q2{Is the best-of-4 at least B-tier?}
  Q2 -->|Yes| Take
  Q2 -->|No| Q4{Am I critically short on this effect type?}
  Q4 -->|Yes| Cover[Take the card that covers it]
  Q4 -->|No| Least[Take the least-bad; you must keep one]
</pre>

There is no "pass" branch: once the 4 are drawn, you keep one of them.

## Multiplayer draft coordination

The offer is not pooled: the gaining spirit keeps one of their 4, and the rest go to the discard. Players do play each phase simultaneously and may confer as they wish, so the coordination happens in conversation.

### Pre-gain discussion

Before Growth, a quick table check:
- "Who is gaining this turn, and from which deck?"
- "What element or effect type is each of us missing?"

Each spirit then chooses their own growth and keeps their own card. The alpha-gaming risk is real even here. See [Alpha-Player Problem](../social/alpha-player-problem.md) for how to keep this coordination fair.

## Forget math

Majors require a Forget unless an effect waives it. The Forgotten card goes to its deck's discard pile (a Unique goes under your Spirit Panel). Evaluate:

- **Which card am I forgetting?** The weakest Unique, typically. Check the spirit chapter for your "forget-candidate."
- **Is the forget candidate still useful?** Some Uniques degrade in the matchup (Wildfire's Flash-Fires vs. England), making them free Forgets. Others are load-bearing (Earth's Guard the Healing Land), making the Forget painful.
- **Net value**: Major gained − forgotten card utility. If negative, the Major isn't worth it.

## Opening-Track archetypes

latentoctopus's glossary names three opening taxonomies around presence-track usage, which correlate tightly with draft shape:

- **Full top / Full bottom**: you empty one presence track before meaningfully starting the other. Produces extreme curves — heavy energy early + slow CP scaling (Full top) or heavy CP early + slow energy (Full bottom). Draft consequence: Full top → fewer-but-bigger cards (Majors earlier); Full bottom → more-but-cheaper (Minor-heavy).
- **Top / Bottom**: you mostly work one track, occasionally pulling from the other. The default for most spirits; moderate curve. Draft consequence: Mixed archetype.
- **Hybrid**: you pull from both tracks roughly equally. Produces a balanced curve; often the answer when the adversary's pressure shape is split across turns. Draft consequence: flexible — often the right answer when the matchup is ambiguous pre-game.

Pick the opening track **before** T1 growth based on your spirit's archetype (see [Major vs. Minor](major-vs-minor.md)) and adversary pressure (see [Tempo](tempo.md)). Drafting Minor-heavy on a Full-top opening pinches your CP; drafting Major-heavy on a Full-bottom opening starves energy. The archetype, opening, and draft are one joint decision, not three.

Source: [latentoctopus Glossary](https://latentoctopus.github.io/glossary/).

## Draft cadence

Most spirits gain 3–5 cards over a game (excluding Major Power Shoppers who gain 5–7). Your draft cadence is:

- **T1**: if a gain-growth option exists, usually take it.
- **T2–T3**: 1–2 more gains, depending on spirit's draft-bias.
- **T4–T6**: Major consideration (if Mixed or Major-heavy bias); more Minors (if Minor-heavy).
- **T7+**: closer draft — the one that ends the game.

Over-gaining (5+ cards on a Minor-heavy spirit) produces a bloated hand you can't play. Under-gaining (2 cards by T6) leaves you without options. Target 3–4 total for most spirits.

## Seen cards stay out

You can't refuse an offer, and the 3 cards you don't keep go to the discard pile, not back into the deck. They stay out of future offers until the deck runs out and its discard is reshuffled. If you are counting what's left in a deck, every gain removes 4 cards from it.

## Pool-size awareness

With every expansion, the Minor deck has 100 cards and the Major deck 78, after Nature Incarnate's dropped cards (Growth Through Sacrifice is out of the Minors). Your offers draw from this pool. Each gain takes 4 cards out of the deck (1 to a hand, 3 to the discard), and the discard comes back only when the deck runs out — rare with the full pool, possible at high player counts. Ask how many gains the whole table will make before you expect the game to end, and how much of a 100-card deck that actually uses.

## Card-specific grades

Each spirit chapter has a **Card Priority Ratings** table (e.g., [Shadows's table](../spirits/low/shadows-flicker-like-flame.md#card-priority-ratings)) listing the top Minors, Majors, and Uniques for that spirit. Refer to those when drafting at the table.

## Common mistakes

```admonish failure title="Common Mistake"
Taking "the cool Major" without threshold-fit. Vigor of the Breaking Dawn is a beautiful card, but its threshold needs 3 Sun and 2 Animal. A Major is still playable without its threshold, so with no Sun beyond the card's own you pay 3 Energy (and a Forget) for the base effect alone — 2 Damage per Dahan. Ask whether that base effect is worth it to you.
```

```admonish failure title="Common Mistake"
Skipping a gain growth on a turn you needed a card "because the next offer might be better." A later offer is a fresh draw from the same shuffled deck — no better on average.
```

```admonish failure title="Common Mistake"
Drafting a Major that perfectly fits a *future* state you'll reach T6 — but you're on T2 with 1E. You paid the Forget for a 4-turn-delayed card.
```

```admonish failure title="Common Mistake"
Gaining blind in multiplayer. Your partners can't take cards from your offer, but they can tell you which elements and effects the table lacks before you choose which deck to draw from. Ask.
```

## Cross-references

- [Major vs. Minor](./major-vs-minor.md) — archetype bias.
- [Energy Curves](./energy-curves.md) — cost fit.
- [Alpha-Player Problem](../social/alpha-player-problem.md) — draft-coordination etiquette.
- [SICK Card Database](https://sick.oberien.de/) — browse + filter all power cards.

---

*Last revised: 2026-10-19*
