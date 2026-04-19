# Contributing

## Tone

- Second person ("you"); present tense.
- Number before nuance: "58% at L6, sample ~400" before "though CI is wide."
- Terse. Every section earns its words.

## Chapter structure

Every spirit chapter follows [`templates/SPIRIT_TEMPLATE.md`](./templates/SPIRIT_TEMPLATE.md). Derived from Rei's BGG format (Many Minds, Shroud, Vengeance). Don't diverge without a reason.

Every adversary chapter follows [`templates/ADVERSARY_TEMPLATE.md`](./templates/ADVERSARY_TEMPLATE.md).

Every scenario chapter follows [`templates/SCENARIO_TEMPLATE.md`](./templates/SCENARIO_TEMPLATE.md).

## Admonitions

Only the six classes in [`templates/ADMONITION_STYLE.md`](./templates/ADMONITION_STYLE.md). Resist scope creep.

## Citation & ethical sourcing

1. **Summarize, don't copy.** No verbatim paragraphs from external sources.
2. **Always credit.** Every pattern claim has a Source Note admonition or inline link.
3. **Link, don't mirror.** Never host copies of community tools' data.
4. **Primary over secondary.** Querki + designer statements first; community interpretations second.
5. **No podcast transcription > ~30 words.** Summarize + link to episode + timestamp.
6. **Discord policy.** Cite channel + general theme; do not quote named users without public-consent. Prefer linking the BGG thread where the same idea was posted.

## Stats claims

Every stat carries:
- Source name + URL
- Sample size (n)
- Date of snapshot
- Caveat (at minimum, reference to [Digital vs Tabletop](./src/statistics/digital-vs-tabletop.md))

## PR format

- One spirit / adversary / scenario chapter per PR ideally.
- If chapter grew from a playtest revision, reference the playlog row in `data/playlog.csv`.
- Tag the PR description with the `Last revised: YYYY-MM-DD` date you set in the chapter footer.

## Build checks

Before submitting:

```bash
mdbook build       # must succeed, stderr empty
mdbook serve       # spot-check rendering
```

Chapter has all required sections (for spirit chapters): At a Glance, Framing, Core Mechanics, Strategic Principles, Opening, Element/Aspect, Card Priorities, Adversary Matchup Matrix, Board Position, Game-Phase, Synergies, Common Mistakes, Tempo Profile, Major vs. Minor, Stat Snapshot, Sources.

## Playtest-the-teaching-chapter

The teaching chapters are unique — they must be playtested, not just written.

Process for a new/revised social chapter:
1. Write to MVP.
2. Actually apply it — teach a real person, run the alpha-player script at a table, etc.
3. Revise based on what diverged.
4. Apply again with a different audience.
5. Revise again.

Only then does the chapter merge.

## Data schema changes

If you change `data/playlog.csv` or `data/spirits.json` schema, also update:
- The [Claude Skills Companion](./src/appendices/claude-skills-guide.md) chapter's schema section.
- The three MVP skills' `SKILL.md` files (they reference specific field names).
- Existing playlog rows should be migrated, not dropped.

## Versioning

- SemVer-ish for pedagogical milestones.
- `CHANGELOG.md` at root tracks infrastructure changes.
- `src/appendices/changelog-of-meta.md` tracks content/chapter changes.
