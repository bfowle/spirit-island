#!/usr/bin/env python3
"""
Fetch Spirit Island board PNGs from spiritislandwiki.com Island_Boards page.

The wiki publishes transparent PNGs for each board variant (Balanced +
Thematic) for base-game boards A-D and Jagged Earth boards E-H. We use
these as authentic backdrops for the si-live map view instead of the
synthetic hex layout.

Outputs to `tools/si-live/frontend/public/board-images/<boardId>-<variant>.png`.

Usage:
    python3 scripts/fetch-board-images.py
"""

from __future__ import annotations
import json
import sys
import time
import urllib.request
import urllib.parse
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT_DIR = REPO / "tools" / "si-live" / "frontend" / "public" / "board-images"
WIKI_API = "https://spiritislandwiki.com/api.php"
HEADERS = {"User-Agent": "spirit-island-mastery-guide/0.1"}
DELAY = 1.5  # sustained-load rate limit per memory/feedback_wiki_scrape_unreliable.md

# Per the Wiki's Island_Boards page: board PNGs are named with this pattern.
# We try a few candidate filenames per board/variant since the wiki isn't
# 100% consistent. First one that resolves is used.
BOARDS = [
    # (id, variant, [candidate filenames])
    # Base-game board balanced sides use letters; thematic sides use cardinal
    # directions. Jagged Earth E/F have thematic (south-east, south-west).
    # Horizons G/H are balanced-only on the Wiki.
    ("A", "balanced", ["Piece_core_board_a.png"]),
    ("A", "thematic", ["Piece_core_board_north_east.png"]),
    ("B", "balanced", ["Piece_core_board_b.png"]),
    ("B", "thematic", ["Piece_core_board_east.png"]),
    ("C", "balanced", ["Piece_core_board_c.png"]),
    ("C", "thematic", ["Piece_core_board_north_west.png"]),
    ("D", "balanced", ["Piece_core_board_d.png"]),
    ("D", "thematic", ["Piece_core_board_west.png"]),
    ("E", "balanced", ["Piece_je_board_e.png"]),
    ("E", "thematic", ["Piece_je_board_south_east.png"]),
    ("F", "balanced", ["Piece_je_board_f.png"]),
    ("F", "thematic", ["Piece_je_board_south_west.png"]),
    ("G", "balanced", ["Piece_horizons_board_g.png"]),
    ("H", "balanced", ["Piece_horizons_board_h.png"]),
]


def api_image_url(filename: str) -> str | None:
    params = {
        "action": "query",
        "titles": f"File:{filename}",
        "prop": "imageinfo",
        "iiprop": "url",
        "format": "json",
        "formatversion": "2",
    }
    url = f"{WIKI_API}?{urllib.parse.urlencode(params)}"
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read().decode("utf-8"))
    pages = data.get("query", {}).get("pages", [])
    if not pages:
        return None
    # A missing file returns a page with `missing: true`
    if pages[0].get("missing"):
        return None
    info = pages[0].get("imageinfo") or []
    if not info:
        return None
    return info[0].get("url") or None


def download(url: str, dest: Path) -> int:
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=60) as resp:
        data = resp.read()
    dest.write_bytes(data)
    return len(data)


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUT_DIR / "README.md").write_text(
        "# Spirit Island board images\n\n"
        "Board PNGs © Greater Than Games, fetched from the "
        "[Spirit Island Wiki](https://spiritislandwiki.com) via "
        "`scripts/fetch-board-images.py`. Used as map-view backdrops in the "
        "si-live companion tool under fair-use conventions.\n"
    )

    last_fetch = 0.0
    failed: list[tuple[str, str]] = []
    resolved_count = 0

    for board_id, variant, candidates in BOARDS:
        dest = OUT_DIR / f"{board_id}-{variant}.png"
        if dest.exists():
            print(f"  exists: {dest.name}", file=sys.stderr)
            continue

        resolved_url = None
        for filename in candidates:
            # rate limit
            now = time.monotonic()
            wait = (last_fetch + DELAY) - now
            if wait > 0:
                time.sleep(wait)
            last_fetch = time.monotonic()

            try:
                url = api_image_url(filename)
                if url:
                    resolved_url = url
                    print(f"  resolved {filename}: {url}", file=sys.stderr)
                    break
            except Exception as e:
                print(f"  ⚠ error resolving {filename}: {e}", file=sys.stderr)

        if not resolved_url:
            print(f"  ✗ could not resolve any of: {candidates}", file=sys.stderr)
            failed.append((board_id, variant))
            continue

        try:
            # rate limit before download too
            now = time.monotonic()
            wait = (last_fetch + DELAY) - now
            if wait > 0:
                time.sleep(wait)
            last_fetch = time.monotonic()
            bytes_written = download(resolved_url, dest)
            print(f"  ✓ {dest.name} ({bytes_written:,} bytes)", file=sys.stderr)
            resolved_count += 1
        except Exception as e:
            print(f"  ✗ download failed for {dest.name}: {e}", file=sys.stderr)
            failed.append((board_id, variant))

    print(f"\nResolved {resolved_count}/{len(BOARDS)} board images.", file=sys.stderr)
    if failed:
        print(f"Failed: {failed}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
