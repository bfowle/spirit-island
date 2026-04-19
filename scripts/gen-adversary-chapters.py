#!/usr/bin/env python3
"""
Generate full adversary chapter markdown from data/references/adversaries/<slug>.json.

Sections per chapter:
- At-a-glance table (expansion, pressure shape, difficulty levels)
- Level-by-level breakdown with is_cliff callouts
- Spirit affinity analysis (which spirit profiles match this adversary)
- Anti-spirit analysis (which profiles struggle)
- Escalation mermaid diagram
- Strategy cliffs admonition block pulled from data/references/strategy-cliffs/
  where relevant.

Usage:
    python3 scripts/gen-adversary-chapters.py
"""

from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
ADV_DIR = REPO / "data" / "references" / "adversaries"
OUT_DIR = REPO / "src" / "adversaries"
REGISTRY = REPO / "data" / "adversaries.json"


# Pair each adversary slug → which spirit power-summary profile is strong vs. weak here.
# Profile keys: fear (high-Fear spirits), offense (damage), control (push/gather),
# defense (defend-heavy), utility (energy/reclaim), dahan (Dahan-reliant).
ADVERSARY_AFFINITY = {
    "england": {
        "strong_vs": {
            "fear": "England is fear-friendly through L3 — Terror tracks flip comfortably against slow Town→City builds.",
            "offense": "Damage plays work well against 2-HP Towns (before L5 HP bump).",
        },
        "weak_vs": {
            "defense": "Defend-heavy plays eventually get out-paced by the sheer Town-to-City scaling by L5+.",
            "control": "Pure control without damage stalls; England eventually has too many Cities to push.",
        },
        "cliff_levels": [3, 4, 5],
    },
    "brandenburg-prussia": {
        "strong_vs": {
            "fear": "BP's Cities arrive early and fear-per-kill scaling is explosive. Best fear-rush matchup.",
            "offense": "Cities give great Damage + Fear returns when destroyed.",
        },
        "weak_vs": {
            "utility": "Pure energy-banking can get outraced by the early City pressure.",
        },
        "cliff_levels": [2, 4, 6],
    },
    "sweden": {
        "strong_vs": {
            "offense": "Damage scales with Invader count and Sweden's coastal front-load gives targets.",
            "control": "Push/Gather disrupts Sweden's coastal-Build strategy.",
        },
        "weak_vs": {
            "fear": "Sweden L2+ caps Fear-card effects; fear-rush spirits stall.",
            "dahan": "Sweden's Homesteaders rule (L3+) converts Dahan to settlers.",
        },
        "cliff_levels": [2, 3, 5],
    },
    "france-plantation-colony": {
        "strong_vs": {
            "defense": "Defending Dahan lands directly counters plantation-formation.",
            "offense": "Damage + presence denial prevents plantation tokens from accumulating.",
        },
        "weak_vs": {
            "dahan": "France's Slavery rule captures Dahan — any spirit reliant on Dahan-outnumbers breaks.",
            "fear": "Fear-rush works but is slower vs. plantation-dense boards.",
        },
        "cliff_levels": [2, 3, 5],
    },
    "russia": {
        "strong_vs": {
            "offense": "Damage handles Russia's Settler waves efficiently.",
            "defense": "Protecting Dahan lands is specifically rewarded.",
        },
        "weak_vs": {
            "fear": "Russia's L3+ fear suppression caps fear-rush value.",
            "dahan": "Russia's Settler Attacks target Dahan; Dahan-reliant spirits struggle.",
        },
        "cliff_levels": [3, 4],
    },
    "scotland": {
        "strong_vs": {
            "offense": "Scotland is damage-friendly — fewer fear traps than other L3+ adversaries.",
            "fear": "Fear still works well; no hard fear-suppression.",
        },
        "weak_vs": {
            "dahan": "Scotland's Clearances at L3 can hurt Dahan-dependent engines.",
        },
        "cliff_levels": [3, 6],
    },
    "habsburg-mining-expedition": {
        "strong_vs": {
            "offense": "Damage keeps up with Mining's explorer/town scaling.",
            "utility": "Energy-banking for big Major plays combats Mining's late-game spike.",
        },
        "weak_vs": {
            "fear": "Mining outpaces fear-rush at L5+; the raw invader accumulation wins the race.",
        },
        "cliff_levels": [3, 5, 7],
    },
    "habsburg-livestock-colony": {
        "strong_vs": {
            "fear": "Livestock scales slowly; fear-rush gets time to flip Terror.",
            "control": "Push/Gather disrupts ranches forming on specific lands.",
        },
        "weak_vs": {
            "defense": "Livestock doesn't punish Defense-heavy play much; those spirits still function, just overkill.",
        },
        "cliff_levels": [3, 5, 8],
    },
}


def render_level_table(adv: dict) -> str:
    lines = [
        "| Level | Name | Escalation | Cliff? | Notes |",
        "|-------|------|------------|:------:|-------|",
    ]
    for lvl in adv.get("levels", []):
        cliff = "⚠️" if lvl.get("is_cliff") else ""
        notes = lvl.get("notes") or ""
        esc = (lvl.get("escalation") or "").replace("\n", " ")
        lines.append(f"| L{lvl.get('level', '?')} | **{lvl.get('name','?')}** | {esc} | {cliff} | {notes} |")
    return "\n".join(lines)


def render_cliff_callouts(adv: dict) -> str:
    cliffs = [l for l in adv.get("levels", []) if l.get("is_cliff")]
    if not cliffs:
        return ""
    out = ["## Strategy Cliffs", ""]
    for c in cliffs:
        out.extend([
            f"```admonish warning title=\"L{c['level']} — {c['name']}\"",
            f"**What changes**: {c.get('escalation', '')}",
            "",
            f"**Impact**: {c.get('notes') or 'See strategic breakdown above.'}",
            "```",
            "",
        ])
    return "\n".join(out)


def render_affinity(slug: str, adv_name: str) -> str:
    affin = ADVERSARY_AFFINITY.get(slug, {})
    if not affin:
        return ""
    lines = [f"## Spirit Affinity vs. {adv_name}", "", "Which spirit archetypes have an easier or harder time against this adversary, based on power-summary profile matching."]
    lines.append("")
    lines.append("### Strong against this adversary")
    lines.append("")
    for profile, reason in (affin.get("strong_vs") or {}).items():
        lines.append(f"- **{profile.title()}-focused spirits** — {reason}")
    lines.append("")
    lines.append("### Struggles against this adversary")
    lines.append("")
    for profile, reason in (affin.get("weak_vs") or {}).items():
        lines.append(f"- **{profile.title()}-focused spirits** — {reason}")
    lines.append("")
    return "\n".join(lines)


def render_mermaid(adv: dict) -> str:
    lines = ["## Escalation Timeline", "", "<pre class=\"mermaid\">", "timeline"]
    lines.append(f"    title {adv.get('name')} difficulty curve")
    for lvl in adv.get("levels", []):
        cliff = " ⚠️" if lvl.get("is_cliff") else ""
        name = lvl.get("name", "?").replace(":", " -")
        lines.append(f"    L{lvl.get('level', '?')}{cliff} : {name}")
    lines.append("</pre>")
    return "\n".join(lines)


def render_chapter(slug: str, adv: dict) -> str:
    name = adv.get("name", slug)
    exp = adv.get("expansion", "?")
    pressure = adv.get("pressure_shape", "(not described)")
    lines = [
        f"# {name}",
        "",
        "```admonish success title=\"Auto-generated from adversary panel data\"",
        f"Escalation levels + cliff classifications derived from `data/references/adversaries/{slug}.json`. Rows marked `[VERIFY physical panel]` need cross-checking against the physical adversary card.",
        "```",
        "",
        "```admonish abstract title=\"At a Glance\"",
        f"| Field | Value |",
        f"|-------|-------|",
        f"| Expansion | {exp} |",
        f"| Pressure shape | {pressure} |",
        f"| Difficulty levels | {', '.join('L' + str(l.get('level','?')) for l in adv.get('levels', []))} |",
        "```",
        "",
        "## Overview",
        "",
        f"**{name}** is a {pressure.lower()} adversary introduced in the {exp} expansion. The level-by-level breakdown below captures each escalation tier; cliff levels (flagged ⚠️) are where your strategic approach must change fundamentally, not just scale up.",
        "",
        "## Level-by-Level Breakdown",
        "",
        render_level_table(adv),
        "",
        render_cliff_callouts(adv),
        render_affinity(slug, name),
        render_mermaid(adv),
        "",
        "## How to pick a spirit vs. this adversary",
        "",
        "1. Read the **Strong-against** list above and find spirits whose Power Summary fits (from [Spirits Overview & Index](../spirits/index.md)).",
        "2. Cross-check the spirit's chapter's `Adversary Matchup Matrix` for the letter grade at your chosen level.",
        "3. For cliff levels (⚠️), read the Strategy Cliffs section in the spirit's chapter to confirm the spirit's mitigation plan works.",
        "",
        "## Related resources",
        "",
        "- [Statistics: Deck Expansion Impact](../statistics/deck-expansion-impact.md) — how this adversary's expansion affects the Fear/Event/Blight decks.",
        "- [Expansion Dilution Claims](../statistics/expansion-dilution-claims.md) — variance analysis per expansion.",
        f"- Spirit Island Wiki: [{name}](https://spiritislandwiki.com/index.php?title={name.replace(' ', '_')}).",
        "",
    ]
    return "\n".join(lines)


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    wrote = 0
    for slug_path in sorted(ADV_DIR.glob("*.json")):
        slug = slug_path.stem
        adv = json.loads(slug_path.read_text())
        out = OUT_DIR / f"{slug}.json"  # not used; reference
        content = render_chapter(slug, adv)
        (OUT_DIR / f"{slug}.md").write_text(content)
        wrote += 1
    print(f"Wrote {wrote} adversary chapters to {OUT_DIR}/")


if __name__ == "__main__":
    main()
