# How to Use This Book

## The three reading modes

This book is optimized for three different reading sessions. Pick the one that matches what you're doing right now.

### Mode 1 — Read Before Your Next Game (15–30 min)

You have time before play. You know your spirit and adversary. Do this:

1. Open the spirit's chapter in [Part III](./spirits/index.md).
2. Read **At a Glance** + **Spirit Overview** + **Key Strategic Principles**. That's ~600 words.
3. Scan the **Adversary Matchup Matrix** row for your adversary. If the grade is B or worse, read the matching row carefully and note the 1–2 reasons.
4. Read the **Opening Strategy** subsection matching your draft choice.
5. Skim the **Common Mistakes** list. That's the payoff per unit of reading time.

If your adversary is one you haven't played much, add the adversary's At a Glance + Spirits That Shine/Struggle from [Part IV](./adversaries/index.md). Another ~300 words.

### Mode 2 — Build a Session Plan (45–90 min)

You're new to the spirit *and* new to the adversary. Or you've been losing and want to diagnose why. Do this:

1. Read the spirit chapter fully, including Card Priority Ratings and Tempo Profile.
2. Read the adversary chapter's level-by-level breakdown for your chosen level.
3. Pick an Opening Strategy variant; write down its round-by-round goals on paper (literally — pen on paper, not a screen).
4. Play.
5. Debrief with the [`si-post-game`](./appendices/claude-skills-guide.md) Claude skill and compare your turns against the chapter's Common Mistakes.

This is the **highest-leverage** mode. Most readers should default to this for any new spirit/adversary combination.

### Mode 3 — At-the-Table Lookup (under 30 sec)

Mid-game. You have a decision and need signal, not a dissertation. Do this:

- Use the [`si-at-the-table`](./appendices/claude-skills-guide.md) Claude skill. You describe board state; it returns options with tradeoffs, not a solve.
- Or, if you have the book open on a second screen: search for the specific card name, or jump to the spirit's **Tempo Profile** for the current round.

Never try to read a whole chapter mid-game. It takes too long and breaks the table's flow.

## The active-learning loop

The book alone won't make you a master. Books rarely do. The loop is:

1. **Challenge** — generate today's drill with [`si-daily-challenge`](./appendices/claude-skills-guide.md). It picks a spirit × adversary × level × learning-goal based on your play history.
2. **Read** — spend 15 minutes on the spirit chapter (Mode 1 above).
3. **Play** — the challenge. Solo, multi-handed, or at-the-table.
4. **Log & Debrief** — run [`si-post-game`](./appendices/claude-skills-guide.md). It logs to `data/playlog.csv` and surfaces the 1–2 mistakes most likely to be what cost you this game.
5. **Iterate** — tomorrow's challenge shifts based on what you logged.

Repeat. After ~20–30 sessions you'll notice you're reading chapters *less*, because the patterns are in your head.

## Notation you'll see everywhere

This book uses the community-standard notation popularized by latentoctopus. Memorize this table:

| Notation        | Meaning                                                        |
|-----------------|----------------------------------------------------------------|
| **Gn**          | the *n*-th growth option, left to right (1-indexed)           |
| **nE**          | *n* energy per turn (e.g., 2E = 2 energy/turn)                |
| **nCP**         | *n* card plays per turn                                       |
| **x/y**         | x energy + y card plays (e.g., 3/2)                           |
| **CE**          | Cumulative Energy across the whole game                       |
| **Full bottom** | opening that empties the bottom presence track                |
| **Full top**    | opening that empties the top presence track                   |
| **Hybrid**      | opening that takes from both tracks roughly equally           |
| **Minor / Major / Mixed** | power-card draft emphasis                            |
| **Reclaim cycle** | turns between one Reclaim and the next                      |
| **Reclaim loop** | reclaiming every turn                                        |

## The admonition palette

Six kinds of callout blocks, each with a specific meaning. See [ADMONITION_STYLE](https://github.com/bfowle/spirit-island/blob/main/templates/ADMONITION_STYLE.md) for the style guide.

- **Pro Tip** — actionable in-game advice.
- **Common Mistake** — frequent trap + why.
- **Stat Insight** — data-backed claim with source/date/sample.
- **Rules Clarification** — from Querki FAQ, not opinion.
- **Sources** — citations at section or chapter end.
- **Math** — probability calculations, worked examples.

## Confidence ratings (inherited from latentoctopus)

Opening-strategy subsections sometimes carry a confidence badge:

- 🟥 **Red** — tentative / theorycrafting.
- 🟨 **Yellow** — somewhat confident, limited play-testing.
- 🟩 **Green** — well-tested, consistent success.

If a chapter doesn't carry a badge, assume the content is in the "yellow" tier.

## How to give feedback

This book is **a living document** and feedback changes it.

- If a chapter's advice failed you mid-game, note it in your `si-post-game` entry. Batched across sessions, these become revision tickets.
- If a stat is stale (digital patch shifted things), open an issue on the repo or DM Brett.
- If a common community pattern isn't covered, same.

See [Meta Changelog](./appendices/changelog-of-meta.md) for the running edit log.

---

*Last revised: 2026-04-19*
