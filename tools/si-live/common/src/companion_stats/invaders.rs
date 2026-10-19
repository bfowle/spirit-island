//! Invader forecast: the coming Invader Phase, resolved in its real order
//! (Ravage → Build → Explore), the next turn's pipeline, and card counting.
//!
//! Each invader card passes Explore → Build → Ravage on three successive
//! turns. Inside one Invader Phase the order is reversed: Ravage first, then
//! Build, then Explore. So a land's Explore odds are computed on the board
//! *after* this phase's Builds (a new Town is a new Explore source), and the
//! card in the Build slot names next turn's Ravage lands.

use crate::public::{PublicInvaderCard, PublicLand, PublicSnapshot, PublicStatusEffect};
use super::adversary;
use crate::rules::{self, RavageLedger};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvaderPhaseForecast {
    /// The turn whose Invader Phase this describes.
    pub turn: u32,
    /// Where the game is relative to it: "before", "ravage", "build", "explore".
    pub position: String,
    pub ravage_cards: Vec<PublicInvaderCard>,
    pub build_cards: Vec<PublicInvaderCard>,
    pub ravages: Vec<RavageForecast>,
    pub builds: Vec<BuildForecast>,
    pub explore: ExploreForecast,
    pub notes: Vec<String>,
    /// England L3+: the card on the High Immigration tile and the Builds it
    /// makes before the Ravage.
    #[serde(default)]
    pub high_immigration_cards: Vec<PublicInvaderCard>,
    #[serde(default)]
    pub high_immigration: Vec<BuildForecast>,
    /// A Stage II Escalation that resolves as Builds (England), if the
    /// Explore card is flagged.
    #[serde(default)]
    pub escalation: Option<EscalationForecast>,
    /// Adversary actions between the printed steps (after the Ravage, after
    /// the Build), applied when the rule decides, listed when players pick.
    #[serde(default)]
    pub rule_actions: Vec<super::adversary::RuleAction>,
    /// Habsburg Mining L1, Ceaseless Mining: the Build card Ravages in Mining lands.
    #[serde(default)]
    pub build_ravages: Vec<RavageForecast>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EscalationForecast {
    pub title: String,
    pub text: String,
    /// Flagged cards among the possible Explore cards (1 of 1 once revealed).
    pub chance: (u32, u32),
    /// Where it Builds. Tied lands on one board: the player picks one.
    pub builds: Vec<BuildForecast>,
    /// Every listed land is affected (Sweden), not one per board (England).
    #[serde(default)]
    pub each_land: bool,
    /// "before the Explore", "after the Explore", "after the cards advance".
    #[serde(default)]
    pub when: String,
    /// What the players choose, if anything ("you pick 1 per board").
    #[serde(default)]
    pub pick: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RavageForecast {
    pub land: String,
    pub ledger: RavageLedger,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BuildForecast {
    pub land: String,
    pub adds: String,
    pub done: bool,
    pub notes: Vec<String>,
    /// Only if this Explore card is drawn (an Escalation that depends on
    /// where the card explores).
    #[serde(default)]
    pub card: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExploreForecast {
    /// Set when the Explore card is already face up.
    pub revealed: Option<PublicInvaderCard>,
    /// Stage on the back of the next card; `None` if the deck is empty.
    pub next_stage: Option<u8>,
    /// Empty deck: the spirits lose at this Explore.
    pub deck_empty: bool,
    pub candidates: Vec<ExploreCandidate>,
    /// Per land: (cards that explore it, candidate cards).
    pub per_land: BTreeMap<String, (u32, u32)>,
    pub pocket_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExploreCandidate {
    pub card: PublicInvaderCard,
    pub lands: Vec<String>,
    /// Lands that match the card but have no Explore source in range.
    pub blocked: Vec<String>,
    /// Adversary and scenario changes to this card's Explore.
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StageCount {
    pub stage: u8,
    pub seen: Vec<String>,
    pub unseen: Vec<PublicInvaderCard>,
    pub left_in_deck: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandOutlook {
    pub id: String,
    pub terrain: String,
    pub coastal: bool,
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    pub dahan: u32,
    pub blight: u32,
    pub presence: u32,
    /// "coastal" (always explorable), "fed" (an inland source in range),
    /// "dead" (no source in range after this phase's Builds), "ocean".
    pub explore_access: String,
    /// Explore sources in range after this phase's Builds (the land itself included).
    pub fed_by: Vec<String>,
    /// Inland lands whose only Explore source is this land.
    pub sole_source_for: Vec<String>,
    pub ravage_now: Option<RavageLedger>,
    pub build_now: Option<String>,
    /// (cards that explore it, candidates).
    pub explore_chance: (u32, u32),
    pub ravage_next: Ravage2,
    /// (cards that would build here next turn, candidates).
    pub build_next_chance: (u32, u32),
    pub badges: Vec<String>,
    /// What an action here changes if it resolves in the Fast phase / Slow phase.
    pub fast_window: Vec<String>,
    pub slow_window: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum Ravage2 {
    #[default]
    No,
    /// Matches next turn's Ravage card and already has Invaders.
    Certain,
    /// Matches it and has Invaders only if explored this phase.
    IfExplored,
}

/// Steps after which this turn's Invader Phase is complete.
fn phase_position(step: &str) -> &'static str {
    match step {
        "Ravage" => "ravage",
        "Build" => "build",
        "Explore" => "explore",
        "AdvanceInvaderCards" | "SlowPower" | "TimePasses" => "after",
        _ => "before",
    }
}

pub struct InvaderView {
    pub phase: InvaderPhaseForecast,
    pub lands: Vec<LandOutlook>,
    pub stages: Vec<StageCount>,
    /// The forecast board after this phase's Builds (for loss clocks).
    pub after_build: Vec<PublicLand>,
}

/// Next turn's Invader Phase, played forward from this turn's forecast
/// board: the Build card Ravages, this turn's Explore card Builds. A lower
/// bound: this turn's Explorers count only once the Explore card is face up,
/// and no Spirit action is assumed. `None` once this turn's Invader Phase is
/// over (then `forecast` already describes next turn). Brett, 2026-10-19
/// (Prussia + Blitz game): Turn 4's four-land Ravage wasn't visible on Turn 3.
pub fn next_turn(s: &PublicSnapshot, now: &InvaderView) -> Option<InvaderView> {
    if phase_position(&s.step) == "after" {
        return None;
    }
    let mut n = s.clone();
    n.turn = s.turn + 1;
    n.step = "SpiritPhase".into();
    n.lands = now.after_build.clone();
    // This turn's Ravage Blight stays (its cascades' targets are the players'
    // pick, so they aren't placed).
    for r in now.phase.ravages.iter().filter(|r| !r.done && r.ledger.adds_blight) {
        if let Some(l) = n.lands.iter_mut().find(|l| l.id == r.land) {
            l.blight += 1 + r.ledger.extra_blight as u32;
        }
    }
    if let Some(c) = now.phase.explore.candidates.iter().find(|_| now.phase.explore.revealed.is_some()) {
        for l in n.lands.iter_mut().filter(|l| c.lands.contains(&l.id)) {
            rules::add_built(l, "Explorer", 1);
        }
    }
    for l in &mut n.lands {
        l.defend = 0;
        l.damage_taken = 0;
        for p in &mut l.pieces {
            p.damage = 0;
        }
    }
    n.status_effects.retain(|e| !matches!(e.source_kind.as_str(), "fear" | "event" | "power"));
    let inv = &mut n.invaders;
    for (slot, cards) in &mut inv.extra_slots {
        if slot.starts_with("HighImmigration") {
            *cards = s.invaders.ravage.clone();
        }
    }
    inv.ravage = s.invaders.build.clone();
    inv.build = s.invaders.explore.clone();
    inv.explore.clear();
    if s.invaders.explore.is_empty() && !inv.upcoming_stages.is_empty() {
        inv.upcoming_stages.remove(0);
    }
    Some(forecast_at(&n))
}

pub fn forecast(s: &PublicSnapshot) -> InvaderView {
    // After this turn's Invader Phase the forecast is for next turn, when
    // Time Passes will have cleared this turn's Defend, Damage and single-turn
    // effects (Fear, Event and Power effects). Playtest R7: Overseas Trade's
    // Defend 6 made next turn's Coastal Ravage look harmless.
    if phase_position(&s.step) == "after" {
        let mut next = s.clone();
        for l in &mut next.lands {
            l.defend = 0;
            l.damage_taken = 0;
            for p in &mut l.pieces {
                p.damage = 0;
            }
        }
        next.status_effects.retain(|e| !matches!(e.source_kind.as_str(), "fear" | "event" | "power"));
        return forecast_at(&next);
    }
    forecast_at(s)
}

/// A Spread of Rampant Green, Choke the Land with Green: "Whenever Invaders
/// would Ravage or Build in a land with your Sacred Site, you may skip it by
/// destroying one of your Presence in that land" (playtest 2026-10-19 Russia
/// game: six Builds/Ravages skipped this way).
fn choke(l: &PublicLand) -> Option<String> {
    (l.presence.get("ASpreadOfRampantGreen").copied().unwrap_or(0) >= 2)
        .then(|| "Choke the Land with Green (destroy 1 Presence there)".to_string())
}

/// What a Build card's resolution needs besides the board.
struct BuildRules<'a> {
    adv: &'a adversary::Adv<'a>,
    effects: &'a [PublicStatusEffect],
    nbrs: &'a BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// Health of a new Town / City.
    town_health: i64,
    city_health: i64,
    /// Boards where Vengeance as a Burning Plague has Presence: there a
    /// Disease stops a Build only if the player spends it.
    vengeance_boards: BTreeSet<String>,
}

const VENGEANCE: &str = "VengeanceAsABurningPlague";

/// Boards where Vengeance as a Burning Plague has Presence.
fn vengeance_boards(lands: &[PublicLand]) -> BTreeSet<String> {
    lands.iter().filter(|l| l.presence.get(VENGEANCE).copied().unwrap_or(0) > 0).map(|l| l.board.clone()).collect()
}

/// One Build card on `board`. The lands it builds in are fixed on the board
/// before it resolves (FAQ: "you check what lands will Build at the beginning
/// of looking at the Build Card"); each Build then adds its piece, which the
/// next card sees. `normal`: the card is in the printed Build space. `skip`:
/// lands where the card does something else (Ceaseless Mining).
#[allow(clippy::too_many_arguments)]
fn build_card(
    card: &PublicInvaderCard,
    board: &mut [PublicLand],
    r: &BuildRules,
    cancels: &mut rules::BuildCancels,
    normal: bool,
    done: bool,
    skip: &BTreeSet<String>,
    notes: &mut Vec<String>,
) -> Vec<BuildForecast> {
    let mut out = Vec::new();
    if !done && let Some(src) = cancels.card(normal) {
        notes.push(format!("{}: whole Build skipped ({src})", card.id));
        return out;
    }
    let before = board.to_vec();
    let mut lands: Vec<(String, Option<String>)> = before
        .iter()
        .filter(|l| !skip.contains(&l.id))
        .filter_map(|l| {
            if rules::builds_in(card, l) {
                Some((l.id.clone(), None))
            } else {
                adversary::builds_without_invaders(r.adv, card, l, &before, r.nbrs).map(|why| (l.id.clone(), Some(why)))
            }
        })
        .collect();
    if !done && let Some(src) = cancels.lowest(normal) {
        // The lowest-numbered land matching the card on each board, whether
        // or not it would have built.
        let mut boards: Vec<&str> = before.iter().map(|l| l.board.as_str()).collect();
        boards.dedup();
        for b in boards {
            if let Some(low) = before.iter().filter(|l| l.board == b && rules::card_matches(card, l)).min_by_key(|l| l.number) {
                if lands.iter().any(|(id, _)| *id == low.id) {
                    notes.push(format!("{}: no Build ({src}: the lowest-numbered matching land)", low.id));
                    lands.retain(|(id, _)| *id != low.id);
                } else {
                    notes.push(format!("{src} skips {}, which would not have built anyway", low.id));
                }
            }
        }
    }
    for (id, why) in lands {
        let Some(i) = board.iter().position(|l| l.id == id) else { continue };
        let (mut piece, mut bnotes) = rules::build_outcome_with(&board[i], r.effects, cancels);
        // Vengeance as a Burning Plague, The Terror of a Slowly Unfolding
        // Plague: "When Disease would prevent a Build on a board with your
        // Presence, you may let the Build happen (Removing no Disease). If you
        // do, 1 Fear." The engine asks (optional RemoveDiseaseToPreventBuild;
        // Prussia + Blitz game: Sands C2 and Mountain C7 built, Sands C6 didn't).
        if piece.is_none() && bnotes.iter().any(|n| n.starts_with("Disease stops")) && r.vengeance_boards.contains(&board[i].board) {
            piece = Some(rules::build_piece(&board[i]));
            bnotes.retain(|n| !n.starts_with("Disease stops"));
            bnotes.push("Can skip: spend the Disease to stop it; or let it Build (Disease stays, +1 Fear: The Terror of a Slowly Unfolding Plague)".into());
        }
        bnotes.extend(why);
        if let Some(c) = choke(&board[i]) {
            bnotes.push(format!("Can skip: {c}"));
        }
        match piece {
            Some(p) => {
                let mut adds = p;
                if !done {
                    rules::add_built(&mut board[i], p, if p == "City" { r.city_health } else { r.town_health });
                    let (more, built) = adversary::after_build(r.adv, board, i, p, r.nbrs, r.town_health);
                    bnotes.extend(more);
                    adds = built.unwrap_or(p);
                }
                out.push(BuildForecast { land: id, adds: adds.to_string(), done, notes: bnotes, card: None });
            }
            None => {
                // The Disease that stopped it is spent (Turn 4 of the England
                // game: Building Boom then built in Jungle D2).
                if !done
                    && bnotes.iter().any(|n| n.starts_with("Disease stops"))
                    && let Some(d) = board[i].tokens.get_mut("Disease")
                {
                    *d -= 1;
                }
                notes.push(format!("{id}: no Build ({})", bnotes.join("; ")));
            }
        }
    }
    out
}

fn forecast_at(s: &PublicSnapshot) -> InvaderView {
    let nbrs = rules::neighbors(s);
    let adv = adversary::Adv::new(s);
    let position = phase_position(&s.step);
    // After this turn's Invader Phase the slots already hold next turn's cards.
    let (turn, position) = if position == "after" { (s.turn + 1, "before") } else { (s.turn, position) };
    let high_immigration_done = position != "before";
    let ravage_done = matches!(position, "build" | "explore");
    let build_done = position == "explore";
    let inv = &s.invaders;
    let mut notes = Vec::new();
    for (slot, cards) in &inv.extra_slots {
        if !cards.is_empty() && !slot.starts_with("HighImmigration") {
            notes.push(format!("{slot} holds {}: an extra invader action this turn (see rules register)", titles(cards)));
        }
    }

    // Adversary rules that change Ravage damage, as effects the ledger applies.
    let mut effects = s.status_effects.clone();
    effects.extend(adversary::ravage_effects(&adv));
    let isolated = rules::isolated_lands(&effects);
    let range = rules::explore_range(&effects);

    // Card counting first: the Explore candidates feed rules that act before
    // the Explore (which lands a new Town would open to Exploring).
    let seen: BTreeSet<&str> = inv
        .explore
        .iter()
        .chain(&inv.build)
        .chain(&inv.ravage)
        .chain(&inv.discard)
        .chain(inv.extra_slots.iter().flat_map(|(_, v)| v))
        .map(|c| c.id.as_str())
        .collect();
    let stages: Vec<StageCount> = (1..=3u8)
        .map(|st| StageCount {
            stage: st,
            seen: inv.pool.iter().filter(|c| c.stage == st && seen.contains(c.id.as_str())).map(|c| c.id.clone()).collect(),
            unseen: inv.pool.iter().filter(|c| c.stage == st && !seen.contains(c.id.as_str())).cloned().collect(),
            left_in_deck: inv.upcoming_stages.iter().filter(|x| **x == st).count() as u32,
        })
        .collect();
    let revealed = inv.explore.first().cloned();
    let next_stage = inv.upcoming_stages.first().copied();
    let candidates_cards: Vec<PublicInvaderCard> = match (&revealed, next_stage) {
        (Some(c), _) => vec![c.clone()],
        (None, Some(st)) => stages[(st - 1) as usize].unseen.clone(),
        (None, None) => Vec::new(),
    };

    // ---- High Immigration (England L3+): the card that left the Ravage slot
    // Builds before the Ravage (rules register R-ADV-ENG-L3). It is not in the
    // printed Build space, so its Builds are not normal (FAQ); the Towns and
    // Cities it adds count for the Ravage and the regular Build (FAQ).
    let england = adv.level(adversary::ENGLAND);
    let health = |kind: &str| {
        s.lands.iter().flat_map(|l| &l.pieces).find(|p| p.kind == kind).map(|p| p.max_health).unwrap_or_else(|| rules::invader_damage(kind) + i64::from(england >= 5))
    };
    let (town_health, city_health) = (health("Town"), health("City"));
    let build_rules = BuildRules { adv: &adv, effects: &effects, nbrs: &nbrs, town_health, city_health, vengeance_boards: vengeance_boards(&s.lands) };
    let mut cancels = rules::BuildCancels::new(&effects);
    let mut board: Vec<PublicLand> = s.lands.clone();
    let high_immigration_cards: Vec<PublicInvaderCard> =
        inv.extra_slots.iter().filter(|(slot, _)| slot.starts_with("HighImmigration")).flat_map(|(_, v)| v.iter().cloned()).collect();
    let mut high_immigration = Vec::new();
    let none = BTreeSet::new();
    for card in &high_immigration_cards {
        high_immigration.extend(build_card(card, &mut board, &build_rules, &mut cancels, false, high_immigration_done, &none, &mut notes));
    }
    if england >= 6 && !high_immigration_cards.is_empty() {
        notes.push("England L6 (Independent Resolve): if no Fear Card resolves this Invader Phase, High Immigration Builds twice".into());
    }

    // One Ravage action in a land, with every rule that changes it.
    let ravage_of = |l: &PublicLand, board: &[PublicLand], done: bool| -> RavageForecast {
        let mut fx = rules::for_land(&effects, l, board, &nbrs);
        fx.extend(adversary::land_effects(&adv, l, board, &nbrs));
        let mut ledger = rules::ravage_ledger(l, &fx);
        if ledger.skip_available.is_none() {
            ledger.skip_available = choke(l);
        }
        adversary::after_ledger(&adv, l, &mut ledger, &effects);
        RavageForecast { land: l.id.clone(), ledger, done }
    };

    // ---- Ravage (first of the printed spaces)
    let ravages: Vec<RavageForecast> =
        board.iter().filter(|l| inv.ravage.iter().any(|c| adversary::ravages_in(&adv, c, l))).map(|l| ravage_of(l, &board, ravage_done)).collect();

    // A counterattack that destroys every Invader whatever the assignment
    // clears the land before Build and Explore: it stops this turn's Build
    // there and can remove an Explore source (Test 1 Turn 3: Jungle E3's
    // Dahan destroyed the Town feeding Mountain E5 and Wetland E4).
    let mut after_build: Vec<PublicLand> = board.clone();
    let mut rule_actions = Vec::new();
    if !ravage_done {
        for r in ravages.iter().filter(|r| r.ledger.counterattack_clears && r.ledger.skip_available.is_none()) {
            if let Some(l) = after_build.iter_mut().find(|l| l.id == r.land) {
                l.explorers = 0;
                l.towns = 0;
                l.cities = 0;
                l.pieces.retain(|p| !rules::is_invader(&p.kind));
                notes.push(format!("{}: the Dahan counterattack destroys every Invader before Build and Explore", l.id));
            }
        }
        rule_actions.extend(adversary::after_ravage_step(&adv, &mut after_build, &ravages, &inv.ravage, &nbrs, turn, town_health));
        // Vengeance, Lingering Pestilence: "When your Presence is Destroyed by
        // anything except a Spirit Action, add 1 Disease where each Destroyed
        // Presence was." Ravage Blight destroys 1 Presence of each Spirit in
        // the land, so the Disease is there for the Build (Prussia + Blitz
        // game, Turn 2: Mountain C7).
        let mut lands = Vec::new();
        for r in ravages.iter().filter(|r| r.ledger.adds_blight && r.ledger.presence_at_risk.contains_key(VENGEANCE)) {
            if let Some(l) = after_build.iter_mut().find(|l| l.id == r.land) {
                *l.tokens.entry("Disease".into()).or_default() += 1;
                lands.push(BuildForecast { land: l.id.clone(), adds: "Disease".into(), done: false, notes: vec!["your destroyed Presence".into()], card: None });
            }
        }
        if !lands.is_empty() {
            rule_actions.push(adversary::RuleAction {
                rule: "Lingering Pestilence (Vengeance as a Burning Plague)".into(),
                when: "after the Ravage".into(),
                text: "Blight destroys your Presence here: +1 Disease, before the Build".into(),
                lands,
                pick: String::new(),
            });
        }
    }

    // ---- Build (second), then the board Explore sees. Each Build card
    // resolves in turn and sees the earlier card's results (FAQ, multiple
    // Build cards); one-use cancels apply once across them.
    let mut builds = Vec::new();
    let mut build_ravages = Vec::new();
    for (i, card) in inv.build.iter().enumerate() {
        // The card that advanced normally is listed last (an Entrench in the
        // Face of Fear card is inserted before it).
        let normal = i + 1 == inv.build.len();
        // Ceaseless Mining: in Mining lands the ordinary Build card Ravages.
        let mining: BTreeSet<String> = if normal {
            after_build.iter().filter(|l| rules::card_matches(card, l) && adversary::ceaseless_mining(&adv, l)).map(|l| l.id.clone()).collect()
        } else {
            BTreeSet::new()
        };
        for l in after_build.iter().filter(|l| mining.contains(&l.id)) {
            let mut r = ravage_of(l, &after_build, build_done);
            r.ledger.applied.insert(0, "Ceaseless Mining (Habsburg Mining L1): the Build card Ravages this Mining land".into());
            build_ravages.push(r);
        }
        builds.extend(build_card(card, &mut after_build, &build_rules, &mut cancels, normal, build_done, &mining, &mut notes));
    }
    if !build_done {
        let cards: Vec<&PublicInvaderCard> = candidates_cards.iter().collect();
        rule_actions.extend(adversary::after_build_step(&adv, &mut after_build, &inv.build, &cards, &nbrs, range, town_health));
    }

    // ---- Explore (third)
    if range > 1 {
        notes.push(format!("Explore range is {range} this turn: lands up to {range} away from a source can be explored"));
    }
    let fed = |land: &PublicLand| -> Vec<&str> {
        rules::explore_sources_within(land, &after_build, &nbrs, range).into_iter().filter(|src| !isolated.contains(*src)).collect()
    };
    let candidates: Vec<ExploreCandidate> = candidates_cards
        .iter()
        .map(|c| {
            let matching: Vec<&PublicLand> = after_build.iter().filter(|l| rules::card_matches(c, l)).collect();
            let open = |l: &&&PublicLand| {
                !isolated.contains(&l.id) && !fed(l).is_empty() && !rules::wilds_stop_explore(l, &effects)
            };
            let lands: Vec<String> = matching.iter().filter(|l| open(l)).map(|l| l.id.clone()).collect();
            ExploreCandidate {
                card: c.clone(),
                notes: adversary::explore_notes(&adv, c, &lands, &after_build),
                lands,
                blocked: matching
                    .iter()
                    .filter(|l| !open(l))
                    .map(|l| {
                        if isolated.contains(&l.id) {
                            format!("{} (Isolated)", l.id)
                        } else if fed(l).is_empty() {
                            l.id.clone()
                        } else {
                            format!("{} (Wilds)", l.id)
                        }
                    })
                    .collect(),
            }
        })
        .collect();
    let den = candidates.len() as u32;
    let mut per_land: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for l in s.lands.iter().filter(|l| !l.is_ocean) {
        let num = candidates.iter().filter(|c| c.lands.contains(&l.id)).count() as u32;
        per_land.insert(l.id.clone(), (num, den));
    }
    let pocket_rule = match (revealed.is_some(), next_stage) {
        (true, _) => "The Explore card is face up: these lands explore.".into(),
        (false, Some(st)) => format!(
            "Next card is Stage {st}: each of the {den} unseen Stage {st} cards is equally likely. Chance a land explores = cards that hit it ÷ {den}."
        ),
        (false, None) => "The Invader deck is empty: the spirits lose at the next Explore.".into(),
    };
    let explore = ExploreForecast { revealed, next_stage, deck_empty: next_stage.is_none() && inv.explore.is_empty(), candidates, per_land, pocket_rule };
    // Escalation: on a flagged card, the adversary's Escalation resolves.
    let escalation =
        adversary::escalation(&adv, &after_build, &explore.candidates, explore.revealed.is_some(), &inv.build, &effects, &cancels, &ravages, &nbrs, range, &mut notes);

    // ---- next turn: the Build card becomes the Ravage card, this Explore card the Build card.
    let explored_now = |id: &str| explore.per_land.get(id).copied().unwrap_or((0, 0));
    let mut lands = Vec::new();
    for (l, lb) in s.lands.iter().zip(&after_build) {
        let ravage_now = ravages.iter().find(|r| r.land == l.id).map(|r| r.ledger.clone());
        let build_now = builds.iter().find(|b| b.land == l.id).map(|b| b.adds.clone());
        let fed_by: Vec<String> = fed(lb).into_iter().map(String::from).collect();
        let explore_access = if l.is_ocean {
            "ocean"
        } else if fed_by.iter().any(|f| s.lands.iter().any(|x| x.id == *f && x.is_ocean)) || l.coastal {
            "coastal"
        } else if fed_by.is_empty() {
            "dead"
        } else {
            "fed"
        };
        let (en, ed) = explored_now(&l.id);
        let matches_build = inv.build.iter().any(|c| rules::card_matches(c, l));
        let ravage_next = if !matches_build || l.is_ocean {
            Ravage2::No
        } else if rules::has_invaders(lb) {
            Ravage2::Certain
        } else if en > 0 {
            Ravage2::IfExplored
        } else {
            Ravage2::No
        };
        let will_have_invaders = rules::has_invaders(lb) || !fed_by.is_empty();
        let build_next_num = if l.is_ocean {
            0
        } else {
            explore.candidates.iter().filter(|c| rules::card_matches(&c.card, l) && will_have_invaders).count() as u32
        };
        let sole_source_for: Vec<String> = if rules::is_explore_source(lb) && !lb.is_ocean {
            after_build
                .iter()
                .filter(|x| !x.is_ocean && !x.coastal && x.id != l.id)
                .filter(|x| {
                    let f = fed(x);
                    f.len() == 1 && f[0] == lb.id
                })
                .map(|x| x.id.clone())
                .collect()
        } else {
            Vec::new()
        };

        let mut badges = Vec::new();
        if ravage_now.is_some() {
            badges.push("ravage-now".to_string());
        }
        if build_now.is_some() {
            badges.push("build-now".into());
        }
        if ravage_next == Ravage2::Certain {
            badges.push("ravage-next".into());
        }
        if ravage_now.is_some() && ravage_next == Ravage2::Certain {
            badges.push("back-to-back".into());
        }
        if explore_access == "dead" && !rules::has_invaders(l) {
            badges.push("explore-dead".into());
        }
        if !sole_source_for.is_empty() {
            badges.push("sole-source".into());
        }

        let (fast_window, slow_window) =
            timing_windows(l, ravage_now.as_ref(), build_now.as_deref(), (en, ed), &ravage_next, &fed_by, &sole_source_for, position);

        lands.push(LandOutlook {
            id: l.id.clone(),
            terrain: l.terrain.clone(),
            coastal: l.coastal,
            explorers: l.explorers,
            towns: l.towns,
            cities: l.cities,
            dahan: l.dahan,
            blight: l.blight,
            presence: l.presence.values().sum(),
            explore_access: explore_access.into(),
            fed_by,
            sole_source_for,
            ravage_now,
            build_now,
            explore_chance: (en, ed),
            ravage_next,
            build_next_chance: (build_next_num, ed),
            badges,
            fast_window,
            slow_window,
        });
    }

    InvaderView {
        phase: InvaderPhaseForecast {
            turn,
            position: position.into(),
            ravage_cards: inv.ravage.clone(),
            build_cards: inv.build.clone(),
            ravages,
            builds,
            explore,
            notes,
            high_immigration_cards,
            high_immigration,
            escalation,
            rule_actions,
            build_ravages,
        },
        lands,
        stages,
        after_build,
    }
}

/// What a Fast action vs a Slow action in this land can still change.
/// Mechanical statements only (no recommendation): Fast resolves before the
/// Invader Phase, Slow after it, so Slow shapes next turn's cards.
#[allow(clippy::too_many_arguments)]
fn timing_windows(
    l: &PublicLand,
    ravage_now: Option<&RavageLedger>,
    build_now: Option<&str>,
    explore: (u32, u32),
    ravage_next: &Ravage2,
    fed_by: &[String],
    sole_source_for: &[String],
    position: &str,
) -> (Vec<String>, Vec<String>) {
    let mut fast = Vec::new();
    let mut slow = Vec::new();
    if l.is_ocean {
        return (fast, slow);
    }
    let before = position == "before";
    if let Some(r) = ravage_now {
        let result = if r.adds_blight {
            format!("{} damage vs health {}: +1 blight{}", r.damage_after_defend, r.land_health, if r.cascades { ", cascades" } else { "" })
        } else {
            format!("{} damage vs health {}: no blight", r.damage_after_defend, r.land_health)
        };
        if before {
            fast.push(format!("Before this turn's Ravage ({result})."));
            slow.push("After this turn's Ravage: too late for it.".into());
        }
    }
    if let Some(piece) = build_now
        && before {
            fast.push(format!("Before this turn's Build: removing every Invader here stops the +{piece}."));
            slow.push(format!("After the Build: the new {piece} stays and counts next turn."));
        }
    if explore.0 > 0 && !l.coastal && before {
        fast.push(format!(
            "Explore {} in {}: removing its source{} ({}) before the Explore stops it.",
            explore.0,
            explore.1,
            if fed_by.len() > 1 { "s" } else { "" },
            fed_by.join(", ")
        ));
    }
    if explore.0 > 0 && !rules::has_invaders(l) {
        slow.push(format!(
            "If explored ({} in {}), clearing that lone Explorer after the Explore stops next turn's Build here.",
            explore.0, explore.1
        ));
    }
    if *ravage_next == Ravage2::Certain {
        let both = "Fast or Slow this turn both act before next turn's Ravage here.";
        fast.push(both.into());
        slow.push(both.into());
    }
    if !sole_source_for.is_empty() {
        slow.push(format!(
            "Only Explore source for {}: clearing its Towns/Cities leaves {} unexplorable.",
            sole_source_for.join(", "),
            if sole_source_for.len() > 1 { "them" } else { "it" }
        ));
    }
    (fast, slow)
}

fn titles(cards: &[PublicInvaderCard]) -> String {
    cards.iter().map(|c| c.id.as_str()).collect::<Vec<_>>().join(", ")
}
