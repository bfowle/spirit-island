#!/usr/bin/env python3
"""
Generate a mechanically-verified "Opener Mechanics" starter section for each
spirit chapter. Derives starting E/CP from the presence tracks, enumerates
growth-option branches, lists Unique cards with cost/speed/elements, states
each innate's speed + threshold demands, and flags phase-timing constraints.

Not a full Rei-format strategic opener — just the factual mechanics every
opener must build on top of, so authors/readers cannot accidentally misstate
starting state or innate-firing timing.

Usage:
    python3 scripts/gen-opener-starter.py <slug>
    python3 scripts/gen-opener-starter.py --all
"""

from __future__ import annotations
import argparse, json, re, sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
WIKI_DIR = REPO / "data" / "references" / "wiki"
DRAFT_DIR = REPO / "data" / "references" / "draft-priority"
SPIRITS_JSON = REPO / "data" / "spirits.json"
OUT_DIR = REPO / "data" / "references" / "opener-mechanics"

ELEMENTS = ("sun", "moon", "fire", "air", "water", "earth", "plant", "animal")

GROWTH_TOKEN_DESC = {
    "reclaim": "Reclaim all discarded+played Power Cards",
    "reclaim1": "Reclaim 1 Power Card (of your choice)",
    "gain1p": "Gain 1 Power Card (Minor unless otherwise noted)",
    "gain2p": "Gain 2 Power Cards",
    "energy0": "(track slot showing 0 Energy)",
    "energy1": "(track slot showing 1 Energy)",
    "energy2": "(track slot showing 2 Energy)",
    "energy3": "(+3 Energy this turn — growth effect, not track reveal)",
}


def parse_int(s, default=0):
    if s is None:
        return default
    try:
        return int(s)
    except (ValueError, TypeError):
        return default


def decode_growth_token(tok):
    if not tok:
        return None
    if tok in GROWTH_TOKEN_DESC:
        return GROWTH_TOKEN_DESC[tok]
    m = re.match(r"addpresence(\d+)$", tok)
    if m:
        return f"Place 1 Presence from a track (Range {m.group(1)})"
    m = re.match(r"energy(\d+)$", tok)
    if m:
        return f"+{m.group(1)} Energy this turn (growth effect)"
    m = re.match(r"card(\d+)$", tok)
    if m:
        return f"(CP track slot showing {m.group(1)} Card Plays)"
    m = re.match(r"movepresence(\d+)$", tok)
    if m:
        return f"Move 1 Presence (Range {m.group(1)})"
    return f"(spirit-specific: `{tok}` — consult spirit panel)"


def format_growth(g):
    """Render a single growth option as markdown bullet."""
    idx = g.get("index")
    effects = []
    for key in ("first", "second", "third"):
        val = g.get(key)
        if val:
            desc = decode_growth_token(val) or val
            effects.append(f"{val} ({desc})")
    return f"- **G{idx}**: " + "; ".join(effects) if effects else f"- **G{idx}**: (no effects parsed)"


def starting_income(energy_track, cp_track):
    """Return (starting_E, starting_CP) from the first uncovered slots."""
    def first_val(track):
        if not track:
            return 0
        for t in track:
            m = re.match(r"energy(\d+)", t)
            if m:
                return int(m.group(1))
            m = re.match(r"card(\d+)", t)
            if m:
                return int(m.group(1))
        return 0
    return first_val(energy_track), first_val(cp_track)


def render_innate(innate):
    name = innate.get("name") or "(unnamed innate)"
    speed = (innate.get("speed") or "?").title()
    rng = innate.get("range") or "?"
    target = innate.get("target") or "?"
    thresholds = innate.get("thresholds") or []
    lines = [f"- **{name}** (Speed: {speed} · Range: {rng} · Target: {target})"]
    for i, thr in enumerate(thresholds):
        levels = []
        for elem in ELEMENTS:
            v = thr.get(elem)
            if v:
                levels.append(f"{v} {elem.title()}")
        effect = (thr.get("effect") or "").replace("\n", " ")
        lines.append(f"  - **L{i+1}** — {' + '.join(levels) if levels else '(no threshold)'}: {effect}")
    return "\n".join(lines)


def render_unique_table(details):
    if not details:
        return "*No Unique cards parsed.*"
    lines = ["| Card | Cost | Speed | Range | Target | Elements | Effect |",
             "|------|------|-------|-------|--------|----------|--------|"]
    for c in details:
        if "error" in c:
            continue
        elems = ", ".join((c.get("elements") or []))
        txt = (c.get("text") or "").replace("\n", " ").replace("|", "\\|")
        if len(txt) > 100:
            txt = txt[:99] + "…"
        lines.append(
            f"| **{c.get('name','?')}** | {c.get('cost','?')} | {c.get('speed','?')} | {c.get('range','?')} | {c.get('target','?')} | {elems} | {txt} |"
        )
    return "\n".join(lines)


def compute_fast_phase_ceiling(spirit):
    """For each element, what's the max Fast-phase contribution from Uniques only?"""
    uniques = spirit.get("unique_card_details") or []
    ceiling = {e: 0 for e in ELEMENTS}
    for c in uniques:
        if (c.get("speed") or "").lower() != "fast":
            continue
        for e in (c.get("elements") or []):
            e = e.lower()
            if e in ceiling:
                ceiling[e] += 1
    return ceiling


def render_section(slug):
    wiki_path = WIKI_DIR / f"{slug}.json"
    if not wiki_path.exists():
        raise FileNotFoundError(f"{wiki_path} not found")
    spirit = json.loads(wiki_path.read_text())

    name = spirit.get("name") or slug
    energy_track = spirit.get("presence_energy_track") or []
    cp_track = spirit.get("presence_cardplay_track") or []
    start_e, start_cp = starting_income(energy_track, cp_track)
    growth_type = spirit.get("growth_type") or "?"
    growths = spirit.get("growths") or []
    innates = spirit.get("innates") or []
    setup = spirit.get("setup") or "(see spirit panel)"

    # Fast-phase element ceiling
    fast_ceil = compute_fast_phase_ceiling(spirit)
    primary_innate_speed = (innates[0].get("speed") or "").lower() if innates else ""

    # Any innate has its threshold elements
    needed_elements = set()
    for innate in innates:
        for thr in innate.get("thresholds") or []:
            for e in ELEMENTS:
                if thr.get(e):
                    needed_elements.add(e)

    # Does Fast-phase ceiling cover L1 threshold from Uniques alone?
    fast_coverage_note = ""
    if primary_innate_speed == "fast" and innates:
        l1 = innates[0].get("thresholds", [{}])[0] if innates[0].get("thresholds") else {}
        shortfalls = []
        for e in ELEMENTS:
            need = parse_int(l1.get(e), 0)
            if need > 0:
                have = fast_ceil.get(e, 0)
                if have < need:
                    shortfalls.append(f"need {need} {e.title()}, Uniques give {have}")
        if shortfalls:
            fast_coverage_note = (
                f"**Fast-phase L1 ceiling from Uniques alone is insufficient** — "
                + "; ".join(shortfalls)
                + ". L1 only fires T1 with a drafted Fast Minor providing the shortfall element(s)."
            )
        else:
            fast_coverage_note = "**Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold)."

    # Build the section
    lines = [
        "## Opener Mechanics — starter reference",
        "",
        "```admonish abstract title=\"Mechanically-verified starting state\"",
        f"Auto-derived from `data/references/wiki/{slug}.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.",
        "```",
        "",
        "### Starting state",
        "",
        f"- **Setup**: {setup}",
        f"- **Starting income** (from `presence_energy_track[0]` = `{energy_track[0] if energy_track else '—'}`, `presence_cardplay_track[0]` = `{cp_track[0] if cp_track else '—'}`): **{start_e} Energy · {start_cp} Card Play**",
        f"- **Hand at start**: 4 Unique Power Cards (listed below)",
        f"- **Growth type**: `{growth_type}` — " + ("pick **one** growth option per turn" if growth_type == "one" else "resolve **all** listed growth options each turn" if growth_type == "all" else "(see spirit panel)"),
        "",
        "### Growth options",
        "",
    ]
    for g in growths:
        lines.append(format_growth(g))
    lines.append("")
    lines.append("**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.")
    lines.append("")
    lines.append("### Energy track")
    lines.append("")
    lines.append(f"`{' · '.join(energy_track) if energy_track else '(not parsed)'}` — income as slots reveal: " + " → ".join(
        str(parse_int(re.match(r'energy(\d+)', t).group(1))) if re.match(r'energy(\d+)', t) else t
        for t in energy_track
    ) if energy_track else "(not parsed)")
    lines.append("")
    lines.append("### Card-play track")
    lines.append("")
    lines.append(f"`{' · '.join(cp_track) if cp_track else '(not parsed)'}` — CP as slots reveal: " + " → ".join(
        str(parse_int(re.match(r'card(\d+)', t).group(1))) if re.match(r'card(\d+)', t) else t
        for t in cp_track
    ) if cp_track else "(not parsed)")
    lines.append("")
    lines.extend([
        "### Innate Powers",
        "",
    ])
    for innate in innates:
        lines.append(render_innate(innate))
    lines.append("")

    lines.extend([
        "### Fast-phase element ceiling from Uniques",
        "",
        "Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:",
        "",
    ])
    fast_line = []
    for e in ELEMENTS:
        v = fast_ceil.get(e, 0)
        if v > 0:
            fast_line.append(f"**{e.title()}** ×{v}")
    lines.append("- Fast-phase Unique elements: " + (", ".join(fast_line) if fast_line else "_(no Fast Uniques — all innate firings require drafted Fast cards)_"))
    if fast_coverage_note:
        lines.append(f"- {fast_coverage_note}")
    lines.append("")

    lines.extend([
        "### Unique Power Cards",
        "",
        render_unique_table(spirit.get("unique_card_details") or []),
        "",
        "### Invader phase by turn (base deck)",
        "",
        "| Turn | Explore | Build | Ravage | Notes |",
        "|------|---------|-------|--------|-------|",
        "| 1 | ✓ | — | — | Ravage-protection effects are **dormant T1**. |",
        "| 2 | ✓ | ✓ | — | First Build; Ravage-protection still dormant. |",
        "| 3 | ✓ | ✓ | ✓ | First Ravage; Ravage-protection becomes material. |",
        "| 4+ | ✓ | ✓ | ✓ | Full cycle continues. |",
        "",
        "Adversary escalation can shift this — check the adversary JSON for deviations (Sweden front-loads a Build; some Habsburg levels add early Builds).",
        "",
        "### Pause-point before writing T1 prose",
        "",
        "```admonish warning title=\"Before claiming what T1 does\"",
        "1. **Compute post-growth E/CP** for every growth × track-choice branch. Don't assume both tracks reveal simultaneously.",
        "2. **Enumerate legal T1 plays** — subsets of hand with sum(costs) ≤ E and count ≤ CP.",
        "3. **Separate Fast vs. Slow elements** — when claiming an innate fires, verify the threshold is met using only elements from its resolution phase (Fast sees Fast; Slow sees Fast + Slow).",
        "4. **Flag dormant effects** — Ravage-protection, Defend N, etc. are **null T1/T2** in base play. Only cite them as opener value when the trigger actually occurs that turn.",
        "5. **State per-turn material effect** for every card play: Fear generated, units pushed/gathered/destroyed, elements contributed. Never narrate dormant effects as if they were active.",
        "```",
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
                section = render_section(slug)
                (OUT_DIR / f"{slug}.md").write_text(section)
                count += 1
            except FileNotFoundError:
                pass
        print(f"wrote {count} opener-mechanics sections to {OUT_DIR}/", file=sys.stderr)
    elif args.slug:
        print(render_section(args.slug))
    else:
        p.error("specify slug or --all")


if __name__ == "__main__":
    main()
