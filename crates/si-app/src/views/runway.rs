//! The Runway, beside the Blight ledger: the Blight Card over the turns,
//! so far and played forward for every order the unseen Invader cards can
//! come in, with no Spirit acting (Lumia's darkest and best timelines).
//! Each line is one outcome, drawn stronger the more card orders lead to
//! it; the band holds the middle 80% of orders. Below zero is past the
//! card, shaded by the exact chance the island is lost that deep (the
//! blighted side is unseen). The cards that can come next sit beside the
//! chart, darkest first; hovering one lights its futures.

use super::chart::{self, Plot};
use super::{fmt, step_index};
use si_common::companion_stats::{CompanionStats, RoundSummary, p_loss};
use si_common::public::PublicSnapshot;
use si_ui::theme::{self, Rgba};
use si_ui::{Align, Pt, Rect, Ui};

const VIOLET: Rgba = Rgba::hex(0xb794f6);
const LOSS: Rgba = theme::RED;

/// Weighted quantile `q` of `(value, weight)` pairs.
fn quantile(v: &mut [(i64, u64)], q: f64) -> i64 {
    v.sort_by_key(|x| x.0);
    let total: u64 = v.iter().map(|x| x.1).sum();
    let mut acc = 0;
    for (x, w) in v.iter() {
        acc += w;
        if acc as f64 >= q * total as f64 {
            return *x;
        }
    }
    v.last().map(|x| x.0).unwrap_or(0)
}

/// Where "now" sits on a turn axis whose integer marks are ends of turns.
pub fn now_x(s: &PublicSnapshot) -> f32 {
    s.turn as f32 - 1.0 + (step_index(&s.step) as f32 + 0.5) / 8.0
}

pub fn min_height() -> f32 {
    256.0
}

/// The middle order's Blight left on the card at each turn.
fn middle(o: &si_common::companion_stats::outlook::Outlook) -> Vec<i64> {
    let cums: Vec<Vec<i64>> = o.paths.iter().map(|p| p.cumulative()).collect();
    (0..o.turns.len())
        .map(|j| {
            let mut v: Vec<(i64, u64)> = o.paths.iter().zip(&cums).map(|(p, c)| (c.get(j).or(c.last()).copied().unwrap_or(0), p.orders)).collect();
            o.blight_left - quantile(&mut v, 0.5)
        })
        .collect()
}

/// `base`: the game as it is, when `stats` is a what-if.
pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, history: &[RoundSummary], base: Option<&CompanionStats>) {
    let o = &stats.outlook;
    let head = theme::SMALL.bold().color(VIOLET);
    let (hw, _) = ui.text(r.x, r.y, "RUNWAY", head);
    ui.text(r.x + hw + 8.0, r.y, "if nothing answers the Invaders", theme::SMALL.color(theme::MUTED));
    if o.turns.is_empty() || o.orders == 0 {
        ui.text(r.x, r.y + 28.0, "No Invader Phase left to play forward.", theme::SMALL.color(theme::MUTED));
        return;
    }
    let orders_label = format!("{}{} card order{}", if base.is_some() { "what-if · " } else { "" }, o.orders, if o.orders == 1 { "" } else { "s" });
    ui.text_in(Rect::new(r.x, r.y, r.w, 16.0), &orders_label, theme::SMALL.color(theme::MUTED), Align::Right);

    let left = o.blight_left;
    let last = *o.turns.last().unwrap_or(&s.turn);
    let cums: Vec<Vec<i64>> = o.paths.iter().map(|p| p.cumulative()).collect();
    let at = |c: &Vec<i64>, j: usize| c.get(j).or(c.last()).copied().unwrap_or(0);
    let worst = cums.iter().map(|c| at(c, o.turns.len() - 1)).max().unwrap_or(0);

    // Layout: chart on the left, the next cards on the right, caption below.
    let caption_h = 58.0;
    let body = Rect::new(r.x, r.y + 24.0, r.w, r.h - 24.0 - caption_h);
    let legend_w = (r.w * 0.36).clamp(200.0, 290.0);
    let chart_r = Rect::new(body.x + 30.0, body.y + 18.0, body.w - legend_w - 52.0, body.h - 40.0);
    let hist: Vec<(u32, i64)> = history.iter().filter(|h| h.turn + 3 >= s.turn && h.turn < s.turn).map(|h| (h.turn, h.blight_remaining)).collect();
    let x0 = hist.first().map(|h| h.0 as f32).unwrap_or(now_x(s)).min(now_x(s)) - 0.25;
    let top = hist.iter().map(|h| h.1).chain([left]).max().unwrap_or(left) as f32 + 0.8;
    // Down to where the island is surely lost: past the deepest blighted
    // side (a card already on it loses at zero, or draws a new one).
    let deepest = s.blight.pool.iter().filter_map(|o| o.flipped_total).max().unwrap_or(6);
    // Shallower when the loss is near-certain sooner.
    let sure = (0..=deepest).find(|d| p_loss(left + d + 1, &s.blight) >= 0.95).map(|d| d + 2).unwrap_or(deepest + 1);
    let floor = if o.flipped && !s.blight.current_still_healthy { 2 } else { sure };
    let bottom = ((left - worst) as f32).max(-(floor as f32)).min(-2.0) - 0.6;
    let p = Plot { r: chart_r, x0, x1: last as f32 + 0.35, y0: bottom, y1: top };

    // Hovered next card, from the legend.
    let mut order: Vec<usize> = (0..o.next_cards.len()).collect();
    let range = |i: usize| -> (i64, i64) {
        let v: Vec<i64> = o.paths.iter().zip(&cums).filter(|(p, _)| p.first == Some(i)).map(|(_, c)| at(c, o.turns.len() - 1)).collect();
        (v.iter().copied().min().unwrap_or(0), v.iter().copied().max().unwrap_or(0))
    };
    order.sort_by_key(|i| std::cmp::Reverse((range(*i).1, range(*i).0)));
    let legend = Rect::new(body.right() - legend_w, body.y, legend_w, body.h);
    let row_h = 21.0;
    let rows_y = legend.y + 20.0;
    let hovered = order.iter().enumerate().find(|(k, _)| ui.hovered(Rect::new(legend.x - 6.0, rows_y + *k as f32 * row_h, legend.w + 6.0, row_h))).map(|(_, i)| *i);

    ui.layer(si_ui::LAYER_MAP);
    ui.push_clip(Rect::new(p.r.x - 2.0, p.r.y - 2.0, p.r.w + 4.0, p.r.h + 4.0));
    // Past the card: shaded by the exact chance the island is lost that
    // deep, with the depths where it passes 1 in 4, 1 in 2, 3 in 4.
    let mut d = 0;
    let mut marks = [(0.25, "1 in 4 lost"), (0.5, "1 in 2 lost"), (0.75, "3 in 4 lost"), (0.999, "lost")].into_iter().peekable();
    while (-(d as f32)) > p.y0 {
        let (ya, yb) = (-(d as f32), (-(d as f32) - 1.0).max(p.y0));
        let risk = p_loss(left + d + 1, &s.blight);
        let c = LOSS.alpha((8.0 + 80.0 * risk as f32) as u8);
        let (xa, xb) = (p.r.x, p.r.right());
        ui.quad([(xa, p.py(ya)), (xb, p.py(ya)), (xb, p.py(yb)), (xa, p.py(yb))], [c, c, c, c]);
        let mut label = None;
        while let Some((at, text)) = marks.peek().copied() {
            if risk < at {
                break;
            }
            label = Some(text);
            marks.next();
        }
        // On the left, under "now": the lines fall away to the right.
        if let Some(text) = label {
            ui.line((p.r.x, p.py(yb)), (p.r.x + 10.0, p.py(yb)), 1.0, LOSS, 0.0);
            ui.text(p.r.x + 14.0, p.py(yb) - 8.0, text, theme::SMALL.color(Rgba::hex(0xff9a98)));
        }
        d += 1;
    }
    chart::turn_axis(ui, &p, s.turn);
    let zero = p.py(0.0);
    ui.line((p.r.x, zero), (p.r.right(), zero), 1.5, LOSS.alpha(200), 0.0);
    let edge = if o.flipped { "card empty" } else { "card flips: blighted side unseen" };
    ui.text(p.r.x + 4.0, zero + 3.0, edge, theme::SMALL.color(Rgba::hex(0xff9a98)));
    ui.pop_clip();
    ui.text_in(Rect::new(p.r.x - 30.0, p.py(left as f32) - 8.0, 24.0, 16.0), &left.to_string(), theme::SMALL.color(theme::MUTED), Align::Right);
    ui.text_in(Rect::new(p.r.x - 30.0, zero - 8.0, 24.0, 16.0), "0", theme::SMALL.color(LOSS), Align::Right);
    ui.push_clip(Rect::new(p.r.x - 2.0, p.r.y - 12.0, p.r.w + 4.0, p.r.h + 14.0));

    // The middle 80% of orders, then each outcome, then the middle order.
    let start: Pt = (p.px(now_x(s)), p.py(left as f32));
    let qs: Vec<(Pt, Pt, Pt)> = (0..o.turns.len())
        .map(|j| {
            let mut v: Vec<(i64, u64)> = o.paths.iter().zip(&cums).map(|(p, c)| (left - at(c, j), p.orders)).collect();
            let x = p.px(o.turns[j] as f32);
            let (lo, mid, hi) = (quantile(&mut v, 0.1), quantile(&mut v, 0.5), quantile(&mut v, 0.9));
            ((x, p.py(hi as f32)), (x, p.py(mid as f32)), (x, p.py(lo as f32)))
        })
        .collect();
    let upper: Vec<Pt> = std::iter::once(start).chain(qs.iter().map(|q| q.0)).collect();
    let lower: Vec<Pt> = std::iter::once(start).chain(qs.iter().map(|q| q.2)).collect();
    chart::band(ui, &upper, &lower, VIOLET.alpha(60), VIOLET.alpha(25));
    let heaviest = o.paths.iter().map(|p| p.orders).max().unwrap_or(1) as f32;
    for (path, cum) in o.paths.iter().zip(&cums) {
        let pts: Vec<Pt> = std::iter::once(start).chain(o.turns.iter().enumerate().map(|(j, t)| p.pt(*t as f32, (left - at(cum, j)) as f32))).collect();
        let lit = hovered.is_some() && path.first == hovered;
        let weight = path.orders as f32 / heaviest;
        let (w, c) = match hovered {
            Some(_) if lit => (2.2, theme::AMBER.alpha(230)),
            Some(_) => (1.0, VIOLET.alpha(22)),
            None => (1.0 + weight, VIOLET.alpha((40.0 + 110.0 * weight) as u8)),
        };
        if lit {
            chart::glow_line(ui, &pts, w, c);
        } else {
            chart::polyline(ui, &pts, w, c);
        }
    }
    // The game as it is: its middle order, faint, under the what-if's.
    if let Some(b) = base {
        let was: Vec<Pt> = std::iter::once(start).chain(b.outlook.turns.iter().zip(middle(&b.outlook)).map(|(t, v)| p.pt(*t as f32, v as f32))).collect();
        chart::dashed(ui, &was, 2.2, theme::TEXT_2.alpha(220), 6.0, 4.0);
        if let Some(end) = was.last() {
            ui.text(end.0 - 30.0, end.1 - 18.0, "was", theme::SMALL.bold().color(theme::TEXT_2));
        }
    }
    if hovered.is_none() {
        let mid: Vec<Pt> = std::iter::once(start).chain(qs.iter().map(|q| q.1)).collect();
        chart::glow_line(ui, &mid, 2.5, Rgba::hex(0xe9dcff));
    }

    // What has happened, and now.
    let past: Vec<Pt> = hist.iter().map(|(t, v)| p.pt(*t as f32, *v as f32)).chain([start]).collect();
    chart::polyline(ui, &past, 2.0, theme::TEXT_2);
    for pt in &past[..past.len() - 1] {
        ui.circle(*pt, 3.0, theme::TEXT_2, theme::TRANSPARENT, 0.0);
    }
    let breath = ui.pulse(2.4);
    ui.circle(start, 7.0 + 3.0 * breath, VIOLET.alpha((50.0 + 50.0 * breath) as u8), theme::TRANSPARENT, 0.0);
    ui.circle(start, 4.5, Rgba::hex(0xe9dcff), theme::TRANSPARENT, 0.0);
    ui.pop_clip();

    // Each turn's Blight (the middle order), over its column.
    for j in 0..o.turns.len() {
        let mut v: Vec<(i64, u64)> = o.paths.iter().map(|p| (p.blight.get(j).copied().unwrap_or(0), p.orders)).collect();
        let (lo, hi) = (quantile(&mut v, 0.1), quantile(&mut v, 0.9));
        let label = if lo == hi { format!("+{lo}") } else { format!("+{lo}–{hi}") };
        let x = p.px(o.turns[j] as f32);
        ui.text_in(Rect::new(x - 30.0, p.r.y - 18.0, 60.0, 14.0), &label, theme::SMALL.bold().color(theme::AMBER), Align::Center);
    }
    ui.layer(si_ui::LAYER_BASE);

    // The next card, darkest first.
    let first_turn = o.next_cards.first().map(|c| c.turn).unwrap_or(s.turn);
    ui.text(legend.x, legend.y, &format!("IF T{first_turn}'S EXPLORE CARD IS"), theme::SMALL.color(theme::MUTED));
    for (k, i) in order.iter().enumerate() {
        let c = &o.next_cards[*i];
        let y = rows_y + k as f32 * row_h;
        if y + row_h > legend.bottom() + 4.0 {
            break;
        }
        let row = Rect::new(legend.x - 6.0, y, legend.w + 6.0, row_h);
        if hovered == Some(*i) {
            ui.fill(row, theme::RAISED, 5.0);
        }
        let mut x = legend.x;
        for (n, t) in c.card.terrains.iter().enumerate() {
            if n > 0 {
                let (w, _) = ui.text(x, y + 3.0, "+", theme::SMALL.color(theme::MUTED));
                x += w + 3.0;
            }
            let (w, _) = ui.text(x, y + 3.0, t, theme::SMALL.bold().color(fmt::terrain(t)));
            x += w + 4.0;
        }
        let (w, _) = ui.text(x + 2.0, y + 3.0, &format!("{} of {}", c.count, c.of), theme::SMALL.color(theme::MUTED));
        x += w + 10.0;
        let _ = x;
        let (lo, hi) = range(*i);
        let total = if lo == hi { format!("+{lo}") } else { format!("+{lo}–{hi}") };
        let tone = if k == 0 && order.len() > 1 { LOSS } else { theme::AMBER };
        ui.text_in(Rect::new(legend.x, y + 3.0, legend.w - 4.0, 15.0), &total, theme::SMALL.bold().color(tone), Align::Right);
    }
    let foot = rows_y + order.len().min(((legend.bottom() - rows_y) / row_h) as usize) as f32 * row_h + 4.0;
    if foot + 16.0 < legend.bottom() + 8.0 {
        ui.text(legend.x, foot, &format!("Blight through T{last}, by the cards after it"), theme::SMALL.color(theme::FAINT));
    }

    // Caption: natural frequencies per turn, then the Lumia shortfall over
    // time (its growth is the snowball).
    let cy = r.bottom() - caption_h + 4.0;
    let n = o.orders;
    let mut line1: Vec<(String, Rgba, bool)> = vec![("Unanswered, ".into(), theme::TEXT_2, false)];
    if !o.flipped {
        match o.flipped_by.iter().position(|f| *f > 0) {
            Some(j) => {
                let all = o.flipped_by[j] == n;
                line1.push((format!("the card flips by T{}", o.turns[j]), theme::TEXT, true));
                line1.push((if all { " in every order.".into() } else { format!(" in {} of {n} orders.", o.flipped_by[j]) }, theme::TEXT_2, false));
            }
            None => line1.push((format!("the card holds through T{last} in every order."), theme::GREEN, true)),
        }
    } else {
        line1.push(("the card is already on its blighted side.".into(), theme::TEXT_2, false));
    }
    let tenths = |p: f64| if p < 0.05 { 0 } else { (p * 10.0).round().max(1.0) as i64 };
    let lost_now = |j: usize| stats.race.cif("blight").and_then(|v| v.get(j).copied()).unwrap_or(0.0);
    let was_lost = |j: usize| base.and_then(|b| b.race.cif("blight")).and_then(|v| v.get(j).copied());
    let lost: Vec<String> = o
        .turns
        .iter()
        .enumerate()
        .filter(|(j, _)| lost_now(*j) >= 0.05 || was_lost(*j).is_some_and(|p| p >= 0.05))
        .map(|(j, t)| match was_lost(j).map(tenths).filter(|w| *w != tenths(lost_now(j))) {
            Some(w) => format!("T{t} {w}→{} in 10", tenths(lost_now(j))),
            None => format!("T{t} {} in 10", tenths(lost_now(j))),
        })
        .collect();
    if !lost.is_empty() {
        line1.push(("  Lost to Blight by: ".into(), theme::TEXT_2, false));
        line1.push((lost.join(" · "), Rgba::hex(0xff9a98), true));
    }
    let parts: Vec<(&str, Rgba, bool)> = line1.iter().map(|(t, c, b)| (t.as_str(), *c, *b)).collect();
    chart::caption(ui, r.x, cy, r.w, &parts);
    let was_mid = base.map(|b| middle(&b.outlook));
    let need: Vec<String> = (0..o.turns.len())
        .map(|j| {
            let mut v: Vec<(i64, u64)> = o.paths.iter().zip(&cums).map(|(p, c)| (at(c, j), p.orders)).collect();
            let now = (quantile(&mut v, 0.5) - (left - 1)).max(0);
            match was_mid.as_ref().and_then(|m| m.get(j)).map(|m| (1 - m).max(0)).filter(|w| *w != now) {
                Some(w) => format!("T{} {w}→{now}", o.turns[j]),
                None => format!("T{} {now}", o.turns[j]),
            }
        })
        .collect();
    let need = need.join(" · ");
    let parts = [
        ("To keep Blight on the card, prevent by: ", theme::TEXT_2, false),
        (need.as_str(), theme::AMBER, true),
        ("  (the middle order, counted from now)", theme::MUTED, false),
    ];
    chart::caption(ui, r.x, cy + 18.0, r.w, &parts);
    ui.text(r.x, cy + 36.0, "Each line is an outcome, stronger the more card orders lead to it; the band holds the middle 80%.", theme::SMALL.color(theme::FAINT));
}
