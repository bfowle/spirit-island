---
name: si-post-game
description: Use when Brett says "SI post-game", "log my game", "I just finished a Spirit Island game", or similar. Normalizes his short game summary, appends a row to ~/repos/spirit-island/data/playlog.csv, and surfaces 1–2 learning points by diffing his game against the chapter's common-mistakes and tempo-profile sections.
user-invocable: true
---

# si-post-game — Log Game + Surface Learning

Normalize Brett's shorthand game report, append to playlog, and give him 1–2 concrete takeaways to feed tomorrow's challenge.

## Data Sources

1. `~/repos/spirit-island/data/playlog.csv` — append here.
2. `~/repos/spirit-island/src/spirits/{tier}/{slug}.md` — the spirit chapter for this game (for common-mistakes + tempo-profile lookup).
3. `~/repos/spirit-island/src/adversaries/{slug}.md` — adversary chapter (if full-written).

## Input Normalization

Brett's input will look something like:
> "Just played Shadows vs Sweden L3. Won on T8, fear 12, blight 4. Almost lost T5 because I forgot to defend."

Parse into:
- date (today if not given)
- player_count (1 unless he says otherwise)
- spirits (`shadows-flicker-like-flame`)
- aspects (none unless he mentions)
- adversary (`sweden`)
- level (3)
- scenario (none unless he mentions)
- board (unknown unless he mentions A/B/C/D/E/F)
- result (`win` / `loss`)
- rounds (8)
- fear_level_end (12 — this is the fear pool total, not Terror level)
- blight_end (4)
- notes (the free-text commentary verbatim)

Ask for anything missing **only if it's load-bearing** (result, spirit, adversary, level). Don't pester for board letter or fear pool if he didn't mention them.

## Append to playlog

Append one row to `~/repos/spirit-island/data/playlog.csv`. Use the schema in the CSV header.

## Surface 1–2 learning points

1. Read the spirit chapter's **Common Mistakes** section.
2. Match Brett's notes + game state against the mistakes list. For example:
   - If he mentions "forgot to defend" and the chapter has a "forgot-to-defend" mistake → surface that.
   - If `rounds` > the chapter's expected game length and he lost → surface "tempo issues" with a pointer to the Tempo Profile table.
   - If `blight_end` is near cascade and he lost → surface blight-management mistake.
3. Limit to 2 points. More = noise.

## Output Format

```
Logged: {date} {spirits} vs {adversary} L{level} — {result} T{rounds}

## What likely cost/made the game

1. **{Learning point 1}** — {one sentence; link to chapter section}
2. **{Learning point 2}** — {optional; only if there's a second signal}

## Rolling stats
- Games logged: {N}
- Win rate overall: {X%}
- Win rate vs {this adversary} at L{level}: {X% (n=Y)}

## Next session suggestion
{One line — either "repeat to cement the lesson" or "rotate to {spirit/adversary} via `si-daily-challenge`"}
```

## Rules

- Never editorialize beyond what the chapter says. If the chapter doesn't call out a mistake, don't invent one.
- Include the note field verbatim in the CSV row so future sessions can reference Brett's own framing.
- If Brett's note contradicts what you'd surface, ask once: "You said X; the chapter flags Y — which felt like the actual issue?"
- CSV appending: use `>>` append, not overwrite. Preserve the header row.

## Edge cases

- **Multi-spirit game**: spirits field gets comma-separated slugs (`shadows-flicker-like-flame,thunderspeaker`).
- **Aspect used**: aspects field gets the aspect name.
- **Multi-session game (rare)**: treat each session as one row.
- **Chapter is a stub**: skip the mistakes lookup; output "chapter not yet written — just logged." Still surface tempo/blight observations from general fundamentals.
