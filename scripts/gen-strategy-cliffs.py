#!/usr/bin/env python3
"""
Generate a per-spirit "Strategy Cliffs" admonition callout to embed into each
chapter's Adversary Matchup Matrix section.

The callout flags the 2-4 per-adversary-level rule shifts most relevant to the
spirit, based on its power_summary profile (high Fear → fear-suppression
matters; high Offense → England L5 HP bump matters; etc.).

Usage:
    python3 scripts/gen-strategy-cliffs.py <slug>
    python3 scripts/gen-strategy-cliffs.py --all  # writes per-slug files
"""

from __future__ import annotations
import argparse, json, sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DRAFT_DIR = REPO / "data" / "references" / "draft-priority"
WIKI_DIR = REPO / "data" / "references" / "wiki"
SPIRITS_JSON = REPO / "data" / "spirits.json"
OUT_DIR = REPO / "data" / "references" / "strategy-cliffs"

# Catalog of known adversary/level cliffs + which spirit profiles they affect.
CLIFFS = [
    {
        "id": "england-l5-hp",
        "title": "England L5 — Buildings +1 HP",
        "body": "Towns become 3-HP (was 2), Cities become 4-HP (was 3). **Damage-only Powers dealing 2 or 3 may no longer kill a Town/City in one go.**",
        "affects": lambda ps: ps.get("offense", 0) >= 3 or ps.get("fear", 0) >= 3,
        "mitigation": "Stack damage from multiple plays or use downgrade Powers (Crops Wither, Tangled Trees) to soften before finishing.",
    },
    {
        "id": "england-l3-coastal-builds",
        "title": "England L3 — Coastal Lands build faster",
        "body": "England's L3 escalation adds an extra Build in coastal lands. **Ocean-adjacent spirits see compounded pressure on their home terrain.**",
        "affects": lambda ps: ps.get("offense", 0) >= 2,  # many spirits care
        "mitigation": "Front-load coastal defense or disruption before T3's first Ravage.",
    },
    {
        "id": "sweden-l2-fear-suppression",
        "title": "Sweden L2+ — Fear-card effects reduced",
        "body": "Sweden's escalation reduces the impact of Fear cards. **Spirits that win by riding Fear cards to Terror-level flips are meaningfully slower.**",
        "affects": lambda ps: ps.get("fear", 0) >= 4,
        "mitigation": "Shift from fear-rush to board-control: favor Damage/Push Majors over more Fear; accept Terror 2 flip ~2 rounds later.",
    },
    {
        "id": "russia-l3-dahan-pressure",
        "title": "Russia L3+ — Dahan under pressure + fear suppression",
        "body": "Russia's L3 escalation targets Dahan directly and suppresses Fear. **Spirits reliant on Dahan density (Shadows of the Dahan, Favors Called Due, Thunderspeaker synergies) lose a key engine.**",
        "affects": lambda ps: ps.get("fear", 0) >= 3,
        "mitigation": "Pre-empt Dahan loss with Defend-heavy Minors (Dahan/Village-fortify cards); lean on Push/Gather Majors to offset Fear deficit.",
    },
    {
        "id": "habsburg-mining-l5-scaling",
        "title": "Habsburg Mining L5+ — Explorer/Town scaling",
        "body": "Habsburg Mining L5+ adds extra Explorers and faster builds. **Aggressive fear-rush openers can get outpaced by raw Invader accumulation.**",
        "affects": lambda ps: ps.get("fear", 0) >= 3,
        "mitigation": "Favor Major Powers with mass destruction (Jungle Hungers, Cleansing Floods, etc.) over Minor-heavy drafts.",
    },
    {
        "id": "france-plantation-dahan-capture",
        "title": "France (Plantation) — Dahan capture threatens your Dahan engine",
        "body": "France's plantation rules convert Dahan to colonists, and Invaders occupy lands with Dahan. **Spirits whose innate/card math counts on Dahan density (Shadows-of-the-Dahan, Favors, Thunderspeaker) are downgraded.**",
        "affects": lambda ps: ps.get("fear", 0) >= 3 or ps.get("control", 0) >= 3,
        "mitigation": "Play Defend Powers on Dahan lands; accept loss of range-extension budget.",
    },
    {
        "id": "brandenburg-prussia-fear-cities",
        "title": "Brandenburg-Prussia — Cities drive Fear-per-kill (favorable swing)",
        "body": "BP's escalation puts Cities on the board early, and each destroyed City dumps Fear into the pool. **Damage-dealing spirits benefit from an inflated Fear curve; weaker spirits may struggle against pre-City pressure.**",
        "affects": lambda ps: ps.get("offense", 0) >= 3 or ps.get("fear", 0) >= 4,
        "mitigation": "Aim at City-dense lands with your highest-damage plays for outsized Fear returns.",
    },
]


def parse_int(s, default=0):
    if s is None:
        return default
    try:
        return int(s)
    except (ValueError, TypeError):
        return default


def render_cliffs(slug: str) -> str:
    dp_path = DRAFT_DIR / f"{slug}.json"
    if not dp_path.exists():
        raise FileNotFoundError(f"{dp_path} not found")
    dp = json.loads(dp_path.read_text())
    ps_raw = dp.get("power_summary") or {}
    ps = {k: parse_int(v) for k, v in ps_raw.items()}
    spirit_name = dp.get("spirit_name") or slug

    applicable = [c for c in CLIFFS if c["affects"](ps)]
    if not applicable:
        return ""

    lines = [
        f"### Strategy Cliffs — per-adversary-level shifts that change {spirit_name}'s math",
        "",
        f"```admonish warning title=\"Cliffs to watch\"",
        f"Not every adversary level is a linear scale-up — some levels flip specific rules that alter what your Powers accomplish. These are the cliffs most relevant to {spirit_name}'s profile (Fear {ps.get('fear','?')}, Offense {ps.get('offense','?')}, Control {ps.get('control','?')}, Defense {ps.get('defense','?')}, Utility {ps.get('utility','?')}).",
        "```",
        "",
    ]
    for c in applicable:
        lines.extend([
            f"#### {c['title']}",
            "",
            f"**What changes**: {c['body']}",
            "",
            f"**Mitigation for {spirit_name}**: {c['mitigation']}",
            "",
        ])
    return "\n".join(lines)


def main():
    p = argparse.ArgumentParser()
    p.add_argument("slug", nargs="?")
    p.add_argument("--all", action="store_true")
    args = p.parse_args()

    if args.all:
        OUT_DIR.mkdir(parents=True, exist_ok=True)
        reg = json.loads(SPIRITS_JSON.read_text())
        count = 0
        for entry in reg.get("spirits", []):
            slug = entry.get("slug")
            if not slug:
                continue
            try:
                section = render_cliffs(slug)
                if section:
                    (OUT_DIR / f"{slug}.md").write_text(section)
                    count += 1
            except FileNotFoundError:
                pass
        print(f"wrote {count} strategy-cliff sections to {OUT_DIR}/", file=sys.stderr)
    elif args.slug:
        print(render_cliffs(args.slug))
    else:
        p.error("specify a slug or --all")


if __name__ == "__main__":
    main()
