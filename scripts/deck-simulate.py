#!/usr/bin/env python3
"""
Monte-Carlo simulator for Spirit Island deck dynamics across expansion combos.

For each expansion combo, runs N trials of a simplified game with no
Adversary, no Scenario and no extra board. Rules followed (sources in
data/references/rules/open-questions.md):

- Fear (Q6, Q7; core rulebook setup steps 2-3): the Fear Pool holds
  4 Fear Markers per player, so each Fear Card costs 4 x players Fear.
  Leftover Fear carries over. The Fear Deck is 9 cards dealt from the
  combo's pool, split 3/3/3 by the Terror Level dividers. Earning the card
  that reveals a divider raises the Terror Level at once; earning the last
  card is a Fear victory. Earned cards resolve in the Invader Phase's Fear
  step at the Terror Level current then (FAQ, Fear and Fear Cards).
- Events (JE rulebook p.12; Q15): one Event per Invader Phase, after the
  Blighted Island effect and before the Fear step. The Turn-1 Event is
  drawn and discarded without resolving. The Event deck is never
  reshuffled. Slave Rebellion is used only against France, so it is left
  out of the pool here.
- Blight (Q11, Q12; NI p.5 errata; JE rulebook "Still Healthy" cards): one
  Blight Card at setup, Healthy side up, with 2 Blight per player + 1.
  When its pool empties the card flips to its other side, refilling the
  pool with that side's number per player (no +1). A Still-Healthy card
  whose pool empties draws a new Blight Card and goes straight to its
  reverse side. When a Blighted side's pool empties, the players lose.

Model inputs that are NOT sourced (placeholders, change them with care):
- Fear generated per turn: Gaussian, mean 2.5, sd 1.0, rounded, floored at 0.
- Net Blight added per turn from Turn 2 (no Ravage on Turn 1, Q24):
  Gaussian, mean 0.5, sd 1.0, rounded (a negative value returns Blight to
  the card).
The simulation does not stop at a win or loss; it reports whether a Fear
victory or a Blight loss was reached within the rounds simulated.

Outputs the mean + 95% CI across trials for:
- Fear Cards earned per game, and how many resolved at Terror Level I/II/III
- Token additions from resolved Fear + Event cards (each token type)
- Invader-favoring vs Spirit-favoring keyword hits from resolved cards
- Blight Cards drawn, whether the island went Blighted, and a keyword
  severity proxy summed over every Blight Card reverse side reached
- "N Fear" phrases on resolved Fear and Event texts (a text count; that
  Fear is not added to the Fear Pool)

Writes <out-dir>/sim_<combo>.json for each combo (default out-dir:
data/references/deck-stats/).

Usage:
    python3 scripts/deck-simulate.py
    python3 scripts/deck-simulate.py --trials 50000 --rounds 10
    python3 scripts/deck-simulate.py --players 2 --fear-per-player 5
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


# Core rulebook setup step 3: 9 Fear Cards, Terror Level dividers splitting
# them 3/3/3 (no Adversary).
FEAR_DECK_LAYOUT = (3, 3, 3)
# Core rulebook setup step 2 and Q6: 4 Fear Markers per player. England L6
# and an extra board each add +1 per player (pass --fear-per-player 5).
FEAR_PER_PLAYER = 4
# Q12 and the NI p.5 errata: Healthy side 2 per player, +1 at setup only.
HEALTHY_BLIGHT_PER_PLAYER = 2
SETUP_EXTRA_BLIGHT = 1
# Q15: Slave Rebellion is added only against France.
FRANCE_ONLY_EVENTS = {"Slave Rebellion"}


def blight_severity(text: str) -> int:
    """Keyword severity proxy for a Blight Card side: Destroy hits + Add
    Blight hits + 2 x Ongoing hits. A text count, not a measured effect."""
    return (
        len(re.findall(r"\bdestroy\w*\b", text, re.IGNORECASE))
        + len(re.findall(r"\badd \d* ?blight\b", text, re.IGNORECASE))
        + len(re.findall(r"\bongoing\b", text, re.IGNORECASE)) * 2
    )


def is_still_healthy(card: dict) -> bool:
    return "still-healthy" in (card.get("cardtype") or "").lower()


def simulate_game(
    fear_deck: list[dict],
    event_deck: list[dict] | None,
    blight_deck: list[dict] | None,
    rounds: int,
    players: int,
    fear_per_player: int,
    fear_per_turn_mean: float,
    fear_per_turn_stdev: float,
    blight_per_turn_mean: float,
) -> dict:
    """Simulate one game of `rounds` turns; return per-game metrics.

    Turn order modeled: Spirit Phase (this turn's Fear is generated and
    Fear Cards are earned), then the Invader Phase: Event, Fear step
    (earned cards resolve), then Ravage/Build/Explore (net Blight change).
    All of a turn's Fear is assumed to arrive before the Invader Phase.
    """
    fear_per_card = fear_per_player * players
    dividers = set()
    running = 0
    for group in FEAR_DECK_LAYOUT[:-1]:
        running += group
        dividers.add(running)

    metrics = {
        "fear_drawn_total": 0,  # Fear Cards earned
        "fear_drawn_by_stage": Counter(),  # resolved at Terror Level I/II/III
        "fear_output": 0,
        "tokens_added": Counter(),
        "spirit_favor_hits": 0,
        "invader_favor_hits": 0,
        "events_drawn": 0,
        "events_resolved": 0,
        "blight_cards_drawn": 0,
        "blight_severity_accum": 0,
        "island_blighted": 0,
        "blight_loss_reached": 0,
        "fear_victory_reached": 0,
    }

    def resolve(text: str) -> None:
        card_m = score_card_text(text)
        metrics["fear_output"] += card_m["fear_amount"]
        metrics["tokens_added"] += card_m["tokens_added"]
        metrics["spirit_favor_hits"] += card_m["spirit_hits"]
        metrics["invader_favor_hits"] += card_m["invader_hits"]

    fear_shuffled = list(fear_deck)
    random.shuffle(fear_shuffled)
    fear_shuffled = fear_shuffled[: sum(FEAR_DECK_LAYOUT)]
    event_shuffled = list(event_deck) if event_deck else []
    random.shuffle(event_shuffled)
    blight_shuffled = list(blight_deck) if blight_deck else []
    random.shuffle(blight_shuffled)

    # Blight setup: one card, Healthy side up, 2 per player + 1.
    blight_card = None
    blight_side = None  # "healthy" | "still_healthy" | "blighted"
    blight_pool = 0
    if blight_shuffled:
        blight_card = blight_shuffled.pop(0)
        metrics["blight_cards_drawn"] += 1
        blight_side = "healthy"
        blight_pool = HEALTHY_BLIGHT_PER_PLAYER * players + SETUP_EXTRA_BLIGHT

    def go_to_reverse(card: dict) -> None:
        nonlocal blight_side, blight_pool
        # Refill with that side's number per player; no +1 on a flip.
        blight_pool += int(card.get("blight_per_player") or 0) * players
        metrics["blight_severity_accum"] += blight_severity(card.get("text") or "")
        if is_still_healthy(card):
            blight_side = "still_healthy"
        else:
            blight_side = "blighted"
            metrics["island_blighted"] = 1

    fear_pool = 0
    terror = 1
    earned = 0
    banked: list[dict] = []

    for turn in range(1, rounds + 1):
        # Spirit Phase: Fear generated this turn (placeholder rate).
        fear_pool += max(0, round(random.gauss(fear_per_turn_mean, fear_per_turn_stdev)))
        while fear_pool >= fear_per_card and fear_shuffled:
            fear_pool -= fear_per_card
            banked.append(fear_shuffled.pop(0))
            earned += 1
            if earned in dividers:
                terror += 1  # Terror Level changes the moment the divider is reached
            if not fear_shuffled:
                metrics["fear_victory_reached"] = 1

        # Invader Phase, Event step: Turn-1 Event is discarded unresolved.
        if event_shuffled:
            ec = event_shuffled.pop(0)
            metrics["events_drawn"] += 1
            if turn > 1:
                metrics["events_resolved"] += 1
                resolve(" ".join(s.get("text") or "" for s in (ec.get("stages") or [])))

        # Fear step: earned cards resolve at the current Terror Level.
        if not metrics["fear_victory_reached"]:
            for card in banked:
                metrics["fear_drawn_by_stage"][f"stage_{terror}"] += 1
                resolve(card.get(f"text_stage_{terror}") or "")
        banked.clear()

        # Ravage/Build/Explore: net Blight change (placeholder rate), from Turn 2.
        if blight_side and turn > 1 and not metrics["blight_loss_reached"]:
            blight_pool -= round(random.gauss(blight_per_turn_mean, 1.0))
            while blight_pool <= 0 and not metrics["blight_loss_reached"]:
                if blight_side == "healthy":
                    go_to_reverse(blight_card)
                elif blight_side == "still_healthy":
                    if not blight_shuffled:
                        break
                    blight_card = blight_shuffled.pop(0)
                    metrics["blight_cards_drawn"] += 1
                    go_to_reverse(blight_card)
                else:
                    metrics["blight_loss_reached"] = 1

    metrics["fear_drawn_total"] = earned
    metrics["terror_level_final"] = terror
    metrics["fear_drawn_by_stage"] = dict(metrics["fear_drawn_by_stage"])
    metrics["tokens_added"] = dict(metrics["tokens_added"])
    return metrics


def run_combo(combo_name: str, keys: list[str], fear, event, blight, trials: int,
              rounds: int, players: int, fear_per_player: int) -> dict:
    fear_combo = load_combo_deck(fear, keys)
    # Events are B&C+ only (base has no Event deck). Skip when not in combo.
    has_events = "bnc" in keys or "je" in keys or "ni" in keys
    event_combo = None
    if has_events:
        event_combo = [c for c in load_combo_deck(event, keys)
                       if c.get("name") not in FRANCE_ONLY_EVENTS]
    # The core game has Blight Cards too (2 in the base box), so every combo
    # draws one at setup.
    blight_combo = load_combo_deck(blight, keys)

    results = []
    for _ in range(trials):
        r = simulate_game(
            fear_combo, event_combo, blight_combo,
            rounds=rounds, players=players, fear_per_player=fear_per_player,
            fear_per_turn_mean=2.5,  # UNSOURCED placeholder, not a measured rate
            fear_per_turn_stdev=1.0,  # UNSOURCED placeholder
            blight_per_turn_mean=0.5,  # UNSOURCED placeholder
        )
        results.append(r)

    def series(key):
        return [r[key] for r in results]

    def token_series(tok):
        return [r["tokens_added"].get(tok, 0) for r in results]

    def stage_series(stage):
        return [r["fear_drawn_by_stage"].get(stage, 0) for r in results]

    agg = {
        "combo_name": combo_name,
        "expansions_included": keys,
        "trials": trials,
        "rounds_per_game": rounds,
        "players": players,
        "fear_per_card": fear_per_player * players,
        "fear_deck_layout": list(FEAR_DECK_LAYOUT),
        "pool_sizes": {
            "fear": len(fear_combo),
            "event": len(event_combo) if event_combo else 0,
            "blight": len(blight_combo),
        },
        "per_game": {
            # Key name kept for scripts/gen-stats-chapters.py; value = cards earned.
            "fear_cards_drawn": mean_ci(series("fear_drawn_total")),
            "fear_resolved_tl1": mean_ci(stage_series("stage_1")),
            "fear_resolved_tl2": mean_ci(stage_series("stage_2")),
            "fear_resolved_tl3": mean_ci(stage_series("stage_3")),
            "fear_output_total": mean_ci(series("fear_output")),
            "events_drawn": mean_ci(series("events_drawn")),
            "events_resolved": mean_ci(series("events_resolved")),
            "blight_cards_drawn": mean_ci(series("blight_cards_drawn")),
            "island_blighted": mean_ci(series("island_blighted")),
            "blight_loss_reached": mean_ci(series("blight_loss_reached")),
            "fear_victory_reached": mean_ci(series("fear_victory_reached")),
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
    p.add_argument("--fear-per-player", type=int, default=FEAR_PER_PLAYER,
                   help="Fear Markers per player per Fear Card: 4 by default; "
                        "5 with England L6 or an extra board (Q6)")
    p.add_argument("--seed", type=int, default=42)
    p.add_argument("--out-dir", type=Path, default=OUT_DIR,
                   help="where to write sim_<combo>.json (default: %(default)s)")
    args = p.parse_args()

    random.seed(args.seed)

    fear = deck_stats.load_deck(DECKS_DIR / "fear.json") if (DECKS_DIR / "fear.json").exists() else []
    event = deck_stats.load_deck(DECKS_DIR / "event.json") if (DECKS_DIR / "event.json").exists() else []
    blight = deck_stats.load_deck(DECKS_DIR / "blight.json") if (DECKS_DIR / "blight.json").exists() else []

    out_dir = args.out_dir
    out_dir.mkdir(parents=True, exist_ok=True)
    combined = {}
    for name, keys in COMBOS.items():
        print(f"Simulating {name} ({args.trials} trials, {args.rounds}-round, {args.players}p)...", file=sys.stderr)
        agg = run_combo(name, keys, fear, event, blight, args.trials, args.rounds,
                        args.players, args.fear_per_player)
        out = out_dir / f"sim_{name}.json"
        out.write_text(json.dumps(agg, indent=2) + "\n")
        print(f"  wrote {out}", file=sys.stderr)
        combined[name] = agg

    (out_dir / "_simulation_combined.json").write_text(json.dumps(combined, indent=2) + "\n")
    print(f"\n=== per-game summary across combos ({args.trials} trials × {args.rounds} rounds, "
          f"{args.players}p, {args.fear_per_player * args.players} Fear per card) ===")
    print(f"{'combo':<12} {'earned':>6} {'TL1/TL2/TL3':>15} {'fear_out':>8} {'ev d/r':>7} "
          f"{'blight':>6} {'blighted':>8} {'+favor':>7} {'-favor':>7} {'net':>7}")
    for name, a in combined.items():
        pg = a["per_game"]
        tl = f"{pg['fear_resolved_tl1']['mean']:.2f}/{pg['fear_resolved_tl2']['mean']:.2f}/{pg['fear_resolved_tl3']['mean']:.2f}"
        print(
            f"{name:<12} "
            f"{pg['fear_cards_drawn']['mean']:>6.2f} "
            f"{tl:>15} "
            f"{pg['fear_output_total']['mean']:>8.2f} "
            f"{pg['events_drawn']['mean']:>3.1f}/{pg['events_resolved']['mean']:<3.1f} "
            f"{pg['blight_cards_drawn']['mean']:>6.2f} "
            f"{pg['island_blighted']['mean']:>8.2f} "
            f"{pg['spirit_favor_hits']['mean']:>7.2f} "
            f"{pg['invader_favor_hits']['mean']:>7.2f} "
            f"{pg['net_favor']['mean']:>+7.2f}"
        )


if __name__ == "__main__":
    main()
