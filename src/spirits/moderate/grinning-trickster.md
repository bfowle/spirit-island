# Grinning Trickster Stirs Up Trouble

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Trickster Openings 1–3](https://latentoctopus.github.io/guide/trickster-opening1/) + [jyonker13's BGG thread 2502216](https://boardgamegeek.com/thread/2502216/openings-grinning-trickster-stirs-trouble).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | Moderate                                           |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "many" — pick multiple growth options per turn (bundled) |
| Power summary (1–5)   | **Offense 4** · Control 3 · Fear 2 · **Defense 5** · Utility 4 |
| Primary Elements      | **Air** (Let's See What Happens) · Moon (starter-dense) · Fire (Arson/Fight) |
| Special Rules         | A Real Flair for Discord (+1 Strife at 1 Energy after Strife-add) + Cleaning Up Messes Is a Drag (Blight removal destroys Presence) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Openings 1–3](https://latentoctopus.github.io/guide/trickster-opening1/) |
| BGG                   | [jyonker13 thread 2502216](https://boardgamegeek.com/thread/2502216) |
```

## Spirit Overview — Framing

Trickster is the **statistical-disruption spirit** — its kit weaponizes Strife tokens and the Minor Power deck's expected distribution. Let's See What Happens is the signature innate: you discard Minors until one has a land-targeting effect, then apply that effect immediately. It looks like a coin flip; it's actually a calibrated bet with specific probabilities.

**Wiki-printed playstyle note** (verbatim):

> Trickster isn't as wanton as it's made out to be — Strife is a very reliable token, and any extra income can easily translate to more of it if you like.

**Designer intent** (R. Eric Reuss / "darker", Discord):

> Trickster is designed to support/reward/encourage somewhat improvisational/seat-of-the-pants play.

**Identity capture** (jyonker13):

> Around 40% of the deck will deal with a lone Explorer. Pushing an Invader is about twice as likely as Gathering one (23% vs 10%). Around a third of the cards provide Fear.

**Complexity signal**: Moderate. Surface complexity is medium (3 innates, 2 special rules, 4 Uniques) but mastery requires *card-counting the Minor discard*. Reclaim loops are *reliable* but not *optimal* — designer says "the best Trickster play I've seen involves as little Reclaiming as they can get away with."

## Starting Setup

> Put **2 Presence** on your starting board: **1 in the highest-numbered land with Dahan, and 1 in land #4**.

## Growth Options (growthtype: "many" — pick multiple)

| Growth | Effects                                                   | Best when                                                      |
|--------|-----------------------------------------------------------|----------------------------------------------------------------|
| **G1** | Sharp1 (Thunder-style token) + Move 1 Presence (Range 1)  | Pair with G2/G3 for placement combos                           |
| **G2** | Add 1 Presence (Range 2)                                   | Board spread                                                   |
| **G3** | Gain 1 Power Card                                          | Card economy                                                   |
| **G4** | Energy + Card Plays (energycardplays = 1E + 1 extra CP)    | Energy spike turn                                              |

Trickster picks **multiple** growth options per turn. Opening cadence uses 2–3 per turn.

## Presence Tracks

- **Energy track** (6 slots): `energy1 → moon → energy2 → any → fire → energy3`
  - 1E → +Moon marker → 2E → +Any element → +Fire → 3E
- **Card-play track** (7 slots): `card2 → pushdahan → card3 → card3 → card4 → airX → card5`
  - 2 CP → Push Dahan (free) → 3 CP → 3 CP → 4 CP → Air scaling → 5 CP

**Starting income**: 1 Energy, 2 Card Plays. **"Any" element slot** on the energy track is unique — reveal to gain a Minor-like any-element marker.

## Core Mechanics & Special Rules

### Special Rule: A Real Flair for Discord

> After one of your Powers adds Strife in a land, you may pay 1 Energy to add 1 Strife within Range 1 of that land.

Per-action Strife multiplier. Each Strife-adding Power is a potential 2-Strife turn if you have 1E.

### Special Rule: Cleaning Up Messes Is a Drag

> After one of your Powers Removes Blight, Destroy 1 of your Presence. Ignore this rule for Let's See What Happens.

Blight-removal *costs* a Presence. Trickster is *not* a Blight-remover by design — avoid drafting Blight-removal Minors and never Blight-remove outside emergency.

### Innate: Let's See What Happens

- **Speed**: Fast · **Range**: 1 · **Target**: Invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Discard Minor Powers from the deck until you get one that targets a land. Use its text effects on target land immediately. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | You may Forget a Power Card to gain the just-used Power Card (to hand) and 1 Energy. |

The signature innate. L1 is the "deck-roulette" effect; L2 lets you keep a card that proved useful. Air-threshold (2 Air) is load-bearing.

### Innate: Why Don't You and Them Fight

- **Speed**: Fast · **Range**: 0 · **Target**: Invaders

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | This Power may be Slow. |
| 2     | 3 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Add 1 Strife. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-sun.png" alt="Sun"> Sun + 3 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire | 1 Invader and 1 Dahan deal Damage to each other. |
| 4     | 3 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | If target land has Beast, 2 Damage. Otherwise, you may Gather 1 Beast. |

The 3-Fire + 3-Sun tier (fynikz's winning pattern on BGG) stops both Build and Ravage: *"often capable of both stopping a build chain and a ravage every turn."*

## Unique Cards (all 4, Wiki-verified)

### Impersonate Authority
- **0 Energy · Slow · Range 1 · Any Land · Sun, Air, Animal**
- *Add 1 Strife.*

0-cost Strife-adder. A Real Flair for Discord adds a 2nd Strife for 1E. Combined with Incite the Mob, one of Trickster's opening two plays.

### Incite the Mob
- **1 Energy · Slow · Range 1 · Land with 1+ Invaders · Moon, Fire, Air, Animal**
- *1 Invader with Strife deals Damage to other Invaders (not to each Invader). 1 Fear per Invader this Power Destroyed.*

Strife-payoff + Fear. Requires Strife pre-set; Impersonate Authority seeds, Incite the Mob collects.

### Overenthusiastic Arson
- **1 Energy · Fast · Range 1 · Any Land · Fire, Air**
- *Destroy 1 Town. Discard the top card of the Minor Power Deck. If it provides Fire: 1 Fear, 2 Damage, and add 1 Blight.*

Town-destroy with ~38% chance of Fire-bonus (from Minor deck composition). The Blight-add on Fire-hit is risk-tolerable because of Trickster's non-Blight-remover identity.

### Unexpected Tigers
- **0 Energy · Slow · Range 1 · Any Land · Moon, Fire, Animal**
- *1 Fear if Invaders are present. If you can gather 1 Beasts, do so, then push 1 Explorer. Otherwise, add 1 Beasts.*

Beast-toolkit; situational utility. First Forget candidate for most matchups (keep longer vs. Russia for loss-condition hedging).

## Key Strategic Principles

1. **Air is the load-bearing element** — 2 Air unlocks Let's See What Happens. Draft Air Minors aggressively.
2. **Don't blight-remove.** Cleaning Up Messes destroys Presence. This is not an emergency kit.
3. **Reclaim loops are reliable but not optimal.** Designer intent is improvisational play; Reclaim every turn is conservative but Trickster-off-design.
4. **Trickster LOVES Energy** (jyonker13): *"Even 1 or 2 can get you more Strife, more cards, or a painless Reclaim."*
5. **Count the Minor discard** before Let's See What Happens — 40% lone-Explorer-dealing, 23% push, 10% gather, 33% fear.
6. **Unexpected Tigers is first Forget** — unless playing Russia (loss-condition hedge).
7. **Arson's 38% Fire-rate** is the correct per-use bet: *"every other use... that mentality keeps me from getting too Overenthusiastic."* (jyonker13)

## Possible Openings

### Shared starting state

- **2 Presence on board**: highest-Dahan land + land #4.
- **4 Uniques in hand**: Impersonate Authority (0E Slow, Sun/Air/Animal), Incite the Mob (1E Slow, Moon/Fire/Air/Animal), Overenthusiastic Arson (1E Fast, Fire/Air), Unexpected Tigers (0E Slow, Moon/Fire/Animal).
- **Starting income**: 1 Energy, 2 Card Plays.

### Opening A — Hybrid Minor 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/trickster-opening1/).

**T1 · Growth**: G3 + G2 top — Gain Minor + Presence.
**T1 · Play**: **Impersonate Authority + Incite the Mob** (prime Strife chain). Moon + Air priority on drafted Minor.
- If Arson-compatible draw: 0-cost Minor + Overenthusiastic Arson.

**T2 · Growth**: G2 top + G3 Minor.
**T2 · Play**: 3 plays hitting 1 Moon + 1 Fire + 2 Air (Let's See What Happens L1 unlock + fire).

**T3 · Growth**: G2 bottom + Reclaim.
**T3 · Play**: Impersonate + Incite + extra card.

**T4 · Growth**: G2 bottom + G4 (Energy+CP).
**T4 · Play**: 3 plays.

**T5 · Growth**: Reclaim; 3 plays.
**T6 · Growth**: G2 bottom + G3 Minor; 4 plays.

Goal: unlock LSWH tier 2 **without** a reclaim loop; Major-capable by T7.

### Opening B — Bottom Track Minor 🟨

From [Opening 2](https://latentoctopus.github.io/guide/trickster-opening2/).

**T1**: G3 + G2 bottom; Impersonate + Incite (Moon + Air priority).
**T2**: G2 bottom + G3; 3 plays at 2 Moon / 1 Fire / 2 Air. Forget from discard to unlock LSWH tier 2.
**T3**: Reclaim + G2 bottom; 3 plays.
**T4**: G2 top + G3 Minor; 3 plays.
**T5+**: Reclaim loop with G4 for energy.

**Critical constraint** (latentoctopus): *"This opening cannot reliably sustain a hand whose overall cost is more than 3."* **Never Forget Incite or Impersonate.**

### Opening C — Top Track Hybrid / Majors 🟥

From [Opening 3](https://latentoctopus.github.io/guide/trickster-opening3/).

**T1**: G3 + G2 top; Impersonate + Arson (if Air drawn) else Unexpected Tigers swap.
**T2**: G2 bottom + G3; Incite + Air card for LSWH L2; 4 cards, 2 Moon.
**T3**: Reclaim + G2 bottom; 2 plays hitting 2 Moon / 1 Fire / 2 Air.
**T4**: G2 bottom + G3 Major (forget from discard to keep 5 in hand); 3 plays.
**T5**: G2 top + G3 Minor; 3 plays.

jyonker13 guidance: go Top when draws are favorable (Dahan-movement for Strife trades), Plays-track when mixed.

### Opening Decision

- **Default Opening A** — balanced, hits LSWH L2 by T2.
- **Opening B** when card-volume over card-quality matters.
- **Opening C** only with Energy-donor partner (River/Downpour/Starlight).

## Card Priority Ratings

### Uniques — Trickster-specific ranking

1. **Impersonate Authority** — Strife-seeder, 0-cost every turn.
2. **Incite the Mob** — Strife-collector + fear.
3. **Overenthusiastic Arson** — Town-destroy with 38% upside.
4. **Unexpected Tigers** — situational, first Forget candidate.

### Top 10 Minor Draft Picks (Air > Moon > Fire)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air flex |
| 2 | **Call to Migrate** | 0 | Fast | Air, Animal | 0-cost Air |
| 3 | **Travel Unsuspected** | 1 | Fast | Air, Water | Air + Water |
| 4 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |
| 5 | **Bats Scout for Raids by Darkness** | 1 | Fast | Moon, Air, Animal | Moon + Air + Animal |
| 6 | **Entrancing Apparitions** | 1 | Fast | Moon, Air | Moon + Air flex |
| 7 | **Gift of Power** | 1 | Fast | Moon | Moon utility |
| 8 | **Visions of Fiery Doom** | 1 | Slow | Moon, Fire | Moon + Fire |
| 9 | **Elemental Boon** | 0 | Fast | Sun, Moon, Fire, Air | Four-element flex 0-cost |
| 10 | **Call of the Dahan Ways** | 1 | Slow | Moon, Earth | Moon flex |

### Top 5 Major Draft Picks (Opening C only)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Instruments of Their Own Ruin** | 3 | Fast | Fire, Air, Animal | Multi-element + Strife-synergy |
| 2 | **Strife-dealing Majors in general** | — | — | — | Multiplies A Real Flair for Discord |
| 3 | **Powerstorm** | 3 | Fast | Sun, Fire, Air | Air + cheap |
| 4 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Animal + Fire (Fight L3) |
| 5 | **Bargains of Power and Protection** | 3 | Fast | Sun, Moon, Fire, Air | Multi-element flex |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Blight-removal Minors | Cleaning Up Messes destroys Presence |
| Pure-damage without Strife synergy | Trickster's offense is Strife-chained, not raw damage |
| Presence-destruction Majors | Non-combo with Trickster's baseline presence-sparse play |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A       | ★★★★☆  | Strife-chain disables Ravage                                 |
| Sweden                  | A / B   | ★★★☆☆  | Build-density rewards Incite the Mob cycles                  |
| France-Plantation       | A       | ★★★☆☆  | Dahan-attract + Strife disruption                            |
| Russia                  | B       | ★★★☆☆  | **Keep Unexpected Tigers** (loss-condition hedge)            |
| Scotland                | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| England                 | B       | ★★★☆☆  | Raw card-volume + Strife-drip                                |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY — latentoctopus no per-board ratings]`. Solo: break Reclaim cycle earlier (City targets dry up faster on one board). Fear generation is outsized on a single board.

## Game-Phase Strategy

### Early (T1–3)
- Impersonate + Incite loop. Seed Strife, collect Fear.
- LSWH L2 unlock by T2–T3 via Minor-draft Air.

### Mid (T4–6)
- 3+ plays per turn; Reclaim cycles stabilize.
- Overenthusiastic Arson selectively for Town-destroy.
- Why Don't You and Them Fight L2 (3 Air) for per-turn Strife spam.

### Late (T7+)
- Major integration (Opening C lines).
- Fight L3/L4 for board-wide Invader vs. Invader damage.

## Synergy Partners (Multiplayer)

- **River** (Sunshine aspect especially) — energy donor; fynikz: *"absolutely makes his game and turns him into a major powerhouse."*
- **Downpour** — Gift of Abundance or Pour Down repeats.
- **Starlight** — element-targeting supports Trickster's LSWH reach.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Over-committing to Reclaim loops.** Designer: improvisational play is the design intent.
2. **Fearing Arson's Blight risk.** 38% Fire-rate; the damage clause is usually worth the Blight.
3. **Ignoring the Minor discard** before Let's See What Happens.
4. **Trying to Blight-remove.** Cleaning Up Messes cascades Presence loss.
5. **Forgetting Incite or Impersonate.** These are the engine — forget Tigers / Arson instead.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | Impersonate + Incite + Air-Minor drafted                  |
| 2    | LSWH L1 live; 3 plays                                     |
| 3    | Reclaim + G2 bottom; LSWH L2 unlocked via Forget          |
| 4    | G4 burst; 3 plays                                         |
| 5–7  | Reclaim cycles; Fight L2/L3 on ravage lands                |
| 8+   | Major integration (if Opening C); Fight L4 via Animal draft |

## Source Notes

- **Mechanics**: `data/references/wiki/grinning-trickster.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus Opening 1/2/3.
- **BGG**: [jyonker13 thread 2502216](https://boardgamegeek.com/thread/2502216/openings-grinning-trickster-stirs-trouble).
