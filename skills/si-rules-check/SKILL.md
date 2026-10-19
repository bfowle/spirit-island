---
name: si-rules-check
description: Use BEFORE writing any spirit-chapter Opening Strategy T1/T2/T3 blocks, Tempo Profile tables, or per-turn Energy/CP/elements claims. Returns the spirit's Wiki-verified starting state + growth-option decoder + legal-play-combination calculator, so the resulting turn-by-turn prose is mechanically valid rather than hallucinated. Resolves the class of bug caught on Shadows T1 (wrote "Concealing + Mantle" when Shadows starts 0E/1CP and neither G2 branch allows that combo).
user-invocable: true
---

# si-rules-check — pre-write mechanics verification

## Why this exists

Real bug caught by Brett: I wrote Shadows's Opening A T1 as "G2 → play Concealing Shadows (0E) + Mantle of Dread (1E), 1E spent, Innate L1 fires." All three claims are false:

1. **Starting Energy is 0**, not 1. Shadows's `presence_energy_track[0] = "energy0"` per the Wiki.
2. **Starting Card Plays is 1**, not 2. `presence_cardplay_track[0] = "card1"`.
3. **G2 = gain 1 Minor + place 1 Presence from *one* track.** Whichever track you uncover gives you its *next* slot's income — not both tracks' slots.
4. Therefore no T1 configuration lets you play both a 0E card and a 1E card.

This skill prevents that class of error. **Invoke it before writing any mechanics-dependent section**.

## When to invoke

- Before writing a spirit chapter's **Opening Strategy** section (T1/T2/T3 blocks).
- Before writing a **Tempo Profile** table (per-round E/CP/presence claims).
- Before claiming an innate fires on a specific turn.
- Before enumerating "legal plays" for a turn.
- Before recommending a growth choice in `si-at-the-table`.

Do **not** use this skill for generic strategy commentary (archetype, synergies, adversary matchup prose) — only for mechanical claims.

## Input

The skill takes one argument: the spirit slug (matching the filename in `data/references/wiki/{slug}.json`).

Example: `si-rules-check shadows-flicker-like-flame`.

## Output

A structured markdown brief the writer can reference:

```
# Shadows Flicker Like Flame — Rules Check

## Starting state (T1, before any growth)
- Energy: 0  (from presence_energy_track[0] = "energy0")
- Card Plays: 1  (from presence_cardplay_track[0] = "card1")
- Hand: 4 Uniques (Concealing Shadows, Crops Wither and Fade, Favors Called Due, Mantle of Dread)
- Elements this turn: 0 of everything (cleared each turn)

## Growth options (pick ONE per turn; "growthtype=one")
- G1 (first=reclaim, second=gain1p): Reclaim all played+discarded. Gain 1 Minor. No presence placed.
  → T1 state after G1: 0E / 1CP (unchanged).
- G2 (first=gain1p, second=addpresence1): Gain 1 Minor. Place 1 Presence Range 1 from ONE track.
  → Place from CP track: reveal card2 → 0E / 2CP (2 cards, 0 energy).
  → Place from Energy track: reveal energy1 → 1E / 1CP (1 card, 1 energy).
- G3 (first=addpresence3, second=energy3): Place 1 Presence Range 3 from ONE track. +3 Energy this turn.
  → Place from CP track: reveal card2 → 3E / 2CP.
  → Place from Energy track: reveal energy1 → 1E+3 = 4E / 1CP.

## Unique cards (cost + speed + elements)
- Concealing Shadows: 0E Fast / Moon, Air
- Crops Wither and Fade: 1E Slow / Moon, Fire, Plant
- Favors Called Due: 1E Slow / Moon, Air, Animal
- Mantle of Dread: 1E Slow / Moon, Fire, Air

## Legal T1 plays (card combinations)
Given starting 0E/1CP + growth choice, enumerate:

**G1 (no track reveal)** → 0E / 1CP:
- 0 cards played, OR
- 1 card at cost ≤ 0: **Concealing Shadows alone** (only legal option).

**G2 via CP track** → 0E / 2CP:
- 0–2 cards played; sum of costs ≤ 0.
- Legal combos: {}, {Concealing}, {Concealing + [any 0-cost Minor drafted]}.
- **Illegal**: Concealing + Mantle (Mantle costs 1E, we have 0E).

**G2 via Energy track** → 1E / 1CP:
- 0–1 cards played; sum of costs ≤ 1.
- Legal combos: {}, {Concealing (0E)}, {Crops Wither (1E)}, {Favors Called Due (1E)}, {Mantle of Dread (1E)}.

**G3 via CP track** → 3E / 2CP:
- 0–2 cards played; sum of costs ≤ 3.
- Legal combos include: {Concealing + Mantle (0E+1E=1E)}, {Crops Wither + Mantle (2E)}, {Favors + Mantle (2E)}, etc.

**G3 via Energy track** → 4E / 1CP:
- 0–1 cards played; sum of costs ≤ 4.
- Legal: any single card.

## Element threshold matching (for the innate)

Innate: Darkness Swallows the Unwary (Fast, Range 1, optional Sacred Site target)
- L1: 2 Moon + 1 Fire  → Gather 1 Explorer.
- L2: 3 Moon + 2 Fire  → Destroy up to 2 Explorer. 1 Fear per destroyed.
- L3: 4 Moon + 3 Fire + 2 Air → 3 Damage. 1 Fear per Invader destroyed.

Cards' element contributions (from Wiki):
- Concealing Shadows: 1 Moon + 1 Air
- Crops Wither: 1 Moon + 1 Fire + 1 Plant
- Favors Called Due: 1 Moon + 1 Air + 1 Animal
- Mantle of Dread: 1 Moon + 1 Fire + 1 Air

For T1 innate firing (need 2M+1F), minimum requirement:
- 2 cards providing 2 Moon + 1 Fire: {Concealing + Mantle}, {Concealing + Crops Wither}, {Crops Wither + any Moon card}, etc. Card speed does not matter: a Slow card's Elements count from the moment it is played (see "Innate element timing").
- **No single card** provides 2M+1F → innate cannot fire T1 on a single-card turn.
- Therefore: **innate L1 on T1 needs 2 Card Plays**: G3-via-CP (3E: Concealing + Crops or Mantle), or G2-via-CP (0E) with a 0-cost Moon+Fire Minor drafted that turn and played alongside Concealing.

## What to cite in prose

When writing T1:
- Cite actual E/CP after the chosen growth.
- Name exact cards played with costs summed.
- Only claim innate firing if the summed elements cross the threshold.
- If you can't hit a threshold legally, say so: "Innate L1 does not fire T1; fires T2 after Reclaim."

## Audit hook

If a previously-authored chapter has a T1 block:
1. Re-run this skill for the spirit.
2. Check whether the chapter's claimed E/CP and play combinations are in the "Legal T1 plays" list.
3. If not, flag with `[VERIFY: illegal per rules-check]` and revise.

## Data sources

- `data/references/wiki/{slug}.json` — parsed spirit data (start tracks, growth options, unique cards).
- `scripts/wiki-fetch.py` — re-fetch if data is stale.
- Future: `tools/si-live/common/` Rust crate — canonical rule engine (Phase A+).

## Rules this skill enforces

1. **Starting state = first-uncovered track slots**. Not an assumption.
2. **One track reveal per presence placement** (not both tracks simultaneously).
3. **Growth options are atomic**: `first` + `second` + (optional) `third` effects all resolve together when you pick that growth.
4. **Card plays this turn ≤ current CP**.
5. **Sum of card costs ≤ current Energy**.
6. **Fast cards' effects resolve in the Fast phase; Slow cards' effects in the Slow phase.** Speed governs effects only, never when Elements arrive.
7. **Elements come from cards played this turn plus uncovered Presence-track Elements; card Elements reset at turn end.**
8. **Innate thresholds check the turn's total Elements — every card played this turn, Fast or Slow, plus uncovered Presence-track Elements — whatever the innate's speed; innates cost no Energy.** A Slow card's Elements count toward a Fast innate. See "Innate element timing" below.
9. **Special-rule usage (e.g., Shadows of the Dahan's 1E range extension) spends energy from the spirit's pool.**
10. **Invader phase contents by turn** determine which card effects are material vs. dormant that turn. See "Invader phase by turn" below.
11. **Dormant-effect rule**: card clauses gated on Ravage / Damage-from-Invaders are null effects on turns where the trigger does not occur in the target land. Don't cite them as opener value on those turns.

## Invader phase by turn (base-game standard invader deck)

As the last step of Setup, the Invaders reveal Card 1, Explore its terrain, and place it on the Build space (core rulebook, "Invaders' Starting Action"). The cards then advance one space per turn:

| Turn  | Explore | Build | Ravage | Notes                                                            |
|-------|---------|-------|--------|------------------------------------------------------------------|
| Setup | ✓       | —     | —      | Card 1 (Stage I) Explores, then moves to the Build space.        |
| 1     | ✓       | ✓     | —      | Card 1 Builds; Card 2 Explores. No Ravage.                       |
| 2     | ✓       | ✓     | ✓      | Card 1 → Ravage (first Ravage of the game); Card 2 → Build; Card 3 → Explore. |
| 3+    | ✓       | ✓     | ✓      | Full cycle continues; each card moves Explore → Build → Ravage → discard. |

**Implications for opener prose**:

- T1 cards tagged "prevents Ravage", "Dahan take no damage from Ravaging", "Invaders can't Ravage here", "Defend N" → null this turn. Only their Fear / elements / non-Ravage effects are material.
- T1 **does** Build (Card 1's terrain), so Build-prevention effects are material from T1.
- T2 is the earliest turn Ravage-gated effects are material, and only in lands Ravaged that turn (Card 1's terrain, known since Setup).
- Build rule: in each matching land **with Invaders**, if it has more Towns than Cities, add a City; otherwise (fewer or equal), add a Town. Lands without Invaders don't Build (exceptions: England L1+, Scotland L3+).
- Events (Branch & Claw+): the Turn-1 Event is drawn and discarded without resolving (Jagged Earth rulebook p.12); it is public in the Event discard. From T2 on, Events resolve in the Invader Phase, after Powers are chosen. Never write "plan Turn 1 around the Event."

**Adversary and scenario caveats**: some change this timing. Blitz gives the Invaders an extra set of actions at the end of Setup ("Explore, then Build and Explore"), so Card 1 should Ravage on Turn 1 (an inference from the card text, not yet confirmed in play; open-questions NQ-J). Brandenburg-Prussia L2+ and Russia L4+ move Stage III cards earlier; England L3+ adds a High Immigration Build before each Ravage; Sweden L4 Accelerates the deck at the end of Setup (discarding a card) and adds a Town per board in a land matching it. Check `data/references/rules/register.md` §4 (adversaries) and §5.2 (scenarios) before claiming standard base-deck timing.

## Innate element timing

Every Power Card played in the Spirit Phase, **Fast or Slow**, grants its Elements from the moment it is played, for the rest of the turn. Speed decides when a card's *effect* resolves, never when its Elements arrive.

> "You gain Elements the moment you pay for a Power Card, regardless of whether the Power is Fast or Slow, and they go away as soon as it leaves play (usually during Time Passes)." — Core rulebook p.16
>
> "You get Elements as soon as you play a Power Card during the Spirit Phase, not when you activate the card later in the turn." — FAQ, "Rules semi-commonly misplayed"
>
> Confirmed by Brett from the physical rulebook: `data/references/rules/open-questions.md` Q10 (register R-FS-2).

| Element source | Counts from | Ends |
|----------------|-------------|------|
| Power Card played in the Spirit Phase (Fast or Slow) | The moment it is played | When it leaves play (normally Time Passes). Forgetting it from play removes its Elements at once. |
| Power Card played mid-turn (by some effect) | That moment | As above |
| Presence-track Element space | The moment it is uncovered, including mid-turn | When it is covered again |
| Presence-track "Any" space | Each turn; the Spirit may choose the Element at any time | The choice is fixed until next turn |

**Consequence**: an innate's thresholds check the turn's total Elements regardless of the innate's speed. A Fast innate counts the Elements of the Slow cards you played this turn. The total changes mid-turn only when an Element is gained (a Presence-track space uncovered, a card played mid-turn) or lost (a card Forgotten from play). Elements are checked, never spent.

**Spirit exceptions**: some Jagged Earth and Nature Incarnate spirits change these rules through their own special rules. Cite one only when that spirit's special-rule text (engine `spirits.json` › `specialRules`) says so. Example: Shifting Memory of Ages' Element Markers each grant 1 Element "for a single Action", so a spent marker counts only for the Power it was spent on.

**Worked example — Shadows T1 Opening A** (G3 revealing the CP track → 3E / 2CP):
- Spirit Phase: play Concealing Shadows (0E, Fast, Moon + Air) and Mantle of Dread (1E, Slow, Moon + Fire + Air). Elements now: **2 Moon + 1 Fire + 2 Air**, for the whole turn.
- Fast Power Phase: Darkness Swallows the Unwary (Fast innate) checks L1 (2 Moon + 1 Fire): **met → L1 fires** (Gather 1 Explorer). L2 (3 Moon + 2 Fire): not met. Concealing's effect also resolves now.
- Invader Phase: Build + Explore (no Ravage on Turn 1).
- Slow Power Phase: Mantle's effect resolves (2 Fear, Push). Its Elements were already counted.

Authoring consequence: "L1 fires T1 via Concealing + Mantle" is **valid**. The invalid claim to catch is the reverse: "a Slow card's Elements arrive too late for a Fast innate." They don't.

**Elements do NOT persist across turns.** At Time Passes, card Elements go away. T2 starts with only uncovered Presence-track Elements, regardless of T1 plays.

## Dormant-effect detection

Card effects that depend on a trigger outside the current turn are **dormant** on turns where the trigger doesn't fire. When citing such a card in opener prose, you must explicitly say so.

Common dormant clauses + their trigger:

| Clause                                                     | Trigger (when material)                                                |
|------------------------------------------------------------|------------------------------------------------------------------------|
| "Dahan take 0/no Damage from Ravaging this turn"           | A Ravage happens in target land THIS turn (T2+ in base)                |
| "Defend N"                                                 | The target land takes Damage from a Ravage THIS turn                    |
| "Invaders can't Ravage here this turn"                     | A Ravage would otherwise happen in target land THIS turn                |
| "Dahan here deal 2 Damage during Ravages this turn"        | A Ravage happens in target land THIS turn                                |
| "Invaders in this land skip their next Build"              | A Build happens in target land THIS turn (T1+ in base; target-specific) |
| "Push N Explorers/Towns that were just brought by Explore" | An Explore placed units in target land THIS turn                       |

**Rule for opener writers**: on any turn where the trigger doesn't fire, write "dormant this turn; first becomes material on T<N>" rather than citing the effect as the card's T<current> value. The card's Fear / elements / other clauses remain material.

## Phase-timing summary block (included in skill output)

When invoked on a spirit, the skill returns a summary like:

```
# Shadows Flicker Like Flame — Phase Timing Summary

## Turn 1 invader contents
- Build (Setup's Explore card) + Explore. Ravage-protection / Defend effects are dormant T1; Build-prevention is live.
- Event (B&C+): drawn and discarded without resolving.

## Turn 2 invader contents
- Ravage (Setup's card) + Build + Explore. First turn Ravage-protection becomes material.

## Turn 3 invader contents
- Ravage + Build + Explore.

## Innate speed
- Darkness Swallows the Unwary: Fast → resolves in the Fast Power Phase, before the Invader Phase. Its thresholds check every Element you have this turn, Slow cards' Elements included.

## Element reach from Uniques alone (all four count, whatever their speed)
- Each Unique gives 1 Moon; only Crops Wither (Slow) and Mantle of Dread (Slow) give Fire. No Presence-track Elements.
- L1 (2M+1F): any 2 Uniques that include Crops or Mantle → minimum 1E / 2 CP (Concealing + Crops, or Concealing + Mantle). Concealing + Favors = 2 Moon, 0 Fire → no L1.
- L2 (3M+2F): Crops + Mantle + any third Moon card → minimum 2E / 3 CP.
- L3 (4M+3F+2A): the Uniques hold only 2 Fire in total → needs Fire from a drafted card and 4+ Card Plays.
```

Authors read this block **before** writing T1/T2/T3 prose and cite specific lines when asserting "L1 fires T<N>" or "dormant this turn" in their chapter text.

## Related

- `memory/feedback_verify_mechanics_before_writing.md` — the rule this skill enforces.
- `scripts/wiki-fetch.py` — data source.
- `skills/si-at-the-table/SKILL.md` — consumer of this skill's output for live-game advice.
