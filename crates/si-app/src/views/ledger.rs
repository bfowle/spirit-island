//! The Ledger (Lumia's method: state a hard budget, inventory what this
//! turn spends from it in resolution order, report the shortfall). Each
//! budget is a lane of tokens drawn with the game's own icons: what the
//! budget holds, the tokens this turn spends lit and grouped by what spends
//! them, the edge where it runs out, red tokens past the edge (the
//! shortfall), and next turn's spend as ghosts. Below the tokens, the
//! inventory: each spend with its step and the exact solve.
//!
//! Blight and Fear run full width, each with its analysis beside it: the
//! Runway (the Blight Card played forward over every card order) and the
//! Pace (Fear needed a turn against the pace so far). The Invader deck and
//! the adversary's loss clocks (time and limits, few tokens each) share the
//! last row.

use super::{Both, Toggle, fmt, pace, runway, step_index};
use crate::icons;
use si_common::companion_stats::whatif::Edit;
use si_common::companion_stats::{CompanionStats, RoundSummary};
use si_common::companion_stats::ledger::{Budget, Spend};
use si_common::public::PublicSnapshot;
use si_ui::theme::{self, Rgba, TextStyle};
use si_ui::{Align, Rect, Ui, span};

const VIOLET: Rgba = Rgba::hex(0xb794f6);
const SKY: Rgba = Rgba::hex(0x7cc4ff);
const TEAL: Rgba = Rgba::hex(0x4fd1c5);
const CORAL: Rgba = Rgba::hex(0xff8a65);
const SPEND: Rgba = theme::AMBER;
const SHORT: Rgba = theme::RED;

const GAP: f32 = 12.0;
const ROW: f32 = 26.0;
/// An inventory row in a narrow lane: chips, then the label under them.
const ROW_COMPACT: f32 = 44.0;
/// Where the token row starts, below the title line.
const TOKENS_Y: f32 = 32.0;

/// The Ledger. Layout follows the game as it is (`both.base`); the numbers,
/// tokens and charts follow the what-if (`both.tried`), so a change shows as
/// movement within a still page. Clicking a fix tries it.
/// Returns the Fear lane's place: the What-If drawer opens over it, so the
/// Blight lane, the Runway and the loss clocks stay in view while trying.
pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, both: &Both, history: &[RoundSummary], out: &mut Vec<Toggle>) -> Option<Rect> {
    let mut slot = None;
    let off = ui.scroll_begin("ledger", r);
    let w = r.w - 8.0;
    let mut y = r.y - off;
    let pairs: Vec<(&Budget, &Budget)> = both.base.ledger.budgets.iter().enumerate().map(|(i, b)| (b, both.tried.ledger.budgets.get(i).filter(|t| t.key == b.key && t.title == b.title).unwrap_or(b))).collect();
    let (wide, narrow): (Vec<_>, Vec<_>) = pairs.into_iter().partition(|(b, _)| b.key == "blight" || b.key == "fear");
    let base = both.trying().then_some(both.base);
    for (b, t) in wide {
        // Room for whichever has more to say, the game or the what-if.
        let h = lane_height(ui, b, w).max(lane_height(ui, t, w)) + added(b, t, w);
        let lane_r = Rect::new(r.x, y, w, h);
        if b.key == "fear" {
            slot = Some(lane_r);
        }
        lane(ui, lane_r, s, both, b, t, out);
        // The analysis beside the ledger.
        let cw = chart_w(b, w);
        if cw > 0.0 {
            let panel = Rect::new(lane_r.right() - cw + 20.0, y + 14.0, cw - 42.0, h - 26.0);
            ui.fill(Rect::new(panel.x - 20.0, y + 16.0, 1.0, h - 32.0), theme::BORDER, 0.0);
            match b.key.as_str() {
                "blight" => runway::draw(ui, panel, s, both.tried, history, base),
                _ => pace::draw(ui, panel, s, both.tried, base),
            }
        }
        y += h + GAP;
    }
    // Time and the adversary's limits, side by side.
    if !narrow.is_empty() {
        let n = narrow.len() as f32;
        let lw = (w - GAP * (n - 1.0)) / n;
        let h = narrow.iter().map(|(b, t)| lane_height(ui, b, lw).max(lane_height(ui, t, lw)) + added(b, t, lw)).fold(0.0, f32::max);
        for (i, (b, t)) in narrow.iter().enumerate() {
            lane(ui, Rect::new(r.x + i as f32 * (lw + GAP), y, lw, h), s, both, b, t, out);
        }
        y += h + GAP;
    }
    ui.scroll_end("ledger", r, y + off - r.y - GAP);
    slot
}

/// Room for the rows a what-if adds below the game's own.
fn added(base: &Budget, tried: &Budget, full: f32) -> f32 {
    if dense(base) {
        return 0.0;
    }
    let w = full - chart_w(base, full);
    tried.items.iter().filter(|it| same(it, base).is_none()).count() as f32 * row_h(w)
}

/// The same spend in the what-if ledger, if it is still there.
fn same<'a>(it: &Spend, in_budget: &'a Budget) -> Option<(usize, &'a Spend)> {
    in_budget.items.iter().enumerate().find(|(_, x)| x.step == it.step && x.land == it.land)
}

/// The analysis panel's width in a lane `w` wide (0: none).
fn chart_w(b: &Budget, w: f32) -> f32 {
    match b.key.as_str() {
        "blight" | "fear" => (w * 0.46).floor(),
        _ => 0.0,
    }
}

/// The title column's width in a lane `w` wide.
fn title_w(w: f32) -> f32 {
    (w * 0.3).clamp(180.0, 230.0)
}

/// Where the inventory starts: under the tokens, and under their brackets
/// when there are any.
fn body_top(b: &Budget) -> f32 {
    match b.key.as_str() {
        "deck" => 132.0,
        "fear" => 108.0,
        _ if !b.items.is_empty() || b.next.unwrap_or(0) > 0 => 126.0,
        _ => 104.0,
    }
}

fn compact(w: f32) -> bool {
    w - title_w(w) - 24.0 < 560.0
}

fn row_h(w: f32) -> f32 {
    if compact(w) { ROW_COMPACT } else { ROW }
}

fn when_line(b: &Budget) -> String {
    let when = if b.goal {
        "When full"
    } else if b.key == "clock" {
        "At the limit"
    } else {
        "When empty"
    };
    format!("{when}: {}", b.at_zero)
}

fn pocket_rule(b: &Budget) -> String {
    format!("Pocket rule: {}", fmt::plain(&b.pocket_rule))
}

fn wrapped_h(ui: &mut Ui, text: &str, style: TextStyle, w: f32) -> f32 {
    ui.measure_spans(&[span(text, style)], Some(w)).1
}

fn lane_height(ui: &mut Ui, b: &Budget, full: f32) -> f32 {
    let w = full - chart_w(b, full);
    let chart = match b.key.as_str() {
        "blight" => runway::min_height(),
        "fear" => pace::min_height(),
        _ => 0.0,
    };
    let tw = title_w(w);
    let body_w = w - tw - 24.0;
    let title = 98.0 + wrapped_h(ui, &when_line(b), theme::SMALL, tw - 30.0) + 16.0;
    let rule = wrapped_h(ui, &pocket_rule(b), theme::SMALL, body_w);
    let body = match b.key.as_str() {
        "deck" => body_top(b) + 18.0 + rule,
        // Its detail is in the Pace's caption.
        "fear" => body_top(b) + b.items.len() as f32 * ROW + 10.0 + rule,
        _ => {
            let rows = if dense(b) { chip_rows(b.items.len(), body_w) as f32 * CHIP_ROW } else { b.items.len() as f32 * row_h(w) };
            body_top(b) + rows + if rows > 0.0 { 10.0 } else { 0.0 } + 18.0 * b.detail.len() as f32 + rule
        }
    };
    title.max(body + 16.0).max(chart + 26.0)
}

/// A busy turn's spends as chips that wrap (land, amount, fix), so the
/// page still fits; the full row shows on hover.
const DENSE: usize = 5;
const CHIP_W: f32 = 176.0;
const CHIP_ROW: f32 = 30.0;

fn dense(b: &Budget) -> bool {
    b.key != "fear" && b.items.len() > DENSE
}

fn chip_rows(n: usize, w: f32) -> usize {
    let per = (((w + 8.0) / (CHIP_W + 8.0)).floor() as usize).max(1);
    n.div_ceil(per)
}

fn chip_rect(i: usize, x: f32, y: f32, w: f32) -> Rect {
    let per = (((w + 8.0) / (CHIP_W + 8.0)).floor() as usize).max(1);
    Rect::new(x + (i % per) as f32 * (CHIP_W + 8.0), y + (i / per) as f32 * CHIP_ROW, CHIP_W, CHIP_ROW - 6.0)
}

fn accent(b: &Budget) -> Rgba {
    match b.key.as_str() {
        "blight" => VIOLET,
        "fear" => SKY,
        "deck" => TEAL,
        _ => CORAL,
    }
}

/// The token icon for a budget's unit.
fn token_icon(b: &Budget) -> Option<u16> {
    match b.unit.as_str() {
        "Blight" => Some(icons::BLIGHT),
        "Fear" => Some(icons::FEAR),
        "Invader/Blight" => Some(icons::EXPLORER),
        "Town/City" | "Town" => Some(icons::TOWN),
        "Coastal City" => Some(icons::CITY),
        _ => None,
    }
}

/// "Towns", "Beast", "Coastal Cities" for a count of `n`; slashed units stay.
fn units(n: i64, unit: &str) -> String {
    if n == 1 || unit.contains('/') {
        unit.to_string()
    } else if let Some(stem) = unit.strip_suffix('y') {
        format!("{stem}ies")
    } else {
        format!("{unit}s")
    }
}

fn spend_total(b: &Budget) -> i64 {
    b.items.iter().map(|s| s.amount).sum()
}

fn lane(ui: &mut Ui, full: Rect, s: &PublicSnapshot, both: &Both, base: &Budget, b: &Budget, out: &mut Vec<Toggle>) {
    let stats = both.tried;
    let acc = accent(b);
    ui.rect(full, theme::PANEL, theme::BORDER, 1.0, 10.0);
    ui.fill(Rect::new(full.x, full.y + 14.0, 3.0, full.h - 28.0), acc, 1.5);
    // The ledger itself; an analysis panel takes the rest.
    let r = Rect::new(full.x, full.y, full.w - chart_w(b, full.w), full.h);
    let tw = title_w(r.w);

    // Title column: the budget and what's left in it.
    let col = Rect::new(r.x + 22.0, r.y + 16.0, tw - 30.0, r.h - 30.0);
    ui.text(col.x, col.y, &b.title.to_uppercase(), TextStyle { size: 15.0, line: 19.0, mono: false, weight: 700, color: acc });
    let spend = spend_total(b);
    let short = !b.goal && b.shortfall > 0;
    let big = TextStyle { size: 44.0, line: 48.0, mono: true, weight: 700, color: if short { SHORT } else { theme::TEXT } };
    let (bw, _) = ui.text(col.x, col.y + 24.0, &b.left.to_string(), big);
    if b.left != base.left {
        ui.text(col.x + bw + 10.0, col.y + 26.0, &format!("was {}", base.left), theme::SMALL.color(theme::AMBER));
    }
    let left_word = match b.key.as_str() {
        _ if b.goal => format!("{} to go", b.unit),
        "deck" => "cards left".into(),
        // A limit: how many more it takes.
        "clock" => format!("more {}", units(b.left, &b.unit)),
        _ => format!("{} left", b.unit),
    };
    let word = fmt::fit(ui, &left_word, theme::SMALL.color(theme::TEXT_2), col.w - bw - 10.0);
    ui.text(col.x + bw + 10.0, col.y + 44.0, &word, theme::SMALL.color(theme::TEXT_2));
    ui.text_wrap(col.x, col.y + 82.0, col.w, &when_line(b), theme::SMALL.color(theme::MUTED));

    // Verdict, top right; in a what-if, what it was.
    let (said, vcolor) = match &b.urgent {
        // A loss before this turn's spends outranks them.
        Some(u) => (fmt::humanize(s, u), SHORT),
        None => verdict(b, spend),
    };
    if b.urgent.is_some() {
        let pulse = ui.pulse(1.4);
        let w = ui.measure(&said, theme::SMALL.bold().size(13.5)).0;
        ui.glow(Rect::new(r.right() - 22.0 - w - 6.0, r.y + 12.0, w + 12.0, 24.0), SHORT.alpha((40.0 + 60.0 * pulse) as u8), 8.0, 10.0);
    }
    let vstyle = TextStyle { size: 13.5, line: 18.0, mono: false, weight: 700, color: vcolor };
    let vw = ui.text_in(Rect::new(r.x, r.y + 14.0, r.w - 22.0, 20.0), &said, vstyle, Align::Right);
    let (was, _) = match &base.urgent {
        Some(u) => (fmt::humanize(s, u), SHORT),
        None => verdict(base, spend_total(base)),
    };
    if was != said {
        // Between the title and the verdict, cut to fit.
        let title_end = col.x + ui.measure(&b.title.to_uppercase(), TextStyle { size: 15.0, line: 19.0, mono: false, weight: 700, color: acc }).0 + 16.0;
        let room = (r.right() - 30.0 - vw - title_end).max(0.0);
        let was = fmt::fit(ui, &format!("was {was}"), theme::SMALL.color(theme::MUTED), (room - 20.0).max(0.0));
        if room > 60.0 {
            ui.text_in(Rect::new(title_end, r.y + 14.0, room, 20.0), &format!("{was}  →"), theme::SMALL.color(theme::MUTED), Align::Right);
        }
    }

    // Tokens.
    let body_w = r.w - tw - 24.0;
    let area = Rect::new(r.x + tw, r.y + TOKENS_Y, body_w, 72.0);
    // Hover is on the game's rows; the tokens are the what-if's.
    let hovered_base = if dense(base) {
        (0..base.items.len()).find(|i| ui.hovered(chip_rect(*i, r.x + tw, r.y + body_top(base), r.w - tw - 24.0)))
    } else {
        hovered_row(ui, r, base)
    };
    let hovered_item = hovered_base.and_then(|i| same(&base.items[i], b)).map(|(j, _)| j);
    match b.key.as_str() {
        "fear" => fear_tokens(ui, area, stats, b, acc, hovered_item),
        "deck" => deck_tokens(ui, area, s, stats, b),
        _ => budget_tokens(ui, area, s, b, acc, hovered_item),
    }

    // Inventory: each spend in resolution order, with its step and solve.
    let mut y = r.y + body_top(base);
    let x = r.x + tw;
    let rule_style = theme::SMALL.color(theme::MUTED);
    if b.key == "deck" {
        let r = &stats.curve.stage_reach;
        let line = format!("Under each card: the lands a card of its stage names here, on average (I {:.1} · II {:.1} · III {:.1}).", r[0], r[1], r[2]);
        let line = fmt::fit(ui, &line, theme::SMALL.color(theme::TEXT_2), body_w);
        ui.text(x, y, &line, theme::SMALL.color(theme::TEXT_2));
        ui.text_wrap(x, y + 18.0, body_w, &pocket_rule(b), rule_style);
        return;
    }
    let w = body_w;
    let now = step_index(&s.step);
    let narrow = compact(r.w);
    let rh = if narrow { ROW_COMPACT } else { ROW };
    if dense(base) {
        for (i, it) in base.items.iter().enumerate() {
            let tried = it.land.as_ref().and_then(|l| both.edits.iter().find(|e| matches!(e, Edit::Defend { land, .. } if land == l)));
            let shown = same(it, b).map(|(_, x)| x);
            let row = Row { hot: hovered_base == Some(i), goal: b.goal, narrow, prevented: shown.is_none(), tried: tried.is_some() };
            if spend_chip(ui, chip_rect(i, x, y, w), s, shown.unwrap_or(it), now, row) {
                match (tried, &it.land, it.defend) {
                    (Some(e), _, _) => out.push(Toggle::Remove(e.clone())),
                    (None, Some(land), Some(amount)) => out.push(Toggle::Add(Edit::Defend { land: land.clone(), amount })),
                    _ => {}
                }
            }
        }
        y += chip_rows(base.items.len(), w) as f32 * CHIP_ROW;
    }
    for (i, it) in base.items.iter().enumerate().filter(|_| !dense(base)) {
        let tried = it.land.as_ref().and_then(|l| both.edits.iter().find(|e| matches!(e, Edit::Defend { land, .. } if land == l)));
        let shown = same(it, b).map(|(_, x)| x);
        let row = Row { hot: hovered_base == Some(i), goal: b.goal, narrow, prevented: shown.is_none(), tried: tried.is_some() };
        if inventory_row(ui, Rect::new(x, y, w, rh), s, shown.unwrap_or(it), now, row) {
            match (tried, &it.land, it.defend) {
                (Some(e), _, _) => out.push(Toggle::Remove(e.clone())),
                (None, Some(land), Some(amount)) => out.push(Toggle::Add(Edit::Defend { land: land.clone(), amount })),
                _ => {}
            }
        }
        y += rh;
    }
    // Spends the what-if adds (the clock moving to another land).
    for it in b.items.iter().filter(|it| same(it, base).is_none() && !dense(base)) {
        let row = Row { hot: false, goal: b.goal, narrow, prevented: false, tried: false };
        inventory_row(ui, Rect::new(x, y, w, rh), s, it, now, row);
        ui.text(x - 14.0, y + 4.0, "+", theme::SMALL.bold().color(theme::AMBER));
        y += rh;
    }
    if !base.items.is_empty() {
        y += 10.0;
    }
    if b.key != "fear" {
        for d in &b.detail {
            let line = fmt::fit(ui, &fmt::humanize(s, d), theme::SMALL.color(theme::TEXT_2), w);
            ui.text(x, y, &line, theme::SMALL.color(theme::TEXT_2));
            y += 18.0;
        }
    }
    let _ = stats;
    ui.text_wrap(x, y, w, &pocket_rule(b), rule_style);
}

/// The lane's one-line verdict.
fn verdict(b: &Budget, spend: i64) -> (String, Rgba) {
    if b.goal {
        return if spend == 0 {
            (format!("{} {} to the next card", b.left, b.unit), theme::TEXT_2)
        } else if b.shortfall == 0 {
            (format!("+{spend} this turn (best case): earns the card"), theme::GREEN)
        } else {
            (format!("+{spend} this turn (best case), {} still to find", b.shortfall), theme::TEXT_2)
        };
    }
    if spend == 0 {
        (if b.key == "deck" { format!("{} turns of cards", b.left) } else { "holds this turn".into() }, theme::GREEN)
    } else if b.shortfall > 0 {
        (format!("−{spend} this turn  ·  prevent {} to hold", b.shortfall), SHORT)
    } else {
        (format!("−{spend} this turn  ·  holds, {} left after", b.left - spend), SPEND)
    }
}

/// Which inventory row the mouse is over (rows sit under the token row).
fn hovered_row(ui: &Ui, r: Rect, b: &Budget) -> Option<usize> {
    let y0 = r.y + body_top(b);
    let tw = title_w(r.w);
    let rh = if compact(r.w) { ROW_COMPACT } else { ROW };
    (0..b.items.len()).find(|i| ui.hovered(Rect::new(r.x + tw, y0 + *i as f32 * rh, r.w - tw, rh)))
}

/// Token size and step that fit `n` tokens in `w`.
fn fit_tokens(n: usize, w: f32) -> (f32, f32) {
    let size = ((w + 8.0) / n.max(1) as f32 - 8.0).clamp(16.0, 40.0);
    (size, size + 8.0)
}

#[derive(Clone, Copy, PartialEq)]
enum Tok {
    /// Already counted against a limit (a loss clock's pieces).
    Used,
    Spent(usize),
    Over(usize),
    /// Next turn's spend; true past the edge.
    Ghost(bool),
    Free,
}

/// A budget of tokens: used (clocks), this turn's spends in order (red past
/// the edge), next turn's ghosts, then what's left.
fn budget_tokens(ui: &mut Ui, area: Rect, s: &PublicSnapshot, b: &Budget, acc: Rgba, hover: Option<usize>) {
    let clock = b.key == "clock";
    let used = if clock { (b.size - b.left).max(0) as usize } else { 0 };
    let budget = b.left.max(0) as usize;
    let spends: Vec<usize> = b.items.iter().map(|i| i.amount.max(0) as usize).collect();
    let spend: usize = spends.iter().sum();
    let next = b.next.unwrap_or(0).max(0) as usize;
    let n = (used + budget).max(used + spend + next).min(48);
    let (t, step) = fit_tokens(n, area.w);
    let y = area.y + 22.0;
    let edge = used + budget;
    let icon = token_icon(b);
    let pulse = ui.pulse(1.6);

    // Assign each token a state.
    let mut toks = vec![Tok::Free; n];
    for tok in toks.iter_mut().take(used.min(n)) {
        *tok = Tok::Used;
    }
    let mut k = used;
    for (item, n_spend) in spends.iter().enumerate() {
        for _ in 0..*n_spend {
            if k < n {
                toks[k] = if k >= edge { Tok::Over(item) } else { Tok::Spent(item) };
            }
            k += 1;
        }
    }
    for _ in 0..next {
        if k < n {
            toks[k] = Tok::Ghost(k >= edge);
        }
        k += 1;
    }

    for (i, tok) in toks.iter().enumerate() {
        let r = Rect::new(area.x + i as f32 * step, y, t, t);
        let lit = matches!(tok, Tok::Spent(j) | Tok::Over(j) if hover == Some(*j));
        let (bg, border, ink) = match tok {
            // Pieces already on the board: solid.
            Tok::Used => (acc.alpha(60), acc.alpha(200), acc),
            Tok::Spent(_) => (SPEND.alpha(55), SPEND, SPEND),
            Tok::Over(_) => (SHORT.alpha((40.0 + 50.0 * pulse) as u8), SHORT, SHORT),
            Tok::Ghost(false) => (theme::TRANSPARENT, acc.alpha(70), acc.alpha(70)),
            Tok::Ghost(true) => (theme::TRANSPARENT, SHORT.alpha(80), SHORT.alpha(80)),
            // A clock's free tokens are room, not pieces; a card's are Blight on it.
            Tok::Free if clock => (theme::TRANSPARENT, theme::BORDER, acc.alpha(45)),
            Tok::Free => (theme::RAISED, theme::BORDER_STRONG, acc),
        };
        if matches!(tok, Tok::Over(_)) {
            ui.glow(r, SHORT.alpha((50.0 + 70.0 * pulse) as u8), 8.0, 10.0);
        }
        if lit {
            ui.glow(r, SPEND.alpha(140), 8.0, 12.0);
        }
        ui.rect(r, bg, border, 1.5, 8.0);
        match icon {
            Some(id) => ui.icon(r.shrink(t * 0.16), id, ink),
            None => ui.circle((r.x + t / 2.0, r.y + t / 2.0), t * 0.22, ink, theme::TRANSPARENT, 0.0),
        }
    }

    // The edge: where the budget runs out.
    if edge <= n {
        let ex = area.x + edge as f32 * step - 4.0;
        let over = spend + used > edge;
        let ec = if over { SHORT } else { theme::MUTED };
        ui.line((ex, y - 14.0), (ex, y + t + 6.0), 2.0, ec, 0.0);
        let word = if b.key == "blight" { "card empty" } else { "Invaders win" };
        edge_label(ui, area, ex, y, word, ec);
    }

    // Brackets under each spend's tokens, named by land.
    let mut k = used;
    for (item, n_spend) in spends.iter().enumerate() {
        if *n_spend == 0 || k >= n {
            k += n_spend;
            continue;
        }
        let x0 = area.x + k as f32 * step;
        let last = (k + n_spend - 1).min(n - 1);
        let x1 = area.x + last as f32 * step + t;
        let by = y + t + 6.0;
        let c = if k + n_spend > edge { SHORT } else { SPEND };
        ui.line((x0 + 2.0, by), (x1 - 2.0, by), 2.0, c.alpha(170), 0.0);
        let it = &b.items[item];
        let name = it.land.as_deref().map(|l| fmt::short(s, l)).unwrap_or_else(|| it.step.clone());
        ui.text_in(Rect::new(x0 - 20.0, by + 3.0, x1 - x0 + 40.0, 16.0), &name, theme::SMALL.bold().color(c), Align::Center);
        k += n_spend;
    }
    // Next turn's ghosts, bracketed and labeled like this turn's spends.
    if next > 0 && used + spend < n {
        let x0 = area.x + (used + spend) as f32 * step;
        let x1 = area.x + (used + spend + next).min(n) as f32 * step - 8.0;
        let by = y + t + 6.0;
        let c = acc.alpha(150);
        ui.line((x0 + 2.0, by), (x1 - 2.0, by), 1.5, c.alpha(90), 0.0);
        let label = format!("next turn −{next}");
        let lw = ui.measure(&label, theme::SMALL).0;
        ui.text_in(Rect::new(x0 - 40.0, by + 3.0, (x1 - x0 + 80.0).max(lw), 16.0), &label, theme::SMALL.color(c), Align::Center);
    }
}

/// A label on an edge line: after it, over what falls past it; before it
/// when that's the end of the row.
fn edge_label(ui: &mut Ui, area: Rect, ex: f32, y: f32, word: &str, color: Rgba) {
    let ww = ui.measure(word, theme::SMALL).0;
    let wx = if ex + 6.0 + ww <= area.right() { ex + 6.0 } else { ex - ww - 6.0 };
    ui.text(wx, y - 20.0, word, theme::SMALL.color(color));
}

/// Fear: the current card's pips (generated, then this turn's best-case
/// gains). The deck by Terror Level and the distances are the Pace's.
fn fear_tokens(ui: &mut Ui, area: Rect, stats: &CompanionStats, b: &Budget, acc: Rgba, hover: Option<usize>) {
    let f = &stats.fear;
    let per = f.required_per_card.max(1) as usize;
    let have = f.generated.max(0) as usize;
    let _ = &f.cards_left_by_level;
    let gains: Vec<usize> = b.items.iter().map(|i| i.amount.max(0) as usize).collect();
    let (t, step) = fit_tokens(per.max(6), (area.w - 110.0).max(120.0));
    let y = area.y + 22.0;
    ui.text(area.x, y - 18.0, "THIS CARD", theme::SMALL.color(theme::MUTED));
    let mut k = have;
    let mut owner: Vec<Option<usize>> = vec![None; per];
    for (j, g) in gains.iter().enumerate() {
        for _ in 0..*g {
            if k < per {
                owner[k] = Some(j);
            }
            k += 1;
        }
    }
    for (i, o) in owner.iter().enumerate() {
        let r = Rect::new(area.x + i as f32 * step, y, t, t);
        let (bg, border, ink) = if i < have {
            (acc.alpha(70), acc, theme::TEXT)
        } else if let Some(j) = o {
            if hover == Some(*j) {
                ui.glow(r, SPEND.alpha(140), 8.0, 12.0);
            }
            (SPEND.alpha(45), SPEND, SPEND)
        } else {
            (theme::RAISED, theme::BORDER_STRONG, acc.alpha(90))
        };
        ui.rect(r, bg, border, 1.5, t / 2.0);
        ui.icon(r.shrink(t * 0.18), icons::FEAR, ink);
    }
    if k >= per {
        let ex = area.x + per as f32 * step;
        ui.text(ex + 4.0, y + t / 2.0 - 8.0, "→ card earned", theme::SMALL.bold().color(theme::GREEN));
    }
}

/// The Invader deck as card backs by stage, next card first, each with the
/// turn it Explores and the lands a card of its stage names on this island
/// (Rei's step: the curve the Spirits' economy races).
fn deck_tokens(ui: &mut Ui, area: Rect, s: &PublicSnapshot, stats: &CompanionStats, b: &Budget) {
    let stages = &s.invaders.upcoming_stages;
    let y = area.y + 22.0;
    let advanced = matches!(s.step.as_str(), "AdvanceInvaderCards" | "SlowPower" | "TimePasses");
    let first = s.turn + u32::from(advanced);
    let (t, step) = fit_tokens(stages.len().max(8), area.w);
    let ch = t * 1.25;
    let spending = !b.items.is_empty();
    for (i, st) in stages.iter().enumerate() {
        let r = Rect::new(area.x + i as f32 * step, y - (ch - t) / 2.0, t * 0.8, ch);
        let c = match st {
            1 => TEAL,
            2 => Rgba::hex(0xf6ad55),
            _ => Rgba::hex(0xfc8181),
        };
        let this_turn = spending && i == 0;
        if this_turn {
            ui.glow(r, SPEND.alpha(120), 6.0, 10.0);
        }
        ui.rect(r, c.alpha(if this_turn { 120 } else { 55 }), if this_turn { SPEND } else { c }, 1.5, 5.0);
        ui.text_in(r, fmt::roman(*st as i64), theme::SMALL.bold().color(theme::TEXT), Align::Center);
        let turn = first + i as u32;
        let tstyle = if this_turn { theme::SMALL.bold().color(SPEND) } else { theme::SMALL.color(theme::MUTED) };
        ui.text_in(Rect::new(r.x - 8.0, r.y - 17.0, r.w + 16.0, 14.0), &format!("T{turn}"), tstyle, Align::Center);
        let reach = stats.curve.stage_reach[(*st as usize).clamp(1, 3) - 1];
        ui.text_in(Rect::new(r.x - 8.0, r.bottom() + 3.0, r.w + 16.0, 14.0), &format!("{reach:.0}"), theme::SMALL.color(c), Align::Center);
    }
    let ex = area.x + stages.len() as f32 * step - 2.0;
    ui.line((ex, y - 14.0), (ex, y + t + 6.0), 2.0, theme::MUTED, 0.0);
    let out = first + stages.len() as u32;
    ui.text(ex + 6.0, y + t / 2.0 - 16.0, "deck empty:", theme::SMALL.color(theme::MUTED));
    ui.text(ex + 6.0, y + t / 2.0, &format!("T{out}'s Explore loses"), theme::SMALL.color(theme::MUTED));
}

/// How an inventory row is drawn.
#[derive(Clone, Copy)]
struct Row {
    hot: bool,
    goal: bool,
    narrow: bool,
    /// The what-if prevents it.
    prevented: bool,
    /// The player is trying its fix.
    tried: bool,
}

/// One spend, with its step, land, amount, label and fix. Returns true when
/// the fix was clicked (to try it, or stop trying it).
fn inventory_row(ui: &mut Ui, r: Rect, s: &PublicSnapshot, it: &Spend, now: usize, row: Row) -> bool {
    let Row { hot, goal, narrow, prevented, tried } = row;
    if hot {
        ui.fill(r.inset(-6.0, 1.0, -6.0, 1.0), theme::RAISED, 5.0);
    }
    // The step: still ahead (lit), now, or already past (dim).
    let at = match it.step.as_str() {
        "Fear" => 3,
        "High Immigration" | "Ravage" | "Invader Phase" => 4,
        "Build" => 5,
        "Explore" => 6,
        _ => 4,
    };
    let (sc, sw) = if at > now {
        (theme::TEXT_2, "ahead")
    } else if at == now {
        (SPEND, "now")
    } else {
        (theme::FAINT, "done")
    };
    let mut x = r.x;
    let step_label = format!("{} · {sw}", it.step);
    let chip_w = (ui.measure(&step_label, theme::SMALL).0 + 22.0).max(96.0);
    let chip = Rect::new(x, r.y + 3.0, chip_w, 20.0);
    ui.rect(chip, theme::TRANSPARENT, sc.alpha(150), 1.0, 10.0);
    ui.text_in(chip, &step_label, theme::SMALL.color(sc), Align::Center);
    x += chip.w + 10.0;
    if let Some(l) = &it.land {
        x += fmt::land_chip(ui, x, r.y + 3.0, s, l).max(118.0) + 10.0;
    }
    let sign = if goal { "+" } else { "−" };
    let amt = if prevented { format!("{sign}0") } else { format!("{sign}{}", it.amount) };
    let amt_color = if prevented { theme::GREEN } else if goal { theme::GREEN } else { SPEND };
    let (aw, _) = ui.text(x, r.y + 4.0, &amt, TextStyle { size: 14.0, line: 18.0, mono: true, weight: 700, color: amt_color });
    x += aw.max(28.0) + 10.0;
    let label = if it.certain { fmt::humanize(s, &it.label) } else { format!("{} (best case)", fmt::humanize(s, &it.label)) };
    let solve = it.solve.as_ref().map(|t| if tried { format!("✓ trying: Defend {}", it.defend.unwrap_or(0)) } else { t.clone() });
    let solve_w = solve.as_ref().map(|t| ui.measure(t, theme::SMALL.bold()).0 + 22.0).unwrap_or(0.0);
    // Narrow: the label gets its own line under the chips.
    let (lx, ly, room) = if narrow { (r.x, r.y + 25.0, r.w - solve_w - 12.0) } else { (x, r.y + 5.0, r.right() - x - solve_w - 12.0) };
    let tone = if prevented { theme::MUTED } else { theme::TEXT_2 };
    let label = fmt::fit(ui, &label, theme::SMALL.color(tone), room.max(40.0));
    let (lw, _) = ui.text(lx, ly, &label, theme::SMALL.color(tone));
    if prevented {
        ui.line((lx, ly + 8.0), (lx + lw, ly + 8.0), 1.0, theme::MUTED, 0.0);
    }
    let Some(solve) = solve else { return false };
    let pill = Rect::new(r.right() - solve_w, r.y + 3.0, solve_w, 20.0);
    let over = ui.hovered(pill) && it.defend.is_some();
    let (fill, border) = match (tried, over) {
        (true, _) => (theme::GREEN.alpha(90), theme::GREEN),
        (false, true) => (theme::GREEN.alpha(55), theme::GREEN),
        _ => (theme::GREEN.alpha(26), theme::GREEN.alpha(170)),
    };
    ui.rect(pill, fill, border, 1.0, 10.0);
    let text = if over && !tried { format!("try it: Defend {}", it.defend.unwrap_or(0)) } else { solve };
    ui.text_in(pill, &text, theme::SMALL.bold().color(if tried { theme::TEXT } else { theme::GREEN }), Align::Center);
    it.defend.is_some() && ui.clicked(pill)
}

/// One spend as a chip: step mark, land, amount, fix. Hover shows the full
/// row; clicking tries the fix (or stops trying it).
fn spend_chip(ui: &mut Ui, r: Rect, s: &PublicSnapshot, it: &Spend, now: usize, row: Row) -> bool {
    let Row { hot, prevented, tried, .. } = row;
    let at = match it.step.as_str() {
        "Fear" => 3,
        "Build" => 5,
        "Explore" => 6,
        _ => 4,
    };
    let mark = if at > now { theme::TEXT_2 } else if at == now { SPEND } else { theme::FAINT };
    ui.rect(r, if hot { theme::RAISED } else { theme::INSET }, if tried { theme::GREEN } else if hot { theme::BORDER_STRONG } else { theme::BORDER }, 1.0, 6.0);
    ui.fill(Rect::new(r.x + 4.0, r.y + 5.0, 3.0, r.h - 10.0), mark, 1.5);
    let land = it.land.as_deref().map(|l| fmt::short(s, l)).unwrap_or_else(|| it.step.clone());
    let terrain = it.land.as_deref().and_then(|l| fmt::land(s, l)).map(|l| fmt::terrain(&l.terrain)).unwrap_or(theme::TEXT_2);
    let (lw, _) = ui.text(r.x + 14.0, r.y + 4.0, &land, theme::SMALL.bold().color(if prevented { theme::MUTED } else { terrain }));
    let amt = if prevented { "−0".to_string() } else { format!("−{}", it.amount) };
    ui.text(r.x + 22.0 + lw, r.y + 4.0, &amt, theme::SMALL.bold().mono().color(if prevented { theme::GREEN } else { SPEND }));
    if let Some(d) = it.defend {
        let label = if tried { format!("✓ Def {d}") } else { format!("Def {d}") };
        let pw = ui.measure(&label, theme::SMALL.bold()).0 + 14.0;
        let pill = Rect::new(r.right() - pw - 4.0, r.y + 3.0, pw, r.h - 6.0);
        ui.rect(pill, theme::GREEN.alpha(if tried { 90 } else { 22 }), theme::GREEN.alpha(170), 1.0, 9.0);
        ui.text_in(pill, &label, theme::SMALL.bold().color(if tried { theme::TEXT } else { theme::GREEN }), Align::Center);
    }
    if hot {
        // The full row, above the chip.
        let full = format!("{} · {} · {}{}", fmt::humanize(s, &it.label), it.step, it.solve.clone().unwrap_or_default(), if it.defend.is_some() { " · click to try" } else { "" });
        let w = ui.measure(&full, theme::SMALL).0 + 16.0;
        ui.overlay(true);
        let tip = Rect::new(r.x, r.y - 26.0, w, 22.0);
        ui.rect(tip, theme::PANEL_HEAD, theme::BORDER_STRONG, 1.0, 6.0);
        ui.text_in(tip, &full, theme::SMALL.color(theme::TEXT), Align::Center);
        ui.overlay(false);
    }
    it.defend.is_some() && ui.clicked(r)
}
