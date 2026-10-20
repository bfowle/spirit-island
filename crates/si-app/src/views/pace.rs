//! The Pace, beside the Fear ledger (Lumia's Fear arithmetic over time):
//! the running Fear total from the start of the game against the totals
//! that mark each Terror Level and victory, the pace that clears the deck
//! before the Invader deck runs out, and the pace so far carried forward.
//! Each turn's Fear stands as a bar along the bottom, so a slowing or
//! quickening engine shows as the bars' slope. Beside it, the other way to
//! win: the board. Each Terror Level still ahead, what blocks its victory
//! now, how that count has moved, and where it heads unanswered.

use super::chart::{self, Plot};
use super::runway::now_x;
use si_common::companion_stats::CompanionStats;
use si_common::companion_stats::pace::blockers;
use si_common::public::PublicSnapshot;
use si_ui::theme::{self, Rgba};
use si_ui::{Align, Pt, Rect, Ui};

const SKY: Rgba = Rgba::hex(0x7cc4ff);

pub fn min_height() -> f32 {
    214.0
}

/// `base`: the game as it is, when `stats` is a what-if.
pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, base: Option<&CompanionStats>) {
    let f = &stats.fear_pace;
    let head = theme::SMALL.bold().color(SKY);
    let (hw, _) = ui.text(r.x, r.y, "PACE", head);
    ui.text(r.x + hw + 8.0, r.y, "Fear toward victory before the Invader deck runs out", theme::SMALL.color(theme::MUTED));
    let deck_out = f.deck_out.unwrap_or(s.turn + 1);
    // Key, in the header.
    let kx = r.right() - 190.0;
    chart::dashed(ui, &[(kx, r.y + 8.0), (kx + 22.0, r.y + 8.0)], 2.0, theme::AMBER, 7.0, 5.0);
    ui.text(kx + 28.0, r.y, "pace to win", theme::SMALL.color(theme::AMBER));
    chart::dashed(ui, &[(kx + 100.0, r.y + 8.0), (kx + 122.0, r.y + 8.0)], 1.5, SKY, 3.0, 4.0);
    ui.text(kx + 128.0, r.y, "your pace", theme::SMALL.color(SKY));

    let caption_h = 40.0;
    let victory = f.marks.last().map(|m| m.1).unwrap_or(f.total).max(1) as f32;
    let board_w = 230.0;
    let chart_r = Rect::new(r.x + 30.0, r.y + 30.0, r.w - 120.0 - board_w - 24.0, r.h - 30.0 - caption_h - 22.0);
    let p = Plot { r: chart_r, x0: 0.6, x1: deck_out as f32 + 0.4, y0: 0.0, y1: victory * 1.04 };
    let now = now_x(s);

    ui.layer(si_ui::LAYER_MAP);
    chart::turn_axis(ui, &p, s.turn);
    // The marks ahead, and the levels already reached.
    for (name, at) in &f.marks {
        let c = if name == "Victory" { theme::GREEN } else { SKY.alpha(200) };
        chart::hline(ui, &p, *at as f32, c, "");
        let label = format!("{} · {at}", name.replace("Terror Level", "Terror"));
        ui.text(p.r.right() + 8.0, p.py(*at as f32) - 8.0, &label, theme::SMALL.color(c));
    }
    for (turn, lvl) in &f.level_ups {
        let x = p.px(*turn as f32);
        ui.line((x, p.r.y), (x, p.r.bottom()), 1.5, SKY.alpha(120), 0.0);
        ui.text(x + 3.0, p.r.y, &format!("Terror {}", super::fmt::roman(*lvl as i64)), theme::SMALL.color(SKY.alpha(200)));
    }

    // Each turn's Fear as a bar along the bottom (a quarter of the height).
    let most = f.per_turn.iter().map(|x| x.1).max().unwrap_or(1).max(1) as f32;
    let bar_h = p.r.h * 0.25;
    for (t, v) in &f.per_turn {
        let x = p.px(*t as f32);
        let h = bar_h * (*v).max(0) as f32 / most;
        let current = *t == s.turn;
        ui.fill(Rect::new(x - 7.0, p.r.bottom() - h, 14.0, h), SKY.alpha(if current { 70 } else { 110 }), 2.0);
        if *v > 0 {
            ui.text_in(Rect::new(x - 15.0, p.r.bottom() - h - 15.0, 30.0, 14.0), &v.to_string(), theme::SMALL.color(SKY), Align::Center);
        }
    }

    // The running total so far, filled to the axis.
    let mut pts: Vec<Pt> = f.totals.iter().filter(|(t, _)| *t < s.turn).map(|(t, v)| p.pt(*t as f32, *v as f32)).collect();
    let here = p.pt(now, f.total as f32);
    pts.push(here);
    let floor: Vec<Pt> = pts.iter().map(|q| (q.0, p.r.bottom())).collect();
    chart::band(ui, &pts, &floor, SKY.alpha(55), SKY.alpha(8));
    chart::glow_line(ui, &pts, 2.0, SKY);

    // The pace that wins in time, and the pace so far carried forward.
    let end = deck_out as f32;
    if let Some(need) = f.need {
        let at_end = (f.total as f32 + need as f32 * (end - now)).min(p.y1);
        chart::dashed(ui, &[here, p.pt(end, at_end)], 2.0, theme::AMBER, 7.0, 5.0);
    }
    let rates: Vec<i64> = f.per_turn.iter().filter(|(t, _)| *t < s.turn).rev().take(3).map(|x| x.1).collect();
    if let (Some(recent), false) = (f.recent, rates.is_empty()) {
        let lo = *rates.iter().min().unwrap_or(&0) as f32;
        let hi = *rates.iter().max().unwrap_or(&0) as f32;
        let span = end - now;
        let y = |rate: f32| (f.total as f32 + rate * span).min(p.y1);
        // The spread of the last three turns' paces, as a fan.
        let fan_hi = [here, p.pt(end, y(hi))];
        let fan_lo = [here, p.pt(end, y(lo))];
        chart::band(ui, &fan_hi, &fan_lo, SKY.alpha(40), SKY.alpha(18));
        chart::dashed(ui, &[here, p.pt(end, y(recent as f32))], 1.5, SKY, 3.0, 4.0);
    }
    let breath = ui.pulse(2.4);
    ui.circle(here, 7.0 + 3.0 * breath, SKY.alpha((50.0 + 50.0 * breath) as u8), theme::TRANSPARENT, 0.0);
    ui.circle(here, 4.5, Rgba::hex(0xdff1ff), theme::TRANSPARENT, 0.0);
    ui.layer(si_ui::LAYER_BASE);
    board_route(ui, Rect::new(r.right() - board_w, r.y + 26.0, board_w, chart_r.h + 26.0), s, stats, base);

    // Caption: the budget ÷ the time left, then the pace so far.
    let cy = r.bottom() - caption_h + 4.0;
    let left = (victory as i64 - f.total).max(0);
    let mut line1: Vec<(String, Rgba, bool)> = vec![("Victory needs ".into(), theme::TEXT_2, false)];
    line1.push((format!("{left} more Fear in {} turns", f.turns_left), theme::TEXT, true));
    if let Some(was) = base.map(|b| (b.fear_pace.marks.last().map(|m| m.1).unwrap_or(0) - b.fear_pace.total).max(0)).filter(|w| *w != left) {
        line1.push((format!(" (was {was})"), theme::AMBER, true));
    }
    line1.push((format!(" (the deck runs out at T{deck_out}'s Explore)"), theme::TEXT_2, false));
    if let Some(need) = f.need {
        line1.push((": ".into(), theme::TEXT_2, false));
        line1.push((format!("{need:.1} a turn"), theme::AMBER, true));
    }
    line1.push((".".into(), theme::TEXT_2, false));
    let parts: Vec<(&str, Rgba, bool)> = line1.iter().map(|(t, c, b)| (t.as_str(), *c, *b)).collect();
    chart::caption(ui, r.x, cy, r.w, &parts);
    let mut line2: Vec<(String, Rgba, bool)> = Vec::new();
    match f.recent {
        Some(recent) => {
            let last: Vec<String> = rates.iter().rev().map(|v| v.to_string()).collect();
            line2.push((format!("Last {} turn{}: {} (", last.len(), if last.len() == 1 { "" } else { "s" }, last.join(", ")), theme::TEXT_2, false));
            line2.push((format!("{recent:.1} a turn"), SKY, true));
            line2.push((")".into(), theme::TEXT_2, false));
            // The marks reached in time, then the first the deck beats.
            let mut ahead: Vec<String> = f.reach.iter().filter_map(|(name, t)| t.map(|t| format!("{} on T{t}", name.replace("Terror Level", "Terror")))).collect();
            if let Some((name, _)) = f.reach.iter().find(|(_, t)| t.is_none()) {
                ahead.push(format!("the deck runs out before {}", name.replace("Terror Level", "Terror")));
            }
            line2.push((format!(". At that pace: {}.", ahead.join("; ")), theme::TEXT_2, false));
        }
        None => line2.push(("No finished turn yet to take a pace from.".into(), theme::MUTED, false)),
    }
    let parts: Vec<(&str, Rgba, bool)> = line2.iter().map(|(t, c, b)| (t.as_str(), *c, *b)).collect();
    chart::caption(ui, r.x, cy + 18.0, r.w, &parts);
}

/// The board route: per Terror Level still ahead, the Invaders blocking its
/// victory now, so far (solid) and unanswered (dashed, the middle order).
fn board_route(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, base: Option<&CompanionStats>) {
    let f = &stats.fear_pace;
    let o = &stats.outlook;
    ui.fill(Rect::new(r.x - 12.0, r.y, 1.0, r.h), theme::BORDER, 0.0);
    ui.text(r.x, r.y, "OR CLEAR THE BOARD", theme::SMALL.color(theme::MUTED));
    let block = ((r.h - 20.0) / f.board.len().max(1) as f32).min(56.0);
    for (k, route) in f.board.iter().enumerate() {
        let y = r.y + 20.0 + k as f32 * block;
        let when = if route.current {
            "now".to_string()
        } else {
            route.arrives.map(|t| format!("≈T{t} at your pace")).unwrap_or("not before the deck runs out".into())
        };
        let (w, _) = ui.text(r.x, y, &format!("Terror {}", super::fmt::roman(route.level as i64)), theme::SMALL.bold().color(SKY));
        ui.text(r.x + w + 6.0, y, &when, theme::SMALL.color(if route.current { theme::GREEN } else { theme::MUTED }));
        let num = si_ui::theme::TextStyle { size: 18.0, line: 22.0, mono: true, weight: 700, color: if route.now == 0 { theme::GREEN } else { theme::TEXT } };
        let (nw, _) = ui.text(r.x, y + 17.0, &route.now.to_string(), num);
        let cond = route.condition.trim_start_matches("no ").to_string();
        let was = base.and_then(|b| b.fear_pace.board.iter().find(|x| x.level == route.level)).map(|x| x.now).filter(|w| *w != route.now);
        let cond = match was {
            Some(w) => format!("{cond} (was {w})"),
            None => cond,
        };
        ui.text(r.x + nw + 6.0, y + 21.0, &cond, theme::SMALL.color(if was.is_some() { theme::AMBER } else { theme::TEXT_2 }));
        // Sparkline: the count so far, then unanswered.
        let mut ahead: Vec<(u32, u32)> = Vec::new();
        for (j, t) in o.turns.iter().enumerate() {
            let mut v: Vec<(u32, u64)> = o.paths.iter().filter_map(|p| p.invaders.get(j).map(|i| (blockers(route.level, i[0], i[1], i[2]), p.orders))).collect();
            v.sort_by_key(|x| x.0);
            let total: u64 = v.iter().map(|x| x.1).sum();
            let mut acc = 0;
            if let Some((m, _)) = v.iter().find(|x| {
                acc += x.1;
                acc * 2 >= total
            }) {
                ahead.push((*t, *m));
            }
        }
        let past: Vec<(u32, u32)> = route.history.iter().copied().filter(|(t, _)| *t < s.turn).chain([(s.turn, route.now)]).collect();
        let (t0, t1) = (past.first().map(|x| x.0).unwrap_or(s.turn), ahead.last().map(|x| x.0).unwrap_or(s.turn).max(s.turn));
        let top = past.iter().chain(&ahead).map(|x| x.1).max().unwrap_or(1).max(1) as f32;
        let spark = Rect::new(r.x + 120.0, y + 17.0, r.w - 124.0, block - 26.0);
        let pt = |t: u32, v: u32| (spark.x + (t - t0) as f32 / (t1 - t0).max(1) as f32 * spark.w, spark.bottom() - v as f32 / top * spark.h);
        let solid: Vec<si_ui::Pt> = past.iter().map(|(t, v)| pt(*t, *v)).collect();
        chart::polyline(ui, &solid, 1.5, SKY);
        let dash: Vec<si_ui::Pt> = std::iter::once((s.turn, route.now)).chain(ahead.iter().copied().filter(|x| x.0 > s.turn)).map(|(t, v)| pt(t, v)).collect();
        chart::dashed(ui, &dash, 1.2, theme::RED.alpha(200), 3.0, 3.0);
        if let Some(last) = solid.last() {
            ui.circle(*last, 2.5, Rgba::hex(0xdff1ff), theme::TRANSPARENT, 0.0);
        }
        if let Some((t, v)) = ahead.last() {
            ui.text(pt(*t, *v).0 - 14.0, spark.y - 13.0, &v.to_string(), theme::SMALL.color(theme::RED.alpha(220)));
        }
    }
}
