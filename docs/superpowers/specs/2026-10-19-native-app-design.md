# Native app (companion + tabletop) — design

Date: 2026-10-19
Status: rev 3. N0 done: the spike window opened on Brett's desktop (Vulkan,
AMD Radeon RX 7900 XTX). N1 built: `tools/si-live/ui` (`si-ui`) and
`tools/si-live/app` (`si-companion.exe`), replaying captures and following
the live Steam game in-process. N2 built: the center view follows the phase
(Spirit/Fast, Event, Fear, Invaders, Slow) and a Rules view explains what is
in play.
Branch: `companion-v2` (worktree `~/repos/spirit-island-steam-companion`).

## 1. Goal

Replace both web frontends (the Steam companion dashboard and the tabletop
tracker) with one native Windows app, written in Rust on winit + wgpu +
glyphon, that reads well at a glance beside the game: **one screen, no
scrolling** on a 1600×900 or larger monitor, showing only what the current
moment of the game needs.

Brett's decisions (2026-10-19): custom winit + wgpu + glyphon UI (not GPUI);
a native Windows .exe; reset both the companion and the tabletop app.
Inspiration: the Sly debugger's look (dark, dense but ordered panels, panel
headers with counts, monospace values, one accent color, a status bar). Its
debugger is built on GPUI; its launcher is the winit + wgpu + glyphon + gltf
stack we use.

### Why the current UI fails (playtest 2026-10-19, U1–U3)

Everything is on screen at once, so nothing says what the companion is trying
to help with; the island needs scrolling; visuals don't encode meaning.
The fix is information design first: each game moment gets a screen that
answers its two or three questions, and everything else waits.

### Non-goals

- Re-implementing game rules in the UI. All logic stays in `si-common`
  (stats, rules), `si-steam` (decode, firewall) and `si-corpus` (data).
- Web, mobile or macOS builds (winit/wgpu allow them later).
- 3D on day one. A gltf island is an optional later milestone (N6).

## 2. Architecture

```
si-app (bin, Windows .exe)
├── shell: window, event loop, modes (Live · Replay · Tabletop · Review)
├── views: one module per screen, each a pure function view-model → UI calls
├── si-ui (crate): the toolkit (§4)
├── data
│   ├── si-steam: save watcher, firewall, companion (in-process; no HTTP)
│   ├── si-common: stats, rules, tabletop schema + state
│   └── si-corpus: card, spirit, adversary, board facts (§6 of the corpus spec)
└── platform: Windows paths (Steam saves, engine DLL), process detection,
    settings in %APPDATA%\si-companion
```

- **No server in between.** The app links the Rust core and runs the save
  watcher on a background thread; the UI reads the latest `CompanionState`
  each frame (the existing `tokio::watch` channel, or a plain `Arc<Mutex>`).
- **The MCP server keeps working.** It reads `data/current-game.json` and
  later `data/steam-game.json`; the app keeps writing both in the same schema.
- **Build:** cross-compiled from WSL (`x86_64-pc-windows-gnullvm`, llvm-mingw
  in `~/.local/toolchains`; ship `libunwind.dll` beside the .exe or link it
  statically), or `cargo build` on Windows if Rust is installed there later.
  Launch from WSL through Windows interop.

## 3. Screens

One window, four regions, fixed proportions (no page scroll; long lists
scroll inside their panel only):

```
┌ top bar: mode · phase tabs (auto-follow the game, click to pin) · source ┐
├───────────────┬───────────────────────────────────────┬──────────────────┤
│ CLOCKS        │ PHASE VIEW (changes with the phase)   │ ISLAND (compact, │
│ Blight buffer │                                       │ fits; badges for │
│  + loss %     │  the 2–3 questions of this moment     │ Defend, powers   │
│ Presence      │  with the facts that answer them      │ used, skips)     │
│ Fear / TL     │                                       │                  │
│ Invader deck  │                                       │ PIPELINE         │
│ Adversary     │                                       │ Ravage·Build·Exp │
│  loss clock   │                                       │ cards + lands    │
│ SPIRIT        │                                       │                  │
│ Energy, plays │                                       │                  │
│ elements,     │                                       │                  │
│ innate levels │                                       │                  │
├───────────────┴───────────────────────────────────────┴──────────────────┤
│ PLAY-BY-PLAY (collapsible; undo-aware: undone actions disappear)         │
├──────────────────────────────────────────────────────────────────────────┤
│ status bar: game · adversary L · turn/step · engine build · capture · fps │
└──────────────────────────────────────────────────────────────────────────┘
```

Phase views (each answers only its questions; from the playtest):

| Phase | Questions | Content |
|---|---|---|
| Growth | Which growth fits this turn? | The options side by side: Energy and Card Plays after, cards in hand (and back after a Reclaim), elements and innate levels each reaches, needs each can meet this turn (U11) |
| Gain a Power (draft) | Which card, and what to Forget? | The 4 drawn cards (from the log the moment they're revealed, U12): cost, speed, range/origin, elements, threshold; pairings with hand vs discard; what it does on the board this turn; Forget candidates and what each loses (U13) |
| Spirit / Fast | What can I still change before the Ravage? | Ravage lands with their ledgers, the Blight line, which lands are one action from safe, what in hand reaches them (Fast only), chance of losing (U16) |
| Event | What does it do, and what's my choice? | The Event (from the log at once, U7), its sections applied to the board, each choice's effect on this Ravage (U9), token placement lens (U15) |
| Fear | What will the banked cards do? | Banked count, Terror Level, effects as they reveal |
| Ravage / Build / Explore | What happened vs the forecast? | Live results, cascades and their targets, pacing changes (U14) |
| Slow | What sets up next turn? | Next turn's pipeline (with this turn's one-turn effects removed, R7), Build and Ravage lands two turns out |
| Review (after the game) | Where did it turn? | The retro timeline and per-turn breakout (companion spec §7.10) |
| Tabletop | (the tracker) | Setup wizard, board editing, decks, spirit panel, the same clocks; writes `current-game.json` in its existing schema |

Visual language: dark background, panel cards with a header (title + count),
one accent (purple) for "yours/now", amber for warnings (Blight line, loss
risk), red only for loss; numbers in a monospace face; terrain colors matched
to Steam (U5: Mountain is grey).

## 4. Toolkit (`si-ui`)

Immediate-mode API over a retained render list:

- **Renderer:** one wgpu pipeline for SDF rounded rectangles, borders and
  lines (instanced quads), one for images (icons, board art), glyphon for text
  (a cache of shaped `Buffer`s keyed by content). Scissor rects for clipping.
- **Layout:** fixed screen regions cut from the window rectangle
  (`Rect::cut_*`, `columns`, `rows`); widgets measure text through glyphon.
  N1 needed nothing more. `taffy` stays an option if the tabletop forms
  need flexbox.
- **Widgets:** panel (header, count, collapse), tabs, table, chip/badge,
  button, toggle, list with inner scroll, tooltip, sparkline/bars, land map
  (polygons + piece glyphs), card tile; for the tabletop mode also text input,
  dropdown, number stepper.
- **Input:** winit events → hit-testing on last frame's layout; keyboard focus;
  DPI-aware (scale factor).
- **Fonts and icons:** N1 uses the Windows system fonts (Segoe UI; Cascadia
  Mono or Consolas for numbers). Bundled OFL fonts and icons (a font or
  glyphon custom glyphs) come with N6.
- **Theme:** tokens (colors, spacing, radii, type scale) in one module.

## 5. Tabletop migration (safety first)

Brett's live tabletop game is in `data/current-game.json`. The native app:

- reads and writes the **same schema** (`si_common::schema::GameState`),
  with the same migrations (`migrate_invader_deck`);
- never writes during development except to a separate data dir
  (`SI_DATA_DIR`), until Brett switches over;
- keeps a timestamped backup on first open of an existing file.

The web tabletop app stays runnable until the native mode reaches parity.
Brett's uncommitted "Aegis" design work in the main checkout is not touched;
where it sets visual direction for the tabletop, the native theme follows it.

## 6. Milestones

| # | Milestone | Done when |
|---|---|---|
| N0 | Toolchain spike | **Done:** .exe builds from WSL and opens on Brett's desktop (Vulkan, RX 7900 XTX) |
| N1 | Toolkit core + shell | **First cut built:** panels, tabs, chips, pips, inner scroll, clipping, status bar; the §3 layout on live and replayed `CompanionState` (in-process source thread, no server); `--screenshot` renders any capture moment (or the live game) offscreen to PNG on the real GPU. Next: Brett's review of the layout; then N2 |
| N2 | Companion phase views | **Built:** Spirit/Fast (lands taking Blight with the Defend or Damage that saves each and what in hand can, Slow cards marked too late in the Fast Phase; lands one Damage from Blight; this turn's Builds and Explores in brief), Event (the revealed Event from the log at once, each section marked by whether it applies now), Fear (banked count and when it resolves, the card resolving, resolved cards with the text that applied), Invaders (the full forecast), Slow (next turn played forward, in the Invaders layout), Rules (loss conditions with when they're checked, each adversary level, scenario and spirit rule in plain words; R or the Rules button). Callouts for next turn's Blight and loss clocks near their limit on every view. `--tab` renders a view for screenshots |
| N3 | Growth and draft views | U11, U12, U13 on corpus card data |
| N4 | Review | Retro timeline and breakout (§7.10 R1–R4) |
| N5 | Tabletop mode | Feature parity with the web tracker, same file, backups |
| N6 | Polish | Animations, icons, optional gltf island, packaging (single folder, start-with-game option) |

## 6.1 Build and run (N1)

```
tools/si-live/app/build-windows.sh            # → tools/si-live/target/si-companion/
si-companion.exe                              # live: follows the Steam game
si-companion.exe --capture <dir> [--at T6/FastPower]     # replay: ← → step (Shift ×10), Space play
si-companion.exe --capture <dir> --at T6/FastPower --screenshot out.png [--size 1920x1080]
```

From WSL, pass Windows paths (`wslpath -w`). Live mode keeps its state and
history in `%LOCALAPPDATA%\si-companion` (`SI_DATA_DIR` overrides; never the
repo's tabletop data). `ui` and `app` are workspace members outside
`default-members`, so the server crates' `cargo build` / `cargo test` are
unchanged.

## 7. Risks

- **Toolkit scope.** Text input, IME, focus and accessibility take real work;
  N5 carries most of it. Mitigation: companion first (read-only views).
- **Cross-compile quirks** (gnullvm runtime DLL, GPU backends). Mitigation:
  DX12 first, Vulkan fallback; keep the spike as a smoke test.
- **Two frontends during migration.** Mitigation: both read the same core; the
  web companion is frozen (no new features) once N2 lands.
