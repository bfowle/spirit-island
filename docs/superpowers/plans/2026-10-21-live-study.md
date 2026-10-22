# si study --live Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `si study --live` opens the study TUI on the Steam game being played: closed turns reviewed as today, the open turn scoring the player's plans beside "do nothing".

**Architecture:** The study module splits into `tui/` modules whose panels read a `TurnCtx` instead of the app. The engine gains an `OpenTurn` built from the journal's trailing turn plus a `Seed` the newest save fills through the firewall, and `score_plan`, the informed scorer for a typed plan. A poller thread feeds the app records and seeds; a review worker and a plan worker score in the background.

**Tech Stack:** Rust 2024 workspace in `/home/brett/repos/spirit-island-steam-companion` (branch `companion-v2`), ratatui 0.30, crossterm 0.29, tmux 3.4 for the screen harness. Spec: `docs/superpowers/specs/2026-10-21-live-study.md`.

---

## Conventions for every task

- Work in `/home/brett/repos/spirit-island-steam-companion`. Build and test offline: `cargo build -q --offline -p si-cli`, `cargo test -q --offline -p <crate>`.
- `export SI_DEFS_DIR=/home/brett/repos/spirit-island-steam-companion/data/engine-defs` and `export SI_CAPTURE_DIR=/home/brett/repos/spirit-island/data/steam-capture` before running the binary or the screen harness. The Shroud capture is `$SI_CAPTURE_DIR/live-e392726411c9`.
- Commit messages: lowercase, the house style of `git log` (`si study: ...`, `hindsight: ...`). No attribution trailers of any kind.
- American spelling everywhere. No `…`, `·`, `●`, `○`, `→` in anything the TUI draws.
- Never write to the Steam folder. The live source only reads.

## File structure

| File | Responsibility |
|---|---|
| `crates/si-cli/tests/tui_smoke.rs` | the screen harness: tmux drives `si study`, panes compared with blessed fixtures under `crates/si-cli/tests/fixtures/tui/` |
| `crates/si-cli/src/tui/mod.rs` | `App`, `Msg`, `Screen`, `run`, the event loop, header, footer, help, popups, inject |
| `crates/si-cli/src/tui/tabs.rs` | the folder tab bar |
| `crates/si-cli/src/tui/text.rs` | wrapping, hanging indents, shared widths, `Panel`, element spans, short forms |
| `crates/si-cli/src/tui/panels.rs` | `TurnCtx`, `TurnKind`, `draw_turn`, the nine panel builders |
| `crates/si-cli/src/tui/pacing.rs` | the Pacing charts |
| `crates/si-cli/src/tui/source.rs` | `Source::{File, Live, Replay}`: the record and seed delivery, the poller, the capture cut |
| `crates/si-cli/src/tui/review.rs` | the long-lived review worker fed turn numbers |
| `crates/si-cli/src/tui/plans.rs` | the plan scorer thread: `OpenTurn`, `none` first, each plan against it |
| `crates/si-record/src/replay.rs` | `Seed` extended with the open tab's fields |
| `crates/si-steam/src/seed.rs` | `from_snapshot`: the only reader of a save on the live path |
| `crates/si-steam/tests/firewall.rs` | `seed_is_invariant_to_hidden_permutations` |
| `crates/si-hindsight/src/live.rs` | `OpenTurn`, `open_turn`, `score_plan`, `unseen_cards`, `plays_on` |
| `crates/si-hindsight/src/lib.rs` | `explore_candidates` on an open turn, `deep_for` ungated, `Line.expected`/`per_card`, `Outcome.after`, `parse_plays`, `recorded_plays`, the closed-turn filter |
| `crates/si-hindsight/tests/open_turn.rs` + `tests/fixtures/open-turn/` | the Shroud cuts at T3 Growth and T3 Fast with their saves |
| `docs/si-cli.md` | section 5h: `si study --live` |

---

### Task 1: the screen harness, blessed before anything moves

**Files:**
- Create: `crates/si-cli/tests/tui_smoke.rs`
- Create: `crates/si-cli/tests/fixtures/tui/study-turn.txt`, `study-l.txt`, `study-enter.txt`, `study-e.txt`, `study-pacing.txt` (blessed by the test)

- [ ] **Step 1: Write the harness**

```rust
//! The study screens under tmux: start `si study` in a detached session, wait for the
//! review to finish, send keys, capture the pane after each one, and compare every line
//! but the footer with a blessed fixture. Skipped when tmux, SI_DEFS_DIR or the Shroud
//! capture are absent. `SI_TUI_BLESS=1` rewrites the fixtures.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const SOCKET: &str = "si-smoke";

fn capture_dir() -> PathBuf {
    let root = std::env::var("SI_CAPTURE_DIR").unwrap_or_else(|_| "/home/brett/repos/spirit-island/data/steam-capture".into());
    Path::new(&root).join("live-e392726411c9")
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tui")
}

fn ready() -> bool {
    if Command::new("tmux").arg("-V").output().is_err() {
        eprintln!("skipping: no tmux");
        return false;
    }
    if std::env::var_os("SI_DEFS_DIR").is_none() {
        eprintln!("skipping: SI_DEFS_DIR not set");
        return false;
    }
    if !capture_dir().join("saves").is_dir() {
        eprintln!("skipping: the Shroud capture is not present");
        return false;
    }
    true
}

fn tmux(args: &[&str]) -> String {
    let out = Command::new("tmux").arg("-L").arg(SOCKET).args(args).output().expect("tmux runs");
    String::from_utf8_lossy(&out.stdout).to_string()
}

struct Session;

impl Session {
    /// `si study <args>` in a detached 220x60 session on this test's own tmux socket.
    fn start(args: &[&str]) -> Session {
        let _ = tmux(&["kill-server"]);
        let cmd = format!("{} study {}", env!("CARGO_BIN_EXE_si"), args.join(" "));
        tmux(&["new-session", "-d", "-x", "220", "-y", "60", &cmd]);
        Session
    }

    fn pane(&self) -> String {
        tmux(&["capture-pane", "-p"])
    }

    /// A tmux key name (`l`, `Enter`, `Tab`), then a beat for the redraw.
    fn key(&self, key: &str) -> String {
        tmux(&["send-keys", key]);
        std::thread::sleep(Duration::from_millis(500));
        self.pane()
    }

    fn wait_for(&self, needle: &str, timeout: Duration) -> String {
        let start = Instant::now();
        loop {
            let p = self.pane();
            if p.contains(needle) {
                std::thread::sleep(Duration::from_millis(300));
                return self.pane();
            }
            assert!(start.elapsed() < timeout, "timed out waiting for {needle:?}; pane:\n{p}");
            std::thread::sleep(Duration::from_millis(300));
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = tmux(&["kill-server"]);
    }
}

/// Every line but the footer: the last line carries timings and progress.
fn body(pane: &str) -> String {
    let lines: Vec<&str> = pane.trim_end_matches('\n').lines().collect();
    lines[..lines.len().saturating_sub(1)].iter().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n")
}

fn check(case: &str, pane: &str) {
    let path = fixtures().join(format!("{case}.txt"));
    let got = body(pane);
    if std::env::var_os("SI_TUI_BLESS").is_some() {
        std::fs::create_dir_all(fixtures()).unwrap();
        std::fs::write(&path, &got).unwrap();
        return;
    }
    let want = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("no fixture {}; run once with SI_TUI_BLESS=1", path.display()));
    if got != want {
        let diffs: Vec<String> = got
            .lines()
            .zip(want.lines())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .take(5)
            .map(|(i, (a, b))| format!("line {}:\n  got  {a}\n  want {b}", i + 1))
            .collect();
        panic!("{case}: the pane differs from the fixture ({} vs {} lines)\n{}", got.lines().count(), want.lines().count(), diffs.join("\n"));
    }
}

#[test]
fn study_screens_on_the_shroud_capture() {
    if !ready() {
        return;
    }
    let cap = capture_dir();
    let s = Session::start(&[cap.to_str().unwrap(), "--fixed-growth"]);
    let pane = s.wait_for("turns reviewed", Duration::from_secs(180));
    check("study-turn", &pane);
    check("study-l", &s.key("l"));
    check("study-enter", &s.key("Enter"));
    check("study-e", &s.key("e"));
    check("study-pacing", &s.key("Tab"));
}
```

- [ ] **Step 2: Bless the fixtures**

Run: `cd /home/brett/repos/spirit-island-steam-companion && SI_TUI_BLESS=1 cargo test -q --offline -p si-cli --test tui_smoke -- --nocapture`
Expected: `test result: ok. 1 passed`; five files under `crates/si-cli/tests/fixtures/tui/`. Open `study-turn.txt` and confirm it shows the T1 tab open, the LINES table and the nine panels (not a blank or an error).

- [ ] **Step 3: Run it twice more without blessing, to prove the panes are stable**

Run: `cargo test -q --offline -p si-cli --test tui_smoke` (twice)
Expected: `1 passed` both times. If a line differs between runs, it is nondeterministic output; find the source (a timing, a thread order) and mask that line in `body()` the way the footer is masked, then re-bless. Report what was masked.

- [ ] **Step 4: Commit**

```bash
git add crates/si-cli/tests/tui_smoke.rs crates/si-cli/tests/fixtures/tui/
git commit -m "si study: a tmux screen harness with blessed panes on the Shroud capture"
```

---

### Task 2: split study.rs into tui/ modules, no behavior change

**Files:**
- Create: `crates/si-cli/src/tui/mod.rs`, `tabs.rs`, `text.rs`, `panels.rs`, `pacing.rs`
- Delete: `crates/si-cli/src/study.rs`
- Modify: `crates/si-cli/src/main.rs` (`mod study;` and the `"study"` arm)

The line ranges below are of `study.rs` at commit `2fbf8e0` (1,749 lines); check them with `grep -n` before cutting, since Task 1 did not touch the file.

- [ ] **Step 1: Cut the file into the modules**

```bash
cd crates/si-cli/src && mkdir tui
S=study.rs
# mod.rs: the crate doc, imports, Msg, Screen, App, RowView, Node, power_names, tree, impl App, run, event_loop, progress_words, inject, draw, centered, draw_header, draw_footer
{ sed -n '1,29p' $S; sed -n '30,432p' $S; sed -n '441,501p' $S; } > tui/mod.rs
# tabs.rs: verdict_short (its doc at 502), draw_tabs
sed -n '502,579p' $S > tui/tabs.rs
# text.rs: ACCENT, header_style, wrap_text, Panel + PanelBody, panel_block .. share_widths, hang
{ sed -n '435,440p' $S; sed -n '580,606p' $S; sed -n '621,933p' $S; sed -n '1567,1579p' $S; } > tui/text.rs
# panels.rs: TurnCtx, impl TurnCtx, draw_turn and the nine builders
{ sed -n '608,620p' $S; sed -n '934,1566p' $S; } > tui/panels.rs
# pacing.rs
sed -n '1582,1729p' $S > tui/pacing.rs
# the two unit tests (1730-1749) are not in any cut: 1734-1739 (wrap) go to text.rs, 1741-1748 (centered) to mod.rs, each in its own `#[cfg(test)] mod tests { use super::*; ... }`
git rm -q $S
```

- [ ] **Step 2: Wire the modules and fix visibility until it builds**

In `tui/mod.rs` add after the imports:

```rust
mod pacing;
mod panels;
mod tabs;
mod text;

use pacing::*;
use panels::*;
use tabs::*;
use text::*;
```

In each submodule add `use super::*;` at the top so the types in `mod.rs` (`App`, `RowView`, `Msg`, `Screen`) resolve, plus the ratatui and si-* imports the file needs (copy the import block from the old header and delete what `cargo build` reports unused). Mark every function and type that another module uses `pub(super)`: in `text.rs` all of them; in `tabs.rs` `draw_tabs`; in `panels.rs` `draw_turn`; in `pacing.rs` `draw_pacing`; in `mod.rs` `App`, `RowView`, `Msg`, `Screen` and `App`'s fields the panels read (`rec`, `defs`, `reviews`, `pool_at`, `row`, `turns`, `codes`, `path`, `is_capture`, `tx`, `status`, `running`, `deck`, `values`, `calibration`, `tries`, `expanded`, `cur`, `screen`). Move the two unit tests: `wrap_breaks_on_words_and_cuts_long_ones` to a `#[cfg(test)] mod tests` in `text.rs`, `centered_fits_inside` to one in `mod.rs`.

In `main.rs`: `mod study;` becomes `mod tui;` and the arm becomes `"study" => tui::run(&args[1..]),`.

Run: `cargo build -q --offline -p si-cli 2>&1 | head -40`
Expected: no errors (warnings about unused imports are fixed by deleting the import). Repeat until clean.

- [ ] **Step 3: Unit tests and the harness still pass**

Run: `cargo test -q --offline -p si-cli`
Expected: the two unit tests and `study_screens_on_the_shroud_capture` pass with no fixture change.

- [ ] **Step 4: Commit**

```bash
git add -A crates/si-cli/src
git commit -m "si study: the screens as tui/ modules (tabs, text, panels, pacing), no change drawn"
```

---

### Task 3: panels read a TurnCtx, never the App

**Files:**
- Modify: `crates/si-cli/src/tui/panels.rs` (`TurnCtx`, `draw_turn`, every builder)
- Modify: `crates/si-cli/src/tui/mod.rs` (`App::turn_ctx`)

- [ ] **Step 1: Replace the `TurnCtx` struct**

```rust
/// What a turn's tab is about: a closed turn carries its review; an open one (Task 15)
/// the turn in play and the plans scored from it.
pub(super) enum TurnKind<'r> {
    Closed(&'r TurnReview),
}

/// What every panel of the Turn screen reads. Built once per frame by `App::turn_ctx`;
/// no builder takes the `App`.
pub(super) struct TurnCtx<'r, 'd> {
    pub rec: &'r GameRecord,
    pub defs: &'d Definitions,
    pub codes: &'r RefCell<si_record::notation::Codes<'d>>,
    pub turn: u32,
    pub spirit: String,
    pub kind: TurnKind<'r>,
    pub t: Option<&'r si_record::TurnRecord>,
    pub s: Option<&'r si_record::SpiritTurn>,
    pub panel: Option<&'r si_model::spirit::SpiritPanel>,
    pub island: si_rules::Island,
    pub rows: Vec<RowView>,
    pub row: usize,
    pub sel: Option<RowView>,
    /// The Blight card's pool at this turn (left, flipped), when a capture kept the save.
    pub pool: Option<(i64, bool)>,
}

impl<'r, 'd> TurnCtx<'r, 'd> {
    pub fn code(&self, id: &str) -> String {
        self.codes.borrow_mut().code(id)
    }
    pub fn review(&self) -> Option<&'r TurnReview> {
        match self.kind {
            TurnKind::Closed(r) => Some(r),
        }
    }
    pub fn hand(&self) -> Vec<String> {
        self.review().map(|r| r.hand.clone()).unwrap_or_default()
    }
    pub fn energy(&self) -> i64 {
        self.review().map(|r| r.energy).unwrap_or(0)
    }
    pub fn plays(&self) -> i64 {
        self.review().map(|r| r.plays).unwrap_or(0)
    }
    pub fn forecast(&self) -> &'r [si_hindsight::deck::TurnForecast] {
        self.review().map(|r| r.forecast.as_slice()).unwrap_or(&[])
    }
    pub fn offered(&self) -> &'r [String] {
        self.review().map(|r| r.offered.as_slice()).unwrap_or(&[])
    }
    pub fn actual(&self) -> Option<&'r si_hindsight::Line> {
        self.review().map(|r| &r.actual)
    }
    pub fn notes(&self) -> &'r [String] {
        self.review().map(|r| r.notes.as_slice()).unwrap_or(&[])
    }
    pub fn choices(&self) -> &'r [si_hindsight::ChoiceReview] {
        self.review().map(|r| r.choices.as_slice()).unwrap_or(&[])
    }
    pub fn terror(&self) -> u8 {
        self.review().map(|r| r.sample.terror).unwrap_or(1)
    }
    pub fn best_growth(&self) -> &'r [String] {
        self.review().map(|r| r.best_growth.as_slice()).unwrap_or(&[])
    }
}
```

Two lifetimes, because `RefCell` is invariant: `App<'a>` holds `RefCell<Codes<'a>>`, and a `TurnCtx` borrowed for one frame could not name `Codes<'a>` with its own shorter lifetime. Keep `line_marks`, `visits`, `step_lands`, `sel_elements` on the impl (`impl<'r, 'd> TurnCtx<'r, 'd>`), with `visits` and `step_lands` reading `self.rec` and `self.defs` instead of `app`.

- [ ] **Step 2: Build the ctx in `mod.rs` and change every builder's signature**

In `mod.rs`:

```rust
impl<'a> App<'a> {
    /// The Turn screen's context for the current tab, or None while the turn is unreviewed.
    pub(super) fn turn_ctx(&self) -> Option<panels::TurnCtx<'_, 'a>> {
        let review = self.review()?;
        let rows = self.rows();
        let row = self.row.min(rows.len().saturating_sub(1));
        let sel = rows.get(row).cloned();
        let t = self.rec.turn(review.turn);
        let s = self.rec.spirit_turn(&review.spirit, review.turn);
        let panel = self.defs.spirits.get(&review.spirit);
        let island = t.map(|t| si_rules::verify::island_for(&self.rec, self.defs, &t.before_fast)).unwrap_or_default();
        Some(panels::TurnCtx { rec: &self.rec, defs: self.defs, codes: &self.codes, turn: review.turn, spirit: review.spirit.clone(), kind: panels::TurnKind::Closed(review), t, s, panel, island, rows, row, sel, pool: self.pool_at.get(&review.turn).copied() })
    }
}
```

`draw_turn(f, app, area)` becomes: build `app.turn_ctx()`; when None draw the "reviewing this turn..." / "no review" message exactly as before (`app.running` decides the wording, so pass `running: bool` as a third argument); else `draw_turn_ctx(f, &ctx, area)` which holds the old layout code with `app` removed. Every builder becomes `fn x_panel(ctx: &TurnCtx, width: u16) -> Panel`; `lines_panel` reads `ctx.rows` and `ctx.row` (its `select`); replace `app.code(..)` with `ctx.code(..)`, `app.rec`/`app.defs` with `ctx.rec`/`ctx.defs`, `review.hand` with `ctx.hand()`, `r.energy` with `ctx.energy()`, `r.plays` with `ctx.plays()`, `ctx.review.forecast` with `ctx.forecast()`, `r.offered` with `ctx.offered()`, `r.actual` with `ctx.actual()` (the growth panel's `rest_of` builds a `Line` from `ctx.actual().cloned().unwrap_or_default()`; give `Line` a `Default` derive if it lacks one, or build the tokens with `si_hindsight::growth_words` made `pub`), `ctx.review.notes` with `ctx.notes()`, `ctx.review.choices` with `ctx.choices()`, `ctx.review.sample.terror` with `ctx.terror()`, `r.best_growth` with `ctx.best_growth()`, and the `app.pool_at` lookups with `ctx.pool`. `reading_panel` and `lines_panel` keep reading `ctx.review()` directly for the review-only numbers (`reading`, `decision`, `luck`, `gap_*`, `candidates`, `positions`, `hidden_effect`, `expected_exogenous`), through `let Some(r) = ctx.review() else { return Panel::text(title, vec![]) };`.

Run: `cargo build -q --offline -p si-cli 2>&1 | head -40`
Expected: clean.

- [ ] **Step 3: The harness still passes**

Run: `cargo test -q --offline -p si-cli`
Expected: all pass, no fixture change.

- [ ] **Step 4: Commit**

```bash
git add crates/si-cli/src/tui
git commit -m "si study: panels built from a TurnCtx, the app no longer threaded through them"
```

---

### Task 4: the Seed carries what the open tab needs

**Files:**
- Modify: `crates/si-record/src/replay.rs:54-82` (`Seed`, `SeedSpirit`)
- (`Seed::apply_meta` lives in `replay.rs` beside the type; nothing in `record.rs` changes.) The spec lists `status_effects` on the Seed; it is left out, since the journal already records them on the turn and nothing on the open tab reads the save's copy.

- [ ] **Step 1: Extend the types**

Replace the `Seed` and `SeedSpirit` definitions with:

```rust
/// What a save says and a log that starts mid-game cannot: the board, each spirit's
/// Energy, piles and revealed track slots, the Invader cards already out of the deck,
/// and (for the turn being played) the face-up slots, the Blight card, the Fear counts.
/// Built from the engine snapshot of a save, by si-steam's `seed::from_snapshot`, which
/// reads only the fields a player can see.
#[derive(Debug, Clone, PartialEq)]
pub struct Seed {
    /// The engine's turn index at the save (0 = turn 1) and its phase.
    pub turn_index: u32,
    pub phase: Phase,
    pub boards: Vec<String>,
    /// Per land: pieces and tokens, and presence per spirit id.
    pub lands: Vec<(LandId, Counts, Vec<(String, u32)>)>,
    pub spirits: Vec<SeedSpirit>,
    pub terror: u8,
    /// Invader cards out of the deck before the join (the slots and the discard).
    pub invader_cards_seen: Vec<String>,
    /// The save this came from: the engine's game id and the file name.
    pub game_id: String,
    pub save_name: String,
    pub adversary: Option<String>,
    pub adversary_level: Option<u8>,
    pub difficulty: Option<i64>,
    pub scenario: Option<String>,
    /// The face-up cards: the Ravage and Build slots, and any extra slot (England's High
    /// Immigration) by the engine's name for it.
    pub ravage_slot: Vec<String>,
    pub build_slot: Vec<String>,
    pub extra_slots: Vec<(String, Vec<String>)>,
    pub blight_card: Option<String>,
    pub blight_pool: Option<i64>,
    pub blight_flipped: bool,
    pub fear_generated: i64,
    pub fear_required: i64,
    /// Fear cards earned and waiting face down (their names stay hidden).
    pub fear_cards_banked: u32,
    pub ending: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeedSpirit {
    pub id: String,
    pub energy: i64,
    pub hand: Vec<String>,
    pub play: Vec<String>,
    pub discard: Vec<String>,
    pub forgotten: Vec<String>,
    /// Track slots a presence has left.
    pub revealed: Vec<String>,
}

impl Seed {
    /// The board as a record keeps it: counts per land, presence summed over spirits.
    pub fn board(&self) -> BTreeMap<LandId, Counts> {
        self.lands.iter().map(|(id, c, _)| (id.clone(), *c)).collect()
    }

    /// One spirit's presence per land.
    pub fn presence_of(&self, spirit: &str) -> BTreeMap<LandId, u32> {
        self.lands.iter().filter_map(|(id, _, p)| p.iter().find(|(s, _)| s == spirit).filter(|(_, n)| *n > 0).map(|(_, n)| (id.clone(), *n))).collect()
    }

    /// What `apply_save` reads from a sibling EndGame save: the adversary when the log
    /// never named it, its level, the difficulty, the scenario.
    pub fn apply_meta(&self, meta: &mut crate::record::GameMeta) {
        if meta.adversary.is_none() {
            meta.adversary = self.adversary.clone();
        }
        if self.adversary_level.is_some() {
            meta.adversary_level = self.adversary_level;
        }
        meta.difficulty = self.difficulty;
        meta.scenario = self.scenario.clone();
    }
}
```

`Counts` must be `Copy` for `board()`: check `#[derive(... Copy ...)]` on it in `record.rs` (it is). Add `use std::collections::BTreeMap;` if missing.

- [ ] **Step 2: Build every crate that constructs a Seed**

Run: `cargo build -q --offline 2>&1 | grep -E "^error" -A6 | head -40`
Expected: one error, in `crates/si-cli/src/main.rs::seed_from_snapshot` (missing fields). Task 5 replaces that function; for now make it compile by filling the new fields with `..` defaults is not possible (no `Default`), so add the fields there with `game_id: snap.game_id.clone(), save_name: String::new(), adversary: None, adversary_level: None, difficulty: None, scenario: None, ravage_slot: vec![], build_slot: vec![], extra_slots: vec![], blight_card: None, blight_pool: None, blight_flipped: false, fear_generated: 0, fear_required: 0, fear_cards_banked: 0, ending: None`.

Run: `cargo test -q --offline -p si-record`
Expected: all pass.

- [ ] **Step 3: Commit**

```bash
git add crates/si-record/src/replay.rs crates/si-cli/src/main.rs
git commit -m "si-record: the Seed carries the slots, the Blight card, the Fear counts and the save's meta"
```

---

### Task 5: si-steam builds the Seed; the firewall test covers it

**Files:**
- Create: `crates/si-steam/src/seed.rs`
- Modify: `crates/si-steam/src/lib.rs` (`pub mod seed;`), `crates/si-steam/Cargo.toml` (deps on si-record, si-model)
- Modify: `crates/si-cli/src/main.rs:70-133` (delete `land_id_of` and `seed_from_snapshot`; `seed_from_args` calls `si_steam::seed::from_snapshot`)
- Modify: `crates/si-steam/tests/firewall.rs` (new test)

- [ ] **Step 1: Add the dependencies**

In `crates/si-steam/Cargo.toml` under `[dependencies]`:

```toml
si-model = { path = "../si-model" }
si-record = { path = "../si-record" }
```

- [ ] **Step 2: Write `seed.rs`**

```rust
//! A save as a `Seed`: the one reader of an engine snapshot on the live path. Only
//! fields a player at the table can see are read (the firewall test
//! `seed_is_invariant_to_hidden_permutations` holds this): the board, each spirit's own
//! piles and Energy, the face-up slots and discard, the Blight card, the Fear counts,
//! the Terror Level. Never the Invader deck, the face-down Explore slot, the Fear, Event,
//! Minor or Major decks, or the names of Fear cards earned face down.

use std::collections::BTreeMap;

use si_model::game::Phase;
use si_model::island::LandId;
use si_record::replay::{Seed, SeedSpirit};
use si_record::Counts;

use crate::engine_map::{EngineSnapshot, PieceKind as K};

/// `MountainA4` / `A4` as the record names a land.
pub fn land_id_of(engine: &str) -> Option<LandId> {
    let short = ["Mountain", "Sands", "Jungle", "Wetland", "Ocean"].iter().find_map(|t| engine.strip_prefix(t)).unwrap_or(engine);
    short.parse().ok()
}

pub fn from_snapshot(snap: &EngineSnapshot, save_name: &str) -> Seed {
    let spirit_of = |s: &str| -> String { snap.spirits.iter().find(|sp| sp.id == s).or_else(|| snap.spirits.iter().find(|sp| sp.id.contains(s) || s.contains(&sp.id))).map(|sp| sp.id.clone()).unwrap_or_else(|| s.to_string()) };
    let mut lands = Vec::new();
    for l in &snap.lands {
        let Some(id) = land_id_of(&l.id) else { continue };
        let mut c = Counts::default();
        let mut presence: BTreeMap<String, u32> = BTreeMap::new();
        for p in &l.pieces {
            match &p.kind {
                K::Explorer => c.explorers += 1,
                K::Town => c.towns += 1,
                K::City => c.cities += 1,
                K::Dahan => c.dahan += 1,
                K::Blight => c.blight += 1,
                K::Presence(s) => *presence.entry(spirit_of(s)).or_insert(0) += 1,
                K::Token(t) => match t.as_str() {
                    "Beast" | "Beasts" => c.beasts += 1,
                    "Disease" => c.disease += 1,
                    "Strife" => c.strife += 1,
                    "Wilds" => c.wilds += 1,
                    "Badlands" => c.badlands += 1,
                    _ => {}
                },
                _ => {}
            }
        }
        c.presence = presence.values().sum();
        lands.push((id, c, presence.into_iter().collect()));
    }
    let spirits = snap
        .spirits
        .iter()
        .map(|sp| SeedSpirit {
            id: sp.id.clone(),
            energy: sp.energy,
            hand: sp.hand.clone(),
            play: sp.play_area.clone(),
            discard: sp.discard.clone(),
            forgotten: sp.forgotten.clone(),
            revealed: sp.track_nodes.iter().filter(|(_, covered)| !covered).map(|(id, _)| id.clone()).collect(),
        })
        .collect();
    let ids = |v: &[crate::engine_map::EngineCard]| -> Vec<String> { v.iter().map(|c| c.id.clone()).collect() };
    let mut seen: Vec<String> = ids(&snap.invader_discard);
    seen.extend(ids(&snap.ravage_slot));
    seen.extend(ids(&snap.build_slot));
    seen.extend(ids(&snap.explore_slot_revealed));
    seen.extend(snap.extra_invader_slots.iter().flat_map(|(_, v)| ids(v)));
    seen.sort();
    seen.dedup();
    let boards: Vec<String> = lands.iter().map(|(id, _, _)| id.board.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    // Terror Level cards leave the game as each level is reached
    let terror = 1 + snap.out_of_game.iter().filter(|c| c.id.starts_with("TerrorLevel")).count() as u8;
    let (adversary, adversary_level) = match snap.adversaries.first() {
        Some((id, level)) => (Some(id.clone()), level.map(|l| l as u8)),
        None => (None, None),
    };
    Seed {
        turn_index: snap.turn_index.max(0) as u32,
        phase: Phase::parse(&snap.state),
        boards,
        lands,
        spirits,
        terror,
        invader_cards_seen: seen,
        game_id: snap.game_id.clone(),
        save_name: save_name.to_string(),
        adversary,
        adversary_level,
        difficulty: snap.difficulty,
        scenario: snap.scenario.clone(),
        ravage_slot: ids(&snap.ravage_slot),
        build_slot: ids(&snap.build_slot),
        extra_slots: snap.extra_invader_slots.iter().map(|(name, v)| (name.clone(), ids(v))).collect(),
        // the card's name is hidden until it flips (its healthy side is the same for all)
        blight_card: snap.blight_card.as_ref().filter(|b| b.flipped).map(|b| b.id.clone()),
        blight_pool: snap.blight_card.as_ref().map(|b| b.blight_remaining),
        blight_flipped: snap.blight_card.as_ref().is_some_and(|b| b.flipped),
        fear_generated: snap.fear_generated,
        fear_required: snap.fear_required,
        fear_cards_banked: snap.fear_earned.len() as u32,
        ending: snap.ending.clone(),
    }
}
```

Add `pub mod seed;` to `lib.rs`. In `main.rs` delete `land_id_of` and `seed_from_snapshot`; `seed_from_args` becomes:

```rust
fn seed_from_args(args: &[String]) -> Result<Option<si_record::replay::Seed>> {
    let Some(p) = args.iter().position(|a| a == "--seed").and_then(|i| args.get(i + 1)) else { return Ok(None) };
    let snap = decode_save(Path::new(p)).ok_or_else(|| anyhow::anyhow!("--seed: {p} is not a readable save"))?;
    let name = Path::new(p).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    Ok(Some(si_steam::seed::from_snapshot(&snap, &name)))
}
```

Run: `cargo build -q --offline 2>&1 | grep -E "^error" -A6 | head`
Expected: clean (fix any other `land_id_of` caller the compiler names by pointing it at `si_steam::seed::land_id_of`).

- [ ] **Step 3: The firewall test**

Append to `crates/si-steam/tests/firewall.rs`:

```rust
#[test]
fn seed_is_invariant_to_hidden_permutations() {
    let Some((defs, snaps)) = load() else { return };
    let mut rng = Lcg(0x5eed);
    for (name, s) in &snaps {
        let base = si_steam::seed::from_snapshot(s, name);
        assert!(!base.lands.is_empty(), "{name}: a seed with no lands");
        for round in 0..3 {
            let permuted = permute_hidden(s, &defs, &mut rng);
            let again = si_steam::seed::from_snapshot(&permuted, name);
            assert!(base == again, "{name}: permutation {round} changed the seed");
        }
    }
}
```

Run: `cargo test -q --offline -p si-steam --test firewall -- --nocapture 2>&1 | tail -8`
Expected: `seed_is_invariant_to_hidden_permutations ... ok`, or `skipping: capture or engine DLL not present` (the test's own skip; say which happened in the task report).

- [ ] **Step 4: Replay, hindsight and study still work with `--seed`**

Run: `cargo test -q --offline -p si-cli && cargo test -q --offline -p si-record`
Expected: all pass (the harness included).

- [ ] **Step 5: Commit**

```bash
git add crates/si-steam/Cargo.toml crates/si-steam/src/lib.rs crates/si-steam/src/seed.rs crates/si-steam/tests/firewall.rs crates/si-cli/src/main.rs Cargo.lock
git commit -m "si-steam: the Seed built from a save in one place, behind the firewall test"
```

---

### Task 6: engine groundwork: `Line.expected`/`per_card`, `Outcome.after`, `deep_for` ungated, `explore_candidates(deck)`

**Files:**
- Modify: `crates/si-hindsight/src/lib.rs`

- [ ] **Step 1: The new fields**

On `Line` (after `tiers`):

```rust
    /// What `informed` added for the face-down Event and Fear cards (`Calibration::expected_exogenous`).
    #[serde(default)]
    pub expected: f64,
    /// The informed score under each candidate Explore card, in `explore_candidates` order
    /// (empty for lines that were not ranked); the live `vs none` reads it.
    #[serde(default)]
    pub per_card: Vec<f64>,
```

On `Outcome` (after `slow`):

```rust
    /// Every land as the simulation left it (the live POSITION panel's `after` column).
    #[serde(default)]
    pub after: BTreeMap<LandId, Counts>,
```

Fix every `Line { .. }` constructor the compiler names with `expected: 0.0, per_card: vec![]`. In the informed loop (near `l.informed = mean;`) add `l.expected = expected_exo;` and `l.per_card = per_card[i].clone();`.

- [ ] **Step 2: `deep_for` without the next-turn gate on the informed branch**

```rust
fn deep_for(rec: &GameRecord, defs: &Definitions, deck: &deck::Deck, t: &TurnRecord, assumed: &[String], informed: bool) -> Option<Deep> {
    if informed {
        // the next turn's Explore is face down: average over its stage's remaining cards
        // (this turn's assumed card cannot come twice); the turn after's Ravage is this
        // turn's assumed Explore. No recorded next turn is needed: an open turn has none.
        let mut pool = deck::pool(rec, defs, deck, t, t.turn + 1).map(|(_, c)| c).unwrap_or_default();
        pool.retain(|c| !assumed.contains(c));
        return Some(Deep { ravage: assumed.to_vec(), high_immigration: vec![], pool });
    }
    rec.turns.iter().find(|n| n.turn == t.turn + 1)?;
    // the next turn ran as recorded; the turn after as recorded when the record has
    // it, else from this turn's Explore card
    let after = rec.turns.iter().find(|n| n.turn == t.turn + 2);
    let ravage = after.map(|a| a.ravage.clone()).filter(|r| !r.is_empty()).unwrap_or_else(|| assumed.to_vec());
    let high_immigration = after.map(|a| a.high_immigration.clone()).unwrap_or_default();
    Some(Deep { ravage, high_immigration, pool: vec![] })
}
```

- [ ] **Step 3: `explore_candidates` takes the deck and serves an open turn**

```rust
/// The Explore cards that could have come this turn, as far as was knowable: every
/// card of the stage the deck order put next that had not been drawn yet. On an open
/// turn (no Explore recorded) the stage comes from the deck order.
pub fn explore_candidates(rec: &GameRecord, defs: &Definitions, deck: &deck::Deck, t: &TurnRecord) -> Vec<Vec<String>> {
    let Some(first) = t.explore.first() else {
        return deck::pool(rec, defs, deck, t, t.turn).map(|(_, cards)| cards.into_iter().map(|c| vec![c]).collect()).unwrap_or_default();
    };
    // ... the rest unchanged
```

Fix the callers the compiler names (`review_turns` has `deck` in scope; `si calibrate` or the CLI may call it too: pass `&deck` there).

- [ ] **Step 4: Everything still passes, the harness included**

Run: `cargo test -q --offline -p si-hindsight -p si-cli`
Expected: all pass. The blessed panes do not change: the last turn's lookahead returns before it reads `Deep`, and every other turn had a next turn already.

- [ ] **Step 5: Commit**

```bash
git add crates/si-hindsight/src/lib.rs crates/si-cli/src
git commit -m "hindsight: lines carry their expectation and per-card scores, outcomes their board after; the deep term and the Explore pool on a turn with no next turn recorded"
```

---

### Task 7: the open-turn fixtures, `OpenTurn` and `open_turn`

**Files:**
- Create: `crates/si-hindsight/tests/fixtures/open-turn/growth/{log-stream.txt,events.txt,saves/232604_642__save-Feedback-002-0047-SpiritPhase.dat.gz}` and `.../fast/{log-stream.txt,events.txt,saves/232658_735__save-Feedback-002-0054-FastPower.dat.gz}`
- Create: `crates/si-hindsight/src/live.rs`, `crates/si-hindsight/tests/open_turn.rs`
- Modify: `crates/si-hindsight/src/lib.rs` (`pub mod live;`), `crates/si-hindsight/Cargo.toml` (dev-dependency si-steam)

- [ ] **Step 1: Cut the fixtures from the Shroud capture**

```bash
cd /home/brett/repos/spirit-island-steam-companion
CAP=/home/brett/repos/spirit-island/data/steam-capture/live-e392726411c9
FX=crates/si-hindsight/tests/fixtures/open-turn
cut_at() { # $1 cutoff HH:MM:SS.mmm, $2 out dir, $3 save file name
  mkdir -p "$2/saves"
  for f in log-stream.txt events.txt; do
    awk -v cut="$1" '
      function ms(t,  a) { split(t, a, /[:.]/); return ((a[1]*60+a[2])*60+a[3])*1000+a[4] }
      BEGIN { c = ms(cut); keep = 1 }
      { if (match($0, /^[0-9][0-9]:[0-9][0-9]:[0-9][0-9]\.[0-9][0-9][0-9]/)) keep = (ms(substr($0, 1, 12)) <= c); if (keep) print }
    ' "$CAP/$f" > "$2/$f"
  done
  cp "$CAP/saves/$3" "$2/saves/"
}
cut_at 23:26:04.642 $FX/growth 232604_642__save-Feedback-002-0047-SpiritPhase.dat.gz
cut_at 23:26:58.735 $FX/fast   232658_735__save-Feedback-002-0054-FastPower.dat.gz
wc -l $FX/*/log-stream.txt; ls -la $FX/*/saves
```

Expected: the growth journal ends at the `***** SAVING *****` line of 23:26:04.321 (about 402 lines), the fast journal at about 438 lines; two saves of about 85 KB.

- [ ] **Step 2: The dev-dependency and the test scaffold**

In `crates/si-hindsight/Cargo.toml`:

```toml
[dev-dependencies]
si-steam = { path = "../si-steam" }
```

`crates/si-hindsight/tests/open_turn.rs`:

```rust
//! The turn being played, cut from the Shroud capture at T3 Growth (before any growth
//! decision) and at T3 Fast, each with the save the app wrote at that moment.

use std::path::Path;
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::live::{open_turn, score_plan};
use si_hindsight::{explore_candidates, Calibration};
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::{GameRecord, Replay};

fn defs() -> Definitions {
    Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/engine"))).unwrap()
}

fn cut(defs: &Definitions, name: &str) -> (Arc<GameRecord>, Seed) {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/open-turn")).join(name);
    let (lines, undo, source) = si_record::replay::load(&root).unwrap();
    let mut rec = Replay::new(Some(defs)).run(&lines, undo.as_deref(), &source);
    let save = std::fs::read_dir(root.join("saves")).unwrap().next().unwrap().unwrap().path();
    let bytes = si_steam::source::read_save(&save).unwrap();
    let snap = si_steam::engine_map::map(&si_steam::nrbf::decode(&bytes).unwrap());
    let seed = si_steam::seed::from_snapshot(&snap, &save.file_name().unwrap().to_string_lossy());
    seed.apply_meta(&mut rec.meta);
    (Arc::new(rec), seed)
}

fn full(defs: &Definitions) -> GameRecord {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/shroud-e392726411c9"));
    let (lines, undo, source) = si_record::replay::load(root).unwrap();
    Replay::new(Some(defs)).run(&lines, undo.as_deref(), &source)
}

#[test]
fn the_growth_cut_opens_turn_three_before_any_growth() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    assert_eq!(rec.meta.turns, 3);
    assert_eq!(seed.turn_index, 2);
    assert_eq!(seed.phase, Phase::SpiritPhase);
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).expect("an open turn");
    assert_eq!((open.turn, open.phase), (3, Phase::SpiritPhase));
    assert!(open.t.explore.is_empty(), "no Explore yet");
    assert_eq!(open.t.ravage, seed.ravage_slot);
    assert_eq!(open.t.build, seed.build_slot);
    assert!(!open.t.ravage.is_empty() && !open.t.build.is_empty(), "slots from the save");
    assert!(!open.t.before_fast.is_empty(), "the board now, from the save");
    assert!(open.s.growth_groups.is_empty(), "nothing taken");
    assert!(!open.s.hand_after.is_empty(), "the hand from the save");
}

#[test]
fn the_fast_cut_opens_turn_three_with_the_growth_taken() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "fast");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).expect("an open turn");
    assert_eq!((open.turn, open.phase), (3, Phase::FastPower));
    assert!(!open.s.growth_groups.is_empty(), "the growth from the journal");
    assert!(!open.t.before_fast.is_empty());
    assert!(open.t.explore.is_empty());
}

#[test]
fn an_open_turn_averages_over_the_stages_unseen_cards() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "fast");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let cands = explore_candidates(&rec, &defs, &open.deck, &open.t);
    assert!(cands.len() > 1, "a pool, not one card: {cands:?}");
    for c in &cands {
        assert!(!seed.invader_cards_seen.contains(&c[0]), "{} is face up already", c[0]);
        assert!(!open.t.ravage.contains(&c[0]) && !open.t.build.contains(&c[0]));
    }
}

#[test]
fn a_finished_turn_is_not_open() {
    let defs = defs();
    let (rec, mut seed) = cut(&defs, "fast");
    seed.turn_index = 0; // a save from another turn
    assert!(open_turn(rec, &defs, &seed, &Calibration::default()).is_none());
}
```

Run: `cargo test -q --offline -p si-hindsight --test open_turn 2>&1 | head`
Expected: compile error, `live` module missing.

- [ ] **Step 3: `live.rs` with `OpenTurn` and `open_turn`**

```rust
//! The turn being played: the journal's trailing turn with the face-up cards and the
//! board the newest save says, and the scoring of a plan from it. Spec:
//! docs/superpowers/specs/2026-10-21-live-study.md, sections 2 and 4.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use si_defs::Definitions;
use si_model::game::Phase;
use si_model::island::LandId;
use si_model::power::{PowerCard, PowerKind, Speed};
use si_record::replay::Seed;
use si_record::{Counts, GameRecord, SpiritTurn, TurnRecord};
use si_rules::invaders::Rules;
use si_rules::verify;

use crate::deck::{self, Deck};
use crate::{Calibration, Line, Outcome, Play, Position, Values};

/// The open turn: everything the scorer reads, so `score_plan` takes it alone.
#[derive(Debug, Clone)]
pub struct OpenTurn {
    pub rec: Arc<GameRecord>,
    pub turn: u32,
    pub phase: Phase,
    pub spirit: String,
    /// The trailing turn with the slots filled and, at Growth, the board now as `before_fast`.
    pub t: TurnRecord,
    pub s: SpiritTurn,
    pub seed: Seed,
    pub deck: Deck,
    pub rules: Rules,
    pub cal: Calibration,
    pub values: Values,
}

/// The phases on which the turn is open: a tab exists and plans are scored.
pub fn is_open_phase(p: Phase) -> bool {
    matches!(p, Phase::SpiritPhase | Phase::FastPower | Phase::BlightedIsland | Phase::Event | Phase::Fear | Phase::HighImmigration | Phase::Ravage | Phase::Build | Phase::Explore | Phase::AdvanceInvaderCards | Phase::SlowPower)
}

/// The phase in the game's words for the tab label.
pub fn phase_word(p: Phase) -> &'static str {
    match p {
        Phase::SpiritPhase => "Growth",
        Phase::FastPower => "Fast",
        Phase::BlightedIsland => "Blight card",
        Phase::Event => "Event",
        Phase::Fear => "Fear",
        Phase::HighImmigration => "High Immigration",
        Phase::Ravage => "Ravage",
        Phase::Build => "Build",
        Phase::Explore => "Explore",
        Phase::AdvanceInvaderCards => "Cards advance",
        Phase::SlowPower => "Slow",
        Phase::TimePasses => "Time passes",
        other => other.name(),
    }
}

/// The journal's trailing turn as the open turn, or None when the game has ended, the
/// save is not at that turn, or the phase has no open tab.
pub fn open_turn(rec: Arc<GameRecord>, defs: &Definitions, seed: &Seed, cal: &Calibration) -> Option<OpenTurn> {
    if seed.ending.is_some() || rec.meta.ending.is_some() {
        return None;
    }
    let spirit = rec.meta.spirits.first().cloned()?;
    let t0 = rec.turns.iter().filter(|t| t.turn >= 1).last()?.clone();
    let turn = t0.turn;
    if seed.turn_index + 1 != turn || !is_open_phase(seed.phase) {
        return None;
    }
    let phase = seed.phase;
    let mut t = t0;
    // the face-up cards, which the journal records only as each step runs
    if t.explore.is_empty() {
        if t.ravage.is_empty() {
            t.ravage = seed.ravage_slot.clone();
        }
        if t.build.is_empty() {
            t.build = seed.build_slot.clone();
        }
        if t.high_immigration.is_empty() {
            t.high_immigration = seed.extra_slots.iter().flat_map(|(_, v)| v.iter().cloned()).collect();
        }
    }
    // the board: the journal snapshots it at the Fast transition; before that, the save's
    if t.before_fast.is_empty() {
        t.before_fast = seed.board();
    }
    let panel = defs.spirits.get(&spirit);
    let mut s = rec.spirits.iter().find(|x| x.turn == turn && x.spirit == spirit).cloned().unwrap_or_else(|| SpiritTurn { turn, spirit: spirit.clone(), ..Default::default() });
    if let Some(ss) = seed.spirits.iter().find(|x| x.id == spirit) {
        if phase == Phase::SpiritPhase {
            // nothing taken yet: the position is the save's
            s.hand_after = ss.hand.clone();
            s.discard_after = ss.discard.clone();
            s.energy_after = ss.energy;
            s.revealed_after = ss.revealed.iter().cloned().collect();
            s.presence_after = seed.presence_of(&spirit);
            if let Some(ts) = panel.map(|p| p.track_state(&s.revealed_after)) {
                s.income = Some(ts.income);
                s.plays = Some(ts.plays);
            }
        } else {
            // the journal and the save disagree: the save is the engine's own state
            let journal: BTreeSet<&String> = s.hand_after.iter().chain(s.cards_played.iter()).collect();
            let save: BTreeSet<&String> = ss.hand.iter().chain(ss.play.iter()).collect();
            if journal != save {
                s.hand_after = ss.hand.iter().chain(ss.play.iter()).filter(|c| !s.cards_played.contains(c)).cloned().collect();
            }
            if s.energy_after != ss.energy {
                s.energy_after = ss.energy;
            }
            let revealed: BTreeSet<String> = ss.revealed.iter().cloned().collect();
            if s.revealed_after != revealed {
                s.revealed_after = revealed;
            }
            if s.presence_after.is_empty() {
                s.presence_after = seed.presence_of(&spirit);
            }
        }
    }
    let level = seed.adversary_level.or(rec.meta.adversary_level).or_else(|| si_metrics::adversary::estimate(&rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    let rules = Rules { adversary: rec.meta.adversary.clone().or_else(|| seed.adversary.clone()), level, spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), ..Default::default() };
    let deck = Deck::for_game(&rec, defs, level);
    let mut values = Values::for_panel(panel);
    values.next_weight = cal.next_weight;
    Some(OpenTurn { rec, turn, phase, spirit, t, s, seed: seed.clone(), deck, rules, cal: cal.clone(), values })
}
```

Add `pub mod live;` to `lib.rs`. `Phase::name` exists (`game.rs:58`); `Values::for_panel` and `Rules::hooks_of` exist. If `si_metrics::adversary::estimate` is not already a dependency path in si-hindsight, it is (`review_turns` calls it).

Run: `cargo test -q --offline -p si-hindsight --test open_turn 2>&1 | tail -12`
Expected: the three `open_turn` tests pass; `score_plan` is still unresolved, so comment its import out until Task 8 (or leave the import and add `pub fn score_plan` as `todo!()` is not acceptable: comment the import).

- [ ] **Step 4: Commit**

```bash
git add crates/si-hindsight/Cargo.toml crates/si-hindsight/src/lib.rs crates/si-hindsight/src/live.rs crates/si-hindsight/tests/ Cargo.lock
git commit -m "hindsight: the open turn from the journal's trailing turn and a save's seed; Shroud cuts at T3 Growth and T3 Fast as fixtures"
```

---

### Task 8: `score_plan`, with `parse_plays` and `recorded_plays` shared with the review

**Files:**
- Modify: `crates/si-hindsight/src/lib.rs` (extract `parse_plays` from `try_line`, `recorded_plays` from `review_turns`; make `play_options`, `innates_of`, `elements_for`, `energy_available`, `apply_plays`, `invader_phase`, `lookahead`, `resolve_slow`, `deep_for`, `with_slow`, `annotate_elements`, `actual_position`, `growth_positions`, `normal_cdf` reachable from `live.rs` with `pub(crate)`)
- Modify: `crates/si-hindsight/src/live.rs` (`score_plan`)
- Modify: `crates/si-hindsight/tests/open_turn.rs`

- [ ] **Step 1: Extract the two helpers in `lib.rs`, behavior unchanged**

From `try_line` (lines 2031-2058 at `2fbf8e0`), the resolver and the token loop become:

```rust
/// Plays in notation (`SLSF>C8 MI GP2@A2`): a card or innate by id, title or short code,
/// `>land` or `@land` for its target.
pub fn parse_plays(defs: &Definitions, text: &str) -> Result<Vec<Play>, String> {
    let mut codes = si_record::notation::Codes::new(Some(defs), false);
    let mut by_code: BTreeMap<String, String> = BTreeMap::new();
    for id in defs.cards.keys().chain(defs.spirits.values().flat_map(|p| p.innates.iter().map(|i| &i.id))) {
        by_code.insert(codes.code(id).to_uppercase(), id.clone());
    }
    let resolve = |name: &str| -> Option<String> {
        if defs.cards.contains_key(name) || defs.spirits.values().flat_map(|p| p.innates.iter()).any(|i| i.id == name) {
            return Some(name.to_string());
        }
        if let Some(c) = defs.card_by_title(name) {
            return Some(c.id.clone());
        }
        by_code.get(&name.to_uppercase()).cloned()
    };
    let mut plays: Vec<Play> = Vec::new();
    for tok in text.split_whitespace() {
        let (name, target) = match tok.split_once('>').or_else(|| tok.split_once('@')) {
            Some((n, l)) => (n, Some(l.parse::<LandId>().map_err(|e| e.to_string())?)),
            None => (tok, None),
        };
        let id = resolve(name).ok_or(format!("unknown Power {name}"))?;
        let innate = !defs.cards.contains_key(&id);
        let (fast, cost) = match defs.cards.get(&id) {
            Some(c) => (c.speed == Speed::Fast, c.cost),
            None => (defs.spirits.values().flat_map(|p| p.innates.iter()).find(|i| i.id == id).is_some_and(|i| i.speed == Speed::Fast), 0),
        };
        let places = defs.cards.get(&id).is_some_and(adds_presence);
        plays.push(Play { power: id, target, fast, innate, cost, places });
    }
    Ok(plays)
}
```

and `try_line` calls `let plays = parse_plays(defs, text)?;`. From `review_turns` (lines 1200-1234), the `actual_plays`/`actual_innates` construction becomes:

```rust
/// The plays the journal records for the spirit this turn, cards and innates apart, each
/// with its target as recorded (or recovered from where it acted).
pub(crate) fn recorded_plays(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, t: &TurnRecord, all_innates: &[&si_model::spirit::Innate]) -> (Vec<Play>, Vec<Play>) {
    let cards: Vec<Play> = s.cards_played.iter().map(|c| { /* the body of the old map, unchanged */ }).collect();
    let innates: Vec<Play> = s.powers_used.iter().filter(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && all_innates.iter().any(|i| i.id == *pw)).map(|(p, pw, t)| Play { power: pw.clone(), target: t.clone(), fast: *p == Phase::FastPower, innate: true, cost: 0, places: false }).collect();
    (cards, innates)
}
```

with `review_turns` doing `let (actual_plays, actual_innates) = recorded_plays(rec, defs, s, t, &all_innates);`. Mark the functions listed in **Files** `pub(crate)`.

Run: `cargo test -q --offline -p si-hindsight -p si-cli`
Expected: all pass, harness unchanged.

- [ ] **Step 2: Write the failing tests**

Append to `tests/open_turn.rs` (restore the `score_plan` import):

```rust
#[test]
fn a_growth_token_the_panel_does_not_offer_is_refused_with_the_groups() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec, &defs, &seed, &Calibration::default()).unwrap();
    let err = score_plan(&defs, &open, None, "G9 | ").unwrap_err();
    assert!(err.contains("G1") && err.contains("no growth position matches"), "{err}");
}

#[test]
fn doing_nothing_scores_on_every_open_tab() {
    let defs = defs();
    for name in ["growth", "fast"] {
        let (rec, seed) = cut(&defs, name);
        let open = open_turn(rec, &defs, &seed, &Calibration::default()).unwrap();
        let none = score_plan(&defs, &open, None, "").unwrap();
        assert!(none.plays.is_empty(), "{name}: nothing played");
        assert!(!none.per_card.is_empty(), "{name}: scored under each Explore card");
        assert!(none.confidence.is_none());
        assert!(!none.outcome.after.is_empty(), "{name}: the board after");
    }
}

#[test]
fn a_growth_plan_is_scored_from_its_position_and_against_none() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let positions = si_hindsight::growth_positions(&rec, &defs, &open.s, &open.t, &si_metrics::reach::island_for(&rec, &defs));
    let first = positions.first().expect("the panel offers growth");
    let text = format!("{} |", first.growth.join(" "));
    let none = score_plan(&defs, &open, None, "").unwrap();
    let plan = score_plan(&defs, &open, Some(&none), &text).unwrap();
    assert_eq!(plan.growth, first.growth);
    assert!(plan.growth_value > 0.0, "slots, cards or Energy kept count");
    assert!(plan.confidence.is_some(), "vs none");
}

#[test]
fn the_fast_plan_of_the_actual_plays_agrees_with_the_review() {
    let defs = defs();
    let cal = Calibration::default();
    let mut full = full(&defs);
    let (rec, seed) = cut(&defs, "fast");
    // the full record has no sibling save in the fixture: take the level from the same save
    seed.apply_meta(&mut full.meta);
    let open = open_turn(rec.clone(), &defs, &seed, &cal).unwrap();
    // the same adversary level and deck offset on both sides
    let level_full = full.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(&full, &defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    assert_eq!(open.rules.level, level_full, "adversary level");
    assert_eq!(open.deck.offset, si_hindsight::deck::Deck::for_game(&full, &defs, level_full).offset, "deck offset");
    let opts = si_hindsight::ReviewOptions { actual_only: true, calibration: Some(cal.clone()), ..Default::default() };
    let review = si_hindsight::review_turns(&full, &defs, &opts, Some(&[3]), &|_| {});
    let actual = &review.turns[0].actual;
    // the actual plays, Slow ones without a land so both paths choose them after the steps
    let text: String = actual.plays.iter().map(|p| match (&p.target, p.fast || p.places) { (Some(l), true) if p.places => format!("{}@{l}", p.power), (Some(l), true) => format!("{}>{l}", p.power), _ => p.power.clone() }).collect::<Vec<_>>().join(" ");
    let plan = score_plan(&defs, &open, None, &text).unwrap();
    assert!((plan.expected - actual.expected).abs() < 0.01, "the expectation terms: plan {} (Event and Fear cards as the open turn counts them) vs review {}", plan.expected, actual.expected);
    let (a, b) = (plan.informed - plan.expected, actual.informed - actual.expected);
    assert!((a - b).abs() < 0.01, "informed less the expectation: plan {a:.3} vs review {b:.3}\nplan {:?}\nreview {:?}", plan.per_card, actual.per_card);
}
```

Run: `cargo test -q --offline -p si-hindsight --test open_turn 2>&1 | head`
Expected: compile error, `score_plan` missing.

- [ ] **Step 3: `score_plan` in `live.rs`**

```rust
/// A plan's growth tokens before `|`, matched to one of the positions the panel allows.
fn match_growth(defs: &Definitions, open: &OpenTurn, graph: &si_model::island::Island, text: &str) -> Result<Position, String> {
    let all = crate::growth_positions(&open.rec, defs, &open.s, &open.t, graph);
    let want: BTreeSet<String> = text
        .split_whitespace()
        .map(|tok| match tok.strip_prefix('+').filter(|r| !r.ends_with('E')) {
            // a gained card by code, title or id
            Some(name) => format!("+{}", crate::parse_plays(defs, name).ok().and_then(|v| v.first().map(|p| p.power.clone())).unwrap_or_else(|| name.to_string())),
            None => tok.to_string(),
        })
        .collect();
    if let Some(p) = all.iter().find(|p| p.growth.iter().cloned().collect::<BTreeSet<_>>() == want) {
        return Ok(p.clone());
    }
    let groups: Vec<String> = defs.spirits.get(&open.spirit).map(|p| p.growth.groups.iter().map(|g| format!("G{}", g.number)).collect()).unwrap_or_default();
    let examples: Vec<String> = all.iter().take(4).map(|p| p.growth.join(" ")).collect();
    Err(format!("no growth position matches \"{text}\"; the panel offers {} ({} positions, such as {})", groups.join(" "), all.len(), examples.join(" / ")))
}

/// What the informed score adds for the face-down cards on an open turn: the Event from
/// turn 2 on, the Fear cards earned so far plus those the plan's own Fear reaches, at the
/// Terror Level the turn began at; zero on the Slow tab, where the board carries them.
fn expectation(open: &OpenTurn, plan_fear: u32) -> (bool, u32, u8) {
    let rec = &open.rec;
    let t = &open.t;
    if open.phase == Phase::SlowPower {
        return (false, 0, open.seed.terror);
    }
    // the Event's term stays until the scoring board carries its effect, which is the Slow tab
    let event = t.turn >= 2;
    let req = open.seed.fear_required.max(1);
    let crossed = ((open.seed.fear_generated + plan_fear as i64) / req - open.seed.fear_generated / req).max(0) as u32;
    let fear_cards = t.fear_cards_earned + crossed;
    let tl_start = rec.turns.iter().filter(|u| u.turn < t.turn).map(|u| u.terror_after).max().unwrap_or(1).clamp(1, 3);
    (event, fear_cards, tl_start)
}

/// Score a plan from the open turn: `G2 e4@C4 | CS>C3 FCD>C4` on the Growth tab, plays
/// only after it. Informed: the mean over the Explore cards still in the stage, the
/// position's growth value, the expectation for the face-down cards. `confidence` is the
/// share of candidates under which the plan beats `baseline` (the `none` plan) with the
/// simulator's error. A play the journal already records is not applied twice.
pub fn score_plan(defs: &Definitions, open: &OpenTurn, baseline: Option<&Line>, text: &str) -> Result<Line, String> {
    let rec: &GameRecord = &open.rec;
    let t = &open.t;
    let spirit = open.spirit.as_str();
    let values = &open.values;
    let rules = &open.rules;
    let graph = si_metrics::reach::island_for(rec, defs);
    let all_innates = crate::innates_of(defs, spirit);
    let (growth_text, plays_text) = match text.split_once('|') {
        Some((g, p)) => (g.trim(), p.trim()),
        None => ("", text.trim()),
    };
    let at_growth = open.phase == Phase::SpiritPhase;
    let at_slow = open.phase == Phase::SlowPower;
    let past_fast = !matches!(open.phase, Phase::SpiritPhase | Phase::FastPower);
    let pos: Position = if at_growth && !growth_text.is_empty() {
        match_growth(defs, open, &graph, growth_text)?
    } else {
        if !growth_text.is_empty() {
            return Err("the growth is taken: write the plays only".into());
        }
        crate::actual_position(rec, defs, &open.s, t)
    };
    let mut notes: Vec<String> = Vec::new();
    let typed = crate::parse_plays(defs, plays_text)?;
    if past_fast {
        if let Some(p) = typed.iter().find(|p| p.fast) {
            return Err(format!("the Fast phase is over: {} is Fast", p.power));
        }
    }
    let (made_cards, made_innates) = crate::recorded_plays(rec, defs, &open.s, t, &all_innates);
    let typed: Vec<Play> = typed
        .into_iter()
        .filter(|p| {
            let made = made_cards.iter().chain(made_innates.iter()).any(|m| m.power == p.power && m.target == p.target);
            if made {
                notes.push(format!("{} made already", p.power));
            }
            !made
        })
        .collect();
    let mut cards: Vec<Play> = made_cards.clone();
    cards.extend(typed.iter().filter(|p| !p.innate).cloned());
    let mut innates: Vec<Play> = made_innates.clone();
    innates.extend(typed.iter().filter(|p| p.innate).cloned());
    let cost: i64 = cards.iter().map(|p| p.cost).sum();
    if cost > pos.energy {
        notes.push(format!("costs {cost} Energy, {} available", pos.energy));
    }
    if cards.len() as i64 > pos.plays {
        notes.push(format!("{} cards against {} plays", cards.len(), pos.plays));
    }
    for c in typed.iter().filter(|p| !p.innate) {
        if !pos.hand.contains(&c.power) {
            notes.push(format!("{} is not in hand", c.power));
        }
    }
    let growth_value = pos.slots as f64 * values.slot + pos.cards_gained as f64 * values.card + (pos.energy - cost).max(0) as f64 * values.energy;
    let mut s2 = open.s.clone();
    s2.presence_after = pos.presence.clone();
    s2.revealed_after = pos.revealed.clone();
    let played: Vec<&PowerCard> = cards.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = crate::elements_for(defs, &s2, &played);
    let all: Vec<Play> = cards.iter().cloned().chain(innates.iter().cloned()).collect();
    // the next turn as the face-up cards project it: its Ravage is this turn's Build slot,
    // its Build this turn's Explore once known (`informed_build` substitutes the candidate)
    let next = TurnRecord { turn: t.turn + 1, ravage: t.build.clone(), build: t.explore.clone(), ..Default::default() };
    let cands: Vec<Vec<String>> = if at_slow { vec![t.explore.clone()] } else { crate::explore_candidates(rec, defs, &open.deck, t) };
    let cands = if cands.is_empty() { vec![vec![]] } else { cands };
    // one run of the plan under one Explore card; `display` skips the Explore and the deep
    // term, for the certain part shown in the panels
    let simulate = |explore: &[String], display: bool| -> (Outcome, si_rules::Island) {
        let mut out = Outcome::default();
        let mut armed = Vec::new();
        let mut island;
        if at_slow {
            island = verify::island_for(rec, defs, &t.before_slow);
        } else {
            island = verify::island_for(rec, defs, &t.before_fast);
            for (id, l) in island.lands.iter_mut() {
                l.c.presence = s2.presence_after.get(id).copied().unwrap_or(0);
            }
            crate::apply_plays(&mut island, defs, &all, true, &elements, rules, spirit, t, &mut armed, &mut out);
            crate::invader_phase(&mut island, rec, t, defs, rules, &mut out, Some(explore), false);
        }
        let informed_build: Option<&[String]> = Some(explore);
        let slow = {
            let eval = |slow: &[Play]| -> f64 {
                let mut isl = island.clone();
                let mut o = out.clone();
                let mut armed2 = armed.clone();
                crate::apply_plays(&mut isl, defs, slow, false, &elements, rules, spirit, t, &mut armed2, &mut o);
                crate::lookahead(&mut isl, rec, Some(&next), defs, rules, &mut o, informed_build, None);
                o.score(values)
            };
            crate::resolve_slow(&island, defs, &all, false, &s2, &graph, &all_innates, &eval)
        };
        let mut all2: Vec<Play> = all.iter().filter(|p| p.fast).cloned().collect();
        all2.extend(slow.iter().cloned());
        crate::apply_plays(&mut island, defs, &all2, false, &elements, rules, spirit, t, &mut armed, &mut out);
        let deep = if display { None } else { crate::deep_for(rec, defs, &open.deck, t, explore, true) };
        crate::lookahead(&mut island, rec, Some(&next), defs, rules, &mut out, informed_build, deep.as_ref());
        out.slow = slow;
        (out, island)
    };
    let per_card: Vec<f64> = cands.iter().map(|c| simulate(c, false).0.score(values) + growth_value).collect();
    let mean = per_card.iter().sum::<f64>() / per_card.len() as f64;
    let (mut out, island) = simulate(&[], true);
    out.after = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
    let (event, fear_cards, terror) = expectation(open, out.fear);
    let expected = open.cal.expected_exogenous(event, fear_cards, terror);
    let informed = mean + expected;
    let confidence = baseline.filter(|b| b.per_card.len() == per_card.len() && !per_card.is_empty()).map(|b| {
        let noise = open.cal.model_sd * std::f64::consts::SQRT_2;
        per_card.iter().zip(&b.per_card).map(|(x, y)| if noise > 0.05 { crate::normal_cdf((x - y) / noise) } else if x >= y { 1.0 } else { 0.0 }).sum::<f64>() / per_card.len() as f64
    });
    out.notes.extend(notes);
    let plays = crate::with_slow(&all, &out);
    let mut line = Line { growth: pos.growth.clone(), growth_actual: !(at_growth && !growth_text.is_empty()), growth_value, ties: 0, informed, luck: 0.0, confidence, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays, outcome: out, score: informed, actual: false, expected, per_card };
    crate::annotate_elements(&mut line, defs, &s2);
    Ok(line)
}
```

`Line.score` is the informed score on an open turn (there is no hindsight yet). Make `growth_positions`, `actual_position`, `normal_cdf`, `parse_plays` `pub` (two already are) and the rest `pub(crate)`.

Run: `cargo test -q --offline -p si-hindsight --test open_turn 2>&1 | tail -20`
Expected: all seven tests pass. If `the_fast_plan_of_the_actual_plays_agrees_with_the_review` fails on the expectation term, the message says both counts: read the Shroud T3 record (`si report` on the full fixture) to see whether the open turn's `fear_cards_earned` plus the plan's crossings differs from the recorded card count, fix `expectation` if the open turn is wrong, and otherwise report the difference with its cause instead of loosening the test. If it fails on `informed - expected`, print both `per_card` vectors (the assertion does) and compare card by card: a difference on every card is the lookahead (the projected T4 against the recorded T4: compare `next.ravage`/`build` with the full record's T4), on some cards the Slow targets (`rechoose`).

- [ ] **Step 4: Commit**

```bash
git add crates/si-hindsight
git commit -m "hindsight: score_plan, the informed score of a typed plan from the open turn; parse_plays and recorded_plays shared with the review"
```

---

### Task 9: the review takes closed turns only

**Files:**
- Modify: `crates/si-hindsight/src/lib.rs` (`review_turns` filter; a `closed_turns` helper)
- Modify: `crates/si-cli/src/tui/mod.rs` (`turns` list)
- Modify: `crates/si-hindsight/tests/open_turn.rs`

- [ ] **Step 1: The failing test**

```rust
#[test]
fn the_review_skips_the_open_turn() {
    let defs = defs();
    let (rec, _) = cut(&defs, "fast");
    assert_eq!(si_hindsight::closed_turns(&rec), vec![1, 2]);
    let opts = si_hindsight::ReviewOptions { growth: false, ..Default::default() };
    let review = si_hindsight::review_turns(&rec, &defs, &opts, Some(&[3]), &|_| {});
    assert!(review.turns.is_empty(), "turn 3 has no Invader steps yet");
}
```

Run: `cargo test -q --offline -p si-hindsight --test open_turn the_review_skips 2>&1 | tail -5`
Expected: compile error, `closed_turns` missing.

- [ ] **Step 2: The helper and the filter**

```rust
/// The turns a review can judge: every turn before the journal's last, and the last when
/// the game ended in it. The turn being played is never one of them.
pub fn closed_turns(rec: &GameRecord) -> Vec<u32> {
    rec.turns.iter().filter(|t| t.turn >= 1 && !t.before_fast.is_empty() && (t.turn < rec.meta.turns || rec.meta.ending.is_some())).map(|t| t.turn).collect()
}
```

In `review_turns`, the loop filter becomes `.filter(|t| closed.contains(&t.turn) && only.is_none_or(|o| o.contains(&t.turn)))` with `let closed = closed_turns(rec);`. In `tui/mod.rs::run`, `let turns: Vec<u32> = si_hindsight::closed_turns(&rec);`.

Run: `cargo test -q --offline -p si-hindsight -p si-cli`
Expected: all pass; the harness panes are unchanged (the Shroud game ended, so every turn is closed).

- [ ] **Step 3: Commit**

```bash
git add crates/si-hindsight/src/lib.rs crates/si-hindsight/tests/open_turn.rs crates/si-cli/src/tui/mod.rs
git commit -m "hindsight: a turn is reviewed once the journal has moved past it or the game ended in it"
```

---

### Task 10: the draw count for THINK ABOUT

**Files:**
- Modify: `crates/si-hindsight/src/live.rs` (`unseen_cards`, `draw_count`, `DrawCount`, `ravage_thresholds`, `RavageThresholds`)
- Modify: `crates/si-hindsight/src/lib.rs` (`play_options` `pub(crate)`)
- Modify: `crates/si-hindsight/tests/open_turn.rs`

- [ ] **Step 1: The failing test**

```rust
#[test]
fn unseen_minors_exclude_what_the_journal_has_shown() {
    let defs = defs();
    let (rec, seed) = cut(&defs, "growth");
    let open = open_turn(rec.clone(), &defs, &seed, &Calibration::default()).unwrap();
    let unseen = si_hindsight::live::unseen_cards(&rec, &defs, PowerKind::Minor);
    let total = defs.cards.values().filter(|c| c.kind == PowerKind::Minor).count();
    assert!(!unseen.is_empty() && unseen.len() < total, "{} of {total}", unseen.len());
    for s in &rec.spirits {
        for c in s.hand_after.iter().chain(s.cards_gained.iter()).chain(s.offered.iter()) {
            assert!(!unseen.iter().any(|u| u.id == *c), "{c} has been seen");
        }
    }
    let land: si_model::island::LandId = open.t.before_fast.keys().next().unwrap().clone();
    let count = si_hindsight::live::draw_count(&defs, &open, PowerKind::Minor, &land);
    assert!(count.playable <= count.unseen);
}
```

(`use si_model::power::PowerKind;` at the top.)

- [ ] **Step 2: The functions**

```rust
/// Power Cards of a kind the journal has never shown: not in any spirit's hand, play,
/// discard, forgotten pile, draft or starting uniques.
pub fn unseen_cards<'d>(rec: &GameRecord, defs: &'d Definitions, kind: PowerKind) -> Vec<&'d PowerCard> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for s in &rec.spirits {
        for c in s.hand_after.iter().chain(s.cards_played.iter()).chain(s.discard_after.iter()).chain(s.cards_gained.iter()).chain(s.cards_forgotten.iter()).chain(s.offered.iter()).chain(s.reclaimed_cards.iter()) {
            seen.insert(c.as_str());
        }
    }
    for id in &rec.meta.spirits {
        if let Some(p) = defs.spirits.get(id) {
            seen.extend(p.uniques.iter().map(|c| c.as_str()));
        }
    }
    let mut v: Vec<&PowerCard> = defs.cards.values().filter(|c| c.kind == kind && !seen.contains(c.id.as_str())).collect();
    v.sort_by(|a, b| a.id.cmp(&b.id));
    v
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawCount {
    pub unseen: usize,
    /// Cards that play on the land this turn: Fast, within the Energy, in range from the
    /// position (the same test the review's candidates pass).
    pub playable: usize,
    /// Unseen cards the simulator cannot read in full, counted as not playing.
    pub unreadable: usize,
}

/// Of the unseen cards of a kind, how many could be played on `land` this turn.
pub fn draw_count(defs: &Definitions, open: &OpenTurn, kind: PowerKind, land: &LandId) -> DrawCount {
    let graph = si_metrics::reach::island_for(&open.rec, defs);
    let pos = crate::actual_position(&open.rec, defs, &open.s, &open.t);
    let unseen = unseen_cards(&open.rec, defs, kind);
    let mut playable = 0;
    let mut unreadable = 0;
    for c in &unseen {
        if !c.fully_parsed() {
            unreadable += 1;
            continue;
        }
        if c.speed != Speed::Fast || c.cost > pos.energy {
            continue;
        }
        if crate::play_options(c, &pos.presence, &open.t.before_fast, &graph).iter().any(|p| p.target.as_ref() == Some(land)) {
            playable += 1;
        }
    }
    DrawCount { unseen: unseen.len(), playable, unreadable }
}
```

Also in `live.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct RavageThresholds {
    /// Damage the Ravage deals in the land with every modifier (0 when no card names it).
    pub damage: u32,
    /// The least Defend that keeps Blight out of the land, when any up to the damage does.
    pub defend_keeps_land: Option<u32>,
    /// The least Defend that loses no Dahan, when any up to the damage does.
    pub defend_keeps_dahan: Option<u32>,
}

/// What this turn's Ravage does to one land under each Defend, by running the simulator
/// on a copy of the island: the damage, and the two Defend numbers THINK ABOUT names.
pub fn ravage_thresholds(island: &si_rules::Island, defs: &Definitions, cards: &[String], rules: &Rules, land: &LandId) -> RavageThresholds {
    let run = |defend: u32| -> Option<(u32, u32, u32)> {
        let mut isl = island.clone();
        isl.lands.get_mut(land)?.defend = defend;
        let mut damage = 0;
        let mut blight = 0;
        let mut lost = 0;
        for card in cards.iter().filter_map(|c| defs.invader_cards.get(c)) {
            for o in si_rules::invaders::ravage(&mut isl, card, rules).into_iter().filter(|o| o.land == *land) {
                damage += o.damage;
                blight += o.blight + o.cascade;
                lost += o.dahan_lost;
            }
        }
        Some((damage, blight, lost))
    };
    let Some((damage, blight0, lost0)) = run(0) else { return RavageThresholds { damage: 0, defend_keeps_land: None, defend_keeps_dahan: None } };
    let find = |keeps: &dyn Fn(u32, u32) -> bool, now: bool| -> Option<u32> {
        if now {
            return Some(0);
        }
        (1..=damage).find(|d| run(*d).is_some_and(|(_, b, l)| keeps(b, l)))
    };
    RavageThresholds { damage, defend_keeps_land: find(&|b, _| b == 0, blight0 == 0), defend_keeps_dahan: find(&|_, l| l == 0, lost0 == 0) }
}
```

Check the ravage outcome's field names in `crates/si-rules/src/invaders.rs` (`land`, `damage`, `blight`, `cascade`, `dahan_lost` as `invader_phase` reads them; add `damage` to the outcome if it is not kept) and that `Land.defend` is `u32`. A test on the fast cut: for a land the Ravage card names with Invaders in it, `damage > 0`, and `defend_keeps_land <= Some(damage)` when Some.

Check `play_options`' signature at `lib.rs` (`fn play_options(c: &PowerCard, presence: &BTreeMap<LandId, u32>, before: &BTreeMap<LandId, Counts>, graph: &Island) -> Vec<Play>`) and `PowerCard::fully_parsed` (exists; the review uses it).

Run: `cargo test -q --offline -p si-hindsight --test open_turn`
Expected: all pass.

- [ ] **Step 3: Commit**

```bash
git add crates/si-hindsight
git commit -m "hindsight: the unseen Minors and Majors, and how many play on a land this turn"
```

---

### Task 11: the live source: the Steam folder polled, a capture cut for `--replay`

**Files:**
- Create: `crates/si-cli/src/tui/source.rs`
- Modify: `crates/si-cli/src/main.rs` (`steam_dir`, `checkpoints_of`, `Checkpoint`, `checkpoint_matches`, `decode_save` become `pub(crate)`)
- Modify: `crates/si-cli/src/tui/mod.rs` (`mod source;`, `Msg::Live`)

- [ ] **Step 1: The types and the message**

In `tui/mod.rs`, `Msg` gains:

```rust
    /// The live source: a fresh record from the journal and the newest save's seed.
    Live(Box<source::LiveUpdate>),
```

`source.rs`:

```rust
//! Where the record comes from: a file or capture (built once), the Steam folder polled
//! while the game is played, or a capture cut at one save for the smoke test and for
//! judging the screen without the game running. Spec section 3.

use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use si_defs::Definitions;
use si_record::replay::Seed;
use si_record::{GameRecord, Replay};

use super::Msg;

pub(super) struct LiveUpdate {
    pub rec: GameRecord,
    /// The newest save, through the firewall; None until one decodes.
    pub seed: Option<Seed>,
    /// The footer's words: "live: T4 Fast, save 003-0089, journal 1,204 lines".
    pub words: String,
    /// Lands where the journal's board and the save's differ.
    pub disagree: Vec<String>,
}

/// The newest complete save in a Steam folder, by write time: a partial write fails to
/// decode and is left for the next poll.
fn newest_save(dir: &Path) -> Option<(PathBuf, SystemTime)> {
    let mut files: Vec<(PathBuf, SystemTime)> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| { let n = n.to_string_lossy(); n.ends_with(".dat") && (n.starts_with("save-Feedback-") || n.starts_with("save-Undo-") || n == "save-ContinueGame.dat") }))
        .filter_map(|p| std::fs::metadata(&p).and_then(|m| m.modified()).ok().map(|t| (p, t)))
        .collect();
    files.sort_by(|a, b| b.1.cmp(&a.1));
    files.into_iter().next()
}

/// The oldest decision save: where a journal that joined in progress starts from.
fn oldest_decision_save(dir: &Path) -> Option<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("save-Feedback-"))).collect();
    files.sort();
    files.into_iter().next()
}

fn seed_of(path: &Path) -> Option<Seed> {
    let snap = crate::decode_save(path)?;
    Some(si_steam::seed::from_snapshot(&snap, &path.file_name()?.to_string_lossy()))
}

/// Lands where the record's board now and the seed's differ, as footer words.
fn disagreements(rec: &GameRecord, seed: &Seed) -> Vec<String> {
    let mut out = Vec::new();
    for (id, c, _) in &seed.lands {
        let Some(l) = rec.lands.get(id) else { continue };
        let j = l.end;
        if (j.explorers, j.towns, j.cities, j.dahan, j.blight) != (c.explorers, c.towns, c.cities, c.dahan, c.blight) {
            out.push(format!("{id}: journal {}E {}T {}C Dh{} Bl{}, save {}E {}T {}C Dh{} Bl{}", j.explorers, j.towns, j.cities, j.dahan, j.blight, c.explorers, c.towns, c.cities, c.dahan, c.blight));
        }
    }
    out
}

/// The record and the open tab's seed from a journal and a save: the replay from setup,
/// or from the oldest decision save when the journal joined in progress.
pub(super) fn build(defs: &Definitions, journal: &[si_record::log::Line], undo: Option<&[(u64, u32)]>, newest: Option<&Path>, join_from: Option<&Path>, source: &str) -> LiveUpdate {
    let seed = newest.and_then(seed_of);
    let mut rec = Replay::new(Some(defs)).run(journal, undo, source);
    let mut joined = None;
    if !rec.meta.from_setup {
        if let Some(j) = join_from.and_then(seed_of) {
            rec = Replay::new(Some(defs)).with_seed(Some(j.clone())).run(journal, undo, source);
            joined = Some(j.save_name.clone());
        }
    }
    if let Some(s) = &seed {
        s.apply_meta(&mut rec.meta);
    }
    let disagree = seed.as_ref().map(|s| disagreements(&rec, s)).unwrap_or_default();
    let phase = seed.as_ref().map(|s| format!("T{} {}", s.turn_index + 1, si_hindsight::live::phase_word(s.phase))).unwrap_or_else(|| "no save yet".into());
    let save = seed.as_ref().map(|s| s.save_name.trim_start_matches("save-Feedback-").trim_end_matches(".dat").to_string()).unwrap_or_default();
    let mut words = format!("live: {phase}, save {save}, journal {} lines", journal.len());
    if let Some(j) = joined {
        words.push_str(&format!(", joined from {j}"));
    }
    if let Some(d) = disagree.first() {
        words.push_str(&format!(" | journal and save disagree in {d}"));
    }
    LiveUpdate { rec, seed, words, disagree }
}

/// Poll the Steam folder every 500 ms; a change in the journal's or the newest save's
/// write time or length sends a fresh `LiveUpdate`.
pub(super) fn spawn_live(dir: PathBuf, defs: Arc<Definitions>, tx: Sender<Msg>) {
    std::thread::spawn(move || {
        let journal = dir.join("log.txt");
        let mut last: Option<(SystemTime, u64, Option<(PathBuf, SystemTime)>)> = None;
        loop {
            let meta = std::fs::metadata(&journal).ok().and_then(|m| m.modified().ok().map(|t| (t, m.len())));
            let newest = newest_save(&dir);
            if let Some((mt, len)) = meta {
                let now = (mt, len, newest.clone());
                if last.as_ref() != Some(&now) {
                    if let Ok(lines) = si_record::replay::read_log(&journal) {
                        let update = build(&defs, &lines, None, newest.as_ref().map(|(p, _)| p.as_path()), oldest_decision_save(&dir).as_deref(), &journal.display().to_string());
                        if tx.send(Msg::Live(Box::new(update))).is_err() {
                            return;
                        }
                    }
                    last = Some(now);
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

/// `--replay <capture> --at Tn/Phase [--save part]`: the capture's journal cut at the
/// chosen save's stamp, that save as the newest. One update, no polling.
pub(super) fn replay_at(capture: &Path, defs: &Definitions, at: Option<(i64, String)>, save: Option<&str>) -> anyhow::Result<LiveUpdate> {
    let all = crate::checkpoints_of(capture)?;
    let chosen = all.iter().find(|c| c.on_final_path && at.as_ref().is_none_or(|(t, ph)| crate::checkpoint_matches(c, Some(*t), Some(ph.as_str()))) && save.is_none_or(|s| c.name.contains(s)));
    let Some(c) = chosen else { anyhow::bail!("no save on the final path matches --at/--save (si checkpoint {} lists them)", capture.display()) };
    // the recorder's stamp HHMMSS_mmm on the file name, as a log clock
    let stamp = c.path.file_name().unwrap().to_string_lossy().split("__").next().unwrap_or_default().to_string();
    let cut_ms = parse_stamp(&stamp).ok_or_else(|| anyhow::anyhow!("no clock on {}", c.path.display()))?;
    let (lines, undo, source) = si_record::replay::load(capture)?;
    let mut keep = true;
    let lines: Vec<si_record::log::Line> = lines.into_iter().filter(|l| { if let Some(ms) = l.ms { keep = ms <= cut_ms; } keep }).collect();
    let undo: Option<Vec<(u64, u32)>> = undo.map(|v| v.into_iter().filter(|(ms, _)| *ms <= cut_ms).collect());
    Ok(build(defs, &lines, undo.as_deref(), Some(&c.path), None, &source))
}

/// `HHMMSS_mmm` to milliseconds of the day.
fn parse_stamp(s: &str) -> Option<u64> {
    let (hms, ms) = s.split_once('_')?;
    if hms.len() != 6 {
        return None;
    }
    let h: u64 = hms[0..2].parse().ok()?;
    let m: u64 = hms[2..4].parse().ok()?;
    let sec: u64 = hms[4..6].parse().ok()?;
    let ms: u64 = ms.parse().ok()?;
    Some(((h * 60 + m) * 60 + sec) * 1000 + ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defs() -> Definitions {
        Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-record/tests/fixtures/engine"))).unwrap()
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../si-hindsight/tests/fixtures/open-turn")).join(name)
    }

    #[test]
    fn stamps_parse() {
        assert_eq!(parse_stamp("232604_642"), Some(((23 * 60 + 26) * 60 + 4) * 1000 + 642));
        assert_eq!(parse_stamp("bad"), None);
    }

    /// The scratch folder filled in stages from the growth and fast cuts: the record
    /// grows, the open tab's phase follows the newest save, a shorter journal rebuilds.
    #[test]
    fn a_folder_filled_in_stages() {
        let defs = defs();
        let dir = std::env::temp_dir().join(format!("si-live-source-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let strip = |from: &Path| -> String {
            // the capture's lines carry a clock the app's own journal does not; the replay reads both
            std::fs::read_to_string(from).unwrap()
        };
        let stage = |journal_from: &str, save_from: &str, save_as: &str| -> LiveUpdate {
            std::fs::write(dir.join("log.txt"), strip(&fixture(journal_from).join("log-stream.txt"))).unwrap();
            let gz = std::fs::read_dir(fixture(save_from).join("saves")).unwrap().next().unwrap().unwrap().path();
            let bytes = si_steam::source::read_save(&gz).unwrap();
            std::fs::write(dir.join(save_as), bytes).unwrap();
            let lines = si_record::replay::read_log(&dir.join("log.txt")).unwrap();
            build(&defs, &lines, None, newest_save(&dir).map(|(p, _)| p).as_deref(), None, "test")
        };
        let a = stage("growth", "growth", "save-Feedback-002-0047-SpiritPhase.dat");
        assert_eq!(a.rec.meta.turns, 3);
        assert_eq!(a.seed.as_ref().unwrap().phase, si_model::game::Phase::SpiritPhase);
        assert!(a.words.starts_with("live: T3 Growth"), "{}", a.words);
        std::thread::sleep(Duration::from_millis(20));
        let b = stage("fast", "fast", "save-Feedback-002-0054-FastPower.dat");
        assert_eq!(b.seed.as_ref().unwrap().phase, si_model::game::Phase::FastPower);
        assert!(b.rec.turns.iter().any(|t| t.turn == 3 && !t.before_fast.is_empty()));
        // an undo: the journal shrinks back to the growth cut; the newest save still says Fast
        let c = stage("growth", "fast", "save-Feedback-002-0055-FastPower.dat");
        assert_eq!(c.rec.meta.turns, 3);
        assert!(c.rec.turns.iter().any(|t| t.turn == 3 && t.before_fast.is_empty()), "the journal is the record");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
```

`si-cli` needs `si-steam` (it has it) and the test needs the hindsight fixtures (Task 7 committed them). Make `decode_save`, `checkpoints_of`, `Checkpoint`, `checkpoint_matches` and `steam_dir` in `main.rs` `pub(crate)`.

Run: `cargo test -q --offline -p si-cli source 2>&1 | tail -6`
Expected: the two unit tests pass.

- [ ] **Step 2: Commit**

```bash
git add crates/si-cli/src
git commit -m "si study: the live source, the Steam folder polled or a capture cut at one save"
```

---

### Task 12: the review worker fed over a channel

**Files:**
- Create: `crates/si-cli/src/tui/review.rs`
- Modify: `crates/si-cli/src/tui/mod.rs` (`run` uses it; `Msg::Turn` and `Msg::Meta` carry a generation)

- [ ] **Step 1: The worker**

```rust
//! The review worker: a long-lived thread fed `(generation, record, turns)` jobs, one
//! `review_turns` call per turn so turns land as they finish, `Msg::Finished` each time
//! its queue drains. A record that changed (an undo) gets a new generation; the screen
//! drops results tagged with an old one.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::{Progress, ReviewOptions};
use si_record::GameRecord;

use super::Msg;

pub(super) struct Job {
    pub generation: u64,
    pub rec: Arc<GameRecord>,
    pub turns: Vec<u32>,
}

pub(super) fn spawn(defs: Arc<Definitions>, opts: ReviewOptions, out: Sender<Msg>) -> Sender<Job> {
    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    std::thread::spawn(move || {
        let mut meta_sent = false;
        while let Ok(job) = rx.recv() {
            let on_progress = |p: Progress| {
                let _ = out.send(Msg::Progress(p));
            };
            for t in job.turns {
                let r = si_hindsight::review_turns(&job.rec, &defs, &opts, Some(&[t]), &on_progress);
                if !meta_sent {
                    let _ = out.send(Msg::Meta(Box::new((r.values.clone(), r.calibration.clone(), r.deck.clone()))));
                    meta_sent = true;
                }
                for tr in r.turns {
                    if out.send(Msg::Turn { generation: job.generation, review: Box::new(tr) }).is_err() {
                        return;
                    }
                }
            }
            // nothing queued behind this job: the footer may say so
            if rx.try_recv().map(|next| { let _ = tx_back(&next); }).is_err() {
                let _ = out.send(Msg::Finished);
            }
        }
    });
    tx
}
```

The `try_recv` peek above cannot put a job back; do it the simple way instead: keep a `VecDeque<Job>` inside the thread, drain `rx.try_recv()` into it before each turn, and send `Msg::Finished` when the deque is empty after a job. Write it that way (the snippet shows the intent; the deque version is the one to build).

In `mod.rs`: `Msg::Turn { generation: u64, review: Box<TurnReview> }`; `App` gains `generation: u64` and `review_tx: Sender<review::Job>`; `run` replaces the inline worker with `let review_tx = review::spawn(Arc::clone(&defs), opts, tx.clone()); review_tx.send(review::Job { generation: 0, rec: Arc::clone(&rec), turns: turns.clone() })`; the event loop inserts a `Msg::Turn` only when `generation == app.generation`.

Run: `cargo test -q --offline -p si-cli`
Expected: all pass, harness panes unchanged (the footer still reads "N turns reviewed").

- [ ] **Step 2: Commit**

```bash
git add crates/si-cli/src/tui
git commit -m "si study: the review worker takes turns over a channel, tagged by record generation"
```

---

### Task 13: the plan worker

**Files:**
- Create: `crates/si-cli/src/tui/plans.rs`
- Modify: `crates/si-cli/src/tui/mod.rs` (`Msg::Plans`)

- [ ] **Step 1: The worker**

```rust
//! The plan scorer: builds the open turn from the record and the seed, scores the `none`
//! plan, then each typed plan against it. Results carry the generation of the record and
//! the turn and phase they were scored on, so a stale batch is dropped.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::live::{open_turn, score_plan, OpenTurn};
use si_hindsight::{Calibration, Line};
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::GameRecord;

use super::Msg;

pub(super) struct Job {
    pub generation: u64,
    pub rec: Arc<GameRecord>,
    pub seed: Seed,
    pub texts: Vec<String>,
}

/// What the open tab shows: the open turn (its forecast and Explore pool computed here),
/// the `none` row, then one row per plan text in order, scored or refused.
pub(super) struct Scored {
    pub generation: u64,
    pub turn: u32,
    pub phase: Phase,
    pub open: Box<OpenTurn>,
    pub cands: Vec<String>,
    pub forecast: Vec<si_hindsight::deck::TurnForecast>,
    pub none: Line,
    pub rows: Vec<(String, Result<Line, String>)>,
}

pub(super) fn spawn(defs: Arc<Definitions>, cal: Calibration, out: Sender<Msg>) -> Sender<Job> {
    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    std::thread::spawn(move || {
        while let Ok(mut job) = rx.recv() {
            // only the newest job matters: drain the queue
            while let Ok(newer) = rx.try_recv() {
                job = newer;
            }
            let Some(open) = open_turn(Arc::clone(&job.rec), &defs, &job.seed, &cal) else {
                let _ = out.send(Msg::NoOpenTurn { generation: job.generation });
                continue;
            };
            let none = match score_plan(&defs, &open, None, "") {
                Ok(l) => l,
                Err(e) => {
                    let _ = out.send(Msg::NoOpenTurn { generation: job.generation });
                    eprintln!("the none plan failed: {e}");
                    continue;
                }
            };
            let rows: Vec<(String, Result<Line, String>)> = job.texts.iter().map(|t| (t.clone(), score_plan(&defs, &open, Some(&none), t))).collect();
            let cands: Vec<String> = si_hindsight::explore_candidates(&open.rec, &defs, &open.deck, &open.t).into_iter().filter_map(|c| c.into_iter().next()).collect();
            let island = si_rules::verify::island_for(&open.rec, &defs, &open.t.before_fast);
            let forecast = si_hindsight::deck::forecast(&open.rec, &defs, &open.deck, &open.t, &island, &open.rules, 3, Some(&cands));
            let scored = Scored { generation: job.generation, turn: open.turn, phase: open.phase, cands, forecast, none, rows, open: Box::new(open) };
            if out.send(Msg::Plans(Box::new(scored))).is_err() {
                return;
            }
        }
    });
    tx
}
```

`Msg` gains `Plans(Box<plans::Scored>)` and `NoOpenTurn { generation: u64 }`. `deck::forecast`'s `rules` parameter is `&Rules`; check its signature (`deck.rs:136`) and adjust the call. si-cli gains nothing new in `Cargo.toml` (si-rules, si-hindsight are there).

Run: `cargo build -q --offline -p si-cli`
Expected: clean (the messages are not yet handled; that is Task 14; an unused warning is fine for one commit).

- [ ] **Step 2: Commit**

```bash
git add crates/si-cli/src/tui
git commit -m "si study: the plan worker scores none, then every plan against it, on the open turn"
```

---

### Task 14: the open tab in the app: `--live`, `--replay`, the tab, the keys, the footer

**Files:**
- Modify: `crates/si-cli/src/tui/mod.rs` (`run`, `App`, the event loop, the help text)
- Modify: `crates/si-cli/src/tui/tabs.rs` (the open tab's label)
- Modify: `crates/si-cli/src/tui/panels.rs` (`TurnKind::Open`)

- [ ] **Step 1: The state**

In `mod.rs`:

```rust
/// The turn being played, as the plan worker last scored it.
pub(super) struct OpenState {
    pub turn: u32,
    pub phase: Phase,
    pub seed: Seed,
    pub scored: Option<plans::Scored>,
}
```

`App` gains: `live: bool`, `open: Option<OpenState>`, `plan_texts: BTreeMap<u32, Vec<String>>` (the plans typed per turn, kept across re-scores), and `tries` becomes `BTreeMap<u32, Vec<si_hindsight::Line>>` so a closed turn can carry several try rows (`rows()` pushes one `try` row per line; `t` on a closed tab pushes to the turn's vector; `d` removes the one under the cursor), `plan_tx: Sender<plans::Job>`, `source_words: String`, `generation: u64`. `App::turns` stays the closed turns; a new `tabs(&self) -> Vec<Tab>` with `enum Tab { Closed(u32), Open(u32, Phase) }` appends the open tab when `self.open.is_some()`; `cur` indexes `tabs()`. `turn()` returns the tab's turn; `review()` returns None on the open tab; `is_open_tab(&self) -> bool`.

`rows()` on the open tab:

```rust
    fn plan_rows(&self) -> Vec<RowView> {
        let Some(sc) = self.open.as_ref().and_then(|o| o.scored.as_ref()) else { return vec![] };
        let mut out = vec![RowView { tag: "none".into(), level: 0, key: "none".into(), line: sc.none.clone(), children: 0, open: false }];
        for (i, (text, r)) in sc.rows.iter().enumerate() {
            let tag = ((b'a' + (i % 26) as u8) as char).to_string();
            match r {
                Ok(l) => out.push(RowView { tag, level: 0, key: format!("plan:{text}"), line: l.clone(), children: 0, open: false }),
                Err(e) => {
                    // a refused plan keeps its row so `d` can remove it; the error sits in the notes
                    let mut l = sc.none.clone();
                    l.plays.clear();
                    l.outcome = Default::default();
                    l.outcome.notes = vec![format!("refused: {e}")];
                    l.informed = f64::NAN;
                    out.push(RowView { tag, level: 0, key: format!("plan:{text}"), line: l, children: 0, open: false });
                }
            }
        }
        out
    }
```

- [ ] **Step 2: `run` parses the live switches**

```rust
    let live = args.iter().any(|a| a == "--live");
    let steam: PathBuf = args.iter().position(|a| a == "--steam").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(crate::steam_dir);
    let replay: Option<PathBuf> = args.iter().position(|a| a == "--replay").and_then(|i| args.get(i + 1)).map(PathBuf::from);
    let at: Option<(i64, String)> = args.iter().position(|a| a == "--at").and_then(|i| args.get(i + 1)).and_then(|s| { let (t, ph) = s.split_once('/')?; Some((t.trim_start_matches('T').parse().ok()?, ph.to_string())) });
    let save_pick: Option<String> = args.iter().position(|a| a == "--save").and_then(|i| args.get(i + 1)).cloned();
```

When `live`: no path argument is required; the record starts empty (`GameRecord::default()`), `source_words = "live: waiting for the journal"`; with `--replay`, `source::replay_at(&capture, &defs, at, save_pick.as_deref())?` is sent to the app as one `Msg::Live` before the loop; else `source::spawn_live(steam, Arc::clone(&defs), tx.clone())`. `is_capture` is false live (`i` is refused anyway). The plan worker is spawned in both modes: `let plan_tx = plans::spawn(Arc::clone(&defs), calibration.clone().unwrap_or_default(), tx.clone());`.

- [ ] **Step 3: The event loop handles the new messages**

```rust
                Msg::Live(u) => {
                    let u = *u;
                    let old = Arc::clone(&app.rec);
                    app.rec = Arc::new(u.rec);
                    app.source_words = u.words;
                    app.generation += 1;
                    // closed turns whose record changed lose their review and are queued again
                    let closed = si_hindsight::closed_turns(&app.rec);
                    app.reviews.retain(|t, _| old.turn(*t) == app.rec.turn(*t) && old.spirit_turn(&app.rec.meta.spirits[0], *t) == app.rec.spirit_turn(&app.rec.meta.spirits[0], *t) && closed.contains(t));
                    let todo: Vec<u32> = closed.iter().copied().filter(|t| !app.reviews.contains_key(t)).collect();
                    app.turns = closed;
                    if !todo.is_empty() {
                        app.running = true;
                        let _ = app.review_tx.send(review::Job { generation: app.generation, rec: Arc::clone(&app.rec), turns: todo });
                    }
                    app.pool_at = BTreeMap::new();
                    if app.rec.meta.turns < old.meta.turns.saturating_sub(1) || (app.rec.meta.from_setup && app.rec.meta.turns <= 1 && old.meta.turns > 1) {
                        // a new game: nothing typed for the old one applies
                        app.plan_texts.clear();
                        app.tries.clear();
                    }
                    // a turn that just closed keeps its plans as try rows (spec section 2, "After the turn closes")
                    if let Some(o) = &app.open {
                        if closed.contains(&o.turn) {
                            let texts = app.plan_texts.get(&o.turn).cloned().unwrap_or_default();
                            let lines: Vec<si_hindsight::Line> = texts.iter().filter_map(|x| si_hindsight::try_line(&app.rec, app.defs, o.turn, x.split('|').next_back().unwrap_or(x).trim()).ok()).collect();
                            if !lines.is_empty() {
                                app.tries.insert(o.turn, lines);
                            }
                        }
                    }
                    // the open tab: the seed names it; the plans typed for that turn are re-scored
                    match u.seed {
                        Some(seed) if si_hindsight::live::is_open_phase(seed.phase) && seed.ending.is_none() => {
                            let turn = seed.turn_index + 1;
                            let texts = app.plan_texts.get(&turn).cloned().unwrap_or_default();
                            app.open = Some(OpenState { turn, phase: seed.phase, seed: seed.clone(), scored: None });
                            let _ = app.plan_tx.send(plans::Job { generation: app.generation, rec: Arc::clone(&app.rec), seed, texts });
                        }
                        _ => app.open = None,
                    }
                    let n = app.tabs().len();
                    if app.cur >= n {
                        app.cur = n.saturating_sub(1);
                    }
                }
                Msg::Plans(sc) => {
                    if sc.generation == app.generation {
                        if let Some(o) = app.open.as_mut() {
                            o.scored = Some(*sc);
                        }
                    }
                }
                Msg::NoOpenTurn { generation } => {
                    if generation == app.generation {
                        if let Some(o) = app.open.as_mut() {
                            o.scored = None;
                        }
                    }
                }
```

`spirit_turn` needs `app.rec.meta.spirits.first()`; guard the empty case. The first `Msg::Live` of a live session should also move `cur` to the open tab: when `app.open` went from None to Some, set `app.cur = app.tabs().len() - 1`.

Keys: the `t` prompt's Enter, on the open tab, pushes the text to `app.plan_texts[turn]` and sends a `plans::Job` with every text for the turn; on a closed tab it stays `try_line`. `d`: on the open tab, with the cursor on row `i >= 1`, removes `plan_texts[turn][i - 1]` and re-sends the job; on a closed tab it removes the `try` row (`app.tries.remove(&t)`). `i` on the open tab: `app.popup = Some(("inject".into(), vec!["the game is at this position already".into()]))`. The prompt title reads `plan` on the open tab: `" plan (G2 e4@C4 | CS>C3 FCD>C4 on the Growth tab, plays only after it; Enter scores it) "`. The help text gains `d  delete the plan or try row under the cursor` and a line on `--live`.

`draw_footer`: `format!("{} | {}", app.source_words, status)` when `app.live`. `draw_header`: ` STUDY ` becomes ` LIVE ` when `app.live`.

- [ ] **Step 4: The tab bar and `TurnKind::Open`**

In `tabs.rs`, labels come from `app.tabs()`: a closed tab as today; an open tab `format!("T{turn} {}", si_hindsight::live::phase_word(phase))` in `ACCENT`, narrowing to `T{turn}` with the others. In `panels.rs`:

```rust
pub(super) enum TurnKind<'r> {
    Closed(&'r TurnReview),
    Open(&'r plans::Scored),
}
```

`App::turn_ctx` builds the Open kind when the current tab is open and `scored` is Some: `t = Some(&sc.open.t)`, `s = Some(&sc.open.s)`, `panel` by `sc.open.spirit`, `island` from `sc.open.t.before_fast` (or `before_slow` on the Slow tab), `rows = self.plan_rows()`, `pool = sc.open.seed.blight_pool.map(|p| (p, sc.open.seed.blight_flipped))`. The accessors in Task 3 gain the Open arms: `hand()` = `sc.open.s.hand_after` plus `cards_played`, `energy()` = `actual_position(..).energy` (compute once in `Scored` as `pub energy: i64, pub plays: i64` set by the worker from `score_plan`'s position; add those two fields to `Scored` and set them in `plans.rs` from `si_hindsight::actual_position`), `forecast()` = `sc.forecast`, `offered()` = `sc.open.s.offered`, `actual()` = None, `notes()` = `&[]`, `choices()` = `&[]`, `terror()` = `sc.open.seed.terror`, `best_growth()` = `&[]`. `draw_turn` on the open tab with `scored` None draws "scoring the open turn..." in dim.

Run: `cargo build -q --offline -p si-cli && cargo test -q --offline -p si-cli`
Expected: clean; the harness unchanged (closed tabs draw as before).

- [ ] **Step 5: A first look, by hand**

Run: `cargo run -q --offline -p si-cli -- study --live --replay $SI_CAPTURE_DIR/live-e392726411c9 --at T3/FastPower`
Expected: the tab bar ends in `T3 Fast` open in cyan; the closed tabs T1 and T2 review; the Turn screen shows the open turn's panels with the LINES panel listing `none` (Task 16 renames and reshapes it). `t`, type `Mists>D5` (any card code from the HAND panel and a land), Enter: a row `a` appears. `d` removes it. `q` quits.

- [ ] **Step 6: Commit**

```bash
git add crates/si-cli/src/tui
git commit -m "si study --live: the open tab, plans typed with t and removed with d, the live footer"
```

---

### Task 15: the panels' open branches: PLAN, THINK ABOUT, POSITION after, INVADER PHASE live

**Files:**
- Modify: `crates/si-cli/src/tui/panels.rs`

- [ ] **Step 1: PLAN**

In `lines_panel`, when `ctx.kind` is `Open(sc)`: title `PLAN  your plans from this position, in the order entered; none = nothing beyond what the journal shows`; headers `["", "growth", "cards", "innates", "score", "vs none", "tiers", "produced"]` (tiers shown under the same width rule); per row: `tag`; growth `l.growth_notation(&code)` (`(as taken)` appended when `l.growth_actual` and the phase is past Growth); cards `l.cards_notation`; innates `l.innates_notation`; score `format!("{:+.2}", l.informed)` or `-` when NaN; `vs none`: `""` for `none`, else by `l.confidence` and `sc.cands.len()`: when `cands.len() > 1`, `format!("{} of {}", (c * n).round(), n)`; when 1, `about N in M` from the share (`share_words(c)`: 0.0..0.125 "about 0 in 4", ..0.375 "about 1 in 4", ..0.625 "about 1 in 2", ..0.875 "about 3 in 4", else "about 4 in 4"); `produced` = `l.outcome.summary()` with the LOSES/FLIPS mark from `ctx.pool` as today. Style: `none` bold, a plan plain, a refused plan dim.

- [ ] **Step 2: THINK ABOUT**

In `reading_panel`, the Open arm builds sentences from `sc.none` and `sc.open`:

```rust
fn think_about(ctx: &TurnCtx, sc: &plans::Scored) -> Vec<String> {
    let open = &sc.open;
    let t = &open.t;
    let seed = &open.seed;
    let none = &sc.none;
    let mut out = Vec::new();
    // each Ravage land where something is at stake, from the none row's board after
    for l in ctx.island.lands.values() {
        let hit = t.ravage.iter().filter_map(|c| ctx.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|tr| c.matches(tr, l.coastal)));
        if !hit || l.c.invaders() == 0 {
            continue;
        }
        let mut stake = Vec::new();
        if l.c.dahan > 0 { stake.push(format!("{} Dahan", l.c.dahan)); }
        if l.c.presence > 0 { stake.push("your presence".into()); }
        // the simulator's own numbers, every adversary and scenario rule applied
        let th = si_hindsight::live::ravage_thresholds(&ctx.island, ctx.defs, &t.ravage, &open.rules, &l.id);
        let saves = match (th.defend_keeps_land, th.defend_keeps_dahan) {
            (Some(a), Some(b)) if l.c.dahan > 0 && b != a => format!("; Defend {b} keeps the Dahan, Defend {a} keeps the land"),
            (Some(a), _) => format!("; Defend {a} keeps the land"),
            (None, _) => String::new(),
        };
        out.push(format!("{} Ravages for {}{}{}.", l.id, th.damage, if stake.is_empty() { String::new() } else { format!(" with {} at stake", stake.join(" and ")) }, saves));
    }
    // the Builds
    let builds: Vec<String> = ctx.island.lands.values().filter(|l| t.build.iter().filter_map(|c| ctx.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|tr| c.matches(tr, l.coastal))) && l.c.invaders() > 0).map(|l| { let a = none.outcome.after.get(&l.id).copied().unwrap_or_default(); format!("{} {}", l.id, if a.cities > l.c.cities { "a City" } else if a.towns > l.c.towns { "a Town" } else { "nothing" }) }).collect();
    if !builds.is_empty() {
        out.push(format!("Builds: {}.", builds.join(", ")));
    }
    if let Some(p) = seed.blight_pool {
        out.push(format!("Blight card: {p} in the pool{}; doing nothing adds {} this turn.", if seed.blight_flipped { " (flipped)" } else { "" }, none.outcome.blight));
    }
    let hand = open.s.hand_after.len() + open.s.cards_played.len();
    let plays = sc.plays;
    out.push(format!("Hand {hand}, plays {plays}{}.", if (hand as i64) < plays { ": a Reclaim next turn unless this turn's growth draws" } else { "" }));
    if open.phase == Phase::SpiritPhase {
        if let Some(panel) = ctx.panel {
            let draws: Vec<(PowerKind, u8)> = panel.growth.groups.iter().filter_map(|g| { let y = g.yield_(); (y.cards > 0).then(|| (if y.major { PowerKind::Major } else { PowerKind::Minor }, g.number)) }).collect();
            for (kind, group) in draws.into_iter().take(2) {
                for l in ctx.island.lands.values().filter(|l| l.c.invaders() > 0 && t.ravage.iter().filter_map(|c| ctx.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|tr| c.matches(tr, l.coastal)))).take(2) {
                    let dc = si_hindsight::live::draw_count(ctx.defs, open, kind, &l.id);
                    out.push(format!("G{group} draws a {:?}: of the {} unseen, {} play this turn on {} ({} not readable).", kind, dc.unseen, dc.playable, l.id, dc.unreadable));
                }
            }
        }
    }
    out
}
```

A growth option that draws yields `cards` with no Minor or Major flag (the kind belongs to the draft), so THINK ABOUT writes one sentence per kind, Minors and Majors, for each Ravage land, naming the groups that draw. `ravage_thresholds` is built in Task 10 beside `draw_count`: it runs `si_rules::invaders::ravage` on a clone of the island for the card(s) naming the land, reads the damage dealt from the outcome, then raises the land's `defend` from 0 upward until the outcome adds no Blight (`defend_keeps_land`) and until it loses no Dahan (`defend_keeps_dahan`), each `None` when no Defend up to the damage does it. Check the growth yield's field names (`GrowthOptionDef::yield_()` returns `cards`, `energy`, `presence`, `reclaim`, `plays_this_turn`; whether a Major is a separate flag or a kind on the option: read `si_model::spirit` and adjust `y.major`). The panel title: `THINK ABOUT  from the none row; nothing here is a verdict`.

- [ ] **Step 3: POSITION's `after` column and INVADER PHASE live**

In `position_panel`, on the open tab the last column is `after` when the land's steps have not run (no recorded visit this turn) and `result` when they have; `after` words from the selected row's `outcome.after`: `format!("{}{}{}", pieces_short(&a), if a.blight != l.c.blight { format!(" Bl{}", a.blight) } else { String::new() }, if a.dahan != l.c.dahan { format!(" Dh{}", a.dahan) } else { String::new() })`, blank when nothing changes. The `steps` column reads `t.ravage`/`t.build` (filled from the seed) and `ctx.forecast()` as today.

In `invaders_panel`, the Open arm:

- `Ravage` / `Build` / `HI build` lines as today from `ctx.t` (the slots are filled).
- `Explore`: `face down; Stage {stage}, {n} of {m} left: {terrains}` from `sc.forecast.first()` (its `stage`, `cards`) and the cards' terrains (`defs.invader_cards[c].terrains` words); once `t.explore` is recorded, `came {card}` as today.
- `Blight card`: `{name}, {pool} in the pool{flipped}; plan {tag} adds {outcome.blight}, leaves {pool - blight}` for the selected row.
- `Fear`: `{fear_cards_banked} cards earned, Terror {terror}, {fear_generated} of {fear_required} toward the next`.
- `Event`: `not drawn` until `t.events` is non-empty, then as today (choices from `t.choices` with `source_kind == "Event"`, selected words only, no verdict).
- Per recorded visit this turn, a line `came {land}: {visit words}; plan said {after words}` for lands where the two differ, `as planned` where they agree.

- [ ] **Step 4: SPIRIT, GROWTH, HAND, LINE on the open tab**

`spirit_panel`: Energy and plays from `sc.energy`/`sc.plays`; the rest as today (the ctx's track and presence). `growth_panel`: on the Growth tab no group is marked taken; the selected plan's group is marked `<- plan`; `offered` from `ctx.offered()`. `hand_panel`: the `in` column reads `made` for a card in `open.s.cards_played`, `plan` for one the selected row plays, `both`. `line_panel`: `scores` reads `format!("score {:+.2} over {} Explore cards; Event and Fear cards expected {:+.2}{}", l.informed, sc.cands.len().max(1), l.expected, l.confidence.map(|c| format!("; beats none {:.0}%", c * 100.0)).unwrap_or_default())`; `made` notes show as `note` lines as today.

Run: `cargo build -q --offline -p si-cli && cargo test -q --offline -p si-cli`
Expected: clean; the harness unchanged.

- [ ] **Step 5: Look at both cuts by hand**

Run: `cargo run -q --offline -p si-cli -- study --live --replay $SI_CAPTURE_DIR/live-e392726411c9 --at T3/SpiritPhase` then `--at T3/FastPower`.
Expected on the Growth tab: PLAN with `none`; GROWTH listing the groups with none taken; THINK ABOUT with the Ravage lands, the Blight card and the draw counts; `t` then `G1 | ` is refused with the groups named (no placement token); `t` then a position copied from the error's examples is accepted and its row shows a growth value. On the Fast tab: `t` then two card codes with lands from HAND; the row's `vs none` reads `N of M`; POSITION's `after` shows the Ravage lands' Blight and Dahan under the selected row. Fix what reads wrong before committing; the spec's section 2 is the reference.

- [ ] **Step 6: Commit**

```bash
git add crates/si-cli/src/tui
git commit -m "si study --live: PLAN, THINK ABOUT, the board after each plan, the Invader phase as it stands"
```

---

### Task 16: the live smoke cases

**Files:**
- Modify: `crates/si-cli/tests/tui_smoke.rs`
- Create: `crates/si-cli/tests/fixtures/tui/live-growth.txt`, `live-growth-plan.txt`, `live-growth-d.txt`, `live-fast.txt`, `live-fast-plan.txt`

- [ ] **Step 1: The cases**

```rust
#[test]
fn live_screens_on_the_shroud_cuts() {
    if !ready() {
        return;
    }
    let cap = capture_dir();
    let cap = cap.to_str().unwrap();
    {
        let s = Session::start(&["--live", "--replay", cap, "--at", "T3/SpiritPhase", "--fixed-growth"]);
        let pane = s.wait_for("turns reviewed", Duration::from_secs(180));
        let pane = s.wait_for("plans scored: 0", Duration::from_secs(60));
        assert!(pane.contains("T3 Growth"), "the open tab:\n{pane}");
        check("live-growth", &pane);
        // a refused plan comes back as a row with its reason (no popup, nothing to close), then d removes it
        s.key("t");
        s.keys("G9 |");
        s.key("Enter");
        let refused = s.wait_for("refused: no growth", Duration::from_secs(60));
        assert!(refused.contains("plans scored: 1"), "{refused}");
        s.key("j");
        s.key("d");
        s.wait_for("plans scored: 0", Duration::from_secs(60));
        // a growth position the panel offers, scored against none
        s.key("t");
        s.keys(GROWTH_PLAN);
        s.key("Enter");
        let pane = s.wait_for("plans scored: 1", Duration::from_secs(60));
        check("live-growth-plan", &pane);
        s.key("j");
        check("live-growth-d", &s.key("d"));
    }
    {
        let s = Session::start(&["--live", "--replay", cap, "--at", "T3/FastPower", "--fixed-growth"]);
        let pane = s.wait_for("turns reviewed", Duration::from_secs(180));
        let pane = s.wait_for("plans scored: 0", Duration::from_secs(60));
        assert!(pane.contains("T3 Fast"), "the open tab:\n{pane}");
        check("live-fast", &pane);
        s.key("t");
        s.keys(FAST_PLAN);
        s.key("Enter");
        check("live-fast-plan", &s.wait_for("plans scored: 1", Duration::from_secs(60)));
    }
}
```

`GROWTH_PLAN` and `FAST_PLAN` are consts from what the Task 15 hand run showed to be accepted: `G2 e5@D3 cp3@D7 |` (a position the refusal note lists) and `LSH` (the Slow innate the Fast-cut position reaches; both hand cards are played already there). `Session::keys(text)` sends a whole string with `tmux send-keys -l`, so spaces and `|` arrive as typed. The PLAN title reads `plans scored: N`, which every wait uses; a refused plan is a row, not a popup.

- [ ] **Step 2: Bless, then run twice**

Run: `SI_TUI_BLESS=1 cargo test -q --offline -p si-cli --test tui_smoke live_screens` then `cargo test -q --offline -p si-cli --test tui_smoke` twice.
Expected: pass each time; the five live fixtures committed. Read `live-fast-plan.txt`: PLAN shows `none` and `a`, `vs none` reads `N of M`, POSITION has `after`.

- [ ] **Step 3: Commit**

```bash
git add crates/si-cli/tests
git commit -m "si study --live: smoke cases on the Shroud cuts at T3 Growth and T3 Fast"
```

---

### Task 17: the runbook and the memory

**Files:**
- Modify: `docs/si-cli.md` (a section 5h after 5g)
- Modify: `/home/brett/.claude/projects/-home-brett-repos-spirit-island/memory/project_live_study_tui.md` (what was built, what was left)

- [ ] **Step 1: Section 5h**

```markdown
## 5h. Live: the study board on the game being played

```
si study --live                                   # the Steam folder ($SI_STEAM_DIR or the Handelabra path)
si study --live --steam <dir>                     # another folder
si study --live --replay <capture> --at T3/SpiritPhase [--save <name part>]   # a capture cut at that save, no game needed
```

Turns already played are the closed tabs, reviewed as soon as the journal has moved
past them. The turn being played is the open tab (`T4 Growth`, `T4 Fast`, `T4 Ravage`,
`T4 Slow`, in cyan). On it:

- **PLAN** (in LINES' place): `none` first, then every plan you typed, in that order,
  never ranked. `t` opens the prompt: on the Growth tab a growth before `|` and plays
  after it (`G2 e4@C4 | CS>C3 FCD>C4`; a growth the panel does not offer is refused with
  the groups and example positions); on every later tab plays only; a Fast card after
  the Fast phase is refused. `score` is the informed score (averaged over the Explore
  cards still in the stage, the growth position's value and the expectation for the
  face-down Event and Fear cards included); `vs none` is how often the plan beats doing
  nothing, as `7 of 9` Explore cards (or `about 3 in 4` when the Explore is known).
  `d` deletes the plan under the cursor. A play the journal has since recorded is not
  applied twice (LINE notes it `made already`). The engine's own ranked lines never
  show on the open tab; they appear when the turn closes.
- **THINK ABOUT** (in READING's place): the Ravage lands with what is at stake and what
  doing nothing adds, the Builds, the Blight card's pool, hand against plays, and on the
  Growth tab how many unseen Minors or Majors would play on each Ravage land this turn.
- **POSITION**'s last column is `after` (what the steps do there under the selected plan,
  with every adversary modifier) until the step is recorded, then `result`.
- **INVADER PHASE** lists the slots, the Explore's pool, the Blight card, the Fear
  counts, the Event once drawn, and each recorded step against what the plan said.
- The footer reads `live: T4 Fast, save 003-0089, journal 1,204 lines | 3 turns
  reviewed`, and names a land where the journal's board and the save's disagree.

Reads only: the journal `log.txt` (replayed from scratch on every change; an undo
rewrites it) and the newest complete save, through the firewall (`si_steam::seed`: no
deck order, no face-down card name ever crosses). A journal that joined in progress
(played on from `si inject`) is seeded from the oldest decision save in the folder.
First spirit only in this version.
```

- [ ] **Step 2: The memory file**

Append to `project_live_study_tui.md` a dated paragraph: built on 2026-10-21 per the plan, the commits, what was left out (section 10 of the spec), anything the hand runs showed that the spec did not foresee.

- [ ] **Step 3: Commit**

```bash
git add docs/si-cli.md
git commit -m "docs: si study --live in the runbook"
```

---

## Done when

- `cargo test -q --offline` passes for every default member, the firewall seed test included (or skipped by its own rule, reported).
- `cargo test -q --offline -p si-cli --test tui_smoke` passes twice in a row with the ten fixtures.
- `si study $SI_CAPTURE_DIR/live-e392726411c9 --fixed-growth` draws what it drew at `2fbf8e0` (the five study fixtures never changed).
- `si study --live` on the real Steam folder during a game shows the open tab moving with the phase; Brett plays one real game with it and his notes go in a playtest doc.
