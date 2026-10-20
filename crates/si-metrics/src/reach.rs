//! `reach`: could a Spirit's hand have acted on a land at a clock position?
//! Section 2.0 of the tempo spec. For each chain and each of its two windows
//! (before the Build: Slow on t0 or Fast on t0+1; before the Ravage: Slow on
//! t0+1 or Fast on t0+2) every card a spirit held that turn is checked for
//! speed, Energy, range from its presence, the card's target rule against the
//! land as it stood, and whether its printed effect acts on that step.
//!
//! Range is measured on the board graph the definitions give; between boards
//! the layout is not logged, so a card whose nearest presence sits on another
//! board gets `in_range: None` and never decides a class on its own.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use si_defs::Definitions;
use si_model::island::{Island, LandId, Terrain};
use si_model::power::{Effect, PowerCard, Slot, Speed, TargetKind};
use si_record::{Counts, GameRecord};

use crate::tempo::Chain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReachCard {
    pub spirit: String,
    pub card: String,
    /// The turn it could have been played in this window.
    pub turn: u32,
    pub speed: Speed,
    pub cost: i64,
    pub payable: bool,
    /// None: the nearest presence is on another board (layout unknown).
    pub in_range: Option<bool>,
    pub target_ok: bool,
    /// Plays the spirit left unused that turn (None when plays are unknown).
    pub plays_free: Option<i64>,
}

impl ReachCard {
    /// Everything checks out, range included.
    pub fn sure(&self) -> bool {
        self.payable && self.in_range == Some(true) && self.target_ok && self.plays_free.is_none_or(|p| p > 0)
    }

    /// Nothing rules it out, but range could not be measured.
    pub fn possible(&self) -> bool {
        self.payable && self.in_range.is_none() && self.target_ok && self.plays_free.is_none_or(|p| p > 0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainReach {
    /// Cards that could have acted before the Build.
    pub build: Vec<ReachCard>,
    /// Cards that could have acted before the Ravage.
    pub ravage: Vec<ReachCard>,
}

impl ChainReach {
    pub fn could_build(&self) -> bool {
        self.build.iter().any(ReachCard::sure)
    }

    pub fn could_ravage(&self) -> bool {
        self.ravage.iter().any(ReachCard::sure)
    }

    /// Some card might have reached the Ravage slot, range unknown.
    pub fn ravage_uncertain(&self) -> bool {
        !self.could_ravage() && self.ravage.iter().any(ReachCard::possible)
    }

    pub fn first_ravage(&self) -> Option<&ReachCard> {
        self.ravage.iter().find(|c| c.sure()).or_else(|| self.ravage.iter().find(|c| c.possible()))
    }
}

/// The island graph for the record's boards (lands the definitions know).
pub fn island_for(rec: &GameRecord, defs: &Definitions) -> Island {
    let boards: Vec<_> = rec.meta.boards.iter().filter_map(|b| defs.board_def(b).cloned()).collect();
    Island::from_boards(&boards)
}

/// The land's pieces as they stood after the last Invader step up to `turn`.
pub fn state_after(rec: &GameRecord, land: &LandId, turn: u32) -> Counts {
    rec.lands.get(land).and_then(|l| l.visits.iter().filter(|v| v.turn <= turn).last().map(|v| v.after)).or_else(|| rec.setup.get(land).copied()).unwrap_or_default()
}

pub fn speed_fits(c: &PowerCard, fast_window: bool) -> bool {
    match (c.speed, fast_window) {
        (Speed::Fast, true) | (Speed::Slow, false) => true,
        (Speed::Slow, true) => c.all_effects().any(|e| matches!(e, Effect::MayBeFast { .. })),
        (Speed::Fast, false) => false,
    }
}

pub fn range_ok(c: &PowerCard, presence: &BTreeMap<LandId, u32>, land: &LandId, island: &Island) -> Option<bool> {
    let Some(r) = &c.range else {
        // a Spirit-targeting card works in the Spirit's own lands
        return Some(presence.get(land).is_some_and(|n| *n > 0));
    };
    let mut unknown = false;
    for (origin, n) in presence {
        if r.from_sacred_site && *n < 2 {
            continue;
        }
        if !r.origin_terrains.is_empty() {
            match island.lands.get(origin) {
                Some(l) if r.origin_terrains.contains(&l.terrain) => {}
                _ => continue,
            }
        }
        match island.distance(origin, land, false) {
            Some(d) if d <= r.distance as u32 => return Some(true),
            Some(_) => {}
            None => unknown = true,
        }
    }
    if unknown { None } else { Some(false) }
}

pub fn target_ok(t: &TargetKind, st: &Counts, terrain: Option<Terrain>, coastal: Option<bool>) -> bool {
    if t.is_spirit() {
        return true;
    }
    match t.0.as_str() {
        "LandWithTwoDahan" => return st.dahan >= 2,
        "LandWithTownCity" => return st.buildings() > 0,
        "LandWithCity" => return st.cities > 0,
        "CoastalLandWithCity" => return st.cities > 0 && coastal != Some(false),
        "LandWithBlight" => return st.blight > 0,
        "LandWithBlightAndInvaders" => return st.blight > 0 && st.invaders() > 0,
        "LandWithNoInvaders" => return st.invaders() == 0,
        "JungleOrNoBlight" => return terrain == Some(Terrain::Jungle) || st.blight == 0,
        "LandWithDisease" => return st.disease > 0,
        "LandWithStrife" => return st.strife > 0,
        "LandWithBeast" => return st.beasts > 0,
        "LandWithTwoBeast" => return st.beasts >= 2,
        "LandWithWilds" => return st.wilds > 0,
        _ => {}
    }
    if t.requires_dahan() && st.dahan == 0 {
        return false;
    }
    if t.requires_invaders() && st.invaders() == 0 {
        return false;
    }
    if t.requires_no_blight() && st.blight > 0 {
        return false;
    }
    if t.coastal_only() && coastal == Some(false) {
        return false;
    }
    if t.inland_only() && coastal == Some(true) {
        return false;
    }
    if let (Some(allowed), Some(terrain)) = (t.terrains(), terrain) {
        if !allowed.contains(&terrain) {
            return false;
        }
    }
    true
}

fn window(rec: &GameRecord, defs: &Definitions, island: &Island, land: &LandId, slot: Slot, slow_turn: u32, fast_turn: u32, state: &Counts) -> Vec<ReachCard> {
    let land_def = island.lands.get(land);
    let (terrain, coastal) = (land_def.map(|l| l.terrain), land_def.map(|l| l.coastal));
    let mut out = Vec::new();
    for s in rec.spirits.iter().filter(|s| s.turn >= 1 && (s.turn == slow_turn || s.turn == fast_turn)) {
        let fast = s.turn == fast_turn;
        let energy = s.energy_after + s.energy_paid;
        let plays_free = s.plays.map(|p| p - s.cards_played.len() as i64);
        let mut cards: Vec<&String> = s.hand_after.iter().chain(s.cards_played.iter()).collect();
        cards.sort();
        cards.dedup();
        for id in cards {
            let Some(c) = defs.cards.get(id) else { continue };
            if !c.acts_on(slot) || !speed_fits(c, fast) {
                continue;
            }
            out.push(ReachCard {
                spirit: s.spirit.clone(),
                card: c.id.clone(),
                turn: s.turn,
                speed: c.speed,
                cost: c.cost,
                payable: c.cost <= energy,
                in_range: range_ok(c, &s.presence_after, land, island),
                target_ok: target_ok(&c.target, state, terrain, coastal),
                plays_free,
            });
        }
    }
    out
}

/// Both windows of one chain.
pub fn chain_reach(rec: &GameRecord, defs: &Definitions, island: &Island, chain: &Chain) -> ChainReach {
    // the steps' real turns: a scenario or adversary can Build or Ravage off
    // the usual clock, and the windows close before the step itself
    let build_turn = chain.build.unwrap_or(chain.t0 + 1);
    let ravage_turn = chain.ravage_turn();
    let before_build = state_after(rec, &chain.land, build_turn.saturating_sub(1));
    let before_ravage = state_after(rec, &chain.land, ravage_turn.saturating_sub(1));
    ChainReach {
        build: window(rec, defs, island, &chain.land, Slot::Build, build_turn.saturating_sub(1), build_turn, &before_build),
        ravage: window(rec, defs, island, &chain.land, Slot::Ravage, ravage_turn.saturating_sub(1), ravage_turn, &before_ravage),
    }
}

/// What the spirit's own Powers at this land cost over the chain: cards
/// played (each card once per turn) and their Energy.
pub fn price(chain: &Chain, defs: &Definitions) -> (u32, i64) {
    let mut seen: Vec<(u32, &str)> = Vec::new();
    let (mut cards, mut energy) = (0u32, 0i64);
    for t in &chain.touches {
        if !t.touch.by_spirit() || !matches!(t.touch.kind, si_record::TouchKind::Power) {
            continue;
        }
        let Some(c) = defs.cards.get(&t.touch.by) else { continue };
        if seen.contains(&(t.touch.turn, c.id.as_str())) {
            continue;
        }
        seen.push((t.touch.turn, c.id.as_str()));
        cards += 1;
        energy += c.cost;
    }
    (cards, energy)
}
