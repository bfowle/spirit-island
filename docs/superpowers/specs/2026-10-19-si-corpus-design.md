# Spirit Island data corpus — design

Date: 2026-10-19
Status: rev 3, proposal (nothing built yet). Rev 2: no per-field review on printed facts. Rev 3: errata and box corrections live on the item they change; four layers (Brett). Decisions in Section 5 marked
**(Brett)** need an answer before milestone C1 starts.
Branch: to be decided (Section 5, D6).

## 1. Goal

Put everything the project knows about Spirit Island (cards, spirit panels,
aspects, adversaries, scenarios, boards, rules, errata, and how rules changed
across printings) into **one machine-readable corpus**. Every fact records
where it came from and whether Brett has checked it. Every product (guide
chapters, the Steam companion, the tabletop app, the Claude skills, the MCP
server) reads the corpus instead of keeping its own copy.

The corpus is also written to be easy for an LLM to use: one generated
markdown page per item, with its facts in front matter, so a session can
load exactly the spirit, card or rule it needs.

## 2. Why now

The 2026-10-18/19 sessions turned up the same failure again and again: a fact
copied by hand into one place, wrong, and then copied further.

- **Wrong data files:** `adversaries.json` (France L3, England L6, Habsburg
  Mining), `scenarios.json` (4 values), `adversary-stacks.json` (keyed by
  difficulty but read by level, so the tabletop app loaded the wrong deck
  order), and 8 scenario reference files.
- **Invented content:** the 8 `data/references/adversaries/*.json` files and
  the adversary chapters built from them (level names, effects, and a
  "Sweden/Russia suppress Fear" claim that is now in 11 files).
- **Rules inverted in prose:** the Build rule in `invader-lifecycle.md`; the
  Presence/Blight relation in `presence-economy.md`; Fear values in
  `fear-track.md`; the first Ravage on Turn 3 (it is Turn 2) in 4 spirit
  chapters, two skills and the opener generator; a "preview next turn's
  Event" flow in the tabletop app.
- **A wrong rule that spread:** "Fast innates can't count Slow cards'
  Elements" went from a memory note into `si-rules-check`, then into the
  Shadows, Finder and Shifting Memory chapters and their opener analysis.
- **Generators that would undo fixes:** `seed-adversary-levels.py`,
  `gen-adversary-chapters.py`, `gen-stats-chapters.py` and
  `gen-opener-starter.py` regenerate files from the old wrong sources.

One checked source, with generated outputs, stops this from recurring.

### Non-goals

- **Re-implementing the game engine.** The Steam engine already implements
  every card. The corpus describes effects; it does not simulate them. The
  engine is the oracle: captured Steam games are backtests (Section 10).
- **Encoding every card as executable logic up front.** Effect semantics are
  layered and built only as far as a consumer needs (Section 6, layer 4).
- **Strategy content.** Ratings, openers, matchups and tier lists stay in the
  guide. The corpus holds facts and rules; chapters cite them.

## 3. Current state (2026-10-19)

| Area | What exists | Source | Quality |
|---|---|---|---|
| Power Cards | `data/decks/{minor,major,unique}.json`: 100 / 78 / 153 cards with text, cost, speed, range, target, elements, threshold as text | Wiki scrape | Specifics often wrong (memory `feedback_wiki_scrape_unreliable`) |
| | Engine `power_cards.json`: 321 cards (101 Minor, 78 Major, 142 Unique), structured `elementOptions` thresholds | Steam DLL (local extract) | Good; Steam lags Nature Incarnate |
| Fear / Event / Blight | `data/decks`: 50 / 62 / 23 (Wiki); engine: 50 / 62 / 25 (23 + Tipping Point + "no card") | Both | Engine structured by section |
| Spirits and aspects | `data/spirits.json` (37 spirits, metadata only); `data/references/wiki/*.json` (39 pages); engine: 34 spirits, 24 aspects (tracks, growth, innates, special rules, setup) | Wiki, engine | Engine lacks 3 Nature Incarnate spirits (Wounded Waters Bleeding, Relentless Gaze of the Sun, Hearth-Vigil) |
| Adversaries | `data/adversaries.json`, `adversary-stacks.json`, `references/adversaries/*.json` | Rebuilt 2026-10-19 from the engine + Brett's answers | Good; the adversary chapters are still invented |
| Scenarios | `data/scenarios.json`, `references/scenarios/*.json` | Fixed 2026-10-19 | Values good; rules text not captured; Steam has 13 of 15 |
| Boards | `data/boards/*.json` (A–H, balanced + thematic); engine `boards.json` (A–H + 6 thematic) | Wiki, engine | Adjacency and setup present; thematic not cross-checked |
| Invader cards | engine `invader_cards.json` (15) | Engine | Good |
| Rules | `data/references/rules/register.md` (310 entries, Invader pipeline), `invader-rules.json`, `open-questions.md` (49 answered), `common/src/rules.rs` | Rulebooks, FAQ, engine, Brett | 39 reviewed by Brett, 29 sourced, 242 unreviewed; core rules outside the pipeline not captured |
| Errata, rule changes | Scattered in the FAQ (errata packs, NI's victory/defeat timing, the +1 blight errata) | FAQ | Not captured as data |
| Effect meaning | `needs.rs` tagger (damage, push, gather, defend, fear, isolate, strife…) | Our code | Partial, untested at scale |

Source documents on hand (memory `reference_rules_sources`): the four
rulebook PDFs (core, Branch & Claw, Jagged Earth, Nature Incarnate), the
official FAQ via `querki.net/raw/darker/spirit-island-faq/<Page>`, archived
Wiki pages, and the Steam engine definitions (`si-steam-dump defs`).

## 4. Source precedence

Brett's rule (2026-10-19, memory `feedback_printed_card_authority`):

1. **Explicit errata** (FAQ "Errata / Updates" entries, rulebook errata).
2. **The printed card or panel** (Brett's copy; the Wiki is usually a faithful
   transcription of it).
3. **Steam engine text.** A Steam wording that differs from the printed card
   without errata is recorded as a *Steam difference*, not adopted.
4. Everything else (community posts, our older files).

FAQ rulings that clarify without changing text (e.g. Jagged Earth p.7
"Ravage Timing") apply on top of the printed text.

## 5. Decisions

| # | Topic | Proposal |
|---|---|---|
| D1 | IDs | The engine identifier (`DarknessSwallowsTheUnwary`, `TheKingdomOfEngland`, `JungleC4`) is the canonical ID. Items Steam doesn't have yet get an ID in the same style, flagged `steam: absent`. |
| D2 | **Printed text in a public repo (Brett)** | The repo is public. Commit IDs, numbers, elements, our own tags, and our own one-line rule summaries with citations. Keep full printed card and rulebook text in a local layer (`data/corpus/local/`, gitignored) rebuilt from Brett's installed game and sources. Note: `data/decks/*.json` already commits Wiki card text; decide whether that stays. |
| D3 | Format | JSON per item kind, validated by JSON Schema (`data/corpus/schema/`). Rust types generated or hand-written to match; Python and TypeScript read the JSON directly. |
| D4 | LLM pages | Generated `data/corpus/pages/<kind>/<id>.md`: front matter with the facts, then a short body with citations. Never edited by hand. |
| D5 | Builder | A Rust binary `si-corpus` in the `tools/si-live` workspace (reuses `engine_defs`, the NRBF decoder and the JSON types): `build`, `diff`, `check`, `pages`. Wiki and FAQ fetching stays in Python scripts that write local caches only. |
| D6 | **Branch (Brett)** | Build on a new branch cut from a freshly fetched `origin/main` once `steam-companion` is merged, or stack on `steam-companion` if it isn't merged by then. |
| D7 | Review | Printed facts carry no review status; Brett only sees disagreements. The disagreement report (`data/corpus/review.md`) lists each field where sources differ, in the open-questions format; his answer becomes a correction on that item. Rules (layer 3) keep a review status. |

## 6. The corpus, in four layers

Each layer is useful on its own; later layers only add.

### Layer 1: identity

One record per item: `id`, `kind`, `names` (printed name, short name,
aliases), `expansion`, `printings` (where known), `steam` (present / absent /
differs). Kinds:

- power-card (Minor, Major, Unique), spirit, aspect, innate, special-rule,
  growth-option, presence-track
- adversary, adversary-level, escalation, loss-condition
- scenario, fear-card, event-card (and its sections), blight-card,
  invader-card, reminder-card
- board, land, token-type, element, terrain

### Layer 2: printed facts

Fields as printed, per kind. For example, a power card has `cost`, `speed`,
`range` (from Presence / Sacred Site / terrain), `target`, `elements`,
`thresholds` (elements → option), `options` (one per printed line or "Or"
choice). A spirit has growth options, both Presence tracks (values and
element/reclaim spaces), innates (speed, range, target, levels and
thresholds), special rules, setup, complexity. An adversary level has
title, difficulty, Fear-deck change, deck recipe and effects. A board has
lands, terrains, coast, adjacency and setup pieces for balanced and
thematic sides.

Printed facts get **no per-field review status** (Brett, 2026-10-19). A
printed number is right or wrong, not a matter of interpretation, and where
the engine and the Wiki agree it is accepted. Three light mechanisms
replace it:

- **A source tag per item:** `engine+wiki` (normal), `wiki-only` (Steam
  doesn't have the item yet, e.g. three Nature Incarnate spirits), or
  `box-only`. `wiki-only` items are the weaker ones.
- **Corrections on the item:** a field changed by errata or by the box is
  corrected on the item itself, with the value it replaces and a citation
  (Section 6.1):

  ```json
  "difficulty": 11,
  "corrections": [
    { "field": "difficulty", "was": 10, "by": "box", "steam": 10,
      "cite": "open-questions Q16" }
  ]
  ```

- **A disagreement report** (`data/corpus/review.md`), regenerated by every
  build: only fields where the sources differ and no correction settles
  them.

Review status stays where interpretation happens: the rules layer (layer 3).

### Layer 3: rules

Every rule as a structured statement, generalizing `register.md`:

| Field | Meaning |
|---|---|
| `id` | `R-…` (existing register IDs kept) |
| `statement` | Our own words, one or two sentences |
| `applies` | Base game, or the expansion or printing it arrived with |
| `changed` | Earlier versions with the printing that changed them (e.g. Victory/Defeat timing: core "immediately" → Jagged Earth "after each action" → Nature Incarnate "after each action tree") |
| `citations` | Rulebook page, FAQ entry URL, card |
| `review` | `reviewed by Brett (…)`, `sourced (…)`, or `unreviewed` |
| `tests` | Code tests or backtest cases that check it |

Scope: the four rulebooks section by section (turn structure, each phase,
Powers, Elements, Presence, Damage/Destroy/Remove/Replace, Fear, Blight,
Dahan, tokens, Isolate, Actions and action trees, Victory/Defeat), plus FAQ
rulings. Migrate `register.md` and the 49 answered open questions first.

### 6.1 Errata and box corrections live on the item

Errata is not a separate layer (Brett, 2026-10-19). It changes one item, so it
is recorded on that item, next to the text it changes, and the item's page
shows everything about it in one place:

`corrections: [{ field, was, by: errata|box, kind: text|rule|clarification,
printing, cite }]`

- **On a card, panel, adversary level or scenario:** Habsburg Mining L2 "on
  each board"; Breath of Darkness's Escape wording; England L6 difficulty 11
  (box); France's Town limit per player (box); Sea Monsters replaced in
  Jagged Earth; Nature Incarnate's dropped cards (Q15, as `status: dropped`
  with the replacement's ID).
- **On a rule, when it reaches many items:** the +1 blight setup errata;
  extra-board "per player → per board" (Blitz, The Great River, Second Wave,
  Powers Long Forgotten, Ward the Shores); Victory/Defeat timing. The rule's
  `changed` field records it, and each affected item lists the rule ID in
  `rules`.

The build also writes generated indexes for audits (`index/errata.md`: every
correction by printing; `index/steam-differences.md`: every place the box
or errata beats Steam). They are outputs, never edited.

### Layer 4: what effects do

- **Tier B, every effect option:** tags with amounts and limits:
  damage (to whom), destroy, push/gather (what, how many), defend, fear,
  isolate, skip (which action), add/remove token, strife, downgrade/upgrade,
  energy, card plays, elements, presence moves, repeat. Generated by a
  parser over the printed text (extending the `needs.rs` tagger), checked by
  invariants and spot review.
- **Tier C, only what the forecast needs:** executable descriptions of
  effects that change the Invader pipeline (the register's families:
  Defend, Ravage damage changes, skips, Isolate, tokens, health). Each is
  checked against the Steam engine with backtests.

## 7. Layout

```
data/corpus/
  schema/            JSON Schema per kind (committed)
  core/              layer 1–2 facts and tags, one file per kind (committed)
  rules/             layer 3 rules, with their changes over time (committed)
  review.md          generated disagreement report (committed)
  index/             generated errata and Steam-difference indexes (committed)
  pages/             generated LLM pages (committed, never hand-edited)
  local/             full printed and rulebook text, caches (gitignored)
```

## 8. Builder (`si-corpus`)

1. **Import:** engine definitions from the installed DLL; Wiki card and
   spirit data from the local cache; the FAQ errata pages; the corrections
   already recorded on items.
2. **Merge:** per field, apply the precedence in Section 4 (corrections
   first), tag each item's source, and record any Steam difference.
3. **Diff:** every field where sources disagree and no correction settles it
   goes to `review.md`.
4. **Index:** write `index/errata.md` and `index/steam-differences.md`.
5. **Check:** schema validation and invariants (Section 10).
6. **Pages:** write `pages/` and the consumer exports (Section 9).

Rerunning after a Steam update or a new Wiki fetch shows exactly what
changed.

## 9. Consumers and migration

| Consumer | Reads today | Switches to |
|---|---|---|
| Steam companion (`si-steam`, `si-common`) | Engine definitions at runtime; `invader-rules.json` | Engine for decoding saves (unchanged); corpus for tabletop-correct facts and rules (e.g. box difficulty, printed Accumulated Devastation) |
| `rules_register.rs` | `invader-rules.json` | Corpus rules layer |
| Tabletop backend (`/api/registry`, `/api/deck/:kind`, `/api/spirit/:slug`, `/api/invader-stack`) | `adversaries.json`, `scenarios.json`, `spirits.json`, `boards/`, `decks/`, `references/wiki/`, `adversary-stacks.json` | Corpus exports in the same shapes first, then the corpus directly |
| Guide chapters | Hand-written facts; generators reading old files | Generated fact tables and boxes from the corpus; prose cites rule IDs |
| Generators (`seed-*`, `gen-adversary-chapters`, `gen-scenario-chapters`, `gen-stats-chapters`, `gen-opener-starter`) | Old, partly invented sources | Rewritten to read the corpus, or retired |
| Skills (`si-rules-check`, `si-at-the-table`, …) | Prose and `spirit-mechanics.md` | Corpus pages |
| MCP server (`si-mcp`) | `data/current-game.json` | Adds corpus lookups (card, spirit, rule by ID) |

`data/references/spirit-mechanics.md` and the Wiki-scraped fields that the
corpus supersedes are retired once nothing reads them.

## 10. Validation

- **Schema:** every file validates.
- **Invariants:** pool sizes (100 / 78 / 50 / 61 + Slave Rebellion / 23
  after dropped cards); 15 Invader cards (4 / 5 / 6 by stage); every adversary
  has levels 0–6 with a deck recipe that yields a legal deck; every board has
  symmetric adjacency; every spirit has two tracks and at least one innate;
  every threshold uses real elements.
- **Cross-source:** engine vs Wiki vs box per field (the disagreement report).
- **Backtests:** each captured Steam game replays through the forecast;
  every mismatch is either a new rule (layer 3) or a bug. Test 1 produced 6
  rules this way.
- **Chapter checks:** a lint that flags numbers in chapters that disagree
  with the corpus (costs, difficulties, Fear values, first Ravage turn).

## 11. Milestones

| # | Milestone | Done when |
|---|---|---|
| C0 | Decisions D2 and D6 | Brett answers |
| C1 | Layers 1–2 from the engine + Wiki, corrections on items, disagreement report | `si-corpus build` runs; the report lists every unsettled disagreement; Brett's existing answers are corrections on their items |
| C2 | Consumers switch | Tabletop registry and decks, adversary and scenario chapters regenerated from the corpus; old generators rewritten or retired; the 11 "Fear suppression" files fixed by regeneration |
| C3 | Layer 3 and errata | `register.md` and `open-questions.md` migrated; core rulebook sections imported; FAQ errata recorded on their items and rules; chapter lint running |
| C4 | Layer 4 tier B | Every effect option tagged; the need finder and combo chapters read the tags |
| C5 | Layer 4 tier C + games | Pipeline effects described and checked against captured games from the matrix in Section 12 |

C1 is about one to two sessions; C2 and C3 about one each; C4 and C5 grow
with use.

## 12. Games to capture (after C1)

Each Steam game is a backtest. Play these to test rules no game has tested
yet:

| Game | Tests |
|---|---|
| England L1–L3 | Builds in empty lands next to 2+ Towns/Cities; High Immigration |
| Brandenburg-Prussia L2+ | The moved Stage III card's stage (NQ-P) |
| Russia L5 | Invader cards inside the Fear deck |
| Sweden L4 | Accelerate at setup; the discarded card is visible |
| Blitz | The setup card Ravages on Turn 1 (NQ-J) |
| A Spread of Rampant Green (Tangles) or another Isolate source | Isolate in the Explore forecast |
| Ocean's Hungry Grasp | Range through Oceans (Q29) |
| Two spirits | Fear per player, multi-spirit decisions |
| An extra board | Per-board blight and fear changes |

## 13. Risks and open questions

- **Public repo and printed text (D2).** Decide before C1.
- **Steam lags the box.** Three Nature Incarnate spirits and two scenarios
  are missing; the corpus fills them from the Wiki and the box, marked
  `steam: absent`.
- **The Wiki is a secondary source.** Its card data is often wrong in
  specifics; it is used as a cross-check and a printed-text transcription,
  never alone for numbers.
- **Aspect count.** Steam has 24 aspects; the box count is unchecked.
- **Tier B parsing at scale.** The tagger will mis-tag unusual wordings;
  invariants and backtests catch some, spot review the rest.
- **Two sources of rules in code.** `rules.rs` encodes rules as code;
  layer 3 records them as data. Each rule in code cites its corpus ID, and a
  test fails if a cited ID is missing.
