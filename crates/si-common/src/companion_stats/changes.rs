//! Play-by-play feed: what changed between consecutive public snapshots.
//!
//! Built from public snapshots only (not from `log.txt`, which names hidden
//! Fear cards), so nothing here can cross the firewall.

use crate::public::{PublicLand, PublicSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Change {
    pub turn: u32,
    /// Step the change happened in (the earlier snapshot's step).
    pub step: String,
    pub text: String,
    /// "invader", "blight", "dahan", "presence", "fear", "card", "energy", "event".
    pub kind: String,
}

fn land_line(la: &PublicLand, lb: &PublicLand) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    let mut pieces = Vec::new();
    for (name, a, b) in [("Explorer", la.explorers, lb.explorers), ("Town", la.towns, lb.towns), ("City", la.cities, lb.cities)] {
        if a != b {
            pieces.push(format!("{:+} {name}", b as i64 - a as i64));
        }
    }
    if !pieces.is_empty() {
        out.push((format!("{}: {}", lb.id, pieces.join(", ")), "invader"));
    }
    if la.blight != lb.blight {
        out.push((format!("{}: {:+} Blight", lb.id, lb.blight as i64 - la.blight as i64), "blight"));
    }
    if la.dahan != lb.dahan {
        out.push((format!("{}: {:+} Dahan", lb.id, lb.dahan as i64 - la.dahan as i64), "dahan"));
    }
    let pa: u32 = la.presence.values().sum();
    let pb: u32 = lb.presence.values().sum();
    if pa != pb {
        out.push((format!("{}: {:+} Presence", lb.id, pb as i64 - pa as i64), "presence"));
    }
    for (t, nb) in &lb.tokens {
        let na = la.tokens.get(t).copied().unwrap_or(0);
        if na != *nb {
            out.push((format!("{}: {:+} {t}", lb.id, *nb as i64 - na as i64), "invader"));
        }
    }
    for (t, na) in &la.tokens {
        if !lb.tokens.contains_key(t) {
            out.push((format!("{}: -{na} {t}", lb.id), "invader"));
        }
    }
    out
}

fn diff(a: &PublicSnapshot, b: &PublicSnapshot) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    if (a.turn, a.step.as_str()) != (b.turn, b.step.as_str()) && a.turn != b.turn {
        out.push((format!("Turn {} begins", b.turn), "event"));
    }
    for lb in &b.lands {
        if let Some(la) = a.lands.iter().find(|l| l.id == lb.id) {
            out.extend(land_line(la, lb));
        }
    }
    let ids = |cards: &[crate::public::NamedCard]| cards.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
    for c in b.events.discard.iter().filter(|c| !ids(&a.events.discard).contains(&c.id)) {
        out.push((format!("Event: {}", c.title), "event"));
    }
    for c in b.fear.discard.iter().filter(|c| !a.fear.discard.iter().any(|x| x.id == c.id)) {
        out.push((format!("Fear card resolved: {}", c.title), "fear"));
    }
    if b.fear.earned_count > a.fear.earned_count {
        out.push((format!("Fear card earned (face down; {} banked)", b.fear.earned_count), "fear"));
    }
    if b.fear.terror_level != a.fear.terror_level {
        out.push((format!("Terror Level {}", b.fear.terror_level), "fear"));
    }
    if b.blight.flipped && !a.blight.flipped {
        let name = b.blight.blighted_card.as_ref().map(|c| c.title.clone()).unwrap_or_default();
        out.push((format!("Blight card flipped: {name} ({} blight)", b.blight.remaining), "blight"));
    }
    let slot = |s: &PublicSnapshot| (s.invaders.ravage.clone(), s.invaders.build.clone(), s.invaders.explore.clone());
    if slot(a) != slot(b)
        && let Some(c) = b.invaders.explore.first().filter(|c| !a.invaders.explore.iter().any(|x| x.id == c.id)) {
            out.push((format!("Explore card revealed: {} ({})", c.id, c.terrains.join(" + ")), "invader"));
        }
    for (sa, sb) in a.spirits.iter().zip(&b.spirits) {
        for c in sb.play_area.iter().filter(|c| !sa.play_area.iter().any(|x| x.id == c.id)) {
            out.push((format!("Played {} ({}, {} Energy)", c.title, c.speed, c.cost), "card"));
        }
        let had = |id: &str| sa.hand.iter().chain(&sa.play_area).chain(&sa.discard).chain(&sa.forgotten).any(|x| x.id == id);
        for c in sb.hand.iter().filter(|c| !had(&c.id)) {
            out.push((format!("Gained {} ({} {})", c.title, c.kind, c.speed), "card"));
        }
        for c in sb.forgotten.iter().filter(|c| !sa.forgotten.iter().any(|x| x.id == c.id)) {
            out.push((format!("Forgot {}", c.title), "card"));
        }
        if sb.energy != sa.energy {
            out.push((format!("Energy {} → {}", sa.energy, sb.energy), "energy"));
        }
    }
    if b.fear.generated != a.fear.generated && b.fear.earned_count == a.fear.earned_count {
        out.push((format!("Fear {} → {} of {}", a.fear.generated, b.fear.generated, b.fear.required_per_card), "fear"));
    }
    out
}

/// The most recent `limit` changes, newest last.
pub fn recent(timeline: &[PublicSnapshot], limit: usize) -> Vec<Change> {
    let mut out: Vec<Change> = Vec::new();
    let start = timeline.len().saturating_sub(limit + 1);
    for w in timeline[start..].windows(2) {
        for (text, kind) in diff(&w[0], &w[1]) {
            out.push(Change { turn: w[0].turn, step: w[0].step.clone(), text, kind: kind.into() });
        }
    }
    let n = out.len();
    out.split_off(n.saturating_sub(limit))
}
