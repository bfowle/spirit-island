# Hindsight: what a finished game says about each turn's choice

Status: design 2026-10-20; steps 1–2 of section 5 built the same day (`crates/si-rules`,
`si verify`): the Invader phase reproduces 1075 of 1116 logged outcomes (96%) across
Brett's 16 recorded games, 100% on the Shroud golden fixture, 98% on Vengeance vs
England 4 with the level given. Remaining mismatches are listed per game by `si verify`
and are mostly the player's counterattack and cascade choices, plus a few open rule
questions (England's Indentured Servants on a coastal land, Sweden's Swayed condition).
Step 3 (Spirit actions from the Effect language) started the same day: 92% of 1334
outcomes over the captures with Invader steps and Powers together; Power damage 125 of
142, pushes 12 of 29 (Plaguebearers' travelling Disease and Gather sources are the gaps).
The engine applies Badlands, and Vengeance's Blight-as-Badlands, once per Power use.
Step 4 started the same day as `si hindsight` v1 (`crates/si-hindsight`): per turn, every
legal line from the recorded hand (Fast and Slow plays with targets, innates reached)
scored on the turn's real Invader steps plus the next turn's at half weight; the
actual line ranked among them; readings Sound / Better available / Mistake on what was
known; `--try` scores a line written in notation. Second pass the same evening: growth
alternatives (every group, slot, legal placement, offered card or the next recorded
draft, Reclaim) are positions the plays are enumerated from; the Event and Fear piece
effects are applied to every line as the log shows them (the difference between the
simulated Fast phase and the recorded state at the first Invader step); every choice
the engine asked for is recorded (`TurnRecord.choices`) and the Event and Fear land
choices are scored against their alternatives where the effect can be moved; the
search runs threaded over positions with a two-stage innate search; each line prints
its elements and innate tiers. Third pass, same evening: the informed score of section 1
(`i_t`) is computed for the best lines and the actual one — averaged over the Explore
cards still in the stage, the Event and Fear effects left out, a card taken from a later
draft not played — lines rank by it, `luck = hindsight − informed`, and the confidence
of section 2b is the share of possible Explore cards under which a line beats the actual;
the readings Sound / Unlucky / Gamble paid / Better available / Mistake follow section 2.
Cards are in three states (`si cards --card`): read and simulated; typed but not simulated
(the turn is judged with a caveat naming the effect left out); not read (not judged).
Coverage plan: `2026-10-20-card-coverage.md`; live replay of a checkpoint in the digital
app: `2026-10-20-live-replay-checkpoints.md`.
Not yet: step confidence from `si verify` agreement, the Event and Fear decks as
distributions (their effects are simply unknown in the informed score), a card gain's
expected value over the deck (it scores the flat card value), deeper lookahead, Push and
Gather choices, Event option choices. Builds on `2026-10-20-tempo-and-growth-model.md`
(ledger, chains, reach) and `2026-10-20-game-model-architecture.md` (si-rules as
replay-then-simulate). Brett's ask, same day:

> point at a 'done' run, and using the oracle we created show turn by turn what
> we did, what was the most optimal thing AFTER the fact once the hidden info is
> shown (events, fear cards, explores, card draws) … was it a mistake (we had the
> info, didn't make the right choice with it) vs. a random event/card/explore that
> screwed us.

## 0. Sources: the log and the save

Two files describe a finished game. `EndGame--<id>.txt` is the engine's log
stream: what happened, in order, with undo reloads re-logged. `EndGame--<id>.dat`
is the engine's serialized state (gzip of .NET binary serialization, decoded by
`si-steam::nrbf` + `engine_map`, nothing unmapped): the island and every spirit at
the end, the adversary and its level, the scenario, the Blight card, the engine's
status effects, its land-distance cache, the RNG seed, a structured **journal**
of the whole game, and every deck's remaining cards **in draw order** (Invader,
Fear with the Terror Level cards in place, Event, Minor and Major Powers).

Of Brett's 61 finished games, only 20 have an `EndGame` log at all (the text log
was not kept for the earlier ones); every one has its save. For hindsight the save is
the primary source of the hidden sequence `H` and of the exact adversary level; the journal is the undo-free event history and should
replace the bare log's heuristic undo repair for finished games. The recorder's
captures keep `events.txt` and the log stream; they should also keep the
`EndGame` save. Hidden information is read only after the game: the companion's
live projection never exposes the decks.

## 1. Definitions

A finished game gives the **realized hidden sequence** `H`: the Invader cards in
draw order, the Event card each turn and what it did, the Fear cards earned and
what they did, the four Power cards revealed at every gain. None of it is known
when the turn's choices are made; all of it is known afterward.

A **decision point** is one Spirit turn `t` with its choice set `A_t`: growth
group and options, presence placements, which cards to play (within plays and
Energy), which land each Fast and Slow Power and innate targets, Dahan and
Invader movement choices inside a Power, Event pay-or-not choices, the card kept
from a gain. The **state** `S_t` is the island and every spirit at the start of
turn `t`.

The **value** of a line of play is the ledger, never a hidden score:

```
v(turn) = − Blight added
          − ω_D · Dahan lost            (ω_D per spirit, D1)
          + ω_F · Fear generated        (Fear cards are the other clock)
          + ω_S · track slots revealed  (scaling, Rei)
          + ω_C · cards gained
V(line)  = Σ_t v(t) + terminal (won / lost, turn)
```

The weights are a `Values` struct with sources, printed beside every number;
the components are always shown raw as well. Rei's own trade (take Blight to
scale) is a statement about ω_S against one Blight, and the report must let
Brett read it either way.

Three plays are evaluated at every decision point, all scored on the realized `H`:

| Play | Chosen how |
|---|---|
| `actual` `a_t` | what the log says Brett did |
| `hindsight` `h_t` | the best play when `H` is known: the simulator runs each candidate forward with the real next Invader card(s), to a depth that covers Explore → Build → Ravage (two turns), and the real Event and Fear effects applied as exogenous inputs |
| `informed` `i_t` | the best play on what was knowable at `t`: the same search, but the next Invader card is an expectation over the cards still in the current stage (their count and terrains are known), the Event and Fear effects are unknown (counted as zero, named as noise), and a card gain is worth its expected value, not the card that came |

## 2. Mistake or luck

With the three plays scored on the realized `H`:

```
gap(t)       = V(h_t) − V(a_t)              what hindsight would have changed
decision(t)  = V(i_t) − V(a_t)              what the known information supported and we did not take
luck(t)      = V(h_t) − V(i_t)              what only the hidden cards could have shown
```

Each turn gets one of four readings, with the hidden card that decided it named:

| Reading | Rule | Report line |
|---|---|---|
| **Sound** | `a_t ≈ i_t` and `gap ≈ 0` | "the right play, and it held" |
| **Unlucky** | `a_t ≈ i_t` and `luck ≫ 0` | "the right play on what you knew; Jungle2 exploring C1 and C4 is what cost 2 Blight; only hindsight finds the line that beats it" |
| **Mistake** | `decision ≫ 0` | "Strike Low on C5 for 1 Fear spent 2 Energy; holding it and playing Fiery Vengeance into C8 was better on what you knew, by Δ" |
| **Gamble paid** | `a_t ≠ i_t` and `V(a_t) ≥ V(i_t)` | "not the percentage play, but the Event saved it" |

`≈` is a tolerance in ledger units (one Blight, or the Dahan weight, whichever is
smaller). The informed play is computed on the information set as the log shows
it: what cards were in hand, what the track gave, which Invader cards had been
seen, which lands were coastal, what the adversary level does.

## 2b. Confidence (Brett, 2026-10-20)

Every number the hindsight report prints carries two confidences, printed beside it.

**Step confidence** is how far the simulator is trusted for the kind of step it
just took, measured, not asserted: the `si verify` agreement for that step and
field over the corpus (Ravage Blight 100%, Build pieces 98%, Dahan losses with
the player's distribution 85%, …), lowered on the spot by any note the step
carries ("counterattack distribution: player's choice", "rule not modeled",
"Event effect replayed from the log"). A line of play is as confident as its
least confident step; the report names that step.

**Information confidence** is how much of what decided the turn was knowable
when the choice was made. For each hidden variable at turn `t` the information
set gives a distribution, and the report prints it in game words:

| Hidden at `t` | What is knowable | Printed as |
|---|---|---|
| next Invader card | the stage, the cards already seen, the deck order the adversary level fixes | "next Explore: Wetland 2 in 4, Jungle 1 in 4, Coastal 1 in 4" |
| Terror Level flip | Fear earned, Fear-deck sizes per level (adversary level) | "Terror II in 2 more Fear cards" |
| Fear card | the cards left in the deck (all known by name) | "1 of 6; 4 of them would stop a Build here" |
| Event | the deck's remaining cards (known by name, order hidden) | "1 of 55; 9 add Blight somewhere" |
| Power card gain | the remaining Minor/Major deck by name | "1 of 88 minors; 31 carry Fire" |
| player choices inside the engine | nothing hidden; the choice is Brett's | not a hidden variable |

The **informed** play's confidence is the share of the hidden outcomes (over
the distribution above) in which it stays the best play. "Informed: hold Strike
Low, 71% (loses only if Jungle2 explores both Jungles)" is the shape of the
line. When the save's deck order is available after the game, the report also
prints what was *deducible* at the time and whether the actual play used it:
the Invader deck's stage order is public once the adversary level is known, so
"a Stage III card next" is knowable at Prussia 2 on turn 4, not luck.

## 3. What the simulator must do, in order of verification

The oracle is the log. Nothing in the simulator is trusted until the replay of
a logged turn reproduces the log.

1. **Invader phase** (`si-rules::invaders`): Ravage (damage from pieces, adversary
   modifiers, Defend, Strife, skips; Blight and cascade; Dahan damage and
   counterattack; presence destroyed), Build (City over Town rule, Disease,
   England's adjacency rule, the High Immigration slot, escalations), Explore
   (matching lands with or adjacent to a Town or City, coastal lands; Wilds and
   Isolate). Verified turn by turn against every recorded game: `si verify`
   prints, per step, lands simulated, lands matching the log, and each mismatch
   with both values. Player choices inside a step (where the cascade went, which
   Invaders the counterattack hit) are compared as counts and flagged as
   "distribution" when only the choice differs.
2. **Spirit actions**: the `Effect` language executed on the state (Damage with
   the player's distribution, Defend, Push and Gather with destinations as
   choices, tokens, Fear, skips), element thresholds from track plus cards,
   innates. Verified by replaying every logged Power use and comparing pieces
   moved, damage dealt and Fear generated.
3. **Growth and Time Passes**: yields, placements, Energy, hand and discard,
   Reclaim. Already reproduced by the record; the simulator must agree with it.
4. **Exogenous inputs**: Event and Fear card effects are not simulated in the
   first version. The logged effect is replayed onto the simulated state where
   its pieces exist (same land, same kind) and dropped with a note where the
   branch has moved them. Adversary Fear-pool and Terror thresholds are
   simulated from the deck sizes.

Only after 1–3 verify does the search run. Search per decision point:
candidates are enumerated from the record's own vocabulary (growth choices from
the panel, plays from the hand within plays and Energy, targets from `reach`
over lands with a chain due), scored by the ledger with a two-turn lookahead on
the real cards, beam of the best 50 at each turn. Branches that need an Event or
Fear effect the branch has invalidated are scored without it and marked.

## 4. Outputs

`si hindsight <log|capture> [--turn n] [--depth 2] [--values default]`, one block per turn:

```
T3  actual:   G2 bottom×2 · Strike Low→C5 (1 Fear) · Fiery Vengeance→C6 (Town killed)   v = −1 Blight, +2 Fear
    informed: G2 bottom×2 · hold Strike Low · Fiery Vengeance→C8                        v = −1 Blight, +1 Fear, +2 Energy kept
    hindsight: same as informed                                                          Jungle2 explores C1, C4 next turn: no card in hand reaches them either way
    reading:  Mistake (small): 2 Energy and a card for 1 Fear; the Blight was coming regardless
```

plus the game summary: readings per turn, total gap split into decision and
luck, the hidden cards that cost the most, and the ledger of the hindsight line
beside the actual one. Numbers appear with their components; a reading is never
printed without the play it is compared to.

## 5. Order of work

1. Record snapshots the Invader phase needs (state before the first Invader
   step each turn, Defend per land, cards per slot incl. High Immigration).
2. `si-rules::invaders` + `si verify`, base rules first, then the adversaries in
   Brett's corpus (England 1–4, Prussia 1–2, Sweden 1–5, Russia 1–5, Habsburg
   Mining 1–2). Verification over all 20 games; the mismatch list is the work queue.
3. Spirit action execution from the `Effect` language; verify over logged uses.
4. The informed and hindsight searches; `si hindsight` on the Vengeance vs
   England 4 game first, read with Brett.
5. Rei as a policy (the architecture spec's `si-policy`) becomes a fourth play
   to compare against: "what Rei's build would have done here".

## 6. Study board (later)

Brett, 2026-10-20: the end state of this is visual. The companion (apps/web, si-ui)
gets a study view that loads a finished game (record plus save), steps through the
turns and phases with the board drawn from the recorded snapshots (`before_fast`,
`before_invaders`, `before_slow`), shows the notation beside it, and lets the player
pick a different play or target and see the simulated outcome and score next to
the actual one, the way a chess player moves pieces on an analysis board. The CLI
`si hindsight --try` is that loop without the board; the engine it needs (si-rules,
si-hindsight) is the same. No UI work before the search and the confidences are in.
