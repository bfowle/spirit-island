# Aspect Power Deltas

Aspects replace a spirit's Special Rule (and sometimes an Innate). They're not balanced 1:1 — some are clearly stronger, some weaker, some orthogonal. This chapter catalogs the directional deltas per aspect.

```admonish warning title="Unsourced estimates"
**No Δ figure in this chapter has a cited source.** They are authoring estimates, not computed from any recorded data, so each one is labeled "unsourced" below. The River Sunshine figure has been removed outright: the stat-models audit (`reports/Spirit Island companion stat models.md`, Appendix B) found it unsourced, and an aggregate win-rate delta is the kind of counting that report excludes. Per-aspect × per-spirit × per-adversary × per-level win-rate data is thin: at 100 games a single win rate is still ± 10 points (1.96 × √(0.25 ÷ 100) ≈ 0.098), wider than the deltas below. Treat the table as a list of aspects to try, not a ranking.
```

## How to read the table

- **+X%** = an unsourced estimate that the aspect's win rate is X pp above the base, averaged across matchups.
- **complexity change** = whether the aspect adds rules (↑) or simplifies (↓).
- **axis shift** = whether it moves the spirit toward a different archetype.

## Shadows Flicker Like Flame

| Aspect | Expansion | Complexity | Δ (unsourced est.) | Axis shift |
|--------|-----------|------------|:--------:|------------|
| Base | — | Low | baseline | Fear-rush + Dahan spatial |
| Reach (JE) | JE | Low ↓ | ~+2% | Simpler targeting; no energy tax |
| Amorphous (Promo 2) | Promo 2 | Mod ↑ | ~+3% | Mobility; 2 free presence moves/turn |
| Madness (JE) | JE | Mod ↑ | ~+5% | Strife-adding; fear-per-presence-destroyed |
| Foreboding (Promo 2) | Promo 2 | Mod ↑ | ~+2% | New innate replaces Shadows-of-Dahan |
| **Dark Fire (NI)** | NI | High ↑ | **~+7%** | Moon↔Fire interchangeable |

**Reading** (if the unsourced estimates hold): Dark Fire would be Shadows's strongest aspect — Moon↔Fire swap is load-bearing for Darkness Swallows. Madness offers a meaningfully different style. Reach is the simpler beginner-pick.

## Other aspects (selected highlights)

| Spirit | Strongest aspect | Δ (unsourced est.) | Notes |
|--------|-----------------|:--------:|-------|
| Lightning's Swift Strike | Wind (JE) | ~+4% | More Air access |
| River Surges in Sunlight | Sunshine (JE) | — (removed: no source) | Extra Sun support |
| Bringer of Dreams | Enticing (JE) | ~+5% | Fear-rush acceleration |
| Thunderspeaker | Warrior (JE) | ~+6% | Dahan damage multiplier |
| Ocean's Hungry Grasp | Deeps (NI) | ~+4% | Deeper-water targeting |
| Keeper of the Wilds | Spreading Hostility (NI) | ~+3% | Land-destruction emphasis |
| Serpent Slumbering | Locus (NI) | ~+4% | Presence-of-self variation |
| Heart of the Wildfire | Transforming (JE) | ~+3% | Blight mitigation |

Expansion tags checked against the Steam engine's aspect list (`spirits.json`): Wind and Sunshine are Jagged Earth; Deeps, Spreading Hostility and Locus are Nature Incarnate (corrected from JE). Enticing, Warrior and Transforming are not in the engine list, so their names and expansion tags are unverified.

## What drives a positive delta?

Strong aspects generally:

1. **Reduce an element-access bottleneck** (Dark Fire on Shadows; Wind on Lightning).
2. **Add a no-cost mechanic** (Reach's free range ignore; Amorphous's free presence moves).
3. **Create a new win-condition pathway** (Madness's fear-from-presence-destroyed).
4. **Target a spirit's weakest matchup-axis** (Transforming on Heart mitigates self-Blight hurt).

Weak aspects tend to:

1. Add complexity without expanding capability.
2. Replace a mechanic the base spirit used well.
3. Conflict with the spirit's existing innate architecture.

## When to pick an aspect

- **New to the spirit**: play base 2-3 games first. Knowing what the aspect replaces is load-bearing.
- **Want simpler**: pick the **Low ↓** complexity aspect if one exists.
- **Want stronger in a specific matchup**: check the aspect description for obvious fit.
- **Want experimental**: pick Mod/High aspect that shifts the strategic axis.

## Data sources

- The Δ estimates have no recorded source (no thread, page or date), so every one is labeled unsourced; the River Sunshine figure was removed. Replace an estimate only with a cited figure, and keep aggregate win-rate counting out of it (see the stat-models report).
- The Spirit Island Wiki lists every aspect with replacement text (parsed into each spirit chapter's Aspects section).
- At 100 games per (spirit × aspect × adversary level × player count) cell, a win rate is still ± 10 points, so a few-point aspect delta needs far more games than a cell usually holds.

## See also

- [Reading mindwanderer](reading-mindwanderer.md) — Wilson CI primer.
- [Expansion Dilution Claims](expansion-dilution-claims.md) — related variance analysis.
- Each spirit chapter's **Aspects** section has full mechanics per aspect.
