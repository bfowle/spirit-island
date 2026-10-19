# Rules register: the invader pipeline

Every rule the companion's invader forecasts (Explore, Build, Ravage, blight,
cascade, deck clock) depend on, with its sources. **Statuses are mixed since
Brett's 2026-10-19 answers (`open-questions.md`): 39 entries are `reviewed by
Brett`, 29 are `sourced`, 242 are still `unreviewed`** (all 310 Status
lines, including the one-line Event, Fear, Blight-card and loss-condition
entries; an entry with both a Brett answer and a sourced one counts as
reviewed). Status values are defined in section 0. Machine-readable twin:
`tools/si-live/common/data/invader-rules.json` (the adversary, spirit,
scenario and Event entries; its `status` field keeps the engine-vs-Wiki
agreement, and a `review` field, plus `resolution` for a settled conflict,
carries the review). Open questions found in this pass are in section 8; they
are merged into `open-questions.md`, which holds the answers.

## 0. How to read this register

**Entry format.** Rule (what the companion should apply) · Changes (the
pipeline element, as a `kind` from the JSON schema) · Engine evidence · Wiki
evidence · Agreement · Resolution (only where an answer settled a conflict or an open
point) · Affects (companion feature) · Status.

**Agreement values.**

| Value | Meaning |
|---|---|
| `agree` | Engine and Wiki state the same rule (wording may differ; differences are noted). |
| `conflict` | Engine and Wiki differ. Each was an open question; an entry an answer settled keeps `conflict` (the engine-vs-Wiki fact) and adds a **Resolution** line. |
| `engine-only` | Only engine evidence was available to this pass (Wiki page missing, not archived, or not parsed). It does **not** mean the Wiki is silent. |
| `wiki-only` | The Wiki states it; the engine logic was not read (method names at most). |

**Status values.**

| Value | Meaning |
|---|---|
| `unreviewed` | Not yet checked against the physical copy or the rulebooks. |
| `reviewed by Brett (date, Q#)` | Settled by Brett's own answer (physical copy or rulebook) to that question in `open-questions.md`. |
| `sourced (date, Q#; source)` | Settled from the rulebooks, FAQ or card text, with Brett's permission (`open-questions.md` "Answer (sourced …; Brett delegated)"). |

An entry can carry both (one part answered by Brett, another sourced); a parenthetical names any part still open or not asked.

**Engine evidence.** Definitions extracted from Brett's installed
`SpiritIslandEngine.dll` (assembly 1.0.9776.27799, SHA-256
`eac8e2edd55a116ae6a94bef0ba2cc4ff1d17dcfdebab6e063651a35893fe750`, checked
against the installed file on 2026-10-19), as JSON keyed by engine identifier:
`adversaries.json`, `spirits.json`, `scenarios.json`, `event_cards.json`,
`fear_cards.json`, `blight_cards.json`, `power_cards.json`,
`invader_cards.json`. Quotes are verbatim except that formatting tags (`<i>`,
`<b>`, `<nobr>`, `{BR}`, `{Bullet}`) are removed; `{Town}`-style tokens are
the engine's icon tokens. Claims about engine *logic* (method names, IL) are
cited from `research_notes/Spirit Island companion stat models/engine_state_model.md`
(Q1, Q3), which read the IL; this pass did not re-read IL.

**Wiki evidence.** spiritislandwiki.com answers every scripted request with a
Cloudflare challenge (HTTP 403), including `scripts/wiki-fetch.py` and curl
(2026-10-18/19). So Wiki text comes from two places:

1. **Archived pages**: raw web.archive.org snapshots (`id_` mode, no Wayback
   rewriting) of the Wiki pages. The revision id is the page's `wgRevisionId`;
   the snapshot time is listed in Appendix A. Cited as `Page@revid`.
   `Sequence_of_Play@12144` and `Explorer@7992` are the same revisions the
   project saved earlier in `explore-phase.md`.
2. **Repo fetcher output** from 2026-04-19: spirit pages
   (`data/references/wiki/*.json`) and the event, fear and blight caches
   (`/home/brett/repos/spirit-island/.wiki-cache/`). The fetcher does not
   record revision ids, so these are cited as `Name@unrecorded`.

Wiki quotes: icons appear as `{Name}` (the image's alt text); image-file icons
such as `{Fasticon.png}` are omitted in section 1–3 quotes; ` / ` joins list
items; `…` marks omitted text. Section 4 Wiki quotes are the level text as
archived.

**Not used as evidence:** `data/references/adversaries/*.json` and
`data/references/scenarios/*.json`. They are hand-written stubs (open-questions
says "level names and effects mostly invented"). Example: the Blitz stub says
"Skip turn 1 of the invader deck"; the engine and the Wiki both say the
Invaders get an *extra* Build and Explore at the end of Setup (R-SC-Blitz-3).

**Inferences** (conclusions drawn from two or more sourced facts, stated by no
single source) are labeled *(inference)*.

**Order.** Sections 1–3 are the core rules. Sections 4–5 group by expansion:
base → Branch & Claw → Promo Pack 1 → Jagged Earth → Promo Pack 2 → Horizons
→ Nature Incarnate. Section 6 covers interactions across layers.

## 1. Turn structure and the invader pipeline

**R-TS-1 · Phase order of a turn**

- **Rule:** Spirit Phase (Growth, Gain Energy, play and pay for Power Cards) → Fast Power Phase → Invader Phase (Blighted Island effect → Event → Fear → Ravage → Build → Explore → Advance Invader Cards) → Slow Power Phase → Time Passes. With England level 3+, the High Immigration Build comes after Fear and before Ravage.
- **Changes:** phase anchor for every forecast
- **Engine:** `Constants.StateOrdering` = [PlayerSetup, InvadersStartingAction, SpiritPhase, FastPower, BlightedIsland, Event, Fear, HighImmigration, Ravage, Build, Explore, AdvanceInvaderCards, SlowPower, TimePasses] (DLL `Constants::.cctor` + FieldRVA; engine_state_model.md Q1).
- **Wiki:** Sequence_of_Play@12144: "Spirit Phase - Growth - Gain Energy - Play and pay for Power Cards / Fast Power Phase (Cards and Innate) / Invader Phase - Blighted Island Effect - Event (except first turn) - Fear Effects - Ravage … - Build … - Explore … - Advance Invader Cards / Slow Power Phase (Cards and Innate) / Time Passes". Blight@7555: "Events happen AFTER any Blighted Island effect and before any Earned Fear Cards."
- **Agreement:** `agree`
- **Affects:** every forecast's anchor step ("before this Ravage", "after this Explore")
- **Status:** unreviewed

**R-TS-2 · The card pipeline across turns**

- **Rule:** Each Invader Card acts on three consecutive turns. Turn t: it is revealed and Explores. Turn t+1: it is in the Build space and Builds. Turn t+2: it is in the Ravage space and Ravages. Then it goes to the discard. So next turn's Build and Ravage cards are already face-up; only the next Explore card is hidden.
- **Changes:** card flow (slots `ExploreSlot` → `BuildSlot` → `RavageSlot` → `DiscardSlot`)
- **Engine:** `AdvanceInvaderCardsStateController.RunState` advances the slots toward `DiscardSlot`; log `AdvanceInvaderCardsAction: CardMovements=(Stage I Mountain, Invader Deck, Build Space)` (engine_state_model.md Q3). Slots are save fields (spec §5 visibility table).
- **Wiki:** Sequence_of_Play@12144 §4: "After Explore, slide all of the Invader Cards left: Move the Ravage Card to the discard, the Build Card to the Ravage Space, and the Explore Card to the Build Space, so that next turn they will Ravage where they Built this turn and Build where they Explored this turn."
- **Agreement:** `agree`
- **Affects:** land-tempo badges (`ahead`, `on`, `b2b`); "if X is next" timelines
- **Status:** reviewed by Brett (2026-10-19, Q24)

**R-TS-3 · Order inside one Invader Phase (oldest card first)**

- **Rule:** Inside one Invader Phase the oldest card acts first: the Ravage card, then the Build card, then the newly revealed Explore card, then the cards advance. *(inference)* So a land that this turn's Explore card hits gets its Explorer *after* this turn's Build; it builds next turn, not this turn. If an action space is empty, that action does not happen.
- **Changes:** step order
- **Engine:** `Constants.StateOrdering` "… Ravage, Build, Explore, AdvanceInvaderCards …".
- **Wiki:** Sequence_of_Play@12144 §3: "The Invader Board has three Action Spaces: Ravage, Build, and Explore. The Invader Card in an Action Space determines which lands will be affected by that Action. If there is no Invader Card in a space, no action is taken." Steps are numbered 3a Ravage, 3b Build, 3c Explore, 4 Advance.
- **Agreement:** `agree`
- **Affects:** invader forecast step order
- **Status:** sourced (2026-10-19, Q28; core rulebook p.10)

**R-TS-4 · A card's lands are fixed before it resolves**

- **Rule:** When an Invader Card resolves, first note every land it affects, then resolve them all (in any order). Pieces that card adds do not change which lands it affects: Towns added by an Explore (Scotland level 1, the Event Promising Farmland) are not Explore sources for that same Explore. The same holds for Ravage and Build.
- **Changes:** `explore_source_change` (timing of the source test)
- **Engine:** `ResolveInvaderCardAction` holds `LandsMatchingInvaderCard` and `CanInvaderActionBePerformedInLand` (names only; IL not read).
- **Wiki:** Invader@9428, FAQ: "When Explore adds Towns - eg, Scotland 1, or the Event "Promising Farmland" - are those Towns a source of Explorers? No. When resolving any Invader Card, first you note which lands are affected, then you resolve all of them - in an order of your choice, if relevant. This is also true for Ravage and Build."
- **Agreement:** `wiki-only`
- **Affects:** Explore eligibility
- **Status:** sourced (2026-10-19, Q28; JE rulebook p.27, quoted in the FAQ)

**R-TS-5 · Setup's initial Explore**

- **Rule:** At the end of Setup the Invaders reveal the top card, Explore in its land type, and put it face-up in the Build space.
- **Changes:** card flow at Setup
- **Engine:** `Constants.StateOrdering` has `InvadersStartingAction` between `PlayerSetup` and `SpiritPhase`. Test 1 (open-questions Q24): the setup card advanced to Build.
- **Wiki:** Game_Setup@8832: "To finalize the Setup, the Invaders take an initial Action. Reveal the top card of the Invader Deck. The Invaders Explore in that land type (see Explore, page 10). Then place that card face-up in the 'Build' Action Space."
- **Agreement:** `agree`
- **Affects:** Turn 1 Build forecast
- **Status:** reviewed by Brett (2026-10-19, Q24; core rulebook p.5)

**R-TS-6 · When the first Build and the first Ravage happen**

- **Rule:** Turn 1 Builds where Setup explored, and its Explore reveals card 2. Turn 2 Ravages where Setup explored and Builds where card 2 explored. Card 2 Ravages on Turn 3. So the first Ravage is generally Turn 2, and Turn 1 is the time to prevent Builds (Q24; core rulebook p.5: the setup Explore card goes to Build). Exceptions Brett named: Sweden L4 (NQ-A) and Blitz, whose extra Build and Explore at the end of Setup puts the setup card in the Ravage space on Turn 1 (NQ-J: inference from the card text, to verify in a Steam Blitz game).
- **Changes:** card flow
- **Engine:** Test 1 live observation (open-questions Q24): "Mountain built Cities on Turn 1, ravaged on Turn 2".
- **Wiki:** derived from Sequence_of_Play@12144 §4 and Game_Setup@8832 (quoted above); no page states it directly.
- **Agreement:** `agree` (derived). Q24 confirmed it. The Shadows chapter already says Turn 2; `skills/si-rules-check/SKILL.md` still says the first Ravage is Turn 3 and needs fixing.
- **Affects:** Turn 1–3 forecasts; opener advice
- **Status:** reviewed by Brett (2026-10-19, Q24); Blitz clause sourced (2026-10-19, NQ-J; inference to verify in Steam)

**R-TS-7 · Deck exhaustion loss**

- **Rule:** If the Explore step has no card to reveal, the Spirits lose. This is a state check, not a fixed count: the loss comes when an Explore must resolve and no card is left, and that Invader Phase still Ravages and Builds first, so a win there (e.g. through Defend or a Dahan counterattack) comes before the failed Explore (Q11). *(inference)* With D cards in the deck after setup changes and nothing changing the deck during play (Setup reveals card 1; Turn t reveals card t+1), the loss comes at Turn D's Explore. Base D = 12.
- **Changes:** `loss_condition` (deck clock)
- **Engine:** `EndingResult.LoseTimeRunsOut` exists; `GameController::GetCurrentInvaderStage` returns 3 when the deck is empty. The loss trigger was not read in IL (engine_state_model.md Q3 gaps).
- **Wiki:** Sequence_of_Play@12144 §3c: "If there is no card to turn up, time has run out and the spirits lose the game."
- **Agreement:** `wiki-only`. Q11 answered.
- **Affects:** deck clock in the race chart
- **Status:** reviewed by Brett (2026-10-19, Q11)

**R-TS-8 · Base Invader Deck**

- **Rule:** 3 Stage I, then 4 Stage II, then 5 Stage III cards (12). The engine's card pool has 4 Stage I (one per terrain), 5 Stage II (four terrains + Coastal Lands) and 6 Stage III (two-terrain) cards, so one card per stage is left out unseen (Q1: Stage I Jungle, Mountain, Sands, Wetland; Stage II the same four with the Escalation icon, plus Coastal Lands; Stage III the six two-terrain pairs). Stage III cards match both of their terrains.
- **Changes:** `deck_recipe` (base)
- **Engine:** `Constants.NumberOfStage1InvaderCards = 3`, `NumberOfStage2InvaderCards = 4`, `NumberOfStage3InvaderCards = 5`; `invader_cards.json`: `Jungle1`, `Mountain1`, `Sands1`, `Wetland1` (stage 1); `Jungle2`, `Mountain2`, `Sands2`, `Wetland2`, `Coastal2` (stage 2); `JungleSands3`, `JungleWetland3`, `MountainJungle3`, `MountainWetland3`, `SandsMountain3`, `SandsWetland3` (stage 3, two `landTypes` each).
- **Wiki:** Game_Setup@8832: "The Invader Deck is 12 cards: 3 Stage I cards on top, then 4 Stage II cards, then 5 Stage III cards on the bottom".
- **Agreement:** `agree` (deck size and order); the 15-card pool is `engine-only`.
- **Affects:** Explore card counting (unseen pool per stage); deck clock
- **Status:** reviewed by Brett (2026-10-19, Q1)

**R-TS-9 · Escalation**

- **Rule:** Only the four single-terrain Stage II cards carry the Escalation flag (Coastal Lands does not). With an Adversary, its Escalation runs each time a flagged card is resolved for the first time, at its Explore. The general rule says "first": unless an Escalation states its timing, it happens before the Explore (FAQ "Stage II escalation", NQ-G). But several adversaries set their own timing: after Exploring (Sweden, France, Habsburg Livestock) or after Advancing (Habsburg Mining); see section 4 and X-2. With two Adversaries, the supporting one's Escalation runs on Stage III cards.
- **Changes:** `escalation`
- **Engine:** `invader_cards.json`: `Jungle2`, `Mountain2`, `Sands2`, `Wetland2` have `"escalation": true`; `Coastal2` has none. Names `AdversaryController.Escalation`, `GetNumberOfEscalationsThatShouldOccur`; log `Source: Stage II Jungle {Escalation}` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3c: "If the card has a flag icon and you are playing with an Adversary, first perform the Escalation effect." Adversary@12228: "The Adversary Panel specifies an Escalation Effect, which is performed when a Stage II Invader Card with a {Escalation} is resolved for the first time. Escalation Effects are also performed when Stage III Invader Cards are resolved for the first time when playing against two Adversaries."
- **Agreement:** `agree`
- **Affects:** invader forecast on Escalation cards
- **Status:** reviewed by Brett (2026-10-19, Q1, Q19); sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-TS-10 · Event step**

- **Rule:** With Events, one Event card is drawn and resolved each Invader Phase, after the Blighted Island effect and before Fear. On Turn 1 it is discarded unresolved (settled by Brett, open-questions "Turn-1 Event").
- **Changes:** Event sections in 5.3 apply at this step
- **Engine:** `EventStateController.RunState`: if `ShouldResolveEventState`, `ResolveEventCard` on `EventDeck.TopCard`, then `MoveCard` → `EventDiscard`; `StateOrdering` puts `Event` after `BlightedIsland` and before `Fear` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144: "If playing with Events, draw and resolve one Event Card. (Exception: On the first turn, discard the Event Card without resolving it.)"
- **Agreement:** `agree`
- **Affects:** invader forecast after the Event reveal
- **Status:** reviewed by Brett (2026-10-18, Turn-1 Event; 2026-10-19, Q8)

**R-TS-11 · Fear step, before the Invader Actions**

- **Rule:** Earned Fear Cards resolve after the Event and before Ravage, in the order earned, at the current Terror Level. Their effects last only for the current turn. So a Fear Card's skip, Defend or Isolate applies to this turn's Ravage, Build and Explore (families in 5.4). The Terror Level changes the moment the divider is reached, even while Fear Cards resolve: later cards use the new level, the card resolving is not bumped up, and a card earned while resolving goes to the bottom of the stack being resolved (Q7; FAQ "Fear and Fear Cards"). Earned cards stay face-down until this step; the Steam log names them when earned, so the firewall redacts them (Q8).
- **Changes:** Fear effects in 5.4 apply at this step
- **Engine:** `FearStateController` → `ResolveAllFearCards`: `FlipStack(FearEarned)`, then `ResolveFearCard` on the top card in order; `StateOrdering` puts `Fear` before `HighImmigration` and `Ravage` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §2: "If any Fear Cards have been Earned, pick up the whole facedown stack, flip it over and resolve the cards one at a time in the order they were earned. … Use the Fear Card's effect listed next to the current Terror Level - this could be higher than it was when the card was earned. Fear Card effects last only for the current turn."
- **Agreement:** `agree`
- **Affects:** invader forecast after the Fear reveal
- **Status:** reviewed by Brett (2026-10-19, Q7, Q8)

**R-TS-12 · Time Passes clears single-turn effects**

- **Rule:** At the end of each turn, Damage, Defend and Elements clear and single-turn effects end. Defend, Isolate and other "this turn" effects never reach the next Invader Phase. Exception: Shroud of Silent Mist's Slow and Silent Death (R-SP-ShroudOfSilentMist-1).
- **Changes:** effect lifetime
- **Engine:** `TimePassesAction`, in order: discard played cards; `SetDamageTaken(0)` on all damage targets; `IslandLocation::ClearDefense` on every land and clear the badland damage keys; expire status effects; `AdjustCardPlays`; `LoseElements`; reset growth; `IncreaseTurnIndex` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144: "All Elements go away. All Damage done during the turn goes away"; Defend@3397: "Defend effects last for the entire turn."; Isolate@4040: "Like all effects not marked by a change to the board, Isolate only lasts for the current turn."
- **Agreement:** `agree`
- **Affects:** which effects the forecast carries to next turn
- **Status:** unreviewed

**R-TS-13 · Current Invader Stage**

- **Rule:** The current Invader Stage is the stage of the card in the Explore slot, or III when the deck is empty.
- **Changes:** input to Stage-keyed effects (Events by Stage, Accumulated Devastation, Ward the Shores Stage III Surge)
- **Engine:** `GameController::GetCurrentInvaderStage` returns the stage of the top `ExploreSlot` card (`GetInvaderStageFromCard`), or 3 if the deck is empty, then applies `AskForInvaderStage` (engine_state_model.md Q3).
- **Wiki:** no page fetched in this pass states it.
- **Agreement:** `engine-only`
- **Affects:** Stage-keyed Event and scenario effects
- **Status:** unreviewed

## 2. Explore, Build, Ravage

### 2.1 Explore

**R-EX-1 · Which lands are explored**

- **Rule:** Add 1 Explorer to every land of the shown type that contains a Town or City, or is adjacent to a Town, City, or Ocean.
- **Changes:** base Explore
- **Engine:** `GameController::GetLandsToExploreFrom` source predicate: the land is not isolated AND (`IsOcean` OR `HasInvaders(explorers:null, towns:true, cities:true)`), plus `AskForExploreSourcesToAdd` (AskType 30), minus `AskForExploreSourcesToRemove` (31). `IsSourceLandInExploreRange` is true if source = target, else if the target is within `GetExploreRange()` = 1 + `AskForExploreRange` of a source (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3c: "Add an Explorer to every land of the shown type which either: Contains a Town or City; or Is adjacent to a Town, City, or Ocean."
- **Agreement:** `agree`
- **Affects:** Explore odds per land
- **Status:** reviewed by Brett (2026-10-19, Q3)

**R-EX-2 · One Explorer per land**

- **Rule:** One Explorer per explored land, however many sources there are, and whether or not the land already has pieces.
- **Changes:** base Explore
- **Engine:** `ExploreAction.NumberOfPiecesToAddByType : Dictionary<IslandPieceType,int>`; the default of 1 Explorer is inferred in engine_state_model.md, not read.
- **Wiki:** Sequence_of_Play@12144 §3c: "No matter how many sources are in or adjacent to an Explored land, you only add one Explorer." Explorer@7992, FAQ: "Yes, if there is a source of invaders, you add one Explorer, whether or not there is anything there already."
- **Agreement:** `agree` (engine default inferred)
- **Affects:** Explore forecast (pieces added)
- **Status:** reviewed by Brett (2026-10-19, Q3)

**R-EX-3 · Explorers are not a source**

- **Rule:** A land with only Explorers does not seed Explore into its neighbors. Towns and Cities are sources, in their own land and in adjacent lands.
- **Changes:** base Explore sources
- **Engine:** source predicate `HasInvaders(explorers:null, towns:true, cities:true)`.
- **Wiki:** Explorer@7992: "Explorers are not themselves a source when Invaders Explore." Town@7993: "Towns act as a source of Explorers {Explorer} when Invaders Explore." City@7994: "Cities act as a source of Explorers {Explorer} when Invaders Explore."
- **Agreement:** `agree`
- **Affects:** Explore eligibility
- **Status:** reviewed by Brett (2026-10-19, Q3)

**R-EX-4 · Coastal lands always have a source: the Ocean**

- **Rule:** Every Coastal land touches an Ocean, so it passes the source test on a matching card whatever its buildings. It can still escape the Explore if the land is Isolated, its Ocean is Isolated, it has Wilds, or a rule takes Coastal lands out of the Explore (Fear Card Quarantine). Brett's "coasts always explore" is the source test; these are its exceptions.
- **Changes:** base Explore sources
- **Engine:** `IsOcean` in the source predicate; isolated sources are excluded.
- **Wiki:** Sequence_of_Play@12144 §3c: "venturing forth from Towns and Cities or approaching from the Ocean". Isolate@4040: "If Oceans are in play, Isolating an Ocean doesn't affect whether lands are Coastal, but does prevent it from being a source of {Explorer}."
- **Agreement:** `agree`
- **Affects:** Explore odds for Coastal lands
- **Status:** reviewed by Brett (2026-10-19, Q3); sourced (2026-10-19, Q27; JE rulebook "Wilds")

**R-EX-5 · Inland lands can be Explore-locked**

- **Rule:** *(inference from R-EX-1 and R-EX-3)* An Inland land with no Town or City in it or in any adjacent land is not explored, even on a matching card. Explorers left there do not break the lock. Clearing the Towns/Cities around an Inland land makes the lock hold.
- **Changes:** base Explore
- **Engine:** same predicate as R-EX-1.
- **Wiki:** Isolate@4040, worked example: "Lands #6 and #7 do not Explore, because they are not adjacent to a source of Explorers."
- **Agreement:** `agree`
- **Affects:** Explore odds; "lock" suggestions on the threat map
- **Status:** reviewed by Brett (2026-10-19, Q3; derived)

**R-EX-6 · Isolate**

- **Rule:** An Isolated land is not explored and is not an Explore source. For Invader pieces it is adjacent to nothing, unless the players allow it. Isolate does not change whether a land is Coastal and does not affect blight cascades. It lasts one turn.
- **Changes:** `isolate`
- **Engine:** the Explore source predicate excludes isolated lands (observed). Whether isolated *targets* are excluded, and how cascade adjacency treats Isolate, was not read.
- **Wiki:** Isolate@4040: "Invaders do not Explore Isolated lands and Isolated lands are not a source of Explorers." "As far as Invader pieces are concerned, an Isolated land isn't adjacent to anything... unless you want it to be." "Isolate does not affect Blight cascades - Blight is not an Invader piece, so is not impacted." "Isolate does not change whether a land is Coastal."
- **Agreement:** `agree` for sources; `wiki-only` for targets, piece adjacency and cascades (section 8, NQ-F, NQ-M)
- **Affects:** Explore eligibility; cascade targets
- **Status:** reviewed by Brett (2026-10-19, Q3; the engine reads NQ-F and NQ-M are still open)

**R-EX-7 · Wilds**

- **Rule:** When Invaders would Explore into a land with Wilds, they remove 1 Wilds instead of adding an Explorer. Wilds do not stop the land from being a source for its neighbors (Q27).
- **Changes:** `explore_change` (token)
- **Engine:** invader actions can be canceled by tokens (`InvaderAction.DoesTokenCancelAction`, `_tokenToCancel`, `TokenSelfCancelResponse`); names only, Wilds amounts not read.
- **Wiki:** Wilds@3164: "When Invaders would Explore into a land with {Wilds}, instead remove one {Wilds} from that land." Sequence_of_Play@12144 (BC/JE update): "Explore (Including Wilds)".
- **Agreement:** `wiki-only`
- **Affects:** Explore forecast per land
- **Status:** sourced (2026-10-19, Q27; JE rulebook "Wilds")

**R-EX-8 · Pieces added before the Explore can be sources for it**

- **Rule:** Unless an Escalation states its timing, it happens before the Explore (FAQ "Stage II escalation": "This means that, e.g., Brandenburg-Prussia can add a Town that then acts as a source for the Exploration that happens immediately afterwards."). So Towns/Cities added by Brandenburg-Prussia (Land Rush), Scotland (Ports Sprawl Outward) and England (Building Boom) are sources for that same Explore. Russia's adds Explorers, which are never sources. Escalations that run after Exploring (Sweden, France, Habsburg Livestock) do not change that Explore.
- **Changes:** `explore_source_change`
- **Engine:** not read.
- **Wiki:** Sequence_of_Play@12144 §3c: "first perform the Escalation effect". Rulings: England@6844, Brandenburg-Prussia@6845, Russia@6851, Scotland@12772: "… Escalation effect happens before Exploring."; Sweden@12523, France_(Plantation_Colony)@7940, Habsburg_Monarchy_(Livestock_Colony)@6850: "… happens after Exploring." The earlier project note `explore-phase.md` states the same inference.
- **Agreement:** `wiki-only` (NQ-G answered from the FAQ)
- **Affects:** Explore odds on Escalation turns
- **Status:** sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-EX-9 · Explore range**

- **Rule:** Explore range is 1 (adjacent). The Event Distant Exploration (Terror Level I) makes it 2 for that Explore step.
- **Changes:** `explore_source_change`
- **Engine:** `GetExploreRange` = 1 + `AskForExploreRange`.
- **Wiki:** Sequence_of_Play@12144 §3c: "Is adjacent to a Town, City, or Ocean." distant-exploration@unrecorded (event cache): "During the Explore step, Invaders Explore at +1 distance. (So a land 2 away from a Town/City/Ocean will be Explored into.)"
- **Agreement:** `agree`
- **Affects:** Explore eligibility
- **Status:** unreviewed

**R-EX-10 · The Coastal Lands card**

- **Rule:** The Stage II Coastal Lands card matches every Coastal land, whatever its terrain.
- **Changes:** base Explore/Build/Ravage matching
- **Engine:** `invader_cards.json` › `Coastal2`: `"landTypes": ["Coastal"]`, stage 2. `GetLandsToExploreFrom`: "If the card's land types include Coastal (5), the ordering is special-cased" (engine_state_model.md Q3).
- **Wiki:** no fetched page states it (Scotland@12772 level 1 refers to "Coastal Lands" Invader cards).
- **Agreement:** `engine-only`
- **Affects:** Explore odds on the Stage II pool
- **Status:** unreviewed

### 2.2 Build

**R-BU-1 · Which lands Build**

- **Rule:** Build in each land of the shown type that has any Invader; an Explorer alone is enough. Lands without Invaders do not Build. Exceptions in this register: England L1, Scotland L3, the Event Bureaucrats Adjust Funding, the blight card Thriving Crops.
- **Changes:** base Build
- **Engine:** `ResolveInvaderCardAction.CanInvaderActionBePerformedInLand`, `BuildInLand` (names; the "has Invaders" test was not read in IL).
- **Wiki:** Sequence_of_Play@12144 §3b: "Wherever there are Invaders in the specified type of land, they add either one City or one Town … Do not Build in lands without Invaders."
- **Agreement:** `wiki-only`
- **Affects:** Build forecast
- **Status:** reviewed by Brett (2026-10-19, Q2)

**R-BU-2 · Town or City**

- **Rule:** If the land has more Towns than Cities, add a City; otherwise add a Town.
- **Changes:** base Build
- **Engine:** `GameController::GetIslandPieceTypeToBuildInLand` counts Towns and Cities among `GetPiecesToConsiderForBuild`; if towns > cities it returns City (3), otherwise Town (2); then `AskForIslandPieceTypeToBuildInLand` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3b: "If the land has more Towns than Cities, add a City. (You are not upgrading the Towns, you are adding a new City from the supply.) In all other cases, add a Town."
- **Agreement:** `agree`. Existing Q2 (`invader-lifecycle.md` states the opposite).
- **Affects:** Build forecast; Ravage damage next turn
- **Status:** reviewed by Brett (2026-10-19, Q2)

**R-BU-3 · One piece per Build**

- **Rule:** A Build adds one piece unless a rule says otherwise (Habsburg Livestock L2: 2 Towns instead of an Inland City).
- **Changes:** base Build
- **Engine:** `GetNumberOfPiecesToBuildInLand` defaults to 1, overridable by `AskForNumberOfPiecesToBuild`.
- **Wiki:** Sequence_of_Play@12144 §3b: "they add either one City or one Town".
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-BU-4 · Disease**

- **Rule:** When Invaders would Build in a land with Disease, they remove 1 Disease instead. Exception: Habsburg Mining Expedition L1+ makes Disease affect Ravage in Mining lands (Q26; R-ADV-HME-L1a).
- **Changes:** `build_change` (token)
- **Engine:** token cancellation names as in R-EX-7; Disease amounts not read.
- **Wiki:** Disease@3166: "When Invaders would Build in a land with {Disease}, instead remove one {Disease} from that land." Sequence_of_Play@12144 (BC/JE update): "Build (Including Disease)".
- **Agreement:** `wiki-only`
- **Affects:** Build forecast
- **Status:** sourced (2026-10-19, Q26; JE rulebook "Disease")

**R-BU-5 · A land explored now builds next turn**

- **Rule:** *(inference from R-TS-2)* A land that gets an Explorer from this turn's Explore card is in next turn's Build list and the following turn's Ravage list, unless the Explorer leaves, the Build is prevented, or the card is moved. This is lumia's `ahead` badge.
- **Changes:** card flow
- **Engine:** as R-TS-2.
- **Wiki:** Sequence_of_Play@12144 §4: "… so that next turn they will … Build where they Explored this turn."
- **Agreement:** `agree` (derived)
- **Affects:** land-tempo badges; Slow-Power planning (3)
- **Status:** reviewed by Brett (2026-10-19, Q24; derived)

### 2.3 Ravage

**R-RA-1 · Invader damage**

- **Rule:** In each land of the shown type with Invaders, the Invaders deal 1 per Explorer, 2 per Town, 3 per City, plus modifiers.
- **Changes:** base Ravage
- **Engine:** `IslandPiece::get_DamageToDeal` returns `NormalMaxHealth` (Explorer 1, Town 2, City 3); `GameController::GetDamageForIslandPiece` applies `AskForOverridenDamageForPiece` and `AskForModifiedDamageForPiece`, floor 0; `GetInvaderDamageForLand` sums the participating Invaders, adds `GetRavageDamageModification`, subtracts `DefenseValue` (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3a: "Wherever there are Invaders in the shown lands, Invaders deal Damage: 1 per Explorer, 2 per Town, and 3 per City."
- **Agreement:** `agree`
- **Affects:** Ravage ledger
- **Status:** unreviewed

**R-RA-2 · Defend, then land and Dahan at the same time**

- **Rule:** Reduce the total by Defend, then deal that total to the land and to the Dahan at the same time (the same total to each).
- **Changes:** base Ravage
- **Engine:** `RavageAction` keeps `AmountOfDamageFromInvadersToLand` and `…ToDahan` separately; one Explorer's 1 Damage logged `DamageDistribution=Dahan5 => 1, JungleG7 => 1`. Test 1 Turn 3 (open-questions Q4): "Jungle E3 with Defend 2 … invaders 3 damage → `Dahan8 => 1, JungleE3 => 1`" — Defend reduced the damage to both.
- **Wiki:** Sequence_of_Play@12144 §3a: "Reduce the total Damage by any Defend powers played (page 18), then simultaneously: - Invaders Damage the Land … - Invaders Fight the Dahan". Defend@3397: "Defend reduces Damage done by Invaders to the land and/or Dahan."
- **Agreement:** `agree` (supports the open part of existing Q4)
- **Affects:** Ravage ledger
- **Status:** reviewed by Brett (2026-10-19, Q4)

**R-RA-3 · Land damage adds one Blight**

- **Rule:** If the land takes damage ≥ its Health (2, plus modifiers, minimum 1) in one Ravage, add 1 Blight, once, however much damage. Land damage does not accumulate. Land-Health modifiers add together and the total can't go below 1 (NI p.14; Q18, NQ-I).
- **Changes:** base Ravage
- **Engine:** `DealDamageAction.AfterAction`: for a land target, if `CurrentHealth > 0` is false, `GameController.AddBlight(…)` with a literal 1, set `DidAddBlight`, then `Land.SetDamageTaken(0)`. `Land::get_MaxHealth` = 2; `GetCurrentMaxHealth` = max(1, 2 + Σ modifiers) (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3a: "If 2 or more Damage is dealt, add a Blight to the land. (See sidebar.) This only happens once, no matter how much Damage is dealt. Partial Damage to the land is ignored." Blight@7555: "Blight is added to the land whenever a single instance of damage to land is 2 or more … Land does not accumulate damage (as Dahan and Invaders do)".
- **Agreement:** `agree`
- **Affects:** Ravage ledger (blight); blight runway
- **Status:** reviewed by Brett (2026-10-19, Q4, Q18)

**R-RA-4 · Dahan losses**

- **Rule:** Invader and Event Damage to Dahan: every 2 Damage destroys a Dahan, applied as efficiently as possible (3 Damage against 3 Dahan destroys 1 and leaves 1 Damaged; it cannot be spread 1 per Dahan); a Dahan with 1 Damage survives and heals at end of turn. Excess Damage to a Dahan is lost. Spirit Power Damage to Dahan is allocated by choice (Q4; Branch & Claw rulebook, "Damaging the Dahan").
- **Changes:** base Ravage
- **Engine:** `GameController::DistributeDamage` (flags `damageShouldBeEfficient`, `invertDahanDamagePriority`; priority not decoded). Test 1 (Q4): excess damage to Dahan was not carried over.
- **Wiki:** Sequence_of_Play@12144 §3a: "Every 2 points of Damage destroys one Dahan. You must destroy Dahan as efficiently as possible; you cannot disperse Damage among multiple Dahan to avoid killing them. If a Dahan is dealt 1 Damage, turn it over to show that it is Damaged. Any surviving Damaged Dahan will recover at the end of the turn."
- **Agreement:** `wiki-only` (engine allocation not decoded)
- **Affects:** Ravage ledger (Dahan lost)
- **Status:** reviewed by Brett (2026-10-19, Q4)

**R-RA-5 · Dahan counterattack**

- **Rule:** After the Invaders' damage, each surviving Dahan deals 2 Damage to the Invaders. They counterattack even if Defend cut the Invaders' damage to 0, but not if the Ravage was skipped or prevented. Some effects move the counterattack before or alongside the Invaders (5.1, 5.3, 5.6). Counterattack Damage is assigned by choice (Q4).
- **Changes:** base Ravage; `dahan_change` hooks
- **Engine:** `GetPossibleDahanCounterattackTiming` defaults to `AfterInvaders`; enum `DahanCounterattackTiming` {BeforeInvaders, Simultaneous, AfterInvaders}; counterattack logged as `DamageSources=[Dahan5/Dahan6] … DamageAmount=4` (2 per Dahan) (engine_state_model.md Q3).
- **Wiki:** Sequence_of_Play@12144 §3a: "After Invader Damage has been fully resolved in a land, any surviving Dahan there fight back. Each Dahan deals 2 Damage to the Invaders. … The Dahan will fight back even if the Ravaging Invaders deal no Damage (for example, due to Defend Powers), but not if the Ravage Action has been skipped or stopped from happening altogether." The_Dahan@7991, FAQ: "If a Ravage is prevented (by, e.g., A Year of Perfect Stillness or Choke the Land with Green) do the Dahan still fight back? No."
- **Agreement:** `agree` for timing and 2 per Dahan; `wiki-only` for "no counterattack when skipped"
- **Affects:** Ravage ledger (counterattack); "saves the land" vs "skips the Ravage" in the need finder
- **Status:** reviewed by Brett (2026-10-19, Q4; the "no counterattack when skipped" clause was not asked)

**R-RA-6 · Blight: Presence loss and cascade**

- **Rule:** Each time Blight is added to a land (including by cascade), destroy 1 Presence of each Spirit there, unless that Spirit's own rules say otherwise; if the land already had Blight, add 1 Blight to one adjacent land, which can chain. The Spirit whose Power caused the cascade picks the land; for Events and other non-Power causes the players agree. Oceans take cascade Blight only with Ocean's Hungry Grasp in play and under Scotland's Ocean Blight rules (Q5). Adding several Blight at once destroys and cascades once. Moving Blight does neither. A cascade needs Blight left on the Blight Card.
- **Changes:** base blight
- **Engine:** `AddBlightAction` → `HandleBlightDestroyingPresence`; `WillCascade = ShouldBlightCascade(…)` = `islandLocation.HasBlight` read before adding (any controller answering AskType 78 can veto; `PreventBlightCascadeStatusEffect` exists); `CascadeBlight` takes `GetAdjacentIslandLocations`, requires the card to hold more than 0 Blight, lets the player choose (`SelectLocationActivation: Reason=CascadingBlight`), then `AddBlight(selected, isFromCascade:true)` (engine_state_model.md Q3).
- **Wiki:** Blight@7555: "Destroy Presence: In that land, destroy one Presence from each Spirit with Presence there." "Cascade: If the land already has any Blight, also add one Blight to any one adjacent land. (If that adjacent land also has Blight, it will then cascade again from there, etc.)" "if you add more than 1 Blight to a land at once, it only Destroys 1 Presence from each Spirit, and only cascades (once) if there was already Blight in the land." "Moving {Blight} does NOT Destroy {Presence} or trigger cascades!"
- **Agreement:** `agree` (the "Blight left on the card" condition is `engine-only`). Existing Q5.
- **Affects:** cascade badge; presence at risk; blight runway
- **Status:** reviewed by Brett (2026-10-19, Q5; core rulebook p.15)

**R-RA-7 · Strife**

- **Rule:** An Invader with Strife deals 0 Damage when it would damage the land or Dahan, and loses 1 Strife. Dahan still counterattack. Using the Strife is required, and it applies to any Invader Damage to the land or Dahan, not only Ravage (Q25).
- **Changes:** `ravage_damage_change` (token)
- **Engine:** names `InvadersWithStrifeDealDamageToOtherInvaders`, `EachInvaderTakesDamagePerStrife`, `ChangeRavageParticipation` (engine_state_model.md Q3); amounts not read.
- **Wiki:** Strife@3334: "Whenever Invaders Damage the Dahan and/or the land, each attacking Invader with any number of {Strife} deals exactly 0 Damage and removes one {Strife}." "Even if Damage is reduced to 0 (by Defend and/or Strife), the Dahan still fight back!"
- **Agreement:** `wiki-only`
- **Affects:** Ravage ledger
- **Status:** sourced (2026-10-19, Q25; JE rulebook "Strife")

**R-RA-8 · Badlands**

- **Rule:** Once per Action, damage to Invaders in a land is +1 per Badlands, and damage to Dahan is +1 per Badlands; no bonus if the damage was reduced to 0.
- **Changes:** `ravage_damage_change` (token)
- **Engine:** names `DealDamageAction.UsedInvaderBadlands`, `UsedDahanBadlands`, `"BadlandInvaderDamageKey"`, `"BadlandDahanDamageKey"` (cleared at Time Passes).
- **Wiki:** Badlands@13204: "One time each Action Damages Invaders in a land, increase that Damage by 1 per {Badlands}. One time each Action Damages Dahan in a land, increase that Damage by 1 per {Badlands}." "If Damage is reduced to 0 - e.g., by Defend Powers protecting Dahan - then {Badlands} aren't relevant."
- **Agreement:** `wiki-only`
- **Affects:** Ravage ledger (Dahan lost, counterattack kills)
- **Status:** unreviewed

**R-RA-9 · Ravage order of modifiers**

- **Rule:** (1) Add each Invader's damage; per-Invader modifiers such as Strife apply here. (2) If the Invaders deal 1 or more, add per-land modifiers. (3) Subtract Defend. (4) At the same time, damage the land and the Dahan (+Badlands to Dahan). (5) Surviving Dahan counterattack, with modifiers in the same order.
- **Changes:** base Ravage
- **Engine:** the ravage coroutine calls, in order: counterattack timing; `GetIslandLocationDefenseValue`; `GetRavageTargetsForLand`; Dahan damage distribution; participation and token removal; per-source adjustments; `HandleRavageInvaderStepModifiers`; `DealDamage`; Dahan destroyed / blight flags; journal (engine_state_model.md Q3).
- **Wiki:** Defend@3397, "Ravage Timing" (from JE p.7): "Add together the Damage from each individual Invader. Modifiers to individual Invader Damage – like {Strife} – happen here. … If Invaders are dealing 1 or more Damage, increase it by any per-land modifiers. Lower Damage with Defend. Simultaneously, a. Deal this Damage to the land. b. Deal this Damage to {Dahan}. If 1 or more Damage is done to {Dahan}, they take +1 Damage per {Badlands}. Surviving {Dahan} deal Damage to Invaders."
- **Agreement:** `agree` in structure; the "only if 1 or more Damage" condition on per-land modifiers is `wiki-only` (section 8, NQ-L)
- **Affects:** Ravage ledger
- **Status:** unreviewed

**R-RA-10 · Only lands with Invaders Ravage**

- **Rule:** A matching land without Invaders does not Ravage.
- **Changes:** base Ravage
- **Engine:** test not read. The engine's Habsburg Livestock L6 text says "(This does not cause lands without Invaders to Ravage.)".
- **Wiki:** Sequence_of_Play@12144 §3a: "Wherever there are Invaders in the shown lands, Invaders deal Damage".
- **Agreement:** `wiki-only`
- **Affects:** Ravage forecast
- **Status:** unreviewed

**R-RA-11 · Health of pieces and Fear from destruction**

- **Rule:** Explorer 1, Town 2, City 3, Dahan 2 Health; land 2. Destroying a Town = 1 Fear, a City = 2 Fear, an Explorer = 0, however it is destroyed (including by Dahan counterattack); removing is not destroying (Q6).
- **Changes:** base values
- **Engine:** `PieceFactory::CreateIslandPiece`: Dahan 2, Explorer 1, Town 2 + Σ `TownMaxHealthAdjustment`, City 3 + Σ `CityMaxHealthAdjustment`; `IslandPieceController::get_FearFromDestroy`: Town 1, City 2, Thief 1, others 0 (engine_state_model.md Q3).
- **Wiki:** Invader@9428: Explorer "Health 1 … Destroyed 0 Fear", Town "Health 2 … 1 Fear", City "Health 3 … 2 Fear"; "Whenever a Town is destroyed, it generates 1 Fear. Whenever a City is destroyed, it generates 2 Fear." The_Dahan@7991: "Each Dahan has 2 Health". Town@7993 FAQ: "Always, regardless of how it's destroyed: … from damage done by Dahan counterattacking …".
- **Agreement:** `agree`
- **Affects:** Ravage ledger (counterattack kills, Fear)
- **Status:** reviewed by Brett (2026-10-19, Q6 for the Fear values; Q4 for land and Dahan Health)

## 3. Fast and Slow

**R-FS-1 · When each speed resolves**

- **Rule:** Fast Powers (cards and innates) resolve after the Spirit Phase and before the Invader Phase. Slow Powers resolve after the Invader Phase and before Time Passes.
- **Changes:** timing
- **Engine:** `Constants.StateOrdering`: … SpiritPhase, FastPower, BlightedIsland … AdvanceInvaderCards, SlowPower, TimePasses.
- **Wiki:** Sequence_of_Play@12144: "Fast Power Phase (Cards and Innate)", "Slow Power Phase (Cards and Innate)", with the Invader Phase between them.
- **Agreement:** `agree`
- **Affects:** every "playable now" check in the need finder
- **Status:** unreviewed

**R-FS-2 · Cards are chosen and paid in the Spirit Phase**

- **Rule:** All Power Cards for the turn, Fast and Slow, are chosen and paid for in the Spirit Phase, and their Elements count at once, even for Slow cards. A Presence-track "Any" space grants one Element per turn, chosen at any time and then fixed until next turn; Elements are lost if the granting card is Forgotten from play (Q10).
- **Changes:** timing
- **Engine:** not read (existing Q10).
- **Wiki:** Sequence_of_Play@12144: "You must immediately pay Energy for all Power Cards played, even Slow ones. Likewise, you immediately gain all Elements from played Power Cards, even if the Powers' effects are Slow. Do not resolve the effects of the Power Cards yet."
- **Agreement:** `wiki-only`
- **Affects:** innate threshold checks
- **Status:** reviewed by Brett (2026-10-19, Q10)

**R-FS-3 · Innate Powers have speeds**

- **Rule:** Each innate is Fast or Slow like a card. The 34 engine spirits have 30 Fast and 33 Slow innates; the 24 aspects add 13 innates (8 Fast, 5 Slow).
- **Changes:** timing
- **Engine:** `spirits.json` › `innatePowers[].speed` (`true` = Fast). All 63 base-spirit innate speeds match the Wiki spirit pages (`data/references/wiki/*.json`, fetcher output).
- **Wiki:** Sequence_of_Play@12144: "Fast Power Phase (Cards and Innate)"; spirit pages (`speed` per innate).
- **Agreement:** `agree`
- **Affects:** innate panel; need finder
- **Status:** unreviewed

**R-FS-4 · A Fast Power cannot wait**

- **Rule:** A Fast Power not used in the Fast Phase is lost; it cannot be held for the Slow Phase.
- **Changes:** timing
- **Engine:** not read.
- **Wiki:** Sequence_of_Play@12144: "You can't delay using a Fast Power until the Slow Phase, even if you want to. Use it now or forego it."
- **Agreement:** `wiki-only`
- **Affects:** need finder
- **Status:** unreviewed

**R-FS-5 · The planning consequence**

- **Rule:** *(inference from R-TS-1, R-TS-2, R-FS-1)* Fast Powers act before this turn's Ravage, Build and Explore, so Fast Defend, skips, Isolate, removal and movement change what this turn's three cards do. Slow Powers act after this turn's Explore and Advance, so they cannot change this turn's Invader Actions; they set up next turn. Next turn's Build card is the card that just explored and next turn's Ravage card is the card that just built. Example: a land explored this turn Builds next turn; removing that Explorer with a Slow Power, or adding Disease there, is what stops the Build.
- **Changes:** timing
- **Engine:** `Constants.StateOrdering` (R-TS-1).
- **Wiki:** Sequence_of_Play@12144 §4 (R-TS-2 quote).
- **Agreement:** `agree` (derived)
- **Affects:** the "this turn / next turn" split in every Power suggestion; land-tempo badges
- **Status:** unreviewed

**R-FS-6 · Every Power Card that skips, isolates or defends is Fast**

- **Rule:** In the engine's 321 Power Cards, all 18 that skip or prevent an Invader Action, all 11 with Isolate and all 40 with Defend are Fast; so are all 8 Defend innates. Slow cards reach the pipeline through lasting changes: tokens (Wilds 7 Fast / 13 Slow, Disease 3 / 8, Strife 4 / 12, Badlands 3 / 14) and moved pieces (Push 35 / 45, Gather 22 / 23). Counts are regex matches (5.6).
- **Changes:** observation of the card pool, not a rule
- **Engine:** `power_cards.json` `speed` and text (5.6).
- **Wiki:** not compared.
- **Agreement:** `engine-only`
- **Affects:** need finder ("playable now" for skip/Defend needs is always Fast)
- **Status:** unreviewed

**R-FS-7 · Effects that change a Power's speed**

- **Rule:** Lightning's Swift Strike (Swiftness of Lightning: 1 Slow → Fast per Air); Lightning Wind aspect (Exaltation of the Storm-Wind); Thunderspeaker (Gather the Warriors and Lead the Furious Assault: "may be Fast" at 4 Air); Grinning Trickster (Why Don't You and Them Fight: "may be Slow" at 3 Moon); Fractured Days (Slip the Flow of Time: resolve 1 Slow Power now); scenario Blitz (all Powers Fast); Event Sprawl Contained by the Wilds, choice "Focus Your Will" (next turn all Slow); Event Investigation of Dangers, Dahan "Rouse the Spirits" (use a Slow Power during the Event). Entries: 5.1, 5.2, 5.3.
- **Changes:** timing
- **Engine:** entries cited in 5.1–5.3.
- **Wiki:** entries cited in 5.1–5.3.
- **Agreement:** see each entry
- **Affects:** need finder
- **Status:** unreviewed


## 4. Adversaries

Every adversary in `adversaries.json` (8) at every level 0–6. Level 0 is the adversary's base: its Escalation and its additional loss condition. Levels are cumulative in play: level N uses levels 1..N. Each entry records only that level's own new effect. Engine difficulty and `fearCardModifiers` were compared with the Wiki table for all 48 levels: the fear decks match at every level (the engine string is the per-level deck as offsets from 3, not an increment), and difficulty matches except England level 6 (resolved: 11 per Brett's physical panel, Q16; the engine's 10 is a Steam-vs-box difference). Where adversary rules leave a choice of land or a tie, the players decide together, choosing what is best for them (Q21). Wiki pages are archived copies (web.archive.org raw snapshots) of spiritislandwiki.com; revision id from the page's `wgRevisionId`.


### 4.1 The Kingdom of Brandenburg-Prussia — `TheKingdomOfBrandenburgPrussia` (Base game)

Wiki page `Brandenburg-Prussia` revision 6845 (archive snapshot 20250427153341). Base difficulty: engine `baseDifficulty` = absent, Wiki = 1 (engine field absent; Wiki only).

**R-ADV-BP-L0a · Brandenburg-Prussia Escalation: Land Rush**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): (Before Exploring) On each board with {Town}/{City}, add 1 {Town} to a land without {Town}. Timing: `before_explore`. The players choose the land, best for them (Q21); the Town is an Explore source that turn (FAQ, NQ-G).
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.stageIIEscalationBody`: "(Before Exploring) On each board with {Town}/{City}, add 1 {Town} to a land without {Town}."
- **Wiki:** Brandenburg-Prussia@6845, Escalation: "Land Rush: On each board with {Town}/{City}, add 1 {Town} to a land without {Town}."; ruling: "Brandenburg-Prussia's Escalation effect happens before Exploring."
- **Agreement:** `agree` — Engine text carries '(Before Exploring)'; Wiki text omits it but the Wiki ruling says 'happens before Exploring.'
- **Affects:** invader forecast on Escalation cards
- **Status:** reviewed by Brett (2026-10-19, Q21); sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-ADV-BP-L0b · Brandenburg-Prussia loss condition:** none. Engine has no `additionalLossCondition`; Wiki (Brandenburg-Prussia@6845) lists "None". `agree`. Status: unreviewed.

**R-ADV-BP-L1 · Brandenburg-Prussia L1 Fast Start**

- **Rule:** During Setup, on each board add 1 {Town} to land #3. Parameters: addTownToLandNumberOnEachBoard = 3.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[1]` (difficulty 2, `fearCardModifiers` "0/0/0"; `addTownToLandNumberOnEachBoard: 3`): "During Setup, on each board add 1 {Town} to land #3."
- **Wiki:** Brandenburg-Prussia@6845, Level 1 (difficulty 2, fear 9 (3/3/3)): "Fast Start: During Setup, on each board add 1 {Town} to land #3."
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-BP-L2 · Brandenburg-Prussia L2 Surge of Colonists**

- **Rule:** When making the Invader Deck, put 1 of the Stage III cards between Stage I and Stage II. (New Deck Order: 111-3-2222-3333) Parameters: moveInvaderCard = [3, 1]; shouldMovedInvaderCardOverrideStage = True.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[2]` (difficulty 4, `fearCardModifiers` "0/0/0"; `moveInvaderCard: [3, 1]`, `shouldMovedInvaderCardOverrideStage: true`): "When making the Invader Deck, put 1 of the Stage III cards between Stage I and Stage II. (New Deck Order: 111-3-2222-3333)"
- **Wiki:** Brandenburg-Prussia@6845, Level 2 (difficulty 4, fear 9 (3/3/3)): "Surge of Colonists: When making the Invader Deck, put 1 of the Stage III cards between Stage I and Stage II. (New Deck Order:111-3-2222-3333)"
- **Agreement:** `agree` — Deck 111-3-2222-3333 (13 cards incl. the moved Stage III). shouldMovedInvaderCardOverrideStage=true: the moved card counts as that slot's stage? Meaning of the flag not read in IL.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-BP-L3 · Brandenburg-Prussia L3 Efficient**

- **Rule:** When making the Invader Deck, remove an additional Stage I card. (New Deck Order: 11-3-2222-3333) Parameters: removeInvaderCard = 1.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[3]` (difficulty 6, `fearCardModifiers` "0/1/0"; `removeInvaderCard: 1`): "When making the Invader Deck, remove an additional Stage I card. (New Deck Order: 11-3-2222-3333)"
- **Wiki:** Brandenburg-Prussia@6845, Level 3 (difficulty 6, fear 10 (3/4/3)): "Efficient: When making the Invader Deck, remove an additional Stage I card. (New Deck Order:11-3-2222-3333)"
- **Agreement:** `agree` — removeInvaderCard is the stage number of the card removed (1). Deck 11-3-2222-3333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-BP-L4 · Brandenburg-Prussia L4 Aggressive Timetable**

- **Rule:** When making the Invader Deck, remove an additional Stage II card. (New Deck Order: 11-3-222-3333) Parameters: removeInvaderCard = 2.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[4]` (difficulty 7, `fearCardModifiers` "1/1/0"; `removeInvaderCard: 2`): "When making the Invader Deck, remove an additional Stage II card. (New Deck Order: 11-3-222-3333)"
- **Wiki:** Brandenburg-Prussia@6845, Level 4 (difficulty 7, fear 11 (4/4/3)): "Aggressive Timetable: When making the Invader Deck, remove an additional Stage II card. (New Deck Order:11-3-222-3333)"
- **Agreement:** `agree` — Removes a Stage II card. Deck 11-3-222-3333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-BP-L5 · Brandenburg-Prussia L5 Ruthlessly Efficient**

- **Rule:** When making the Invader Deck, remove an additional Stage I card. (New Deck Order: 1-3-222-3333) Parameters: removeInvaderCard = 1.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[5]` (difficulty 9, `fearCardModifiers` "1/1/0"; `removeInvaderCard: 1`): "When making the Invader Deck, remove an additional Stage I card. (New Deck Order: 1-3-222-3333)"
- **Wiki:** Brandenburg-Prussia@6845, Level 5 (difficulty 9, fear 11 (4/4/3)): "Ruthlessly Efficient: When making the Invader Deck, remove an additional Stage I card. (New Deck Order:1-3-222-3333)"
- **Agreement:** `agree` — Deck 1-3-222-3333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-BP-L6 · Brandenburg-Prussia L6 Terrifyingly Efficient**

- **Rule:** When making the Invader Deck, remove all Stage I cards. (New Deck Order: 3-222-3333) Parameters: removeInvaderCard = 1.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfBrandenburgPrussia.levels[6]` (difficulty 10, `fearCardModifiers` "1/1/1"; `removeInvaderCard: 1`): "When making the Invader Deck, remove all Stage I cards. (New Deck Order: 3-222-3333)"
- **Wiki:** Brandenburg-Prussia@6845, Level 6 (difficulty 10, fear 12 (4/4/4)): "Terrifyingly Efficient: When making the Invader Deck, remove all Stage I cards. (New Deck Order:3-222-3333)"
- **Agreement:** `agree` — Engine field removes one more Stage I card; with L3+L5 already applied that is the last one, matching 'remove all Stage I cards'. Deck 3-222-3333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

### 4.2 The Kingdom of England — `TheKingdomOfEngland` (Base game)

Wiki page `England` revision 6844 (archive snapshot 20250427153352). Base difficulty: engine `baseDifficulty` = absent, Wiki = 1 (engine field absent; Wiki only).

**R-ADV-ENG-L0a · England Escalation: Building Boom**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): On each board with {Town}/{City}, build in the land with the most {Town}/{City}. Timing: `before_explore`.
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.stageIIEscalationBody`: "On each board with {Town}/{City}, build in the land with the most {Town}/{City}."
- **Wiki:** England@6844, Escalation: "Building Boom: On each board with {Town}/{City}, Build in the land with the most {Town}/{City}."; ruling: "England's Escalation effect is an Invader Action, not an Adversary Action.[1]"
- **Agreement:** `agree` — Engine text gives no timing. Wiki rulings: 'England's Escalation effect is an Invader Action, not an Adversary Action.' and 'happens before Exploring.'
- **Affects:** invader forecast on Escalation cards
- **Status:** sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-ADV-ENG-L0b · England loss condition: Proud & Mighty Capital**

- **Rule:** If 7 or more {Town}/{City} are ever in a single land, the Invaders win.
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.additionalLossCondition`: "If 7 or more {Town}/{City} are ever in a single land, the Invaders win."
- **Wiki:** England@6844, Additional Loss Condition: "Proud & Mighty Capital: If 7 or more {Town}/{City} are ever in a single land, the Invaders win."
- **Agreement:** `agree`
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**R-ADV-ENG-L1 · England L1 Indentured Servants Earn Land**

- **Rule:** Invader Build actions affect lands without Invaders, if they are adjacent to at least 2 {Town}/{City} before the Build Action. Parameters: builds_in_lands_without_invaders = True; condition = adjacent to >= 2 Town/City before the Build Action; applies_to = lands matching the Build card.
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[1]` (difficulty 3, `fearCardModifiers` "0/1/0"; no machine fields): "Invader Build actions affect lands without Invaders, if they are adjacent to at least 2 {Town}/{City} before the Build Action."
- **Wiki:** England@6844, Level 1 (difficulty 3, fear 10 (3/4/3)): "Indentured Servants Earn Land: Invader Build Cards affect matching lands without Invaders if they are adjacent to at least 2 {Town}/{City}."
- **Agreement:** `agree` — Engine adds 'before the Build Action'; Wiki says 'Build Cards affect matching lands'. Same rule.
- **Affects:** Build forecast
- **Status:** reviewed by Brett (2026-10-19, Q2)

**R-ADV-ENG-L2 · England L2 Criminals and Malcontents**

- **Rule:** During Setup, on each board add 1 {City} to land #1, and 1 {Town} to land #2. Parameters: addCityToLandNumberOnEachBoard = 1; addTownToLandNumberOnEachBoard = 2.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[2]` (difficulty 4, `fearCardModifiers` "1/1/0"; `addCityToLandNumberOnEachBoard: 1`, `addTownToLandNumberOnEachBoard: 2`): "During Setup, on each board add 1 {City} to land #1, and 1 {Town} to land #2."
- **Wiki:** England@6844, Level 2 (difficulty 4, fear 11 (4/4/3)): "Criminals and Malcontents: During Setup, on each board add 1 {City} to land #1 and 1 {Town} to land #2."
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-ENG-L3 · England L3 High Immigration (I)**

- **Rule:** Put the “High Immigration” tile on the Invader board, to the left of “Ravage”. The Invaders take this Build action each Invader phase before Ravaging. Cards slide left from Ravage to it, and from it to the discard pile. Remove the tile when a Stage II card slides onto it, putting that card in the discard. Parameters: action = build; slot = HighImmigration; when = each Invader Phase, before Ravage (StateOrdering: HighImmigration precedes Ravage); card_flow = Ravage -> HighImmigration -> discard; tile_removed = when a Stage II card slides onto it; that card is discarded; addInvaderSlotIdentifier = HighImmigration; addInvaderSlotLeftOfPosition = Ravage; addInvaderSlotTitle = High Immigration.
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[3]` (difficulty 6, `fearCardModifiers` "1/2/1"; `addInvaderSlotIdentifier: "HighImmigration"`, `addInvaderSlotLeftOfPosition: "Ravage"`, `addInvaderSlotTitle: "High Immigration"`): "Put the “High Immigration” tile on the Invader board, to the left of “Ravage”. The Invaders take this Build action each Invader phase before Ravaging. Cards slide left from Ravage to it, and from it to the discard pile. Remove the tile when a Stage II card slides onto it, putting that card in the discard."
- **Wiki:** England@6844, Level 3 (difficulty 6, fear 13 (4/5/4)): "High Immigration (I): Put the "High Immigration" tile on the Invader board, to the left of "Ravage". The Invaders take this Build action each Invader phase before Ravaging. Cards slide left from Ravage to it, and from it to the discard pile. Remove the tile when a Stage II card slides onto it, putting that card in the discard."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-ADV-ENG-L4 · England L4 High Immigration (full)**

- **Rule:** The extra Build tile remains out the entire game. Parameters: action = build; slot = HighImmigration; when = tile stays out the entire game (overrides L3 removal).
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[4]` (difficulty 7, `fearCardModifiers` "1/2/2"; no machine fields): "The extra Build tile remains out the entire game."
- **Wiki:** England@6844, Level 4 (difficulty 7, fear 14 (4/5/5)): "High Immigration (full): The extra Build tile remains out the entire game."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-ADV-ENG-L5 · England L5 Local Autonomy**

- **Rule:** {Town}/{City} have +1 Health. Parameters: cityMaxHealthAdjustment = 1; townMaxHealthAdjustment = 1.
- **Changes:** `piece_health`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[5]` (difficulty 9, `fearCardModifiers` "1/2/2"; `cityMaxHealthAdjustment: 1`, `townMaxHealthAdjustment: 1`): "{Town}/{City} have +1 Health."
- **Wiki:** England@6844, Level 5 (difficulty 9, fear 14 (4/5/5)): "Local Autonomy: {Town}/{City} have +1 Health."
- **Agreement:** `agree` — Town 3, City 4 Health.
- **Affects:** Ravage ledger (counterattack kills); Build/victory blockers
- **Status:** unreviewed

**R-ADV-ENG-L6a · England L6 Independent Resolve**

- **Rule:** During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot). Parameters: extraFearPerPlayer = 1; effect = fear pool +1 per player (fear per card).
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[6]` (difficulty 10, `fearCardModifiers` "1/2/1"; `extraFearPerPlayer: 1`): "During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot)."
- **Wiki:** England@6844, Level 6 (difficulty 11, fear 13 (4/5/4)): "Independent Resolve: During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot.)"
- **Agreement:** `agree` — Level difficulty conflict: engine 10, Wiki 11 (existing Q16); the effect text agrees.
- **Resolution:** Difficulty 11, per Brett's physical panel (Q16). The engine's 10 is a Steam-vs-box difference; `data/adversaries.json` needs the 11. The +1 Fear per player is confirmed (Q6).
- **Affects:** see note
- **Status:** reviewed by Brett (2026-10-19, Q6, Q16; physical panel)

**R-ADV-ENG-L6b · England L6 Independent Resolve**

- **Rule:** During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot). Parameters: action = build; slot = HighImmigration; when = twice in any Invader Phase where no Fear Cards resolve; no effect if the slot is empty.
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `TheKingdomOfEngland.levels[6]` (difficulty 10, `fearCardModifiers` "1/2/1"; `extraFearPerPlayer: 1`): "During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot)."
- **Wiki:** England@6844, Level 6 (difficulty 11, fear 13 (4/5/4)): "Independent Resolve: During Setup, add an additional {Fear} to the Fear Pool per player in the game. During any Invader Phase where you resolve no Fear Cards, perform the Build from High Immigration twice. (This has no effect if no card is on the extra Build slot.)"
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

### 4.3 The Kingdom of Sweden — `TheKingdomOfSweden` (Base game)

Wiki page `Sweden` revision 12523 (archive snapshot 20250427153340). Base difficulty: engine `baseDifficulty` = absent, Wiki = 1 (engine field absent; Wiki only).

**R-ADV-SWE-L0a · Sweden Escalation: Swayed by the Invaders**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): After Invaders Explore into each land, if that land has at least as many Invaders as {Dahan}, replace 1 {Dahan} with 1 {Town}. Timing: `after_explore`.
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.stageIIEscalationBody`: "After Invaders Explore into each land, if that land has at least as many Invaders as {Dahan}, replace 1 {Dahan} with 1 {Town}."
- **Wiki:** Sweden@12523, Escalation: "Swayed by the Invaders: After Invaders Explore into each land this Phase, if that land has at least as many Invaders as {Dahan}, replace 1 {Dahan} with 1 {Town}."; ruling: "Sweden's Escalation effect happens after Exploring."
- **Agreement:** `agree` — Wiki adds 'this Phase'; Wiki ruling: 'happens after Exploring' and counts pieces, not Health.
- **Affects:** invader forecast on Escalation cards
- **Status:** unreviewed

**R-ADV-SWE-L0b · Sweden loss condition:** none. Engine has no `additionalLossCondition`; Wiki (Sweden@12523) lists "None". `agree`. Status: unreviewed.

**R-ADV-SWE-L1 · Sweden L1 Heavy Mining**

- **Rule:** When a Ravage does 4+ overkill to the land, add an extra {Blight}. (This does not cause extra cascades / {Presence} destruction.) Parameters: extra_blight = 1; trigger_engine = 4+ overkill to the land; trigger_wiki = at least 6 Damage to the land; extra_cascade = False; extra_presence_destroyed = False.
- **Changes:** `ravage_blight_change`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[1]` (difficulty 2, `fearCardModifiers` "0/0/0"; no machine fields): "When a Ravage does 4+ overkill to the land, add an extra {Blight}. (This does not cause extra cascades / {Presence} destruction.)"
- **Wiki:** Sweden@12523, Level 1 (difficulty 2, fear 9 (3/3/3)): "Heavy Mining: If the Invaders do at least 6 Damage to the land during Ravage, add an extra {Blight}. The additional {Blight} does not destroy Presence or cause cascades."
- **Agreement:** `conflict` — Engine '4+ overkill'; Wiki 'at least 6 Damage'. Same at land Health 2 (2 + 4 = 6); differs under land-Health modifiers. Existing Q18.
- **Resolution:** Use the Nature Incarnate wording (NI p.14), as the engine does (Q18). Overkill = Damage beyond the land's Health, so a Ravage with 4+ overkill adds 2 Blight instead of 1. The Wiki's "at least 6 Damage" is the same only at land Health 2.
- **Affects:** Ravage ledger (blight added); blight runway
- **Status:** reviewed by Brett (2026-10-19, Q18; NI p.14)

**R-ADV-SWE-L2 · Sweden L2 Population Pressure at Home**

- **Rule:** During Setup, on each board add 1 {City} to land #4. On boards where land #4 starts with {Blight}, put that {Blight} in land #5 instead. Parameters: addCityToLandNumberOnEachBoard = 4; moveBlightFromLandNumberToLandNumber = [4, 5].
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[2]` (difficulty 3, `fearCardModifiers` "0/1/0"; `addCityToLandNumberOnEachBoard: 4`, `moveBlightFromLandNumberToLandNumber: [4, 5]`): "During Setup, on each board add 1 {City} to land #4. On boards where land #4 starts with {Blight}, put that {Blight} in land #5 instead."
- **Wiki:** Sweden@12523, Level 2 (difficulty 3, fear 10 (3/4/3)): "Population Pressure at Home: During Setup, on each board add 1 {City} to land #4. On boards where land #4 starts with {Blight}, put that {Blight} in land #5 instead."
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-SWE-L3 · Sweden L3 Fine Steel for Tools and Guns**

- **Rule:** {Town} deal 3 Damage. {City} deal 5 Damage. Parameters: cityDamageAdjustment = 2; townDamageAdjustment = 1.
- **Changes:** `ravage_damage_change`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[3]` (difficulty 5, `fearCardModifiers` "0/1/0"; `cityDamageAdjustment: 2`, `townDamageAdjustment: 1`): "{Town} deal 3 Damage. {City} deal 5 Damage."
- **Wiki:** Sweden@12523, Level 3 (difficulty 5, fear 10 (3/4/3)): "Fine Steel for Tools and Guns: {Town} deal 3 Damage. {City} deal 5 Damage."
- **Agreement:** `agree` — Town 3, City 5 Damage.
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-ADV-SWE-L4a · Sweden L4 Royal Backing**

- **Rule:** At the end of Setup (after resolving the initial Explore Card), Accelerate the Invader Deck. (Discard the topmost card of the lowest Invader Stage remaining in the deck.) On each board, add 1 {Town} to the land with fewest Invaders that matches the discarded card. (FAQ "Sweden" wording, NQ-A; the discarded card is seen, NQ-E.) Parameters: accelerate = 1; engine_timing = after the initial Explore; wiki_timing = during Setup, after adding all other Invaders; machine_fields = none in engine defs.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[4]` (difficulty 6, `fearCardModifiers` "0/1/1"; no machine fields): "At the end of Setup, Accelerate the Invader Deck. (After the initial Explore, discard the topmost card of the lowest Invader Stage in the deck.) On each board, add 1 {Town} to the land matching the discarded card with the fewest Invaders."
- **Wiki:** Sweden@12523, Level 4 (difficulty 6, fear 11 (3/4/4)): "Royal Backing: During Setup, after adding all other Invaders, Accelerate the Invader Deck. On each board, add 1 {Town} to the land of that terrain with the fewest Invaders."
- **Agreement:** `conflict` — Engine: Accelerate 'at the end of Setup ... (After the initial Explore ...)'. Wiki: 'During Setup, after adding all other Invaders'. Whether the initial-Explore card can be the discarded one depends on this.
- **Resolution:** The engine's timing is right. FAQ (Sweden): "At the end of Setup (after resolving the initial Explore Card), Accelerate the Invader Deck." So the initial-Explore card is never the discarded one; the discarded card goes face-up to the Invader discard (NQ-E).
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** sourced (2026-10-19, NQ-A; FAQ "Sweden")

**R-ADV-SWE-L4b · Sweden L4 Royal Backing**

- **Rule:** At the end of Setup (after resolving the initial Explore Card), Accelerate the Invader Deck. (Discard the topmost card of the lowest Invader Stage remaining in the deck.) On each board, add 1 {Town} to the land with fewest Invaders that matches the discarded card. (FAQ "Sweden" wording, NQ-A; the discarded card is seen, NQ-E.) Parameters: adds = 1 Town per board, in the land matching the discarded card with the fewest Invaders.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[4]` (difficulty 6, `fearCardModifiers` "0/1/1"; no machine fields): "At the end of Setup, Accelerate the Invader Deck. (After the initial Explore, discard the topmost card of the lowest Invader Stage in the deck.) On each board, add 1 {Town} to the land matching the discarded card with the fewest Invaders."
- **Wiki:** Sweden@12523, Level 4 (difficulty 6, fear 11 (3/4/4)): "Royal Backing: During Setup, after adding all other Invaders, Accelerate the Invader Deck. On each board, add 1 {Town} to the land of that terrain with the fewest Invaders."
- **Agreement:** `agree` — Engine 'land matching the discarded card'; Wiki 'land of that terrain'.
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** sourced (2026-10-19, NQ-A; FAQ "Sweden")

**R-ADV-SWE-L5 · Sweden L5 Mining Rush**

- **Rule:** When Ravaging adds at least 1 {Blight} to a land, also add 1 {Town} to an adjacent land without {Town}/{City}. Cascading {Blight} does not cause this effect. The players choose the land, best for them (Q21). Parameters: trigger = a Ravage adds >= 1 Blight to a land (not cascades); adds = 1 Town to an adjacent land without Town/City.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[5]` (difficulty 7, `fearCardModifiers` "1/1/1"; no machine fields): "When Ravaging adds at least 1 {Blight} to a land, also add 1 {Town} to an adjacent land without {Town}/{City}. Cascading {Blight} does not cause this effect."
- **Wiki:** Sweden@12523, Level 5 (difficulty 7, fear 12 (4/4/4)): "Mining Rush: When Ravaging adds at least 1 {Blight} to a land, also add 1 {Town} to an adjacent land without {Town}/{City}. Cascading {Blight} does not cause this effect."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** reviewed by Brett (2026-10-19, Q21)

**R-ADV-SWE-L6 · Sweden L6 Prospecting Outpost**

- **Rule:** During Setup, on each board add 1 {Town} and 1 {Blight} to land #8. The {Blight} comes from the box, not the Blight Card. Parameters: addBlightToLandNumberOnEachBoard = 8; addTownToLandNumberOnEachBoard = 8.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfSweden.levels[6]` (difficulty 8, `fearCardModifiers` "1/1/2"; `addBlightToLandNumberOnEachBoard: 8`, `addTownToLandNumberOnEachBoard: 8`): "During Setup, on each board add 1 {Town} and 1 {Blight} to land #8. The {Blight} comes from the box, not the Blight Card."
- **Wiki:** Sweden@12523, Level 6 (difficulty 8, fear 13 (4/4/5)): "Prospecting Outpost: During setup, on each board add 1 {Town} and 1 {Blight} to land #8. The {Blight} comes from the box, not the Blight Card."
- **Agreement:** `agree` — Blight from the box, not the Blight Card.
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

### 4.4 The Kingdom of France (Plantation Colony) — `TheKingdomOfFrance` (Branch & Claw)

Wiki page `France_(Plantation_Colony)` revision 7940 (archive snapshot 20250427160406). Base difficulty: engine `baseDifficulty` = 2, Wiki = 2. Difficulties 2; 3, 5, 7, 8, 9, 10 confirmed against the archived Wiki (2024; open-questions checklist, 2026-10-19).
 Engine `requiresTokens`: ['Strife'].

**R-ADV-FRA-L0a · France Escalation: Demand for New Cash Crops**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): After Exploring, on each board, pick a land of the shown terrain. If it has {Town}/{City}, add 1 {Blight}. Otherwise, add 1 {Town}. Timing: `after_explore`.
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.stageIIEscalationBody`: "After Exploring, on each board, pick a land of the shown terrain. If it has {Town}/{City}, add 1 {Blight}. Otherwise, add 1 {Town}."
- **Wiki:** France_(Plantation_Colony)@7940, Escalation: "Demand for New Cash Crops: After Exploring, on each board, pick a land of the shown terrain. If it has {Town}/{City}, add 1 {Blight}. Otherwise, add 1 {Town}."; ruling: "France's Escalation effect happens after Exploring."
- **Agreement:** `agree`
- **Affects:** invader forecast on Escalation cards
- **Status:** unreviewed

**R-ADV-FRA-L0b · France loss condition: Sprawling Plantations**

- **Rule:** Before Setup, return all but 7 {Town} per player to the box. Invaders win if you ever cannot place a {Town}. (Brett's physical panel, Q17.) With an extra board the limit stays 7 per player. Towns that permanently leave the island or are replaced return to France's pool; Towns in The Endless Dark still count (FAQ, France).
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.additionalLossCondition`: "There is a limit of 7 {Town} per starting board. Invaders win if you ever cannot place a {Town}."
- **Wiki:** France_(Plantation_Colony)@7940, Additional Loss Condition: "Sprawling Plantations: Before Setup, return all but 7 {Town} per player to the box. Invaders win if you ever cannot place a {Town}."
- **Agreement:** `conflict` — Engine: 'limit of 7 Town per starting board'. Wiki: 'return all but 7 Town per player to the box'. Same at 1 board per player; differs with an extra board. Existing Q17.
- **Resolution:** Per player, per Brett's physical panel (Q17). Steam's "7 per starting board" differs from the box only with an extra board ("each player" effects happen once per player, JE rulebook).
- **Affects:** loss clocks (race chart)
- **Status:** reviewed by Brett (2026-10-19, Q17; physical panel)

**R-ADV-FRA-L1 · France L1 Frontier Explorers**

- **Rule:** Except during Setup: After Invaders successfully Explore into a land which had no {Town}/{City}, add 1 {Explorer} there. Parameters: after = successful Explore into a land that had no Town/City; adds_explorer = 1; except = Setup.
- **Changes:** `explore_change`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/0"; no machine fields): "Except during Setup: After Invaders successfully Explore into a land which had no {Town}/{City}, add 1 {Explorer} there."
- **Wiki:** France_(Plantation_Colony)@7940, Level 1 (difficulty 3, fear 9 (3/3/3)): "Frontier Explorers: Except during Setup: After Invaders successfully Explore into a land which had no {Town}/{City}, add 1 {Explorer} there."
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-ADV-FRA-L2a · France L2 Slave Labor**

- **Rule:** During Setup, put the “Slave Rebellion” Event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}. Parameters: event_deck = Slave Rebellion placed under the top 3 Event cards at Setup.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[2]` (difficulty 5, `fearCardModifiers` "0/1/0"; no machine fields): "During Setup, put the “Slave Rebellion” Event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}."
- **Wiki:** France_(Plantation_Colony)@7940, Level 2 (difficulty 5, fear 10 (3/4/3)): "Slave Labor: During Setup, put the "Slave Rebellion" event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-FRA-L2b · France L2 Slave Labor**

- **Rule:** During Setup, put the “Slave Rebellion” Event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}. Parameters: after = Build in a land with >= 2 Explorers; replace = all but 1 Explorer with Towns.
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[2]` (difficulty 5, `fearCardModifiers` "0/1/0"; no machine fields): "During Setup, put the “Slave Rebellion” Event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}."
- **Wiki:** France_(Plantation_Colony)@7940, Level 2 (difficulty 5, fear 10 (3/4/3)): "Slave Labor: During Setup, put the "Slave Rebellion" event under the top 3 cards of the Event Deck. After Invaders Build in a land with 2 {Explorer} or more, replace all but 1 {Explorer} there with an equal number of {Town}."
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-ADV-FRA-L3 · France L3 Early Plantations**

- **Rule:** During Setup, on each board add 1 {Town} to the highest-numbered land without {Town}. Add 1 {Town} to land #1. Parameters: addTownToHighestNumberLandWithoutTownOnEachBoard = True; addTownToLandNumberOnEachBoard = 1.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[3]` (difficulty 7, `fearCardModifiers` "1/1/0"; `addTownToHighestNumberLandWithoutTownOnEachBoard: true`, `addTownToLandNumberOnEachBoard: 1`): "During Setup, on each board add 1 {Town} to the highest-numbered land without {Town}. Add 1 {Town} to land #1."
- **Wiki:** France_(Plantation_Colony)@7940, Level 3 (difficulty 7, fear 11 (4/4/3)): "Early Plantation: During Setup, on each board add 1 {Town} to the highest-numbered land without {Town}. Add 1 {Town} to land #1."
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-FRA-L4 · France L4 Triangle Trade**

- **Rule:** Whenever Invaders Build a Coastal {City}, add 1 {Town} to the adjacent land with the fewest {Town}. Parameters: trigger = Invaders Build a Coastal City; adds = 1 Town to the adjacent land with fewest Town.
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[4]` (difficulty 8, `fearCardModifiers` "1/1/1"; no machine fields): "Whenever Invaders Build a Coastal {City}, add 1 {Town} to the adjacent land with the fewest {Town}."
- **Wiki:** France_(Plantation_Colony)@7940, Level 4 (difficulty 8, fear 12 (4/4/4)): "Triangle Trade: Whenever Invaders Build a Coastal {City}, add 1 {Town} to the adjacent land with the fewest {Town}."
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-ADV-FRA-L5 · France L5 Slow-Healing Ecosystem**

- **Rule:** When you remove {Blight} from the board, put it here instead of onto the Blight Card. As soon as you have 3 {Blight} per player here, move it all back to the Blight Card. Parameters: blight_removed = held on the adversary panel until 3 per player, then all returns to the Blight Card.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[5]` (difficulty 9, `fearCardModifiers` "1/2/1"; no machine fields): "When you remove {Blight} from the board, put it here instead of onto the Blight Card. As soon as you have 3 {Blight} per player here, move it all back to the Blight Card."
- **Wiki:** France_(Plantation_Colony)@7940, Level 5 (difficulty 9, fear 13 (4/5/4)): "Slow-Healing Ecosystem: When you remove {Blight} from the board, put it here instead of onto the Blight Card. As soon as you have 3 {Blight} per player here, move it all back to the Blight Card."
- **Agreement:** `agree` — Changes the blight-card count used by the blight-loss forecast.
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-FRA-L6a · France L6 Persistent Explorers**

- **Rule:** After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}. Parameters: after = resolving an Explore Card; adds = 1 Explorer per board to a land without Explorers.
- **Changes:** `explore_change`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[6]` (difficulty 10, `fearCardModifiers` "1/2/2"; no machine fields): "After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}."
- **Wiki:** France_(Plantation_Colony)@7940, Level 6 (difficulty 10, fear 14 (4/5/5)): "Persistent Explorers: After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}."
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-ADV-FRA-L6b · France L6 Persistent Explorers**

- **Rule:** After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}. Parameters: fear_cards_never_remove_explorers = True; instead = may Push.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfFrance.levels[6]` (difficulty 10, `fearCardModifiers` "1/2/2"; no machine fields): "After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}."
- **Wiki:** France_(Plantation_Colony)@7940, Level 6 (difficulty 10, fear 14 (4/5/5)): "Persistent Explorers: After resolving an Explore Card, on each board add 1 {Explorer} to a land without any. Fear Card effects never remove {Explorer}. If one would, you may instead Push that {Explorer}."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

### 4.5 The Habsburg Monarchy (Livestock Colony) — `TheHabsburgMonarchy` (Jagged Earth)

Wiki page `Habsburg_Monarchy_(Livestock_Colony)` revision 6850 (archive snapshot 20250427153353). Base difficulty: engine `baseDifficulty` = 2, Wiki = 2.

**R-ADV-HLC-L0a · Habsburg (Livestock) Escalation: Seek Prime Territory**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): After Exploring: On each board with 4 or fewer {Blight}, add 1 {Town} to a land without {Town}/{Blight}. On each board with 2 or fewer {Blight}, do so again. Timing: `after_explore`.
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.stageIIEscalationBody`: "After Exploring: On each board with 4 or fewer {Blight}, add 1 {Town} to a land without {Town}/{Blight}. On each board with 2 or fewer {Blight}, do so again."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Escalation: "Seek Prime Territory: After Exploring: On each board with 4 or fewer {Blight}, add 1 {Town} to a land without {Town}/{Blight}. On each board with 2 or fewer {Blight}, do so again."; ruling: "Habsburg's Escalation effect happens after Exploring."
- **Agreement:** `agree`
- **Affects:** invader forecast on Escalation cards
- **Status:** unreviewed

**R-ADV-HLC-L0b · Habsburg (Livestock) loss condition: Irreparable Damage**

- **Rule:** Track how many {Blight} come off the Blight Card during Ravages that do 6+ overkill to the land. If that number ever exceeds players, the Invaders win.
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.additionalLossCondition`: "Track how many {Blight} come off the Blight Card during Ravages that do 6+ overkill to the land. If that number ever exceeds players, the Invaders win."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Additional Loss Condition: "Irreparable Damage: Track how many {Blight} come off the Blight Card during Ravages that do 8+ Damage to the land. If that number ever exceeds players, the Invaders win."
- **Agreement:** `conflict` — Engine: Ravages that do '6+ overkill to the land'. Wiki: Ravages that do '8+ Damage to the land'. Same at land Health 2; differs when land Health is modified. Existing Q18.
- **Resolution:** Use the Nature Incarnate wording (NI p.14), as the engine does (Q18): count Blight taken off the card during Ravages with 6+ overkill (Damage beyond the land's Health); more than the number of players loses. The Wiki's "8+ Damage" is the same only at land Health 2.
- **Affects:** loss clocks (race chart)
- **Status:** reviewed by Brett (2026-10-19, Q18; NI p.14)

**R-ADV-HLC-L1 · Habsburg (Livestock) L1 Migratory Herders**

- **Rule:** After the normal Build Step: In each land matching a Build Card, Gather 1 {Town} from a land not matching a Build Card. (In board/land order.) Parameters: after = the normal Build Step; gather = 1 Town into each land matching a Build card, from a non-matching land (board/land order).
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[1]` (difficulty 3, `fearCardModifiers` "0/1/0"; no machine fields): "After the normal Build Step: In each land matching a Build Card, Gather 1 {Town} from a land not matching a Build Card. (In board/land order.)"
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 1 (difficulty 3, fear 10 (3/4/3)): "Migratory Herders: After the normal Build Step: In each land matching a Build Card, Gather 1 {Town} from a land not matching a Build Card. (In board/land order.)"
- **Agreement:** `agree` — Wiki Blight page: still activates when an Event turns the Ravage Step into a Build Step.
- **Affects:** Build forecast
- **Status:** unreviewed

**R-ADV-HLC-L2a · Habsburg (Livestock) L2 More Rural Than Urban**

- **Rule:** During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}. Parameters: addTownToLandNumberOnEachBoard = 2.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[2]` (difficulty 5, `fearCardModifiers` "1/2/-1"; `addTownToLandNumberOnEachBoard: 2`): "During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 2 (difficulty 5, fear 11 (4/5/2)): "More Rural Than Urban: During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}."
- **Agreement:** `agree` — Text also adds 1 Town to the highest-numbered land without Setup symbols; no engine machine field for that part.
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-HLC-L2b · Habsburg (Livestock) L2 More Rural Than Urban**

- **Rule:** During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}. Parameters: inland_city_build = becomes 2 Towns.
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[2]` (difficulty 5, `fearCardModifiers` "1/2/-1"; `addTownToLandNumberOnEachBoard: 2`): "During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 2 (difficulty 5, fear 11 (4/5/2)): "More Rural Than Urban: During Setup, on each board, add 1 {Town} to land #2 and 1 {Town} to the highest-numbered land without Setup symbols. During Play, when Invaders would Build 1 {City} in an Inland land, they instead Build 2 {Town}."
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-ADV-HLC-L3 · Habsburg (Livestock) L3 Fast Spread**

- **Rule:** When making the Invader Deck, Remove 1 additional Stage I Card. (New deck order: 11-2222-33333) Parameters: removeInvaderCard = 1.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[3]` (difficulty 6, `fearCardModifiers` "1/2/0"; `removeInvaderCard: 1`): "When making the Invader Deck, Remove 1 additional Stage I Card. (New deck order: 11-2222-33333)"
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 3 (difficulty 6, fear 12 (4/5/3)): "Fast Spread: When making the Invader Deck, Remove 1 additional Stage I Card. (New deck order: 11-2222-33333)"
- **Agreement:** `agree` — Deck 11-2222-33333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-HLC-L4 · Habsburg (Livestock) L4 Herds Thrive in Verdant Lands**

- **Rule:** {Town} in lands without {Blight} are Durable: they have +2 Health, and “Destroy {Town}” effects instead deal 2 Damage (to those {Town} only) per {Town} they could Destroy. (”Destroy all {Town}“ works normally.) Parameters: town_health_plus = 2; where = lands without Blight; destroy_town = instead 2 Damage per Town.
- **Changes:** `piece_health`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[4]` (difficulty 8, `fearCardModifiers` "1/2/0"; no machine fields): "{Town} in lands without {Blight} are Durable: they have +2 Health, and “Destroy {Town}” effects instead deal 2 Damage (to those {Town} only) per {Town} they could Destroy. (”Destroy all {Town}“ works normally.)"
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 4 (difficulty 8, fear 12 (4/5/3)): "Herds Thrive in Verdant Lands: {Town} in lands without {Blight} are Durable: they have +2 Health, and "Destroy {Town}" effects instead deal 2 Damage (to those {Town} only) per {Town} they could Destroy. ("Destroy all {Town}" works normally.)"
- **Agreement:** `agree` — Durable Towns.
- **Affects:** Ravage ledger (counterattack kills); Build/victory blockers
- **Status:** unreviewed

**R-ADV-HLC-L5a · Habsburg (Livestock) L5 Wave of Immigration**

- **Rule:** Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}. Parameters: addReminderCard = Habsburg; addReminderCardUnderInvaderCards = 5.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[5]` (difficulty 9, `fearCardModifiers` "1/3/0"; `addReminderCard: "Habsburg"`, `addReminderCardUnderInvaderCards: 5`): "Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 5 (difficulty 9, fear 13 (4/6/3)): "Wave of Immigration: Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When Revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}."
- **Agreement:** `agree` — Reminder card under the top 5 Invader cards, before the initial Explore. Wiki ruling: resolve it after the Explore that turn.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-HLC-L5b · Habsburg (Livestock) L5 Wave of Immigration**

- **Rule:** Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}. Parameters: on_reveal = per board: 1 City to a Coastal land without City, 1 Town to the 3 Inland lands with fewest Blight.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[5]` (difficulty 9, `fearCardModifiers` "1/3/0"; `addReminderCard: "Habsburg"`, `addReminderCardUnderInvaderCards: 5`): "Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}."
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 5 (difficulty 9, fear 13 (4/6/3)): "Wave of Immigration: Before the initial Explore, put the Habsburg Reminder Card under the top 5 Invader Cards. When Revealed, on each board, add 1 {City} to a Coastal land without {City} and 1 {Town} to the 3 Inland lands with the fewest {Blight}."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-HLC-L6 · Habsburg (Livestock) L6 Far-Flung Herds**

- **Rule:** Ravages do +2 Damage (total) if any adjacent lands have {Town}. (This does not cause lands without Invaders to Ravage.) Parameters: total_plus = 2; condition = any adjacent land has Town; does_not_make_lands_without_invaders_ravage = True.
- **Changes:** `ravage_damage_change`
- **Engine:** `adversaries.json` › `TheHabsburgMonarchy.levels[6]` (difficulty 10, `fearCardModifiers` "2/3/0"; no machine fields): "Ravages do +2 Damage (total) if any adjacent lands have {Town}. (This does not cause lands without Invaders to Ravage.)"
- **Wiki:** Habsburg_Monarchy_(Livestock_Colony)@6850, Level 6 (difficulty 10, fear 14 (5/6/3)): "Far-Flung Herds: Ravages do +2 Damage (total) if any adjacent lands have {Town}. (This does not cause lands without Invaders to Ravage.)"
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

### 4.6 The Tsardom of Russia — `TheTsardomOfRussia` (Jagged Earth)

Wiki page `Russia` revision 6851 (archive snapshot 20250427193320). Base difficulty: engine `baseDifficulty` = 1, Wiki = 1.
 Engine `requiresTokens`: ['Beast'].

**R-ADV-RUS-L0a · Russia Escalation: Stalk the Predators**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): On each board: Add 2 {Explorer} (total) among lands with {Beast}. If you can’t, instead add 2 {Explorer} among lands with {Beast} on a different board. Timing: `before_explore`. It adds Explorers, which are never Explore sources (NQ-G).
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.stageIIEscalationBody`: "On each board: Add 2 {Explorer} (total) among lands with {Beast}. If you can’t, instead add 2 {Explorer} among lands with {Beast} on a different board."
- **Wiki:** Russia@6851, Escalation: "Stalk the Predators: On each board: Add 2 {Explorer} (total) among lands with {Beasts}. If you can't, instead add 2 {Explorer} among lands with {Beasts} on a different board."; ruling: "Russia's Escalation effect happens before Exploring."
- **Agreement:** `agree` — Engine text gives no timing; Wiki ruling: 'happens before Exploring.'
- **Affects:** invader forecast on Escalation cards
- **Status:** sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-ADV-RUS-L0b · Russia loss condition: Hunters Swarm the Island**

- **Rule:** Track {Beast} Destroyed by Adversary rules. If that number ever exceeds the number of {Beast} on the island, the Invaders win.
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.additionalLossCondition`: "Track {Beast} Destroyed by Adversary rules. If that number ever exceeds the number of {Beast} on the island, the Invaders win."
- **Wiki:** Russia@6851, Additional Loss Condition: "Hunters Swarm the Island: Put {Beasts} Destroyed by Adversary rules on this panel. If there are ever more {Beasts} on this panel than on the island, the Invaders win."
- **Agreement:** `agree` — Wording differs (engine 'Track ... exceeds'; Wiki 'put on this panel ... more than'); same rule.
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**R-ADV-RUS-L1a · Russia L1 Hunters Seek Shell and Hide**

- **Rule:** During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land. Parameters: addBeastToHighestNumberLandWithoutTownCityOnEachBoard = True; addExplorerToHighestNumberLandWithoutTownCityOnEachBoard = True.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/1"; `addBeastToHighestNumberLandWithoutTownCityOnEachBoard: true`, `addExplorerToHighestNumberLandWithoutTownCityOnEachBoard: true`, `explorerDamageAdjustment: 1`): "During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land."
- **Wiki:** Russia@6851, Level 1 (difficulty 3, fear 10 (3/3/4)): "Hunters Bring Home Shell and Hide: During Setup, on each board, add 1 {Beasts} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beasts} in that land."
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-RUS-L1b · Russia L1 Hunters Seek Shell and Hide**

- **Rule:** During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land. Parameters: explorerDamageAdjustment = 1.
- **Changes:** `ravage_damage_change`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/1"; `addBeastToHighestNumberLandWithoutTownCityOnEachBoard: true`, `addExplorerToHighestNumberLandWithoutTownCityOnEachBoard: true`, `explorerDamageAdjustment: 1`): "During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land."
- **Wiki:** Russia@6851, Level 1 (difficulty 3, fear 10 (3/3/4)): "Hunters Bring Home Shell and Hide: During Setup, on each board, add 1 {Beasts} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beasts} in that land."
- **Agreement:** `agree` — Explorers deal 2 Damage.
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-ADV-RUS-L1c · Russia L1 Hunters Seek Shell and Hide**

- **Rule:** During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land. Parameters: trigger = Ravage adds Blight to a land (including cascades); destroy_beast = 1.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/1"; `addBeastToHighestNumberLandWithoutTownCityOnEachBoard: true`, `addExplorerToHighestNumberLandWithoutTownCityOnEachBoard: true`, `explorerDamageAdjustment: 1`): "During Setup, on each board, add 1 {Beast} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beast} in that land."
- **Wiki:** Russia@6851, Level 1 (difficulty 3, fear 10 (3/3/4)): "Hunters Bring Home Shell and Hide: During Setup, on each board, add 1 {Beasts} and 1 {Explorer} to the highest-numbered land without {Town}/{City}. During Play, {Explorer} do +1 Damage. When Ravage adds {Blight} to a land (including cascades), Destroy 1 {Beasts} in that land."
- **Agreement:** `agree` — Feeds the Hunters Swarm loss condition.
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-RUS-L2 · Russia L2 A Sense for Impending Disaster**

- **Rule:** The first time each Action would Destroy {Explorer}: If possible, 1 of those {Explorer} is instead Pushed; 1 {Fear} when you do so. Parameters: first_explorer_destroy_per_action = pushed instead if possible; +1 Fear.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[2]` (difficulty 4, `fearCardModifiers` "1/0/1"; no machine fields): "The first time each Action would Destroy {Explorer}: If possible, 1 of those {Explorer} is instead Pushed; 1 {Fear} when you do so."
- **Wiki:** Russia@6851, Level 2 (difficulty 4, fear 11 (4/3/4)): "A Sense for Impending Disaster: The first time each Action would Destroy {Explorer}: If possible, 1 of those {Explorer} is instead Pushed; 1 {Fear} when you do so."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-RUS-L3 · Russia L3 Competition Among Hunters**

- **Rule:** Ravage Cards also match lands with 3 or more {Explorer}. (If the land already matched the Ravage Card, it still Ravages just once per Card. ) Parameters: action = ravage; slot = Ravage; when = Ravage Cards also match lands with >= 3 Explorers (once per card).
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[3]` (difficulty 6, `fearCardModifiers` "1/1/0"; no machine fields): "Ravage Cards also match lands with 3 or more {Explorer}. (If the land already matched the Ravage Card, it still Ravages just once per Card. )"
- **Wiki:** Russia@6851, Level 3 (difficulty 6, fear 11 (4/4/3)): "Competition Among Hunters: Ravage Cards also match lands with 3 or more {Explorer}. (If the land already matched the Ravage Card, it still Ravages just once.)"
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-ADV-RUS-L4 · Russia L4 Accelerated Exploitation**

- **Rule:** When making the Invader Deck, put 1 Stage III Card after each Stage II Card. (New Deck Order: 111-2-3-2-3-2-3-2-33) Parameters: moveMultipleInvaderCards = [3, 2].
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[4]` (difficulty 7, `fearCardModifiers` "1/1/1"; `moveMultipleInvaderCards: [3, 2]`): "When making the Invader Deck, put 1 Stage III Card after each Stage II Card. (New Deck Order: 111-2-3-2-3-2-3-2-33)"
- **Wiki:** Russia@6851, Level 4 (difficulty 7, fear 12 (4/4/4)): "Accelerated Exploitation: When making the Invader Deck, put 1 Stage III Card after each Stage II Card. (New Deck Order: 111-2-3-2-3-2-3-2-33)"
- **Agreement:** `agree` — Deck 111-2-3-2-3-2-3-2-33.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-RUS-L5a · Russia L5 Entrench in the Face of Fear**

- **Rule:** Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up). "Unused" = the cards left over after the whole Invader Deck is built (adversary changes included), taken face down; they stay hidden until revealed, so the firewall never names them before. They are Invader Cards, not Fear Cards: they need no Fear to pass and show a land type. Earning the 3rd Fear Card uncovers the Stage II card, the 7th the Stage III; each goes face-up to the Build space at once (Q20). A Stage II card placed this way triggers its Escalation when first resolved (FAQ, Russia). Parameters: putUnusedStageIIIUnderFearCards = 7; putUnusedStageIIUnderFearCards = 3.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[5]` (difficulty 9, `fearCardModifiers` "1/2/1"; `putUnusedStageIIIUnderFearCards: 7`, `putUnusedStageIIUnderFearCards: 3`): "Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up)."
- **Wiki:** Russia@6851, Level 5 (difficulty 9, fear 13 (4/5/4)): "Entrench in the Face of Fear: Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up)."
- **Agreement:** `agree` — Cards placed under Fear cards #3 and #7; identities hidden, positions public.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** reviewed by Brett (2026-10-19, Q20)

**R-ADV-RUS-L5b · Russia L5 Entrench in the Face of Fear**

- **Rule:** Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up). Parameters: action = build; slot = Build; when = when revealed from the Fear Deck, placed face-up in the Build space.
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[5]` (difficulty 9, `fearCardModifiers` "1/2/1"; `putUnusedStageIIIUnderFearCards: 7`, `putUnusedStageIIUnderFearCards: 3`): "Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up)."
- **Wiki:** Russia@6851, Level 5 (difficulty 9, fear 13 (4/5/4)): "Entrench in the Face of Fear: Put an unused Stage II Invader Card under the top 3 Fear Cards, and an unused Stage III Card under the top 7 Fear Cards. When one is revealed, immediately place it in the Build space (face-up)."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** reviewed by Brett (2026-10-19, Q20)

**R-ADV-RUS-L6 · Russia L6 Pressure for Fast Profit**

- **Rule:** After the Ravage Step of turn 2+, on each board where it added no {Blight}: In the land with the most {Explorer} (min. 1), add 1 {Explorer} and 1 {Town}. Parameters: after = the Ravage Step of turn 2+; where = each board where Ravage added no Blight; adds = 1 Explorer + 1 Town in the land with most Explorers (min 1).
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheTsardomOfRussia.levels[6]` (difficulty 11, `fearCardModifiers` "2/2/1"; no machine fields): "After the Ravage Step of turn 2+, on each board where it added no {Blight}: In the land with the most {Explorer} (min. 1), add 1 {Explorer} and 1 {Town}."
- **Wiki:** Russia@6851, Level 6 (difficulty 11, fear 14 (5/5/4)): "Pressure for Fast Profit: After the Ravage Step of turn 2+, on each board where it added no {Blight}: In the land with the most {Explorer} (min. 1), add 1 {Explorer} and 1 {Town}."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

### 4.7 The Kingdom of Scotland — `TheKingdomOfScotland` (Promo Pack 2)

Wiki page `Scotland` revision 12772 (archive snapshot 20260702065756). Base difficulty: engine `baseDifficulty` = absent, Wiki = 1 (engine field absent; Wiki only).

**R-ADV-SCO-L0a · Scotland Escalation: Ports Sprawl Outward**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): On the single board with the most Coastal {Town}/{City}, add 1 {Town} to the N lands with the fewest {Town}. (N = # of players.) Timing: `before_explore`. The players choose the N lands and break ties, best for them (Q21). It happens before Exploring, so its Towns are Explore sources that turn (FAQ, NQ-G).
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.stageIIEscalationBody`: "On the single board with the most Coastal {Town}/{City}, add 1 {Town} to the N lands with the fewest {Town}. (N = # of players.)"
- **Wiki:** Scotland@12772, Escalation: "Ports Sprawl Outward: On the single board with the most Coastal {Town}/{City}, add 1 {Town} to the N lands with the fewest {Town} (N = # of players.)"; ruling: "Scotland's Escalation effect happens before Exploring."
- **Agreement:** `agree` — Engine text gives no timing; Wiki ruling: 'happens before Exploring.'
- **Affects:** invader forecast on Escalation cards
- **Status:** reviewed by Brett (2026-10-19, Q21); sourced (2026-10-19, NQ-G; FAQ "Stage II escalation")

**R-ADV-SCO-L0b · Scotland loss condition: Trade Hub**

- **Rule:** If the number of Coastal lands with {City} is ever greater than (2 x # of boards), the Invaders win.
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.additionalLossCondition`: "If the number of Coastal lands with {City} is ever greater than (2 x # of boards), the Invaders win."
- **Wiki:** Scotland@12772, Additional Loss Condition: "Trade Hub: If the number of Coastal lands with {City} is ever greater than (2 x # of boards), the Invaders win."
- **Agreement:** `agree` — Wiki ruling: uses the current number of Coastal lands and boards.
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**R-ADV-SCO-L1 · Scotland L1 Trading Port**

- **Rule:** After Setup, in Coastal lands, Explore Cards add 1 {Town} instead of 1 {Explorer}. “Coastal Lands” Invader Cards do this for maximum 2 lands per board. Parameters: coastal_explore_adds = 1 Town instead of 1 Explorer; coastal_lands_card_limit = max 2 lands per board; from = after Setup.
- **Changes:** `explore_change`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[1]` (difficulty 3, `fearCardModifiers` "0/1/0"; no machine fields): "After Setup, in Coastal lands, Explore Cards add 1 {Town} instead of 1 {Explorer}. “Coastal Lands” Invader Cards do this for maximum 2 lands per board."
- **Wiki:** Scotland@12772, Level 1 (difficulty 3, fear 10 (3/4/3)): "Trading Port: After Setup, in Coastal lands, Explore Cards add 1 {Town} instead of 1 {Explorer} . "Coastal Lands" Invader cards do this for maximum 2 lands per board."
- **Agreement:** `agree` — Wiki Explorer errata: an Explore adding 2 Explorers adds 1 Town + 1 Explorer. Wiki Invader FAQ: Towns added by Explore are not sources for that Explore.
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-ADV-SCO-L2a · Scotland L2 Seize Opportunity**

- **Rule:** During Setup, add 1 {City} to land #2. Place “Coastal Lands” as the 3rd Stage II Card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.) Parameters: addCityToLandNumberOnEachBoard = 2.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[2]` (difficulty 4, `fearCardModifiers` "1/1/0"; `addCityToLandNumberOnEachBoard: 2`, `moveTopInvaderCardPositions: [2, 2, 1]`, `removeInvaderCardBeforeRandomizing: "Coastal2"`, `replaceInvaderCardPosition: [2, 2]`, `replaceInvaderCardWithIdentifier: "Coastal2"`): "During Setup, add 1 {City} to land #2. Place “Coastal Lands” as the 3rd Stage II Card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.)"
- **Wiki:** Scotland@12772, Level 2 (difficulty 4, fear 11 (4/4/3)): "Seize Opportunity: During Setup, add 1 {City} to land #2. Place "Coastal Lands" as the 3rd Stage II card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.)"
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**R-ADV-SCO-L2b · Scotland L2 Seize Opportunity**

- **Rule:** During Setup, add 1 {City} to land #2. Place “Coastal Lands” as the 3rd Stage II Card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.) Parameters: moveTopInvaderCardPositions = [2, 2, 1]; removeInvaderCardBeforeRandomizing = Coastal2; replaceInvaderCardPosition = [2, 2]; replaceInvaderCardWithIdentifier = Coastal2.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[2]` (difficulty 4, `fearCardModifiers` "1/1/0"; `addCityToLandNumberOnEachBoard: 2`, `moveTopInvaderCardPositions: [2, 2, 1]`, `removeInvaderCardBeforeRandomizing: "Coastal2"`, `replaceInvaderCardPosition: [2, 2]`, `replaceInvaderCardWithIdentifier: "Coastal2"`): "During Setup, add 1 {City} to land #2. Place “Coastal Lands” as the 3rd Stage II Card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.)"
- **Wiki:** Scotland@12772, Level 2 (difficulty 4, fear 11 (4/4/3)): "Seize Opportunity: During Setup, add 1 {City} to land #2. Place "Coastal Lands" as the 3rd Stage II card, and move the two Stage II Cards above it up by one. (New Deck Order: 11-22-1-C2-33333, where C is the Stage II Coastal Lands Card.)"
- **Agreement:** `agree` — Deck 11-22-1-C2-33333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-SCO-L3 · Scotland L3 Chart the Coastline**

- **Rule:** In Coastal lands, Build Cards affect lands without Invaders, so long as there is an adjacent {City}. Parameters: builds_in_lands_without_invaders = True; where = Coastal lands matching a Build card; condition = an adjacent City.
- **Changes:** `build_change`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[3]` (difficulty 6, `fearCardModifiers` "1/2/1"; no machine fields): "In Coastal lands, Build Cards affect lands without Invaders, so long as there is an adjacent {City}."
- **Wiki:** Scotland@12772, Level 3 (difficulty 6, fear 13 (4/5/4)): "Chart the Coastline: In Coastal lands, Build Cards affect lands without Invaders, so long as there is an adjacent {City}."
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-ADV-SCO-L4 · Scotland L4 Ambition of a Minor Nation**

- **Rule:** During Setup, replace the bottom Stage I Card with the bottom Stage III Card. (New Deck Order: 11-22-3-C2-3333) Parameters: moveInvaderCard = [3, 1]; replaceInsteadOfMove = True.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[4]` (difficulty 7, `fearCardModifiers` "2/2/1"; `moveInvaderCard: [3, 1]`, `replaceInsteadOfMove: true`): "During Setup, replace the bottom Stage I Card with the bottom Stage III Card. (New Deck Order: 11-22-3-C2-3333)"
- **Wiki:** Scotland@12772, Level 4 (difficulty 7, fear 14 (5/5/4)): "Ambition of a Minor Nation: During Setup, replace the bottom Stage I Card with the bottom Stage III Card. (New Deck Order: 11-22-3-C2-3333))"
- **Agreement:** `agree` — Deck 11-22-3-C2-3333.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-SCO-L5 · Scotland L5 Runoff and Bilgewater**

- **Rule:** After a Ravage Action adds {Blight} to a Coastal land, add 1 {Blight} to that board’s Ocean (without cascading). Treat the Ocean as a Coastal Wetland for this rule and for {Blight} removal/movement. Parameters: trigger = Ravage adds Blight to a Coastal land; adds = 1 Blight to that board's Ocean, no cascade; ocean_as = Coastal Wetland for this rule and Blight removal/movement.
- **Changes:** `ravage_blight_change`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[5]` (difficulty 8, `fearCardModifiers` "2/3/1"; no machine fields): "After a Ravage Action adds {Blight} to a Coastal land, add 1 {Blight} to that board’s Ocean (without cascading). Treat the Ocean as a Coastal Wetland for this rule and for {Blight} removal/movement."
- **Wiki:** Scotland@12772, Level 5 (difficulty 8, fear 15 (5/6/4)): "Runoff and Bilgewater: After a Ravage Action adds {Blight} to a Coastal Land, add 1 {Blight} to that board's Ocean (without cascading). Treat the Ocean as a Coastal Wetland for this rule and for {Blight} removal/movement."
- **Agreement:** `agree`
- **Affects:** Ravage ledger (blight added); blight runway
- **Status:** unreviewed

**R-ADV-SCO-L6 · Scotland L6 Exports Fuel Inward Growth**

- **Rule:** After the Ravage Step, add 1 {Town} to each Inland land that matches a Ravage Card and is within {1Range} of {Town}/{City}. Parameters: after = the Ravage Step; adds = 1 Town to each Inland land matching a Ravage card within 1 Range of Town/City.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `TheKingdomOfScotland.levels[6]` (difficulty 10, `fearCardModifiers` "3/3/1"; no machine fields): "After the Ravage Step, add 1 {Town} to each Inland land that matches a Ravage Card and is within {1Range} of {Town}/{City}."
- **Wiki:** Scotland@12772, Level 6 (difficulty 10, fear 16 (6/6/4)): "Exports Fuel Inward Growth: After the Ravage step, add 1 {Town} to each Inland land that matches a Ravage card and is within 1"
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

### 4.8 Habsburg Mining Expedition — `HabsburgMiningExpedition` (Nature Incarnate)

Wiki page `Habsburg_Mining_Expedition` revision 12138 (archive snapshot 20250427153340). Base difficulty: engine `baseDifficulty` = 1, Wiki = 1. Brett's panel image confirms difficulty 1; L1–L6 3, 4, 5, 7, 9, 10 and Fear Decks 9 (3/3/3), 10 (3/3/4), 11 (3/4/4), 12 (4/4/4), 13 (4/5/4), 13 (4/5/4) (open-questions checklist, 2026-10-19).

**R-ADV-HME-L0a · Habsburg (Mining) Escalation: Mining Tunnels**

- **Rule:** Each time a Stage II card with the Escalation flag is resolved for the first time (Wiki Adversary@12228): After Advancing Invader Cards: On each board, Explore in 2 lands whose terrains don't match a Ravage or Build Card (no source required). Timing: `after_advance`.
- **Changes:** `escalation`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.stageIIEscalationBody`: "After Advancing Invader Cards: On each board, Explore in 2 lands whose terrains don't match a Ravage or Build Card (no source required)."
- **Wiki:** Habsburg_Mining_Expedition@12138, Escalation: "Mining Tunnels: After Advancing Invader Cards: On each board, Explore in 2 lands whose terrains don't match a Ravage or Build Card (no source required)."
- **Agreement:** `agree`
- **Affects:** invader forecast on Escalation cards
- **Status:** unreviewed

**R-ADV-HME-L0b · Habsburg (Mining) loss condition: Land Stripped Bare**

- **Rule:** At the end of the {Fast} Phase, the Invaders win if any land has at least 8 total Invaders/{Blight} (combined).
- **Changes:** `loss_condition`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.additionalLossCondition`: "At the end of the {Fast} Phase, the Invaders win if any land has at least 8 total Invaders/{Blight} (combined)."
- **Wiki:** Habsburg_Mining_Expedition@12138, Additional Loss Condition: "Land Stripped Bare: At the end of the {Fasticon.png} Phase, the Invaders win if any land has at least 8 total Invaders/{Blight} (combined)."
- **Agreement:** `agree` — Checked at the end of the Fast Phase (both sources).
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**R-ADV-HME-L1a · Habsburg (Mining) L1 Avarice Rewarded + Ceaseless Mining**

- **Rule:** When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack). Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions). Parameters: trigger = Blight added by a Ravage would cascade; instead = Upgrade 1 Explorer/Town (before Dahan counterattack).
- **Changes:** `cascade_change`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/0"; no machine fields): "When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack). Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions)."
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 1 (difficulty 3, fear 9 (3/3/3)): "Avarice Rewarded: When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack.) Ceaseless Mining: Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions)."
- **Agreement:** `agree`
- **Affects:** cascade badge; blight runway; presence at risk
- **Status:** unreviewed

**R-ADV-HME-L1b · Habsburg (Mining) L1 Avarice Rewarded + Ceaseless Mining**

- **Rule:** When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack). Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions). Parameters: action = ravage; slot = Build; when = Build Step: in Mining lands (>= 3 Invaders) Build Cards cause Ravage Actions instead of Build Actions; Disease affects those Ravages as if Builds.
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[1]` (difficulty 3, `fearCardModifiers` "0/0/0"; no machine fields): "When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack). Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions)."
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 1 (difficulty 3, fear 9 (3/3/3)): "Avarice Rewarded: When {Blight} added by a Ravage Action would cascade, instead Upgrade 1 {Explorer}/{Town} (before {Dahan} counterattack.) Ceaseless Mining: Lands with 3 or more Invaders are Mining lands. In Mining lands: • {Disease} and modifiers to {Disease} affect Ravage Actions as though they were Build Actions. • During the Build Step, Build Cards cause Ravage Actions (instead of Build Actions)."
- **Agreement:** `agree` — Ceaseless Mining (additionalGameEffectBodies).
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-ADV-HME-L2 · Habsburg (Mining) L2 Miners Come From Far and Wide**

- **Rule:** Setup: Add 1 {Explorer} in each land with no {Dahan}. On each board, Add 1 {Disease} and 1 {City} in the highest-numbered land with a {Town} Setup symbol. Parameters: adds = 1 Explorer in each land with no Dahan; per board 1 Disease + 1 City in the highest-numbered land with a Town Setup symbol; machine_fields = none in engine defs.
- **Changes:** `setup_pieces`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[2]` (difficulty 4, `fearCardModifiers` "0/0/1"; no machine fields): "Setup: Add 1 {Explorer} in each land with no {Dahan}. On each board, Add 1 {Disease} and 1 {City} in the highest-numbered land with a {Town} Setup symbol."
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 2 (difficulty 4, fear 10 (3/3/4)): "Miners Come From Far and Wide: Setup: Add 1 {Explorer} in each land with no {Dahan}. Add 1 {Disease} and 1 {City} in the highest-numbered land with a {Town} Setup symbol."
- **Agreement:** `conflict` — Engine: 'On each board, Add 1 Disease and 1 City'. Wiki omits 'On each board'.
- **Resolution:** On each board. FAQ errata (Habsburg Mining Expedition, NI 1st printing): "Add 1 Disease and 1 City in the highest-numbered land with a Town Setup symbol on each board; don't just add them to the single highest-numbered land … on the entire island." Brett's panel image prints the text without "on each board"; the errata adds it (NQ-B).
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** sourced (2026-10-19, NQ-B; FAQ errata)

**R-ADV-HME-L3 · Habsburg (Mining) L3 Mining Boom (I)**

- **Rule:** After the Build Step, on each board: Choose a land with {Explorer}. Upgrade 1 {Explorer} there. Parameters: after = the Build Step; per_board = Upgrade 1 Explorer in a chosen land with Explorer.
- **Changes:** `other`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[3]` (difficulty 5, `fearCardModifiers` "0/1/1"; no machine fields): "After the Build Step, on each board: Choose a land with {Explorer}. Upgrade 1 {Explorer} there."
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 3 (difficulty 5, fear 11 (3/4/4)): "Mining Boom (I): After the Build Step, on each board: Choose a land with {Explorer}. Upgrade 1 {Explorer} there."
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-ADV-HME-L4 · Habsburg (Mining) L4 Untapped Salt Deposits**

- **Rule:** Setup: Remove the Stage II “Coastal Lands” card before randomly choosing Stage II cards. Place the “Salt Deposits” card in place of the 2nd Stage II card. (New Deck Order: 111-2S22-33333, where S is the Salt Deposits card. {Escalation} ignores S.) Parameters: additionalInvaderCards = [{"adversaryIdentifier": "HabsburgMiningExpedition", "adversaryLevelNumber": 4, "body": ["Explore/Build Space:", "Non-Mining Lands", "", "Ravage Space:", "Mining Lands", "<size=70%>&", "Do not advance this card</size>"], "flippedBody": ["This card cannot be removed from the Invader deck"], "identifier": "SaltDeposits", "landTypes": ["SaltDeposits"], "stage": 2, "title": "Salt Deposits"}]; removeInvaderCardBeforeRandomizing = Coastal2; replaceInvaderCardPosition = [2, 1]; replaceInvaderCardWithIdentifier = SaltDeposits.
- **Changes:** `deck_recipe`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[4]` (difficulty 7, `fearCardModifiers` "1/1/1"; `removeInvaderCardBeforeRandomizing: "Coastal2"`, `replaceInvaderCardPosition: [2, 1]`, `replaceInvaderCardWithIdentifier: "SaltDeposits"`, `additionalInvaderCards: [SaltDeposits …]`): "Setup: Remove the Stage II “Coastal Lands” card before randomly choosing Stage II cards. Place the “Salt Deposits” card in place of the 2nd Stage II card. (New Deck Order: 111-2S22-33333, where S is the Salt Deposits card. {Escalation} ignores S.)"
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 4 (difficulty 7, fear 12 (4/4/4)): "Untapped Salt Deposits: Setup: Remove the Stage II 'Coastal Lands' card before randomly choosing Stage II cards. Place the 'Salt Deposits' card in place of the 2nd Stage II card. (New Deck Order: 111-2S22-33333, where S is the Salt Deposits card. Escalation ignores S.)"
- **Agreement:** `agree` — Deck 111-2S22-33333; Escalation ignores Salt Deposits. Salt Deposits: Explore/Build on Non-Mining lands, Ravage on Mining lands, does not advance from Ravage.
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** unreviewed

**R-ADV-HME-L5 · Habsburg (Mining) L5 Mining Boom (II)**

- **Rule:** Instead of Mining Boom (I), after the Build Step, on each board: Choose a land with {Explorer}. Build there, then Upgrade 1 {Explorer}. (Build normally in a Mining land.) Parameters: action = build; slot = after Build Step; when = per board, a chosen land with Explorer: Build there (normal Build even in a Mining land), then Upgrade 1 Explorer; replaces Mining Boom (I).
- **Changes:** `extra_invader_action`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[5]` (difficulty 9, `fearCardModifiers` "1/2/1"; no machine fields): "Instead of Mining Boom (I), after the Build Step, on each board: Choose a land with {Explorer}. Build there, then Upgrade 1 {Explorer}. (Build normally in a Mining land.)"
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 5 (difficulty 9, fear 13 (4/5/4)): "Mining Boom (II): Instead of Mining Boom (I), after the Build Step, on each board: Choose a land with {Explorer}. Build there, then Upgrade 1 {Explorer}. (Build normally in a Mining land.)"
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-ADV-HME-L6 · Habsburg (Mining) L6 The Empire Ascendant**

- **Rule:** Setup and During the Explore Step: On boards with 3 or fewer {Blight}, Add +1 {Explorer} in each land successfully explored. (Max. 2 lands per board per Explore Card.) Parameters: plus_explorer = 1; where = each land successfully explored, boards with <= 3 Blight; limit = 2 lands per board per Explore Card; also = Setup.
- **Changes:** `explore_change`
- **Engine:** `adversaries.json` › `HabsburgMiningExpedition.levels[6]` (difficulty 10, `fearCardModifiers` "1/2/1"; no machine fields): "Setup and During the Explore Step: On boards with 3 or fewer {Blight}, Add +1 {Explorer} in each land successfully explored. (Max. 2 lands per board per Explore Card.)"
- **Wiki:** Habsburg_Mining_Expedition@12138, Level 6 (difficulty 10, fear 13 (4/5/4)): "The Empire Ascendant: Setup and During the Explore Step: On boards with 3 or fewer {Blight}: Add +1 {Explorer} in each land successfully explored. (Max. 2 lands per board per Explore Card.)"
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

## 5. Spirits, scenarios, events, and decks that change the pipeline


### 5.1 Spirits (all 34 in `spirits.json`, with their 24 aspects)

Scan: every `specialRules` body and every `innatePowers` option of all 34 spirits and all 24 aspects in `spirits.json`. An entry is listed when it changes Explore, Build, Ravage (damage, Defend, participation, health), blight/cascade, adjacency or Isolate, Dahan counterattack, Invader-deck order, or Power speed. Ordinary Damage, Push, Gather, Fear and Energy effects are not listed. Wiki evidence is the repo fetcher's spirit JSON (`data/references/wiki/<slug>.json`, committed 2026-04-19; the fetcher does not record revision ids). Aspect pages were not fetched, so aspect entries are `engine-only`.


**A Spread of Rampant Green** (`ASpreadOfRampantGreen`, Base game)

**R-SP-ASpreadOfRampantGreen-1 · A Spread of Rampant Green — Choke the Land with Green**

- **Rule:** Whenever Invaders would Ravage or Build in a land with your {SacredSite}, you may skip it by destroying one of your {Presence} in that land. Parameters: actions = ["ravage", "build"]; where = land with your Sacred Site; cost = destroy 1 of your Presence in that land; optional = True.
- **Changes:** `skip_invader_action`
- **Engine:** `spirits.json` › `ASpreadOfRampantGreen` › `specialRules[ChokeTheLandWithGreen].body`: "Whenever Invaders would Ravage or Build in a land with your {SacredSite}, you may skip it by destroying one of your {Presence} in that land."
- **Wiki:** `data/references/wiki/a-spread-of-rampant-green.json` (fetcher output, revid not recorded): "Whenever Invaders would Ravage or Build in a land with your Sacred Site, you may prevent it by destroying one of your Presence in that land." (text similarity 0.96)
- **Agreement:** `agree` — Wiki: 'prevent it'; engine: 'skip it'. The_Dahan@7991 FAQ names Choke the Land with Green: a prevented Ravage gets no Dahan counterattack.
- **Affects:** invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage)
- **Status:** unreviewed

**R-SP-ASpreadOfRampantGreen-2 · A Spread of Rampant Green — All-Enveloping Green**

- **Rule:** (Fast) 3 Plant+1 Water: Defend 2. / 4 Plant+2 Water: Instead, Defend 4. / 1 Earth+5 Plant+3 Water: Also, remove 1 {Blight}. Parameters: mechanism = defend; levels = [2, 4]; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `ASpreadOfRampantGreen` › `innatePowers[AllEnvelopingGreen]`: "(Fast) 3 Plant+1 Water: Defend 2. / 4 Plant+2 Water: Instead, Defend 4. / 1 Earth+5 Plant+3 Water: Also, remove 1 {Blight}."
- **Wiki:** `data/references/wiki/a-spread-of-rampant-green.json` (fetcher output, revid not recorded): "Defend 2. / Instead, Defend 4. / Also, remove 1 Blight." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SP-ASpreadOfRampantGreen-3 · A Spread of Rampant Green (Tangles aspect) — Impenetrable Tangles of Greenery**

- **Rule:** (Fast) 2 Plant+1 Sun: Add 1 {Wilds}. / 3 Plant+2 Sun: Push 1 {Explorer}. / 4 Plant+2 Sun: Isolate target land. / 5 Plant+3 Sun: Downgrade 1 {City}. Add 1 {Wilds}. Parameters: level1 = Add 1 Wilds; speed = fast.
- **Changes:** `explore_change`
- **Engine:** `spirits.json` › `ASpreadOfRampantGreen` › aspects[Tangles] › `innatePowers[ImpenetrableTanglesOfGreenery]`: "(Fast) 2 Plant+1 Sun: Add 1 {Wilds}. / 3 Plant+2 Sun: Push 1 {Explorer}. / 4 Plant+2 Sun: Isolate target land. / 5 Plant+3 Sun: Downgrade 1 {City}. Add 1 {Wilds}."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only` — Wilds: the next Explore into that land removes a Wilds instead (Wiki Wilds page).
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-SP-ASpreadOfRampantGreen-4 · A Spread of Rampant Green (Tangles aspect) — Impenetrable Tangles of Greenery**

- **Rule:** (Fast) 2 Plant+1 Sun: Add 1 {Wilds}. / 3 Plant+2 Sun: Push 1 {Explorer}. / 4 Plant+2 Sun: Isolate target land. / 5 Plant+3 Sun: Downgrade 1 {City}. Add 1 {Wilds}. Parameters: level3 = Isolate target land; speed = fast.
- **Changes:** `isolate`
- **Engine:** `spirits.json` › `ASpreadOfRampantGreen` › aspects[Tangles] › `innatePowers[ImpenetrableTanglesOfGreenery]`: "(Fast) 2 Plant+1 Sun: Add 1 {Wilds}. / 3 Plant+2 Sun: Push 1 {Explorer}. / 4 Plant+2 Sun: Isolate target land. / 5 Plant+3 Sun: Downgrade 1 {City}. Add 1 {Wilds}."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Explore eligibility (isolated land neither explores nor is a source)
- **Status:** unreviewed

**Bringer of Dreams and Nightmares** (`BringerOfDreamsAndNightmares`, Base game)

**R-SP-BringerOfDreamsAndNightmares-1 · Bringer of Dreams and Nightmares — To Dream a Thousand Deaths**

- **Rule:** Your Powers never cause Damage, nor can they Destroy anything other than your own {Presence}. / When your Powers would Destroy (or deal enough Damage to Destroy) {Explorer}/{Town}/{City}, generate 0/2/5 {Fear} instead. The Power Pushes all {Explorer}/{Town} it would destroy. / Notes: A single Power cannot Destroy a given Invader more than once. Powers which cause Damage via {Dahan} are affected just like all others. All effects other than Damage/Destroy work as usual. Parameters: powers_destroy_invaders = False; instead = Fear 0/2/5 and Push the Explorer/Town; effect = pushed Invaders stay on the island and keep Building/Ravaging/acting as Explore sources.
- **Changes:** `other`
- **Engine:** `spirits.json` › `BringerOfDreamsAndNightmares` › `specialRules[ToDreamAThousandDeaths].body`: "Your Powers never cause Damage, nor can they Destroy anything other than your own {Presence}. / When your Powers would Destroy (or deal enough Damage to Destroy) {Explorer}/{Town}/{City}, generate 0/2/5 {Fear} instead. The Power Pushes all {Explorer}/{Town} it would destroy. / Notes: A single Power cannot Destroy a given Invader more than once. Powers which cause Damage via {Dahan} are affected just like all others. All effects other than Damage/Destroy work as usual."
- **Wiki:** `data/references/wiki/bringer-of-dreams-and-nightmares.json` (fetcher output, revid not recorded): "Your Powers never cause Damage, nor can they Destroy anything other than your own Presence. When your Powers would Destroy (or deal enough Damage to Destroy) Explorer/Town/City, instead generate 0/2/5 Fear. The Power Pushes all Explorer/Town it would Destroy. Notes: A single Power cannot Destroy a given Invader more than once. Powers that cause Damage via Dahan are affected just like all others. All effects other than Damage/Destroy work as usual." (text similarity 0.89)
- **Agreement:** `agree` — Violence aspect replaces this rule with Fear 1/3/5; same pipeline effect.
- **Affects:** see note
- **Status:** unreviewed

**Lightning's Swift Strike** (`LightningsSwiftStrike`, Base game)

**R-SP-LightningsSwiftStrike-1 · Lightning's Swift Strike — Swiftness of Lightning**

- **Rule:** For every {Air} you have, you may use 1 {Slow} Power as if it were {Fast}. (Power Cards or your Innate Powers) Parameters: timing = per Air, use 1 Slow Power as if Fast.
- **Changes:** `other`
- **Engine:** `spirits.json` › `LightningsSwiftStrike` › `specialRules[SwiftnessOfLightning].body`: "For every {Air} you have, you may use 1 {Slow} Power as if it were {Fast}. (Power Cards or your Innate Powers)"
- **Wiki:** `data/references/wiki/lightning-swift-strike.json` (fetcher output, revid not recorded): "For every Air you have, you may use 1 Slow Power as if it were Fast. (Power Cards or your Innate Powers.)" (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-SP-LightningsSwiftStrike-2 · Lightning's Swift Strike (Wind aspect) — Exaltation of the Storm-Wind**

- **Rule:** (Fast) 1 Air: You and target Spirit may each make one of your non-Major {Slow} Powers {Fast}. / 3 Air: You and target Spirit each gain {+1Range} with all your Powers this turn. / 4 Air+1 Water: You and target Spirit may each Push up to 2 {Explorer} from one of your respective lands. / 5 Air+2 Water: Repeat this Power. Parameters: timing = you and target Spirit may each make one non-Major Slow Power Fast; speed = fast.
- **Changes:** `other`
- **Engine:** `spirits.json` › `LightningsSwiftStrike` › aspects[Wind] › `innatePowers[ExaltationOfTheStormWind]`: "(Fast) 1 Air: You and target Spirit may each make one of your non-Major {Slow} Powers {Fast}. / 3 Air: You and target Spirit each gain {+1Range} with all your Powers this turn. / 4 Air+1 Water: You and target Spirit may each Push up to 2 {Explorer} from one of your respective lands. / 5 Air+2 Water: Repeat this Power."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** see note
- **Status:** unreviewed

**R-SP-LightningsSwiftStrike-3 · Lightning's Swift Strike (Pandemonium aspect) — Lightning-Torn Skies Incite Pandemonium**

- **Rule:** (Slow) 2 Air+3 Fire: 2 {Fear}. Add 1 {Strife}. / 3 Air+4 Fire: 2 {Fear}. Add 1 {Strife}. / 4 Air+5 Fire+1 Moon: 3 {Fear}. Add 1 {Strife}. / 5 Air+5 Fire+2 Moon: 4 {Fear}. Add 1 {Strife}. Parameters: mechanism = Strife; speed = slow.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `LightningsSwiftStrike` › aspects[Pandemonium] › `innatePowers[LightningTornSkiesIncitePandemonium]`: "(Slow) 2 Air+3 Fire: 2 {Fear}. Add 1 {Strife}. / 3 Air+4 Fire: 2 {Fear}. Add 1 {Strife}. / 4 Air+5 Fire+1 Moon: 3 {Fear}. Add 1 {Strife}. / 5 Air+5 Fire+2 Moon: 4 {Fear}. Add 1 {Strife}."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only` — Slow Strife persists to the next turn's Ravage (Strife is removed only when used).
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Ocean's Hungry Grasp** (`OceansHungryGrasp`, Base game)

**R-SP-OceansHungryGrasp-1 · Ocean's Hungry Grasp — Ocean in Play**

- **Rule:** You may place/move {Presence} into Oceans, but may not place/move {Presence} into Inland lands. Oceans on boards with any of your {Presence} are treated as Coastal Wetlands for Powers and {Blight}. Any Invaders or {Dahan} moved to those Oceans are Drowned. Oceans take cascade Blight only with Ocean's Hungry Grasp in play (Q5). With Ocean's Hungry Grasp on a board, Powers and Special Rules may measure Range through that board's Ocean; otherwise the Ocean is not a land and Range never counts through it (Q29; FAQ, Ocean's Hungry Grasp). Parameters: oceans = on boards with your Presence, treated as Coastal Wetlands for Powers and Blight; drown = Invaders/Dahan moved there are Destroyed.
- **Changes:** `cascade_change`
- **Engine:** `spirits.json` › `OceansHungryGrasp` › `specialRules[OceanInPlay].body`: "You may place/move {Presence} into Oceans, but may not place/move {Presence} into Inland lands. Oceans on boards with any of your {Presence} are treated as Coastal Wetlands for Powers and {Blight}. Any Invaders or {Dahan} moved to those Oceans are Drowned."
- **Wiki:** `data/references/wiki/ocean-hungry-grasp.json` (fetcher output, revid not recorded): "You may add/move Presence into Oceans, but may not add/move Presence into Inland lands. On boards where you have 1 or more Presence, Oceans are treated as Coastal Wetlands for Spirit Powers and Blight. You Drown any Invaders or Dahan moved to those Oceans. DROWNING Destroy Drowned pieces, placing Drowned Invaders here. At any time you may exchange (X) Health of these Invaders for 1 Energy, where X = number of players. (Ignore modifiers to Invader Health.)" (text similarity 0.27)
- **Agreement:** `agree` — Engine 'for Powers'; Wiki 'for Spirit Powers'. Read manually: same rule; similarity is low because the Wiki string runs on into the next rule.
- **Affects:** cascade badge; blight runway; presence at risk; Range in the need finder (Q29)
- **Status:** reviewed by Brett (2026-10-19, Q5); sourced (2026-10-19, Q29; FAQ "Ocean's Hungry Grasp")

**R-SP-OceansHungryGrasp-2 · Ocean's Hungry Grasp (Deeps aspect) — Reclaimed by the Deeps**

- **Rule:** (Slow) 2 Water: 1 Damage per {Deeps}, to {Town}/{City} only. / 2 Earth+4 Water: If at least 3 {Deeps} and no {Town}/{City} are present: Drown all {Explorer}/{Dahan}. Remove 2 {Deeps} and Push all remaining {Deeps}. Target land permanently becomes part of the Ocean on its board. / 2 Moon+3 Water: Repeat this Power in target land or in a land with your {Presence}. Parameters: level2 = target land permanently becomes part of the Ocean on its board; speed = slow.
- **Changes:** `adjacency_change`
- **Engine:** `spirits.json` › `OceansHungryGrasp` › aspects[Deeps] › `innatePowers[ReclaimedByTheDeeps]`: "(Slow) 2 Water: 1 Damage per {Deeps}, to {Town}/{City} only. / 2 Earth+4 Water: If at least 3 {Deeps} and no {Town}/{City} are present: Drown all {Explorer}/{Dahan}. Remove 2 {Deeps} and Push all remaining {Deeps}. Target land permanently becomes part of the Ocean on its board. / 2 Moon+3 Water: Repeat this Power in target land or in a land with your {Presence}."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only` — Neighbors of that land become Coastal: they gain the Ocean as an Explore source.
- **Affects:** Explore sources; cascade targets; Range
- **Status:** unreviewed

**River Surges in Sunlight** (`RiverSurgesInSunlight`, Base game) — base panel: none found; aspect entries only.

**R-SP-RiverSurgesInSunlight-1 · River Surges in Sunlight (Haven aspect) — Call to a Sunlit Haven**

- **Rule:** (Fast) 1 Sun+1 Water: Defend E. / 1 Earth+1 Sun+2 Water: In an adjacent land with your {Presence}, Defend E. / 1 Animal+1 Sun: In target land or adjacent land with your {Presence}, Gather up to E {Dahan}. / 2 Plant+1 Sun+1 Water: Remove up to E Health worth of Invaders. Parameters: mechanism = defend; amount = E; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `RiverSurgesInSunlight` › aspects[Haven] › `innatePowers[CallToASunlitHaven]`: "(Fast) 1 Sun+1 Water: Defend E. / 1 Earth+1 Sun+2 Water: In an adjacent land with your {Presence}, Defend E. / 1 Animal+1 Sun: In target land or adjacent land with your {Presence}, Gather up to E {Dahan}. / 2 Plant+1 Sun+1 Water: Remove up to E Health worth of Invaders."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SP-RiverSurgesInSunlight-2 · River Surges in Sunlight (Travel aspect) — People Tend To The River, River Tends To The People**

- **Rule:** Your lands with 4 or more {Dahan} are considered {SacredSite}, and have Defend 1. Parameters: mechanism = defend; amount = 1; where = your lands with 4+ Dahan; always_on = True.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `RiverSurgesInSunlight` › aspects[Travel] › `specialRules[PeopleTendToTheRiverRiverTendsToThePeople].body`: "Your lands with 4 or more {Dahan} are considered {SacredSite}, and have Defend 1."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Shadows Flicker Like Flame** (`ShadowsFlickerLikeFlame`, Base game) — base panel: none found; aspect entries only.

**R-SP-ShadowsFlickerLikeFlame-1 · Shadows Flicker Like Flame (Madness aspect) — Shadows Cast a Subtle Madness**

- **Rule:** When you add {Presence} during Growth, you may also add 1 {Strife} in that land. Parameters: mechanism = Strife; when = adding Presence during Growth (Spirit Phase, before this turn's Ravage).
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `ShadowsFlickerLikeFlame` › aspects[Madness] › `specialRules[ShadowsCastASubtleMadness].body`: "When you add {Presence} during Growth, you may also add 1 {Strife} in that land."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Thunderspeaker** (`Thunderspeaker`, Base game)

**R-SP-Thunderspeaker-1 · Thunderspeaker — Sworn to Victory**

- **Rule:** After Invaders destroy {Dahan} during a Ravage, for each destroyed {Dahan} Destroy 1 of your {Presence} within {1Range}. / (Resolve the entire Ravage in that land – including, if relevant, losing {Presence} to {Blight}, cascading, and counterattack – before destroying {Presence} from this special rule.) Parameters: after_ravage_destroys_dahan = destroy 1 of your Presence within 1 Range per Dahan; order = after the whole Ravage in that land: blight, cascade, counterattack.
- **Changes:** `dahan_change`
- **Engine:** `spirits.json` › `Thunderspeaker` › `specialRules[SwornToVictory].body`: "After Invaders destroy {Dahan} during a Ravage, for each destroyed {Dahan} Destroy 1 of your {Presence} within {1Range}. / (Resolve the entire Ravage in that land – including, if relevant, losing {Presence} to {Blight}, cascading, and counterattack – before destroying {Presence} from this special rule.)"
- **Wiki:** `data/references/wiki/thunderspeaker.json` (fetcher output, revid not recorded): "After a Ravage Action destroys 1 or more Dahan, for each Dahan Destroyed, Destroy 1 of your Presence within Range 1." (text similarity 0.38)
- **Agreement:** `agree` — Read manually: the engine adds the resolution-order parenthetical; same rule.
- **Affects:** Ravage ledger (Dahan damage, counterattack)
- **Status:** unreviewed

**R-SP-Thunderspeaker-2 · Thunderspeaker — Lead the Furious Assault**

- **Rule:** (Slow) 4 Air: This power may be {Fast}. / 1 Fire+2 Sun: Destroy 1 {Town} for every 2 {Dahan} in target land. / 3 Fire+4 Sun: Destroy 1 {City} for every 3 {Dahan} in target land. Parameters: timing = Slow; Air 4: may be Fast.
- **Changes:** `other`
- **Engine:** `spirits.json` › `Thunderspeaker` › `innatePowers[LeadTheFuriousAssault]`: "(Slow) 4 Air: This power may be {Fast}. / 1 Fire+2 Sun: Destroy 1 {Town} for every 2 {Dahan} in target land. / 3 Fire+4 Sun: Destroy 1 {City} for every 3 {Dahan} in target land."
- **Wiki:** `data/references/wiki/thunderspeaker.json` (fetcher output, revid not recorded): "This Power may be Fast. / Destroy 1 Town for every 2 Dahan in target land. / Destroy 1 City for every 3 Dahan in target land." (text similarity 1.00)
- **Agreement:** `agree` — GatherTheWarriors has the same Air 4 option.
- **Affects:** see note
- **Status:** unreviewed

**Vital Strength of the Earth** (`VitalStrengthOfTheEarth`, Base game)

**R-SP-VitalStrengthOfTheEarth-1 · Vital Strength of the Earth — Earth's Vitality**

- **Rule:** Defend 3 in every land where you have {SacredSite}. Parameters: mechanism = defend; amount = 3; where = every land with your Sacred Site; always_on = True.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `VitalStrengthOfTheEarth` › `specialRules[EarthsVitality].body`: "Defend 3 in every land where you have {SacredSite}."
- **Wiki:** `data/references/wiki/vital-strength-of-the-earth.json` (fetcher output, revid not recorded): "Defend 3 in every land where you have Sacred Site." (text similarity 0.99)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SP-VitalStrengthOfTheEarth-2 · Vital Strength of the Earth (Resilience aspect) — Anchor the Land's Resilience**

- **Rule:** In lands with your {SacredSite}, the land has +8 Health. / (So, usually, 10 Damage will add {Blight}. This does not protect {Dahan} at all; Damage to them is unaffected.) Land-Health modifiers add together and the total can't go below 1 (NI p.14; NQ-I). Parameters: land_health_plus = 8; where = lands with your Sacred Site; dahan_unaffected = True.
- **Changes:** `ravage_blight_change`
- **Engine:** `spirits.json` › `VitalStrengthOfTheEarth` › aspects[Resilience] › `specialRules[AnchorTheLandsResilience].body`: "In lands with your {SacredSite}, the land has +8 Health. / (So, usually, 10 Damage will add {Blight}. This does not protect {Dahan} at all; Damage to them is unaffected.)"
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (blight added); blight runway
- **Status:** sourced (2026-10-19, NQ-I; NI rulebook p.14)

**Keeper of the Forbidden Wilds** (`KeeperOfTheForbiddenWilds`, Branch & Claw)

**R-SP-KeeperOfTheForbiddenWilds-1 · Keeper of the Forbidden Wilds — Spreading Wilds**

- **Rule:** (Slow) 2 Sun: Push 1 {Explorer} from target land per 2 {Sun} you have. / 1 Plant: If target land has no {Explorer}, add 1 {Wilds}. / 3 Plant: This Power has {+1Range}. / 1 Air: This Power has {+1Range}. Parameters: adds = 1 Wilds if target land has no Explorer; speed = slow.
- **Changes:** `explore_change`
- **Engine:** `spirits.json` › `KeeperOfTheForbiddenWilds` › `innatePowers[SpreadingWilds]`: "(Slow) 2 Sun: Push 1 {Explorer} from target land per 2 {Sun} you have. / 1 Plant: If target land has no {Explorer}, add 1 {Wilds}. / 3 Plant: This Power has {+1Range}. / 1 Air: This Power has {+1Range}."
- **Wiki:** `data/references/wiki/keeper-of-the-forbidden-wilds.json` (fetcher output, revid not recorded): "Push 1 Explorer from target land per 2 Sun you have. / If target land has no Explorer, add 1 Wilds. / This Power has Range +1. / This Power has Range +1." (text similarity 0.97)
- **Agreement:** `agree` — Slow: placed after this turn's Explore, so it blocks the next Explore into that land.
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-SP-KeeperOfTheForbiddenWilds-2 · Keeper of the Forbidden Wilds (SpreadingHostility aspect) — Lands Hostile to Humanity**

- **Rule:** Your {SacredSite} may also count as Badlands. (Choose for each Action.) Parameters: mechanism = Badlands; sacred_site_counts_as_badlands = choose per Action.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `KeeperOfTheForbiddenWilds` › aspects[SpreadingHostility] › `specialRules[LandsHostileToHumanity].body`: "Your {SacredSite} may also count as Badlands. (Choose for each Action.)"
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Sharp Fangs Behind the Leaves** (`SharpFangsBehindTheLeaves`, Branch & Claw): none found. Beast movement and Damage only (aspects Unconstrained, Encircle too).

**Heart of the Wildfire** (`HeartOfTheWildfire`, Promo Pack 1)

**R-SP-HeartOfTheWildfire-1 · Heart of the Wildfire — Blazing Presence**

- **Rule:** After you add or move {Presence} after Setup, in the land it goes to: / For each {Fire} showing on your {Presence} tracks, do 1 Damage. / If 2 {Fire} or more are showing on your {Presence} tracks, add 1 {Blight}. / Push all {Beast} and any number of {Dahan}. / If you add multiple {Presence} into a land at the same time, only do the above effects once. Parameters: adds_blight = when your Presence is added/moved with 2+ Fire showing; non_ravage_blight = True.
- **Changes:** `other`
- **Engine:** `spirits.json` › `HeartOfTheWildfire` › `specialRules[BlazingPresence].body`: "After you add or move {Presence} after Setup, in the land it goes to: / For each {Fire} showing on your {Presence} tracks, do 1 Damage. / If 2 {Fire} or more are showing on your {Presence} tracks, add 1 {Blight}. / Push all {Beast} and any number of {Dahan}. / If you add multiple {Presence} into a land at the same time, only do the above effects once."
- **Wiki:** `data/references/wiki/heart-of-the-wildfire.json` (fetcher output, revid not recorded): "Post-Setup, after your Presence is added/moved, in the land it goes to: * For each Simplefire showing on your Presence Tracks, do 1 Damage. * If 2 Simplefire or more are showing on your Presence Tracks, add 1 Blight. * Push all Beast and any number of Dahan. If you add multiple Presence into a land at the same time, only do the above effects once." (text similarity 0.75)
- **Agreement:** `agree` — Read manually: same rule.
- **Affects:** see note
- **Status:** unreviewed

**R-SP-HeartOfTheWildfire-2 · Heart of the Wildfire — Destructive Nature**

- **Rule:** {Blight} added due to Spirit effects (Powers, Special Rules, Scenario-based Rituals, etc) does not destroy your {Presence}. (This includes cascades.) Parameters: spirit_effect_blight_destroys_your_presence = False; includes_cascades = True.
- **Changes:** `cascade_change`
- **Engine:** `spirits.json` › `HeartOfTheWildfire` › `specialRules[DestructiveNature].body`: "{Blight} added due to Spirit effects (Powers, Special Rules, Scenario-based Rituals, etc) does not destroy your {Presence}. (This includes cascades.)"
- **Wiki:** `data/references/wiki/heart-of-the-wildfire.json` (fetcher output, revid not recorded): "Blight added due to Spirit effects (Powers, Special Rules, Scenario-based Rituals, etc) does not destroy your Presence. (This includes cascades.)" (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** cascade badge; blight runway; presence at risk
- **Status:** unreviewed

**Serpent Slumbering Beneath the Island** (`SerpentSlumberingBeneathTheIsland`, Promo Pack 1) — base panel: none found; aspect entries only.

**R-SP-SerpentSlumberingBeneathTheIsland-1 · Serpent Slumbering Beneath the Island (Locus aspect) — Locus of the Serpent's Regard**

- **Rule:** Spirits with absorbed {Presence} can use your {Presence} at {Locus} for targeting. / {Locus} doesn't count towards your Deep Slumber limit. / After uncovering the {Earth} space of your {Presence} track, Empower {Locus}. If {Locus} is empowered, it may count as {Badlands}. Parameters: mechanism = Badlands; where = empowered Locus may count as Badlands.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `SerpentSlumberingBeneathTheIsland` › aspects[Locus] › `specialRules[LocusOfTheSerpentsRegard].body`: "Spirits with absorbed {Presence} can use your {Presence} at {Locus} for targeting. / {Locus} doesn't count towards your Deep Slumber limit. / After uncovering the {Earth} space of your {Presence} track, Empower {Locus}. If {Locus} is empowered, it may count as {Badlands}."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Fractured Days Split the Sky** (`FracturedDaysSplitTheSky`, Jagged Earth)

**R-SP-FracturedDaysSplitTheSky-1 · Fractured Days Split the Sky — Slip the Flow of Time**

- **Rule:** (Fast) 1 Air+3 Moon: Target Spirit may Resolve 1 {Slow} Power now. / 2 Moon+2 Sun: Target Spirit may Reclaim 1 Power Card from their discarded or played cards. (If they Reclaim a card from play, they lose its Elements.) / 2 Air+3 Sun: Target Spirit may play a Power Card by paying its cost. Parameters: timing = level 1: target Spirit may resolve 1 Slow Power now (in the Fast Phase).
- **Changes:** `other`
- **Engine:** `spirits.json` › `FracturedDaysSplitTheSky` › `innatePowers[SlipTheFlowOfTime]`: "(Fast) 1 Air+3 Moon: Target Spirit may Resolve 1 {Slow} Power now. / 2 Moon+2 Sun: Target Spirit may Reclaim 1 Power Card from their discarded or played cards. (If they Reclaim a card from play, they lose its Elements.) / 2 Air+3 Sun: Target Spirit may play a Power Card by paying its cost."
- **Wiki:** not found in the fetched page
- **Agreement:** `engine-only` — Wiki page fetched; the fetcher parsed this innate's speed (Fast, matches) but no threshold text.
- **Affects:** see note
- **Status:** unreviewed

**R-SP-FracturedDaysSplitTheSky-2 · Fractured Days Split the Sky — Visions of a Shifting Future**

- **Rule:** (Slow) 2 Air+2 Moon+1 Sun: Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.) / 2 Air+3 Moon+2 Sun: Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup. Parameters: invader_deck = look at the top card; return it and shuffle the top 2; level 2 may put it on the bottom; speed = slow.
- **Changes:** `other`
- **Engine:** `spirits.json` › `FracturedDaysSplitTheSky` › `innatePowers[VisionsOfAShiftingFuture]`: "(Slow) 2 Air+2 Moon+1 Sun: Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.) / 2 Air+3 Moon+2 Sun: Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup."
- **Wiki:** `data/references/wiki/fractured-days-split-the-sky.json` (fetcher output, revid not recorded): "Look at the top card of either the Invader Deck or (if relevant) the Event Deck. Return it, then shuffle that deck's top 2 cards. (For the Invader Deck, differing Invader Stages may give away which card is next.) / Instead of returning-and-shuffling, you may put the card you looked at on the bottom of its deck. You may not do this for cards specially placed during Setup." (text similarity 1.00)
- **Agreement:** `agree` — Changes deck order: the companion must treat the top two cards as re-randomized and must not reveal the looked-at card unless the player does.
- **Affects:** see note
- **Status:** unreviewed

**Grinning Trickster Stirs Up Trouble** (`GrinningTricksterStirsUpTrouble`, Jagged Earth)

**R-SP-GrinningTricksterStirsUpTrouble-1 · Grinning Trickster Stirs Up Trouble — Why Don't You And Them Fight**

- **Rule:** (Fast) 3 Moon: This Power may be {Slow}. / 3 Air: Add 1 {Strife}. / 3 Fire+3 Sun: 1 Invader and 1 {Dahan} deal Damage to each other. / 3 Animal: If target land has {Beast}, 2 Damage. Otherwise, you may Gather 1 {Beast}. Parameters: timing = Moon 3: this Fast Power may be Slow; strife = Air 3: Add 1 Strife.
- **Changes:** `other`
- **Engine:** `spirits.json` › `GrinningTricksterStirsUpTrouble` › `innatePowers[WhyDontYouAndThemFight]`: "(Fast) 3 Moon: This Power may be {Slow}. / 3 Air: Add 1 {Strife}. / 3 Fire+3 Sun: 1 Invader and 1 {Dahan} deal Damage to each other. / 3 Animal: If target land has {Beast}, 2 Damage. Otherwise, you may Gather 1 {Beast}."
- **Wiki:** `data/references/wiki/grinning-trickster.json` (fetcher output, revid not recorded): "This Power may be Slow. / Add 1 Strife. / 1 Invader and 1 Dahan deal Damage to each other. / If target land has Beast, 2 Damage. Otherwise, you may Gather 1 Beast." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**Lure of the Deep Wilderness** (`LureOfTheDeepWilderness`, Jagged Earth)

**R-SP-LureOfTheDeepWilderness-1 · Lure of the Deep Wilderness — Enthrall the Foreign Explorers**

- **Rule:** For each of your {Presence} in a land, up to 2 {Explorer} do not participate in Ravage. Parameters: explorers_not_participating = up to 2 per your Presence in the land.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `LureOfTheDeepWilderness` › `specialRules[EnthrallTheForeignExplorers].body`: "For each of your {Presence} in a land, up to 2 {Explorer} do not participate in Ravage."
- **Wiki:** `data/references/wiki/lure-of-the-deep-wilderness.json` (fetcher output, revid not recorded): "For each of your Presence in a land, up to 2 Explorer do not participate in Ravage." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Many Minds Move as One** (`ManyMindsMoveAsOne`, Jagged Earth)

**R-SP-ManyMindsMoveAsOne-1 · Many Minds Move as One — Beset and Confound the Invaders**

- **Rule:** (Fast) 1 Air+2 Animal+2 Beast: 2 {Fear} and Defend 2. / 2 Air+3 Animal+3 Beast: Instead, 3 {Fear} and Defend 4. / 3 Air+4 Animal+4 Beast: Instead, 4 {Fear} and Defend 7. / 4 Air+5 Animal+5 Beast+1 Earth: Instead, 6 {Fear} and Defend 10. Parameters: mechanism = defend; levels = [2, 4, 7, 10]; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `ManyMindsMoveAsOne` › `innatePowers[BesetAndConfoundTheInvaders]`: "(Fast) 1 Air+2 Animal+2 Beast: 2 {Fear} and Defend 2. / 2 Air+3 Animal+3 Beast: Instead, 3 {Fear} and Defend 4. / 3 Air+4 Animal+4 Beast: Instead, 4 {Fear} and Defend 7. / 4 Air+5 Animal+5 Beast+1 Earth: Instead, 6 {Fear} and Defend 10."
- **Wiki:** `data/references/wiki/many-minds-move-as-one.json` (fetcher output, revid not recorded): "2 Fear and Defend 2. / Instead, 3 Fear and Defend 4. / Instead, 4 Fear and Defend 7. / Instead, 6 Fear and Defend 10." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Shifting Memory of Ages** (`ShiftingMemoryOfAges`, Jagged Earth)

**R-SP-ShiftingMemoryOfAges-1 · Shifting Memory of Ages — Learn the Invaders' Tactics**

- **Rule:** (Fast) 2 Earth: Defend 2. / 1 Air+2 Earth: Instead, Defend 3. / 3 Air+4 Earth+2 Moon: Instead, Defend 2 per card in the Invader discard pile. Parameters: mechanism = defend; levels = [2, 3, "2 per card in the Invader discard"]; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `ShiftingMemoryOfAges` › `innatePowers[LearnTheInvadersTactics]`: "(Fast) 2 Earth: Defend 2. / 1 Air+2 Earth: Instead, Defend 3. / 3 Air+4 Earth+2 Moon: Instead, Defend 2 per card in the Invader discard pile."
- **Wiki:** `data/references/wiki/shifting-memory-of-ages.json` (fetcher output, revid not recorded): "Defend 2. / Instead, Defend 3. / Instead, Defend 2 per card in the Invader discard pile." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Shroud of Silent Mist** (`ShroudOfSilentMist`, Jagged Earth)

**R-SP-ShroudOfSilentMist-1 · Shroud of Silent Mist — Slow and Silent Death**

- **Rule:** Invaders and {Dahan} in your lands don't heal Damage. / During Time Passes: 1 {Fear} (max. 5) per land of yours with Damaged Invaders. Gain 1 Energy per 3 lands of yours with Damaged Invaders. Parameters: no_heal = Invaders and Dahan in your lands keep Damage through Time Passes.
- **Changes:** `piece_health`
- **Engine:** `spirits.json` › `ShroudOfSilentMist` › `specialRules[SlowAndSilentDeath].body`: "Invaders and {Dahan} in your lands don't heal Damage. / During Time Passes: 1 {Fear} (max. 5) per land of yours with Damaged Invaders. Gain 1 Energy per 3 lands of yours with Damaged Invaders."
- **Wiki:** `data/references/wiki/shroud-of-silent-mist.json` (fetcher output, revid not recorded): "Invaders and Dahan in your lands don't heal Damage. During Time Passes: 1 Fear (max. 5) per land of yours with Damaged Invaders. Gain 1 Energy per 3 lands of yours with Damaged Invaders." (text similarity 0.99)
- **Agreement:** `agree` — Carry-over Damage changes next turn's Ravage and counterattack results.
- **Affects:** Ravage ledger (counterattack kills); Build/victory blockers
- **Status:** unreviewed

**R-SP-ShroudOfSilentMist-2 · Shroud of Silent Mist (Stranded aspect) — Stranded in the Shifting Mist**

- **Rule:** Once each {Fast} phase, Isolate one of your lands. Parameters: once_each_fast_phase = Isolate one of your lands.
- **Changes:** `isolate`
- **Engine:** `spirits.json` › `ShroudOfSilentMist` › aspects[Stranded] › `specialRules[StrandedInTheShiftingMist].body`: "Once each {Fast} phase, Isolate one of your lands."
- **Wiki:** not available (aspect page not fetched)
- **Agreement:** `engine-only`
- **Affects:** Explore eligibility (isolated land neither explores nor is a source)
- **Status:** unreviewed

**Starlight Seeks Its Form** (`StarlightSeeksItsForm`, Jagged Earth)

**R-SP-StarlightSeeksItsForm-1 · Starlight Seeks Its Form — Air Moves, Earth Endures**

- **Rule:** (Fast) 3 Air: Push up to 2 {Explorer} or 1 {Town}. / 3 Earth: Defend 5. Parameters: mechanism = defend; amount = 5; threshold = Earth 3; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `StarlightSeeksItsForm` › `innatePowers[AirMovesEarthEndures]`: "(Fast) 3 Air: Push up to 2 {Explorer} or 1 {Town}. / 3 Earth: Defend 5."
- **Wiki:** `data/references/wiki/starlight-seeks-its-form.json` (fetcher output, revid not recorded): "Push up to 2 Explorer or 1 Town. / Defend 5." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Stone's Unyielding Defiance** (`StonesUnyieldingDefiance`, Jagged Earth)

**R-SP-StonesUnyieldingDefiance-1 · Stone's Unyielding Defiance — Bestow the Endurance of Bedrock**

- **Rule:** When {Blight} is added to one of your lands, unless the {Blight} then outnumbers your {Presence}, it does not cascade or Destroy {Presence} (yours or others'). Parameters: in_your_lands = Blight does not cascade or destroy Presence unless Blight then outnumbers your Presence.
- **Changes:** `cascade_change`
- **Engine:** `spirits.json` › `StonesUnyieldingDefiance` › `specialRules[BestowTheEnduranceOfBedrock].body`: "When {Blight} is added to one of your lands, unless the {Blight} then outnumbers your {Presence}, it does not cascade or Destroy {Presence} (yours or others')."
- **Wiki:** `data/references/wiki/stone-unyielding-defiance.json` (fetcher output, revid not recorded): "When Blight is added to one of your lands, unless the Blight then outnumbers your Presence, it does not cascade or destroy Presence (yours or others')." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** cascade badge; blight runway; presence at risk
- **Status:** unreviewed

**R-SP-StonesUnyieldingDefiance-2 · Stone's Unyielding Defiance — Hold the Island Fast with a Bulwark of Will**

- **Rule:** (Fast) 2 Earth: When {Blight} is added to one of your lands, you may pay 2 Energy per {Blight} to take it from the box instead of from the Blight Card. (Handle any cascade separately.) / 4 Earth: The cost is 1 Energy instead of 2. / 6 Earth+1 Plant: When an Event or Blight Card directly Destroys {Presence} (yours or others), you may prevent any number of {Presence} from being Destroyed by paying 1 Energy each. “Directly” means “not by adding {Blight}”.) Parameters: blight_from_box = pay 2 (Earth 4: 1) Energy per Blight added to your lands; speed = fast.
- **Changes:** `other`
- **Engine:** `spirits.json` › `StonesUnyieldingDefiance` › `innatePowers[HoldTheIslandFastWithABulwarkOfWill]`: "(Fast) 2 Earth: When {Blight} is added to one of your lands, you may pay 2 Energy per {Blight} to take it from the box instead of from the Blight Card. (Handle any cascade separately.) / 4 Earth: The cost is 1 Energy instead of 2. / 6 Earth+1 Plant: When an Event or Blight Card directly Destroys {Presence} (yours or others), you may prevent any number of {Presence} from being Destroyed by paying 1 Energy each. “Directly” means “not by adding {Blight}”.)"
- **Wiki:** `data/references/wiki/stone-unyielding-defiance.json` (fetcher output, revid not recorded): "When Blight is added to one of your lands, you may pay 2 Energy per Blight to take it from the box instead of the Blight Card. (Handle any cascade separately.) / The cost is 1 Energy instead of 2. / When an Event or Blight card directly destroys Presence (yours or others'), you may prevent any number of Presence from being destroyed by paying 1 Energy each. ("Directly" means "not by adding Blight".)" (text similarity 0.99)
- **Agreement:** `agree` — Changes the Blight Card count, not where Blight lands.
- **Affects:** see note
- **Status:** unreviewed

**R-SP-StonesUnyieldingDefiance-3 · Stone's Unyielding Defiance — Let Them Break Themselves Against the Stone**

- **Rule:** (Fast) 3 Earth: After Invaders deal 1 or more Damage to target land, 2 Damage. / 5 Earth: Also deal half of the Damage Invaders did to the land (rounding down). / 7 Earth+2 Sun: Repeat this Power. Parameters: after_invaders_damage_land = 2 Damage (Earth 5: + half the land Damage); speed = fast.
- **Changes:** `other`
- **Engine:** `spirits.json` › `StonesUnyieldingDefiance` › `innatePowers[LetThemBreakThemselvesAgainstTheStone]`: "(Fast) 3 Earth: After Invaders deal 1 or more Damage to target land, 2 Damage. / 5 Earth: Also deal half of the Damage Invaders did to the land (rounding down). / 7 Earth+2 Sun: Repeat this Power."
- **Wiki:** not found in the fetched page
- **Agreement:** `engine-only` — Wiki page fetched; the fetcher parsed this innate's speed (Fast, matches) but no threshold text.
- **Affects:** see note
- **Status:** unreviewed

**Vengeance as a Burning Plague** (`VengeanceAsABurningPlague`, Jagged Earth)

**R-SP-VengeanceAsABurningPlague-1 · Vengeance as a Burning Plague — The Terror of a Slowly Unfolding Plague**

- **Rule:** When {Disease} would prevent a Build on a board with your {Presence}, you may let the Build happen (Removing no {Disease}). If you do, 1 {Fear}. Parameters: disease_prevented_build = may happen anyway (no Disease removed), +1 Fear; where = boards with your Presence.
- **Changes:** `build_change`
- **Engine:** `spirits.json` › `VengeanceAsABurningPlague` › `specialRules[TheTerrorOfASlowlyUnfoldingPlague].body`: "When {Disease} would prevent a Build on a board with your {Presence}, you may let the Build happen (Removing no {Disease}). If you do, 1 {Fear}."
- **Wiki:** `data/references/wiki/vengeance-burning-plague.json` (fetcher output, revid not recorded): "When Disease would prevent a Build on a board with your Presence, you may let the Build happen (removing no Disease). If you do, 1 Fear." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-SP-VengeanceAsABurningPlague-2 · Vengeance as a Burning Plague — Lingering Pestilence**

- **Rule:** When your {Presence} is Destroyed by anything except a Spirit Action, add 1 {Disease} where each {DestroyedPresenceNew} was. Parameters: adds_disease = where your Presence is Destroyed by anything except a Spirit Action (e.g. Ravage Blight).
- **Changes:** `build_change`
- **Engine:** `spirits.json` › `VengeanceAsABurningPlague` › `specialRules[LingeringPestilence].body`: "When your {Presence} is Destroyed by anything except a Spirit Action, add 1 {Disease} where each {DestroyedPresenceNew} was."
- **Wiki:** `data/references/wiki/vengeance-burning-plague.json` (fetcher output, revid not recorded): "When your Presence is destroyed by anything except a Spirit action, add 1 Disease where each destroyed Presence was." (text similarity 0.98)
- **Agreement:** `agree` — Disease blocks the next Build there (Wiki Disease page).
- **Affects:** Build forecast
- **Status:** unreviewed

**Volcano Looming High** (`VolcanoLoomingHigh`, Jagged Earth)

**R-SP-VolcanoLoomingHigh-1 · Volcano Looming High — Collapse in a Blast of Lava and Steam**

- **Rule:** When your {Presence} is Destroyed, in that land, deal 1 Damage per Destroyed {Presence} to both Invaders and to {Dahan}. Parameters: on_presence_destroyed = 1 Damage per destroyed Presence to Invaders and to Dahan in that land.
- **Changes:** `other`
- **Engine:** `spirits.json` › `VolcanoLoomingHigh` › `specialRules[CollapseInABlastOfLavaAndSteam].body`: "When your {Presence} is Destroyed, in that land, deal 1 Damage per Destroyed {Presence} to both Invaders and to {Dahan}."
- **Wiki:** `data/references/wiki/volcano-looming-high.json` (fetcher output, revid not recorded): "When your Presence is destroyed, in that land, deal 1 Damage per destroyed Presence to both Invaders and to Dahan." (text similarity 1.00)
- **Agreement:** `agree` — Triggers when Ravage Blight destroys Volcano's Presence.
- **Affects:** see note
- **Status:** unreviewed

**Downpour Drenches the World** (`DownpourDrenchesTheWorld`, Promo Pack 2)

**R-SP-DownpourDrenchesTheWorld-1 · Downpour Drenches the World — Drench the Landscape**

- **Rule:** Spirit Actions and Special Rules treat your {SacredSite} as Wetlands in addition to the printed terrain. Parameters: sacred_site_terrain = also Wetland, for Spirit Actions and Special Rules.
- **Changes:** `other`
- **Engine:** `spirits.json` › `DownpourDrenchesTheWorld` › `specialRules[DrenchTheLandscape].body`: "Spirit Actions and Special Rules treat your {SacredSite} as Wetlands in addition to the printed terrain."
- **Wiki:** `data/references/wiki/downpour-drenches-the-world.json` (fetcher output, revid not recorded): "Spirit Actions and Special Rules treat your Sacred Site as Wetlands in addition to the printed terrain." (text similarity 1.00)
- **Agreement:** `agree` — Text names Spirit Actions and Special Rules only; whether an Invader Card counts is an open question.
- **Resolution:** No: Invader Cards drive Invader Actions, not Spirit Actions, so a Wetland Invader Card does not match Downpour's Sacred Sites (NQ-K; card text, engine and Wiki agree).
- **Affects:** see note
- **Status:** sourced (2026-10-19, NQ-K; card text)

**R-SP-DownpourDrenchesTheWorld-2 · Downpour Drenches the World — Rain and Mud Suppress Conflict**

- **Rule:** (Fast) 1 Air+3 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. (Total, in its land.) / 1 Earth+5 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. / 3 Air+2 Earth+9 Water: 2 {Fear}. In your lands, Invaders and {Dahan} have -1 Health (min. 1). Parameters: mechanism = defend; amount = 1 per your Presence (total, per land); speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `DownpourDrenchesTheWorld` › `innatePowers[RainAndMudSuppressConflict]`: "(Fast) 1 Air+3 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. (Total, in its land.) / 1 Earth+5 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. / 3 Air+2 Earth+9 Water: 2 {Fear}. In your lands, Invaders and {Dahan} have -1 Health (min. 1)."
- **Wiki:** `data/references/wiki/downpour-drenches-the-world.json` (fetcher output, revid not recorded): "Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.) / Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. / 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1)." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SP-DownpourDrenchesTheWorld-3 · Downpour Drenches the World — Rain and Mud Suppress Conflict**

- **Rule:** (Fast) 1 Air+3 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. (Total, in its land.) / 1 Earth+5 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. / 3 Air+2 Earth+9 Water: 2 {Fear}. In your lands, Invaders and {Dahan} have -1 Health (min. 1). Parameters: counterattack_damage = -1 per your Presence (total, per land); level3 = Invaders and Dahan -1 Health in your lands.
- **Changes:** `dahan_change`
- **Engine:** `spirits.json` › `DownpourDrenchesTheWorld` › `innatePowers[RainAndMudSuppressConflict]`: "(Fast) 1 Air+3 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. (Total, in its land.) / 1 Earth+5 Water: Each of your {Presence} grants Defend 1 and lowers {Dahan} counterattack Damage by 1. / 3 Air+2 Earth+9 Water: 2 {Fear}. In your lands, Invaders and {Dahan} have -1 Health (min. 1)."
- **Wiki:** `data/references/wiki/downpour-drenches-the-world.json` (fetcher output, revid not recorded): "Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. (Total, in its land.) / Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. / 2 Fear. In your lands, Invaders and Dahan have -1 Health (min 1)." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (Dahan damage, counterattack)
- **Status:** unreviewed

**R-SP-DownpourDrenchesTheWorld-4 · Downpour Drenches the World — Water Nourishes Life's Growth**

- **Rule:** (Fast) 2 Plant+3 Water: Gain 1 Energy. You may Remove 1 {Blight} by Removing 1 of your {Presence} (from target land). / 1 Earth+2 Plant+5 Water: Gain +1 Energy. Gather up to 1 {Dahan}. / 2 Earth+3 Plant+7 Water: When {Blight} would be added to target land, instead leave it on the card. Parameters: level3 = Blight that would be added to target land stays on the Blight Card; speed = fast.
- **Changes:** `ravage_blight_change`
- **Engine:** `spirits.json` › `DownpourDrenchesTheWorld` › `innatePowers[WaterNourishesLifesGrowth]`: "(Fast) 2 Plant+3 Water: Gain 1 Energy. You may Remove 1 {Blight} by Removing 1 of your {Presence} (from target land). / 1 Earth+2 Plant+5 Water: Gain +1 Energy. Gather up to 1 {Dahan}. / 2 Earth+3 Plant+7 Water: When {Blight} would be added to target land, instead leave it on the card."
- **Wiki:** `data/references/wiki/downpour-drenches-the-world.json` (fetcher output, revid not recorded): "Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (From target land). / Gain +1 Energy. Gather up to 1 Dahan. / When Blight would be added to target land, instead leave it on the card." (text similarity 0.99)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (blight added); blight runway
- **Status:** unreviewed

**Finder of Paths Unseen** (`FinderOfPathsUnseen`, Promo Pack 2)

**R-SP-FinderOfPathsUnseen-1 · Finder of Paths Unseen — Open the Ways**

- **Rule:** You may make up to two of your lands adjacent at a time. You may change which lands are adjacent once between Actions. Parameters: pairs = up to two of your lands adjacent at a time; change = once between Actions.
- **Changes:** `adjacency_change`
- **Engine:** `spirits.json` › `FinderOfPathsUnseen` › `specialRules[OpenTheWays].body`: "You may make up to two of your lands adjacent at a time. You may change which lands are adjacent once between Actions."
- **Wiki:** `data/references/wiki/finder-of-paths-unseen.json` (fetcher output, revid not recorded): "You may make up to two of your lands adjacent at a time. You may change which lands are adjacent once between Actions." (text similarity 1.00)
- **Agreement:** `agree` — Changes Explore sources (adjacency to Town/City/Ocean).
- **Affects:** Explore sources; cascade targets; Range
- **Status:** unreviewed

**R-SP-FinderOfPathsUnseen-2 · Finder of Paths Unseen — Close the Ways**

- **Rule:** (Fast) 1 Air+2 Water: Isolate target land. Parameters: speed = fast; separate_powers = ["CloseTheWaysWater", "CloseTheWaysEarth", "CloseTheWaysPlant"].
- **Changes:** `isolate`
- **Engine:** `spirits.json` › `FinderOfPathsUnseen` › `innatePowers[CloseTheWaysWater]`: "(Fast) 1 Air+2 Water: Isolate target land."
- **Wiki:** not found in the fetched page
- **Agreement:** `engine-only` — Three separate innates, each 'Isolate target land'. Wiki page fetched; the fetcher parsed this innate's speed (Fast, matches) but no threshold text.
- **Affects:** Explore eligibility (isolated land neither explores nor is a source)
- **Status:** unreviewed

**Devouring Teeth Lurk Underfoot** (`DevouringTeethLurkUnderfoot`, Horizons): none found. Territorial Aggression adds Power Damage only.

**Eyes Watch From the Trees** (`EyesWatchFromTheTrees`, Horizons)

**R-SP-EyesWatchFromTheTrees-1 · Eyes Watch From the Trees — Mischief and Sabotage**

- **Rule:** (Fast) 1 Moon+2 Plant: 1 {Fear} and Defend 2. / 2 Moon+3 Plant: Instead, 1 {Fear} and Defend 4. / 2 Air+2 Moon+4 Plant: Instead, 3 {Fear} and Defend 6. / 3 Air+3 Moon+5 Plant: Instead, 5 {Fear} and Defend 12. Parameters: mechanism = defend; levels = [2, 4, 6, 12]; speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `EyesWatchFromTheTrees` › `innatePowers[MischiefAndSabotage]`: "(Fast) 1 Moon+2 Plant: 1 {Fear} and Defend 2. / 2 Moon+3 Plant: Instead, 1 {Fear} and Defend 4. / 2 Air+2 Moon+4 Plant: Instead, 3 {Fear} and Defend 6. / 3 Air+3 Moon+5 Plant: Instead, 5 {Fear} and Defend 12."
- **Wiki:** `data/references/wiki/eyes-watch-from-the-trees.json` (fetcher output, revid not recorded): "1 Fear and Defend 2. / Instead, 1 Fear and Defend 4. / Instead, 3 Fear and Defend 6. / Instead, 5 Fear and Defend 12." (text similarity 1.00)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Fathomless Mud of the Swamp** (`FathomlessMudOfTheSwamp`, Horizons)

**R-SP-FathomlessMudOfTheSwamp-1 · Fathomless Mud of the Swamp — Offer No Firm Foundations**

- **Rule:** At your {SacredSite}, Build actions add {Explorer} instead of {Town}/{City}. Parameters: where = your Sacred Site; build_adds = Explorer instead of Town/City.
- **Changes:** `build_change`
- **Engine:** `spirits.json` › `FathomlessMudOfTheSwamp` › `specialRules[OfferNoFirmFoundations].body`: "At your {SacredSite}, Build actions add {Explorer} instead of {Town}/{City}."
- **Wiki:** `data/references/wiki/fathomless-mud-of-the-swamp.json` (fetcher output, revid not recorded): "At your Sacred Site, Build actions add Explorer instead of Town/City." (text similarity 0.98)
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**Rising Heat of Stone and Sand** (`RisingHeatOfStoneAndSand`, Horizons)

**R-SP-RisingHeatOfStoneAndSand-1 · Rising Heat of Stone and Sand — Blistering Heat**

- **Rule:** At your {SacredSite}, Invaders have -1 Health (min. 1). Parameters: invader_health = -1; min = 1; where = your Sacred Site.
- **Changes:** `piece_health`
- **Engine:** `spirits.json` › `RisingHeatOfStoneAndSand` › `specialRules[BlisteringHeat].body`: "At your {SacredSite}, Invaders have -1 Health (min. 1)."
- **Wiki:** `data/references/wiki/rising-heat-of-stone-and-sand.json` (fetcher output, revid not recorded): "At your Sacred Site, Invaders have -1 Health (min. 1)." (text similarity 0.99)
- **Agreement:** `agree` — Changes counterattack kills, not Invader Ravage damage.
- **Affects:** Ravage ledger (counterattack kills); Build/victory blockers
- **Status:** unreviewed

**Sun-Bright Whirlwind** (`SunBrightWhirlwind`, Horizons): none found. Pushes on Growth; Damage innate.

**Breath of Darkness Down Your Spine** (`BreathOfDarknessDownYourSpine`, Nature Incarnate)

**R-SP-BreathOfDarknessDownYourSpine-1 · Breath of Darkness Down Your Spine — Terror Stalks the Land**

- **Rule:** You have an Incarna ({BreathOfDarknessDownYourSpine}). It may count as {Beast}. Empower {BreathOfDarknessDownYourSpine} after uncovering {EmpowerIncarna}. You may Abduct 1 {Explorer}/{Town} at empowered {BreathOfDarknessDownYourSpine} each {Fast} Phase. / To Abduct a piece, Move it to The Endless Dark ({TheEndlessDark}). When pieces Escape, Move them to non-Ocean lands with your {Presence}/{BreathOfDarknessDownYourSpine}; if they have no legal land to move to, you lose. / When your Powers would directly damage or directly destroy the only Invader in a land, instead Abduct it. (Check if it is alone at the start of the damage/destroy instruction. {TheEndlessDark} is not a land.) Parameters: abduct = 1 Explorer/Town at empowered Incarna each Fast Phase; your Powers that would damage/destroy a lone Invader abduct it instead; effect = the piece leaves its land (no longer a Build/Ravage participant or Explore source) until it Escapes.
- **Changes:** `other`
- **Engine:** `spirits.json` › `BreathOfDarknessDownYourSpine` › `specialRules[TerrorStalksTheLand].body`: "You have an Incarna ({BreathOfDarknessDownYourSpine}). It may count as {Beast}. Empower {BreathOfDarknessDownYourSpine} after uncovering {EmpowerIncarna}. You may Abduct 1 {Explorer}/{Town} at empowered {BreathOfDarknessDownYourSpine} each {Fast} Phase. / To Abduct a piece, Move it to The Endless Dark ({TheEndlessDark}). When pieces Escape, Move them to non-Ocean lands with your {Presence}/{BreathOfDarknessDownYourSpine}; if they have no legal land to move to, you lose. / When your Powers would directly damage or directly destroy the only Invader in a land, instead Abduct it. (Check if it is alone at the start of the damage/destroy instruction. {TheEndlessDark} is not a land.)"
- **Wiki:** `data/references/wiki/breath-of-darkness.json` (fetcher output, revid not recorded): "</br>You have an Incarna ({{incarna|breath}}). Empower {{incarna|breath}} after uncovering {{empower}}. You may Abduct 1 Explorer/Town at empowered {{incarna|breath}} each Fast phase. To Abduct a piece, Move it to [[The Endless Dark]] ({{endlessdark}}). When pieces Escape, Move them to non-Ocean lands with your Presence/{{incarna|breath}}; if they have no legal land to move to, you lose. When your Powers would directly damage or directly destroy the only Invader in a land, instead Abduct it. (Check if it is alone at the start of the damage/destroy instruction. {{endlessdark}} is not a land.)" (text similarity 0.06)
- **Agreement:** `conflict` — Engine adds 'It may count as {Beast}.'; the Wiki copy (fetched 2026-04-19) has no such clause. The Abduct rule itself matches.
- **Affects:** see note
- **Resolution:** The printed panel governs: the engine's "It may count as {Beast}" has no errata behind it (the only FAQ errata is the Escape wording, already in the engine text), so the companion does not treat the Incarna as a Beast; Steam difference (NQ-N).
- **Status:** reviewed by Brett (2026-10-19, NQ-N; printed card governs unless explicit errata (Brett's rule, 2026-10-19))

**R-SP-BreathOfDarknessDownYourSpine-2 · Breath of Darkness Down Your Spine — A Space of Strange and Infinite Darkness**

- **Rule:** The Endless Dark ({TheEndlessDark}) is on the island, but isn’t a land and isn’t on any board. / • This cannot be a {SacredSite}. / • Spirits can’t target this even if they have {Presence} here. / • Adding {Blight} here doesn't destroy {Presence}. / • Special Rules that apply to lands do nothing here. / • Invaders can’t take Actions here. / • Pieces here are on the island when checking for Victory/Defeat. Parameters: where = The Endless Dark (not a land); rule = Invaders can't take Actions here.
- **Changes:** `skip_invader_action`
- **Engine:** `spirits.json` › `BreathOfDarknessDownYourSpine` › `specialRules[ASpaceOfStrangeAndInfiniteDarkness].body`: "The Endless Dark ({TheEndlessDark}) is on the island, but isn’t a land and isn’t on any board. / • This cannot be a {SacredSite}. / • Spirits can’t target this even if they have {Presence} here. / • Adding {Blight} here doesn't destroy {Presence}. / • Special Rules that apply to lands do nothing here. / • Invaders can’t take Actions here. / • Pieces here are on the island when checking for Victory/Defeat."
- **Wiki:** not found in the fetched page
- **Agreement:** `engine-only`
- **Affects:** invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage)
- **Status:** unreviewed

**Dances Up Earthquakes** (`DancesUpEarthquakes`, Nature Incarnate)

**R-SP-DancesUpEarthquakes-1 · Dances Up Earthquakes — Land Creaks with Tension**

- **Rule:** (Fast) 1 Earth: If you have at least 1 {Impending}, Add 1 {Quake} in one of your lands. / 1 Earth+1 Moon: In one of your lands, Defend 1 per {Impending} (max. 3). / 2 Earth+1 Moon: If you have at least 3 {Impending}, Add 1 {Quake} in one of your lands. / 3 Earth+2 Moon: In one of your lands, Defend 1 per {Impending} (max. 3). Parameters: mechanism = defend; amount = 1 per Impending (max 3); speed = fast.
- **Changes:** `ravage_damage_change`
- **Engine:** `spirits.json` › `DancesUpEarthquakes` › `innatePowers[LandCreaksWithTension]`: "(Fast) 1 Earth: If you have at least 1 {Impending}, Add 1 {Quake} in one of your lands. / 1 Earth+1 Moon: In one of your lands, Defend 1 per {Impending} (max. 3). / 2 Earth+1 Moon: If you have at least 3 {Impending}, Add 1 {Quake} in one of your lands. / 3 Earth+2 Moon: In one of your lands, Defend 1 per {Impending} (max. 3)."
- **Wiki:** `data/references/wiki/dances-up-earthquakes.json` (fetcher output, revid not recorded): "If you have at least 1 {{impendingcard}}, Add 1 {{quake}} in one of your lands. / In one of your lands, Defend 1 per {{impendingcard}} (max. 3). / If you have at least 3 {{impendingcard}}, Add 1 {{quake}} in one of your lands. / In one of your lands, Defend 1 per {{impendingcard}} (max. 3)." (text similarity 0.97)
- **Agreement:** `agree` — Read manually: same text (Wiki icon templates lower the similarity).
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**Ember-Eyed Behemoth** (`EmberEyedBehemoth`, Nature Incarnate): none found. Incarna movement and Damage only.

**Towering Roots of the Jungle** (`ToweringRootsOfTheJungle`, Nature Incarnate)

**R-SP-ToweringRootsOfTheJungle-1 · Towering Roots of the Jungle — Heart-Tree Guards the Land**

- **Rule:** You have an Incarna ({ToweringRootsOfTheJungle}). / • Your Powers get {+1Range} if {ToweringRootsOfTheJungle} is in the origin land. / • Invaders/{Dahan}/{Beast} can't be damaged or destroyed at {ToweringRootsOfTheJungle}. / • Empower {ToweringRootsOfTheJungle} the first time it's in a land with 3 or more {Vitality}. / • Skip all Build Actions at empowered {ToweringRootsOfTheJungle}. Parameters: action = build; where = empowered Incarna; also = Invaders/Dahan/Beast can't be damaged or destroyed at the Incarna.
- **Changes:** `skip_invader_action`
- **Engine:** `spirits.json` › `ToweringRootsOfTheJungle` › `specialRules[HeartTreeGuardsTheLand].body`: "You have an Incarna ({ToweringRootsOfTheJungle}). / • Your Powers get {+1Range} if {ToweringRootsOfTheJungle} is in the origin land. / • Invaders/{Dahan}/{Beast} can't be damaged or destroyed at {ToweringRootsOfTheJungle}. / • Empower {ToweringRootsOfTheJungle} the first time it's in a land with 3 or more {Vitality}. / • Skip all Build Actions at empowered {ToweringRootsOfTheJungle}."
- **Wiki:** `data/references/wiki/towering-roots-of-the-jungle.json` (fetcher output, revid not recorded): "</br>You have an Incarna ({{incarna|roots}}). * Your Powers get Range +1 if {{incarna|roots}} is in the origin land. * Invaders/Dahan/Beast can't be damaged or destroyed at {{incarna|roots}}. * Empower {{incarna|roots}} the first time it's in a land with 3 or more {{vitality}}. * Skip all Build Actions at empowered {{incarna|roots}}." (text similarity 0.21)
- **Agreement:** `agree` — Read manually: same text (Wiki icon templates lower the similarity).
- **Affects:** invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage)
- **Status:** unreviewed

**R-SP-ToweringRootsOfTheJungle-2 · Towering Roots of the Jungle — Enduring Vitality**

- **Rule:** Some of your Actions Add Vitality Tokens ({Vitality}). / (Each {Vitality} in a land with no {Blight} prevents 1 {Blight} from being added, then is Removed.) Parameters: vitality = each Vitality in a land with no Blight prevents 1 Blight, then is Removed.
- **Changes:** `ravage_blight_change`
- **Engine:** `spirits.json` › `ToweringRootsOfTheJungle` › `specialRules[EnduringVitality].body`: "Some of your Actions Add Vitality Tokens ({Vitality}). / (Each {Vitality} in a land with no {Blight} prevents 1 {Blight} from being added, then is Removed.)"
- **Wiki:** `data/references/wiki/towering-roots-of-the-jungle.json` (fetcher output, revid not recorded): "</br>Some of your Actions Add Vitality Tokens ({{vitality}}). (Each {{vitality}} in a land with no Blight prevents 1 Blight from being added, then is Removed.)" (text similarity 0.99)
- **Agreement:** `agree`
- **Affects:** Ravage ledger (blight added); blight runway
- **Status:** unreviewed

**Wandering Voice Keens Delirium** (`WanderingVoiceKeensDelirium`, Nature Incarnate)

**R-SP-WanderingVoiceKeensDelirium-1 · Wandering Voice Keens Delirium — A Clarion Voice Given Form**

- **Rule:** You have an Incarna ({WanderingVoiceKeensDelirium}). It may count as {SacredSite}. If {WanderingVoiceKeensDelirium} is empowered, whichever land contains {WanderingVoiceKeensDelirium} is Isolated. Parameters: where = land with empowered Incarna; always_on = True.
- **Changes:** `isolate`
- **Engine:** `spirits.json` › `WanderingVoiceKeensDelirium` › `specialRules[AClarionVoiceGivenForm].body`: "You have an Incarna ({WanderingVoiceKeensDelirium}). It may count as {SacredSite}. If {WanderingVoiceKeensDelirium} is empowered, whichever land contains {WanderingVoiceKeensDelirium} is Isolated."
- **Wiki:** `data/references/wiki/wandering-voice.json` (fetcher output, revid not recorded): "</br>You have an Incarna ({{incarna|voice}}). If Empowered, it Isolates its land." (text similarity 0.42)
- **Agreement:** `agree` — Isolate clause matches. Engine adds 'It may count as {SacredSite}.'; the Wiki copy has no such clause.
- **Affects:** Explore eligibility (isolated land neither explores nor is a source)
- **Resolution:** The printed panel governs: the engine's "It may count as {SacredSite}" has no errata behind it, so the companion does not treat the Incarna as a Sacred Site; Steam difference (NQ-O).
- **Status:** reviewed by Brett (2026-10-19, NQ-O; printed card governs unless explicit errata (Brett's rule, 2026-10-19))

**R-SP-WanderingVoiceKeensDelirium-2 · Wandering Voice Keens Delirium — Spread Tumult and Delusion**

- **Rule:** When your Actions add/move {WanderingVoiceKeensDelirium} to a land, Add 1 {Strife} there. / In lands with or adjacent to {WanderingVoiceKeensDelirium}: if {Strife} is present, {Dahan} do not participate in Ravage. (They do not take Damage or counterattack. Isolate has no effect on {WanderingVoiceKeensDelirium} and {Dahan} being adjacent.) Parameters: dahan_do_not_participate_in_ravage = lands with or adjacent to the Incarna where Strife is present.
- **Changes:** `dahan_change`
- **Engine:** `spirits.json` › `WanderingVoiceKeensDelirium` › `specialRules[SpreadTumultAndDelusion].body`: "When your Actions add/move {WanderingVoiceKeensDelirium} to a land, Add 1 {Strife} there. / In lands with or adjacent to {WanderingVoiceKeensDelirium}: if {Strife} is present, {Dahan} do not participate in Ravage. (They do not take Damage or counterattack. Isolate has no effect on {WanderingVoiceKeensDelirium} and {Dahan} being adjacent.)"
- **Wiki:** `data/references/wiki/wandering-voice.json` (fetcher output, revid not recorded): "</br>When your Actions add/move {{incarna|voice}} to a land with Invaders, Add 1 Strife in the destination land. In lands with or adjacent to {{incarna|voice}}: if Strife is present, Dahan do not participate in Ravage. (They do not take Damage or counterattack. Isolate has no effect on {{incarna|voice}} and Dahan being adjacent.) SENSELESS ROAMING</br>When your Actions add Strife to an Explorer/Town, you may Push it." (text similarity 0.09)
- **Agreement:** `agree` — Engine: add Strife 'to a land'; Wiki: 'to a land with Invaders'. Dahan-participation clause matches word for word; the Strife clause differs (see note).
- **Resolution:** No practical difference: Strife can only be added to an Invader ("If there are no Invaders, you cannot add Strife", JE rulebook), so "to a land" and "to a land with Invaders" play the same (NQ-O, Strife part).
- **Affects:** Ravage ledger (Dahan damage, counterattack)
- **Status:** sourced (2026-10-19, NQ-O Strife part; JE rulebook "Strife")

Spirits scanned: 34 (plus 24 aspects). Spirits with at least one pipeline entry: 30 (3 of them through aspects only). Spirits with none found: 4 (Sharp Fangs Behind the Leaves, Devouring Teeth Lurk Underfoot, Sun-Bright Whirlwind, Ember-Eyed Behemoth).

### 5.2 Scenarios (all 13 in `scenarios.json`)

Wiki evidence: archived scenario pages (revision ids below). Engine has no Destiny Unfolds or Surges of Colonization: they are not in Steam yet (Q22, Brett 2026-10-19; both are in the Nature Incarnate rulebook).


**Blitz** (`Blitz`, Base game, engine difficulty 0)

**R-SC-Blitz-1 · Blitz — All Powers are**

- **Rule:** All Powers are {Fast}. Parameters: timing = all Powers are Fast; already-Fast Powers cost 1 less.
- **Changes:** `other`
- **Engine:** `scenarios.json` › `Blitz` › `backBodySections[Rules Changes]`: "All Powers are {Fast}."
- **Wiki:** Blitz@3296: "All powers are {Fasticon.png}. Powers which were already {Fasticon.png} (or made {Fasticon.png}) cost 1 less Energy. (For Innate Powers and Power Cards costing 0, instead gain 1 Energy.)"
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-SC-Blitz-2 · Blitz — When Exploring, once per board**

- **Rule:** When Exploring, once per board, add an additional {Explorer}. Parameters: plus_explorer = 1; limit = once per board.
- **Changes:** `explore_change`
- **Engine:** `scenarios.json` › `Blitz` › `backBodySections[Rules Changes]`: "When Exploring, once per board, add an additional {Explorer}."
- **Wiki:** Blitz@3296: "When Exploring, once per board, add an additional {Explorer}."
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-SC-Blitz-3 · Blitz — additional set of actions**

- **Rule:** The Invaders get an additional set of actions at the end of Setup. (So they will Explore, then Build and Explore.) Parameters: action = build+explore; slot = setup; when = end of Setup: Explore, then Build and Explore.
- **Changes:** `extra_invader_action`
- **Engine:** `scenarios.json` › `Blitz` › `backBodySections[Setup Changes]`: "The Invaders get an additional set of actions at the end of Setup. (So they will Explore, then Build and Explore.)"
- **Wiki:** Blitz@3296: "The invaders get an additional set of actions at the end of Setup. (So they will Explore, then Build and Explore.)"
- **Agreement:** `agree` — So the setup card is in Ravage on Turn 1 (inference from the card flow in Sequence of Play §4; open question).
- **Resolution:** Yes, the setup card Ravages on Turn 1: a full set of Invader actions ends by advancing the cards, so the first card moves Build → Ravage (NQ-J; inference from the card text, to verify in a Steam Blitz game). FAQ (Blitz): the setup Explores/Builds are not the Build/Explore Steps, so adversary effects tied to those steps don't apply during the extra set.
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** sourced (2026-10-19, NQ-J; Blitz card text and FAQ "Blitz"; inference to verify in Steam)

**R-SC-Blitz-4 · Blitz — Put an additional**

- **Rule:** Put an additional {Blight} per board on the Blight Card. Parameters: blight_card = +1 Blight per board (engine) / per player (Wiki; Wiki note: per board with an extra board).
- **Changes:** `other`
- **Engine:** `scenarios.json` › `Blitz` › `backBodySections[Setup Changes]`: "Put an additional {Blight} per board on the Blight Card."
- **Wiki:** Blitz@3296: "Put an additional {Blight} per player on the Blight Card.[2]"
- **Agreement:** `conflict` — Per board vs per player.
- **Resolution:** Per board, both lines. FAQ (Blitz): Setup "an additional Blight per player" should be "per board"; Rule Changes "1 less Blight per player" should be "per board" (NQ-D). Without an extra board the two are equal.
- **Affects:** see note
- **Status:** sourced (2026-10-19, NQ-D; FAQ "Blitz")

**Dahan Insurrection** (`DahanInsurrection`, Base game, engine difficulty 4)

**R-SC-DahanInsurrection-1 · Dahan Insurrection — Military Response**

- **Rule:** Military Response: After an action destroys 1 or more {Town}/{City}, add 1 {Explorer} per {Town} destroyed and 1 {Town} per {City} destroyed. These new Invaders are added to the land with {Dahan} closest to the destroyed {Town}/{City}. (Add the new Invaders after fully resolving the Power or effect which destroyed the old one.) Parameters: after_town_city_destroyed = add 1 Explorer per Town / 1 Town per City destroyed, in the land with Dahan closest to it.
- **Changes:** `other`
- **Engine:** `scenarios.json` › `DahanInsurrection` › `backBodySections[Rules Changes]`: "Military Response: After an action destroys 1 or more {Town}/{City}, add 1 {Explorer} per {Town} destroyed and 1 {Town} per {City} destroyed. These new Invaders are added to the land with {Dahan} closest to the destroyed {Town}/{City}. (Add the new Invaders after fully resolving the Power or effect which destroyed the old one.)"
- **Wiki:** Dahan_Insurrection@4718: "Military Response: Whenever a {City} is destroyed, add 1 {Town} to the nearest land with {Dahan}. Whenever a {Town} is destroyed, add 1 {Explorer} to the nearest land with {Dahan}. (Add the new invaders after fully resolving the Power or effect which destroyed the old one).[2]"
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**R-SC-DahanInsurrection-2 · Dahan Insurrection — Constant Raiding**

- **Rule:** Constant Raiding: After an action moves 1 or more {Dahan}, each {Dahan} that moved does 1 Damage in their land. (Damage happens after fully resolving the Power or other effect which moved them. If a Power moves a {Dahan} multiple times, it only does damage in the land where it ends up.) Parameters: moved_dahan = each Dahan moved by an action does 1 Damage in its land.
- **Changes:** `dahan_change`
- **Engine:** `scenarios.json` › `DahanInsurrection` › `backBodySections[Rules Changes]`: "Constant Raiding: After an action moves 1 or more {Dahan}, each {Dahan} that moved does 1 Damage in their land. (Damage happens after fully resolving the Power or other effect which moved them. If a Power moves a {Dahan} multiple times, it only does damage in the land where it ends up.)"
- **Wiki:** Dahan_Insurrection@4718: "Constant Raiding: After an Action moves 1 or more {Dahan}, each {Dahan} that moved deals 1 Damage in their land. (Damage happens after fully resolving the Power or other effect which moved them. If a Power moves a {Dahan} multiple times, it only does damage in the land where it ends up.)[1]"
- **Agreement:** `agree`
- **Affects:** Ravage ledger (Dahan damage, counterattack)
- **Status:** unreviewed

**R-SC-DahanInsurrection-3 · Dahan Insurrection — fewer than 2**

- **Rule:** There are fewer than 2 {Dahan} per player left alive. Parameters: dahan_per_player_min = 2.
- **Changes:** `loss_condition`
- **Engine:** `scenarios.json` › `DahanInsurrection` › `backBodySections[Additional Loss Condition]`: "There are fewer than 2 {Dahan} per player left alive."
- **Wiki:** Dahan_Insurrection@4718: "There are fewer than 2 {Dahan} per player left alive."
- **Agreement:** `agree`
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**Guard the Isle's Heart** (`GuardTheIslesHeart`, Base game, engine difficulty 0)

**R-SC-GuardTheIslesHeart-1 · Guard the Isle's Heart — Anytime after Turn 1**

- **Rule:** Anytime after Turn 1: There is a {Town}/{City} in an Inner Land. Parameters: town_city_in_inner_land = after Turn 1; inner_land_1_2_players = land #7 on each board.
- **Changes:** `loss_condition`
- **Engine:** `scenarios.json` › `GuardTheIslesHeart` › `backBodySections[Rules Changes]`: "Anytime after Turn 1: There is a {Town}/{City} in an Inner Land."
- **Wiki:** Guard_the_Isle's_Heart@3282: "Anytime after Turn 1: There is a {Town} / {City} in an Inner Land."
- **Agreement:** `agree`
- **Affects:** loss clocks (race chart)
- **Status:** unreviewed

**R-SC-GuardTheIslesHeart-2 · Guard the Isle's Heart — Remove all**

- **Rule:** • Remove all {Town}. Parameters: remove_all_towns = True; inner_land = 1 Explorer + 1 Presence; removed_events = ["BureaucratsAdjustFunding", "InvestedAristocracy"].
- **Changes:** `setup_pieces`
- **Engine:** `scenarios.json` › `GuardTheIslesHeart` › `backBodySections[Setup Changes]`: "• Remove all {Town}."
- **Wiki:** Guard_the_Isle's_Heart@3282: "- Remove all {Town}. - In each Inner Land (see Rules Changes) add 1 {Explorer} and 1 {Presence} from the player starting on that board. (If this is illegal, as for Ocean's Hungry Grasp, add the {Presence} in the nearest legal land on that board.) - Each Spirit starts with 2 additional Power Cards, by drawing a single card from each of the 2 Power Decks."
- **Agreement:** `agree` — Removed events from engine field removeEventCardsBeforeRandomizing; not on the Wiki page.
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**Rituals of Terror** (`RitualsOfTerror`, Base game, engine difficulty 3)

**R-SC-RitualsOfTerror-1 · Rituals of Terror — Do not resolve Fear Cards**

- **Rule:** Do not resolve Fear Cards during the Invader Phase. Parameters: fear_cards_resolve = only when the Ritual is performed, not in the Invader Phase; terror_level = earned by Ritual, not Fear.
- **Changes:** `fear_deck`
- **Engine:** `scenarios.json` › `RitualsOfTerror` › `backBodySections[Rules Changes]`: "Do not resolve Fear Cards during the Invader Phase."
- **Wiki:** Rituals_of_Terror@3285: "Do not resolve earned Fear Cards during the Invader Phase (If you have trouble remembering this, you can put earned Fear cards on this card instead.)"
- **Agreement:** `agree`
- **Affects:** fear runway; Terror Level timing
- **Status:** unreviewed

**Powers Long Forgotten** (`PowersLongForgotten`, Branch & Claw, engine difficulty 1)

**R-SC-PowersLongForgotten-1 · Powers Long Forgotten — TwistedBronze**

- **Rule:** A loop of twisted bronze set with deep red stones: Invader benefit: For purposes of Explore and Build, Invaders in any Sands are considered to be in all Sands. Spirit benefit: Spirits may shift their {Presence} from a Sands to another at any time. Parameters: invader_benefit = for Explore and Build, Invaders in any Sands are in all Sands.
- **Changes:** `explore_source_change`
- **Engine:** `scenarios.json` › `PowersLongForgotten` › `benefits[TwistedBronze]`: "A loop of twisted bronze set with deep red stones: Invader benefit: For purposes of Explore and Build, Invaders in any Sands are considered to be in all Sands. Spirit benefit: Spirits may shift their {Presence} from a Sands to another at any time."
- **Wiki:** Powers_Long_Forgotten@3141: "| A loop of twisted bronze set with deep red stones | For purposes of Explore and Build, Invaders in any Sands are considered to be in all Sands | Spirits may shift their {Presence} from a Sands to another at any time"
- **Agreement:** `agree`
- **Affects:** Explore eligibility (sources, range)
- **Status:** unreviewed

**R-SC-PowersLongForgotten-2 · Powers Long Forgotten — JaggedTooth**

- **Rule:** An enormous jagged tooth, dripping water: Invader benefit: Whenever Invaders Explore into Wetlands, they immediately Build there. Spirit benefit: Each Spirit gains +2 Energy during any Spirit Phase that they do not Reclaim any cards. Parameters: action = build; slot = explore; when = whenever Invaders Explore into a Wetland, they immediately Build there.
- **Changes:** `extra_invader_action`
- **Engine:** `scenarios.json` › `PowersLongForgotten` › `benefits[JaggedTooth]`: "An enormous jagged tooth, dripping water: Invader benefit: Whenever Invaders Explore into Wetlands, they immediately Build there. Spirit benefit: Each Spirit gains +2 Energy during any Spirit Phase that they do not Reclaim any cards."
- **Wiki:** Powers_Long_Forgotten@3141: "| An enormous jagged tooth, dripping water. | Whenever Invaders Explore into Wetland, they immediately Build there. | Each Spirit gains +2 Energy during any Spirit Phase that they do not Reclaim any cards."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-SC-PowersLongForgotten-3 · Powers Long Forgotten — IcyRibBone**

- **Rule:** A rib-bone crusted with unmelting ice: Invader benefit: Next turn: Spirits get only 1 Card Play. Spirit benefit: During the next normal Ravage: Invaders do -3 Damage per land. Parameters: spirit_benefit = next normal Ravage: Invaders do -3 Damage per land.
- **Changes:** `ravage_damage_change`
- **Engine:** `scenarios.json` › `PowersLongForgotten` › `benefits[IcyRibBone]`: "A rib-bone crusted with unmelting ice: Invader benefit: Next turn: Spirits get only 1 Card Play. Spirit benefit: During the next normal Ravage: Invaders do -3 Damage per land."
- **Wiki:** Powers_Long_Forgotten@3141: "| A rib-bone crusted with unmelting ice | Next Turn: Spirits only get 1 Card Play. | During the next normal Ravage: Invaders do -3 Damage per land."
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SC-PowersLongForgotten-4 · Powers Long Forgotten — FistSizedEmber**

- **Rule:** A fist-sized ember, glowing steadily: Invader benefit: Whenever a {Town}/{City} is added to a Jungle or Wetland without {Town}/{City}, also add 1 {Blight}. Spirit benefit: Spirits do not need to Forget when gaining a Major Power. Parameters: invader_benefit = Town/City added to a Jungle/Wetland without Town/City also adds 1 Blight.
- **Changes:** `other`
- **Engine:** `scenarios.json` › `PowersLongForgotten` › `benefits[FistSizedEmber]`: "A fist-sized ember, glowing steadily: Invader benefit: Whenever a {Town}/{City} is added to a Jungle or Wetland without {Town}/{City}, also add 1 {Blight}. Spirit benefit: Spirits do not need to Forget when gaining a Major Power."
- **Wiki:** Powers_Long_Forgotten@3141: "| A fist-sized ember, glowing steadily | Whenever a {Town} / {City} is added to a Jungle or Wetland without {Town} / {City}, also add 1 {Blight} | Spirits do not need to Forget when gaining a Major Power"
- **Agreement:** `agree`
- **Affects:** see note
- **Status:** unreviewed

**Rituals of the Destroying Flame** (`RitualsOfTheDestroyingFlame`, Branch & Claw, engine difficulty 3)

**R-SC-RitualsOfTheDestroyingFlame-1 · Rituals of the Destroying Flame — Invader Immunity**

- **Rule:** Invader Immunity: Whenever anything would Damage or Destroy Invaders, ignore that Damage/Destruction unless either: • There is a Flame Marker within {1Range}. • The Spirit using the Power or ability pays 2 Energy. For non-Spirit effects (Events, {Dahan} counterattacks, etc) any Spirit may pay the Energy. 2 Energy covers a single {Dahan} counterattack, a single land affected by a main/{Dahan}/token Event, or a single land affected by a Fear Card. Parameters: invader_immunity = Damage/Destroy ignored unless a Flame Marker within 1 Range or 2 Energy paid; 2 Energy covers one Dahan counterattack.
- **Changes:** `dahan_change`
- **Engine:** `scenarios.json` › `RitualsOfTheDestroyingFlame` › `backBodySections[Rules Changes]`: "Invader Immunity: Whenever anything would Damage or Destroy Invaders, ignore that Damage/Destruction unless either: • There is a Flame Marker within {1Range}. • The Spirit using the Power or ability pays 2 Energy. For non-Spirit effects (Events, {Dahan} counterattacks, etc) any Spirit may pay the Energy. 2 Energy covers a single {Dahan} counterattack, a single land affected by a main/{Dahan}/token Event, or a single land affected by a Fear Card."
- **Wiki:** Rituals_of_the_Destroying_Flame@2109: "Invader Immunity Whenever anything would Damage or Destroy Invaders, ignore that Damage/Destruction unless either: - There is a Flame Marker within 1"
- **Agreement:** `agree`
- **Affects:** Ravage ledger (Dahan damage, counterattack)
- **Status:** unreviewed

**Second Wave** (`SecondWave`, Branch & Claw, engine difficulty 1)

**R-SC-SecondWave-1 · Second Wave — Pre-printed Invaders start closer**

- **Rule:** • Pre-printed Invaders start closer to the coast: any which would go into land #6+ instead go to the land numbered 3 lower (So a {Town} printed on land #7 would instead be put into land #4.) Parameters: shift = Invaders printed on land #6+ go to the land numbered 3 lower; +1 Town in land #5.
- **Changes:** `setup_pieces`
- **Engine:** `scenarios.json` › `SecondWave` › `backBodySections[Second Game Setup]`: "• Pre-printed Invaders start closer to the coast: any which would go into land #6+ instead go to the land numbered 3 lower (So a {Town} printed on land #7 would instead be put into land #4.)"
- **Wiki:** Second_Wave@12060: "- Pre-printed Invaders start closer to the coast: any which would go into land #6 + instead go into the land numbered 3 lower. (So a {Town} printed on land #7 would instead be put into land #4.)"
- **Agreement:** `agree`
- **Affects:** initial board; Turn 1 Build forecast
- **Status:** unreviewed

**Ward the Shores** (`WardTheShores`, Branch & Claw, engine difficulty 2)

**R-SC-WardTheShores-1 · Ward the Shores — Defend 3**

- **Rule:** Once placed, each Warding Pattern grants Defend 3 in its land. They are destroyed any time {Blight} is added to their land. Parameters: mechanism = defend; amount = 3; where = lands with a Warding Pattern (destroyed when Blight is added there).
- **Changes:** `ravage_damage_change`
- **Engine:** `scenarios.json` › `WardTheShores` › `backBodySections[Rules Changes]`: "Once placed, each Warding Pattern grants Defend 3 in its land. They are destroyed any time {Blight} is added to their land."
- **Wiki:** Ward_the_Shores@1274: "Once placed, each Warding Pattern grants Defend 3 in its land. They are destroyed any time {Blight} is added to their land."
- **Agreement:** `agree`
- **Affects:** Ravage ledger (damage, Defend, Dahan lost, blight)
- **Status:** unreviewed

**R-SC-WardTheShores-2 · Ward the Shores — Stage III Surge**

- **Rule:** Stage III Surge: During Stage III, Explore adds 1 {Town} in addition to the usual 1 {Explorer}. Parameters: stage = 3; explore_adds = 1 Town in addition to 1 Explorer.
- **Changes:** `explore_change`
- **Engine:** `scenarios.json` › `WardTheShores` › `backBodySections[Rules Changes]`: "Stage III Surge: During Stage III, Explore adds 1 {Town} in addition to the usual 1 {Explorer}."
- **Wiki:** Ward_the_Shores@1274: "Stage III Surge: During Stage III, Explore adds 1 {Town} in addition to the usual 1 {Explorer}."
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**Despicable Theft** (`DespicableTheft`, Jagged Earth, engine difficulty 2)

**R-SC-DespicableTheft-1 · Despicable Theft — Seek to Escape by Ship**

- **Rule:** Seek to Escape by Ship: Before the Ravage Step, all Thieves on the island move: • Thieves adjacent to the Ocean escape: For each, lose 4 earned {Fear} (if possible). If the Thief has a number, also Accelerate the Invader Deck that many times. (Discard the topmost card of the lowest Invader Stage remaining in the deck.) • Other Thieves move 1 land towards their board's Ocean, if possible. Thieves do not count as Invaders, and do not participate in Invader Actions. Parameters: accelerate = N times per escaping numbered Thief; when = before the Ravage Step; fear = lose 4 earned Fear per escaping Thief.
- **Changes:** `deck_recipe`
- **Engine:** `scenarios.json` › `DespicableTheft` › `backBodySections[What Thieves Do]`: "Seek to Escape by Ship: Before the Ravage Step, all Thieves on the island move: • Thieves adjacent to the Ocean escape: For each, lose 4 earned {Fear} (if possible). If the Thief has a number, also Accelerate the Invader Deck that many times. (Discard the topmost card of the lowest Invader Stage remaining in the deck.) • Other Thieves move 1 land towards their board's Ocean, if possible. Thieves do not count as Invaders, and do not participate in Invader Actions."
- **Wiki:** Despicable_Theft@12058: "Seek to Escape by Ship: Before the Ravage Step, all Thieves on the island move: - Thieves adjacent to the Ocean escape: For each, lose 4 earned {Fear} (if possible). If the Thief has a number, also Accelerate the Invader Deck that many times. (Remove the topmost card of the lowest Invader Stage remaining in the deck.) - Other Thieves move 1 land towards their board's Ocean, if possible."
- **Agreement:** `agree` — Engine says Accelerate = 'Discard the topmost card'; Wiki says 'Remove the topmost card'.
- **Resolution:** Accelerate discards the card face-up into the Invader discard (seen), as the engine says; the Wiki's "Remove" is reminder wording (NQ-E; JE rulebook definition, FAQ "Elemental Invocation").
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** sourced (2026-10-19, NQ-E for the Accelerate wording; JE rulebook "Accelerate")

**R-SC-DespicableTheft-2 · Despicable Theft — Arrive with**

- **Rule:** Arrive with {Explorer}: After Setup, when Invaders successfully Explore into a land numbered 6 or higher, 1 Thief is added there from the pool. Parameters: after_successful_explore_into = land #6+; adds = 1 Thief.
- **Changes:** `explore_change`
- **Engine:** `scenarios.json` › `DespicableTheft` › `backBodySections[What Thieves Do]`: "Arrive with {Explorer}: After Setup, when Invaders successfully Explore into a land numbered 6 or higher, 1 Thief is added there from the pool."
- **Wiki:** Despicable_Theft@12058: "Arrive with {Explorer}: After Setup, when Invaders successfully Explore into a land numbered 6 or higher, also add 1 Thief there from the pool."
- **Agreement:** `agree` — Thieves are not Invaders and never take Invader Actions.
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**Elemental Invocation** (`ElementalInvocation`, Jagged Earth, engine difficulty 1)

**R-SC-ElementalInvocation-1 · Elemental Invocation — Accelerate the Invader Deck**

- **Rule:** On each board, add 1 {Town} to land #6. Add 1 Fear Card to Terror Level I. Accelerate the Invader Deck. (Discard the topmost card of the lowest Invader Stage in the deck. At Setup, this will nearly always be Stage I.) Parameters: accelerate = 1; when = Setup; also = 1 Town in land #6 per board; +1 Fear Card at Terror Level I (fearCardModifiers 1/0/0).
- **Changes:** `deck_recipe`
- **Engine:** `scenarios.json` › `ElementalInvocation` › `backBodySections[Setup Changes]`: "On each board, add 1 {Town} to land #6. Add 1 Fear Card to Terror Level I. Accelerate the Invader Deck. (Discard the topmost card of the lowest Invader Stage in the deck. At Setup, this will nearly always be Stage I.)"
- **Wiki:** Elemental_Invocation@3350: "On each board, add 1 {Town} to land #6. Add 1 Fear card to Terror Level I. Accelerate the Invader Deck. (Remove the topmost card of the lowest Invader Stage in the deck. At Setup, this will nearly always be Stage I.)"
- **Agreement:** `agree`
- **Resolution:** Accelerate discards the card face-up into the Invader discard: it is seen and leaves the unseen pool as a known card. FAQ (Elemental Invocation): the rulebook's "discarded" beats the reminder text's "removed" ("rules text trumps reminder text") (NQ-E).
- **Affects:** deck clock; Explore card counting (unseen pool per stage)
- **Status:** sourced (2026-10-19, NQ-E; JE rulebook "Accelerate", FAQ "Elemental Invocation")

**The Great River** (`TheGreatRiver`, Jagged Earth, engine difficulty 3)

**R-SC-TheGreatRiver-1 · The Great River — After the Build Step**

- **Rule:** After the Build Step, Invaders Build in all Coastal lands without {Town}/{City} but with {Explorer}. Parameters: action = build; slot = after Build Step; when = all Coastal lands without Town/City but with Explorer.
- **Changes:** `extra_invader_action`
- **Engine:** `scenarios.json` › `TheGreatRiver` › `backBodySections[Rules Changes]`: "After the Build Step, Invaders Build in all Coastal lands without {Town}/{City} but with {Explorer}."
- **Wiki:** The_Great_River@12067: "After the Build Step, Invaders Build in all Coastal lands without {Town}/{City} but with {Explorer}."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-SC-TheGreatRiver-2 · The Great River — After the Explore Step**

- **Rule:** After the Explore Step, Invaders Explore into all Coastal lands without {Town}/{City}. Parameters: action = explore; slot = after Explore Step; when = all Coastal lands without Town/City.
- **Changes:** `extra_invader_action`
- **Engine:** `scenarios.json` › `TheGreatRiver` › `backBodySections[Rules Changes]`: "After the Explore Step, Invaders Explore into all Coastal lands without {Town}/{City}."
- **Wiki:** The_Great_River@12067: "After the Explore Step, Invaders Explore into all Coastal lands without {Town}/{City}."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

**R-SC-TheGreatRiver-3 · The Great River — At the end of the**

- **Rule:** At the end of the {Slow} phase, each {Town} moves one land Inland (farther away from the Ocean), starting with the most-Inland lands and working their way to the least-Inland. Parameters: when = end of the Slow Phase; move = each Town one land Inland (most-Inland first); ignores Isolate unless a Spirit pays 1 Energy.
- **Changes:** `other`
- **Engine:** `scenarios.json` › `TheGreatRiver` › `backBodySections[Relentless Migration Eastwards:]`: "At the end of the {Slow} phase, each {Town} moves one land Inland (farther away from the Ocean), starting with the most-Inland lands and working their way to the least-Inland."
- **Wiki:** The_Great_River@12067: "At the end of the {Slowicon.png} phase, move each {Town} one land Inland (farther away from the Ocean). Start with the most-Inland lands and work your way to the least-Inland. (You can mark the lands that are Range 2 from the Ocean with Scenario Markers to make this easier, if desired.)"
- **Agreement:** `agree` — Runs after Slow Powers, so it moves Towns after the player's last chance to act this turn and before next turn's Build/Ravage.
- **Affects:** see note
- **Status:** unreviewed

**R-SC-TheGreatRiver-4 · The Great River — If more than 3**

- **Rule:** If more than 3 {Town} per starting board in total escape off the edge of the map, the Invaders win. Parameters: escaped_towns_max_engine = 3 per starting board; escaped_towns_max_wiki = 3 per player.
- **Changes:** `loss_condition`
- **Engine:** `scenarios.json` › `TheGreatRiver` › `backBodySections[Rules Changes]`: "If more than 3 {Town} per starting board in total escape off the edge of the map, the Invaders win."
- **Wiki:** The_Great_River@12067: "If more than 3 {Town} per player escape off the edge of the map (are on this Scenario Card), the Invaders win."
- **Agreement:** `conflict` — Per starting board vs per player.
- **Resolution:** Per board. FAQ ("When playing with an extra board"): "The Great River - Additional Loss Conditions - the per-player measurement should be per-board … 'If more than (3 x # of boards) Towns escape…'". Without an extra board the two are equal (NQ-C).
- **Affects:** loss clocks (race chart)
- **Status:** sourced (2026-10-19, NQ-C; FAQ "When playing with an extra board")

**A Diversity of Spirits** (`ADiversityOfSpirits`, Promo Pack 2, engine difficulty 0): none found. Communication and turn-order limits only; Strength in Diversity is a Growth bonus.

**Varied Terrains** (`VariedTerrains`, Promo Pack 2, engine difficulty 2)

**R-SC-VariedTerrains-1 · Varied Terrains — Mountains May Hide Gold**

- **Rule:** Mountains May Hide Gold: Add +1 {Explorer} when Exploring Mountains. (Including during Setup, if appropriate.) Parameters: terrain = Mountain; plus_explorer = 1; also = Setup.
- **Changes:** `explore_change`
- **Engine:** `scenarios.json` › `VariedTerrains` › `backBodySections[Rules Changes]`: "Mountains May Hide Gold: Add +1 {Explorer} when Exploring Mountains. (Including during Setup, if appropriate.)"
- **Wiki:** Varied_Terrains@6140: "Mountains May Hide Gold: Add +1 {Explorer} when Exploring Mountains. (Including during Setup, if appropriate.)"
- **Agreement:** `agree`
- **Affects:** Explore odds and Explore forecast per land
- **Status:** unreviewed

**R-SC-VariedTerrains-2 · Varied Terrains — Jungles Clear-Cut**

- **Rule:** Jungles Clear-Cut for Land: After Invaders Build 1 or more {City} in a Jungle, add 1 {Blight} there. Parameters: terrain = Jungle; after_city_build = add 1 Blight.
- **Changes:** `build_change`
- **Engine:** `scenarios.json` › `VariedTerrains` › `backBodySections[Rules Changes]`: "Jungles Clear-Cut for Land: After Invaders Build 1 or more {City} in a Jungle, add 1 {Blight} there."
- **Wiki:** Varied_Terrains@6140: "Jungles Clear-Cut for Land: After Invaders Build 1 or more {City} in a Jungle, add 1 {Blight} there."
- **Agreement:** `agree`
- **Affects:** Build forecast
- **Status:** unreviewed

**R-SC-VariedTerrains-3 · Varied Terrains — Fragile Sands**

- **Rule:** Fragile Sands: After Setup: {Blight} added to Sands cascades as if {Blight} were already present, even when it is not. Parameters: terrain = Sands; always_cascades = True; from = after Setup.
- **Changes:** `cascade_change`
- **Engine:** `scenarios.json` › `VariedTerrains` › `backBodySections[Rules Changes]`: "Fragile Sands: After Setup: {Blight} added to Sands cascades as if {Blight} were already present, even when it is not."
- **Wiki:** Varied_Terrains@6140: "Fragile Sands: After Setup: {Blight} added to Sands cascades as if {Blight} were already present, even when it is not."
- **Agreement:** `agree`
- **Affects:** cascade badge; blight runway; presence at risk
- **Status:** unreviewed

**R-SC-VariedTerrains-4 · Varied Terrains — Fertile Wetlands**

- **Rule:** Fertile Wetlands: After Invaders successfully Build in a Wetland, they Build there a second time. Parameters: action = build; slot = build; when = after a successful Build in a Wetland, Build there a second time.
- **Changes:** `extra_invader_action`
- **Engine:** `scenarios.json` › `VariedTerrains` › `backBodySections[Rules Changes]`: "Fertile Wetlands: After Invaders successfully Build in a Wetland, they Build there a second time."
- **Wiki:** Varied_Terrains@6140: "Fertile Wetlands: After Invaders successfully Build in a Wetland, they Build there a second time."
- **Agreement:** `agree`
- **Affects:** invader forecast (Ravage/Build/Explore land lists); land-tempo badges
- **Status:** unreviewed

### 5.3 Events (all 62 in `event_cards.json`)

Each listed section changes the pipeline when it resolves (Event step: after Blighted Island, before Fear). Every section of every event was scanned; unlisted sections add or move pieces, Fear, Energy or Blight without changing a rule. Wiki evidence is the repo fetcher's event cache (`/home/brett/repos/spirit-island/.wiki-cache/event_*.json`, fetched 2026-04-19, revision ids not recorded); `agree` means text similarity ≥ 0.84 or a manual read found the same rule. Years of Little Rain's Canny Defense section was missed by that scan and added on 2026-10-19 (Q30). Pool (Q15, Brett 2026-10-19; NI rulebook p.5 "Dropped Cards"): Outpaced, War Touches the Island's Shores and A Strange Madness Among the Beasts are dropped and are not in the engine; Far-off Wars Touch the Island replaces War Touches the Island's Shores; Slave Rebellion is added only against France (Steam's Test 1 pool: 61 Events, plus Slave Rebellion with France).


*Branch & Claw events*

- **CulturalAssimilation**
  - `other` — **Reprisal Against the Dahan** [TerrorLevelIII]. Rule: note = 3 Damage to Dahan ignoring Defend (not a Ravage). Engine `event_cards.json` › `CulturalAssimilation`: "On Each Board: Choose a land with {Dahan} and {Town}/{City}. Invaders do 3 Damage to {Dahan} there, ignoring Defend Powers.". Wiki `event_cultural_assimilation.json`: "On Each Board: Choose a land with Dahan and Town/City. Invaders do 3 Damage to Dahan there, ignoring Defend Powers.". **`agree`**. Affects: see note. Status: unreviewed.
- **DistantExploration**
  - `explore_source_change` — **Distant Exploration** [TerrorLevelI]. Rule: explore_range = +1; duration = this Invader Phase's Explore step. Engine `event_cards.json` › `DistantExploration`: "During the Explore step, Invaders Explore at +1 distance. (So a land 2 away from a {Town}/{City}/Ocean will be Explored into.)". Wiki `event_distant_exploration.json`: "During the Explore step, Invaders Explore at +1 distance. (So a land 2 away from a Town/City/Ocean will be Explored into.)". **`agree`**. Affects: Explore eligibility (sources, range). Status: unreviewed.
  - `ravage_damage_change` — **Fearful Mobs** [TerrorLevelII/TerrorLevelIII]. Rule: per_land_plus = 3; where = lands with 3 or more Invaders. Engine `event_cards.json` › `DistantExploration`: "When Ravaging, Invaders do +3 Damage (per land) in lands with 3 or more Invaders.". Wiki `event_distant_exploration.json`: "When Ravaging, Invaders do +3 Damage (per land) in lands with 3 or more Invaders.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **FarmersSeekTheDahanForAid**
  - `piece_health` — **Spurn the Invaders** [Choice]. Rule: town_city_health = -1; min = 1; duration = until end of turn. Engine `event_cards.json` › `FarmersSeekTheDahanForAid`: "• On Each Board: 2 Damage to {Dahan} in a land with {Town}/{City} and {Dahan}. • On Each Board: Add 1 {Blight} to a land with at least 2 {Town}/{City}. • {Town}/{City} have -1 Health (min. 1) until the end of the turn.". Wiki `event_farmers_seek_the_dahan_for_aid.json`: " • On Each Board: 2 Damage to Dahan in a land with Town/City. • On Each Board: Add 1 Blight to a land with at least 2 Town/City. • Town/City have -1 Health (min. 1) until the end of the turn.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
  - `skip_invader_action` — **Teach the Invaders** [Choice]. Rule: action = ravage; replaced_by = build; which = the next normal Ravage (may be a future turn). Engine `event_cards.json` › `FarmersSeekTheDahanForAid`: "• On Each Board: Add 1 {Town} to a land with {Dahan}. • The next normal Ravage becomes a Build. (This could be on a future turn.)". Wiki `event_farmers_seek_the_dahan_for_aid.json`: " farming techniques more in tune with the island's life. You recommend they:". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **HeavyFarming**
  - `ravage_damage_change` — **Heavy Farming** [HealthyIsland]. Rule: per_town_plus = 1; which = next normal Ravage (card waits by the Ravage space). Engine `event_cards.json` › `HeavyFarming`: "During the next normal Ravage, each {Town} does +1 Damage. (If there is no Ravage this Phase, leave this card by the Ravage Space until one happens.)". Wiki `event_heavy_farming.json`: "During the next normal Ravage, each Town does +1 Damage. (If there is no Ravage this Phase, leave this card by the Ravage Space until one happens.)". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **InterestingDiscoveries**
  - `ravage_damage_change` — **Increasing Aggression** [StageII/StageIII]. Rule: per_land_plus = 1. Engine `event_cards.json` › `InterestingDiscoveries`: "Invaders do +1 Damage (per land) when Ravaging.". Wiki `event_interesting_discoveries.json`: "Invaders do +1 Damage (per land) when Ravaging.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **InvadersSurgeInland**
  - `ravage_damage_change` — **Canny Defense** [Dahan]. Rule: mechanism = defend; amount = 1 per Dahan; where = every land. Engine `event_cards.json` › `InvadersSurgeInland`: "During Ravage, in every land, Defend 1 per {Dahan} in the land.". Wiki `event_invaders_surge_inland.json`: "During Ravage, in every land, Defend 1 per Dahan in the land.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: sourced (2026-10-19, Q30; same Canny Defense text as Years of Little Rain).
  - `other` — **Invaders Surge Inland** [HealthyIsland]. Rule: note = moves a Coastal Town 1-2 lands inland: moves an Explore source. Engine `event_cards.json` › `InvadersSurgeInland`: "On a Coastal land on Each Board: Move 1 {Town} one or two lands Inland.". Wiki `event_invaders_surge_inland.json`: "On a Coastal land on Each Board: Move 1 Town one or two lands Inland.". **`agree`**. Affects: see note. Status: unreviewed.
- **InvestigationOfDangers**
  - `ravage_damage_change` — **Destroy the Unnatural!** [TerrorLevelII/TerrorLevelIII]. Rule: per_land_plus = 3; where = lands with Presence. Engine `event_cards.json` › `InvestigationOfDangers`: "Invaders do +3 Damage (per land) when Ravaging in lands with {Presence}.". Wiki `event_investigation_of_dangers.json`: "Invaders do +3 Damage (per land) when Ravaging in lands with Presence.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
  - `other` — **Rouse the Spirits** [Dahan]. Rule: timing = eligible Spirits may use a Slow Power during the Event. Engine `event_cards.json` › `InvestigationOfDangers`: "Each Spirit with at least 3 {Dahan} among all its lands may use a {Slow} Power now (instead of later).". Wiki `event_investigation_of_dangers.json`: "Each Spirit with at least 3 Dahan among all its lands may use a Slow Power now (instead of later).". **`agree`**. Affects: see note. Status: unreviewed.
- **MissionariesArrive**
  - `ravage_damage_change` — **Curse the Tongues of the Foreigners** [Choice]. Rule: per_city_plus = 3; which = next normal Ravage. Engine `event_cards.json` › `MissionariesArrive`: "Cost: 4 Energy per player. Aided by {Sun}. • Each Spirit adds 1 {Disease} to a land with {City}. • 2 {Fear} per player. • During the next normal Ravage, {City} do +3 Damage.". Wiki `event_missionaries_arrive.json`: " Cost: 4 Energy per player. Aided by Sun. • Each Spirit: Add 1 Disease to a land with City. • 2 Fear per player. • During the next normal Ravage, City do +3 Damage.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **NewSpeciesSpread**: none found.
- **PopulationRises**: none found.
- **PromisingFarmland**
  - `explore_change` — **Promising Farmland** [HealthyIsland]. Rule: once_per_board = place 1 Town instead of 1 Explorer. Engine `event_cards.json` › `PromisingFarmland`: "When Exploring, once per board, place 1 {Town} instead of 1 {Explorer}.". Wiki `event_promising_farmland.json`: "When Exploring, once per board, place 1 Town instead of 1 Explorer.". **`agree`**. Affects: Explore odds and Explore forecast per land. Status: unreviewed.
  - `extra_invader_action` — **New Cash Crops Take Hold** [BlightedIsland]. Rule: action = ravage; slot = event; when = a Ravage Step for one terrain not shown under any Invader Action (preventable per board). Engine `event_cards.json` › `PromisingFarmland`: "Perform a Ravage Step for one terrain not shown under any Invader Action. Spirits may prevent this on any/all boards by destroying 2 {Presence} from each board to be protected.". Wiki `event_promising_farmland.json`: "Invaders immediately Ravage in 1 terrain type not showing under any Invader Action. Spirits may prevent this on any/all boards by destroying 2 Presence from each board to be protected.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `ravage_damage_change` — **Canny Defense** [Dahan]. Rule: mechanism = defend; amount = 1 per Dahan; where = every land. Engine `event_cards.json` › `PromisingFarmland`: "During Ravage, in every land, Defend 1 per {Dahan} in the land.". Wiki `event_promising_farmland.json`: "During Ravage, in every land, Defend 1 per Dahan in the land.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: sourced (2026-10-19, Q30; same Canny Defense text as Years of Little Rain).
- **PuttingDownRoots**
  - `skip_invader_action` — **Stricken** [Disease/Strife]. Rule: action = ravage; where = lands with Disease/Strife. Engine `event_cards.json` › `PuttingDownRoots`: "In lands with {Disease}/{Strife}, Invaders skip Ravage Actions.". Wiki `event_putting_down_roots.json`: "In lands with Disease/Strife, Invaders skip Ravage Actions.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **RisingInterestInTheIsland**
  - `deck_recipe` — **Ignore Their Curiosity** [Choice]. Rule: invader_deck = return the top card to the box (skip cards specially placed during Setup); visibility = unseen: nobody looks at it, the deck count drops by one and the card stays in the unseen pool (Q23). Engine `event_cards.json` › `RisingInterestInTheIsland`: "• Return the top card of the Invader Deck to the box. (Skip cards specially placed during Setup.) • On Each Board: Add 1 {Town} to a land without one.". Wiki `event_rising_interest_in_the_island.json`: " • Return the top card of the Invader Deck to the box. (Skip cards specifically placed during Setup.) • On Each Board: Add 1 Town to a land without one.". **`agree`**. Affects: deck clock; Explore card counting (unseen pool per stage). Status: reviewed by Brett (2026-10-19, Q23; FAQ Branch & Claw).
  - `ravage_damage_change` — **Weave Lies in the Minds of Their Observers** [Choice]. Rule: per_town_city_plus = 1; which = next normal Ravage; fear_deck = return top Fear Card to the box, unseen (Q23). Engine `event_cards.json` › `RisingInterestInTheIsland`: "Cost: 4 Energy per player. Aided by {Air}. • Return the top Fear Card to the box. • During the next normal Ravage, each {Town}/{City} does +1 Damage.". Wiki `event_rising_interest_in_the_island.json`: " Cost: 4 Energy per player. Aided by Air. • Return the top Fear Card to the box. • During the next normal Ravage, each Town/City deals +1 Damage.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: reviewed by Brett (2026-10-19, Q23, for the unseen Fear Card).
- **SacredSitesUnderThreat**: none found.
- **SearchForNewLands**: none found.
- **SeekingTheInterior**
  - `other` — **Seeking the Interior** [StageI]. Rule: note = pushes Coastal Explorers inland: moves future Build lands. Engine `event_cards.json` › `SeekingTheInterior`: "In each Coastal land, Push 1 {Explorer} one land Inland.". Wiki `event_seeking_the_interior.json`: "In each Coastal land, Push 1 Explorer one land Inland.". **`agree`**. Affects: see note. Status: unreviewed.
- **SlaveRebellion**
  - `other` — **Small Uprising** [StageI/StageII]. Rule: adversary = TheKingdomOfFrance L2; adds = Strife to 1 Town per board; draw another Event; card returns to the Event Deck. Engine `event_cards.json` › `SlaveRebellion`: "On Each Board: Add {Strife} to 1 {Town}. After finishing the Event Card, draw another one, then return this card to the Event Deck as per Setup.". Wiki `event_slave_rebellion.json`: "On Each Board: Add Strife to 1 Town. After finishing the Event Card, draw another one, then return this card to the Event Deck as per Setup.". **`agree`**. Affects: see note. Status: unreviewed.
  - `other` — **Rebellion** [StageIII]. Rule: adversary = TheKingdomOfFrance L2; effect = Destroy 1 Town per board, Strife damage, draw another Event, card discarded. Engine `event_cards.json` › `SlaveRebellion`: "On Each Board: Destroy 1 {Town}. Add 1 {Strife} to any 2 {Town}/{City}. Then, every Invader takes 1 Damage per {Strife} it has. After finishing this Event Card, draw another one. This card is discarded.". Wiki `event_slave_rebellion.json`: "On Each Board: Destroy 1 Town. Add Strife to any 2 Towns/Cities. Then, every Invader takes 1 Damage per Strife it has. After finishing this Event Card, draw another one. This card is discarded.". **`agree`**. Affects: see note. Status: unreviewed.
- **StrangeTalesAttractExplorers**
  - `other` — **Strange Tales Attract Explorers** [StageI]. Rule: after = advancing Invader Cards; adds = 1 Explorer to each land with Sacred Site. Engine `event_cards.json` › `StrangeTalesAttractExplorers`: "Now: 1 {Fear} per Spirit that has at least 1 {SacredSite}. After advancing Invader Cards: Add 1 {Explorer} to each land with {SacredSite}.". Wiki `event_strange_tales_attract_explorers.json`: "Now: 1 Fear per Spirit that has at least 1 Sacred Site. After advancing Invader Cards: Add 1 Explorer to each land with a Sacred Site.". **`agree`**. Affects: see note. Status: unreviewed.
  - `extra_invader_action` — **Fortification** [StageII/StageIII]. Rule: action = build; slot = event; when = after advancing Invader Cards: a Build Step for one terrain not shown under any Invader Action. Engine `event_cards.json` › `StrangeTalesAttractExplorers`: "After advancing Invader Cards: perform a Build Step for one terrain not shown under any Invader Action. (If there is no such terrain, nothing happens.)". Wiki `event_strange_tales_attract_explorers.json`: "After advancing Invader Cards: Invaders Build in one terrain not shown under any Invader Action. (If there's no such terrain, nothing happens.)". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
- **TightKnitCommunities**
  - `piece_health` — **Tight-Knit Communities** [HealthyIsland]. Rule: town_city_health = 1; duration = rest of this turn. Engine `event_cards.json` › `TightKnitCommunities`: "For the rest of this turn, {Town}/{City} have +1 Health.". Wiki `event_tight-knit_communities.json`: "For the rest of the turn, Towns/Cities have +1 Health.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
- **UrbanDevelopment**
  - `ravage_damage_change` — **Urban Development** [HealthyIsland]. Rule: per_city_plus = 2; which = next normal Ravage (card waits by the Ravage space). Engine `event_cards.json` › `UrbanDevelopment`: "During the next normal Ravage, each {City} does +2 Damage. (If there is no Ravage this Phase, leave this card by the Ravage Space until one happens.)". Wiki `event_urban_development.json`: "During the next normal Ravage, each City deals +2 Damage. (If there is no Ravage this Phase, leave this card by the Ravage Space until one happens.)". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
  - `cascade_change` — **Festering Pits of Blight** [BlightedIsland]. Rule: adds_blight = 1; cascade = False; where = a land with at least 2 Blight, boards with Invaders. Engine `event_cards.json` › `UrbanDevelopment`: "On Each Board with Invaders: Add 1 {Blight} to a land with at least 2 {Blight}, but do not cascade.". Wiki `event_urban_development.json`: "On Each Board with Invaders: Add 1 Blight to a land with at least 2 Blight, but do not cascade.". **`agree`**. Affects: cascade badge; blight runway; presence at risk. Status: unreviewed.
  - `build_change` — **Lingering Plagues** [Disease]. Rule: disease_prevents_build = False; duration = this Invader Phase. Engine `event_cards.json` › `UrbanDevelopment`: "On Each Board: Add 1 {Disease}. {Disease} doesn't prevent Builds this Invader Phase.". Wiki `event_urban_development.json`: "On Each Board: Add 1 Disease. Ignore Disease during Builds this Invader Phase.". **`agree`** — Wiki: 'Ignore Disease during Builds'.. Affects: Build forecast. Status: unreviewed.
- **WaveOfReconnaissance**
  - `explore_change` — **Wave of Reconnaissance** [StageI]. Rule: plus_explorer = 1; where = each land Explored. Engine `event_cards.json` › `WaveOfReconnaissance`: "When Exploring: Add 1 additional {Explorer} to each land Explored.". Wiki `event_wave_of_reconnaissance.json`: "When Exploring: Add 1 additional Explorer to each land Explored.". **`agree`**. Affects: Explore odds and Explore forecast per land. Status: unreviewed.
  - `skip_invader_action` — **Stricken** [Disease/Strife]. Rule: action = ravage; where = lands with Disease/Strife. Engine `event_cards.json` › `WaveOfReconnaissance`: "In lands with {Disease}/{Strife}, Invaders skip Ravage Actions.". Wiki `event_wave_of_reconnaissance.json`: "Invaders do not Ravage in lands with Disease/Strife.". **`agree`** — Wiki: 'Invaders do not Ravage'.. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **WellPreparedExplorers**
  - `piece_health` — **Well-Prepared Explorers** [HealthyIsland]. Rule: explorer_health = 1; duration = rest of this turn. Engine `event_cards.json` › `WellPreparedExplorers`: "For the rest of this turn, {Explorer} have +1 Health.". Wiki `event_well-prepared_explorers.json`: "For the rest of the turn, Explorers have +1 Health.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
- **YearsOfLittleRain**
  - `piece_health` — **Let the Plants Die and the Land Wither** [Choice]. Rule: town_city_dahan_health = -1; min = 1; duration = until end of turn. Engine `event_cards.json` › `YearsOfLittleRain`: "• For each board, discard the top Minor Power. If it lacks {Water}, add 1 {Blight} to a Sands. • {Town}, {City}, and {Dahan} have -1 Health (min. 1) until the end of the turn.". Wiki `event_years_of_little_rain.json`: " • For each board, discard the top Minor Power. If it lacks Water, add 1 Blight to a Sands. • Towns, Cities, and Dahan have -1 Health (minimum 1) until the end of the turn.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
  - `ravage_damage_change` — **Canny Defense** [Dahan]. Rule: mechanism = defend; amount = 1 per Dahan; where = every land. Engine `event_cards.json` › `YearsOfLittleRain`: "During Ravage, in every land, Defend 1 per {Dahan} in the land.". Wiki `event_years_of_little_rain.json`: "During Ravage, in every land, Defend 1 per Dahan in the land.". **`agree`** — Missed by the first scan; added 2026-10-19 (Q30). Test 1 Turn 5, Mountain C7: 2 Dahan, 3 Damage, 1 to the land. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: sourced (2026-10-19, Q30; card text).

*Jagged Earth events*

- **BureaucratsAdjustFunding**
  - `build_change` — **Bureaucrats Adjust Funding** [TerrorLevelI/TerrorLevelII]. Rule: 9plus_town_city_board = Build Cards skip highest-numbered matching land; 3minus_town_city_board = 1 extra Build Action in highest-numbered matching land, even without Invaders. Engine `event_cards.json` › `BureaucratsAdjustFunding`: "On Each Board with 9 or more {Town}/{City}: Build Cards skip the highest-numbered matching land. On Each Board with 3 or fewer {Town}/{City}: Build Cards cause 1 extra Build Action in the highest-numbered matching land. (This can affect a land without Invaders.)". Wiki `event_bureaucrats_adjust_funding.json`: "On Each Board with 9 or more Towns/Cities: Build Cards skip the highest-numbered matching land. On Each Board with 3 or fewer Towns/Cities: Build Cards cause 1 extra Build Action in the highest-numbered matching land. (This can Build in a land without Invaders.)". **`agree`**. Affects: Build forecast. Status: unreviewed.
  - `extra_invader_action` — **Ransack and Run** [TerrorLevelIII]. Rule: action = ravage; slot = event; when = after the Ravage Step, land with most City on each board, then Remove 1 City. Engine `event_cards.json` › `BureaucratsAdjustFunding`: "After the Ravage Step, On Each Board: In the land with the most {City} (min. 1), Ravage, then Remove 1 {City}.". Wiki `event_bureaucrats_adjust_funding.json`: "After the Ravage Step, On Each Board: In the land with the most Cities (minimum 1), Ravage, then Remove 1 City.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `ravage_damage_change` — **Coordinated Defense** [Dahan]. Rule: mechanism = defend; amount = 5; where = lands with Dahan and Presence. Engine `event_cards.json` › `BureaucratsAdjustFunding`: "When Invaders Ravage, if the land has both {Dahan} and {Presence}, Defend 5.". Wiki `event_bureaucrats_adjust_funding.json`: "When Invaders Ravage, if the land has both Dahan and Presence, Defend 5.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **CitiesRise**
  - `ravage_blight_change` — **Depleted Soil** [BlightedIsland]. Rule: land_health = -1; where = lands with Town/City. Engine `event_cards.json` › `CitiesRise`: "In lands with {Town}/{City}, the land has -1 Health.". Wiki `event_cities_rise.json`: "In lands with Towns/Cities, it only takes 1 Damage to add Blight.". **`conflict`** — Engine 'the land has -1 Health'; Wiki 'it only takes 1 Damage to add Blight'. Same unless another land-Health modifier applies.. Resolution: the engine's wording is right, "the land has -1 Health" (NI p.14); land-Health modifiers add together and the total can't go below 1 (NQ-I). Affects: Ravage ledger (blight added); blight runway. Status: sourced (2026-10-19, NQ-I; NI rulebook p.14).
  - `ravage_damage_change` — **Careful Defense** [Dahan]. Rule: mechanism = defend; amount = 2; where = lands with Dahan. Engine `event_cards.json` › `CitiesRise`: "When Invaders Ravage, if the land has {Dahan}, Defend 2.". Wiki `event_cities_rise.json`: "When Invaders Ravage, if the land has Dahan, Defend 2.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **CivicEngagement**
  - `ravage_damage_change` — **Civic Engagement** [HealthyIsland]. Rule: per_land_plus = 1; where = lands with Town/City; condition = per board, if no Strife could be removed. Engine `event_cards.json` › `CivicEngagement`: "On Each Board: Remove 1 {Strife}. If you can't, Invaders do +1 Damage (per land) when Ravaging in lands with {Town}/{City}.". Wiki `event_civic_engagement.json`: "On Each Board: Remove 1 Strife. If you can't, Invaders do +1 Damage (per land) when Ravaging in lands with Towns/Cities.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **CoastalTownsMultiply**
  - `ravage_damage_change` — **Cooperation Among Towns** [BlightedIsland]. Rule: per_land_plus = 1; where = lands with Town/City and no Strife. Engine `event_cards.json` › `CoastalTownsMultiply`: "Invaders do +1 Damage (per land) when Ravaging in lands with {Town}/{City} and no {Strife}.". Wiki `event_coastal_towns_multiply.json`: "Invaders do +1 Damage (per land) when Ravaging in lands with Towns/Cities and no Strife.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **DahanTradeWithTheInvaders**
  - `dahan_change` — **Ignore Their Interest** [Choice]. Rule: dahan_participate_in_ravage = False; duration = this turn. Engine `event_cards.json` › `DahanTradeWithTheInvaders`: "• {Dahan} do not participate in Ravages this turn.". Wiki `event_dahan_trade_with_the_invaders.json`: " • Dahan do not participate in Ravages this turn.". **`agree`**. Affects: Ravage ledger (Dahan damage, counterattack). Status: unreviewed.
  - `dahan_change` — **Change and Help Them Learn From the Enemy** [Choice]. Rule: dahan_participate_in_ravage = False; duration = this turn; later = from next turn on each Dahan provides Defend 1 in its land. Engine `event_cards.json` › `DahanTradeWithTheInvaders`: "Cost: 6 Energy per player, aided by {Air}. • {Dahan} do not participate in Ravages this turn. • Each Spirit Forgets a Power Card. • From next turn on, each {Dahan} provides Defend 1 in its land.". Wiki `event_dahan_trade_with_the_invaders.json`: " Cost: 6 Energy per player, aided by Air. • Dahan do not participate in Ravages this turn. • Each Spirit Forgets a Power Card. • From next turn on, each Dahan provides Defend 1 in its land. (There is a Reminder Card for this.)". **`agree`**. Affects: Ravage ledger (Dahan damage, counterattack). Status: unreviewed.
- **EagerExplorers**
  - `build_change` — **Eager Explorers** [TerrorLevelI]. Rule: after = first successful Build Action on each board; adds = 1 Explorer to an adjacent land without Invaders. Engine `event_cards.json` › `EagerExplorers`: "On Each Board: After the first successful Build Action, add 1 {Explorer} to an adjacent land without Invaders.". Wiki `event_eager_explorers.json`: "On Each Board: After the first successful Build Action, add 1 Explorer to an adjacent land without Invaders.". **`agree`**. Affects: Build forecast. Status: unreviewed.
  - `ravage_damage_change` — **Xenophobia** [TerrorLevelII/TerrorLevelIII]. Rule: where = lands with Dahan; per_invader_land = -1; per_invader_dahan = 1. Engine `event_cards.json` › `EagerExplorers`: "When Ravaging in lands with {Dahan}, each individual Invader does -1 Damage to the land and +1 Damage to {Dahan}.". Wiki `event_eager_explorers.json`: "When Ravaging in lands with Dahan, each individual Invader does -1 Damage to the land and +1 Damage to Dahan.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **FortuneSeekers**
  - `explore_change` — **Fortune-Seekers** [HealthyIsland]. Rule: next_explore_card_matches = all lands without Town/City; escalation = ignored. Engine `event_cards.json` › `FortuneSeekers`: "The next Explore Card matches all lands without {Town}/{City} instead of the printed land type. Ignore any {Escalation} on it.". Wiki `event_fortune-seekers.json`: "The next Explore Card matches all lands without Towns/Cities instead of the printed land type. Ignore any Escalation on it.". **`agree`**. Affects: Explore odds and Explore forecast per land. Status: unreviewed.
  - `ravage_blight_change` — **The Land's Bounty Exhausted** [BlightedIsland]. Rule: land_health = -1; where = lands without Presence. Engine `event_cards.json` › `FortuneSeekers`: "In lands without {Presence}, the land has -1 Health.". Wiki `event_fortune-seekers.json`: "In lands without Presence, it only takes 1 Damage to add Blight.". **`conflict`** — Engine '-1 Health'; Wiki 'only takes 1 Damage to add Blight'. Same unless another land-Health modifier applies.. Resolution: the engine's wording is right, "the land has -1 Health" (NI p.14); land-Health modifiers add together and the total can't go below 1 (NQ-I). Affects: Ravage ledger (blight added); blight runway. Status: sourced (2026-10-19, NQ-I; NI rulebook p.14).
- **GradualCorruption**: none found.
- **HardWorkingSettlers**
  - `extra_invader_action` — **Act Cautiously in the Background** [Choice]. Rule: action = ravage; slot = event; when = immediately, land with most Invaders matching a Ravage Card, each board. Engine `event_cards.json` › `HardWorkingSettlers`: "On Each Board: • Push up to 2 {Dahan}. • Add 1 {Town} to a land without {Town}. • Immediately Ravage in the land with the most Invaders that matches a Ravage Card.". Wiki `event_hard-working_settlers.json`: " On Each Board: • Push up to 2 Dahan. • Add 1 Town to a land without Towns. • Immediately Ravage in the land with the most Invaders that matches a Ravage Card.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `skip_invader_action` — **Create Unnerving Distractions** [Choice]. Rule: action = ravage; which = up to 1 matching land per board (players choose); deck = remove the bottommost Stage II and Stage III cards from the game. Engine `event_cards.json` › `HardWorkingSettlers`: "...at the cost of piquing their interest. • Ravage Cards skip up to one matching land on each board. (Players choose which.) • 1 {Fear} per player. • Remove the bottommost Stage II and Stage III Cards in the Invader Deck from the game.". Wiki `event_hard-working_settlers.json`: " ...at the cost of piquing their interest. • Ravage Cards skip up to one matching land on each board. (Players choose which.) • 1 Fear per player. • Remove the bottommost Stage II and Stage III Cards in the Invader Deck from the game.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **HarvestBountyHarvestDust**
  - `extra_invader_action` — **Harvest Bounty, Harvest Dust** [StageI/StageII]. Rule: action = ravage; slot = event; when = if Healthy: a land with Town/City on each board. Engine `event_cards.json` › `HarvestBountyHarvestDust`: "If the island is Healthy, On Each Board: Choose a land with {Town}/{City}. Ravage there. If the island is Blighted, On Each Board: Push 1 {Town} from a land with 2 or more {Town}/{City} to a land without {Town}/{City}.". Wiki `event_harvest_bounty_harvest_dust.json`: "If the island is Healthy, On Each Board: Choose a land with Towns/Cities. Ravage there. If the island is Blighted, On Each Board: Push 1 Town from a land with 2 or more Towns/Cities to a land without Towns/Cities.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `cascade_change` — **Widespread Clearcutting** [StageIII]. Rule: adds_blight = 1; cascade = False. Engine `event_cards.json` › `HarvestBountyHarvestDust`: "On Each Board: Choose a land with {Town}/{City}. Add 1 {Blight} there, without cascading. Players may Destroy 2 {Presence}/{Dahan} in that land to prevent adding {Blight}.". Wiki `event_harvest_bounty_harvest_dust.json`: "On Each Board: Choose a land with Towns/Cities. Add 1 Blight there, without cascading. Players may Destroy 2 Presence/Dahan in that land to prevent adding Blight.". **`agree`**. Affects: cascade badge; blight runway; presence at risk. Status: unreviewed.
- **InvestedAristocracy**
  - `other` — **Invested Aristocracy** [StageI/StageII]. Rule: after = Ravage Step; adds = 1 Town to highest-numbered land with no Invaders, on boards where no Blight was added. Engine `event_cards.json` › `InvestedAristocracy`: "After the Ravage Step, On Each Board: If no {Blight} was added to this board, add 1 {Town} to the highest-numbered land with no Invaders.". Wiki `event_invested_aristocracy.json`: "After the Ravage Step, On Each Board: If no Blight was added to this board, add 1 Town to the highest-numbered land with no Invaders.". **`agree`**. Affects: see note. Status: unreviewed.
  - `ravage_damage_change` — **Foreign Dignitaries Visit** [StageIII]. Rule: if_healthy = +2 total in lowest-numbered matching land per board; if_blighted = add 1 Fear Card to top of Fear Deck. Engine `event_cards.json` › `InvestedAristocracy`: "If the island is Healthy, Ravages do +2 Damage (total) in the lowest-numbered matching land on each board. If the island is Blighted, add 1 Fear Card to the top of the Fear Deck.". Wiki `event_invested_aristocracy.json`: "If the island is Healthy, Ravages do +2 Damage (total) in the lowest-numbered matching land on each board. If the island is Blighted, add 1 Fear Card to the top of the Fear Deck.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **LesserSpiritsImperiled**: none found.
- **LifesBalanceTilts**
  - `piece_health` — **Let Destruction Have Its Day** [Choice]. Rule: invader_dahan_health = -1; min = 1; duration = this turn. Engine `event_cards.json` › `LifesBalanceTilts`: "• On Each Board: Remove 1 {Beast}. Add 1 {Disease}. • For Each Board: Discard the top Minor Power. If it is {Fast}, add 1 {Blight}. • Invaders and {Dahan} have -1 Health (min. 1) this turn.". Wiki `event_lifes_balance_tilts.json`: " • On Each Board: Remove 1 Beasts. Add 1 Disease. • For Each Board: Discard the top Minor Power. If it is Fast, add 1 Blight. • Invaders and Dahan have -1 Health (min. 1) this turn.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
  - `piece_health` — **Fortify the Resilience of Life** [Choice]. Rule: invader_dahan_health = 1; duration = this turn. Engine `event_cards.json` › `LifesBalanceTilts`: "Cost: 4 Energy per Player, aided by {Animal}. • On Each Board: Add 1 {Beast}. Remove 1 {Disease}. • For Each Board: Discard the top Minor Power. If it is {Slow}, Remove 1 {Blight}. • Invaders and {Dahan} have +1 Health this turn.". Wiki `event_lifes_balance_tilts.json`: " Cost: 4 Energy per player, aided by Animal. • On Each Board: Add 1 Beasts. Remove 1 Disease. • For Each Board: Discard the top Minor Power. If it is Slow, Remove 1 Blight. • Invaders and Dahan have +1 Health this turn.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
- **MapmakersChartTheWild**
  - `explore_change` — **Mapmakers Chart the Wild** [StageI/StageII]. Rule: wilds_ignored = True; boards_without_wilds = +1 Explorer in each land successfully Explored. Engine `event_cards.json` › `MapmakersChartTheWild`: "Ignore {Wilds} during the Explore step. On Each Board without {Wilds}: Explore Actions add +1 {Explorer} (in each land successfully Explored).". Wiki `event_mapmakers_chart_the_wild.json`: "Ignore Wilds during the Explore Step. On Each Board without Wilds: Explore Actions add +1 Explorer (in each land successfully Explored).". **`agree`**. Affects: Explore odds and Explore forecast per land. Status: unreviewed.
  - `ravage_damage_change` — **Increasing Aggression** [StageIII]. Rule: per_land_plus = 1. Engine `event_cards.json` › `MapmakersChartTheWild`: "Invaders do +1 Damage (per land) when Ravaging.". Wiki `event_mapmakers_chart_the_wild.json`: "Invaders do +1 Damage (per land) when Ravaging.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **NoBraveryWithoutNumbers**
  - `skip_invader_action` — **No Bravery Without Numbers** [StageI/StageII]. Rule: action = ravage; unless = more Invaders than the current Terror Level. Engine `event_cards.json` › `NoBraveryWithoutNumbers`: "Invaders do not Ravage in lands unless there are more Invaders than the current Terror Level.". Wiki `event_no_bravery_without_numbers.json`: "Invaders do not Ravage in lands unless there are more Invaders than the current Terror Level.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
  - `skip_invader_action` — **Courage of the Crowd** [StageIII]. Rule: action = ravage; unless = more Invaders than the current Terror Level; per_land_plus = 2. Engine `event_cards.json` › `NoBraveryWithoutNumbers`: "Invaders do not Ravage in lands unless there are more Invaders than the current Terror Level. Ravage Actions do +2 Damage (per land).". Wiki `event_no_bravery_without_numbers.json`: "Invaders do not Ravage in lands unless there are more Invaders than the current Terror Level. Ravage actions do +2 Damage (per land).". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **NuminousCrisis**
  - `ravage_damage_change` — **Careful Defense** [Dahan]. Rule: mechanism = defend; amount = 2; where = lands with Dahan. Engine `event_cards.json` › `NuminousCrisis`: "When Invaders Ravage, if the land has {Dahan}, Defend 2.". Wiki `event_numinous_crisis.json`: "When Invaders Ravage, if the land has Dahan, Defend 2.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **Overconfidence**
  - `ravage_damage_change` — **Terror Breeds Aggression** [TerrorLevelII/TerrorLevelIII]. Rule: per_land_plus = 1 per 2 Fear Cards in the Earned pile. Engine `event_cards.json` › `Overconfidence`: "For every 2 Fear Cards currently in the Earned Fear Cards pile, Invaders do +1 Damage (per land) when Ravaging.". Wiki `event_overconfidence.json`: "For every 2 Fear Cards currently in the Earned Fear Cards pile, Invaders do +1 Damage (per land) when Ravaging.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
  - `ravage_damage_change` — **Coordinated Defense** [Dahan]. Rule: mechanism = defend; amount = 5; where = lands with Dahan and Presence. Engine `event_cards.json` › `Overconfidence`: "When Invaders Ravage, if the land has both {Dahan} and {Presence}, Defend 5.". Wiki `event_overconfidence.json`: "When Invaders Ravage, if the land has both Dahan and Presence, Defend 5.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **ProvincialSeat**
  - `extra_invader_action` — **Provincial Seat** [HealthyIsland]. Rule: action = build; slot = event; when = a land with City on each board. Engine `event_cards.json` › `ProvincialSeat`: "On Each Board: Choose a land with {City}. Build there.". Wiki `event_provincial_seat.json`: "On Each Board: Choose a land with at least 1 City. Build there.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `ravage_damage_change` — **Rally to the Defense of Distant Kin** [Dahan]. Rule: mechanism = defend; amount = 1 per Dahan; where = the gathered-into land on each board. Engine `event_cards.json` › `ProvincialSeat`: "On Each Board: Gather 1 {Dahan} into a land with {Dahan}. In that land, Defend 1 per {Dahan}.". Wiki `event_provincial_seat.json`: "On Each Board: Gather 1 Dahan into a land with Dahan. In that land, Defend 1 per Dahan.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **PullTogetherInAdversity**
  - `ravage_damage_change` — **Pull Together in Adversity** [TerrorLevelI/TerrorLevelII]. Rule: strife = ignore 1 Strife on each Invader this turn. Engine `event_cards.json` › `PullTogetherInAdversity`: "For the rest of this turn, ignore 1 {Strife} on each Invader. (This includes not removing that {Strife}.)". Wiki `event_pull_together_in_adversity.json`: "For the rest of this turn, ignore 1 Strife on each Invader. (This includes not removing that Strife.)". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
  - `cascade_change` — **Purge the Land with Salt and Fire** [TerrorLevelIII]. Rule: adds_blight = 1; cascade = False. Engine `event_cards.json` › `PullTogetherInAdversity`: "On Each Board in a land with 2 or more {Town}: Add 1 {Blight} without cascading. Add 2 {Badlands}. Remove all {Town}.". Wiki `event_pull_together_in_adversity.json`: "On Each Board in a land with 2 or more Towns: Add 1 Blight without cascading. Add 2 Badlands. Remove all Towns.". **`agree`**. Affects: cascade badge; blight runway; presence at risk. Status: unreviewed.
  - `ravage_damage_change` — **Careful Defense** [Dahan]. Rule: mechanism = defend; amount = 2; where = lands with Dahan. Engine `event_cards.json` › `PullTogetherInAdversity`: "When Invaders Ravage, if the land has {Dahan}, Defend 2.". Wiki `event_pull_together_in_adversity.json`: "When Invaders Ravage, if the land has Dahan, Defend 2.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **RelentlessOptimism**: none found.
- **RemnantsOfASpiritsHeart**
  - `piece_health` — **Leave its Strength with the Dahan** [Choice]. Rule: dahan_health = 10; invader_health_in_dahan_lands = 1; duration = this turn. Engine `event_cards.json` › `RemnantsOfASpiritsHeart`: "• {Dahan} have +10 Health this turn. • In lands with {Dahan}, Invaders have +1 Health this turn.". Wiki `event_remnants_of_a_spirits_heart.json`: " • Dahan have +10 Health this turn. • In lands with Dahan, Invaders have +1 Health this turn.". **`agree`**. Affects: Ravage ledger (counterattack kills); Build/victory blockers. Status: unreviewed.
  - `ravage_damage_change` — **With Patient Focus Craft a Great Warding** [Choice]. Rule: mechanism = defend; spirit_defend = -4 per land this turn; later = during one future Spirit Phase the players jointly choose: Defend 4 in all lands that turn (Q31). Engine `event_cards.json` › `RemnantsOfASpiritsHeart`: "Cost: 4 Energy per player, aided by {Earth}. • Defend from Spirits is 4 lower per land this turn (min. 0). • During one future Spirit Phase, players may jointly decide to grant Defend 4 to all lands that turn.". Wiki `event_remnants_of_a_spirits_heart.json`: " Cost: 4 Energy per player, aided by Earth. • Defend from Spirits is 4 lower per land this turn (min. 0) • During one future Spirit Phase, players may jointly decide to grant Defend 4 to all lands that turn. (There is a Reminder Card for this.)". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: sourced (2026-10-19, Q31; card text).
- **ResourcefulPopulace**
  - `ravage_damage_change` — **Strip the Land Bare** [TerrorLevelII/TerrorLevelIII]. Rule: per_town_plus = 1; after_blight = Remove 1 Explorer and 1 Town from the Ravaged land. Engine `event_cards.json` › `ResourcefulPopulace`: "{Town} each do +1 Damage when Ravaging. After a Ravage Action adds {Blight}, Remove 1 {Explorer} and 1 {Town} from the Ravaged land.". Wiki `event_resourceful_populace.json`: "Towns each do +1 Damage when Ravaging. After a Ravage Action adds Blight, Remove 1 Explorer and 1 Town from the Ravaged land.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **SeekNewFarmland**
  - `other` — **Voracious Consumption** [BlightedIsland]. Rule: after = Ravage Step; adds = 1 Blight to a land with City and no Blight, each board (preventable). Engine `event_cards.json` › `SeekNewFarmland`: "On Each Board: After the Ravage Step, add 1 {Blight} to a land with {City} and no {Blight}. Spirits may prevent this on any/all boards by Destroying 2 {Presence} from each board to be protected.". Wiki `event_seek_new_farmland.json`: "On Each Board: After the Ravage Step, add 1 Blight to a land with a City and no Blight. Spirits may prevent this on any/all boards by Destroying 2 Presence from each board to be protected.". **`agree`**. Affects: see note. Status: unreviewed.
  - `dahan_change` — **Engage on Their Own Terms** [Dahan]. Rule: dahan_damage_each = 1; counterattack_timing = BeforeInvaders. Engine `event_cards.json` › `SeekNewFarmland`: "During Ravage, {Dahan} only do 1 Damage each, but deal Damage before Invaders.". Wiki `event_seek_new_farmland.json`: "During Ravage, Dahan only do 1 Damage each, but deal Damage before Invaders.". **`agree`**. Affects: Ravage ledger (Dahan damage, counterattack). Status: unreviewed.
- **SmallerPortsSpringUp**
  - `extra_invader_action` — **Extensive Building** [StageIII]. Rule: action = build; slot = event; when = a land with Invaders not matching a Build Card, each board. Engine `event_cards.json` › `SmallerPortsSpringUp`: "On Each Board: Build in a land with Invaders not matching a Build Card.". Wiki `event_smaller_ports_spring_up.json`: "On Each Board: Build in a land with Invaders not matching a Build Card.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `skip_invader_action` — **Stricken** [Disease/Strife]. Rule: action = ravage; where = lands with Disease/Strife. Engine `event_cards.json` › `SmallerPortsSpringUp`: "In lands with {Disease}/{Strife}, Invaders skip Ravage Actions.". Wiki `event_smaller_ports_spring_up.json`: "In lands with Disease/Strife, Invaders skip Ravage Actions.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **SprawlContainedByTheWilds**
  - `extra_invader_action` — **Sprawl Contained by the Wilds** [HealthyIsland]. Rule: action = build; slot = event; when = land with most Town/City (min 1) on each board; also adds 1 Wilds to a land without Invaders. Engine `event_cards.json` › `SprawlContainedByTheWilds`: "On Each Board: Add 1 {Wilds} to a land without Invaders. Build in the land with the most {Town}/{City} (min 1).". Wiki `event_sprawl_contained_by_the_wilds.json`: "On Each Board: Add 1 Wilds to a land without Invaders. Build in the land with the most Towns/Cities (min. 1).". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `other` — **Focus Your Will Towards Wholeness** [Choice]. Rule: timing = next turn, that Spirit's Powers are all Slow and cannot be made Fast. Engine `event_cards.json` › `SprawlContainedByTheWilds`: "Add 1 of your {DestroyedPresenceNew} to one of your lands. Next turn, all your Powers are {Slow} and cannot be made {Fast}.". Wiki `event_sprawl_contained_by_the_wilds.json`: "—". **`agree`**. Affects: see note. Status: unreviewed.
  - `dahan_change` — **Lay Ambushes** [Dahan]. Rule: counterattack_timing = Simultaneous. Engine `event_cards.json` › `SprawlContainedByTheWilds`: "During Ravage, {Dahan} deal Damage at the same time as Invaders.". Wiki `event_sprawl_contained_by_the_wilds.json`: "During Ravage, Dahan deal Damage at the same time as Invaders.". **`agree`**. Affects: Ravage ledger (Dahan damage, counterattack). Status: unreviewed.
- **TemporaryTruce**
  - `dahan_change` — **Temporary Truce** [TerrorLevelI]. Rule: in_lands_with_dahan = 1 Dahan and 1 Town/City do not participate in Ravage. Engine `event_cards.json` › `TemporaryTruce`: "In lands with {Dahan}: 1 {Dahan} and 1 {Town}/{City} do not participate in Ravages. (They neither take nor deal Damage.)". Wiki `event_temporary_truce.json`: "In lands with Dahan: 1 Dahan and 1 Town/City do not participate in Ravages. (They neither take nor deal Damage.)". **`agree`**. Affects: Ravage ledger (Dahan damage, counterattack). Status: unreviewed.
  - `skip_invader_action` — **Temporary Caution** [TerrorLevelII/TerrorLevelIII]. Rule: action = ravage; which = 1 Ravage Action per board in a land where Dahan outnumber Town/City. Engine `event_cards.json` › `TemporaryTruce`: "During the Ravage Step, On Each Board: Skip 1 Ravage Action in a land where {Dahan} outnumber {Town}/{City}.". Wiki `event_temporary_truce.json`: "During the Ravage Step, On Each Board: Skip 1 Ravage Action in a land where Dahan outnumber Towns/Cities.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
- **TheFrontierCalls**
  - `explore_change` — **The Frontier Calls** [HealthyIsland]. Rule: plus_explorer = 1; where = lands without Town/City. Engine `event_cards.json` › `TheFrontierCalls`: "Explore Actions add +1 {Explorer} to lands without {Town}/{City}.". Wiki `event_the_frontier_calls.json`: "Explore Actions add +1 Explorer to lands without Towns/Cities.". **`agree`**. Affects: Explore odds and Explore forecast per land. Status: unreviewed.
- **TheStrugglesOfGrowth**
  - `cascade_change` — **Blight Deepens** [BlightedIsland]. Rule: adds_blight = 1; cascade = False; where = a land with Blight, each board (preventable). Engine `event_cards.json` › `TheStrugglesOfGrowth`: "On Each Board: Add 1 {Blight} to a land with {Blight}, but do not cascade. Spirits may prevent this on any/all boards by Destroying 1 {Presence} from each board to be protected.". Wiki `event_the_struggles_of_growth.json`: "On Each Board: Add 1 Blight to a land with Blight, but do not cascade. Spirits may prevent this on any/all boards by Destroying 1 Presence from each board to be protected.". **`agree`**. Affects: cascade badge; blight runway; presence at risk. Status: unreviewed.
  - `build_change` — **Irregular Outbreaks** [Disease]. Rule: disease_prevents_build = False; duration = this Invader Phase. Engine `event_cards.json` › `TheStrugglesOfGrowth`: "Ignore {Disease} during Builds this Invader Phase. 1 {Fear} per board with {Disease}.". Wiki `event_the_struggles_of_growth.json`: "Ignore Disease during Builds this Invader Phase. 1 Fear per board with Disease.". **`agree`**. Affects: Build forecast. Status: unreviewed.
  - `ravage_damage_change` — **Careful Defense** [Dahan]. Rule: mechanism = defend; amount = 2; where = lands with Dahan. Engine `event_cards.json` › `TheStrugglesOfGrowth`: "When Invaders Ravage, if the land has {Dahan}, Defend 2.". Wiki `event_the_struggles_of_growth.json`: "When Invaders Ravage, if the land has Dahan, Defend 2.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **ThrivingTrade**: none found.
- **WoundedLandsAttractExplorers**: none found.

*Nature Incarnate events*

- **AccumulatedDevastation**
  - `ravage_damage_change` — **Accumulated Devastation** [HealthyIsland]. Rule: when = first Ravage Action on each board; land_damage_plus = Invader Stage; dahan_damage_plus = 0. Engine `event_cards.json` › `AccumulatedDevastation`: "On Each Board: During the first Ravage Action, increase the total Damage Invaders deal to the land by the Invader Stage. (Damage to {Dahan} in that land is not increased.)". Wiki `event_accumulated_devastation.json`: "On Each Board: During the first successful Ravage Action (in one land) this Invader Phase, increase the total Damage ravaging Invaders deal to each land (but not to Dahan) by the current Invader Stage. Do this for at most one Ravage Action per board (for one Ravage Action in one land).". **`conflict`** — Engine 'the first Ravage Action'; Wiki 'the first successful Ravage Action (in one land)'.. Resolution: the printed text (Wiki transcription) applies — the first *successful* Ravage Action per board; no errata; per-land bonus, so JE p.7 Ravage Timing applies (FAQ .7w4gec1; NQ-H, NQ-L). Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: reviewed by Brett (2026-10-19, NQ-H; printed card governs unless explicit errata (Brett's rule, 2026-10-19)).
  - `isolate` — **Territorial Beasts** [Beast]. Rule: where = a land with Beast, each board. Engine `event_cards.json` › `AccumulatedDevastation`: "On Each Board: Isolate a land with {Beast}. 1 {Fear} if Invaders are present.". Wiki `event_accumulated_devastation.json`: "On Each Board: Isolate a land with Beasts. 1 Fear if Invaders are present.". **`agree`**. Affects: Explore eligibility (isolated land neither explores nor is a source). Status: unreviewed.
- **AnOminousDawn**: none found.
- **EtherealConjunction**
  - `build_change` — **Irregular Outbreaks** [Disease]. Rule: disease_prevents_build = False; duration = this Invader Phase. Engine `event_cards.json` › `EtherealConjunction`: "{Disease} doesn't prevent Builds this Invader Phase. 1 {Fear} per board with {Disease}.". Wiki `event_ethereal_conjunction.json`: "Disease doesn't prevent Builds this Invader Phase. 1 Fear per board with Disease.". **`agree`**. Affects: Build forecast. Status: unreviewed.
- **FarOffWarsTouchTheIsland**
  - `fear_deck` — **Far-Off Wars Touch the Island** [Choice]. Rule: adds = 1 Fear Card to the top of the Fear Deck. Engine `event_cards.json` › `FarOffWarsTouchTheIsland`: "Invaders from a different, distant land launch an assault upon the island while the Invaders here rally together in a wave of nationalism. Defending the island with them might come at too high a price. • Add a Fear Card to the top of the Fear Deck.". Wiki `event_far-off_wars_touch_the_island.json`: "Invaders from a different, distant land launch an assault upon the island while the Invaders here rally together in a wave of nationalism. Defending the island with them might come at too high a price. <div> * Add a Fear Card to the top of the Fear Deck. * On Each Board, in the land with the most Towns/Cities, choose (for that board) either:</div> ENCOURAGE THE DESTRUCTION <div> * 4 Fear. Destroy half of Towns (round up) and half of Cities (round up). Add 2 Blight without cascading.</div> BLUNT THE OFFENSIVE <div> * Destroy 1 Town but do not gain Fear.</div>". **`agree`** — Engine text is in the section's mainBody field.. Affects: fear runway; Terror Level timing. Status: unreviewed.
  - `cascade_change` — **Encourage the Destruction** [Choice]. Rule: adds_blight = 2; cascade = False. Engine `event_cards.json` › `FarOffWarsTouchTheIsland`: "• 4 {Fear}. Destroy half of {Town} (round up) and half of {City} (round up). Add 2 {Blight} without cascading.". Wiki `event_far-off_wars_touch_the_island.json`: " <div> * 4 Fear. Destroy half of Towns (round up) and half of Cities (round up). Add 2 Blight without cascading.</div>". **`agree`**. Affects: cascade badge; blight runway; presence at risk. Status: unreviewed.
- **FocusedFarming**: none found.
- **InfluxOfSettlers**
  - `skip_invader_action` — **Lethargy and Degeneration** [BlightedIsland]. Rule: action = ravage; which = first Ravage Action (one land) on each board, unless prevented. Engine `event_cards.json` › `InfluxOfSettlers`: "On Each Board: Skip the first Ravage Action (in one land). Add 1 {Blight} to a land with {Town}/{City}. Spirits may prevent either/both effects on any/all boards (choose per board) by Destroying 2 {Presence} from each board to be protected.". Wiki `event_influx_of_settlers.json`: "On Each Board: Skip the first Ravage Action (in one land). Add 1 Blight to a land with Towns/Cities. Spirits may prevent either/both effects on any/all boards (choose per board) by Destroying 2 Presence from each board to be protected.". **`agree`**. Affects: invader forecast (skipped lands); counterattack ledger (no counterattack on a skipped Ravage). Status: unreviewed.
  - `ravage_damage_change` — **Network of Support and Trade** [Dahan]. Rule: mechanism = defend; amount = 1 + 1 per adjacent Dahan; where = lands with Dahan. Engine `event_cards.json` › `InfluxOfSettlers`: "When Invaders Ravage, if the land has {Dahan}, Defend 1 + Defend 1 per adjacent {Dahan}.". Wiki `event_influx_of_settlers.json`: "When Invaders Ravage, if the land has Dahan, Defend 1 + Defend 1 per adjacent Dahan.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **SearchForUnclaimedLand**
  - `extra_invader_action` — **Search For Unclaimed Lands** [HealthyIsland]. Rule: action = explore; slot = event; when = land with fewest Invaders/Dahan on each board, no source required. Engine `event_cards.json` › `SearchForUnclaimedLand`: "On Each Board: Explore in the land with the fewest total Invaders/{Dahan} (no source required).". Wiki `event_search_for_unclaimed_land.json`: "—". **`engine-only`** — No Wiki cache file for this event.. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `ravage_damage_change` — **Widespread Hunting** [BlightedIsland]. Rule: total_plus = 2; where = Town/City Ravage with no Beast (else Destroy 1 Beast); limit = once per board. Engine `event_cards.json` › `SearchForUnclaimedLand`: "When {Town}/{City} Ravage in a land: • If no {Beast} are present, Invaders do +2 Damage (total). • Otherwise, Destroy 1 {Beast}. Do this at most once per board (for one Ravage Action in one land).". Wiki `event_search_for_unclaimed_land.json`: "When Towns/Cities Ravage in a land:<div> * If no Beasts are present, Invaders do +2 Damage (total). * Otherwise, Destroy 1 Beasts.</div>Do this at most once per board (for one Ravage Action in one land).". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **TerrorSpikesUpwards**
  - `ravage_damage_change` — **Final Harvest** [TerrorLevelIII]. Rule: per_city_plus = 3; after_each_ravage = Remove 1 Town/City. Engine `event_cards.json` › `TerrorSpikesUpwards`: "Each {City} does +3 Damage during Ravage Actions. After each Ravage Action, Remove 1 {Town}/{City} from that land.". Wiki `event_terror_spikes_upwards.json`: "Each City does +3 Damage during Ravage Actions. After each Ravage Action, Remove 1 Town/City from that land.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.
- **VisionsOutOfTime**
  - `extra_invader_action` — **Head Off the Problems Yourself** [Choice]. Rule: action = ravage; slot = Omen token; when = after each Ravage Step starting next turn, at each Omen; then remove that Omen. Engine `event_cards.json` › `VisionsOutOfTime`: "Add 1 Omen token to the land without Omens that has the most Invaders (min. 1). After each Ravage Step, starting next turn, at each Omen: 2 {Fear}. Invaders Ravage. Remove 1 Omen.". Wiki `event_visions_out_of_time.json`: " Add 1 Omen token (Scenario Marker) to the land without Omens that has the most Invaders (min. 1). After the Ravage Step next turn, at each Omen: 2 Fear. Invaders Ravage. Remove 1 Omen.". **`agree`**. Affects: invader forecast (Ravage/Build/Explore land lists); land-tempo badges. Status: unreviewed.
  - `ravage_damage_change` — **Strength From Trade** [Dahan]. Rule: mechanism = defend; amount = 2; where = lands with Dahan; plus = +1 Damage in lands with Town/City and no Dahan. Engine `event_cards.json` › `VisionsOutOfTime`: "Lands with {Dahan} have Defend 2. In lands with {Town}/{City} and no {Dahan}, Ravage Actions do +1 Damage.". Wiki `event_visions_out_of_time.json`: "Lands with Dahan have Defend 2. In lands with Towns/Cities and no Dahan, Ravage Actions do +1 Damage.". **`agree`**. Affects: Ravage ledger (damage, Defend, Dahan lost, blight). Status: unreviewed.

### 5.4 Fear cards (50): effect families

Fear Cards resolve in the Fear step, after the Event and before Ravage, at the current Terror Level (Wiki Sequence_of_Play@12144 §2). So a revealed card's skip, Defend and Isolate effects apply to this turn's Ravage, Build and Explore. Families are regex matches on engine `fear_cards.json` `body` (one string per Terror Level). Every pipeline-family text was compared with the repo fetcher's Wiki fear cache (50 of 50 cards found): all similarity ≥ 0.86 (`agree`).

| Family | Card × Terror Level matches | Cards |
|---|---|---|
| `remove_destroy_invaders` | 46 | 20: AngryMobs@TL2, AngryMobs@TL3, BeliefTakesRoot@TL3, DahanAttack@TL1, DepartTheDangerousLand@TL1, DepartTheDangerousLand@TL2, DepartTheDangerousLand@TL3, Depopulation@TL2, Depopulation@TL3, EmigrationAccelerates@TL1, EmigrationAccelerates@TL2, EmigrationAccelerates@TL3, FearOfTheUnseen@TL1, FearOfTheUnseen@TL2, FearOfTheUnseen@TL3, FleeFromDangerousLands@TL2, FleeFromDangerousLands@TL3, FleeThePestilentLand@TL1, FleeThePestilentLand@TL2, FleeThePestilentLand@TL3, Isolation@TL1, Isolation@TL2, Isolation@TL3, MimicTheDahan@TL1, Restlessness@TL3, Scapegoats@TL1, Scapegoats@TL2, Scapegoats@TL3, SeekSafety@TL3, SenseOfDread@TL1, SenseOfDread@TL2, SenseOfDread@TL3, TallTalesOfSavagery@TL1, TallTalesOfSavagery@TL2, TallTalesOfSavagery@TL3, TooManyMonsters@TL1, TooManyMonsters@TL2, TooManyMonsters@TL3, TradeSuffers@TL2, TradeSuffers@TL3, Unsettled@TL1, Unsettled@TL2, Unsettled@TL3, WaryOfTheInterior@TL1, WaryOfTheInterior@TL2, WaryOfTheInterior@TL3 |
| `skip_explore` | 9 | 4: AvoidTheDahan@TL1, ExplorersAreReluctant@TL1, ExplorersAreReluctant@TL2, ExplorersAreReluctant@TL3, PanickedByWildBeasts@TL2, PanickedByWildBeasts@TL3, Quarantine@TL1, Quarantine@TL2, Quarantine@TL3 |
| `skip_build` | 15 | 8: AvoidTheDahan@TL2, AvoidTheDahan@TL3, ImmigrationSlows@TL1, ImmigrationSlows@TL2, ImmigrationSlows@TL3, OverseasTradeSeemsSafer@TL2, OverseasTradeSeemsSafer@TL3, PanickedByWildBeasts@TL2, PanickedByWildBeasts@TL3, Quarantine@TL3, SupplyChainsAbandoned@TL2, SupplyChainsAbandoned@TL3, TradeSuffers@TL1, Unsettled@TL2, Unsettled@TL3 |
| `defend` | 23 | 9: BeliefTakesRoot@TL1, BeliefTakesRoot@TL2, BesetByManyTroubles@TL1, BesetByManyTroubles@TL2, BesetByManyTroubles@TL3, DahanGainTheEdge@TL1, DahanGainTheEdge@TL2, DahanGainTheEdge@TL3, DahanOnTheirGuard@TL1, DahanOnTheirGuard@TL2, DahanOnTheirGuard@TL3, DauntedByTheDahan@TL2, DauntedByTheDahan@TL3, Demoralized@TL1, Demoralized@TL2, Demoralized@TL3, OverseasTradeSeemsSafer@TL1, OverseasTradeSeemsSafer@TL2, OverseasTradeSeemsSafer@TL3, PlanForDeparture@TL2, PlanForDeparture@TL3, SpreadingTimidity@TL2, SpreadingTimidity@TL3 |
| `strife` | 33 | 11: BesetByManyTroubles@TL1, BesetByManyTroubles@TL2, BesetByManyTroubles@TL3, CivilUnrest@TL1, CivilUnrest@TL2, CivilUnrest@TL3, DahanThreaten@TL1, DahanThreaten@TL2, DahanThreaten@TL3, Discord@TL1, Discord@TL2, Discord@TL3, NervesFray@TL1, NervesFray@TL2, NervesFray@TL3, Panic@TL1, Panic@TL2, Panic@TL3, PanickedByWildBeasts@TL1, PanickedByWildBeasts@TL2, PanickedByWildBeasts@TL3, StrugglesOverFarmland@TL1, StrugglesOverFarmland@TL2, StrugglesOverFarmland@TL3, TheologicalStrife@TL1, TheologicalStrife@TL2, TheologicalStrife@TL3, Unrest@TL1, Unrest@TL2, Unrest@TL3, Unsettled@TL1, Unsettled@TL2, Unsettled@TL3 |
| `invader_damage_down` | 8 | 3: CommunitiesInDisarray@TL1, CommunitiesInDisarray@TL2, CommunitiesInDisarray@TL3, DauntedByTheDahan@TL1, DauntedByTheDahan@TL2, DauntedByTheDahan@TL3, DistractedByLocalTroubles@TL2, DistractedByLocalTroubles@TL3 |
| `push_gather_invaders` | 15 | 7: DahanEnheartened@TL1, FleeFromDangerousLands@TL1, PlanForDeparture@TL1, PlanForDeparture@TL2, PlanForDeparture@TL3, Restlessness@TL1, Restlessness@TL2, Retreat@TL1, Retreat@TL2, Retreat@TL3, SeekCompany@TL1, SeekCompany@TL2, SeekCompany@TL3, SeekSafety@TL1, SeekSafety@TL2 |
| `isolate` | 7 | 3: DauntedByTheDahan@TL3, SpreadingTimidity@TL1, SpreadingTimidity@TL2, SpreadingTimidity@TL3, SupplyChainsAbandoned@TL1, SupplyChainsAbandoned@TL2, SupplyChainsAbandoned@TL3 |
| `card_movement` | 3 | 2: ExplorersAreReluctant@TL2, ExplorersAreReluctant@TL3, ImmigrationSlows@TL2 |
| `skip_ravage` | 5 | 3: PanickedByWildBeasts@TL3, Quarantine@TL3, TreadCarefully@TL1, TreadCarefully@TL2, TreadCarefully@TL3 |

Pipeline-specific Fear texts (engine, verbatim):
- **Avoid the Dahan** (`AvoidTheDahan`): TL1: Skip all Explore Actions in lands with at least 2 {Dahan}. | TL2: Skip all Build Actions in lands where {Dahan} outnumber {Town}/{City} (including lands with no {Town}/{City}). | TL3: Skip Build Actions in lands with {Dahan}.
- **Explorers are Reluctant** (`ExplorersAreReluctant`): TL1: During the next normal Explore, skip the lowest-numbered land matching the Invader Card on each board. | TL2: Skip the next normal Explore. During the next Invader Phase, draw an additional Explore Card. | TL3: Skip the next normal Explore, but still reveal a card. (Perform the {Escalation} if relevant.) Cards shift left as usual.
- **Immigration Slows** (`ImmigrationSlows`): TL1: During the next normal Build, skip the lowest-numbered land matching the Invader Card on each board. | TL2: Skip the next normal Build. The Build Card remains in place instead of shifting left. | TL3: Skip the next normal Build. The Build Card shifts left as usual.
- **Quarantine** (`Quarantine`): TL1: Explore does not affect Coastal lands. | TL2: Explore does not affect Coastal lands. Lands with {Disease} are not a source of Invaders when Exploring. | TL3: Explore does not affect Coastal lands. Invaders do not act in lands with {Disease}.
- **Panicked by Wild Beasts** (`PanickedByWildBeasts`): TL1: Each player adds 1 {Strife} in a land with or adjacent to {Beast}. | TL2: Each player adds 1 {Strife} in a land with or adjacent to {Beast}. Invaders skip their normal Explore and Build in lands with {Beast}. | TL3: Each player adds 1 {Strife} in a land with or adjacent to {Beast}. Invaders skip all normal Actions in lands with {Beast}.
- **Overseas Trade Seems Safer** (`OverseasTradeSeemsSafer`): TL1: Defend 3 in all Coastal lands. | TL2: Defend 6 in all Coastal lands. Skip Build Actions in Coastal lands that would add {City}. | TL3: Defend 9 in all Coastal lands. Skip all Build Actions in Coastal lands.
- **Trade Suffers** (`TradeSuffers`): TL1: Skip all Build Actions in lands with {City}. | TL2: Each player may Downgrade 1 {Town} in a Coastal land. | TL3: Each player may Downgrade 1 {Town}/{City} in a Coastal land.
- **Tread Carefully** (`TreadCarefully`): TL1: Each player may choose a land with {Dahan} or adjacent to at least 5 {Dahan}. Invaders do not Ravage there this turn. | TL2: Each player may choose a land with {Dahan} or adjacent to at least 3 {Dahan}. Invaders do not Ravage there this turn. | TL3: Each player may choose a land with {Dahan} or adjacent to {Dahan}. Invaders do not Ravage there this turn.
- **Supply Chains Abandoned** (`SupplyChainsAbandoned`): TL1: On Each Board: Isolate one land. | TL2: On Each Board: Isolate one land. If {Town}/{City} are present, skip all Build Actions (in that land). | TL3: On Each Board: Isolate two lands. In each of those lands, if {Town}/{City} are present, skip all Build Actions (in that land).
- **Unsettled** (`Unsettled`): TL1: On Each Board: Choose a land with {Beast}/{Strife}/{Dahan}. Downgrade 1 {Town}/{City} there. | TL2: On Each Board: Choose a land with {Beast}/{Strife}/{Dahan}. Downgrade 1 {Town}/{City} there or skip the next Build Action there (this turn). | TL3: On Each Board: Choose a land with {Beast}/{Strife}/{Dahan}. Remove 1 Invader there or skip the next Build Action there (this turn).
- **Spreading Timidity** (`SpreadingTimidity`): TL1: Each player chooses a land to Isolate. | TL2: Each player chooses a different land to Isolate. Also, Defend 2 in those lands. | TL3: Each player chooses a different land to Isolate. Also, Defend 4 in those lands.
- **Daunted by the Dahan** (`DauntedByTheDahan`): TL1: 1 {Fear} per board with both Invaders and {Dahan}. Invaders do -6 Damage to {Dahan} (per land) during Ravage. | TL2: 1 {Fear} per board with both Invaders and {Dahan}. Lands with {Dahan} have Defend 3. Invaders do -6 Damage to {Dahan} (per land) during Ravage. | TL3: 1 {Fear} per board with both Invaders and {Dahan}. Lands with {Dahan} have Defend 3. Invaders do -6 Damage to {Dahan} (per land) during Ravage. Isolate all lands with {Dahan}.
- **Distracted by Local Troubles** (`DistractedByLocalTroubles`): TL1: On Each Board, in a land matching a Ravage Card: 1 Damage. | TL2: Invaders do -1 Damage per Damage they have taken. On Each Board, in a land matching a Ravage Card: 1 Damage each to up to 2 Invaders. | TL3: Invaders do -1 Damage per Damage they have taken. On Each Board, in two lands matching a Ravage Card: 2 Damage (per land).

**Distracted by Local Troubles, TL2/TL3 (Q32).** "Invaders do -1 Damage per Damage they have taken" means −1 per point of Damage each Invader has already taken, not −1 per Invader; the engine's `PerDamageTaken` flag is this. Wiki fear cache: "Invaders each do -1 Damage per Damage they have taken." The companion's forecast (`rules.rs` `ravage_adjustment`) now reads that flag and multiplies by the Damage each Invader has taken (test `minus_one_per_damage_taken_floors_each_invader`); before the fix it applied −1 per Invader, and Test 1 Turn 6 matched only because both Explorers had taken Damage. Status: sourced (2026-10-19, Q32; card text).

### 5.5 Blight cards (25): flipped effects that touch the pipeline

Engine `blight_cards.json` `flippedEffectBody`; Wiki = repo fetcher blight cache (revision ids not recorded). The engine counts Blight per starting board (`flippedNumberPerStartingBoard`); on the box it is per player, and boards are counted instead of players only with an extra board; the +1 errata applies at setup, not on a flip (Q12, Brett 2026-10-19). Tipping Point is dropped (NI p.5; Q15).

- `ravage_blight_change` — **All Things Weaken** (`AllThingsWeaken`, flipped 3 per board): "Ongoing, starting next turn: Invaders, {Dahan}, and the land have -1 Health (min. 1). When you add {Blight}, it Destroys all {Presence}/{Beast} in that land and 1 {Presence} (total) in an adjacent land." Wiki: "Ongoing, starting next turn: Invaders and Dahan have -1 Health (min. 1). The land takes Blight on 1 less Damage (normally 1). When you add Blight, it Destroys all Presence/Beasts in that land and 1 Presence (total) in an adjacent land." **`conflict`** (similarity 0.80) — Engine 'the land have -1 Health'; Wiki 'The land takes Blight on 1 less Damage'. Same wording split as NQ-I.. Resolution: "Invaders, Dahan, and the land have -1 Health" is right (NI p.14); land-Health modifiers add together, min. 1 (NQ-I). Status: sourced (2026-10-19, NQ-I; NI rulebook p.14).
- `ravage_damage_change` — **Intensifying Exploitation** (`IntensifyingExploitation`, flipped 5 per board): "Ongoing, starting next turn: During Ravage Actions, Invaders deal +2 Damage (per land)." Wiki: "Ongoing, starting next turn: During Ravage Actions, Invaders deal +2 Damage (per land)." **`agree`** (similarity 1.00). Status: unreviewed.
- `extra_invader_action` — **Thriving Crops** (`ThrivingCrops`, flipped 2 per board): "Immediately: On Each Board, Build in 3 lands. (Build Actions in lands without Invaders normally Build 1 {Town}.)" Wiki: "Immediately: On Each Board, Build in 3 lands. (Build Actions in lands without Invaders normally Build 1 Town.)" **`agree`** (similarity 1.00). Status: unreviewed.
- `other` — **Promising Farmlands** (`PromisingFarmlands`, flipped 4 per board): "Immediately, on each board: Add 1 {Town} and 1 {City} to an Inland land with no {Town}/{City}." Wiki: "Immediately, on each board: Add 1 Town and 1 City to an Inland land with no Towns/Cities." **`agree`** (similarity 0.99). Status: unreviewed.
- `other` — **Thriving Communities** (`ThrivingCommunities`, flipped 4 per board): "Immediately, on each board: In 4 different lands with {Explorer}/{Town}, Replace 1 {Town} with 1 {City} or Replace 1 {Explorer} with 1 {Town}." Wiki: "Immediately, on each board: In 4 different lands with Explorer/Town, Replace 1 Town with 1 City or Replace 1 Explorer with 1 Town." **`agree`** (similarity 0.99). Status: unreviewed.
- `other` — **Unnatural Proliferation** (`UnnaturalProliferation`, flipped 3 per board): "Immediately: Each Spirit adds 1 {Presence} to a land with their {Presence}. On Each Board: Add 1 {Dahan} to a land with {Dahan}, and 2 {City} to the land with fewest {Town}/{City} (min. 1)." Wiki: "Immediately: Each Spirit adds 1 Presence to a land with their Presence. On Each Board: Add 1 Dahan to a land with Dahan, and 2 City to the land with the fewest Town/City (min. 1)." **`agree`** (similarity 0.98). Status: unreviewed.
- `other` — **Untended Land Crumbles** (`UntendedLandCrumbles`, flipped 4 per board): "Each Invader Phase: On Each Board: Add 1 {Blight} to a land adjacent to {Blight}. Spirits may prevent this on any/all boards; each board to be protected requires jointly paying 3 Energy or Destroying 1 {Presence} from that board." Wiki: "At the start of each Invader Phase, On Each Board: Add 1 Blight to a land adjacent to Blight. Spirits may prevent this on any/all boards; each board to be protected requires jointly paying 3 Energy or Destroying 1 Presence from that board." **`agree`** (similarity 0.96). Status: unreviewed.
- `other` — **Strong Earth Shatters Slowly** (`StrongEarthShattersSlowly`, flipped 2 per board): "Immediately: Each player adds 1 {Blight} (from this card) to a land adjacent to {Blight}." Wiki: "Immediately: Each player adds 1 Blight (from this card) to a land adjacent to Blight." **`agree`** (similarity 1.00). Status: unreviewed.
- `other` — **Disintegrating Ecosystem** (`DisintegratingEcosystem`, flipped 5 per player, per board with an extra board): "Immediately, on each board: Destroy 1 {Beast}, then add 1 {Blight} to a land with {Town}/{City}." Wiki: "Immediately, on each board: Destroy 1 Beasts, then add 1 Blight to a land with Towns/Cities." **`agree`** (similarity 0.99). Brett's physical card (Q13): 5 Blight per player on the Blighted side; the added Blight comes from the card's new pool, and Blight removed from the board returns to the card (FAQ). Status: reviewed by Brett (2026-10-19, Q13; physical card).
- `fear_deck` — **Invaders Find the Land to Their Liking** (`InvadersFindTheLandToTheirLiking`, flipped 2 per board): "Immediately: If the Terror Level is I / II / III, increase the requirement to earn a Fear Card by 1 / 1.5 / 2 {Fear} per player. (Round down at Terror Level II.)" Wiki: "Immediately: If the Terror Level is I / II / III, add 1 / 1.5 / 2 Fear Markers per player to the Fear pool. (Round down at Terror Level II.)" **`agree`** (similarity 0.75) — Read manually: raising the Fear needed per card and adding Fear Markers to the Fear pool are the same rule.. Status: unreviewed.

### 5.6 Power cards (321): recurring effect families

Counts from engine `power_cards.json` text (body, threshold options, conditional and set-aside text), by `powerType`; regex families, so a card can count in several. Wiki not compared for this family view. Speed from the engine `speed` field (true = Fast; verified against the Wiki for all 63 base-spirit innates, see 3). Steam's Test 1 pool: Minors 100 (Growth Through Sacrifice dropped, NI p.5; Roiling Bog and Snagging Thorn replaces it), Majors 78 (Q15).

| Family | Minor | Major | Unique | Total | Fast / Slow |
|---|---|---|---|---|---|
| skip or prevent an Invader Action | 2 | 9 | 7 | 18 | 18 / 0 |
| Isolate | 5 | 4 | 2 | 11 | 11 / 0 |
| Defend | 17 | 10 | 13 | 40 | 40 / 0 |
| Push | 26 | 13 | 41 | 80 | 35 / 45 |
| Gather | 17 | 10 | 18 | 45 | 22 / 23 |
| adds Wilds | 6 | 8 | 6 | 20 | 7 / 13 |
| adds Disease | 5 | 5 | 1 | 11 | 3 / 8 |
| adds Strife | 7 | 5 | 4 | 16 | 4 / 12 |
| adds Badlands | 6 | 6 | 5 | 17 | 3 / 14 |
| Ravage participation | 1 | 0 | 3 | 4 | 4 / 0 |
| Dahan damage timing | 0 | 1 | 1 | 2 | 2 / 0 |
| cascade prevention | 1 | 0 | 0 | 1 | 1 / 0 |
| forces an Invader Action | 0 | 1 | 1 | 2 | 1 / 1 |
| Invader Deck manipulation | 0 | 0 | 1 | 1 | 1 / 0 |

*skip or prevent an Invader Action*: A Dreadful Tide of Scurrying Flesh (Unique, Fast), A Year of Perfect Stillness (Unique, Fast), Call to Trade (Minor, Fast), Dream of the Untouched Land (Major, Fast), Exaltation of the Incandescent Sky (Major, Fast), Indomitable Claim (Major, Fast), Infestation of Venomous Spiders (Major, Fast), Irresistible Call (Major, Fast), Paralyzing Fright (Major, Fast), Peace of the Nighttime Sky (Unique, Fast), Prey on the Builders (Unique, Fast), Resounding Footfalls Sow Dismay (Unique, Fast), Rites of the Land's Rejection (Minor, Fast), Sleep and Never Waken (Major, Fast), Solidify Echoes of Majesty Past (Major, Fast), Strike Low with Sudden Fevers (Unique, Fast), Sweltering Exhaustion (Unique, Fast), Trees Radiate Celestial Brilliance (Major, Fast)

*cascade prevention*: Entrap the Forces of Corruption (Minor, Fast)

*Dahan damage timing*: Voice of Command (Major, Fast), Words of Warning (Unique, Fast)

*forces an Invader Action*: Blur the Arc of Years (Unique, Fast), Manifest Incarnation (Major, Slow)

*Invader Deck manipulation*: The Past Returns Again (Unique, Fast)

*Ravage participation*: Exhale Confusion and Delirium (Unique, Fast), Roiling Bog and Snagging Thorn (Minor, Fast), Terrifying Rampage (Unique, Fast), Unnerving Pall (Unique, Fast)

## 6. Interactions across layers

**X-1 · Two Adversaries at once**

- **Rule:** Use all changes from both, except: Fear Deck = 3 per Terror Level + both adversaries' changes; the leading Adversary's Escalation applies normally and the supporting one's applies on all Stage III cards (France/Sweden: random terrain); Invader Deck: all supporting instructions in order, then all leading instructions; skip impossible instructions; ambiguous removal takes the top-most applicable card, ambiguous move the bottom-most; Brandenburg-Prussia L2 reads "Move the bottom-most Stage III card just below the bottom-most Stage I card"; Scotland: another adversary's setup pieces bound for a Coastal land other than #2 go to an adjacent Inland land (Q19 wording; this register first summarized it as a City); France L2+: Town pool +1 per player per level of the other Adversary.
- **Engine:** `Constants.CombinedAdversaryDifficultyFactor = 0.625` (engine_state_model.md Q1). Combination logic not read.
- **Wiki:** Adversary@12228, "Combining Two Adversaries" (all of the above, quoted in substance: "first follow all of the instructions on the supporting Adversary, in order, and then follow all of the instructions on the leading Adversary, in order"; "If it is ever ambiguous which card you are supposed to remove, remove the top-most applicable card in the deck."; "If it is ever ambiguous which card you are supposed to move, move the bottom-most applicable card in the deck."; "The combined Difficulty of two Adversaries is said to be roughly equal to the higher of the two Difficulties plus 50-75% of the lower.").
- **Agreement:** `wiki-only` (engine factor 0.625 sits inside the Wiki's 50–75% range). Q19 answered (Brett: confirmed, as JE p.19 states).
- **Affects:** deck recipe interpreter; fear runway
- **Status:** reviewed by Brett (2026-10-19, Q19; JE p.19)

**X-2 · Escalation timing by Adversary**

| Adversary | Escalation | Engine text timing | Wiki ruling | When, relative to Explore |
|---|---|---|---|---|
| Brandenburg-Prussia | Land Rush | "(Before Exploring)" | "happens before Exploring." | before |
| England | Building Boom | none | "happens before Exploring."; "an Invader Action, not an Adversary Action." | before (a Build) |
| Sweden | Swayed by the Invaders | "After Invaders Explore into each land" | "happens after Exploring." | after |
| France | Demand for New Cash Crops | "After Exploring" | "happens after Exploring." | after |
| Habsburg Livestock | Seek Prime Territory | "After Exploring" | "happens after Exploring." | after |
| Russia | Stalk the Predators | none | "happens before Exploring." | before |
| Scotland | Ports Sprawl Outward | none | "happens before Exploring." | before |
| Habsburg Mining | Mining Tunnels | "After Advancing Invader Cards" | (no ruling) | after Advance (an Explore with no source) |

Sources: `adversaries.json` › `stageIIEscalationBody`; the Wiki adversary pages listed in Appendix A. Status: sourced (2026-10-19, NQ-G; FAQ "Stage II escalation": an Escalation with no stated timing happens before the Explore).

**X-3 · "Accelerate the Invader Deck"**

- **Rule:** Accelerate = take the topmost card of the lowest Invader Stage still in the deck out of the deck (one card off the deck clock). Used by Sweden L4, Elemental Invocation (Setup) and Despicable Theft (escaping numbered Thieves). The card is discarded face-up into the Invader discard: it is seen, the companion may name it, and it leaves the unseen pool as a known card (NQ-E). Exception (FAQ): Salt Deposits can't be Accelerated out; discard the next Stage II card (or the top-most Stage III) instead.
- **Engine:** Sweden L4: "(After the initial Explore, discard the topmost card of the lowest Invader Stage in the deck.)"; `scenarios.json` › `ElementalInvocation` and `DespicableTheft`: "(Discard the topmost card of the lowest Invader Stage …)".
- **Wiki:** Sweden@12523 ruling: "To "Accelerate the Invader Deck" means to discard the topmost card of the lowest Invader Stage still remaining in the Invader Deck. (If the deck is in order, this will simply be the top card of the deck, but if an Adversary has changed its ordering, the card to remove may be deeper down.)" Elemental_Invocation@3350 and Despicable_Theft@12058: "(Remove the topmost card of the lowest Invader Stage …)".
- **Agreement:** `agree` on which card; `conflict` on "discard" vs "remove" (visibility)
- **Resolution:** "Discard" is right: the JE rulebook defines Accelerate as a discard, and FAQ (Elemental Invocation) says the rulebook's "discarded" beats the reminder text's "removed" (NQ-E).
- **Affects:** deck clock; card counting; firewall (is the card seen?)
- **Status:** sourced (2026-10-19, NQ-E; JE rulebook "Accelerate", FAQ "Elemental Invocation")

**X-4 · Effects that change the Invader Deck during play**

Each changes the deck clock (R-TS-7) or the unseen pool. Engine identifiers; see the cited entries.

| Effect | What it does to the deck | Entry |
|---|---|---|
| Russia L5 Entrench in the Face of Fear | an unused Stage II card under Fear card 3 and a Stage III under Fear card 7; when revealed, placed face-up in Build; identities hidden until then (Q20) | R-ADV-RUS-L5a, R-ADV-RUS-L5b |
| Habsburg Livestock L5 Wave of Immigration | a non-Invader reminder card under the top 5 Invader cards | R-ADV-HLC-L5a |
| Habsburg Mining L4 Salt Deposits | a special Stage II card in the 2nd Stage II slot; "Do not advance this card" from Ravage | R-ADV-HME-L4 |
| Sweden L4, Elemental Invocation, Despicable Theft | Accelerate (X-3): the card is discarded face-up, seen (NQ-E) | R-ADV-SWE-L4a, R-SC-ElementalInvocation-1, R-SC-DespicableTheft-1 |
| Event Rising Interest in the Island, "Ignore Their Curiosity" | top Invader card returned to the box (cards placed at Setup skipped), unseen (Q23) | 5.3 |
| Event Hard-Working Settlers, "Create Unnerving Distractions" | bottommost Stage II and Stage III cards removed from the game | 5.3 |
| Fear Card Explorers Are Reluctant TL2 / TL3 | skip the next normal Explore; TL2 draws an additional Explore card next Invader Phase; TL3 still reveals a card | 5.4 |
| Fear Card Immigration Slows TL2 | skip the next normal Build; the Build card stays in place instead of shifting | 5.4 |
| Fractured Days, Visions of a Shifting Future | look at the top card, shuffle the top 2, or put it on the bottom | R-SP-FracturedDaysSplitTheSky-2 |
| Power Card The Past Returns Again | swap the top Invader card with a discard within 1 Stage | 5.6 |

**X-5 · Invader Actions outside the three slots**

These add a Ravage, Build or Explore that the three face-up cards do not show. The forecast must add them on the turn they apply.

- Adversaries: England L3/L4/L6 (High Immigration Build before Ravage), Russia L3 (Ravage also in lands with 3+ Explorers), Russia L5 (cards from the Fear Deck into Build), Habsburg Mining L1 (Build cards Ravage in Mining lands), L5 (a Build per board after the Build Step), Escalation (England Build before Explore; Habsburg Mining Explore after Advance, no source).
- Scenarios: Blitz (extra Build + Explore at Setup), The Great River (Build in Coastal lands with Explorers after the Build Step; Explore into Coastal lands without Town/City after the Explore Step), Varied Terrains (Wetlands Build twice), Powers Long Forgotten (Jagged Tooth: Explore into Wetland → Build).
- Events: Ransack and Run, Act Cautiously in the Background, Harvest Bounty, New Cash Crops Take Hold, Provincial Seat, Extensive Building, Sprawl Contained by the Wilds, Fortification, Search for Unclaimed Lands, Head Off the Problems Yourself (Omen Ravages), Teach the Invaders (next Ravage becomes a Build).
- Blight card: Thriving Crops (Build in 3 lands per board).
- Power Cards: Blur the Arc of Years, Manifest Incarnation (force Build/Ravage).

**X-6 · Other combinations named by the sources**

- Scotland L1 with effects that add 2 Explorers: 1 Town + 1 Explorer (Explorer@7992, errata: "So if an Explore would normally add 2 Explorers - due to an Event, or combining Scotland with France - it would instead add 1 Town and 1 Explorer."). `wiki-only`.
- Habsburg Livestock L1 still gathers Towns when an Event turns the Ravage Step into a Build Step (Blight@7555, Clarifications: "Even if an Event changes the Ravage Step to a Build Step, "Migratory Herders" (gathering Towns into lands slated to Ravage the following turn) still activates as normal."). `wiki-only`.
- France's Town limit includes Towns placed by Setup icons (France_(Plantation_Colony)@7940 ruling: "All {Town} placed onto the island come from this limited supply, including {Town} placed on lands with the pre-printed {Town} icon."). `wiki-only`.
- Dahan Insurrection Military Response and Isolate: "The simplest ruling is "Military Response ignores Isolate"" (Isolate@4040). `wiki-only`.

## 7. The pipeline exceptions that matter most for forecasting

Ranked by how often they change the Explore/Build/Ravage lists or the deck clock.

1. **Adversary deck recipes** change the deck clock and when each Stage arrives: Prussia L2–L6 (Stage III after Stage I; deck down to 8), Russia L4 (Stage II and III alternate) and L5 (Stage II/III cards from the Fear Deck straight into Build), Scotland L2/L4 (Coastal Lands fixed as the 3rd Stage II card; a Stage III among Stage II), Habsburg Livestock L3 and Habsburg Mining L4. The `deck_recipe` entries in section 4.
2. **England High Immigration (L3+)**: a fourth slot; the old Ravage card Builds again before Ravage every turn (L4: all game; L6: twice when no Fear Card resolves). R-ADV-ENG-L3/L4/L6b.
3. **Builds in lands without Invaders**: England L1 (adjacent to 2+ Town/City), Scotland L3 (Coastal, adjacent City). Breaks R-BU-1.
4. **Explore that adds Towns or ignores sources**: Scotland L1 (Coastal Explore adds a Town), Habsburg Mining Escalation and Search for Unclaimed Lands (no source needed), The Great River (all Coastal lands without Town/City). Breaks R-EX-1.
5. **Ravage cards matching more lands**: Russia L3 (3+ Explorers), Habsburg Mining L1 (Build cards Ravage Mining lands).
6. **Escalation timing and sources**: before vs after Explore vs after Advance (X-2); before-Explore Towns are sources (R-EX-8; FAQ, NQ-G).
7. **Fear Cards revealed in the Fear step**: skip Explore/Build/Ravage per land, Isolate, Defend, and move cards (Immigration Slows TL2 keeps the Build card; Explorers Are Reluctant TL2 adds an Explore card). 5.4.
8. **Isolate and adjacency changes**: Isolate (no Explore in or from the land), Finder of Paths Unseen (new adjacencies), Ocean's Deeps (land becomes Ocean, neighbors become Coastal), Wandering Voice (Incarna Isolates its land). R-EX-6, 5.1.
9. **Skips**: a skipped Ravage also cancels the Dahan counterattack (R-RA-5). Ravage skips: A Spread of Rampant Green (Choke the Land, also Builds), the skip Power Cards (all Fast), Events Stricken, No Bravery Without Numbers, Lethargy and Degeneration, Temporary Caution, Fear Card Tread Carefully. Build skips: Towering Roots at its empowered Incarna, Disease, Fear Cards Avoid the Dahan, Immigration Slows, Overseas Trade Seems Safer. 5.1, 5.3, 5.4.
10. **Damage and blight thresholds**: Sweden L3 (Town 3, City 5), Russia L1 (Explorer 2), Habsburg Livestock L6 (+2), England L5 (Town/City +1 Health), Vital Strength Resilience (+8 land Health), Sweden L1 and Habsburg loss (overkill wording, NI p.14; Q18). R-ADV, 5.1.

## 8. New open questions (merged into `open-questions.md`)

Same format as `open-questions.md`. Questions marked *(engine read)* need an IL read, not the physical copy.

- [x] **NQ-A. Sweden L4 Royal Backing timing.** Is the Invader Deck Accelerated after the initial Explore (engine) or during Setup after adding Invaders (Wiki), possibly before the initial Explore? And is the Town added to "the land matching the discarded card" (engine) or "the land of that terrain" (Wiki)?
  *Depends on:* Sweden L4+ deck recipe, which card is gone, Turn 1 Build forecast. *Sources:* engine `TheKingdomOfSweden.levels[4]`: "At the end of Setup, Accelerate … (After the initial Explore, discard …)"; Sweden@12523 L4: "During Setup, after adding all other Invaders, Accelerate the Invader Deck."
  **Answer:** see `open-questions.md` NQ-A: sourced 2026-10-19 (FAQ, Sweden): the engine's timing, after the initial Explore; Town in the land with fewest Invaders matching the discarded card (R-ADV-SWE-L4a).

- [x] **NQ-B. Habsburg Mining L2: per board?** "Add 1 Disease and 1 City in the highest-numbered land with a Town Setup symbol": on each board (engine) or once (Wiki text)?
  *Depends on:* initial board at L2+. *Sources:* engine: "On each board, Add 1 {Disease} and 1 {City} …"; Habsburg_Mining_Expedition@12138 L2 omits "On each board".
  **Answer:** see `open-questions.md` NQ-B: sourced 2026-10-19 (FAQ errata): on each board (R-ADV-HME-L2).

- [x] **NQ-C. The Great River loss count.** More than 3 escaped Towns per starting board (engine) or per player (Wiki)?
  *Depends on:* Great River loss clock with an extra board. *Sources:* engine `TheGreatRiver`: "more than 3 {Town} per starting board"; The_Great_River@12067: "more than 3 {Town} per player".
  **Answer:** see `open-questions.md` NQ-C: sourced 2026-10-19 (FAQ, extra board): per board (R-SC-TheGreatRiver-4).

- [x] **NQ-D. Blitz blight count.** +1 Blight per board on the Blight Card (engine) or per player (Wiki; its footnote says per board when playing with an extra board)? Same for "1 less per starting board" when Blighted.
  *Depends on:* blight runway in Blitz with an extra board. *Sources:* engine `Blitz` Setup/Rules Changes; Blitz@3296 and its querki footnotes.
  **Answer:** see `open-questions.md` NQ-D: sourced 2026-10-19 (FAQ, Blitz): per board, both lines (R-SC-Blitz-4).

- [x] **NQ-E. Is an Accelerated card seen?** When the deck is Accelerated (Sweden L4, Elemental Invocation, Despicable Theft), does the card go face-up to the Invader discard or out of the game unseen?
  *Depends on:* firewall (may the companion name it?) and card counting (does it leave the unseen pool as a known card?). *Sources:* engine texts say "Discard the topmost card"; Sweden@12523 ruling says "discard"; Elemental_Invocation@3350 and Despicable_Theft@12058 say "Remove the topmost card".
  **Answer:** see `open-questions.md` NQ-E: sourced 2026-10-19 (JE rulebook, FAQ): discarded face-up, seen (X-3).

- [ ] **NQ-F. Isolate and cascades in the engine.** *(engine read)* Does `CascadeBlight`'s `GetAdjacentIslandLocations` ignore Isolate, as the Wiki rule requires?
  *Depends on:* cascade targets on Isolated lands. *Sources:* Blight@7555 and Isolate@4040: "Isolate does not affect Blight cascades"; engine not read.
  **Answer:** still open (engine read in progress; see `open-questions.md` NQ-F). The rule itself, Isolate does not affect cascades, is confirmed by Brett (Q3, R-EX-6).

- [x] **NQ-G. Do Towns added by a before-Explore Escalation count as sources for that Explore?** (Brandenburg-Prussia Land Rush, Russia, Scotland; England's Escalation is a Build.)
  *Depends on:* Explore odds on Escalation turns. *Sources:* Sequence_of_Play@12144 "first perform the Escalation effect"; adversary rulings "happens before Exploring"; Invader@9428 FAQ (pieces added *by the card's own Explore* are not sources). No source states the Escalation case.
  **Answer:** see `open-questions.md` NQ-G: sourced 2026-10-19 (FAQ, Stage II escalation): yes; unless stated, an Escalation happens before the Explore (R-EX-8).

- [x] **NQ-H. Accumulated Devastation: first Ravage, or first successful Ravage?** If the first matching Ravage on a board is skipped, does the bonus move to the next one?
  *Depends on:* Ravage ledger on that Event turn. *Sources:* engine: "During the first Ravage Action …"; accumulated_devastation@unrecorded (Wiki cache): "During the first successful Ravage Action (in one land) this Invader Phase …".
  **Answer:** answered: first successful Ravage Action, per the printed text (`open-questions.md` NQ-H).

- [x] **NQ-I. Land Health wording (Depleted Soil, The Land's Bounty Exhausted, blight card All Things Weaken).** Engine: "the land has -1 Health". Wiki cache: "it only takes 1 Damage to add Blight". They differ only when another land-Health change applies (e.g. Vital Strength Resilience +8, All Things Weaken -1). Which applies then?
  *Depends on:* blight threshold per land. *Sources:* engine `CitiesRise`, `FortuneSeekers`, `AllThingsWeaken` ("the land have -1 Health"); cities_rise@unrecorded, fortune-seekers@unrecorded, all_things_weaken@unrecorded ("The land takes Blight on 1 less Damage").
  **Answer:** see `open-questions.md` NQ-I: sourced 2026-10-19 (NI p.14): the engine's "-1 Health"; modifiers add, min. 1 (5.3, 5.5).

- [x] **NQ-J. Blitz: does the setup card Ravage on Turn 1?** Setup is "Explore, then Build and Explore", so the setup card would reach the Ravage space for Turn 1 *(inference)*.
  *Depends on:* Turn 1 Ravage forecast in Blitz. *Sources:* engine `Blitz` Setup Changes; Blitz@3296 (same text); card flow Sequence_of_Play@12144 §4.
  **Answer:** see `open-questions.md` NQ-J: sourced 2026-10-19 (card text; inference to verify in Steam): yes (R-SC-Blitz-3).

- [x] **NQ-K. Downpour's Drench the Landscape and Invader Cards.** Does Downpour's Sacred Site match a Wetland Invader Card? The text names "Spirit Actions and Special Rules" only.
  *Depends on:* Build/Ravage/Explore lists with Downpour. *Sources:* engine `DownpourDrenchesTheWorld.specialRules[DrenchTheLandscape]`; downpour-drenches-the-world@unrecorded (same text).
  **Answer:** see `open-questions.md` NQ-K: sourced 2026-10-19 (card text): no (R-SP-DownpourDrenchesTheWorld-1).

- [ ] **NQ-L. Per-land Ravage modifiers only when the Invaders deal 1+ Damage.** *(engine read)* Does the engine add per-land bonuses (Events such as Increasing Aggression, Habsburg L6 +2) only when the Invaders' own total is ≥ 1, as JE p.7 says?
  *Depends on:* Ravage ledger with Strife or tiny Invader groups. *Sources:* Defend@3397 "Ravage Timing"; engine `GetRavageDamageModification` condition not read.
  **Answer:** still open (engine read in progress; see `open-questions.md` NQ-L).

- [ ] **NQ-M. Isolated lands as Explore targets in the engine.** *(engine read)* The engine source predicate excludes Isolated lands; does the target side also exclude them?
  *Depends on:* Explore odds with Isolate. *Sources:* Isolate@4040: "Invaders do not Explore Isolated lands"; engine_state_model.md Q3 (sources only).
  **Answer:** still open (engine read in progress; see `open-questions.md` NQ-M). The rule itself is confirmed by Brett (Q3, R-EX-6).

- [x] **NQ-N. Breath of Darkness's Incarna "may count as Beast".** In the engine text only; the Wiki copy (2026-04-19) lacks it.
  *Depends on:* Russia L1/Escalation (lands with Beast), Beast Events and Fear Cards. *Sources:* engine `BreathOfDarknessDownYourSpine.specialRules[TerrorStalksTheLand]`: "It may count as {Beast}."; breath-of-darkness@unrecorded.
  **Answer:** answered: printed text without "may count as Beast" (no errata; Steam difference) (`open-questions.md` NQ-N).

- [x] **NQ-O. Wandering Voice text differences.** Engine: Spread Tumult adds Strife when the Incarna moves "to a land"; Wiki: "to a land with Invaders". Engine: the Incarna "may count as {SacredSite}"; Wiki copy lacks it.
  *Depends on:* where Dahan stop counterattacking; Sacred Site targeting. *Sources:* engine `WanderingVoiceKeensDelirium.specialRules`; wandering-voice@unrecorded.
  **Answer:** answered (`open-questions.md` NQ-O): Strife part no difference; Sacred Site part per the printed text, without it (no errata; Steam difference).

- [ ] **NQ-P. Brandenburg-Prussia L2's moved Stage III card.** *(engine read)* `shouldMovedInvaderCardOverrideStage: true`: does the moved card count as Stage III, or as the Stage of the slot it sits in, for "current Invader Stage" effects (R-TS-13) and for Escalation?
  *Depends on:* Stage-keyed Events, Accumulated Devastation, card counting. *Sources:* engine `TheKingdomOfBrandenburgPrussia.levels[2]`; Wiki silent.
  **Answer:** still open (engine read in progress; see `open-questions.md` NQ-P).

Existing questions this pass touched (no new answer): Q1, Q2, Q4 (R-RA-2 supports "Defend reduces both"), Q5, Q10, Q11, Q16 (England L6 difficulty 10 vs 11 confirmed against Wiki England@6844), Q17 (France loss, confirmed against France_(Plantation_Colony)@7940), Q18 (Sweden L1 and Habsburg loss wording, confirmed against Sweden@12523 and Habsburg_Monarchy_(Livestock_Colony)@6850), Q19, Q20, Q22, Q24. All have since been answered (`open-questions.md`, 2026-10-19).

## Appendix A. Wiki sources

Archived pages (web.archive.org raw snapshots of spiritislandwiki.com; revision id = `wgRevisionId`):

| Page | Revision | Snapshot (UTC) |
|---|---|---|
| Adversary | 12228 | 2025-04-27 15:33 |
| Badlands | 13204 | 2026-07-02 05:14 |
| Blight | 7555 | 2025-07-13 21:47 |
| Blitz | 3296 | 2026-07-02 07:03 |
| Brandenburg-Prussia | 6845 | 2025-04-27 15:33 |
| City | 7994 | 2025-04-27 15:35 |
| Dahan_Insurrection | 4718 | 2026-06-26 17:24 |
| Defend | 3397 | 2025-04-27 15:34 |
| Despicable_Theft | 12058 | 2026-07-02 07:00 |
| Disease | 3166 | 2026-07-02 05:14 |
| Elemental_Invocation | 3350 | 2026-07-02 07:02 |
| England | 6844 | 2025-04-27 15:33 |
| Explorer | 7992 | 2025-09-30 20:45 |
| France_(Plantation_Colony) | 7940 | 2025-04-27 16:04 |
| Game_Setup | 8832 | 2025-04-27 15:52 |
| Guard_the_Isle's_Heart | 3282 | 2026-07-02 07:03 |
| Habsburg_Mining_Expedition | 12138 | 2025-04-27 15:33 |
| Habsburg_Monarchy_(Livestock_Colony) | 6850 | 2025-04-27 15:33 |
| Invader | 9428 | 2025-04-27 15:41 |
| Isolate | 4040 | 2025-04-27 15:34 |
| Powers_Long_Forgotten | 3141 | 2026-07-02 07:03 |
| Rituals_of_Terror | 3285 | 2026-07-02 07:01 |
| Rituals_of_the_Destroying_Flame | 2109 | 2025-04-27 15:40 |
| Russia | 6851 | 2025-04-27 19:33 |
| Scotland | 12772 | 2026-07-02 06:57 |
| Second_Wave | 12060 | 2025-04-27 15:40 |
| Sequence_of_Play | 12144 | 2025-12-07 01:33 |
| Strife | 3334 | 2025-04-27 15:35 |
| Sweden | 12523 | 2025-04-27 15:33 |
| The_Dahan | 7991 | 2026-07-02 05:14 |
| The_Great_River | 12067 | 2026-07-02 07:00 |
| Town | 7993 | 2026-07-02 05:14 |
| Varied_Terrains | 6140 | 2026-07-02 07:00 |
| Ward_the_Shores | 1274 | 2025-04-27 15:40 |
| Wilds | 3164 | 2026-07-02 05:14 |

Repo fetcher outputs (no revision ids recorded; cited `Name@unrecorded`):

- Spirit pages: `data/references/wiki/<slug>.json` (39 files, committed 2026-04-19 in this repo).
- Event, Fear and Blight card caches: `/home/brett/repos/spirit-island/.wiki-cache/event_*.json`, `fear_*.json`, `blight_*.json` (fetched 2026-04-19).

Not retrieved: aspect pages (only 5 Shadows aspects exist locally), and pages with no archive snapshot (Explore, Build, Ravage, Invader Phase, Escalation, Speed/Fast/Slow, Innate Power, Coastal, Inland, Range).

## Appendix B. Coverage

| Layer | Covered | agree | conflict | engine-only | wiki-only |
|---|---|---|---|---|---|
| Adversary level effects | 8 adversaries × levels 0–6 (56 levels) | 68 | 5 | 0 | 0 |
| Spirit entries | 34 spirits + 24 aspects scanned; 30 spirits have entries | 36 | 1 | 16 | 0 |
| Scenario entries | 13 scenarios; 12 have entries | 27 | 2 | 0 | 0 |
| Event sections | 62 events; 51 have entries | 89 | 3 | 1 | 0 |

Section 1–3 core entries: 13 turn-structure, 10 Explore, 5 Build, 11 Ravage, 7 Fast/Slow. Fear Cards (50), Blight Cards (25) and Power Cards (321) are covered as families in 5.4–5.6.
