//! `CompanionState`: the public snapshot, its stats, the game's history,
//! and source health. Built only from `PublicSnapshot`s, so everything here
//! is already on the public side of the firewall.

use serde::Serialize;
use si_common::companion_stats::review::{self, GameReview};
use si_common::companion_stats::{self, CompanionStats, RoundSummary};
use si_common::public::{PublicDecision, PublicSnapshot};

/// Engine per-turn state order (`Constants.StateOrdering`), used to order
/// snapshots within a turn and to detect undo.
const STATE_ORDER: &[&str] = &[
    "PlayerSetup", "InvadersStartingAction", "SpiritPhase", "Grow", "GainEnergy", "PlayPayPower",
    "FastPower", "BlightedIsland", "Event", "Fear", "HighImmigration", "Ravage", "Build", "Explore",
    "AdvanceInvaderCards", "SlowPower", "TimePasses",
];

pub fn step_rank(step: &str) -> usize {
    STATE_ORDER.iter().position(|s| *s == step).unwrap_or(STATE_ORDER.len())
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CompanionState {
    /// Increments on every change; SSE clients use it to drop duplicates.
    pub seq: u64,
    pub public: Option<PublicSnapshot>,
    pub stats: Option<CompanionStats>,
    pub history: Vec<RoundSummary>,
    /// Hindsight review of the game so far (the dashboard shows it after the game).
    pub review: Option<GameReview>,
    /// Play-by-play: the latest changes between snapshots, newest last.
    pub changes: Vec<companion_stats::changes::Change>,
    /// The player's decisions this game so far, from the log (when a
    /// capture log is available: replay, or a live game being recorded).
    pub decisions: Vec<PublicDecision>,
    pub source: SourceStatus,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SourceStatus {
    /// "live", "replay", or "off".
    pub mode: String,
    /// "running", "not-running", or "unknown".
    pub process: String,
    /// File name of the save this state came from (names carry only the
    /// public step, e.g. `save-Feedback-003-0085-FastPower.dat`).
    pub last_save: Option<String>,
    pub updated_at_ms: u64,
    pub stale: bool,
    pub engine_build: String,
    pub decode_errors: u32,
    pub replay: Option<ReplayStatus>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ReplayStatus {
    pub capture: String,
    pub index: usize,
    pub total: usize,
    pub playing: bool,
    pub speed: f64,
    /// Capture wall-clock time of the current save, `HH:MM:SS`.
    pub clock: String,
}

/// Keeps one game's timeline of public snapshots and derives the state.
#[derive(Default)]
pub struct Companion {
    timeline: Vec<PublicSnapshot>,
    decisions: Vec<PublicDecision>,
    seq: u64,
}

impl Companion {
    /// Add a snapshot. Returns `false` if it duplicates the current one.
    /// A snapshot from another game resets the timeline; one older than the
    /// last (undo) drops everything after it.
    pub fn push(&mut self, snap: PublicSnapshot) -> bool {
        if let Some(last) = self.timeline.last() {
            if last.game_id != snap.game_id {
                self.timeline.clear();
            } else if *last == snap {
                return false;
            } else {
                let key = (snap.turn, step_rank(&snap.step));
                self.timeline.retain(|s| (s.turn, step_rank(&s.step)) <= key);
            }
        }
        self.timeline.push(snap);
        self.seq += 1;
        true
    }

    pub fn reset(&mut self) {
        self.timeline.clear();
        self.seq += 1;
    }

    pub fn timeline(&self) -> &[PublicSnapshot] {
        &self.timeline
    }

    pub fn set_decisions(&mut self, decisions: Vec<PublicDecision>) {
        if decisions != self.decisions {
            self.decisions = decisions;
            self.seq += 1;
        }
    }

    pub fn state(&self, source: SourceStatus) -> CompanionState {
        let public = self.timeline.last().cloned();
        let history = companion_stats::round_summaries(&self.timeline);
        let stats = public.as_ref().map(|p| companion_stats::compute(p, &history));
        let review = public.as_ref().map(|_| review::review(&self.timeline, &self.decisions));
        let changes = companion_stats::changes::recent(&self.timeline, 40);
        CompanionState { seq: self.seq, public, stats, history, review, changes, decisions: self.decisions.clone(), source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snap(game: &str, turn: u32, step: &str, energy: i64) -> PublicSnapshot {
        let mut s = PublicSnapshot { game_id: game.into(), turn, step: step.into(), ..Default::default() };
        s.spirits.push(si_common::public::PublicSpirit { energy, ..Default::default() });
        s
    }

    #[test]
    fn undo_drops_newer_snapshots() {
        let mut c = Companion::default();
        assert!(c.push(snap("g", 2, "SpiritPhase", 1)));
        assert!(c.push(snap("g", 2, "FastPower", 1)));
        assert!(c.push(snap("g", 2, "Ravage", 1)));
        // Undo back into the Spirit phase.
        assert!(c.push(snap("g", 2, "SpiritPhase", 3)));
        let steps: Vec<_> = c.timeline().iter().map(|s| s.step.as_str()).collect();
        assert_eq!(steps, ["SpiritPhase", "SpiritPhase"]);
    }

    #[test]
    fn duplicate_is_ignored_and_new_game_resets() {
        let mut c = Companion::default();
        assert!(c.push(snap("g", 1, "SpiritPhase", 1)));
        assert!(!c.push(snap("g", 1, "SpiritPhase", 1)));
        assert!(c.push(snap("h", 1, "PlayerSetup", 0)));
        assert_eq!(c.timeline().len(), 1);
    }
}
