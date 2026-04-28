# Matchup Axis — spirit × adversary ratings

The community maintains a canonical spirit-vs-adversary rating framework called the **Matchup Axis**, most recently versioned as v2.0 on BoardGameGeek. It's a per-cell 1–5 star rating across (spirit × adversary), with annotation for *why* each pairing is hard or easy. It's the fastest way to answer "will this pairing teach me something or punish me?" before a game.

This appendix is not a reproduction of the community table (it's community-maintained and updates periodically). It's a **how to read + use** reference that points back to the source of truth.

## Source

- [Updated Match-Up Axis v2.0 + guide archive (BGG thread 3035670)](https://boardgamegeek.com/thread/3035670/updated-match-up-axis-v20-plus-guide-archive) — master thread with full table + revision history.
- Tier-list videos referenced in the source thread: the [Basic Strategy + Tier List playlist](https://www.youtube.com/playlist?list=PL7VhWAfBC-gD1kC48ciT0srwRwWJb4bqA) has per-adversary discussion.
- Cross-referenced against [mindwanderer digital stats](https://mindwanderer.net/si/) for empirical validation where sample sizes allow.

## Rating scale (community convention)

| Stars | Label | Meaning |
|---|---|---|
| ★★★★★ | Stomps | Spirit actively wins the matchup. Wide strategy space; most openings work. |
| ★★★★☆ | Favored | Spirit has strong tools vs. this adversary; standard openings reliable. |
| ★★★☆☆ | Even | Matchup is fair. Requires normal level of play; no structural advantage. |
| ★★☆☆☆ | Disfavored | Spirit's toolkit fights the adversary shape. Needs specific drafts / Openings. |
| ★☆☆☆☆ | Hard counter | Spirit's native gameplan is actively blocked. Win is possible but heavily draft-dependent. |

## How to read a cell

A Matchup Axis entry is typically three things:

1. **The star rating** (1–5).
2. **A one-line why** ("Shadows vs. Russia: ★☆☆☆☆ — Russia's dahan-kill pressure punishes fear-rush economy that needs Terror 2 to re-open board control").
3. **A corrective note** — what you'd need to change in your standard opening to make it work ("need early Defend-heavy minors; skip Concealing Shadows T1").

## How this book uses the Matchup Axis

Every spirit chapter has an **Adversary Matchup Matrix** section that mirrors the Axis ratings for that spirit across all 8 adversaries. Those ratings sync with v2.0 where the community has consensus and are flagged `[tentative — my own read]` where community coverage is thin.

When our chapter disagrees with the community Axis, it's noted explicitly with reasoning. The Axis is authoritative when community sample sizes are large and the consensus has been stable across multiple revisions; it's advisory when the matchup is new (e.g. NI content pre-stabilization) or when community discussion is split.

## Using the Axis for game prep

1. **Pre-draft check**: before you sit down, look up your spirit × adversary × level cell. A ★★☆☆☆ or lower means budget an extra 15 minutes to re-read your spirit chapter's Adversary Matchup section.
2. **Draft shaping**: the Axis's one-line-why usually names the specific resource/mechanic that's under pressure. Prioritize minors that address it.
3. **Opening selection**: for disfavored matchups, the spirit chapter's "Opening B" or a hybrid opening is usually the answer — not the default Opening A.
4. **Post-game calibration**: if you won a ★☆☆☆☆ matchup, the Axis is your sanity check — you did something unusual (or got lucky). Log specifics so you can reproduce (or so you know it was variance).

## Known limitations

- **Sample size**: for niche combinations (NI spirit × B&C adversary × level 5+), community samples are small and Axis entries carry high uncertainty.
- **Expansion purchase variance**: the Axis implicitly assumes "all expansions". If you're playing Base + JE only, several matchups shift materially (fewer counter-Events available, fewer fear cards, no tokens from one expansion).
- **Player-skill variance**: the Axis is calibrated to experienced play. A ★★★★★ matchup is still hard at level 6 if you haven't learned the spirit.
- **Scenario modifier**: the Axis usually doesn't overlay scenario modifiers. Treat "Blitz" or "Second Wave" as a separate axis of difficulty on top of the star rating.

## Related references

- [Combined Difficulty Chart (BGG filepage 262410)](https://boardgamegeek.com/filepage/262410) — per-cell numeric difficulty (Base adversary level + scenario modifier + spirit handicap).
- [SI Digital Data Analysis Series #5 — per-spirit-vs-adversary performance](https://boardgamegeek.com/thread/3617445/si-digital-data-analysis-series-5-how-does-each-sp) — 2.27M-game empirical backstop.
- [Reading mindwanderer](../statistics/reading-mindwanderer.md) — how to reconcile community ratings with stat-based win rates.
- [Stats-Based True Solo Tier List (BGG 2938959)](https://boardgamegeek.com/thread/2938959/) — adjacent ranking framework focused on solo play.

## Source maintenance

When BGG v3.0 (or the current-latest) publishes, update:

1. This appendix's source URL if the thread ID moves.
2. Per-spirit Adversary Matchup Matrix sections across `src/spirits/`.
3. The [Sources & Citations](sources.md) appendix entry.
