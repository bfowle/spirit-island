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

## v0.2.2 — 2026-04-19 — Accuracy audit completed (all 13 spirits corrected)

**All 13 authored spirit chapters surgically corrected** against Wiki-verified mechanics data (saved to `data/references/spirit-mechanics.md`):

- Shadows, Green, Bringer, Earth (severe) — innate + unique + special-rule rewrites.
- Keeper, Serpent, Finder, Downpour (severe) — all Unique card names were wrong; rewrites substantial.
- Thunderspeaker, Ocean, Lightning, River, Fangs (moderate-to-severe) — missing innates/special rules added; wrong Unique names replaced.

Every chapter now:
- References `data/references/spirit-mechanics.md` as its authoritative source.
- Carries `[VERIFY]` markers on fields the Wiki scrape didn't fully surface (card costs/speeds/elements, exact effect wording, aspect confirmation).
- Includes an Expansion Sensitivity section (template addition).
- Has per-level strategy-cliff callouts where rules change strategy fundamentally (e.g., England L5 +1 HP buildings).

**Verification punch-list** saved to `data/references/verification-punchlist.md` — Brett can work through with physical copy + expansions to resolve `[VERIFY]` items.

**Process going forward**: `feedback_never_hallucinate_mechanics.md` rule in effect; no new spirit chapters written without Wiki-verified data first.

## v0.2.1 — 2026-04-19 — Accuracy audit initiated (M2 chapters under revision)

**Critical correctness issue identified**: Shadows chapter (and likely others) contain mechanical hallucinations — innates/cards/mechanics mis-attributed. Examples from Shadows:
- "Dark and Tangled Wood" labeled as innate (it's a Horizons card).
- "Favors Called Due" labeled as innate (it's a Unique card).
- Framed around "strife" (not a base-Shadows mechanic).

**Remediation in progress**:
- Research agent fetching authoritative spirit mechanics from Spirit Island Wiki for all 13 authored spirit chapters.
- Each chapter audited against Wiki data; inaccurate sections rewritten.
- Going forward: `feedback_never_hallucinate_mechanics.md` rule — no mechanics written from memory; Wiki-verify before authoring.

**New structural rules (from Brett, 2026-04-19)**:
- `feedback_expansion_split_strategy.md` — main guide assumes all expansions; per-spirit/per-adversary-level chapters flag expansion-sensitive advice and specific adversary-level strategy cliffs (e.g., "River plays well vs England until ENG5 when buildings get +1 HP").
- Mermaid diagrams for expansion-dependency and strategy-cliff decisions.

Re-issues of v0.2.x will follow as chapters are corrected. v0.2.0's "24 chapters authored" count is accurate, but the *content* of those chapters needs correctness review before use.

## v0.2.0 — 2026-04-19 — M2 complete

Full authoring of 24 chapters; Brett priority-1 + teach-anchor + B&C/promo priority spirits + base adversaries + base scenarios + cleanup + aspects.

**Spirit chapters added (12)**:

Base priority-1:
- `spirits/low/thunderspeaker.md` (Dahan-Rush canonical; 3 latentoctopus openings)
- `spirits/low/ocean-hungry-grasp.md` (Coastal drowning + territorial partner needs)
- `spirits/low/a-spread-of-rampant-green.md` (Major-shopping + Terrain Control)
- `spirits/low/bringer-of-dreams-and-nightmares.md` (Fear-only win path)

Base teach-anchors:
- `spirits/low/lightning-swift-strike.md` (Direct Damage, Fast-Energy)
- `spirits/low/river-surges-in-sunlight.md` (Terrain Control; Bounty-loop)
- `spirits/low/vital-strength-of-the-earth.md` (Defend & Outlast canonical)

B&C + promo priority:
- `spirits/moderate/sharp-fangs-behind-the-leaves.md` (4 latentoctopus openings)
- `spirits/moderate/keeper-of-the-forbidden-wilds.md` (Major Power Shopping)
- `spirits/moderate/serpent-slumbering-beneath-the-island.md` (Late-Game Juggernaut)
- `spirits/moderate/finder-of-paths-unseen.md` (Mobility + Terrain Control)
- `spirits/moderate/downpour-drenches-the-world.md` (Energy Denial via complementarity)

**Adversary chapters added (3)**:
- `adversaries/index.md` (landing page + comparison matrix)
- `adversaries/brandenburg-prussia.md` (late-explosion adversary)
- `adversaries/sweden.md` (fear-suppression adversary)

**Scenario chapters added (5)**:
- `scenarios/index.md` (landing page + scenario-adversary compatibility)
- `scenarios/blitz.md` (time compression)
- `scenarios/guard-the-isles-heart.md` (inland objective)
- `scenarios/rituals-of-terror.md` (fear race)
- `scenarios/dahan-insurrection.md` (dahan scoring)

**Cleanup (4)**:
- `foundations/reading-a-card.md` (card anatomy + reading order)
- `reader-self-assessment.md` (progressive-disclosure self-routing)
- `appendices/if-youve-only-played.md` (gap-based reading paths)
- `spirits/aspects.md` (31 aspects + when-to-suggest-in-a-group per Brett)

**Table audit + backfill (5 fundamentals chapters)**:
- Expanded per-spirit tables to all 39 spirits in `major-vs-minor`, `energy-curves`, `dahan`, `blight-track`, `terrain`.

**Pronoun fix**: all spirits now use they/them per game canon.

**Memory added**: `feedback_all_spirits_in_tables.md`, `feedback_spirit_pronouns.md`.

Remaining stubs: 25 (down from 37).

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
