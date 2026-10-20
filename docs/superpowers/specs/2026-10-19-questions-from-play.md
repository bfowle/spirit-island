# The questions from play, and the screen each one needs

Source: Brett's own words during Test 1 (2026-10-18, Shadows vs Prussia L2,
boards C+E, played step by step with Claude) and the live Shadows vs
Prussia L6 game (2026-10-19, playtest U11–U16), plus the three coverage-game
answers. Dates and turns are from the session transcripts.

Every question was asked at a decision, about a plan or a choice. None was
"how much Blight is left". The models (Lumia's ledger, card counting, the
"if X is next" timelines, Rei's pace) are the engine under the answer; the
screen is the question.

## The questions, by moment

### Growth (asked 5 times)

| When | Brett |
|---|---|
| Test 1 T3 | "I could use a Defend (minor) card, for sure, especially C4 with 3 Dahan in there. Not much I can do about the other C1/E3 Ravages. Builds only in E1 (Disease in C2)." |
| Test 1 T3 | "A specific question/need: what % of getting a Minor-drawn Defend card to help in C4… is this worth it?" |
| Test 1 T3 | "Reclaim does NOT accelerate our growth/pace; postponing Reclaims is a massive strategy. Given the cards left (only 4 shown in the Minor pile), show the % chance of a Defend card, and if we don't get one, how helpful is that acceleration of growth." |
| Test 1 T3 | "What about factoring the 1–2 Fear cards we'll have and the Event card?" |
| Test 1 T4 | "G2 (gain a Major): the only Ravage we care about is 2 damage in C2 (Strife on the City). A great opportunity to get ahead on Builds since two land types Build this phase. Or Reclaim and gain a Minor/Major then?" |
| Test 1 T4 | "We need to do something about C7: 2 Explorers, City, Town; it Builds once more → 9 damage, and it has Blight. Also 5 Wetland/Mountain lands with a single Explorer all getting Towns." |
| Test 1 T4 | "Leaning Reclaim + Major; 5 Energy and +3 each round, so the economy is starting. Lure can save C2 this round." |
| Prussia 6 T3 | "1 Fear card banked, 1 Fear from a second (Concealing gets it). Another Power card, further down the Energy track toward a Major? Ravage in Wetlands, Builds in Jungle: all 4 lands do something." |
| Prussia 6 T4 | "We should Reclaim, 0 cards in hand." |

What answers it: per growth option, side by side, never ranked:
- **This Ravage** (ledger): Blight per land with and without the option's
  play; what each land needs (Defend N, or any effect that saves it).
- **The draw** (counting): of the unseen Minors/Majors, how many hold a card
  that saves the named land *and is playable now* (speed, cost, reach), as
  "about 27 of 96" plus the 1-in-N pocket rule. The need finder
  (`needs.rs`) does this.
- **The other unknowns** (counting): banked Fear cards that could save it
  (16 of 50 at Terror I; 23 of 49 at Terror II), the Event as a common shift.
- **And later** (pace): Energy, plays and presence by turn for this option,
  the Reclaim schedule, against the lands the deck hits per turn (the
  Curve's step). "Reclaim now vs later" is an order question, not a count
  question (spec 7.6).
- **What's coming at the lands** (timelines): which lands Build then Ravage
  within 2 turns (C7's 9 damage; the five single-Explorer lands), from the
  face-up cards and the next card's options.

### Presence placement (Brett, 2026-10-19, after the direction review)

"certain spirits WANT their presence in certain places, not just for targeting. e.g. if your unique powers are 0 range you want to be in with the thing you're targeting. or dahan (tspeaker) / animals (fangs). a lot of strategy comes from presence play."

Where the presence goes is a lever on the Growth screen, beside which track it comes off. What answers it, per candidate land, in game words:
- **Reach it opens**: which cards in hand (and in the discard, after a Reclaim) can now target which Ravage/Build lands, by each card's printed range and "from Sacred Site" rule; range-0 cards need the presence in the land itself.
- **The spirit's own rules**: Thunderspeaker's powers follow the Dahan, Sharp Fangs' follow the Beasts, Ocean's live in the Oceans; the special-rule text names it, and the land's Dahan/Beast counts say whether it holds.
- **Sacred Sites**: a second presence makes one, which innates and "from Sacred Site" cards use.
- **Risk**: Blight in that land destroys the presence (each Ravage ledger already says which lands Blight this turn).

Data: `PublicPowerCard.range`, `from_sacred_site`, `target`; `PublicInnate.range`; `PublicSpirit.special_rules`; land Dahan/Beast counts; the Ravage ledgers. Verify each spirit's placement rule against its printed panel before encoding it.

### Draft and Forget (asked 5 times)

| When | Brett |
|---|---|
| Test 1 T3 | "Lure actually solves C4 by gathering out. It's not a Defend card but it works for the Blight. Or Cycles of Time removes a Blight. What do we think?" |
| Test 1 T3 | "Lure also gives us Shadow elements; keeping on-element cards and using innates is massive." |
| Test 1 T4 | "If we let C2 kill our Presence this round and focus elsewhere, next turn Blazing Renewal has 2 destroyed Presence to hit C7." |
| Test 1 T4 | "Blazing if we want to start smashing (needs destroyed Presence, not easy for Shadows), or Spiders for Beast economy (Beasts come up in a lot of Events) and skips, but then we need Fear/damage later." |
| Prussia 6 T3 | "Dissolve replaces and pushes (pairs with Crops). Jungle Hungers: we'll never hit the threshold. Vanish: a plain Remove, no Fear." |
| Prussia 6 T3 | "What is the best card to Forget: Mantle, Drift, Crops? A Defend in only 2 land types, or two cards with +2 Fear?" |
| Prussia 6 T3 | "Present the tradeoff: with this Major we get X; when we Forget Y we lose this; the new card pairs with these cards in hand." And: "what a new card would accomplish this turn on the current board." |
| Prussia 6 T4 | "Picking between 4 Minors: a Defend 5 but off-element; Lure, entirely on-element." |

What answers it (U13): per candidate, three values side by side: what it
does on this board this turn (resolved at its speed against the forecast
board), what it pairs with (elements toward the innate's levels, same-land
combos; split hand-now vs discard), what each Forget costs (Fast
protection, elements, Fear). "Saves the land" means any effect, not the
Defend keyword (Lure, Cycles: Brett found this live).

### Fast phase: my plan vs this Ravage (asked 3 times)

| When | Brett |
|---|---|
| Test 1 T2 | "I plan Concealing Shadows in C3 (2 Dahan to protect and fight back, 4 damage to clear it) and Favors Called Due in C4 to outnumber for 3 Fear. How do we evaluate this plan for this round with what I have?" |
| Test 1 T2 | "Why is it Fear 5 instead of 4 this round? Because of the second board?" |
| Prussia 6 T5 | "Coastal Ravage, 2 lands already blighted for a cascade, into B4 which already has Blight. This is grim. What would the companion say here?" |

What answers it: the ledger with the plan applied (the what-if): per
Ravage land, damage after the plan, Blight yes/no, Dahan kept, Presence
kept, counterattack Fear; the Blight card after the turn and the chance of
losing at that depth (U16: 5 Blight → 76%); "B3 is worth 3 of the 5" and
the exact save for each land; what's still hidden (Fear cards, Event) and
how often it helps; rules in play that explain the numbers ("Fear is 5 per
card: two boards"; Sweden's Fine Steel; Habsburg's Fast-phase check).

### Event and Fear choices (asked 2 times)

| When | Brett |
|---|---|
| Prussia 6 T3 | "Beasts and Events are powerful; deciding where to put them is key." |
| Prussia 6 T2 (U9) | War Among the Dahan: which push decides whether B8's counterattack clears the land and how much Fear the turn earns. |

What answers it: the ledger rerun per legal choice (U9, U15): for each
land, what this choice does to this Ravage and what the unseen Events'
token sections would do there (counts from the public pool).

### After the game

"A retroactive turn-by-turn evaluation of what went wrong, which choices
were costly, and exactly what was on the board when each decision was
made" (spec 7.10): the same per-moment screens, replayed, with flags and a
luck ledger.

### Standing requirements, from the coverage games

- "These should show the ACTUAL things like Sweden's extra damage" (every
  adversary modifier applied, named in the ledger).
- "T3 four-land Ravage, didn't see that coming" (next turn's Ravage counted
  against the Blight that will be left, loud when it empties the card).
- "Not familiar with Habsburg Mining at all" (the rules in play explained
  where they bite, with when they're checked).
- "Augment, present things to think about; never make the call" (7.6).
- "A single screen, not much scrolling; I can't tell what it's trying to do
  to aid me" (U1–U3).

## How the models fit

| Model | Its job on the screen |
|---|---|
| Lumia's ledger (budget → inventory → shortfall) | Scores any plan or option at the coming step: Blight per land, what's left on the card, the exact save per land. It is the engine of every column, not a view of its own. |
| Card counting (hypergeometric on unseen pools) | The unknowns column: the draw, the banked Fear cards, the Event, the next Invader card, as "N of M" with a pocket rule. |
| "If X is next" timelines (exact enumeration) | What's coming at each land in 1–2 turns (Build → Ravage), for the Growth and plan-ahead questions. Not "nobody acts for four turns". |
| Rei's pace (presence-track projection vs the deck's step) | The "and later" column of each growth option: Reclaim now vs later, the track to take, the turn the economy outgrows the step. |
| Loss clocks and risk | The stakes line under the plan: the card after this turn, P(loss) at that depth, the adversary's clock and when it's checked. |

## What this means for the layout

One screen per moment, headed by the question of that moment, with the
player's plan or the options as the lever, in three columns: what's fixed,
what you control (the options, side by side), what's hidden (counted). Pace
is the last column of a growth option, not a chart of its own. The budgets
(Blight card, Fear, deck, clocks) are a narrow status strip, not the page.

Of what was built on 2026-10-19 (Ledger lanes, Runway, Pace, Curve,
what-if drawer), the core math carries over: `ledger.rs`, `outlook.rs`,
`pace.rs`, `curve.rs`, `whatif.rs`, `needs.rs`, `prompts()`,
`rules_in_play`, `loss_clocks`. The layout does not.

First screen to mock up: **Growth**, on Test 1 Turn 3 (the case Brett
worked by hand, spec 7.5's worked example), then **Fast: my plan vs this
Ravage** on Prussia 6 Turn 5 ("this is grim").
