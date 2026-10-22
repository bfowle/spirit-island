//! The review worker: a long-lived thread fed `(generation, record, turns)` jobs, one
//! `review_turns` call per turn so turns land as they finish, `Msg::Finished` each time
//! its queue drains. A record that changed (an undo) gets a new generation; the screen
//! drops results tagged with an old one.

use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;

use si_defs::Definitions;
use si_hindsight::{Progress, ReviewOptions};
use si_record::GameRecord;

use super::Msg;

pub(super) struct Job {
    pub generation: u64,
    pub rec: Arc<GameRecord>,
    pub turns: Vec<u32>,
}

pub(super) fn spawn(defs: Arc<Definitions>, opts: ReviewOptions, out: Sender<Msg>) -> Sender<Job> {
    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    std::thread::spawn(move || {
        let mut meta_sent = false;
        let mut queue: VecDeque<Job> = VecDeque::new();
        loop {
            // wait for a job when idle; otherwise take what has arrived meanwhile
            if queue.is_empty() {
                match rx.recv() {
                    Ok(j) => queue.push_back(j),
                    Err(_) => return,
                }
            }
            loop {
                match rx.try_recv() {
                    Ok(j) => queue.push_back(j),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }
            let Some(job) = queue.pop_front() else { continue };
            let on_progress = |p: Progress| {
                let _ = out.send(Msg::Progress(p));
            };
            for t in job.turns {
                let r = si_hindsight::review_turns(&job.rec, &defs, &opts, Some(&[t]), &on_progress);
                if !meta_sent {
                    let _ = out.send(Msg::Meta(Box::new((r.values.clone(), r.calibration.clone(), r.deck.clone()))));
                    meta_sent = true;
                }
                for tr in r.turns {
                    if out.send(Msg::Turn { generation: job.generation, review: Box::new(tr) }).is_err() {
                        return;
                    }
                }
            }
            // nothing queued behind this job: the footer may say so
            loop {
                match rx.try_recv() {
                    Ok(j) => queue.push_back(j),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }
            if queue.is_empty() && out.send(Msg::Finished { generation: job.generation }).is_err() {
                return;
            }
        }
    });
    tx
}
