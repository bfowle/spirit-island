# si study --live: the study board on the game in progress

Status: design 2026-10-21, approved in chat by Brett the same day (section 9). Follows
the study TUI spec (2026-10-20-study-tui.md), the direction doc's contract
(2026-10-19-direction.md section 2: items 1, 3, 15, 16, 20, 24, 34) and the questions
from play (2026-10-19-questions-from-play.md). Brett's ask: "What do we want to do for
a TUI now with a game in progress... the companion TUI (needs built, similarly, shared
where we can), for what we'd have up and showing us while playing?"

## 1. What it is

`si study --live` is `si study` pointed at the Steam game being played. One binary, one
tab bar, the same nine panels. Every turn already played is a closed tab, reviewed by
the hindsight engine as soon as its Invader steps are in the app's journal, exactly as
`si study <game>` reviews a finished game. The turn being played is one open tab. On it
the LINES panel becomes PLAN: lines the player enters, scored from the open position
against the face-up Ravage and Build cards and the Explore cards still in the stage.

What the open tab never does: rank, highlight, pick a default, or show the engine's
own best lines. It scores what the player proposes, beside "do nothing". The ranked
tree for a turn appears only when the turn closes, as the review. Brett's questions
during play were always "here is my plan at this phase, evaluate it"; the contract
says the companion augments the call and never makes it (direction items 15 and 24,
`feedback_deliberate_play_muscle`).

The companion reads the Steam folder and writes nothing, ever (direction item 34).

## 2. The open tab

The tab bar is the study bar with one more folder: `╭ T4 Growth ╮`, `╭ T4 Fast ╮`,
`╭ T4 Ravage ╮`, `╭ T4 Slow ╮`, the phase in the game's words, drawn in the accent color
(the one color that means "now"). Closed turns keep their verdict labels. Pacing is
unchanged and charts the closed turns.

The phase word by engine state: SpiritPhase = Growth; FastPower = Fast; Event, Fear,
Ravage, Build, Explore = that word; SlowPower = Slow; TimePasses = Time passes;
anything else = the engine's state name.

### PLAN (top left, in place of LINES)

One row per plan, in the order entered, never re-sorted. The first row is always
`none`: nothing beyond what the journal shows taken and played so far this turn.
Columns: `tag` (`none`, `a`, `b`, `c`...), `growth`, `cards`, `innates`, `score`,
`vs none`, `tiers`, `produced`.

- `score` is the informed score: the mean over the Explore cards still in the stage,
  the Event and Fear effects at their calibrated expectation, as hindsight's `informed`.
  The expectation (`Calibration::expected_exogenous(event, fear_cards, terror)`) takes
  the same inputs the review takes from a closed turn, as far as the open turn knows
  them: `event` = true from turn 2 on (the turn-1 Event is discarded unresolved) on
  every tab before Slow; `fear_cards` = the Fear cards the journal shows earned this
  turn so far (`t.fear_cards_earned`) plus the cards the plan's own Fear reaches the
  threshold for (the simulator's Fear against the save's `fear_generated` and
  `fear_required`); `terror` = the Terror Level the turn started at. The term stays in
  force until the scoring board carries the Event's and the Fear cards' effects, which
  is the Slow tab: there it is zero, and nothing is added for the next turn (the
  calibration has no such term). The `Line` records the expectation it included in
  a new field, `expected`, which the review's lines carry too; LINE shows it as
  "Event and Fear cards expected +0.4".
- `score` includes the growth position's value (slots revealed, cards gained, Energy
  kept, as `growth_value` prices them), exactly as the review's informed score of a
  line does: the matched position's on the Growth tab, the position as taken on every
  later tab. The `Line` carries it in `growth_value` and its tokens in `growth`, so
  `vs none` on the Growth tab compares growth options on what they differ in.
- `vs none` is "beats doing nothing under N of M Explore cards", as a count: `7 of 9`.
  It uses the measured simulator error (`model_sd`) the way the review's confidence
  does, so a one-point lead over the whole pool reads about 7 of 9, not 9 of 9. On the
  Slow tab the Explore is known (M = 1) and the column shows the share under the
  simulator's error alone, as `about 3 in 4`.
- `produced` is the ledger in the review's tokens: `Bl1 Dh2 kill 2E | next Bl1`.

`t` opens the prompt for a plan in notation. On the Growth tab a plan may start with
growth tokens before a `|`: `G2 e4@C4 | CS>C3 FCD>C4`; on the Fast tab the growth is
taken and the plan is plays only, `CS>C3 FCD>C4`; on the Slow tab the plays are Slow
plays on the board after the Invader steps. A Slow card written without a land takes
the best land after the steps, as `--try` does today. `d` deletes the plan under the
cursor. `j`/`k` move; the row under the cursor drives POSITION, SPIRIT, HAND and LINE,
as the selected line does on a closed tab.

The growth tokens must name one of the growth positions the panel allows from the
open position (`growth_positions`); the prompt says which token was not legal and
lists the groups the panel offers. A gain-a-card growth is a position with one more
card to come and no card to play from it until the draft is offered; once the journal
shows the draft (`SpiritTurn.offered`), a plan may name one offered card as
`+<card>` and play it this turn at its speed.

Plans are kept per turn. When the state changes (a card played, an undo, the phase
moving on), every plan on the open tab is re-scored from the new position. A play the
journal now records (the same card in the same land) is not applied a second time:
`score_plan` drops it from the plan and LINE marks it `made`; a plan whose card is
gone from the hand another way stays, with the note the review gives today ("CS was
not in hand").

Which board a plan is scored from, by tab: Growth, the save's board, with the plan's
growth position applied; Fast, the journal's `before_fast`; Event, Fear, Ravage, Build
and Explore (the Invader phase running), `before_fast` still, with the Fast plays the
journal shows folded into `none` and the expectation term in force, since no board
after the Event and Fear cards exists until the Slow transition; Slow, the journal's
`before_slow`. On the Invader-step and Slow tabs a plan may name Slow plays only; a
Fast card typed there is refused with "the Fast phase is over".

### THINK ABOUT (under PLAN, in place of READING)

Short sentences in game words, computed from the `none` row's ledger and the hand.
Nothing in it is a verdict or a ranking.

- Each land this turn's Ravage reaches where something is at stake: "C4 Ravages for 6
  with every modifier: 3 Dahan and your presence; Defend 3 keeps the Dahan, Defend 6
  keeps the land." The damage is the simulator's number, adversary and scenario rules
  applied (`feedback_companion_actual_numbers`).
- Each land this turn's Build reaches: "E1 Builds a Town. C7 Builds a City beside 2
  Blight: 9 damage next Ravage."
- The Blight card: "3 in the pool; doing nothing adds 2 this turn."
- Hand against plays: "Hand 2, plays 2: a Reclaim next turn unless this turn's growth
  draws."
- On the Growth tab, when a growth option draws a card: "Of the 61 unseen Minors, 14
  play this turn on C4 (Fast, cost within your Energy, in range); about 1 in 4." The
  pool is every card of that kind in the definitions minus the ones the journal has
  shown (drafted, in any hand, discard or forgotten pile); "plays on the land" means
  the card's parsed effect acts there within its printed range from the open position
  (the same test `candidate_plays` makes). A card the grammar does not simulate is
  counted as not playing and the count says "N not readable".

### POSITION (right column)

The land rows as on a closed tab: terrain, pieces, Dahan, Blight, presence, tokens.
`steps` lists the face-up cards reaching the land this turn (`R`, `B`, `HI`) and the
land's Explore chance from the stage's pool (`X40%`). `plan` marks the selected plan's
plays and presence placements there. The last column is `after` while the step has
not run: what the simulator says the steps do there under the selected plan (`Bl1
Dh2`, `kill 2E`, `+T`); once the journal records the step the column shows `result`,
the recorded outcome, in the same place.

### INVADER PHASE (under POSITION)

- `Ravage: <card> in C1, C4` and `Build: <card> in E1, C7`: the cards in the slots and
  the lands they act in from the board now (the TUI's `step_lands`, which moves with
  the panels in section 6). England's High Immigration slot is listed as `HI` before
  the Ravage.
- `Explore: face down; Stage II, 5 of 9 left` with the terrains those cards name.
- `Blight card: <name>, 3 in the pool; plan b adds 1, leaves 2` for the selected plan.
- `Fear: 2 cards earned, Terror II, 4 of 9 toward the next`.
- `Event: not drawn` until the journal shows it, then the card and each choice it
  asked with what was chosen; the same for Fear cards.
- As each step is recorded the line gains its result beside what the selected plan
  said: `Ravage: C1 Bl1 Dh2, as planned; C4 kill 2E, plan said Bl1` (direction item
  27: what happened against the forecast).

### FORECAST, SPIRIT, GROWTH, HAND, LINE

As on a closed tab, with the selected plan in the selected row's place. SPIRIT lights
each innate's tiers for the plan's elements; GROWTH lists the groups in yield words and
marks the one the journal shows taken, or none yet on the Growth tab; HAND marks which
cards the selected plan plays; LINE is the plan in words. FORECAST already reads the
deck per coming turn from the record.

### After the turn closes

When the journal records Time Passes, the open tab becomes a closed tab: the review
worker scores it, the actual line appears first, the ranked tree under it, and the
plans the player entered stay as `try` rows beneath. The next turn opens as the
journal shows its Spirit phase.

## 3. The live source

A thread polls the Steam folder (`--steam <dir>`, default `steam_dir()`:
`SI_STEAM_DIR` or the Handelabra path) every 500 ms and sends the screen one message,
`Msg::Live(Box<(GameRecord, Option<Seed>)>)`, whenever the journal or the newest save
changed (modification time or length; a rewrite to the same length still changes the
time).

**The journal** (`log.txt`) is the record of the past. Any change re-reads the whole
file and replays it from scratch through `si_record::Replay` (a full game of 1,050
lines replays in 0.3 s in a debug build, measured 2026-10-21). A shorter file is an
undo or a new game; the app rewrites the journal on undo
(`project_companion_log_backfill`), so replaying from scratch is the exact answer and
no tail logic is needed. Closed turns whose `TurnRecord` or `SpiritTurn` changed since
they were reviewed (both derive `PartialEq`) lose their review and are queued again;
the open turn's plans are kept and re-scored. A journal that restarts at setup clears
reviews and plans: a new game.

**The newest complete save** among `save-Feedback-*.dat`, `save-Undo-*.dat` and
`save-ContinueGame.dat` by modification time, decoded as `decode_save` does today (a
partial write fails to decode and is retried next poll). From it, through the firewall
(section 5), a `Seed`: the turn index and phase; the adversary, its level, the
difficulty and scenario (what `apply_save` reads from a sibling EndGame save today);
the Ravage, Build and extra slots; the Blight card and its pool; Fear generated and
required, the banked Fear cards' count and the Terror Level; each spirit's Energy,
hand, play area, discard, forgotten pile and revealed track slots; the status effects
in force; the ending; the board. These fill what the journal never says.

**A journal that joined in progress.** A game played on from an injected checkpoint
(`si inject`) writes a journal that starts at the load, so `rec.meta.from_setup` is
false (the current `log.txt` on 2026-10-21 is one). The replay is then seeded, as
`--seed` does today, with the oldest decision save in the folder (the app keeps step
saves for the current game only, and the first one is written at the first decision
after the load); the footer says "joined at T4 Fast from save-Feedback-003-0089". A
journal that starts at setup needs no seed; the newest save's `Seed` serves the open
tab only.

**Agreement.** The save's turn index and phase name the open tab. The save wins for
what the journal cannot know: the slots, the hand, Energy and revealed track slots,
the Blight pool and the Fear counts, and the board on the Growth tab, where the
journal has no phase-start snapshot yet. On the Fast and Slow tabs every plan is
scored from the journal's phase-start snapshot (`before_fast`, `before_slow`), since
a save written mid-phase already carries the effects of the cards played, which the
`none` row accounts for; using it as the start would apply a played card twice. The
replay's board now is compared land by land with the save's and a difference is
shown in the footer ("journal and save disagree in C4: journal 2E 1T, save 2E"); it
is a warning, not a correction.

**`--replay <capture> [--at Tn/Phase] [--save <file part>]`** feeds the same source
from a recorder capture instead of the Steam folder: the journal prefix up to the
chosen save's stamp in `log-stream.txt` and that save as the newest. `--at` takes the
earliest save matching the turn and phase on the path the game finally took, the start
of that phase as the game went, exactly as `si inject` chooses (`--save` picks another
by name, `--any-branch` admits an undone attempt). It exists for the smoke test and
for judging the screen at a moment without the game running; it is the same code path
with a different directory and clock.

## 4. Engine changes (si-record, si-hindsight)

**The types, and which crate owns each.** Three crates meet at the open turn, so the
seam is one plain-data type in the lowest of them:

- `si_record::replay::Seed` (exists: "what a save says and a log that starts mid-game
  cannot") gains the fields the open tab needs beyond the replay's own: `game_id`,
  `save_name`, `ravage_slot`, `build_slot`, `extra_slots: Vec<(String, Vec<String>)>`
  (England's High Immigration among them), `blight_card: Option<String>`,
  `blight_pool: Option<u32>`, `fear_generated`, `fear_required`, `fear_cards_banked`,
  `status_effects: Vec<StatusEffect>`, `ending: Option<String>`, and per spirit the
  play area it already has. It stays serde-free plain data with si-model ids only.
- `si_steam::seed::from_snapshot(&EngineSnapshot) -> Seed` is today's
  `seed_from_snapshot` moved out of si-cli with `land_id_of` and extended to the new
  fields; si-cli's `seed_from_args` calls it, so `si study <game> --seed` and
  `si hindsight --seed` keep working. si-steam gains dependencies on si-record and
  si-model (no cycle: si-record depends only on si-model and si-defs; si-steam only
  on si-common today).
- `si_hindsight::OpenTurn { rec: Arc<GameRecord>, turn: u32, phase: Phase, t:
  TurnRecord, s: SpiritTurn, seed: Seed, deck: deck::Deck, rules: Rules, cal:
  Calibration }`, built by `si_hindsight::open_turn(rec: Arc<GameRecord>, defs,
  seed: &Seed, cal: &Calibration) -> Option<OpenTurn>`: `None` when the journal has
  no trailing turn, the game has ended, or the save is not at the replay's trailing
  turn (`seed.turn_index + 1 != trailing.turn`, the save's index being 0-based, as
  `checkpoint_matches` compares them). It owns everything the scorer reads, so
  `score_plan` takes it and the definitions alone.

1. **The open turn in the record.** The replay already builds the trailing turn's
   `TurnRecord` and `SpiritTurn` from a journal that stops mid-turn (`si hindsight
   "$STEAM/log.txt"` depends on it): growth once taken, `before_fast` at the Fast
   transition, plays as made, `before_slow` at the Slow transition. What it lacks is
   the face-up cards, which land in `t.ravage` and `t.build` only as each step runs.
   `open_turn` clones the trailing turn and fills `ravage`, `build` and
   `high_immigration` from the seed's slots, sets `before_fast` to the seed's board
   when the phase is still Growth (presence per the plan's growth position is applied
   by the scorer, as `growth_positions` does), and takes the `SpiritTurn`'s hand,
   Energy and revealed slots from the seed when the journal and seed disagree. A turn
   with `explore` empty is an open turn.
2. **The Explore pool of an open turn.** `explore_candidates(rec, defs, t)` returns
   nothing when `t.explore` is empty. It gains a `deck: &Deck` parameter (the review
   builds one with `Deck::for_game` already; `OpenTurn.deck` is the other caller's)
   and takes the stage from `deck::pool` (which already reads `Deck::stage_at(turn)`)
   when no Explore card is recorded, so an open turn averages over the stage's unseen
   cards. The closed-turn path is unchanged.
3. **`score_plan`, and `try_line` unchanged in meaning.** `try_line` today is a
   hindsight number: the recorded Explore card, the recorded Event and Fear effects
   (`invader_phase(..., explore: None, exogenous: true)`) and the recorded next turn.
   It keeps that meaning, since the closed tab's `try` row and `si hindsight --try`
   show it beside hindsight scores. `score_plan(defs, open: &OpenTurn, baseline:
   Option<&Line>, text: &str) -> Result<Line, String>` is the informed counterpart:
   it parses growth tokens before `|` and matches them to one of `growth_positions`
   (error otherwise), parses plays as `try_line` does (the parser is shared), and
   scores the way the review's `simulate(..., informed: true, deep: true)` does: no
   exogenous effects, the Fast plays, then `invader_phase` under each candidate Explore
   card, the Slow plays resolved after the steps, then `lookahead` with
   `informed_build: Some(candidate)` into a projected next turn and the `Deep` term for
   the turn after (the review weighs it at `next_weight` squared in `Outcome::score`).
   `Deep` is built as `deep_for(..., informed: true)` builds it, Ravage = the candidate
   Explore and pool = `deck::pool(turn + 1)` minus the candidate; `deep_for`'s first
   line today returns `None` unless the record has a next turn, a gate the informed
   branch does not need, so it is lifted there and both paths share the helper.
   `informed` = the mean over the candidates plus the position's `growth_value` (as
   the review's `score_under` adds it) plus the expectation of section 2, which the
   line records in `expected`; `confidence` = the share of candidates under which
   the plan beats `baseline` with `model_sd` noise, as the review computes it against
   the actual line (`None` when no baseline is given; the plans thread scores the
   `none` plan first and passes it for the others). The projected next turn is a
   `TurnRecord` with Ravage = the open turn's Build slot, Build = the open turn's
   Explore card once known (`informed_build` handles the unknown case), Explore empty,
   no status effects; adversary extra slots on it are not modeled in v1 and the LINE
   panel says so when one is in play.
4. **The review filter.** `review_turns` and the study worker take closed turns only:
   a turn is closed when its `explore` is recorded or the game ended in it. Today a
   journal cut in the Fast phase would review the open turn with no Invader steps.
5. **The draw count** (THINK ABOUT's last item): `unseen_cards(rec, defs, kind) ->
   Vec<&PowerCard>` in si-hindsight and `plays_on(card, land, open) -> bool` built on
   `candidate_plays`' reach and effect tests.

Nothing else in the engine changes. The scoring of a plan reuses `apply_plays`,
`invader_phase`, `resolve_slow` and `lookahead` as they stand.

## 5. The firewall

The live source reads only these fields of the engine snapshot: `game_id`,
`turn_index`, `state`, `difficulty`, `adversaries`, `scenario`, `lands`, `spirits`
(Energy, hand, play area, discard, forgotten, track nodes), `ravage_slot`,
`build_slot`, `explore_slot_revealed`, `extra_invader_slots`, `invader_discard`,
`fear_generated`, `fear_required`, the count of `fear_earned`, the Terror Level cards
in `out_of_game`, `blight_card`, `status_effects`, `ending`. Never the Invader deck,
the face-down Explore slot, the Fear, Event, Minor or Major decks, or the names of
Fear cards earned face down.

`si_steam::seed::from_snapshot` (section 4) is the only reader, and the firewall test
in `crates/si-steam/tests/firewall.rs` gains `seed_is_invariant_to_hidden_permutations`:
for every capture save, `from_snapshot` is equal before and after `permute_hidden`
(`Seed` derives `PartialEq` for it). The hindsight scorer sees the open turn only
through `Seed`, so no derived number can depend on a hidden order (direction item 16:
"not a cheat, at all").

## 6. The code split (shared where we can)

`crates/si-cli/src/study.rs` is one file of about 1,750 lines whose panel builders
borrow the whole `App`. It becomes a module:

- `tui/mod.rs`: `App`, `Msg`, `Screen`, the event loop, `run(args)` for both
  `si study <game>` and `si study --live`.
- `tui/source.rs`: the two sources behind one trait-free enum: `Source::File` (the
  record built once, as today) and `Source::Live` (the poller thread of section 3).
  Both deliver a `GameRecord`; live, each message carries the newest `Seed` too.
- `tui/review.rs`: the review worker as a long-lived thread fed turn numbers over a
  channel (today it takes a fixed list), so closed turns can be queued and re-queued.
  It sends `Msg::Finished` each time its queue drains, so the footer reads "N turns
  reviewed" on both sources (live, after the source words: "live: T4 Fast, save
  003-0089, journal 1,204 lines | 3 turns reviewed"); the screen harness waits on it.
- `tui/plans.rs`: the plan scoring thread: it builds the `OpenTurn` from the record
  and the seed (`si_hindsight::open_turn`), scores the `none` plan first, then each
  text against it, `(turn, phase, texts) -> Vec<Line>` through `score_plan`; the UI
  thread never scores.
- `tui/tabs.rs`, `tui/pacing.rs`, `tui/text.rs` (wrap, hanging indents, shared
  widths, element spans, the short forms): moved, not changed.
- `tui/panels.rs`: the nine builders over a `TurnCtx` that owns what it needs (the
  record, definitions, codes, the turn's records, the spirit panel, the island, the
  rows and the selected row) and a `TurnKind`: `Closed(&TurnReview)` or
  `Open(&OpenTurn, &[Line])`. A builder takes `&TurnCtx` and a width, never `&App`.
  PLAN and LINES are one builder with the columns chosen by `TurnKind`; THINK ABOUT and
  READING likewise; INVADER PHASE's live lines are one more branch of the same builder.

`si study <game>` must render the same screens after the split as before it. The guard
is the screen harness of section 8, written and blessed on the Shroud capture before
the split begins; the split is done when it is green again. (The study TUI was only
ever smoke-tested by hand under tmux; no harness exists in the repo today.)

## 7. Keys

All study keys stand. `t` is the plan prompt on an open tab (the try prompt on a
closed one). One key is new: `d` deletes the plan or try row under the cursor. `i` is
refused on the open tab with "the game is at this position already". The footer shows
the source ("live: T4 Fast, save 003-0089, journal 1,204 lines") and the review
worker's progress as today.

## 8. Testing

- Engine: a fixture cut from the Shroud capture under
  `crates/si-hindsight/tests/fixtures/open-turn/`: `log-stream.txt` up to the earliest
  T3 SpiritPhase save on the final path (`save-Feedback-002-0047`, before any growth
  decision, so growth tokens are exercised) and up to the T3 FastPower save
  (`save-Feedback-002-0054`), with those two saves as `.dat.gz` (86 KB each). The
  precedent is `crates/si-record/tests/fixtures/shroud-e392726411c9/`, a committed cut
  of the same game's journal and events; this fixture adds two saves, a small step
  beyond it. Tests: `open_turn` fills the slots from the seed and leaves `explore`
  empty; `explore_candidates` on the cut record lists Stage II's unseen cards;
  `score_plan` rejects a growth token the panel does not offer and names the groups;
  the review filter skips the open turn.
- The two paths agree: `score_plan` on the Fast cut, given the actual plays as the
  plan, returns `informed - expected` within 0.01 of the actual line's
  `informed - expected` from `review_turns(full record, actual_only: true, only: [3])`.
  Both average the same candidate set (the stage's unseen cards, the card that came
  among them) with the same `Deep` term, and the projected next turn carries the same
  face-up cards as the recorded T4 when no adversary extra slot is in play, which holds
  for this game. The expectation terms are compared apart, since the open turn counts
  Fear cards from the simulator's own Fear where the review has the recorded count:
  the test asserts the two counts agree on this fixture and says so when they do not.
  It first asserts that both paths run under the same adversary level and the same
  `Deck` offset (the full record takes its level from the capture's EndGame save
  through `apply_save`, the open turn from the seed).
- Firewall: the invariance test of section 5 over every save of the capture
  `firewall.rs::capture_dir()` names (`SI_STEAM_CAPTURE`, else its default; the new
  test skips when the directory is absent, as the existing ones do).
- Source: `Source::Live` over a scratch directory the test fills in stages from a
  capture (journal prefix, then a save, then more journal, then a shorter journal):
  the record's turn count and the open tab's phase after each stage; reviews dropped
  and re-queued after the undo.
- Screen harness: `crates/si-cli/tests/tui_smoke.rs`, an integration test that shells
  out to tmux (Rust drives the binaries, per `feedback_rust_not_python_runners`),
  skipped unless tmux, `SI_DEFS_DIR` and the Shroud capture are present. It starts
  `tmux -L si-smoke new-session -d -x 220 -y 60 "<si> study <args>"`, polls
  `capture-pane -p` until the footer reads "N turns reviewed" (what `Msg::Finished`
  sets today), sends a key script, captures the pane after each key, and compares
  every line but the footer with
  `crates/si-cli/tests/fixtures/tui/<case>.txt`; `SI_TUI_BLESS=1` rewrites the
  fixtures. Cases: the finished Shroud capture with `--fixed-growth` (Turn screen,
  `l`, `Enter`, `e`, `Tab` to Pacing), blessed before the split; then `--live --replay
  <Shroud capture> --at T3/SpiritPhase` and `--at T3/FastPower` (the open tab's label,
  `t` with a plan and its row, `d` removing it, POSITION's `after` column present).

## 9. Decisions (Brett, 2026-10-21)

1. During play: the player's plans and "do nothing" only; the ranked tree is withheld
   until the turn closes. "go with your rec."
2. Plans are entered as notation first; picking cards in HAND and lands in POSITION
   with keys is the next step, not this one. "yes."
3. One app: `si study --live`, not a second binary. "si study --live is fine for now."
4. Several spirits: the first spirit only in v1, as the review does today; a tab per
   spirit is required later. "we need to handle multiple spirits; but we can start
   with just a single for now."

## 10. Not in this build

A tab per spirit; picking targets on POSITION; the Draft and Forget comparison (what a
card does on this board, what it pairs with, what the Forget costs); scoring Event and
Fear choices live (they are listed as the journal records them); adversary extra slots
on the projected next turn; the open turn ghosted on the Pacing charts; an adversary's
rules explained where they bite; the ranked tree during play.
