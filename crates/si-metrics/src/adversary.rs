//! The adversary level is not logged. It shows through three things the log
//! does carry: the Invader deck's stage order (levels reorder and trim the
//! deck), the Fear deck's size per Terror Level (levels add cards), and the
//! pieces the level adds at setup. Every level consistent with all three is a
//! candidate; the report prints the set, never a guess.

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::island::LandId;
use si_model::setup::Adversary;
use si_record::{Counts, GameRecord};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelEstimate {
    pub adversary: String,
    pub candidates: Vec<u8>,
    /// Difficulty range over the candidates (adversary alone, no scenario).
    pub difficulty: Option<(u8, u8)>,
    pub evidence: Vec<String>,
}

impl LevelEstimate {
    pub fn label(&self) -> String {
        match self.candidates.as_slice() {
            [] => "?".into(),
            [one] => one.to_string(),
            many => format!("{}-{}", many[0], many[many.len() - 1]),
        }
    }
}

fn stage_of(card: &str, defs: &Definitions) -> Option<u8> {
    defs.invader_cards.get(card).map(|c| c.stage).or_else(|| card.chars().last().and_then(|c| c.to_digit(10)).map(|d| d as u8))
}

/// Pieces the board definition plus the level's setup additions expect on
/// each land of `board` (towns, cities, blight exact; explorers a floor).
fn expected_setup(defs: &Definitions, adv: &Adversary, level: u8, board: &str) -> Option<Vec<(LandId, Counts)>> {
    let b = defs.board_def(board)?;
    let mut lands: Vec<(LandId, Counts)> = b
        .lands
        .iter()
        .map(|l| (l.id.clone(), Counts { explorers: l.initial.explorers as u32, towns: l.initial.towns as u32, cities: l.initial.cities as u32, dahan: l.initial.dahan as u32, blight: l.initial.blight as u32, ..Default::default() }))
        .collect();
    let number = |id: &LandId| id.number;
    let mut levels: Vec<_> = adv.levels.iter().filter(|l| l.number <= level).collect();
    levels.sort_by_key(|l| l.number);
    for l in levels {
        let s = &l.setup;
        for n in &s.towns_in_land {
            if let Some(e) = lands.iter_mut().find(|(id, _)| number(id) == *n) {
                e.1.towns += 1;
            }
        }
        for n in &s.cities_in_land {
            if let Some(e) = lands.iter_mut().find(|(id, _)| number(id) == *n) {
                e.1.cities += 1;
            }
        }
        for n in &s.blight_in_land {
            if let Some(e) = lands.iter_mut().find(|(id, _)| number(id) == *n) {
                e.1.blight += 1;
            }
        }
        if let Some((from, to)) = s.move_blight {
            let moved = lands.iter_mut().find(|(id, c)| number(id) == from && c.blight > 0).map(|e| {
                e.1.blight -= 1;
            });
            if moved.is_some() {
                if let Some(e) = lands.iter_mut().find(|(id, _)| number(id) == to) {
                    e.1.blight += 1;
                }
            }
        }
        if s.town_highest_without_town {
            if let Some(e) = lands.iter_mut().filter(|(_, c)| c.towns == 0).max_by_key(|(id, _)| number(id)) {
                e.1.towns += 1;
            }
        }
        if s.explorer_highest_without_town_city {
            if let Some(e) = lands.iter_mut().filter(|(_, c)| c.buildings() == 0).max_by_key(|(id, _)| number(id)) {
                e.1.explorers += 1;
            }
        }
    }
    Some(lands)
}

fn setup_consistent(rec: &GameRecord, defs: &Definitions, adv: &Adversary, level: u8) -> Option<bool> {
    if rec.setup.is_empty() || !rec.meta.from_setup {
        return None;
    }
    let mut checked = false;
    for board in &rec.meta.boards {
        let Some(expected) = expected_setup(defs, adv, level, board) else { continue };
        let (mut want_blight, mut got_blight) = (0u32, 0u32);
        for (land, want) in expected {
            let got = rec.setup.get(&land).copied().unwrap_or_default();
            checked = true;
            // the level's additions must be there; more may be (a scenario's
            // extra Build and Explore, the setup Explore itself)
            if got.towns < want.towns || got.cities < want.cities || got.explorers < want.explorers {
                return Some(false);
            }
            want_blight += want.blight;
            got_blight += got.blight;
        }
        // Blight per board: a level's setup move of the starting Blight
        // (Sweden 2) is not logged, so lands cannot be compared one by one
        if got_blight < want_blight {
            return Some(false);
        }
    }
    checked.then_some(true)
}

pub fn estimate(rec: &GameRecord, defs: &Definitions) -> Option<LevelEstimate> {
    let id = rec.meta.adversary.as_ref()?;
    let adv = defs.adversaries.get(id)?;
    if let Some(level) = rec.meta.adversary_level {
        let difficulty = adv.level(level).map(|l| (l.difficulty, l.difficulty));
        return Some(LevelEstimate { adversary: id.clone(), candidates: vec![level], difficulty, evidence: vec!["stated by the engine save beside the log".into()] });
    }
    let mut seen: Vec<String> = Vec::new();
    for c in rec.turns.iter().flat_map(|t| t.explore.iter()) {
        if !seen.contains(c) {
            seen.push(c.clone());
        }
    }
    let observed: Vec<u8> = seen.iter().filter_map(|c| stage_of(c, defs)).collect();
    let fear = rec.fear_earned_by_level;
    let terror_end = rec.turns.last().map(|t| t.terror_after).unwrap_or(1);
    let mut evidence = vec![format!("Invader deck seen: {}", observed.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""))];
    if terror_end >= 2 {
        evidence.push(format!("Terror Level I had {} Fear cards", fear[0]));
    } else {
        evidence.push(format!("{} Fear cards earned, Terror Level I never finished", fear[0]));
    }
    if terror_end >= 3 {
        evidence.push(format!("Terror Level II had {} Fear cards", fear[1]));
    }
    let mut candidates = Vec::new();
    let mut setup_said = None;
    // the save's difficulty, when it holds no scenario's: the level's own. Level 0 is a
    // candidate only on that evidence: the base difficulty, or (where the definitions leave
    // the base out) a difficulty below every level's
    let saved = rec.meta.difficulty.filter(|_| rec.meta.scenario.is_none());
    let lowest = adv.levels.iter().map(|l| l.difficulty as i64).min();
    let base_known = adv.base_difficulty > 0;
    let difficulty_of = |n: u8| -> Option<i64> { if n == 0 { base_known.then_some(adv.base_difficulty as i64).or(saved.filter(|d| lowest.is_some_and(|m| *d < m))) } else { adv.level(n).map(|l| l.difficulty as i64) } };
    if let Some(d) = saved {
        evidence.push(format!("the save beside the log says difficulty {d}"));
    }
    for n in 0..=6u8 {
        if n == 0 && (saved.is_none() || saved != difficulty_of(0)) {
            continue;
        }
        if n > 0 && adv.level(n).is_none() {
            continue;
        }
        if saved.is_some_and(|d| difficulty_of(n) != Some(d)) {
            continue;
        }
        let order = adv.stages_at(n);
        let deck_ok = observed.len() <= order.len() && observed.iter().zip(order.iter()).all(|(a, b)| a == b);
        let fd = adv.fear_deck_at(n);
        let fear_ok = (if terror_end >= 2 { fear[0] == fd[0] as u32 } else { fear[0] <= fd[0] as u32 }) && (if terror_end >= 3 { fear[1] == fd[1] as u32 } else { fear[1] <= fd[1] as u32 });
        let setup = setup_consistent(rec, defs, adv, n);
        if setup.is_some() {
            setup_said = Some(true);
        }
        if deck_ok && fear_ok && setup.unwrap_or(true) {
            candidates.push(n);
        }
    }
    match setup_said {
        Some(_) => evidence.push("setup pieces checked against each level's additions".into()),
        None => evidence.push("setup pieces not available (log joined after setup, or unknown board)".into()),
    }
    let difficulty = (!candidates.is_empty()).then(|| {
        let ds: Vec<u8> = candidates.iter().filter_map(|n| difficulty_of(*n).map(|d| d as u8)).collect();
        (*ds.iter().min().unwrap(), *ds.iter().max().unwrap())
    });
    Some(LevelEstimate { adversary: id.clone(), candidates, difficulty, evidence })
}
