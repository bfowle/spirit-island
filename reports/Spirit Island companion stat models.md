# Count the Lines, Then Price the Curve

The best model for a fair-play Spirit Island companion is a stack of exact ledgers, not a win-probability engine. Probability belongs only where a hidden card meets a land. The Steam engine fixes every invader, fear, event and power-card order at setup from one seeded generator. It then resolves Ravage, Build, damage, fear and blight deterministically from public piece counts. So the only uncertainty a tabletop player faces is the order of exchangeable unseen cards ([engine IL, `Game::SetupRNG`, `Game::.ctor`, `CreateInvaderDeck`][dll]). That structure makes lumia's way of thinking directly computable: hard budgets, land-by-land inventories, resolution order, growth classes, and darkest-timeline bounds ([lumia glossary](https://iam.lumia.gay/si_glossary); [growth in spirit island](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)). It also turns Rei's "stay ahead of the invader curve" into a literal plot: a deterministic economy projection from the spirit's presence-track graph against an invader-pressure step function built from the adversary's deck recipe. Probability models serve that frame in strict order:
1. Exact card counting (the hypergeometric family plus an exchangeability lemma) feeds a per-land threat map tagged with lumia's land-tempo windows.
2. An "if this card comes next" table shows the darkest and best timelines.
3. Common-random-number Monte Carlo is used only for multi-turn races. It is shown as competing-risk cumulative incidence over named clocks, bracketed by a rollout lower bound and a hindsight upper bound.
4. Depth-one expectimax, with chance nodes placed exactly at reveals, shows consequences and never verdicts.

Adversaries, expansions and player count mostly change the inputs to these models (deck recipes, extra clocks, token modifiers, players N versus boards B), not the models themselves. They do break two v1 assumptions. Six of eight adversaries carry a loss clock that the v1 race chart leaves unmodeled. The project's adversary-stack data, fear-track model and several fundamentals chapters state rules that the engine and the rulebooks contradict. Appendix A lists the questions for the statistician and Appendix B is the physical-copy checklist.

## One seeded shuffle at setup leaves only card order uncertain

All randomness in the Steam game happens at setup. `Game::SetupRNG` seeds one serializable copy of the .NET subtractive generator from the wall clock. `Game::.ctor` then builds the adversaries, island, spirits, event deck, blight card, fear values, invader deck, fear deck and power decks in a fixed order ([engine IL][dll]; [`Handelabra.SerializableRandom`][ecdll]).

How each deck is built:
- **Invader deck:** sequential uniform picks without replacement, stage by stage: 3 of 4 Stage I cards, 4 of 5 Stage II, 5 of 6 Stage III. The cards go into the slot the engine titles "Invader Deck."
- **Fear deck:** a uniform random subset of the allowed fear pool. It is stacked as {3,3,3} plus the adversary level's cumulative modifiers, with Terror Level dividers between the groups.
- **Event, Minor and Major decks:** a Durstenfeld Fisher–Yates shuffle ([`CreateInvaderDeck`, `CreateFearDeck`, `GenericListExtensions.Shuffle`][dll]).

After setup, a static cross-reference of all 36,017 engine methods finds the generator consumed only by power-deck reshuffles, France's and Sweden's escalations, one Fractured Days power, and three scenarios ([engine xref][dll]). **Randomness during play is therefore epistemic: a hidden order fixed at the start, not fresh sampling.** The space of invader orders is small. The base deck has 4!·5!·6! = 2,073,600 orderings, a count Steve496 posted and the community research rechecked ([BGG 2785706](https://boardgamegeek.com/thread/2785706)).

Resolution is deterministic given the board. The engine's constants table fixes these values ([`Constants`][dll]):
- Health: Explorer 1, Town 2, City 3, Dahan 2, land 2.
- Fear from destruction: Town 1, City 2, Explorer 0.
- Fear pool: 4 fear markers per spirit.
- Decks: 9 base fear cards; 3, 4 and 5 invader cards per stage.

Land damage at or above the land's current health (2 plus modifiers, floor 1) adds exactly one blight, because the call passes a literal 1. The land's damage then resets. Blight cascades only when the land already held blight, and the player picks the adjacent land it goes to ([`DealDamageAction.AfterAction`, `ShouldBlightCascade`, `CascadeBlight`][dll]; [LOG-B][logB]). Every default passes through an "ask" protocol of 138 `AskType` hooks and 46 status-effect types. This protocol is how adversaries, tokens and powers bend the base rule ([engine model and controllers][dll]). The engine also ships its own preview layer (`GetRavagePreviewForLand`, `GetInvaderActionPreviewForLand`, `IsTerrorWinPossible`). That layer is a ready oracle for regression-testing the companion's `rules.rs` against the game ([dll]).

Hidden information is narrow, and its reveal points are hard commits. Cards carry `IsKnown` and `IsFlipped` flags. Flipping an unknown card clears the undo files (117 clears across four logs), so peek-and-undo is impossible inside the game ([`CardController::get_ShouldClearSavesAfterFlip`][dll]; [LOG-A][logA]). The text log, however, names each fear card when it is earned (`CardBeingMoved=WaryOfTheInterior, To=Fear Earned`), and every save serializes the generator state ([LOG-B][logB]; [save][save]). That is why the spec drops RNG fields in the decoder and routes raw log events through a single firewall module ([spec §5][spec]).

Only a few decisions carry real uncertainty: growth, card plays, power targeting, and the choice of power deck. All of them are made before the explore card, event, fear card or power reveal is known. Cascade destinations, counterattack allocation, presence loss and action ordering are optimizations over a known state ([activation types and reveal points][dll]; inference from the state order).

These facts collapse most of the statistics. Take a stage of m cards with one removed unseen and the rest uniformly shuffled. It is equivalent to a single uniform permutation of all m cards. After j cards of that stage are seen:
- each unseen card is next with probability exactly 1/(m−j);
- the removal changes how many draws remain in the stage, not which card is next;
- once m−1 cards are seen, the removed card is known by elimination.

This is a symmetry derivation from the shuffle model ([Grinstead & Snell §3.3, §5.1][gs]; [Diaconis & Freedman 1980][df]). The belief state is then just the unseen multiset of each deck plus any known positions. The POMDP framing ([Kaelbling, Littman & Cassandra 1998][klc]) reduces to planning on that information state. The invaders are scripted and hold no private information, so Long et al.'s "non-locality" failure has nothing to arise from. That failure needs an opponent who steers play toward regions it privately knows are favorable. Strategy fusion appears only if a planner places chance nodes before decisions that precede a reveal ([Long et al. 2010][long]; [Cowling, Powley & Whitehouse 2012][cowling]).

Two teaching consequences follow:
- **"It's due" is partly correct for invader cards.** Cards within a stage are negatively dependent, so seeing one terrain lowers the chance of another of the same terrain. That is wrong reasoning for dice, and the contrast is a clean lesson that exchangeable is not independent. The Diaconis–Freedman error term scales with draws divided by deck size, so binomial approximations are badly off for 5- to 12-card stages ([Diaconis & Freedman][df]).
- **"Rules for and against the player" maps straight onto the display.** Face-up cards and every rule term are certainties (probability 0 or 1). Face-down cards are the only uncertainty. Every number can be drawn as a base value plus named modifier terms, each signed for or against the player. Examples are Sweden L3's Town damage of 3, a Badlands +1, and a Defend of 2.

## Lumia's ledgers and Rei's curve split "tempo" into two measurable things

### Land tempo is a per-land clock the companion can tag exactly

Lumia's blog is a puzzle-and-proof blog, not an economy blog. Its twelve theory posts contain:
- constructive turn-one wins with running fear and Energy ledgers;
- impossibility proofs by resource counting;
- a 68-spirit-or-aspect by 8-adversary feasibility sheet;
- one explicit card-sequence probability;
- one post on asymptotic growth classes.

The blog never states an Energy-per-card-play value ([si_theory index](https://iam.lumia.gay/si_theory.html); [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html); [cast down thresholded on turn 1](https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html)). The glossary supplies the state vocabulary, and its "tempo" is about lands, not engines: "where in the explore-build-ravage sequence your actions dealing with a land are happening. ahead tempo means they are happening between explore and build, on tempo means they are happening between build and ravage" ([glossary](https://iam.lumia.gay/si_glossary)). The project's [tempo.md][tempo] uses the same word for the other idea: "the rate at which you convert currencies into board effect per turn." That is Rei's sense. The companion should name the two separately: **land tempo** (per land, per turn) and **economy momentum** (per spirit, across turns).

Land tempo is exactly computable. Each invader card spends one turn in each slot ([invader-lifecycle.md][lifecycle]). The engine's state order fixes when spirits can act relative to each invader step: Spirit, Fast, Blighted Island, Event, Fear, High Immigration, Ravage, Build, Explore, Advance, Slow, Time Passes ([`Constants.StateOrdering`][dll]). My inference from that order:
- Fast powers act **on tempo** for lands in this turn's Build and Ravage slots.
- Slow powers resolve after the Explore and the card advance. They act **ahead of tempo** for lands that just received Explorers and will build next turn.

That gives the Fast-versus-Slow choice a precise meaning: Slow trades this turn's ravage for knowledge of where the explore landed.

The glossary supplies a ready set of per-land badges ([glossary](https://iam.lumia.gay/si_glossary)):
- ahead window (explored, not yet built);
- on window (built, ravaging next);
- b2b ("a land that has or is likely to have two consecutive invader actions occurring");
- capital ("a land that will have 7 buildings during the invader phase," which is England's loss line);
- threat, problem land, solve, take blight to scale, and pocket.

Face-up cards make most badges certain. The only probabilistic input is the next explore. That is uniform over the unseen cards of a known stage, multiplied by a deterministic source test: the land is not isolated, and it holds or neighbors a Town, City or Ocean ([`GameController::GetLandsToExploreFrom`][dll]; [Sequence of Play §3c, wiki copy](https://spiritislandwiki.com/index.php?title=Sequence_of_Play)).

### Economy momentum is a deterministic projection against a step-function curve

Rei supplies the valuation side. Lumia's glossary credits it explicitly: "algo: (rei's understanding of) the algorithm that the developers use to balance growth options, presence tracks, and power cards by assigning values to effects, like how chess assigns values to pieces" ([glossary](https://iam.lumia.gay/si_glossary)). Rei's Minor value chart assumes "card play is worth ~2.5e," derived from an algorithm that "in its entirety hasn't been revealed" ([Aqualicy/Rei, BGG 3569628](https://boardgamegeek.com/thread/3569628); [Vz-Rei, Reddit](https://www.reddit.com/r/spiritisland/comments/1n6804w/)). That number is contested in the same thread:
- Designer darker questioned valuing gifted Energy above its face value.
- Maximilian_Gerhards showed the formula gives a 0-cost card a 2-Energy edge over an identical 1-cost card, when by definition the edge should be 1.
- Tongar1234 noted that the opportunity cost of a card play changes with game size.

No other numeric exchange rate exists. The older consensus was "a spectrum of when and how often you go for major powers," not a price ([BGG 2497103](https://boardgamegeek.com/thread/2497103)). The companion should therefore show momentum in natural units: Energy per turn, card plays per turn, presence on the island, and element coverage. These come exactly from the engine's presence-track graph (`PresenceTrackNode._nextNodes`, 22 `TrackNodeType` values, `GetSpiritEnergyPerTurn`, `GetSpiritCardPlaysPerTurn`) for each growth line ([dll]). Any single valuation should be a user-set parameter with a sensitivity band. Lumia's natural-unit benchmarks fit this style: "3/3 is a good place to get to for major-slinging strategies," and her presence-per-growth notation ("earth is a 1/1/1 spirit, river is a 0/2/1 spirit") ([glossary](https://iam.lumia.gay/si_glossary)).

The curve those numbers race against is a step function. Darker described the stage step-up as coming "at a constant point in time," so the game does not self-balance, with Stage I, II and III cards hitting roughly 2, 3 and 4 lands per board ([BGG 1823719](https://boardgamegeek.com/thread/1823719)). A balanced board has two lands of each terrain ([rulebook transcription][rulespal]). So a single-terrain card matches 2B lands, Coastal Lands 3B, and a two-terrain Stage III card 4B, where B is the number of boards (derivation). Lumia's growth post sets aside "the ramping from invader stages" as the main invader-side nonlinearity. It argues that most spirit-side quantities grow linearly unless "cards gained let you gain more cards" ([growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)).

Combining the two, as my own synthesis rather than either author's, "staying ahead of the invader curve" becomes a chart drawn without any outcome data:
- one line: the spirit's projected action economy by turn, for each growth line;
- the other line: expected lands hit by explore, build and ravage per turn, derived from card backs, the adversary's deck recipe and exchangeable terrains.

The step lands on a different turn for each adversary level, so the chart itself teaches the matchup:
- Brandenburg-Prussia L2 moves a Stage III card into position 4.
- Sweden L4 accelerates the deck after setup.
- Russia L4 alternates stages.
- Habsburg Livestock L5 hides a reminder card that spikes around turn 4 (inferred from its position under the top five cards).
- England L3 builds twice from Stage I cards.

Sources: [Brandenburg-Prussia][w-bp], [Sweden][w-swe], [Russia][w-rus], [Habsburg Monarchy][w-hlc], [England][w-eng]; engine `AdversaryList` ([dll]).

### Ledgers, growth classes and timelines give every number a budget

Lumia's method supplies the numbers on both axes. Her posts follow the same steps each time:
1. State a hard budget.
2. Inventory each land's contribution.
3. Check resolution order.
4. Report the shortfall.

Examples: "destroying all those buildings would only be 9 fear in total… prussia 4 or higher would need 16 fear generated"; "(16 fear in total, hitting tl2)"; "this is 1 fear off from getting another fear card" ([East board](https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html); [T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html); [low-level T1](https://iam.lumia.gay/si_theory/2023/11/18/low_level_t1_victories.html)).

The engine computes the same budgets directly:
- **Fear to the next Terror Level** = (cards above the next divider) × (fear per card) − (fear already generated). Overflow carries from card to card ([`GetFearRequiredForNextTerrorLevel`, `GenerateFear`][dll]).
- **Victory condition:** no Explorers, Towns or Cities at Terror Level I; no Towns or Cities at II; no Cities at III ([`TerrorLevelCardList`][dll]).
- **Blight budget:** 2B+1 on the healthy side ([`CreateBlightCard`][dll]; [NI rulebook p.5][ni]).
- **One land's ravage:** with invader damage D, Defend d and land health h (2 plus modifiers, floor 1), blight is added if and only if D − d ≥ h. The Defend that "solves" the land is therefore D − h + 1. At h = 2 that is the community's "Defend needed = total damage − 1" ([BGG 3032282](https://boardgamegeek.com/thread/3032282); derivation from [`Land::GetCurrentMaxHealth`][dll]).

The ledger style checks itself. I summed the per-level fear-deck deltas under the combined-adversary rule ([JE rulebook p.19][je]), and the result reproduces every combined total lumia published:
- Scotland 6 + Sweden 6: 7/7/6 = 20 cards, at 4 fear each = 80 fear.
- England 6 + Habsburg Livestock 6: 6/8/4 = 18 cards. At 5 fear per card = 90.
- All eight adversaries at L6: 15/19/12 = 46 cards.

My check used the per-level tables ([adversary pages][w-adv]); lumia's totals are in [T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html). The same arithmetic closes two gaps in the research notes. First, lumia's switch from 4 to 5 fear per card in England games is England L6's `extraFearPerPlayer: 1` ([engine `AdversaryList`][dll]). Second, her "saltsburg" is Habsburg Mining Expedition (my reading). She says its ravages "upgrade instead of cascade" and that at L6 its "terror 1 has only 4 cards." That matches Habsburg Mining's L1 rule Avarice Rewarded, which turns ravage cascades into upgrades, and its 4/5/4 L6 fear deck ([NI p.12][ni]; [wiki][w-hme]). Her "cowsburg" is Habsburg Livestock Colony.

Her growth taxonomy works as labels on effects, not as live numbers ([growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)):
- escalating repeat costs total n(n−1)/2 for Fractured Days' Slip the Flow of Time and Relentless Gaze's Relentless Punishment;
- range coverage grows roughly quadratically with range;
- minimum blight cascades grow as n^1.5;
- Lure of the Deep Wilderness's 4-Moon gather adds up to 1/6 of the land's population per turn (a factor of 7/6);
- Melt Earth chains follow meme(n) = 1 + ⌈n/2⌉·meme(n−1), which is factorial.

The companion should compute the finite-horizon versions and keep the class only as a "linear or ramping" label:
- exact range sets by breadth-first search over the engine's `LandAdjacencyMatrix` and `LandDistanceCache`;
- the repeat-cost ledger;
- the loaded-cluster cascade reach.

Her darkest and best timelines are the bounds. The worst unseen next invader card is an exact enumeration over at most six cards. Her spirit-free wins, built by choosing deck orders, show how wide luck's range is ([no spirits](https://iam.lumia.gay/si_theory/2023/12/29/spiritless_island.html); [level 6s with no spirits](https://iam.lumia.gay/si_theory/2024/01/10/spiritless_island_2.html)). She also reports that Spirit Island is NP-hard, though the proof is unpublished. That is a first-principles reason for the companion to show budgets and consequences rather than solve turns ([October update](https://iam.lumia.gay/2023/10/30/october_update.html)).

## Probability earns a panel only where a hidden card meets a land

The model catalog below ranks each candidate by how well it fits the two structural facts above. Hidden information is exchangeable deck order only, and resolution is deterministic given the cards. Fit ratings follow the statistics review, where 5 means "build first" and 1 means "metaphor only." Every model is conditional on the current state. None takes aggregate win/loss counts as input.

| Layer | Model | Tempo or momentum question | Exact or simulated | Fit | Spec home |
|---|---|---|---|---|---|
| Ledger | Deterministic rule resolution with named, signed modifier terms | Does this land blight; what solves it; fear to next card; what blocks victory | Exact | 5 | §7.1–7.3 |
| Counting | Hypergeometric, multivariate and negative hypergeometric, on the exchangeability lemma | Next explore terrain; P(element E in a draw of 4); turns until terrain T | Exact, closed form | 5 | §7.1, §7.4 |
| Land tempo | Per-land threat map: deterministic invader steps run for each possible next card | Which lands enter ahead or on windows in 1–2 turns; b2b; capital | Exact enumeration | 5 | §7.1 (extend) |
| Timelines | Stratified "if X is next" table (conditioning, Rao–Blackwell) | Darkest and best timeline per next card | Exact or hybrid | 5 | New |
| Momentum | Presence-track graph projection per growth line | Energy/plays/presence by turn against the pressure step | Exact | 4 | §7.4 (extend) |
| Race | Competing-risks cumulative incidence from forward rollouts | Which clock binds, and by when | Simulated | 4 | §7.2 (replace projection) |
| Comparison | Common-random-number Monte Carlo with Wilson intervals | Is line A better than line B, and in how many futures | Simulated | 5 | Later |
| Planning | Depth-1 expectimax, chance nodes at reveal time | Which line keeps an answer for every card | Search + simulation | 4 | Later |
| Bound | Determinization / hindsight optimization | Best case with perfect foresight | Simulated | 3 | Later |
| Learning | Proper scoring against the exact p* | How good is my count | Exact | 5 | Optional mode |

### Counting and enumeration cover most questions

Card questions are exact and cost microseconds. P(X ≥ 1) = 1 − C(N−K, n)/C(N, n) answers "at least one of K matching cards in n draws from N." The negative hypergeometric gives the expected position of the first match as (N+1)/(K+1) ([Grinstead & Snell][gs]). Community figures reproduce cleanly under this model:
- P(at least one card with a given element in a 4-card Minor draw) = 1 − C(60,4)/C(100,4) = 87.56% ([BGG 2605598](https://boardgamegeek.com/thread/2605598)).
- Miss streaks for one element: 12.4%, 1.37% and 0.13% over one, two and three Minor gains ([BGG 2603885](https://boardgamegeek.com/thread/2603885)).

The power-deck model must follow three engine rules. A gain reveals 4 cards and keeps 1, and the other three go to the discard where everyone can see them ([rulebook transcription][rulespal]; [`GainPowerCardJournalEntry.PowerCardsNotGained`][dll]). When the deck runs short, the discard is reshuffled in, which returns those cards to the unseen pool ([`ReshufflePowerDeck`][dll]). Overlapping categories ("Fire" and "Plant") are not a partition, so the complement event must be computed directly.

The per-land threat map is the most useful spatial display. For each possible next invader card, run the deterministic explore, build and ravage steps on the current board and weight the result by the card's probability. One card ahead means at most six cases. Two cards ahead within a stage means |U|(|U|−1) cases. Face-up cards contribute certainty, so the map must separate "certain" from "possible" visually.

Network-epidemic, percolation and sandpile theory supply vocabulary ("sources," "frontier," "loaded cluster"). Their thresholds are infinite-graph limits that do not transfer to a few dozen deterministic nodes ([Pastor-Satorras et al. 2015](https://arxiv.org/abs/1408.2701)). The stratified table, "if Jungle comes next, these three lands build; if Wetland, two," is the darkest-timeline view and the most instructive single display. Proportional stratification never increases variance, and exact substitution of closed-form sub-probabilities lowers it further ([Owen, *Monte Carlo Theory, Methods and Examples*, ch. 8][owen]).

### Simulation is for races and comparisons, and its estimand must be labeled

Multi-turn races need forward simulation once rules depend on board state. The correct display is a set of cumulative incidence curves, one per outcome: win, blight loss, deck exhaustion, and each adversary's loss condition. Two clocks need special treatment:
- **The deck clock is deterministic.** Base games lose at the Explore of turn 12. It acts like administrative censoring at a known time, not a random hitting time.
- **Clocks must not be computed in isolation.** Computing each loss clock as if the others did not exist gives the "latent failure time" quantity. It overstates every risk, and the naive per-cause numbers can sum past 100% ([Putter, Fiocco & Geskus 2007][putter]).

The discrete-time formulas are simple: CIF_j(t) = Σ_{u≤t} h_j(u)·S(u−1), estimated as the share of rollouts ending by cause j by round t. Per-round hazards answer the live question "is this round the dangerous one?" The spec's current §7.2 plan projects each clock at a rolling rate. That is a first-passage approximation whose increments are neither identically distributed nor independent of the player's policy. It is fine as a quick heuristic and wrong as the race chart ([spec §7.2][spec]).

Monte Carlo estimates come with fixed rules:
- **Intervals:** use Wilson intervals, because Wald coverage is "chaotic" ([Brown, Cai & DasGupta 2001][bcd]).
- **Sample sizes:** about 2,401 rollouts for ±2 percentage points at 95% in the worst case, and about 385 for ±5.
- **Comparisons:** use common random numbers. Paired comparisons shrink the standard error by √(1−ρ) and allow a display like "A is better in 31% of futures, worse in 8%, the same in 61%."
- **Stability:** fix seeds per game state so the same state always shows the same number, and round to the displayed precision ([Owen][owen]).

The estimand is always P(outcome | state, rollout policy). That is a lower bound on what good play achieves. Hindsight optimization, which plans perfectly inside each sampled future, gives an upper bound because of strategy fusion. Together they bracket the true value: V^rollout ≤ V* ≤ V^hindsight ([Yoon et al. 2008][yoon]; [Long et al.][long]). Bjarnason, Fern and Tadepalli built on exactly this framing in Klondike. Their Sparse UCT won about 35% of games and treated face-down cards "as chance events at the moment they are revealed," which works when hidden information has no effect until revealed ([as reported in Cowling et al.][cowling]).

Spirit Island meets that condition, so depth-1 expectimax with chance nodes at reveal time is the best-founded planner. Its output fits a "decision fan": "after A, every next card leaves you an answer; B is excellent against Jungle and bad against Wetland." Information-set MCTS, POMCP and rolling-horizon evolution (the Pandemic precedent) are correct but heavier than the structure needs. Their outputs are also opaque to a learner ([Silver & Veness 2010](https://neurips.cc/virtual/2010/poster/2145); [Sfikas & Liapis 2021][sfikas]). Gambler's ruin and branching-process growth rates stay analogies, because Spirit Island's blight drain is lumpy and two-dimensional and invader growth is per-land and card-triggered ([Grinstead & Snell][gs]).

There is a practical sequencing point. Rollouts need a full Rust rules engine across all content, and that is the real cost driver. A v1 can still draw an honest race from exact pieces:
- the deterministic deck clock;
- an invaders-only projection, enumerated exactly over the next few cards (the "if you do nothing" pressure);
- the fear runway from guaranteed sources plus the land inventory.

Policy rollouts come later and should be labeled with their policy.

### Displays that build judgment instead of dependence

The learning research points one way:
- **Natural frequencies.** People reasoned correctly three times as often when given natural frequencies instead of probabilities (16% to 46%) ([Hoffrage et al. 2015, on Gigerenzer & Hoffrage](https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2015.00642/full)). The display should say "2 of the 3 unseen Stage II cards explore here," not "67%."
- **Distributions.** Quantile dotplots got decision-makers to 97% of optimal payoff, while text uncertainty underperformed ([Fernandes et al. 2018](https://mucollective.northwestern.edu/project/uncertainty-bus)). "Usually 1 blight; in 1 of 8 futures, 3 with a cascade" belongs in a 20-dot strip.
- **Deltas, not verdicts.** Chess coaches report that students who chase the evaluation bar "learn moves without learning ideas" ([Randjelovic, Chess.com](https://www.chess.com/blog/mizant/when-the-engine-makes-you-worse-at-chess)). PokerStars banned situation-specific real-time analyzers while allowing counted statistics ([PokerNews 2015](https://www.pokernews.com/news/2015/10/pokerstars-announces-changes-to-third-party-tools-policy-22920.htm)). The companion should show "what changes if you hit land X," never "play this."
- **Pocket rules.** Pair each exact number with a rule the player can compute at the table, in the style of Karsten's mana tables, which he calls "rules of thumb" ([Karsten 2018, archived](https://web.archive.org/web/20181011120856id_/http://www.channelfireball.com/articles/how-many-colored-mana-sources-do-you-need-to-consistently-cast-your-spells-a-guilds-of-ravnica-update)). The Spirit Island version is "1 ÷ unseen cards left in this stage."
- **An optional "hold" toggle.** Cognitive-forcing designs that make people decide before seeing the aid cut overreliance, but users rated them least favorably ([Buçinca, Malaya & Gajos 2021](https://arxiv.org/abs/2102.09692)). Failed attempts before seeing the answer improve learning ([Kornell, Hays & Bjork 2009](https://pubmed.ncbi.nlm.nih.gov/19586265/)). An always-on second-monitor aid is the textbook setting for automation complacency ([Parasuraman & Manzey 2010](https://journals.sagepub.com/doi/10.1177/0018720810376055)). So the spec's rejection of predict-then-reveal as the v1 default should keep an optional, off-by-default toggle that blurs a panel until a guess is committed. Each guess q can then be scored against the exact p* by its expected Brier excess, (q − p*)², which gives feedback with no outcome noise ([Gneiting & Raftery 2007][gr]).
- **Fading.** Per-stat explanations should collapse once predictions are reliable, because full worked solutions stop helping as expertise grows ([expertise-reversal literature](https://researchportal.hw.ac.uk/en/publications/the-expertise-reversal-effect-and-worked-examples-in-tutored-prob/)).

Fairness precedents support the firewall but are weaker than it. The accepted line in other games is the "pencil and paper" test ([HDT FAQ](https://github.com/HearthSim/Hearthstone-Deck-Tracker/wiki/FAQ)). Pandemic's rulebook opens the discard piles "because Pandemic is a test of cooperation and mettle (and not of memory)" ([Pandemic rules](https://aadl.org/files/catalog_guides/Pandemic_Rules.pdf)). The spec's permutation-invariance test is stricter than any precedent found. It permutes hidden identities within look-alike groups and requires byte-identical output ([spec §5][spec]). It has one corollary the spec should state outright: **every derived statistic, including Monte Carlo rollouts, must sample hidden orders from the public belief, never from the save.** A rollout seeded from the save's actual deck order would leak through the numbers even though no card name appears. The invariance test catches this only if rollout output is in the fixtures.

## Adversaries, expansions and player count rewrite recipes and clocks, not the math

### Every adversary level is a deterministic plug-in plus a deck recipe

All eight adversaries are deterministic rule changes. Randomness enters only through four channels:
- the deck recipe (how many unseen cards are removed per stage, and which cards move to known positions);
- Russia L5 hiding invader cards inside the fear deck;
- combined-adversary terrain coin flips;
- player tie-breaks, which are choices and so need candidate sets in the forecast.

Within each stage the unseen cards stay exchangeable, so next-card odds remain a ratio of counts once a recipe interpreter has run. One-step explore, build and ravage forecasts stay exact. Multi-step projection needs simulation wherever a rule depends on board state: England L1/L6, Scotland L3/L6, Russia L3/L6, both Habsburgs, Sweden L5, France L2/L5, and every extra loss condition ([adversary tables, wiki snapshots][w-adv]; [engine `AdversaryList`][dll]).

| Adversary | Deck recipe by level | Fear deck L0→L6 | Extra loss clock | Rules that need simulation past one step | Signed for/against at the table |
|---|---|---|---|---|---|
| Brandenburg-Prussia | L2 111-3-2222-3333; L3 11 cards; L4 10; L5 9; L6 3-222-3333 (8, setup explore is Stage III) | 9 → 12 (4/4/4) | None; deck clock ends at the turn-8 Explore at L6 | Land Rush ties | Against: early Stage III, shorter deck. For: no extra loss clock |
| England | No deck change; L3 adds a High Immigration slot (Stage I cards), L4 keeps it | 9 → 13 (4/5/4) | 7+ Town/City in one land | L1 widens build targets; L6 doubles High Immigration when no fear card resolves | Against: capital badge, L5 +1 building health, L6 fear per card 5N |
| Sweden | L4 accelerates one card after setup | 9 → 13 (4/4/5) | None | L5 Mining Rush adds Towns after blighting ravages | Against: L1 4+ overkill adds a second blight; L3 Town 3 / City 5 damage |
| France | No deck change | 9 → 14 (4/5/5) | Town supply cap (7 per player on the wiki, per board in the engine) | L2 Slave Labor, L5 Slow-Healing Ecosystem | Against: L6 fear cards push Explorers instead of removing them |
| Habsburg Livestock | L3 11-2222-33333; L5 reminder under the top 5 cards | 9 → 14 (5/6/3) | More than N blight from 6+ overkill ravages | L1 gathers Towns; L4 Durable Towns (+2 health) | Against: L6 +2 ravage damage next to a Town; known spike near turn 4 |
| Habsburg Mining | L4 removes Coastal Lands, puts Salt Deposits at position 5 | 9 → 13 (4/5/4) | 8+ invaders plus blight in a land at end of Fast | L1 Mining lands turn Builds into Ravages | Against: permanent Salt ravage. For: Coastal never comes, cascades become upgrades (more fear) |
| Russia | L4 111-2-3-2-3-2-3-2-33; L5 unused invader cards under fear cards 3 and 7 | 9 → 14 (5/5/4) | Beasts on the panel outnumber the island's | L3 ravages lands with 3+ Explorers; L6 punishes ravage prevention | For: L2 +1 fear per Explorer-destroying action. Against: L1 Explorers deal 2 |
| Scotland | L2 11-22-1-C2-33333; L4 11-22-3-C2-3333 | 9 → 16 (6/6/4) | Coastal Cities > 2 × boards | L3 coastal builds next to Cities; L6 inland Towns after ravage | For: Coastal card at a known position. Against: escalation adds Towns in N lands |

Deck length sets the deck clock. With D cards after setup changes, setup reveals card 1 and turn t reveals card t+1, so the loss arrives at turn D's Explore (inference from the Sequence of Play). Base D is 12. Prussia L3–L6 give 11, 10, 9 and 8; Habsburg Livestock L3+ and Scotland L4+ give 11; Sweden L4+ is effectively 11.

Recipes also change what is knowable. Habsburg Mining L4 and Scotland L2 remove randomness by putting a card at a known position. Prussia L3–L6 and Habsburg Livestock L3 add randomness, since more cards are removed unseen and a terrain may never appear. Russia L5 turns the normally never-seen removed card into one that will surface. Event branches read the invader stage from the top card of the deck ([JE p.12][je]), so recipes also change which event text fires.

Digital win-rate data shows its largest per-level drops at rules that remove or accelerate Stage I, raise building health, or come from Russia's L3 and L5 rules. Only that mechanism-level pattern is used here, as a guide to where warnings belong. The rates themselves are the aggregate counting this report excludes ([Jonathan4290 series, e.g. #7](https://boardgamegeek.com/thread/3636188)).

### Combined adversaries add a second escalation clock and a terrain coin

Jagged Earth p.19 combines two adversaries at independent levels and applies "all game changes from both," with these exceptions ([JE p.19][je]):
- **Fear deck:** each Terror Level gets 3 cards plus both adversaries' deltas.
- **Escalation:** the leading adversary escalates on Stage II as normal. The supporting adversary "applies its Escalation Effect on all Stage III Invader Cards." For France and Sweden, the terrain is chosen at random between the card's two terrains.
- **Deck instructions:** the supporting adversary's run first, then the leading adversary's, with tie-break rules: remove the top-most applicable card, move the bottom-most.
- **Worked example:** Prussia L4 leading with Russia L4 supporting gives 11-33-2-3-2-3-2-3, a 10-card deck.
- **Difficulty:** the rulebook gives "the higher of the two Difficulties plus 50-75% of the lower." The engine's `CombinedAdversaryDifficultyFactor` is 0.625, the midpoint of that band ([`GetDifficultyRating`][dll]).
- **Loss conditions:** both adversaries' apply. They are checked only once no triggered actions remain, per Nature Incarnate's errata ([NI p.13][ni]). That matches lumia's "losses are checked at the end of action trees" ([T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)).

The FAQ adds specific pair rulings. For example, Habsburg Mining L4 with Scotland L2 yields 11-2S-1-C2-33333 whichever adversary leads ([querki FAQ, Combining Adversaries][q-ca]). Experienced players report that the formula understates some pairs. Steve496 found double-level-3 pairs "generally harder than single level 6 adversaries (England 3 / Habsburg 3 being notably brutal)" ([BGG 3057568](https://boardgamegeek.com/thread/3057568)). So difficulty should be shown as a band, never as a probability.

What the models must add:
- an executable recipe interpreter, applied supporting-first;
- a two-branch mixture for each supporting France or Sweden escalation;
- up to two adversary loss clocks in the race.

The engine IL did not show how two adversaries' fear modifiers combine, and no multi-adversary game is in the logs, so this path needs a live check.

### Expansions add a fourth hidden deck, token modifiers and a mixture blight clock

**Branch & Claw** adds the Event deck. An event is drawn each Invader Phase after Blighted Island and before Fear, which matches the engine's state order ([JE p.12][je]; [`Constants.StateOrdering`][dll]). On Turn 1 the Event is drawn and discarded without resolving. The user has confirmed this against the physical copy. The card is public in the Event discard and is known to be out of the deck, so the unseen Event pool shrinks by one on Turn 1. The engine does the same, moving an Event to the discard in round 1 (`SeekingTheInterior` observed) ([spec §9][spec]). The project's earlier belief, a face-up preview that resolves on Turn 2, was wrong, and so is the tabletop si-live `EventDeckState.previewed` model built on it.

Events branch on public state. Of the engine's 62 events, 25 branch on Healthy/Blighted, 15 are choices, 12 branch on Terror Level and 10 on invader stage. So P(the next event's effective text) is closed-form over the unseen cards. The engine never reshuffles the Event deck ("The Event Deck is empty so an Event cannot be resolved!"), so draws are exchangeable without replacement, not independent ([`EventStateController`][dll]).

Branch & Claw tokens enter the ravage arithmetic in the Jagged Earth p.7 order ([JE p.7–8][je]):
- Wilds cancels an Explore.
- Disease cancels a Build.
- Strife zeroes an invader's damage.
- Badlands adds 1 damage per token.

Designer darker says Events were meant to be "on-average slightly negative," to offset the starting Disease and Beasts ([BGG 2506650](https://boardgamegeek.com/thread/2506650)).

**Jagged Earth** adds Still Healthy blight cards, which make the blight clock a two-stage mixture. It also adds the no-Events option. The engine confirms that option adds a Terror Level II fear card, inserts Command Beasts reminder cards, and removes immediate-effect or 2-per-board blight cards ([`CreateFearDeck`, `CreateInvaderDeck`, `CreateBlightCard`][dll]).

**Nature Incarnate** adds four things:
- **The +1 blight errata:** 2 per player + 1 at setup only. Darker explained it as a fencepost fix ([NI p.5][ni]; [BGG 2578232](https://boardgamegeek.com/thread/2578232)).
- **Vitality,** which blocks blight.
- **Habsburg Mining Expedition.**
- **Retirements** of five cards.

The flipped side of the blight card is the main new random variable. Its blight count X per player is drawn from the unseen blight cards. With a full collection and no exclusions, E[X] = 73/23 ≈ 3.17 on X ∈ {1,…,5}. The solo redraw rule for 2-per-player cards changes that distribution at N = 1 ([wiki Blight Card][w-blc]; [NI p.14][ni]).

Pool sizes differ by source. A full collection holds 100 Minors, 78 Majors, 50 fear cards, 62 events and 23 blight cards per the wiki. The engine lists 101 Minors, splitting Branch & Claw 32 against the wiki's 31 active ([wiki snapshots][w-adv]; [engine `PowerCardList`][dll]). For Steam odds the companion should read the save's `Allowed*` lists, because those are the pools the Steam game samples from. For chapter text, the physical copy rules. Any Steam-versus-box difference is itself a finding to log. Community odds tables from before Nature Incarnate need recomputation, although past about 100 Minors, growing the deck barely changes draw variance ([BGG 2903594](https://boardgamegeek.com/thread/2903594)).

### One, two or N spirits scale pools by players and exposure by boards

Most scaling is linear in N, and it must be kept separate from scaling in B:
- **Fear per card:** 4 × spirits. The engine raises the per-spirit figure to 5 when boards = spirits + 1, and adds England L6's +1 per player on top ([`Game::CreateFearValues`][dll]).
- **Blight before the flip:** 2B+1, with B the number of boards.
- **Invader deck:** fixed at 12 cards whatever N is, so the deck clock is identical at every player count ([JE p.18][je]; [rulebook transcription][rulespal]).

The fraction of the island each card hits is the same at every N, so per-land probabilities do not change with player count while island totals scale with B. The companion should display per-land and per-board figures so this invariance is visible. Variance is the real difference. A bad card or event "in solo… is a 100% bad thing, in a game with N it might be a 1/N bad thing" ([biophys, BGG 2827046](https://boardgamegeek.com/thread/2827046)). Fear is a team-wide step function: "Generating 1/6 of a fear card in a 6 spirit game has no impact" ([Arctem, BGG 2852467](https://boardgamegeek.com/thread/2852467)). So the fear runway should be shown both in cards and in each spirit's share.

Multi-handed solo is mechanically an N-player game: "The game mechanics scale with the number of Players, not the number of people" ([querki FAQ, Core][q-core]). The engine's `IsGameMultiplayer` flag changes only prompts and action ordering ([dll]). The extra-board option is the only official way to run one spirit on two boards. It gives fear 5 per card, blight 2·2+1, an extra board whose setup depends on N, and +2 to +4 difficulty ([JE p.18][je]).

Rules scale in two different units. Scotland's escalation (N lands), Habsburg Livestock's loss (more than N), France's Town cap (7N on the wiki, 7B in the engine) and England L6 scale by players. Scotland's loss (2B) and most setup and escalation effects scale by boards. The companion must store N and B separately and tag each rule's unit. Designer darker separates "hypothetical" difficulty, which falls with player count, from "practical" difficulty, which he says is roughly flat across 1–6 spirits ([BGG 3371600](https://boardgamegeek.com/thread/3371600)). That claim is not a probability the companion should compute.

Scenarios swap clocks rather than models:
- Rituals of Terror, Ward the Shores and Dahan Insurrection replace the fear-victory predicate.
- Surges of Colonization reveals several cards per turn, which a multivariate hypergeometric still covers.
- Despicable Theft accelerates the deck by hidden Thief numbers, which needs simulation.
- Destiny Unfolds replaces draws with known draft pools.

The engine's scenario list has 13 entries and does not include Surges or Destiny ([wiki Surges][w-soc]; [engine `ScenarioList`][dll]).

## The v1 spec gets the firewall right and the race chart wrong

**Section 5 is sound.** Beyond the rollout corollary above, it needs three additions:
- A check of whether the Steam UI shows earned fear identities before resolution. The † row in the visibility table already marks this. The log does name them ([LOG-B][logB]).
- A note that the serialized generator state would let anyone predict future reshuffles and France or Sweden escalation picks. That is a second reason to drop it, beyond deck order.
- Visibility rules for Russia L5's invader cards inside the fear deck. Their positions are public setup facts; their identities are hidden.

**Section 7.1** needs two changes:
- Replace `data/adversary-stacks.json` with a verified recipe interpreter. That file keys levels by difficulty value rather than level 0–6, and most of its stacks are wrong (Appendix B).
- Add land-tempo badges and the darkest-timeline card. Per-land explore odds must include the source predicate plus Wilds and Isolate.

**Section 7.2** needs three changes:
- Model fear by dividers: cards above the next divider × (4N, or 5N for England L6 or an extra board) − generated. Cards earned in one window all resolve at the new Terror Level ([dthurston, BGG 2611841](https://boardgamegeek.com/thread/2611841)).
- Replace the rolling-rate projections with the exact deck clock plus cumulative incidence.
- Model the adversary loss clocks. Six of the eight adversaries have one, and the spec currently only warns about them ([spec §7.2][spec]).

**Section 7.3** needs the post-flip X mixture, B rather than N, loaded-cluster cascade reach, and the blight rules of Sweden L1, Scotland L5, France L5 and Habsburg Mining L1. Adding blight destroys one presence of each spirit in the land ([`AddBlightAction.AfterAction`, `HandleBlightDestroyingPresence`][dll]; [rulebook transcription][rulespal]).

**Section 7.4** needs four changes:
- Grow from "energy and card plays per round (history)" into the presence-track projection per growth line, so momentum is forward-looking.
- Implement draw-4-discard-3.
- Track reshuffles.
- Make the threshold check phase-aware: Slow-card elements count toward Fast innates, because "Power Card elements count from the moment the card is paid for" ([rulebook transcription][rulespal]).

The fundamentals chapters show the same failure pattern. They state rules from memory, and if `rules.rs` were seeded from them, the companion would print wrong numbers:
- [invader-lifecycle.md][lifecycle] inverts the Build rule.
- [presence-economy.md][presence] reverses the cause and effect between presence loss and blight.
- [fear-track.md][fear] treats each earned fear card as a Terror Level change.

The memory rule ("never hallucinate mechanics") applies directly. Every rule in `rules.rs` should cite the engine method, a rulebook page and the physical copy, and the engine's preview functions should serve as test oracles. The 45 `EndGame` journals offer validation that involves no win/loss counting. Their `ExploreJournalEntry`, `RavageCompleteJournalEntry` and `AddBlightJournalEntry` rows let each per-land forecast be scored against what happened, using a proper score ([journal entry types][dll]).

## Conclusion

The research changes where the difficulty sits. For this game the statistics are mostly counting, and counting is easy to get right. The hard, error-prone part is rules fidelity. Every quantitative error found in this pass was a misstated rule, not a misapplied distribution: an inverted Build rule, fear modeled as Terror Level jumps, presence treated as a blight shield, a power-gain discard sent to the bottom of the deck, adversary stacks keyed by difficulty. The Steam engine turns out to be a better rules reference than the project's wiki scrape. It even encodes the rulebook's heuristics, such as the 0.625 midpoint of the combined-difficulty band and the thematic and extra-board difficulty modifiers. The physical copy still arbitrates the tabletop chapters, and the Turn-1 Event ruling shows the process working: the box settled a belief the project had carried as fact.

The tempo-and-momentum frame also gives probability a job. A number earns screen space only when it attaches to a land window (ahead, on, b2b, capital) or to a named clock (fear runway, blight runway, deck, adversary loss). Lumia's ledgers say what the budget is. Rei's curve says whether the engine outruns the invader step. The hypergeometric and enumeration layer says which card moves which land into which window. Shown as natural-frequency counts beside a pocket rule, and never as a ranked verdict, that combination teaches the counting habit both authors share, without the evaluation-bar dependence that engine-style advisors produce.

## Appendix A: questions and open modeling problems for the statistician

| # | Question | Why it matters to the companion | What exists now | Open issue or suggested approach |
|---|---|---|---|---|
| 1 | Does within-stage exchangeability survive every deck-recipe operation? The operations are: remove unseen, move by position, replace, accelerate, Russia L5's "unused" insertion, and combined supporting-then-leading ordering. | All explore odds rest on "uniform over unseen cards of a known stage." | A symmetry lemma for one removal per stage ([Diaconis & Freedman][df]); recipe strings in the adversary tables | Characterize which operations preserve exchangeability given public card backs. Whether Russia L5 chooses randomly among several unused cards is unknown. Also check that the engine's `Next(int)` modulo bias and clock-seed truncation are negligible ([`SerializableRandom`][ecdll]). |
| 2 | Exact multi-turn per-land threat across a stage boundary, when explore sources change after builds and adversary post-step rules fire | The land-tempo badge for turn t+2 | Exact enumeration for 1–2 cards | Find a closed form or a cheap enumeration under an "invaders only" policy. Where is the break-even point for switching to simulation? |
| 3 | Which estimand should the race chart show? | §7.2 replacement | Cumulative incidence from rollouts ([Putter et al.][putter]) | Choice of rollout policy; how to display policy dependence; whether "cause-deleted" counterfactual clocks are safe to show a learner |
| 4 | How tight is the hindsight upper bound? | Bracketing V^rollout ≤ V* ≤ V^hindsight | Determinization theory ([Yoon et al.][yoon]; [Long et al.][long]) | Information-relaxation penalties (Brown, Smith & Sun 2010; citation unverified) to tighten the bound; how large the gap is empirically |
| 5 | A fear-runway band from 2–8 rounds of history | §7.2 "projected round for each terror level" | Exact thresholds; first-passage and Wald approximations | Increments rise with time and depend on policy. Compare a renewal band with "guaranteed fear plus land inventory." Treat small samples honestly. |
| 6 | Turns to blight loss after the flip | §7.3 | X mixture, E[X] = 73/23 for a full pool; solo redraw rule; Still Healthy second draw | A compound process with cascades whose destinations the player chooses. Model it as a mixture over X joined to a graph optimization, not as a random walk. |
| 7 | The shortest cascade chain the player can force | Cascade badge | Loaded-cluster bound by breadth-first search | An algorithm, with proof, for the minimum chain length when the player picks destinations. Is the cluster bound tight? |
| 8 | An implicit price list for effects ("the algo") | Momentum valuation | Rei's ~2.5E per play, disputed in [BGG 3569628](https://boardgamegeek.com/thread/3569628) | Hedonic regression of printed costs on effects from the engine's card definitions. This infers designer prices, not win rates. Estimate the time discount for gifted Energy and test Gerhards' 0-cost inconsistency. |
| 9 | A scalar for "ahead of the curve" that does not collapse into a win rate | The momentum dashboard | Natural-unit curves against expected lands hit per turn | Candidates: lands solvable per turn minus lands threatened per turn, or a ratio. Test stability on logged states. |
| 10 | Finite-horizon breakpoints for lumia's growth classes | Linear-versus-ramping labels | Asymptotic classes ([growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)) | When do quadratic or factorial effects actually overtake linear ones within 12 turns, given real constants? |
| 11 | P(reach an innate's element threshold within k gains, choosing 1 of 4 each time) | §7.4 draft odds | One draw at a time is exact; Starlight has only lower bounds ([BGG 2605598](https://boardgamegeek.com/thread/2605598)) | Exact dynamic programming over multivariate hypergeometric states with an optimal pick rule and reshuffles |
| 12 | Draws from a deck plus a reshuffled discard | §7.4 accuracy late in the game | Engine reshuffles the discard when the deck runs short ([`ReshufflePowerDeck`][dll]) | A two-pool draw distribution; how to handle disagreement between the allowed pool minus visible cards and the deck count |
| 13 | "Will this ravage still blight?" given events and earned fear cards | The Thip-style alert | Thip's figures multiply as if independent, on pools from before Nature Incarnate ([BGG 2692815](https://boardgamegeek.com/thread/2692815)) | Exact without-replacement computation over current fear and event pools, conditioned on Terror Level and branch |
| 14 | Monte Carlo display stability | A live UI | Wilson intervals; common random numbers; 2,401 rollouts for ±2 points ([Owen][owen]; [Brown, Cai & DasGupta][bcd]) | Synchronize random streams "by meaning" when actions change how many cards are drawn; a rounding policy; stopping when the interval is narrow enough |
| 15 | Scoring the learner when p* is itself simulated | The optional hold mode | Expected Brier excess (q − p*)² ([Gneiting & Raftery][gr]) | Correct for Monte Carlo error in p*; reliability diagrams with few forecasts per bin |
| 16 | Validating the companion's own forecasts without win/loss counting | Trust in the numbers | 45 `EndGame` journals with explore, ravage and blight entries ([dll]) | Proper-score the per-land and per-step forecasts against journal outcomes; check that scoring cannot leak hidden state |
| 17 | Long et al.'s leaf correlation, bias and disambiguation measured on Spirit Island | Choosing between determinization and expectimax | Never measured ([Long et al.][long]) | Estimate from random playouts once a rules engine exists |

## Appendix B: physical-copy verification checklist

Rule points, in the order a `rules.rs` implementation needs them. "☑" marks an item already settled against the physical copy.

| ☐ | Rule point | What the sources say | Companion impact |
|---|---|---|---|
| ☑ | Turn-1 Event | **Resolved by the user's physical copy:** Jagged Earth p.12 is correct. The Event is drawn and discarded without resolving; it is public and known to be out of the deck. The engine matches ([JE p.12][je]; [spec §9][spec]). | Event pool minus 1 on Turn 1. Fix the memory note and si-live's `previewed` model (below). |
| ☐ | Invader pool: Stage I = 4 single-terrain cards; Stage II = Coastal Lands + 4 single-terrain cards with the escalation flag; Stage III = 6 two-terrain cards; one removed per stage | Engine `InvaderCardList` ([dll]); rulebook ([rulespal]) | Every explore odd |
| ☐ | Build: more Towns than Cities → add a City; otherwise add a Town; builds only in lands with invaders | Engine `GetIslandPieceTypeToBuildInLand` ([dll]); rulebook ([rulespal]) | Build forecast; the invader-lifecycle chapter |
| ☐ | Explore source: the land has a Town or City, or is adjacent to a Town, City or Ocean; only one Explorer per land; Explorers are not a source (except under Surges) | Wiki §3c copy; engine `GetLandsToExploreFrom` ([dll]) | Per-land explore odds |
| ☐ | Ravage: one blight if damage after Defend ≥ land health; Dahan destroyed efficiently; counterattack after invaders by default; Jagged Earth p.7 order for Strife and Badlands; does Defend also reduce damage to Dahan? | Rulebook ([rulespal]); [JE p.7][je]; the engine's handling of Defend against Dahan was not read | Ravage ledger |
| ☐ | Adding blight destroys one presence of each spirit in the land; a cascade happens only if the land already had blight, and the player picks the adjacent land | Engine `AddBlightAction` ([dll]); rulebook ([rulespal]) | Presence risk; cascade badge |
| ☐ | Blight pool: healthy side 2 per player + 1 at setup only; flipped side X per player with no +1; per board with an extra board; 5 per player + 1 with no card; solo redraw of 2-per-player cards | [NI p.5, p.14][ni]; [querki errata][q-blt]; [JE p.18][je] | Blight runway |
| ☐ | Fear: 4 per player per card; +1 per player for England L6 and for an extra board; Town 1, City 2, Explorer 0; Terror Level changes at dividers; cards earned in one window resolve at the new level | Engine `CreateFearValues`, `get_FearFromDestroy` ([dll]); [BGG 2611841](https://boardgamegeek.com/thread/2611841) | Fear runway |
| ☐ | When the Terror Level changes: immediately when the divider is reached, or later? | In LOG-B the level change is sourced to the Power card Visions of Fiery Doom, which suggests it happens mid-resolution (inference) ([LOG-B][logB]) | Victory-blocker timing |
| ☐ | Are earned fear cards face-down until resolved, and does the Steam UI show them? | Rulebook face-down ([rulespal]); the log names them ([LOG-B][logB]) | Firewall † row |
| ☐ | Power gain: draw 4, keep 1, the rest to the discard; a Major requires a Forget; the discard is reshuffled when the deck runs out | Rulebook ([rulespal]); engine `ReshufflePowerDeck` ([dll]) | Draft odds |
| ☐ | England L6 difficulty: 10 or 11 | 10 in the engine and base rulebook; 11 on the wiki ([w-eng]) | Difficulty display |
| ☐ | France Town cap: per player or per board | Wiki per player; engine per board; JE p.18 bases it on boards with an extra board ([w-fra]; [JE][je]) | France loss clock |
| ☐ | Sweden L1 and Habsburg Livestock loss: "≥6 / 8+ damage" or "4+ / 6+ overkill" | Nature Incarnate p.14 overkill wording ([NI][ni]); [querki FAQ, JE][q-je] | Blight ledger when land health is modified |
| ☐ | Combined adversaries: fear deck summed per Terror Level; supporting escalation on every Stage III card; supporting-first deck order; random terrain for France and Sweden | [JE p.19][je]; [querki FAQ, Combining][q-ca] | Recipe interpreter; race |
| ☐ | Russia L5: are "unused" cards chosen randomly, and how do they count in the fear deck? | Not found in the sources ([w-rus]) | Exchangeability; firewall |
| ☐ | Tie-break rules for "add to a land" choices (Prussia escalation, Sweden L5, Scotland escalation) | Only "players make decisions" ([JE p.12][je]) | Candidate sets in forecasts |
| ☐ | Disintegrating Ecosystem: 3 or 5 blight per player | Conflicting posts and the project scrape; the wiki table lists 5 ([w-blc]) | Post-flip X distribution |
| ☐ | Loss timing: losses checked at the end of action trees; deck exhaustion at turn D's Explore; a spirit with no presence loses | [NI p.13][ni]; rulebook ([rulespal]); lumia | Race clocks |
| ☐ | Active and retired cards (Tipping Point, Growth Through Sacrifice, three Events) and final pool sizes | [NI p.5][ni]; engine counts differ (101 Minors) ([dll]) | Every pool |
| ☐ | Horizons board: 5N+1 blight; the +1 errata does not apply | Secondary source only ([OGR](https://officialgamerules.org/game-rules/horizons-of-spirit-island/); [querki errata][q-blt]) | Blight runway on Horizons |

Project data and chapter errors. Each conflicts with the engine, the rulebooks or the research sources; confirm against the box, then fix.

| ☐ | File | Project claim | What the sources say |
|---|---|---|---|
| ☐ | Memory `feedback_event_card_turn1_preview.md` and si-live `EventDeckState.previewed` | Turn-1 Event is a face-up preview that resolves on Turn 2 | **Wrong, settled by the physical copy:** it is drawn and discarded without resolving ([JE p.12][je]) |
| ☐ | `data/adversaries.json` | France `[2,3,5,6,8,9,10]`; Habsburg Mining `[1,3,5,6,7,9,10]` | France L3 = 7; Habsburg Mining `[1,3,4,5,7,9,10]` ([w-fra]; [NI p.12][ni]; [dll]) |
| ☐ | `data/adversary-stacks.json` | Levels keyed by difficulty value; 62 `[VERIFY]` lines | Key by level 0–6. Correct stacks: Prussia L2–L6, Russia L4+, Habsburg Livestock L3+, Scotland L2–L4, Habsburg Mining L4+, Sweden L4 accelerate (tables above) |
| ☐ | `data/references/adversaries/*.json` | Level names and effects such as England "Ironclads" and Prussia L2 "Mercenaries" | Mostly invented; regenerate from the wiki and engine ([w-eng]; [w-bp]) |
| ☐ | `data/scenarios.json` | Great River 1, Varied Terrains 0, Destiny Unfolds 2; Surges missing | 3, 2 and −1\* (0 to −2); add Surges +2/+7 ([w-soc]; engine `ScenarioList` [dll]) |
| ☐ | Spec §7.2 | Adversary loss conditions not modeled in v1, only warned | Six of eight adversaries have one; the race chart needs them ([w-adv]) |
| ☐ | [fear-track.md][fear] | Explorer kill = 1 fear; Town = 2 and City = 3 in the exchange table | Explorer 0, Town 1, City 2 ([`Constants`][dll]) |
| ☐ | [fear-track.md][fear] | "12 total to reach Terror 3"; "each flip shifts the Terror Level"; Terror-1/2/3 card types; flip at Time Passes | Level changes at dividers: 3 cards (12 fear) to reach TL II solo, 6 cards to reach TL III. Fear cards are one pool cut by dividers. Time Passes has no terror step ([`CreateFearDeck`, `TimePassesAction`][dll]) |
| ☐ | [fear-track.md][fear] | Sweden "penalize[s] fear cards"; Russia "fear-suppression"; "45-card fear deck"; mindwanderer terror-by-turn win rates | No Sweden rule touches fear. Russia L2 adds fear. France L6 is the adversary that weakens fear cards. The pool is 50. The win-rate statistics are unsourced and are aggregate counting ([w-swe]; [w-rus]) |
| ☐ | [tempo.md][tempo] | Prussia "flat-early"; Russia "fear-blocked"; tempo defined as a conversion rate | Prussia L2+ brings Stage III to turn 3 and L6 has no Stage I. Russia L2 adds fear. Split "land tempo" from "economy momentum" ([glossary](https://iam.lumia.gay/si_glossary)) |
| ☐ | [invader-lifecycle.md][lifecycle] | 1 Town + 0 Cities builds a Town; Stage II is "double-terrain"; Stage III is coastal/inland or three-terrain; England "Stage III adds Cities to coasts" | That land builds a City. Stage II is single-terrain plus Coastal. Stage III is six two-terrain cards. England's escalation is Building Boom on Stage II ([dll]; [rulespal]; [w-eng]) |
| ☐ | [presence-economy.md][presence] | Ravage destroys presence, which "usually adds blight"; placing presence "saves 1 blight"; every spirit starts with 3 presence on the board | Adding blight destroys presence, not the reverse. Presence does not prevent blight. Starting presence varies (lumia: Towering Roots and Wounded Waters start with 4) ([`AddBlightAction`][dll]; [more T1 victories](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html)) |
| ☐ | [card-draft-theory.md][draft] | The other 3 return to the bottom; "pass to shuffle" improves the next offer; a pooled multiplayer offer; Minor ~101 / Major ~80; mindwanderer stat on passing Majors | They go to the discard. You cannot pass a gain. No source supports the pooled offer. Pools are 100–101 and 78. The statistic is unsourced and aggregate ([rulespal]; [dll]) |
| ☐ | [energy-curves.md][energy] | Per-spirit Energy targets "directional," unsourced | Compute from the engine's `SpiritList` tracks and `si-rules-check` |
| ☐ | [statistics/index.md][stats-index] | "Event draws are independent per turn" | One shuffled deck, no reshuffle: exchangeable, not independent ([`CreateEventDeck`][dll]) |
| ☐ | `src/statistics/digital-vs-tabletop.md` | Digital is "cryptographically random; every draw is truly independent"; tabletop is "~3–7% easier" | A seeded subtractive generator fixes the decks at setup ([ecdll]); the percentage is unsourced |
| ☐ | `src/statistics/reading-mindwanderer.md` | "Self-reported," "digital… + tabletop, blended" | All data is from Spirit Island Digital; the models are regressions ([mindwanderer stats](https://mindwanderer.net/si/stats.html)) |
| ☐ | `src/statistics/deck-expansion-impact.md`, `fear-card-expected-value.md`, `scripts/deck-simulate.py` | Fear cards drawn at "TL1→TL2 (4 fear) and TL2→TL3 (8 fear)"; 2.00 cards per game | One card per 4N fear; Terror Levels at dividers ([dll]; [BGG 2611841](https://boardgamegeek.com/thread/2611841)) |
| ☐ | `src/statistics/major-vs-minor-rates.md` | Majors are "3-card-view-pick-1," "≥3 Energy," gated on threshold | Draw 4; 2-cost Majors exist; Majors are playable without their threshold ([BGG 2551422](https://boardgamegeek.com/thread/2551422); [BGG 2519969](https://boardgamegeek.com/thread/2519969)) |
| ☐ | `src/statistics/event-deck-risk-profiles.md` | "JE events skew pro-spirit" | The designer intended events to be slightly negative ([BGG 2506650](https://boardgamegeek.com/thread/2506650)) |
| ☐ | `src/statistics/blight-deck-severity.md` and `src/appendices/fear-blight-event-decks.md` | Base blight cards are "Wildfires, A Pall Upon the Land"; keyword severity proxy; no errata; fear crossing at 4/8/12 × N | Base cards per darker are Downward Spiral and Memory Fades to Dust. "Wildfires" is not in the 23-card active pool. Add the +1 errata. Fix the fear model ([BGG 2695288](https://boardgamegeek.com/thread/2695288); [w-blc]) |
| ☐ | `src/statistics/aspect-power-deltas.md` | Sunshine "~+3%" (unsourced) | Unsourced, and it is aggregate counting, the excluded genre. Remove or source it ([BGG 3658424](https://boardgamegeek.com/thread/3658424)) |
| ☐ | `src/appendices/sources.md` | Jonathan4290 "Thirteen-entry" series; "2.27M" games | 26 entries; 2.27M is log lines, not games ([BGG 3569421](https://boardgamegeek.com/thread/3569421)) |

[dll]: file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll
[ecdll]: file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/EngineCommon.dll
[logA]: file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt
[logB]: file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt
[save]: file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat
[spec]: file:///home/brett/repos/spirit-island-steam-companion/docs/superpowers/specs/2026-10-18-steam-companion-design.md
[tempo]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/tempo.md
[fear]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/fear-track.md
[lifecycle]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/invader-lifecycle.md
[energy]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/energy-curves.md
[presence]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/presence-economy.md
[draft]: file:///home/brett/repos/spirit-island-steam-companion/src/fundamentals/card-draft-theory.md
[stats-index]: file:///home/brett/repos/spirit-island-steam-companion/src/statistics/index.md
[rulespal]: https://rulespal.com/spirit-island/rulebook
[je]: https://boardgame.bg/spirit%20island%20jagged%20earth%20rules.pdf
[ni]: https://boardgame.bg/spirit%20island%20nature%20incarnate%20rules.pdf
[q-core]: https://querki.net/raw/darker/spirit-island-faq/Core+Game
[q-je]: https://querki.net/raw/darker/spirit-island-faq/Jagged+Earth
[q-ca]: https://querki.net/raw/darker/spirit-island-faq/Combining+Adversaries
[q-blt]: https://querki.net/raw/darker/spirit-island-faq/one-more-blight-errata
[w-adv]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Adversary
[w-bp]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Brandenburg-Prussia
[w-eng]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=England
[w-swe]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Sweden
[w-fra]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=France
[w-hlc]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Habsburg_Monarchy
[w-hme]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Habsburg_Mining_Expedition
[w-rus]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=The_Tsardom_of_Russia
[w-blc]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Blight_Card
[w-soc]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Surges_of_Colonization
[gs]: https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf
[df]: https://projecteuclid.org/euclid.aop/1176994828
[owen]: https://artowen.su.domains/mc/
[bcd]: https://projecteuclid.org/euclid.ss/1009213286
[putter]: https://www.est.ufmg.br/~enricoc/pdf/sobre/Putter2007.pdf
[long]: https://cdn.aaai.org/ojs/7562/7562-13-11092-1-2-20201228.pdf
[cowling]: https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf
[klc]: https://people.smp.uq.edu.au/YoniNazarathy/Control4406_2014/resources/KaelblingLittmanCassandra1998.pdf
[gr]: https://apps.dtic.mil/sti/html/tr/ADA459827/index.html
[yoon]: https://mlanthology.org/aaai/2008/yoon2008aaai-probabilistic
[sfikas]: https://arxiv.org/abs/2103.15090v1
