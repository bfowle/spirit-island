#!/usr/bin/env python3
"""
Per-expansion marginal-contribution analysis for every deck.

Instead of looking at combo pools (base / base+B&C / all), this isolates each
expansion's contribution: compute mean, median, stdev, CV, IQR on the cards
attributable to each expansion. This lets us test claims like:

  "B&C alone feels swingy" → expect high CV in B&C fear outputs
  "JE balances that" → expect JE CV lower than B&C
  "NI dilutes" → expect NI mean below B&C/JE mean

Outputs:
  data/references/deck-stats/per_expansion_deltas.json
  data/references/deck-stats/dilution_summary.json  (computed dilution metrics)

Usage:
    python3 scripts/expansion-deltas.py
"""

from __future__ import annotations
import argparse
import json
import math
import re
import statistics as stats
import sys
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DECKS_DIR = REPO / "data" / "decks"
OUT_DIR = REPO / "data" / "references" / "deck-stats"

# Import helpers from deck-stats without colliding on filename hyphens.
from importlib.machinery import SourceFileLoader
ds = SourceFileLoader(
    "deck_stats_helper",
    str(REPO / "scripts" / "deck-stats.py"),
).load_module()

classify_expansion = ds.classify_expansion
FEAR_AMOUNT_RX = ds.FEAR_AMOUNT_RX
TOKEN_PATTERNS = ds.TOKEN_PATTERNS
EFFECT_KEYWORDS = ds.EFFECT_KEYWORDS
SPIRIT_FAVORING = ds.SPIRIT_FAVORING
INVADER_FAVORING = ds.INVADER_FAVORING

EXPANSIONS = ["base", "bnc", "je", "ni"]
EXPANSION_LABELS = {
    "base": "Base game",
    "bnc": "Branch & Claw",
    "je": "Jagged Earth",
    "ni": "Nature Incarnate",
}


def assign_exclusive_expansion(card: dict) -> str | None:
    """Assign each card to exactly one 'home' expansion — its earliest or
    most-specific expansion tag. This is what we want for marginal analysis:
    `bnc only` = cards that appeared exclusively in B&C (not also in base)."""
    exps = classify_expansion(card.get("expansion") or card.get("set") or "")
    # Priority order (earliest expansion wins as "home" when a card is in multiple):
    for key in ("base", "bnc", "je", "ni"):
        if key in exps:
            return key
    return None


def card_score(text: str) -> dict:
    """Compute per-card metrics usable for marginal analysis."""
    text = text or ""
    fear_amount = sum(int(m.group(1)) for m in FEAR_AMOUNT_RX.finditer(text))
    tokens = sum(1 for pat in TOKEN_PATTERNS.values() if pat.search(text))
    spirit = sum(1 for pat in SPIRIT_FAVORING if pat.search(text))
    invader = sum(1 for pat in INVADER_FAVORING if pat.search(text))
    destroy_hits = len(re.findall(r"\bdestroy\w*\b", text, re.IGNORECASE))
    damage_hits = len(re.findall(r"\b\d+\s+damage\b", text, re.IGNORECASE))
    push_hits = len(re.findall(r"\bpush\w*\b", text, re.IGNORECASE))
    blight_add = len(re.findall(r"\badd \d* ?blight\b", text, re.IGNORECASE))
    return {
        "fear_amount": fear_amount,
        "tokens_touched": tokens,
        "spirit_favor": spirit,
        "invader_favor": invader,
        "net_favor": spirit - invader,
        "destroy": destroy_hits,
        "damage": damage_hits,
        "push": push_hits,
        "blight_add": blight_add,
    }


def iqr(values: list[float]) -> tuple[float, float, float]:
    """Return (Q1, median, Q3)."""
    if not values:
        return (0.0, 0.0, 0.0)
    s = sorted(values)
    n = len(s)
    def q(p):
        idx = max(0, min(n - 1, int(p * (n - 1))))
        return s[idx]
    return (q(0.25), q(0.50), q(0.75))


def moments(values: list[float]) -> dict:
    if not values:
        return {"n": 0, "mean": 0, "stdev": 0, "cv": 0, "min": 0, "max": 0,
                "q1": 0, "median": 0, "q3": 0}
    m = stats.fmean(values)
    s = stats.stdev(values) if len(values) > 1 else 0
    cv = (s / m) if m else 0
    q1, med, q3 = iqr(values)
    return {
        "n": len(values),
        "mean": round(m, 3),
        "stdev": round(s, 3),
        "cv": round(cv, 3),
        "min": min(values),
        "max": max(values),
        "q1": q1,
        "median": med,
        "q3": q3,
    }


def analyze_deck(cards: list[dict], deck_kind: str) -> dict:
    """Per-expansion analysis of a single deck. For fear cards, concatenate
    all three stages into one text blob per card. For events, concatenate
    all stages. For minor/major, use text. For blight, use text."""
    by_exp: dict[str, list[dict]] = {k: [] for k in EXPANSIONS}
    for c in cards:
        exp = assign_exclusive_expansion(c)
        if exp is None:
            continue
        if deck_kind == "fear":
            text = " ".join(filter(None, [
                c.get("text_stage_1"), c.get("text_stage_2"), c.get("text_stage_3"),
            ]))
        elif deck_kind == "event":
            text = " ".join((s.get("text") or "") for s in (c.get("stages") or []))
        else:
            text = c.get("text") or ""
        score = card_score(text)
        score["name"] = c.get("name")
        by_exp[exp].append(score)

    result = {}
    for exp in EXPANSIONS:
        cards_here = by_exp[exp]
        if not cards_here:
            result[exp] = {"pool": 0}
            continue
        result[exp] = {
            "pool": len(cards_here),
            "fear_amount": moments([c["fear_amount"] for c in cards_here]),
            "tokens_touched": moments([c["tokens_touched"] for c in cards_here]),
            "spirit_favor": moments([c["spirit_favor"] for c in cards_here]),
            "invader_favor": moments([c["invader_favor"] for c in cards_here]),
            "net_favor": moments([c["net_favor"] for c in cards_here]),
            "destroy": moments([c["destroy"] for c in cards_here]),
            "push": moments([c["push"] for c in cards_here]),
            "blight_add": moments([c["blight_add"] for c in cards_here]),
            "examples_high_output": sorted(cards_here, key=lambda x: -x["fear_amount"])[:3],
            "examples_high_spirit": sorted(cards_here, key=lambda x: -x["net_favor"])[:3],
            "examples_high_invader": sorted(cards_here, key=lambda x: x["net_favor"])[:3],
        }
    return result


def swinginess_ratio(deck_analysis: dict, metric: str = "fear_amount") -> dict:
    """How swingy is each expansion's contribution, measured by CV on `metric`?
    A higher CV = more variance relative to the mean = more swingy."""
    out = {}
    for exp in EXPANSIONS:
        d = deck_analysis.get(exp, {})
        if isinstance(d.get(metric), dict):
            out[exp] = {
                "n": d[metric]["n"],
                "cv": d[metric]["cv"],
                "iqr_span": d[metric]["q3"] - d[metric]["q1"],
                "mean": d[metric]["mean"],
                "range": d[metric]["max"] - d[metric]["min"],
            }
    return out


def dilution_delta(deck_analysis: dict, metric: str = "fear_amount") -> dict:
    """Compute how each later expansion shifts the running-mean relative to
    base+everything-before."""
    running: list[dict] = []  # {exp, pool_cumulative, total_metric}
    cum_cards = []
    for exp in EXPANSIONS:
        d = deck_analysis.get(exp, {})
        if isinstance(d.get(metric), dict):
            # Reconstruct individual card values: n entries with 'mean' won't do,
            # but we can approximate mean shift using pool × mean
            n = d[metric]["n"]
            mean = d[metric]["mean"]
            cum_cards.append((exp, n, mean, mean * n))
    out = {}
    total_n, total_sum = 0, 0.0
    running_mean_before = None
    for exp, n, mean, summ in cum_cards:
        total_n_new = total_n + n
        total_sum_new = total_sum + summ
        running_mean_new = total_sum_new / total_n_new if total_n_new else 0
        dilution = (running_mean_new - running_mean_before) if running_mean_before is not None else None
        out[exp] = {
            "pool_added": n,
            "mean_added": mean,
            "cumulative_pool": total_n_new,
            "cumulative_mean": round(running_mean_new, 3),
            "delta_from_previous": round(dilution, 3) if dilution is not None else None,
        }
        total_n = total_n_new
        total_sum = total_sum_new
        running_mean_before = running_mean_new
    return out


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--output", default=str(OUT_DIR / "per_expansion_deltas.json"))
    args = p.parse_args()

    decks = {}
    for name in ("minor", "major", "fear", "event", "blight"):
        path = DECKS_DIR / f"{name}.json"
        if path.exists():
            decks[name] = json.loads(path.read_text()).get("cards") or []

    analysis = {name: analyze_deck(cards, name) for name, cards in decks.items()}
    swing = {
        name: {
            metric: swinginess_ratio(analysis[name], metric)
            for metric in ("fear_amount", "net_favor", "tokens_touched")
        }
        for name in analysis
    }
    dilution = {
        name: {
            metric: dilution_delta(analysis[name], metric)
            for metric in ("fear_amount", "net_favor", "tokens_touched")
        }
        for name in analysis
    }

    out = {
        "per_deck_per_expansion": analysis,
        "swinginess_cv": swing,
        "dilution_shift": dilution,
    }
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    Path(args.output).write_text(json.dumps(out, indent=2, default=str) + "\n")
    print(f"wrote {args.output}", file=sys.stderr)

    # Print a human-readable summary of the Fear-deck swinginess, which is
    # what the user asked about.
    fear = analysis.get("fear", {})
    print("\n=== Fear deck: swinginess per expansion ===")
    print(f"{'expansion':<20} {'n':>4} {'mean':>7} {'stdev':>7} {'CV':>7} {'IQR':>7} {'min':>4} {'max':>4}")
    for exp in EXPANSIONS:
        d = fear.get(exp, {})
        if "fear_amount" not in d:
            print(f"{EXPANSION_LABELS[exp]:<20} (empty)")
            continue
        fa = d["fear_amount"]
        iqr_span = fa["q3"] - fa["q1"]
        print(
            f"{EXPANSION_LABELS[exp]:<20} "
            f"{fa['n']:>4} "
            f"{fa['mean']:>7.2f} "
            f"{fa['stdev']:>7.2f} "
            f"{fa['cv']:>7.2f} "
            f"{iqr_span:>7.1f} "
            f"{fa['min']:>4} "
            f"{fa['max']:>4}"
        )

    print("\n=== Fear deck: net-favor per expansion (spirit minus invader hits) ===")
    print(f"{'expansion':<20} {'n':>4} {'mean':>7} {'stdev':>7} {'CV':>7} {'median':>7}")
    for exp in EXPANSIONS:
        d = fear.get(exp, {})
        if "net_favor" not in d:
            continue
        nf = d["net_favor"]
        print(
            f"{EXPANSION_LABELS[exp]:<20} "
            f"{nf['n']:>4} "
            f"{nf['mean']:>7.2f} "
            f"{nf['stdev']:>7.2f} "
            f"{nf['cv']:>7.2f} "
            f"{nf['median']:>7.1f}"
        )

    print("\n=== Dilution of Fear deck over expansions (running mean shift) ===")
    print("metric: fear_amount (sum of 'N Fear' instructions per card, combined stages)")
    d = dilution["fear"]["fear_amount"]
    for exp, v in d.items():
        delta = v["delta_from_previous"]
        dtxt = f"{delta:+.2f}" if delta is not None else "  base"
        print(f"  {EXPANSION_LABELS[exp]:<20} +{v['pool_added']:>3} cards, mean-added {v['mean_added']:>5.2f}, cumulative-mean {v['cumulative_mean']:>5.2f} ({dtxt})")

    # Event deck equivalent
    event = analysis.get("event", {})
    if event:
        print("\n=== Event deck: swinginess + dilution ===")
        print(f"{'expansion':<20} {'n':>4} {'mean':>7} {'stdev':>7} {'CV':>7}")
        for exp in EXPANSIONS:
            d = event.get(exp, {})
            if "net_favor" not in d:
                continue
            nf = d["net_favor"]
            print(f"{EXPANSION_LABELS[exp]:<20} {nf['n']:>4} {nf['mean']:>7.2f} {nf['stdev']:>7.2f} {nf['cv']:>7.2f}")

    # Minor/Major quick view
    print("\n=== Minor deck: mean fear-amount per card by expansion ===")
    minor = analysis.get("minor", {})
    for exp in EXPANSIONS:
        d = minor.get(exp, {})
        if "fear_amount" in d:
            print(f"  {EXPANSION_LABELS[exp]:<20} n={d['fear_amount']['n']:>3}, mean={d['fear_amount']['mean']:.2f}, CV={d['fear_amount']['cv']:.2f}")

    print("\n=== Major deck: mean fear-amount per card by expansion ===")
    major = analysis.get("major", {})
    for exp in EXPANSIONS:
        d = major.get(exp, {})
        if "fear_amount" in d:
            print(f"  {EXPANSION_LABELS[exp]:<20} n={d['fear_amount']['n']:>3}, mean={d['fear_amount']['mean']:.2f}, CV={d['fear_amount']['cv']:.2f}")


if __name__ == "__main__":
    main()
