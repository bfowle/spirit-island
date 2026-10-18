# Steam Companion — design

Date: 2026-10-18
Status: draft (rev 3, after two spec reviews), awaiting Brett's review
Branch: `steam-companion` (worktree `~/repos/spirit-island-steam-companion`)

## 1. Goal

Turn si-live into a **Spirit Island companion** for the Steam (Handelabra)
version of the game. While Brett plays on Steam, a second-monitor dashboard
shows live state, percentages, and breakdowns so he can learn the math of the
game: invader odds, ravage outcomes, fear and blight timelines, and power-draw
odds.

**Not a cheat.** The companion shows only what a player at a physical table
could know. Section 5 makes this a hard, tested rule that covers every path
out of the backend.

### Non-goals (this spec)

- History analytics over past games (45 `EndGame--*.dat` files exist). Next
  spec; the decoder built here makes it cheap.
- Predict-then-reveal teaching mode. Rejected for v1 in favor of an always-on
  dashboard.
- The memory-reading source (`si-probe`). Phase B; outlined in Section 12,
  specified separately.
- Choosing the final host layout. The `mock-backend-api` branch has an
  unmerged layout redesign; the companion ships as its own frontend root
  (Section 8).
- Multiplayer (online) games. Detected and flagged, not supported.
- MCP changes. Later, `get_game_state` gains a `source` parameter and reads
  `data/steam-game.json`, so `si-at-the-table` can see live Steam state.

## 2. Decisions

| Topic | Decision |
|---|---|
| Relation to tabletop mode | Coexist. Steam mode is read-only, has its own state file, and its own frontend root. Tabletop state is never read or written by it. |
| Teaching posture | Always-on dashboard. Every number visible, updated live. |
| v1 stat families | All four: invader forecast, fear and victory race, blight and loss risk, spirit economy and powers. |
| Display | Second monitor, Windows browser, dense layout. |
| Ingestion | Staged. Phase A: file-based (log tail + save decoder). Phase B: `si-probe`, a read-only process reader. |
| Language | Rust for every component. No C#. |
| Reverse engineering | REA (`~/repos/rea`, MCP `rea`) for the engine model map, update diffs, and dev-time runtime observation. |
| UI base | Self-contained companion components behind a fixed API. Host layout decided later. |

## 3. Ground truth: what the game exposes

Observed on 2026-10-18 against Spirit Island **v3.4.1 build 840** (Steam,
Windows 11, Unity 2022.3.62f3, Mono runtime).

Game data folder (Windows):
`C:\Users\brett\AppData\LocalLow\Handelabra Games Inc_\Spirit Island\`
(WSL: `/mnt/c/Users/brett/AppData/LocalLow/Handelabra Games Inc_/Spirit Island/`)

| File | Content | Written |
|---|---|---|
| `log.txt` | Plain-text action log, one line per engine action: `DidPerformAction: [<ActionType>: …]`, `ChangeStateAction From=… To=…`, `***** SAVING *****`, `***** CLEARING UNDO FILES *****`. Rewritten from `GameView Start` at each new game. **Contains hidden information** (e.g., the name of each Fear card when earned). | Live, per action |
| `save-<tag>-<NNN>-<NNNN>-<State>.dat` | Full engine model, MS-NRBF (.NET BinaryFormatter) of `Handelabra.SpiritIsland.Engine.Model.Game`. About 6 rolling files; `NNNN` is a sequence number. | At each state change (`SAVING`) |
| `save-Undo-NNN.dat` | Same format; undo checkpoints inside a state. | Per undoable decision |
| `EndGame--<id>.dat` | Final model of a finished game (45 files). | At game end |
| `EndGame--<id>.txt` | Full action log of a finished game (4 files, April 2026 games only). | At game end |
| `Player.log` | Unity player log (version, startup). | Per launch |

Process facts: no debug or profiler port listens; one HTTPS connection to the
Parse backend; Steam and Photon UDP. Game logic lives in
`SpiritIslandEngine.dll` (5.9 MB, pure IL, .NET Framework target). REA reports
5,432 types and 21,420 fields; `Game` has 83 fields and 211 methods. Save-file
field names equal the engine's backing-field names
(`<TurnIndex>k__BackingField`), so one REA-derived map serves both sources.

## 4. Architecture

```
SpiritIsland.exe (Windows, Mono)
 ├─ log.txt ──── poll ~250 ms ─────► LogParser ──► RawLogEvent ──────────┐
 └─ save-*.dat ─ on SAVING / mtime ► NRBF decoder ─► EngineSnapshot ─────┤   (hidden info inside)
                                                                         │
                       ┌─────────── FIREWALL: projection.rs ◄────────────┘
                       │   EngineSnapshot → PublicSnapshot
                       │   RawLogEvent    → PublicEvent
                       ▼
          si-common: PublicSnapshot + PublicEvent  (no field can hold hidden data)
                       │
          si-common stats: fn(&PublicSnapshot, &[RoundSummary]) → CompanionStats
                       │
          si-steam: CompanionState { public, events, stats, source, game }
                       ▼
          backend: GET /api/companion/state · /stream (SSE) · /health
                       ▼
          frontend: separate root, #/companion (Section 8)
```

### Crates and units

Dependency direction: `si-common` ← `si-steam` ← `backend`. Folders:
`si-common` is `tools/si-live/common/`; the new `si-steam` crate is
`tools/si-live/steam/`. File paths below are relative to those folders. The
MCP server is not changed in v1 (see Non-goals).

| Unit | Crate / file | Job |
|---|---|---|
| `public` types | `si-common/src/public.rs` | `PublicSnapshot`, `PublicEvent`, `RoundSummary`. Pure data, no hidden fields by construction. |
| `rules` | `si-common/src/rules.rs` | Game rules used by stats (Ravage, Build, cascade, victory, card pools per stage), each with a Wiki citation. |
| stats | `si-common/src/companion_stats/` | One module per stat family. Input: `&PublicSnapshot` + history. Output: `CompanionStats`. |
| `nrbf` | `si-steam/src/nrbf.rs` | Generic MS-NRBF decoder → object tree with class and member names. Knows nothing about Spirit Island. |
| `engine_map` | `si-steam/engine-model.json` + `src/engine_map.rs` | REA-generated class/field map; object tree → typed `EngineSnapshot`. |
| `log_parse` | `si-steam/src/log_parse.rs` | Log lines → `RawLogEvent` (state change, fear generated, pieces moved, card moved/flipped, game over, `Unparsed`). |
| `projection` | `si-steam/src/projection.rs` | **The firewall.** The only code that reads hidden fields or raw log events. Produces public types only. |
| `watcher` | `si-steam/src/watcher.rs` | Polls file size and mtime (WSL gets no inotify events for Windows writes on `/mnt/c`). Picks the truth save, detects new games, log growth, truncation. |
| `companion` | `si-steam/src/companion.rs` | Builds `CompanionState`; keeps per-round history. |
| routes | `backend/src/` | Spawns the watcher task; serves state, SSE, health. |

`GameSource` (in `si-steam`) is a trait that yields `EngineSnapshot`s and
`RawLogEvent`s. Phase A implements it with files; Phase B with `si-probe`.
Both feed the same firewall.

### `PublicSnapshot` contents

The stats need these public inputs; `GameState` alone lacks most of them.

- Round, phase/state, adversary + level, scenario, difficulty, board IDs.
- Lands: terrain, coastal flag, adjacency, pieces (explorers, towns, cities,
  dahan, blight, presence by spirit, tokens).
- Spirits: energy, card plays, presence tracks (revealed slots), hand,
  discard, played this turn (with speed), forgotten, elements by source.
- Invader deck: for each face-down card, its **stage** (printed on the card
  back, so public); flipped cards per slot (`explore`, `build`, `ravage`) with
  terrain; discard pile.
- Fear: generated, required per card, earned count (no names), cards left per
  terror level, terror level, discard (names, resolved cards only).
- Events: deck count, revealed/resolved names.
- Powers: Minor and Major deck counts, discard piles, and the allowed pools
  (`AllowedMinorPowerCardIdentifiers`, `AllowedMajorPowerCardIdentifiers`).
- Blight: count on the card, flipped flag, healthy-side text; blighted side
  only after the flip.

### Configuration

- `SI_STEAM_DIR`: game data folder. Default: resolve `%USERPROFILE%` through
  `cmd.exe` + `wslpath`, then append the LocalLow path.
- `SI_STEAM_POLL_MS`: poll interval, default 250.
- Game detection: `tasklist.exe` every 5 s → `not-running` / `menu` /
  `in-game`. `in-game` also requires log activity since `GameView Start`.

### Which save is the truth

The truth is the most recently written save file, main or undo. Ties go to
the higher sequence number. Undo saves are decoded like any other. An undo
shows up as a save whose state is older than the previous truth; the
companion replaces its state and drops newer `PublicEvent`s. The first
milestone confirms this behavior against live play before the rest builds on
it.

### Persistence

- Steam mode never reads or writes `data/current-game.json` and never calls
  `PUT /api/state`.
- `data/steam-game.json`: latest `CompanionState`, for backend restarts (and
  later for the MCP server). Gitignored.
- `data/steam-history/<game-id>.jsonl`: one `PublicSnapshot` per save.
  Gitignored. `game-id` = first 12 hex chars of SHA-256 of the engine seed;
  the seed itself never leaves the decoder.

## 5. The information firewall

### Rule

> The companion shows only information that a player at a physical table,
> playing the same game, could know at that moment. Odds come from public
> rules plus cards already seen — never from the save's hidden contents.
> This applies to every byte that leaves the backend: state, SSE events,
> health output, warnings, logs written to disk.

### Why "derive from rules", not "count the save's deck"

The base invader deck uses 3 of 4 Stage I cards; one is removed at random and
never seen. If the companion computed Explore odds from the save's actual
remaining deck, it would reveal which card was removed. The public odds treat
every unseen Stage I card as equally likely to be next. Every deck works this
way: the unseen set is *(card pool from rules and setup) − (cards seen)*,
never *(the save's deck list)*. Card **backs** (an invader card's stage) are
public and are read from the save per card.

### Visibility table

Rows marked † are verified against the tabletop rules (`si-wiki-fetch` /
`si-rules-check`) before their code lands.

| Engine data | Public form |
|---|---|
| `RNG`, `InitialRNG`, `RandomSeed` | None. Dropped in the decoder. |
| Invader deck | Stage of each face-down card (back), count remaining. Terrain odds from rules. |
| `ExploreSlot`, `BuildSlot`, `RavageSlot`, `DiscardSlot` | Card shown only once flipped face-up; empty slots shown empty. |
| `FearDeck` | Count per terror level (divider positions are public). |
| `FearEarned` | Count only, until each card resolves †. |
| `FearDiscard`, `FearGenerated`, `FearRequiredForFearCard`, `TerrorLevelCard` | Shown. |
| `EventDeck` | Count only. An event shown once revealed. |
| `MinorPowerDeck`, `MajorPowerDeck` | Count only. Draw odds from the allowed pool − cards outside the deck. |
| Discards, hands, played, forgotten cards | Shown (co-op open information). |
| `BlightCard` | Count on the card, healthy side; blighted side only after the flip †. |
| `Island`, `_pieces`, `_spirits`, presence, tracks, energy, elements | Shown. |
| `Adversary`, `Scenario`, `DifficultyRating`, `TurnIndex`, `State` | Shown. |
| `Journal` | Used for history only through `projection.rs`. |

### Log events through the firewall

`log.txt` names hidden cards (example:
`MoveCardAction: CardBeingMoved=WaryOfTheInterior, To=Fear Earned, From=Fear Deck`).
So `RawLogEvent` never leaves `si-steam`. `projection.rs` maps each one to a
`PublicEvent` or drops it:

- Each engine card zone is classified in one table in `projection.rs`:

  | Zone class | Zones (engine names, from the log and `Game` fields) |
  |---|---|
  | Hidden | Fear Deck, Fear Earned, Event Deck, Invader Deck, Minor/Major Power Deck, Blight Deck |
  | Public | Fear Discard, Event Discard, Play Area, Invader Discard, Explore/Build/Ravage Space, Power Discards, spirit hands, played, forgotten |
  | Mixed — decided per card type | Out of Game (a Terror Level divider is public; a card removed at setup is hidden) |
  | Unknown | Any zone not in this table — **treated as hidden** |

- A move whose destination is hidden → `PublicEvent` without the card name
  (e.g., `FearCardEarned { terror_level }`).
- A move into a public zone → name allowed.
- `Unparsed` lines → counted by action type in `/health`. Raw text is never
  served, stored, or put in `warnings`.

### Enforcement

1. `projection.rs` is the only module that reads hidden fields or
   `RawLogEvent`. Public types live in `si-common` and have no field that can
   hold a deck order, a seed, or a hidden card name.
2. **Invariance test (snapshots).** For each fixture, permute hidden card
   identities only within groups that look the same to a player: invader cards
   within the same stage, Fear cards within the same terror level, Event and
   power cards within their deck, the blight card with others of the same
   healthy side. The resulting `PublicSnapshot` and `CompanionStats` must be
   byte-identical to the unpermuted ones.
3. **Invariance test (log).** For each fixture log, rename every card whose
   destination is a hidden, mixed, or unknown zone to a different card of the
   same group. The resulting `PublicEvent` stream must be identical. A
   separate test asserts that every zone seen in the fixtures is in the zone
   table, so a new zone fails loudly instead of being guessed.
4. **Field-name test.** Serialized `CompanionState`, SSE payloads, and
   `/health` never contain `RNG`, `Seed`, or deck-order arrays.

## 6. CompanionState and API

```rust
// si-steam
pub struct CompanionState {
    pub public: PublicSnapshot,      // si-common
    pub events: Vec<PublicEvent>,    // si-common; events after the last save
    pub stats: CompanionStats,       // si-common
    pub history: Vec<RoundSummary>,  // si-common; per-round fear, blight, pieces, energy
    pub source: SourceStatus,        // files | probe; process state; last save/log time;
                                     // stale flag; engine build + map match
    pub game: GameState,             // derived from `public`, for winprob.ts and MCP compatibility
    pub warnings: Vec<String>,       // unmapped pieces/cards, approximations (no raw log text)
}
```

`game` is a convenience view built from `public`. It is not edited and is
never sent to `PUT /api/state`.

Endpoints (new, read-only):

- `GET /api/companion/state` → `CompanionState`.
- `GET /api/companion/stream` → SSE; one `state` event per change, plus
  `status` events for process and staleness changes.
- `GET /api/companion/health` → source status, decoder diagnostics, unparsed
  line counts by action type.

## 7. Stats, v1

Signature: `fn(&PublicSnapshot, &[RoundSummary]) -> FamilyStats`. Each stat
carries a `basis` string (what it assumes) and an `approx` flag. Rules live in
`si-common/src/rules.rs`, each verified with `si-rules-check` before coding
(memory rule: never hallucinate mechanics).

### 7.1 Invader forecast

- **Ravage forecast**, per land matching the face-up Ravage card: invader
  damage, Defend, damage to the land, Dahan destroyed, Dahan counterattack,
  blight added, cascade target. Tokens and adversary rules that change the
  result are applied when mapped; otherwise `approx` with a warning.
- **Build forecast**, per land matching the face-up Build card.
- **Explore odds**: next card's stage (from its back), probability of each
  terrain or coastal result. Candidate set per stage = rules pool for that
  stage − cards seen. Per land: P(explored next round); lands with Brett's
  presence or sacred sites at risk.
- **Card counting**: per stage, terrains seen vs. unseen.
- **Adversary changes to the pool.** `data/adversary-stacks.json` is mostly
  unverified (61 `[VERIFY]` entries), so it is not used for stage order (the
  card backs give that). Adversary changes to *which cards are in a stage's
  pool* are added to `rules.rs` per adversary level only after verification;
  until then the stat uses the base pool and is marked `approx` with a warning
  naming the adversary level.

### 7.2 Fear and victory race

- Fear to next card; cards earned and left per terror level; projected round
  for each terror level.
- Fear per round (from history) with a rolling rate and a min/max band.
- Current victory condition and the invaders still blocking it.
- **Race chart:** projected win round vs. projected end-of-time round
  (invader deck runs out; exact, public deck size) vs. projected blight-card
  flip round (before the flip) or blight-loss round (after the flip).
  Adversary-specific loss conditions are not modeled in v1; a warning names
  the adversary when it has one.

### 7.3 Blight and loss risk

- Blight left on the card; flipped or not.
- Lands one Ravage from blight; lands with blight (cascade risk) and their
  cascade neighbors.
- Projected round of the flip (or of the loss, after the flip) at the current
  rate.

### 7.4 Spirit economy and powers

- Energy and card plays per round (history); presence-track progress.
- Elements vs. innate thresholds. `common::stats::compute_innate_status`
  exists but has no caller and no spec loader; v1 adds a loader from the
  spirit Wiki JSON and makes the check phase-aware, per the rule verified with
  `si-rules-check`.
- Power draws: hypergeometric P(≥ k cards with element E) for a Minor or Major
  draw (reuse `common::probability::hypergeom_at_least`). Unseen pool =
  allowed pool − cards outside the deck now (hands, discards, played,
  forgotten, removed). The pool size must equal the deck count; a mismatch is
  a warning and the stat is `approx`. Card elements from
  `data/decks/minor.json` and `major.json`; the 3 cards with no elements count
  as "no element".
- Requires an engine-card-ID → repo-card-slug map (Section 14).

## 8. Dashboard UI

- **Separate frontend root.** `main.ts` mounts `CompanionApp.vue` when the URL
  hash is `#/companion`, and `App.vue` otherwise. `App.vue` (with its state
  load, watchers, and `saveState` timer) never mounts on the companion route,
  so the companion cannot write tabletop state.
- New folder `tools/si-live/frontend/src/companion/`: `CompanionApp.vue`,
  `CompanionDashboard.vue`, one panel per stat family, `SourceStatus.vue`,
  `IslandHeatMap.vue`, and `useCompanionStream.ts` (SSE client with
  reconnect). The companion keeps its own SSE-fed ref.
- **Read-only, purpose-built panels.** The existing `Board.vue`,
  `InvaderDeck.vue`, and `FearDeck.vue` are v-model editors and are not
  reused. `winprob.ts` (a pure function of `GameState`) is reused on
  `CompanionState.game`.
- **Island heat map geometry.** `boardLayouts.ts` is not used: it has real
  positions only for board A, eyeballed ones for B, C and D aliased to B, and
  nothing for E–H or for multi-board placement (about 25 of Brett's 45 games
  use C, E, F or G). The engine stores no land coordinates; `LandCorner` holds
  topology (which lands meet at each corner, and side order per land). So
  `IslandHeatMap.vue` draws a **schematic layout** computed in the backend
  from adjacency + corner topology + the engine's board arrangement
  (`BoardLayout`), for every board and combination. Each land is a labeled
  cell with terrain color, risk shading, and piece counts. Artwork-accurate
  geometry is a later improvement.
- Edits outside `companion/`: `main.ts` (root choice) and one link in
  `App.vue` to `#/companion` that opens a new tab (`target="_blank"`), because
  `main.ts` picks its root once at page load. `CompanionApp.vue` reloads the
  page on a `hashchange` away from `#/companion`. `AppShell.vue` is not
  touched (it holds uncommitted work in the main checkout).
- Layout for a second monitor: status strip on top (round, phase, source
  health, race chart); island heat map (each land shaded by
  ravage/build/explore risk); four panel columns below.
- Charts follow the `dataviz` skill and the existing Aegis design tokens.

## 9. Errors and edge cases

| Case | Handling |
|---|---|
| Save read while the game writes it | Decode fails or short read → retry with backoff; keep the last good snapshot; `stale` flag after 2 s. |
| Undo | See "Which save is the truth". Events after the last save are dropped when an older state arrives. |
| New game | `log.txt` shrinks or starts with `GameView Start` → new `game_id`, history reset. |
| Game at menu or closed | Show the last state, marked `not live`. |
| Game update | Compare the engine DLL SHA-256 with the one in `engine-model.json`. On mismatch: banner, keep running, flag unmapped fields. |
| Unknown class, field, piece, or card | Kept internally; a warning names its type, never a hidden card name. |
| Unparsed log line | Counted by action type in `/health`. |
| Multiplayer game | `IsGameMultiplayer` → warning; v1 stats may be wrong. |
| Turn-1 Event | The engine moves an Event card to the discard during round 1 (`SeekingTheInterior` observed). The companion's event panel uses its own model from engine data, not the tabletop `EventDeckState.previewed` model. |

## 10. Testing

- **Fixtures** (gzip-compressed, `si-steam/tests/fixtures/`): the April 27
  game's rolling saves (`Ravage`, `Build`, `Explore` ×2,
  `AdvanceInvaderCards`, `SlowPower`, `Undo-000`), its `EndGame` `.dat` and
  `.txt`, plus two older `EndGame` `.dat` files with a different spirit and
  board (one on board G).
- **Decoder**: every fixture decodes with zero unknown record types. An
  ignored integration test decodes all 45 `EndGame` `.dat` files under
  `SI_STEAM_DIR`.
- **Golden tests**: fixture → `PublicSnapshot` JSON.
- **Firewall**: the two invariance tests and the field-name test (Section 5).
- **Log parser**: unit tests from real log lines; the 4 `EndGame` `.txt` logs
  parse with a known unparsed count; replaying the April game's log ends
  consistent with its final save.
- **Stats**: hand-computed cases (hypergeometric, ravage, explore odds).
- **Live check and fixture capture** (M1): run against the running game;
  updates within one poll interval of a `SAVING` line; undo behavior
  confirmed. The April 27 saves were all written in the last 13 seconds of
  that game, so M1 also captures new fixtures during live play: an early
  round (Growth and Fast phases, early invader deck), a mid-game save with an
  earned but unresolved Fear card, and an undo. These join the firewall tests.

## 11. REA integration

- `si-steam/scripts/gen-engine-model.sh` runs
  `rea inspect-managed-members --json` on `SpiritIslandEngine.dll` for the
  model types and writes `engine-model.json` (types, fields, field types,
  engine SHA-256, MVID). Checked in.
- On a game update: `rea compare-managed-members` (old DLL vs. new) → a report
  of changed model members; then regenerate the map.
- Frida (via REA) is a dev-time microscope only, for Phase B. Never shipped.

## 12. Phase B outline: `si-probe`

- Rust Windows binary, cross-compiled from WSL.
- Opens `SpiritIsland.exe` with read-only access; never writes memory, never
  injects, never touches the game folder.
- Walks Mono root domain → `SpiritIslandEngine` image → game controller static
  → `Game` graph, using `engine-model.json`.
- Emits `EngineSnapshot` through `GameSource`, so it passes the same firewall;
  POSTs to the backend on localhost.
- Specified in its own document after Phase A ships.

## 13. Milestones

1. **M1 — Read the game.** `si-steam` crate: NRBF decoder, engine map (REA),
   log parser, firewall, watcher, `PublicSnapshot`, API + SSE + health,
   firewall tests, live check (including undo).
2. **M2 — Stats.** `rules.rs` (verified), the four stat families, history.
3. **M3 — Dashboard.** Companion root, panels, schematic island layout (backend)
   and heat map, race chart.

Each milestone ends with working, tested software.

## 14. Risks and open questions

1. **NRBF coverage.** BinaryFormatter has ~20 record types; the decoder must
   handle all that appear (classes with members, primitive and object arrays,
   references, nulls, generic `List`/`Dictionary`/`Tuple`/`HashSet`/`Stack`).
   Mitigation: decode all 45 `EndGame` files.
2. **ID mapping.** Engine card identifiers (`WaryOfTheInterior`,
   `CropsWitherAndFade`) must map to repo card slugs for elements, and engine
   spirit identifiers (`ShadowsFlickerLikeFlame`) to repo spirit slugs for the
   innate loader and `GameState.spirits` keys. Build each map from both sides;
   unmatched IDs are warnings.
3. **Rules accuracy.** Ravage, Build, cascade, victory, invader stage pools,
   and innate timing must be verified with `si-rules-check` before coding.
4. **Board geometry.** Engine land IDs (`SandsA4`) must map to si-live board
   layouts (`A4`). Expected to be direct; verify per board.
5. **Polling cost on `/mnt/c`.** `stat` over 9P every 250 ms is cheap; full
   save reads (~450 KB) happen only on new saves.
6. **Game-id linking.** Linking a live `game-id` to its later `EndGame--<id>`
   file is left to the history spec.
