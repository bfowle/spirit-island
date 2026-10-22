//! The Invader phase. Base rules from the rulebook; adversary rules from the
//! level texts in the definitions (England, Brandenburg-Prussia, Sweden,
//! Russia). Where the rules leave a choice to the player (where a cascade
//! lands, which Invaders the Dahan hit, where an escalation Town goes) the
//! simulation takes the engine's default or marks the outcome with a note,
//! so the verifier can tell a rule gap from a choice.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use si_model::decks::{InvaderCard, InvaderTarget};
use si_model::event::InvaderStep;
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
    /// The Dahan here deal their Ravage damage with the Invaders', the ones destroyed
    /// included (Words of Warning).
    pub dahan_simultaneous: bool,
    /// Isolated by a Spirit effect this turn.
    pub isolated: bool,
    /// Blight would be added here but stays on the card this turn (Water Nourishes Life's Growth).
    pub no_blight: bool,
    /// Ravage damage adjustments that apply to this land only (a Fear card's chosen land): (amount, Invader kinds; empty = all).
    pub damage_adjust: Vec<(i32, Vec<PieceKind>)>,
    /// Damage that stays on a piece until Time Passes: one entry a damaged piece (its kind,
    /// the Damage on it). A Town with 1 Damage falls to 1 more.
    #[serde(default)]
    pub wounds: Vec<(PieceKind, u32)>,
}

/// The value of destroying a piece, for choosing which the Damage takes (a Town worth three
/// Explorers, a City five).
fn kill_value(k: PieceKind) -> u32 {
    match k {
        PieceKind::Explorer => 1,
        PieceKind::Town => 3,
        PieceKind::City => 5,
        PieceKind::Dahan => 0,
    }
}

impl Land {
    fn count_mut(&mut self, k: PieceKind) -> &mut u32 {
        match k {
            PieceKind::Explorer => &mut self.c.explorers,
            PieceKind::Town => &mut self.c.towns,
            PieceKind::City => &mut self.c.cities,
            PieceKind::Dahan => &mut self.c.dahan,
        }
    }

    /// Every piece of the kinds, with the Health it has left (`full` is a kind's Health with
    /// its bonuses): the damaged ones first, most damaged first.
    pub fn pieces_left(&self, kinds: &[PieceKind], full: &dyn Fn(PieceKind) -> u32) -> Vec<(PieceKind, u32)> {
        let mut out = Vec::new();
        for &k in kinds {
            let n = match k {
                PieceKind::Explorer => self.c.explorers,
                PieceKind::Town => self.c.towns,
                PieceKind::City => self.c.cities,
                PieceKind::Dahan => self.c.dahan,
            } as usize;
            let mut hurt: Vec<u32> = self.wounds.iter().filter(|(w, _)| *w == k).map(|(_, d)| *d).collect();
            hurt.sort_by(|a, b| b.cmp(a));
            for i in 0..n {
                let d = hurt.get(i).copied().unwrap_or(0);
                out.push((k, full(k).saturating_sub(d).max(1)));
            }
        }
        out
    }

    /// One piece of `kind` with `left` Health destroyed (its wound goes with it).
    fn destroy_piece(&mut self, kind: PieceKind, left: u32, full: u32) {
        let n = self.count_mut(kind);
        if *n == 0 {
            return;
        }
        *n -= 1;
        let dmg = full.saturating_sub(left);
        if dmg > 0 {
            if let Some(i) = self.wounds.iter().position(|(k, d)| *k == kind && *d == dmg) {
                self.wounds.remove(i);
            }
        }
        self.trim_wounds();
    }

    /// `dmg` more Damage on one piece of `kind` that has `left` Health: it stays, wounded.
    fn wound_piece(&mut self, kind: PieceKind, left: u32, full: u32, dmg: u32) {
        let had = full.saturating_sub(left);
        if had > 0 {
            if let Some(w) = self.wounds.iter_mut().find(|(k, d)| *k == kind && *d == had) {
                w.1 += dmg;
                return;
            }
        }
        self.wounds.push((kind, dmg));
    }

    /// No more wounds of a kind than pieces of it: a piece that left took its wound along.
    pub fn trim_wounds(&mut self) {
        for k in [PieceKind::Explorer, PieceKind::Town, PieceKind::City, PieceKind::Dahan] {
            let n = match k {
                PieceKind::Explorer => self.c.explorers,
                PieceKind::Town => self.c.towns,
                PieceKind::City => self.c.cities,
                PieceKind::Dahan => self.c.dahan,
            } as usize;
            let mut idx: Vec<usize> = self.wounds.iter().enumerate().filter(|(_, (w, _))| *w == k).map(|(i, _)| i).collect();
            // drop the least damaged first
            idx.sort_by_key(|i| self.wounds[*i].1);
            let extra = idx.len().saturating_sub(n);
            let mut drop: Vec<usize> = idx.into_iter().take(extra).collect();
            drop.sort_by(|a, b| b.cmp(a));
            for i in drop {
                self.wounds.remove(i);
            }
        }
    }

    /// `total` Damage to the Dahan, as the rules make it fall: "You must Destroy Dahan as
    /// efficiently as possible; you cannot disperse Damage among multiple Dahan to avoid
    /// killing them. If a Dahan is dealt 1 Damage, turn it over to show that it is Damaged"
    /// (core rulebook p. 9). The damaged ones fall first; what is left stays on one. How many
    /// were destroyed.
    pub fn take_dahan_damage(&mut self, total: u32, full: u32) -> u32 {
        let mut left = total;
        let mut killed = 0;
        loop {
            let pieces = self.pieces_left(&[PieceKind::Dahan], &|_| full);
            let Some(&(_, h)) = pieces.iter().min_by_key(|(_, h)| *h) else { break };
            if h > left {
                if left > 0 {
                    self.wound_piece(PieceKind::Dahan, h, full, left);
                }
                break;
            }
            self.destroy_piece(PieceKind::Dahan, h, full);
            left -= h;
            killed += 1;
        }
        killed
    }

    /// `each` Damage to every piece of `kinds` (a "to each" effect): the ones it reaches
    /// destroyed, the rest wounded. The pieces destroyed, by kind.
    pub fn take_each_damage(&mut self, each: u32, kinds: &[PieceKind], full: &dyn Fn(PieceKind) -> u32) -> Vec<PieceKind> {
        let mut killed = Vec::new();
        if each == 0 {
            return killed;
        }
        for (k, h) in self.pieces_left(kinds, full) {
            if h <= each {
                self.destroy_piece(k, h, full(k));
                killed.push(k);
            } else {
                self.wound_piece(k, h, full(k), each);
            }
        }
        killed
    }

    /// `total` Damage to the pieces of `kinds`, spent as the player would: the most valuable
    /// set it destroys (each piece at the Health it has left), the pieces in `first` before
    /// any (the line's stated kills; with `only`, nothing else dies), the rest on a survivor
    /// where it kills nothing and stays until Time Passes. The pieces destroyed, by kind.
    pub fn take_damage(&mut self, total: u32, kinds: &[PieceKind], full: &dyn Fn(PieceKind) -> u32, first: &[PieceKind], only: bool) -> Vec<PieceKind> {
        let mut left = total;
        let mut killed: Vec<PieceKind> = Vec::new();
        for &k in first.iter().filter(|k| kinds.contains(k)) {
            let pieces = self.pieces_left(&[k], full);
            let Some(&(_, h)) = pieces.iter().filter(|(_, h)| *h <= left).min_by_key(|(_, h)| *h) else { continue };
            self.destroy_piece(k, h, full(k));
            left -= h;
            killed.push(k);
        }
        if !only {
            // the most valuable set of the rest the Damage can destroy (a knapsack on Health)
            let pieces = self.pieces_left(kinds, full);
            let n = pieces.len();
            let cap = left as usize;
            // ties go to the bigger pieces: three Cities, a Town and an Explorer over two
            // Cities and three Towns for 12 (as Thunderspeaker's player took them)
            let worth = |k: PieceKind| kill_value(k) * 1000 + match k {
                PieceKind::City => 100,
                PieceKind::Town => 10,
                _ => 1,
            };
            let mut best = vec![vec![0u32; cap + 1]; n + 1];
            for i in 0..n {
                let (k, h) = pieces[i];
                for c in 0..=cap {
                    best[i + 1][c] = best[i][c];
                    if (h as usize) <= c {
                        let v = best[i][c - h as usize] + worth(k);
                        if v > best[i + 1][c] {
                            best[i + 1][c] = v;
                        }
                    }
                }
            }
            let mut c = cap;
            let mut take: Vec<(PieceKind, u32)> = Vec::new();
            for i in (0..n).rev() {
                if best[i + 1][c] != best[i][c] {
                    let (k, h) = pieces[i];
                    take.push((k, h));
                    c -= h as usize;
                }
            }
            for (k, h) in take {
                self.destroy_piece(k, h, full(k));
                left -= h;
                killed.push(k);
            }
        }
        // what is left wounds the most valuable survivor it does not destroy
        if left > 0 {
            let survivors = self.pieces_left(kinds, full);
            if let Some(&(k, h)) = survivors.iter().filter(|(_, h)| *h > left).max_by_key(|(k, h)| (kill_value(*k), u32::MAX - *h)) {
                self.wound_piece(k, h, full(k), left);
            }
        }
        killed
    }
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
    /// The loss conditions' game-long counts as the turn began.
    #[serde(default)]
    pub clocks: LossClocks,
    /// Where the player sent each cascading Blight this turn, as the record has it: the
    /// adjacent lands offered and the one chosen.
    #[serde(default)]
    pub cascades: Vec<(Vec<LandId>, LandId)>,
    /// The order the player ravaged the matching lands in this turn, as the record has it.
    #[serde(default)]
    pub ravage_order: Vec<LandId>,
    /// The Terror Level in effect, for the victory it sets (no Invaders at I, no Towns or
    /// Cities at II, no Cities at III); 0 checks no victory.
    #[serde(default)]
    pub terror: u8,
    /// The scenario in play (Blitz: an extra Explorer once per board when exploring).
    #[serde(default)]
    pub scenario: Option<String>,
    /// The Ravages and Builds the player skipped this turn by destroying a presence in a land
    /// with their Sacred Site (Rampant Green: Choke the Land with Green), as the record has
    /// them; which ones to skip is the player's call.
    #[serde(default)]
    pub chokes: Vec<(InvaderStep, LandId)>,
}

/// The game-long counts the loss conditions keep, as they stood when the turn began
/// (the record's earlier turns, the save's Blight Card).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LossClocks {
    /// Players in the game (the Habsburg Monarchy's limit); 0 reads as 1.
    pub players: u32,
    /// Blight left on the Blight Card and whether the card has flipped; None when unknown.
    pub blight_pool: Option<(i64, bool)>,
    /// The Tsardom of Russia: Beasts destroyed by adversary rules this game.
    pub beasts_destroyed: u32,
    /// The Habsburg Monarchy: Blight off the card in Ravages with 6+ overkill this game.
    pub irreparable: u32,
}

/// What the turn has added so far to the game-long counts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tally {
    /// Blight taken from the Blight Card this turn.
    pub blight: u32,
    pub beasts_destroyed: u32,
    pub irreparable: u32,
}

fn boards_in(island: &Island) -> usize {
    island.lands.keys().map(|l| l.board.clone()).collect::<std::collections::BTreeSet<_>>().len().max(1)
}

/// A loss condition the board and the game-long counts show, as the reason the Invaders win:
/// the core ones (the Spirit's last presence gone, a flipped Blight Card emptied) and every
/// adversary's own (England's capital, France's Town supply, Scotland's ports, Russia's
/// hunters, the Habsburg Monarchy's irreparable damage). The adversaries' conditions hold at
/// every level. None while the game goes on.
pub fn loss(island: &Island, rules: &Rules, tally: Tally) -> Option<String> {
    if let Some((pool, true)) = rules.clocks.blight_pool {
        if pool > 0 && tally.blight as i64 >= pool {
            return Some(format!("the Blight Card ran out: {pool} left on it, {} taken", tally.blight));
        }
    }
    let presence: u32 = island.lands.values().map(|l| l.c.presence).sum();
    if presence == 0 && !island.lands.is_empty() {
        return Some("the Spirit has no presence left on the island".into());
    }
    let boards = boards_in(island);
    match rules.adversary.as_deref() {
        Some("TheKingdomOfEngland") => island.lands.values().find(|l| l.c.buildings() >= 7).map(|l| format!("England's Proud & Mighty Capital: {} Town/City in {}", l.c.buildings(), l.id)),
        Some("TheKingdomOfScotland") => {
            let n = island.lands.values().filter(|l| l.coastal && l.c.cities > 0).count();
            (n > 2 * boards).then(|| format!("Scotland's Trade Hub: {n} Coastal lands with a City, more than {}", 2 * boards))
        }
        Some("TheKingdomOfFrance") => {
            let towns: u32 = island.lands.values().map(|l| l.c.towns).sum();
            (towns as usize > 7 * boards).then(|| format!("France's Sprawling Plantations: a Town with none left to place ({} per board)", 7))
        }
        Some("TheTsardomOfRussia") => {
            let destroyed = rules.clocks.beasts_destroyed + tally.beasts_destroyed;
            let beasts: u32 = island.lands.values().map(|l| l.c.beasts).sum();
            (destroyed > beasts).then(|| format!("Russia's Hunters Swarm the Island: {destroyed} Beasts destroyed by the Hunters, {beasts} on the island"))
        }
        Some("TheHabsburgMonarchy") => {
            let n = rules.clocks.irreparable + tally.irreparable;
            let players = rules.clocks.players.max(1);
            (n > players).then(|| format!("the Habsburg Monarchy's Irreparable Damage: {n} Blight off the card in Ravages with 6+ overkill, more than {players}"))
        }
        _ => None,
    }
}

/// The loss conditions checked at the end of the Fast phase: Habsburg Mining Expedition's
/// Land Stripped Bare (8 Invaders and Blight in one land), and every condition `loss` checks.
pub fn loss_after_fast(island: &Island, rules: &Rules, tally: Tally) -> Option<String> {
    if rules.adversary.as_deref() == Some("HabsburgMiningExpedition") {
        if let Some(l) = island.lands.values().find(|l| l.c.invaders() + l.c.blight >= 8) {
            return Some(format!("Habsburg Mining's Land Stripped Bare: {} Invaders and Blight in {}", l.c.invaders() + l.c.blight, l.id));
        }
    }
    loss(island, rules, tally)
}

/// How the game stands after an effect: `Ok` the Spirits win, `Err` the Invaders do, None
/// it goes on. An effect that brings both is a win: "In the rare case that a single Power
/// Card or other effect causes you to both win and lose, you win a sacrifice victory: you
/// are Destroyed, but the Island, the Dahan, and many other Spirits survive" (core
/// rulebook, the Victory sidebar). `after_fast` adds the losses checked as the Fast phase
/// ends (Habsburg Mining's Land Stripped Bare).
pub fn ending(island: &Island, rules: &Rules, tally: Tally, after_fast: bool) -> Option<Result<String, String>> {
    if let Some(w) = victory(island, rules) {
        return Some(Ok(w));
    }
    let lost = if after_fast { loss_after_fast(island, rules, tally) } else { loss(island, rules, tally) };
    lost.map(Err)
}

/// How near each loss condition in play stands, in words: the count against its limit.
/// The forward view a player keeps (England's capital two buildings short, France's Town
/// supply, Russia's hunters against the Beasts left, the Blight Card's pool).
pub fn clocks(island: &Island, rules: &Rules, tally: Tally) -> Vec<String> {
    let mut out = Vec::new();
    let boards = boards_in(island);
    match rules.adversary.as_deref() {
        Some("TheKingdomOfEngland") => {
            if let Some(l) = island.lands.values().filter(|l| l.c.buildings() > 0).max_by_key(|l| l.c.buildings()) {
                out.push(format!("England's capital: {} has {} of 7 Town/City", l.id, l.c.buildings()));
            }
        }
        Some("TheKingdomOfScotland") => {
            let n = island.lands.values().filter(|l| l.coastal && l.c.cities > 0).count();
            out.push(format!("Scotland's Trade Hub: {n} Coastal lands with a City; more than {} loses", 2 * boards));
        }
        Some("TheKingdomOfFrance") => {
            let towns: u32 = island.lands.values().map(|l| l.c.towns).sum();
            out.push(format!("France's Town supply: {towns} of {} placed", 7 * boards));
        }
        Some("TheTsardomOfRussia") => {
            let beasts: u32 = island.lands.values().map(|l| l.c.beasts).sum();
            out.push(format!("Russia's Hunters: {} Beasts destroyed, {beasts} on the island; more destroyed than on the island loses", rules.clocks.beasts_destroyed + tally.beasts_destroyed));
        }
        Some("TheHabsburgMonarchy") => out.push(format!("Habsburg Irreparable Damage: {} of {} Blight off the card in 6+ overkill Ravages; one more than that loses", rules.clocks.irreparable + tally.irreparable, rules.clocks.players.max(1))),
        Some("HabsburgMiningExpedition") => {
            if let Some(l) = island.lands.values().max_by_key(|l| l.c.invaders() + l.c.blight) {
                out.push(format!("Land Stripped Bare: {} has {} of 8 Invaders and Blight (checked at the end of the Fast phase)", l.id, l.c.invaders() + l.c.blight));
            }
        }
        _ => {}
    }
    if let Some((pool, flipped)) = rules.clocks.blight_pool {
        let left = pool - tally.blight as i64;
        out.push(if flipped { format!("Blight Card: {left} left (flipped); taking the last loses") } else { format!("Blight Card: {left} left before it flips") });
    }
    out
}

/// The player's split of `damage` among the Invaders: the most valuable set it destroys,
/// a Town worth three Explorers and a City five (the generic ledger with the Fear each gives),
/// the larger pieces first on a tie. `have` and `health` are (Explorers, Towns, Cities);
/// the result is how many of each die.
pub fn best_kills(damage: u32, have: (u32, u32, u32), health: (u32, u32, u32)) -> (u32, u32, u32) {
    let (he, ht, hc) = (health.0.max(1), health.1.max(1), health.2.max(1));
    let mut best = (0u32, 0u32, 0u32, 0u32);
    for c in 0..=have.2 {
        for t in 0..=have.1 {
            let cost = c * hc + t * ht;
            if cost > damage {
                continue;
            }
            let e = have.0.min((damage - cost) / he);
            let value = 5 * c + 3 * t + e;
            if (value, c, t) > (best.0, best.3, best.2) {
                best = (value, e, t, c);
            }
        }
    }
    (best.1, best.2, best.3)
}

impl Island {
    /// Time Passes: the turn's Defend, skips and other one-turn effects end; the pieces stay.
    pub fn time_passes(&mut self) {
        for l in self.lands.values_mut() {
            *l = Land { id: l.id.clone(), terrain: l.terrain, coastal: l.coastal, adjacent: std::mem::take(&mut l.adjacent), c: l.c, ..Default::default() };
        }
    }
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
    /// The player skips this step here by destroying one of their presence, as the record has
    /// them doing, while the land still holds their Sacred Site (Choke the Land with Green).
    fn chokes(&self, step: InvaderStep, land: &Land) -> bool {
        land.c.presence >= 2 && self.chokes.contains(&(step, land.id.clone())) && self.hooks.iter().any(|(_, h)| *h == si_model::spirit::RuleHook::SkipStepByDestroyingPresence)
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

    /// The spirit gathers 1 presence toward a Power's land first (Mists Shift and Flow).
    pub fn gathers_presence_on_targeting(&self, spirit: &str) -> bool {
        self.hooks.iter().any(|(s, h)| s == spirit && *h == si_model::spirit::RuleHook::GatherPresenceOnTargeting)
    }

    /// The spirit's Sacred Site counts as a Beast (A Joining of Swarms and Flocks).
    pub fn site_beast_for(&self, spirit: &str) -> bool {
        self.hooks.iter().any(|(s, h)| s == spirit && *h == si_model::spirit::RuleHook::SacredSiteIsBeast)
    }

    /// The spirit's Beasts gather and push from and to lands up to 2 away (Fly Fast as Thought).
    pub fn beasts_far_for(&self, spirit: &str) -> bool {
        self.hooks.iter().any(|(s, h)| s == spirit && *h == si_model::spirit::RuleHook::BeastsMoveTwoLands)
    }

    /// The spirit's presence moves with the Dahan it is with (Ally of the Dahan).
    pub fn presence_with_dahan_for(&self, spirit: &str) -> bool {
        self.hooks.iter().any(|(s, h)| s == spirit && *h == si_model::spirit::RuleHook::PresenceMovesWithDahan)
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
    /// Beasts the adversary's rules destroyed here (Russia's Hunters).
    #[serde(default)]
    pub beasts_destroyed: u32,
    /// Blight off the card in a Ravage with 6+ overkill (the Habsburg Monarchy's count).
    #[serde(default)]
    pub irreparable: u32,
    /// The Invaders the Dahan destroyed, by kind (a Town gives 1 Fear, a City 2).
    #[serde(default)]
    pub destroyed: Vec<PieceKind>,
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
fn explore_step(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
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
    let mut blitz_boards: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
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
            // Blitz: once per board, the Explore adds an extra Explorer (in the first land explored)
            if rules.scenario.as_deref() == Some("Blitz") && blitz_boards.insert(id.board.clone()) {
                l.c.explorers += 1;
                o.added.push(PieceKind::Explorer);
                o.notes.push("Blitz: +1 Explorer, once per board".into());
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
    } else if rules.chokes(InvaderStep::Build, l) {
        l.c.presence -= 1;
        o.presence_destroyed += 1;
        o.notes.push("Build skipped: a presence destroyed to choke it (Choke the Land with Green)".into());
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
fn build_step(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
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
fn ravage_step(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let mut targets = matching(island, card);
    if rules.russia(3) {
        for l in island.lands.values() {
            if l.c.explorers >= 3 && !targets.contains(&l.id) {
                targets.push(l.id.clone());
            }
        }
    }
    // the player's order when the record has it (a win can come between two lands)
    targets.sort_by_key(|id| rules.ravage_order.iter().position(|r| r == id).unwrap_or(usize::MAX));
    let mut out = Vec::new();
    for id in targets {
        if let Some(o) = ravage_land(island, &id, rules) {
            out.push(o);
        }
        if victory(island, rules).is_some() {
            break;
        }
    }
    out
}

/// Explore with one card; a piece the step took took its wound along.
pub fn explore(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let out = explore_step(island, card, rules);
    for l in island.lands.values_mut() {
        l.trim_wounds();
    }
    out
}

/// Build with one card; a piece the step took took its wound along.
pub fn build(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let out = build_step(island, card, rules);
    for l in island.lands.values_mut() {
        l.trim_wounds();
    }
    out
}

/// Ravage with one card; a piece the step took took its wound along.
pub fn ravage(island: &mut Island, card: &InvaderCard, rules: &Rules) -> Vec<Outcome> {
    let out = ravage_step(island, card, rules);
    for l in island.lands.values_mut() {
        l.trim_wounds();
    }
    out
}

/// The Terror Level's victory, as the reason the Spirits win: no Invaders on the island at
/// Terror Level I, no Towns or Cities at II, no Cities at III. None while the game goes on,
/// and always when the Terror Level is not known (`rules.terror` 0).
pub fn victory(island: &Island, rules: &Rules) -> Option<String> {
    if rules.terror == 0 || island.lands.is_empty() {
        return None;
    }
    let (e, t, c) = island.lands.values().fold((0, 0, 0), |(e, t, c), l| (e + l.c.explorers, t + l.c.towns, c + l.c.cities));
    match rules.terror {
        1 => (e + t + c == 0).then(|| "no Invaders on the island (Terror Level I)".to_string()),
        2 => (t + c == 0).then(|| "no Towns or Cities on the island (Terror Level II)".to_string()),
        _ => (c == 0).then(|| "no Cities on the island (Terror Level III)".to_string()),
    }
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
    if rules.chokes(InvaderStep::Ravage, l) {
        l.c.presence -= 1;
        o.presence_destroyed += 1;
        o.notes.push("Ravage skipped: a presence destroyed to choke it (Choke the Land with Green)".into());
        return Some(o);
    }
    let land_adjust = l.damage_adjust.clone();
    let adjust = |kind: PieceKind, base: i32| -> u32 {
        let mut d = base + rules.piece_damage_mod;
        for (a, kinds) in rules.damage_adjust.iter().chain(land_adjust.iter()) {
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
    let mut cascades = false;
    if dmg >= 2 && l.no_blight {
        o.notes.push("Blight not added: a Spirit effect keeps it on the card".into());
    }
    if dmg >= 2 && !l.no_blight {
        o.blight = 1;
        if rules.sweden(1) && dmg >= 6 {
            o.blight += 1;
            o.notes.push("Heavy Mining: 4+ overkill adds a Blight".into());
        }
        cascades = l.c.blight > 0 && !l.no_cascade;
        if l.c.presence > 0 {
            o.presence_destroyed = 1;
            o.notes.push("1 presence per Spirit present destroyed (one Spirit assumed)".into());
        }
        if rules.russia(1) && l.c.beasts > 0 {
            l.c.beasts -= 1;
            o.beasts_destroyed += 1;
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
    // as efficiently as the rules require: a Dahan already Damaged this turn falls first,
    // what is left stays on one (core rulebook p. 9)
    let dahan_before = l.c.dahan;
    let mut hurt = l.clone();
    o.dahan_lost = hurt.take_dahan_damage(to_dahan, dahan_health);
    let survivors = dahan_before - o.dahan_lost;
    // Words of Warning: the Dahan deal their damage with the Invaders', so the ones
    // this Ravage destroys strike back too
    let striking = if l.dahan_simultaneous { l.c.dahan } else { survivors };
    if l.dahan_simultaneous && o.dahan_lost > 0 {
        o.notes.push(format!("{} Dahan destroyed still dealt their damage (simultaneously with the Invaders)", o.dahan_lost));
    }
    let counter = if striking > 0 { striking * 2 + badlands } else { 0 };
    // the player distributes: the most valuable set the damage kills, each Invader at the
    // Health it has left this turn (England 5: +1 Health); the rest stays on a survivor
    let bonus = rules.england(5) as u32;
    let full = move |k: PieceKind| k.health() as u32 + bonus;
    let killed = l.take_damage(counter, &[PieceKind::Explorer, PieceKind::Town, PieceKind::City], &full, &[], false);
    for k in &killed {
        o.invaders_destroyed += 1;
        o.destroyed.push(*k);
    }
    if counter > 0 {
        o.notes.push("counterattack distribution: the most valuable Invaders it kills (player's choice)".into());
    }
    l.c.blight += o.blight;
    // the Dahan as the Invaders' Damage left them, wounds and all
    l.c.dahan = hurt.c.dahan;
    let dahan_wounds: Vec<(PieceKind, u32)> = hurt.wounds.iter().filter(|(k, _)| *k == PieceKind::Dahan).cloned().collect();
    l.wounds.retain(|(k, _)| *k != PieceKind::Dahan);
    l.wounds.extend(dahan_wounds);
    l.c.presence = l.c.presence.saturating_sub(o.presence_destroyed);
    if cascades {
        place_cascade(island, &id, rules, &mut o, 0);
    }
    // the Habsburg Monarchy counts the Blight a Ravage of 6+ overkill takes off the card
    if rules.adversary.as_deref() == Some("TheHabsburgMonarchy") && dmg >= 8 {
        o.irreparable = o.blight + o.cascade;
    }
    sworn_to_victory(island, &id, rules, &mut o);
    Some(o)
}

/// Blight cascading out of `from`: onto the land the record says the player chose (when the
/// cascade offered the same lands), else onto the neighbor a player would pick (no Blight, no
/// presence, no Dahan, the fewest Invaders). Blight landing where Blight already is cascades
/// again; each Blight added destroys a presence there, and Russia's Hunters a Beast.
fn place_cascade(island: &mut Island, from: &LandId, rules: &Rules, o: &mut Outcome, depth: u32) {
    if depth > 8 {
        return;
    }
    let options: Vec<LandId> = island.lands.get(from).map(|l| l.adjacent.iter().filter(|a| island.lands.get(*a).is_some_and(|n| n.terrain != Some(Terrain::Ocean))).cloned().collect()).unwrap_or_default();
    let mut offered = options.clone();
    offered.sort();
    let recorded: Option<LandId> = rules
        .cascades
        .iter()
        .find(|(o, _)| {
            let mut x = o.clone();
            x.sort();
            x == offered
        })
        .map(|(_, to)| to.clone())
        .filter(|t| options.contains(t));
    let guessed = || options.iter().min_by_key(|a| island.lands.get(*a).map(|l| (l.c.blight > 0, l.c.presence, l.c.dahan, l.c.invaders())).unwrap_or((true, 0, 0, 0))).cloned();
    let Some(to) = recorded.clone().or_else(guessed) else { return };
    let Some(l) = island.lands.get_mut(&to) else { return };
    let chains = l.c.blight > 0 && !l.no_cascade;
    l.c.blight += 1;
    o.cascade += 1;
    if l.c.presence > 0 {
        l.c.presence -= 1;
        o.presence_destroyed += 1;
    }
    if rules.russia(1) && l.c.beasts > 0 {
        l.c.beasts -= 1;
        o.beasts_destroyed += 1;
    }
    o.notes.push(format!("cascade onto {to}{}", if recorded.is_some() { "" } else { " (player's choice)" }));
    if chains {
        place_cascade(island, &to, rules, o, depth + 1);
    }
}

/// Thunderspeaker's Sworn to Victory: after a Ravage destroys Dahan, 1 of the spirit's
/// presence within range of the land is destroyed per Dahan. Which presence is the
/// player's choice: the ravaged land's first, then a neighbor's without Dahan, the fewest
/// first. Stacks stay, presence with Dahan stays (Manifestation multiplies both): what the
/// player chose both times the recorded games asked (Thunderspeaker T5, both attempts).
fn sworn_to_victory(island: &mut Island, id: &LandId, rules: &Rules, o: &mut Outcome) {
    let Some(range) = rules.hooks.iter().find_map(|(_, h)| if let si_model::spirit::RuleHook::PresenceDestroyedPerDahanKilled { range } = h { Some(*range) } else { None }) else { return };
    if o.dahan_lost == 0 {
        return;
    }
    let mut left = o.dahan_lost;
    let mut lands: Vec<LandId> = vec![id.clone()];
    if range >= 1 {
        let mut adj: Vec<LandId> = island.lands.get(id).map(|l| l.adjacent.clone()).unwrap_or_default();
        adj.sort_by_key(|a| island.lands.get(a).map(|l| (l.c.dahan > 0, l.c.presence)).unwrap_or((true, 0)));
        lands.extend(adj);
    }
    let mut destroyed = 0;
    for land in lands {
        let Some(l) = island.lands.get_mut(&land) else { continue };
        let k = l.c.presence.min(left);
        l.c.presence -= k;
        left -= k;
        destroyed += k;
        if left == 0 {
            break;
        }
    }
    if destroyed > 0 {
        o.presence_destroyed += destroyed;
        o.notes.push(format!("Sworn to Victory: {destroyed} presence destroyed for {} Dahan lost (which presence is the player's choice)", o.dahan_lost));
    }
}

/// The laws the Invader steps keep, stated from outside them, and the generators their
/// fuzz test draws from (`tests::every_invader_step_keeps_its_laws`). Under the base rules
/// (no adversary, no scenario, no flags) the laws are exact; under an adversary's rules
/// they are the ones every level keeps (`adversary_holds`). `si fuzz --rules` runs every
/// adversary at every level through them.
pub mod oracle {
    use super::*;
    use si_model::oracle::{check, Rng, Violation};

    const KINDS: [&str; 11] = ["E", "T", "C", "Dh", "Bl", "Pr", "Be", "Di", "St", "Wi", "Ba"];

    fn fields(c: &Counts) -> [u32; 11] {
        [c.explorers, c.towns, c.cities, c.dahan, c.blight, c.presence, c.beasts, c.disease, c.strife, c.wilds, c.badlands]
    }

    fn changed(before: &Island, after: &Island) -> Vec<LandId> {
        after.lands.iter().filter(|(id, l)| before.lands.get(*id).is_none_or(|b| b.c != l.c)).map(|(id, _)| id.clone()).collect()
    }

    /// Explore under the base rules: an Explorer into each matching land the Invaders
    /// reach (a Town or City in it or next door, or the Ocean beside it) unless it is
    /// skipped or isolated; Wilds take the Explore instead; nothing else changes.
    pub fn explore_holds(card: &InvaderCard, before: &Island, after: &Island) -> Vec<Violation> {
        let mut out = Vec::new();
        for (id, b) in &before.lands {
            let Some(a) = after.lands.get(id) else { continue };
            let reached = b.coastal || b.c.buildings() > 0 || before.buildings_adjacent(id) > 0;
            let explores = b.terrain != Some(Terrain::Ocean) && card_matches(card, b) && reached && !b.skip_explore && !b.isolated;
            let mut want = b.c;
            if explores {
                if b.c.wilds > 0 {
                    want.wilds -= 1;
                } else {
                    want.explorers += 1;
                }
            }
            check(&mut out, a.c == want, "Explore adds an Explorer where the Invaders reach", || format!("{id} ({:?}, coastal {}, reached {reached}): {:?} -> {:?}", b.terrain, b.coastal, b.c, a.c));
        }
        out
    }

    /// Build under the base rules: in each matching land with Invaders, a City where Towns
    /// outnumber Cities, else a Town; Disease stops it and goes; a skip stops it; nothing
    /// else changes.
    pub fn build_holds(card: &InvaderCard, before: &Island, after: &Island) -> Vec<Violation> {
        let mut out = Vec::new();
        for (id, b) in &before.lands {
            let Some(a) = after.lands.get(id) else { continue };
            let builds = b.terrain != Some(Terrain::Ocean) && card_matches(card, b) && b.c.invaders() > 0;
            let mut want = b.c;
            if builds && !b.skip_build {
                if b.c.disease > 0 {
                    want.disease -= 1;
                } else if b.c.towns > b.c.cities {
                    want.cities += 1;
                } else {
                    want.towns += 1;
                }
            }
            check(&mut out, a.c == want, "Build adds one Town or City where Invaders are", || format!("{id}: {:?} -> {:?}", b.c, a.c));
        }
        out
    }

    /// Ravage under the base rules: only the matching lands with Invaders and the lands
    /// Blight cascades into change; no Invader, Dahan, presence or token ever appears; a
    /// land takes at most one Blight of its own, each cascade one in a neighbor; the
    /// outcome's counts are the board's; the Dahan lose no more than the Damage kills;
    /// Defend and Invaders' Damage under 2 add no Blight.
    pub fn ravage_holds(card: &InvaderCard, before: &Island, after: &Island, outcomes: &[Outcome]) -> Vec<Violation> {
        let mut out = Vec::new();
        let (b0, b1): ([u32; 11], [u32; 11]) = (sum(before), sum(after));
        for i in [0usize, 1, 2, 3, 5, 6, 7, 8, 9, 10] {
            check(&mut out, b1[i] <= b0[i], "a Ravage adds no piece but Blight", || format!("{} {} -> {}", KINDS[i], b0[i], b1[i]));
        }
        let blight_added: u32 = outcomes.iter().map(|o| o.blight + o.cascade).sum();
        check(&mut out, b1[4] <= b0[4] + blight_added, "Blight as the outcomes say", || format!("Blight {} -> {}, {blight_added} added", b0[4], b1[4]));
        let ravaged: Vec<&LandId> = outcomes.iter().map(|o| &o.land).collect();
        for id in changed(before, after) {
            let by_cascade = ravaged.iter().any(|r| before.lands.get(*r).is_some_and(|l| l.adjacent.contains(&id))) || outcomes.iter().any(|o| o.cascade > 0);
            check(&mut out, ravaged.contains(&&id) || by_cascade, "a Ravage acts where it ravages", || format!("{id} changed"));
        }
        for o in outcomes {
            let (Some(b), Some(a)) = (before.lands.get(&o.land), after.lands.get(&o.land)) else { continue };
            check(&mut out, card_matches(card, b) && b.c.invaders() > 0, "a Ravage where the card says, with Invaders", || format!("{} ravaged", o.land));
            check(&mut out, o.blight <= 1, "one Blight of a land's own", || format!("{} took {}", o.land, o.blight));
            check(&mut out, o.dahan_lost == b.c.dahan - a.c.dahan.min(b.c.dahan), "the Dahan lost are the board's", || format!("{}: {} lost, {} -> {}", o.land, o.dahan_lost, b.c.dahan, a.c.dahan));
            // a Badlands adds its Damage to what reaches the Dahan
            let to_dahan = if o.damage > 0 { o.damage + b.c.badlands } else { 0 };
            let hurt: u32 = b.wounds.iter().filter(|(k, _)| *k == PieceKind::Dahan).map(|(_, d)| *d).sum();
            check(&mut out, o.dahan_lost * (2 + b.dahan_health) <= to_dahan + hurt, "the Damage kills the Dahan lost", || format!("{}: {} Dahan for {} Damage, {} Badlands, {hurt} on them already", o.land, o.dahan_lost, o.damage, b.c.badlands));
            check(&mut out, o.invaders_destroyed == b.c.invaders() - a.c.invaders().min(b.c.invaders()), "the Invaders destroyed are the board's", || format!("{}: {} destroyed, {} -> {}", o.land, o.invaders_destroyed, b.c.invaders(), a.c.invaders()));
            check(&mut out, o.damage >= 2 || o.blight == 0, "under 2 Damage adds no Blight", || format!("{}: {} Damage, {} Blight", o.land, o.damage, o.blight));
        }
        out
    }

    fn sum(island: &Island) -> [u32; 11] {
        let mut t = [0u32; 11];
        for l in island.lands.values() {
            for (i, x) in fields(&l.c).iter().enumerate() {
                t[i] += x;
            }
        }
        t
    }

    /// What every adversary's rules keep at every level, in every step: no Dahan, presence
    /// or Beast ever appears, Blight appears only in a Ravage, and a game is never both won
    /// and lost.
    pub fn adversary_holds(step: InvaderStep, before: &Island, after: &Island, rules: &Rules) -> Vec<Violation> {
        let mut out = Vec::new();
        let (b0, b1) = (sum(before), sum(after));
        for i in [3usize, 5, 6] {
            check(&mut out, b1[i] <= b0[i], "no Dahan, presence or Beast appears", || format!("{step:?}: {} {} -> {}", KINDS[i], b0[i], b1[i]));
        }
        if step != InvaderStep::Ravage {
            check(&mut out, b1[4] == b0[4], "Blight only in a Ravage", || format!("{step:?}: Blight {} -> {}", b0[4], b1[4]));
        }
        // a board that both wins and loses is a sacrifice victory, a win
        let won = victory(after, rules);
        let lost = loss(after, rules, Tally::default());
        let end = ending(after, rules, Tally::default(), false);
        let want = match (&won, &lost) {
            (Some(w), _) => Some(Ok(w.clone())),
            (None, Some(l)) => Some(Err(l.clone())),
            (None, None) => None,
        };
        check(&mut out, end == want, "a win and a loss together are a win", || format!("{won:?} / {lost:?} ended {end:?}"));
        out
    }

    /// The Terror Level's victory as the board shows it: at I no Invaders, at II no Towns or
    /// Cities, at III no Cities.
    pub fn victory_holds(island: &Island, rules: &Rules) -> Vec<Violation> {
        let mut out = Vec::new();
        let t = sum(island);
        let want = match rules.terror {
            0 => false,
            1 => t[0] + t[1] + t[2] == 0,
            2 => t[1] + t[2] == 0,
            _ => t[2] == 0,
        };
        check(&mut out, victory(island, rules).is_some() == want, "victory as the Terror Level says", || format!("TL{} E{} T{} C{}", rules.terror, t[0], t[1], t[2]));
        out
    }

    /// A small island to act on: an Ocean and six lands in a ring with two chords, the two
    /// lands beside the Ocean coastal, random pieces and tokens in each, now and then a
    /// Defend or a skip.
    pub fn arbitrary_island(rng: &mut Rng) -> Island {
        let id = |n: u8| LandId::new("A", n);
        let edges: [(u8, u8); 10] = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 1), (1, 4), (2, 5), (0, 1), (0, 2)];
        let terrains = [Terrain::Ocean, Terrain::Mountain, Terrain::Wetland, Terrain::Jungle, Terrain::Sands, Terrain::Mountain, Terrain::Jungle];
        let mut lands: BTreeMap<LandId, Land> = BTreeMap::new();
        for n in 0..=6u8 {
            let adjacent: Vec<LandId> = edges.iter().filter_map(|(a, b)| if *a == n { Some(id(*b)) } else if *b == n { Some(id(*a)) } else { None }).collect();
            let mut c = Counts::default();
            if n > 0 {
                let small = |r: &mut Rng, hi: u32| if r.one_in(3) { 0 } else { r.between(0, hi) };
                c.explorers = small(rng, 3);
                c.towns = small(rng, 2);
                c.cities = small(rng, 1);
                c.dahan = small(rng, 3);
                c.blight = small(rng, 1);
                c.presence = small(rng, 2);
                c.beasts = small(rng, 2);
                c.disease = small(rng, 1);
                c.strife = small(rng, 1);
                c.wilds = small(rng, 1);
                c.badlands = small(rng, 1);
            }
            let coastal = adjacent.contains(&id(0));
            let mut land = Land { id: id(n), terrain: Some(terrains[n as usize]), coastal, adjacent, c, ..Default::default() };
            if n > 0 {
                land.wounds = arbitrary_wounds(rng, &land.c);
                land.defend = if rng.one_in(4) { rng.between(1, 3) } else { 0 };
                land.skip_ravage = rng.one_in(10);
                land.skip_build = rng.one_in(10);
                land.skip_explore = rng.one_in(10);
            }
            lands.insert(id(n), land);
        }
        Island { lands }
    }

    /// A random game state on real boards (the definitions' layouts, lands touching across
    /// boards along their shared numbers when there are two): random pieces and tokens in
    /// every land but the Oceans, now and then a Defend or a skip.
    pub fn arbitrary_island_on(rng: &mut Rng, boards: &[&si_model::island::BoardDef]) -> Island {
        let mut lands: BTreeMap<LandId, Land> = BTreeMap::new();
        for b in boards {
            for d in &b.lands {
                let mut c = Counts::default();
                if d.terrain != Terrain::Ocean {
                    let small = |r: &mut Rng, hi: u32| if r.one_in(3) { 0 } else { r.between(0, hi) };
                    c.explorers = small(rng, 3);
                    c.towns = small(rng, 2);
                    c.cities = small(rng, 1);
                    c.dahan = small(rng, 3);
                    c.blight = small(rng, 1);
                    c.presence = small(rng, 2);
                    c.beasts = small(rng, 1);
                    c.disease = small(rng, 1);
                    c.strife = small(rng, 1);
                    c.wilds = small(rng, 1);
                    c.badlands = small(rng, 1);
                }
                let mut land = Land { id: d.id.clone(), terrain: Some(d.terrain), coastal: d.coastal(), adjacent: d.adjacent.iter().map(|n| LandId::new(&d.id.board, *n)).collect(), c, ..Default::default() };
                land.wounds = arbitrary_wounds(rng, &land.c);
                land.defend = if rng.one_in(5) { rng.between(1, 3) } else { 0 };
                land.skip_ravage = rng.one_in(12);
                land.skip_build = rng.one_in(12);
                land.skip_explore = rng.one_in(12);
                lands.insert(d.id.clone(), land);
            }
        }
        // two boards touch: a land of one beside the same-numbered land of the other
        if boards.len() == 2 {
            let code = |b: &si_model::island::BoardDef| b.lands.first().map(|l| l.id.board.clone()).unwrap_or_default();
            for n in [1u8, 2, 3] {
                let (a, b) = (LandId::new(&code(boards[0]), n), LandId::new(&code(boards[1]), n));
                if lands.contains_key(&a) && lands.contains_key(&b) {
                    lands.get_mut(&a).unwrap().adjacent.push(b.clone());
                    lands.get_mut(&b).unwrap().adjacent.push(a.clone());
                }
            }
        }
        Island { lands }
    }

    /// Damage already on some pieces this turn: now and then a Town, City or Dahan wounded.
    pub fn arbitrary_wounds(rng: &mut Rng, c: &Counts) -> Vec<(PieceKind, u32)> {
        let mut w = Vec::new();
        for (k, n, full) in [(PieceKind::Town, c.towns, 2), (PieceKind::City, c.cities, 3), (PieceKind::Dahan, c.dahan, 2)] {
            for _ in 0..n {
                if rng.one_in(4) {
                    w.push((k, rng.between(1, full - 1)));
                }
            }
        }
        w
    }

    /// The wounds' law: no more wounds of a kind than pieces of it, none at or past its
    /// piece's Health (`bonus` the Invaders' extra Health).
    pub fn wounds_hold(island: &Island, bonus: u32, dahan_bonus: u32) -> Vec<Violation> {
        let mut out = Vec::new();
        for l in island.lands.values() {
            for (k, n) in [(PieceKind::Explorer, l.c.explorers), (PieceKind::Town, l.c.towns), (PieceKind::City, l.c.cities), (PieceKind::Dahan, l.c.dahan)] {
                let w: Vec<u32> = l.wounds.iter().filter(|(x, _)| *x == k).map(|(_, d)| *d).collect();
                check(&mut out, w.len() as u32 <= n, "no more wounds than pieces", || format!("{}: {} wounded {k:?}, {n} there", l.id, w.len()));
                let full = if k == PieceKind::Dahan { 2 + dahan_bonus + l.dahan_health } else { k.health() as u32 + bonus };
                check(&mut out, w.iter().all(|d| *d < full), "a wound short of the piece's Health", || format!("{}: {k:?} wounds {w:?}, Health {full}", l.id));
            }
        }
        out
    }

    /// An Invader card of any stage: one terrain at I, a terrain or the coast at II (an
    /// escalation half the time), two terrains at III.
    pub fn arbitrary_card(rng: &mut Rng) -> InvaderCard {
        let t = [Terrain::Mountain, Terrain::Wetland, Terrain::Jungle, Terrain::Sands];
        let pick = |r: &mut Rng| InvaderTarget::Terrain(t[r.below(4)]);
        let stage = rng.between(1, 3) as u8;
        let (targets, escalation) = match stage {
            1 => (vec![pick(rng)], false),
            2 => (vec![if rng.one_in(5) { InvaderTarget::Coastal } else { pick(rng) }], rng.coin()),
            _ => (vec![pick(rng), pick(rng)], false),
        };
        InvaderCard { id: format!("Fuzz{stage}"), stage, targets, escalation }
    }

    /// Rules with an adversary at a level, or none, and a Terror Level.
    pub fn arbitrary_rules(rng: &mut Rng) -> Rules {
        let adversaries = ["TheKingdomOfEngland", "TheKingdomOfBrandenburgPrussia", "TheKingdomOfSweden", "TheTsardomOfRussia", "TheKingdomOfFrance", "TheHabsburgMonarchy", "HabsburgMiningExpedition", "TheKingdomOfScotland"];
        let adversary = if rng.one_in(3) { None } else { Some(adversaries[rng.below(adversaries.len())].to_string()) };
        Rules { adversary, level: rng.between(0, 6) as u8, terror: rng.between(1, 3) as u8, ..Default::default() }
    }
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
        assert_eq!(o.invaders_destroyed, 1, "the survivor's 2 damage kills the Town (2), worth more than the Explorer (1)");
        let l = &isl.lands[&"A1".parse().unwrap()];
        assert_eq!((l.c.explorers, l.c.towns, l.c.dahan, l.c.blight), (1, 0, 1, 1));
    }

    #[test]
    fn every_loss_condition_reads_off_the_board_and_the_counts() {
        let adv = |a: &str| Rules { adversary: Some(a.into()), ..Default::default() };
        let one = |c: Counts| island(vec![land("A1", Terrain::Sands, true, &["A2"], c), land("A2", Terrain::Jungle, false, &["A1"], Counts { presence: 1, ..Default::default() })]);
        let t0 = Tally::default();
        // England: 7 Town/City in one land
        assert!(loss(&one(Counts { towns: 4, cities: 2, ..Default::default() }), &adv("TheKingdomOfEngland"), t0).is_none());
        assert!(loss(&one(Counts { towns: 4, cities: 3, ..Default::default() }), &adv("TheKingdomOfEngland"), t0).is_some_and(|r| r.contains("A1")));
        // France: more Towns than 7 per board
        assert!(loss(&one(Counts { towns: 7, ..Default::default() }), &adv("TheKingdomOfFrance"), t0).is_none());
        assert!(loss(&one(Counts { towns: 8, ..Default::default() }), &adv("TheKingdomOfFrance"), t0).is_some());
        // Scotland: Coastal lands with a City over 2 per board (one coastal land here: never)
        assert!(loss(&one(Counts { cities: 3, ..Default::default() }), &adv("TheKingdomOfScotland"), t0).is_none());
        // Russia: more Beasts destroyed by the Hunters than Beasts on the island
        let mut r = adv("TheTsardomOfRussia");
        r.clocks.beasts_destroyed = 1;
        assert!(loss(&one(Counts { beasts: 1, ..Default::default() }), &r, t0).is_none());
        assert!(loss(&one(Counts { beasts: 1, ..Default::default() }), &r, Tally { beasts_destroyed: 1, ..t0 }).is_some());
        // the Habsburg Monarchy: irreparable Blight over the players
        let mut h = adv("TheHabsburgMonarchy");
        h.clocks.players = 1;
        assert!(loss(&one(Counts::default()), &h, Tally { irreparable: 1, ..t0 }).is_none());
        assert!(loss(&one(Counts::default()), &h, Tally { irreparable: 2, ..t0 }).is_some());
        // Habsburg Mining: 8 Invaders and Blight in a land at the end of the Fast phase
        assert!(loss_after_fast(&one(Counts { explorers: 5, towns: 1, blight: 2, ..Default::default() }), &adv("HabsburgMiningExpedition"), t0).is_some());
        assert!(loss(&one(Counts { explorers: 5, towns: 1, blight: 2, ..Default::default() }), &adv("HabsburgMiningExpedition"), t0).is_none(), "checked at the Fast phase's end only");
        // core: a flipped Blight Card emptied (not an unflipped one), the last presence gone
        let mut b = Rules::default();
        b.clocks.blight_pool = Some((2, true));
        assert!(loss(&one(Counts::default()), &b, Tally { blight: 1, ..t0 }).is_none());
        assert!(loss(&one(Counts::default()), &b, Tally { blight: 2, ..t0 }).is_some());
        b.clocks.blight_pool = Some((2, false));
        assert!(loss(&one(Counts::default()), &b, Tally { blight: 3, ..t0 }).is_none(), "an unflipped card flips");
        let bare = island(vec![land("A1", Terrain::Sands, false, &[], Counts::default())]);
        assert!(loss(&bare, &Rules::default(), t0).is_some_and(|r| r.contains("presence")));
    }

    #[test]
    fn russias_hunters_and_the_habsburg_count_come_out_of_the_ravage() {
        let mut isl = island(vec![land("A1", Terrain::Sands, false, &[], Counts { towns: 5, beasts: 1, presence: 1, ..Default::default() })]);
        let r = Rules { adversary: Some("TheTsardomOfRussia".into()), level: 1, ..Default::default() };
        let out = ravage(&mut isl, &card(Terrain::Sands), &r);
        assert_eq!(out[0].beasts_destroyed, 1);
        let mut isl = island(vec![land("A1", Terrain::Sands, false, &[], Counts { towns: 4, presence: 1, ..Default::default() })]);
        let h = Rules { adversary: Some("TheHabsburgMonarchy".into()), ..Default::default() };
        let out = ravage(&mut isl, &card(Terrain::Sands), &h);
        assert_eq!((out[0].damage, out[0].irreparable), (8, 1), "8 damage: 6 overkill, its Blight counts");
    }

    #[test]
    fn sworn_to_victory_destroys_presence_per_dahan_lost() {
        // 12 damage in A1 kills both Dahan: the Blight takes A1's presence, then Sworn to Victory
        // takes 2 more within range 1: lands without Dahan first, the fewest first (A3, then A4),
        // A2's presence with its Dahan last
        let mut isl = island(vec![
            land("A1", Terrain::Sands, false, &["A2", "A3", "A4"], Counts { towns: 6, dahan: 2, presence: 1, ..Default::default() }),
            land("A2", Terrain::Jungle, false, &["A1"], Counts { dahan: 1, presence: 2, ..Default::default() }),
            land("A3", Terrain::Jungle, false, &["A1"], Counts { presence: 1, ..Default::default() }),
            land("A4", Terrain::Jungle, false, &["A1"], Counts { presence: 2, ..Default::default() }),
        ]);
        let rules = Rules { hooks: vec![("Thunderspeaker".into(), si_model::spirit::RuleHook::PresenceDestroyedPerDahanKilled { range: 1 })], ..Default::default() };
        let out = ravage(&mut isl, &card(Terrain::Sands), &rules);
        assert_eq!((out[0].dahan_lost, out[0].presence_destroyed), (2, 3));
        let p = |id: &str| isl.lands[&id.parse().unwrap()].c.presence;
        assert_eq!((p("A1"), p("A2"), p("A3"), p("A4")), (0, 2, 0, 1));
    }

    #[test]
    fn words_of_warning_dahan_strike_back_as_they_fall() {
        // 4 damage kills the lone Dahan; with Words of Warning it still deals its 2 and kills a Town
        let mk = || island(vec![land("A1", Terrain::Sands, false, &[], Counts { towns: 2, dahan: 1, ..Default::default() })]);
        let mut isl = mk();
        let out = ravage(&mut isl, &card(Terrain::Sands), &Rules::default());
        assert_eq!((out[0].dahan_lost, out[0].invaders_destroyed), (1, 0));
        let mut isl = mk();
        isl.lands.get_mut(&"A1".parse().unwrap()).unwrap().dahan_simultaneous = true;
        let out = ravage(&mut isl, &card(Terrain::Sands), &Rules::default());
        assert_eq!((out[0].dahan_lost, out[0].invaders_destroyed, out[0].destroyed.clone()), (1, 1, vec![PieceKind::Town]));
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

    #[test]
    fn every_invader_step_keeps_its_laws() {
        use super::oracle::*;
        use si_model::oracle::{property, Violation};
        // the base rules: Explore, Build and Ravage exactly as the rulebook has them
        property(
            "invader steps",
            1,
            6000,
            |rng| (arbitrary_island(rng), arbitrary_card(rng), rng.below(3)),
            |(island, card, step)| {
                let rules = Rules::default();
                let card = InvaderCard { escalation: false, ..card.clone() };
                let mut after = island.clone();
                let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match step {
                    0 => (explore(&mut after, &card, &rules), 0),
                    1 => (build(&mut after, &card, &rules), 1),
                    _ => (ravage(&mut after, &card, &rules), 2),
                }));
                let Ok((outcomes, s)) = ran else { return vec![Violation::new("no panic", "the step panicked")] };
                match s {
                    0 => explore_holds(&card, island, &after),
                    1 => build_holds(&card, island, &after),
                    _ => ravage_holds(&card, island, &after, &outcomes),
                }
            },
        );
        // every adversary at every level: the laws all of them keep
        property(
            "invader steps under an adversary",
            2,
            6000,
            |rng| (arbitrary_island(rng), arbitrary_card(rng), arbitrary_rules(rng), rng.below(3)),
            |(island, card, rules, step)| {
                let mut after = island.clone();
                let s = [InvaderStep::Explore, InvaderStep::Build, InvaderStep::Ravage][*step];
                let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match s {
                    InvaderStep::Explore => explore(&mut after, card, rules),
                    InvaderStep::Build => build(&mut after, card, rules),
                    InvaderStep::Ravage => ravage(&mut after, card, rules),
                }));
                if ran.is_err() {
                    return vec![Violation::new("no panic", "the step panicked")];
                }
                let mut v = adversary_holds(s, island, &after, rules);
                v.extend(victory_holds(&after, rules));
                v.extend(wounds_hold(&after, rules.invader_health_bonus(), 0));
                v
            },
        );
    }
}
