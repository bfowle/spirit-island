#!/usr/bin/env python3
"""
Fetch canonical Board geometry from spiritislandwiki.com's Island_Boards page.

The Wiki page contains per-board wikitable blocks with columns:
  Land Number | Terrain | Setup Symbols | Adjacencies | Coastal? | Token?

Each board has two variants:
  - "Balanced" (front of physical board; used for most games)
  - "Thematic" (back of physical board; named, e.g., "North East")

Parses both and writes data/boards/<board_file>.json.

Schema:
    {
      "board_id": "A",
      "expansion": "base",
      "variants": {
        "balanced": {
          "name": "Board A",
          "lands": {
            "1": {"terrain": "mountain", "coastal": true, "setup": [],
                  "starting_dahan": 0, "starting_town": 0, "starting_city": 0,
                  "starting_blight": 0, "token": "beasts"},
            ...
          },
          "adjacencies": {"1": [2, 4, 5, 6], ...}
        },
        "thematic": {...}
      }
    }

Usage:
    python3 scripts/fetch-board-geometry.py
"""

from __future__ import annotations
import json, re, sys, urllib.request
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT_DIR = REPO / "data" / "boards"
WIKI_API = "https://spiritislandwiki.com/api.php"

TERRAIN_MAP = {"mountain": "mountain", "wetland": "wetland", "jungle": "jungle", "sand": "sands", "sands": "sands"}

TOKEN_NAMES = {"beast", "beasts", "disease", "strife", "wilds", "wild", "plantation", "badlands", "isolate", "fortress"}

# Items that go in starting-unit counts (not tokens)
UNIT_ITEMS = {"dahan", "town", "city", "blight", "explorer"}


def fetch_wikitext(page: str) -> str:
    url = f"{WIKI_API}?action=parse&page={page}&prop=wikitext&format=json&formatversion=2"
    req = urllib.request.Request(url, headers={"User-Agent": "spirit-island-guide"})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read().decode("utf-8"))["parse"]["wikitext"]


def extract_tokens_in_cell(cell: str) -> list[str]:
    """Return lowercased token names from a cell like '{{mountain}}' or
    '{{city}}{{dahan}}{{dahan}}'."""
    return [m.group(1).lower() for m in re.finditer(r"\{\{([^|}]+?)\}\}", cell)]


def parse_adjacencies(cell: str) -> list[int]:
    return [int(n) for n in re.findall(r"\b\d+\b", cell)]


def parse_land_row(row: str) -> dict | None:
    # Each row has 5 (thematic) or 6 (balanced) cells delimited by
    # `| <content>` lines. Empty cells render as bare `|`.
    cells: list[str] = []
    for line in row.split("\n"):
        stripped = line.rstrip()
        if not stripped.startswith("|"):
            continue
        if stripped.startswith("|-") or stripped.startswith("|}"):
            continue
        cells.append(stripped[1:].strip())
    if len(cells) < 5:
        return None
    try:
        land_num = int(re.search(r"\d+", cells[0]).group())
    except (AttributeError, ValueError):
        return None

    terrain_tokens = extract_tokens_in_cell(cells[1])
    terrain = next((TERRAIN_MAP[t] for t in terrain_tokens if t in TERRAIN_MAP), "unknown")

    setup_items = extract_tokens_in_cell(cells[2])
    # Split setup items into units (dahan/town/city/blight/explorer) + tokens
    # (beasts/disease/strife/wilds/plantation/…). Unknown items are dropped.
    unit_counts = {u: setup_items.count(u) for u in UNIT_ITEMS}
    setup_tokens = [t for t in setup_items if t in TOKEN_NAMES]

    adjacencies = parse_adjacencies(cells[3])
    coastal = "checkmark" in cells[4].lower()

    # Balanced boards have a separate Token column; thematic don't
    if len(cells) >= 6:
        token_cell_tokens = extract_tokens_in_cell(cells[5])
        if token_cell_tokens:
            setup_tokens.append(token_cell_tokens[0])
    # Dedupe token list
    seen: set[str] = set()
    final_tokens: list[str] = []
    for t in setup_tokens:
        if t not in seen:
            seen.add(t)
            final_tokens.append(t)

    return {
        "land_num": land_num,
        "terrain": terrain,
        "coastal": coastal,
        "setup_raw": setup_items,
        "starting": unit_counts,
        "adjacencies": adjacencies,
        "tokens": final_tokens,
    }


def parse_tables_in_section(section_body: str) -> list[list[dict]]:
    """Return a list of tables; each table is a list of land-row dicts."""
    tables = re.findall(
        r'\{\|\s*class="wikitable[^\n]*\n(.*?)\|\}',
        section_body,
        re.DOTALL,
    )
    parsed = []
    for tbl in tables:
        # Each row starts with |- (separator); header is before first |-.
        parts = re.split(r"^\|-", tbl, flags=re.MULTILINE)
        rows = []
        for part in parts[1:]:
            land = parse_land_row(part)
            if land:
                rows.append(land)
        if 6 <= len(rows) <= 12:
            parsed.append(rows)
    return parsed


def normalize_variant(name: str, rows: list[dict]) -> dict:
    lands = {}
    adjacencies = {}
    for r in rows:
        lid = str(r["land_num"])
        s = r["starting"]
        lands[lid] = {
            "terrain": r["terrain"],
            "coastal": r["coastal"],
            "setup_raw": r["setup_raw"],
            "starting_dahan": s["dahan"],
            "starting_town": s["town"],
            "starting_city": s["city"],
            "starting_blight": s["blight"],
            "starting_explorer": s["explorer"],
            "tokens": r["tokens"],
        }
        adjacencies[lid] = [str(n) for n in r["adjacencies"]]
    return {"name": name, "lands": lands, "adjacencies": adjacencies}


def main():
    wt = fetch_wikitext("Island_Boards")
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    # Section headers: `==Board X (Thematic Name)==`
    sections = re.split(r"\n==\s*([^=\n]+?)\s*==\n", wt)
    # sections = [preamble, title1, body1, title2, body2, ...]

    boards: dict[str, dict] = {}
    for i in range(1, len(sections), 2):
        title = sections[i].strip()
        body = sections[i + 1]
        m = re.match(r"Board\s+([A-H])(?:\s*\((.+)\))?", title)
        if not m:
            continue
        letter, thematic_name = m.group(1), (m.group(2) or "").strip()

        # Each board has two variants in subsections. Find all tables in
        # the full body — typically 2 tables per board section.
        tables = parse_tables_in_section(body)
        if len(tables) < 1:
            print(f"[{letter}] no tables found in section", file=sys.stderr)
            continue

        variants: dict[str, dict] = {}
        if tables:
            variants["balanced"] = normalize_variant(f"Board {letter}", tables[0])
        if len(tables) > 1:
            variants["thematic"] = normalize_variant(thematic_name or f"Board {letter} (thematic)", tables[1])

        if letter in ("A", "B", "C", "D"):
            expansion = "base"
            fname = f"base_{letter}.json"
        elif letter in ("E", "F"):
            expansion = "jagged-earth"
            fname = f"je_{letter}.json"
        elif letter in ("G", "H"):
            expansion = "horizons-of-spirit-island"
            fname = f"hosi_{letter}.json"
        else:
            continue

        doc = {
            "source": "spiritislandwiki.com/index.php?title=Island_Boards",
            "board_id": letter,
            "expansion": expansion,
            "variants": variants,
        }
        boards[letter] = doc
        (OUT_DIR / fname).write_text(json.dumps(doc, indent=2) + "\n")
        print(f"wrote {fname}: {len(variants)} variant(s)", file=sys.stderr)

    # Remove stale old files that don't match the new convention
    for old_name in ("je_E.json", "je_F.json", "je_G.json", "je_H.json"):
        # keep je_E/je_F; the G/H files now get renamed to hosi_G/hosi_H
        p = OUT_DIR / old_name
        if old_name in ("je_G.json", "je_H.json") and p.exists():
            p.unlink()
            print(f"removed stale {old_name}", file=sys.stderr)

    print(f"\nwrote {len(boards)} boards to {OUT_DIR}/", file=sys.stderr)


if __name__ == "__main__":
    main()
