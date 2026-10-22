//! The turn being played: the journal's trailing turn with the face-up cards and the
//! board the newest save says, and the scoring of a plan from it. Spec:
//! docs/superpowers/specs/2026-10-21-live-study.md, sections 2 and 4.

use std::collections::BTreeSet;
use std::sync::Arc;

use si_defs::Definitions;
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::{GameRecord, SpiritTurn, TurnRecord};
use si_rules::invaders::Rules;

use crate::deck::Deck;
use crate::{Calibration, Values};

/// The open turn: everything the scorer reads, so `score_plan` takes it alone.
#[derive(Debug, Clone)]
pub struct OpenTurn {
    pub rec: Arc<GameRecord>,
    pub turn: u32,
    pub phase: Phase,
    pub spirit: String,
    /// The trailing turn with the slots filled and, at Growth, the board now as `before_fast`.
    pub t: TurnRecord,
    pub s: SpiritTurn,
    pub seed: Seed,
    pub deck: Deck,
    pub rules: Rules,
    pub cal: Calibration,
    pub values: Values,
}

/// The phases on which the turn is open: a tab exists and plans are scored.
pub fn is_open_phase(p: Phase) -> bool {
    matches!(p, Phase::SpiritPhase | Phase::FastPower | Phase::BlightedIsland | Phase::Event | Phase::Fear | Phase::HighImmigration | Phase::Ravage | Phase::Build | Phase::Explore | Phase::AdvanceInvaderCards | Phase::SlowPower)
}

/// The phase in the game's words for the tab label.
pub fn phase_word(p: Phase) -> &'static str {
    match p {
        Phase::SpiritPhase => "Growth",
        Phase::FastPower => "Fast",
        Phase::BlightedIsland => "Blight card",
        Phase::Event => "Event",
        Phase::Fear => "Fear",
        Phase::HighImmigration => "High Immigration",
        Phase::Ravage => "Ravage",
        Phase::Build => "Build",
        Phase::Explore => "Explore",
        Phase::AdvanceInvaderCards => "Cards advance",
        Phase::SlowPower => "Slow",
        Phase::TimePasses => "Time passes",
        other => other.name(),
    }
}

/// The journal's trailing turn as the open turn, or None when the game has ended, the
/// save is not at that turn, or the phase has no open tab.
pub fn open_turn(rec: Arc<GameRecord>, defs: &Definitions, seed: &Seed, cal: &Calibration) -> Option<OpenTurn> {
    if seed.ending.is_some() || rec.meta.ending.is_some() {
        return None;
    }
    let spirit = rec.meta.spirits.first().cloned()?;
    let t0 = rec.turns.iter().filter(|t| t.turn >= 1).last()?.clone();
    let turn = t0.turn;
    if seed.turn_index + 1 != turn || !is_open_phase(seed.phase) {
        return None;
    }
    let phase = seed.phase;
    let mut t = t0;
    // the face-up cards, which the journal records only as each step runs
    if t.explore.is_empty() {
        if t.ravage.is_empty() {
            t.ravage = seed.ravage_slot.clone();
        }
        if t.build.is_empty() {
            t.build = seed.build_slot.clone();
        }
        if t.high_immigration.is_empty() {
            t.high_immigration = seed.extra_slots.iter().flat_map(|(_, v)| v.iter().cloned()).collect();
        }
    }
    // the board: the journal snapshots it at the Fast transition; before that, the save's
    if t.before_fast.is_empty() {
        t.before_fast = seed.board();
    }
    let panel = defs.spirits.get(&spirit);
    let mut s = rec.spirits.iter().find(|x| x.turn == turn && x.spirit == spirit).cloned().unwrap_or_else(|| SpiritTurn { turn, spirit: spirit.clone(), ..Default::default() });
    if let Some(ss) = seed.spirits.iter().find(|x| x.id == spirit) {
        if phase == Phase::SpiritPhase {
            // nothing taken yet: the position is the save's
            s.hand_after = ss.hand.clone();
            s.discard_after = ss.discard.clone();
            s.energy_after = ss.energy;
            s.revealed_after = ss.revealed.iter().cloned().collect();
            s.presence_after = seed.presence_of(&spirit);
            if let Some(ts) = panel.map(|p| p.track_state(&s.revealed_after)) {
                s.income = Some(ts.income);
                s.plays = Some(ts.plays);
            }
        } else {
            // the journal and the save disagree: the save is the engine's own state
            let journal: BTreeSet<&String> = s.hand_after.iter().chain(s.cards_played.iter()).collect();
            let save: BTreeSet<&String> = ss.hand.iter().chain(ss.play.iter()).collect();
            if journal != save {
                s.hand_after = ss.hand.iter().chain(ss.play.iter()).filter(|c| !s.cards_played.contains(c)).cloned().collect();
            }
            if s.energy_after != ss.energy {
                s.energy_after = ss.energy;
            }
            let revealed: BTreeSet<String> = ss.revealed.iter().cloned().collect();
            if s.revealed_after != revealed {
                s.revealed_after = revealed;
            }
            if s.presence_after.is_empty() {
                s.presence_after = seed.presence_of(&spirit);
            }
        }
    }
    let level = seed.adversary_level.or(rec.meta.adversary_level).or_else(|| si_metrics::adversary::estimate(&rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    // the loss conditions' counts as the turn began, the Blight Card as the save holds it
    let clocks = crate::loss_clocks(&rec, turn, seed.blight_pool.map(|p| (p, seed.blight_flipped)));
    let rules = Rules { scenario: rec.meta.scenario.clone().or_else(|| seed.scenario.clone()), adversary: rec.meta.adversary.clone().or_else(|| seed.adversary.clone()), level, spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), clocks, terror: seed.terror.max(1), ..Default::default() };
    let deck = Deck::for_game(&rec, defs, level);
    let mut values = Values::for_panel(panel);
    values.next_weight = cal.next_weight;
    Some(OpenTurn { rec, turn, phase, spirit, t, s, seed: seed.clone(), deck, rules, cal: cal.clone(), values })
}

// ---- scoring a plan ----

use si_model::power::PowerCard;
use si_rules::verify;

use crate::{Line, Outcome, Play, Position};

/// A plan's growth tokens before `|`, matched to one of the positions the panel allows.
fn match_growth(defs: &Definitions, open: &OpenTurn, graph: &si_model::island::Island, text: &str) -> Result<Position, String> {
    let all = crate::growth_positions(&open.rec, defs, &open.s, &open.t, graph);
    let want: BTreeSet<String> = text
        .split_whitespace()
        .map(|tok| match tok.strip_prefix('+').filter(|r| !r.ends_with('E')) {
            // a gained card by code, title or id
            Some(name) => format!("+{}", crate::parse_plays(defs, name).ok().and_then(|v| v.first().map(|p| p.power.clone())).unwrap_or_else(|| name.to_string())),
            None => tok.to_string(),
        })
        .collect();
    // a position is its group and its placements; the card a draft gave, the Reclaim and the
    // Energy yield follow from the group, and a review's line names a card the live
    // position cannot know yet
    // the lands, not the slot codes: a review's line names the slot the journal reported,
    // the live position names the next uncovered node, and the two can differ by one
    let key = |tokens: &BTreeSet<String>| -> (Option<String>, Vec<String>) {
        let group = tokens.iter().find(|t| t.starts_with('G') && t[1..].chars().all(|c| c.is_ascii_digit() || c == '+')).cloned();
        let mut places: Vec<String> = tokens.iter().filter_map(|t| t.split_once('@').map(|(_, land)| land.to_string())).collect();
        places.sort();
        (group, places)
    };
    let want_key = key(&want);
    let mut candidates: Vec<&Position> = all.iter().filter(|p| key(&p.growth.iter().cloned().collect()) == want_key).collect();
    // among the positions on those lands, the one whose tracks match the typed slots (`cp` or
    // `e` before the number), then the one sharing the most tokens
    let track_of = |t: &str| -> Option<(String, String)> { t.split_once('@').map(|(slot, land)| (slot.trim_end_matches(|c: char| c.is_ascii_digit()).to_string(), land.to_string())) };
    let want_tracks: BTreeSet<(String, String)> = want.iter().filter_map(|t| track_of(t)).collect();
    candidates.sort_by_key(|p| {
        let tracks = p.growth.iter().filter_map(|t| track_of(t)).filter(|x| want_tracks.contains(x)).count();
        let same = p.growth.iter().filter(|t| want.contains(*t)).count();
        std::cmp::Reverse((tracks, same))
    });
    if let Some(p) = candidates.first() {
        return Ok((*p).clone());
    }
    let groups: Vec<String> = defs.spirits.get(&open.spirit).map(|p| p.growth.groups.iter().map(|g| format!("G{}", g.number)).collect()).unwrap_or_default();
    // the typed group's own positions first, so the fix is a copy away
    let same_group: Vec<String> = all.iter().filter(|p| key(&p.growth.iter().cloned().collect()).0 == want_key.0).take(14).map(|p| p.growth.join(" ")).collect();
    let examples: Vec<String> = if same_group.is_empty() { all.iter().take(4).map(|p| p.growth.join(" ")).collect() } else { same_group };
    Err(format!("no growth position matches \"{text}\"; the panel offers {} ({} positions; {}: {})", groups.join(" "), all.len(), want_key.0.as_deref().unwrap_or("for example"), examples.join(" / ")))
}

/// What the informed score adds for the face-down cards on an open turn: the Event from
/// turn 2 on, the Fear cards earned so far plus those the plan's own Fear reaches, at the
/// Terror Level the turn began at; zero on the Slow tab, where the board carries them.
fn expectation(open: &OpenTurn, plan_fear: u32) -> (bool, u32, u8) {
    let rec = &open.rec;
    let t = &open.t;
    let tl_start = rec.turns.iter().filter(|u| u.turn < t.turn).map(|u| u.terror_after).max().unwrap_or(1).clamp(1, 3);
    if open.phase == Phase::SlowPower {
        return (false, 0, tl_start);
    }
    // the Event's term stays until the scoring board carries its effect, which is the Slow tab
    let event = t.turn >= 2;
    let req = open.seed.fear_required.max(1);
    let crossed = ((open.seed.fear_generated + plan_fear as i64) / req - open.seed.fear_generated / req).max(0) as u32;
    let fear_cards = t.fear_cards_earned + crossed;
    (event, fear_cards, tl_start)
}

/// Score a plan from the open turn: `G2 e4@C4 | CS>C3 FCD>C4` on the Growth tab, plays
/// only after it. Informed: the mean over the Explore cards still in the stage, the
/// position's growth value, the expectation for the face-down cards. `confidence` is the
/// share of candidates under which the plan beats `baseline` (the `none` plan) with the
/// simulator's error. A play the journal already records is not applied twice.
pub fn score_plan(defs: &Definitions, open: &OpenTurn, baseline: Option<&Line>, text: &str) -> Result<Line, String> {
    let scenario_defs = defs.for_scenario(open.rec.meta.scenario.as_deref().or(open.seed.scenario.as_deref()));
    let defs: &Definitions = &scenario_defs;
    let rec: &GameRecord = &open.rec;
    let t = &open.t;
    let spirit = open.spirit.as_str();
    let values = &open.values;
    let rules = &open.rules;
    let graph = si_metrics::reach::island_for(rec, defs);
    let all_innates = crate::innates_of(defs, spirit);
    let (growth_text, plays_text) = match text.split_once('|') {
        Some((g, p)) => (g.trim(), p.trim()),
        None => ("", text.trim()),
    };
    let at_growth = open.phase == Phase::SpiritPhase;
    let at_slow = open.phase == Phase::SlowPower;
    let past_fast = !matches!(open.phase, Phase::SpiritPhase | Phase::FastPower);
    let pos: Position = if at_growth && !growth_text.is_empty() {
        match_growth(defs, open, &graph, growth_text)?
    } else {
        if !growth_text.is_empty() {
            return Err("the growth is taken: write the plays only".into());
        }
        crate::actual_position(rec, defs, &open.s, t)
    };
    let mut notes: Vec<String> = Vec::new();
    let typed = crate::parse_plays(defs, plays_text)?;
    let (made_cards, made_innates) = crate::recorded_plays(rec, defs, &open.s, t, &all_innates);
    // the plays already made this turn come first, in the order made; a typed play that
    // names one of them is that play's land and moves (a card played, not yet used), not a
    // second play of the same card; one made and used already keeps its land
    let mut all: Vec<Play> = crate::in_use_order(made_cards.iter().cloned().chain(made_innates.iter().cloned()).collect(), &open.s);
    let mut typed_kept: Vec<Play> = Vec::new();
    for p in typed {
        match all.iter_mut().find(|m| m.power == p.power) {
            // used already: its moves were made as the journal has them, whatever the plan writes
            Some(m) if m.target.is_some() && m.target == p.target => notes.push(format!("{} made already at {}", p.power, p.target.as_ref().map(|l| l.to_string()).unwrap_or_default())),
            Some(m) if m.target.is_none() => {
                m.target = p.target.clone();
                m.fast = p.fast;
                m.forced = p.forced;
                m.moves = p.moves.clone();
                m.exact = p.exact;
                m.kills = p.kills.clone();
                notes.push(format!("{} made already: the plan says where", p.power));
            }
            Some(m) => notes.push(format!("{} made already at {}: the plan's {} is dropped", p.power, m.target.as_ref().map(|l| l.to_string()).unwrap_or_default(), p.target.as_ref().map(|l| l.to_string()).unwrap_or_default())),
            None => typed_kept.push(p),
        }
    }
    let typed = typed_kept;
    // a Fast play the journal made already is the plan naming it; a new one is too late
    if past_fast {
        if let Some(p) = typed.iter().find(|p| p.fast) {
            return Err(format!("the Fast phase is over: {} is Fast", p.power));
        }
    }
    all.extend(typed.iter().cloned());
    let cards: Vec<Play> = all.iter().filter(|p| !p.innate).cloned().collect();
    let cost: i64 = cards.iter().map(|p| p.cost).sum();
    if cost > pos.energy {
        notes.push(format!("costs {cost} Energy, {} available", pos.energy));
    }
    if cards.len() as i64 > pos.plays {
        notes.push(format!("{} cards against {} plays", cards.len(), pos.plays));
    }
    for c in typed.iter().filter(|p| !p.innate) {
        if !pos.hand.contains(&c.power) {
            notes.push(format!("{} is not in hand", c.power));
        }
    }
    let growth_value = pos.slots as f64 * values.slot + pos.cards_gained as f64 * values.card + (pos.energy - cost).max(0) as f64 * values.energy;
    let s2 = crate::spirit_at(&open.s, &pos);
    let played: Vec<&PowerCard> = cards.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = crate::elements_for(defs, &s2, &played);
    if let Some(e) = crate::forced_speed_error(defs, spirit, &typed, &elements) {
        return Err(e);
    }
    // `all` acts in order: the plays made already as made, then the plan's as written
    // the next turn as the face-up cards project it: its Ravage is this turn's Build slot,
    // its Build this turn's Explore once known (`informed_build` substitutes the candidate)
    let next = TurnRecord { turn: t.turn + 1, ravage: t.build.clone(), build: t.explore.clone(), ..Default::default() };
    let cands: Vec<Vec<String>> = if at_slow { vec![t.explore.clone()] } else { crate::explore_candidates(rec, defs, &open.deck, t) };
    let cands = if cands.is_empty() { vec![vec![]] } else { cands };
    // one run of the plan under one Explore card; `display` skips the Explore and the deep
    // term, for the certain part shown in the panels
    let simulate = |explore: &[String], display: bool| -> (Outcome, si_rules::Island) {
        let mut out = Outcome::default();
        let mut armed = Vec::new();
        let mut island;
        if at_slow {
            island = verify::island_at(rec, defs, t, "before_slow");
        } else {
            island = verify::island_at(rec, defs, t, "before_fast");
            for (id, l) in island.lands.iter_mut() {
                l.c.presence = s2.presence_after.get(id).copied().unwrap_or(0);
            }
            crate::apply_plays(&mut island, defs, &all, true, &elements, rules, spirit, t, &mut armed, &mut out);
            crate::invader_phase(&mut island, rec, t, defs, rules, &mut out, Some(explore), false);
            // a won game stops: no Slow phase, no next turn
            if out.wins.is_some() {
                return (out, island);
            }
        }
        let informed_build: Option<&[String]> = Some(explore);
        let slow = {
            let eval = |slow: &[Play]| -> f64 {
                let mut isl = island.clone();
                let mut o = out.clone();
                let mut armed2 = armed.clone();
                crate::apply_plays(&mut isl, defs, slow, false, &elements, rules, spirit, t, &mut armed2, &mut o);
                crate::lookahead(&mut isl, rec, Some(&next), defs, rules, &mut o, informed_build, None);
                o.score(values)
            };
            crate::resolve_slow(&island, defs, &all, false, &s2, &graph, &all_innates, &eval)
        };
        let mut all2: Vec<Play> = all.iter().filter(|p| p.fast).cloned().collect();
        all2.extend(slow.iter().cloned());
        crate::apply_plays(&mut island, defs, &all2, false, &elements, rules, spirit, t, &mut armed, &mut out);
        crate::victory_after_slow(&island, rules, t, &mut out);
        let deep = if display { None } else { crate::deep_for(rec, defs, &open.deck, t, explore, true) };
        // the island as this turn leaves it (the panels' `after`), before the next turn's steps
        let end = island.clone();
        if out.wins.is_some() {
            out.slow = slow;
            return (out, end);
        }
        crate::lookahead(&mut island, rec, Some(&next), defs, rules, &mut out, informed_build, deep.as_ref());
        out.slow = slow;
        (out, end)
    };
    let per_card: Vec<f64> = cands.iter().map(|c| simulate(c, false).0.score(values) + growth_value).collect();
    let mean = per_card.iter().sum::<f64>() / per_card.len() as f64;
    let (mut out, island) = simulate(&[], true);
    out.after = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
    let (event, fear_cards, terror) = expectation(open, out.fear);
    let expected = open.cal.expected_exogenous(event, fear_cards, terror);
    let informed = mean + expected;
    let confidence = baseline.filter(|b| b.per_card.len() == per_card.len() && !per_card.is_empty()).map(|b| {
        let noise = open.cal.model_sd * std::f64::consts::SQRT_2;
        per_card.iter().zip(&b.per_card).map(|(x, y)| if noise > 0.05 { crate::normal_cdf((x - y) / noise) } else if x >= y { 1.0 } else { 0.0 }).sum::<f64>() / per_card.len() as f64
    });
    out.notes.extend(notes);
    let plays = crate::with_slow(&all, &out);
    let mut line = Line { growth: pos.growth.clone(), growth_actual: !(at_growth && !growth_text.is_empty()), growth_value, ties: 0, informed, luck: 0.0, confidence, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], expected, per_card, plays, outcome: out, score: informed, actual: false };
    crate::annotate_elements(&mut line, defs, &s2);
    Ok(line)
}

// ---- the draw count and the Ravage thresholds for THINK ABOUT ----

use si_model::island::LandId;
use si_model::power::{PowerKind, Speed};

/// Power Cards of a kind the journal has never shown: not in any spirit's hand, play,
/// discard, forgotten pile, draft or starting uniques.
pub fn unseen_cards<'d>(rec: &GameRecord, defs: &'d Definitions, kind: PowerKind) -> Vec<&'d PowerCard> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for s in &rec.spirits {
        for c in s.hand_after.iter().chain(s.cards_played.iter()).chain(s.discard_after.iter()).chain(s.cards_gained.iter()).chain(s.cards_forgotten.iter()).chain(s.offered.iter()).chain(s.reclaimed_cards.iter()) {
            seen.insert(c.as_str());
        }
    }
    for id in &rec.meta.spirits {
        if let Some(p) = defs.spirits.get(id) {
            seen.extend(p.uniques.iter().map(|c| c.as_str()));
        }
    }
    let mut v: Vec<&PowerCard> = defs.cards.values().filter(|c| c.kind == kind && !seen.contains(c.id.as_str())).collect();
    v.sort_by(|a, b| a.id.cmp(&b.id));
    v
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawCount {
    pub unseen: usize,
    /// Cards that play on the land this turn: Fast, within the Energy, in range from the
    /// position (the same test the review's candidates pass).
    pub playable: usize,
    /// Unseen cards the simulator cannot read in full, counted as not playing.
    pub unreadable: usize,
}

/// Of the unseen cards of a kind, how many could be played on `land` this turn.
pub fn draw_count(defs: &Definitions, open: &OpenTurn, kind: PowerKind, land: &LandId) -> DrawCount {
    let graph = si_metrics::reach::island_for(&open.rec, defs);
    let pos = crate::actual_position(&open.rec, defs, &open.s, &open.t);
    let unseen = unseen_cards(&open.rec, defs, kind);
    let mut playable = 0;
    let mut unreadable = 0;
    for c in &unseen {
        if !c.fully_parsed() {
            unreadable += 1;
            continue;
        }
        if c.speed != Speed::Fast || c.cost > pos.energy {
            continue;
        }
        if crate::play_options(c, &pos.presence, &open.t.before_fast, &graph).iter().any(|p| p.target.as_ref() == Some(land)) {
            playable += 1;
        }
    }
    DrawCount { unseen: unseen.len(), playable, unreadable }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RavageThresholds {
    /// Damage the Ravage deals in the land with every modifier (0 when no card names it).
    pub damage: u32,
    /// The least Defend that keeps Blight out of the land, when any up to the damage does.
    pub defend_keeps_land: Option<u32>,
    /// The least Defend that loses no Dahan, when any up to the damage does.
    pub defend_keeps_dahan: Option<u32>,
}

/// What this turn's Ravage does to one land under each Defend, by running the simulator
/// on a copy of the island: the damage, and the two Defend numbers THINK ABOUT names.
pub fn ravage_thresholds(island: &si_rules::Island, defs: &Definitions, cards: &[String], rules: &Rules, land: &LandId) -> RavageThresholds {
    let run = |defend: u32| -> Option<(u32, u32, u32)> {
        let mut isl = island.clone();
        isl.lands.get_mut(land)?.defend = defend;
        let mut damage = 0;
        let mut blight = 0;
        let mut lost = 0;
        for card in cards.iter().filter_map(|c| defs.invader_cards.get(c)) {
            for o in si_rules::invaders::ravage(&mut isl, card, rules).into_iter().filter(|o| o.land == *land) {
                damage += o.damage;
                blight += o.blight + o.cascade;
                lost += o.dahan_lost;
            }
        }
        Some((damage, blight, lost))
    };
    let Some((damage, blight0, lost0)) = run(0) else { return RavageThresholds { damage: 0, defend_keeps_land: None, defend_keeps_dahan: None } };
    let find = |keeps: &dyn Fn(u32, u32) -> bool, now: bool| -> Option<u32> {
        if now {
            return Some(0);
        }
        (1..=damage).find(|d| run(*d).is_some_and(|(_, b, l)| keeps(b, l)))
    };
    RavageThresholds { damage, defend_keeps_land: find(&|b, _| b == 0, blight0 == 0), defend_keeps_dahan: find(&|_, l| l == 0, lost0 == 0) }
}
