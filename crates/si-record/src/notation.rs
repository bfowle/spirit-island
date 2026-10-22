//! Spirit Island notation (SIN): a recorded game as moves, reveals and
//! results. Spec: docs/superpowers/specs/2026-10-20-notation.md. Two forms:
//! display (short codes, one line per phase) and canonical (engine ids).

use std::collections::BTreeMap;

use si_defs::Definitions;
use si_model::game::Phase;
use si_model::island::PieceKind;

use crate::record::{GameRecord, SpiritTurn, TouchKind};

const STOPWORDS: [&str; 12] = ["of", "the", "a", "an", "and", "to", "with", "into", "for", "in", "on", "from"];

/// Short codes for cards and innates: title initials with stopwords dropped,
/// a digit on collisions, a legend of the codes used.
pub struct Codes<'d> {
    defs: Option<&'d Definitions>,
    canonical: bool,
    codes: BTreeMap<String, String>,
    used: BTreeMap<String, String>,
}

impl<'d> Codes<'d> {
    pub fn new(defs: Option<&'d Definitions>, canonical: bool) -> Self {
        let mut titles: Vec<(String, String)> = Vec::new();
        if let Some(d) = defs {
            titles.extend(d.cards.values().map(|c| (c.id.clone(), c.title.clone())));
            titles.extend(d.spirits.values().flat_map(|s| s.innates.iter().map(|i| (i.id.clone(), i.title.clone()))));
        }
        titles.sort();
        let mut codes: BTreeMap<String, String> = BTreeMap::new();
        let mut taken: BTreeMap<String, u32> = BTreeMap::new();
        for (id, title) in titles {
            let base = code_of(&title);
            let n = taken.entry(base.clone()).or_insert(0);
            *n += 1;
            let code = if *n == 1 { base } else { format!("{base}{n}") };
            codes.insert(id, code);
        }
        Codes { defs, canonical, codes, used: BTreeMap::new() }
    }

    /// The code for a card or innate id (the id itself in canonical form or
    /// when the definitions do not know it).
    pub fn code(&mut self, id: &str) -> String {
        if self.canonical {
            return id.to_string();
        }
        let code = self.codes.get(id).cloned().unwrap_or_else(|| code_of(&split_camel(id)));
        let title = self.defs.and_then(|d| d.cards.get(id).map(|c| c.title.clone()).or_else(|| d.spirits.values().flat_map(|s| s.innates.iter()).find(|i| i.id == id).map(|i| i.title.clone()))).unwrap_or_else(|| split_camel(id));
        self.used.insert(code.clone(), title);
        code
    }

    pub fn legend(&self) -> String {
        self.used.iter().map(|(c, t)| format!("{c} = {t}")).collect::<Vec<_>>().join(", ")
    }

    /// The codes used so far with their titles, for a table legend.
    pub fn entries(&self) -> Vec<(String, String)> {
        self.used.iter().map(|(c, t)| (c.clone(), t.clone())).collect()
    }
}

fn code_of(title: &str) -> String {
    let words: Vec<&str> = title.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty() && !STOPWORDS.contains(&w.to_ascii_lowercase().as_str())).collect();
    if words.len() == 1 {
        return words[0].chars().take(4).collect::<String>().to_uppercase();
    }
    words.iter().filter_map(|w| w.chars().next()).collect::<String>().to_uppercase()
}

fn split_camel(id: &str) -> String {
    let mut out = String::new();
    for (i, c) in id.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

fn slot_code(slot: &str, canonical: bool) -> String {
    if canonical {
        return slot.to_string();
    }
    let digits: String = slot.chars().rev().take_while(|c| c.is_ascii_digit()).collect::<Vec<_>>().into_iter().rev().collect();
    if slot.contains("Energy") {
        format!("e{digits}")
    } else if slot.contains("CardPlays") {
        format!("cp{digits}")
    } else {
        slot.to_string()
    }
}

fn piece_code(k: PieceKind) -> &'static str {
    match k {
        PieceKind::Explorer => "E",
        PieceKind::Town => "T",
        PieceKind::City => "C",
        PieceKind::Dahan => "D",
    }
}

fn added(v: &[PieceKind]) -> String {
    let mut m: BTreeMap<&str, u32> = BTreeMap::new();
    for k in v {
        *m.entry(piece_code(*k)).or_insert(0) += 1;
    }
    m.iter().map(|(k, n)| if *n == 1 { format!("+{k}") } else { format!("+{n}{k}") }).collect::<Vec<_>>().join("")
}

fn growth_line(s: &SpiritTurn, codes: &mut Codes, canonical: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    if !s.growth_groups.is_empty() {
        parts.push(s.growth_groups.iter().map(|g| format!("G{g}")).collect::<Vec<_>>().join("+"));
    }
    if s.reclaimed {
        parts.push("Rc".into());
    }
    for (slot, land) in &s.presence_placed {
        let at = land.as_ref().map(|l| format!("@{l}")).unwrap_or_default();
        match slot {
            Some(id) => parts.push(format!("{}{at}", slot_code(id, canonical))),
            None => parts.push(format!("+Pr{at}")),
        }
    }
    for (from, to) in &s.presence_moved {
        parts.push(format!("Pr {from}→{to}"));
    }
    for c in &s.cards_gained {
        parts.push(format!("+{}", codes.code(c)));
    }
    for c in &s.cards_forgotten {
        parts.push(format!("−{}", codes.code(c)));
    }
    if s.presence_destroyed > 0 && s.growth_groups.is_empty() {
        parts.push(format!("Pr†×{}", s.presence_destroyed));
    }
    parts.join(" ")
}

fn power_line(rec: &GameRecord, s: &SpiritTurn, phase: Phase, codes: &mut Codes) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut seen_lands: Vec<String> = Vec::new();
    for (p, power, target) in s.powers_used.iter().filter(|(p, _, _)| *p == phase) {
        let _ = p;
        let code = codes.code(power);
        match target {
            None => parts.push(code),
            Some(land) => {
                // results this power produced in the land, plus moves in the land this phase (once)
                let mut res: Vec<String> = Vec::new();
                if let Some(l) = rec.lands.get(land) {
                    let (mut dmg, mut def, mut rec_bl) = (0u32, 0u32, 0u32);
                    for t in l.touches.iter().filter(|t| t.turn == s.turn && t.phase == phase && t.by == *power) {
                        match t.kind {
                            TouchKind::Damage(n) => dmg += n,
                            TouchKind::Defend(n) => def += n,
                            TouchKind::Recover(n) => rec_bl += n,
                            _ => {}
                        }
                    }
                    if dmg > 0 {
                        res.push(format!("dmg{dmg}"));
                    }
                    if def > 0 {
                        res.push(format!("Df{def}"));
                    }
                    if rec_bl > 0 {
                        res.push(format!("Bl−{rec_bl}"));
                    }
                    if !seen_lands.contains(&land.to_string()) {
                        let (mut out, mut into) = (0u32, 0u32);
                        for t in l.touches.iter().filter(|t| t.turn == s.turn && t.phase == phase) {
                            if let TouchKind::Moved { out: o, into: i } = t.kind {
                                out += o;
                                into += i;
                            }
                        }
                        if out > 0 {
                            res.push(format!("⇒{out}"));
                        }
                        if into > 0 {
                            res.push(format!("⇐{into}"));
                        }
                        seen_lands.push(land.to_string());
                    }
                }
                let bracket = if res.is_empty() { String::new() } else { format!("[{}]", res.join(" ")) };
                parts.push(format!("{code}>{land}{bracket}"));
            }
        }
    }
    parts.join("  ")
}

fn invader_line(rec: &GameRecord, turn: u32, canceled: &[(si_model::event::InvaderStep, Option<si_model::island::LandId>, String)]) -> String {
    let Some(t) = rec.turn(turn) else { return String::new() };
    let mut parts: Vec<String> = Vec::new();
    if !t.events.is_empty() {
        parts.push(format!("Ev {}", t.events.join(" ")));
    }
    if !t.fear_cards.is_empty() || t.fear_cards_earned > 0 {
        let names = if t.fear_cards.is_empty() { String::new() } else { format!(" {}", t.fear_cards.join(" ")) };
        parts.push(format!("Fr+{}{}", t.fear_cards_earned, names));
    }
    let step_text = |label: &str, cards: &[String], step: si_model::event::InvaderStep| -> Vec<String> {
        cards
            .iter()
            .map(|card| {
                let mut lands: Vec<String> = Vec::new();
                for l in rec.lands.values() {
                    for v in l.visits.iter().filter(|v| v.turn == turn && v.step == Some(step) && v.card.as_deref() == Some(card.as_str())) {
                        let mut r: Vec<String> = Vec::new();
                        let a = added(&v.added);
                        if !a.is_empty() {
                            r.push(a);
                        }
                        if v.blight > 0 {
                            r.push(if v.blight == 1 { "Bl".into() } else { format!("Bl+{}", v.blight) });
                        }
                        if v.cascade > 0 {
                            r.push("Bl~".into());
                        }
                        if v.dahan_lost > 0 {
                            r.push(if v.dahan_lost == 1 { "D†".into() } else { format!("D†×{}", v.dahan_lost) });
                        }
                        if v.invaders_destroyed > 0 {
                            r.push(format!("†{}", v.invaders_destroyed));
                        }
                        if step == si_model::event::InvaderStep::Ravage && v.defend > 0 {
                            r.push(format!("Df{}", v.defend));
                        }
                        lands.push(format!("{} {}", l.land, r.join(" ")).trim_end().to_string());
                    }
                }
                for (st, land, source) in canceled {
                    if *st == step {
                        if let Some(land) = land {
                            if !lands.iter().any(|x| x.starts_with(&format!("{land} "))) {
                                lands.push(format!("{land} ×{}", short_source(source)));
                            }
                        }
                    }
                }
                let esc = if card.ends_with('2') { "^" } else { "" };
                format!("{label} {card}{esc}[{}]", lands.join("  "))
            })
            .collect()
    };
    parts.extend(step_text("HI", &t.high_immigration, si_model::event::InvaderStep::Build));
    parts.extend(step_text("R", &t.ravage, si_model::event::InvaderStep::Ravage));
    parts.extend(step_text("B", &t.build, si_model::event::InvaderStep::Build));
    parts.extend(step_text("X", &t.explore, si_model::event::InvaderStep::Explore));
    parts.join(" · ")
}

fn short_source(s: &str) -> String {
    if s.starts_with("Stage ") {
        "Di".into()
    } else {
        code_of(s)
    }
}

/// The whole game in notation.
pub fn notate(rec: &GameRecord, defs: Option<&Definitions>, canonical: bool) -> String {
    let mut codes = Codes::new(defs, canonical);
    let mut out = String::new();
    let name = |id: &str| defs.and_then(|d| d.spirits.get(id)).map(|p| p.title.clone()).unwrap_or_else(|| id.to_string());
    let short = |id: &str| defs.and_then(|d| d.spirits.get(id)).map(|p| p.short_title.clone()).filter(|s| !s.is_empty()).unwrap_or_else(|| id.to_string());
    let game_id = rec.meta.source.rsplit('/').next().unwrap_or(&rec.meta.source).trim_start_matches("EndGame--").trim_end_matches(".txt").trim_start_matches("live-").to_string();
    out.push_str(&format!("[Game \"{game_id}\"]\n"));
    out.push_str(&format!("[Source \"{}\"]\n", if rec.meta.source.ends_with(".txt") { "EndGame log" } else { "capture" }));
    out.push_str(&format!("[Spirits \"{}\"]\n", rec.meta.spirits.iter().map(|s| name(s)).collect::<Vec<_>>().join(", ")));
    out.push_str(&format!("[Boards \"{}\"]\n", rec.meta.boards.join(" ")));
    match (&rec.meta.adversary, rec.meta.adversary_level) {
        (Some(a), Some(l)) => out.push_str(&format!("[Adversary \"{}\" {l}]\n", defs.and_then(|d| d.adversaries.get(a)).map(|x| x.title.clone()).unwrap_or(a.clone()))),
        (Some(a), None) => out.push_str(&format!("[Adversary \"{}\" ?]\n", defs.and_then(|d| d.adversaries.get(a)).map(|x| x.title.clone()).unwrap_or(a.clone()))),
        (None, _) => out.push_str("[Adversary \"-\"]\n"),
    }
    out.push_str(&format!("[Scenario \"{}\"]\n", rec.meta.scenario.clone().unwrap_or("-".into())));
    let result = match rec.meta.ending {
        Some(si_model::game::Ending::WinInvadersDestroyed) => format!("W Invaders T{}", rec.meta.turns),
        Some(si_model::game::Ending::WinTerrorVictory) => format!("W Terror T{}", rec.meta.turns),
        Some(si_model::game::Ending::LoseTooMuchBlight) => format!("L Blight T{}", rec.meta.turns),
        Some(si_model::game::Ending::LoseAdversary) => format!("L Adversary T{}", rec.meta.turns),
        Some(si_model::game::Ending::LoseInvaderDeck) => format!("L Deck T{}", rec.meta.turns),
        Some(si_model::game::Ending::LosePresence) => format!("L Presence T{}", rec.meta.turns),
        Some(si_model::game::Ending::Other) => format!("ended T{}", rec.meta.turns),
        None => "*".into(),
    };
    out.push_str(&format!("[Result \"{result}\"]\n\n"));
    let many = rec.meta.spirits.len() > 1;
    for t in rec.turns.iter().filter(|t| t.turn >= 1) {
        out.push_str(&format!("T{}\n", t.turn));
        let spirits: Vec<&SpiritTurn> = rec.spirits.iter().filter(|s| s.turn == t.turn).collect();
        for s in &spirits {
            let who = if many { format!("{}: ", short(&s.spirit)) } else { String::new() };
            let g = growth_line(s, &mut codes, canonical);
            if !g.is_empty() {
                out.push_str(&format!("  G  {who}{g}\n"));
            }
            if !s.cards_played.is_empty() {
                let played: Vec<String> = s.cards_played.iter().map(|c| codes.code(c)).collect();
                out.push_str(&format!("  P  {who}{}  ({}E) E={}\n", played.join(" "), s.energy_paid, s.energy_after));
            }
            let f = power_line(rec, s, Phase::FastPower, &mut codes);
            if !f.is_empty() {
                out.push_str(&format!("  F  {who}{f}\n"));
            }
        }
        let canceled: Vec<_> = t.canceled.iter().map(|c| (c.step, c.land.clone(), c.source.clone())).collect();
        let inv = invader_line(rec, t.turn, &canceled);
        if !inv.is_empty() {
            out.push_str(&format!("  I  {inv}\n"));
        }
        for s in &spirits {
            let who = if many { format!("{}: ", short(&s.spirit)) } else { String::new() };
            let sl = power_line(rec, s, Phase::SlowPower, &mut codes);
            if !sl.is_empty() {
                out.push_str(&format!("  S  {who}{sl}\n"));
            }
        }
        out.push_str(&format!("  TP Bl={} Fr+{} TL{}\n", t.board_after.blight, t.fear_cards_earned, t.terror_after));
    }
    if !canonical && !codes.legend().is_empty() {
        out.push_str(&format!("\n{{{}}}\n", codes.legend()));
    }
    out
}
