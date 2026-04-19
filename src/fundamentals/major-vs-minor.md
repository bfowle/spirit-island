# Major vs. Minor — A Stats-Backed Framework

This is the most important draft decision in Spirit Island, and also the one most often handled by feel rather than framework. This chapter gives you the framework.

## The core question

Every time you grow on a "gain a power card" slot, you choose: **Minor** (cheap, supportive, 4 cards offered) or **Major** (expensive, game-changing, 4 cards offered, costs you a Forget).

Over a game, most spirits gain 3–6 power cards. Getting the Minor/Major ratio right is worth ~15–20% on your win rate at high difficulty. Getting it wrong is how you lose games you "should" have won.

## The four archetypes

Spirits fall into four draft biases. Learn which archetype your spirit is before you draft.

### 1. Minor-heavy

Cheap, curve-fit, multi-card plays every turn. Minors slot naturally into these spirits' energy + CP.

**Spirits here**: Sharp Fangs, Many Minds (opening 1), Downpour, Shroud (opening 1), Lightning, River (openings 1–2 early).

**Rule**: gain 2–3 Minors before you even think about a Major. If your Minors are all 0–1 cost, you can sustain 3+ CP per turn at 2–3 energy. Majors would starve the curve.

### 2. Major-heavy

Expensive, game-changing, spirit wants to hit a threshold (usually 3 CP + 3E+) to fire Majors reliably.

**Spirits here**: Vital Strength of the Earth (classic), Keeper, Stone's Unyielding, Starlight, Covets.

**Rule**: go Major as early as your curve supports — often T2–T4. Forget a weak Unique; 0 Minors is fine. Shopping 3+ Majors over a game is normal.

### 3. Mixed

Some Minors to sustain the curve, 1–2 high-value Majors for late-game closing. Most spirits default here.

**Spirits here**: Thunderspeaker, Shadows, Ocean, Green, Bringer, Fractured Days, Trickster (all openings), Finder, Serpent.

**Rule**: gain 2 Minors T1–T3, then a Major T4–T6, then another Minor or Major depending on what the matchup demands.

### 4. Flexible / Adversary-driven

The spirit's strength is breadth — different matchups want different drafts. Play the adversary, not the spirit.

**Spirits here**: Wildfire (Early Major vs England; Full-bottom minors vs Sweden), Sharp Fangs across its 4 openings, Lure (depends on opening choice), Behemoth.

**Rule**: read the adversary's pressure curve first (see [Tempo](./tempo.md)), then pick archetype 1–3 to match.

## The four decision factors

In order of weight:

### 1. Energy curve fit

**The question**: does your spirit generate enough energy by T3 to play the Major you gain?

Majors cost 3–8 energy. A 4-cost Major played on T4 means you hit 4E by T4. Most spirits that aren't Earth don't.

```admonish tip title="Pro Tip"
Before gaining a Major, check: "Can I play this card within 2 rounds of gaining it?" If no, pass. Majors rotting in hand are pure tempo loss.
```

### 2. Forget cost

When you gain a Major, you forget a card. If your spirit has a bad Unique (classic: Earth's Draw of the Fresh Waters on some matchups; Wildfire's Flash-Fires), that's a bonus.

If your spirit has no weak Unique, forgetting is a real cost. Minors don't ask for a Forget — they're free to slot in.

### 3. Card pool variance

Majors have 4-card offers that skew toward specific elements or effects. If your spirit has narrow element requirements, the Major offer often has 0–1 useful cards. Passing and going Minor is better than taking a misfit Major.

Minors have 4-card offers with better average fit for most spirits because the minor pool is broader + cheaper.

### 4. Game-phase fit

- **Early game (T1–3)** — Minors almost always win. They're cheap enough to play immediately.
- **Mid (T4–6)** — Majors start paying off. Mixed archetypes pivot here.
- **Late (T7+)** — either you've already shopped Majors, or you're playing Minor-heavy and don't care.

## The mermaid decision tree

```
          Turn 1–3: which archetype is your spirit?
                            |
           +----------------+----------------+
           |                |                |
      Minor-heavy       Mixed           Major-heavy
           |                |                |
     Gain Minors     Gain Minors T1–3   Gain Major T1–3
     all the way     Major T4–6         More Majors later
                     Minor/Major late
```

If you play **Flexible**, read the adversary first — see the Adversary Matchup Matrix in that spirit's chapter.

## The stats

```admonish note title="Stat Insight"
Per mindwanderer (2026-Q1 digital data, N ~ 12,000 games L3+):

- **Minor-heavy spirits winning runs**: 78% of games drafted 3+ Minors before any Major.
- **Major-heavy spirits winning runs**: 64% drafted a Major by T3.
- **Cross-spirit overall**: win rate is ~8% higher when the draft matches the archetype classification above.

Sample sizes vary per spirit. Use the per-spirit chapter's draft-bias rating, which accounts for the stat.
```

See the [Major vs. Minor Draft Rates](../statistics/major-vs-minor-rates.md) chapter for the full per-spirit breakdown.

## The passing game

Sometimes the right play is **decline a card gain**. You can't decline a growth option that says "gain a card," but you can pass on the card offered and take a Forget-only move, or you can take a 4th card you won't use just to avoid a Forget.

You pass a Major when:
- No card in the offer fits your element profile.
- Your energy curve won't support it for 2+ turns.
- Your Unique cards are all too valuable to Forget.

You pass a Minor when:
- Rare — almost never pass a Minor.
- Exception: hand size cap constraints (some spirits cap lower).

## The "just take the shiny" trap

Majors look great on the card. Vigor of the Breaking Dawn reads like the game-winner. But if you play it once at T6 and never again, you overpaid.

**Rule of thumb**: a Major is only worth its cost if you play it **twice** in the game. Once-and-done Majors are net losses unless they outright closed the game.

```admonish failure title="Common Mistake"
Drafting Vigor / Terrifying Nightmares / Sea Monsters / Fiery Power on a spirit that can't hit the element threshold for the full effect. You just paid the Major cost for a mediocre sub-threshold effect.
```

## A per-spirit rule of thumb

From the priority spirits (see per-chapter pages for the full stat-backed recommendations):

| Spirit             | Draft Bias     | Opening card rule                                        |
|--------------------|----------------|----------------------------------------------------------|
| Shadows            | Mixed          | Minors T1–T2, Major T3–T5 if Moon/Fire majors offered    |
| Thunderspeaker     | Mixed          | Minors T1–T3, pivot Major late; dahan-synergy majors OP  |
| Ocean              | Mixed          | 2 Minors early, Major T5+ once drowning is reliable      |
| Green              | Major-heavy    | Major T2–T3 if Plant majors offered; 0–1 Minor total     |
| Bringer            | Mixed          | Minors first; Terrifying Nightmares late as finisher     |
| Fangs              | Minor-heavy    | 3 Minors before a Major; see Fangs chapter for variants  |
| Keeper             | Major-heavy    | Major T2–T3; the spirit's energy supports it             |
| Serpent            | Mixed          | Minor-heavy early; Major when element profile fits       |
| Finder             | Mixed          | Minors for mobility; 1 Major to close                    |
| Downpour           | Minor-heavy    | Minors complementing existing toolkit (per latentoctopus)|
| Fractured Days     | Mixed          | Majors once time-skip engine is online; early Minors     |
| Many Minds         | Minor-heavy    | 0-cost Minors per Rei/latentoctopus; Major only late    |
| Shifting Memory    | Mixed          | Card-focused spirit; see chapter                         |
| Shroud             | Minor-heavy    | 0-cost Minors; Major only once energy pipeline mature   |
| Stone              | Major-heavy    | The paradigm Major-heavy spirit; Major T2–T4             |
| Vengeance          | Mixed          | Disease/Blight Majors shine; Fetid Breath is A-tier      |

## Cross-references

- [Tempo](./tempo.md) — why curve fit matters.
- [Energy Curves](./energy-curves.md) — per-spirit target energy trajectories.
- [Card Draft Theory](./card-draft-theory.md) — the deep mechanics of the draft itself.
- [Major vs. Minor Draft Rates](../statistics/major-vs-minor-rates.md) — the full stat page.

```admonish abstract title="Sources"
- latentoctopus opening guides (per-spirit draft emphasis): https://latentoctopus.github.io/
- Rei on Vengeance (card priority grades): BGG #2709070
- mindwanderer digital win-rate dataset: https://mindwanderer.net/si/stats.html
- Cardboard Crew tier list + draft notes: https://thecardboardcrew.com/spirit-island-spirits/
```

---

*Last revised: 2026-04-19*
