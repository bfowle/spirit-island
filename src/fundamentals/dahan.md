# Dahan — The Island's People

Dahan are the native humans of the island — not spirits, not invaders. They're neutral pieces that act on invaders in specific conditions, absorb damage during ravages, and generate fear when defending. Getting dahan wrong is the fastest path to losing a game you were "winning."

## The three roles of a dahan

1. **Counterattack unit** — when a land is ravaged, surviving dahan deal 2 damage to invaders in that land.
2. **Damage sink** — ravage damage is taken first by dahan (2 HP each); once dahan die, presence gets destroyed.
3. **Scoring asset** — some scenarios + some powers reward dahan preservation.

Ignoring role 1 is the most common mistake. Ignoring role 2 is the most *expensive* mistake.

## Dahan mechanics

- **HP**: 2 per dahan.
- **Damage output (ravage retaliation)**: 2 per dahan.
- **Build / Explore interaction**: dahan don't interact with Build or Explore; only Ravage.
- **Movement**: can be Pushed or Gathered by powers.
- **Destruction**: killed when they take 2+ damage. Destroyed dahan leave no token.

## Dahan in the ravage sequence

During a Ravage:

1. Invaders and dahan calculate damage.
2. Invaders deal damage = total invader health in the land (1 Explorer + 2 Town + 3 City = varies).
3. Dahan deal damage = 2 × surviving dahan count (in retaliation).
4. Damage is dealt simultaneously; then:
   - Dahan damage above 2 per dahan kills them.
   - Invader damage can be absorbed by dahan, then by presence, then land becomes blight.

**Key insight**: you can deliberately put dahan in the ravage line to kill invaders via retaliation, *even if* the dahan die.

### The "martyr" play

On a T2–T3 with 2 dahan in a ravage land containing 1 Town and 1 Explorer:

- Invaders deal 3 damage → kills 1 dahan (2 HP used) + 1 presence (1 HP used).
- Dahan deal 4 damage → kills the Town (2 HP) + the Explorer (1 HP). 1 damage overflow.
- Result: you lose 1 dahan + 1 presence, but the land is *clear* and no blight added.

Without the dahan, that ravage is: 3 damage → 3 presence destroyed → 1–3 blight depending on defend.

**Trade evaluation**: 1 dahan + 1 presence for a clear land + both invaders dead. Usually worth it early-game, sometimes worth it late-game if it prevents a cascade.

## Dahan as fear

Several powers generate fear per dahan in a land. Thunderspeaker is the archetype:

- **Thunderspeaker innates** generate fear scaling with dahan count on-the-board.
- **Dahan Insurrection scenario** — dahan become the scoring asset.
- **Bringer's fear cards** interact with dahan density.

If your spirit cares about dahan, you protect them and sometimes *grow* their population (via certain powers like Manifestation of Power and Glory).

## Dahan in multiplayer

Dahan are shared across all spirits. Neither you nor your teammate "owns" them. This creates coordination questions:

- **Who defends the dahan ravage?** — ambiguous by default. Agree before Fast powers.
- **Who buffs the dahan counterattack?** — typically Thunderspeaker or a dahan-themed spirit.
- **Can dahan be sacrificed across spirits' plans?** — yes; but surprise it-dies-anyway plays feel bad. Flag before playing.

## Dahan by adversary

- **Brandenburg-Prussia**: dahan-friendly; no direct dahan punishment.
- **England**: neutral.
- **Sweden**: neutral.
- **France**: enslavement mechanics — dahan can be removed to the Plantation.
- **Russia**: Settler mechanics — dahan get pushed/killed by Settlers.
- **Habsburg Mining**: neutral.
- **Scotland**: dahan-friendly.
- **Habsburg Livestock**: dahan-friendly.

France and Russia flag dahan-centric spirits (Thunderspeaker, Dahan-Insurrection-scenario builds) as risky.

## Dahan by spirit bias

- **Thunderspeaker**: dahan as power multiplier. Grows dahan density; defends them aggressively; wins via dahan-fear-cards.
- **Fangs**: dahan-adjacent; Ranging Hunt uses dahan+beasts.
- **Bringer**: cares about dahan count for some fear card effects.
- **Wildfire**: doesn't care; will ravage them away.
- **Vengeance**: dahan are secondary; disease concentration is primary.

The spirit chapter's At-a-Glance should flag "dahan-sensitive" as an archetype when it applies.

## Stat insight

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1):

- Games with dahan count dropping below 5 by T4: -22% win rate vs baseline.
- Dahan Insurrection scenario win rate spikes +15% when played with Thunderspeaker.
- Multi-handed 2-spirit runs: Thunderspeaker + Sharp Fangs win +7% vs typical partner distribution (both value dahan differently but compatibly).

Caveats: digital data; dahan tracking is noisy because dahan death-timestamps aren't always logged.
```

## Heuristics

```admonish tip title="Pro Tip"
- Before T1 Slow powers, count dahan in each Ravage land. 0 dahan + presence = blight almost certain. 2+ dahan + no presence = invaders probably die.
- Keep at least 1 dahan per ravage land through T4. After T4, they start dying to high-density ravages regardless.
- If you can move dahan (via Push/Gather), moving them *into* a ravage land is a legitimate offensive play, not just defensive.
```

## Common mistakes

```admonish failure title="Common Mistake"
Letting Thunderspeaker's dahan die to early ravages to save presence. The dahan-fear engine goes cold and you lose the 2-CP spike that was the spirit's plan.
```

```admonish failure title="Common Mistake"
Sacrificing dahan early (T1–T2) to "tank" ravages when defend powers would have worked. Dahan are finite; defend is replenishable. Defaulting to dahan sacrifice tilts long-term dahan population.
```

## Cross-references

- [Thunderspeaker](../spirits/low/thunderspeaker.md) — the dahan-centric spirit.
- [Fear Track](./fear-track.md) — dahan-retaliation kills generate fear.
- [Blight Track](./blight-track.md) — dahan absorption delays blight.
- [Dahan Insurrection](../scenarios/dahan-insurrection.md) — dahan-scoring scenario.

---

*Last revised: 2026-04-19*
