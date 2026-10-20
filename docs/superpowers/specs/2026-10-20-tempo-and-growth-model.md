# Tempo and growth, as numbers first — model, tests, CLI, corpus

Date: 2026-10-20
Status: proposal after the step back. Nothing here is built. Decisions marked **(Brett)** need an answer before coding.
Replaces, for now: the VS screen direction (2026-10-19-direction.md stays the long-term backbone; no screen work until the tables below say something true about real games).

## 0. Why

The VS page said "4.4x behind" about a game Brett had just won. The model
measured prevention only, so a spirit that removes Invaders after they land
read as losing. Brett's verdict: "this doesn't feel like the right direction at
all". The step back: define the raw quantities the way the best guides define
them, compute them from the game's own logs, check them with tests and CLI
reports over the 33 logged games, and only then decide what a screen shows.

## 1. What the sources say (read 2026-10-20)

Rei's three BGG guides are saved in `docs/guides/` (HTML with images) and as
text in `research_notes/rei/` (`vengeance-burning-plague.md`,
`many-minds-move-as-one.md`, `shroud-of-silent-mist.md`). All three open with
the same definition, last revised by Rei in June 2024:

> Tempo, in general, can be represented by a few different things in SI:
> - The timing at which you are playing actions relative to the Turn timing of an Invader action (Ravaging / Building / Exploring)
>   - "On Tempo w/ the Invaders": utilizing the Spirit's actions after the land Builds (via Slow Power) **or** on the Turn the Invaders are Ravaging (via Fast Power) to "solve the land" so it doesn't add blight
>   - "Above Invader's Tempo": utilizing the Spirit's actions to affect upcoming actions (Builds / Explores) such that you're preventing them from even having a chance to Ravage before having to use a Slow Power (post Build) or Fast Power (turn of Ravage). Both of which end up as On Tempo Actions
>   - While we want to prevent as much Blight as possible, sometimes we need to sacrifice lands to Blight in order to play On Tempo
> - **Also** the timing at which your track progression / track development / track scaling reaches a certain state / threshold by a certain Turn Timer (i.e. 2e/3card plays by Turn 4)

Rei's second sense, stated outright in the Many Minds guide ("The Hasteful"):

> Your tempo is the rate at which you grow the Spirit by taking presence off your track, how many cards you gain, and how much energy you gain.

Every build Rei publishes carries a three-part label, which is the taxonomy we
will measure games against:

| Axis | Values Rei uses |
|---|---|
| Track scaling | Fast Track Scaling · Minus Tempo Track Scaling · Problematic Scaling |
| Stance toward the Invaders | On Tempo w/ Invaders · Above Tempo against the Invaders |
| Blight | Uses Blight for Tempo leverage · Uses Blight for Scaling leverage · Uses less Blight for Scaling · Uses Blight to fix Scaling |

Power spikes, in Rei's x/ycp notation (energy income / card plays):

| Spirit | Spike | Rei's words |
|---|---|---|
| Vengeance | 3/3cp | "3/3cp is your power spike… energy sustainability every turn in addition to unlocked Tier 2 innate thresholds" |
| Many Minds (minors) | 2/4cp | "energy sustainability for 1 cost minors, card gains when you feel like it, and the extra air" |
| Many Minds (majors) | 3/3cp | "as always, 3/3cp is your power spike" |
| Shroud | 1/3cp; 1/4cp + reclaim | "1/3cp is your power spike, BUT 1/4cp+reclaim is when you Transcend" |

Other Rei concepts that become measurements: **scaling** ("placing more
presence and unlocking… is MUCH more valuable than" playing filler, playing a
low-value minor, or reclaiming because you can't act); **weak turn** (one big
weak turn beats lingering ones); **reclaim loop** (stuck reclaiming every other
turn because critical cards sit in the discard); **stockpiling** (banking
Energy across turns to play Majors several turns running); **doubling up**
(the same terrain on the Stage I card and the Stage II card behind it, which
makes skipping that land's Ravage worth a growth turn); **action economy**
(not doing a thing twice; being a turn ahead so Events can finish lands for
you); **gambling** on Events and Fear cards with stated odds (a ravaging land
with 1 Explorer, 1 Town, 2 Dahan and your presence is saved 40.7% of the time
by the Event alone; 34% at Terror 1 and 56% at Terror 2 by the Fear card).

Lumia (`iam.lumia.gay`, saved notes in `research_notes/…/lumia_si_theory_blog.md`)
adds the vocabulary of **growth order** for repeatable effects: constant,
linear, quadratic (Slip the Flow of Time's n(n−1)/2 cost; range coverage
~n²), n^1.5 (Blight cascades), exponential (Lair gathering), factorial (the
Bringer + Melt Earth meme). Her glossary's "tempo" is the land clock: ahead =
between Explore and Build, on = between Build and Ravage. Her first principle:
most things in the game are linear; compounding is the exception worth
studying. Brett: both sides' growth mechanics must be typed from the rules
(Section 3).

## 2. Definitions, each one computable from a log

### 2.0 The land ledger: one vocabulary for Rei, Lumia and the chain value

Rei names positions on a land's clock; Lumia keeps a running ledger per turn
("(+2=9 fear generated)", "(should have 3 energy remaining)") and resolves
actions as an ordered tree; our chain model prices an Invader action by the
chain it starts. One structure carries all three:

```
Ledger(L) = [ (turn, phase, actor, effect, Δpieces, Δblight, Δdahan, Δfear) … ]   for land L
clock(e)  = BeforeExplore | AfterExplore | AfterBuild | AfterRavage              for an entry e, relative to the chain it meets
```

The **remaining chain** after clock position k is what the Invaders would still
do in L if nothing else happened:

```
R(BeforeExplore) = { Explore, Build, Ravage, seeds }      seeds = the Explores this land's Town/City feeds into unfed neighbors
R(AfterExplore)  = { Build, Ravage, seeds }
R(AfterBuild)    = { Ravage, seeds }                      (a Town stays and seeds; a City seeds and makes the next Ravage worse)
R(AfterRavage)   = { seeds }                              plus the Blight already on the land
```

The **value of an intervention** at k is the consequence it takes away:
`V(k) = Σ_{a ∈ R(k) removed} (blight(a) + ω_D·dahan(a) + ω_F·fear(a)) + tail(seeds removed)`.
Preventing an Explore takes away the Build, the Ravage and the Explores that
Town would have seeded ("the next two turns that much better"). Preventing a
Build takes away a Ravage that was always coming. Acting at the Ravage slot
**prevents** this turn's Blight (Defend, Dahan counterattack, or killing the
pieces first); if the buildings survive, the land is back in the cycle next
turn and the same price is due again. **Removing** Blight is a different thing
altogether: a separate card effect or innate, spent after Blight is already on
the land; in the ledger it is a `Recover` entry at `AfterRavage`, never a
"solve".

The **price of an intervention** is the Spirit side of the same ledger and is
what makes the clock positions unequal (Brett, 2026-10-20):

```
price(e) = cards(e) + energy(e)         cards played go to the discard until a Reclaim
hand(t+1) = hand(t) − cards spent(t) + cards gained(t)
Reclaim forced when hand(t) < plays(t)  → a growth that reveals no track slot → scaling slows
```

Defending one Ravage with Dahan support can cost two cards (move Dahan in,
then Defend for the counterattack) for one land, on a turn when N other lands
are moving Explore → Build → Ravage untouched; both cards sit in the discard
while those lands come due. Spending cards at the Ravage slot therefore
compounds: the land cycles back, the hand shrinks, the Reclaim comes early,
the track stops growing, and the Invader step function keeps stepping. Spending
one card at the Explore or Build slot pays once. The report carries, per turn:

| Column | Meaning |
|---|---|
| `due` | lands with a chain reaching Build or Ravage this turn |
| `acted` | lands the Spirit touched, by clock position |
| `cards / land` | price paid per land handled |
| `V / card` | value taken away per card spent (efficiency); Above actions should dominate |
| `hand → discard` | cards left in hand after the turn, cards in discard, turns until a forced Reclaim at this spend rate |
| `reach` | which clock positions this hand can act at in each due land: Defend or Dahan effects for the Ravage slot; damage, push or Disease/Beast/Strife for Build and Explore; Blight removal is a Recover effect only |

`reach` is the capability Brett points at: a hand with no Defend and no Dahan
movement cannot play On tempo at a Ravage however many plays it has; gaining a
card to solve one Ravage while another land Builds is how a spirit falls
behind. This is the chain/pace model's "capacity" restated per land and per
card, and it is also the test for D2 (`Untouched` requires `reach`).

Rei's names are the clock positions (Above = Before/AfterExplore, On =
AfterBuild); Lumia's ledger is the bookkeeping that makes every entry checkable
against the log; the chain value is the price. `chain.rs` already computes the
tail by re-walking the outlook with one action stopped; it moves into
`si-metrics` and reads the ledger instead of the snapshot.

Dahan weight ω_D is **per spirit**, from the spirit's own rules (Brett, D1):
Thunderspeaker's presence rides with the Dahan, so a Dahan loss there costs
presence; Vengeance clears lands with no Dahan present; Ocean drowns Dahan with
the Invaders. The weight is a sourced constant on the spirit's definition
(`si-defs`), never a guess; the raw Dahan loss is always printed beside it.

### 2.1 Land tempo (per land, per Invader card)

A **chain** is one Invader card's life in one land:

```
Explore (turn t) → Build (turn t+1) → Ravage (turn t+2)
```

The log attributes every step exactly: the Invader phase runs
`ChangeStateAction To=Explore|Build|Ravage`, `ResolveInvaderCardAction …
InvaderActionType: Explore|Build|Ravage, InvaderCard: Mountain1` names the card,
`AddIslandPiecesAction` names the land for each piece added, `RavageAction:
LandTarget:` names each ravaged land, `AddBlightAction` carries the amount and
the cascade flag. Adversary and Event placements show as `MovePiecesAction`
from `Out of Game`, so a chain may start at a Build (Habsburg Mining Boom) or
at a rule placement; the chain starts at the first Invader step observed in
the land.

The **chain clock** positions any Spirit action on a land:

| Clock position | When | Rei's name |
|---|---|---|
| `BeforeExplore` | before turn t's Explore (presence, Beasts, Disease, Strife placed; Fast on turn t) | Above |
| `AfterExplore` | Slow on turn t, or Fast on turn t+1 (before the Build) | Above |
| `AfterBuild` | Slow on turn t+1, or Fast on turn t+2 (the Ravage turn) | On |
| `AfterRavage` | Slow on turn t+2 or later | Late (cleanup) |

A **touch** is one Spirit effect on the land with its clock position: a Power
or innate targeting the land (`UsePowerAction … PowerTarget: SandsC2`, with
`PhaseType=FastPower|SlowPower` on the activation), a `DefendAction` naming the
land, damage or destruction of pieces in the land, pushes or gathers in or out,
Dahan moved in, tokens added. Growth placements count as touches at
`BeforeExplore`/`AfterExplore` (a Sacred Site placed in a just-explored land is
Rei's "gamble").

The **outcome** at the chain's Ravage slot:

| Outcome | Observed as |
|---|---|
| `Prevented` | no `RavageAction` for the land at t+2 (no Invaders left, or the Ravage was skipped by Strife, Disease, a Skip effect, an Event) |
| `Cleared` | no Town or City remains at the Ravage slot (killed before or during it); the land leaves the Invader cycle |
| `Defended` | the land Ravaged and Blight was prevented (Defend, Dahan counterattack, damage first) but buildings remain; the land is back next turn and the price is due again |
| `Blighted` | `AddBlightAction` in the land from that Ravage (direct) |
| `Cascaded` | Blight arrived from a neighbor's cascade (not the chain's own Ravage) |
| `Recovered` | Blight removed afterwards by a Blight-removal effect or innate (`Recover` entry); the Blight still happened and still counted on the card |

The **tempo class** of the chain combines the earliest effective touch and the
outcome:

| Class | Rule |
|---|---|
| `Above` | outcome `Prevented`, and a touch at `BeforeExplore`/`AfterExplore` removed or blocked the Build or Explore |
| `On` | outcome `Prevented`, `Cleared` or `Defended` with the deciding touch at `AfterBuild` |
| `Gambled` | outcome `Prevented`, `Cleared` or `Defended` with no deciding Spirit touch (Event, Fear card, Dahan counterattack did it) |
| `Failed` | a touch at `AfterExplore`/`AfterBuild` and the land still `Blighted` (the land needed more than it got; Shroud T5 below) |
| `Sacrificed` | outcome `Blighted`, no deciding touch, and the Spirit could not have touched it: card plays exhausted, or no card in hand with `reach` for that slot and a payable cost |
| `Untouched` | outcome `Blighted`, no touch, and a playable, in-range, relevant card and a free play were available (left on the table) |
| `Late` | touches only at `AfterRavage` |

D1 (Brett, 2026-10-20): "solved" = no Blight. Dahan losses are reported
beside it and weighted per spirit (Section 2.0).
D2 (open): `Sacrificed` vs `Untouched` by "could the Spirit have touched it"
(plays left, a card in hand that reaches the land, Energy to pay). Worked
example from Brett's Shroud win (EndGame--8584091393976121238, 8 turns):

| Turn | Growth | Plays | Blight landed in | Shroud touched | Reading |
|---|---|---|---|---|---|
| T3 | G3 (card + presence) | 2 of 2, 4 Energy banked | Jungle D2, Jungle D6 | D2 only after the Ravage (Slow Wrackroot); D6 never | both `Sacrificed`: plays were exhausted; Energy is not the binding limit. A growth turn with Blight = "took Blight to scale" |
| T4 | Reclaim | 2 of 2 | Wetland D1 ×2 | D1 only after the Ravage (Slow Call of the Dahan Ways) | `Sacrificed` + `Late` touch |
| T5 | G3 | 2 of 2 | Jungle D2 ×2, Sands D4 | D2 twice, Fast, on the Ravage turn | D2 `Failed` (On-tempo touch, still Blighted); D4 `Sacrificed` |

The alternative is report-only: print Blight, plays used, playable cards left,
growth, and let the reader judge each line.

Per turn and per game the report gives counts and shares per class, the Blight
per chain, and the mix of Fast vs Slow deciding touches.

### 2.2 Track tempo (per spirit, per turn)

The **track state** after Growth on turn n: energy income E, card plays CP,
elements from revealed track slots, presence left on each track, presence on
the island, Energy banked, **cards in hand, cards in the discard, cards
forgotten**, cards gained so far, reclaim this turn, cards played this turn,
and the growth choice (group and options, from
`SelectGrowthGroupActivation` and `SpiritPhaseActivation … GrowthOptionController=`).
Slot values come from the engine's `presenceTrackNodes` (we already decode
them: `spirits.rs::track_value`). Notation in reports: `x/ycp`.

Derived:

| Quantity | Definition |
|---|---|
| **Spike turn** | first turn with E ≥ x and CP ≥ y for the spirit's spike (table above; other spirits from their chapters; else none) |
| **Benchmark** | Rei's "by Turn 4" (or the chapter's); report spike turn − 4 |
| **Scaling rate** | presence revealed from tracks by turn n (max is 2 per growth turn with G2) and cards gained by turn n |
| **Weak turn** | cards played < CP, or 0 played; flagged, not judged (Rei: fine when it buys a growth) |
| **Hand runway** | turns the hand lasts at this turn's spend rate before a Reclaim is forced (hand < plays); the discard is the cards a Reclaim would bring back. Brett: cards in hand next turn vs. cards in the discard is part of tempo and ramp, not bookkeeping |
| **Reclaim price** | what the Reclaim growth gave up: the track slot (income, play or element) the alternative growth would have revealed that turn; partial reclaims ("Reclaim 1" slots) priced the same way |
| **Reclaim loop** | reclaims on turns n and n+2 |
| **Stockpile** | banked Energy rising for ≥ 2 consecutive turns |
| **Elements online** | per innate tier, first turn the track plus the turn's played cards reach it |

### 2.3 Build profile (Rei's three axes, measured over turns 1–4)

| Axis | Measurement |
|---|---|
| Track scaling | `Fast`: presence revealed by T3 equals the maximum the growth options allow. `Minus`: one presence traded for a card or Energy (a G3-type growth) on T1 or T2. `Problematic`: a reclaim before T4 with no growth that turn (Rei's "Hasteful": G2, G2, Reclaim). |
| Stance | share of handled chains (classes `Above` + `On`) that are `Above`; ≥ 50% reads "Above", else "On" |
| Blight | Blight added by T4 in lands holding the spirit's presence, over all Blight by T4; and whether a growth happened on a Blight turn ("took Blight to scale") |

Rei's five Many Minds openings and the Vengeance and Shroud openings are
written out as growth sequences in the guides and carry his labels. They are
the fixtures: the classifier must label them as Rei does (Section 7).

### 2.4 Odds Rei quotes

The guides give event and Fear odds for one position: a ravaging land with
1 Explorer, 1 Town, 2 Dahan, and 1–2 presence. We recompute them from the
engine's `event_cards.json` and `fear_cards.json` and print both numbers. If
ours differ, the report says by how much and why (expansion pool, Terror level,
ruling). The same function later answers "what are the odds this land is saved
without me" for any land.

### 2.5 Invader deck patterns

**Doubling up**: Stage I terrain X followed by Stage II terrain X. Once the
Stage I card is in the Build slot, P(the next card matches) follows from the
Stage II cards left, which are public. Reported per turn; Rei uses it to decide
whether a growth turn that skips one land's Ravage is worth it.

## 2.6 What affects play: coverage check (Brett, 2026-10-20)

Brett listed the forces that decide a turn. Each one, checked against the
engine definitions, the current crates, and the types planned here.

| Force | Verified against the game | In the crates today | Planned type / rule | Gap closed by |
|---|---|---|---|---|
| **Energy economy**: pay for cards, keep a reserve for Event "pay to get the better half" | Event cards carry pay-choice sections (`PublicEventCard.sections`) | `energy`, `energy_income`, knapsack spends to zero; Events read but their pay options not priced | `TrackRow.banked`, `TrackRow.reserve`; `EventTax`: from the Event deck still unseen, the distribution of pay-to-choose costs and what the paid branch buys; `capacity_now` keeps `reserve` back when the Event is still to come | si-metrics `odds::event_tax`, si-policy knapsack reserve |
| **Card plays**: N direct effects, and every played card's elements bank toward innates and card thresholds | engine: `AdjustCardPlaysAction`, `GainElementsAction`; thresholds count the turn's played cards, Fast and Slow, plus track elements and markers | plays from track; innate levels from elements of chosen plays (`needs.rs`) | `TrackRow.plays`, `played`; `ElementBank { track, markers, played_fast, played_slow }`; thresholds evaluated on the whole turn's bank for innates and for card thresholds (`PowerCard.threshold`) | si-rules `elements::bank`, `thresholds::reached` |
| **Elements and innates**: "free" effects on top of the cards paid for | `innatePowers[].thresholds[]` with element sets and bodies (Thunderspeaker: Gather the Warriors Slow R1, Lead the Furious Assault Slow R0) | innate card synthesized at the reached tier (`needs.rs::innate_card`) | `Innate { tiers }`; a touch by an innate is an `Actor::Innate` with price 0 cards, so `V / card` credits it to the cards that banked the elements | si-record attribution of `UsePowerAction` for innates |
| **Track unlocks**: slots that are not income or plays: permanent elements, Reclaim One, pay-to-gain, growth options, Energy | `presenceTrackNodes[].trackNodeType` ∈ EnergyPerTurn, CardPlaysPerTurn, Element, ReclaimCards(1), PayEnergyToGainPowerCard, GrowthOption; Thunderspeaker has Reclaim 1 after plays 3; Shroud has a Move-Presence growth option and Reclaim 1 on its plays track | `TrackNode.kind` carried as a string; `track_value` reads income and plays only; Reclaim One and pay-to-gain not priced | `TrackNode` enum with every kind; `Spike` and scaling rate count them; **Reclaim price** uses Reclaim-One slots; `Hand runway` counts pay-to-gain as a card source | si-model `TrackNode`, si-metrics 2.2 |
| **Range and targeting**: where presence may go, range from presence or Sacred Site, +range rules, land-type and "with Dahan" targets, origin requirements | Volcano: *Mountain Home* "Presence may only be added or moved into Mountains"; *Volcanic Peaks* "+1 Range if you have 3 or more Presence in the origin land"; Pyroclastic Bombardment range 2 from a Sacred Site; Manifestation of Power and Glory range 0, `LandWithDahan` | `needs.rs::reaches`: range from presence or Sacred Site, target type, ocean rule; spirit special rules **not** modeled (comment says so) | `Placement` rule per spirit (allowed terrains, adjacency, "lands with Dahan" for Thunderspeaker's G2); `Range { from: Presence|SacredSite, distance, modifiers }`; `TargetKind`; `reach(hand, land)` is a rules function and feeds `reach` in the ledger and the `Untouched` test | si-rules `targeting`, si-defs placement rules per spirit |
| **The growth option itself**: a growth can pay Energy, place several presence, add a play, gather Beasts | Thunderspeaker G3 = presence + 4 Energy; Volcano G3 = card + presence (range 4) + 1 card play this turn + 2 Energy; Many Minds G3 gathers Beasts | engine `growthGroups` decoded; `growth.rs` parses Wiki shorthand; growth choices logged (`SelectGrowthGroupActivation`) but their yields not priced | `GrowthChoice { group, options, yields: Yield { energy, plays_this_turn, presence: Vec<Placement>, cards, reclaim, moves, tokens } }`; `TrackRow.growth`; scaling rate counts presence placed from any option; the tempo report prices a Reclaim growth against the alternative option's yield | si-model `GrowthChoice`, si-metrics 2.2 |
| **Hand vs discard**: combos, their cycle, racing a Reclaim One | Thunderspeaker: Voice of Thunder (0, Slow, R1, push up to 4 Dahan or 2 Fear) sets up Manifestation of Power and Glory (3, Slow, R0, land with Dahan: 1 Fear, each Dahan deals damage equal to your Presence there); Gather the Warriors (innate, Slow) also brings Dahan; Reclaim 1 on the plays track brings one of them back without a full Reclaim | hand, play area, discard, forgotten in the snapshot; `curve.rs` forces Reclaim when hand < plays; no combo or cycle model | `Combo { cards, phase, effect }` detected from records (cards played together ≥ 2 times, or a card plus the innate it feeds); `Cycle { combo, turns_between_uses, recovered_by: Reclaim|ReclaimOne|Gain }`; `Hand runway` and `Reclaim price` (2.2); a combo's cadence is a growth order of its own (`Growth::Sawtooth { period }`) | si-metrics `combos`, si-record play grouping |

Everything in the table is observable in the logs we have (growth choice, cards
played, Energy paid and gained, Event choices, power targets, innate uses), so
each row gets a test on a real game, not only a synthetic one.

## 3. Growth as a type (Lumia), both sides

```rust
pub enum GrowthOrder { Constant, Linear, Quadratic, Power(f64), Exponential(f64), Factorial, Step { period: u32 }, Sawtooth { period: u32 } }

pub trait Growth {
    fn name(&self) -> &str;
    fn side(&self) -> Side;                 // Spirit | Invader
    fn order(&self) -> GrowthOrder;         // asymptotic class, from the rules
    fn value(&self, n: u32) -> f64;         // closed form after n uses / turns
    fn source(&self) -> &str;               // rule or card text the form comes from
}
```

Spirit-side implementations (each one cites the rule it encodes):
`TrackIncome` and `TrackPlays` (step functions of presence revealed),
`TrackElements` (step), `PresenceOnIsland` (linear, ≤ 2 a turn),
`HandSize` (sawtooth: +gains, reset by Reclaim), `BankedEnergy` (linear in
income minus spend), `RangeCoverage` (lands within range r of a Sacred Site,
~quadratic in r on the hex-like boards), `RepeatCost` (Slip the Flow of Time,
Relentless Punishment: n(n−1)/2), `LairGather` (exponential), `MeltEarth`
(Lumia's recurrence meme(n) = 1 + ceil(n/2)·meme(n−1)).

Invader-side implementations: `LandChain` (Explorer → Town → Ravage; constant
per land), `Spread` (Explores seed from adjacent Towns and Cities: geometric
until the board saturates, i.e. logistic), `StageEscalation` (step in time:
Stage I/II/III card counts and adversary escalations), `BlightCascade`
(Lumia's ~n^1.5), `TerrorLevel` (step in Fear), `DeckDepth` (linear countdown
to the Invader-deck loss).

Tests pin the closed forms (0, 1, 3, 6, 10, 15, 21 for the quadratic repeat
cost; the meme recurrence; step values straight from a spirit's track array).
The corpus report fits each game's observed series (income, plays, presence,
banked Energy; Invader strength, Blight) to these orders and reports the
best-fit class per series, so games can be binned by growth shape as well as
by Rei's axes.

## 4. The designers' pricing (Brett's sidenote)

The engine dump has 321 Power Cards with cost, speed, range, target type,
elements and effect text (`si-steam-dump defs`). `CardEffects` parses each
body into counts: damage, Defend, push, gather, Fear, Blight removed, Dahan
and Beast effects, tokens placed, conditional bonus, repeat. A linear fit of
cost on those features plus speed, range and element count gives the implied
exchange rates (one damage, one Defend point, one push, one Fear, the Slow
discount, each in Energy). Residuals name the cards priced under their effect.
Uses: a rules-grounded Energy-equivalent for any play (replacing Rei's disputed
~2.5E per card play), and the "why" behind 0- and 1-cost minors. Tests: hand-
parsed features for a dozen cards; the fit's sign and rough size on the
obvious coefficients.

## 5. Rust shape

New module `si-common/src/tempo/` (pure data and math, no I/O):

```rust
pub struct GameRecord { pub meta: GameMeta, pub turns: Vec<TurnRecord>, pub chains: Vec<LandChain>, pub spirits: Vec<SpiritCurve>, pub board: Vec<BoardRow> }
pub struct GameMeta { pub id: String, pub spirits: Vec<SpiritId>, pub adversary: Option<(String, u8)>, pub scenario: Option<String>, pub boards: Vec<String>, pub result: Result_, pub turns: u32, pub source: Source /* EndGameLog | Capture */ }

pub struct LandChain { pub land: LandId, pub card: Option<InvaderCardId>, pub stage: Option<u8>, pub explore: Option<u32>, pub build: Option<u32>, pub ravage: Option<u32>, pub outcome: Outcome, pub blight: u32, pub dahan_lost: u32, pub touches: Vec<Touch>, pub class: TempoClass }
pub struct Touch { pub turn: u32, pub phase: Phase /* Growth | Fast | Slow | Event */, pub clock: ChainClock, pub by: Actor /* Power(id) | Innate(id) | Growth | Dahan | Event | FearCard */, pub effect: Effect }
pub enum ChainClock { BeforeExplore, AfterExplore, AfterBuild, AfterRavage }
pub enum Outcome { Prevented { why: Skip }, Solved, Blighted, Cascaded }
pub enum TempoClass { Above, On, Gambled, Sacrificed, Untouched, Late }

pub struct SpiritCurve { pub spirit: SpiritId, pub rows: Vec<TrackRow>, pub spike: Option<Spike>, pub spike_turn: Option<u32>, pub profile: BuildProfile }
pub struct TrackRow { pub turn: u32, pub income: u32, pub plays: u32, pub elements: ElementSet, pub presence_left: (u8, u8), pub on_island: u8, pub banked: i32, pub hand: u8, pub gained: u8, pub played: u8, pub reclaimed: bool, pub growth: GrowthChoice }
pub struct Spike { pub income: u32, pub plays: u32, pub label: String /* "3/3cp" */, pub by_turn: u32, pub source: &'static str }
pub struct BuildProfile { pub scaling: Scaling, pub stance: Stance, pub blight: BlightUse }

pub struct BoardRow { pub turn: u32, pub explorers: u32, pub towns: u32, pub cities: u32, pub blight: u32, pub dahan: u32, pub fear: u32, pub terror: u8, pub invaded_lands: u32 }

pub trait Growth { /* Section 3 */ }
pub struct CardEffects { /* Section 4 */ }
```

`si-steam/src/record.rs` builds a `GameRecord` from an EndGame log or a
capture, reusing `backfill.rs` (turn boundaries, undo cuts, bulk stamps,
rewrites) and `log_parse.rs` (decisions). Existing `RoundSummary` rows stay
for the live companion; `BoardRow` is the same data for finished games.

## 6. What the log gives (verified on EndGame--8584091383357911121.txt)

- Phase order per turn: `SpiritPhase → FastPower → BlightedIsland → Event → Fear → Ravage → Build → Explore → AdvanceInvaderCards → SlowPower → TimePasses`.
- `ResolveInvaderCardAction: Source: Stage I Mountain, InvaderActionType: Explore|Build|Ravage, InvaderCard: Mountain1` after the pieces it added.
- `AdvanceInvaderCardsAction: CardMovements=(Stage I Mountain, Build Space, Ravage Space)`.
- `UsePowerAction: Source: <title>, PowerToUse:<id>, PowerTarget: <Terrain><Board><n>`; the activation line carries `PhaseType=FastPower|SlowPower`. Innates use the same line.
- `PlayPowerCardAction` + `PayEnergyAction Amount:` per card played; `AdjustCardPlaysAction`.
- `DefendAction: … IslandLocationsToDefend=[Sands C2], Amount=5`; `DealDamageAction … DamageDistribution=…`; `DestroyPiecesAction … PiecesBeingDestroyed=…`; `GatherPiecesActivation: LandToGatherInto=`; `PushPiecesActivation: IslandLocationToPushFrom=`.
- `SelectGrowthGroupActivation: Selected=<Spirit> Growth Group 2`; `SpiritPhaseActivation: … GrowthOptionController=Add a Presence | Gain Power Card | +1 Card Play This Turn | Move a Presence | Gain 2 Energy`; `MovePresenceAction` track node → land.
- Undo: `IsUndoActivation=True` lines and the save-file reloads handled today by `backfill.rs`.

## 7. Tests

1. **Classifier fixtures from the guides.** Rei's openings written as growth
   and play sequences: Vengeance Fast Minors (G3, G2, G2), Vengeance Minus Minors,
   Many Minds Drafter (G2 TB, G2 TT, G2 BB+pay), Extra Friend (G3, G2, G2,
   Reclaim), Hasteful (G2, G2, Reclaim), Big Birb, Ponderer, Shroud Unleashed
   (G2, G3, Reclaim|G2), Gambler, Confounding. Expected: Rei's own labels on
   the scaling axis; spike turns from his images (Vengeance 3/3cp by T4).
2. **Chain clock and class** on synthetic chains, one per class, including a
   Build prevented by Disease (Above), a Fast Defend on the Ravage turn (On),
   an Event save with no touch (Gambled), Blight while all plays went elsewhere
   (Sacrificed).
3. **Golden game**: the Shroud win (`live-e392726411c9`), every turn's track
   row and every chain, checked by hand once and frozen.
4. **Odds**: Rei's 40.7% / 34% / 56% reproduced or the difference explained.
5. **Growth forms**: closed forms and track step values.
6. **Card features**: a dozen hand-parsed cards.

## 8. CLI (`si-steam-dump`, new subcommands)

```
si-steam-dump tempo <EndGame.txt | capture dir>      one game: chains table, track table, board table, profile
si-steam-dump corpus <dir>... [--csv out.csv]         one row per game + the groupings in Section 9
si-steam-dump odds [--terror 1|2|3] [--land 1E1T2D]   event/fear save odds for a position
si-steam-dump cards                                  pricing fit and residuals
```

Each prints plain tables; `--json` for the same data.

## 9. Reports and grouping

`reports/game-corpus-<date>.md`, one row per game: spirit(s), adversary and
level, result and turn, spike label and spike turn, presence by T3, cards
gained by T3, first reclaim turn, chains by class (counts and shares),
Blight at T4 and at the end, Fear cards by T4, best-fit growth class per
series. Then the groupings Brett asked for:

- by Rei's scaling axis (Fast / Minus / Problematic) → win rate, Blight at T4
- by stance (Above-leaning / On-leaning) → Blight per chain, Fear per turn
- by Blight use (leverage vs less) → spike turn
- by growth shape of income and plays (step early / step late / flat)
- the same, split by adversary level band (≤3, 4–5, 6), since Rei warns level 6 "vastly warps & changes builds"

Thirty-three games is small; the report states counts with every rate and
does not claim significance. The point is to see Brett's own habits in Rei's
vocabulary and to find which quantities actually separated his wins from his
losses, before any of it is drawn.

## 10. Decisions

- D1 — decided: "solved" means no Blight; Dahan loss reported separately and weighted per spirit from its rules.
- **(Brett)** D2 — `Sacrificed` vs `Untouched` by "could have touched it", or report-only (example in 2.1).
- D3 — decided for now: spikes from a Rei guide or a chapter we wrote, otherwise none. Next: derive a spike from the panel (`Spike::derive(panel, Minors|Majors)`: the first track state whose income sustains the intended card costs at that many plays and whose track elements reach the next innate tier) and check it against Rei's three; if it matches, we can write tempo guides for the other spirits from the model.
- **(Brett)** D4 — the per-game report is three tables and a profile line; is anything else needed to judge a game?
- D5 (mine) — a chain that never Builds because the Explorer was removed counts as `Above` even if the Spirit's touch was the Growth placement (Shroud's and Many Minds' "gamble" with a Sacred Site); the report shows the actor.

## 11. Order of work

1. `tempo/` types + Growth trait + fixtures from the guides (tests only).
2. `record.rs`: GameRecord from a log; golden test on the Shroud win.
3. `tempo` CLI on one game; read it with Brett.
4. `corpus` over the 33 games; the grouping report; read it with Brett.
5. Odds and card pricing.
6. Only then: which of these numbers a screen should carry.
