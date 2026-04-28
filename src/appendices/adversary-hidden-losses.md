# Adversary Hidden Loss Conditions

The base loss condition is universal — fear deck exhausted **or** blight pool depleted **or** time runs out (Invader deck emptied). But several adversaries add *additional* loss triggers that are easy to forget in the mid-game and catastrophic when they fire. This appendix is the quick-reference for those hidden triggers.

Each entry cites the Wiki page for the adversary; check there for level-specific variations.

## The trigger table

| Adversary | Hidden loss trigger | First active at | Notes |
|---|---|---|---|
| **Scotland** | Cities-in-coastal-lands ≥ 2 × boards | L3+ | Mandatory watch on coastal City count once L3 escalation starts adding Coastal Cities on Explore. |
| **Habsburg Mining Expedition** | 8+ pieces in a land (Invaders + blight + mine) triggers land-specific loss | L0+ | Loss condition scales with level; mine tokens count as pieces for this purpose. |
| **Russia** | Dahan-destruction quota via Pogroms | L2+ | Specific Event + Escalation combination can eliminate Dahan faster than blight math suggests. |
| **Sweden** | No additional pure-loss trigger, but Ravage-on-coast Build-spam can flood coastal blight pool faster than blight-track anticipates | L3+ | Read as "blight-track loss arrives faster than expected" rather than a new loss line. |
| **France (Plantation Colony)** | Dahan-attract mechanic means standard loss conditions arrive via an unexpected path | L1+ | Dahan clustered for defense *attract* Invaders; your board shape defends poorly against Ravage-escalation unless re-planned. |
| **Brandenburg-Prussia** | Cascade-blight via Sands concentration | L4+ | Not a new loss *line* but a structural loss accelerator — one Sands Ravage can cascade 2 blight in a single turn. |
| **England** | Stage III coastal-Cities rush | L5+ | Not a new loss *line*; instead, fear-rush becomes the only viable win path as coastal Cities compound past board-clear speed. |
| **Habsburg Livestock Colony** | Animal-token accumulation leads to Escalation-based Invader spawning off-sequence | L2+ | Standard loss conditions, but one specific trigger is the Animal-token count passing a threshold on a bad Event. |

## Scotland — coastal-city cap

Scotland's most common hidden-loss surprise: the Stage II (L3+) escalation puts Cities on coastal lands via Explore, replacing what would have been Explorers with Cities. Combined with Scotland's Coastal Cities Build bias at higher levels, coastal lands can accumulate 2–3 Cities before the fear deck has moved meaningfully.

**Watch trigger**: at the end of Invader Phase T4, count coastal Cities across all boards. If > (1 × boards), the Scotland-specific loss timer is active. At > (2 × boards), standard base-game clocks have already failed; your win path is a direct coastal-City removal or immediate fear-rush to Terror 3.

Wiki: [Scotland](https://spiritislandwiki.com/index.php?title=The_Kingdom_of_Scotland).

## Habsburg Mining Expedition — 8-piece land-loss

Habsburg Mining's central mechanic: **mine tokens** placed on lands act as persistent Invader infrastructure. The hidden loss: when a land reaches 8+ pieces (counting Invaders, blight, *and* mine tokens), that land is specifically "lost" per the Habsburg rule — contributing to a cumulative loss-count triggered at a per-level threshold.

**Watch trigger**: a coastal land with 1 mine + 1 City + 2 Towns + 1 Explorer + 3 blight = 8 pieces. That configuration is closer than it reads — each Build adds a Town, each Ravage adds Blight, and the mine is permanent. Check land counts explicitly on turns following a Habsburg Build in lands already containing a mine.

Wiki: [Habsburg Mining Expedition](https://spiritislandwiki.com/index.php?title=Habsburg_Monarchy_(Mining_Expedition)).

## Russia — Dahan pogrom quota

Russia's escalation and Event interactions at L2+ can destroy Dahan at a pace that breaks the standard Dahan-absorbs-Ravage math. Pogrom events can destroy multiple Dahan in a single Event resolution. If you were planning on Dahan-absorb-the-Ravage as your defensive pattern, several Pogroms in succession make it non-viable.

**Watch trigger**: Dahan-destruction Events with no kill-trigger offset. In a solo Russia L3 game, losing 4+ Dahan by T5 means a defensive Dahan-based strategy has failed — switch to fear-rush or spirit-Defend-powers as your Ravage mitigation.

Wiki: [Russia](https://spiritislandwiki.com/index.php?title=Tsardom_of_Russia).

## Habsburg Livestock Colony — animal-token acceleration

Habsburg Livestock's Animal tokens accumulate on certain lands; when their count crosses a level-specific threshold, Events can pull Animal-count-modifier effects that spawn pieces off the standard Invader-deck sequence. The hidden loss is a sudden off-turn Invader addition that wasn't on the Invader card.

**Watch trigger**: end-of-phase Animal-token counts past L2's listed threshold. Plan draws assume Events can spike pressure unexpectedly.

Wiki: [Habsburg Livestock Colony](https://spiritislandwiki.com/index.php?title=Habsburg_Monarchy_(Livestock_Colony)).

## Usage note

**Reading per-adversary chapters is still required.** This appendix is a *quick-reference* for the hidden triggers, not a replacement for the Adversary chapter, which covers escalation timing, level-by-level strategy, and spirit counter-matchups. When you draft for a specific adversary, open both the adversary chapter and this table.

Several entries here are marked `[VERIFY — level-specific thresholds pending per-chapter review]` in spirit and indicate where we still owe deeper per-level numerics. Contributions welcome.

## Cross-references

- Per-adversary detailed chapters: `src/adversaries/{adversary-slug}.md`.
- [Blight Track](../fundamentals/blight-track.md) — the base loss condition that hidden triggers usually accelerate.
- [Matchup Axis Reference](matchup-axis.md) — community's spirit × adversary rating framework; hidden-loss triggers drive many 1-star and 2-star cells.
- [Wiki — List of Adversaries](https://spiritislandwiki.com/index.php?title=List_of_Adversaries) — canonical source for level-specific rule text.
