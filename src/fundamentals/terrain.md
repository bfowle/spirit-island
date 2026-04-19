# Terrain

Every land on the island is one of four terrains. Terrain determines what powers can target it, where invaders explore to, which innates fire, and which spirits feel at home. Reading the terrain spread on your board — before you even pick growth — is the single skill that separates mid-level play from advanced play.

## The four terrains

| Terrain    | Icon reference  | Notes                                                                |
|------------|-----------------|----------------------------------------------------------------------|
| Jungle     | Green leaf      | Usually the most numerous terrain; many Plant + Dahan effects         |
| Mountain   | Peaks           | Often sparse; where sacred sites consolidate; many Fire/Earth effects |
| Sand       | Desert          | Coastal-adjacent frequently; many Sun effects; often low-dahan       |
| Wetland    | Blue marsh      | Water-heavy; dahan-friendly; slow-build; low ravage density          |

Rivers and mountain-ringed centers don't count as their own terrain — they're land-type modifiers.

## Coastal vs. inland

**Coastal**: the land shares a border with the ocean (the blue area at the edge of the board). Invaders Explore from the ocean into coastal lands.

**Inland**: no ocean border. Invaders Explore into inland from adjacent invader-held lands.

The distinction matters constantly:

- Ocean's innates care about coastal presence.
- England Stage III introduces coastal Cities.
- Explore chain analysis depends on whether the source is a coastal or inland land.

## Terrain and the invader deck

Invader cards specify terrain:

> "Stage I card: Ravage Jungle, Build Mountain, Explore Sand."

The terrain mix of your board determines which cards hit you harder. A jungle-heavy board means jungle Ravage cards hurt more (more lands at once); a mountain-sparse board means Mountain-build cards hurt less but consolidate pressure.

**Pre-game terrain audit**: look at your board before drafting. Count:

- Number of lands per terrain.
- Coastal terrain distribution (jungle + coastal? mountain + inland only?).
- Which terrains have dahan starting in them.

This shapes which opening variant to pick.

## Terrain-gated powers

Many powers specify valid target terrain:

- "Target: Jungle or Wetland."
- "Target: Any terrain."
- "Target: Sacred Site in Mountain."

When drafting, check whether the offered card's terrain restriction matches where your presence is (or will be). A Mountain-only card on a mountain-sparse board is near-dead.

## Spirits by preferred terrain

Not every spirit cares, but many do:

| Spirit                            | Preferred terrain                    | Why                                     |
|-----------------------------------|--------------------------------------|-----------------------------------------|
| A Spread of Rampant Green         | Jungle + Wetland                     | Plant-heavy innates                     |
| Ocean's Hungry Grasp              | Coastal (all types)                  | Drowning mechanic                        |
| Heart of the Wildfire             | Any terrain — scales with blight     | Destruction-agnostic                     |
| Keeper of the Forbidden Wilds     | Jungle + Mountain                    | Sacred-site scaling                      |
| Lure of the Deep Wilderness       | Jungle (pulls toward wilds)          | Thematic + mechanical                    |
| Sharp Fangs Behind the Leaves     | Jungle + Mountain                    | Beast deployment                         |
| Stone's Unyielding Defiance       | Mountain (sacred-site density)       | Defend via mountain anchors              |
| Volcano Looming High              | Mountain                             | Innate eruption mechanic                 |
| River Surges in Sunlight          | Wetland + jungle                     | Water-flow theme                         |
| Thunderspeaker                    | Any (dahan-dependent, not terrain)   | Dahan lives anywhere with dahan token   |

## Terrain and innates

Some innates reference terrain directly: "If target land is Jungle, deal +2 damage." This shifts element + terrain calculus together. Before drafting a Jungle-bonus innate spirit, verify the board has 4+ Jungle lands.

## Rivers

Rivers are printed modifiers on certain lands. They:

- Don't count as water for element purposes (Water-element is on cards; rivers are geography).
- Alter adjacency for some spirits (a river-edged land has more neighbors or specific neighbors).
- Interact with specific cards (e.g., "push across a river" effects).

On boards with rivers (most Jagged Earth boards), factor river geometry into push/gather planning.

## Terrain and blight

Blight affects the land regardless of terrain, but **terrain-specific cards that reduce blight** (Green's Fresh Paint the Island, some Keeper cards) require matching terrain. If your blight cascade risk is in a Mountain and your blight-removal cards target Jungle, you have no solution — read the threat ahead.

## Multi-spirit terrain splits

In 2+ player games, a common coordination pattern is terrain splitting:

> "You handle the Jungle/Wetland side; I'll take Mountain/Sand side."

This is often wrong:

- The invader deck *moves* pressure between terrains turn by turn.
- A rigid split leaves one spirit under-employed most turns.

Better pattern: "You focus on the inland hub; I'll focus on the coastal lands," regardless of terrain. Terrain is a scan-before-turn variable, not a partnership boundary.

## Reading ahead

Top 3 terrain reads before Turn 1:

1. **Which terrain has the most lands?** — that's where builds compound fastest.
2. **Is there a "lonely terrain" (1–2 lands)?** — Ravage or Build there can be a trap; invaders concentrate in scarce terrain.
3. **Coastal balance** — what proportion of lands are coastal? >50% = Ocean-strong board; <30% = inland-only strategies favored.

## Common mistakes

```admonish failure title="Common Mistake"
Drafting a terrain-restricted Minor without checking your presence. "Target: Sand or Wetland" — but your presence is entirely Jungle/Mountain. The card is stranded for 2–3 turns until you shift presence.
```

```admonish failure title="Common Mistake"
Ignoring terrain diversity when picking a spirit. A new-player Green on a low-jungle board is a frustrating game. Before choosing the spirit, glance at the board.
```

```admonish failure title="Common Mistake"
Forgetting that Explore moves through terrain chains. An isolated inland Town won't explore forward unless an adjacent land has an invader; plan reserves accordingly.
```

## Cross-references

- [Adjacency & Range](./adjacency-and-range.md) — how terrain + adjacency interact.
- [Presence Economy](./presence-economy.md) — terrain-aware presence placement.
- [Blight Track](./blight-track.md) — terrain-specific blight removal.

---

*Last revised: 2026-04-19*
