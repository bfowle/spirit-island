# Token Economies

Branch & Claw and Jagged Earth added five token types that turn single-turn Power effects into *persistent* Invader-phase modifiers. Each token is its own sub-economy — placed differently, removed differently, with its own interaction curve. Playing without understanding the five as a system means drafting power cards whose payoff you can't time correctly.

This chapter covers Badlands, Beasts, Disease, Strife, Wilds — plus Isolate, which isn't a token but behaves like a phase-breaking primitive in the same family.

## The common property — persistence

Most Power Card effects resolve this turn and vanish. Tokens are the exception. Once placed:

- They stay in the land (or on the Invader, for Strife) until explicitly removed.
- They compound across turns — two Badlands in a land stack; three Strife on one Town stack.
- They interact with Invader phase steps automatically, not only when a spirit plays a power.

This changes draft math. A token-placing card is paying for *future* turns of effect. A card that puts 2 Badlands in a land is functionally a "+2 damage per ravage until someone clears them" ongoing bonus, not a one-shot.

```admonish tip title="Rule of thumb"
Token cards are under-valued on element-affinity scoring and over-valued on element count. They're worth roughly *1.5× to 2×* their Destroy-equivalent because of persistence. Evaluate by expected future turns alive.
```

## Badlands (Jagged Earth)

> One time each Action Damages Invaders in a land, increase that Damage by 1 per Badlands. One time each Action Damages Dahan in a land, increase that Damage by 1 per Badlands.

**Economy type**: damage amplifier.

Each Badlands in a land adds +1 damage to one Invader-damaging Action per turn *and* +1 damage to one Dahan-damaging Action per turn. Badlands don't damage on their own — they're a multiplier on other damage sources.

**Strategic notes**:
- Badlands damage is **mandatory** when the rule triggers — this matters when you'd rather not amplify, e.g. when Dahan would survive without the +1 but die with it.
- Works with Ravage damage (Ravage is an Action): 1 Badlands + 1 Town Ravaging means 3 damage to the land instead of 2, adding Blight where it wouldn't have.
- Under Ember-Eyed Behemoth and Heart of the Wildfire, Badlands are a core value multiplier — they turn small pings into City kills.

**When to place**: lands with queued multi-instance damage; lands where the spirit has elemental thresholds that re-trigger.

**When to avoid**: lands with high Dahan count where you're running a defensive line (you'll accidentally over-damage your own Dahan).

## Beasts (Branch & Claw)

> Beasts tokens do not have an intrinsic effect.

**Economy type**: Event/Power fuel.

Beasts are dormant on their own. Their value comes from (a) Event cards that trigger Beasts effects and (b) Power cards with "in lands with Beasts: additional effect" riders. Roughly half of B&C+ Event cards have a Beasts-clause; when a Beasts Event triggers, Beasts usually generate Fear, deal Damage, or Destroy Explorers.

**Strategic notes**:
- Beasts are a **bet on Events**. Without Events in play, Beasts rarely pay off.
- Sharp Fangs Behind the Leaves is built around Beasts — every other spirit treats them as supplementary.
- Clumping Beasts in one land is usually worse than spreading them: per-land Event triggers tend to be per-land, not per-token.

## Disease (Branch & Claw)

> When Invaders would Build in a land with Disease, instead remove one Disease from that land.

**Economy type**: Build-prevention fuel.

Disease cancels one Build per token. This is the only passive Build-prevention mechanic in the game. Since [Build-prevention compounds](invader-lifecycle.md) across turns, each Disease token is worth roughly as much as one on-turn Push-or-Clear power aimed at the Build slot.

**Strategic notes**:
- Disease in lands with already-high City density is less useful (Cities mean the Build adds Town regardless, but the land keeps growing because of the City).
- Best targets: lands with 1 Town + 1+ Explorers where you want the whole chain to stop.
- Stackable: 3 Disease = 3 prevented Builds. Combined with interior Explore-lockout (see [Board Shaping](board-shaping.md)), 3 Disease + a cleared adjacent chain can mothball a land for most of the game.

## Strife (Branch & Claw)

> Whenever Invaders Damage the Dahan and/or the land, each attacking Invader with any number of Strife deals exactly 0 Damage and removes one Strife.

**Economy type**: Ravage-cancel per-Invader.

Strife is placed on a **specific Invader**, not the land. Every time that Invader would Ravage, its damage is 0 and one Strife is consumed. Multi-Strife stacks on one Invader cancel multiple future Ravages.

**Strategic notes**:
- Strife is required to fire when triggered — you cannot save it for a "worse" Ravage. Place with that in mind.
- If Invaders Damage other Invaders, Strife does **not** cancel and is not removed — the Invader-on-Invader case is irrelevant to planning.
- Dahan **still counter-attack** when Strife reduces damage to 0 (this is the Strife-specific exception to the normal "no-damage = no-counter" rule). Combined with Thunderspeaker or a Dahan-fishing build, Strife + Dahan in a land is an auto-clear engine.
- Strife is City-equalizer: a Strifed City does 0 damage this turn, effectively demoting it for Ravage purposes.

## Wilds (Branch & Claw)

> When Invaders would Explore into a land with Wilds, instead remove one Wilds from that land.

**Economy type**: Explore-prevention fuel.

Wilds is to Explore what Disease is to Build — one token = one cancelled Explore. With the [Explore rule's coast-always-Explores property](board-shaping.md), Wilds is most valuable on coastal lands (where no amount of interior-chain-breaking can Explore-lock them).

**Strategic notes**:
- Wilds on coastal lands: defeats the Ocean-as-source rule for that one Explore per token.
- Wilds on interior lands: duplicates the effect of breaking the adjacency chain (both prevent Explore). Lower value.
- Stack with Disease for double-prevention in a land you're trying to lock out entirely.

## Isolate (rule, not a token)

Not a token, but part of the same phase-breaking family:

> When adjacency normally matters between this land and another, they instead count as not adjacent.

**Economy type**: adjacency severance.

Isolate makes a land "an island" for rule purposes: Explorers can't push into it, Invaders can't move into it, push/gather/adjacency powers can't target across the line. Isolation is typically granted by Shadows (Sweet Breath of Dreams, etc.), Serpent, and a handful of scenario/Event effects.

**Strategic notes**:
- Isolated coastal land **still Explores from the Ocean** — Isolate doesn't sever ocean-adjacency in the base rule. Check the specific power text; some explicitly also sever ocean-adjacency.
- Isolate stacks with concentration: isolate a kill zone to prevent reinforcements for the turn you need to clear it.
- Isolate combined with Wilds on a coastal land is a full-Explore lockout for one Explore (Wilds consumed, Isolate prevents reinforcement via interior adjacency).

See [Board Shaping](board-shaping.md) for tactical use.

## Comparative token value table

| Token | Placement | Trigger | Per-token value | Scaling |
|---|---|---|---|---|
| Badlands | Land | Any damage-dealing Action | +1 damage on one Invader-dmg Action + one Dahan-dmg Action per turn | Linear (stack to scale) |
| Beasts | Land | Event / Power rider | Variable (often 0 without Events) | Discontinuous |
| Disease | Land | Build would occur | −1 Build (cancelled) | Linear (stack = multiple turns of Build-prevention) |
| Strife | Invader | That Invader would Damage | −all-damage from that Invader for one Ravage | Linear (stack on Invader) |
| Wilds | Land | Explore would occur | −1 Explore (cancelled) | Linear (stack = multiple turns of Explore-prevention) |

## Draft heuristic

- **High token-draft value when**: Events are in play (Beasts spike), your spirit has a damage-amplifier (Badlands), you're running a lockdown board-shape (Disease+Wilds), your partner runs Dahan-heavy offense (Strife to set up counter-attacks).
- **Low token-draft value when**: base game only (Beasts/Strife/Wilds/Disease don't exist), solo kill-race builds (you're winning on Fear, not board-durability), scenario cancels or redirects tokens.

## Common mistakes

```admonish failure title="Three patterns"
1. **Counting a Badlands as "passive Fear"** — Badlands don't generate Fear directly. They amplify damage, which may trigger kill-based Fear, but zero-kill turns with Badlands still generate zero Badlands-fear.
2. **Placing Strife on the biggest Invader.** A City's Strife burns the Strife on a single Ravage; an Explorer's Strife burns on a single 1-damage Ravage that Defend would have absorbed anyway. Target Strife at **Towns** (high damage, many future turns alive).
3. **Stacking Wilds on an interior land that's already Explore-locked.** Wilds does nothing if no Explore would have happened. Coastal lands are where the token pays rent.
```

## Further reading

- [Invader Lifecycle](invader-lifecycle.md) — Disease/Wilds are token-form applications of the Prevent-Builds / Prevent-Explores framework.
- [Board Shaping](board-shaping.md) — Isolate and Wilds complement the Explore-lockout tactic.
- [Dahan](dahan.md) — Strife + Dahan counter-attack is a documented auto-clear pattern.
- [Spirit Island Wiki — Tokens](https://spiritislandwiki.com/index.php?title=Tokens) — canonical rule text for all 5 types.

## Source notes

Rule text verbatim from Spirit Island Wiki, revision 12151 (Tokens page, fetched 2026-04-20) and the per-token sub-pages.
