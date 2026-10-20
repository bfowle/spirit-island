//! The Invader phase. Base rules from the rulebook; adversary rules from the
//! level texts in the definitions (England, Brandenburg-Prussia, Sweden,
//! Russia). Where the rules leave a choice to the player (where a cascade
//! lands, which Invaders the Dahan hit, where an escalation Town goes) the
//! simulation takes the engine's default or marks the outcome with a note,
//! so the verifier can tell a rule gap from a choice.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use si_model::decks::{InvaderCard, InvaderTarget};
use si_model::island::{LandId, PieceKind, Terrain};
use si_record::Counts;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Land {
    pub id: LandId,
    pub terrain: Option<Terrain>,
    pub coastal: bool,
    pub adjacent: Vec<LandId>,
    pub c: Counts,
    pub defend: u32,
    /// Set by Spirit effects this turn (Strike Low, a skip).
    pub skip_ravage: bool,
    pub skip_build: bool,
    pub skip_explore: bool,
    /// Invaders that do not take part in the Ravage (Strife, Unnerving Pall).
    pub no_participation: u32,
    /// Damage taken off the Ravage total by a Spirit effect ("Invaders do -6 Damage on their Ravage").
    pub damage_minus: u32,
    /// Blight added here does not cascade this turn (Entrap the Forces of Corruption).
    pub no_cascade: bool,
    /// Extra Health on the Dahan here this turn (Promises of Protection).
    pub dahan_health: u32,
    /// Isolated by a Spirit effect this turn.
    pub isolated: bool,
    /// Blight would be added here but stays on the card this turn (Water Nourishes Life's Growth).
    pub no_blight: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Island {
    pub lands: BTreeMap<LandId, Land>,
}

impl Island {
    pub fn buildings_adjacent(&self, id: &LandId) -> u32 {
        self.lands.get(id).map(|l| l.adjacent.iter().filter_map(|a| self.lands.get(a)).map(|a| a.c.buildings()).sum()).unwrap_or(0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rules {
    pub adversary: Option<String>,
    pub level: u8,
    pub spirits: Vec<String>,
    /// Extra Explorers on an Explore into lands of the given engine target type
    /// ("LandWithoutTownCity", or "" for any) — The Frontier Calls.
    pub explore_bonus: Vec<(u32, String)>,
    /// Ravage damage adjustment per Invader of the listed kinds (empty: every Invader).
    pub damage_adjust: Vec<(i32, Vec<PieceKind>)>,
    /// Each Invader's damage changed by this much (Mesmerized Tranquility −1).
    pub piece_damage_mod: i32,
    pub dahan_health_bonus: BTreeMap<LandId, i32>,
    /// No City may be built in a Coastal land (Overseas Trade Seems Safer).
    pub no_coastal_city: bool,
    /// Disease is ignored during the Invader phase (Urban Development).
    pub ignore_disease: bool,
    pub isolated: Vec<LandId>,
    /// The spirits' special-rule hooks (spirit id, hook), from their panels.
    pub hooks: Vec<(String, si_model::spirit::RuleHook)>,
    /// Beasts count as Badlands (Hard-Working Settlers).
    pub beasts_as_badlands: bool,
}

impl Rules {
    fn is(&self, id: &str, min_level: u8) -> bool {
        self.adversary.as_deref() == Some(id) && self.level >= min_level
    }
    fn england(&self, l: u8) -> bool {
        self.is("TheKingdomOfEngland", l)
    }
    fn prussia(&self, l: u8) -> bool {
        self.is("TheKingdomOfBrandenburgPrussia", l)
    }
    fn sweden(&self, l: u8) -> bool {
        self.is("TheKingdomOfSweden", l)
    }
    fn russia(&self, l: u8) -> bool {
        self.is("TheTsardomOfRussia", l)
    }
    fn vengeance(&self) -> bool {
        self.hooks.iter().any(|(_, h)| matches!(h, si_model::spirit::RuleHook::DiseaseBuildForFear { .. }))
    }
    /// Extra Health on Towns and Cities (England 5).
    pub fn invader_health_bonus(&self) -> u32 {
        self.england(5) as u32
    }
    pub fn blight_as_badlands_for(&self, spirit: &str) -> bool {
        self.hooks.iter().any(|(s, h)| s == spirit && *h == si_model::spirit::RuleHook::BlightCountsAsBadlands)
    }

    /// The spirits' special-rule hooks read from their panels.
    pub fn hooks_of(defs: &si_defs::Definitions, spirits: &[String]) -> Vec<(String, si_model::spirit::RuleHook)> {
        spirits.iter().filter_map(|id| defs.spirits.get(id).map(|p| (id, p))).flat_map(|(id, p)| p.special_rules.iter().flat_map(|r| r.hooks.iter()).map(move |h| (id.clone(), h.clone()))).collect()
    }
}

/// What one step did to one land.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub land: LandId,
    pub added: Vec<PieceKind>,
    pub blight: u32,
    pub cascade: u32,
    /// Damage the Invaders dealt to the land (after Defend).
    pub damage: u32,
    pub dahan_lost: u32,
    pub invaders_destroyed: u32,
    pub presence_destroyed: u32,
    /// What a player's choice could have produced instead (Vengeance letting a
    /// Build through Disease).
    pub alternative: Vec<PieceKind>,
    /// Choices the rules leave to the player, or rules not modeled.
    pub notes: Vec<String>,
}

impl Outcome {
    fn new(land: &LandId) -> Outcome {
        Outcome { land: land.clone(), ..Default::default() }
    }
}

fn card_matches(card: &InvaderCard, land: &Land) -> bool {
    card.targets.iter().any(|t| match t {
        InvaderTarget::Terrain(tr) => land.terrain == Some(*tr),
        InvaderTarget::Coastal => land.coastal,
        InvaderTarget::Other(_) => false,
    })
}

fn matching(island: &Island, card: &InvaderCard) -> Vec<LandId> {
    island.lands.values().filter(|l| l.terrain != Some(Terrain::Ocean) && card_matches(card, l)).map(|l| l.id.clone()).collect()
}

/// Explore: an Explorer into each matching land that holds or touches a Town
/// or City, or is coastal. Wilds take the Explore instead. Stage II cards
/// carry the adversary's escalation.
pub fn explore(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let mut out = Vec::new();
    if card.escalation && rules.prussia(1) {
        out.push(Outcome { land: LandId::new("?", 0), notes: vec!["Land Rush: +1 Town in a land without Town on each board with Town/City (player's choice)".into()], ..Default::default() });
    }
    if card.escalation && rules.russia(1) {
        out.push(Outcome { land: LandId::new("?", 0), notes: vec!["Stalk the Predators: +2 Explorers among lands with Beasts (player's choice)".into()], ..Default::default() });
    }
    if card.escalation && rules.england(1) {
        // Building Boom: on each board with Town/City, build in the land with the most
        let mut by_board: BTreeMap<String, (u32, LandId)> = BTreeMap::new();
        for l in island.lands.values() {
            let b = l.c.buildings();
            if b > 0 {
                let e = by_board.entry(l.id.board.clone()).or_insert((0, l.id.clone()));
                if b > e.0 {
                    *e = (b, l.id.clone());
                }
            }
        }
        for (_, (_, id)) in by_board {
            let mut o = build_land(island, &id, rules);
            o.notes.push("Building Boom (escalation)".into());
            out.push(o);
        }
    }
    let targets = matching(island, card);
    for id in targets {
        let reached = {
            let l = &island.lands[&id];
            l.coastal || l.c.buildings() > 0 || island.buildings_adjacent(&id) > 0
        };
        if !reached {
            continue;
        }
        let isolated = rules.isolated.contains(&id);
        let l = island.lands.get_mut(&id).unwrap();
        let mut o = Outcome::new(&id);
        if l.skip_explore || isolated || l.isolated {
            o.notes.push(if isolated || l.isolated { "Isolated: no Explore into the land".into() } else { "Explore skipped by a Spirit effect".into() });
        } else if l.c.wilds > 0 {
            l.c.wilds -= 1;
            o.notes.push("Wilds removed instead of the Explore".into());
        } else {
            l.c.explorers += 1;
            o.added.push(PieceKind::Explorer);
            for (n, target) in &rules.explore_bonus {
                let applies = target.is_empty() || (target == "LandWithoutTownCity" && l.c.buildings() == 0);
                if applies {
                    l.c.explorers += n;
                    o.added.extend(std::iter::repeat(PieceKind::Explorer).take(*n as usize));
                    o.notes.push(format!("+{n} Explorer from an Event modifier"));
                }
            }
            if card.escalation && rules.sweden(1) && l.c.invaders() >= l.c.dahan && l.c.dahan > 0 {
                l.c.dahan -= 1;
                l.c.towns += 1;
                o.added.push(PieceKind::Town);
                o.notes.push("Swayed by the Invaders: 1 Dahan replaced with a Town".into());
            }
        }
        out.push(o);
    }
    // one outcome per land (an escalation Build and the Explore itself merge)
    let mut merged: Vec<Outcome> = Vec::new();
    for o in out {
        match merged.iter_mut().find(|m| m.land == o.land && m.land.board != "?") {
            Some(m) => {
                m.added.extend(o.added);
                m.notes.extend(o.notes);
            }
            None => merged.push(o),
        }
    }
    merged
}

pub fn build_land(island: &mut Island, id: &LandId, rules: &Rules) -> Outcome {
    let l = island.lands.get_mut(id).unwrap();
    let mut o = Outcome::new(id);
    let piece = if l.c.towns > l.c.cities { PieceKind::City } else { PieceKind::Town };
    if l.skip_build {
        o.notes.push("Build skipped by a Spirit effect".into());
    } else if l.c.disease > 0 && !rules.ignore_disease {
        l.c.disease -= 1;
        if rules.vengeance() {
            o.alternative = vec![piece];
            o.notes.push("Disease would stop the Build; Vengeance may let it happen for 1 Fear".into());
        } else {
            o.notes.push("Disease stopped the Build".into());
        }
    } else if piece == PieceKind::City && l.coastal && rules.no_coastal_city {
        o.notes.push("no City in a Coastal land this turn (Fear card)".into());
    } else {
        match piece {
            PieceKind::City => l.c.cities += 1,
            _ => l.c.towns += 1,
        }
        o.added.push(piece);
    }
    o
}

/// Build: in each matching land with Invaders (England 1: also lands without
/// Invaders next to two or more Towns and Cities), a City if Towns outnumber
/// Cities, else a Town. Disease stops it.
pub fn build(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let mut out = Vec::new();
    for id in matching(island, card) {
        let has_invaders = island.lands[&id].c.invaders() > 0;
        let england = rules.england(1) && !has_invaders && island.buildings_adjacent(&id) >= 2;
        if has_invaders || england {
            let mut o = build_land(island, &id, rules);
            if england {
                o.notes.push("Indentured Servants: Build into a land without Invaders".into());
            }
            out.push(o);
        }
    }
    out
}

/// Ravage: Invaders deal their damage to the land and the Dahan, Defend
/// absorbs first, two or more Damage adds Blight (cascading onto an adjacent
/// land when Blight was already there), surviving Dahan strike back.
pub fn ravage(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let mut targets = matching(island, card);
    if rules.russia(3) {
        for l in island.lands.values() {
            if l.c.explorers >= 3 && !targets.contains(&l.id) {
                targets.push(l.id.clone());
            }
        }
    }
    let mut out = Vec::new();
    for id in targets {
        if let Some(o) = ravage_land(island, &id, rules) {
            out.push(o);
        }
    }
    out
}

/// One land's Ravage: damage from its Invaders against Defend, Blight and
/// cascade, Dahan losses and their counterattack. `None` when no Invaders are
/// there. Also what a Power's forced Ravage runs.
pub fn ravage_land(island: &mut Island, id: &LandId, rules: &Rules) -> Option<Outcome> {
    let id = id.clone();
    if island.lands[&id].c.invaders() == 0 {
        return None;
    }
    let l = island.lands.get_mut(&id).unwrap();
    let mut o = Outcome::new(&id);
    if l.skip_ravage {
        o.notes.push("Ravage skipped by a Spirit effect".into());
        return Some(o);
    }
    let adjust = |kind: PieceKind, base: i32| -> u32 {
        let mut d = base + rules.piece_damage_mod;
        for (a, kinds) in &rules.damage_adjust {
            if kinds.is_empty() || kinds.contains(&kind) {
                d += a;
            }
        }
        d.max(0) as u32
    };
    let (ex, tn, ct) = (adjust(PieceKind::Explorer, 1 + rules.russia(1) as i32), adjust(PieceKind::Town, if rules.sweden(3) { 3 } else { 2 }), adjust(PieceKind::City, if rules.sweden(3) { 5 } else { 3 }));
    let mut parts: Vec<u32> = Vec::new();
    parts.extend(std::iter::repeat(ct).take(l.c.cities as usize));
    parts.extend(std::iter::repeat(tn).take(l.c.towns as usize));
    parts.extend(std::iter::repeat(ex).take(l.c.explorers as usize));
    if l.no_participation > 0 {
        // which Invaders sit out is the player's choice; the largest first
        let n = (l.no_participation as usize).min(parts.len());
        parts.drain(..n);
        o.notes.push(format!("{} Invaders did not take part (player's choice of which)", l.no_participation));
    }
    let raw: u32 = parts.iter().sum();
    let dmg = raw.saturating_sub(l.defend).saturating_sub(l.damage_minus);
    o.damage = dmg;
    if dmg >= 2 && l.no_blight {
        o.notes.push("Blight not added: a Spirit effect keeps it on the card".into());
    }
    if dmg >= 2 && !l.no_blight {
        o.blight = 1;
        if rules.sweden(1) && dmg >= 6 {
            o.blight += 1;
            o.notes.push("Heavy Mining: 4+ overkill adds a Blight".into());
        }
        if l.c.blight > 0 && !l.no_cascade {
            o.cascade = 1;
            o.notes.push("cascade onto an adjacent land (player's choice)".into());
        }
        if l.c.presence > 0 {
            o.presence_destroyed = 1;
            o.notes.push("1 presence per Spirit present destroyed (one Spirit assumed)".into());
        }
        if rules.russia(1) && l.c.beasts > 0 {
            l.c.beasts -= 1;
            o.notes.push("Hunters: 1 Beast destroyed with the Blight".into());
        }
        if rules.sweden(5) {
            o.notes.push("Mining Rush: +1 Town in an adjacent land without Town/City (player's choice)".into());
        }
    }
    // Dahan take the same damage; Badlands add to it
    let badlands = l.c.badlands + if rules.beasts_as_badlands { l.c.beasts } else { 0 };
    let to_dahan = if dmg > 0 { dmg + badlands } else { 0 };
    let dahan_health = (2 + rules.dahan_health_bonus.get(&id).copied().unwrap_or(0) + l.dahan_health as i32).max(1) as u32;
    o.dahan_lost = l.c.dahan.min(to_dahan / dahan_health);
    let survivors = l.c.dahan - o.dahan_lost;
    let counter = if survivors > 0 { survivors * 2 + badlands } else { 0 };
    // the player distributes: cheapest kills first (England 5: +1 Health)
    let bonus = rules.england(5) as u32;
    let mut health: Vec<(u32, PieceKind)> = Vec::new();
    health.extend(std::iter::repeat((1u32, PieceKind::Explorer)).take(l.c.explorers as usize));
    health.extend(std::iter::repeat((2 + bonus, PieceKind::Town)).take(l.c.towns as usize));
    health.extend(std::iter::repeat((3 + bonus, PieceKind::City)).take(l.c.cities as usize));
    let mut left = counter;
    for (h, k) in &health {
        if left >= *h {
            left -= *h;
            o.invaders_destroyed += 1;
            match k {
                PieceKind::Explorer => l.c.explorers -= 1,
                PieceKind::Town => l.c.towns -= 1,
                PieceKind::City => l.c.cities -= 1,
                PieceKind::Dahan => {}
            }
        }
    }
    if counter > 0 {
        o.notes.push("counterattack distribution: cheapest Invaders first (player's choice)".into());
    }
    l.c.blight += o.blight;
    l.c.dahan -= o.dahan_lost;
    l.c.presence = l.c.presence.saturating_sub(o.presence_destroyed);
    Some(o)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn land(id: &str, terrain: Terrain, coastal: bool, adj: &[&str], c: Counts) -> Land {
        Land { id: id.parse().unwrap(), terrain: Some(terrain), coastal, adjacent: adj.iter().map(|a| a.parse().unwrap()).collect(), c, ..Default::default() }
    }

    fn island(lands: Vec<Land>) -> Island {
        Island { lands: lands.into_iter().map(|l| (l.id.clone(), l)).collect() }
    }

    fn card(t: Terrain) -> InvaderCard {
        InvaderCard { id: format!("{}1", t.name()), stage: 1, targets: vec![InvaderTarget::Terrain(t)], escalation: false }
    }

    #[test]
    fn ravage_blights_at_two_damage_and_dahan_strike_back() {
        let mut isl = island(vec![land("A1", Terrain::Sands, true, &[], Counts { explorers: 1, towns: 1, dahan: 2, ..Default::default() })]);
        let out = ravage(&mut isl, &card(Terrain::Sands), &Rules::default());
        let o = &out[0];
        assert_eq!((o.damage, o.blight, o.cascade), (3, 1, 0));
        assert_eq!(o.dahan_lost, 1, "3 damage kills one Dahan of two");
        assert_eq!(o.invaders_destroyed, 1, "the survivor's 2 damage kills the Explorer (1) with 1 left, not the Town (2)");
        let l = &isl.lands[&"A1".parse().unwrap()];
        assert_eq!((l.c.explorers, l.c.towns, l.c.dahan, l.c.blight), (0, 1, 1, 1));
    }

    #[test]
    fn defend_and_skips_stop_the_blight() {
        let mut isl = island(vec![land("A1", Terrain::Sands, false, &[], Counts { towns: 1, ..Default::default() })]);
        isl.lands.get_mut(&"A1".parse().unwrap()).unwrap().defend = 1;
        let out = ravage(&mut isl, &card(Terrain::Sands), &Rules::default());
        assert_eq!((out[0].damage, out[0].blight), (1, 0));
        isl.lands.get_mut(&"A1".parse().unwrap()).unwrap().skip_ravage = true;
        let out = ravage(&mut isl, &card(Terrain::Sands), &Rules::default());
        assert_eq!(out[0].damage, 0);
    }

    #[test]
    fn build_prefers_a_city_over_towns_and_disease_blocks() {
        let mut isl = island(vec![land("A1", Terrain::Jungle, false, &[], Counts { explorers: 1, towns: 1, ..Default::default() }), land("A2", Terrain::Jungle, false, &[], Counts { explorers: 1, disease: 1, ..Default::default() })]);
        let out = build(&mut isl, &card(Terrain::Jungle), &Rules::default());
        assert_eq!(out[0].added, vec![PieceKind::City]);
        assert!(out[1].added.is_empty() && isl.lands[&"A2".parse().unwrap()].c.disease == 0);
    }

    #[test]
    fn explore_needs_a_source_and_wilds_take_it() {
        let mut isl = island(vec![
            land("A1", Terrain::Wetland, true, &["A2"], Counts::default()),
            land("A2", Terrain::Wetland, false, &["A1", "A3"], Counts::default()),
            land("A3", Terrain::Wetland, false, &["A2"], Counts { towns: 1, ..Default::default() }),
            land("A4", Terrain::Wetland, true, &[], Counts { wilds: 1, ..Default::default() }),
        ]);
        let out = explore(&mut isl, &card(Terrain::Wetland), &Rules::default());
        let lands: Vec<String> = out.iter().map(|o| o.land.to_string()).collect();
        assert_eq!(lands, vec!["A1", "A2", "A3", "A4"], "A1 coastal, A2 touches the Town, A3 holds it, A4 coastal");
        assert!(out[3].added.is_empty(), "Wilds took the Explore");
    }

    #[test]
    fn england_builds_into_empty_lands_next_to_two_buildings() {
        let mut isl = island(vec![land("A1", Terrain::Sands, false, &["A2"], Counts::default()), land("A2", Terrain::Jungle, false, &["A1"], Counts { towns: 1, cities: 1, ..Default::default() })]);
        let rules = Rules { adversary: Some("TheKingdomOfEngland".into()), level: 1, ..Default::default() };
        let out = build(&mut isl, &card(Terrain::Sands), &rules);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].added, vec![PieceKind::Town]);
    }
}
