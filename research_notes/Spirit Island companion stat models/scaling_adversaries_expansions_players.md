# Scaling dimensions (adversaries, expansions, player count, scenarios) and what they do to the companion's probability and decision models

Conventions used in every table below.

- **Models** (the seven in scope): **INV** = invader-deck next-card odds (stage pools, one random card removed per stage); **EBR** = per-land Explore/Build/Ravage forecast; **FEAR** = fear-to-next-card and terror-level timeline; **BLT** = blight pool and cascade risk; **PWR** = power-deck draw odds (hypergeometric); **ELEM** = element-threshold checks; **RACE** = multi-clock race (fear victory vs. blight loss vs. invader-deck exhaustion vs. adversary/scenario loss).
- **Type** of change: **D** = deterministic given public state; **D/c** = deterministic but players pick among ties (forecast must show a candidate set); **R** = adds a random variable; **H** = changes the hidden-information structure; **K** = moves a clock (win or loss).
- **CF?** = does the change keep a closed-form model (✓) or force simulation (✗ SIM). "One-step" = the forecast for cards already face-up; "multi-step" = projecting past the next unrevealed invader card.
- Sources are cited with reference-style links. Their full URLs are in the link list at the end of this file. "Engine" means the Handelabra Spirit Island (Steam) engine definition files that the parent session extracted to this session's scratchpad. That copy is temporary, so re-extract the files if they are needed later.

---

## Q1. Adversaries by level (base + expansions): escalation, deck changes, extra Explore/Build/Ravage, setup, fear deck, loss conditions, difficulty; deterministic vs. random

### Takeaway
All eight adversaries are deterministic rule changes with no dice. Their randomness enters only through the invader deck (more unseen-removed cards, moved cards, or "unused" cards brought back), through the fear deck (Russia L5 hides invader cards in it), through combined-adversary terrain picks, and through player tie-break choices. Every deck change keeps the per-stage exchangeable closed form for **INV**. The rules that break closed-form multi-turn **EBR** and **RACE** projections are the state-dependent ones: England L1/L6, Scotland L3/L6, Russia L3/L6, both Habsburgs (Mining lands, Salt Deposits, Migratory Herders, Durable Towns), Sweden L5, France L2/L5, and every additional loss condition.

### Cited Findings

#### Universal adversary rules
- An adversary's "base level" (level 0) is its Escalation plus any additional loss condition. Levels are cumulative: "if you're fighting vs. Level 3, you also use the effects from Level 1 and Level 2." — [BASE rulebook, Adversaries][BASE]; [Wiki: Adversary][W-ADV]
- Escalation happens when a Stage II card with the Escalation icon is revealed: "If the card has a flag icon and you are playing with an Adversary, first perform the Escalation effect." — [Wiki: Sequence of Play §3c][W-SOP]. In the engine card list, only the four single-terrain Stage II cards (Jungle/Mountain/Sands/Wetland) carry `escalation:true`. The Stage II "Coastal Lands" card does not. — Engine `InvaderCardList.json` [ENG-INV]
- Invader card pool: Stage I = J, M, S, W (4); Stage II = Coastal + J, M, S, W (5); Stage III = 6 two-terrain cards (JS, JW, MJ, MW, SM, SW). — Engine [ENG-INV]. The base deck is 3/4/5 cards, made by "Mix up the cards from each Stage and choose without looking at them, putting unused cards back in the box … you can just remove one of each." — [BASE rulebook, Making the Invader Deck][BASE]
- Each level sets the fear deck size per Terror Level: "Each Level shows how many Fear Cards to use and how many cards go in the top/middle/bottom of the Fear Deck." — [BASE][BASE]. The default is 9 cards as 3/3/3. — [Wiki: Fear and Terror][W-FEAR]
- Base-game difficulty table (BP, England, Sweden): base level = 1 for each. BP L1 = 2, Sweden L1 = 2, England L1 = 3, Sweden L2 = 3, BP L2 = England L2 = 4, Sweden L3 = 5, BP L3 = England L3 = Sweden L4 = 6, BP L4 = England L4 = Sweden L5 = 7, Sweden L6 = 8, BP L5 = England L5 = 9, BP L6 = England L6 = 10. Thematic maps add +3. — [BASE reference page][BASE]. The transcription flattens the table and drops its columns. I assigned levels to adversaries by matching the engine's difficulty values, and every row is consistent with them.
- Reminder Cards placed in the invader deck (Habsburg L5+, the no-Events option) "are not considered Invader Cards … Effects which alter the Invader Deck ignore Reminder Cards … A Reminder Card's depth … will be changed when Invader Cards are added to or removed from the top of the deck." — [JE rulebook p.9][JE]
- When more than one card sits on an Invader space, each card resolves separately, bottom card first. When no card sits there, that step does nothing. Cards on a space advance together. — [JE rulebook p.11][JE]
- NI rulebook errata: adversary loss conditions are checked only once no triggered actions remain. "Can't place" losses (France) are noted immediately, but the triggered actions may still be played out to look for a sacrifice victory. — [NI rulebook p.13][NI]
- NI land-health rewording: Sweden L1 = "4+ overkill → Add 2 instead of 1 Blight". Habsburg Livestock loss = Blight off the card "during Ravages that do 6+ overkill". Overkill is damage above land Health (normally 2, min 1, modifiers stack). — [NI rulebook p.14][NI]; [querki FAQ (JE)][Q-JE]

#### Per-adversary level tables
Fear deck is shown as total (TL I/TL II/TL III). Difficulty is in parentheses. Rules are paraphrased. Each table's sources: the wiki adversary page (Wayback 2023–2025 snapshot) and the engine `AdversaryList.json` [ENG-ADV]. They agree on every fear-deck count and on every difficulty except England L6 (see Conflicts).

**Brandenburg-Prussia** (base game) — [Wiki][W-BP]; [ENG-ADV]; [BASE][BASE]. No additional loss condition. Escalation "Land Rush": (before Exploring) on each board with Town/City, add 1 Town to a land without Town.

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 (3/3/3) | Land Rush escalation | EBR | D/c |
| 1 (2) | 9 (3/3/3) | Setup: +1 Town in land #3 on each board | EBR initial state | D |
| 2 (4) | 9 (3/3/3) | Move 1 Stage III card between Stage I and Stage II → **111-3-2222-3333** | INV (position 4 = Stage III), Event stage | D (position), R (identity) |
| 3 (6) | 10 (3/4/3) | Remove 1 more Stage I → **11-3-2222-3333** (11 cards) | INV (Stage I: 2 of 4 used), RACE deck −1 | H, K |
| 4 (7) | 11 (4/4/3) | Remove 1 more Stage II → **11-3-222-3333** (10) | INV, RACE −1 | H, K |
| 5 (9) | 11 (4/4/3) | Remove 1 more Stage I → **1-3-222-3333** (9) | INV, RACE −1 | H, K |
| 6 (10) | 12 (4/4/4) | Remove all Stage I → **3-222-3333** (8); the setup Explore is a Stage III card | INV, EBR (2-terrain explore at setup), RACE −1 | H, K |

- BP L2's early Stage III card counts as Stage II only for "What Invader Stage is the game in?" (Events, Second Wave, Ward the Shores, no-Events Beasts rule). It counts as Stage III for card-specific checks, such as combined-adversary Escalation. — [querki FAQ (JE)][Q-JE]; [JE p.12][JE]

**England** (base game) — [Wiki][W-ENG]; [ENG-ADV]. Loss "Proud & Mighty Capital": 7+ Town/City ever in one land. Escalation "Building Boom": on each board with Town/City, Build in the land with the most Town/City.

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 | Building Boom; 7-in-a-land loss | EBR, RACE | D/c, K |
| 1 (3) | 10 (3/4/3) | Build Cards also affect matching lands without Invaders if adjacent to ≥2 Town/City | EBR build target set (adjacency predicate) | D |
| 2 (4) | 11 (4/4/3) | Setup: +1 City land #1, +1 Town land #2 per board | EBR initial state, RACE (capital loss) | D |
| 3 (6) | 13 (4/5/4) | "High Immigration" tile left of Ravage: an extra Build each Invader Phase, before Ravage, using the card that slid off Ravage. Tile is removed when a Stage II card slides onto it | EBR (Stage I cards build twice) | D |
| 4 (7) | 14 (4/5/5) | High Immigration stays all game | EBR (every card builds twice) | D |
| 5 (9) | 14 (4/5/5) | Town/City +1 Health | EBR kill/damage thresholds, FEAR (fewer kills) | D |
| 6 (10 per engine and base rulebook; 11 per wiki) | 13 (4/5/4) | Setup: +1 Fear per player in the Fear Pool. In any Invader Phase where no Fear Card resolves, the High Immigration Build happens twice | FEAR (threshold 5N), EBR coupled to fear | D, K |

**Sweden** (base game) — [Wiki][W-SWE]; [ENG-ADV]; [NI p.14][NI]; [querki FAQ (Core)][Q-CORE]. No additional loss condition. Escalation "Swayed by the Invaders": after Invaders Explore into each land (for that card only), if Invaders ≥ Dahan there, replace 1 Dahan with 1 Town. — [querki FAQ (JE)][Q-JE]

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 | Swayed escalation | EBR, Dahan count | D |
| 1 (2) | 9 (3/3/3) | Heavy Mining: a Ravage with 4+ overkill adds an extra Blight (no extra cascade or Presence loss) | BLT (blight per Ravage becomes a step function of damage) | D |
| 2 (3) | 10 (3/4/3) | Setup: +1 City land #4; land #4's Blight moves to #5 | EBR initial state | D |
| 3 (5) | 10 (3/4/3) | Town deals 3 damage, City deals 5 | EBR Ravage damage → BLT, Dahan deaths | D |
| 4 (6) | 11 (3/4/4) | After the initial Explore, Accelerate the deck (discard the topmost card of the lowest stage). +1 Town per board in the land with the fewest Invaders matching the discarded card | INV (one more card seen), RACE deck −1 | H (reveals), K |
| 5 (7) | 12 (4/4/4) | Mining Rush: when a Ravage adds ≥1 Blight, add 1 Town to an adjacent land without Town/City (not from cascades) | EBR spread tied to blight events | D/c |
| 6 (8) | 13 (4/4/5) | Setup: +1 Town and +1 Blight (from the box) in land #8 per board | BLT cascade exposure (pool unchanged) | D |

- Royal Backing restated by the FAQ: "At the end of Setup (after resolving the initial Explore Card), Accelerate … On each board, add 1 Town to the land with fewest Invaders that matches the discarded card." — [querki FAQ (Combining Adversaries)][Q-CA]

**France (Plantation Colony)** (Branch & Claw; requires Strife) — [Wiki][W-FRA]; [ENG-ADV]; [querki FAQ (B&C)][Q-BC]. Loss "Sprawling Plantations": the Town supply is capped. The wiki says "return all but 7 Town per player to the box"; the engine says "limit of 7 Town per starting board". Invaders win if a Town can't be placed. Escalation "Demand for New Cash Crops": after Exploring, on each board, pick a land of the shown terrain. If it has Town/City, add 1 Blight; otherwise add 1 Town.

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (2) | 9 | Escalation; Town-cap loss | EBR, BLT, RACE | D/c, K |
| 1 (3) | 9 (3/3/3) | After a successful Explore into a land with no Town/City, add 1 more Explorer (not during Setup) | EBR explore yield (2 Explorers) | D |
| 2 (5) | 10 (3/4/3) | Setup: "Slave Rebellion" Event goes under the top 3 Event cards. After a Build in a land with ≥2 Explorers, replace all but 1 Explorer with Towns | EBR build output; Event deck gets a known card at a known depth | D, H |
| 3 (7) | 11 (4/4/3) | Setup: +1 Town in the highest-numbered land without Town, +1 Town in land #1 per board | EBR initial state, RACE (Town cap) | D |
| 4 (8) | 12 (4/4/4) | Triangle Trade: building a Coastal City adds 1 Town to the adjacent land with the fewest Towns | EBR | D/c |
| 5 (9) | 13 (4/5/4) | Slow-Healing Ecosystem: removed Blight goes on the adversary panel, not the Blight Card. It returns to the card once 3 per player have collected | BLT (effective pool shrinks with removals) | D, K |
| 6 (10) | 14 (4/5/5) | After each Explore Card, +1 Explorer per board in a land without any. Fear Cards never remove Explorers (Push instead) | EBR; FEAR-card effect value drops | D/c |

- Slave Rebellion timing without the Event deck: "on Turns 4 and 8 and, in unusual circumstances, Turn 12." — [querki FAQ (B&C)][Q-BC]

**Habsburg Monarchy (Livestock Colony)** (Jagged Earth) — [Wiki][W-HLC]; [ENG-ADV]; [JE p.22][JE]. Loss "Irreparable Damage": count Blight taken off the Blight Card during Ravages with 6+ overkill (wiki wording: "8+ Damage"). If the count ever exceeds the number of players, Invaders win. Escalation "Seek Prime Territory": after Exploring, on each board with ≤4 Blight, add 1 Town to a land without Town/Blight; on boards with ≤2 Blight, do it again.

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (2) | 9 | Escalation; Irreparable Damage loss | EBR, BLT, RACE | D/c, K |
| 1 (3) | 10 (3/4/3) | After the Build Step, each land matching a Build Card Gathers 1 Town from a non-matching land (board/land order) | EBR (Towns move toward the next Ravage) | D |
| 2 (5) | 11 (4/5/2) | Setup: +1 Town land #2, +1 Town in the highest-numbered land without setup symbols. A Build of 1 City in an Inland land becomes 2 Towns | EBR | D |
| 3 (6) | 12 (4/5/3) | Remove 1 more Stage I → **11-2222-33333** (11) | INV, RACE −1 | H, K |
| 4 (8) | 12 (4/5/3) | Towns in lands without Blight are Durable: +2 Health; "Destroy Town" becomes 2 Damage per Town | EBR kill math, FEAR | D |
| 5 (9) | 13 (4/6/3) | Reminder card under the top 5 invader cards. On reveal, each board: +1 City in a Coastal land without City, +1 Town in each of the 3 Inland lands with the fewest Blight | EBR spike at a known time | D |
| 6 (10) | 14 (5/6/3) | Ravages do +2 damage (total) if any adjacent land has a Town | EBR Ravage damage → BLT | D |

- Wave of Immigration timing: it resolves after the invader card above it is fully resolved, before the "After the Explore Step" Escalation. — [querki FAQ (Combining Adversaries)][Q-CA]

**Habsburg Mining Expedition** (Nature Incarnate) — [NI rulebook p.12][NI]; [Wiki][W-HME]; [ENG-ADV]; [querki FAQ (NI)][Q-NI]. Loss "Land Stripped Bare": at the end of the Fast Phase, any land with ≥8 Invaders+Blight combined. Escalation "Mining Tunnels": after advancing invader cards, on each board Explore in 2 lands whose terrain matches no Ravage or Build card (no source required).

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 | Mining Tunnels; 8-piece loss | EBR (extra explores), RACE | D/c, K |
| 1 (3) | 9 (3/3/3) | Avarice Rewarded: a Ravage-caused cascade instead Upgrades 1 Explorer/Town. Ceaseless Mining: lands with ≥3 Invaders are Mining lands; there, Build Cards cause Ravage Actions (not Builds) and Disease affects Ravages | EBR (build/ravage swap is state-dependent), BLT (cascades → upgrades) | D |
| 2 (4) | 10 (3/3/4) | Setup: +1 Explorer in each land with no Dahan; per board +1 Disease and +1 City in the highest-numbered land with a Town setup symbol | EBR initial state | D |
| 3 (5) | 11 (3/4/4) | Mining Boom I: after the Build Step, on each board choose a land with an Explorer and Upgrade 1 there | EBR | D/c |
| 4 (7) | 12 (4/4/4) | Remove Stage II "Coastal Lands" before randomizing. "Salt Deposits" (S) replaces the 2nd Stage II card → **111-2S22-33333**. S matches non-Mining lands in Explore/Build and Mining lands in Ravage. Once in the Ravage space it never leaves (an extra Ravage every turn, resolved first) and can't be discarded or Accelerated away | INV (Coastal impossible, S at a known position), EBR (permanent Ravage of all Mining lands) | D, H |
| 5 (9) | 13 (4/5/4) | Mining Boom II: choose a land with an Explorer, Build there, then Upgrade 1 | EBR | D/c |
| 6 (10) | 13 (4/5/4) | During Setup and Explore, on boards with ≤3 Blight, +1 Explorer in each successfully explored land (max 2 lands per board per Explore Card) | EBR explore yield | D/c |

- If an effect would Accelerate S out of the deck, discard the next Stage II card under it instead (or the top Stage III if no Stage II remains). — [querki FAQ (NI)][Q-NI]
- Ceaseless Mining, Mining Boom and Empire Ascendant apply only to the normal Build/Explore steps, not to extra steps such as England's High Immigration. — [querki FAQ (Combining Adversaries)][Q-CA]

**Russia** (Jagged Earth; requires Beasts) — [Wiki][W-RUS]; [ENG-ADV]; [JE p.22][JE]. Loss "Hunters Swarm the Island": Beasts destroyed by adversary rules go on the panel. If the panel ever holds more than the island, Invaders win. Escalation "Stalk the Predators": on each board, add 2 Explorers among lands with Beasts (or on another board's Beast lands if impossible).

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 | Escalation; Beast-count loss | EBR, RACE | D/c, K |
| 1 (3) | 10 (3/3/4) | Setup: per board +1 Beast and +1 Explorer in the highest-numbered land without Town/City. Explorers deal +1 damage (2 each). When a Ravage adds Blight (cascades included), destroy 1 Beast there | EBR Ravage damage → BLT; RACE Beast clock | D |
| 2 (4) | 11 (4/3/4) | The first time each Action would destroy Explorers, 1 is Pushed instead and gives +1 Fear | EBR (Explorer survival), FEAR (extra fear per action) | D |
| 3 (6) | 11 (4/4/3) | Ravage Cards also match lands with ≥3 Explorers (Ravage once per card) | EBR (Ravage set depends on state) | D |
| 4 (7) | 12 (4/4/4) | 1 Stage III after each Stage II → **111-2-3-2-3-2-3-2-33** | INV (alternating stages), Event stage flips | D |
| 5 (9) | 13 (4/5/4) | An unused Stage II card goes under the top 3 Fear Cards and an unused Stage III under the top 7. When revealed, it goes face-up straight into the Build space | INV, FEAR→EBR coupling | **H**, R |
| 6 (11) | 14 (5/5/4) | After the Ravage Step of turn 2+, on each board where Ravage added no Blight: the land with the most Explorers (min 1) gets +1 Explorer and +1 Town | EBR (punishes Ravage prevention) | D/c |

**Scotland** (Promo Pack 2 / Feather & Flame) — [Wiki][W-SCO]; [ENG-ADV]; [JE p.19 sidebar][JE]. Loss "Trade Hub": Coastal lands with a City > 2 × number of boards. Escalation "Ports Sprawl Outward": on the one board with the most Coastal Town/City, add 1 Town to each of the N lands with the fewest Towns (N = number of players).

| Lvl (Diff) | Fear deck | Rule | Models | Type |
|---|---|---|---|---|
| 0 (1) | 9 | Escalation (scales with N); Trade Hub loss | EBR, RACE | D/c, K |
| 1 (3) | 10 (3/4/3) | After Setup, in Coastal lands Explore Cards add 1 Town instead of 1 Explorer. The "Coastal Lands" card does this in at most 2 lands per board | EBR (coastal explore yields Towns) | D/c |
| 2 (4) | 11 (4/4/3) | Setup: +1 City land #2. "Coastal Lands" becomes the 3rd Stage II card; the two Stage II cards above it move up by one → **11-22-1-C2-33333** | INV (position 6 known = Coastal; Stage I reappears at position 5), Event stage | D, H |
| 3 (6) | 13 (4/5/4) | In Coastal lands, Build Cards affect lands without Invaders if there is an adjacent City | EBR build target set | D |
| 4 (7) | 14 (5/5/4) | Setup: replace the bottom Stage I card with the bottom Stage III card → **11-22-3-C2-3333** (11) | INV, RACE −1 | D, K |
| 5 (8) | 15 (5/6/4) | After a Ravage adds Blight to a Coastal land, add 1 Blight to that board's Ocean (no cascade). The Ocean counts as a Coastal Wetland for this and for Blight removal | BLT (extra drain per coastal blight event) | D |
| 6 (10) | 16 (6/6/4) | After the Ravage Step, +1 Town in each Inland land that matches a Ravage card and is within range 1 of Town/City | EBR | D |

#### Conflicts between sources
- **England L6 difficulty:** 11 on the wiki ([W-ENG]). 10 in the engine ([ENG-ADV]) and in the base rulebook difficulty table ([BASE]). Unresolved. A later printing may have changed it, but that is unverified.
- **France Town cap unit:** "7 Town per player" (wiki [W-FRA]) vs. "per starting board" (engine [ENG-ADV]). These are the same in normal games. They differ with an extra board, where JE p.18 says to base the pool "off of the number of boards, not the number of players" ([JE p.18][JE]).
- **Sweden L1 / Habsburg Livestock loss wording:** "≥6 Damage" / "8+ Damage" (wiki) vs. "4+ / 6+ overkill" (engine, [NI p.14][NI]). They agree on a 2-Health land. They diverge when land Health is modified, and the FAQ confirms the Health-adjusted reading ([Q-JE][Q-JE]).
- **Russia L1 name:** "Hunters Bring Home Shell and Hide" (wiki) vs. "Hunters Seek Shell and Hide" (engine). Cosmetic.

#### Project data errors found (verified against the sources above)
- `data/adversaries.json`
  - France `difficulty_levels` is `[2,3,5,6,8,9,10]`. L3 should be **7**: `[2,3,5,7,8,9,10]` ([W-FRA]; [ENG-ADV]).
  - Habsburg Mining Expedition is `[1,3,5,6,7,9,10]`. It should be **`[1,3,4,5,7,9,10]`** ([NI p.12][NI]; [W-HME]).
  - England L6 = 10 matches the engine and base rulebook but not the wiki (11). Flag it as unresolved.
  - BP, Sweden, Russia, Scotland and Habsburg Livestock match both sources.
- `data/adversary-stacks.json`
  - **Structural bug:** the `levels` keys are difficulty values copied from `adversaries.json`, not levels 0–6 (England keys "0,1,3,4,6,7,9,10"; Russia has key "11"). The BP key "6" holds the true **L6** stack (3-222-3333), but in this file's scheme key 6 means difficulty 6 (= BP L3).
  - Wrong stacks:
    - BP L2–L5 should be 111-3-2222-3333 / 11-3-2222-3333 / 11-3-222-3333 / 1-3-222-3333.
    - Russia L4+ should be 111-2-3-2-3-2-3-2-33 (L5 also places invader cards in the fear deck).
    - Habsburg Livestock L3+ should be 11-2222-33333.
    - Scotland L2–3 should be 11-22-1-C2-33333 and L4+ 11-22-3-C2-3333.
    - Habsburg Mining L4+ should be 111-2S22-33333.
    - Sweden L4+ adds a post-setup Accelerate.
  - England and France never change the deck, so their `[VERIFY]` flags can be cleared. England L3+ adds an invader **slot**, not a deck change.
- `data/references/adversaries/*.json`: all eight files use difficulty numbers as level numbers. Most level names and effects are invented. Examples:
  - england.json: L1 "Contempt" (actual: Indentured Servants Earn Land), L2 "Endless Influx" (actual: Criminals and Malcontents), L3 "Shores of Wealth" (actual: High Immigration I), L4 "Puppet Government" (actual: High Immigration full), and fictitious L6 "Ironclads" / L7 "Steamrolled" (England has L1–L6 only).
  - brandenburg-prussia.json: L2 "Mercenaries: Cities +1 damage" (actual: Surge of Colonists, a deck change).
  - sweden.json: L2 "Fear-Dampened Colonies" (actual: Population Pressure at Home).
  - france-plantation-colony.json: L2 "Slavery", L3 "Sugar" (actual: Slave Labor, Early Plantations).
  - russia.json and scotland.json: names such as "Frontier Warlords", "Second Sons", "Clearances" are not in any source.
  - Only fragments are right: England "+1 HP to Town/City" (the L5 effect; the name is wrong), BP "+1 Town at setup" (L1), and Sweden L3 "Town 3 / City 5 damage" (missing from the file entirely).
  - Treat these files as unusable; regenerate them from the tables above.
- `docs/superpowers/specs/2026-10-18-steam-companion-design.md` §7.2 says "adversary-specific loss conditions are not modeled in v1". Six of eight adversaries have one (England, France, both Habsburgs, Russia, Scotland; BP and Sweden have none), so a v1 race chart is silently wrong against those six.

### Inferences

**Model-change matrix: adversaries.** Each cell says what the model must add. Inferred from the rules above.

| Model | What adversaries change | Closed form? |
|---|---|---|
| INV | Per-position stage pools. Number unseen-removed per stage: base 1/1/1; BP L3 2/1/1, L4 2/2/1, L5 3/2/1, L6 4/2/1; HLC L3 2/1/1; Scotland L4 2/1/1 plus the bottom Stage III moved up. Known cards at known positions: Scotland L2+ C at position 6, HME L4+ S at position 5. Cards seen early: Sweden L4 Accelerate. Russia L5: one unseen Stage II and one unseen Stage III return from the "unused" pool, and their timing is set by the fear count. Within a stage, unseen cards stay exchangeable, so P(next = c) = 1/(#unseen of that stage) still holds once removed cards and known placements are counted in. Coastal is impossible against HME L4+ unless Scotland L2 returns it. | ✓ (needs a deck-recipe engine, not just stage counts) |
| EBR one-step | Rule plug-ins on the face-up cards: England L1/Scotland L3 widen Build targets; Russia L3 widens Ravage targets; HME Mining lands turn Build into Ravage, plus the permanent S Ravage; extra Build steps (England HI ×1 or ×2); damage modifiers (Sweden L3, Russia L1, HLC L6, England L5 Health, HLC L4 Durable); Escalations; post-step adds (Sweden L5, Scotland L6, Russia L6, HME L3/L5, HLC L1 Gather, France L2/L4/L6). | ✓ deterministic, but tie-break choices need candidate sets or a worst case |
| EBR multi-step | Every rule above depends on board state. Combined with the unrevealed next card, multi-turn projection compounds nonlinearly. | ✗ SIM |
| FEAR | Cards per TL (table above). Threshold per card 4N, or 5N against England L6. Russia L2 adds fear per Explorer-destroy action. Russia L5 puts invader cards inside the fear deck, so fear progress triggers Builds. | ✓ for "fear to next card" and "cards to next TL"; ✗ SIM if fear rate is projected jointly with board state |
| BLT | Extra blight sources: Sweden L1 overkill, France escalation, Scotland L5 Ocean. Sweden L6 adds setup Blight from the box. France L5 diverts removed Blight. HME L1 converts Ravage cascades to upgrades. HLC loss counts Blight from 6+ overkill Ravages. | One-step ✓; pool trajectory ✗ SIM |
| PWR, ELEM | None of the eight panels touches power decks or elements (per the tables). | unchanged |
| RACE | Deck length / exhaustion turn (inferred from "If there is no card to turn up … the spirits lose" [W-SOP]): with D cards after setup changes, setup reveals card 1 and turn t reveals card t+1, so the loss comes at the turn-D Explore. Base D=12; BP L3–L6 11/10/9/8; HLC L3+ 11; Scotland L4+ 11; Sweden L4+ 11 effective. Adversary loss clocks: England 7-in-a-land, France Town cap, HLC overkill count > N, HME 8 pieces at end of Fast, Russia Beast ratio, Scotland coastal cities > 2B. | Deck clock ✓ exact; adversary clocks ✗ SIM (first passage of a board process). One-turn-ahead loss checks are exact given face-up cards. |

- Event "stage" is read from the top card of the deck ([JE p.12][JE]), so deck recipes change which Event branch fires: Russia L4 alternates II/III; Scotland L2 shows Stage I again at position 5. BP L2's early Stage III counts as Stage II. Event-branch odds therefore need the same recipe engine.
- Information gained: HME L4 and Scotland L2 remove randomness (one card known). BP L3–L6 and HLC L3 add randomness (more unseen-removed cards, and a terrain may never appear). Russia L5 converts the normally never-seen removed card into a card that will be seen.
- England L3 timing (inferred from the slot description): cards reach the High Immigration tile two turns after their Build turn. With an unmodified deck, HI Builds use Stage I cards on turns 3–5. At the end-of-turn-5 Advance, the first Stage II card slides onto the tile, which discards it, so turn 6 has no HI Build.
- HLC L5 timing: the reminder resolves after the 5th invader card. That card is revealed on turn 4, so the spike lands in turn 4's Explore step unless the deck is modified above it. Inference.
- Russia L5 reveal timing: the cards sit under the 3rd and 7th fear cards. Inference: they surface when the 3rd and 7th fear cards are earned, which against a 4/5/4 deck is TL I and TL II respectively.

### Gaps
- Which England L6 difficulty (10 or 11) is current was not settled. The physical panel wasn't checked.
- Whether "unused" Stage II/III cards for Russia L5 are chosen at random when more than one is unused (e.g., combined with BP L4) was not found in the sources. Treat it as random.
- Tie-break rules for "add to a land" choices (BP escalation, Sweden L5, Scotland escalation ties) were not found beyond "players make decisions" ([JE p.12][JE], Events). A forecast should show candidate sets.
- Exact page numbers of the base-rulebook adversary panels weren't captured: the RulesPal transcription has no pagination, and the official PDF fetched was image-only.

---

## Q2. Combined adversaries: leading vs. supporting, levels, escalations, deck changes, loss conditions, difficulty formula

### Takeaway
The official rules (JE p.19) combine two adversaries at independent levels. Use all changes from both, except: fear deck = 3 + Σ deltas per TL; the leading adversary's Escalation fires on Stage II and the supporting one's on **every Stage III card**; and deck instructions run supporting-first, then leading, with set tie-break rules. Difficulty ≈ the higher difficulty + 50–75% of the lower. The deck stays closed-form for INV. The Stage III escalation adds a recurring state change, and a random terrain pick for France/Sweden.

### Cited Findings
- Decide two adversaries and a level for each, and "use all game changes from both of them" with these exceptions. — [JE rulebook p.19][JE]; same text on [Wiki: Adversary][W-ADV]
  - **Fear deck:** "For each Terror Level, use 3 Fear Cards (the base) plus the extra cards added or subtracted by each Adversary. (So … a 3-4-3 Fear Deck and a 4-4-4 Fear Deck … 4-5-4.)" — [JE p.19][JE]
  - **Escalation:** "Choose one Adversary to be leading; its Escalation Effect applies normally. The other Adversary is supporting and applies its Escalation Effect on all Stage III Invader Cards. (If the Escalation is specific to a terrain – as with France and Sweden – choose one of the terrains randomly.)" — [JE p.19][JE]. The FAQ adds: don't use top/bottom terrain ("not equally distributed"); spin the card if needed. — [querki FAQ (Combining Adversaries)][Q-CA]
  - **Invader deck:** "first follow all of the instructions on the supporting Adversary, in order, and then follow all of the instructions on the leading Adversary, in order." Skip impossible instructions. If the card to remove is ambiguous, remove the top-most applicable card. If the card to move is ambiguous, move the bottom-most applicable card. BP L2 is read as "Move the bottom-most Stage III card just below the bottom-most Stage I card." — [JE p.19][JE]
  - **Worked example** (BP L4 leading + Russia L4 supporting): 111-2222-33333 → Russia 4: 111-2-3-2-3-2-3-2-33 → BP 2: 111-3-2-3-2-3-2-3-2-3 → BP 3: 11-3-2-3-2-3-2-3-2-3 → BP 4: **11-33-2-3-2-3-2-3** (10 cards). — [JE p.19][JE]
  - **Difficulty:** "roughly equal to the higher of the two Difficulties plus 50-75% of the lower … occasional exceptions … haven't tested every single combination." — [JE p.19][JE]
  - **Scotland sidebar:** if the other adversary's setup would add a City to a Coastal land other than #2, add it to an adjacent Inland land instead. — [JE p.19][JE]
  - **France sidebar:** vs. France L2+, add 1 Town per player to the Town pool for each level of the other adversary. — [JE p.19][JE]
- Combination FAQ rulings:
  - HME L4 + Scotland L2 gives **11-2S-1-C2-33333** whichever leads. The Coastal card removed by HME L4 can't be used by "random unused Stage II" effects (Russia L5) but can be placed by Scotland L2. — [NI p.12][NI]; [Q-CA]
  - A Sweden Escalation that lands on a non-Explore card (e.g., via Russia L5) does nothing. — [Q-CA]
  - Migratory Herders (HLC L1) is intended to check only the printed Build space, but checking the extra England HI card is also acceptable. — [Q-CA]
  - Royal Backing's Accelerate matters in combinations because the top card may be a later stage moved upward (e.g., against BP L5). — [Q-CA]
- The losing conditions of both adversaries apply ("use all game changes from both"). Each loss is checked per the NI timing errata. — [JE p.19][JE]; [NI p.13][NI]

### Inferences
| Model | Change under combination | Closed form? |
|---|---|---|
| INV | Need an executable deck-recipe interpreter: an ordered list of primitive ops (remove-unseen, move, replace, insert-known, Accelerate) applied supporting-first, plus the tie-break rules. After the recipe runs, per-position stage pools are still exchangeable within a stage. | ✓ |
| EBR | The supporting Escalation fires on **each** Stage III Explore (5+ times instead of once). Against France/Sweden it applies to a random one of the card's two terrains (P = ½ each). Forecast must branch on that coin. | One-step ✓ (two-branch mixture); multi-step ✗ SIM |
| FEAR | Cards per TL = 3 + Δ_lead + Δ_support per TL. Some deltas are negative (HLC L2+ TL III = −1 or 0), and the sum can make a TL group small. Threshold per card unchanged, except England L6's +N. | ✓ |
| BLT / RACE | Loss conditions add up, so the competing-risk set has up to 2 adversary clocks plus blight and deck. Deck length comes from the recipe (example: 10). | Deck ✓; joint race ✗ SIM |
| Difficulty display | Show a band: D_hi + 0.5·D_lo to D_hi + 0.75·D_lo (rulebook heuristic, not a probability). | n/a |

### Gaps
- No official per-pair difficulty table exists. The project's England chapter links a BGG "Combined Difficulty Chart" thread (boardgamegeek.com/thread/2483958); it is community-made and wasn't verified here.
- Rules for three or more adversaries were not found (the rules describe two).

---

## Q3. Expansions: what each adds or changes for these models, and how card pools change deck compositions

### Takeaway
Branch & Claw adds the largest structural changes. It adds a fourth hidden deck (Events; one random card per turn after turn 1) whose branches key off Blight-card side, terror level, or invader stage. It adds tokens that cancel Explore (Wilds), Build (Disease) or individual Invader damage (Strife). It makes Blight Cards mandatory. Jagged Earth adds Badlands (+damage), Isolate (blocks Explore and adjacency), "Still Healthy" blight cards (the blight clock becomes a two-stage mixture), the 5–6-player boards, and the play options (combined adversaries, extra board, archipelago, no Events). Nature Incarnate adds Incarna, Vitality (blocks Blight), the 8-piece adversary, a fixed **+1 Blight setup errata**, and retires five cards. No expansion adds new invader card types except HME's Salt Deposits. The hypergeometric pools are owned-set dependent: full collection minors = 100, majors = 78, fear = 50, events = 62 (61 random + France's Slave Rebellion), blight = 23.

### Cited Findings

#### Deck pool sizes by owned set (active cards)
| Deck | Base / Horizons | Branch & Claw | Jagged Earth | Promo 2 (F&F) | Nature Incarnate | All owned | Sources |
|---|---|---|---|---|---|---|---|
| Invader | 15 (4/5/6) | 0 | 0 | 0 | +1 Salt Deposits (HME L4 only) | 15 (+S) | [ENG-INV]; [NI p.12][NI] |
| Fear | 15 | 15 | 6 | 5 | 9 | **50** | [Wiki: Fear][W-FEAR] (50 active, 0 retired); engine count 15/15/6/5/9 |
| Event | 0 | 26 printed, **23** active | 30 | 0 | 9 (includes Far-off Wars, the replacement) | **62** active | [Wiki: Event Card][W-EVC]; [NI p.5][NI]; engine count 23/30/9 |
| Blight | 2 | 7 printed, **6** active | 7 | 0 | 8 | **23** active | [Wiki: Blight Card][W-BLC]; [NI p.5][NI] |
| Minor | 36 | 31 printed, **30** active | 33 | 0 | 1 (Roiling Bog and Stinging Thorn) | **100** | [Wiki: Minor Power][W-MIN]; [NI p.5][NI] |
| Major | 22 | 21 | 23 | 0 | 12 new (+1 corrected reprint of Draw Towards a Consuming Void) | **78** | [Wiki: Major Power][W-MAJ]; [Wiki: NI][W-NI]; [NI p.5][NI] |

- NI "Dropped Cards" (remove if you own B&C): Outpaced, War Touches the Island's Shores, A Strange Madness Among the Beasts (Events); Growth Through Sacrifice (Minor); Tipping Point (Blight). Replacements: Far-off Wars Touch the Island and Roiling Bog and Stinging Thorn. — [NI rulebook p.5][NI]
- Horizons contains "The 22 Major Power Cards and 36 Minor Power Cards from the Base Game", "The 15 Fear Cards from the Base Game", and "The same Invader Deck from the Base Game". — [Wiki: Horizons][W-HOSI]
- Promo Pack 2 adds Scotland, 2 scenarios, 5 fear cards and 5 aspects; Promo Pack 1 adds no fear, blight or event cards. — [Wiki: Promo Pack 2][W-P2]; [W-BLC]; [W-EVC]
- Adding only part of an expansion's power cards "near-certain[ly]" changes element distributions: "some Spirits will be invisibly penalized / rewarded." — [Wiki: Event (Q&A on partial use)][W-EV]; [querki FAQ (B&C)][Q-BC]
- Engine count conflict: the engine lists B&C Minor = 32, Major = 21, NI Minor = 0 ([ENG-PWR]), vs. the wiki's 31/21/1. The project's `data/decks/minor.json` (B&C 30, NI 1, total 100) matches the wiki after NI drops.

#### Branch & Claw (and the B&C rules reprinted in JE)
- **Event step:** "each Invader Phase, after any Blighted Island effect and before Resolving any Earned Fear Cards, draw and resolve one Event Card." "On the first turn of the game, draw an Event Card, but do not resolve it; just place it in the discard pile." — [JE p.12][JE]; [W-SOP]
- Events branch on (1) Healthy/Blighted, (2) Terror Level, or (3) Invader stage, read from the top card of the invader deck. Most also have a Token Event and a Dahan Event. Choice Events cost "Energy per player", "Aided by" an element. — [JE p.12–13][JE]
- Engine Event deck by branching variable (62 cards): Healthy/Blighted 25, Choice 15, Terror Level 12, Invader Stage 10 (Slave Rebellion included). — [ENG-EVT] (count computed from the file)
- Events that move clocks or decks (engine text) [ENG-EVT]:
  - Rising Interest in the Island returns the top invader card to the box.
  - Far-Off Wars Touch the Island and Foreign Dignitaries (Blighted) add a Fear Card to the top of the Fear Deck.
  - Overconfidence discards 2 earned Fear Cards or costs 2 Fear per player.
  - Terror Spikes Upwards resolves an earned Fear Card at TL+1.
  - Numinous Crisis can remove Blight until the card flips.
  - World's Vibrance Fades and Sacred Sites Under Threat remove 1 Blight per player from the card to the box. Power Fades removes 1 per spirit (choice). Self-Sacrifice Buys Time takes the next X Blight from the box.
  - Fortune-Seekers makes the next Explore match all lands without Town/City. Fortification adds an extra Build Step. Bureaucrats Adjust Funding skips or adds Builds.
  - Missionaries Arrive / New Species Spread / Years of Little Rain / Life's Balance Tilts discard the top Minor Power(s) as a randomizer. Missionaries and New Species return themselves under the top 2 Event cards. Lesser Spirits Imperiled discards 1 Minor per player.
- **Tokens:**
  - Wilds: "When Invaders would Explore into a land with Wilds, instead Remove one Wilds."
  - Disease: same for Build.
  - Strife sits on an Invader; that Invader "deals exactly 0 Damage and Removes one Strife" whenever it damages Dahan or the land. Dahan still fight back.
  - Beasts have no intrinsic effect.
  - Sources: [JE p.7][JE]; [W-WLD]; [W-DIS]; [W-STR]; [W-BEA]
- **Ravage order (JE p.7, "a change from existing rules"):**
  1. Sum individual Invader damage; apply per-Invader modifiers and Strife.
  2. If total ≥1, add per-land modifiers.
  3. Subtract Defend.
  4. Damage the land and the Dahan (Badlands +1 per token to Dahan).
  5. Surviving Dahan counterattack (Badlands +1).

  — [JE p.7][JE]
- **Setup:** 1 Beasts in the lowest-numbered land with no setup icons and 1 Disease in land #2 per board. Blight Card required with Events. — [JE p.6][JE]; [W-SETUP]
- France (Plantation Colony), 4 scenarios, 15 fear, 7 blight, 26 events, 21 major / 31 minor. — [Wiki: Branch and Claw][W-BC]

#### Jagged Earth
- 10 spirits, 2 adversaries (Russia, Habsburg Livestock), 3 scenarios, boards E/F ("allow games with up to 6 players"), 23 major / 33 minor, 30 events, 6 fear, 7 blight, 6 aspects, Badlands. Play options: combined adversaries, archipelago, extra board, playing without Events, fewer tokens. — [Wiki: Jagged Earth][W-JE]; [JE p.6][JE]
- **Badlands:** "The first time each Action Damages Invaders in a land, increase that Damage by 1 per Badlands" (same for Dahan). It only boosts Damage, not Destroy, and is mandatory. — [JE p.8][JE]; [W-BAD]. NI errata: it need not apply to the first Damage instance. — [NI p.13][NI]
- **Isolate:** "Invaders do not Explore Isolated lands and Isolated lands are not a source of Explorers." It does not affect cascades or Coastal status, and lasts one turn. — [Wiki: Isolate][W-ISO]
- **Still Healthy cards:** "When this Blight pool runs out, you do not lose – instead … draw a new Blight Card and go straight to its reverse side." Keep the whole shuffled blight stack on the space. — [JE p.9][JE]. Blight supply is unlimited (use proxies). — [JE p.9][JE]
- **Playing without Events:**
  - Add 1 extra Fear Card to TL II.
  - Put two "Command Beasts" reminder cards in the invader deck, before the first Stage II and before the first Stage III.
  - Don't use Blight Cards whose Blighted side is 2 per player or immediate-only.

  — [JE p.20][JE]
- **Fewer tokens:** after Setup, adding an unused token type instead gives 1 Energy and 1 Fear. — [JE p.21][JE]

#### Nature Incarnate
- 8 spirits (4 with Incarna), 20 aspects, Habsburg Mining Expedition, 2 scenarios (Destiny Unfolds, Surges of Colonization), 12 majors + 1 corrected reprint, 1 replacement minor, 9 events, 9 fear, 8 blight. Tokens: 24 Deeps, 20 Quake, 18 Vitality. — [Wiki: Nature Incarnate][W-NI]; [NI components][NI]
- **Blight errata:** "If you are using a Healthy Island card, treat it as if it said '2 per player, +1'. If you are not using a Healthy Island card, treat the Blight space as if it said '5 per player, +1' … purely a Setup change; there is no change for when the Blight card flips." — [NI p.5][NI]; [querki errata][Q-BLT]
- **Vitality:** "Each [Vitality] in a land with no [Blight], prevents 1 [Blight] from being added and is then Removed." — [NI p.8 / glossary][NI]; [W-VIT]. The second icon in the NI PDF didn't extract as text; the wiki renders it as Blight.
- **Incarna:** a special Presence token. Off-island it is "only an Incarna, not any of the pieces printed on it." Some Incarna are Empowered permanently. — [Wiki: Incarna][W-INC]
- NI blight cards add new **ongoing per-Invader-Phase** presence destruction (Attenuated Essence, Blight Corrodes the Spirit, The Border of Life and Death) and **Intensifying Exploitation** (+2 Ravage damage per land). — [ENG-BLT]; [W-BLC]
- Solo: "you may draw a new Blight Card if you flip one with 2 [Blight] on a Blighted Island side … Don't redraw Still-Healthy Island Cards." — [NI p.14][NI]

#### Blight-card pool (Blighted side, Blight per player; SH = Still Healthy)
From [W-BLC] and [ENG-BLT]:

| Per player | Cards |
|---|---|
| 1 | Border of Life and Death (SH) |
| 2 | Aid from Lesser Spirits, Back Against the Wall, Burn Brightest, Shattered Fragments, Invaders Find the Land (SH), Strong Earth Shatters Slowly (SH), Thriving Crops (SH) |
| 3 | A Pall Upon the Land, Erosion of Will, All Things Weaken, Unnatural Proliferation, Slow Dissolution of Will |
| 4 | Memory Fades to Dust, Promising Farmlands, Power Corrodes the Spirit, Thriving Communities, Untended Land Crumbles, Attenuated Essence, Blight Corrodes the Spirit |
| 5 | Downward Spiral, Disintegrating Ecosystem, Intensifying Exploitation |

- The engine gives every card's Healthy side as 2 per starting board.
- Clock-relevant immediate effects:
  - Erosion of Will: +2 Fear per player.
  - Invaders Find the Land: raises the fear needed per card by +1/+1.5/+2 per player at TL I/II/III.
  - Promising Farmlands: +1 Town and +1 City per board.
  - Disintegrating Ecosystem: +1 Blight per board.
  - Strong Earth: +1 Blight per player adjacent to Blight.
  - Thriving Crops: Build in 3 lands per board.
  - Unnatural Proliferation: +2 Cities per board.

### Inferences
**Model-change matrix: expansions.**

| Model | Change | Closed form? |
|---|---|---|
| INV | No new invader card types (only S). Events/Fear that touch the deck: Rising Interest returns the top card unseen (−1 deck, +1 unseen-removed); Explorers are Reluctant (B&C fear) skips or adds an Explore; Fortune-Seekers rewrites the next card's match set. Odds stay a ratio of unseen counts. | ✓ |
| EBR | Tokens make per-land outcomes depend on token counts (Wilds/Disease cancel one action each; Strife zeroes chosen Invaders; Badlands +1/token/action; Vitality blocks a Blight; Beasts appear via Events). One-step stays exact but needs the JE p.7 Ravage order. The next Event (one of ~60 unseen) can add Builds or Ravages, change damage, or move pieces before Ravage. | One-step ✓ conditional on the Event; a "next Invader Phase" forecast needs a mixture over the unseen Event deck (finite and exchangeable). Multi-step ✗ SIM |
| Event deck (new hidden deck) | Unseen pool = owned active Events − discards (turn-1 discard included) − France's Slave Rebellion (known depth) − Guard the Isle's Heart removals. Self-returning cards (Missionaries, New Species, Slave Rebellion) create known-depth cards. Which branch fires depends on public state (Blight side, TL, stage), so P(next Event's effective text) is closed-form over unseen cards. | ✓ for card odds; effects ✗ SIM |
| FEAR | Fear pool grows with owned sets (draws stay uniform from the pool). Deck-length changes: no-Events option +1 TL II card; Events add cards to top; Overconfidence removes earned cards; Erosion of Will and Invaders Find the Land change fear and threshold on Blight flip. | Card counts ✓; timeline ✗ SIM once Events are in |
| BLT | Pre-flip pool = 2N+1 (errata). Post-flip refill = X·N, where X is a random draw from the unseen Blight cards. That is a finite mixture: in a full collection with no exclusions, `E[X]` = 73/23 ≈ 3.17 and X ∈ {1,…,5}. SH cards add a recursive second draw, already flipped. Events remove card Blight to the box (shortens the clock) or take Blight from the box (lengthens it). Ongoing NI/base effects destroy Presence each Invader Phase. Solo redraw rule changes the X distribution at N=1. | Flip-time X distribution ✓ (finite mixture); cascade/drain trajectory ✗ SIM |
| PWR | Unseen pool = owned active cards − drops − visible cards (hands, discards, played, forgotten, Event-discarded randomizers). Blight cards draw extras (Aid from Lesser Spirits: N+1 minors; Shattered Fragments: N+2 majors). Events discard top minors to the public discard. Each such reveal is a hypergeometric update, not a model break. | ✓ |
| ELEM | Events "Aided by" an element (elements count as Energy toward costs only). NI flexible element spaces and multi-card aspects change a spirit's element supply. Power Corrodes the Spirit punishes ≥3 cards in play or a 4+ cost card, which changes the value of reaching thresholds. Partial expansion pools shift element frequencies [W-EV]. | Threshold check ✓ (deterministic); P(reach threshold next turn) via PWR ✓ |
| RACE | New clocks: Blight-flip timing (SH cards), Event-driven fear/blight jumps, Presence-destruction clocks (a spirit at 0 Presence; the base-game loss rule wasn't fetched, see Gaps). | ✗ SIM |

- The Turn-1 Event is drawn and discarded **without resolving** ([JE p.12][JE]), so it is public information that shrinks the unseen Event pool by 1. The project memory note "first Event card is face-up during Setup/T1 but doesn't resolve until T2" (`feedback_event_card_turn1_preview.md`) does not match the printed rule. Check what the Steam game actually does before coding either behavior.

### Gaps
- Whether the power-deck discard is reshuffled into a new deck when the deck runs out, and how that interacts with draw odds, was not verified in this pass. It is needed for a correct PWR model late in long games.
- Exact Event-card counts per branching type come from the engine only. They weren't cross-checked against the wiki or physical cards.
- The exact Horizons fear/blight board numbers per player count come only from a secondary site (see Q4).
- Aspect effects were not cataloged. Aspects replace a spirit's special rule or innate or change its setup ([W-ASP]). They touch ELEM and EBR for that spirit but not the shared decks.

---

## Q4. Player count (1–6 spirits/boards), solo vs. multi-handed, extra boards: how fear pool, blight, boards/lands, invader spread, setup pieces and difficulty scale

### Takeaway
Most scaling is linear in the number of players N: one board per player, 4N fear per card, 2N+1 Blight before the flip, X·N after. The invader deck (12 cards) does **not** scale, so the deck clock is identical at every player count. Each terrain card hits 2 lands per board and Coastal Lands 3 per board, so Explore/Build/Ravage volume scales with boards. Multi-handed solo is mechanically an N-player game ("mechanics scale with the number of Players, not the number of people"). The JE extra-board option decouples boards from players: Blight scales with boards, fear gets +1 per player, and setup invaders on the extra board depend on N. "Per board" and "per player" effects then diverge, and every model that reads N must distinguish B (boards) from N (players).

### Cited Findings
- **Boards:** "Randomly pick one Island Board per player"; "The game is played with one island board per player … eight numbered lands, with exactly two of each terrain." Lands touching at a corner are adjacent. Lands adjacent to the Ocean are Coastal. — [BASE][BASE]
- **Balanced board facts (project data, wiki-sourced):** every balanced board (A–H) has 8 lands, coastal lands #1–3, 2 lands of each terrain, and setup of 1 City + 1 Town + 1 Blight. Thematic boards have 9–10 lands with uneven terrain counts. — `data/boards/*.json` (source field: spiritislandwiki.com Island_Boards) [W-IB]
- **Fear:** "Put 4 Fear Markers per player into the Fear Pool." — [BASE][BASE]; [W-FEAR]. Fear from destroying a Town = 1, a City = 2 (does not scale). — [W-FEAR]
- **Blight:**
  - Healthy side "generally 2 per player" + 1 (errata); no Blight Card: 5 per player + 1. — [W-BLC]; [NI p.5][NI]
  - On flip, add N·X more (no +1). — [querki FAQ (Core)][Q-CORE]
  - Why the +1 exists: without it, "Usable Blight without flip" was 1, 3, 5, 7, 9, 11 for 1–6 players (1.0 to 1.83 per player). The +1 makes usable Blight per player constant. — [querki errata][Q-BLT]
- **Solo:** "a single board as the whole island … you can target yourself with Powers that specifically target 'Another Spirit' … the luck of the draw is high." — [BASE][BASE]. Solo players may redraw a Blight Card whose Blighted side is 2 per player. — [NI p.14][NI]; [W-BLC] (citing B&C p.17)
- **Multi-handed:** "one person can play as many Spirits as they'd like. However, that one person is taking on the roles of multiple Players … The game mechanics scale with the number of Players, not the number of people who are participating." Exceptions: A Diversity of Spirits (no effect if one person plays) and Destiny Unfolds (multi-handed "considerably reduces the Difficulty"). — [querki FAQ (Core)][Q-CORE]
- **5–6 players:** JE boards E/F; standard layouts "Crab" (5) and "Two Centers" (6), plus alternates. The layout affects difficulty: land #3 not adjacent to another board means worse cascades; an unconnected tip (#8) creates safe zones. — [JE p.6, p.16][JE]. Avoid pairing E+B and F+D with ≤4 boards (also G+C and H+A for Horizons boards). — [W-IB]
- **Archipelago:** facing-Ocean coastal lands are Range 2 apart. Targeting a spirit on another islet costs 1 Energy per Ocean jump. "+1 Difficulty at most." — [JE p.17][JE]
- **Extra Island Board** — [JE p.18][JE]
  - "One additional board adds approximately +2-4 Difficulty … +2 at Difficulty 0, +3 at Difficulty 3, and +4 at Difficulty 6."
  - Game-wide: "Base the size of your Blight pool on the number of Island Boards rather than the number of players, both during Setup and when flipping a Blight Card. Add +1 Fear Marker per player to the Fear pool. 'Each Board' effects happen once per board. 'Each player' effects happen once per player."
  - Extra-board setup: Dahan, Blight and tokens normal. No adversary setup Invaders or Blight. Other setup Invaders by player count:

    | Players | Setup Invaders on the extra board |
    |---|---|
    | 1 | none (no initial Explore either) |
    | 2 | initial Explore only |
    | 3 | Coastal City + initial Explore |
    | 4 | Inland Town + initial Explore |
    | 5 | as normal |

  - "Larger French Plantation Colony: Base the size of the Town pool off of the number of boards."
- Extra board + scenarios: Powers Long Forgotten, Blitz, Second Wave, Great River and Ward the Shores switch "per player" to "per board" (Great River loss: "more than (3 × # of boards) Towns escape"). — [querki FAQ (Extra Island Board)][Q-XB]
- Dream of the Untouched Land adds a board mid-game. If no board is available, you don't get to skip Builds or "Each Board" adversary actions. — [Q-JE]
- **Per-player vs. per-board text in adversaries** (see Q1 tables):
  - Per player: Scotland Escalation (N lands), HLC loss (> N), France Town pool (per player on wiki, per board in engine), England L6 (+1 Fear per player).
  - Per board: Scotland loss (2 × boards). Most setup and Escalation effects run "on each board".
- **Fear cards:** "each player" effects scale with players: "each player gets a choice so that the effects of the Fear Card scale properly with the number of players." — [querki FAQ (Horizons)][Q-HOSI]
- **Events:** Choice costs are "some amount of Energy per player"; "On each board" lets you choose differently per board. — [JE p.12–13][JE]
- **Horizons (secondary source; mark unverified):** a two-sided game board for 2 or 3 players; Blight pool = 5 × players + 1 (6/11/16 for 1/2/3); Fear pool = 4 × players. — [officialgamerules.org Horizons][OGR-HOSI]. The +1 errata "does not affect the board from Horizons of Spirit Island." — [Q-BLT]
- **Difficulty from player count:** no official per-player-count difficulty adjustment was found. The base rulebook's only statement is the qualitative solo note above; JE gives the extra-board +2 to +4 and archipelago ≤ +1.

### Inferences
**Scaling table.** N = players (spirits); B = boards (B = N unless the extra board is used, then B = N+1). Derived from the cited rules.

| Quantity | Formula | Effect on models |
|---|---|---|
| Fear per card | 4N; +N extra board; +N England L6; +k·N for Invaders Find the Land (k = 1/1.5/2, round down at TL II) | FEAR threshold ✓ closed form |
| Fear cards in deck | 9 + adversary deltas (no N) | FEAR: total fear to victory = threshold × cards, linear in N |
| Blight before flip | 2B+1 (Blitz +1 per board; Horizons 5N+1 with no card; no card 5B+1) | BLT ✓; usable-before-flip per board = 2 (errata) |
| Blight after flip | X·B, X ~ unseen Blight cards (solo may redraw X=2 non-SH) | BLT mixture ✓ |
| Lands | 8B on balanced boards (thematic 9–10 per board) | EBR scales with B |
| Lands matched per invader card | Stage I/II terrain: 2B; Coastal Lands: 3B; Stage III: 4B | INV per-land P(explored) is unchanged per land; expected new Explorers ∝ B |
| Invader deck | 12 (adversary/scenario-dependent, never N) | RACE deck clock is independent of N |
| Setup Invaders/Blight | per board (1 City, 1 Town, 1 Blight on balanced; adversary adds per board; extra board per table) | EBR initial state ∝ B |
| Escalations / adversary adds | mostly "on each board" (∝ B); Scotland Escalation ∝ N | EBR |
| Adversary loss thresholds | Scotland 2B; HLC > N; France 7N (or 7B) | RACE |
| Events | "on each board" ∝ B; Choice costs ∝ N | EBR, decision costs |

- Variance (inference): the deck clock and the per-card terrain draws are the same at every N, so the fraction of the island hit per card is the same. With B=1, though, a single unlucky draw (e.g., a Stage III card hitting both lands a spirit is building in) is a larger share of the whole game. This matches the rulebook's "luck of the draw is high" for solo. A model should report per-land probabilities, not island totals, so N-invariance is visible.
- Solo two-handed vs. one spirit: two-handed is an exact 2-player game (B=2, fear 8 per card, Blight 5 before flip, 2X after). The FAQ makes it so, but its claim is about mechanics, not equal difficulty. The extra-board option is the only official way to run 1 spirit on 2 boards. That gives fear 4+1=5 per card, Blight 2·2+1=5 before flip, the extra board starts with no Invaders, and difficulty rises by about 2–4.
- The companion must store B and N separately and tag every rule's scaling unit. Without that, extra-board, archipelago (cross-islet targeting cost) and multi-handed games will silently mis-scale fear and Blight.

### Gaps
- The base-game rule on Presence loss (a spirit with no Presence) and its interaction with player count wasn't fetched. It is relevant to the Presence-destruction clocks of several Blight cards.
- No official difficulty numbers exist for 1 vs. 2 vs. 4 players. Community win-rate data is outside this assignment.
- Horizons' printed 2/3-player board numbers come from a secondary site only. The rulebook wasn't obtained.
- How the Steam game handles extra-board and archipelago options wasn't checked.

---

## Q5. Scenarios that change clocks, decks, or victory/loss conditions

### Takeaway
About half the scenarios change a clock or the information structure:
- Rituals of Terror, Ward the Shores and Dahan Insurrection replace the fear victory clock.
- Surges of Colonization changes how many invader cards are revealed per turn. Elemental Invocation Accelerates the deck. Despicable Theft Accelerates it randomly via hidden Thief numbers.
- Blitz changes the Blight pool and every power's speed. Guard the Isle's Heart and The Great River add loss conditions.
- Destiny Unfolds replaces power draws with drafted pools. Powers Long Forgotten adds hidden locations.

Project `data/scenarios.json` has wrong difficulty for Great River, Varied Terrains and Destiny Unfolds, and is missing Surges of Colonization.

### Cited Findings
Sources: the engine `ScenarioList.json` [ENG-SCN], and the wiki for Surges, Destiny and Blitz ([W-SOC], [W-DU], [W-BLITZ]).

| Scenario (Diff) | Rule change relevant to models | Models | Type |
|---|---|---|---|
| Blitz (0) | All Powers Fast; already-Fast powers cost 1 less. Once per board when Exploring, +1 Explorer. Setup: +1 Blight per board on the card; Invaders get an extra Build+Explore at the end of Setup. On flip, 1 less Blight per board | EBR, BLT, ELEM (no Slow phase) | D, K |
| Guard the Isle's Heart (0) | Extra loss: after turn 1, any Town/City in an Inner Land (land #7 at 1–2 players). Setup removes all Towns; +1 Explorer and +1 Presence in Inner Lands; each spirit draws 1 Minor + 1 Major. Removes the Events "Bureaucrats Adjust Funding" and "Invested Aristocracy" | RACE, PWR, Event pool | K |
| Rituals of Terror (3) | Terror Levels come only from the Ritual (land with 1 Presence per spirit, ≥3 Dahan per player, pay 3 Energy + destroy 3 Presence). Fear Cards don't resolve in the Invader Phase. All Fear Cards go in the deck; when it runs out, earn no more | FEAR replaced, RACE | K |
| Dahan Insurrection (4) | No normal fear victory. TL2: no land where Town/City outnumber Dahan. TL3+: fewer than 1 such land per player. Extra loss: fewer than 2 Dahan per player. Moved Dahan deal 1 damage each; destroying Town/City adds Invaders near Dahan | RACE, EBR | K |
| Second Wave (1) | Follow-up game: if the Blight Card stayed Healthy, adversary level +1. If it flipped, the refill uses the Blight saved from game 1, not the printed X. Set-aside powers from game 1 usable from Stage III | BLT (X known), RACE | H (X known) |
| Powers Long Forgotten (1) | 4 hidden locations per board; 1 per board + 1 contain a power source. Searched when a land has ≥3 Dahan (spirits) or ≥2 Invaders (Invaders; 3 vs. France) | new hidden RV | R, H |
| Ward the Shores (2) | No fear victory. TL2+: a Warding Pattern in every Coastal land. All Fear Cards in the deck, reshuffled when it runs out. Stage III Explore also adds 1 Town | FEAR, RACE, EBR | K |
| Rituals of the Destroying Flame (3) | Invaders immune to Damage/Destroy unless a Flame Marker is within 1 or 2 Energy is paid (also for Dahan counterattacks, Events, Fear) | EBR kill math, FEAR | D |
| Despicable Theft (2) | Thieves (12 blank + 1,1,1,2,2,3) arrive with Explores into lands #6+. Escaping Thieves cost 4 earned Fear each and Accelerate the deck by their hidden number | FEAR (fear can go down), INV/RACE (random deck shortening) | R, H, K |
| Elemental Invocation (1) | Setup: +1 Town land #6, +1 Fear Card TL I, Accelerate the deck. Element Markers on land corners add elements to land-targeting powers and count as 2 Energy toward "Aided by" Choice Events | ELEM (land-dependent elements), RACE −1 | D, K |
| The Great River (3) | Boards stacked with Oceans on one side. Extra Builds/Explores in Coastal lands; Towns migrate inland each turn. Extra loss: more than 3 Towns per starting board escape off the east edge | EBR, RACE | D, K |
| A Diversity of Spirits (0) | Communication limits only. No effect if one person plays | — | — |
| Varied Terrains (2) | Mountains +1 Explorer when Explored; a City built in a Jungle adds Blight; Sands cascade as if already Blighted; Wetlands Build twice | EBR, BLT (Sands cascade) | D |
| Destiny Unfolds (−1*, usually 0 to −2) | Each spirit drafts a Destiny (2 rounds × 6 cards: 4 minor + 2 major, then 5 minor + 1 major). Gaining a power takes a card from the Destiny instead of the deck while possible | PWR replaced by known choice; deck depletion | H |
| Surges of Colonization (+2/+7, NI) | Explorers are an Explore source. Even turns resolve 1 extra Explore Card (Larger Surges: 2 extra); odd turns skip the first normal Explore. Setup resolves 1 extra. Final Surge uses unused/discarded Stage III cards. Larger Surges adds 1 Fear Card per TL | INV (multi-card draws), EBR, RACE | D, K |

- Surges explore schedule with an unaltered deck: "1+1 / / 1+2 / / 2+2 / / 2+3 / / 3+3 / / 3+3 / / loss". Larger Surges: "1+1 / / 1+2+2 / / 2+2+3 / / 3+3+3 / / 3+(3)+(3) / / loss". — [W-SOC]
- If a scenario and an adversary contradict, "the Scenario takes precedence." Scenario difficulty runs 0 to 10. — [BASE][BASE]
- Extra board: the scenarios listed in Q4 switch "per player" to "per board". — [Q-XB]

#### Project data errors (`data/scenarios.json`)
- `the-great-river` difficulty_mod 1 → engine says **3** ([ENG-SCN]).
- `varied-terrains` 0 → engine says **2** ([ENG-SCN]).
- `destiny-unfolds` 2 → wiki says **−1\*** (varies 0 to −2) ([W-DU]).
- **Surges of Colonization** (NI, +2/+7) is missing ([W-SOC]).
- The other ten values match the engine.

### Inferences
- **INV under Surges:** each even turn reveals 2–3 cards in a row, so "next-card odds" becomes a joint draw without replacement from the per-position pools. A multivariate hypergeometric over the same exchangeable pools stays closed-form ✓. The loss turn changes: with Larger Surges and a 12-card deck, the wiki schedule ends at turn 10 instead of 12 (read from the schedule string; inference).
- **Despicable Theft:** the deck-exhaustion clock becomes random (Thief numbers 1,1,1,2,2,3 among 18). The distribution of total Acceleration needs simulation ✗, because escapes depend on Thief movement and on the spirits' ability to destroy Thieves.
- **Rituals of Terror / Ward the Shores / Dahan Insurrection:** the FEAR timeline model must be swapped for a scenario-specific victory predicate. Rituals of Terror also decouples fear from the terror level, so "fear to next card" no longer predicts terror.
- **Second Wave:** after the flip, X is known (Blight carried over), which removes the hidden Blight-card side.
- **Destiny Unfolds:** while a Destiny holds cards, P(gain card with element E) is 0 or 1 (a choice from a known set). Hypergeometric odds only resume once the Destiny is empty.

### Gaps
- Great River and Varied Terrains difficulty come from the engine only (not wiki-checked).
- Whether the Steam game implements Surges of Colonization and Destiny Unfolds wasn't checked. Neither is in the engine scenario list, which has 13 scenarios.
- Guard the Isle's Heart Inner Land numbers for 3–4 players are images in the engine text and were not captured.

---

[W-ADV]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Adversary
[W-ENG]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=England
[W-BP]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Brandenburg-Prussia
[W-SWE]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Sweden
[W-FRA]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=France
[W-HLC]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Habsburg_Monarchy
[W-HME]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Habsburg_Mining_Expedition
[W-RUS]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=The_Tsardom_of_Russia
[W-SCO]: https://web.archive.org/web/2023/https://spiritislandwiki.com/index.php?title=Scotland
[W-BLC]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Blight_Card
[W-FEAR]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Fear
[W-EVC]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Event_Card
[W-EV]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Event
[W-SOP]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Sequence_of_Play
[W-SETUP]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Game_Setup
[W-DIS]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Disease
[W-STR]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Strife
[W-WLD]: https://web.archive.org/web/2023/https://spiritislandwiki.com/index.php?title=Wilds
[W-VIT]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Vitality
[W-BEA]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Beasts
[W-BAD]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Badlands
[W-ISO]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Isolate
[W-INC]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Incarna
[W-NI]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Nature_Incarnate
[W-BC]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Branch_and_Claw
[W-JE]: https://web.archive.org/web/2023/https://spiritislandwiki.com/index.php?title=Jagged_Earth
[W-P2]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Promo_Pack_2
[W-HOSI]: https://web.archive.org/web/2023/https://spiritislandwiki.com/index.php?title=Horizons_of_Spirit_Island
[W-SOC]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Surges_of_Colonization
[W-DU]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Destiny_Unfolds
[W-BLITZ]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Blitz
[W-MIN]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Minor_Power
[W-MAJ]: https://web.archive.org/web/2024/https://spiritislandwiki.com/index.php?title=Major_Power
[W-IB]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Island_Boards
[W-ASP]: https://web.archive.org/web/2025/https://spiritislandwiki.com/index.php?title=Aspect
[BASE]: https://rulespal.com/spirit-island/rulebook
[JE]: https://boardgame.bg/spirit%20island%20jagged%20earth%20rules.pdf
[NI]: https://boardgame.bg/spirit%20island%20nature%20incarnate%20rules.pdf
[Q-CORE]: https://querki.net/raw/darker/spirit-island-faq/Core+Game
[Q-JE]: https://querki.net/raw/darker/spirit-island-faq/Jagged+Earth
[Q-BC]: https://querki.net/raw/darker/spirit-island-faq/Branch+%26+Claw
[Q-NI]: https://querki.net/raw/darker/spirit-island-faq/Nature+Incarnate
[Q-CA]: https://querki.net/raw/darker/spirit-island-faq/Combining+Adversaries
[Q-XB]: https://querki.net/raw/darker/spirit-island-faq/Extra+Island+Board
[Q-HOSI]: https://querki.net/raw/darker/spirit-island-faq/Horizons+of+Spirit+Island
[Q-BLT]: https://querki.net/raw/darker/spirit-island-faq/one-more-blight-errata
[OGR-HOSI]: https://officialgamerules.org/game-rules/horizons-of-spirit-island/
[ENG-ADV]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.AdversaryList.json
[ENG-INV]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.InvaderCardList.json
[ENG-EVT]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.EventCardList.json
[ENG-BLT]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.BlightCardList.json
[ENG-PWR]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.PowerCardList.json
[ENG-SCN]: file:///tmp/claude-1000/-home-brett-repos-spirit-island/191ccfe4-5c3d-49f2-8151-00bb6ac6137d/scratchpad/meta/Handelabra.SpiritIsland.Engine.Definitions.ScenarioList.json
