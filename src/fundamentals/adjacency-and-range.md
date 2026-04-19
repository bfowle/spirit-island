# Adjacency & Range

Adjacency is deceptively simple: two lands are adjacent if they share a border. Range is how many adjacent-hops a power can reach. Everything else — every targeting rule, every push/gather question, every "can I play this card here?" — derives from this.

And despite the simplicity, adjacency questions come up every game, usually under time pressure, usually with real stakes.

## The basic rule

Two lands are **adjacent** if they share a border on the board. Adjacency is symmetric (if A is adjacent to B, B is adjacent to A).

The ocean counts as one "land" for adjacency purposes:

- Coastal lands are adjacent to the ocean.
- Coastal lands on the same ocean are **not** adjacent to each other unless they share a land border.

## Range

Range-N means the target land is N hops away from a qualifying source.

- **Range-0 or "your presence"**: the target land *contains* your presence.
- **Range-1**: target is adjacent to a land with your presence.
- **Range-2**: target is up to 2 hops away from a land with your presence.

Hops count through the ocean for range purposes *only if* the power allows it — most powers restrict to land-to-land adjacency.

### Range examples

- Your presence is in a jungle land. A Range-1 power can target that jungle land OR any adjacent land (jungle's neighbors).
- Two lands apart via a mountain: Range-2 can hop over.
- Two coastal lands, not sharing a border, both adjacent to ocean: Range-1 can NOT hop between them through the ocean (unless the power specifies ocean-traversal).

## "Source of your presence" vs. "your presence"

Many powers distinguish:

- **"Target land with your presence"**: the presence itself must be in the target.
- **"Source of your presence"**: the presence is adjacent to or in the target, depending on range.
- **"Any" source**: no restriction — sometimes with the caveat "from any Spirit's presence" in multiplayer.

Read the card carefully. "Source of your presence" with Range-1 does NOT require your presence *in* the target; only adjacent-from.

## Sacred sites

2+ presence in one land = a sacred site. Some powers key off sacred-site adjacency rather than presence adjacency. For those, a lone presence in an adjacent land does *not* satisfy the targeting — the sacred site itself must be positioned correctly.

## Ocean and coastal

The ocean is one continuous "land space" for targeting purposes in most contexts:

- Powers with "Target: ocean" exist (rare, typically Ocean-spirit cards).
- Coastal lands around the same ocean are not adjacent to each other via ocean (no hop-through) unless stated.
- Ocean's Hungry Grasp has specific exceptions: its presence in the ocean treats coastal lands as adjacent.

## Shroud's Mists Shift and Flow

Shroud of Silent Mist has a special rule that modifies adjacency for its own targeting: when playing a power, Shroud may move (gather) one presence into the target land or an adjacent land to meet range requirements. This effectively extends Shroud's reach by 1 land hop, mid-play, for the cost of disrupting presence position.

Rei's strategic note: this is the rule that gives Shroud its "spatial mobility" feel and supports the damaged-invader farming playstyle.

## Finder and vision

Finder of Paths Unseen uses Vision tokens that interact with adjacency in ways other spirits don't. Targeting can reach further via Vision-token chains. Not a universal rule; Finder-specific.

## Adjacency edge cases

### River crossings

Some boards mark rivers on land borders. By default, rivers don't block adjacency — two lands on either side of a river are still adjacent. But specific powers say "across a river" or "not across a river," modifying from the base.

### Coastal diagonals

Coastal lands touching only at a corner (not a full border) are NOT adjacent. Corner-touching is not shared-border.

### "Within Range N" vs. "Range N"

"Within Range N" includes 0 (your presence counts). "Range N" sometimes excludes 0 — read the card.

### Beast/Dahan movement

Push and Gather move units between adjacent lands by default, unless a spirit rule extends (Many Minds: Beasts move up to 2 lands via Fly Fast as Thought).

## Pre-turn adjacency audit

Before finalizing your turn, check adjacency for each card:

1. **Does my presence (or Sacred Site) satisfy the source requirement?**
2. **Is the target within range from the correct source type?**
3. **Are there any special adjacency rules (Shroud's Mists, Ocean's coastal extension, river crossings)?**

Most adjacency errors come from trying to target a land that *looked* in-range but wasn't. Visual intuition fails on Spirit Island's dense boards — verify by counting hops.

## Common mistakes

```admonish failure title="Common Mistake"
Targeting a Range-1 power from a "near-but-not-adjacent" land. Always count hops. Shared corner is not shared border.
```

```admonish failure title="Common Mistake"
Forgetting Shroud's gather-to-target on a Range-0 power. You have more flexibility than you think — use it.
```

```admonish failure title="Common Mistake"
Treating the ocean as a hop-through space for generic powers. It isn't — ocean is specific to Ocean's Hungry Grasp and a handful of explicit effects.
```

```admonish failure title="Common Mistake"
Ignoring sacred-site-only adjacency. A power with "target land adjacent to your Sacred Site" does NOT trigger off a single-presence land. Check the card's exact wording.
```

```admonish failure title="Common Mistake"
Forgetting "Your presence" vs. "Source of your presence" distinction when the card's effect is flexible. They're different; one requires presence IN the target, the other only adjacent.
```

## Cross-references

- [Terrain](./terrain.md) — adjacency + terrain shape.
- [Presence Economy](./presence-economy.md) — presence placement decisions factor in future range.
- [Spirit Island Wiki — Adjacency rules](https://spiritislandwiki.com/)
- [Querki FAQ — Rules commonly misplayed](https://querki.net/raw/darker/spirit-island-faq/Rules-semi-commonly-misplayed) — has specific adjacency clarifications

---

*Last revised: 2026-04-19*
