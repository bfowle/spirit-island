# Shifting Memory of Ages

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, suggested-draft cards, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, board ratings.

Strategic framing informed by [GodsLittleCow's BGG Strategy Guide (thread 2915319)](https://boardgamegeek.com/thread/2915319/strategy-guide-shifting-memory-of-ages) and [jyonker13's Openings thread (BGG 2506389)](https://boardgamegeek.com/thread/2506389/openings-shifting-memory-ages). Paraphrased in our own voice; direct quotes where noted.
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Moderate                                           |
| Play Difficulty       | 3 `[VERIFY physical spirit panel]`                 |
| Growth type           | "one" — pick **one** growth option per turn        |
| Power summary (1–5)   | Offense 1 · Control 2 · Fear 2 · **Defense 4** · **Utility 5** |
| Primary Elements      | **Moon** (Observe L1/L2) · **Earth** (Learn all tiers) · **Air** (Learn L2/L3, Observe L2) |
| Special Rules         | Long Ages of Knowledge and Forgetfulness (discard-instead-of-Forget) + Insights into the World's Nature (Prepare Element Markers) |
| Aspects (NI)          | Intensify · Mentor `[VERIFY mechanics — aspect JSON not yet scraped]` |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | Opening-linked via [BGG 2506389 — jyonker13](https://boardgamegeek.com/thread/2506389) |
| BGG Strategy Guide    | [GodsLittleCow, thread 2915319](https://boardgamegeek.com/thread/2915319) |
```

## Spirit Overview — Framing

Shifting Memory is the **Jagged Earth Major-Power engine** — an energy-rich, card-play-poor spirit whose identity is banking Moon-fed element markers to threshold Majors on demand. The design space: **every Major you draft must be played at threshold**, because Shifting Memory's 2-CP ceiling means you can't afford single-land, below-threshold Majors that the card economy accepts from other spirits.

**Wiki-printed playstyle note** (verbatim):

> Starts with little ability to influence the board — most of what it does in that regard will come from new Power Cards. Extremely good with Major Powers and usually wants to take them early and often. Can either try sprinting towards victory with its phenomenal Energy Growth or build up towards becoming a late-game powerhouse.

**The honest complexity signal**: Moderate is accurate but understates the *planning* load. Shifting Memory has few per-turn inputs (2 CP) but each input is a big lever — a Major drafted-to-gifted, a marker banked-for-later, a Reclaim-timing call. Most mistakes are not mechanical; they're resource-sequencing ones that compound over 3–4 turns.

**Identity, restated from GodsLittleCow's BGG guide**:

> Memory is a moon spirit. Moons are what fuels your furnace of elemental markers, because you want to be hitting the 2nd tier of Observe every turn.

Moon is primary; Earth is secondary (for Defend innate); Air is tertiary (threshold boost on both innates, but you can't cheaply scale into it early).

## Starting Setup

> Put **2 Presence** on your starting board in the **highest-numbered land that is Sands or Mountain**. Prepare **1 Moon, 1 Air, and 1 Earth marker** (put them by your Special Rules).

Three markers at setup is the opening bankroll. Do **not** spend all three T1; at least one (usually Moon) should survive to T2–T3 to keep Observe L2 available when your Major lands.

## Growth Options (growthtype: "one" — pick one per turn)

| Growth | Effects                                 | Best when                                                               |
|--------|-----------------------------------------|-------------------------------------------------------------------------|
| **G1** | Reclaim + Add 1 Presence (Range 0)      | Hand depleted (Majors forgotten) + want sacred-site build               |
| **G2** | Gain 1 Power Card + Add 1 Presence (Range 2) | Drafting a Major; opening-track acceleration (top track)           |
| **G3** | Add 1 Presence (Range 1) + +2 Energy    | Energy-bank mid-game; opening-track acceleration (bottom track)        |
| **G4** | **+9 Energy**                           | One-shot fuel for an expensive Major-play turn. Used late or emergency. |

**G4 is a trap if used as a default.** GodsLittleCow's guide explicitly avoids G4 in the "Fast Major" opening; it's for emergency play of a 6+ cost Major or late-game Major-stacking turns. Base rule of thumb: **if you're on G4 before T4, something has gone wrong upstream.**

## Presence Tracks

As Presence leaves each track, these values are revealed (cumulative per-turn gain):

- **Energy track** (8 slots): `energy0 → energy1 → energy2 → energy3marker → energy4 → reclaim1E → energy5 → energy6marker`
  - Translated: **0 → 1 → 2 → 3E + Prepare 1 Marker → 4 → 5E with Reclaim → 5 → 6E + Prepare 1 Marker**
- **Card-play track** (5 slots): `card1 → card2 → card2 → markersforplay → card3`
  - Translated: **1 → 2 → 2 → 2 + spend markers as plays → 3**

**Starting income**: 0 Energy, 1 Card Play. The slot-0 on energy track is `energy0` — Shifting Memory has *no* native income from turn 1; you live off the 3 setup markers + whatever Major you hold.

**The `markersforplay` slot** on the CP track is a unique conversion: once revealed, you may spend Element Markers *as card plays*. This is how the late-game 4-Major turns happen — you bank 4 markers and spend them as CP on the same turn you G4 for +9 Energy.

## Core Mechanics & Special Rules

### Special Rule: Long Ages of Knowledge and Forgetfulness

> When you would Forget a Power Card from your hand, you may instead discard it. (Max. once per Action.)

**Strategic implication**: Blighted-Island-triggered Forget, Event-card Forget, or any Forget from Major-gain cost becomes a **Discard** instead. The card cycles back via Reclaim. Shifting Memory's Majors are essentially never-Forgotten — you draft → play → discard → Reclaim → play again. This is the mechanical reason Major-drafting is cheap for Memory: the Forget penalty that prices Majors for other spirits is a discard-cost-adjustment for you.

One hard limit: **max once per Action**. If two effects would Forget simultaneously (rare, usually Event-card chain), only one is redirected to discard.

### Special Rule: Insights into the World's Nature

> Some of your Actions let you Prepare Element Markers, which are kept here until used. Choose the Elements freely. (I.e., you are not limited to Elements you have at the time.) Each Element Marker spent grants 1 of that Element for a single Action. (E.g., one Power use.)

**Economy type**: discrete single-use element tokens. Not "have an element all turn" — **per-Action injection**. If you spend Moon marker on Power X, only Power X sees that extra Moon; a later same-turn Power doesn't.

- Markers never expire. They persist across turns until spent.
- **Setup grants 3 markers** (1 Moon, 1 Air, 1 Earth). No Turn-1 Prepare action is required to reach this baseline.
- `simplemoon` / `simpleair` / `simpleearth` on the Wiki template refer to the setup-marker templates; functionally they're regular markers.

```admonish tip title="Marker-hoarding priority (per GodsLittleCow)"
Priorities for which marker to Prepare, in order of draft/target:
1. **Always hold ≥1 Moon** — it's the fuel for Observe L2 every turn
2. Elements needed to threshold the **Major you plan to play this turn**
3. **Earth for Learn L1/L2 Defend** — you're hitting Defend "most every turn as your 3rd action"
4. Second Moon / second Earth as insurance

Spend pattern: **pay 1 marker into Observe to convert a 2-action land into a threshold hit — gamble on lands with Beasts/Dahan/Disease/likely Explores**. Occasionally pay 2 Moons if 3 actions are already guaranteed in the target land.
```

### Innate: Learn the Invaders' Tactics

- **Speed**: Fast · **Range**: 1 · **Target**: Invaders

| Level | Thresholds                                     | Effect                                                             |
|-------|------------------------------------------------|--------------------------------------------------------------------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | Defend 2. |
| 2     | 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 2 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | *Instead*, Defend 3. |
| 3     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 4 <img class="si" src="/spirit-island/theme/icons/element-earth.png" alt="Earth"> Earth | *Instead*, **Defend 2 per card in the Invader discard pile**. |

L1 is the workhorse — 2 Earth is a single marker spend (or 1 drafted earth minor). L2 adds Air for +1 Defend; trivial when you've drafted an Air Minor. L3 scales ferociously mid-late game: at T5 with 3–4 Invader discards, it's Defend 6–8 per cast. It's how Shifting Memory shuts down England coastal-City ravages.

### Innate: Observe the Ever-Changing World

- **Speed**: Fast · **Range**: 1 · **Target**: Any

| Level | Thresholds                                    | Effect                                                             |
|-------|-----------------------------------------------|--------------------------------------------------------------------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | Prepare 1 Element Marker. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | *Instead*, after each of the next three Actions that change which pieces are in target land, Prepare 1 Element Marker. (Any Action can trigger — not just your own.) |

Observe is Shifting Memory's **element-generation engine**. L1 always fires (1 Moon marker = 1 free Moon → Prepare 1 marker = net neutral, +1 insurance). L2 is the huge lever: *three* marker spawns from one innate cast, if the target land sees three piece-changes (Build + Ravage + teammate push → 3 markers).

**Target-land selection for L2**: lands where you expect multiple actions. Ravage+Build+Explore is the natural 3-hit target (if invaders present). Teammate-heavy board-shaping in a land also triggers. Dormant lands (no invaders, no dahan adjacency) are zero-yield — don't waste L2 on them.

## Unique Cards (all 4, Wiki-verified)

### Boon of Ancient Memories

- **1 Energy · Slow · No Range · Any Spirit · Moon, Water, Earth, Plant**
- *If you target yourself, gain a Minor Power. Otherwise: Target Spirit gains a Power Card. If it's a Major Power, they may pay 2 Energy instead of Forgetting a Power Card.*

**Two modes**: self → Minor gain (essentially G2's gain portion as a card play). Other-spirit → gift a card gain + Forget-avoidance clause. The Forget-avoidance rarely triggers in practice (most partners value the card-gain more than the anti-Forget) but becomes a huge lever for partners like Immense-aspect Lightning who can delay their Reclaim one turn. Most common forget-fodder for Memory's Major-draft openings — it's the weakest of the four Uniques but carries Water/Plant elements useful as a threshold-filler when drafted back via Reclaim.

### Study the Invaders' Fears

- **0 Energy · Fast · Range 0 · Land with 1+ Town/City · Moon, Air, Animal**
- *2 Fear. Turn the top card of the Fear Deck face-up.*

0-cost Fast that reveals the **next Fear-card effect to the table**. Its information-value scales with player count: 2P is mild planning, 4P+ can re-plan fully — GodsLittleCow's comment that "pre-seeing the T1 fear card often equates to an extra action per player/board" lands hardest in 4P+ games. As a solo spirit, Study is good (2 Fear, threshold-relevant Moon/Air) but not load-bearing.

### Share Secrets of Survival

- **0 Energy · Fast · Range 1 from Sacred Site · Any Land · Sun, Air, Earth**
- *Each time Dahan would be Destroyed in target land, Destroy 2 fewer Dahan.* **OR** *Gather up to 2 Dahan.*
- **Threshold 3 Air**: You may do both.

Memory's best Unique per GodsLittleCow. Dahan-preservation + gather, **0 cost Fast**. In Ravage-heavy matchups (England, Sweden), the "2 fewer destroyed" clause is a second Defend effect layered on top of Learn's innate. At threshold (3 Air — easily hit with any air-drafted Minor), it's a two-effect card for one play: save Dahan + position them for counter-attacks. Sacred-site-R1 targeting means Shifting Memory's presence placement (sacred site on starting land) determines reach; early G1 Reclaim to place presence on the starting board preserves this targeting.

### Elemental Teachings

- **0 Energy · Fast · No Range · Any Spirit · Moon, Air, Earth**
- *Prepare 1 Element Marker. Discard up to 3 Element Markers. Target Spirit gains those Elements. (They can be any combination of Elements — the same or different.)*

The **element-gifting card** — Memory's multiplayer superpower. In solo it prepares a marker and does nothing else useful; in multi-spirit it hand-delivers up to 3 elements to a partner whose innate is starving. Best targets per GodsLittleCow: **Serpent** (wants all elements for Loci/Incarnates), **Starlight** (Starlight's Seeks wants specific elements to lock), **Fractured Days** (element-starved at base), **Lure**, **Finder**, **Earth** (for late innate stacking).

Carries Moon/Air/Earth natively — all three of Memory's own primary elements, so even the self-Prepare effect progresses your own thresholds.

## Key Strategic Principles

1. **Memory is a Moon spirit first, Earth second, Air third.** Draft Moon-heavy Minors; Earth Minors fill Learn and give secondary innate coverage; Air is valuable but don't chase it at the cost of Moon.
2. **Every Major plays at threshold.** With only 2 CP, a below-threshold single-land Major is a wasted play. Pair every Major-draft with a plan for how its threshold gets hit by T+1 (marker spend, minor-draft, or Reclaim-cycle).
3. **G4 is not the opening** — it's the late-game spike. Use G2/G3 in the opening to build tracks + Major-gain. G4 arrives at T4+ when a 5–6 cost Major is in hand.
4. **Hold at least 1 Moon marker at end of each turn.** This makes *next* turn's Observe L1 free — you cast Observe, spend the Moon-marker into it, get a new Prepare from the L1 effect. Break-even in markers, +1 elemental flexibility on the other actions.
5. **Forget = Discard.** Every Forget-cost Major you draft is effectively a deferred Reclaim. Don't hesitate to pay the "Forget" to gain a Major — that card returns to hand on Reclaim turns.
6. **Reclaim is a placement Growth.** G1 gives Reclaim + presence; don't think of Reclaim turns as "just card recycling" — they're also how you position for sacred-site-R1 targeting on Share Secrets.
7. **Learn L3 scales off Invader discard.** Once the deck starts discarding (turn 2+), Learn L3 grows by 2 Defend per turn. At T6+ it's often Memory's single-biggest defensive turn.
8. **The `markersforplay` CP slot unlocks 3+ card turns.** Don't waste it on early-game 1-marker turns; save it for the G4+markers-as-plays combo turn when you have a big Major to play.

```admonish tip title="The Moon-marker loop"
Start of turn: ≥1 Moon marker in bank. Cast Observe L1 (spends 1 Moon → gain 1 Moon) anywhere with an action happening. Result: end of turn, still ≥1 Moon. The loop is what makes Memory feel "energy-rich, marker-rich" across an entire game.
```

## Possible Openings

Three Rei-format opening variants drawn from GodsLittleCow's BGG guide + the jyonker13 latentoctopus openings thread. Each is a turn-by-turn rehearsal for the first 4 turns + a T5 state audit. Pick the variant that matches your adversary + scenario before game start — don't improvise T1. Cross-reference [Deliberate Play](../../fundamentals/deliberate-play.md) for the per-phase checklist.

Confidence scale: 🟥 tentative · 🟨 somewhat tested · 🟩 well-tested.

### Shared starting state (all openings)

Verified per `si-rules-check shifting-memory-of-ages`:

- **2 Presence on board**: highest-numbered Sands or Mountain.
- **4 Uniques in hand**: Boon of Ancient Memories (1E Slow, Moon/Water/Earth/Plant), Study the Invaders' Fears (0E Fast, Moon/Air/Animal), Elemental Teachings (0E Fast, Moon/Air/Earth), Share Secrets of Survival (0E Fast, Sun/Air/Earth).
- **3 Element Markers prepared**: 1 Moon, 1 Air, 1 Earth.
- **Starting income**: 0 Energy, 1 Card Play.
- Growth type **"one"**: pick one of G1/G2/G3/G4 per turn. G4 = +9 Energy (single-effect).

### Base-deck invader phase by turn (affects Ravage-protection cards)

| Turn | Explore | Build | Ravage | Consequence for opener prose                                     |
|------|---------|-------|--------|------------------------------------------------------------------|
| 1    | Stage 1 | —     | —      | No Ravage yet. Setup + T1 Explore only.                          |
| 2    | Stage 1 | Stg 1 | —      | Still no Ravage. T2 is pure build-up.                            |
| 3    | Stage 2 | Stg 1 | Stg 1  | **First Ravage** — prep Learn or Share Secrets by end of T2.     |
| 4    | Stage 2 | Stg 2 | Stg 1  | Ravage continues; Stage-2 spread begins.                         |
| 5    | Stage 3 | Stg 2 | Stg 2  | Stage-2 Ravage hits wider terrain set.                           |

### Growth-option decoder (T1 income branches)

| Growth | Opens up                                   | Income next turn (if reveal-slot-1) |
|--------|--------------------------------------------|-------------------------------------|
| G1     | Reclaim now (nothing to Reclaim T1)        | Bottom-track slot 1: CP track → 2 CP next turn |
| G2     | Gain Minor/Major + Top-track slot 1        | Top-track slot 1: 1 Energy next turn |
| G3     | +2 Energy + Top-track slot 1               | Top-track slot 1: 1 Energy next turn + 2 Energy now (= 2E for T1 card plays) |
| G4     | +9 Energy (bank)                           | No presence placement → no income advancement |

G3 is the T1 special: the +2E immediate covers a 1-cost card + a 1-cost card (two Slow plays if paired with any other energy). G2 + Major-gain is GodsLittleCow's preferred T1 branch — you pick which Major you commit to on T1 with full Minor/Major-tier information.

### Fast-phase element ceiling — load-bearing constraint

On T1, before any marker spend:
- Hand elements if all 4 played: Moon × 4, Air × 3, Earth × 2, Water × 1, Plant × 1, Animal × 1, Sun × 1.
- Observe L1 (1 Moon) can fire off any single card with Moon.
- Learn L1 (2 Earth) — one marker spend fills this if paired with Boon of Ancient Memories' Earth (= 2 Earth total).
- Observe L2 (2 Moon + 1 Air) — one marker spend on Moon + any Moon/Air card covers.

You can't reach Learn L3 (2 Moon + 3 Air + 4 Earth) without 4 turns of Earth-minor stacking. Don't plan T1–T3 around L3.

### Opening A — Fast Major (via G2 T1, Forget Boon) 🟨 (default)

**T1 · Growth**: G2 — Gain a Power Card (draft **Major**, Forget Boon of Ancient Memories) + Add 1 Presence on Top track (reveals slot 1: 1 E next turn).
- **Rationale**: Boon is the weakest Unique + its "Forget" is redirected to Discard via Special Rule, so it returns on Reclaim. Net cost: one card discarded, one Major in hand.

**T1 · Play** (0 Energy):
- Share Secrets of Survival (0E Fast, Sun/Air/Earth) → R1 from sacred-site land #5. Pre-positions against T3 ravage + banks elements.
- *Pause-point*: If the Major drafted costs 4+ and you're unsure about T2 energy, consider G3 instead (trades Major-gain for +2E now).

**T1 · Invader**: Explore only. No ravage consequence for Shifting Memory this turn.

**T1 end state**: 0 Energy, 4 cards in hand (3 Uniques + 1 Major, Boon in discard), 3 markers (1M/1A/1E), 3 Presence (2 start + 1 from G2).

**T2 · Growth**: G3 — Add 1 Presence on Bottom track (reveals slot 1: 2 CP) + 2 Energy.
- Income this turn: 1E (from T1's G2 slot reveal) + 2E (G3 immediate) = 3 Energy. CP = 2 this turn.

**T2 · Play** (3 Energy, 2 CP):
- The Major (if cost ≤ 3E and threshold reachable with markers). Example: a 2E Major needs 1E remaining after — pays for a fast play.
- Elemental Teachings (0E Fast, Moon/Air/Earth) → self-target Prepare + discard 3 markers to gift a partner. In solo, Prepare only (net +1 marker).
- **Alternative if Major is 4+ cost**: delay Major to T3, play Study the Invaders' Fears (0E, 2 Fear) + Share Secrets (0E, R1 defend).

**T2 · Invader**: Still no Ravage. Build only.

**T2 end state**: ~1 Energy banked (depends on Major cost), 2 cards in hand + Boon in discard, 3–4 markers.

**T3 · Growth**: G1 — Reclaim (brings Boon + played cards back) + Add Presence (Range 0, on sacred-site land).
- Income: 1E from T1 reveal + 1E from T2 reveal = 2E base this turn.

**T3 · Play** (2E, 2 CP):
- Major again (if applicable) — cost threshold now reachable with banked markers.
- Share Secrets or Learn-innate via Earth marker for first Ravage Defend.
- **Pause-point**: T3 is the first Ravage. If the Major's effect doesn't hit the ravage land AND you skipped Learn L1 prep, you will blight. Don't waste the Major unless it covers the ravage.

**T4 · Growth**: G2 — Gain second Major (Forget Boon again via discard-redirect) + top-track slot.

**T4 end state** (the "fully unlocked" audit point):
- **2 Majors in the rotation**.
- **~3 Energy, 2 CP baseline** (with markers-for-CP slot still available).
- Innate Defend active on most ravage lands.
- 6+ markers banked across the game.

**Confidence**: 🟨 somewhat tested. GodsLittleCow runs this baseline; multiple variants covered in his guide for 2-cost / 3-cost / 4-cost / 5+ cost Major drafts.

### Opening B — Slow Burn (reclaim-delay) 🟥

**When to pick**: the Major drafted on T1 is a **weak T2 target** (single-land, no partner synergy, threshold out of reach). Instead of forcing it, you stall one turn for a better Major or event preview.

**T1 · Growth**: G2 — Gain (draft Major, **Forget Study the Invaders' Fears + Boon**). Add Presence Top track.
- Unusual double-forget. Boon + Study both redirect to discard via Special Rule.

**T1 · Play**: Share Secrets + Elemental Teachings.

**T2 · Growth**: G2 again — Gain Minor or Major. Add Presence Top track.
- Income: 2E (two top-track slot reveals).

**T2 · Play** (2E, 2 CP): Share Secrets + Teachings again; hold Majors.

**T3 · Growth**: G3 — Add Presence Bottom + 2E.
- Major plays now: cost ≤ 3E + 2 CP for Major + 1 Minor.

**T5 · Reclaim** (first Reclaim).

Trade-off: Slow Burn sacrifices T2 board impact for T3 Major selection. Good vs. adversaries where T3 ravage is survivable (base England L1–2). Bad vs Prussia (T3 ravage is already a cliff).

### Opening C — Double Major 🟥 (advanced)

**Pick when**: 4P+ game where partners cover T2–T3 ravage; you're the late-game closer.

**T1 · G2 Top** — draft Major, forget Boon.
**T2 · G3 Bottom** — draft second Major via Boon-self-target (gains Minor, but combined with the T1 Major already in hand you have 2 big cards).
**T3 · G4** — +9 Energy. Play BOTH Majors if thresholds hit (expensive marker spend).

GodsLittleCow flags this as "rarely correct" — thresholding two Majors plus innates in a single turn is a multi-marker burn that leaves T4 starved. Use only when you've pre-drafted matching-element Majors (two Moon-based Majors, say).

### Opening Decision

- **Default to Opening A (Fast Major)** for base England, Prussia, Sweden at L3+.
- **Opening B (Slow Burn)** if T1 Major-draft is poor or if adversary permits T3 ravage softening.
- **Opening C (Double Major)** only in 4P+ with clear partner ravage coverage and a pre-planned matching-element pair.

## Card Priority Ratings

```admonish abstract title="Full-pool draft analysis"
Scored across the full Major + Minor deck (Base + B&C + JE + NI), weighted by Shifting Memory's innate element demands (Moon-heavy, Earth-secondary, Air-tertiary), mid-game energy estimate (5–7E by T4), and primary-innate speed. See [data/references/draft-priority/shifting-memory-of-ages.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/shifting-memory-of-ages.json) for full scoring.

- **Primary elements (innate-weighted)**: **Moon** (wt 4.5), **Earth** (wt 4.0), **Air** (wt 2.5), tie-breakers Fire/Water/Plant/Animal
- **Mid-game energy estimate (T3–T5 avg)**: 5.5E (phenomenal for a moderate-complexity spirit)
- **Primary innate speed**: Fast (both innates)
```

### Uniques — ranked (per GodsLittleCow)

1. **Share Secrets of Survival** — Best card. Dahan preservation + gather at 0E Fast. Pair with any Air minor to hit 3-Air threshold (do both halves).
2. **Elemental Teachings** — Second best. Multiplayer superpower; solo it's a +1 marker card.
3. **Boon of Ancient Memories** — Good but common Forget-fodder. Water/Plant are off-axis; its forget-avoidance clause for partners with Immense-aspect spirits is niche-valuable.
4. **Study the Invaders' Fears** — Good but most often Forgotten in multiplayer to gain Majors. Its Fear-deck-preview is player-count-scaling: 2P mild, 4P+ strong.

### Top 10 Minor Draft Picks (from full pool)

| # | Card | Cost | Speed | Elements | Reason (element-score → why pick) |
|---|------|------|-------|----------|------------------------------------|
| 1 | **Gift of Power** | 1 | Fast | Moon | Pure Moon-feeder (7.5 elem-score) — fuels Observe loop every turn |
| 2 | **Melt Earthen Flesh** | 0 | Slow | Moon, Earth, Plant | Moon+Earth double-prime (8.2) |
| 3 | **Drifting Into Stillness** | 1 | Slow | Moon, Plant | Moon-feeder + Defense synergy (7.0) |
| 4 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Moon + Fear (6.5) |
| 5 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air threshold fuel (7.0) |
| 6 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Moon + multi-element (6.8) |
| 7 | **Sea Monsters** | 2 | Slow | Moon, Water, Animal | Moon + off-axis utility (6.2) |
| 8 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Dahan-synergy + Moon/Earth (7.4) |
| 9 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | Earth-feeder for Learn (6.0) |
| 10 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | Moon + Earth at 0 cost (7.8) |

### Top 5 Major Draft Picks (from full pool)

GodsLittleCow's guide ranks Majors by: **Moon element (coloured green); multi-land effects (bolded)**. His S-tier is the intersection.

| # | Card | Cost | Speed | Elements | Why (per GLC) |
|---|------|------|-------|----------|----------------|
| 1 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | "S-tier: cheap, has moon, big defend, combos with Secrets' Dahan movement; counterattack fear + build-prevention" |
| 2 | **Tsunami** | 7 | Slow | Moon, Water, Earth | Multi-land + Moon; threshold-reachable with enough marker prep |
| 3 | **Sky Stretches to Shore** | 2 | Fast | Sun, Moon, Air | 2-cost Moon Fast — lowest-threshold-barrier of S-tier Majors |
| 4 | **Dream of the Untouched Land** | 4 | Fast | Moon, Sun, Plant | A-tier per reader comment in GLC thread; moon-feeder + big board effect |
| 5 | **Manifest Incarnation** | 3 | Fast | Moon, Plant, Animal, Water | Moon + NI utility |

### Cards to Avoid (anti-synergy flagged)

| Card | Reason (per GLC) |
|------|------------------|
| **Irresistible Call** | C-tier — shines only with a Volcano/destroy-combo; avoid vs Russia without partner |
| **Powerstorm** | No elemental synergy for Memory; combo-dependent (Lightning/Trickster partners) |
| **Blazing Renewal** | Late-game only — destroys Presence (can't afford early) |
| **Land Thrashes in Anger** | Late-game only — needs a heavy setup |
| **Murderous Darkness** | Late-game only — fear-intensive, off-axis elements |
| Single-land Majors below Moon-threshold | By default — Memory's CP ceiling punishes single-land non-threshold plays |

## Aspects (NI) — `[VERIFY mechanics]`

Shifting Memory has two Nature Incarnate aspects. Their JSON hasn't been scraped yet; mechanics below are from Wiki text + community summary, flagged `[VERIFY from physical aspect panel]`:

### Intensify — `[VERIFY]`

Alters the Prepare mechanic so markers stack or intensify per Prepare. Increases threshold reach at the cost of complexity. Preferred for solo/2P high-difficulty play.

### Mentor — `[VERIFY]`

Alters the Teach/Gift mechanic — shifts more value toward partner-gifting. Preferred for 3P+ games.

Aspect JSON import pending; `data/references/wiki/aspects/` currently only covers 5 aspects (Amorphous, Dark Fire, Foreboding, Madness, Reach).

## Adversary Matchup Matrix

| Adversary               | Rating | Matchup-specific note                                                 |
|-------------------------|--------|------------------------------------------------------------------------|
| England                 | ★★★★☆  | Learn L3 scales beautifully against coastal-City ravages               |
| Brandenburg-Prussia     | ★★★★☆  | "Fast Major opening is correct; early repeated terrains easier because T2 Major clears the land" (GLC). **TL2 wins typical.** |
| Sweden                  | ★★★★☆  | "Fast Major, don't stall; defend one land per turn with innate, handle other with Major." (GLC) **TL3 wins ~50% of his** |
| France-Plantation       | ★★★☆☆  | `[VERIFY]` — GLC marks this WIP; Dahan-attract mechanic may interact poorly with Secrets |
| Habsburg Mining         | ★★★☆☆  | `[VERIFY]` — WIP |
| Habsburg Livestock      | ★★☆☆☆  | `[VERIFY]` — WIP |
| Russia                  | ★★★☆☆  | `[VERIFY]` — WIP. Pogrom events reduce Dahan economy that Secrets relies on |
| Scotland                | ★★★☆☆  | `[VERIFY]` — WIP. Coastal City count is Scotland's hidden loss trigger; Memory's Defend innate helps |

### Strategy Cliff — Prussia (T2–T3)

Prussia's T2 ravage on a land already 1-Towned is a first-blight risk. Memory's response: T2 Major-play into that land (Trees Radiate, Sky Stretches) + Learn L1 prep. GLC: "expect 3 blight on first ravage unless a fast major saves you."

### Strategy Cliff — Sweden (T3 fear race)

Sweden punishes stalling. Memory wins ~50% of Sweden games at TL3 (fear track exhaustion) rather than board-clear. Draft Fear-heavy Majors + keep Study in play for the fear-preview.

## Board / Map Configuration

Per GodsLittleCow:

| Board | Rating | Reason |
|-------|--------|--------|
| **E** | ★★★★★ | Range-1 sacred-site access from land #5; free cascades; movable Dahan; only downside is no Dahan on blighted land |
| **B** | ★★★★☆ | Good Dahan density + standard adjacency |
| **C** | ★★★★☆ | Good Dahan + coastal access |
| **D** | ★★★★☆ | Workable |
| **A** | ★★☆☆☆ | Bad — adjacency and Dahan distribution both weak |
| **F** | ★★☆☆☆ | Decentralized Dahan + awkward starting blight |

Thematic: `[VERIFY — board ratings from base variant only]`

**Dahan placement heuristic**: select boards where **3 lands each have 2 Dahan, groupable in one Gather**. Isolated-Dahan boards starve Share Secrets's gather-half.

## Game-Phase Strategy

### Early (T1–3)

- Top-track G2 on T1 (draft Major, Forget Boon as discard-redirect).
- Bottom-track G3 on T2 (+2E, unlock 2 CP).
- G1 Reclaim on T3 (brings Boon back for re-Forget cycle).
- **Target by end of T3**: 1 Major in the rotation, 2 CP baseline, Learn L1 + Observe L1 firing each turn.

### Mid (T4–6)

- Second Major drafted by T4 (G2 again, Forget Study or Boon).
- Learn L2 becomes the Defend backbone (1 Air + 2 Earth — easily reachable with 1 marker).
- Observe L2 whenever target-land has 3 pending actions.
- **Target by end of T6**: 2–3 Majors, ~5E baseline, Learn L2 reliable.

### Late (T7+)

- Learn L3 scales (Defend 2 × Invader-discard-count).
- G4 for big Major-stack turns.
- `markersforplay` CP slot converts 4+ banked markers into card plays.
- **Target by end of T9**: game won on fear or board-clear; if not, Memory's late-game Majors close.

## Synergy Partners (Multiplayer)

Top-tier partners (per GLC):

1. **Serpent Slumbering Beneath the Island** — classic pairing. Serpent wants all elements for Loci/Incarnates; Memory's Teachings gifts them.
2. **Fractured Days Split the Sky** — all three Slip-tiers benefit from Memory's element-gifts.
3. **Vital Strength of the Earth** — Memory solves both Earth's pain points (energy starvation + draft diversity).
4. **Immense-aspect Lightning** — Boon's "pay 2 Energy instead of Forget" clause lets Lightning delay Reclaim one full turn.
5. **Thunderspeaker / Mist / Downpour / Vengeance / Shadows / Lure** — all Boon-gift targets per GLC.

**Support priority** (what Memory wants from partners):

1. Extra card plays
2. Slow→Fast conversion
3. Proliferation (token-adding)
4. Reclaim boosts
5. Element-gifting reciprocal
6. Card-gain support
7. Energy (lowest priority — Memory is already energy-rich)

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Default G4 as opening fuel.** G4 on T1–T3 wastes your energy-ceiling timing. Reserve G4 for T4+ Major-stack turns.
2. **Sub-threshold Major plays.** A 4-cost Major without threshold is ~2 points of effect for 4 Energy + 1 CP — that's a loss vs. a 1-cost Minor at threshold. GLC: "sometimes people throw money away simply because you can gain +9 at a time."
3. **Forgetting Secrets or Teachings.** GLC's direct pushback on someone who suggested Forgeting Secrets: "two saved Dahan can destroy a city for 0E, plus fast-phase Dahan movement combos." **Only Forget Boon and Study**.
4. **Drafting Irresistible Call or Powerstorm early.** These are combo-dependent; take them only once you know the partner-side (Volcano/Lightning).
5. **Stalling vs Sweden.** Sweden punishes delay; don't Slow-Burn open this matchup.
6. **Ignoring Dahan placement at board select.** Isolated Dahan is bad for Memory; want 3 lands with 2 Dahan each groupable in one Secrets-Gather.
7. **Not Reclaiming often enough.** Because Special Rule lets Forget→Discard, and Events/Blighted Island need card-discards, keeping 1–2 cards in hand is sometimes valuable. Reclaim also places presence via G1.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + Major drafted + Share Secrets played                 |
| 2    | G3 + Major played (if cost ≤ 3E) or held                  |
| 3    | G1 Reclaim + Major re-played (if applicable) + Learn L1   |
| 4    | G2 + second Major drafted — "fully unlocked" (2 CP, 3E baseline, 1–2 markers, 2 Majors in rotation) |
| 5–6  | Learn L2 reliable; Observe L2 on action-heavy lands       |
| 7+   | Learn L3 scales; G4 for stack turns                       |

## Expansion Sensitivity

- **Base + JE only**: plays as-designed. Secrets + Teachings + Learn L1/L2 is the core loop.
- **+ B&C**: Event-card reveal (Study) becomes more valuable; markers used to threshold B&C Majors.
- **+ NI**: Incarna-aspect partners become prime Teachings targets.
- **Without JE**: not possible — Memory is a JE spirit.
- **Without NI**: aspects (Intensify / Mentor) unavailable; base spirit still functional.

## Stat Snapshot `[VERIFY mindwanderer]`

mindwanderer win rates (approximate, last fetched): solo L5 ~42%, solo L6 ~36%. 2P averaged ~52% at L5. Below-mean for a JE spirit — reflects the learning curve on marker-management + Major-threshold planning. Veterans report significantly higher rates with the Fast-Major opening.

## Source Notes

- **Mechanics**: `data/references/wiki/shifting-memory-of-ages.json` (Wiki-parsed 2026-04-23).
- **Strategy**: [GodsLittleCow BGG thread 2915319](https://boardgamegeek.com/thread/2915319/strategy-guide-shifting-memory-of-ages) — paraphrased; direct quotes noted inline.
- **Openings**: [jyonker13 BGG thread 2506389](https://boardgamegeek.com/thread/2506389/openings-shifting-memory-ages) — cross-referenced.
- **Wiki rulings** (special-rule ambiguities): [Shifting Memory of Ages Wiki page §Rulings](https://spiritislandwiki.com/index.php?title=Shifting_Memory_of_Ages#Rulings).
