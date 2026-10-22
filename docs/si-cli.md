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
`si --help` lists the commands; `si <command> --help` its arguments and options.

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
si journal <capture dir | save.dat[.gz]>  # the same record from a save's own journal, no log
si journal <capture dir> --check         # its end against the board the save holds, land by land
si journal <capture dir> --entries       # the journal's entries, one per line
si journal <capture dir> --against-log   # what differs from the log-built record of the capture
```

**The save's journal.** Every decision save carries the engine's own typed journal of
the whole game (`Game.Journal._journalEntries`, the material `log.txt` is written from),
and every command that takes a game (`replay`, `report`, `corpus`, `verify`, `notate`,
`hindsight`, `study`, `journal`) builds the record from the capture's newest save when
the capture holds saves: no log, no undo guessing (an undo reloads a save whose journal
is the truth), no journal copies. The record's source line names the save. An EndGame
log has no save and is read as before; `--log` makes any command read the log of a
capture instead (`si journal --against-log` shows what that changes). `--check` replays it and compares
the record's end with the board the save holds; it matches on every land of the 18
captures with saves, which is the reader's acceptance test. The log-built record was
wrong where the log never carried a fact: presence a Power pushed or gathered, and the
Beasts and Disease Jagged Earth adds to the base boards at setup. Design and findings:
`docs/superpowers/specs/2026-10-21-save-journal-reader.md`.

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
  columns never wrap) with the rows `actual`, `best`, `then…`: the growth in notation (`G1 cp5@A5 Rc
  +SHG -SFFW`: group, each presence's slot and land, Reclaim, card gained, card
  forgotten; `+MI (draft T4)` when the gain is taken from a later turn's recorded
  draft; `(as played)` marks rows that keep your growth; `+6 alike` folds rows with
  the same plays and score from another placement or slot); every play in one column,
  cards and innates together, in the order they act: the Fast ones as played (the actual
  line in the order the journal has them), then the Slow ones after the Invader steps
  (`EW GP2@A2 {AEG>A2} RBST>A2`: `CARD>land` targets the land, `CARD@land` places
  presence there, a card alone acts on the Spirit or in all its lands at once; a
  Spirit-target card that acts in one of the Spirit's lands, like Fiery Vengeance, is
  tried in each presence land and written `FV>C5`; an innate in braces, `F:` before an
  innate taken Fast through its "may be Fast" tier, gathers and pushes in brackets, and
  `(WW)` a card played for its elements only, where no land gains from it this turn:
  the line scores the same without its effect, in hindsight and under every Explore
  card); the elements as the eight letters
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

What the simulator plays out beyond the cards' text (2026-10-22, from Brett's
Thunderspeaker game, checked step by step against the save journal):

- **Spirit rules**: Thunderspeaker's presence goes along with each Dahan moved out of
  its land (Ally of the Dahan; the player never kept it back in the recorded games), and
  after a Ravage destroys Dahan, one presence within range 1 per Dahan is destroyed
  (Sworn to Victory: the ravaged land's first, then a neighbor's without Dahan, the
  fewest first, as the player chose both times). Shroud gathers one presence toward a
  Power's land first (Mists Shift and Flow: where the record says, else into the target).
- **Cards**: Manifestation of Power and Glory deals Dahan times presence as one amount;
  Lead the Furious Assault destroys a Town per 2 Dahan (a City per 3); Words of Warning's
  Dahan strike back as they fall; Boon of Swarming Bedevilment defends every land of the
  target Spirit.
- **Choices**: Damage, a Power's or the Dahan's, goes to the most valuable set it destroys
  (a Town worth three Explorers, a City five: two Towns over a City for 4 Damage). An
  "or" takes the branch the journal recorded, else the one the line's brackets ask for,
  else the first that does not wait on a Dahan move (Voice of Thunder without a bracket
  takes its 2 Fear). A Dahan push the line does not place is not taken. Where Dahan go is
  searched one choice at a time: a gather's source (the fullest-first default, or every
  Dahan of one neighbor), a second source, then the push's destination or none; for a
  Slow play when its land is chosen after the Invader steps, for a Fast one on the ranked
  lines, and an innate that moves Dahan which a ranked line leaves out is tried at every
  land with its moves chosen (a line without Gather the Warriors found no use for it). An
  enumerated line moves its Dahan before its other Slow plays. For a spirit whose presence
  moves with the Dahan, an innate is offered on a land next to its presence too (Lead the
  Furious Assault, range 0, on the land Gather the Warriors filled) and acts only if the
  presence got there. The innates go on the best 40 card sets and on the best set of each
  element reach (the four Air that take an innate Fast), up to 150 innate combinations each.
- **Loss conditions**, every one, checked after the Fast phase and after every Invader
  step, in this turn, the next turn's lookahead and the turn after next's: the core ones
  (the Spirit's last presence gone; the flipped Blight Card's last Blight taken, from the
  capture's saves, per turn) and each adversary's own at every level: England's Proud &
  Mighty Capital (7 Town/City in a land), France's Sprawling Plantations (more Towns than
  7 per board), Scotland's Trade Hub (Coastal lands with a City over 2 per board),
  Russia's Hunters Swarm the Island (Beasts destroyed by the Hunters, counted over the
  game from the record, over the Beasts left), the Habsburg Monarchy's Irreparable Damage
  (Blight off the card in Ravages of 6+ overkill, over the game, over the players) and
  Habsburg Mining's Land Stripped Bare (8 Invaders and Blight in a land at the end of the
  Fast phase). A line that loses scores the ledger's loss (-25, under every line that does
  not); the next turn's loss with no Spirit action counts at the next-turn weight, the
  turn after next's at its square times the share of Explore cards that bring it. Rows
  say `LOSES`, `| next LOSES`, `| then LOSES 33%`; `si hindsight` prints `LOSS` with the
  reason. **Loss clocks**: each condition in play as a count against its limit (`England's
  capital: A1 has 5 of 7 Town/City`, `Blight Card: 2 left (flipped)`, `Russia's Hunters:
  1 Beasts destroyed, 1 on the island`), at the top of THINK ABOUT for the turn being
  played, in READING for a closed turn, `CLOCKS` in `si hindsight`. THINK ABOUT also says
  when doing nothing loses, and, when every card left in the stage escalates, where
  England's Building Boom builds next. The Invader deck running out is not checked.
- **Turns**: the turn's Defend, skips and Words of Warning end before the next turn's
  steps (they used to carry into the lookahead); a line starts from the presence at the
  Fast phase, not the turn's end; the record's Power Defends are no longer credited to
  every line as if an Event gave them.

Limits: one spirit (the first) is reviewed; a card held for later scores only its
Energy kept; gather sources without a bracket are the fullest adjacent lands; Event
option choices are listed, not scored; a spirit rule not named above is not simulated
(Rampant Green's presence destroyed to stop a Ravage, Many Minds' presence as Beasts,
a Power repeated by Gift of Twinned Days); bare logs with many undo reloads can re-log a
play or a payment; a second presence placed in the same growth measures its Range from
the presence before the growth, not from the first one placed; the Event and Fear cards
enter the informed score as averages, not card by card (their effects are not typed);
a line that earns an extra Fear card by its own Fear gets the Fear at the ledger rate,
not the card's average worth.

## 5f. Calibration

```
si calibrate                 # the captures and the Steam folder; writes data/calibration.json
si calibrate <path>... --out <json> --dry-run
si calibrate --dry-run --turns   # also each turn's simulator error, to see which turns a change moved
```

Four numbers in the hindsight scoring have no rule behind them, so they are measured
over the corpus and written to `data/calibration.json`, which `si hindsight` reads from
beside the definitions folder (`--calibration <json>` names another; without a file the
built-in values are the last measurement). Each line of the report says what it
measured over how many turns:

| Constant | How it is measured | 2026-10-22, 27 games, 167 turns |
|---|---|---|
| Event, Fear card worth | least squares over every turn's hidden effect on the actual line (its hindsight score minus its informed score under the Explore card that came): an Event indicator and the Fear cards resolved, per Terror Level | Event +0.40; Fear card TL1 +0.44, TL2 +0.80, TL3 +2.23 (3 turns only) |
| next-turn weight | Blight and Dahan the next turn's Ravage and Build took, over what they would have taken with no Spirit action (the lookahead on the recorded position) | 0.34 over 114 turns |
| a turn's luck | standard deviation of hindsight minus informed on the actual line, with the constants above (centered: its mean is near zero) | 1.35 over 144 turns (mean +0.07) |
| simulator error | RMS, in ledger units, of the recorded state at the Slow phase minus the simulated Invader phase of the actual line (the `conf` column's noise) | 1.48 over 150 turns, mean +0.28 (the simulator runs a little pessimistic; 1.44 before the 2026-10-22 rules on the same corpus, where an old bug credited the record's Power Defends to every line and hid other spirits' unsimulated rules) |
| the ledger against results | each game's actual lines summed per turn; win and loss means, and the share of win/loss pairs the ledger orders right | wins -0.69, losses -4.92; 79% of pairs, losses scored (73% before, 66% before Builds counted) |

The constants are measured with a lost game scored at zero (`ReviewOptions::score_losses`):
they describe what the hidden cards and the simulator do to pieces, and one loss, a step of
25, would swamp a turn's usual swing. The ledger against results is measured with losses
scored, and passes each capture's Blight Card per turn.

The Event coefficient covers everything face down on an Event turn beyond the Explore
card: the Event itself, the Fear cards' Defends and skips (status effects, not pieces),
and the next turn's cards inside the lookahead. The ledger line is a measurement, not a
fit: with 21 decided games the weights stay as printed; when the corpus passes about a
hundred games they get fitted to win probability (decision-structure spec, section 3a).
Re-run after `si import` or a batch of new captures; the pass takes a few seconds (the
actual line only, three passes).

## 5g. Study: the review as screens

```
si study <game>                           # the hindsight review, turn by turn, in the terminal
si study <game> --fixed-growth            # growth as played: the review lands in seconds
si study "$STEAM/EndGame--n.txt" --seed <save>   # a branch played from a checkpoint
```

`si study` opens the same review `si hindsight` prints as screens to move through
(ratatui, the stack adroit uses). The record draws at once; the review runs in a
worker thread one turn at a time and each turn appears as it finishes, the footer
showing the turn and the growth positions tried so far.

- A tab bar across the top of both screens: each turn as a folder tab with its verdict
  (`sound`, `better 0.7`, `MISTAKE 3.6`, colored green, yellow, red), the current turn
  open into the screen below, and the Turn and Pacing screens as tabs at the right end.
- **Turn** screen: nine panels. On a terminal 150 columns or wider the
  decision runs down the left (LINES, READING, LINE), the island down the right
  (POSITION, INVADER PHASE, FORECAST) and the spirit's resources across the bottom
  (SPIRIT, GROWTH, HAND), each panel as tall as its content; narrower, the same panels
  stack in rows. LINES: the actual line first, then each growth (`best`, `then`) carrying
  its best play set's numbers and `[N play sets]`, under it each play set by card names
  with `[N targetings]`, under that each targeting with its lands; the best growth
  starts open to its play sets; Enter or Space opens or closes the group under the
  cursor, `e`/`c` open or close them all. READING: the verdict with the decision, luck
  and gap numbers and what the hidden cards were worth. LINE: the selected row in words.
  POSITION: the island at the Fast phase, one row per land with its pieces, the Invader
  steps reaching it this turn (`R B X50%`), the selected row's plays aimed there (`line`,
  the row lit) and what the steps did there (`result`). INVADER PHASE: the face-up cards
  and their lands, the Explore's pool and the card that came, the Event and Fear cards
  with the choices they asked and each one's verdict, the Blight card, the board after.
  FORECAST: the deck per coming turn. SPIRIT: Energy, plays, track elements, presence,
  and each innate's tiers lit where the selected row reaches them. Elements are always
  the eight in printed order (Sun Moon Fire Air Water Earth Plant Animal) as `Su1 Mo3
  Fi1 Ai0 Wa1 Ea0 Pl5 An0`, each in its element's color with a zero count dimmed; an
  innate's threshold lists only the elements it needs. GROWTH: the panel's
  groups in yield words, the one taken and the best line's, the draft offered. HAND:
  every card with cost, speed, range, target, elements and who plays it (actual, row,
  both). `h`/`l` or the arrows move between turns, `1`-`9` jump, `j`/`k` move along the
  rows; the row under the cursor drives the POSITION, SPIRIT, HAND and LINE panels.
- **Pacing** screen (`Tab`): six charts over the turns from the record and the review:
  Blight on the island, added per turn and left on the card (the loss clock); Explorers,
  Towns, Cities and lands with Invaders; Fear cards per turn and cumulative with the
  Terror Level (the win clock); the spirit's Energy, plays, cards played and presence on
  the island (Rei's curve); your line's informed and hindsight against the best informed
  line per turn; and the decision gap and luck per turn as bars (luck green when the
  cards helped, red when they cost). Block and half-block markers only.
- `t` opens a prompt for a line in notation (`SLSF>C8 MI`); it is scored like `--try`
  and shown as a `try` row. A line is one sequence, cards and innates together, and the
  plays act in the order written: `GW>A5[>A1x2] SA>A5[<A1]` pushes two Dahan to A1 and
  then gathers one back, where the other order would find A1 empty. A play's phase is
  its printed speed: `WW>A7` is Fast, `VT` a Slow card that takes its land after the
  Invader steps. `F:LFA>A4` takes an innate printed Slow in the Fast phase through its
  "This Power may be Fast" tier (Thunderspeaker's 4 Air), and the line is refused with
  the reason when its elements do not reach that tier. On the screen the style says
  what a play is: a Fast play orange, a Slow one blue-violet (the CLI's two colors; the
  SPIRIT panel's innate speeds and HAND's F and S too); an innate is underlined; a card
  played for its elements only is dim and has no land (LINE says so). Plain text
  (`si hindsight`, a plan carried from a review) writes an innate in braces
  (`{GW>A5}`), the Fast use of a Slow innate with `F:`, and an elements-only card in
  parentheses (`(WW)`, which a typed line may use too).
  A Power that gathers or pushes names the lands in brackets after its target:
  `GW>A1[<A3,<A5,>A4x2]` is Gather the Warriors on A1 taking one Dahan each from A3
  and A5, then pushing two to A4 (a count after `x`, one when there is none; spaces
  between the moves are read too). In a review's line the bracket is what the simulator
  did; typed, it is the whole of what the Power moves (`GW>A5[]` moves nothing), and a
  land the Power cannot reach is skipped. Without a bracket the simulator gathers from
  the fullest adjacent land and takes no Dahan push (where Dahan go is the player's call),
  and says so in the line's notes; an "or" card without one takes its other branch
  (`VT>A1` is Voice of Thunder's 2 Fear, `VT>A5[>A1x4]` its push). `i` loads the current turn's start-of-turn checkpoint into
  the running game by running `si inject <capture> --turn n --phase SpiritPhase` as a
  child process and shows what it printed; a bare EndGame log has no checkpoints and
  the panel says so. `?` shows the keys, `q` quits.

Spec: `docs/superpowers/specs/2026-10-20-study-tui.md`. Not yet: the board drawn from
the recorded snapshots, picking a target on it, several spirits.

## 5h. Live: the study board on the game being played

```
si study --live                                   # the Steam folder ($SI_STEAM_DIR or the Handelabra path)
si study --live --steam <dir>                     # another folder
si study --live --replay <capture> --at T3/SpiritPhase [--save <name part>]   # a capture cut at that save, no game needed
```

Turns already played are the closed tabs, reviewed as soon as the journal has moved
past them. The turn being played is the open tab (`T4 Growth`, `T4 Fast`, `T4 Ravage`,
`T4 Slow`, in cyan; the header reads LIVE). On it:

- **PLAN** (in LINES' place; its title counts `plans scored: N`): `none` first, then
  every plan you typed, in that order, never ranked. `t` opens the prompt: on the Growth
  tab a growth before `|` and plays after it (`G2 e5@D3 cp3@D7 | CS>C3`; a growth the
  panel does not offer is refused in its own row, and LINE's note lists the groups and
  example positions to copy); on every later tab plays only; a Fast card after the Fast
  phase is refused. `score` is the informed score: the mean over the Explore cards still
  in the stage, the growth position's value and the expectation for the face-down Event
  and Fear cards included. `vs none` is how often the plan beats doing nothing, as
  `4 of 5` Explore cards (or `about 3 in 4` when the Explore is known). `d` deletes the
  plan under the cursor. A play the journal has since recorded is not applied twice
  (LINE notes it `made already`). The engine's own ranked lines never show on the open
  tab; they appear when the turn closes, and the plans you typed stay as `try` rows.
- **THINK ABOUT** (in READING's place): each Ravage land with the simulator's damage,
  what is at stake and the Defend that keeps the Dahan or the land; the Builds; the
  Blight card's pool and what doing nothing adds; hand against plays; on the Growth tab
  how many unseen Minors and Majors would play this turn on each Ravage land.
- **POSITION**'s last column is `after` (the land as this turn leaves it under the
  selected plan, every adversary modifier applied; not the next turn's steps) until the
  step is recorded, then the recorded result.
- **INVADER PHASE** lists the slots and their lands, the Explore's pool, the Blight card,
  the Fear counts, the Event once drawn, and each recorded step against what the plan said.
- The footer reads `live: T4 Fast, save 003-0089-FastPower, journal 1,204 lines | 3 turns
  reviewed`, and names a land where the journal's board and the save's disagree.

Reads the Steam folder only: the journal `log.txt` (replayed from scratch on every
change; an undo rewrites it) and the newest complete save, through the firewall
(`si_steam::seed`: no deck order, no face-down card name ever crosses; the Blight card
is named only once it has flipped). First spirit only in this version.

**The record comes from the newest save.** When the newest decision save decodes and its
journal reaches back to setup, the screen's record is built from that journal (the footer
says `save journal`); `log.txt` is the fallback for a folder without a usable save. A
turn closes on the first save after its Time Passes, as the app saves at every decision.

**A rollback keeps the attempt.** When the record shrinks onto a turn that was closed and
reviewed (`i`, or the app's own Undo across turns), that turn opens with the earlier
attempt's lines as its first plans: the line you played and the review's three best,
each scored against the position as it stands and noted `last attempt: played, scored
-2.5 then` or `last attempt: best then, scored -1.2`. A carried line keeps its Slow
lands and its gathers and pushes (the same cards come again), and the plays the turn
has made already are matched by card, so a card played and not yet used takes the
plan's land instead of showing twice. Lines carry only onto the same position: when an
earlier turn went otherwise, the status line says the attempt began elsewhere and
nothing is carried. `d` removes one, `t` adds yours. A
different game id is a new game and clears everything typed. When the rollback happened
before the screen was up (the app was relaunched, or the screen restarted), name the
branch's save: `si study --live --attempt <capture>/saves/<the EndGame or last save of
that branch>` reviews that attempt's turn as soon as the live game opens it and seeds the
plans the same way. To read the whole branch, open its save itself:
`si study <capture>/saves/<that save>` (every turn it holds, reviewed; `i` there still
loads a turn's start from the capture).

**It records as it watches.** The app keeps only the last turn's decision saves, so the
live screen copies every save it sees into `$SI_CAPTURE_DIR/live-<game id>/` (`saves/`
as the recorder names them, `events.txt`, the journal as `log-stream.txt`, and Unity's
`Player.log` as `player-stream.txt`: the app's own debug log, with each action's source
and activation, the intents it puts to the player, and its save bookkeeping; it restarts
with the app, so the stream holds one copy per launch), the same capture the old
companion's recorder wrote. A screen restarted over the same game appends only what
the streams lack. On the open tab, THINK ABOUT leads with what the app is asking the
player right now, read from `Player.log` (`The app asks: Use Power: Sudden Ambush
(Fast).`, or `The app's last step: ...` once answered): only the intent lines are read,
the activation's name and its public attributes, never a deck. That capture is what `i` on a closed tab
loads: `si inject <capture> --turn n --phase SpiritPhase` puts that turn's start into
the running game, and the journal that restarts at the load is seeded from the very
save injected, so the branch you then play keeps its real turn numbers and its own
tabs. A turn that began before the screen was up has no start to load, and `i` says so.
Keep `si study --live` up from the first turn of a game you may want to replay.

The trap behind the 2026-10-21 reports of a branch the screen never saw: the app closes
`log.txt` when a game ends (`Closing friendly log file` in its `Player.log`, from
`LogRecorder.CloseFriendlyLogFileOutput`) and opens it again only when the game view
starts afresh (`GameViewController.Start`, which truncates it). A reload in place after
that, the app's own Undo at the end screen or `i` inside the ended game, keeps saving
but journals nothing, so such a branch existed only as the saves the capture keeps.
`si inject` now calls `LogRecorder.InitializeFriendlyLog(path, append: true)` before its
in-place reload, so the journal continues in the same file; a reload from the main menu
takes the Continue route, where the app writes a fresh journal that the screen stitches
to the capture's earlier copy, seeding it from the last save before its first line. The
footer still names a journal that stopped while saves go on. Every one of those saves
carries the engine's own typed journal of the whole game (ChangeState, UsePower,
MoveIslandPieces, AddIslandPieces, DestroyPieces, AdjustFearGenerated, PlayPowerCard,
UseGrowthOption, MovePresence, BuildLand, MoveCards, GainElements, ...), the material
`log.txt` itself is written from, and the record is read from it (section 4, the footer's
`save journal`), with `log.txt` as the fallback; that retired the undo guessing too.

The screen harness (`cargo test -p si-cli --test tui_smoke`, tmux driven, blessed panes
under `crates/si-cli/tests/fixtures/tui/`, `SI_TUI_BLESS=1` to rewrite them) covers the
study screens on the Shroud capture and the live screens on its T3 Growth and T3 Fast
cuts (`crates/si-hindsight/tests/fixtures/open-turn/`). The firewall tests need a folder
of plain `.dat` saves: `SI_STEAM_CAPTURE=<dir> cargo test -p si-steam --test firewall`
(gunzip a capture's `saves/*.dat.gz` into it).

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
Undo path, after the script reopens the app's journal in append mode (the app closes
`log.txt` at a game over and would otherwise play on with no journal; section 5h). Click the game window after starting the command: Unity runs no frames while
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
