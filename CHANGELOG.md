# Changelog

Infrastructure and tooling changes. For content (chapter) changes, see [`src/appendices/changelog-of-meta.md`](./src/appendices/changelog-of-meta.md).

## [0.1.0] — 2026-04-19 — MVP

### Added

- Initial mdbook scaffolding (9 Parts, 100+ chapters stubbed)
- 18 fully-authored chapters (see meta changelog)
- Gruvbox theme vendored
- mdbook-admonish preprocessor wired
- 3 MVP Claude skills (`si-daily-challenge`, `si-post-game`, `si-at-the-table`)
- 3 deferred-skill skeletons (`si-combo-drill`, `si-matchup-drill`, `si-aspect-explorer`)
- `scripts/install-skills.sh` — symlink skills to `~/.claude/skills/`
- `scripts/make-stubs.sh` — regenerate chapter stubs
- Templates for spirit, adversary, scenario, admonition style
- Data seeds: `spirits.json`, `adversaries.json`, `scenarios.json`, empty `playlog.csv`
- GitHub Actions workflow for Pages deploy (not yet live)
- Contributing guide + ethical sourcing rules

### Known limitations

- `mdbook-toc`, `mdbook-mermaid`, `mdbook-katex` preprocessors not enabled (upstream compatibility with mdbook 0.5 pending). Mermaid diagrams embed via `<pre class="mermaid">` + client-side JS.
- Most spirit/adversary/scenario chapters are stubs. See the meta changelog for the authoring roadmap.

### Toolchain versions

- mdbook 0.4.52
- mdbook-admonish 1.20.0
- Rust 1.93+
