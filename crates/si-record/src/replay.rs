//! Walk a log's events with the turn and phase tracked and build the record.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use anyhow::{Context, Result};
use si_defs::Definitions;
use si_model::event::{Event, InvaderStep};
use si_model::game::Phase;
use si_model::island::{LandId, PieceKind, TokenKind};

use crate::log::{self, Line, Marker, Parsed};
use crate::record::*;

/// Read a log file (EndGame--*.txt, or a capture's log-stream.txt).
pub fn read_log(path: &Path) -> Result<Vec<Line>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(text.lines().map(log::split_stamp).collect())
}

/// `events.txt` of a capture: the clock at which each `save-Undo-k.dat` was written.
pub fn read_undo_times(path: &Path) -> Result<Vec<(u64, u32)>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut out = Vec::new();
    for raw in text.lines() {
        let l = log::split_stamp(raw);
        let (Some(ms), Some(rest)) = (l.ms, l.text.strip_prefix("NEW")) else { continue };
        if let Some(k) = rest.trim().strip_prefix("save-Undo-").and_then(|r| r.trim_end_matches(".dat").parse::<u32>().ok()) {
            out.push((ms, k));
        }
    }
    Ok(out)
}

/// A game folder (capture) or a single log file.
pub fn load(path: &Path) -> Result<(Vec<Line>, Option<Vec<(u64, u32)>>, String)> {
    if path.is_dir() {
        let lines = read_log(&path.join("log-stream.txt"))?;
        let undo = path.join("events.txt");
        let undo = if undo.exists() { Some(read_undo_times(&undo)?) } else { None };
        Ok((lines, undo, path.display().to_string()))
    } else {
        Ok((read_log(path)?, None, path.display().to_string()))
    }
}

pub struct Replay<'d> {
    pub defs: Option<&'d Definitions>,
}

/// The digital edition's adversaries: identifier and title, as log lines name
/// them (`Source=TheKingdomOfSweden`, `TheTsardomOfRussiaAdversary`,
/// `Source: Habsburg Mining Expedition`).
pub const ADVERSARIES: [(&str, &str); 8] = [
    ("HabsburgMiningExpedition", "Habsburg Mining Expedition"),
    ("TheHabsburgMonarchy", "The Habsburg Monarchy"),
    ("TheKingdomOfBrandenburgPrussia", "The Kingdom of Brandenburg-Prussia"),
    ("TheKingdomOfEngland", "The Kingdom of England"),
    ("TheKingdomOfFrance", "The Kingdom of France"),
    ("TheKingdomOfScotland", "The Kingdom of Scotland"),
    ("TheKingdomOfSweden", "The Kingdom of Sweden"),
    ("TheTsardomOfRussia", "The Tsardom of Russia"),
];

#[derive(Default)]
struct Piles {
    hand: Vec<String>,
    play: Vec<String>,
    discard: Vec<String>,
}

fn key(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase()
}

fn remove_key(v: &mut Vec<String>, k: &str) -> Option<String> {
    let i = v.iter().position(|c| key(c) == k)?;
    Some(v.remove(i))
}

impl Piles {
    /// Take a card out of every pile (the same card never sits in two).
    fn take(&mut self, k: &str) -> Option<String> {
        remove_key(&mut self.hand, k).or_else(|| remove_key(&mut self.play, k)).or_else(|| remove_key(&mut self.discard, k))
    }
}

struct Walk<'d> {
    defs: Option<&'d Definitions>,
    rec: GameRecord,
    turn: u32,
    phase: Phase,
    invaded: bool,
    pieces: HashMap<String, (LandId, PieceKind)>,
    presence: BTreeMap<LandId, BTreeMap<String, u32>>,
    blight: BTreeMap<LandId, u32>,
    tokens: BTreeMap<LandId, BTreeMap<TokenKind, u32>>,
    /// Boards named by the log's adjacency line, in order.
    boards: Vec<String>,
    /// The boards' printed pieces have been placed (they are never logged).
    seeded: bool,
    revealed: BTreeMap<String, BTreeSet<String>>,
    energy: BTreeMap<String, i64>,
    piles: BTreeMap<String, Piles>,
    terror: u8,
    // the Invader step in progress
    step_adds: Vec<(LandId, PieceKind)>,
    step_ravaged: Vec<LandId>,
    /// The Invader card named by the Ravage in progress, for a game that ends mid-step.
    ravage_source: Option<String>,
    step_blight: BTreeMap<LandId, (u32, u32)>,
    step_damage: BTreeMap<LandId, u32>,
    step_dahan_lost: BTreeMap<LandId, u32>,
    step_invaders_destroyed: BTreeMap<LandId, u32>,
    defend_this_turn: BTreeMap<LandId, u32>,
    /// Piece ids a move already took off the island this step (the engine
    /// logs a destroyed piece as a move out and then a destroy).
    gone_this_step: BTreeSet<String>,
    power_phase: BTreeMap<String, Phase>,
    spirit_names: BTreeMap<String, String>,
    single_spirit: Option<String>,
}

fn si_record_canceled(step: si_model::event::InvaderStep, land: Option<LandId>, source: String) -> crate::record::Canceled {
    crate::record::Canceled { step, land, source }
}

impl<'d> Replay<'d> {
    pub fn new(defs: Option<&'d Definitions>) -> Self {
        Replay { defs }
    }

    pub fn run(&self, lines: &[Line], undo_times: Option<&[(u64, u32)]>, source: &str) -> GameRecord {
        let mut coverage = Coverage::default();
        let (copy, copies) = log::timed_last_copy(lines);
        coverage.copies = copies;
        let adversary = ADVERSARIES.iter().find(|(id, title)| copy.iter().any(|l| l.text.contains(id) || l.text.contains(title))).map(|(id, _)| id.to_string());
        let parsed: Vec<(Option<u64>, Parsed)> = copy.iter().map(|l| (l.ms, log::parse(&l.text))).collect();
        let kept = resolve_undo(parsed, undo_times, &mut coverage);
        let mut w = Walk {
            defs: self.defs,
            rec: GameRecord { meta: GameMeta { source: source.to_string(), adversary, ..Default::default() }, coverage, ..Default::default() },
            turn: 0,
            phase: Phase::Other,
            invaded: false,
            pieces: HashMap::new(),
            presence: BTreeMap::new(),
            blight: BTreeMap::new(),
            tokens: BTreeMap::new(),
            boards: Vec::new(),
            seeded: false,
            revealed: BTreeMap::new(),
            energy: BTreeMap::new(),
            piles: BTreeMap::new(),
            terror: 1,
            step_adds: vec![],
            step_ravaged: vec![],
            ravage_source: None,
            step_blight: BTreeMap::new(),
            step_damage: BTreeMap::new(),
            step_dahan_lost: BTreeMap::new(),
            step_invaders_destroyed: BTreeMap::new(),
            defend_this_turn: BTreeMap::new(),
            gone_this_step: BTreeSet::new(),
            power_phase: BTreeMap::new(),
            spirit_names: BTreeMap::new(),
            single_spirit: None,
        };
        for (_, p) in kept {
            match p {
                Parsed::Event(e) => {
                    *w.rec.coverage.typed.entry(e.kind().to_string()).or_insert(0) += 1;
                    w.apply(e);
                }
                Parsed::Many(events) => {
                    for e in events {
                        *w.rec.coverage.typed.entry(e.kind().to_string()).or_insert(0) += 1;
                        w.apply(e);
                    }
                }
                Parsed::Marker(m) => w.marker(m),
                Parsed::Skipped(name) => *w.rec.coverage.skipped.entry(name).or_insert(0) += 1,
                Parsed::Noise => w.rec.coverage.noise += 1,
            }
        }
        w.finish_turn();
        w.rec.meta.turns = w.turn;
        let ends: Vec<(LandId, Counts)> = w.rec.lands.keys().map(|l| (l.clone(), w.counts_of(l))).collect();
        for (land, end) in ends {
            if let Some(l) = w.rec.lands.get_mut(&land) {
                l.end = end;
            }
        }
        let mut rec = w.rec;
        rec.meta.spirits = w.spirit_names.values().cloned().collect::<BTreeSet<_>>().into_iter().collect();
        rec.meta.boards = if w.boards.is_empty() { rec.lands.keys().map(|l| l.board.clone()).collect::<BTreeSet<_>>().into_iter().collect() } else { w.boards.clone() };
        rec
    }
}

/// Drop the events an undo reload reverted.
///
/// Exact when the capture's save-write clock is available. Without it, the
/// reload is known to revert at least the actions since the last completed
/// save (the checkpoint state is a saved state), so that tail is dropped;
/// then every fact the game cannot record twice (a track slot revealed, a
/// card gained or played in a turn, an Invader card's step in a turn, a
/// phase change in a turn …) that shows up again after the reload proves the
/// undo reached back to its first occurrence, and the segment from there to
/// the reload is dropped too.
fn resolve_undo(parsed: Vec<(Option<u64>, Parsed)>, undo_times: Option<&[(u64, u32)]>, cov: &mut Coverage) -> Vec<(Option<u64>, Parsed)> {
    let mut kept: Vec<(Option<u64>, Parsed)> = Vec::with_capacity(parsed.len());
    let mut saves: Vec<(usize, Option<u64>)> = Vec::new();
    let mut facts: HashMap<String, usize> = HashMap::new();
    // the boundary between a heuristically resolved undo's past and its redo
    let mut reload_pos: Option<usize> = None;
    // the turn proxy for per-turn facts: how many Invader cards have been
    // revealed (each is revealed once per game, so an undone reveal that is
    // logged again does not move it)
    let mut revealed: std::collections::BTreeSet<String> = Default::default();
    for (ms, p) in parsed {
        match &p {
            Parsed::Marker(Marker::Saving) => saves.push((kept.len(), ms)),
            Parsed::Marker(Marker::LoadedUndo(k)) => {
                cov.undo_reloads += 1;
                let exact = match (undo_times, ms) {
                    (Some(times), Some(load_ms)) => match times.iter().filter(|(t, kk)| *kk == *k && *t < load_ms).map(|(t, _)| *t).max() {
                        Some(write_ms) => saves.iter().filter(|(_, t)| t.is_some_and(|t| t <= write_ms + 80)).map(|(pos, _)| *pos).last(),
                        None => {
                            cov.notes.push(format!("undo {k}: no save-Undo-{k} write before the reload"));
                            None
                        }
                    },
                    _ => None,
                };
                let target = match exact {
                    Some(pos) => pos,
                    None => {
                        cov.undo_unresolved += 1;
                        saves.last().map(|(pos, _)| *pos).unwrap_or(kept.len())
                    }
                };
                let end = kept.len();
                drop_range(&mut kept, &mut saves, &mut facts, target, end, cov);
                if exact.is_none() {
                    reload_pos = Some(target);
                }
                continue;
            }
            _ => {}
        }
        if let Parsed::Event(e) = &p {
            if let Some(fact) = unique_fact(e, revealed.len() as u32) {
                if let (Some(&first), Some(reload)) = (facts.get(&fact), reload_pos) {
                    if first < reload {
                        cov.undo_repairs += 1;
                        drop_range(&mut kept, &mut saves, &mut facts, first, reload, cov);
                        reload_pos = Some(first);
                    }
                }
                facts.insert(fact, kept.len());
            }
            if let Event::InvaderCardRevealed { card } = e {
                revealed.insert(card.clone());
            }
        }
        kept.push((ms, p));
    }
    kept
}

/// Remove kept[from..to) and shift every index that pointed past it.
fn drop_range(kept: &mut Vec<(Option<u64>, Parsed)>, saves: &mut Vec<(usize, Option<u64>)>, facts: &mut HashMap<String, usize>, from: usize, to: usize, cov: &mut Coverage) {
    let to = to.min(kept.len());
    if from >= to {
        return;
    }
    let dropped = kept[from..to].iter().filter(|(_, p)| matches!(p, Parsed::Event(_) | Parsed::Many(_))).count();
    cov.undo_dropped += dropped as u32;
    kept.drain(from..to);
    let shift = to - from;
    saves.retain(|(pos, _)| *pos < from || *pos >= to);
    for (pos, _) in saves.iter_mut() {
        if *pos >= to {
            *pos -= shift;
        }
    }
    facts.retain(|_, pos| *pos < from || *pos >= to);
    for pos in facts.values_mut() {
        if *pos >= to {
            *pos -= shift;
        }
    }
}

/// A fact the game cannot record twice unless the first time was undone.
fn unique_fact(e: &Event, epoch: u32) -> Option<String> {
    Some(match e {
        Event::PresenceMoved { spirit, node: Some(n), .. } => format!("slot {spirit} {n}"),
        Event::CardGained { card, deck, .. } if !deck.contains("Discard") => format!("gain {card} e{epoch}"),
        Event::CardPlayed { card, .. } => format!("play {card} e{epoch}"),
        Event::CardForgotten { card, .. } => format!("forget {card}"),
        Event::InvaderCardResolved { step, card } => format!("invader {step:?} {card} e{epoch}"),
        Event::InvaderCardRevealed { card } => format!("reveal {card}"),
        Event::FearCardResolved { card } => format!("fear {card}"),
        Event::EventResolved { card } => format!("event {card} e{epoch}"),
        Event::TerrorLevel { level } => format!("terror {level}"),
        _ => return None,
    })
}

impl<'d> Walk<'d> {
    fn spirit_id(&mut self, name: &str) -> String {
        let name = name.strip_suffix("Incarna").unwrap_or(name);
        if let Some(id) = self.spirit_names.get(name) {
            return id.clone();
        }
        let id = self.defs.and_then(|d| d.spirit_by_any_name(name)).map(|s| s.id.clone()).unwrap_or_else(|| name.to_string());
        self.spirit_names.insert(name.to_string(), id.clone());
        if self.spirit_names.values().collect::<BTreeSet<_>>().len() == 1 {
            self.single_spirit = Some(id.clone());
        } else {
            self.single_spirit = None;
        }
        id
    }

    fn spirit_turn(&mut self, spirit: &str) -> &mut SpiritTurn {
        let turn = self.turn;
        if let Some(i) = self.rec.spirits.iter().position(|s| s.spirit == spirit && s.turn == turn) {
            return &mut self.rec.spirits[i];
        }
        let mut st = SpiritTurn { turn, spirit: spirit.to_string(), ..Default::default() };
        st.energy_after = self.energy.get(spirit).copied().unwrap_or(0);
        if self.piles.get(spirit).is_none() {
            let panel = self.defs.and_then(|d| d.spirits.get(spirit));
            let hand = panel.map(|p| p.uniques.clone()).unwrap_or_default();
            self.piles.insert(spirit.to_string(), Piles { hand, ..Default::default() });
            if let Some(p) = panel {
                self.revealed.entry(spirit.to_string()).or_default().extend(p.starting_revealed());
            }
        }
        self.rec.spirits.push(st);
        self.rec.spirits.last_mut().unwrap()
    }

    fn turn_rec(&mut self) -> &mut TurnRecord {
        let turn = self.turn;
        if let Some(i) = self.rec.turns.iter().position(|t| t.turn == turn) {
            return &mut self.rec.turns[i];
        }
        self.rec.turns.push(TurnRecord { turn, terror_after: self.terror, ..Default::default() });
        self.rec.turns.last_mut().unwrap()
    }

    fn ledger(&mut self, land: &LandId) -> &mut LandLedger {
        let defs = self.defs;
        self.rec.lands.entry(land.clone()).or_insert_with(|| LandLedger {
            land: land.clone(),
            terrain: defs.and_then(|d| d.board_def(&land.board)).and_then(|b| b.lands.iter().find(|l| l.id == *land)).map(|l| l.terrain),
            ..Default::default()
        })
    }

    /// Place the printed pieces of each board in play. The log never records
    /// them: Dahan, the setup City and Town, starting Blight and tokens only
    /// show up once something happens to them.
    fn seed_boards(&mut self) {
        if self.seeded {
            return;
        }
        let Some(defs) = self.defs else { return };
        let boards = self.boards.clone();
        for code in boards {
            let Some(b) = defs.board_def(&code) else { continue };
            for l in &b.lands {
                let land = l.id.clone();
                let mut n = 0;
                for (count, kind) in [(l.initial.explorers, PieceKind::Explorer), (l.initial.towns, PieceKind::Town), (l.initial.cities, PieceKind::City), (l.initial.dahan, PieceKind::Dahan)] {
                    for _ in 0..count {
                        n += 1;
                        self.pieces.insert(format!("init:{land}:{n}"), (land.clone(), kind));
                    }
                }
                if l.initial.blight > 0 {
                    *self.blight.entry(land.clone()).or_insert(0) += l.initial.blight as u32;
                }
                for (count, kind) in [(l.initial.beasts, TokenKind::Beast), (l.initial.wilds, TokenKind::Wilds), (l.initial.disease, TokenKind::Disease), (l.initial.strife, TokenKind::Strife), (l.initial.badlands, TokenKind::Badlands)] {
                    if count > 0 {
                        *self.tokens.entry(land.clone()).or_default().entry(kind).or_insert(0) += count as u32;
                    }
                }
                self.seeded = true;
            }
        }
    }

    /// Drop one of the printed (never individually logged) pieces of `kind`
    /// from `land`, when the log refers to a piece id we have not seen.
    fn take_seeded(&mut self, land: &LandId, kind: PieceKind) -> bool {
        let id = self.pieces.iter().find(|(id, (l, k))| id.starts_with("init:") && l == land && *k == kind).map(|(id, _)| id.clone());
        match id {
            Some(id) => {
                self.pieces.remove(&id);
                true
            }
            None => false,
        }
    }

    fn adjust_token(&mut self, land: &LandId, kind: TokenKind, delta: i32) {
        let e = self.tokens.entry(land.clone()).or_default().entry(kind).or_insert(0);
        *e = (*e as i32 + delta).max(0) as u32;
    }

    fn counts_of(&self, land: &LandId) -> Counts {
        let t = |k: TokenKind| self.tokens.get(land).and_then(|m| m.get(&k)).copied().unwrap_or(0);
        let mut c = Counts { blight: self.blight.get(land).copied().unwrap_or(0), presence: self.presence.get(land).map(|m| m.values().sum()).unwrap_or(0), beasts: t(TokenKind::Beast), disease: t(TokenKind::Disease), strife: t(TokenKind::Strife), wilds: t(TokenKind::Wilds), badlands: t(TokenKind::Badlands), ..Default::default() };
        for (l, k) in self.pieces.values() {
            if l == land {
                match k {
                    PieceKind::Explorer => c.explorers += 1,
                    PieceKind::Town => c.towns += 1,
                    PieceKind::City => c.cities += 1,
                    PieceKind::Dahan => c.dahan += 1,
                }
            }
        }
        c
    }

    fn board_counts(&self) -> (Counts, u32) {
        let mut c = Counts { blight: self.blight.values().sum(), ..Default::default() };
        let mut invaded: BTreeSet<&LandId> = BTreeSet::new();
        for (l, k) in self.pieces.values() {
            match k {
                PieceKind::Explorer => c.explorers += 1,
                PieceKind::Town => c.towns += 1,
                PieceKind::City => c.cities += 1,
                PieceKind::Dahan => c.dahan += 1,
            }
            if k.is_invader() {
                invaded.insert(l);
            }
        }
        (c, invaded.len() as u32)
    }

    fn touch(&mut self, land: &LandId, by: &str, spirit: Option<String>, kind: TouchKind) {
        let (turn, phase) = (self.turn, self.phase);
        self.ledger(land).touches.push(Touch { turn, phase, by: by.to_string(), spirit, kind });
    }

    fn attribute_power(&mut self, power: &str) -> Option<String> {
        if let Some(owner) = self.defs.and_then(|d| d.owner_of_power(power)).map(|s| s.id.clone()) {
            return Some(owner);
        }
        if let Some(s) = &self.single_spirit {
            return Some(s.clone());
        }
        let k = key(power);
        self.piles.iter().find(|(_, p)| p.play.iter().chain(&p.hand).any(|c| key(c) == k)).map(|(s, _)| s.clone())
    }

    fn finish_turn(&mut self) {
        if self.turn == 0 {
            return;
        }
        let (counts, invaded) = self.board_counts();
        let terror = self.terror;
        let t = self.turn_rec();
        t.board_after = counts;
        t.lands_with_invaders_after = invaded;
        t.terror_after = terror;
        let spirits: Vec<String> = self.rec.spirits.iter().filter(|s| s.turn == self.turn).map(|s| s.spirit.clone()).collect();
        for sp in spirits {
            let revealed = self.revealed.get(&sp).cloned().unwrap_or_default();
            let state = self.defs.and_then(|d| d.spirits.get(&sp)).map(|p| p.track_state(&revealed));
            let energy = self.energy.get(&sp).copied().unwrap_or(0);
            let (hand, discard) = self.piles.get(&sp).map(|p| (p.hand.clone(), p.discard.clone())).unwrap_or_default();
            let presence: BTreeMap<LandId, u32> = self.presence.iter().filter_map(|(l, m)| m.get(&sp).filter(|n| **n > 0).map(|n| (l.clone(), *n))).collect();
            let st = self.spirit_turn(&sp);
            st.presence_after = presence;
            st.revealed_after = revealed;
            st.income = state.as_ref().map(|s| s.income);
            st.plays = state.as_ref().map(|s| s.plays);
            st.energy_after = energy;
            st.hand_after = hand;
            st.discard_after = discard;
        }
    }

    fn marker(&mut self, m: Marker) {
        match m {
            Marker::GrowthGroup { spirit, group } => {
                let id = self.spirit_id(&spirit);
                let pick = self.defs.and_then(|d| d.spirits.get(&id)).map(|p| p.growth.pick as usize).unwrap_or(1);
                let st = self.spirit_turn(&id);
                // re-selections (after an undo) replace the earlier choice
                if st.growth_groups.len() >= pick {
                    st.growth_groups.remove(0);
                }
                st.growth_groups.push(group);
            }
            Marker::GrowthOption { spirit, option, undo } => {
                let id = self.spirit_id(&spirit);
                let st = self.spirit_turn(&id);
                if undo {
                    if let Some(i) = st.growth_options.iter().rposition(|o| *o == option) {
                        st.growth_options.remove(i);
                    }
                } else {
                    st.growth_options.push(option);
                }
            }
            Marker::PowerPhase { power, phase } => {
                self.power_phase.insert(power, phase);
            }
            Marker::Boards(boards) => {
                if self.boards.is_empty() {
                    self.boards = boards;
                    self.seed_boards();
                }
            }
            Marker::CardChoice { reason, selected, choices } => {
                if self.turn > 0 {
                    let spirit = self.single_spirit.clone().or_else(|| self.rec.spirits.iter().rev().find(|s| s.turn == self.turn).map(|s| s.spirit.clone()));
                    if reason == "GainPower" && !choices.is_empty() {
                        if let Some(id) = &spirit {
                            self.spirit_turn(id).offered = choices.clone();
                        }
                    }
                    let c = Choice { phase: self.phase, kind: "Card".into(), reason, selected, choices, source: String::new(), source_kind: "Spirit".into(), optional: false, spirit, instance: 0 };
                    let t = self.turn_rec();
                    if !t.choices.iter().any(|x| x.kind == c.kind && x.reason == c.reason && x.selected == c.selected && x.choices == c.choices) {
                        t.choices.push(c);
                    }
                }
            }
            Marker::Choice(mut c) => {
                if self.turn > 0 {
                    c.phase = self.phase;
                    c.spirit = self.single_spirit.clone();
                    let t = self.turn_rec();
                    if !t.choices.iter().any(|x| x.kind == c.kind && x.instance == c.instance && x.selected == c.selected && x.source == c.source) {
                        t.choices.push(c);
                    }
                }
            }
            Marker::GameViewStart | Marker::Saving | Marker::LoadedUndo(_) | Marker::LoadedContinue => {}
        }
    }

    fn apply(&mut self, e: Event) {
        match e {
            Event::PhaseChanged { to, .. } => {
                if to == Phase::PlayerSetup {
                    self.rec.meta.from_setup = true;
                }
                if (to == Phase::FastPower || to == Phase::SlowPower) && self.turn > 0 {
                    let lands: BTreeSet<LandId> = self.pieces.values().map(|(l, _)| l.clone()).chain(self.blight.keys().cloned()).chain(self.presence.keys().cloned()).chain(self.tokens.keys().cloned()).collect();
                    let snapshot: BTreeMap<LandId, Counts> = lands.iter().map(|l| (l.clone(), self.counts_of(l))).collect();
                    let t = self.turn_rec();
                    if to == Phase::FastPower {
                        t.before_fast = snapshot;
                    } else {
                        t.before_slow = snapshot;
                    }
                }
                if to.is_invader_step() && !self.invaded && self.turn > 0 {
                    // the state the Invader steps start from
                    let lands: BTreeSet<LandId> = self.pieces.values().map(|(l, _)| l.clone()).chain(self.blight.keys().cloned()).chain(self.presence.keys().cloned()).chain(self.tokens.keys().cloned()).collect();
                    let snapshot: BTreeMap<LandId, Counts> = lands.iter().map(|l| (l.clone(), self.counts_of(l))).collect();
                    let defend = self.defend_this_turn.clone();
                    let t = self.turn_rec();
                    t.before_invaders = snapshot;
                    t.defend = defend;
                }
                if to.is_invader_step() {
                    self.invaded = true;
                }
                if to == Phase::SpiritPhase && (self.turn == 0 || (self.phase == Phase::TimePasses && self.invaded) || (self.phase == Phase::InvadersStartingAction)) {
                    if self.turn > 0 {
                        self.finish_turn();
                    }
                    self.turn += 1;
                    self.invaded = false;
                    self.defend_this_turn.clear();
                    self.turn_rec();
                    if self.turn == 1 {
                        let lands: BTreeSet<LandId> = self.pieces.values().map(|(l, _)| l.clone()).chain(self.blight.keys().cloned()).chain(self.presence.keys().cloned()).collect();
                        self.rec.setup = lands.iter().map(|l| (l.clone(), self.counts_of(l))).collect();
                    }
                }
                if to == Phase::TimePasses {
                    self.finish_turn();
                }
                self.phase = to;
                self.gone_this_step.clear();
                self.step_adds.clear();
                self.step_ravaged.clear();
                self.step_blight.clear();
                self.step_damage.clear();
                self.step_dahan_lost.clear();
                self.step_invaders_destroyed.clear();
            }
            Event::InvaderCardRevealed { card } => {
                self.turn_rec().explore.push(card);
            }
            Event::InvaderCardResolved { step, card } => {
                let lands: Vec<LandId> = match step {
                    InvaderStep::Ravage => self.step_ravaged.clone(),
                    _ => self.step_adds.iter().map(|(l, _)| l.clone()).collect::<BTreeSet<_>>().into_iter().collect(),
                };
                let high_immigration = self.phase == Phase::HighImmigration;
                match step {
                    InvaderStep::Build if high_immigration => self.turn_rec().high_immigration.push(card.clone()),
                    InvaderStep::Build => self.turn_rec().build.push(card.clone()),
                    InvaderStep::Ravage => self.turn_rec().ravage.push(card.clone()),
                    InvaderStep::Explore => {}
                }
                let turn = self.turn;
                for land in lands {
                    let added: Vec<PieceKind> = self.step_adds.iter().filter(|(l, _)| *l == land).map(|(_, k)| *k).collect();
                    let (blight, cascade) = self.step_blight.get(&land).copied().unwrap_or((0, 0));
                    let visit = InvaderVisit {
                        turn,
                        step: Some(step),
                        card: Some(card.clone()),
                        added,
                        blight,
                        cascade,
                        damage: self.step_damage.get(&land).copied().unwrap_or(0),
                        defend: self.defend_this_turn.get(&land).copied().unwrap_or(0),
                        dahan_lost: self.step_dahan_lost.get(&land).copied().unwrap_or(0),
                        invaders_destroyed: self.step_invaders_destroyed.get(&land).copied().unwrap_or(0),
                        after: self.counts_of(&land),
                    };
                    self.ledger(&land).visits.push(visit);
                }
                self.step_adds.clear();
                self.step_ravaged.clear();
                self.step_blight.clear();
                self.step_damage.clear();
                self.step_dahan_lost.clear();
                self.step_invaders_destroyed.clear();
            }
            Event::Ravaged { land, source } => {
                if !self.step_ravaged.contains(&land) {
                    self.step_ravaged.push(land);
                }
                if source.is_some() {
                    self.ravage_source = source;
                }
            }
            Event::Canceled { step, land, source } => {
                if self.turn > 0 {
                    self.turn_rec().canceled.push(si_record_canceled(step, land, source));
                }
            }
            Event::StatusEffect { kind, source, params } => {
                if self.turn > 0 {
                    self.turn_rec().status_effects.push(crate::record::StatusEffect { kind, source, params });
                }
            }
            Event::PiecesAdded { moves } => {
                for m in moves {
                    if let (Some(to), Some(kind)) = (&m.to, m.kind) {
                        self.pieces.insert(m.piece.clone(), (to.clone(), kind));
                        if self.phase.is_invader_step() {
                            self.step_adds.push((to.clone(), kind));
                        }
                        self.ledger(to);
                    } else if let (Some(to), None, Some(tk)) = (&m.to, m.kind, TokenKind::parse(&m.piece)) {
                        self.adjust_token(to, tk, 1);
                    }
                }
            }
            Event::PiecesMoved { moves } => {
                let spirit_phase = matches!(self.phase, Phase::FastPower | Phase::SlowPower | Phase::SpiritPhase);
                let mut out: BTreeMap<LandId, u32> = BTreeMap::new();
                let mut into: BTreeMap<LandId, u32> = BTreeMap::new();
                for m in moves {
                    let known = self.pieces.contains_key(&m.piece);
                    let prev = self.pieces.get(&m.piece).map(|(l, _)| l.clone()).or(m.from.clone());
                    if let (false, Some(from), Some(kind)) = (known, &m.from, m.kind) {
                        // a printed piece the log names for the first time
                        self.take_seeded(from, kind);
                    }
                    match (&m.to, m.kind) {
                        (Some(to), Some(kind)) => {
                            self.pieces.insert(m.piece.clone(), (to.clone(), kind));
                            *into.entry(to.clone()).or_insert(0) += 1;
                            self.ledger(to);
                        }
                        (None, Some(kind)) => {
                            // off the island: in an Invader step this is a
                            // death (the destroy line follows and is skipped)
                            self.pieces.remove(&m.piece);
                            self.gone_this_step.insert(m.piece.clone());
                            if self.phase.is_invader_step() {
                                if let Some(p) = &prev {
                                    if kind == PieceKind::Dahan {
                                        *self.step_dahan_lost.entry(p.clone()).or_insert(0) += 1;
                                    } else {
                                        *self.step_invaders_destroyed.entry(p.clone()).or_insert(0) += 1;
                                    }
                                }
                            }
                        }
                        (to, None) => {
                            if let Some(tk) = TokenKind::parse(&m.piece) {
                                if let Some(from) = &m.from {
                                    self.adjust_token(from, tk, -1);
                                }
                                if let Some(to) = to {
                                    self.adjust_token(to, tk, 1);
                                }
                            }
                        }
                    }
                    if let (Some(p), Some(to)) = (prev, &m.to) {
                        if to != &p {
                            *out.entry(p).or_insert(0) += 1;
                        }
                    }
                }
                if spirit_phase {
                    let lands: BTreeSet<LandId> = out.keys().chain(into.keys()).cloned().collect();
                    for l in lands {
                        let kind = TouchKind::Moved { out: out.get(&l).copied().unwrap_or(0), into: into.get(&l).copied().unwrap_or(0) };
                        self.touch(&l, "move", None, kind);
                    }
                }
            }
            Event::PiecesDestroyed { pieces, source } => {
                let invader_step = self.phase.is_invader_step();
                // a destroyed piece the log never placed: one of the printed
                // ones; its land is the one named in the source, else the
                // only land holding such a piece
                let named: Option<LandId> = source.split("LandTarget: ").nth(1).and_then(|r| r.split(',').next()).and_then(|l| l.trim().parse().ok());
                for (id, _) in pieces {
                    if self.gone_this_step.contains(&id) {
                        continue;
                    }
                    if !self.pieces.contains_key(&id) {
                        if let Some(kind) = PieceKind::parse(&id) {
                            let holders: Vec<LandId> = self.pieces.iter().filter(|(pid, (_, k))| pid.starts_with("init:") && *k == kind).map(|(_, (l, _))| l.clone()).collect::<BTreeSet<_>>().into_iter().collect();
                            let land = named.clone().filter(|l| holders.contains(l)).or_else(|| (holders.len() == 1).then(|| holders[0].clone()));
                            if let Some(land) = land {
                                if let Some(sid) = self.pieces.iter().find(|(pid, (l, k))| pid.starts_with("init:") && *l == land && *k == kind).map(|(pid, _)| pid.clone()) {
                                    if let Some(v) = self.pieces.remove(&sid) {
                                        self.pieces.insert(id.clone(), v);
                                    }
                                }
                            }
                        }
                    }
                    if let Some((land, kind)) = self.pieces.remove(&id) {
                        if invader_step {
                            if kind == PieceKind::Dahan {
                                *self.step_dahan_lost.entry(land).or_insert(0) += 1;
                            } else {
                                *self.step_invaders_destroyed.entry(land).or_insert(0) += 1;
                            }
                        } else if kind.is_invader() {
                            self.touch(&land, &source, None, TouchKind::Damage(1));
                        }
                    }
                }
            }
            Event::TokensAdded { land: Some(land), kind: Some(kind), count } => {
                self.adjust_token(&land, kind, count as i32);
                if matches!(self.phase, Phase::FastPower | Phase::SlowPower | Phase::SpiritPhase) {
                    self.touch(&land, "tokens", None, TouchKind::Moved { out: 0, into: 0 });
                }
            }
            Event::Damage { source, land, amount, .. } => {
                if let Some(land) = land {
                    if self.phase.is_invader_step() {
                        *self.step_damage.entry(land).or_insert(0) += amount;
                    } else {
                        self.touch(&land, &source, None, TouchKind::Damage(amount));
                    }
                }
            }
            Event::Defend { lands, amount, source } => {
                for l in lands {
                    *self.defend_this_turn.entry(l.clone()).or_insert(0) += amount;
                    self.touch(&l, &source, None, TouchKind::Defend(amount));
                }
            }
            Event::BlightAdded { land, amount, cascade } => {
                let (turn, phase) = (self.turn, self.phase);
                if let Some(land) = land {
                    *self.blight.entry(land.clone()).or_insert(0) += amount;
                    self.ledger(&land).blight.push((turn, phase, amount, cascade));
                    if phase.is_invader_step() {
                        let e = self.step_blight.entry(land).or_insert((0, 0));
                        if cascade {
                            e.1 += amount
                        } else {
                            e.0 += amount
                        }
                    }
                }
                let t = self.turn_rec();
                if cascade {
                    t.blight_cascaded += amount
                } else {
                    t.blight_added += amount
                }
            }
            Event::BlightRemoved { land, amount } => {
                if let Some(land) = land {
                    let b = self.blight.entry(land.clone()).or_insert(0);
                    *b = b.saturating_sub(amount);
                    self.touch(&land, "remove blight", None, TouchKind::Recover(amount));
                }
            }
            Event::Fear { amount, .. } => {
                if amount > 0 {
                    self.turn_rec().fear_generated += amount;
                }
            }
            Event::FearCardEarned => {
                self.rec.fear_earned_by_level[(self.terror.clamp(1, 3) - 1) as usize] += 1;
                self.turn_rec().fear_cards_earned += 1;
            }
            Event::FearCardResolved { card } => self.turn_rec().fear_cards.push(card),
            Event::TerrorLevel { level } => {
                self.terror = level;
                self.turn_rec().terror_after = level;
            }
            Event::EventResolved { card } => self.turn_rec().events.push(card),
            Event::PowerUsed { power, target, .. } => {
                let phase = self.power_phase.get(&power).copied().unwrap_or(self.phase);
                let spirit = self.attribute_power(&power);
                if let Some(sp) = &spirit {
                    let st = self.spirit_turn(sp);
                    let entry = (phase, power.clone(), target.clone());
                    if st.powers_used.last() != Some(&entry) {
                        st.powers_used.push(entry);
                    }
                }
                if let Some(land) = target {
                    let (turn, _) = (self.turn, ());
                    self.ledger(&land).touches.push(Touch { turn, phase, by: power, spirit, kind: TouchKind::Power });
                }
            }
            Event::CardPlayed { card, .. } => {
                let spirit = self.attribute_power(&card);
                if let Some(sp) = spirit {
                    let k = key(&card);
                    if let Some(p) = self.piles.get_mut(&sp) {
                        let c = p.take(&k).unwrap_or(card.clone());
                        p.play.push(c);
                    }
                    let st = self.spirit_turn(&sp);
                    if !st.cards_played.iter().any(|c| key(c) == k) {
                        st.cards_played.push(card);
                    }
                }
            }
            Event::CardUnplayed { card, .. } => {
                let spirit = self.attribute_power(&card);
                if let Some(sp) = spirit {
                    let k = key(&card);
                    if let Some(p) = self.piles.get_mut(&sp) {
                        if let Some(c) = remove_key(&mut p.play, &k) {
                            p.hand.push(c);
                        }
                    }
                    let st = self.spirit_turn(&sp);
                    if let Some(i) = st.cards_played.iter().rposition(|c| key(c) == k) {
                        st.cards_played.remove(i);
                    }
                }
            }
            Event::EnergyPaid { spirit, amount } => {
                let id = self.spirit_id(&spirit);
                *self.energy.entry(id.clone()).or_insert(0) -= amount;
                let after = self.energy[&id];
                let st = self.spirit_turn(&id);
                st.energy_paid += amount;
                st.energy_after = after;
            }
            Event::EnergyGained { spirit, amount } => {
                let id = self.spirit_id(&spirit);
                *self.energy.entry(id.clone()).or_insert(0) += amount;
                let after = self.energy[&id];
                let st = self.spirit_turn(&id);
                st.energy_gained += amount;
                st.energy_after = after;
            }
            Event::PresenceMoved { spirit, node, from, to, destroyed } => {
                let id = self.spirit_id(&spirit);
                if let Some(n) = &node {
                    self.revealed.entry(id.clone()).or_default().insert(n.clone());
                }
                if let Some(f) = &from {
                    if let Some(m) = self.presence.get_mut(f) {
                        let c = m.entry(id.clone()).or_insert(0);
                        *c = c.saturating_sub(1);
                    }
                }
                if let Some(t) = &to {
                    *self.presence.entry(t.clone()).or_default().entry(id.clone()).or_insert(0) += 1;
                    self.ledger(t);
                }
                if destroyed {
                    self.spirit_turn(&id).presence_destroyed += 1;
                } else if let (Some(f), Some(t)) = (&from, &to) {
                    if f != t {
                        self.spirit_turn(&id).presence_moved.push((f.clone(), t.clone()));
                        if self.turn > 0 {
                            self.touch(t, "presence move", Some(id.clone()), TouchKind::Presence);
                        }
                    }
                } else if from.is_none() && to.is_some() {
                    // a slot or off-board presence arriving on the island
                    // (a slot-to-card move, e.g. Fractured Days, is not a placement)
                    let placed = (node.clone(), to.clone());
                    self.spirit_turn(&id).presence_placed.push(placed);
                    if let Some(t) = &to {
                        if self.turn > 0 {
                            self.touch(t, "growth", Some(id.clone()), TouchKind::Presence);
                        }
                    }
                }
            }
            Event::CardGained { spirit, card, deck } => {
                let sp = spirit.map(|s| self.spirit_id(&s)).or_else(|| self.single_spirit.clone());
                if let Some(sp) = sp {
                    let from_discard = deck.contains("Discard");
                    let k = key(&card);
                    if let Some(p) = self.piles.get_mut(&sp) {
                        let c = p.take(&k).unwrap_or(card.clone());
                        p.hand.push(c);
                    }
                    let st = self.spirit_turn(&sp);
                    if from_discard {
                        st.reclaimed = true;
                        if !st.reclaimed_cards.iter().any(|c| key(c) == k) {
                            st.reclaimed_cards.push(card);
                        }
                    } else if !st.cards_gained.iter().any(|c| key(c) == k) {
                        st.cards_gained.push(card);
                    }
                }
            }
            Event::CardsDiscarded { spirit, cards } => {
                let sp = spirit.map(|s| self.spirit_id(&s)).or_else(|| self.single_spirit.clone());
                if let Some(sp) = sp {
                    if let Some(p) = self.piles.get_mut(&sp) {
                        for c in cards {
                            let k = key(&c);
                            let c = p.take(&k).unwrap_or(c);
                            p.discard.push(c);
                        }
                    }
                }
            }
            Event::CardForgotten { spirit, card } => {
                let sp = spirit.map(|s| self.spirit_id(&s)).or_else(|| self.attribute_power(&card)).or_else(|| self.single_spirit.clone());
                if let Some(sp) = sp {
                    let k = key(&card);
                    if let Some(p) = self.piles.get_mut(&sp) {
                        p.take(&k);
                    }
                    let st = self.spirit_turn(&sp);
                    if !st.cards_forgotten.iter().any(|c| key(c) == k) {
                        st.cards_forgotten.push(card);
                    }
                }
            }
            Event::GameOver { ending } => {
                self.rec.meta.ending = Some(ending);
                // a game that ends during the Ravage never logs ResolveInvaderCardAction for
                // that card: take it from the Ravage lines, so the turn's Ravage is on record
                if self.phase == Phase::Ravage && self.turn > 0 && self.turn_rec().ravage.is_empty() {
                    if let Some(id) = self.ravage_source.as_deref().and_then(invader_card_id_from_title) {
                        self.turn_rec().ravage.push(id);
                    }
                }
            }
            Event::TimePassed => {}
            Event::SlotsAdvanced { .. }
            | Event::ElementsGained { .. }
            | Event::PlaysAdjusted { .. }
            | Event::GrowthChosen { .. }
            | Event::GrowthOptionTaken { .. }
            | Event::TokensAdded { .. }
            | Event::StatusAdded { .. }
            | Event::EventRevealed { .. }
            | Event::Undo { .. }
            | Event::Setup { .. }
            | Event::Unparsed { .. } => {}
        }
    }
}

/// "Stage III Mountain+Jungle" → "MountainJungle3", "Stage II Coastal" → "Coastal2": the
/// engine's card ids are the terrains joined, then the stage digit.
fn invader_card_id_from_title(title: &str) -> Option<String> {
    let rest = title.strip_prefix("Stage ")?;
    let (stage, terrains) = rest.split_once(' ')?;
    let digit = match stage {
        "I" => '1',
        "II" => '2',
        "III" => '3',
        _ => return None,
    };
    let joined: String = terrains.split('+').map(|t| t.trim()).collect::<Vec<_>>().join("");
    if joined.is_empty() {
        return None;
    }
    Some(format!("{joined}{digit}"))
}
