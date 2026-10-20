# C. Information-design referee: the companion screens

Judged against Brett's words (brett-messages.md), the native-app spec §1/§3, the companion spec §7.5/§7.6/§7.10, questions-from-play.md, and the "displays that build judgment" section of the stat-models report. Screenshots: l-hme, v3-prussia, v3-england, c-27, c-hme, lab-0-drawer, lab-8-t4-two.

## 1. Verdict on what was built

**What every screen is organized around: resource budgets.** The page is called "The Ledger" and its rows are Blight, Fear, Invader deck, and one adversary clock. That is an inventory of what each pool holds. Not one of Brett's questions (questions-from-play.md) was "how much is left"; every one was "given my options, what happens at the coming step, and what's coming after". The spec itself says the ledger is "the engine of every column, not a view of its own". The build made the engine the page.

**Per screenshot.**

*l-hme.png (first Ledger, HME L4 T3 Ravage).*
- Eye meets first: "Turn 3", then four big numerals (3, 7, 8, 1) in four hues. Four instruments, no spine.
- Encodings to learn: four different token glyphs (ghost, mask, card, explorer), lit/dim/red/ghost token states, bracket labels under tokens, step chips ("Ravage · now"), land chips in terrain color, a delta numeral, a green "solve" pill, and two lines of pocket-rule prose per lane. About nine encodings on one page; zero shared axes.
- Text to graphic: roughly 60% of the ink is sentences.
- Verdict sentences replacing graphics: "−2 this turn · holds, 1 left after", "prevent 1 to hold", "7 → 8 of 8 Invaders + Blight in NE7 Mountain".
- Already on the game's screen: Blight left, Fear to next card, cards left in the deck, the NE7 piece count.
- Not available from the game: the exact Defend per land, next turn's ghost spend, the blighted-side distribution ("23 unseen cards: 1×2 · 7×4 …", shown as text).
- The one thing that mattered here, that Land Stripped Bare ends the game at T4 before Blight ever does, is not drawn; it has to be inferred by reading two lanes and doing arithmetic.

*v3-prussia.png / v3-england.png (Ledger + Runway + Pace).*
- Now three turn axes on one page, none aligned, with different origins and scales: Runway (T5–T8, y = Blight left), Pace (T1–T8 or T1–T12, y = Fear total), Invader deck (T5, T6, T7 as card tiles). A reader cannot look down a column and see "turn 6".
- Encodings: everything from l-hme plus glowing lines, a shaded P(loss) zone (saturation = probability, the weakest Cleveland–McGill channel), dashed projections in two colors, a stacked bar under a line, a mini slope chart with red dashed tails, and a side list of next-card deltas in terrain colors. Six meaning-bearing hues (purple, red, amber, green, sky, teal) plus terrain colors in running text.
- Verdict sentences: "Unanswered, the card is already on its blighted side. Lost to Blight by: T5 8 in 10 · T6 10 in 10 …"; "Victory needs 29 more Fear in 4 turns … 7.2 a turn. Last 3 turns: 5, 7, 5 (5.7 a turn). At that pace: Terror III on T8; the deck runs out before Victory." Each is the chart restated, and the sentence, not the chart, is what the reader actually uses.
- Already in the game: 1 Blight left, 1 Fear to go, 3 cards left, Terror II.
- Not in the game: the per-order enumeration, the required-slope line. These are the right models drawn in the wrong place, on their own axes, under their own prose.
- The red pulsing tokens in the Blight lane (five of them) are the loudest thing on the page and encode "this turn spends past the card", which the game's own Blight card already makes obvious.

*c-27.png / c-hme.png (the Curve page).*
- Four charts, three y-units (lands, Energy/plays, multiples of T1), one diverging bar pair, and a ratio band "1.5 to 3.5 Energy per play" that in c-hme widens until it says nothing. c-hme also has a rendering fault (orphan "Inv." label, a floating bar block, a −∞ bar at T11).
- The question Rei's curve answers, "on which turn does my economy outgrow the step?", is a crossing. It is never drawn as a crossing; it is split across two charts in different units and then collapsed into "×2.0 vs ×3.0" multiples, which are abstract and attach to no decision. "Growth: Invaders +20% a turn, you +20%: about even" is the verdict sentence doing the chart's job again.
- Nothing on this page is per option. Brett's pace question was always "Reclaim now vs later", a comparison of two lines; this page has one line.

*lab-0-drawer.png (what-if drawer).*
- The drawer is the island as a 12-row table: Explorer/Town/City counts per land with "+Defend" and Dahan buttons. That is the thing Brett said not to build ("we don't need to reinvent an entire game inside of a companion"). It is manipulation software (Victor) where the task is information software.
- The lever offered is "destroy a piece". A player cannot do that; a player plays a card, picks a growth, chooses a target. The companion's lever has to be the player's actual options.

*lab-8-t4-two.png (two tried changes).*
- Before/after is shown as strikethrough text, "(was 39)", "was 0", "+" prefixes, and a status-bar sentence "What if: Destroy Explorer in NE7 · Destroy Explorer in NE7". Deltas as text, on top of a page already full of text. And the Blight lane now pulses eight tokens red; the what-if made the page louder, not clearer.

**The single biggest reason it reads as "strange".** There is no spine. The game's whole tension is a race between endings (Invaders win by Land Stripped Bare at T4; Blight loss at T5; deck at T11; Fear victory never), but the four endings are drawn as four unrelated instruments on four different axes, so the race is never on screen. The reader has to assemble it from lanes and then the prose assembles it for him. Everything else (nine encodings, six hues, three turn axes, verdict sentences, piece-editing) follows from that missing spine: with no shared axis to hang on, each model got its own chart and its own caption.

A second reason, nearly as large: the only lever on screen is a piece edit, so the screens never show "my plan vs the coming step", which is the question Brett asked at every single decision.

## 2. The information architecture

Three layers, by Shneiderman's overview → zoom → details, each owning one question.

### Always-on: THE RACE (top strip, ~150 px)

Question: *Which ending arrives first, and on which turn, if nothing answers it?* (Brett: "the backbone of the competing goals is what is key"; "comparison of how fast we grow vs the opponent".)

Content: one horizontal turn axis from now to the deck's last Explore. One row per ending, Invader-side rows above the axis, Spirit-side rows below, fixed order, never re-sorted:
- Invaders win: each adversary loss condition in play (Land Stripped Bare, Proud & Mighty Capital, …), Blight (card empties on its blighted side), Invader deck (next Explore with no card).
- Spirits win: Fear victory (Terror III + deck empty), board clear at each Terror Level.

Each row is a quantile strip along the axis: dots at the turn where that ending arrives, one dot per 1/10 (or 1/20) of enumerated card orders ("54 of 60 by T5"). A deterministic ending (the deck) is one solid tick. The row whose dots arrive first is marked *binds* with one bracket. A ghost version of every row is kept for overlay when an option is hovered.

Models: deck recipe (certain); `outlook.rs` enumeration for Blight; the adversary-clock enumeration; `pace.rs` for Fear and board-clear. Competing-risk semantics (stat-models report): rows are cumulative incidence by turn from the same enumerated futures, so they never sum past the number of orders.

Wins and losses sit on one track because that is how the game is decided: the first dot on the axis ends it. The ribbon (phase) and the status bar also belong here.

### Per-moment: THE DECISION (center, ~600 px)

Question: the one Brett asked at that phase (questions-from-play.md), printed as the panel title. One screen per moment:

| Moment | Title (the question) | Lever columns | Goal rows (first row = the binding ending) |
|---|---|---|---|
| Growth | Which growth fits this turn? | the 2–3 growth options | binding ending; the land(s) at stake this Ravage; card after the turn; hand/plays/Energy; cards run out (forced Reclaim) turn; elements/innate reach |
| Draft / Forget | Which card, and what to Forget? | the 4 drawn cards (and each Forget candidate) | what it does on this board this turn; what it pairs with; what Forgetting costs |
| Fast | What can I still change before this Fast phase ends? | "do nothing", the player's plan, each Fast play that reaches a Ravage/clock land | binding ending; Blight after this Ravage; Dahan/Presence kept; Fear earned; Energy/plays left; Slow still possible |
| Event / Fear choice | Which placement, which push? | each legal choice | same rows, re-run per choice |
| Invaders (Ravage/Build/Explore) | What happened vs the forecast? | none (one column: forecast vs actual) | the same rows |
| Slow | What sets up next turn? | each Slow play | next turn's Ravage/Build lands; the clocks at the T+1 check |
| Review | Where did it turn? | one column per turn | the race strip replayed; flags F1–F10 as chips |

Layout is §7.5 as geometry, not prose:

```
┌ FIXED (face-up) ─┬─ LEVERS: options side by side, fixed order ─┬─ COUNTED (hidden) ─┐
│ solid shapes,    │ a consequence table:                        │ dot strips "N of M"│
│ hard edges,      │   rows   = goals (race endings first,       │ one per unseen     │
│ exact numbers    │            then what the option buys)       │ deck: Minor/Major, │
│ the lands at     │   columns= options (never ranked)           │ Fear cards, Event, │
│ stake, drawn as  │   cells  = a graphic, never a sentence      │ next Invader card  │
│ 8-slot stacks /  │   factors common to all columns are drawn   │ with the pocket    │
│ damage ledgers   │   once, spanning (Hammond–Keeney–Raiffa)    │ rule under each    │
└──────────────────┴─────────────────────────────────────────────┴────────────────────┘
```

How the decision attaches to the binding goal: the first goal row is always the race's binding ending, in the same encoding (dots on the turn axis). Hovering any option cell overlays that option's futures on THE RACE above (ghost = now, solid = after). So the player sees, in the same strip, whether the option moves the binding dot later, moves another ending earlier, or changes nothing. That is "deltas, not verdicts".

How the player's plan is the lever: in the Fast screen the first lever column is "do nothing" (the reference), the second is "your plan", built by clicking the cards in hand listed in the FIXED column (a card is a lever the player really has; a piece edit is not). Growth options, drafted cards and Event choices are levers the game itself enumerates, so those screens need no composing.

Pace is a row inside the Growth and Draft tables ("cards run out on T_", "Energy/plays by turn"), on the shared axis; it is not a page.

### On-demand: DETAILS (hover, click, one key)

Question: *Why is that number what it is?*
- Hover a land chip → its ledger: damage, Defend, Dahan, the exact save, the rules in play that change it (Sweden's Fine Steel, Mining Boom, "checked at the end of each Fast phase"). This is where the current Ledger lane and its pocket rule live.
- Hover a race row → the budget lane (tokens, this turn's spend, next turn's ghosts) and the per-order breakdown ("if T5's Explore card is Jungle+Sands …").
- Hover a dot strip → the count and the pocket rule ("27 of 96; four looks: about 3 in 4").
- Click the pace row → the economy slopegraph per option against the deck's step (the surviving part of the Curve).
- Bottom strip, always visible but one line: three think-about prompts from the §7.6 catalog chosen for this moment, each a question, none answered.
- `H`: the optional predict-first hold (blur the COUNTED column until a guess is typed).
- Review mode replays all of the above per turn.

## 3. The visual grammar

Rules the whole app obeys. Each one is there because it changes a decision on the screens above.

1. **Color means side, nothing else.** Invader-side endings and pressure: one warm hue. Spirit-side endings and economy: one cool hue. The accent (purple) marks only *now* and *the option under the cursor*. Grey is "reference / do nothing". Terrain colors appear only on land chips, matched to Steam. Severity, probability and "warning" are never colors; red glow, pulsing and amber callouts go. (Few: one visual vocabulary per dashboard; Bertin: a retinal variable carries one meaning.)
2. **Horizontal position means turn, on one shared axis.** Every panel that has a time dimension uses the same x-mapping, the same origin (now) and the same end (the deck's last Explore), vertically aligned so a column is a turn (Tufte, small multiples on a common scale; Cleveland–McGill: position on a common scale is the most accurately read channel). No panel gets its own turn axis.
3. **Certainty is drawn by edge.** Face-up facts are solid shapes with hard edges and an exact number (an 8-slot stack for Land Stripped Bare with 6 filled). Counted unknowns are dots, hatched slots, or ghosts, always labeled "N of M". Nothing uncertain is ever drawn solid.
4. **Futures are quantile strips.** A distribution over turns is 10 (or 20) dots placed on the turn axis, each dot one tenth of the enumerated orders (Kay/Fernandes quantile dotplots: judged at 97% of optimal; text uncertainty underperforms). The count sits beside the strip in natural frequency, "54 of 60 orders", never "90%" alone (Gigerenzer/Hoffrage: natural frequencies triple correct reasoning).
5. **Before/after is an overlay.** The reference ("do nothing" / "now") is a ghost under the solid tried state, in the same place. For per-option economies over turns, a slopegraph with one line per option. Never strikethrough, never "(was N)".
6. **Magnitude is length on a common baseline.** Blight left, damage vs health, Energy, plays: bars or filled slots. Never area, never saturation, never a glow radius.
7. **Numbers appear exactly twice.** As the count beside a dot strip or slot stack, and as the cost of a lever (Energy, plays). Every other number is a hover. There are no sentences under charts; a chart that needs a caption is redrawn.
8. **Type: three sizes.** Title 15 (the question), body 13, small 11.5 for axis labels; one big numeral size reserved for the FIXED column. Mono for counts.
9. **Density: one screen, 1600×900, no scroll.** At most 7 labeled groups on screen (the race, three columns, the prompt line, the ribbon, the status bar), at most 3 encodings to learn (slot stack, dot strip, slopegraph), and at most 5 option columns. Lists longer than that scroll inside their cell.
10. **Options are a consequence table.** Goals as rows, alternatives as columns, fixed order by lever, factors common to all columns drawn once spanning them so the eye drops them (Hammond, Keeney & Raiffa, *Smart Choices*: consequence table, then eliminate dominated and common factors). The table lets the player see dominance; the app never computes it.
11. **Never on screen:** a ranking, a highlighted "best", a default selection, a verdict sentence ("holds", "grim", "about even", "at that pace…"), a bare percentage, the island map, the hand as a panel, a piece editor, a log stream, a multiple-of-turn-1 ratio.
12. **The question is the title.** Every per-moment panel is headed by the player's question verbatim from the catalog, and every element in it is labeled by the sub-question it answers (Victor, *Magic Ink*: information software answers a question the user is already asking; the graphic is the answer, context decides what is shown).

Of the cited principles, the ones that changed a decision here: Cleveland–McGill (position over saturation: kill the P(loss) shading, draw dots on the axis); Kay et al. (dot strips instead of lines-per-outcome and "8 in 10" sentences); Gigerenzer (N of M everywhere, with the pocket rule as the hover); Hammond–Keeney–Raiffa (the per-moment panel is a consequence table, which is also how "never ranked" becomes a layout rather than a rule); Tufte's shared axis (one turn axis, which is what makes the race a race); Victor (the lever must be a thing the player can do, so the piece editor goes); Shneiderman (which layer each existing piece belongs to); Few (one screen, one vocabulary, no glow).

## 4. Wireframes for the first two screens

Both at 1600×900. Column scale ≈ 10.7 px per character, row scale ≈ 19 px. Legend inside each frame: `■` face-up fact, `▨` added by a face-up card, `░` added in every enumerated order, `●` one tenth of the orders/draws, `○` a tenth that does not arrive, `┆` now, `[A]` always-on, `[M]` per-moment, `[H]` hover/click. Cells marked *illustrative* depend on the hand or the recipe, which the engine supplies; counts given in the brief are used as given.

### (a) Fast phase, Habsburg Mining Expedition L4, Turn 3

```
px  0                                                                                                                 1600
    ┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
  0 │ T3   Spirit ── ●FAST ── Event ── Fear ── Ravage ── Build ── Explore ── Slow       Habsburg Mining L4 · Lightning + River │[A] ribbon
 36 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ THE RACE   which ending arrives first, unanswered?                                                 60 card orders │[A] title
    │            ┆T3        T4        T5        T6        T7        T8        T9        T10       T11      beyond       │[A] one turn axis
    │ INVADERS   ┆                                                                                                       │
    │ Land Stripped Bare  ┆          ●●●●●●●●●● 60 of 60     ◄ BINDS   NE7 reaches 8 at T4's end-of-Fast check           │ model: adversary clock, every order
    │ Blight card empties ┆          ○           ●●●●●●●●●○ 54 of 60 by T5                                              │ model: outlook.rs, no Spirit action
    │ Invader deck ends   ┆                                                                                 ▌ T11 certain│ model: deck recipe
    │ SPIRITS    ┆                                                                                                       │
    │ Fear victory        ┆                                                                                   ○ 0 of 60 before T11 │ model: pace.rs
    │ Board clear, Terror II ┆                                                                                ○ none yet │ model: pace::blockers
190 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ FAST · T3    What can I still change before this Fast phase ends?                     binding: Land Stripped Bare, T4 │[M] the question
    ├───────────────────────────┬──────────────────────────────────────────────────────────────────┬─────────────────────┤
    │ FIXED  face-up            │ YOUR PLAN   options side by side, fixed order, unranked          │ COUNTED  hidden     │
    │                           │ do nothing │ your plan      │ A  Fast → NE7      │ B  Fast → NW3   │                     │ columns: the reference, the plan
    │ NE7 Mountain              │ (reference)│ (click cards   │ remove 1 Invader   │ Defend 2        │ T3 Explore card     │   the player composes, then
    │  ■■■■■■▨░  6 of 8         │            │  at left)      │                    │                 │  ●●○○○○○○○○ 1 of 5  │   each Fast play in hand that
    │  ■ now  ▨ T3 Build (face-up Mountain)  ░ T4 (every order)                                     │  each terrain       │   reaches a land in the FIXED
    │  checked: end of each Fast phase                                                               │  Coastal·Mtn·Sands  │   column (si-corpus reach rules)
    │                           │            │                │                    │                 │  Jungle·Wetland     │
    │ NW3 Wetland  Ravage now   │            │                │                    │                 │                     │
    │  damage ■■■ 3 vs health ■■ 2  → Blight; Defend 2 saves it                                      │ Event               │
    │                           │            │                │                    │                 │  common shift,      │ common factor: drawn once,
    │ Blight card  ■■■ 3 left   │            │                │                    │                 │  same for every     │   spans all columns
    │  this Ravage −2 (NW3, NE7)│            │                │                    │                 │  column             │
    │                           │            │                │                    │                 │                     │
    │ HAND (levers)             │ ── rows are the goals; the race's binding ending first ──────────── │ Fear cards banked   │
    │  Lightning: [card][card]  │                                                                    │  none · 7 Fear to go│
    │  River:     [card][card]  │ Land Stripped Bare  T4 ●●●●●●●●●● │ ·          │ T5 ●●●●●●●○○○ *   │ T4 ●●●●●●●●●● │     │ * illustrative: engine fills
    │  (Fast only, cost shown)  │ Blight after T3     ■■■→■ 1 left  │ ·          │ ■■■→■ 1 left      │ ■■■→■■ 2 left  │     │   from the hand and the ledger
    │  Energy  ■■■■ 4 · plays 2 │ Blight loss by T5   ●●●●●●●●●○    │ ·          │ ●●●●●●●●○○        │ ●●●●●●○○○○     │     │   (ledger.rs, needs.rs,
    │                           │ Dahan kept          ■■ 2          │ ·          │ ■■ 2              │ ■■ 2           │     │    whatif.rs recompute)
    │                           │ Fear this turn      ·             │ ·          │ +1                │ ·              │     │
    │                           │ Energy · plays left 4 · 2         │ ·          │ 2 · 1             │ 3 · 1          │     │
    │                           │ Slow still possible 2 cards       │ ·          │ 1 card            │ 1 card         │     │
    │                           │                                                                    │                     │
    │                           │ hover a column → its futures overlay THE RACE (ghost = now, solid = after)              │[H]
810 ├───────────────────────────┴──────────────────────────────────────────────────────────────────┴─────────────────────┤
    │ think about:  Which clock does it move, and by how much?  ·  Is this factor common to all options?  ·  Does it remove the problem or move it? │[A] prompts (§7.6)
870 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ Habsburg Mining Expedition L4 · Lightning's Swift Strike, River Surges in Sunlight          live · save 165/240 · H hold │[A] status
900 └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

[H] hover NE7 chip   → ledger flyout: 4 Explorers · 2 Towns · 0 Blight = 6; +1 T3 Build (Mountain face-up); +1 T4 Explore in every
                        order; Land Stripped Bare: "8 Invaders + Blight in one land, checked at the end of each Fast phase";
                        Mining Boom and the Habsburg L4 modifiers that apply here, each named with its number.
[H] hover a race row → the budget lane (tokens, this turn's spend bracketed by land, next turn's ghosts) and the per-order
                        breakdown ("if T3's Explore card is Coastal: …"). This is where the current Ledger lane lives.
[H] hover a dot strip→ "54 of 60 orders · pocket rule: 1 ÷ unseen cards in this stage".
```

What this screen says without prose: the top dot row ends the game at T4 in every order, a full turn before Blight; the FIXED stack shows the 6→7→8 count and that the 7th slot is already face-up; the plan column is empty until the player fills it; the two Fast plays that reach NE7 or NW3 are shown as what they do to each goal, and only A moves the binding row.

### (b) Growth, Test 1 Turn 3, Shadows Flicker Like Flame vs Brandenburg-Prussia L2

Growth options from the wiki data: G1 Reclaim All + Gain a Power Card; G2 Gain a Power Card + Add Presence (range 1); G3 Add Presence (range 3) + 3 Energy. Tracks: Energy 0·1·3·4·5·6, Plays 1·2·3·3·4·5 (wiki scrape; verify against the physical board before shipping). Draw counts from the brief: 27 of 96 unseen Minors save C4 and are playable now (17 Defend-only); Fear cards 16 of 50 at Terror I. Derived, exact: a draw of 4 holds one of 27 in 1 − C(69,4)/C(96,4) = 74% ("about 7 of 10"); one banked Fear card saves C4 in 16 of 50 (32%), two cards in 1 − C(34,2)/C(50,2) = 54%; independent decks combine by multiplying misses.

```
px  0                                                                                                                 1600
    ┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
  0 │ T3   ●GROWTH ── Fast ── Event ── Fear ── Ravage ── Build ── Explore ── Slow              Brandenburg-Prussia L2 · Shadows │[A] ribbon
 36 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ THE RACE   which ending arrives first, unanswered?                                                  enumerated orders │[A]
    │            ┆T3        T4        T5        T6        T7        T8        T9        T10       T11      beyond       │[A] one turn axis
    │ INVADERS   ┆                                                                                                       │
    │ Blight card flips   ┆●●●●●●●●●● this Ravage (C1, E3, C4)   ◄ BINDS      healthy side: 3 left, −3 unanswered       │ model: ledger.rs (face-up)
    │ Blight loss (blighted side) ┆                    ○○●●●●     ●●●●○○  *illustrative; outlook.rs                      │
    │ Invader deck ends   ┆                                                                            ▌ T(recipe) certain│ model: deck recipe, Prussia L2
    │ SPIRITS    ┆                                                                                                       │
    │ Fear victory        ┆                                                                                   ○ not before the deck │ model: pace.rs
    │ Board clear, Terror I ┆                                                                                 ○ none yet │
190 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ GROWTH · T3    Which growth fits this turn?                                          binding: Blight card flips, T3   │[M] the question
    ├───────────────────────────┬──────────────────────────────────────────────────────────────────┬─────────────────────┤
    │ FIXED  face-up            │ OPTIONS   fixed order, unranked                                  │ COUNTED  hidden     │
    │                           │ G1 Reclaim All + Minor │ G2 Minor + Presence r1 │ G3 Presence r3 + 3 Energy │                     │
    │ C4 Sands  Ravage this turn│                        │                        │                        │ MINOR DECK  96 unseen│
    │  damage ■■■ vs health ■■  │                        │                        │                        │  4 seen (T1 draw)   │
    │  3 Dahan ■■■  needs Defend 2                        │                        │                        │  saves C4, playable │
    │ C1 Jungle · E3 Wetland    │                        │                        │                        │  now: 27 of 96      │
    │  Blight regardless ■■     │                        │                        │                        │  ●●●○○○○○○○         │
    │                           │                        │                        │                        │  Defend-only 17     │
    │ Blight card  ■■■ 3 left   │ ── goals ───────────────────────────────────────────────────────── │  other saves  10    │
    │  healthy side             │ C4 saved by the draw   ●●●●●●●○○○ 7 of 10 │ ●●●●●●●○○○ 7 of 10 │ ·  no draw    │  (Lure, Cycles …)   │
    │                           │ C4 saved by Fear cards  ─────── 1 card ●●●○○○○○○○ 3 of 10 · 2 cards ●●●●●○○○○○ 5 of 10 ─────── │  a draw of 4 holds  │ common factor: spans
    │ Builds  E1 only (Disease C2)│ Event                  ─────────────────── common shift ───────────────────────────── │  one: ●●●●●●●○○○   │   all three columns
    │                           │ C4 saved, all sources  ●●●●●●●●○○ 8 of 10 │ ●●●●●●●●○○ 8 of 10 │ ●●●○○○○○○○ 3 of 10 │  7 of 10            │
    │ HAND now                  │ Card after T3  holds   ■■■→■ in 8 of 10   │ ■■■→■ in 8 of 10   │ ■■■→■ in 3 of 10   │                     │
    │  [Mantle][Crops][Bats] *  │                flips   ■■■→□ in 2 of 10   │ ■■■→□ in 2 of 10   │ ■■■→□ in 7 of 10   │ FEAR DECK 50 unseen │
    │  discard: [Conceal][Favors]│                        │                    │                    │  Terror I           │
    │  Energy ■ 1 · plays 2 *   │ hand · plays this turn 6 · 2               │ 4 · 2  (3 if r→plays)│ 3 · 2  Energy +3   │  saves C4: 16 of 50 │
    │                           │                        │                    │                    │  ●●●○○○○○○○         │
    │ TRACKS  (wiki; verify)    │ Energy a turn  T3 T4 T5 T6 ┆ along the energy track / plays track (slopegraph, shared axis) │ banked now 1,       │
    │  Energy 0·1·[3]·4·5·6     │   G1 ──────────── 1 1 1 1  flat │ G2 ──╱──── 3 4 5 6  │ G3 ──╱──── 3 4 5 6 (+3 now)    │  +1 before Ravage   │
    │  Plays  1·[2]·3·3·4·5     │   deck step ▁▁▂▂▃▃ lands named a turn (curve.rs), same axis                            │  = 2 at the Ravage  │
    │                           │ Cards run out  ┆T3   T4   T5   T6   T7                                                 │                     │
    │                           │   (forced Reclaim) G1 ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬ T6  │ G2 ▬▬▬▬▬▬▬▬▬▬ T5  │ G3 ▬▬▬▬ T4 * │                     │ "postponing Reclaim"
    │                           │ Innate reach  Darkness Swallows: elements per option (hover)                            │                     │   as position on the axis
    │                           │ hover a column → its futures overlay THE RACE; hover a cell → the count and pocket rule  │[H]
810 ├───────────────────────────┴──────────────────────────────────────────────────────────────────┴─────────────────────┤
    │ think about:  Is this about order or about count?  ·  Which effects might you be missing?  ·  What does this card buy now, and later? │[A]
870 ├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
    │ Brandenburg-Prussia L2 · Shadows Flicker Like Flame · boards C+E                          replay · Test 1 · H hold │[A]
900 └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

* illustrative: the hand, Energy and the forced-Reclaim turn come from the snapshot and the track projection; the dot
  counts in the goal rows are the exact combinations of the counts in the COUNTED column (miss × miss).
[H] hover "27 of 96"  → the need finder's list by effect type (Defend 17 · gather/push 6 · remove Blight 2 · …) and the pocket
                         rule "about a quarter of the deck; four looks → about 3 in 4".
[H] hover C4 chip     → damage 3, health 2, Dahan 3 (counterattack 6), Defend 2 saves land and Dahan; Prussia L2 rules that apply.
[H] click the pace row→ the economy slopegraph per option against the deck's step, full width (the surviving Curve).
```

What this screen says without prose: G1 and G2 give the same chance of saving C4 (the draw), so between them the decision is the bottom two rows (hand size now, and when the cards run out), which is Brett's "order, not count" point drawn as bar length on the turn axis; G3 trades the draw for Energy and a longer reach, and its C4 row is the Fear-card row alone. The Fear-card and Event rows span all three columns so the eye drops them from the comparison.

## 5. What to cut, keep, and rebuild

**Survives as-is**
- The turn ribbon (phase tabs with the current phase lit): it is the one piece that already answers "where am I".
- The status bar.
- The painter and theme: `rect/fill/line/circle/polygon/icon/text/spans/clip/scroll/layers` cover every mark in §3 and §4; no toolkit work is needed. `glow` and `pulse` stay in the crate but leave the views.
- The engine under the views: `ledger.rs`, `outlook.rs`, `pace.rs`, `curve.rs`, `whatif.rs` (the recompute-on-a-copy core), `needs.rs`, `prompts()`, `rules_in_play`, `loss_clocks`. All of §4 is drawn from their outputs.

**Survives as a component, moved**
- The Ledger lane (tokens, bracketed spends, the exact solve per land): becomes the hover flyout of a race row and of a land chip (on-demand). Its pocket-rule line moves into the same flyout.
- The Runway's per-order enumeration and its "if the next card is X" list: becomes the Blight row of THE RACE (dots on the shared axis) and that row's hover. The line-per-outcome chart and the P(loss) shading are not kept.
- The Pace's required-slope vs pace-so-far: becomes the Fear row of THE RACE (arrival turn as dots) with the slope chart as the row's hover. The "or clear the board" blockers become the board-clear rows.
- The Curve's economy lines and presence-track projection: become the per-option "Energy a turn" slopegraph row in Growth and Draft, on the shared axis.
- The Curve's "lands named a turn" step: becomes a thin grey band along the top of THE RACE (the pressure the whole race runs against).
- The "Defend N, or remove N Damage" pill: becomes the FIXED column's per-land need (one line, no pill).
- Land chips in terrain color, and the game's piece icons: keep for land labels and slot stacks only.

**Discarded**
- The Ledger as a page: a budget inventory is not a question (questions-from-play.md).
- The four separate turn axes: replaced by one.
- The "Who's growing faster" multiples panel and the 1.5–3.5 Energy-per-play band: a ratio with no decision attached; "about even" is a verdict.
- "Each turn's change" diverging bars: acceleration of an aggregate the player never acts on (and it renders wrong in c-hme).
- The what-if drawer (land × piece table with destroy/+Defend/+Dahan): a piece editor is the island rebuilt in the companion, and its lever is not one the player has. The recompute engine behind it survives under the option columns.
- Strikethrough / "(was N)" / "+" prefixed deltas and the status-bar "What if: Destroy …" sentence: replaced by the ghost/solid overlay.
- Every caption sentence under a chart ("Unanswered, the card…", "Victory needs…", "At that pace…", "Growth: … about even", "−5 this turn · prevent 5 to hold"): the chart they restate is redrawn so it needs none.
- Red pulsing tokens, glows on lines and markers, the saturation-coded loss zone: decoration on the weakest channel.
- Six meaning-bearing hues: down to two sides, one accent, grey.

## 6. A drift test

Apply to any proposed screen, panel or feature before building it. A "no" on any item means it is not built.

1. Is it headed by a question Brett actually asked at this phase (questions-from-play.md), with that question as its title?
2. Is the lever something the player can do in the game this phase (a growth option, a card, a target, an Event choice), not an edit to the board?
3. Does every time-based element sit on the one shared turn axis, aligned with THE RACE?
4. Is every hidden quantity drawn as "N of M" dots or slots, with no bare percentage and no adjective?
5. Can the screen be read correctly with every sentence deleted?
6. Does it show at least one thing the game's own screen does not?
7. Is there nothing ranked, highlighted as best, selected by default, or stated as a verdict?
8. Does it fit 1600×900 with no page scroll and at most seven labeled groups?
9. Does it reuse an existing encoding (side-color, turn-position, dots, slot stacks, slopegraph) rather than add a new one?
10. Does it avoid reproducing the island, the hand as a panel, a log stream, or any panel the game already has?
