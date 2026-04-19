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

## v0.1.3 — 2026-04-19 — Fundamentals + social chapters complete

All remaining stub chapters in Part II (Fundamentals) and Part VII (Playing with Others) fully authored.

**Fundamentals added (5)**:
- `fundamentals/terrain.md` — four terrains + coastal/inland + terrain-gated powers + spirits by preferred terrain.
- `fundamentals/adjacency-and-range.md` — adjacency rules, Range-N, edge cases (Shroud Mists, Finder vision), common misreads.
- `fundamentals/energy-curves.md` — per-spirit target income/spend tables, four curve archetypes, deviation cost.
- `fundamentals/card-draft-theory.md` — 4-offer evaluation, spirit-fit axes, multiplayer pooling, forget math.
- `fundamentals/blight-track.md` — cascade mechanics, pool management, blight-positive spirits, mitigation toolkit.

**Social added (7)**:
- `social/two-player.md` — sweet-spot coordination, pairing patterns, territory splits, disagreement handling.
- `social/three-and-four-player.md` — role assignments, 4-heroes problem, table-talk scaling.
- `social/mixed-skill-tables.md` — asymmetric spirit assignment (the reverse-intuition move), handicapping.
- `social/convention-and-meetup-norms.md` — venue-specific etiquette, honest time estimates, rules clarification protocols.
- `social/playing-with-strangers.md` — three archetypes (Rules-Focused, Vibes, Competitive), signal-reading, norm-setting.
- `social/post-game-debrief.md` — three-sentence debrief format, never-do list, debrief by audience.
- `social/teaching-anti-patterns.md` — 27 explicit don'ts with failure-mode rationale.

Both parts now fully authored. Part II and Part VII are complete.

## v0.1.2 — 2026-04-19 — Part VI fleshed out

Full authoring of all 7 Combos & Archetypes chapters:

- `combos/archetype-index.md` — landing page + archetype-to-spirit mapping + adversary fit + multiplayer coverage guidance.
- `combos/energy-denial.md` — deny invader actions via isolation/push/card-disruption; primary: Downpour, Trickster, Shroud.
- `combos/dahan-rush.md` — scale dahan into triple-duty (offense/defense/fear); primary: Thunderspeaker, Hearth-Vigil, Ferocious Warrior.
- `combos/fear-rush.md` — race to Terror 3 via innate + card fear; primary: Shadows, Bringer, Many Minds, Shroud.
- `combos/terrain-control.md` — move invaders to kill-zones or isolate lands; primary: Green, Lure, River, Finder.
- `combos/defend-and-outlast.md` — absorb early pressure for late Major engine; primary: Earth, Keeper, Stone.
- `combos/major-power-shopping.md` — aggressive Major drafting, Forget-heavy; primary: Earth, Keeper, Stone, Covets.

Each chapter follows the archetype-chapter spine defined in the index: Identity, Core Mechanics, Spirits That Embody It (primary + secondary), Execution Pattern (worked example), Strong/Weak Adversary Matchups, Multiplayer Synergies, Common Mistakes, Source Notes.

## v0.1.1 — 2026-04-19 — Opener format upgraded

- Revised `templates/SPIRIT_TEMPLATE.md` Opening Strategy section to require full 3–4 turn rehearsal blocks per variant (growth, cards played, presence, elements, E/CP state, milestone, pivot advice).
- Updated `spirits/low/shadows-flicker-like-flame.md` Opening A + B to the new format; added experimental Opening C stub.
- Captured format convention in `memory/feedback_opener_format.md` for future authoring.

## Process notes

- Each chapter update re-stamps its `Last revised: YYYY-MM-DD` footer.
- This changelog gets an entry for any substantive content change (not typo fixes).
- Chapters revised after playtest feedback note the trigger — e.g., "v0.3 — 2026-05-03 — Shadows chapter revised after Brett logged 8 games and tempo-row was consistently off for L5."
