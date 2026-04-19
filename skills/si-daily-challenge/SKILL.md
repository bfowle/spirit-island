---
name: si-daily-challenge
description: Use when Brett asks for "today's Spirit Island challenge" or says "/si-daily-challenge". Generates a structured daily drill — spirit × adversary × level × scenario × learning goal — based on his playlog history in ~/repos/spirit-island/data/playlog.csv. Anti-repeats spirits from the last 5 sessions and progresses difficulty via a 2-wins-at-level rule.
user-invocable: true
---

# si-daily-challenge — Daily Spirit Island Drill

Generates a single daily challenge card that Brett plays next, pulling his learning priorities and playlog to rotate spirits + adversaries intelligently.

## Data Sources

1. `~/repos/spirit-island/data/playlog.csv` — Brett's logged games. Schema: `date,player_count,spirits,aspects,adversary,level,scenario,board,result,rounds,fear_level_end,blight_end,notes`. May be empty (no games logged yet).
2. `~/repos/spirit-island/data/spirits.json` — 42 spirits. Key fields: `slug`, `name`, `complexity`, `draft_bias`, `archetypes`, `primary_elements`, `brett_priority` (bool), `chapter_status` (`full`/`stub`).
3. `~/repos/spirit-island/data/adversaries.json` — 8 adversaries with their difficulty-level ladders.
4. `~/repos/spirit-island/data/scenarios.json` — 14 scenarios with `difficulty_mod`.

## Selection Algorithm

### 1. Load playlog
- Read CSV; if empty, treat as "first session" and pick a beginner-friendly priority spirit.

### 2. Anti-repeat spirit filter
- Exclude spirits that appear in the last 5 log entries.
- Prefer spirits with `brett_priority == true`.
- Prefer spirits with `chapter_status == "full"` so Brett has a chapter to read first.
- If all priority spirits are in the recent-5 cooldown, broaden to all spirits.

### 3. Adversary rotation
- Count adversary frequencies in last 10 log entries.
- Pick the least-recently-played adversary from his four go-to's (England, Sweden, Brandenburg-Prussia, and one "stretch" like Russia or France).
- For Brett's first 20 logged games, stay in base + B&C adversaries unless he explicitly asked for a stretch.

### 4. Level progression
- Compute Brett's rolling win rate at his current level against this adversary.
- If he has 2 wins in a row at the current level with this adversary → suggest level up.
- If he has 2 losses in a row → suggest level down.
- Default starting level: 1.

### 5. Scenario choice
- Most challenges are scenario-free (simpler).
- Roll scenario ~20% of the time, biased toward easier difficulty_mods (Blitz, Second Wave, Varied Terrains) until he's experienced with the adversary.

### 6. Learning goal
- Pick one topic to focus on, rotating across chapters:
  - "Tempo audit — track per-turn energy/CP banked vs spent"
  - "Major vs. Minor draft — follow the archetype bias for this spirit"
  - "Fear generation pace — track fear per turn vs the Terror-2 target"
  - "Opening adherence — follow Opening A from the spirit chapter; don't improvise T1–T3"
  - "Card priority — grade each power card you see in offerings using the spirit chapter's grades"
  - "Adversary pressure — identify the cliff turn before it hits"
  - "Anti-alpha (if multiplayer) — no mid-turn suggestions for your partner"
- Rotate goals; don't repeat the same goal twice in a row.

## Output Format

Emit a single compact markdown block:

```
## Today's Challenge — {YYYY-MM-DD}

**Spirit**: {name} ({complexity}, {draft_bias})
**Adversary**: {adversary} Level {N}
**Scenario**: {name or "none"}
**Player count**: 1 (true solo) / 1 multi-handed / other
**Learning goal**: {goal}

### Read first (15 min)
- Spirit chapter: [{slug}](../src/{chapter_path})
- {Fundamentals chapter linked to goal}
- Adversary chapter: [{adversary}](../src/adversaries/{slug}.md)

### Turn-1 commit
Before you start, write on paper: "By T3 I will have {concrete milestone — pulled from the spirit chapter's Tempo Profile}."

### After the game
Run `si-post-game` to log; tomorrow's challenge shifts based on results.
```

## Anti-alpha note

When outputting, never prescribe specific plays. Say "Opening A" not "play Offering of Fear and Flame T1 then Minor T2." The chapter has the details; the challenge should point Brett at the chapter.

## Edge cases

- **Empty playlog**: pick Shadows (full chapter) + England L1 + no scenario + goal = "Tempo audit."
- **Brett asks for a specific spirit/adversary**: honor it; skip the rotation. Still emit the full challenge card.
- **Brett says "I want harder"**: bump level by 1 beyond the algorithm's output.
- **Brett says "easier"**: bump level down or swap scenario for none.

## Interaction after output

After showing the challenge, ask: "Ready for this, or want me to pick a different {spirit/adversary/goal}?" — one-line prompt only.
