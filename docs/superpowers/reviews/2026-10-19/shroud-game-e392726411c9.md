# Shroud of Silent Mist, no adversary, board D: the game of 2026-10-19 (capture live-e392726411c9)

Won on Turn 6, Slow Phase: `WinInvadersDestroyed` (board clear at Terror Level II). 303 saves; the forecast matched the engine on every Ravage, Build and Explore of all six turns (backtest: 0 mismatches).

## What happened, turn by turn

| Turn | Invaders (set to act → did) | Shroud | Blight card | Fear |
|---|---|---|---|---|
| 1 | Explore 2 · Build 2 (D5, D8 Towns) · no Ravage | 1 play, 0 Energy: The Fog Closes In (Slow) | 3 | 0 |
| 2 | Explore 2 · Build 1 (D6) · Ravage 2 → 1 Blight (D8), 1 Presence destroyed | Unnerving Pall + Flowing Forms (Fast); 2 pieces Ravaged away, 2 by the Event | 3 → 2 | 1 → 2 |
| 3 | Explore 3 · Build 1 (D3) · Ravage 2 → 2 Blight (D2, D6): the card flips to 5 | Dissolving Vapors + Animated Wrackroot (Slow): 4 pieces removed after the Invader phase | 2 → flips → 5 | 3 → 0, 1 card earned → Terror II on T4 |
| 4 | Explore 2 · Build 1 (D2) · Ravage 1 → 1 Blight (D1), 2 Presence destroyed | Call of the Dahan Ways + Fog (Slow): 2 removed; 3 Energy banked | 5 → 4 | 1 |
| 5 | Explore 2 · no Build · Ravage 1 → 1 Blight + cascade (D2 → D4), 1 Presence destroyed | Savage Transformation (Slow) + Flowing Forms (Fast): 2 removed, 2 by the Event; 3 Energy still banked | 4 → 2 | 0 |
| 6 | Explore 2 · Build 1 (D3) · no Blight | Growth: gained Tsunami (over Instruments, Bloodwrack, Pent-Up Calamity), forgot Savage Transformation; Event Sacred Sites Under Threat: paid 1 Energy, discarded Wrackroot for 2 damage in D2; Tsunami (Slow) on D2: City and Towns gone | 2 | 3 |

Pieces: 21 arrived (2 setup, 13 Explored, 6 Built), 21 removed (12 in Slow Phases, 5 by Events, 3 by Dahan at Ravages, 1 by a Fear card). Invader damage on the board: 13, 11, 11, 12, 9, 7 by turn: the attrition turned on Turn 4. Cost: 5 Presence destroyed by Blight, the Blight card down to 2 of 5 on the Blighted side, so one more Ravage round at Turn 7 (D3 Town, 3 damage) was the loss.

## What the companion said, and why it was wrong

At Turn 6 the versus screen read: pressure 67 against 0 stopped, **7.4× behind, backlog 263, catch up never**, first ending Blight card 100% by T8. Two things were wrong, one small, one structural.

1. **Blight beyond the card was counted.** On an island with Blight in six of eight lands, every Ravage cascades, and the counterfactual (that Ravage stopped) differed from the base by 23 Blight over four phases. Blight past what the card can take is not a cost: the game is already lost. Fixed: Blight per future is capped at what the card can still take (its count, plus the mean hidden side before the flip). Weights at Turn 6 went from Explore 12 · Build 1.2 · Ravage 23.5 to 2.7 · 1.3 · 2.0; the pressure line from 67 to 6.6.

2. **"Catch up" only knew prevention.** Capacity was what the plays *stopped* (actions set to act that did not happen). Shroud did not stop the Invaders; it let them act and removed them afterwards, every Slow Phase, and won the board clear at Terror II once Tsunami landed. That win was never in the model's race: the race only put probabilities on Invader endings and listed the Spirits' endings as text. Fixed: the pace now carries the board-clear side (what stands at this Terror Level against pieces removed minus pieces arriving a turn) and the Fear side, and "catch up" became **"your win"**: the earliest of outpacing, board clear and Fear victory at the current pace.

With both fixes, at Turn 6 start: pressure 10, stopped 5, 2.0× behind, backlog 37; 2 pieces stand at Terror II with 3.0 leaving and 3.6 arriving a turn, so no board clear at that pace (Tsunami, gained that Growth, made it happen); Fear 19 of 36 at 4.0 a turn → Fear victory T11. The screen's verdict went from "never" to "your win · Fear victory · T11".

## What the game itself says about pace

- The Blight-card race was real: 2 left on the Blighted side with Ravages pending; the win came one Ravage ahead of the loss.
- The chain, in this game: three of the five Ravage Blights started as a lone Explorer that one Slow play could have ended (D8 Turn 1, D6 Turn 1, D1 Turn 3; the review's reflection names them). Shroud's Slow kills answered the Builds a turn late each time, which is where the Presence losses came from.
- Energy: 3 banked unused on Turns 4 and 5 with 0 plays left; Shroud's Energy track ended at 2 (0, 1, Water, 2, Air: all revealed) so the flat line on the engine chart is the track, not a bug. The plays track (3, 4, Reclaim, 5) was untouched.
- Turn numbering, settled: the saves are right (the game ended on Turn 6). The log parser counted every `TimePassesAction`, and an undo checkpoint reload re-logs the Time-Passes-to-Spirit-Phase transition, so its labels ran one high ("T7"). It now counts a turn only after an Invader Phase was played.
