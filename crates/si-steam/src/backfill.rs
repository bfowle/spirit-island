//! Backfill: the turns before the companion joined a game, rebuilt from the
//! game's own `log.txt`. The log holds the whole session (every action the
//! engine performed, with a "GameView Start" line per game, or per rewrite
//! of the log from the game's journal), and the live recorder reads all of
//! it when it starts, so a late join still has the current game's log from
//! its first turn. No save exists for those turns, so each is rebuilt
//! backwards from the first save seen (the anchor): the anchor's counts,
//! less what the log added and plus what it removed after that turn. Only
//! public facts are read (pieces, Blight, Fear, Presence, Energy, cards
//! flipped face up); the whitelist is in `parse_log`'s spirit.
//!
//! Undo: a checkpoint `save-Undo-k.dat` is written the moment a decision is
//! made and `LOADED FILE: save-Undo-k.dat` reverts to it; the actions logged
//! in between are dropped, as `parse_log` does, when their lines carry real
//! times (lines read in one bulk read at startup all share that moment, so
//! an undo made before the companion joined cannot be told apart).

use si_common::companion_stats::{RoundSummary, SpiritRow};
use si_common::public::{PublicInvaderCard, PublicSnapshot};
use std::collections::BTreeMap;

const CHECKPOINT_LAG_MS: u64 = 2000;

/// One public fact the log states.
#[derive(Debug, Clone, PartialEq)]
enum Fact {
    /// A piece of this kind added to a land; `true` when an Invader card
    /// placed it (an Explore or a Build), `false` for a rule or a Power.
    Added(String, String, bool),
    Removed(String),
    /// Blight onto the island: amount, land, from a Ravage and not a cascade.
    Blight(i64, String, bool),
    BlightRemoved,
    /// Blight put on the card from the box: the card flipped.
    CardFlipped(i64),
    Fear(i64),
    FearCardEarned,
    TerrorStep,
    HandIn(String),
    HandOut(String),
    /// Presence placed from a track node (spirit, node id).
    PresencePlaced(String, String),
    PresenceDestroyed(String),
    EnergyGained(String, i64),
    EnergyPaid(String, i64),
    ExploreCard(String),
    /// A land Ravaged (Blight or not).
    Ravaged(String),
}

#[derive(Debug, Clone)]
struct Event {
    ms: u64,
    turn: u32,
    step: String,
    fact: Fact,
}

/// One turn's deltas, folded from its events.
#[derive(Debug, Default, Clone)]
struct TurnDelta {
    added: BTreeMap<String, i64>,
    removed: BTreeMap<String, i64>,
    blight_added: i64,
    blight_removed: i64,
    card_flipped: i64,
    fear_generated: i64,
    fear_cards_earned: i64,
    terror_steps: i64,
    presence_placed: BTreeMap<String, i64>,
    presence_destroyed: BTreeMap<String, i64>,
    nodes_revealed: Vec<String>,
    energy_gained: BTreeMap<String, i64>,
    energy_paid: BTreeMap<String, i64>,
    hand_in: BTreeMap<String, i64>,
    hand_out: BTreeMap<String, i64>,
    explored: Vec<String>,
    built: Vec<String>,
    blighted: Vec<String>,
    ravaged: Vec<String>,
    explore_card: Option<String>,
}

fn stamp(line: &str) -> Option<(u64, &str)> {
    let (t, rest) = line.split_once("  ")?;
    let (hms, ms) = t.split_once('.')?;
    let mut parts = hms.split(':').map(|p| p.parse::<u64>().ok());
    let (h, m, s) = (parts.next()??, parts.next()??, parts.next()??);
    Some((((h * 60 + m) * 60 + s) * 1000 + ms.parse::<u64>().ok()?, rest.trim_start()))
}

fn between<'a>(s: &'a str, start: &str, end: &[&str]) -> Option<&'a str> {
    let i = s.find(start)? + start.len();
    let rest = &s[i..];
    let j = end.iter().filter_map(|e| rest.find(e)).min().unwrap_or(rest.len());
    Some(rest[..j].trim())
}

fn is_land(s: &str) -> bool {
    let t = s.replace(' ', "");
    ["Jungle", "Mountain", "Sands", "Wetland", "Ocean"].iter().any(|p| t.starts_with(p)) && t.chars().last().is_some_and(|c| c.is_ascii_digit())
}

/// "Explorer3" → "Explorer".
fn kind(name: &str) -> String {
    name.trim().trim_end_matches(|c: char| c.is_ascii_digit()).to_string()
}

/// `PieceMovements=[A ( X => Y), B ( Z => W)]` → [(A, X, Y), (B, Z, W)].
fn movements(text: &str) -> Vec<(String, String, String)> {
    let Some(list) = between(text, "PieceMovements=[", &["]"]) else { return Vec::new() };
    list.split("),")
        .filter_map(|m| {
            let (name, rest) = m.split_once('(')?;
            let (from, to) = rest.split_once("=>")?;
            Some((name.trim().to_string(), from.trim().to_string(), to.trim().trim_end_matches(')').trim().to_string()))
        })
        .collect()
}

/// The spirit a track node or an out-of-game location belongs to, by id prefix.
fn spirit_of<'a>(s: &'a PublicSnapshot, place: &str) -> Option<&'a str> {
    let p = place.replace(' ', "");
    s.spirits.iter().map(|sp| sp.id.as_str()).filter(|id| p.starts_with(id)).max_by_key(|id| id.len())
}

/// The facts one log line states, if any.
fn facts(action: &str, step: &str, anchor: &PublicSnapshot) -> Vec<Fact> {
    let mut out = Vec::new();
    let energy_amount = |t: &str| between(t, "Amount:", &[","]).and_then(|a| a.parse::<i64>().ok()).unwrap_or(0);
    if action.starts_with("AddIslandPiecesAction:") {
        for (name, _, to) in movements(action) {
            let k = kind(&name);
            if matches!(k.as_str(), "Explorer" | "Town" | "City" | "Dahan") {
                out.push(Fact::Added(k, to.replace(' ', ""), true));
            }
        }
    } else if action.starts_with("MovePiecesAction:") {
        for (name, from, to) in movements(action) {
            let k = kind(&name);
            let gone = to.replace(' ', "");
            let piece = matches!(k.as_str(), "Explorer" | "Town" | "City" | "Dahan");
            if gone == "OutOfGame" {
                match k.as_str() {
                    "Explorer" | "Town" | "City" | "Dahan" => out.push(Fact::Removed(k)),
                    "Blight" => out.push(Fact::BlightRemoved),
                    _ => {}
                }
            } else if piece && !is_land(&from) && is_land(&to) {
                // Onto the island from outside it: an upgrade or a rule's
                // placement (Habsburg's Mining Boom, Events).
                out.push(Fact::Added(k, gone, false));
            } else if k == "Blight" && gone.starts_with("Blight") {
                // Back onto the card: a Power removed it; from the box onto
                // the card: the card flipped.
                out.push(if is_land(&from) { Fact::BlightRemoved } else { Fact::CardFlipped(1) });
            }
        }
    } else if action.starts_with("AddBlightAction:") {
        let amount = between(action, "AmountOfBlight:", &["]"]).and_then(|a| a.parse::<i64>().ok()).unwrap_or(1);
        let land = between(action, "Destination:", &[","]).map(|l| l.replace(' ', "")).unwrap_or_default();
        // A Build-step Ravage is Habsburg Mining's Ceaseless Mining.
        out.push(Fact::Blight(amount, land, matches!(step, "Ravage" | "Build") && action.contains("IsFromCascade: False")));
    } else if action.starts_with("AdjustFearGeneratedAction:") {
        let amount = between(action, "AmountOfFearBeingGenerated=", &[","]).and_then(|a| a.parse::<i64>().ok()).unwrap_or(0);
        if amount > 0 {
            out.push(Fact::Fear(amount));
        }
    } else if action.starts_with("MoveCardAction:") {
        if action.contains("To=Fear Earned, From=Fear Deck") {
            out.push(Fact::FearCardEarned);
        }
        if let Some(card) = between(action, "CardBeingMoved=", &[","]) {
            if (card == "TerrorLevelII" || card == "TerrorLevelIII") && action.contains("To=Out of Game") {
                out.push(Fact::TerrorStep);
            }
        }
        if let Some(sp) = between(action, "To=", &[","]).and_then(|to| to.strip_suffix("'s Hand")).and_then(|who| anchor.spirits.iter().find(|s| s.title == who)) {
            out.push(Fact::HandIn(sp.id.clone()));
        }
        if let Some(sp) = between(action, "From=", &[","]).and_then(|from| from.strip_suffix("'s Hand")).and_then(|who| anchor.spirits.iter().find(|s| s.title == who)) {
            out.push(Fact::HandOut(sp.id.clone()));
        }
    } else if action.starts_with("MovePresenceAction:") {
        for (_, from, to) in movements(action) {
            let (from_land, to_land) = (is_land(&from), is_land(&to));
            if !from_land && to_land {
                if let Some(sp) = spirit_of(anchor, &from) {
                    out.push(Fact::PresencePlaced(sp.to_string(), from.replace(' ', "")));
                }
            } else if from_land && !to_land {
                if let Some(sp) = spirit_of(anchor, &to) {
                    out.push(Fact::PresenceDestroyed(sp.to_string()));
                }
            }
        }
    } else if action.starts_with("GainEnergyAction:") {
        if let Some(sp) = between(action, "SpiritGainingEnergy:", &[" "]).and_then(|id| anchor.spirits.iter().find(|s| s.id == id)) {
            out.push(Fact::EnergyGained(sp.id.clone(), energy_amount(action)));
        }
    } else if action.starts_with("PayEnergyAction:") {
        if let Some(sp) = between(action, "SpiritPayingEnergy:", &[" "]).and_then(|id| anchor.spirits.iter().find(|s| s.id == id)) {
            out.push(Fact::EnergyPaid(sp.id.clone(), energy_amount(action)));
        }
    } else if action.starts_with("RavageAction: LandTarget:") {
        if let Some(land) = between(action, "LandTarget:", &[","]) {
            out.push(Fact::Ravaged(land.replace(' ', "")));
        }
    } else if action.starts_with("FlipCardAction: Source=ResolveInvaderCardAction") {
        if let Some(card) = between(action, "CardBeingFlipped=", &[",", "]"]) {
            out.push(Fact::ExploreCard(card.to_string()));
        }
    }
    out
}

/// The current game's lines, with the best time each has: the log may hold
/// the game more than once (the game rewrites `log.txt` from its journal and
/// the recorder re-reads it whole, stamping every line with that moment);
/// the last copy is complete, and an earlier copy's real times are carried
/// over onto the lines it shares.
fn game_lines(log: &str, anchor_ms: u64) -> Vec<(u64, String)> {
    let lines: Vec<(u64, &str)> = log.lines().filter_map(stamp).filter(|(ms, _)| *ms <= anchor_ms).collect();
    let starts: Vec<usize> = lines.iter().enumerate().filter(|(_, (_, t))| t.starts_with("GameView Start")).map(|(i, _)| i).collect();
    let Some(&last) = starts.last() else { return Vec::new() };
    let mut out: Vec<(u64, String)> = lines[last + 1..].iter().map(|(ms, t)| (*ms, t.to_string())).collect();
    // Earlier copies of the same game: the same first lines (each copy runs
    // to the next "GameView Start").
    let segment = |a: usize| {
        let end = starts.iter().copied().find(|b| *b > a).unwrap_or(lines.len());
        &lines[a + 1..end]
    };
    let same_game = |a: usize| {
        let (x, y) = (segment(a), &lines[last + 1..]);
        let n = x.len().min(y.len()).min(12);
        n > 0 && x[..n].iter().zip(&y[..n]).all(|(p, q)| p.1 == q.1)
    };
    // Every earlier copy lends its times to the lines it shares (each copy
    // was followed by live lines until the next rewrite, so between them
    // they time all but the lines read in bulk at the recorder's start).
    // The copies are aligned line by line, skipping what one has and the
    // other lacks (a rewrite from the journal drops some bookkeeping lines).
    for &earlier in starts.iter().filter(|a| **a < last && same_game(**a)) {
        let seg = segment(earlier);
        let mut j = 0;
        for o in out.iter_mut() {
            let window = &seg[j..seg.len().min(j + 80)];
            if let Some(off) = window.iter().position(|(_, t)| *t == o.1) {
                o.0 = o.0.min(window[off].0);
                j += off + 1;
            }
        }
    }
    out
}

/// Lines read in one bulk read (the recorder's start, or a re-read after the
/// game rewrote its log) all carry that moment as their stamp: a run of
/// many lines with one stamp. Such lines have no time of their own.
fn bulk_lines(lines: &[(u64, String)]) -> Vec<bool> {
    let mut out = vec![false; lines.len()];
    let mut i = 0;
    while i < lines.len() {
        let mut j = i;
        while j < lines.len() && lines[j].0 == lines[i].0 {
            j += 1;
        }
        if j - i >= 30 {
            for b in &mut out[i..j] {
                *b = true;
            }
        }
        i = j;
    }
    out
}

/// The turns before `anchor` (the first snapshot the companion has, at
/// turn N), rebuilt from `log` (the capture's `log-stream.txt`) for turns
/// 1..N. `anchor_ms` is the anchor save's wall clock; `undo_writes` the undo
/// checkpoints' write times (`log_parse::undo_write_times`). Empty when the
/// log does not hold this game from its start.
pub fn rows(log: &str, anchor: &PublicSnapshot, anchor_ms: u64, undo_writes: &BTreeMap<String, Vec<u64>>) -> Vec<RoundSummary> {
    let n = anchor.turn;
    if n <= 1 {
        return Vec::new();
    }
    let lines = game_lines(log, anchor_ms);
    if lines.is_empty() {
        return Vec::new();
    }

    // Turns count as the saves do: Time Passes into the Spirit Phase starts
    // the next one, once per Invader Phase played (an undo checkpoint reload
    // logs that transition again without an Invader Phase between).
    let mut events: Vec<Event> = Vec::new();
    let mut steps: Vec<(u64, u32, String, bool)> = Vec::new();
    let mut turn = 1u32;
    let mut step = "PlayerSetup".to_string();
    let mut invaded = false;
    let bulk = bulk_lines(&lines);
    for (i, (ms, text)) in lines.iter().enumerate() {
        if let Some(file) = between(text, "LOADED FILE: ", &[" *****"]) {
            // An undo on a line with no time of its own cannot be placed.
            if bulk[i] {
                continue;
            }
            if let Some(&t) = undo_writes.get(file).and_then(|ts| ts.iter().filter(|t| **t <= *ms).max()) {
                let cut = events.iter().rev().find(|e| e.ms <= t && t - e.ms <= CHECKPOINT_LAG_MS).map(|e| e.ms).unwrap_or_else(|| t.saturating_sub(50));
                if std::env::var_os("SI_BACKFILL_DEBUG").is_some() {
                    let dropped = events.iter().filter(|e| e.ms >= cut).count();
                    eprintln!("undo {file} at {ms}: checkpoint {t}, cut {cut}, dropping {dropped} of {} (turn {turn} {step})", events.len());
                }
                events.retain(|e| e.ms < cut);
                steps.retain(|(at, ..)| *at < cut);
                if let Some((_, tn, st, inv)) = steps.last() {
                    turn = *tn;
                    step = st.clone();
                    invaded = *inv;
                }
            }
            continue;
        }
        if let Some(change) = text.strip_prefix("DidPerformAction: [ChangeStateAction: ") {
            if change.starts_with("From=TimePasses/TimePasses, To=SpiritPhase/SpiritPhase") && invaded {
                turn += 1;
                invaded = false;
                if std::env::var_os("SI_BACKFILL_DEBUG").is_some() {
                    eprintln!("turn {turn} starts at {ms}");
                }
            }
            if let Some(to) = between(change, "To=", &["/"]) {
                step = to.to_string();
                if matches!(step.as_str(), "Ravage" | "Build" | "Explore") {
                    invaded = true;
                }
            }
            steps.push((*ms, turn, step.clone(), invaded));
            continue;
        }
        let Some(action) = text.strip_prefix("DidPerformAction: [") else { continue };
        for fact in facts(action, &step, anchor) {
            events.push(Event { ms: *ms, turn, step: step.clone(), fact });
        }
    }

    let mut deltas: BTreeMap<u32, TurnDelta> = BTreeMap::new();
    for e in &events {
        let d = deltas.entry(e.turn).or_default();
        match &e.fact {
            Fact::Added(k, land, by_card) => {
                *d.added.entry(k.clone()).or_default() += 1;
                if !by_card {
                } else if e.step == "Explore" && k == "Explorer" && !d.explored.contains(land) {
                    d.explored.push(land.clone());
                } else if e.step == "Build" && (k == "Town" || k == "City") && !d.built.contains(land) {
                    d.built.push(land.clone());
                }
            }
            Fact::Removed(k) => *d.removed.entry(k.clone()).or_default() += 1,
            Fact::Blight(amount, land, ravage) => {
                d.blight_added += amount;
                if *ravage && !d.blighted.contains(land) {
                    d.blighted.push(land.clone());
                }
            }
            Fact::BlightRemoved => d.blight_removed += 1,
            Fact::CardFlipped(k) => d.card_flipped += k,
            Fact::Fear(k) => d.fear_generated += k,
            Fact::FearCardEarned => d.fear_cards_earned += 1,
            Fact::TerrorStep => d.terror_steps += 1,
            Fact::HandIn(sp) => *d.hand_in.entry(sp.clone()).or_default() += 1,
            Fact::HandOut(sp) => *d.hand_out.entry(sp.clone()).or_default() += 1,
            Fact::PresencePlaced(sp, node) => {
                *d.presence_placed.entry(sp.clone()).or_default() += 1;
                d.nodes_revealed.push(node.clone());
            }
            Fact::PresenceDestroyed(sp) => *d.presence_destroyed.entry(sp.clone()).or_default() += 1,
            Fact::EnergyGained(sp, k) => *d.energy_gained.entry(sp.clone()).or_default() += k,
            Fact::EnergyPaid(sp, k) => *d.energy_paid.entry(sp.clone()).or_default() += k,
            Fact::ExploreCard(id) => d.explore_card = Some(id.clone()),
            Fact::Ravaged(land) => {
                if !d.ravaged.contains(land) {
                    d.ravaged.push(land.clone());
                }
            }
        }
    }
    if deltas.is_empty() {
        return Vec::new();
    }

    // Blight on the card, forward from the healthy side (2 per board + 1),
    // then from the flipped side's count once the log shows the flip.
    let boards = anchor.boards.len().max(1) as i64;
    let healthy_total = 2 * boards + 1;
    let card_by: BTreeMap<u32, (i64, bool)> = {
        let mut left = healthy_total;
        let mut flipped = false;
        deltas
            .iter()
            .map(|(t, d)| {
                if d.card_flipped > 0 {
                    left += d.card_flipped;
                    flipped = true;
                }
                left -= d.blight_added;
                (*t, (left, flipped))
            })
            .collect()
    };
    let pool = |id: &str| anchor.invaders.pool.iter().find(|c| c.id == id).cloned();
    let explore_cards: BTreeMap<u32, String> = deltas.iter().filter_map(|(t, d)| d.explore_card.clone().map(|c| (*t, c))).collect();
    let island = |f: fn(&si_common::public::PublicLand) -> u32| anchor.lands.iter().filter(|l| !l.is_ocean).map(f).sum::<u32>() as i64;
    let get = |m: &BTreeMap<String, i64>, k: &str| m.get(k).copied().unwrap_or(0);

    // Each turn's end state: the anchor, less every later turn's deltas.
    let mut out: Vec<RoundSummary> = Vec::new();
    for t in 1..n {
        let mut explorers = island(|l| l.explorers);
        let mut towns = island(|l| l.towns);
        let mut cities = island(|l| l.cities);
        let mut dahan = island(|l| l.dahan);
        let mut blight_on_island = anchor.blight.on_island as i64;
        let mut fear_total = (anchor.fear.earned_count as i64 + anchor.fear.discard.len() as i64) * anchor.fear.required_per_card + anchor.fear.generated;
        let mut fear_cards = anchor.fear.earned_count as i64 + anchor.fear.discard.len() as i64;
        let mut terror = anchor.fear.terror_level as i64;
        let mut presence: BTreeMap<String, (i64, i64)> = anchor.spirits.iter().map(|s| (s.id.clone(), (s.presence_on_island as i64, s.presence_destroyed as i64))).collect();
        let mut energy: BTreeMap<String, i64> = anchor.spirits.iter().map(|s| (s.id.clone(), s.energy)).collect();
        let mut hand: BTreeMap<String, i64> = anchor.spirits.iter().map(|s| (s.id.clone(), s.hand.len() as i64)).collect();
        let mut revealed: BTreeMap<String, Vec<String>> =
            anchor.spirits.iter().map(|s| (s.id.clone(), s.track.iter().filter(|x| x.revealed).map(|x| x.id.clone()).collect())).collect();
        for d in deltas.iter().filter(|(u, _)| **u > t).map(|(_, d)| d) {
            explorers += get(&d.removed, "Explorer") - get(&d.added, "Explorer");
            towns += get(&d.removed, "Town") - get(&d.added, "Town");
            cities += get(&d.removed, "City") - get(&d.added, "City");
            dahan += get(&d.removed, "Dahan") - get(&d.added, "Dahan");
            blight_on_island += d.blight_removed - d.blight_added;
            fear_total -= d.fear_generated;
            fear_cards -= d.fear_cards_earned;
            terror -= d.terror_steps;
            for (sp, v) in presence.iter_mut() {
                v.0 += get(&d.presence_destroyed, sp) - get(&d.presence_placed, sp);
                v.1 -= get(&d.presence_destroyed, sp);
            }
            for (sp, e) in energy.iter_mut() {
                *e += get(&d.energy_paid, sp) - get(&d.energy_gained, sp);
            }
            for (sp, h) in hand.iter_mut() {
                *h += get(&d.hand_out, sp) - get(&d.hand_in, sp);
            }
            for nodes in revealed.values_mut() {
                nodes.retain(|id| !d.nodes_revealed.contains(id));
            }
        }
        let d = deltas.get(&t).cloned().unwrap_or_default();
        let (blight_remaining, blight_flipped) = card_by.range(..=t).next_back().map(|(_, v)| *v).unwrap_or((healthy_total, false));
        let spirits: Vec<SpiritRow> = anchor
            .spirits
            .iter()
            .map(|s| {
                let nodes = revealed.get(&s.id).cloned().unwrap_or_default();
                let best = |k: &str| s.track.iter().filter(|x| x.kind == k && nodes.contains(&x.id)).filter_map(|x| x.value).max().unwrap_or(0);
                SpiritRow {
                    id: s.id.clone(),
                    energy_income: best("EnergyPerTurn"),
                    plays: best("CardPlaysPerTurn"),
                    energy: energy.get(&s.id).copied().unwrap_or(0).max(0),
                    hand: hand.get(&s.id).copied().unwrap_or(0).max(0) as u32,
                    presence: presence.get(&s.id).map(|v| v.0).unwrap_or(0).max(0) as u32,
                    track_elements: s.track.iter().filter(|x| x.kind.contains("Element") && nodes.contains(&x.id)).count() as u32,
                }
            })
            .collect();
        let card_at = |u: i64| if u < 0 { None } else { explore_cards.get(&(u as u32)).and_then(|id| pool(id)) };
        let explored: Vec<PublicInvaderCard> = card_at(t as i64).into_iter().collect();
        let built: Vec<PublicInvaderCard> = card_at(t as i64 - 1).into_iter().collect();
        let ravaged: Vec<PublicInvaderCard> = card_at(t as i64 - 2).into_iter().collect();
        out.push(RoundSummary {
            turn: t,
            last_step: "TimePasses".into(),
            fear_total: fear_total.max(0),
            fear_cards_earned: fear_cards.max(0) as u32,
            terror_level: terror.clamp(1, 3) as u8,
            blight_on_island: blight_on_island.max(0) as u32,
            blight_remaining: blight_remaining.max(0),
            blight_flipped,
            explorers: explorers.max(0) as u32,
            towns: towns.max(0) as u32,
            cities: cities.max(0) as u32,
            dahan: dahan.max(0) as u32,
            invaded_lands: 0,
            presence_on_island: spirits.iter().map(|s| s.presence).sum(),
            presence_destroyed: presence.values().map(|v| v.1.max(0) as u32).sum(),
            energy: spirits.iter().map(|s| s.energy).sum(),
            cards_in_hand: spirits.iter().map(|s| s.hand).sum(),
            snapshots: 0,
            energy_income: spirits.iter().map(|s| s.energy_income).sum(),
            plays: spirits.iter().map(|s| s.plays).sum(),
            ravaged,
            built,
            explored,
            forecast_start: [0.0; 3],
            actual: [d.explored.len() as f64, d.built.len() as f64, d.ravaged.len() as f64],
            blighting: [0.0, d.blighted.len() as f64, 0.0],
            forecast_next: [0.0; 3],
            spirits,
            from_log: true,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movements_split() {
        let m = movements("MovePiecesAction: PieceMovements=[Town5 ( Jungle B4 => OutOfGame), Explorer5 ( Jungle B4 => OutOfGame)]");
        assert_eq!(m, vec![("Town5".to_string(), "Jungle B4".to_string(), "OutOfGame".to_string()), ("Explorer5".to_string(), "Jungle B4".to_string(), "OutOfGame".to_string())]);
        assert_eq!(kind("Explorer12"), "Explorer");
    }

    #[test]
    fn an_earlier_copy_lends_its_times() {
        let log = "01:00:00.000  GameView Start\n01:00:01.000  A\n01:00:02.000  B\n02:00:00.000  GameView Start\n02:00:00.000  A\n02:00:00.000  B\n02:00:05.000  C\n";
        let lines = game_lines(log, 9_000_000);
        let times: Vec<u64> = lines.iter().map(|(ms, _)| *ms).collect();
        assert_eq!(times, vec![3_600_000 + 1000, 3_600_000 + 2000, 7_200_000 + 5000]);
    }
}
