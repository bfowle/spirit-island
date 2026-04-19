#!/usr/bin/env python3
"""
Score Minor + Major power cards for each spirit → draft-priority rankings.

Inputs:
    data/decks/minor.json       — full Minor pool (scripts/wiki-fetch.py deck minor)
    data/decks/major.json       — full Major pool
    data/references/wiki/<slug>.json — parsed spirit data (innate elements,
                                       thresholds, presence tracks, power_summary)
    data/spirits.json           — spirit registry

Outputs:
    data/references/draft-priority/<slug>.json — {minors_ranked, majors_ranked,
                                                  avoid, debug info}

Scoring dimensions (summed):
  1. Element alignment — sum of (card elements ∩ spirit innate elements),
     each threshold-element weighted by count × level-importance (L1=3, L2=2, L3=1).
  2. Cost match — 0/1E bonus for low-income spirits; penalty when cost
     exceeds spirit's mid-game energy estimate.
  3. Speed alignment — bonus when card speed matches spirit's primary innate speed
     (Fast innate ← wants Fast cards to feed it).
  4. Effect synergy — keyword matches on card text weighted by spirit's
     power_summary rating for that axis (Fear / Offense / Control / Defense / Utility).
  5. Anti-synergy — penalty + "avoid" flag when card conflicts with the spirit's
     reliance (e.g., Destroys-Dahan vs. Dahan-reliant spirits).

Usage:
    python3 scripts/draft-priority.py --spirit shadows-flicker-like-flame
    python3 scripts/draft-priority.py --all
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DECKS_DIR = REPO / "data" / "decks"
WIKI_DIR = REPO / "data" / "references" / "wiki"
OUTPUT_DIR = REPO / "data" / "references" / "draft-priority"
SPIRITS_JSON = REPO / "data" / "spirits.json"

ELEMENTS = ("sun", "moon", "fire", "air", "water", "earth", "plant", "animal")

# Effect-keyword groups as regex patterns matched against card text + threshold text.
# Each axis's score contribution = spirit's power_summary rating (1-5) × 0.3
# when ANY pattern in the axis appears. Low-rated axes (< 3) don't contribute.
# Patterns are designed to avoid false positives (e.g., "take no damage" shouldn't
# register as offense via bare "damage" substring).
EFFECT_KEYWORDS: dict[str, tuple[str, ...]] = {
    "fear": (r"\bfear\b",),
    "offense": (r"\bdestroy\b", r"\d+ damage", r"deals? \d+", r"\bkill"),
    "control": (r"\bpush\b", r"\bgather\b", r"\breplace\b", r"\bmove\b"),
    "defense": (r"\bdefend\b", r"take no damage", r"\bprevent", r"\bisolate\b"),
    "utility": (r"add \d+ presence", r"\brepeat\b", r"\breclaim\b", r"\+\d+ energy",
                r"card play", r"gain an energy", r"gain a power"),
}

# Anti-synergy rules: cards flagged for avoidance.
ANTI_SYNERGY_RULES = [
    {
        "pattern": re.compile(r"destroy(?:s)? all dahan|destroy(?:s)? \d+ dahan", re.IGNORECASE),
        "reason": "destroys Dahan",
        "severity": 3,
    },
    {
        "pattern": re.compile(r"destroy.*presence", re.IGNORECASE),
        "reason": "destroys Presence",
        "severity": 2,
    },
    {
        "pattern": re.compile(r"add \d+ blight", re.IGNORECASE),
        "reason": "adds Blight",
        "severity": 1,
    },
]


def parse_int(s: str | None, default: int = 0) -> int:
    if s is None:
        return default
    s = str(s).strip()
    try:
        return int(s)
    except ValueError:
        return default


def estimate_mid_game_energy(track: list[str]) -> float:
    """Average energy income over rounds 3-5, assuming one presence placed per turn.

    Track slots are tokens like 'energy0', 'energy1', 'energy3' etc. — the digit is
    the income once that slot is revealed.
    """
    values: list[int] = []
    for slot in track:
        m = re.match(r"energy(\d+)", slot)
        if m:
            values.append(int(m.group(1)))
    if not values:
        return 0.0
    if len(values) < 3:
        return float(values[-1])
    # T3 income ≈ values[2], T4 ≈ values[3], T5 ≈ values[4]
    window = values[2:5] if len(values) >= 5 else values[2:]
    return sum(window) / len(window)


def build_innate_element_weights(spirit: dict) -> dict[str, float]:
    """Sum each element's threshold demand, weighted by innate-level importance.

    L1 weight 3, L2 weight 2, L3 weight 1. Scaled by 0.3 overall. Multiple
    innates accumulate.
    """
    weights: dict[str, float] = {}
    for innate in spirit.get("innates", []):
        for i, thr in enumerate(innate.get("thresholds", [])):
            level_weight = max(1, 3 - i)  # L1=3, L2=2, L3=1, deeper=1
            for elem in ELEMENTS:
                count = parse_int(thr.get(elem), default=0)
                if count > 0:
                    weights[elem] = weights.get(elem, 0.0) + count * level_weight * 0.3
    return weights


def score_card(
    card: dict,
    spirit: dict,
    innate_weights: dict[str, float],
    psummary: dict,
    mid_energy: float,
    is_major: bool,
) -> tuple[float, list[str]]:
    score = 0.0
    reasons: list[str] = []

    # Element alignment
    card_elems = {e.strip().lower() for e in card.get("elements") or []}
    elem_points = 0.0
    aligned: list[str] = []
    for e in card_elems:
        w = innate_weights.get(e, 0.0)
        if w > 0:
            elem_points += w
            aligned.append(e)
    if elem_points > 0:
        score += elem_points
        aligned.sort()
        reasons.append(f"elements {'+'.join(aligned)} → {elem_points:.1f}")

    # Cost match
    cost = parse_int(card.get("cost"))
    cost_ceiling = mid_energy + (2.0 if is_major else 0.0)
    if cost == 0:
        score += 1.0
        reasons.append("0-cost (always affordable)")
    elif cost <= 1:
        score += 0.5
    if cost > cost_ceiling:
        penalty = (cost - cost_ceiling) * 0.5
        score -= penalty
        reasons.append(f"cost {cost} > mid-game {mid_energy:.1f} (−{penalty:.1f})")

    # Speed alignment to spirit's primary innate
    innates = spirit.get("innates") or []
    innate_speed = (innates[0].get("speed") or "").lower() if innates else ""
    card_speed = (card.get("speed") or "").lower()
    if innate_speed and card_speed and innate_speed == card_speed:
        score += 0.5
        reasons.append(f"{card_speed.title()} speed matches innate")

    # Effect synergy via regex matching × power_summary rating
    combined_text = ((card.get("text") or "") + " " + (card.get("threshold") or "")).lower()
    for axis, patterns in EFFECT_KEYWORDS.items():
        rating = parse_int(psummary.get(axis))
        if rating < 3:
            continue
        weight = (rating - 2) * 0.3
        for pat in patterns:
            m = re.search(pat, combined_text)
            if m:
                score += weight
                reasons.append(f"{m.group(0)!r} → {axis}={rating} (+{weight:.1f})")
                break

    return score, reasons


def score_anti_synergy(card: dict) -> list[dict]:
    hits: list[dict] = []
    text = (card.get("text") or "") + " " + (card.get("threshold") or "")
    for rule in ANTI_SYNERGY_RULES:
        if rule["pattern"].search(text):
            hits.append({"reason": rule["reason"], "severity": rule["severity"]})
    return hits


def rank_spirit(spirit_slug: str, minors_data: dict, majors_data: dict) -> dict:
    wiki_path = WIKI_DIR / f"{spirit_slug}.json"
    if not wiki_path.exists():
        raise FileNotFoundError(f"no wiki data at {wiki_path}")
    spirit = json.loads(wiki_path.read_text())

    innate_weights = build_innate_element_weights(spirit)
    mid_energy = estimate_mid_game_energy(spirit.get("presence_energy_track") or [])
    psummary = spirit.get("power_summary") or {}

    def score_deck(cards: list[dict], is_major: bool) -> list[dict]:
        # Dedupe by card name — the Wiki keeps base + reprint variants of each
        # card as separate pages, but for draft-priority they're the same card.
        seen: set[str] = set()
        unique_cards: list[dict] = []
        for c in cards:
            name = c.get("name") or ""
            if name in seen:
                continue
            seen.add(name)
            unique_cards.append(c)
        ranked = []
        for c in unique_cards:
            base_score, reasons = score_card(c, spirit, innate_weights, psummary, mid_energy, is_major)
            anti = score_anti_synergy(c)
            if anti:
                pen = sum(a["severity"] for a in anti)
                base_score -= pen
                reasons.append("anti-synergy: " + ", ".join(a["reason"] for a in anti))
            ranked.append({
                "name": c.get("name"),
                "score": round(base_score, 2),
                "cost": parse_int(c.get("cost")),
                "speed": c.get("speed"),
                "range": c.get("range"),
                "target": c.get("target"),
                "elements": c.get("elements") or [],
                "text": c.get("text"),
                "threshold": c.get("threshold"),
                "expansion": c.get("expansion"),
                "reasons": reasons,
                "anti_synergy": [a["reason"] for a in anti] or None,
            })
        ranked.sort(key=lambda x: -x["score"])
        return ranked

    minors_ranked = score_deck(minors_data.get("cards") or [], is_major=False)
    majors_ranked = score_deck(majors_data.get("cards") or [], is_major=True)

    avoid = []
    for bucket in (minors_ranked, majors_ranked):
        for c in bucket:
            if c.get("anti_synergy"):
                avoid.append({
                    "name": c["name"],
                    "reasons": c["anti_synergy"],
                    "score": c["score"],
                })

    return {
        "spirit": spirit_slug,
        "spirit_name": spirit.get("name"),
        "power_summary": psummary,
        "innate_element_weights": {k: round(v, 2) for k, v in sorted(innate_weights.items(), key=lambda x: -x[1])},
        "mid_game_energy_estimate": round(mid_energy, 2),
        "minors_ranked": minors_ranked,
        "majors_ranked": majors_ranked,
        "avoid": avoid,
    }


def print_top(result: dict, n: int = 5) -> None:
    print(f"\n{result['spirit_name']} ({result['spirit']})")
    print(f"  innate elements: {result['innate_element_weights']}")
    print(f"  mid-game energy est: {result['mid_game_energy_estimate']}")
    print(f"\n  Top {n} Minors:")
    for c in result["minors_ranked"][:n]:
        elems = "+".join(c["elements"]) or "—"
        print(f"    {c['score']:5.2f}  {c['name']:38}  {c['speed'] or '?':5} {c['cost']}E  {elems}")
    print(f"\n  Top {n} Majors:")
    for c in result["majors_ranked"][:n]:
        elems = "+".join(c["elements"]) or "—"
        print(f"    {c['score']:5.2f}  {c['name']:38}  {c['speed'] or '?':5} {c['cost']}E  {elems}")
    if result["avoid"]:
        print(f"\n  Avoid ({len(result['avoid'])} flagged):")
        for a in result["avoid"][:3]:
            print(f"    {a['name']}: {', '.join(a['reasons'])}")


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--spirit", help="Score one spirit by slug")
    p.add_argument("--all", action="store_true", help="Score every spirit in spirits.json with wiki data")
    p.add_argument("--show-top", type=int, default=5, help="How many top ranks to print (default 5)")
    args = p.parse_args()

    if not args.spirit and not args.all:
        p.error("specify --spirit <slug> or --all")

    minors_data = json.loads((DECKS_DIR / "minor.json").read_text())
    majors_data = json.loads((DECKS_DIR / "major.json").read_text())

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    if args.spirit:
        result = rank_spirit(args.spirit, minors_data, majors_data)
        out = OUTPUT_DIR / f"{args.spirit}.json"
        out.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
        print(f"Wrote {out}", file=sys.stderr)
        print_top(result, n=args.show_top)
        return

    # --all
    registry = json.loads(SPIRITS_JSON.read_text())
    written = 0
    skipped = 0
    for entry in registry.get("spirits", []):
        slug = entry.get("slug")
        if not slug:
            continue
        if not (WIKI_DIR / f"{slug}.json").exists():
            skipped += 1
            continue
        try:
            result = rank_spirit(slug, minors_data, majors_data)
            out = OUTPUT_DIR / f"{slug}.json"
            out.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
            written += 1
            top3 = ", ".join(c["name"] for c in result["minors_ranked"][:3])
            print(f"{slug}: minors top3 → {top3}")
        except Exception as e:
            print(f"error on {slug}: {e}", file=sys.stderr)
            skipped += 1
    print(f"\nDone: {written} written, {skipped} skipped (missing wiki data or error).", file=sys.stderr)


if __name__ == "__main__":
    main()
