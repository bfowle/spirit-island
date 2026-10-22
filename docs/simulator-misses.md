# Simulator misses

Where the simulator still parts from the games, gathered on 2026-10-22 from the 27
recorded games (18 captures and 9 EndGame logs). Two measures, both from `si fuzz`:

| Measure | Now | Command |
|---|---|---|
| Recorded Power uses that, played alone from the board as it began, leave the board, Fear and Defend as the game did | 449 of 545 | `si fuzz --lines 0 --powers` |
| Closed turns whose actual line, written in notation and played back, ends as the record does | 58 of 145 | `si fuzz --lines 0 --boards` |

`si fuzz --use <game> <turn> <Power>` explains one use: the effects run, the notes, the
board as simulated beside the game's. Each Power below is filed by its first
difference; a Power's other misses may have other causes.

## Causes, by uses they cost

### 1. Fear the simulator counts wrong: 9 Powers, 23 uses

The board ends right, the Fear does not. No note says why, so each needs `--use` and the journal. Leads: Smash destroys a Town and the simulator gives 0 Fear for it; Terrifying Nightmares reaches 6 in the game, which points at a threshold the simulator's elements miss; Incite the Mob overcounts (7 against 3).

Smash, Stomp, and Flatten (4/10), Darkness Swallows the Unwary (9/14), Savage Revenge (8/11), Terrifying Nightmares (0/3), Manifest Incarnation (0/2), Paralyzing Fright (2/3), Incite the Mob (1/2), Scarred and Stony Land (1/2), Talons of Lightning (8/9).

### 2. Tokens put where the player chose: 10 Powers, 15 uses

Strife, Wilds and Beasts added or moved to a land other than the game's, or added where the game added none. Not yet checked whether the record's choices hold the player's pick for each.

Impersonate Authority (0/2), Why Don't You And Them Fight (0/2), Gift of the Untamed Wild (0/2), Unexpected Tigers (0/2), Tigers Hunting (1/3), Fire in the Sky (0/1), Ravaged Undergrowth Slithers Back to Life (0/1), The Wounded Wild Turns on its Assailants (0/1), Infestation of Venomous Spiders (0/1), Perils of the Deepest Island (1/2).

### 3. Pieces moved to other lands: 8 Powers, 13 uses

Pushes, gathers and Blight removal that land somewhere other than the game's. Draw Towards a Consuming Void and Crops Wither and Fade also change what is left.

Crops Wither and Fade (2/5), Draw Towards a Consuming Void (0/3), Mantle of Dread (1/3), Plaguebearers (10/11), Pursue with Scratches, Pecks, and Stings (2/3), Wrap in Wings of Sunlight (2/3), Plague Ships Sail to Distant Ports (2/3), Favor of the Sun and Star-Lit Dark (1/2).

### 4. Conditions the executor never evaluates: 5 Powers, 10 uses

The note says `condition not evaluated` for "{Beast} are present", "your {Presence} is present", "you pushed any {Dahan} into a land with {Town}/{City}" and Terror Level conditions (TerrorI, TerrorII, TerrorIII). Each is a check on the land or on what the use did. Likely the cheapest group to close.

Haunted By Primal Memories (3/6), Surging Lahar (0/3), Harbingers of the Lightning (0/2), Pull Beneath the Hungry Earth (0/1), Terror Turns to Madness (0/1).

### 5. Spirit economy and paid choices: 4 Powers, 10 uses

The effect is on the Spirit or costs something the line does not hold: Let's See What Happens draws a Minor and uses it, Tidal Boon gives Energy and a Push from one of the target Spirit's lands, Unleash a Torrent pays X Energy for X Damage, Blur the Arc of Years repeats for Time.

Let's See What Happens (1/5), Tidal Boon (0/3), Blur the Arc of Years (4/6), Unleash a Torrent of the Self's Own Essence (0/1).

### 6. Presence moved or placed: 5 Powers, 8 uses

Presence ending in another land than the game's: likely Thunderspeaker's presence going along with the Dahan (Gather the Warriors, Call to Guard) and Many Minds' Sacred Site moving as a Beast (The Teeming Host Arrives, Boon of Swarming Bedevilment); Solidify Echoes' placement. Not yet traced.

Boon of Swarming Bedevilment (2/5), The Teeming Host Arrives (9/11), Gather the Warriors (6/7), Call to Guard (3/4), Solidify Echoes of Majesty Past (1/2).

### 7. Riders and rules not applied: 4 Powers, 8 uses

Never Heard From Again's rider ("If this Power destroys any {Explorer}, 1 {Fear}. If this Power destroys 5 or more {Explorer}, +1 {Fear}"); Unquenchable Flames' "Invaders do not heal Damage at end of turn"; Absorb Corruption's "you may do both"; Settle into Hunting Grounds' "Your {Presence} may count as {Badlands} and {Beast}" (Creepers in the Russia game deals 2 where the simulator deals 1).

Never Heard From Again (0/3), Unquenchable Flames (1/3), Creepers Tear into Mortar (18/20), Absorb Corruption (0/1).

### 8. Record data: 1 Power, 4 uses

Epidemics Run Rampant, T3 of a bare EndGame log: the game dealt 5 Damage from what the record counts as 4 Disease; an undo in that turn likely left the Disease count one short. A capture (save journal) would settle it.

Epidemics Run Rampant (9/13).

### 9. Defend: 2 Powers, 3 uses

Learn the Invaders' Tactics and Air Moves, Earth Endures grant Defend the simulator does not (0 against 3, 0 against 5).

Learn the Invaders' Tactics (2/4), Air Moves, Earth Endures (1/2).

### 10. "You may instead" read as a condition: 2 Powers, 2 uses

A parser misread, and `si cards` calls these cards "fully read and simulated": "You may instead Drown 1 City" (Ocean Breaks the Shore) and "You may instead Replace 1 Town with 1 {Dahan}" (Call of the Dahan Ways' threshold) read as a Conditional whose condition is the word "instead". Each is a choice against the effect before it (Drown a Town, or instead a City). The parser law (`sentence_holds`) should catch it; then it reads as the Instead marker does, as a choice.

Ocean Breaks the Shore (6/7), Call of the Dahan Ways (0/1).

## Turn-level causes known

A turn can end otherwise than the record for reasons beyond the Powers above:

- **Badlands once per Action.** The register says Badlands add once per Action; the effect
  executor adds them to each Damage of a Power (two Damage tiers, a Collapse inside the
  Power's own Damage). Fix in the executor, held by a law.
- **Adversary and escalation picks.** Q21 (answered): the players choose what is best for
  them. Where the record does not show the pick, the new rules take a fixed land (least
  presence, Dahan, Invaders); they should score the candidates as the Slow targets are.
- **Fear and Event cards' own pushes.** The residual from the log puts their pieces back by
  count, so a pushed piece can land in the wrong neighbor (Vengeance T2: Retreat pushed 2
  Explorers to C7, the simulator has one in C1).
- **Russia 5, Entrench in the Face of Fear.** The Stage II card the Fear deck reveals into the
  Build space, and its escalation in the Build step, are not modeled.
- **A second Spirit's Powers.** In a two-Spirit game the review plays only the first
  Spirit's cards and innates. The other's presence stands where the game had it; its Fast
  effects reach the turn through the log's residual at the first Invader step, its Slow
  ones not at all.
- **Explosive Eruption's X in generated lines.** A line that does not state X destroys no
  presence, so the search never tries Eruption.

## Rules with no game behind them

Written from the printed level text, held by unit tests and the adversary laws, but no
recorded game plays them: England 6, Russia 6, Sweden 5, Habsburg Mining 6, the Habsburg
Monarchy 1, 2, 6 and its escalation, France 1, 2, 4, 6 and its escalation, Scotland 1, 3, 6
and its escalation. One captured game each would confirm or correct them.

## Not modeled

The Habsburg Monarchy 4 (Durable Towns, "Destroy Town" as Damage); France 5 (Blight kept off
the card) and the Fear part of France 6; Scotland 5 (Blight in the Ocean); Pent-Up
Calamity's "If you have removed tokens, return up to 2 of them"; a Repeat another Power
grants (Gift of Twinned Days) is replayed as recorded, never generated.

## Appendix A: every Power that misses

| Power | Exact of uses | First difference (simulated/game) | Note |
|---|---|---|---|
| Smash, Stomp, and Flatten (innate) | 4/10 | EndGame--8584246844275351304.txt T1 G5: Fear 0/1 |  |
| Darkness Swallows the Unwary (innate) | 9/14 | EndGame--8584242708433629839.txt T2 A5: A5 E 2/1, Fear 0/2 |  |
| Epidemics Run Rampant (innate) | 9/13 | EndGame--8584090800205489232.txt T3 C2: C2 T 2/1, Fear 2/3 |  |
| Let's See What Happens (innate) | 1/5 | live-f8731eb4de8b T1 F2: Fear 0/2, Defend 0/3 | not simulated: Spirit economy: Discard Minor Powers from the deck until you get one that targets a land, no... |
| Boon of Swarming Bedevilment | 2/5 | live-08f3197f48e3 T2 C7: C4 Pr 1/0, C7 Pr 2/3 |  |
| Crops Wither and Fade | 2/5 | EndGame--8584242708433629839.txt T5 A2: A2 E 4/3 T 1/3 C 1/0 |  |
| Draw Towards a Consuming Void | 0/3 | live-7c6c9792d645 T6 A2: A3 E 3/2 T 1/0, A4 E 1/0, Fear 8/9 |  |
| Haunted By Primal Memories | 3/6 | live-0d0b30b99a33 T4 D7: Fear 1/3 | condition not evaluated: {Beast} are present |
| Never Heard From Again (innate) | 0/3 | live-d4e99e2d3930 T1 D7: Fear 0/1 |  |
| Savage Revenge (innate) | 8/11 | EndGame--8584090818584452031.txt T5 C2: C2 T 3/2, Fear 0/1 |  |
| Surging Lahar | 0/3 | EndGame--8584246844275351304.txt T1 G5: G5 Ba 0/1 | condition not evaluated: your {Presence} is present |
| Terrifying Nightmares | 0/3 | EndGame--8584248061651619129.txt T4 B3: Fear 2/6 |  |
| Tidal Boon | 0/3 | live-0d0b30b99a33 T3 D0: D0 T 0/1, D1 T 1/0 Dh 3/1, D5 Dh 1/2, D7 Dh 2/3 | not simulated: Spirit economy: Target Spirit gains 2 Energy and may Push 1 {Town} and up to 2 {Dahan} from ... |
| Blur the Arc of Years | 4/6 | live-0494e5c4ee45 T4 A4: A4 Bl 1/0 | not simulated: out of the model's reach: You may repeat this Power on the same land by spending 1 Time |
| Creepers Tear into Mortar (innate) | 18/20 | live-27696743c529 T5 A8: A8 T 1/0, Fear 0/1 |  |
| Gift of the Untamed Wild | 0/2 | live-d4e99e2d3930 T1 -: D5 Wi 1/0, D7 Wi 0/1 |  |
| Harbingers of the Lightning | 0/2 | live-bae43cb355ed T2 NE7: Fear 0/1 | condition not evaluated: you pushed any {Dahan} into a land with {Town}/{City} |
| Impersonate Authority | 0/2 | live-f8731eb4de8b T1 F5: F5 St 1/0 |  |
| Learn the Invaders' Tactics (innate) | 2/4 | live-94b1bc80ae34 T2 B5: Defend 0/3 |  |
| Manifest Incarnation | 0/2 | EndGame--8584090800205489232.txt T5 C6: C6 C 2/1 Dh 0/2, Fear 10/12 |  |
| Mantle of Dread | 1/3 | EndGame--8584242708433629839.txt T1 A5: A5 E 1/2 T 0/1, A6 E 1/0 T 1/0 |  |
| The Teeming Host Arrives (innate) | 9/11 | live-08f3197f48e3 T6 C5: C4 Pr 0/1, C5 Pr 3/2 |  |
| Tigers Hunting | 1/3 | EndGame--8584246844275351304.txt T3 G2: G2 T 2/1, G3 Be 0/1, Fear 2/3 |  |
| Unexpected Tigers | 0/2 | live-f8731eb4de8b T2 F4: F4 Be 2/1 |  |
| Unquenchable Flames | 1/3 | EndGame--8584248061651619129.txt T3 B2: B2 Ba 0/1 | not simulated: Invader-phase rule: Invaders do not heal Damage at end of turn |
| Why Don't You And Them Fight (innate) | 0/2 | live-f8731eb4de8b T5 F1: F1 St 1/0 |  |
| Absorb Corruption | 0/1 | live-f8731eb4de8b T3 F5: F8 Bl 1/0 | not simulated: you may do both |
| Air Moves, Earth Endures (innate) | 1/2 | live-f2d6cf68bb77 T8 C2: Defend 0/5 |  |
| Call of the Dahan Ways | 0/1 | live-e392726411c9 T4 D1: D1 E 0/1 T 1/0 Pr 2/1, D7 Pr 0/1 | condition not evaluated: instead |
| Call to Guard | 3/4 | live-a251b07f4c4e T3 F8: F5 Pr 2/3, F8 Pr 1/0 |  |
| Favor of the Sun and Star-Lit Dark | 1/2 | live-0494e5c4ee45 T5 A8: A5 Bl 0/1, A8 Bl 1/0 |  |
| Fire in the Sky | 0/1 | live-f8731eb4de8b T7 F2: F2 St 1/0 |  |
| Gather the Warriors (innate) | 6/7 | live-a251b07f4c4e T4 F5: F5 Pr 1/2, F8 Pr 4/3 |  |
| Incite the Mob | 1/2 | live-f8731eb4de8b T5 F2: Fear 7/3 |  |
| Infestation of Venomous Spiders | 0/1 | EndGame--8584092615760401096.txt T5 C7: C7 Be 1/2, Fear 1/2 |  |
| Ocean Breaks the Shore (innate) | 6/7 | live-0d0b30b99a33 T7 D2: D2 C 1/0, Fear 0/2 | condition not evaluated: instead |
| Paralyzing Fright | 2/3 | live-94b1bc80ae34 T5 B2: Fear 4/8 |  |
| Perils of the Deepest Island | 1/2 | live-d4e99e2d3930 T3 D7: D1 Be 0/1, D7 Be 1/0 |  |
| Plague Ships Sail to Distant Ports | 2/3 | EndGame--8584090800205489232.txt T8 C2: C1 Di 4/2, C3 Di 5/3, Fear 4/3 |  |
| Plaguebearers | 10/11 | EndGame--8584090818584452031.txt T4 C2: C2 T 3/1, C4 T 1/3 Di 0/1 |  |
| Pull Beneath the Hungry Earth | 0/1 | live-5ec66f090060 T2 D3: D3 E 1/0, Fear 0/1 | condition not evaluated: your {Presence} is present |
| Pursue with Scratches, Pecks, and Stings | 2/3 | live-9ba0c15beb5d T2 C8: C7 E 0/1, C8 E 1/0 |  |
| Ravaged Undergrowth Slithers Back to Life | 0/1 | EndGame--8584090818584452031.txt T3 C8: C7 Wi 0/1 |  |
| Scarred and Stony Land | 1/2 | EndGame--8584246831828257582.txt T6 A7: Fear 0/1 |  |
| Solidify Echoes of Majesty Past | 1/2 | EndGame--8584090800205489232.txt T7 C7: C1 Pr 0/1, C5 Pr 2/3 |  |
| Talons of Lightning | 8/9 | EndGame--8584246844275351304.txt T5 G8: G8 E 1/0 T 1/0, Fear 6/7 |  |
| Terror Turns to Madness | 0/1 | live-94b1bc80ae34 T2 B3: Fear 0/3 | condition not evaluated: TerrorI, condition not evaluated: TerrorII, condition not evaluated: TerrorIII |
| The Wounded Wild Turns on its Assailants | 0/1 | EndGame--8584090818584452031.txt T6 C2: C2 Be 1/2 |  |
| Unleash a Torrent of the Self's Own Essence | 0/1 | EndGame--8584246844275351304.txt T7 G5: G5 E 2/0 T 1/0, Fear 0/1 | not simulated: Spirit economy: Pay X Energy to deal X Damage in a land at {0Range} |
| Wrap in Wings of Sunlight | 2/3 | EndGame--8584242708433629839.txt T5 A7: A4 Dh 0/2, A8 Dh 2/0 |  |

## Appendix B: every closed turn that ends otherwise than the record

By game: the turns that end exactly, then each other turn's lands (simulated/record;
E Explorer, T Town, C City, Dh Dahan, Bl Blight, Pr presence, Be Beasts, Di Disease, St
Strife, Wi Wilds, Ba Badlands).

**live-0494e5c4ee45** (FracturedDaysSplitTheSky vs no adversary): exact 3 of 9 (T1, T3, T9)

- T2: A2 T 0/1 Di 0/1
- T4: A4 Bl 1/0, A8 Di 0/1
- T5: A8 E 1/2 T 1/0 C 0/1 Dh 2/1 Bl 2/1
- T6: A2 E 2/0 T 0/1 Dh 0/1, A5 Bl 1/0
- T7: A1 E 3/2, A5 E 2/1 T 1/0
- T8: A5 Dh 2/1

**live-08f3197f48e3** (ManyMindsMoveAsOne vs no adversary): exact 4 of 5 (T1, T3, T4, T5)

- T2: C4 Pr 1/0, C7 Pr 0/1

**live-0d0b30b99a33** (OceansHungryGrasp vs TheKingdomOfEngland 3): exact 1 of 7 (T1)

- T2: D4 Bl 1/0, D5 T 1/0 Dh 0/1 Bl 2/1
- T3: D1 Dh 3/1, D5 Dh 1/2, D7 Dh 2/3
- T4: D1 T 2/1 C 1/0, D2 T 0/1
- T5: D0 T 1/0, D5 T 0/1
- T6: D1 Dh 1/0, D2 Dh 0/1, D7 T 0/1 C 0/1
- T7: D2 C 1/0

**live-27696743c529** (ASpreadOfRampantGreen vs TheTsardomOfRussia 5): exact 5 of 7 (T1, T2, T3, T4, T7)

- T5: A3 E 4/5, A8 E 4/5 T 1/0 C 1/0
- T6: A1 E 1/0, A3 T 0/1

**live-27696743c529-034924** (ASpreadOfRampantGreen vs TheTsardomOfRussia 5): exact 5 of 7 (T1, T2, T3, T4, T7)

- T5: A3 E 4/5, A8 E 4/5 T 1/0 C 1/0
- T6: A1 E 1/0, A3 T 0/1

**live-3a72b583b5da** (Thunderspeaker vs TheKingdomOfEngland 4): exact 4 of 4 (T1, T2, T3, T4)


**live-3de87de1fa52** (Thunderspeaker vs TheKingdomOfBrandenburgPrussia 2): exact 1 of 2 (T2)

- T1: A3 E 2/1, A8 E 1/2

**live-5ec66f090060** (OceansHungryGrasp vs TheKingdomOfEngland 3): exact 0 of 2

- T1: D2 T 1/0 Dh 1/0, D4 Dh 0/1
- T2: D1 E 1/2

**live-7c6c9792d645** (VolcanoLoomingHigh vs no adversary): exact 5 of 5 (T1, T2, T3, T4, T5)


**live-94b1bc80ae34** (ShiftingMemoryOfAges vs no adversary): exact 2 of 5 (T1, T5)

- T2: B3 T 0/1 Dh 1/2 Bl 1/0, B5 Bl 1/0 Pr 0/1
- T3: B1 E 1/0 T 0/1
- T4: B8 Dh 2/3

**live-9ba0c15beb5d** (ManyMindsMoveAsOne vs TheKingdomOfEngland 4): exact 5 of 6 (T1, T3, T4, T5, T6)

- T2: C2 T 1/2 Pr 0/1, C4 Pr 1/0, C7 E 1/2, C8 E 1/0

**live-a251b07f4c4e** (Thunderspeaker vs TheKingdomOfSweden 4): exact 3 of 6 (T1, T2, T3)

- T4: F5 Pr 1/2, F6 Bl 1/0, F8 E 1/0 Dh 2/3 Bl 4/2 Pr 2/3
- T5: F3 C 1/0
- T6: F7 Bl 2/0

**live-bae43cb355ed** (LightningsSwiftStrike+RiverSurgesInSunlight vs HabsburgMiningExpedition 4): exact 0 of 3

- T1: NE1 E 1/4 T 0/1, NE2 E 1/0 T 1/0, NE5 E 2/0 T 1/0
- T2: NE3 Dh 2/0, NE5 Dh 0/3, NE8 E 1/2, NE9 E 1/0, NW2 T 0/3, NW3 T 3/0
- T3: NE7 E 3/4, NW2 T 3/2, NW8 E 1/0 T 1/0

**live-cb0788649796** (VengeanceAsABurningPlague vs TheKingdomOfBrandenburgPrussia 2): exact 1 of 3 (T1)

- T2: C1 E 2/1, C7 E 4/5
- T3: C3 E 5/4, C7 E 6/7

**live-d4e99e2d3930** (LureOfTheDeepWilderness vs no adversary): exact 0 of 5

- T1: D5 Wi 1/0, D7 E 2/0 Wi 0/1
- T2: D3 E 3/1
- T3: D1 Be 1/2, D7 Be 1/0
- T4: D1 E 3/1, D4 Wi 1/0, D5 Wi 0/1, D7 E 0/2
- T5: D7 Pr 2/1

**live-e392726411c9** (ShroudOfSilentMist vs no adversary): exact 3 of 5 (T1, T2, T3)

- T4: D1 E 0/1 T 1/0
- T5: D1 Pr 0/1

**live-f2d6cf68bb77** (StarlightSeeksItsForm vs no adversary): exact 3 of 7 (T1, T5, T6)

- T2: C1 T 1/0 Dh 0/1 Bl 1/0 Pr 0/1
- T3: C4 Bl 1/0, C5 E 1/0 Dh 1/2 Bl 2/1 Pr 1/2
- T4: C6 T 1/0 Dh 0/1 Bl 1/0 Pr 0/1
- T7: C2 T 1/2

**live-f8731eb4de8b** (GrinningTricksterStirsUpTrouble vs no adversary): exact 1 of 7 (T4)

- T1: F5 E 1/0 T 0/1 St 1/0
- T2: F4 Be 2/1
- T3: F6 Pr 1/0, F8 Bl 1/0
- T5: F1 Bl 1/0 Pr 0/1 St 1/0, F2 E 5/4 T 2/1 C 0/1 Dh 0/2 Bl 3/1 St 1/0, F3 Bl 2/1, F4 E 0/1, F5 Bl 1/0 Pr 1/2
- T6: F1 E 1/0, F2 Bl 2/1, F5 T 2/0 Dh 0/2 Bl 2/0 Pr 0/2 Be 2/1, F8 Bl 1/0
- T7: F2 St 1/0, F6 E 0/1, F7 Di 0/1, F8 T 0/1

**EndGame--8584090800205489232.txt** (VengeanceAsABurningPlague vs TheKingdomOfEngland 4): exact 3 of 7 (T1, T2, T6)

- T3: C2 T 2/1, C3 T 1/0
- T4: C2 Di 3/2, C6 C 0/1
- T5: C1 T 3/2 C 2/3, C3 E 1/0 T 1/0 Bl 2/1, C4 Bl 1/0, C5 Pr 1/2, C6 C 0/1 Bl 0/1 Pr 1/0 Di 0/1, C7 Bl 2/3, C8 T 0/1 Bl 1/0 Pr 0/1
- T7: C1 E 5/4 Pr 0/1, C5 Pr 1/2 Di 0/1

**EndGame--8584090818584452031.txt** (VengeanceAsABurningPlague vs TheKingdomOfEngland 4): exact 2 of 6 (T1, T2)

- T3: C7 Wi 0/1
- T4: C1 E 2/1, C2 T 3/1, C4 Di 0/1, C7 T 0/2
- T5: C2 T 3/2 Di 1/2
- T6: C1 T 2/1, C2 C 1/0 Be 1/2 Di 2/3

**EndGame--8584090834841040893.txt** (VengeanceAsABurningPlague vs TheKingdomOfEngland 4): exact 2 of 5 (T1, T2)

- T3: C5 Pr 1/0 Di 0/1, C8 Bl 1/2
- T4: C2 C 2/1 Dh 1/0
- T5: C2 T 1/0 Bl 1/0 Pr 0/1 Di 2/1

**EndGame--8584090885044354762.txt** (VengeanceAsABurningPlague vs TheKingdomOfEngland 4): exact 3 of 5 (T1, T2, T5)

- T3: C1 E 1/0 T 1/2, C2 T 2/1 C 2/4 Di 2/3, C4 E 1/0 C 0/1, C8 E 2/1 T 0/1
- T4: C2 E 3/2 C 4/3 Di 4/5

**EndGame--8584092615760401096.txt** (ShadowsFlickerLikeFlame vs TheKingdomOfBrandenburgPrussia 2): exact 0 of 6

- T1: E2 T 1/0, E7 E 1/0
- T2: C3 E 1/0 C 1/0, C4 Dh 1/3, E2 Bl 1/0, E6 E 1/0, E7 C 1/0
- T3: E4 E 1/0, E5 Wi 0/1, E7 Bl 1/0
- T4: C2 Bl 1/0 Pr 0/1
- T5: C7 E 1/0 Dh 2/1, E5 E 1/0, E7 T 0/1 C 1/0
- T6: E7 Bl 1/0 Pr 0/1

**EndGame--8584242708433629839.txt** (ShadowsFlickerLikeFlame vs TheKingdomOfBrandenburgPrussia ?): exact 0 of 4

- T1: A5 E 1/2 T 0/1, A6 E 1/0 T 1/0
- T2: A5 E 4/2, A7 T 0/1
- T3: A5 E 1/2, A8 E 1/0
- T4: A2 Bl 1/0, A5 T 0/1

**EndGame--8584246831828257582.txt** (StonesUnyieldingDefiance vs TheKingdomOfScotland ?): exact 1 of 6 (T1)

- T2: A2 T 1/0, A3 T 0/1, A8 T 1/0 Dh 0/2 Bl 1/0 Pr 1/2
- T3: A1 E 1/2 T 1/0, A2 T 0/1 Pr 0/1
- T4: A1 Bl 1/0 Pr 2/3, A4 T 1/0 Pr 2/3, A5 T 1/0
- T5: A1 E 1/0 T 1/0 Pr 2/3, A6 E 1/0 T 1/0 Bl 1/0 Pr 0/1
- T6: A1 T 1/0, A2 Dh 2/3, A3 Bl 1/0 Pr 0/1, A4 Bl 3/2 Pr 3/4

**EndGame--8584246844275351304.txt** (EmberEyedBehemoth vs TheKingdomOfBrandenburgPrussia 6): exact 1 of 6 (T2)

- T1: G5 Ba 0/1, G6 Ba 0/1
- T3: G2 T 2/1, G4 Be 1/2, G5 T 1/0
- T4: G2 E 3/0 Ba 0/1, G5 Bl 1/2, G6 Bl 0/1 Pr 2/1
- T5: G3 T 2/0 C 1/0, G4 Bl 0/1 Be 2/1, G7 Pr 2/1, G8 E 1/0 T 1/0
- T6: G6 E 2/0 T 1/0 Pr 2/3, G7 Dh 2/3 Bl 1/0 Pr 1/2

**EndGame--8584248061651619129.txt** (ShadowsFlickerLikeFlame vs TheKingdomOfBrandenburgPrussia 6): exact 0 of 5

- T1: B3 E 2/1 T 0/2 C 1/0, B6 T 1/2, B8 E 1/0 T 1/0
- T2: B3 E 1/0 T 2/1 Dh 0/2 Pr 0/1, B7 T 1/0
- T3: B1 T 1/0 Bl 1/0, B2 T 2/1 Ba 0/1, B4 Dh 3/4
- T4: B2 T 2/1 C 1/0 Ba 1/2
- T5: B2 E 1/2 T 0/1, B4 E 6/4 T 2/4 C 1/0
