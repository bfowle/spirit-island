//! The versus screen (Brett, 2026-10-19): the Spirit on the left, the
//! Invaders on the right, the game's art dimmed behind each, numbers and
//! charts from the chain model in the middle. Little text: panel titles in
//! game words, a tooltip on hover for what a number is.
//!
//! ```text
//! tabs (one per Spirit)      TURN 3 · SPIRIT PHASE      adversary · ‹ ›
//! ┌ engine (Energy, plays) ┐ ┌ pace: Invaders vs you ┐ ┌ pressure per turn ┐
//! ├ what your plays stop   ┤ ├ how far behind       ┤ ├ what each action   ┤
//! ├ this turn │ innate     ┤ ├ backlog              ┤ │ plants if left     │
//! └───────────┴────────────┘ └ which ending first   ┘ ├ towns as seeds │ budgets ┘
//! ```

use super::fmt;
use super::{Action, ViewState};
use si_common::companion_stats::chain::Chain;
use si_common::companion_stats::{curve, CompanionStats, RoundSummary};
use si_common::public::{PublicSnapshot, PublicSpirit};
use si_steam::companion::CompanionState;
use si_steam::source::ReplayCommand;
use si_ui::theme::{self, Rgba, TextStyle};
use si_ui::{Align, Pt, Rect, Ui};

// Chart colors: Explore, Build, Ravage; the Spirit's line; gold, sky.
const X: Rgba = Rgba::hex(0x4fd1c5);
const B: Rgba = Rgba::hex(0xe0b45a);
const R: Rgba = Rgba::hex(0xf0885f);
const VIO: Rgba = theme::ACCENT;
const GOLD: Rgba = Rgba::hex(0xf6c453);
const SKY: Rgba = Rgba::hex(0x7cc4ff);
const DIM: Rgba = theme::MUTED;
const CARD: Rgba = Rgba(14, 15, 20, 236);

const MID_W: f32 = 320.0;
const TOP_H: f32 = 44.0;

const HEAD: TextStyle = TextStyle { size: 10.5, line: 14.0, mono: false, weight: 600, color: Rgba::hex(0x9aa0b2) };
const LABEL: TextStyle = TextStyle { size: 9.5, line: 12.0, mono: false, weight: 600, color: Rgba::hex(0x9aa0b2) };
const TICK: TextStyle = TextStyle { size: 10.5, line: 13.0, mono: true, weight: 400, color: theme::MUTED };
const NUM: TextStyle = TextStyle { size: 12.5, line: 16.0, mono: true, weight: 400, color: Rgba::hex(0xcfd2db) };
const BIG: TextStyle = TextStyle { size: 24.0, line: 26.0, mono: true, weight: 700, color: theme::TEXT };
const HUGE: TextStyle = TextStyle { size: 30.0, line: 32.0, mono: true, weight: 700, color: theme::TEXT };

/// The spirit art id for a tab, the adversary's for the right side.
pub fn art_key_spirit(sp: &PublicSpirit) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(a) = &sp.aspect {
        keys.push(format!("Sprite_{}Back_{}", sp.id, a));
    }
    keys.push(format!("Sprite_{}Back", sp.id));
    keys
}

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, history: &[RoundSummary], v: &mut ViewState, st: &CompanionState, actions: &mut Vec<Action>) {
    let mut r = r;
    let top = r.cut_top(TOP_H);
    let side_w = ((r.w - MID_W) / 2.0).floor();
    let mut rest = r;
    let left = rest.cut_left(side_w);
    let right = rest.cut_right(side_w);
    let mid = rest;

    if v.tab >= s.spirits.len() {
        v.tab = 0;
    }
    let sp = &s.spirits[v.tab.min(s.spirits.len().saturating_sub(1))];
    let ch = &stats.chain;

    // The art on the base layer; everything else on the map layer above it
    // (within a layer, shapes draw before icons), the dimming first.
    ui.layer(si_ui::LAYER_BASE);
    backdrop(ui, left, art_key_spirit(sp).iter().find_map(|k| v.art.get(k).copied()), (0.0, 0.5), 1.15);
    // The adversary's Invader-phase art where the game has one, else its panel.
    let adv_art = s.adversaries.first().and_then(|a| {
        let phase = match a.id.as_str() {
            "HabsburgMiningExpedition" => Some("Sprite_PhaseInvaderHabsburgMining"),
            "TheKingdomOfEngland" => Some("Sprite_PhaseInvaderEngland"),
            _ => None,
        };
        phase.and_then(|k| v.art.get(k).copied()).or_else(|| v.art.get(&format!("Sprite_{}", a.id)).copied())
    });
    backdrop(ui, right, adv_art, (1.0, 0.5), 1.15);
    ui.layer(si_ui::LAYER_MAP);
    // The art kept to a glow at the outer edges: dimmed overall, then faded
    // out toward the middle and the top, where the cards and text sit.
    for (col, outer_left) in [(left, true), (right, false)] {
        ui.fill(col, Rgba(11, 12, 16, 150), 0.0);
        let clear = Rgba(11, 12, 16, 0);
        let dark = Rgba(11, 12, 16, 235);
        let (o, i) = if outer_left { (clear, dark) } else { (dark, clear) };
        ui.quad([(col.x, col.y), (col.right(), col.y), (col.right(), col.bottom()), (col.x, col.bottom())], [o, i, i, o]);
        let top = Rect::new(col.x, col.y, col.w, col.h * 0.42);
        ui.quad([(top.x, top.y), (top.right(), top.y), (top.right(), top.bottom()), (top.x, top.bottom())], [Rgba(11, 12, 16, 210), Rgba(11, 12, 16, 210), clear, clear]);
    }
    ui.fill(mid, Rgba::hex(0x0f1016), 0.0);
    ui.fill(Rect::new(mid.x, mid.y, 1.0, mid.h), theme::BORDER, 0.0);
    ui.fill(Rect::new(mid.right() - 1.0, mid.y, 1.0, mid.h), theme::BORDER, 0.0);

    top_bar(ui, top, s, v, st, actions);
    spirit_side(ui, left.inset(16.0, 14.0, 16.0, 14.0), s, sp, stats, history, v);
    middle(ui, mid.inset(10.0, 8.0, 10.0, 14.0), s, stats, history);
    invader_side(ui, right.inset(16.0, 14.0, 16.0, 14.0), s, stats, ch, v);
}

/// The game's art behind a column, scaled to cover it (times `zoom`) and
/// anchored at `anchor` (fractions of the column and of the image: (0, 0.5)
/// keeps the image's left edge on the column's left, centered vertically).
fn backdrop(ui: &mut Ui, col: Rect, art: Option<crate::icons::Art>, anchor: (f32, f32), zoom: f32) {
    // No fill under the art: within a layer, shapes draw after images.
    let Some(a) = art else { return };
    ui.push_clip(col);
    let scale = (col.w / a.w as f32).max(col.h / a.h as f32) * zoom;
    let (w, h) = (a.w as f32 * scale, a.h as f32 * scale);
    let x = col.x + (col.w - w) * anchor.0;
    let y = col.y + (col.h - h) * anchor.1;
    ui.image(Rect::new(x, y, w, h), a.id, 0.55);
    ui.pop_clip();
}

fn top_bar(ui: &mut Ui, r: Rect, s: &PublicSnapshot, v: &mut ViewState, st: &CompanionState, actions: &mut Vec<Action>) {
    ui.fill(r, Rgba::hex(0x13141a), 0.0);
    ui.fill(Rect::new(r.x, r.bottom() - 1.0, r.w, 1.0), theme::BORDER, 0.0);
    // One tab per Spirit.
    let mut x = r.x + 10.0;
    for (i, sp) in s.spirits.iter().enumerate() {
        let on = i == v.tab;
        let style = TextStyle { size: 12.5, line: 16.0, mono: false, weight: if on { 600 } else { 500 }, color: if on { theme::TEXT } else { theme::TEXT_2 } };
        let w = ui.measure(&sp.title, style).0 + 24.0;
        let tab = Rect::new(x, r.y + 8.0, w, 28.0);
        ui.rect(tab, if on { VIO.alpha(46) } else { theme::TRANSPARENT }, if on { VIO } else { theme::BORDER_STRONG }, 1.0, 6.0);
        ui.text_in(tab, &sp.title, style, Align::Center);
        if ui.clicked(tab) {
            v.tab = i;
        }
        x += w + 8.0;
    }
    // The moment, and whether the companion joined the game late.
    let step = super::STEPS[super::step_index(&s.step)].to_uppercase();
    let moment = match s.ending.as_deref() {
        Some(e) => {
            let (won, how) = ending_words(e);
            format!("TURN {} · GAME OVER · {} · {}", s.turn, if won { "YOU WON" } else { "YOU LOST" }, how.to_uppercase())
        }
        None => format!("TURN {} · {} PHASE", s.turn, step),
    };
    let mid = Rect::new(r.x + (r.w - MID_W) / 2.0, r.y, MID_W, r.h);
    let first_seen = st.history.first().map(|h| h.turn).unwrap_or(1);
    if first_seen > 1 {
        ui.text_in(Rect::new(mid.x, r.y + 4.0, mid.w, 20.0), &moment, TextStyle { size: 14.0, line: 18.0, mono: false, weight: 600, color: theme::TEXT }, Align::Center);
        let note = format!("joined on T{first_seen} · T1–T{} from the cards only", first_seen - 1);
        ui.text_in(Rect::new(mid.x, r.y + 24.0, mid.w, 16.0), &note, theme::SMALL.color(theme::AMBER), Align::Center);
    } else {
        ui.text_in(mid, &moment, TextStyle { size: 14.0, line: 18.0, mono: false, weight: 600, color: theme::TEXT }, Align::Center);
    }
    // The adversary, and the step buttons in a replay.
    let mut right = Rect::new(r.right() - 10.0, r.y + 8.0, 0.0, 28.0);
    if st.source.replay.is_some() {
        for (label, delta) in [("›", 1i64), ("‹", -1i64)] {
            let b = Rect::new(right.x - 30.0, right.y, 30.0, 28.0);
            ui.rect(b, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, 6.0);
            ui.text_in(b, label, TextStyle { size: 16.0, line: 18.0, mono: false, weight: 500, color: theme::TEXT_2 }, Align::Center);
            if ui.clicked(b) {
                actions.push(Action::Replay(ReplayCommand::Step(delta)));
            }
            right.x -= 34.0;
        }
        right.x -= 12.0;
    }
    let adv = s.adversaries.iter().map(|a| format!("{} · Level {}", a.title, a.level)).collect::<Vec<_>>().join(" + ");
    let style = TextStyle { size: 14.0, line: 18.0, mono: false, weight: 500, color: theme::TEXT };
    let w = ui.measure(&adv, style).0;
    ui.text_in(Rect::new(right.x - w, r.y, w, r.h), &adv, style, Align::Left);
}

// ---------------------------------------------------------------- cards

/// A translucent card with a title and, when `about` is given, a small (i)
/// at its top right that opens the explanation on hover. Returns the
/// content rectangle.
fn card(ui: &mut Ui, r: Rect, title: &str, about: &str) -> Rect {
    ui.rect(r, CARD, Rgba(255, 255, 255, 20), 1.0, 8.0);
    let inner = r.inset(12.0, 8.0, 12.0, 8.0);
    let title_w = if about.is_empty() { inner.w } else { inner.w - 22.0 };
    let (_, h) = ui.text_wrap(inner.x, inner.y, title_w, &title.to_uppercase(), HEAD);
    if !about.is_empty() {
        let info = Rect::new(inner.right() - 15.0, inner.y - 1.0, 15.0, 15.0);
        let hover = ui.hovered(info);
        ui.circle((info.x + 7.5, info.y + 7.5), 7.0, if hover { VIO.alpha(70) } else { theme::TRANSPARENT }, if hover { VIO } else { theme::BORDER_STRONG }, 1.0);
        ui.text_in(info, "i", TextStyle { size: 10.0, line: 12.0, mono: false, weight: 700, color: if hover { theme::TEXT } else { theme::MUTED } }, Align::Center);
        if hover {
            ui.wants_hand = true;
            bubble(ui, info, about);
        }
    }
    Rect::new(inner.x, inner.y + h + 2.0, inner.w, (inner.h - h - 2.0).max(0.0))
}

/// The explanation in game words, in a box under an (i).
fn bubble(ui: &mut Ui, at: Rect, text: &str) {
    let w = 300.0;
    ui.layer(si_ui::LAYER_OVERLAY);
    let (_, h) = ui.measure_spans(&[si_ui::span(text, theme::SMALL.color(theme::TEXT))], Some(w - 24.0));
    let x = (at.right() - w).max(ui.screen.x + 8.0);
    let y = (at.bottom() + 6.0).min(ui.screen.bottom() - h - 24.0);
    let box_r = Rect::new(x, y, w, h + 16.0);
    ui.rect(box_r, Rgba::hex(0x1a1b24), VIO, 1.0, 8.0);
    ui.text_wrap(x + 12.0, y + 8.0, w - 24.0, text, theme::SMALL.color(theme::TEXT));
    ui.layer(si_ui::LAYER_MAP);
}

/// The engine's ending as a result and its reason, in game words.
fn ending_words(ending: &str) -> (bool, &'static str) {
    match ending {
        "WinInvadersDestroyed" => (true, "board clear"),
        "WinTerrorVictory" => (true, "Fear victory"),
        e if e.starts_with("Win") => (true, "victory"),
        "LoseTooMuchBlight" => (false, "Blight"),
        "LoseAdversary" => (false, "the adversary's condition"),
        "LoseNoPresence" => (false, "no Presence left"),
        e if e.contains("InvaderDeck") || e.contains("InvaderCards") => (false, "the Invader deck ran out"),
        _ => (false, "defeat"),
    }
}

/// A sentence or two under a chart, in the game's words: what to make of it.
fn caption(ui: &mut Ui, r: Rect, text: &str) {
    ui.push_clip(r);
    ui.text_wrap(r.x, r.y, r.w, text, theme::SMALL.color(theme::TEXT_2));
    ui.pop_clip();
}

/// A small label over a big number.
fn stat(ui: &mut Ui, x: f32, y: f32, label: &str, value: &str, color: Rgba, style: TextStyle) -> f32 {
    ui.text(x, y, label, LABEL);
    let (w, _) = ui.text(x, y + 13.0, value, style.color(color));
    w.max(ui.measure(label, LABEL).0)
}

// ---------------------------------------------------------------- charts

struct Axis {
    r: Rect,
    n: usize,
    ymax: f32,
}

impl Axis {
    /// Center x of column `i`.
    fn cx(&self, i: usize) -> f32 {
        self.r.x + (i as f32 + 0.5) * self.r.w / self.n as f32
    }
    fn lx(&self, i: usize) -> f32 {
        self.r.x + i as f32 * self.r.w / (self.n.max(2) - 1) as f32
    }
    fn py(&self, v: f32) -> f32 {
        self.r.bottom() - (v.min(self.ymax) / self.ymax.max(1e-6)) * self.r.h
    }
}

fn grid(ui: &mut Ui, a: &Axis, xs: &[u32], now: u32, centered: bool) {
    let step = if a.ymax > 300.0 { 100.0 } else if a.ymax > 120.0 { 50.0 } else if a.ymax > 25.0 { 10.0 } else if a.ymax > 8.0 { 2.0 } else { 1.0 };
    let mut v = step;
    while v < a.ymax {
        let y = a.py(v);
        ui.line((a.r.x, y), (a.r.right(), y), 1.0, theme::BORDER.alpha(150), 0.0);
        ui.text_in(Rect::new(a.r.x - 30.0, y - 7.0, 26.0, 14.0), &format!("{v:.0}"), TICK, Align::Right);
        v += step;
    }
    for (i, t) in xs.iter().enumerate() {
        let x = if centered { a.cx(i) } else { a.lx(i) };
        let style = if *t == now { TICK.color(theme::TEXT).bold() } else { TICK };
        ui.text_in(Rect::new(x - 18.0, a.r.bottom() + 4.0, 36.0, 14.0), &format!("T{t}"), style, Align::Center);
    }
}

/// Stacked columns per turn (the current one framed), an optional line over them.
fn stacked(ui: &mut Ui, r: Rect, xs: &[u32], parts: &[&[f32]], colors: &[Rgba], alpha: &[f32], now: u32, line: Option<&[f32]>, ymax: f32) {
    let a = Axis { r: r.inset(32.0, 10.0, 8.0, 20.0), n: xs.len().max(1), ymax };
    grid(ui, &a, xs, now, true);
    let bw = a.r.w / a.n as f32 * 0.6;
    for i in 0..xs.len() {
        let mut y0 = 0.0;
        let al = alpha.get(i).copied().unwrap_or(1.0);
        for (p, c) in parts.iter().zip(colors) {
            let v = p.get(i).copied().unwrap_or(0.0);
            if v <= 0.0 {
                continue;
            }
            let top = a.py(y0 + v);
            let bot = a.py(y0);
            ui.fill(Rect::new(a.cx(i) - bw / 2.0, top, bw, (bot - top).max(1.0)), c.alpha((255.0 * al) as u8), 1.5);
            y0 += v;
        }
        if y0 > 0.0 {
            ui.text_in(Rect::new(a.cx(i) - 20.0, a.py(y0) - 16.0, 40.0, 14.0), &format!("{y0:.0}"), TICK.color(theme::TEXT), Align::Center);
        }
        if xs[i] == now {
            ui.rect(Rect::new(a.cx(i) - bw / 2.0 - 5.0, a.r.y - 4.0, bw + 10.0, a.r.h + 8.0), theme::TRANSPARENT, VIO.alpha(200), 1.0, 3.0);
        }
    }
    if let Some(l) = line {
        let pts: Vec<Pt> = l.iter().enumerate().map(|(i, v)| (a.cx(i), a.py(*v))).collect();
        super::chart::glow_line(ui, &pts, 2.5, VIO);
        for p in &pts {
            ui.circle(*p, 3.0, VIO, theme::TRANSPARENT, 0.0);
        }
    }
}

struct Series<'a> {
    values: &'a [Option<f32>],
    color: Rgba,
    label: &'a str,
    /// Dashed past the current turn.
    project: bool,
    faint: bool,
}

/// Lines per turn: solid so far, dashed ahead; a shaded fill under one series.
fn lines(ui: &mut Ui, r: Rect, xs: &[u32], series: &[Series], now: u32, ymax: f32, hline: Option<f32>, marks: &[usize], fill_below: Option<usize>) {
    let a = Axis { r: r.inset(32.0, 10.0, 48.0, 20.0), n: xs.len().max(2), ymax };
    grid(ui, &a, xs, now, false);
    let ni = xs.iter().position(|t| *t == now).unwrap_or(0);
    ui.fill(Rect::new(a.lx(ni) - 7.0, a.r.y - 4.0, 14.0, a.r.h + 8.0), VIO.alpha(34), 2.0);
    if let Some(h) = hline {
        let y = a.py(h);
        super::chart::dashed(ui, &[(a.r.x, y), (a.r.right(), y)], 1.0, theme::GREEN, 3.0, 3.0);
        ui.text(a.r.right() + 4.0, y - 7.0, &format!("{h:.0}"), TICK.color(theme::GREEN));
    }
    for i in marks {
        let x = a.lx(*i);
        let yb = a.r.bottom() + 2.0;
        ui.triangle((x - 5.0, yb), (x + 5.0, yb), (x, yb - 8.0), R);
    }
    let mut labels: Vec<(f32, String, Rgba)> = Vec::new();
    for (k, s) in series.iter().enumerate() {
        let col = if s.faint { s.color.alpha(120) } else { s.color };
        let pts: Vec<Option<Pt>> = s.values.iter().enumerate().map(|(i, v)| v.map(|v| (a.lx(i), a.py(v)))).collect();
        if fill_below == Some(k) {
            let present: Vec<Pt> = pts.iter().flatten().copied().collect();
            let base: Vec<Pt> = present.iter().map(|p| (p.0, a.r.bottom())).collect();
            super::chart::band(ui, &present, &base, s.color.alpha(60), s.color.alpha(8));
        }
        let mut solid = Vec::new();
        let mut dash = Vec::new();
        for (i, p) in pts.iter().enumerate() {
            if let Some(p) = p {
                if i <= ni || !s.project {
                    solid.push(*p);
                }
                if i >= ni && s.project {
                    dash.push(*p);
                }
            }
        }
        super::chart::polyline(ui, &solid, if s.faint { 1.5 } else { 2.5 }, col);
        super::chart::dashed(ui, &dash, if s.faint { 1.2 } else { 2.0 }, col, 5.0, 4.0);
        if let Some(last) = pts.iter().flatten().last() {
            let v = s.values.iter().flatten().last().copied().unwrap_or(0.0);
            let num = if v >= 10.0 { format!("{v:.0}") } else { format!("{v:.1}") };
            let text = if s.label.is_empty() { num } else { format!("{} {num}", s.label) };
            labels.push((last.1 - 7.0, text, col));
        }
        if let Some(Some(p)) = pts.get(ni) {
            ui.circle(*p, 3.5, col, theme::TRANSPARENT, 0.0);
        }
        let _ = k;
    }
    // End labels pushed apart so lines ending near the same value stay readable.
    labels.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut last_y = f32::MIN;
    for (y, text, col) in labels {
        let y = y.max(last_y + 12.0);
        ui.text(a.r.right() + 5.0, y, &text, TICK.color(col));
        last_y = y;
    }
}

/// Horizontal stacked bars with a label, a chance and a total.
fn hbars(ui: &mut Ui, r: Rect, rows: &[(String, String, [f32; 5], f32)], colors: &[Rgba; 5], row_h: f32) {
    let maxv = rows.iter().map(|x| x.3).fold(0.0f32, f32::max).max(1e-6);
    let lw = 110.0;
    let vw = 40.0;
    let bw = r.w - lw - 34.0 - vw - 6.0;
    for (j, (label, chance, parts, total)) in rows.iter().enumerate() {
        let y = r.y + j as f32 * row_h;
        if y + row_h > r.bottom() + 1.0 {
            break;
        }
        ui.text_in(Rect::new(r.x, y, lw, row_h), label, NUM.color(theme::TEXT), Align::Left);
        ui.text_in(Rect::new(r.x + lw, y, 34.0, row_h), chance, TICK, Align::Left);
        let mut x0 = r.x + lw + 34.0;
        for (p, c) in parts.iter().zip(colors) {
            let w = bw * p / maxv;
            if w > 0.5 {
                ui.fill(Rect::new(x0, y + 4.0, w, row_h - 8.0), *c, 0.0);
                x0 += w;
            }
        }
        ui.text_in(Rect::new(r.x + lw + 34.0 + bw + 6.0, y, vw, row_h), &format!("{total:.1}"), NUM.bold().color(theme::TEXT), Align::Left);
    }
}

/// `n` squares, `filled` of them colored, the last `struck` crossed in red,
/// `hatch` more outlined. Returns the width.
fn pips(ui: &mut Ui, x: f32, y: f32, n: usize, filled: usize, color: Rgba, struck: usize, hatch: usize) -> f32 {
    let (w, gap) = (10.0, 3.0);
    for i in 0..n {
        let px = x + i as f32 * (w + gap);
        let rr = Rect::new(px, y, w, w);
        if i < filled {
            ui.fill(rr, color, 2.0);
            if i + struck >= filled {
                ui.line((px + 2.0, y + 2.0), (px + w - 2.0, y + w - 2.0), 2.0, Rgba::hex(0xff6b4a), 0.0);
                ui.line((px + w - 2.0, y + 2.0), (px + 2.0, y + w - 2.0), 2.0, Rgba::hex(0xff6b4a), 0.0);
            }
        } else if i < filled + hatch {
            ui.rect(rr, theme::TRANSPARENT, color.alpha(180), 1.0, 2.0);
        } else {
            ui.fill(rr, theme::BORDER, 2.0);
        }
    }
    n as f32 * (w + gap)
}

// ---------------------------------------------------------------- the Spirit

fn spirit_side(ui: &mut Ui, r: Rect, s: &PublicSnapshot, sp: &PublicSpirit, stats: &CompanionStats, history: &[RoundSummary], v: &ViewState) {
    let ch = &stats.chain;
    let xs: Vec<u32> = ch.turns.iter().map(|t| t.turn).collect();
    let now = ch.turn;
    let mut col = r;
    let gap = 12.0;

    // 1. Energy and card plays per turn.
    // The two charts take the height; the bottom row is sized to its content.
    let ni = xs.iter().position(|t| *t == now).unwrap_or(0);
    let bottom_h = 236.0_f32.min(col.h * 0.3);
    let free = (col.h - bottom_h - 2.0 * gap).max(300.0);
    let engine_r = col.cut_top((free * 0.54).round());
    col.cut_top(gap);
    let body = card(ui, engine_r, "Energy and card plays per turn · solid: presence off the Card Plays track · faint: off the Energy track · bank: Energy kept · ● elements uncovered · ▲ Reclaim turn", "Energy a turn and card plays a turn, from the presence tracks. Past turns are what the tracks showed; ahead is one presence a turn off the Card Plays track (faint: off the Energy track). The shaded line is Energy banked at each turn's end; a dot is an element node uncovered that turn (presence that went to elements instead of Energy or plays). A Reclaim turn is one where the hand is short of the plays.");
    let (e_plays, p_plays) = route(sp, "CardPlaysPerTurn", &xs, now, history);
    let (e_energy, p_energy) = route(sp, "EnergyPerTurn", &xs, now, history);
    let marks = reclaim_turns(sp, &p_plays, &xs, now, &s.step);
    // Energy banked at each turn's end so far, and this turn's: where the
    // income went when the tracks stayed flat.
    let bank: Vec<Option<f32>> = xs
        .iter()
        .map(|t| {
            if *t < now {
                history.iter().find(|h| h.turn == *t).and_then(|h| h.spirits.iter().find(|x| x.id == sp.id)).map(|x| x.energy as f32)
            } else if *t == now {
                Some(sp.energy as f32)
            } else {
                None
            }
        })
        .collect();
    let ymax = e_plays.iter().chain(&p_plays).chain(&e_energy).chain(&p_energy).chain(&bank).flatten().fold(0.0f32, |m, v| m.max(*v)) + 1.0;
    let mut chart_r = body;
    let stats_r = chart_r.cut_bottom(44.0);
    let caption_r = chart_r.cut_bottom(36.0);
    {
        let income = si_common::companion_stats::spirits::track_value(sp, "EnergyPerTurn", true).unwrap_or(0);
        let plays_now = si_common::companion_stats::spirits::track_value(sp, "CardPlaysPerTurn", true).unwrap_or(0);
        let elements = sp.track.iter().filter(|x| x.revealed && x.kind.contains("Element")).count();
        let climb = p_plays.iter().enumerate().skip(ni + 1).find(|(_, v)| v.is_some_and(|v| v > plays_now as f32)).map(|(i, v)| (xs[i], v.unwrap()));
        let next = match climb {
            Some((t, v)) => format!(" Down the plays track, {v:.0} plays from T{t}."),
            None => String::new(),
        };
        let text = format!(
            "{income} Energy a turn and {plays_now} plays, {} banked. {} presence went to elements (the dots).{next}",
            sp.energy,
            if elements == 0 { "No".to_string() } else { elements.to_string() }
        );
        caption(ui, caption_r, &text);
    }
    lines(
        ui,
        chart_r,
        &xs,
        &[
            Series { values: &bank, color: GOLD, label: "bank", project: false, faint: true },
            Series { values: &e_plays, color: GOLD, label: "E", project: true, faint: false },
            Series { values: &p_plays, color: VIO, label: "P", project: true, faint: false },
            Series { values: &e_energy, color: GOLD, label: "E′", project: true, faint: true },
            Series { values: &p_energy, color: VIO, label: "P′", project: true, faint: true },
        ],
        now,
        ymax,
        None,
        &marks,
        Some(0),
    );
    // Elements the presence bought off the tracks instead, per turn so far.
    {
        let a = Axis { r: chart_r.inset(32.0, 10.0, 48.0, 20.0), n: xs.len().max(2), ymax };
        let mut prev = 0u32;
        for (i, t) in xs.iter().enumerate() {
            let n = if *t < now {
                history.iter().find(|h| h.turn == *t).and_then(|h| h.spirits.iter().find(|x| x.id == sp.id)).map(|x| x.track_elements)
            } else if *t == now {
                Some(sp.track.iter().filter(|x| x.revealed && x.kind.contains("Element")).count() as u32)
            } else {
                None
            };
            let Some(n) = n else { continue };
            for k in prev..n {
                let y = a.r.y + 6.0 + (k - prev) as f32 * 9.0;
                ui.circle((a.lx(i), y), 3.0, X, theme::TRANSPARENT, 0.0);
            }
            prev = n.max(prev);
        }
    }
    let plays_now = sp.card_plays.max(si_common::companion_stats::spirits::track_value(sp, "CardPlaysPerTurn", true).unwrap_or(0));
    let mut x = stats_r.x;
    for (label, value, color) in [
        ("ENERGY", sp.energy.to_string(), GOLD),
        ("PLAYS", plays_now.to_string(), VIO),
        ("HAND", sp.hand.len().to_string(), theme::TEXT),
        ("DISCARD", sp.discard.len().to_string(), theme::TEXT),
        ("PRESENCE", sp.presence_on_island.to_string(), theme::TEXT),
    ] {
        x += stat(ui, x, stats_r.y + 4.0, label, &value, color, BIG) + 22.0;
    }

    // 2. What the plays stop, per turn.
    let cap_r = col.cut_top(free - (free * 0.54).round());
    col.cut_top(gap);
    let body = card(ui, cap_r, "What your plays stop, per turn · past: measured · now: best plays · ahead: at this rate", "Each Invader action is worth what it plants (its chain). This is how much of that your plays stop each turn: past turns measured from the game (what was set to act, less what happened), this turn the best legal plays against the Ravages and Builds on the table, ahead the plays a turn at this turn's rate.");
    let capacity: Vec<f32> = ch.turns.iter().map(|t| t.capacity as f32).collect();
    let alpha: Vec<f32> = ch.turns.iter().map(|t| basis_alpha(&t.basis)).collect();
    let ymax = capacity.iter().fold(0.0f32, |m, v| m.max(*v)) * 1.3;
    let mut body = body;
    let caption_r = body.cut_bottom(36.0);
    {
        let last = ni.checked_sub(1).and_then(|i| ch.turns.get(i));
        let nowt = &ch.turns[ni];
        let mut cuts: Vec<String> = ch.capacity_now.iter().flat_map(|c| c.cuts.iter()).map(|(card, target, _)| {
            let t: Vec<&str> = target.split(' ').collect();
            format!("{card} on the {} {}", t.get(1).map(|id| fmt::short(s, id)).unwrap_or_default(), t[0])
        }).collect();
        cuts.truncate(2);
        let text = match last {
            Some(l) => format!(
                "Last turn your plays stopped {:.0} of the Invaders' {:.0}; this turn's best is {:.0} of {:.0}{}.",
                l.capacity, l.pressure, nowt.capacity, nowt.pressure,
                if cuts.is_empty() { String::new() } else { format!(" ({})", cuts.join(", ")) }
            ),
            None => format!("This turn's best plays stop {:.0} of the Invaders' {:.0}.", nowt.capacity, nowt.pressure),
        };
        caption(ui, caption_r, &text);
    }
    stacked(ui, body, &xs, &[&capacity], &[VIO], &alpha, now, None, ymax.max(6.0));
    // Turns rebuilt from the game's log have no "set to act" to measure against.
    let a = Axis { r: body.inset(32.0, 10.0, 8.0, 20.0), n: xs.len().max(1), ymax: ymax.max(6.0) };
    for (i, t) in xs.iter().enumerate() {
        if history.iter().any(|h| h.turn == *t && h.from_log) {
            ui.text_in(Rect::new(a.cx(i) - 20.0, a.r.bottom() - 16.0, 40.0, 14.0), "log", TICK.color(DIM), Align::Center);
        }
    }

    // 3. This turn's best plays, and the innate.
    let bottom = col;
    let mut left = bottom;
    let right = left.cut_right((bottom.w - gap) / 2.0);
    left.cut_right(gap);
    let body = card(ui, left, "This turn · best plays", "The most chain value the Spirits' legal plays can stop this turn, against this turn's Ravages and Builds and next turn's Ravages: each card to one action, within plays and Energy, innate levels the elements reach included.");
    let cap_now: f64 = ch.capacity_now.iter().map(|c| c.best).sum();
    let plays_all: i64 = ch.capacity_now.iter().map(|c| c.plays).sum();
    let pressure_now = ch.turns.iter().find(|t| t.turn == now).map(|t| t.pressure).unwrap_or(0.0);
    let (w, _) = ui.text(body.x, body.y + 2.0, &format!("{cap_now:.1}"), HUGE.color(VIO));
    ui.text(body.x + w + 8.0, body.y + 14.0, &format!("of {pressure_now:.1} · {plays_all} plays"), theme::SMALL);
    let mut y = body.y + 42.0;
    for c in &ch.capacity_now {
        let who = c.spirit.split(' ').next().unwrap_or("").to_string();
        for (cardname, target, value) in &c.cuts {
            let tgt = target.split(' ').collect::<Vec<_>>();
            let land = tgt.get(1).map(|id| fmt::short(s, id)).unwrap_or_default();
            let more = if target.contains("next turn") { " next turn" } else { "" };
            let what = format!("{} {land}{more}", tgt[0]);
            let worth = format!("  {value:.1}");
            let spans = [
                si_ui::span(&who, theme::SMALL),
                si_ui::span("  ", theme::SMALL),
                si_ui::span(cardname, NUM),
                si_ui::span("  → ", theme::SMALL),
                si_ui::span(&what, NUM.color(R)),
                si_ui::span(&worth, NUM.bold()),
            ];
            let (_, h) = ui.spans(body.x, y, &spans, Some(body.w));
            y += h + 2.0;
        }
    }
    if y == body.y + 42.0 {
        ui.text(body.x, y, "no play stops an action this turn", theme::SMALL.color(DIM));
    }

    innate_card(ui, right, sp, v);
}

fn basis_alpha(basis: &str) -> f32 {
    if basis == "played" {
        1.0
    } else if basis.starts_with("over") {
        0.85
    } else {
        0.45
    }
}

/// A Spirit's Energy and plays per turn so far (from the history), then
/// along one track, one presence a turn.
fn route(sp: &PublicSpirit, kind: &str, xs: &[u32], now: u32, history: &[RoundSummary]) -> (Vec<Option<f32>>, Vec<Option<f32>>) {
    let mut e = Vec::new();
    let mut p = Vec::new();
    for t in xs {
        if *t < now {
            let row = history.iter().find(|h| h.turn == *t).and_then(|h| h.spirits.iter().find(|x| x.id == sp.id));
            e.push(row.map(|x| x.energy_income as f32));
            p.push(row.map(|x| x.plays as f32));
        } else {
            let (ee, pp) = curve::projected(sp, kind, (*t - now) as usize);
            e.push(Some(ee as f32));
            p.push(Some(pp as f32));
        }
    }
    (e, p)
}

/// The turns the hand runs short of the plays along `plays`.
fn reclaim_turns(sp: &PublicSpirit, plays: &[Option<f32>], xs: &[u32], now: u32, step: &str) -> Vec<usize> {
    let before_plays = matches!(step, "PlayerSetup" | "InvadersStartingAction" | "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower");
    let mut hand = sp.hand.len() as i64;
    let mut disc = (sp.discard.len() + sp.play_area.len()) as i64;
    let mut out = Vec::new();
    for (k, t) in xs.iter().enumerate() {
        if *t < now {
            continue;
        }
        let p = plays.get(k).copied().flatten().unwrap_or(0.0) as i64;
        if *t > now {
            hand += 1;
        } else if !before_plays {
            continue;
        }
        if hand < p {
            out.push(k);
            hand += disc - i64::from(*t > now);
            disc = 0;
        }
        let played = p.min(hand).max(0);
        hand -= played;
        disc += played;
    }
    out
}

fn innate_card(ui: &mut Ui, r: Rect, sp: &PublicSpirit, _v: &ViewState) {
    let title = sp.innate_powers.first().map(|i| format!("{} · {} · range {}", i.title, i.speed, i.range.map(|r| r.to_string()).unwrap_or_default())).unwrap_or_else(|| "Innate".into());
    let body = card(ui, r, &format!("{title} · elements you have / need"), "The innate's levels: for each element, what the cards in play and the tracks give, against what the level needs. Green: met.");
    let mut y = body.y;
    for i in sp.innate_powers.iter().take(2) {
        if sp.innate_powers.len() > 1 {
            ui.text(body.x, y, &i.title, LABEL);
            y += 14.0;
        }
        for (li, lv) in i.levels.iter().take(3).enumerate() {
            let mut spans = vec![si_ui::span("", NUM)];
            let label = format!("L{}  ", li + 1);
            spans[0] = si_ui::span(&label, NUM.color(DIM));
            let mut parts: Vec<(String, Rgba)> = Vec::new();
            for (el, need) in &lv.elements {
                let have = sp.elements.get(el).copied().unwrap_or(0);
                parts.push((format!("{} {have}/{need}", &el[..el.len().min(2)]), if have >= *need { theme::GREEN } else { R }));
            }
            let text: Vec<(String, Rgba)> = parts;
            let mut all: Vec<si_ui::Span> = vec![spans[0]];
            let joined: Vec<String> = text.iter().map(|(t, _)| t.clone()).collect();
            let seps: Vec<String> = (0..joined.len()).map(|k| if k + 1 < joined.len() { " · ".to_string() } else { String::new() }).collect();
            for (k, (t, c)) in text.iter().enumerate() {
                all.push(si_ui::span(t, NUM.color(*c)));
                all.push(si_ui::span(&seps[k], NUM.color(DIM)));
            }
            let (_, h) = ui.spans(body.x, y, &all, Some(body.w));
            y += h + 2.0;
        }
        y += 6.0;
    }
    // The hand and the discard, as names with cost and speed.
    for (label, cards) in [("HAND", &sp.hand), ("DISCARD", &sp.discard)] {
        ui.text(body.x, y, label, LABEL);
        y += 13.0;
        let text = if cards.is_empty() {
            "—".to_string()
        } else {
            cards.iter().map(|c| format!("{} {}{}", c.title.split(' ').next().unwrap_or(""), c.cost, &c.speed[..1])).collect::<Vec<_>>().join(" · ")
        };
        let (_, h) = ui.text_wrap(body.x, y, body.w, &text, NUM);
        y += h + 6.0;
    }
}

// ---------------------------------------------------------------- the middle

/// The board's own account, turn by turn: Invader strength on the island
/// (Explorer 1, Town 2, City 3), Blight, Dahan. What "keeping up" feels
/// like at the table, with no model in between.
fn board_card(ui: &mut Ui, r: Rect, s: &PublicSnapshot, history: &[RoundSummary], xs: &[u32], now: u32) {
    let mut body = card(ui, r, "Are you keeping up · the board, turn by turn", "Invader strength on the island at each turn's end (an Explorer 1, a Town 2, a City 3), Blight on the island, and Dahan. Flat or falling strength and flat Blight is keeping up, whatever the Invaders tried. This counts what you remove after it lands; the pace charts below count only what you stop before it happens.");
    let caption_r = body.cut_bottom(48.0);
    let row = |t: u32| history.iter().find(|h| h.turn == t);
    let load_of = |x: u32, tw: u32, c: u32| (x + 2 * tw + 3 * c) as f32;
    let island: Vec<&si_common::public::PublicLand> = s.lands.iter().filter(|l| !l.is_ocean).collect();
    let now_load = load_of(island.iter().map(|l| l.explorers).sum(), island.iter().map(|l| l.towns).sum(), island.iter().map(|l| l.cities).sum());
    let now_blight = s.blight.on_island as f32;
    let now_dahan = island.iter().map(|l| l.dahan).sum::<u32>() as f32;
    let series = |f: &dyn Fn(&RoundSummary) -> f32, now_v: f32| -> Vec<Option<f32>> {
        xs.iter().map(|t| if *t < now { row(*t).map(f) } else if *t == now { Some(now_v) } else { None }).collect()
    };
    let load = series(&|h| load_of(h.explorers, h.towns, h.cities), now_load);
    let blight = series(&|h| h.blight_on_island as f32, now_blight);
    let dahan = series(&|h| h.dahan as f32, now_dahan);
    let ymax = load.iter().chain(&dahan).flatten().fold(0.0f32, |m, v| m.max(*v)) * 1.15 + 1.0;
    lines(
        ui,
        body,
        xs,
        &[
            Series { values: &load, color: B, label: "Invaders", project: false, faint: false },
            Series { values: &blight, color: VIO, label: "Blight", project: false, faint: false },
            Series { values: &dahan, color: SKY, label: "Dahan", project: false, faint: true },
        ],
        now,
        ymax,
        None,
        &[],
        Some(0),
    );
    // The verdict: the last three turns against the three before.
    let at = |back: u32| now.checked_sub(back).and_then(|t| row(t)).map(|h| (load_of(h.explorers, h.towns, h.cities), h.blight_on_island as f32));
    let text = match at(3) {
        _ if s.ending.is_some() => {
            let (won, how) = ending_words(s.ending.as_deref().unwrap_or(""));
            format!("{} on T{now}: {how}. Invaders ended at {now_load:.0}, Blight at {now_blight:.0}.", if won { "Won" } else { "Lost" })
        }
        Some((l3, b3)) => {
            let dl = now_load - l3;
            let db = now_blight - b3;
            let strength = if dl >= 3.0 {
                format!("Invaders up {dl:.0} in three turns ({l3:.0} → {now_load:.0})")
            } else if dl <= -3.0 {
                format!("Invaders down {:.0} in three turns ({l3:.0} → {now_load:.0})", -dl)
            } else {
                format!("Invaders holding at about {now_load:.0} for three turns")
            };
            let blight_s = if db > 0.0 {
                format!("Blight up {db:.0} ({b3:.0} → {now_blight:.0})")
            } else if db < 0.0 {
                format!("Blight down to {now_blight:.0}")
            } else {
                format!("Blight flat at {now_blight:.0}")
            };
            let verdict = if dl < 3.0 && db <= 0.0 { "Keeping up." } else if dl >= 6.0 || db >= 2.0 { "Slipping." } else { "Holding, barely." };
            format!("{verdict} {strength}; {blight_s}.")
        }
        None => format!("Invaders at {now_load:.0}, Blight {now_blight:.0}; three turns of play needed for a trend."),
    };
    caption(ui, caption_r, &text);
}

fn middle(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, history: &[RoundSummary]) {
    let ch = &stats.chain;
    let xs: Vec<u32> = ch.turns.iter().map(|t| t.turn).collect();
    let now = ch.turn;
    let mut col = r;
    let vs_r = col.cut_top(50.0);
    ui.text_in(vs_r, "VS", TextStyle { size: 40.0, line: 44.0, mono: false, weight: 700, color: VIO }, Align::Center);
    // Heights by share of the column: the board, the pace, the gap, the ending.
    let avail = col.h - 30.0;
    let board_r = col.cut_top((avail * 0.27).round());
    col.cut_top(10.0);
    board_card(ui, board_r, s, history, &xs, now);

    let parts = pressure_parts(ch);
    let alpha: Vec<f32> = ch.turns.iter().map(|t| basis_alpha(&t.basis)).collect();
    let capacity: Vec<f32> = ch.turns.iter().map(|t| t.capacity as f32).collect();
    let pressure: Vec<f32> = ch.turns.iter().map(|t| t.pressure as f32).collect();
    let ymax = pressure.iter().chain(&capacity).fold(0.0f32, |m, v| m.max(*v)) * 1.15;

    let pace_r = col.cut_top((avail * 0.26).round());
    col.cut_top(10.0);
    let mut body = card(ui, pace_r, "Before they act · Invaders ━ you", "Per turn: what the Invaders do, each action counted as what it plants (bars: Explore, Build, Ravage), against what your plays stop before it happens (the line). Pieces you remove afterwards are not here; the board card above has them. Solid bars are played or face-up cards; faint ones the average of the unseen cards.");
    let ni = xs.iter().position(|t| *t == now).unwrap_or(0);
    let caption_r = body.cut_bottom(48.0);
    {
        let t = &ch.turns[ni];
        let lands: f64 = t.lands.iter().sum();
        let behind_since = (0..=ni).rev().take_while(|i| ch.turns[*i].ratio.is_none_or(|r| r > 1.0)).last().map(|i| ch.turns[i].turn);
        let streak = match behind_since {
            Some(t0) if t0 < now => format!(" Bars above the line since T{t0}: the rest you deal with after it lands."),
            _ => " Bars above the line: the rest you deal with after it lands.".to_string(),
        };
        caption(ui, caption_r, &format!("T{now}: the Invaders act in {lands:.0} lands, worth {:.0}; your plays stop {:.0} before it happens.{streak}", t.pressure, t.capacity));
    }
    stacked(ui, body, &xs, &[&parts[0], &parts[1], &parts[2]], &[X, B, R], &alpha, now, Some(&capacity), ymax);

    let behind_r = col.cut_top((avail * 0.34).round());
    col.cut_top(10.0);
    let body = card(ui, behind_r, "Started ÷ stopped before it happens", "What the Invaders start each turn divided by what your plays stop before it happens: 1 means nothing gets through. Below it, the backlog: what got through and was never stopped, carried turn to turn. A player who removes Invaders after they land runs a high number here and still keeps up; the board card is the judge of that.");
    let mut b = body;
    let ratio_r = b.cut_top((b.h * 0.30).round());
    let ratio: Vec<Option<f32>> = ch.turns.iter().map(|t| t.ratio.map(|x| x as f32)).collect();
    let rmax = ratio.iter().flatten().fold(0.0f32, |m, v| m.max(*v)) * 1.1;
    lines(ui, ratio_r, &xs, &[Series { values: &ratio, color: R, label: "×", project: true, faint: false }], now, rmax.max(2.0), Some(1.0), &[], None);
    let behind = ratio.get(ni).copied().flatten();
    let ratio_caption = b.cut_top(32.0);
    caption(
        ui,
        ratio_caption,
        &match behind {
            Some(x) if x > 1.05 => format!("{x:.1}×: they start {x:.1} for every one you stop before it happens."),
            Some(x) if x < 0.95 => format!("{x:.1}×: you stop more than they start, before it happens."),
            Some(_) => "About even: you stop as much as they start.".to_string(),
            None => "Nothing stopped before it happened this turn.".to_string(),
        },
    );
    b.cut_top(4.0);
    ui.text(b.x, b.y, "GOT THROUGH · STARTED AND NOT STOPPED", HEAD);
    b.cut_top(16.0);
    let back_r = b.cut_top((b.h * 0.42).round());
    let backlog: Vec<Option<f32>> = ch.turns.iter().map(|t| Some(t.backlog as f32)).collect();
    let bmax = backlog.iter().flatten().fold(0.0f32, |m, v| m.max(*v)) * 1.1;
    lines(ui, back_r, &xs, &[Series { values: &backlog, color: GOLD, label: "", project: true, faint: false }], now, bmax.max(2.0), None, &[], Some(0));
    let back_caption = b.cut_top(32.0);
    {
        let nowb = ch.turns[ni].backlog;
        let lastb = ni.checked_sub(1).map(|i| ch.turns[i].backlog);
        let text = match lastb {
            Some(l) if nowb > l + 0.5 => format!("{nowb:.0} got through unstopped, up from {l:.0} last turn. What you removed afterwards is on the board card."),
            Some(l) if nowb + 0.5 < l => format!("{nowb:.0} got through unstopped, down from {l:.0} last turn."),
            _ => format!("{nowb:.0} got through unstopped. What you removed afterwards is on the board card."),
        };
        caption(ui, back_caption, &text);
    }
    b.cut_top(6.0);
    let third = b.w / 3.0;
    stat(ui, b.x, b.y, "START ÷ STOP", &behind.map(|x| format!("{x:.1}×")).unwrap_or_else(|| "—".into()), R, BIG);
    stat(ui, b.x + third, b.y, "GOT THROUGH", &format!("{:.0}", ch.turns.get(ni).map(|t| t.backlog).unwrap_or(0.0)), GOLD, BIG);
    let (win_label, win_value, win_color) = match (&s.ending, &ch.win) {
        (Some(e), _) => {
            let (won, how) = ending_words(e);
            (format!("{} · {}", if won { "WON" } else { "LOST" }, how.split(' ').next().unwrap_or("").to_uppercase()), format!("T{}", s.turn), if won { theme::GREEN } else { R })
        }
        (None, Some((how, t))) => (format!("YOUR WIN · {}", how.split(' ').next().unwrap_or("").to_uppercase()), format!("T{t}"), theme::GREEN),
        (None, None) => ("YOUR WIN".to_string(), "none yet".to_string(), theme::TEXT),
    };
    // The big figure only when it fits the column; words get the small one.
    let style = if ui.measure(&win_value, BIG).0 <= third - 8.0 { BIG } else { NUM.bold().size(15.0) };
    stat(ui, b.x + 2.0 * third, b.y, &win_label, &win_value, win_color, style);

    // Which ending comes first.
    let race_r = col;
    let body = card(ui, race_r, "First ending · % of futures", "Over every order the unseen Invader cards can come in, with no Spirit action: the share of futures in which each ending has arrived first by that turn.");
    let race = &stats.race;
    let cols = race.turns.len().max(1) as f32;
    let lw = 118.0;
    let cw = ((body.w - lw) / cols).min(44.0);
    let mut y = body.y;
    for (k, t) in race.turns.iter().enumerate() {
        ui.text_in(Rect::new(body.x + lw + k as f32 * cw, y, cw, 14.0), &format!("T{t}"), TICK, Align::Right);
    }
    y += 16.0;
    for e in race.endings.iter().filter(|e| !e.cif.is_empty()) {
        if y + 16.0 > body.bottom() {
            break;
        }
        let title = e.title.replace("Blight card flips, then empties", "Blight card").replace("Blight card empties", "Blight card");
        ui.text_in(Rect::new(body.x, y, lw, 16.0), &title, theme::SMALL, Align::Left);
        for (k, v) in e.cif.iter().take(race.turns.len()).enumerate() {
            let c = if *v >= 0.5 { R } else if *v > 0.0 { theme::TEXT } else { DIM };
            ui.text_in(Rect::new(body.x + lw + k as f32 * cw, y, cw, 16.0), &format!("{:.0}", v * 100.0), NUM.color(c), Align::Right);
        }
        y += 17.0;
    }
    // The Spirits' side: the board clear at this Terror Level, and Fear.
    y += 6.0;
    let cl = &ch.clear;
    let when = |t: Option<u32>| t.map(|t| format!("T{t}")).unwrap_or_else(|| "—".into());
    let rows = [
        (format!("Board clear · Terror {}", ["I", "II", "III"][(cl.level as usize).clamp(1, 3) - 1]), format!("{} stand · {:.1} out · {:.1} in · {}", cl.standing, cl.removed_per_turn, cl.arrived_per_turn, when(cl.arrives))),
        ("Fear victory".to_string(), {
            let fp = &stats.fear_pace;
            let victory = fp.marks.last().map(|m| m.1).unwrap_or(0);
            let reach = fp.reach.iter().find(|(m, _)| m == "Victory").and_then(|(_, t)| *t);
            format!("{} of {victory} · {:.1}/turn · {}", fp.total, fp.recent.unwrap_or(0.0), when(reach))
        }),
    ];
    for (label, value) in rows {
        if y + 32.0 > body.bottom() {
            break;
        }
        ui.text_in(Rect::new(body.x, y, body.w, 16.0), &label, theme::SMALL.color(theme::GREEN), Align::Left);
        ui.text_in(Rect::new(body.x + 10.0, y + 15.0, body.w - 10.0, 16.0), &value, NUM, Align::Left);
        y += 34.0;
    }
}

/// Explore, Build, Ravage pressure per turn (lands × this board's weights).
fn pressure_parts(ch: &Chain) -> [Vec<f32>; 3] {
    let w = ch.weights;
    [
        ch.turns.iter().map(|t| (t.lands[0] * w[0]) as f32).collect(),
        ch.turns.iter().map(|t| (t.lands[1] * w[1]) as f32).collect(),
        ch.turns.iter().map(|t| (t.lands[2] * w[2]) as f32).collect(),
    ]
}

// ---------------------------------------------------------------- the Invaders

fn invader_side(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, ch: &Chain, _v: &ViewState) {
    let xs: Vec<u32> = ch.turns.iter().map(|t| t.turn).collect();
    let now = ch.turn;
    let mut col = r;
    let gap = 12.0;
    let w = ch.weights;

    // The two charts take the height; the bottom row is sized to its content.
    let ni = xs.iter().position(|t| *t == now).unwrap_or(0);
    let bottom_h = 236.0_f32.min(col.h * 0.3);
    let free = (col.h - bottom_h - 2.0 * gap).max(300.0);

    // 1. Pressure per turn.
    let press_r = col.cut_top((free * 0.5).round());
    col.cut_top(gap);
    let body = card(ui, press_r, &format!("Pressure · what the Invaders do, weighted by what it plants · Explore {:.1} · Build {:.1} · Ravage {:.1} each · faint: unseen cards", w[0], w[1], w[2]), "Lands the Invaders act in each turn, each action weighted by the chain it starts on this board (the mean value of this turn's Explores, Builds and Ravages). Past turns: what happened. Next turns: the mean over every order of the unseen cards. Later: the stage's average card, on the lands that will have Invaders.");
    let parts = pressure_parts(ch);
    let alpha: Vec<f32> = ch.turns.iter().map(|t| basis_alpha(&t.basis)).collect();
    let ymax = ch.turns.iter().map(|t| t.pressure as f32).fold(0.0f32, f32::max) * 1.15;
    let mut chart_r = body;
    let legend = chart_r.cut_bottom(16.0);
    let caption_r = chart_r.cut_bottom(36.0);
    {
        let t = &ch.turns[ni];
        let text = format!(
            "T{now}: {:.0} Explores worth {:.1} each, {:.0} Builds worth {:.1}, {:.0} Ravages worth {:.1}: {:.0} in all. Worth = what each leads to over the next four Invader phases if nobody stops it.",
            t.lands[0], w[0], t.lands[1], w[1], t.lands[2], w[2], t.pressure
        );
        caption(ui, caption_r, &text);
    }
    stacked(ui, chart_r, &xs, &[&parts[0], &parts[1], &parts[2]], &[X, B, R], &alpha, now, None, ymax.max(5.0));
    let mut x = legend.x;
    for (label, c) in [("Explore", X), ("Build", B), ("Ravage", R)] {
        ui.fill(Rect::new(x, legend.y + 3.0, 10.0, 10.0), c, 2.0);
        let (lw, _) = ui.text(x + 14.0, legend.y, label, theme::SMALL);
        x += lw + 28.0;
    }

    // 2. What each action plants.
    let plants_r = col.cut_top(free - (free * 0.5).round());
    col.cut_top(gap);
    let body = card(ui, plants_r, "This turn · what each action plants if left alone · actions · Blight · Dahan · health · seeds", "Each Invader action on the table this turn (E Explore, B Build, R Ravage; the chance a card makes it happen), and what it leads to over the next four Invader phases if nobody stops it: more actions, Blight, Dahan lost, Invader health left on the board, and the chains a new Town would seed later.");
    let colors = [B, VIO, SKY, DIM, X];
    let rows: Vec<(String, String, [f32; 5], f32)> = ch
        .actions
        .iter()
        .map(|a| {
            let label = format!("{} {}", &a.step[..1], fmt::short(s, &a.land));
            let chance = if a.chance.1 > 1 { format!("{}/{}", a.chance.0, a.chance.1) } else { String::new() };
            (label, chance, [a.actions as f32, a.blight as f32, 0.5 * a.dahan as f32, 0.5 * a.pieces as f32, a.tail.max(0.0) as f32], a.value as f32)
        })
        .collect();
    let mut legend_r = body;
    let bars_r = legend_r.cut_top(body.h - 16.0);
    hbars(ui, bars_r, &rows, &colors, 20.0);
    let mut x = legend_r.x;
    for (label, c) in [("actions", B), ("Blight", VIO), ("Dahan", SKY), ("health", DIM), ("seeds", X)] {
        ui.fill(Rect::new(x, legend_r.y + 3.0, 10.0, 10.0), c, 2.0);
        let (lw, _) = ui.text(x + 14.0, legend_r.y, label, theme::SMALL);
        x += lw + 24.0;
    }

    // 3. Towns as seeds, and the budgets.
    let bottom = col;
    let mut left = bottom;
    let right = left.cut_right((bottom.w - gap) / 2.0);
    left.cut_right(gap);
    seeds_card(ui, left, s, ch);
    budgets_card(ui, right, s, stats);
}

fn seeds_card(ui: &mut Ui, r: Rect, s: &PublicSnapshot, ch: &Chain) {
    let body = card(ui, r, "Towns as seeds", "Every Town or City is a source: each time a card names an empty land next to it, an Explorer lands there, Builds next turn, Ravages the turn after, and becomes a source itself. How many new sources one source makes per three-turn cycle, from the chance the next card names each empty neighbor.");
    let src = &ch.sources;
    let third = body.w / 3.0;
    stat(ui, body.x, body.y, "TOWNS", &src.now.to_string(), B, BIG);
    stat(ui, body.x + third, body.y, "EMPTY LANDS", &src.empty.to_string(), theme::TEXT, BIG);
    stat(ui, body.x + 2.0 * third, body.y, "LANDS", &src.lands.to_string(), DIM, BIG);
    // The island as a strip: Towns, lands with Explorers only, empty.
    let mut cells: Vec<Rgba> = s.lands.iter().filter(|l| !l.is_ocean).map(|l| if l.towns + l.cities > 0 { B } else if l.explorers > 0 { Rgba::hex(0x7a4a3a) } else { theme::BORDER }).collect();
    cells.sort_by_key(|c| if *c == B { 0 } else if *c == theme::BORDER { 2 } else { 1 });
    let y = body.y + 50.0;
    let cw = ((body.w - 2.0) / cells.len().max(1) as f32).min(16.0);
    for (i, c) in cells.iter().enumerate() {
        ui.fill(Rect::new(body.x + i as f32 * cw, y, cw - 3.0, 14.0), *c, 2.0);
    }
    let y = y + 26.0;
    ui.text(body.x, y, "EACH TOWN SEEDS", LABEL);
    let (w, _) = ui.text(body.x + 112.0, y - 3.0, &format!("{:.2}", src.r), NUM.bold().size(15.0).color(X));
    ui.text(body.x + 112.0 + w + 6.0, y, "new per 3 turns", theme::SMALL.color(DIM));
    // Towns if unanswered: now, in 3, in 6 turns.
    let y = y + 24.0;
    ui.text(body.x, y, "TOWNS IF UNANSWERED", LABEL);
    let vals = [src.now as f32, (src.now as f64 * src.factor).min(src.lands as f64) as f32, (src.now as f64 * src.factor * src.factor).min(src.lands as f64) as f32];
    let labels = [format!("T{}", ch.turn), format!("T{}", ch.turn + 3), format!("T{}", ch.turn + 6)];
    let mini = Rect::new(body.x + 130.0, y - 6.0, body.w - 130.0, (body.bottom() - y).max(40.0).min(60.0));
    let top = src.lands as f32;
    let bw = mini.w / 3.0 * 0.55;
    for (i, (v, lab)) in vals.iter().zip(&labels).enumerate() {
        let cx = mini.x + (i as f32 + 0.5) * mini.w / 3.0;
        let h = v / top * (mini.h - 24.0);
        ui.fill(Rect::new(cx - bw / 2.0, mini.bottom() - 14.0 - h, bw, h.max(1.0)), B.alpha(if i == 0 { 255 } else { 140 }), 1.5);
        ui.text_in(Rect::new(cx - 20.0, mini.bottom() - 28.0 - h, 40.0, 14.0), &format!("{v:.0}"), TICK.color(theme::TEXT), Align::Center);
        ui.text_in(Rect::new(cx - 20.0, mini.bottom() - 13.0, 40.0, 13.0), lab, TICK, Align::Center);
    }
    let cap_y = mini.bottom() - 14.0 - (mini.h - 24.0);
    super::chart::dashed(ui, &[(mini.x, cap_y), (mini.right(), cap_y)], 1.0, DIM, 3.0, 3.0);
}

fn budgets_card(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let body = card(ui, r, "Budgets", "What runs out: the Blight card (this Ravage's Blight crossed out), the adversary's loss condition where it stands and after this Ravage, Fear toward the next card, Fear cards left at each Terror Level, and the Invader deck by stage.");
    let bl = &stats.blight;
    let fear = &stats.fear;
    let lw = 104.0;
    let row_h = 30.0;
    let mut y = body.y + 2.0;
    let label = |ui: &mut Ui, y: f32, text: &str| {
        ui.text_wrap(body.x, y, lw - 8.0, text, LABEL);
    };
    let note = |ui: &mut Ui, x: f32, y: f32, big: &str, small: &str, color: Rgba| {
        let (w, _) = ui.text(x, y - 2.0, big, NUM.bold().size(14.0).color(color));
        ui.text(x + w + 5.0, y, small, theme::SMALL.color(DIM));
    };
    // Blight card.
    label(ui, y, "BLIGHT CARD");
    let left = bl.remaining.max(0) as usize;
    let struck = (bl.adds_this_phase_min.max(0) as usize).min(left);
    let w = pips(ui, body.x + lw, y + 1.0, left.max(1), left, VIO, struck, 0);
    let flips = struck >= left && left > 0;
    note(ui, body.x + lw + w + 4.0, y, &format!("−{}", bl.adds_this_phase_min), if flips { "this Ravage · flips" } else { "this Ravage" }, R);
    y += row_h;
    // The adversary's clock.
    if let Some(c) = stats.loss_clocks.first() {
        let lim = c.limit.max(1) as usize;
        let now = (c.now.max(0) as usize).min(lim);
        let after = c.after.unwrap_or(c.now).max(0) as usize;
        label(ui, y, &c.title.to_uppercase());
        let w = pips(ui, body.x + lw, y + 1.0, lim.min(12), now, R, 0, after.saturating_sub(now));
        note(ui, body.x + lw + w + 4.0, y, &format!("{}", c.now), &format!("of {} · {after} after", c.limit), theme::TEXT);
        y += row_h;
    }
    // Fear to the next card, and the cards left by Terror Level.
    label(ui, y, "FEAR");
    let req = fear.required_per_card.max(1) as usize;
    let w = pips(ui, body.x + lw, y + 1.0, req.min(14), (fear.generated.max(0) as usize).min(req), SKY, 0, 0);
    note(ui, body.x + lw + w + 4.0, y, &fear.generated.to_string(), &format!("of {}", fear.required_per_card), SKY);
    y += row_h;
    // Fear cards left, a group per Terror Level (the levels passed dimmed).
    label(ui, y, "FEAR CARDS");
    let mut x = body.x + lw;
    for (k, n) in fear.cards_left_by_level.iter().enumerate() {
        let c = if k as u8 + 1 < fear.terror_level { SKY.alpha(90) } else { SKY };
        for _ in 0..*n {
            ui.fill(Rect::new(x, y + 2.0, 7.0, 9.0), c, 1.5);
            x += 9.0;
        }
        x += 7.0;
    }
    let total: u32 = fear.cards_left_by_level.iter().sum();
    note(ui, x, y, &total.to_string(), "left", theme::TEXT);
    y += row_h;
    // The Invader deck, by stage.
    label(ui, y, "INVADER DECK");
    let mut x = body.x + lw;
    for st in &s.invaders.upcoming_stages {
        let c = match st {
            1 => Rgba::hex(0x8f9bb3),
            2 => B,
            _ => R,
        };
        ui.fill(Rect::new(x, y + 1.0, 10.0, 10.0), c, 2.0);
        x += 13.0;
    }
    let n2 = s.invaders.upcoming_stages.iter().filter(|x| **x == 2).count();
    let n3 = s.invaders.upcoming_stages.iter().filter(|x| **x == 3).count();
    note(ui, x + 4.0, y, &s.invaders.upcoming_stages.len().to_string(), &format!("· {n2} II · {n3} III"), theme::TEXT);
}
