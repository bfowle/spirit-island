# Event Deck — Risk Profiles per Expansion

```admonish abstract title="Scope"
The Event deck is introduced by **Branch & Claw** and expanded by **Jagged Earth** and **Nature Incarnate**. Each Invader Phase, after any Blighted Island effect and before the Earned Fear Cards resolve, you draw and resolve one Event (Branch & Claw rulebook). On Turn 1 the Event is drawn and discarded without resolving; it is public in the Event discard (Jagged Earth rulebook p.12). A typical card has a main Event that branches on the Island's health, the Terror Level or the Invader Stage (or is a Choice Event), plus a Token Event and a Dahan Event. The rulebook calls the two main branches "both bad" and the Token and Dahan Events "potentially good".
```

## Pool size + per-game draws

The pool counts only Events still in use. Nature Incarnate drops three Branch & Claw Events (Outpaced, War Touches the Island's Shores, A Strange Madness Among the Beasts) and replaces War Touches the Island's Shores with Far-off Wars Touch the Island (NI rulebook p.5). Slave Rebellion is added only against France, so the pool column below includes it and the "without France" column doesn't. The all-expansions pool matches the Steam game's (61 + Slave Rebellion; `data/references/rules/open-questions.md` Q15). The deck is not reshuffled during a game (the Steam engine never does), so draws are exchangeable, not independent.

| Combo | Event pool (incl. Slave Rebellion) | Without France | Events drawn / resolved per 8-turn game |
|-------|-----------:|-----------:|-------------------------------:|
| **Base only (Horizons + Base game)** | 0 | 0 | 0 / 0 |
| **Base + B&C** | 23 | 22 | 8 / 7 |
| **Base + JE** | 30 | 30 | 8 / 7 |
| **Base + B&C + JE** | 53 | 52 | 8 / 7 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 62 | 61 | 8 / 7 |
| **NI additions only (delta from JE)** | 9 | 9 | 8 / 7 |

An 8-turn game draws 8 Events and resolves 7. The Jagged Earth rulebook keeps the Turn-1 draw to keep the timing right for France and as a reminder, and notes that not resolving it makes the game "very slightly easier" on average.

## Spirit-favoring vs. Invader-favoring hits

Heuristic regex classifier counting pro-spirit vs. pro-invader phrasing in each Event card's full text: every branch of the main Event plus the Token and Dahan Events, although only one main branch resolves. Every phrase counts the same, so a count is not a measure of how much an Event helps or hurts. The pools include Slave Rebellion (1 spirit hit, 0 invader hits); without France, subtract 1 card and 1 spirit hit from each row that includes B&C.

| Combo | Pool | Spirit-favor total | Invader-favor total | Net favor |
|-------|-----:|-------------------:|--------------------:|----------:|
| **Base + B&C** | 23 | 19 | 17 | **+2** |
| **Base + JE** | 30 | 38 | 22 | **+16** |
| **Base + B&C + JE** | 53 | 57 | 39 | **+18** |
| **All expansions (Base + B&C + JE + NI + Promos)** | 62 | 68 | 43 | **+25** |
| **NI additions only (delta from JE)** | 9 | 11 | 4 | **+7** |

## Top 5 spirit-favoring + invader-favoring events (all expansions)

### Most spirit-favoring

- **Dahan Trade with the Invaders** (Jagged Earth) — spirit-hits 3, invader-hits 0 (net **+3**)
- **Numinous Crisis** (Jagged Earth) — spirit-hits 3, invader-hits 0 (net **+3**)
- **The Struggles of Growth** (Jagged Earth) — spirit-hits 4, invader-hits 1 (net **+3**)
- **Far-off Wars Touch the Island** (Nature Incarnate) — spirit-hits 2, invader-hits 0 (net **+2**)
- **Harvest Bounty, Harvest Dust** (Jagged Earth) — spirit-hits 3, invader-hits 1 (net **+2**)

### Most invader-favoring

- **Farmers Seek the Dahan for Aid** (Branch and Claw) — spirit-hits 0, invader-hits 2 (net **-2**)
- **Wounded Lands Attract Explorers** (Jagged Earth) — spirit-hits 0, invader-hits 1 (net **-1**)
- **Tight-Knit Communities** (Branch and Claw) — spirit-hits 0, invader-hits 1 (net **-1**)
- **Thriving Trade** (Jagged Earth) — spirit-hits 1, invader-hits 2 (net **-1**)
- **Resourceful Populace** (Jagged Earth) — spirit-hits 0, invader-hits 1 (net **-1**)

## Interpretation

```admonish info title="Rules Clarification"
Events were designed to be slightly negative on average, to offset the Disease and Beasts the players start with (designer darker, [BGG 2506650](https://boardgamegeek.com/thread/2506650)). The Branch & Claw rulebook describes most cards' two main branches as "both bad" and the Token and Dahan Events as "potentially good".
```

- The keyword classifier reads every Event pool as net pro-spirit, which contradicts the design intent above. Read that as a limit of the classifier, not a finding: it sums phrases from both main branches and from the "potentially good" Token and Dahan Events, and it gives every phrase equal weight.
- In the simulation (7 resolved Events plus the resolved Fear Cards per game, Slave Rebellion excluded), the net keyword count is +1.25 for Base + B&C, +5.07 for Base + JE, +3.33 for Base + B&C + JE and +3.83 for all expansions; NI's cards alone give +6.79. For scale, Fear Cards alone (the base-only row) net +1.50. These rank the pools by phrasing, not by how hard the Events make the game (see [Deck Expansion Impact](deck-expansion-impact.md)).
- The Turn-1 Event never resolves, so plan Turn 1 around the board, not around the Event.
