# Rule Reference — Explore (Invader Phase, step 3c)

One-line summary: During Explore, add one Explorer to every land matching the revealed terrain that contains — or is adjacent to — a Town, City, or Ocean.

## Canonical rule text (verbatim)

Source: Spirit Island Wiki, [Sequence of Play](https://spiritislandwiki.com/index.php?title=Sequence_of_Play), revision 12144, section **3c. Explore**.

> **3c. Explore**
>
> Turn the top card of the Invader Deck face-up. Invaders Explore in accessible lands of the shown type, venturing forth from Towns and Cities or approaching from the Ocean. If the card has a flag icon and you are playing with an Adversary, first perform the Escalation effect. If there is no card to turn up, time has run out and **the spirits lose the game.**
>
> Add an Explorer to every land of the shown type which either:
>
> - Contains a Town or City; or
> - Is adjacent to a Town, City, or Ocean.
>
> No matter how many sources are in or adjacent to an Explored land, you only add one Explorer. Explorers are added directly from the supply, not moved around on the board.

## Why this matters for board-shaping

1. **Coastal lands are permanent Explore sources.** Any land adjacent to the Ocean is always eligible to receive an Explorer on a matching Explore draw — independent of whether you cleared its buildings. You cannot "Explore-lock" a coastal land short of removing it from the map entirely (scenarios aside).
2. **Interior lands can go Explore-dead.** An interior land with no Town/City of its own, and no Town/City in any adjacent land, receives no Explorer on a matching Explore draw. Clearing the building chain in a region turns those kills durable.
3. **One Explorer per land, regardless of source count.** A land adjacent to two Towns and one City still gets exactly one Explorer per matching Explore draw. Sources are binary (yes/no) for the purposes of this rule, not cumulative.
4. **Explorers are not a source.** A land containing only an Explorer (and no Town/City) does **not** seed Explore into adjacent lands. Confirmed via the Explorer page FAQ: *"Explorers are not themselves a source when Invaders Explore."* (Wiki, [Explorer](https://spiritislandwiki.com/index.php?title=Explorer) rev. 7992.)

## Adjacency clarifications

- The Ocean counts as an adjacent "Ocean" for every coastal land — this is the text's third source type.
- Lands are not adjacent to themselves; the self-contains clause ("contains a Town or City") handles the land-itself case separately from the adjacency clause.
- When a spirit power or special rule makes two lands "adjacent" (Finder of Paths Unseen's *Open the Ways*, etc.), that adjacency counts for Explore — a Town/City in one of the newly-adjacent lands can seed an Explore into the other.

## Edge cases / rule interactions

- **Scotland L1 Trading Port** errata: "in Coastal lands, Explore cards add 1 Town instead of 1 Explorer." Errata confirmed on [Explorer page](https://spiritislandwiki.com/index.php?title=Explorer) FAQ. An Event or Adversary combination that adds 2 Explorers instead becomes "1 Town + 1 Explorer" under this errata.
- **Wilds token** (JE): modifies Explore in tagged lands — see the [Wilds](https://spiritislandwiki.com/index.php?title=Wilds) page for the specific effect. Not a universal rule; triggers only when the token is present.
- **Adversary Escalation** with a flag-icon Explore card fires *before* the Explore resolves — Escalation may add pieces (including buildings) that then count as sources for the same turn's Explore.
- **No-Explore-source lands** (interior lands with no adjacent Town/City and no adjacent Ocean) are the only lands that can be truly Explore-locked. Identifying these lands is the mechanical basis of the "Explore-lockout" tactic.

## Citation hygiene

When citing this rule in a chapter, link back to this file. When the rule text itself is quoted, quote verbatim — do not paraphrase. If the Wiki page revision advances, verify the quoted text still matches and update the REVID above.

Fetched 2026-04-20 via `curl` against `spiritislandwiki.com/api.php`. Fallback source for the rulebook PDF: base rulebook, page 9 (Invader Phase, step 3c).
