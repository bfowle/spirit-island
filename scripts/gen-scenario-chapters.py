#!/usr/bin/env python3
"""
Generate scenario chapters from data/references/scenarios/<slug>.json.

Usage:
    python3 scripts/gen-scenario-chapters.py
"""

from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SCEN_DIR = REPO / "data" / "references" / "scenarios"
OUT_DIR = REPO / "src" / "scenarios"


# Spirit profile → scenario fit hints, for a Generic Suitability table.
SCENARIO_HINTS = {
    "blitz": {
        "best_for": "Fear-rush spirits. 6-round fuse matches fear-rush tempo.",
        "worst_for": "Energy-bank spirits (Stone's Unyielding Defiance, Starlight). They never get to spend.",
    },
    "guard-the-isles-heart": {
        "best_for": "Control + Dahan spirits (Thunderspeaker, Hearth-Vigil) that can defend one critical land.",
        "worst_for": "Spread-heavy spirits; concentrated defense isn't their strength.",
    },
    "rituals-of-terror": {
        "best_for": "High-Fear spirits — the ritual often mirrors fear-pool mechanics.",
        "worst_for": "[VERIFY]",
    },
    "dahan-insurrection": {
        "best_for": "Dahan-reliant spirits (Shadows, Thunderspeaker, Hearth-Vigil).",
        "worst_for": "Anti-Dahan spirits (Heart of the Wildfire, Volcano Looming High).",
    },
    "second-wave": {
        "best_for": "Energy-bank + Major-shopping spirits for sustained long-game play.",
        "worst_for": "Fear-rush openers that run out of steam past the first wave.",
    },
    "powers-long-forgotten": {
        "best_for": "Minor-draft-density spirits (Grinning Trickster, Shifting Memory of Ages).",
        "worst_for": "[VERIFY]",
    },
    "ward-the-shores": {
        "best_for": "Coastal-damage + push spirits (Ocean's Hungry Grasp naturally).",
        "worst_for": "Inland-focused spirits.",
    },
    "rituals-of-destroying-flame": {
        "best_for": "Damage-heavy spirits that can keep pace with fire rituals.",
        "worst_for": "Defend-heavy spirits.",
    },
    "despicable-theft": {
        "best_for": "Fast-innate spirits that pre-empt theft triggers.",
        "worst_for": "[VERIFY]",
    },
    "elemental-invocation": {
        "best_for": "Multi-element innate spirits (Wandering Voice, Fractured Days).",
        "worst_for": "Mono-element spirits (Rampant Green, Heart of the Wildfire).",
    },
    "a-diversity-of-spirits": {
        "best_for": "(Spirit pick is constrained; read the scenario card). Rewards cross-archetype coverage.",
        "worst_for": "Homogeneous team compositions.",
    },
    "the-great-river": {
        "best_for": "River Surges in Sunlight (thematic); coastal/water spirits.",
        "worst_for": "[VERIFY]",
    },
    "varied-terrains": {
        "best_for": "Terrain-flexible spirits.",
        "worst_for": "Terrain-specific spirits (A Spread of Rampant Green favors jungles; Rising Heat favors sands).",
    },
    "destiny-unfolds": {
        "best_for": "[VERIFY]",
        "worst_for": "[VERIFY]",
    },
}


def render_chapter(slug: str, s: dict) -> str:
    name = s.get("name", slug)
    expansion = s.get("expansion", "?")
    difficulty_delta = s.get("difficulty_delta", "?")
    hints = SCENARIO_HINTS.get(slug, {})

    lines = [
        f"# {name}",
        "",
        "```admonish abstract title=\"At a Glance\"",
        f"| Field | Value |",
        f"|-------|-------|",
        f"| Expansion | {expansion} |",
        f"| Difficulty delta | {difficulty_delta} |",
        f"| Scenario slug | `{slug}` |",
        "```",
        "",
        "## Setup modifications",
        "",
        s.get("setup_modifications") or "(See scenario card.)",
        "",
        "## Win condition",
        "",
        s.get("win_condition_modifications") or "Standard (Terror 3 + all Invaders destroyed).",
        "",
        "## Loss condition",
        "",
        s.get("loss_condition_modifications") or "Standard (Blight cap reached + no blighted-land remaining to flip).",
        "",
    ]
    if s.get("notes"):
        lines.extend(["## Notes", "", s["notes"], ""])

    lines.extend([
        "## Spirit fit",
        "",
        f"**Best for**: {hints.get('best_for', '[VERIFY play]')}",
        "",
        f"**Worst for**: {hints.get('worst_for', '[VERIFY play]')}",
        "",
        "## Cross-references",
        "",
        "- [Spirits Overview](../spirits/index.md)",
        "- [Adversaries Overview](../adversaries/index.md)",
        f"- Spirit Island Wiki: [{name}](https://spiritislandwiki.com/index.php?title={name.replace(' ', '_')}).",
        "",
    ])
    return "\n".join(lines)


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    wrote = 0
    for slug_path in sorted(SCEN_DIR.glob("*.json")):
        slug = slug_path.stem
        s = json.loads(slug_path.read_text())
        content = render_chapter(slug, s)
        out = OUT_DIR / f"{slug}.md"
        # Don't overwrite Blitz / Guard which may have hand-authored content.
        if out.exists() and out.stat().st_size > 1000:
            print(f"skip {slug}: has substantive content")
            continue
        out.write_text(content)
        wrote += 1
        print(f"wrote {out}")
    print(f"\nwrote {wrote} scenario chapters")


if __name__ == "__main__":
    main()
