#!/usr/bin/env python3
"""
Apply inline Spirit Island Wiki icons to mdbook prose.

Replaces plain-text patterns like "2 Fear" / "Dahan" / "Moon" with an
accompanying `<img>` tag so readers get visual cues alongside the words.
Only modifies text OUTSIDE code fences, admonish blocks' config lines,
tables' structural markup, and mermaid blocks — prose is the target.

Idempotent: re-running on already-processed files should be a no-op (or
at worst replace `<img ...>` with the same tag). We achieve this by
skipping any match that already has an `<img class="si"` within a few
characters before it.

Usage:
    python3 scripts/apply-si-icons.py src/spirits/low/shadows-flicker-like-flame.md
    python3 scripts/apply-si-icons.py --default-set   # apply to Shadows chapter
"""

from __future__ import annotations
import argparse, re, sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

# ICON MAP: slug → filename under theme/icons/
ICON_FILES: dict[str, str] = {
    # Elements (color PNGs)
    "moon":    "element-moon.png",
    "sun":     "element-sun.png",
    "fire":    "element-fire.png",
    "air":     "element-air.png",
    "water":   "element-water.png",
    "earth":   "element-earth.png",
    "plant":   "element-plant.png",
    "animal":  "element-animal.png",
    # Units (black SVG silhouettes — CSS filters for dark mode)
    "explorer": "unit-explorer.svg",
    "town":     "unit-town.svg",
    "city":     "unit-city.svg",
    "dahan":    "unit-dahan.svg",
    # Resources
    "fear":        "resource-fear.svg",
    "blight":      "resource-blight.svg",
    "sacred-site": "resource-sacred-site.svg",
    # Speed
    "fast": "speed-fast.svg",
    "slow": "speed-slow.svg",
    # Terrain
    "mountain": "terrain-mountain.svg",
    "wetland":  "terrain-wetland.svg",
    "jungle":   "terrain-jungle.svg",
    "sands":    "terrain-sands.svg",
}


def icon_tag(slug: str, relative_to_src: bool = True) -> str:
    """Generate <img class="si"> tag for a given icon slug. The path is
    resolved relative to mdbook's html output, which is rooted at site-url.
    Since mdbook copies `theme/` to `theme/` in output, we use a relative
    path from src/*/*/*.md — two directories up to src, then ../theme/...
    """
    filename = ICON_FILES[slug]
    return f'<img class="si" src="/spirit-island/theme/icons/{filename}" alt="{slug.title()}">'


# Patterns: text contexts to inject icons into. Ordered = match priority.
# Each tuple: (compiled_pattern, replacement_fn).
# Replacement fn: takes match, returns replaced string (including original
# text, with icon injected).

def inject_before(text: str, icon_slug: str) -> str:
    """Prepend icon to the matched text (e.g., '2 Fear' → '2 <img> Fear')."""
    return f'{icon_tag(icon_slug)} {text}'


def inject_after(text: str, icon_slug: str) -> str:
    return f'{text} {icon_tag(icon_slug)}'


# Match `N Fear` / `1 Dahan` / `3 Explorers` etc — number + token.
NUM_TOKEN_PATTERNS = [
    (re.compile(r"\b(\d+)\s+(Fear)\b"),                 "fear"),
    (re.compile(r"\b(\d+)\s+(Dahan)\b"),                "dahan"),
    (re.compile(r"\b(\d+)\s+(Explorer)(s?)\b"),         "explorer"),
    (re.compile(r"\b(\d+)\s+(Town)(s?)\b"),             "town"),
    (re.compile(r"\b(\d+)\s+(City)\b|\b(\d+)\s+(Cities)\b"), "city"),
    (re.compile(r"\b(\d+)\s+(Blight)\b"),               "blight"),
]

# Match element names in threshold-like contexts: N Moon / N Fire / …
ELEMENT_THRESHOLD = re.compile(
    r"\b(\d+)\s+(Moon|Sun|Fire|Air|Water|Earth|Plant|Animal)\b"
)

# Standalone element-name mentions in display contexts like "Moon + Fire"
# (only when preceded/followed by another element or an element operator).
ELEMENT_IN_LIST = re.compile(
    r"(?<!\w)(Moon|Sun|Fire|Air|Water|Earth|Plant|Animal)(?!\w)"
)

# Standalone Fast / Slow words (speed descriptors)
SPEED_TAG = re.compile(r"(?<!\w)(Fast|Slow)(?!\w)")


def skip_match(md_lines: list[str], line_idx: int, col: int) -> bool:
    """Return True if the match is inside a code fence / admonish config /
    mermaid block / table row / HTML tag we shouldn't touch."""
    line = md_lines[line_idx]
    # Raw HTML tag contexts
    if "<img" in line[:col] and "</img" not in line[:col]:
        return True
    # Code span `...`
    before = line[:col]
    if before.count("`") % 2 == 1:
        return True
    # Admonish fence `admonish note "..."` — skip the title line
    if re.match(r"^\s*```admonish", line):
        return True
    return False


def in_code_fence(md_lines: list[str], idx: int) -> bool:
    """Track fenced code blocks."""
    in_fence = False
    fence_marker = None
    for i in range(idx + 1):
        stripped = md_lines[i].strip()
        if not in_fence:
            m = re.match(r"^(```|~~~)(.*)$", stripped)
            if m:
                in_fence = True
                fence_marker = m.group(1)
        else:
            if stripped == fence_marker or stripped.startswith(fence_marker):
                in_fence = False
                fence_marker = None
    return in_fence


def in_html_tag_attr(line: str, col: int) -> bool:
    """Very rough: if we're inside < > of an HTML tag at this column."""
    before = line[:col]
    last_lt = before.rfind("<")
    last_gt = before.rfind(">")
    return last_lt > last_gt


def process_line(line: str, in_code: bool) -> str:
    if in_code:
        return line
    # Skip lines that are obvious table separators / mermaid config /
    # admonish config.
    stripped = line.strip()
    if stripped.startswith("```") or stripped.startswith("~~~"):
        return line
    if re.match(r"^\s*\|[\s:|-]+\|\s*$", line):
        return line  # table separator row

    out = line

    # Replace number+token patterns first (these are most contextual).
    for pat, icon_slug in NUM_TOKEN_PATTERNS:
        def sub(m, icon=icon_slug):
            start = m.start()
            if in_html_tag_attr(out, start):
                return m.group(0)
            # Avoid double-injection if a tag sits immediately before us
            before = out[max(0, start - 60):start]
            if '<img class="si"' in before and before.rstrip().endswith('>'):
                return m.group(0)
            return f'{m.group(0)} {icon_tag(icon)}'
        out = pat.sub(sub, out)

    # Element thresholds: "2 Moon" → "2 <moon> Moon"
    def elem_sub(m):
        count = m.group(1)
        elem = m.group(2).lower()
        if elem not in ICON_FILES:
            return m.group(0)
        start = m.start()
        before = out[max(0, start - 60):start]
        if '<img class="si"' in before and before.rstrip().endswith('>'):
            return m.group(0)
        return f'{count} {icon_tag(elem)} {m.group(2)}'
    out = ELEMENT_THRESHOLD.sub(elem_sub, out)

    return out


def apply_to_file(path: Path, dry_run: bool = False) -> tuple[int, str]:
    """Process one markdown file. Returns (changes_count, new_content)."""
    content = path.read_text()
    lines = content.split("\n")
    new_lines: list[str] = []
    in_code = False
    fence_marker = None
    changes = 0
    for line in lines:
        stripped = line.strip()
        # Track fence state
        if not in_code:
            m = re.match(r"^(```|~~~)(.*)$", stripped)
            if m:
                in_code = True
                fence_marker = m.group(1)
                new_lines.append(line)
                continue
        else:
            if fence_marker and stripped.startswith(fence_marker):
                in_code = False
                fence_marker = None
            new_lines.append(line)
            continue

        processed = process_line(line, in_code)
        if processed != line:
            changes += 1
        new_lines.append(processed)

    new_content = "\n".join(new_lines)
    if not dry_run:
        path.write_text(new_content)
    return changes, new_content


# ─────────────────────────────────────────────────────────────────────────────
# Opener Turn snapshot injection
# ─────────────────────────────────────────────────────────────────────────────
#
# For each `#### Turn N` block in a spirit chapter's Possible Openings section,
# insert a summary snapshot right after the heading. The snapshot is a small
# <dl class="si-snapshot">...</dl> block showing what board/state looks like at
# that turn.
#
# Content is hand-authored per-spirit per-turn; this module only provides a
# helper that emits the HTML. To keep scope manageable, we currently bake
# Shadows's snapshots inline below.

SHADOWS_SNAPSHOTS: dict[str, str] = {
    # Keyed by (opening-letter, turn-number). Content is the <dl> innerHTML.
    "A-1": """<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G3</strong> revealing CP track — gain <strong>+3 Energy</strong> + 1 Presence Range 3</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>3E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> <img class="si" src="/spirit-island/theme/icons/speed-fast.svg" alt="Fast"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Mantle of Dread</strong> (self) <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> · 1E spent</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">1 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> Moon + 1 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> Air · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear generated</span> <span class="si-pool"><strong>2E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I — Explore only</span></dd>
</dl>
""",
    "A-2": """<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing Energy track (<code>energy1</code>) — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">2E banked + 1E income = <strong>3E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Crops Wither</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span> <span class="si-land"><strong>Favors Called Due</strong> <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">0 — no Fast cards played · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T2</dt>
  <dd><span class="si-pool">2–5 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 City → Town (if targeted)</span> <span class="si-pool">Dahan gathered</span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I+II — Build + Explore (first <strong>Build</strong>)</span></dd>
</dl>
""",
    "A-3": """<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G1</strong> — Reclaim (all played back to hand) + Gain 1 Minor</dd>

  <dt>Income</dt>
  <dd><span class="si-pool">1E banked + 1E income = <strong>2E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> (target Ravage land w/ Dahan) <img class="si" src="/spirit-island/theme/icons/speed-fast.svg" alt="Fast"> <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"></span> <span class="si-land">+ drafted Fast Moon+Fire Minor (if in hand) → L1 may fire</span></dd>

  <dt>Key T3 note</dt>
  <dd><span class="si-pool"><strong>First Ravage turn</strong> — Concealing's protection now <em>material</em></span></dd>

  <dt>Invader card</dt>
  <dd><span class="si-pool">Stage I+II+III — first <img class="si" src="/spirit-island/theme/icons/resource-blight.svg" alt="Blight"> <strong>Ravage</strong></span></dd>
</dl>
""",
    "B-1": """<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G2</strong> revealing CP track — gain 1 Minor + 1 Presence Range 1</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>0E</strong></span> <span class="si-pool"><strong>2CP</strong></span></dd>

  <dt>Best-case Fast plays</dt>
  <dd><span class="si-land"><strong>Concealing Shadows</strong> + <strong>Land of Haunts and Embers</strong> (drafted 0-cost Minor)</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">2 <img class="si" src="/spirit-island/theme/icons/element-moon.png" alt="Moon"> + 1 <img class="si" src="/spirit-island/theme/icons/element-fire.png" alt="Fire"> + 2 <img class="si" src="/spirit-island/theme/icons/element-air.png" alt="Air"> · <strong>L1 FIRES T1</strong> → Gather 1 Explorer</span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">3 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> gathered by L1</span> <span class="si-pool">up to 2 pushed by Haunts</span></dd>
</dl>
""",
    "C-1": """<dl class="si-snapshot">
  <dt>Growth</dt>
  <dd><strong>G3</strong> revealing Energy track — 1E income perm + 3E growth effect</dd>

  <dt>Income</dt>
  <dd><span class="si-pool"><strong>4E</strong></span> <span class="si-pool"><strong>1CP</strong></span></dd>

  <dt>Slow plays</dt>
  <dd><span class="si-land"><strong>Mantle of Dread</strong> (self) <img class="si" src="/spirit-island/theme/icons/speed-slow.svg" alt="Slow"> 1E · 2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear + push</span></dd>

  <dt>Fast-phase elements</dt>
  <dd><span class="si-land">0 — no Fast card · L1 <strong class="subtle">does NOT fire</strong></span></dd>

  <dt>End of T1</dt>
  <dd><span class="si-pool">2 <img class="si" src="/spirit-island/theme/icons/resource-fear.svg" alt="Fear"> Fear</span> <span class="si-pool"><strong>3E banked</strong></span> <span class="si-pool">1 <img class="si" src="/spirit-island/theme/icons/unit-explorer.svg" alt="Explorer"> + 1 <img class="si" src="/spirit-island/theme/icons/unit-town.svg" alt="Town"> pushed</span></dd>
</dl>
""",
}


def inject_snapshots(path: Path, snapshots: dict[str, str]) -> int:
    """Insert snapshot blocks after each `#### Turn N` header under an
    `### Opening X — ...` section. Only adds the snapshot when the header
    isn't already followed by one."""
    content = path.read_text()
    lines = content.split("\n")
    new_lines: list[str] = []
    current_opening: str | None = None  # 'A', 'B', 'C'
    inserts = 0

    opening_re = re.compile(r"^###\s+Opening\s+([A-Z])\b")
    turn_re = re.compile(r"^####\s+Turn\s+(\d+)\b")

    i = 0
    while i < len(lines):
        line = lines[i]
        new_lines.append(line)
        m_open = opening_re.match(line)
        if m_open:
            current_opening = m_open.group(1)
        else:
            m_turn = turn_re.match(line)
            if m_turn and current_opening:
                key = f"{current_opening}-{m_turn.group(1)}"
                snapshot = snapshots.get(key)
                if snapshot:
                    # Look ahead: is the next non-blank line already a si-snapshot?
                    j = i + 1
                    while j < len(lines) and lines[j].strip() == "":
                        j += 1
                    if j < len(lines) and "si-snapshot" in lines[j]:
                        i += 1
                        continue
                    # Insert blank line, snapshot block, blank line
                    new_lines.append("")
                    new_lines.extend(snapshot.rstrip("\n").split("\n"))
                    new_lines.append("")
                    inserts += 1
        i += 1

    path.write_text("\n".join(new_lines))
    return inserts


DEFAULT_TARGETS = [
    REPO / "src/spirits/low/shadows-flicker-like-flame.md",
]

AUTHORED_CHAPTERS = [
    # Low complexity
    REPO / "src/spirits/low/river-surges-in-sunlight.md",
    REPO / "src/spirits/low/lightning-swift-strike.md",
    REPO / "src/spirits/low/shadows-flicker-like-flame.md",
    REPO / "src/spirits/low/vital-strength-of-the-earth.md",
    REPO / "src/spirits/low/ocean-hungry-grasp.md",
    REPO / "src/spirits/low/a-spread-of-rampant-green.md",
    REPO / "src/spirits/low/thunderspeaker.md",
    REPO / "src/spirits/low/bringer-of-dreams-and-nightmares.md",
    REPO / "src/spirits/low/rising-heat-of-stone-and-sand.md",
    REPO / "src/spirits/low/sun-bright-whirlwind.md",
    REPO / "src/spirits/low/devouring-teeth-lurk-underfoot.md",
    REPO / "src/spirits/low/eyes-watch-from-the-trees.md",
    REPO / "src/spirits/low/fathomless-mud-of-the-swamp.md",
    # Moderate
    REPO / "src/spirits/moderate/keeper-of-the-forbidden-wilds.md",
    REPO / "src/spirits/moderate/heart-of-the-wildfire.md",
    REPO / "src/spirits/moderate/serpent-slumbering-beneath-the-island.md",
    REPO / "src/spirits/moderate/grinning-trickster.md",
]

# Non-authored spirit stubs (still get inline icon injection for prose that mentions units/elements)
NON_AUTHORED_SPIRITS = [
    REPO / "src/spirits/moderate/downpour-drenches-the-world.md",
    REPO / "src/spirits/moderate/finder-of-paths-unseen.md",
    REPO / "src/spirits/moderate/many-minds-move-as-one.md",
    REPO / "src/spirits/moderate/sharp-fangs-behind-the-leaves.md",
    REPO / "src/spirits/moderate/shifting-memory-of-ages.md",
    REPO / "src/spirits/moderate/shroud-of-silent-mist.md",
    REPO / "src/spirits/moderate/vengeance-burning-plague.md",
    REPO / "src/spirits/moderate/volcano-looming-high.md",
    REPO / "src/spirits/high/breath-of-darkness.md",
    REPO / "src/spirits/high/dances-up-earthquakes.md",
    REPO / "src/spirits/high/ember-eyed-behemoth.md",
    REPO / "src/spirits/high/fractured-days-split-the-sky.md",
    REPO / "src/spirits/high/hearth-vigil.md",
    REPO / "src/spirits/high/lure-of-the-deep-wilderness.md",
    REPO / "src/spirits/high/relentless-gaze-of-the-sun.md",
    REPO / "src/spirits/high/starlight-seeks-its-form.md",
    REPO / "src/spirits/high/stone-unyielding-defiance.md",
    REPO / "src/spirits/high/towering-roots-of-the-jungle.md",
    REPO / "src/spirits/high/wounded-waters-bleeding.md",
    REPO / "src/spirits/very-high/wandering-voice.md",
]

ADVERSARIES = [
    REPO / "src/adversaries/brandenburg-prussia.md",
    REPO / "src/adversaries/england.md",
    REPO / "src/adversaries/france-plantation-colony.md",
    REPO / "src/adversaries/habsburg-livestock-colony.md",
    REPO / "src/adversaries/habsburg-mining-expedition.md",
    REPO / "src/adversaries/russia.md",
    REPO / "src/adversaries/scotland.md",
    REPO / "src/adversaries/sweden.md",
]

SCENARIOS = [
    REPO / "src/scenarios/a-diversity-of-spirits.md",
    REPO / "src/scenarios/blitz.md",
    REPO / "src/scenarios/dahan-insurrection.md",
    REPO / "src/scenarios/despicable-theft.md",
    REPO / "src/scenarios/destiny-unfolds.md",
    REPO / "src/scenarios/elemental-invocation.md",
    REPO / "src/scenarios/guard-the-isles-heart.md",
    REPO / "src/scenarios/powers-long-forgotten.md",
    REPO / "src/scenarios/rituals-of-destroying-flame.md",
    REPO / "src/scenarios/rituals-of-terror.md",
    REPO / "src/scenarios/second-wave.md",
    REPO / "src/scenarios/the-great-river.md",
    REPO / "src/scenarios/varied-terrains.md",
    REPO / "src/scenarios/ward-the-shores.md",
]


def main():
    p = argparse.ArgumentParser()
    p.add_argument("targets", nargs="*", help="Specific markdown files to process")
    p.add_argument("--default-set", action="store_true", help="Apply to the Shadows chapter")
    p.add_argument("--all-authored", action="store_true", help="Apply to all 17 authored spirit chapters")
    p.add_argument("--non-authored", action="store_true", help="Apply to non-authored spirit stubs (moderate/high/very-high)")
    p.add_argument("--adversaries", action="store_true", help="Apply to all 8 adversary chapters")
    p.add_argument("--scenarios", action="store_true", help="Apply to all 14 scenario chapters")
    p.add_argument("--all", action="store_true", help="Apply to every tracked chapter (authored + non-authored + adversaries + scenarios)")
    p.add_argument("--snapshots", action="store_true", help="Also inject Shadows Turn snapshots")
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args()

    targets = [Path(t) for t in args.targets]
    if args.default_set:
        targets.extend(DEFAULT_TARGETS)
    if args.all_authored:
        targets.extend(AUTHORED_CHAPTERS)
    if args.non_authored:
        targets.extend(NON_AUTHORED_SPIRITS)
    if args.adversaries:
        targets.extend(ADVERSARIES)
    if args.scenarios:
        targets.extend(SCENARIOS)
    if args.all:
        targets.extend(AUTHORED_CHAPTERS + NON_AUTHORED_SPIRITS + ADVERSARIES + SCENARIOS)
    # Dedupe while preserving order
    seen = set()
    targets = [t for t in targets if not (t in seen or seen.add(t))]
    if not targets:
        p.error("no targets; pass filenames, --default-set, --all-authored, --non-authored, --adversaries, --scenarios, or --all")

    total_changes = 0
    for t in targets:
        if not t.exists():
            print(f"missing: {t}", file=sys.stderr)
            continue
        changes, _ = apply_to_file(t, dry_run=args.dry_run)
        print(f"{t.relative_to(REPO)}: {changes} line change(s)")
        total_changes += changes

        # Shadows-specific: inject opener snapshots
        if args.snapshots and t.stem == "shadows-flicker-like-flame":
            n = inject_snapshots(t, SHADOWS_SNAPSHOTS)
            print(f"{t.relative_to(REPO)}: {n} snapshot block(s) inserted")

    print(f"\nTotal line changes: {total_changes}")


if __name__ == "__main__":
    main()
