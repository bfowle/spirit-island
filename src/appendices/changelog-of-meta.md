# Meta Changelog

Per-chapter revision log. Dated by when the revision shipped, not by when the meta changed.

## v0.1 — 2026-04-19 — MVP

Initial publication.

**Authored**:
- `introduction.md`
- `how-to-use.md`
- `foundations/core-loop.md`
- `foundations/terminology.md`
- `fundamentals/tempo.md`
- `fundamentals/major-vs-minor.md`
- `fundamentals/presence-economy.md`
- `fundamentals/fear-track.md`
- `fundamentals/dahan.md`
- `spirits/index.md`
- `spirits/low/shadows-flicker-like-flame.md` (canonical Rei-format example)
- `social/index.md`
- `social/teaching-methods.md`
- `social/alpha-player-problem.md`
- `appendices/glossary.md`
- `appendices/sources.md`
- `appendices/claude-skills-guide.md`
- `appendices/changelog-of-meta.md` (this file)

**Infrastructure**:
- mdbook 0.4.52 + mdbook-admonish preprocessor
- Gruvbox theme vendored
- Data seeds: `spirits.json`, `adversaries.json`, `scenarios.json`, empty `playlog.csv`
- 3 MVP Claude skills: `si-daily-challenge`, `si-post-game`, `si-at-the-table`
- 3 skeleton skills: `si-combo-drill`, `si-matchup-drill`, `si-aspect-explorer`
- `scripts/install-skills.sh` for skill symlink management
- `scripts/make-stubs.sh` for chapter stub generation

**Pending (future milestones)**:
- M2: remaining base + B&C adversaries (England/Sweden/Brandenburg-Prussia full), remaining base-spirit priority chapters, B&C scenarios, `si-matchup-drill` logic.
- M3: Jagged Earth priority spirits (Fractured Days, Many Minds, Shroud, Vengeance, Stone, Shifting Memory), combos Part, `si-combo-drill` logic.
- M4: Nature Incarnate spirits, full aspects coverage, statistics Part, `si-aspect-explorer` logic.
- M5+: remaining 20+ spirits filled out, card reference appendix curated, progressive-disclosure index complete.

## v0.1.1 — 2026-04-19 — Opener format upgraded

- Revised `templates/SPIRIT_TEMPLATE.md` Opening Strategy section to require full 3–4 turn rehearsal blocks per variant (growth, cards played, presence, elements, E/CP state, milestone, pivot advice).
- Updated `spirits/low/shadows-flicker-like-flame.md` Opening A + B to the new format; added experimental Opening C stub.
- Captured format convention in `memory/feedback_opener_format.md` for future authoring.

## Process notes

- Each chapter update re-stamps its `Last revised: YYYY-MM-DD` footer.
- This changelog gets an entry for any substantive content change (not typo fixes).
- Chapters revised after playtest feedback note the trigger — e.g., "v0.3 — 2026-05-03 — Shadows chapter revised after Brett logged 8 games and tempo-row was consistently off for L5."
