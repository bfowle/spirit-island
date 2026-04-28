# Board Shaping — Push, Kill Zones, and Explore Lockout

Every Spirit Island turn eventually reduces to one question: *where do the pieces go?* Push them toward the coast, or into a kill zone? Leave this land at 2 Invaders, or pull a fourth one in and light it up? The answers depend on three decisions that compound across turns — get any of them wrong and the board shape collapses faster than your Fear track can generate burst.

This chapter is the decision framework. It assumes you've read [Adjacency & Range](adjacency-and-range.md) and [Terrain](terrain.md); those cover *what* moves where, not *when you should want it to*.

## Why board-shaping matters

Fear generation gets the headlines, but **board state is the scoreboard** for most games. A board with 4 Blight and 12 Invaders on T5 is losing regardless of fear count; a board with 0 Blight and a known-dying ravage lane is winning regardless of whether Terror 2 has hit. Every push, every gather, every Remove/Destroy choice is shaping that state.

The critical property of board state: it compounds. A Town you left alive on T3 is a City that drops two Invaders into an adjacent land on T6. A coastal land you diluted into has two more Explorers by T5 because the Ocean never stops seeding. Decisions made at mid-game get amortized across every remaining turn.

## The three decisions inside every push

When a power offers "push up to N pieces," you're actually making three decisions in sequence:

1. **Direction.** Coastal, interior, into a land with Dahan, into a land adjacent to your planned kill zone?
2. **Density.** Concentrate pieces into a single land (for an AOE) or dilute them so no land hits ravage critical mass?
3. **Durability.** Will the push put pieces somewhere they'll *stay* dead after the kill, or somewhere Explore will re-seed them next matching draw?

Good players juggle all three implicitly. New players usually fixate on #1 and treat #2 and #3 as rounding errors — which is why T6 always looks worse than T3 did.

## Coast always explores — the statistical floor

```admonish abstract title="Exact rule — Explore (Invader Phase, 3c)"
Add an Explorer to every land of the shown type which either:
- Contains a Town or City; or
- Is adjacent to a Town, City, or **Ocean**.

No matter how many sources are in or adjacent to an Explored land, you only add one Explorer.
```

Source: see [data/references/rules/explore-phase.md](https://github.com/brettfowle/spirit-island/blob/main/data/references/rules/explore-phase.md) for the verbatim rule and Wiki revision ID.

The "adjacent to Ocean" clause is the fault line every board-shaping decision runs along. **Every coastal land is a permanent Explore source.** You cannot Explore-lock the coast short of removing it from the map entirely (which only a few scenario/power effects can do). An interior land, by contrast, can go Explore-dead: if neither the land itself nor any of its adjacent lands contains a Town or City, a matching Explore card adds nothing.

### The statistical weight

Each Invader Deck card is one of four terrains (or a Stage-specific terrain pair). On any given Explore:

- **Coastal lands matching the terrain**: every single one gets an Explorer. No exceptions.
- **Interior lands matching the terrain**: only the ones still linked to a Town/City chain get an Explorer.

On a standard 8-land board, roughly half the lands are coastal. So at least half of every matching Explore draw is *guaranteed* to land on the coast. Clearing coastal Invaders buys you one turn of relief; Explore refills them on the next matching card. Clearing interior Invaders — *and* the building chain feeding them — buys you permanent relief until Build or an Event reseeds.

This is why experienced players prioritize clearing interior Towns: the kill is durable. A coastal Town killed on T4 becomes an Explorer on T5 (matching terrain) and a Town again on T7 Build. An interior Town killed on T4, with its adjacent chain also broken, stays dead.

```admonish tip title="Practical heuristic"
Before you clear an interior land, check the 4–6 lands adjacent to it. If none of *them* contains a Town/City either, your clear just Explore-locked a pocket of the island. That's a 3-turn head start on Blight.
```

## Concentrate vs. dilute — decision framework

Given the push you're about to make, should you pack Invaders into one land (concentrate) or spread them across multiple lands (dilute)?

| Factor | Favors **concentrate** (kill zone) | Favors **dilute** (buy turns) |
|---|---|---|
| AOE kill power in hand / offering | Yes — Pyroclastic, Terrifying Nightmares, Sleep, Vengeance, Vanish Softly | No — can't punish the pile |
| Fear deck proximity to Terror 2 / 3 | Threshold is 1–2 cards away; mass-kill flips the track | Far from threshold; Fear won't spike anyway |
| Ravage timing | Kill resolves **before** Ravage (Fast AOE, or Slow that beats Ravage) | Ravage imminent — concentration = mass Blight |
| Blight pool health | Healthy — can absorb one bad Ravage if the kill misses | Near cascade — dilute to buy turns |
| Dahan in target land | Empty land or invader-only land | Dahan present — keep them alive, spread Invaders away |
| Adversary ramp | Escalation-lite (base England L1–3) | Escalation-heavy (Prussia Town→City, Sweden Build spam) |
| Coastal stakes | Interior kill zone — coast stays clean | Coast has upgrades already — don't feed it more |
| Explore durability | Interior target — kill is durable if chain also breaks | Coastal target — kill will re-seed anyway |

No single row wins; you're summing the column. Concentration is a big-swing play with a tight window; dilution is a steady-state play that costs you Fear.

```admonish failure title="The classic concentration trap"
Pushing three Invaders into a coastal land on T4 because you had a Slow AOE queued — then the AOE turns out to be conditional on an element you didn't hit, the Ravage fires through 4 Invaders, and the land goes 2-Blight. The concentration was correct *conditional on the kill*; without it, you would have been better off diluting and taking the smaller Ravage.
```

## Adversary riders

Ramp shape changes which of the two defaults is correct. These ride on top of the table above:

- **England (L3+)**: Stage III adds Cities to coastal lands on Build. Diluting into the coast feeds future Cities. Prefer interior kill zones even when AOE is shaky. See [England chapter](../adversaries/england.md).
- **Brandenburg-Prussia**: Sands attracts building upgrades; piling Invaders in Sands is a trap unless the kill is locked the same turn. See [Brandenburg-Prussia chapter](../adversaries/brandenburg-prussia.md).
- **Sweden**: Build-heavy; concentration lets a single Build/Ravage cycle compound. Dilute by default unless the kill is fast.
- **France (Plantation Colony, B&C)**: Dahan attract Invaders. Push Invaders *away* from Dahan lands; concentrate in Dahan-empty lands. See [France Plantation chapter](../adversaries/france-plantation-colony.md).
- **Scotland / Russia**: `[VERIFY — adversary-specific ramp riders pending per-chapter review]`.

## Spirit-archetype riders

Your spirit's native toolkit biases the default:

- **Push / move-heavy** (Keeper, Finder, Wind, River): dilute by default. Your offense is geography, not damage. Kill zones only when a teammate is providing the AOE.
- **Damage-heavy** (Fire, Wildfire, Serpent, Volcano, Lightning): concentrate by default. Your kit is built for kill zones. Dilution leaves power on the table.
- **Isolate-heavy** (Shadows, Serpent): concentrate *and* isolate. A kill zone you can Isolate stops reinforcements for the turn it takes to light it up.
- **Defense-heavy** (Stone, Mist, Vital Strength): dilute by default. Your kit blunts Ravage; you don't need to prevent the Ravage, you need to survive it.
- **Fear farmers** (Bringer, Shadows, Vengeance, Wandering Voice): concentrate in lands where the AOE will generate fear-per-kill, but dilute on off-turns to keep the board stable for long enough to finish the fear deck.

## Worked example — T4 vs. Brandenburg-Prussia L5

Mid-game, 2-spirit: River Surges in Sunlight + Sharp Fangs Behind the Leaves. Your board has three threats:

- Interior Jungle at 2 Towns (adjacent chain: one more Town two lands over).
- Coastal Sands at 1 City + 2 Explorers.
- Interior Mountain at 1 Town (no adjacent Town/City chain).

Ravage Card revealed: Jungle. Fangs has Devouring Teeth queued (Slow, destroys 2 Invaders in a Jungle/Mountain). River has Wash Away prepared with Push-4.

**Decision tree:**

- *Concentrate into the Jungle*: Push the Sands Explorers inland, light up Devouring Teeth for 2 kills. Kills a Town but leaves the other Town and the City. Next turn's Ravage still hits coastal Sands with a City.
- *Dilute from the Jungle*: Push the Jungle Towns into Mountain (Explore-dead) and Sands (coast, stays live). Devouring Teeth loses a target; you've traded 2 kills for durability on the Mountain push.

The Prussia rider says "Sands concentration = trap without locked kill." You don't have a Sands kill. Dilute the Jungle Towns: 1 into Mountain (durable), 1 into Sands (accepting the coast-always-explores cost). Use Devouring Teeth on the remaining Jungle Town; you still get 1 Invader cleared durably, and the Mountain land is now Explore-locked.

## Common mistakes

```admonish failure title="Three patterns to watch for"
1. **Diluting into a coastal land that already has 2+ buildings.** You're adding pieces to a land that Build will escalate anyway. Push-to-coast only works when the coastal land is low-density.
2. **Concentrating without AOE online.** "I'll deal with it next turn" is how kill zones become Blight sources. Don't pile pieces you can't punish.
3. **Ignoring Explore-lockout when clearing interior Towns.** If you clear a Town and leave an adjacent Town alive, the land is not Explore-locked — it still seeds from the neighbor. Full lockout requires clearing the whole adjacent chain.
```

## Further reading

- [Adjacency & Range](adjacency-and-range.md) — what "adjacent" means mechanically.
- [Terrain](terrain.md) — terrain scarcity shapes which lands are candidates for kill zones.
- [Dahan](dahan.md) — Dahan presence flips the concentrate/dilute default.
- [The Fear Track](fear-track.md) — kill zones are fear-burst plays; dilution is fear-slow.
- [The Blight Track](blight-track.md) — Blight health gates concentration aggression.
- [Combos — Terrain Control](../combos/terrain-control.md) — specific power combinations that exploit Explore-lockout.
- [Explore-phase rule reference](https://github.com/brettfowle/spirit-island/blob/main/data/references/rules/explore-phase.md) — verbatim Wiki text + adjacency clarifications.
