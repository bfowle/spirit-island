# The Blight Track

Blight is the game's slow-death timer. Every ravage that can't absorb its damage adds blight. Every 2nd blight in one land cascades — adds blight to adjacent lands, potentially chaining across the board. The blight pool has finite tokens; when it runs out, you lose.

Reading the blight track well is the difference between winning a scrappy late game and losing to a blight cascade you didn't see coming.

## How blight works

### Adding blight

When a ravage's damage to a land exceeds the land's absorption:
1. Dahan absorb first (2 HP each).
2. Your presence absorbs next (1 HP per token, destroys the token).
3. Remaining damage = blight added to the land.

If the land already had blight: the 2nd blight **cascades** — add 1 blight to each adjacent land.

### The blight pool

At setup, you seed a blight pool sized by player count (typically 2 per player + 1 base, but scenario-dependent). Each blight token placed on the board comes from the pool.

When the pool runs out: **you lose**. Full stop.

### Healed blight

Some powers remove blight from a land, returning the token to the pool. These are precious — the archetype's only insurance.

## The blight budget

Over a 6–8 round game, expect:

| Game length | Pool consumed (typical) | Cascades (bad) |
|-------------|-------------------------|-----------------|
| 6 rounds    | 3–4 blight              | 0               |
| 7 rounds    | 4–6 blight              | 0–1             |
| 8 rounds    | 6–8 blight              | 1–2             |
| 9+ rounds   | 8+ (near-cap)           | 2+ (dangerous)  |

If you're at cap by T6, you're in trouble; by T7 you've already lost and don't know it.

## Cascades explained

A cascade adds 1 blight to each **adjacent** land (not just one). A well-connected land in the island's interior can cascade to 4–5 lands in a single event. A corner coastal land cascades to only 1–2.

**Cascade-prevention > cascade-containment**. Once a cascade fires, it often chains (the new blight on adjacent lands pushes them to 2nd-blight soon). Preventing is 5x easier than recovering.

## Reading ahead

Top 3 things to check each turn for blight:

1. **Which lands have 1 blight already?** — those are the cascade candidates.
2. **What's the upcoming ravage?** — if it hits a 1-blight land with overflow damage, you're about to cascade.
3. **Blight pool count** — what's left in the pool?

Most players miss #1. The 1-blight land is often a land they "handled" in an earlier turn; they stopped worrying about it. Cascade arrives from neglect, not from the new ravage directly.

## Mitigation toolkit

### Prevent the overflow

- **Defend** reduces ravage damage directly. The cheapest blight prevention.
- **Dahan** absorb damage before presence; keep dahan alive in 1-blight lands.
- **Presence-sink** in ravage-target lands if dahan aren't enough.

### Prevent the cascade

- **Kill invaders before ravage** (Fast powers). No ravage = no damage = no blight.
- **Push invaders out of 1-blight lands** before ravage.
- **Remove the 1st blight** via blight-removal powers.

### Recover after cascade

Hardest — often requires multiple turns to heal multiple lands. Fangs, Green, and Keeper have the strongest blight-removal toolkits.

## Blight by spirit

All 39 spirits below, grouped by their interaction with blight.

### Blight-removing (actively heals blight)

| Spirit                          | Blight interaction                                                   |
|---------------------------------|----------------------------------------------------------------------|
| Keeper of the Forbidden Wilds   | Heavy blight removal via Unique cards                                |
| A Spread of Rampant Green       | Blight removal + prevention via Plant mechanics                      |
| Sharp Fangs Behind the Leaves   | Blight removal via Beast-themed cards                                |
| Towering Roots of the Jungle    | Blight removal via jungle-defend mechanics                            |
| Fathomless Mud of the Swamp     | Blight prevention via wetland-defend                                  |
| Wounded Waters Bleeding         | Water-Incarna interacts with blight removal *TBD*                     |

### Blight-preventing (prevents ravage damage before it becomes blight)

| Spirit                          | Blight interaction                                                   |
|---------------------------------|----------------------------------------------------------------------|
| Vital Strength of the Earth     | Heavy defend stack; rarely blights                                    |
| Stone's Unyielding Defiance     | Sacred-site defend coverage; rarely blights                           |
| Hearth-Vigil                    | Defends dahan, indirect blight prevention                              |
| Downpour Drenches the World     | Isolation + defend; blocks ravage cascades                             |
| Ocean's Hungry Grasp            | Drowning removes invaders before they ravage; indirect prevention    |
| River Surges in Sunlight        | Push removes invaders from ravage targets                             |
| Lure of the Deep Wilderness     | Pulls invaders out of build/ravage lands                              |

### Blight-positive (intentionally adds blight; scales with it)

| Spirit                          | Blight interaction                                                   |
|---------------------------------|----------------------------------------------------------------------|
| Heart of the Wildfire           | Blight-positive (scales with blight); intentionally adds blight      |
| Vengeance as a Burning Plague   | Blight-positive (island blight fuels damage multipliers)             |
| Volcano Looming High            | Destruction creates blight; scaling late game                         |

### Blight-neutral (depend on defend + dahan; no special interaction)

| Spirit                          | Blight interaction                                                   |
|---------------------------------|----------------------------------------------------------------------|
| Lightning's Swift Strike        | Fast damage prevents ravages; no removal                              |
| Shadows Flicker Like Flame      | Strife reduces ravage damage; no removal                              |
| Thunderspeaker                  | Dahan retaliation prevents ravages; no removal                        |
| Bringer of Dreams and Nightmares | Fear-focused; blight-neutral                                         |
| Finder of Paths Unseen          | Mobility-focused; blight-neutral                                      |
| Serpent Slumbering Beneath the Island | Late-spike damage; blight-neutral                              |
| Devouring Teeth Lurk Underfoot  | Simple damage; blight-neutral                                         |
| Eyes Watch from the Trees       | Support; blight-neutral                                               |
| Rising Heat of Stone and Sand   | Direct damage; blight-neutral                                         |
| Sun-Bright Whirlwind            | Mobility; blight-neutral                                              |
| Shifting Memory of Ages         | Card-focused; blight-neutral                                          |
| Grinning Trickster Stirs Up Trouble | Disruption; blight-neutral                                        |
| Many Minds Move as One          | Beast-fear focus; blight-neutral                                      |
| Shroud of Silent Mist           | Damaged-invader focus; blight-neutral                                 |
| Fractured Days Split the Sky    | Time-skip focus; blight-neutral                                       |
| Starlight Seeks Its Form        | Form-dependent; mostly blight-neutral                                 |
| Ember-Eyed Behemoth             | Badlands-positive (blight as badlands for damage); interacts         |
| Breath of Darkness Down Your Spine | Fear-focused; blight-neutral                                       |
| Relentless Gaze of the Sun      | Sun-Incarna; blight-neutral                                           |
| Wandering Voice Keens Delirium  | Strife-fear; blight-neutral                                           |
| Dances Up Earthquakes           | Earth-Incarna damage; blight-neutral                                  |
| Covets Gleaming Shards of Earth | Major-hoarding; blight-neutral                                        |
| Ferocious Warrior of the Lost Lands | Dahan-warrior focus; blight-neutral                               |

## Blight-positive spirits

Two spirits *want* blight (Wildfire, Vengeance). Their partners must tolerate this:

- **Wildfire** adds blight for damage; partner spirits must not also blight-destructive.
- **Vengeance** treats blight as badlands (damage multiplier). Higher island blight = better Vengeance late-game.

```admonish warning title="Multiplayer Coordination"
A Wildfire or Vengeance spirit at the table means blight numbers approach the cap deliberately. Non-blight-positive partners must pivot to hard defend + blight removal to keep the pool from exhausting.
```

## Blight cards (Branch & Claw+)

B&C introduces a blight deck — 24 cards with specific effects on blight. Ignore at your peril:

- Some cards add blight directly.
- Some cards remove blight.
- Some cards shift blight between lands.

See [Fear, Blight & Event Decks Reference](../appendices/fear-blight-event-decks.md) (M5 milestone) for per-card analysis.

## Scenario blight modifiers

Several scenarios adjust blight:

- **Second Wave**: extra blight seeded at setup.
- **Rituals of the Destroying Flame**: blight accelerates dramatically.
- **Powers Long Forgotten**: limits powers that remove blight.

Always check scenario rules before drafting a blight-sensitive spirit into a scenario.

## Stat insight

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1):

- Games with 0 cascades: win rate ~72%.
- Games with 1 cascade: win rate ~46%.
- Games with 2+ cascades: win rate ~24%.

A cascade roughly halves your win probability. Prevention is worth almost any opportunity cost.
```

## Common mistakes

```admonish failure title="Common Mistake"
Ignoring a 1-blight land after "solving" it. The blight persists until removed. Next ravage in that land can cascade.
```

```admonish failure title="Common Mistake"
Defending the wrong land. Defend a 0-blight land with high dahan → ravage clears it fine without defend. Defend a 1-blight land with low dahan → defend saves the cascade.
```

```admonish failure title="Common Mistake"
Not tracking blight pool count. "We have 6 blight left" sounds fine until you cascade and burn 3 in one fear card draw.
```

```admonish failure title="Common Mistake"
Playing Wildfire + Keeper at the same table without coordination. Wildfire wants blight; Keeper wants to remove it. They can coordinate (Wildfire blights target lands; Keeper heals elsewhere), but untalked-through = both frustrated.
```

## Cross-references

- [Terrain](./terrain.md) — cascade spreads through adjacency.
- [Dahan](./dahan.md) — damage absorption.
- [Presence Economy](./presence-economy.md) — presence as blight insurance.
- [Fear, Blight & Event Decks](../appendices/fear-blight-event-decks.md) — per-card reference.

---

*Last revised: 2026-04-19*
