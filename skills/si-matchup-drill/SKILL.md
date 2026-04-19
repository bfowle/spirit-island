---
name: si-matchup-drill
description: Use when Brett asks for a "Spirit Island matchup drill" or says "drill me on {adversary}". Picks spirits across the grade spectrum vs. a given adversary and quiz-mode tests pattern recognition. NOT YET IMPLEMENTED — skeleton only; full logic lands in M2.
user-invocable: true
---

# si-matchup-drill — Adversary-Focused Drill

**Status**: Skeleton. Full logic lands in Milestone 2 alongside the adversary chapters.

Intended behavior:

1. Take an adversary (or ask Brett to name one).
2. Select spirits across the matchup-matrix grade spectrum (A+, B, D) — *not* just favorable matchups. Forcing adaptive play.
3. Cycle through levels 0 → 6 across sessions.
4. Quiz mode: before play, ask 3 pattern-recognition questions pulled from the adversary chapter ("what round does Stage II start?", "what's the worst escalation on L5?"). Grade answers after.

Until M2 ships: if invoked, fall back to pointing at England L3 vs Shadows as the matchup with the fullest chapter coverage today.
