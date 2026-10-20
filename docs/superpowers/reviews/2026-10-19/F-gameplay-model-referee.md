# F. Gameplay-model referee: the Rust conceptualization of Spirit Island

Scope: `tools/si-live` in `/home/brett/repos/spirit-island-steam-companion` (crates `common`, `steam`; `backend`/`app` only where they touch the model). Read in full: `common/src/public.rs`, `rules.rs`, `rules_register.rs`, `companion_stats/{mod,invaders,adversary,backtest,whatif,review,needs,outlook,ledger,spirits}.rs`, `steam/src/{engine_map,projection,engine_defs,companion,log_parse}.rs`, `steam/tests/{backtest,firewall}.rs`, `common/data/invader-rules.json` (structure and counts), the design spec §3–5, 7.8, 7.11, 14, the corpus and questions-from-play specs, the six playtest "Rules and forecast" tables, `data/references/rules/{register,open-questions}.md`, and Brett's messages. Nothing was modified.

---

## 1. The game as the code conceives it

### Entities and granularity

The model is **one `PublicSnapshot` per save file** (`common/src/public.rs`), produced by `steam/src/projection.rs::project` from an `EngineSnapshot` (`steam/src/engine_map.rs::map`) plus card/spirit definitions pulled from the game DLL at runtime (`steam/src/engine_defs.rs`). Everything downstream is a pure function of that struct.

| Entity | Type | Granularity | Notes |
|---|---|---|---|
| Island | `PublicBoard{id, lands}`, `PublicSnapshot.adjacency: Vec<(id,id)>` | land ids, undirected edges incl. Ocean | Engine `LandDistanceCache` **is decoded** (`engine_map.rs::map_distances`) but **not projected**; distance is re-derived by BFS in `needs.rs::distance`. |
| Land | `PublicLand` | per land: `terrain` (string), `is_ocean`, `coastal`, counts `explorers/towns/cities/dahan/blight`, `presence: BTreeMap<spirit,u32>`, `tokens: BTreeMap<name,u32>`, `pieces: Vec<PublicPiece>`, `damage_taken`, `defend`, `health_modifier` | Counts and pieces are stored twice (counts + per-piece list) and must be kept in sync by hand (`rules::add_built`, `whatif::apply`, `adversary::after_build`). Incarna is folded into `presence` (`projection.rs:83`), so an Incarna looks like a Presence to Sacred-Site and Blight logic. |
| Piece | `PublicPiece{kind, max_health, damage, strife}` | per Invader/Dahan/Thief | `kind` is a string. Health includes engine `HealthModifiers` (England L5, Habsburg L4). |
| Spirit | `PublicSpirit` | energy, card plays, phase status, elements (cards + tracks), hand/play/discard/forgotten/revealed as `PublicPowerCard`, `track: Vec<TrackNode>`, presence on island/destroyed, `innate_powers: Vec<PublicInnate>` (thresholds + body text), `special_rules` text | **No growth options** are projected although the spirit defs contain them (corpus spec §3). No Sacred Site concept beyond `presence ≥ 2` in `invaders.rs::choke`. |
| Power card | `PublicPowerCard{cost, speed, kind, elements, range, from_sacred_site, target, body: Vec<String>}` | printed facts + text | Effects exist only as text. `needs.rs::tag_options` scrapes a dozen keywords from the body. |
| Invader deck | `PublicInvaderDeck{upcoming_stages, explore, build, ravage, discard, extra_slots, pool}` | card backs (stage) for face-down; `PublicInvaderCard{stage, escalation, terrains}` for face-up | `pool` is the rules pool so odds are pool − seen. Russia L5 cards in the Fear deck are not visible as such (only `FearCard` class is counted there). |
| Fear | `PublicFear{generated, required_per_card, terror_level, earned_count, deck_by_level, discard, resolving, removed_unseen}` | counts; card text only once resolved | **No unseen Fear pool** (engine `allowed_fear` is decoded but not projected). |
| Events | `PublicEvents{deck_count, shown, discard}`; `PublicEventCard{sections[types, body]}` | text by section once face up | **No unseen Event pool** (`allowed_event` decoded, not projected). The reveal is captured from the log (`PublicDecision.event`) before the save shows it. |
| Blight card | `PublicBlight{flipped, remaining, on_island, healthy_side, blighted_card, blighted_text, unseen_flipped, unseen_still_healthy, current_still_healthy, pool: Vec<BlightOption>}` | counts + pool of flipped-side totals | The one place a hidden-card *distribution* is modeled explicitly. Flip "Immediately" effects are text only (E7). |
| Adversary | `AdversaryRef{id, level, panel: BTreeMap<kind,u32>}` | id + level + panel pieces | Rules are code in `companion_stats/adversary.rs`, keyed by id string constants; text and review status in `invader-rules.json`. |
| Scenario | `scenario: Option<String>` | id only | Only Blitz has code (`adversary.rs::Adv::blitz`, `spirits.rs`). |
| Status effects | `PublicStatusEffect{kind, source, source_kind, params: BTreeMap<String, StatusParam>}` | engine class name minus "StatusEffect", raw engine member names as params | This is the model's real "effect semantics": the engine's own modifier objects, interpreted by `rules.rs` (Defend, AdjustRavageDamage, CancelAction, ChangeRavageParticipation, SkipInvaderActions, PreventBlightCascade, IgnoreToken, Isolate, ModifyExploreRange, AdjustHealth). Params cross the firewall only when the source is public. |
| Decisions | `PublicDecision{turn, step, kind, text, land, card, ms, event}` | whitelisted log lines | Used by `review.rs` for "actions here" only. |
| Ending | `ending: Option<String>` | engine enum name | 11 ending kinds known (`engine_map.rs::ENDING_RESULTS`); only `LoseTooMuchBlight`, deck-out and adversary clocks have any forward model. |

### Transitions the code can play forward

All in `common/src/companion_stats/invaders.rs::forecast_at` with hooks into `adversary.rs` and rules in `rules.rs`:

1. **High Immigration** (England L3+): `build_card` on the card in `extra_slots` before the Ravage; non-normal Build; shares one-use cancels.
2. **Ravage** per matching land with Invaders (`adversary::ravages_in`, incl. Russia L3): `rules::ravage_ledger` computes per-invader damage with Strife, `AdjustRavageDamage` (per-invader / per-land / per-damage-taken), Defend (flat, per-piece, spirit-presence, adjacent-Dahan, token-scoped via `for_land`), participation removals (Temporary Truce), Dahan protection (Concealing Shadows), Ravage cancels (Disease/Strife via `CancelAction`), skips, `PreventBlightCascade`; then Dahan destroyed (efficient assignment), counterattack 2/Dahan, best-case kill set by Fear (`best_destroy`), `counterattack_clears`; `adversary::after_ledger` adds Heavy Mining, Runoff, Avarice, Russia Beast/Explorer notes, Irreparable-Damage counting.
3. **After Ravage**: counterattack-clears empty the land on the forecast board; `adversary::after_ravage_step` (Scotland L6 applied; Russia L6 applied when untied; Sweden L5 listed); Vengeance Lingering Pestilence adds Disease.
4. **Build**: each card in `inv.build` in order (Entrench card first, normal last); lands fixed on the board before the card (FAQ); England L1 / Scotland L3 empty-land builds; `BuildCancels` with uses (whole-card, lowest-numbered, per-land with `targetTypes`/`islandPieceTypes`); Disease stop (spent), Vengeance choice, Choke; `adversary::after_build` (Habsburg L2, France L2, France L4); Ceaseless Mining diverts the card to Ravages in Mining lands.
5. **After Build**: Migratory Herders (applied when one source), Mining Boom (listed).
6. **Explore**: candidates = revealed card or unseen cards of the next stage; sources = Ocean or Town/City within `explore_range` (BFS not through Ocean), minus Isolated, minus Wilds; per-land `(hits, candidates)`; adversary notes (Scotland L1, France L1/L6, Habsburg Mining L6, Blitz).
7. **Escalation** on flagged Stage II cards: England applied as Builds; Sweden applied per explored land; Prussia/France/Livestock/Russia/Scotland/Mining listed as options.
8. **Advance + Time Passes** (`advance`, `time_passes`): adds this turn's Ravage Blight (no cascade placement), adds Explorers only if the Explore card is face up, clears Defend/damage, drops `fear|event|power` effects, slides cards, pops one stage. **Does not** remove destroyed Dahan, counterattack kills short of a full clear, or Presence; does not run Spirit, Fast, Event, Fear steps.
9. **Loss clocks** (`adversary::loss_clocks`) for England, France, Scotland, Mining (end-of-Fast-phase check, value after this phase), Russia (panel count), Livestock (panel count, unverified).
10. **Enumeration** (`outlook.rs`): 4 Invader Phases over every order of unseen cards grouped by (stage, terrains, escalation), no Spirit action, fewest cascades.
11. **What-if** (`whatif.rs`): four edits (Destroy, Defend, +Dahan, Track reveal) applied to a snapshot copy; Fear earning with Terror Level roll-over.

### The hidden/public boundary

Rule (spec §5, implemented in `projection.rs`): a value crosses only if a player at the table could know it. Concretely: deck orders and identities never cross (stages from card backs do); earned Fear is a count; Events/Fear cards show text only once face up or in a discard; Minor/Major draw pools are `allowed − seen` (with a warning if that disagrees with the deck size, `projection.rs:194`); the blight card is its healthy side plus the pool of cards sharing that side; status-effect `params` and `source` cross only when the source is a public card/rule (`projection.rs:275`), otherwise only the class name; decision card names cross only for `PUBLIC_CARD_REASONS`. The seed is hashed to a game id. What is **dropped though public**: land distances, allowed Fear/Event pools, growth options, the Blight card's `allowed_blight` as a list of names (kept as anonymous `BlightOption`s, which is correct).

### The turn as the code sees it

```
engine Constants.StateOrdering (invader-rules.json "state_ordering"; companion.rs STATE_ORDER adds Grow/GainEnergy/PlayPayPower):

 PlayerSetup → InvadersStartingAction
   → SpiritPhase[Grow, GainEnergy, PlayPayPower]   (no model: growth options not projected; whatif::Track only)
   → FastPower                                      (no model: plan = what-if edits; needs.rs counts cards that "save" a land)
   → BlightedIsland                                 (flip effects: text only)
   → Event                                          (seen via log; semantics only through resulting engine status effects)
   → Fear                                           (banked count; card text once resolved; effects via status effects)
   → HighImmigration ─┐
   → Ravage           │  forecast_at(): position "before" | "ravage" | "build" | "explore"
   → Build            │  ravage_ledger / build_card / explore candidates / escalation / rule_actions
   → Explore         ─┘  loss_clocks(after_build) · ledger() · needs() · outlook() (×4 phases)
   → AdvanceInvaderCards ─┐ phase_position() = "after": forecast is for turn+1 via time_passes()
   → SlowPower            │ (next_turn() = advance(): blight stays, Explorers added if card face up,
   → TimePasses          ─┘  cards slide; Dahan losses / partial counterattack kills NOT applied)
```

Caveat on the diagram: `backtest.rs:90` records that the `AdvanceInvaderCards` save is written **before** the cards move, yet `invaders::phase_position` maps that step to `"after"`, so `forecast_at` treats the still-unmoved slots as next turn's cards for the duration of that step (the just-resolved Ravage card forecast as next turn's Ravage). Transient, but it is a wrong forecast whenever the companion is viewed at that step.

---

## 2. Coverage matrix

Status key: **V** modeled and verified against the engine (row cited); **M** modeled, unverified; **P** partial; **N** not modeled. "Data" says whether the save/defs expose what is needed.

| Mechanic | Status | Where | Evidence / what is missing | Data available? |
|---|---|---|---|---|
| Ravage damage per Invader, Defend reduces land and Dahan, blight at health | V | `rules::ravage_ledger` | Test 1 T3 (Jungle E3), Q4; `defend_two_saves_the_land…` test | yes |
| Dahan efficient destruction, counterattack 2 each, excess not carried | V (destroyed) / **P** (kills) | `ravage_ledger`, `best_destroy` | Q4; kills are "best case by Fear" — the engine asks the player (`DealDamageActivation`); backtest never checks kills | log has the assignment (`RawDecision::Damage`) |
| Pre-existing damage carried into counterattack | V | uses `max_health − damage` | Prussia 6 R4 | yes |
| Cascades (yes/no), cascade cluster, fewest extra | V flag / **P** target | `ravage_ledger.cascades`, `mod.rs::cascade_min_extra` | Q5; target is the player's pick: `advance()` never places it; backtest excludes cascade targets (`backtest.rs:112`) | log has the choice |
| Presence destruction by Blight | **P** | `presence_at_risk` = 1 per spirit per blight | not applied to the forecast board; not counted in `advance`; cascades' Presence loss ignored; no `LoseSpiritDestroyed` clock (U18 asked for one) | yes |
| Blight card flip, Still-Healthy redraw, P(loss) over the pool | V/M | `projection::blight_pool`, `mod.rs::p_loss` | Q11, Q12; U16 76% reproduced in `loss_risk_through_still_healthy_chains`; **flip "Immediately" effects N** (England E7) | text only |
| Fear earning (destroy Town 1 / City 2), Terror Level roll-over | M | `rules::destroy_fear`, `whatif::fear` | Q6, Q7; `whatif` test; Fear from Powers/Events/Escalations not modeled; Fear per card read from save (scales with players/boards) | yes |
| Fear card content | **P** | `PublicFearCard.body` once resolved; `fear_card_families` tags in JSON (10 families, not loaded by code) | banked cards: count only; no unseen pool in `PublicFear`; resolution effects arrive only as engine status effects after the fact | `allowed_fear` decoded, not projected |
| Fear/Terror victory condition | V (blockers) | `rules::terror_victory_blockers`, `pace.rs::BoardRoute` | Q6; "Victory when revealed" (deck empty) in `pace.rs` marks | yes |
| Events: T1 discarded, reveal timing | V | `log_parse::EventRevealed`, `projection::decisions` | Prussia 6 R5/U7 | yes |
| Event content/choices | **P** | `PublicEventCard.sections` (types, body); 93 register entries with hand-written `params` (`invader-rules.json` "events") | no executable semantics; register entries are display-only (`rules_register::APPLIED` excludes them); choices (`RawDecision::Effect`) recorded, not forecast | text yes; pool not projected |
| Strife (0 damage, loses 1) | V | `ravage_ledger` | Test 1 T4 (Sands C2), Q25 | yes |
| Disease stops Build (spent) | V | `rules::build_outcome_with` | Test 1 T5, Q26; E4 spent-token fix | yes |
| Wilds stops Explore | V | `rules::wilds_stop_explore` | Test 1 T2 | yes |
| Isolate | M (untested) | `rules::isolated_lands` (heuristic: `kind.contains("Isolate")`) | Russia G5: never fired | yes |
| Beasts | **P** | counted (Russia clock, Stalk the Predators); flagged `unmodeled` in Ravage | no Event/Fear interaction | yes |
| Badlands, Vitality | N (flagged) | `ravage_ledger` pushes to `unmodeled` | Badlands +1 damage rule not applied | yes |
| Power cards: printed facts (cost, speed, elements, range, SS, target) | V-by-defs | `projection::power_card` | 321 cards from the DLL | yes |
| Power card effect semantics | **P** | `needs.rs::tag_options`/`reduction` keyword tagger | no resolution onto a board (no push/gather targets, no range for pushes, no element gain for drafted cards into innates, no Fear/Blight-remove application); not tested at scale | text only in defs (`body`); no structured effect data |
| Innates and thresholds | V-by-defs | `PublicInnate.levels`, `spirits.rs::spirit_stats` (met now / cheapest hand set) | thresholds from `elementOptions`; Blitz speed; effects text only | yes |
| Growth options | **N** | — | `TrackNode` projected; growth options exist in defs but not in `PublicSpirit`; `common/src/growth.rs` is the old Wiki parser used only by `schema.rs` | yes (defs) |
| Presence tracks | V-by-defs | `TrackNode{kind,value,elements}` | `track_value`, `Edit::Track` | yes |
| Explore sources, range 1, Oceans, Isolated, +range Events | V | `rules::explore_sources_within`, `explore_range` | Q3; Russia game (Distant Exploration) | yes |
| Build piece (Town/City), England L1 empty-land builds | V | `rules::build_piece`, `builds_without_invaders` | Q2; England E1 | yes |
| Multiple Build/Ravage cards (Entrench) | V | `build_card` loop | Russia G2 | yes |
| England | Esc **V**, L1 **V**, L3 **V**, L2/L5 state, L6 **N** (note only) | `adversary.rs`, spec 7.11 | England game E1–E3 | yes |
| Sweden | Esc **V**, L1 **V** (amount checked), L3 **V**, L5 options, L2/L4/L6 state | | Sweden W1–W5 | yes |
| Russia | Esc options **V**, L1 **V**, L3 **V**, L2 note, L6 **M**, clock **V** | | Russia G3/G4 | yes |
| Prussia | Esc options **V**, L1–L6 state; L2 Stage III early **V** | | Test 1, Prussia 6 R2, Blitz B5 | yes |
| France | all **M**/options; loss clock M; L6 "Fear never removes Explorers" **N** | | no game | yes |
| Scotland | all **M**/options; clock M | | no game | yes |
| Habsburg Livestock | L1/L2/L6 **M**; L4 state (unverified); L5 **N**; clock unverified | | no game | yes |
| Habsburg Mining | Esc **V**, L1 Ceaseless **V**, Avarice **M**, L3 **V**, L5/L6 M, clock **V** | | Habsburg H1–H5 | yes |
| Scenarios | Blitz: 3 rules (**V** T1 Ravage, extra Explorer as option, all-Fast); 12 others **N** | `Adv::blitz`, `spirits.rs` | Blitz B1/B2/B7 | ids yes; rules text in JSON |
| Expansions (B&C, JE, F&F, Horizons, NI) | implicit | pools come from `allowed_*` lists | no expansion field; NI partly absent from Steam (3 spirits, 2 scenarios); Incarna folded into presence | yes via allowed lists |
| 1 vs 2 vs N spirits and boards | **P** | `Adv.players = spirits.len()`, Fear per card from save, `initial_boards` for blight totals, loss limits scale | `multiplayer` only warns; no per-spirit reach/plan interplay; `stage_reach` averages per board | yes |
| Thematic maps | V (decode) | land ids like `WetlandNW3` map; adjacency from the matrix | Habsburg game H6 | yes |
| Ocean's Hungry Grasp | **P** | range through Ocean in `needs.rs::distance` (Q29) | cascade into Ocean, drowning, Ocean presence not modeled (Q5) | yes |
| Spirit special rules affecting the pipeline | **P** | 3 modeled (Green Choke, Vengeance ×2) hard-coded in `invaders.rs`; register lists ~50 for 34 spirits | | yes (status effects + defs text) |
| Energy/card-play economy per turn | M | `curve.rs`, `pace.rs`, `RoundSummary.energy_income/plays` | income from revealed track max; no growth-option branching | yes |

---

## 3. Fitness for the questions

### Growth (5 asks)
- **Today**: per land, the Ravage ledger and the exact Defend/damage margin (`ledger.rs::ravage_spend`); of the unseen Minors/Majors, how many hold an option that "saves" the land and reaches it at its speed (`needs.rs::spirit_needs`); pace and curve.
- **Missing**: (a) the growth options themselves as data (`PublicSpirit` has no growth list; what a G-option gives in Energy/plays/presence/cards cannot be enumerated); (b) a what-if "take option X" that yields the hand after the gain (unknown card → a distribution over the unseen pool, which `needs` partly does) and the track node revealed; (c) banked Fear cards and the Event as counted unknowns (no `PublicFear.unseen`, no `PublicEvents.unseen`).
- **Smallest change**: project `defs.spirits[id].growth*` into `PublicSpirit.growth: Vec<PublicGrowthOption{cost/choices, effects as tagged tokens}>`; add `Edit::Growth{spirit, option}` that reveals the track node and credits Energy/plays; project `allowed_fear − fear_discard − removed` as `PublicFear.unseen: Vec<PublicFearCard>` (3-level body) and `allowed_event − event_discard` as `PublicEvents.unseen`.

### Draft and Forget (5 asks)
- **Today**: per candidate card: printed facts, elements, which innate thresholds the hand can reach (`spirits.rs::reachable_with`, hand only), which Ravage lands it could save by keyword (`needs::reduction`), reach by range/target.
- **Missing**: resolving a card on the forecast board ("what would this card accomplish this turn"): Push/Gather need a destination/source land and adjacency; Damage needs an assignment; Fear needs Terror arithmetic; Defend only in Fast; speed vs the step reached; element gain of the *candidate* toward innates (not just hand); Forget cost = elements lost + Fast protection lost + Fear lost, needs the same tags applied to the discard.
- **Smallest change**: promote `needs::EffectTags` to a `PublicPowerCard.effects: Vec<EffectOption>` field filled at projection from the text tagger (tier B of the corpus spec), then an `Edit::Play{spirit, card, land, option, assignment}` in `whatif.rs` that applies Defend/Damage/Destroy/Push/Gather/Fear/Strife/Isolate/token adds to the snapshot; `spirits::spirit_stats` already computes thresholds from `sp.elements`, so a drafted card's elements can be added by a `with_card` variant. Evidence requirement: tags come from printed text (defs `body`), checked against physical cards; no engine method needed because the engine never exposes structured effects.

### Fast plan vs this Ravage (3 asks)
- **Today**: closest to answered. `whatif::apply` + `compute` recomputes every ledger with Destroy/Defend/Dahan edits; `ledger` gives shortfall; `p_loss` the depth risk; `rules_in_play` the explanations; Sweden/Russia modifiers named in `applied`.
- **Missing**: Push/Gather/Isolate/Strife/Skip edits; Dahan movement; Presence destruction into `advance`; the counterattack's actual assignment; Fear earned by the plan's destroys only (not by card Fear); hidden-help counts for the Fear step (no Fear pool).
- **Smallest change**: extend `Edit` with `Push{land, to, kind}`, `Gather{land, from, kind}`, `Isolate{land}`, `Strife{land, kind}`, `Skip{land, step}`, `RemoveBlight{land}`, `Token{land, kind, ±n}`; have `apply` also update `pieces` consistently (today Destroy removes the first piece of a kind regardless of damage/strife).

### Event and Fear choices (2 asks)
- **Today**: the engine's resulting status effects are applied once the card is face up and public (R5 confirmed Defend arrives at the Ravage save). Choices are logged.
- **Missing**: forecast *before* resolution: the sections' legal choices (push which Dahan, place Beasts where) as a set of what-if branches; and "what the unseen Events' token sections would do here" needs the Event pool + section tags.
- **Smallest change**: Event section tagging (same tagger, `eventSectionTypes` already give the gate), `PublicEvents.unseen`, and reuse `Edit`s for each legal choice.

### Retro
- **Today**: `review.rs` attributes Blight by step, builds Invader chains with Fast/Slow points, flow added/removed, pressure jumps, decisions per turn. Verified on Test 1 (`review_attributes_all_test1_blight`).
- **Missing**: `review::invader_damage` is a local 1/2/3 formula ignoring Strife and adversary damage; cause attribution lacks "Blight card"/"Event" (R8); no luck ledger (realized vs expected Explore/draw); no per-decision replay of the forecast at decision time (needs snapshot↔decision alignment: `PublicSnapshot` carries no capture time, `PublicDecision.ms` does).
- **Smallest change**: add `captured_ms` to `PublicSnapshot`; use `rules::ravage_ledger` in review instead of the local formula; record `outlook_check` per turn as the luck ledger (it already exists in `backtest.rs`).

### The competing-goals race (Blight loss, adversary losses, deck exhaustion, Presence loss, Fear win, board-clear win)
- **Today**: `ledger.rs` lays out Blight, Fear (goal), deck, and adversary clocks as budgets; `outlook.rs` gives `flipped_by`/`lost_by` per turn and `deck_out`; `pace.rs` gives Fear marks and board-clear blockers per Terror Level with arrival turns at the recent pace.
- **Missing**: Presence loss has no clock at all (`LoseSpiritDestroyed`); adversary clocks are evaluated only one phase ahead and only on the `after_build` board (not inside `outlook`'s 4-phase enumeration, so per-land counts across paths for England's 7-in-one-land or Mining's 8 are not available); board-clear is a count, never a forward model; Fear win is a pace extrapolation, not a count over what the forecast destroys; no joint view (which ending comes first under the no-action path).
- **Smallest change**: compute `loss_clocks` and `presence_on_island` inside `outlook::Walk::step` for every path (the data is already on `view.after_build`), and add a `PresenceClock{spirit, on_island, at_risk_this_phase}` built from `presence_at_risk`.

---

## 4. Structural judgments

**Snapshot per save vs state + event log.** The snapshot-per-save unit is right for the firewall (one projection point) and for backtests (diffs between saves). It is wrong as the *only* unit for retro and what-if: the timeline has no timestamps, no causal edges, and a step can hold several saves (undo). Decisions live in a separate list keyed by `(turn, step, ms)`. Recommendation: keep snapshots as truth, add a derived event log (`changes.rs` is halfway there) with `captured_ms`, so review/luck/what-if-at-decision can align.

**Is a new rule one place to add?** Mostly yes for adversaries: `adversary.rs` is organized by hook (`ravage_effects`, `land_effects`, `ravages_in`, `after_ledger`, `builds_without_invaders`, `after_build`, `after_ravage_step`, `after_build_step`, `escalation`, `explore_notes`, `loss_clocks`). But (a) spirit rules are hard-coded in `invaders.rs` (`choke`, `VENGEANCE`, `vengeance_boards`) rather than in a spirit plug-in; (b) the register's `applied` flag (`rules_register::APPLIED` = deck_recipe, fear_deck, piece_health, setup_pieces) **contradicts** `adversary.rs`: Sweden L3 Fine Steel and England L1 are applied by code yet reported `applied: false` in `rules_in_play`; (c) England L6 is a note string in `forecast_at`; (d) Blitz is tested by string in two files. Two sources of truth for "applied".

**Duplicated logic.** `review::invader_damage` vs `rules::invader_damage` + ledger (review ignores Strife/adversaries); `adversary::invaders` vs `rules::has_invaders`; `adversary::adjacent` vs `rules::adjacent_towns_cities`; `land()` lookup helpers in `review.rs`, `backtest.rs`, `adversary.rs`; `needs::distance` BFS vs the engine's decoded-but-dropped distance cache; `Still-Healthy` detection by `blighted_text.contains("draw a new Blight Card")` in `mod.rs` (×2) and `ledger.rs`, while `PublicBlight.current_still_healthy` already exists; Fear-per-card counting in `whatif::fear` vs `mod.rs::fear_stats`.

**Strings as enums.** Step names (`"Ravage"`, `"SpiritPhase"`, 44 occurrences across crates), piece kinds, token names, terrains, `position`, `explore_access`, badges, `RuleAction.when`, `LossClock.trigger` ("at"/"over"), `source_kind`, status-effect kinds and **param keys** (engine backing-field names that already differ across the three engine builds; the heuristic `k.to_ascii_lowercase().contains("isolate")` in `isolated_lands` is the symptom). Bugs it has caused or risks: the `AdvanceInvaderCards` mis-mapping above; `whatif::apply` Destroy decrements a count and removes "the first piece of that kind" independently (count/pieces drift if a kind string is misspelled by a caller); `review::cause` and `backtest` label parsing by `split(' ')` on "Land +Piece" strings. A `Step`, `PieceKind`, `Terrain`, `TokenKind` enum in `public.rs` with serde would remove a class of silent mismatches.

**Firewall testability.** Enforcement 2 (snapshot permutation invariance) is implemented and strong (`steam/tests/firewall.rs::public_output_is_invariant_to_hidden_permutations`, with a weakness check). Gaps vs spec §5: it checks `project()` only, not `CompanionStats` (spec says stats must be byte-identical too — trivially true since stats are a pure function of the snapshot, but untested); Enforcement 3 (log invariance, zone table) does not exist — `log_parse` is a whitelist instead; the tests skip without Brett's local capture and DLL, so CI proves nothing on another machine.

**Undo and multiple saves per step.** `companion.rs::Companion::push` orders by `(turn, step_rank)` and drops later snapshots on a backward key, keeping equal keys. An undo *within* the same step is indistinguishable from progress: stale and corrected snapshots coexist, and `backtest::first(timeline, t, step)` picks the **first** one. `log_parse` reconciles undo by checkpoint write time with a 2 s lag window (`CHECKPOINT_LAG_MS`), a heuristic that worked in Test 1 and Prussia 6. The timeline is trustworthy for step-boundary diffs and brittle for intra-step history.

**Scaling to what-if and multi-turn play-forward.** `forecast_at` is already a pure function of a snapshot, so what-if composes (`whatif::apply` → `compute`). Limits: `advance()` is invader-only and drops Dahan deaths and Presence loss, so a 2–4 turn path over-counts counterattacks and never loses a spirit; `outlook` branches only on card kind and clones the whole `PublicSnapshot` per branch (fine at 4 phases, not at Spirit choices); counts and `pieces` must be mutated together everywhere; `Edit` has no cost model (Energy, card plays, speed window), so a "plan" cannot be rejected as illegal; the engine's `LandDistanceCache` is not projected, so Range queries are re-derived per call.

---

## 5. What the backtests prove and don't

`common/src/companion_stats/backtest.rs::backtest` compares, per completed Invader Phase, the forecast made at the **Ravage-step save** against the Build/Explore/Slow saves: (1) the set of lands that gained Blight among forecast Ravage lands, excluding cascade neighbors and lands with an available skip; (2) Blight **amount** in predicted lands (Heavy Mining); (3) High Immigration builds; (4) Builds as "Land +Piece" sets, minus Mining Boom upgrades and skips by choice; (5) Escalation builds/swaps/Explorers per adversary shape; (6) Explore lands for the card actually drawn. `steam/tests/backtest.rs` runs this for Test 1 only (6 phases, 0 mismatches); the other six games are checked by hand with `si-steam-dump backtest` (playtest docs state 0 mismatches on all seven). `outlook_check` confirms the realized card was among the enumerated rows and its Explore lands match.

So the backtests validate the **deterministic invader resolution given the revealed card and the public status effects at the Ravage save**: matching, Explore sources and range, Build piece, Disease/Wilds/Strife, Defend variants, cancels, England L1/L3/Esc, Sweden L1/L3/Esc, Russia L1/L3/Esc, Mining L1/L3/Esc, Blitz T1.

They do **not** validate: Explore/draw probabilities (only membership, no calibration); the counterattack kill set or Fear earned; Dahan destroyed counts; cascade targets and Presence destroyed; `advance`/`next_turn` and `outlook` beyond the first phase (`OutlookCheck.next_unanswered` vs `next_actual` is printed, never asserted); loss clock values (Mining verified by playtest narrative, not test); fear bookkeeping across cards (`RoundSummary.fear_total`); Event/Fear-card forecasting before resolution (Fast-phase saves are not compared); any player-driven outcome (skips are *removed* from the prediction, so a wrong skip model is invisible); anything at steps other than Ravage→Slow; France, Scotland, Livestock entirely; Isolate; Avarice.

---

## 6. Recommendations, ordered

1. **Project the public pools and facts the engine already decodes** (`projection.rs`): `PublicFear.unseen` (allowed_fear − discard − removed), `PublicEvents.unseen` (allowed_event − discard), `PublicSnapshot.distances` (from `EngineSnapshot.distances`), `PublicSpirit.growth` (from defs), `captured_ms`. Unlocks: Growth (unknowns column), Event/Fear choices, Draft reach, Retro alignment. Evidence: pools are setup facts (spec §5 visibility table); growth from DLL `SpiritList`, spot-checked against the printed panels (`si-rules-check`). Firewall test must be extended to permute these pools.
2. **Structured effect options on cards, events and fear cards** (`PublicPowerCard.effects`, `PublicEventSection.effects`, `PublicFearCard.effects`, tier B of the corpus spec), generated from printed text by a tagger that replaces `needs::tag_options`, with invariants and a disagreement report. Unlocks: Draft/Forget, Fast plan, Event/Fear counts, Growth's "card that saves the land". Evidence: printed card text (physical copy wins); the engine offers no structured effects, so the DLL `body` is a transcription to check, not the authority.
3. **Make `Edit` a complete action vocabulary with a legality/cost model** (`whatif.rs`): Play{card, option, land, assignment}, Push, Gather, Isolate, Strife, Skip, RemoveBlight, Token, Growth, plus Energy/plays/speed checks; mutate counts and `pieces` through one helper. Unlocks: every "my plan vs this Ravage" and "what would this card accomplish" question. Evidence: rulebook action definitions (core: Push/Gather/Damage/Destroy; JE: Isolate, Strife), FAQ for speed timing.
4. **Finish `advance()`** so multi-turn paths are honest: remove destroyed Dahan, apply counterattack kills (best case flagged), destroy Presence on Blight (and cascades' Presence), add a `PresenceClock`; run `loss_clocks` inside `outlook::Walk::step`. Unlocks: the competing-goals race, England/Mining per-land clocks across paths, U18. Evidence: engine `DealDamageAction.AfterAction`, rulebook Blight (Q5), Victory/Defeat timing FAQ (Q11).
5. **One source of truth for "applied"**: compute `RuleInPlay.applied` from the code (a registry of rule ids each hook claims) or move the `APPLIED` list to per-entry flags in `invader-rules.json` and assert in a test that every rule `adversary.rs` applies is marked. Unlocks: trust in `rules_in_play` for Habsburg-style "not familiar" explanations. Evidence: none new; a consistency test.
6. **Enums for steps, pieces, tokens, terrains, positions, triggers** in `public.rs`, plus `phase_position` fixed for `AdvanceInvaderCards` (treat as "explore"/pre-advance, consistent with `backtest.rs:90`). Unlocks: correctness at that step; safer what-if. Evidence: engine `StateType`/`IslandPieceType`/`TokenType` enum tables already in `engine_map.rs`.
7. **Spirit rules as a plug-in** mirroring `adversary.rs` (hooks keyed by spirit id and aspect), starting with the register's `ravage_damage_change`/`skip_invader_action`/`build_change` entries for Rampant Green, Vengeance, Thunderspeaker, Vital Strength, Keeper, Shroud, Stone, Towering Roots. Unlocks: correct ledgers for the spirits Brett plays. Evidence: printed panels; engine status effects observed in captures (each needs a game).
8. **Backtest widening**: assert `next_turn`'s Ravage set against the next turn's actual Ravage lands (today only printed); compare Dahan destroyed and counterattack kills against the `Damage` decisions in the log; compare `fear_total` deltas with destroy Fear + status effects; run all seven captures in `steam/tests/backtest.rs` (parameterized by a captures dir), and extend the permutation test to the new pools and to `compute()`. Unlocks: evidence for the multi-turn and player-driven parts. Evidence: the captures themselves.
9. **Fill the gaps the register already names**: Badlands damage, Beset TL III, England L6, Livestock L5, France L6, flip Immediately effects, Avarice (needs a game), Isolate (needs a game), Ocean cascade/drowning for Ocean's Hungry Grasp. Evidence per item: printed card/panel + the specific engine status-effect class observed in a capture (schedule the games in corpus spec §12).
10. **Scenario plug-in** beyond Blitz for the 12 scenarios in `invader-rules.json`, starting with those that change the pipeline (Ward the Shores, Second Wave, The Great River, Varied Terrains). Evidence: scenario card text (Brett's copy) and a capture each.
