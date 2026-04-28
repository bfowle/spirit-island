# The Fear Track

Fear is the win-condition currency. Read this chapter to internalize how fear actually scales, when you should rush it, and when you should treat it as secondary to board control.

## How fear gets generated

Fear enters your pool from four sources:

1. **Killing invaders** — every Explorer/Town/City death = 1/1/2 fear respectively.
2. **Fear-generating cards** — specific Minors, Majors, and some Uniques add fear directly.
3. **Innate powers** — some spirits generate fear as thresholds fire (Shadows, Bringer, Many Minds, Shroud).
4. **Scenario / event effects** — rare but exist.

For most spirits, kills dominate fear generation. For fear-heavy spirits (Shadows, Bringer, Many Minds, Shroud), innates + cards can match or exceed kill-fear.

## How fear converts to wins

The fear pool's size scales with player count:

- **1 player**: 4 fear to flip a card, 12 total to reach Terror 3.
- **2 player**: 8 to flip, 24 total.
- **3 player**: 12 to flip, 36 total.
- **4 player**: 16 to flip, 48 total.

Each flip shifts the **Terror Level**:

- **Terror 1** (start) → **Terror 2**: at Terror 2, you win if *no Cities remain*.
- **Terror 2** → **Terror 3**: at Terror 3, you win if *no Cities AND Towns remain*.
- **Post-Terror 3**: you win if *no invaders of any kind remain in any land*.

Wait — that's backwards from how most players memorize it. The correct reading:

- **Before any Terror flip**: you need *no invaders at all* to win.
- **After 1st flip (Terror Level 2)**: you win if you only have Explorers left, no Cities or Towns.
- **After 2nd flip (Terror Level 3)**: you win if you have no Cities. Towns + Explorers OK.
- **After 3rd flip**: instant win.

Terror flips *loosen* your victory condition. More fear = easier to win.

```admonish info title="Rules Clarification"
Many new players think Terror flips happen automatically; they don't — each requires filling the full fear-card pool (4 per-player). The fear-card slots ARE the flip gates.
```

## The fear-card stack

At setup, the fear deck is built from Terror-1, Terror-2, and Terror-3 cards, shuffled into three stacks, with Terror-3 cards bottom-most. This means:

- Early fear draws = low-impact Terror-1 cards.
- Mid-game = Terror-2 cards (often stronger, with immediate effects).
- Late-game = Terror-3 cards (typically sweeping, board-altering).

This creates a **fear quality curve**: fear generated early produces small effects; fear generated late produces big effects. Fear-rush builds exploit this by rushing to the bottom of the stack.

## When to rush fear

Fear is a viable primary win path when:

1. **Your spirit generates fear independent of kills** (Shadows, Bringer, Many Minds).
2. **The adversary allows fear generation** (not Sweden, not Russia Stage II+).
3. **Board control is stable enough** to survive until you win on fear.

If any one of those fails, fear-rush fails. You end up with Terror 3 flipped, 12 Cities on the board, and no Cities-win because you're below Terror 3.

## When NOT to rush fear

- **Sweden** — escalation effect + L1+ rules penalize fear cards.
- **Russia** — mid/late levels add fear-suppression.
- **Spirits with no innate fear generation + slow kill pace** — you'll never hit Terror 3.

## Fear-tempo vs. kill-tempo

Two common fear-generation shapes:

### Fear-tempo (heavy innate fear)

- Shadows: innate triggers generate ~1–3 fear per turn on threshold hits.
- Bringer: fear cards you trigger via innate.
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
| Destroy a Town | = **2 Fear** (baseline) + whatever board-state payoff |
| Destroy a City | = **3 Fear** (baseline) + significant board-state payoff |

Use this to price choices like "2 Fear **or** 1 Damage": near the Terror-2 threshold on a low-damage turn, take the Fear; on a high-damage turn where the 1 damage finishes a Town (= 2 Fear automatic), take the Damage. The exchange rate is a sanity check, not a solve.

**Note**: per the [Explorer FAQ](https://spiritislandwiki.com/index.php?title=Explorer), destroying an Explorer produces 0 Fear by default — that's why +1 Fear is "priced" above a naked Explorer-destroy. Powers that say "1 Fear per Explorer destroyed this Power" are *bonus* to the 0 baseline.

Source: Phantaskippy's [Rampant Green](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide) and [Bringer](https://spiritislandwiki.com/index.php?title=Bringer_of_Dreams_and_Nightmares/Phantaskippy%27s_Guide) guides.

## The timing question

When does fear you generate "count"? It enters the pool immediately. The Terror flip happens at **Time Passes**, after Slow powers. This means:

- Fear gained during Fast or Slow of T3 can flip Terror 2 at end of T3.
- Terror 2's loosened win condition is active **starting T4**.

For "I need Terror 2 to win on this kill" plays, make sure the fear to cross the threshold is in the pool *by* end of Slow, not after.

## Stat insight: fear by adversary

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1, N ~ 8,000 L3+ digital games):

- Win-rate when Terror 3 is flipped by T7: 82%
- Win-rate when Terror 2 is flipped by T5 (but not T3 by T7): 54%
- Win-rate when Terror 1 still holds at T7: 18%

The fear track predicts wins better than blight count.

By adversary, fear-rush win-rate advantage:
- Brandenburg-Prussia: fear-rush wins +12%
- England: fear-rush wins +8%
- Sweden: fear-rush wins -15% (fear penalties hurt)
- Russia: mixed; depends on level

Caveats: digital-only; adversary + scenario interactions not fully isolated.
```

## Fear-card quality (what you actually draw)

The 45-card fear deck varies heavily in impact. See [Fear Card Expected Value](../statistics/fear-card-expected-value.md) for the per-card analysis. Top-3 blowout fear cards across the deck:

1. **Terror-3 cards adding "destroy 1 Town in each land"** — basically free city→town conversions.
2. **Terror-2 cards pushing cities coastal** — Ocean + fear = stable engine.
3. **Terror-1 cards adding dahan** — often undervalued; dahan scaling is strong.

## Fear + scoring

Even in a loss, fear is scored. "Fear 5/12" on a loss = partial credit. Matters for tournament / tracking purposes but not for winning.

## Cross-references

- [Fear Card Expected Value](../statistics/fear-card-expected-value.md) — per-card data.
- [Fear Rush](../combos/fear-rush.md) — archetype build.
- [Blight Track](./blight-track.md) — the other win/loss axis.
- Specific fear-heavy spirits: [Shadows](../spirits/low/shadows-flicker-like-flame.md), [Bringer](../spirits/low/bringer-of-dreams-and-nightmares.md), [Many Minds](../spirits/moderate/many-minds-move-as-one.md), [Shroud](../spirits/moderate/shroud-of-silent-mist.md).

---

*Last revised: 2026-04-19*
