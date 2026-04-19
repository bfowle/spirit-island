# Thunderspeaker

```admonish warning title="Accuracy status — partial revision 2026-04-19"
Mechanical sections corrected against [Spirit Island Wiki](../../../data/references/spirit-mechanics.md). **Previous errors**: labeled "Thunderous Spirits (Manifestation of Power and Glory)" as an innate — Manifestation is a Unique card; missed the Gather the Warriors + Lead the Furious Assault innate pair entirely; missed both special rules including the critical **Sworn to Victory** (Thunderspeaker loses Presence when Dahan die nearby).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Base                                               |
| Complexity            | Moderate                                           |
| Play Difficulty       | 1                                                  |
| Archetypes            | Dahan-Rush · Fear generation via dahan kills       |
| Primary Elements      | Sun, Fire, Air, Animal                             |
| Typical Opening       | Hybrid; dahan-cluster formation                    |
| Typical Draft Bias    | Mixed                                              |
| Rei's Guide           | Not covered by Rei                                 |
| latentoctopus         | [Thunderspeaker openings](https://latentoctopus.github.io/guide/thunderspeaker-opening1/) — 3 variants |
| Aspects               | Warrior, Tactician, Mentor `[VERIFY]`              |
```

## Spirit Overview — Framing

Thunderspeaker is the **dahan warrior spirit**. Dahan move with Thunderspeaker's presence; Thunderspeaker's innates gather and direct Dahan attacks to destroy invaders. Critical risk: **Sworn to Victory** destroys Thunderspeaker's own Presence whenever a nearby Ravage kills Dahan — the spirit's scaling compounds *or collapses* depending on Dahan preservation.

**One-line fantasy**: the voice that rouses the Dahan from fear to action.

## Core Mechanics & Special Rules

### Special Rule: Ally of the Dahan

Your Presence may move with Dahan (move 1 Presence whenever Dahan moves from one of your lands to another).

**Strategic implication**: Dahan movement carries Thunderspeaker along. This bridges range + enables creative positioning.

### Special Rule: Sworn to Victory

After a Ravage Action destroys 1 or more Dahan, for each Dahan destroyed, destroy 1 of your Presence within 1 Range.

**Strategic implication**: this is Thunderspeaker's **hidden downside**. Letting Dahan die is not just losing future retaliation — it's losing your own Presence. Dahan preservation is load-bearing.

### Innate: Gather the Warriors — Slow (can be Fast), 1 Range, Any land

- **4 Air, 1 Animal**: Gather up to 1 Dahan per Air; Push up to 1 Dahan per Sun.

### Innate: Lead the Furious Assault — Slow (can be Fast), 0 Range, Any land

- **2 Sun, 1 Fire**: Destroy 1 Town for every 2 Dahan in target land.
- **4 Sun, 3 Fire**: Destroy 1 City for every 3 Dahan in target land.

**Strategic implication**: both innates can be made Fast (pairs with Swiftness-like effects at high thresholds). Lead the Furious Assault turns dahan-density into direct Town/City destruction — a 4-Dahan land can kill 2 Towns at Level 1.

### Unique Cards

Per Wiki (full details `[VERIFY]`):

- **Manifestation of Power and Glory** — Damage scales with Dahan and Presence in target land.
- **Sudden Ambush** — Prevents Invaders from building in lands with Dahan.
- **Voice of Thunder** — Dahan movement or Fear generation.
- **Words of Warning** — Defensive support during Ravages.

## Key Strategic Principles

1. **Sworn to Victory makes Dahan preservation critical.** A Ravage killing 2 Dahan = 2 lost Presence. Defend Dahan aggressively.
2. **Dahan density is the damage engine.** Lead the Furious Assault scales with Dahan count. 4+ Dahan per key land.
3. **Sun + Fire is the core element pair.** Both innates scale with Sun; Lead the Furious Assault needs Fire.
4. **Ally of the Dahan = spatial flexibility.** Dahan movement carries Presence along.
5. **Sudden Ambush prevents builds.** Dahan-land defense via build-prevention is Thunderspeaker's Fast-phase gift.
6. **Manifestation scales with Dahan + Presence.** Highest-damage-output card Thunderspeaker has.
7. **Voice of Thunder for fear.** Dahan-scaling fear closes Terror 2.

## Opening Strategy

### Opening A — Hybrid, Minor-heavy 🟨

**Source**: [latentoctopus Opening 1](https://latentoctopus.github.io/guide/thunderspeaker-opening1/).

#### Turn 1

- **Growth**: G3 bottom.
- **Cards played**: Sudden Ambush or Manifestation of Power and Glory.
- **Presence placement**: inland Dahan-land.
- **Elements by end**: 1 Sun, 1 Fire.

#### Turn 2

- **Growth**: G2 top.
- **Cards played**: Words of Warning + Minor.
- **Elements by end**: 2 Sun, 1 Fire, 1 Air.

#### Turn 3

- **Growth**: G1 Reclaim + G3 Minor.
- **Cards played**: 3 cards.
- **Elements by end**: 2 Sun, 1 Fire, 2 Air, 1 Animal approaching.

#### Turn 4 — state audit

- **Presence**: 6–7.
- **Dahan in key lands**: 3–4.
- **Engine**: Lead the Furious Assault firing at Level 1.

See latentoctopus for Opening 2 (energy-focused) and Opening 3 (late-game scaling).

### Opening decision

<pre class="mermaid">
graph TD
  Start[Round 1 — Thunderspeaker] --> Adv{Adversary?}
  Adv -->|Default| A[Opening A - Hybrid]
  Adv -->|England L0-L3| A
  Adv -->|Russia/France| A2[Opening A + early fear-rush pivot]
  Adv -->|Other| A
</pre>

## Element & Aspect Preferences

**Preferred elements**: Sun (primary), Fire (secondary), Air + Animal (for Gather the Warriors innate).

**Aspects**: Warrior, Tactician, Mentor `[VERIFY]`.

## Card Priority Ratings

Thunderspeaker is **Mixed**.

### Uniques

| Card | Grade | Notes |
|---|---|---|
| Manifestation of Power and Glory | A+ | Damage scales with Dahan + Presence. |
| Sudden Ambush | A | Build-prevention in Dahan lands. |
| Words of Warning | A | Defensive support. |
| Voice of Thunder | A- | Dahan/fear generation. |

### Majors

Terrifying Nightmares, Voice of Command, Vigor of the Breaking Dawn — Sun/Air/Animal alignment.

## Adversary Matchup Matrix

| Adversary | L0 | L3 | L5 | L6 | Notes |
|---|---|---|---|---|---|
| England | A | A- | B+ | B | Town density = Lead the Furious Assault kills. **L5 cliff**: +1 HP buildings mean a 2-Dahan land no longer kills a Town at Level 1 (needs 4 Dahan). |
| Brandenburg-Prussia | A | A- | B+ | B | Cities = Level 2 innate kills. |
| Sweden | A- | B+ | B | B- | Fear penalties. |
| France | C+ | C | D+ | D | **Avoid** — dahan capture + Sworn to Victory compound badly. |
| Habsburg Mining | B+ | B | B- | C+ | Scaling. |
| Russia | C+ | C | C- | D | Settler mechanics destroy dahan → Sworn to Victory destroys your Presence. |
| Scotland | A | A- | B+ | B | Favorable. |
| Habsburg Livestock | A- | B+ | B | B | Decent. |

### Strategy Cliff — Dahan-hostile adversaries

```admonish info title="Strategy Cliff — France / Russia"
**What changes**: these adversaries destroy or capture Dahan systematically.

**Impact on Thunderspeaker**: every killed Dahan triggers Sworn to Victory, destroying 1 Presence. The scaling engine goes into reverse.

**Mitigation**: don't play Thunderspeaker here. If forced, prioritize Fast-phase Dahan preservation (Words of Warning, Sudden Ambush) over offensive Dahan use.
```

## Synergy Partners

```admonish tip title="Best Partners"
- **Fangs** — shared battlefield; dahan + beasts retaliate together.
- **Shadows** — fear-rush + dahan-kill-fear compounds.
- **Bringer** — Dreams of the Dahan scales with Thunderspeaker's dahan density.
- **Earth** — Earth defends + Gift of Strength repeats Thunderspeaker's Major plays.
```

```admonish warning title="Anti-Synergy"
- **Vengeance / Wildfire** — blight kills dahan; Sworn to Victory costs Presence.
- **Volcano** — destruction kills dahan unconditionally.
```

## Common Mistakes

```admonish failure title="Common Mistake — Ignoring Sworn to Victory"
Letting Dahan die is a double loss: no future retaliation + Presence destroyed. Defend Dahan aggressively.
```

```admonish failure title="Common Mistake"
Spreading presence too thin. Dahan-Rush wants 3–4 clustered lands with 3+ Dahan each.
```

```admonish failure title="Common Mistake"
Drafting Air-only Minors over Sun. Sun is primary; Air is secondary.
```

```admonish failure title="Common Mistake"
Playing Thunderspeaker against France or Russia without pivoting strategy.
```

## Tempo Profile

| Round | Energy | CP | Presence | Dahan in Key Lands | Key Play |
|---|---|---|---|---|---|
| 1 | 1E | 2 | 4 | 2–3 | Unique |
| 2 | 1E | 2 | 5 | 3–4 | Unique + Minor |
| 3 | 1–2E | 2 | 6 | 3–4 | Reclaim + 2 Minors |
| 4 | 2E | 3 | 7 | 4–5 | Lead the Furious Assault L1 |
| 5 | 2–3E | 3 | 7 | 4–5 | Major gain prep |
| 6 | 3E | 3 | 6–7 | 4–5 | Fear rush |
| 7 | 3E | 3 | 6 | 4 | Major + innate |
| 8 | 3E | 3 | 5 | 3 | Terror 3 close |

Cliff turn: **T4**. Lead the Furious Assault must fire; Dahan density 3+.

## Major vs. Minor

**Draft bias**: Mixed.

## Expansion Sensitivity

- **Base only**: fully functional.
- **+ Branch & Claw**: event cards can kill Dahan via scenarios; Thunderspeaker's Sworn to Victory amplifies the cost.
- **+ Jagged Earth / later**: deeper Major pool.

## Stat Snapshot

```admonish note title="Stat Insight"
Per mindwanderer `[VERIFY]`:
- Solo L6: ~48%.
- Best vs. England L6 (~60%).
- Worst vs. France L6 (~28%), Russia L6 (~32%).
```

## Source Notes

```admonish abstract title="Sources"
- Authoritative mechanics: [data/references/spirit-mechanics.md](../../../data/references/spirit-mechanics.md).
- [latentoctopus Thunderspeaker openings](https://latentoctopus.github.io/guide/thunderspeaker-opening1/).
- Cross-reference: [Dahan fundamentals](../../fundamentals/dahan.md), [Dahan Rush archetype](../../combos/dahan-rush.md).
```

---

*Last revised: 2026-04-19 — v0.2.1 (surgical correction; `[VERIFY]` markers pending physical-copy check)*
