# Playtest: Thunderspeaker vs Kingdom of Sweden L4

Date: 2026-10-19 · Steam, solo, board F · capture
`/home/brett/repos/spirit-island/data/steam-capture/live-a251b07f4c4e` (main
checkout, gitignored) · coverage game #3 · **lost Turn 7 to Blight**
(`LoseTooMuchBlight`, in the Ravage).

Game-end-only run. Findings come from the backtest, the review and the game
log. A live screenshot from Turn 6 confirmed that the native app's live mode
reads the game on Windows.

## Rules and forecast

| # | Turn | Finding | Fix |
|---|---|---|---|
| W1 | all | **Fine Steel for Tools and Guns** (L3): "Town deal 3 Damage. City deal 5 Damage" (log: `City3-5`, `Town4-3`). The forecast used 2 and 3, so every Ravage with a Town or City was low. | Fixed. Applied as damage adjustments, named in the ledger. |
| W2 | 3 | **Heavy Mining** (L1): "When a Ravage does 4+ overkill to the land, add an extra Blight." Sands F8 took 7 Damage and 2 Blight; the healthy card went from 3 to 1 in one Ravage. The forecast counted 1. | Fixed. The ledger has `extra_blight`, which counts toward the Blight clock (no extra cascade or Presence loss). The backtest now checks each land's Blight amount. |
| W3 | 4 | Temporary Truce (TL I): "In lands with Dahan: 1 Dahan and 1 Town/City do not participate in Ravages." On Sands F8 the City sat out, so 5 Damage − Defend 4 = 1: no Blight. The forecast said Blight. | Fixed. Applied only in lands with Dahan (`TargetType` LandWithDahan). The pieces that would deal the most sit out (the players' pick, Q21). |
| W4 | 4 | **Swayed by the Invaders** (Escalation): "After Invaders Explore into each land, if that land has at least as many Invaders as Dahan, replace 1 Dahan with 1 Town." Wetland F3's only Dahan became a Town; that land Blighted on Turns 6 and 7. | Fixed. Forecast per flagged card: the explored lands where it happens. Backtest compares the swaps. |
| W5 | 5 | Immigration Slows (TL I): "During the next normal Build, skip the lowest-numbered land matching the Invader Card on each board." It skipped Wetland F3; Wetland F7 still built a City. The forecast skipped the whole Build. | Fixed. A `LowestNumberMatchingLand` cancel skips one land per board, not the card. |
| W6 | 6 | Wave of Reconnaissance (Disease/Strife): "Invaders do not Ravage in lands with Disease/Strife." Panicked by Wild Beasts had just put Strife on Wetland F7's City, so F7 didn't Ravage. The forecast said Blight. Putting Down Roots (Turn 5) has the same section. | Fixed. A Ravage `CancelAction` with `ifTokenTypes` cancels the Ravage where those tokens are; Strife counts on the pieces. Shown as "NO RAVAGE". |
| W7 | 7 | Beset by Many Troubles (TL II): "In each land with Badlands/Beasts/Disease/Wilds/Strife, or adjacent to 3 or more such tokens, Defend 5" showed as not modeled. It didn't reach a Ravage land this time. | Fixed for TL I/II (range 1). TL III ("every token grants Defend 3 in its land and adjacent lands") is still not modeled. |
| W8 | setup | **Royal Backing** (L4): Mountain1 was discarded at setup (Accelerate) and a Town went to a Mountain. The card shows in the discard, so card counting already treats it as seen. | No change. |
| W9 | 7 | With W1–W6, the Turn 7 forecast at the start of the Fast phase reads: Blight card 3 left; F1 and F2 (2 Explorers, 1 Town: 5 Damage each) and F3 (City: 10 Damage, ×2 Blight, cascade) add at least 5. Card runs out, 100%. After the Fast phase: at least 4, still a loss. | Forecast only. The old model reached the same verdict with smaller numbers. |

Regression check: backtests on Test 1, Prussia, Russia and England are
unchanged; Russia still has its one known Escalation miss (G3).

## Review summary

- **Blight:** 1 from setup, then 6 from Ravage: F8 2 on Turn 3 (Heavy Mining), F3 on Turn 6, and F1, F2, F3 on Turn 7. The healthy card started with 3, dropped to 1 on Turn 3, and flipped on Turn 6 to Erosion of Will (3). It emptied on Turn 7's third Ravage.
- **Fear:** 4 cards resolved by Turn 7; Terror Level II from Turn 6.
- **Dahan:** 6 → 2. One was lost to Swayed by the Invaders and the rest to Ravages.
- **Presence:** 6 destroyed, most on Turn 7.
- **Invaders:** 25 added (Explore 16, Build 9). Removed: Slow 11, Fast 6, counterattacks 4, Event 2. Pressure by turn: 10, 12, 13, 12, 15, 21, 14; the largest jump was Turn 6 (+6).
- **Unused:** Turn 2, 1 Card Play with 0 Energy; Turn 5, 1 Card Play with 3 Energy.

## For the retro (facts, two questions)

- **Turn 3, Sands F8** decided the healthy card. Sands I Ravaged F8 with 2 Explorers, a City and 1 Dahan: 7 Damage under Fine Steel, so 2 Blight (Heavy Mining), the Dahan, and one of your Presence. With W1–W2, the companion now shows this on Turn 3 as "BLIGHT ×2". Without them it read as a 1-Blight Ravage.
- **Turn 4** went well: Temporary Truce, Words of Warning and Defend 4 held F8 (1 Damage got through, no Blight), and your Slow phase then cleared its Invaders.
- **Wetland F3** is where the Swayed swap happened. Its Dahan became a Town on Turn 4, it built a City on Turn 6, and it Blighted on Turns 6 (the flip) and 7.
- **Sands F1 and Jungle F2** held only Explorers after the Turn 5 Explore (1 on F1, 2 on F2). They built Towns on Turn 6 (Coastal Build), and on Turn 7 each dealt 5 (Coastal Ravage).
- **Questions:**
  1. On Turn 5 you left a Card Play and 3 Energy unused. Was that deliberate, saving for Turn 6?
  2. With Sweden's Fine Steel and Heavy Mining, would you have played Turn 3 differently if the companion had shown F8 as 2 Blight and the card going to 1?

**Brett (2026-10-19):** (1) Turn 5 is no longer remembered; the unused play was probably deliberate. (2) Yes: the companion must show the actual numbers, adversary modifiers like Sweden's extra damage included. That led to an audit of every adversary rule against the forecast (see the companion spec).
