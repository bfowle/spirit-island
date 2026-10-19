# Invader Lifecycle — Prevent Builds, Not Ravages

The single most-repeated fundamental in Spirit Island community writing: **stopping a Build is strictly better than stopping a Ravage of equal magnitude.** Yet new players instinctively target Ravages, because Ravages are the loud thing — they damage the land, they kill Dahan, they add Blight. Builds are quiet. They just put a Town down.

That Town Ravages next turn, and it stays to Build again (and to seed Explores next door) every time its terrain comes back. This chapter is the math and the heuristics behind why preventing Builds compounds.

## The three-card slide

Each Invader card lives in three slots across three turns:

```
Turn N:    Explore → [Build]  [Ravage]  (card A: just explored)
                                        (card B: Builds this turn)
                                        (card C: Ravages this turn)
Turn N+1:  Explore → [Build]  [Ravage]  (card B moves to Ravage)
                                        (card C discards)
Turn N+2:  Explore → [Build]  [Ravage]  (card A now Ravages)
```

A terrain that's in the Ravage slot right now Explored two turns ago. A terrain in the Build slot will Ravage next turn. An Explore happening now will Ravage in two turns. **Every card you see has a future role.** The lands Ravaging this turn are already set, and only Fast Powers resolve before that Ravage; your power curve should already be acting on the Build slot (preventing *next* turn's Ravage) and on the current Explore (preventing the Ravage two turns out).

The slide starts during Setup: the Invaders Explore the first card and place it on the Build space, so **Turn 1 Builds and Turn 2 is the first Ravage** (core rulebook, "Invaders' Starting Action"). Turn 1 is your first chance to stop a Build. At each Spirit Phase you know which lands Ravage this turn and which Build (and so Ravage next turn), but not which will Explore.

## Why Build-prevention beats Ravage-prevention

Say a land has 1 Town + 1 Explorer and Builds this turn. Two paths:

**Path A — let it Build, prevent the Ravage next turn.**
- Build turn: land becomes 1 Town + 1 City + 1 Explorer (Build rule: more Towns than Cities adds a City; otherwise, including equal, adds a Town — at 1 Town, 0 Cities, add a City).
- Next turn's Ravage: 6 damage (1 per Explorer + 2 per Town + 3 per City) → 1 Blight (which Destroys one Presence of each Spirit there) and up to 3 Dahan Destroyed, unless defended.
- You spent a Defend power next turn. The Town and the new City are still alive to Build next time their terrain rolls in.

**Path B — shrink or stop the Build this turn (with a Fast Power, so it lands before the Build resolves).**
- Destroy the Town (1 Fear): the land has 1 Explorer, so 0 Towns and 0 Cities → the Build adds a Town. Now 1 Town + 1 Explorer, and next turn's Ravage is 3 damage — cheaper to Defend, or survivable without it.
- Clear both pieces: with no Invaders in the land, it doesn't Build at all, and unless new Invaders arrive, it has nothing to Ravage with next turn.
- Push only the Explorer out: the lone Town still means more Towns than Cities, so a City is added (1 Town + 1 City, 5 damage). Little gained.
- You spent the prevention power this turn but saved a Defend next turn, and the land carries fewer Invaders into every later Build and Ravage.

```admonish tip title="The heuristic"
If a land is about to Build and you can afford exactly one intervention: **intervene on the Build**, not the Ravage. Builds compound; Ravages are discrete.
```

## When prevention breaks — Build-immune threats

Some lands can't be prevented via Build-disruption:

- **Cities**: a land Builds as long as it has any Invader, and a City (3 Health) is the hardest piece to clear. Killing the Explorers beside it changes nothing: the Build still adds a Town (Cities equal to or more than Towns) or another City (more Towns than Cities). You can't "prevent" the Build unless you Push/Destroy the City itself and turn the land into a no-Invader land (every piece gone).
- **Adversary builds and additions**: several Adversary rules Build or add pieces *outside* the Build card's own Build. England's Escalation (Building Boom, on Stage II Escalation cards): on each board with Towns/Cities, Build in the land with the most Towns/Cities. England L1+ also Builds in matching lands without Invaders that are adjacent to at least 2 Towns/Cities, and England L3+ adds a second Build slot (High Immigration) that Builds before each Ravage (at L3 until a Stage II card reaches it; from L4 all game). Sweden's Escalation replaces Dahan with Towns after Exploring, and Sweden L5 adds a Town next to a land that a Ravage Blighted. Clearing the lands that match the Build card doesn't stop these.
- **Event-card builds** (B&C+): Events can add pieces mid-phase. The Event is revealed in the Invader Phase, after you have chosen and paid for your Powers, so you can't plan this turn around it.

For those threats, prevention shifts to: **clear the Invader entirely**, or accept the Ravage and Defend hard.

## Downgrade as a damage primitive (Nature Incarnate)

NI introduced **Downgrade** as an alternative to Damage and Destroy. To Downgrade an Invader is to Replace it with the next-smaller type (NI rulebook, "Upgrade & Downgrade"):

- A City is replaced with a Town.
- A Town is replaced with an Explorer.
- Downgrading an Explorer Removes it.

This matters for Build-prevention math. A land with 1 City + 0 Towns Builds a Town (Cities equal to or more than Towns), ending at 1 City + 1 Town. Downgrading the City in the Fast phase, before the Build, turns the land into 1 Town + 0 Cities — which now Builds a **City** (more Towns than Cities), ending at 1 Town + 1 City. The next Ravage is 5 damage either way: one Downgrade before the Build bought nothing. Two Downgrades (City → Town → Explorer) leave 1 Explorer, which Builds a Town: 1 Explorer + 1 Town, a 3-damage Ravage. A third Downgrade Removes the Explorer, and the land doesn't Build at all. Before you commit, compare what two or three Downgrades cost you against Destroying the City outright (3 Damage) with the cards you actually hold.

```admonish info title="Downgrade and Fear"
A Downgrade is a Replace (or, for an Explorer, a Remove), not a Destroy, so it generates no Fear by itself; Fear comes from Destroying a Town (1) or City (2) and from Fear icons on the Power. The Downgraded piece keeps any Damage it had taken, so it is Destroyed at once if that Damage equals its new Health — and that Destroy generates Fear normally. (NI rulebook, "Upgrade & Downgrade"; core rulebook, "Fear and Terror".)
```

## Juggle anti-pattern — don't overspend on one land

A named Discord/Phantaskippy idiom: *don't juggle the invaders*. If stopping one land's Ravage takes two of your three card plays, you just paid double for a single-land outcome while another land Built and another Explored. Over a multi-turn horizon, spreading one prevention across three lands beats stacking two preventions on one land.

```admonish failure title="Signs you're juggling"
- You played Defend *and* a clear *and* a Push into the same land.
- Your partner's board has an unattended land that Built last turn and Ravages next.
- You "won" the land but the Fear deck didn't budge because no kills landed.
```

The rule of thumb: **one power per land, across lands.** Defend one. Clear one. Push pieces out of a third. Save the multi-power-stack play for when it wins the game (final-turn kill zone, or a cascade-preventing mega-Defend).

## Reading the Invader deck — pressure budget

The Invader deck's shape determines how much prevention you can afford. Without an Adversary the deck is 3 Stage I cards, then 4 Stage II, then 5 Stage III, each drawn from a slightly larger set with the rest returned to the box unseen. The setup Explore and Turns 1–2 reveal Stage I cards, Turns 3–6 Stage II, and Turns 7–11 Stage III (each card then Builds a turn later and Ravages two turns later). Adversaries change the deck, so recount for yours. Count per-Stage:

- **Stage I**: single-terrain cards (Jungle, Mountain, Sands, Wetland). Each action hits the two lands of one terrain on each board. Your prevention budget is highest here.
- **Stage II**: the same four terrains, now printed with the Escalation flag, plus the Coastal Lands card, which hits every Coastal land at once. With an Adversary, each Escalation card triggers that Adversary's Escalation effect when it is revealed (before the Explore unless the Adversary says otherwise); without one, ignore the flags.
- **Stage III**: two-terrain cards (Jungle+Sands, Jungle+Wetland, Mountain+Jungle, Mountain+Wetland, Sands+Mountain, Sands+Wetland), so every action hits four lands per board. Pressure peaks, and the deck is running out: if an Explore needs a card and the deck is empty, you lose.

## The single-Explorer rule

The most common tactical application:

> If a land about to Build contains only one Explorer, killing/pushing that Explorer cancels the entire Build for that land.

One card play for one piece of intervention, no Invader piece added. (Exceptions: England L1+ and Scotland L3+ let some Build cards affect lands without Invaders; check your Adversary.) Combined with Build-prevention compounding, single-Explorer lands are the cheapest high-value targets on the board. Experienced players pattern-match these lands before evaluating anything else.

## Common mistakes

```admonish failure title="Three patterns"
1. **Reacting to Ravages because they're loud.** The Ravage this turn is already happening; even if you Defend it to zero, the Invaders stay, and the land Builds and Ravages again the next time its terrain comes up (unless the Invaders are removed). Attacking the Build is the only durable move.
2. **Single-Explorer lands left alive.** Every unattended single-Explorer land gains a Town the next time its terrain Builds, and from then on that Town is an Explore source for every adjacent land.
3. **Overcounting Cities as Build-preventable.** A City in the land means that land Builds no matter what you do to the Explorers or other pieces. Either clear the City itself (and everything with it) or accept the downstream Ravage.
```

## Further reading

- [Board Shaping](board-shaping.md) — where to push, once you've decided to intervene on a Build or Ravage.
- [The Blight Track](blight-track.md) — "cut your losses" framework for when prevention fails.
- [Dahan](dahan.md) — Dahan counterattacks stack with Build-prevention to clear lands without your own presence risk.
- [Tempo](tempo.md) — the per-turn pressure budget that decides how many preventions you can afford.
- [Explore-phase rule reference](https://github.com/brettfowle/spirit-island/blob/main/data/references/rules/explore-phase.md) — Explorers-without-a-source don't seed, which is why single-Explorer kills are durable.

## Source notes

- Prevent-Builds priority formalized in [The Thoughtful Gamer's 7 Strategy Tips](https://thethoughtfulgamer.com/2019/05/15/7-strategy-tips-for-spirit-island/) and extended in [Greater Than Games forum "Cut Your Losses" thread](https://forums.greaterthangames.com/t/strategy-blight-and-presence-when-to-cut-your-losses/16920).
- Juggle anti-pattern: Phantaskippy's Wiki guides (Discord-originated), e.g. [A Spread of Rampant Green guide](https://spiritislandwiki.com/index.php?title=A_Spread_of_Rampant_Green/Phantaskippy%27s_Guide).
- Downgrade mechanics: Nature Incarnate rulebook, "Upgrade & Downgrade"; [Spirit Island Wiki — Downgrade](https://spiritislandwiki.com/index.php?title=Downgrade).
- Build rule, setup Explore, Invader deck Stages: core rulebook ("Making the Invader Deck", "Invaders' Starting Action", "Build") + [Sequence of Play](https://spiritislandwiki.com/index.php?title=Sequence_of_Play) §3.
- Adversary Build changes (England, Scotland, Sweden): the project rules register, `data/references/rules/register.md` §4.
