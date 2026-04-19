---
name: si-combo-drill
description: Use when Brett asks for "Spirit Island combo drill" or "practice a spirit pairing". Picks 2 spirits to play as a pairing and suggests an adversary that tests the combo's failure mode. NOT YET IMPLEMENTED — skeleton only; full logic lands in M3.
user-invocable: true
---

# si-combo-drill — 2-Spirit Pairing Drill

**Status**: Skeleton. Full logic lands in Milestone 3 alongside Part VI (Combos & Archetypes).

Intended behavior:

1. Rotate through archetype pairings:
   - Same-archetype stack (two fear-farms: Shadows + Bringer)
   - Cross-archetype support (direct-damage + fear: Fangs + Many Minds)
   - Spicy unorthodox (Vengeance + Sharp Fangs — territorial conflict; teaches coordination)
2. Tier progression: famous combos → community mid-tier → Brett's weakness areas.
3. Suggest an adversary that *tests* the combo's failure mode (e.g., a combo that fights over territory gets an England to force spread-defense).
4. Link to the archetype page in `combos/` Part once those are written.

Until M3 ships: if invoked, return a fallback message pointing to Shadows + Bringer as a well-documented starter combo (both priority spirits, both fear-focused), vs. England L3.
