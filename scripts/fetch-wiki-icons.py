#!/usr/bin/env python3
"""
Fetch Spirit Island icon SVG/PNG assets from spiritislandwiki.com.

The si-live frontend uses these for the UI — elements, units, tokens,
resources, speed markers, terrain symbols. They're @ Greater Than Games;
we use them for this personal guide under fair-use + attribution.

Outputs to `tools/si-live/frontend/src/assets/icons/<slug>.svg|.png`.

Usage:
    python3 scripts/fetch-wiki-icons.py
"""

from __future__ import annotations
import json
import re
import sys
import time
import urllib.request
import urllib.parse
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT_DIR = REPO / "tools" / "si-live" / "frontend" / "src" / "assets" / "icons"
WIKI_API = "https://spiritislandwiki.com/api.php"
HEADERS = {"User-Agent": "spirit-island-mastery-guide/0.1"}
DELAY = 1.0  # respect rate limits

# Slug → Wiki filename. Preferred format is in the filename.
ICONS: dict[str, str] = {
    # Elements — PNG (no SVG on wiki)
    "element-moon": "Moonelement.png",
    "element-sun": "Sunelement.png",
    "element-fire": "Fireelement.png",
    "element-air": "Airelement.png",
    "element-water": "Waterelement.png",
    "element-earth": "Earthelement.png",
    "element-plant": "Plantelement.png",
    "element-animal": "Animalelement.png",

    # Units — SVG
    "unit-explorer": "Explorer.svg",
    "unit-town": "Town.svg",
    "unit-city": "City.svg",
    "unit-dahan": "Dahan.svg",

    # Resources — SVG
    "resource-fear": "Fear.svg",
    "resource-blight": "Blight.svg",
    "resource-sacred-site": "SacredSite.svg",

    # Speed — SVG (colored variants)
    "speed-fast": "FastColor.svg",
    "speed-slow": "SlowColor.svg",

    # Terrain — SVG
    "terrain-mountain": "Mountain.svg",
    "terrain-wetland": "Wetland.svg",
    "terrain-jungle": "Jungle.svg",
    "terrain-sands": "Sands.svg",
}


def api_image_url(filename: str) -> str:
    """Resolve a File:X name to its canonical URL via the Wiki API."""
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
        raise RuntimeError(f"no pages for {filename}")
    info = pages[0].get("imageinfo") or []
    if not info:
        raise RuntimeError(f"no imageinfo for {filename} — does it exist?")
    return info[0].get("url", "")


def download(url: str, dest: Path) -> None:
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=30) as resp:
        dest.write_bytes(resp.read())


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    # Attribution README
    (OUT_DIR / "README.md").write_text(
        "# Spirit Island icon assets\n\n"
        "Icons in this directory are © Greater Than Games, fetched from the "
        "[Spirit Island Wiki](https://spiritislandwiki.com) via "
        "`scripts/fetch-wiki-icons.py`. Used here for a personal strategy "
        "guide + training tool under fair-use conventions.\n"
    )

    last_fetch = 0.0
    failed: list[str] = []
    for slug, filename in ICONS.items():
        ext = Path(filename).suffix.lower()
        dest = OUT_DIR / f"{slug}{ext}"
        if dest.exists():
            continue
        # rate limit
        now = time.monotonic()
        wait = (last_fetch + DELAY) - now
        if wait > 0:
            time.sleep(wait)
        last_fetch = time.monotonic()
        try:
            url = api_image_url(filename)
            if not url:
                raise RuntimeError("empty url")
            print(f"fetching {slug} ← {filename} ({url})", file=sys.stderr)
            download(url, dest)
        except Exception as e:
            print(f"  FAIL {slug}: {e}", file=sys.stderr)
            failed.append(slug)

    print(f"\nwrote {len(ICONS) - len(failed)} icons to {OUT_DIR}", file=sys.stderr)
    if failed:
        print(f"failed: {failed}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
