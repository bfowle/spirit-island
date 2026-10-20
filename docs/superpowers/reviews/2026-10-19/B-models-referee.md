# B. Models referee: what the research specifies, what is built, and the stack as it should be

Scope: `reports/Spirit Island companion stat models.md` (the report), spec §5/§7/§14, `2026-10-19-questions-from-play.md`, Brett's messages (2026-10-18 15:45–17:25; 2026-10-19 22:00 onward), the `companion_stats` crate as of 2026-10-19 18:13, `rules.rs`, and the six playtest docs. Files are under `/home/brett/repos/spirit-island-steam-companion/crates/si-common/src/` unless stated. No code was written and no repository file was changed.

The one-paragraph verdict. The deterministic layers (ledger, rules, exact enumeration over unseen card orders, the blight-flip mixture) are built faithfully and, where the engine can check them, verified: seven recorded games, zero mismatches on Ravage/Build/Explore. The probabilistic framing on top of them is where the direction has drifted. The Runway shows a *cause-specific latent failure* quantity (Blight loss as if no other ending existed, under a "no spirit acts for four turns" policy) as the headline number; the Pace fits a mean of three non-stationary, policy-driven observations and extrapolates it to the turn a mark is reached; the Curve collapses Energy and plays into one scalar with Rei's contested 2.5 rate and compares its *multiple of Turn 1* to a terrain-footprint multiple. Each of those is exactly the kind of number the report said not to show. The fix is not more simulation; it is to re-aggregate the enumeration that already exists into a competing-risks cumulative incidence per ending with first arrival winning, to carry per-path per-land state so the adversary clocks live inside the same race, and to label the player-driven endings (Fear, board clear) as scenarios, never probabilities.

---

## 1. The model stack as the research specifies it

Build order from the report ("Probability models serve that frame in strict order", and the fit table, lines 138–149). Each layer: what it computes, what is exact, what is assumed, inputs.

| # | Layer (report name) | Computes | Exact | Assumed / labeled | Inputs | Report cite |
|---|---|---|---|---|---|---|
| 1 | **Ledger** (Lumia: budget → inventory → resolution order → shortfall) | Per land: invader damage with signed modifier terms, Defend, damage to land, Dahan destroyed, counterattack, blight yes/no, cascade. Per budget: Blight card (2B+1 healthy side), Fear to next card and to next divider (cards above divider × 4N − generated, overflow carried), deck clock (loss at turn D's Explore), each adversary loss condition as a count against its limit. The solve per land: Defend = D − h + 1. | All of it, given the face-up cards and the board. | Player choices (cascade target, counterattack allocation, Escalation picks) shown as candidate sets or min/max, never guessed. | Public snapshot; rules register | §"Ledgers, growth classes and timelines"; lines 107–111; 165–169 (deck clock deterministic) |
| 2 | **Counting** (hypergeometric family on the exchangeability lemma) | P(next card is kind k) = n_k / m over the stage's unseen multiset; P(≥1 of K matching in a draw of 4 from N); negative hypergeometric for first match; Event/Fear-card branch odds from the public pool conditioned on Healthy/Blighted and Terror Level. | Closed form. | Pool = allowed list − cards seen; a mismatch with the deck count is a warning. Overlapping categories computed directly, not as a partition. | Save's `Allowed*` lists; seen cards; discards | lines 34–39, 151–157 |
| 3 | **Land tempo** (per-land threat map with badges) | For each possible next card, run Explore→Build→Ravage deterministically; badges ahead/on/b2b/capital; separate certain (face-up) from possible (next card). | Exact enumeration (≤ 6 cases one card ahead; U(U−1) two ahead within a stage). | Explore source predicate + Wilds + Isolate. | Layer 1 + layer 2 | lines 64–71, 159–161 |
| 4 | **Timelines** ("if X is next" table; darkest and best) | Per next card: which lands explore, then build, then ravage; min and max blight across the cards that follow. Stratified by next card (Rao–Blackwell: never increases variance). | Exact or hybrid. | Same as 3. | Layer 3 | lines 143, 161 |
| 5 | **Momentum** (Rei's curve) | Per growth line: Energy/turn, card plays/turn, presence on the island, element coverage, by turn, from the presence-track graph; plotted against expected lands hit per turn from the deck recipe (Stage I/II/III ≈ 2B/3B/4B lands, exactly computable from the terrain mix). | The projection along a chosen growth line is exact; the step function's land counts are exact given the board and the unseen terrains. | **Natural units only.** Any Energy-per-play valuation is a user-set parameter with a sensitivity band, never a fact (Rei's 2.5 is disputed in-thread by darker, Gerhards, Tongar). | Presence tracks; deck recipe; board | lines 73–95, 313–317; App. A #8, #9 |
| 6 | **Race** (competing-risks cumulative incidence) | One CIF curve per ending: Fear win, Blight loss, deck exhaustion, each adversary loss clock; per-round hazards for "is this the dangerous round". Deck clock as administrative censoring at a known time. | The deck clock exact; others from forward rollouts under a labeled policy. | **Never compute clocks in isolation** (latent-failure quantity overstates every risk; naive per-cause numbers sum past 100%). Estimand is P(outcome \| state, rollout policy), a lower bound on good play. v1 may draw an honest race from exact pieces: deck clock + invaders-only exact enumeration ("if you do nothing") + fear runway from guaranteed sources. | Layers 1–4, repeated per turn | lines 163–186, 356–364 (spec 7.2) |
| 7 | **Comparison** (common-random-number MC, Wilson) | "A better than B in x% of futures, worse in y%, same in z%". | Simulated. | CRN; fixed seed per state; rounding. | A rules engine for all content | lines 171–175 |
| 8 | **Planning** (depth-1 expectimax, chance nodes at reveal) | Decision fan: for each lever option, each next card applied through the ledger → saves/hurts/neutral. | Search over exact chance nodes. | No verdicts; options in fixed order. | Layers 1–4 | lines 177–179; spec 7.5 |
| 9 | **Bound** (hindsight / determinization) | Upper bound V^hindsight to bracket V^rollout. | Simulated. | Strategy fusion makes it an upper bound only. | Rules engine | line 177 |
| 10 | **Learning** (proper scoring against exact p*) | Optional hold: expected Brier excess (q − p*)². | Exact when p* exact. | Correct for MC error when p* is simulated. | p* from 2–4 | lines 195, App. A #15 |

Display contract that every layer must meet (lines 188–196): natural frequencies ("2 of 3 unseen Stage II cards"), distributions not point values (20-dot strip), deltas not verdicts ("what changes if you hit land X"), a pocket rule beside each exact number, fading explanations, an optional predict-first hold. Firewall corollary (line 198, spec §5 rule 5): every derived number, including any rollout, samples hidden orders from the public belief, never from the save.

The report's three v1 corrections to §7.2 (line 306–309) are the most relevant to this review: fear by dividers (done), **replace rolling-rate projections with the exact deck clock plus cumulative incidence** (not done: the rolling rate is what `pace.rs` ships), and model the adversary loss clocks (done as counts, not as part of the race).

---

## 2. What is implemented, and how faithful it is

Summary table first; details per module after it.

| Module | Computes | Faithful to report/spec? | Main flaws |
|---|---|---|---|
| `rules.rs::ravage_ledger` | Per-land Ravage with signed terms; engine status effects applied; adversary hooks | Yes (layer 1); 7 games, 0 mismatches | Loss-on-empty rule inferred from two games, not a register entry; pocket rules hard-code land health 2 |
| `ledger.rs` | Budgets: Blight, Fear, deck, each loss clock; spend inventory; shortfall; solve per land | Yes for Blight/deck; Fear budget narrow; clock timing misframed | Shortfall is "card does not change", not "no loss"; no minimum-cost cover of the shortfall; Mining clock's "prevent this turn" is the wrong deadline; France limit per player, engine per board |
| `outlook.rs` | Exact enumeration over unseen-card orders, 4 Invader Phases, no spirit action; next-card rows; flipped_by; lost_by | Layers 3–4 exact and correct. Layer 6 **misframed** | lost_by is the Blight-only latent quantity; deck-out and adversary endings are not in the race; merged paths drop per-land state; cascade blight counted but not placed; Escalation/options not applied; "Unanswered" is the headline |
| `mod.rs::p_loss` | P(Blight loss \| k more Blight) through the flip mixture and Still-Healthy chains | Exact over the public pool; the 76/42/18 test checks by hand | Immediate effects and solo redraw not modeled (labeled floor); emptying = loss is an inferred rule |
| `pace.rs` | Fear marks (exact), need per turn (exact arithmetic), recent pace = mean of last 3 finished turns, reach turns, board route | Marks and need are Lumia arithmetic and fine. Recent pace and reach are the "rolling rate" the report said to replace | Three non-stationary, policy-driven observations; no uncertainty representable; extrapolated to a turn |
| `curve.rs` + `app/views/curve.rs` | Lands named per stage; two track projections; growth multiples with a 2.5 (1.5–3.5) Energy-per-play scalar | Natural units in `common`; the app adds the exchange rate and "who's growing faster" | Terrain footprint ≠ pressure; one presence per turn from one track ≠ growth options; multiples of a tiny Turn-1 base; the exchange rate is exactly what the report said never to show as a fact |
| `whatif.rs` | Destroy / Defend / +Dahan / Track edits on a copy of the snapshot; fear and Terror Level bookkeeping | Correct pattern (deltas, nothing written back) | Cannot express push/gather, Strife, Isolate, skip, partial damage, tokens, blight removal; cost and card plays not enforced; Track edit adds presence to no land |
| `needs.rs` | P(draw of 4 holds a card that saves land L and reaches it, and is Fast) | Layer 2 correct (hypergeometric, draw 4 keep 1) | Computed into `CompanionStats.needs`, **displayed nowhere in the native app**; "playable now" ignores cost and card plays; conditional effects counted; binomial pocket rule unlabeled |
| `adversary.rs::loss_clocks` | now / after / limit / trigger / checked for England, France, Scotland, Mining, Russia, Livestock | Counts correct where verified (Mining, Russia) | Not in the race; no Presence clock; France unit; Livestock running count unverified; `after` is one phase only |
| `invaders.rs` | Forecast at any step; `next_turn`/`advance` | Yes | `advance` drops cascade blight and option-based Escalations (lower bound, unlabeled) |
| `backtest.rs` | Deterministic forecast vs engine per turn; `outlook_check` | Validates layers 1–3 and pool bookkeeping | Cannot validate anything probabilistic or policy-dependent; `next_unanswered` vs actual is unfalsifiable as stated |
| `review.rs` | Blight attribution, invader chains, flow, pressure, reflections | 7.7 implemented | `invader_damage` hard-codes 1/2/3 (Sweden L3 pressure wrong); cause labels miss Blight card/Event (R8) |

### ledger.rs

- **Shortfall `spend − (remaining − 1)`.** Correct for the question "how much of this turn's spend must I prevent so the card does not empty" (empty = flip on the healthy side, a redraw on a Still-Healthy side, a loss on a Blighted side). It is *not* "how much to prevent to avoid losing": before the flip, a flip is survivable, and the loss budget is the mixture (`p_loss`). The ledger's `at_zero` string carries this distinction, but `shortfall` is one number for three different stakes. Keep the number, name it "to keep this card".
- **Solve `D − h + 1`** (`margin = damage_after_defend − land_health + 1`). Correct: blight iff D − d ≥ h, so additional Defend d ≥ D − h + 1. `damage_after_defend` already nets existing Defend, so the figure is *additional* Defend, which is what a player wants. `land_health` honors `health_modifier` and `damage_taken`. The pocket rules in `ledger.rs` ("Health (2) … Damage − 1") and the budget text hard-code h = 2; they should print the land's actual health.
- **Shortfall is not a cover.** A shortfall of 2 can be one land's cascade or two lands' single blights; the ledger lists each spend's solve but never the cheapest set of solves that covers the shortfall (a tiny knapsack over ≤ 4 items). Lumia's method ends in "the shortfall is X; the cheapest fix is Y". Add the cover.
- **Cascades as minimum extra**: `cascade_min.max(1)` = BFS distance to the nearest unblighted land, Oceans excluded (`mod.rs::cascade_min_extra`). Correct as a lower bound and labeled "fewest". The `safe_neighbors` and `cluster` fields are the right companions. Not carried: a cascade that spares Presence may cost more Blight (two objectives); show both the min-Blight route and the min-Presence route.
- **Fear as a goal.** Budget = fear to the *next card*, gains = counterattack fear best case (`certain: false`, right). Three gaps: the goal the player asks about is the next *divider* (Terror Level), which is only in `detail`; fear from planned plays is not in the inventory (the what-if `Destroy` edit does add it, so the lever exists); and the banked face-down cards, which are the player's actual unknown, have no counted pool (`PublicSnapshot` has `minor_unseen`/`major_unseen` but no unseen Fear or Event pool, so the "16 of 50 at Terror I" number from spec 7.2 cannot be computed today).
- **Loss clocks as budgets.** `left = limit − now (+1 when trigger is "over")` matches the register texts: England "7 or more" (at, 7), Scotland "greater than 2 × boards" (over), Russia "exceeds Beasts on the island" (over, with a moving limit `beasts − lost`), Livestock "exceeds players" (over), Mining "at least 8" (at). France: code uses `7 × players`; the report (line 217) and engine use per *board*. Same number in solo on one board; wrong with an extra board. The Mining urgency line ("Invaders win at the end of this Fast Phase: remove N now") is right when `now ≥ 8` during Spirit/Fast. But `shortfall = spend − (left − 1)` for a clock "checked at the end of the Fast Phase" misframes the deadline: a count that reaches 8 *after this Invader Phase* is lost at the end of **next** turn's Fast Phase, so the player has the whole next Spirit Phase plus Fast Phase to remove one piece. The ledger should carry a per-clock `deadline_step` and compute the shortfall against it. This is a competing-risks ordering fact, not a UI nicety.
- **Not present as budgets**: Presence (a spirit with no presence loses; U18), and the Blight budget is not among `loss_clocks`, so the two lists have different shapes for the same kind of thing.

### outlook.rs

- **Enumeration.** Branch per *kind* (stage, sorted terrains, escalation flag) with weight = count of that kind in the stage's unseen pool; remove one of that kind; recurse. Since the stage of each face-down card is public and the pool size at each depth is common to all branches at that depth, weights are proportional to the exact probability of each kind sequence under within-stage exchangeability (report lines 34–39). `orders` is therefore a correct natural-frequency base ("in 4 of 12 orders"). The removed-at-setup card is handled correctly by never shrinking the pool to the deck length. Unit test `every_order_counted_once` checks 3·2·2. This is layer 3–4 done right.
- **Policy.** No spirit action; cascades at their fewest; Events and Fear cards not drawn; flip immediates not applied; player-pick Escalations and rule actions (Land Rush Town, Sweden swap, Russia Explorers, Mining Boom upgrade) *not placed*. The doc comment says "a floor on the pressure, not a prediction"; the app's headline word is "Unanswered". Two of those omissions are not floors: a Prussia Land Rush Town and a Sweden Dahan→Town swap *always* happen on a flagged Stage II card; only the land is the player's pick. The consistent labeled policy for a floor is "players pick the least harmful option", and that minimum should be applied, not zero.
- **Cascade blight is counted but not placed** (`advance`: "its cascades' targets are the players' pick, so they aren't placed"). Later turns in the same path then compute cascade distances on a board missing those blights and never mark the receiving land as blighted (so a later Ravage there is modeled as non-cascading). The count stays a floor, but it is an inconsistent floor: the Blight *card* is charged while the *board* is not. Fix: place the cascade on the min-distance target (the same choice the count assumes), and record that it is an assumption.
- **Horizon.** `PHASES = 4`; the last phase's Explore is not branched because it adds no Blight within the horizon. True for Blight; not true for Habsburg Mining's load (an Explorer counts) or for England's capital via a Building Boom at that Explore. Harmless at horizon 4 only because the check comes after the horizon. If endings other than Blight enter the race, branch the last Explore too (cost is ≤ 6×).
- **Cutoffs.** There is no `.min(48)` anywhere in `common/src` (grep over the crate). The real caps are `.min(12)` in `needs.rs::reduction` and `.min(16)` in `rules.rs::best_destroy`: exhaustive subset searches that silently ignore pieces beyond the cap. No recorded land has exceeded them (Habsburg NE7 peaked at 7 Invaders + 1 Blight), but silent truncation should become an `unmodeled` note, not a quiet `min`.
- **Merged paths lose what competing risks need.** `OutlookPath` keeps per turn: blight added, island totals [E, T, C], counts of lands ravaging/building/exploring. Merging on those drops: max Town+City in one land (England), total Towns (France), coastal lands with a City (Scotland), max Invaders+Blight in one land (Mining), 6+-overkill blight (Livestock), Beasts destroyed (Russia), presence per spirit per land (Presence clock), the step within the turn at which each threshold is crossed. Compute each ending's first-arrival (turn, step) *inside the walk*, store that tuple on the leaf, then merge.
- **`lost_by` via `p_loss`**: `lost_by[j] = Σ_paths w · p_loss(cum_j)`, normalized. Within a path the only randomness left is the flip card, and `p_loss(cum_j)` is exactly P(the Blight card is exhausted by turn j | this order), monotone in j, so `lost_by` is a valid CDF of the Blight hitting time *as if Blight were the only ending*. That is the Putter–Fiocco–Geskus latent-failure quantity the report (line 167) says not to show. On a path that ends at the deck's exhaustion (`deck_out`, leaf pushed early, `cum.get(j).or(cum.last())`), the Blight probability keeps counting past the game's end. On a path where Land Stripped Bare fires at T4, `lost_by[T5]` still charges Blight. Section 3 gives the corrected quantity.
- **`flipped_by`**: exact count of orders with cumulative blight ≥ remaining on a healthy card, monotone (test checks), zeros once flipped. Fine. It is the one output on this chart that is a pure count with no mixture; label it as such and put it on the Blight lane, not in a race.
- **`next_cards`**: per next-card kind, count/of, the lands it explores, min/max blight per later turn, orders starting with it. This *is* the "if X is next" table (layer 4). It reports only Blight min/max; it should also report which lands Build then Ravage (the land list per turn), which is what Brett asked at Test 1 T4 ("C7 … 9 damage").

### pace.rs

- **Exact and fine**: `marks` (Terror II/III/Victory totals from `(out + cards) × req`), `total`, `deck_out` (turn + cards left, +1 after the cards advance; the test's 7 cards at Turn 6 → T13 is right), `turns_left` including the deck-out turn (its Fear and Ravage precede its Explore; Brett Q11), `need = (victory − total)/turns_left` as a budget rate, `level_ups`, `board` blockers per level. This is Lumia arithmetic and belongs on the Fear lane.
- **Not defensible**: `recent` = mean of the last 3 *finished* turns, and `reach` = `turn + ceil((mark − total)/recent)`. Fear per turn is non-stationary by construction (Terror Level raises fear-card strength and lowers the victory bar; the spirit's economy grows; invader density grows), quantized by cards of 4N, and chosen by the player each turn. A sample mean of three such values has no sampling model; a standard error would be a fiction and a point extrapolation is worse. The report's Appendix A #5 asks for "a renewal band" *compared against* "guaranteed fear plus land inventory", and says "treat small samples honestly". Honest here means: show the realized per-turn values as dots on the lane; show `need` as the line to beat; show reach as **scenario lines** at user-visible rates ("at 3 a turn: T10; at 5: T8; at your best turn so far, 6: T7"), never as "your pace" or a single arrival turn; and show the *guaranteed* floor (Dahan counterattack best case this turn, fear from cards in hand that can be played this turn) as the only number called a forecast.
- **Board route** uses the same extrapolated pace for `arrives`. Same retirement.

### curve.rs (and the app's curve view)

- **Invader side**: `stage_reach[i]` = mean lands matched per unseen card of stage i (falls back to darker's 2B/3B/4B with no pool). "Lands named" is the card's terrain footprint, not pressure: Explore acts only where there is a source, Build and Ravage only where there are Invaders; a Stage III double-terrain card over four empty lands is zero pressure. The forward enumeration already produces, per turn and per order, the actual Ravage/Build/Explore land counts, damage and Blight (`OutlookPath.lands`, `blight`; `review.rs::pressure`-style damage is one more field). Rei's "curve" is about whether the engine outruns *required answers*, so the honest invader line is "lands that act, damage threatened, Blight threatened, per turn, from the face-up cards and the next card's options", with the certain part (T+1, T+2 lands) drawn solid and the uncertain part (T+3, T+4) as a band over orders. That is a caricature only in the sense that it ignores the player; it is at least the right unit.
- **Spirit side**: `projected(sp, kind, n)` takes `n` unrevealed nodes from the track with the most nodes of `kind` and reads the best revealed value: a correct reading of track semantics, giving two envelope lines (all-Energy, all-Plays). It is not a per-growth-option projection (Reclaim vs gain a card vs +presence +Energy; two presence for some spirits), it ignores Energy *stock* and spend (the Ocean game had 20 Energy idle with plays binding), hand size and Reclaim cadence, and elements. Spec 7.4/7.5 asked for "per growth line"; `growth.rs` already has `GrowthOption`. The honest momentum panel is a table per growth option for the next 2–3 turns: Energy income, Energy on hand after this turn's plays, card plays, cards in hand, elements reachable, Reclaim turn. Natural units, no scalar.
- **The exchange rate**: `app/views/curve.rs` combines Energy and plays at 2.5 (band 1.5–3.5) into one "worth", divides by Turn 1's worth, and draws "WHO'S GROWING FASTER as multiples of Turn 1" against the invader multiple. The report (line 80) says any single valuation is a user-set parameter with a sensitivity band, never a fact; the band is present, but the chart's whole point is the scalar. Multiples of a near-zero Turn-1 base (Shadows: 0 Energy income at Turn 1) are unbounded; the invader multiple is of terrain footprint. This panel answers a question nobody asked and should go (Section 5).

### whatif.rs

- **Edits**: `Destroy{land,kind}` (removes a piece, adds its Fear, earns cards and raises the Terror Level when a section empties: correct divider semantics), `Defend{land,amount}`, `Dahan{land}`, `Track{spirit,track}` (reveals the next node, `presence_on_island += 1` but places it in no land, so range, Sacred Sites and the presence-at-risk map do not move). Applied to a copy; every stat recomputes; the app shows base→tried deltas ("T5 7→3 in 10"). This is the right pattern and the one place the app already does "deltas, not verdicts".
- **Fear bookkeeping**: cards are earned off the top of the current section; the banked card's *effect* is an unknown draw from the Fear pool and is not represented (no pool in the snapshot).
- **Cannot be expressed**: Push/Gather (moving pieces between lands: Lure, Mantle, Favors; Brett's actual T3 and T4 saves), Strife, Isolate, Skip, Wilds/Disease/Badlands/Beasts, Blight removal (Cycles of Time), partial damage (`PublicPiece.damage` exists but no edit), downgrade, presence placement in a land, Energy and card-play budgets (nothing stops a what-if the hand cannot pay for). Spec 7.5's "levers" are mostly these.

### needs.rs

- Wired into `CompanionStats.needs` (computed in `mod.rs::compute`), serialized, and **not read by any `app/src` view** (grep: no `needs.`/`save_land`/`p_fast_reach` consumer outside `common`). The Vue frontend is out of scope by Brett's reset. So the one model Brett asked for by name on 2026-10-18 at 15:59 is computed and invisible.
- Statistics are right: unseen pool = allowed − seen; P(≥1 in 4) hypergeometric (draw 4 keep 1, the other 3 public). The pocket rule prints `1 − (1 − k/n)^4` (binomial); the error is ~4/96 (report line 42) and acceptable, but it should carry "≈".
- "Saves the land" = `reduction(tags) ≥ margin` with best-case assignment; conditional lines count; so `effect` is an upper bound. "Playable now" (spec 7.4) requires speed, cost ≤ Energy, a card play free, reach; the code checks speed and reach only. Majors in particular are counted with no Energy or Forget constraint. Spirit reach rules (Shadows' 1 Energy to target a Dahan land) are noted in `basis` as not applied: correct labeling.

### mod.rs: `p_loss`

- The chain: before the flip, the current card is uniform over `same_healthy_side` cards; its flipped value X; if the blight to place reaches X and the side is Blighted, loss; if Still-Healthy, redraw uniformly from the rest and recurse. After the flip, the same from `remaining`. The playtest U16 figures (5→76%, 4→42%, 3→18%) check by hand against the 22-card pool (4/22 = 18.2%; (4/21 + 9)/22 = 41.8%). Exact over the public pool.
- Assumptions, labeled in the doc comment: Immediately effects not applied (Promising Farmland added a Town and a City in the England game; Strong Earth added a Blight in the Prussia game: both change other clocks), the solo redraw of 2-per-player cards not modeled (Brett uses it), `flipped_total` already scaled per board. One assumption is *not* labeled: "a Blighted side that empties loses" (versus "loses when Blight must be added and none remains"). Two recorded games (Sweden T7, Prussia T6) ended `LoseTooMuchBlight` exactly on emptying, so the engine agrees; it still belongs in the rules register with a citation, not only in a test.

### adversary.rs: loss clocks

- `now`, `after` (one Invader Phase ahead, set only when it changes), `limit`, `trigger`, `at` (land), `checked`. England/Scotland/Mining counts are pure board reads; Russia and Livestock read the adversary panel (Russia verified at game end; Livestock unverified). Mining's `after` adds the forecast Ravage and Ceaseless Mining Blight per land and was verified on the Habsburg game (6→7→8). The urgency logic is right; the deadline semantics in the ledger are not (above).
- Missing: the Presence clock (no presence = loss; the Blighted card's per-phase effects such as Memory Fades to Dust drive it), and the France unit (per board).

### backtest.rs

- Validates, per turn, from the Ravage-step save: Ravage Blight lands and amounts, Builds (with Choke/Disease/Vengeance skips classified as choices), High Immigration, Escalation Builds, Mining Boom upgrades, Explore lands for the card actually drawn. This is exactly layer 1 and layer 3 for the realized card, and it is why "7 games, 0 mismatches" means something.
- `outlook_check`: `in_rows` (the drawn card's kind was in the unseen pool) is the important firewall-and-bookkeeping check (40/40). `next_unanswered` (min,max) beside `next_actual` is "the gap is what the spirits prevented, or a model error", which cannot distinguish the two and is not a check. Drop it or turn it into a one-sided check with the no-action board *after* the actual Fast phase (then spirits cannot have acted, and only Events/Fear cards can move the number; those are logged and can be subtracted).
- Cannot validate: `p_loss` (4 flips in 7 games), `lost_by`, pace, curve, needs. See Section 6 for what *can* be done.

---

## 3. Competing risks as the backbone

Brett (2026-10-19 22:47): "the statistical model … competing-something … is the basis of this game where so many things come into play and compete for attention." The report agrees (lines 165–169). The existing enumeration already contains almost everything needed; what changes is the *aggregation*.

### 3.1 Endings, and when each is checked

Each ending has a check step. Within one turn the order is the engine's state order (report line 58): Spirit → Fast → [end of Fast: **Habsburg Mining**] → Blighted Island → Event → Fear [**Fear victory** "when revealed"; fear-card effects] → High Immigration → Ravage [**Blight loss** when the card empties; "ever" clocks whose counts a Ravage changes: **Livestock**, **Russia**; **Presence**; board-clear win if the last blocker dies] → Build [**England capital**, **France Towns**, **Scotland coastal Cities**] → Explore [**deck exhaustion** when no card remains; Explorers raise Mining's load for *next* turn's check] → Escalation (Towns: England, Prussia, Sweden; counts for England/France) → Advance → Slow → Time Passes. Nature Incarnate's errata: losses are checked once no triggered actions remain (report line 237), so cascades resolve before the Blight check.

The Mining case is the one with a *deferred* check: a load that reaches 8 during turn j's Invader Phase is lost at the end of turn j+1's Fast Phase. The Habsburg game ended that way.

### 3.2 Per path: deterministic endings plus one mixture

Fix the policy (no spirit action, min-cascade routing placed on the board, players' picks at their least-harmful option; all labeled). Then along one card order, everything is deterministic except the Blight card's hidden side. For each path p:

- `T_D(p)`: the turn whose Explore finds no card (exists in the walk as `deck_out`; make it per path).
- `T_A,k(p)`: for each adversary clock k, the first (turn, step) at which its count meets its trigger, respecting the check step (Mining: end of the Fast Phase after the phase that reached 8). Requires per-land state after each step (see 3.4).
- `T_P,s(p)`: for each spirit s, the first turn its presence on the island reaches 0 under the routing policy (blight destroys one presence of each spirit in the land; cascades under the min-Blight route; Blighted-side per-phase effects as a labeled gap).
- Blight: `cum_j(p)` = Blight taken off the card through turn j's Ravage (exists). `F_B,p(j) = p_loss(cum_j(p), blight)` is exactly P(the card is exhausted by turn j's Ravage | order p). It is monotone in j.

Let `a(p)` be the earliest deterministic ending on path p, at (turn, step) `(t_a, s_a)`, and let `cum⁻(p)` be the Blight off the card at the last Ravage *strictly before* `(t_a, s_a)` (if the deterministic ending is at turn t_a's Build or Explore, that turn's Ravage counts; if it is at the end of t_a's Fast Phase, it does not).

Then, per path:

- Blight CIF: `CIF_B,p(j) = p_loss(cum_{min(j, t_a*)}(p))` where `t_a*` is the last Ravage turn before `(t_a, s_a)`; flat afterwards.
- Deterministic ending CIF: `CIF_a,p(j) = (1 − p_loss(cum⁻(p))) · 1[j ≥ t_a]`; zero for every other deterministic ending on this path.
- Survival: `S_p(j) = 1 − Σ_causes CIF_cause,p(j)`.

Over paths: `CIF_cause(j) = Σ_p w_p · CIF_cause,p(j) / Σ_p w_p`. Each CIF is non-decreasing, they sum to ≤ 1, and `S(j) = 1 − Σ CIF(j)`. No simulation: at horizon 4 the walk has a few hundred leaves; the mixture is closed form.

This is exact over the public pool **given the policy**. The policy is the only assumption, and it is the same one the Runway already makes.

### 3.3 Natural-frequency display

Orders are counts; the Blight mixture is a probability. Keep them separate on screen: "Land Stripped Bare at the end of T4's Fast Phase: **every order** (10 of 10). Deck runs out: T8's Explore, every order. Blight: **never first**." When the Blight cause has mass, give it in tenths of orders *and* name the mixture: "Blight by T5: 4 in 10 orders, if the hidden side is a 2- or 3-Blight card (12 of 22 unseen)". The 20-dot strip from the report (line 192) fits: dots colored by ending, by turn.

### 3.4 Per-path state to add to the walk

Per turn, after Ravage / after Build / after Explore+Escalation: `max_town_city_one_land` (England), `towns_total` (France), `coastal_lands_with_city` (Scotland), `max_load_one_land` = Invaders + Blight (Mining), `overkill6_blight` increment (Livestock), `beasts_destroyed` increment and `beasts_on_island` (Russia), `presence[spirit]` after blight and cascades (Presence), `fear_guaranteed` (counterattack best case; see 3.6), `cum_blight` (exists), plus the land lists per step (for the "if X is next" rows). These are all already computed somewhere in `forecast_at`/`loss_clocks`/`ravage_ledger`; the change is to call `loss_clocks`-style counters on the path board at each step and record first crossings, then merge leaves on the tuple of (first-ending, turn, step, cum_blight sequence) instead of the current (blight, invaders, lands).

### 3.5 Where the flip mixture enters, and the Still-Healthy redraw

Only in the Blight hazard. The hazard at turn j's Ravage on path p, given survival to that step, is `h_B,p(j) = (p_loss(cum_j) − p_loss(cum_{j−1})) / (1 − p_loss(cum_{j−1}))`. The Still-Healthy redraw is already inside `p_loss` (`draw_loss` recursion over the remaining pool), so nothing changes in the mixture. Two labeled gaps remain: the new card's Immediately effect (may add Blight, Towns, Cities: feeds the other clocks) and the Blighted side's per-phase effect (feeds the Presence clock). Both are *known texts* once drawn and *a countable pool* before; the first-pass label is "not applied", and the second pass is a per-card effect table.

### 3.6 Player-driven endings: Fear win and board clear

No exact computation exists without a policy, and the no-action policy makes them *impossible* (0 fear from spirits). Defensible baselines, each labeled with its name on screen:

1. **Unanswered floor**: fear from Dahan counterattacks only (automatic; allocation is the players' pick, so show best-case and worst-case), fear-card draws not counted. Enters the race as a deterministic `T_W(p)` per path if the floor reaches the deck; usually it does not, and the honest line is "Fear victory: not before the deck runs out, with no spirit action".
2. **Scenario rates**: the player chooses a rate from a small set of dots (the realized per-turn values are the natural candidates: "3, 5, 6 so far"), or sets one. For a rate r, `T_W(r)` = the turn `total + r·(t − turn) ≥ victory mark`, and every loss CIF is censored at `T_W(r)` for display ("if you keep earning 5 a turn, the Fear deck empties at T8, before Blight can arrive in 7 of 10 orders"). Show as scenario lines; never average them, never attach an interval.
3. **Board clear**: `terror_victory_blockers` now, and per path the blockers after each step under no action (monotone up). The honest display is the count of blockers by turn per order, with the Terror Level marks as the bar lowering, and the required removal rate per turn beside it. A probability of a board-clear win has no basis and must not appear.

### 3.7 What is exact, what is assumed, what is never a probability

- **Exact**: order counts and weights; the deck-out turn per path; each deterministic clock's first crossing per path given the policy; the Blight mixture given a Blight count; the required fear per turn; the card pools and draw odds.
- **Labeled assumptions**: no spirit action; cascades routed to the nearest unblighted land and placed there; players take the least harmful option at Escalations and rule actions; Events and Fear cards not drawn (their fear not earned; their effects not applied); flip Immediates and Blighted-side per-phase effects not applied; 4-phase horizon.
- **Never shown as a probability**: P(win); P(Fear victory); "your pace" as an estimate with error; any number that blends the no-action floor with a scenario rate; the latent Blight-only `lost_by`; a combined "P(loss)".

### 3.8 The hazard question: "is this the dangerous turn?"

`h_cause(j) = (CIF_cause(j) − CIF_cause(j−1)) / S(j−1)`: "of the futures still alive entering T5, 6 in 10 end at T5's Ravage, by Blight". Two teaching points the companion should make explicit, because they follow from the pipeline, not from probability. First, T+1's Ravage lands are **certain** (the Build card is face up) and T+2's Ravage lands are certain in identity (the Explore card is face up once drawn; uncertain only in the Explorers the next card adds). So for two turns the "danger" is a ledger, not a hazard; the spread over orders appears only at T+3 and T+4. The Runway should draw T+1 and T+2 as single lines and only fan out after. Second, the Fast-phase check for Mining means the "dangerous turn" is the one *after* the load is reached, and the answer window is that turn's Spirit and Fast phases; the ledger must say so.

### 3.9 The Habsburg Turn 3 case

At the start of Turn 3, Mountain NE7 held 7 (4 Explorers, 3 Towns) and the Build card (face up) would Ravage it under Ceaseless Mining, adding a Blight: 8 in **every** order, since no unseen card touches that count before the check at the end of Turn 4's Fast Phase. A correct race shows `CIF_Mining(T4) = 1.0` (10 of 10 orders) and `CIF_Blight(·) = 0` on every turn (censored at T4's Fast Phase), with the Blight lane still showing the card's budget as a ledger. The Blight-only view's "lost by T5 in 9 of 10" was the cause-deleted latent quantity: it answers "if Land Stripped Bare did not exist, how often would Blight have ended the game by T5", which no player at that table was asking. The what-if that matters is "remove 1 from NE7 before the end of T4's Fast Phase" (one `Destroy` edit): the Mining CIF drops to 0 at T4 and the Blight race reappears. That delta is the lesson, and the existing what-if pattern already shows deltas; it only needs the right base quantity.

---

## 4. Question → model → status

From `2026-10-19-questions-from-play.md`.

| Question (moment) | Model(s) that answer it | Exact or assumption | Status | Smallest change to make it answerable |
|---|---|---|---|---|
| **Growth** — "Blight per land with and without the option's play; what each land needs" | Ledger per growth option (`ravage_ledger`, `ledger.rs`) with the option's plays applied as what-if edits | Exact given the plays | Partial: `whatif` has Defend/Destroy/Dahan/Track; no Push/Gather, no Reclaim, no "gain a card" | Add `Edit::Move{from,to,kind}` and `Edit::Growth{option}` (uses `growth.rs::GrowthOption`: track reveals, Energy, Reclaim returns the discard to hand) |
| Growth — "the draw: how many unseen Minors/Majors hold a card that saves the named land and is playable now" | `needs.rs` (hypergeometric, draw 4) | Exact count; "saves" is an upper bound | Implemented, **not displayed**; cost and card plays not checked | Render `SpiritNeeds.save_land` on the Growth screen; add `cost ≤ energy_after_option && plays_free` to `count()`'s reach predicate; mark the pocket rule "≈" |
| Growth — "banked Fear cards that could save it (16 of 50); the Event as a common shift" | Counting over unseen Fear and Event pools, conditioned on Terror Level and Healthy/Blighted | Exact | **Missing** at the data level: `PublicSnapshot` has no unseen Fear or Event pool | Add `fear_unseen`/`event_unseen` (allowed − seen, by Terror Level section) to the projection; tag card text as `needs.rs` does for powers |
| Growth — "and later: Energy, plays, presence by turn for this option; Reclaim now vs later" | Momentum projection per growth option (`growth.rs` + track graph) against the next turns' certain and possible lands | Exact per option for income; hand/Reclaim exact; spend is the player's | Partial: `curve.rs` projects two envelope tracks, not options; no hand/Reclaim | Replace `GrowthLine` with a per-option 3-turn table (income, Energy on hand, plays, hand size, Reclaim turn, elements) |
| Growth — "what's coming at the lands within 2 turns (C7's 9 damage)" | Timelines: `next_turn` + `outlook.next_cards` with land lists | Exact for T+1 (certain) and T+2 identity; T+2 amounts depend on the next card | Partial: `next_cards` carries lands explored and Blight min/max only | Add per-turn land lists and damage to `NextCard` and `OutlookPath` |
| **Draft and Forget** — "what each candidate does on this board this turn, at its speed" | Ledger with the candidate's effect applied as edits at Fast (before Ravage) or Slow (after Explore) | Exact where the effect is expressible | **Missing**: tags exist (`tag_options`), no resolver from tags to edits; speed window exists conceptually in `timing_windows` | A `tags → Vec<Edit>` resolver for the expressible families (Defend, Damage→Destroy, Push/Gather→Move, Strife) with "not modeled" for the rest; run `whatif::apply` on the forecast board at the card's speed |
| Draft and Forget — "what it pairs with (elements toward innate levels; same-land combos)" | `spirits.rs::InnateStatus` / `stats.rs::compute_innate_status` with the candidate added; split hand-now vs discard | Exact (element arithmetic) | Partial: innate status exists; "with candidate" and "hand vs discard" split missing (U11, U13) | Compute innate levels for hand ∪ {candidate} and for hand ∪ discard ∪ {candidate}; show the delta per level |
| Draft and Forget — "what each Forget costs (Fast protection, elements, Fear)" | Same element arithmetic; a count of Fast Defend/Fear sources in hand+discard | Exact | **Missing** | Per card in hand+discard: speed, Defend n, Fear n, elements; subtract and show |
| **Fast plan vs this Ravage** — "per Ravage land after the plan: damage, Blight, Dahan, Presence, counterattack Fear" | `whatif::apply` + `ravage_ledger` | Exact | Implemented for Defend/Destroy/Dahan; **Push/Gather missing**, cost not enforced | `Edit::Move`; enforce Energy and card plays on the what-if list (gray out what the hand cannot pay for) |
| Fast plan — "the Blight card after the turn and P(loss) at that depth (5 → 76%)" | `p_loss` on the plan's Blight count | Exact over the pool; Immediates not applied (labeled) | Implemented | Show the flip pool as a dot strip beside the number (22 cards) |
| Fast plan — "B3 is worth 3 of the 5 and the exact save per land" | Ledger spends and solves | Exact | Implemented (`Spend.solve`, `defend`) | Add the cheapest-cover line ("prevent 2: B3's Town out, or Defend 2 on B2") |
| Fast plan — "what's still hidden (Fear cards, Event) and how often it helps" | Counting over Fear/Event pools, conditioned | Exact | **Missing** (no pools) | As above: add the pools, count cards whose text Defends/removes in the named lands |
| Fast plan — "rules in play that explain the numbers (Fear is 5: two boards; Fine Steel; Mining's Fast check)" | `RavageLedger.applied`, `rules_in_play`, `LossClock.checked` | Exact (facts) | Implemented; the Fear "why 5" reason is not spelled out | Add the fear-per-card derivation (4N, +1 extra board, +1 England L6) to `FearStats.pocket_rule` |
| **Event and Fear choices** — "the ledger rerun per legal choice (which push decides B8's counterattack)" | Ledger per enumerated option (depth-1 expectimax without the chance node: the choice is the player's) | Exact per option | **Missing** as a feature; `whatif` cannot move pieces | `Edit::Move` + an option enumerator for the Event's legal targets (lands with Dahan; adjacent lands) |
| Event and Fear — "what the unseen Events' token sections would do in each legal land (35 of 58 have a Beast section)" | Counting over the unseen Event pool by section | Exact | **Missing** (no Event pool) | Add the pool; tag sections (Beast/Disease/Wilds/Strife/Badlands) from card text |
| **After the game** — "where it turned, which choices were costly, the board at each decision; luck ledger" | `review.rs` (7.7) + spec 7.10 (decision points, flags F1–F10, counterfactual runner, luck) | Mechanistic attribution is exact; flags are pattern facts | 7.7 implemented; 7.10 **missing** (R1–R4); `review.rs::invader_damage` ignores Sweden L3 | Use `ravage_ledger` for pressure; join `log_parse` decisions to the snapshot before each; implement F1/F2/F4/F7 first (all computable from existing ledgers and the log) |
| Standing — "actual numbers, every adversary modifier" | `adversary.rs` hooks; `applied` list | Exact | Implemented (7.11 table) | Verify the unverified rows as games arrive |
| Standing — "next turn's four-land Ravage, loud when it empties the card" | `invaders::next_turn` + `BlightStats.next_turn` | Exact for the land identities; amounts a floor | Implemented | Draw T+1 as certain in the race (3.8) |
| Standing — "unfamiliar adversary: where its rules bite and when checked" | `rules_in_play` + `LossClock.checked` | Facts | Implemented as text | Attach the check step to each clock lane (deadline marker) |

---

## 5. Misframings to retire

1. **"Unanswered … lost by T5 in 9 of 10" as the Runway headline.** It is the Blight-only latent failure probability under a policy no one follows, read as a forecast. The report (lines 167–169) names this error exactly. Replace with the competing-risks race of Section 3: per ending, per turn, first arrival wins; T+1 and T+2 drawn as certain; the no-action label on the panel, not in a tooltip; the what-if delta beside it.

2. **"If nobody acts for four turns" as the frame of the Runway.** Brett's words: "Not 'nobody acts for four turns'" (`questions-from-play.md`, model table). The exact enumeration is right and should stay; its *framing* becomes "what is coming at the lands" (the land-tempo and timeline layers) and the stakes line under a plan, where "unanswered" is the floor against which the plan's delta is shown.

3. **`lost_by` counting Blight on paths that already ended** (deck out, adversary clock). A CDF that keeps rising after the game is over. Censor at the first ending (3.2).

4. **"WHO'S GROWING FASTER as multiples of Turn 1" with a 2.5 (1.5–3.5) Energy-per-play scalar.** Rei's rate is contested in its own thread (report line 75–78); the report allows it only as a user-set parameter, never as a chart's premise. Multiples of a near-zero base are meaningless (Shadows at Turn 1). Replace with the per-growth-option natural-unit table (Section 2, curve.rs) and the invader side in "lands that act / damage / Blight per turn" from the enumeration.

5. **"Lands named" as the invader step.** Terrain footprint is not pressure. Use the enumeration's acting lands, damage and Blight per turn, certain part solid, uncertain part as a band over orders.

6. **Mean of the last three finished turns as "your pace", and the "reach" turns derived from it** (Pace: "Last 3 turns: 3, 5, 6 (4.7 a turn). At that pace: Terror III on T9"). Non-stationary, policy-driven, n = 3, no sampling model. Keep the marks, the realized dots and `need`; show reach only as scenario lines at named rates; the only forecast on the lane is the guaranteed floor for this turn.

7. **Loss-clock shortfall as "prevent this turn"** for a clock checked at the end of the Fast Phase. The deadline is the end of *next* turn's Fast Phase when `after` reaches the limit. Add `deadline_step` per clock; compute the shortfall against it; say "you have next turn's Spirit and Fast phases".

8. **Blight budget `shortfall` as one number for three stakes** (flip / redraw / loss). Name it "to keep this card"; put the loss stake on the mixture.

9. **Fear budget to the next card as the goal.** The goal the questions name is the next divider (Terror Level) and the deck; keep the card as the unit but set `left` and `size` to the divider.

10. **`next_unanswered` vs `next_actual` in `outlook_check`.** Unfalsifiable; it conflates spirit action with model error. Replace with the after-Fast-phase one-sided check (Section 6).

11. **Hard-coded land health 2 in pocket rules; hard-coded 1/2/3 damage in `review.rs::pressure`.** Both contradict "show the ACTUAL numbers" (Sweden L3, England L5, Vitality). Read health and damage from the ledger.

12. **Silent `.min(12)` / `.min(16)` subset caps.** Turn into an `unmodeled` note when exceeded.

---

## 6. Verification plan

Captures: `/home/brett/repos/spirit-island/data/steam-capture/{live-27696743c529 (Green/Russia5, lost T6 Beasts), live-0d0b30b99a33 (Ocean/England3, won T8 Fear), live-a251b07f4c4e (Thunderspeaker/Sweden4, lost T7 Blight), live-cb0788649796 (Vengeance/Prussia2 Blitz, lost T4 Blight), live-bae43cb355ed (Lightning+River/Habsburg Mining4, lost T4 Land Stripped Bare), live-3de87de1fa52 and live-5ec66f090060 (partial), live-0494e5c4ee45}` and `/home/brett/repos/spirit-island-steam-companion/data/steam-capture/{2026-10-18-test1 (Shadows/Prussia2, lost T7 Blight), live-39ab9d8f115f (Shadows/Prussia6, lost T6 Blight)}`. Tools: `si-steam-dump backtest|outlook|review|stats|timeline <capture>` (`crates/si-steam/src/bin/dump.rs`).

### 6.1 Deterministic layers (no policy needed)

- **Ledger and rules** (`backtest`): already 7/7 games, 0 mismatches at the Ravage-step save. Keep as the regression gate. Add the engine's own preview functions (`GetRavagePreviewForLand`, `GetInvaderActionPreviewForLand`, report line 28) as a second oracle on each save through the REA tooling, so the ledger is checked at *every* step, not only at Ravage entry, and with Defend/Event effects present.
- **Loss clocks**: for each game, the clock that ended it must read `now ≥ limit` (or `>`) at the ending save and, one step earlier, `after` must have predicted it. Habsburg (Mining T4) and Russia (Beasts T6) are the two positive cases; the other five are negatives (no clock fired). Add this as `clock_check` to the backtest.
- **Enumeration bookkeeping**: `in_rows` (the realized Explore card's kind was in the unseen pool; 40/40 so far) is the permutation-invariance check in disguise and must stay at 100%. Add the same for the Blight flip (the realized flipped card must be in `unseen_flipped`: 4 flips available, England T6, Prussia6 T3, Sweden T6, Blitz T2) and for every Fear card and Event once those pools exist.
- **Spec §5 rule 5**: extend the invariance fixtures to the `outlook`/race output (permute hidden identities within look-alike groups; require byte-identical `CompanionStats`). Without this there is no proof the race does not leak.

### 6.2 The race (policy-dependent, but checkable in two places)

- **At the last decision point.** From the save at the ending turn after the Fast Phase (step `BlightedIsland`/`Event`/`Fear`, or `Ravage` for a Blight loss), the no-action CIF for the realized cause must be exactly 1 at that turn in every order and every other cause 0: at that point no spirit action remains and the Ravage is deterministic (Events and Fear cards aside). Seven games, seven hard checks. For the Habsburg game the check is at Turn 4's Spirit-phase save: `CIF_Mining(T4) = 1`.
- **Dominance before that.** Under a no-action policy the realized ending turn should be ≥ the floor's ending turn *unless* an Event or Fear card accelerated it; both are named in the log. Report per game and turn: floor ending (turn, cause), realized (turn, cause), and the log's Event/Fear-card interventions. This is a one-sided sanity check, not a score.
- **Not a proper-scoring target.** The race is P(ending | state, no action), which is not a forecast of what happened, so Brier/log/time-dependent Brier (Graf) against outcomes would score the policy, not the model. Do not do it.

### 6.3 The exact probabilistic pieces (scoreable in principle, under-powered now)

- **Explore kind**: the predicted distribution is uniform over unseen kinds *by construction*, so a log score against realized kinds only re-checks the pool. Still worth running once (≈45 Explores) as a PIT histogram to catch a bookkeeping bias (e.g., a card wrongly marked seen).
- **Flip card**: log score of the realized flipped value under the predicted mixture; 4 observations, sanity only.
- **Need finder**: for each Minor/Major draw in the logs (`RevealCardsAction`), check that the 4 revealed cards were in `minor_unseen`/`major_unseen` (bookkeeping), and score P(≥1 saver) against whether one was revealed (≈12 draws across games; expect calibration to be untestable, bookkeeping to be perfect).
- **The learner**: the proper scoring rule in this project is for Brett's guesses, not for outcomes: expected Brier excess (q − p*)² against the exact p* (report line 195, App. A #15). It needs no outcomes and is available the moment the hold toggle exists.

### 6.4 Pace and curve

- `need` and `marks` are arithmetic; unit-test against the England win (fear 13 cards resolved by T8: at every turn the realized total vs `marks` and `need`).
- Score the retired "recent pace" reach predictions against realized Terror Level turns across the 7 games before deleting them, so the retirement is on the record as evidence, not taste. Expect large misses; that is the point.

### 6.5 What cannot be verified without a play policy

Anything that compares options (7.5 expectimax outputs, draft lens "what it accomplishes"), the Fear-win scenarios, and the board-clear route. For these the only honest checks are internal: the ledger per option re-derives from the same `ravage_ledger` the backtest validates; the option enumerator lists every legal choice (compare against the engine's `Ask` options in the log where present); and the display never ranks.

---

### Closing note on the display contract

Every number above has a natural-frequency form (orders, cards, Blight, Fear), a delta form (base → tried), and a pocket rule ("1 ÷ unseen cards in this stage", "Defend = damage − health + 1", "fear to the divider = cards above it × 4N − generated", "the deck is the turns left"). The two outputs that do not have one, the 2.5-rate growth multiple and the mean-of-three pace, are the two to retire. The one output Brett asked for by name, the need finder, has all three forms and is not on screen.
