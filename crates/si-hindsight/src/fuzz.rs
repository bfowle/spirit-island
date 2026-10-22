//! Random legal lines on every closed turn of a game, each scored as a typed line is, with
//! the game logic's invariants checked on every one:
//!
//! - nothing panics;
//! - the same line scores the same twice;
//! - every score is finite;
//! - a line the generator built from the enumerator's own legality is not refused;
//! - the line written out (`plays_text_kept`) and read back plays the same;
//! - a gather or push moves pieces only between adjacent lands;
//! - presence never comes from nowhere, and none goes missing uncounted;
//! - a line never both wins and loses;
//!
//! and on the actual line: it is not refused, and a game that went on past a turn did not
//! end in it. Seeded: a finding names its seed and replays.
//!
//! `fuzz_open` does the same for the turn being played, as the live screen scores plans at a
//! save (`live::score_plan`): doing nothing is never refused, and the invariants above hold on
//! every random plan.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::power::{PowerCard, Speed};
use si_model::island::LandId;
use si_record::{Counts, GameRecord, SpiritTurn, TurnRecord};
use si_rules::invaders::Island;
use si_rules::verify;

use si_model::game::Phase;
use si_model::oracle::Rng;

use crate::live::{score_plan, OpenTurn};
use crate::repeat;
use crate::{actual_position, closed_turns, growth_positions, spirit_at, dahan_moves, elements_for, in_use_order, innate_targets_reach, innates_of, may_be_fast, parse_plays, play_options, push_lands, recorded_plays, try_plays, Line, Move, Play, Position, Rules};

#[derive(Debug, Clone)]
pub struct FuzzOptions {
    /// Random lines per closed turn.
    pub lines: usize,
    pub seed: u64,
}

impl Default for FuzzOptions {
    fn default() -> Self {
        FuzzOptions { lines: 30, seed: 1 }
    }
}

/// One broken invariant: the turn, which check, what was seen, the line in notation, and the
/// seed that builds it again.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub turn: u32,
    /// Where an open-turn finding was cut (`T3 Fast, save-Feedback-...`); empty for a closed turn.
    #[serde(default)]
    pub at: String,
    pub check: String,
    pub detail: String,
    pub line: String,
    pub seed: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FuzzReport {
    pub turns: usize,
    pub lines: usize,
    pub findings: Vec<Finding>,
    /// The closed turns whose actual line, written in notation and played from the text,
    /// comes to the board the record has as the Slow phase ends, land for land.
    #[serde(default)]
    pub exact: Vec<u32>,
    /// The other closed turns: where the text's board and the record's differ
    /// (`A3 E 1/0 Dh 2/3`: the simulator's count, then the record's).
    #[serde(default)]
    pub inexact: Vec<(u32, String)>,
}

/// A random line the enumerator could have built: cards from the hand within the plays and
/// Energy at legal targets (a Slow one left for the Invader steps half the time), each
/// innate reached unused, Slow, or Fast where it may be, random gathers and pushes asked of
/// the Powers that move Dahan, and the whole in a random order.
#[allow(clippy::too_many_arguments)]
fn random_line(rng: &mut Rng, defs: &Definitions, pos: &Position, s2: &SpiritTurn, t: &TurnRecord, graph: &si_model::island::Island, island: &Island, innates: &[&si_model::spirit::Innate], presence_moves: bool) -> Vec<Play> {
    let mut hand: Vec<&PowerCard> = pos.hand.iter().filter_map(|c| defs.cards.get(c)).collect();
    rng.shuffle(&mut hand);
    let k = rng.below(pos.plays.max(0) as usize + 1);
    let mut energy = pos.energy;
    let mut plays: Vec<Play> = Vec::new();
    for c in hand {
        if plays.len() >= k {
            break;
        }
        if c.cost > energy {
            continue;
        }
        let options = play_options(c, &pos.presence, &t.before_fast, graph);
        if options.is_empty() {
            continue;
        }
        let mut p = options[rng.below(options.len())].clone();
        if !p.fast && !p.places && rng.coin() {
            p.target = None;
        }
        energy -= c.cost;
        plays.push(p);
    }
    let played: Vec<&PowerCard> = plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = elements_for(defs, s2, &played);
    for inn in innates {
        if !inn.tiers.iter().any(|tier| !tier.modifier && elements.satisfies(&tier.elements)) {
            continue;
        }
        let printed_fast = inn.speed == Speed::Fast;
        let can_fast = printed_fast || may_be_fast(inn, &elements);
        match rng.below(3) {
            0 => continue,
            1 if !printed_fast => plays.push(Play { power: inn.id.clone(), target: None, fast: false, innate: true, cost: 0, places: false, forced: false, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None }),
            _ if can_fast => {
                let targets = innate_targets_reach(inn, s2, &t.before_fast, graph, presence_moves);
                if targets.is_empty() {
                    continue;
                }
                let tg = targets[rng.below(targets.len())].clone();
                plays.push(Play { power: inn.id.clone(), target: Some(tg), fast: true, innate: true, cost: 0, places: false, forced: !printed_fast, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None });
            }
            _ => plays.push(Play { power: inn.id.clone(), target: None, fast: false, innate: true, cost: 0, places: false, forced: false, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None }),
        }
    }
    for p in plays.iter_mut() {
        let Some(tg) = p.target.clone() else { continue };
        let (gathers, pushes) = dahan_moves(defs, &p.power);
        if !(gathers || pushes) || rng.below(3) == 0 {
            continue;
        }
        let around = push_lands(island, &tg);
        if around.is_empty() {
            continue;
        }
        if gathers && rng.coin() {
            p.moves.push(Move { from: around[rng.below(around.len())].clone(), to: tg.clone(), piece: None, n: 1 + rng.below(3) as u32 });
        }
        if pushes && rng.coin() {
            p.moves.push(Move { from: tg.clone(), to: around[rng.below(around.len())].clone(), piece: None, n: 1 + rng.below(3) as u32 });
        }
        p.exact = !p.moves.is_empty() && rng.coin();
    }
    rng.shuffle(&mut plays);
    // each Power's Repeats after it, a Fast one at a random land it may take (`repeat`)
    let usual = |p: &Play| -> Vec<LandId> {
        match defs.cards.get(&p.power) {
            Some(c) => play_options(c, &pos.presence, &t.before_fast, graph).into_iter().filter_map(|o| o.target).collect(),
            None => innates.iter().find(|i| i.id == p.power).map(|i| innate_targets_reach(i, s2, &t.before_fast, graph, presence_moves)).unwrap_or_default(),
        }
    };
    let mut lines = repeat::expand(defs, &plays, &elements, &usual, graph, &pos.presence, 8);
    lines.swap_remove(rng.below(lines.len()))
}

/// What a line produced, for comparing two runs that should agree.
fn produced(l: &Line) -> String {
    let o = &l.outcome;
    format!(
        "Bl{} Dh{} Pr{} Pr+{} E{} T{} C{} Fr{} bT{} bC{} loses {:?} wins {:?} next {:?} {:?}",
        o.blight, o.dahan_lost, o.presence_destroyed, o.presence_added, o.explorers_destroyed, o.towns_destroyed, o.cities_destroyed, o.fear, o.built_towns, o.built_cities, o.loses, o.wins, o.next_loses, o.after
    )
}

fn panic_text(e: Box<dyn std::any::Any + Send>) -> String {
    e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_else(|| "panic".into())
}

/// Every closed turn of the game: the actual line, then `opts.lines` random lines.
pub fn fuzz_game(rec: &GameRecord, defs: &Definitions, opts: &FuzzOptions) -> FuzzReport {
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let defs: &Definitions = &scenario_defs;
    let mut rep = FuzzReport::default();
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let graph = si_metrics::reach::island_for(rec, defs);
    let presence_moves = Rules { hooks: Rules::hooks_of(defs, &rec.meta.spirits), ..Default::default() }.presence_with_dahan_for(&spirit);
    let innates = innates_of(defs, &spirit);
    let code = |id: &str| id.to_string();
    for turn in closed_turns(rec) {
        let Some(t) = rec.turns.iter().find(|x| x.turn == turn) else { continue };
        let Some(s) = rec.spirits.iter().find(|x| x.turn == turn && x.spirit == spirit) else { continue };
        if t.before_fast.is_empty() {
            continue;
        }
        rep.turns += 1;
        let mut find = |check: &str, detail: String, line: String, seed: u64| rep.findings.push(Finding { turn, at: String::new(), check: check.into(), detail, line, seed });
        // the actual line: playable, and no ending the record did not have
        let (cards, inns) = recorded_plays(rec, defs, s, t, &innates);
        // what the record shows was played, at the speed it was used (`recorded`: not held to
        // a typed plan's speed check; its notation is, and must pass it)
        let actual: Vec<Play> = in_use_order(cards.into_iter().chain(inns).collect(), s);
        let went_on = rec.turns.iter().any(|u| u.turn == turn + 1 && !u.before_fast.is_empty());
        let actual_run = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| try_plays(rec, defs, turn, actual.clone()))) {
            Err(e) => {
                find("panic", panic_text(e), "the actual line".into(), 0);
                None
            }
            Ok(Err(e)) => {
                find("actual refused", e, "the actual line".into(), 0);
                None
            }
            Ok(Ok(l)) => {
                if went_on {
                    if let Some(r) = &l.outcome.loses {
                        find("ended early", format!("the actual line loses ({r}), but the game went on"), l.plays_notation(&code, true, true), 0);
                    }
                    if let Some(r) = &l.outcome.wins {
                        find("ended early", format!("the actual line wins ({r}), but the game went on"), l.plays_notation(&code, true, true), 0);
                    }
                }
                Some(l)
            }
        };
        // the actual line in notation: written, read back and played, the same as the
        // record's plays, and the board it comes to against the record's as the Slow phase ends
        if let Some(l) = &actual_run {
            let text = l.plays_text_kept(&code);
            match parse_plays(defs, &text) {
                Err(e) => find("actual notation", format!("`{text}` does not read back: {e}"), text.clone(), 0),
                Ok(back) => {
                    if let Some(again) = written(&back, &text) {
                        find("notation text", again, text.clone(), 0);
                    }
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| try_plays(rec, defs, turn, back))) {
                        Ok(Ok(l2)) => {
                            if produced(&l2) != produced(l) {
                                find("actual notation", format!("`{text}` plays otherwise than the record's plays: {} vs {}", produced(l), produced(&l2)), text.clone(), 0);
                            }
                            if !t.after_slow.is_empty() && !l2.outcome.after.is_empty() {
                                match board_diff(&l2.outcome.after, &t.after_slow) {
                                    None => rep.exact.push(turn),
                                    Some(d) => rep.inexact.push((turn, d)),
                                }
                            }
                        }
                        Ok(Err(e)) => find("actual notation", format!("`{text}` read back is refused: {e}"), text.clone(), 0),
                        Err(e) => find("panic", format!("reading back `{text}`: {}", panic_text(e)), text.clone(), 0),
                    }
                }
            }
        }
        let pos = actual_position(rec, defs, s, t);
        let s2 = spirit_at(s, &pos);
        let island = verify::island_at(rec, defs, t, "before_fast");
        // every spirit's presence on the island at the Fast phase (a two-spirit game has both)
        let presence0: u32 = t.before_fast.values().map(|c| c.presence).sum();
        for i in 0..opts.lines {
            let seed = opts.seed.wrapping_mul(1_000_003).wrapping_add((turn as u64) << 20).wrapping_add(i as u64);
            let mut rng = Rng::new(seed);
            let plays = random_line(&mut rng, defs, &pos, &s2, t, &graph, &island, &innates, presence_moves);
            let shown = Line { plays: plays.clone(), ..empty_line() }.plays_notation(&code, true, true);
            rep.lines += 1;
            let run = |p: Vec<Play>| std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| try_plays(rec, defs, turn, p)));
            let line = match run(plays.clone()) {
                Err(e) => {
                    find("panic", panic_text(e), shown, seed);
                    continue;
                }
                Ok(Err(e)) => {
                    find("refused", format!("built from the enumerator's own legality, refused: {e}"), shown, seed);
                    continue;
                }
                Ok(Ok(l)) => l,
            };
            // the same line twice
            if let Ok(Ok(again)) = run(plays.clone()) {
                if produced(&again) != produced(&line) || (again.score - line.score).abs() > 1e-9 {
                    find("not deterministic", format!("{} vs {}", produced(&line), produced(&again)), shown.clone(), seed);
                }
            }
            for v in line.holds(defs, &island, presence0) {
                find(v.law, v.detail, shown.clone(), seed);
            }
            let played: Vec<&PowerCard> = plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
            for v in repeat::holds(defs, &plays, &elements_for(defs, &s2, &played), &graph) {
                find(v.law, v.detail, shown.clone(), seed);
            }
            // written out and read back, the line plays the same, and writes the same again
            let text = line.plays_text_kept(&code);
            match parse_plays(defs, &text) {
                Err(e) => find("notation", format!("`{text}` does not read back: {e}"), shown.clone(), seed),
                Ok(back) => match {
                    if let Some(again) = written(&back, &text) {
                        find("notation text", again, shown.clone(), seed);
                    }
                    run(back)
                } {
                    Ok(Ok(l2)) if produced(&l2) != produced(&line) => find("notation", format!("`{text}` read back plays otherwise: {} vs {}", produced(&line), produced(&l2)), shown.clone(), seed),
                    Ok(Err(e)) => find("notation", format!("`{text}` read back is refused: {e}"), shown.clone(), seed),
                    Err(e) => find("panic", format!("reading back `{text}`: {}", panic_text(e)), shown.clone(), seed),
                    _ => {}
                },
            }
        }
    }
    rep
}

/// The turn being played, as the live screen scores it at one save: doing nothing, then
/// random plans the screen could be given (a growth position the panel offers on the Growth
/// tab, then plays from the hand the journal has not made yet, Slow ones only once the Fast
/// phase is over), each through `score_plan`. Doing nothing is never refused; every plan
/// holds the closed turns' invariants, scores the same twice, and scores the same typed back
/// in as the screen writes it.
pub fn fuzz_open(open: &OpenTurn, defs: &Definitions, opts: &FuzzOptions, at: &str) -> FuzzReport {
    let scenario_defs = defs.for_scenario(open.rec.meta.scenario.as_deref().or(open.seed.scenario.as_deref()));
    let sdefs: &Definitions = &scenario_defs;
    let rec: &GameRecord = &open.rec;
    let t = &open.t;
    let turn = open.turn;
    let mut rep = FuzzReport { turns: 1, ..Default::default() };
    let code = |id: &str| id.to_string();
    let finding = |check: &str, detail: String, line: String, seed: u64| Finding { turn, at: at.to_string(), check: check.into(), detail, line, seed };
    let score = |text: &str, base: Option<&Line>| std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| score_plan(defs, open, base, text)));
    let none = match score("", None) {
        Err(e) => {
            rep.findings.push(finding("panic", panic_text(e), "doing nothing".into(), 0));
            return rep;
        }
        Ok(Err(e)) => {
            rep.findings.push(finding("refused", format!("doing nothing is refused: {e}"), "doing nothing".into(), 0));
            return rep;
        }
        Ok(Ok(l)) => l,
    };
    let at_slow = open.phase == Phase::SlowPower;
    let past_fast = !matches!(open.phase, Phase::SpiritPhase | Phase::FastPower);
    let island = verify::island_for(rec, sdefs, &t.before_fast);
    let graph = si_metrics::reach::island_for(rec, sdefs);
    let innates = innates_of(sdefs, &open.spirit);
    let presence_moves = open.rules.presence_with_dahan_for(&open.spirit);
    let actual = actual_position(rec, sdefs, &open.s, t);
    let positions: Vec<Position> = if open.phase == Phase::SpiritPhase { growth_positions(rec, sdefs, &open.s, t, &graph) } else { Vec::new() };
    let (made_cards, made_innates) = recorded_plays(rec, sdefs, &open.s, t, &innates);
    // the presence the plan starts from: the position's and the other Spirits' as recorded
    let others: u32 = crate::others_presence(rec, t.turn, &open.spirit).values().sum();
    let start = |pos: &Position| -> u32 { if at_slow { t.before_slow.values().map(|c| c.presence).sum() } else { pos.presence.values().sum::<u32>() + others } };
    for v in none.holds(defs, &island, start(&actual)) {
        rep.findings.push(finding(v.law, v.detail, "doing nothing".into(), 0));
    }
    for i in 0..opts.lines {
        let seed = opts.seed.wrapping_mul(1_000_003).wrapping_add((turn as u64) << 20).wrapping_add(i as u64);
        let mut rng = Rng::new(seed);
        let (pos, growth) = match positions.len() {
            0 => (actual.clone(), String::new()),
            n => {
                let p = positions[rng.below(n)].clone();
                let g = format!("{} | ", p.growth.join(" "));
                (p, g)
            }
        };
        // the cards the journal has made this turn are not the plan's to play again
        let mut rest = pos.clone();
        rest.hand.retain(|c| !made_cards.iter().any(|m| m.power == *c));
        rest.plays -= made_cards.len() as i64;
        rest.energy -= made_cards.iter().map(|m| m.cost).sum::<i64>();
        let s2 = spirit_at(&open.s, &pos);
        let mut plays = random_line(&mut rng, sdefs, &rest, &s2, t, &graph, &island, &innates, presence_moves);
        plays.retain(|p| !made_innates.iter().any(|m| m.power == p.power) && !(past_fast && p.fast));
        let text = format!("{growth}{}", Line { plays, ..empty_line() }.plays_text_kept(&code));
        rep.lines += 1;
        let line = match score(&text, Some(&none)) {
            Err(e) => {
                rep.findings.push(finding("panic", panic_text(e), text, seed));
                continue;
            }
            Ok(Err(e)) => {
                rep.findings.push(finding("refused", format!("a plan built from the enumerator's own legality, refused: {e}"), text, seed));
                continue;
            }
            Ok(Ok(l)) => l,
        };
        if let Ok(Ok(again)) = score(&text, Some(&none)) {
            if produced(&again) != produced(&line) || (again.score - line.score).abs() > 1e-9 {
                rep.findings.push(finding("not deterministic", format!("{} vs {}", produced(&line), produced(&again)), text.clone(), seed));
            }
        }
        for v in line.holds(defs, &island, start(&pos)) {
            rep.findings.push(finding(v.law, v.detail, text.clone(), seed));
        }
        // the plan as the screen writes it, typed back in
        let back = format!("{growth}{}", line.plays_text_kept(&code));
        match score(&back, Some(&none)) {
            Ok(Ok(l2)) if produced(&l2) != produced(&line) => rep.findings.push(finding("notation", format!("`{back}` read back plays otherwise: {} vs {}", produced(&line), produced(&l2)), text.clone(), seed)),
            Ok(Err(e)) => rep.findings.push(finding("notation", format!("`{back}` read back is refused: {e}"), text.clone(), seed)),
            Err(e) => rep.findings.push(finding("panic", format!("reading back `{back}`: {}", panic_text(e)), text.clone(), seed)),
            _ => {}
        }
    }
    rep
}

/// The plays read from `text`, written again: None when they write the same text, else what
/// they wrote.
fn written(plays: &[Play], text: &str) -> Option<String> {
    let code = |id: &str| id.to_string();
    let again = Line { plays: plays.to_vec(), ..empty_line() }.plays_text_kept(&code);
    (again != text).then(|| format!("`{text}` read back writes `{again}`"))
}

/// Where two boards differ, land by land (`A3 E 1/0 Dh 2/3`: the first board's count, then
/// the second's); a land missing from one has nothing on it. None when they agree.
fn board_diff(a: &BTreeMap<LandId, Counts>, b: &BTreeMap<LandId, Counts>) -> Option<String> {
    let zero = Counts::default();
    let mut out: Vec<String> = Vec::new();
    for id in a.keys().chain(b.keys()).collect::<std::collections::BTreeSet<_>>() {
        let (x, y) = (a.get(id).unwrap_or(&zero), b.get(id).unwrap_or(&zero));
        let fields = [("E", x.explorers, y.explorers), ("T", x.towns, y.towns), ("C", x.cities, y.cities), ("Dh", x.dahan, y.dahan), ("Bl", x.blight, y.blight), ("Pr", x.presence, y.presence), ("Be", x.beasts, y.beasts), ("Di", x.disease, y.disease), ("St", x.strife, y.strife), ("Wi", x.wilds, y.wilds), ("Ba", x.badlands, y.badlands)];
        let d: Vec<String> = fields.iter().filter(|(_, p, q)| p != q).map(|(k, p, q)| format!("{k} {p}/{q}")).collect();
        if !d.is_empty() {
            out.push(format!("{id} {}", d.join(" ")));
        }
    }
    (!out.is_empty()).then(|| out.join(", "))
}

fn empty_line() -> Line {
    Line { growth: vec![], growth_actual: true, growth_value: 0.0, ties: 0, informed: 0.0, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays: vec![], outcome: Default::default(), score: 0.0, expected: 0.0, per_card: vec![], actual: false }
}

/// One Power use the record has, played alone: from the board as it began, the play as the
/// record has it (its land, moves, branch, placements, the turn's elements), against the
/// board as it ended. `diff` None: the rules did what the game did.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseCheck {
    pub turn: u32,
    pub power: String,
    pub target: Option<LandId>,
    /// Where the simulated board and the game's differ after it (`A3 E 1/0`: simulated/game).
    pub diff: Option<String>,
    /// Every effect of the Power is one the simulator executes.
    pub simulated: bool,
}

/// Every Power use the record has, each played alone against the game's own result.
pub fn power_uses(rec: &GameRecord, defs: &Definitions) -> Vec<UseCheck> {
    each_use(rec, defs, None).into_iter().map(|(c, _)| c).collect()
}

/// One recorded use played alone and explained: what the record has it doing (its land,
/// moves, kills, Damage already on pieces), the effects the simulator ran, every land that
/// changed (simulated, then the game), the Fear and Defend, the notes. `si fuzz --use`.
pub fn explain_use(rec: &GameRecord, defs: &Definitions, turn: u32, power: &str) -> Vec<String> {
    each_use(rec, defs, Some((turn, power))).into_iter().map(|(_, d)| d).collect()
}

fn each_use(rec: &GameRecord, defs: &Definitions, only: Option<(u32, &str)>) -> Vec<(UseCheck, String)> {
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let defs: &Definitions = &scenario_defs;
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let innates = innates_of(defs, &spirit);
    let mut out = Vec::new();
    for t in rec.turns.iter().filter(|t| t.turn > 0 && !t.power_uses.is_empty() && only.is_none_or(|(n, _)| n == t.turn)) {
        let Some(s) = rec.spirits.iter().find(|x| x.turn == t.turn && x.spirit == spirit) else { continue };
        let rules = crate::turn_rules(rec, defs, t);
        let (cards, inns) = recorded_plays(rec, defs, s, t, &innates);
        let all: Vec<Play> = cards.iter().chain(inns.iter()).cloned().collect();
        let played: Vec<&PowerCard> = cards.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
        let elements = elements_for(defs, s, &played);
        let mut nth: BTreeMap<String, usize> = BTreeMap::new();
        // a use the record saw end (a log that never marked the end has no board after it)
        for u in t.power_uses.iter().filter(|u| !u.after.is_empty()) {
            let k = nth.entry(u.power.clone()).or_insert(0);
            let i = *k;
            *k += 1;
            if only.is_some_and(|(_, p)| p != u.power) {
                continue;
            }
            // another spirit's Power in a game of two: not this spirit's to check
            let Some(base) = all.iter().filter(|p| p.power == u.power).nth(i).or_else(|| all.iter().find(|p| p.power == u.power)) else { continue };
            // this use's own account: its land, what it moved and what it destroyed
            let moves = if u.moves.is_empty() { base.moves.clone() } else { verify::moves_of_use(u) };
            let play = Play { target: u.target.clone().or(base.target.clone()), fast: u.phase == Some(Phase::FastPower), moves, recorded: true, kills: Some(verify::kills_of(u)), ..base.clone() };
            let mut one = t.clone();
            one.power_uses = vec![u.clone()];
            let start = verify::with_wounds(verify::island_for(rec, defs, &u.before), Some(&u.wounds));
            let mut island = start.clone();
            let mut o = crate::Outcome::default();
            let mut armed = Vec::new();
            crate::apply_plays(&mut island, defs, std::slice::from_ref(&play), play.fast, &elements, &rules, &spirit, &one, &mut armed, &mut o);
            let after: BTreeMap<LandId, Counts> = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
            let simulated = defs.cards.get(&u.power).map(|c| c.not_simulated().is_empty()).unwrap_or(true);
            // what the board does not show: the Fear it generated, the Defend it granted
            let defend: u32 = island.lands.values().map(|l| l.defend).sum();
            let mut diff: Vec<String> = board_diff(&after, &u.after).into_iter().collect();
            if o.fear != u.fear {
                diff.push(format!("Fear {}/{}", o.fear, u.fear));
            }
            // a Defend some Powers grant as a lasting rule leaves no Defend entry: compared
            // only where the game logged one
            if u.defend > 0 && defend != u.defend {
                diff.push(format!("Defend {defend}/{}", u.defend));
            }
            let check = UseCheck { turn: t.turn, power: u.power.clone(), target: play.target.clone(), diff: (!diff.is_empty()).then(|| diff.join(", ")), simulated };
            let details = if only.is_some() {
                let (base_fx, th) = verify::power_effects(defs, &u.power).unwrap_or_default();
                let land = play.target.as_ref().and_then(|l| start.lands.get(l));
                let mut d = vec![
                    format!("T{} {} on {} ({:?}), elements {elements}", t.turn, u.power, play.target.as_ref().map(|l| l.to_string()).unwrap_or("-".into()), u.phase),
                    format!("the record's moves {:?}, kills {:?}, wounds {:?}", u.moves, u.destroyed, u.wounds),
                    format!("effects run {:?}", si_rules::effects::reached_effects(&base_fx, &th, &elements)),
                ];
                let _ = land;
                for (id, c) in &after {
                    let was = u.before.get(id).copied().unwrap_or_default();
                    let game = u.after.get(id).copied().unwrap_or_default();
                    if *c != was || game != was {
                        d.push(format!("{id}: before {was:?}\n      sim  {c:?}\n      game {game:?}"));
                    }
                }
                d.push(format!("Fear sim {} game {}; Defend sim {defend} game {}", o.fear, u.fear, u.defend));
                d.push(format!("notes {:?}", o.notes));
                d.push(match &check.diff {
                    None => "the same as the game".into(),
                    Some(x) => format!("differs: {x}"),
                });
                d.join("\n")
            } else {
                String::new()
            };
            out.push((check, details));
        }
    }
    out
}

/// The findings grouped by check, for a one-line summary.
pub fn by_check(r: &FuzzReport) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for f in &r.findings {
        *m.entry(f.check.clone()).or_insert(0) += 1;
    }
    m
}
