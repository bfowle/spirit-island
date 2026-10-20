//! Step 1 of the 2026-10-19 direction: a static mockup of the Growth screen
//! on Test 1, Turn 3 (Shadows Flicker Like Flame vs Brandenburg-Prussia L2,
//! boards C+E), drawn from fixed numbers read off the capture
//! (`si-steam-dump stats … 63`), in the §6 grammar: Sly-style named
//! instruments, THE RACE on one shared turn axis, the decision instrument
//! headed by the question Brett asked, FIXED / LEVERS / COUNTED columns,
//! budgets as side instruments. Nothing here reads live data.
//!
//! Numbers (Test 1 T3 Growth save): Blight card healthy, 3 left; Ravage
//! Jungle (C1, C4, E3: 3 damage vs health 2 each, Blight each); Build Sands
//! (E1 +Town; C2 Disease); next Explore Stage III (Prussia L2), 6 kinds;
//! Fear 4 of 5, 1 banked, 1 Terror I card left; deck 9 cards → out at T12;
//! Shadows: Energy 3 (+1 a turn), 2 plays, hand 1 (Crops, Slow), discard 4;
//! Minors: 96 unseen, 40 save C4, 34 reach, 19 Fast and reach; a draw of 4
//! holds one of the 19 in 59 of 100.

use super::chart;
use super::fmt;
use crate::icons;
use si_ui::theme::{self, Rgba, TextStyle};
use si_ui::{Align, Pt, Rect, Ui};

const INVADER: Rgba = Rgba::hex(0xf0885f);
const SPIRIT: Rgba = Rgba::hex(0x7cc4ff);
const ACCENT: Rgba = theme::ACCENT;
const REF: Rgba = theme::MUTED;

const NOW: u32 = 3;
const DECK_OUT: u32 = 12;

fn head() -> TextStyle {
    TextStyle { size: 11.5, line: 15.0, mono: false, weight: 700, color: theme::TEXT_2 }
}
fn small() -> TextStyle {
    theme::SMALL
}
fn mono() -> TextStyle {
    TextStyle { size: 12.0, line: 15.0, mono: true, weight: 500, color: theme::TEXT }
}
fn question() -> TextStyle {
    TextStyle { size: 15.0, line: 20.0, mono: false, weight: 700, color: theme::TEXT }
}

/// A named instrument: panel, header with NAME and a count, inner rect.
fn instrument(ui: &mut Ui, r: Rect, name: &str, count: &str) -> Rect {
    ui.rect(r, theme::PANEL, theme::BORDER, 1.0, 6.0);
    let hr = Rect::new(r.x, r.y, r.w, 26.0);
    ui.fill(Rect::new(r.x + 1.0, r.y + 1.0, r.w - 2.0, 25.0), theme::PANEL_HEAD, 5.0);
    ui.fill(Rect::new(r.x + 10.0, r.y + 9.0, 8.0, 8.0), ACCENT.alpha(170), 2.0);
    ui.text(r.x + 24.0, r.y + 6.0, name, head());
    ui.text_in(Rect::new(hr.x, hr.y + 6.0, hr.w - 10.0, 15.0), count, mono().color(theme::MUTED), Align::Right);
    ui.fill(Rect::new(r.x + 1.0, r.y + 26.0, r.w - 2.0, 1.0), theme::BORDER, 0.0);
    r.inset(12.0, 34.0, 12.0, 10.0)
}

/// Ten dots: `n` filled of 10, the rest hollow; the count beside.
fn dots(ui: &mut Ui, x: f32, y: f32, n: usize, color: Rgba, label: &str) -> f32 {
    for i in 0..10 {
        let c = (x + i as f32 * 11.0 + 4.0, y + 7.0);
        if i < n {
            ui.circle(c, 4.0, color, theme::TRANSPARENT, 0.0);
        } else {
            ui.circle(c, 4.0, theme::TRANSPARENT, color.alpha(110), 1.2);
        }
    }
    let (w, _) = ui.text(x + 116.0, y, label, mono().color(color));
    116.0 + w
}

/// Slot stack: `filled` solid of `total`, the rest outlined.
fn slots(ui: &mut Ui, x: f32, y: f32, total: usize, filled: usize, color: Rgba) -> f32 {
    for i in 0..total {
        let r = Rect::new(x + i as f32 * 12.0, y, 9.0, 14.0);
        if i < filled {
            ui.fill(r, color, 2.0);
        } else {
            ui.rect(r, theme::TRANSPARENT, color.alpha(120), 1.0, 2.0);
        }
    }
    total as f32 * 12.0
}

/// Hatched: a quantity that cannot be computed yet (needs a layer-1 change).
fn hatched(ui: &mut Ui, r: Rect, label: &str) {
    ui.rect(r, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, 3.0);
    let mut x = r.x - r.h;
    while x < r.right() {
        let a = (x.max(r.x), if x < r.x { r.y + (r.x - x) } else { r.y });
        let b = ((x + r.h).min(r.right()), if x + r.h > r.right() { r.y + r.h - (x + r.h - r.right()) } else { r.bottom() });
        ui.line(a, b, 1.0, theme::BORDER_STRONG.alpha(140), 0.0);
        x += 7.0;
    }
    ui.text(r.right() + 6.0, r.y, label, small().color(theme::FAINT));
}

fn chip(ui: &mut Ui, x: f32, y: f32, land: &str, terrain: &str) -> f32 {
    let c = fmt::terrain(terrain);
    let label = format!("{land} {terrain}");
    let w = ui.measure(&label, small().bold()).0 + 14.0;
    let r = Rect::new(x, y, w, 18.0);
    ui.rect(r, theme::RAISED, theme::TRANSPARENT, 0.0, 4.0);
    ui.fill(Rect::new(x, y + 3.0, 3.0, 12.0), c, 1.5);
    ui.text(x + 9.0, y + 2.0, &label, small().bold().color(theme::TEXT));
    w
}

fn pieces(ui: &mut Ui, x: f32, y: f32, items: &[(u16, u32, Rgba)]) -> f32 {
    let mut cx = x;
    for (id, n, color) in items {
        for _ in 0..*n {
            ui.icon(Rect::new(cx, y, 14.0, 14.0), *id, *color);
            cx += 15.0;
        }
        cx += 5.0;
    }
    cx - x
}

pub fn draw(ui: &mut Ui) {
    let s = ui.screen;
    ui.fill(s, theme::BG, 0.0);
    let gutter = 10.0;

    // Ribbon.
    let rib = Rect::new(s.x, s.y, s.w, 34.0);
    ui.fill(rib, theme::PANEL_HEAD, 0.0);
    ui.fill(Rect::new(rib.x, rib.bottom() - 1.0, rib.w, 1.0), theme::BORDER, 0.0);
    let big = TextStyle { size: 16.0, line: 20.0, mono: false, weight: 700, color: theme::TEXT };
    ui.text(rib.x + 14.0, rib.y + 7.0, "T3", big);
    let steps = ["GROWTH", "Fast", "Event", "Fear", "Ravage", "Build", "Explore", "Slow"];
    let mut x = rib.x + 60.0;
    for (i, st) in steps.iter().enumerate() {
        let on = i == 0;
        let style = if on { small().bold().color(theme::TEXT) } else { small().color(theme::MUTED) };
        let w = ui.measure(st, style).0;
        if on {
            ui.fill(Rect::new(x - 8.0, rib.y + 6.0, w + 16.0, 22.0), ACCENT.alpha(60), 11.0);
            ui.rect(Rect::new(x - 8.0, rib.y + 6.0, w + 16.0, 22.0), theme::TRANSPARENT, ACCENT, 1.0, 11.0);
        }
        ui.text(x, rib.y + 9.0, st, style);
        x += w + 26.0;
    }
    ui.text_in(Rect::new(rib.x, rib.y + 9.0, rib.w - 14.0, 15.0), "Brandenburg-Prussia L2  ·  Shadows Flicker Like Flame  ·  boards C + E", small().color(theme::TEXT_2), Align::Right);

    // Status bar.
    let status = Rect::new(s.x, s.bottom() - 26.0, s.w, 26.0);
    ui.fill(status, theme::PANEL_HEAD, 0.0);
    ui.fill(Rect::new(status.x, status.y, status.w, 1.0), theme::BORDER, 0.0);
    ui.fill(Rect::new(status.x, status.y + 1.0, 70.0, 25.0), ACCENT.alpha(190), 0.0);
    ui.text_in(Rect::new(status.x, status.y + 6.0, 70.0, 15.0), "REPLAY", small().bold().color(theme::TEXT), Align::Center);
    ui.text(status.x + 84.0, status.y + 6.0, "Test 1 · 2026-10-18  |  save 64 / 191  |  Turn 3 · Growth  |  120 card orders  |  4.1 ms", mono().color(theme::MUTED).size(11.5));
    ui.text_in(Rect::new(status.x, status.y + 6.0, status.w - 14.0, 15.0), "mockup · fixed numbers from the capture  |  H hold", mono().color(theme::MUTED).size(11.5), Align::Right);

    // Think-about line.
    let think = Rect::new(s.x, status.y - 24.0, s.w, 24.0);
    ui.text(think.x + 14.0, think.y + 4.0, "think about", small().bold().color(theme::MUTED));
    ui.text(think.x + 96.0, think.y + 4.0, "Is this about order or about count?     ·     Which effects might you be missing?     ·     What does this card buy now, and later?", small().color(theme::TEXT_2));

    // Columns: main (race + decision) and the side column of budget instruments.
    let body = Rect::new(s.x + gutter, rib.bottom() + gutter, s.w - 2.0 * gutter, think.y - rib.bottom() - 2.0 * gutter);
    let side_w = 232.0;
    let main = Rect::new(body.x, body.y, body.w - side_w - gutter, body.h);
    let side = Rect::new(main.right() + gutter, body.y, side_w, body.h);

    // ---- THE RACE
    let race_r = Rect::new(main.x, main.y, main.w, 214.0);
    let race = instrument(ui, race_r, "THE RACE  ·  which ending arrives first, unanswered?", "120 card orders · no Spirit action");
    let label_w = 196.0;
    let p = chart::Plot { r: Rect::new(race.x + label_w, race.y + 8.0, race.w - label_w - 70.0, 138.0), x0: NOW as f32 - 0.5, x1: DECK_OUT as f32 + 0.5, y0: 0.0, y1: 1.0 };
    for t in NOW..=DECK_OUT {
        let x = p.px(t as f32);
        ui.line((x, p.r.y), (x, p.r.bottom()), 1.0, if t == NOW { ACCENT.alpha(90) } else { theme::BORDER.alpha(160) }, 0.0);
        let style = if t == NOW { small().bold().color(theme::TEXT) } else { small().color(theme::MUTED) };
        ui.text_in(Rect::new(x - 20.0, p.r.bottom() + 2.0, 40.0, 14.0), &format!("T{t}"), style, Align::Center);
    }
    ui.text(race.x, race.y + 2.0, "INVADERS", small().bold().color(INVADER.alpha(200)));
    ui.text(race.x, race.y + 82.0, "SPIRITS", small().bold().color(SPIRIT.alpha(200)));
    let row = |ui: &mut Ui, y: f32, name: &str, side: Rgba| {
        ui.text(race.x + 12.0, y, name, small().color(theme::TEXT_2));
        ui.line((p.r.x, y + 8.0), (p.r.right(), y + 8.0), 1.0, side.alpha(40), 0.0);
    };
    // Blight card flips: every order, this Ravage.
    let y = race.y + 20.0;
    row(ui, y, "Blight card flips", INVADER);
    let x3 = p.px(3.0);
    ui.fill(Rect::new(x3 - 2.0, y - 1.0, 4.0, 18.0), INVADER, 1.0);
    ui.text(x3 + 10.0, y, "this Ravage, every order", mono().color(INVADER));
    ui.text(x3 + 196.0, y, "C1 · C4 · E3: 3 Blight vs 3 on the healthy side", small().color(theme::TEXT_2));
    ui.text(p.r.right() + 8.0, y, "◄ BINDS", small().bold().color(INVADER));
    // Blight loss on the blighted side.
    let y = race.y + 40.0;
    row(ui, y, "Blight card empties", INVADER);
    dots(ui, p.px(4.0) - 4.0, y, 0, INVADER, "");
    dots(ui, p.px(5.0) - 4.0, y, 10, INVADER, "by T5, 98 of 100");
    ui.text(p.px(7.6), y, "the blighted side holds 2 to 10 Blight: 23 unseen cards", small().color(theme::MUTED));
    // Deck.
    let y = race.y + 60.0;
    row(ui, y, "Invader deck ends", INVADER);
    let x12 = p.px(12.0);
    ui.fill(Rect::new(x12 - 2.0, y - 1.0, 4.0, 18.0), INVADER.alpha(200), 1.0);
    ui.text(x12 - 112.0, y, "T12's Explore", mono().color(INVADER.alpha(200)));
    // Fear victory: scenario lines.
    let y = race.y + 98.0;
    row(ui, y, "Fear victory", SPIRIT);
    ui.text(p.r.x + 4.0, y, "9 of 40 Fear", mono().color(SPIRIT));
    // Scenario lines, never a probability: the pace that wins in time, and your best turn.
    let marks = [(12.0f32, "at 3.1 a turn: T12", 200u8), (7.0, "at 8 a turn (your T2): T7", 130)];
    for (at, label, a) in marks.iter() {
        let xa = p.px(*at);
        chart::dashed(ui, &[(p.px(3.0) + 96.0, y + 8.0), (xa, y + 8.0)], 1.2, SPIRIT.alpha(*a), 4.0, 4.0);
        ui.circle((xa, y + 8.0), 4.0, theme::TRANSPARENT, SPIRIT.alpha(*a), 1.4);
        let w = ui.measure(label, small()).0;
        ui.text(xa - w - 8.0, y - 13.0, label, small().color(SPIRIT.alpha(*a)));
    }
    // Board clear.
    let y = race.y + 118.0;
    row(ui, y, "Board clear", SPIRIT);
    ui.text(p.r.x + 4.0, y, "Terror I: 12 Invaders stand", mono().color(SPIRIT));
    ui.text(p.r.x + 230.0, y, "Terror II at 1 more Fear: 6 Towns and Cities stand", small().color(SPIRIT.alpha(180)));
    ui.text(p.r.right() + 8.0, y, "counts", small().color(theme::FAINT));

    // ---- THE DECISION
    let dec_r = Rect::new(main.x, race_r.bottom() + gutter, main.w, main.bottom() - race_r.bottom() - gutter);
    let dec = instrument(ui, dec_r, "GROWTH  ·  T3", "binding: the Blight card flips this Ravage");
    ui.text(dec.x, dec.y - 4.0, "Which growth fits this turn?", question());
    let top = dec.y + 24.0;
    let fixed_w = 272.0;
    let counted_w = 296.0;
    let fixed = Rect::new(dec.x, top, fixed_w, dec.bottom() - top);
    let counted = Rect::new(dec.right() - counted_w, top, counted_w, dec.bottom() - top);
    let levers = Rect::new(fixed.right() + 16.0, top, counted.x - fixed.right() - 32.0, dec.bottom() - top);
    ui.fill(Rect::new(fixed.right() + 8.0, top, 1.0, fixed.h), theme::BORDER, 0.0);
    ui.fill(Rect::new(counted.x - 8.0, top, 1.0, fixed.h), theme::BORDER, 0.0);

    // FIXED
    let mut y = fixed.y;
    ui.text(fixed.x, y, "FIXED", small().bold().color(theme::MUTED));
    ui.text(fixed.x + 48.0, y, "face-up", small().color(theme::FAINT));
    y += 22.0;
    let land_row = |ui: &mut Ui, y: f32, land: &str, terrain: &str, note: &str, items: &[(u16, u32, Rgba)]| {
        let w = chip(ui, fixed.x, y, land, terrain);
        pieces(ui, fixed.x + w + 8.0, y + 2.0, items);
        ui.text(fixed.x + 14.0, y + 20.0, note, small().color(theme::TEXT_2));
    };
    let town = (icons::TOWN, 1u32, Rgba::hex(0xd9c2a5));
    let exp = (icons::EXPLORER, 1u32, Rgba::hex(0xd9c2a5));
    let dah = |n: u32| (icons::DAHAN, n, Rgba::hex(0x9fd3a8));
    ui.text(fixed.x, y, "RAVAGE · Jungle", small().bold().color(INVADER));
    y += 18.0;
    land_row(ui, y, "C4", "Jungle", "3 vs 2 → Blight · Dahan clear it after (+1 Fear)", &[exp, town, dah(3), (icons::SACRED_SITE, 1, ACCENT)]);
    y += 40.0;
    land_row(ui, y, "C1", "Jungle", "3 vs 2 → Blight · 1 Dahan dies", &[exp, town, dah(1)]);
    y += 40.0;
    land_row(ui, y, "E3", "Jungle", "3 vs 2 → Blight · Dahan kill the Town (+1 Fear)", &[exp, town, dah(2)]);
    y += 40.0;
    ui.text(fixed.x, y, "each: Defend 2, or any effect that saves it", small().color(theme::MUTED));
    y += 24.0;
    ui.text(fixed.x, y, "BUILD · Sands", small().bold().color(INVADER));
    ui.text(fixed.x + 100.0, y, "E1 +Town  ·  C2 no Build (Disease)", small().color(theme::TEXT_2));
    y += 18.0;
    ui.text(fixed.x, y, "EXPLORE · Stage III", small().bold().color(INVADER));
    ui.text(fixed.x + 124.0, y, "6 kinds, 1 of 6 each", small().color(theme::TEXT_2));
    y += 28.0;
    ui.text(fixed.x, y, "SHADOWS", small().bold().color(SPIRIT));
    y += 18.0;
    ui.text(fixed.x, y, "Energy", small().color(theme::TEXT_2));
    ui.text(fixed.x + 60.0, y, "3   +1 a turn", mono());
    y += 17.0;
    ui.text(fixed.x, y, "plays", small().color(theme::TEXT_2));
    ui.text(fixed.x + 60.0, y, "2", mono());
    y += 17.0;
    ui.text(fixed.x, y, "hand", small().color(theme::TEXT_2));
    ui.text(fixed.x + 60.0, y, "1   Crops Wither and Fade (Slow)", mono());
    y += 17.0;
    ui.text(fixed.x, y, "discard", small().color(theme::TEXT_2));
    ui.text(fixed.x + 60.0, y, "4", mono());
    ui.text(fixed.x + 80.0, y + 1.0, "Bats · Mantle · Concealing · Favors", small().color(theme::TEXT_2));
    y += 22.0;
    ui.text(fixed.x, y, "tracks", small().color(theme::TEXT_2));
    let track = |ui: &mut Ui, x: f32, y: f32, name: &str, vals: &[&str], revealed: usize| {
        ui.text(x, y, name, small().color(theme::MUTED));
        let mut cx = x + 46.0;
        for (i, v) in vals.iter().enumerate() {
            let r = Rect::new(cx, y - 1.0, 18.0, 17.0);
            if i < revealed {
                ui.fill(r, SPIRIT.alpha(70), 3.0);
            } else if i == revealed {
                ui.rect(r, theme::TRANSPARENT, SPIRIT, 1.2, 3.0);
            } else {
                ui.rect(r, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, 3.0);
            }
            ui.text_in(r, v, mono().size(11.0).color(if i <= revealed { theme::TEXT } else { theme::MUTED }), Align::Center);
            cx += 21.0;
        }
    };
    track(ui, fixed.x + 60.0, y, "Energy", &["0", "1", "3", "4", "5", "6"], 2);
    y += 20.0;
    track(ui, fixed.x + 60.0, y, "plays", &["1", "2", "3", "3", "4", "5"], 2);

    // LEVERS: the consequence table. Each row: its name on a line, then one cell per option.
    let cols = ["G1  Reclaim All + Power Card", "G2  Power Card + Presence (r1)", "G3  Presence (r3) + 3 Energy"];
    let col_w = levers.w / 3.0;
    let col_x = |i: usize| levers.x + i as f32 * col_w;
    let mut y = levers.y;
    ui.text(levers.x, y, "LEVERS", small().bold().color(theme::MUTED));
    ui.text(levers.x + 56.0, y, "your options, fixed order, unranked · hover one to overlay its futures on THE RACE", small().color(theme::FAINT));
    y += 22.0;
    for (i, c) in cols.iter().enumerate() {
        let r = Rect::new(col_x(i), y, col_w - 10.0, 22.0);
        ui.rect(r, theme::RAISED, if i == 1 { ACCENT } else { theme::BORDER_STRONG }, 1.0, 5.0);
        ui.text_in(r, c, small().bold().color(theme::TEXT), Align::Center);
    }
    y += 32.0;
    let rowlab = |ui: &mut Ui, y: f32, name: &str, sub: &str| {
        ui.text(levers.x, y, name, small().bold().color(theme::TEXT_2));
        if !sub.is_empty() {
            let w = ui.measure(name, small().bold()).0;
            ui.text(levers.x + w + 10.0, y, sub, small().color(theme::FAINT));
        }
    };
    let rule = |ui: &mut Ui, y: f32| ui.line((levers.x, y), (levers.right() - 10.0, y), 1.0, theme::BORDER.alpha(150), 0.0);
    // Row 1: the binding ending.
    rowlab(ui, y, "Blight card flips this Ravage", "the binding ending");
    let cy = y + 17.0;
    for i in 0..3 {
        let x = col_x(i);
        if i < 2 {
            dots(ui, x, cy, 4, INVADER, "4 of 10");
            ui.text(x, cy + 15.0, "flips unless the draw saves C4", small().color(theme::MUTED));
        } else {
            dots(ui, x, cy, 10, INVADER, "10 of 10");
            ui.text(x, cy + 15.0, "nothing in hand acts before the Ravage", small().color(theme::MUTED));
        }
    }
    y += 52.0;
    rule(ui, y - 6.0);
    rowlab(ui, y, "C4 saved by the draw", "Fast, in reach, playable now");
    let cy = y + 17.0;
    for i in 0..3 {
        let x = col_x(i);
        if i < 2 {
            dots(ui, x, cy, 6, SPIRIT, "59 of 100");
            ui.text(x, cy + 15.0, "a draw of 4 holds one of the 19 that do", small().color(theme::MUTED));
        } else {
            ui.text(x, cy, "no draw", mono().color(theme::MUTED));
        }
    }
    y += 52.0;
    rule(ui, y - 6.0);
    rowlab(ui, y, "C4 saved by a Fear card", "1 banked · 1 more at 1 Fear (Terror II)");
    hatched(ui, Rect::new(col_x(0), y + 18.0, col_w * 3.0 - 10.0, 14.0), "");
    ui.text_in(Rect::new(col_x(0), y + 17.0, col_w * 3.0 - 10.0, 16.0), "common to every option  ·  not yet counted: the Fear pool is not projected", small().color(theme::TEXT_2), Align::Center);
    y += 40.0;
    rule(ui, y - 6.0);
    rowlab(ui, y, "The Event", "");
    hatched(ui, Rect::new(col_x(0), y + 18.0, col_w * 3.0 - 10.0, 14.0), "");
    ui.text_in(Rect::new(col_x(0), y + 17.0, col_w * 3.0 - 10.0, 16.0), "common to every option  ·  not yet counted: the Event pool is not projected", small().color(theme::TEXT_2), Align::Center);
    y += 40.0;
    rule(ui, y - 6.0);
    rowlab(ui, y, "Card after the Ravage", "healthy side, 3 left");
    let cy = y + 18.0;
    for i in 0..3 {
        let x = col_x(i);
        slots(ui, x, cy, 3, 3, INVADER.alpha(150));
        ui.text(x + 40.0, cy, "→", mono().color(theme::MUTED));
        if i < 2 {
            slots(ui, x + 56.0, cy, 3, 1, INVADER);
            ui.text(x + 100.0, cy, "1 left · 6 of 10", mono().color(theme::TEXT_2));
            ui.text(x, cy + 17.0, "flips · 4 of 10", small().color(theme::MUTED));
        } else {
            slots(ui, x + 56.0, cy, 3, 0, INVADER);
            ui.text(x + 100.0, cy, "flips · 10 of 10", mono().color(INVADER));
        }
    }
    y += 56.0;
    rule(ui, y - 6.0);
    rowlab(ui, y, "Hand · plays · Energy this turn", "");
    let cy = y + 17.0;
    let econ = [("6 cards · 2 plays · 4 Energy", "Reclaim brings 4 back; Energy +1"), ("2 cards · 2 plays · 4 Energy", "plays track: 3 plays · Energy track: 6"), ("1 card · 2 plays · 7 Energy", "Energy track: 9 · 1 play unused")];
    for (i, (e, sub)) in econ.iter().enumerate() {
        ui.text(col_x(i), cy, e, mono().color(theme::TEXT));
        ui.text(col_x(i), cy + 15.0, sub, small().color(theme::MUTED));
    }
    y += 52.0;
    rule(ui, y - 6.0);
    // Pace rows on the shared turn axis: one mini axis for both.
    let q = chart::Plot { r: Rect::new(col_x(0), y, col_w * 3.0 - 150.0, 14.0), x0: NOW as f32 - 0.5, x1: DECK_OUT as f32 + 0.5, y0: 0.0, y1: 1.0 };
    let axis_bottom = y + 150.0;
    for t in NOW..=DECK_OUT {
        let x = q.px(t as f32);
        ui.line((x, y + 14.0), (x, axis_bottom), 1.0, theme::BORDER.alpha(110), 0.0);
        ui.text_in(Rect::new(x - 16.0, axis_bottom + 2.0, 32.0, 12.0), &format!("T{t}"), small().size(10.5).color(theme::FAINT), Align::Center);
    }
    rowlab(ui, y, "Cards run out (forced Reclaim)", "bar length = turns of plays left, on the race's axis");
    let bars: [(u32, &str, Rgba); 3] = [(5, "G1  T6", SPIRIT), (3, "G2  T4", SPIRIT.alpha(160)), (3, "G3  T4", SPIRIT.alpha(110))];
    for (i, (until, label, c)) in bars.iter().enumerate() {
        let yy = y + 20.0 + i as f32 * 12.0;
        let r = Rect::new(q.px(3.0) - 6.0, yy, q.px(*until as f32 + 1.0) - q.px(3.0), 9.0);
        ui.fill(r, *c, 2.0);
        ui.text(r.right() + 6.0, yy - 2.0, label, mono().size(11.0).color(*c));
    }
    y += 64.0;
    rowlab(ui, y, "Energy a turn, if the Presence takes the Energy track", "");
    let lines: [(&[(f32, f32)], &str, Rgba, bool); 3] = [(&[(3.0, 1.0), (7.0, 1.0)], "G1  1 a turn", SPIRIT.alpha(120), false), (&[(3.0, 1.0), (4.0, 3.0), (7.0, 3.0)], "G2  3 a turn", SPIRIT, false), (&[(3.0, 1.0), (4.0, 3.0), (7.0, 3.0)], "G3  3 a turn, +3 now", SPIRIT.alpha(160), true)];
    let yp = |v: f32| y + 68.0 - v * 13.0;
    for (k, (pts, label, c, dash)) in lines.iter().enumerate() {
        let poly: Vec<Pt> = pts.iter().map(|(t, v)| (q.px(*t), yp(*v) + if *dash { 4.0 } else { 0.0 })).collect();
        if *dash {
            chart::dashed(ui, &poly, 1.6, *c, 5.0, 4.0);
        } else {
            chart::polyline(ui, &poly, 1.6, *c);
        }
        ui.text(q.px(7.0) + 12.0, y + 16.0 + k as f32 * 13.0, label, mono().size(11.0).color(*c));
    }
    let band: Vec<Pt> = (NOW..=9).map(|t| (q.px(t as f32), y + 78.0 - [3.0f32, 4.0, 4.0, 5.0, 6.0, 7.0, 7.0][(t - NOW) as usize] * 1.6)).collect();
    chart::polyline(ui, &band, 1.2, INVADER.alpha(130));
    ui.text(q.px(9.0) + 8.0, y + 62.0, "lands the Invaders act in, a turn (floor)", small().size(10.5).color(INVADER.alpha(160)));

    // COUNTED
    let mut y = counted.y;
    ui.text(counted.x, y, "COUNTED", small().bold().color(theme::MUTED));
    ui.text(counted.x + 68.0, y, "hidden, from the public pools", small().color(theme::FAINT));
    y += 22.0;
    ui.text(counted.x, y, "MINOR DECK", small().bold().color(SPIRIT));
    ui.text_in(Rect::new(counted.x, y, counted.w, 15.0), "96 unseen · 4 seen", mono().color(theme::MUTED), Align::Right);
    y += 20.0;
    let fam = |ui: &mut Ui, y: f32, name: &str, k: usize, of: usize, tenths: usize| {
        ui.text(counted.x, y, name, small().color(theme::TEXT_2));
        dots(ui, counted.x + 124.0, y, tenths, SPIRIT, &format!("{k} of {of}"));
    };
    ui.text(counted.x, y, "saves C4 (any effect)", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 4, SPIRIT, "40 of 96");
    y += 18.0;
    ui.text(counted.x, y, "… and reaches it", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 4, SPIRIT, "34 of 96");
    y += 18.0;
    ui.text(counted.x, y, "… Fast, playable now", small().bold().color(theme::TEXT));
    dots(ui, counted.x + 124.0, y, 2, SPIRIT, "19 of 96");
    y += 18.0;
    ui.text(counted.x + 12.0, y, "a draw of 4 holds one:", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 6, SPIRIT, "59 of 100");
    y += 18.0;
    ui.text(counted.x + 12.0, y, "pocket rule: 1 − (1 − 19/96)⁴ ≈ 6 in 10", small().color(theme::MUTED));
    y += 24.0;
    ui.text(counted.x, y, "by effect", small().bold().color(theme::MUTED));
    y += 17.0;
    for (name, k, tenths) in [("Defend 2+", 16, 2), ("Push/Gather Dahan", 16, 2), ("Push Invaders", 15, 2), ("Remove Blight", 9, 1), ("Strife", 7, 1), ("Isolate", 4, 0)] {
        fam(ui, y, name, k, 96, tenths);
        y += 17.0;
    }
    y += 10.0;
    ui.text(counted.x, y, "MAJOR DECK", small().bold().color(SPIRIT));
    ui.text_in(Rect::new(counted.x, y, counted.w, 15.0), "78 unseen", mono().color(theme::MUTED), Align::Right);
    y += 20.0;
    ui.text(counted.x, y, "saves C4, Fast, in reach", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 2, SPIRIT, "19 of 78");
    y += 18.0;
    ui.text(counted.x + 12.0, y, "a draw of 4 holds one:", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 7, SPIRIT, "68 of 100");
    y += 18.0;
    ui.text(counted.x + 12.0, y, "costs a Forget; 3+ Energy to play", small().color(theme::MUTED));
    y += 26.0;
    ui.text(counted.x, y, "FEAR CARDS", small().bold().color(SPIRIT));
    ui.text_in(Rect::new(counted.x, y, counted.w, 15.0), "1 banked · Terror I: 1 left", mono().color(theme::MUTED), Align::Right);
    y += 20.0;
    hatched(ui, Rect::new(counted.x, y, 110.0, 14.0), "not yet counted");
    y += 18.0;
    ui.text(counted.x, y, "1 more Fear before the Fear step → Terror II:", small().color(theme::TEXT_2));
    y += 15.0;
    ui.text(counted.x, y, "both cards resolve at Terror II", small().color(theme::TEXT_2));
    y += 26.0;
    ui.text(counted.x, y, "NEXT INVADER CARD", small().bold().color(INVADER));
    ui.text_in(Rect::new(counted.x, y, counted.w, 15.0), "Stage III · 6 kinds", mono().color(theme::MUTED), Align::Right);
    y += 20.0;
    ui.text(counted.x, y, "C4 explored next", small().color(theme::TEXT_2));
    dots(ui, counted.x + 124.0, y, 5, INVADER, "3 of 6");
    y += 18.0;
    ui.text(counted.x, y, "Prussia L2: a Stage III card sits 3rd", small().color(theme::MUTED));
    y += 26.0;
    ui.text(counted.x, y, "THE EVENT", small().bold().color(INVADER));
    ui.text_in(Rect::new(counted.x, y, counted.w, 15.0), "59 in the deck", mono().color(theme::MUTED), Align::Right);
    y += 20.0;
    hatched(ui, Rect::new(counted.x, y, 110.0, 14.0), "not yet counted");

    // ---- Side column: budgets as instruments.
    let mut sy = side.y;
    let inst = |ui: &mut Ui, sy: &mut f32, h: f32, name: &str, count: &str| -> Rect {
        let r = Rect::new(side.x, *sy, side.w, h);
        *sy += h + gutter;
        instrument(ui, r, name, count)
    };
    let r = inst(ui, &mut sy, 92.0, "BLIGHT CARD", "healthy");
    slots(ui, r.x, r.y, 3, 3, INVADER);
    ui.text(r.x + 44.0, r.y - 1.0, "3 left", mono());
    ui.text(r.x, r.y + 20.0, "this Ravage −3 → flips", small().color(theme::TEXT_2));
    ui.text(r.x, r.y + 36.0, "other side: 2–10 · 23 cards", small().color(theme::MUTED));
    let r = inst(ui, &mut sy, 96.0, "FEAR", "4 / 5");
    for i in 0..5 {
        let c = (r.x + 9.0 + i as f32 * 23.0, r.y + 9.0);
        ui.circle(c, 7.0, if i < 4 { SPIRIT.alpha(80) } else { theme::TRANSPARENT }, SPIRIT.alpha(if i < 4 { 230 } else { 110 }), 1.2);
        ui.icon(Rect::new(c.0 - 5.0, c.1 - 5.0, 10.0, 10.0), icons::FEAR, if i < 4 { theme::TEXT } else { SPIRIT.alpha(120) });
    }
    ui.text(r.x, r.y + 24.0, "1 banked · Terror I: 1 card left", small().color(theme::TEXT_2));
    ui.text(r.x, r.y + 40.0, "deck 1 · 3 · 3 → 31 to victory", small().color(theme::MUTED));
    let r = inst(ui, &mut sy, 78.0, "INVADER DECK", "9 cards");
    let stages = [3u8, 2, 2, 2, 2, 3, 3, 3, 3];
    for (i, st) in stages.iter().enumerate() {
        let rr = Rect::new(r.x + i as f32 * 22.0, r.y, 16.0, 22.0);
        let c = if *st == 3 { Rgba::hex(0xfc8181) } else { Rgba::hex(0xf6ad55) };
        ui.rect(rr, c.alpha(if i == 0 { 120 } else { 50 }), c, 1.0, 3.0);
        ui.text_in(rr, if *st == 3 { "III" } else { "II" }, small().size(9.5).bold().color(theme::TEXT), Align::Center);
    }
    ui.text(r.x, r.y + 28.0, "out at T12's Explore", small().color(theme::MUTED));
    let r = inst(ui, &mut sy, 118.0, "SHADOWS", "4 Presence");
    let kv = |ui: &mut Ui, y: f32, k: &str, v: &str| {
        ui.text(r.x, y, k, small().color(theme::TEXT_2));
        ui.text_in(Rect::new(r.x, y, r.w, 15.0), v, mono(), Align::Right);
    };
    kv(ui, r.y, "Energy", "3  (+1)");
    kv(ui, r.y + 17.0, "plays", "2");
    kv(ui, r.y + 34.0, "hand · discard", "1 · 4");
    kv(ui, r.y + 51.0, "Fire · Moon · Air", "0 · 0 · 0");
    kv(ui, r.y + 68.0, "innate L1 needs", "Fire 1 · Moon 2");
    let r = inst(ui, &mut sy, 96.0, "PRESENCE", "C4 ×2 · C5 · E8");
    ui.text(r.x, r.y, "C4 is a Sacred Site", small().color(theme::TEXT_2));
    ui.text(r.x, r.y + 16.0, "Blight on C4 destroys 1", small().color(theme::TEXT_2));
    ui.text(r.x, r.y + 32.0, "Blight on C5 or E8 destroys 1", small().color(theme::TEXT_2));
    ui.text(r.x, r.y + 48.0, "no Presence clock in play", small().color(theme::MUTED));
    let rest = side.bottom() - sy;
    let r = inst(ui, &mut sy, rest, "RULES IN PLAY", "3");
    ui.text(r.x, r.y, "Prussia L2 · Fast Start", small().bold().color(theme::TEXT_2));
    ui.text(r.x, r.y + 15.0, "Stage III card 3rd in the deck", small().color(theme::MUTED));
    ui.text(r.x, r.y + 36.0, "Surge of Colonists", small().bold().color(theme::TEXT_2));
    ui.text(r.x, r.y + 51.0, "Fear 5 per card: two boards", small().color(theme::MUTED));
    ui.text(r.x, r.y + 72.0, "Land Rush (Escalation)", small().bold().color(theme::TEXT_2));
    ui.text(r.x, r.y + 87.0, "on Stage II cards only", small().color(theme::MUTED));
    let _ = REF;
}
