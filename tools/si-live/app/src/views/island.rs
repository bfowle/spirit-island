//! Right column: the island as compact land tiles (pieces and this turn's
//! actions as badges) and the Invader cards in their slots.

use super::fmt;
use si_common::companion_stats::CompanionStats;
use si_common::companion_stats::invaders::Ravage2;
use si_common::public::{PublicLand, PublicSnapshot};
use si_ui::theme::{self, TextStyle};
use si_ui::widgets::{self, panel};
use si_ui::{Align, Rect, Ui, span};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    // The island takes what its tiles need; the Invader cards get the rest.
    let rows = s.lands.len().div_ceil(3).max(1) as f32;
    let island_h = (widgets::PANEL_HEAD_H + 18.0 + rows * TILE_H + (rows - 1.0) * 6.0).min(r.h - 118.0 - theme::GAP);
    let mut col = r;
    let top = col.cut_top(island_h);
    col.cut_top(theme::GAP);
    island(ui, top, s, stats);
    pipeline(ui, col, s, stats);
}

const TILE_H: f32 = 96.0;

fn island(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let lands: Vec<&PublicLand> = s.lands.iter().collect();
    let c = panel(ui, r, "Island", Some(&format!("{} board{}", s.boards.len(), if s.boards.len() == 1 { "" } else { "s" })));
    ui.push_clip(c);
    // Three columns; rows fill the height (Ocean first, then lands 1–8).
    let cols = 3usize;
    let rows = lands.len().div_ceil(cols).max(1);
    let gap = 6.0;
    let tile_h = ((c.h - gap * (rows - 1) as f32) / rows as f32).clamp(56.0, TILE_H);
    let tile_w = (c.w - gap * (cols - 1) as f32) / cols as f32;
    for (i, l) in lands.iter().enumerate() {
        let (row, colno) = (i / cols, i % cols);
        let t = Rect::new(c.x + colno as f32 * (tile_w + gap), c.y + row as f32 * (tile_h + gap), tile_w, tile_h);
        tile(ui, t, l, stats);
    }
    ui.pop_clip();
}

fn tile(ui: &mut Ui, t: Rect, l: &PublicLand, stats: &CompanionStats) {
    let f = &stats.invader_phase;
    let ravage = f.ravages.iter().find(|r| r.land == l.id);
    let build = f.builds.iter().find(|b| b.land == l.id);
    let hi = f.high_immigration.iter().find(|b| b.land == l.id);
    let outlook = stats.lands.iter().find(|o| o.id == l.id);
    let border = if ravage.is_some_and(|r| r.ledger.adds_blight) { theme::RED.alpha(160) } else { theme::BORDER };
    ui.rect(t, theme::RAISED, border, 1.0, theme::RADIUS);
    ui.fill(Rect::new(t.x, t.y, 4.0, t.h), fmt::terrain(&l.terrain), 2.0);
    let inner = t.inset(10.0, 6.0, 8.0, 6.0);
    ui.push_clip(inner);

    // Header: number and terrain; a coastal land has an Ocean-blue top edge.
    if l.coastal {
        ui.fill(Rect::new(t.x + 6.0, t.y, t.w - 12.0, 2.0), fmt::terrain("Ocean"), 1.0);
    }
    let num = format!("{}{}", fmt::board_short(&l.board), l.number);
    let (nw, _) = ui.text(inner.x, inner.y, &num, theme::BODY.bold());
    ui.text(inner.x + nw + 6.0, inner.y + 1.0, &l.terrain, theme::SMALL.color(theme::MUTED));
    if l.defend > 0 {
        let d = format!("Defend {}", l.defend);
        let w = ui.measure(&d, theme::SMALL).0 + 10.0;
        widgets::chip(ui, inner.right() - w, inner.y, &d, theme::BG, theme::GREEN);
    }

    // Pieces.
    let mono = TextStyle { size: 12.5, line: 16.0, mono: true, weight: 500, color: theme::TEXT };
    let mut spans_text: Vec<(String, TextStyle)> = Vec::new();
    for (n, label, color) in [
        (l.explorers, "Explorer", theme::TEXT),
        (l.towns, "Town", theme::TEXT),
        (l.cities, "City", theme::TEXT),
        (l.dahan, "Dahan", theme::GREEN),
        (l.blight, "Blight", theme::RED),
    ] {
        if n > 0 {
            let short = &label[..1];
            spans_text.push((format!("{n}{short} "), mono.color(color)));
        }
    }
    let presence: u32 = l.presence.values().sum();
    if presence > 0 {
        spans_text.push((format!("P{presence} "), mono.color(theme::ACCENT)));
    }
    for (tok, n) in &l.tokens {
        if *n > 0 {
            let short: String = tok.chars().take(3).collect();
            spans_text.push((format!("{n}{short} "), mono.color(theme::TEXT_2)));
        }
    }
    let strife: i64 = l.pieces.iter().map(|p| p.strife).sum();
    if strife > 0 {
        spans_text.push((format!("{strife}Str "), mono.color(theme::AMBER)));
    }
    let y = inner.y + 20.0;
    let pieces_bottom = if spans_text.is_empty() {
        ui.text(inner.x, y, "empty", theme::SMALL.color(theme::FAINT));
        y + 16.0
    } else {
        let spans: Vec<_> = spans_text.iter().map(|(t, st)| span(t, *st)).collect();
        y + ui.spans(inner.x, y, &spans, Some(inner.w)).1
    };

    // This turn's actions, as badges from the bottom-left; a badge that
    // doesn't fit starts a row above.
    let mut badges: Vec<(String, si_ui::theme::Rgba, si_ui::theme::Rgba, bool)> = Vec::new();
    if let Some(b) = hi {
        badges.push((format!("HI +{}", &b.adds[..1]), theme::BG, theme::AMBER, true));
    }
    if let Some(r) = ravage {
        if r.ledger.cancelled.is_some() {
            badges.push(("no Ravage".to_string(), theme::MUTED, theme::MUTED, false));
        } else {
            let (label, bg) = if r.ledger.adds_blight { ("RAVAGE ▸ BLIGHT", theme::RED.alpha(140)) } else { ("RAVAGE", theme::AMBER.alpha(70)) };
            badges.push((label.to_string(), theme::TEXT, bg, true));
        }
    }
    if let Some(b) = build {
        badges.push((format!("BUILD +{}", &b.adds[..1]), theme::BG, theme::AMBER, true));
    }
    if let Some((n, d)) = f.explore.per_land.get(&l.id).copied().filter(|(n, _)| *n > 0) {
        let label = if f.explore.revealed.is_some() { "EXPLORE".to_string() } else { format!("EXPLORE {n}/{d}") };
        badges.push((label, theme::TEXT, theme::BLUE.alpha(90), true));
    }
    if outlook.is_some_and(|o| o.badges.iter().any(|b| b == "ravage-next")) {
        badges.push(("ravage next".to_string(), theme::MUTED, theme::MUTED, false));
    }
    let mut x = inner.x;
    let mut by = inner.bottom() - 18.0;
    for (label, fg, bg, filled) in &badges {
        let w = ui.measure(label, theme::SMALL).0 + 12.0;
        if x > inner.x && x + w > inner.right() {
            x = inner.x;
            by -= 21.0;
        }
        // No room left above the pieces: the rest (least important last) wait.
        if by < pieces_bottom + 2.0 {
            break;
        }
        x += if *filled { widgets::chip(ui, x, by, label, *fg, *bg) } else { widgets::tag(ui, x, by, label, *fg) } + 4.0;
    }
    ui.pop_clip();
}

fn pipeline(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let inv = &s.invaders;
    let c = panel(ui, r, "Invader cards", Some(&format!("discard {}", inv.discard.len())));
    ui.push_clip(c);
    let mut slots: Vec<(&str, Vec<si_common::public::PublicInvaderCard>)> = Vec::new();
    for (slot, cards) in &inv.extra_slots {
        if slot.starts_with("HighImmigration") {
            slots.push(("High Immig.", cards.clone()));
        }
    }
    slots.push(("Ravage", inv.ravage.clone()));
    slots.push(("Build", inv.build.clone()));
    slots.push(("Explore", inv.explore.clone()));
    let cols = c.columns(slots.len(), 8.0);
    for ((name, cards), col) in slots.iter().zip(cols) {
        ui.text_in(Rect::new(col.x, col.y, col.w, 16.0), &name.to_uppercase(), theme::HEAD, Align::Left);
        let mut y = col.y + 22.0;
        if cards.is_empty() {
            let text = if *name == "Explore" {
                match stats.invader_phase.explore.next_stage {
                    Some(st) => format!("next: Stage {}", fmt::roman(st as i64)),
                    None => "deck empty".into(),
                }
            } else {
                "empty".into()
            };
            ui.text(col.x, y + 2.0, &text, theme::SMALL.color(theme::MUTED));
        }
        for card in cards {
            ui.push_clip(col);
            fmt::card_chip(ui, col.x, y, card);
            ui.pop_clip();
            y += 26.0;
        }
    }

    // Next turn: the Build card becomes the Ravage card.
    let y = c.y + 76.0;
    if y + 40.0 < c.bottom() {
        ui.fill(Rect::new(c.x, y - 8.0, c.w, 1.0), theme::BORDER, 0.0);
        ui.text(c.x, y, "NEXT TURN'S RAVAGE", theme::HEAD);
        let certain: Vec<&str> = stats.lands.iter().filter(|o| o.ravage_next == Ravage2::Certain).map(|o| o.id.as_str()).collect();
        let maybe: Vec<&str> = stats.lands.iter().filter(|o| o.ravage_next == Ravage2::IfExplored).map(|o| o.id.as_str()).collect();
        let mut x = c.x;
        let mut yy = y + 22.0;
        if certain.is_empty() && maybe.is_empty() {
            ui.text(x, yy, "No land yet", theme::SMALL.color(theme::MUTED));
        }
        for id in &certain {
            let w = fmt::land_chip(ui, x, yy, s, id);
            x += w + 5.0;
            if x > c.right() - 110.0 {
                x = c.x;
                yy += 24.0;
            }
        }
        if !maybe.is_empty() {
            if !certain.is_empty() {
                x = c.x;
                yy += 26.0;
            }
            let names: Vec<String> = maybe.iter().map(|id| fmt::short(s, id)).collect();
            ui.text(x, yy + 2.0, &format!("if explored now: {}", names.join(", ")), theme::SMALL);
        }
    }
    ui.pop_clip();
}
