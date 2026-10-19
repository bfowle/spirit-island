# Playtest: Lightning's Swift Strike + River Surges in Sunlight vs Habsburg Mining Expedition L4

Date: 2026-10-19 · Steam, two spirits, thematic map (NorthWest + NorthEast)
· capture `/home/brett/repos/spirit-island/data/steam-capture/live-bae43cb355ed`
(main checkout, gitignored) · coverage game #5 · **lost at the end of Turn
4's Fast Phase to Land Stripped Bare** (`LoseAdversary`: Mountain NE7 held 8
Invaders + Blight).

Game-end-only run. The native app's live mode rendered this game during
Turn 1 (thematic boards and two spirits decode). Findings come from the
backtest, the review and the game log.

## Rules and forecast

| # | Turn | Finding | Fix |
|---|---|---|---|
| H1 | 1–3 | **Mining Boom I** (L3): "After the Build Step, on each board: Choose a land with Explorer. Upgrade 1 Explorer there." It happened every turn, one land per board (NW3 and NE3, then NW3 and NE7, then NW5 and NE7). The backtest counted the upgrades as Builds: 4 mismatches. | Fixed in the backtest: an Explorer fewer and a Town more, with Mining Boom in play, is the upgrade, listed as a rule action. The forecast already listed the options. |
| H2 | 2, 3 | **Ceaseless Mining** (L1): in Mining lands (3+ Invaders) the Build card Ravages instead, and Disease stops that Ravage. Turn 2: Wetland NW3's Build card Ravage was stopped by Disease (1 removed). Turn 3: Mountain NE7's Build card Ravage dealt 9 and added a Blight. | Verified: forecast and engine agree. |
| H3 | 3 | **Mining Tunnels** (Escalation): "On each board, Explore in 2 lands whose terrains don't match a Ravage or Build Card." After Sands II: Jungle NW4, Wetland NW7, Jungle NE2, Jungle NE5, all among the predicted Jungle and Wetland lands. | Verified (options). The backtest now filters the options to the drawn card. |
| H4 | 3–4 | **Land Stripped Bare** (loss): "At the end of the Fast Phase, the Invaders win if any land has at least 8 total Invaders/Blight." Mountain NE7: 6 at the start of Turn 3, 7 after Turn 3's Event, then Ceaseless Mining's Blight made 8. The clock showed the count but not where this Invader Phase would take it, and nothing called it out. | Fixed. The clock forecasts the value after this Invader Phase (the forecast board plus its Ravage and Ceaseless Mining Blight): 6 → 7 at the start of Turn 3, 7 → 8 after its Event. A loss clock within one of its limit gets a callout at the top of the Invader Phase view (amber at one step, red at the limit), with when it's checked. |
| H5 | — | Avarice Rewarded (L1: a cascade becomes an upgrade) never came up: no Blight landed where Blight already was. | Still unverified. |
| H6 | 1 | Native app with two spirits and thematic boards: land names read "NorthWest1", the Spirit panel showed only Lightning. | Fixed (NW1/NE5; each spirit in three lines). |

All seven recorded games now match the engine on every turn.

## Review summary

- **Blight:** 4 in all: NE3 and NE5 (Turn 2 Ravage), NW3 (Turn 3 Ravage), NE7 (Turn 3, Ceaseless Mining). The healthy card went 5 → 3 → 1 and never flipped; the game ended on the other clock.
- **Mountain NE7:** 3 Explorers at setup (a Mining land from the start). Turn 2: 4 Explorers and 2 Towns. Turn 3: 4 Explorers, 3 Towns, 1 Blight = 8.
- **Fear:** 8 per card (2 players), 1 card resolved; Terror Level I throughout.
- **Invaders:** 24 added (Explore 13, Build 11). Removed: Fast 5, Ravage 2, Event 1. Pressure by turn: 39, 45, 57, 57.

## For the retro (facts, one question)

- Land Stripped Bare is checked at the end of each Fast Phase. With NE7 at 8 on Turn 4, only a Fast action in NE7 could save the game. Lightning's Swift Strike can make Slow Powers Fast with Air.
- Question: was the Fast-Phase timing of that check clear in the game, or did it surprise you?

**Brett (2026-10-19):** "not really clear, I'm not familiar with Habs Mining
at all." So timing belongs in the callout (now: "checked at the end of each
Fast Phase"), and an unfamiliar adversary needs its rules explained in
context: N2 adds a Rules view of what is in play.
