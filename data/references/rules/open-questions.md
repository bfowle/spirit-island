# Open rules questions

For Brett to answer from the physical copy, in bulk. Each entry: the question,
what depends on it, what the sources say, and an answer line. When answered,
mark the box, fill the answer, and the rule moves to the rules register
(`register.md`) as `reviewed by Brett`.

Sources are summarized; full citations are in
`reports/Spirit Island companion stat models.md` (Appendix B).

## Settled

- [x] **Turn-1 Event.** Drawn and discarded without resolving; public in the
  Event discard (Jagged Earth rulebook p.12).
  **Answer (Brett, 2026-10-18):** Rulebook is correct. You can view it, it is
  discarded, and it does not resolve that game.

## Core rules (base game)

- [x] **Q1. Invader card pool per stage.** Stage I = 4 single-terrain cards;
  Stage II = Coastal Lands + 4 single-terrain cards with Escalation; Stage III
  = 6 two-terrain cards; one removed unseen per stage (deck 3·4·5).
  *Depends on:* every Explore odd. *Sources:* engine card list; rulebook.
  **Answer (Brett, 2026-10-19):** Confirmed (no adversary). Deck is 3 Stage I,
  then 4 Stage II, then 5 Stage III. Stage I: Jungle, Mountain, Sands,
  Wetland. Stage II: the same four with the Escalation icon printed, plus
  Coastal Lands. Stage III: every two-terrain pair, printed as Jungle+Sands,
  Jungle+Wetland, Mountain+Jungle, Mountain+Wetland, Sands+Mountain,
  Sands+Wetland (matches the engine ids `JungleSands3` … `SandsWetland3`).

- [x] **Q2. Build.** If a land has more Towns than Cities, add a City;
  otherwise add a Town. Builds only happen in lands that have Invaders.
  *Depends on:* build forecast; `invader-lifecycle.md` currently states the
  opposite. *Sources:* engine; rulebook.
  **Answer (Brett, 2026-10-19):** Correct (no adversary). More Towns than
  Cities: add a City. Fewer or equal: add a Town. Builds need Invaders in
  the land, except where an adversary changes it: England L1 (Indentured
  Servants Earn Land) also builds in matching lands without Invaders that
  are adjacent to at least 2 Towns/Cities (register R-ADV-ENG-L1 agrees;
  not yet applied by the forecast).

- [x] **Q3. Explore sources.** A land is explored if it has a Town or City, or
  is adjacent to a Town, City, or the Ocean. One Explorer per land. Explorers
  are not a source.
  *Depends on:* per-land Explore odds. *Sources:* engine; wiki copy.
  **Answer (Brett, 2026-10-19):** Correct. Also apply the Jagged Earth
  Isolate rules (register R-EX-6: an Isolated land is not explored and is not
  a source; Isolating an Ocean stops it being a source but the lands stay
  Coastal; Isolate does not affect cascades; it lasts one turn). The
  forecast does not apply Isolate yet.

- [x] **Q4. Ravage details.** One blight if damage after Defend ≥ land health
  (2). Invader damage goes in full to both the land and the Dahan. Dahan
  counterattack after the Invaders by default.
  Open: **does Defend also reduce damage to the Dahan?** How is damage split
  across several Dahan?
  *Depends on:* ravage ledger. *Sources:* rulebook; engine (Defend-vs-Dahan
  code not read).
  *Engine evidence (Test 1, Turn 3):* Jungle E3 with Defend 2 (Spreading
  Timidity): invaders 3 damage → `Dahan8 => 1, JungleE3 => 1`. Defend reduced
  the damage to both the land and the Dahan. Jungle C1 (no Defend, 1 Dahan):
  `Dahan1 => 2, JungleC1 => 3`; the excess damage to Dahan was not carried
  over. Confirm against the rulebook.
  **Answer (Brett, 2026-10-19):** Confirmed. (1) Defend shields the land and
  the Dahan by the same amount: Defend 2 keeps a Dahan from being damaged and
  the land from being blighted. (2) Invader and Event damage to Dahan must
  Destroy them as efficiently as possible: 3 damage against 3 Dahan destroys
  1 (2 damage) and leaves 1 with 1 damage; it cannot be spread 1 per Dahan.
  Spirit Power damage to Dahan is allocated by choice. (3) Excess damage to a
  Dahan is lost. (4) Counterattack is 2 damage per surviving Dahan, assigned
  by choice. *Source:* Branch & Claw rulebook, "Damaging the Dahan" and
  "Health Changes" (Health never below 1; a piece already holding damage ≥
  its new Health is Destroyed at once).

- [x] **Q5. Blight effects.** Adding blight destroys one presence of each
  spirit in the land. A cascade happens only if the land already had blight;
  the player picks the adjacent land.
  *Depends on:* presence risk; cascade badge; `presence-economy.md` states the
  reverse. *Sources:* engine; rulebook.
  **Answer (Brett, 2026-10-19):** Correct (core rulebook p.15). Adding
  Blight destroys one Presence of each Spirit in the land unless that
  Spirit's own rules say otherwise. The cascade target is picked by the
  Spirit whose Power caused it; for Events and other non-Power causes the
  players agree. "Presence as insurance" in `presence-economy.md` is wrong:
  Invaders do not damage Presence, and Blight destroys Presence, not the
  reverse. Oceans take cascade Blight only with Ocean's Hungry Grasp in play
  and under Scotland's Ocean blight rules.

- [x] **Q6. Fear.** 4 fear per player per card. Destroying a Town = 1 fear,
  City = 2, Explorer = 0. Terror Level changes at the dividers. Fear cards
  earned in one window all resolve at the new Terror Level.
  *Depends on:* fear runway; `fear-track.md` has different numbers.
  *Sources:* engine constants; BGG 2611841.
  **Answer (Brett, 2026-10-19):** Confirmed. 4 Fear Markers per player in the
  Fear Pool; England L6 adds +1 per player, and an extra board adds +1 per
  player (JE rulebook "Game-wide rules and changes"; Test 1, solo with boards
  C+E, needed 5 per card). Fear comes from *destroying* (not removing) a Town
  (1) or City (2), from Powers with Fear, and from Events (e.g. Beasts) and
  other effects. Leftover Fear carries over. Victory: Terror Level 1 no
  Invaders, 2 no Towns or Cities, 3 no Cities; and a Fear victory at once
  when the Fear Deck is empty ("Victory when revealed" printed under the
  deck). `fear-track.md` lines 9, 28, 99–100 are wrong.

- [x] **Q7. When does the Terror Level change?** Immediately when the divider
  is reached (mid-power), or later? A game log shows the change sourced to a
  power card.
  *Depends on:* victory-blocker timing.
  *Engine evidence (Test 1, Turn 3):* Display Your Wrath earned the last
  Terror Level I card during the Event; the log shows `TerrorLevelII → Out of
  Game` and `SetTerrorLevelCardAction: Source: Dahan Trade with the Invaders`
  at once, and both earned cards then resolved at Terror Level II. Confirm
  this matches the tabletop rule.
  **Answer (Brett, 2026-10-19):** Confirmed. The Terror Level changes the
  moment the divider is reached (or the last card is revealed), as cards are
  banked into Earned Fear Cards; banked cards resolve at that level. Even
  while resolving Fear Cards, a card that destroys enough Towns/Cities (e.g.
  Scapegoats) can earn another card and raise the level; later cards resolve
  at the new level, but the card currently resolving is not bumped up.
  *FAQ (Fear and Fear Cards):* "subsequent Fear Cards immediately start using
  the new Terror Level"; an earned card goes to the bottom of the stack being
  resolved. *FAQ (Core Game):* winning by revealing the Victory space is
  immediate, whatever the last card says; other wins and losses are checked
  after each action tree (Q11).

- [x] **Q8. Earned fear cards.** Face-down until resolved at the table? Does
  the Steam UI show them before they resolve? (The Steam log names them.)
  *Depends on:* firewall rule.
  **Answer (Brett, 2026-10-18, live in Test 1):** Banked fear cards are face
  down by default until revealed in the Fear phase (after Events). The log
  still names each one when earned, so the firewall must redact it.

- [x] **Q9. Power gain.** Draw 4, keep 1, the other 3 to the discard (not the
  bottom of the deck). A Major requires a Forget. The discard reshuffles into
  the deck when it runs out.
  *Depends on:* draft odds; `card-draft-theory.md` says bottom of deck.
  **Answer (Brett, 2026-10-19):** Correct (core rulebook p.18). The unkept
  cards are discarded, never returned to the bottom of the deck. A Major
  requires a Forget unless an effect waives it (e.g. Unlock the Gates of
  Deepest Power gains a Major without Forgetting). A Forgotten card goes to
  its deck's discard. When a deck runs out, shuffle that deck's discard;
  rare in practice with the full card pool, possible at high player counts.

- [x] **Q10. Element timing.** Power Card elements count from the moment the
  card is paid for, so Slow cards' elements count toward Fast innates.
  *Depends on:* innate threshold checks.
  **Answer (Brett, 2026-10-19):** Confirmed. Every card played that turn,
  Fast or Slow, grants its Elements for the entire turn from the moment it is
  played. A Presence-track "Any" space grants one Element each turn: "the
  Spirit may choose which Element they get at any time, but then cannot
  change it until the next turn" (usually chosen in Growth, e.g. Fire, then
  fixed for the rest of the turn). Elements are lost if the granting card is
  Forgotten from play. Some Jagged Earth and Nature Incarnate spirits change
  these rules through their own special rules, each with a straightforward
  ruling. *FAQ:* "Rules semi-commonly misplayed"; Core Game, Elements.

- [x] **Q11. Loss timing.** Losses are checked once no triggered actions
  remain (end of action trees). Deck exhaustion loses at the Explore of the
  turn the deck runs out. A spirit with no presence loses.
  *Depends on:* race clocks. *Sources:* NI p.13; rulebook; lumia.
  **Answer (Brett, 2026-10-19):** Confirmed, with these corrections.
  *Blight card:* when the last Blight leaves the Healthy side, flip it and
  follow the other side, which usually adds more Blight to the pool. Some
  cards flip from Healthy to a still-Healthy side with further rules. When
  the last Blight leaves the Blighted side, that is usually an immediate
  loss (the printed "you lose"). The Healthy card flips mid-Ravage and that
  Ravage keeps going. *Invader deck:* not a fixed count (many rules change
  the deck); it is a state check: the loss happens when an Explore step must
  resolve and no card is left. That Invader Phase still Ravages and Builds
  first, so a win there (e.g. through Defend or a Dahan counterattack) comes
  before the failed Explore. *Other timing:* Events and Powers that perform
  Invader Actions (a Ravage, even a Build) can trigger an adversary's
  alternate loss condition, even during the Spirit phase. Wins and losses
  are checked after each action tree (FAQ, Core Game, "What is the timing
  of Victory / Defeat?").

## Blight card and pools

- [x] **Q12. Blight pool sizes.** Healthy side 2 per player + 1 (setup only).
  Flipped side X per player with no +1. Per board with an extra board. 5 per
  player + 1 with no blight card. Solo redraw of 2-per-player cards.
  *Depends on:* blight runway. *Sources:* NI p.5, p.14; FAQ errata; JE p.18.
  **Answer (Brett, 2026-10-19):** Correct. The errata changed setup to
  2 per player + 1 (5 per player + 1 with no Blight Card); the +1 does not
  apply when a card flips and refills the pool (FAQ "one more blight
  errata"; not for the Horizons board). With an extra board, count boards
  instead of players, at setup and on flip (JE rulebook). Still-Healthy cards
  draw a new Blight Card and go straight to its reverse side (JE rulebook).
  Brett has used the optional solo redraw of 2-per-player Blighted cards
  (B&C rulebook, Solo Play), especially before Jagged Earth and Nature
  Incarnate, so a solo flip to such a card may be redrawn at the table.
  NI p.14 adds: "Don't redraw Still-Healthy Island Cards".

- [x] **Q13. Disintegrating Ecosystem.** 3 or 5 blight per player?
  *Sources:* conflicting posts; wiki table says 5.
  **Answer (Brett, 2026-10-19, physical card):** 5 per player. The card
  reads: "Immediately, on each board: Destroy 1 Beast, then add 1 Blight to a
  land with Town/City", with 5 Blight per player on the Blighted side. Matches
  the engine (`flippedNumberPerStartingBoard: 5`) and the FAQ
  (Disintegrating Ecosystem): the added Blight comes from the card's new
  pool, and Blight removed from the board returns to the card.

- [x] **Q14. Horizons boards.** 5 per player + 1 blight, and the +1 errata does
  not apply? (Secondary source only.)
  **Answer (Brett, 2026-10-19):** Correct. The Horizons Invader board was
  printed after the errata and its printed "+1" already is the errata, so
  do not add another +1. *FAQ:* the errata "does not affect the board from
  Horizons of Spirit Island".

- [x] **Q15. Active and retired cards.** Which cards Nature Incarnate retired
  (Tipping Point, Growth Through Sacrifice, three Events), and the final pool
  sizes (Minors 100 or 101, Majors 78, Fear 50, Events 62, Blight 23).
  *Depends on:* every deck pool.
  **Answer (Brett, 2026-10-19):** NI rulebook p.5 "Dropped Cards": with
  Branch & Claw, remove Outpaced, War Touches the Island's Shores, A Strange
  Madness Among the Beasts (Events), Growth Through Sacrifice (Minor) and
  Tipping Point (Blight). Replacements: Far-off Wars Touch the Island (for
  War Touches the Island's Shores) and Roiling Bog and Snagging Thorn (for
  Growth Through Sacrifice; the rulebook paragraph misprints it "Stinging
  Thorn", the card list says "Snagging Thorn"). Jagged Earth replaced the
  Branch & Claw Major Sea Monsters with an updated card (balance with Many
  Minds Move as One). Slave Rebellion is added only against France.
  *Steam (Test 1 allowed pools):* Minors 100 (Growth Through Sacrifice
  excluded), Majors 78, Fear 50, Events 61 (+ Slave Rebellion with France),
  Blight 23 (Tipping Point excluded). The three dropped Events are not in
  the engine at all; Far-off Wars and Roiling Bog are.

## Adversaries

- [x] **Q16. England L6 difficulty.** 10 (engine, base rulebook) or 11 (wiki)?
  **Answer (Brett, 2026-10-19, physical panel):** 11. The Steam engine's 10
  is a Steam-vs-box difference; `data/adversaries.json` (10) needs fixing.

- [x] **Q17. France Town cap.** Per player (wiki) or per board (engine)?
  **Answer (Brett, 2026-10-19, physical panel):** Per player. "Sprawling
  Plantations: Before Setup, return all but 7 Town per player to the box.
  Invaders win if you ever cannot place a Town." With an extra board the
  limit stays 7 per player ("each player" effects happen once per player, JE
  rulebook), so Steam's "7 per starting board" differs from the box when
  playing with an extra board. *FAQ (France):* Towns that permanently leave
  the island or are replaced return to France's pool; Towns in The Endless
  Dark still count.

- [x] **Q18. Sweden L1 and Habsburg Livestock loss wording.** "≥ 6 / 8+
  damage" or "4+ / 6+ overkill"?
  *Sources:* NI p.14 overkill wording; FAQ.
  **Answer (Brett, 2026-10-19):** Use the Nature Incarnate wording (NI p.14),
  as the engine does. Overkill = Damage beyond the land's Health. Sweden L1
  Heavy Mining: a Ravage with 4+ overkill adds 2 Blight instead of 1.
  Habsburg Livestock Irreparable Damage: track Blight taken off the card
  during Ravages with 6+ overkill; more than the number of players loses.
  Land Health is 2; modifiers stack and the total can't go below 1 (also
  answers NQ-I).

- [x] **Q19. Combined adversaries.** Fear deck = 3 + both adversaries' changes
  per Terror Level; supporting Escalation on every Stage III card; supporting
  deck instructions first, then leading; random terrain for France/Sweden.
  *Sources:* JE p.19; FAQ.
  **Answer (Brett, 2026-10-19):** Confirmed, as JE p.19 states: per Terror
  Level 3 + both adversaries' changes; leading Escalation normal, supporting
  Escalation on every Stage III card (random terrain for France/Sweden);
  supporting deck instructions first, then leading; skip impossible
  instructions; ambiguous Remove = top-most, ambiguous move = bottom-most;
  Prussia L2 = move the bottom-most Stage III card just below the
  bottom-most Stage I card. With Scotland, other setup pieces bound for a
  Coastal land other than #2 go to an adjacent Inland land. With France L2+,
  France's Town pool grows by 1 per player per level of the other
  adversary.

- [x] **Q20. Russia L5.** How are the "unused" invader cards chosen, and how do
  they count in the fear deck?
  **Answer (Brett, 2026-10-19):** (1) "Unused" = the cards left over after
  the entire Invader Deck is built (adversary changes included), taken face
  down, so they stay hidden until revealed (firewall: never named before).
  (2) They are Invader Cards, not Fear Cards: they need no Fear to pass and
  show a land type. (3) Each resolves when uncovered: the Stage II card is
  exposed by earning the 3rd Fear Card, the Stage III by the 7th, and goes
  face-up to the Build space at once. *FAQ (Russia):* a Stage II card placed
  this way triggers its Escalation when first resolved.

- [x] **Q21. Tie-breaks.** For "add to a land" choices (Prussia escalation,
  Sweden L5, Scotland escalation), who picks and how?
  **Answer (Brett, 2026-10-19):** The players decide together where adversary
  pieces go and break ties, choosing what is best for them (Prussia, Sweden
  L5, Scotland alike). *FAQ (Stage II escalation):* Prussia's Land Rush Town
  goes in any land without a Town on each board with a Town/City, "best for
  you (and it's expected you will)", and happens before Exploring, so it can
  be an Explore source that turn (answers NQ-G for Prussia); Scotland ties:
  "players break it". The companion lists candidate lands and the players'
  best pick rather than predicting one.

- [x] **Q23. Cards "returned to the box".** When an effect returns the top
  Fear Card (Rising Interest in the Island, Weave Lies) or the top Invader
  card (Ignore Their Curiosity) to the box, does anyone look at it? Assumed
  no. The Steam log names the Fear card (`MoveCardAction … To=Out of Game,
  From=Fear Deck`), so the firewall treats it as hidden.
  **Answer (Brett, 2026-10-19):** Confirmed. Nobody looks at it (FAQ, Branch
  & Claw: "When you return an Invader or Fear card to the box, can you look
  at it? No."). The firewall keeps its identity hidden; the deck count drops
  by one and the card stays in the unseen pool.

- [x] **Q24. First Ravage turn.** Setup explores the first card and advances
  it to Build, so Turn 1 builds and Turn 2 ravages (seen live in Test 1:
  Mountain built Cities on Turn 1, ravaged on Turn 2). The Shadows chapter's
  turn table and `si-rules-check` say the first Ravage is Turn 3 and that
  Concealing Shadows' protection is dormant on Turn 2. Confirm the table rule.
  **Answer (Brett, 2026-10-19):** Confirmed: the first Ravage is generally
  Turn 2 (core rulebook p.5: the setup Explore card goes to Build). Turn 1 is
  a good time to prevent Builds. Each turn you know which lands will Build
  (last turn's Explore card) but not which will Explore, so you can plan the
  Ravage two turns ahead. Fix the Shadows turn table and `si-rules-check`
  (Concealing Shadows is live on Turn 2). Adversary/scenario exceptions:
  NQ-A (Sweden L4), NQ-J (Blitz).

## Steam game content

- [x] **Q22. Scenarios in Steam.** Does the Steam game include Surges of
  Colonization and Destiny Unfolds? (The engine's 13-scenario list has
  neither.)
  **Answer (Brett, 2026-10-19):** Not in Steam yet; Nature Incarnate content
  is still being added to the Steam app. Both are in the NI rulebook. The
  companion only needs the engine's 13; when a new engine build adds them,
  re-run the defs dump (engine SHA changes).

## Project data and chapter fixes to confirm

Each conflicts with the engine, rulebooks or research. Confirm against the
box, then fix. Details: report Appendix B, second table.

- [ ] `data/adversaries.json`: France L3 difficulty 7 (file: 6); Habsburg
  Mining `[1,3,4,5,7,9,10]`; England L6 11 (Q16).
  *Confirmed (2026-10-19):* France 2; 3, 5, 7, 8, 9, 10 (archived Wiki,
  2024). Habsburg Mining 1; 3, 4, 5, 7, 9, 10 (Brett's panel image; fear
  decks 9 (3/3/3), 10 (3/3/4), 11 (3/4/4), 12 (4/4/4), 13 (4/5/4),
  13 (4/5/4)).
- [ ] `data/adversary-stacks.json`: key by level 0–6 (file keys by
  difficulty); correct the stacks for Prussia L2–L6, Russia L4+, Habsburg
  Livestock L3+, Scotland L2–L4, Habsburg Mining L4+, Sweden L4.
- [ ] `data/references/adversaries/*.json`: level names and effects mostly
  invented; regenerate.
- [ ] `data/scenarios.json`: Great River 3, Varied Terrains 2, Destiny Unfolds
  −1 (0 to −2); add Surges of Colonization (+2/+7).
  *Confirmed (2026-10-19):* all four from archived Wiki pages (Great River
  3 and Varied Terrains 2 also match the engine; Destiny Unfolds "-1*",
  usually 0 to -2 by familiarity; Surges "+2/+7"). Neither NI scenario is in
  Steam yet (Q22).
- [ ] `src/fundamentals/fear-track.md`: fear values, Terror Level model,
  adversary claims.
- [ ] `src/fundamentals/tempo.md`: Prussia and Russia claims; split land tempo
  from economy momentum.
- [ ] `src/fundamentals/invader-lifecycle.md`: Build rule inverted; stage
  descriptions; England escalation.
- [ ] `src/fundamentals/presence-economy.md`: presence/blight cause reversed;
  starting presence varies.
- [ ] `src/fundamentals/card-draft-theory.md`: unpicked cards go to the
  discard; no passing a gain; pool sizes.
- [ ] `src/fundamentals/energy-curves.md`: compute targets from track data.
- [ ] `src/statistics/index.md`: Event draws are exchangeable, not
  independent.
- [ ] `src/statistics/digital-vs-tabletop.md`: digital uses a seeded generator
  fixed at setup; the percentage is unsourced.
- [ ] `src/statistics/reading-mindwanderer.md`: data is all Spirit Island
  Digital, analyzed with regressions.
- [ ] `src/statistics/deck-expansion-impact.md`,
  `fear-card-expected-value.md`, `scripts/deck-simulate.py`: fear model.
- [ ] `src/statistics/major-vs-minor-rates.md`: draw 4; 2-cost Majors exist;
  playable without threshold.
- [ ] `src/statistics/event-deck-risk-profiles.md`: Events designed slightly
  negative on average.
- [ ] `src/statistics/blight-deck-severity.md` and
  `src/appendices/fear-blight-event-decks.md`: base blight cards; +1 errata;
  fear model.
- [ ] `src/statistics/aspect-power-deltas.md`: Sunshine figure unsourced.
- [ ] `src/appendices/sources.md`: Jonathan4290 series has 26 entries; 2.27M is
  log lines, not games.
- [ ] `common/src/schema.rs` `EventDeckState.previewed` and tabletop
  `EventDeck.vue`: encode the wrong Turn-1 Event rule.
- [ ] Spirit chapter openers: remove any "plan Turn 1 around the previewed
  Event" advice.

## From the forecast backtest (2026-10-19)

Each rule below is now applied by the companion because the Steam engine did
it in Test 1. Confirm each matches the tabletop rule.

- [x] **Q25. Strife in Ravage.** An Invader with Strife deals no Ravage damage,
  and 1 Strife is removed from it.
  *Engine (Test 1 Turn 4, Sands C2):* `Strife1 ( City 1's Token Location =>
  OutOfGame)`, then `DamageAmountsBySource=[Explorer6-1]`: the City dealt
  nothing, no blight.
  **Answer (sourced, 2026-10-19; Brett delegated):** Confirmed by the JE rulebook
  ("Strife"): "Whenever Invaders Damage the Dahan and/or the land, each
  attacking Invader with any number of Strife deals exactly 0 Damage and
  Removes one Strife. Using the Strife is required." This applies to any
  Invader Damage to the land/Dahan, not only Ravage.

- [x] **Q26. Disease stops a Build.** A Build in a land with Disease is
  cancelled and 1 Disease is removed.
  *Engine (Test 1 Turn 5, Jungle C1):* `Disease1 ( Jungle C1 => OutOfGame)`,
  then `CancelAction: BuildAction`.
  **Answer (sourced, 2026-10-19; Brett delegated):** Confirmed by the JE rulebook
  ("Disease"): when Invaders would Build in a land with Disease, instead
  Remove one Disease ("Prevents the next Build and is then Removed").
  Exception: Habsburg Mining Expedition L1+ makes Disease affect Ravage in
  Mining lands.

- [x] **Q27. Wilds stop an Explore.** An Explore into a land with Wilds is
  cancelled and 1 Wilds is removed.
  *Engine (Test 1 Turn 2, Sands C6):* `Wilds2 ( Sands C6 => OutOfGame)`, then
  `CancelAction: ExploreAction`.
  **Answer (sourced, 2026-10-19; Brett delegated):** Confirmed by the JE rulebook
  ("Wilds"): "When Invaders would Explore into a land with Wilds, instead
  Remove one Wilds from that land." Wilds do not stop the land from being a
  source for its neighbors.

- [x] **Q28. Counterattack removes an Explore source the same turn.** The
  Dahan counterattack (during Ravage) can destroy a Town or City that would
  have been the Explore source for neighboring lands; those lands are then
  not explored that turn, because Ravage resolves before Explore.
  *Engine (Test 1 Turn 3):* Jungle E3's Dahan destroyed its Town; Mountain E5
  and Wetland E4 matched the Explore card and were not explored.
  **Answer (sourced, 2026-10-19; Brett delegated):** Confirmed. The Invader Actions
  resolve Ravage, then Build, then Explore (core rulebook p.10), and which
  lands an Invader Card affects is checked when that card is resolved (JE
  rulebook p.27, quoted in the FAQ). A Town/City destroyed by the
  counterattack is gone before the Explore Card checks for sources.

- [x] **Q29. Range through Oceans.** Does Range count through an Ocean (as a
  land), e.g. range 2 from C1 reaching C3 via the Ocean? The need finder
  currently counts Oceans as lands for Range. (On boards C + E this never
  changes a result at range ≤ 1.)
  **Answer (sourced, 2026-10-19; Brett delegated):** No. "The Ocean itself is not a
  land, and is not in play" (core rulebook, Island Boards). Range counts
  through lands only. Exception (FAQ, Ocean's Hungry Grasp): with Ocean's
  Hungry Grasp on a board, Powers and Special Rules may measure Range through
  that board's Ocean. **Code fix:** the need finder must skip Oceans in its
  Range BFS unless Ocean's Hungry Grasp is in play.

- [x] **Q30. "Dahan grant 1 Defense per piece" (Years of Little Rain).**
  Applies in every land, Defend 1 per Dahan there?
  *Engine (Test 1 Turn 5, Mountain C7):* 2 Dahan, 3 damage → 1 to the land.
  **Answer (sourced, 2026-10-19; Brett delegated):** Yes. Card text (Years of Little
  Rain, Dahan section "Canny Defense"): "During Ravage, in every land, Defend
  1 per Dahan in the land." Matches the engine (Mountain C7: 2 Dahan, 3
  damage, 1 to the land).

- [x] **Q31. Remnants of a Spirit's Heart.** The engine stored this Event's
  effect as Defend 4 with `AffectsAllLands` (Turn 6). Is it really Defend 4 in
  every land, or one land per board?
  **Answer (sourced, 2026-10-19; Brett delegated):** All lands, on one turn the
  players choose. Card text (Remnants of a Spirit's Heart, choice "With
  Patient Focus Craft a Great Warding", cost 4 Energy per player, aided by
  Earth): "Defend from Spirits is 4 lower per land this turn (min. 0).
  During one future Spirit Phase, players may jointly decide to grant Defend
  4 to all lands that turn." The engine's Turn 6 effect is that grant being
  used.

- [x] **Q32. Distracted by Local Troubles (Terror Level II).** The engine
  stored −1 Ravage damage per Invader with no land list (all lands), and a
  flag `PerDamageTaken` whose meaning is not confirmed. Card text vs engine?
  *Engine (Test 1 Turn 6, Jungle C4):* 2 Explorers dealt 0.
  **Answer (sourced, 2026-10-19; Brett delegated):** The card text differs from the
  companion's rule. Terror Level II and III: "Invaders do -1 Damage per
  Damage they have taken" (plus the land Damage lines). So the reduction is
  1 per point of Damage an Invader has already taken, not 1 per Invader.
  The engine's `PerDamageTaken` flag is this. **Code fix:** the forecast
  (`rules.rs` `ravage_adjustment`) currently applies -1 per Invader; it must
  apply -1 per Damage taken by each dealing Invader. Test 1 Turn 6 matched
  only because the two Explorers had taken damage. *Engine read:*
  `DistractedByLocalTroubles` creates an AdjustRavageDamage effect with
  `perDamageTaken: true`; each Invader deals max(0, its damage − its own
  Damage taken); it applies to any Ravage Action (not only the normal
  Ravage) and to nothing outside a Ravage. **Fixed** in the companion
  (2026-10-19); the Test 1 backtest still matches.


## From the rules register (2026-10-19)

Merged from `register.md` §8 (the rules-register pass over every adversary
level, spirit, scenario, and Event). NQ-E affects the firewall.

Same format as `open-questions.md`. Questions marked *(engine read)* need an IL read, not the physical copy.

- [x] **NQ-A. Sweden L4 Royal Backing timing.** Is the Invader Deck Accelerated after the initial Explore (engine) or during Setup after adding Invaders (Wiki), possibly before the initial Explore? And is the Town added to "the land matching the discarded card" (engine) or "the land of that terrain" (Wiki)?
  *Depends on:* Sweden L4+ deck recipe, which card is gone, Turn 1 Build forecast. *Sources:* engine `TheKingdomOfSweden.levels[4]`: "At the end of Setup, Accelerate … (After the initial Explore, discard …)"; Sweden@12523 L4: "During Setup, after adding all other Invaders, Accelerate the Invader Deck."
  **Answer (sourced, 2026-10-19; Brett delegated):** The engine's version. FAQ
  (Sweden) rephrases Royal Backing: "At the end of Setup (after resolving
  the initial Explore Card), Accelerate the Invader Deck. (Discard the
  topmost card of the lowest Invader Stage remaining in the deck.) On each
  board, add 1 Town to the land with fewest Invaders that matches the
  discarded card." 

- [x] **NQ-B. Habsburg Mining L2: per board?** "Add 1 Disease and 1 City in the highest-numbered land with a Town Setup symbol": on each board (engine) or once (Wiki text)?
  *Depends on:* initial board at L2+. *Sources:* engine: "On each board, Add 1 {Disease} and 1 {City} …"; Habsburg_Mining_Expedition@12138 L2 omits "On each board".
  **Answer (sourced, 2026-10-19; Brett delegated):** On each board. FAQ errata
  (Habsburg Mining Expedition, NI 1st printing): "Add 1 Disease and 1 City
  in the highest-numbered land with a Town Setup symbol on each board; don't
  just add them to the single highest-numbered land … on the entire island."
  (Brett's panel image prints the text without "on each board"; the errata
  adds it. Difficulty per the panel: base 1; L1–L6 3, 4, 5, 7, 9, 10.)

- [x] **NQ-C. The Great River loss count.** More than 3 escaped Towns per starting board (engine) or per player (Wiki)?
  *Depends on:* Great River loss clock with an extra board. *Sources:* engine `TheGreatRiver`: "more than 3 {Town} per starting board"; The_Great_River@12067: "more than 3 {Town} per player".
  **Answer (sourced, 2026-10-19; Brett delegated):** Per board. FAQ ("When playing
  with an extra board"): "The Great River - Additional Loss Conditions - the
  per-player measurement should be per-board … 'If more than (3 x # of
  boards) Towns escape…'". Without an extra board the two are equal.

- [x] **NQ-D. Blitz blight count.** +1 Blight per board on the Blight Card (engine) or per player (Wiki; its footnote says per board when playing with an extra board)? Same for "1 less per starting board" when Blighted.
  *Depends on:* blight runway in Blitz with an extra board. *Sources:* engine `Blitz` Setup/Rules Changes; Blitz@3296 and its querki footnotes.
  **Answer (sourced, 2026-10-19; Brett delegated):** Per board, both lines. FAQ:
  "Blitz - Setup - 'an additional Blight per player' should be 'per board'.
  Rule Changes - '1 less Blight per player' should be 'per board'." 

- [x] **NQ-E. Is an Accelerated card seen?** When the deck is Accelerated (Sweden L4, Elemental Invocation, Despicable Theft), does the card go face-up to the Invader discard or out of the game unseen?
  *Depends on:* firewall (may the companion name it?) and card counting (does it leave the unseen pool as a known card?). *Sources:* engine texts say "Discard the topmost card"; Sweden@12523 ruling says "discard"; Elemental_Invocation@3350 and Despicable_Theft@12058 say "Remove the topmost card".
  **Answer (sourced, 2026-10-19; Brett delegated):** Seen. Accelerate discards the
  card into the Invader discard (JE rulebook definition). FAQ (Elemental
  Invocation): the rulebook's "discarded" beats the reminder text's
  "removed", "rules text trumps reminder text". The companion may name the
  card, and it leaves the unseen pool as a known card. Exception (FAQ):
  Salt Deposits can't be Accelerated out; discard the next Stage II card (or
  the top-most Stage III) instead.

- [x] **NQ-F. Isolate and cascades in the engine.** *(engine read)* Does `CascadeBlight`'s `GetAdjacentIslandLocations` ignore Isolate, as the Wiki rule requires?
  *Depends on:* cascade targets on Isolated lands. *Sources:* Blight@7555 and Isolate@4040: "Isolate does not affect Blight cascades"; engine not read.
  **Answer (engine read, 2026-10-19; static CIL of the DLL, eac8e2ed…):** Yes, cascades ignore
  Isolate, matching the tabletop rule. `GameController/<CascadeBlight>` calls
  `GetAdjacentIslandLocations` with `shouldApplyIsolate: false`
  (`isForBlight: true`); `BaseController::AskIfLandsAreAdjacent` only vetoes
  adjacency for Isolate when that flag is true.

- [x] **NQ-G. Do Towns added by a before-Explore Escalation count as sources for that Explore?** (Brandenburg-Prussia Land Rush, Russia, Scotland; England's Escalation is a Build.)
  *Depends on:* Explore odds on Escalation turns. *Sources:* Sequence_of_Play@12144 "first perform the Escalation effect"; adversary rulings "happens before Exploring"; Invader@9428 FAQ (pieces added *by the card's own Explore* are not sources). No source states the Escalation case.
  **Answer (sourced, 2026-10-19; Brett delegated):** Yes. FAQ (Stage II escalation):
  "Do Stage II Escalations happen before or after the Explore? If it is not
  specified, it happens before the Explore. This means that, e.g.,
  Brandenburg-Prussia can add a Town that then acts as a source for the
  Exploration that happens immediately afterwards." So Towns/Cities from
  Prussia (Land Rush), Scotland (Ports Sprawl Outward) and England (Building
  Boom) count as sources that turn. Russia's adds Explorers (never sources);
  Sweden, France and Habsburg Livestock say "after Exploring".

- [x] **NQ-H. Accumulated Devastation: first Ravage, or first successful Ravage?** If the first matching Ravage on a board is skipped, does the bonus move to the next one?
  *Depends on:* Ravage ledger on that Event turn. *Sources:* engine: "During the first Ravage Action …"; accumulated_devastation@unrecorded (Wiki cache): "During the first successful Ravage Action (in one land) this Invader Phase …".
  *Still open (physical card):* engine "During the first Ravage Action"; Wiki cache "first successful Ravage Action (in one land)". The FAQ is silent. Related rule (JE p.7, FAQ): per-land modifiers never raise 0 Invader Damage above 0.
  **Answer (Brett's rule, 2026-10-19):** The printed card governs unless explicit errata changes it (then the errata, which Steam usually carries).
  No errata found (FAQ page for the card has none), so use the printed text
  (Wiki transcription, Nature Incarnate): "On Each Board: During the first
  successful Ravage Action (in one land) this Invader Phase, increase the
  total Damage ravaging Invaders deal to each land (but not to Dahan) by the
  current Invader Stage. Do this for at most one Ravage Action per board."
  A skipped or zero-damage Ravage doesn't use it up; Steam's shorter text is
  not errata. The card's only FAQ entry (Brett,
  https://querki.net/raw/darker/spirit-island-faq/.7w4gec1) is the JE p.7
  "Ravage Timing" ruling, a clarification rather than a text change:
  per-Invader modifiers apply first, and a per-land bonus like this one never
  raises 0 Invader Damage above 0 (engine agrees, NQ-L).

- [x] **NQ-I. Land Health wording (Depleted Soil, The Land's Bounty Exhausted, blight card All Things Weaken).** Engine: "the land has -1 Health". Wiki cache: "it only takes 1 Damage to add Blight". They differ only when another land-Health change applies (e.g. Vital Strength Resilience +8, All Things Weaken -1). Which applies then?
  *Depends on:* blight threshold per land. *Sources:* engine `CitiesRise`, `FortuneSeekers`, `AllThingsWeaken` ("the land have -1 Health"); cities_rise@unrecorded, fortune-seekers@unrecorded, all_things_weaken@unrecorded ("The land takes Blight on 1 less Damage").
  **Answer (sourced, 2026-10-19; Brett delegated):** NI rulebook p.14: "The land has
  -1 Health" for All Things Weaken and The Land's Bounty Exhausted / Depleted
  Soil; Resilience +8 at Sacred Sites; "No matter how many modifiers you
  apply to the land Health, the final total can't go below 1 Health." The
  engine's wording is right; modifiers add together (see Q18).

- [x] **NQ-J. Blitz: does the setup card Ravage on Turn 1?** Setup is "Explore, then Build and Explore", so the setup card would reach the Ravage space for Turn 1 *(inference)*.
  *Depends on:* Turn 1 Ravage forecast in Blitz. *Sources:* engine `Blitz` Setup Changes; Blitz@3296 (same text); card flow Sequence_of_Play@12144 §4.
  **Answer (sourced, 2026-10-19; Brett delegated):** Yes (inference from the card
  text, to verify in a Steam Blitz game). Blitz Setup Changes: "The Invaders
  get an additional set of actions at the end of Setup. (So they will
  Explore, then Build and Explore.)" A full set ends by advancing the cards,
  so the first card moves Build → Ravage and Ravages on Turn 1. FAQ (Blitz):
  setup Explores/Builds are not the Build/Explore Steps, so adversary
  effects tied to those steps don't apply during that extra set.

- [x] **NQ-K. Downpour's Drench the Landscape and Invader Cards.** Does Downpour's Sacred Site match a Wetland Invader Card? The text names "Spirit Actions and Special Rules" only.
  *Depends on:* Build/Ravage/Explore lists with Downpour. *Sources:* engine `DownpourDrenchesTheWorld.specialRules[DrenchTheLandscape]`; downpour-drenches-the-world@unrecorded (same text).
  **Answer (sourced, 2026-10-19; Brett delegated):** No. The text (engine and Wiki
  agree): "Spirit Actions and Special Rules treat your Sacred Sites as
  Wetlands." Invader Cards drive Invader Actions, not Spirit Actions, so a
  Wetland Invader Card does not match Downpour's Sacred Sites.

- [x] **NQ-L. Per-land Ravage modifiers only when the Invaders deal 1+ Damage.** *(engine read)* Does the engine add per-land bonuses (Events such as Increasing Aggression, Habsburg L6 +2) only when the Invaders' own total is ≥ 1, as JE p.7 says?
  *Depends on:* Ravage ledger with Strife or tiny Invader groups. *Sources:* Defend@3397 "Ravage Timing"; engine `GetRavageDamageModification` condition not read.
  **Answer (engine read, 2026-10-19; static CIL of the DLL, eac8e2ed…):** Yes, gated as JE p.7
  says. In `RavageAction`'s coroutine each Invader's damage (base + its own
  adjustments) is floored at 0 and summed; per-land adjustments
  (`GetGeneralDamageAdjustment`: Habsburg Livestock L6 +2, Increasing
  Aggression +1, …) are added only when that total is > 0; Defend is
  subtracted after. The companion's Ravage ledger now does the same.

- [x] **NQ-M. Isolated lands as Explore targets in the engine.** *(engine read)* The engine source predicate excludes Isolated lands; does the target side also exclude them?
  *Depends on:* Explore odds with Isolate. *Sources:* Isolate@4040: "Invaders do not Explore Isolated lands"; engine_state_model.md Q3 (sources only).
  **Answer (engine read, 2026-10-19; static CIL of the DLL, eac8e2ed…):** Yes. `GameController::
  CanLandBeExplored` returns `!IsIsolated` (only Skip effects override it),
  and `ResolveInvaderCardAction::CacheExploreSourceLands` filters targets by
  it; Isolated lands are neither sources nor targets. The companion's
  Explore forecast now excludes both (Q3).

- [x] **NQ-N. Breath of Darkness's Incarna "may count as Beast".** In the engine text only; the Wiki copy (2026-04-19) lacks it.
  *Depends on:* Russia L1/Escalation (lands with Beast), Beast Events and Fear Cards. *Sources:* engine `BreathOfDarknessDownYourSpine.specialRules[TerrorStalksTheLand]`: "It may count as {Beast}."; breath-of-darkness@unrecorded.
  *Still open (NI panel):* engine "It may count as Beast"; the FAQ and the Wiki copy don't mention it.
  **Answer (Brett's rule, 2026-10-19):** The printed card governs unless explicit errata changes it (then the errata, which Steam usually carries).
  No errata adds "It may count as Beast" (the FAQ's only Breath of Darkness
  errata is about where escaping pieces Move). Use the printed text as
  transcribed on the Wiki, without it; the Steam text's addition is a
  Steam difference. Low-priority physical check when convenient.

- [x] **NQ-O. Wandering Voice text differences.** Engine: Spread Tumult adds Strife when the Incarna moves "to a land"; Wiki: "to a land with Invaders". Engine: the Incarna "may count as {SacredSite}"; Wiki copy lacks it.
  *Depends on:* where Dahan stop counterattacking; Sacred Site targeting. *Sources:* engine `WanderingVoiceKeensDelirium.specialRules`; wandering-voice@unrecorded.
  **Answer (sourced, 2026-10-19; Brett delegated):** *Strife part:* no practical
  difference: Strife can only be added to an Invader ("If there are no
  Invaders, you cannot add Strife", JE rulebook), so "to a land" and "to a
  land with Invaders" play the same.
  **Answer (Brett's rule, 2026-10-19):** The printed card governs unless explicit errata changes it (then the errata, which Steam usually carries).
  Sacred Site part: no
  errata found adding "may count as Sacred Site" to the Incarna, so use the
  printed text as transcribed on the Wiki, without it; Steam's addition is
  a Steam difference. Low-priority physical check when convenient.

- [x] **NQ-P. Brandenburg-Prussia L2's moved Stage III card.** *(engine read)* `shouldMovedInvaderCardOverrideStage: true`: does the moved card count as Stage III, or as the Stage of the slot it sits in, for "current Invader Stage" effects (R-TS-13) and for Escalation?
  *Depends on:* Stage-keyed Events, Accumulated Devastation, card counting. *Sources:* engine `TheKingdomOfBrandenburgPrussia.levels[2]`; Wiki silent.
  **Answer (engine read, 2026-10-19; static CIL of the DLL, eac8e2ed…):** The moved Stage III card
  gets `OverrideStage = 2` (`GameModifier::MakeSetupInvaderDeckAdjustments`),
  so while it is the next or current Explore card, "current Invader Stage"
  is 2; Prussia's `ClearOverrideStageResponse` clears it once the card leaves
  the Explore slot (Stage III in Build/Ravage). Escalation ignores the
  override: the leading adversary's Escalation needs the card's printed
  Escalation flag (Stage III cards have none), and a supporting adversary's
  Stage-III Escalation keys on the printed Stage 3, so it does fire there.

Existing questions this pass touched (no new answer): Q1, Q2, Q4 (R-RA-2 supports "Defend reduces both"), Q5, Q10, Q11, Q16 (England L6 difficulty 10 vs 11 confirmed against Wiki England@6844), Q17 (France loss, confirmed against France_(Plantation_Colony)@7940), Q18 (Sweden L1 and Habsburg loss wording, confirmed against Sweden@12523 and Habsburg_Monarchy_(Livestock_Colony)@6850), Q19, Q20, Q22, Q24.
