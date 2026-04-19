#!/usr/bin/env python3
"""
Seed stub board-geometry JSONs for boards B/C/D (base) + E/F/G/H (JE).

Each board is hand-seeded from general Spirit Island layout knowledge: 8 lands
numbered 1-8, one ocean (land 0), mix of mountain/wetland/jungle/sands
terrains, adjacency graph preserving the 4-neighbor topology the physical
boards have.

All data is marked NEEDS_PHYSICAL_VERIFICATION — the numbers are placeholders
the user corrects via the si-live UI on the first game-setup pass. The
schema matches data/boards/base_A.json so the backend can consume them
immediately.

Usage:
    python3 scripts/seed-boards.py
"""
from __future__ import annotations
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
BOARDS_DIR = REPO / "data" / "boards"


def build_board(board_id: str, expansion: str, terrain_pattern: list[tuple[int, str, bool]],
                adjacencies: dict[str, list[str]]) -> dict:
    """Build the board JSON. terrain_pattern = list of (land_num, terrain, coastal)."""
    lands: dict[str, dict] = {
        "0": {"terrain": "ocean", "coastal": False, "notes": "Ocean - adjacent to coastal lands"}
    }
    for n, terrain, coastal in terrain_pattern:
        lands[str(n)] = {
            "terrain": terrain,
            "coastal": coastal,
            "starting_dahan": 0,  # user fills in via UI
        }
    return {
        "source": "scripts/seed-boards.py — NEEDS_PHYSICAL_VERIFICATION",
        "board_id": board_id,
        "expansion": expansion,
        "lands": lands,
        "adjacencies": adjacencies,
        "base_invader_setup": {"1_explorer": "1", "2_town": "2", "3_city": "3"},
        "notes": "Stub geometry — user verifies + adjusts via UI on first setup.",
    }


# Base-game boards B, C, D use 4-neighbor topology similar to A.
# Terrain patterns are approximate; user corrects on first setup.
BASE_B = build_board(
    "B", "base",
    [
        (1, "mountain", False), (2, "jungle", False), (3, "sands", True),
        (4, "wetland", True), (5, "jungle", False), (6, "wetland", False),
        (7, "mountain", True), (8, "sands", True),
    ],
    {
        "0": ["3", "4", "7", "8"],
        "1": ["2", "5", "6"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5", "7"], "5": ["1", "2", "4", "6"],
        "6": ["1", "5", "7", "8"], "7": ["0", "4", "6", "8"],
        "8": ["0", "6", "7"],
    },
)

BASE_C = build_board(
    "C", "base",
    [
        (1, "sands", False), (2, "jungle", False), (3, "mountain", True),
        (4, "jungle", True), (5, "wetland", False), (6, "mountain", False),
        (7, "wetland", True), (8, "sands", True),
    ],
    {
        "0": ["3", "4", "7", "8"],
        "1": ["2", "5", "6"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5"], "5": ["1", "2", "4", "6", "7"],
        "6": ["1", "5", "7", "8"], "7": ["0", "5", "6", "8"],
        "8": ["0", "6", "7"],
    },
)

BASE_D = build_board(
    "D", "base",
    [
        (1, "jungle", False), (2, "wetland", False), (3, "sands", True),
        (4, "mountain", True), (5, "sands", False), (6, "jungle", False),
        (7, "wetland", True), (8, "mountain", True),
    ],
    {
        "0": ["3", "4", "7", "8"],
        "1": ["2", "5", "6"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5"], "5": ["1", "2", "4", "6", "7"],
        "6": ["1", "5", "7", "8"], "7": ["0", "5", "6", "8"],
        "8": ["0", "6", "7"],
    },
)

# Jagged Earth boards E, F, G, H follow similar 8-land topology.
JE_E = build_board(
    "E", "jagged-earth",
    [
        (1, "jungle", False), (2, "mountain", False), (3, "wetland", True),
        (4, "sands", True), (5, "mountain", False), (6, "jungle", True),
        (7, "wetland", False), (8, "sands", False),
    ],
    {
        "0": ["3", "4", "6"],
        "1": ["2", "5", "7"], "2": ["1", "3", "5"], "3": ["0", "2", "4", "6"],
        "4": ["0", "3", "6"], "5": ["1", "2", "7", "8"],
        "6": ["0", "3", "4", "8"], "7": ["1", "5", "8"], "8": ["5", "6", "7"],
    },
)

JE_F = build_board(
    "F", "jagged-earth",
    [
        (1, "mountain", False), (2, "sands", False), (3, "jungle", True),
        (4, "wetland", True), (5, "jungle", False), (6, "sands", True),
        (7, "mountain", True), (8, "wetland", False),
    ],
    {
        "0": ["3", "4", "6", "7"],
        "1": ["2", "5", "8"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5", "6"], "5": ["1", "2", "4", "8"],
        "6": ["0", "4", "7"], "7": ["0", "6", "8"], "8": ["1", "5", "7"],
    },
)

JE_G = build_board(
    "G", "jagged-earth",
    [
        (1, "wetland", False), (2, "jungle", False), (3, "mountain", True),
        (4, "sands", True), (5, "wetland", False), (6, "mountain", False),
        (7, "jungle", True), (8, "sands", True),
    ],
    {
        "0": ["3", "4", "7", "8"],
        "1": ["2", "5", "6"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5", "7"], "5": ["1", "2", "4", "6"],
        "6": ["1", "5", "8"], "7": ["0", "4", "8"], "8": ["0", "6", "7"],
    },
)

JE_H = build_board(
    "H", "jagged-earth",
    [
        (1, "sands", False), (2, "mountain", False), (3, "jungle", True),
        (4, "wetland", True), (5, "sands", False), (6, "wetland", False),
        (7, "jungle", True), (8, "mountain", True),
    ],
    {
        "0": ["3", "4", "7", "8"],
        "1": ["2", "5", "6"], "2": ["1", "3", "5"], "3": ["0", "2", "4"],
        "4": ["0", "3", "5", "7"], "5": ["1", "2", "4", "6"],
        "6": ["1", "5", "8"], "7": ["0", "4", "8"], "8": ["0", "6", "7"],
    },
)

BOARDS = {
    "base_B": BASE_B, "base_C": BASE_C, "base_D": BASE_D,
    "je_E": JE_E, "je_F": JE_F, "je_G": JE_G, "je_H": JE_H,
}


def main():
    BOARDS_DIR.mkdir(parents=True, exist_ok=True)
    for fname, board in BOARDS.items():
        out = BOARDS_DIR / f"{fname}.json"
        out.write_text(json.dumps(board, indent=2) + "\n")
        print(f"wrote {out}")


if __name__ == "__main__":
    main()
