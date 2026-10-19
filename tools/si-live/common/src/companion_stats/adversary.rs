//! Adversary and scenario rules that change the forecast's numbers (rules
//! register §4; Brett, 2026-10-19: the companion shows "the ACTUAL things,
//! like Sweden's extra damage"). The Invader forecast calls one hook per
//! point where a rule acts. A rule the players resolve by choosing (Q21:
//! players choose, best for them) is listed with its options and not applied;
//! a rule that decides on its own is applied to the forecast board.
//!
//! Setup pieces and deck recipes need no hook: the game state already shows
//! them.

use super::invaders::{BuildForecast, EscalationForecast, ExploreCandidate, RavageForecast};
use crate::public::{PublicInvaderCard, PublicLand, PublicSnapshot, PublicStatusEffect, StatusParam};
use crate::rules::{self, RavageLedger};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const ENGLAND: &str = "TheKingdomOfEngland";
pub const SWEDEN: &str = "TheKingdomOfSweden";
pub const RUSSIA: &str = "TheTsardomOfRussia";
pub const PRUSSIA: &str = "TheKingdomOfBrandenburgPrussia";
pub const FRANCE: &str = "TheKingdomOfFrance";
pub const SCOTLAND: &str = "TheKingdomOfScotland";
pub const LIVESTOCK: &str = "TheHabsburgMonarchy";
pub const MINING: &str = "HabsburgMiningExpedition";

pub type Nbrs<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;

/// An adversary or scenario action outside the printed Ravage, Build and
/// Explore, with the lands it touches. `pick` is empty when the rule decides
/// (the forecast applies it); otherwise it says what the players choose.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RuleAction {
    pub rule: String,
    /// "after the Ravage", "after the Build", "with the Explore".
    pub when: String,
    pub text: String,
    pub lands: Vec<BuildForecast>,
    pub pick: String,
}

/// One way the Invaders can win, as a count against its limit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LossClock {
    pub title: String,
    pub adversary: String,
    pub now: i64,
    /// After this Invader Phase as forecast, when it changes the count.
    pub after: Option<i64>,
    pub limit: i64,
    /// The Invaders win when the count is ≥ the limit ("at") or > it ("over").
    pub trigger: String,
    /// The land holding the maximum, for a per-land clock.
    pub at: Option<String>,
    /// The clock in one line, for a glance.
    #[serde(default)]
    pub line: String,
    /// When the game checks it, if not continuously.
    #[serde(default)]
    pub checked: String,
    pub note: String,
}

pub struct Adv<'a> {
    pub s: &'a PublicSnapshot,
    /// Players: one per spirit.
    pub players: i64,
}

impl<'a> Adv<'a> {
    pub fn new(s: &'a PublicSnapshot) -> Self {
        Adv { s, players: s.spirits.len().max(1) as i64 }
    }

    /// The adversary's level, or −1 when it isn't in the game.
    pub fn level(&self, id: &str) -> i64 {
        self.s.adversaries.iter().find(|a| a.id == id).map(|a| a.level).unwrap_or(-1)
    }

    pub fn has(&self, id: &str, level: i64) -> bool {
        self.level(id) >= level
    }

    pub fn blitz(&self) -> bool {
        self.s.scenario.as_deref() == Some("Blitz")
    }
}

fn damage_effect(source: String, amount: i64, per: Option<&str>) -> PublicStatusEffect {
    let mut params: BTreeMap<String, StatusParam> = [("DamageAdjustment".to_string(), StatusParam::Int(amount))].into_iter().collect();
    if let Some(kind) = per {
        params.insert("PerInvaderOfType".into(), StatusParam::List(vec![kind.into()]));
    }
    PublicStatusEffect { kind: "AdjustRavageDamage".into(), source: Some(source), source_kind: "adversary".into(), params }
}

fn invaders(l: &PublicLand) -> u32 {
    l.explorers + l.towns + l.cities
}

fn adjacent<'b>(l: &PublicLand, board: &'b [PublicLand], nbrs: &Nbrs) -> Vec<&'b PublicLand> {
    nbrs.get(l.id.as_str()).into_iter().flatten().filter_map(|n| board.iter().find(|x| x.id == *n)).filter(|x| !x.is_ocean).collect()
}

fn land_forecast(l: &PublicLand, adds: &str, notes: Vec<String>) -> BuildForecast {
    BuildForecast { land: l.id.clone(), adds: adds.into(), done: false, notes, card: None }
}

// ------------------------------------------------------------------ Ravage

/// Damage changes for every Ravage.
pub fn ravage_effects(a: &Adv) -> Vec<PublicStatusEffect> {
    let mut out = Vec::new();
    // Sweden L3, Fine Steel for Tools and Guns: "Town deal 3 Damage. City
    // deal 5 Damage" (playtest 2026-10-19 Sweden game: City3-5 in the log).
    if a.has(SWEDEN, 3) {
        out.push(damage_effect("Fine Steel, Sweden L3: Towns deal 3".into(), 1, Some("Town")));
        out.push(damage_effect("Fine Steel, Sweden L3: Cities deal 5".into(), 2, Some("City")));
    }
    // Russia L1, Hunters Seek Shell and Hide: "Explorer do +1 Damage"
    // (playtest 2026-10-19 Russia game: an Explorer dealt 2 on Wetland A2).
    if a.has(RUSSIA, 1) {
        out.push(damage_effect("Hunters Seek Shell and Hide, Russia L1: Explorers +1".into(), 1, Some("Explorer")));
    }
    out
}

/// Damage changes that depend on the land's neighbors.
pub fn land_effects(a: &Adv, l: &PublicLand, board: &[PublicLand], nbrs: &Nbrs) -> Vec<PublicStatusEffect> {
    let mut out = Vec::new();
    // Habsburg Livestock L6, Far-Flung Herds: "Ravages do +2 Damage (total)
    // if any adjacent lands have Town. (This does not cause lands without
    // Invaders to Ravage.)" A per-land bonus: it needs the Invaders' own damage.
    if a.has(LIVESTOCK, 6) && adjacent(l, board, nbrs).iter().any(|x| x.towns > 0) {
        out.push(damage_effect("Far-Flung Herds, Habsburg L6: a Town is adjacent".into(), 2, None));
    }
    out
}

/// Does this Ravage card Ravage here?
pub fn ravages_in(a: &Adv, card: &PublicInvaderCard, l: &PublicLand) -> bool {
    // Russia L3, Competition Among Hunters: "Ravage Cards also match lands
    // with 3 or more Explorer" (once per card even if it already matched).
    rules::ravages_in(card, l) || (a.has(RUSSIA, 3) && !l.is_ocean && l.explorers >= 3)
}

/// Habsburg Mining L1: "Lands with 3 or more Invaders are Mining lands."
pub fn mining_land(a: &Adv, l: &PublicLand) -> bool {
    a.has(MINING, 1) && !l.is_ocean && invaders(l) >= 3
}

/// The Ravage doesn't happen: no damage, no Blight, no counterattack.
fn cancel(ledger: &mut RavageLedger, src: String) {
    ledger.adds_blight = false;
    ledger.cascades = false;
    ledger.extra_blight = 0;
    ledger.damage_after_defend = 0;
    ledger.dahan_destroyed = 0;
    ledger.counterattack_damage = 0;
    ledger.counterattack_can_destroy.clear();
    ledger.counterattack_fear = 0;
    ledger.counterattack_clears = false;
    ledger.presence_at_risk.clear();
    ledger.applied.push(format!("No Ravage here ({src})"));
    ledger.cancelled = Some(src);
}

/// Rules that act on a Ravage's result.
pub fn after_ledger(a: &Adv, l: &PublicLand, ledger: &mut RavageLedger, effects: &[PublicStatusEffect]) {
    if ledger.cancelled.is_some() {
        return;
    }
    // Habsburg Mining L1: in Mining lands, "Disease and modifiers to Disease
    // affect Ravage Actions as though they were Build Actions": a Disease
    // stops the Ravage and is removed.
    let disease_ignored = effects.iter().any(|e| {
        e.kind == "IgnoreToken" && matches!(e.params.get("TokenTypesToIgnore"), Some(StatusParam::List(t)) if t.iter().any(|x| x == "Disease"))
    });
    if mining_land(a, l) && l.tokens.get("Disease").copied().unwrap_or(0) > 0 && !disease_ignored {
        cancel(ledger, "Disease in a Mining land, Habsburg Mining L1 (1 Disease removed)".into());
        return;
    }
    let overkill = ledger.damage_after_defend - ledger.land_health;
    // Sweden L1, Heavy Mining: "When a Ravage does 4+ overkill to the land,
    // add an extra Blight. (This does not cause extra cascades / Presence
    // destruction.)" Sweden game, Turn 3: 7 Damage to Sands F8 added 2.
    if a.has(SWEDEN, 1) && ledger.adds_blight && overkill >= 4 {
        ledger.extra_blight += 1;
        ledger.applied.push(format!("Heavy Mining, Sweden L1: {overkill} overkill adds an extra Blight (no extra cascade or Presence loss)"));
    }
    // Scotland L5, Runoff and Bilgewater: "After a Ravage Action adds Blight
    // to a Coastal land, add 1 Blight to that board's Ocean (without
    // cascading)." That Blight also comes off the Blight Card.
    if a.has(SCOTLAND, 5) && ledger.adds_blight && l.coastal {
        ledger.extra_blight += 1;
        ledger.applied.push("Runoff and Bilgewater, Scotland L5: +1 Blight to this board's Ocean (no cascade)".into());
    }
    // Habsburg Mining L1, Avarice Rewarded: "When Blight added by a Ravage
    // Action would cascade, instead Upgrade 1 Explorer/Town (before Dahan
    // counterattack)."
    if a.has(MINING, 1) && ledger.cascades {
        ledger.cascades = false;
        ledger.applied.push("Avarice Rewarded, Habsburg Mining L1: no cascade; 1 Explorer/Town upgrades instead, before the counterattack".into());
    }
    // Russia L1: "When Ravage adds Blight to a land (including cascades),
    // Destroy 1 Beast in that land." Each one counts for Hunters Swarm the Island.
    if a.has(RUSSIA, 1) && ledger.adds_blight && l.tokens.get("Beast").copied().unwrap_or(0) > 0 {
        ledger.applied.push("Russia L1: the Blight destroys 1 Beast here (counts toward Hunters Swarm the Island)".into());
    }
    // Russia L2, A Sense for Impending Disaster: "The first time each Action
    // would Destroy Explorer: If possible, 1 of those Explorer is instead
    // Pushed; 1 Fear when you do so."
    if a.has(RUSSIA, 2) && ledger.counterattack_can_destroy.iter().any(|k| k == "Explorer") {
        ledger.applied.push("Russia L2: of the Explorers the counterattack destroys, 1 is Pushed instead (+1 Fear)".into());
    }
    // Habsburg Livestock, Irreparable Damage: Blight from Ravages with 6+ overkill is tracked.
    if a.has(LIVESTOCK, 0) && ledger.adds_blight && overkill >= 6 {
        ledger.applied.push(format!("{overkill} overkill: this Blight counts toward Irreparable Damage (Habsburg loss)"));
    }
}

// ------------------------------------------------------------------ Build

/// A land without Invaders that the Build card still builds in, with why.
pub fn builds_without_invaders(a: &Adv, card: &PublicInvaderCard, l: &PublicLand, before: &[PublicLand], nbrs: &Nbrs) -> Option<String> {
    if !rules::card_matches(card, l) || rules::has_invaders(l) {
        return None;
    }
    // England L1 (see `rules::builds_without_invaders`).
    if a.has(ENGLAND, 1) && rules::builds_without_invaders(card, l, before, nbrs) {
        let n = rules::adjacent_towns_cities(l, before, nbrs);
        return Some(format!("No Invaders, but {n} Town/City adjacent (England L1, Indentured Servants Earn Land)"));
    }
    // Scotland L3, Chart the Coastline: "In Coastal lands, Build Cards affect
    // lands without Invaders, so long as there is an adjacent City."
    if a.has(SCOTLAND, 3) && l.coastal && adjacent(l, before, nbrs).iter().any(|x| x.cities > 0) {
        return Some("No Invaders, but a City adjacent (Scotland L3, Chart the Coastline)".into());
    }
    None
}

/// Rules that act when a piece is built in `board[i]`. Returns notes, and
/// the piece actually built when a rule changes it.
pub fn after_build(a: &Adv, board: &mut [PublicLand], i: usize, piece: &str, nbrs: &Nbrs, town_health: i64) -> (Vec<String>, Option<&'static str>) {
    let mut notes = Vec::new();
    let mut built = None;
    // Habsburg Livestock L2, More Rural Than Urban: "when Invaders would Build
    // 1 City in an Inland land, they instead Build 2 Town."
    if a.has(LIVESTOCK, 2) && piece == "City" && !board[i].coastal {
        let l = &mut board[i];
        l.cities -= 1;
        l.pieces.retain({
            let mut gone = false;
            move |p| {
                if !gone && p.kind == "City" && p.damage == 0 {
                    gone = true;
                    return false;
                }
                true
            }
        });
        rules::add_built(l, "Town", town_health);
        rules::add_built(l, "Town", town_health);
        notes.push("2 Towns instead of a City (More Rural Than Urban, Habsburg L2)".into());
        built = Some("Town");
    }
    // France L2, Slave Labor: "After Invaders Build in a land with 2 Explorer
    // or more, replace all but 1 Explorer there with an equal number of Town."
    if a.has(FRANCE, 2) && board[i].explorers >= 2 {
        let n = board[i].explorers - 1;
        let l = &mut board[i];
        l.explorers = 1;
        let mut dropped = 0;
        l.pieces.retain(|p| {
            if p.kind == "Explorer" && dropped < n {
                dropped += 1;
                return false;
            }
            true
        });
        for _ in 0..n {
            rules::add_built(l, "Town", town_health);
        }
        notes.push(format!("{n} Explorer{} become Town{} (Slave Labor, France L2)", if n == 1 { "" } else { "s" }, if n == 1 { "" } else { "s" }));
    }
    // France L4, Triangle Trade: "Whenever Invaders Build a Coastal City, add
    // 1 Town to the adjacent land with the fewest Town."
    if a.has(FRANCE, 4) && piece == "City" && board[i].coastal {
        let me = board[i].clone();
        let adj: Vec<(String, u32)> = adjacent(&me, board, nbrs).iter().map(|x| (x.id.clone(), x.towns)).collect();
        if let Some(fewest) = adj.iter().map(|(_, t)| *t).min() {
            let tied: Vec<&String> = adj.iter().filter(|(_, t)| *t == fewest).map(|(id, _)| id).collect();
            if let [only] = tied.as_slice() {
                if let Some(x) = board.iter_mut().find(|x| x.id == **only) {
                    rules::add_built(x, "Town", town_health);
                }
                notes.push(format!("Triangle Trade (France L4): +1 Town in {only}"));
            } else {
                notes.push(format!("Triangle Trade (France L4): +1 Town in one of {} (you pick)", tied.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
            }
        }
    }
    (notes, built)
}

/// Habsburg Mining L1, Ceaseless Mining: "During the Build Step, Build Cards
/// cause Ravage Actions (instead of Build Actions)" in Mining lands. Only the
/// ordinary Build Step's card (FAQ: not High Immigration's).
pub fn ceaseless_mining(a: &Adv, l: &PublicLand) -> bool {
    mining_land(a, l)
}

// ------------------------------------------------------------------ between steps

/// Lands that would newly become explorable for `cards` if `id` had a Town.
fn opens_explore(id: &str, board: &[PublicLand], nbrs: &Nbrs, cards: &[&PublicInvaderCard], range: i64) -> Vec<String> {
    let mut with = board.to_vec();
    if let Some(l) = with.iter_mut().find(|l| l.id == id) {
        l.towns += 1;
    }
    board
        .iter()
        .filter(|l| !l.is_ocean && cards.iter().any(|c| rules::card_matches(c, l)))
        .filter(|l| rules::explore_sources_within(l, board, nbrs, range).is_empty() && !rules::explore_sources_within(l, &with, nbrs, range).is_empty())
        .map(|l| l.id.clone())
        .collect()
}

fn explore_note(opens: &[String]) -> String {
    if opens.is_empty() { "opens no new Explore".into() } else { format!("opens Explore into {}", opens.join(", ")) }
}

/// Rules after the Ravage Step, before the Build. Applies the ones that decide.
pub fn after_ravage_step(a: &Adv, board: &mut [PublicLand], ravages: &[RavageForecast], ravage_cards: &[PublicInvaderCard], nbrs: &Nbrs, turn: u32, town_health: i64) -> Vec<RuleAction> {
    let mut out = Vec::new();
    // Scotland L6, Exports Fuel Inward Growth: "After the Ravage Step, add 1
    // Town to each Inland land that matches a Ravage Card and is within 1
    // Range of Town/City."
    if a.has(SCOTLAND, 6) {
        let before = board.to_vec();
        let mut lands = Vec::new();
        for l in board.iter_mut().filter(|l| !l.is_ocean && !l.coastal && ravage_cards.iter().any(|c| rules::card_matches(c, l))) {
            let src = before.iter().find(|x| x.id == l.id);
            let near = src.is_some_and(|s| s.towns + s.cities > 0 || adjacent(s, &before, nbrs).iter().any(|x| x.towns + x.cities > 0));
            if near {
                rules::add_built(l, "Town", town_health);
                lands.push(land_forecast(l, "Town", vec![]));
            }
        }
        if !lands.is_empty() {
            out.push(RuleAction { rule: "Exports Fuel Inward Growth (Scotland L6)".into(), when: "after the Ravage".into(), text: "+1 Town in each Inland land matching a Ravage Card within 1 of a Town/City".into(), lands, pick: String::new() });
        }
    }
    // Russia L6, Pressure for Fast Profit: "After the Ravage Step of turn 2+,
    // on each board where it added no Blight: In the land with the most
    // Explorer (min. 1), add 1 Explorer and 1 Town."
    if a.has(RUSSIA, 6) && turn >= 2 {
        let boards: BTreeSet<String> = board.iter().map(|l| l.board.clone()).collect();
        for b in boards {
            let blighted = ravages.iter().any(|r| r.ledger.adds_blight && board.iter().any(|l| l.id == r.land && l.board == b));
            if blighted {
                continue;
            }
            let most = board.iter().filter(|l| l.board == b && !l.is_ocean).map(|l| l.explorers).max().unwrap_or(0);
            if most == 0 {
                continue;
            }
            let tied: Vec<String> = board.iter().filter(|l| l.board == b && l.explorers == most).map(|l| l.id.clone()).collect();
            let mut lands = Vec::new();
            if let [only] = tied.as_slice() {
                if let Some(l) = board.iter_mut().find(|l| l.id == *only) {
                    rules::add_built(l, "Explorer", 1);
                    rules::add_built(l, "Town", town_health);
                    lands.push(land_forecast(l, "Explorer + Town", vec![]));
                }
            } else {
                for id in &tied {
                    if let Some(l) = board.iter().find(|l| l.id == *id) {
                        lands.push(land_forecast(l, "Explorer + Town", vec![format!("tied at {most} Explorers")]));
                    }
                }
            }
            out.push(RuleAction {
                rule: "Pressure for Fast Profit (Russia L6)".into(),
                when: "after the Ravage".into(),
                text: format!("Board {b} adds no Ravage Blight: +1 Explorer and +1 Town where the most Explorers are"),
                lands,
                pick: if tied.len() > 1 { "you pick among the tied lands".into() } else { String::new() },
            });
        }
    }
    // Sweden L5, Mining Rush: "When Ravaging adds at least 1 Blight to a land,
    // also add 1 Town to an adjacent land without Town/City. Cascading Blight
    // does not cause this effect."
    if a.has(SWEDEN, 5) {
        for r in ravages.iter().filter(|r| r.ledger.adds_blight && !r.done) {
            let Some(l) = board.iter().find(|l| l.id == r.land) else { continue };
            let lands: Vec<BuildForecast> = adjacent(l, board, nbrs).into_iter().filter(|x| x.towns + x.cities == 0).map(|x| land_forecast(x, "Town", vec![])).collect();
            if !lands.is_empty() {
                out.push(RuleAction {
                    rule: "Mining Rush (Sweden L5)".into(),
                    when: "after the Ravage".into(),
                    text: format!("Blight in {}: +1 Town in an adjacent land without Town/City", r.land),
                    lands,
                    pick: "you pick 1".into(),
                });
            }
        }
    }
    out
}

/// Rules after the Build Step, before the Explore.
pub fn after_build_step(a: &Adv, board: &mut [PublicLand], build_cards: &[PublicInvaderCard], explore_cards: &[&PublicInvaderCard], nbrs: &Nbrs, range: i64, town_health: i64) -> Vec<RuleAction> {
    let mut out = Vec::new();
    // Habsburg Livestock L1, Migratory Herders: "After the normal Build Step:
    // In each land matching a Build Card, Gather 1 Town from a land not
    // matching a Build Card. (In board/land order.)"
    if a.has(LIVESTOCK, 1) && !build_cards.is_empty() {
        let matches = |l: &PublicLand| build_cards.iter().any(|c| rules::card_matches(c, l));
        let mut order: Vec<String> = board.iter().filter(|l| matches(l)).map(|l| l.id.clone()).collect();
        order.sort_by_key(|id| board.iter().find(|l| l.id == *id).map(|l| (l.board.clone(), l.number)));
        let mut lands = Vec::new();
        for id in order {
            let Some(to) = board.iter().find(|l| l.id == id).cloned() else { continue };
            let from: Vec<String> = adjacent(&to, board, nbrs).into_iter().filter(|x| !matches(x) && x.towns > 0).map(|x| x.id.clone()).collect();
            match from.as_slice() {
                [] => {}
                [only] => {
                    if let Some(src) = board.iter_mut().find(|l| l.id == *only) {
                        src.towns -= 1;
                        if let Some(i) = src.pieces.iter().position(|p| p.kind == "Town") {
                            src.pieces.remove(i);
                        }
                    }
                    if let Some(dst) = board.iter_mut().find(|l| l.id == id) {
                        rules::add_built(dst, "Town", town_health);
                    }
                    lands.push(land_forecast(&to, "Town", vec![format!("from {only}")]));
                }
                many => lands.push(land_forecast(&to, "Town", vec![format!("from one of {} (you pick)", many.join(", "))])),
            }
        }
        if !lands.is_empty() {
            out.push(RuleAction { rule: "Migratory Herders (Habsburg L1)".into(), when: "after the Build".into(), text: "Each land matching a Build Card gathers 1 Town from a non-matching neighbor".into(), lands, pick: String::new() });
        }
    }
    // Habsburg Mining L3, Mining Boom (I): "After the Build Step, on each
    // board: Choose a land with Explorer. Upgrade 1 Explorer there." L5
    // (Mining Boom II) instead: "Build there, then Upgrade 1 Explorer." The
    // new Town is an Explore source.
    if a.has(MINING, 3) {
        let ii = a.has(MINING, 5);
        let lands: Vec<BuildForecast> = board
            .iter()
            .filter(|l| !l.is_ocean && l.explorers > 0)
            .map(|l| {
                let opens = opens_explore(&l.id, board, nbrs, explore_cards, range);
                land_forecast(l, if ii { "Build + upgrade" } else { "Explorer → Town" }, vec![explore_note(&opens)])
            })
            .collect();
        if !lands.is_empty() {
            let (rule, text) = if ii {
                ("Mining Boom II (Habsburg Mining L5)", "On each board, a land with Explorers Builds, then upgrades 1 Explorer")
            } else {
                ("Mining Boom I (Habsburg Mining L3)", "On each board, upgrade 1 Explorer in a land with Explorers")
            };
            out.push(RuleAction { rule: rule.into(), when: "after the Build".into(), text: text.into(), lands, pick: "you pick 1 per board".into() });
        }
    }
    let _ = town_health;
    out
}

// ------------------------------------------------------------------ Escalation

/// What a flagged Explore card's Escalation does, for the adversary that has
/// one modeled. `candidates`: the possible Explore cards with their lands.
#[allow(clippy::too_many_arguments)]
pub fn escalation(
    a: &Adv,
    board: &[PublicLand],
    candidates: &[ExploreCandidate],
    revealed: bool,
    build_cards: &[PublicInvaderCard],
    effects: &[PublicStatusEffect],
    cancels: &rules::BuildCancels,
    ravages: &[RavageForecast],
    nbrs: &Nbrs,
    range: i64,
    notes: &mut Vec<String>,
) -> Option<EscalationForecast> {
    let flagged: Vec<&ExploreCandidate> = candidates.iter().filter(|c| c.card.escalation).collect();
    if flagged.is_empty() {
        return None;
    }
    let chance = (flagged.len() as u32, candidates.len() as u32);
    let boards: Vec<String> = a.s.boards.iter().map(|b| b.id.clone()).collect();
    let on = |b: &str| -> Vec<&PublicLand> { board.iter().filter(|l| l.board == b && !l.is_ocean).collect() };
    let if_drawn = |c: &ExploreCandidate| if revealed { None } else { Some(format!("if {} is drawn", c.card.id)) };
    let make = |id: &str, title: String, text: String, builds: Vec<BuildForecast>, each_land: bool, when: &str, pick: &str| {
        let _ = id;
        Some(EscalationForecast { title, text: text.replace("<i>", "").replace("</i>", ""), chance, builds, each_land, when: when.into(), pick: pick.into() })
    };
    let reg = |id: &str| crate::rules_register::escalation(id).map(|(t, x, _)| (t, x)).unwrap_or_default();

    // England, Building Boom: "On each board with Town/City, build in the land
    // with the most Town/City", before the Explore. That land already has a
    // Town/City, so it adds no Explore source (rules register R-ADV-ENG-L0a).
    if a.has(ENGLAND, 0) {
        let (title, text) = reg(ENGLAND);
        let mut builds = Vec::new();
        for b in &boards {
            let top = on(b).into_iter().map(|l| l.towns + l.cities).max().unwrap_or(0);
            if top == 0 {
                continue;
            }
            let tied: Vec<&PublicLand> = on(b).into_iter().filter(|l| l.towns + l.cities == top).collect();
            for l in &tied {
                let (piece, mut bnotes) = rules::build_outcome_with(l, effects, &mut cancels.clone());
                if tied.len() > 1 {
                    bnotes.push(format!("tied at {top} Town/City with {} other land(s): the player picks one", tied.len() - 1));
                }
                match piece {
                    Some(p) => builds.push(BuildForecast { land: l.id.clone(), adds: p.to_string(), done: false, notes: bnotes, card: None }),
                    None => notes.push(format!("{}: no {title} Build ({})", l.id, bnotes.join("; "))),
                }
            }
        }
        return make(ENGLAND, title, text, builds, false, "before the Explore", "one per board");
    }
    // Sweden, Swayed by the Invaders: "After Invaders Explore into each land,
    // if that land has at least as many Invaders as Dahan, replace 1 Dahan
    // with 1 Town" (Sweden game, Turn 4: Wetland F3's only Dahan).
    if a.has(SWEDEN, 0) {
        let (title, text) = reg(SWEDEN);
        let mut builds = Vec::new();
        for c in &flagged {
            for id in &c.lands {
                let Some(l) = board.iter().find(|l| l.id == *id) else { continue };
                if l.dahan > 0 && invaders(l) + 1 >= l.dahan {
                    let mut n = vec![format!("replaces 1 of {} Dahan", l.dahan)];
                    n.extend(if_drawn(c));
                    builds.push(BuildForecast { land: l.id.clone(), adds: "Town".into(), done: false, notes: n, card: Some(c.card.id.clone()) });
                }
            }
        }
        return make(SWEDEN, title, text, builds, true, "after the Explore", "every land listed");
    }
    // Brandenburg-Prussia, Land Rush: "On each board with Town/City, add 1
    // Town to a land without Town", before Exploring; the Town is an Explore
    // source that turn (FAQ). The players pick; lands that open no new
    // Explore are listed first.
    if a.has(PRUSSIA, 0) {
        let (title, text) = reg(PRUSSIA);
        let cards: Vec<&PublicInvaderCard> = flagged.iter().map(|c| &c.card).collect();
        // A land whose Towns the Dahan counterattack can destroy may have none
        // by then (Prussia game, Turn 2: Mountain B7's Town fell and Land Rush
        // went there).
        let towns_may_fall = |l: &PublicLand| {
            l.towns > 0
                && ravages.iter().any(|r| {
                    r.land == l.id && !r.done && r.ledger.counterattack_can_destroy.iter().filter(|k| *k == "Town").count() as u32 >= l.towns
                })
        };
        let mut builds = Vec::new();
        for b in &boards {
            if on(b).into_iter().all(|l| l.towns + l.cities == 0) {
                continue;
            }
            let mut opts: Vec<(Vec<String>, &PublicLand, bool)> = on(b)
                .into_iter()
                .filter(|l| l.towns == 0 || towns_may_fall(l))
                .map(|l| (opens_explore(&l.id, board, nbrs, &cards, range), l, l.towns > 0))
                .collect();
            opts.sort_by_key(|(o, l, maybe)| (*maybe, o.len(), l.number));
            builds.extend(opts.into_iter().map(|(o, l, maybe)| {
                let mut n = vec![explore_note(&o)];
                if maybe {
                    n.push("only if the Dahan counterattack destroys its Town".into());
                }
                land_forecast(l, "Town", n)
            }));
        }
        return make(PRUSSIA, title, text, builds, false, "before the Explore", "you pick 1 per board");
    }
    // France, Demand for New Cash Crops: "After Exploring, on each board, pick
    // a land of the shown terrain. If it has Town/City, add 1 Blight.
    // Otherwise, add 1 Town."
    if a.has(FRANCE, 0) {
        let (title, text) = reg(FRANCE);
        let mut builds = Vec::new();
        for c in &flagged {
            for l in board.iter().filter(|l| !l.is_ocean && c.card.terrains.contains(&l.terrain)) {
                let adds = if l.towns + l.cities > 0 { "Blight" } else { "Town" };
                let mut n = Vec::new();
                n.extend(if_drawn(c));
                builds.push(BuildForecast { land: l.id.clone(), adds: adds.into(), done: false, notes: n, card: Some(c.card.id.clone()) });
            }
        }
        return make(FRANCE, title, text, builds, false, "after the Explore", "you pick 1 per board");
    }
    // Habsburg Livestock, Seek Prime Territory: "After Exploring: On each board
    // with 4 or fewer Blight, add 1 Town to a land without Town/Blight. On
    // each board with 2 or fewer Blight, do so again."
    if a.has(LIVESTOCK, 0) {
        let (title, text) = reg(LIVESTOCK);
        let mut builds = Vec::new();
        for b in &boards {
            let blight: u32 = on(b).into_iter().map(|l| l.blight).sum();
            let n = if blight <= 2 { 2 } else if blight <= 4 { 1 } else { 0 };
            if n == 0 {
                continue;
            }
            for l in on(b).into_iter().filter(|l| l.towns == 0 && l.blight == 0) {
                builds.push(land_forecast(l, "Town", vec![format!("{n} Town{} on board {b}", if n == 1 { "" } else { "s" })]));
            }
        }
        return make(LIVESTOCK, title, text, builds, false, "after the Explore", "you pick per board");
    }
    // Russia, Stalk the Predators: "On each board: Add 2 Explorer (total)
    // among lands with Beasts", before the Explore; Explorers are never
    // Explore sources (NQ-G). Explains the Russia game's Mountain A1 Explorer.
    if a.has(RUSSIA, 0) {
        let (title, text) = reg(RUSSIA);
        let builds: Vec<BuildForecast> = board
            .iter()
            .filter(|l| !l.is_ocean && l.tokens.get("Beast").copied().unwrap_or(0) > 0)
            .map(|l| land_forecast(l, "Explorer", vec![format!("{} Explorer{} there now", l.explorers, if l.explorers == 1 { "" } else { "s" })]))
            .collect();
        return make(RUSSIA, title, text, builds, false, "before the Explore", "2 Explorers per board among these (you split them)");
    }
    // Scotland, Ports Sprawl Outward: "On the single board with the most
    // Coastal Town/City, add 1 Town to the N lands with the fewest Town
    // (N = # of players)", before Exploring; the Towns are Explore sources.
    if a.has(SCOTLAND, 0) {
        let (title, text) = reg(SCOTLAND);
        let cards: Vec<&PublicInvaderCard> = flagged.iter().map(|c| &c.card).collect();
        let coastal = |b: &str| on(b).into_iter().filter(|l| l.coastal).map(|l| l.towns + l.cities).sum::<u32>();
        let top = boards.iter().map(|b| coastal(b)).max().unwrap_or(0);
        let mut builds = Vec::new();
        for b in boards.iter().filter(|b| coastal(b) == top) {
            let fewest = on(b).into_iter().map(|l| l.towns).min().unwrap_or(0);
            for l in on(b).into_iter().filter(|l| l.towns == fewest) {
                let opens = opens_explore(&l.id, board, nbrs, &cards, range);
                builds.push(land_forecast(l, "Town", vec![explore_note(&opens)]));
            }
        }
        let pick = format!("you pick {} of the fewest-Town lands", a.players);
        return make(SCOTLAND, title, text, builds, false, "before the Explore", &pick);
    }
    // Habsburg Mining, Mining Tunnels: "After Advancing Invader Cards: On each
    // board, Explore in 2 lands whose terrains don't match a Ravage or Build
    // Card (no source required)." After the Advance the Build card is the
    // Ravage card and the flagged card is the Build card.
    if a.has(MINING, 0) {
        let (title, text) = reg(MINING);
        let mut builds = Vec::new();
        for c in &flagged {
            for l in board.iter().filter(|l| !l.is_ocean) {
                let matched = build_cards.iter().any(|b| rules::card_matches(b, l)) || rules::card_matches(&c.card, l);
                if !matched {
                    let mut n = Vec::new();
                    n.extend(if_drawn(c));
                    builds.push(BuildForecast { land: l.id.clone(), adds: "Explorer".into(), done: false, notes: n, card: Some(c.card.id.clone()) });
                }
            }
        }
        return make(MINING, title, text, builds, false, "after the cards advance", "you pick 2 per board");
    }
    None
}

// ------------------------------------------------------------------ Explore

/// What changes about one Explore card's result.
pub fn explore_notes(a: &Adv, card: &PublicInvaderCard, lands: &[String], board: &[PublicLand]) -> Vec<String> {
    let mut out = Vec::new();
    let find = |id: &String| board.iter().find(|l| l.id == *id);
    let names = |v: Vec<&PublicLand>| v.iter().map(|l| l.id.clone()).collect::<Vec<_>>().join(", ");
    // Scotland L1, Trading Port: "After Setup, in Coastal lands, Explore Cards
    // add 1 Town instead of 1 Explorer. 'Coastal Lands' Invader Cards do this
    // for maximum 2 lands per board."
    if a.has(SCOTLAND, 1) {
        let coastal: Vec<&PublicLand> = lands.iter().filter_map(find).filter(|l| l.coastal).collect();
        if !coastal.is_empty() {
            let limit = if card.terrains.iter().any(|t| t == "Coastal") { " (at most 2 per board for Coastal Lands: you pick)" } else { "" };
            out.push(format!("Town instead of Explorer in {}{limit} (Trading Port, Scotland L1)", names(coastal)));
        }
    }
    // France L1, Frontier Explorers: "After Invaders successfully Explore into
    // a land which had no Town/City, add 1 Explorer there."
    if a.has(FRANCE, 1) {
        let empty: Vec<&PublicLand> = lands.iter().filter_map(find).filter(|l| l.towns + l.cities == 0).collect();
        if !empty.is_empty() {
            out.push(format!("2 Explorers in {} (Frontier Explorers, France L1)", names(empty)));
        }
    }
    // France L6, Persistent Explorers: "After resolving an Explore Card, on
    // each board add 1 Explorer to a land without any."
    if a.has(FRANCE, 6) {
        out.push("Then +1 Explorer per board in a land without any (Persistent Explorers, France L6: you pick)".into());
    }
    // Habsburg Mining L6, The Empire Ascendant: "On boards with 3 or fewer
    // Blight, Add +1 Explorer in each land successfully explored. (Max. 2
    // lands per board per Explore Card.)"
    if a.has(MINING, 6) {
        let calm: BTreeSet<&str> = a
            .s
            .boards
            .iter()
            .filter(|b| board.iter().filter(|l| l.board == b.id).map(|l| l.blight).sum::<u32>() <= 3)
            .map(|b| b.id.as_str())
            .collect();
        let hit: Vec<&PublicLand> = lands.iter().filter_map(find).filter(|l| calm.contains(l.board.as_str())).collect();
        if !hit.is_empty() {
            out.push(format!("+1 Explorer in up to 2 of {} per board (The Empire Ascendant, Habsburg Mining L6)", names(hit)));
        }
    }
    // Blitz: "When Exploring, once per board, add an additional Explorer."
    if a.blitz() && !lands.is_empty() {
        out.push("+1 Explorer in one explored land per board (Blitz: you pick)".into());
    }
    out
}

// ------------------------------------------------------------------ loss clocks

/// Each adversary's additional loss condition as a count against its limit,
/// now and after this Invader Phase as forecast (`after`: the forecast board
/// after the Build). Counts kept on the adversary's panel come from the save.
pub fn loss_clocks(a: &Adv, after: &[PublicLand], ravages: &[RavageForecast], build_ravages: &[RavageForecast]) -> Vec<LossClock> {
    let s = a.s;
    let mut out = Vec::new();
    let max_by = |lands: &[PublicLand], f: &dyn Fn(&PublicLand) -> i64| -> (i64, Option<String>) {
        lands.iter().filter(|l| !l.is_ocean).map(|l| (f(l), l.id.clone())).max().map(|(n, id)| (n, Some(id))).unwrap_or((0, None))
    };
    let changed = |now: i64, then: i64| (then != now).then_some(then);
    for adv in &s.adversaries {
        let Some((title, text, _)) = crate::rules_register::loss_condition(&adv.id) else { continue };
        let mut c = LossClock { title, adversary: adv.title.clone(), note: text, trigger: "at".into(), ..Default::default() };
        match adv.id.as_str() {
            ENGLAND => {
                let tc = |l: &PublicLand| (l.towns + l.cities) as i64;
                let (now, at) = max_by(&s.lands, &tc);
                c.now = now;
                c.at = at;
                c.after = changed(now, max_by(after, &tc).0);
                c.limit = 7;
            }
            FRANCE => {
                let towns = |v: &[PublicLand]| v.iter().map(|l| l.towns as i64).sum::<i64>();
                c.now = towns(&s.lands);
                c.after = changed(c.now, towns(after));
                c.limit = 7 * a.players;
                c.trigger = "over".into();
                c.note = format!("Towns on the island against France's pool of {} (Invaders win if a Town can't be placed)", c.limit);
            }
            SCOTLAND => {
                let coastal = |v: &[PublicLand]| v.iter().filter(|l| l.coastal && l.cities > 0).count() as i64;
                c.now = coastal(&s.lands);
                c.after = changed(c.now, coastal(after));
                c.limit = 2 * s.boards.len().max(1) as i64;
                c.trigger = "over".into();
            }
            MINING => {
                // Checked at the end of the Fast Phase. After this Invader
                // Phase: the forecast board plus the Blight its Ravages (and
                // Ceaseless Mining's) add in each land (Habsburg game, Turn 3:
                // Mountain NE7 went from 7 to 8 and was lost on Turn 4).
                let load = |l: &PublicLand| (l.explorers + l.towns + l.cities + l.blight) as i64;
                let (now, at) = max_by(&s.lands, &load);
                let blighted = |l: &PublicLand| -> i64 {
                    ravages.iter().chain(build_ravages).filter(|r| !r.done && r.ledger.adds_blight && r.land == l.id).map(|r| 1 + r.ledger.extra_blight).sum()
                };
                let (then, then_at) = max_by(after, &|l: &PublicLand| load(l) + blighted(l));
                c.now = now;
                c.at = if then > now { then_at } else { at };
                c.after = changed(now, then);
                c.limit = 8;
                c.checked = "at the end of each Fast Phase".into();
                c.note = "Invaders + Blight in one land, checked at the end of the Fast Phase (after: before this turn's Explore and Mining Boom)".into();
            }
            RUSSIA => {
                let destroyed = adv.panel.get("Beast").copied().unwrap_or(0) as i64;
                let beasts: i64 = s.lands.iter().map(|l| l.tokens.get("Beast").copied().unwrap_or(0) as i64).sum();
                let lost = ravages
                    .iter()
                    .filter(|r| !r.done && r.ledger.adds_blight)
                    .filter(|r| s.lands.iter().any(|l| l.id == r.land && l.tokens.get("Beast").copied().unwrap_or(0) > 0))
                    .count() as i64;
                c.now = destroyed;
                c.after = changed(destroyed, destroyed + lost);
                c.limit = beasts - lost;
                c.trigger = "over".into();
                c.note = format!("Beasts destroyed by Russia's rules (from the adversary panel) against the {beasts} Beasts on the island{}", if lost > 0 { format!(", {} after this Ravage", beasts - lost) } else { String::new() });
            }
            LIVESTOCK => {
                let tracked = adv.panel.get("Blight").copied().unwrap_or(0) as i64;
                let adds: i64 = ravages
                    .iter()
                    .filter(|r| !r.done && r.ledger.adds_blight && r.ledger.damage_after_defend - r.ledger.land_health >= 6)
                    .map(|r| 1 + r.ledger.extra_blight + r.ledger.cascades as i64)
                    .sum();
                c.now = tracked;
                c.after = changed(tracked, tracked + adds);
                c.limit = a.players;
                c.trigger = "over".into();
                c.note = "Blight off the card in Ravages with 6+ overkill (the running count reads the adversary panel; unverified until a Habsburg game)".into();
            }
            _ => continue,
        }
        c.line = clock_line(&adv.id, &c);
        out.push(c);
    }
    out
}

fn clock_line(id: &str, c: &LossClock) -> String {
    let then = c.after.map(|a| format!(" → {a}")).unwrap_or_default();
    let at = c.at.as_deref().map(|l| format!(" in {l}")).unwrap_or_default();
    match id {
        ENGLAND => format!("{}{then} of 7 Town/City{at}", c.now),
        FRANCE => format!("{}{then} Towns placed of {}", c.now, c.limit),
        SCOTLAND => format!("{}{then} Coastal lands with a City; loss above {}", c.now, c.limit),
        MINING => format!("{}{then} of 8 Invaders + Blight{at}", c.now),
        RUSSIA => format!("{}{then} destroyed vs {} Beasts left; loss when more are destroyed", c.now, c.limit),
        LIVESTOCK => format!("{}{then} Blight from 6+ overkill; loss above {}", c.now, c.limit),
        _ => format!("{}{then} of {}", c.now, c.limit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{AdversaryRef, PublicBoard, PublicPiece};

    /// A snapshot with one adversary and lands `ids` (terrain from the name;
    /// `coastal` per land), all adjacent to the first.
    fn snap(adversary: &str, level: i64, ids: &[(&str, bool)]) -> PublicSnapshot {
        let lands = ids
            .iter()
            .enumerate()
            .map(|(i, (id, coastal))| PublicLand {
                id: id.to_string(),
                board: "A".into(),
                number: i as i64 + 1,
                terrain: id.trim_end_matches(char::is_numeric).into(),
                coastal: *coastal,
                ..Default::default()
            })
            .collect();
        PublicSnapshot {
            adversaries: vec![AdversaryRef { id: adversary.into(), title: adversary.into(), level, panel: BTreeMap::new() }],
            boards: vec![PublicBoard { id: "A".into(), lands: ids.iter().map(|(id, _)| id.to_string()).collect() }],
            adjacency: ids[1..].iter().map(|(id, _)| (ids[0].0.to_string(), id.to_string())).collect(),
            spirits: vec![Default::default()],
            lands,
            ..Default::default()
        }
    }

    fn put(l: &mut PublicLand, kind: &str, n: u32) {
        for _ in 0..n {
            let hp = match kind {
                "City" => 3,
                "Town" | "Dahan" => 2,
                _ => 1,
            };
            l.pieces.push(PublicPiece { kind: kind.into(), max_health: hp, damage: 0, strife: 0 });
            match kind {
                "Explorer" => l.explorers += 1,
                "Town" => l.towns += 1,
                "City" => l.cities += 1,
                "Dahan" => l.dahan += 1,
                _ => {}
            }
        }
    }

    fn card(terrain: &str) -> PublicInvaderCard {
        PublicInvaderCard { id: format!("{terrain}1"), stage: 1, terrains: vec![terrain.into()], ..Default::default() }
    }

    #[test]
    fn far_flung_herds_adds_two_beside_a_town() {
        let mut s = snap(LIVESTOCK, 6, &[("Jungle1", false), ("Sands2", false)]);
        put(&mut s.lands[0], "Explorer", 1);
        let nbrs = rules::neighbors(&s);
        let a = Adv::new(&s);
        assert!(land_effects(&a, &s.lands[0], &s.lands, &nbrs).is_empty());
        let mut board = s.lands.clone();
        put(&mut board[1], "Town", 1);
        let fx = land_effects(&a, &board[0], &board, &nbrs);
        assert_eq!(rules::ravage_ledger(&board[0], &fx).invader_damage, 3);
    }

    #[test]
    fn mining_avarice_and_disease() {
        let mut s = snap(MINING, 1, &[("Mountain1", false)]);
        put(&mut s.lands[0], "Explorer", 2);
        put(&mut s.lands[0], "Town", 1);
        s.lands[0].blight = 1;
        let a = Adv::new(&s);
        let mut ledger = rules::ravage_ledger(&s.lands[0], &[]);
        assert!(ledger.cascades);
        after_ledger(&a, &s.lands[0], &mut ledger, &[]);
        assert!(ledger.adds_blight && !ledger.cascades, "Avarice Rewarded upgrades instead of cascading");
        let mut diseased = s.lands[0].clone();
        diseased.tokens.insert("Disease".into(), 1);
        let mut ledger = rules::ravage_ledger(&diseased, &[]);
        after_ledger(&a, &diseased, &mut ledger, &[]);
        assert!(ledger.cancelled.is_some() && !ledger.adds_blight, "Disease stops a Ravage in a Mining land");
    }

    #[test]
    fn runoff_adds_ocean_blight_on_coastal_lands() {
        let mut s = snap(SCOTLAND, 5, &[("Wetland1", true), ("Sands2", false)]);
        put(&mut s.lands[0], "Town", 1);
        put(&mut s.lands[1], "Town", 1);
        let a = Adv::new(&s);
        for (i, extra) in [(0, 1), (1, 0)] {
            let mut ledger = rules::ravage_ledger(&s.lands[i], &[]);
            after_ledger(&a, &s.lands[i], &mut ledger, &[]);
            assert_eq!(ledger.extra_blight, extra);
        }
    }

    #[test]
    fn chart_the_coastline_builds_beside_a_city() {
        let s = snap(SCOTLAND, 3, &[("Wetland1", true), ("Jungle2", false)]);
        let nbrs = rules::neighbors(&s);
        let a = Adv::new(&s);
        assert!(builds_without_invaders(&a, &card("Wetland"), &s.lands[0], &s.lands, &nbrs).is_none());
        let mut board = s.lands.clone();
        put(&mut board[1], "City", 1);
        assert!(builds_without_invaders(&a, &card("Wetland"), &board[0], &board, &nbrs).is_some());
    }

    #[test]
    fn more_rural_than_urban_and_slave_labor() {
        let s = snap(LIVESTOCK, 2, &[("Mountain1", false)]);
        let nbrs = rules::neighbors(&s);
        let a = Adv::new(&s);
        let mut board = s.lands.clone();
        put(&mut board[0], "Town", 1);
        rules::add_built(&mut board[0], "City", 3);
        let (_, built) = after_build(&a, &mut board, 0, "City", &nbrs, 2);
        assert_eq!((built, board[0].towns, board[0].cities), (Some("Town"), 3, 0));

        let s = snap(FRANCE, 2, &[("Sands1", false)]);
        let a = Adv::new(&s);
        let mut board = s.lands.clone();
        put(&mut board[0], "Explorer", 3);
        rules::add_built(&mut board[0], "Town", 2);
        after_build(&a, &mut board, 0, "Town", &nbrs, 2);
        assert_eq!((board[0].explorers, board[0].towns), (1, 3));
    }

    #[test]
    fn loss_clocks_read_counts() {
        let mut s = snap(RUSSIA, 1, &[("Jungle1", false)]);
        s.adversaries[0].panel.insert("Beast".into(), 2);
        s.lands[0].tokens.insert("Beast".into(), 1);
        let c = &loss_clocks(&Adv::new(&s), &s.lands, &[], &[])[0];
        assert_eq!((c.now, c.limit, c.trigger.as_str()), (2, 1, "over"));

        let mut s = snap(MINING, 0, &[("Mountain1", false), ("Sands2", false)]);
        put(&mut s.lands[1], "Explorer", 4);
        put(&mut s.lands[1], "Town", 2);
        s.lands[1].blight = 1;
        let c = &loss_clocks(&Adv::new(&s), &s.lands, &[], &[])[0];
        assert_eq!((c.now, c.limit, c.at.as_deref()), (7, 8, Some("Sands2")));
    }

    #[test]
    fn land_rush_lists_lands_that_open_no_explore_first() {
        // Board: Jungle1 (Town) beside Sands2 and Wetland3; Wetland3 also
        // borders Mountain4, which nothing feeds. A Town on Wetland3 or on
        // Mountain4 itself opens Mountain4 to a Mountain card; one on Sands2
        // opens nothing.
        let mut s = snap(PRUSSIA, 0, &[("Jungle1", false), ("Sands2", false), ("Wetland3", false), ("Mountain4", false)]);
        s.adjacency.retain(|(_, b)| b != "Mountain4");
        s.adjacency.push(("Wetland3".into(), "Mountain4".into()));
        put(&mut s.lands[0], "Town", 1);
        let nbrs = rules::neighbors(&s);
        let a = Adv::new(&s);
        let mut mountain = card("Mountain");
        mountain.escalation = true;
        let cands = vec![ExploreCandidate { card: mountain, lands: vec![], blocked: vec!["Mountain4".into()], notes: vec![] }];
        let cancels = rules::BuildCancels::new(&[]);
        let e = escalation(&a, &s.lands, &cands, true, &[], &[], &cancels, &[], &nbrs, 1, &mut Vec::new()).unwrap();
        let order: Vec<&str> = e.builds.iter().map(|b| b.land.as_str()).collect();
        assert_eq!(order, ["Sands2", "Wetland3", "Mountain4"]);
        assert!(e.builds[0].notes[0].contains("no new"));
        assert!(e.builds[1].notes[0].contains("Mountain4"));
    }
}
