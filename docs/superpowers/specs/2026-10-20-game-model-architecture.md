# The game in code: model, replay, metrics, clients

Date: 2026-10-20
Status: accepted 2026-10-20 (decisions in Section 9). Companion to `2026-10-20-tempo-and-growth-model.md` (the quantities) and `2026-10-19-si-corpus-design.md` (the facts). Decisions marked **(Brett)**.

## 0. What Brett asked for (2026-10-20)

- A CLI that takes an entire gameplay log and replays it through the game
  mechanics written as Rust structs and traits, reproduces everything the log
  says happened, and reports or plays it back.
- On that model: power card costs, every kind of growth (spirit and invader),
  Rei's tempo, and questions asked against real runs. Rei's builds run as
  guides against actual games.
- "I basically want the game mapped out properly in code. Period."
- Shared Rust for the whole game in one place; clients (CLI, native UI, web)
  as thin consumers, broken out later. Reorganize the repo if needed.

## 1. The principle: the log is the oracle

We do not get to decide whether the model is right. The 33 full game logs
(and every future game) decide. A replay runs the log's events through the
model; wherever the model computes a consequence the log also records
(damage dealt, Blight added, Fear generated, Dahan killed, cascades, Terror
level, Energy, elements, card plays, presence), the two are compared. Where a
capture also has saves, the whole state is compared at every save.

The output of a replay is therefore two things: the reconstructed game, and a
**coverage report**: which rules the model reproduced, which events it could
only copy from the log, and where it disagreed. That list is the work queue.
Rules exercised by Brett's games get written first; the rest wait until a
game exercises them.

## 2. Crates

Until 2026-10-20 the Rust lived in `tools/si-live/` as `common` (8.6k lines: public
snapshot, rules register, companion_stats), `steam` (4.1k: NRBF, engine defs,
projection, log parse, backfill, companion, dump CLI), `ui` (1.4k), `app`
(4.5k), `backend` (1.5k, the Vue tabletop server), `mcp` (0.2k). Proposed
layout at the repo root:

```
crates/
  si-model      the game as data; no I/O, no rules
  si-rules      the turn as a state machine; Invader phase, Growth, Fear, Blight, Time Passes; effect resolution
  si-defs       definitions → model types (engine dump today, the si-corpus later); aspects, adversaries, scenarios, boards
  si-record     a played game: Steam log → typed events; NRBF saves → snapshots; undo/rewrite handling; the replay driver and its diff
  si-metrics    pure functions over a record or a state: tempo, growth classes, odds, card pricing, chain/pace, projections
  si-policy     strategies as code: Rei's builds, the capacity knapsack, later the Monte Carlo runner
  si-cli        the one binary: replay, report, corpus, odds, cards, defs, dump
  si-ui         the wgpu toolkit (unchanged)
  si-companion  the native window (today's app), a client of si-record + si-metrics
  si-serve      the tabletop web backend (today's backend) and mcp, clients of the same crates
apps/web        the Vue tabletop frontend (today's apps/web)
```

What moves where:

| Today | Goes to |
|---|---|
| `common/public.rs`, `schema.rs`, `elements.rs`, `growth.rs` (track slot math) | `si-model` |
| `common/rules.rs`, `rules_register.rs` | `si-rules` (the register stays the written source of each rule) |
| `common/companion_stats/*` (chain, outlook, curve, pace, race, needs, spirits, adversary, invaders, ledger, whatif, backtest, review) | `si-metrics` (curve, chain, pace, odds) and `si-policy` (needs/knapsack, whatif, backtest) |
| `steam/nrbf.rs`, `engine_map.rs`, `projection.rs`, `source.rs` | `si-record` (saves → snapshots) |
| `steam/engine_defs.rs` | `si-defs` |
| `steam/log_parse.rs`, `backfill.rs` | `si-record` (typed log events; the backward walk becomes a check against the forward replay) |
| `steam/companion.rs` | `si-companion` |
| `steam/bin/dump.rs` | `si-cli` |
| `ui`, `app` | `si-ui`, `si-companion` |
| `backend`, `mcp`, `frontend` | `si-serve`, `apps/web` |

Retired: the hand-maintained `data/references/*.json` copies (the si-corpus
design already says so); any companion_stats module that the replay shows to
be wrong rather than incomplete.

## 3. `si-model`: the game as data

- **Island**: `Board` (A–H, thematic), `Land { id, terrain, coastal, adjacent, pieces, tokens, presence }`, `Terrain`, adjacency across boards, Ocean.
- **Pieces**: `Explorer | Town | City | Dahan` with health and damage; `Blight`; tokens `Beast | Wilds | Disease | Strife | Badlands | Vitality | Isolate`.
- **Spirit**: `SpiritPanel { tracks: Vec<TrackNode>, growth_groups, innates, uniques, special_rules, setup }`, `SpiritState { revealed nodes, presence on island (per land, sacred sites), energy, hand, played, discard, forgotten, elements this turn, plays this turn }`. Aspects as deltas on the panel.
- **Powers**: `PowerCard { id, title, cost, speed, range: Range, target: TargetKind, elements, effects: Vec<Effect>, threshold: Option<(ElementSet, Vec<Effect>)> }`; `Innate { tiers: Vec<(ElementSet, Vec<Effect>)>, speed, range }`.
- **Decks**: `InvaderDeck` (Stage I/II/III cards, slots Explore/Build/Ravage, adversary escalation), `FearDeck` (cards earned, Terror level thresholds), `EventDeck`, `BlightCard { healthy, flipped, pool }`, Minor/Major decks and discards (public part only when read from a game).
- **Setup**: `Adversary { name, level, flag, escalation, rules per level, difficulty }`, `Scenario`, `GameSetup { spirits, boards, adversary, scenario, players }`.
- **GameState**: all of the above at one moment, plus `turn`, `phase`, `fear { pool, earned, generated }`, `terror`, `defeat/victory` conditions.

Everything is plain data with serde; one `GameState` can be printed, diffed,
hashed and stored.

## 4. `si-rules`: the turn as a state machine

```rust
pub enum Phase { Growth, FastPower, BlightedIsland, Event, Fear, Ravage, Build, Explore, Advance, SlowPower, TimePasses }

pub enum Event {        // what happened, as the log records it
    Growth(SpiritId, GrowthChoice), PlayCard(SpiritId, CardId), UsePower { spirit, power, target, choices },
    InvaderStep { step: Step, card: InvaderCardId, lands: Vec<LandId> }, Ravage(LandId), Build(LandId), Explore(LandId),
    AddPieces(..), MovePieces(..), Damage(..), Destroy(..), Defend(..), Blight { land, amount, cascade }, Fear(i32), FearCardEarned, TerrorLevel(u8),
    Energy(SpiritId, i32), Elements(..), PresencePlaced(..), PresenceDestroyed(..), CardGained(..), Forget(..), Reclaim(..),
    EventCard(..), FearCard(..), PhaseChange(Phase), TimePasses,
}

pub trait Rule { fn applies(&self, s: &GameState, e: &Event) -> bool; fn apply(&self, s: &mut GameState, e: &Event) -> Vec<Event>; fn source(&self) -> &str; }
```

Two modes over the same code:

- **Replay** (first): the driver feeds logged `Event`s. For each one the
  rules compute the expected consequences (a Ravage's damage from the pieces
  present, Defend and Dahan; Dahan counterattack; Blight and cascade; Fear from
  destroyed Towns and Cities; Terror level from Fear cards; income at Growth;
  elements from revealed slots and played cards; innate tiers reached) and the
  driver compares them with the logged consequences. Match → the rule is
  verified on that event. Mismatch → reported with the turn, phase, land, both
  values. Unknown effect (a card the effect language cannot express yet) →
  copied from the log and listed as uncovered.
- **Simulate** (later, as coverage allows): the driver asks a `Policy` for
  decisions and the rules generate the consequences themselves; randomness
  from the decks.

**Effects language**: `Effect` is a small AST (Damage(n, to), Defend(n),
Push(n, what), Gather(n, what), Fear(n), RemoveBlight(n), AddToken(kind, n),
Isolate, Skip(step), DestroyPresence, GainEnergy(n), Repeat, Conditional(cond,
effects), PerElement(..), Range/Target modifiers …). It is parsed from the
engine's card text with per-card overrides where the text is irregular, and
checked by replay: a card's parse is verified every time a game plays it and
the logged damage, pushes and fear match. The coverage report lists cards by
how many logged uses verified them.

## 5. `si-record`: a played game

- `LogEvent` parser over the Steam log (today's `log_parse` and `backfill`
  know the formats: rewrites, bulk stamps, undo reloads, turn boundaries). It
  yields `Vec<(Stamp, Turn, Phase, Event)>`.
- Save decoder (NRBF → `GameState` snapshot), today's `engine_map` +
  `projection`, extended from the public snapshot to the full state.
- `Replay::run(log, saves) -> (GameRecord, Coverage)`: forward from setup
  (`PlayerSetup` is in 18 of the 19 EndGame logs; captures that joined late
  start from their first save and run both ways, as `backfill` does today).
- `GameRecord`: the state after every phase of every turn, every chain, every
  decision, every spirit's track row (the tempo spec's types).

## 6. `si-metrics` and `si-policy`

`si-metrics` holds the tempo spec: land chains and classes, track tempo and
spikes, build profiles, growth classes with the `Growth` trait, odds from the
decks, card pricing, and the existing chain/pace projections once they are
re-expressed over `GameState`. All pure functions, all tested on records.

`si-policy` holds Rei's builds as `Policy` implementations (growth choice,
play choice, placement priority) with his labels, so a logged game can be
scored for "how far from Rei's Fast Minors did this run stay, and where did it
diverge", and so a simulated game can be driven by a build once simulation
exists. The capacity knapsack and what-if walks move here.

## 7. `si-cli`

```
si replay <log|capture> [--at T3:Ravage] [--step] [--diff-saves]   reconstruct; print state; coverage summary
si report <log|capture>                                           the tempo spec's tables for one game
si corpus <dir>... [--csv]                                        one row per game, groupings, coverage totals
si odds / si cards / si defs                                       as in the tempo spec
```

## 8. Migration

Done 2026-10-20 (steps 1–2): the worktree is committed and the layout below is in place on `companion-v2` (`crates/si-common`, `si-steam`, `si-ui`, `si-app`, `si-backend`, `si-mcp`; `apps/web`; workspace `Cargo.toml` at the root; `scripts/si-live-start.sh`; `docs/si-live.md`; `docs/design-explorations`). Crate names are unchanged in this step; the carve-out into `si-model`, `si-rules`, `si-record`, `si-metrics`, `si-policy`, `si-cli` and the renames (`si-app` → `si-companion`, `si-backend` → `si-serve`) are step 3. The `si-corpus` crate from branch `worktree-agent-a8141e783fd8756a8` (C1 of the data-corpus design, 2 commits) lands as `crates/si-corpus` when that branch merges; it is the natural home of `si-defs`.

1. Commit the worktree (56 files, all from the companion sessions; Brett
   splits or approves the split). Nothing moves on top of uncommitted work.
2. New branch from fresh `origin/main` for the reorganization. `git mv`
   crate by crate so history follows; the workspace `Cargo.toml` moves to the
   root; `tools/si-live` stops existing. Build and tests green after each move
   (no behavior change in this step).
3. Carve `si-model` out of `common` (types only), then `si-rules` (the Invader
   phase first, since every log exercises it every turn), then `si-record`'s
   replay driver on one log, then the coverage report over all 33.
4. `si-metrics` from the tempo spec, on records.
5. Clients re-pointed; the native app keeps working on the same data.

The guide (`src/`, mdbook), `data/`, `skills/`, `scripts/`, `reports/`,
`research_notes/`, `docs/` stay where they are; the si-corpus design decides
`data/` later.

## 9. Decisions

All decided by Brett on 2026-10-20:
- A1 — the ten crates in Section 2.
- A2 — one repo: Rust at `crates/`, the Vue app at `apps/web`.
- A3 — commit the worktree as it stands (grouped commits, no split review).
- A4 — replay-and-verify only until coverage says the effects language covers the cards Brett plays; simulation and Rei-as-policy after.
- A5 (Brett, 2026-10-20) — hindsight analysis of a finished game: per turn, the actual play against the best play on what was knowable and the best play with the hidden cards known; mistakes separated from luck. Design in `2026-10-20-hindsight-analysis.md`; it is the first consumer of the simulate mode and sets the verification order (Invader phase → Spirit actions → search).
