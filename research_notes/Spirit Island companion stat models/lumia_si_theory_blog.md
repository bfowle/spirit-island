# Lumia's Spirit Island theory blog (iam.lumia.gay): models, methods, and how they map to a live companion

Scope note: I fetched every post as raw HTML with curl (not a summarizing fetcher), so the quotes below are verbatim, lowercase as the author writes. The author is "lumia" (she/her, MIT math major; see [about](https://iam.lumia.gay/about)). Spirit and card names are the author's shorthand. Her own decodings come from her [glossary](https://iam.lumia.gay/si_glossary). Where I decode a nickname myself, I mark it "(my reading)".

**Headline finding (read this first):** the blog is not an economy-valuation blog in the Rei sense. It has **no** Energy-per-card-play value, no presence-track tempo tables, no element-threshold odds, and no damage-per-Energy or fear-per-Energy tables. The post the user remembered, ["growth in spirit island"](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html), is about **asymptotic growth rates of repeatable effects** (constant, linear, quadratic, n^1.5, exponential, factorial). It is not about the Growth phase. The rest of the theory series consists of mathematician/puzzler work:
- extremal "how much X can you get on turn 1" puzzles;
- constructive turn-1 victory proofs with running fear and Energy ledgers;
- impossibility proofs by resource counting;
- an exhaustive 68 spirit/aspect × 8 adversary feasibility matrix;
- one explicit probability-of-sequence calculation.

The glossary adds the state vocabulary: tempo, action economy, x/y Energy/card plays, b2b, capital, threat, solve, darkest timeline, and more. The glossary also explicitly references Rei's "algo".

---

## Framework: how the blog reasons (core concepts, vocabulary, first principles, links to tempo and momentum)

### Takeaway
The blog reasons like a mathematician:
1. **Classify** how a quantity scales when an effect repeats (constant or linear vs "ramping").
2. **Pose extremal or existence questions** ("how many…", "can X win on turn 1", "can the island win with no spirits").
3. **Prove them constructively**: an ordered action script with running Energy and fear ledgers, checked against a hard budget such as fear needed to reach a Terror Level.
4. **Prove impossibility** by counting resources.
5. **Enumerate** the whole spirit × adversary space.
6. **Attach probabilities** only by multiplying independent draw odds.

The posts never use the words "momentum", "curve", "snowball", "compound" or "economy" (except "action economy" in the glossary). The tempo and economy-over-time concepts live in the [glossary](https://iam.lumia.gay/si_glossary), and the "ramping" idea lives in the growth post. Both connect naturally to Rei-style momentum thinking, but the author does not make that link herself.

### Cited Findings

#### First principle: most things are linear; compounding ("ramping") is the exception worth studying
- "so most things in spirit island happen only once and do the same thing every time it happens. for this reason, most growths in spirit island are either constant or linear. this includes health of invaders in a land (ignoring the ramping from invader stages), or amount of cards gained over time (unless cards gained let you gain more cards). however, some things don't do the same thing every time, and therefore can cause ramping." — [growth in spirit island, 2023-09-03](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)
- "so most of the non-linear growths come from doing things over and over, which is a thing that not all spirits have the ability to do." — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)
- The post orders its examples by growth class: "part 1: quadratic growths", "part 2: n^1.5??", "part 3: exponential growth", "part 4: super-exponential growth". It concludes the Melt Earth recurrence "is a factorial growth speed, which is faster than exponential." — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)

#### Framing a turn: an ordered action tree with running ledgers
- Every constructive post writes the turn in order: growth / "spirit phases" → "fast:" → invader actions (ravage, build, explore, with fear cards and blight-card flips inside) → "slow:". Each step carries a running ledger, for example "(should have 3 energy remaining)", "(+2=9 fear generated)", "(+5=14 fear in total)", "(16 fear in total, hitting tl2)". — [spirit island turn 1 victories, 2023-07-26](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
- Timing and resolution order are treated as first-class. Examples:
  - "this doesn't hit england lc because it's in the middle of an action tree (sweep threshold repeat is a triggered action), and losses are checked at the end of action trees" — [T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - "since there is no sky stretches or exaltation, this thresholded cast down will resolve in slow phase, which means as long as waters can survive 1 invader phase, it will win." — [cast down thresholded on turn 1, 2026-02-13](https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html)

#### Budget arithmetic: fear needed vs fear available
- The win target is a fear budget: "scotland and sweden only have 20 fear cards when combined, which only needs 80 fear total. so this is already a victory at tl4!" and "the fear deck is pretty thicc. it's 15/19/12 which is a total of 46 fear cards, which makes 230 fear to burn through." — [T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
- Land-by-land fear inventory: "4 from haunts and embers / 1 from daunted by the dahan / 4 from east4 … / 4 from east3 … / 1 from the east2 town / for a total of 14. so destroying everything will just win against lower levels of prussia, and for higher levels, you just need to generate 2 extra fear". It also notes "prussia 4 or higher would need 16 fear generated." — [turn 1 victories on east board, 2025-01-12](https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html)
- Near-miss accounting: "this is 1 fear off from getting another fear card to remove the last explorer :dismay:" — [t1 victories against low level adversaries, 2023-11-18](https://iam.lumia.gay/si_theory/2023/11/18/low_level_t1_victories.html)

#### Existence, extremal, and impossibility proofs
- Existence: "naturally, when i see a new spirit, one of my first thoughts is 'can this spirit win turn 1?'" — [t1 victories with bad spirits, 2024-06-30](https://iam.lumia.gay/si_theory/2024/06/30/bad_spirit_t1_victories.html)
- Extremal: "here are some optimizations that don't involve controlling what cards you draw" (max Gift of Proliferation uses, max presence, max Power cards removed, max Energy, max tokens). — [if you don't control rng, part a, 2024-03-01](https://iam.lumia.gay/si_theory/2024/03/01/no_rng_control.html)
- Impossibility by counting: "it's pretty easy to get 3 empowers with fractured+green+mentor, but it isn't possible to get 4. here's the proof: we need 2 non-incarna spirits that can give out 3 prolifs, a card play, and a repeat. the only spirit that can give out multiple prolifs is green, and the only spirit that can increase that to 3 prolifs is fractured. … (fractured can gain at most 3 time, which is at most 3 repeats of prolif)" — [part a](https://iam.lumia.gay/si_theory/2024/03/01/no_rng_control.html)
- Complexity theory: "hardness proofs. where i figure out if spirit island is np-hard. (spoilers: it is, so i'm instead looking at pspace-hardness)" — [oops i haven't posted in two months, 2023-10-30](https://iam.lumia.gay/2023/10/30/october_update.html)

#### Luck as a bound: the "best timeline" and the "darkest timeline"
- "can the island drive off the invaders without any spirits?" The answer is yes, by choosing deck orders (event, fear and blight cards): "terror 1 victory!" — [winning at spirit island... with no spirits?, 2023-12-29](https://iam.lumia.gay/si_theory/2023/12/29/spiritless_island.html)
- Glossary counterpart: "darkest timeline: the worst sequence of invader cards for the current game state" — [glossary](https://iam.lumia.gay/si_glossary)

#### Vocabulary: economy and tempo over time (glossary, verbatim)
- "tempo: where in the explore-build-ravage sequence your actions dealing with a land are happening. ahead tempo means they are happening between explore and build, on tempo means they are happening between build and ravage" — [glossary](https://iam.lumia.gay/si_glossary)
- "action economy: how many actions the spirit is able to cause on a certain turn, generally ones that affect the board or disrupt invader actions" — [glossary](https://iam.lumia.gay/si_glossary)
- "(number)/(number): (first number) energy gain and (second) number card plays — 3/3 is a good place to get to for major-slinging strategies" — [glossary](https://iam.lumia.gay/si_glossary)
- "(number)/(number)/(number) spirit: a spirit that adds (first number) presence on reclaim, (second number) presence using a second growth option, and (third number) presence using a third growth option — earth is a 1/1/1 spirit, river is a 0/2/1 spirit, and shadows is a 0/1/1 spirit" — [glossary](https://iam.lumia.gay/si_glossary)
- "algo: (rei's understanding of) the algorithm that the developers use to balance growth options, presence tracks, and power cards by assigning values to effects, like how chess assigns values to pieces" — [glossary](https://iam.lumia.gay/si_glossary). This is the blog's only explicit Rei reference.
- Economy-over-time decisions:
  - "down turn: a turn where you play fewer cards or are less powerful in order to keep growing"
  - "underplay: play fewer cards than your number of card plays — i will underplay on turn 3 so i can play 4 cards on turn 4 next turn to hit my max innate"
  - "pivot: to switch to a different strategy, often from one focusing on top track and majors to one focusing on bottom track and minors or vice versa"
  - "casino: a strategy that involves constantly gaining power cards and not reclaiming many cards at a time"
  - "reclaim loop: the act of reclaiming for multiple turns in a row, usually playing the same cards each time"
  - "predraft: make a decision (typically selecting a minor power) while considering how it would improve various majors that you could draft later"
  - "yolo major: a major draft that you expect to play this turn or hope to solve some problems on your board"
  - "trap: a choice that looks better than it actually is — river's top track is a trap, rushing bottom track is the most effective strategy"
  - "win condition: a power that, when used several times, will win the game"

  — [glossary](https://iam.lumia.gay/si_glossary)
- Land-state and risk vocabulary:
  - "b2b: back to back, a land that has or is likely to have two consecutive invader actions occurring"
  - "capital: a land that will have 7 buildings during the invader phase"
  - "threat: a land that will cause a loss this turn or come close to doing so"
  - "problem land: a land that is difficult to deal with, or a land that will blight or be a loss condition this or next turn"
  - "solve: prevent a land from blighting the next time it ravages"
  - "take blight to scale: not deal with a land that is adding a blight during a ravage, often in order to dedicate actions to solving other lands"
  - "bpp: blight per player (on the blight card), or a measure of total blight taken during a game"
  - "blocker: a land or invader that is preventing victory"
  - "pocket: a group of inland lands that do not get explored because they have no sources of invaders, or to make such a group exist"
  - "dump land", "fear farm", "first strike", "edge case"

  — [glossary](https://iam.lumia.gay/si_glossary)

### Inferences
- **Momentum = ramping.** Rei-style momentum (resources that buy more resources, so you get ahead of the invader curve and stay there) is exactly the growth post's "ramping" class: "amount of cards gained over time (unless cards gained let you gain more cards)" is linear *unless* it feeds back. The blog gives a math vocabulary for momentum: constant < linear < quadratic < exponential < factorial. Rei gives the valuation side (the "algo"). This pairing is my synthesis; neither source makes it.
- **The invader side is ramped by stages, the spirit side mostly linear.** The growth post brackets "the ramping from invader stages" as the main non-linearity on the invader side. That matches designer darker's point (in the project's existing notes) that the stage step-up "comes at a constant point in time" so the game is not self-balancing. Together they give a "curve" picture: invader pressure is a step function in time, spirit power is linear unless you build a ramp. That is the user's "staying ahead of the curve" ([community_analyses.md](/home/brett/repos/spirit-island-steam-companion/research_notes/Spirit%20Island%20companion%20stat%20models/community_analyses.md), darker quote at line 66).
- **Two separate meanings of "tempo".** The glossary's tempo is **land-level timing** inside the explore→build→ravage cycle ("ahead tempo" vs "on tempo"). Rei-style tempo or momentum is **spirit-level trajectory over turns**. A companion should name them separately, for example "land tempo" vs "economy momentum".
- **The way of thinking a companion should copy:**
  (a) state the hard budget (fear to next Terror Level, blight left, Energy, card plays);
  (b) list the land-by-land contributions;
  (c) check timing and resolution order;
  (d) ask whether an effect is linear or ramping;
  (e) ask what the darkest-timeline and best-timeline outcomes are.
- **Optimal play is out of reach, so use heuristics.** Her NP-hard claim (proof not published on the blog) is a first-principles reason the companion should present heuristics and budgets, not "solve" turns. That fits the project's anti-alpha, "options with tradeoffs" stance.

### Gaps
- No post discusses mid-game economy curves (Energy and card plays by turn), presence-track value, or "momentum" by name. Those concepts must come from Rei and others.
- The NP-hardness / PSPACE-hardness work is mentioned but not published on the blog. I found no proof text.
- The glossary marks play-group-specific terms with asterisks. The asterisks survived text extraction only on some terms ("drugs*", "eggplant*", "kick*", "leaf*", "rave party*", "suck*", "tall disc stack*"), so the full list of play-group terms may be incomplete.

---

## Key Question 1: What is the full list of Spirit Island posts (titles, dates, URLs)? Is there an index, cross-posting, or reader discussion?

### Takeaway
The blog is a Jekyll site with a Spirit Island index at [/si_theory.html](https://iam.lumia.gay/si_theory.html) ("my spirit island posts"). It holds **12 si_theory posts** (2023-07-26 → 2026-02-13), **6 si_general posts** (songs, pickup lines, parodies), and a **glossary**. Three uncategorized posts touch Spirit Island: a buddha-nature hosting guide, an "october update" that mentions NP-hardness, and the "Dahan Konundrum" puzzle-hunt author's notes. The live index matches the Wayback captures, with no deleted Spirit Island posts. I found no BGG or Reddit handle for the author. Her stated channel is the Spirit Island Discord and its play-by-post (pbp) community. The turn-1 victory work started from a 2022 r/spiritisland challenge thread by someone else.

### Cited Findings
- Index discovery:
  - [/feed.xml](https://iam.lumia.gay/feed.xml) is an Atom feed (Jekyll 3.10.0), but it holds only the 10 latest entries.
  - /atom.xml, /rss.xml, /sitemap.xml and /si_theory/ return 404.
  - [/si_theory.html](https://iam.lumia.gay/si_theory.html) (page headline "my spirit island posts") and [/archive.html](https://iam.lumia.gay/archive.html) ("archive of everything") are the full indexes.
  - robots.txt is "User-agent: * / Disallow: /".

  (fetched 2026-10-18)
- **si_theory posts** (all fetched in full):

| Date | Title (as written) | URL | Kind of analysis |
|---|---|---|---|
| 2023-07-26 | spirit island turn 1 victories | https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html | constructive T1 wins, fear budgets up to all 8 adversaries at L6 |
| 2023-09-03 | growth in spirit island | https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html | asymptotic growth classes, recurrence meme(n) |
| 2023-11-12 | more spirit island turn 1 victory shenanigans | https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html | enumeration of 544 aspect × adversary pairs, tier list, Google Sheet |
| 2023-11-18 | t1 victories against low level adversaries | https://iam.lumia.gay/si_theory/2023/11/18/low_level_t1_victories.html | constructive wins, near-miss fear accounting |
| 2023-11-22 | turn 1 victories against double 6s | https://iam.lumia.gay/si_theory/2023/11/22/66_t1_victories.html | constructive wins vs two L6 adversaries |
| 2023-12-29 | winning at spirit island... with no spirits? | https://iam.lumia.gay/si_theory/2023/12/29/spiritless_island.html | luck bound: deck order alone wins |
| 2024-01-10 | defeating level 6s with no spirits!? | https://iam.lumia.gay/si_theory/2024/01/10/spiritless_island_2.html | same, vs each L6 adversary |
| 2024-03-01 | if you don't control rng, part a | https://iam.lumia.gay/si_theory/2024/03/01/no_rng_control.html | 6-spirit T1 extremal puzzles (presence, cards, Energy) and an impossibility proof |
| 2024-04-16 | if you don't control rng, part b (tokens) | https://iam.lumia.gay/si_theory/2024/04/16/no_rng_control_b.html | 6-spirit T1 token maximization with additive tallies |
| 2024-06-30 | t1 victories with bad spirits | https://iam.lumia.gay/si_theory/2024/06/30/bad_spirit_t1_victories.html | constructive wins, minimal-requirement reasoning |
| 2025-01-12 | turn 1 victories on east board | https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html | fear inventory by land, three Prussia level bands, full tier list |
| 2026-02-13 | cast down thresholded on turn 1 | https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html | explicit probability of a card sequence |

- **si_general posts** (no models):
  - [welcome to spirit island, 2023-08-23](https://iam.lumia.gay/si_general/2023/08/23/welcometospiritisland.html): song parody, written after "mention in the 68-spirit pbp"
  - [spirit island pickup lines, 2024-02-14](https://iam.lumia.gay/si_general/2024/02/14/si_pickup_lines.html)
  - [welcome to the play-by-posts, 2024-06-28](https://iam.lumia.gay/si_general/2024/06/28/welcometopbp.html): Discord pbp culture; mentions "68-spirit games", "we had board game geek … we just made a second server"
  - [song for the 144p pbp, 2025-04-01](https://iam.lumia.gay/si_general/2025/04/01/144p_song.html)
  - [another song for the 144p pbp, 2025-06-25](https://iam.lumia.gay/si_general/2025/06/25/144p_song2.html)
  - [idk another random song that i wanted to write, 2025-11-01](https://iam.lumia.gay/si_general/2025/11/01/the_english_went_down_to_island.html)
- **Glossary:** [glossary of spirit island jargon](https://iam.lumia.gay/si_glossary) (undated). It says "terms with asterisks are words that are specific to my play group … if you want me to add anything, tell me on discord". It links the digital edition's Steam page as "digital: the digital app made by handelabra".
- **Uncategorized posts touching Spirit Island:**
  - [buddha nature spirit island, 2023-07-20](https://iam.lumia.gay/2023/07/20/buddha_nature_si.html): a hosting format in which players learn the rules by trying interactions and a "buddha master" judges them; "if an action requires luck … it doesn't have the buddha nature"; no math.
  - [oops i haven't posted in two months, 2023-10-30](https://iam.lumia.gay/2023/10/30/october_update.html): NP-hardness, a dahan conlang, and an early T1 tier list image.
  - [author's notes for dahan konundrum, 2023-06-12](https://iam.lumia.gay/puzzles/2023/06/12/dahan_konundrum_authors_notes.html): an MIT Mystery Hunt 2023 puzzle built from "one game, 3 players playing low complexity spirits, using basegame only"; she "hit all 9 of starlight's innate power thresholds" in one game.
  - The "how to ruin card games" parts [0](https://iam.lumia.gay/2023/04/16/ruin_card_games_p0.html) and [1](https://iam.lumia.gay/2023/06/20/ruin_card_games_p1.html) only mention Spirit Island in passing.
- **Wayback completeness check:** the CDX index for iam.lumia.gay/* lists 63 URLs. Every Spirit Island URL is one of the posts above, some also under older paths without the category prefix (for example /2023/07/26/si_t1_victories.html, /2023/09/03/si_growth.html). There are no extra Spirit Island posts. — [Wayback CDX](https://web.archive.org/cdx/search/cdx?url=iam.lumia.gay/*&output=txt&fl=original,timestamp,statuscode&collapse=urlkey)
- **Author identity:** "hi. i am lumia." She uses she/her pronouns, has "a math major and linguistics minor", and describes MIT as "the college i graduated from". The about page lists no BGG or Reddit handle. — [about](https://iam.lumia.gay/about)
- **The Reddit origin of the T1 series:** "a year ago, someone made a reddit post about winning on turn 1. there were several solutions found. there was one each with trickster, fractured, volcano, and snek." — [T1 victories](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html), linking [r/spiritisland "Puzzle Challenge: Turn 1 True Solo Victory"](https://www.reddit.com/r/spiritisland/comments/utnfs2/puzzle_challenge_turn_1_true_solo_victory/). Reddit blocked direct fetching. The [Wayback snapshot from 2022-05-20](http://web.archive.org/web/20220520065840/https://www.reddit.com/r/spiritisland/comments/utnfs2/puzzle_challenge_turn_1_true_solo_victory/) shows the rules: "you can choose any spirit and any board. You can also choose an adversary at any level if you like but you don't have to. No scenarios. / you can stack the Power decks, the fear deck and the invader deck. Event won't be resolved on turn 1 so that doesn't matter. / you have to win on Turn 1." The snapshot was taken 3 minutes after posting and shows 0 comments. The poster's name is not visible.
- **Companion Google Sheet:** linked as "link to the google sheet containing all the combos i know" ("if you find more, you can comment it on the sheet") — [more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html) → [Sheet "turn 1 against level 6 adversaries"](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1614933105). It is publicly exportable, with tabs "checkboxes" (gid 1614933105), "full games" (gid 2118374696) and "t1 stats" (gid 1166920778). Contents are in Q2.

### Inferences
- The "years ago" memory most likely refers to the 2023 posts (growth, T1 victories), since the blog started in 2023 (first post 2023-03-27).
- Reader discussion appears to happen on the Spirit Island Discord pbp channels, which are not publicly indexed, and as comments on the Google Sheet. No public BGG or Reddit threads discussing the blog turned up.

### Gaps
- I could not confirm the author's BGG or Reddit usernames. Searches for "iam.lumia.gay" returned nothing relevant.
- I could not read Reddit comments on the 2022 challenge thread (live Reddit is blocked; the only snapshot predates any comments), so the original trickster/fractured/volcano/snek solutions are known only from her summary.
- I found no BGG cross-posts.

---

## Key Question 2: For each post, what models and approaches does it use? Exact formulas, numbers, worked examples, and assumptions

### Takeaway
There is one genuine closed-form or asymptotic post (growth), one explicit probability calculation (cast down), and many constructive or extremal results with exact tallies. The reusable "models" are:
- (M1) escalating repeat cost n(n-1)/2;
- (M2) range-area coverage ~ quadratic in range;
- (M3) blight cascade growth n^1.5;
- (M4) Lair population growth by 1/6 of its size per turn (exponential);
- (M5) the bounce-fear recurrence meme(n)=1+ceil(n/2)*meme(n-1) (factorial);
- (M6) fear budget = fear cards × fear per card, with land-by-land fear inventories;
- (M7) additive contribution tallies for maximum tokens or presence;
- (M8) impossibility by resource counting;
- (M9) probability of a specific sequence as a product of draw odds;
- (M10) feasibility enumeration over 544 aspect × adversary pairs.

### Cited Findings

#### growth in spirit island (2023-09-03) — https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html
- **M1, quadratic repeat cost:** "this includes slip the flow of time, with an increasing time cost, and relentless punishment, with an increasing energy cost. they're both n(n-1)/2 for n uses, which is order n^2." (Glossary: "gays: relentless gaze of the sun (spirit)"; "fractured" = Fractured Days (my reading).) Computed here, the totals for n = 1…7 are 0, 1, 3, 6, 10, 15, 21.
- **M2, range coverage:**
  - "let's assume that each land has about the same size and same diameter. there are a finite number of different lands, so this can be approximately true."
  - "so within range n of the lair would be roughly a circle of radius (n+0.5) lands. the area increases quadratically, and since the area of a land is roughly constant, the number of lands you can cover with that range increases quadratically."
  - The motivating effect: "the 4 moon innate on lair … mentions a range equal to how much air lair has."
  - Assumption: equal-size lands on a continuous plane.
- **M3, blight cascade growth:**
  - "let's say we've been adding blight to a land repeatedly. if we do the minimum cascading possible, at some point we'll have cascaded into every land within range n."
  - "because there are order n^2 lands within range n, there are order n lands at exactly range n. so to go from range n to range n+1, we have to add n times order n blight, which is order n^2 blight in total."
  - "the sum of the first n squares is order n^3. so you're adding order n^3 blight in total to add order n^2 blight in the original land, which is a growth rate of order n^1.5."
  - My reading: total island blight T ~ n^3 against origin-land blight B ~ n^2, so T ~ B^1.5, under "minimum cascading".
- **M4, exponential growth (Lair):**
  - "the 4 moon innate of the lair innate gathers 1 invader for every 6 explorers (and dahan) in the lair. so if there's a sufficient supply of invaders appearing near the lair, the cardinality of the lair can increase by 1/6 of its size every turn. this is exponential growth. this is also how we managed to get 79 explorers in one land in a 3-player game."
  - "i wonder if you can build an anti-6/6 strategy out of this."
  - Computed here, not stated: that is a per-turn factor of up to 7/6 while supply lasts.
- **M5, super-exponential bounce fear (Bringer + Melt Earth):** ("bodan: bringer of dreams and nightmares (spirit)", per the [glossary](https://iam.lumia.gay/si_glossary).)
  - One use: "melt earth 'destroys' the town for 2 extra fear, then kicks it somewhere … this essentially doubles the fear generation against towns and explorers in adjacent lands".
  - Two adjacent lands: "this triples the fear generated".
  - Three uses (a1, a2, b): "this has gone from a 3x multiplier to a 7x multiplier on fear."
  - General: "if you define melt earth melt earth of n (shortened as meme(n)) to be the fear multiplier of n uses of melt earth, divided as evenly as possible between two lands. we have meme(n)=1+ceil(n/2)*meme(n-1). this is a factorial growth speed, which is faster than exponential."
  - Computed here, with meme(0)=1 implied by the 2x/3x/7x examples: meme(1..8) = 2, 3, 7, 15, 46, 139, 557, 2229.
  - Generalizations: "call to guard does approximately the same thing as melt earth, except that it doesn't automatically destroy things, but it works with badlands to destroy e.g. cowsburg towns." "vengeance scales faster, because the number of initial triggers is n instead of ceil(n/2). however, since it's limited by amount of invader health, which doesn't grow super quickly, people haven't designed strategies around this."
- Limits: these are order-of-growth results under idealized supply. They ignore piece limits, adjacency irregularity and timing windows. The n^1.5 result assumes minimum cascading.

#### spirit island turn 1 victories (2023-07-26) — https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html
- Origin and history:
  - The Reddit solutions: "trickster, volcano, and snek all won against prussia 0, while the solution with fractured won against up to prussia 3 … the solution with volcano has stopped working, because it uses a retired card".
  - A Nature Incarnate preview line using Transformative Sacrifice ("tsac") "no longer works after nature incarnate delivery, since shattered fragments of power was nerfed from 3 energy to 2 between the reveal and the delivery."
  - "it turns out that nature incarnate just has a bunch of cards that are amazing for early wins. this probably came at a cost of them being weaker as yolo majors."
- **Fear-budget results (M6):**
  - Base Bringer vs Saltsburg 6: "its terror 1 has only 4 cards". The line reaches "(16 fear in total, hitting tl2)", then "(18 fear in total) for a tl2 victory". The planned optimum was "10 fear from exaltation, 2 from predatory, 2 from night terrors, and 3 from dreams, for a total of 17", but "the best is actually 14 fear" due to placement constraints.
  - Violence aspect: "violence would generate 1-2 more fear from exaltation …, 1 more from predatory nightmares, and 2 more from having bats scout … that would be 5-6 more fear in the fast phase, which could total 20 fear. that would be enough to take down a double adversary!" The line reaches "(+2=24 fear in total)" with a "tl2 victory" vs Saltsburg + France 6.
  - Sweden 6 + Scotland 6: running total "+18=82 fear total", against "scotland and sweden only have 20 fear cards when combined, which only needs 80 fear total. so this is already a victory at tl4!"
  - England 6 + Cowsburg 6: "+26=101 fear total", against "cowsburg 6 + england 6 have a total of 18 fear cards combined, which is only 90 fear, so this is already a victory at tl4!"
  - All 8 adversaries at L6: "the fear deck is pretty thicc. it's 15/19/12 which is a total of 46 fear cards, which makes 230 fear to burn through." "because of england lc, the most fear i can generate is around 240, if england lc is removed it's possible to generate upwards of 300 fear by adding thriving crops." The result: "235 is greater than 230. tl4 victory!"
  - Bounce multipliers in practice: "drought in d1 kicks the 3 towns to d7, then melt earth kicks everything back to d1 (+18=46 fear total)". Late in the all-8 line, a single Sweep into the Sea plus Melt Earth resolution is "+84=193 fear total".
- Assumptions: true solo (one spirit), stacked decks (per the Reddit challenge), balanced boards except where noted, and an escalation order she chooses ("i will force scotland to escalate after prussia, france, and cowsburg").
- Thematic boards: "a bad thing about using the balanced side is that it only has 6 dahan. the thematic boards have a lot more. on northeast and east boards, it's pretty easy to get 7 dahan into one land".

#### more spirit island turn 1 victory shenanigans (2023-11-12) — https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html
- **M10, enumeration:** "since the release of nature incarnate, out of all 8*68=544 aspect-adversary pairs, surprisingly many are beatable at level 6 on turn 1. in fact, at least a quarter of them are." "(but please check my work, some of it might be wrong)".
- Pattern (sufficient conditions): "any spirit who can play haunts and embers 3 times on turn 1, with at least 2 of those happening before the blight card flips, will beat prussia, france, and saltsburg 6 by hitting terror 2. a lot of spirits can do this by having 3 energy, a card gain, and at least 3 presence on the island after t1 growth, and this lets them take twinned days and haunts and embers with tsac."
- "over 3/4 of the combos i've found involve tsac."
- "prussia, france, and saltsburg are almost always beaten at terror 2. the only aspects that defeat any of these at other levels are locus against france specifically, which wins at terror 1, and violence, which wins at terror 4."
- Expansion dependence: "only 4 of these combos were possible before nature incarnate". The helpful Nature Incarnate additions:
  - "border between life and death. border is basically one more blight that can be on the island for free, which means that spirits can get 4 fear in all 3 uses of haunts and embers and still survive aid from lesser spirits"
  - "shattered fragments of power"
  - "the fear card daunted by the dahan … generates fear without needing any towns on the island to exist"
  - "spirits starting with more presence than before, sometimes because they have an incarna … tree and waters both start with 4 presence on the island"
- Near misses: "a bunch of combos are one explorer away from winning … the most notable instance of this is sparking against scotland 6."
- **Tier list image** ([updatedtierlist.jpg](https://iam.lumia.gay/assets/img/updatedtierlist.jpg)). Rows: "yes" (Violence), "ss (8)", "s (6-7)", "a (4)", "b (3)", "c (1)", "d (< level 6)", "f".
  - Cross-checked against the sheet, the parenthetical counts are the number of level-6 adversaries beaten on turn 1. The s row's Bringer (bodan), Enticing and Towering Roots (tree) have 7, 6 and 7 in the sheet. The a row's Sharp Fangs, Encircle and Gaze have 4 each. (Inference from the cross-check.)
- **The Google Sheet** ([link](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1614933105); exported 2026-10-18):
  - "checkboxes" tab: 68 rows (spirit or aspect) × 8 adversaries (prussia, england, sweden, france, cowsburg, russia, scotland, saltsburg). The top-left cell reads **150**.
    - Cell values: −1 in 144 cells, 9 in 5 cells, 14 in 1 cell, 0 in 394 cells. 144 + 5 + 1 = 150.
    - The 9-coded cells (sparking/trickster/hv/tactician vs Saltsburg, vengeance vs Cowsburg) have "board east starting" lines in "full games". The 14-coded cell (mentor vs Prussia) is "board southeast starting". So 9 and 14 appear to flag thematic-board solutions (inference).
    - Per-adversary counts of −1 cells: Prussia 25, England 10, Sweden 13, France 26, Cowsburg 14, Russia 14, Scotland 18, Saltsburg 24.
    - Rows with all 8 adversaries beaten: earth, resilience, might, nourishing, eeb, violence, locus, waters, fractured.
  - "full games" tab: top-left **105**, with 143 non-empty cells, each a step-by-step line ("board a starting / initial explore includes wetlands / g3, gain tsac, …").
  - "t1 stats" tab (usage counts across solutions):
    - cards: tsac 97, haunts 129, drought 40, twinned 103, sky stretches 32;
    - blight cards: strong earth 84, border 89, thriving 8, aid 122, shattered 17;
    - boards: a 22, b 2, c 6, d 72, e 15, f 18, g 6, h 0, northwest 0, northeast 0, west 0, east 5, southwest 0, southeast 1;
    - "build 32".

#### t1 victories against low level adversaries (2023-11-18) — https://iam.lumia.gay/si_theory/2023/11/18/low_level_t1_victories.html
- Sparking (Lightning aspect, my reading), board D: the running tally reaches "(total 9)" for a terror-level win at low level.
- vs Scotland 6: "(total 11)" then "this is 1 fear off from getting another fear card to remove the last explorer :dismay:".
- Both Wildfires on thematic SE: total "8" fear. The line is branch-conditioned ("if trans… if cis…"; glossary: "cis: base heart of the wildfire", "trans: transforming wildfire (aspect)").

#### turn 1 victories against double 6s (2023-11-22) — https://iam.lumia.gay/si_theory/2023/11/22/66_t1_victories.html
- "there are 10 spirit/aspects that i know can defeat a double 6 on turn 1, increased from the 9 mentioned previously."
- Lines with fear tallies:
  - waters vs Cowsburg 6 + Sweden 6 ("9 fear total"; "the extra starting blight is very useful against cowsburg")
  - "any earth" vs Cowsburg 6 + Sweden 6 (12)
  - fractured vs Scotland 6 + Russia 6 (12)
  - trickster vs Cowsburg 6 + Scotland 6 (10; "arson is just so powerful if you control rng")
  - eeb vs Cowsburg 6 + Scotland 6 (24; "eeb can just casually max out its innate on turn 1")
  - locus vs Cowsburg 6 + Scotland 6 (22; "this win uses a silly major")

#### winning at spirit island... with no spirits? (2023-12-29) — https://iam.lumia.gay/si_theory/2023/12/29/spiritless_island.html
- Method: choose the board (D), the initial explore, and the event, fear and blight card order, then play zero spirits. "well, the spirits may be absent, but what if the invaders are extremely unlucky?" The ending is "terror 1 victory!". Turn 2's event (Far-Off Wars) generates "4 fear and adding 2 blight".

#### defeating level 6s with no spirits!? (2024-01-10) — https://iam.lumia.gay/si_theory/2024/01/10/spiritless_island_2.html
- No-spirit wins at L6:
  - Prussia 6: "20 fear total, reaching terror 2" → "terror 2 victory!"
  - France 6: terror 2 at 20 fear
  - Cowsburg 6: "terror 1 victory!"
  - Russia 6: "24 fear total, reaching terror 2"
  - Scotland 6: terror 1
  - Saltsburg 6: "20 fear total, reaching terror 2"
- "(i haven't yet found a way to defeat england or sweden 6)".
- Inconsistency in the text: it says "these invaders try this again with 4 other countries" but then lists five (France, Cowsburg, Russia, Scotland, Saltsburg).

#### if you don't control rng, part a (2024-03-01) — https://iam.lumia.gay/si_theory/2024/03/01/no_rng_control.html
- Setup: six spirits on turn 1, no deck stacking ("optimizations that don't involve controlling what cards you draw"). Each puzzle is a script with a running count in parentheses.
- Results:
  - max Gift of Proliferation uses by Green: **6** ("green repeats prolif twice (6)")
  - max presence Serpent ("snek") can add on T1: **8** ("this is enough for snek to have 4 card plays or 6 energy income on turn 2"; "this also works for locus")
  - max Regrowth presence on the island: **14**
  - max incarna empowers: **3**, with the impossibility proof quoted in the Framework
  - max Minor Powers removed from the deck: **35**
  - max Majors: **27**
  - max impending cards (Dances Up Earthquakes): **9** ("except that there will only be like 4 quake tokens to use with the right innate")
  - max Energy for one spirit: **27** (repeat Boon of Vigor)
- Assumptions: an arbitrary six-spirit team (6-player), specific board conditions (for example "snek needs to be on a board whose land 5 is within range 7 of a land with blight and invaders").

#### if you don't control rng, part b (tokens) (2024-04-16) — https://iam.lumia.gay/si_theory/2024/04/16/no_rng_control_b.html
- **M7, additive contribution tallies** (verbatim sums):
  - beasts: "4 (minds) + 3 (fangs) + 2 (lure) + 2 (locus) + 2 (might) + 4 (fractured) + 6 (default setup) + 6 (russia 1) = 29"
  - disease: "for a total of 23"
  - wilds: "3 (keeper) + 2 (behemoth) + 2 (tangles) + 6 (fractured) + 2 (downpour) + 5 (thematic boards) = 20"
  - badlands: "3 (gays) + 2 (gays with support) + 2 (volcano) + 2 (lure) + 1 (lure with elements) + 3 (slips and repeats) + 5 (thematic boards) = 18"
  - strife: "3 (voice) + 2 (mentor helping voice) + 3 (boddys) + 9 (fractured slipping boddys) + 2 (trickster) + 4 (locus helping trickster) + 3 (thematic boards) = 26"
  - strife on a single invader: "26 - 3 - 2 = 21" ("on a city, this would go down by 1 more to 20")
  - vitality: "1 (tree g2) + 1 (locus) + 1 (nourishing) + 1 (blooming) + 1 (locus repeat) + 1 (nourishing repeat) + 2 (slips) + 1 (downpour) + 4 (thematic boards) = 13 vitality in total"
  - quake: "2 from innates and 5 from rumblings for a total of 7"
  - deeps: "this hits all 3 levels of the deeps adding innate"
- Method: rank spirits by direct contribution, then list "support" spirits (repeats, slips, elemental support), then choose the six that maximize the total. It is a knapsack-style greedy with feasibility checks ("this does come at the cost of not being able to support fractured").

#### t1 victories with bad spirits (2024-06-30) — https://iam.lumia.gay/si_theory/2024/06/30/bad_spirit_t1_victories.html
- "any shadows against saltsburg 1"; "notice that i didn't use any special rules of any shadows, so this just works with all shadows."
- Minimal-requirement reasoning: "so the minor we get from aid would need to be able to deal at least 2 damage to the d2 city, plus destroy the explorer that comes from the turn 1 explore. so it must deal at least 3 damage, 2 of which is in d2." That leads to "any non-madness shadows against saltsburg 1 prussia 4", with a per-aspect reachability "proof that all shadows except madness can use purifying flame on d2".
- New spirit check: "(apparently covets is pretty good in multiplayer but sucks in solo)". The thematic East line vs Prussia 3 + Saltsburg 3 reaches "12 fear total", then "afterwards, i realized that i was actually playing sunshine."
- "hmm, saltsburg and thematic boards both seem to make winning on turn 1 a lot easier. but how much easier?"

#### turn 1 victories on east board (2025-01-12) — https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html
- **M6, land-by-land fear inventory:** "destroying all those buildings would only be 9 fear in total. adding prussia would only add 1 town, and prussia 4 or higher would need 16 fear generated." Then the 14-fear inventory quoted in the Framework.
- Adversary-rule interaction: "saltsburg ravages upgrade instead of cascade, which is a good thing since upgrading invaders means more fear for destroying them. however, it also means that the two ravages are only going to add 2 blight."
- Level banding: "### low level prussia: prussia 1-3", "### mid level prussia: prussia 4-5" ("generate 2 fear to reach terror 2, which is at 16 fear"), "### high level prussia: prussia 6". Spirits are grouped into those bands (sections 4.1–4.3).
- Full tier list ([fulltierlist.png](https://iam.lumia.gay/assets/img/fulltierlist.png)). Rows: violence, ss, s, a, b, c, "c through sacrifice", "d♭", "d but cooler", "d", "d but questionable", "d♯", "d♯ through sacrifice", "e♭", "e", "would not be in f if it started in any other land" (Haven, Travel, base River), "f".
  - Labeled entries include ss: Might, Nourishing, Resilience, Locus; d♭: Amorphous, Dark Fire, Foreboding, Reach; e♭: Madness.
  - "Sacrifice victory" is defined in-line: "this is a sacrifice victory. memory can't deal with both east2 and east3 but can threshold drought to take out the east3 city without the ravage happening." Elsewhere: "drought wipes e2, adds a blight for sacrifice victory" ([sheet](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=2118374696)).

#### cast down thresholded on turn 1 (2026-02-13) — https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html
- Scope: "these work only against adversary combos that include sweden 6. i will demonstrate on board a. no invader cards matter."
- **M9, probability of a sequence** (verbatim list):
  - "gain tsac: 4/78"
  - "tsac takes haunts and embers, constancy, skies herald: 1/(100c3)=1/161700"
  - "flip border: 1/20 (can ignore all other still-healthy cards because they don't affect anything)"
  - "flip shattered fragments: 1/19"
  - "take unlock: 3/77"
  - "gain nature's connection: 4/97"
  - "unlock gains powerstorm: 2/76"
  - "unlock gains exaltation: 2/75"
  - "unlock gains cast down: 2/74"
  - "this multiplies to 1 in 39,321,133,595,493,750 which is 1 in 39.32 quadrillion."
- **Verified here:** the exact product of the nine fractions is 1/39,321,133,595,493,750 (Python `fractions`), and C(100,3) = 161,700.
- Implied model (my inference, not stated): the falling denominators assume a 78-card Major deck (78 → 77 → 76 → 75 → 74 as Majors leave), a 100-card Minor deck (100 → 97 after three Minors), and 20 then 19 candidate blight cards. Each step is "the specific card is among k cards revealed". The steps are treated as independent, and the result counts only this one line (other successful lines are ignored), so it is a lower bound on the probability of *some* T1 cast-down win.

### Inferences
- The blog's quantitative content splits into three groups:
  - **closed-form scaling laws:** M1–M5, which are real formulas;
  - **budget and tally arithmetic:** M6–M7, rules-faithful and exact for the stated line;
  - **combinatorial search results:** M8–M10, existence, impossibility and coverage, and a single sequence probability.
- Nothing in the blog estimates a distribution over normal games. Every result is "with this exact line or deck order", which is why it pairs well with live, state-specific computation rather than with aggregate stats.

### Gaps
- The meaning of the "full games" tab's top-left "105" and the "t1 stats" row "build 32" is not explained anywhere I could find.
- The 9 and 14 codes in "checkboxes" are interpreted by cross-reference only.
- The tier-list images were read visually. Several entries are unlabeled spirit art, so full row membership is uncertain beyond the labeled entries and the sheet cross-check.
- I did not verify card texts (Melt Earth, Call to Guard, Haunts and Embers, Transformative Sacrifice, etc.) against the wiki. The formulas above are the author's claims. Per project rules, run `si-wiki-fetch` before turning any of them into app logic.

---

## Key Question 3: Which approaches are situational (computable live from game state) and which are static? How could each become a live stat in the Steam companion?

### Takeaway
The most useful live stats are:
- the **fear budget and runway** (M6), with a land-by-land "fear in reach" inventory;
- **land tempo tags** from the glossary (ahead tempo vs on tempo, b2b, capital, threat, problem land, solve or take-blight-to-scale);
- **blight-budget and cascade-chain views** (M3);
- **exact range coverage** by graph BFS (M2, replacing her continuous approximation);
- **escalating-repeat cost ledgers** (M1);
- **deck-state draw odds** (M9, generalized).

The growth taxonomy itself (M1–M5) is static knowledge used to label effects as linear or ramping. The T1 feasibility matrix and tier lists (M10) are static pre-game facts. The extremal token and presence maxima (M7, M8) are static curiosities with little in-game use.

### Cited Findings (approach → static or situational → live-stat design)
- **M6 fear budget and runway (situational).**
  - The post computes "fear cards × fear per card" ("20 fear cards … only needs 80 fear total"; "46 fear cards, which makes 230 fear") and per-land fear inventories ("for a total of 14"). — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html), [East](https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html)
  - Live stat: "fear to next Terror Level / to fear-deck victory", plus "fear available in reach" = Σ over lands of (fear from destroying the buildings there) + guaranteed fear sources. Show the shortfall the way she does ("1 fear off from getting another fear card").
  - Inputs: current fear pool, fear cards remaining per stage, pieces per land.
- **Victory-condition tracker (situational).**
  - The posts reason about what must be removed at each terror level: "terror 1 victory", "terror 2 victory" once settlements are gone with explorers left, near misses "one explorer away". — [spiritless 2](https://iam.lumia.gay/si_theory/2024/01/10/spiritless_island_2.html), [more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html)
  - Live stat: "blockers to victory now" (glossary: "blocker: a land or invader that is preventing victory" — [glossary](https://iam.lumia.gay/si_glossary)).
- **Land tempo and risk tags (situational).**
  - Glossary definitions of tempo, b2b, capital, threat, problem land, solve, take blight to scale, darkest timeline, pocket — [glossary](https://iam.lumia.gay/si_glossary).
  - Live stat: per-land badges computed from the invader track (explore/build/ravage cards, plus the visible next explore after it is revealed):
    - "ahead-tempo window" (explored, not yet built);
    - "on-tempo window" (built, ravaging next);
    - "b2b";
    - "capital (7 buildings)" for England;
    - "threat"/"problem land";
    - "pocket".
  - A "darkest timeline" preview can enumerate the worst remaining invader card for the current state.
- **M3 blight cascade (situational).**
  - "if we do the minimum cascading possible …" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html). In the T1 lines, blight-card flips (Border, Strong Earth, Aid) are sequenced deliberately — [more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html).
  - Live stat: "blight left before flip / before loss", "cascade chain if land X blights" (adjacent blighted lands), and a bpp running measure (glossary "bpp").
- **M2 range coverage (situational).**
  - "within range n … roughly a circle of radius (n+0.5) lands" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html).
  - Live stat: exact count and highlight of lands reachable at range r from current presence or sacred sites (BFS on the real board graph). Show the marginal lands gained per +1 range, which is what makes her "quadratic" point concrete.
- **M1 escalating repeat cost (situational).**
  - "n(n-1)/2 for n uses" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html).
  - Live stat for Fractured Days (Time) and Relentless Gaze (Energy): "next repeat costs n-1; total so far n(n-1)/2", next to current Time or Energy.
- **M4 compounding population (situational, niche).**
  - Lair "can increase by 1/6 of its size every turn" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html).
  - Live stat: "Lair incarna-land size and projected next-turn gather", with a ramp label.
- **M5 bounce-fear multiplier (situational, niche).**
  - "meme(n)=1+ceil(n/2)*meme(n-1)" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html).
  - Live stat: when multiple Melt Earth / Call to Guard effects are active, show the expected fear per pushed town. Verify card text first.
- **M9 draw odds (situational).**
  - "gain tsac: 4/78 … 1/(100c3) …" — [cast down](https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html).
  - Live stat: hypergeometric odds that a named card or category appears in the next k-card draft, given current deck composition (discards and removed cards known from the log). Multiply only when the steps are truly sequential and independent.
- **Economy-over-time prompts (situational coaching, not numeric).**
  - Glossary "down turn", "underplay", "pivot", "casino", "reclaim loop", "predraft", "yolo major", "3/3 … for major-slinging" — [glossary](https://iam.lumia.gay/si_glossary).
  - Live use: label a turn's plan ("this is a down turn: you are investing"), and flag when the player reaches x/y = 3/3.
- **Static or pre-game items:**
  - the growth-class taxonomy (label effects linear vs ramping);
  - the T1 feasibility matrix and tier lists ([sheet](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1614933105)): which spirit/aspect × L6 adversary pairs admit a turn-1 burst with stacked decks;
  - the x/y/z presence-per-growth classification ("earth is a 1/1/1 spirit, river is a 0/2/1 spirit") — [glossary](https://iam.lumia.gay/si_glossary).

### Inferences
- **The highest-value live stat from this blog is the fear runway with a per-land inventory.** It is exact, cheap to compute from game state, directly tied to a victory threshold, and teaches her habit of counting before acting. It complements the project's existing fear-trigger work in community_analyses.md.
- **Land tempo tags give the "where am I relative to the invader cycle" half of momentum.** A Rei-style economy curve (Energy and card plays per turn against expected invader pressure) gives the "is my engine ramping" half. Together they form the "ahead of the curve" dashboard the user wants. This is synthesis, not something the blog does.
- **Relation to the existing model catalog.** M3 and M4 are informal versions of the multi-type branching "snowball rate" model already listed in [stats_models_references.md](/home/brett/repos/spirit-island-steam-companion/research_notes/Spirit%20Island%20companion%20stat%20models/stats_models_references.md) (M8 there: dominant eigenvalue ρ > 1 means unchecked growth). Her Lair "1/6 of its size every turn" is a concrete ρ = 7/6 example on the spirit side. The "darkest timeline" is the worst case of the lookahead and rollout models listed there.

### Gaps
- None of the blog's methods estimate *expected* values under normal play. Turning budgets into probabilities (for example, the chance of reaching TL2 by turn N) needs other models (fear-card odds, invader-deck odds) from the existing notes.
- The live-stat mappings above are my design proposals and are not validated against the Steam client's observable state. Check them against engine_state_model.md before building.

---

## Key Question 4: How do the approaches treat adversary rules, expansions, and player count?

### Takeaway
Adversaries are central: L6 single, "double 6s", and all eight at L6 together. Adversary rules are used as levers:
- Saltsburg upgrades instead of cascading;
- England's building health and loss condition;
- Russia's fear cards and pushed explorer;
- Sweden 6 enabling cast down;
- combined fear-deck sizes.

Expansions matter: Nature Incarnate is credited with enabling almost all T1 wins. Thematic boards help (more dahan). Player count is almost always **true solo** for victory posts, **six spirits** for the extremal puzzles, and 3-player in one anecdote.

### Cited Findings
- **Adversary rules used as mechanics:**
  - "saltsburg ravages upgrade instead of cascade, which is a good thing since upgrading invaders means more fear for destroying them" — [East](https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html)
  - Saltsburg 6 "terror 1 has only 4 cards" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - "the combos involving russia need some setup to earn fear from russia 2 kicking explorers, as russia starts with 5 fear cards at tl1." — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - England: "for the longest time, i thought melt earth didn't work into this combo (it apparently works normally if the melt earth land is blighted) … scour didn't work nearly as well against this adversary pair … rumbling earthquakes, since that completely ignores the health bonuses from both adversaries." The England loss condition ("england lc") caps the all-8 line "around 240" fear. — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - Sweden 6 is required for the cast-down lines: "these work only against adversary combos that include sweden 6." — [cast down](https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html)
  - Escalation ordering in combined games: "i will force scotland to escalate after prussia, france, and cowsburg, since scotland can add towns in lands with towns … england escalation is forced to be in lands 2 or 3". Combined fear deck "15/19/12 … 46 fear cards". — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - Prussia levels are banded 1–3 / 4–5 / 6 by what they add (an extra town; TL2 at 16 fear; escalation) — [East](https://iam.lumia.gay/si_theory/2025/01/12/east_t1_victories.html)
  - Spirit-free wins found vs Prussia, France, Cowsburg, Russia, Scotland and Saltsburg 6, "(i haven't yet found a way to defeat england or sweden 6)" — [spiritless 2](https://iam.lumia.gay/si_theory/2024/01/10/spiritless_island_2.html)
  - The sheet's per-adversary counts of found T1 wins: England lowest (10), then Sweden (13), Cowsburg and Russia (14), Scotland (18), Saltsburg (24), Prussia (25), France (26) — [sheet](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1614933105)
- **Expansions:**
  - "only 4 of these combos were possible before nature incarnate", plus the list of Nature Incarnate enablers (Border, Shattered Fragments, Daunted by the Dahan, more starting presence and incarna) — [more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html)
  - The volcano solution is broken by a retired card; Shattered Fragments was "nerfed from 3 energy to 2" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - Usage stats show the reliance on Nature Incarnate blight cards: border 89, aid 122, strong earth 84 of ~150 solutions — [sheet t1 stats](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1166920778)
- **Boards:**
  - "a bad thing about using the balanced side is that it only has 6 dahan. the thematic boards have a lot more" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - "saltsburg and thematic boards both seem to make winning on turn 1 a lot easier" — [bad spirits](https://iam.lumia.gay/si_theory/2024/06/30/bad_spirit_t1_victories.html)
  - Solutions concentrate on board D (72 uses) — [sheet t1 stats](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1166920778)
- **Player count:**
  - The T1 series follows the Reddit "Turn 1 True Solo Victory" challenge — [Wayback snapshot](http://web.archive.org/web/20220520065840/https://www.reddit.com/r/spiritisland/comments/utnfs2/puzzle_challenge_turn_1_true_solo_victory/)
  - Exaltation of the Incandescent Sky grants "an extra card play and two slow-to-fasts, or twice that in true solo" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - The RNG puzzles use six spirits — [part a](https://iam.lumia.gay/si_theory/2024/03/01/no_rng_control.html), [part b](https://iam.lumia.gay/si_theory/2024/04/16/no_rng_control_b.html)
  - "79 explorers in one land in a 3-player game" — [growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)
  - "(apparently covets is pretty good in multiplayer but sucks in solo)" — [bad spirits](https://iam.lumia.gay/si_theory/2024/06/30/bad_spirit_t1_victories.html)

### Inferences
- **4 vs 5 fear per card.** The Sweden + Scotland budget uses 4 fear per card (20 cards → 80). The England-containing budgets use 5 per card (18 → 90; 46 → 230). That fits an England-level rule adding Fear to the pool per player, but the post does not say so and I did not verify the England rule here. A live fear-runway stat must take fear-per-card from the actual adversary setup, not assume 4 × players.
- **Exploitability, not difficulty.** The per-adversary T1-win counts measure how exploitable each adversary is to a turn-1 burst under stacked decks. They are not a difficulty or win-rate measure (they are not aggregate outcomes). The ordering (England and Sweden hardest; France, Prussia and Saltsburg easiest) is still a useful qualitative signal of which adversary rules punish burst fear.
- **Different objectives, not a contradiction.** Thematic boards make T1 wins easier (more dahan). The community treats thematic as +1/+3 difficulty for normal games (existing notes, community_analyses.md line 273). These are different objectives.

### Gaps
- No post treats 2-, 3- or 4-player games quantitatively beyond the anecdote. No formula scales with player count except the fear-per-card arithmetic.
- England's fear-pool rule needs verification against the wiki or the rulebook before use.

---

## Key Question 5: Where do the posts agree or conflict with each other, with other community math (Rei ~2.5 Energy per card play, MindWanderer's regressions), or with the rules?

### Takeaway
Internally the posts are consistent, with three small discrepancies:
- the share of solutions using Transformative Sacrifice (">3/4" in the post vs 97/150 in the sheet);
- the "4 other countries" vs five listed;
- 4 vs 5 fear per card, explainable by England's rules.

Against the community: the blog does not compete with Rei's valuation or MindWanderer's regressions. It is orthogonal. It is exact, line-specific and possibility-focused, where they are average-value or aggregate. Its glossary explicitly defers to Rei's "algo". Several rules interpretations are asserted without citation and should be verified before reuse.

### Cited Findings
- **Transformative Sacrifice share:** "over 3/4 of the combos i've found involve tsac" ([more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html), at the "a quarter of 544" = 136 point). The sheet's "t1 stats" shows tsac 97 against a checkbox total of 150 (~65%) — [sheet](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1166920778). The stats tab may be stale or count differently.
- **Count consistency:** "there are 10 spirit/aspects that i know can defeat a double 6 on turn 1, increased from the 9 mentioned previously" ([double 6s](https://iam.lumia.gay/si_theory/2023/11/22/66_t1_victories.html)) is consistent with "i have found 8 so far, namely the ones in tier ss" plus Violence ([more T1](https://iam.lumia.gay/si_theory/2023/11/12/more_t1_victories.html)).
- **Rei:** the blog's only valuation concept is "algo: (rei's understanding of) the algorithm that the developers use to balance growth options, presence tracks, and power cards by assigning values to effects" ([glossary](https://iam.lumia.gay/si_glossary)). The blog never states an Energy-per-card-play number. Its only economy benchmark is "3/3 is a good place to get to for major-slinging strategies" ([glossary](https://iam.lumia.gay/si_glossary)). The project's notes record Rei's "card play is worth ~2.5e" and its critiques ([community_analyses.md](/home/brett/repos/spirit-island-steam-companion/research_notes/Spirit%20Island%20companion%20stat%20models/community_analyses.md), line 113).
- **MindWanderer:** that work is logistic and ridge regression over digital games (community_analyses.md, line 144). The blog has no aggregate statistics. Its only "dataset" is a feasibility matrix of hand-found lines ([sheet](https://docs.google.com/spreadsheets/d/1G3oiYoZM1kmO5WzvqOsFINzIdjNzVGgEXWnyQD_SP3M/edit#gid=1614933105)).
- **Rules points asserted by the author (verify before reuse):**
  - "losses are checked at the end of action trees" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - Melt Earth vs England + Cowsburg "apparently works normally if the melt earth land is blighted" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html)
  - Thresholded Cast Down without Sky Stretches or Exaltation "will resolve in slow phase" — [cast down](https://iam.lumia.gay/si_theory/2026/02/13/t1_castdown.html)
  - The Reddit challenge's "Event won't be resolved on turn 1" — [Wayback](http://web.archive.org/web/20220520065840/https://www.reddit.com/r/spiritisland/comments/utnfs2/puzzle_challenge_turn_1_true_solo_victory/). This matches the project memory note that the first Event card is previewed during Turn 1 and resolves on Turn 2.
- **Card changes that broke lines:** a retired card (volcano solution) and the Shattered Fragments nerf "from 3 energy to 2" — [T1](https://iam.lumia.gay/si_theory/2023/07/26/si_t1_victories.html). The blog's lines are tied to card versions.
- **Growth framing vs designer commentary:** the growth post brackets "the ramping from invader stages" as the invader-side non-linearity ([growth](https://iam.lumia.gay/si_theory/2023/09/03/si_growth.html)). That is consistent with darker's note in the project's notes that the stage step-up "comes at a constant point in time" (community_analyses.md, line 66).

### Inferences
- **Precision and versioning.** For the companion, the blog's style (exact ledgers, explicit resolution order) is more rules-precise than value-chart heuristics. It is also brittle to card errata, so any stat derived from it must be versioned against the Steam build's card data.
- **No conflict with Rei; they are complementary.** Rei supplies "what is an action worth"; lumia supplies "how does it scale, what is the budget, when does it resolve". MindWanderer supplies population averages, which the user has excluded, and the blog does not touch them.

### Gaps
- I did not verify the asserted rules points or the England fear-pool rule against the official rules or wiki in this pass. Use `si-rules-check` / `si-wiki-fetch` before encoding them.
- I could not find any reader critique of the blog's math, because the Discord is not indexed and the sheet comments were not exported.
