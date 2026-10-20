//! The Curve (Rei, with darker's stage rule): the Invader side as the lands
//! the cards in Ravage, Build and Explore name each turn, a step that climbs
//! as Stage II and III cards move through the slots; the Spirit side as
//! Energy and card plays a turn from the presence tracks, so far and
//! projected along each track. Beside them, who is growing faster: both as
//! multiples of Turn 1, and each turn's change (the acceleration).
//!
//! Natural units throughout. The one exchange rate, a card play at about
//! 2.5 Energy (Rei's chart; contested in the same thread), only combines
//! the economy for the growth comparison, with a band from 1.5 to 3.5.

use super::chart::{self, Plot};
use super::{Both, fmt};
use super::runway::now_x;
use si_common::companion_stats::CompanionStats;
use si_common::companion_stats::curve::{CurveTurn, Step};
use si_common::public::PublicSnapshot;
use si_ui::theme::{self, Rgba, TextStyle};
use si_ui::{Align, Pt, Rect, Ui};

const RAVAGE: Rgba = Rgba::hex(0xef6f6c);
const BUILD: Rgba = Rgba::hex(0xf2a93b);
const EXPLORE: Rgba = Rgba::hex(0x4fd1c5);
const ENERGY: Rgba = Rgba::hex(0xf6c453);
const PLAYS: Rgba = Rgba::hex(0xb794f6);
const YOU: Rgba = Rgba::hex(0x8b7cf6);
/// Rei's card-play value and the band around it.
const PLAY_E: (f64, f64, f64) = (1.5, 2.5, 3.5);

fn panel(ui: &mut Ui, r: Rect, title: &str, sub: &str, color: Rgba) -> Rect {
    ui.rect(r, theme::PANEL, theme::BORDER, 1.0, 10.0);
    let (w, _) = ui.text(r.x + 20.0, r.y + 14.0, title, TextStyle { size: 13.0, line: 17.0, mono: false, weight: 700, color });
    ui.text(r.x + 28.0 + w, r.y + 15.0, sub, theme::SMALL.color(theme::MUTED));
    r.inset(20.0, 40.0, 20.0, 14.0)
}

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, both: &Both) {
    let stats = both.tried;
    let c = &stats.curve;
    if c.turns.is_empty() {
        ui.text(r.x, r.y + 20.0, "No Invader deck to chart.", theme::BODY.color(theme::MUTED));
        return;
    }
    let left_w = (r.w * 0.64).floor();
    let top_h = (r.h * 0.56).floor();
    let step_r = panel(ui, Rect::new(r.x, r.y, left_w, top_h), "THE INVADER STEP", "lands the cards in Ravage, Build and Explore name each turn", RAVAGE);
    let econ_r = panel(ui, Rect::new(r.x, r.y + top_h + 12.0, left_w, r.h - top_h - 12.0), "YOUR ECONOMY", "Energy and card plays a turn, from the presence tracks", ENERGY);
    let grow_r = panel(ui, Rect::new(r.x + left_w + 12.0, r.y, r.w - left_w - 12.0, r.h), "WHO'S GROWING FASTER", "as multiples of Turn 1", YOU);

    // Both left charts share one turn axis, so a turn lines up top to bottom.
    let last = c.turns.last().map(|t| t.turn).unwrap_or(1);
    let plot_x = (step_r.x + 34.0, step_r.w - 150.0);
    step_chart(ui, step_r, plot_x, last, s, stats);
    economy_chart(ui, econ_r, plot_x, last, s, stats);
    growth(ui, grow_r, last, s, stats);
}

fn step_color(i: usize) -> Rgba {
    [RAVAGE, BUILD, EXPLORE][i]
}

fn step_chart(ui: &mut Ui, r: Rect, (px, pw): (f32, f32), last: u32, s: &PublicSnapshot, stats: &CompanionStats) {
    let c = &stats.curve;
    let most = c.turns.iter().map(CurveTurn::lands).fold(1.0, f64::max) as f32;
    let p = Plot { r: Rect::new(px, r.y + 22.0, pw, r.h - 70.0), x0: 0.4, x1: last as f32 + 0.6, y0: 0.0, y1: most * 1.18 };
    ui.layer(si_ui::LAYER_MAP);
    let now = p.px(s.turn as f32);
    ui.fill(Rect::new(now - 26.0, p.r.y - 6.0, 52.0, p.r.h + 6.0), theme::ACCENT.alpha(18), 6.0);
    for v in grid(most) {
        let y = p.py(v);
        ui.line((p.r.x, y), (p.r.right(), y), 1.0, theme::BORDER.alpha(140), 0.0);
        ui.text_in(Rect::new(p.r.x - 34.0, y - 8.0, 28.0, 16.0), &format!("{v:.0}"), theme::SMALL.color(theme::MUTED), Align::Right);
    }
    let bar = (p.px(1.0) - p.px(0.0)) * 0.6;
    for t in &c.turns {
        let x = p.px(t.turn as f32);
        let mut y0 = 0.0f32;
        for (i, st) in [&t.ravage, &t.build, &t.explore].into_iter().enumerate() {
            if st.stage == 0 || st.lands <= 0.0 {
                continue;
            }
            let y1 = y0 + st.lands as f32;
            let seg = Rect::new(x - bar / 2.0, p.py(y1), bar, p.py(y0) - p.py(y1) - 1.0);
            let col = step_color(i);
            if st.known {
                ui.rect(seg, col.alpha(190), theme::TRANSPARENT, 0.0, 3.0);
            } else {
                // Only the stage on the back: the stage's average, open.
                ui.rect(seg, col.alpha(45), col.alpha(170), 1.0, 3.0);
            }
            if seg.h >= 14.0 {
                ui.text_in(seg, fmt::roman(st.stage as i64), theme::SMALL.bold().color(if st.known { theme::BG } else { col }), Align::Center);
            }
            y0 = y1;
        }
        let known = [&t.ravage, &t.build, &t.explore].iter().all(|x| x.known || x.stage == 0);
        let label = if known { format!("{:.0}", t.lands()) } else { format!("≈{:.0}", t.lands()) };
        ui.text_in(Rect::new(x - 24.0, p.py(y0) - 17.0, 48.0, 14.0), &label, theme::SMALL.bold().color(theme::TEXT_2), Align::Center);
    }
    chart::turn_axis(ui, &p, s.turn);
    ui.layer(si_ui::LAYER_BASE);

    // When later stages reach Ravage: the steps of the curve.
    let mut flags = Vec::new();
    for stage in [2u8, 3] {
        if let Some(t) = c.turns.iter().find(|t| t.ravage.stage == stage) {
            flags.push(format!("Stage {} Ravages from T{}", fmt::roman(stage as i64), t.turn));
        }
    }
    // Key and notes, right of the plot.
    let kx = p.r.right() + 22.0;
    let mut ky = p.r.y;
    for (i, name) in ["Ravage", "Build", "Explore"].iter().enumerate() {
        ui.fill(Rect::new(kx, ky + 3.0, 12.0, 10.0), step_color(i).alpha(190), 2.0);
        ui.text(kx + 18.0, ky, name, theme::SMALL.color(theme::TEXT_2));
        ky += 18.0;
    }
    ui.rect(Rect::new(kx, ky + 3.0, 12.0, 10.0), theme::MUTED.alpha(45), theme::MUTED, 1.0, 2.0);
    ui.text_wrap(kx + 18.0, ky, 100.0, "back only: the stage's average", theme::SMALL.color(theme::MUTED));
    ky += 44.0;
    for f in &flags {
        let (_, h) = ui.text_wrap(kx, ky, 110.0, f, theme::SMALL.bold().color(RAVAGE));
        ky += h + 6.0;
    }
    let reach = c.stage_reach;
    let rule = format!(
        "Pocket rule (darker): Stage I, II, III cards name about 2, 3, 4 lands a board. Here, {} board{}: I {:.1} · II {:.1} · III {:.1}.",
        c.boards,
        if c.boards == 1 { "" } else { "s" },
        reach[0],
        reach[1],
        reach[2]
    );
    let rule = fmt::fit(ui, &rule, theme::SMALL.color(theme::MUTED), r.w);
    ui.text(r.x, r.bottom() - 16.0, &rule, theme::SMALL.color(theme::MUTED));
}

/// Gridline values for a 0..`most` axis.
fn grid(most: f32) -> Vec<f32> {
    let step = if most > 24.0 { 10.0 } else if most > 10.0 { 5.0 } else { 2.0 };
    (1..).map(|k| k as f32 * step).take_while(|v| *v <= most * 1.1).collect()
}

fn economy_chart(ui: &mut Ui, r: Rect, (px, pw): (f32, f32), last: u32, s: &PublicSnapshot, stats: &CompanionStats) {
    let c = &stats.curve;
    let growth_max = c.growth.iter().flat_map(|g| g.points.iter().map(|x| x.1.max(x.2))).max().unwrap_or(0);
    let hist_max = c.economy.iter().map(|e| e.energy.max(e.plays)).max().unwrap_or(0);
    let most = growth_max.max(hist_max).max(2) as f32;
    let p = Plot { r: Rect::new(px, r.y + 14.0, pw, r.h - 50.0), x0: 0.4, x1: last as f32 + 0.6, y0: 0.0, y1: most * 1.2 };
    ui.layer(si_ui::LAYER_MAP);
    let now = p.px(s.turn as f32);
    ui.fill(Rect::new(now - 26.0, p.r.y - 6.0, 52.0, p.r.h + 6.0), theme::ACCENT.alpha(18), 6.0);
    for v in grid(most) {
        let y = p.py(v);
        ui.line((p.r.x, y), (p.r.right(), y), 1.0, theme::BORDER.alpha(140), 0.0);
        ui.text_in(Rect::new(p.r.x - 34.0, y - 8.0, 28.0, 16.0), &format!("{v:.0}"), theme::SMALL.color(theme::MUTED), Align::Right);
    }
    // Presence on the island, as faint columns with their count.
    let top_presence = c.economy.iter().map(|e| e.presence).max().unwrap_or(1).max(1) as f32;
    for e in &c.economy {
        let x = p.px(e.turn as f32);
        let h = p.r.h * 0.35 * e.presence as f32 / top_presence;
        ui.fill(Rect::new(x - 9.0, p.r.bottom() - h, 18.0, h), theme::BLUE.alpha(40), 3.0);
        ui.text_in(Rect::new(x - 15.0, p.r.bottom() - h - 14.0, 30.0, 12.0), &e.presence.to_string(), theme::SMALL.color(theme::BLUE.alpha(160)), Align::Center);
    }
    let series = |f: fn(&si_common::companion_stats::curve::Economy) -> i64| -> Vec<Pt> { c.economy.iter().map(|e| p.pt(e.turn as f32, f(e) as f32)).collect() };
    let energy = series(|e| e.energy);
    let plays = series(|e| e.plays);
    // Projections along each track, from the current values.
    for (g, color, pick) in [(&c.growth.first(), ENERGY, 1usize), (&c.growth.get(1), PLAYS, 2)] {
        let Some(g) = g else { continue };
        let from = if pick == 1 { energy.last() } else { plays.last() };
        let mut pts: Vec<Pt> = from.into_iter().copied().collect();
        pts.extend(g.points.iter().map(|x| p.pt(x.0 as f32, if pick == 1 { x.1 } else { x.2 } as f32)));
        chart::dashed(ui, &pts, 1.6, color.alpha(200), 5.0, 4.0);
        if let Some(end) = pts.last() {
            let v = g.points.last().map(|x| if pick == 1 { x.1 } else { x.2 }).unwrap_or(0);
            ui.text(end.0 + 6.0, end.1 - 8.0, &v.to_string(), theme::SMALL.bold().color(color));
        }
    }
    chart::glow_line(ui, &energy, 2.2, ENERGY);
    chart::glow_line(ui, &plays, 2.2, PLAYS);
    for pt in energy.iter() {
        ui.circle(*pt, 3.0, ENERGY, theme::TRANSPARENT, 0.0);
    }
    for pt in plays.iter() {
        ui.circle(*pt, 3.0, PLAYS, theme::TRANSPARENT, 0.0);
    }
    chart::turn_axis(ui, &p, s.turn);
    ui.layer(si_ui::LAYER_BASE);

    let kx = p.r.right() + 22.0;
    let mut ky = p.r.y;
    for (name, col) in [("Energy a turn", ENERGY), ("Card plays a turn", PLAYS)] {
        ui.fill(Rect::new(kx, ky + 6.0, 14.0, 3.0), col, 1.0);
        ui.text(kx + 20.0, ky, name, theme::SMALL.color(theme::TEXT_2));
        ky += 18.0;
    }
    ui.fill(Rect::new(kx, ky + 3.0, 12.0, 10.0), theme::BLUE.alpha(40), 2.0);
    ui.text(kx + 18.0, ky, "presence on the island", theme::SMALL.color(theme::TEXT_2));
    ky += 22.0;
    ui.text_wrap(kx, ky, 120.0, "dashed: one presence a turn from that track", theme::SMALL.color(theme::MUTED));
    let _ = s;
}

/// Economy as one number for the growth comparison: Energy + card plays at
/// `play` Energy each.
fn worth(energy: i64, plays: i64, play: f64) -> f64 {
    energy as f64 + play * plays as f64
}

fn growth(ui: &mut Ui, r: Rect, last: u32, s: &PublicSnapshot, stats: &CompanionStats) {
    let c = &stats.curve;
    let Some(base_turn) = c.turns.iter().find(|t| t.lands() > 0.0) else { return };
    let base_inv = base_turn.lands().max(1.0);
    let Some(e0) = c.economy.first() else {
        ui.text(r.x, r.y, "No turn of your economy recorded yet.", theme::SMALL.color(theme::MUTED));
        return;
    };
    let base_you = worth(e0.energy, e0.plays, PLAY_E.1).max(1.0);
    let inv: Vec<(u32, f64, bool)> = c.turns.iter().filter(|t| t.lands() > 0.0).map(|t| (t.turn, t.lands() / base_inv, [&t.ravage, &t.build, &t.explore].iter().all(|x: &&Step| x.known || x.stage == 0))).collect();
    let you: Vec<(u32, f64)> = c.economy.iter().map(|e| (e.turn, worth(e.energy, e.plays, PLAY_E.1) / base_you)).collect();
    // Ahead: the two tracks × the card-play band.
    let ahead: Vec<(u32, f64, f64, f64)> = (0..c.growth.first().map(|g| g.points.len()).unwrap_or(0))
        .map(|k| {
            let mut vals = Vec::new();
            let mut mid = 0.0;
            for g in &c.growth {
                let (t, e, pl) = g.points[k];
                let _ = t;
                vals.push(worth(e, pl, PLAY_E.0) / base_you);
                vals.push(worth(e, pl, PLAY_E.2) / base_you);
                mid += worth(e, pl, PLAY_E.1) / base_you / c.growth.len() as f64;
            }
            let t = c.growth[0].points[k].0;
            (t, vals.iter().copied().fold(f64::MAX, f64::min), mid, vals.iter().copied().fold(0.0, f64::max))
        })
        .collect();
    let most = inv.iter().map(|x| x.1).chain(you.iter().map(|x| x.1)).chain(ahead.iter().map(|x| x.3)).fold(1.0, f64::max) as f32;

    let chart_h = (r.h * 0.42).floor();
    let p = Plot { r: Rect::new(r.x + 34.0, r.y + 10.0, r.w - 70.0, chart_h - 30.0), x0: 0.6, x1: last as f32 + 0.4, y0: 0.0, y1: most * 1.15 };
    ui.layer(si_ui::LAYER_MAP);
    for v in [1.0f32, 2.0, 3.0, 4.0, 6.0, 8.0].iter().filter(|v| **v <= most * 1.1) {
        let y = p.py(*v);
        ui.line((p.r.x, y), (p.r.right(), y), 1.0, if *v == 1.0 { theme::BORDER_STRONG } else { theme::BORDER.alpha(140) }, 0.0);
        ui.text_in(Rect::new(p.r.x - 34.0, y - 8.0, 28.0, 16.0), &format!("×{v:.0}"), theme::SMALL.color(theme::MUTED), Align::Right);
    }
    let ipts: Vec<Pt> = inv.iter().map(|x| p.pt(x.0 as f32, x.1 as f32)).collect();
    let split = inv.iter().position(|x| !x.2).unwrap_or(inv.len());
    if split > 0 {
        chart::glow_line(ui, &ipts[..split], 2.2, RAVAGE);
    }
    chart::dashed(ui, &ipts[split.saturating_sub(1)..], 2.0, RAVAGE.alpha(200), 6.0, 4.0);
    let ypts: Vec<Pt> = you.iter().map(|x| p.pt(x.0 as f32, x.1 as f32)).collect();
    if let Some(start) = ypts.last().copied() {
        let upper: Vec<Pt> = std::iter::once(start).chain(ahead.iter().map(|x| p.pt(x.0 as f32, x.3 as f32))).collect();
        let lower: Vec<Pt> = std::iter::once(start).chain(ahead.iter().map(|x| p.pt(x.0 as f32, x.1 as f32))).collect();
        chart::band(ui, &upper, &lower, YOU.alpha(60), YOU.alpha(25));
        let mid: Vec<Pt> = std::iter::once(start).chain(ahead.iter().map(|x| p.pt(x.0 as f32, x.2 as f32))).collect();
        chart::dashed(ui, &mid, 1.8, YOU, 6.0, 4.0);
    }
    chart::glow_line(ui, &ypts, 2.2, YOU);
    chart::turn_axis(ui, &p, s.turn);
    ui.layer(si_ui::LAYER_BASE);

    // In words: since Turn 1, and by the end of the deck.
    let mut y = r.y + chart_h + 4.0;
    let now_inv = inv.iter().filter(|x| x.0 <= s.turn).last().map(|x| x.1).unwrap_or(1.0);
    let end_inv = inv.iter().filter(|x| x.0 < last).last().map(|x| x.1).unwrap_or(now_inv);
    let now_you = you.last().map(|x| x.1).unwrap_or(1.0);
    let (lo, hi) = ahead.iter().filter(|x| x.0 < last).last().map(|x| (x.1, x.3)).unwrap_or((now_you, now_you));
    let e1 = c.economy.last().cloned().unwrap_or_default();
    let line = |ui: &mut Ui, y: f32, parts: &[(&str, Rgba, bool)]| chart::caption(ui, r.x, y, r.w, parts);
    let a = format!("×{now_inv:.1}");
    let a_end = format!("×{end_inv:.1} by T{}", last - 1);
    y += line(ui, y, &[("Invaders: ", theme::TEXT_2, false), (a.as_str(), RAVAGE, true), (" now, ", theme::TEXT_2, false), (a_end.as_str(), RAVAGE, true), (" (lands named a turn).", theme::TEXT_2, false)]) + 4.0;
    let b = format!("×{now_you:.1}");
    let b_end = format!("×{lo:.1}–{hi:.1} by T{}", last - 1);
    let detail = format!(" (Energy {}→{}, plays {}→{}); along your tracks ", e0.energy, e1.energy, e0.plays, e1.plays);
    y += line(ui, y, &[("You: ", theme::TEXT_2, false), (b.as_str(), YOU, true), (detail.as_str(), theme::TEXT_2, false), (b_end.as_str(), YOU, true), (".", theme::TEXT_2, false)]) + 10.0;

    // Growth a turn, unitless (so lands and Energy compare): the compound
    // rate over the last three turns.
    let rate = |series: &[(u32, f64)]| -> Option<f64> {
        let now = series.iter().filter(|x| x.0 <= s.turn).last()?;
        let then = series.iter().filter(|x| x.0 + 3 <= now.0).last()?;
        let turns = (now.0 - then.0) as f64;
        (then.1 > 0.0 && turns > 0.0).then(|| (now.1 / then.1).powf(1.0 / turns) - 1.0)
    };
    let inv_series: Vec<(u32, f64)> = inv.iter().map(|x| (x.0, x.1)).collect();
    if let (Some(ri), Some(ry)) = (rate(&inv_series), rate(&you)) {
        let a = format!("{:+.0}% a turn", ri * 100.0);
        let b = format!("{:+.0}%", ry * 100.0);
        let verdict = if ri > ry + 0.02 { "the Invaders are pulling away" } else if ry > ri + 0.02 { "you are gaining on them" } else { "about even" };
        let tail = format!(" over the last 3 turns: {verdict}.");
        y += line(ui, y, &[("Growth: Invaders ", theme::TEXT_2, false), (a.as_str(), RAVAGE, true), (", you ", theme::TEXT_2, false), (b.as_str(), YOU, true), (tail.as_str(), theme::TEXT_2, false)]) + 12.0;
    }

    // Acceleration: each turn's change, one row each (their own units).
    ui.text(r.x, y, "EACH TURN'S CHANGE", theme::SMALL.color(theme::MUTED));
    y += 20.0;
    let d_inv: Vec<(u32, f64)> = c.turns.windows(2).filter(|w| w[1].turn < last).map(|w| (w[1].turn, w[1].lands() - w[0].lands())).collect();
    let d_you: Vec<(u32, f64)> = c
        .economy
        .windows(2)
        .map(|w| (w[1].turn, worth(w[1].energy, w[1].plays, PLAY_E.1) - worth(w[0].energy, w[0].plays, PLAY_E.1)))
        .chain(c.growth.first().into_iter().flat_map(|g| {
            let mut prev = worth(e1.energy, e1.plays, PLAY_E.1);
            let mut out = Vec::new();
            for (k, pt) in g.points.iter().enumerate() {
                let v = c.growth.iter().map(|g| worth(g.points[k].1, g.points[k].2, PLAY_E.1)).sum::<f64>() / c.growth.len() as f64;
                out.push((pt.0, v - prev));
                prev = v;
            }
            out
        }))
        .filter(|x| x.0 < last)
        .collect();
    let row_h = ((r.bottom() - y - 100.0) / 2.0).clamp(34.0, 84.0);
    for (k, (series, color, label)) in [(&d_inv, RAVAGE, "Invaders, lands"), (&d_you, YOU, "You, Energy + plays")].into_iter().enumerate() {
        let q = Plot { r: Rect::new(p.r.x, y + 16.0 + k as f32 * (row_h + 24.0), p.r.w, row_h), x0: p.x0, x1: p.x1, y0: -1.0, y1: 1.0 };
        let big = series.iter().map(|x| x.1.abs()).fold(1.0, f64::max) as f32;
        let mid = q.py(0.0);
        ui.line((q.r.x, mid), (q.r.right(), mid), 1.0, theme::BORDER_STRONG, 0.0);
        ui.text(r.x, q.r.y - 16.0, label, theme::SMALL.color(color));
        let bw = ((q.px(1.0) - q.px(0.0)) * 0.42).max(4.0);
        for (t, d) in series.iter() {
            let h = (*d as f32 / big) * (q.r.h / 2.0 - 2.0);
            let x = q.px(*t as f32) - bw / 2.0;
            let col = color.alpha(if *t > s.turn { 80 } else { 210 });
            if h >= 0.0 {
                ui.fill(Rect::new(x, mid - h, bw, h.max(1.0)), col, 2.0);
            } else {
                ui.fill(Rect::new(x, mid, bw, -h), col.alpha(90), 2.0);
            }
            if d.abs() >= 0.5 && (h.abs() > 10.0) {
                let ty = if h >= 0.0 { mid - h - 13.0 } else { mid - h + 1.0 };
                ui.text_in(Rect::new(x - 10.0, ty, bw + 20.0, 12.0), &format!("{d:+.0}"), theme::SMALL.color(col), Align::Center);
            }
        }
    }
    let q_bottom = y + 16.0 + 2.0 * row_h + 24.0;
    let _ = now_x(s);

    // The next jump in the Invader step.
    let next_jump = d_inv.iter().filter(|(t, d)| *t > s.turn && *d >= 2.0).map(|(t, d)| (*t, *d)).next();
    let mut fy = q_bottom + 12.0;
    if let Some((t, d)) = next_jump {
        let what = c.turns.iter().find(|x| x.turn == t).map(|x| {
            let prev = c.turns.iter().find(|p| p.turn + 1 == t);
            let steps = [("Ravage", &x.ravage, prev.map(|p| &p.ravage)), ("Build", &x.build, prev.map(|p| &p.build)), ("Explore", &x.explore, prev.map(|p| &p.explore))];
            steps.iter().filter(|(_, now, before)| before.is_some_and(|b| now.stage > b.stage)).map(|(n, now, _)| format!("Stage {} reaches {n}", fmt::roman(now.stage as i64))).collect::<Vec<_>>().join(", ")
        });
        let jump = format!("+{d:.0} lands a turn on T{t}");
        let why = what.filter(|w| !w.is_empty()).map(|w| format!(" ({w})")).unwrap_or_default();
        fy += line(ui, fy, &[("Next jump: ", theme::TEXT_2, false), (jump.as_str(), RAVAGE, true), (why.as_str(), theme::TEXT_2, false)]) + 4.0;
    }
    let note = "Growth combines a card play at 2.5 Energy (Rei's chart; contested); the band spans 1.5 to 3.5, both tracks.";
    ui.text_wrap(r.x, fy, r.w, note, theme::SMALL.color(theme::FAINT));
}
