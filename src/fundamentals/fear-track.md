# The Fear Track

Fear is the win-condition currency. Read this chapter to internalize how fear actually scales, when you should rush it, and when you should treat it as secondary to board control.

## How fear gets generated

Fear enters your pool from four sources:

1. **Destroying invaders** — Explorer/Town/City = 0/1/2 fear respectively, and only when the piece is *Destroyed* (by Damage or a Destroy effect). Removing, Pushing, Gathering, or Replacing (including Downgrading) a piece generates none.
2. **Fear-generating cards** — specific Minors, Majors, and some Uniques add fear directly.
3. **Innate powers and special rules** — Shadows (Darkness Swallows the Unwary), Bringer (Night Terrors), and Many Minds (Beset and Confound the Invaders) have Fear on innate tiers; Shroud's Slow and Silent Death special rule generates Fear during Time Passes.
4. **Events, scenarios, and other effects** — e.g., Beasts Events.

For most spirits, kills dominate fear generation. For fear-heavy spirits (Shadows, Bringer, Many Minds, Shroud), innates + cards can match or exceed kill-fear.

## How fear converts to wins

Each Fear Card costs **4 Fear per player**: the Fear Pool holds 4 Fear Markers per player, and when all of them have moved to Generated Fear, you earn the top card of the Fear Deck. Leftover Fear carries over toward the next card. England L6 and playing with an extra board each add 1 more Fear per player per card.

With the base Fear Deck (9 cards, split 3/3/3 by two Terror Level dividers), the totals are:

- **1 player**: 4 fear per card; 12 to reach Terror Level 2, 24 to reach Terror Level 3, 36 to empty the deck.
- **2 player**: 8 per card; 24 / 48 / 72.
- **3 player**: 12 per card; 36 / 72 / 108.
- **4 player**: 16 per card; 48 / 96 / 144.

Adversary levels change how many cards sit at each Terror Level, so recount for yours.

Reaching a divider raises the **Terror Level**, and each level loosens your victory condition:

- **Terror Level 1** (start): you win if *no Invaders* remain on the island.
- **Terror Level 2**: you win if *no Towns or Cities* remain. Explorers are OK.
- **Terror Level 3**: you win if *no Cities* remain. Towns + Explorers OK.
- **Fear Deck empty**: revealing the Victory space under the last card is an immediate win, whatever the board looks like.

Terror Levels *loosen* your victory condition. More fear = easier to win.

```admonish info title="Rules Clarification"
The Terror Level doesn't change per card. Each Fear Card is earned by filling the Fear Pool; the Terror Level changes the moment you earn the last card above a divider (even partway through a Power), and the new victory condition takes effect immediately, so you might win at once. Earned cards resolve during the next Invader Phase, each using the effect for the Terror Level current *when it resolves* — which can be higher than when it was earned.
(Core rulebook, "Fear and Terror"; [Querki FAQ — Fear and Fear Cards](https://querki.net/raw/darker/spirit-island-faq/Fear+and+Fear+Cards).)
```

## The fear-card stack

At setup, the Fear Cards are shuffled together and 9 go on the Fear Deck space, with the Terror Level 3 divider 3 cards from the bottom and the Terror Level 2 divider 3 cards above that (adversaries change those counts). There are no separate Terror-1/2/3 cards: every Fear Card prints three effects, one per Terror Level, and you use the line for the Terror Level current when the card resolves. This means:

- The same card resolves differently depending on when it is flipped, not when it was earned.
- Fear earned just before a divider is crossed resolves at the new, higher level.
- Each divider crossed also loosens the victory condition above.

Read the three lines on the Fear Cards you see and compare them; that comparison is your **fear quality curve**. Fear-rush builds aim to cross the dividers sooner, so more cards resolve at higher Terror Levels and the victory condition loosens earlier.

## When to rush fear

Fear is a viable primary win path when:

1. **Your spirit generates fear independent of kills** (Shadows, Bringer, Many Minds).
2. **The adversary's fear changes don't outrun you.** Count the Fear Cards your adversary level puts at each Terror Level, and read its fear-related rules (e.g., England L6 makes each card cost more; France L6 stops Fear Cards from removing Explorers).
3. **Board control is stable enough** to survive until you win on fear.

If any one of those fails, fear-rush fails. You end up at Terror Level 3 with Cities still on the board, and even the loosest board condition (no Cities) out of reach.

## When NOT to rush fear

- **Adversary levels with long Fear Decks** — the deck grows from the base 9 cards to 14 at several level-4-to-6 adversaries and to 16 at Scotland L6; that is up to 7 more cards' worth of Fear before the Fear Deck empties. Count the cards at each Terror Level on your level, not just the total.
- **England L6** — each Fear Card costs 1 more Fear per player, and any Invader Phase in which you resolve no Fear Cards performs the High Immigration Build twice.
- **France L6** — Fear Card effects never remove Explorers (you may Push them instead), which blunts every "remove Explorer" line.
- **Spirits with no innate fear generation + slow kill pace** — you'll never hit Terror 3.

## Fear-tempo vs. kill-tempo

Two common fear-generation shapes:

### Fear-tempo (heavy innate fear)

- Shadows: innate triggers generate ~1–3 fear per turn on threshold hits.
- Bringer: Night Terrors adds 1–3 Fear by tier, and their Powers generate 0/2/5 Fear instead of Destroying an Explorer/Town/City.
- Many Minds: Beset and Confound scales fear up to 6.

These spirits produce fear even without kills. They're also often worse at straight damage — they need fear as their closing condition because they can't clear cities fast.

### Kill-tempo (fear from bodies)

- Wildfire, Fangs, Lightning: most fear comes from kills.
- They clear cities fast; fear rises in parallel.
- Often over-kill so hard that they win on "no invaders" rather than Terror-3-unlocked-win.

**Heuristic**: match your opening to your spirit's shape. Don't try to fear-rush Fangs; don't try to board-wipe with Bringer.

## Damage ↔ fear exchange rate

A community heuristic from Phantaskippy's Discord-originated guides: when a Power offers you a *choice* between damage and fear, you can price the tradeoff. The rough rates that hold in most mid-game situations:

| Offer | Equivalent |
|---|---|
| +3 Fear | ≈ Push 1 Town out of a dangerous land (Town is 2 damage next Ravage + future Build participation) |
| +1 Fear | ≈ 1 damage to an Explorer (Destroy = 0 Fear by default; small Fear is about cheap board-state-change) |
| +2 Fear per Dahan retained | ≈ keeping a Dahan alive on a Ravage land (Dahan counter = 2 dmg + its next-Ravage 2 dmg) |
| Destroy a Town | = **1 Fear** (baseline) + whatever board-state payoff |
| Destroy a City | = **2 Fear** (baseline) + significant board-state payoff |

Use this to price choices like "2 Fear **or** 1 Damage": near the Terror-2 threshold on a low-damage turn, take the Fear; on a high-damage turn where the 1 damage finishes a Town (= 1 Fear automatic), take the Damage. The exchange rate is a sanity check, not a solve.

**Note**: per the [Explorer FAQ](https://spiritislandwiki.com/index.php?title=Explorer), destroying an Explorer produces 0 Fear by default — that's why +1 Fear is "priced" above a naked Explorer-destroy. Powers that say "1 Fear per Explorer destroyed this Power" are *bonus* to the 0 baseline.

Source: Phantaskippy's [Rampant Green](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide) and [Bringer](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares/Phantaskippy%27s_Guide) guides. Fear values per Destroy: core rulebook, "Fear and Terror".

## The timing question

When does fear you generate "count"? Immediately. The moment the Fear Pool empties you earn a card, and if that card is the last one above a divider, the Terror Level changes right then — not at Time Passes. This means:

- Fear that crosses a divider during the Fast phase of T3 makes the Terror Level 2 victory condition live for the rest of T3: the Invader Phase, the Dahan counterattack, and your Slow powers.
- Earned cards are flipped and resolved in the next Invader Phase's Fear step, at the Terror Level current then.

For "I need Terror 2 to win on this kill" plays, the fear that crosses the divider only has to land *before* the kill, not before the turn ends. Players choose the order in which their Powers resolve, so ask: can the Fear resolve first?

## Fear-card quality (what you actually draw)

The Fear Card pool is 50 cards with every expansion (the count in the Steam game's allowed pool); a game uses 9 plus your adversary's changes, so most of the pool stays in the box each game. See [Fear Card Expected Value](../statistics/fear-card-expected-value.md) for the per-card analysis.

## Fear + scoring

Fear isn't part of the score. The rulebook's optional scoring uses difficulty, a win bonus, the Invader Cards remaining (on a win) or used (on a loss), living Dahan, and Blight on the island. A fear-heavy loss scores no better for the fear alone.

## Cross-references

- [Fear Card Expected Value](../statistics/fear-card-expected-value.md) — per-card data.
- [Fear Rush](../combos/fear-rush.md) — archetype build.
- [Blight Track](./blight-track.md) — the other win/loss axis.
- Specific fear-heavy spirits: [Shadows](../spirits/low/shadows-flicker-like-flame.md), [Bringer](../spirits/low/bringer-of-dreams-and-nightmares.md), [Many Minds](../spirits/moderate/many-minds-move-as-one.md), [Shroud](../spirits/moderate/shroud-of-silent-mist.md).

---

*Last revised: 2026-10-19*
