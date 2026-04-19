#!/usr/bin/env python3
"""
Generate a spirit chapter markdown file from the Wiki-parsed JSON data.

The generated chapter has mechanical sections filled directly from the JSON
(innates, Uniques, special rules, growth, presence tracks, suggested cards).
Strategic-framing sections use light templates with [VERIFY] markers the
author will enhance over time.

Usage:
    python3 scripts/generate-spirit-chapter.py \\
        --json data/references/wiki/shadows-flicker-like-flame.json \\
        --output src/spirits/low/shadows-flicker-like-flame.md

    # Batch-generate for all spirits in spirits.json
    python3 scripts/generate-spirit-chapter.py --batch

The script is designed to be re-run safely: previously-authored strategic
content should be preserved by editing after generation, not by this script.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


TIER_MAP = {
    "low": "low",
    "moderate": "moderate",
    "high": "high",
    "very high": "very-high",
}


def elements_list(elements: list[str]) -> str:
    """Format element list for display."""
    return ", ".join(e.title() for e in elements) if elements else "—"


def thresholds_table(thresholds: list[dict]) -> str:
    """Render innate thresholds as a markdown table."""
    if not thresholds:
        return "_(no thresholds listed in Wiki)_"

    lines = ["| Level | Thresholds | Effect |", "|-------|------------|--------|"]
    for i, thr in enumerate(thresholds, start=1):
        parts = []
        for elem in ("sun", "moon", "fire", "air", "water", "earth", "plant", "animal"):
            if elem in thr:
                parts.append(f"{thr[elem]} {elem.title()}")
        threshold_text = " + ".join(parts) if parts else "—"
        effect = thr.get("effect", "—") or "—"
        lines.append(f"| {i} | {threshold_text} | {effect} |")
    return "\n".join(lines)


def growth_row(g: dict) -> str:
    """Render one growth option as a table row."""
    effects = []
    for key in ("first", "second", "third"):
        v = g.get(key)
        if v:
            effects.append(f"{key}={v}")
    return f"| G{g['index']} | {', '.join(effects)} |"


def render_unique_card(card: dict) -> str:
    """Render a Unique card's block."""
    if "error" in card:
        return f"- **{card.get('name', '?')}** — `[VERIFY]` (Wiki fetch error: {card['error']})\n"
    lines = [
        f"#### {card.get('name', '?')}",
        "",
        f"- **{card.get('cost', '?')} Energy · {card.get('speed', '?')} · Range {card.get('range', '?')} · {card.get('target', '?')} · {elements_list(card.get('elements', []))}**",
        f"- *{card.get('text', '?')}*",
        "",
    ]
    if card.get("threshold") and card["threshold"].lower() != "no threshold":
        lines.append(f"- **Threshold**: {card['threshold']}")
        lines.append("")
    return "\n".join(lines)


def render_suggested_card(card: dict) -> str:
    """Render one suggested card row."""
    if "error" in card:
        return f"| {card.get('name', '?')} | — | — | — | — | — | _Wiki error_ |"
    return (
        f"| **{card.get('name', '?')}** "
        f"| {card.get('cost', '?')} "
        f"| {card.get('speed', '?')} "
        f"| {card.get('range', '?')} "
        f"| {card.get('target', '?')} "
        f"| {elements_list(card.get('elements', []))} "
        f"| {card.get('text', '?')} |"
    )


def power_summary_display(ps: dict) -> str:
    if not ps:
        return "_(not provided on Wiki)_"
    keys = ("offense", "control", "fear", "defense", "utility")
    parts = [f"{k.title()} {ps[k]}" for k in keys if k in ps]
    return " · ".join(parts)


def derive_primary_elements(spirit: dict) -> str:
    """Heuristic: primary element is whichever appears most in innate thresholds + unique cards."""
    counter: dict[str, int] = {}
    for innate in spirit.get("innates", []):
        for thr in innate.get("thresholds", []):
            for elem in ("sun", "moon", "fire", "air", "water", "earth", "plant", "animal"):
                if elem in thr:
                    counter[elem] = counter.get(elem, 0) + 1
    for card in spirit.get("unique_card_details", []):
        for elem in card.get("elements", []):
            counter[elem.lower()] = counter.get(elem.lower(), 0) + 1
    if not counter:
        return "_(unknown)_"
    sorted_elems = sorted(counter.items(), key=lambda kv: -kv[1])
    return ", ".join(e.title() for e, _ in sorted_elems[:4])


def render_chapter(spirit: dict, aspects_note: str = "") -> str:
    """Produce the full chapter markdown."""
    name = spirit["name"]
    complexity = spirit.get("complexity", "?").lower()
    expansion = spirit.get("expansion", "?")
    growths = spirit.get("growths", [])
    growth_type = spirit.get("growth_type", "?")
    power_summary = spirit.get("power_summary", {})
    special = spirit.get("special_rules") or "_(no special rule)_"
    innates = spirit.get("innates", [])
    unique_details = spirit.get("unique_card_details", [])
    suggested = spirit.get("suggested_card_details", [])
    playstyle = spirit.get("playstyle", "")
    setup = spirit.get("setup", "")
    pe = spirit.get("presence_energy_track", [])
    pc = spirit.get("presence_cardplay_track", [])

    primary_elements = derive_primary_elements(spirit)

    # Growth table
    growth_table_lines = ["| Growth | Effects |", "|--------|---------|"]
    for g in growths:
        growth_table_lines.append(growth_row(g))
    growth_table = "\n".join(growth_table_lines) if growths else "_(not surfaced by Wiki template)_"

    # Innates
    innates_sections = []
    for innate in innates:
        innates_sections.append(
            f"### Innate: {innate.get('name', '?')}\n\n"
            f"- **Speed**: {innate.get('speed', '?')} · **Range**: {innate.get('range', '?')}"
            f"{(' (optionally from a ' + innate['option'] + ' site)') if innate.get('option') else ''}"
            f" · **Target**: {innate.get('target', '?')}\n\n"
            f"{thresholds_table(innate.get('thresholds', []))}\n"
        )
    innates_block = "\n\n".join(innates_sections) if innates_sections else "_(no innates listed)_"

    # Uniques
    uniques_block = "\n".join(render_unique_card(c) for c in unique_details)

    # Suggested cards split minor / major
    suggested_minors = [c for c in suggested if not _is_err(c) and c.get("card_type", "").lower() == "minor"]
    suggested_majors = [c for c in suggested if not _is_err(c) and c.get("card_type", "").lower() == "major"]
    suggested_other = [c for c in suggested if _is_err(c) or c.get("card_type", "").lower() not in ("minor", "major")]

    def table_for(cards: list[dict]) -> str:
        if not cards:
            return "_(none in Wiki's suggested list)_"
        header = "| Card | Cost | Speed | Range | Target | Elements | Effect |\n|------|------|-------|-------|--------|----------|--------|"
        rows = [render_suggested_card(c) for c in cards]
        return header + "\n" + "\n".join(rows)

    return f"""# {name}

```admonish success title="Mechanics Wiki-verified"
Card data, innate thresholds, special rules, growth options, presence track, and suggested-draft cards below were parsed deterministically from the Spirit Island Wiki via `scripts/wiki-fetch.py` (MediaWiki API → raw wikitext → template field extraction — no LLM summarization). Remaining `[VERIFY]` items: Play Difficulty (not in Wiki spirit template; spirit panel only), **aspect mechanics** (aspect-page parser pending), live mindwanderer stats, and board ratings (play data).
```

```admonish abstract title="At a Glance"
| Field                 | Value                                              |
|-----------------------|----------------------------------------------------|
| Expansion             | {expansion}                                        |
| Complexity            | {complexity.title()}                               |
| Play Difficulty       | `[VERIFY from spirit panel]`                       |
| Growth type           | "{growth_type}" — see Growth Options below         |
| Power summary (1–5)   | {power_summary_display(power_summary)}             |
| Primary Elements      | {primary_elements} (derived from innates + uniques)|
| Aspects               | {aspects_note or '`[VERIFY from physical aspect panels]`'} |
```

## Spirit Overview — Framing

**Wiki-printed playstyle note**:

> {playstyle if playstyle else '_(no playstyle text on Wiki spirit page)_'}

Strategic framing `[VERIFY: enhance with play experience]`.

## Starting Setup

> {setup if setup else '_(no setup text on Wiki spirit page)_'}

## Growth Options ({growth_type})

{growth_table}

**Growth token reference** (Wiki shorthand):
- `reclaim` — Reclaim all discarded Power Cards.
- `gain1p` / `gain2p` — Gain 1 or 2 Power Cards (Minor).
- `addpresence1` / `addpresence2` / `addpresence3` — Add 1 Presence from track, Range N.
- `energy1` / `energy2` / `energy3` — +1/+2/+3 Energy.
- `card1` / `card2` — +1/+2 Card Plays this turn.
- (Other tokens documented on [Wiki Spirit template reference](https://spiritislandwiki.com/) pages.)

## Presence Tracks

As Presence leaves each track, these values are revealed:

- **Energy track**: {", ".join(pe) if pe else "_(unknown)_"}
- **Card-play track**: {", ".join(pc) if pc else "_(unknown)_"}

## Core Mechanics & Special Rules

### Special Rule

{special}

{innates_block}

## Unique Cards (all, Wiki-verified)

{uniques_block}

## Suggested Draft Cards (Wiki-recommended)

### Minor Powers

{table_for(suggested_minors)}

### Major Powers

{table_for(suggested_majors)}

{("### Other (error or unclassified)\\n\\n" + table_for(suggested_other)) if suggested_other else ""}

## Key Strategic Principles

`[VERIFY and enhance]` — strategic principles should be derived from Wiki-verified mechanics above.

1. Use the Special Rule to its fullest (see above for exact text).
2. Element thresholds drive innate firing — see the innate tables above.
3. Suggested draft cards are Wiki-recommended; pattern-match to your matchup.

## Opening Strategy

`[VERIFY: needs play data]` — opening variants should be rehearsed turn-by-turn per the [SPIRIT_TEMPLATE.md](../../../templates/SPIRIT_TEMPLATE.md) opener format.

## Card Priority Ratings

**Uniques**: see above, all 4 cards are starting-deck and usually all grade A-tier for the spirit's intended playstyle.

**Suggested Minors + Majors**: see tables above. Wiki's suggestions reflect community-recommended drafts.

`[VERIFY: ratings per matchup pending]`.

## Adversary Matchup Matrix

`[VERIFY all grades]` — template only; fill in per-adversary notes after play.

| Adversary            | L0 | L3 | L5 | L6 | Notes `[VERIFY]`     |
|----------------------|----|----|----|----|----------------------|
| England              | ?  | ?  | ?  | ?  |                      |
| Brandenburg-Prussia  | ?  | ?  | ?  | ?  |                      |
| Sweden               | ?  | ?  | ?  | ?  |                      |
| France (Plantation)  | ?  | ?  | ?  | ?  |                      |
| Habsburg Mining      | ?  | ?  | ?  | ?  |                      |
| Russia               | ?  | ?  | ?  | ?  |                      |
| Scotland             | ?  | ?  | ?  | ?  |                      |
| Habsburg Livestock   | ?  | ?  | ?  | ?  |                      |

## Board / Map Configuration

`[VERIFY via play]` — base boards A–D, Jagged Earth E–H, and thematic ratings pending per-spirit play experience.

## Game-Phase Strategy

`[VERIFY: needs play data]`.

## Synergy Partners (Multiplayer)

`[VERIFY: needs multi-spirit play data]` — archetype-based hints from [Archetype Index](../../combos/archetype-index.md) are the starting point.

## Common Mistakes

`[VERIFY: collect from play]`.

## Tempo Profile

`[VERIFY: per-round targets need playtest]`.

## Expansion Sensitivity

- **Base only**: core Uniques + Innate + Special Rule functional if expansion = Base.
- **+ Branch & Claw**: events + blight deck introduce variance.
- **+ Jagged Earth**: Major/Minor pool deepens.
- **+ Nature Incarnate**: additional aspects may unlock; check the aspect column above.

Per-expansion specifics `[VERIFY]`.

## Stat Snapshot

```admonish note title="Stat Insight `[VERIFY from mindwanderer]`"
Pending re-scrape of mindwanderer current data. Historical directional figures unavailable in this template draft.
```

## Source Notes

```admonish abstract title="Sources"
- **Authoritative mechanics** (this chapter): `data/references/wiki/{slug(name)}.json` — parsed via `scripts/wiki-fetch.py`.
- Spirit Island Wiki — [{name}](https://spiritislandwiki.com/index.php?title={name.replace(' ', '_')}).
- Cross-reference: [Archetype Index](../../combos/archetype-index.md).
- Related skill: [si-wiki-fetch](../../../skills/si-wiki-fetch/SKILL.md).
```

---

*Chapter draft generated from Wiki data {_today()}. Strategic prose needs enhancement from play experience. See [`templates/SPIRIT_TEMPLATE.md`](../../../templates/SPIRIT_TEMPLATE.md) for the full Rei-format spine.*
"""


def slug(name: str) -> str:
    return name.lower().replace("'", "").replace(" ", "-").replace(",", "")


def _is_err(c: dict) -> bool:
    return "error" in c


def _today() -> str:
    import datetime

    return datetime.date.today().isoformat()


def load_aspects_from_memory() -> dict:
    """Return a dict of spirit-name → aspect markdown string, based on Brett's physical-copy
    verifications (hardcoded below; update as Brett confirms more spirits)."""
    return {
        "Shadows Flicker Like Flame": "**B&C**: Amorphous, Foreboding · **JE**: Madness, Reach · **NI**: Dark Fire",
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", help="Path to a single spirit JSON")
    ap.add_argument("--output", help="Path to write the generated chapter")
    ap.add_argument("--batch", action="store_true", help="Generate chapters for all spirits in data/spirits.json")
    ap.add_argument("--dry-run", action="store_true", help="Print chapter to stdout instead of writing")
    args = ap.parse_args()

    aspects_map = load_aspects_from_memory()

    if args.batch:
        registry = json.loads(Path("data/spirits.json").read_text())
        for entry in registry["spirits"]:
            slug_id = entry["slug"]
            tier = entry.get("complexity") or "moderate"
            tier_dir = TIER_MAP.get(tier.lower(), "moderate")
            out_path = Path(f"src/spirits/{tier_dir}/{slug_id}.md")
            json_path = Path(f"data/references/wiki/{slug_id}.json")
            if not json_path.exists():
                print(f"skip {slug_id}: no JSON", file=sys.stderr)
                continue
            try:
                spirit = json.loads(json_path.read_text())
                aspects = aspects_map.get(spirit["name"], "")
                md = render_chapter(spirit, aspects_note=aspects)
                if args.dry_run:
                    print(f"--- {out_path} ---")
                    print(md)
                else:
                    out_path.parent.mkdir(parents=True, exist_ok=True)
                    out_path.write_text(md)
                    print(f"wrote {out_path}", file=sys.stderr)
            except Exception as e:
                print(f"error {slug_id}: {e}", file=sys.stderr)
        return

    if not args.json:
        ap.error("--json required unless --batch")
    spirit = json.loads(Path(args.json).read_text())
    aspects = aspects_map.get(spirit["name"], "")
    md = render_chapter(spirit, aspects_note=aspects)
    if args.dry_run or not args.output:
        print(md)
    else:
        Path(args.output).parent.mkdir(parents=True, exist_ok=True)
        Path(args.output).write_text(md)
        print(f"wrote {args.output}", file=sys.stderr)


if __name__ == "__main__":
    main()
