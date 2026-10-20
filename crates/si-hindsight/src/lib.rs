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
//! effect can be placed in another land. Not yet: Event and Fear effects of the
//! next turn, deeper lookahead, Slow targets judged on the post-Invader state.

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
    pub source: String,
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
            source: "Blight 1 (the loss clock); Dahan and presence half each; Explorer/Town/City a quarter, half, three quarters; an Invader pushed out a quarter; Fear a quarter (four Fear is a card); a track slot revealed a half (Rei: scaling over cards), a card gained a quarter, an Energy kept a tenth; the next turn's Blight and Dahan at half weight".into(),
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
            for r in pr.reasons {
                v.source.push_str("; ");
                v.source.push_str(&r);
            }
        }
        v
    }
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
    /// The next turn's Invader steps run on the position this line leaves,
    /// with no further Spirit action: what the position carries forward.
    pub next_blight: u32,
    pub next_dahan_lost: u32,
    pub notes: Vec<String>,
}

impl Outcome {
    pub fn score(&self, v: &Values) -> f64 {
        self.blight as f64 * v.blight
            + self.dahan_lost as f64 * v.dahan
            + self.presence_destroyed as f64 * v.presence
            + self.explorers_destroyed as f64 * v.explorer
            + self.towns_destroyed as f64 * v.town
            + self.cities_destroyed as f64 * v.city
            + self.invaders_pushed as f64 * v.explorer
            + self.fear as f64 * v.fear
            + 0.5 * (self.next_blight as f64 * v.blight + self.next_dahan_lost as f64 * v.dahan)
    }

    pub fn summary(&self) -> String {
        let mut p = Vec::new();
        if self.blight > 0 {
            p.push(format!("Blight {}", self.blight));
        }
        if self.dahan_lost > 0 {
            p.push(format!("Dahan lost {}", self.dahan_lost));
        }
        if self.presence_destroyed > 0 {
            p.push(format!("presence lost {}", self.presence_destroyed));
        }
        let k: Vec<String> = [(self.explorers_destroyed, "E"), (self.towns_destroyed, "T"), (self.cities_destroyed, "C")].iter().filter(|(n, _)| *n > 0).map(|(n, k)| format!("{n}{k}")).collect();
        if !k.is_empty() {
            p.push(format!("destroyed {}", k.join(" ")));
        }
        if self.invaders_pushed > 0 {
            p.push(format!("pushed {}", self.invaders_pushed));
        }
        if self.fear > 0 {
            p.push(format!("Fear {}", self.fear));
        }
        if self.next_blight > 0 || self.next_dahan_lost > 0 {
            p.push(format!("next turn: Blight {}{}", self.next_blight, if self.next_dahan_lost > 0 { format!(", Dahan {}", self.next_dahan_lost) } else { String::new() }));
        }
        if p.is_empty() { "nothing changed".into() } else { p.join(", ") }
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    /// The growth in notation tokens (`G2`, `e4@C4`, `c2@C4`, `Rc`, `+<card id>`, `+1E`); empty when growth is not reviewed.
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
    /// Index of the growth position the line starts from (0 = as played).
    #[serde(default)]
    pub position: usize,
    /// The elements the line has this turn (track plus cards), as "Fire 3 Animal 4".
    #[serde(default)]
    pub elements: String,
    /// Each innate and the tier its elements reach: ("Epidemics Run Rampant", 2, 3) = tier 2 of 3; 0 = not reached.
    #[serde(default)]
    pub tiers: Vec<(String, u8, u8)>,
    pub plays: Vec<Play>,
    pub outcome: Outcome,
    pub score: f64,
    pub actual: bool,
}

impl Line {
    /// The plays in notation, cards first and innates in brackets: `SLSF>C5  FV  [ERR>C2 SR>C1]`.
    pub fn notation(&self, code: &dyn Fn(&str) -> String) -> String {
        if self.plays.is_empty() {
            return "(nothing)".into();
        }
        let word = |p: &Play| match &p.target {
            Some(l) => format!("{}>{l}", code(&p.power)),
            None => code(&p.power),
        };
        let cards: Vec<String> = self.plays.iter().filter(|p| !p.innate).map(word).collect();
        let innates: Vec<String> = self.plays.iter().filter(|p| p.innate).map(word).collect();
        let mut out = if cards.is_empty() { "no cards".to_string() } else { cards.join(" ") };
        if !innates.is_empty() {
            out.push_str(&format!(" | innate {}", innates.join(" ")));
        }
        out
    }

    /// The elements and innate tiers in words: `Fire 3 Animal 4 → Epidemics Run Rampant tier 2 of 3`.
    pub fn elements_words(&self, code: &dyn Fn(&str) -> String) -> String {
        let tiers: Vec<String> = self.tiers.iter().map(|(id, reached, of)| if *reached == 0 { format!("{} —", code(id)) } else { format!("{} {reached}/{of}", code(id)) }).collect();
        if tiers.is_empty() { self.elements.clone() } else { format!("{}  innates {}", self.elements, tiers.join(" ")) }
    }

    /// The growth in notation: `G2 e4@C4 c2@C4 +HSAI`.
    pub fn growth_notation(&self, code: &dyn Fn(&str) -> String) -> String {
        growth_words(&self.growth, code)
    }
}

/// `Sun + Moon + 2 Fire + Animal` → `Su1 Mo1 Fi2 An1`: two letters per element and the count.
pub fn element_short(set: &ElementSet) -> String {
    set.to_string()
        .split(" + ")
        .filter(|t| !t.trim().is_empty())
        .map(|t| {
            let mut parts = t.trim().split(' ');
            let first = parts.next().unwrap_or("");
            let (n, name) = match first.parse::<u8>() {
                Ok(n) => (n, parts.next().unwrap_or("")),
                Err(_) => (1, first),
            };
            let abbr = match name {
                "Animal" => "An",
                "Air" => "Ai",
                other => &other[..other.len().min(2)],
            };
            format!("{abbr}{n}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Fill a line's elements and innate tiers from its plays and the position's track.
fn annotate_elements(line: &mut Line, defs: &Definitions, s2: &SpiritTurn) {
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

fn growth_words(tokens: &[String], code: &dyn Fn(&str) -> String) -> String {
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
    /// Print a line per turn on stderr while working.
    pub progress: bool,
    /// Lines (best by hindsight, plus the actual) that get the informed score and are ranked by it.
    pub informed_lines: usize,
}

impl Default for ReviewOptions {
    fn default() -> Self {
        ReviewOptions { growth: true, max_positions: 100, max_lines_per_position: 3000, innate_beam: 40, max_innate_options: 60, keep_per_position: 300, progress: false, informed_lines: 30 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Review {
    pub values: Values,
    pub turns: Vec<TurnReview>,
    pub notes: Vec<String>,
}

fn innates_of<'d>(defs: &'d Definitions, spirit: &str) -> Vec<&'d si_model::spirit::Innate> {
    defs.spirits.get(spirit).map(|p| p.innates.iter().collect()).unwrap_or_default()
}

fn is_power_source(defs: &Definitions, source: &str) -> bool {
    defs.card_by_title(source).is_some() || defs.spirits.values().flat_map(|s| s.innates.iter()).any(|i| i.title == source)
}

/// The Invader steps of the turn, run on the island as it stands after the
/// candidate's Fast plays. Event and Fear effects the engine applied are
/// taken from the log as status effects (Spirit-sourced ones excluded, since
/// they depend on the candidate).
/// `explore`: the Explore card to run instead of the one that came (the informed
/// score averages over the cards still in the stage). `exogenous`: apply what the
/// Event and Fear cards did (status effects, Defend) as the log shows; off for the
/// informed score, since they were face down when the choice was made.
fn invader_phase(island: &mut Island, rec: &GameRecord, t: &TurnRecord, defs: &Definitions, base_rules: &Rules, out: &mut Outcome, explore: Option<&[String]>, exogenous: bool) {
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
    let explore_cards: Vec<String> = explore.map(|e| e.to_vec()).unwrap_or_else(|| t.explore.clone());
    let steps: Vec<(&Vec<String>, si_model::event::InvaderStep)> = vec![(&t.high_immigration, si_model::event::InvaderStep::Build), (&t.ravage, si_model::event::InvaderStep::Ravage), (&t.build, si_model::event::InvaderStep::Build), (&explore_cards, si_model::event::InvaderStep::Explore)];
    for (cards, step) in steps {
        for card_id in cards {
            let Some(card) = defs.invader_cards.get(card_id) else { continue };
            let outcomes = match step {
                si_model::event::InvaderStep::Ravage => invaders::ravage(island, card, &rules),
                si_model::event::InvaderStep::Build => invaders::build(island, card, &rules),
                si_model::event::InvaderStep::Explore => invaders::explore(island, card, &rules),
            };
            for o in outcomes {
                out.blight += o.blight + o.cascade;
                out.dahan_lost += o.dahan_lost;
                out.presence_destroyed += o.presence_destroyed;
                // counterattack kills: cheapest first in the simulator
                out.explorers_destroyed += o.invaders_destroyed;
                out.notes.extend(o.notes.into_iter().filter(|n| !n.starts_with("counterattack") && !n.starts_with("cascade")));
            }
        }
    }
}

fn apply_plays(island: &mut Island, defs: &Definitions, plays: &[Play], fast: bool, elements: &ElementSet, rules: &Rules, spirit: &str, t: &TurnRecord, armed: &mut Vec<effects::TriggerArmed>, out: &mut Outcome) {
    for p in plays.iter().filter(|p| p.fast == fast) {
        let Some(target) = &p.target else { continue };
        let Some((base, thresholds)) = verify::power_effects(defs, &p.power) else { continue };
        // where the player put this Power's spread pieces and which piece or branch it picked, when the log recorded it
        let ctx = Ctx { health_bonus: rules.invader_health_bonus(), elements: elements.clone(), blight_as_badlands: rules.blight_as_badlands_for(spirit), rules: Some(rules.clone()), placements: verify::placements_of(defs, t, &p.power), picks: verify::picks_of(defs, t, &p.power) };
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
        // skips the Power grants to its land (Strike Low), as the engine would record them
        if let Some(l) = island.lands.get_mut(target) {
            for e in base.iter().chain(thresholds.iter().filter(|(need, _)| elements.satisfies(need)).flat_map(|(_, es)| es.iter())) {
                verify::flag(l, e);
            }
        }
        let a = effects::apply(&base, &thresholds, island, target, &ctx);
        let destroyed_invader = a.destroyed.iter().any(|k| *k != PieceKind::Dahan);
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

/// Add what a Power did to the line's outcome: Fear, pushes, pieces destroyed
/// (with the Fear a Town or City gives), pieces removed, forced Ravages.
fn fold_applied(out: &mut Outcome, a: &effects::Applied) {
    out.fear += a.fear;
    out.invaders_pushed += a.pushed_pieces.iter().filter(|k| k.is_invader()).count() as u32;
    for r in &a.ravaged {
        out.blight += r.blight + r.cascade;
        out.dahan_lost += r.dahan_lost;
        out.presence_destroyed += r.presence_destroyed;
        out.explorers_destroyed += r.invaders_destroyed;
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
fn lookahead(island: &mut Island, rec: &GameRecord, next: Option<&TurnRecord>, defs: &Definitions, rules: &Rules, out: &mut Outcome, informed_build: Option<&[String]>) {
    let Some(n) = next else { return };
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
    out.next_blight = probe.blight;
    out.next_dahan_lost = probe.dahan_lost;
}

/// The Explore cards that could have come this turn, as far as was knowable: every
/// card of the stage the deck order put next that had not been drawn yet.
pub fn explore_candidates(rec: &GameRecord, defs: &Definitions, t: &TurnRecord) -> Vec<Vec<String>> {
    let Some(first) = t.explore.first() else { return vec![] };
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
fn energy_available(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, prev_after: i64) -> i64 {
    let _ = rec;
    let growth_energy: i64 = defs.spirits.get(&s.spirit).map(|p| s.growth_groups.iter().filter_map(|g| (*g as usize).checked_sub(1).and_then(|i| p.growth.groups.get(i))).map(|g| g.yield_().energy).sum()).unwrap_or(0);
    let from_track = prev_after + s.income.unwrap_or(0) + growth_energy;
    let from_log = s.energy_after + s.energy_paid;
    from_track.min(from_log).max(s.energy_after)
}

fn elements_for(defs: &Definitions, s: &SpiritTurn, played: &[&PowerCard]) -> ElementSet {
    let mut set = defs.spirits.get(&s.spirit).map(|p| p.track_state(&s.revealed_after).elements).unwrap_or_default();
    for c in played {
        set.extend(&c.elements);
    }
    set
}

/// Lands a card may target this turn: in range of the spirit's presence and
/// allowed by the card's target rule, as the land stood at the Fast phase.
fn targets_for(card: &PowerCard, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<LandId> {
    let mut out = Vec::new();
    for (id, land) in &graph.lands {
        if land.terrain == si_model::island::Terrain::Ocean {
            continue;
        }
        let st = before.get(id).copied().unwrap_or_default();
        if si_metrics::reach::range_ok(card, &s.presence_after, id, graph) != Some(true) {
            continue;
        }
        if !si_metrics::reach::target_ok(&card.target, &st, Some(land.terrain), Some(land.coastal)) {
            continue;
        }
        out.push(id.clone());
    }
    out
}

fn innate_targets(innate: &si_model::spirit::Innate, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<LandId> {
    let pseudo = PowerCard { id: innate.id.clone(), title: innate.title.clone(), kind: si_model::power::PowerKind::Unique, cost: 0, speed: innate.speed, range: innate.range.clone(), target: innate.target.clone(), elements: ElementSet::new(), body: vec![], thresholds: vec![], effects: vec![], expansion: String::new() };
    targets_for(&pseudo, s, before, graph)
}

/// Every set of up to `plays` cards from the hand within the Energy, each
/// Fast card with each legal target; Slow cards are played for their
/// elements only. Capped so a wide hand does not explode.
fn candidate_plays(hand: &[&PowerCard], plays: usize, energy: i64, s: &SpiritTurn, before: &BTreeMap<LandId, Counts>, graph: &si_model::island::Island) -> Vec<Vec<Play>> {
    let per_card: Vec<Vec<Play>> = hand
        .iter()
        .map(|c| {
            let fast = c.speed == Speed::Fast;
            if c.range.is_some() {
                // land-targeting card, Fast or Slow: every legal target (Slow targets are judged on the Fast-phase state; close enough for one turn)
                let mut v: Vec<Play> = targets_for(c, s, before, graph).into_iter().map(|t| Play { power: c.id.clone(), target: Some(t), fast, innate: false, cost: c.cost }).collect();
                if v.is_empty() {
                    v.push(Play { power: c.id.clone(), target: None, fast, innate: false, cost: c.cost });
                }
                v
            } else if c.target.is_spirit() {
                // own lands: scored where the spirit has presence
                let mut v: Vec<Play> = s.presence_after.keys().map(|l| Play { power: c.id.clone(), target: Some(l.clone()), fast, innate: false, cost: c.cost }).collect();
                if v.is_empty() {
                    v.push(Play { power: c.id.clone(), target: None, fast, innate: false, cost: c.cost });
                }
                v
            } else {
                vec![Play { power: c.id.clone(), target: None, fast, innate: false, cost: c.cost }]
            }
        })
        .collect();
    let mut out: Vec<Vec<Play>> = vec![vec![]];
    // choose subsets of cards (by index) of size <= plays, then one option per chosen card
    let n = hand.len();
    for mask in 0u32..(1u32 << n.min(12)) {
        let idx: Vec<usize> = (0..n.min(12)).filter(|i| mask & (1 << i) != 0).collect();
        if idx.is_empty() || idx.len() > plays {
            continue;
        }
        let cost: i64 = idx.iter().map(|i| hand[*i].cost).sum();
        if cost > energy {
            continue;
        }
        let mut combos: Vec<Vec<Play>> = vec![vec![]];
        for i in &idx {
            let mut next = Vec::new();
            for c in &combos {
                for opt in &per_card[*i] {
                    let mut c2 = c.clone();
                    c2.push(opt.clone());
                    next.push(c2);
                }
            }
            combos = next;
            if combos.len() > 3000 {
                combos.truncate(3000);
            }
        }
        out.extend(combos);
    }
    out
}

/// Review every turn of the game with the default options.
pub fn review(rec: &GameRecord, defs: &Definitions) -> Review {
    review_with(rec, defs, &ReviewOptions::default())
}

/// Review every turn of the game.
pub fn review_with(rec: &GameRecord, defs: &Definitions, opts: &ReviewOptions) -> Review {
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let values = Values::for_panel(defs.spirits.get(&spirit));
    let mut notes = Vec::new();
    if rec.meta.spirits.len() > 1 {
        notes.push("more than one spirit: only the first spirit's growth and plays are reviewed in this version".into());
    }
    let level = rec.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    let rules = Rules { adversary: rec.meta.adversary.clone(), level, spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), ..Default::default() };
    let graph = si_metrics::reach::island_for(rec, defs);
    let pick = defs.spirits.get(&spirit).map(|p| p.growth.pick).unwrap_or(1);
    if opts.growth && pick > 1 {
        notes.push(format!("{spirit} picks {pick} growth groups a turn: growth alternatives are not enumerated for it yet; the growth is taken as played"));
    }
    let enumerate_growth = opts.growth && pick <= 1;
    let all_innates = innates_of(defs, &spirit);
    let mut turns = Vec::new();
    for t in rec.turns.iter().filter(|t| t.turn >= 1 && !t.before_fast.is_empty()) {
        let Some(s) = rec.spirits.iter().find(|s| s.turn == t.turn && s.spirit == spirit) else { continue };
        let ended_before_slow = t.before_slow.is_empty();
        let next_turn = rec.turns.iter().find(|n| n.turn == t.turn + 1);
        // positions: the growth actually taken first, then every growth the panel allowed
        let actual_pos = actual_position(rec, defs, s, t);
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
        if opts.progress {
            eprint!("  T{} {} growth position(s) [", t.turn, positions.len());
        }
        // the actual line, as recorded (targets included)
        let actual_plays: Vec<Play> = s
            .cards_played
            .iter()
            .map(|c| {
                let card = defs.cards.get(c);
                let fast = card.is_some_and(|c| c.speed == Speed::Fast);
                let mut target = s.powers_used.iter().find(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && pw == c).and_then(|(_, _, t)| t.clone());
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
                Play { power: c.clone(), target, fast, innate: false, cost: card.map(|c| c.cost).unwrap_or(0) }
            })
            .collect();
        let actual_innates: Vec<Play> = s.powers_used.iter().filter(|(p, pw, _)| (*p == Phase::FastPower || *p == Phase::SlowPower) && all_innates.iter().any(|i| i.id == *pw)).map(|(p, pw, t)| Play { power: pw.clone(), target: t.clone(), fast: *p == Phase::FastPower, innate: true, cost: 0 }).collect();
        // Event and Fear choices whose effect the simulator can place in another land
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
        let ctx_exo = Ctx { health_bonus: rules.invader_health_bonus(), elements: ElementSet::new(), blight_as_badlands: false, rules: None, placements: vec![], picks: vec![] };
        // the rest of the Event and Fear effects: what the log shows at the first Invader step
        // beyond what the actual Fast plays and the choices above account for
        let actual_readable = actual_plays.iter().all(|p| defs.cards.get(&p.power).is_none_or(|c| c.fully_parsed()));
        let residual: Option<Delta> = if actual_readable && !t.before_invaders.is_empty() {
            let mut island = verify::island_for(rec, defs, &t.before_fast);
            let mut rules_t = rules.clone();
            let played: Vec<&PowerCard> = actual_plays.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
            let elements = elements_for(defs, s, &played);
            let mut probe = Outcome::default();
            let mut all: Vec<Play> = actual_plays.clone();
            all.extend(actual_innates.iter().cloned());
            let mut armed = Vec::new();
            apply_plays(&mut island, defs, &all, true, &elements, &rules, &spirit, t, &mut armed, &mut probe);
            for (_, land, e) in &scorable {
                apply_choice_effect(&mut island, &mut rules_t, land, e, &ctx_exo, &mut probe);
            }
            Some(delta_between(&island, &t.before_invaders))
        } else {
            None
        };
        // `informed`: score on what was knowable — the given Explore card (or the one that
        // came), no Event or Fear effects, cards only hindsight knows (`drop`) not played
        let simulate = |plays: &[Play], innate_plays: &[Play], s2: &SpiritTurn, moved: Option<(usize, &LandId)>, explore: Option<&[String]>, informed: bool, drop: &[String]| -> Outcome {
            let mut island = verify::island_for(rec, defs, &t.before_fast);
            let mut rules_t = rules.clone();
            let kept: Vec<Play> = plays.iter().filter(|p| !(informed && drop.contains(&p.power))).cloned().collect();
            let played: Vec<&PowerCard> = kept.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
            let elements = elements_for(defs, s2, &played);
            let mut out = Outcome::default();
            let mut all: Vec<Play> = kept;
            all.extend(innate_plays.iter().cloned());
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
                    apply_delta(&mut island, d);
                }
            }
            invader_phase(&mut island, rec, t, defs, &rules_t, &mut out, explore, !informed);
            if !ended_before_slow {
                apply_plays(&mut island, defs, &all, false, &elements, &rules_t, &spirit, t, &mut armed, &mut out);
                let assumed: Vec<String> = explore.map(|e| e.to_vec()).unwrap_or_else(|| t.explore.clone());
                lookahead(&mut island, rec, next_turn, defs, &rules_t, &mut out, if informed { Some(&assumed) } else { None });
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
                let targets = innate_targets(inn, s2, &t.before_fast, &graph);
                let mut next = Vec::new();
                for o in &options {
                    next.push(o.clone());
                    for tg in &targets {
                        let mut o2 = o.clone();
                        o2.push(Play { power: inn.id.clone(), target: Some(tg.clone()), fast: inn.speed == Speed::Fast, innate: true, cost: 0 });
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
            let mut s2 = s.clone();
            s2.presence_after = pos.presence.clone();
            s2.revealed_after = pos.revealed.clone();
            let mut ids = pos.hand.clone();
            ids.sort();
            ids.dedup();
            let hand: Vec<&PowerCard> = ids.iter().filter_map(|c| defs.cards.get(c)).collect();
            let mut candidates = candidate_plays(&hand, pos.plays.max(0) as usize, pos.energy, &s2, &t.before_fast, &graph);
            if candidates.is_empty() {
                candidates.push(vec![]);
            }
            candidates.truncate(opts.max_lines_per_position.max(1));
            let tried = candidates.len();
            let mut first: Vec<(f64, Vec<Play>, Outcome, f64)> = candidates
                .into_iter()
                .map(|cand| {
                    let out = simulate(&cand, &[], &s2, None, None, false, &[]);
                    let cost: i64 = cand.iter().map(|p| p.cost).sum();
                    let gv = growth_value(pos, cost);
                    (out.score(&values) + gv, cand, out, gv)
                })
                .collect();
            first.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            let mut lines: Vec<Line> = Vec::new();
            for (i, (score0, cand, out0, gv)) in first.into_iter().enumerate() {
                if i < opts.innate_beam {
                    let mut options = innate_options(&cand, &s2);
                    options.truncate(opts.max_innate_options.max(1));
                    for inn in options {
                        if inn.is_empty() {
                            lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score0, luck: 0.0, confidence: None, position: pi, elements: String::new(), tiers: vec![], plays: cand.clone(), outcome: out0.clone(), score: score0, actual: false });
                            continue;
                        }
                        let out = simulate(&cand, &inn, &s2, None, None, false, &[]);
                        let score = out.score(&values) + gv;
                        let mut plays = cand.clone();
                        plays.extend(inn);
                        lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score, luck: 0.0, confidence: None, position: pi, elements: String::new(), tiers: vec![], plays, outcome: out, score, actual: false });
                    }
                } else {
                    lines.push(Line { growth: pos.growth.clone(), growth_actual: pi == 0, growth_value: gv, ties: 0, informed: score0, luck: 0.0, confidence: None, position: pi, elements: String::new(), tiers: vec![], plays: cand, outcome: out0, score: score0, actual: false });
                }
            }
            lines.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            lines.truncate(opts.keep_per_position.max(1));
            for l in lines.iter_mut() {
                annotate_elements(l, defs, &s2);
            }
            if opts.progress {
                eprint!(".");
            }
            (tried, lines)
        };
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).clamp(1, 16).min(positions.len().max(1));
        let results: Vec<(usize, Vec<Line>)> = if threads <= 1 {
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
        let actual_out = simulate(&actual_plays, &actual_innates, s, None, None, false, &[]);
        let actual_gv = growth_value(&actual_pos, actual_cost);
        let mut actual_line = Line { growth: actual_pos.growth.clone(), growth_actual: true, growth_value: actual_gv, ties: 0, informed: 0.0, luck: 0.0, confidence: None, position: 0, elements: String::new(), tiers: vec![], plays: actual_plays.iter().cloned().chain(actual_innates.iter().cloned()).collect(), outcome: actual_out.clone(), score: actual_out.score(&values) + actual_gv, actual: true };
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
        if opts.progress {
            eprintln!("] {} play sets, {} lines kept, {:.1}s", count, lines.len(), started.elapsed().as_secs_f64());
        }
        // the informed score: the best lines by hindsight and the actual one, each averaged
        // over the Explore cards that could have come, with the Event and Fear effects left out
        let cands = explore_candidates(rec, defs, t);
        let mut top: Vec<Line> = Vec::new();
        for l in &lines {
            if l.actual || top.len() < opts.informed_lines.max(1) {
                top.push(l.clone());
            }
        }
        let score_under = |l: &Line, explore: Option<&[String]>| -> f64 {
            let pos = &positions[l.position.min(positions.len() - 1)];
            let mut s2 = s.clone();
            s2.presence_after = pos.presence.clone();
            s2.revealed_after = pos.revealed.clone();
            let (cards, inns): (Vec<Play>, Vec<Play>) = l.plays.iter().cloned().partition(|p| !p.innate);
            simulate(&cards, &inns, &s2, None, explore, true, &pos.hindsight_cards).score(&values) + l.growth_value
        };
        let per_card: Vec<Vec<f64>> = top.iter().map(|l| if cands.is_empty() { vec![score_under(l, None)] } else { cands.iter().map(|c| score_under(l, Some(c))).collect() }).collect();
        let actual_idx = top.iter().position(|l| l.actual);
        for (i, l) in top.iter_mut().enumerate() {
            let mean = per_card[i].iter().sum::<f64>() / per_card[i].len() as f64;
            l.informed = mean;
            l.luck = l.score - mean;
            l.confidence = match actual_idx {
                Some(a) if a != i && cands.len() > 1 => Some(per_card[i].iter().zip(&per_card[a]).filter(|(x, y)| x >= y).count() as f64 / cands.len() as f64),
                _ => None,
            };
        }
        top.sort_by(|a, b| b.informed.partial_cmp(&a.informed).unwrap().then(b.score.partial_cmp(&a.score).unwrap()).then(b.actual.cmp(&a.actual)));
        let lines = top;
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
                    h.push(format!("the Explore card ({}) was face down at the Fast phase, but it was the last card of its stage: knowable", t.explore.join(", ")));
                } else {
                    h.push(format!("the Explore card ({}) was face down at the Fast phase; it could have been {} ({} cards left in Stage {}): the informed score averages over them", t.explore.join(", "), others.join(", "), cands.len(), defs.invader_cards.get(&t.explore[0]).map(|c| c.stage).unwrap_or(0)));
                }
            }
            if !t.events.is_empty() {
                h.push(format!("the Event ({}) was face down", t.events.join(", ")));
            }
            if t.fear_cards_earned > 0 {
                h.push(format!("{} Fear card(s) resolved", t.fear_cards_earned));
            }
            match &residual {
                Some(d) if !d.0.is_empty() => h.push(format!("Event and Fear piece effects applied to every line as the log shows them ({} land(s) changed between the Fast phase and the first Invader step)", d.0.len())),
                Some(_) => {}
                None if !actual_readable => h.push("Event and Fear piece effects not applied: the actual line plays a card the simulator cannot read in full".into()),
                None => {}
            }
            if !s.offered.is_empty() {
                h.push(format!("the gain offered {}; each is tried as the card taken", s.offered.join(", ")));
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
            tnotes.push(format!("{} is not fully readable by the simulator; it sits in both lines at the same target, so the comparison stands with that caveat", unread_all.join(", ")));
        }
        {
            let mut typed_only: Vec<String> = Vec::new();
            for l in std::iter::once(&actual_line).chain(lines.first()) {
                for p in l.plays.iter().filter(|p| !p.innate) {
                    if let Some(c) = defs.cards.get(&p.power) {
                        let ns = c.not_simulated();
                        if !ns.is_empty() {
                            let line = format!("{}: {}", c.title, ns.join("; "));
                            if !typed_only.contains(&line) {
                                typed_only.push(line);
                            }
                        }
                    }
                }
            }
            if !typed_only.is_empty() {
                tnotes.push(format!("typed but not simulated, so the scores leave them out — {}", typed_only.join(" · ")));
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
        let reading = if !untargeted.is_empty() {
            format!("Not judged: the log shows {} played but never used (un-played after an undo, or a use the log dropped), so the actual line is incomplete", untargeted.join(", "))
        } else if !unread_all.is_empty() && !same_unread {
            format!("Not judged: the simulator cannot read {} in full yet (see si cards --card), so the scores on this turn are not trustworthy", unread_all.join(", "))
        } else if decision <= 0.5 {
            if actual_line.luck <= -1.0 {
                format!("Unlucky: the right play on what you knew (within {decision:.1} of the best informed line); the hidden cards cost {:.1}", -actual_line.luck)
            } else {
                "Sound: within half a Blight of the best line on what was knowable".to_string()
            }
        } else if actual_line.score >= best_hindsight - 0.5 {
            format!("Gamble paid: {decision:.1} behind the percentage play on what was known, but the hidden cards made it the best line")
        } else if decision <= 1.5 {
            format!("Better available on what was known: about {decision:.1} in ledger value, {}", where_from(gap_growth, gap_plays))
        } else {
            format!("Mistake on what was known: {decision:.1} in ledger value, {}", where_from(gap_growth, gap_plays))
        };
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
        tnotes.push("one-turn horizon plus the next turn's Invader steps at half weight; a card held for later scores only its Energy kept".into());
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
                    let out = simulate(&actual_plays, &actual_innates, s, Some((ci, &land)), None, false, &[]);
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
        let actual_luck = actual_line.luck;
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
        });
    }
    Review { values, turns, notes }
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
struct Delta(BTreeMap<LandId, [i64; 11]>);

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

fn apply_delta(island: &mut Island, d: &Delta) {
    for (l, diff) in &d.0 {
        if let Some(land) = island.lands.get_mut(l) {
            let c = &mut land.c;
            let fields: [&mut u32; 11] = [&mut c.explorers, &mut c.towns, &mut c.cities, &mut c.dahan, &mut c.blight, &mut c.presence, &mut c.beasts, &mut c.disease, &mut c.strife, &mut c.wilds, &mut c.badlands];
            for (f, x) in fields.into_iter().zip(diff.iter()) {
                *f = (*f as i64 + x).max(0) as u32;
            }
        }
    }
}

pub fn try_line(rec: &GameRecord, defs: &Definitions, turn: u32, text: &str) -> Result<Line, String> {
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let values = Values::for_panel(defs.spirits.get(&spirit));
    let t = rec.turns.iter().find(|t| t.turn == turn).ok_or("no such turn")?;
    let s = rec.spirits.iter().find(|s| s.turn == turn && s.spirit == spirit).ok_or("no spirit turn")?;
    if t.before_fast.is_empty() {
        return Err("no Fast-phase state recorded for that turn".into());
    }
    let level = rec.meta.adversary_level.or_else(|| si_metrics::adversary::estimate(rec, defs).and_then(|e| e.candidates.first().copied())).unwrap_or(0);
    let rules = Rules { adversary: rec.meta.adversary.clone(), level, spirits: rec.meta.spirits.clone(), hooks: Rules::hooks_of(defs, &rec.meta.spirits), ..Default::default() };
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
    let mut plays: Vec<Play> = Vec::new();
    for tok in text.split_whitespace() {
        let (name, target) = match tok.split_once('>') {
            Some((n, l)) => (n, Some(l.parse::<LandId>().map_err(|e| e.to_string())?)),
            None => (tok, None),
        };
        let id = resolve(name).ok_or(format!("unknown Power {name}"))?;
        let innate = !defs.cards.contains_key(&id);
        let (fast, cost) = match defs.cards.get(&id) {
            Some(c) => (c.speed == Speed::Fast, c.cost),
            None => (defs.spirits.values().flat_map(|p| p.innates.iter()).find(|i| i.id == id).is_some_and(|i| i.speed == Speed::Fast), 0),
        };
        plays.push(Play { power: id, target, fast, innate, cost });
    }
    let cards: Vec<Play> = plays.iter().filter(|p| !p.innate).cloned().collect();
    let innates: Vec<Play> = plays.iter().filter(|p| p.innate).cloned().collect();
    let prev_after = rec.spirits.iter().find(|p| p.spirit == spirit && p.turn == turn - 1).map(|p| p.energy_after).unwrap_or(0);
    let energy = energy_available(rec, defs, s, prev_after);
    let cost: i64 = cards.iter().map(|p| p.cost).sum();
    let plays_allowed = s.plays.unwrap_or(cards.len() as i64);
    let mut out_notes = Vec::new();
    if cost > energy {
        out_notes.push(format!("costs {cost} Energy, {energy} available"));
    }
    if cards.len() as i64 > plays_allowed {
        out_notes.push(format!("{} cards against {} plays", cards.len(), plays_allowed));
    }
    let hand: Vec<String> = s.hand_after.iter().chain(s.cards_played.iter()).cloned().collect();
    for c in &cards {
        if !hand.contains(&c.power) {
            out_notes.push(format!("{} was not in hand that turn", c.power));
        }
    }
    let mut island = verify::island_for(rec, defs, &t.before_fast);
    let played: Vec<&PowerCard> = cards.iter().filter_map(|p| defs.cards.get(&p.power)).collect();
    let elements = elements_for(defs, s, &played);
    let mut out = Outcome::default();
    let mut all = cards.clone();
    all.extend(innates.iter().cloned());
    let mut armed = Vec::new();
    apply_plays(&mut island, defs, &all, true, &elements, &rules, &spirit, t, &mut armed, &mut out);
    invader_phase(&mut island, rec, t, defs, &rules, &mut out, None, true);
    apply_plays(&mut island, defs, &all, false, &elements, &rules, &spirit, t, &mut armed, &mut out);
    let next_turn = rec.turns.iter().find(|n| n.turn == turn + 1);
    lookahead(&mut island, rec, next_turn, defs, &rules, &mut out, None);
    out.notes.extend(out_notes);
    let score = out.score(&values);
    Ok(Line { growth: vec![], growth_actual: true, growth_value: 0.0, ties: 0, informed: score, luck: 0.0, confidence: None, position: 0, elements: String::new(), tiers: vec![], plays: all, outcome: out, score, actual: false })
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
    /// The growth in notation tokens (`G2`, `e4@C4`, `c2@C4`, `Rc`, `+<card id>`, `+1E`).
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
        format!("c{digits}")
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
        let y = g.yield_();
        let k = y.presence as usize;
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
            // placements: combinations with repetition over the legal lands
            let legal: Vec<LandId> = match &y.placement {
                Some(p) if k > 0 => lands.iter().filter(|l| placement_ok(p, graph, &presence0, l, &t.before_fast.get(*l).copied().unwrap_or_default())).cloned().collect(),
                _ => vec![],
            };
            let mut placements: Vec<Vec<LandId>> = vec![vec![]];
            for _ in 0..k {
                let mut next = Vec::new();
                for pl in &placements {
                    let from = pl.last().and_then(|last| legal.iter().position(|x| x == last)).unwrap_or(0);
                    for l in legal.iter().skip(from) {
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
                    s.offered.iter().map(|c| CardPick { card: Some(c.clone()), draft_turn: None, forget: None }).collect()
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
                        out.push(Position { presence, revealed: revealed.clone(), hand, energy: energy0 + y.energy + st.income, plays: st.plays, growth, slots: slot_ids.len() as u32, cards_gained, energy_gained: y.energy, actual: false, hindsight_cards });
                    }
                }
            }
        }
    }
    out
}

/// The position the record says the turn's plays started from.
pub fn actual_position(rec: &GameRecord, defs: &Definitions, s: &SpiritTurn, t: &TurnRecord) -> Position {
    let _ = t;
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
    let growth_energy: i64 = defs.spirits.get(&s.spirit).map(|p| s.growth_groups.iter().filter_map(|g| (*g as usize).checked_sub(1).and_then(|i| p.growth.groups.get(i))).map(|g| g.yield_().energy).sum()).unwrap_or(0);
    if growth_energy > 0 {
        growth.push(format!("+{growth_energy}E"));
    }
    Position {
        presence: s.presence_after.clone(),
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
