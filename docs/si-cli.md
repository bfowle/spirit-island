# Running the game-model tools (`si`)

Everything below runs from the companion worktree on WSL:

```
cd /home/brett/repos/spirit-island-steam-companion
```

The crates are `crates/si-model` (the game as data), `si-defs` (engine tables →
model), `si-record` (Steam log → game record), `si-metrics` (Rei's tempo, Lumia's
growth, reach, ledger), `si-rules` (the Invader phase simulated and verified),
`si-cli` (the `si` binary). Specs: `docs/superpowers/specs/2026-10-20-*.md`.

## 1. Build and test

```
cargo build --release -p si-cli            # binary: target/release/si
cargo test                                 # every game crate (needs no network)
```

If cargo complains about the network, add `--offline`; every dependency is already
in the local cache.

Below, `si` means `target/release/si` (or `cargo run -q -p si-cli --` while developing).

## 2. The engine definitions (once per game update)

The tools need the digital edition's definition tables (spirits, cards, boards,
adversaries, decks). They are read from the game's DLL into JSON:

```
cargo run -q -p si-steam --bin si-steam-dump -- defs data/engine-defs
export SI_DEFS_DIR=/home/brett/repos/spirit-island-steam-companion/data/engine-defs
```

`data/engine-defs/` is git-ignored (game content); regenerate it after a Steam
update. With `SI_DEFS_DIR` set you can drop `--defs <dir>` from every command below.

## 3. Where the games are

| What | Path |
|---|---|
| Finished games, Steam log + save | `/mnt/c/Users/brett/AppData/LocalLow/Handelabra Games Inc_/Spirit Island/EndGame--<id>.txt` and `.dat` |
| The game in progress | same folder, `log.txt` and `save-Feedback-*.dat` |
| Recorder captures (exact undo clock) | `/home/brett/repos/spirit-island/data/steam-capture/live-<id>/` and `/mnt/c/Users/brett/AppData/Local/si-companion/steam-capture/live-<id>/` |

A capture directory or an `EndGame--*.txt` file is what every command takes as
its first argument. A capture is better than a bare log: its `events.txt` carries
the save clock, so undo reloads are placed exactly. When an `EndGame--<id>.dat`
save sits beside the `.txt`, the tools read the adversary level, difficulty,
scenario and Blight card from it.

Shell quoting: the Steam folder has spaces, so keep it in a variable:

```
STEAM="/mnt/c/Users/brett/AppData/LocalLow/Handelabra Games Inc_/Spirit Island"
CAPS=/home/brett/repos/spirit-island/data/steam-capture
```

## 4. One game

```
si report "$STEAM/EndGame--8584090885044354762.txt"      # the Vengeance vs England 4 game
si report $CAPS/live-e392726411c9                          # the Shroud win (capture)
```

What the report prints, top to bottom:

- **header**: spirits, boards, adversary, turns, ending; the save's level and
  decks when a save exists; the level inferred from the log otherwise; a caution
  line when undo reloads had no clock (bare log).
- **LAND TEMPO**: one row per chain (one Invader card's pass over one land:
  Explore t → Build t+1 → Ravage t+2). `t0 E B R` are the turns, `outcome` what the
  Ravage slot did, `class` Rei's reading (above, on, gambled, failed, sacrificed,
  untouched, late), `price` the cards and Energy aimed at that land, `reach` which
  held card could have acted when the land was left to Blight, then the touches by
  clock position (`X→B` = after the Explore, before the Build; `B→R` = before the
  Ravage).
- **LEDGER**: per turn, lands due to Build and to Ravage, how the Ravage-due ones
  ended, cards and Energy spent, hand → discard, hand runway (turns before a
  forced Reclaim).
- **by Ravage turn** and the game totals by class.
- **TRACK TEMPO**: per spirit per turn, growth group, x/ycp, presence placed from
  the track (and destroyed), cards gained, Reclaim, played/plays, Energy, hand,
  discard; then Rei's spikes and the turn they were met, weak turns, reclaim
  loops, and the PROFILE line (scaling Fast/Minus/Problematic, stance Above/On,
  Blight by T4).
- **GROWTH SHAPES**: each per-turn series and the simplest Lumia order that fits.

```
si replay <game> --json > game.json      # the whole record as JSON
si replay <game> --lands                 # the record as text with every land ledger
```

## 5. Every game at once

```
si corpus "$STEAM" $CAPS /mnt/c/Users/brett/AppData/Local/si-companion/steam-capture --md > reports/corpus.md
```

Captures are read first; an `EndGame` log of a game a capture already holds is
dropped, and the same game recorded twice is counted once. The output is the
table in `reports/game-corpus-2026-10-20.md` plus the groupings (wins by
scaling, stance, Blight by T4, adversary difficulty, Blight shape, lands left to
Blight). Drop `--md` for a plain-text table.

## 5b. A game as notation

```
si notate <game>                 # Spirit Island notation: growth, plays, Power>target[results], the Invader phase, Time Passes; legend at the end
si notate <game> --canonical     # engine identifiers instead of short codes (for the simulator)
```

Spec: `docs/superpowers/specs/2026-10-20-notation.md`.

## 5c. Hindsight

```
si hindsight <game>                       # every turn: growth and plays against every legal line from the same position
si hindsight <game> --turn 5 --top 5      # one turn, five alternatives
si hindsight <game> --fixed-growth        # the growth as played; only the plays vary (much faster)
si hindsight <game> --turn 6 --try "SLSF>C8 MI>C1"   # score a line you write (growth as played), beside the actual one
si hindsight "$STEAM/log.txt"             # mid-game: the app's own journal; every finished turn is reviewed
si hindsight "$STEAM/EndGame--n.txt" --seed $CAPS/live-…/saves/<the save you loaded>.gz   # a branch played from a checkpoint
si calibrate [--dry-run]                  # re-measure the constants hindsight has no rule for, over the corpus (section 5f)
```

`log.txt` beside the saves is the app's live journal in the EndGame format, so a game
in progress can be reviewed at any time without the recorder; the turn being played is
not scored until its Invader steps are in the log. A game played on from a reloaded
checkpoint (`si inject`) writes a log that starts at the load, so its Energy, hand,
presence and the cards already out of the Invader deck are unknown to the replay:
`--seed` gives it the save it was loaded from (`si inject` prints the exact flag), and
the turns then carry their real numbers (T4, T5 … not T1, T2).

While it runs, stderr shows one line per turn: a 40-cell bar that fills as the turn's
growth positions are tried (in cyan when color is on), then the counts of positions,
play sets and lines kept, `ranking` while the kept lines are scored over the Explore
cards, and the turn's time once that is done (a full game takes a couple of minutes
with the growth enumerated; `--fixed-growth` is near instant). The report then prints,
per turn:

- the hand, plays and Energy you had, and how many play sets over how many growth
  positions were tried;
- a table in the house style (comfy-table as in talaria-cli: box borders, bold header,
  cells wrap onto more lines rather than being cut; the tag, number, element and tier
  columns never wrap) with the rows `actual`, `best`, `then…`: the growth in notation (`G1 c5@A5 Rc
  +SHG -SFFW`: group, each presence's slot and land, Reclaim, card gained, card
  forgotten; `+MI (draft T4)` when the gain is taken from a later turn's recorded
  draft; `(as played)` marks rows that keep your growth; `+6 alike` folds rows with
  the same plays and score from another placement or slot); the cards played and the
  innates used, each in its own column (`EW GP2@A2 RBST>A2` and `AEG>A2`: `CARD>land`
  targets the land, `CARD@land` places presence there, a card alone acts on the Spirit
  or in all its lands at once; a Spirit-target card that acts in one of the Spirit's
  lands, like Fiery Vengeance, is tried in each presence land and written `FV>C5`;
  innates are underlined in color); the elements as the eight letters
  with counts in printed order (`S1 M2 F1 A0 W1 E1 P3 A1`, each in its element's
  color); each innate's tiers as a fraction (`CTM 2/3` = tier 2 of 3 reached), so a
  better element pairing shows up as a better innate; and what the line produced in
  short tokens (`Bl2 Dh1 Pr1 Pr+1 kill 3E push 1 Fr1 built 1T | next Bl1 built 1T`:
  Blight, Dahan lost, presence lost, presence added, Invaders destroyed, pushed out,
  Fear, Towns and Cities the Build added, next turn), with
  `FLIPS`/`LOSES` when the line's Blight reaches the Blight card's pool; the key to
  every column and token (`KEY`) and the card codes (`CODES`) are tables at the end. No
  ambiguous-width glyphs (dots, ellipses) anywhere, so the columns align;
- `same G`: the best line from the growth you actually took, when another growth ranks
  first;
- four numbers per row: **informed** (the score on what was knowable: averaged over
  the Explore cards still in the stage, the face-down Event and Fear cards counted at
  their measured average worth (section 5f) rather than at what they turned out to do,
  a card taken from a later turn's draft not played), **hindsight** (on the cards that
  came), **luck** (hindsight minus informed: how far the hidden cards ran from their
  expectation) and **conf** (how sure the review is that the line beats yours: under
  each possible Explore card, the chance that its lead outweighs the simulator's own
  error (section 5f; two scores each off by that much), averaged over the cards; a lead
  of one ledger point reads about 70%, of three about 93%). Rows are ranked by the
  informed score, so a line that only works because it "knew" the Explore or the draft
  sinks to the bottom with a low confidence while still being shown. Slow plays are
  committed before the Explore and
  their lands chosen after it, as the player does: the table shows the lands chosen
  under the card that came; under another card the informed score may have placed them
  elsewhere;
- `READING`: one line on the informed scores, with the gap measured against a turn's
  usual luck (the corpus standard deviation, section 5f; Cohen's effect-size bands):
  Sound (under a fifth of it), Unlucky (sound, and the hidden cards cost more than a
  whole swing), Gamble paid (behind on what was known, the hidden cards made it the
  best line), Better available (a fifth to four fifths), or Mistake (more than four
  fifths), with the gap split between growth and plays; `Not judged` when the actual
  or best line plays a card the simulator cannot read in full, or when the log shows a
  card paid for but never used (un-played after an undo);
- `CHOICES`: a small table, one row per prompt the engine raised in the Event, Fear and
  Blight steps (which land takes the Damage, the Blight, the Town, the Strife…; a
  Power's branch): the step, the card, what it asked, the verdict (`best option`,
  `within 0.2 of best`, `better land open, by 1.5`, or `not scored` when the simulator
  cannot place that effect yet) and every option with its score, the chosen one marked
  `<-`;
- `HIDDEN`: labeled lines for what was face down when you chose: `Explore` (the card
  and what else it could have been), `Event`, `Fear` (cards resolved), `Effects` (what
  the Event and Fear cards were worth to your line against what was expected of them,
  the piece part separately), `Draft` (the four cards offered at the gain);
- `FORECAST`: the Invader deck from this Fast phase, one row per coming turn: the stage
  it draws from (the adversary's deck order, lined up with the record), the cards still
  possible, and each land's chance of an Explore, split into `re-hit` (the land already
  holds Invaders or Blight, so a second wave lands on the first) and `fresh`, with the
  expected count of each. A card still in a stage is at any of that stage's future draw
  positions with the same probability, so the chances are exact; which lands can be
  explored is read off the board as this turn's face-up Ravage and Build leave it, so
  later turns undercount lands that later Builds would open;
- `NOTES`: short caveats on the turn: cards whose text the simulator types but does not
  execute (`not simulated: …`), a game that ended mid-turn, an undo that re-logged a
  play, a Slow play whose land was better chosen after the Explore (`Slow targets: 0.8
  better was open…`, counted as its own gap, not as luck).

The report opens with a `GAME` table (source, spirit, board, adversary and level, the
deck order by stage, result, a `record` row when a bare log's undo reloads were placed
one save back) and closes with `KEY`: one short row per term, the ledger's values, the
spirit's own adjustments, the horizon weights and the calibration constants, all taken
from the review itself, so the numbers in the key are the numbers that scored the lines.

How the scoring works: every growth the panel allows from the previous turn's end
state is a position (group; top or bottom slot for each presence; every legal
placement; each card of the offered draft, or the next recorded draft of that deck when
the turn did not gain; the Reclaim). From each position every set of plays within the
Energy and plays is tried at every legal target (first without innates, then the best
40 sets with every innate target), the Fast plays run, the Event and Fear piece effects
are applied as the log shows them, the turn's real Invader steps run (a Town or City
the Build adds counts against the line at the piece's value, so a Build the line stops,
by Disease or by killing the Explorer first, is credited), the Slow cards take the best
land each on the board those steps left, then the next turn's Invader steps at the
calibrated weight (how much of an unattended threat materializes once the Spirit plays
its next turn: 0.33 on the corpus). The lines that make the ranking are
then scored one turn deeper: the turn after next's Ravage (its card is this turn's
Explore) on what the next turn leaves, averaged over the next turn's possible Explore
cards, at the weight squared. The ledger's exchange rates are printed; the per-spirit
part (how much a Dahan or a presence weighs) comes from the spirit's own rules
(`SpiritPanel::priorities`), not from a name.

Presence a Power places (Gift of Proliferation, boons) is the play's, not the
growth's: the record keeps it apart (`SpiritTurn.presence_by_power`), the line writes it
`GP2@A2` and scores it as a growth's worth (the presence and the slot it came off), and
a Sacred Site it makes in the Fast phase is in range for the cards played after it. A
spirit with an always-group (Rampant Green's "Add a Presence to Jungle or Wetland")
places that presence in every growth position before the chosen group's.

Limits: one spirit (the first) is reviewed; a card held for later scores only its
Energy kept; Push and Gather choices, counterattack distributions and the Event's
option choices are listed, not scored; bare logs with many undo reloads can re-log a
play or a payment; a second presence placed in the same growth measures its Range from
the presence before the growth, not from the first one placed; the Event and Fear cards
enter the informed score as averages, not card by card (their effects are not typed);
a line that earns an extra Fear card by its own Fear gets the Fear at the ledger rate,
not the card's average worth.

## 5f. Calibration

```
si calibrate                 # the captures and the Steam folder; writes data/calibration.json
si calibrate <path>... --out <json> --dry-run
```

Four numbers in the hindsight scoring have no rule behind them, so they are measured
over the corpus and written to `data/calibration.json`, which `si hindsight` reads from
beside the definitions folder (`--calibration <json>` names another; without a file the
built-in values are the last measurement). Each line of the report says what it
measured over how many turns:

| Constant | How it is measured | 2026-10-21, 24 games, 149 turns |
|---|---|---|
| Event, Fear card worth | least squares over every turn's hidden effect on the actual line (its hindsight score minus its informed score under the Explore card that came): an Event indicator and the Fear cards resolved, per Terror Level | Event +0.90; Fear card TL1 +0.18, TL2 +0.61, TL3 +0.28 |
| next-turn weight | Blight and Dahan the next turn's Ravage and Build took, over what they would have taken with no Spirit action (the lookahead on the recorded position) | 0.33 over 98 turns |
| a turn's luck | standard deviation of hindsight minus informed on the actual line, with the constants above (mean +0.03: centered) | 1.37 over 148 turns |
| simulator error | RMS, in ledger units, of the recorded state at the Slow phase minus the simulated Invader phase of the actual line (the `conf` column's noise) | 1.40 over 131 turns, mean +0.33 (the simulator runs a little pessimistic) |
| the ledger against results | each game's actual lines summed per turn; win and loss means, and the share of win/loss pairs the ledger orders right | wins -0.66, losses -1.88; 72% of pairs (66% before Builds counted) |

The Event coefficient covers everything face down on an Event turn beyond the Explore
card: the Event itself, the Fear cards' Defends and skips (status effects, not pieces),
and the next turn's cards inside the lookahead. The ledger line is a measurement, not a
fit: with 21 decided games the weights stay as printed; when the corpus passes about a
hundred games they get fitted to win probability (decision-structure spec, section 3a).
Re-run after `si import` or a batch of new captures; the pass takes a few seconds (the
actual line only, three passes).

## 5e. Checkpoints and live replay

```
si checkpoint $CAPS/live-e392726411c9                         # every save the app wrote during the game, with turn and phase
si checkpoint $CAPS/live-e392726411c9 --turn 3 --phase SpiritPhase          # the saves at the start of turn 3's Spirit phase
si checkpoint $CAPS/live-e392726411c9 --turn 3 --phase SpiritPhase --export /tmp/ckpt   # write the first match as the app's .dat
```

Only recorder captures keep the saves (`saves/*.dat.gz`, one per decision point, plus
the app's undo copies); an EndGame log has only its final `.dat`. Each row shows the
save's id (first 12 hex digits of its sha256): copies share it, and a reload that lands
on an existing id is a fork.

```
si import "$STEAM/EndGame--8584090663658034924.txt"      # a game the recorder missed → $CAPS/live-<game id>/
```

`si import` builds a capture after the fact from what the app's folder still holds:
the EndGame log becomes `log-stream.txt`, the game's final `.dat` and every
`save-Feedback`/`save-Undo` of the same game (the app keeps only the current game's
step saves, and clears the undo copies as it goes) go under `saves/`, their write times
into `events.txt`. The game id is the save's engine-seed hash, as the recorder names
captures; a branch of a game already captured (a reloaded checkpoint played on) gets
`live-<id>-<last six digits of the EndGame number>`. So: a game the recorder watched
has every step; an old game gets its log and its final state; the current game gets
its last few steps too. Review a branch with `--seed` (above).

```
si checkpoint $CAPS/live-e392726411c9 --graph            # the checkpoint DAG: distinct saves, forks (undo branches), leaves
si checkpoint $CAPS/live-e392726411c9 --graph --dot | dot -Tsvg > dag.svg
si inject $CAPS/live-e392726411c9 --turn 3 --phase SpiritPhase --fetch   # first time: also downloads frida-inject.exe
si inject $CAPS/live-e392726411c9 --turn 6 --phase SpiritPhase           # load that checkpoint into the RUNNING app, no restart
```

`si inject` prints, with the load, the `--seed` flag that reviews the branch once it
is played out. It needs the app running on Windows, at the home screen or inside a game. It
places the checkpoint in the game's save folder as `save-Undo-990.dat`, runs Frida's
standalone `frida-inject.exe` (fetched once into the companion's Windows tools folder)
against `SpiritIsland.exe` with `tools/live-replay/reload_checkpoint.js`, and follows the
script's log until it reports `done` or an error. At the home screen the app's Continue
picks the file up (its undo fallback); in a game the app reloads in place through its own
Undo path. Click the game window after starting the command: Unity runs no frames while
the window is in the background. Verified 2026-10-20. Mechanism and findings:
`docs/superpowers/specs/2026-10-20-live-replay-checkpoints.md`.

## 5d. Colors

Every command colors its output on a terminal: lands by terrain (Mountain grey,
Sands yellow, Jungle green, Wetland blue), Fast orange and Slow blue-violet, Blight
purple, Fear magenta, Dahan orange-brown, Rei's classes from green (above) to red
(sacrificed, untouched), scores by sign. `--no-color` or `NO_COLOR=1` turns it off;
`--color` or `SI_COLOR=1` forces it on when piping.

## 6. The rules oracle

```
si verify <game>                 # simulate each turn's Invader phase and every Fast and Slow Power use from the recorded state, compare with the log
si verify <game> --level 4       # force the adversary level when no save sits beside the log
si verify <game> --all           # list every mismatch, not only the first 40
```

Each row of the mismatch list names the turn, step (Invader step or Fast/Slow),
card or Power, land, the field, the simulated and the logged value, and the notes
(a player's choice such as the counterattack distribution or an "or" branch, an
unparsed sentence, a rule not modeled). Fields ending in `(player's choice)` are
the ones a choice could explain. `explained by the log` counts outcomes a logged
cancel or status effect accounts for. Bare logs with many undo reloads re-log the
same Power use; captures do not.

## 7. The cards

```
si cards                          # how many Power Cards parse fully, how many are also simulated in full; innate tiers; slot capabilities
si cards --unparsed 60            # the sentences the grammar does not read yet, by frequency (none at the moment)
si cards --innates                # innate tiers the grammar does not read in full (none at the moment)
si cards --card "Strike Low with Sudden Fevers"   # the printed text, each sentence "read as" game words: • simulated, ◦ typed but not simulated, ✗ not read
si cards --features > cards.tsv   # one row per card: cost, speed, range, counted effects
si cards --fit minor              # cost regressed on what the card does (minor | major | unique | all)
```

## 8. The saves themselves

`si-steam-dump` reads a `.dat` save (gzip of .NET binary serialization) directly:

```
cargo run -q -p si-steam --bin si-steam-dump -- sum  "$STEAM/EndGame--<id>.dat"   # decoded state: lands, decks in order, spirits, status effects
cargo run -q -p si-steam --bin si-steam-dump -- hist "$STEAM/EndGame--<id>.dat"   # object classes in the save
cargo run -q -p si-steam --bin si-steam-dump -- tree "$STEAM/EndGame--<id>.dat" . 1   # the root object; path like `Journal` or `_spirits.0` to drill down
```

`sum` shows what the log never says: the adversary level and difficulty, the
remaining Invader, Fear, Event and Power decks in draw order, the Blight card's
state, the engine's status effects and its land-distance cache. Hidden
information is for post-game reading only; the companion never shows it during
play.

## 9. Fixtures and tests worth knowing

- `crates/si-metrics/tests/rei_openings.rs`: Rei's ten builds as growth sequences
  and the labels the classifier must return.
- `crates/si-record/tests/shroud.rs`: the Shroud win as the golden replay.
- `crates/si-defs/tests/defs.rs`: definitions and the card-text parser.
- `crates/si-rules/src/invaders.rs` (unit tests): Ravage, Build, Explore cases.
