# Deliberate Play

The single biggest difference between intermediate and advanced Spirit Island is **not** knowing more cards or playing a stronger spirit. It's **stopping between actions** to ask what the board actually looks like and what options it actually affords — before reaching for the play your opener rehearsed or the card that "feels right".

This is the habit this chapter is built to train.

## The auto-pilot trap

Auto-pilot is what happens when you execute without evaluating:

- You play Concealing Shadows T1 *because that's what the opener said*, without asking whether Concealing's headline effect (Ravage-protection) is even material this turn.
- You pick Growth G2 *because you usually pick G2*, without tallying what income each track would give you across the next three turns.
- You target Mantle of Dread on land #6 *because that's where your presence is*, without noticing that the Build just happened in land #3 and your push matters more there.
- You let Darkness Swallows fire whatever level triggers *because it's free*, without asking whether Level 1's Gather or Level 2's Destroy better fits the situation.

Every one of those decisions *might* be correct. But when you make them without a deliberate evaluation, you give up the ability to notice when they're wrong — which is the exact moment the game turns on you.

**Auto-pilot is the default intermediate failure mode.** You escape it by installing a few short, fixed pauses into every turn.

## The per-phase checklist

Laminate this. Or keep a notecard. Or paste it inside the box. Every turn, in every phase below, ask these questions *out loud* before acting.

### Growth phase

1. **What is my hand / discard / elements going into this turn?** Count cards in each pile; count total Moon / Fire / Air / Sun / Water / Earth / Plant / Animal available.
2. **What's my Energy income next round for each growth choice × track choice?** Compute all branches before picking. (Example: Shadows G3-via-CP = 3E this turn / 2CP; G3-via-Energy = 4E / 1CP.)
3. **What's the highest-leverage presence placement on the current board?** The land with the most-imminent Build? A land that gives me range into the next Ravage target? A sacred-site candidate?
4. **Which option best matches the 3-turn plan I set at game start?** If none do, am I pivoting — and what's my new plan?

### Energy phase (after growth resolves)

1. **How much Energy and how many CP do I have right now?**
2. **Which card combinations can I afford?** Enumerate every legal subset of my hand by (sum of costs ≤ Energy) × (count ≤ CP). Don't skip to the "obvious" one.
3. **Which cards do I MUST play this turn vs. bank for later?** A card that threshold-enables my innate THIS turn is must-play. A card that's just "strong" is bank-able.

### Fast Power phase

1. **In what order do I resolve Fast cards + Fast innates?** Order matters — e.g., a Gather that moves Dahan into a land enables a conditional Fear on a later Slow play.
2. **What elements are in the pool as each Fast innate threshold is checked?** Fast innates see only Fast-played elements. Write it out if you're not sure.
3. **Are there Dahan movements / damage / explorations triggered by my Fast effects?** Resolve the board consequences fully, one effect at a time.

### Invader phase

1. **Explore: where do new Explorers arrive?** (Any land adjacent to a City, Town, or ocean for coastal — per the Invader card + adversary modifiers.)
2. **Build: which lands have Invaders that trigger a Build?** What does the *current* Build card say about which terrain Builds?
3. **Ravage: which lands Ravage?** Compute Damage → Dahan fight-back → Blight. Don't skip steps; mistakes here cascade into T+1 planning errors.

### Slow Power phase

1. **In what order do I resolve Slow cards + Slow innates?** Again, order matters — a Push that removes an Invader before a Fear-conditional evaluates can make/break the conditional.
2. **Now the full turn's element pool is visible to Slow innates.** Re-check thresholds.
3. **Did any Event / Blight / other side-effect happen this turn that I need to factor into the end state?**

### Time Passes

1. **What's my state entering next turn?** Hand / discard / forgotten; Energy bank; Presence on board + tracks; Fear pool; Blight pool; Terror level.
2. **Does the plan I set at game start still apply, or have board conditions changed enough that I'm pivoting?** Name the pivot trigger explicitly.

## Cross-turn prompts

Every 2–3 rounds, step back and re-evaluate at a higher altitude:

- **3-turn lookahead**: what do I want the board to look like at the end of T+3? Which plays between now and then move it there?
- **Pivot triggers**: list 2–3 board conditions under which I abandon the current plan. Examples: "if Fear pool is < 4 by T3, shift to Favors-triggering plays"; "if a City appears in a coast I can't hit, draft a Push major".
- **Engagement audit**: am I actually making decisions, or am I executing a script? If the last three turns felt rote, something's probably being left on the table.

## Common auto-pilot failures + their remediation

| Auto-pilot failure                                                     | Remediation                                                                                          |
|------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| Playing Concealing / Defend / Ravage-protection cards on no-Ravage turns | Check the invader deck's current T-column: no Ravage → these cards are 1 Fear + elements only. Don't narrate their protection effect as T1/T2 value. |
| Picking a growth option by habit                                       | Compute income for at least two growth × track branches before choosing.                              |
| Targeting a Power on the land with your presence rather than the highest-leverage land | Check range-extension options (spirit's special rule + Shadows-of-the-Dahan-style rules) before narrowing to R0 targets. |
| Claiming your innate "fires this turn" without checking phase          | Fast innates see only Fast elements. Slow innates see Fast + Slow. Elements don't persist across turns. Recount at each resolution. |
| Executing the opener verbatim T4+                                      | Openers rehearse T1–T3. T4+ is adaptive play by definition. Name the pivot trigger at T3's end.       |
| Treating Wiki-suggested cards as draft priority                        | Wiki-suggested = HoSI beginner-deck bundle. Full-pool draft ranks by element alignment + cost curve + synergy, not by who packaged which starter deck. |

## How this chapter integrates elsewhere

- **Per-spirit Possible Openings** — each turn block in every spirit chapter includes a **"Pause-point"** callout referencing one or two of the questions above, specialized to what most often trips players up on that spirit.
- **`si-at-the-table` skill** — responses surface the decision-tree (options + deciding factor), not just a conclusion. That's the whole point of the skill.
- **`si-live` tool** — per-phase UI prompts that show the state-relevant questions (e.g., during Growth: current track state, income branches; during Fast: element pool, innate-threshold proximity).
- **`si-rules-check` skill** — returns the phase-timing summary so writers can't accidentally bake auto-pilot assumptions into the chapter text.

## The minimum viable practice

If you only have energy for one pause per turn, make it **Growth**. Wrong growth choice at T1 is the single biggest correctable swing in a Spirit Island game: it compounds for eight rounds. Growth is where auto-pilot costs the most.

If you have energy for two pauses per turn, add the **pre-Fast-phase element tally** — before playing any Fast card, count the elements your planned Fast plays will produce in Fast phase. This catches the "my innate fires because I have the right total elements" error where the total includes Slow elements the Fast innate will never see.

Three pauses: add the **end-of-turn pivot check**. "Does my 3-turn plan still apply?" If yes, continue. If no, spend 30 seconds naming the new plan before Time Passes.

Build these three pauses into reflex and the rest of the framework follows.
