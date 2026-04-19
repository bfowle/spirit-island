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
- 2 cards providing 2 Moon + 1 Fire: {Concealing + Mantle}, {Concealing + Crops Wither}, {Crops Wither + any Moon card}, etc.
- **No single card** provides 2M+1F → innate cannot fire T1 on a single-card turn.
- Therefore: **innate L1 requires at least G3-via-CP-track + 2 qualifying cards, OR deferring L1 firing to T2 via reclaim cycle.**

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
6. **Fast cards resolve in Fast phase; Slow in Slow phase.**
7. **Elements accumulate from cards played that turn only; reset at turn end.**
8. **Innate firing requires threshold elements *this turn*; free of energy cost.**
9. **Special-rule usage (e.g., Shadows of the Dahan's 1E range extension) spends energy from the spirit's pool.**

## Related

- `memory/feedback_verify_mechanics_before_writing.md` — the rule this skill enforces.
- `scripts/wiki-fetch.py` — data source.
- `skills/si-at-the-table/SKILL.md` — consumer of this skill's output for live-game advice.
