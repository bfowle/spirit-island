#!/usr/bin/env python3
"""
Seed per-level adversary escalation data as data/references/adversaries/<slug>.json.

Each file contains the adversary's difficulty levels with escalation text
describing what rule changes at that level. Key cliffs are called out so the
`si-live` setup wizard and `si-rules-check` skill can cite them.

Coverage is intentionally partial on first pass — the most-cited cliffs
(England L3 coastal, England L5 HP bump, Sweden fear suppression, Russia
dahan pressure, etc.) are captured; the rest stays as [VERIFY physical panel].

Usage:
    python3 scripts/seed-adversary-levels.py
"""
from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT_DIR = REPO / "data" / "references" / "adversaries"


def lvl(n: int, name: str, escalation: str, cliff: bool = False, notes: str = ""):
    return {"level": n, "name": name, "escalation": escalation, "is_cliff": cliff, "notes": notes}


ADVERSARIES = {
    "england": {
        "slug": "england", "name": "England",
        "expansion": "base",
        "pressure_shape": "slow builds, long tail; Towns steadily become Cities; L5 HP bump",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Contempt", "Coastal Lands: additional Town during Setup.", False, "[VERIFY Wiki]"),
            lvl(2, "Endless Influx", "Additional Invader card into deck each turn.", True,
                "Raw Invader pressure jumps — plan for faster board crawl"),
            lvl(3, "Shores of Wealth", "Coastal Lands Build every Invader Phase.", True,
                "The classic England T1 cliff — coastal pressure compounds"),
            lvl(4, "Puppet Government", "Towns grow into Cities.", True,
                "City density spikes — damage-only plays fall off"),
            lvl(5, "Sustained Immigration", "Buildings gain +1 HP (Town = 3, City = 4).", True,
                "Damage-math cliff: 3-damage innates no longer 1-shot 3-HP Towns"),
            lvl(6, "Ironclads", "Buildings are harder to kill.", False, "[VERIFY physical panel]"),
            lvl(7, "Steamrolled", "Invaders ravage more aggressively.", False, "[VERIFY physical panel]"),
        ],
    },
    "brandenburg-prussia": {
        "slug": "brandenburg-prussia", "name": "Brandenburg-Prussia",
        "expansion": "base",
        "pressure_shape": "flat-early, steep-late; Cities on board early; fear-per-kill bonuses",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Professional Soldiers", "Additional Town in Setup (per adversary panel).", False),
            lvl(2, "Mercenaries", "Cities deal +1 Damage during Ravages.", True,
                "Ravage damage ramps faster — Defend plays become load-bearing"),
            lvl(4, "Disciplined Ranks", "City defense + more Towns.", True, "[VERIFY]"),
            lvl(6, "Inexorable Advance", "Heavy city/town pressure.", False, "[VERIFY]"),
            lvl(7, "Total War", "Near-constant City pressure on coasts.", False, "[VERIFY]"),
            lvl(9, "Industrialization", "Late-game spike.", False, "[VERIFY]"),
            lvl(10, "Imperial Ambitions", "Maximum pressure.", False, "[VERIFY]"),
        ],
    },
    "sweden": {
        "slug": "sweden", "name": "Sweden",
        "expansion": "base",
        "pressure_shape": "high-early plateau; fear-card penalties; coastal Build early",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Prosperity", "Build column shifts earlier.", False, "[VERIFY]"),
            lvl(2, "Fear-Dampened Colonies", "Fear-card effects reduced at this and higher levels.", True,
                "Fear-rush spirits stall — shift to board-control archetype"),
            lvl(3, "Homesteaders", "Additional Dahan converted to settlers during Setup.", True,
                "Dahan-reliant spirits downgraded"),
            lvl(5, "Wave of Colonists", "Heavier coastal Builds.", False, "[VERIFY]"),
            lvl(6, "Firm Foothold", "[VERIFY physical panel]", False),
            lvl(7, "Entrenched", "[VERIFY physical panel]", False),
            lvl(8, "Massive Colonization", "Maximum pressure.", False),
        ],
    },
    "france-plantation-colony": {
        "slug": "france-plantation-colony", "name": "France (Plantation Colony)",
        "expansion": "branch-and-claw",
        "pressure_shape": "Dahan capture + plantation-tokens; punishes Dahan-reliance",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(2, "Slavery", "Dahan captured → Settlers/Plantations.", True,
                "Dahan-engine spirits (Shadows-of-Dahan, Thunderspeaker) lose count"),
            lvl(3, "Sugar", "Plantation tokens added during Ravage.", True,
                "Blight accelerates from plantation density"),
            lvl(5, "Entrenched Estates", "Plantations harder to remove.", False, "[VERIFY]"),
            lvl(6, "Coffee", "[VERIFY]", False),
            lvl(8, "Tobacco", "[VERIFY]", False),
            lvl(9, "Indentured Workforce", "[VERIFY]", False),
            lvl(10, "Total Plantation", "Maximum pressure.", False),
        ],
    },
    "russia": {
        "slug": "russia", "name": "Russia",
        "expansion": "jagged-earth",
        "pressure_shape": "Dahan pressure + fear-card suppression from mid-late",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Frontier Warlords", "[VERIFY]", False),
            lvl(3, "Colonization Drive", "Fear-card penalties + Dahan under pressure.", True,
                "Fear-rush + Dahan-reliant spirits both compromised at this level"),
            lvl(4, "Settler Attacks", "Dahan directly targeted.", True, "[VERIFY]"),
            lvl(6, "Conscription", "[VERIFY]", False),
            lvl(7, "Peasant Levies", "[VERIFY]", False),
            lvl(9, "Grand Army", "[VERIFY]", False),
            lvl(11, "Maximum Pressure", "[VERIFY]", False),
        ],
    },
    "scotland": {
        "slug": "scotland", "name": "Scotland",
        "expansion": "promo-2",
        "pressure_shape": "mixed; mild Town/City escalation; few hard cliffs",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Second Sons", "[VERIFY]", False),
            lvl(3, "Clearances", "[VERIFY]", True, "Possible Dahan pressure cliff"),
            lvl(4, "Defoliation", "[VERIFY]", False),
            lvl(6, "Resources Drained", "[VERIFY]", False),
            lvl(7, "[Verify]", "[VERIFY]", False),
            lvl(8, "[Verify]", "[VERIFY]", False),
            lvl(10, "Total", "[VERIFY]", False),
        ],
    },
    "habsburg-mining-expedition": {
        "slug": "habsburg-mining-expedition", "name": "Habsburg Mining Expedition",
        "expansion": "nature-incarnate",
        "pressure_shape": "scaling Explorer/Town late; aggressive Build counts",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(1, "Quick Profit", "[VERIFY]", False),
            lvl(3, "Surface Mining", "[VERIFY]", False),
            lvl(5, "Industrial Scaling", "Extra Explorers/Towns each turn.", True,
                "Raw Invader accumulation outpaces fear-rush"),
            lvl(6, "Heavy Machinery", "[VERIFY]", False),
            lvl(7, "Deep Shaft", "[VERIFY]", False),
            lvl(9, "Exhausting the Land", "[VERIFY]", False),
            lvl(10, "Maximum Pressure", "[VERIFY]", False),
        ],
    },
    "habsburg-livestock-colony": {
        "slug": "habsburg-livestock-colony", "name": "Habsburg Livestock Colony",
        "expansion": "jagged-earth",
        "pressure_shape": "Livestock tokens, slow-ramp; favorable to most spirits",
        "levels": [
            lvl(0, "Base", "No escalation.", False),
            lvl(2, "Cattle Drives", "[VERIFY]", False),
            lvl(3, "Grazing Expansion", "Livestock tokens added.", True, "[VERIFY]"),
            lvl(5, "Large Herds", "[VERIFY]", False),
            lvl(6, "Organized Ranches", "[VERIFY]", False),
            lvl(8, "Grand Haciendas", "[VERIFY]", False),
            lvl(9, "[VERIFY]", "[VERIFY]", False),
            lvl(10, "Maximum", "[VERIFY]", False),
        ],
    },
}


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for slug, data in ADVERSARIES.items():
        out = OUT_DIR / f"{slug}.json"
        out.write_text(json.dumps(data, indent=2) + "\n")
        print(f"wrote {out}")
    print(f"\nAll {len(ADVERSARIES)} adversary stubs written. [VERIFY physical panel] markers flag rows needing enrichment.")


if __name__ == "__main__":
    main()
