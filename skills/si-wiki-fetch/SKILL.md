---
name: si-wiki-fetch
description: Use when you need authoritative Spirit Island mechanical data (spirits, unique cards, innates, special rules) — NEVER write mechanics from memory or from LLM-summarized WebFetch. This skill invokes `scripts/wiki-fetch.py`, a deterministic MediaWiki-API-based parser that returns exact card text, costs, speeds, elements, and thresholds. Use before authoring or revising any spirit chapter, card reference, or fact-checking a mechanical claim.
user-invocable: true
---

# si-wiki-fetch — Deterministic Spirit Island Wiki parser

## Why this exists

Previous attempts to fetch Wiki data via WebFetch (which runs an LLM summarizer over the page) produced **hallucinated specifics** — wrong costs, wrong speeds, wrong effect text, wrong elements. That data is unreliable.

This skill uses `scripts/wiki-fetch.py` to hit the MediaWiki API directly, get raw wikitext, and parse the `{{PowerCardArticle|...}}` / `{{Spirit|...}}` template fields deterministically. No LLM in the pipeline. No hallucination.

## Hard Rules

1. **Before citing any mechanical detail** (card cost, speed, range, target, elements, effect text, innate thresholds, special rules): run this skill.
2. **Never paraphrase card text in a way that invents mechanics.** The parser output is the canonical form; copy it verbatim into chapters.
3. **If the Wiki page is missing or the template fields are absent**: say so explicitly. Flag `[VERIFY physical copy]` rather than inventing.
4. **Cache respected**: the script caches under `.wiki-cache/` — re-runs are free. Only use `--no-cache` if a Wiki page has been updated recently.
5. **Rate-limiting respected**: default 1.5s between requests; exponential backoff on 429/5xx. Don't override `--delay` below 1s unless you have a reason.

## How to invoke

```bash
# Single card
python3 scripts/wiki-fetch.py card "Crops Wither and Fade"

# Single spirit (spirit panel only — innates, special rules, unique-card names)
python3 scripts/wiki-fetch.py spirit "Shadows Flicker Like Flame"

# Spirit + all its Unique cards (most useful for authoring)
python3 scripts/wiki-fetch.py batch-spirit "Shadows Flicker Like Flame" --output-dir data/references/wiki/

# Every spirit in data/spirits.json (takes a while due to rate-limiting)
python3 scripts/wiki-fetch.py batch-all --output-dir data/references/wiki/
```

## Output shape

### `spirit` / `batch-spirit` output

```json
{
  "type": "spirit",
  "name": "Shadows Flicker Like Flame",
  "expansion": "Base Game",
  "complexity": "low",
  "special_rules": "SHADOWS OF THE DAHAN ...",
  "innates": [
    {
      "name": "DARKNESS SWALLOWS THE UNWARY",
      "speed": "fast",
      "range": "1",
      "target": "any",
      "option": "sacred",
      "thresholds": [
        {"moon": "2", "fire": "1", "effect": "Gather 1 Explorer."},
        ...
      ]
    }
  ],
  "unique_cards": ["Concealing Shadows", "Crops Wither and Fade", ...],
  "unique_card_details": [ ... ]  // only if batch-spirit
}
```

### `card` output

```json
{
  "type": "power_card",
  "name": "Crops Wither and Fade",
  "spirit": "Shadows Flicker Like Flame",
  "card_type": "Unique",
  "cost": "1",
  "speed": "Slow",
  "range": "0",
  "target": "Any Land",
  "elements": ["moon", "fire", "plant"],
  "text": "2 Fear. Replace 1 Town with 1 Explorer. **OR** Replace 1 City with 1 Town.",
  "threshold": "No Threshold"
}
```

## Canonical workflow when writing a spirit chapter

1. Run: `python3 scripts/wiki-fetch.py batch-spirit "Name" --output-dir data/references/wiki/`
2. Open the resulting JSON. Copy-paste into the chapter:
   - **Special Rules** section: use `spirit.special_rules`.
   - **Innate Powers** section: list each innate with `name`, `speed`, `range`, `target`, and each threshold's elements + effect.
   - **Unique Cards** section: for each card in `unique_card_details`, write a block with cost, speed, range, target, elements, text.
3. Brett's At-a-Glance table: expansion = `spirit.expansion`; complexity = `spirit.complexity`; aspects **still need physical-copy verification** (the Wiki doesn't surface aspect lists cleanly).
4. Strategic-framing prose should be built *from* the mechanical facts, not in advance of them.

## Known limitations

- **Aspects list**: not yet scraped. Wiki stores aspects on separate pages; the spirit-page template doesn't list them cleanly. Flag aspects with `[VERIFY]` until scripted.
- **Cost/speed/target when card page lacks a field**: the parser reports whatever the Wiki has; missing fields come through as `null` / unset.
- **Element icons in text**: the parser replaces `{{fear}}` → "Fear", `{{explorer}}` → "Explorer", etc. See `TOKEN_MAP` in the script for the full list.
- **Adversaries, scenarios, event cards, fear cards, blight cards**: not yet supported — the script currently has parsers for `{{Spirit}}` and `{{PowerCardArticle}}` templates only. Extensions welcome.

## When to pull fresh data

- A spirit/card recently had errata → use `--no-cache` on that specific page.
- You're writing a chapter for the first time → batch-spirit will cache everything, safe to rerun.

## Never use WebFetch for mechanical data

If you find yourself reaching for `WebFetch("https://spiritislandwiki.com/...")`: stop and use this skill instead. WebFetch runs LLM summarization; that's how the original hallucinations happened.
