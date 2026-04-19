# Tempo

Tempo is the implicit currency that makes or breaks every Spirit Island game. You never see it on a card or a track — you feel it when your board collapses on T4 because you grew too slowly on T1, or when you win on T7 because you banked the right turn on T3.

This chapter gives you the vocabulary and the heuristics to reason about it deliberately.

## What tempo actually is

Tempo is **the rate at which you convert currencies into board effect per turn**.

There are three interacting tempo axes:

1. **Energy tempo** — how fast your per-turn energy grows.
2. **Card-play tempo** — how fast your per-turn CP grows.
3. **Presence tempo** — how fast you occupy the board, enabling targeting + damage sinks.

The goal isn't to maximize all three. It's to **keep all three above the adversary's pressure curve** at every round. Go below the curve and you lose; go *far* above the curve and you over-invested in growth when you should have been stopping ravages.

## The adversary's pressure curve

Each adversary produces a pressure curve — a rising line of "how much board effect you need to deliver this turn to not fall behind." The shape varies:

- **Brandenburg-Prussia** — *flat-early, steep-late*. T1–2 are calm; T5+ has Cities bombarding coasts.
- **England** — *gradual rise, long tail*. Towns accumulate steadily; late-game they become Cities everywhere.
- **Sweden** — *high-early, plateau*. You must defend hard T2–3; if you survive, scaling matters less.
- **Russia** — *dahan-pressure, fear-blocked*. Kills dahan while suppressing your fear gain.

Your opening choice should be a bet on your spirit's tempo matching this curve, not on the spirit's "power level" in abstract.

## The three tempo axes, one at a time

### Energy tempo

Most spirits open at **1E per turn**. Some (Lightning) at 2E+, some (Fangs, Wildfire) well below effective income due to cost-heavy cards.

What raises energy tempo:
- Growth slots that give +energy.
- Presence track revealing passive energy.
- Powers that generate energy (e.g., Let's See What Happens for Trickster).
- Spirit-specific generation (e.g., Shroud's damaged-invader bonus).

**Heuristic**: if you end a turn with more than 2 energy banked, you likely under-spent last turn. Exception: you're saving for a high-cost card next turn.

### Card-play tempo

Most spirits open at **2CP** (sometimes 1CP). CP scaling is the tightest bottleneck past T3 for almost every spirit.

What raises CP:
- Growth slots that unlock +CP.
- Reclaim — not a CP gain per se, but enables cycling the same 2 cards forever.
- Rare spirit-specific mechanics (Many Minds' special rule; Fractured Days' engine).

**Heuristic**: if you're not playing your maximum CP every turn past T3, you've over-invested elsewhere. Holding cards is almost never correct for most spirits.

### Presence tempo

Presence does three things at once — targeting, sacred sites, damage absorption. Tempo here is subtler.

What raises presence tempo:
- Growth slots that place presence.
- Spirit-specific mechanics (Fangs' Beasts, Many Minds' Beasts-as-presence).
- Moving existing presence (Shroud, Shadows).

**Heuristic**: your presence count usually peaks around T3–T4, then gradually decays as ravages destroy it. If your presence count rises past T5, you're over-growing.

## Banking vs. spending

Every turn, you choose between **banking** (saving a resource for next turn) and **spending** (deploying it now). The banking question is the hardest strategic question in the game.

### When to bank

- You have a high-cost play coming up next turn that solves a bigger problem.
- The adversary's immediate pressure is low (T1 standard-deck is often forgiving).
- Your spirit has a reclaim coming that would waste the cards you'd play now.

### When to spend

- A ravage is about to hit and you have the cards to prevent or absorb.
- Elements this turn cross an innate threshold you'll lose access to next turn.
- Fear payout — "I generate 3 fear this turn or I don't" is a now-or-never.

### The most common banking mistake

> "I'll hold this fast power for when the ravage is worse."

Usually wrong. The ravage is worst *when it happens*. If you can prevent or mitigate now, do it; the next worst ravage is worse than the current one, but you'll have more resources then.

```admonish failure title="Common Mistake"
Holding cards in hand past T3 because "I might need them." Unless you're explicitly planning a reclaim, held cards are under-deployed tempo. Play your CP max.
```

## The tempo cliff

Every spirit has a **cliff turn** — the turn where, if you haven't set up properly, you collapse. Examples:

- **Wildfire**: T4 collapse if you don't have 2 Fire + 3 Plant by end of T3.
- **Shadows**: T5 collapse if you don't have a reliable damage pipeline online.
- **Bringer**: T3 collapse if you don't have the fear loop running.
- **Ocean**: T4 collapse if you haven't established coastal drowning by T3.

A spirit chapter's **Tempo Profile** table shows the expected state per round. Diverge at your peril — or knowingly, as a read.

## Tempo vs. elements

Elements are not free. Gaining elements via growth costs you presence-placement or CP-gain. Element-rich plays cost cards.

The trade:
- **High-element growth** — you sacrifice presence for faster innate threshold activation. Great for spirits with huge innate payoffs (Shadows, Lightning).
- **Low-element growth** — you sacrifice innate power for presence + CP. Great for spirits whose cards carry the load (Thunderspeaker, some Earth openings).

## Per-turn tempo audit (a drill)

After each game, ask:

1. **Which turn was my peak?** If not T6–T8, you may have spiked too early or late.
2. **Which turn was my weakest?** Was it planned (reclaim, growth-heavy) or forced (collapse)?
3. **Did I bank on a turn I shouldn't have?** — count banked energy > 2 as a signal.
4. **Did I spend on a turn I shouldn't have?** — check if you forced a threshold activation that cost a card you wanted later.
5. **Did I misread the adversary's pressure curve?** — which turn did it surprise you?

Log the answer in [`si-post-game`](../appendices/claude-skills-guide.md). Across 10–15 games, tempo errors become the pattern you see, and you fix them.

## Summary heuristics

```admonish tip title="Pro Tip"
- If you bank > 2 energy into next turn, you underspent. Play the card.
- If you hold cards past T3 without a reclaim plan, you under-CP'd. Play them.
- If your presence is still rising past T5, you over-grew. Shift to card plays.
- If your adversary's L6 win rate on your spirit (per mindwanderer) is under 30%, your tempo assumption needs to be more aggressive than the default.
```

## Cross-references

- [Presence Economy](./presence-economy.md) — the deep dive on the presence axis.
- [Energy Curves](./energy-curves.md) — per-spirit target energy curves.
- [Major vs. Minor](./major-vs-minor.md) — tempo implications of card-draft choice.
- [The Fear Track](./fear-track.md) — tempo of fear generation.

```admonish abstract title="Sources"
- latentoctopus glossary (Gn, nE, nCP notation): https://latentoctopus.github.io/glossary/
- Rei on Vengeance's late-game juggernaut scaling (a canonical tempo-arc example): BGG thread #2709070.
- mindwanderer stats for per-spirit tempo shape validation: https://mindwanderer.net/si/stats.html
```

---

*Last revised: 2026-04-19*
