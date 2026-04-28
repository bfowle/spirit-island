# Reading Two-Spirit Synergy Data

mindwanderer and the community-run SI Digital Data Analysis series surface per-spirit win rates well, but they don't directly answer the question 2-player groups actually ask: *"given that I'm playing Spirit X, which other spirit should my partner pick?"*

Heidi Kalbe's **Team Spirit Island Tableau dashboard** is the closest public source. This chapter is a reader's guide.

## Source

- [TeamSpiritIsland Tableau dashboard](https://public.tableau.com/app/profile/heidi.kalbe/viz/TeamSpiritIsland/TeamSpirit)

Built on Spirit Island Digital stats data (same pipeline as mindwanderer's broader dataset). Outputs a **pairwise win-rate heatmap**: rows = Spirit A, columns = Spirit B, cells = win rate of the pair across games meeting the dashboard's filters.

## What the cells actually measure

Each cell in the heatmap is:

> Win rate for all digital games where {Spirit A} + {Spirit B} were the two spirits, filtered by (adversary, level, scenario, player-count).

The dashboard lets you toggle those filters. The result cell is a raw win rate, *not* a synergy-corrected metric. That matters:

- A cell reading "Lightning + Shadows 70% vs. England L3" is not strictly measuring synergy. It's measuring the pair's combined performance — Lightning would probably be at 68% with many partners at that level, so the "synergy premium" for Shadows specifically is ~2%, not 20%.
- To estimate actual synergy: `cell_win_rate − (expected_win_rate_from_individual_stats)`. Expected rate is a rough average of each spirit's solo performance at that adversary/level.

## How to read it well

1. **Filter hard.** Default view blends all adversaries and levels. Always filter to the adversary-level you're about to play. Unfiltered numbers are close to noise.
2. **Check n per cell.** Tableau should surface a sample size. Two-spirit cells are always smaller than solo cells — the dataset partitions by pair, which sparsifies fast. Cells with n < 30 are suggestive at best.
3. **Compare to same-row baseline.** Pick your spirit's row. The median cell is your spirit's rough "average partner" win rate. Cells above the median are plausible synergies; cells far above suggest either genuine synergy or small-n variance.
4. **Cross-check with BGG SI Digital Data Analysis Series.** The series (entries 5–13) has pair-performance prose that adds context the raw heatmap hides — e.g., "Lightning + Shadows plays well at L3 *because* Shadows's fear econ feeds Lightning's fear-contingent Powers, not because of element synergy."

## Known limitations

- **Digital-only**: the dashboard is built on Spirit Island Digital data, so the `Digital vs. Tabletop` caveats ([see that chapter](digital-vs-tabletop.md)) all apply. Shuffle quality, rules-strictness, and player-skill distribution skew tabletop numbers away from what the dashboard shows.
- **Skill-heterogeneous**: the dashboard doesn't weight by player skill. Pairs chosen mostly by new players will look weaker than they "should" and vice versa.
- **Methodology changes**: same [BGG thread 3555502 methodology caveat](https://boardgamegeek.com/thread/3555502) that applies to mindwanderer — post-Handelabra-migration data is not identical to pre-migration data. Where the dashboard aggregates across that boundary, treat older comparisons with extra skepticism.
- **Aspect variants** may or may not be disambiguated in the dashboard depending on version. Check the filter panel; if aspects aren't separated, treat each base-spirit cell as an aspect-blend.

## Using the dashboard for game prep

Before a 2-player game:

1. **Filter to your adversary + level + scenario.**
2. **Find your spirit's row** (or your partner's column).
3. **Sort by win rate descending.** The top 3 partner candidates are your leading pair picks.
4. **Sanity-check with n.** If n < 20 on your top pick, it's a tie with the spirit below it.
5. **Cross-reference the archetype recommendations in [Two-Player Dynamics](../social/two-player.md)** — the dashboard surfaces quantitative signal, but archetype-coverage reasoning is the qualitative backstop. They should agree; when they don't, the disagreement usually means small-n or a specific matchup quirk you should dig into.

## Cross-references

- [Reading mindwanderer](reading-mindwanderer.md) — base stats framework and Wilson-CI; same reasoning applies to cell values here.
- [Digital vs. Tabletop](digital-vs-tabletop.md) — caveats for porting digital pair-win-rates to your physical table.
- [Two-Player Dynamics](../social/two-player.md) — qualitative companion to this dashboard; archetype-coverage reasoning.
- [SI Digital Data Analysis Series](https://boardgamegeek.com/thread/3617445/si-digital-data-analysis-series-5-how-does-each-sp) — community prose layer on top of the raw numbers.
