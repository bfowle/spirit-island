# Statistical, probabilistic, and decision-theoretic models for a Spirit Island learning companion

Legend used throughout. **[V]** = citation details confirmed this session against the linked page or PDF. **[U]** = from prior bibliographic knowledge, NOT confirmed this session; check before citing. Spirit Island (SI) mechanics are taken only from the structure given in the assignment. Any further rule detail is marked "(verify vs rulebook)". Derivations marked "derivation" are math worked here, not sourced claims.

Scope exclusion honored: no model below takes aggregate win/loss counts as its only input (no win-rate tables, no Elo). Every quantity is conditional on the current game state, from exact computation or from forward simulation of that state.

---

## 1. Combinatorics of hidden decks: hypergeometric, multivariate hypergeometric, negative hypergeometric, exchangeability

### Takeaway
Card questions in SI are exact, closed-form, and cheap. The sufficient statistic is the multiset of unseen cards in each deck plus the known stage-boundary positions. Exchangeability of a uniform shuffle proves that a removed-and-unseen card leaves every unseen card equally likely to be next. These belong in the companion as exact numbers, never simulated.

### Cited Findings
- Grinstead & Snell define the hypergeometric distribution in Sec. 5.1. Take N balls, k red, and draw n without replacement; X = number of red drawn. The book is free under the GNU Free Documentation License (2006 Doyle version of the AMS 2nd ed.). It also has Sec. 3.3 "Card Shuffling". — [Grinstead & Snell PDF, Berkeley mirror](https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf) [V]; [Columbia mirror](https://www.cs.columbia.edu/~djhsu/coms4771-f23/Grinstead-Snell.pdf) [V, license note via search]
  - Full citation: Grinstead, C. M., & Snell, J. L. (1997). *Introduction to Probability* (2nd rev. ed.). American Mathematical Society. Free PDF: https://math.dartmouth.edu/~prob/prob/prob.pdf (mirror links above verified; the Dartmouth canonical URL is [U]).
- Diaconis & Freedman (1980), *Annals of Probability* 8(4):745–764, "Finite Exchangeable Sequences." Suppose a finite exchangeable 0-1 sequence of length k extends to an exchangeable sequence of length n. Then it lies within variation distance 4k/n of a mixture of i.i.d. Bernoulli sequences; for t-valued outcomes the bound is 2tk/n. — [Utah bib record](https://ftp.math.utah.edu/pub/tex/bib/idx/annprobab1980/8/4/745_764.html) [V]; [Project Euclid](https://projecteuclid.org/euclid.aop/1176994828) [V]
- Frank Karsten's mana-base work is the usual deck-math reference point. Primary titles: "How Many Lands Do You Need in Your Deck: An Updated Analysis" ([ChannelFireball URL](https://www.channelfireball.com/article/How-Many-Lands-Do-You-Need-in-Your-Deck-An-Updated-Analysis/cd1c1a24-d439-4a8e-b369-b936edb0b38a/); [TCGplayer mirror](https://infinite.tcgplayer.com/article/how-many-lands-do-you-need-in-your-deck-an-updated-analysis/cd1c1a24-d439-4a8e-b369-b936edb0b38a)), plus "How Many Colored Mana Sources Do You Need to Consistently Cast Your Spells?" and its "A 2022 Update". A secondary source says this work rests on hypergeometric probability with a 90% consistency target. — [scrollvault summary (secondary)](https://scrollvault.net/guides/mana-bases.html); [land-calc crate (links primary)](https://rust-digger.code-maven.com/crates/land-calc). The primary article would not render in WebFetch, so its methodology is unverified.

### Model cards

**M1. Univariate hypergeometric ("x of y unseen; P(at least one in n draws)")**
- Question answered: "Of the N unseen cards, K have property A. What is P(≥1 A in my next n draws)? What is the distribution of the count?"
- Formulas: P(X=x) = C(K,x)C(N−K,n−x)/C(N,n). P(X≥1) = 1 − C(N−K,n)/C(N,n). E[X] = nK/N. Var[X] = n(K/N)(1−K/N)(N−n)/(N−1), where (N−n)/(N−1) is the finite-population correction.
- Assumptions: the unseen cards are in uniformly random order given everything observed (fair shuffle, no peek or reorder effects); sampling without replacement; N and K known exactly.
- Inputs: deck composition minus seen cards. For the invader deck, also the stage-boundary positions.
- Cost: closed form, O(n). Microseconds.
- Presenting to a learner: show "3 of 7 unseen" as a strip of card backs with the matching ones highlighted, then the percentage. Teach the complement trick: "P(no A) is easier; subtract from 1."
- Pitfalls: (i) Using the binomial (with replacement). It overstates variance, and by Diaconis–Freedman its error grows like draws/deck size, which is large for 5–12-card stages. (ii) Conditioning on information the player does not have. (iii) Forgetting reshuffle or discard-recycling rules for power decks, which break pure without-replacement (verify vs rulebook).
- Refs: Grinstead & Snell Sec. 5.1 [V]; Johnson, Kemp & Kotz (2005) [U]; Karsten (deck-math exemplar) [title V, content via secondary].
- Fit: 5/5.

**M2. Multivariate hypergeometric (several categories at once)**
- Question: "In the next n draws, what is the joint chance of x₁ cards of category 1, x₂ of category 2, ...?" Examples: "both of my weak terrains come up this stage"; "one of the next minor powers offers element E with cost ≤ c".
- Formula: P = Π C(Kᵢ,xᵢ)/C(N,n) with Σxᵢ = n. Cov(Xᵢ,Xⱼ) = −n(Kᵢ/N)(Kⱼ/N)(N−n)/(N−1). Categories are negatively correlated.
- Assumptions: as M1, plus the categories must PARTITION the unseen set.
- Inputs: counts per category.
- Cost: closed form; enumeration over category vectors is tiny for SI deck sizes.
- Presenting: a stacked bar of "what the next n draws could look like" with probabilities. Or the single headline P(goal), with "why" on drill-down.
- Pitfalls: power-card properties (elements, speed, cost) overlap, so "Fire" and "Plant" are not a partition. Partition by full signature, or compute the complement event directly ("no card has both Fire and cost ≤ 1"). Double counting across overlapping categories is the most common bug.
- Refs: Johnson, Kemp & Kotz (2005) [U]; Grinstead & Snell (hypergeometric, Sec. 5.1) [V].
- Fit: 4/5. Very useful for power-deck draft questions; the overlap bookkeeping is the cost.

**M3. Negative hypergeometric ("draws until")**
- Question: "How many draws until the first (or r-th) card of type A?" Examples: "rounds until a terrain-X invader card", "fear cards until a specific effect".
- Formulas: with W = draws strictly before the first A, P(W ≥ w) = C(N−K,w)/C(N,w). The expected position of the r-th A among N cards with K A's is r(N+1)/(K+1). For the first one, (N+1)/(K+1) is an easy rule of thumb to teach.
- Assumptions: as M1. The count must respect stage boundaries; across a boundary, use the next stage's composition.
- Cost: closed form.
- Presenting: "On average the next Wetland arrives in about 2 rounds, but there's a 1-in-4 chance it doesn't come this stage." Pair the mean with a tail probability so learners don't anchor on the mean.
- Pitfalls: using the geometric distribution (with replacement); ignoring that the removed card can make "never this stage" possible (see M4).
- Refs: Johnson, Kemp & Kotz (2005) [U].
- Fit: 4/5.

**M4. Exchangeability lemma for the removed-and-unseen card (derivation)**
- Setup: a stage has m distinct cards; one is removed uniformly at random and unseen, and the other m−1 are shuffled uniformly. This is equivalent to one uniform random permutation σ of all m cards, with the deck = σ(1..m−1) and the removed card = σ(m).
- Claim: after j observed draws from the stage (j ≤ m−2), (σ(j+1), …, σ(m)) is a uniform random permutation of the unseen set U, with |U| = m−j. Hence:
  1. P(next card = c | history) = 1/(m−j) for every c ∈ U. This is identical to a deck with no removal and the same unseen set. The removal changes how many draws are left in the stage, not which card is next.
  2. P(c appears among the remaining r = m−1−j draws of the stage) = r/(m−j).
  3. P(c is the removed card | history) = 1/(m−j).
  4. After all m−1 draws, if the stage composition is known, the removed card is identified by elimination.
  5. Category version: if K cards in U share property A, P(next ∈ A) = K/(m−j), and the count of A in the remaining r draws is Hypergeometric(N = m−j, K, n = r).
  6. Different stages are shuffled independently, so draws in different stages are independent. Within a stage they are negatively associated.
- Assumptions: uniform shuffle; no effect has revealed, reordered, added, or removed cards. Any such effect means conditioning on that event, and the belief may stop being uniform. If an adversary or scenario changes the deck build, recompute the stage sizes and boundary positions from the actual build (verify vs rulebook).
- Presenting: "Each of the 3 unseen Stage II cards is equally likely to be next: 1 in 3. The removed card doesn't change that. It only means one of them won't show up this stage."
- Pitfalls: the intuition that "the removed card makes some cards less likely next"; treating the removed card as known.
- Refs: Diaconis & Freedman (1980) [V]; Grinstead & Snell Sec. 3.3 [V section exists]. The lemma itself is elementary symmetry. I found no SI-specific published source and none is needed.
- Fit: 5/5. This is the foundation of the companion's whole belief model.

### Inferences
- Belief state for all SI hidden decks: given exchangeability, the posterior over unseen order is uniform over permutations consistent with what has been seen. So the belief state reduces to (unseen multiset per deck, known boundary positions, any known top cards from peek effects). Belief updating is "remove the seen card." This collapses the POMDP machinery in Sec. 5 to a much simpler stochastic-planning problem.
- Within-stage negative dependence means SI deck "it's due" reasoning is partly valid, unlike for dice; see Sec. 7. The companion should teach that distinction explicitly.
- Diaconis–Freedman gives a quantitative reason never to approximate SI deck draws as binomial. With draws k comparable to deck size n, the k/n-scale bound is not small.

### Gaps
- The Karsten primary articles would not render, so I could not confirm the methodology (exact hypergeometric vs. simulation with mulligans) or the 90% threshold firsthand. That claim rests on a secondary site.
- Unverified reference to confirm: Johnson, N. L., Kemp, A. W., & Kotz, S. (2005). *Univariate Discrete Distributions* (3rd ed.). Wiley. This covers hypergeometric and negative hypergeometric.
- The SI-specific deck facts the formulas need (stage sizes, which effects peek or reorder, power-deck reshuffle rules) were out of scope. They need physical-copy verification.

---

## 2. Stochastic processes: absorbing Markov chains, phase-type distributions, first passage, gambler's ruin, branching processes

### Takeaway
Absorbing-chain and discrete phase-type machinery is the right mathematical language for "race to finish" questions, but SI's full state is far too large to use exactly. Two consequences follow. (a) It is best used on a small abstracted state, or as the presentation layer for simulation output. (b) SI's invader-deck clock is a deterministic deadline, not a random hitting time, so the problem is finite-horizon, time-inhomogeneous forward recursion rather than an infinite-horizon fundamental matrix. Gambler's ruin and branching processes are teaching analogies, not computational models, for SI.

### Cited Findings
- Grinstead & Snell Sec. 11.2 (absorbing Markov chains) covers the following. Definition 11.3: for canonical form P = [[Q, R], [0, I]], the fundamental matrix is N = (I − Q)⁻¹, and nᵢⱼ is the expected number of visits to transient state j starting from i. Theorem 11.5: expected steps to absorption t = Nc, with c a column of ones. Absorption probabilities are B = NR. — [Grinstead & Snell PDF](https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf) [V, text read]
- Grinstead & Snell also has Sec. 10.2 "Branching Processes", Sec. 11.5 "Mean First Passage Time", and Sec. 12.2 "Gambler's Ruin". — [same PDF, table of contents](https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf) [V]
- Latouche, G., & Ramaswami, V. (1999). *Introduction to Matrix Analytic Methods in Stochastic Modeling*. ASA-SIAM Series on Statistics and Applied Probability. ISBN 0898714257. Part II, "The Method of Phases", includes Ch. 2 "PH Distributions". — [KIT OPAC record](https://opac.lib.kit.ac.jp/opac/opac_link/bibid/BB00079617) [V]; [woodslane listing](https://woodslane.com.au/products/9780898714258) [V]. Depth of discrete-PH coverage not confirmed.
- Bladt, M., & Nielsen, B. F. (2017). *Matrix-Exponential Distributions in Applied Probability*. Springer, Probability Theory and Stochastic Modelling vol. 81. DOI 10.1007/978-1-4939-7049-0. Matrix-exponential distributions are the non-negative distributions with rational Laplace transforms; phase-type is a subclass. The book is mainly continuous-time. — [Springer](https://link.springer.com/978-1-4939-7047-6) [V]
- Athreya, K. B., & Ney, P. E. (1972). *Branching Processes*. Springer, Grundlehren der mathematischen Wissenschaften vol. 196. Dover reprint 2004. Ch. V covers multi-type processes, including "moments and the Frobenius theorem" (Perron–Frobenius growth rate). — [Springer](https://link.springer.com/book/9783642653735) [V]; [CiNii](https://ci.nii.ac.jp/ncid/BA00018611) [V]
- Kimmel, M., & Axelrod, D. E. (2015). *Branching Processes in Biology* (2nd ed.). Springer, Interdisciplinary Applied Mathematics vol. 19. DOI 10.1007/978-1-4939-1559-0. The 1st ed. is 2002. It covers Galton–Watson, age-dependent, Bellman–Harris, and multitype processes. — [Springer](https://www.springer.com/us/book/9781493915583) [V]
- Kemeny & Snell's *Finite Markov Chains* (1976 Springer edition) is cited alongside Grinstead & Snell Ch. 11 as the standard treatment of the fundamental matrix. — [Princeton LIPS blog](https://lips.cs.princeton.edu/the-fundamental-matrix-of-a-finite-markov-chain) [V for the mention only]

### Model cards

**M5. Absorbing Markov chain on an abstracted state (finite-horizon form)**
- Question: "From here, what is P(win) vs P(each loss), and how many rounds is this likely to take?"
- Formulas: infinite-horizon, time-homogeneous form: B = NR gives absorption probabilities per absorbing class and t = Nc gives expected time. SI form (inference): the invader deck is a known-length clock, so use time-indexed transition matrices Pₜ and push the distribution forward, πₜ₊₁ = πₜPₜ, up to the final round H. The absorbed mass by class and round gives the race curves in Sec. 3.
- Assumptions: Markov property ON THE CHOSEN ABSTRACTION. The future depends on the past only through the abstract state. A Markov chain models a FIXED policy; with choices it becomes an MDP (Sec. 5).
- Inputs: the abstract state, for example (round, fear to next terror level, blight left in pool, number of "loaded" lands), plus transition probabilities. These come either from card combinatorics (Sec. 1) or from short simulations from representative states.
- Cost: closed form. O(s³) for N, O(H·s²) for forward recursion. Trivial for s up to about 10³–10⁴.
- Presenting: a state diagram with a few named states ("safe / stressed / one cascade from loss") and arrows labeled with percentages. Learners see "where the probability mass flows."
- Pitfalls: lumping the true state into abstract states generally destroys the Markov property (lumpability rarely holds exactly), so numbers are approximations. Treating the deck clock as random. Estimating the transition matrix from games played under a different policy than the one being evaluated.
- Refs: Grinstead & Snell Ch. 11 [V]; Kemeny & Snell (1960/1976) [U]; Puterman (1994) for the controlled version [U].
- Fit: 3/5. Excellent for teaching the shape of a race; weak as an exact predictor for SI because of abstraction error.

**M6. Discrete phase-type distribution with competing absorbing states**
- Question: "Not just the mean: what is P(game still undecided after k more rounds)? What is P(loss specifically by blight in round k)?"
- Formulas: initial distribution α over transient states. P(T > k) = α Qᵏ 1. P(T = k, absorbed in class j) = α Qᵏ⁻¹ R eⱼ. E[T] = α N 1. The second expression is exactly a discrete-time cause-specific cumulative incidence increment (Sec. 3).
- Assumptions/inputs/cost: as M5.
- Presenting: a histogram of "round the game ends", stacked by outcome.
- Pitfalls: as M5. Also note that PH families are dense in distributions on the positive integers, so a fitted PH can mimic anything; that flexibility is not evidence of a correct mechanism.
- Refs: Neuts (1981) [U]; Latouche & Ramaswami (1999) [V existence/PH chapter]; Bladt & Nielsen (2017) [V, continuous focus].
- Fit: 3/5. Same caveats as M5; the main value is that it ties Markov chains to survival presentation.

**M7. Gambler's ruin / random walk with absorbing barriers (teaching analogy)**
- Question: "With B blight left and a net drain per round, what is P(running out before I win)?"
- Formula (standard): with steps +1 w.p. p and −1 w.p. q = 1−p, absorbing at 0 and N, starting at i: P(reach N before 0) = (1 − (q/p)ⁱ)/(1 − (q/p)ᴺ) for p ≠ q, and i/N for p = q. The expected duration for p = q is i(N − i).
- Assumptions: i.i.d. ±1 steps on ONE axis. SI breaks all three: blight additions are lumpy (cascades), state-dependent, and non-identically distributed, and the race is two-dimensional (fear progress vs. blight drain vs. deck deadline).
- Cost: closed form.
- Presenting: the intuition that "a small per-round edge compounds" and that "starting buffer matters non-linearly near the barrier."
- Pitfalls: quoting gambler's-ruin numbers as SI predictions.
- Refs: Grinstead & Snell Sec. 12.2 [V]; Feller (1968) Ch. XIV [U]; Tijms (2012), whose 3rd ed. adds hitting probabilities in random walks [V, see Sec. 8].
- Fit: 2/5. Use as an explainer only.

**M8. Multi-type branching process / matrix population projection (teaching analogy for invader growth)**
- Question: "If I leave this area alone, how fast does it snowball?" The answer is the dominant (Perron) eigenvalue ρ of the mean matrix M (expected next-round counts of each type per current individual): ρ > 1 means unchecked growth.
- Assumptions (branching): individuals reproduce INDEPENDENTLY with type-dependent offspring laws. SI violates this. Per the given structure, Build and Explore act per LAND (presence and adjacency conditions), not per invader. They are triggered by the drawn card's land-selection criterion, are deterministic once the card is known, are spatially constrained, and are actively suppressed by players. Invader dynamics are an occupancy/contact process on a graph (Sec. 6), not independent reproduction.
- Inputs: a mean matrix estimated from rule simulation of an untouched region.
- Cost: an eigenvalue of a 3×3 or small land-level matrix. Trivial.
- Presenting: "growth multiplier per round if untouched" for a region. Useful to motivate early suppression ("a town is both a builder and an explore source").
- Pitfalls: treating explorers, towns, and cities as independent breeders; mean-field projections on a graph of a few dozen nodes.
- Refs: Athreya & Ney (1972) Ch. V [V]; Kimmel & Axelrod (2015) [V]; Grinstead & Snell Sec. 10.2 [V]; Harris (1963) [U].
- Fit: 2/5 (analogy).

**M9. First passage / renewal approximation for "rounds until the fear threshold"**
- Question: "At my current fear rate, how many rounds until the next terror level or fear victory?"
- Formula (inference): if per-round fear increments Xᵢ were i.i.d. with mean μ and variance σ², the stopping time T of the cumulative sum crossing threshold F satisfies Wald's identity E[S_T] = μE[T], so E[T] ≈ F/μ plus an overshoot correction. A normal approximation gives P(T ≤ k) ≈ Φ((kμ − F)/(σ√k)).
- Assumptions: i.i.d. increments. In SI, fear generation is policy-dependent and usually rises over the game, so increments are neither identically distributed nor independent of state.
- Cost: closed form.
- Presenting: "about 3 rounds at your current pace; 4 if you lose a round to defense."
- Pitfalls: extrapolating a rising rate as constant; ignoring that fear-card positions and terror-level dividers are KNOWN (deterministic) once the fear deck is built, so only the generation rate is uncertain (verify the fear-deck construction vs rulebook).
- Refs: Grinstead & Snell Sec. 11.5 (first passage in ergodic chains) [V]; Feller vol. 2 on renewal theory [U].
- Fit: 3/5. Useful quick heuristic; Monte Carlo (Sec. 4) supersedes it for real numbers.

### Inferences
- Structural point for the statistician (inference from the given structure): SI's race is (random fear-accumulation clock) vs. (random blight-drain clock) vs. (deterministic invader-deck deadline), plus any adversary conditions. The deadline behaves like administrative censoring at a known time (Sec. 3). It is not a hitting time, so infinite-horizon absorbing-chain results do not apply to it directly.
- The decision-relevant randomness in SI is narrow: card order (exchangeable, Sec. 1) and, in some content, other random elements (verify which, vs rulebook). Given cards, rule resolution is mostly deterministic. So "Markov chain on abstract state, fit from simulation" is usually dominated by "exact rule engine + exact card enumeration + Monte Carlo over remaining card orders." Abstracted chains are mainly for pedagogy.

### Gaps
- Unverified references to confirm:
  - Kemeny, J. G., & Snell, J. L. (1960). *Finite Markov Chains*. Van Nostrand; reprinted 1976, Springer (Undergraduate Texts in Mathematics). Covers the fundamental matrix and lumpability.
  - Neuts, M. F. (1981). *Matrix-Geometric Solutions in Stochastic Models: An Algorithmic Approach*. Johns Hopkins University Press (Dover reprint 1994). Covers discrete and continuous phase-type.
  - Feller, W. (1968). *An Introduction to Probability Theory and Its Applications*, Vol. 1 (3rd ed.). Wiley. Ch. XIV, random walk and ruin problems.
  - Harris, T. E. (1963). *The Theory of Branching Processes*. Springer.
  - Caswell, H. (2001). *Matrix Population Models* (2nd ed.). Sinauer.
- I found no published work applying phase-type or absorbing-chain models to Spirit Island specifically.
- Whether Latouche & Ramaswami or Bladt & Nielsen treat DISCRETE phase-type in depth was not confirmed. Neuts (1981) is the safer [U] citation for discrete PH.

---

## 3. Survival / time-to-event framing: "rounds until loss" and competing risks

### Takeaway
The right presentation of "how is this race going" is a set of state-conditional cumulative incidence curves, one per outcome (win, blight loss, deck loss, adversary loss), estimated from forward simulation or exact recursion from the current state. The classic competing-risks pitfall applies directly: computing each loss clock as if the others did not exist overstates every risk. The deck deadline is administrative censoring at a known round.

### Cited Findings
- Putter, H., Fiocco, M., & Geskus, R. B. (2007). Tutorial in biostatistics: Competing risks and multi-state models. *Statistics in Medicine*, 26, 2389–2430 (online Oct. 2006). Standard survival analysis covers one event type. Competing-risks models describe progression to several possible event types, and multi-state models add intermediate states. The practical focus is on estimating cumulative incidence functions and state/transition probabilities. — [Full text PDF](https://www.est.ufmg.br/~enricoc/pdf/sobre/Putter2007.pdf) [V]; [Oxford NDM listing](https://www.ndm.ox.ac.uk/publications/693970) [V]
- The R `survival` package vignette "Competing Risks and Multistate Models" (Therneau, Crowson & Atkinson) reproduces the Putter et al. examples and is a usable implementation reference. — [CRAN vignette](https://packages.oit.ncsu.edu/cran/web/packages/survivalVignettes/vignettes/tutorial.html) [V]

### Model cards

**M10. Discrete-time competing-risks / cumulative incidence from state-conditional simulation**
- Question: "By round t, what is P(we've won)? P(lost to blight)? P(lost to deck exhaustion)? Which clock is binding?"
- Formulas (discrete time): cause-specific hazard hⱼ(t) = P(T = t, cause = j | T ≥ t). Overall survival S(t) = Π_{u≤t} (1 − Σⱼ hⱼ(u)). Cumulative incidence CIFⱼ(t) = Σ_{u≤t} hⱼ(u) S(u−1). With R rollouts from the current state, the empirical estimate is simply CIF̂ⱼ(t) = (#rollouts ending by cause j at or before t)/R. Every rollout ends with a cause, so there is no censoring except the deterministic deck deadline, which is itself a cause.
- Assumptions: the rollout (or recursion) policy defines the estimand (Sec. 4). Outcomes are mutually exclusive per rollout. If two end conditions can trigger in the same phase, the rulebook's resolution order must be encoded (verify vs rulebook).
- Inputs: rollouts from the CURRENT state (complies with the exclusion: no historical win/loss tables).
- Cost: simulation cost (Sec. 4); the curve construction itself is trivial.
- Presenting: a stacked-area "race chart" over rounds. Bands for win / each loss type / still undecided must sum to 100% at every round. Interactive: "if I take action A, the blight band shrinks by x pp by round 6."
- Pitfalls: (i) Computing P(blight loss) with the win and deck clocks switched off, then quoting it as the real risk. That is the latent-failure-time quantity, and it overstates incidence. The analog of 1 − Kaplan–Meier with competing events treated as censored is that it is ≥ the CIF. (ii) Summing such naive per-cause probabilities to more than 100%. (iii) Reading hazards as probabilities over the whole game.
- Refs: Putter, Fiocco & Geskus (2007) [V]; Fine & Gray (1999) [U]; Kalbfleisch & Prentice (2002) [U].
- Fit: 4/5. This is the most learner-legible way to show multi-clock races. The 1-point deduction is because it is only as good as the rollout policy.

### Inferences
- A simulator can compute counterfactual "cause-deleted" clocks, which are non-identifiable from observational data (Tsiatis 1975 [U]). That is useful for teaching ("if blight weren't an issue, the deck would end you on round 9"), but must be labeled as hypothetical so learners don't confuse it with the real cumulative incidence.
- Hazard plots per round ("this round has a 12% chance of a blight-loss event") are a good companion to CIF curves for in-the-moment play. They answer "is THIS round the dangerous one?"

### Gaps
- Unverified references to confirm:
  - Fine, J. P., & Gray, R. J. (1999). A proportional hazards model for the subdistribution of a competing risk. *Journal of the American Statistical Association*, 94(446), 496–509.
  - Kalbfleisch, J. D., & Prentice, R. L. (2002). *The Statistical Analysis of Failure Time Data* (2nd ed.). Wiley.
  - Singer, J. D., & Willett, J. B. (1993). It's about time: Using discrete-time survival analysis to study duration and the timing of events. *Journal of Educational Statistics*, 18(2), 155–195.
  - Tsiatis, A. (1975). A nonidentifiability aspect of the problem of competing risks. *PNAS*, 72(1), 20–22.
- Regression-style survival models (Cox, Fine–Gray) need many observed games and covariates. Fit from a playlog, they would drift toward the excluded "aggregate outcomes" category. They are listed only as references, not recommended for the in-game layer.

---

## 4. Simulation: Monte Carlo estimation, confidence intervals, variance reduction, sample sizes for a live UI

### Takeaway
Forward Monte Carlo from the current state is the workhorse for every SI question that the closed forms in Secs. 1–2 can't reach. To make it trustworthy and stable in a live UI:
- Use Wilson intervals, not Wald.
- Use common random numbers whenever comparing actions.
- Integrate out the next card exactly (stratify/condition) instead of sampling it.
- Size runs to the precision you display: about 2,400 rollouts for ±2 pp worst-case at 95%.

The estimand is always "probability under the rollout policy," which is a LOWER bound on what good play achieves. That must be shown to the learner.

### Cited Findings
- Owen, A. B. (2013–). *Monte Carlo Theory, Methods and Examples*. Free online book in progress. Ch. 8 "Variance Reduction" covers antithetics, stratification, common random numbers, conditioning, control variates, and moment matching/reweighting. Ch. 9 covers importance sampling; Ch. 10 covers advanced variance reduction (Latin hypercube, etc.). — [artowen.su.domains/mc](https://artowen.su.domains/mc/) [V]
- Brown, L. D., Cai, T. T., & DasGupta, A. (2001). Interval estimation for a binomial proportion. *Statistical Science*, 16(2), 101–133. The coverage of the standard Wald interval is "chaotic" and textbook safety rules are unreliable. The authors recommend the Wilson interval (attributed to Wilson, 1927) or the equal-tailed Jeffreys interval for small n, and Agresti–Coull for larger n. — [Project Euclid](https://projecteuclid.org/euclid.ss/1009213286) [V]; [DOI 10.1214/ss/1009213286](https://www.doi.org/10.1214/SS/1009213286) [V]
- Bjarnason, R., Fern, A., & Tadepalli, P. (2009). Lower bounding Klondike Solitaire with Monte-Carlo planning. *Proc. 19th Int. Conf. on Automated Planning and Scheduling (ICAPS)*, 26–33. Sparse UCT won about 35% of Klondike games, "more than double" the estimated human win rate. — citation and result as reported in [Cowling et al. 2012 PDF](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf) [V via secondary citation]
- Silver & Veness (2010, POMCP) need only a black-box simulator of the domain, not explicit probability models. — [NeurIPS 2010 poster page](https://neurips.cc/virtual/2010/poster/2145) [V]
- In a Hearthstone MCTS study, simulator speed largely determines the performance of search methods. — [arXiv 1808.04794](https://arxiv.org/pdf/1808.04794) [V via search summary only]

### Model cards

**M11. Plain Monte Carlo rollouts from the current state**
- Question: anything not closed-form. Examples: "P(win | this state, this plan)", "P(blight cascade ≥ 2 this round)", "distribution of invaders on board 2 after the next invader phase".
- Estimator: p̂ = X/n with Var = p(1−p)/n. Report a Wilson 95% interval.
- Sample sizes (derivation, n = z²p(1−p)/m², z = 1.96, worst case p = 0.5): ±10 pp → 97; ±5 pp → 385; ±3 pp → 1,068; ±2 pp → 2,401; ±1 pp → 9,604. For p near 0.1, ±2 pp needs about 865.
- Assumptions: an exact rule engine; correct shuffle model (Sec. 1); a specified rollout policy for future player decisions.
- Inputs: full game state plus the unseen-card belief.
- Cost: n × (rollout cost). Feasibility depends on engine speed, so profile before promising a refresh rate.
- Presenting: a single headline percentage, rounded to the displayed precision, with a thin interval bar. Never show more digits than the interval supports.
- Pitfalls: (i) Estimand confusion: the result is P(outcome | state, ROLLOUT POLICY), not "your" win probability. A weak rollout bot biases it downward. (ii) Wald intervals at extreme p (e.g., 1% blight-out risks). (iii) Re-randomizing on every UI refresh makes numbers flicker; fix the seeds.
- Refs: Owen (2013–) [V]; Brown, Cai & DasGupta (2001) [V]; Asmussen & Glynn (2007) [U].
- Fit: 5/5.

**M12. Common random numbers (CRN) for comparing candidate actions**
- Question: "Is play A better than play B, and by how much?"
- Method: run A and B on the SAME sampled card orders (same seeds). Var(X_A − X_B) = Var X_A + Var X_B − 2Cov. With per-rollout outcome correlation ρ and equal variances, SE(diff) = √(2p(1−p)(1−ρ)/n), versus √(2p(1−p)/n) when independent.
- Assumptions: synchronize randomness by meaning (the k-th card drawn from deck D uses the same random stream) so outcomes stay correlated after the paths diverge.
- Cost: same as M11, with far fewer rollouts needed for a stable ranking.
- Presenting: "A beats B by 6 pp (±2)." Also show the paired breakdown: "A is better in 31% of futures, worse in 8%, same in 61%." That breakdown teaches dominance.
- Pitfalls: drawing desynchronized random streams after actions change the number of draws; reading a difference as significant without its interval.
- Refs: Owen Ch. 8 (CRN) [V]; Glasserman (2004) [U].
- Fit: 5/5. This is essential for any "which play?" feature.

**M13. Conditional Monte Carlo / exact stratification over the next card(s)**
- Question: same as M11, with lower variance and a richer explanation.
- Method: by M4 the next invader card is uniform over a small unseen set U. Enumerate each c ∈ U exactly (weight 1/|U|) and simulate only the remaining randomness within each stratum. Wherever a sub-question has a closed form (Sec. 1), substitute the exact probability for the sampled indicator (Rao–Blackwellization).
- Cost: |U| strata × n/|U| rollouts. Proportional stratification never increases variance relative to plain sampling.
- Presenting: a natural "if X is next: a%; if Y is next: b%" table. This is the most instructive display for SI, because learners see which card would hurt and why.
- Pitfalls: forgetting stage boundaries when enumerating two or more cards ahead.
- Refs: Owen Ch. 8 (stratification, conditioning) [V].
- Fit: 5/5.

**M14. Sequential stopping / anytime display**
- Method: keep simulating until the Wilson half-width falls below the display precision (or a time budget is hit), and show provisional numbers in a muted style.
- Pitfall: optional stopping biases naive p-values. For display-precision stopping of a point estimate this is minor, but don't present "significance" tests computed this way.
- Fit: 4/5 (UX necessity).

### Inferences
- Bracketing the true optimal value (derivation). Let V^π be the rollout-policy value, V* the optimal non-anticipative value, and V^HO the hindsight (determinized, clairvoyant) value. Then V^π ≤ V* ≤ V^HO. The second inequality holds because any non-anticipative policy yields, per sampled future, a feasible action sequence, and the clairvoyant optimum can only do better (E[max] ≥ max E). The Bjarnason et al. title ("Lower bounding...") reflects the first inequality. Showing both bounds ("between 41% and 58% depending on play quality") is honest and pedagogically rich.
- Display rule of thumb for a live UI: round to 5 pp unless the Wilson half-width is under 2.5 pp. Keep seeds fixed per game state so the same state always shows the same number.

### Gaps
- Unverified references to confirm:
  - Asmussen, S., & Glynn, P. W. (2007). *Stochastic Simulation: Algorithms and Analysis*. Springer.
  - Glasserman, P. (2004). *Monte Carlo Methods in Financial Engineering*. Springer. CRN and control variates.
  - Wilson, E. B. (1927). Probable inference, the law of succession, and statistical inference. *JASA*, 22(158), 209–212.
  - Agresti, A., & Coull, B. A. (1998). Approximate is better than "exact" for interval estimation of binomial proportions. *The American Statistician*, 52(2), 119–126.
- No benchmark exists for SI rollout speed. Whether 2,400 rollouts fit in an interactive budget depends on the engine and must be measured.

---

## 5. Decision theory under hidden information: EV vs risk, MDP/POMDP, expectimax, determinization (PIMC) and its pitfalls, ISMCTS, MCTS

### Takeaway
Because the Invaders are a scripted AI with no private information, and the only hidden information is exchangeable deck order revealed on a schedule, SI is effectively a single-agent (team) stochastic planning problem against nature. It is not an imperfect-information game against an adversary. Consequences:
- The belief state is trivial (the unseen-card sets).
- Frank & Basin's non-locality error largely does not arise.
- Strategy fusion does arise wherever a decision precedes a card reveal.

The best-fitting decision model is expectimax or MCTS with chance nodes at REVEAL time (the approach Bjarnason et al. used for Klondike), with Monte Carlo leaf evaluation. Plain determinization (PIMC) is easy but optimistic; it gives the upper bound in the bracketing above.

### Cited Findings
- Long, J. R., Sturtevant, N. R., Buro, M., & Furtak, T. (2010). Understanding the success of perfect information Monte Carlo sampling in game tree search. *Proc. AAAI-10*, 134–140. DOI 10.1609/aaai.v24i1.7562. — [AAAI OJS](https://ojs.aaai.org/index.php/AAAI/article/view/7562) [V]; [PDF](https://cdn.aaai.org/ojs/7562/7562-13-11092-1-2-20201228.pdf) [V, text read]
  - Strategy fusion: PIMC "(incorrectly) believes it can use a different strategy in each world, whereas in reality there are situations (or information sets) which consist of multiple perfect information scenarios." Two conditions are needed for it to cause an error: anti-correlated values in one part of the tree, and a move guaranteed to be better elsewhere. [V]
  - Non-locality: in imperfect-information games "a node's value may depend on other regions of the game tree not contained within its subtree, primarily due to the opponent's ability to direct the play towards regions of the tree that he knows (or at least guesses) are favorable for him, using private information." [V]
  - Three measurable properties predict PIMC quality: leaf correlation (probability all sibling terminal nodes share a payoff; low means late decisions still matter), bias (tendency of the game to favor one player), and disambiguation factor (how fast information sets shrink with depth). [V]
  - Findings: PIMC is at its worst when leaf correlation is low. Its performance "improves drastically" as disambiguation approaches 1, because the game approaches perfect information. With disambiguation in the 0.7–0.9 range, low correlation actually helps PIMC. [V]
- Frank, I., & Basin, D. (1998). Search in games with incomplete information: A case study using Bridge card play. *Artificial Intelligence*, 100(1–2), 87–123. This is the origin of the strategy-fusion and non-locality critique. — [Wikipedia ref + Ian Frank page](https://chessprogramming.org/Ian_Frank) [V]; [Cazenave & Ventos, arXiv 1911.07960](https://arxiv.org/pdf/1911.07960) [V]; full volume/issue/pages confirmed in the [Cowling et al. reference list](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf) [V]
- Cowling, P. I., Powley, E. J., & Whitehouse, D. (2012). Information Set Monte Carlo Tree Search. *IEEE Transactions on Computational Intelligence and AI in Games*, 4(2), 120–143. DOI 10.1109/TCIAIG.2012.2200894. — [White Rose eprint + PDF](https://eprints.whiterose.ac.uk/75048/) [V; vol. 4(2) is from prior knowledge, while pages and DOI are V]
  - ISMCTS searches "trees of information sets" instead of trees of game states. [V]
  - Russell & Norvig call determinization "averaging over clairvoyance." Determinization "will never choose to make an information gathering play" or an information-hiding play. [V, quoted in the Cowling et al. text]
  - SO-ISMCTS addresses strategy fusion by searching a single tree of information sets. MO-ISMCTS addresses strategy fusion from partially observable opponent moves. [V]
  - Bjarnason et al. treat Klondike's face-down cards "as chance events at the moment they are revealed. This works for single-player games where the hidden information does not influence the game until it is revealed, but generally does not work for multiplayer games." HOP-UCT is equivalent to determinization (hindsight optimization). [V]
  - Determinization (also called PIMC) is the state of the art for Bridge (Ginsberg's GIB) and Skat. [V]
- Browne, C. B., Powley, E., Whitehouse, D., Lucas, S. M., Cowling, P. I., Rohlfshagen, P., Tavener, S., Perez, D., Samothrakis, S., & Colton, S. (2012). A survey of Monte Carlo tree search methods. *IEEE TCIAIG*, 4(1), 1–43. DOI 10.1109/TCIAIG.2012.2186810. The paper has ten authors. — [Maastricht CRIS](https://cris.maastrichtuniversity.nl/en/publications/a-survey-of-monte-carlo-tree-search-methods) [V]; [PDF mirror](https://turing.iimas.unam.mx/~luis/cursos/IA2022-1/lecturas/SurveyMCTS.pdf) [V]
- Kocsis, L., & Szepesvári, C. (2006). Bandit based Monte-Carlo planning. *Proc. ECML 2006*, 282–293 (UCT). — [Cowling et al. reference list](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf) [V via secondary citation]
- Ginsberg, M. L. (2001). GIB: Imperfect information in a computationally challenging game. *Journal of Artificial Intelligence Research*, 14, 303–358. — [Cowling et al. reference list](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf) [V via secondary citation]
- Yoon, S., Fern, A., Givan, R., & Kambhampati, S. (2008). Probabilistic planning via determinization in hindsight. *Proc. AAAI 2008*, 1010–1016. Hindsight optimization samples futures, solves each deterministic problem, and analyzes when the approach is guaranteed to work and where it fails. — [ML Anthology](https://mlanthology.org/aaai/2008/yoon2008aaai-probabilistic) [V]. Origin: Chong, E. K. P., Givan, R. L., & Chang, H. S. (2000). A framework for simulation-based network control via hindsight optimization. *Proc. IEEE CDC*, 1433–1438. — [Cowling et al. reference list](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf) [V via secondary citation]
- Kaelbling, L. P., Littman, M. L., & Cassandra, A. R. (1998). Planning and acting in partially observable stochastic domains. *Artificial Intelligence*, 101, 99–134. This is the standard paper introducing POMDPs to AI; it won the AIJ Classic Paper Award announced at IJCAI 2021. — [Publication record](https://apice.unibo.it/xwiki/bin/view/Publication/pomdpplanning) [V]; [PDF](https://people.smp.uq.edu.au/YoniNazarathy/Control4406_2014/resources/KaelblingLittmanCassandra1998.pdf) [V]; [Brown award note](https://awards.cs.brown.edu/2021/08/23/michael-littman-and-collaborators-win-ijcais-classic-aij-paper-award/) [V]
- Silver, D., & Veness, J. (2010). Monte-Carlo planning in large POMDPs. *Advances in Neural Information Processing Systems 23*, 2164–2172 (POMCP). It combines a Monte Carlo belief update with MCTS from the current belief, needs only a black-box simulator, and scales to about 10¹⁸ (Battleship) and 10⁵⁶ (PO-PacMan) states. — [NeurIPS](https://neurips.cc/virtual/2010/poster/2145) [V]; [MIT DSpace](https://dspace.mit.edu/handle/1721.1/100395) [V]
- Rockafellar, R. T., & Uryasev, S. (2000). Optimization of conditional value-at-risk. *Journal of Risk*, 2(3), 21–41. Introduces CVaR (mean excess loss) optimization via linear/nonsmooth programming. — [Risk.net](https://risk.net/journal-risk/2161159/optimization-conditional-value-risk) [V]; [IDEAS](https://ideas.repec.org/a/rsk/journ4/2161159.html) [V]. Conflict: [CAS abstract](https://casact.org/abstract/optimization-conditional-value-risk) lists pp. 21–42, and one index lists a different volume/issue. The majority citation is vol. 2, issue 3, pp. 21–41.
- Cooperative board-game AI precedents:
  - Sfikas, K., & Liapis, A. "Collaborative Agent Gameplay in the Pandemic Board Game" (FDG 2020; arXiv 2103.11388). The agent controls all players, weighs the chance of winning against the risk of losing, and uses a Rolling Horizon Evolutionary Algorithm on a game-state abstraction that reduces branching and simulates stochasticity. — [arXiv](https://arxiv.org/abs/2103.11388v1) [V]
  - Sfikas, K., & Liapis, A. (2021). Playing against the board: Rolling horizon evolutionary algorithms against Pandemic. *IEEE Transactions on Games*. DOI 10.1109/TG.2021.3069766. Collaborative games require balancing short-term risk mitigation against long-term winning strategy. — [arXiv 2103.15090](https://arxiv.org/abs/2103.15090v1) [V]; code: [Pandemic-AI-Framework](https://github.com/konsfik/Pandemic-AI-Framework) [V]
  - Sauma-Chacón, P., & Eger, M. (2020). PAIndemic: A planning agent for Pandemic. *Proc. AAAI Conf. on AI and Interactive Digital Entertainment (AIIDE)*, 16(1), 287–293. DOI 10.1609/aiide.v16i1.7443. State-space planning with a domain heuristic plus Monte Carlo sampling for hidden information; it won about one third of games in a realistic setup. — [AAAI OJS](https://ojs.aaai.org/index.php/AIIDE/article/view/7443) [V]
  - Chacón, P. S., & Eger, M. "Pandemic as a Challenge for Human-AI Cooperation" (2020, listed as an AIIDE-associated paper). — [UCR CITIC listing](https://citic.ucr.ac.cr/node/1004) [V for existence only; no abstract retrieved]
  - Bard, N., Foerster, J. N., et al. (2020). The Hanabi challenge: A new frontier for AI research. *Artificial Intelligence*, 280, 103216. DOI 10.1016/j.artint.2019.103216; arXiv 1902.00506. A purely cooperative 2–5-player imperfect-information game that foregrounds reasoning about other agents' beliefs and intentions. — [Google Research](https://research.google/pubs/the-hanabi-challenge-a-new-frontier-for-ai-research/) [V]; [arXiv](https://arxiv.org/abs/1902.00506) [V]
- Spirit Island: two searches (general web and arXiv-targeted) found no academic AI or decision-model work on Spirit Island. — [search 1 results incl. Wikipedia](https://en.wikipedia.org/wiki/Spirit_Island_(board_game)) [V negative]; arXiv-targeted search returned only general game-AI papers [V negative]

### Model cards

**M15. Expected-value (win-probability) maximization over candidate plays**
- Question: "Which of these plays maximizes P(win)?" With binary terminal utility, EV equals P(win).
- Method: evaluate each candidate with M11–M13 and pick the max. Compare with CRN (M12).
- Assumptions: a fixed rollout policy after the decision (one-step lookahead / "policy rollout"). Value depends on that policy.
- Cost: k candidates × n rollouts.
- Presenting: a ranked list with paired differences and intervals. Explicitly flag "within noise" ties as ties.
- Pitfalls: using intermediate proxies (blight count, fear) as if they were utility; ignoring the rollout-policy bias.
- Refs: Sutton & Barto (2018) on rollout/policy evaluation [U]; Puterman (1994) [U].
- Fit: 5/5 as the core advisor primitive. The user's anti-alpha constraint suggests showing trade-offs, not "the answer."

**M16. Risk measures: probability of ruin within horizon, tail risk (CVaR) on proxies**
- Question: "Same average outcome, but which play risks a cascade?"
- Formulas: P(ruin by round t) = CIF_blight(t) (M10). For a loss proxy L (e.g., blight added next round), VaR_α = the α-quantile of L, and CVaR_α = E[L | L ≥ VaR_α] (for continuous L; for discrete L use the Rockafellar–Uryasev minimization formula).
- Note (inference): when terminal utility is binary, maximizing P(win) is already the fully risk-correct objective. Risk measures matter at the PROXY level (one-round lookahead with heuristic leaves), where mean-only proxies hide tail cascades.
- Cost: free, given the simulation samples.
- Presenting: show the distribution (dot plot of next-round blight across sampled futures), not just the mean: "usually 1 blight; 1 in 8 futures, 3 blight with a cascade."
- Pitfalls: optimizing a proxy's mean; reporting VaR of a discrete variable without saying how ties are handled.
- Refs: Rockafellar & Uryasev (2000) [V]; Artzner, Delbaen, Eber & Heath (1999) [U].
- Fit: 3/5. Valuable for teaching risk literacy; secondary to P(win).

**M17. Finite-horizon MDP / expectimax with chance nodes at reveal time (recommended decision model)**
- Question: "Planning a round or two ahead, which sequence of decisions is best given that I'll SEE the next invader card before some later decisions?"
- Model: decision nodes for player choices; chance nodes exactly where a card is revealed, branching over U with weights from Sec. 1 (uniform by exchangeability). Leaves are evaluated by Monte Carlo rollouts or a heuristic value. Backward induction: V(decision) = max over children; V(chance) = Σ p·V(child).
- Why it fits SI (inference): there are no adversary min nodes (the Invaders are scripted), the hidden information belongs to nature and is exchangeable, and the reveal schedule is known. Because the chance node sits AFTER the decisions that precede the reveal, the planner cannot tailor those earlier decisions to the card. Strategy fusion is avoided by construction. This is exactly the condition Bjarnason et al. exploited (hidden information doesn't influence play until revealed).
- Assumptions: an exact rule engine; the correct reveal timing per phase (verify vs rulebook); a manageable branching factor (prune to sensible candidate plays).
- Cost: exponential in depth. Depth 1–2 rounds with pruning and sampled chance nodes (sparse sampling) is feasible; full-game depth is not.
- Presenting: a decision tree "fan": "if you do A now, then whichever card comes you have a good answer; B is great vs Jungle but bad vs Wetland." This directly builds deliberate-play skill.
- Pitfalls: placing chance nodes at the start (that is determinization, M19); a heuristic leaf evaluator dominating the result at shallow depth.
- Refs: Puterman (1994) [U]; Sutton & Barto (2018) [U]; Russell & Norvig AIMA, expectiminimax [U]; Ballard (1983) *-minimax [U].
- Fit: 4/5. The best-founded model; the 1-point deduction is cost.

**M18. POMDP / belief-state formulation**
- Question: the formal frame for "acting optimally with hidden decks."
- Assumptions: a known transition and observation model.
- Inference: by M4, the SI belief is a uniform distribution over orders of the unseen sets, so belief tracking is exact and trivial. The POMDP is equivalent to an MDP on the "information state" (seen cards + public board). General POMDP solvers (point-based value iteration, POMCP) add little beyond M17. Their value is conceptual (teaching "act on your information state, not the true state"), or for any future content that adds non-exchangeable hidden information.
- Refs: Kaelbling, Littman & Cassandra (1998) [V]; Silver & Veness (2010) [V].
- Fit: 2/5 as an algorithm, 4/5 as a conceptual frame.

**M19. Determinization / PIMC / hindsight optimization**
- Question: "Averaging over sampled full deck orders, which play does best if I could plan perfectly within each world?"
- Method: sample complete orders of all unseen cards consistent with observations, solve each as perfect information, and average the per-action values.
- Assumptions: averaging per-world optima approximates the value of acting under uncertainty. That holds only when information is revealed before it matters or when leaf correlation is high (Long et al.).
- Cost: (#worlds) × (perfect-information solve). Often cheaper and simpler to code than M17.
- Pitfalls:
  - Strategy fusion. Within each sampled world the solver "knows" future cards, so it overvalues flexible "wait and see" lines whose payoff depends on information the player won't have at decision time. This produces an optimistic value: the upper bound V^HO in Sec. 4.
  - It never values information-gathering plays. Any SI effect that reveals or reorders deck cards (verify which exist) would be undervalued.
  - Non-locality: per Long et al., this stems from an OPPONENT with private information steering play. SI's scripted Invaders have none, so non-locality should be largely absent (inference).
- Presenting: as "best case if you could see the future," explicitly labeled as optimistic. It pairs naturally with M11's pessimistic rollout value.
- Refs: Long et al. (2010) [V]; Frank & Basin (1998) [V]; Ginsberg (2001) [V-secondary]; Yoon et al. (2008) [V].
- Fit: 3/5. Useful as an upper bound and a quick baseline; not a primary advisor.

**M20. Information Set MCTS (SO-ISMCTS)**
- Question: same as M17, anytime and scalable to deeper search.
- Inference: for a single team against nature with exchangeable hidden decks, SO-ISMCTS (one tree over information sets, re-determinizing each iteration) behaves like MCTS with chance nodes at reveal time. MO-ISMCTS (for partially observable opponent moves) is unnecessary because the Invaders' moves are public.
- Cost: anytime; quality grows with iterations; needs a fast simulator.
- Pitfalls: heavy engineering; the tree statistics are hard to explain to a learner; it inherits rollout-policy bias at leaves.
- Refs: Cowling, Powley & Whitehouse (2012) [V]; Browne et al. (2012) [V]; Kocsis & Szepesvári (2006) [V-secondary].
- Fit: 3/5. Correct, but more machinery than SI's structure needs for 1–2-round advice. Consider it for a later "strong bot" for rollouts.

**M21. MCTS/UCT with explicit chance nodes**
- Same niche as M20. Simpler to explain: UCT at decision nodes, sampled or enumerated chance nodes at reveals.
- Refs: Browne et al. (2012) [V]; Kocsis & Szepesvári (2006) [V-secondary].
- Fit: 3/5.

**M22. POMCP**
- Refs: Silver & Veness (2010) [V]. Fit: 2/5. Its belief-particle machinery solves a problem SI does not have (Sec. 1 makes belief exact).

**M23. Rolling Horizon Evolutionary Algorithm (Pandemic precedent)**
- Question: "Find a good multi-player action sequence for the next few turns by evolving sequences evaluated in simulation."
- Fit (inference): Pandemic is the closest published structural analog (cooperative, deck-driven threat, risk vs. long-term goal). RHEA is open-loop, though: it commits to action sequences across stochastic events, so it has a strategy-fusion-like weakness when reveals occur within the horizon.
- Refs: Sfikas & Liapis (2020, 2021) [V]; Sauma-Chacón & Eger (2020) [V].
- Fit: 2/5 for a teaching companion. Its outputs are opaque to learners.

### Inferences
- SI's place in Long et al.'s parameter space (inference, unmeasured):
  - Hidden information is revealed one card per round on a known schedule, so disambiguation is moderate and complete by the end of each stage.
  - There is no opponent with private information, so non-locality is largely absent.
  - Leaf correlation is plausibly moderate-to-high late in a game ("decided" positions). Long et al.'s method of measuring these properties from random playouts could be applied to an SI engine to check this empirically.
- Hanabi is NOT a structural analog. Its hard part is hidden information BETWEEN teammates and theory of mind. In SI the only hidden information is nature's decks (assuming open table talk among players; verify vs rulebook), so a single "team agent" model is appropriate.
- Recommended architecture: M4 belief → M13 stratified next-card enumeration → M17 depth-1 expectimax over a pruned candidate set → M11/M12 CRN rollouts at the leaves → present via M10 race chart plus the M15 ranked comparison, with the M19 upper bound and the rollout lower bound bracketing the true value.

### Gaps
- Unverified references to confirm:
  - Puterman, M. L. (1994). *Markov Decision Processes: Discrete Stochastic Dynamic Programming*. Wiley (Wiley-Interscience paperback 2005).
  - Sutton, R. S., & Barto, A. G. (2018). *Reinforcement Learning: An Introduction* (2nd ed.). MIT Press. Free: http://incompleteideas.net/book/the-book-2nd.html
  - Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). Pearson. Expectiminimax; the "averaging over clairvoyance" critique appears in earlier editions too.
  - Ballard, B. W. (1983). The *-minimax search procedure for trees containing chance nodes. *Artificial Intelligence*, 21(3), 327–350.
  - Artzner, P., Delbaen, F., Eber, J.-M., & Heath, D. (1999). Coherent measures of risk. *Mathematical Finance*, 9(3), 203–228.
  - Kearns, M., Mansour, Y., & Ng, A. Y. (2002). A sparse sampling algorithm for near-optimal planning in large Markov decision processes. *Machine Learning*, 49, 193–208.
  - Brown, D. B., Smith, J. E., & Sun, P. (2010). Information relaxations and duality in stochastic dynamic programs. *Operations Research*, 58(4), 785–801. Formalizes the "hindsight value ≥ optimal value" upper bound.
- Cowling et al. 2012 volume/issue "4(2)" is from prior knowledge. The repository confirmed only pages 120–143 and the DOI.
- No published SI decision-theory or AI work was found. Long et al.'s properties (leaf correlation etc.) have never been measured for SI.

---

## 6. Graph / spatial models: diffusion, percolation, cellular automata for blight cascade and invader spread

### Takeaway
On SI's small adjacency graph with rule-driven, card-triggered dynamics, the best "spatial model" is exact: run the deterministic rule engine for each possible next card and weight by card probability. The result is a per-land threat heatmap that is both exact and explainable. Network-epidemic, percolation, sandpile, and threshold-cascade theory is valuable as metaphor and intuition ("loaded clusters," "sources of spread"). Its asymptotic results (thresholds, critical exponents) do not transfer to a few dozen nodes with deterministic rules.

### Cited Findings
- Pastor-Satorras, R., Castellano, C., Van Mieghem, P., & Vespignani, A. (2015). Epidemic processes in complex networks. *Reviews of Modern Physics*, 87, 925. arXiv 1408.2701. A review of epidemic spreading on heterogeneous networks, covering the main analytical models and generalized social contagion. — [Network Science Institute](https://www.networkscienceinstitute.org/publications/rev-mod-phys-87-925-2015) [V]; [arXiv](https://arxiv.org/abs/1408.2701?context=cs) [V]

### Model cards

**M24. Exact per-land threat heatmap (deterministic rules × card distribution)**
- Question: "Which lands will be explored, built, or ravaged in the next k rounds, and with what probability?"
- Method: for each ordered sequence of the next k unseen invader cards (by M4, all orders within a stage are equally likely; across stages independent), run the deterministic invader steps on the current board and record per-land events. Weight each sequence by its probability. For k = 1 there are only |U| cases; k = 2 has |U|(|U|−1) within a stage. This is tiny.
- Cards already face-up contribute probability 0 or 1. Only face-down cards add uncertainty, so the map should visually separate "certain" from "possible."
- Assumptions: no player action is modeled in between (an "if you do nothing" map). Optionally re-run it after a candidate play to show the delta.
- Cost: closed form by enumeration. Milliseconds.
- Presenting: a heatmap overlay with exact fractions ("explored in 2 of 3 possible next cards"). Toggle "before / after my planned play." The single most useful spatial teaching display.
- Pitfalls: forgetting that explore sources change after build steps within the same round (order of invader steps matters; verify vs rulebook); mixing certain and probabilistic information in one color scale.
- Refs: Sec. 1 references (Grinstead & Snell [V]; Diaconis & Freedman [V]). No spatial-theory reference is needed because this is exact computation.
- Fit: 5/5.

**M25. Blight cascade as a threshold / sandpile cascade (analogy)**
- Question: "How far can one blight placement cascade?"
- Analogy: a land already holding blight is "loaded." Adding blight there topples blight to an adjacent land (given structure). Chains propagate through connected loaded lands, like toppling in a sandpile model or activation in a threshold-cascade model. Upper bound (inference): the cascade from land L can only continue through lands that are loaded at the time of arrival. So the connected component of loaded lands containing L, plus its boundary, bounds the reach, under the assumption that each step goes to one adjacent land (verify the exact cascade rule and who chooses the destination vs rulebook).
- Cost: graph search (BFS) on the adjacency graph. Trivial.
- Presenting: highlight "loaded clusters" and their boundaries. Teach that "cleansing one land in the middle of a cluster cuts the cascade path."
- Pitfalls: importing self-organized-criticality power laws to a tiny finite board.
- Refs: Bak, Tang & Wiesenfeld (1987) [U]; Watts (2002) [U]; Granovetter (1978) [U].
- Fit: 3/5 as an explanation layer on top of exact computation. The cluster/boundary concept is actionable.

**M26. Network contagion / contact process for invader spread (analogy)**
- Analogy: given structure, explore spreads invaders into lands adjacent to towns, cities, or ocean, like an SI-type ("susceptible-infected") contagion on a graph where "infected" lands seed neighbors. Player removals act like recovery or immunization.
- Useful concepts: "sources" (towns/cities) vs. "susceptible frontier"; immunization targeting (removing high-degree sources has outsized effect).
- Pitfalls: mean-field epidemic thresholds assume large random graphs and stochastic per-edge transmission. SI's spread is deterministic given the card and gated by terrain, so the "transmission probability" is really the card-selection probability. Exactness (M24) beats analogy.
- Refs: Pastor-Satorras et al. (2015) [V]; Newman, *Networks* (2nd ed., 2018, OUP) [U].
- Fit: 2/5.

**M27. Percolation / cluster view**
- Use: the language of "spanning clusters" for "is the whole board connected through blighted/loaded lands?"
- Pitfall: percolation thresholds are infinite-lattice limits. They are meaningless at SI scale beyond metaphor.
- Refs: Stauffer & Aharony (1994) [U]; Grimmett (1999) [U].
- Fit: 1/5.

**M28. Influence maximization / defense-set selection (analogy for "which lands to protect")**
- Analogy: choosing k lands to defend to minimize expected spread resembles influence minimization or immunization. Greedy methods are near-optimal when the objective is submodular (Kempe, Kleinberg & Tardos 2003 [U]).
- Inference: on SI-sized graphs, exhaustive evaluation of small defense sets with M24 + M11 is feasible, so approximation guarantees are unnecessary. Teach the idea of diminishing returns, not the algorithm.
- Fit: 2/5.

### Inferences
- The SI board graph is small. The given structure says lands on boards with adjacency; the per-board land count is not given (verify vs rulebook). So every spatial question at 1–2-round depth can be answered exactly by enumeration plus the rule engine. Spatial-process theory should be used only to name and explain patterns (clusters, frontiers, sources), never to compute.

### Gaps
- Unverified references to confirm:
  - Bak, P., Tang, C., & Wiesenfeld, K. (1987). Self-organized criticality: An explanation of 1/f noise. *Physical Review Letters*, 59(4), 381–384.
  - Watts, D. J. (2002). A simple model of global cascades on random networks. *PNAS*, 99(9), 5766–5771.
  - Granovetter, M. (1978). Threshold models of collective behavior. *American Journal of Sociology*, 83(6), 1420–1443.
  - Kempe, D., Kleinberg, J., & Tardos, É. (2003). Maximizing the spread of influence through a social network. *Proc. ACM KDD*, 137–146.
  - Stauffer, D., & Aharony, A. (1994). *Introduction to Percolation Theory* (2nd ed.). Taylor & Francis.
  - Grimmett, G. (1999). *Percolation* (2nd ed.). Springer.
  - Newman, M. E. J. (2018). *Networks* (2nd ed.). Oxford University Press.
- No published spatial analysis of SI boards was found.

---

## 7. Learning and calibration: proper scoring rules, calibration plots, cognitive biases

### Takeaway
The companion has an unusual teaching advantage: it knows the EXACT probability p* for most card questions. A learner's forecast q can therefore be scored against p* directly using the expected proper score. This removes outcome noise and gives immediate, fair feedback, which is far faster calibration training than outcome-based Brier scores. Bias drills should target the conjunction fallacy (multi-step plans) and law-of-small-numbers effects. They should also teach the nuance that, without replacement, "it's due" reasoning is partly CORRECT, unlike for dice.

### Cited Findings
- Gneiting, T., & Raftery, A. E. (2007). Strictly proper scoring rules, prediction, and estimation. *Journal of the American Statistical Association*, 102(477), 359–378. DOI 10.1198/016214506000001437. It reviews proper scoring rules (logarithmic, quadratic/Brier, spherical, CRPS), includes Savage's representation and the links to Bregman divergences and entropy, and covers interval scores. It uses a positively oriented convention (higher = better), opposite to the common loss convention. — [UW tech report version (DTIC)](https://apps.dtic.mil/sti/html/tr/ADA459827/index.html) [V]; [Edinburgh copy](https://webhomes.maths.ed.ac.uk/~flindgre/scores/PredictionScores.pdf) [V]
- Tversky, A., & Kahneman, D. (1983). Extensional versus intuitive reasoning: The conjunction fallacy in probability judgment. *Psychological Review*, 90(4), 293–315. DOI 10.1037/0033-295X.90.4.293. The conjunction rule is P(A&B) ≤ P(A), P(B). Representativeness and availability can make a conjunction seem more probable, and violations appear in lay people and experts. More than 80% failed in the Linda problem. Hertwig & Gigerenzer dispute the interpretation (pragmatic inference). — [Full text PDF (UCSD)](https://pages.ucsd.edu/~mckenzie/TverskyKahneman1983PsychRev.pdf) [V]; [Frontiers commentary](https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2015.01832/full) [V]
- Miller, J. B., & Sanjurjo, A. (2018). Surprised by the hot hand fallacy? A truth in the law of small numbers. *Econometrica*, 86(6), 2019–2047. A common streak-based measure is biased in finite sequences. The bias shrinks with sequence length but grows with streak length. Correcting it reverses the canonical hot-hand study's conclusion. The working-paper title was "Surprised by the Gambler's and Hot Hand Fallacies?" — [Econometric Society](https://econometricsociety.org/publications/econometrica/2018/11/01/surprised-hot-hand-fallacy-truth-law-small-numbers) [V]; [SSRN](https://papers.ssrn.com/abstract=2627354) [V]

### Model cards

**M29. Proper scoring of learner forecasts against the exact probability (expected-score feedback)**
- Question: "How good is my probability intuition?"
- Method (derivation): the learner states q for an event whose exact probability is p* (from Sec. 1 or M24).
  - Expected Brier score under p*: E[(q − Y)²] = (q − p*)² + p*(1 − p*). The first term is the learner's excess loss; the second is irreducible.
  - Expected log-score regret: KL(p* ‖ q) = p* log(p*/q) + (1 − p*) log((1 − p*)/(1 − q)).
  - Score the excess term immediately. Optionally also score realized outcomes so learners experience variance.
- Assumptions: the "exact" p* is correct (engine plus card model). For simulation-based p̂, include its Monte Carlo error.
- Cost: closed form.
- Presenting: "You said 50%; it was 33%." Show a running calibration score per question type (deck draws, cascades, races). Prefer the Brier score for learners (bounded, intuitive). Use the log score to punish overconfident 0%/100% calls.
- Pitfalls: scoring only on outcomes (slow, noisy, rewards luck: "resulting"); letting learners answer 0% or 100% under the log score (infinite penalty). Clamp the inputs, or explain why the penalty is infinite.
- Refs: Gneiting & Raftery (2007) [V]; Brier (1950) [U].
- Fit: 5/5 for the learning layer.

**M30. Calibration (reliability) diagrams with Murphy decomposition**
- Question: "Across many forecasts, when I say 70%, does it happen 70% of the time?"
- Method: bin the learner's q values and plot the observed frequency (or mean p*) per bin against bin mean q. Murphy decomposition: Brier = reliability − resolution + uncertainty.
- Pitfall: too few forecasts per bin. Use wide bins or isotonic smoothing, and show bin counts.
- Refs: Murphy (1973) [U]; DeGroot & Fienberg (1983) [U].
- Fit: 4/5. Slower feedback than M29; good for weekly review.

**M31. Bias-targeted drills**
- Conjunction fallacy: multi-step plans ("next card is X AND I draw a usable minor AND the cascade goes my way"). Make learners multiply, and note that without replacement the factors are NOT independent; condition step by step. Ref: Tversky & Kahneman (1983) [V].
- Law of small numbers / gambler's fallacy: in SI decks, seeing a terrain card DOES lower the chance of another of the same terrain in that stage (negative dependence, M2), so "it's due" is partly valid for cards. For independent events it is a fallacy. Drill the distinction explicitly. Refs: Tversky & Kahneman (1971) [U]; Miller & Sanjurjo (2018) [V] (finite-sequence subtleties).
- Base-rate neglect: when a vivid board situation suggests danger, ask "what fraction of the unseen cards actually trigger it?" Refs: Kahneman & Tversky (1973) [U]; Bar-Hillel (1980) [U].
- Outcome bias ("resulting") in post-game review: evaluate decisions by their probability at the time, using the logged state and M11/M13, not by what happened. Ref: Baron & Hershey (1988) [U].
- Overconfidence: Lichtenstein, Fischhoff & Phillips (1982) [U].
- Fit: 4/5.

### Inferences
- Expected-score feedback (M29) turns every in-game card question into a calibration trial with zero outcome noise. That is a structural advantage of a rules-aware companion over generic forecasting training. It also fits the user's "deliberate-play muscle" goal: predict first, then reveal.
- For a statistician audience: SI's within-stage dependence makes it a clean, tangible demonstration that exchangeable ≠ independent. The Diaconis–Freedman k/n bound (Sec. 1) quantifies how far it is from i.i.d.

### Gaps
- Unverified references to confirm:
  - Brier, G. W. (1950). Verification of forecasts expressed in terms of probability. *Monthly Weather Review*, 78(1), 1–3.
  - Murphy, A. H. (1973). A new vector partition of the probability score. *Journal of Applied Meteorology*, 12(4), 595–600.
  - DeGroot, M. H., & Fienberg, S. E. (1983). The comparison and evaluation of forecasters. *The Statistician*, 32(1/2), 12–22.
  - Tversky, A., & Kahneman, D. (1971). Belief in the law of small numbers. *Psychological Bulletin*, 76(2), 105–110.
  - Tversky, A., & Kahneman, D. (1974). Judgment under uncertainty: Heuristics and biases. *Science*, 185(4157), 1124–1131.
  - Kahneman, D., & Tversky, A. (1973). On the psychology of prediction. *Psychological Review*, 80(4), 237–251.
  - Bar-Hillel, M. (1980). The base-rate fallacy in probability judgments. *Acta Psychologica*, 44(3), 211–233.
  - Lichtenstein, S., Fischhoff, B., & Phillips, L. D. (1982). Calibration of probabilities: The state of the art to 1980. In D. Kahneman, P. Slovic, & A. Tversky (Eds.), *Judgment under Uncertainty: Heuristics and Biases* (pp. 306–334). Cambridge University Press.
  - Baron, J., & Hershey, J. C. (1988). Outcome bias in decision evaluation. *Journal of Personality and Social Psychology*, 54(4), 569–579.
- I found no studies of calibration training specifically for board-game players. The effectiveness of M29-style feedback is an inference from forecasting literature, not a measured result.

---

## 8. Game-design math references (books)

### Takeaway
For the companion's reference shelf:
- Grinstead & Snell (free) covers hypergeometric, branching, absorbing chains, first passage, and gambler's ruin in one place, and is the best single free text.
- Tijms (3rd ed.) and Haigh are the most learner-friendly.
- Epstein is the deepest on games of chance.
- Elias–Garfield–Gutschera is the design-theory frame (luck vs. skill, information).
- Beasley is a compact classic.
- Kemeny & Snell, Puterman, and Sutton & Barto are the formal backbones.

### Cited Findings
- Elias, G. S., Garfield, R., & Gutschera, K. R. (2012). *Characteristics of Games*. MIT Press. ISBN 978-0-262-01713-8 (hardcover), 336 pp. Paperback reissue 2020, ISBN 978-0-262-54269-2. Listings conflict on the 2012 date (Aug. 24 vs Sep. 7). — [Tenlong (HC)](https://www.tenlong.com.tw/products/9780262017138) [V]; [MIT Press Bookstore (PB)](https://mitpressbookstore.mit.edu/book/9780262542692) [V]
- Beasley, J. D. (1989). *The Mathematics of Games*. Oxford University Press, Recreations in Mathematics no. 5. 169 pp. ISBN 0198532067. 1990 OUP paperback ISBN 0192861077; a Dover reprint also exists. — [Mathematical Gazette review listing](https://www.cambridge.org/core/journals/mathematical-gazette/article/mathematics-of-games-by-john-d-beasley-pp-169-1495-1989-isbn-0198532067-oxford-university-press/B9A9491F9EBB31A7A0BFD301DE6A2788) [V]; [Sophia Univ. OPAC](https://www.lib.sophia.ac.jp/opac/opac_link/bibid/1000379292) [V]
- Epstein, R. A. *The Theory of Gambling and Statistical Logic*. Edition history conflicts across sources:
  - A library record lists a revised edition, Academic Press, New York, 1995, 450 pp. [V]
  - An Elsevier record dated 2009 exists [V].
  - An MAA review discusses a "second edition" (Academic Press) and quotes the foreword referencing "the revised 1977 edition" [V].
  - Elsevier lists ISBN 978-0-08-057184-3 for the revised edition [V].
  - Recommended citation (flagged): Epstein, R. A. (2009). *The Theory of Gambling and Statistical Logic* (2nd ed.). Academic Press/Elsevier. The 1967 first edition is [U].
  - Sources: [Elsevier shop](https://shop.elsevier.com/books/the-theory-of-gambling-and-statistical-logic-revised-edition/epstein/978-0-08-057184-3) [V]; [MAA review](https://old.maa.org/node/113131) [V]; [UV catalog record](https://ebusca.uv.mx/Record/cern-1320461) [V]
- Grinstead, C. M., & Snell, J. L. (1997). *Introduction to Probability* (2nd rev. ed.). AMS. Free under GNU FDL. Relevant sections: 3.3 Card Shuffling; 5.1 (hypergeometric); 10.2 Branching Processes; 11.2 Absorbing Markov Chains; 11.5 Mean First Passage Time; 12.2 Gambler's Ruin. — [PDF](https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf) [V]
- Tijms, H. (2012). *Understanding Probability* (3rd ed.). Cambridge University Press. ISBN 9781107658561, 562 pp. Earlier editions were subtitled "Chance Rules in Everyday Life." The 3rd ed. adds coincidences, optimal stopping, Kelly betting, and hitting probabilities in random walks. — [CUP front matter PDF](https://assets.cambridge.org/97811076/58561/frontmatter/9781107658561_frontmatter.pdf) [V]; [MAA review](https://old.maa.org/node/112839) [V]
- Haigh, J. (1999). *Taking Chances: Winning with Probability*. Oxford University Press. xiv + 330 pp. 2000 paperback; a 2003 updated edition adds a "Probability for Lawyers" chapter (single retailer source for 2003). David Aldous's review notes coverage of lotteries, dice and card games, casino games, game shows, and racetrack betting. — [Aldous review PDF](https://www.stat.berkeley.edu/~aldous/157/Books/haigh.pdf) [V]; [Lehmanns (2003 ed.)](https://www.lehmanns.de/shop/sachbuch-ratgeber/5384480-9780198526636-taking-chances) [V]; [MAA review](https://old.maa.org/node/108563) [V]
- Kemeny & Snell, *Finite Markov Chains*: cited as the deeper companion to Grinstead & Snell Ch. 11. — [Princeton LIPS](https://lips.cs.princeton.edu/the-fundamental-matrix-of-a-finite-markov-chain) [V mention only]

### Inferences
- Reading order for a learner, by fit to SI questions: Grinstead & Snell Sec. 5.1 and Ch. 11 → Tijms (random walks, hitting probabilities) → Haigh (intuition) → Elias et al. (luck/skill/information as design levers) → Sutton & Barto Chs. 3–4 and 8 (MDPs, planning, rollouts) → Browne et al. and Cowling et al. (search under hidden information).
- For the statistician: Kemeny & Snell (lumpability, fundamental matrix), Neuts (phase-type), Putter et al. (competing risks), Brown–Cai–DasGupta (binomial intervals), and Gneiting–Raftery (scoring).

### Gaps
- Unverified: Kemeny & Snell (1960 Van Nostrand; 1976 Springer UTM) full details; Puterman (1994, Wiley); Sutton & Barto (2018, MIT Press, free online); Feller (1968); Epstein 1st ed. (1967, Academic Press). The Epstein edition conflict is unresolved: cite with "2nd ed." and check the copy at hand.
- I found no game-design book that treats Spirit Island's mathematics specifically.

---

## 9. Overall catalog: fit ratings to Spirit Island's structure (synthesis of the objective)

### Takeaway
The models with the highest fit are those that exploit SI's two structural facts: (1) hidden information is only exchangeable deck order, and (2) rule resolution is deterministic given cards. Those models are exact card combinatorics, exact next-card enumeration through the rule engine, CRN Monte Carlo from the current state, expectimax with chance nodes at reveal time, competing-risks race charts, and expected-score calibration feedback. Classical stochastic-process and spatial-process models (gambler's ruin, branching, epidemics, percolation) are mainly explanatory analogies at SI's scale.

### Cited Findings
- The ratings below synthesize the cited findings in Secs. 1–8. The key supports are:
  - Exchangeability and the hypergeometric distribution ([Grinstead & Snell](https://www.stat.berkeley.edu/users/aldous/134/grinstead.pdf); [Diaconis & Freedman](https://projecteuclid.org/euclid.aop/1176994828)).
  - PIMC's failure modes and their dependence on opponent private information and disambiguation ([Long et al. 2010](https://cdn.aaai.org/ojs/7562/7562-13-11092-1-2-20201228.pdf)).
  - Reveal-time chance nodes being valid when hidden information has no effect until revealed ([Cowling et al. 2012](https://eprints.whiterose.ac.uk/id/eprint/75048/1/CowlingPowleyWhitehouse2012.pdf)).
  - Binomial interval guidance ([Brown, Cai & DasGupta 2001](https://projecteuclid.org/euclid.ss/1009213286)).
  - Variance-reduction toolkit ([Owen](https://artowen.su.domains/mc/)).
  - Competing-risks framing ([Putter et al. 2007](https://www.est.ufmg.br/~enricoc/pdf/sobre/Putter2007.pdf)).
  - Proper scoring ([Gneiting & Raftery 2007](https://apps.dtic.mil/sti/html/tr/ADA459827/index.html)).

### Fit table (5 = build first, 1 = metaphor only)

| # | Model | Question it answers | Closed form vs simulation | Fit | Main reason |
|---|---|---|---|---|---|
| M4 | Exchangeability lemma (removed unseen card) | Is each unseen card equally likely next? | Closed form | 5 | Foundation of the belief model; makes belief exact |
| M1 | Hypergeometric | P(≥1 of type A in n draws) | Closed form | 5 | Exact, instant, teachable |
| M24 | Exact per-land threat heatmap | Where will invaders hit in k rounds? | Exact enumeration | 5 | Deterministic rules × small card set |
| M11 | Monte Carlo from current state | Anything not closed-form | Simulation | 5 | General; needs a rule engine; estimand = rollout policy |
| M12 | Common random numbers | Is play A better than B? | Simulation | 5 | Essential for stable comparisons |
| M13 | Stratify/condition on next card | Lower variance; "if X next" table | Hybrid | 5 | Exact over the small next-card set; most instructive display |
| M15 | EV / P(win) maximization | Which play maximizes P(win)? | Simulation | 5 | Core advisor primitive (present as trade-offs) |
| M29 | Expected-score calibration feedback | How good is my probability sense? | Closed form | 5 | Exact p* known; zero-noise feedback |
| M2 | Multivariate hypergeometric | Joint draws across categories | Closed form | 4 | Overlapping-category bookkeeping |
| M3 | Negative hypergeometric | Draws until X | Closed form | 4 | Good tail/mean pair |
| M10 | Competing-risks CIF race chart | Which clock binds, by when? | From simulation/recursion | 4 | Best multi-clock display; rollout-policy dependent |
| M17 | Expectimax with reveal-time chance nodes | Best 1–2-round plan given future reveals | Search + simulation | 4 | Avoids strategy fusion by construction; cost |
| M30 | Reliability diagrams | Long-run calibration | Closed form | 4 | Needs many forecasts |
| M31 | Bias drills | Conjunction, small numbers, base rates, resulting | n/a | 4 | Targeted to SI decision errors |
| M14 | Sequential stopping/anytime | UX for live numbers | Simulation | 4 | Necessity for live UI |
| M18 | POMDP frame | Formal frame | n/a | 2 (algorithm) / 4 (concept) | Belief is trivial in SI |
| M5 | Absorbing Markov chain (abstracted) | Race structure | Closed form on abstraction | 3 | Abstraction breaks Markov property |
| M6 | Discrete phase-type | Distribution of ending round | Closed form on abstraction | 3 | Same caveat; bridges to survival |
| M9 | First passage / Wald | Rounds to fear threshold | Closed form approx. | 3 | Quick heuristic; non-i.i.d. increments |
| M16 | Ruin probability / CVaR on proxies | Tail risk of a play | From simulation | 3 | P(win) already risk-correct; useful for proxies |
| M19 | Determinization / PIMC / hindsight | Upper bound "if you could see the future" | Simulation + solve | 3 | Strategy fusion → optimistic; good as upper bound |
| M20 | SO-ISMCTS | Deep anytime planning | Simulation | 3 | Correct but heavy; reduces to chance-node MCTS here |
| M21 | MCTS/UCT with chance nodes | Same | Simulation | 3 | Same |
| M25 | Threshold/sandpile cascade view | Cascade reach | Graph search | 3 | Cluster/boundary concept is actionable |
| M7 | Gambler's ruin | Pool depletion intuition | Closed form | 2 | 1-D i.i.d. assumptions badly violated |
| M8 | Multi-type branching | Snowball rate | Closed form | 2 | Per-land, deterministic, spatial dynamics |
| M22 | POMCP | Large-belief planning | Simulation | 2 | Solves a problem SI lacks |
| M23 | RHEA (Pandemic precedent) | Sequence optimization | Simulation | 2 | Open-loop; opaque to learners |
| M26 | Network contagion | Spread intuition | Analogy | 2 | Deterministic, terrain-gated spread |
| M28 | Influence/immunization | Which lands to defend | Analogy | 2 | Exhaustive evaluation feasible instead |
| M27 | Percolation | Spanning clusters | Analogy | 1 | Infinite-lattice limits irrelevant |
| — | Win-rate tables, Elo, outcome-only survival fits | — | — | EXCLUDED | Aggregate win/loss input only (per constraint) |

### Inferences
- Suggested build order:
  1. M4 + M1–M3 (deck panel).
  2. M24 (threat heatmap).
  3. M11 + M12 + M13 with Wilson intervals (comparisons).
  4. M10 race chart.
  5. M29 predict-then-reveal calibration loop.
  6. M17 depth-1 expectimax "fan" for deliberate-play prompts.
  7. Optionally, M19 vs. rollout bracketing to show the range of achievable play.
- "Rules for and against the player" (the user's framing) maps cleanly onto the companion's display. Face-up (known) invader information has probability 0 or 1 and should be shown as certainty: information that works for the player. Face-down cards are the uncertainty that works against the player. Separating these visually is itself a teaching tool.

### Gaps
- None of the fit ratings were empirically validated on an SI engine. They are analytic judgments from the cited structure. Long et al.'s playout-based measurement of leaf correlation, bias, and disambiguation could validate the PIMC rating on an SI engine.
- SI-specific rule details that affect several models were deliberately not researched (out of scope) and must be checked against the physical rulebook: reveal timing within a round, step order within the invader phase, the exact cascade rule and who chooses the destination, power-deck reshuffle, deck-modifying effects, and simultaneous-end-condition precedence.
