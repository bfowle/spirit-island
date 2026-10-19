# Steam Companion — design

Date: 2026-10-18
Status: rev 7, building (M1 built; M2 stats and the M3 dashboard in progress). Rev 5 adds Sections 7.7–7.9
and the backtest (Section 10), from Brett's requests on 2026-10-18. Rev 6 adds Section 7.10 (retro), from
the 2026-10-19 playtest (`docs/playtests/2026-10-19-shadows-vs-prussia6.md`). Rev 7 adds Section 7.11
(adversary rule coverage), from the Sweden game.
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
5. **Derived numbers too.** Every stat, including Monte Carlo rollouts, samples
   hidden orders from the public belief (the unseen multiset per deck), never
   from the save. A rollout seeded from the real deck order would leak through
   its numbers with no card name in sight. Rollout outputs are part of the
   invariance fixtures.

The serialized RNG state matters beyond deck order: it would predict future
power-deck reshuffles and France/Sweden escalation picks. Russia L5 puts
invader cards inside the fear deck; their positions are public setup facts,
their identities hidden.

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

The model choices follow the research report
`reports/Spirit Island companion stat models.md`: exact ledgers first,
probability only where a hidden card meets a land. Layers, in build order:
ledgers (deterministic resolution with named, signed modifier terms) →
card counting (hypergeometric family on the unseen-card lemma) → per-land
threat map with land-tempo badges → "if X is next" timelines → momentum
projection → simulation (race and comparisons only). Displays use natural
frequencies ("2 of the 3 unseen Stage II cards explore here"), show what
changes rather than what to play, and pair each exact number with a pocket
rule ("1 ÷ unseen cards left in this stage"). The engine's own preview
functions (`GetRavagePreviewForLand`, `GetInvaderActionPreviewForLand`,
`IsTerrorWinPossible`) serve as test oracles for `rules.rs`.

`PublicSnapshot` stores spirits **N** and boards **B** separately, and each
rule in `rules.rs` is tagged with the unit it scales by (fear per card scales
with N; blight pool, lands hit per card, and Scotland's loss with B).

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
  Explore eligibility includes the source rule (the land has a Town or City,
  or is adjacent to one or to the Ocean) and Wilds/Isolate effects.
- **Land-tempo badges** per land (lumia's glossary): `ahead` (explored, not yet
  built), `on` (built, ravaging next), `b2b` (two invader actions in a row),
  `capital` (7 buildings during the Invader Phase). Face-up cards make most
  badges certain; the map separates certain from possible.
- **"If X is next" table:** for each possible next invader card, which lands
  explore, build, and ravage — the darkest and best timeline, exactly.
- **Card counting**: per stage, terrains seen vs. unseen.
- **Adversary deck recipes.** `data/adversary-stacks.json` is wrong (levels
  keyed by difficulty, most stacks incorrect), so it is not used. A recipe
  interpreter in `rules.rs` applies each adversary level's deck operations
  (remove unseen, move to a known position, replace, accelerate, Russia L5
  insertion; combined adversaries supporting-first), each verified and listed
  in the rules register. Unverified levels fall back to the base recipe,
  marked `approx` with a warning.

### 7.2 Fear and victory race

- **Fear runway by dividers:** fear to the next Terror Level = (cards above
  the next divider) × (fear per card) − fear generated, with overflow carried.
  Fear per card is 4 per spirit, 5 with an extra board, +1 per player at
  England L6. Shown in cards and as each spirit's share.
- **Terror Level timing as an accelerator.** For each banked (face-down) fear
  card, show the Terror Level it will resolve at, and the value of crossing
  the next divider *before* the Fear phase: the Level is set the moment the
  divider is reached, so every card resolving that phase uses the higher
  Level's text. Test 1, Turn 3: Display Your Wrath's +1 fear earned the last
  Terror Level I card during the Event; both banked cards resolved at
  Level II, and the share of fear cards that could save a ravaging land rose
  from 16 of 50 (32%) to 23 of 49 (47%). Spreading Timidity (Defend 2) then
  saved a land.
- Fear per round (from history), shown as observed values, not a fitted rate.
- Current victory condition and the invaders still blocking it.
- **Race chart, v1 (no play policy needed):** the exact deck clock (loss at
  turn D's Explore; D from the deck recipe), an invaders-only projection
  enumerated exactly over the next few cards ("if you do nothing"), and the
  fear runway from guaranteed sources plus the land inventory.
- **Race chart, later (simulation):** one cumulative incidence curve per
  outcome — fear win, blight loss, deck exhaustion, and each adversary loss
  clock (six of eight adversaries have one) — never each clock alone.
  Labeled with its rollout policy; Wilson intervals; common random numbers;
  a fixed seed per state.

### 7.3 Blight and loss risk

- Blight left on the card (2B + 1 before the flip); flipped or not.
- Before the flip, the flipped side's blight count X is a mixture over the
  unseen blight cards (shown as a distribution, not a single number).
- Lands one Ravage from blight; lands with blight (cascade risk), their
  cascade neighbors, and the loaded-cluster cascade reach.
- Presence at risk: adding blight destroys one presence of each spirit in the
  land.
- Adversary blight rules (Sweden L1, Scotland L5, France L5, Habsburg Mining
  L1) as signed modifier terms.

### 7.4 Spirit economy and powers

- Energy and card plays per round (history), plus a forward **momentum
  projection** per growth line from the engine's presence-track graph,
  plotted against the invader-pressure step function from the deck recipe
  (Rei's "stay ahead of the curve" as a chart). Any Energy-per-card-play
  valuation is a user-set parameter with a sensitivity band, never a fact.
- Elements vs. innate thresholds. `common::stats::compute_innate_status`
  exists but has no caller and no spec loader; v1 adds a loader from the
  spirit Wiki JSON and makes the check phase-aware, per the rule verified with
  `si-rules-check`.
- Power draws: hypergeometric P(≥ k cards with element E) for a Minor or Major
  draw of 4, keep 1, the other 3 to the public discard (reuse
  `common::probability::hypergeom_at_least`). Reshuffles of the discard into
  the deck are tracked. Unseen pool =
  allowed pool − cards outside the deck now (hands, discards, played,
  forgotten, removed). The pool size must equal the deck count; a mismatch is
  a warning and the stat is `approx`. Card elements from
  `data/decks/minor.json` and `major.json`; the 3 cards with no elements count
  as "no element".
- Requires an engine-card-ID → repo-card-slug map (Section 14).
- **Need finder** (requested by Brett during Test 1, Turn 3). The player
  names a need — Defend ≥ n, Damage ≥ n, Gather/Push Dahan, Fear, Isolate, an
  element — and the panel shows P(a Minor or Major draw of 4 holds at least
  one card that meets it **and is playable now**: speed fits the phase, cost
  ≤ current Energy, a card play free), as natural frequencies plus a pocket
  rule. Next to it, the ledger delta of each growth option at the coming
  Ravage: blight per land, P(blight card flips this turn), presence lost,
  Dahan lost, fear cards earned, economy change. It shows what each option
  buys and never ranks them. Needs effect tags per card (Defend n, Damage n,
  Fear n, Gather/Push, Isolate, …), taken from the engine's embedded card
  definitions, cross-checked against `data/decks/*.json`.
  **"Saves the land" means any effect, not one keyword**: Defend, moving the
  Town out, Strife on the Town, enough damage, blight returned to the card,
  skipping the Ravage. A Defend-only count gave 17 of 96 Minors (55%); all
  effects give about 27 (74%) — Brett found the miss live (Lure of the
  Unknown, Cycles of Time and Tide).
  **"Playable now" includes reach**: range from presence or Sacred Site,
  target restrictions ("one of your lands", "land with Dahan", "land with no
  invaders"), and spirit rules that extend reach (Shadows pays 1 Energy to
  target a land with Dahan). Test 1, Turn 4: a plan to downgrade C7 with
  Crops (range 0), push with Mantle ("your lands") and damage with Bats ("per
  Dahan") failed on reach alone; C7 had neither presence nor Dahan.
  **Each candidate card gets three values**, shown side by side:
  1. *This turn:* what it changes at the anchor step.
  2. *Repeatability:* how likely its conditions keep holding (e.g. "a land with
     no invaders within range 2" gets scarcer as the island fills), and
     whether it removes the problem or moves it (a gathered Town is still a
     Town).
  3. *Elements:* its contribution to the spirit's innate thresholds, weighted
     by the spirit's scarce element, and the card plays needed to reach each
     level with it vs. without it (Shadows: Fire is the bottleneck; Lure's
     Moon + Fire + Air makes Darkness Swallows the Unwary L3 reachable at 4
     card plays, which the starting deck cannot reach at all).
  Worked example (Test 1, Turn 3, Shadows vs Prussia L2): C4 needed Defend 2;
  about 15 of 96 unseen Minors qualify, so P ≈ 1 in 2 per draw of 4, which
  made "Reclaim + Minor" and "Minor + Presence" equal on blight (50% flip)
  and different on Dahan kept vs. economy grown.

## 7.5 The scenario lens (core interaction)

Stated by Brett during Test 1, Turn 3, after working the "save C4 / keep
the island healthy" decision by hand. The dashboard is organized around this
interaction; the stat families of Sections 7.1–7.4 feed it.

**Anchor:** a question about a coming step, picked by the player or suggested
by the phase: "this turn's Ravage", "next Explore", "does the blight card flip
this turn", "can I earn a fear card before Ravage".

**Inputs, in three groups, shown side by side:**

| Group | Examples | Treatment |
|---|---|---|
| Fixed facts | Face-up invader cards, pieces, tokens, blight left, fear pool, hand, Energy, card plays | Ledger terms, certain |
| Levers (you control) | Growth option, which track a presence comes off, cards to play, targets, Event choices, fear-card targeting, payments ("aided by Air") | Enumerated as options (a handful, not every permutation) |
| Unknowns (you can't control, but can count) | Next invader card, a Minor/Major draw, earned fear cards still face-down, the coming Event, the blight card's other side | Exact distributions from public pools (Section 5); independent decks combine by multiplying misses |

**Model:** depth-1 expectimax with chance nodes at reveal points
(the research report's recommended planner). For each lever option, each
possible unknown card is applied to the current state through the rules
ledger (not keyword tags), giving "saves land X / hurts land X / neutral";
probabilities come from the counts. Multi-turn effects use the momentum
projection.

**Output, for each option (never ranked):**
1. The anchor's probability as a natural-frequency bar ("island still healthy
   after this Ravage: A 79%, B 69%, C 32%"), with the parts that make it up
   ("Defend draw 55% or 2 fear cards 54%").
2. An outcome-by-branch table (unknown resolved one way vs. the other).
3. What each option buys beyond the anchor (Dahan kept, presence, cards,
   Energy, Reclaim timing).
4. **Pace:** the option's economy curve (Energy, card plays, presence, cards
   gained, Reclaim schedule) against the adversary's pressure curve (lands hit
   per invader card from the deck recipe, builds, blight runway, fear runway)
   over the next turns, with the gap's trend and the turn, if any, where the
   economy outgrows the pressure ("outgrowing the adversary").

Worked example: Test 1, Turn 3 (Shadows vs Prussia L2) — Defend-draw odds
(17 of 96 Minors), fear-card saves (16 of 50 at Terror Level I), the Event as a
common shift, and the Reclaim-now vs. Reclaim-later pace comparison.

## 7.6 Augment, don't decide (core principle)

Stated by Brett during Test 1, Turn 4: the companion augments the player's
choices with math and **presents things to think about**; it never makes the
call. Every lens and panel follows these rules:

- No rankings, no "best move", no highlighted option, no default selection.
  Options appear in a fixed order (by lever), never sorted by value.
- Every number answers a named question and shows its parts, as natural
  frequencies ("17 of 96 Minors"), with a pocket rule the player can
  compute at the table.
- Each decision shows **think-about prompts**. The catalog below grew out of
  the Test 1 decisions; each prompt names the panel that answers it.

| Prompt | Answered by | Test 1 example |
|---|---|---|
| What is certain this step, and what is still hidden? | Fixed facts / unknowns columns | Turn 3: Ravage lands certain; Minor draw, fear cards, Event hidden |
| Which land is one action from safe, and which is not worth saving? | Cost-to-save per land | Turn 4: a lone-Explorer land is 1 action; C7 at 9 damage is not |
| What does this card buy now, and later? | This turn / repeatability / elements | Lure: saves C4 now; scarcer targets later; enables innate L3 |
| Can it reach? | Reach check (range, targets, spirit rules) | Turn 4: Crops, Mantle, Bats could not reach C7 |
| When does it act? | Timing (Fast/Slow, next action a skip hits, Terror Level at resolution) | Spiders now skips this Build; held, it skips Turn 5's Ravage |
| Which clock does it move, and by how much? | Race / loss table | Turn 4: 9 → 6 Turn-5 blights moves loss risk 57% → 4% |
| What is this turn's bottleneck? | Resource strip (Energy, card plays, elements) | Turn 4: 8 Energy but 2 card plays |
| Is this factor common to all options? | Common-factor callout | Turn 3: the Event shifts every option alike |
| Is this about order or about count? | Pace / Reclaim schedule | Turn 4: G1 vs G2 differ in order only; L2 lands on Turn 5 either way |
| Does it remove the problem or move it? | Repeatability value | Lure gathers a Town elsewhere; the Town still exists |
| Which effects might you be missing? | Need finder over all effect types | Turn 3: Defend-only count missed Lure and Cycles |

- The optional predict-first hold (Section 6 of the report) stays available
  for players who want to commit a guess before the numbers show.

## 7.7 Post-game review

Requested by Brett on 2026-10-18: "where did the momentum shift, where were
suboptimal picks made, what can we learn from each scenario after the game is
over … too much focus on ravages and not preventing builds (or explores)".
Built in `si-common/src/companion_stats/review.rs` from the game's timeline of
public snapshots (so it has the same firewall guarantees as the live view).
Each change between two snapshots is attributed to the step it happened in
(a save is written on entering a step, so blight from a Ravage first shows in
the Build save).

- **Blight attribution**: each blight by turn, land, and cause (`Ravage`,
  `cascade`, `Event`, `BlightedIsland`, …).
- **Invader chains**: for each blight from a Ravage, the same card's Explore
  and Build in that land, and the Invaders standing there at each point a
  Fast or Slow action could act ("Turn 3, after Explore: 1 Explorer" →
  "Turn 5, before Ravage: 3 damage"). This is the prevention-vs-reaction view.
- **Flow by phase**: Invaders added (Explore, Build, Event, …) vs removed
  (Fast, Slow, Fear, Ravage counterattack, Event) per turn, at island level so
  pushes and gathers cancel.
- **Pressure**: total Ravage damage the island's Invaders could deal at the
  end of each turn, and the turn of the largest jump.
- **Reflections**: questions with this game's facts (never a grade, per 7.6):
  where blight came from; which Ravage blights began as a lone Explorer; did
  removal keep pace with arrival and in which phase; what happened in the
  largest jump; were unused plays or banked Energy deliberate.

Test 1 (Shadows vs Prussia L2, lost to blight Turn 7): 6 blight from Ravage,
2 from cascades, 5 from Events; 3 Ravage blights began as a lone Explorer two
turns earlier; Explore added 29 Invaders and Build 17, while Fast powers
removed none net (Lure and similar cards moved Invaders), Slow 3, Fear 3,
counterattacks 12, Events 13; the largest pressure jump was Turn 4 (+19
damage: 6 Builds and 6 Explores).

Later: decision points from the log (`UseActivationJournalEntry` / log
`Proceed` lines: growth chosen, cards kept, targets), so the review can name
the choice at each chain point. The log names hidden cards, so this runs only
after the game ends, through `projection.rs`.

## 7.8 Invader pipeline invariants and the timing lens

Stated by Brett on 2026-10-18 as things the companion must treat as obvious:
Explore → Build → Ravage across turns, resolved in reverse order within each
Invader Phase; adversary changes (England's extra Build); Fast vs Slow; inland
lands can be made unexplorable by clearing sources, coasts cannot. Built in
`si-common/src/companion_stats/invaders.rs` and `rules.rs`:

- **Resolution order.** The forecast applies this phase's Ravage, then its
  Build, then computes Explore on the resulting board. A Build can create an
  Explore source; a counterattack that destroys every Invader whatever the
  damage assignment removes one (Test 1 Turn 3: Jungle E3's Dahan destroyed
  the Town that fed Mountain E5 and Wetland E4, so neither was explored).
- **Two-turn pipeline per land.** This phase: Ravage (ledger), Build (+Town or
  +City), Explore chance. Next phase: Ravage next turn (the Build card's lands;
  certain when Invaders are already there), Build next turn (chance from this
  Explore's candidates). Badges: `ravage-now`, `build-now`, `ravage-next`,
  `back-to-back`, `explore-dead`, `sole-source`.
- **Explore access.** `coastal` (always explorable; the Ocean is a permanent
  source), `fed` (sources in range, listed), `dead` (no source in range after
  this phase's Builds). `sole_source_for`: inland lands whose only source is
  this land, so clearing its Towns and Cities leaves them unexplorable.
- **Fast vs Slow windows** per land, as mechanical statements: what an action
  resolving in the Fast phase (before the Invader Phase) still changes here
  this turn, and what a Slow action (after it) changes for next turn, e.g.
  "If explored (2 in 5), clearing that lone Explorer after the Explore stops
  next turn's Build here".
- **Modifiers from the engine's own status effects.** Status effects from
  public cards cross the firewall with their parameters. Applied: Defend
  (flat, per piece), Ravage damage per Invader, Skip (optional, uses), no
  cascade, health changes (already on the pieces). Token rules applied with
  engine evidence: Strife (no Ravage damage, loses 1), Disease (cancels a
  Build, 1 removed), Wilds (cancels an Explore, 1 removed). Anything with
  conditions the rules do not interpret is flagged `unmodeled`, never guessed.
- **Adversary and spirit exceptions.** The rules register
  (`data/references/rules/register.md`, `common/data/invader-rules.json`)
  lists every adversary level, spirit, scenario, and Event that changes the
  pipeline, with engine and Wiki evidence. Effects not yet applied show as
  notes on the phase forecast (extra invader slots are already detected).

## 7.9 Stochastic-process framings that fit (and one that does not)

Brett asked whether M. S. Bartlett's *An Introduction to Stochastic
Processes* (1955) or later work applies. What fits:

1. **Birth–death–immigration.** Per region, Explore is immigration, Build is
   birth (it needs Invaders present), spirit removal is death. Coastal lands
   have immigration that never stops (the Ocean is always a source); an
   inland cluster has an absorbing "explore-dead" state once every source in
   range is cleared. This is the stochastic form of 7.8's Explore access.
   Candidate stat: turns until a dead land is re-seeded, from the deck's
   terrain mix.
2. **Epidemics on a graph / fade-out vs persistence** (Bartlett's 1957 work
   on community size). Blight cascades spread over the adjacency graph
   through already-blighted lands; a connected blighted cluster is the
   "susceptible network". Candidate stat: size of the blighted component each
   land touches (cascade reach), next to `cascade_chains`.
3. **Absorbing Markov chains and competing risks.** The game ends in
   absorbing states (Fear win, blight loss, deck exhaustion, adversary
   losses); the race chart (7.2) is the first-passage-time distribution per
   outcome, shown as cumulative incidence.
4. **Linear filter / impulse response.** Each invader card is an impulse
   through a three-tap delay line (Explore, Build, Ravage on consecutive
   turns); adversaries change the taps (England adds a Build) or the input
   (deck recipes). Removing an Explorer at the first tap removes the two later
   responses; acting at Ravage removes only the last. This is the mental model
   the review's invader chains display.
5. **Bartlett's formula for autocorrelation variance**, as a limit. With about
   8 turns per game the 95% band is about ±2/√8 ≈ ±0.7, so "momentum" cannot
   be detected statistically inside one game. The review therefore attributes
   mechanistically (which Explore and Build caused which Ravage), and any
   cross-game statistics wait for enough games.

What does not fit: **spectral estimation and power spectra** (periodograms,
Bartlett's averaged periodogram). They need long, stationary series; a game is
8–12 turns and non-stationary (stages escalate, Terror Levels change), and the
only periodicity (the three-turn pipeline) is known exactly from the rules, so
there is nothing to estimate. Likewise, Gaussian (Wiener/Kalman) filtering of
the hidden deck is worse than exact card counting.

## 7.10 Retro: turn-by-turn evaluation (rev 6)

Brett's request after the 2026-10-19 playtest (`docs/playtests/2026-10-19-shadows-vs-prussia6.md`):
a retroactive, turn-by-turn evaluation of what went wrong, which choices were
costly, and exactly what was on the board when each decision was made, as a
timeline with a breakout per turn. Section 7.7's review (Blight causes, chains,
flow) is the starting point; this adds decisions.

**Principles.**

- **Judge by what was knowable then.** Each decision is evaluated against the
  public snapshot at the moment it was made (the firewall projection), never
  against cards revealed later. Hidden information that arrived afterward (the
  Explore card, Events, Fear cards, power draws) is shown separately.
- **Separate decisions from luck.** Every outcome is attributed: a decision,
  an Event, a Fear card, the Explore card, or a rule. Saves that came from
  hidden information go in a luck ledger, not to the player's credit or blame.
- **Facts and flags, not verdicts.** Flags name a pattern ("damage reduced but
  still at the Blight line") with the numbers; the reader judges. This keeps
  section 7.6 (augment, don't decide) after the game too.
- **Counterfactuals only where the rules model them**, and for one phase:
  "Defend 5 on B3 → 2 fewer Blight this Ravage", never multi-turn simulations
  presented as fact. A multi-turn "what if" is labeled hindsight.

**Data.** Decision points come from `log_parse` decisions (with the undo fixes
in the playtest's U19) joined to the snapshot just before each decision. The
forecast at that moment is `CompanionStats` recomputed on that snapshot.
Outcomes come from the following snapshots (`changes.rs`), and reveals come
from the log with timestamps.

**Per-turn record.**

| Part | Contents |
|---|---|
| Board then | Ravage, Build and Explore cards; each Ravage ledger; Blight card buffer and the chance of losing (U16); Presence count; Fear and Terror Level; hand, discard, Energy and Card Plays |
| Decisions | Growth, draft and Forget, cards played with targets, innate levels, Event and Fear choices (push destinations, token placement, cascade targets) |
| Alternatives visible then | The companion's prompts, need finder and draft lens (U13) at that moment |
| Outcome | Blight added and its causes, pieces added/removed by source, Fear, buffer change; forecast vs actual |
| Flags | From the catalog below |
| Luck | Saves or losses from information revealed after the decision |

**Flag catalog (initial).**

| Flag | Pattern | Example (2026-10-19) |
|---|---|---|
| F1 | A Ravage land one action from safe left unaddressed | |
| F2 | Damage reduced but still at or above the Blight line | T2: Crops on B1 3 → 2 |
| F3 | Pieces moved into a land the visible cards say Builds or Ravages next, or that already has Blight | T4: Mantle pushed into B6 with Sands+Wetland face up |
| F4 | The last Fast protection Forgotten or passed while the Blight buffer is low | T3 Forgot Drift; T4 passed Sap; T5 Forgot Concealing |
| F5 | Energy and plays spent only on Slow cards when this turn's Ravage decides the game | T6 |
| F6 | Blight buffer turning points (largest drops; flips; new cards) | T2: 3 → 1 |
| F7 | Saves from hidden information (luck ledger) | Canny Defense T3, Sense of Dread T4, Seek Safety T5 |
| F8 | A draft or Forget that skipped an option meeting a current need | T4: Sap would have stopped B8 |
| F9 | Presence spent while Presence is a loss clock | T5: Transformative Sacrifice |
| F10 | A move into a doomed-anyway land (positive: no extra Blight) | T1: Mantle into B2 |

**Counterfactual runner.** Rerun the Ravage ledger and Blight count for one
phase with one change: Defend N on a land, a different push or gather
destination, pieces removed, or a different card from the same draw (if it
can act this turn). Report the change in Blight and cascades, nothing more.

**UI.** A retro timeline (one row per turn: Blight buffer, Blight added, the
key decisions as chips colored by flag) with a click-through breakout per turn
(board then, decisions, alternatives, outcome, flags, luck). A short "where it
turned" summary at the top lists the largest buffer drops and the flags behind
them.

**Tests.** The 2026-10-19 capture (gitignored, tests skip without it) as a
fixture, with the playtest log's hand-written breakdown as the reference
answer; synthetic snapshots for each flag.

**Milestones.** R1 decision-point snapshots and per-turn records; R2 flags
F1–F10 and the luck ledger; R3 counterfactual runner; R4 the timeline and
breakout UI. R1 depends on the decision-parser fixes (playtest U17, U19) and
R2 on the loss-risk model (U16) and the cancel-effect fixes (R4, R6, R7).

## 7.11 Adversary rule coverage (rev 7)

Brett, 2026-10-19 (Sweden game): the companion shows "the ACTUAL things like
Sweden's extra dmg". Every adversary rule that changes a number the
companion shows is applied in `companion_stats/adversary.rs`, one hook per
point where the rule acts. A rule the players resolve by choosing (Q21) is
listed with its options instead of being guessed. Each adversary's own loss
condition is a clock (`LossClock`), read from the save where the engine
keeps the count on the adversary's panel (Russia: destroyed Beasts).

Status key:
- **verified**: modeled, and the backtest matches the engine in a recorded game.
- **modeled**: applied from the printed rule; no recorded game yet.
- **options**: the players choose; the forecast lists the choices.
- **state**: already in the game state (setup pieces, deck recipes, piece Health).
- **clock**: a loss clock.
- **not modeled**.

| Adversary | Rule | Status |
|---|---|---|
| England | Building Boom (Escalation) | verified (England game) |
| | Proud & Mighty Capital (loss) | clock |
| | L1 Indentured Servants Earn Land | verified |
| | L2 Criminals and Malcontents | state |
| | L3/L4 High Immigration | verified (L3); L4 reads the same slot |
| | L5 Local Autonomy | state (piece Health from the save), not yet seen |
| | L6 Independent Resolve | note only (High Immigration twice when no Fear Card resolves): not modeled |
| Sweden | Swayed by the Invaders (Escalation) | verified (Sweden game) |
| | L1 Heavy Mining | verified (Blight amounts checked) |
| | L2 Population Pressure, L4 Royal Backing, L6 Prospecting Outpost | state |
| | L3 Fine Steel for Tools and Guns | verified |
| | L5 Mining Rush | options |
| Russia | Stalk the Predators (Escalation) | options; verified (Russia game, Mountain A1) |
| | Hunters Swarm the Island (loss) | clock (adversary panel count verified: 2 at that game's end) |
| | L1 Explorers +1 Damage | verified; the Beast it destroys is noted and counted |
| | L2 A Sense for Impending Disaster | note on counterattacks (1 Explorer Pushed) |
| | L3 Competition Among Hunters | verified |
| | L4, L5 deck | state; L5's card in the Build slot is verified (G2) |
| | L6 Pressure for Fast Profit | modeled (applied when one land has the most Explorers) |
| Brandenburg-Prussia | Land Rush (Escalation) | options; verified (Test 1, Prussia game, Prussia + Blitz game) |
| | L1–L6 | state; L2's early Stage III card verified in play (NQ-P: it explored on Turn 2 as a Stage III option) |
| France | Demand for New Cash Crops (Escalation) | options |
| | Sprawling Plantations (loss) | clock (Towns on the island against 7 per player) |
| | L1 Frontier Explorers | modeled (Explore note per card) |
| | L2 Slave Labor | modeled |
| | L3 Early Plantations, L5 Slow-Healing Ecosystem | state |
| | L4 Triangle Trade | modeled (options when the fewest-Town land ties) |
| | L6 Persistent Explorers | options; "Fear never removes Explorers": not modeled |
| Scotland | Ports Sprawl Outward (Escalation) | options |
| | Trade Hub (loss) | clock |
| | L1 Trading Port | modeled (Explore note) |
| | L2, L4 | state |
| | L3 Chart the Coastline | modeled |
| | L5 Runoff and Bilgewater | modeled |
| | L6 Exports Fuel Inward Growth | modeled |
| Habsburg Livestock | Seek Prime Territory (Escalation) | options |
| | Irreparable Damage (loss) | clock: this phase's count is modeled; the running count reads the adversary panel, unverified |
| | L1 Migratory Herders | modeled (options when several neighbors can give a Town) |
| | L2 More Rural Than Urban | modeled |
| | L3 Fast Spread | state |
| | L4 Herds Thrive in Verdant Lands | state if the engine's piece Health includes it: unverified |
| | L5 Wave of Immigration | not modeled (the reminder card in the deck) |
| | L6 Far-Flung Herds | modeled |
| Habsburg Mining | Mining Tunnels (Escalation) | options; verified (Habsburg game) |
| | Land Stripped Bare (loss) | clock with the value after this Invader Phase; verified (the Habsburg game was lost to it) |
| | L1 Ceaseless Mining | verified (Build cards Ravage in Mining lands; Disease stops those Ravages) |
| | L1 Avarice Rewarded | modeled (no cascade); not yet seen |
| | L2, L4 | state |
| | L3/L5 Mining Boom | options, with which lands a new Town would open to Exploring; L3 verified (one upgrade per board each turn) |
| | L6 The Empire Ascendant | modeled (Explore note) |
| Blitz (scenario) | extra Explorer per board | options (Explore note); seen in the Prussia + Blitz game (Sands C2: 2 Explorers) |
| | extra setup actions, extra Blight | state; the Turn 1 Ravage is verified (NQ-J) |
| | all Powers Fast | modeled (spirit panel: hand and innates count as Fast) |

Game 4 (Prussia L2 + Blitz) verified the Land Rush and Blitz rows; game 5
(Habsburg Mining L4) verified Ceaseless Mining, Mining Boom I, Mining
Tunnels and Land Stripped Bare.

Loss clocks within one step of their limit (now or after this Invader Phase)
and next turn's Ravage when it would empty or flip the Blight card are
called out at the top of the native app's Invader Phase view (Brett, games 4
and 5: neither was visible a turn ahead).

Spirit rules that change the numbers follow the same principle. Modeled so
far: A Spread of Rampant Green's Choke the Land with Green (a skip), and
Vengeance as a Burning Plague's The Terror of a Slowly Unfolding Plague (a
Disease-stopped Build becomes the player's choice) and Lingering Pestilence
(Ravage Blight on its Presence adds a Disease before the Build).

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
| Turn-1 Event | Rule (Jagged Earth rulebook p.12, confirmed by Brett): the Turn-1 Event is drawn and discarded without resolving. The engine matches (`SeekingTheInterior` moved Event Deck → Event Discard in round 1). The card is public (Event discard) and leaves the unseen pool. The companion's event panel uses its own model; the tabletop `EventDeckState.previewed` model encodes the wrong rule and is not used. |

## 10. Testing

- **Fixtures policy (rev 5).** The repo is public, so Brett's saves are not
  checked in. Tests that need real data read the local capture
  (`data/steam-capture/2026-10-18-test1`, gitignored, or `SI_STEAM_CAPTURE`)
  and the installed engine DLL, and skip when either is absent. Rules have
  synthetic unit tests that always run.
- **Backtest (rev 5).** `companion_stats/backtest.rs`: for every turn, the
  forecast made on entering Ravage (after the Fast phase) against what the
  engine then did (Ravage blight per land, Builds with piece type, Explores
  for the revealed card). Test 1 drove the rules: Strife, Disease, Wilds,
  Defend per Dahan, −1 damage per Invader, and counterattack clears were each
  found by a mismatch. `steam/tests/backtest.rs` asserts zero mismatches on
  all six Invader Phases of Test 1.
- **Fixtures** (original plan, superseded above): the April 27
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
   Every rule in `rules.rs` is listed in a **rules register**
   (`data/references/rules/register.md`), grouped base → Branch & Claw →
   Jagged Earth → Horizons → Nature Incarnate, plus a cross-layer interactions
   section. Each entry has its sources (rulebook page, engine evidence) and a
   status: `unreviewed` or `reviewed by Brett`. Brett reviews the register in
   batches; the dashboard marks a stat whose rules are not yet reviewed.
   Open conflicts go to `data/references/rules/open-questions.md`.
3. **M3 — Dashboard.** Companion root, panels, schematic island layout (backend)
   and heat map, race chart.

Each milestone ends with working, tested software.

**Status (2026-10-19).** M1 built: NRBF decoder (all 327 Test 1 saves and all
46 EndGame files decode), engine map, DLL definition extractor, firewall with
invariance and field-name tests, live watcher and capture replay, API + SSE +
health + replay control. In place of a raw `log.txt` event stream, the
play-by-play is derived from snapshot diffs (it cannot leak hidden Fear
cards), and the log is parsed only for the player's own decisions
(`log_parse.rs`, whitelisted kinds, undo-aware). Live mode records every game
as a capture (`data/steam-capture/live-<game-id>/`, saves gzipped) so it can be
replayed and reviewed. M2 largely built: invader forecast (backtested to zero
mismatches on Test 1), status-effect and token rules, fear, blight, spirit
economy, innate thresholds, draw odds, need finder, rules in play (rules
register), prompts, post-game review with decisions. M3 built: the dashboard
(`frontend/src/companion/`). Not yet: race chart and simulation (7.2), the
scenario lens's option enumeration (7.5), the momentum projection (7.4), and
MCP access to `steam-game.json`.

## 14. Risks and open questions

0. **Several engine builds.** Brett's save folder holds saves from three
   engine builds (1.0.9116, 1.0.9602, 1.0.9776) whose fields differ. The
   decoder is generic, but `engine_map` must tolerate missing or renamed
   fields per build; fixtures include at least one save per build.
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
