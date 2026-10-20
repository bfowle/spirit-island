# Card coverage: every Power, Fear card and Event read by the simulator

Status: plan, 2026-10-20 (evening). Brett: "let's get the rest of these properly done."

## 0. Where it stands

| What | Count | Read in full | Where the gaps are listed |
|---|---|---|---|
| Power Cards | 321 | 321 read (every Major, Minor and Unique); 203 read and simulated in full, 118 typed with effects the simulator does not execute yet | `si cards --card <name>` marks each sentence |
| Innate tiers | 192 | 192 read | `si cards --innates` |
| Fear cards | 50 (3 Terror Levels each) | 0 | this plan, §2 |
| Events | 62 (sections by kind) | 0 | this plan, §3 |

A card has three states, printed by `si cards --card`: **read and simulated** (every
sentence typed, every effect executed by `si-rules::effects`), **typed, not simulated**
(the sentence has a type and a meaning but the simulator does not execute it yet; hindsight
judges the turn with a caveat), and **not read** (hindsight does not judge the turn).

First pass done the same evening (223 → 250 read): area scopes, the four trigger kinds,
Hazards' token choice, repeats and extra plays, the spirit-economy effects, Invader-phase
rules and Dahan Health, and the explicit `Unmodeled` kind; the five cards from Brett's
games read in full (Portents of Disaster and Scream Disease's trigger are simulated).

Second pass, same night (250 → 321 read, 192 of 192 tiers): element-set counts ("for
each 2 Moon 2 Earth you have"), Destroy per token, own-presence destruction, Health-worth
removal, Strife infighting and Dahan duels, Blight kept on the card, forced Build, more
area scopes (within Range N, every land, your lands), and the last-resort typed kinds
`SpiritEffect`, `SpecialToken`, `Timing`, `Movement`, `DeckEffect`, `Distribution`,
`Clarification`, `Variable`, `Choose` — each names what it is and whether the simulator
executes it (`◦` in `si cards --card`). The parser no longer splits inside parentheses and
reads a trailing parenthetical as a clarification of its own.

What "typed, not simulated" still covers (118 cards carry at least one): Spirit-economy
gifts (Energy, elements, cards, Reclaim, Repeat, extra plays), expansion spirits' own
tokens and markers, timing effects for later phases, movement beyond Push and Gather,
deck effects, Range bonuses, and the Invader-phase rules not yet wired into
`invaders::ravage`. Hindsight judges a turn that plays one with a caveat naming the
effect left out. Simulating them is the next layer, in the order the corpus meets them.

## 1. Power Cards: the classes (done)

The 145 unread sentences fall into classes; each class is one typed `Effect` and one
simulator behavior:

1. **Area scopes** — "In each adjacent land", "In that land and each adjacent land",
   "both target and one adjacent land", "up to N adjacent lands": `Effect::Area { scope,
   then }`, executed over the lands (this pass).
2. **Triggers** — "The next time an Invader is destroyed in target land this turn, N
   Fear", "Once this turn, after target Spirit uses a Power targeting a land, add 1
   Disease to that land", "When Powers generate Fear in target land, Defend N per Fear",
   "When Blight is added to target land, it doesn't cascade": `Effect::Trigger { when,
   then, once }`, executed by the play loop for the later plays of the line (this pass:
   the first two and no-cascade; the engine logs them as `ReflectionStatusEffect`).
3. **Token choices** — Hazards Spread Across the Island: `TokenChoice { options }` then
   `AddChosenToken(n)`, the recorded `AddTokenOfType` choice when the log has it.
4. **Repeats and extra plays** — Powerstorm, Gift of Twinned Days, Spur On, Gift of
   Flowing Power: `Repeat` / `PlayAnother`; typed; simulated later by enumerating the
   repeated card among the line's plays.
5. **Spirit economy** — gain Energy per X, Reclaim N, regain a forgotten Unique, make N
   Slow Powers Fast, +Range: typed (`EnergyPer`, `Reclaim`, `RegainForgotten`,
   `MakeFast`, `RangeBonus`); the ledger values Energy and cards, Range feeds targeting.
6. **Invader-phase rules** — "During Ravage, Dahan deal damage simultaneously", "Invaders
   do not heal", "Dahan have +N Health", "Explorers fight alongside Dahan":
   `InvaderRule(String)` typed, each simulated in `invaders::ravage` as it is needed
   (`DahanHealth(n)` this pass).
7. **Board and deck manipulation** — Dream of the Untouched Land, Cast Down into the
   Briny Deep, The Past Returns Again, Weave Together the Fabric of Place, Pour Time
   Sideways, Absolute Stasis: `Unmodeled(kind)` typed with an explicit kind; the
   simulator says so. These are not in Brett's corpus.
8. **Expansion tokens and markers** — Quake, Impending, MarkedBeast, BreathOfDarkness,
   WanderingVoice, TheEndlessDark: typed per spirit as they enter the corpus.

Order: the five cards in Brett's games first (Portents of Disaster, Scream Disease Into
the Wind, Hazards Spread Across the Island, Powerstorm, Solidify Echoes of Majesty Past),
then classes 1–6 across the deck, measured by `si cards` and by `si verify` on the logged
uses, then 7–8.

## 2. Fear cards

50 cards, three texts each, 153 distinct sentences, repetitive: "Each player chooses a
different land with Dahan. In each: Gather up to N Dahan", "Defend N in all Coastal
lands", "Invaders do -N Damage to Dahan (per land) during Ravage", "Each player adds N
Strife in a land with …", "Skip the next normal Build". The model: `FearCard { levels:
[FearText; 3] }`, `FearText { scope: Scope, effects: Vec<Effect> }` where `Scope` is
"each player chooses a land with …" (the recorded `Choice` supplies the land), "all
Coastal lands", "lands with Presence", "each board". The effects reuse the Power
grammar; the Invader-phase modifiers ("-N Damage during Ravage", "do not heal", "Explore
does not affect Coastal lands") become `InvaderRule`s applied for the turn. Verified by
`si verify` against the logged Fear resolutions (the engine logs each as a status effect
or piece action). The hindsight informed score can then treat the Fear deck as a
distribution (the cards left are known by name) instead of "unknown".

## 3. Events

62 cards, sections by kind (Dahan 58, Beast 38, HealthyIsland/BlightedIsland 25 each,
Disease 21, Choice 15, Terror Level I–III 12 each, Stage I–III 10 each), 284 distinct
sentences. The model: `EventCard { sections: Vec<EventSection> }` as the definitions
already carry, each section typed as `Choice { options, costs }` (the recorded
`SelectEffect` and `PayEventCost` choices supply the branch), `PerBoard { effects }`
("On Each Board: …", the recorded `SelectBoard`/`SelectLocation` supplies the land),
`Conditional { on: Healthy|Blighted|TerrorLevel|Stage }`, with effects from the Power
grammar plus the Event-only ones ("Spirits may prevent this by destroying N Presence",
"Each Beast destroys N Explorer"). Verified the same way. Events are the largest piece
and come last; until then they stay exogenous (applied from the log).

## 4. Measurement

`si cards` prints the three-state counts per kind after every pass; `si verify` over the
21 games is the correctness gate (today: 92–95% of outcomes per game, 100% on the Shroud
fixture); a card is "done" when every logged use of it verifies.
