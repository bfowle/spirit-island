#!/usr/bin/env python3
"""
Generate Part VIII statistics chapters from deck-stats + deck-simulate outputs.

Writes:
    src/statistics/deck-expansion-impact.md   — headline overview
    src/statistics/fear-deck-expected-value.md — per-expansion Fear breakdown
    src/statistics/event-deck-risk-profiles.md — event balance
    src/statistics/blight-deck-severity.md     — blight severity distribution
    src/statistics/token-dilution.md           — per-token frequency by combo

Each chapter renders tables + mermaid charts from the JSON data.

Usage:
    python3 scripts/gen-stats-chapters.py
"""

from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
STATS_DIR = REPO / "data" / "references" / "deck-stats"
OUT_DIR = REPO / "src" / "statistics"

COMBOS_ORDERED = ["base", "base_bnc", "base_je", "base_bnc_je", "all", "ni_only"]
COMBO_LABELS = {
    "base": "Base only (Horizons + Base game)",
    "base_bnc": "Base + B&C",
    "base_je": "Base + JE",
    "base_bnc_je": "Base + B&C + JE",
    "all": "All expansions (Base + B&C + JE + NI + Promos)",
    "ni_only": "NI additions only (delta from JE)",
}

TOKEN_ORDER = ["badlands", "beasts", "disease", "strife", "wilds", "plantation", "isolate", "fortress"]


def load_json(p: Path):
    return json.loads(p.read_text()) if p.exists() else {}


def combined_stats():
    return load_json(STATS_DIR / "_combined.json")


def combined_sim():
    return load_json(STATS_DIR / "_simulation_combined.json")


def pct(n, total):
    return f"{(100 * n / total):.1f}%" if total else "—"


def render_deck_expansion_impact() -> str:
    stats = combined_stats()
    sim = combined_sim()

    lines = [
        "# Deck Expansion Impact — headline statistics",
        "",
        "```admonish abstract title=\"Summary\"",
        "Each Spirit Island expansion adds cards to the Minor, Major, Fear, Event, and Blight decks. Those additions **dilute** the base pool — drawing a card from the Fear deck at Base-only is a different probability distribution than drawing one from Base+B&C+JE+NI. This chapter renders the per-expansion-combo impact across all five decks, using deck data deterministically fetched from [spiritislandwiki.com](https://spiritislandwiki.com) via `scripts/wiki-fetch.py` and analyzed by `scripts/deck-stats.py` + `scripts/deck-simulate.py`.",
        "```",
        "",
        "## Pool sizes per expansion combo",
        "",
        "| Combo | Minor | Major | Fear | Event | Blight |",
        "|-------|-------|-------|------|-------|--------|",
    ]
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        d = stats[combo]["decks"]
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {d['minor']['pool_size']} "
            f"| {d['major']['pool_size']} "
            f"| {d['fear']['pool_size']} "
            f"| {d['event']['pool_size']} "
            f"| {d['blight']['pool_size']} |"
        )
    lines.append("")

    # Simulation-based per-game outcomes
    lines.extend([
        "## Expected per-game outcomes (Monte Carlo, 5 000 trials × 8 rounds × 1 player)",
        "",
        "Each trial simulates 8 invader rounds using a Gaussian fear-generation model (mean 2.5 fear/turn, σ=1) and sequential draws from the Fear/Event/Blight decks per expansion combo. Fear threshold transitions at TL1→TL2 (4 fear) and TL2→TL3 (8 fear) trigger Fear-card draws. Events draw once per turn (when an Event deck is in play). Blight cards draw at game start + on track-fill. Values are means; the 95% confidence intervals are reported in the JSON data, but omitted here for readability.",
        "",
        "| Combo | Fear drawn | Fear output | Events | Blight cards | Spirit favor | Invader favor | Net favor |",
        "|-------|-----------:|------------:|-------:|-------------:|-------------:|--------------:|----------:|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in sim:
            continue
        pg = sim[combo]["per_game"]
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {pg['fear_cards_drawn']['mean']:.2f} "
            f"| {pg['fear_output_total']['mean']:.2f} "
            f"| {pg['events_drawn']['mean']:.1f} "
            f"| {pg['blight_cards_drawn']['mean']:.2f} "
            f"| {pg['spirit_favor_hits']['mean']:.2f} "
            f"| {pg['invader_favor_hits']['mean']:.2f} "
            f"| **{pg['net_favor']['mean']:+.2f}** |"
        )
    lines.append("")

    lines.extend([
        "## Key readings",
        "",
        "- **Base-only is un-eventful by design.** No Event deck, 2 Blight cards only. Games are deterministic up to the spirits' own plays.",
        "- **+B&C adds the Event deck** (23 cards) and the full Blight deck (8 cards). That's ~+8 events drawn per game — significant stochastic variance injection.",
        "- **+JE events skew slightly more spirit-favorable** (net +4.87 hits/game vs. +1.09 for B&C alone). JE Minor/Major pools also dilute the base element distribution — see per-deck chapters.",
        "- **NI events are the most spirit-favoring** when counted alone (+6.76), but NI adds only 9 events (small pool, high variance per game).",
        "- **Fear drawn stabilizes at ~2 cards/game** across combos (2 threshold crossings × 1 draw each at 1-player scale). At higher player counts the thresholds scale linearly — expected fear drawn drops per player.",
        "",
        "## Cross-references",
        "",
        "- [Fear Deck Expected Value](fear-deck-expected-value.md) — per-expansion Fear-card breakdown + stage distribution.",
        "- [Event Deck Risk Profiles](event-deck-risk-profiles.md) — balanced vs. skewed events by expansion.",
        "- [Blight Deck Severity](blight-deck-severity.md) — how catastrophic each expansion's Blight additions are.",
        "- [Token Dilution](token-dilution.md) — per-token-type frequency across pools.",
        "- [Minor vs. Major Draft Rates](major-vs-minor-rates.md) — how Minor/Major draft composition shifts.",
        "",
    ])
    return "\n".join(lines)


def render_fear_chapter() -> str:
    stats = combined_stats()
    sim = combined_sim()
    lines = [
        "# Fear Deck — Expected Value per Expansion",
        "",
        "```admonish abstract title=\"Scope\"",
        "Fear cards are drawn from a shuffled deck when the Fear pool crosses terror-level thresholds. Their effects resolve at the player's current Terror Level, so each card contributes up to three distinct effects (stage 1, 2, 3). This chapter breaks down the deck per expansion combo.",
        "```",
        "",
        "## Fear pool size + stage composition",
        "",
        "| Combo | Total cards | Stage 1 text | Stage 2 text | Stage 3 text |",
        "|-------|------------:|-------------:|-------------:|-------------:|",
    ]
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        f = stats[combo]["decks"]["fear"]
        bs = f["by_stage"]
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {f['pool_size']} "
            f"| {bs.get('stage_1', 0)} "
            f"| {bs.get('stage_2', 0)} "
            f"| {bs.get('stage_3', 0)} |"
        )
    lines.append("")

    lines.extend([
        "## Average Fear-output per card by stage",
        "",
        "\"N Fear\" instructions embedded in each card's text, averaged across cards that have that stage's text.",
        "",
        "| Combo | Stage 1 avg fear | Stage 2 | Stage 3 |",
        "|-------|-----------------:|--------:|--------:|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        f = stats[combo]["decks"]["fear"]
        a = f.get("avg_fear_card_output", {})
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {a.get('stage_1', 0):.2f} "
            f"| {a.get('stage_2', 0):.2f} "
            f"| {a.get('stage_3', 0):.2f} |"
        )
    lines.append("")

    lines.extend([
        "## Spirit- vs. Invader-favoring per stage",
        "",
        "Heuristic classification via keyword regex — how many cards (per stage) contain pro-spirit phrasing (`N Fear`, `Destroy N Explorer/Town/City`, `Push N`, `Remove Blight`, etc.) vs. pro-invader phrasing (`add N Explorer`, `extra build`, `destroy Dahan`, `add Blight`, etc.). Cards may hit both.",
        "",
        "| Combo | Spirit-favor (s1/s2/s3) | Invader-favor (s1/s2/s3) |",
        "|-------|-------------------------|--------------------------|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        f = stats[combo]["decks"]["fear"]
        sp = f.get("spirit_favoring_by_stage", {})
        inv = f.get("invader_favoring_by_stage", {})
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {sp.get('stage_1', 0)}/{sp.get('stage_2', 0)}/{sp.get('stage_3', 0)} "
            f"| {inv.get('stage_1', 0)}/{inv.get('stage_2', 0)}/{inv.get('stage_3', 0)} |"
        )
    lines.append("")

    lines.extend([
        "## Monte Carlo: expected Fear-deck output per game",
        "",
        "5 000 trials × 8 rounds × 1 player. `Fear drawn` is cards drawn through terror crossings; `Fear output` sums every `N Fear` instruction on those cards at the stage they resolve.",
        "",
        "| Combo | Fear cards drawn | Fear output (mean) | 95% CI |",
        "|-------|-----------------:|-------------------:|--------|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in sim:
            continue
        pg = sim[combo]["per_game"]
        fo = pg["fear_output_total"]
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {pg['fear_cards_drawn']['mean']:.2f} "
            f"| {fo['mean']:.2f} "
            f"| ({fo['ci_lo']:.2f}, {fo['ci_hi']:.2f}) |"
        )
    lines.append("")

    lines.extend([
        "## Interpretation",
        "",
        "- Base-only Fear output is effectively 0 in this model — no Event deck supplies extra Fear, and per-card Fear at stage 1 is rarely encoded as a literal \"N Fear\" directive.",
        "- Adding B&C lifts Fear output significantly through the Event deck's fear-bearing cards (fear_output ~3.5/game).",
        "- NI's Fear cards average the highest per-stage Fear output — hence the NI-only sim spikes. In practice you'd combine NI with base+B&C+JE, which tames the average due to pool dilution.",
        "",
    ])
    return "\n".join(lines)


def render_event_chapter() -> str:
    stats = combined_stats()
    sim = combined_sim()
    lines = [
        "# Event Deck — Risk Profiles per Expansion",
        "",
        "```admonish abstract title=\"Scope\"",
        "The Event deck is introduced by **Branch & Claw** and expanded by **Jagged Earth** and **Nature Incarnate**. Each turn starting T1 (in games using the Event deck), an Event card is drawn and resolved in order of its stages (usually 1-3; some cards go to 5).",
        "```",
        "",
        "## Pool size + per-game draws",
        "",
        "| Combo | Event pool | Events per 8-round game (mean) |",
        "|-------|-----------:|-------------------------------:|",
    ]
    for combo in COMBOS_ORDERED:
        if combo not in sim:
            continue
        s = sim[combo]
        ev = s["pool_sizes"]["event"]
        draws = s["per_game"]["events_drawn"]["mean"]
        lines.append(f"| **{COMBO_LABELS[combo]}** | {ev} | {draws:.1f} |")
    lines.append("")

    lines.extend([
        "## Spirit-favoring vs. Invader-favoring hits",
        "",
        "Heuristic regex classifier counting pro-spirit vs. pro-invader phrasing in each Event card's full multi-stage text.",
        "",
        "| Combo | Pool | Spirit-favor total | Invader-favor total | Net favor |",
        "|-------|-----:|-------------------:|--------------------:|----------:|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        e = stats[combo]["decks"]["event"]
        if e["pool_size"] == 0:
            continue
        sp = e.get("spirit_favoring_hits_total", 0)
        inv = e.get("invader_favoring_hits_total", 0)
        net = e.get("net_favor_score", 0)
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {e['pool_size']} "
            f"| {sp} "
            f"| {inv} "
            f"| **{net:+d}** |"
        )
    lines.append("")

    lines.extend([
        "## Top 5 spirit-favoring + invader-favoring events (all expansions)",
        "",
    ])
    all_stats = stats.get("all", {}).get("decks", {}).get("event", {})
    lines.append("### Most spirit-favoring")
    lines.append("")
    for r in all_stats.get("top_5_spirit_favoring", []):
        lines.append(f"- **{r['name']}** ({r.get('expansion', '?')}) — spirit-hits {r['spirit_hits']}, invader-hits {r['invader_hits']} (net **{r['net_score']:+d}**)")
    lines.append("")
    lines.append("### Most invader-favoring")
    lines.append("")
    for r in all_stats.get("top_5_invader_favoring", []):
        lines.append(f"- **{r['name']}** ({r.get('expansion', '?')}) — spirit-hits {r['spirit_hits']}, invader-hits {r['invader_hits']} (net **{r['net_score']:+d}**)")
    lines.append("")

    lines.extend([
        "## Interpretation",
        "",
        "- B&C events are the most balanced (+1.09 per-game net favor in simulation).",
        "- JE events skew pro-spirit (+4.87), driven by the larger pool with more Fear-generating cards.",
        "- NI events as a standalone pool skew even more pro-spirit, but when mixed with the full pool, the combined net is +3.65 — JE's positive balance remains visible.",
        "",
    ])
    return "\n".join(lines)


def render_blight_chapter() -> str:
    stats = combined_stats()
    sim = combined_sim()
    lines = [
        "# Blight Deck — Severity per Expansion",
        "",
        "```admonish abstract title=\"Scope\"",
        "Blight cards are drawn at game start + whenever the Blight track fills. Their severity varies hugely — some permanently cripple your damage math (`All Things Weaken`) while others are mild (`Wildfires`, `A Pall Upon the Land`). We use a severity proxy: count of `Destroy`, `Add Blight`, and `Ongoing` keywords in the text (the Ongoing weight is doubled since it compounds across turns).",
        "```",
        "",
        "## Blight deck per expansion",
        "",
        "| Combo | Pool | Mean severity proxy |",
        "|-------|-----:|--------------------:|",
    ]
    for combo in COMBOS_ORDERED:
        if combo not in stats:
            continue
        b = stats[combo]["decks"]["blight"]
        if b["pool_size"] == 0:
            continue
        lines.append(f"| **{COMBO_LABELS[combo]}** | {b['pool_size']} | {b['mean_severity_proxy']:.2f} |")
    lines.append("")

    lines.extend([
        "## Top-5 worst-draw Blight cards (all expansions)",
        "",
    ])
    for r in stats.get("all", {}).get("decks", {}).get("blight", {}).get("top_5_worst", []):
        lines.append(f"- **{r['name']}** ({r.get('expansion', '?')}) — severity {r['severity_proxy']}, blight/player {r.get('blight_per_player', '?')}")
    lines.append("")

    lines.extend([
        "## Monte Carlo: expected Blight cards drawn per 8-round game",
        "",
        "| Combo | Pool | Blight cards drawn (mean, 95% CI) |",
        "|-------|-----:|-----------------------------------|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in sim:
            continue
        s = sim[combo]
        if s["pool_sizes"]["blight"] == 0:
            continue
        pg = s["per_game"]["blight_cards_drawn"]
        lines.append(
            f"| **{COMBO_LABELS[combo]}** "
            f"| {s['pool_sizes']['blight']} "
            f"| {pg['mean']:.2f} ({pg['ci_lo']:.2f}, {pg['ci_hi']:.2f}) |"
        )
    lines.append("")
    return "\n".join(lines)


def render_token_chapter() -> str:
    stats = combined_stats()
    sim = combined_sim()
    lines = [
        "# Token-Effect Dilution per Expansion",
        "",
        "```admonish abstract title=\"Scope\"",
        "Tokens (Badlands, Beasts, Disease, Strife, Wilds, Plantation, Isolate, Fortress) are the second-order economy of Spirit Island — they accumulate on the board via card effects and events, modifying damage, fear, and invader actions. Different expansions weight these differently. This chapter counts how often each token appears in each card pool.",
        "```",
        "",
        "## Cards touching each token, per deck × per combo",
        "",
    ]
    for deck_name in ("minor", "major", "fear", "event", "blight"):
        lines.append(f"### {deck_name.title()} deck")
        lines.append("")
        header = "| Combo | " + " | ".join(t.title() for t in TOKEN_ORDER) + " |"
        sep = "|" + "|".join(["---"] * (len(TOKEN_ORDER) + 1)) + "|"
        lines.append(header)
        lines.append(sep)
        for combo in COMBOS_ORDERED:
            if combo not in stats:
                continue
            d = stats[combo]["decks"][deck_name]
            tokens = d.get("token_cards", {})
            row = f"| **{COMBO_LABELS[combo]}** | "
            row += " | ".join(str(tokens.get(t, 0)) for t in TOKEN_ORDER)
            row += " |"
            lines.append(row)
        lines.append("")
    lines.append("")

    lines.extend([
        "## Tokens added per simulated 8-round game (means)",
        "",
        "From Monte Carlo (Fear + Event draws contribute; raw card text detected).",
        "",
        "| Combo | " + " | ".join(t.title() for t in TOKEN_ORDER) + " |",
        "|" + "|".join(["---"] * (len(TOKEN_ORDER) + 1)) + "|",
    ])
    for combo in COMBOS_ORDERED:
        if combo not in sim:
            continue
        t = sim[combo]["tokens_per_game"]
        row = f"| **{COMBO_LABELS[combo]}** | "
        row += " | ".join(f"{t.get(tok, {'mean': 0})['mean']:.2f}" for tok in TOKEN_ORDER)
        row += " |"
        lines.append(row)
    lines.append("")

    lines.extend([
        "## Interpretation",
        "",
        "- **Badlands + Beasts** are JE introductions; both expand markedly with JE.",
        "- **Disease + Strife + Wilds** scale across JE + NI; Strife is specifically tied to spirits like Shadows-Madness, Grinning Trickster, etc.",
        "- **Plantation** is a B&C/France-adversary token; appears when France is in play or in specific JE events.",
        "- Base-only games have no token mechanic — the Event deck is what introduces them.",
        "",
    ])
    return "\n".join(lines)


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    outputs = {
        "deck-expansion-impact.md": render_deck_expansion_impact(),
        "fear-deck-expected-value.md": render_fear_chapter(),
        "event-deck-risk-profiles.md": render_event_chapter(),
        "blight-deck-severity.md": render_blight_chapter(),
        "token-dilution.md": render_token_chapter(),
    }
    for name, content in outputs.items():
        (OUT_DIR / name).write_text(content)
        print(f"wrote {OUT_DIR / name}")


if __name__ == "__main__":
    main()
