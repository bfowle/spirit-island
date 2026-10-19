# Digital vs. Tabletop

Community win rates such as [mindwanderer](reading-mindwanderer.md)'s come from Spirit Island Digital only; they contain no tabletop games. The two modes differ in ways that matter for planning a physical game. This chapter catalogs the differences so you apply appropriate skepticism when using digital data to plan tabletop games.

## Where digital differs from tabletop

### Shuffle + draw rigor

**Digital**: all the randomness happens at setup. The engine seeds one pseudorandom generator (the .NET subtractive generator, seeded from the clock) and uses it to fix the order of every deck: Invader, Fear, Event, Minor and Major. After setup the generator is used only for Power-deck reshuffles, France's and Sweden's Escalations, one Fractured Days Power and three Scenarios. So randomness during play is a hidden order fixed at the start, not fresh sampling. Draws are *exchangeable*, not independent: each unseen card is equally likely to be next, and each card you see changes the odds for the rest (see [Statistics: Why and How](index.md), lens 4). Source: the engine read in the companion's stat-models report (`reports/Spirit Island companion stat models.md`, "One seeded shuffle at setup").

**Tabletop**: the decks are only as random as the shuffle. The draw rules are the same (draw without replacement; unkept Power Cards go to the discard and return only when that deck's discard is reshuffled), so the same exchangeable counting applies if the deck is well shuffled.

**Impact**: unmeasured. No source used in this book measures how much a typical tabletop shuffle changes draw odds. The book's counting assumes a fair shuffle in both modes.

### Rules strictness

**Digital**: rules enforced. You can't skip a Dahan counterattack, miss a Fear threshold, or forget invader-deck advancement.

**Tabletop**: human-enforced. Forgetting threshold crossings, miscounting innate elements, or failing to resolve Slow events in the right order happens regularly.

**Impact**: rules mistakes at the table can push results either way, and missed Invader steps tend to favor the players. No source used in this book measures the size of the effect, so don't apply a fixed correction to digital win rates.

### Turn pacing

**Digital**: no time pressure from other people at the table.

**Tabletop**: group pressure encourages faster turns. Missed tactical opportunities are common.

**Impact** (reasoning, not measured): faster play costs most on spirits with many decisions per turn, so expect complex spirits to lose more at a rushed table than simple ones.

### Alpha-player effect

**Digital solo**: you're alone.

**Tabletop multiplayer**: whoever knows the game best tends to direct everyone's turns. See [The Alpha-Player Problem](../social/alpha-player-problem.md).

**Impact**: digital multi-handed-solo stats translate poorly to tabletop multiplayer. Three heads on three spirits is NOT one head on three spirits.

## Selection bias

- **Digital data covers only the people who play the digital game**, at the difficulties they choose. Whether that population is stronger or weaker than your table is not measured.
- **Tabletop reports** (forum polls, post-game threads) come from players motivated to post, which can skew toward memorable games.

Neither bias is quantified in any source used here. Treat extreme rates with the most suspicion, and read the sample size first (see [Reading mindwanderer](reading-mindwanderer.md)).

## Practical implications

1. **Don't apply a fixed discount to digital rates.** No source measures the digital-to-tabletop gap, so any single percentage would be invented.
2. **Expect your own tabletop results to vary.** Rules slips, rushed turns and shuffling all add noise that digital data doesn't contain.
3. **If you play both, keep separate logs.** Don't blend.
4. **Community data informs archetype ranking, not absolute win-rate expectation.** A ranking such as "Bringer is a top-5 solo spirit" carries across modes better than a single figure such as "Bringer wins X% vs England L6".

## See also

- [Reading mindwanderer](reading-mindwanderer.md) — more on CI interpretation.
- [Self-Tracking Your Games](self-tracking.md) — build a tabletop-specific log.
