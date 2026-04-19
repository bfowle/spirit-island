# Presence Economy

Presence is the game's most overloaded resource. It's simultaneously:

- a **targeting anchor** (most powers require presence in or near the target land),
- a **board currency** (sacred sites unlock innate abilities + certain card effects),
- a **damage sink** (when ravages destroy it, they spare dahan and delay blight),
- a **tempo investment** (growth slots that place presence trade against energy/CP growth).

Treating presence as any one of these is the beginner mistake. This chapter teaches you to reason about all four simultaneously.

## The opening budget

You start with **3 presence on the board** (seeded by the spirit setup) and **10 presence in your supply** to place throughout the game. Over a typical 8-round game you'll place ~7–9 presence. The rest sit in supply or stay on the track unused.

**The rule**: not every presence needs a home. Sometimes the right play is to leave a slot unplaced and take an energy/CP growth instead. Presence just-in-case is a common tempo leak.

## The four presence roles, quantified

### 1. Targeting anchor

Powers with "Target: your presence" or "Range N from your presence" require at least 1 presence in the relevant area. Spread-heavy spirits (Green, Lightning, Thunderspeaker) need broad coverage. Focused spirits (Wildfire, Starlight) need density, not spread.

**Heuristic**: count the ravage lands on your spirit's horizon. You usually need 1 presence adjacent to each critical ravage target. Minimum coverage for most base-adversary games: 5 lands.

### 2. Sacred sites

A land with 2+ of your presence is a sacred site. Many innates and some powers require "target a sacred site." Stone, Keeper, Earth lean on sacred-site density; Fangs deliberately avoids them.

**Heuristic**: if your spirit's innates or cards say "sacred site," you want 2–3 sacred sites by T4. If they don't, a sacred site is wasted presence density — spread instead.

### 3. Damage sink

When a ravage hits a land with your presence, and damage exceeds dahan health, your presence gets destroyed (sometimes along with dahan). Destroyed presence usually adds blight to the land.

This is **presence as insurance**: if a ravage will happen and you can't prevent it, placing presence in that land sacrifices one presence to save 1 blight. 1 blight = 1 step closer to cascade.

**Heuristic**: place presence in high-ravage terrain *before* T3 for exactly this reason. The presence acts as a shield.

### 4. Tempo investment

Every presence placed costs an energy/CP growth you didn't take. Over 8 turns, 4 presence placements means 4 growths not taken elsewhere.

**Heuristic**: if you're at 2E and 3CP by T5 with 5 presence on the board, you spent right. If you're at 1E and 2CP with 9 presence, you over-invested in placement.

## The presence curve

Most spirits follow a predictable presence arc:

| Round | Presence in Supply | Presence on Board | Presence on Track |
|-------|--------------------|-------------------|-------------------|
| Start | 10                 | 3                 | 10 (unlocked)     |
| T1    | 8–9                | 4–5               | 9–10              |
| T3    | 6–7                | 6–8               | 7–8               |
| T5    | 4–5                | 7–9               | 5–6               |
| T7    | 3–4                | 6–8 (decaying)    | 3–4               |

The decay on T7+ is from ravages destroying board presence. If your presence count *rises* past T5, you're over-growing.

## Presence destruction isn't always bad

For most spirits, losing presence is painful. For some, it's the engine:

- **Vengeance**: presence destruction adds Disease. A planned sacrifice accelerates the spirit.
- **Wildfire**: presence destruction enables 1-blight wipes of low-health targets.
- **Serpent**: some aspects convert destroyed presence to other board tokens.

For these spirits, the presence economy flips: spending presence *is* the damage engine.

## Territorial spirits vs. flexible spirits

Some spirits care about which lands their presence occupies. Others don't.

**Territorial spirits** (presence position matters):
- **Shroud** — wants presence surrounding invaders in specific lands.
- **Stone** — sacred-site clusters determine which lands are "safe."
- **Shifting Memory** — presence location gates memory cycling.

**Flexible spirits** (presence location is fungible):
- **Lightning** — short-range; moves where targets are.
- **Trickster** — cares about element sources, not positions.
- **Fractured Days** — the time-skipping engine is mostly independent of position.

Your opening should acknowledge this. Territorial spirits bias toward presence-placement growths; flexible spirits bias toward energy/CP growths.

## The "destroy your own presence" trade

Rare but real. Some plays give you the option to destroy your own presence for an effect (e.g., certain unique powers, certain Majors). Do this only when:

1. The effect clears a ravage that would otherwise cost 2+ blight.
2. The presence destroyed was in a terrible land (low-value for targeting, low-value as sink).
3. Your spirit converts destroyed presence into value (Vengeance, some Incarna spirits).

```admonish failure title="Common Mistake"
Destroying your own presence "for the effect" on a non-Vengeance/Wildfire spirit. Unless the math is clear (2 blight prevented > 1 presence lost), you're compounding a problem.
```

## Reading presence on other spirits (multiplayer)

When you have a partner, count *their* presence too. Cross-spirit presence coverage matters for:

- **Defend sharing** — a Thunderspeaker with dahan nearby can defend your ravage if your presence is adjacent.
- **Fear coordination** — Bringer's fear cards trigger more broadly with more spirits' presence on the board.
- **Territory splits** — see [Teaching Methods](../social/teaching-methods.md) for conventions on who takes which terrain.

## Cross-references

- [Tempo](./tempo.md) — how presence interacts with the energy/CP axes.
- [Terrain](./terrain.md) — where to place presence by land type.
- [Dahan](./dahan.md) — dahan as alternative/complementary damage sinks.
- [Blight Track](./blight-track.md) — the downside of presence destruction.

---

*Last revised: 2026-04-19*
