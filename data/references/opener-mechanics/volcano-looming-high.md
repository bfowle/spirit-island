## Opener Mechanics — starter reference

```admonish abstract title="Mechanically-verified starting state"
Auto-derived from `data/references/wiki/volcano-looming-high.json`. This section states the **factual mechanics** every opener must build on (starting income, growth options, innate thresholds, Fast-vs-Slow timing). It is **not** a strategic opener — use this as the foundation, then apply [Deliberate Play](../../fundamentals/deliberate-play.md) + `si-rules-check` before writing T1/T2/T3 prose.
```

### Starting state

- **Setup**: Put 1 Presence and 1 Badlands on your starting board in a mountain of your choice. Push all Dahan from that land.
- **Starting income** (from `presence_energy_track[0]` = `energy1`, `presence_cardplay_track[0]` = `card1`): **1 Energy · 1 Card Play**
- **Hand at start**: 4 Unique Power Cards (listed below)
- **Growth type**: `one` — pick **one** growth option per turn

### Growth options

- **G1**: reclaim (Reclaim all discarded+played Power Cards); gain1p (Gain 1 Power Card (Minor unless otherwise noted)); energy3 ((+3 Energy this turn — growth effect, not track reveal))
- **G2**: addpresence0 (Place 1 Presence from a track (Range 0)); addpresence0 (Place 1 Presence from a track (Range 0))
- **G3**: gain1p (Gain 1 Power Card (Minor unless otherwise noted)); addpresence4 (Place 1 Presence from a track (Range 4)); new+1cardplay ((spirit-specific: `new+1cardplay` — consult spirit panel))

**Presence-track reveal rules**: placing Presence (via a growth option with `addpresence*`) reveals **one** track slot — either the next Energy slot or the next Card-Play slot, not both. The choice determines your permanent-income trajectory from that turn onward.

### Energy track

`energy1 · energy2 · earth · energy3 · energy4 · energy5` — income as slots reveal: 1 → 2 → earth → 3 → 4 → 5

### Card-play track

`card1 · fireX · earthX · card2 · airX · card3 · fireX · card4` — CP as slots reveal: 1 → fireX → earthX → 2 → airX → 3 → fireX → 4

### Innate Powers

- **EXPLOSIVE ERUPTION** (Speed: Fast · Range: 0 · Target: any)
  - **L1** — (no threshold): Destroy X (1 or more) of your Presence in target land; {{volcanodestroyedpresence}} checks how many you destroyed. This Power does Damage (separately and equally) to both Invaders and Dahan. Ranges below can't be increased.
  - **L2** — 2 Fire + 2 Earth: In one land within Range 1, X Damage.
  - **L3** — 3 Fire + 3 Earth: Generate X Fear.
  - **L4** — 4 Fire + 2 Air + 4 Earth: In each land within Range 1, 4 Damage. Add 1 Blight to target land; doing so does not Destroy your Presence.
  - **L5** — 5 Fire + 3 Air + 5 Earth: In each land within Range 2, +4 Damage. In each land adjacent to the target, add 1 Blight if it doesn't have any.
- **POWERED BY THE FURNACE OF THE EARTH** (Speed: Slow · Range: 0 · Target: any)
  - **L1** — 3 Earth: Add 1 of your destroyed Presence.
  - **L2** — 3 Fire: Gain a Power Card.
  - **L3** — 4 Fire + 4 Earth: Move up to 2 of your Presence from other lands to target land.
  - **L4** — 5 Fire: Return up to 2 of your destroyed Presence to your Presence tracks.

### Fast-phase element ceiling from Uniques

Fast innates resolve in Fast phase and can only see elements from **Fast cards played before the innate**. Slow-card elements arrive too late to feed a Fast innate. This is the element ceiling Fast plays from your Uniques alone can contribute each turn:

- Fast-phase Unique elements: _(no Fast Uniques — all innate firings require drafted Fast cards)_
- **Fast-phase L1 ceiling from Uniques alone is sufficient** — you can fire L1 T1 without drafting (play enough Fast Uniques to meet the threshold).

### Unique Power Cards

*No Unique cards parsed.*

### Invader phase by turn (base deck)

| Turn | Explore | Build | Ravage | Notes |
|------|---------|-------|--------|-------|
| 1 | ✓ | — | — | Ravage-protection effects are **dormant T1**. |
| 2 | ✓ | ✓ | — | First Build; Ravage-protection still dormant. |
| 3 | ✓ | ✓ | ✓ | First Ravage; Ravage-protection becomes material. |
| 4+ | ✓ | ✓ | ✓ | Full cycle continues. |

Adversary escalation can shift this — check the adversary JSON for deviations (Sweden front-loads a Build; some Habsburg levels add early Builds).

### Pause-point before writing T1 prose

```admonish warning title="Before claiming what T1 does"
1. **Compute post-growth E/CP** for every growth × track-choice branch. Don't assume both tracks reveal simultaneously.
2. **Enumerate legal T1 plays** — subsets of hand with sum(costs) ≤ E and count ≤ CP.
3. **Separate Fast vs. Slow elements** — when claiming an innate fires, verify the threshold is met using only elements from its resolution phase (Fast sees Fast; Slow sees Fast + Slow).
4. **Flag dormant effects** — Ravage-protection, Defend N, etc. are **null T1/T2** in base play. Only cite them as opener value when the trigger actually occurs that turn.
5. **State per-turn material effect** for every card play: Fear generated, units pushed/gathered/destroyed, elements contributed. Never narrate dormant effects as if they were active.
```
