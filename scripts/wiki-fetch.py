#!/usr/bin/env python3
"""
Fetch and deterministically parse Spirit Island Wiki pages.

Uses MediaWiki API to get raw wikitext, then hand-parses the template
structure. No LLM summarization. No hallucination.

Usage:
    python3 wiki-fetch.py spirit "Shadows Flicker Like Flame"
    python3 wiki-fetch.py card "Crops Wither and Fade"
    python3 wiki-fetch.py batch-spirit "Shadows Flicker Like Flame"   # spirit + all its uniques
    python3 wiki-fetch.py --output-dir data/references/wiki/ batch-spirit "Shadows Flicker Like Flame"

Output: JSON with all template fields, written to stdout or to a file
under the output-dir if specified.
"""

from __future__ import annotations

import argparse
import json
import random
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

WIKI_API = "https://spiritislandwiki.com/api.php"

# Rate-limiting defaults. The Wiki is aggressive about throttling; default to
# 1.5s between requests + exponential backoff on 429/5xx. Override with flags.
DEFAULT_DELAY_SECS = 1.5
MAX_RETRIES = 5
BACKOFF_BASE_SECS = 2.0

_LAST_FETCH_TS: float = 0.0

# Wiki markup tokens → human-readable replacements, applied post-parse.
# Keys are matched verbatim as {{key|...}} or {{key}} in effect text fields.
TOKEN_MAP = {
    "fear": "Fear",
    "town": "Town",
    "city": "City",
    "explorer": "Explorer",
    "dahan": "Dahan",
    "blight": "Blight",
    "presence": "Presence",
    "energy": "Energy",
    "sacred": "Sacred Site",
    "beasts": "Beasts",
    "beast": "Beast",
    "wilds": "Wilds",
    "strife": "Strife",
    "disease": "Disease",
    "badlands": "Badlands",
    "defend": "Defend",
    "damage": "Damage",
    "push": "Push",
    "gather": "Gather",
    "destroy": "Destroy",
    "ravage": "Ravage",
    "build": "Build",
    "explore": "Explore",
    "or": " **OR** ",
    # Additional tokens found in Wiki text fields:
    "fast": "Fast",
    "slow": "Slow",
    "sacredsite": "Sacred Site",
    "moon": "Moon",
    "sun": "Sun",
    "air": "Air",
    "water": "Water",
    "earth": "Earth",
    "fire": "Fire",
    "plant": "Plant",
    "animal": "Animal",
    "simplewater": "Water",
    "wild": "Wilds",
    "town": "Town",
    "city": "City",
}


def clean_wiki_text(text: str | None) -> str | None:
    """Replace wiki-markup tokens like {{fear}} or {{invader|explorer}} with readable text.

    Also strips:
    - <ref>...</ref> tags and their content (errata references).
    - <br/> / <br> → space.
    - MediaWiki bold/italic markers (''' and '').
    - Common template-with-arg patterns like {{range|1}} → "Range 1".
    """
    if text is None:
        return None
    import re

    # Strip <ref>...</ref> blocks entirely (including any content).
    text = re.sub(r"<ref[^>]*>.*?</ref>", "", text, flags=re.DOTALL)
    # Strip self-closing <ref /> tags
    text = re.sub(r"<ref[^/]*/>", "", text)

    def replace_template(match: "re.Match[str]") -> str:
        inner = match.group(1).strip()
        parts = [p.strip() for p in inner.split("|")]
        head = parts[0].lower()
        # {{range|1}} → "Range 1"
        if head == "range" and len(parts) > 1:
            return f"Range {parts[1]}"
        # {{speed|fast}} → "Fast"
        if head == "speed" and len(parts) > 1:
            return TOKEN_MAP.get(parts[1].lower(), parts[1].title())
        # {{element|water}} → "Water"
        if head == "element" and len(parts) > 1:
            return TOKEN_MAP.get(parts[1].lower(), parts[1].title())
        # {{invader|explorer}} → "Explorer"
        if head == "invader" and len(parts) > 1:
            return TOKEN_MAP.get(parts[1].lower(), parts[1].title())
        # {{Energycost|cost=7}} → "Cost 7"
        if head == "energycost":
            for part in parts[1:]:
                if "=" in part:
                    k, v = part.split("=", 1)
                    if k.strip().lower() == "cost":
                        return f"Cost {v.strip()}"
            return match.group(0)
        # Preserve threshold tokens for readability
        if head == "threshold" and len(parts) > 1:
            return match.group(0)
        return TOKEN_MAP.get(head, match.group(0))

    # Iterate the regex a few times to catch nested patterns.
    for _ in range(3):
        new_text = re.sub(r"\{\{([^{}]+)\}\}", replace_template, text)
        if new_text == text:
            break
        text = new_text

    text = text.replace("<br/>", " ").replace("<br>", " ")
    # MediaWiki markers: ''' = bold, '' = italic — strip pairs
    text = re.sub(r"'{2,}", "", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text



def _throttle(delay: float) -> None:
    """Sleep as needed so at least `delay` seconds have passed since the last fetch."""
    global _LAST_FETCH_TS
    now = time.monotonic()
    wait = (_LAST_FETCH_TS + delay) - now
    if wait > 0:
        time.sleep(wait)
    _LAST_FETCH_TS = time.monotonic()


def _strip_html_comments(text: str) -> str:
    """Remove <!-- ... --> blocks from wikitext. Some pages use them to annotate
    language fields (e.g. `| <!--English-->name_en=Foo`) which confuses the key
    parser. Stripping them before parsing is safe — they're just comments."""
    import re
    return re.sub(r"<!--.*?-->", "", text, flags=re.DOTALL)


def fetch_category_members(category: str, delay: float = DEFAULT_DELAY_SECS) -> list[str]:
    """List page titles in a MediaWiki category, paginating via cmcontinue.

    `category` is the category name without the `Category:` prefix, e.g.
    "Minor_Power". Only returns pages in namespace 0 (main content, not talk /
    template / file pages).
    """
    titles: list[str] = []
    cmcontinue: str | None = None
    while True:
        params = {
            "action": "query",
            "list": "categorymembers",
            "cmtitle": f"Category:{category}",
            "cmlimit": "500",
            "cmnamespace": "0",
            "format": "json",
            "formatversion": "2",
        }
        if cmcontinue:
            params["cmcontinue"] = cmcontinue
        url = f"{WIKI_API}?{urllib.parse.urlencode(params)}"
        req = urllib.request.Request(url, headers={"User-Agent": "spirit-island-mastery-guide/0.1"})

        last_err: Exception | None = None
        for attempt in range(MAX_RETRIES):
            _throttle(delay)
            try:
                with urllib.request.urlopen(req, timeout=30) as resp:
                    body = resp.read().decode("utf-8")
                data = json.loads(body)
                if "error" in data:
                    raise RuntimeError(f"Wiki API error listing Category:{category}: {data['error']}")
                members = data.get("query", {}).get("categorymembers", [])
                for m in members:
                    titles.append(m["title"])
                cmcontinue = data.get("continue", {}).get("cmcontinue")
                last_err = None
                break
            except urllib.error.HTTPError as e:
                last_err = e
                if e.code in (429, 500, 502, 503, 504):
                    retry_after = e.headers.get("Retry-After") if e.headers else None
                    backoff = float(retry_after) if retry_after else BACKOFF_BASE_SECS * (2 ** attempt) + random.uniform(0, 1)
                    print(f"  [rate-limit] HTTP {e.code} on Category:{category}, waiting {backoff:.1f}s (retry {attempt + 1}/{MAX_RETRIES})", file=sys.stderr)
                    time.sleep(backoff)
                    continue
                raise
            except urllib.error.URLError as e:
                last_err = e
                backoff = BACKOFF_BASE_SECS * (2 ** attempt) + random.uniform(0, 1)
                print(f"  [network] {e} on Category:{category}, waiting {backoff:.1f}s (retry {attempt + 1}/{MAX_RETRIES})", file=sys.stderr)
                time.sleep(backoff)
                continue
        else:
            raise RuntimeError(f"Failed listing Category:{category} after {MAX_RETRIES} retries: {last_err}")

        if not cmcontinue:
            break

    return titles


def fetch_wikitext(page: str, delay: float = DEFAULT_DELAY_SECS) -> str:
    """Fetch raw wikitext for a wiki page via the MediaWiki API with rate-limit handling.

    Respects a minimum delay between requests and retries with exponential
    backoff on 429 / 5xx responses.
    """
    params = {
        "action": "parse",
        "page": page,
        "prop": "wikitext",
        "format": "json",
        "formatversion": "2",
    }
    url = f"{WIKI_API}?{urllib.parse.urlencode(params)}"
    req = urllib.request.Request(url, headers={"User-Agent": "spirit-island-mastery-guide/0.1"})

    last_err: Exception | None = None
    for attempt in range(MAX_RETRIES):
        _throttle(delay)
        try:
            with urllib.request.urlopen(req, timeout=30) as resp:
                body = resp.read().decode("utf-8")
            data = json.loads(body)
            if "error" in data:
                raise RuntimeError(f"Wiki API error for '{page}': {data['error']}")
            return _strip_html_comments(data["parse"]["wikitext"])
        except urllib.error.HTTPError as e:
            last_err = e
            if e.code in (429, 500, 502, 503, 504):
                # Respect Retry-After if provided; else exponential backoff with jitter.
                retry_after = e.headers.get("Retry-After") if e.headers else None
                if retry_after:
                    try:
                        backoff = float(retry_after)
                    except ValueError:
                        backoff = BACKOFF_BASE_SECS * (2 ** attempt)
                else:
                    backoff = BACKOFF_BASE_SECS * (2 ** attempt) + random.uniform(0, 1)
                print(f"  [rate-limit] HTTP {e.code} for '{page}', waiting {backoff:.1f}s before retry {attempt + 1}/{MAX_RETRIES}", file=sys.stderr)
                time.sleep(backoff)
                continue
            raise
        except urllib.error.URLError as e:
            last_err = e
            backoff = BACKOFF_BASE_SECS * (2 ** attempt) + random.uniform(0, 1)
            print(f"  [network] {e} for '{page}', waiting {backoff:.1f}s before retry {attempt + 1}/{MAX_RETRIES}", file=sys.stderr)
            time.sleep(backoff)
            continue

    raise RuntimeError(f"Failed to fetch '{page}' after {MAX_RETRIES} retries: {last_err}")


def parse_template(text: str, start: int = 0) -> tuple[dict, int]:
    """
    Parse a single {{TemplateName|field1|field2=value|...}} from `text` starting at `start`.

    Returns (parsed_dict, end_index).

    The parsed_dict has:
      - "_name" : template name (first token before the first |)
      - "_positional" : list of unnamed positional args
      - named fields as top-level keys

    Values containing nested {{...}} are preserved verbatim as strings (the caller
    can recurse into them).
    """
    assert text[start:start + 2] == "{{", f"expected '{{{{' at offset {start}, got {text[start:start+2]!r}"
    i = start + 2
    depth = 1
    current = []
    fields: list[str] = []

    # Walk char by char tracking brace + link depth so | inside nested templates
    # doesn't split fields at the wrong level.
    link_depth = 0  # for [[ ]] link syntax
    while i < len(text) and depth > 0:
        c = text[i]
        nxt2 = text[i:i + 2]
        if nxt2 == "{{":
            depth += 1
            current.append(nxt2)
            i += 2
            continue
        if nxt2 == "}}":
            depth -= 1
            if depth == 0:
                # End of template
                fields.append("".join(current))
                i += 2
                break
            current.append(nxt2)
            i += 2
            continue
        if nxt2 == "[[":
            link_depth += 1
            current.append(nxt2)
            i += 2
            continue
        if nxt2 == "]]":
            link_depth = max(0, link_depth - 1)
            current.append(nxt2)
            i += 2
            continue
        if c == "|" and depth == 1 and link_depth == 0:
            fields.append("".join(current))
            current = []
            i += 1
            continue
        current.append(c)
        i += 1

    if not fields:
        raise RuntimeError("empty template encountered")

    name = fields[0].strip()
    parsed: dict = {"_name": name, "_positional": []}
    for field in fields[1:]:
        stripped = field.strip()
        if "=" in stripped:
            # Named field. The first "=" separates (but watch for = inside
            # nested templates).
            eq = find_top_level_equals(stripped)
            if eq >= 0:
                key = stripped[:eq].strip()
                value = stripped[eq + 1:].strip()
                parsed[key] = value
                continue
        parsed["_positional"].append(stripped)

    return parsed, i


def find_top_level_equals(s: str) -> int:
    """Find first = not inside {{...}} or [[...]]."""
    depth = 0
    link = 0
    for idx in range(len(s)):
        nxt2 = s[idx:idx + 2]
        if nxt2 == "{{":
            depth += 1
            continue
        if nxt2 == "}}":
            depth = max(0, depth - 1)
            continue
        if nxt2 == "[[":
            link += 1
            continue
        if nxt2 == "]]":
            link = max(0, link - 1)
            continue
        if s[idx] == "=" and depth == 0 and link == 0:
            return idx
    return -1


def find_first_template(text: str, template_name_prefix: str | None = None) -> int:
    """Return the index of the first {{ in text (optionally where the template name starts with prefix)."""
    search_from = 0
    while True:
        idx = text.find("{{", search_from)
        if idx < 0:
            return -1
        if template_name_prefix is None:
            return idx
        name_region = text[idx + 2:idx + 2 + len(template_name_prefix) + 20]
        if name_region.lstrip().startswith(template_name_prefix):
            return idx
        search_from = idx + 2


def extract_nested_templates(text: str) -> list[dict]:
    """Find all top-level {{...}} templates in a text fragment (for innate fields, etc.)."""
    out = []
    i = 0
    while True:
        j = text.find("{{", i)
        if j < 0:
            break
        tpl, end = parse_template(text, j)
        out.append(tpl)
        i = end
    return out


def parse_spirit_page(wikitext: str) -> dict:
    """Parse a {{Spirit|...}} template from a spirit page into a structured dict."""
    idx = find_first_template(wikitext, "Spirit")
    if idx < 0:
        raise RuntimeError("No {{Spirit|...}} template found on page")
    tpl, _ = parse_template(wikitext, idx)

    # innates field contains nested {{Innate|...}} templates with {{Threshold|...}} children
    innates_raw = tpl.get("innates", "")
    innates = []
    for itpl in extract_nested_templates(innates_raw):
        if itpl["_name"] != "Innate":
            continue
        # Innate's positional arg typically includes the name followed by threshold templates
        innate = {
            "name": None,
            "speed": itpl.get("turn"),
            "range": itpl.get("range"),
            "target": itpl.get("target"),
            "option": itpl.get("option"),
            "thresholds": [],
        }
        # Positional list: first text is the name in all-caps-bold, remaining are {{Threshold}} templates
        for pos in itpl["_positional"]:
            pos = pos.strip()
            if not pos:
                continue
            if pos.startswith("{{"):
                # parse nested threshold templates within this positional
                for sub in extract_nested_templates(pos):
                    if sub["_name"] != "Threshold":
                        continue
                    thr = {
                        "moon": sub.get("m"),
                        "sun": sub.get("s"),
                        "fire": sub.get("f"),
                        "air": sub.get("a"),
                        "water": sub.get("w"),
                        "earth": sub.get("e"),
                        "plant": sub.get("p"),
                        "animal": sub.get("an") or sub.get("beast"),
                        "effect": sub["_positional"][0] if sub["_positional"] else None,
                    }
                    # Drop None threshold entries
                    thr = {k: v for k, v in thr.items() if v is not None}
                    innate["thresholds"].append(thr)
            else:
                # Raw text — usually the innate's name
                if innate["name"] is None:
                    # Strip markup like '''''NAME'''''
                    cleaned = pos.strip().strip("'").strip()
                    innate["name"] = cleaned
        innates.append(innate)

    # Unique cards list — comma-separated string
    uniques_raw = tpl.get("uniquepowers", "")
    uniques = [u.strip() for u in uniques_raw.split(",") if u.strip()]

    # Clean innate effect text
    for innate in innates:
        for thr in innate["thresholds"]:
            if "effect" in thr:
                thr["effect"] = clean_wiki_text(thr["effect"])

    # Parse growth options — each growth is a {{growth|first=X|second=Y|...}} template.
    growths = []
    for idx in (1, 2, 3, 4, 5):
        raw = tpl.get(f"growth{idx}")
        if not raw:
            continue
        inner = extract_nested_templates(raw)
        if not inner:
            growths.append({"index": idx, "raw": raw})
            continue
        g = inner[0]
        growths.append(
            {
                "index": idx,
                "first": g.get("first"),
                "second": g.get("second"),
                "third": g.get("third"),
            }
        )

    # Parse presence tracks — each {{PresenceT|slot1|slot2|...}}
    def _parse_presence_track(raw: str | None) -> list[str]:
        if not raw:
            return []
        inner = extract_nested_templates(raw)
        if not inner:
            return []
        return [p.strip() for p in inner[0].get("_positional", [])]

    presence_energy = _parse_presence_track(tpl.get("presence1"))
    presence_cardplay = _parse_presence_track(tpl.get("presence2"))

    # Parse power summary — {{Powersummary|offense=4|control=3|...}}
    psummary: dict = {}
    ps_raw = tpl.get("psummary")
    if ps_raw:
        inner = extract_nested_templates(ps_raw)
        if inner:
            p = inner[0]
            for stat in ("offense", "control", "fear", "defense", "utility"):
                if p.get(stat):
                    psummary[stat] = p[stat]

    # Suggested cards (draft recommendations)
    suggested_cards = []
    for idx in range(1, 20):
        c = tpl.get(f"suggestedcard{idx}")
        if c:
            suggested_cards.append(c.strip())

    return {
        "source": "spiritislandwiki.com",
        "type": "spirit",
        "name": tpl.get("name") or tpl.get("name_en"),
        "expansion": tpl.get("gamebox"),
        "complexity": tpl.get("complexity"),
        "setup": clean_wiki_text(tpl.get("setup")),
        "playstyle": clean_wiki_text(tpl.get("playstyle")),
        "special_rules": clean_wiki_text(tpl.get("special")),
        "special_rules_raw": tpl.get("special"),
        "innates": innates,
        "unique_cards": uniques,
        "growth_type": tpl.get("growthtype"),
        "growths": growths,
        "presence_energy_track": presence_energy,
        "presence_cardplay_track": presence_cardplay,
        "power_summary": psummary,
        "suggested_cards": suggested_cards,
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_") and k not in {"innates"}},
    }


def parse_aspect_page(wikitext: str) -> dict:
    """Parse an {{AspectCardArticle|...}} template into a structured dict."""
    idx = find_first_template(wikitext, "AspectCardArticle")
    if idx < 0:
        raise RuntimeError("No {{AspectCardArticle|...}} template found on page")
    tpl, _ = parse_template(wikitext, idx)

    # Collect all rowXXX fields — these define what the aspect replaces/adds.
    row_fields = {}
    for k, v in tpl.items():
        if k.startswith("row") and not k.startswith("_"):
            row_fields[k[3:]] = clean_wiki_text(v)

    # Collect "Special Rule N" pairs + the un-numbered "Special Rule Name/Text".
    special_rules = []
    # Numbered first (aspects with multiple rules).
    idx = 1
    while True:
        name = row_fields.pop(f"Special Rule {idx} Name", None)
        text = row_fields.pop(f"Special Rule {idx} Text", None)
        if not name and not text:
            break
        special_rules.append({"name": name, "text": text})
        idx += 1
    # Un-numbered single-rule aspects.
    name_un = row_fields.pop("Special Rule Name", None)
    text_un = row_fields.pop("Special Rule Text", None)
    if name_un or text_un:
        special_rules.append({"name": name_un, "text": text_un})

    # Aspects that replace an innate — capture innate definition.
    innate_override = {}
    for key in ("Innate Name", "Speed", "Range", "Target", "Innate Thresholds"):
        v = row_fields.pop(key, None)
        if v:
            innate_override[key.lower().replace(" ", "_")] = v

    return {
        "source": "spiritislandwiki.com",
        "type": "aspect",
        "name": tpl.get("name") or tpl.get("name_en"),
        "spirit": tpl.get("spirit"),
        "expansion": tpl.get("set"),
        "replaces": row_fields.pop("Replaces", None),
        "complexity_change": row_fields.pop("Complexity", None),
        "special_rules": special_rules,
        "innate_override": innate_override or None,
        "other_row_fields": row_fields,
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_")},
    }


def parse_fear_card_page(wikitext: str) -> dict:
    """Parse a {{FearCardArticle|...}} template — has 3 stages (TL1/2, TL2/3, TL3)."""
    idx = find_first_template(wikitext, "FearCardArticle")
    if idx < 0:
        raise RuntimeError("No {{FearCardArticle|...}} template found")
    tpl, _ = parse_template(wikitext, idx)
    return {
        "source": "spiritislandwiki.com",
        "type": "fear_card",
        "name": tpl.get("name") or tpl.get("name_en"),
        "expansion": tpl.get("set"),
        "text_stage_1": clean_wiki_text(tpl.get("text_en1") or tpl.get("text1")),
        "text_stage_2": clean_wiki_text(tpl.get("text_en2") or tpl.get("text2")),
        "text_stage_3": clean_wiki_text(tpl.get("text_en3") or tpl.get("text3")),
        "status": tpl.get("cardstatus") or "Active",
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_")},
    }


def parse_event_card_page(wikitext: str) -> dict:
    """Parse an {{EventCardArticle|...}} template — may have up to 5 event sections."""
    idx = find_first_template(wikitext, "EventCardArticle")
    if idx < 0:
        raise RuntimeError("No {{EventCardArticle|...}} template found")
    tpl, _ = parse_template(wikitext, idx)
    events = []
    for i in range(1, 8):
        ev_type = tpl.get(f"eventType{i}")
        ev_name = tpl.get(f"eventName{i}")
        text = tpl.get(f"text_en{i}") or tpl.get(f"text{i}")
        if not (ev_type or ev_name or text):
            continue
        events.append({
            "event_type": ev_type,
            "event_name": ev_name,
            "text": clean_wiki_text(text),
        })
    return {
        "source": "spiritislandwiki.com",
        "type": "event_card",
        "name": tpl.get("name") or tpl.get("name_en"),
        "expansion": tpl.get("set"),
        "stages": events,
        "status": tpl.get("cardstatus") or "Active",
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_")},
    }


def parse_blight_card_page(wikitext: str) -> dict:
    """Parse a {{BlightCardArticle|...}} template."""
    idx = find_first_template(wikitext, "BlightCardArticle")
    if idx < 0:
        raise RuntimeError("No {{BlightCardArticle|...}} template found")
    tpl, _ = parse_template(wikitext, idx)
    return {
        "source": "spiritislandwiki.com",
        "type": "blight_card",
        "name": tpl.get("name") or tpl.get("name_en"),
        "expansion": tpl.get("set"),
        "cardtype": tpl.get("cardtype"),
        "blight_per_player": tpl.get("blightperplayer"),
        "text": clean_wiki_text(tpl.get("text_en") or tpl.get("text")),
        "status": tpl.get("cardstatus") or "Active",
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_")},
    }


def parse_card_page(wikitext: str) -> dict:
    """Parse a {{PowerCardArticle|...}} template into a structured dict."""
    idx = find_first_template(wikitext, "PowerCardArticle")
    if idx < 0:
        raise RuntimeError("No {{PowerCardArticle|...}} template found on page")
    tpl, _ = parse_template(wikitext, idx)
    raw_text = tpl.get("text_en") or tpl.get("text")
    raw_threshold = tpl.get("thresholdtext_en") or tpl.get("thresholdtext")
    return {
        "source": "spiritislandwiki.com",
        "type": "power_card",
        "name": tpl.get("name") or tpl.get("name_en"),
        "spirit": tpl.get("spirit") or tpl.get("unique"),
        "card_type": tpl.get("cardtype"),
        "expansion": tpl.get("set"),
        "cost": tpl.get("cost"),
        "speed": tpl.get("speed"),
        "range": tpl.get("range"),
        "target": tpl.get("target"),
        "elements": [e.strip() for e in (tpl.get("elements") or "").split(",") if e.strip()],
        "text": clean_wiki_text(raw_text),
        "text_raw": raw_text,
        "threshold": clean_wiki_text(raw_threshold),
        "artist": tpl.get("artist"),
        "status": tpl.get("cardstatus"),
        "raw_template": {k: v for k, v in tpl.items() if not k.startswith("_")},
    }


def slugify(name: str) -> str:
    return name.lower().replace("'", "").replace(" ", "-").replace(",", "")


def _cache_path(args, page_name: str, kind: str) -> Path | None:
    cache_dir = getattr(args, "cache_dir", None)
    if not cache_dir:
        return None
    return Path(cache_dir) / f"{kind}_{slugify(page_name)}.json"


def cmd_spirit(args, page_name: str) -> dict:
    cache = _cache_path(args, page_name, "spirit")
    if cache and cache.exists() and not getattr(args, "no_cache", False):
        return json.loads(cache.read_text())
    wt = fetch_wikitext(page_name, delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
    data = parse_spirit_page(wt)
    if cache:
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(data, indent=2, ensure_ascii=False))
    return data


def cmd_card(args, page_name: str) -> dict:
    cache = _cache_path(args, page_name, "card")
    if cache and cache.exists() and not getattr(args, "no_cache", False):
        return json.loads(cache.read_text())
    wt = fetch_wikitext(page_name, delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
    data = parse_card_page(wt)
    if cache:
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(data, indent=2, ensure_ascii=False))
    return data


def cmd_aspect(args, page_name: str) -> dict:
    cache = _cache_path(args, page_name, "aspect")
    if cache and cache.exists() and not getattr(args, "no_cache", False):
        return json.loads(cache.read_text())
    wt = fetch_wikitext(page_name, delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
    data = parse_aspect_page(wt)
    if cache:
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(data, indent=2, ensure_ascii=False))
    return data


def cmd_batch_spirit(args, page_name: str) -> dict:
    """Fetch spirit + all its Uniques + all its suggested Minor/Major cards.

    Rate-limit aware; caches individual pages.
    """
    spirit = cmd_spirit(args, page_name)
    cards = []
    for unique_name in spirit["unique_cards"]:
        try:
            cards.append(cmd_card(args, unique_name.replace(" ", "_")))
        except Exception as e:
            cards.append({"name": unique_name, "error": str(e)})
    spirit["unique_card_details"] = cards

    suggested = []
    for card_name in spirit.get("suggested_cards", []):
        try:
            suggested.append(cmd_card(args, card_name.replace(" ", "_")))
        except Exception as e:
            suggested.append({"name": card_name, "error": str(e)})
    spirit["suggested_card_details"] = suggested
    return spirit


def cmd_fear_event_blight_deck(args, kind: str) -> dict:
    """Walk Category:{Fear,Event,Blight}_Card and parse each page.

    `kind` is "fear" | "event" | "blight". Writes
    {output-dir}/{kind}.json with all cards.
    """
    assert kind in ("fear", "event", "blight"), f"unknown deck kind: {kind}"
    category = {"fear": "Fear_Card", "event": "Event_Card", "blight": "Blight_Card"}[kind]
    parser_fn = {
        "fear": parse_fear_card_page,
        "event": parse_event_card_page,
        "blight": parse_blight_card_page,
    }[kind]
    print(f"==> listing Category:{category}", file=sys.stderr)
    titles = fetch_category_members(category, delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
    print(f"    {len(titles)} page titles found", file=sys.stderr)

    keep_all_statuses = bool(getattr(args, "all_statuses", False))
    cards: list[dict] = []
    skipped: list[dict] = []
    for i, title in enumerate(titles, 1):
        page = title.replace(" ", "_")
        cache = _cache_path(args, page, kind)
        try:
            if cache and cache.exists() and not getattr(args, "no_cache", False):
                data = json.loads(cache.read_text())
            else:
                wt = fetch_wikitext(page, delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
                data = parser_fn(wt)
                if cache:
                    cache.parent.mkdir(parents=True, exist_ok=True)
                    cache.write_text(json.dumps(data, indent=2, ensure_ascii=False))
        except Exception as e:
            print(f"  [{i}/{len(titles)}] {title}: error — {e}", file=sys.stderr)
            skipped.append({"title": title, "reason": f"parse error: {e}"})
            continue
        status = (data.get("status") or "").strip()
        if not keep_all_statuses and status != "Active":
            skipped.append({"title": title, "reason": f"status={status!r}"})
            continue
        cards.append(data)
        if i % 10 == 0:
            print(f"  [{i}/{len(titles)}] … {len(cards)} kept", file=sys.stderr)

    print(f"==> {kind}: kept {len(cards)}, skipped {len(skipped)}", file=sys.stderr)

    result = {
        "source": "spiritislandwiki.com",
        "category": category,
        "kind": kind,
        "count": len(cards),
        "cards": cards,
        "skipped": skipped,
    }
    if args.output_dir:
        out = Path(args.output_dir) / f"{kind}.json"
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
        print(f"Wrote {out} ({len(cards)} cards)", file=sys.stderr)
    return result


def cmd_deck(args, card_type: str) -> dict:
    """Walk Category:Power_Card and split into Minor / Major decks.

    The Wiki tags every power card with a single `Category:Power Card`; no
    per-type categories exist. We walk that single category, parse each page,
    then split by the PowerCardArticle's `cardtype` field.

    When `card_type == "all"`, writes both {output-dir}/minor.json and major.json
    (plus unique.json). When `card_type` is "minor" or "major", writes only that
    deck's JSON.

    Filter: cardstatus == "Active" by default (drops erratum-retired versions).
    """
    assert card_type in ("minor", "major", "all"), f"unknown deck type: {card_type}"
    print("==> listing Category:Power_Card (all power cards)", file=sys.stderr)
    titles = fetch_category_members("Power_Card", delay=getattr(args, "delay", DEFAULT_DELAY_SECS))
    print(f"    {len(titles)} page titles found", file=sys.stderr)

    keep_all_statuses = bool(getattr(args, "all_statuses", False))
    buckets: dict[str, list[dict]] = {"Minor": [], "Major": [], "Unique": [], "Other": []}
    skipped: list[dict] = []

    for i, title in enumerate(titles, 1):
        try:
            data = cmd_card(args, title.replace(" ", "_"))
        except Exception as e:
            print(f"  [{i}/{len(titles)}] {title}: error — {e}", file=sys.stderr)
            skipped.append({"title": title, "reason": f"parse error: {e}"})
            continue
        status = (data.get("status") or "").strip()
        cardtype = (data.get("card_type") or "").strip()
        if not keep_all_statuses and status != "Active":
            skipped.append({"title": title, "reason": f"status={status!r} (not Active)"})
            continue
        bucket = cardtype if cardtype in buckets else "Other"
        buckets[bucket].append(data)
        if i % 25 == 0:
            print(f"  [{i}/{len(titles)}] … Minor={len(buckets['Minor'])} Major={len(buckets['Major'])} Unique={len(buckets['Unique'])} Other={len(buckets['Other'])}", file=sys.stderr)

    print(
        f"==> Power cards parsed: Minor={len(buckets['Minor'])}, Major={len(buckets['Major'])}, "
        f"Unique={len(buckets['Unique'])}, Other={len(buckets['Other'])}, skipped={len(skipped)}",
        file=sys.stderr,
    )

    def _dedupe(cards: list[dict]) -> list[dict]:
        """Drop errata'd duplicates — for each card name, prefer the entry
        whose raw_template.errata is empty (= canonical current printing).
        The Wiki lists both the legacy and re-issued printings for many base
        cards; only the current one belongs in the deck pool."""
        by_name: dict[str, list[dict]] = {}
        for c in cards:
            by_name.setdefault(c.get("name"), []).append(c)
        out: list[dict] = []
        for name, group in by_name.items():
            canonical = [c for c in group if not c.get("raw_template", {}).get("errata", "")]
            if canonical:
                out.append(canonical[0])
            else:
                # All copies carry an errata marker — keep first arbitrarily
                out.append(group[0])
        return out

    def _bundle(name: str, cards: list[dict]) -> dict:
        deduped = _dedupe(cards)
        return {
            "source": "spiritislandwiki.com",
            "card_type": name,
            "count": len(deduped),
            "cards": deduped,
        }

    result = {
        "minor": _bundle("Minor", buckets["Minor"]),
        "major": _bundle("Major", buckets["Major"]),
        "unique": _bundle("Unique", buckets["Unique"]),
        "other": _bundle("Other", buckets["Other"]),
        "skipped": skipped,
    }

    if args.output_dir:
        out_dir = Path(args.output_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        to_write = [card_type] if card_type in ("minor", "major") else ["minor", "major", "unique"]
        for key in to_write:
            out_path = out_dir / f"{key}.json"
            out_path.write_text(json.dumps(result[key], indent=2, ensure_ascii=False) + "\n")
            print(f"Wrote {out_path} ({result[key]['count']} cards)", file=sys.stderr)

    return result


def cmd_batch_all(args) -> dict:
    """Fetch every spirit listed in data/spirits.json. Rate-limit aware; cached."""
    spirits_json = Path(args.spirits_json or "data/spirits.json")
    registry = json.loads(spirits_json.read_text())
    results = {"spirits": [], "errors": []}
    for entry in registry["spirits"]:
        slug = entry["slug"]
        name = entry["name"]
        try:
            print(f"[{len(results['spirits']) + 1}/{len(registry['spirits'])}] {name}", file=sys.stderr)
            data = cmd_batch_spirit(args, name.replace(" ", "_"))
            if args.output_dir:
                out_dir = Path(args.output_dir)
                out_dir.mkdir(parents=True, exist_ok=True)
                (out_dir / f"{slug}.json").write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
            results["spirits"].append({"slug": slug, "name": name, "status": "ok"})
        except Exception as e:
            print(f"  error: {e}", file=sys.stderr)
            results["errors"].append({"slug": slug, "name": name, "error": str(e)})
    return results


def main():
    p = argparse.ArgumentParser()
    p.add_argument("command", choices=["spirit", "card", "aspect", "batch-spirit", "batch-all", "deck", "feb-deck"])
    p.add_argument("name", nargs="?", help="Wiki page name (spirit/card/batch-spirit) or deck type 'minor'/'major' for deck command")
    p.add_argument("--all-statuses", action="store_true", help="deck: keep all cards regardless of status (default keeps only Active)")
    p.add_argument("--output-dir", help="Write parsed output JSON here as {slug}.json")
    p.add_argument("--cache-dir", default=".wiki-cache", help="Cache raw fetches here (default: .wiki-cache)")
    p.add_argument("--no-cache", action="store_true", help="Bypass cache; always fetch fresh")
    p.add_argument("--delay", type=float, default=DEFAULT_DELAY_SECS, help=f"Minimum seconds between API requests (default: {DEFAULT_DELAY_SECS})")
    p.add_argument("--spirits-json", help="Path to spirits.json for batch-all (default: data/spirits.json)")
    args = p.parse_args()

    if args.command == "batch-all":
        data = cmd_batch_all(args)
        print(json.dumps(data, indent=2), file=sys.stderr)
        return

    if args.command == "deck":
        if not args.name or args.name not in ("minor", "major", "all"):
            p.error("'deck' requires name='minor', 'major', or 'all'")
        cmd_deck(args, args.name)
        return

    if args.command == "feb-deck":
        if not args.name or args.name not in ("fear", "event", "blight"):
            p.error("'feb-deck' requires name='fear', 'event', or 'blight'")
        cmd_fear_event_blight_deck(args, args.name)
        return

    if not args.name:
        p.error(f"'{args.command}' requires a page name")

    page = args.name.replace(" ", "_")

    if args.command == "spirit":
        data = cmd_spirit(args, page)
    elif args.command == "card":
        data = cmd_card(args, page)
    elif args.command == "aspect":
        data = cmd_aspect(args, page)
    else:
        data = cmd_batch_spirit(args, page)

    out_json = json.dumps(data, indent=2, ensure_ascii=False)

    if args.output_dir:
        out_dir = Path(args.output_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        fname = out_dir / f"{slugify(args.name)}.json"
        fname.write_text(out_json + "\n")
        print(f"Wrote {fname}", file=sys.stderr)
    else:
        print(out_json)


if __name__ == "__main__":
    main()
