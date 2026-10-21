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
  The format is .NET binary serialization (NRBF, first bytes `00 01`), which the
  recorder gzips when it keeps a copy; nothing is unmapped.
- **Hindsight names the line to try.** `si hindsight <game> --turn N` prints the actual
  and the best lines in notation, with their informed and hindsight scores.

## 0b. What the game's own code does (read from MainAssembly.dll, 2026-10-20)

Brett's requirement, same night: the checkpoint must load into the **running** app, no
restart. The app already has that path, because Undo is a reload:

- `Handelabra.SpiritIsland.View.GameViewController.ReloadFromFile(string filename, bool
  allowCloudSave, bool isUndo, bool isFullSync)`: loads the named save through
  `GameSerializer.LoadGame(filename, ref viewData, bool, bool)`, stops the running
  coroutines and activations, tears the views down (`CleanupGameController`,
  `PrepareToReplaceIslandView`), writes the loaded bytes over the continue file
  ("ReloadFromFile overwriting continue file with loaded file"), deletes undo files at
  and after the loaded one when `isUndo`, and calls `DisplayGame`. `UndoManager.
  ActuallyUndo` calls it with `_undoFileName`; the main menu's Continue goes through
  `GameSerializer.LoadContinueGame` instead.
- Save names: `GetSaveFileFullName(name)` = `"save-"` (+ `"MP-"` in multiplayer) + name
  + `".dat"`. The continue file is `save-ContinueGame.dat`; undo files are
  `save-Undo-{index:000}.dat`; the per-decision copies are
  `save-Feedback-{turnIndex:000}-{journalIndex:0000}-{state}.dat` (`FeedbackSaveIndex`
  is the turn index, 0-based, kept by `SaveForContinue` and reset in `RemoveContinue`;
  a capture of a 7-turn game shows `000` … `006`).
- Storage: `GameSerializer.UseSteamCloud` is `SteamManager.Initialized`, so with Steam
  running the continue and undo files live in Steam Cloud (`SteamRemoteStorage.FileWrite`
  / `FileRead`), not on disk, which is why the folder shows only the Feedback copies and
  no `save-Undo-*.dat`. `GameSerializer.WriteBytes(string name, byte[] bytes, bool, bool,
  bool? isMultiplayer)` and `ReadBytes(name, bool)` go to the same place the loader reads
  from, whichever it is.
- `GameViewController.Update()` exists: the main-thread hook point. Unity view code
  must run on the main thread, so the injector does its work inside that method.

## 1. The mechanism, and how it launches from WSL

No Python, no frida-server, no firewall rule. Frida ships a standalone Windows binary,
`frida-inject.exe`, that attaches to a process by name and loads a script; WSL runs
Windows executables directly, so `si inject` (Rust, in `si-cli`) does everything:

1. Finds `frida-inject.exe` in the companion's Windows tools folder
   (`/mnt/c/Users/<user>/AppData/Local/si-companion/tools`); `--fetch` downloads
   `frida-inject-17.24.3-windows-x86_64.exe.xz` from the Frida releases and unpacks it
   there (curl + xz, once).
2. Checks `SpiritIsland.exe` is running (`tasklist.exe`); refuses otherwise.
3. Picks the checkpoint (`--turn`, `--phase`), writes it under `tools/checkpoints/` as
   the `.dat` the app wrote and once more under the name the app will load it as
   (`--name`, default `save-Undo-990.dat`), and converts the paths with `wslpath -w`.
4. Runs `frida-inject.exe -n SpiritIsland.exe -s tools/live-replay/reload_checkpoint.js
   -P {path, name, log} -i`. frida-inject calls the script's `rpc.exports.init(stage,
   parameters)`; `-i` keeps it attached until `si inject` closes its stdin.
5. The script (Mono embedding API from `mono-2.0-bdwgc.dll`) resolves
   `GameViewController` and `GameSerializer`, checks the four methods it calls exist with
   the arities read above, hooks `GameViewController.Update` (JIT address from
   `mono_compile_method`) and, on the next frame, on the main thread: `GameSerializer.
   WriteBytes(name, bytes, false, false, null)`, `DoesSaveFilenameExist(name, false)`,
   `controller.ReloadFromFile(name, false, true, false)`. Each step is appended as a
   JSON line to `tools/checkpoints/inject.log` (frida-inject holds its stdout until it
   exits in attach mode, so the file is the channel `si inject` follows); a managed
   exception comes back as text in an `error` line; `done` ends the run.
6. `si inject` closes frida-inject's stdin, which detaches it, and reports.

Verified on 2026-10-20: `frida-inject.exe` runs from WSL, attaches to a Windows process
of the same user, loads the script, delivers `-P` parameters to `init`, exits cleanly on
stdin EOF with no process left behind. Brett's first run against the game reached `setup`
(154 images; every method the script calls resolved with its arity) and `read`, then
waited: the app was at the main menu, so `GameViewController.Update` never ran. Two
consequences built in the same evening:

- a second route for the main menu: the checkpoint is written as `save-ContinueGame.dat`,
  `GameSerializer.IsContinueValid()` confirms the app can load it (it loads the file to
  check), and `MainMenuViewController.ContinueClicked()` opens it; the script hooks both
  controllers' `Update` and takes whichever frame comes first;
- frame diagnostics: `hooked` (addresses), `frame` (the first main-thread frame seen),
  and a `waiting` line every 8 s while no frame has come, because Unity stops running
  frames while the window is unfocused or minimized. `si inject` waits 90 s by default.

**Verified working, 2026-10-20 (fourth run).** From the home screen, `si inject … --turn 3
--phase SpiritPhase` opened the Shroud game at turn 3 in the running app, no restart.
What the run showed, and what it changed in the design:

- `UseSteamCloud` is true on Brett's machine, so the continue-file lookup goes to Steam
  Cloud and does not see a disk file. The app's own fallback does: `GetContinueGameFileName`
  scans the save folder for the newest `save-Undo-*.dat` ("Continue file missing but found
  undo: save-Undo-990.dat"), and `LoadDataFromFile` read it from disk ("Successfully
  loaded data from save-Undo-990.dat"). `IsContinueValid` loaded it once to check,
  `ContinueClicked` loaded it again and showed the game. So the undo name is the one
  that matters, for both routes; `si inject` places only that file.
- Three earlier runs failed for reasons now fixed: the hook waited on the in-game
  controller while the app sat at the main menu (both controllers are hooked now); the
  script's `WriteBytes` call faulted inside the app (the file is placed from WSL instead,
  and the native calls let faults propagate to Mono, whose handler turns a null
  dereference into a managed exception); frida-inject held its stdout in attach mode (the
  script writes a log file that `si inject` follows).
- The reloaded game keeps its game id (the save carries it), so its later saves and log
  lines continue under that id: the checkpoint DAG shows the branch as a fork off the
  loaded node without any new identifier.

Still to confirm: the in-game route (`ReloadFromFile` with `allowCloudSave=false`) on an
open game, and what the recorder does with a game whose id it has already captured.

## 1b. Checkpoints as a graph

Every save is content-addressed: its id is the sha256 of its bytes. Copies share the id,
so nothing new has to be minted when a checkpoint is exported or reloaded; a reload that
lands on an existing id is a fork, and the saves the app writes after it hang off that
node. `si checkpoint <capture…> --graph [--dot]` builds it: nodes are distinct saves
(turn, state, file, game), each save's parent is the save before it on the capture's
clock, and a save whose id already exists moves the tip back to it. The Shroud capture
is 302 files, 185 distinct saves, 7 forks: the undo branches Brett took. A live-replay
branch records as a new capture whose first save matches a node of the parent, so two
captures of one game join at the fork; `--graph` over both draws the tree. Every path
from the root is a line of play; the hindsight lines are hypothetical edges out of a
node; the ledger walks the realized path. The recorder should keep the id of the save it
observed with each `LoadedUndo`/reload so the join needs no hashing later.

## 1c. The study TUI (next build)

`si study <capture…>` (ratatui, in the offline crate cache): one screen per turn,
stepped with ←/→. Left, the growth and plays as recorded in notation with the glyph
line (elements, innate tiers); right, the hindsight block for that turn (informed,
hindsight, luck, confidence, the choices the game asked for); bottom, the checkpoint
DAG around this turn with the current node marked and the forks it has. Keys: `l` loads
the turn's Spirit-phase checkpoint into the running app (`si inject` in process, with
the log tail shown), `f` on a fork switches which branch the right panel follows, `h`
re-runs hindsight with `--try` on a line typed in notation, `e` exports the checkpoint.
The TUI only reads the record and the saves; the simulator and injector are the same
code `si hindsight` and `si inject` use.

## 2. Phases (revised)

1. Live run of the injector on a checkpoint of the open game (an Undo-equivalent),
   then on a checkpoint of another captured game. Half an hour each with Brett at the
   machine.
2. `si checkpoint` list and export; the hindsight block for the chosen turn printed
   beside it.
3. Branch review (built 2026-10-20 night): a branch's EndGame log starts at the load
   (`LOADED FILE: save-Undo-990.dat`), so `si hindsight <log> --seed <the checkpoint
   save>` seeds the replay from the save's engine snapshot (board, presence per spirit,
   Energy, piles, revealed slots, turn, phase, the Invader cards already out of the
   deck); the turns keep their real numbers and the hidden-card math knows the deck.
   `si inject` prints the flag. `si import <EndGame log>` builds a `live-<id>[-branch]/`
   capture from the log and the saves the app still has (its final `.dat`, the current
   game's step saves), so an unrecorded game or a branch lands beside the captures.
   Still open: the recorder itself on a reloaded game id (it would append to the
   parent's capture; the DAG's fork rule handles the saves, the log needs a branch
   marker).
4. Edited checkpoints (NRBF encoder, round-trip test over the 4,949 kept saves; first
   edit Energy) so a turn can start with the hindsight line's growth already taken.
5. The study board's "play this live" button.

## 3. Risks

Unity view code off the main thread crashes the app: the hook keeps everything inside
`Update`. A managed exception inside `ReloadFromFile` is reported, not swallowed; the
app's own log (`Player.log`) carries its "ReloadFromFile …" lines for the diagnosis.
Steam Cloud: a checkpoint written into the cloud slot is a normal save to the app;
`isUndo` trims later undo files, as a real Undo would. Nothing here edits a save's
contents until Phase 4.
