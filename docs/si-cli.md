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
```

While it runs, stderr shows one line per turn with a dot per growth position tried
(a full game takes a few seconds on a many-core machine; `--fixed-growth` is near
instant). The report then prints, per turn:

- the hand, plays and Energy you had, and how many play sets over how many growth
  positions were tried;
- `actual`, `best`, `then…` rows: the growth in notation (`G2 e4@C4 c2@C4`, `G1 Rc +MI
  +1E`, `G3 e5@C2 +MI (draft T4) -SLSF +1E` when the gain is taken from a later turn's
  recorded draft and a card is forgotten for a Major; `(as played)` marks rows that keep
  your growth; `+6 alike` folds rows with the same plays and score from another
  placement or slot), then the plays, cards first and innates after `| innate`
  (`SLSF>C2 FV>C4 | innate ERR>C2 SR>C1`; innates are underlined in color), the score,
  and what the line produced;
- under each row, the line's elements in short form (`Su1 Fi3 Ai1 Ea1 An2`: Sun, Moon,
  Fire, Air, Water, Earth, Plant, Animal, each with its count, colored) and the tier
  each innate reaches (`ERR 2/3`, `SR —`), so a better element pairing shows up as a
  better innate;
- `same G`: the best line from the growth you actually took, when another growth ranks
  first;
- four numbers per row: **informed** (the score on what was knowable: averaged over
  the Explore cards still in the stage, the Event and Fear effects left out, a card
  taken from a later turn's draft not played), **hindsight** (on the cards that came),
  **luck** (hindsight minus informed) and **conf** (the share of possible Explore cards
  under which the line beats the actual one). Rows are ranked by the informed score, so
  a line that only works because it "knew" the Explore or the draft sinks to the bottom
  with a 0% confidence while still being shown;
- `READING`: on the informed scores: Sound, Unlucky (right on what you knew, the hidden
  cards cost you), Gamble paid (behind on what was known, the hidden cards made it
  work), Better available, or Mistake on what was known, with the gap split between
  growth and plays; `Not judged` when the actual or best line plays a card the
  simulator cannot read in full, or when the log shows a card paid for but never used
  (un-played after an undo);
- `CHOICES THE GAME ASKED FOR`: every decision the engine prompted for in the Event
  and Fear steps (which land takes the Damage, the Blight, the Town, the Strife…), one
  row each with the effect, the verdict and every option's score, the chosen one
  marked; prompts the simulator cannot model yet are listed under "also asked";
- `HIDDEN WHEN YOU CHOSE`: the Explore card, the Event, the Fear cards, the draft
  offered at the gain, and whether the Event and Fear piece effects were applied from
  the log;
- `NOTES`: caveats on the turn (undo re-logging, horizon).

How the scoring works: every growth the panel allows from the previous turn's end
state is a position (group; top or bottom slot for each presence; every legal
placement; each card of the offered draft, or the next recorded draft of that deck when
the turn did not gain; the Reclaim). From each position every set of plays within the
Energy and plays is tried at every legal target (first without innates, then the best
40 sets with every innate target), the Fast plays run, the Event and Fear piece effects
are applied as the log shows them, the turn's real Invader steps run, the Slow plays,
then the next turn's Invader steps at half weight. The ledger's exchange rates are
printed; the per-spirit part (how much a Dahan or a presence weighs) comes from the
spirit's own rules (`SpiritPanel::priorities`), not from a name.

Limits: one spirit (the first) is reviewed; a card held for later scores only its
Energy kept; Push and Gather choices, counterattack distributions and the Event's
option choices are listed, not scored; bare logs with many undo reloads can re-log a
play or a payment.

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
