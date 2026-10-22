//! Hindsight, first version: one turn at a time, the Fast phase. For each
//! turn the hand, plays and Energy the spirit had are known from the record;
//! every legal set of Fast plays and targets is simulated, followed by the
//! Invader steps that really came (their cards were already on the board when
//! the choice was made, so this is "informed" information, not hindsight,
//! except the Explore). The ledger scores each line; the actual line is one
//! of them. Spec: docs/superpowers/specs/2026-10-20-hindsight-analysis.md.
//!
//! Slow plays are applied after the Invader steps on the simulated position,
//! and the next turn's Invader steps are run on what the line leaves (no
//! further Spirit action), at half weight.
//!
//! Second pass: growth is reviewed too. Every growth the panel allows from the
//! turn's starting position (group, top or bottom slot for each presence, every
//! legal placement, each card of the offered draft, the Reclaim) is a
//! `Position`, and the plays are enumerated from each position; the actual
//! growth and plays are one line among them. The Event and Fear piece effects
//! are applied to every line as the log shows them (the difference between the
//! simulated Fast phase and the recorded state at the first Invader step), and
//! every Event or Fear choice the player was asked for (which land takes the
//! damage, the Blight, the Town…) is scored against its alternatives where the
//! effect can be placed in another land.
//!
//! Third pass (decision-structure spec, section 3): a line commits its Slow cards before
//! the Explore and takes their lands after the Invader steps (`resolve_slow`); the
//! informed score adds the measured expectation of the Event and Fear cards
//! (`Calibration`) instead of leaving them out; the lookahead runs one more expected
//! turn for the ranked lines (`Deep`); the next-turn weight and the reading bands come
//! from `si calibrate`; the Invader deck is forecast per land (`deck`). Not yet: the
//! Event and Fear decks simulated card by card, growth for spirits that pick two groups.

pub mod deck;
pub mod fuzz;
pub mod live;
pub mod repeat;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::island::{LandId, PieceKind};
use si_model::game::Phase;
use si_model::power::{DamageTarget, Effect, PowerCard, Speed};
use si_model::spirit::{Placement, Reclaim, Track};
use si_model::ElementSet;
use si_record::{Choice, Counts, GameRecord, SpiritTurn, TouchKind, TurnRecord};
use si_rules::effects::{self, Ctx};
pub use si_rules::effects::Move;
use si_rules::invaders::{self, Island, Rules};
use si_rules::verify;

/// The ledger's exchange rates, printed with every score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Values {
    pub blight: f64,
    pub dahan: f64,
    pub presence: f64,
    pub explorer: f64,
    pub town: f64,
    pub city: f64,
    pub fear: f64,
    /// A presence track slot revealed by growth (Rei: scaling over cards).
    pub slot: f64,
    /// A Power Card gained.
    pub card: f64,
    /// An Energy left unspent at the end of the turn.
    pub energy: f64,
    /// Weight of the next turn's Blight and Dahan (the lookahead with no Spirit action):
    /// the share of such a threat that materializes once the Spirit plays its next turn,
    /// measured by `si calibrate`. The turn after counts at its square.
    #[serde(default = "default_next_weight")]
    pub next_weight: f64,
    /// The game lost (an adversary's loss condition): worse than any turn's pieces, so a
    /// losing line ranks under every line that does not lose. The next turn's loss counts
    /// at the next-turn weight (the Spirit plays first and may stop it).
    #[serde(default = "default_loss")]
    pub loss: f64,
    pub source: String,
    /// The spirit's own adjustments, one sentence each (`SpiritPanel::priorities`).
    #[serde(default)]
    pub reasons: Vec<String>,
}

fn default_next_weight() -> f64 {
    0.5
}

fn default_loss() -> f64 {
    -25.0
}

impl Values {
    /// The generic ledger: the same for every spirit.
    pub fn generic() -> Values {
        Values {
            blight: -1.0,
            dahan: -0.5,
            presence: -0.5,
            explorer: 0.25,
            town: 0.5,
            city: 0.75,
            fear: 0.25,
            slot: 0.5,
            card: 0.25,
            energy: 0.1,
            next_weight: 0.5,
            loss: default_loss(),
            source: "Blight 1 (the loss clock); Dahan and presence half each; Explorer/Town/City a quarter, half, three quarters, destroyed for and built against; an Invader pushed out a quarter; Fear a quarter (four Fear is a card); a track slot revealed a half (Rei: scaling over cards), a card gained a quarter, an Energy kept a tenth; the next turn's Blight, Dahan and Builds at the calibrated weight, the turn after at its square".into(),
            reasons: vec![],
        }
    }

    /// The generic ledger scaled by what the spirit's own rules say
    /// (`SpiritPanel::priorities`): the panel decides, not a name.
    pub fn for_panel(panel: Option<&si_model::spirit::SpiritPanel>) -> Values {
        let mut v = Values::generic();
        if let Some(p) = panel {
            let pr = p.priorities();
            v.dahan *= pr.dahan;
            v.presence *= pr.presence;
            v.reasons = pr.reasons;
        }
        v
    }
}

/// Measured constants the review uses where the rules give none: the expected ledger
/// value of an Event and of a Fear card, the share of a next-turn threat that
/// materializes once the Spirit plays its turn, and the spread of a turn's luck the
/// reading bands are standardized against. `si calibrate` measures them over the corpus
/// and writes `data/calibration.json`; the built-in values are the last measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calibration {
    pub source: String,
    /// Ledger value of an Event on the turn's actual line (least squares with the Fear cards
    /// over the whole hidden effect: what the line scored on the cards that came minus what
    /// it scored on the Explore card alone).
    pub event_mean: f64,
    pub event_n: u32,
    /// Per Terror Level: ledger value of one Fear card, fitted the same way.
    pub fear_card_mean: [f64; 3],
    pub fear_card_n: [u32; 3],
    /// Realized next-turn Blight and Dahan over the threat with no Spirit action.
    pub next_weight: f64,
    pub next_n: u32,
    /// Standard deviation of a turn's luck: the Explore card's spread plus the exogenous residual.
    pub luck_sd: f64,
    pub luck_n: u32,
    /// The simulator's own error on the actual line's Invader phase, in ledger units: RMS of
    /// the recorded state at the Slow phase minus the simulated one. Confidence is smoothed by it.
    #[serde(default)]
    pub model_sd: f64,
    #[serde(default)]
    pub model_n: u32,
}

impl Default for Calibration {
    fn default() -> Self {
        // the last `si calibrate` run (data/calibration.json carries the current one)
        Calibration { source: "built-in: si calibrate 2026-10-21 over 24 games, 149 turns (captures and the Steam folder)".into(), event_mean: 0.895, event_n: 116, fear_card_mean: [0.176, 0.611, 0.283], fear_card_n: [70, 32, 7], next_weight: 0.332, next_n: 98, luck_sd: 1.370, luck_n: 148, model_sd: 1.402, model_n: 131 }
    }
}

impl Calibration {
    /// What the informed score adds for the turn's face-down Event and Fear cards: the
    /// Event's mean when one resolves (from turn 2) and each resolved Fear card's mean at
    /// the Terror Level the turn began at. The means are fitted to the whole hidden
    /// effect on a turn (pieces, Defends, skips, and the next turn's cards in the
    /// lookahead), so nothing is added for the next turn separately.
    pub fn expected_exogenous(&self, event: bool, fear_cards: u32, terror: u8) -> f64 {
        let tl = terror.clamp(1, 3) as usize - 1;
        (if event { self.event_mean } else { 0.0 }) + self.fear_card_mean[tl] * fear_cards as f64
    }

    pub fn words(&self) -> String {
        format!(
            "Event {:+.2} ({} turns), Fear card TL1 {:+.2} TL2 {:+.2} TL3 {:+.2} ({}/{}/{} turns), next-turn weight {:.2} ({} turns), a turn's luck sd {:.2} ({} turns), simulator error {:.2} ({} turns); {}",
            self.event_mean, self.event_n, self.fear_card_mean[0], self.fear_card_mean[1], self.fear_card_mean[2], self.fear_card_n[0], self.fear_card_n[1], self.fear_card_n[2], self.next_weight, self.next_n, self.luck_sd, self.luck_n, self.model_sd, self.model_n, self.source
        )
    }
}

/// The standard normal distribution function (Abramowitz and Stegun 7.1.26, error
/// under 1.5e-7).
pub fn normal_cdf(x: f64) -> f64 {
    let z = x / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.3275911 * z.abs());
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let erf = 1.0 - poly * (-z * z).exp();
    0.5 * (1.0 + if z >= 0.0 { erf } else { -erf })
}

/// Raw material one turn contributes to `si calibrate`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    pub event: bool,
    pub fear_cards: u32,
    /// Terror Level as the turn began.
    pub terror: u8,
    /// Ledger value of the Event and Fear piece effects as the log shows them.
    pub exogenous: Option<f64>,
    /// The whole hidden effect on the actual line: its score on the cards that came (best
    /// Slow targets) minus its score on the real Explore card with the Event and Fear cards
    /// left out. What `Calibration` fits.
    pub hidden_effect: Option<f64>,
    /// The actual line's score under each possible Explore card (informed mode, before the exogenous expectation).
    pub per_card: Vec<f64>,
    /// Next turn's Ravage and Build on the recorded position with no Spirit action: Blight, Dahan lost.
    pub next_threat: Option<(u32, u32)>,
    /// What the next turn's Invader steps took as recorded: Blight (cascades included), Dahan lost.
    pub next_actual: Option<(u32, u32)>,
    /// Ledger value of the recorded state at the Slow phase minus the simulated one for the
    /// actual line (the simulator's error on this turn's Invader phase).
    #[serde(default)]
    pub model_error: Option<f64>,
}

/// What a line produced over the turn, in raw pieces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub blight: u32,
    pub dahan_lost: u32,
    pub presence_destroyed: u32,
    pub explorers_destroyed: u32,
    pub towns_destroyed: u32,
    pub cities_destroyed: u32,
    pub invaders_pushed: u32,
    pub fear: u32,
    /// Presence a Power added to the island (Gift of Proliferation): a growth's worth,
    /// the presence and the track slot it came off.
    #[serde(default)]
    pub presence_added: u32,
    /// Towns and Cities the turn's Build added (a Build the line stopped is one fewer).
    #[serde(default)]
    pub built_towns: u32,
    #[serde(default)]
    pub built_cities: u32,
    /// The next turn's Invader steps run on the position this line leaves,
    /// with no further Spirit action: what the position carries forward.
    pub next_blight: u32,
    pub next_dahan_lost: u32,
    /// Invaders the next turn's Ravage destroys by Dahan counterattack on the position this
    /// line leaves: where the Dahan stand is worth something (Thunderspeaker's whole game).
    #[serde(default)]
    pub next_invaders_destroyed: u32,
    #[serde(default)]
    pub next_built_towns: u32,
    #[serde(default)]
    pub next_built_cities: u32,
    /// The turn after next: its Ravage on what the next turn leaves, averaged over the
    /// next turn's possible Explore cards (fractional), counted at the weight squared.
    #[serde(default)]
    pub next2_blight: f64,
    #[serde(default)]
    pub next2_dahan_lost: f64,
    /// The Slow plays with the targets chosen after the Invader steps (`resolve_slow`).
    #[serde(default)]
    pub slow: Vec<Play>,
    /// Per Power applied, in order: the pieces it gathered in and pushed out, with the lands.
    #[serde(default)]
    pub moves: Vec<(String, Option<LandId>, Vec<Move>)>,
    /// Per Power applied, in order: what it destroyed where the player chooses, by land.
    #[serde(default)]
    pub kills: Vec<(String, Vec<(LandId, si_model::power::PieceFilter)>)>,
    /// Every land as the simulation left it (the live POSITION panel's `after` column).
    #[serde(default)]
    pub after: BTreeMap<LandId, Counts>,
    /// The Invaders win this turn: the loss condition, in words.
    #[serde(default)]
    pub loses: Option<String>,
    /// The Spirits win this turn: the Terror Level's victory, in words. The game stops there.
    #[serde(default)]
    pub wins: Option<String>,
    /// The Invaders win in the next turn's steps, with no further Spirit action.
    #[serde(default)]
    pub next_loses: Option<String>,
    /// The share of the next turn's possible Explore cards under which the turn after next's
    /// Ravage loses the game.
    #[serde(default)]
    pub next2_loses: f64,
    /// This turn's additions to the game-long loss counts: Beasts the adversary's rules
    /// destroyed (Russia), Blight off the card in 6+ overkill Ravages (Habsburg Monarchy).
    #[serde(default)]
    pub beasts_destroyed_by_rules: u32,
    #[serde(default)]
    pub irreparable_blight: u32,
    pub notes: Vec<String>,
}

impl Outcome {
    pub fn score(&self, v: &Values) -> f64 {
        // a win is the mirror of a loss, and ends the game: no later loss counts
        if self.wins.is_some() {
            return -v.loss + self.fear as f64 * v.fear;
        }
        let lost = if self.loses.is_some() { v.loss } else { 0.0 } + if self.next_loses.is_some() { v.next_weight * v.loss } else { 0.0 } + self.next2_loses * v.next_weight * v.next_weight * v.loss;
        lost + self.blight as f64 * v.blight
            + self.dahan_lost as f64 * v.dahan
            + self.presence_destroyed as f64 * v.presence
            + self.explorers_destroyed as f64 * v.explorer
            + self.towns_destroyed as f64 * v.town
            + self.cities_destroyed as f64 * v.city
            + self.invaders_pushed as f64 * v.explorer
            + self.fear as f64 * v.fear
            + self.presence_added as f64 * (v.slot - v.presence)
            - (self.built_towns as f64 * v.town + self.built_cities as f64 * v.city)
            + v.next_weight * (self.next_blight as f64 * v.blight + self.next_dahan_lost as f64 * v.dahan + self.next_invaders_destroyed as f64 * v.explorer - self.next_built_towns as f64 * v.town - self.next_built_cities as f64 * v.city)
            + v.next_weight * v.next_weight * (self.next2_blight * v.blight + self.next2_dahan_lost * v.dahan)
    }

    pub fn summary(&self) -> String {
        self.words(false)
    }

    /// The outcome in short tokens (`Bl2 Dh1 x1E 2T Fr3 next:Bl1`) or in words.
    pub fn words(&self, long: bool) -> String {
        let mut p = Vec::new();
        if let Some(r) = &self.wins {
            p.push(if long { format!("the Spirits win: {r}") } else { "WINS".to_string() });
        }
        if let Some(r) = &self.loses {
            p.push(if long { format!("the Invaders win: {r}") } else { "LOSES".to_string() });
        }
        if self.blight > 0 {
            p.push(if long { format!("Blight {}", self.blight) } else { format!("Bl{}", self.blight) });
        }
        if self.dahan_lost > 0 {
            p.push(if long { format!("Dahan lost {}", self.dahan_lost) } else { format!("Dh{}", self.dahan_lost) });
        }
        if self.presence_destroyed > 0 {
            p.push(if long { format!("presence lost {}", self.presence_destroyed) } else { format!("Pr{}", self.presence_destroyed) });
        }
        if self.presence_added > 0 {
            p.push(if long { format!("presence added {}", self.presence_added) } else { format!("Pr+{}", self.presence_added) });
        }
        let k: Vec<String> = [(self.explorers_destroyed, "E"), (self.towns_destroyed, "T"), (self.cities_destroyed, "C")].iter().filter(|(n, _)| *n > 0).map(|(n, k)| format!("{n}{k}")).collect();
        if !k.is_empty() {
            p.push(if long { format!("destroyed {}", k.join(" ")) } else { format!("kill {}", k.join(" ")) });
        }
        if self.invaders_pushed > 0 {
            p.push(if long { format!("pushed {}", self.invaders_pushed) } else { format!("push {}", self.invaders_pushed) });
        }
        if self.fear > 0 {
            p.push(if long { format!("Fear {}", self.fear) } else { format!("Fr{}", self.fear) });
        }
        let built = |t: u32, c: u32| -> String {
            let k: Vec<String> = [(t, "T"), (c, "C")].iter().filter(|(n, _)| *n > 0).map(|(n, k)| format!("{n}{k}")).collect();
            k.join(" ")
        };
        if self.built_towns + self.built_cities > 0 {
            p.push(if long { format!("built {}", built(self.built_towns, self.built_cities)) } else { format!("built {}", built(self.built_towns, self.built_cities)) });
        }
        if self.next_blight > 0 || self.next_dahan_lost > 0 || self.next_invaders_destroyed > 0 || self.next_built_towns + self.next_built_cities > 0 {
            let b = built(self.next_built_towns, self.next_built_cities);
            let k = self.next_invaders_destroyed;
            p.push(if long {
                format!("next turn: Blight {}{}{}{}", self.next_blight, if self.next_dahan_lost > 0 { format!(", Dahan {}", self.next_dahan_lost) } else { String::new() }, if k > 0 { format!(", Dahan destroy {k}") } else { String::new() }, if b.is_empty() { String::new() } else { format!(", built {b}") })
            } else {
                format!("| next{}{}{}{}{}", if self.next_loses.is_some() { " LOSES" } else { "" }, if self.next_blight > 0 { format!(" Bl{}", self.next_blight) } else { String::new() }, if self.next_dahan_lost > 0 { format!(" Dh{}", self.next_dahan_lost) } else { String::new() }, if k > 0 { format!(" kill {k}") } else { String::new() }, if b.is_empty() { String::new() } else { format!(" built {b}") })
            });
        } else if self.next_loses.is_some() && !long {
            p.push("| next LOSES".to_string());
        }
        if let (Some(r), true) = (&self.next_loses, long) {
            p.push(format!("next turn the Invaders win unless stopped: {r}"));
        }
        if self.next2_blight > 0.05 || self.next2_dahan_lost > 0.05 || self.next2_loses > 0.0 {
            p.push(if long {
                format!("turn after: Blight {:.1}{}{}", self.next2_blight, if self.next2_dahan_lost > 0.05 { format!(", Dahan {:.1}", self.next2_dahan_lost) } else { String::new() }, if self.next2_loses > 0.0 { format!(", the Invaders win under {:.0}% of the next Explore cards", self.next2_loses * 100.0) } else { String::new() })
            } else {
                format!("| then{}{}{}", if self.next2_loses > 0.0 { format!(" LOSES {:.0}%", self.next2_loses * 100.0) } else { String::new() }, if self.next2_blight > 0.05 { format!(" Bl{:.1}", self.next2_blight) } else { String::new() }, if self.next2_dahan_lost > 0.05 { format!(" Dh{:.1}", self.next2_dahan_lost) } else { String::new() })
            });
        }
        if p.is_empty() { "-".into() } else { p.join(if long { ", " } else { " " }) }
    }
}

/// One card played at a target (None: a Spirit-target card, or a Slow card
/// whose effect is not scored this turn).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    pub power: String,
    pub target: Option<LandId>,
    pub fast: bool,
    pub innate: bool,
    pub cost: i64,
    /// The target is where the Power places presence (Gift of Proliferation): written `GP2@A2`,
    /// as growth writes a placement, not `>` as a land target.
    #[serde(default)]
    pub places: bool,
    /// An innate printed Slow taken Fast through a "This Power may be Fast" tier its
    /// elements reach: written `F:LFA>A4`.
    #[serde(default)]
    pub forced: bool,
    /// The gathers and pushes the line asks of this Power (`GW>A1[<A3 <A5 >A4]`); the
    /// moves it made are in the outcome.
    #[serde(default)]
    pub moves: Vec<Move>,
    /// `moves` is everything the Power moved (the record's, or a bracket the player wrote):
    /// no gather or push beyond it, none at all when it is empty.
    #[serde(default)]
    pub exact: bool,
    /// Played for its elements only: where it lands changes nothing this turn (`(WW)`).
    /// Its effect is not applied; its elements count.
    #[serde(default)]
    pub inert: bool,
    /// The play as the journal has it (the actual line): its recorded speed and land stand
    /// as made, whatever a plan's checks would say.
    #[serde(default)]
    pub recorded: bool,
    /// The pieces it destroys where the player chooses, by land (`-T`, `-C3:Ex2` in its
    /// bracket); None leaves the choice to the simulation's best guess.
    #[serde(default)]
    pub kills: Option<Vec<(LandId, si_model::power::PieceFilter)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    /// The growth in notation tokens (`G2`, `e4@C4`, `cp2@C4`, `Rc`, `+<card id>`, `+1E`); empty when growth is not reviewed.
    #[serde(default)]
    pub growth: Vec<String>,
    /// This line starts from the growth actually taken.
    #[serde(default)]
    pub growth_actual: bool,
    /// The growth's own part of the score: slots, cards, Energy kept.
    #[serde(default)]
    pub growth_value: f64,
    /// Other lines with the same plays and the same score (different placements or slots) folded into this one.
    #[serde(default)]
    pub ties: u32,
    /// The score on what was knowable: averaged over the Explore cards still in the
    /// stage, with the Event and Fear effects left out and a card gained from a later
    /// draft not played. Lines rank by this.
    #[serde(default)]
    pub informed: f64,
    /// `score - informed`: what the hidden cards gave or took.
    #[serde(default)]
    pub luck: f64,
    /// Share of the possible Explore cards under which this line beats the actual one (None for the actual line, or when the Explore is known).
    #[serde(default)]
    pub confidence: Option<f64>,
    /// Actual line only: its hindsight score with the Slow targets chosen after the Invader
    /// steps minus the score with the targets as recorded. A Slow mis-target, not luck.
    #[serde(default)]
    pub slow_gap: f64,
    /// Actual line only: the Slow plays as the simulator would have targeted them.
    #[serde(default)]
    pub slow_best: Vec<Play>,
    /// Index of the growth position the line starts from (0 = as played).
    #[serde(default)]
    pub position: usize,
    /// The elements the line has this turn (track plus cards), as "Fire 3 Animal 4".
    #[serde(default)]
    pub elements: String,
    /// Each innate and the tier its elements reach: ("Epidemics Run Rampant", 2, 3) = tier 2 of 3; 0 = not reached.
    #[serde(default)]
    pub tiers: Vec<(String, u8, u8)>,
    /// What `informed` added for the face-down Event and Fear cards (`Calibration::expected_exogenous`).
    #[serde(default)]
    pub expected: f64,
    /// The informed score under each candidate Explore card, in `explore_candidates` order
    /// (empty for lines that were not ranked); the live `vs none` reads it.
    #[serde(default)]
    pub per_card: Vec<f64>,
    pub plays: Vec<Play>,
    pub outcome: Outcome,
    pub score: f64,
    pub actual: bool,
}

impl Line {
    /// The laws every scored line keeps (the corpus fuzzer and the live one hold every line
    /// to them): finite scores; a game never both won and lost in the outcome (a win and a
    /// loss together are a win, and the game stops at either); moves only along the
    /// island's edges, each of a piece or more, unless the Power's Move reaches farther
    /// ("to any land"); presence never from nowhere and none missing uncounted (`presence0`
    /// on the island as the line starts).
    pub fn holds(&self, defs: &Definitions, island: &Island, presence0: u32) -> Vec<si_model::oracle::Violation> {
        use si_model::oracle::check;
        let mut out = Vec::new();
        check(&mut out, self.score.is_finite() && self.informed.is_finite(), "finite scores", || format!("score {} informed {}", self.score, self.informed));
        check(&mut out, !(self.outcome.wins.is_some() && self.outcome.loses.is_some()), "won and lost", || format!("{:?} / {:?}", self.outcome.wins, self.outcome.loses));
        for (power, _, moves) in &self.outcome.moves {
            for m in moves {
                // a presence added from the island comes from within the Power's range; a
                // Beast (or Many Minds' Sacred Site moved as one) may go two lands
                let presence = m.piece == Some(si_model::power::PieceFilter::Presence);
                let beast = m.piece == Some(si_model::power::PieceFilter::Token(si_model::island::TokenKind::Beast));
                let far = verify::power_effects(defs, power).is_some_and(|(base, th)| effects::move_reaches(&base, m.piece.as_ref(), island, &m.from, &m.to) || th.iter().any(|(_, e)| effects::move_reaches(e, m.piece.as_ref(), island, &m.from, &m.to)));
                let adjacent = presence || far || island.lands.get(&m.from).is_some_and(|l| l.adjacent.contains(&m.to)) || (beast && effects::two_away(island, &m.from).contains(&m.to));
                check(&mut out, adjacent && m.n > 0, "move", || format!("{power} moved {} from {} to {}: not adjacent or nothing", m.n, m.from, m.to));
            }
        }
        let after: u32 = self.outcome.after.values().map(|c| c.presence).sum();
        if !self.outcome.after.is_empty() {
            let most = presence0 + self.outcome.presence_added;
            let least = most.saturating_sub(self.outcome.presence_destroyed);
            check(&mut out, after <= most, "presence", || format!("{after} presence at the turn's end, {presence0} at the start and {} added", self.outcome.presence_added));
            check(&mut out, after >= least, "presence", || format!("{after} presence at the turn's end, {presence0} at the start, {} added, {} destroyed: some went uncounted", self.outcome.presence_added, self.outcome.presence_destroyed));
        }
        out
    }
}

impl Line {
    /// The plays in notation as plain text, in the order they act, innates in braces:
    /// `SLSF>C5 {ERR>C2} FV {SR>C1}`.
    pub fn notation(&self, code: &dyn Fn(&str) -> String) -> String {
        self.plays_notation(code, true, true)
    }

    /// One play in notation: `WW>A7`, a placement `GP2@A2`, a Slow card or innate taking its
    /// land later `VT`. The phase is the printed speed; the screen colors Slow plays, and
    /// plain text marks a Power printed Slow that acts Fast `F:LFA>A4` (`marked`): an innate
    /// through its tier, or a Power another play makes Fast. The
    /// pieces it gathered in and pushed out follow in brackets: `GW>A1[<A3,<A5,>A4x2]`,
    /// `<A3` one piece in from A3, `>A4x2` two pieces out to A4. A bare move is a Dahan; any
    /// other kind is named after a colon, `>A4:T` a Town, `<A2x2:Be` two Beasts. `[]` is a
    /// move the line asked for that moved nothing.
    pub fn play_word(p: &Play, moves: &[Move], code: &dyn Fn(&str) -> String, marked: bool) -> String {
        Self::play_word_with(p, moves, p.kills.as_deref().unwrap_or(&[]), code, marked)
    }

    /// `play_word` with the pieces it destroyed where the player chose: `-T` in its land,
    /// `-C3:Ex2` two Explorers in C3, after the moves in the bracket; `-` alone, stated and
    /// none destroyed. Plain text only (`marked`): the screen's produced column says what died.
    pub fn play_word_with(p: &Play, moves: &[Move], kills: &[(LandId, si_model::power::PieceFilter)], code: &dyn Fn(&str) -> String, marked: bool) -> String {
        // played for its elements: no land, since none gains from it (plain text writes `(WW)`)
        if p.inert {
            return if marked { format!("({})", code(&p.power)) } else { code(&p.power) };
        }
        let name = if marked && p.forced { format!("F:{}", code(&p.power)) } else { code(&p.power) };
        let head = match &p.target {
            Some(l) if p.places => format!("{name}@{l}"),
            Some(l) => format!("{name}>{l}"),
            None => name,
        };
        let nowhere = LandId::new("?", 0);
        let target = match &p.target {
            Some(t) => t,
            // a Power on a Spirit that moved pieces: its moves as whole paths
            None if moves.iter().any(|m| m.n > 0) => &nowhere,
            None => return head,
        };
        // one word a path and kind, however many entries made it
        let mut merged: Vec<Move> = Vec::new();
        for m in moves {
            match merged.iter_mut().find(|x| x.from == m.from && x.to == m.to && x.piece == m.piece) {
                Some(x) => x.n += m.n,
                None => merged.push(m.clone()),
            }
        }
        let moves = &merged[..];
        let mut words: Vec<String> = Vec::new();
        let times = |w: String, m: &Move| -> String {
            let kind = m.piece.as_ref().filter(|k| **k != si_model::power::PieceFilter::Piece(PieceKind::Dahan)).and_then(effects::piece_code).map(|c| format!(":{c}")).unwrap_or_default();
            if m.n > 1 { format!("{w}x{}{kind}", m.n) } else { format!("{w}{kind}") }
        };
        for m in moves.iter().filter(|m| m.to == *target && m.from != *target) {
            words.push(times(format!("<{}", m.from), m));
        }
        for m in moves.iter().filter(|m| m.from == *target && m.to != *target) {
            words.push(times(format!(">{}", m.to), m));
        }
        for m in moves.iter().filter(|m| m.from != *target && m.to != *target) {
            words.push(times(format!("{}>{}", m.from, m.to), m));
        }
        // the pieces destroyed, by land and kind, the Power's land first
        let mut counted: Vec<(&LandId, &si_model::power::PieceFilter, u32)> = Vec::new();
        for (l, k) in kills.iter().filter(|(l, _)| l == target).chain(kills.iter().filter(|(l, _)| l != target)) {
            match counted.iter_mut().find(|(l2, k2, _)| *l2 == l && *k2 == k) {
                Some(c) => c.2 += 1,
                None => counted.push((l, k, 1)),
            }
        }
        // what died is plain text's (a typed line replays it exactly); on screen the
        // produced column says it ("kill 2E"), so the bracket keeps to the moves
        if !marked {
            counted.clear();
        }
        let none_killed = counted.is_empty();
        for (l, k, n) in counted {
            let Some(c) = effects::piece_code(k) else { continue };
            let at = if l == target { String::new() } else { format!("{l}:") };
            words.push(if n > 1 { format!("-{at}{c}x{n}") } else { format!("-{at}{c}") });
        }
        // stated, and nothing destroyed: `-` (its Damage kills nothing; "up to" took none)
        if none_killed && p.kills.is_some() && marked {
            words.push("-".into());
        }
        match words.is_empty() {
            // asked for, or the whole of what it moved, and nothing moved: the bracket keeps
            // the play from its default moves (on screen, only a move asked)
            true if (p.exact && marked) || !p.moves.is_empty() => format!("{head}[]"),
            true => head,
            false => format!("{head}[{}]", words.join(",")),
        }
    }

    /// What the `i`th play destroyed where the player chose (the outcome's), else what the
    /// line said it would.
    pub fn kills_of(&self, i: usize) -> Vec<(LandId, si_model::power::PieceFilter)> {
        let Some(p) = self.plays.get(i) else { return Vec::new() };
        let nth = self.plays[..i].iter().filter(|q| q.power == p.power).count();
        match self.outcome.kills.iter().filter(|(pw, _)| *pw == p.power).nth(nth) {
            Some((_, k)) => k.clone(),
            None => p.kills.clone().unwrap_or_default(),
        }
    }

    /// The moves the `i`th play made (the outcome's, in order of application), else the
    /// ones the line asked for.
    pub fn moves_of(&self, i: usize) -> Vec<Move> {
        let Some(p) = self.plays.get(i) else { return Vec::new() };
        let nth = self.plays[..i].iter().filter(|q| q.power == p.power).count();
        match self.outcome.moves.iter().filter(|(pw, _, _)| *pw == p.power).nth(nth) {
            Some((_, _, m)) => m.clone(),
            None => p.moves.clone(),
        }
    }

    fn words(&self, code: &dyn Fn(&str) -> String, innate: bool, marked: bool) -> Vec<String> {
        self.plays.iter().enumerate().filter(|(_, p)| p.innate == innate).map(|(i, p)| Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, marked)).collect()
    }

    /// The Power Cards played, each with its target (`EW GP2@A2 RBST>A2`); `(nothing)` when
    /// no card was played.
    pub fn cards_notation(&self, code: &dyn Fn(&str) -> String) -> String {
        let cards = self.words(code, false, false);
        if cards.is_empty() { "(nothing)".to_string() } else { cards.join(" ") }
    }

    /// The innates used, each with its target (`AEG>A2 CTM>A3`); empty when none.
    pub fn innates_notation(&self, code: &dyn Fn(&str) -> String) -> String {
        self.words(code, true, false).join(" ")
    }

    /// `innates_notation` for plain text, where no color says the phase: an innate taken
    /// Fast through its tier carries `F:`.
    pub fn innates_notation_marked(&self, code: &dyn Fn(&str) -> String) -> String {
        self.words(code, true, true).join(" ")
    }

    /// Every word of a Slow play (the bracket's words too), for the screen to color.
    pub fn slow_words(&self, code: &dyn Fn(&str) -> String) -> BTreeSet<String> {
        self.plays.iter().enumerate().filter(|(_, p)| !p.fast).flat_map(|(i, p)| Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, false).split(' ').map(String::from).collect::<Vec<_>>()).collect()
    }

    /// Every word of a Fast play (the bracket's words too), for the screen to color.
    pub fn fast_words(&self, code: &dyn Fn(&str) -> String) -> BTreeSet<String> {
        self.plays.iter().enumerate().filter(|(_, p)| p.fast).flat_map(|(i, p)| Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, false).split(' ').map(String::from).collect::<Vec<_>>()).collect()
    }

    /// Every word of a play made for its elements only, for the screen to dim.
    pub fn inert_words(&self, code: &dyn Fn(&str) -> String) -> BTreeSet<String> {
        self.plays.iter().enumerate().filter(|(_, p)| p.inert).map(|(i, p)| Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, false)).collect()
    }

    /// The plays made for their elements only, in words, for the line's notes.
    pub fn inert_note(&self, code: &dyn Fn(&str) -> String) -> Option<String> {
        let names: Vec<String> = self.plays.iter().filter(|p| p.inert).map(|p| code(&p.power)).collect();
        (!names.is_empty()).then(|| format!("{} played for {} elements: no land gains from {} this turn", names.join(", "), if names.len() == 1 { "its" } else { "their" }, if names.len() == 1 { "it" } else { "them" }))
    }

    /// Every word of an innate's play, for the screen to underline.
    pub fn innate_words(&self, code: &dyn Fn(&str) -> String) -> BTreeSet<String> {
        self.plays.iter().enumerate().filter(|(_, p)| p.innate).flat_map(|(i, p)| Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, false).split(' ').map(String::from).collect::<Vec<_>>()).collect()
    }

    /// The plays in the order they act: the Fast ones as listed (the recorded order for the
    /// actual line), then the Slow ones after the Invader steps, then the ones played for
    /// their elements only, which act nowhere.
    pub fn sequence(&self) -> Vec<usize> {
        let fast = self.plays.iter().enumerate().filter(|(_, p)| p.fast && !p.inert).map(|(i, _)| i);
        let slow = self.plays.iter().enumerate().filter(|(_, p)| !p.fast && !p.inert).map(|(i, _)| i);
        let inert = self.plays.iter().enumerate().filter(|(_, p)| p.inert).map(|(i, _)| i);
        fast.chain(slow).chain(inert).collect()
    }

    /// Every play in one sequence, cards and innates alike, in the order they act
    /// (`SA>A5[<A4] WW>A3 GW>A5[<A4x2,<A8x2,>A1x2] LFA>A1 VT MPG`); `(nothing)` when none.
    /// The screen underlines the innates; plain text (`braced`) writes them `{GW>A5}`.
    pub fn plays_notation(&self, code: &dyn Fn(&str) -> String, marked: bool, braced: bool) -> String {
        let words: Vec<String> = self
            .sequence()
            .into_iter()
            .map(|i| {
                let p = &self.plays[i];
                let w = Self::play_word_with(p, &self.moves_of(i), &self.kills_of(i), code, marked);
                if braced && p.innate { format!("{{{w}}}") } else { w }
            })
            .collect();
        if words.is_empty() { "(nothing)".to_string() } else { words.join(" ") }
    }

    /// The plays as a line to type back (`t`, a plan carried from a review): `F:` kept, a
    /// Fast play with its land, a Slow one bare (it takes its land after the Invader
    /// steps), the gathers and pushes in brackets where the land is known.
    /// The plays as a plan writes them, every land and move kept, Slow ones included: an
    /// earlier attempt's line carried onto the same position (the same cards will come).
    pub fn plays_text_kept(&self, code: &dyn Fn(&str) -> String) -> String {
        self.sequence().into_iter().map(|i| Self::play_word_with(&self.plays[i], &self.moves_of(i), &self.kills_of(i), code, true)).collect::<Vec<_>>().join(" ")
    }

    pub fn plays_text(&self, code: &dyn Fn(&str) -> String) -> String {
        self.plays
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let q = if p.fast || p.places { p.clone() } else { Play { target: None, kills: None, ..p.clone() } };
                Self::play_word_with(&q, &self.moves_of(i), &self.kills_of(i), code, true)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The elements and innate tiers in words: `Fire 3 Animal 4 → Epidemics Run Rampant tier 2 of 3`.
    pub fn elements_words(&self, code: &dyn Fn(&str) -> String) -> String {
        let tiers: Vec<String> = self
            .tiers
            .iter()
            .map(|(id, reached, of)| {
                let dots = format!("{reached}/{of}");
                format!("{}{dots}", code(id))
            })
            .collect();
        if tiers.is_empty() { self.elements.clone() } else { format!("{}  {}", self.elements, tiers.join(" ")) }
    }

    /// The growth in notation: `G2 e4@C4 cp2@C4 +HSAI`.
    pub fn growth_notation(&self, code: &dyn Fn(&str) -> String) -> String {
        growth_words(&self.growth, code)
    }
}

/// `Sun + Moon + 2 Fire + Animal` → `Su1 Mo1 Fi2 An1`: two letters per element and the count.
/// The eight elements in printed order, first letter and count, always all eight so the
/// column has one width: `S1 M0 F3 A1 W0 E1 P0 A2` (Sun Moon Fire Air Water Earth Plant Animal).
pub fn element_short(set: &ElementSet) -> String {
    let mut counts = [0u8; 8];
    for t in set.to_string().split(" + ").filter(|t| !t.trim().is_empty()) {
        let mut parts = t.trim().split(' ');
        let first = parts.next().unwrap_or("");
        let (n, name) = match first.parse::<u8>() {
            Ok(n) => (n, parts.next().unwrap_or("")),
            Err(_) => (1, first),
        };
        if let Some(i) = ELEMENT_ORDER.iter().position(|e| *e == name) {
            counts[i] = n;
        }
    }
    ELEMENT_ORDER.iter().zip(counts.iter()).map(|(name, n)| format!("{}{n}", &name[..1])).collect::<Vec<_>>().join(" ")
}

pub const ELEMENT_ORDER: [&str; 8] = ["Sun", "Moon", "Fire", "Air", "Water", "Earth", "Plant", "Animal"];

/// The legend, printed once per report.
/// Fill a line's elements and innate tiers from its plays and the position's track.
pub(crate) fn annotate_elements(line: &mut Line, defs: &Definitions, s2: &SpiritTurn) {
    let played: Vec<&PowerCard> = line.plays.iter().filter(|p| !p.innate).filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = elements_for(defs, s2, &played);
    line.elements = element_short(&elements);
    line.tiers = innates_of(defs, &s2.spirit)
        .iter()
        .map(|inn| {
            let acting: Vec<&si_model::spirit::Tier> = inn.tiers.iter().filter(|t| !t.modifier).collect();
            let reached = acting.iter().enumerate().filter(|(_, t)| elements.satisfies(&t.elements)).map(|(i, _)| i as u8 + 1).max().unwrap_or(0);
            (inn.id.clone(), reached, acting.len() as u8)
        })
        .collect();
}

pub fn growth_words(tokens: &[String], code: &dyn Fn(&str) -> String) -> String {
    tokens
        .iter()
        .map(|t| {
            if (t.starts_with('+') && !t.ends_with('E') && t != "+card") || (t.starts_with('-') && t.len() > 2) {
                format!("{}{}", &t[..1], code(&t[1..]))
            } else {
                t.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnReview {
    pub turn: u32,
    pub spirit: String,
    pub hand: Vec<String>,
    pub plays: i64,
    pub energy: i64,
    pub actual: Line,
    /// Best lines first (the actual line included where it ranks).
    pub ranked: Vec<Line>,
    pub candidates: usize,
    /// Score gap between the best line and the actual one.
    pub gap: f64,
    pub reading: String,
    /// What was hidden when the choice was made, replayed as logged.
    pub hidden: Vec<String>,
    pub notes: Vec<String>,
    /// Growth positions tried (the actual one first).
    #[serde(default)]
    pub positions: usize,
    /// The gap's part the plays account for (best line from the actual growth vs the actual line).
    #[serde(default)]
    pub gap_plays: f64,
    /// The gap's part the growth accounts for (best line overall vs best line from the actual growth).
    #[serde(default)]
    pub gap_growth: f64,
    /// The growth of the best line, in tokens.
    #[serde(default)]
    pub best_growth: Vec<String>,
    /// Best informed score minus the actual line's informed score: what the known information supported and was not taken.
    #[serde(default)]
    pub decision: f64,
    /// The actual line's hindsight score minus its informed score: what the hidden cards gave (+) or took (-).
    #[serde(default)]
    pub luck: f64,
    /// The Power Cards offered at this turn's gain.
    #[serde(default)]
    pub offered: Vec<String>,
    /// Event, Fear and other choices the player was asked for this turn, each against its alternatives.
    #[serde(default)]
    pub choices: Vec<ChoiceReview>,
    /// Ledger value of the Event and Fear piece effects as the log shows them, and how many lands they changed.
    #[serde(default)]
    pub exogenous: Option<(f64, usize)>,
    /// What the informed score added for the face-down Event and Fear cards.
    #[serde(default)]
    pub expected_exogenous: f64,
    /// What they did to the actual line, all told (see `Sample::hidden_effect`).
    #[serde(default)]
    pub hidden_effect: Option<f64>,
    #[serde(default)]
    pub sample: Sample,
    /// The Invader deck from this Fast phase: each land's chance of an Explore over the next turns.
    #[serde(default)]
    pub forecast: Vec<deck::TurnForecast>,
    /// Each loss condition in play as the turn began: its count against its limit.
    #[serde(default)]
    pub clocks: Vec<String>,
}

/// One recorded choice (an Event's or Fear card's land, a Power's "or"
/// branch, a cascade target) reviewed against what else was on offer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceReview {
    pub phase: Phase,
    pub source: String,
    pub source_kind: String,
    pub reason: String,
    pub selected: String,
    pub optional: bool,
    /// Each option with its score when the simulator can place the effect there; best first.
    pub options: Vec<(String, Option<f64>)>,
    pub actual_score: Option<f64>,
    /// Best option's score minus the chosen one's.
    pub gap: Option<f64>,
    pub note: String,
}

/// What a review enumerates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewOptions {
    /// Enumerate growth alternatives beside the actual growth.
    pub growth: bool,
    /// Growth positions tried per turn, the actual one included (taken round-robin over the growth groups).
    pub max_positions: usize,
    /// Play sets tried per position (first stage: every set without innates).
    pub max_lines_per_position: usize,
    /// Play sets per position that get their innates tried at every target (second stage: the best of the first).
    pub innate_beam: usize,
    /// Innate target combinations tried per play set.
    pub max_innate_options: usize,
    /// Lines kept per position after scoring.
    pub keep_per_position: usize,
    /// Lines (best by hindsight, plus the actual) that get the informed score and are ranked by it.
    pub informed_lines: usize,
    /// The measured constants; None takes the built-in ones.
    pub calibration: Option<Calibration>,
    /// Score the actual line only (no alternatives): what `si calibrate` needs.
    pub actual_only: bool,
    /// The Blight Card at each turn's start (Blight left, flipped), from the capture's saves;
    /// a turn missing here scores no Blight Card loss.
    pub blight_pool: BTreeMap<u32, (i64, bool)>,
    /// Score a lost game at the ledger's loss value. Off for `si calibrate`'s constants, which
    /// measure what the hidden cards and the simulator do to pieces: a loss is a step of 25
    /// that would swamp a turn's usual swing.
    pub score_losses: bool,
}

impl Default for ReviewOptions {
    fn default() -> Self {
        ReviewOptions { growth: true, max_positions: 100, max_lines_per_position: 3000, innate_beam: 40, max_innate_options: 150, keep_per_position: 300, informed_lines: 30, calibration: None, actual_only: false, blight_pool: BTreeMap::new(), score_losses: true }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Review {
    pub values: Values,
    pub turns: Vec<TurnReview>,
    pub notes: Vec<String>,
    #[serde(default)]
    pub calibration: Calibration,
    pub deck: deck::Deck,
}

fn innates_of<'d>(defs: &'d Definitions, spirit: &str) -> Vec<&'d si_model::spirit::Innate> {
    defs.spirits.get(spirit).map(|p| p.innates.iter().collect()).unwrap_or_default()
}

/// Is the source a Spirit Power (a card or an innate), by title (the engine's status
/// effects) or by id (the record's touches)?
fn is_power_source(defs: &Definitions, source: &str) -> bool {
    defs.cards.contains_key(source) || defs.card_by_title(source).is_some() || defs.spirits.values().flat_map(|s| s.innates.iter()).any(|i| i.title == source || i.id == source)
}

/// The Invader steps of the turn, run on the island as it stands after the
/// candidate's Fast plays. Event and Fear effects the engine applied are
/// taken from the log as status effects (Spirit-sourced ones excluded, since
/// they depend on the candidate).
/// `explore`: the Explore card to run instead of the one that came (the informed
/// score averages over the cards still in the stage). `exogenous`: apply what the
/// Event and Fear cards did (status effects, Defend) as the log shows; off for the
/// informed score, since they were face down when the choice was made.
pub(crate) fn invader_phase(island: &mut Island, rec: &GameRecord, t: &TurnRecord, defs: &Definitions, base_rules: &Rules, out: &mut Outcome, explore: Option<&[String]>, exogenous: bool) {
    let mut rules = base_rules.clone();
    let mut filtered = t.clone();
    filtered.status_effects.retain(|se| exogenous && !is_power_source(defs, &se.source));
    let mut notes = Vec::new();
    verify::apply_status_effects(island, &mut rules, &filtered, &mut notes);
    // Defend the engine logged from Fear and Event cards (not from Powers, which the candidate re-derives)
    if exogenous {
        for l in rec.lands.values() {
            let exo: u32 = l.touches.iter().filter(|x| x.turn == t.turn && !is_power_source(defs, &x.by)).filter_map(|x| if let si_record::TouchKind::Defend(n) = x.kind { Some(n) } else { None }).sum();
            if exo > 0 {
                if let Some(land) = island.lands.get_mut(&l.land) {
                    land.defend += exo;
                }
            }
        }
    }
    // a loss condition, at the end of the Fast phase and after every step
    let tally = |out: &Outcome| invaders::Tally { blight: out.blight, beasts_destroyed: out.beasts_destroyed_by_rules, irreparable: out.irreparable_blight };
    // the game ends as soon as it is won or lost (a win and a loss together are a win, the
    // rulebook's sacrifice victory); it stops there
    if out.wins.is_some() || out.loses.is_some() {
        return;
    }
    match invaders::ending(island, &rules, tally(out), true) {
        Some(Ok(w)) => {
            out.wins = Some(w);
            return;
        }
        Some(Err(l)) => {
            out.loses = Some(l);
            return;
        }
        None => {}
    }
    let explore_cards: Vec<String> = explore.map(|e| e.to_vec()).unwrap_or_else(|| t.explore.clone());
    let steps: Vec<(&Vec<String>, si_model::event::InvaderStep)> = vec![(&t.high_immigration, si_model::event::InvaderStep::Build), (&t.ravage, si_model::event::InvaderStep::Ravage), (&t.build, si_model::event::InvaderStep::Build), (&explore_cards, si_model::event::InvaderStep::Explore)];
    // the boards the Ravage Step added Blight on (Russia 6 acts on the others)
    let mut blighted: BTreeSet<String> = BTreeSet::new();
    for (i, (cards, step)) in steps.into_iter().enumerate() {
        // each card's Action, then what the adversary does after the step: after the Ravage
        // (Russia 6, Scotland 6), after the Build (Habsburg Mining's Mining Boom, the Habsburg
        // Monarchy's Migratory Herders), Habsburg Mining's Mining Tunnels once a Stage II card
        // has explored and the cards advanced (the Ravage and Build spaces then hold this
        // turn's Build and Explore cards)
        let mut actions: Vec<Option<&String>> = cards.iter().map(Some).collect();
        // England 6, Independent Resolve: no Fear Card resolved, the High Immigration Build twice
        if i == 0 && rules.england_resolves_twice() {
            actions.extend(cards.iter().map(Some));
        }
        if i == 1 || i == 2 || (i == 3 && cards.iter().any(|c| defs.invader_cards.get(c).is_some_and(|c| c.escalation))) {
            actions.push(None);
        }
        for card_id in actions {
            let outcomes = match (card_id, step) {
                (Some(c), _) => {
                    let Some(card) = defs.invader_cards.get(c) else { continue };
                    match step {
                        si_model::event::InvaderStep::Ravage => invaders::ravage(island, card, &rules),
                        si_model::event::InvaderStep::Build => invaders::build(island, card, &rules),
                        si_model::event::InvaderStep::Explore => invaders::explore(island, card, &rules),
                    }
                }
                (None, si_model::event::InvaderStep::Build) => {
                    let build: Vec<&si_model::decks::InvaderCard> = t.build.iter().filter_map(|c| defs.invader_cards.get(c)).collect();
                    invaders::after_build(island, &rules, &build)
                }
                (None, si_model::event::InvaderStep::Ravage) => {
                    let ravage: Vec<&si_model::decks::InvaderCard> = t.ravage.iter().filter_map(|c| defs.invader_cards.get(c)).collect();
                    invaders::after_ravage(island, &rules, &ravage, &blighted)
                }
                (None, _) => {
                    let next: Vec<&si_model::decks::InvaderCard> = t.build.iter().chain(explore_cards.iter()).filter_map(|c| defs.invader_cards.get(c)).collect();
                    invaders::mining_tunnels(island, &rules, &next)
                }
            };
            for o in outcomes {
                if step == si_model::event::InvaderStep::Ravage && o.blight + o.cascade > 0 {
                    blighted.insert(o.land.board.clone());
                }
                out.blight += o.blight + o.cascade;
                out.dahan_lost += o.dahan_lost;
                out.presence_destroyed += o.presence_destroyed;
                if step == si_model::event::InvaderStep::Build {
                    out.built_towns += o.added.iter().filter(|k| **k == PieceKind::Town).count() as u32;
                    out.built_cities += o.added.iter().filter(|k| **k == PieceKind::City).count() as u32;
                }
                // counterattack kills, by kind, with the Fear a Town or City gives
                count_kills(out, &o.destroyed);
                out.fear += o.fear;
                out.beasts_destroyed_by_rules += o.beasts_destroyed;
                out.irreparable_blight += o.irreparable;
                out.notes.extend(o.notes.into_iter().filter(|n| !n.starts_with("counterattack") && !n.starts_with("cascade")));
            }
            match invaders::ending(island, &rules, tally(out), false) {
                Some(Ok(w)) => {
                    out.wins = Some(w);
                    return;
                }
                Some(Err(l)) => {
                    out.loses = Some(l);
                    return;
                }
                None => {}
            }
        }
    }
}

pub(crate) fn apply_plays(island: &mut Island, defs: &Definitions, plays: &[Play], fast: bool, elements: &ElementSet, rules: &Rules, spirit: &str, t: &TurnRecord, armed: &mut Vec<effects::TriggerArmed>, out: &mut Outcome) {
    for p in plays.iter().filter(|p| p.fast == fast && !p.inert) {
        // a Spirit-target card with no land acts in the Spirit's lands (an Area over its
        // presence): anchored on one of them, the Area reaches the rest
        // (one whose line names its moves acts where they start: Boon of Swarming
        // Bedevilment's presence pushed from C4)
        let anchor: Option<LandId> = p.target.clone().or_else(|| defs.cards.get(&p.power).filter(|c| c.target.is_spirit()).and_then(|_| p.moves.first().filter(|_| !p.places).map(|m| m.from.clone()).or_else(|| island.lands.values().find(|l| l.c.presence > 0).map(|l| l.id.clone()))));
        let Some(target) = &anchor else { continue };
        // an innate the line placed where presence may walk in with the Dahan acts only if
        // it did (the enumerator offers those lands for a spirit whose presence moves so)
        if p.innate && !p.recorded && rules.presence_with_dahan_for(spirit) {
            let range = defs.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == p.power).and_then(|i| i.range.clone());
            if let Some(r) = range {
                if r.origin_terrains.is_empty() && !in_range_now(island, target, &r) {
                    out.notes.push(format!("{} on {target}: out of range of the presence when used", p.power));
                    out.moves.push((p.power.clone(), Some(target.clone()), Vec::new()));
                    out.kills.push((p.power.clone(), Vec::new()));
                    continue;
                }
            }
        }
        let Some((base, thresholds)) = verify::power_effects(defs, &p.power) else { continue };
        // where the player put this Power's spread pieces and which piece or branch it picked, when the log recorded it
        let ctx = Ctx { health_bonus: rules.invader_health_bonus(), elements: elements.clone(), blight_as_badlands: rules.blight_as_badlands_for(spirit), rules: Some(rules.clone()), placements: verify::placements_of(defs, t, &p.power), picks: verify::picks_of(defs, t, &p.power), moves: p.moves.clone(), moves_exact: p.exact, branches: verify::branches_of(defs, t, &p.power), presence_with_dahan: rules.presence_with_dahan_for(spirit), kills: p.kills.clone(), site_beast: rules.site_beast_for(spirit), beasts_far: rules.beasts_far_for(spirit), collapse: rules.collapse_for(spirit) };
        // Shroud's Mists Shift and Flow: 1 presence gathered toward the land first. The record's
        // choice for this Power this turn where it fits (the land it gathered into, when that
        // is the target or beside it; none when the player declined), else into the target
        if rules.gathers_presence_on_targeting(spirit) {
            let near = |l: &LandId| l == target || island.lands.get(target).is_some_and(|x| x.adjacent.contains(l));
            let into: Option<LandId> = match verify::presence_gather_of(defs, t, &p.power) {
                Some(Some(l)) if near(&l) => Some(l),
                Some(None) => None,
                _ => Some(target.clone()),
            };
            if let Some(into) = into {
                effects::gather_presence(island, &into);
            }
        }
        // triggers waiting on "a Power targeting a land" fire on this play's land
        let mut fired: Vec<usize> = Vec::new();
        for (i, tr) in armed.iter().enumerate() {
            if tr.when.to_ascii_lowercase().contains("uses a power targeting a land") {
                let r = effects::apply(&tr.then, &[], island, target, &ctx);
                out.notes.push(format!("trigger fired on {target}: {}", tr.then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")));
                fold_applied(out, &r);
                if tr.once {
                    fired.push(i);
                }
            }
        }
        for i in fired.into_iter().rev() {
            armed.remove(i);
        }
        let a = effects::apply(&base, &thresholds, island, target, &ctx);
        // the skips and Isolation the Power grants its land (Strike Low), as the engine would
        // record them, once it has acted (Melt Earth into Quicksand's Damage comes before its
        // Isolate: an Explorer Russia saves may still flee)
        if let Some(l) = island.lands.get_mut(target) {
            for e in base.iter().chain(thresholds.iter().filter(|(need, _)| elements.satisfies(need)).flat_map(|(_, es)| es.iter())) {
                verify::flag(l, e);
            }
        }
        let destroyed_invader = a.destroyed.iter().any(|k| *k != PieceKind::Dahan);
        // every play applied has its entry, empty when it moved nothing, so the line shows
        // what happened and never the request (`<A3x9` is "all", asked)
        out.moves.push((p.power.clone(), Some(target.clone()), a.moves.clone()));
        out.kills.push((p.power.clone(), a.killed.clone()));
        fold_applied(out, &a);
        // triggers waiting on an Invader destroyed in their land
        let mut fired: Vec<usize> = Vec::new();
        for (i, tr) in armed.iter().enumerate() {
            if destroyed_invader && tr.land == *target && tr.when.to_ascii_lowercase().contains("invader is destroyed") {
                let r = effects::apply(&tr.then, &[], island, target, &ctx);
                out.notes.push(format!("trigger fired on {target}: {}", tr.then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")));
                fold_applied(out, &r);
                if tr.once {
                    fired.push(i);
                }
            }
        }
        for i in fired.into_iter().rev() {
            armed.remove(i);
        }
        armed.extend(a.triggers.iter().cloned());
    }
}

/// Invaders destroyed, by kind, with the Fear a Town (1) or a City (2) gives.
fn count_kills(out: &mut Outcome, kinds: &[PieceKind]) {
    for k in kinds {
        match k {
            PieceKind::Explorer => out.explorers_destroyed += 1,
            PieceKind::Town => {
                out.towns_destroyed += 1;
                out.fear += 1;
            }
            PieceKind::City => {
                out.cities_destroyed += 1;
                out.fear += 2;
            }
            PieceKind::Dahan => {}
        }
    }
}

/// Add what a Power did to the line's outcome: Fear, pushes, pieces destroyed
/// (with the Fear a Town or City gives), pieces removed, forced Ravages.
fn fold_applied(out: &mut Outcome, a: &effects::Applied) {
    out.fear += a.fear;
    out.presence_added += a.presence_added;
    out.presence_destroyed += a.presence_destroyed;
    out.invaders_pushed += a.pushed_pieces.iter().filter(|k| k.is_invader()).count() as u32;
    for r in &a.ravaged {
        out.blight += r.blight + r.cascade;
        out.dahan_lost += r.dahan_lost;
        out.presence_destroyed += r.presence_destroyed;
        count_kills(out, &r.destroyed);
    }
    for k in &a.removed {
        match k {
            PieceKind::Explorer => out.explorers_destroyed += 1,
            PieceKind::Town => out.towns_destroyed += 1,
            PieceKind::City => out.cities_destroyed += 1,
            PieceKind::Dahan => {}
        }
    }
    for k in &a.destroyed {
        match k {
            PieceKind::Explorer => out.explorers_destroyed += 1,
            PieceKind::Town => {
                out.towns_destroyed += 1;
                out.fear += 1;
            }
            PieceKind::City => {
                out.cities_destroyed += 1;
                out.fear += 2;
            }
            PieceKind::Dahan => {}
        }
    }
    out.notes.extend(a.notes.iter().filter(|n| n.starts_with("unparsed") || n.starts_with("condition") || n.starts_with("not simulated")).cloned());
}

/// `informed_build`: for the informed score, the next turn's Build is this turn's
/// assumed Explore card, its Explore is unknown (not run) and its Event and Fear
/// effects are not applied; `None` runs the next turn as it came.
pub(crate) fn lookahead(island: &mut Island, rec: &GameRecord, next: Option<&TurnRecord>, defs: &Definitions, rules: &Rules, out: &mut Outcome, informed_build: Option<&[String]>, deep: Option<&Deep>) {
    let Some(n) = next else { return };
    // this turn's Defend, skips and Words of Warning end before the next turn's Invaders act
    island.time_passes();
    // the game-long loss counts as this line leaves them
    let rules = &clocks_after(rules, out);
    let mut probe = Outcome::default();
    match informed_build {
        Some(build) => {
            let mut n2 = n.clone();
            n2.build = build.to_vec();
            n2.explore.clear();
            invader_phase(island, rec, &n2, defs, rules, &mut probe, None, false);
        }
        None => invader_phase(island, rec, n, defs, rules, &mut probe, None, true),
    }
    let rules = &clocks_after(rules, &probe);
    out.next_blight = probe.blight;
    out.next_dahan_lost = probe.dahan_lost;
    out.next_loses = probe.loses.clone();
    // the counterattack's kills land in `explorers_destroyed` (cheapest first)
    out.next_invaders_destroyed = probe.explorers_destroyed + probe.towns_destroyed + probe.cities_destroyed;
    out.next_built_towns = probe.built_towns;
    out.next_built_cities = probe.built_cities;
    // one turn deeper: the turn after next's Ravage (its card is this turn's Explore) on
    // what the next turn leaves, averaged over the next turn's possible Explore cards
    let Some(d) = deep else { return };
    if d.ravage.is_empty() {
        return;
    }
    let after = TurnRecord { turn: n.turn + 1, ravage: d.ravage.clone(), high_immigration: d.high_immigration.clone(), ..Default::default() };
    let run = |isl: &mut Island| -> (u32, u32, bool) {
        isl.time_passes();
        let mut p = Outcome::default();
        invader_phase(isl, rec, &after, defs, rules, &mut p, Some(&[]), false);
        (p.blight, p.dahan_lost, p.loses.is_some())
    };
    if d.pool.is_empty() {
        let mut isl = island.clone();
        let (b, dh, lost) = run(&mut isl);
        out.next2_blight = b as f64;
        out.next2_dahan_lost = dh as f64;
        out.next2_loses = if lost { 1.0 } else { 0.0 };
    } else {
        let (mut b, mut dh, mut lost) = (0.0, 0.0, 0.0);
        for card in &d.pool {
            let mut isl = island.clone();
            let explore = TurnRecord { turn: n.turn, explore: vec![card.clone()], ..Default::default() };
            let mut p = Outcome::default();
            invader_phase(&mut isl, rec, &explore, defs, rules, &mut p, None, false);
            let (b1, d1, l1) = run(&mut isl);
            b += b1 as f64;
            dh += d1 as f64;
            if l1 || p.loses.is_some() {
                lost += 1.0;
            }
        }
        let k = d.pool.len() as f64;
        out.next2_blight = b / k;
        out.next2_dahan_lost = dh / k;
        out.next2_loses = lost / k;
    }
}

/// The loss counts carried past a turn: the Beasts and irreparable Blight it added, the Blight
/// it took off the card (a card it flips has an unseen pool, so that one reads unknown).
pub(crate) fn clocks_after(rules: &Rules, out: &Outcome) -> Rules {
    let mut r = rules.clone();
    // the record's cascade choices and Ravage order are this turn's
    r.cascades.clear();
    r.ravage_order.clear();
    r.clocks.beasts_destroyed += out.beasts_destroyed_by_rules;
    r.clocks.irreparable += out.irreparable_blight;
    r.clocks.blight_pool = match r.clocks.blight_pool {
        Some((p, true)) => Some((p - out.blight as i64, true)),
        Some((p, false)) if (out.blight as i64) < p => Some((p - out.blight as i64, false)),
        _ => None,
    };
    r
}

/// The Terror Level as the turn's Invader steps began: the record's, else the level the
/// turn before ended at.
pub fn terror_at_invaders(rec: &GameRecord, t: &TurnRecord) -> u8 {
    if t.terror_at_invaders > 0 {
        return t.terror_at_invaders;
    }
    rec.turns.iter().filter(|u| u.turn < t.turn).map(|u| u.terror_after).max().unwrap_or(1).max(1)
}

/// The Spirits' victory after the Slow plays, at the Terror Level the turn ended at (Terror
/// Level III reached in the Slow phase wins there when no City is left).
pub(crate) fn victory_after_slow(island: &Island, rules: &Rules, t: &TurnRecord, out: &mut Outcome) {
    if out.wins.is_some() || out.loses.is_some() {
        return;
    }
    let mut r = rules.clone();
    r.terror = r.terror.max(t.terror_after);
    out.wins = invaders::victory(island, &r);
}

/// The order the player ravaged the matching lands in, as the turn's choices have it.
pub fn ravage_order_of(t: &TurnRecord) -> Vec<LandId> {
    t.choices.iter().filter(|c| c.reason == "RavageLand").filter_map(|c| c.selected.parse::<LandId>().ok()).collect()
}

/// Where the player sent each cascading Blight in the turn: the lands offered and the one chosen.
pub fn cascades_of(t: &TurnRecord) -> Vec<(Vec<LandId>, LandId)> {
    t.choices
        .iter()
        .filter(|c| c.reason == "CascadingBlight")
        .filter_map(|c| Some((c.choices.iter().filter_map(|x| x.parse::<LandId>().ok()).collect(), c.selected.parse::<LandId>().ok()?)))
        .collect()
}

/// The loss conditions' game-long counts as turn `turn` begins: Russia's Beasts destroyed by
/// the Hunters (Beasts lost between the first Invader step and the Slow phase, where only the
/// adversary's rules remove them), the Habsburg Monarchy's Blight off the card in Ravages of
/// 6+ overkill, the players, and the Blight Card as the save holds it.
pub fn loss_clocks(rec: &GameRecord, turn: u32, pool: Option<(i64, bool)>) -> invaders::LossClocks {
    let mut c = invaders::LossClocks { players: rec.meta.spirits.len().max(1) as u32, blight_pool: pool, ..Default::default() };
    let adversary = rec.meta.adversary.as_deref();
    if adversary == Some("TheTsardomOfRussia") {
        // the save journal names each Beast the Hunters take to the adversary's panel
        c.beasts_destroyed = rec.turns.iter().filter(|t| t.turn < turn).map(|t| t.beasts_to_adversary).sum();
    }
    if adversary == Some("TheHabsburgMonarchy") {
        for l in rec.lands.values() {
            for v in l.visits.iter().filter(|v| v.turn < turn && v.step == Some(si_model::event::InvaderStep::Ravage) && v.damage >= 8) {
                c.irreparable += v.blight + v.cascade;
            }
        }
    }
    c
}

/// The Explore cards that could have come this turn, as far as was knowable: every
/// card of the stage the deck order put next that had not been drawn yet. On an open
/// turn (no Explore recorded) the stage comes from the deck order.
pub fn explore_candidates(rec: &GameRecord, defs: &Definitions, deck: &deck::Deck, t: &TurnRecord) -> Vec<Vec<String>> {
    let Some(first) = t.explore.first() else {
        return deck::pool(rec, defs, deck, t, t.turn).map(|(_, cards)| cards.into_iter().map(|c| vec![c]).collect()).unwrap_or_default();
    };
    let Some(card) = defs.invader_cards.get(first) else { return vec![] };
    let mut drawn: BTreeSet<String> = BTreeSet::new();
    for u in rec.turns.iter().filter(|u| u.turn < t.turn) {
        drawn.extend(u.explore.iter().cloned());
        drawn.extend(u.build.iter().cloned());
        drawn.extend(u.ravage.iter().cloned());
        drawn.extend(u.high_immigration.iter().cloned());
    }
    drawn.extend(t.build.iter().cloned());
    drawn.extend(t.ravage.iter().cloned());
    drawn.extend(t.high_immigration.iter().cloned());
    let mut v: Vec<Vec<String>> = defs.invader_cards.iter().filter(|(id, c)| c.stage == card.stage && !drawn.contains(*id)).map(|(id, _)| vec![id.clone()]).collect();
    if !v.iter().any(|c| c[0] == *first) {
        v.push(vec![first.clone()]);
    }
    v.sort();
    v
}

/// Energy the spirit could spend this turn: last turn's bank plus this
/// turn's income and the growth's printed Energy. The log's own gained and
/// paid amounts are an upper bound (a bare log re-logs them after an undo).
/// The Energy the chosen growth groups yield (by group number), with the always-group's.
fn growth_energy_of(panel: &si_model::spirit::SpiritPanel, groups: &[u8]) -> i64 {
    let chosen: i64 = groups.iter().filter_map(|n| panel.growth.groups.iter().find(|g| g.number == *n)).map(|g| g.yield_().energy).sum();
    let always: i64 = if groups.is_empty() { 0 } else { panel.growth.always.as_ref().map(|a| a.yield_().energy).unwrap_or(0) };
    chosen + always
}

pub(crate) fn energy_available(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, prev_after: i64) -> i64 {
    let _ = rec;
    let growth_energy: i64 = defs.spirits.get(&s.spirit).map(|p| growth_energy_of(p, &s.growth_groups)).unwrap_or(0);
    let from_track = prev_after + s.income.unwrap_or(0) + growth_energy;
    let from_log = s.energy_after + s.energy_paid;
    from_track.min(from_log).max(s.energy_after)
}

/// An innate printed Slow has a "This Power may be Fast" tier these elements reach.
pub fn may_be_fast(inn: &si_model::spirit::Innate, elements: &ElementSet) -> bool {
    inn.speed != Speed::Fast && inn.tiers.iter().any(|t| t.modifier && t.body.to_ascii_lowercase().contains("fast") && elements.satisfies(&t.elements))
}

/// Why a line's `F:` innate cannot be Fast with these elements, if it cannot.
pub fn forced_speed_error(defs: &Definitions, spirit: &str, plays: &[Play], elements: &ElementSet) -> Option<String> {
    // the Powers in the line that let Slow ones act Fast (Slip the Flow of Time), and the
    // spirit's own rule (Swiftness of Lightning: 1 per Air): how many
    let by_rule: u32 = defs.spirits.get(spirit).map(|p| p.special_rules.iter().flat_map(|r| r.hooks.iter()).map(|h| match h {
        si_model::spirit::RuleHook::SlowAsFastPer(e) => elements.count(*e) as u32,
        _ => 0,
    }).sum()).unwrap_or(0);
    let mut granted: u32 = by_rule + plays.iter().map(|p| speed_grants(defs, &p.power, elements)).sum::<u32>();
    for p in plays.iter().filter(|p| p.forced && !p.recorded) {
        let inn = defs.spirits.values().flat_map(|sp| sp.innates.iter()).find(|i| i.id == p.power);
        if inn.is_some_and(|i| may_be_fast(i, elements)) {
            continue;
        }
        if granted > 0 {
            granted -= 1;
            continue;
        }
        return Some(match inn {
            Some(inn) => match inn.tiers.iter().find(|t| t.modifier && t.body.to_ascii_lowercase().contains("fast")) {
                Some(t) => format!("{} is Slow as printed: its {} tier takes it Fast, and this line has {}", inn.title, t.elements, elements),
                None => format!("{} is Slow as printed, has no tier that takes it Fast, and nothing in this line makes a Power Fast", inn.title),
            },
            None => format!("{} is Slow as printed and nothing in this line makes a Power Fast", defs.cards.get(&p.power).map(|c| c.title.as_str()).unwrap_or(&p.power)),
        });
    }
    None
}

/// How many Slow Powers a play lets act Fast, its tiers as the elements reach them.
fn speed_grants(defs: &Definitions, power: &str, elements: &ElementSet) -> u32 {
    fn count(es: &[Effect]) -> u32 {
        es.iter().map(|e| match e {
            Effect::MakeFast(k) => *k as u32,
            Effect::Conditional { then, .. } | Effect::Per { then, .. } => count(then),
            Effect::Choice(bs) => bs.iter().map(|b| count(b)).max().unwrap_or(0),
            _ => 0,
        }).sum()
    }
    let Some((base, th)) = verify::power_effects(defs, power) else { return 0 };
    count(&base) + th.iter().filter(|(els, _)| elements.satisfies(els)).map(|(_, es)| count(es)).sum::<u32>()
}

/// Does the Power damage or destroy pieces where the player chooses which (its bracket says
/// what died)?
pub(crate) fn destroys_pieces(defs: &Definitions, power: &str) -> bool {
    fn any(es: &[Effect]) -> bool {
        es.iter().any(|e| match e {
            Effect::Damage { to, .. } => !matches!(to, DamageTarget::Dahan | DamageTarget::EachDahan),
            // a token it removes is the player's pick too ("Remove up to half of {Beast}")
            Effect::Destroy { .. } | Effect::RemoveToken { .. } => true,
            Effect::Per { then, .. } | Effect::Conditional { then, .. } | Effect::Area { then, .. } => any(then),
            Effect::Choice(b) => b.iter().any(|x| any(x)),
            _ => false,
        })
    }
    verify::power_effects(defs, power).is_some_and(|(base, th)| any(&base) || th.iter().any(|(_, es)| any(es)))
}

/// Does the Power gather or push pieces (its bracket says what moved)?
pub(crate) fn moves_pieces(defs: &Definitions, power: &str) -> bool {
    fn any(es: &[Effect]) -> bool {
        es.iter().any(|e| match e {
            Effect::Push { .. } | Effect::Gather { .. } => true,
            Effect::Per { then, .. } | Effect::Conditional { then, .. } | Effect::Area { then, .. } => any(then),
            Effect::Choice(b) => b.iter().any(|x| any(x)),
            _ => false,
        })
    }
    verify::power_effects(defs, power).is_some_and(|(base, th)| any(&base) || th.iter().any(|(_, es)| any(es)))
}

pub(crate) fn elements_for(defs: &Definitions, s: &SpiritTurn, played: &[&PowerCard]) -> ElementSet {
    let mut set = defs.spirits.get(&s.spirit).map(|p| p.track_state(&s.revealed_after).elements).unwrap_or_default();
    // a card's Elements once, however often the line uses it (a Repeat gives none)
    for (i, c) in played.iter().enumerate() {
        if !played[..i].iter().any(|d| d.id == c.id) {
            set.extend(&c.elements);
        }
    }
    // gained beyond the track and the cards: a growth option, a wildcard's choice, and a
    // Power Card's grant when that card is in the line
    for (source, els) in &s.elements_gained {
        if defs.cards.contains_key(source) && !played.iter().any(|c| c.id == *source) {
            continue;
        }
        for e in els.iter().filter_map(|e| si_model::element::Element::parse(e)) {
            set.add(e, 1);
        }
    }
    set
}

/// The spirit's turn as a growth position leaves it: its presence and track, and the
/// elements a growth option gained only when it is the growth taken.
pub(crate) fn spirit_at(s: &SpiritTurn, pos: &Position) -> SpiritTurn {
    let mut s2 = s.clone();
    s2.presence_after = pos.presence.clone();
    s2.revealed_after = pos.revealed.clone();
    if !pos.actual {
        s2.elements_gained.retain(|(source, _)| !source.contains("/GainElements"));
    }
    s2
}

/// A card whose effect places presence (Gift of Proliferation, boons of growth).
fn adds_presence(c: &PowerCard) -> bool {
    c.all_effects().any(|e| matches!(e, Effect::AddPresence { .. }))
}

/// A Spirit-target card whose effect lands in one of the target Spirit's lands (Fiery
/// Vengeance: "1 Fear and 1 Damage in one of target Spirit's lands"): the player picks
/// the land, so the line names it (`FV>C5`). Not a card whose effects are all on the
/// Spirit's economy, nor one that acts in every presence land at once (an Area).
fn acts_in_one_land(c: &PowerCard) -> bool {
    if !c.target.is_spirit() || adds_presence(c) {
        return false;
    }
    let land_effect = |e: &Effect| {
        matches!(
            e,
            Effect::Damage { .. }
                | Effect::Defend(_)
                | Effect::Push { .. }
                | Effect::Gather { .. }
                | Effect::FearIfInvaders(_)
                | Effect::RemoveBlight(_)
                | Effect::AddBlight(_)
                | Effect::AddDahan(_)
                | Effect::AddToken { .. }
                | Effect::RemoveToken { .. }
                | Effect::Remove { .. }
                | Effect::Destroy { .. }
                | Effect::Replace { .. }
                | Effect::Downgrade(_)
                | Effect::Isolate
                | Effect::SkipRavage
                | Effect::SkipBuild
                | Effect::SkipExplore
                | Effect::SkipAll
                | Effect::NoCascade
                | Effect::NoBlight
                | Effect::Ravage
                | Effect::Build
                | Effect::Infighting(_)
                | Effect::Duel { .. }
                | Effect::DahanHealth(_)
                | Effect::DahanSimultaneous
                | Effect::DahanImmune
                | Effect::InvaderDamageMinus(_)
                | Effect::Move { .. }
                | Effect::Per { .. }
                | Effect::Conditional { .. }
                | Effect::Choice(_)
        )
    };
    c.all_effects().any(land_effect)
}

/// The lands a Spirit-target card may act in: where the Spirit has presence.
fn presence_lands(presence: &BTreeMap<LandId, u32>) -> Vec<LandId> {
    presence.iter().filter(|(_, n)| **n > 0).map(|(l, _)| l.clone()).collect()
}

/// How far from the spirit's presence the card's presence may go (its own text, else Range 1).
fn presence_range(c: &PowerCard) -> u32 {
    c.all_effects()
        .find_map(|e| match e {
            Effect::AddPresence { range: Some(r), .. } => Some(*r as u32),
            _ => None,
        })
        .unwrap_or(1)
}

/// Lands within `range` of any presence (not Ocean): where a placed presence may go.
fn presence_add_targets(presence: &BTreeMap<LandId, u32>, range: u32, graph: &si_model::island::Island) -> Vec<LandId> {
    graph
        .lands
        .iter()
        .filter(|(_, l)| l.terrain != si_model::island::Terrain::Ocean)
        .filter(|(id, _)| presence.iter().any(|(from, n)| *n > 0 && graph.distance(from, id, false).is_some_and(|d| d <= range)))
        .map(|(id, _)| id.clone())
        .collect()
}

/// Lands a card may target this turn from the given presence: in range (from a Sacred
/// Site where the card says so) and allowed by the card's target rule, as the land
/// stood at the Fast phase.
fn targets_from(card: &PowerCard, presence: &BTreeMap<LandId, u32>, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<LandId> {
    let mut out = Vec::new();
    for (id, land) in &graph.lands {
        if land.terrain == si_model::island::Terrain::Ocean {
            continue;
        }
        let st = before.get(id).copied().unwrap_or_default();
        if si_metrics::reach::range_ok(card, presence, id, graph) != Some(true) {
            continue;
        }
        if !si_metrics::reach::target_ok(&card.target, &st, Some(land.terrain), Some(land.coastal)) {
            continue;
        }
        out.push(id.clone());
    }
    out
}

/// Lands a card may target this turn from the spirit's presence as recorded.
fn targets_for(card: &PowerCard, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<LandId> {
    targets_from(card, &s.presence_after, before, graph)
}

fn innate_targets(innate: &si_model::spirit::Innate, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<LandId> {
    let pseudo = PowerCard { id: innate.id.clone(), title: innate.title.clone(), kind: si_model::power::PowerKind::Unique, cost: 0, speed: innate.speed, range: innate.range.clone(), target: innate.target.clone(), elements: ElementSet::new(), body: vec![], thresholds: vec![], effects: vec![], expansion: String::new() };
    targets_for(&pseudo, s, before, graph)
}

/// The innate's lands as the presence may stand when it acts: for a spirit whose presence
/// moves with the Dahan, a land next to its presence too, where presence walks in with a
/// Dahan gathered or pushed there first (Lead the Furious Assault, range 0, on the land
/// Gather the Warriors filled). The simulator checks the range when the innate acts.
fn innate_targets_reach(innate: &si_model::spirit::Innate, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island, presence_moves: bool) -> Vec<LandId> {
    let mut out = innate_targets(innate, s, before, graph);
    if presence_moves {
        let mut s2 = s.clone();
        for l in s.presence_after.keys() {
            for a in graph.lands.get(l).map(|x| x.adjacent.iter().cloned().collect::<Vec<_>>()).unwrap_or_default() {
                s2.presence_after.entry(a).or_insert(1);
            }
        }
        for x in innate_targets(innate, &s2, before, graph) {
            if !out.contains(&x) {
                out.push(x);
            }
        }
    }
    out
}

/// Is `target` within `distance` of the spirit's presence as the island stands now (from a
/// Sacred Site when the range says so)?
fn in_range_now(island: &Island, target: &LandId, range: &si_model::power::Range) -> bool {
    let need = if range.from_sacred_site { 2 } else { 1 };
    let mut frontier: Vec<LandId> = vec![target.clone()];
    let mut seen: BTreeSet<LandId> = frontier.iter().cloned().collect();
    for step in 0..=range.distance {
        if frontier.iter().any(|l| island.lands.get(l).is_some_and(|x| x.c.presence >= need)) {
            return true;
        }
        if step == range.distance {
            break;
        }
        let mut next = Vec::new();
        for l in &frontier {
            for a in island.lands.get(l).map(|x| x.adjacent.clone()).unwrap_or_default() {
                if seen.insert(a.clone()) {
                    next.push(a);
                }
            }
        }
        frontier = next;
    }
    false
}

/// One card's ways to be played from the given presence: a land-targeting card at each
/// legal target; a Spirit-target card that places presence at each land the presence may
/// go; any other Spirit-target card once, with no land (it acts on the Spirit, or in all
/// the Spirit's lands as an Area).
pub(crate) fn play_options(c: &PowerCard, presence: &BTreeMap<LandId, u32>, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<Play> {
    let fast = c.speed == Speed::Fast;
    let one = |target: Option<LandId>, places: bool| Play { power: c.id.clone(), target, fast, innate: false, cost: c.cost, places, forced: false, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None };
    if (c.range.is_some() || acts_in_one_land(c)) && !fast {
        // a Slow card commits now and takes its land after the Invader steps (`resolve_slow`)
        vec![one(None, false)]
    } else if c.range.is_some() {
        let mut v: Vec<Play> = targets_from(c, presence, before, graph).into_iter().map(|t| one(Some(t), false)).collect();
        if v.is_empty() {
            v.push(one(None, false));
        }
        v
    } else if acts_in_one_land(c) {
        // the Spirit picks one of its lands: each is a way to play the card
        let mut v: Vec<Play> = presence_lands(presence).into_iter().map(|t| one(Some(t), false)).collect();
        if v.is_empty() {
            v.push(one(None, false));
        }
        v
    } else if c.target.is_spirit() && adds_presence(c) {
        let mut v: Vec<Play> = presence_add_targets(presence, presence_range(c), graph).into_iter().map(|t| one(Some(t), true)).collect();
        if v.is_empty() {
            v.push(one(None, false));
        }
        v
    } else {
        vec![one(None, false)]
    }
}

/// Every set of up to `plays` cards from the hand within the Energy, each card with each
/// of its ways to be played. A card that places presence goes first, and the presence it
/// adds counts for the cards after it (a Sacred Site made in the Fast phase is in range
/// for the rest of the turn; targets are chosen when a Power resolves). Capped so a wide
/// hand does not explode.
fn candidate_plays(hand: &[&PowerCard], plays: usize, energy: i64, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<Vec<Play>> {
    let mut out: Vec<Vec<Play>> = vec![vec![]];
    let n = hand.len().min(12);
    for mask in 0u32..(1u32 << n) {
        let mut idx: Vec<usize> = (0..n).filter(|i| mask & (1 << i) != 0).collect();
        if idx.is_empty() || idx.len() > plays {
            continue;
        }
        let cost: i64 = idx.iter().map(|i| hand[*i].cost).sum();
        if cost > energy {
            continue;
        }
        idx.sort_by_key(|i| !adds_presence(hand[*i]));
        let mut combos: Vec<(Vec<Play>, BTreeMap<LandId, u32>)> = vec![(vec![], s.presence_after.clone())];
        for i in &idx {
            let c = hand[*i];
            let mut next = Vec::new();
            for (so_far, presence) in &combos {
                for opt in play_options(c, presence, before, graph) {
                    let mut presence2 = presence.clone();
                    if opt.places {
                        if let Some(t) = &opt.target {
                            *presence2.entry(t.clone()).or_insert(0) += 1;
                        }
                    }
                    let mut c2 = so_far.clone();
                    c2.push(opt);
                    next.push((c2, presence2));
                }
            }
            combos = next;
            if combos.len() > 3000 {
                combos.truncate(3000);
            }
        }
        out.extend(combos.into_iter().map(|(p, _)| p));
    }
    out
}

/// Choose the targets of the Slow plays on the island the Invader steps left: each Slow
/// card or innate in turn takes the legal target that scores best (with the lookahead)
/// given the Slow plays already placed. A play that carries a target keeps it unless
/// `rechoose` (the informed score, where the recorded target was chosen for another
/// Explore card). A Spirit-target card keeps its anchor; a play with no legal target is
/// left untargeted and skipped.
/// Does the Power push Dahan (Voice of Thunder, Gather the Warriors)? Where Dahan go is the
/// player's choice, so a line tries each adjacent land for them.
pub(crate) fn pushes_dahan(defs: &Definitions, power: &str) -> bool {
    dahan_moves(defs, power).1
}

/// Does the Power gather Dahan, push Dahan (Gather the Warriors both, Voice of Thunder a push)?
pub(crate) fn dahan_moves(defs: &Definitions, power: &str) -> (bool, bool) {
    fn any(es: &[Effect], gather: bool) -> bool {
        es.iter().any(|e| match e {
            Effect::Push { what, .. } if !gather => what.iter().any(|f| matches!(f, si_model::power::PieceFilter::Piece(PieceKind::Dahan))),
            Effect::Gather { what, .. } if gather => what.iter().any(|f| matches!(f, si_model::power::PieceFilter::Piece(PieceKind::Dahan))),
            Effect::Per { then, .. } | Effect::Conditional { then, .. } => any(then, gather),
            Effect::Choice(b) => b.iter().any(|x| any(x, gather)),
            _ => false,
        })
    }
    let Some((base, th)) = verify::power_effects(defs, power) else { return (false, false) };
    let has = |g: bool| any(&base, g) || th.iter().any(|(_, es)| any(es, g));
    (has(true), has(false))
}

/// A gather of every Dahan the Power may take from `from` into `to`.
fn gather_all(from: &LandId, to: &LandId) -> Move {
    Move { from: from.clone(), to: to.clone(), piece: Some(si_model::power::PieceFilter::Piece(PieceKind::Dahan)), n: 9 }
}

/// The best gathers and pushes for one play on `tg` of a Power that moves Dahan, found one
/// choice at a time: the gather's source (the fullest-first default, or every Dahan of one
/// neighbor), a second source beside it, then the push's destination (or none). `eval`
/// scores the line with the play in place. A Power that moves no Dahan is scored as it is.
fn best_moves(island: &Island, defs: &Definitions, p: &Play, tg: &LandId, eval: &dyn Fn(&Play) -> f64) -> (f64, Play) {
    let (gathers, pushes) = dahan_moves(defs, &p.power);
    let base = Play { target: Some(tg.clone()), moves: Vec::new(), exact: false, kills: None, ..p.clone() };
    let mut best = (eval(&base), base.clone());
    let consider = |cand: Play, best: &mut (f64, Play)| {
        let v = eval(&cand);
        if v > best.0 + 1e-9 {
            *best = (v, cand);
        }
    };
    let neighbors = push_lands(island, tg);
    if gathers {
        let sources: Vec<LandId> = neighbors.iter().filter(|a| island.lands.get(*a).is_some_and(|l| l.c.dahan > 0)).cloned().collect();
        for s in &sources {
            consider(Play { moves: vec![gather_all(s, tg)], exact: true, ..base.clone() }, &mut best);
        }
        if best.1.exact {
            let first = best.1.moves.clone();
            for s in sources.iter().filter(|s| !first.iter().any(|m| m.from == **s)) {
                let mut moves = first.clone();
                moves.push(gather_all(s, tg));
                consider(Play { moves, exact: true, ..base.clone() }, &mut best);
            }
        }
    }
    if pushes && (gathers || island.lands.get(tg).is_some_and(|l| l.c.dahan > 0)) {
        let start = best.1.clone();
        for d in &neighbors {
            let mut moves = start.moves.clone();
            moves.extend(push_all(tg, d));
            consider(Play { moves, ..start.clone() }, &mut best);
        }
    }
    best
}

/// The lands a piece pushed out of `from` can go to: the adjacent ones, Ocean aside.
fn push_lands(island: &Island, from: &LandId) -> Vec<LandId> {
    island.lands.get(from).map(|l| l.adjacent.iter().filter(|a| island.lands.get(*a).is_some_and(|n| n.terrain != Some(si_model::island::Terrain::Ocean))).cloned().collect()).unwrap_or_default()
}

/// A Dahan push into `to`, as many as the Power may.
fn push_all(from: &LandId, to: &LandId) -> Vec<Move> {
    vec![Move { from: from.clone(), to: to.clone(), piece: Some(si_model::power::PieceFilter::Piece(PieceKind::Dahan)), n: 9 }]
}

pub(crate) fn resolve_slow(island: &Island, defs: &Definitions, plays: &[Play], rechoose: bool, s: &SpiritTurn, graph: &si_model::island::Island, innates: &[&si_model::spirit::Innate], eval: &dyn Fn(&[Play]) -> f64) -> Vec<Play> {
    let presence: BTreeMap<LandId, u32> = island.lands.iter().filter(|(_, l)| l.c.presence > 0).map(|(id, l)| (id.clone(), l.c.presence)).collect();
    let counts: BTreeMap<LandId, Counts> = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
    let mut now = s.clone();
    now.presence_after = presence.clone();
    let mut decided: Vec<Play> = Vec::new();
    for (at, p) in plays.iter().enumerate().filter(|(_, p)| !p.fast) {
        let targets: Option<Vec<LandId>> = if p.innate {
            innates.iter().find(|i| i.id == p.power).map(|i| innate_targets(i, &now, &counts, graph))
        } else {
            defs.cards.get(&p.power).and_then(|c| if c.range.is_some() { Some(targets_from(c, &presence, &counts, graph)) } else if acts_in_one_land(c) { Some(presence_lands(&presence)) } else { None })
        };
        let Some(targets) = targets else {
            decided.push(p.clone());
            continue;
        };
        // a Repeat goes where it says, from the land its first use took (`repeat`)
        let targets = match (decided.iter().find(|q| q.power == p.power).and_then(|q| q.target.clone()), repeat::place(defs, &p.power)) {
            (Some(first), Some(at)) => repeat::lands(at, &first, &targets, graph, &presence),
            _ => targets,
        };
        if (p.target.is_some() && !rechoose) || p.inert {
            decided.push(p.clone());
            continue;
        }
        let mut best: Option<(f64, Play)> = None;
        // a use is scored with the Repeats still to come in the same land, where they may go
        // there: one Damage alone may kill nothing three on one Town do (Creepers)
        let same_land = matches!(repeat::place(defs, &p.power), Some(si_model::power::RepeatAt::Usual | si_model::power::RepeatAt::TargetOrPresence));
        let to_come: Vec<&Play> = if same_land { plays[at + 1..].iter().filter(|q| q.power == p.power && !q.fast && (q.target.is_none() || rechoose)).collect() } else { Vec::new() };
        for tg in targets {
            // a land chosen after the Invader steps comes with its moves chosen then too: where
            // a gather takes from, where a push sends (or no push: Voice of Thunder's Fear)
            let score = |c: &Play| -> f64 {
                let mut trial = decided.clone();
                trial.push(c.clone());
                trial.extend(to_come.iter().map(|q| Play { target: Some(tg.clone()), ..(*q).clone() }));
                eval(&trial)
            };
            let (v, cand) = best_moves(island, defs, p, &tg, &score);
            if best.as_ref().is_none_or(|(b, _)| v > *b) {
                best = Some((v, cand));
            }
        }
        decided.push(best.map(|(_, c)| c).unwrap_or_else(|| Play { target: None, kills: None, ..p.clone() }));
    }
    decided
}

/// The line's plays with the Slow targets the simulation chose under the Explore card
/// that came (Fast plays as given, then the resolved Slow plays).
/// The other Spirits' presence after their Growth this turn, by land, as recorded: the review
/// plays the first Spirit's turn; the rest stand where the game had them.
pub(crate) fn others_presence(rec: &GameRecord, turn: u32, spirit: &str) -> BTreeMap<LandId, u32> {
    let mut out: BTreeMap<LandId, u32> = BTreeMap::new();
    for s in rec.spirits.iter().filter(|s| s.turn == turn && s.spirit != spirit) {
        for (l, n) in &s.presence_after {
            *out.entry(l.clone()).or_insert(0) += n;
        }
    }
    out
}

/// The plays in the order the record used them (a play the record did not use keeps its place).
pub(crate) fn in_use_order(mut plays: Vec<Play>, s: &SpiritTurn) -> Vec<Play> {
    let order: Vec<&str> = s.powers_used.iter().map(|(_, p, _)| p.as_str()).collect();
    plays.sort_by_key(|p| order.iter().position(|o| *o == p.power).unwrap_or(usize::MAX));
    plays
}

pub(crate) fn with_slow(plays: &[Play], out: &Outcome) -> Vec<Play> {
    if out.slow.is_empty() {
        return plays.to_vec();
    }
    plays.iter().filter(|p| p.fast).cloned().chain(out.slow.iter().cloned()).collect()
}

/// The turn after next, for the deeper lookahead: its Ravage card (this turn's Explore,
/// the card that came or the one assumed), England's extra Build when recorded, and
/// the next turn's possible Explore cards (empty when the next turn ran as recorded,
/// its Explore included).
pub struct Deep {
    pub ravage: Vec<String>,
    pub high_immigration: Vec<String>,
    pub pool: Vec<String>,
}

pub(crate) fn deep_for(rec: &GameRecord, defs: &Definitions, deck: &deck::Deck, t: &TurnRecord, assumed: &[String], informed: bool) -> Option<Deep> {
    if informed {
        // the next turn's Explore is face down: average over its stage's remaining cards
        // (this turn's assumed card cannot come twice); the turn after's Ravage is this
        // turn's assumed Explore. No recorded next turn is needed: an open turn has none.
        let mut pool = deck::pool(rec, defs, deck, t, t.turn + 1).map(|(_, c)| c).unwrap_or_default();
        pool.retain(|c| !assumed.contains(c));
        return Some(Deep { ravage: assumed.to_vec(), high_immigration: vec![], pool });
    }
    rec.turns.iter().find(|n| n.turn == t.turn + 1)?;
    // the next turn ran as recorded; the turn after as recorded when the record has
    // it, else from this turn's Explore card
    let after = rec.turns.iter().find(|n| n.turn == t.turn + 2);
    let ravage = after.map(|a| a.ravage.clone()).filter(|r| !r.is_empty()).unwrap_or_else(|| assumed.to_vec());
    let high_immigration = after.map(|a| a.high_immigration.clone()).unwrap_or_default();
    Some(Deep { ravage, high_immigration, pool: vec![] })
}

/// The Slow plays with their lands cleared, for the simulator to choose (presence
/// placements keep theirs).
fn clear_slow_targets(v: &[Play]) -> Vec<Play> {
    v.iter().map(|p| if p.fast || p.places { p.clone() } else { Play { target: None, kills: None, ..p.clone() } }).collect()
}

/// Review every turn of the game with the default options.
pub fn review(rec: &GameRecord, defs: &Definitions) -> Review {
    review_with(rec, defs, &ReviewOptions::default())
}

/// What a review reports while it works (one turn at a time; `Positions` from any thread).
#[derive(Debug, Clone, PartialEq)]
pub enum Progress {
    /// A turn's enumeration starts: how many growth positions it will try.
    Start { turn: u32, positions: usize },
    /// One more growth position done.
    Positions { turn: u32, done: usize, total: usize },
    /// Enumeration done: play sets tried and lines kept; the ranking over the Explore cards starts.
    Ranking { turn: u32, play_sets: usize, lines: usize },
    /// The turn is reviewed.
    Done { turn: u32, lines: usize, explore_cards: usize, seconds: f64 },
}

/// The plays the journal records for the spirit this turn, cards and innates apart, each
/// with its target as recorded (or recovered from where it acted).
pub(crate) fn recorded_plays(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, t: &TurnRecord, all_innates: &[&si_model::spirit::Innate]) -> (Vec<Play>, Vec<Play>) {
    let cards: Vec<Play> = s
        .cards_played
        .iter()
        .map(|c| {
            let card = defs.cards.get(c);
            let fast = card.is_some_and(|c| c.speed == Speed::Fast);
            let mut target = s.powers_used.iter().find(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && pw == c).and_then(|(_, _, t)| t.clone());
            if target.is_none() {
                // a Spirit-target card that placed presence: where it went (from the track, or
                // from the island: "Add" may take a presence already on the board)
                target = s.presence_by_power.iter().find(|(pw, _, _)| pw == c).and_then(|(_, _, l)| l.clone()).or_else(|| verify::use_of(t, c, 0).and_then(|u| u.moves.iter().find(|(k, _, _)| k == "Presence").map(|(_, _, to)| to.clone())));
            }
            if target.is_none() {
                // a Spirit-target card (Fiery Vengeance) names its land only in the damage it dealt
                target = rec.lands.values().find(|l| l.touches.iter().any(|x| x.turn == t.turn && x.by == *c)).map(|l| l.land.clone());
            }
            if target.is_none() {
                // a card whose land the player chose (Solidify Echoes: "Choose one of target Spirit's lands"):
                // the recorded choice, the Defend or chosen-land prompt before any "Adding…" one
                let placed = verify::placements_of(defs, t, c);
                target = placed.iter().find(|(r, _)| !r.starts_with("Adding")).or(placed.first()).map(|(_, l)| l.clone());
            }
            // what it moved, each kind named, as the use recorded it (else the touches)
            let moves = verify::use_of(t, c, 0).filter(|u| !u.moves.is_empty()).map(verify::moves_of_use).unwrap_or_else(|| verify::recorded_moves(rec, t.turn, c, target.as_ref()));
            // the phase it was used in: a card printed Slow another Power made Fast acted then
            let printed_fast = fast;
            let fast = match s.powers_used.iter().find(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && pw == c) {
                Some((p, _, _)) => *p == Phase::FastPower,
                None => printed_fast,
            };
            let kills = verify::use_of(t, c, 0).filter(|_| destroys_pieces(defs, c)).map(verify::kills_of);
            Play { power: c.clone(), target, fast, innate: false, cost: card.map(|c| c.cost).unwrap_or(0), places: card.is_some_and(|c| c.target.is_spirit() && adds_presence(c)), forced: fast && !printed_fast, exact: moves_pieces(defs, c), moves, inert: false, recorded: true, kills }
        })
        .collect();
    // a card used again (Sea Monsters' Repeat): each later use a play of its own, at the land
    // it took, with no Energy (`repeat`)
    let mut cards = cards;
    for c in &s.cards_played {
        let Some(first) = cards.iter().find(|p| p.power == *c).cloned() else { continue };
        let mut nth = 1;
        while let Some(u) = verify::use_of(t, c, nth) {
            let moves = if u.moves.is_empty() { Vec::new() } else { verify::moves_of_use(u) };
            let kills = destroys_pieces(defs, c).then(|| verify::kills_of(u));
            let fast = u.phase.map(|p| p == Phase::FastPower).unwrap_or(first.fast);
            cards.push(Play { target: u.target.clone().or(first.target.clone()), fast, cost: 0, moves, kills, ..first.clone() });
            nth += 1;
        }
    }
    let turn_no = t.turn;
    let turn_rec = t;
    // an innate used in the Fast phase though printed Slow went through its "may be Fast" tier: the mark says so
    let used: Vec<&(Phase, String, Option<LandId>)> = s.powers_used.iter().filter(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && all_innates.iter().any(|i| i.id == *pw)).collect();
    let innates: Vec<Play> = used
        .iter()
        .enumerate()
        .map(|(k, (p, pw, t))| {
            let fast = *p == Phase::FastPower;
            let printed_slow = all_innates.iter().find(|i| i.id == *pw).is_some_and(|i| i.speed != Speed::Fast);
            // which use of it this is, for the record's account of what it did
            let nth = used[..k].iter().filter(|(_, q, _)| q == pw).count();
            let moves = verify::use_of(turn_rec, pw, nth).filter(|u| !u.moves.is_empty()).map(verify::moves_of_use).unwrap_or_else(|| verify::recorded_moves(rec, turn_no, pw, t.as_ref()));
            let kills = verify::use_of(turn_rec, pw, nth).filter(|_| destroys_pieces(defs, pw)).map(verify::kills_of);
            Play { power: pw.clone(), target: t.clone(), fast, innate: true, cost: 0, places: false, forced: fast && printed_slow, moves, exact: moves_pieces(defs, pw), inert: false, recorded: true, kills }
        })
        .collect();
    (cards, innates)
}

/// The pieces the record saw a Power gather into or push out of its land this turn, as
/// the line's requests, so the actual line replays the player's own lands.
/// What the turn's Event and Fear cards did, for every line to replay as the record has it.
pub(crate) struct Exo {
    /// The land choices whose effect the simulator can place in another land: (choice index, land, effect).
    pub scorable: Vec<(usize, LandId, ChoiceEffect)>,
    /// The rest: what the log shows at the first Invader step beyond what the actual Fast
    /// plays and the choices account for, land by land.
    pub residual: Option<Delta>,
    /// The whole Event and Fear piece effect (choices included) in ledger value, and the lands it touched.
    pub exogenous: Option<(f64, usize)>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn exo_effects(rec: &GameRecord, defs: &Definitions, t: &TurnRecord, s: &SpiritTurn, rules: &Rules, spirit: &str, values: &Values, actual_plays: &[Play], actual_innates: &[Play]) -> Exo {
    let scorable: Vec<(usize, LandId, ChoiceEffect)> = t
        .choices
        .iter()
        .enumerate()
        .filter(|(_, c)| c.kind == "Location" && matches!(source_kind_of(defs, c).as_str(), "Event" | "Fear"))
        .filter_map(|(i, c)| {
            let land: LandId = c.selected.parse().ok()?;
            let e = choice_effect(rec, defs, t, c, &land)?;
            Some((i, land, e))
        })
        .collect();
    let ctx_exo = Ctx { presence_with_dahan: rules.presence_with_dahan_for(spirit), ..Ctx::plain(rules.invader_health_bonus(), ElementSet::new()) };
    let actual_readable = actual_plays.iter().all(|p| defs.cards.get(&p.power).is_none_or(|c| c.fully_parsed()));
    if !actual_readable || t.before_invaders.is_empty() {
        return Exo { scorable, residual: None, exogenous: None };
    }
    let mut rules_t = rules.clone();
    let mut probe = Outcome::default();
    // the board as the Fast phase ended: the record's own when it has it, else the actual
    // Fast plays simulated (a log without the snapshot); what the Event and Fear steps did is
    // the step from there, so a Power's effect the simulator misses is not counted as theirs
    let mut island = if !t.after_fast.is_empty() {
        verify::island_for(rec, defs, &t.after_fast)
    } else {
        let mut island = verify::island_at(rec, defs, t, "before_fast");
        let played: Vec<&PowerCard> = actual_plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
        let elements = elements_for(defs, s, &played);
        let all: Vec<Play> = in_use_order(actual_plays.iter().chain(actual_innates.iter()).cloned().collect(), s);
        let mut armed = Vec::new();
        apply_plays(&mut island, defs, &all, true, &elements, rules, spirit, t, &mut armed, &mut probe);
        island
    };
    let full = delta_between(&island, &t.before_invaders);
    for (_, land, e) in &scorable {
        apply_choice_effect(&mut island, &mut rules_t, land, e, &ctx_exo, &mut probe);
    }
    let mut residual = delta_between(&island, &t.before_invaders);
    // the Damage the Event and Fear steps left on pieces (Civil Unrest's on a City), as the
    // record's wounds before the Invader steps less those as the Fast phase ended
    if let (Some(at), Some(was)) = (t.wounds.get("before_invaders"), t.wounds.get("after_fast")) {
        for (land, ws) in at {
            let mut left: Vec<(PieceKind, u32)> = was.get(land).cloned().unwrap_or_default();
            for w in ws {
                match left.iter().position(|x| x == w) {
                    Some(i) => {
                        left.remove(i);
                    }
                    None => residual.1.entry(land.clone()).or_default().push(*w),
                }
            }
        }
    }
    Exo { scorable, residual: Some(residual), exogenous: Some((delta_value(&full, values), full.0.len())) }
}

/// The turns a review can judge: every turn before the journal's last, and the last when
/// the game ended in it. The turn being played is never one of them.
pub fn closed_turns(rec: &GameRecord) -> Vec<u32> {
    rec.turns.iter().filter(|t| t.turn >= 1 && !t.before_fast.is_empty() && (t.turn < rec.meta.turns || rec.meta.ending.is_some())).map(|t| t.turn).collect()
}

/// Review every turn (with the options) and report nothing.
pub fn review_with(rec: &GameRecord, defs: &Definitions, opts: &ReviewOptions) -> Review {
    review_turns(rec, defs, opts, None, &|_| {})
}

/// Review every turn of the game.
/// Review the turns in `only` (every turn when None), reporting progress through
/// `on_progress`; the TUI reviews one turn per call in a worker thread.
pub fn review_turns(rec: &GameRecord, defs: &Definitions, opts: &ReviewOptions, only: Option<&[u32]>, on_progress: &(dyn Fn(Progress) + Sync)) -> Review {
    // the Powers as the scenario plays them (Blitz: all Fast, a card 1 Energy cheaper)
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let defs: &Definitions = &scenario_defs;
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let cal = opts.calibration.clone().unwrap_or_default();
    let mut values = Values::for_panel(defs.spirits.get(&spirit));
    if !opts.score_losses {
        values.loss = 0.0;
    }
    values.next_weight = cal.next_weight;
    let mut notes: Vec<String> = Vec::new();
    if rec.meta.spirits.len() > 1 {
        notes.push("more than one spirit: only the first spirit's growth and plays are reviewed in this version".into());
    }
    let level = rec.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    let graph = si_metrics::reach::island_for(rec, defs);
    let deck = deck::Deck::for_game(rec, defs, level);
    let pick = defs.spirits.get(&spirit).map(|p| p.growth.pick).unwrap_or(1);
    if opts.growth && pick > 1 && !opts.actual_only {
        notes.push(format!("{spirit} picks {pick} growth groups a turn: growth alternatives are not enumerated for it yet; the growth is taken as played"));
    }
    let enumerate_growth = opts.growth && pick <= 1 && !opts.actual_only;
    let all_innates = innates_of(defs, &spirit);
    let mut turns = Vec::new();
    let closed = closed_turns(rec);
    for t in rec.turns.iter().filter(|t| closed.contains(&t.turn) && only.is_none_or(|o| o.contains(&t.turn))) {
        let Some(s) = rec.spirits.iter().find(|s| s.turn == t.turn && s.spirit == spirit) else { continue };
        // the turn's rules and the record's choices, as every replay of it has them; the loss
        // conditions' counts as this turn began (the Blight pool the caller knows, if any)
        let rules = {
            let mut r = turn_rules(rec, defs, t);
            r.clocks = loss_clocks(rec, t.turn, opts.blight_pool.get(&t.turn).copied());
            r
        };
        let clocks = invaders::clocks(&verify::island_at(rec, defs, t, "before_fast"), &rules, invaders::Tally::default());
        let ended_before_slow = t.before_slow.is_empty();
        let next_turn = rec.turns.iter().find(|n| n.turn == t.turn + 1);
        // positions: the growth actually taken first, then every growth the panel allowed
        let actual_pos = actual_position(rec, defs, s, t);
        // the spirit turn as the plays saw it: presence after the growth, before any Power placed more
        let s_growth: SpiritTurn = {
            let mut x = s.clone();
            x.presence_after = actual_pos.presence.clone();
            x
        };
        let s = &s_growth;
        let mut positions: Vec<Position> = vec![actual_pos.clone()];
        if enumerate_growth {
            // round-robin over the growth groups, so a cap does not starve the later groups
            let all = growth_positions(rec, defs, s, t, &graph);
            let mut by_group: BTreeMap<String, Vec<Position>> = BTreeMap::new();
            for p in all {
                by_group.entry(p.growth.first().cloned().unwrap_or_default()).or_default().push(p);
            }
            let mut queues: Vec<std::collections::VecDeque<Position>> = by_group.into_values().map(|v| v.into_iter().collect()).collect();
            while positions.len() < opts.max_positions.max(1) && queues.iter().any(|q| !q.is_empty()) {
                for q in queues.iter_mut() {
                    if let Some(p) = q.pop_front() {
                        if !positions.iter().any(|x| same_position(x, &p)) {
                            positions.push(p);
                        }
                    }
                }
            }
            positions.truncate(opts.max_positions.max(1));
        }
        let started = std::time::Instant::now();
        let done = std::sync::atomic::AtomicUsize::new(0);
        let total = positions.len().max(1);
        on_progress(Progress::Start { turn: t.turn, positions: positions.len() });
        // the actual line, as recorded (targets included)
        let (actual_plays, actual_innates) = recorded_plays(rec, defs, s, t, &all_innates);
        let ctx_exo = Ctx { presence_with_dahan: rules.presence_with_dahan_for(&spirit), ..Ctx::plain(rules.invader_health_bonus(), ElementSet::new()) };
        let Exo { scorable, residual, exogenous } = exo_effects(rec, defs, t, s, &rules, &spirit, &values, &actual_plays, &actual_innates);
        let actual_readable = actual_plays.iter().all(|p| defs.cards.get(&p.power).is_none_or(|c| c.fully_parsed()));
        // the simulator's error on this turn's Invader phase: the actual Fast plays, the Event
        // and Fear effects from the log and the real Invader steps, against the recorded state
        // at the Slow phase (calibration material for the confidence smoothing)
        let model_error: Option<f64> = match &residual {
            Some(d) if !t.before_slow.is_empty() => {
                let mut island = verify::island_at(rec, defs, t, "before_fast");
                let others = others_presence(rec, t.turn, &spirit);
                for (id, l) in island.lands.iter_mut() {
                    l.c.presence = s.presence_after.get(id).copied().unwrap_or(0) + others.get(id).copied().unwrap_or(0);
                }
                let mut rules_t = rules.clone();
                let played: Vec<&PowerCard> = actual_plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
                let elements = elements_for(defs, s, &played);
                let mut probe = Outcome::default();
                // in the order played, as every other replay of the actual line
                let all: Vec<Play> = in_use_order(actual_plays.iter().chain(actual_innates.iter()).cloned().collect(), s);
                let mut armed = Vec::new();
                apply_plays(&mut island, defs, &all, true, &elements, &rules, &spirit, t, &mut armed, &mut probe);
                for (_, land, e) in &scorable {
                    apply_choice_effect(&mut island, &mut rules_t, land, e, &ctx_exo, &mut probe);
                }
                probe.presence_destroyed += apply_delta(&mut island, d);
                invader_phase(&mut island, rec, t, defs, &rules_t, &mut probe, None, true);
                Some(delta_value(&delta_between(&island, &t.before_slow), &values))
            }
            _ => None,
        };
        // `informed`: score on what was knowable — the given Explore card (or the one that
        // came), no Event or Fear effects, cards only hindsight knows (`drop`) not played
        let simulate = |plays: &[Play], innate_plays: &[Play], s2: &SpiritTurn, moved: Option<(usize, &LandId)>, explore: Option<&[String]>, informed: bool, drop: &[String], deep: bool| -> Outcome {
            let mut island = verify::island_at(rec, defs, t, "before_fast");
            // the position's presence: its growth, before any Power placed more (the other
            // Spirits' as recorded)
            let others = others_presence(rec, t.turn, &spirit);
            for (id, l) in island.lands.iter_mut() {
                l.c.presence = s2.presence_after.get(id).copied().unwrap_or(0) + others.get(id).copied().unwrap_or(0);
            }
            let mut rules_t = rules.clone();
            let kept: Vec<Play> = plays.iter().filter(|p| !(informed && drop.contains(&p.power))).cloned().collect();
            let played: Vec<&PowerCard> = kept.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
            let elements = elements_for(defs, s2, &played);
            let mut out = Outcome::default();
            let mut all: Vec<Play> = kept;
            all.extend(innate_plays.iter().cloned());
            // the recorded line goes in the order it was played (an innate before a card
            // when it was): every play of it is in the record's use order
            let order: Vec<&str> = s2.powers_used.iter().map(|(_, p, _)| p.as_str()).collect();
            if !all.is_empty() && all.iter().all(|p| order.contains(&p.power.as_str())) {
                all.sort_by_key(|p| order.iter().position(|o| *o == p.power));
            } else {
                // an enumerated line moves its Dahan first in the Slow phase, then strikes where
                // they went (Voice of Thunder into the land Manifestation targets)
                all.sort_by_key(|p| !p.fast && !pushes_dahan(defs, &p.power));
            }
            let mut armed = Vec::new();
            apply_plays(&mut island, defs, &all, true, &elements, &rules_t, &spirit, t, &mut armed, &mut out);
            if !informed {
                for (i, land, e) in &scorable {
                    let land = match moved {
                        Some((j, l)) if j == *i => l.clone(),
                        _ => land.clone(),
                    };
                    apply_choice_effect(&mut island, &mut rules_t, &land, e, &ctx_exo, &mut out);
                }
                if let Some(d) = &residual {
                    out.presence_destroyed += apply_delta(&mut island, d);
                }
            }
            invader_phase(&mut island, rec, t, defs, &rules_t, &mut out, explore, !informed);
            // a won game stops: no Slow phase, no next turn
            if !ended_before_slow && out.wins.is_none() {
                let assumed: Vec<String> = explore.map(|e| e.to_vec()).unwrap_or_else(|| t.explore.clone());
                let informed_la: Option<&[String]> = if informed { Some(&assumed) } else { None };
                // the Slow targets, chosen on the island the Invader steps left: a line commits its
                // Slow cards before the Explore and its lands after, as the player does. The
                // informed score re-chooses them under each Explore card (`informed`).
                let slow = {
                    let eval = |slow: &[Play]| -> f64 {
                        let mut isl = island.clone();
                        let mut o = out.clone();
                        let mut armed2 = armed.clone();
                        apply_plays(&mut isl, defs, slow, false, &elements, &rules_t, &spirit, t, &mut armed2, &mut o);
                        lookahead(&mut isl, rec, next_turn, defs, &rules_t, &mut o, informed_la, None);
                        o.score(&values)
                    };
                    resolve_slow(&island, defs, &all, informed, s2, &graph, &all_innates, &eval)
                };
                all.retain(|p| p.fast);
                all.extend(slow.iter().cloned());
                apply_plays(&mut island, defs, &all, false, &elements, &rules_t, &spirit, t, &mut armed, &mut out);
                victory_after_slow(&island, &rules_t, t, &mut out);
                if out.wins.is_none() {
                    let deep_ctx = if deep { deep_for(rec, defs, &deck, t, &assumed, informed) } else { None };
                    lookahead(&mut island, rec, next_turn, defs, &rules_t, &mut out, informed_la, deep_ctx.as_ref());
                }
                out.slow = slow;
            }
            out
        };
        // innate choice per candidate: each innate reached at each legal target (or unused)
        let innate_options = |plays: &[Play], s2: &SpiritTurn| -> Vec<Vec<Play>> {
            let played: Vec<&PowerCard> = plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
            let elements = elements_for(defs, s2, &played);
            let mut options: Vec<Vec<Play>> = vec![vec![]];
            for inn in &all_innates {
                let reached = inn.tiers.iter().any(|tier| !tier.modifier && elements.satisfies(&tier.elements));
                if !reached {
                    continue;
                }
                let slow = inn.speed != Speed::Fast;
                // a Slow innate whose "may be Fast" tier these elements reach is offered both ways
                let may_fast = slow && may_be_fast(inn, &elements);
                let targets = if slow && !may_fast { vec![] } else { innate_targets_reach(inn, s2, &t.before_fast, &graph, rules.presence_with_dahan_for(&spirit)) };
                let mut next = Vec::new();
                for o in &options {
                    next.push(o.clone());
                    if slow {
                        // a Slow innate commits now and takes its land after the Invader steps
                        let mut o2 = o.clone();
                        o2.push(Play { power: inn.id.clone(), target: None, fast: false, innate: true, cost: 0, places: false, forced: false, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None });
                        next.push(o2);
                        if !may_fast {
                            continue;
                        }
                    }
                    for tg in &targets {
                        let mut o2 = o.clone();
                        o2.push(Play { power: inn.id.clone(), target: Some(tg.clone()), fast: true, innate: true, cost: 0, places: false, forced: slow, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None });
                        next.push(o2);
                    }
                }
                options = next;
            }
            options
        };
        let growth_value = |pos: &Position, cost: i64| -> f64 { pos.slots as f64 * values.slot + pos.cards_gained as f64 * values.card + (pos.energy - cost).max(0) as f64 * values.energy };
        // every position: first every play set without innates, then the innates at every target on the best sets
        let per_position = |pi: usize, pos: &Position| -> (usize, Vec<Line>) {
            let s2 = spirit_at(s, pos);
            let mut ids = pos.hand.clone();
            ids.sort();
            ids.dedup();
            let hand: Vec<&PowerCard> = ids.iter().filter_map(|c| defs.cards.get(c)).collect();
            let mut candidates = candidate_plays(&hand, pos.plays.max(0) as usize, pos.energy, &s2, &t.before_fast, &graph);
            if candidates.is_empty() {
                candidates.push(vec![]);
            }
            candidates.truncate(opts.max_lines_per_position.max(1));
            // each Power's Repeats as plays of their own after it (`repeat`): a Fast one at each
            // land it may take
            let usual = |p: &Play| -> Vec<LandId> {
                match defs.cards.get(&p.power) {
                    Some(c) => targets_from(c, &s2.presence_after, &t.before_fast, &graph),
                    None => all_innates.iter().find(|i| i.id == p.power).map(|i| innate_targets(i, &s2, &t.before_fast, &graph)).unwrap_or_default(),
                }
            };
            let elements_of = |cand: &[Play]| -> ElementSet {
                let played: Vec<&PowerCard> = cand.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
                elements_for(defs, &s2, &played)
            };
            let candidates: Vec<Vec<Play>> = candidates.into_iter().flat_map(|cand| repeat::expand(defs, &cand, &elements_of(&cand), &usual, &graph, &s2.presence_after, 4)).collect();
            let tried = candidates.len();
            let mut first: Vec<(f64, Vec<Play>, Outcome, f64)> = candidates
                .into_iter()
                .map(|cand| {
                    let out = simulate(&cand, &[], &s2, None, None, false, &[], false);
                    let cost: i64 = cand.iter().map(|p| p.cost).sum();
                    let gv = growth_value(pos, cost);
                    (out.score(&values) + gv, cand, out, gv)
                })
                .collect();
            first.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            // the innates go on the best card sets, and on the best set of each reach: which
            // tier of each innate its elements reach and whether a "may be Fast" tier opens.
            // A set scored without its innates may rank low only because the innate it
            // enables is what saves the turn (the four Air that take Lead the Furious Assault
            // Fast into the land about to hold England's seventh building)
            let reach = |cand: &[Play]| -> Vec<(usize, bool)> {
                let played: Vec<&PowerCard> = cand.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
                let elements = elements_for(defs, &s2, &played);
                all_innates.iter().map(|inn| (inn.tiers.iter().filter(|t| !t.modifier && elements.satisfies(&t.elements)).count(), may_be_fast(inn, &elements))).collect()
            };
            let mut seen_reach: Vec<Vec<(usize, bool)>> = Vec::new();
            let beam: Vec<bool> = first
                .iter()
                .enumerate()
                .map(|(i, (_, cand, _, _))| {
                    let r = reach(cand);
                    let new_reach = !seen_reach.contains(&r);
                    if new_reach {
                        seen_reach.push(r);
                    }
                    i < opts.innate_beam || new_reach
                })
                .collect();
            let mut lines: Vec<Line> = Vec::new();
            for (i, (score0, cand, out0, gv)) in first.into_iter().enumerate() {
                if beam[i] {
                    let els = elements_of(&cand);
                    let mut options: Vec<Vec<Play>> = innate_options(&cand, &s2).into_iter().flat_map(|inn| repeat::expand(defs, &inn, &els, &usual, &graph, &s2.presence_after, 4)).collect();
                    options.truncate(opts.max_innate_options.max(1));
                    for inn in options {
                        if inn.is_empty() {
                            lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score0, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: pi, elements: String::new(), tiers: vec![], plays: with_slow(&cand, &out0), outcome: out0.clone(), score: score0, expected: 0.0, per_card: vec![], actual: false });
                            continue;
                        }
                        let out = simulate(&cand, &inn, &s2, None, None, false, &[], false);
                        let score = out.score(&values) + gv;
                        let mut plays = cand.clone();
                        plays.extend(inn);
                        let plays = with_slow(&plays, &out);
                        lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: pi, elements: String::new(), tiers: vec![], plays, outcome: out, score, expected: 0.0, per_card: vec![], actual: false });
                    }
                } else {
                    lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score0, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: pi, elements: String::new(), tiers: vec![], plays: with_slow(&cand, &out0), outcome: out0, score: score0, expected: 0.0, per_card: vec![], actual: false });
                }
            }
            lines.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            lines.truncate(opts.keep_per_position.max(1));
            for l in lines.iter_mut() {
                annotate_elements(l, defs, &s2);
            }
            let d = done.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            on_progress(Progress::Positions { turn: t.turn, done: d, total });
            (tried, lines)
        };
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).clamp(1, 16).min(positions.len().max(1));
        let results: Vec<(usize, Vec<Line>)> = if opts.actual_only {
            Vec::new()
        } else if threads <= 1 {
            positions.iter().enumerate().map(|(pi, pos)| per_position(pi, pos)).collect()
        } else {
            let indexed: Vec<(usize, &Position)> = positions.iter().enumerate().collect();
            let chunk = indexed.len().div_ceil(threads);
            let per_position = &per_position;
            std::thread::scope(|sc| {
                let handles: Vec<_> = indexed.chunks(chunk).map(|part| sc.spawn(move || part.iter().map(|(pi, pos)| per_position(*pi, pos)).collect::<Vec<_>>())).collect();
                handles.into_iter().flat_map(|h| h.join().expect("hindsight worker")).collect()
            })
        };
        let mut lines: Vec<Line> = Vec::new();
        let mut count = 0usize;
        for (tried, ls) in results {
            count += tried;
            lines.extend(ls);
        }
        let actual_cost: i64 = actual_plays.iter().map(|p| p.cost).sum();
        let actual_out = simulate(&actual_plays, &actual_innates, s, None, None, false, &[], false);
        let actual_gv = growth_value(&actual_pos, actual_cost);
        // the same line with its Slow targets chosen after the Invader steps: a Slow
        // mis-target is reported as its own gap, not folded into luck
        let (slow_gap, slow_best) = if ended_before_slow || actual_plays.iter().chain(actual_innates.iter()).all(|p| p.fast) {
            (0.0, vec![])
        } else {
            let cleared = |v: &[Play]| -> Vec<Play> { v.iter().map(|p| if p.fast || p.places { p.clone() } else { Play { target: None, kills: None, ..p.clone() } }).collect() };
            let o = simulate(&cleared(&actual_plays), &cleared(&actual_innates), s, None, None, false, &[], false);
            ((o.score(&values) - actual_out.score(&values)).max(0.0), o.slow)
        };
        let mut actual_line = Line { growth: actual_pos.growth.clone(), growth_actual: true, growth_value: actual_gv, ties: 0, informed: 0.0, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays: in_use_order(actual_plays.iter().cloned().chain(actual_innates.iter().cloned()).collect(), s), outcome: actual_out.clone(), score: actual_out.score(&values) + actual_gv, expected: 0.0, per_card: vec![], actual: true };
        actual_line.slow_gap = slow_gap;
        actual_line.slow_best = slow_best;
        annotate_elements(&mut actual_line, defs, s);
        lines.push(actual_line.clone());
        lines.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.plays.len().cmp(&b.plays.len())).then(b.actual.cmp(&a.actual)));
        // the same plays (as a set) from the same growth once; lines with the same plays and
        // score from another placement or slot fold into the first as ties
        let key = |l: &Line| -> Vec<(String, Option<LandId>, bool, bool)> {
            let mut k: Vec<_> = l.plays.iter().map(|p| (p.power.clone(), p.target.clone(), p.fast, p.innate)).collect();
            k.sort();
            k
        };
        let actual_key = (actual_line.growth.clone(), key(&actual_line));
        let mut kept: Vec<Line> = Vec::new();
        let mut seen: Vec<(Vec<String>, Vec<(String, Option<LandId>, bool, bool)>)> = Vec::new();
        for l in lines {
            let k = (l.growth.clone(), key(&l));
            if !l.actual && k == actual_key {
                continue;
            }
            if seen.contains(&k) {
                continue;
            }
            if let Some(prev) = kept.iter_mut().find(|p| !p.actual && !l.actual && (p.score - l.score).abs() < 1e-9 && key(p) == k.1) {
                prev.ties += 1;
                seen.push(k);
                continue;
            }
            seen.push(k);
            kept.push(l);
        }
        let mut lines = kept;
        if !lines.iter().any(|l| l.actual) {
            lines.push(actual_line.clone());
        }
        lines.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.plays.len().cmp(&b.plays.len())).then(b.actual.cmp(&a.actual)));
        on_progress(Progress::Ranking { turn: t.turn, play_sets: count, lines: lines.len() });
        // the informed score: the best lines by hindsight and the actual one, each averaged
        // over the Explore cards that could have come, with the Event and Fear effects left out
        let cands = explore_candidates(rec, defs, &deck, t);
        // the face-down Event and Fear cards: what a player should have expected of them
        let tl_start = rec.turns.iter().filter(|u| u.turn < t.turn).map(|u| u.terror_after).max().unwrap_or(1).clamp(1, 3);
        let fear_k = if t.fear_cards.is_empty() { t.fear_cards_earned } else { t.fear_cards.len() as u32 };
        let expected_exo = cal.expected_exogenous(!t.events.is_empty(), fear_k, tl_start);
        let mut top: Vec<Line> = Vec::new();
        for l in &lines {
            if l.actual || top.len() < opts.informed_lines.max(1) {
                top.push(l.clone());
            }
        }
        let spirit_turn_at = |l: &Line| -> SpiritTurn {
            let pos = &positions[l.position.min(positions.len() - 1)];
            let s2 = spirit_at(s, pos);
            s2
        };
        // the ranked lines' Dahan moves: a Fast play that gathers or pushes Dahan gets its
        // sources and destinations chosen (the enumerator tried only the fullest-first gather
        // and no push), and an innate that moves Dahan which the line leaves out is tried at
        // every land with its moves chosen, so a line without Gather the Warriors is one that
        // found no use for it, not one that never tried
        let fast_island = verify::island_at(rec, defs, t, "before_fast");
        for l in top.iter_mut().filter(|l| !l.actual) {
            let s2 = spirit_turn_at(l);
            let score_of = |plays: &[Play]| -> f64 {
                let (cards, inns): (Vec<Play>, Vec<Play>) = plays.iter().cloned().partition(|p| !p.innate);
                simulate(&cards, &inns, &s2, None, None, false, &[], false).score(&values)
            };
            let mut cur = score_of(&l.plays);
            for i in 0..l.plays.len() {
                let p = l.plays[i].clone();
                let (g, pu) = dahan_moves(defs, &p.power);
                if !p.fast || !p.moves.is_empty() || p.exact || p.inert || !(g || pu) {
                    continue;
                }
                let Some(tg) = p.target.clone() else { continue };
                let (v, cand) = {
                    let plays = &l.plays;
                    best_moves(&fast_island, defs, &p, &tg, &|c: &Play| {
                        let mut tr = plays.clone();
                        tr[i] = c.clone();
                        score_of(&tr)
                    })
                };
                if v > cur + 1e-9 {
                    l.plays[i] = cand;
                    cur = v;
                }
            }
            let played: Vec<&PowerCard> = l.plays.iter().filter(|p| !p.innate).filter_map(|p| defs.cards.get(&p.power)).collect();
            let elements = elements_for(defs, &s2, &played);
            for inn in &all_innates {
                let (g, pu) = dahan_moves(defs, &inn.id);
                let reached = inn.tiers.iter().any(|tier| !tier.modifier && elements.satisfies(&tier.elements));
                let fast = inn.speed == Speed::Fast || may_be_fast(inn, &elements);
                if !(g || pu) || !reached || !fast || l.plays.iter().any(|p| p.power == inn.id) {
                    continue;
                }
                let base = Play { power: inn.id.clone(), target: None, fast: true, innate: true, cost: 0, places: false, forced: inn.speed != Speed::Fast, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None };
                let mut best: Option<(f64, Play)> = None;
                for tg in innate_targets_reach(inn, &s2, &t.before_fast, &graph, rules.presence_with_dahan_for(&spirit)) {
                    let plays = &l.plays;
                    let (v, cand) = best_moves(&fast_island, defs, &base, &tg, &|c: &Play| {
                        let mut tr = plays.clone();
                        tr.push(c.clone());
                        score_of(&tr)
                    });
                    if best.as_ref().is_none_or(|(b, _)| v > *b) {
                        best = Some((v, cand));
                    }
                }
                if let Some((v, cand)) = best {
                    if v > cur + 1e-9 {
                        l.plays.push(cand);
                        cur = v;
                    }
                }
            }
        }
        // the ranked lines are scored one turn deeper in both modes; the actual line's Slow
        // gap with it
        for l in top.iter_mut() {
            let s2 = spirit_turn_at(l);
            let (cards, inns): (Vec<Play>, Vec<Play>) = l.plays.iter().cloned().partition(|p| !p.innate);
            let out = simulate(&cards, &inns, &s2, None, None, false, &[], true);
            l.score = out.score(&values) + l.growth_value;
            if l.actual && !ended_before_slow && l.plays.iter().any(|p| !p.fast) {
                let o = simulate(&clear_slow_targets(&cards), &clear_slow_targets(&inns), &s2, None, None, false, &[], true);
                l.slow_gap = (o.score(&values) + l.growth_value - l.score).max(0.0);
                l.slow_best = o.slow;
            }
            l.outcome = out;
        }
        let score_under = |l: &Line, explore: Option<&[String]>| -> f64 {
            let pos = &positions[l.position.min(positions.len() - 1)];
            let s2 = spirit_turn_at(l);
            let (cards, inns): (Vec<Play>, Vec<Play>) = l.plays.iter().cloned().partition(|p| !p.innate);
            simulate(&cards, &inns, &s2, None, explore, true, &pos.hindsight_cards, true).score(&values) + l.growth_value
        };
        let per_card: Vec<Vec<f64>> = top.iter().map(|l| if cands.is_empty() { vec![score_under(l, None)] } else { cands.iter().map(|c| score_under(l, Some(c))).collect() }).collect();
        let actual_idx = top.iter().position(|l| l.actual);
        let actual_per_card: Vec<f64> = actual_idx.map(|a| per_card[a].clone()).unwrap_or_default();
        // the whole hidden effect on the actual line: its hindsight score (best Slow targets)
        // minus its informed score under the Explore card that came
        let hidden_effect: Option<f64> = actual_idx.and_then(|a| {
            let i = if cands.is_empty() { Some(0) } else { cands.iter().position(|c| *c == t.explore) };
            i.and_then(|i| per_card[a].get(i).copied()).map(|under_real| top[a].score + top[a].slow_gap - under_real)
        });
        for (i, l) in top.iter_mut().enumerate() {
            let mean = per_card[i].iter().sum::<f64>() / per_card[i].len() as f64 + expected_exo;
            l.informed = mean;
            l.expected = expected_exo;
            l.per_card = per_card[i].clone();
            l.luck = l.score + l.slow_gap - mean;
            // how sure the line beats the actual one: under each possible Explore card, the
            // chance the gap outweighs the simulator's own error (two scores, each off by
            // `model_sd`), averaged over the cards; a plain count of cards when no error is measured
            l.confidence = match actual_idx {
                Some(a) if a != i => {
                    let noise = cal.model_sd * std::f64::consts::SQRT_2;
                    let per: Vec<f64> = per_card[i].iter().zip(&per_card[a]).map(|(x, y)| if noise > 0.05 { normal_cdf((x - y) / noise) } else if x >= y { 1.0 } else { 0.0 }).collect();
                    if per.is_empty() || (noise <= 0.05 && cands.len() <= 1) { None } else { Some(per.iter().sum::<f64>() / per.len() as f64) }
                }
                _ => None,
            };
        }
        // a play whose land changes nothing is played for its elements: the line scores the
        // same with its effect left out, in hindsight and under every Explore card that could
        // have come. Marked, so the line does not name a land for it; no score moves
        for l in top.iter_mut() {
            let s2 = spirit_turn_at(l);
            let pos = &positions[l.position.min(positions.len() - 1)];
            let scores_of = |plays: &[Play]| -> Vec<f64> {
                let (cards, inns): (Vec<Play>, Vec<Play>) = plays.iter().cloned().partition(|p| !p.innate);
                let mut v = vec![simulate(&cards, &inns, &s2, None, None, false, &[], false).score(&values)];
                if cands.is_empty() {
                    v.push(simulate(&cards, &inns, &s2, None, None, true, &pos.hindsight_cards, false).score(&values));
                }
                for c in &cands {
                    v.push(simulate(&cards, &inns, &s2, None, Some(c), true, &pos.hindsight_cards, false).score(&values));
                }
                v
            };
            let full = scores_of(&l.plays);
            for i in 0..l.plays.len() {
                let p = &l.plays[i];
                // a card the simulator cannot read in full may do what it does not see
                let readable = defs.cards.get(&p.power).is_none_or(|c| c.fully_parsed() && c.not_simulated().is_empty());
                if p.inert || p.places || p.target.is_none() || !readable {
                    continue;
                }
                let mut trial = l.plays.clone();
                trial[i].inert = true;
                if scores_of(&trial).iter().zip(&full).all(|(a, b)| (a - b).abs() < 1e-9) {
                    l.plays[i].inert = true;
                }
            }
            // an innate that changes nothing is an innate not used (it has no elements to give)
            l.plays.retain(|p| !(p.inert && p.innate));
        }
        top.sort_by(|a, b| b.informed.partial_cmp(&a.informed).unwrap().then(b.score.partial_cmp(&a.score).unwrap()).then(b.actual.cmp(&a.actual)));
        // lines with the same growth, the same cards and innates and the same two scores
        // differ only in a target that changes nothing: one row, `+N alike`
        let same_powers = |l: &Line| -> (Vec<String>, Vec<(String, bool, bool)>) {
            let mut k: Vec<(String, bool, bool)> = l.plays.iter().map(|p| (p.power.clone(), p.fast, p.innate)).collect();
            k.sort();
            (l.growth.clone(), k)
        };
        let mut folded: Vec<Line> = Vec::new();
        for l in top {
            if let Some(prev) = folded.iter_mut().find(|p| !p.actual && !l.actual && (p.informed - l.informed).abs() < 1e-6 && (p.score - l.score).abs() < 1e-6 && same_powers(p) == same_powers(&l)) {
                prev.ties += 1 + l.ties;
                continue;
            }
            folded.push(l);
        }
        let lines = folded;
        let actual_line = lines.iter().find(|l| l.actual).cloned().unwrap_or(actual_line);
        let best_hindsight = lines.iter().map(|l| l.score).fold(actual_line.score, f64::max);
        let best_informed = lines.iter().map(|l| l.informed).fold(actual_line.informed, f64::max);
        let gap = best_hindsight - actual_line.score;
        let decision = (best_informed - actual_line.informed).max(0.0);
        let best_same_growth = lines.iter().filter(|l| l.growth_actual).map(|l| l.informed).fold(actual_line.informed, f64::max);
        let gap_plays = (best_same_growth - actual_line.informed).max(0.0);
        let gap_growth = (best_informed - best_same_growth).max(0.0);
        let best_growth: Vec<String> = lines.first().map(|l| l.growth.clone()).unwrap_or_default();
        let hidden: Vec<String> = {
            let mut h = Vec::new();
            if !t.explore.is_empty() {
                let others: Vec<String> = cands.iter().map(|c| c[0].clone()).filter(|c| *c != t.explore[0]).collect();
                if others.is_empty() {
                    h.push(format!("Explore: {}, the last card of its stage (knowable)", t.explore.join(", ")));
                } else {
                    h.push(format!("Explore: {} face down; could have been {} ({} left in Stage {})", t.explore.join(", "), others.join(", "), cands.len(), defs.invader_cards.get(&t.explore[0]).map(|c| c.stage).unwrap_or(0)));
                }
            }
            if !t.events.is_empty() {
                h.push(format!("Event: {}", t.events.join(", ")));
            }
            if t.fear_cards_earned > 0 {
                h.push(format!("Fear: {} card(s) resolved", t.fear_cards_earned));
            }
            match (hidden_effect, exogenous) {
                (Some(x), Some((pieces, n))) => h.push(format!("Effects: the Event and Fear cards were worth {x:+.2} to your line against {expected_exo:+.2} expected (pieces {pieces:+.2} over {n} land(s); the rest Defends, skips and the next turn's cards); applied from the log to every line")),
                (Some(x), None) => h.push(format!("Effects: the Event and Fear cards were worth {x:+.2} to your line against {expected_exo:+.2} expected")),
                (None, _) if !actual_readable => h.push("Effects: not applied; the actual line plays a card the simulator cannot read in full".into()),
                _ => {}
            }
            if !s.offered.is_empty() {
                h.push(format!("Draft: {}; each tried as the card taken", s.offered.join(", ")));
            }
            h
        };
        let mut tnotes: Vec<String> = Vec::new();
        // step confidence: a line that plays a card the simulator cannot read in full is not judged
        let unread = |l: &Line| -> Vec<String> { l.plays.iter().filter(|p| !p.innate).filter_map(|p| defs.cards.get(&p.power)).filter(|c| !c.fully_parsed()).map(|c| c.title.clone()).collect() };
        let unread_actual = unread(&actual_line);
        let unread_best = lines.first().map(unread).unwrap_or_default();
        let shared = |l: &Line| -> Vec<Play> { l.plays.iter().filter(|p| !p.innate && defs.cards.get(&p.power).is_some_and(|c| !c.fully_parsed())).cloned().collect() };
        let same_unread = lines.first().is_some_and(|b| shared(b) == shared(&actual_line));
        let mut unread_all: Vec<String> = unread_actual.iter().chain(unread_best.iter()).cloned().collect();
        unread_all.sort();
        unread_all.dedup();
        if !unread_all.is_empty() && same_unread {
            tnotes.push(format!("{} not fully readable; it sits in both lines at the same target, so the comparison stands", unread_all.join(", ")));
        }
        {
            let mut typed_only: Vec<String> = Vec::new();
            for l in std::iter::once(&actual_line).chain(lines.first()) {
                for p in l.plays.iter().filter(|p| !p.innate) {
                    if let Some(c) = defs.cards.get(&p.power) {
                        let ns = c.not_simulated();
                        if !ns.is_empty() {
                            let line = c.title.clone();
                            if !typed_only.contains(&line) {
                                typed_only.push(line);
                            }
                        }
                    }
                }
            }
            if !typed_only.is_empty() {
                tnotes.push(format!("not simulated: {} (text the simulator types but does not execute; si cards --card shows it)", typed_only.join(", ")));
            }
        }
        let where_from = |gap_growth: f64, gap_plays: f64| -> String {
            if gap_growth > 0.25 && gap_plays > 0.25 {
                format!("{gap_growth:.1} of it from the growth, {gap_plays:.1} from the plays")
            } else if gap_growth > 0.25 {
                "from the growth".to_string()
            } else {
                "from the plays, with the Ravage and Build cards already face up".to_string()
            }
        };
        // a land-targeting card paid for but never used: un-played after an undo, or a use the
        // log dropped; a Slow card the game ended before is simply unresolved
        let untargeted: Vec<String> = actual_plays
            .iter()
            .filter(|p| p.target.is_none() && defs.cards.get(&p.power).is_some_and(|c| c.range.is_some()) && !(ended_before_slow && !p.fast))
            .map(|p| defs.cards.get(&p.power).map(|c| c.title.clone()).unwrap_or(p.power.clone()))
            .collect();
        // bands as effect sizes against a turn's usual luck (Cohen: 0.2 small, 0.8 large)
        let sd = cal.luck_sd.max(0.25);
        let effect = decision / sd;
        let reading = if !untargeted.is_empty() {
            format!("Not judged: the log shows {} played but never used (un-played after an undo, or a use the log dropped), so the actual line is incomplete", untargeted.join(", "))
        } else if !unread_all.is_empty() && !same_unread {
            format!("Not judged: the simulator cannot read {} in full yet (see si cards --card), so the scores on this turn are not trustworthy", unread_all.join(", "))
        } else if effect < 0.2 {
            if actual_line.luck <= -sd {
                format!("Unlucky: the right play on what you knew (within {decision:.1} of the best informed line); the hidden cards cost {:.1}, more than a turn's usual swing of {sd:.1}", -actual_line.luck)
            } else {
                format!("Sound: within {decision:.1} of the best line on what was knowable, under a fifth of a turn's usual swing ({sd:.1})")
            }
        } else if actual_line.score >= best_hindsight - 0.2 * sd {
            format!("Gamble paid: {decision:.1} behind the percentage play on what was known, but the hidden cards made it the best line")
        } else if effect < 0.8 {
            format!("Better available on what was known: about {decision:.1} in ledger value ({effect:.1} of a turn's usual swing), {}", where_from(gap_growth, gap_plays))
        } else {
            format!("Mistake on what was known: {decision:.1} in ledger value ({effect:.1} times a turn's usual swing of {sd:.1}), {}", where_from(gap_growth, gap_plays))
        };
        if actual_line.slow_gap > 0.25 {
            tnotes.push(format!("Slow targets: {:.1} better was open after the Explore ({}); counted as its own gap, not as luck", actual_line.slow_gap, actual_line.slow_best.iter().map(|p| match &p.target { Some(l) => format!("{}>{l}", p.power), None => p.power.clone() }).collect::<Vec<_>>().join(" ")));
        }
        if ended_before_slow && rec.meta.ending.is_some() && t.turn == rec.meta.turns {
            tnotes.push("the game ended in this turn's Invader phase: Slow plays and the lookahead are not scored for any line".into());
        }
        if s.cards_played.len() as i64 > s.plays.unwrap_or(i64::MAX) {
            tnotes.push(format!("{} cards recorded against {} plays: an undo re-logged a play; the actual line is read as recorded", s.cards_played.len(), s.plays.unwrap_or(0)));
        }
        let unparsed: usize = actual_line.outcome.notes.iter().filter(|n| n.starts_with("unparsed")).count();
        if unparsed > 0 {
            tnotes.push(format!("{unparsed} card sentence(s) the simulator could not read in the actual line"));
        }
        if positions.len() == 1 && enumerate_growth && t.turn > 1 {
            tnotes.push("no growth alternative found from the previous turn's position (first turn, or a position the record does not carry)".into());
        }
        // the choices the engine asked for: Event and Fear lands, Power branches, cascade targets
        let mut choices: Vec<ChoiceReview> = Vec::new();
        for (ci, c) in t.choices.iter().enumerate() {
            let kind = source_kind_of(defs, c);
            let reviewed = matches!(kind.as_str(), "Event" | "Fear" | "Blight") || (kind == "Power" && c.kind == "Effect");
            if !reviewed || c.kind == "Card" || c.choices.len() < 2 {
                continue;
            }
            let land_name = |o: &str| o.parse::<LandId>().map(|l| l.to_string()).unwrap_or_else(|_| o.to_string());
            let selected = land_name(&c.selected);
            let mut options: Vec<(String, Option<f64>)> = Vec::new();
            let mut actual_score = None;
            let note;
            if let Some((_, _, e)) = scorable.iter().find(|(i, _, _)| *i == ci) {
                for opt in &c.choices {
                    let Ok(land) = opt.parse::<LandId>() else {
                        options.push((opt.clone(), None));
                        continue;
                    };
                    let out = simulate(&actual_plays, &actual_innates, s, Some((ci, &land)), None, false, &[], false);
                    let score = out.score(&values) + actual_gv;
                    if land.to_string() == selected {
                        actual_score = Some(score);
                    }
                    options.push((land.to_string(), Some(score)));
                }
                note = format!("{} placed in each land in turn, the actual growth and plays kept, then the turn's Invader steps", e.words());
            } else {
                options = c.choices.iter().map(|o| (land_name(o), None)).collect();
                note = format!("not scored: the effect behind {} is not modeled yet; the options are listed", c.reason);
            }
            options.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let best_opt = options.iter().filter_map(|(_, s)| *s).fold(None, |m: Option<f64>, x| Some(m.map_or(x, |m| m.max(x))));
            let gap = match (actual_score, best_opt) {
                (Some(a), Some(b)) => Some(b - a),
                _ => None,
            };
            let review = ChoiceReview { phase: c.phase, source: c.source.clone(), source_kind: kind, reason: c.reason.clone(), selected, optional: c.optional, options, actual_score, gap, note };
            // the same prompt re-logged across an undo, or asked once per board: once is enough
            if !choices.iter().any(|x: &ChoiceReview| x.source == review.source && x.reason == review.reason && x.selected == review.selected && x.options.iter().map(|o| &o.0).eq(review.options.iter().map(|o| &o.0))) {
                choices.push(review);
            }
        }
        let hand_ids: Vec<String> = {
            let mut v = actual_pos.hand.clone();
            v.sort();
            v.dedup();
            v
        };
        on_progress(Progress::Done { turn: t.turn, lines: lines.len(), explore_cards: cands.len().max(1), seconds: started.elapsed().as_secs_f64() });
        let actual_luck = actual_line.luck;
        // calibration material: the next turn's threat with no Spirit action against what it took
        let (next_threat, next_actual) = match next_turn {
            Some(n) if !n.before_fast.is_empty() => {
                let mut isl = verify::island_for(rec, defs, &n.before_fast);
                let mut probe = Outcome::default();
                invader_phase(&mut isl, rec, n, defs, &rules, &mut probe, None, false);
                let visits = || rec.lands.values().flat_map(|l| l.visits.iter()).filter(|v| v.turn == n.turn && matches!(v.step, Some(si_model::event::InvaderStep::Ravage) | Some(si_model::event::InvaderStep::Build)));
                let blight: u32 = visits().map(|v| v.blight + v.cascade).sum();
                let dahan: u32 = visits().map(|v| v.dahan_lost).sum();
                (Some((probe.blight, probe.dahan_lost)), Some((blight, dahan)))
            }
            _ => (None, None),
        };
        let sample = Sample { event: !t.events.is_empty(), fear_cards: fear_k, terror: tl_start, exogenous: exogenous.map(|(x, _)| x), hidden_effect, per_card: actual_per_card, next_threat, next_actual, model_error };
        // the deck from this Fast phase, on the board as this turn's face-up Ravage and Build leave it
        let forecast = {
            let mut isl = verify::island_at(rec, defs, t, "before_fast");
            let mut probe = Outcome::default();
            invader_phase(&mut isl, rec, t, defs, &rules, &mut probe, Some(&[]), false);
            let this: Vec<String> = cands.iter().map(|c| c[0].clone()).collect();
            deck::forecast(rec, defs, &deck, t, &isl, &rules, 3, Some(&this))
        };
        turns.push(TurnReview {
            turn: t.turn,
            spirit: spirit.clone(),
            hand: hand_ids,
            plays: actual_pos.plays,
            energy: actual_pos.energy,
            actual: actual_line,
            ranked: lines,
            candidates: count,
            gap,
            reading,
            hidden,
            notes: tnotes,
            positions: positions.len(),
            gap_plays,
            gap_growth,
            best_growth,
            decision,
            luck: actual_luck,
            offered: s.offered.clone(),
            choices,
            exogenous,
            expected_exogenous: expected_exo,
            hidden_effect,
            sample,
            forecast,
            clocks,
        });
    }
    Review { values, turns, notes, calibration: cal, deck }
}

/// Event, Fear or what the log said: the card tables decide when the engine's
/// identifier only said "rule".
fn source_kind_of(defs: &Definitions, c: &Choice) -> String {
    let n = norm(&c.source);
    if !n.is_empty() {
        if defs.fear_cards.values().any(|f| norm(&f.title) == n || norm(&f.id) == n) {
            return "Fear".into();
        }
        if defs.event_cards.keys().any(|id| norm(id) == n) {
            return "Event".into();
        }
    }
    c.source_kind.clone()
}

fn norm(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase()
}

/// A piece the simulator can add or remove in a land.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Explorers,
    Towns,
    Dahan,
    Blight,
    Beasts,
    Disease,
    Wilds,
    Strife,
}

impl Field {
    fn get_mut(self, c: &mut Counts) -> &mut u32 {
        match self {
            Field::Explorers => &mut c.explorers,
            Field::Towns => &mut c.towns,
            Field::Dahan => &mut c.dahan,
            Field::Blight => &mut c.blight,
            Field::Beasts => &mut c.beasts,
            Field::Disease => &mut c.disease,
            Field::Wilds => &mut c.wilds,
            Field::Strife => &mut c.strife,
        }
    }

    fn word(self) -> &'static str {
        match self {
            Field::Explorers => "Explorer",
            Field::Towns => "Town",
            Field::Dahan => "Dahan",
            Field::Blight => "Blight",
            Field::Beasts => "Beasts",
            Field::Disease => "Disease",
            Field::Wilds => "Wilds",
            Field::Strife => "Strife",
        }
    }
}

/// The effect a recorded Event or Fear choice placed in a land, as far as the
/// record lets the simulator move it to another land.
#[derive(Debug, Clone, PartialEq)]
enum ChoiceEffect {
    /// Damage to Invaders; `per_dahan` when the card deals it per Dahan in the land.
    Damage { n: u32, per_dahan: bool },
    Add(Field),
    Remove(Field),
    Isolate,
}

impl ChoiceEffect {
    fn words(&self) -> String {
        match self {
            ChoiceEffect::Damage { n, per_dahan: true } => format!("{n} Damage (1 per Dahan in the land)"),
            ChoiceEffect::Damage { n, per_dahan: false } => format!("{n} Damage"),
            ChoiceEffect::Add(f) => format!("1 {} added", f.word()),
            ChoiceEffect::Remove(f) => format!("1 {} destroyed", f.word()),
            ChoiceEffect::Isolate => "the land Isolated".into(),
        }
    }
}

fn choice_effect(rec: &GameRecord, defs: &Definitions, t: &TurnRecord, c: &Choice, land: &LandId) -> Option<ChoiceEffect> {
    match c.reason.as_str() {
        "DealDamage" => {
            let src = norm(&c.source);
            let per_dahan = defs.fear_cards.values().find(|f| norm(&f.title) == src || norm(&f.id) == src).is_some_and(|f| {
                let b = f.body.join(" ").to_lowercase();
                b.contains("dahan") && b.contains("damage") && (b.contains("per ") || b.contains("each "))
            });
            let mut n: u32 = rec.lands.values().find(|l| l.land == *land).map(|l| l.touches.iter().filter(|x| x.turn == t.turn && norm(&x.by) == src).filter_map(|x| if let TouchKind::Damage(n) = x.kind { Some(n) } else { None }).sum()).unwrap_or(0);
            if n == 0 && per_dahan {
                n = t.before_fast.get(land).map(|c| c.dahan).unwrap_or(0);
            }
            if n == 0 {
                return None;
            }
            Some(ChoiceEffect::Damage { n, per_dahan })
        }
        "AddBlight" => Some(ChoiceEffect::Add(Field::Blight)),
        "AddingTown" => Some(ChoiceEffect::Add(Field::Towns)),
        "AddingDahan" => Some(ChoiceEffect::Add(Field::Dahan)),
        "AddingBeast" => Some(ChoiceEffect::Add(Field::Beasts)),
        "AddingDisease" => Some(ChoiceEffect::Add(Field::Disease)),
        "AddingWild" => Some(ChoiceEffect::Add(Field::Wilds)),
        "AddingStrife" => Some(ChoiceEffect::Add(Field::Strife)),
        "DestroyExplorer" => Some(ChoiceEffect::Remove(Field::Explorers)),
        "IsolateLand" => Some(ChoiceEffect::Isolate),
        _ => None,
    }
}

fn apply_choice_effect(island: &mut Island, rules: &mut Rules, land: &LandId, e: &ChoiceEffect, ctx: &Ctx, out: &mut Outcome) {
    match e {
        ChoiceEffect::Damage { n, per_dahan } => {
            let amount = if *per_dahan { island.lands.get(land).map(|l| l.c.dahan).unwrap_or(0) } else { *n };
            if amount == 0 {
                return;
            }
            let a = effects::apply(&[Effect::Damage { amount: amount.min(255) as u8, to: DamageTarget::Invaders }], &[], island, land, ctx);
            out.fear += a.fear;
            for k in a.destroyed {
                match k {
                    PieceKind::Explorer => out.explorers_destroyed += 1,
                    PieceKind::Town => {
                        out.towns_destroyed += 1;
                        out.fear += 1;
                    }
                    PieceKind::City => {
                        out.cities_destroyed += 1;
                        out.fear += 2;
                    }
                    PieceKind::Dahan => {}
                }
            }
        }
        ChoiceEffect::Add(f) => {
            if let Some(l) = island.lands.get_mut(land) {
                *f.get_mut(&mut l.c) += 1;
            }
        }
        ChoiceEffect::Remove(f) => {
            if let Some(l) = island.lands.get_mut(land) {
                let v = f.get_mut(&mut l.c);
                if *v > 0 {
                    *v -= 1;
                    if *f == Field::Explorers {
                        out.explorers_destroyed += 1;
                    }
                }
            }
        }
        ChoiceEffect::Isolate => rules.isolated.push(land.clone()),
    }
}

/// Per land, the logged pieces at the first Invader step minus the simulated
/// ones after the Fast phase: the Event and Fear effects (and any simulator
/// error on the actual line), applied to every line alike.
#[derive(Debug, Clone, Default, PartialEq)]
struct Delta(BTreeMap<LandId, [i64; 11]>, BTreeMap<LandId, Vec<(PieceKind, u32)>>);

fn counts_vec(c: &Counts) -> [i64; 11] {
    [c.explorers, c.towns, c.cities, c.dahan, c.blight, c.presence, c.beasts, c.disease, c.strife, c.wilds, c.badlands].map(|x| x as i64)
}

fn delta_between(sim: &Island, logged: &BTreeMap<LandId, Counts>) -> Delta {
    let mut d = Delta::default();
    let lands: BTreeSet<LandId> = sim.lands.keys().cloned().chain(logged.keys().cloned()).collect();
    for l in lands {
        let a = sim.lands.get(&l).map(|x| counts_vec(&x.c)).unwrap_or([0; 11]);
        let b = logged.get(&l).map(counts_vec).unwrap_or([0; 11]);
        let mut diff = [0i64; 11];
        let mut any = false;
        for i in 0..11 {
            diff[i] = b[i] - a[i];
            any |= diff[i] != 0;
        }
        if any {
            d.0.insert(l, diff);
        }
    }
    d
}

/// The ledger value of a piece delta (logged minus simulated): Blight added, Dahan and
/// presence lost and Invaders added count against; Invaders removed count for. Tokens
/// are not valued.
fn delta_value(d: &Delta, v: &Values) -> f64 {
    let mut total = 0.0;
    for diff in d.0.values() {
        let [e, t, c, dahan, blight, presence, ..] = *diff;
        total += blight as f64 * v.blight;
        total += (-dahan) as f64 * v.dahan;
        total += (-presence) as f64 * v.presence;
        total += (-e) as f64 * v.explorer + (-t) as f64 * v.town + (-c) as f64 * v.city;
    }
    total
}

/// The residual's law: no kind grows by more than the change adds, none shrinks by more than
/// it takes, and the presence it reports lost is what left the board. A debug build checks
/// it on every application; `tests::the_residual_adds_and_takes_no_more_than_it_says` fuzzes it.
#[cfg_attr(not(any(test, debug_assertions)), allow(dead_code))]
fn delta_holds(before: &Island, d: &Delta, after: &Island, presence_lost: u32) -> Vec<si_model::oracle::Violation> {
    use si_model::oracle::check;
    let mut out = Vec::new();
    let total = |isl: &Island, i: usize| -> i64 { isl.lands.values().map(|l| counts_vec(&l.c)[i]).sum() };
    for i in 0..11 {
        let up: i64 = d.0.values().map(|x| x[i].max(0)).sum();
        let down: i64 = d.0.values().map(|x| (-x[i]).max(0)).sum();
        let (t0, t1) = (total(before, i), total(after, i));
        check(&mut out, t1 <= t0 + up, "the residual adds no more than it says", || format!("kind {i}: {t0} -> {t1}, {up} added"));
        check(&mut out, t1 >= t0 - down, "the residual takes no more than it says", || format!("kind {i}: {t0} -> {t1}, {down} taken"));
    }
    let lost: i64 = before.lands.iter().map(|(id, l)| (l.c.presence as i64 - after.lands.get(id).map(|x| x.c.presence as i64).unwrap_or(0)).max(0)).sum();
    check(&mut out, presence_lost as i64 == lost, "the presence lost is what left", || format!("{presence_lost} reported, {lost} left"));
    out
}

/// The Event and Fear steps' change on the island, land by land; the presence it took, so the
/// line counts it lost.
fn apply_delta(island: &mut Island, d: &Delta) -> u32 {
    #[cfg(debug_assertions)]
    let before = island.clone();
    let lost = apply_delta_unchecked(island, d);
    #[cfg(debug_assertions)]
    {
        let broken = delta_holds(&before, d, island, lost);
        debug_assert!(broken.is_empty(), "the residual broke its law: {broken:?}");
    }
    lost
}

fn apply_delta_unchecked(island: &mut Island, d: &Delta) -> u32 {
    // the Damage those steps left stays on the pieces until Time Passes
    for (id, ws) in &d.1 {
        if let Some(l) = island.lands.get_mut(id) {
            l.wounds.extend(ws.iter().copied());
        }
    }
    fn field(c: &mut Counts, i: usize) -> &mut u32 {
        match i {
            0 => &mut c.explorers,
            1 => &mut c.towns,
            2 => &mut c.cities,
            3 => &mut c.dahan,
            4 => &mut c.blight,
            5 => &mut c.presence,
            6 => &mut c.beasts,
            7 => &mut c.disease,
            8 => &mut c.strife,
            9 => &mut c.wilds,
            _ => &mut c.badlands,
        }
    }
    // the removals first; what the line did not have to remove is a move that did not happen
    // here, so as much is not added where the record's pieces went (no piece from nowhere)
    let mut short = [0i64; 11];
    let mut presence_lost = 0;
    for (l, diff) in &d.0 {
        let Some(land) = island.lands.get_mut(l) else { continue };
        for (i, x) in diff.iter().enumerate().filter(|(_, x)| **x < 0) {
            let f = field(&mut land.c, i);
            let took = (*f as i64).min(-x);
            short[i] += -x - took;
            *f -= took as u32;
            if i == 5 {
                presence_lost += took as u32;
            }
        }
    }
    for (l, diff) in &d.0 {
        let Some(land) = island.lands.get_mut(l) else { continue };
        for (i, x) in diff.iter().enumerate().filter(|(_, x)| **x > 0) {
            let skip = short[i].min(*x);
            short[i] -= skip;
            *field(&mut land.c, i) += (x - skip) as u32;
        }
    }
    for l in island.lands.values_mut() {
        l.trim_wounds();
    }
    presence_lost
}

/// Plays in notation (`SLSF>C8 MI GP2@A2`): a card or innate by id, title or short code,
/// `>land` or `@land` for its target.
pub fn parse_plays(defs: &Definitions, text: &str) -> Result<Vec<Play>, String> {
    let mut codes = si_record::notation::Codes::new(Some(defs), false);
    let mut by_code: BTreeMap<String, String> = BTreeMap::new();
    for id in defs.cards.keys().chain(defs.spirits.values().flat_map(|p| p.innates.iter().map(|i| &i.id))) {
        by_code.insert(codes.code(id).to_uppercase(), id.clone());
    }
    let resolve = |name: &str| -> Option<String> {
        if defs.cards.contains_key(name) || defs.spirits.values().flat_map(|p| p.innates.iter()).any(|i| i.id == name) {
            return Some(name.to_string());
        }
        if let Some(c) = defs.card_by_title(name) {
            return Some(c.id.clone());
        }
        by_code.get(&name.to_uppercase()).cloned()
    };
    // a play's bracket may hold spaces (`GW>A1[<A3 <A5 >A4]`, or commas): words are joined until it closes
    let mut tokens: Vec<String> = Vec::new();
    for w in text.split_whitespace() {
        match tokens.last_mut() {
            Some(last) if last.contains('[') && !last.contains(']') => {
                last.push(' ');
                last.push_str(w);
            }
            _ => tokens.push(w.to_string()),
        }
    }
    let mut plays: Vec<Play> = Vec::new();
    for tok in &tokens {
        // `(WW)`: played for its elements, on no land that matters
        let (tok, inert) = match tok.strip_prefix('(').and_then(|t| t.strip_suffix(')')) {
            Some(inner) => (inner, true),
            None => (tok.as_str(), false),
        };
        let (tok, bracket) = match tok.split_once('[') {
            Some((head, rest)) => (head, Some(rest.trim_end_matches(']'))),
            None => (tok, None),
        };
        // `F:` takes a Power printed Slow in the Fast phase: an innate through its "may be Fast"
        // tier, or any Power another play in the line makes Fast (Slip the Flow of Time);
        // `S:` is its printed phase said out loud
        let (phase, tok) = match tok.get(..2).map(|p| p.to_ascii_uppercase()) {
            Some(p) if p == "F:" => (Some(true), &tok[2..]),
            Some(p) if p == "S:" => (Some(false), &tok[2..]),
            _ => (None, tok),
        };
        let (name, target) = match tok.split_once('>').or_else(|| tok.split_once('@')) {
            Some((n, l)) => (n, Some(l.parse::<LandId>().map_err(|e| e.to_string())?)),
            None => (tok, None),
        };
        let id = resolve(name).ok_or(format!("unknown Power {name}"))?;
        let innate = !defs.cards.contains_key(&id);
        let (printed_fast, cost) = match defs.cards.get(&id) {
            Some(c) => (c.speed == Speed::Fast, c.cost),
            None => (defs.spirits.values().flat_map(|p| p.innates.iter()).find(|i| i.id == id).is_some_and(|i| i.speed == Speed::Fast), 0),
        };
        let (fast, forced) = match phase {
            Some(true) => (true, !printed_fast),
            Some(false) if printed_fast => return Err(format!("{name} is Fast as printed; a Fast Power is not taken Slow")),
            _ => (printed_fast, false),
        };
        // `@` is a Spirit-target card's presence land; a land-target card that adds presence targets it (`>`)
        let places = defs.cards.get(&id).is_some_and(|c| c.target.is_spirit() && adds_presence(c));
        let mut moves: Vec<Move> = Vec::new();
        let mut kills: Option<Vec<(LandId, si_model::power::PieceFilter)>> = None;
        if let Some(b) = bracket {
            // a Power on a Spirit (no land) writes its moves as whole paths (`Boon[C4>C7:Pr]`)
            let tg_opt = target.clone();
            let need_land = || format!("{name}: a gather or push needs the Power's land first (`{name}>A1[<A3,>A4]`), or a whole path (`{name}[A3>A4]`)");
            let tg = tg_opt.clone().unwrap_or_else(|| LandId::new("?", 0));
            // a bare move is a Dahan's when the Power moves Dahan that way, else the Power's own kind
            let (gathers_dahan, pushes_dahan) = dahan_moves(defs, &id);
            for w in b.split([' ', ',']).filter(|w| !w.is_empty()) {
                // `-T`, `-C3:Ex2`: pieces it destroys, by kind, in its land or the one named
                if let Some(k) = w.strip_prefix('-') {
                    if k.is_empty() {
                        // `-`: it destroys nothing where the player chooses
                        kills.get_or_insert_with(Vec::new);
                        continue;
                    }
                    let (k, n) = match k.rsplit_once('x') {
                        Some((h, c)) if !c.is_empty() && c.chars().all(|d| d.is_ascii_digit()) => (h, c.parse::<usize>().unwrap_or(1)),
                        _ => (k, 1),
                    };
                    let (land, code) = match k.split_once(':') {
                        Some((l, c)) => (l.parse::<LandId>().map_err(|e| e.to_string())?, c),
                        None if tg_opt.is_some() => (tg.clone(), k),
                        None => return Err(need_land()),
                    };
                    let piece = effects::piece_of_code(code).ok_or(format!("{name}: `{code}` is not a piece; E, T, C, Dh, Be, Di, St, Wi, Ba"))?;
                    kills.get_or_insert_with(Vec::new).extend(std::iter::repeat_n((land, piece), n));
                    continue;
                }
                let (w, kind) = match w.split_once(':') {
                    Some((h, k)) => (h, Some(effects::piece_of_code(k).ok_or(format!("{name}: `{k}` is not a piece; E, T, C, Dh, Be, Di, St, Wi, Ba"))?)),
                    None => (w, None),
                };
                let (w, n) = match w.rsplit_once('x') {
                    Some((h, k)) if k.chars().all(|c| c.is_ascii_digit()) && !k.is_empty() => (h, k.parse::<u32>().unwrap_or(1)),
                    _ => (w, 1),
                };
                let (from, to) = if let Some(l) = w.strip_prefix('<') {
                    if tg_opt.is_none() {
                        return Err(need_land());
                    }
                    (l.parse::<LandId>().map_err(|e| e.to_string())?, tg.clone())
                } else if let Some(l) = w.strip_prefix('>') {
                    if tg_opt.is_none() {
                        return Err(need_land());
                    }
                    (tg.clone(), l.parse::<LandId>().map_err(|e| e.to_string())?)
                } else if let Some((f, t)) = w.split_once('>') {
                    (f.parse::<LandId>().map_err(|e| e.to_string())?, t.parse::<LandId>().map_err(|e| e.to_string())?)
                } else {
                    return Err(format!("{name}: `{w}` is not a move; write `<A3` for a piece gathered in from A3, `>A4` for one pushed out to A4"));
                };
                let dahan = si_model::power::PieceFilter::Piece(PieceKind::Dahan);
                let kind = kind.or_else(|| ((to == tg && gathers_dahan) || (from == tg && pushes_dahan)).then(|| dahan.clone()));
                match moves.iter_mut().find(|m| m.from == from && m.to == to && m.piece == kind) {
                    Some(m) => m.n += n,
                    None => moves.push(Move { from, to, piece: kind, n }),
                }
            }
        }
        // a Power written again is its Repeat: no Energy (`repeat`)
        let cost = if plays.iter().any(|q| q.power == id) { 0 } else { cost };
        // a bracket is the whole of what the Power moves (`GW>A5[]` gathers and pushes nothing)
        plays.push(Play { power: id, target, fast, innate, cost, places, forced, moves, exact: bracket.is_some(), inert, recorded: false, kills });
    }
    Ok(plays)
}

/// The notation's law: plays written out read back, and written again, to the same text, and
/// read back the same plays: the Power, its land, its speed, a Fast mark, an elements-only
/// play, its kills. (That the plays then play the same is the corpus fuzzer's check, which
/// needs a game.)
pub fn notation_holds(defs: &Definitions, plays: &[Play]) -> Vec<si_model::oracle::Violation> {
    use si_model::oracle::{check, Violation};
    let code = |id: &str| id.to_string();
    let text = Line { plays: plays.to_vec(), ..blank_line() }.plays_text_kept(&code);
    let back = match parse_plays(defs, &text) {
        Ok(b) => b,
        Err(e) => return vec![Violation::new("reads back", format!("`{text}`: {e}"))],
    };
    let mut out = Vec::new();
    let again = Line { plays: back.clone(), ..blank_line() }.plays_text_kept(&code);
    check(&mut out, again == text, "writes the same again", || format!("`{text}` reads back as `{again}`"));
    let line = Line { plays: plays.to_vec(), ..blank_line() };
    let order = line.sequence();
    check(&mut out, back.len() == plays.len(), "reads back every play", || format!("{} of {}", back.len(), plays.len()));
    for (b, i) in back.iter().zip(order) {
        let p = &plays[i];
        // an elements-only play acts nowhere: its speed is not the notation's to keep
        let speed = p.inert || (b.fast == p.fast && b.forced == p.forced);
        let same = b.power == p.power && b.target == p.target && speed && b.inert == p.inert && (p.kills.is_none() || b.kills == p.kills);
        check(&mut out, same, "reads back the same play", || format!("{:?} -> {:?}", (&p.power, &p.target, p.fast, p.forced, p.inert, &p.kills), (&b.power, &b.target, b.fast, b.forced, b.inert, &b.kills)));
    }
    out
}

fn blank_line() -> Line {
    Line { growth: vec![], growth_actual: true, growth_value: 0.0, ties: 0, informed: 0.0, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays: vec![], outcome: Default::default(), score: 0.0, expected: 0.0, per_card: vec![], actual: false }
}

pub fn try_line(rec: &GameRecord, defs: &Definitions, turn: u32, text: &str) -> Result<Line, String> {
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let plays = parse_plays(&scenario_defs, text)?;
    try_plays(rec, defs, turn, plays)
}

/// The rules in force on a closed turn: the adversary at its level, the spirits' hooks, the
/// loss clocks as the turn began, and the record's choices (cascades, chokes, Ravage order).
pub(crate) fn turn_rules(rec: &GameRecord, defs: &Definitions, t: &TurnRecord) -> Rules {
    let level = rec.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    Rules { scenario: rec.meta.scenario.clone(), adversary: rec.meta.adversary.clone(), level, spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), clocks: loss_clocks(rec, t.turn, None), cascades: cascades_of(t), chokes: verify::chokes_of(rec, t), turn: t.turn, fear_cards_resolved: Some(if t.fear_cards.is_empty() { t.fear_cards_earned } else { t.fear_cards.len() as u32 }), upgrade_lands: verify::mining_lands_of(t).0, tunnel_lands: verify::mining_lands_of(t).1, disease_builds: verify::disease_builds_of(rec, defs, t), skip_answers: verify::skip_answers_of(t), escalation_lands: verify::escalation_lands_of(t), escalated_elsewhere: verify::escalated_elsewhere_of(t), ravage_order: ravage_order_of(t), terror: terror_at_invaders(rec, t), ..Default::default() }
}

/// What a written line asks beyond the turn as it stood: more Energy than there was, more
/// cards than plays (a Repeat is no card played), a card not in hand, a Repeat its tiers do
/// not reach or a land it may not take. The line is still played, to see what it would do;
/// these are said beside it (`try_plays` puts them in its notes).
pub fn line_checks(rec: &GameRecord, defs: &Definitions, turn: u32, text: &str) -> Vec<String> {
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let Some(s) = rec.spirits.iter().find(|s| s.turn == turn && s.spirit == spirit) else { return Vec::new() };
    parse_plays(&scenario_defs, text).map(|plays| checks_of(rec, &scenario_defs, s, &plays)).unwrap_or_default()
}

fn checks_of(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, plays: &[Play]) -> Vec<String> {
    let cards: Vec<&Play> = repeat::first_uses(plays).into_iter().filter(|p| !p.innate).collect();
    let prev_after = rec.spirits.iter().find(|p| p.spirit == s.spirit && p.turn + 1 == s.turn).map(|p| p.energy_after).unwrap_or(0);
    let energy = energy_available(rec, defs, s, prev_after);
    let cost: i64 = cards.iter().map(|p| p.cost).sum();
    let plays_allowed = s.plays.unwrap_or(cards.len() as i64);
    let mut out = Vec::new();
    if cost > energy {
        out.push(format!("costs {cost} Energy, {energy} available"));
    }
    if cards.len() as i64 > plays_allowed {
        out.push(format!("{} cards against {} plays", cards.len(), plays_allowed));
    }
    let hand: Vec<String> = s.hand_after.iter().chain(s.cards_played.iter()).cloned().collect();
    for c in &cards {
        if !hand.contains(&c.power) {
            out.push(format!("{} was not in hand that turn", c.power));
        }
    }
    let played: Vec<&PowerCard> = cards.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = elements_for(defs, s, &played);
    let graph = si_metrics::reach::island_for(rec, defs);
    out.extend(repeat::holds(defs, plays, &elements, &graph).into_iter().map(|v| v.to_string()));
    out
}

/// A line of plays scored on a closed turn, as `try_line` scores a written one: the plays act
/// in the order given, the Event and Fear cards as they came, the real Invader steps, the
/// next turn. The outcome's `after` is the island as the turn leaves it.
pub fn try_plays(rec: &GameRecord, defs: &Definitions, turn: u32, plays: Vec<Play>) -> Result<Line, String> {
    let scenario_defs = defs.for_scenario(rec.meta.scenario.as_deref());
    let defs: &Definitions = &scenario_defs;
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let values = Values::for_panel(defs.spirits.get(&spirit));
    let t = rec.turns.iter().find(|t| t.turn == turn).ok_or("no such turn")?;
    let s = rec.spirits.iter().find(|s| s.turn == turn && s.spirit == spirit).ok_or("no spirit turn")?;
    if t.before_fast.is_empty() {
        return Err("no Fast-phase state recorded for that turn".into());
    }
    let rules = turn_rules(rec, defs, t);
    let out_notes = checks_of(rec, defs, s, &plays);
    let mut island = verify::island_at(rec, defs, t, "before_fast");
    let played: Vec<&PowerCard> = repeat::first_uses(&plays).into_iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = elements_for(defs, s, &played);
    if let Some(e) = forced_speed_error(defs, &spirit, &plays, &elements) {
        return Err(e);
    }
    let graph = si_metrics::reach::island_for(rec, defs);
    let mut out = Outcome::default();
    // the plays act in the order written: a gather after the push that brought the Dahan
    let mut all = plays.clone();
    let mut armed = Vec::new();
    apply_plays(&mut island, defs, &all, true, &elements, &rules, &spirit, t, &mut armed, &mut out);
    // the Event and Fear cards as they came, as every other line of the turn replays them
    let all_innates_t = innates_of(defs, &spirit);
    let (actual_plays, actual_innates) = recorded_plays(rec, defs, s, t, &all_innates_t);
    let exo = exo_effects(rec, defs, t, s, &rules, &spirit, &values, &actual_plays, &actual_innates);
    let mut rules_t = rules.clone();
    let ctx_exo = Ctx { presence_with_dahan: rules.presence_with_dahan_for(&spirit), ..Ctx::plain(rules.invader_health_bonus(), ElementSet::new()) };
    for (_, land, e) in &exo.scorable {
        apply_choice_effect(&mut island, &mut rules_t, land, e, &ctx_exo, &mut out);
    }
    if let Some(d) = &exo.residual {
        out.presence_destroyed += apply_delta(&mut island, d);
    }
    let rules = rules_t;
    invader_phase(&mut island, rec, t, defs, &rules, &mut out, None, true);
    if out.wins.is_some() {
        // a won game stops: no Slow phase, no next turn
        out.after = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
        out.notes.extend(out_notes);
        let score = out.score(&values);
        return Ok(Line { growth: vec![], growth_actual: true, growth_value: 0.0, ties: 0, informed: score, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays: all, outcome: out, score, expected: 0.0, per_card: vec![], actual: false });
    }
    let next_turn = rec.turns.iter().find(|n| n.turn == turn + 1);
    // a Slow play written without a land takes the best one after the Invader steps
    let innates_all = innates_of(defs, &spirit);
    let slow = {
        let eval = |slow: &[Play]| -> f64 {
            let mut isl = island.clone();
            let mut o = out.clone();
            let mut armed2 = armed.clone();
            apply_plays(&mut isl, defs, slow, false, &elements, &rules, &spirit, t, &mut armed2, &mut o);
            lookahead(&mut isl, rec, next_turn, defs, &rules, &mut o, None, None);
            o.score(&values)
        };
        resolve_slow(&island, defs, &all, false, s, &graph, &innates_all, &eval)
    };
    all.retain(|p| p.fast);
    all.extend(slow.iter().cloned());
    apply_plays(&mut island, defs, &all, false, &elements, &rules, &spirit, t, &mut armed, &mut out);
    victory_after_slow(&island, &rules, t, &mut out);
    out.after = island.lands.iter().map(|(id, l)| (id.clone(), l.c)).collect();
    if out.wins.is_none() {
        lookahead(&mut island, rec, next_turn, defs, &rules, &mut out, None, None);
    }
    out.slow = slow;
    out.notes.extend(out_notes);
    let score = out.score(&values);
    Ok(Line { growth: vec![], growth_actual: true, growth_value: 0.0, ties: 0, informed: score, luck: 0.0, confidence: None, slow_gap: 0.0, slow_best: vec![], position: 0, elements: String::new(), tiers: vec![], plays: all, outcome: out, score, expected: 0.0, per_card: vec![], actual: false })
}

// ---- growth: the position a turn starts from, and every growth the panel allows ----

/// A spirit's position after Growth, before plays: where its presence stands,
/// which slots are revealed, what it holds, what it can spend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub presence: BTreeMap<LandId, u32>,
    pub revealed: BTreeSet<String>,
    pub hand: Vec<String>,
    pub energy: i64,
    pub plays: i64,
    /// The growth in notation tokens (`G2`, `e4@C4`, `cp2@C4`, `Rc`, `+<card id>`, `+1E`).
    pub growth: Vec<String>,
    pub slots: u32,
    pub cards_gained: u32,
    pub energy_gained: i64,
    pub actual: bool,
    /// Cards in this hand that only hindsight knows (taken from a later turn's recorded draft).
    #[serde(default)]
    pub hindsight_cards: Vec<String>,
}

/// A card gained at a growth: which card, from which turn's recorded draft
/// when the turn itself did not gain one, and the card forgotten for a Major.
#[derive(Debug, Clone, PartialEq)]
struct CardPick {
    card: Option<String>,
    draft_turn: Option<u32>,
    forget: Option<String>,
}

/// Two positions that play the same: same presence, same slots, same hand.
pub fn same_position(a: &Position, b: &Position) -> bool {
    let sorted = |h: &Vec<String>| {
        let mut v = h.clone();
        v.sort();
        v
    };
    a.presence == b.presence && a.revealed == b.revealed && sorted(&a.hand) == sorted(&b.hand) && a.energy == b.energy
}

fn slot_code(id: &str) -> String {
    let digits: String = id.chars().rev().take_while(|c| c.is_ascii_digit()).collect::<Vec<_>>().into_iter().rev().collect();
    if id.contains("Energy") {
        format!("e{digits}")
    } else if id.contains("CardPlays") {
        format!("cp{digits}")
    } else {
        id.to_string()
    }
}

/// The next unrevealed slot on a track: every slot it requires is revealed.
fn next_slot(panel: &si_model::spirit::SpiritPanel, revealed: &BTreeSet<String>, track: Track) -> Option<String> {
    panel.tracks.iter().filter(|n| n.track == track && !revealed.contains(&n.id) && n.requires.iter().all(|r| revealed.contains(r))).map(|n| n.id.clone()).next()
}

fn placement_ok(p: &Placement, graph: &si_model::island::Island, presence: &BTreeMap<LandId, u32>, land: &LandId, st: &Counts) -> bool {
    let Some(l) = graph.lands.get(land) else { return false };
    if l.terrain == si_model::island::Terrain::Ocean && !matches!(p, Placement::Ocean) {
        return false;
    }
    let range = match p {
        Placement::Any { range } | Placement::Terrains { range, .. } | Placement::WithDahan { range } | Placement::WithInvaders { range } | Placement::WithTownCityOrBlight { range } | Placement::NoBlight { range } | Placement::Inland { range } | Placement::Coastal { range } | Placement::Other { range, .. } => *range as u32,
        Placement::Ocean => 99,
    };
    let in_range = presence.iter().any(|(from, n)| *n > 0 && graph.distance(from, land, false).is_some_and(|d| d <= range));
    if !in_range {
        return false;
    }
    match p {
        Placement::Terrains { terrains, .. } => terrains.contains(&l.terrain),
        Placement::WithDahan { .. } => st.dahan > 0,
        Placement::WithInvaders { .. } => st.invaders() > 0 || st.dahan > 0,
        Placement::WithTownCityOrBlight { .. } => st.buildings() > 0 || st.blight > 0,
        Placement::NoBlight { .. } => st.blight == 0,
        Placement::Inland { .. } => !l.coastal,
        Placement::Coastal { .. } => l.coastal,
        Placement::Ocean => l.terrain == si_model::island::Terrain::Ocean,
        Placement::Any { .. } | Placement::Other { .. } => true,
    }
}

/// Every growth the panel allows from the turn's starting position, each as
/// the position it produces: slot choices (top or bottom for each presence),
/// every legal placement, each card of the offered draft at a gain, the
/// Reclaim (each discard card when the growth reclaims one).
pub fn growth_positions(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, t: &TurnRecord, graph: &si_model::island::Island) -> Vec<Position> {
    let Some(panel) = defs.spirits.get(&s.spirit) else { return vec![] };
    let prev = rec.spirits.iter().find(|p| p.spirit == s.spirit && p.turn == t.turn - 1);
    let presence0: BTreeMap<LandId, u32> = match prev {
        Some(p) => p.presence_after.clone(),
        None => rec.setup.iter().filter(|(_, c)| c.presence > 0).map(|(l, c)| (l.clone(), c.presence)).collect(),
    };
    if presence0.is_empty() {
        return vec![];
    }
    let revealed0: BTreeSet<String> = prev.map(|p| p.revealed_after.clone()).unwrap_or_else(|| panel.starting_revealed());
    let hand0: Vec<String> = prev.map(|p| p.hand_after.clone()).unwrap_or_else(|| panel.uniques.clone());
    let discard0: Vec<String> = prev.map(|p| p.discard_after.clone()).unwrap_or_default();
    let energy0 = prev.map(|p| p.energy_after).unwrap_or(0);
    let lands: Vec<LandId> = graph.lands.keys().cloned().collect();
    let mut out = Vec::new();
    for g in &panel.growth.groups {
        // the group's yield with the always-group's on top (Rampant Green places a presence
        // every growth before its choice); each presence keeps its own placement rule
        let gy = g.yield_();
        let ay = panel.growth.always.as_ref().map(|a| a.yield_());
        let mut y = gy.clone();
        let mut rules: Vec<Placement> = Vec::new();
        if let Some(a) = &ay {
            y.energy += a.energy;
            y.cards += a.cards;
            y.plays_this_turn += a.plays_this_turn;
            y.presence += a.presence;
            if y.reclaim.is_none() {
                y.reclaim = a.reclaim.clone();
            }
            rules.extend(std::iter::repeat(a.placement.clone().unwrap_or(Placement::Any { range: 1 })).take(a.presence as usize));
        }
        rules.extend(std::iter::repeat(gy.placement.clone().unwrap_or(Placement::Any { range: 1 })).take(gy.presence as usize));
        let k = rules.len();
        // slot sequences for the presences placed: each from the Energy or the Card Plays track
        let mut slot_seqs: Vec<Vec<Track>> = vec![vec![]];
        for _ in 0..k {
            let mut next = Vec::new();
            for seq in &slot_seqs {
                for tr in [Track::Energy, Track::Plays] {
                    let mut s2 = seq.clone();
                    s2.push(tr);
                    s2.sort_by_key(|t| matches!(t, Track::Plays));
                    if !next.contains(&s2) {
                        next.push(s2);
                    }
                }
            }
            slot_seqs = next;
        }
        for seq in &slot_seqs {
            let mut revealed = revealed0.clone();
            let mut slot_ids: Vec<String> = Vec::new();
            let mut ok = true;
            for tr in seq {
                match next_slot(panel, &revealed, *tr) {
                    Some(id) => {
                        revealed.insert(id.clone());
                        slot_ids.push(id);
                    }
                    None => ok = false,
                }
            }
            if !ok {
                continue;
            }
            // placements: each presence over the lands its rule allows (the lands as they
            // stood at the Fast phase); two presences under the same rule are a combination,
            // not a permutation
            let legal: Vec<Vec<LandId>> = rules.iter().map(|p| lands.iter().filter(|l| placement_ok(p, graph, &presence0, l, &t.before_fast.get(*l).copied().unwrap_or_default())).cloned().collect()).collect();
            let mut placements: Vec<Vec<LandId>> = vec![vec![]];
            for (i, legal_i) in legal.iter().enumerate() {
                let same_rule = i > 0 && rules[i] == rules[i - 1];
                let mut next = Vec::new();
                for pl in &placements {
                    let from = if same_rule { pl.last().and_then(|last| legal_i.iter().position(|x| x == last)).unwrap_or(0) } else { 0 };
                    for l in legal_i.iter().skip(from) {
                        let mut p2 = pl.clone();
                        p2.push(l.clone());
                        next.push(p2);
                    }
                }
                if next.is_empty() {
                    // nothing legal: the presence stays on the track (the slot still counts)
                    next = placements.clone();
                }
                placements = next;
                placements.truncate(60);
            }
            // cards: the offered draft when the turn gained a card; else the next draft the
            // record shows for each deck (the deck order was fixed, so that is what a gain
            // here would have shown), a Major with each card the hand could forget for it
            let card_options: Vec<CardPick> = if y.cards > 0 {
                if !s.offered.is_empty() {
                    // the draft shown this turn; a Major costs a card: each card in hand, or one from the discard
                    let mut picks = Vec::new();
                    for c in &s.offered {
                        if defs.cards.get(c).is_some_and(|k| k.kind == si_model::power::PowerKind::Major) {
                            let mut forgets: Vec<String> = hand0.clone();
                            if let Some(d) = discard0.first() {
                                forgets.push(d.clone());
                            }
                            for f in forgets {
                                picks.push(CardPick { card: Some(c.clone()), draft_turn: None, forget: Some(f) });
                            }
                        } else {
                            picks.push(CardPick { card: Some(c.clone()), draft_turn: None, forget: None });
                        }
                    }
                    picks
                } else {
                    let mut picks = Vec::new();
                    for kind in [si_model::power::PowerKind::Minor, si_model::power::PowerKind::Major] {
                        let later = rec.spirits.iter().filter(|p| p.spirit == s.spirit && p.turn > t.turn && !p.offered.is_empty()).find(|p| p.offered.iter().filter_map(|c| defs.cards.get(c)).next().is_some_and(|c| c.kind == kind));
                        let Some(later) = later else { continue };
                        for c in &later.offered {
                            if kind == si_model::power::PowerKind::Major {
                                // a Major costs a card: each card in hand, or one from the discard (no cost this turn)
                                let mut forgets: Vec<String> = hand0.clone();
                                if let Some(d) = discard0.first() {
                                    forgets.push(d.clone());
                                }
                                for f in forgets {
                                    picks.push(CardPick { card: Some(c.clone()), draft_turn: Some(later.turn), forget: Some(f) });
                                }
                            } else {
                                picks.push(CardPick { card: Some(c.clone()), draft_turn: Some(later.turn), forget: None });
                            }
                        }
                    }
                    if picks.is_empty() {
                        picks.push(CardPick { card: None, draft_turn: None, forget: None });
                    }
                    picks
                }
            } else {
                vec![CardPick { card: None, draft_turn: None, forget: None }]
            };
            // the Reclaim: all of the discard, or one card of it (each tried)
            let reclaim_options: Vec<Vec<String>> = match y.reclaim {
                Some(Reclaim::All) | Some(Reclaim::Half) => vec![discard0.clone()],
                Some(Reclaim::One) if !discard0.is_empty() => discard0.iter().map(|c| vec![c.clone()]).collect(),
                _ => vec![vec![]],
            };
            for pl in &placements {
                for card in &card_options {
                    for rc in &reclaim_options {
                        let mut presence = presence0.clone();
                        for l in pl {
                            *presence.entry(l.clone()).or_insert(0) += 1;
                        }
                        let mut hand = hand0.clone();
                        let mut hindsight_cards: Vec<String> = Vec::new();
                        let mut growth = vec![format!("G{}", g.number)];
                        for (i, id) in slot_ids.iter().enumerate() {
                            growth.push(format!("{}{}", slot_code(id), pl.get(i).map(|l| format!("@{l}")).unwrap_or_default()));
                        }
                        if y.reclaim.is_some() {
                            hand.extend(rc.iter().cloned());
                            growth.push(match y.reclaim {
                                Some(Reclaim::One) => match rc.first() {
                                    Some(c) => format!("Rc1:{c}"),
                                    None => "Rc1".to_string(),
                                },
                                _ => "Rc".to_string(),
                            });
                        }
                        let mut cards_gained = 0;
                        if y.cards > 0 {
                            cards_gained = y.cards as u32;
                            match &card.card {
                                Some(c) => {
                                    hand.push(c.clone());
                                    growth.push(format!("+{c}"));
                                    if let Some(dt) = card.draft_turn {
                                        growth.push(format!("(draft T{dt})"));
                                        hindsight_cards.push(c.clone());
                                    }
                                    if let Some(f) = &card.forget {
                                        if let Some(i) = hand.iter().position(|h| h == f) {
                                            hand.remove(i);
                                        }
                                        growth.push(format!("-{f}"));
                                    }
                                }
                                None => growth.push("+card".to_string()),
                            }
                        }
                        if y.energy > 0 {
                            growth.push(format!("+{}E", y.energy));
                        }
                        let st = panel.track_state(&revealed);
                        out.push(Position { presence, revealed: revealed.clone(), hand, energy: energy0 + y.energy + st.income, plays: st.plays + y.plays_this_turn as i64, growth, slots: slot_ids.len() as u32, cards_gained, energy_gained: y.energy, actual: false, hindsight_cards });
                    }
                }
            }
        }
    }
    out
}

/// The position the record says the turn's plays started from.
pub fn actual_position(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, t: &TurnRecord) -> Position {
    let mut hand: Vec<String> = s.hand_after.iter().chain(s.cards_played.iter()).cloned().collect();
    hand.sort();
    hand.dedup();
    let prev_after = rec.spirits.iter().find(|p| p.spirit == s.spirit && p.turn == s.turn - 1).map(|p| p.energy_after).unwrap_or(0);
    let mut growth: Vec<String> = vec![if s.growth_groups.is_empty() { "G?".to_string() } else { s.growth_groups.iter().map(|g| format!("G{g}")).collect::<Vec<_>>().join("+") }];
    for (slot, land) in &s.presence_placed {
        if let Some(id) = slot {
            growth.push(format!("{}{}", slot_code(id), land.as_ref().map(|l| format!("@{l}")).unwrap_or_default()));
        }
    }
    if s.reclaimed {
        growth.push("Rc".to_string());
    }
    for c in &s.cards_gained {
        growth.push(format!("+{c}"));
    }
    for c in &s.cards_forgotten {
        growth.push(format!("-{c}"));
    }
    let growth_energy: i64 = defs.spirits.get(&s.spirit).map(|p| growth_energy_of(p, &s.growth_groups)).unwrap_or(0);
    if growth_energy > 0 {
        growth.push(format!("+{growth_energy}E"));
    }
    Position {
        presence: {
            // after the growth, before any Power placed more (those placements are the plays'):
            // the board at the Fast phase when it is one spirit's. The turn's end is not it, for a
            // spirit whose presence moves with the Dahan or dies with them (Thunderspeaker)
            let at_fast: BTreeMap<LandId, u32> = t.before_fast.iter().filter(|(_, c)| c.presence > 0).map(|(l, c)| (l.clone(), c.presence)).collect();
            if rec.meta.spirits.len() == 1 && !at_fast.is_empty() {
                at_fast
            } else {
                let mut p = s.presence_after.clone();
                for (_, _, land) in &s.presence_by_power {
                    if let Some(l) = land {
                        if let Some(n) = p.get_mut(l) {
                            *n = n.saturating_sub(1);
                            if *n == 0 {
                                p.remove(l);
                            }
                        }
                    }
                }
                p
            }
        },
        revealed: s.revealed_after.clone(),
        hand,
        energy: energy_available(rec, defs, s, prev_after),
        plays: s.plays.unwrap_or(s.cards_played.len() as i64),
        growth,
        slots: s.presence_placed.iter().filter(|(n, _)| n.is_some()).count() as u32,
        cards_gained: s.cards_gained.len() as u32,
        energy_gained: growth_energy,
        actual: true,
        hindsight_cards: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use si_model::oracle::{property, Rng, Violation};

    fn defs() -> Option<Definitions> {
        let d = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
        d.is_dir().then(|| Definitions::from_dir(&d).expect("the engine definitions load"))
    }

    /// Plays the notation can say, from every card and innate in the definitions: a land on
    /// board A or none, Fast as printed or marked `F:`, played for elements only, moves of
    /// any kind in and out, kills stated or not, a bracket exact or not.
    fn arbitrary_plays(rng: &mut Rng, defs: &Definitions) -> Vec<Play> {
        let cards: Vec<&PowerCard> = defs.cards.values().collect();
        let innates: Vec<&si_model::spirit::Innate> = defs.spirits.values().flat_map(|s| s.innates.iter()).collect();
        let land = |r: &mut Rng| LandId::new("A", r.between(1, 8) as u8);
        let mut out: Vec<Play> = Vec::new();
        for _ in 0..rng.between(1, 4) {
            let (power, printed_fast, innate) = if rng.coin() {
                let c = cards[rng.below(cards.len())];
                (c.id.clone(), c.speed == Speed::Fast, false)
            } else {
                let i = innates[rng.below(innates.len())];
                (i.id.clone(), i.speed == Speed::Fast, true)
            };
            if out.iter().any(|p| p.power == power) {
                continue;
            }
            let places = defs.cards.get(&power).is_some_and(|c| c.target.is_spirit() && adds_presence(c));
            let target = if rng.one_in(4) { None } else { Some(land(rng)) };
            let forced = !printed_fast && rng.one_in(4);
            let mut p = Play { power, target: target.clone(), fast: printed_fast || forced, innate, cost: 0, places, forced, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None };
            if !p.innate && rng.one_in(8) {
                p.inert = true;
                p.target = None;
            }
            if let (Some(tg), false) = (&target, p.inert || p.places) {
                for _ in 0..rng.below(3) {
                    let other = land(rng);
                    if other == *tg {
                        continue;
                    }
                    let piece = Some(si_rules::effects::oracle::arbitrary_filter(rng)).filter(|f| si_rules::effects::piece_code(f).is_some() && *f != si_model::power::PieceFilter::Piece(PieceKind::Dahan));
                    let (from, to) = if rng.coin() { (other, tg.clone()) } else { (tg.clone(), other) };
                    p.moves.push(Move { from, to, piece, n: rng.between(1, 3) });
                }
                if rng.one_in(3) {
                    let k: Vec<(LandId, si_model::power::PieceFilter)> = (0..rng.below(3)).map(|_| (tg.clone(), si_model::power::PieceFilter::Piece([PieceKind::Explorer, PieceKind::Town, PieceKind::City][rng.below(3)]))).collect();
                    p.kills = Some(k);
                }
                p.exact = !p.moves.is_empty() || p.kills.is_some() || rng.one_in(4);
            }
            out.push(p);
        }
        // a Repeat of a Power already in the line: its speed, no Energy, a land of its own
        if let Some(first) = rng.pick(&out).cloned().filter(|p| !p.inert && rng.coin()) {
            out.push(Play { target: Some(land(rng)), cost: 0, moves: Vec::new(), kills: None, exact: false, ..first });
        }
        out
    }

    #[test]
    fn every_line_writes_and_reads_back_the_same() {
        let Some(defs) = defs() else { return };
        property("notation", 1, 3000, |rng| arbitrary_plays(rng, &defs), |plays| notation_holds(&defs, plays));
    }

    #[test]
    fn the_residual_adds_and_takes_no_more_than_it_says() {
        property(
            "residual",
            1,
            6000,
            |rng| {
                let island = si_rules::invaders::oracle::arbitrary_island(rng);
                let mut d = Delta::default();
                for id in island.lands.keys() {
                    if rng.coin() {
                        let mut x = [0i64; 11];
                        for v in x.iter_mut() {
                            *v = rng.between(0, 4) as i64 - 2;
                        }
                        d.0.insert(id.clone(), x);
                    }
                }
                (island, d)
            },
            |(island, d)| {
                let mut after = island.clone();
                let lost = apply_delta(&mut after, d);
                let v: Vec<Violation> = delta_holds(island, d, &after, lost);
                v
            },
        );
    }
}
