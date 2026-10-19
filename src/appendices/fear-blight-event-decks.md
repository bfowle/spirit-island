# Fear, Blight & Event Decks Reference

Quick reference for the three non-Power decks. For per-expansion statistical breakdowns see Part VIII.

## Fear deck

Earned with Fear, one card at a time. Each Fear card prints three texts — Terror Level I, II and III — and only the current Terror Level's text resolves.

**Pool sizes** (active cards, from the wiki data; the engine's `fear_cards.json` gives the same split):

- Base: 15 cards
- +B&C: +15 → 30 cards
- +JE: +6 → 36 cards
- +NI: +9 → 45 cards
- +Promo Pack 2 (Feather and Flame): +5 → 50 cards

See [Fear Card Expected Value](../statistics/fear-card-expected-value.md) for per-Terror-Level output and [Expansion Dilution Claims](../statistics/expansion-dilution-claims.md) for swing analysis.

### Mechanics summary

1. **Setup.** Put 4 Fear Markers per player in the Fear Pool (+1 per player with England L6 or an extra board). Deal 9 Fear Cards, split 3/3/3 by the Terror Level II and III dividers (Adversaries change the split).
2. **Generating Fear.** Destroying a Town = 1 Fear, a City = 2, an Explorer = 0. Only destroying counts, not removing. Powers, Events and other effects also generate Fear.
3. **Earning a card.** When every marker in the Fear Pool has been generated (4 × players Fear), move the top Fear Card face down to Earned Fear Cards and return the markers; leftover Fear carries over.
4. **Terror Level.** Earning the card that reveals a divider raises the Terror Level at once, and the new victory condition applies at once. Solo, that is the 3rd card (12 Fear) for TL II and the 6th (24 Fear) for TL III. Earning the last card is a Fear victory.
5. **Resolving.** Earned cards flip in the Invader Phase's Fear step (after the Event) and resolve in the order earned, each at the Terror Level current at that moment. A card earned while resolving goes to the bottom of the stack and uses any new Terror Level.

Sources: core rulebook setup and "Fear"; FAQ, Fear and Fear Cards; `data/references/rules/open-questions.md` Q6–Q8.

### Typical card count

Monte Carlo (`scripts/deck-simulate.py`, 5 000 trials × 8 turns × 1 player, at an **unsourced placeholder** rate of 2.5 Fear per turn): **about 4.6 Fear Cards earned per game** (20 Fear, less an average leftover of about 1.5, ÷ 4). With N players each card costs 4N Fear, so the same total rate earns 1/N as many.

## Blight deck

One card is drawn at setup, Healthy Island side up. When the last Blight leaves it, flip it and follow the other side. A Still-Healthy card instead draws a new Blight Card when its pool runs out and goes straight to that card's reverse side (Jagged Earth rulebook). When the last Blight leaves a Blighted side, the players usually lose.

**Pool sizes** (active cards; matches the engine and the Steam allowed pool):

- Base: 2 (`Downward Spiral`, `Memory Fades to Dust`)
- +B&C: +6 → 8 (Tipping Point is dropped by Nature Incarnate)
- +JE: +7 → 15
- +NI: +8 → 23

See [Blight Deck Severity](../statistics/blight-deck-severity.md) for severity rankings.

### Setup reminder

- **Every game with a Blight Card**: place it Healthy Island side up **without looking at the back** (core rulebook setup step 5). With Jagged Earth, put the whole shuffled stack on the Blight space and use the top card. Branch & Claw makes a Blight Card required.
- **Blight on the card**: 2 per player **+ 1** at setup (5 per player + 1 with no Blight Card). The +1 errata applies at setup only, never when a card flips; the Horizons board already prints it (NI rulebook p.5; FAQ). On a flip, add the other side's number per player. With an extra board, count boards instead of players, at setup and on the flip (Jagged Earth rulebook).
- **Solo option**: you may redraw a Blighted card with only 2 Blight per player, but not a Still-Healthy card (Branch & Claw rulebook; NI rulebook p.14).

The hidden back introduces uncertainty. Strategies that assume a specific Blight escalation (e.g., "don't fill because we're on All Things Weaken") require flipping first or accepting risk.

## Event deck

Drawn once per Invader Phase in games with B&C+. On Turn 1 the Event is drawn and discarded without resolving; it is public in the Event discard (Jagged Earth rulebook p.12). A typical card has a main Event that branches on the Island's health, the Terror Level or the Invader Stage (or is a Choice Event), plus a Token Event and a Dahan Event (Branch & Claw rulebook).

**Pool sizes** (active cards):

- Base: 0 (no Event deck)
- +B&C: +23, including Slave Rebellion, which is added only against France
- +JE: +30
- +NI: +9, including Far-off Wars Touch the Island, the replacement for the dropped War Touches the Island's Shores

Total all-expansions: 61 cards + Slave Rebellion with France. Nature Incarnate drops three Branch & Claw Events: Outpaced, War Touches the Island's Shores and A Strange Madness Among the Beasts (NI rulebook p.5; `open-questions.md` Q15).

See [Event Deck Risk Profiles](../statistics/event-deck-risk-profiles.md) for balance + swinginess.

### Event timing

Events resolve **early in the Invader Phase**: after any Blighted Island effect and before the Earned Fear Cards, so before the Ravage, Build and Explore (Branch & Claw rulebook). An Event that adds Invaders stacks with that turn's Build/Ravage.

## Cross-expansion dilution summary

| Deck | B&C swinginess (CV) | JE swinginess | NI swinginess |
|------|---------------------|---------------|---------------|
| Fear | **2.64** (highest) | **1.09** (lowest) | 2.12 |
| Event | **10.92** (highest) | 2.19 | **1.07** (lowest) |

Full analysis in [Expansion Dilution Claims](../statistics/expansion-dilution-claims.md).

## Data provenance

- Card data: `data/decks/fear.json`, `event.json`, `blight.json`.
- Parsing: `scripts/wiki-fetch.py` (MediaWiki API → `FearCardArticle` / `EventCardArticle` / `BlightCardArticle` templates).
- Analysis: `scripts/deck-stats.py` + `scripts/deck-simulate.py` + `scripts/expansion-deltas.py`.
