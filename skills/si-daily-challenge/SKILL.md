---
name: si-daily-challenge
description: Use when Brett asks for "today's Spirit Island challenge" or says "/si-daily-challenge". Generates a structured daily drill — spirit × adversary × level × scenario × board × player-count × learning goal — seeded by date, varied per reroll, and informed by his playlog history in ~/repos/spirit-island/data/playlog.csv.
user-invocable: true
---

# si-daily-challenge — Daily Spirit Island Drill

Generates a single daily challenge card for Brett. The selection logic lives in
`generate.py` (same directory) so the randomization, weighting, and playlog
reads are deterministic and testable instead of hand-rolled in prose.

## How to invoke

Default daily run (date-seeded, stable within a single day):

```bash
python3 ~/.claude/skills/si-daily-challenge/generate.py
```

Pipe stdout into your response verbatim — it is already formatted as a
markdown challenge card.

## Re-rolls and overrides

Pass flags to vary the draw. The script already handles edge cases; prefer a
flag over re-describing intent in prose.

| Brett says | Flag |
| --- | --- |
| "pick a different spirit" | `--different-spirit` |
| "pick a different adversary" | `--different-adversary` |
| "different goal" | `--different-goal` |
| "reroll" / "another one" | `--reroll N` (bump N each time) |
| "I want harder" | `--harder` |
| "easier" | `--easier` |
| "give me X spirit" | `--spirit <slug>` |
| "vs England" / "against <adv>" | `--adversary <slug>` |
| "at level N" | `--level N` |
| "solo" / "2-handed" / "3-handed" | `--players 1|2|3` |
| "no scenario" | `--no-scenario` |
| "with <scenario>" | `--scenario <slug>` |

Multiple flags compose. If Brett re-runs `/si-daily-challenge` within the same
day, bump `--reroll` (1, 2, 3…) so the draw actually changes — the default
seed is the date.

## Selection logic (reference — actual behaviour lives in generate.py)

1. **Spirit** — weighted random across all 37 spirits. Priority flag adds +2,
   full chapter adds +2 (both stack). Recent-5 from playlog are excluded.
2. **Adversary** — from Brett's go-to pool (England / Sweden / Brandenburg-
   Prussia / France-Plantation). Least-recently-played wins, random tiebreak.
3. **Level** — walks that adversary's `difficulty_levels` ladder. Default =
   lowest rung; +1 after 2 consecutive wins at the current rung against that
   adversary; −1 after 2 consecutive losses.
4. **Scenario** — 20% chance; weighted toward diff_mod ≤ 1.
5. **Board** — one random board per spirit. 70% base (A–D), 30% expansion
   (JE E–F, HoSI G–H).
6. **Player count** — 55% true solo, 30% 2-handed, 10% 3-handed, 5% 2-player.
7. **Learning goal** — date-rotated through 7 categories; `--different-goal`
   skips one slot.

## Output shape

The script prints:

```
## Today's Challenge — YYYY-MM-DD

**Spirit**: …
**Adversary**: … Level …
**Scenario**: …
**Board**: …
**Player count**: …
**Learning goal**: …

### Read first (15 min)
- Spirit chapter: …
- Fundamentals: …
- Adversary chapter: …

### Turn-1 commit
Before you start, write on paper: "By T3 I will have …"

### After the game
Run `si-post-game` to log; tomorrow's challenge shifts based on results.
```

A stub-chapter footnote appears when the chosen spirit has no full chapter,
pointing Brett at the spirit index + Wiki.

## JSON mode

For programmatic callers: `generate.py --json` emits the raw selection as
JSON (spirit, adversary, level, scenario, boards, goal, player_count, seed).

## Anti-alpha note

Never prescribe specific plays. The card names chapters and goals; the
chapter has the turn-by-turn detail. Advisory, not prescriptive.

## After output

Ask in one line: "Ready for this, or want me to swap the spirit / adversary /
goal / level?" — then stop. Don't narrate further.
