#!/usr/bin/env python3
"""
Monte-Carlo simulator for Spirit Island deck dynamics across expansion combos.

For each expansion combo, runs N trials of a simplified 8-round game:

- Fear deck is shuffled and drawn against a Fear-generation model:
  the Fear pool increases by a configurable per-turn rate; each time the pool
  crosses the threshold, a Fear card is drawn at the current Terror level.
  Terror 2/3 transitions are applied as Fear threshold fills up.
- Event deck is shuffled and drawn once per turn (starting T1 for B&C+ games
  and skipped for base-only).
- Blight deck is drawn once at game start + each time the Blight track fills
  (approximated by a configurable Blight-per-turn rate).

Outputs the mean + Wilson 95% CI across trials for:
- Fear drawn per game, split by stage (1, 2, 3)
- Token additions from Fear + Event cards (each token type)
- Invader-favoring vs Spirit-favoring net from Event draws
- Blight card draws + severity-proxy accumulated
- Expected fear-card output per game (sum of "N Fear" instructions that fire)

Writes data/references/deck-stats/sim_<combo>.json for each combo.

Usage:
    python3 scripts/deck-simulate.py
    python3 scripts/deck-simulate.py --trials 50000 --rounds 10
"""

from __future__ import annotations
import argparse
import json
import math
import random
import re
import statistics as stats
import sys
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DECKS_DIR = REPO / "data" / "decks"
OUT_DIR = REPO / "data" / "references" / "deck-stats"

from importlib.machinery import SourceFileLoader

deck_stats = SourceFileLoader(
    "deck_stats_helper",
    str(REPO / "scripts" / "deck-stats.py"),
).load_module()

COMBOS = deck_stats.COMBOS
classify_expansion = deck_stats.classify_expansion
card_is_in_combo = deck_stats.card_is_in_combo
TOKEN_PATTERNS = deck_stats.TOKEN_PATTERNS
SPIRIT_FAVORING = deck_stats.SPIRIT_FAVORING
INVADER_FAVORING = deck_stats.INVADER_FAVORING
FEAR_AMOUNT_RX = deck_stats.FEAR_AMOUNT_RX


def load_combo_deck(deck: list[dict], combo_keys: list[str]) -> list[dict]:
    seen: set[str] = set()
    out = []
    for c in deck:
        if card_is_in_combo(c, combo_keys) and c.get("name") not in seen:
            seen.add(c.get("name", ""))
            out.append(c)
    return out


def score_card_text(text: str) -> dict:
    """Return per-card metrics: fear_amount, tokens_added (counter),
    spirit_hits, invader_hits."""
    out = {
        "fear_amount": sum(int(m.group(1)) for m in FEAR_AMOUNT_RX.finditer(text or "")),
        "tokens_added": Counter(),
        "spirit_hits": 0,
        "invader_hits": 0,
    }
    for tok, pat in TOKEN_PATTERNS.items():
        if pat.search(text or ""):
            out["tokens_added"][tok] += 1
    for pat in SPIRIT_FAVORING:
        if pat.search(text or ""):
            out["spirit_hits"] += 1
    for pat in INVADER_FAVORING:
        if pat.search(text or ""):
            out["invader_hits"] += 1
    return out


def wilson_95(x: float, n: float) -> tuple[float, float]:
    """Wilson 95% score interval for a proportion."""
    if n == 0:
        return (0.0, 1.0)
    z = 1.959_963
    p = x / n
    denom = 1 + z * z / n
    center = (p + z * z / (2 * n)) / denom
    margin = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / denom
    return (max(0, center - margin), min(1, center + margin))


def mean_ci(values: list[float]) -> dict:
    if not values:
        return {"mean": 0, "ci_lo": 0, "ci_hi": 0, "n": 0}
    m = stats.fmean(values)
    if len(values) < 2:
        return {"mean": round(m, 3), "ci_lo": round(m, 3), "ci_hi": round(m, 3), "n": len(values)}
    s = stats.stdev(values)
    # Normal-approx SE CI; stable when n is large.
    se = s / math.sqrt(len(values))
    return {
        "mean": round(m, 3),
        "ci_lo": round(m - 1.96 * se, 3),
        "ci_hi": round(m + 1.96 * se, 3),
        "stdev": round(s, 3),
        "n": len(values),
    }


def simulate_game(
    fear_deck: list[dict],
    event_deck: list[dict] | None,
    blight_deck: list[dict] | None,
    rounds: int,
    players: int,
    fear_per_turn_mean: float,
    fear_per_turn_stdev: float,
    blight_per_turn_mean: float,
) -> dict:
    """Simulate one 8-round game; return per-game aggregated metrics.

    Model assumptions (for replacement later):
    - Fear threshold transitions: TL1→TL2 at `4 × players` fear; TL2→TL3 at
      `8 × players` fear; win at `12 × players` fear. Each Fear card drawn
      pulls from fear_deck without replacement; at draw, the current TL
      determines which stage's text resolves.
    - Event deck: 1 draw per turn starting T1 (only if event_deck provided).
    - Blight deck: 1 draw at game start; another if Blight-per-turn total
      exceeds the current card's threshold — simplified: 1 blight card / game.
    """
    fear_pool = 0
    terror = 1
    blight_track = 0

    fear_threshold_tl1 = 4 * players
    fear_threshold_tl2 = 8 * players

    metrics = {
        "fear_drawn_total": 0,
        "fear_drawn_by_stage": Counter(),
        "fear_output": 0,
        "tokens_added": Counter(),
        "spirit_favor_hits": 0,
        "invader_favor_hits": 0,
        "events_drawn": 0,
        "blight_cards_drawn": 0,
        "blight_severity_accum": 0,
    }

    fear_shuffled = list(fear_deck)
    random.shuffle(fear_shuffled)
    event_shuffled = list(event_deck) if event_deck else []
    random.shuffle(event_shuffled)
    blight_shuffled = list(blight_deck) if blight_deck else []
    random.shuffle(blight_shuffled)

    # Initial Blight draw at game start (if Blight deck in play)
    if blight_shuffled:
        bc = blight_shuffled.pop(0)
        metrics["blight_cards_drawn"] += 1
        text = bc.get("text") or ""
        metrics["blight_severity_accum"] += (
            len(re.findall(r"\bdestroy\w*\b", text, re.IGNORECASE))
            + len(re.findall(r"\badd \d* ?blight\b", text, re.IGNORECASE))
            + len(re.findall(r"\bongoing\b", text, re.IGNORECASE)) * 2
        )

    for turn in range(1, rounds + 1):
        # Fear generated this turn (Gaussian around mean)
        turn_fear = max(0, round(random.gauss(fear_per_turn_mean, fear_per_turn_stdev)))
        fear_pool += turn_fear

        # Resolve Fear threshold crossings & card draws
        while fear_pool >= (fear_threshold_tl1 if terror == 1 else fear_threshold_tl2 if terror == 2 else 999):
            if terror == 1:
                fear_pool -= fear_threshold_tl1
                terror = 2
                stage = 1
            elif terror == 2:
                fear_pool -= fear_threshold_tl2
                terror = 3
                stage = 2
            else:
                break
            # Draw the appropriate number of Fear cards per threshold: simplification
            # — 1 card per threshold crossing at the TL we just left.
            if fear_shuffled:
                card = fear_shuffled.pop(0)
                metrics["fear_drawn_total"] += 1
                metrics["fear_drawn_by_stage"][f"stage_{stage}"] += 1
                text = card.get(f"text_stage_{stage}") or ""
                card_m = score_card_text(text)
                metrics["fear_output"] += card_m["fear_amount"]
                metrics["tokens_added"] += card_m["tokens_added"]
                metrics["spirit_favor_hits"] += card_m["spirit_hits"]
                metrics["invader_favor_hits"] += card_m["invader_hits"]

        # Event draw (per turn if deck present)
        if event_shuffled:
            ec = event_shuffled.pop(0)
            metrics["events_drawn"] += 1
            ev_text = " ".join(s.get("text") or "" for s in (ec.get("stages") or []))
            card_m = score_card_text(ev_text)
            metrics["fear_output"] += card_m["fear_amount"]
            metrics["tokens_added"] += card_m["tokens_added"]
            metrics["spirit_favor_hits"] += card_m["spirit_hits"]
            metrics["invader_favor_hits"] += card_m["invader_hits"]

        # Blight tick
        blight_track += random.gauss(blight_per_turn_mean, 1.0)
        if blight_track > 3 and blight_shuffled:
            bc = blight_shuffled.pop(0)
            metrics["blight_cards_drawn"] += 1
            text = bc.get("text") or ""
            metrics["blight_severity_accum"] += (
                len(re.findall(r"\bdestroy\w*\b", text, re.IGNORECASE))
                + len(re.findall(r"\badd \d* ?blight\b", text, re.IGNORECASE))
                + len(re.findall(r"\bongoing\b", text, re.IGNORECASE)) * 2
            )
            blight_track = 0

    metrics["terror_level_final"] = terror
    metrics["fear_drawn_by_stage"] = dict(metrics["fear_drawn_by_stage"])
    metrics["tokens_added"] = dict(metrics["tokens_added"])
    return metrics


def run_combo(combo_name: str, keys: list[str], fear, event, blight, trials: int, rounds: int, players: int) -> dict:
    fear_combo = load_combo_deck(fear, keys)
    # Events are B&C+ only (base has no Event deck). Skip when not in combo.
    has_events = "bnc" in keys or "je" in keys or "ni" in keys
    event_combo = load_combo_deck(event, keys) if has_events else None
    # Blight deck is also B&C+
    has_blight = has_events
    blight_combo = load_combo_deck(blight, keys) if has_blight else None

    results = []
    for _ in range(trials):
        r = simulate_game(
            fear_combo, event_combo, blight_combo,
            rounds=rounds, players=players,
            fear_per_turn_mean=2.5,  # typical — base games average ~2-3 fear/turn from spirits
            fear_per_turn_stdev=1.0,
            blight_per_turn_mean=0.5,
        )
        results.append(r)

    # Aggregate per-metric
    def series(key):
        return [r[key] for r in results]

    def token_series(tok):
        return [r["tokens_added"].get(tok, 0) for r in results]

    agg = {
        "combo_name": combo_name,
        "expansions_included": keys,
        "trials": trials,
        "rounds_per_game": rounds,
        "players": players,
        "pool_sizes": {
            "fear": len(fear_combo),
            "event": len(event_combo) if event_combo else 0,
            "blight": len(blight_combo) if blight_combo else 0,
        },
        "per_game": {
            "fear_cards_drawn": mean_ci(series("fear_drawn_total")),
            "fear_output_total": mean_ci(series("fear_output")),
            "events_drawn": mean_ci(series("events_drawn")),
            "blight_cards_drawn": mean_ci(series("blight_cards_drawn")),
            "blight_severity_accum": mean_ci(series("blight_severity_accum")),
            "spirit_favor_hits": mean_ci(series("spirit_favor_hits")),
            "invader_favor_hits": mean_ci(series("invader_favor_hits")),
            "net_favor": mean_ci([r["spirit_favor_hits"] - r["invader_favor_hits"] for r in results]),
        },
        "terror_level_final_distribution": dict(Counter(r["terror_level_final"] for r in results)),
        "tokens_per_game": {tok: mean_ci(token_series(tok)) for tok in TOKEN_PATTERNS.keys()},
    }
    return agg


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--trials", type=int, default=5000)
    p.add_argument("--rounds", type=int, default=8)
    p.add_argument("--players", type=int, default=1)
    p.add_argument("--seed", type=int, default=42)
    args = p.parse_args()

    random.seed(args.seed)

    fear = deck_stats.load_deck(DECKS_DIR / "fear.json") if (DECKS_DIR / "fear.json").exists() else []
    event = deck_stats.load_deck(DECKS_DIR / "event.json") if (DECKS_DIR / "event.json").exists() else []
    blight = deck_stats.load_deck(DECKS_DIR / "blight.json") if (DECKS_DIR / "blight.json").exists() else []

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    combined = {}
    for name, keys in COMBOS.items():
        print(f"Simulating {name} ({args.trials} trials, {args.rounds}-round, {args.players}p)...", file=sys.stderr)
        agg = run_combo(name, keys, fear, event, blight, args.trials, args.rounds, args.players)
        out = OUT_DIR / f"sim_{name}.json"
        out.write_text(json.dumps(agg, indent=2) + "\n")
        print(f"  wrote {out}", file=sys.stderr)
        combined[name] = agg

    (OUT_DIR / "_simulation_combined.json").write_text(json.dumps(combined, indent=2) + "\n")
    print(f"\n=== per-game summary across combos ({args.trials} trials × {args.rounds} rounds, {args.players}p) ===")
    print(f"{'combo':<16} {'fear_drawn':>12} {'fear_out':>10} {'ev':>4} {'blight':>7} {'+favor':>7} {'-favor':>7} {'net':>7}")
    for name, a in combined.items():
        pg = a["per_game"]
        print(
            f"{name:<16} "
            f"{pg['fear_cards_drawn']['mean']:>12.2f} "
            f"{pg['fear_output_total']['mean']:>10.2f} "
            f"{pg['events_drawn']['mean']:>4.1f} "
            f"{pg['blight_cards_drawn']['mean']:>7.2f} "
            f"{pg['spirit_favor_hits']['mean']:>7.2f} "
            f"{pg['invader_favor_hits']['mean']:>7.2f} "
            f"{pg['net_favor']['mean']:>+7.2f}"
        )


if __name__ == "__main__":
    main()
