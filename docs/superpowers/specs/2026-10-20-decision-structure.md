# The game as a decision structure

Status: written 2026-10-20 from the conversation with Brett ("the graph theory around
DAG decisions the player makes, all the various mechanics working this way, forming
multiple DAGs all hitting multiple paths: what kind of graph theory mostly makes sense
for this? should we restructure everything we're doing around this?"), with his
sidenote on the Invader deck doubling up between stages. Companion to
`2026-10-20-hindsight-analysis.md` (what hindsight computes) and
`2026-10-20-notation.md` (how a line is written).

## 1. What the structure is

The structure is not general graph theory. It is an **extensive-form game tree with
chance nodes**, which for a one-player game with no hidden opponent is a **Markov
decision process**. Spirit Island's hidden information is only the deck order, and
the deck compositions are known (the Invader deck by stage, the Fear and Event decks
by card pool), so the state including the remaining multisets is fully observed.
Layer the states by turn and phase and the state graph is a DAG, since time never
goes back. Player decisions and deck reveals alternate as node types:

| Node | Who moves | Edges out |
|---|---|---|
| decision | the player | growth group and slots, where presence went, which cards, each Power's target and inner choices, Event and Fear land choices |
| chance | the decks | the Explore card (uniform over the stage's remaining cards), the Event, the Fear cards, the Power draft |
| transition | the rules | deterministic: Ravage, Build, Explore placement, cascades, counterattacks, Fear thresholds |

The notation spec already splits a line into decision, reveal and result; those are
exactly decision node, chance node and transition. The record (`si-record`) is one
path through the tree. `si-rules` is the transition function. The ledger is a static
evaluation at the horizon.

What applies:

- **Expectimax**: decision node, chance node, evaluate at a horizon. Hindsight today
  is a one-ply expectimax, lines times Explore cards, horizon 1.5 turns.
- **MCTS with chance nodes**: the practical search for this branching factor. Growth
  times placements times card subsets times targets times Fast ordering is far too
  wide for full expectimax at depth.
- **Information sets and determinization**: the informed score evaluates a line at
  the information set by averaging over determinizations of the hidden card. The
  hindsight score evaluates it at the one true leaf. Perfect-information Monte Carlo
  has a known flaw, strategy fusion: a plan fixed before the reveal is scored as if
  it could not adapt after it. Slow targets were exactly that flaw here (section 3c).
- **The within-turn dependencies** (presence enabling a Sacred Site enabling a
  target, elements enabling an innate tier, a Fast push changing what a Slow damage
  finds) are a constraint DAG, not a search problem. Enumerate in dependency order
  and prune; `candidate_plays` already orders presence-placing cards first and carries
  the presence forward.

Restructure? No. What is missing is a thin explicit interface over the three pieces
that already exist: a State node, `legal_actions(state, phase)`, `step(state, action)`
and `chance_outcomes(state)` with probabilities. Hindsight's enumerate-then-simulate,
a what-if parser for the notation and any deeper search would all sit on it. That is
a refactor of `si-hindsight`'s enumeration into `si-rules`, not a rewrite of the
model (section 5). The real ceiling is the evaluation function and the horizon, not
the graph.

## 2. Hindsight minus informed is luck: a borrowed model

The decomposition is standard decision theory: judge a choice by its expected value
on the information at decision time, not by what came. The working ancestors:

| Where | Error (our `decision`) | Luck (our `luck`) |
|---|---|---|
| backgammon analysis (GNU Backgammon, eXtreme Gammon) | best move's equity minus the chosen move's | equity after the roll minus the average over all 36 rolls |
| poker trackers | – | winnings minus the all-in EV line |
| chess engines | centipawn loss (no chance half) | – |
| decision theory | regret against the best option in the set | realized minus expected value |

Confidence (the share of chance outcomes under which one option beats the other) is
the **probability of superiority**, also called the common-language effect size. It
weighs every Explore card equally, which is right for a shuffled stage. As a plain
count over the Explore cards it degenerated to 0%, 100% or nothing (Brett, 2026-10-20:
"massively off"): the Explore card rarely flips which of two Fast lines is better, so
one line won under every card. Now the count is smoothed by the simulator's measured
error (`model_sd`, section 3d): under each card the chance that the lead outweighs two
scores each off by that much, averaged over the cards. A lead of one ledger point reads
about 70%, of three about 93%. The Event and Fear cards do not enter it: they are
applied identically to every line, so under the model their difference has no variance,
which understates the uncertainty of lines that leave different boards for an Event to
act on.

What is ours, and was weak as first built:

a. **The ledger** is a hand-built static evaluation, like a material count.
   Backgammon equity is a win probability. Ours is in made-up units, so both luck
   and error are in those units.
b. **Informed zeroed the Event and Fear effects** instead of averaging them. In
   informed mode the recorded choice effects and the residual delta were skipped; in
   hindsight they were applied from the log. So the luck column held the whole
   Event/Fear effect of the turn, not its deviation from what a player should have
   expected. That is a bias, not luck.
c. **Slow targets were fixed per line before the Explore.** The real player picks
   them after the Invader steps. Lines whose Slow play would have adapted were
   undersold, and the actual line's informed score under another Explore card used a
   target chosen for the card that came.
d. **The 0.5 next-turn weight and the 0.5 / 1.5 reading bands were arbitrary**, like
   chess's inaccuracy/mistake/blunder cutoffs.

## 3. Strengthening each (built 2026-10-20)

### 3a. The ledger: measure before refitting

The corpus is 16 captures plus bare logs, too few games to fit eight weights by
logistic regression without overfitting. The step taken is to measure the ledger
against results: `si calibrate` prints, for every game, the sum of the actual lines'
ledger scores and the result, and the separation between wins and losses (means and
the share of win/loss pairs the ledger orders correctly): 66% of pairs at first, 72%
once the Build's Towns and Cities count against a line (section 3e). When the corpus
passes about a hundred games the weights get fitted to win probability; until then
the ledger stays as printed and the measurement says how far to trust it.

Blight already has a loss-clock meaning that the flat weight misses: a Blight when the
pool is nearly empty ends the game. The pool-aware term is deferred to the fit; the
LINES table marks lines whose Blight reaches the pool (`FLIPS`, `LOSES`).

### 3b. Informed averages the Event and Fear decks

The decks' compositions are known but their effects are not typed (62 Events, 50 Fear
cards at three Terror Levels), so the expectation is measured, not simulated. For
every turn of every corpus game the **hidden effect** on the actual line is its
hindsight score (best Slow targets) minus its informed score under the Explore card
that came: everything the face-down cards did beyond the Explore, pieces and status
effects (Defends, skips) alike, the next turn's cards inside the lookahead included.
`si calibrate` fits it by least squares to an Event indicator and the Fear cards
resolved per Terror Level (no intercept: a turn with neither has no hidden effect by
definition). The first attempt averaged the piece delta alone and left a +0.64 mean in
luck, the Fear cards' Defends; the fit over the whole effect centers luck at +0.03.

The informed score of every line adds `event_mean` when the turn resolves an Event
(from turn 2 on) and `fear_card_mean[TL]` per Fear card the turn resolved (TL as the
turn began). The same constant for every line, so the ranking is unchanged; luck now
measures deviation from expectation. The HIDDEN section prints what the cards were
worth to the line beside what was expected, the piece part separately. Measured
2026-10-21 over 24 games: Event +0.90, Fear card TL1 +0.18, TL2 +0.61, TL3 +0.28. Not
covered: a line that earns an extra Fear card by its own Fear gets the card's four Fear
at the ledger rate and not the card's average worth (`fear_card_mean / 4` per Fear
point would be the refinement); the Fear and Event decks card by card.

### 3c. Slow targets are chosen after the Explore

A candidate line now commits its Slow **cards** before the Invader steps and chooses
their **targets** on the island the Invader steps leave, in both scoring modes:
each Slow card (and Slow innate) in turn takes the legal target that scores best
with the lookahead, given the Slow plays already placed. The LINES table shows the
targets chosen under the Explore card that came; under another card the informed
score may have placed them elsewhere. Lines with the same Fast plays and the same
Slow cards fold into one.

For the actual line the recorded Slow targets stand in the hindsight score (that is
what happened), and `slow_gap` is the hindsight score with the best Slow targets
minus the recorded one: a Slow mis-target is reported as its own number, not folded
into luck. The informed score of the actual line chooses targets per Explore card
like any other line. A line typed with `--try` may fix a Slow target with `>`; left
bare, the simulator chooses it.

### 3d. Weight and bands from the corpus

`next_weight` replaces 0.5. It is measured: over every turn `t` with a recorded turn
`t+1`, the Blight and Dahan the next turn's Ravage and Build actually took, divided by
what they would have taken had the Spirit not acted (the lookahead's threat on the
recorded position). The ratio is the share of the unattended threat that
materializes once the Spirit plays its next turn.

The reading bands are standardized against the game's own noise. `luck_sd` is the
standard deviation of per-turn luck on the actual line over the corpus, with the
constants above in force (1.37 over 148 turns on 2026-10-21; the next-turn weight
0.33 over 98 turns). `model_sd` is the simulator's own error: the RMS, in ledger
units, of the recorded state at the Slow phase minus the simulated Invader phase of
the actual line (1.40 over 131 turns, mean +0.33: the simulator runs a little
pessimistic); it is the noise the confidence column is smoothed by. `decision / luck_sd` is an effect size and the bands follow
Cohen's conventions: Sound below 0.2 (the choice mattered less than a fifth of a
turn's noise), Better available from 0.2 to 0.8, Mistake from 0.8 (the choice
outweighed the noise). Unlucky needs luck at or below minus one `luck_sd`; Gamble
paid needs the actual hindsight within 0.2 `luck_sd` of the best. Without a
calibration file the built-in values are the ones measured on the corpus on the date
in `Calibration::source`. The ledger-against-results line (section 3a): wins -0.66
and losses -1.88 per turn, 72% of win/loss pairs ordered right over 21 decided games.

### 3e. Pieces prevented count like pieces destroyed

The ledger valued a Town destroyed at half a point and a Town prevented at nothing.
That asymmetry is what hid the value of Vengeance's turn-1 Disease in Brett's example
(a Disease placed where the Build comes stops a Town; the line read only its Fear): a
Town or City the turn's Build adds now counts against every line at the piece's value,
this turn in full and next turn at the weight, so stopping a Build by Disease, by
Wilds, by killing the lone Explorer first, or by any skip, is credited exactly as much
as destroying the piece after. The Explore's Explorers are not counted (the informed
lookahead cannot run the next Explore), so both scores stay comparable. The ledger's
separation of wins from losses rose from 66% to 72% of pairs with this term alone.

Spirit-target cards that act in one of the Spirit's lands (Fiery Vengeance: "1 Fear
and 1 Damage in one of target Spirit's lands") are tried in each presence land and
written `FV>C5`; before, the card was anchored on the first presence land and written
bare.

## 3f. Tokens, accelerators, Beasts (Brett, 2026-10-20)

Brett: "comparing a guide like Rei's which accelerates a specific spirit's
powers/strategy by placing say a Disease token T1 for Vengeance: you weigh a different
'best option' based on dmg/killing things only... how do we model this better
per-spirit, per-token, gameplay mechanics with individual accelerators; even think
Beasts and how they operate with many Events, and some spirits massively accelerate
with Beasts, or against opponents like Russia who have a loss condition based on
Beasts."

What a token is worth has three parts, and the model has one of them so far:

1. **Its rule effect over the horizon.** Disease stops the next Build, Wilds the next
   Explore, Strife turns a Ravage, Badlands adds damage, Beasts deal damage (with the
   right rules) and are Russia's clock. The simulator already runs Disease and Wilds
   in the Invader steps; with 3e the stopped Build is now paid for, so a turn-1
   Disease where the Stage I Build will land reads as half a Town plus the Ravage it
   averts next turn, averaged over the Explore cards that bring the Build there. This
   is general, no spirit knowledge needed, and it is the part Rei's opener depends on.
2. **Its value to the spirit's own engine.** Vengeance's innates count Disease and its
   Powers add it; Lure's and Sharp Fangs' count Beasts; Keeper's count Wilds. That is
   spirit knowledge, so it lives on the panel: `SpiritPanel::priorities()` (today:
   Dahan and presence weights with reasons) grows a `tokens` map derived from the
   panel's own text, an innate or special rule that counts a token kind, or a Power
   that needs one in its target, gives that token a weight equal to the share of an
   innate tier it unlocks over the horizon. No spirit-id checks in consumer crates
   (Brett's standing rule); the hindsight ledger reads the map like it reads the Dahan
   weight.
3. **Its interaction with the Event deck.** Event cards carry sections keyed by the
   engine (`EventSection.kinds`: Beast, Dahan, Token, Healthy and Blighted Island),
   and a Beast section fires only where Beasts stand. So Beasts are worth more on a
   board with many Events left than the Invader rules alone say, and the expected
   Event value (section 3b) should be conditional on the board: typed Event sections,
   simulated card by card over the remaining Event pool, give that directly; until the
   62 cards are typed, a cheaper step is to fit the Event coefficient on board
   features (Beasts present, Dahan count) once the corpus is large enough to carry
   the extra terms.

Adversary clocks belong to the adversary model, not the ledger: Russia's "Hunters
Swarm the Island" (engine text) counts Beasts destroyed by adversary rules and the
Invaders win when that number exceeds the Beasts on the island; from level 1, a Ravage
that adds Blight destroys a Beast in that land. So against Russia a Beast in a land
that will Blight is a loss-clock piece, and a line that keeps Beasts out of Blight
lands or adds them elsewhere is worth the clock's distance, the same shape as the
Blight pool term (section 3a). The adversary definitions already carry the text; the
typed loss-condition hook is the next step there.

Rei's opener as a line to compare against stays the right test: write the Vengeance
turn-1 Disease line in notation, score it with `--try`, and the token work is done when
the review ranks it where Rei does.

## 4. The deck as an explicit, multi-turn chance node (Brett's sidenote)

A Stage II card repeating a Stage I terrain lands a second Explore on lands still
carrying the first wave, with cascades, where the skipped terrain opens fresh lands
instead. The deck is drawn without replacement from a known composition, so for each
land the chance of an Explore on each of the next turns is exact: a card still in a
stage is at any of that stage's future draw positions with probability `1 / cards
remaining in the stage`, and the stage of each future turn is read off the
adversary's deck order (`Adversary::stages_at(level)`, Sweden's Accelerate included).

`FORECAST` in `si hindsight` prints, for the next three turns from the turn's Fast
phase: the stage, the cards still possible, and for each land the chance of an
Explore, marked **re-hit** when the land already holds Invaders or Blight (the
projected board after this turn's face-up Ravage and Build) and **fresh** when it
holds none; then the expected re-hits and expected fresh openings per turn. Stacking
costs are convex (cascades, lost presence); spread costs are linear but drain the
Blight pool toward the card flip.

The ledger's horizon deepens by one expected turn for the lines that get the informed
score: after the next turn's Invader steps, the turn after's Ravage (its card is this
turn's Explore, known in hindsight and the averaged candidate when informed) runs on
the island those steps leave, averaged over the next turn's possible Explore cards,
and counts at `next_weight²`. This is the same ledger one ply deeper with the chance
node averaged, not a new exposure weight.

## 5. Later: the node interface

```
State        the island, every spirit's position, decks as remaining multisets, phase
actions(s)   legal decisions at s (growth, plays with targets, Event/Fear lands)
step(s, a)   deterministic transition
chance(s)    the reveal at s with its probabilities (Explore card: uniform over the stage)
value(s)     the ledger, later a fitted win probability
```

Hindsight, `--try`, the notation parser and a depth-limited expectimax or MCTS all
sit on these five. Until then the pieces live where they are: `si-rules` for `step`,
`si-hindsight` for `actions` and `chance`, `Values` for `value`.
