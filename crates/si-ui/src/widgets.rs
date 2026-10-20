//! Widgets built on the painter: panels with headers, chips, pips, bars.

use crate::geom::Rect;
use crate::painter::{Align, Ui};
use crate::theme::{self, Rgba, TextStyle};

pub const PANEL_HEAD_H: f32 = 26.0;

/// A panel card with a header (title, optional count on the right). Returns
/// the content rectangle, inset by the panel padding.
pub fn panel(ui: &mut Ui, r: Rect, title: &str, count: Option<&str>) -> Rect {
    ui.rect(r, theme::PANEL, theme::BORDER, 1.0, theme::RADIUS);
    let mut body = r;
    let head = body.cut_top(PANEL_HEAD_H);
    ui.fill(Rect::new(head.x + 1.0, head.bottom() - 1.0, head.w - 2.0, 1.0), theme::BORDER, 0.0);
    let inner = head.inset(theme::PAD, 0.0, theme::PAD, 0.0);
    ui.text_in(inner, &title.to_uppercase(), theme::HEAD, Align::Left);
    if let Some(c) = count {
        ui.text_in(inner, c, theme::SMALL.mono(), Align::Right);
    }
    body.inset(theme::PAD, 8.0, theme::PAD, theme::PAD)
}

/// A small rounded label. Returns its width.
pub fn chip(ui: &mut Ui, x: f32, y: f32, text: &str, fg: Rgba, bg: Rgba) -> f32 {
    let style = TextStyle { size: 11.0, line: 14.0, mono: false, weight: 600, color: fg };
    let (w, _) = ui.measure(text, style);
    let r = Rect::new(x, y, w + 10.0, 18.0);
    ui.fill(r, bg, theme::RADIUS_SM);
    ui.text_in(r, text, style, Align::Center);
    r.w
}

/// A chip with a colored border and no fill, for secondary tags.
pub fn tag(ui: &mut Ui, x: f32, y: f32, text: &str, fg: Rgba) -> f32 {
    let style = TextStyle { size: 11.0, line: 14.0, mono: false, weight: 500, color: fg };
    let (w, _) = ui.measure(text, style);
    let r = Rect::new(x, y, w + 10.0, 18.0);
    ui.rect(r, theme::TRANSPARENT, fg.alpha(110), 1.0, theme::RADIUS_SM);
    ui.text_in(r, text, style, Align::Center);
    r.w
}

/// `hit` of `total` dots. Returns the width.
pub fn pips(ui: &mut Ui, x: f32, y: f32, hit: i64, total: i64, on: Rgba) -> f32 {
    let d = 9.0;
    for i in 0..total.max(0) {
        let r = Rect::new(x + i as f32 * (d + 4.0), y, d, d);
        if i < hit {
            ui.fill(r, on, d / 2.0);
        } else {
            ui.rect(r, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, d / 2.0);
        }
    }
    total.max(0) as f32 * (d + 4.0)
}

/// A horizontal bar filled to `frac`.
pub fn bar(ui: &mut Ui, r: Rect, frac: f32, on: Rgba) {
    ui.fill(r, theme::INSET, r.h / 2.0);
    let f = frac.clamp(0.0, 1.0);
    if f > 0.0 {
        ui.fill(Rect::new(r.x, r.y, (r.w * f).max(r.h), r.h), on, r.h / 2.0);
    }
}

/// A label and a value on one line: the label muted, the value right-aligned.
pub fn kv(ui: &mut Ui, r: Rect, key: &str, value: &str, value_style: TextStyle) {
    ui.text_in(r, key, theme::SMALL, Align::Left);
    ui.text_in(r, value, value_style, Align::Right);
}

/// A clickable text button. Returns true when clicked.
pub fn button(ui: &mut Ui, r: Rect, text: &str, active: bool) -> bool {
    let hover = ui.hovered(r);
    let (bg, fg) = match (active, hover) {
        (true, _) => (theme::ACCENT_DIM, theme::TEXT),
        (false, true) => (theme::RAISED, theme::TEXT),
        (false, false) => (theme::TRANSPARENT, theme::TEXT_2),
    };
    ui.fill(r, bg, theme::RADIUS_SM);
    let style = TextStyle { size: 12.0, line: 16.0, mono: false, weight: if active { 600 } else { 500 }, color: fg };
    ui.text_in(r, text, style, Align::Center);
    ui.clicked(r)
}
