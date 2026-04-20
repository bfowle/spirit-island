# Design explorations

Standalone HTML mockups exploring different visual directions for si-live. Open any file in a browser — each is self-contained (no build, no deps).

## Files

1. **`01-atmospheric-night.html`** — what's currently shipped (warm-neutral dark + coral accent). Baseline for comparison.
2. **`02-warm-paper.html`** — light mode inspired by parchment / printed board-game notebooks. Sepia accent, warm beige surfaces.
3. **`03-minimalist-mono.html`** — strict grayscale, no accent. Typographic hierarchy does all the work. Zen / Anthropic-like.
4. **`04-data-first-terminal.html`** — information-dense, Bloomberg-terminal / tmux-status-line feel. Maximal per-screen data.

## How to pick a direction

Open all 4 side-by-side in tabs. For each, ask:

- Does it **surface the right information** at a glance? (Round, phase, fear/blight pools, spirit resources, board summary.)
- Does it feel **calm enough for long-session play**? (high-contrast strobes wear you down; muted palettes scale.)
- Does it **leave room for icons + spirit art** without feeling busy?

Reply with the direction you want to adopt (or a hybrid — "1 but lighter background") and I'll apply it to the live Vue components.

## Content parity

Each mockup renders the same synthetic state (Shadows vs. England L3, Round 3, mid-game) so visual comparison is apples-to-apples.
