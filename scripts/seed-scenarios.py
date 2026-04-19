#!/usr/bin/env python3
"""
Seed per-scenario setup modification stubs as data/references/scenarios/<slug>.json.

Each file describes the scenario's setup deltas from a baseline game:
- starting invader changes
- special tokens/markers
- win/loss conditions modifications
- spirit-pick constraints (if any)

All 14 scenarios get stubs. Most details are [VERIFY] against physical
scenario cards; the well-known scenario mechanics (Blitz 6-round, Guard
the Isle's Heart, Dahan Insurrection) are described.

Usage:
    python3 scripts/seed-scenarios.py
"""
from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT_DIR = REPO / "data" / "references" / "scenarios"


def scen(slug, name, expansion, difficulty, setup, win, loss, notes=""):
    return {
        "slug": slug,
        "name": name,
        "expansion": expansion,
        "difficulty_delta": difficulty,
        "setup_modifications": setup,
        "win_condition_modifications": win,
        "loss_condition_modifications": loss,
        "notes": notes,
    }


SCENARIOS = [
    scen("blitz", "Blitz", "base", "+3",
         "Skip turn 1 of the invader deck (card 2 is revealed immediately as the first Explore).",
         "Standard — Terror 2 or higher at end.", "Standard.",
         "6-round game; Shadows is a strong fit because fear-rush lines up with time pressure."),
    scen("guard-the-isles-heart", "Guard the Isle's Heart", "base", "+0",
         "Designate one inner land as 'the Heart'; Invaders cannot be in the Heart at game end.",
         "The Heart is clear at game end AND standard Terror/Fear conditions.",
         "Standard + Heart breached.", "[VERIFY exact setup from scenario card]"),
    scen("rituals-of-terror", "Rituals of Terror", "base", "+3",
         "[VERIFY] Invaders perform ritual actions to reduce your Fear pool.",
         "Standard.", "Fear-count specific loss condition.",
         "[VERIFY physical card]"),
    scen("dahan-insurrection", "Dahan Insurrection", "branch-and-claw", "+1",
         "[VERIFY] Dahan revolt as a second faction against Invaders.",
         "Dahan-count win condition.", "Standard.", "[VERIFY]"),
    scen("second-wave", "Second Wave", "branch-and-claw", "+1",
         "[VERIFY] Invaders continue arriving after stage 3.", "Standard.", "Standard.", "[VERIFY]"),
    scen("powers-long-forgotten", "Powers Long Forgotten", "branch-and-claw", "+1",
         "[VERIFY] Additional forgotten Power cards enter the deck.", "Standard.", "Standard.", "[VERIFY]"),
    scen("ward-the-shores", "Ward the Shores", "jagged-earth", "+1",
         "[VERIFY] Coastal lands have extra protection requirements.", "Standard.", "Coastal Blight-count loss.", "[VERIFY]"),
    scen("rituals-of-destroying-flame", "Rituals of the Destroying Flame", "jagged-earth", "+3",
         "[VERIFY] Invaders perform fire rituals damaging land.", "Standard.", "Fire-count loss.", "[VERIFY]"),
    scen("despicable-theft", "Despicable Theft", "jagged-earth", "+0",
         "[VERIFY] Invaders steal resources triggering effects.", "Standard.", "Standard.", "[VERIFY]"),
    scen("elemental-invocation", "Elemental Invocation", "jagged-earth", "+0",
         "[VERIFY] Spirits must manifest elemental shrines.", "Shrine-count win condition.", "Standard.", "[VERIFY]"),
    scen("a-diversity-of-spirits", "A Diversity of Spirits", "jagged-earth", "+0",
         "Spirits must be from different archetypes — see scenario card.",
         "Standard.", "Standard.", "Spirit-pick constraint matters at draft time."),
    scen("the-great-river", "The Great River", "jagged-earth", "+1",
         "[VERIFY] River system modifies adjacency.", "Standard.", "Standard.", "[VERIFY]"),
    scen("varied-terrains", "Varied Terrains", "jagged-earth", "+0",
         "[VERIFY] Terrain rules modified.", "Standard.", "Standard.", "[VERIFY]"),
    scen("destiny-unfolds", "Destiny Unfolds", "nature-incarnate", "+2",
         "[VERIFY] Fate-driven triggers across rounds.", "Standard.", "Standard.", "[VERIFY]"),
]


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for s in SCENARIOS:
        out = OUT_DIR / f"{s['slug']}.json"
        out.write_text(json.dumps(s, indent=2) + "\n")
        print(f"wrote {out}")
    print(f"\n{len(SCENARIOS)} scenario stubs written. [VERIFY physical card] flags rows needing enrichment.")


if __name__ == "__main__":
    main()
