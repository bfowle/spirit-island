---
name: si-aspect-explorer
description: Use when Brett asks to "try an aspect" or "what aspect for {spirit}". Walks through the spirit's aspects with stat-backed deltas and recommends which to try next based on gaps in his playlog. NOT YET IMPLEMENTED — skeleton only; full logic lands in M4.
user-invocable: true
---

# si-aspect-explorer — Aspect Recommendation

**Status**: Skeleton. Full logic lands in Milestone 4 alongside `spirits/aspects.md` and `statistics/aspect-power-deltas.md`.

Intended behavior:

1. Given a spirit (or pick from Brett's mastery list), walk through all its aspects with:
   - What mechanically changes
   - Win-rate delta vs. base (where data exists)
   - Which archetypes each aspect enables
2. Recommend which aspect to try next based on gaps in playlog (archetypes he hasn't practiced).
3. Emphasize when-to-suggest-an-aspect-in-a-group context (per Brett's stated goal — aspects in multiplayer need explicit advocacy).

Until M4 ships: if invoked, fall back to general guidance — "aspects are unplayed in your log; Madness on Shadows is community consensus #1 aspect pick."
