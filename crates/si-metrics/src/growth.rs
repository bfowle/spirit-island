//! Growth as a type (Lumia), both sides. Each implementation names the rule
//! or post its closed form comes from; `fit` bins an observed series by the
//! order that explains it best.

use serde::{Deserialize, Serialize};
use si_model::spirit::SpiritPanel;

use crate::fit::least_squares;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GrowthOrder {
    Constant,
    Linear,
    Quadratic,
    Power(f64),
    Exponential(f64),
    Factorial,
    Step { period: u32 },
    Sawtooth { period: u32 },
    /// Growth that saturates (Explores spread until the board is full).
    Logistic,
}

impl GrowthOrder {
    pub fn name(&self) -> String {
        match self {
            GrowthOrder::Constant => "constant".into(),
            GrowthOrder::Linear => "linear".into(),
            GrowthOrder::Quadratic => "quadratic".into(),
            GrowthOrder::Power(p) => format!("n^{p:.1}"),
            GrowthOrder::Exponential(b) => format!("{b:.2}^n"),
            GrowthOrder::Factorial => "factorial".into(),
            GrowthOrder::Step { period } => format!("step/{period}"),
            GrowthOrder::Sawtooth { period } => format!("sawtooth/{period}"),
            GrowthOrder::Logistic => "logistic".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Spirit,
    Invader,
}

pub trait Growth {
    fn name(&self) -> &str;
    fn side(&self) -> Side;
    fn order(&self) -> GrowthOrder;
    /// Closed form after `n` uses or turns.
    fn value(&self, n: u32) -> f64;
    /// The rule or text the form comes from.
    fn source(&self) -> &str;
}

// ---- Spirit side ------------------------------------------------------------

/// A presence track read left to right: the value after `n` slots revealed.
pub struct TrackSteps {
    pub name: String,
    pub values: Vec<i64>,
    pub source: String,
}

impl TrackSteps {
    fn walk(panel: &SpiritPanel, track: si_model::spirit::Track, pick: impl Fn(&si_model::spirit::TrackState) -> i64, name: &str) -> TrackSteps {
        let mut revealed = panel.starting_revealed();
        let mut values = vec![pick(&panel.track_state(&revealed))];
        let mut nodes: Vec<&si_model::spirit::TrackNode> = panel.tracks.iter().filter(|n| n.track == track && !revealed.contains(&n.id)).collect();
        // follow the `next` links from the revealed start
        let mut ordered = Vec::new();
        let mut cur: Option<&si_model::spirit::TrackNode> = panel.tracks.iter().find(|n| n.track == track && revealed.contains(&n.id));
        while let Some(c) = cur {
            cur = c.next.first().and_then(|id| nodes.iter().position(|n| n.id == *id)).map(|i| nodes.remove(i));
            if let Some(n) = cur {
                ordered.push(n);
            }
        }
        for n in ordered {
            revealed.insert(n.id.clone());
            values.push(pick(&panel.track_state(&revealed)));
        }
        TrackSteps { name: format!("{} {name}", panel.title), values, source: format!("{}'s printed presence track", panel.title) }
    }

    pub fn income(panel: &SpiritPanel) -> TrackSteps {
        Self::walk(panel, si_model::spirit::Track::Energy, |s| s.income, "Energy per turn")
    }

    pub fn plays(panel: &SpiritPanel) -> TrackSteps {
        Self::walk(panel, si_model::spirit::Track::Plays, |s| s.plays, "Card Plays per turn")
    }
}

impl Growth for TrackSteps {
    fn name(&self) -> &str {
        &self.name
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Step { period: 1 }
    }
    fn value(&self, n: u32) -> f64 {
        self.values[(n as usize).min(self.values.len() - 1)] as f64
    }
    fn source(&self) -> &str {
        &self.source
    }
}

/// Presence on the island: at most `per_turn` placed each Growth.
pub struct PresenceOnIsland {
    pub start: u32,
    pub per_turn: u32,
}

impl Growth for PresenceOnIsland {
    fn name(&self) -> &str {
        "presence on the island"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Linear
    }
    fn value(&self, n: u32) -> f64 {
        (self.start + self.per_turn * n) as f64
    }
    fn source(&self) -> &str {
        "Growth: the largest presence option on every panel we hold places two a turn"
    }
}

/// Banked Energy with a fixed income and spend.
pub struct BankedEnergy {
    pub start: i64,
    pub income: i64,
    pub spend: i64,
}

impl Growth for BankedEnergy {
    fn name(&self) -> &str {
        "banked Energy"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Linear
    }
    fn value(&self, n: u32) -> f64 {
        (self.start + (self.income - self.spend) * n as i64) as f64
    }
    fn source(&self) -> &str {
        "Spirit phase: gain Energy per turn from the track, pay for the cards played"
    }
}

/// Hand size with gains each turn and a Reclaim every `period` turns.
pub struct HandSawtooth {
    pub start: u32,
    pub gain_per_turn: u32,
    pub plays: u32,
    pub period: u32,
}

impl Growth for HandSawtooth {
    fn name(&self) -> &str {
        "cards in hand"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Sawtooth { period: self.period }
    }
    fn value(&self, n: u32) -> f64 {
        (self.start + self.gain_per_turn * n) as f64 - (self.plays * (n % self.period.max(1))) as f64
    }
    fn source(&self) -> &str {
        "Time Passes discards played cards; Reclaim returns the discard to hand"
    }
}

/// A Power repeated on the same land, each repeat one Energy (or Time) dearer.
pub struct RepeatCost;

impl Growth for RepeatCost {
    fn name(&self) -> &str {
        "repeat cost"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Quadratic
    }
    fn value(&self, n: u32) -> f64 {
        (n * n.saturating_sub(1) / 2) as f64
    }
    fn source(&self) -> &str {
        "Lumia: each additional repeat costs one more than the last, so n repeats cost n(n−1)/2 in total"
    }
}

/// Beasts gathered into a lair that doubles them.
pub struct LairGather;

impl Growth for LairGather {
    fn name(&self) -> &str {
        "lair gather"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Exponential(2.0)
    }
    fn value(&self, n: u32) -> f64 {
        2f64.powi(n as i32)
    }
    fn source(&self) -> &str {
        "Lumia: a doubling each turn is exponential growth"
    }
}

/// Lumia's Melt Earth recurrence: meme(n) = 1 + ceil(n/2) · meme(n−1).
pub struct MeltEarth;

impl Growth for MeltEarth {
    fn name(&self) -> &str {
        "Melt Earth into Quicksand repeats"
    }
    fn side(&self) -> Side {
        Side::Spirit
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Factorial
    }
    fn value(&self, n: u32) -> f64 {
        let mut v = 1f64;
        for k in 1..=n {
            v = 1.0 + ((k + 1) / 2) as f64 * v;
        }
        v
    }
    fn source(&self) -> &str {
        "Lumia, docs/guides/lumia: meme(n) = 1 + ceil(n/2)·meme(n−1)"
    }
}

// ---- Invader side -----------------------------------------------------------

/// One Invader card's chain in one land: Explore, Build, Ravage, then done.
pub struct LandChain;

impl Growth for LandChain {
    fn name(&self) -> &str {
        "land chain"
    }
    fn side(&self) -> Side {
        Side::Invader
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Constant
    }
    fn value(&self, _n: u32) -> f64 {
        3.0
    }
    fn source(&self) -> &str {
        "Invader phase: each card is Explored, then Built, then Ravaged, three steps per land"
    }
}

/// Lands with Invaders as Explores seed from Towns and Cities: geometric
/// until the board saturates.
pub struct Spread {
    pub lands: u32,
    pub start: u32,
    pub rate: f64,
}

impl Growth for Spread {
    fn name(&self) -> &str {
        "spread of Invaders over lands"
    }
    fn side(&self) -> Side {
        Side::Invader
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Logistic
    }
    fn value(&self, n: u32) -> f64 {
        let l = self.lands as f64;
        let s = self.start.max(1) as f64;
        l / (1.0 + (l / s - 1.0) * (-self.rate * n as f64).exp())
    }
    fn source(&self) -> &str {
        "Explore: Explorers enter lands adjacent to a Town or City (or the coast), so Towns breed more lands with Invaders until none are left"
    }
}

/// The Invader deck: cards per stage set when each stage begins.
pub struct StageEscalation {
    pub stages: Vec<u8>,
}

impl Growth for StageEscalation {
    fn name(&self) -> &str {
        "Invader stage"
    }
    fn side(&self) -> Side {
        Side::Invader
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Step { period: 4 }
    }
    fn value(&self, n: u32) -> f64 {
        self.stages.get(n as usize).copied().unwrap_or(3) as f64
    }
    fn source(&self) -> &str {
        "the Invader deck order (base 111-2222-33333; adversary levels reorder it)"
    }
}

/// Blight on the island with cascades: Lumia's ~n^1.5.
pub struct BlightCascade;

impl Growth for BlightCascade {
    fn name(&self) -> &str {
        "Blight with cascades"
    }
    fn side(&self) -> Side {
        Side::Invader
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Power(1.5)
    }
    fn value(&self, n: u32) -> f64 {
        (n as f64).powf(1.5)
    }
    fn source(&self) -> &str {
        "Lumia: Blight added to a Blighted land cascades, so Blight grows faster than linearly, about n^1.5"
    }
}

/// Turns until the Invader deck runs out.
pub struct DeckDepth {
    pub cards: u32,
}

impl Growth for DeckDepth {
    fn name(&self) -> &str {
        "Invader deck depth"
    }
    fn side(&self) -> Side {
        Side::Invader
    }
    fn order(&self) -> GrowthOrder {
        GrowthOrder::Linear
    }
    fn value(&self, n: u32) -> f64 {
        self.cards.saturating_sub(n) as f64
    }
    fn source(&self) -> &str {
        "one Invader card is revealed each Explore step"
    }
}

// ---- Fitting an observed series --------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fit {
    pub order: GrowthOrder,
    pub rmse: f64,
    /// Fitted parameters in the order's own terms (slope, curvature or base).
    pub params: Vec<f64>,
}

/// The simplest order within 10% of the best error over constant, linear,
/// quadratic and exponential forms. The error is the RMSE scaled by
/// sqrt(n / (n − p)) for p fitted parameters, so a three-parameter curve has
/// to earn its place on a short series. Series shorter than three points
/// give no fit.
pub fn fit(series: &[f64]) -> Option<Fit> {
    if series.len() < 3 {
        return None;
    }
    let n: Vec<f64> = (0..series.len()).map(|i| i as f64).collect();
    let len = series.len() as f64;
    let rmse = |pred: &[f64], params: usize| {
        let raw = (pred.iter().zip(series).map(|(p, y)| (p - y).powi(2)).sum::<f64>() / len).sqrt();
        raw * (len / (len - params as f64).max(1.0)).sqrt()
    };
    let mut fits: Vec<Fit> = Vec::new();
    let mean = series.iter().sum::<f64>() / series.len() as f64;
    fits.push(Fit { order: GrowthOrder::Constant, rmse: rmse(&vec![mean; series.len()], 1), params: vec![mean] });
    if let Some(b) = least_squares(&n.iter().map(|t| vec![1.0, *t]).collect::<Vec<_>>(), series) {
        let pred: Vec<f64> = n.iter().map(|t| b[0] + b[1] * t).collect();
        fits.push(Fit { order: GrowthOrder::Linear, rmse: rmse(&pred, 2), params: vec![b[1]] });
    }
    if series.len() >= 5 {
        if let Some(b) = least_squares(&n.iter().map(|t| vec![1.0, *t, t * t]).collect::<Vec<_>>(), series) {
            let pred: Vec<f64> = n.iter().map(|t| b[0] + b[1] * t + b[2] * t * t).collect();
            fits.push(Fit { order: GrowthOrder::Quadratic, rmse: rmse(&pred, 3), params: vec![b[1], b[2]] });
        }
        if series.iter().all(|y| *y > 0.0) {
            let logs: Vec<f64> = series.iter().map(|y| y.ln()).collect();
            if let Some(b) = least_squares(&n.iter().map(|t| vec![1.0, *t]).collect::<Vec<_>>(), &logs) {
                let pred: Vec<f64> = n.iter().map(|t| (b[0] + b[1] * t).exp()).collect();
                fits.push(Fit { order: GrowthOrder::Exponential(b[1].exp()), rmse: rmse(&pred, 2), params: vec![b[1].exp()] });
            }
        }
    }
    let best = fits.iter().map(|f| f.rmse).fold(f64::INFINITY, f64::min);
    let spread = series.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - series.iter().cloned().fold(f64::INFINITY, f64::min);
    // simplest form whose error is within 10% of the best, or within 2% of the
    // series' range (so flat noise reads as constant)
    let bar = (best * 1.1).max(spread * 0.02) + 1e-9;
    fits.into_iter().find(|f| f.rmse <= bar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_forms() {
        let r = RepeatCost;
        assert_eq!((0..7).map(|n| r.value(n) as u32).collect::<Vec<_>>(), vec![0, 0, 1, 3, 6, 10, 15]);
        let m = MeltEarth;
        assert_eq!((0..6).map(|n| m.value(n) as u32).collect::<Vec<_>>(), vec![1, 2, 3, 7, 15, 46]);
        assert_eq!(LairGather.value(5), 32.0);
        assert_eq!(PresenceOnIsland { start: 2, per_turn: 2 }.value(3), 8.0);
        assert_eq!(HandSawtooth { start: 4, gain_per_turn: 0, plays: 2, period: 3 }.value(2), 0.0);
        assert_eq!(StageEscalation { stages: vec![1, 1, 1, 2, 2, 2, 2, 3] }.value(3), 2.0);
        let s = Spread { lands: 8, start: 2, rate: 0.7 };
        assert!(s.value(0) > 1.9 && s.value(0) < 2.1 && s.value(20) > 7.9);
    }

    #[test]
    fn series_fits_pick_the_simplest_adequate_order() {
        assert_eq!(fit(&[2.0, 2.0, 2.0, 2.0]).unwrap().order, GrowthOrder::Constant);
        assert_eq!(fit(&[1.0, 3.0, 5.0, 7.0, 9.0]).unwrap().order, GrowthOrder::Linear);
        assert_eq!(fit(&[0.0, 1.0, 4.0, 9.0, 16.0, 25.0]).unwrap().order, GrowthOrder::Quadratic);
        assert!(matches!(fit(&[1.0, 2.0, 4.0, 8.0, 16.0, 32.0]).unwrap().order, GrowthOrder::Exponential(b) if (b - 2.0).abs() < 0.05));
        assert!(fit(&[1.0, 2.0]).is_none());
    }
}
