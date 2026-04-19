# Event Deck — Risk Profiles per Expansion

```admonish abstract title="Scope"
The Event deck is introduced by **Branch & Claw** and expanded by **Jagged Earth** and **Nature Incarnate**. Each turn starting T1 (in games using the Event deck), an Event card is drawn and resolved in order of its stages (usually 1-3; some cards go to 5).
```

## Pool size + per-game draws

| Combo | Event pool | Events per 8-round game (mean) |
|-------|-----------:|-------------------------------:|
| **Base only (Horizons + Base game)** | 0 | 0.0 |
| **Base + B&C** | 23 | 8.0 |
| **Base + JE** | 30 | 8.0 |
| **Base + B&C + JE** | 53 | 8.0 |
| **All expansions (Base + B&C + JE + NI + Promos)** | 62 | 8.0 |
| **NI additions only (delta from JE)** | 9 | 8.0 |

## Spirit-favoring vs. Invader-favoring hits

Heuristic regex classifier counting pro-spirit vs. pro-invader phrasing in each Event card's full multi-stage text.

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

- B&C events are the most balanced (+1.09 per-game net favor in simulation).
- JE events skew pro-spirit (+4.87), driven by the larger pool with more Fear-generating cards.
- NI events as a standalone pool skew even more pro-spirit, but when mixed with the full pool, the combined net is +3.65 — JE's positive balance remains visible.
