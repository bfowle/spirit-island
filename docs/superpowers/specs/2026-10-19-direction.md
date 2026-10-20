# The direction (2026-10-19)

Design of record for the Spirit Island Steam companion (`/home/brett/repos/spirit-island-steam-companion`, workspace `tools/si-live`, branch `companion-v2`). Written by the cohesion referee from Brett's own messages (both sessions, including the mid-turn ones), the five reviews (A intent, F gameplay model, B models, C information design, E code state), the four Sly screenshots Brett attached to his reset request, the specs, the playtests, and the built screens. Where a review disagrees with Brett, Brett wins; where reviews disagree with each other, the choice is stated with its reason.

Quotes are verbatim, typos included. Dates: `2026-10-18` is session 191ccfe4 (Test 1, Shadows vs Prussia L2, played step by step); `2026-10-19` is session 05faae41 (rules Q&A, the live Prussia 6 game, five coverage games, the native reset).

## 0. How to use this document

This is the design of record: a future session opens it cold, builds the next step in §9, and never builds what §2 and §6 forbid. It supersedes the 2026-10-18 spec's §2 "Always-on dashboard. Every number visible… dense layout" and §8 (the Vue dashboard), the native-app spec's §3 four-region layout and its N2–N6 milestone list, the Ledger/Runway/Pace/Curve/what-if layout in the uncommitted code, and three memory-file sentences corrected in §8; it keeps the 2026-10-18 spec's §1 goal, §5 firewall, §7.1–7.11 models and principles (with the corrections in §5), the native-app spec's §1 goal (one screen, no scroll, native Rust, the Sly look), §2 architecture and §4 toolkit, the corpus spec, the questions-from-play doc (promoted, with its layout conclusion amended by §6), and every memory file except as corrected. Any change to this document is made by quoting Brett, with the date, next to the line it changes; a session that cannot quote him does not change it.

## 1. Purpose, in Brett's words

- **2026-10-18 · 13:18** (founding): "lets morph / utilize this app to be the 'spirit island companion' showing percentages, statistics, current state in various breakdowns/visualizations/inspectiosn in order to LEARN the stats/math around the game and help you think about probabilities and strategies throughout gameplay. not a cheat, at all!!!"
- **2026-10-18 · 14:04**: "I don't care about 'statistics' the makers of the digital game collect (just a count of wins/losses this information is useless to us), the win/loss doesn't tell us much to our all which is more around the specific scenario and rules for and against the player at the given moment."
- **2026-10-18 · 16:18**: "It needs to show specific scenarios and probabilities and helpful knowledge on direct questions like the one we're asking about a given phase (e.g. Invader phase -> Ravage) given ALL the things we can a.) directly affect and control b.) that we can't directly affect but can take into account for the things we can ... And also it should lay out the comparison of how we're growing and keeping pace in either direction. Vs outgrowing an adversary especially."
- **2026-10-18 · 17:17** (augment, don't decide; the back-and-forth is the input): "we need the companion to be... helpful/augmenting certain choices with math/stats, and PRESENTING things to THINK ABOUT during these decisions, but not doing it for them/making the calls directly for them. you adn i here, are doing back and forth to help figure tehse choices out to present this information for the tool, thats fine"
- **2026-10-19 · 19:54**: "re-thina bout from the... lumia + rei approaches of showing statistical/analyical thigns ina reasonable/companion way aside from the game running"
- **2026-10-19 · 20:59**: "I would like the math and stats that are pulling similar to lamia and rei, and convey that in a helpful and visually pleasing way to augment what players probably already know about the game but can't quite always grasp"
- **2026-10-19 · 22:00**: "Think entirely from the perspective of a player who is looking to augment their gameplay. And learn. And tinker"
- **2026-10-19 · 22:47**: "What about the statistical model you mentioned around competing-something... This is the basis of this game where so many things come into play and compete for attention"
- **2026-10-19 · 23:03**: "I mean we don't need to reinvent an entire game inside of a companion for a game. Similar/like concepts are shared. The visualizations of the statistics, models, analytics, questions and solutions; the back one of the competing goals is what is key here"

The companion is a second window, in Rust, that reads the running Steam game through an information firewall and sits beside it. At each decision it answers the direct question the player is asking at that moment, separating what is fixed, what the player can actually do, and what is hidden but countable, and it lays the player's growth against the adversary's pressure so the player can see which ending is arriving first and what moves it. Its idiom is Lumia's ledgers and Rei's curve; its numbers are counts over public pools with a pocket rule beside each; its posture is to present things to think about and never make the call. It exists so Brett learns the math and builds the habit of pausing to evaluate. It is not a cheat, not a move ranker, not a restatement of the game's own screen, not a port of the old tabletop panels, not a re-implementation of the game, and not a win/loss counter. The content of its screens comes from the questions Brett asks in chat while playing real games; one view is proposed, built after his yes, used by him in a real game, and only then the next.

## 2. The contract: principles that cannot be eroded

Each item: one sentence, Brett's quote(s), and where it is encoded. "(derived)" marks a principle from Claude's research that Brett accepted without objection; it binds but does not carry his authority. A §9 step that breaks any item is not built.

### What it shows

1. **It sits beside the running Steam game, read-only, and answers direct questions at a decision.** "lets hook into whenever you're running the digital version of hte game on steam" (2026-10-18 · 13:18); "live data -> in a ui -> alongside the actual gameplay" (2026-10-18 · 19:03); "aside from the game running" (2026-10-19 · 19:54); "this is where the companion app would really help: a specific qeustion/need: what % of getting a minor-drawn defend card to help in C4... is this worth it, etc." (2026-10-18 · 15:59); "lets check waht our companion says for... helping pick Growth on this turn" (2026-10-19 · 15:19); "looki nto wwaht kind of advice/information teh comapnion woudl have at this point" (2026-10-19 · 15:45). Encoded: spec §1; native-app §1; spec §7.5; questions-from-play.
2. **Every screen separates fixed facts, levers the player controls, and unknowns that can be counted.** "given ALL the things we can a.) directly affect and control b.) that we can't directly affect but can take into account for the things we can" (2026-10-18 · 16:18). Encoded: spec §7.5 three columns; questions-from-play. Not yet encoded in any built view.
3. **A lever is something the player can actually do this phase: a growth option, a card in hand with a target, which track, an Event choice, a Fear-card target.** Brett's levers, in his own play: "G2 (gain a MAJOR power...)… or we reclaim... and gain a minor/major then?" (2026-10-18 · 17:04); "Lure can save C2 this ro und" (17:08); "i placed concealing in C3" (15:51). Spec §7.5 lists them. Encoded: spec §7.5. The built what-if (destroy any piece) is not a lever; see §8.
4. **Growth vs pace, and outgrowing the adversary, is always laid out.** "it should lay out the comparison of how we're growing and keeping pace in either direction. Vs outgrowing an adversary especially" (2026-10-18 · 16:18); "eventually the ramp and acceleration curves / comparison of how fast we grow vs the opponent" (2026-10-19 · 20:58); "Rei's specific kidn of thinking, around momentum and keeping ahead of the curve" (2026-10-18 · 14:51). Encoded: spec §7.4, §7.5 output 4.
5. **Reclaim timing is strategy: show the draw odds and the growth gained if the draw misses.** "reclaim does NOT accelerate our growth/pace... postponing Reclaims is a MASSIVE strategy proven in SI through and through… show me actual statistical view into % chance of getting a defend card. and if we dont get one, how helpful is that acceleration of growth" (2026-10-18 · 16:02); "im leaning Reclaim + gain Major; i have 5 energy, and +3 each round... so the enegy economy is starting" (17:08); "we should relcaim, 0 cards in chand" (2026-10-19 · 15:36). Encoded: spec §7.5 worked example; questions-from-play ("order question, not count question"). No code computes a per-option Reclaim schedule.
6. **Every hidden pool that touches the step is counted: Minor/Major draw, banked Fear cards, the Event, the Blight card's other side, the next Invader card.** "What about factoring the 1-2 fear cards we'll have available and the Event card" (2026-10-18 · 16:08); "given what we know about cards left in private information (only have 4 cards shown in the minor pile so far)" (16:02). Encoded: spec §7.5 unknowns row, §7.2, §7.3. Fear and Event pools are not in `PublicSnapshot` (§4).
7. **"Saves the land" means any effect, not the Defend keyword; a new card is judged by what it does on this board this turn, what it pairs with (hand vs discard), and what the Forget costs; elements and innates are a core value.** "Lure...actually solves C4 by gathering out of 4 -> 5 ... its not a defend card but it works for the blight, or cycles of time... removes a blight" (2026-10-18 · 16:54); "Lure also gives us Shadow elements; this is a massive thing about SI, keeping 'on element' cards, and using innate powers" (16:55); "with this major we get X... when we forget Y we gain/lose this... but also the new card [major/minor choice] pairs with <other cards in the hand we have>" (2026-10-19 · 15:26); "and also comparing with what's readily aviable this turn vs in the discard" (15:26, mid-turn); "what a new card this round/turn would accomplish at all in teh current situation ont he board" (15:27); "what is the best card to forget" (15:24); "we have a defend 5... but off-element. we have lure-- entirely on element" (15:38). Encoded: spec §7.4 three values and need finder; playtest U13; questions-from-play.
8. **Beasts, Events and token placement are a decision; pacing effects of Events and Escalations are surfaced.** "spiders, for beast economy (these come up in a LOT OF EVENTS…)" (2026-10-18 · 17:19); "beasts and events are quite powerful. deciding where to put these is key" (2026-10-19 · 15:31); "Promising Farmland adds a town instead of an explorer on the explore phase... taht's a mechanic that changes pacing quite a bit" (15:30, mid-turn). Encoded: playtest U14, U15; questions-from-play. Not in code.
9. **The pipeline invariants are obvious to the companion; Fast solves this turn, Slow sets up next turn; inland lands can be made unexplorable, coasts cannot.** "Explore->Build->Ravage for each ivnader phase across turns; resolved in reverse order… fast powers solve things 'this turn'. also things like: clearing inner-lands prevent Explores due to adjacency, whereas coasts always explore... this should all be obvious to the companion" (2026-10-19 · 03:29); "T2 is generally when you see the first ravage. making T1 a nice time to prevent builds" (13:41). Encoded: spec §7.8; `invaders.rs`.
10. **A clock that decides the game a turn ahead is shown loudly, with when it is checked; an unfamiliar adversary's rules are explained where they bite.** "T3 roud land ravage, not really didnt see that coming" (2026-10-19 · 18:42); "not really clear, im not familiar with Habs Mining at all" (19:12). Encoded: spec §7.11; Habsburg playtest H4; the deleted `views/rules.rs` (no replacement yet).
11. **Every number is the ACTUAL number the engine will produce, with every adversary and scenario modifier applied; player choices are listed as options.** "yeah i mean these shoudl show the ACTUAL things like sweden's extra dmg..." (2026-10-19 · 18:11). Encoded: spec §7.11; `feedback_companion_actual_numbers.md`; `adversary.rs`.
12. **Pace covers every way the game ends, not Fear alone.** "Is the pace only fear victory? Or are you accounting for tier 1-3 wins too" (2026-10-19 · 21:20, mid-turn). Encoded: `pace.rs::BoardRoute`; memory ground-up file. The board-clear route is a count, never a probability (§5).
13. **Post-game review: where momentum shifted, which picks were costly, judged by what was knowable then, luck separated, flags not verdicts.** "evaluating post-game things; e.g. where did the momentum shift, where were suboptimal picks made… too much focus on ravages and not preventing builds (or explroes if possible)" (2026-10-18 · 19:03); "retroactive, turn by turn evaluateion of waht went wrong; what was a suboptimal choice...what exactly we saw on the board then decided that didnt go our way. we should see this in a helpful timeline and brekaout. show me this first in the plan/chat here, and then paln fro it for realt oa dd to the companion" (2026-10-19 · 15:55, mid-turn). Encoded: spec §7.7, §7.10; `review.rs`; the Prussia 6 retro table (reference answer).
14. **It shows derived math the player cannot grasp at the table, never a restatement of the game's own screen.** "I guess. It shows basically the info we already know ok the game... But it's a start." (2026-10-19 · 20:58); "augment what players probably already know about the game but can't quite always grasp" (20:59). Encoded: memory ground-up file.

### What it never shows

15. **No rankings, no best move, no highlighted option, no default selection, no verdict sentence; options in a fixed order; deltas, not verdicts.** From item 1's 17:17 quote (derived rules: spec §7.6 "No rankings, no 'best move', no highlighted option, no default selection"; report "show 'what changes if you hit land X,' never 'play this'"). Encoded: spec §7.6; `feedback_deliberate_play_muscle.md`. Violated by `ledger.rs::verdict` and `curve.rs` "pulling away" (§5, §7).
16. **Not a cheat: only what a player at the table could know; every derived number samples hidden orders from the public belief, never the save.** "not a cheat, at all!!!" (2026-10-18 · 13:18). Encoded: spec §5 (rule, zone table, invariance tests); report corollary; `steam/tests/firewall.rs`. Intact.
17. **No aggregate win/loss statistics; no fitted rates; momentum cannot be detected statistically inside one game.** 14:04 quote in §1 (derived: spec §7.2 "observed values, not a fitted rate"; report §7.9). Encoded: spec §7.2; report. Violated by `pace.rs::recent` and `curve.rs` growth rate (§5).
18. **Any Energy-per-card-play valuation is a user-set parameter with a sensitivity band, never a fact; natural units otherwise.** (derived) Encoded: spec §7.4; `common/.../curve.rs` header. Violated by `app/views/curve.rs` (§5).
19. **Stochastic framings only where they fit.** "i only want things taht DO FIT for this tool/compnaion,just mostly curious" (2026-10-18 · 19:03). Encoded: spec §7.9.
20. **Never reinvent the game inside the companion: no island map as a panel, no piece editor, no hand panel, no log stream, no tabletop tracker in the companion's screens.** 23:03 quote in §1; "the layout is horrendous... its so much info smashed together… the island view is so big i have to scroll vertically to see it… paly by play doesnt show undos/overwrite that info its just a stream of logs" (2026-10-19 · 15:07). Encoded: nowhere until now (add to the spec). A gltf island (native-app N6) is not ruled out by Brett ("winit+gpu+gltf+glyphon") but is not built before the first screens; see §10.

### Display

21. **Natural frequencies with a pocket rule beside every number; percentages allowed beside the count, never alone.** Brett's own vocabulary is "%" ("what % of getting a minor-drawn defend card", 2026-10-18 · 15:59; "% chance of getting a defend card", 16:02); the research adds counts ("2 of the 3 unseen Stage II cards", not "67%") and pocket rules ("1 ÷ unseen cards in this stage"). Resolution in §8. Encoded: spec §7 preamble, §7.6; report "Displays that build judgment".
22. **Distributions are drawn, not summarized: a dot strip over turns (one dot per tenth of the enumerated orders), never a mean, never a saturation zone.** (derived; report: quantile dotplots; Kay/Fernandes) Encoded: report; C §3 rule 4. Not in code.
23. **Visualization trumps text; a chart that needs a caption is redrawn.** "with all that stack (winit, wgpu, gltf, glyphon, etc.) we should lean towards much more helpful, visual, cool looking things" (2026-10-19 · 19:44); "come up with great visualizations along the way" (20:05); "Add visualizations, graphs" (20:58); "How do we best visualize this, lay it out, for a human brain to comprehend all these things" (22:50); "design paradigms for UX/UI design for such things; where visualization trumps tons of text explanation" (22:58). Encoded: nowhere (add to the spec).
24. **Think-about prompts are on every decision screen; an optional off-by-default "hold" may exist.** 17:17 quote; `feedback_deliberate_play_muscle.md` (Brett, 2026-04-19: "how to take a step back per turn, per phase, per instruction and think of what the board state is… HOW CAN I ACTUALLY CHANGE THIS GAME STATE"). Encoded: spec §7.6 catalog; `companion_stats/mod.rs::prompts()` (computed, never drawn in the native app).

### Layout and look

25. **One screen, no page scroll, on a monitor; it must be obvious what the app is trying to do to aid you.** "i imagine a single screen, not much scrolling (if at all) if it's pulle dup on a monitor" (2026-10-19 · 16:02); "its so all over the place, there'e SO MUCH INFO showna t once, it doesnt read well at all. i cant tell waht its actually tryign to do to aide me" (16:02). Encoded: native-app §1.
26. **The look is Sly's: a dark grid of panels, each one named instrument with a header and a count, monospace values, one accent, a status bar, sparkline rows on a shared axis, Linear|Graph-style toggles; dense is fine when every panel is one named thing in one grammar.** "redesign entirely in Rust-- with a slick winit+gpu+gltf+glyphon UI/UX see: https://github.com/ioncodes/sly -- [Image #6] [Image #7] [Image #8] [Image #9] -- for inspiration" (2026-10-19 · 19:3x, mid-turn); "Feel free to use those crates docs and examples and stuff people have build with them as inspiration of what's possible" (21:02). Encoded: native-app §1 ("dark, dense but ordered panels, panel headers with counts, monospace values, one accent color, a status bar"). Reconciled with item 25 in §6: what Brett rejected was panels without a shared grammar ("smashed together"), not the count of panels.
27. **Visual fidelity to the game where the game's objects appear: terrain colors matched to Steam, a Defend or a played card shown on the land it touched, undo-aware history.** "mountain card is 'red' in color? its grey on the game" (15:07); "in B8 we placed a defend with a card, the companion doesnt show anything right now on defend... or that a card was applied at all to any specific land" (15:12); "paly by play doesnt show undos" (15:07). Encoded: playtest U4, U5, U8; native-app §3 (retired layout). Survives as land-chip badges and as the "what happened vs forecast" moment, not as an island panel or a log.
28. **Ground up from the Rust structs and data; no retrofit of the old panels.** "the idea wasnt to retrofit all the randomass shit we built months ago on that so we need to rethink how / what to build from the ground up, given all our rust structs and data and such" (2026-10-19 · 19:52); "feel like you're trying to retrofit the old stuff in with teh new stuff; i feel like we need to go back to the drawing board on...the latest plans… one at a time" (19:49, mid-turn). Encoded: memory ground-up file.

### Models

29. **Lumia's ledgers and Rei's curve are the idiom; competing goals are the backbone.** "these blog writeups are the kind of theory/way of thinking i want to be able to approach this; alogn with Rei's specific kidn of thinking" (2026-10-18 · 14:51); 22:47 and 23:03 quotes in §1. Encoded: report; spec §7 preamble; memory ground-up file. The attention sense of "competing" is encoded nowhere until §3 of this document.
30. **Exact counting and enumeration over public pools first; simulation only for races and comparisons, with its policy labeled; never compute a clock in isolation.** (derived; report "Simulation is for races and comparisons, and its estimand must be labeled"; "Clocks must not be computed in isolation") Encoded: spec §7.2 race chart v1; report. Violated by `outlook.rs::lost_by` (§5).
31. **Adversary rules, expansions, and player/board count are factored into every number.** "Are you also being sure to factor into these 1. Adversary rules 2. Expansions 3. 1 vs 2 vs N number of spirits and/or adversaries" (2026-10-18 · 14:25). Encoded: report; spec §7 (N vs B units), §7.11.

### Rules fidelity

32. **Printed card text governs unless explicit errata; rules questions go to Brett; never hallucinate a mechanic; the engine is the oracle for what the digital game will do.** "always assume the physical/card printed effects, UNLESS tehre's errata explicitly updating these, which the steam version has" (2026-10-19 · 14:34); "If you ever have any rules questions let me know I can clarify them or write them to a document that I can clarify in bulk" (2026-10-18 · 15:05); "we should probably review the rules in some kind of way especially whena dding a bunch of layered expansion on there" (15:10). Encoded: `feedback_printed_card_authority.md`, `feedback_rules_questions_to_brett.md`, `feedback_never_hallucinate_mechanics.md`; rules register; corpus spec.
33. **Everything the project knows about the game is machine-readable data first, so later work rests on data.** "would it make much sense to be able to digitize all the SI panels, cards, rules, etc. in a LLM friendly data driven way/form first....to allow for all other things?" (2026-10-19 · 14:27); "errata (layer 4) could also belong ot the individual item right? like rule/card/etc." (14:48, mid-turn). Encoded: `2026-10-19-si-corpus-design.md` (merged to main).
34. **The companion never writes game state, Steam's or the tabletop's.** Brett's pasted plan: "Separate data/steam-game.json, so your tabletop game is never written" (2026-10-18 · 19:03); `feedback_never_put_live_state.md`. Encoded: spec §2; native-app §5; E §6 verified no writes.

### Process

35. **One view, Brett's yes, he plays a real game with it, then the next.** "yeah lets start w/the ledgeer --- lets restart all the window we're building" (2026-10-19 · 20:05); "What do we want to test out first in the entire plan of these things." (21:57); "lets start off one...at least ONE item for the companion guide" (19:49, mid-turn); "we'll also do some play by play throughout it to see live data -> in a ui -> alongside the actual gameplay… we can then runa ny other scenarios you wanna see throughout" (2026-10-18 · 19:03); "if that's playing multiple games with the companion app to collect real scenarios and data that way" (2026-10-19 · 14:27). Encoded: memory ground-up file ("Propose one view, get Brett's yes, build it, let him use it, then the next": the memory's wording, consistent with every quote above). The evening of 2026-10-19 broke it (four views in one pass; A's E3).
36. **Prose before pickers: when the framing is wrong, explain the direction; do not offer multiple-choice menus.** The memory file states this; no verbatim message says it. Brett's closest words: "I feel like I'm stuck explaining it to you so what should we do here......" (22:35); "show me waht that plan is again" (19:49). Kept as a working rule, labeled inference; §10 asks Brett.
37. **The chat during real play is the design input; the questions from play are the catalog of screens.** 17:17 quote; "Recall when you and I would do things step by step in actual gameplay and I'd ask questions and get ideas for stuff to show? We should have those in context somewhere and should look at how that'd fit the ledger/pace questions, approaches, and statistical analysis/models" (2026-10-19 · 22:38). Encoded: questions-from-play; one sentence in `feedback_deliberate_play_muscle.md`.
38. **The direction is laid out in three layers: the Rust conceptualization of the gameplay, the statistical models, the UX/UI on top.** "Look where we're at on... rust based conceptualization of the gameplay, the data/ structures/ everything; the statistical models, concepts, what we want to be able to answer with math/stats; the ux/ui as a layer of those things^" (2026-10-19 · 23:07, mid-turn). Encoded: this document's §4, §5, §6.
39. **Rust for every component; REA for the engine model.** "Why a C# version of the process inspector route? We should use rust and @~/repos/rea" (2026-10-18 · 13:31). Encoded: spec §2.

## 3. The backbone: competing goals

Brett used "competing" in two senses on 2026-10-19 (22:47 and 23:03), and both are the backbone. They are one structure seen at two scales: the statistical sense is the race between endings over turns; the attention sense is the same race read at this phase, where it tells the player which lands and which plays matter now.

### (a) The statistical sense: several endings race, the first to arrive wins

Endings in play and the step at which each is checked (engine state order; `invader-rules.json` "state_ordering"; NI errata: losses are checked once no triggered actions remain):

| Ending | Side | Checked at | Determined by |
|---|---|---|---|
| Adversary loss clock (England capital, France Towns, Scotland coastal Cities, Habsburg Mining load, Livestock, Russia Beasts) | Invaders | the clock's own step: Mining at the **end of the Fast Phase** (deferred: a load reached in turn j's Invader Phase is lost at the end of turn j+1's Fast Phase); Russia/Livestock at the Ravage that changes the count; England/France/Scotland after Build and Escalation | board counts per path; exact given the policy |
| Blight card empties on its blighted side | Invaders | each Ravage (and cascades) | per-path Blight count × the hidden-side mixture (`mod.rs::p_loss`) |
| Invader deck exhausted | Invaders | the Explore with no card | the deck recipe: deterministic (administrative censoring at a known turn) |
| Spirit with no Presence | Invaders | after Blight/cascade placement | per-path Presence under the routing policy; Blighted-side per-phase effects a labeled gap |
| Fear victory (Terror III + deck empty; "Victory when revealed") | Spirits | the Fear step | player-driven; never a probability (scenario lines only) |
| Board clear at Terror Level I/II/III | Spirits | after any destroy | player-driven; blockers as a count per turn, never a probability |

The math (B §3, using the enumeration that already exists in `outlook.rs`): fix the policy (no Spirit action; cascades routed to the nearest unblighted land and placed on the board; players take the least harmful option at Escalations and rule actions; Events and Fear cards not drawn; flip Immediates not applied) and enumerate every order of the unseen Invader cards, grouped by kind, weighted by the count of that kind in the stage's pool. Along one order p everything is deterministic except the Blight card's hidden side. Per path record the first (turn, step) at which each deterministic ending fires, `a(p)` the earliest of them, and `cum_j(p)` the Blight taken off the card through turn j's Ravage. Then:

- Blight: `CIF_B,p(j) = p_loss(cum_{min(j, t*)}(p))`, flat after the last Ravage strictly before `a(p)`.
- Deterministic ending a: `CIF_a,p(j) = (1 − p_loss(cum⁻(p))) · 1[j ≥ t_a]`; zero for every other deterministic ending on that path.
- Over paths: `CIF_cause(j) = Σ_p w_p · CIF_cause,p(j) / Σ_p w_p`; `S(j) = 1 − Σ_cause CIF_cause(j)`; the hazard `h_cause(j) = (CIF_cause(j) − CIF_cause(j−1)) / S(j−1)` answers "is this the dangerous turn".

No simulation: a few hundred leaves at horizon 4; the mixture is closed form. **Exact**: order counts and weights; the deck-out turn per path; each deterministic clock's first crossing given the policy; the Blight mixture given a count; the required Fear per turn; the card pools and draw odds. **Labeled assumptions**: the policy above; the horizon. **Never a probability**: P(win), P(Fear victory), "your pace" with an error, any blend of the floor with a scenario rate, the latent Blight-only `lost_by`, a combined "P(loss)". **Censoring**: a Blight CDF stops rising on paths that have already ended; T+1's Ravage lands are certain (the Build card is face up) and T+2's identities are certain once the Explore card is up, so the first two turns are a ledger, not a hazard, and the fan opens only at T+3.

Per-path state the walk must carry (today it merges on blight/invaders/lands and loses all of this): max Town+City in one land, total Towns, coastal lands with a City, max Invaders+Blight in one land, 6+ overkill Blight, Beasts destroyed and on the island, Presence per spirit, guaranteed Fear (counterattack best case), the acting-land lists per step, and the step within the turn at which each threshold is crossed.

### (b) The attention sense: lands, cards, Energy and plays compete for a few actions each turn

At any phase the player has a fixed number of card plays, a fixed Energy, a hand, and innate thresholds; the Ravage lands, the Build lands, the Explore sources, the Fear divider, the Blight card and the adversary's clock all want those actions. Which clock binds (the ending whose dots arrive first in (a)) decides which lands matter this phase: a land that feeds the binding clock is worth an action; a land that is "doomed anyway" or only feeds a clock that arrives later is not. The lever is a real action (item 3), and its value is the delta it makes to the binding row and to the rows behind it. This is what Brett was doing by hand on Test 1 Turn 4 ("you can probably start seeing hnow theres so many leveers to pull on, with so many mechanics all comign together", 2026-10-18 · 17:17) and on Prussia 6 Turn 5 ("this is grim", 2026-10-19 · 15:45).

### How every question hangs off it

| Question (questions-from-play) | Hangs off the backbone as |
|---|---|
| Growth: which option? | the option's plays as a lever against the binding row this Ravage; the draw, Fear cards and Event as counted unknowns on the same rows; the option's economy against the lands the deck hits per turn (Rei), with the Reclaim turn as position on the turn axis |
| Draft and Forget | each candidate resolved at its speed against the forecast board: which row it moves; elements toward innates as the second value; the Forget as a row lost |
| Fast: my plan vs this Ravage | the plan as lever columns; "do nothing" as the floor; rows = the race endings first, then Blight after this turn, Dahan and Presence kept, Fear earned, Energy/plays left |
| Event and Fear choices | each legal choice as a column, same rows |
| Slow | which row T+1's ledger moves |
| Pace / outgrowing | the Spirit-side rows against the Invader-side rows on one axis; the turn, if any, where the player's rows overtake |
| Retro | the race strip replayed per turn; where the binding row changed hands; luck in its own ledger |
| Actual numbers, loss clock a turn ahead, unfamiliar adversary | each clock is a row with its check step drawn as the deadline tick; its rules in play are the row's flyout |

### The canonical illustration: Habsburg Mining Expedition L4, Lightning + River, thematic NW+NE, Turn 3 → Turn 4

Facts from the capture (`live-bae43cb355ed`, playtest H4, and the `lab-0-drawer.png` screenshot of T3's Fast step): Mountain NE7 holds 4 Explorers and 2 Towns = 6 at Turn 3's Fast Phase. The Build card, face up, is Mountain; NE7 has 3+ Invaders, so under Ceaseless Mining (L1) the Build card Ravages there instead of Building: 8 damage against health 2 adds 1 Blight (NE7 becomes 7, certain in every order). The 8th piece can come from Turn 3's Event (hidden; in the real game it did) or from Turn 3's Explore if the card matches Mountain (1 of 5 unseen Stage II kinds; NE7 is its own source). Land Stripped Bare is checked at the **end of each Fast Phase**, so the first check after that is the end of Turn 4's Fast Phase. The Blight card had 3 left and Turn 3's Ravage spends 3 (NW3, NE4, NE7): it flips in every order unless one is prevented.

- **At T3's Fast Phase** the race reads (computed 2026-10-19 by `si-steam-dump race live-bae43cb355ed 164`): Land Stripped Bare at the end of T4's Fast in 12 of 60 orders (the Explore card alone: Mountain, 1 of 5), 59 of 100 by T5's; the Event's share is not computable until the Event pool is projected (§4, change 1) and is drawn as "not yet counted". Blight: the card flips this Ravage in every order; the island is lost to Blight by T4's Ravage in 41 of 100 futures (the hidden side's mixture), and that row stops rising there because the orders where Mining fires at T4's end-of-Fast come before T4's Ravage within the turn (censoring); together the two rows reach 100 by T5. Deck: T11's Explore, every order. Fear victory and board clear: not before the deck with no Spirit action. Which clock binds depends on the hidden 8th piece; NE7 feeds both the Blight row and the Mining row, so it is the land that matters.
- **At T4's Spirit Phase** NE7 is 8, face up; the Mining row is 72 of 72 orders at the end of this Fast Phase (computed: `race … 193`); the Blight row is censored to zero; the only lever that moves the binding row is a Fast action in NE7 (Lightning's Swift Strike can make Slow Powers Fast with Air) removing one Invader or Blight before the Fast Phase ends. The built Ledger showed "LAND STRIPPED BARE −1 this turn · holds, 1 left after", which was the deadline misread (B §5 item 7); the built Runway showed "Lost to Blight by T4 6 in 10", the cause-deleted latent quantity (B §5 item 1). The delta the companion must show is: with one piece out of NE7 this Fast Phase, the Mining dots vanish and the Blight race reappears; that is the lesson, and the existing what-if recompute already produces it once the base quantity is the race.

B §3.9 wrote "At the start of Turn 3, Mountain NE7 held 7"; the capture says 6. The capture wins; a session drawing this screen re-reads it with `si-steam-dump` at `T3/FastPower` before drawing a number.

## 4. Layer 1: the Rust conceptualization of the gameplay

### What the model is today (F §1)

One `PublicSnapshot` per save file (`common/src/public.rs`), produced by `steam/src/projection.rs::project` from an `EngineSnapshot` (`steam/src/engine_map.rs::map`) plus card and spirit definitions read from the game DLL (`steam/src/engine_defs.rs`). Everything downstream is a pure function of that struct: `companion_stats::compute` → `CompanionStats`.

| Entity | Type | Notes |
|---|---|---|
| Island, land, piece | `PublicBoard`, `PublicLand` (terrain string, coastal, counts explorers/towns/cities/dahan/blight, presence map, tokens map, `pieces: Vec<PublicPiece>`, `damage_taken`, `defend`, `health_modifier`), `PublicPiece{kind, max_health, damage, strife}` | counts and pieces stored twice and kept in sync by hand; Incarna folded into presence; engine land distances decoded (`map_distances`) but not projected |
| Spirit | `PublicSpirit` (energy, plays, elements, hand/play/discard/forgotten/revealed as `PublicPowerCard`, `track: Vec<TrackNode>`, presence counts, `innate_powers: Vec<PublicInnate>`, special-rule text) | **no growth options** projected although the defs hold them |
| Power card | `PublicPowerCard{cost, speed, kind, elements, range, from_sacred_site, target, body: Vec<String>}` | effects are text only; `needs.rs::tag_options` scrapes keywords |
| Invader deck | `PublicInvaderDeck{upcoming_stages, explore, build, ravage, discard, extra_slots, pool}` | stages of face-down cards (backs) are public; odds = pool − seen |
| Fear | `PublicFear{generated, required_per_card, terror_level, earned_count, deck_by_level, discard, resolving, removed_unseen}` | **no unseen Fear pool** (engine `allowed_fear` decoded, not projected) |
| Events | `PublicEvents{deck_count, shown, discard}` | **no unseen Event pool** (`allowed_event` decoded, not projected); the reveal comes from the log |
| Blight card | `PublicBlight{flipped, remaining, on_island, healthy_side, blighted_card, …, pool: Vec<BlightOption>}` | the one hidden-card distribution modeled explicitly; flip "Immediately" effects are text |
| Adversary, scenario | `AdversaryRef{id, level, panel}`, `scenario: Option<String>` | rules are code in `companion_stats/adversary.rs` keyed by id strings; text and review status in `common/data/invader-rules.json`; only Blitz has scenario code |
| Status effects | `PublicStatusEffect{kind, source, source_kind, params}` | the engine's own modifiers (Defend, AdjustRavageDamage, CancelAction, ChangeRavageParticipation, SkipInvaderActions, PreventBlightCascade, IgnoreToken, Isolate, ModifyExploreRange, AdjustHealth), interpreted by `rules.rs`; params cross the firewall only when the source is public |
| Decisions, ending | `PublicDecision{turn, step, kind, text, land, card, ms, event}`; `ending: Option<String>` | whitelisted log lines; 11 ending kinds known, three with a forward model |

Transitions the code plays forward (`companion_stats/invaders.rs::forecast_at`, hooks in `adversary.rs`, rules in `rules.rs`): High Immigration; Ravage per land via `rules::ravage_ledger` (Strife, damage modifiers, every Defend variant, participation removals, Dahan protection, cancels, skips, cascade prevention, efficient Dahan destruction, counterattack, best-case kills by Fear, adversary after-ledger hooks); after-Ravage hooks; Build per card (`build_card`: Entrench, England L1, Scotland L3, cancels, Disease, Choke, Habsburg/France after-build, Ceaseless Mining diversion); Explore candidates and sources (range, Ocean, Isolate, Wilds); Escalations (England and Sweden applied, the option-based ones listed); `advance()` and `time_passes()` to the next turn; `adversary::loss_clocks` one phase ahead; `outlook.rs` enumeration of 4 Invader Phases over unseen-card orders; `whatif.rs` with four edits.

Firewall: `projection.rs` is the only reader of hidden fields; `steam/tests/firewall.rs::public_output_is_invariant_to_hidden_permutations` permutes hidden identities within look-alike groups and requires a byte-identical `PublicSnapshot`. Gaps vs spec §5: the invariance test covers `project()` only (not `CompanionStats`/`outlook`); the log-invariance test and zone-table assertion (enforcement 3) do not exist (`log_parse` is a whitelist); both capture-dependent tests skip on any machine without the capture and DLL.

What the backtests prove (`companion_stats/backtest.rs::backtest`, `steam/tests/backtest.rs`, and `si-steam-dump backtest` on all seven captures: 0 mismatches): the deterministic Invader resolution from the Ravage-step save given the revealed card and the public status effects: matching, Explore sources and range, Build piece, Disease/Wilds/Strife, Defend variants, cancels, England L1/L3/Escalation, Sweden L1/L3/Escalation, Russia L1/L3/Escalation, Mining L1/L3/Escalation, Blitz T1; `outlook_check` confirms the realized card was among the enumerated kinds (40 of 40). They do **not** prove: any probability; counterattack kills or Fear earned; Dahan destroyed; cascade targets and Presence destroyed; `advance`/`outlook` beyond the first phase; loss-clock values (Mining verified by narrative only); Event/Fear-card forecasting before resolution; anything at a step other than Ravage→Slow; France, Scotland, Livestock; Isolate; Avarice.

### Gaps, ranked by the questions each unlocks (B and F agree on all of these)

| # | Gap | Where | Questions it blocks |
|---|---|---|---|
| G1 | No unseen Fear pool, no unseen Event pool in `PublicSnapshot`; `allowed_fear`/`allowed_event` decoded and dropped | `projection.rs`, `public.rs` | Growth (unknowns column), Fast plan (what is still hidden), Event/Fear choices (token-section counts), the Habsburg T3 Event share; the "16 of 50 at Terror I" number cannot be computed today |
| G2 | No growth options in `PublicSpirit` (defs hold them; `common/src/growth.rs::GrowthOption` is the old Wiki parser) | `projection.rs` | Growth (every column), Reclaim schedule, per-option economy |
| G3 | Loss clocks and Presence not inside the enumeration; `OutlookPath` merges drop per-land state; `lost_by` is the Blight-only latent quantity; deck-out paths keep counting Blight | `outlook.rs::Walk::step`, `adversary::loss_clocks` | the race (§3) and everything that reads it |
| G4 | `advance()` drops destroyed Dahan, counterattack kills short of a full clear, Presence destroyed by Blight and cascades; cascade Blight counted but not placed; option-based Escalations and rule actions not placed (a Prussia Land Rush Town and a Sweden swap always happen; only the land is the pick) | `invaders.rs::advance` | the race beyond T+1; Presence clock (U18); honest multi-turn paths |
| G5 | `Edit` vocabulary is Destroy/Defend/Dahan/Track only: no Push, Gather, Isolate, Strife, Skip, RemoveBlight, Token, partial damage, presence placement, Growth, Play; no Energy/plays/speed legality; Destroy mutates count and pieces separately | `whatif.rs` | Fast plan vs this Ravage with real levers; Event choices; Draft "what it accomplishes" |
| G6 | No structured card effects: `PublicPowerCard.body` is text; `needs::tag_options` is a keyword tagger with silent `.min(12)`/`.min(16)` caps; no Event section tags; no Fear-card effect table | `public.rs`, `needs.rs`, corpus tier B | Draft and Forget; "card that saves the land" beyond keywords; Event/Fear counts |
| G7 | Two sources of truth for "applied": `rules_register::APPLIED` (deck_recipe, fear_deck, piece_health, setup_pieces) contradicts `adversary.rs` (Sweden L3, England L1 applied by code, reported `applied: false`); England L6 is a note string; Blitz tested by string in two files | `rules_register.rs`, `adversary.rs`, `invaders.rs` | trust in `rules_in_play` for the "unfamiliar adversary" flyout |
| G8 | `phase_position` maps `AdvanceInvaderCards` to "after" although that save is written before the cards move (`backtest.rs:90`): the just-resolved Ravage card is forecast as next turn's Ravage during that step | `invaders.rs::phase_position` | a wrong forecast whenever the companion is viewed at that step |
| G9 | Strings as enums: step names (44 occurrences), piece kinds, token names, terrains, positions, triggers, status-effect kinds and engine param keys (`isolated_lands` matches `contains("isolate")`) | `public.rs` and callers | silent mismatches; the G8 class of bug |
| G10 | Snapshot-per-save has no `captured_ms`, so decisions (`PublicDecision.ms`) cannot be aligned to the board before them; undo within a step is indistinguishable from progress (`companion.rs::Companion::push` keeps equal keys; `backtest::first` picks the first); `review::invader_damage` hard-codes 1/2/3 and ignores Strife and adversaries; cause labels miss Blight card and Event (R8) | `public.rs`, `companion.rs`, `review.rs` | Retro (R1–R4); the luck ledger |
| G11 | Spirit rules hard-coded in `invaders.rs` (Green's Choke, Vengeance ×2) instead of a plug-in like `adversary.rs`; ~50 register entries for 34 spirits unmodeled; Badlands, Vitality, Beasts, Ocean cascade/drowning flagged `unmodeled` | `invaders.rs`, `rules.rs` | correct ledgers for the spirits Brett plays |
| G12 | Duplicated logic: `review::invader_damage` vs `rules::invader_damage`; `adversary::invaders` vs `rules::has_invaders`; `needs::distance` BFS vs the engine's dropped distance cache; Still-Healthy detection by text in three places while `PublicBlight.current_still_healthy` exists; Fear-per-card counting in two places; the deck-out three-step test in `pace.rs`, `curve.rs`, `ledger.rs` | various | maintenance; a second way to be wrong |

### The ordered changes, each with its evidence requirement

Order follows the screens in §9 (race first, then Growth, then Fast plan, then Draft), which is also close to F's order by unlock count.

| Step | Change | Files/functions | Unlocks | Evidence required before it lands |
|---|---|---|---|---|
| L1-1 | Carry per-path state and first crossings inside the walk: call `loss_clocks`-style counters on the path board after each step; add `PresenceClock{spirit, on_island, at_risk_this_phase}` from `presence_at_risk`; merge leaves on (first ending, turn, step, cum-Blight sequence); make `deck_out` per path; aggregate the CIFs of §3; delete `lost_by` as a headline; branch the last Explore when Mining or England is in play; parameterize `PHASES` and measure the cost per decoded save | `outlook.rs::Walk::step`, `OutlookPath`, `adversary.rs::loss_clocks`, `mod.rs` | the race; G3 | B §6.2: at the last decision save of each of the seven games the realized cause has CIF = 1 at that turn in every order and every other cause 0 (Habsburg: `CIF_Mining(T4) = 1` at T4's Spirit save; Russia: Beasts at T6); a `clock_check` in the backtest (clock reads `now ≥ limit` at the ending save and `after` predicted it one step earlier); the permutation test extended to `outlook` output (spec §5 rule 5) |
| L1-2 | Finish `advance()`: remove destroyed Dahan; apply counterattack kills (best case, flagged); destroy Presence on Blight and cascades; place cascade Blight on the min-distance target and label it; place the always-happening Escalation/rule-action pieces at the least harmful land and label it; add `deadline_step` per clock (Mining: end of the next Fast Phase) and compute shortfall against it | `invaders.rs::advance`, `adversary.rs::LossClock`, `ledger.rs` | honest multi-turn paths; the Mining deadline; G4 | widen the backtest: assert `next_turn`'s Ravage set against the next turn's actual Ravage lands (today printed, never asserted); compare Dahan destroyed and counterattack kills with the log's `Damage` decisions; rulebook Blight (Q5), Victory/Defeat timing (Q11) |
| L1-3 | Project what the engine already decodes: `PublicFear.unseen` (allowed − discard − removed, by Terror Level section), `PublicEvents.unseen` (allowed − discard), `PublicSnapshot.distances`, `PublicSpirit.growth: Vec<PublicGrowthOption>` (from the DLL `SpiritList`), `captured_ms` | `projection.rs`, `public.rs`, `engine_map.rs` | Growth's unknowns and options; Event/Fear choices; Draft reach; Retro alignment; G1, G2, G10 | pools are setup facts (spec §5 visibility table); growth options spot-checked against the printed panels (`si-rules-check`); the firewall test must permute the new pools; the realized flipped Blight card, every Fear card and every Event must be in its unseen pool across the captures (bookkeeping, 100%) |
| L1-4 | Make `Edit` a complete action vocabulary with legality: `Play{spirit, card, land, option, assignment}`, `Move{from, to, kind}` (Push/Gather), `Isolate`, `Strife`, `Skip{land, step}`, `RemoveBlight`, `Token{land, kind, ±n}`, `Growth{spirit, option}` (reveals the node, credits Energy/plays, a Reclaim returns the discard), presence placement in a land; Energy, card plays and speed window checked; counts and `pieces` mutated through one helper; option enumerators for an Event's legal targets | `whatif.rs`, `rules.rs` | Fast plan with real levers; Event choices; Growth deltas; G5 | rulebook action definitions (core Push/Gather/Damage/Destroy; JE Isolate, Strife); FAQ for speed timing; the enumerated options compared with the engine's `Ask` options in the log where present |
| L1-5 | Structured effects: `PublicPowerCard.effects: Vec<EffectOption>`, `PublicEventSection.effects`, `PublicFearCard.effects` from a tagger that replaces `needs::tag_options`, with invariants, a disagreement report, and `unmodeled` notes instead of silent `.min` caps; a `tags → Vec<Edit>` resolver for the expressible families (Defend, Damage→Destroy, Push/Gather→Move, Strife, RemoveBlight, Fear) | `public.rs`, `needs.rs`, corpus tier B | Draft and Forget; "saves the land" for every effect; Event/Fear counts; G6 | printed card text (physical copy wins); the DLL `body` is a transcription to check, not the authority; the Test 1 Minor draws (`RevealCardsAction`) must be in `minor_unseen` and the Defend-only vs all-effects counts (17 vs about 27 of 96) reproduced |
| L1-6 | One source of truth for "applied": a registry of rule ids each `adversary.rs` hook claims, or per-entry flags in `invader-rules.json`, with a test that every rule the code applies is marked; England L6 and Blitz through the same path | `rules_register.rs`, `adversary.rs`, `invader-rules.json` | the clock flyout; G7 | a consistency test only |
| L1-7 | Enums for `Step`, `PieceKind`, `Terrain`, `TokenKind`, position, trigger, status-effect kind in `public.rs` (serde); fix `phase_position` for `AdvanceInvaderCards` (pre-advance, consistent with `backtest.rs:90`); the three deck-out tests become one helper beside `phase_position` | `public.rs`, `invaders.rs`, `pace.rs`, `curve.rs`, `ledger.rs` | G8, G9, G12 | engine `StateType`/`IslandPieceType`/`TokenType` tables already in `engine_map.rs`; a backtest run at every step, not only Ravage |
| L1-8 | Retro data: use `rules::ravage_ledger` in `review.rs`; add Blight-card and Event causes; join `log_parse` decisions to the snapshot before each via `captured_ms`; record `outlook_check` per turn as the luck ledger; make the Fast-phase one-sided check (floor after the actual Fast phase) replace `next_unanswered` | `review.rs`, `backtest.rs`, `companion.rs` | Retro R1–R4; G10 | the Prussia 6 retro table as the reference answer (`review_attributes_all_test1_blight` already passes for Test 1) |
| L1-9 | Spirit rules as a plug-in mirroring `adversary.rs` (hooks keyed by spirit id and aspect), starting with the register's entries for Rampant Green, Vengeance, Thunderspeaker, Vital Strength, Keeper, Shroud, Stone, Towering Roots; then the register's named gaps (Badlands, Beset TL III, England L6, Livestock L5, France L6, flip Immediates, Avarice, Isolate, Ocean cascade); then scenarios beyond Blitz (Ward the Shores, Second Wave, The Great River, Varied Terrains first) | `invaders.rs` → `spirits.rs` plug-in, `adversary.rs`, scenario module | correct ledgers per spirit; G11 | printed panels and cards; one capture per rule (schedule the games; corpus spec §12) |

Data-level rule for all of it: the corpus (`2026-10-19-si-corpus-design.md`) is the home of printed facts and effect tags; the engine is the oracle for what the digital game does; the companion never re-implements the engine (corpus non-goal; Brett 23:03).

## 5. Layer 2: the models and what they answer

### The stack, in build order (B §1; the report's "strict order")

| # | Layer | Computes | Exact | Assumed / labeled | Status |
|---|---|---|---|---|---|
| 1 | Ledger (Lumia: budget → inventory → resolution order → shortfall) | per land: damage with signed terms, Defend, Dahan, counterattack, Blight yes/no, cascade; per budget: Blight card, Fear to the next card and the next divider, deck clock, each adversary clock; the solve per land `D − h + 1` | all of it given the face-up cards | player picks (cascade target, counterattack allocation, Escalation lands) as candidate sets or min/max | built (`rules.rs::ravage_ledger`, `ledger.rs`); 7 games, 0 mismatches; changes: name `shortfall` "to keep this card"; add the cheapest cover of the shortfall (a tiny knapsack over ≤ 4 spends); print the land's actual health, not 2; clock deadlines per L1-2; Fear budget to the divider |
| 2 | Counting (hypergeometric on exchangeability) | P(next card is kind k) = n_k/m; P(≥1 of K in a draw of 4 from N); Event/Fear branch odds from the public pool by Terror Level and Healthy/Blighted | closed form | pool = allowed − seen (mismatch is a warning) | built for Invader and Minor/Major pools (`needs.rs`, `projection.rs`); missing for Fear and Event (L1-3) |
| 3 | Land tempo | per possible next card: Explore→Build→Ravage per land; badges | exact (≤ 6 cases one card ahead) | Explore source predicate, Wilds, Isolate | built (`invaders.rs`, `outlook.next_cards`); add per-turn land lists and damage |
| 4 | Timelines ("if X is next"; darkest and best) | per next card: which lands explore, build, ravage; min/max Blight over the cards after | exact | same | built (`outlook.rs`); the four-turn frame is retired (below), the enumeration stays |
| 5 | Momentum (Rei) | per growth option: Energy, plays, Energy on hand after this turn's plays, hand size, Reclaim turn, elements reachable, over the next 2–3 turns; against the lands that act, damage and Blight threatened per turn from the enumeration (certain part solid, uncertain as a band over orders) | exact per option; acting lands exact given the board | natural units only; any Energy-per-play scalar is a user-set parameter | partial: `curve.rs` projects two envelope tracks, not options; `GrowthLine` to be replaced by a per-option table once G2 lands |
| 6 | Race (competing risks) | one CIF per ending, first arrival wins; per-turn hazards; deck clock as administrative censoring | exact over orders given the policy | the no-action policy; horizon | the enumeration exists; the aggregation is wrong (`lost_by`); L1-1 |
| 7 | Comparison (CRN Monte Carlo, Wilson) | "A better than B in x% of futures" | simulated | needs a full rules engine | not built; not before layers 1–6 |
| 8 | Planning (depth-1 expectimax at reveal points) | per lever option, each next card through the ledger → saves/hurts/neutral | exact chance nodes | no verdicts | the lever columns of §6; L1-4 |
| 9 | Bound (hindsight) | upper bound to bracket rollouts | simulated | strategy fusion | not built |
| 10 | Learning (proper scoring against exact p*) | the optional hold: expected Brier excess (q − p*)² | exact when p* is | | not built; needs the hold toggle |

### Question → model → status (B §4 merged with questions-from-play)

| Question (moment) | Model(s) | Exact / assumption | Status | Smallest change |
|---|---|---|---|---|
| Growth: Blight per land with and without the option's play; what each land needs | ledger per option with the option's plays as edits | exact given the plays | partial: no Move/Growth/Reclaim edit | L1-4 (`Edit::Move`, `Edit::Growth`) |
| Growth: how many unseen Minors/Majors hold a card that saves the named land and is playable now | `needs.rs` (hypergeometric, draw 4) | exact count; "saves" an upper bound | **computed, displayed nowhere**; cost and plays unchecked; pocket rule binomial unlabeled | render `SpiritNeeds.save_land`; add `cost ≤ energy_after_option && plays_free`; mark the pocket rule "≈" |
| Growth: banked Fear cards that could save it (16 of 50 at Terror I); the Event as a common shift | counting over unseen Fear and Event pools | exact | **missing at the data level** | L1-3 |
| Growth: Energy, plays, presence by turn per option; Reclaim now vs later | momentum per option | exact per option | partial (`curve.rs` envelopes) | per-option 3-turn table after L1-3 |
| Growth: what is coming at the lands within 2 turns (C7's 9 damage) | timelines | exact T+1; T+2 identity exact | partial (`next_cards` carries lands explored and Blight min/max) | per-turn land lists and damage on `NextCard`/`OutlookPath` |
| Draft and Forget: what each candidate does on this board this turn at its speed | ledger with the candidate's effect as edits at Fast (before Ravage) or Slow (after Explore) | exact where expressible | **missing** (tags exist, no resolver) | L1-5 resolver + `whatif::apply` at the card's speed |
| Draft and Forget: pairings (elements toward innate levels; same-land combos), hand-now vs discard | `spirits.rs::InnateStatus` with the candidate added | exact | partial (U11, U13) | innate levels for hand ∪ {candidate} and hand ∪ discard ∪ {candidate}; the delta per level |
| Draft and Forget: what each Forget costs (Fast protection, elements, Fear) | element arithmetic; a count of Fast Defend/Fear sources | exact | **missing** | per card in hand+discard: speed, Defend n, Fear n, elements |
| Fast: per Ravage land after the plan: damage, Blight, Dahan, Presence, counterattack Fear | `whatif::apply` + `ravage_ledger` | exact | built for Defend/Destroy/Dahan; Push/Gather missing; cost unenforced | L1-4 |
| Fast: the Blight card after the turn and P(loss) at that depth (5 → 76%) | `p_loss` | exact over the pool; Immediates and the solo redraw not applied (labeled); "a Blighted side that empties loses" inferred from two games, to be registered with a citation | built | show the flip pool as a dot strip beside the number |
| Fast: "B3 is worth 3 of the 5" and the exact save per land | ledger spends and solves | exact | built | add the cheapest-cover line |
| Fast: what is still hidden (Fear cards, Event) and how often it helps | counting over Fear/Event pools | exact | **missing** (no pools) | L1-3, then count cards whose text helps the named lands |
| Fast: rules in play that explain the numbers (Fear is 5: two boards; Fine Steel; Mining's Fast check) | `RavageLedger.applied`, `rules_in_play`, `LossClock.checked` | facts | built; the Fear "why 5" derivation not spelled out; `applied` inconsistent (G7) | add the fear-per-card derivation (4N, +1 extra board, +1 England L6) to `FearStats.pocket_rule`; L1-6 |
| Event and Fear choices: the ledger rerun per legal choice | ledger per enumerated option | exact per option | **missing** (`whatif` cannot move pieces) | L1-4 + option enumerator |
| Event and Fear choices: what the unseen Events' token sections would do in each legal land (35 of 58 have a Beast section) | counting over the Event pool by section | exact | **missing** (no pool) | L1-3 + section tags (L1-5) |
| After the game: where it turned, costly choices, the board at each decision; luck ledger | `review.rs` + spec §7.10 flags F1–F10 and counterfactual runner | attribution exact; flags are pattern facts | 7.7 built; 7.10 missing; `invader_damage` wrong under Sweden L3 | L1-8; implement F1/F2/F4/F7 first |
| Standing: actual numbers with every modifier | `adversary.rs` hooks | exact | built (§7.11) | verify the unverified rows as games arrive |
| Standing: next turn's Ravage, loud when it empties the card | `invaders::next_turn`, `BlightStats.next_turn` | exact identities; amounts a floor | built | draw T+1 as certain in the race |
| Standing: unfamiliar adversary: where its rules bite and when checked | `rules_in_play`, `LossClock.checked` | facts | built as text | the clock row's deadline tick and flyout (§6) |
| Pace: Fear total vs Terror/Victory marks; need per turn to the deck-out turn; board-clear blockers per Terror Level | `pace.rs` marks, `need`, `level_ups`, `blockers` | exact arithmetic (Lumia) | built | keep; retire `recent`/`reach` (below) |

### Misframings to retire, each with its replacement

| Retire | Why | Replace with |
|---|---|---|
| "Unanswered … lost by T5 in 9 of 10" as the headline (the Runway) | Blight-only latent failure under a policy nobody follows, read as a forecast; the report names this error | the race of §3: per ending, per turn, first arrival wins; T+1 and T+2 drawn as certain; the policy label on the panel; the what-if delta beside it |
| "If nobody acts for four turns" as the frame | questions-from-play: "Not 'nobody acts for four turns'" | the enumeration stays as "what is coming at the lands" (timelines) and as the floor under a plan |
| `lost_by` counting Blight on paths that already ended | a CDF that keeps rising after the game is over | censor at the first ending |
| "WHO'S GROWING FASTER as multiples of Turn 1" with a 2.5 (1.5–3.5) Energy-per-play scalar | contested in its own thread; the report allows it only as a user-set parameter; multiples of a near-zero Turn-1 base (Shadows) are unbounded | the per-growth-option natural-unit table; the invader side as acting lands/damage/Blight per turn from the enumeration |
| "Lands named" as the invader step | terrain footprint is not pressure (Explore needs a source; Build and Ravage need Invaders) | acting lands, damage and Blight per turn; certain part solid, uncertain a band over orders |
| Mean of the last three finished turns as "your pace" and the "reach" turns from it ("5, 7, 5 (5.7 a turn). At that pace: Terror III on T8") | non-stationary, policy-driven, n = 3, no sampling model | the realized per-turn values as dots on the lane; `need` as the line to beat; reach only as **scenario lines** at named rates ("at 3 a turn: T10; at 5: T8; at your best turn so far: T7"); the guaranteed floor (counterattack best case, cards in hand playable this turn) is the only number called a forecast |
| "Pulling away / gaining on them / about even" and "holds this turn / prevent 5 to hold" (`ledger.rs::verdict`, `curve.rs`) | verdict sentences (item 15); "holds" was wrong on the facts for the Mining clock | the count and its deadline: "to keep this card: prevent N"; "Land Stripped Bare: 8 of 8 at the end of this Fast Phase" |
| Loss-clock shortfall as "prevent this turn" for a clock checked at the end of the Fast Phase | the deadline is the end of **next** turn's Fast Phase when `after` reaches the limit | `deadline_step` per clock; "you have next turn's Spirit and Fast phases" |
| Blight `shortfall` as one number for three stakes (flip / redraw / loss) | a flip is survivable; the loss stake is the mixture | "to keep this card: N"; the loss stake on `p_loss` |
| Fear budget to the next card as the goal | the questions name the next divider and the deck | keep the card as the unit; set `left` and `size` to the divider |
| `next_unanswered` vs `next_actual` in `outlook_check` | conflates Spirit action with model error; unfalsifiable | the one-sided check from the after-Fast-phase save (only Events and Fear cards can move the number; both are logged) |
| Hard-coded land health 2 in pocket rules; 1/2/3 damage in `review.rs::pressure` | contradicts "show the ACTUAL numbers" (Sweden L3, England L5, Vitality) | read health and damage from the ledger |
| Silent `.min(12)` / `.min(16)` subset caps | truncation without a note | an `unmodeled` note when exceeded |

### Verification plan

Deterministic layers keep the regression gate they have (seven captures, `si-steam-dump backtest`, 0 mismatches) and widen it: the engine's own preview functions (`GetRavagePreviewForLand`, `GetInvaderActionPreviewForLand`) as a second oracle at every step through REA; `clock_check` (the ending clock reads its limit at the ending save and `after` predicted it a step earlier; Habsburg and Russia positive, five negatives); `in_rows` stays 100% and gains the Blight flip (four flips available), Fear cards and Events once their pools exist; the permutation fixtures extend to `outlook`/race output and to `compute()`. The race is checked where it is checkable: at each game's last decision save the realized cause must have CIF = 1 in every order and every other cause 0 (seven hard checks); before that, the realized ending turn must be ≥ the floor's unless a logged Event or Fear card accelerated it (one-sided). The race is P(ending | state, no action), not a forecast of what happened, so it is never Brier-scored against outcomes. The exact probabilistic pieces (Explore kind, flip card, need finder) are scored once as PIT/log-score sanity on ~45 Explores, 4 flips and ~12 draws, expecting bookkeeping to be perfect and calibration to be untestable. Before `recent`/`reach` are deleted their predictions are scored against the realized Terror Level turns across the seven games so the retirement is on the record as evidence. Option comparisons, Fear-win scenarios and the board route can only be checked internally (same `ravage_ledger`, every legal option listed, nothing ranked).

### The display contract, attached to every number

Each number has three forms or it is not shown: a natural-frequency form (orders, cards, Blight, Fear: "27 of 96"), a delta form (base → tried, drawn as overlay), and a pocket rule the player can compute at the table ("1 ÷ unseen cards in this stage"; "Defend = damage − health + 1"; "fear to the divider = cards above it × 4N − generated"; "the deck is the turns left"). A percentage may sit beside the count in mono (Brett: "what %"), never alone and never for a quantity that is not a probability. The two outputs with no such forms, the 2.5-rate growth multiple and the mean-of-three pace, are the two retired above. The one output Brett asked for by name, the need finder, has all three forms and is not on screen; it goes on the first screen.

## 6. Layer 3: the UX/UI as a layer on 1 and 2

### How C and the Sly screenshots fit

Brett's complaint about the old dashboard was legibility, not quantity: "so much info smashed together, it's way too complicated and not easy to follow; the visualizations are terrible" (15:07); "SO MUCH INFO showna t once, it doesnt read well at all. i cant tell waht its actually tryign to do to aide me" (16:02). His reference, attached to the reset request, is dense: Sly's CPU view has fourteen panels on one 1600×900 screen. It is legible because every panel is **one named instrument**: a header with an icon, a NAME and a count or state ("MMIO 234", "DSP VOICES 7/8 active", "CALL STACK 2", "SCREEN 256×224"), a collapse chevron, mono values right-aligned, one purple accent that means only "current/selected" (the PC line, the active tab, the chosen save state), rows that share one axis (the voice scopes: one sparkline per voice, same horizontal scale), a view toggle on the data ("Linear | Graph"), a top tab bar of subsystems (CPU · PPU · APU · Super FX · Lua), and a status bar (file, mapper, region, timings, "paused @ 7E:DEEA"). The colored arrows in the disassembly are many hues, but each hue is one jump edge, never severity.

So the rule is: **density is fine when every panel is one named instrument in the shared grammar.** What is not fine: panels that each bring their own turn axis (the built Ledger had three), prose captions that restate the chart (the built Runway and Pace), severity encoded as color and glow (the built red pulsing tokens), and budgets as the page (the built Ledger). C's "at most 7 labeled groups" is replaced by the drift test below; the number of instruments is whatever fits 1600×900 with no page scroll. C's "numbers appear exactly twice" is replaced by: a number appears where a decision reads it (a count beside a strip, a cost on a lever, a register-style value in a budget instrument), never as a caption.

The mapping, Sly → companion:

| Sly | Companion |
|---|---|
| Top tab bar (CPU · PPU · APU · …) | the moment tabs: Growth · Draft · Fast · Event/Fear · Invaders · Slow · Review, auto-following the game's phase, click to pin |
| The disassembly: the largest center panel, the current line in the accent | the decision instrument: headed by the question of this moment; the lever columns; the current option under the cursor in the accent |
| Voice scopes: one sparkline row per voice on one shared axis | THE RACE: one row per ending on one shared turn axis |
| Registers, MMIO, DSP: small mono-value panels in side columns | the budgets (Blight card, Fear to the divider, deck, each clock, Energy/plays/elements per spirit): small instruments in a side column with header + count; never the page |
| Call stack | the pipeline: Ravage · Build · Explore cards and the lands each names (T+1, T+2) |
| Builtins with a filter box | Rules in play (the deleted `rules.rs`, returned as an on-demand instrument with a filter) |
| "Linear \| Graph" | list \| strip toggles on the same data: the race as rows of dots or as the per-order list; the pool as "27 of 96" or as the card list by effect |
| Console | the one-line "last change" (undo-aware: a reverted change disappears), not a log stream |
| Status bar | game · adversary L · spirits · turn/step · source (live/replay, save n/m) · hold |

### The information architecture (C §2, kept)

Three layers by overview → zoom → detail, each owning one question.

**Always-on: THE RACE** (top strip). Question: which ending arrives first, and on which turn, if nothing answers it? One horizontal turn axis from now to the deck's last Explore. One row per ending in fixed order, Invader side above the axis, Spirit side below: each adversary loss condition in play, Blight, Invader deck; Fear victory, board clear per Terror Level. Each row is a dot strip: one dot per tenth (or twentieth) of the enumerated orders at the turn that ending arrives, with the count beside it ("54 of 60 by T5"); a deterministic ending is one solid tick; a row whose quantity needs a layer-1 change is drawn hatched with "not yet counted"; the row whose dots arrive first carries one bracket "binds". Player-driven rows are counts of blockers or scenario lines, never dots of probability. Every row keeps a ghost copy for overlay when an option is hovered. T+1 and T+2 are drawn as certain. The turn ribbon (phase tabs) and the status bar belong here.

**Per-moment: THE DECISION** (center, the largest instrument). Headed by the question of this moment, verbatim from the catalog. Three columns: FIXED (face-up facts: the lands at stake as slot stacks and damage ledgers, the hand as levers with cost, Energy and plays), LEVERS (options side by side in fixed order, never ranked: a consequence table whose rows are the goals, race endings first, then what the option buys; cells are graphics; factors common to all columns drawn once spanning them), COUNTED (hidden pools as dot strips "N of M" with the pocket rule under each). Hovering an option overlays its futures on THE RACE (ghost = now, solid = after): deltas, not verdicts.

| Moment | Title (the question) | Lever columns | Goal rows (first row = the binding ending) |
|---|---|---|---|
| Growth | Which growth fits this turn? | the growth options | binding ending; the land(s) at stake this Ravage; card after the turn; hand/plays/Energy; cards run out (forced Reclaim) turn; elements/innate reach |
| Draft / Forget | Which card, and what to Forget? | the 4 drawn cards (and each Forget candidate) | what it does on this board this turn; what it pairs with; what Forgetting costs |
| Fast | What can I still change before this Fast Phase ends? | "do nothing", "your plan" (cards in hand with targets), each Fast play that reaches a Ravage/clock land | binding ending; Blight after this Ravage; Dahan/Presence kept; Fear earned; Energy/plays left; Slow still possible |
| Event / Fear choice | Which placement, which push? | each legal choice | same rows, re-run per choice |
| Invaders | What happened vs the forecast? | one column: forecast vs actual | the same rows |
| Slow | What sets up next turn? | each Slow play | next turn's Ravage/Build lands; the clocks at the T+1 check |
| Review | Where did it turn? | one column per turn | the race strip replayed; flags F1–F10 as chips; luck in its own row |

**On-demand: DETAILS** (hover, click, one key). Hover a land chip: its ledger (damage, Defend, Dahan, the exact save, the rules in play that change it, with when they are checked). Hover a race row: its budget lane (tokens, this turn's spend bracketed by land, next turn's ghosts) and the per-order breakdown ("if T5's Explore card is Jungle+Sands…"). Hover a dot strip: the count and the pocket rule. Click the pace row: the per-option economy slopegraph against the acting-lands band. Bottom line: three think-about prompts from §7.6 chosen for this moment, each a question, none answered. `H`: the optional hold.

### The visual grammar (C §3, amended)

1. Color means side, and nothing else: Invader-side endings and pressure one warm hue; Spirit-side endings and economy one cool hue; the accent marks only "now" and "the option under the cursor"; grey is "reference / do nothing"; terrain colors appear only on land chips, matched to Steam (Mountain grey). Severity, probability and warning are never colors; no glow, no pulse.
2. Horizontal position means turn, on one shared axis with one origin (now) and one end (the deck's last Explore), vertically aligned across every instrument that has time.
3. Certainty is drawn by edge: face-up facts are solid shapes with hard edges and an exact number (an 8-slot stack for Land Stripped Bare with 6 filled); counted unknowns are dots, hatched slots or ghosts, labeled "N of M"; nothing uncertain is drawn solid.
4. Futures are dot strips (quantile dotplots), with the count in natural frequency beside them; a percentage may follow in mono.
5. Before/after is an overlay (ghost under solid) or a slopegraph; never strikethrough, never "(was N)".
6. Magnitude is length on a common baseline (slot stacks, bars); never area, saturation or glow radius.
7. A number appears where a decision reads it; there are no sentences under charts; a chart that needs a caption is redrawn.
8. Type: a title size (the question), body, small for axes; mono for values; one big numeral size reserved for the FIXED column and instrument counts.
9. One screen, 1600×900, no page scroll; lists scroll inside their instrument.
10. Options are a consequence table: goals as rows, alternatives as columns, fixed order by lever, common factors spanning. The table lets the player see dominance; the app never computes it.
11. Never on screen: a ranking, a highlighted best, a default selection, a verdict sentence, a bare percentage, the island map, the hand as a panel, a piece editor, a log stream, a multiple-of-turn-1 ratio.
12. The question is the title; every element is labeled by the sub-question it answers.

### Wireframe (a): Fast Phase, Habsburg Mining Expedition L4, Turn 3 (corrected per §3)

Legend: `■` face-up fact, `▨` added by a face-up card, `▒` hatched: not yet countable (needs a layer-1 change), `●` one tenth of the orders, `○` a tenth that does not arrive, `┆` now, `▌` certain tick. Rows marked * are illustrative until L1-1/L1-3 land.

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ T3   Spirit ── ●FAST ── Event ── Fear ── Ravage ── Build ── Explore ── Slow     Habsburg Mining L4 · 2 spirits│ ribbon
├──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ▦ THE RACE   which ending arrives first, unanswered?                                    60 card orders   │ instrument
│              ┆T3        T4        T5        T6        T7        T8        T9        T10       T11        │ one axis
│ INVADERS                                                                                                 │
│ Land Stripped Bare   ┆          ●●○○○○○○○○ 12 of 60 (Explore: Mountain)  ▒ + Event share: not yet counted │ checked: end of T4's Fast
│ Blight card empties  ┆ ▌flips this Ravage (every order)  ●●●●●●○○○○ by T4's Ravage *   ●●●● by T5 *       │ censored where Mining fires first
│ Invader deck ends    ┆                                                                            ▌ T11  │ recipe: 8 cards left
│ SPIRITS                                                                                                  │
│ Fear victory         ┆  not before the deck, unanswered   (8 Fear per card: 2 players)                   │
│ Board clear, Terror I┆  blockers: N  (count, never a probability)                                        │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ▦ FAST · T3   What can I still change before this Fast Phase ends?        binding: see race (Mining or Blight)│ the question
├────────────────────────┬──────────────────────────────────────────────────────────┬──────────────────────┤
│ FIXED  face-up         │ LEVERS  fixed order, unranked                            │ COUNTED  hidden      │
│ NE7 Mountain           │ do nothing │ your plan     │ A  Fast → NE7   │ B  Fast → NW3 │ T3 Explore card      │
│  ■■■■■■▨░  6 of 8      │ (reference)│ (click cards  │ remove 1 Inv.   │ Defend 2      │  ●●○○○○○○○○ Mtn 1 of 5│
│  ■ now ▨ T3 Ceaseless  │            │  at left)     │                 │               │  Coastal·Sands·      │
│  Mining Blight (certain)│           │               │                 │               │  Jungle·Wetland 1 of 5│
│  ░ 8th: Explore Mtn or │            │               │                 │               │ Event                │
│  Event; check end T4 Fast│          │               │                 │               │  ▒ pool not projected │
│ NW3 Wetland  Ravage    │            │               │                 │               │  (common factor)     │
│  ■■■ 3 vs ■■ 2 → Defend 2│          │               │                 │               │ Fear cards banked    │
│ NE4 Wetland  Ravage    │            │               │                 │               │  0 · 8 to the card   │
│  ■■■ 3 vs ■■ 2 → Defend 2│ ── goal rows ───────────────────────────────────────────── │                      │
│ Blight card ■■■ 3 left │ Stripped Bare T4 ●●○○… +▒│ ·  │ ○○○○○○○○○○ 0 *   │ ●●○○… +▒ │                      │
│  this Ravage −3        │ Blight after T3  ■■■→□ 0 │ ·  │ ■■■→■ 1 (NE7 safe)│ ■■■→■ 1  │                      │
│ HAND (levers, Fast only)│ Blight loss by T4 ●●●●●●○○○○│ · │ ●●●○○○○○○○ *    │ ●●●●○○○○○○ *│                   │
│  Lightning: [..][..]   │ Dahan kept       ■■ 2    │ ·  │ ■■ 2             │ ■■ 2     │                      │
│  River:     [..][..]   │ Fear this turn   ·       │ ·  │ +1               │ ·        │                      │
│  Energy ■■■■ · plays   │ Energy · plays   4 · 2   │ ·  │ 2 · 1            │ 3 · 1    │                      │
│                        │ Slow still possible 2    │ ·  │ 1                │ 1        │                      │
│                        │ hover a column → its futures overlay THE RACE (ghost = now, solid = after)        │
├────────────────────────┴──────────────────────────────────────────────────────────┴──────────────────────┤
│ ▦ BUDGETS (side column, Sly-style instruments): BLIGHT CARD 3 · FEAR 0/8 · DECK 8 · LAND STRIPPED BARE 6→7 of 8 · LIGHTNING E/plays · RIVER E/plays │
│ think about:  Which clock does it move, and by how much?  ·  Is this factor common to all options?  ·  When does it act? │
│ Habsburg Mining Expedition L4 · Lightning's Swift Strike, River Surges in Sunlight    replay · save 165/240 · H hold │ status
└──────────────────────────────────────────────────────────────────────────────────────────────────────────┘
hover NE7 → 4 Explorers · 2 Towns · 0 Blight = 6; +1 Blight from Ceaseless Mining (Build card Mountain, face-up); 8th from
            the Explore (Mountain, 1 of 5) or the Event; "Land Stripped Bare: 8 Invaders + Blight in one land, checked at the
            end of each Fast Phase"; Mining Boom I (upgrade, count unchanged); every Habsburg L4 modifier named with its number.
```

What it says without prose: NE7 is 6 now, 7 for certain, and the 8th depends on hidden cards; the check is the end of T4's Fast Phase, so the player has this Fast Phase, this Slow Phase, T4's Spirit Phase and T4's Fast Phase to take one piece out; the Blight card flips this Ravage whatever happens; only option A moves both rows. At T4's Spirit Phase the same screen shows the Mining row as `▌ end of this Fast Phase, every order` and the Blight row empty.

### Wireframe (b): Growth, Test 1 Turn 3, Shadows Flicker Like Flame vs Brandenburg-Prussia L2, boards C+E (corrected)

Facts: C4 Sands Ravages for 3 against health 2 with 3 Dahan (needs Defend 2, or any effect that saves it); C1 Jungle and E3 Wetland Blight regardless; Builds only in E1 (Disease in C2); the Blight card is healthy with 3 left, so three Ravage Blights flip it; Fear is 5 per card (two boards: Brett's "why is it fear 5 instead of 4"); 1 Fear card banked and one more reachable before the Ravage. Growth options from Brett's play record: G1 Reclaim All + Gain a Power Card; G2 Gain a Power Card + Add Presence; G3 Add Presence + 3 Energy. Tracks come from the engine's `TrackNode`s, not the Wiki scrape (memory: the scrape is unreliable). Minor pool: 96 unseen (4 seen at T1); the spec's worked numbers 27 of 96 save C4 and are playable now (17 Defend-only; Lure and Cycles among the other 10), a draw of 4 holds one in 1 − C(69,4)/C(96,4) ≈ 74% ("about 3 in 4"), to be recomputed by `needs.rs` with cost and plays enforced.

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ T3   ●GROWTH ── Fast ── Event ── Fear ── Ravage ── Build ── Explore ── Slow       Brandenburg-Prussia L2 · Shadows│
├──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ▦ THE RACE   which ending arrives first, unanswered?                                                     │
│              ┆T3        T4        T5        T6        T7        T8        T9        T10       T11        │
│ Blight card flips    ┆▌ this Ravage (C1, E3, C4), every order ◄ BINDS    healthy side 3 left, −3 unanswered│
│ Blight loss (other side) ┆            ●●●●○○○○○○ *   ●●●●●●●○○○ *   (mixture over the 23-card pool)        │
│ Invader deck ends    ┆                                                                      ▌ T(recipe)   │
│ Fear victory         ┆  not before the deck, unanswered (5 per card)                                     │
│ Board clear, Terror I┆  blockers: N                                                                      │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ▦ GROWTH · T3   Which growth fits this turn?                                   binding: Blight card flips, T3│
├────────────────────────┬──────────────────────────────────────────────────────────┬──────────────────────┤
│ FIXED  face-up         │ OPTIONS  fixed order, unranked                            │ COUNTED  hidden      │
│ C4 Sands  Ravage now   │ G1 Reclaim + Minor │ G2 Minor + Presence │ G3 Presence + 3 Energy│ MINOR DECK 96 unseen │
│  ■■■ 3 vs ■■ 2 · 3 Dahan│                   │                     │                       │  saves C4, playable  │
│  needs Defend 2 or save│                    │                     │                       │  now: 27 of 96       │
│ C1 Jungle · E3 Wetland │                    │                     │                       │  ●●●○○○○○○○          │
│  Blight regardless ■■  │                    │                     │                       │  Defend-only 17 ·    │
│ Blight card ■■■ 3 left │ ── goal rows ───────────────────────────────────────────────────│  other saves 10      │
│  healthy; −3 unanswered│ C4 saved by the draw  ●●●●●●●○○○ 7 of 10 │ ●●●●●●●○○○ 7 of 10 │ · no draw │  a draw of 4 holds   │
│ Builds: E1 only        │ C4 saved by Fear cards ─── ▒ not yet counted (needs the Fear pool) ─── spans all ───│  one: ●●●●●●●○○○ 7/10│
│  (Disease in C2)       │ Event                  ─── common shift ▒ (needs the Event pool) ─── spans all ───│ FEAR DECK            │
│ HAND now [Mantle][Crops][Bats]│ Card after T3  ■■■→■ in 7 of 10 │ ■■■→■ in 7 of 10 │ ■■■→□ flips    │  ▒ 50 unseen at TL I:│
│  discard [Conceal][Favors]│ hand · plays this turn  6 · 2  │ 4 · 2 (3 if plays track)│ 3 · 2, Energy +3 │  pool not projected  │
│  Energy ■ · plays 2    │ Energy a turn  T3 T4 T5 T6 (slopegraph, shared axis) G1 flat │ G2 ╱ │ G3 ╱ +3 now│  banked 1, +1 before │
│ TRACKS (engine nodes)  │ Cards run out (forced Reclaim)  G1 ▬▬▬▬▬▬▬▬▬▬ T6 │ G2 ▬▬▬▬▬▬ T5 │ G3 ▬▬▬ T4 * │  the Ravage = 2      │
│                        │ Innate reach  Darkness Swallows the Unwary: elements per option (hover)           │                      │
│                        │ acting lands a turn (band over orders) ▁▁▂▂▃▃ on the same axis                    │                      │
├────────────────────────┴──────────────────────────────────────────────────────────┴──────────────────────┤
│ ▦ BUDGETS: BLIGHT CARD 3 healthy · FEAR 4/5, 1 banked · DECK n · SHADOWS 1 E / 2 plays / elements          │
│ think about:  Is this about order or about count?  ·  Which effects might you be missing?  ·  What does this card buy now, and later? │
│ Brandenburg-Prussia L2 · Shadows Flicker Like Flame · boards C+E                     replay · Test 1 · H hold │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────┘
hover "27 of 96" → the need finder's list by effect (Defend 17 · gather/push 6 · remove Blight 2 · …) and the pocket rule
                   "about a quarter of the deck; four looks → about 3 in 4 · 74%".
```

What it says without prose: G1 and G2 give the same chance of saving C4 (the draw), so between them the decision is the bottom rows: hand size now and when the cards run out, which is Brett's "order, not count" point drawn as bar length on the turn axis; G3 trades the draw for Energy and reach, and its C4 row is the Fear-card row alone (which cannot be drawn until the Fear pool exists). The Fear-card and Event rows span all three columns so the eye drops them from the comparison.

### The drift test (apply to any proposed screen, instrument or feature; one "no" and it is not built)

1. Is it headed by a question Brett actually asked at this phase (questions-from-play), with that question as its title?
2. Is the lever something the player can do in the game this phase (a growth option, a card with a target, an Event choice, a Fear-card target, a track), not an edit to the board?
3. Does every time-based element sit on the one shared turn axis, aligned with THE RACE?
4. Is every hidden quantity drawn as "N of M" dots or slots, with the count beside it and no bare percentage?
5. Can the screen be read correctly with every sentence deleted?
6. Does it show at least one thing the game's own screen does not?
7. Is there nothing ranked, highlighted as best, selected by default, or stated as a verdict?
8. Is every panel one named instrument (header, name, count) in the shared grammar, and does the whole fit 1600×900 with no page scroll?
9. Does it reuse an existing encoding (side color, turn position, dots, slot stacks, slopegraph, ghost overlay) rather than add one?
10. Does it avoid reproducing the island, the hand as a panel, a log stream, a piece editor, or any panel the game already has?

## 7. Where we are: what carries forward, what is discarded

Nothing is committed until Brett says so. The index is inconsistent right now (only the nine view deletions are staged; a bare `git commit` would record a non-building tree), so the first git action is the split below, after his go-ahead, with `git fetch origin` first and only named files staged.

### Core math (`si-common`): keep, with the named changes

| Piece | Verdict | Change required by §5 |
|---|---|---|
| `rules.rs` `add_built` Explorer arm | keep | none (the real bug fix: lookahead added Towns where it meant Explorers) |
| `invaders.rs` `advance`/`time_passes`/`next_turn_start` | keep | L1-2 (Dahan, kills, Presence, cascades placed, Escalation pieces placed); L1-7 `phase_position` |
| `mod.rs` `RoundSummary` card slots, `slot_cards`, Explore back-fill | keep | unit tests across the `AdvanceInvaderCards` boundary and the back-fill loop (breaks for Russia's Fear-deck cards) |
| `adversary.rs` `LossClock.unit` | keep | add `deadline_step`; France unit per §8 |
| `ledger.rs` | keep | `shortfall` named "to keep this card"; cheapest cover; actual land health; the `urgent` prose moves to a view; Blight `size` as a drawing hint moves to the view |
| `outlook.rs` | keep | L1-1 (per-path clocks, Presence, censoring, CIFs; `lost_by` retired); `PHASES` parameterized; cost measured |
| `pace.rs` | keep | `recent`/`reach` retired after being scored; marks, `need`, `level_ups`, `blockers` stay |
| `curve.rs` | keep | `projected()` replaced by a per-growth-option table after L1-3; `stage_reach` fallback labeled in `basis`; "lands named" replaced by acting lands from the enumeration |
| `whatif.rs` | keep | L1-4 (the full `Edit` vocabulary, legality, one mutation helper) |
| `needs.rs` | keep | cost and plays enforced; "≈" on the pocket rule; `unmodeled` note instead of `.min` |
| `backtest.rs` `outlook_check` | keep | `next_unanswered` replaced by the one-sided after-Fast check; `clock_check` added |
| `spirits.rs` visibility change | keep | |
| `prompts()`, `rules_in_play`, `review.rs`, `changes.rs` | keep (computed, undrawn) | wired into the instruments of §6 |

### Toolkit (`si-ui`): keep

SDF lines/glows/circles, the mesh pipeline and `quad`, icons via resvg and glyphon custom glyphs, three layers, the animation clock, `offscreen.rs` icons+time, `Key::Tab`: all keep. `glow` and `pulse` stay in the crate and leave the views (the grammar bans severity glow). `polygon`/`triangle`/`arrow` are unused: keep only if Brett wants a gltf/land view later (§10), otherwise drop in the toolkit commit.

### App (`si-app`): what survives as a component, what is discarded

| Piece | Verdict |
|---|---|
| `icons.rs` | keep (asset glue) |
| `views/chart.rs` (`Plot`, `turn_axis`, `hline`, `polyline`, `dashed`, `band`) | keep; `caption` goes; move toward a `charts` module; `turn_axis` becomes THE RACE's one shared axis |
| `views/runway.rs` `now_x()`, `quantile()` | keep (extract); the Runway panel, its line-per-outcome chart, the P(loss) shading and the captions are discarded; the per-order enumeration and "if the next card is X" list become the Blight row and its hover |
| `views/pace.rs` | discard as a page; required-slope vs realized dots becomes the Fear row and its hover; the board-route sparkline math moves to `pace.rs` core |
| `views/ledger.rs` | discard as a page; salvage `budget_tokens` (token rows, brackets, edge line), `fear_tokens`, `deck_tokens`, `inventory_row`/`spend_chip`, `units()` as the budget instruments and the land-chip flyout; `verdict()` is deleted |
| `views/curve.rs` | discard; the economy slopegraph becomes the per-option row in Growth/Draft on the shared axis; the stacked stage bars become the acting-lands band; the 2.5-rate multiples panel and "each turn's change" are deleted |
| `views/whatif.rs` | the drawer frame and the piece editor are discarded; `button`, `toggle`, the base-vs-tried recompute cache keyed on `(seq, edits)`, the moment-reset of edits, and the land tiering (Ravages now / Builds now / Ravages next) survive under the lever columns |
| `views/mod.rs` | `Page::{Ledger,Curve}`, the page switch and the status line are discarded; `Both`, `Toggle`, `step_index`, the ribbon are kept |
| `main.rs` CLI | keep `--whatif` and the offscreen plumbing (reproducible screenshots); rename/remove `--page ledger\|curve` and `--drawer` with the layout |
| the nine deleted N1/N2 views | keep the deletion; the Rules view's content returns as an on-demand instrument; the Defend/powers-used badges return on land chips; the play-by-play returns as the one-line undo-aware "last change" |

### Docs

Keep `2026-10-19-questions-from-play.md` (commit it). Mark the 2026-10-18 spec §2 and §8 and the native-app spec §3 and milestone table as superseded by this document (one line each, quoting Brett's 15:07/16:02/19:52), and append §2 items 20, 23 and the three-layer statement (item 38) to the spec. Correct the memory file per §8.

### The commit split (E §5), so the discarded layout reverts in one step while the core survives

1. `rules: Explorers built in the lookahead count as Explorers` — `common/src/rules.rs` alone.
2. `companion: Invader Phase advance, Time Passes, cards per turn in the history` — `invaders.rs`, `spirits.rs`, `adversary.rs`, and the `RoundSummary`/`slot_cards`/`round_summaries` hunks of `mod.rs` (`git add -p`).
3. `companion: ledgers, card-order outlook, Fear pace, the curve, and what-if edits` — the five new core modules, the remaining `mod.rs` hunks, `backtest.rs` `outlook_check`, `dump.rs` `outlook`; the 10 new tests.
4. `si-ui: SDF lines and glows, meshes, icons via resvg, layers, animation clock` — `ui/*`, `Cargo.lock`.
5. `native app: chart primitives, game icons, fresh exe on build` — `chart.rs`, `icons.rs`, `build-windows.sh`.
6. `native app: Ledger and Curve pages, Runway, Pace, What-If drawer (layout superseded by the 2026-10-19 direction)` — `main.rs`, `views/{mod,fmt,ledger,runway,pace,curve,whatif}.rs`, the nine deletions. One commit, so it can be reverted or cherry-picked from as a unit.
7. `docs: the questions from play and the screen each one needs` — the questions-from-play doc, plus this document under `docs/superpowers/specs/2026-10-19-direction.md` if Brett wants it in the repo.

## 8. Contradictions found, and how each is resolved

| Sources that disagree | Side 1 | Side 2 | Resolution | Who decides |
|---|---|---|---|---|
| Dense (Sly) vs minimal (C) | Brett's reset request attached four Sly screenshots, fourteen panels on a screen, as "inspiration"; "lean towards much more helpful, visual, cool looking things" (19:44) | C §3 rule 9: "At most 7 labeled groups on screen… at most 3 encodings" | Brett's complaint was "smashed together… I can't tell what it's trying to do", not "too much". Dense-but-ordered: every panel one named instrument in one grammar (§6); C's group cap replaced by the drift test; C's center-table architecture and shared axis kept | decided here; Brett confirms on the first mockup (§9 step 1) |
| What-if as piece editor vs levers are real actions vs "tinker" | the built drawer: "Click a piece to destroy it, add Defend or Dahan"; Brett: "And tinker" (22:00) | spec §7.5 levers are growth options, cards, targets, Event choices; C: "a piece edit is not a lever"; Brett: "we don't need to reinvent an entire game inside of a companion" (23:03) | levers are real actions (item 3); the recompute engine survives under the option columns; the free piece editor is not shown; "tinker" is read as trying one's actual options | Brett (§10 Q2) |
| The Runway/outlook horizon | `outlook::PHASES = 4`, "no Spirit action" as the headline; C's axis "to the deck's last Explore" | questions-from-play: "Not 'nobody acts for four turns'"; B: T+1/T+2 are certain, the fan opens at T+3 | the race axis runs to the deck's last Explore (a deterministic tick); enumerated rows run to the horizon the walk can afford (4 phases now; parameterized and costed), beyond it rows are blank, never extrapolated; the frame is "what is coming at the lands" and "the floor under a plan", labeled with its policy | decided here |
| Percentages vs counts | Brett: "what % of getting a minor-drawn defend card" (15:59); "% chance of getting a defend card" (16:02) | spec §7.6 and the report: natural frequencies, "not 67%" | counts are mandatory with a pocket rule; a percentage may sit beside the count in mono; never a bare percentage; never a percentage for a non-probability (pace, growth) | Brett wins on allowing %; decided here on the form; Brett confirms on the mockup |
| Budgets as page vs strip | the built Ledger page; memory ground-up file "Lumia: hard budget → land inventory…" | questions-from-play: "a narrow status strip, not the page"; Brett: "It shows basically the info we already know" (20:58) | budgets are Sly-style side instruments (header + count) and hover flyouts, never the page | decided here (Brett's 20:58) |
| Pace board-clear column | A §4 E12: "That question is not in brett-messages.md" | mid-turn record: "Is the pace only fear victory? Or are you accounting for tier 1-3 wins too" (21:20) | confirmed asked; the board-clear rows stay in the race as blocker counts per Terror Level, never a probability; A's E12 and Q12 are resolved except the "4 or 5 plugins" | resolved |
| France clock: per player vs per board | `adversary.rs` uses `7 × players`; Brett's Q17 answer: "return all but 7 Town per player to the box. Invaders win if you ever cannot place a Town" (printed panel) | B: the report and the engine count per board | the clock is the Town supply: the companion must show what the engine will do (item 11), so read the actual supply from the save if the engine exposes it, else count per the engine's rule; same number in solo on one board; a two-board France game decides it and none is recorded | Brett (§10 Q6) and a France capture |
| 2026-10-18 spec §2 vs native-app §1 | §2: "Always-on dashboard. Every number visible, updated live… Second monitor, Windows browser, dense layout" | native-app §1: "one screen, no scrolling… showing only what the current moment of the game needs"; Brett 15:07, 16:02 | §2 superseded (browser, every-number-always-on); native-app §1 stands, amended by §6 (one screen per moment with the race always on) | resolved (Brett's words) |
| `needs.rs` computed but never shown | E §3: no `app/src` consumer; B: "the one model Brett asked for by name… is computed and invisible" | Brett 15:59, 16:02 asked for exactly this | it is the COUNTED column of the first screen (§9 step 1) | resolved |
| Memory-file claims (`feedback_companion_ground_up_lumia_rei.md`) | "He also rejected multiple-choice pickers twice while the framing was wrong; explain the direction in prose first" | no verbatim message says it (A principle 47) | keep as a labeled inference (item 36); ask Brett (§10 Q8). The other two claims A flagged ("can't quite always grasp", "tier 1–3 wins") are confirmed by the mid-turn record and stay. "He points at the stack's crates (winit, wgpu, glyphon, gltf, resvg)": Brett named "winit, wgpu, gltf, glyphon, etc." and "those crates docs and examples"; resvg is Claude's addition, harmless. The claim "4–5 plugins… screenshots" is confirmed for the screenshots (Sly) and open for the plugins (§10 Q9) | Brett for Q8/Q9 |
| Habsburg T3 count | B §3.9: "NE7 held 7 (4 Explorers, 3 Towns)" at the start of T3; "8 in every order" | playtest H4 and the T3 Fast screenshot: 6 (4 Explorers, 2 Towns); 7 after the Ceaseless Mining Blight; 8 after the Event | the capture wins (§3): 7 certain, the 8th conditional at T3; 8 certain at T4's Spirit Phase | resolved |
| Verdict strings | `ledger.rs::verdict` ("holds", "prevent N to hold"), `curve.rs` ("pulling away") | spec §7.6 (from Brett's 17:17); "holds" was wrong for the Mining deadline | deleted; counts with deadlines (§5) | resolved |
| Island and play-by-play | Brett asked for Mountain grey, Defend badges on lands, undo-aware play-by-play (15:07, 15:12); native-app N6 lists an optional gltf island; Brett named gltf in his stack | C rule 11: never the island map, never a log stream; Brett 23:03 | no island panel and no log stream in the decision screens; land chips carry terrain color and Defend/power badges wherever a land appears; "last change" is one undo-aware line; a gltf island is not built before the first screens | Brett (§10 Q7) for a later island |
| Retro timing | Brett 15:55: "show me this first in the plan/chat here, and then paln fro it for realt oa dd to the companion"; 19:03 | questions-from-play puts the live screens first | the chat retro exists (the Prussia 6 reference table); the live screens come first; Retro is §9 step 6 unless Brett reorders | Brett (§10 Q5) |
| Rules view | deleted with N2; Brett: "not familiar with Habs Mining at all" (19:12) | C does not mention it | returns as an on-demand instrument (Sly's "builtins" with a filter) and as the clock row's flyout; not a page | decided here |

## 9. Next steps, in order, each with a definition of done

Process contract (item 35): one thing, Brett's yes, then the next. Commits happen only when Brett says so (§7 split).

| # | Build or write | Layer | Serves | What Brett SEES to say yes or no | Explicitly NOT done |
|---|---|---|---|---|---|
| 1 | **One static mockup image** of the Growth screen on Test 1 Turn 3 in the §6 grammar: Sly-style grid, THE RACE strip on top, the Growth instrument headed by its question with the three columns, the need finder's "27 of 96" as the COUNTED column, budgets as side instruments, prompts line, status bar. Rendered with the existing offscreen painter from fixed numbers (the spec §7.5 worked example; rows that need L1 changes hatched "not yet counted"). | 3 | items 2, 3, 5, 14, 21–26, 35, 36 | a single PNG at 1600×900, judged in under a minute: "is this the shape?" | no code in `common`; no new view wired to live data; no second screen; no commit |
| 2 | **The race for real**: L1-1 and L1-2 (per-path clocks, Presence clock, cascades placed, censoring, CIF aggregation, `deadline_step`; `lost_by` and `verdict` deleted) with the B §6.2 checks. | 1 + 2 | items 10, 11, 12, 15, 29, 30 | a table from the seven captures: per game, at the last decision save, the race's binding ending and turn vs the actual ending (seven rows, all must agree), plus the Habsburg T3 and T4 rows of §3 as the companion now computes them | no screen yet; no Fear/Event pools; no new edits |
| 3 | **The Growth instrument live** (the step-1 mockup made real): L1-3 (growth options, Fear and Event pools, `captured_ms`, firewall test extended), `needs.rs` with cost and plays enforced and wired in, the per-option economy table and the forced-Reclaim turn on the shared axis, THE RACE strip drawn from step 2. | 1 + 2 + 3 | items 1–6, 14, 21–24 | the running view, replayed on Test 1 T3 and Prussia 6 T3/T4, then used by Brett in one real game at every Growth phase; his notes in a playtest doc | no Draft screen; no Fast plan; no Event choices; no Retro |
| 4 | **Fast: my plan vs this Ravage** on Prussia 6 Turn 5 ("this is grim") and Habsburg T3/T4: L1-4 (the `Edit` vocabulary with legality; cards in hand as levers with targets; Event-choice enumeration), the lever columns with "do nothing" and "your plan", overlay on THE RACE. | 1 + 3 | items 2, 3, 9, 10, 11, 15, 20 | the running view in a real game at every Fast phase; the Habsburg T4 delta (one piece out of NE7 → the Mining row vanishes) shown on the replay | no piece editor; no Draft; no structured card effects beyond what `Edit` needs |
| 5 | **Draft and Forget** on Prussia 6 T3 (four Majors) and T4 (four Minors): L1-5 (structured effects from the corpus, tier B; the resolver), the three values per candidate, pairings split hand-now vs discard, the Forget cost row. | 1 + 2 + 3 | items 7, 21 | the running view at every Gain-a-Power step in a real game | no Event/Fear-choice screen (it reuses step 4's enumerator and lands after); no Retro |
| 6 | **Retro**: L1-8 (ledger-based pressure, Blight-card/Event causes, decisions aligned to boards, luck ledger), the Review tab as the race strip replayed per turn with flags F1–F10 as chips, checked against the Prussia 6 reference table. | 1 + 2 + 3 | items 13, 15 | the Review tab on the Prussia 6 capture matching the hand-written retro row for row, then on his next lost game | no counterfactual runner beyond one phase; no multi-turn hindsight presented as fact |

Standing alongside every step, not a step: L1-6, L1-7 (the `applied` registry; enums; the `AdvanceInvaderCards` fix) land with step 2's core commit; L1-9 (spirit plug-in, register gaps, scenarios) proceeds one capture at a time as Brett plays the matchups the corpus spec §12 schedules.

## 10. Questions only Brett can answer

Dropped because the sources answer them: A's Q1 (page by question or by budget: Brett's 20:58 and 22:38 answer it: by question, budgets as instruments), Q2's order (questions-from-play: Growth first, then Fast; A's Q13 is folded into Q5 below), Q3 (the Runway: resolved in §8), Q5 (one-line verdicts: resolved by §7.6; "holds" was wrong on the facts), Q6 (the 2.5 rate: the spec already says user-set parameter or nothing; dropped from the chart), Q10 (the process contract: Brett's 20:05, 21:57, 19:49 state it), Q11 (captions: his 22:58 answers it), Q12 (the missing quotes: found in the mid-turn record; only the plugins remain), Q14 (which spec is current: §0 of this document).

| # | Question (one line to answer) | What changes with the answer |
|---|---|---|
| 1 | Is the §9 step-1 mockup (Sly-style grid, race strip on top, one question-headed instrument in the center, budgets as side instruments) the shape you want? | yes: step 2 begins; no: a second mockup, in prose first, before any code |
| 2 | "Tinker": trying your actual options (G1/G2/G3, which card, which target, which push), or free edits to the board? | actual options: the piece editor stays out; free edits: a labeled "sandbox" instrument is added after step 4, still never written to the game |
| 3 | "Competing goals": both the race between endings over turns (statistical) and the per-phase contest for your actions (attention), with the race always on top and the attention sense inside each decision instrument? | yes: §3 and §6 stand; no: say which sense comes first |
| 4 | Percentages: is "27 of 96 · 74%" (count first, percent beside it in mono, pocket rule on hover) right, or counts only? | sets the number format on every instrument |
| 5 | Order after the first screen: Growth → Fast plan → Draft/Forget → Retro, or Retro earlier? | reorders §9 steps 3–6 |
| 6 | France's Sprawling Plantations: on two boards in solo, does the Steam game give 7 Towns (per player) or 14 (per board)? Your printed panel says per player. | sets the France clock's limit and whether the supply is read from the save |
| 7 | A land view later: a compact gltf/2D island as an on-demand instrument (never a panel in the decision screens), or no island at all in the companion? | keeps or drops `polygon`/`triangle`/`arrow` and native-app N6 |
| 8 | The memory file says you "rejected multiple-choice pickers twice while the framing was wrong; explain the direction in prose first." Is that right as a standing rule? | keeps or removes item 36 and the memory sentence |
| 9 | Which "4 or 5 extra plugins around visualizations and graphics" (21:01) did you mean? Only frontend-design, four LSPs and riff are installed. | names the crates/plugins to draw on for the look; nothing else changes |
| 10 | The deleted Rules view, the land Defend/powers-used badges and an undo-aware "last change" line: back as on-demand instruments and chip badges (not pages or a log), as §6 proposes? | adds or removes them from step 3's scope |
