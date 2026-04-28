# Invader Lifecycle — Prevent Builds, Not Ravages

The single most-repeated fundamental in Spirit Island community writing: **stopping a Build is strictly better than stopping a Ravage of equal magnitude.** Yet new players instinctively target Ravages, because Ravages are the loud thing — they damage the land, they kill Dahan, they add Blight. Builds are quiet. They just put a Town down.

That Town is next turn's Ravage. And next turn's Ravage adds another Town. This chapter is the math and the heuristics behind why preventing Builds compounds.

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

A terrain that's in the Ravage slot right now Explored two turns ago. A terrain in the Build slot will Ravage next turn. An Explore happening now will Ravage in two turns. **Every card you see has a future role.** The Ravage firing this turn is a done deal; your power curve should already be acting on the Build slot (preventing *next* turn's Ravage) and on the current Explore (preventing the Ravage two turns out).

## Why Build-prevention beats Ravage-prevention

Say a land has 1 Town + 1 Explorer and Builds this turn. Two paths:

**Path A — let it Build, prevent the Ravage next turn.**
- Build turn: land becomes 2 Towns + 1 Explorer (Build rule: if Towns ≥ Cities, add Town — at 1 Town, 0 Cities, add Town).
- Next turn's Ravage: 5 damage (1 per Explorer + 2 per Town × 2) → 1 Blight + Dahan/presence deaths unless defended.
- You spent a Defend power next turn. The Town is still alive to Build next time it rolls in.

**Path B — prevent the Build this turn (push the Explorer out, or kill/displace the Town before Build resolves).**
- Build turn: land has 1 Explorer → Build happens on just the Explorer, adding 1 Town. Now 1 Town + 1 Explorer.
- Next turn's Ravage: 3 damage — cheaper to Defend, or survivable without it.
- You spent the prevention power this turn but saved a Defend next turn, and the downstream Ravage is permanently smaller.

```admonish tip title="The heuristic"
If a land is about to Build and you can afford exactly one intervention: **intervene on the Build**, not the Ravage. Builds compound; Ravages are discrete.
```

## When prevention breaks — Build-immune threats

Some lands can't be prevented via Build-disruption:

- **Cities**: Building in a land with ≥1 City adds another Town (Town goes to the empty slot). You can't "prevent" the Build unless you can Push/Destroy the City itself or turn the land into a no-Invader land (Push *every* piece out).
- **Adversary Escalation builds**: several Adversary abilities add pieces *outside* the Build action (England's Stage III adds Cities to coasts; Sweden's extra Towns on Build). Those fire regardless of Build-prevention on the current card.
- **Event-card builds** (B&C+): Events can add pieces mid-phase. Read the Event before you plan your turn.

For those threats, prevention shifts to: **clear the Invader entirely**, or accept the Ravage and Defend hard.

## Downgrade as a damage primitive (Nature Incarnate)

NI introduced **Downgrade** as an alternative to Damage and Destroy. A Downgrade replaces:

- A City with a Town (City → Town, Town returns to the box).
- A Town with an Explorer.
- An Explorer is Destroyed (Downgrade on an Explorer behaves as Destroy for Fear purposes).

This matters for Build-prevention math. A land with 1 City + 0 Towns Builds a Town. Downgrading the City on the Fast phase before Build turns the land into 1 Town + 0 Cities — which still Builds a Town (Towns ≥ Cities, so add Town). But the land now Ravages for 2 instead of 3. Two consecutive Downgrades (City → Town → Explorer) are often cheaper than one Destroy and yield the same "next Ravage is smaller" payoff.

```admonish note title="Downgrade vs. Destroy for Fear"
Downgrading a City generates Fear as if destroying the City-that-was (check the specific power text). Downgrading a Town generates Fear as if destroying the Town. The Explorer result of a Town-Downgrade is a new piece, not the destroyed one; it doesn't need to be Destroyed to have generated Fear.
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

The Invader deck's shape determines how much prevention you can afford. Count per-Stage:

- **Stage 1 (turns 1–2 typically)**: one Explore, one Build, one Ravage per turn. Pressure is linear. Your prevention budget is highest here.
- **Stage 2 (turns 3–5)**: the deck pools into pairs. Double-terrain cards mean one prevention power doesn't fully cover the Build/Ravage slot. Pressure is super-linear — same 3 actions per turn, but hitting 2 terrains.
- **Stage 3 (turns 6+)**: cards that target terrain-pairs AND specify coastal/inland, or three-terrain-at-a-time effects on Adversary flags. Pressure peaks; Fear-win or board-clear must be imminent.

The pressure budget is why spirits with efficient board-wide Innates (Shadows's Darkness Swallows, Keeper's Innates, Fractured Days) over-perform at high difficulty: their prevention scales with the deck's terrain-spread.

## The single-Explorer rule

The most common tactical application:

> If a land about to Build contains only one Explorer, killing/pushing that Explorer cancels the entire Build for that land.

One card play for one piece of intervention, no Invader piece added. Combined with Build-prevention compounding, single-Explorer lands are the cheapest high-value targets on the board. Experienced players pattern-match these lands before evaluating anything else.

## Common mistakes

```admonish failure title="Three patterns"
1. **Reacting to Ravages because they're loud.** The Ravage this turn is already happening; even if you Defend it to zero, the land Ravages again two turns later (unless the Invader is removed). Attacking the Build is the only durable move.
2. **Single-Explorer lands left alive.** Every unattended single-Explorer land is an implicit 1-Town-per-turn generator. By T6 your board has 4–6 Towns you "didn't have" on T3.
3. **Overcounting Cities as Build-preventable.** A City in the land means Build adds a Town regardless of what you do to the Explorer/other pieces. Either clear the City itself or accept the downstream Ravage.
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
- Downgrade mechanics: [Spirit Island Wiki — Downgrade](https://spiritislandwiki.com/index.php?title=Downgrade).
- Invader deck Stages: rulebook + [Sequence of Play](https://spiritislandwiki.com/index.php?title=Sequence_of_Play) §3.
