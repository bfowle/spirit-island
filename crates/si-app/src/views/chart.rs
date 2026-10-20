//! Chart parts for the analysis panels: a plot area that maps game units
//! to pixels, a turn axis, gridlines, lines, dashed projections and
//! gradient bands. Charts draw on the map layer, above the panel cards, so
//! their meshes (bands, zones) sit on top of the card backgrounds.

use si_ui::theme::{self, Rgba};
use si_ui::{Align, Pt, Rect, Ui};

#[derive(Debug, Clone, Copy)]
pub struct Plot {
    pub r: Rect,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}

impl Plot {
    pub fn px(&self, x: f32) -> f32 {
        self.r.x + (x - self.x0) / (self.x1 - self.x0).max(1e-6) * self.r.w
    }

    pub fn py(&self, y: f32) -> f32 {
        self.r.bottom() - (y - self.y0) / (self.y1 - self.y0).max(1e-6) * self.r.h
    }

    pub fn pt(&self, x: f32, y: f32) -> Pt {
        (self.px(x), self.py(y))
    }
}

/// Turn ticks along the bottom with faint columns; the current turn lit.
pub fn turn_axis(ui: &mut Ui, p: &Plot, now: u32) {
    let first = p.x0.ceil().max(1.0) as u32;
    let last = p.x1.floor() as u32;
    for t in first..=last {
        let x = p.px(t as f32);
        let lit = t == now;
        ui.line((x, p.r.y), (x, p.r.bottom()), 1.0, if lit { theme::ACCENT.alpha(70) } else { theme::BORDER.alpha(140) }, 0.0);
        let style = if lit { theme::SMALL.bold().color(theme::TEXT) } else { theme::SMALL.color(theme::MUTED) };
        ui.text_in(Rect::new(x - 22.0, p.r.bottom() + 5.0, 44.0, 14.0), &format!("T{t}"), style, Align::Center);
    }
}

/// A horizontal line across the plot, with a label at its right end.
pub fn hline(ui: &mut Ui, p: &Plot, y: f32, color: Rgba, label: &str) {
    let py = p.py(y);
    dashed(ui, &[(p.r.x, py), (p.r.right(), py)], 1.0, color, 4.0, 4.0);
    if !label.is_empty() {
        let w = ui.measure(label, theme::SMALL).0;
        ui.text(p.r.right() - w, py - 16.0, label, theme::SMALL.color(color));
    }
}

pub fn polyline(ui: &mut Ui, pts: &[Pt], width: f32, color: Rgba) {
    for w in pts.windows(2) {
        ui.line(w[0], w[1], width, color, 0.0);
    }
}

/// A soft glow under a line, for the one series that matters most.
pub fn glow_line(ui: &mut Ui, pts: &[Pt], width: f32, color: Rgba) {
    for w in pts.windows(2) {
        ui.line(w[0], w[1], width * 4.0, color.alpha(color.3 / 4), width * 3.0);
    }
    polyline(ui, pts, width, color);
}

/// A dashed polyline: projections, required paces, thresholds.
pub fn dashed(ui: &mut Ui, pts: &[Pt], width: f32, color: Rgba, dash: f32, gap: f32) {
    let mut on = true;
    let mut left = dash;
    for w in pts.windows(2) {
        let (a, b) = (w[0], w[1]);
        let len = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        if len < 1e-3 {
            continue;
        }
        let mut at = 0.0;
        while at < len {
            let step = left.min(len - at);
            let p0 = (a.0 + (b.0 - a.0) * at / len, a.1 + (b.1 - a.1) * at / len);
            let p1 = (a.0 + (b.0 - a.0) * (at + step) / len, a.1 + (b.1 - a.1) * (at + step) / len);
            if on {
                ui.line(p0, p1, width, color, 0.0);
            }
            at += step;
            left -= step;
            if left <= 1e-3 {
                on = !on;
                left = if on { dash } else { gap };
            }
        }
    }
}

/// The area between two series sharing x positions, shaded `top` along the
/// upper edge to `bottom` along the lower.
pub fn band(ui: &mut Ui, upper: &[Pt], lower: &[Pt], top: Rgba, bottom: Rgba) {
    for (u, l) in upper.windows(2).zip(lower.windows(2)) {
        ui.quad([u[0], u[1], l[1], l[0]], [top, top, bottom, bottom]);
    }
}

/// A row of small text spans with colors, left to right: a caption whose
/// numbers stand out.
pub fn caption(ui: &mut Ui, x: f32, y: f32, w: f32, parts: &[(&str, Rgba, bool)]) -> f32 {
    let spans: Vec<si_ui::Span> = parts
        .iter()
        .map(|(t, c, bold)| si_ui::span(t, if *bold { theme::SMALL.bold().color(*c) } else { theme::SMALL.color(*c) }))
        .collect();
    ui.spans(x, y, &spans, Some(w)).1
}
