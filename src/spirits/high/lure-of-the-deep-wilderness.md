# Lure of the Deep Wilderness

```admonish success title="Mechanics Wiki-verified 2026-04-23"
Card data, innate thresholds, special rules, growth options, presence track, and unique-card text below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py`. Remaining `[VERIFY]`: Play Difficulty, aspect mechanics, live mindwanderer stats, board ratings.

Strategic framing paraphrased from [latentoctopus Lure Openings 1–2](https://latentoctopus.github.io/guide/lure-opening1/) + [jyonker13's BGG openings thread 2510069](https://boardgamegeek.com/thread/2510069/openings-lure-deep-wilderness).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | Jagged Earth                                       |
| Complexity            | High                                               |
| Play Difficulty       | `[VERIFY physical spirit panel]`                   |
| Growth type           | "oneandone" — pick **one growth from G1–G3 AND one from G4** per turn |
| Power summary (1–5)   | **Offense 4** · **Control 4** · **Fear 4** · Defense 2 · Utility 1 |
| Primary Elements      | **Moon** (Forsake Society all tiers) · **Plant** (Never Heard From L2/L3/L4) · Air (Forsake L2 + L4) |
| Special Rules         | Home of the Island's Heart (Inland-only placement) + Enthrall the Foreign Explorers (2 Explorers/Presence don't Ravage) |
| Aspects               | None                                               |
| Rei's Guide           | Not covered                                        |
| latentoctopus         | [Openings 1–2](https://latentoctopus.github.io/guide/lure-opening1/) |
| BGG                   | [jyonker13 thread 2510069](https://boardgamegeek.com/thread/2510069) |
```

## Spirit Overview — Framing

Lure is a **token-blender centralizer** — spread thin across the middle of the island, then pull Invaders into a token-stacked killbox. Every Unique seeds a token or pulls pieces inland; every innate tier converts centralized tokens + Invaders into Fear + damage.

**Wiki-printed playstyle note**:

> Controls inland space by forbidding its own placement near coasts — then draws pieces into that deep interior to be ground down by stacked tokens.

**Identity capture** (jyonker13, BGG):

> A moderate complexity Spirit with a powerful built-in nuke whose Starters alone have all the Elements and effects you require to activate your strong innates? Hail, the Tokenspeaker!

**The non-obvious constraint** (Sh0rtz, BGG): starter uniques are so strong that Majors must *compete* with Swallowed by the Wilderness, not supplement it: *"you can reasonably expect it to deal 3–7 damage (2–5 before badlands), which is major power level for just 1 energy."*

**Complexity signal**: High. The Inland-only placement rule and "oneandone" growth pattern make Lure a planning-heavy spirit. Killbox topology must be designed on board-select, then held across 4+ turns.

## Starting Setup

> Put **3 Presence** on your starting board: **2 in land #8, and 1 in land #7**. Add **1 Beast to land #8**.

Three presences starting, pre-seeded with a Beast for immediate Swallowed by the Wilderness damage.

## Growth Options (growthtype: "oneandone" — pick one from G1–G3 + one from G4)

| Growth | Effects                                                     | Best when                                                |
|--------|-------------------------------------------------------------|----------------------------------------------------------|
| **G1** | Reclaim + +1 Energy                                         | Hand depleted; Reclaim turn                              |
| **G2** | Add 1 Presence (Range 4, Inland only)                       | Deep-interior spread                                     |
| **G3** | Prepare 1 Moon/Air/Plant marker + +2 Energy                 | Threshold-boost + energy spike                           |
| **G4** | Gain 1 Power Card (**Minor OR Major**)                      | Always picked as second growth                           |

**"oneandone" is distinctive**: every turn is a card-gain turn (G4) *plus* a main-effect growth from G1/G2/G3. This is why Lure's deck fills up fast and why the 3-plays-loop variant works.

## Presence Tracks

- **Energy track** (6 slots): `energy1 → energy2 → moon → energy3plant → energy4air → energy5reclaim`
  - 1E → 2E → +Moon marker → 3E + Plant → 4E + Air → 5E + Reclaim
- **Card-play track** (6 slots): `card1 → card2 → animalX → card3 → card4 → card5reclaim1`
  - 1 CP → 2 CP → Animal scaling → 3 CP → 4 CP → 5 CP + Reclaim 1

**Starting income**: 1 Energy, 1 Card Play. Energy curves smoothly without spikes — Lure's economy is planned, not improvised.

## Core Mechanics & Special Rules

### Special Rule: Home of the Island's Heart

> Your Presence may only be added/moved to lands that are Inland.

**Hard placement constraint**. Lure cannot place on coastal lands, ever. Board selection is existential — a board where most interior lands are distant from each other guts Lure's killbox.

### Special Rule: Enthrall the Foreign Explorers

> For each of your Presence in a land, up to 2 Explorer do not participate in Ravage.

Passive Explorer-cancel. 2 Presence = up to 4 Explorers skip Ravage. Key defensive clause — preserves interior lands without spending Powers.

### Innate: Forsake Society to Chase After Dreams

- **Speed**: Slow · **Range**: 1 · **Target**: Invaders

Text (per Wiki): *After this Power replaces pieces with Explorer: Gather any number of those Explorer into your lands. If target land has any Town/City remaining, 1 Fear.*

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon | Replace 1 Explorer with 1 Explorer. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | *Instead*, replace 1 Town with 2 Explorers. |
| 3     | 3 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | *Instead*, replace 1 City with 3 Explorers. |
| 4     | 4 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air | Repeat this Power. |

**Forsake's genius**: downgrade-to-Explorer then Gather the new Explorers into your killbox. L3 takes a coastal City and turns it into 3 Explorers in your Inland killbox for Never Heard From to grind.

### Innate: Never Heard From Again

- **Speed**: Slow · **Range**: 0 · **Target**: Inland

Text: *If this Power destroys any Explorer, 1 Fear. If this Power destroys 5 or more Explorer, +1 Fear.*

| Level | Thresholds | Effect |
|-------|------------|--------|
| 1     | 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> Fire + 3 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Add 1 Badlands. |
| 2     | 2 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Destroy up to 2 Explorers per Badlands/Beast/Disease/Wilds token. |
| 3     | 4 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant + 1 <img class="si" src="/spirit-island/theme/icons/element-animal.png" alt="Animal"> Animal | 2 Damage. |
| 4     | 6 <img class="si" src="/spirit-island/theme/icons/element-plant.png" alt="Plant"> Plant | Repeat this Power. |

The killbox-cash-out innate. L2 scales with tokens — 3 Badlands + 2 Beasts + 1 Wilds = 12 Explorers destroyed in one cast.

## Unique Cards (all 4, Wiki-verified)

### Gift of the Untamed Wild
- **0 Energy · Slow · No Range · Any Spirit · Moon, Fire, Air, Plant**
- *Target Spirit chooses to either: Add 1 Wilds to one of their lands.* **OR** *Replace 1 of their Presence with 1 Disease.*

Ally-token-gift. Lure can self-target (since the target is "Any Spirit") to seed Wilds/Disease on its own killbox, but the gift flows cleaner to a partner whose kit benefits from tokens.

### Perils of the Deepest Island
- **1 Energy · Slow · Range 0 · Inland Land · Moon, Plant, Animal**
- *1 Fear. Add 1 Badlands. Add 1 Beast within 1 Range. Push up to 2 Dahan.*

The killbox-seeder. 1 Badlands + 1 Beast + Dahan-push per play. Carries Moon + Plant + Animal — three of Lure's four primary elements.

### Softly Beckon Ever Inward
- **2 Energy · Slow · Range 0 · Inland Land · Moon, Air**
- *Gather up to 2 Explorers. Gather up to 2 Towns. Gather up to 2 Beasts. Gather up to 2 Dahan.*

**The magnet.** Gathers *everything* — up to 8 pieces. Combined with Forsake's downgrade-then-Gather, Softly Beckon pulls the entire adjacent board into one land for Swallowed's AOE.

### Swallowed by the Wilderness
- **1 Energy · Fast · Range 0 · Inland Land · Fire, Air, Plant, Animal**
- *2 Fear. 1 Damage per Beast/Disease/Wilds/Badlands. (Count max. 5 tokens.)*

**The payoff.** 1 Fear + 1 damage per token, capped at 5 tokens = 5 damage + 2 Fear at 1E Fast. jyonker13: *"major power level for just 1 energy."* This is the card Majors must out-compete.

## Key Strategic Principles

1. **Spread thin across interior lands, then pull.** Lure doesn't clear adjacent lands — it *makes* adjacent lands pull into its killbox.
2. **Moon is primary, Plant secondary, Air tertiary.** Animal falls out of good drafts but is the "least important" (bmartin2009 flags: Animal is easy to hit *in isolation* but hard *while also* hitting Moon+Air).
3. **Any Major must out-value Swallowed at 1E.** Most don't.
4. **Coastal problems are Forsake Society's job.** L2 downgrades coastal Towns into Explorers that Gather inland.
5. **3-plays loop** works vs Prussia but fails vs England/Scotland (build volumes overwhelm token economy).
6. **Don't lose Perils.** It seeds the killbox; "basically sets up a new blender single handedly" (jyonker13).
7. **Dahan Moving Out** — Perils + Softly Beckon shuffle Dahan into and out of the killbox for counterattacks.

## Possible Openings

### Shared starting state

- **3 Presence + 1 Beast** on land #8 / #7 starting.
- **4 Uniques in hand**: Gift of the Untamed Wild (0E Slow, Moon/Fire/Air/Plant), Perils of the Deepest Island (1E Slow, Moon/Plant/Animal), Softly Beckon Ever Inward (2E Slow, Moon/Air), Swallowed by the Wilderness (1E Fast, Fire/Air/Plant/Animal).
- **Starting income**: 1 Energy, 1 Card Play.

### Opening A — Full Bottom Track / Minors 🟨 (default)

From [latentoctopus Opening 1](https://latentoctopus.github.io/guide/lure-opening1/).

**T1 · Growth**: G2 bottom (+ G4 Minor) — Add Presence + Gain Minor (Moon/Air priority).
**T1 · Play** (1E, 1 CP): **Gift of the Untamed Wild + Perils of the Deepest Wild**.
- Forsake L2 and Never Heard From L2 both live. 5 cards, 1 play.

Wait — Gift + Perils = 1 Plant + 1 Animal + 1 Moon + 1 Air + 1 Moon; enough for Forsake L1 / L2 and Never Heard From L1. 0E + 1E = 1E spent.

**T2 · Growth**: G2 bottom + G3 Moon; **Softly Beckon + Swallowed**.
- Forsake L2; 3 cards, 4E.

**T3 · Growth**: Reclaim (G1) + G3 Moon (or G4 if City needs pressure); 2 plays, 5–6 cards, 8E.

**T4 · Growth**: G2 bottom + G3 Moon/Plant or G4.
- 3 plays, 4 cards, 9E.

**Caveat**: "Playing 1-cost Minors will often require not playing one of your 1- or 2-cost Uniques, so it's generally recommended to pick 0-cost."

### Opening B — Hybrid Majors (jyonker13 canonical) 🟨

From [Opening 2](https://latentoctopus.github.io/guide/lure-opening2/).

**T1 · Growth**: G2 bottom + G3 Plant.
**T1 · Play**: Gift + Perils. Left innate L2, right innate L1+L2. 4 cards, 3E.

**T2 · Growth**: G2 top + G4 Major (target ≤4E, Moon or Plant elements).
**T2 · Play**: Softly Beckon + Swallowed (or Major if 2–3 cost hit).
- 3 cards, 5E.

**T3 · Growth**: Reclaim (G1) + G4 Major (replacing) or Minor.
**T3 · Play**: Major + Gift. 5 cards, 8E.

**T4 · Growth**: G2 bottom + G3 Moon/Plant.
**T4 · Play**: Softly Beckon + Swallowed.

Core idea (jyonker13): *"Gain an early Major and alternate playing it with playing Softly Beckon Ever Inward every reclaim cycle."*

### Opening C — 3-Plays Loop (BGG bizenboat, doolee-validated) 🟥

**T1**: Plays-track + Plant; Softly Beckon + Perils.
**T2**: Plays-track + Moon; Swallowed + Gift.
**T3**: Reclaim + Moon Minor; Softly Beckon + Perils + Forsake.
**T4**: Plays-track + Moon; Swallowed + Gift + Minor.
**T5+**: Reclaim loop at 3 plays.

doolee: validated 3/3 at L6 two-spirit. **Trade-off** (Sh0rtz, gpope): fragile to presence-destroying Events and to Scotland/England build volumes — *"really struggles to keep up against the likes of Scotland or England where you're contending with 4+ new buildings per turn."*

### Opening Decision

- **Default Opening A** (Full Bottom Minors) — reliable across Prussia / Sweden / France / Russia.
- **Opening B** (Hybrid Majors) when the Major draft lands with Moon or Plant at ≤4E.
- **Opening C** (3-Plays Loop) vs Prussia with partner cover; avoid vs England/Scotland.

## Card Priority Ratings

### Uniques — Lure-specific ranking

1. **Swallowed by the Wilderness** — the payoff; 5-damage 1E Fast.
2. **Softly Beckon Ever Inward** — the magnet; gathers everything.
3. **Perils of the Deepest Island** — the seeder; 1 Badlands + 1 Beast per play.
4. **Gift of the Untamed Wild** — ally gift + self-target Wilds/Disease.

### Top 10 Minor Draft Picks (Moon > Plant > Air)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Predatory Nightmares** | 0 | Fast | Moon, Animal | 0-cost Moon |
| 2 | **Drifting Into Stillness** | 1 | Slow | Moon, Plant | Moon + Plant |
| 3 | **Pull Beneath the Hungry Earth** | 0 | Slow | Moon, Earth | 0-cost Moon |
| 4 | **Call to Isolation** | 0 | Slow | Water, Animal | Isolate helps killbox |
| 5 | **Strange Tales of the Sky** | 1 | Fast | Moon, Air | Moon + Air |
| 6 | **Unrelenting Growth** | 0 | Slow | Sun, Plant | 0-cost Plant |
| 7 | **Quicken the Earth's Struggles** | 0 | Slow | Earth, Plant, Animal | 0-cost Plant + Animal |
| 8 | **Song of Sanctity** | 0 | Slow | Sun, Plant, Animal | 0-cost Plant |
| 9 | **Call to Bloodshed** | 0 | Slow | Moon, Animal | 0-cost Moon |
| 10 | **Rain of Blood** | 1 | Slow | Moon, Fire, Water | Moon + Fire (Never Heard From L1) |

### Top 5 Major Draft Picks (Opening B)

| # | Card | Cost | Speed | Elements | Why |
|---|------|------|-------|----------|-----|
| 1 | **Insatiable Hunger of the Swarm** | 3 | Fast | Animal, Plant | Plant + Animal multi-land |
| 2 | **Melt Earthen Flesh** | 0 | Slow | Moon, Earth, Plant | Moon + Plant at 0E (!) |
| 3 | **Trees Radiate Ancient Sanctity** | 3 | Fast | Moon, Sun, Plant, Earth | Moon + Plant board-wide |
| 4 | **Dream of the Untouched Land** | 4 | Fast | Moon, Sun, Plant | Moon + Plant |
| 5 | **Tigers Hunting** | 3 | Fast | Fire, Animal | Cheap damage |

### Cards to Avoid

| Card | Reason |
|------|--------|
| Coastal-only Powers | Inland-only placement makes them deadweight |
| Single-Animal Minors that miss Moon/Air | Bmartin2009's warning |
| Presence-destroying effects | Lure's kit is Presence-thin |

## Adversary Matchup Matrix

| Adversary               | Opening | Rating | Matchup note                                                 |
|-------------------------|---------|--------|--------------------------------------------------------------|
| Brandenburg-Prussia     | A / C   | ★★★★☆  | Strong matchup; 3-plays loop viable                          |
| Sweden                  | A / B   | ★★★☆☆  | Build-spam still survivable                                  |
| France-Plantation       | A       | ★★★☆☆  | Dahan synergy via Perils push                                |
| Russia                  | A / B   | ★★★☆☆  | `[VERIFY]` — Pogrom events stress presence                    |
| **England**             | A / B   | ★★☆☆☆  | **Weak matchup** — coastal Cities bypass Inland-only placement |
| **Scotland**            | A       | ★★☆☆☆  | **Weak matchup** — build volumes overwhelm token economy     |
| Habsburg Mining         | A       | ★★★☆☆  | `[VERIFY]`                                                   |
| Habsburg Livestock      | A       | ★★☆☆☆  | `[VERIFY]`                                                   |

## Board / Map Configuration

`[VERIFY — latentoctopus no per-board ratings]`. Heuristic: favor boards where **interior lands are densely adjacent** (enables Softly Beckon magnet reach). Boards with scattered interior lands (widely-spaced #7/#8) gut Lure's killbox topology.

## Game-Phase Strategy

### Early (T1–3)
- Seed killbox: Perils (Badlands + Beast), Gift (Wilds).
- Forsake L1/L2 every Slow phase.
- Swallowed T1–T2 builds Fear + damage.

### Mid (T4–6)
- Softly Beckon on coastal-adjacent killbox.
- Never Heard From L2 (2 Plant) scales with token count.
- Major-integration (Opening B).

### Late (T7+)
- Forsake L3 (3 Moon + 2 Air + 1 Animal) converts Cities to 3 Explorers.
- Never Heard From L3/L4 = Repeat Power for multi-land grinding.

## Synergy Partners (Multiplayer)

- **Coastal-focused partners** — Ocean, Serpent coast-clear what Lure can't reach.
- **Invader-push partners** — any partner who funnels Invaders into Lure's lands.
- **Gift of the Untamed Wild targets** — partners whose kit wants extra Wilds/Disease tokens.

## Common Mistakes

```admonish failure title="Patterns to watch for"
1. **Letting Majors replace Swallowed.** Most Majors don't exceed its per-energy value.
2. **Drafting Animal-heavy Minors that miss Moon+Air.** Animal is easy to hit *in isolation* but doesn't stack with Moon/Air threshold paths.
3. **Committing to 3-plays loop against high-build adversaries.** Scotland/England overwhelm the token economy.
4. **Forgetting Perils.** jyonker13: *"Perils basically sets up a new blender single handedly."*
5. **Ignoring Inland-only placement at board select.** Widely-spaced interior lands = dead Lure game.
```

## Tempo Profile

| Turn | Target state                                              |
|------|-----------------------------------------------------------|
| 1    | G2 + G4 Minor; Gift + Perils; Forsake L2 unlocked         |
| 2    | G2 + G3 Moon; Softly Beckon + Swallowed                   |
| 3    | Reclaim + G3 Moon or G4 Major (Opening B)                 |
| 4    | 3 plays; killbox active (3+ tokens)                       |
| 5–7  | Softly Beckon → Never Heard From L2 cycle                 |
| 8+   | Forsake L3 downgrades Cities; Repeat via L4                |

## Source Notes

- **Mechanics**: `data/references/wiki/lure-of-the-deep-wilderness.json` (Wiki-parsed 2026-04-23).
- **Openings**: latentoctopus Opening 1 + Opening 2.
- **BGG**: [jyonker13 thread 2510069](https://boardgamegeek.com/thread/2510069/openings-lure-deep-wilderness).
