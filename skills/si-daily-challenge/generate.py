#!/usr/bin/env python3
"""
si-daily-challenge generator.

Selects a Spirit Island daily drill (spirit × adversary × level × scenario ×
board × goal × player-count) from Brett's playlog + reference data.

Design:
  - Date-seeded RNG so a single day is stable; --reroll or --seed for variation.
  - Weighted selection prefers priority spirits and full-chapter spirits but
    still covers the full roster when the filter would be empty.
  - Adversary rotates through Brett's go-to pool (least-recently-played wins,
    random tiebreak).
  - Level walks that adversary's `difficulty_levels` ladder based on streaks.
  - Boards weighted toward base A-D; expansion boards surface ~30% of the time.
  - Override flags let Brett ask for a different spirit/adversary/goal or
    harder/easier without fighting the skill prose.

Invoked by SKILL.md. Prints a markdown card to stdout.
"""
from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import os
import random
import sys
from pathlib import Path

REPO = Path(os.path.expanduser("~/repos/spirit-island"))
DATA = REPO / "data"
PLAYLOG = DATA / "playlog.csv"
SPIRITS = DATA / "spirits.json"
ADVERSARIES = DATA / "adversaries.json"
SCENARIOS = DATA / "scenarios.json"
BOARDS_DIR = DATA / "boards"

GOTO_ADVERSARIES = [
    "england",
    "sweden",
    "brandenburg-prussia",
    "france-plantation-colony",
]

LEARNING_GOALS = [
    "Tempo audit — track per-turn energy/CP banked vs spent",
    "Major vs. Minor draft — follow the archetype bias for this spirit",
    "Fear generation pace — track fear per turn vs the Terror-2 target",
    "Opening adherence — follow Opening A from the spirit chapter; don't improvise T1–T3",
    "Card priority — grade each power card you see in offerings using the spirit chapter's grades",
    "Adversary pressure — identify the cliff turn before it hits",
    "Anti-alpha (if multiplayer) — no mid-turn suggestions for your partner",
]

GOAL_TO_FUNDAMENTAL = {
    "Tempo audit": "fundamentals/tempo.md",
    "Major vs. Minor draft": "fundamentals/major-vs-minor.md",
    "Fear generation pace": "fundamentals/fear-track.md",
    "Opening adherence": "fundamentals/tempo.md",
    "Card priority": "fundamentals/card-draft-theory.md",
    "Adversary pressure": "fundamentals/tempo.md",
    "Anti-alpha": "fundamentals/deliberate-play.md",
}

PLAYER_COUNT_MODES = [
    ("1 (true solo)", 1, 0.55),
    ("1 multi-handed × 2 spirits", 2, 0.30),
    ("1 multi-handed × 3 spirits", 3, 0.10),
    ("2 players × 2 spirits", 2, 0.05),
]


def load_json(path: Path) -> dict:
    with path.open() as fh:
        return json.load(fh)


def load_playlog() -> list[dict]:
    if not PLAYLOG.exists():
        return []
    with PLAYLOG.open() as fh:
        reader = csv.DictReader(fh)
        return list(reader)


def list_boards() -> list[dict]:
    boards = []
    for f in sorted(BOARDS_DIR.glob("*.json")):
        try:
            data = load_json(f)
        except Exception:
            continue
        boards.append(
            {
                "file": f.name,
                "id": data.get("board_id", f.stem),
                "expansion": data.get("expansion", "unknown"),
                "variants": list(data.get("variants", {}).keys()),
            }
        )
    return boards


def weighted_choice(rng: random.Random, items: list, weights: list[float]):
    if not items:
        return None
    total = sum(weights)
    if total <= 0:
        return rng.choice(items)
    r = rng.uniform(0, total)
    acc = 0.0
    for item, w in zip(items, weights):
        acc += w
        if r <= acc:
            return item
    return items[-1]


def recent_spirit_slugs(log: list[dict], n: int) -> set[str]:
    out: set[str] = set()
    for row in log[-n:]:
        spirits_cell = (row.get("spirits") or "").strip()
        if not spirits_cell:
            continue
        for s in spirits_cell.replace(";", ",").split(","):
            slug = s.strip().lower()
            if slug:
                out.add(slug)
    return out


def select_spirit(rng: random.Random, spirits: list[dict], log: list[dict], exclude: set[str]) -> dict:
    recent = recent_spirit_slugs(log, 5)
    blocked = recent | exclude

    def weight(s: dict) -> float:
        w = 1.0
        if s.get("brett_priority"):
            w += 2.0
        if s.get("chapter_status") == "full":
            w += 2.0
        return w

    candidates = [s for s in spirits if s.get("slug") not in blocked]
    if not candidates:
        candidates = [s for s in spirits if s.get("slug") not in exclude]
    if not candidates:
        candidates = list(spirits)
    weights = [weight(s) for s in candidates]
    return weighted_choice(rng, candidates, weights)


def select_adversary(rng: random.Random, adversaries: list[dict], log: list[dict], exclude: set[str]) -> dict:
    by_slug = {a["slug"]: a for a in adversaries}
    pool = [s for s in GOTO_ADVERSARIES if s in by_slug and s not in exclude]
    if not pool:
        pool = [a["slug"] for a in adversaries if a["slug"] not in exclude]
    if not pool:
        pool = [a["slug"] for a in adversaries]

    freq = {s: 0 for s in pool}
    for row in log[-10:]:
        slug = (row.get("adversary") or "").strip().lower()
        if slug in freq:
            freq[slug] += 1
    min_freq = min(freq.values())
    least = [s for s, c in freq.items() if c == min_freq]
    chosen = rng.choice(least)
    return by_slug[chosen]


def pick_level(rng: random.Random, adversary: dict, log: list[dict], delta: int) -> int:
    ladder = adversary.get("difficulty_levels") or [1]
    adv_slug = adversary["slug"]
    history = [r for r in log if (r.get("adversary") or "").strip().lower() == adv_slug]
    current_level = ladder[0]
    if history:
        last_level_raw = (history[-1].get("level") or "").strip()
        try:
            current_level = int(last_level_raw)
        except ValueError:
            current_level = ladder[0]

    step = 0
    last_two = history[-2:]
    if len(last_two) == 2 and all((r.get("result") or "").lower().startswith("w") for r in last_two) \
            and all((r.get("level") or "") == str(current_level) for r in last_two):
        step = +1
    elif len(last_two) == 2 and all((r.get("result") or "").lower().startswith("l") for r in last_two) \
            and all((r.get("level") or "") == str(current_level) for r in last_two):
        step = -1

    step += delta

    if current_level in ladder:
        idx = ladder.index(current_level)
    else:
        idx = 0
    idx = max(0, min(len(ladder) - 1, idx + step))
    return ladder[idx]


def select_scenario(rng: random.Random, scenarios: list[dict], want: bool | None) -> dict | None:
    if want is False:
        return None
    roll = rng.random()
    if want is None and roll > 0.20:
        return None
    easy = [s for s in scenarios if (s.get("difficulty_mod") or 0) <= 1]
    pool = easy if easy else scenarios
    weights = [max(1.0, 3.0 - (s.get("difficulty_mod") or 0)) for s in pool]
    return weighted_choice(rng, pool, weights)


def select_board(rng: random.Random, boards: list[dict], count: int) -> list[dict]:
    base = [b for b in boards if b.get("expansion") == "base"]
    exp = [b for b in boards if b.get("expansion") != "base"]
    chosen: list[dict] = []
    pool = list(boards)
    for _ in range(count):
        if not pool:
            break
        if rng.random() < 0.70 and base:
            pick = rng.choice(base)
        elif exp:
            pick = rng.choice(exp)
        else:
            pick = rng.choice(pool)
        chosen.append(pick)
        pool = [b for b in pool if b["id"] != pick["id"]]
        base = [b for b in base if b["id"] != pick["id"]]
        exp = [b for b in exp if b["id"] != pick["id"]]
    return chosen


def pick_goal(rng: random.Random, log: list[dict], exclude_idx: int | None = None) -> str:
    day_seed = dt.date.today().toordinal()
    ordered = LEARNING_GOALS[:]
    rng.shuffle(ordered)
    base_idx = day_seed % len(ordered)
    if exclude_idx is not None and len(ordered) > 1:
        base_idx = (base_idx + 1 + exclude_idx) % len(ordered)
    return ordered[base_idx]


def pick_player_count(rng: random.Random, forced: int | None) -> tuple[str, int]:
    if forced is not None:
        if forced == 1:
            return ("1 (true solo)", 1)
        return (f"1 multi-handed × {forced} spirits", forced)
    labels = [m[0] for m in PLAYER_COUNT_MODES]
    counts = [m[1] for m in PLAYER_COUNT_MODES]
    weights = [m[2] for m in PLAYER_COUNT_MODES]
    idx = weighted_choice(rng, list(range(len(labels))), weights)
    return (labels[idx], counts[idx])


def seed_for(date: dt.date, reroll: int) -> int:
    return int(date.strftime("%Y%m%d")) * 1000 + reroll


def format_card(ctx: dict) -> str:
    spirit = ctx["spirit"]
    adversary = ctx["adversary"]
    level = ctx["level"]
    scenario = ctx["scenario"]
    boards = ctx["boards"]
    goal = ctx["goal"]
    player_label = ctx["player_label"]
    today = ctx["date"].isoformat()

    spirit_path = spirit.get("chapter_path") or f"spirits/{spirit['slug']}.md"
    fund_fragment = None
    for k, v in GOAL_TO_FUNDAMENTAL.items():
        if goal.startswith(k):
            fund_fragment = v
            break
    fund_line = f"- Fundamentals: [{fund_fragment.split('/')[-1].replace('.md','')}](../src/{fund_fragment})" if fund_fragment else ""

    adv_path = adversary.get("chapter_path") or f"adversaries/{adversary['slug']}.md"

    scenario_line = "none" if scenario is None else f"{scenario['name']} (diff {scenario.get('difficulty_mod', 0):+d})"
    board_line = ", ".join(f"{b['id']} ({b['expansion']})" for b in boards) if boards else "player choice"

    lines = [
        f"## Today's Challenge — {today}",
        "",
        f"**Spirit**: {spirit['name']} ({spirit.get('complexity','?')}, {spirit.get('draft_bias','?')}-draft)",
        f"**Adversary**: {adversary['name']} Level {level}",
        f"**Scenario**: {scenario_line}",
        f"**Board**: {board_line}",
        f"**Player count**: {player_label}",
        f"**Learning goal**: {goal}",
        "",
        "### Read first (15 min)",
        f"- Spirit chapter: [{spirit['slug']}](../src/{spirit_path})",
    ]
    if fund_line:
        lines.append(fund_line)
    lines.append(f"- Adversary chapter: [{adversary['name']}](../src/{adv_path})")

    chapter_note = ""
    if spirit.get("chapter_status") != "full":
        chapter_note = " _(stub chapter — rely on [spirit index](../src/spirits/index.md) + Wiki; post-game notes will help flesh it out.)_"
    if chapter_note:
        lines.append(chapter_note)

    lines += [
        "",
        "### Turn-1 commit",
        f"Before you start, write on paper: *\"By T3 I will have {{concrete milestone from the {spirit['name']} Tempo Profile}}.\"*",
        "",
        "### After the game",
        "Run `si-post-game` to log; tomorrow's challenge shifts based on results.",
    ]
    return "\n".join(lines)


def parse_args():
    p = argparse.ArgumentParser(description="Generate today's Spirit Island challenge.")
    p.add_argument("--reroll", type=int, default=0, help="Bump the daily seed; each reroll gives a new draw.")
    p.add_argument("--seed", type=int, default=None, help="Override the full seed (for testing).")
    p.add_argument("--date", type=str, default=None, help="Override today's date (YYYY-MM-DD).")
    p.add_argument("--different-spirit", action="store_true", help="Exclude the top spirit result.")
    p.add_argument("--different-adversary", action="store_true", help="Exclude the top adversary result.")
    p.add_argument("--different-goal", action="store_true", help="Skip the first-rotated goal.")
    p.add_argument("--spirit", type=str, default=None, help="Force a spirit slug.")
    p.add_argument("--adversary", type=str, default=None, help="Force an adversary slug.")
    p.add_argument("--level", type=int, default=None, help="Force a difficulty level.")
    p.add_argument("--players", type=int, default=None, help="Force player count (1–4).")
    p.add_argument("--harder", action="store_true", help="Bump level +1 on the ladder.")
    p.add_argument("--easier", action="store_true", help="Bump level −1 on the ladder.")
    p.add_argument("--scenario", type=str, default=None, help="Force a scenario slug; 'none' disables.")
    p.add_argument("--no-scenario", action="store_true", help="Force no scenario.")
    p.add_argument("--json", action="store_true", help="Emit raw JSON instead of markdown.")
    return p.parse_args()


def main() -> int:
    args = parse_args()
    today = dt.date.fromisoformat(args.date) if args.date else dt.date.today()
    seed = args.seed if args.seed is not None else seed_for(today, args.reroll)
    rng = random.Random(seed)

    spirits = load_json(SPIRITS)["spirits"]
    adversaries = load_json(ADVERSARIES)["adversaries"]
    scenarios = load_json(SCENARIOS)["scenarios"]
    boards = list_boards()
    log = load_playlog()

    # Spirit
    spirit = None
    if args.spirit:
        spirit = next((s for s in spirits if s["slug"] == args.spirit), None)
        if spirit is None:
            print(f"Unknown spirit slug: {args.spirit}", file=sys.stderr)
            return 2
    else:
        exclude_spirits: set[str] = set()
        spirit = select_spirit(rng, spirits, log, exclude_spirits)
        if args.different_spirit and spirit:
            spirit = select_spirit(rng, spirits, log, {spirit["slug"]})

    # Adversary
    adversary = None
    if args.adversary:
        adversary = next((a for a in adversaries if a["slug"] == args.adversary), None)
        if adversary is None:
            print(f"Unknown adversary slug: {args.adversary}", file=sys.stderr)
            return 2
    else:
        exclude_adv: set[str] = set()
        adversary = select_adversary(rng, adversaries, log, exclude_adv)
        if args.different_adversary and adversary:
            adversary = select_adversary(rng, adversaries, log, {adversary["slug"]})

    # Level
    if args.level is not None:
        level = args.level
    else:
        delta = 0
        if args.harder:
            delta += 1
        if args.easier:
            delta -= 1
        level = pick_level(rng, adversary, log, delta)

    # Scenario
    scenario = None
    if args.scenario and args.scenario.lower() != "none":
        scenario = next((s for s in scenarios if s["slug"] == args.scenario), None)
        if scenario is None:
            print(f"Unknown scenario slug: {args.scenario}", file=sys.stderr)
            return 2
    elif args.no_scenario or (args.scenario and args.scenario.lower() == "none"):
        scenario = None
    else:
        scenario = select_scenario(rng, scenarios, want=None)

    # Player count + boards
    player_label, player_count = pick_player_count(rng, args.players)
    board_count = player_count if player_count <= 4 else 4
    chosen_boards = select_board(rng, boards, board_count)

    # Goal
    goal = pick_goal(rng, log, exclude_idx=0 if args.different_goal else None)

    ctx = {
        "date": today,
        "seed": seed,
        "spirit": spirit,
        "adversary": adversary,
        "level": level,
        "scenario": scenario,
        "boards": chosen_boards,
        "goal": goal,
        "player_label": player_label,
        "player_count": player_count,
    }

    if args.json:
        serialisable = {
            "date": today.isoformat(),
            "seed": seed,
            "spirit": spirit,
            "adversary": adversary,
            "level": level,
            "scenario": scenario,
            "boards": chosen_boards,
            "goal": goal,
            "player_label": player_label,
            "player_count": player_count,
        }
        print(json.dumps(serialisable, indent=2))
    else:
        print(format_card(ctx))
    return 0


if __name__ == "__main__":
    sys.exit(main())
