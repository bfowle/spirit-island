# Wounded Waters Bleeding

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text parsed via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings, physical-copy opening verification.

Strategic framing paraphrased from [BGG thread 3141249](https://boardgamegeek.com/thread/3141249) + community discussion. No Rei guide, no latentoctopus, no Phantaskippy coverage.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Nature Incarnate                                   |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "one" — pick one growth per turn (forced T1–T2)    |
| Power summary (1–5)   | **Offense 4** · **Control 5** · Fear 2 · Defense 1 · Utility 1 |
| Primary Elements      | **Water** (Swirl and Spill) · **Animal** (Sanguinary Taint) · Fire + Plant for L3s |
| Special Rules         | Seeking a Path Towards Healing (bleed Presence/Forget until Healing Cards claimed) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Not listed                                         |
| BGG                   | [thread 3141249](https://boardgamegeek.com/thread/3141249) |
```

## Spirit Overview — Framing

Wounded Waters is a **split-identity bleeding spirit** — every turn before its first Heal (T3 earliest) it loses Presence or Forgets a card. It chooses between Water (control/push) or Animal (damage/beasts), then between Renew (control finish) or Taste of Ruin (fear finish). Steve496 (BGG):

> Despite the somewhat forced opening build... it still feels like a really complicated spirit, with actions on the track and triggered actions and just a lot of bookkeeping to do. I forget about mechanics and have to backtrack and fix stuff way more often than I do with the other new spirits.

**Wiki-printed playstyle note**:

> Healing-track spirit with forced opening growth. Complex triggers; very high bookkeeping load.

**Complexity signal**: Wiki says High; community consensus trends Very High.

## Starting Setup

> On your starting board, put **2 Presence in a land with Blight**, then put **2 Presence and 1 Blight (from the box)** in the highest-numbered land with a Town Setup Symbol. You start with your **4 Unique Power Cards and 4 Energy**.

4 starting Presence, 4 starting Energy. Unusually high both.

## Growth Options (growthtype: "one" — forced T1–T2)

| Growth | Effects                                                               | Best when                                              |
|--------|-----------------------------------------------------------------------|--------------------------------------------------------|
| **G1** | Reclaim + Gain 1 Power Card + +1 Energy                               | Reclaim cycle                                          |
| **G2** | Gain 1 Power Card + Add Presence (R2)                                 | Card + spread                                          |
| **G3** | Add Presence (R3) + +3 Energy + Add 1 Destroyed Presence              | Rebuild after bleed-losses                             |

T1 forced: reclaim + 1 presence + 1 energy. T2 forced: 1 presence + 2 presence. Optional Growth rows unlock *after* claiming first Healing Card (T3 earliest).

## Presence Tracks

- **Energy track**: `blank → blank → blank → blank → energy3 → energy4fireorplant → energy5any`
- **Card-play track**: `energy0card1 → wateroranimal → gatherblight → energy1card2`

**Starting income**: 4 Energy (!), 1 Card Play. Exceptionally high Energy; low CP; the `gatherblight` CP slot is a Wounded-Waters signature — pulls Blight from adjacent lands onto target for AOE Major detonation.

## Core Mechanics & Special Rules

### Special Rule: Seeking a Path Towards Healing

> After playing cards each Spirit Phase:
> - Claim a Healing Marker (Element Marker) matching whichever of Water or Animal you have more of. (You break ties.)
> - You may then Claim a Healing Card if you meet its requirements. (First Healing Card available Turn 3.)
> - Then Destroy 1 Presence or Forget a Power Card (unless a Healing Card removes the rule).

The bleeding. Every turn before first Heal (T3+) destroys Presence or Forgets a card. The Heal cards are **Roiling Waters** (Animal-path, Fear-heavy) or **Serene Waters** (Water-path, control-heavy), with second-tier evolutions into Renew or Taste of Ruin.

### Innate: Swirl and Spill

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds                              | Effect                                                                 |
|-------|-----------------------------------------|------------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water | Push up to 2 Explorer/Dahan/Blight. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Fear. Push up to 2 Town/Presence/Beasts. |
| 3     | 5 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant + 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | In one land pushed into, Downgrade all Town and all City. |

### Innate: Sanguinary Taint

- **Speed**: Slow · **Range**: 1 · **Target**: Any

| Level | Thresholds                              | Effect                                     |
|-------|-----------------------------------------|---------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Fear. 1 Damage. Push 1 Dahan. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Damage. Add 1 Beast. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-water.png" alt="Water"> Water + 5 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 1 Fear. 4 Damage. Add 1 Disease. |

## Unique Cards (all 4, Wiki-verified)

`[VERIFY exact cost/speed/text from physical copy — script parse was incomplete]`:

- **Blood Water and Bloodlust** — Animal-path damage unique.
- **Draw to the Water's Edge** — Gather effect.
- **Wrack with Pain and Grief** — Fear-damage unique.
- **Boon of Corrupted Blood** — ally-gift with blood-cost clause.

## Key Strategic Principles

1. **Commit to a path by T3 heal.** Animal (Roiling — fear) vs Water (Serene — control).
2. **The R2 Presence slot gathers Blight.** Pull Blight from target lands onto the slot for AOE Major detonation — Wounded Waters' hidden superpower.
3. **Save hybrid W+A Uniques until 3 plays online.** Hit tier-2 on both innates simultaneously.
4. **Blight-adding Majors are unusually good here** (Drought, Poisoned Land, Spill Bitterness) because you can pre-gather the Blight.
5. **Roiling > Serene for fear generation.** davypi: *"Roiling generates fear but Serene doesn't."*

## Possible Openings

### Shared starting state

- **4 Presence** (2 on Blight-land + 2 on Town-symbol-land with 1 Blight added).
- **4 Uniques** in hand.
- **4 Energy**, 1 Card Play.

### Opening A — Community-standard 🟨

**T1 · Growth**: Forced (Reclaim + 1 presence + 1E).
**T1 · Play**: Gain **Water OR Animal Minor** (aligns with eventual heal path). Place first Healing Marker on **R1**.

**T2 · Growth**: Forced (1 presence + 2 presence).
**T2 · Play**: Gain opposite-element Minor (flex for either heal path). Place Healing Marker on R2 (keeps Serene vs. Roiling open).

**T3 · Growth**: First optional row unlocks *after* Healing. Gain a **Major**.
**T3 · Play**: Steve496 — *"I've mostly been opening minor/minor/major and picking a healing card based on what direction those cards favor."* Animal/Beast Major → **Roiling**; push/downgrade Major → **Serene**. Commit heal at end of Spirit Phase.

**T4–T5**: Second Healing Card → Renew (control) or Taste of Ruin (fear).

### Opening Decision

- **Default Opening A**. Not many variants documented.

## Card Priority Ratings

### Top 5 Minor Draft Picks (Water + Animal)

| # | Card | Why |
|---|------|-----|
| 1 | **Absorb Essence** | 0-cost Water |
| 2 | **Call to Isolation** | 0-cost Water + Animal |
| 3 | **Predatory Nightmares** | 0-cost Animal |
| 4 | **Call to Bloodshed** | 0-cost Animal |
| 5 | **Call of the Deeps** | Water + Animal |

### Top 5 Major Draft Picks (Blight-adder or Water/Animal)

| # | Card | Why |
|---|------|-----|
| 1 | **Drought** | Blight-adder; pair with gather-blight |
| 2 | **Poisoned Land** | Blight-adder |
| 3 | **Spill Bitterness Into the Earth** | Blight-adder |
| 4 | **Bargain of Coursing Paths** | Aminar's pick |
| 5 | **Insatiable Hunger of the Swarm** | Animal multi-land |

## Adversary Matchup Matrix

| Adversary                  | Rating | Note                                            |
|----------------------------|--------|-------------------------------------------------|
| Habsburg Mining (High Surges) | ★★☆☆☆ | AP-heavy; drowns in double-build pre-heal     |
| Most other matchups        | ★★★☆☆  | `[VERIFY]` — coverage is thin                   |

## Game-Phase Strategy

### Early (T1–T2)
- Forced growth. Gather Minors matching planned heal.
- Bleed 2 Presence + Forget 0 cards (or reverse) — 4 starting presence absorbs the early bleed.

### Mid (T3–T6)
- First Heal T3 (Roiling or Serene).
- Second Heal T4–T5 (Renew or Taste of Ruin).
- Major integration.

### Late (T7+)
- Path-specific closes.
- Gather-blight AOE Major detonation.

## Synergy Partners (Multiplayer)

- **Blight-location-sensitive** partners: Keeper, Wildfire, Fangs, Serpent, Stone, Vengeance.

## Common Mistakes

```admonish failure title="Named mistakes"
1. **Over-committing to Serene alone** — no fear generation.
2. **Forgetting the bleed trigger** before first heal (most-cited bookkeeping slip).
3. **Failing to use the blight-gather** — the hidden superpower.
```

## Source Notes

- **Mechanics**: `data/references/wiki/wounded-waters-bleeding.json` (Wiki-parsed 2026-04-23).
- **Openings**: [BGG thread 3141249](https://boardgamegeek.com/thread/3141249).
- **Coverage**: Thinnest of the NI spirits — treat as starting point, physical-copy verify.
