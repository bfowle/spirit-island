# Downpour Drenches the World

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics (if any), live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Downpour concepts](https://latentoctopus.github.io/guide/downpour-concepts/) + [jyonker13's BGG openings thread 2496834](https://boardgamegeek.com/thread/2496834/openings-dowpour-drenches-world).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Promotional Pack 2 (Feather and Flame)             |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "one" — pick **one** growth option per turn (each bundles 2–4 effects) |
| Power summary (1–5)   | Offense 2 · Control 3 · Fear 1 · **Defense 5** · Utility 3 |
| Primary Elements      | **Water** (all tiers, Special-Rule fuel) · Plant · Air · Earth |
| Special Rules         | Drench the Landscape (Sacred Site also Wetlands) + Pour Down Power Across the Island (2 Water → +1 Energy OR Repeat a land-targeting Power Card) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Concepts](https://latentoctopus.github.io/guide/downpour-concepts/) + [Opening 1](https://latentoctopus.github.io/guide/downpour-opening1/) |
```

## Spirit Overview — Framing

Downpour is the game's **team-amplifier** — a spirit whose kit multiplies everyone else's plays rather than stacking its own damage. The Pour Down Power Across the Island rule converts Water into the two rarest currencies: **bonus Energy** or **a second cast of a land-targeting Power Card**. That choice — *repeat a Power or bank Energy* — is the most-repeated decision across a Downpour game.

**Wiki-printed playstyle note** (verbatim):

> Cares about the question "How useful is this Power in the current context?" even more than most Spirits; it rarely plays all its Power Cards in any given Reclaim cycle (some get discarded to Growth), and for those it does play, it often has the option of using them multiple times.

**Identity capture** (jyonker13, BGG):

> It's a Spirit of excess, and of understanding relative value, so on some level your success is determined by how well you shore up the gaps in your team's abilities.

**Complexity signal**: High is accurate. Three layered decisions per turn — which card to play, which to repeat, whether to Energy-bank — on top of element counting, Wetlands-spread topology, and team coordination. Not a beginner spirit.

## Starting Setup

> Put **1 Presence** on your starting board in the **lowest-numbered Wetlands**.

Only 1 starting presence — Downpour builds density slowly. Every early Growth adds board coverage, which in turn powers Rain and Mud Suppress Conflict and widens the Wetlands footprint (via Drench the Landscape at sacred sites).

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                                                    | Best when                                                        |
|--------|----------------------------------------------------------------------------|------------------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + Move 1 Presence (Range 2)                    | Hand depleted; repositioning presence to ally's board            |
| **G2** | +Presence (R2) + +Presence (R2) + 2 Water markers *(cost: discard 2 cards)* | Need board spread + Water-engine fuel                            |
| **G3** | Gain 1 Power Card + Add Presence (Range 3) + +1 Energy                     | Default opening + midgame; balanced card + placement + energy    |

**G2's discard cost matters** — 2 cards from hand pay for the +2 presence + 2 Water. Early-game G2 is a T2–T3 move when Reclaim is close, not T1.

## Presence Tracks

- **Energy track** (8 slots): `energy1 → water → plant → water → energy2air → water → earth → doublewater`
  - 1E → +Water marker → +Plant → +Water → 2E + Air → +Water → +Earth → 2× Water
- **Card-play track** (6 slots): `card1 → movepres → waterX → card2 → movepres → card3`
  - 1 CP → Move Presence → Water scaling → 2 CP → Move Presence → 3 CP

**Starting income**: 1 Energy, 1 Card Play. The energy track is unusual — most slot-reveals are *markers*, not energy. Water markers stack as a persistent second currency feeding Pour Down every turn.

## Core Mechanics & Special Rules

### Special Rule: Drench the Landscape

> Spirit Actions and Special Rules treat your Sacred Site as Wetlands in addition to the printed terrain.

A sacred site (2+ presence) is *also* a Wetland for your Unique-targeting. Expands Dark Skies (R1 from Wetland) reach and triggers Wetland-conditional clauses on Unbearable Deluge and Foundations Sink into Mud.

### Special Rule: Pour Down Power Across the Island

> For each 2 Water you have, during the Fast/Slow phase you may either:
> - Gain 1 Energy; or
> - Repeat a land-targeting Power Card by paying its cost again. (It need not target the same land.)
> (Max 5 times per turn, no matter how much Water you have.)

The engine. 10 Water = 5 uses, mixing Energy-gain and Power-repeat freely. The 5-use cap bounds the late-game ceiling.

```admonish tip title="Energy-bank vs. Repeat — the signature decision"
jyonker13: *"Accumulating Energy can be absurdly good on this Spirit."* Converting 10 Water into 5 Energy instead of 5 Repeats gives you a 5E spike for next turn's Major — a legitimate line, not a default loss. Per-turn: *does this turn have a better Power to Repeat, or does next turn need the Energy?*
```

### Innate: Rain and Mud Suppress Conflict

- **Speed**: Fast · **Target**: You (passive)

| Level | Thresholds                  | Effect                                                                 |
|-------|-----------------------------|------------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1. *(Total, in its land.)* |
| 2     | 5 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | *Instead*, each of your Presence grants Defend 1 and lowers Dahan counterattack damage by 1 *(per presence — the cap is removed)*. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 9 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | 2 Fear. In your lands, Invaders and Dahan have −1 Health (min 1). |

Rain and Mud scales *per Presence*. A 3-presence land under L2 is Defend 3 + counter-reduction 3. The Dahan-counter-reduction half also hits allies' Dahan output, which is why sloppy placement can gimp partner damage.

### Innate: Water Nourishes Life's Growth

- **Speed**: Fast · **Range**: 0 · **Target**: Any

| Level | Thresholds                  | Effect                                                                 |
|-------|-----------------------------|------------------------------------------------------------------------|
| 1     | 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gain 1 Energy. You may remove 1 Blight by removing one of your Presence (from target land). |
| 2     | 5 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Gain +1 Energy. Gather up to 1 Dahan. |
| 3     | 7 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | When Blight would be added to target land, instead leave it on the card. |

Blight-cancel at L3 is Downpour's late-game ceiling alongside Tsunami.

## Unique Cards (all 4, Wiki-verified)

### Dark Skies Loose a Stinging Rain
- **1 Energy · Fast · Range 1 from a Wetland · Any Land · Moon, Air, Water**
- *Isolate target land. Push up to 1 Explorer and up to 2 Dahan.*

Ranged Isolate — sacred sites double as Wetlands for extended reach. Push-into-ally-lands enables counterattacks.

### Gift of Abundance
- **1 Energy · Fast · No Range · Another Spirit · Sun, Air, Water, Plant**
- *Target Spirit either gains 2 Energy, or may Repeat one Power Card this turn by paying its cost. Either you or target Spirit may add 1 Destroyed Presence to a Wetland where you have Presence.*

Downpour's signature ally-gift and the opening's default play. Scales with partner capability; recovers destroyed Presence. latentoctopus recommends this as the T1 default.

### Unbearable Deluge
- **0 Energy · Fast · Range 0 · Any Land · Air, Water, Earth**
- *1 Fear. Push 2 Dahan. Defend 3. If target land is a Wetland, Isolate it.*

0-cost Fast Defend + Isolate in Wetlands (including sacred-site Wetlands via Drench the Landscape). The universal Defend option when Rain-and-Mud isn't online yet.

### Foundations Sink into Mud
- **1 Energy · Slow · Range 0 · Any Land · Water, Earth**
- *2 Damage to Town. If target land is a Wetland, you may instead deal 1 Damage to each Town/City.*

Wetland-conditional AOE: in a Wetland with multiple Towns/Cities, this is a board-clearer at 1E. Pour Down repeat converts it into a 2-land AOE turn.

## Key Strategic Principles

1. **Water is the engine.** Every Water source — track, marker, card, innate — fuels Pour Down. Draft Water-producing Minors aggressively.
2. **Repeat vs. Energy is a per-turn decision.** Don't auto-Repeat. Bank Energy when this turn lacks a high-value Repeat target.
3. **Wetland-footprint = reach.** Every second presence on a land = sacred site = Wetland. Density doubles your Wetland-triggered effects.
4. **Majors are a T8+ concern.** latentoctopus: *"A Major can be gained earlier on (turn 5 or 6), but usually the Energy is better spent repeating Uniques or Minors."*
5. **Rain and Mud is double-edged.** Counterattack-reduction hits Dahan counters too — coordinate placement with partners before T3.
6. **Gift of Abundance is a twice-per-turn card.** Pour Down's Repeat option lets it fire twice most turns — effectively 4 Energy per cycle to an ally.

```admonish failure title="The canonical Downpour mistake"
jyonker13: wasting all 4–5 Pour Down uses on Repeats when 2–3 should have been Energy for next turn's Major. Downpour is the only spirit where *"do less this turn"* is often mathematically correct.
```

## Possible Openings

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

### Shared starting state

- **1 Presence on board**: lowest-numbered Wetland.
- **4 Uniques in hand**: Dark Skies Loose a Stinging Rain (1E Fast, Moon/Air/Water), Gift of Abundance (1E Fast, Sun/Air/Water/Plant), Unbearable Deluge (0E Fast, Air/Water/Earth), Foundations Sink into Mud (1E Slow, Water/Earth).
- **Starting income**: 1 Energy, 1 Card Play.
- Growth type **"one"**: pick one of G1/G2/G3 per turn.

### Opening A — Bottom-track Hybrid (Minors) 🟨 (default)

Adapted from [latentoctopus Opening 1](https://latentoctopus.github.io/guide/downpour-opening1/).

**T1 · Growth**: G3 — Gain 1 Power Card (draft **Minor**, Water/Plant priority) + Add Presence (R3) + +1 Energy.
- Income: 1E (track) + 1E (G3) = 2E this turn.

**T1 · Play** (2E, 1 CP): **Gift of Abundance** targeting an ally (2 Energy gift or Repeat enable).
- *Pause-point*: solo or no suitable ally → swap to Unbearable Deluge (0E Defend) + bank 1E.

**T2 · Growth**: G2 — +2 Presence + 2 Water markers (cost: discard 2 cards).
- *Pause-point*: discard the 2 cards you need *least* this cycle — drafted weak Minors, or Foundations Sink when Wetland-trigger isn't reachable.

**T2 · Play** (1E, 2 CP): Unbearable Deluge + drafted Minor.
- Pour Down: bank 1 Energy → enter T3 with 2E cushion.

**T3 · Growth**: G1 — Reclaim + Gain 1 Power Card + Move 1 Presence (R2).
- Reclaims hand. +1 card. Income: 1E (track) + 2E banked = 3E.

**T3 · Play** (3E, 2 CP): Gift of Abundance (+Repeat via Pour Down → double-gift to ally) + Dark Skies Loose a Stinging Rain on a Ravage land.
- *Pause-point*: **First Ravage.** Rain-and-Mud L1 (1 Air + 3 Water) reachable? If not, Unbearable Deluge's Defend 3 covers one land instead.

**T4 · Growth**: G2 or G3 — depending on card-hand state.

**T4 end state** (audit):
- 5–6 presence on board.
- 2 CP baseline.
- Rain and Mud L1 reliably online.
- Water-marker bank sufficient for 2–3 Pour Down uses per turn.
- 2–3 Minors drafted (Water-prime). No Major yet.

**Confidence**: 🟨 latentoctopus's primary opening. Plays well in 2P+; solo is slower (Gift's ally-benefit wastes on self).

### Opening B — 3-CP Bottom Track 🟥 (variant)

latentoctopus explicitly flags this as *generally not worth it*: lower Energy income, later Major, and 2 CP is usually sufficient because Pour Down repeats plays.

- **Diverges at T6**: G2 bottom to 3 CP. T7 Reclaim + Gain. T8 G2 bottom + play 3.

Use only when partner's kit specifically eats extra Downpour CP (rare).

### Opening Decision

- **Default to Opening A** across all base adversaries.
- **Solo**: Opening A still works; substitute self-target where Gift's ally-benefit would be wasted.
- **3-CP variant**: only when partner coordination demands it.

## Card Priority Ratings

### Uniques (Downpour-specific ranking)

1. **Gift of Abundance** — ally multiplier, played every turn.
2. **Unbearable Deluge** — 0-cost Fast Defend; Wetland Isolate.
3. **Dark Skies Loose a Stinging Rain** — ranged Isolate + Push.
4. **Foundations Sink into Mud** — Wetland AOE; situational but powerful.

### Top 10 Minor Draft Picks (Water-prime)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Water-dense + Fear |
| 2 | **Drifting Into Stillness** | 1 | Slow | Moon, Plant | Plant-feeder |
| 3 | **Absorb Essence** | 0 | Fast | Water | Pure Water |
| 4 | **Purify the Land** | 0 | Slow | Moon, Water, Plant | Blight-removal + elements |
| 5 | **Ravaged Undergrowth Slithers Back** | 0 | Slow | Water, Earth, Plant | Triple-prime elements |
| 6 | **Call to Isolation** | 0 | Slow | Water, Animal | Isolate utility |
| 7 | **Travel Unsuspected** | 1 | Fast | Air, Water | Air/Water for Rain-and-Mud |
| 8 | **Gift of Proliferation** | 1 | Fast | Plant, Water | Plant+Water for Water Nourishes |
| 9 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Water-rich expensive |
| 10 | **Call of the Deeps** | 1 | Slow | Water, Animal | Single-Water utility |

### Top 5 Major Draft Picks

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Late-game closer; Water-rich |
| 2 | **Manifest Incarnation** | 3 | Fast | Moon, Plant, Animal, Water | Multi-element utility |
| 3 | **Flow Like Water, Reach Like Air** | 2 | Fast | Sun, Air, Water | Water-feeder |
| 4 | **Dissolve Into Mist** | 4 | Fast | Air, Water | Water + utility |
| 5 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Off-Water but high-impact board control |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Single-land non-Water Majors | Pour Down wants multi-land repeat targets |
| Explorer-destruction heavy kit | Downpour prefers Explorers *moved* not destroyed |

## Adversary Matchup Matrix

| Adversary               | Rating | Matchup note                                                          |
|-------------------------|--------|-----------------------------------------------------------------------|
| England                 | ★★★★☆  | Rain-and-Mud scales vs. coastal City ravages; Water Nourishes de-Blights |
| Brandenburg-Prussia     | ★★★☆☆  | Build pressure punishes slow setup; need T2 Wetland Deluge online      |
| Sweden                  | ★★★☆☆  | Fear-light kit fights Downpour tempo                                   |
| Russia                  | ★★☆☆☆  | **Bad matchup per latentoctopus** — lacks Explorer-removal; minors must cover |
| France-Plantation       | ★★★☆☆  | `[VERIFY]`                                                             |
| Habsburg Mining         | ★★★☆☆  | `[VERIFY]`                                                             |
| Scotland                | ★★★☆☆  | `[VERIFY]`                                                             |
| Habsburg Livestock      | ★★☆☆☆  | `[VERIFY]`                                                             |

## Board / Map Configuration

`[VERIFY — latentoctopus does not provide per-board ratings]`. Heuristic: favor boards with **central Wetlands + adjacent low-number lands**. Board E's Wetland-heavy topology pairs naturally with Drench the Landscape.

## Game-Phase Strategy

### Early (T1–3)
- G3 → G2 → G1 cadence. Gift of Abundance every turn.
- Rain-and-Mud L1 online by T3.

### Mid (T4–6)
- 2 CP reliable. Pour Down 2–3 uses per turn.
- Foundations Sink on multi-Town Wetlands is the mid-game spike.

### Late (T7+)
- Major drafted (Tsunami, Manifest Incarnation).
- L2 innates reliable; Pour Down at 5-use ceiling.

## Synergy Partners (Multiplayer)

- **Damage-dealing partners** — Gift's Repeat turns their Majors into double-plays.
- **Stacked-Town partners** — Foundations Sink benefits from banked Towns.
- **Card-hungry partners** — Gift's 2E gift fuels Major-drafters.
- **Explorer-removal partners** — covers Downpour's Russia weakness.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **All Pour Down on Repeats.** Energy-banking is legitimate.
2. **Rain-and-Mud gimping ally Dahan.** Coordinate placement before T3.
3. **Early Majors.** Downpour's Energy is better spent on Repeats through T6.
4. **Ignoring Wetland topology.** Without density, Unbearable Deluge's Isolate never fires.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G3 + Gift of Abundance + 1 Minor drafted                  |
| 2    | G2 + Unbearable Deluge + 1 Minor played                   |
| 3    | G1 Reclaim + Gift Repeat + Dark Skies; Rain-and-Mud L1   |
| 4–5  | G2/G3 alternating; 2–3 Pour Down uses/turn                |
| 6–7  | 2 CP baseline; Water Nourishes L2                         |
| 8+   | Major drafted; L2 innates reliable                        |

## Source Notes

- **Mechanics**: `data/references/wiki/downpour-drenches-the-world.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus concepts + Opening 1 pages.
- **BGG**: [jyonker13 thread 2496834](https://boardgamegeek.com/thread/2496834/openings-dowpour-drenches-world).
