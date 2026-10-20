# Live replay: reload the digital game at a checkpoint and play the other line

Status: plan, 2026-10-20 (evening). Brett's ask, same day:

> use the SI digital app to reload at checkpointed times based on the *.dat/saves/logs
> and reload at those stages to test these other routes so we can learn. e.g. if I'm on
> the last couple rounds and something drastic was missed, I'd like to LEARN from that
> and see it live, with all of it preloaded, so I can tinker. Instead of trying to set it
> all up again, hook into REA → the Steam app, set and load at that moment in time.

## 0. What already exists

- **The app saves at every decision point.** The Steam folder holds
  `save-Feedback-<slot>-<journal index>-<phase>.dat` (e.g. `save-Feedback-007-0201-FastPower.dat`),
  a rolling window of the last few, plus `save-Undo-NNN.dat` copies the app loads when
  the player undoes ("LoadedUndo" in the log). `MainAssembly.dll` carries the names
  `FeedbackSaveIndex` and `ViewDataFeedbackSaveIndexKey`, so the app remembers which
  save index it is on, and `_hasSavedSinceLastUndoCopy` drives the undo copies.
- **The recorder keeps every one of them.** Each capture directory has a `saves/`
  folder with every Feedback and Undo save the app wrote, gzipped and timestamped
  (`161558_614__save-Feedback-000-0004-SpiritPhase.dat.gz`): 4,949 saves across the
  captures, roughly one per decision. A captured game therefore has a checkpoint at
  every Spirit phase, Fast phase and Slow phase of every turn. A bare EndGame game has
  only its final save.
- **The saves decode completely.** `si-steam-dump sum <save>` reads a Feedback save as
  it reads an EndGame save: turn and phase, every land, the decks in draw order, the
  spirit's hand, play, discard and forgotten piles, the tracks, the status effects.
  The format is gzip of .NET binary serialization (NRBF); nothing is unmapped.
- **Hindsight names the line to try.** `si hindsight <game> --turn N` prints the actual
  and the best lines in notation, with their informed and hindsight scores.

## 1. Phases

### Phase 1 — checkpoint load, no reverse engineering

`si checkpoint <capture> --turn 6 [--phase SpiritPhase|FastPower|SlowPower]` lists the
saves that match, and `--load` (app closed) backs up the Steam folder's current saves,
copies the chosen save in under the active slot and the next index
(`save-Feedback-<slot>-<index>-<phase>.dat`), and prints what to do: open the app,
Continue. The hindsight block for that turn prints beside it, so the alternative line is
on screen while the board is on screen.

Open questions, answered by hand on one game before anything else is built:

1. Which save does Continue load: the highest index of the active slot, or the index in
   `FeedbackSaveIndex`? (Expectation: the key; then it must be set too — it lives in
   the app's view data, `Parse.settings` or PlayerPrefs; `si-steam-dump` can read
   `Parse.settings` the same way it reads a save if it is NRBF.)
2. Does the app check the save against its build (`build 1.0.9776.27799` in the save)
   or anything else? (Expectation: version only.)
3. Does loading a save from another game's slot confuse the EndGame bookkeeping (game id
   `live-…`)? The recorder must keep recording the new branch as a new game.

REA is the tool for these questions only if trying it does not answer them: static
first (`inspect_managed_members` on `MainAssembly.dll` around `FeedbackSaveIndex`,
`_hasSavedSinceLastUndoCopy` and the `save-` format string, to read the load path and
any validation), dynamic (Frida on the running app) only if the static read is unclear.

### Phase 2 — the branch is a game of its own

The recorder sees the reload as a game continuing from a save; the branch gets its own
capture (`live-<id>` with the save's game id and a branch marker), its own record and
its own hindsight. `si report` and `si hindsight` take a `--from <checkpoint>` so the
branch is read from the fork, and the corpus lists branches under their parent game
rather than as independent games (they share the hidden sequence up to the fork).

### Phase 3 — preloaded alternatives (edited checkpoints)

Writing a save: an NRBF encoder in `si-steam` with a round-trip test over every kept
save (decode → encode → identical bytes, all 4,949). Then edits with the smallest blast
radius first: Energy, a card moved between hand and discard, presence on a track slot,
a piece in a land. The app's journal inside the save is left alone (it is history, not
state) until an edit shows the app reads it on load. The goal state is "load turn 6 with
the hindsight line's growth already taken" so the Fast phase starts at the alternative;
the stretch is "load turn 6 with the Fast plays already made".

### Phase 4 — the study board

The companion's study view (hindsight spec §6) gets a "play this live" button that
runs Phase 1 for the turn on screen.

## 2. Order of work

1. By hand: copy one captured Spirit-phase save into the Steam folder, open the app,
   Continue; note which index loads and whether the key must be set. Half an hour.
2. `si checkpoint` list + load, with backups, from the answer to 1.
3. Recorder: a branch marker when the app loads a save it did not write itself; keep the
   EndGame save with each capture.
4. NRBF encoder round-trip; first edit (Energy); load test.
5. Study-board button.

## 3. Risks

The app may refuse a save that is not the newest of its slot, or may reload its own
rolling copy on Continue; the key is then the pointer to set. Edited saves may desync the
journal; Phase 3 is therefore last and gated on the round-trip test. Steam cloud sync may
overwrite the folder: disable it for the app while testing.
