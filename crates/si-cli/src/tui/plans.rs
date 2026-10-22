//! The plan scorer: builds the open turn from the record and the seed, scores the `none`
//! plan, then each typed plan against it. Results carry the generation of the record and
//! the turn and phase they were scored on, so a stale batch is dropped.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::live::{open_turn, score_plan, OpenTurn};
use si_hindsight::{Calibration, Line};
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::GameRecord;

use super::Msg;

pub(super) struct Job {
    pub generation: u64,
    pub rec: Arc<GameRecord>,
    pub seed: Seed,
    pub texts: Vec<String>,
}

/// What the open tab shows: the open turn (its forecast and Explore pool computed here),
/// the `none` row, then one row per plan text in order, scored or refused.
pub(super) struct Scored {
    pub generation: u64,
    pub turn: u32,
    pub phase: Phase,
    pub open: Box<OpenTurn>,
    /// Energy and plays at the position the plans start from.
    pub energy: i64,
    pub plays: i64,
    pub cands: Vec<String>,
    pub forecast: Vec<si_hindsight::deck::TurnForecast>,
    pub none: Line,
    pub rows: Vec<(String, Result<Line, String>)>,
}

pub(super) fn spawn(defs: Arc<Definitions>, cal: Calibration, out: Sender<Msg>) -> Sender<Job> {
    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    std::thread::spawn(move || {
        while let Ok(mut job) = rx.recv() {
            // only the newest job matters: drain the queue
            while let Ok(newer) = rx.try_recv() {
                job = newer;
            }
            let Some(open) = open_turn(Arc::clone(&job.rec), &defs, &job.seed, &cal) else {
                let _ = out.send(Msg::NoOpenTurn { generation: job.generation });
                continue;
            };
            let none = match score_plan(&defs, &open, None, "") {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("the none plan failed: {e}");
                    let _ = out.send(Msg::NoOpenTurn { generation: job.generation });
                    continue;
                }
            };
            let rows: Vec<(String, Result<Line, String>)> = job.texts.iter().map(|t| (t.clone(), score_plan(&defs, &open, Some(&none), t))).collect();
            let cands: Vec<String> = si_hindsight::explore_candidates(&open.rec, &defs, &open.deck, &open.t).into_iter().filter_map(|c| c.into_iter().next()).collect();
            let island = si_rules::verify::island_for(&open.rec, &defs, &open.t.before_fast);
            let forecast = si_hindsight::deck::forecast(&open.rec, &defs, &open.deck, &open.t, &island, &open.rules, 3, Some(&cands));
            let pos = si_hindsight::actual_position(&open.rec, &defs, &open.s, &open.t);
            let scored = Scored { generation: job.generation, turn: open.turn, phase: open.phase, energy: pos.energy, plays: pos.plays, cands, forecast, none, rows, open: Box::new(open) };
            if out.send(Msg::Plans(Box::new(scored))).is_err() {
                return;
            }
        }
    });
    tx
}
