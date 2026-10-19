//! Left column: the clocks. How far is each way the game can end, and what
//! does the spirit have to work with this turn?

use super::fmt;
use si_common::companion_stats::CompanionStats;
use si_common::public::PublicSnapshot;
use si_ui::theme;
use si_ui::widgets::{self, panel};
use si_ui::{Align, Rect, Ui, span};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let mut col = r;
    let next_line = stats.blight.next_turn.as_ref().is_some_and(|n| n.adds_min > 0);
    let blight = col.cut_top(if next_line { 188.0 } else { 172.0 });
    col.cut_top(theme::GAP);
    let fear = col.cut_top(148.0);
    col.cut_top(theme::GAP);
    let deck = col.cut_top(96.0 + 36.0 * stats.loss_clocks.len() as f32);
    col.cut_top(theme::GAP);
    blight_panel(ui, blight, s, stats);
    fear_panel(ui, fear, stats);
    deck_panel(ui, deck, s, stats);
    spirit_panel(ui, col, s, stats);
}

fn blight_panel(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let b = &stats.blight;
    let side = if b.flipped { "blighted side" } else { "healthy side" };
    let c = panel(ui, r, "Blight", Some(side));
    ui.push_clip(c);
    let warn = b.remaining <= 2;
    let big = theme::NUM_BIG.color(if warn { theme::AMBER } else { theme::TEXT });
    let (w, h) = ui.text(c.x, c.y, &b.remaining.to_string(), big);
    ui.text(c.x + w + 8.0, c.y + 3.0, "left on the card", theme::SMALL);
    ui.text(c.x + w + 8.0, c.y + 17.0, &format!("{} on the island", b.on_island), theme::SMALL.color(theme::MUTED));
    if b.flipped {
        let title = s.blight.blighted_card.as_ref().map(|c| c.title.as_str()).unwrap_or("Blighted");
        let tw = ui.measure(title, theme::SMALL).0;
        widgets::chip(ui, c.right() - tw - 14.0, c.y + 4.0, title, theme::TEXT, theme::RED.alpha(90));
    }
    let mut y = c.y + h + 8.0;
    // After this turn's Invader Phase, the forecast is next turn's.
    let after = matches!(s.step.as_str(), "AdvanceInvaderCards" | "SlowPower" | "TimePasses");
    if b.adds_this_phase_min > 0 {
        let which = if after { "Next Invader Phase" } else { "This Invader Phase" };
        let line = format!("{which}: at least +{}, so the {}", b.adds_this_phase_min, b.outcome_this_phase);
        let (_, lh) = ui.text_wrap(c.x, y, c.w, &line, theme::BODY.color(theme::AMBER));
        y += lh + 4.0;
    } else {
        ui.text(c.x, y, "No Blight forecast this Invader Phase", theme::BODY.color(theme::TEXT_2));
        y += 22.0;
    }
    if let Some(n) = b.next_turn.as_ref().filter(|n| n.adds_min > 0) {
        let (color, verdict) = match n.outcome.as_str() {
            o if o.contains("(loss)") => (theme::RED, "loss"),
            "card holds" => (theme::TEXT_2, "holds"),
            "card flips" => (theme::AMBER, "flips"),
            o if o.contains("new Blight Card") => (theme::AMBER, "new card"),
            _ => (theme::AMBER, "on the new side"),
        };
        let left = n.remaining_after_this.map(|l| format!(" vs {l} left")).unwrap_or_default();
        let line = format!("Next turn's Ravage: +{}{left} → {verdict}", n.adds_min);
        ui.text(c.x, y, &line, theme::BODY.color(color));
        y += 20.0;
    }
    if b.loss_risk_now > 0.0 {
        let line = format!("Chance of losing {}: {:.0}%", if after { "next phase" } else { "this phase" }, b.loss_risk_now * 100.0);
        ui.text(c.x, y, &line, theme::BODY.bold().color(theme::RED));
        y += 20.0;
    }
    if let Some(chain) = b.cascade_chains.first() {
        let line = format!("Cascade risk: {} (+{} more at least)", fmt::short(s, &chain.land), chain.min_extra);
        ui.text(c.x, y, &line, theme::SMALL);
    }
    ui.pop_clip();
}

fn fear_panel(ui: &mut Ui, r: Rect, stats: &CompanionStats) {
    let f = &stats.fear;
    let c = panel(ui, r, "Fear", Some(&format!("{} to win", fmt::plural(f.to_clear_deck, "fear", "fear"))));
    ui.push_clip(c);
    let tl = format!("Terror Level {}", fmt::roman(f.terror_level as i64));
    ui.text(c.x, c.y, &tl, theme::TITLE);
    let left: Vec<String> = f.cards_left_by_level.iter().enumerate().map(|(i, n)| format!("{} {n}", fmt::roman(i as i64 + 1))).collect();
    ui.text_in(Rect::new(c.x, c.y, c.w, 20.0), &left.join("  "), theme::SMALL.mono(), Align::Right);
    let y = c.y + 30.0;
    let w = widgets::pips(ui, c.x, y + 4.0, f.generated, f.required_per_card, theme::ACCENT);
    ui.text(c.x + w + 8.0, y, &format!("{} of {} toward the next card", f.generated, f.required_per_card), theme::SMALL);
    let mut y = y + 24.0;
    if f.earned_banked > 0 {
        let line = format!("{} earned: resolves in the Fear step", fmt::plural(f.earned_banked as i64, "card", "cards"));
        ui.text(c.x, y, &line, theme::BODY.color(theme::ACCENT));
        y += 20.0;
    }
    let next = match f.to_next_level {
        Some(n) => format!("Next Terror Level in {n} fear"),
        None => "At the last Terror Level".into(),
    };
    ui.text(c.x, y, &next, theme::SMALL);
    y += 18.0;
    let vic = format!("Victory now: {} ({} left)", f.victory_condition, f.victory_blockers);
    let vic = fmt::fit(ui, &vic, theme::SMALL.color(theme::MUTED), c.w);
    ui.text(c.x, y, &vic, theme::SMALL.color(theme::MUTED));
    ui.pop_clip();
}

fn deck_panel(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let up = &s.invaders.upcoming_stages;
    let c = panel(ui, r, "Invader deck · adversary", Some(&format!("{} cards", up.len())));
    ui.push_clip(c);
    let mut x = c.x;
    for (i, st) in up.iter().enumerate() {
        let label = fmt::roman(*st as i64);
        let cell = Rect::new(x, c.y, 24.0, 20.0);
        let next = i == 0;
        ui.rect(cell, if next { theme::ACCENT_DIM } else { theme::RAISED }, if next { theme::ACCENT } else { theme::BORDER }, 1.0, 3.0);
        ui.text_in(cell, label, theme::SMALL.mono().color(if next { theme::TEXT } else { theme::TEXT_2 }), Align::Center);
        x += 27.0;
    }
    if up.is_empty() {
        ui.text(c.x, c.y, "Deck empty: the next Explore loses the game", theme::BODY.color(theme::RED));
    }
    let mut y = c.y + 28.0;
    for a in s.adversaries.iter().take(1) {
        let title = format!("{} · Level {}", a.title, a.level);
        ui.text(c.x, y, &title, theme::BODY.bold());
        y += 19.0;
    }
    // Each adversary's own loss condition, counted against its limit.
    for clock in &stats.loss_clocks {
        let value = clock.after.unwrap_or(clock.now).max(clock.now);
        let left = if clock.trigger == "over" { clock.limit - value + 1 } else { clock.limit - value };
        let color = if left <= 0 {
            theme::RED
        } else if left <= 1 {
            theme::AMBER
        } else {
            theme::TEXT_2
        };
        ui.text(c.x, y, &clock.title, theme::SMALL.bold().color(color));
        let line = fmt::humanize(s, &clock.line);
        let (_, h) = ui.text_wrap(c.x, y + 15.0, c.w, &line, theme::SMALL.color(color));
        y += 15.0 + h + 4.0;
    }
    ui.pop_clip();
}

fn spirit_panel(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    match stats.spirits.as_slice() {
        [] => {}
        [sp] => {
            let raw = s.spirits.iter().find(|x| x.id == sp.id);
            let c = panel(ui, r, &sp.title, raw.and_then(|x| x.aspect.as_deref()));
            spirit_block(ui, c, sp, false);
        }
        many => {
            let c = panel(ui, r, "Spirits", Some(&many.len().to_string()));
            let rows = c.rows(many.len(), 6.0);
            for (sp, row) in many.iter().zip(rows) {
                spirit_block(ui, row, sp, true);
            }
        }
    }
}

/// One spirit: Energy and Card Plays, elements, innate levels reached.
/// `named`: several spirits share the panel, so each block has its title.
fn spirit_block(ui: &mut Ui, c: Rect, sp: &si_common::companion_stats::spirits::SpiritStats, named: bool) {
    if named {
        return spirit_line(ui, c, sp);
    }
    ui.push_clip(c);
    let mut y = c.y;
    let big = theme::NUM_BIG.size(if named { 17.0 } else { 22.0 });
    let (w, h) = ui.text(c.x, y, &sp.energy.to_string(), big.color(theme::AMBER));
    let per = sp.energy_per_turn.map(|e| format!("Energy  +{e}/turn")).unwrap_or_else(|| "Energy".into());
    ui.text(c.x + w + 6.0, y + h / 2.0 - 8.0, &per, theme::SMALL);
    let x2 = c.x + c.w / 2.0;
    let (w, _) = ui.text(x2, y, &sp.card_plays.to_string(), big.color(theme::ACCENT));
    ui.text(x2 + w + 6.0, y + h / 2.0 - 8.0, "Card Plays", theme::SMALL);
    y += h + 6.0;

    // Elements this turn.
    let mut x = c.x;
    for (el, n) in sp.elements.iter().filter(|(_, n)| **n > 0) {
        let label = format!("{el} {n}");
        let w = widgets::chip(ui, x, y, &label, theme::TEXT, theme::RAISED);
        x += w + 4.0;
        if x > c.right() - 60.0 {
            x = c.x;
            y += 22.0;
        }
    }
    if sp.elements.values().all(|n| *n <= 0) {
        ui.text(c.x, y, "No elements yet this turn", theme::SMALL.color(theme::MUTED));
    }
    y += if named { 22.0 } else { 28.0 };

    // Innates: which levels the elements reach now.
    for inn in &sp.innates {
        let met = inn.levels.iter().filter(|l| l.met_now).count();
        let mut xx = c.x;
        for l in &inn.levels {
            let d = Rect::new(xx, y + 5.0, 8.0, 8.0);
            if l.met_now {
                ui.fill(d, theme::ACCENT, 4.0);
            } else {
                ui.rect(d, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, 4.0);
            }
            xx += 11.0;
        }
        let detail = format!("  {} · {met}/{} levels", inn.speed, inn.levels.len());
        let spans = [
            span(&inn.title, theme::BODY.color(if met > 0 { theme::TEXT } else { theme::TEXT_2 })),
            span(&detail, theme::SMALL.color(theme::MUTED)),
        ];
        ui.spans(xx + 4.0, y, &spans, None);
        y += 21.0;
    }
    let hand = format!("Hand: {} Fast · {} Slow", sp.hand_fast.len(), sp.hand_slow.len());
    ui.text(c.x, y + 2.0, &hand, theme::SMALL);
    ui.pop_clip();
}

/// A spirit in three lines, for several spirits in one panel: name with
/// Energy and Card Plays, elements, innate levels reached.
fn spirit_line(ui: &mut Ui, c: Rect, sp: &si_common::companion_stats::spirits::SpiritStats) {
    ui.push_clip(c);
    let nums = format!("{} E (+{}) · {} plays", sp.energy, sp.energy_per_turn.unwrap_or(0), sp.card_plays);
    let nw = ui.measure(&nums, theme::SMALL.mono()).0;
    let title = fmt::fit(ui, &sp.title, theme::BODY.bold(), c.w - nw - 8.0);
    ui.text(c.x, c.y, &title, theme::BODY.bold());
    ui.text_in(Rect::new(c.x, c.y, c.w, 18.0), &nums, theme::SMALL.mono().color(theme::AMBER), Align::Right);
    let els: Vec<String> = sp.elements.iter().filter(|(_, n)| **n > 0).map(|(e, n)| format!("{e} {n}")).collect();
    let els = if els.is_empty() { "No elements yet".to_string() } else { els.join(" · ") };
    let line = fmt::fit(ui, &els, theme::SMALL, c.w);
    ui.text(c.x, c.y + 20.0, &line, theme::SMALL);
    let mut x = c.x;
    for inn in &sp.innates {
        for l in &inn.levels {
            let d = Rect::new(x, c.y + 42.0, 7.0, 7.0);
            if l.met_now {
                ui.fill(d, theme::ACCENT, 3.5);
            } else {
                ui.rect(d, theme::TRANSPARENT, theme::BORDER_STRONG, 1.0, 3.5);
            }
            x += 9.0;
        }
        let name = inn.title.split_whitespace().take(2).collect::<Vec<_>>().join(" ");
        let (w, _) = ui.text(x + 3.0, c.y + 37.0, &name, theme::SMALL.color(theme::TEXT_2));
        x += w + 14.0;
    }
    ui.pop_clip();
}
