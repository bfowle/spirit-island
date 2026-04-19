# Keeper of the Forbidden Wilds

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki authoritative data](../../../data/references/spirit-mechanics.md). **Previous errors**: all 4 Unique card names were wrong; innate names were hallucinated. Strategic framing preserved where possible.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Branch & Claw                                      |
| Complexity            | Moderate                                           |
| Play Difficulty       | 2                                                  |
| Archetypes            | Major Power Shopping · Defend & Outlast · Wilds-scaling |
| Primary Elements      | Sun, Plant (primary), Fire, Air                    |
| Typical Opening       | Slow-ramp; Sacred Site + Wilds formation           |
| Typical Draft Bias    | Major-heavy                                        |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | Not currently listed                               |
| Aspects               | `[VERIFY]`                                         |
```

## Spirit Overview — Framing

Keeper is the **Wilds-token defender + Major-Power shopper**. Forbidden Ground special rule pushes Dahan out of newly-created Sacred Sites — a unique dahan-interaction. Spreading Wilds adds **Wilds tokens** to lands; Wilds scale Sacrosanct Wilderness damage.

**One-line fantasy**: the forest forbids. Where Keeper's presence roots, the land itself rejects colonial intrusion — Dahan retreat to safer ground, and what crosses Keeper's thresholds is punished.

**The honest complexity signal**: Moderate. Keeper has two innates + two special rules + Wilds-token mechanic. Players learning Keeper often miss the Dahan-pushing effect of Forbidden Ground and end up with dahan clustered incorrectly.

## Core Mechanics & Special Rules

### Special Rule: Forbidden Ground

After you create a Sacred Site, **Push all Dahan from that land**. Dahan Events never move Dahan to your Sacred Site, but Powers can.

**Strategic implication**: Keeper's Sacred Sites are dahan-hostile by design. Don't build Sacred Sites in dahan-dense lands unless you're OK pushing the dahan out.

### Innate: Punish Those Who Trespass — Slow, 0 Range, Any land

- Base (2 Sun, 1 Fire, 2 Plant): 2 Damage; Destroy 1 Dahan.
- Upgrade (2 Sun, 2 Fire, 3 Plant): +1 Damage per Sun/Plant you have.
- Upgrade (4 Plant): Split this Power's Damage between target land and another 1 of your lands.

**Strategic implication**: Punish's base effect destroys 1 Dahan — another dahan-hostile element. Drafts around this have to accept Dahan-loss or target Dahan-free lands.

### Innate: Spreading Wilds — Slow, 1 Range, Land without Blight

- (2 Sun): Push 1 Explorer per 2 Sun you have.
- (1 Plant): If target has no Explorer, add 1 Wilds.
- (3 Plant): +1 Range.
- (1 Air): +1 Range.

**Strategic implication**: Wilds accumulate via Spreading Wilds in Explorer-free lands. The Wilds mechanic feeds Sacrosanct Wilderness Unique.

### Unique Cards

All four starting Uniques per Wiki:

- **Boon of Growing Power** — `[VERIFY: cost, speed, elements]`. Grants another Spirit energy and card draw.
- **Regrow from Roots** — `[VERIFY: cost, speed, elements]`. Removes blight in Jungle/Wetlands.
- **Sacrosanct Wilderness** — `[VERIFY: cost, speed, elements]`. Adds Wilds and deals damage based on Wilds present.
- **Towering Wrath** — `[VERIFY: cost, speed, elements]`. Damage scales with Sacred Sites in target area.

## Key Strategic Principles

1. **Forbidden Ground is dahan-hostile.** Sacred Sites push Dahan out. Accept this; don't try to Dahan-cluster around Keeper.
2. **Wilds are a secondary resource.** Spreading Wilds + Sacrosanct Wilderness = damage scaling via Wilds. Draft around this.
3. **Sun + Plant is the primary element pair.** Both innates want both.
4. **Boon of Growing Power is a multiplayer lever.** In solo it's limited; in multiplayer it enables a partner's high-cost Major play.
5. **Regrow from Roots = Jungle/Wetland blight removal.** Terrain-gated; check board.
6. **Towering Wrath scales with Sacred Site count.** 2–3 Sacred Sites = heavy damage.
7. **Major Power Shopping standard.** Forget the weakest Unique T3–T4 for a Sun/Plant Major.

## Opening Strategy

### Opening A — Sacred Site formation + Early Major 🟥

**Target arc**: 2 Sacred Sites by T3 · Level 1 innates firing T3 · Major T4+.

#### Turn 1

- **Growth**: **G2 top** or **G3 bottom** depending on energy need.
- **Cards played**: one Unique that fits (Boon of Growing Power for multiplayer; Regrow from Roots if board is jungle-heavy).
- **Presence placement**: form first Sacred Site in an Explorer-present land (Wilds prep requires Explorer-free; Sacred Site prep separate).
- **Elements by end**: 1 Sun, 1 Plant.
- **E / CP state**: 1E / 2CP → 0E / 2CP.

#### Turn 2

- **Growth**: **G3 bottom** (Minor gain) or **G4 Major gain** if Plant-Sun Major is offered.
- **Cards played**: Unique + drafted Minor.
- **Presence placement**: second Sacred Site forming.
- **Elements by end**: 2 Sun, 1 Plant, 1 Air.
- **E / CP state**: 1E / 2CP → 0E / 2CP.

#### Turn 3

- **Growth**: **G1 Reclaim**.
- **Cards played**: Major (if gained) + Sacrosanct Wilderness.
- **Elements by end**: 2 Sun, 1 Fire, 2 Plant (Level 1 Punish Those Who Trespass threshold).
- **E / CP state**: 2E / 2CP → 0–1E / 2CP.
- **Milestone**: Punish firing for 2 damage; Wilds accumulating.

#### Turn 4 — state audit

- **Presence**: 5–6; 2 Sacred Sites; 1–2 Wilds on board.
- **Energy / CP**: 2E / 2–3CP.
- **Engine**: Major in rotation; Punish firing; Sacrosanct Wilderness scales.

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Keeper] --> Adv{Adversary?}
  Adv -->|Default| A[Opening A - SS formation + Early Major]
  Adv -->|Russia L5+| B[Defend-heavy variant]
  Adv -->|Multiplayer partner with high-cost Majors| A
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Sun, Plant (primary), Fire, Air.

**Aspects**: `[VERIFY — Wiki didn't surface aspect names]`.

## Card Priority Ratings

Keeper is **Major-heavy**.

### Uniques

| Card                    | Grade | Notes                                          |
|-------------------------|-------|------------------------------------------------|
| Towering Wrath          | A+    | Sacred-Site-scaling damage.                    |
| Sacrosanct Wilderness   | A     | Wilds-scaling damage; play when Wilds are on board.|
| Regrow from Roots       | A-    | Blight removal (Jungle/Wetland only).          |
| Boon of Growing Power   | A (multiplayer) / B (solo) | Partner gift.                   |

### Minors & Majors

Sun-Plant duals preferred. Vigor of the Breaking Dawn, Tigers Hunting, Rouse the Trees and Stones are canonical Major targets.

## Adversary Matchup Matrix

| Adversary            | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England              | A+ | A  | A- | B+ | Slow-build matches; **L5 cliff**: buildings +1 HP means Punish's base 2 damage no longer kills Towns alone. |
| Brandenburg-Prussia  | A- | B+ | B  | B- | Fast cities outpace.                   |
| Sweden               | A  | A- | B+ | B  | Defend + Majors handle.                |
| France               | A  | A- | B+ | B  | Forbidden Ground's dahan-push actually helps vs. plantation. |
| Habsburg Mining      | A+ | A  | A- | B+ | Best matchup.                          |
| Russia               | A  | A- | B+ | B  | Defend absorbs.                        |
| Scotland             | A  | A- | B+ | B  | Decent.                                |
| Habsburg Livestock   | A  | A- | B+ | B  | Fine.                                  |

## Synergy Partners

```admonish tip title="Best Partners"
- **Lightning** — Lightning kills early; Keeper's Majors close late.
- **Shadows** — Shadows fear-rushes; Keeper absorbs board.
- **Bringer** — Boon of Growing Power on Bringer's Call-on-Midnight's-Dream Major turn = double Major.
- **Fangs** — Fangs beast damage + Keeper's defensive wall.
```

```admonish warning title="Anti-Synergy"
- **Thunderspeaker** — Keeper's Forbidden Ground pushes Dahan out of Sacred Sites; Thunderspeaker wants Dahan concentrated. Coordinate Sacred Site placement carefully.
- **Earth** — double Major-shoppers compete for offers.
```

## Common Mistakes

```admonish failure title="Common Mistake"
Forming Sacred Sites in dahan-dense lands without realizing Forbidden Ground pushes them out. Scout the dahan before committing presence-density.
```

```admonish failure title="Common Mistake"
Drafting Majors without Sun-Plant threshold.
```

```admonish failure title="Common Mistake"
Over-concentrating presence in 1 land. 2–3 Sacred Sites spread > 1 mega-dense land.
```

## Tempo Profile

| Round | Energy | CP | Presence | Sacred Sites | Wilds | Majors | Key Play |
|---|---|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 0 | 0 | 0 | Unique + Minor |
| 2 | 2E | 2 | 5 | 1 | 0 | 1 in hand | G4 Major gain |
| 3 | 3E | 2 | 6 | 1–2 | 1 | 1 | Major fires; Wilds forming |
| 4 | 2–3E | 3 | 6–7 | 2 | 1–2 | 1 | Level 2 innate |
| 5 | 3E | 3 | 7 | 2 | 2 | 1–2 | 2nd Major gain |
| 6 | 3–4E | 3 | 7 | 2 | 2–3 | 2 | 2 Majors |
| 7 | 4E | 3 | 6 | 2 | 3 | 2–3 | Major chain |
| 8 | 4E | 3 | 6 | 2 | 3 | 2–3 | Close |

## Expansion Sensitivity

- **Base only**: Keeper is a Branch & Claw spirit — not available in base-only.
- **+ Branch & Claw (minimum)**: full functionality.
- **+ Jagged Earth**: deeper Major pool.
- **+ Nature Incarnate**: no core shift.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~58%.
- Best vs. Habsburg Mining L6 (~70%).
- Worst vs. Brandenburg-Prussia L6 (~42%).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- Spirit Island Wiki — Keeper page.
- Cross-reference: [Major Power Shopping](../../combos/major-power-shopping.md), [Defend & Outlast](../../combos/defend-and-outlast.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
