# Digital vs. Tabletop

Community win rates (mindwanderer and similar) blend tabletop and digital. They shouldn't — the two have measurably different dynamics. This chapter catalogues the differences so you apply appropriate skepticism when using aggregated data to plan tabletop games.

## Where digital differs from tabletop

### Shuffle + draw rigor

**Digital**: cryptographically random; every draw is truly independent.

**Tabletop**: shuffles are imperfect. Classic gaming-shuffle studies suggest 5-7 riffle shuffles for full randomization; most tabletop players do 1-3. This produces subtle clumping — Minors drafted together are slightly more likely to show up again soon than pure-random would suggest.

**Impact**: tabletop Minor drafts have slightly more variance than digital. Spirits that depend on drawing a specific card type (Moon+Fire for Shadows's L1) get worse tail outcomes tabletop.

### Rules strictness

**Digital**: rules enforced. You can't skip a Dahan counterattack, miss a Fear threshold, or forget invader-deck advancement.

**Tabletop**: human-enforced. Forgetting threshold crossings, miscounting innate elements, or failing to resolve Slow events in the right order happens regularly.

**Impact**: tabletop tends to be *easier* than it should be (in the player's favor) by ~3-7% at high difficulty. Your 60% tabletop win rate is closer to 55% digital.

### Turn pacing

**Digital**: unlimited thinking time. Most take 1-2 minutes per turn.

**Tabletop**: group pressure encourages faster turns. Missed tactical opportunities are common.

**Impact**: tabletop plays below-optimal. This hurts high-skill spirits (Fractured Days, Starlight) more than low-complexity spirits (River, Vital).

### Alpha-player effect

**Digital solo**: you're alone.

**Tabletop multiplayer**: whoever knows the game best tends to direct everyone's turns. See [The Alpha-Player Problem](../social/alpha-player-problem.md).

**Impact**: digital multi-handed-solo stats translate poorly to tabletop multiplayer. Three heads on three spirits is NOT one head on three spirits.

## Selection bias

- **Digital data is dominated by online-engaged players** — skews harder-difficulty, more experienced. Rates over-represent this population.
- **Tabletop data is dominated by motivated post-game reporters** — skews toward memorable games, not the middle-of-the-road average.

Both biases inflate dispersion at the tails. Point estimates near 50% are probably closest to truth; estimates at 80%+ or <20% are most suspect.

## Practical implications

1. **Subtract 3-5% from high-adversary-level digital rates** when planning careful tabletop games.
2. **Add variance to tabletop estimates** — your tabletop rate will swing more game-to-game than digital suggests. Imperfect shuffling drives it.
3. **If you play both, keep separate logs.** Don't blend.
4. **Community data informs archetype ranking, not absolute win-rate expectation.** "Bringer is a top-5 solo spirit" is robust across modes; "Bringer wins 63% vs England L6" is not.

## See also

- [Reading mindwanderer](reading-mindwanderer.md) — more on CI interpretation.
- [Self-Tracking Your Games](self-tracking.md) — build a tabletop-specific log.
