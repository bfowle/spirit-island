# E. Code-state audit: uncommitted work in `spirit-island-steam-companion` (branch `companion-v2`)

Audited read-only on 2026-10-19. Repository: `/home/brett/repos/spirit-island-steam-companion`, workspace `tools/si-live`. HEAD is `0efc7a1 native app N2: phase views and a Rules view`. The branch has no upstream and there are no stashes. `crates/si-steam/tests/zz_probe.rs` is gone (the directory holds only `backtest.rs` and `firewall.rs`).

Totals: 18 tracked files modified (+1160 / −309), 9 files deleted and staged (−1218), 14 untracked files (3,290 lines: 1,217 core, 1,895 app, 41 shader, 137 doc).

## 1. Inventory of uncommitted changes

Category key: CORE MATH = si-common stats/rules; DATA/FIREWALL = si-steam; TOOLKIT = si-ui; LAYOUT/VIEW = si-app views; BUILD/TOOLING = scripts, lockfile, CLI; DOCS.

### Modified (tracked)

| Path (under `tools/si-live/`) | What it does | Category | Size | Tests |
|---|---|---|---|---|
| `common/src/rules.rs` | `add_built`: `"Explorer"` now increments `l.explorers` (previously fell through to `towns += 1`). This is the real bug fix: next-turn lookahead added Towns where it meant Explorers. | CORE MATH | +1 | No direct unit test of the Explorer arm. Covered indirectly by the si-steam backtests (0 mismatches on 7 games per the account) and by `adversary.rs` callers. |
| `common/src/companion_stats/invaders.rs` | Refactor: `next_turn` split into `advance()` (board at start of next turn, no spirit action) + `forecast_at`; new `time_passes()` and `next_turn_start()`; `phase_position`/`forecast_at` made `pub(crate)`. Also a behavior change: the Ravage card is now pushed onto `inv.discard` when cards advance (so it stays "seen"). | CORE MATH | +58 / −20 | No unit tests in the file (0 `#[test]`). Exercised by `outlook.rs` tests and the steam backtests. |
| `common/src/companion_stats/mod.rs` | Registers 5 new modules; `CompanionStats` gains `ledger`, `outlook`, `fear_pace`, `curve` (all `#[serde(default)]`); `RoundSummary` gains `energy_income`, `plays`, `ravaged`, `built`, `explored`; `round_summaries` records per-turn Invader cards via new `slot_cards()` and back-fills a turn's Explore from the next turn's Build; `phase_blight` made `pub(crate)`. | CORE MATH | +74 / −2 | Existing mod tests (2 markers) unchanged; no new test for `slot_cards` or the back-fill loop. |
| `common/src/companion_stats/adversary.rs` | `LossClock.unit` field (singular unit name) set per adversary; Russia Beast pluralization. | CORE MATH | +14 / −2 | Existing tests (8 markers) still pass; `unit` not asserted. |
| `common/src/companion_stats/backtest.rs` | New `OutlookCheck` + `outlook_check()`: replays each turn's Ravage snapshot through `outlook()` and compares the explored card's row to the actual Explore. | CORE MATH (verification tooling) | +58 | No unit test; it is itself a check, driven by `dump outlook`. |
| `common/src/companion_stats/spirits.rs` | `track_value` made `pub(crate)` (used by `round_summaries`). | CORE MATH | +1 / −1 | n/a |
| `steam/src/bin/dump.rs` | New `outlook` subcommand printing `outlook_check` rows (OK/MISS per turn). | BUILD/TOOLING | +17 / −1 | n/a (binary) |
| `ui/Cargo.toml` | Adds `resvg = "0.45"`. | TOOLKIT | +1 | n/a |
| `ui/src/lib.rs` | Re-exports `LAYER_BASE/MAP/OVERLAY`, `Pt`, `IconSource`. | TOOLKIT | +2 / −2 | n/a |
| `ui/src/painter.rs` | SDF shapes generalized (kind + softness params); 3 layers instead of 2; `MeshVertex`, `IconDraw`; `Ui::pulse`, `layer`, `glow`, `line`, `circle`, `polygon`, `quad`, `triangle`, `arrow`, `icon`; `time`/`animating` fields; `Key::Tab`. | TOOLKIT | +180 / −4 | None (0 tests in si-ui except `geom.rs`). Validated only by building and screenshots. |
| `ui/src/rect.wgsl` | Shader: `sd_segment` for lines, soft falloff for glows, margin for softness. | TOOLKIT | +49 / −9 | None |
| `ui/src/render.rs` | Second pipeline (mesh, `mesh.wgsl`); icon registry + `rasterize()` via resvg (SVG mask / PNG color) fed to glyphon `prepare_with_custom`; 3 text renderers; per-layer mesh ranges. | TOOLKIT | +264 / −80 | None |
| `ui/src/offscreen.rs` | `render_png` takes `icons` and `time` so screenshots include icons and a fixed animation phase. | TOOLKIT | +15 / −1 | None |
| `app/src/main.rs` | CLI: `--tab` replaced by `--page`, `--whatif`, `--drawer`; icons registered on both GPU paths; `ui.time` fed from `Instant`; redraw at ~60 fps while `animating`, else 60 ms poll; `Tab` key mapped. | LAYOUT/VIEW (+ CLI) | +67 / −14 | None |
| `app/src/views/mod.rs` | Rewritten: `Page::{Ledger,Curve}`, `Both` (base vs what-if stats), `Toggle`, `STEPS` ribbon, `step_index`, what-if cache keyed on `(st.seq, edits)`, moment-reset of edits, status line. Replaces top bar/phase tabs/3-column layout. | LAYOUT/VIEW | +393 (net rewrite) | None |
| `app/src/views/fmt.rs` | Removes `plural`, `card_label`, `card_chip` (only the deleted views used them). | LAYOUT/VIEW (helpers) | +1 / −28 | None |
| `app/build-windows.sh` | `rm -f` the exe before `cp`. | BUILD/TOOLING | +3 | n/a |
| `Cargo.lock` | resvg dependency tree (usvg, rustybuzz, tiny-skia path, image-webp, zune-jpeg, gif, …) and a second `png` version (0.17.16 alongside 0.18.1). | BUILD/TOOLING | +240 | n/a |

### Deleted (staged in the index)

All nine from commit `0efc7a1` (N2): `app/src/views/{clocks,event,fear,island,phase,plays,rules,slow,spirit}.rs` — the old dashboard-panel views (phase view, island, fear, clocks, spirit, plays, rules, event, slow). LAYOUT/VIEW, −1,218 lines, no tests. Note the deletions are staged while everything else is unstaged; a `git commit` without `-a` right now would commit only these deletions and break the build (mod.rs still references nothing of them, but main.rs/mod.rs changes are unstaged, so the committed tree would have `mod clocks;` etc. pointing at deleted files).

### Untracked

| Path | What it does | Category | Lines | Tests |
|---|---|---|---|---|
| `common/src/companion_stats/ledger.rs` | Lumia ledgers: `Budget`/`Spend` per Blight card, Fear card, Invader deck, and each adversary loss clock; shortfall, next-turn spend, exact Defend per Ravage (`damage − health + 1`), `urgent` for Fast-Phase-checked clocks already at limit. | CORE MATH | 286 | 2 tests (cascade/shortfall on Prussia T5 figures; Heavy Mining extra). |
| `common/src/companion_stats/outlook.rs` | Exact enumeration of the next 4 Invader Phases over every unseen-card order (cards grouped by kind), no spirit action: per-path Blight/pieces/lands, `next_cards` rows, `flipped_by`, `lost_by` (uses `p_loss`), `deck_out`. | CORE MATH | 303 | 2 tests (order count 3×2×2=12; deck-out ends the walk). |
| `common/src/companion_stats/pace.rs` | Fear running totals vs Terror/Victory marks, need-per-turn to the deck-out turn, recent/game pace, reach turns, `BoardRoute` blockers per Terror Level (uses `rules::terror_victory_blockers`). | CORE MATH | 192 | 1 test (Russia T6 figures). |
| `common/src/companion_stats/curve.rs` | Rei curve: per-turn `Step` for Ravage/Build/Explore (known cards or stage average `stage_reach`), `Economy` history, `GrowthLine` projections along each presence track (`projected()`). | CORE MATH | 241 | 2 tests (track projection; step from history then stages). |
| `common/src/companion_stats/whatif.rs` | `Edit::{Destroy,Defend,Dahan,Track}`, `parse()`, `describe()`, `apply()` on a snapshot copy with Fear-card/Terror-level bookkeeping. | CORE MATH | 195 | 3 tests. |
| `ui/src/mesh.wgsl` | Per-vertex-color triangle shader with clip. | TOOLKIT | 41 | None |
| `app/src/icons.rs` | Registers 7 SVG masks from `frontend/src/assets/icons/` via `include_bytes!` (Blight, Fear, Explorer, Town, City, Dahan, Sacred Site). | LAYOUT/VIEW (asset glue) | 32 | None |
| `app/src/views/chart.rs` | `Plot` (unit→pixel map), `turn_axis`, `hline`, `polyline`, `glow_line`, `dashed`, `band`, `caption`. | LAYOUT/VIEW (reusable primitives) | 113 | None |
| `app/src/views/ledger.rs` | The Ledger page: token lanes per budget, inventory rows/chips, verdicts, what-if deltas, embeds Runway and Pace. | LAYOUT/VIEW | 714 | None |
| `app/src/views/runway.rs` | Blight-card-over-turns chart from `outlook`: weighted quantile band, per-path lines, loss shading, next-card legend, captions. `now_x()` helper. | LAYOUT/VIEW | 291 | None |
| `app/src/views/pace.rs` | Fear pace chart + "or clear the board" sparklines. | LAYOUT/VIEW | 197 | None |
| `app/src/views/curve.rs` | The Curve page: stacked stage bars, economy lines + projections, growth multiples with the 1.5–3.5 Energy-per-play band, acceleration rows. | LAYOUT/VIEW | 361 | None |
| `app/src/views/whatif.rs` | The W drawer: land rows with piece buttons, +Defend, +Dahan, undo; presence-track toggles. | LAYOUT/VIEW | 187 | None |
| `docs/superpowers/specs/2026-10-19-questions-from-play.md` | Brett's in-play questions by moment, the model each needs, and a layout conclusion (one screen per moment; budgets as a status strip). | DOCS | 137 | n/a |

## 2. Test and build status

Run in `tools/si-live` on 2026-10-19 (debug profile; si-common/si-steam at `opt-level = 2`).

`cargo test -q -p si-common`:
```
running 48 tests
................................................
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo test -q -p si-steam`:
```
running 8 tests   → ok. 8 passed  (0.17s)     [unit tests]
running 0 tests   → ok.                        [bin dump]
running 2 tests   → ok. 2 passed  (1.41s)     [tests/backtest.rs]
running 3 tests   → ok. 3 passed  (4.65s)     [tests/firewall.rs]
running 0 tests   → ok.                        [doc tests]
```

`cargo build -q -p si-app`: exit 0, no output (no warnings).

No compiler warnings were emitted by any of the three commands.

Capture-dependent tests: `steam/tests/backtest.rs` and `steam/tests/firewall.rs` both `eprintln!("skipping: capture or engine DLL not present")` and return early when `data/steam-capture/2026-10-18-test1/saves` or the engine DLL at `/mnt/c/Program Files (x86)/Steam/steamapps/common/Spirit Island/.../SpiritIslandEngine.dll` is absent. Re-running them with `--nocapture` printed no "skipping" line, so on this machine they ran against the capture (hence the 1.4 s and 4.6 s durations). On a machine without the capture they pass vacuously.

Of the 48 si-common tests, 10 are new today: `ledger` (2), `outlook` (2), `pace` (1), `curve` (2), `whatif` (3). Nothing in si-ui or si-app has tests.

## 3. Dependencies between the pieces

### View → core

| View file | si-common consumed | si-ui consumed |
|---|---|---|
| `views/mod.rs` | `companion_stats::compute`, `whatif::{apply, Edit}`, `CompanionStats`; si-steam `CompanionState`, `ReplayCommand` | `Ui`, `Rect`, `Key`, `Align`, theme; `line`, `circle`, `pulse`, `clicked` |
| `views/ledger.rs` | `ledger::{Budget,Spend}`, `whatif::Edit`, `CompanionStats` (`.fear`, `.curve.stage_reach`), `RoundSummary` | `icon`, `glow`, `line`, `circle`, `pulse`, `hovered`, `clicked`, `overlay`, `scroll_begin/end`, `text_wrap`, `measure_spans`, `span` |
| `views/runway.rs` | `outlook::Outlook` (via `stats.outlook`), `p_loss`, `RoundSummary.blight_remaining` | `quad`, `line`, `circle`, `pulse`, `layer`, `push_clip/pop_clip`, `hovered` |
| `views/pace.rs` | `fear_pace`, `pace::blockers`, `stats.outlook.paths` (for the board sparkline) | `line`, `circle`, `pulse`, `layer` |
| `views/curve.rs` | `curve::{Curve,CurveTurn,Step,Economy}` | `line`, `circle`, `layer`, `text_wrap` |
| `views/whatif.rs` | `whatif::{apply,Edit}`, `stats.invader_phase`, `stats.next_phase`, `stats.loss_clocks` | `icon`, `glow`, `hovered`, `clicked`, `overlay` |
| `views/chart.rs` | none | `line`, `quad`, `spans`, `measure`, `text` |
| `icons.rs` | none | `IconSource` |

`runway::now_x` and `chart::Plot` are shared by `pace.rs` and `curve.rs`; `ledger.rs` calls `runway::draw` and `pace::draw` directly.

### Core consumed by nothing in the views (dead if views are discarded, but still computed and serialized)

`CompanionStats` fields the new views never read: `needs` (needs.rs), `prompts`, `rules_in_play`, `changes` (via `CompanionState.changes`), `lands` (LandOutlook), `spirits` (spirit stats), `blight`/`fear` top-level stats (only `fear` via `fear_tokens`), `basis`. `review::review()` is computed in `steam/src/companion.rs:115` into `CompanionState.review` and printed by `dump review`; no view draws it. The grep in §6 confirms none of `needs`, `prompts`, `rules_in_play`, `review` appear in `app/src`. All of these predate today and were consumed by the deleted N1/N2 views (e.g. `rules.rs` drew `rules_in_play`, `phase.rs` drew `prompts`/`lands`). They remain reachable through the serialized `CompanionState` (`steam-game.json`, MCP) so they are not dead in si-common terms, only undisplayed.

New core consumed only by the views: `ledger`, `outlook` (also by `backtest::outlook_check`), `fear_pace`, `curve`, `whatif`. All four stats are also serialized into `CompanionStats`, so they reach `steam-game.json` and the MCP `get_game_state` whether or not a view exists.

### Toolkit features → consumers

| Feature | Added today? | Used by |
|---|---|---|
| Icons via resvg (`IconSource`, `add_icon`, `Ui::icon`, custom glyphs) | yes | `ledger.rs` (tokens), `whatif.rs` (piece buttons); registered in `main.rs` and `offscreen.rs` via `icons::all()` |
| Mesh pipeline (`mesh.wgsl`, `MeshVertex`) | yes | only through `quad` |
| `quad` | yes | `chart::band` (runway band, pace fan, curve growth band), `runway.rs` loss shading |
| `polygon`, `triangle`, `arrow` | yes | **nothing** — zero call sites in `app/` (and the deleted `island.rs` at HEAD predates them). Speculative API; the `mesh.wgsl` header comment "Land fills and arrow heads" describes a use that does not exist. |
| `glow` (soft SDF) | yes | `ledger.rs` (5), `whatif.rs` (1), `chart::glow_line` |
| `line` (SDF segment) | yes | every view; `chart::{polyline,dashed,hline}` |
| `circle` | yes | ribbon, runway, pace, curve |
| Layers (`LAYER_BASE/MAP/OVERLAY`, `Ui::layer`) | yes (3rd layer) | runway, pace, curve switch to MAP for chart content; `overlay(true)` in whatif drawer and ledger chip tooltip |
| Animation (`Ui::time`, `pulse`, `animating`) | yes | ribbon breath, ledger over-budget tokens and urgent verdict, runway/pace "now" dot; `main.rs` drives the 16 ms redraw |
| Offscreen PNG (`render_png` with icons + fixed time) | extended | `main.rs --screenshot` |
| Custom glyphs (`prepare_with_custom`) | yes | the icon path above |
| Existing: `scroll_begin/end`, `push_clip`, `text_wrap`, `spans` | no | ledger (scroll), runway (clip), several |

### Does si-common / si-steam compile and test cleanly with the app views removed?

Yes, by construction. The dependency direction is strictly `si-app → {si-ui, si-steam, si-common}` and `si-steam → si-common`; neither `common/` nor `steam/` imports anything from `app/` or `ui/` (grep for `si_app`/`si_ui` in `common/src` and `steam/src`: none). `common` and `steam` are workspace `default-members`; `ui` and `app` are not, so a plain `cargo test` never builds the views. The new core modules depend only on `crate::public`, `crate::rules`, and sibling `companion_stats` modules. Removing `app/src/views/*` and `app/src/icons.rs` would leave si-common's 48 tests and si-steam's 13 tests unaffected. The only coupling is the other way: `whatif::Edit::parse` exists for the `--whatif` CLI flag, and `backtest::outlook_check` exists for `dump outlook`; both live in core and compile without the app.

## 4. Carry-forward classification

### Core math (si-common)

| Piece | Verdict | Why |
|---|---|---|
| `rules.rs` `add_built` Explorer arm | **KEEP** | A one-line correctness fix with backtest evidence; independent of any layout. |
| `invaders.rs` `advance`/`time_passes`/`next_turn_start` refactor | **KEEP** | Pure factoring of existing behavior that `outlook` needs; one behavior change (Ravage card pushed to `discard` on advance) is correct for card-counting and is what the backtests exercised. |
| `companion_stats/mod.rs` `RoundSummary` card slots + `slot_cards` + Explore back-fill | **KEEP WITH CHANGES** | Needed by `curve` and useful history on its own. Add a unit test for `slot_cards` across the AdvanceInvaderCards boundary and for the back-fill loop (both are heuristics on snapshot timing). |
| `adversary.rs` `LossClock.unit` | **KEEP** | Data enrichment; harmless, serde-defaulted. |
| `ledger.rs` | **KEEP WITH CHANGES** | Sound Lumia model; the spec says it is "the engine of every column". Changes: the Blight `size` formula (`remaining + on_island`) is a drawing hint, not a game quantity — move to the view or document; `urgent` string is prose formatted in core (`"Invaders win at the end of this Fast Phase: remove …"`) that belongs in a view layer. |
| `outlook.rs` | **KEEP WITH CHANGES** | Exact enumeration is the spec's "If X is next" timeline; tests are sound. Changes: `PHASES = 4` is a constant the spec says is wrong for the layout ("Not 'nobody acts for four turns'") — parameterize the horizon; measure cost (each branch clones a `PublicSnapshot`; worst case ~kinds³ leaves) before it runs on every save decode. |
| `pace.rs` | **KEEP** | Fear arithmetic and board-route blockers are layout-independent; one test on real figures. |
| `curve.rs` | **KEEP WITH CHANGES** | Step/economy data is the Rei model. Changes: `projected()` infers tracks by stripping digits from node ids and picking the track with most `kind` nodes — an undocumented assumption about engine ids; `stage_reach` falls back to "darker's rule" `(i+2)·boards` with no pool — fine but label it in `basis`. |
| `whatif.rs` | **KEEP** | Clean, tested, read-only by design (`apply` returns a copy). The `Edit::parse` grammar is CLI-only but cheap. |
| `backtest.rs` `outlook_check` | **KEEP** | Verification tooling for `outlook`; the account's "40/40 next-card rows" claim rests on it. |
| `spirits.rs` visibility change | **KEEP** | Trivial. |

### Toolkit (si-ui)

| Piece | Verdict | Why |
|---|---|---|
| SDF lines/glows (`rect.wgsl`, `line`, `glow`, `circle`) | **KEEP** | Generic primitives; any chart or map needs them. |
| Mesh pipeline + `quad` | **KEEP** | Needed for gradient bands/shading in any chart. |
| `polygon`, `triangle`, `arrow` | **UNDECIDED** | Unused anywhere. Keep only if a board/map view is planned (the mesh.wgsl comment assumes one); otherwise they are dead API in a crate with no tests. |
| Icons via resvg + glyphon custom glyphs | **KEEP** | Game icons are needed by every design direction; pulls ~25 crates (`resvg`, `usvg`, `rustybuzz`, image decoders) into the Windows build, which is the cost. |
| 3 layers | **KEEP** | Small, general. |
| `Ui::time`/`pulse`/`animating` + 60 fps loop in `main.rs` | **KEEP WITH CHANGES** | Mechanism is fine; the specific "breathing" uses are layout. `pulse` sets `animating = true` for the whole frame, so any pulsing element forces 60 fps redraws of the full UI — acceptable now, worth a note. |
| `offscreen.rs` icons+time params | **KEEP** | Needed for reproducible screenshots. |
| `Key::Tab` | **KEEP** | Trivial. |

### App (si-app)

| Piece | Verdict | Why |
|---|---|---|
| `icons.rs` | **KEEP** | Asset glue (ids + `include_bytes!`), independent of layout. Add elements/presence as needed. |
| `views/chart.rs` | **KEEP** (move toward si-ui or a `charts` module) | Reusable chart primitives: `Plot`, `turn_axis`, `hline`, `polyline`, `glow_line`, `dashed`, `band`, `caption`. Nothing page-specific in it. Candidate for si-ui `widgets`-level reuse. |
| `views/runway.rs` `now_x()` and `quantile()` | **KEEP** (extract) | Small, reusable: the fractional-turn position from `step_index`, and a weighted quantile. The rest of the file is the Runway panel. |
| `views/runway.rs` (panel) | **DISCARD / UNDECIDED** | The spec's own conclusion rejects a four-turn no-action projection chart as a view; the loss-shading-by-`p_loss` and next-card legend are ideas to carry, not code. |
| `views/pace.rs` | **DISCARD** | Spec: "Pace is the last column of a growth option, not a chart of its own." The board-route sparkline computation (median blockers per turn from `outlook.paths`) is view-side math that should move to `pace.rs` core if kept. |
| `views/ledger.rs` | **DISCARD**, salvage components | Spec: "the budgets … are a narrow status strip, not the page." Reusable pieces inside it: `budget_tokens` token-row drawing (`Tok` states, brackets, edge line), `fear_tokens` pips, `deck_tokens` card backs, `inventory_row`/`spend_chip` (a spend with step, land chip, amount, solve pill), `units()` pluralizer. These are components, not layout. |
| `views/curve.rs` | **DISCARD**, salvage ideas | Page layout. The step chart's stacked stage bars and the growth-multiples computation (`worth`, `rate`) are reusable; the 2.5-Energy-per-play exchange rate is contested and belongs in a documented `basis`. |
| `views/whatif.rs` | **KEEP WITH CHANGES** | The drawer frame is layout, but `land_row` (land chip + piece buttons that destroy + Defend + Dahan + undo), `button`, `toggle`, and the tier ranking of lands (Ravages now / Builds now / Ravages next) are reusable interaction components for any "my plan vs this Ravage" screen, which the spec names as the second screen. |
| `views/mod.rs` | **DISCARD** page/ribbon, **KEEP** scaffolding | The `Both` (base vs tried) pattern, the what-if cache keyed on `(seq, edits)`, the moment-reset of edits, `Toggle`, and `step_index` are layout-independent and worth keeping; `Page::{Ledger,Curve}`, the ribbon, and the status line are the rejected layout. |
| `views/fmt.rs` removals | **KEEP** | Removes helpers only the deleted views used. |
| Deleted N1/N2 views | **KEEP the deletion** | Brett rejected them explicitly; the commit history keeps them. |
| `main.rs` CLI (`--page`, `--whatif`, `--drawer`) | **KEEP WITH CHANGES** | `--whatif` and offscreen icon/time plumbing are useful for reproducible screenshots of any design; `--page ledger|curve` and `--drawer` name the rejected layout and should be renamed/removed with it. |

### Build/tooling/docs

| Piece | Verdict | Why |
|---|---|---|
| `build-windows.sh` rm-before-copy | **KEEP** | Correct and explained (see §6). |
| `dump outlook` | **KEEP** | Verification command for a kept core module. |
| `Cargo.lock` | **KEEP** | Follows `ui/Cargo.toml`. |
| `docs/superpowers/specs/2026-10-19-questions-from-play.md` | **KEEP** | It is the direction-review input; its final section already records the carry-forward conclusion. |

## 5. Risks of committing as-is vs splitting

Committing everything as one commit would mix a one-line rules fix, a toolkit expansion, five new core models with tests, a verification tool, and a page layout Brett has said is the wrong direction. Later `git revert` or `git log -S` on the layout would drag the core math with it. The index is also inconsistent right now: only the nine deletions are staged. A bare `git commit` would record a tree whose `views/mod.rs` still declares `mod clocks;` etc. against deleted files — a non-building commit.

Nothing found that must not be committed: no probe files (zz_probe.rs confirmed absent), no `dbg!`, no `TODO`/`FIXME`, no scratchpad or `/tmp` paths, no screenshot paths in code (only the `--screenshot out.png` usage line in the main.rs doc comment). The Windows-specific items are all pre-existing and documented (`/mnt/c` DLL default in `engine_defs.rs`, `%LOCALAPPDATA%` in `data.rs`, `tasklist.exe` polling, the gnullvm target in the build script). The lockfile gains a duplicate `png` (0.17.16 for resvg's tiny-skia alongside 0.18.1 used by si-ui) — a size cost, not a correctness one.

Recommended split, in order (messages in the repository's style; no attribution lines):

1. `rules: Explorers built in the lookahead count as Explorers`
   `common/src/rules.rs`. The real bug fix, alone, so it is bisectable and revertable on its own.

2. `companion: Invader Phase advance, Time Passes, cards per turn in the history`
   `common/src/companion_stats/invaders.rs`, `common/src/companion_stats/spirits.rs`, `common/src/companion_stats/adversary.rs`, and the `RoundSummary`/`slot_cards`/`round_summaries` parts of `common/src/companion_stats/mod.rs` (the `pub mod` lines and `CompanionStats` fields for the new modules go in commit 3 — this needs a partial stage with `git add -p`).

3. `companion: ledgers, card-order outlook, Fear pace, the curve, and what-if edits`
   `common/src/companion_stats/{ledger,outlook,pace,curve,whatif}.rs`, the remaining `mod.rs` hunks (module registration, `CompanionStats` fields, `compute()` wiring), `common/src/companion_stats/backtest.rs` (`outlook_check`), `steam/src/bin/dump.rs` (`outlook` subcommand). Tests: the 10 new si-common tests belong here.

4. `si-ui: SDF lines and glows, meshes, icons via resvg, layers, animation clock`
   `ui/Cargo.toml`, `ui/src/{lib,painter,render,offscreen}.rs`, `ui/src/{rect,mesh}.wgsl`, `Cargo.lock`. Optionally drop `polygon`/`triangle`/`arrow` from `painter.rs` first (unused) or keep them with a note; the direction review decides.

5. `native app: drop the N1/N2 panel views`
   The nine staged deletions plus the matching `fmt.rs` removals. To keep this commit building on its own it must also include the `views/mod.rs` and `main.rs` changes, which drags the Ledger/Curve layout in — so either (a) fold 5 and 6 together, or (b) accept that this commit is the layout commit and name it so. Recommendation: fold into 6.

6. `native app: Ledger and Curve pages, Runway, Pace, What-If drawer (layout under review)`
   `app/src/main.rs`, `app/src/icons.rs`, `app/src/views/{mod,fmt,chart,ledger,runway,pace,curve,whatif}.rs`, the nine deletions, `app/build-windows.sh`. One commit so the whole rejected layout can be reverted or cherry-picked from in one step while commits 1–4 stay. If the review wants `chart.rs`, `icons.rs`, and the build-script fix kept regardless, split them out ahead as `native app: chart primitives, game icons, fresh exe on build`.

7. `docs: the questions from play and the screen each one needs`
   `docs/superpowers/specs/2026-10-19-questions-from-play.md`.

Before committing, per the global rules: `git fetch origin` and confirm `companion-v2`'s base is current (the branch has no upstream yet), and stage only the named files — never `git add -A`.

## 6. Known defects and loose ends

Unwired or undisplayed computations (all computed on every `compute()` call and serialized, none drawn by the new views):
- `needs.rs` (`CompanionStats.needs`) — the spec's "the draw" column depends on it; no view reads it.
- `prompts()` (`CompanionStats.prompts`, `mod.rs:458`) — was drawn by the deleted `phase.rs`.
- `rules_in_play` — was drawn by the deleted `rules.rs` view.
- `review::review()` → `CompanionState.review` (`steam/src/companion.rs:115`) — only `dump review` prints it.
- `changes::recent()` → `CompanionState.changes` — was the play-by-play in deleted `plays.rs`.
- `CompanionStats.lands` (LandOutlook), `.spirits`, `.next_phase` (only `whatif.rs` reads `next_phase`), `.loss_clocks` (read by `ledger()` in core and `whatif.rs` tiering).

Unlabeled assumptions in the new core:
- `outlook::PHASES = 4` horizon, and cascades counted at their minimum (documented in the module header but not in `CompanionStats.basis`).
- `curve::projected()` reconstructs presence tracks from node-id text (`"<spirit><Track><n>"`) and picks the track holding the most nodes of the requested kind. Works on the tested spirits; not a documented engine contract.
- `curve::stage_reach` "darker's rule" fallback `(stage+1)·boards` when no pool is known.
- `curve.rs` view: Energy-per-card-play exchange rate 2.5 with a 1.5–3.5 band, labeled in the view only ("contested").
- `ledger.rs`: Blight budget `size` includes `on_island`; Fear budget `shortfall` uses counterattack best case (`certain: false` on items, but the lane verdict says "best case" only in text).
- `pace.rs`: the deck-out turn adds `u32::from(explored)` based on step names `AdvanceInvaderCards | SlowPower | TimePasses`; `curve.rs` and `ledger.rs::deck_tokens` duplicate the same three-step test. Should be one helper next to `invaders::phase_position`.
- `mod.rs` Explore back-fill: when a turn was only seen before its Explore, its explored card is inferred as the next turn's Build card minus the previous Build cards (`out[i].built − out[i-1].built`). Breaks for Build slots holding extra cards (Russia's Fear-deck cards, noted in the `RoundSummary` doc comment) — no test.

Leftover suppressors (harmless, but they mark half-finished reasoning): `let _ = stats;` (`views/ledger.rs:364`), `let _ = &f.cards_left_by_level;` (`ledger.rs:532`), `let _ = x;` (`views/runway.rs:760`), `let _ = s;` (`views/curve.rs:919`), `let _ = t;` (`curve.rs:946`), `let _ = now_x(s);` (`curve.rs:1060`).

Dead toolkit API: `Ui::polygon`, `Ui::triangle`, `Ui::arrow` have no callers (see §3).

Performance not measured: `outlook()` clones a `PublicSnapshot` per branch per phase (up to kinds³ leaves) and runs inside `compute()` on every decoded save (several per second live, per the workspace `Cargo.toml` comment); the what-if path runs `compute()` a second time on every edit change. The status line no longer shows `frame_ms`, so the cost is invisible in the app.

`build-windows.sh` rm-before-copy: `cp` onto an existing exe overwrites the file in place; Windows maps a running exe's image from that file and keeps serving the old image to new launches of the same path (and may lock the file). Deleting first forces a fresh inode so the next launch gets the new build. Correct; the comment in the script says the same.

CLI flags: `--page ledger|curve` (default ledger), `--whatif "destroy:NE7:Town;defend:NE7:8;dahan:A3;track:Spirit:Energy"` (parsed once a snapshot arrives via `Edit::parse`, land by id or short name), `--drawer` (opens the W drawer). `--tab` was removed. These exist so `--screenshot` can reproduce a what-if state headlessly. `--capture` and `--at` are unchanged.

Writes to game state — verified none from the app/views:
- `app/src` has no `fs::write`, `File::create`, `OpenOptions`, or `remove_file`. `whatif::apply` returns a new snapshot; nothing is sent to the source thread (`ReplayCommand` is seek/play only).
- `si-ui` writes only the PNG at the `--screenshot` path (`offscreen.rs:83`).
- `si-steam` (pre-existing, unchanged today) writes under `data_dir()` = `$SI_DATA_DIR` or `%LOCALAPPDATA%\si-companion` (`app/src/data.rs:34`, comment: "never the repo's tabletop data"): `steam-capture/live-<game>/saves/*.gz` + `events.txt`/`log-stream.txt` (recorder, off with `SI_STEAM_RECORD=0`), `steam-history/<game>.jsonl`, and `steam-game.json` (atomic tmp+rename). It reads the Steam save folder (`%USERPROFILE%\AppData\LocalLow\Handelabra Games Inc_\Spirit Island`) and never writes into it.
- `data/current-game.json` appears only in a doc comment (`common/src/schema.rs:1`); no code path in si-live's Rust touches it.
- `dump.rs` writes `<out>/<name>.json` only for its explicit export subcommand (pre-existing).

Index inconsistency: the nine view deletions are staged; everything else is unstaged (see §5).
