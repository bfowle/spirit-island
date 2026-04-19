#!/usr/bin/env python3
"""
Generate a spirit chapter's Card Priority Ratings section from draft-priority
data + Wiki beginner-deck bundle.

Output structure per spirit:
  ## Card Priority Ratings
  ### Uniques
  ### Top Minor Draft Picks (from full pool)
  ### Top Major Draft Picks (from full pool)
  ### HoSI Beginner Deck Bundle
  ### Cards to Avoid

Usage:
    python3 scripts/gen-card-priority.py shadows-flicker-like-flame
    python3 scripts/gen-card-priority.py --all

Writes to stdout by default. With --all, writes per-spirit markdown files to
data/references/card-priority-sections/<slug>.md for splicing into chapters.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DRAFT_DIR = REPO / "data" / "references" / "draft-priority"
WIKI_DIR = REPO / "data" / "references" / "wiki"
SPIRITS_JSON = REPO / "data" / "spirits.json"
OUT_DIR = REPO / "data" / "references" / "card-priority-sections"

N_MINOR_TOP = 10
N_MAJOR_TOP = 5


def fmt_elements(elems: list[str]) -> str:
    return ", ".join(e.title() for e in (elems or []))


def fmt_cost(cost: int) -> str:
    return f"{cost}"


def fmt_speed(speed: str | None) -> str:
    return (speed or "—").title() if speed else "—"


def truncate(text: str | None, n: int = 100) -> str:
    if not text:
        return ""
    t = text.replace("\n", " ").strip()
    return t if len(t) <= n else t[: n - 1] + "…"


def render_top_table(ranked: list[dict], n: int, header: str) -> str:
    rows = ranked[:n]
    if not rows:
        return f"\n*No cards matched for {header}.*\n"
    lines = [
        f"| # | Card | Cost | Speed | Elements | Effect (truncated) | Why this pick |",
        f"|---|------|------|-------|----------|--------------------|---------------|",
    ]
    for i, c in enumerate(rows, 1):
        reasons = c.get("reasons") or []
        top_reason = reasons[0] if reasons else ""
        lines.append(
            "| {i} | **{name}** | {cost} | {speed} | {elements} | {text} | {reason} |".format(
                i=i,
                name=(c.get("name") or "").replace("|", "\\|"),
                cost=fmt_cost(c.get("cost", 0)),
                speed=fmt_speed(c.get("speed")),
                elements=fmt_elements(c.get("elements") or []),
                text=truncate(c.get("text"), 90).replace("|", "\\|"),
                reason=top_reason.replace("|", "\\|"),
            )
        )
    return "\n".join(lines)


def render_avoid_table(avoid: list[dict]) -> str:
    if not avoid:
        return "\n*No anti-synergy cards flagged for this spirit.*\n"
    # Dedupe by name (the avoid list may contain both Minor + Major copies)
    seen = set()
    rows = []
    for a in avoid:
        if a["name"] in seen:
            continue
        seen.add(a["name"])
        rows.append(a)
    lines = ["| Card | Reason(s) |", "|------|-----------|"]
    for a in rows[:15]:
        lines.append(f"| **{a['name']}** | {', '.join(a['reasons'])} |")
    return "\n".join(lines)


def render_hosi_bundle(suggested: list[dict] | None) -> str:
    if not suggested:
        return "\n*No HoSI beginner-deck bundle for this spirit.*\n"
    lines = ["| Card | Type | Cost | Speed | Elements | Effect (truncated) |",
             "|------|------|------|-------|----------|--------------------|"]
    for c in suggested:
        if "error" in c:
            lines.append(f"| {c.get('name')} | — | — | — | — | (fetch error: {c.get('error')}) |")
            continue
        lines.append(
            "| **{name}** | {ct} | {cost} | {speed} | {elements} | {text} |".format(
                name=(c.get("name") or "").replace("|", "\\|"),
                ct=(c.get("card_type") or "—"),
                cost=fmt_cost(int(c.get("cost") or "0")),
                speed=fmt_speed(c.get("speed")),
                elements=fmt_elements(c.get("elements") or []),
                text=truncate(c.get("text"), 90).replace("|", "\\|"),
            )
        )
    return "\n".join(lines)


def render_section(slug: str) -> str:
    dp_path = DRAFT_DIR / f"{slug}.json"
    wiki_path = WIKI_DIR / f"{slug}.json"
    if not dp_path.exists():
        raise FileNotFoundError(f"{dp_path} not found — run scripts/draft-priority.py --all")
    if not wiki_path.exists():
        raise FileNotFoundError(f"{wiki_path} not found — run scripts/wiki-fetch.py batch-all")

    dp = json.loads(dp_path.read_text())
    wiki = json.loads(wiki_path.read_text())

    spirit_name = dp.get("spirit_name") or wiki.get("name") or slug
    innate_weights = dp.get("innate_element_weights") or {}
    mid_energy = dp.get("mid_game_energy_estimate", 0)
    psummary = dp.get("power_summary") or {}

    primary_elems = ", ".join(
        f"**{k.title()}** (wt {v})" for k, v in list(innate_weights.items())[:3]
    )

    # Build sections
    top_minors = render_top_table(dp.get("minors_ranked") or [], N_MINOR_TOP, "Minors")
    top_majors = render_top_table(dp.get("majors_ranked") or [], N_MAJOR_TOP, "Majors")
    hosi = render_hosi_bundle(wiki.get("suggested_card_details") or [])
    avoid = render_avoid_table(dp.get("avoid") or [])

    unique_names = [u["name"] for u in (wiki.get("unique_card_details") or []) if u.get("name")]

    lines = [
        f"## Card Priority Ratings",
        f"",
        f"```admonish abstract title=\"Full-pool draft analysis\"",
        f"Scored across all {114 if True else '—'} Minor + {98 if True else '—'} Major cards in the full deck (Base + B&C + JE + NI), weighted by {spirit_name}'s innate element demands, mid-game energy estimate, primary-innate speed, and power-summary ratings. See [data/references/draft-priority/{slug}.json](https://github.com/brettfowle/spirit-island/blob/main/data/references/draft-priority/{slug}.json) for full scoring + reasons.",
        f"",
        f"- **Primary elements (innate-weighted)**: {primary_elems or '—'}",
        f"- **Mid-game energy estimate (T3–T5 avg)**: {mid_energy}E",
        f"- **Power summary**: Offense {psummary.get('offense','?')} · Control {psummary.get('control','?')} · Fear {psummary.get('fear','?')} · Defense {psummary.get('defense','?')} · Utility {psummary.get('utility','?')}",
        f"```",
        f"",
        f"### Uniques",
        f"",
    ]
    if unique_names:
        lines.append("The spirit's own " + str(len(unique_names)) + " Unique Power cards (always in hand; always A-tier by default — see Uniques section above for full text):")
        lines.append("")
        for n in unique_names:
            lines.append(f"- **{n}**")
        lines.append("")
    else:
        lines.append("*No Unique cards listed.*")
        lines.append("")

    lines.extend([
        f"### Top {N_MINOR_TOP} Minor Draft Picks (from full pool)",
        f"",
        top_minors,
        f"",
        f"### Top {N_MAJOR_TOP} Major Draft Picks (from full pool)",
        f"",
        top_majors,
        f"",
        f"### HoSI Beginner Deck Bundle — for reference only",
        f"",
        f"```admonish note title=\"Not a draft-priority list\"",
        f"These are the cards shipped with {spirit_name} in the **Horizons of Spirit Island** beginner bundle — a curated onboarding subset, **not an optimized draft list**. The picks above (Top Minor / Major) draw from the full expansion pool. Keep this table for historical reference or when playing with a HoSI-only card pool.",
        f"```",
        f"",
        hosi,
        f"",
        f"### Cards to Avoid (anti-synergy flagged)",
        f"",
        avoid,
        f"",
    ])
    return "\n".join(lines)


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("slug", nargs="?", help="Spirit slug")
    p.add_argument("--all", action="store_true", help="Generate for every spirit")
    p.add_argument("--write", action="store_true", help="Write to data/references/card-priority-sections/<slug>.md")
    args = p.parse_args()

    if not args.slug and not args.all:
        p.error("specify a spirit slug or --all")

    if args.all:
        OUT_DIR.mkdir(parents=True, exist_ok=True)
        registry = json.loads(SPIRITS_JSON.read_text())
        count = 0
        for entry in registry.get("spirits", []):
            slug = entry.get("slug")
            if not slug:
                continue
            try:
                section = render_section(slug)
                out = OUT_DIR / f"{slug}.md"
                out.write_text(section)
                count += 1
            except FileNotFoundError as e:
                print(f"skip {slug}: {e}", file=sys.stderr)
        print(f"Wrote {count} sections to {OUT_DIR}/", file=sys.stderr)
        return

    section = render_section(args.slug)
    if args.write:
        OUT_DIR.mkdir(parents=True, exist_ok=True)
        out = OUT_DIR / f"{args.slug}.md"
        out.write_text(section)
        print(f"Wrote {out}", file=sys.stderr)
    else:
        print(section)


if __name__ == "__main__":
    main()
