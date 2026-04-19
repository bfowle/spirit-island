#!/usr/bin/env python3
"""
Per-expansion statistical breakdown of every card pool in Spirit Island.

For each of the 5 decks (Minor, Major, Fear, Event, Blight) and each of 5
expansion-combo pool sizes (base, +B&C, +JE, +NI, all), computes:

- pool size
- element distribution (Power cards only) with % of pool
- cost distribution (Power cards only)
- speed distribution (Fast/Slow/Fast-or-Slow)
- fear-effect amount distribution (count of "N Fear" instructions in text)
- token-addition frequency per token type
  (Badlands, Beasts, Disease, Strife, Wilds, Plantation, Isolate)
- effect-keyword frequency
  (destroy, damage, push, gather, defend, add blight, remove blight, repeat, reclaim)
- invader-favoring vs spirit-favoring classifier for Event cards
- Blight severity scoring

Writes one summary-per-expansion-combo JSON to data/references/deck-stats/
plus a combined all-combos summary.

Usage:
    python3 scripts/deck-stats.py
    python3 scripts/deck-stats.py --combo base        # just one
    python3 scripts/deck-stats.py --print-summary     # short stdout report
"""

from __future__ import annotations
import argparse
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DECKS_DIR = REPO / "data" / "decks"
OUT_DIR = REPO / "data" / "references" / "deck-stats"

# Map Wiki `set` field substrings → normalized expansion keys.
EXPANSION_MATCHERS: list[tuple[str, str]] = [
    ("horizons of spirit island", "hosi"),          # HoSI: beginner subset
    ("spirit island", "base"),                       # base game (check last - many cards include other sets too)
    ("branch and claw", "bnc"),
    ("branch & claw", "bnc"),
    ("jagged earth", "je"),
    ("nature incarnate", "ni"),
    ("promo pack 2", "promo2"),
    ("feather and flame", "promo2"),
    ("promo pack 1", "promo1"),
    ("apocrypha", "apocrypha"),
]

# Ordered from most-common → rarely-appearing tokens to give each a clear regex.
TOKEN_PATTERNS: dict[str, re.Pattern] = {
    "badlands": re.compile(r"\bbadlands?\b", re.IGNORECASE),
    "beasts": re.compile(r"\bbeasts?\b", re.IGNORECASE),
    "disease": re.compile(r"\bdisease\b", re.IGNORECASE),
    "strife": re.compile(r"\bstrife\b", re.IGNORECASE),
    "wilds": re.compile(r"\bwilds?\b", re.IGNORECASE),
    "plantation": re.compile(r"\bplantation\b", re.IGNORECASE),
    "isolate": re.compile(r"\bisolate[ds]?\b", re.IGNORECASE),
    "fortress": re.compile(r"\bfortress(?:es)?\b", re.IGNORECASE),
}

EFFECT_KEYWORDS: dict[str, re.Pattern] = {
    "destroy": re.compile(r"\bdestroy\w*\b", re.IGNORECASE),
    "damage": re.compile(r"\b\d+\s+damage\b", re.IGNORECASE),
    "push": re.compile(r"\bpush\w*\b", re.IGNORECASE),
    "gather": re.compile(r"\bgather\w*\b", re.IGNORECASE),
    "defend": re.compile(r"\bdefend\b", re.IGNORECASE),
    "add_blight": re.compile(r"\badd \d* ?blight\b", re.IGNORECASE),
    "remove_blight": re.compile(r"\bremove \d* ?blight\b", re.IGNORECASE),
    "repeat": re.compile(r"\brepeat this\b", re.IGNORECASE),
    "reclaim": re.compile(r"\breclaim\w*\b", re.IGNORECASE),
    "replace": re.compile(r"\breplace\b", re.IGNORECASE),
}

# N Fear regex for counting fear-generation
FEAR_AMOUNT_RX = re.compile(r"(\d+)\s+fear\b", re.IGNORECASE)

# Invader-favoring / spirit-favoring classifier heuristics for Event cards.
# Each matcher adds a score bucket.
INVADER_FAVORING = [
    re.compile(r"\badd \d+ (?:Explorer|Town|City|invader)", re.IGNORECASE),
    re.compile(r"invader(?:s)? perform", re.IGNORECASE),
    re.compile(r"extra build", re.IGNORECASE),
    re.compile(r"coastal (?:lands? )?build", re.IGNORECASE),
    re.compile(r"(?:extra|additional) ravage", re.IGNORECASE),
    re.compile(r"add 1 blight", re.IGNORECASE),
    re.compile(r"destroy (?:\d+\s+)?dahan", re.IGNORECASE),
    re.compile(r"replace 1 dahan", re.IGNORECASE),
]
SPIRIT_FAVORING = [
    re.compile(r"\d+ fear\b", re.IGNORECASE),
    re.compile(r"destroy \d* (?:explorer|town|city|invader)", re.IGNORECASE),
    re.compile(r"push \d+", re.IGNORECASE),
    re.compile(r"remove blight", re.IGNORECASE),
    re.compile(r"gain (?:a )?power", re.IGNORECASE),
    re.compile(r"add presence", re.IGNORECASE),
    re.compile(r"take no damage", re.IGNORECASE),
    re.compile(r"defend \d+", re.IGNORECASE),
    re.compile(r"gain \d+ energy", re.IGNORECASE),
]

# Expansion combos we report on.
COMBOS: dict[str, list[str]] = {
    "base": ["base", "hosi"],
    "base_bnc": ["base", "hosi", "bnc"],
    "base_je": ["base", "hosi", "je"],
    "base_bnc_je": ["base", "hosi", "bnc", "je"],
    "all": ["base", "hosi", "bnc", "je", "ni", "promo2", "promo1"],
    "ni_only": ["ni"],
}


def classify_expansion(set_field: str | None) -> list[str]:
    """Given a Wiki `set` field (may contain multiple comma-sep entries), return
    the list of normalized expansion keys present. Returns empty if no match."""
    if not set_field:
        return []
    text = set_field.lower()
    found: list[str] = []
    for needle, key in EXPANSION_MATCHERS:
        if needle in text and key not in found:
            found.append(key)
    return found


def card_is_in_combo(card: dict, combo_keys: list[str]) -> bool:
    """A card belongs to a combo if its classified expansions intersect the combo keys."""
    card_exps = classify_expansion(card.get("expansion") or card.get("set") or "")
    return any(k in combo_keys for k in card_exps)


def count_fear_amount(text: str | None) -> int:
    """Sum of all 'N Fear' instructions in the text."""
    if not text:
        return 0
    return sum(int(m.group(1)) for m in FEAR_AMOUNT_RX.finditer(text))


def count_keyword_hits(text: str | None, patterns: dict[str, re.Pattern]) -> dict[str, int]:
    if not text:
        return {}
    return {k: len(pat.findall(text)) for k, pat in patterns.items() if pat.search(text)}


def aggregate_power_deck(cards: list[dict], combo_keys: list[str]) -> dict:
    """Stats for Minor/Major pool within a given expansion combo."""
    subset = [c for c in cards if card_is_in_combo(c, combo_keys)]
    # Dedupe by name
    seen: set[str] = set()
    dedup: list[dict] = []
    for c in subset:
        if c.get("name") in seen:
            continue
        seen.add(c.get("name", ""))
        dedup.append(c)

    element_counter = Counter()
    cost_counter = Counter()
    speed_counter = Counter()
    token_counter = Counter()
    effect_counter = Counter()
    fear_amounts: list[int] = []

    for c in dedup:
        for e in c.get("elements") or []:
            element_counter[e.lower()] += 1
        cost_counter[int(c.get("cost") or 0)] += 1
        speed_counter[(c.get("speed") or "").lower()] += 1
        text = c.get("text") or ""
        for tok, pat in TOKEN_PATTERNS.items():
            if pat.search(text):
                token_counter[tok] += 1
        for kw, pat in EFFECT_KEYWORDS.items():
            if pat.search(text):
                effect_counter[kw] += 1
        fear_amounts.append(count_fear_amount(text))

    return {
        "pool_size": len(dedup),
        "element_distribution": dict(element_counter.most_common()),
        "cost_distribution": {str(k): v for k, v in sorted(cost_counter.items())},
        "speed_distribution": dict(speed_counter),
        "token_cards": dict(token_counter),
        "effect_cards": dict(effect_counter),
        "fear_amount": {
            "total_fear_generated_across_pool": sum(fear_amounts),
            "cards_with_fear": sum(1 for v in fear_amounts if v > 0),
            "mean_fear_per_fear_card": round(
                sum(v for v in fear_amounts if v > 0) / max(1, sum(1 for v in fear_amounts if v > 0)),
                2,
            ),
        },
    }


def aggregate_fear_deck(cards: list[dict], combo_keys: list[str]) -> dict:
    subset = [c for c in cards if card_is_in_combo(c, combo_keys)]
    seen: set[str] = set()
    dedup = []
    for c in subset:
        if c.get("name") in seen:
            continue
        seen.add(c.get("name", ""))
        dedup.append(c)

    # Fear cards are resolved at the player's current Terror level. Stage 1 =
    # always; Stage 2 = TL2+; Stage 3 = TL3.
    tl_summary = {"stage_1": [], "stage_2": [], "stage_3": []}
    token_counter = Counter()
    spirit_favor = Counter()  # aggregate favor across all stages
    invader_favor = Counter()

    for c in dedup:
        for k_in, k_out in (("text_stage_1", "stage_1"),
                             ("text_stage_2", "stage_2"),
                             ("text_stage_3", "stage_3")):
            text = c.get(k_in) or ""
            if not text:
                continue
            tl_summary[k_out].append({
                "name": c.get("name"),
                "expansion": c.get("expansion"),
                "text": text[:200],
                "fear_output": count_fear_amount(text),
            })
            for tok, pat in TOKEN_PATTERNS.items():
                if pat.search(text):
                    token_counter[tok] += 1
            for pat in SPIRIT_FAVORING:
                if pat.search(text):
                    spirit_favor[k_out] += 1
                    break
            for pat in INVADER_FAVORING:
                if pat.search(text):
                    invader_favor[k_out] += 1
                    break

    return {
        "pool_size": len(dedup),
        "by_stage": {k: len(v) for k, v in tl_summary.items()},
        "token_cards": dict(token_counter),
        "spirit_favoring_by_stage": dict(spirit_favor),
        "invader_favoring_by_stage": dict(invader_favor),
        "avg_fear_card_output": {
            stage: round(
                sum(c.get("fear_output", 0) for c in cards_list) / max(1, len(cards_list)),
                2,
            )
            for stage, cards_list in tl_summary.items()
        },
        "sample_titles": {stage: [c["name"] for c in cards_list[:5]] for stage, cards_list in tl_summary.items()},
    }


def aggregate_event_deck(cards: list[dict], combo_keys: list[str]) -> dict:
    subset = [c for c in cards if card_is_in_combo(c, combo_keys)]
    seen: set[str] = set()
    dedup = []
    for c in subset:
        if c.get("name") in seen:
            continue
        seen.add(c.get("name", ""))
        dedup.append(c)

    token_counter = Counter()
    spirit_hits = 0
    invader_hits = 0
    net_favor = 0
    stage_counter = Counter()

    card_scores: list[dict] = []
    for c in dedup:
        stages = c.get("stages") or []
        text_joined = " ".join(s.get("text") or "" for s in stages)
        stage_counter[len(stages)] += 1
        this_spirit = sum(1 for pat in SPIRIT_FAVORING if pat.search(text_joined))
        this_invader = sum(1 for pat in INVADER_FAVORING if pat.search(text_joined))
        spirit_hits += this_spirit
        invader_hits += this_invader
        score = this_spirit - this_invader
        net_favor += score
        card_scores.append({
            "name": c.get("name"),
            "expansion": c.get("expansion"),
            "spirit_hits": this_spirit,
            "invader_hits": this_invader,
            "net_score": score,
        })
        for tok, pat in TOKEN_PATTERNS.items():
            if pat.search(text_joined):
                token_counter[tok] += 1

    card_scores.sort(key=lambda x: -x["net_score"])
    return {
        "pool_size": len(dedup),
        "stages_distribution": {str(k): v for k, v in sorted(stage_counter.items())},
        "token_cards": dict(token_counter),
        "spirit_favoring_hits_total": spirit_hits,
        "invader_favoring_hits_total": invader_hits,
        "net_favor_score": net_favor,
        "top_5_spirit_favoring": card_scores[:5],
        "top_5_invader_favoring": card_scores[-5:][::-1],
    }


def aggregate_blight_deck(cards: list[dict], combo_keys: list[str]) -> dict:
    subset = [c for c in cards if card_is_in_combo(c, combo_keys)]
    seen = set()
    dedup = []
    for c in subset:
        if c.get("name") in seen:
            continue
        seen.add(c.get("name", ""))
        dedup.append(c)

    severity = []
    for c in dedup:
        text = c.get("text") or ""
        score = 0
        # Rough severity proxy: count of "Destroy" + "Add Blight" + "Ongoing"
        score += len(re.findall(r"\bdestroy\w*\b", text, re.IGNORECASE))
        score += len(re.findall(r"\badd \d* ?blight\b", text, re.IGNORECASE))
        score += len(re.findall(r"\bongoing\b", text, re.IGNORECASE)) * 2
        severity.append({"name": c.get("name"), "expansion": c.get("expansion"), "severity_proxy": score,
                         "blight_per_player": c.get("blight_per_player")})

    severity.sort(key=lambda x: -x["severity_proxy"])
    mean_severity = sum(s["severity_proxy"] for s in severity) / max(1, len(severity))
    return {
        "pool_size": len(dedup),
        "mean_severity_proxy": round(mean_severity, 2),
        "top_5_worst": severity[:5],
        "all_cards": severity,
    }


def load_deck(path: Path) -> list[dict]:
    raw = json.loads(path.read_text())
    return raw.get("cards") or []


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--combo", help="run for a single combo key")
    p.add_argument("--print-summary", action="store_true")
    args = p.parse_args()

    minors = load_deck(DECKS_DIR / "minor.json") if (DECKS_DIR / "minor.json").exists() else []
    majors = load_deck(DECKS_DIR / "major.json") if (DECKS_DIR / "major.json").exists() else []
    fear = load_deck(DECKS_DIR / "fear.json") if (DECKS_DIR / "fear.json").exists() else []
    event = load_deck(DECKS_DIR / "event.json") if (DECKS_DIR / "event.json").exists() else []
    blight = load_deck(DECKS_DIR / "blight.json") if (DECKS_DIR / "blight.json").exists() else []

    combos = {args.combo: COMBOS[args.combo]} if args.combo and args.combo in COMBOS else COMBOS
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    combined: dict[str, dict] = {}
    for combo_name, keys in combos.items():
        stats = {
            "combo_name": combo_name,
            "expansions_included": keys,
            "decks": {
                "minor": aggregate_power_deck(minors, keys),
                "major": aggregate_power_deck(majors, keys),
                "fear": aggregate_fear_deck(fear, keys),
                "event": aggregate_event_deck(event, keys),
                "blight": aggregate_blight_deck(blight, keys),
            },
        }
        out = OUT_DIR / f"{combo_name}.json"
        out.write_text(json.dumps(stats, indent=2) + "\n")
        print(f"wrote {out}")
        combined[combo_name] = stats

    # Combined all-combos summary for quick cross-combo comparison
    (OUT_DIR / "_combined.json").write_text(json.dumps(combined, indent=2) + "\n")
    print(f"wrote {OUT_DIR / '_combined.json'}")

    if args.print_summary:
        print("\n=== Pool sizes per combo ===")
        print(f"{'combo':<18} {'minor':>6} {'major':>6} {'fear':>5} {'event':>6} {'blight':>7}")
        for name, s in combined.items():
            print(
                f"{name:<18} "
                f"{s['decks']['minor']['pool_size']:>6} "
                f"{s['decks']['major']['pool_size']:>6} "
                f"{s['decks']['fear']['pool_size']:>5} "
                f"{s['decks']['event']['pool_size']:>6} "
                f"{s['decks']['blight']['pool_size']:>7}"
            )


if __name__ == "__main__":
    main()
