# Playtest: Vengeance as a Burning Plague vs Brandenburg-Prussia L2, Blitz

Date: 2026-10-19 · Steam, solo, board C · capture
`/home/brett/repos/spirit-island/data/steam-capture/live-cb0788649796` (main
checkout, gitignored) · coverage game #4 · **lost Turn 4 to Blight**
(`LoseTooMuchBlight`, in the Ravage).

An earlier capture, `live-3de87de1fa52`, is a 3-turn start of the same
setup with Thunderspeaker on board A; its backtest is clean too. The
coverage list asked for board G, so board G is still untested.

Game-end-only run. Findings come from the backtest, the review and the game
log.

## Rules and forecast

| # | Turn | Finding | Fix |
|---|---|---|---|
| B1 | 1 | **Blitz, Turn 1 Ravage (NQ-J):** the extra set of setup actions put the setup card in the Ravage space, so it Ravaged on Turn 1. The forecast predicted Wetland C5 and C8; both took Blight, and C5's cascaded into Jungle C4. | Verified in Steam; NQ-J's inference holds. |
| B2 | 1 | **Blitz, extra Explorer:** "When Exploring, once per board, add an additional Explorer." Sands C2 got 2 Explorers on the Turn 1 Explore. | Shown as an option note on each Explore card. |
| B3 | 2, 3 | **Vengeance, The Terror of a Slowly Unfolding Plague:** "When Disease would prevent a Build on a board with your Presence, you may let the Build happen (Removing no Disease). If you do, 1 Fear." The engine asks (`RemoveDiseaseToPreventBuild`, optional). You let Sands C2 build (Turn 2) and Mountain C7 build a City (Turn 3), and spent the Disease on Sands C6. The forecast treated Disease as automatic: 2 Build mismatches. | Fixed. On boards with Vengeance's Presence, a Disease-stopped Build is forecast as a Build with "can skip: spend the Disease". The backtest reads C6 as skipped by choice. |
| B4 | 2 | **Vengeance, Lingering Pestilence:** "When your Presence is Destroyed by anything except a Spirit Action, add 1 Disease where each Destroyed Presence was." Ravage Blight on Mountain C7 destroyed a Presence and added the Disease that C7's Turn 3 Build then met. | Fixed. Ravage Blight on Vengeance's Presence adds a Disease to the forecast board before the Build (shown as an action after the Ravage). |
| B5 | 2 | **Prussia L2's early Stage III card (NQ-P):** MountainJungle3 came up as the Turn 2 Explore card. The forecast listed the Stage III cards as that Explore's options, and its lands matched. | Verified in play. |
| B6 | 3 | **Land Rush:** the forecast listed Jungle C4, Wetland C5 and Wetland C8; you picked C4. | Verified (options). |
| B7 | all | **Blitz, "All Powers are Fast":** every removal this game happened in the Fast phase, but the spirit panel split your hand into Fast and Slow and labeled innates by their printed speed. | Fixed. Under Blitz, every hand card and innate counts as Fast. |
| B8 | setup | **Blitz, extra Blight:** the healthy Blight card had 4 (1 already on the island). | State; no change. |

All seven recorded games now match the engine on every turn.

## Review summary

- **Blight:** 3 on Turn 1 (C5, C8, cascade into C4), 2 on Turn 2 (C3 and C7, which flipped the card to Downward Spiral, 4), 1 on Turn 3 (C6). Turn 3's Event (Blight Spreads) you answered by destroying 2 Presence instead of adding Blight. Turn 4's Event (Accumulated Devastation, blighted side) removed 1 Blight from the card to the box. The Turn 4 Ravage then emptied it.
- **Turn 4:** from the Spirit phase the forecast reads "at least 8 Blight vs 2 left: 100% loss". MountainJungle3 Ravaged Jungle C1, Mountain C3, Jungle C4 and Mountain C7, three of them with cascades.
- **Presence:** 3 destroyed on Turn 1, 8 by the end.
- **Invaders:** 18 added (Explore 12, Build 6). Removed: Fast 5, Ravage 2, Event 2. Pressure by turn: 24, 32, 42, 33.
- **Unused:** Turn 1, 1 Card Play with 0 Energy; Turn 3, 1 Card Play with 5 Energy.

## For the retro (facts, one question)

- On Turn 1 the Blight card went from 4 to 1, before you had a Spirit phase to answer it: that is Blitz.
- At the start of Turn 3 the forecast said "card holds, 0%" (Ravages in Sands C2 and C6). Turn 3's Explore and Build then filled Mountain C7 (7 Explorers, 2 Towns, 2 Cities) and Mountain C3, and Turn 4's Ravage card hit four lands.
- Question: during Turn 3, did you see Turn 4's four Ravage lands coming? The native app lists them under "Next turn's Ravage". Should a Ravage that will empty the Blight card next turn be louder than that?

**Brett (2026-10-19):** "T3 round land ravage, not really didn't see that coming."
So the companion now plays this turn's forecast forward one turn
(`invaders::next_turn`) and counts next turn's Ravage against the Blight that
will be left (`BlightStats.next_turn`). At the start of this game's Turn 3 it
reads: Mountain + Jungle III Ravages C1, C3, C4, C7, at least +10 Blight vs 1
left, so the card runs out (loss). In the native app that is a red callout at
the top of the Invader Phase view (amber when it would only flip the card),
plus a line in the Blight panel.
