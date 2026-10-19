# Playtest: Ocean's Hungry Grasp vs Kingdom of England L3

Date: 2026-10-19 · Steam, solo, board D · capture
`/home/brett/repos/spirit-island/data/steam-capture/live-0d0b30b99a33` (main
checkout, gitignored) · coverage game #2 · **won Turn 8 by Terror Victory**
(the Fear deck emptied in the Fast phase).

Level 3 as planned (difficulty 6, Fear deck 4/5/4). It covers L1–L3 and
the Escalation; L4 (High Immigration all game), L5 (Local Autonomy,
Town/City +1 Health) and L6 are outside this game's scope. An earlier
capture of the same setup, `live-5ec66f090060`, stops at Turn 3.

Game-end-only run (no live observations). Findings come from the backtest
(`si-steam-dump backtest`, which now also checks High Immigration and
Escalation Builds), the review, and the game log.

## Rules and forecast

| # | Turn | Finding | Fix |
|---|---|---|---|
| E1 | 1, 3, 6 | **England L1, Indentured Servants Earn Land** ("Invader Build actions affect lands without Invaders, if they are adjacent to at least 2 Town/City before the Build Action"). Built empty: Mountain D5 on Turn 1 (its Explorer had been moved to Wetland D1 in the Fast phase; 4 Town/City adjacent), D5 again from High Immigration on Turn 3, Sands D7 on Turn 3 (4 adjacent), Wetland D3 on Turn 6 (5 adjacent). Wetland D3 on Turn 4 (1 adjacent City) did not. Three Build mismatches. | Fixed. Forecast builds there, counted on the board before each Build card (FAQ, "Does the order of Builds matter"), with the note "No Invaders, but N Town/City adjacent". |
| E2 | 3–5 | **High Immigration**: the card leaving the Ravage slot Builds again before the Ravage. Turn 3: Mountain1, D5 +Town, D8 +City. Turn 4: Wetland1, D1's Build cancelled by Unsettled. Turn 5: Sands1, D4 +Town. The tile went when Coastal2 (Stage II) slid onto it after Turn 5. The forecast only said the tile "holds a card". | Fixed. Builds forecast before the Ravage. They are not normal Builds (FAQ), one-use cancels are shared with the regular Build, and their pieces count for the Ravage and the regular Build. Backtest compares them: all match. The web view lists them above the Build. |
| E3 | 4–6 | **Building Boom** (Escalation) on Jungle2, Wetland2 and Sands2. Turn 4: a three-way tie at 1 Town/City (Wetland D1, Jungle D2, Mountain D5), and you picked D2 (+Town). Turn 5: D2 +City. Turn 6: D2 +Town. The forecast had only a generic note. | Fixed. Forecast lists the tied lands per board and what each would get. The backtest checks for one Build per board among them: all match. |
| E4 | 4 | The first E3 forecast left D2 out of Building Boom because of its Disease token. That Disease had already been spent stopping the regular Coastal Build there. | Fixed. A Disease that stops a forecast Build is removed from the forecast board. |
| E5 | 2 | Influx of Settlers, Dahan section (**Network of Support and Trade**): "When Invaders Ravage, if the land has Dahan, Defend 1 + Defend 1 per adjacent Dahan". Mountain D5 (Town and 1 Dahan, with 5 adjacent Dahan) took no Blight, and its Dahan destroyed the Town. Mountain D8 (no Dahan) took the game's first Blight. The forecast had Blight on both. | Fixed. Defend computed with the neighbors (`AdjacentOnlyDefendPerPiece`). |
| E6 | 6 | Investigation of Dangers (Terror Level II/III): "Invaders do +3 Damage (per land) when Ravaging in lands with Presence". It showed as not modeled. Jungle D2 had your Presence: 13 damage. | Fixed. Applied in lands with Presence. |
| E7 | 6 | The Blight card flipped in the Turn 6 Ravage (healthy side 3: D2's Blight plus its cascade into Sands D4). **Promising Farmlands**' immediate effect added a Town and a City to an Inland land; you picked Sands D7 from D6, D7 and D8. | Not modeled: a flip's immediate effects. Show the Blight card's text with the loss-risk line when a flip is possible. |
| E8 | 8 | The end-of-game save showed Terror Level I: the Terror Level card is gone once the deck empties. | Fixed. Falls back to the Fear deck (III when it is empty). |
| E9 | — | England's loss condition **Proud & Mighty Capital** (7+ Town/City in one land) wasn't shown. Peak this game: 4, on Jungle D2 (Turns 6–7). | Partial: a forecast note with the most now and after this phase's Builds. A proper clock comes with the adversary loss clocks (Russia G4, U18). |

Regression check after the fixes: backtests on Test 1, Shadows vs Prussia 6
and Green vs Russia 5 are unchanged. Russia still has its one known
Escalation miss (G3).

## Review summary

- **Blight:** 3 from Ravage (D8 on Turn 2, D2 on Turns 5 and 6), plus 1 cascade (D4, Turn 6). The card flipped on Turn 6 to Promising Farmlands (4) and ended with 4 left.
- **Fear:** 13 cards. Resolved 2 by Turn 3, 5 by Turn 4, 7 by Turn 6, 10 by Turn 7; the last 3 were earned in Turn 8's Fast phase.
- **Invaders:** 33 added (Explore 20, Build 13). Removed: Slow 17, Fear 11, Event 5, Fast 4, counterattacks 4.
- **Pressure by turn:** 18, 19, 20, 14, 20, 26, 23, 23. The largest jump was Turn 6 (+6).
- **Energy:** every Card Play was used each turn, while Energy piled up: 11 left on Turns 3 and 5, 20 on Turn 6, 12 on Turn 7. Card Plays, not Energy, limited Ocean (you took the Card Plays track on Turn 6).

## For the retro (facts, one question)

- After Turn 2, both Ravage Blights came from Jungle D2. On Turn 4 Building Boom's tie let you choose D1, D2 or D5, and you put the Town on D2. D2 then Ravaged on Turn 5 (Coastal2) and Turn 6 (Jungle2). Its Turn 6 Ravage dealt 13 damage, including Investigation of Dangers' +3 for your Presence, and flipped the Blight card. Mountain D5 is Inland and had no Ravage card coming before Turn 9. Question: was D2 chosen to keep the new Town within reach of your Ocean powers?
