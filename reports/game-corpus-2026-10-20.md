# Game corpus — Brett's logged games measured in Rei's terms (2026-10-20, second pass)

Produced by `si corpus <Steam EndGame folder> <captures…> --defs <engine dump> --md`
(crate `si-cli`; model in `crates/si-model`, replay in `crates/si-record`, measures in
`crates/si-metrics`). Definitions: `docs/superpowers/specs/2026-10-20-tempo-and-growth-model.md`.

Second pass over the same logs, after four changes to the measuring:

1. **Scaling** now follows the labels Rei puts on his own builds. The ten builds he
   names across the Vengeance, Many Minds and Shroud guides are test fixtures
   (`crates/si-metrics/tests/rei_openings.rs`) and every one comes back with his label.
2. **Reach** is computed: for each Blighted land, whether a card in hand could have acted
   before the Build or before the Ravage (speed, Energy, range from presence on the
   board, the card's target rule, the printed effect's step). *Untouched* and
   *Sacrificed* now mean what the spec says.
3. The replay **places each board's printed pieces** (Dahan, the setup City and Town,
   starting Blight and tokens). The log never records them, so every land count before
   this pass was short by them.
4. The **adversary level** comes from the engine save when one exists; otherwise it
   is inferred from the Invader deck order, the Fear deck's size per Terror Level and
   the setup additions, as the set of levels consistent with all three. (The inference
   read Sweden 5 where the save says 4; the Fear-deck sizes per level need checking.)

## What was measured

21 distinct games (15 from recorder captures with an exact undo clock, 6 from bare
EndGame logs with the repaired heuristic), 18 finished. The same game recorded twice
(a capture and its EndGame log) is counted once; 14 EndGame logs were duplicates, one
capture was too short (1 turn). Where the engine's `EndGame--<id>.dat` save exists
(beside the log, or matched to a capture by game id) the adversary, its level and the
game difficulty are read from it; four games the log never named an adversary for
turned out to be adversary games.

How to read a row:

- **level / diff** — the adversary levels consistent with the evidence (a range when
  the evidence cannot split them; `?` when nothing fits, which happens on bare logs
  whose Fear-card count was disturbed by undo re-logging) and their difficulty.
- **scaling** — Rei's track-scaling axis over turns 1–3. *Fast*: every growth took the
  full presence the panel allows, except at most one that took a card instead
  (Vengeance G3, Shroud G3). *Minus*: a second trade, or a growth that placed less than
  the full presence (Many Minds G3). *Problematic*: a Reclaim by turn 3 that the hand
  forced (hand smaller than plays). `pres T3` is presence placed from the track against
  the ceiling; `cards T3` is every card gained by turn 3.
- **stance** — Rei's land-tempo axis: share of handled chains that were *Above* (the
  land never got to Ravage) rather than *On* (solved after the Build or on the Ravage
  turn); 50% or more reads Above.
- **spike** — Rei's x/ycp power spike for the spirits he wrote about (Vengeance 3/3,
  Many Minds 2/4, Shroud 1/3) and the first turn the track met it.
- **above / on / gambled / failed / sacrificed / untouched / late** — chains by class.
  A chain is one Invader card's pass over one land (Explore t → Build t+1 → Ravage t+2).
  *Gambled*: saved with no Spirit action (Event, Fear card, Dahan). *Failed*: the Spirit
  acted before the Ravage and the land Blighted anyway. *Sacrificed*: Blighted with no
  Spirit action, and no card in hand could have acted at that step. *Untouched*:
  Blighted with no Spirit action while a held card could have acted there with a play
  to spare (a capability, not a guaranteed save). *Late*: only touched after the Ravage.
- **blight T4 / end / shape** — Blight added by the end of turn 4 and by the end
  (cascades included), and the simplest growth order that explains the cumulative
  Blight series (Lumia's orders; a curve has to earn its place on a short series).
- **weak turns** — turns with plays left unused or nothing played (Rei: fine when they
  buy a growth).

Caveats: rows marked *heuristic* carry undo uncertainty; `pres T3` can exceed the
ceiling when cards or rules place presence (Green, Ocean); a scenario is not detected
(the Vengeance game shows a pre-game Build and Explore); counts this small show habits,
not significance.

## The games

| spirit | board | adversary | level | diff | result | turns | undo | scaling | stance | pres T3 | cards T3 | reclaim | spike | above | on | gambled | failed | sacrificed | untouched | late | blight T4 | blight end | blight shape | fear cards T4 | weak turns |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Green | A | Russia | 5 | 9 | lost | 6 | exact | Fast | Above | 7/3 | 3 | T4 | - | 5 | 3 | 4 | 2 | 0 | 0 | 0 | 1 | 3 | quadratic | 2 | 2 |
| Behemoth | G | BrandenburgPrussia | 6 | 10 | lost | 7 | heuristic | Fast | Above | 5/6 | 3 | T4 | - | 8 | 5 | 3 | 2 | 1 | 1 | 0 | 4 | 6 | linear | 3 | 7 |
| Days | A | - | - | - | won | 10 | exact | Minus | On | 1/3 | 3 | T4 | - | 6 | 3 | 4 | 6 | 0 | 0 | 0 | 2 | 9 | linear | 2 | 3 |
| Trickster | F | - | - | - | won | 8 | exact | Minus | On | 3/6 | 3 | T5 | - | 3 | 5 | 2 | 4 | 0 | 0 | 0 | 2 | 4 | quadratic | 2 | 3 |
| Lightning+River | NWNE | HabsburgMining | 4 | 7 | lost | 4 | exact | Problematic | On | 3/6 | 1 | T3 | - | 2 | 4 | 1 | 3 | 0 | 0 | 0 | 4 | 4 | linear | 1 | 1 |
| Lure | D | - | - | - | won | 6 | exact | Problematic | Above | 2/6 | 1 | T3 | - | 3 | 2 | 1 | 2 | 1 | 0 | 0 | 1 | 5 | quadratic | 3 | 2 |
| Many Minds | C | - | - | - | won | 6 | exact | Fast | On | 6/6 | 2 | T4 | 2/4cp never | 4 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | constant | 4 | 0 |
| Ocean | D | England | 3 | 6 | won | 8 | exact | Minus | On | 4/6 | 3 | T5 | - | 5 | 6 | 3 | 2 | 1 | 0 | 0 | 1 | 4 | linear | 5 | 1 |
| Ocean | D | - | - | - | open | 3 | exact | Fast | Unknown | 7/6 | 3 | - | - | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 3 | 3 | linear | 1 | 1 |
| Shadows | CE | BrandenburgPrussia | 2 | 4 | lost | 7 | heuristic | Minus | On | 4/3 | 3 | T4 | - | 6 | 8 | 6 | 2 | 4 | 0 | 0 | 4 | 13 | linear | 2 | 1 |
| Shadows | A | BrandenburgPrussia | ? | - | won | 5 | heuristic | Minus | Above | 3/3 | 2 | T5 | - | 3 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | quadratic | 5 | 5 |
| Shadows | B | BrandenburgPrussia | 6 | 10 | won | 6 | heuristic | Fast | Above | 3/3 | 3 | T4 | - | 6 | 3 | 4 | 3 | 0 | 0 | 0 | 3 | 3 | quadratic | 5 | 6 |
| Memory | B | - | - | - | won | 6 | exact | Fast | On | 3/3 | 3 | T5 | - | 2 | 7 | 0 | 1 | 0 | 0 | 0 | 1 | 1 | linear | 4 | 0 |
| Shroud | D | - | - | - | won | 6 | exact | Fast | Above | 5/6 | 1 | T4 | 1/3cp never | 3 | 3 | 0 | 3 | 1 | 0 | 1 | 4 | 6 | linear | 3 | 1 |
| Starlight | C | - | - | - | open | 7 | exact | Problematic | On | 3/9 | 3 | T3 | - | 0 | 8 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | linear | 1 | 0 |
| Stone | A | Scotland | 1 | 3 | won | 7 | heuristic | Minus | Above | 5/3 | 3 | T4 | - | 4 | 4 | 2 | 3 | 0 | 1 | 1 | 3 | 5 | linear | 2 | 7 |
| Thunderspeaker | A | - | - | - | open | 3 | exact | Problematic | Unknown | 3/6 | 3 | T3 | - | 0 | 1 | 0 | 1 | 2 | 0 | 0 | 3 | 3 | linear | 1 | 0 |
| Thunderspeaker | F | Sweden | 4 | 6 | lost | 7 | exact | Problematic | On | 3/6 | 2 | T3 | - | 3 | 5 | 2 | 2 | 0 | 0 | 0 | 2 | 6 | linear | 2 | 2 |
| Vengeance | C | BrandenburgPrussia | 2 | 4 | lost | 4 | exact | Fast | On | 6/6 | 0 | T4 | 3/3cp never | 0 | 3 | 2 | 2 | 2 | 1 | 0 | 7 | 7 | linear | 4 | 2 |
| Vengeance | C | England | 4 | 7 | lost | 6 | heuristic | Fast | On | 5/6 | 1 | T4 | 3/3cp T6 | 2 | 3 | 1 | 0 | 3 | 1 | 1 | 5 | 7 | linear | 7 | 2 |
| Volcano | A | - | - | - | won | 6 | exact | Fast | Above | 5/6 | 4 | T6 | - | 4 | 3 | 2 | 0 | 0 | 0 | 0 | 1 | 2 | linear | 3 | 1 |

## Groupings (18 finished games; counts, not significance)

| grouping | games | won |
|---|---|---|
| scaling Fast | 9 | 5 |
| scaling Minus | 6 | 5 |
| scaling Problematic | 3 | 1 |
| stance Above | 8 | 6 |
| stance On | 10 | 5 |
| blight by T4 at most 2 | 10 | 8 |
| blight by T4 3 to 4 | 6 | 3 |
| blight by T4 at least 5 | 2 | 0 |
| adversary difficulty at most 4 (lowest candidate) | 3 | 1 |
| adversary difficulty 5 to 7 | 4 | 1 |
| adversary difficulty at least 8 | 3 | 1 |
| no adversary | 7 | 7 |
| blight grows constant | 1 | 1 |
| blight grows linear | 12 | 6 |
| blight grows quadratic | 5 | 4 |
| no Sacrificed or Untouched chains | 10 | 7 |
| 1 to 2 Sacrificed or Untouched | 5 | 4 |
| 3+ Sacrificed or Untouched | 3 | 0 |

## Readings

**Rei's builds, read in Brett's games.** The Many Minds win on board C is the Drafter
to the turn: G2, G2, G2 to 2/3cp on turn 3 with the pay-2 minor, Reclaim on turn 4, 3/3cp
(Rei's Majors spike) on turn 5, zero Blight by turn 4. The Shroud win on D has the
Unleashed shape: G2, G2, G3 with one card by turn 3 (5 of 6 presence), Reclaim on turn 4,
a 60% Above share. The Vengeance loss against Prussia level 2 matches none of Rei's
paths: three G2s up the Energy track (2/2cp, then 3/2cp, 3/2cp), no card by turn 3, two
weak turns (1 of 2 plays on turns 1 and 3), presence destroyed on every turn, seven
Blight by turn 4. Rei's Fast Minors path takes the minor on turn 1 and the two bottom
slots on turns 2 and 3, reaching 2/3cp on turn 3 before the Reclaim. Three plays never
came.

**Leaving lands to Blight.** With reach computed, the Prussia 2 Vengeance game's three unhandled
chains split: two *Sacrificed* (Plaguebearers was the only Slow card in hand and needs
Disease in the land) and one *Untouched* (Fiery Vengeance could have put 1 Damage into
Wetland C5 in turn 1's Fast phase with a play to spare; 1 Damage would not have saved a
land with a Town, so this is a capability, not a save). All three games with three or more
left-to-Blight chains were lost; Prussia 2 is one of them.

**Adversaries.** Read from the saves: Prussia 6 three times (Behemoth on G lost, Shadows
on B won, Shadows on A unknown level), Prussia 2 twice (both lost), England 4 (lost),
England 3 (Ocean, won), Russia 5 (lost), Sweden 4 (lost), Habsburg Mining 4 (lost),
Scotland 1 (Stone, won). All seven games without an adversary were won; three of the
eleven adversary games were. The losses are the adversary games at difficulty 4 and
up, with the Shadows win against Prussia 6 the exception worth a hindsight pass.

**Forced Reclaims.** Thunderspeaker reclaimed on turn 3 in both games with the hand
short of the plays, Rei's Problematic shape; the Sweden 5 game was lost, the other is
open. Lure (turn-3 Reclaim, won) and Lightning with River (turn-3 Reclaim, lost at
Habsburg Mining) are the other two.

**Blight shape.** Blight grew linearly in eleven games and faster than linearly in five
(Green vs Russia, Trickster, Lure, both Shadows wins); Many Minds added none.

## Card pricing (`si cards --fit`)

The effect parser types 220 of 321 Power Cards fully (83 of 101 Minors, 42 of 78
Majors, 95 of 142 Uniques) and 133 of 192 innate tiers; the rest keep their unparsed
sentences and are listed by `si cards --unparsed`. Cards that can act on an Invader
step: 40 at the Explore, 150 at the Build, 219 at the Ravage.

Regressing cost on what a card does (counts of Damage, Defend, Push, Gather, Fear,
Blight, tokens, skips, Energy and card gains, plus speed, range and kind):

| fit | cards | R² | what the coefficients say |
|---|---|---|---|
| all kinds | 220 | 0.79 | a Major costs 2.56 Energy more than its counted effects; damage to each Invader 0.60, Blight removed 0.60, range 0.17 per land, Slow −0.12 |
| Minors | 83 | 0.36 | Blight removed 0.59, Destroy 0.22, Gather Dahan 0.16, damage to each 0.16, range 0.13, Slow −0.10; everything else is within the noise of a 0-or-1 cost |
| Majors | 42 | 0.60 | damage to each Invader 0.83, Gather Invaders 0.54, Blight removed 0.54, Push Invaders 0.49, Slow +0.39, Sacred-Site range +1.03 |

Minors priced above their counted effect: Like Calls to Like, Confounding Mists,
Visions of Fiery Doom, Entrancing Apparitions. Below it (0-cost cards whose text the
parser values fully): Lure of the Unknown, Rain of Blood, Drift Down into Slumber, Flow
Downriver Blow Downwind, Territorial Strife. Thresholds and conditional text are not
yet valued, so these are the first residuals to read, not verdicts.

## Limits

- Without a save the adversary level is a candidate set, and bare logs whose
  Fear-card counts were disturbed by undo re-logging read `?`. Scenarios are read
  from the save only.
- 41 of Brett's 61 finished games have a save but no text log; they are not in this
  table yet. The save carries the engine's journal of the whole game, which the
  hindsight spec names as the next source.
- *Untouched* says a held card could act at that step with a play to spare, not that
  it would have saved the land. Across two boards the layout is not logged, so range
  between boards is unknown and the plays-left reading stays.
- Rows marked heuristic carry undo uncertainty (5 of 20).
- Growth shapes on six-turn series are coarse by design.

## Next

- Rei's openings played on Steam and captured, so the classifier is tested on real
  logs, not transcriptions: Vengeance Fast Minors (G3, G2, G2), Many Minds Drafter and
  Extra Friend, Shroud Unleashed.
- Value the chain, not just its price: Blight prevented, Dahan saved (weighted per
  spirit, D1) and Fear per card, so `V / card` is a number.
- Thresholds and conditional text in the pricing features; the Fear and Event odds
  Rei quotes (40.7%, 34%, 56%) from the decks.
- Scenario detection from the setup lines; element checks on reach (a threshold
  effect counts only when the elements were there).
