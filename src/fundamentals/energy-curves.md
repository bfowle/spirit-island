# Energy Curves

Every spirit has an energy curve — a per-round target income and spend profile. Hit the curve and your engine fires reliably; fall below and your hand rots; over-bank and your tempo stalls.

This chapter gives you the curve shapes for the four archetypes plus specific notes for the Brett-priority spirits.

## What "energy curve" means

Two numbers per round:

1. **Income** — what your presence track + growth produces this turn.
2. **Spend** — what your playable hand actually costs.

Target: income ≈ spend + 1 (a small buffer). Equality is fine; banking > 2 is usually wrong. Spending more than this turn's income is only possible from Energy banked earlier (unspent Energy carries over); if you need to do that every turn, you gained cards your curve can't afford.

```admonish example title="What to compute"
There is no universal per-round target: income depends on which Growth options you pick. Compute your own curve. Income on turn N = the highest uncovered number on your Energy track (after the presence you have added from it) + any Energy that turn's Growth gives. Spend = the cost of the cards you plan to play. Take the track arrays from your panel or from the [`si-rules-check`](../appendices/claude-skills-guide.md) skill, write down the growth sequence you plan for T1–T5, and fill in income and spend per turn. The archetypes below describe shapes to compare against, not numbers to hit.
```

## The four curve archetypes

### 1. Slow-ramp (Defend & Outlast, Major Power Shopping)

Example: Vital Strength of the Earth, Keeper of the Forbidden Wilds.

Slow-ramp banks 1E through T2–T3 to afford T5 Majors. Deviation cost: skipping the bank means no Major on T5, and the archetype fails.

### 2. Fast-energy (Lightning, high-E Horizons spirits)

Fast-energy spends aggressively — cards cycle, presence lands, and the spirit's pressure stays constant. Banking > 1E past T3 indicates you dropped a play.

### 3. Flat-cheap (Minor-heavy spirits)

Example: Downpour, Many Minds, Shroud (openings 1–2), Fangs.

Flat-cheap lives on 0–1 cost Minors. Income rarely clears 3E because it doesn't need to. Banking = error (you drafted the wrong Minors).

### 4. Late-spike (Vengeance, Serpent)

Late-spike lives for T7–T8. Early turns are weak by design; late turns are game-ending.

## Per-spirit energy tracks

All 37 spirits below. The numbers are each spirit's starting Energy per turn and the highest Energy space on its presence tracks, from the Steam engine's track data (`spirits.json`); the three spirits not yet in Steam are marked. Your T3 and T5 income falls between the two, depending on how many Energy-track spaces your growth uncovers by then — compute it as above. Physical panels are the ground truth if they differ.

**Base (8)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| River Surges in Sunlight | Mid | 1E | 5E |
| Lightning's Swift Strike | Fast-energy | 1E | 5E |
| Shadows Flicker Like Flame | Flat-cheap | 0E | 6E |
| Vital Strength of the Earth | Slow-ramp | 2E | 8E |
| Ocean's Hungry Grasp | Mid | 0E | 2E |
| A Spread of Rampant Green | Slow-ramp-ish | 0E | 3E |
| Thunderspeaker | Fast-energy | 1E | 3E |
| Bringer of Dreams and Nightmares | Flat-cheap | 2E | 5E |

**Branch & Claw (2)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Sharp Fangs Behind the Leaves | Flat-cheap | 1E | 4E |
| Keeper of the Forbidden Wilds | Slow-ramp | 2E | 9E |

**Promo — Feather & Flame (2)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Heart of the Wildfire | Flat-cheap (tight) | 0E | 3E |
| Serpent Slumbering Beneath the Island | Late-spike | 1E | 12E (track branches; read the panel) |

**Promo Pack 2 (2)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Downpour Drenches the World | Flat-cheap | 1E | 2E |
| Finder of Paths Unseen | Mid | 0E | 2E, plus "+Energy" spaces (track branches; read the panel) |

**Horizons (5)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Devouring Teeth Lurk Underfoot | Mid | 2E | 7E |
| Eyes Watch from the Trees | Mid | 1E | 4E |
| Fathomless Mud of the Swamp | Mid | 1E | 5E |
| Rising Heat of Stone and Sand | Mid | 1E | 5E |
| Sun-Bright Whirlwind | Mid | 1E | 6E |

**Jagged Earth (10)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Shifting Memory of Ages | Mid | 0E | 6E |
| Grinning Trickster Stirs Up Trouble | Mid (via innate) | 1E | 3E |
| Many Minds Move as One | Flat-cheap | 0E | 4E |
| Shroud of Silent Mist | Flat-cheap | 0E | 2E |
| Vengeance as a Burning Plague | Late-spike | 1E | 4E |
| Volcano Looming High | Mid | 1E | 5E |
| Stone's Unyielding Defiance | Slow-ramp | 2E | 6E |
| Lure of the Deep Wilderness | Mid | 1E | 5E |
| Fractured Days Split the Sky | Mid-high | 1E | 3E |
| Starlight Seeks Its Form | Flexible | 1E | 4E (track is shaped in play; read the panel) |

**Nature Incarnate (8)**:

| Spirit | Curve | Starting E income | Highest Energy space |
|---|---|---|---|
| Ember-Eyed Behemoth | Mid-high | 0E | 5E |
| Hearth-Vigil | Mid | not in Steam; read the panel | not in Steam; read the panel |
| Towering Roots of the Jungle | Slow-ramp | 1E | 6E |
| Breath of Darkness Down Your Spine | Mid | 1E | 5E |
| Relentless Gaze of the Sun | Mid | not in Steam; read the panel | not in Steam; read the panel |
| Wandering Voice Keens Delirium | Mid | 0E | 4E |
| Wounded Waters Bleeding | Mid | not in Steam; read the panel | not in Steam; read the panel |
| Dances Up Earthquakes | Mid | 1E | 4E, plus Impending Energy spaces (read the panel) |

## Deviation cost

Falling 1E below curve for 1 turn costs you ~1 card play.  
Falling 1E below curve for 3 turns compounds to ~3 missed plays.  
Banking 2+ E over curve means you missed a play; the extra energy doesn't make up for it.

## When to deviate on purpose

- **Save for a Major next turn**: legitimate bank. Limit: 1 turn.
- **Scenario-specific**: some scenarios change costs or plays. In Powers Long Forgotten, one Invader benefit makes Fire Powers cost an extra Energy to Damage/Destroy Invaders, and another limits Spirits to 1 Card Play next turn — both can force a bank.
- **Emergency stop**: burning 3E on A Year of Perfect Stillness (Vital Strength's Unique: Invaders skip all Actions in the target land this turn) can save a game; ask whether it's worth spiking out-of-curve for one turn.

## Repeat-a-Power — the hidden energy multiplier

Repeat effects (Vital Strength's Gift of Strength innate; the Major Power Powerstorm; innate tiers that read "Repeat this Power," such as A Spread of Rampant Green's Creepers Tear into Mortar, Finder's Lay Paths They Cannot Help But Walk, Lure's two innates, and Stone's Let Them Break Themselves Against the Stone) don't show up on your curve — but they can double a turn's effective card-play economy when they fire. Treat them as a separate axis:

- **Per-play value**: a 2-cost Minor with an effect worth ~X is normally X per play. Repeated, it's 2X. Read whether your Repeat makes you pay the cost again: Powerstorm does ("by paying its cost again"), so you get 2X for 4 Energy and save only the Card Play; Gift of Strength's text asks for no payment but only reaches cards costing 1 or less.
- **CP cost**: the card is played once (one CP) and resolves twice. On a 2-CP turn, that's half your effect-generation pipeline doubled.
- **Element count**: Repeating a Power re-counts its elements *for threshold purposes*? **No** — elements come from cards played, not from each resolution. Check the specific Repeat's text.

Strategic implications:

- A Repeat-capable spirit prices its cards differently. Vital Strength's Gift of Strength makes every 0- and 1-cost card worth more to them (and to the Spirit they target); a spirit holding Powerstorm saves Card Plays, not Energy.
- Repeat-on-Innate (Lay Paths They Cannot Help But Walk's Level-3 "Repeat this Power") is the element-thresholds-matter-most kind of Repeat. Draft minors that feed the threshold; the Innate pays back twice.

Draft heuristic: if your spirit has native Repeat access, ask what each Repeat-legal Power in the offer is worth played twice, and price the offer on that. This often flips a borderline Minor into a pick.

Card and innate texts: the Steam engine's `power_cards.json` and `spirits.json`. Source: community discussion at [BGG thread 3061696 — Repeating Slow Power Card with Fast Innate](https://boardgamegeek.com/thread/3061696/repeating-slow-power-card-with-fast-innate-power); rules detail at [Wiki — Repeat](https://spiritislandwiki.com/index.php?title=Repeat).

## Cross-references

- [Tempo](./tempo.md) — energy is one axis of tempo.
- [Major vs. Minor](./major-vs-minor.md) — draft decisions shaped by curve.
- [Card Draft Theory](./card-draft-theory.md) — draft in alignment with curve.

---

*Last revised: 2026-10-19*
