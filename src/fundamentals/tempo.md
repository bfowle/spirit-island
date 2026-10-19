# Tempo

Tempo is the implicit currency that makes or breaks every Spirit Island game. You never see it on a card or a track — you feel it when your board collapses on T4 because you grew too slowly on T1, or when you win on T7 because you banked the right turn on T3.

This chapter gives you the vocabulary and the heuristics to reason about it deliberately.

## What tempo actually is

"Tempo" gets used for two different things. Keep them apart:

- **Land tempo** — where each land sits in the Invader pipeline. Every land is somewhere on Explore → Build → Ravage: an Explore card this turn means a Build there next turn and a Ravage the turn after (see [Invader Lifecycle](./invader-lifecycle.md)). Land tempo is the Invaders' clock, land by land, and you read it off the Invader board.
- **Economy momentum** — how fast your own engine grows: your per-turn Energy, Card Plays, and presence on the board.

Economy momentum has three interacting axes:

1. **Energy** — how fast your per-turn energy grows.
2. **Card plays** — how fast your per-turn CP grows.
3. **Presence** — how fast you occupy the board, enabling targeting and uncovering your tracks.

The goal isn't to maximize all three. It's to **keep your economy ahead of the land tempo** at every round: enough Energy, Card Plays, and reach to deal with the lands about to Build and Ravage. Fall behind and the lands get away from you; go *far* ahead and you over-invested in growth when you should have been stopping ravages.

## The adversary's pressure curve

Each adversary bends the land tempo — the rising line of "how much board effect you need to deliver this turn to not fall behind." What each one actually changes (levels are cumulative):

- **Brandenburg-Prussia** — *a compressed deck*. L1 adds a Town to land #3 on each board at Setup. L2+ moves one Stage III card to just after Stage I, so a two-terrain card is Explored on Turn 3 at L2; L3 and L5 each remove a Stage I card, pulling it earlier, and L6 has no Stage I at all. The deck also shrinks (12 cards at L2, 8 at L6), so the Time-runs-out clock comes sooner. Escalation (Land Rush) adds a Town on each board with Towns/Cities, before the Explore.
- **England** — *extra Builds*. Escalation (Building Boom) Builds in the land with the most Towns/Cities on each board; L1+ Builds in lands without Invaders next to 2+ Towns/Cities; L3+ adds a second Build slot (High Immigration) that Builds before each Ravage. Loss condition: 7+ Towns/Cities in a single land.
- **Sweden** — *heavier Ravages*. L1+ adds an extra Blight on a Ravage with 4+ overkill; L3+ Towns deal 3 Damage and Cities 5; L5+ adds a Town next to any land a Ravage Blights. Escalation replaces Dahan with Towns after the Explore.
- **Russia** — *Explorer pressure*. L1+ Explorers deal +1 Damage and Ravage Blight Destroys Beasts (feeding the Hunters Swarm loss condition); L2+ the first Explorer each Action would Destroy is Pushed instead, for 1 Fear; L3+ Ravage cards also match lands with 3+ Explorers; L4+ puts a Stage III card after each Stage II card. Escalation adds 2 Explorers among lands with Beasts on each board.

Your opening choice should be a bet on your spirit's economy momentum keeping up with this land tempo, not on the spirit's "power level" in abstract.

## The three economy axes, one at a time

### Energy tempo

From the Steam engine's presence-track data, 19 of the 34 Steam spirits open at **1E per turn**, 10 at 0E (e.g., Shadows, Wildfire, Ocean, Many Minds), and 5 at 2E (Bringer, Devouring Teeth, Keeper, Stone, Vital Strength). Read your own panel's leftmost Energy space.

What raises energy tempo:
- Growth slots that give +energy.
- Presence track revealing passive energy.
- Powers that generate energy (e.g., Let's See What Happens for Trickster).
- Spirit-specific generation (e.g., Shroud's damaged-invader bonus).

**Heuristic**: if you end a turn with more than 2 energy banked, you likely under-spent last turn. Exception: you're saving for a high-cost card next turn.

### Card-play tempo

From the same track data, 27 of the 34 Steam spirits open at **1CP** and 7 at 2CP (Breath of Darkness, Bringer, Dances Up Earthquakes, Trickster, Lightning, Fangs, Starlight). CP scaling is the tightest bottleneck past T3 for almost every spirit.

What raises CP:
- Growth slots that unlock +CP.
- Reclaim — not a CP gain per se, but enables cycling the same 2 cards forever.
- Spirit-specific track spaces, where your panel shows them.

**Heuristic**: if you're not playing your maximum CP every turn past T3, you've over-invested elsewhere. Holding cards is almost never correct for most spirits.

### Presence tempo

Presence does three things at once — targeting, sacred sites, and uncovering your tracks. Tempo here is subtler.

What raises presence tempo:
- Growth slots that place presence.
- Moving existing presence (Shroud's Mists Shift and Flow; Shadows' Amorphous aspect; Fangs' presence moving with Beasts; Thunderspeaker's with Dahan).

**Heuristic**: Invaders never damage presence; your count usually falls because Blight added to one of your lands (or another effect) Destroys it. If the count is falling, ask which Ravages Blighted your lands and whether the land tempo showed them coming.

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
- If your adversary's L6 win rate on your spirit (per mindwanderer) is under 30%, your tempo assumption needs to be more aggressive than the default.
```

## Cross-references

- [Presence Economy](./presence-economy.md) — the deep dive on the presence axis.
- [Energy Curves](./energy-curves.md) — per-spirit target energy curves.
- [Major vs. Minor](./major-vs-minor.md) — tempo implications of card-draft choice.
- [The Fear Track](./fear-track.md) — tempo of fear generation.

```admonish abstract title="Sources"
- latentoctopus glossary (Gn, nE, nCP notation): https://latentoctopus.github.io/glossary/
- Land tempo vs. economy momentum distinction: lumia's Spirit Island glossary, https://iam.lumia.gay/si_glossary
- Adversary effects: the project rules register, `data/references/rules/register.md` §4 (engine `adversaries.json` cross-checked with the Wiki).
- Starting Energy/CP counts: the Steam engine's `spirits.json` presence tracks.
- Rei on Vengeance's late-game juggernaut scaling (a canonical tempo-arc example): BGG thread #2709070.
- mindwanderer stats for per-spirit tempo shape validation: https://mindwanderer.net/si/stats.html
```

---

*Last revised: 2026-10-19*
