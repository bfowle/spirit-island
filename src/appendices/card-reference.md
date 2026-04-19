# Curated Card Reference

A cross-expansion catalogue of the most-frequently-drafted cards, organized by archetype. This is a reading reference, not a draft calculator — for the latter, use each spirit's [Card Priority Ratings](../spirits/index.md) section (full-pool ranked per spirit).

## Organization

1. **By archetype** — grouped by what-they-do (Fear, Damage, Push, Dahan, etc.).
2. **By element** — for element-hungry innates, grouped by element signature.
3. **By cost curve** — what 0-cost, 1-cost, 3+-cost cards exist.

## By archetype

### Fear-generators

- **Vengeance of the Dead** (Major, Moon+Fire+Animal, 3E Fast) — 3 Fear + conditional damage
- **Angry Mobs** (Fear card, TL1) — agency when the fear pool crosses
- **Favors Called Due** (Shadows Unique, 1E Slow) — up to 3 Fear if Dahan outnumber
- **Terrifying Nightmares** (Major, Moon+Air, 4E Fast) — 2 Fear + push up to 4
- **Shadows of the Burning Forest** (Minor, Moon+Fire+Plant, 0E Slow) — 2 Fear + Mountain/Jungle push
- **Visions of Fiery Doom** (Minor, Moon+Fire, 1E Fast) — 1 Fear + push

### Damage-dealers

- **Pillar of Living Flame** (Major, Fire+Air, 3E Fast) — 4 Damage
- **Crashing Torrent of the Sky** (Major, Water+Air, 2E Slow) — 3 Damage
- **Tigers Hunting** (Major, Animal, 3E Fast) — 3 Damage + Beasts
- **Pyroclastic Flow** (Major, Fire+Earth, 3E Slow) — heavy damage + blight
- **The Jungle Hungers** (Major, Moon+Plant, 3E Slow) — board wipe ⚠️ destroys Dahan

### Push / Gather (control)

- **Entwined Power** (Major, Plant+Sun, 3E Fast) — major control + gather
- **Gift of Living Energy** (Minor, Sun+Animal, 1E Slow) — target-spirit buff
- **Call to Migrate** (Minor, Animal, 1E Fast) — move all Dahan

### Dahan interactions

- **Call of the Dahan Ways** (Minor, Moon+Water+Animal, 1E Slow) — Replace 1 Explorer with 1 Dahan
- **Inflame the Fires of War** (Minor, Fire+Animal, 1E Slow) — Dahan do Damage
- **War Leader of the Isle** (Major, Sun+Air+Animal, 3E Slow) — Dahan empowerment

### Presence-placement (land-control)

- **Boon of Vigor** (Minor, Sun+Plant, 0E Slow) — Target Spirit presence grants
- **Voracious Growth of the Understory** (Minor, Plant, 1E Slow) — Defend via presence density

## By element signature

### Moon-heavy (Shadows, Bringer, Breath of Darkness)

See Shadows chapter's top 10 Minor picks — all Moon-bearing, most add Fire or Air.

### Sun-heavy (Relentless Gaze, Keeper of the Forbidden Wilds)

- **Sunset's Fire Flows Across the Land** (Minor, Sun+Moon+Fire+Water)
- **Purifying Flame** (Minor, Sun+Fire)
- **Favor of the Sun and Star-lit Dark** (Minor, Sun+Moon)

### Earth-heavy (Vital Strength, Towering Roots, Dances Up Earthquakes)

- **Absorb Corruption** (Minor, Earth+Plant, Sun+Water on threshold)
- **Carapaced Land** (Minor, Earth+Sun)
- **Quicken the Earth's Struggles** (Minor, Earth+Animal)

### Animal-heavy (Sharp Fangs, Serpent, Hearth-Vigil)

- **Savage Mawbeasts** (Minor, Animal)
- **Prowling Panthers** (Minor, Fire+Moon+Animal)
- **Settle Into Hunting-Grounds** (Major, Moon+Fire+Plant+Animal)

## By cost curve

### 0-cost Fast Minors (play-every-turn candidates)

- Land of Haunts and Embers (Moon+Fire+Air)
- Lure of the Unknown (Moon+Fire+Air+Plant)
- Roiling Bog and Snagging Thorn (Moon+Fire+Water+Plant)

### 0-cost Slow Minors

- Animated Wrackroot (Moon+Fire+Plant)
- Shadows of the Burning Forest (Moon+Fire+Plant)
- Boon of Vigor (Sun+Plant)

### 1-cost Fast Minors

- Dark and Tangled Woods (Moon+Earth+Plant)
- Visions of Fiery Doom (Moon+Fire)
- Rites of the Land's Rejection (Moon+Fire+Earth)

### 1-cost Slow Minors

- Call of the Dahan Ways (Moon+Water+Animal)
- Favors of Story & Season (Moon+Animal)
- Unquenchable Flames (Moon+Fire+Earth)

### 3+ cost Majors

See [Major vs. Minor Draft Rates](../statistics/major-vs-minor-rates.md) for draft-bias context.

## Data source

Every card listed here is present in `data/decks/{minor,major}.json`, parsed from the [Spirit Island Wiki](https://spiritislandwiki.com) via `scripts/wiki-fetch.py`. For authoritative card text + thresholds, cross-reference the JSON directly.

## See also

- Per-spirit **Card Priority Ratings** — full-pool ranked picks.
- [Major vs. Minor Draft Rates](../statistics/major-vs-minor-rates.md) — strategic draft framework.
- [Fear, Blight & Event Decks](fear-blight-event-decks.md) — those decks' cards.
