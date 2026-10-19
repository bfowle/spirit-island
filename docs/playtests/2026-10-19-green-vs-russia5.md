# Playtest: A Spread of Rampant Green (Tangles) vs Tsardom of Russia L5

Date: 2026-10-19 · Steam, solo, board A · capture
`/home/brett/repos/spirit-island/data/steam-capture/live-27696743c529` (main
checkout, gitignored) · coverage game #1 · **lost Turn 6 to Russia's loss
condition** (`LoseAdversary`).

Game-end-only run (no live observations). Findings come from the backtest
(`si-steam-dump backtest`) and the game log.

## Rules and forecast

| # | Turn | Finding | Fix |
|---|---|---|---|
| G1 | 1–5 | Six Builds/Ravages forecast but cancelled: Brett used **Choke the Land with Green** ("Whenever Invaders would Ravage or Build in a land with your Sacred Site, you may skip it by destroying one of your Presence in that land"). Log: `SelectPieceActivation: Reason=DestroyToPreventBuild` / `DestroyToPreventRavage`, then `CancelAction … Source: A Spread of Rampant Green`. | Forecast: mark each Ravage/Build in a land with Green's Sacred Site as "can be skipped (destroy 1 Presence)", like a skip. Backtest: classify cancels that follow a player's DestroyToPrevent choice as "prevented by choice", not a mismatch. |
| G2 | 5 | **Entrench in the Face of Fear** fired: earning a Fear Card uncovered Stage II `Jungle2`, which went straight to the Build space mid-Spirit Phase (`MoveCardAction: Jungle2 To=Build Space, From=Fear Deck`). The Build slot then held **two cards** (Jungle2, Jungle+Wetland). The backtest predicted no Builds that turn but a City was built on A3. | Forecast every card in the Build slot in order (each card's Build sees the previous card's results; FAQ, multiple Build cards), and check why this turn's forecast came out empty. Same for multiple Ravage cards. |
| G3 | 5 | Explore added an Explorer to Mountain A1 that the forecast didn't expect. | Investigate: likely Russia's Escalation (Stalk the Predators: add 2 Explorers among lands with Beasts) from the Entrench card's first resolution, or Competition Among Hunters. |
| G4 | 6 | Lost to **Russia's additional loss condition**: "Track Beasts Destroyed by Adversary rules. If that number ever exceeds the number of Beasts on the island, the Invaders win." The companion never showed this clock. | Adversary loss conditions as live clocks (with U18's Presence clock): Russia's destroyed-Beast count vs Beasts on the island. |
| G5 | — | Tangles' Isolate (innate Level 3) never fired, so Isolate is still untested. | Retest Isolate in a later game. |
| G6 | — | The capture's `log-stream.txt` again starts with the previous game's lines (all stamped 12:17:05.771). | Same as Prussia playtest U10: cut the recorder's log at this game's `GameView Start`. |

## Review summary

Blight: 2 from Ravage, 1 cascade. Invaders added 21 (Explore 14, Build 7);
removed 8 (Slow 3, Fear 2, counterattacks 3). Pressure by turn 10, 16, 17,
15, 20, 20; largest jump Turn 2 (+6). Energy left unspent: Turn 3 (6), Turn 4
(3).
