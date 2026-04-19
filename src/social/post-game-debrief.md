# The Post-Game Debrief

The game ends. You won, or you lost. The pieces are on the table; the terror cards are face-down. The next 60 seconds determine whether the session was a good experience for everyone, whether anyone learned, and whether you'll play together again.

Get this wrong and the lesson-of-the-session is "Spirit Island games are unpleasant post-mortems." Get it right and losses become data, wins become confidence, and the table wants a rematch.

## The three-sentence debrief

The ideal debrief is **three sentences total**, distributed across the table. Not per player; *total*.

**Sentence 1 (the result)**: what happened objectively.

> "We lost to blight cascade on T7."

**Sentence 2 (one learning)**: the single most load-bearing thing someone (anyone) saw.

> "I think not killing the Town in jungle on T3 was the mistake."

**Sentence 3 (the next move)**: rematch, new config, or end.

> "Want to play again, slightly easier this time?"

That's it. Three sentences. If someone wants more analysis, they'll ask.

## Why short

Most debriefs are too long. Long debriefs:

- Bury the newer player under information they can't use yet.
- Make the loser feel scrutinized.
- Extract "what should we have done?" when nobody knows.
- Devolve into rehashing turns.

The three-sentence rule is a hard limit. Expand only when asked.

## Losing gracefully

### If the loss was close

- Acknowledge the close call.
- Don't pin blame on a single moment — Spirit Island loses compound from many small decisions.
- If someone offers "I could have played X differently," let them own it; don't amplify.

### If the loss was bad

- Don't say "that was bad." Say "that got away from us."
- Skip to "want to try again?" quickly.
- Let everyone breathe.

### If the loss was confusing

- "I didn't fully follow the last two turns."
- "Want to check one rule before we play again?" — normalizes consulting FAQ post-game.

## Winning gracefully

### If the win was close

- Celebrate the moment: "Turn 8 was incredible."
- Attribute contributions to specific players: "That defend on T5 is why we had room."
- Offer rematch at same difficulty; push up only if the group pushes.

### If the win was easy

- Don't under-sell the win: "We absolutely handled that."
- But note: "Want to bump adversary next game?"
- Don't say "that was too easy" — it implies the group was over-leveled for the table, which can feel condescending.

### If you carried

Don't say "I carried." Even if true. The table remembers. Attribute to teamwork.

## The "what would I do differently?" question

If someone asks:

- **If it's the newer player asking**: answer concretely for their spirit, 1–2 sentences.
- **If it's the veteran asking aloud for discussion**: the group can engage; keep to 3–4 exchanges.
- **If you're asked by someone else**: answer short, then redirect. "T5 defend, I think. What did you notice?"

## Never-do-list

### Don't debrief a loss that wasn't yours

If partner spirit made a clearly-suboptimal call, *don't bring it up*. They know. They saw it. Telling them is rubbing salt.

### Don't debrief a new player's turn

They're still processing. If they want to discuss, they'll open the topic. Your job is silence + support.

### Don't start with "you know what you should have done..."

Always. Always. Pure anti-pattern.

### Don't debrief while the board is still up

If debrief goes past 2 minutes, start resetting the board. The physical act of resetting signals "we're past this game."

## The debrief for different audiences

### With a veteran partner

Longer debriefs are fine; both can engage in analysis. Still cap at 5 minutes or so.

### With a teachmate / new player

Three sentences. Extract what they observed first, not what you observed. "What felt hard / easy / surprising?"

### At a convention / stranger table

Shortest possible. Strangers don't owe you a post-mortem. "Good game, thanks!" is complete.

### Solo multi-handed

A deliberate self-debrief is useful:

- "What turn was my best?"
- "What turn was my worst?"
- "What would I do differently if I ran this again?"
- Log to `data/playlog.csv` via [`si-post-game`](../appendices/claude-skills-guide.md) skill.

## Feeding the debrief into growth

After the table breaks, if you want to learn from the game:

1. **Run `si-post-game`** — logs the game + surfaces 1–2 learning points.
2. **Note what diverged from chapter advice** — was the Shadows opening off because of an adversary-specific reason not captured in the chapter?
3. **If an entry in the chapter's Common Mistakes section actually cost you the game**: note it for revision in `data/references/` or open an issue.

The book is living. Debriefs are how it gets updated.

## Emotional register

The debrief's tone matters more than the content.

- **Neutral > enthusiastic** for losses. "We lost" is fine. "Heart-breaking loss!" is performative.
- **Specific > general** for wins. "Your defend on jungle T5 was the play" beats "you did great."
- **Questions > statements** when someone else is processing. "What surprised you?" opens.
- **Non-judgmental tone always**. Spirit Island is hard; mistakes are universal.

## When to skip the debrief entirely

Sometimes:

- The game was miserable and extracting "one learning" would be cruel.
- Someone is clearly drained.
- The session is over and everyone wants to leave.

Skip it. "Good game" is a complete debrief.

## The debrief ritual for regular partners

If you play regularly with the same partner:

1. **Track collective progress** — 10th win vs. England is worth celebrating.
2. **Rotate who debriefs first** — gives both voices space.
3. **Occasionally skip** — not every game needs analysis.
4. **Log to a shared place** — the playlog in this book's `data/playlog.csv` works for solo; for regular partners, shared Google Sheet or BGG play logs.

## Common mistakes

```admonish failure title="Common Mistake"
Over-analyzing a loss. Five minutes of "what if we had..." teaches nothing — losses compound from many sources; rarely one fixable moment.
```

```admonish failure title="Common Mistake"
Debriefing the new player's turns without invitation. Let them drive the learning pace.
```

```admonish failure title="Common Mistake"
Saying "that was a bad play" to partner's face. The play might have been bad; the critique in public is always worse.
```

```admonish failure title="Common Mistake"
Skipping the debrief on a win. A 30-second win-debrief locks in learning for the next session; silence doesn't.
```

```admonish failure title="Common Mistake"
Extending the debrief into the next game's prep. Close the previous game fully before starting the next.
```

## Cross-references

- [Teaching Methods](./teaching-methods.md) — pre-game framing.
- [Alpha-Player Problem](./alpha-player-problem.md) — debrief without alpha-ing.
- [Mixed-Skill Tables](./mixed-skill-tables.md) — audience-specific debriefs.
- [`si-post-game` skill](../appendices/claude-skills-guide.md) — solo structured debrief.

---

*Last revised: 2026-04-19*
