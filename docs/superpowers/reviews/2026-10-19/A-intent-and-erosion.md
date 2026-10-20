# Review A: intent and erosion — the Steam companion

Reviewer: independent; sources read in the order given (Brett's verbatim messages, the assistant's own account, the memory files, the 2026-10-18 and 2026-10-19 specs, the questions-from-play doc, all six playtest logs, the stat-models report), plus the uncommitted native-app code and the screenshots of what was built. Where the assistant's account conflicts with Brett's messages, Brett's messages win and I say so.

Date convention: `2026-10-18 · 13:18` is session 191ccfe4 (Test 1, played step by step); everything on 2026-10-19 is session 05faae41. Quotes are verbatim, typos included.

---

## 1. The founding intent, in Brett's words

**2026-10-18 · 13:18** (the first message of the project):

> looking back at this project. lets hook into whenever you're running the digital version of hte game on steam. that process. and lets morph / utilize this app to be the 'spirit island companion' showing percentages, statistics, current state in various breakdowns/visualizations/inspectiosn in order to LEARN the stats/math around the game and help you think about probabilities and strategies throughout gameplay. not a cheat, at all!!!

**2026-10-18 · 14:04** (what the numbers are for, and what they are not):

> I don't care about "statistics" the makers of the digital game collect (just a count of wins/losses this information is useless to us), the win/loss doesn't tell us much to our all which is more around the specific scenario and rules for and against the player at the given moment.

**2026-10-18 · 16:18** (the shape of the help, stated after working Test 1 Turn 3 by hand):

> Ultimately the companion needs to draw out this kind of stuff in a helpful way. It needs to show specific scenarios and probabilities and helpful knowledge on direct questions like the one we're asking about a given phase (e.g. Invader phase -> Ravage) given ALL the things we can a.) directly affect and control b.) that we can't directly affect but can take into account for the things we can ... And also it should lay out the comparison of how we're growing and keeping pace in either direction. Vs outgrowing an adversary especially.

**2026-10-18 · 17:17** (the line that must never be crossed):

> you can probably start seeing hnow theres so many leveers to pull on, with so many mechanics all comign together; we need the companion to be... helpful/augmenting certain choices with math/stats, and PRESENTING things to THINK ABOUT during these decisions, but not doing it for them/making the calls directly for them. you adn i here, are doing back and forth to help figure tehse choices out to present this information for the tool, thats fine

**2026-10-19 · 19:54** (the re-statement at the reset):

> re-thina bout from the... lumia + rei approaches of showing statistical/analyical thigns ina  reasonable/companion way aside from the game running

**2026-10-19 · 22:00**:

> Think entirely from the perspective of a player who is looking to augment their gameplay. And learn. And tinker

**The purpose, as precisely as the sources allow.** The companion is a second window that reads the running Steam game and, beside it, answers the *direct questions a player asks at a decision* ("what % of a Minor-drawn Defend to help C4… is this worth it"; "best card to forget"; "this is grim, what would the companion say"). It exists so Brett LEARNS the stats and math of the game and builds the habit of pausing to think, not so the companion plays for him. It shows only what a player at a physical table could know ("not a cheat, at all!!!"). For each question it separates what is fixed, what the player controls, and what is hidden but countable, and it lays the player's growth against the adversary's pressure ("outgrowing an adversary especially"). It presents things to think about and never makes the call. Its analytical idiom is Lumia's ledgers and Rei's momentum ("keeping ahead of the curve"). The back-and-forth in chat during real play is where the content of those screens comes from. It is NOT: a cheat; an engine that ranks moves; a restatement of what the game already shows; a port of the old tabletop panels; a re-implementation of the game; a wins/losses counter.

---

## 2. Every stated principle and constraint

Each entry: principle; verbatim source(s) with date; where it is encoded. Entries marked **(derived)** come from Claude's research or specs and were accepted by Brett without objection, but are not Brett's words; I do not promote them to Brett's authority. Entries marked **(unverified)** rest on quotes that appear only in the assistant's account or a memory file and NOT in the verbatim message record.

### What the companion is for

1. **It hooks into the running Steam game and sits beside it.** "lets hook into whenever you're running the digital version of hte game on steam" (2026-10-18 · 13:18); "live data -> in a ui -> alongside the actual gameplay" (2026-10-18 · 19:03); "aside from the game running" (2026-10-19 · 19:54). Encoded: 2026-10-18 spec §1, §2 (Display: second monitor); native-app spec §1.

2. **Its purpose is to LEARN the stats/math and to think about probabilities and strategies during play.** "in order to LEARN the stats/math around the game and help you think about probabilities and strategies throughout gameplay" (2026-10-18 · 13:18). Encoded: spec §1; `user_profile.md` ("Develop stats-driven decision-making").

3. **Not a cheat: show only what a player at a table could know.** "not a cheat, at all!!!" (2026-10-18 · 13:18); "given what we know about cards left in private information (only have 4 cards shown in the minor pile so far)" (2026-10-18 · 16:02). Encoded: spec §5 (the firewall, permutation-invariance test); report's rollout corollary ("every derived statistic, including Monte Carlo rollouts, must sample hidden orders from the public belief, never from the save").

4. **Augment, don't decide. Present things to think about; never make the call.** "helpful/augmenting certain choices with math/stats, and PRESENTING things to THINK ABOUT during these decisions, but not doing it for them/making the calls directly for them" (2026-10-18 · 17:17). Encoded: spec §7.6 ("No rankings, no 'best move', no highlighted option, no default selection"; think-about prompt catalog); `feedback_deliberate_play_muscle.md`; `user_profile.md` ("Avoid alpha-gaming Brett's decisions").

5. **Build the deliberate-play muscle: pause per phase, per instruction.** Brett (2026-04-19, in `feedback_deliberate_play_muscle.md`): "how to take a step back per turn, per phase, per instruction and think of what the board state is, what possibilities, HOW CAN I ACTUALLY CHANGE THIS GAME STATE, etc. instead of going on auto-pilot". The memory applies it explicitly to the tool UI: "every actionable phase should show the decision-relevant state, not just a 'Next Phase' button" and "The companion never ranks or highlights an option; it shows levers, odds as counts, consequences, and think-about prompts."

6. **Aggregate win/loss statistics are useless; everything is conditional on the specific scenario and the rules in play.** "just a count of wins/losses this information is useless to us… which is more around the specific scenario and rules for and against the player at the given moment" (2026-10-18 · 14:04). Encoded: report ("None takes aggregate win/loss counts as input"); spec §1 non-goal (history analytics).

### What to show

7. **Answer direct questions about a coming step, at a given phase.** "a specific qeustion/need: what % of getting a minor-drawn defend card to help in C4... is this worth it, etc." (2026-10-18 · 15:59); "helpful knowledge on direct questions like the one we're asking about a given phase (e.g. Invader phase -> Ravage)" (2026-10-18 · 16:18); "lets check waht our companion says for... helping pick Growth on this turn" (2026-10-19 · 15:19); "looki nto wwaht kind of advice/information teh comapnion woudl have at this point" (2026-10-19 · 15:45). Encoded: spec §7.5 (anchor question); native-app spec §3 (phase views with "the 2–3 questions of this moment"); questions-from-play ("the screen is the question").

8. **Levers you control vs things you can't control but can count.** "given ALL the things we can a.) directly affect and control b.) that we can't directly affect but can take into account for the things we can" (2026-10-18 · 16:18). Encoded: spec §7.5 (Fixed facts / Levers / Unknowns columns); questions-from-play ("three columns: what's fixed, what you control… what's hidden (counted)").

9. **Lay out growth vs pace, and outgrowing the adversary.** "it should lay out the comparison of how we're growing and keeping pace in either direction. Vs outgrowing an adversary especially" (2026-10-18 · 16:18); "eventually the ramp and acceleration curves / comparison of how fast we grow vs the opponent" (2026-10-19 · 20:58). Encoded: spec §7.4 (momentum projection vs step function), §7.5 output 4 ("the turn, if any, where the economy outgrows the pressure"); `curve.rs`.

10. **Reclaim timing is strategy: postponing Reclaims accelerates growth; show the draw odds AND the acceleration if the draw misses.** "reclaim does NOT accelerate our growth/pace... postponing Reclaims is a MASSIVE strategy proven in SI through and through… show me actual statistical view into % chance of getting a defend card. and if we dont get one, how helpful is that acceleration of growth" (2026-10-18 · 16:02). Encoded: spec §7.5 worked example ("Reclaim-now vs. Reclaim-later pace comparison"); questions-from-play ("'Reclaim now vs later' is an order question, not a count question").

11. **Count every hidden pool that touches the step: Minor/Major draw, banked Fear cards, the Event, the Blight card's other side.** "What about factoring the 1-2 fear cards we'll have available and the Event card" (2026-10-18 · 16:08). Encoded: spec §7.5 unknowns row, §7.2 (Terror Level timing), §7.3 (post-flip mixture).

12. **On-element cards and innates are a core value axis.** "Lure also gives us Shadow elements; this is a massive thing about SI, keeping 'on element' cards, and using innate powers on the Spirit board" (2026-10-18 · 16:55); "we have a defend 5... but off-element. we have lure-- entirely on element brand" (2026-10-19 · 15:38). Encoded: spec §7.4 (three values per card; elements weighted by scarce element); playtest U11, U13.

13. **A new card is judged by what it does on this board this turn, what it pairs with (hand vs discard), and what the Forget costs.** "i do like presenting via teh companion ... the tradeoff and/or pairign with teh new card we'r ekeeping. 'with this major we get X... when we forget Y we gain/lose this... but also the new card [major/minor choice] pairs with <other cards in the hand we have>'" (2026-10-19 · 15:26); "the companion app would show...what a new card this round/turn would accomplish at all in teh current situation ont he board" (2026-10-19 · 15:27); "what is the best card to forget" (2026-10-19 · 15:24). Encoded: playtest U13; native-app spec §3 Gain-a-Power row; questions-from-play "Draft and Forget".

14. **"Saves the land" means any effect, not the Defend keyword.** "Lure...actually solves C4 by gathering out of 4 -> 5 ... its not a defend card but it works for the blight, or cycles of time... removes a blight" (2026-10-18 · 16:54). Encoded: spec §7.4 need finder ("A Defend-only count gave 17 of 96 Minors (55%); all effects give about 27 (74%) — Brett found the miss live").

15. **Beast economy; where Beasts and Events go is key.** "spiders, for beast economy (these come up in a LOT OF EVENTS, and are very ehlpful - beast stocks / stonks)" (2026-10-18 · 17:19); "at the 'Beasts Provoekd' staged, beasts and events are quite powerful. deciding where to put these is key" (2026-10-19 · 15:31). Encoded: playtest U15 (token-placement lens counted from the public Event pool); questions-from-play "Event and Fear choices".

16. **Pipeline invariants are obvious to the companion; Fast vs Slow is a timing choice tied to card type; inland lands can be made unexplorable, coasts cannot.** "Explore->Build->Ravage for each ivnader phase across turns; resolved in reverse order… fast vs slow powers and how those are DIRECTLY correlated to minor/major power cards and innates, usually clearing things int eh slow phase helps for the next rounds; fast powers solve things 'this turn'. also things like: clearing inner-lands prevent Explores due to adjacency, whereas coasts always explore… this should all be obvious to the companion" (2026-10-19 · 03:29). Encoded: spec §7.8 (badges `explore-dead`, `sole-source`, Fast/Slow windows).

17. **Post-game review: momentum shifts, costly picks, prevention vs reaction.** "evaluating post-game things; e.g. where did the momentum shift, where were suboptimal picks made, what can we learn from each scenario asfter the game is over. e.g. certain card plays/combos werent as good - too much focus on ravages and not preventing builds (or explroes if possible)" (2026-10-18 · 19:03). Encoded: spec §7.7, §7.10; `review.rs`; the Prussia 6 playtest "Retro" table.

18. **Retro judged by what was knowable then; decisions separated from luck; flags, not verdicts.** (derived, from spec §7.10, written from Brett's post-game request and the Prussia 6 playtest.) Encoded only in spec §7.10 and the playtest retro table.

19. **Show things a turn ahead, loudly, when they decide the game.** Brett: "T3 roud land ravage, not really didnt see that coming" (2026-10-19 · 18:42). The fix (next turn's Ravage counted against the Blight that will be left; red callout) is Claude's inference recorded in the Blitz playtest. Encoded: spec §7.11 last paragraph; `ledger.rs` "next turn" ghosts.

20. **An unfamiliar adversary's rules must be explained in context, with when they're checked.** Brett: "not really clear, im not familiar with Habs Mining at all" (2026-10-19 · 19:12). Claude's inference: the N2 Rules view. Encoded: native-app spec §6 N2 ("Rules (loss conditions with when they're checked…)"). **That view was deleted in the 20:05 reset** (`views/rules.rs` D) and nothing replaced it.

### What never to show / display style

21. **No rankings, no best move, no highlighted option; options in fixed order.** (derived; spec §7.6, from Brett's 17:17 principle.) Encoded: spec §7.6; `feedback_deliberate_play_muscle.md`.

22. **Natural frequencies with a pocket rule beside each number.** (derived; the report's "Displays that build judgment": "say '2 of the 3 unseen Stage II cards explore here,' not '67%'"; "Pair each exact number with a rule the player can compute at the table".) Encoded: spec §7 preamble, §7.6; `feedback_companion_ground_up_lumia_rei.md`. Note: Brett's own vocabulary is percentages ("what % of getting", 2026-10-18 · 15:59 and 16:02); he never asked for counts over percentages, and never objected to counts.

23. **Deltas, not verdicts: show what changes if you act, never "play this".** (derived; report: "The companion should show 'what changes if you hit land X,' never 'play this.'") Encoded: spec §7 preamble ("show what changes rather than what to play").

24. **Any Energy-per-card-play valuation is a user-set parameter with a sensitivity band, never a fact.** (derived; report: Rei's 2.5E "contested in the same thread"; spec §7.4: "Any Energy-per-card-play valuation is a user-set parameter with a sensitivity band, never a fact".)

25. **Rates are observed values, not fitted; momentum cannot be detected statistically inside one game.** (derived; spec §7.2 "Fear per round (from history), shown as observed values, not a fitted rate"; report §7.9 item 5: "'momentum' cannot be detected statistically inside one game.")

26. **Probability earns a panel only where a hidden card meets a land or a named clock.** (derived; report Conclusion: "A number earns screen space only when it attaches to a land window… or to a named clock".)

27. **An optional, off-by-default "hold" (predict-first) toggle.** (derived; report; spec §7.6 last bullet.) Never mentioned by Brett.

28. **Derived math the player can't grasp at the table, not a restatement of the board.** Brett on the first Ledger: "I guess. It shows basically the info we already know ok the game... But it's a start. Add visualizations, graphs, things that could help statistics and the explanation of things we want to convey" (2026-10-19 · 20:58). Encoded: `feedback_companion_ground_up_lumia_rei.md` ("Restating the board is not enough").

### Layout and visual

29. **One screen, not much scrolling, not everything at once; it must be clear what the app is trying to do to aid you.** "the layout is horrendous... its so much info smashed together, it's way too complicated and not easy to follow; the visualizations are terrible / the island view is so big i have to scroll vertically to see it" (2026-10-19 · 15:07); "the ui rigth now for the companion is horrendous. its so all over the place, there'e SO MUCH INFO showna t once, it doesnt read well at all. i cant tell waht its actually tryign to do to aide me... i imagine a single screen, not much scrolling (if at all) if it's pulle dup on a monitor" (2026-10-19 · 16:02). Encoded: playtest U1–U3; native-app spec §1 ("one screen, no scrolling… showing only what the current moment of the game needs"). Note: the 2026-10-18 spec §2 still says "Always-on dashboard. Every number visible… dense layout" — superseded by Brett but not revised.

30. **Visual fidelity to the game; undo-aware play-by-play; show what a card did to a land.** "paly by play doesnt show undos/overwrite that info its just a stream of logs / mountain card is 'red' in color? its grey on the game" (2026-10-19 · 15:07); "in B8 we placed a defend with a card, the companion doesnt show anything right now on defend... or that a card was applied at all to any specific land" (2026-10-19 · 15:12). Encoded: U4, U5, U8; native-app spec §3 ("undo-aware", "Mountain is grey", "badges for Defend, powers used"). The island and play-by-play regions were deleted in the reset.

31. **Use the stack's visual power; great visualizations; visualization trumps text.** "with all that stack (winit, wgpu, gltf, glyphon, etc.) we should lean towards much more helpful, visual, cool looking things" (2026-10-19 · 19:44); "come up with great visualizations along the way" (20:05); "Add visualizations, graphs" (20:58); "How do we best visualize this, lay it out, for a human brain to comprehend all these things" (22:50); "Is there any kind of source material, techniques, and design paradigms for UX/UI design for such things; where visualization trumps tons of text explanation" (22:58). Encoded: memory ground_up ("He points at the stack's crates… as inspiration"). No spec encodes "visualization trumps text".

32. **Don't reinvent the game inside the companion; the visualizations of the statistics, models, questions and solutions, and the backbone of competing goals, are what is key.** "I mean we don't need to reinvent an entire game inside of a companion for a game. Similar/like concepts are shared. The visualizations of the statistics, models, analytics, questions and solutions; the back one of the competing goals is what is key here" (2026-10-19 · 23:03). Encoded: nowhere (the native-app non-goal "Re-implementing game rules in the UI" is about where rules code lives, not about what the UI shows).

33. **Competing goals are the backbone.** "What about the statistical model you mentioned around competing-something... This is the basis of this game where so many things come into play and compete for attention" (2026-10-19 · 22:47). Encoded: report (competing-risks cumulative incidence, §7.9 item 3, §7.2 later) — the statistical sense only; Brett's "compete for attention" sense is in no document.

### Models

34. **Lumia's ledgers and Rei's momentum are the idiom.** "look throughout this blog for spirit island writeups, on these specific kinds of models/approaches" (2026-10-18 · 14:47); "these blog writeups are the kind of theory/way of thinking i want to be able to approach this; alogn with Rei's specific kidn of thinking, around momentum and keeping ahead of the curve" (2026-10-18 · 14:51); "re-thina bout from the... lumia + rei approaches" (2026-10-19 · 19:54). Encoded: report ("Lumia's ledgers and Rei's curve split 'tempo' into two measurable things"); spec §7 preamble; `feedback_companion_ground_up_lumia_rei.md`; `feedback_rei_format_canonical.md`.

35. **Exact counting and enumeration over the public pool first; simulation only for races and comparisons, with its policy labeled.** (derived; report "Probability earns a panel…": "Counting and enumeration cover most questions"; "Simulation is for races and comparisons, and its estimand must be labeled".) Encoded: spec §7 preamble build order; §7.2 "Race chart, v1 (no play policy needed)".

36. **Stochastic framings only where they fit.** "i only want things taht DO FIT for this tool/compnaion,just mostly curious" (2026-10-18 · 19:03). Encoded: spec §7.9.

37. **Factor adversary rules, expansions, and player/adversary count.** "Are you also being sure to factor into these 1. Adversary rules 2. Expansions 3. 1 vs 2 vs N number of spirits and/or adversaries" (2026-10-18 · 14:25). Encoded: report section "Adversaries, expansions and player count rewrite recipes and clocks, not the math"; spec §7 (N vs B units), §7.11.

### Rules fidelity

38. **Show the ACTUAL numbers with every adversary/scenario modifier applied.** "yeah i mean these shoudl show the ACTUAL things like sweden's extra dmg..." (2026-10-19 · 18:11). Encoded: spec §7.11; `feedback_companion_actual_numbers.md`; Sweden playtest W1–W2.

39. **Printed card wins unless explicit errata.** "always assume the physical/card printed effects, UNLESS tehre's errata explicitly updating these, which the steam version has" (2026-10-19 · 14:34). Encoded: `feedback_printed_card_authority.md`.

40. **Rules questions go to Brett; a systematic rules review of layered expansions.** "If you ever have any rules questions let me know I can clarify them or write them to a document that I can clarify in bulk" (2026-10-18 · 15:05); "even if tehres not 'disagreement' we should probably review the rules in some kind of way especially whena dding a bunch of layered expansion on there" (2026-10-18 · 15:10); "lets go through the open-questions.md one at a time" (2026-10-19 · 12:19). Encoded: `feedback_rules_questions_to_brett.md`; rules register.

41. **Rules fidelity is the hard part; never hallucinate mechanics.** (derived; report Conclusion: "Every quantitative error found in this pass was a misstated rule, not a misapplied distribution.") Encoded: `feedback_never_hallucinate_mechanics.md`; spec §7 ("each verified with `si-rules-check` before coding").

42. **Digitize every panel, card, rule and erratum in machine-readable form first, so later work rests on data.** "would it make much sense to be able to digitize all the SI panels, cards, rules, etc. in a LLM friendly data driven way/form first....to allow for all other things?… so all and any work we do from that point on we know we've defined in config/data/something easily machine-readable" (2026-10-19 · 14:27). Encoded: `2026-10-19-si-corpus-design.md`.

### Process

43. **The back-and-forth in chat during real play IS the design input.** "you adn i here, are doing back and forth to help figure tehse choices out to present this information for the tool, thats fine" (2026-10-18 · 17:17); "Recall when you and I would do things step by step in actual gameplay and I'd ask questions and get ideas for stuff to show? We should have those in context somewhere and should look at how that'd fit the ledger/pace questions, approaches, and statistical analysis/models" (2026-10-19 · 22:38). Encoded: one sentence in `feedback_deliberate_play_muscle.md` ("Working decisions out together in chat is the design process and is fine"); the questions-from-play doc.

44. **Play games with the companion to collect real scenarios.** "we'll also do some play by play throughout it to see live data -> in a ui -> alongside the actual gameplay… we can then runa ny other scenarios you wanna see throughout" (2026-10-18 · 19:03); "if that's playing multiple games with the companion app to collect real scenarios and data that way" (2026-10-19 · 14:27). Encoded: six playtest logs.

45. **Ground up from the Rust structs and data; don't retrofit the old panels.** "the idea wasnt to retrofit all the randomass shit we built months ago on that so we need to rethink how / what to build from the ground up, given all our rust structs and data and such" (2026-10-19 · 19:52). Encoded: `feedback_companion_ground_up_lumia_rei.md`.

46. **Start with the ledger; restart the window; one thing first.** "yeah lets start w/the ledgeer --- lets restart all the window we're building; use the stack (winit, wgpu, gltf, glyphon, etc.) -- come up with great visualizations along the way" (2026-10-19 · 20:05); "What do we want to test out first in the entire plan of these things." (21:57). The memory file sharpens this into "Propose one view, get Brett's yes, build it, let him use it, then the next" — that sentence is the memory's encoding (written 21:28), not Brett's words; it is nonetheless the only reading of "start with the ledger" plus "what do we want to test out first" that fits.

47. **Explain the direction in prose first; no multiple-choice pickers while the framing is wrong.** (unverified) The memory says "He also rejected multiple-choice pickers twice while the framing was wrong; explain the direction in prose first." No message in the verbatim record says this. Brett's closest words: "I feel like I'm stuck explaining it to you so what should we do here......" (22:35). Treat as plausible but unconfirmed.

48. **Companion window is read-only toward the game and toward tabletop state.** `feedback_never_put_live_state.md` (2026-04-20 incident); spec §2 ("Steam mode is read-only… Tabletop state is never read or written by it"); native-app §5. Brett's own pasted text (2026-10-18 · 19:03) includes "Separate data/steam-game.json, so your tabletop game is never written."

49. **Rust for every component; REA for the engine model.** "Why a C# version of the process inspector route? We should use rust and @~/repos/rea" (2026-10-18 · 13:31). Encoded: spec §2.

50. **Memory-file and assistant-account quotes that are NOT in the record.** The assistant's account and `feedback_companion_ground_up_lumia_rei.md` quote Brett saying: "feel like you're trying to retrofit the old stuff in with the new stuff"; "I would like the math and stats that are pulling similar to lumia and rei, and convey that in a helpful and visually pleasing way to augment what players probably already know about the game but can't quite always grasp"; "is the pace only fear victory, or tier 1–3 wins too"; and that he "had provided screenshots and 4–5 plugins for visualization". None of these sentences appear in `brett-messages.md`, which is presented as "every message Brett wrote across both sessions, verbatim". Either the export is incomplete (there are gaps between 20:58 and 21:57, and between 22:02 and 22:35) or the account paraphrased. Per the review rules, Brett's messages win: these lines must not carry weight as principles until Brett confirms them (see §6, Q12).

---

## 3. The evolution of the plan

| When | Brett's words | What it changed |
|---|---|---|
| 2026-10-18 · 13:18 | "lets morph / utilize this app to be the 'spirit island companion' showing percentages, statistics… in order to LEARN the stats/math… not a cheat, at all!!!" | Founding. si-live becomes a Steam companion. |
| 2026-10-18 · 13:31–13:35 | "Why a C# version… We should use rust and @~/repos/rea"; "i dont need riff here. just do our plan with this session" | Rust only; REA for the engine; no side quests. |
| 2026-10-18 · 14:04–14:51 | "What statistical models… Check actual statistics references, books… cross correlate with BGG threads, blogs"; "I don't care about 'statistics' the makers… collect"; "factor into these 1. Adversary rules 2. Expansions 3. 1 vs 2 vs N"; "look throughout this blog [Lumia]"; "alogn with Rei's specific kidn of thinking, around momentum and keeping ahead of the curve" | The research report; Lumia + Rei become the idiom; conditional-on-state models only. |
| 2026-10-18 · 15:02–15:10 | T1 Event rule corrected; "If you ever have any rules questions let me know"; "we should probably review the rules in some kind of way" | Rules go to Brett; rules register planned. |
| 2026-10-18 · 15:26–17:25 (Test 1) | 15:59 "this is where the companion app would really help: a specific qeustion/need"; 16:02 Reclaim/Defend-draw question; 16:08 Fear cards + Event; 16:18 the levers/unknowns/pace framing; 16:20 "Yeah mock that up"; 16:55 on-element; 17:17 augment-don't-decide + "back and forth… thats fine"; 17:19 beast economy | Spec §7.4 need finder, §7.5 scenario lens, §7.6 augment-don't-decide, all written from this play. |
| 2026-10-18 · 19:03 (re-sent 2026-10-19 · 03:29) | "i dont see any of this work actually started; go back to our plan... start building. we'll also do some play by play throughout it to see live data -> in a ui -> alongside the actual gameplay"; post-game review; Bartlett "only… things taht DO FIT"; pipeline invariants "this should all be obvious to the companion" | Build pressure; spec §7.7, §7.8, §7.9. |
| 2026-10-19 · 12:19–13:45 | "lets go through the open-questions.md one at a time" (Q1–Q24 answered) | Rules settled; printed-card precedence. |
| 2026-10-19 · 14:27 | "would it make much sense to… digitize all the SI panels, cards, rules, etc.… first… Then after that plan on next steps...if that's playing multiple games with the companion app to collect real scenarios" | Corpus spec; data-first; playtest program. |
| 2026-10-19 · 14:34–14:58 | "always assume the physical/card printed effects, UNLESS tehre's errata"; "sure write taht up as a spec next"; "commit it now"; "dont open a pr just merge it into main" | Printed-card authority memory; corpus spec merged. |
| 2026-10-19 · 15:02–15:53 (Prussia 6, live) | 15:07 "the layout is horrendous… the visualizations are terrible… scroll vertically… paly by play doesnt show undos… mountain card is 'red'"; 15:12 Defend not shown; 15:19 "helping pick Growth"; 15:24–15:27 draft/forget tradeoffs and "what a new card… would accomplish"; 15:31 Beasts/Events placement; 15:45 "this is grim… what kind of advice/information teh comapnion woudl have" | U1–U19; spec §7.10 retro; the first explicit layout rejection. |
| 2026-10-19 · 16:01–16:02 | "ok lets take as tep back now… what did you need next"; "1. do it 2. do it… 4. do it"; "the ui rigth now for the companion is horrendous… i imagine a single screen, not much scrolling" | Reset #0 (the web UI judged unfit). Coverage games start. |
| 2026-10-19 · 16:16–19:12 (coverage games) | 17:32 "the companion window that launched on windows native is... just blank?"; 18:11 "these shoudl show the ACTUAL things like sweden's extra dmg"; 18:42 "T3 roud land ravage, not really didnt see that coming"; 19:12 "im not familiar with Habs Mining at all" | §7.11 adversary coverage; loss clocks a turn ahead; N1/N2 native app (phase views, Rules view). |
| 2026-10-19 · 19:42–19:54 | "i closed the existing windows desktop app, re-run it for me?"; "with all that stack… we should lean towards much more helpful, visual, cool looking things"; "again, show me the reset/plan we started yesterday"; "the idea wasnt to retrofit all the randomass shit we built months ago… rethink how / what to build from the ground up"; "re-thina bout from the... lumia + rei approaches… aside from the game running" | **Reset #1.** N1/N2 panels (phase, island, fear, clocks, spirit, plays, rules, event, slow) judged a retrofit; later deleted. |
| 2026-10-19 · 20:05 | "yeah lets start w/the ledgeer --- lets restart all the window we're building; use the stack… come up with great visualizations along the way" | **Reset #2.** The Ledger is the first (and only authorized) view. |
| 2026-10-19 · 20:58 | "I guess. It shows basically the info we already know ok the game... But it's a start. Add visualizations, graphs, things that could help statistics and the explanation of things we want to convey around the ledger and eventually the ramp and acceleration curves / comparison of how fast we grow vs the opponent" | Lukewarm yes on the Ledger; "eventually" for the curves. (Claude then built Runway, Pace, Curve, what-if, lab in one pass — the account admits "without a go-ahead on any single piece".) |
| 2026-10-19 · 21:57–22:02 | "What do we want to test out first in the entire plan of these things."; "Think entirely from the perspective of a player who is looking to augment their gameplay. And learn. And tinker"; "Go ahead and test it" | Brett no longer knows what the pieces are for; re-anchors on the player. |
| 2026-10-19 · 22:35 | "Yeah I still find the entire thing strange to look at. I think you're still way far away on even a good direction for this layout and what the app is trying to accomplish. I feel like I'm stuck explaining it to you so what should we do here......" | The second build rejected. |
| 2026-10-19 · 22:38 | "Recall when you and I would do things step by step in actual gameplay and I'd ask questions and get ideas for stuff to show? We should have those in context somewhere and should look at how that'd fit the ledger/pace questions" | Back to the questions from play as the source; `2026-10-19-questions-from-play.md` written. |
| 2026-10-19 · 22:47–23:03 | "the statistical model you mentioned around competing-something... This is the basis of this game"; "How do we best visualize this, lay it out, for a human brain"; "source material… where visualization trumps tons of text explanation"; "we don't need to reinvent an entire game inside of a companion… the back one of the competing goals is what is key here" | Three new framings (competing goals, visualization over text, don't reinvent the game) with no document home. |
| 2026-10-19 · 23:05 | "launch whatever multi-agent, cohesion referee, etc. you need to… the direction is lost. The concepts are being eroded and overwritten. This need to be deeply thought out not just a quick fire answer" | This review. |

---

## 4. Erosions and overwrites

Ranked by severity. "Blocks the purpose" means the companion, as built, cannot do the thing it exists for; "dilutes" means a principle is weakened or contradicted; "cosmetic" means fixable without changing direction.

### E1 — Blocks the purpose. The questions from play are not what was built; the budgets are.
- **Principles:** 7, 8, 10, 12, 13, 15, 43.
- **Evidence.** Brett asked the Growth question five times and the Draft/Forget question five times (questions-from-play tables; 2026-10-18 · 15:59, 16:02, 16:08, 17:04–17:08; 2026-10-19 · 15:19–15:27, 15:36–15:38). The spec made "the scenario lens" the core interaction on 2026-10-18 (§7.5) and the native-app spec put Growth and Draft views at milestone N3. What exists in `crates/si-app/src/views/` after the reset is `ledger.rs`, `runway.rs`, `pace.rs`, `curve.rs`, `whatif.rs`, `chart.rs`: three budget lanes (Blight, Fear, Invader deck, plus an adversary clock) and two pages ("THE LEDGER", "THE CURVE"; `Page` enum, `mod.rs:77`). There is no Growth view, no Draft/Forget view, no Event-choice view, no "my plan vs this Ravage" view. Nothing in the native app computes per-growth-option pace, Reclaim-now-vs-later, hand-vs-discard pairings, element contribution to innates, or the token-placement count from the Event pool. The questions-from-play doc itself concedes this: "Of what was built on 2026-10-19… the core math carries over… The layout does not."
- **Brett's reaction.** "It shows basically the info we already know ok the game" (20:58); "What do we want to test out first in the entire plan of these things." (21:57); "I still find the entire thing strange to look at… way far away on even a good direction for this layout and what the app is trying to accomplish" (22:35); "Recall when you and I would do things step by step… and I'd ask questions and get ideas for stuff to show?" (22:38).
- **Why this is an overwrite, not an omission.** The Ledger page organizes the screen by *budget* (how much Blight/Fear/deck is left) and the Runway by *time* (four phases forward). Brett's framing (16:18) organizes by *decision*: anchor question, then levers, then countable unknowns, then pace. The budgets were supposed to be "the stakes line under the plan" (questions-from-play: "The budgets… are a narrow status strip, not the page"). Building the budgets as the page inverted the hierarchy, and the lever column — the thing that distinguishes a learning aid from a dashboard — has no home.

### E2 — Blocks the purpose. Think-about prompts and the need finder are computed and never shown.
- **Principles:** 4, 5, 7, 14, 21.
- **Evidence.** `common/src/companion_stats/mod.rs:458` defines `prompts()` (the §7.6 catalog) and `mod.rs:203` computes it into the stats; `needs.rs` (409 lines) implements the need finder Brett asked for at 15:59 on 2026-10-18. A grep of `app/src` finds no reference to `prompts`, `needs`, or `review` from any native view. The memory `feedback_deliberate_play_muscle.md` says the companion "shows levers, odds as counts, consequences, and think-about prompts"; the native app shows none of the prompts. The "hold" toggle (principle 27) is likewise absent, which is fine on its own, but together with the missing prompts the app has no pause-point mechanism at all — the exact habit the memory says every player-facing surface must build.
- **Brett's reaction.** "i cant tell waht its actually tryign to do to aide me" (16:02, about the web UI, and functionally repeated at 22:35).
- **Severity note.** Without prompts or a lever column, what remains is numbers and charts; that is a dashboard, which is what Brett said at 15:07 and 16:02 he could not read.

### E3 — Blocks the process (and is the proximate cause of "the direction is lost"). Four views were built in one pass without a yes on any.
- **Principles:** 43, 46, (47 unverified).
- **Evidence.** Brett authorized one thing: "lets start w/the ledgeer" (20:05). His reaction to it was "I guess… But it's a start. Add visualizations, graphs… around the ledger and *eventually* the ramp and acceleration curves" (20:58, emphasis mine). The assistant's account, item 3: "Then, in one pass without a go-ahead on any single piece: the Runway…; the Pace…; a second page, the Curve…; then a what-if layer… and a 'lab' replay." The memory file that encodes "Propose one view, get Brett's yes, build it, let him use it, then the next" is time-stamped 21:28 — i.e., it was written while or after the rule it states was being broken. `git status` shows ~1,160 insertions across 27 files, uncommitted, for the evening.
- **Brett's reaction.** "What do we want to test out first in the entire plan of these things." (21:57); "I feel like I'm stuck explaining it to you" (22:35).
- **Why it matters.** Principle 43 says the content comes from the back-and-forth. Four views built silently gave Brett nothing to react to one at a time, so his reactions could only be global ("the entire thing strange to look at"), which is exactly the signal the assistant then could not act on.

### E4 — Blocks (document level). Three specs and the code describe four different layouts; none was retired.
- **Principles:** 29, 45, 46.
- **Evidence.** (a) `2026-10-18-steam-companion-design.md` §2 (rev 7, last touched 2026-10-19 15:10): "Teaching posture: Always-on dashboard. Every number visible, updated live… Display: Second monitor, Windows browser, dense layout." Brett rejected density at 15:07 and 16:02. Not revised. (b) `2026-10-19-native-app-design.md` §3 (last commit 15:21): four regions — Clocks | Phase view | Island + Pipeline | Play-by-play — with N2 "Built". Brett called that a retrofit at 19:52; the views were deleted at 20:05; the spec still says "Built" and still lists N3–N6 against that layout. (c) `2026-10-19-questions-from-play.md` (18:41, uncommitted): "One screen per moment, headed by the question… budgets… are a narrow status strip, not the page." (d) The code: two pages, Ledger lanes + Curve, budgets *as* the page. Nothing says which document is current. This is the literal meaning of "the concepts are being eroded and overwritten": each reset added a layout without retiring the previous one.
- **Brett's reaction.** "again, show me the reset/plan we started yesterday" (19:51) — he asked for the plan and got a new build instead.

### E5 — Dilutes. Verdict language crept into views that were supposed to show deltas.
- **Principles:** 4, 21, 23.
- **Evidence.** `app/src/views/ledger.rs:367–384` has a function literally named `verdict` producing the lane headline ("holds this turn", "prevent 5 to hold", "+N this turn (best case): earns the card"). `curve.rs:295`: `let verdict = if ri > ry + 0.02 { "the Invaders are pulling away" } else if ry > ri + 0.02 { "you are gaining on them" } else { "about even" }`. The Pace caption (screenshot `v3-prussia.png`): "At that pace: Terror III on T8; the deck runs out before Victory." These are not rankings of options (so §7.6's hard line is not crossed), but "pulling away / gaining on them" is a judgment label of exactly the evaluation-bar kind the report warned against ("students who chase the evaluation bar 'learn moves without learning ideas'"), and it is fitted from a 3-turn compound rate (see E6). The what-if drawer sorts lands by a `rank` closure (`whatif.rs:50,70`) — a threat ordering, not an option ranking; cosmetic, but the word is a tell.
- **Brett's reaction.** None specific; he never saw these in isolation (E3).

### E6 — Dilutes. The Curve breaks three model principles its own doc comment states.
- **Principles:** 24, 25, 22.
- **Evidence.** `common/src/companion_stats/curve.rs` header: "Natural units only: no exchange rate between the two." `app/src/views/curve.rs:28`: `const PLAY_E: (f64, f64, f64) = (1.5, 2.5, 3.5);` and line 359: "Growth combines a card play at 2.5 Energy (Rei's chart; contested); the band spans 1.5 to 3.5". Spec §7.4: "Any Energy-per-card-play valuation is a user-set parameter with a sensitivity band, never a fact." The band is present; the user-set part is not; the model layer and the view layer contradict each other. `curve.rs:284–299` computes a compound growth rate over the last three turns and prints it as "{:+.0}% a turn" — a fitted rate on 3 points, which the report (§7.9 item 5) says cannot carry information inside one game, and a percentage where principle 22 asks for counts. "WHO'S GROWING FASTER… as multiples of Turn 1" is a ratio chart, not the crossover turn Brett named ("the turn, if any, where the economy outgrows the pressure", §7.5).
- **Brett's reaction.** "eventually the ramp and acceleration curves" (20:58) — he asked for this later, not now, and did not ask for an exchange rate.

### E7 — Dilutes. The Runway answers a question nobody asked, and its policy assumption is one the project's own doc rejects.
- **Principles:** 7, 8, 26, 35.
- **Evidence.** `runway.rs` header: "played forward for every order the unseen Invader cards can come in, with no Spirit acting". `outlook.rs`: "A floor on the pressure, not a prediction of the game." The report did propose an "invaders-only projection, enumerated exactly over the next few cards ('if you do nothing')" — but as one of three ingredients of a *race chart* floor, not as the dominant panel of the first page. The questions-from-play doc (written after) says of the timelines model: 'Not "nobody acts for four turns".' The Runway occupies roughly half of the Blight lane (screenshot `v3-prussia.png`) and has no lever: nothing the player controls appears on it. Lumia's "darkest and best timeline" is a *next-card* enumeration (one card ahead, at most six cases) tied to specific lands; the Runway aggregates to a Blight-left number four phases out, which is neither a land window nor a clock the player can act on this turn (principle 26).
- **Brett's reaction.** No direct one; subsumed in 22:35.

### E8 — Dilutes. The first Ledger restated the board.
- **Principles:** 28, 31.
- **Evidence.** Screenshot `ledger-hme.png` (the Ledger before charts): a row of Blight tokens, a row of Fear tokens, a row of deck cards, each with a count — the same numbers the Steam UI shows in its own panels. Brett's reaction is the principle: "It shows basically the info we already know ok the game" (20:58). The fix that followed (E7, E6) added charts rather than the derived quantities Brett had listed (Reclaim acceleration, Defend-draw odds, pairings).

### E9 — Dilutes. Text explains what the visuals should carry.
- **Principles:** 31, 29.
- **Evidence.** Screenshot `v3-prussia.png`: under the Runway, three lines of prose ("Unanswered, the card is already on its blighted side. Lost to Blight by: T5 8 in 10 · T6 10 in 10…"; "To keep Blight on the card, prevent by: T5 5 · T6 11…"; "Each line is an outcome, stronger the more card orders lead to it; the band holds the middle 80%."); under Pace, two more; under each lane, a pocket-rule sentence; a legend line in each header. The page explains itself in captions because the marks do not carry the meaning. Brett at 22:58 asked for "design paradigms… where visualization trumps tons of text explanation."
- **Brett's reaction.** "I still find the entire thing strange to look at" (22:35).

### E10 — Dilutes. The what-if models abstract edits, not the player's levers.
- **Principles:** 8, 32.
- **Evidence.** `whatif.rs`: "Click a piece to destroy it, add Defend or Dahan; every number recomputes." Spec §7.5 lists the levers as "Growth option, which track a presence comes off, cards to play, targets, Event choices, fear-card targeting, payments." A free piece editor is the tabletop tracker's board editor in a drawer — close to the retrofit Brett rejected — and it lets the player try things that no card in hand can do, which teaches nothing about *this* decision. Brett's 23:03 "we don't need to reinvent an entire game inside of a companion" was said after this was built and in a broader context; I flag the drawer as at risk under it rather than a confirmed violation. His 22:00 "And tinker" could be read as endorsing a what-if; which kind is an open question (§6, Q7).

### E11 — Dilutes. Things Brett asked for were built in N2, then deleted in the reset, with no replacement.
- **Principles:** 20, 30.
- **Evidence.** `git status`: `D views/rules.rs` (the Rules-in-play view built for "im not familiar with Habs Mining at all", 19:12), `D views/plays.rs` (the play-by-play that was to become undo-aware, U4), `D views/island.rs` (land badges for Defend/powers used, U8), `D views/spirit.rs` (elements, innate levels; and the Blitz "all powers Fast" fix, B7), `D views/phase.rs`, `event.rs`, `fear.rs`, `clocks.rs`, `slow.rs`. Brett did say "lets restart all the window we're building" (20:05), so the deletion was authorized; but the needs those views answered were his, and the new pages have no element/innate display, no rules-in-play, no play-by-play, no land-level Defend badge. The Ledger's next-turn ghosts and the adversary loss-clock lane (`v2-27.png`, "HUNTERS SWARM THE ISLAND") are the parts of N1/N2 that did survive.

### E12 — Dilutes (record integrity). The memory and the account cite Brett lines that are not in the record, and one built feature rests on them.
- **Principle:** 50.
- **Evidence.** The Pace view's "OR CLEAR THE BOARD" column (board-clear route per Terror Level) is justified in the memory by "He also asked whether the Fear pace counted Terror Level I–III board wins; it hadn't, so the board route was added." That question is not in `brett-messages.md`. Likewise "pulling similar to lumia and rei… visually pleasing way to augment what players probably already know… but can't quite always grasp" is the memory's headline justification and is not in the record. The account's item 4 ("Brett also said he had provided screenshots and 4–5 plugins for visualization; Claude could not find them in the session and did not stop to ask") is also absent from the record — and if true, is itself a process failure (not stopping to ask). These may be real messages missing from the export; until Brett confirms, they are not principles. The memory files are what future sessions read first, so a fabricated or paraphrased quote there becomes "Brett said" forever.

### E13 — Cosmetic. Mixed number formats.
- **Principle:** 22.
- **Evidence.** Natural frequencies are used in places ("8 in 10", "1 of 4", "24 card orders") and pocket rules are present on every lane — this principle largely survived. But the Runway shades by `p_loss` as a probability, the Curve prints "+N% a turn", and the spec's own worked examples (§7.4, §7.5) mix "17 of 96" with "55%" and "79%". Pick one and state it.

### What survived intact (so it is not lost in the noise)
- The firewall (principle 3): the Runway enumerates unseen orders from the public pool; the Blight mixture is over unseen cards; nothing reads the save's deck. No erosion found.
- Actual numbers (38), printed-card precedence (39), rules-to-Brett (40), adversary coverage §7.11: intact and verified against seven recorded games.
- Loss clocks a turn ahead (19): present as ghost tokens and the adversary lane.
- Pocket rules (22, half): present on every lane.

---

## 5. Concepts at risk of being lost

Each: Brett's words; where it lives now (if anywhere); why it is at risk.

1. **"The back-and-forth we do here IS the design input."** — "you adn i here, are doing back and forth to help figure tehse choices out to present this information for the tool, thats fine" (2026-10-18 · 17:17); "Recall when you and I would do things step by step in actual gameplay and I'd ask questions and get ideas for stuff to show? We should have those in context somewhere" (2026-10-19 · 22:38). Lives in: one sentence of `feedback_deliberate_play_muscle.md`; the questions-from-play doc (uncommitted). At risk because no spec names it as the *method*: specs list features, and the evening's build skipped the back-and-forth entirely (E3).

2. **Levers you control vs things you can't control but can count.** — 2026-10-18 · 16:18 (quoted in §1). Lives in: spec §7.5 table; questions-from-play "three columns". At risk: no built view has a lever column; the what-if drawer substitutes free edits for the real levers (E10).

3. **Outgrowing the adversary.** — "Vs outgrowing an adversary especially" (2026-10-18 · 16:18); "comparison of how fast we grow vs the opponent" (2026-10-19 · 20:58). Lives in: spec §7.5 output 4 ("the turn, if any, where the economy outgrows the pressure"). At risk: the Curve replaced the crossover turn with a growth-multiple ratio and a 3-turn fitted rate (E6); nothing computes "the turn you outgrow it" per growth option.

4. **Postponing Reclaims is a massive strategy; show the acceleration if the draw misses.** — 2026-10-18 · 16:02 (quoted in §2.10); "im leaning Reclaim + gain Major; i have 5 energy, and +3 each round... so the enegy economy is starting" (2026-10-18 · 17:08); "we should relcaim, 0 cards in chand" (2026-10-19 · 15:36). Lives in: spec §7.5 worked example; questions-from-play (Growth). At risk: no code computes a Reclaim schedule or per-option pace; the Curve projects "one presence a turn from one track", which is not a growth option.

5. **On-element cards and innates.** — 2026-10-18 · 16:55; 2026-10-19 · 15:38 (quoted in §2.12). Lives in: spec §7.4 "three values"; U11, U13. At risk: `views/spirit.rs` (the only native display of elements and innate levels) was deleted; nothing replaced it; the draft lens was never built.

6. **Beast economy.** — "spiders, for beast economy (these come up in a LOT OF EVENTS, and are very ehlpful - beast stocks / stonks)" (2026-10-18 · 17:19). Lives in: nowhere as a model. The nearest artifact is U15's count ("35 of 58 unseen Events have a Beast section") in the Prussia 6 playtest. At risk: not in any spec section as a stat; the "repeatability" value in §7.4 is the closest hook.

7. **Where to put Beasts/Events is key.** — "beasts and events are quite powerful. deciding where to put these is key" (2026-10-19 · 15:31); "Beasts Provoked: Brett put the Beast on B6" (playtest T3). Lives in: U15; native-app spec Event row ("token placement lens (U15)"); questions-from-play. At risk: the native-app spec that carried it is the retired layout; nothing built.

8. **Pairings and tradeoffs for draft/forget; what the new card accomplishes on this board.** — 2026-10-19 · 15:24, 15:26, 15:27 (quoted in §2.13). Lives in: U13 (the most detailed note in the playtest), questions-from-play. At risk: milestone N3 never started; the corpus card data it depends on is a separate spec.

9. **Retro, judged by what was knowable then; luck separated from decisions.** — Brett's request: "where did the momentum shift, where were suboptimal picks made… too much focus on ravages and not preventing builds" (2026-10-18 · 19:03). Lives in: spec §7.7, §7.10; `review.rs`; the Prussia 6 "Retro" table (a hand-written reference answer). At risk: no native Review page; the "lab" replay is a save-stepper, not the retro; N4 untouched.

10. **The competing goals are the backbone.** — "What about the statistical model you mentioned around competing-something... This is the basis of this game where so many things come into play and compete for attention" (2026-10-19 · 22:47); "the back one of the competing goals is what is key here" (23:03). Lives in: the report's competing-risks model (which clock ends the game first), which is narrower than what Brett means ("compete for attention": Ravage vs Build vs Fear vs economy vs Presence, this turn). No document captures Brett's sense. At risk because the only encoded meaning is the statistical one, and a future session will build a cumulative-incidence chart and call it done.

11. **Visualization trumps text.** — 2026-10-19 · 22:58 (quoted in §2.31); "How do we best visualize this, lay it out, for a human brain to comprehend all these things" (22:50). Lives in: nowhere. The native-app spec's "Visual language" paragraph is a palette, not a principle.

12. **Don't reinvent the game inside the companion.** — 2026-10-19 · 23:03 (quoted in §2.32). Lives in: nowhere in the UI sense. At risk: the island map, the piece-level what-if, and any future "tabletop mode" in the same window all sit on the wrong side of this line unless Brett draws it (§6, Q7).

13. **Augment, learn, tinker — from the player's seat.** — "Think entirely from the perspective of a player who is looking to augment their gameplay. And learn. And tinker" (2026-10-19 · 22:00). "Tinker" is new as of this message and is in no document. At risk: it is the only word Brett has offered that could license a what-if, and nobody has asked him what he meant.

14. **Fast vs Slow as a timing choice tied to card type; Slow clears for next turn, Fast solves this turn; inland lands can be made unexplorable.** — 2026-10-19 · 03:29 (quoted in §2.16). Lives in: spec §7.8 (`explore-dead`, `sole-source`, Fast/Slow windows) and `invaders.rs`. At risk: no native view shows a Fast/Slow window or an explore-dead badge; the deleted phase views had "Slow cards marked too late in the Fast Phase".

15. **Fear-card timing as an accelerator (cross the divider before the Fear phase).** — Test 1 Turn 3: "i chose Dispaly Your Wrath to get taht second Fear card" (2026-10-18 · 17:00); "banked fear cards are FACE DOWN by default until they're revealed in the Fear card phase" (15:56). Lives in: spec §7.2 ("the value of crossing the next divider *before* the Fear phase"). At risk: the Pace view shows Terror marks and a pace line, not the this-turn divider value.

16. **The anchor question ("this turn's Ravage", "does the blight card flip this turn", "can I earn a fear card before Ravage").** — 2026-10-18 · 16:18. Lives in: spec §7.5 only. At risk: no view is headed by a question.

17. **Undo-aware play-by-play; Defend badges on lands; terrain colors matched to Steam.** — 2026-10-19 · 15:07, 15:12. Lives in: U4, U5, U8; native-app spec §3. At risk: the views were deleted (E11); Mountain grey survived in the land chips.

18. **Rules in play explained in context, with when they are checked.** — "not really clear, im not familiar with Habs Mining at all" (2026-10-19 · 19:12). Lives in: the Habsburg playtest; the deleted `rules.rs` view. At risk: deleted without replacement (E11).

19. **Play multiple games to collect real scenarios; let Brett use a view before the next.** — 2026-10-19 · 14:27; 20:05/21:57. Lives in: the six playtest logs (a good record). At risk: the evening's views were tested by screenshot replay ("Go ahead and test it", 22:02) rather than by Brett playing with one.

20. **Rules review of layered expansions, scheduled, not blocking.** — 2026-10-18 · 15:10. Lives in: `feedback_rules_questions_to_brett.md`, the rules register. Not at risk; listed for completeness.

---

## 6. Open questions only Brett can answer

Each changes what gets built. One line each.

1. **Page organization.** Is a screen organized by the *question of the moment* (Growth / Draft+Forget / my plan vs this Ravage / Event choice), with the budgets (Blight, Fear, deck, clocks) as a thin strip — or by the budgets, with the questions inside each lane?

2. **First screen.** The questions-from-play doc proposes Growth on Test 1 Turn 3 first, then "my plan vs this Ravage" on Prussia 6 Turn 5. Is that the order, and is a mock-up in prose/sketch wanted before any code?

3. **The Runway.** Should a "nobody acts for four turns" projection exist at all, or only the one-card-ahead "if Jungle is next, these lands build" table tied to named lands?

4. **Percentages vs counts.** You ask "what %"; the spec shows "27 of 96" plus a pocket rule. Both, or counts only?

5. **One-line summaries.** Is "holds this turn / prevent 5 to hold" acceptable, and is "the Invaders are pulling away" a verdict the companion should never utter?

6. **Rei's 2.5 Energy per card play.** A slider you set, natural units only (no combined economy), or drop "who's growing faster" entirely?

7. **"Don't reinvent the game."** Does that rule out the island map and the piece-level what-if (destroy a piece, add Defend), or only a board editor? What does "tinker" mean to you: trying your *actual* options (G1/G2/G3, which card, which target) or free edits?

8. **"Competing goals."** The statistical model (which clock ends the game first, as curves), the per-turn attention competition (Ravage vs Build vs Fear vs economy vs Presence, this turn), or both — and which first?

9. **Deleted N2 views.** Should a Rules-in-play view, an undo-aware play-by-play, and land-level Defend/powers-used badges come back in the new design, or are they out?

10. **Process contract.** One view → your yes → you play a real game with it → the next. Is that the contract, and does "a view" mean one question screen?

11. **Text.** Is the caption style under each chart (two or three sentences of explanation) what "visualization trumps text" means to remove?

12. **The missing quotes.** Did you say these, and if so where: "I would like the math and stats that are pulling similar to lumia and rei… visually pleasing way to augment what players… can't quite always grasp"; "is the pace only fear victory, or tier 1–3 wins too"; "feel like you're trying to retrofit the old stuff"; and did you share screenshots and 4–5 visualization plugins (where are they)? If not, those lines come out of the memory file.

13. **Retro timing.** Is the post-game review (where it turned, luck ledger) wanted in the native app before or after the live question screens?

14. **Which spec is current.** May the 2026-10-18 spec's "dense always-on dashboard" (§2) and the native-app spec's four-region layout (§3) be marked superseded, with the questions-from-play doc promoted to the design of record once you have answered 1–3?
