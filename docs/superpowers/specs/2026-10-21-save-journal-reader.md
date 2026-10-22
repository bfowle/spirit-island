# The save-journal reader

**Date:** 2026-10-21. **Status:** built the same day; `si journal --check` matches the save's board on every land of all 18 captures that hold saves.

## Why

The record of a game has come from `log.txt`, the app's friendly log, since the first
replay. That log is written by eleven places in the view layer, it is closed when a game
ends and only reopened when the game view starts afresh (the reload trap of 2026-10-21),
it is truncated at every `GameView Start`, and an undo rewrites nothing in it, so the
replay guesses which lines an undo reverted when no save clock is at hand. Every one of
those problems is a property of the log, not of the game.

Every decision save carries the engine's own typed journal (`Game.Journal._journalEntries`,
607 entries by turn 5 of the Thunderspeaker game): the material `log.txt` is written from,
kept by the engine as it stands, so an undo simply reloads a save whose journal is the
truth. Reading the record from the newest save's journal removes the log from the path:
no undo guessing, no journal copies, no reload trap, and the live screen follows the game
from the same files the app writes at every decision.

## What is read

`si_steam::journal::entries(&graph)` walks the journal list and types each entry
(`Kind`): ChangeState, UseActivation (the player's answer to an activation, with its
selection), AddPieces, MovePieces, DestroyPieces, MovePresence, GainEnergy, PayEnergy,
GainElements, LoseElements, PlayPowerCard, StartPower, UsePower, ModifyPowers,
UseGrowthOption, FinishGrowthOption, AdjustFear, FlipCard, FlipCardStack, InvaderState,
Explore, BuildLand, RavageLand, RavageComplete, DealDamage, AddBlight, AddTokens,
MoveTokens, MoveCards, RevealCards, GainPowerCard, ResolveEventCard, ResolveFearCard,
RuleAction, FinishRuleAction, GameSetup. Anything else is `Other(class)` and counted.
Each entry keeps the engine's turn index, state index and state (`Invader/Ravage`).

Model objects are resolved to the identifiers the log already uses: lands `SandsA4`,
pieces `Town2`, spirits `Thunderspeaker`, cards `Sands1`, track slots
`ThunderspeakerCardPlays5`; locations by their title (`Out of Game`,
`Thunderspeaker's Hand`) so the log reader's pile rules apply unchanged.

Firewall: an entry names a card only once the engine moved or flipped it in the open.
The reader never looks into a deck or a face-down card; the save's hidden state is not
on its path.

## From entries to the record

`journal::parsed(entries, boards)` yields the replay's own `Parsed` items (events and
markers), and `Replay::run_parsed` walks them exactly as the log's lines are walked, so
every review, ledger and screen downstream stays the same. The mapping follows the log
reader line by line, with three state machines the log did not need:

- An Invader card's step is `InvaderCardResolved` when its state ends (the next
  ChangeState, or the next card's InvaderState), after its pieces and Ravages, as the
  log's `ResolveInvaderCardAction` came last.
- A growth group is marked once per spirit and turn (the first option used from it).
- `StartPower` marks the power's phase, `UsePower` records the use.

`journal::record(&graph, defs, source)` is the whole path: one save in, a `GameRecord` out.

## Where it is used

- `si-steam-dump journal <save>` prints the resolved entries, one per line;
  `journal-raw` the object graph under them.
- `si journal <capture dir | save>`: the record from the newest save's journal, and with
  `--against-log` the differences from the log-built record of the same capture (turns,
  ending, each turn's Invader cards, Fear, Blight, and each land's end counts). The
  acceptance test is that difference on the captures that hold saves.
- `si study --live`: the record is built from the newest save's journal whenever a save
  decodes and its journal starts at setup; the footer says `(save journal)`. The log
  path stays as the fallback for a folder without a usable save.

## Not in this build

- The replay loaders of `si report`, `si replay` and `si hindsight` keep reading the
  log; switching them is a follow-up once the differences are zero on the corpus.
- RuleAction brackets (which rule caused what) are counted, not yet carried into the
  record's touches.

## What building it found (2026-10-21)

- The engine places Jagged Earth's base-board tokens (a Beast on A1, Disease on A2, a Beast
  on B5, Disease on C2, a Beast on C4, Disease on D2, a Beast on D3, as the captures show)
  at setup with no journal entry, and the board definitions carry only the printed tokens
  of the newer boards. `journal::setup_tokens` seeds them: a token the journal never added,
  at the land its first move or destroy names, else where the save holds it; the ones the
  board definition prints are left to the walk's own seeding.
- One Blight action is journaled once per piece it adds, each entry carrying the running
  total (`added 1 after 0`, then `added 2 after 0`): adjacent entries for the same land and
  action are read as one.
- The journal writes DestroyPieces before the move off the island; the log wrote the move
  first and the walk reads them in that order (the move removes, the destroy of a piece
  already gone is skipped). The reader holds a destroy until the move has gone through.
- The log never carried presence a Power pushed or gathered (`MovePiecesAction` of a
  presence piece, which the log reader ignores), so every log-built record had presence
  where it no longer stood. The journal's MovePresence entries do; the live screen's
  POSITION column changed for the Shroud fixtures (D8 presence 3 to 1) for that reason.
- A card revealed and not gained goes back from the spirit's reveal pile to the deck's
  discard; only a card leaving the hand, discard or play area for a deck's discard is
  forgotten.
- A land never touched had no ledger in the record; every land of a board in play has one
  now, so the end names the printed pieces that stood all game.
- Choices: the journal's UseActivation entries carry the selection and the lands offered
  (from the activation identifier) but not the effect texts the log quoted; the hindsight
  review's choice scoring reads lands, so it works, and Effect choices list an index.
- Growth options are named by the engine's identifier (`Thunderspeaker/AddPresence/3_0`)
  where the log quoted the controller's title; nothing downstream reads the text.
