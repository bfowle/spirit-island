//! Center: the coming Invader Phase in the order it resolves (High
//! Immigration, Ravage, Build, Escalation, Explore). Each row says what
//! happens in a land and why; the player looks here for what is still
//! changeable before it resolves.

use super::fmt;
use si_common::companion_stats::CompanionStats;
use si_common::companion_stats::invaders::{BuildForecast, InvaderPhaseForecast, RavageForecast};
use si_common::public::{PublicInvaderCard, PublicSnapshot};
use si_ui::theme::{self, Rgba};
use si_ui::widgets::{self, panel};
use si_ui::{Rect, Ui, span};

const ROW: f32 = 26.0;

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let f = &stats.invader_phase;
    let when = match f.position.as_str() {
        "before" => "coming up",
        "ravage" => "Ravaging now",
        "build" => "Building now",
        "explore" => "Exploring now",
        other => other,
    };
    let c = panel(ui, r, &format!("Invader Phase · Turn {}", f.turn), Some(when));
    let off = ui.scroll_begin("phase", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = callouts(ui, x, y, w, s, stats);
    y = question(ui, x, y, w, "What will the Invaders do, and what can still change it?");
    y = invader_body(ui, x, y, w, s, f);
    ui.scroll_end("phase", c, y + off - c.y);
}

/// Loud warnings decided by what happens this turn: next turn's Ravage when
/// it empties or flips the Blight card (Brett, Prussia + Blitz game: Turn 4's
/// four-land Ravage wasn't visible on Turn 3), and an adversary loss clock
/// within one step of its limit (Habsburg game: Mountain NE7 reached 8 on
/// Turn 3, lost at the end of Turn 4's Fast Phase).
pub(super) fn callouts(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, stats: &CompanionStats) -> f32 {
    let mut y = y;
    let callout = |ui: &mut Ui, y: &mut f32, color: Rgba, head: &str, body: &str| {
        let (_, bh) = ui.measure_spans(&[span(body, theme::BODY)], Some(w - 24.0));
        let r = Rect::new(x, *y, w, 30.0 + bh);
        ui.rect(r, color.alpha(28), color.alpha(150), 1.0, theme::RADIUS_SM);
        ui.text(x + 12.0, *y + 7.0, head, theme::BODY.bold().color(color));
        ui.text_wrap(x + 12.0, *y + 26.0, w - 24.0, body, theme::BODY);
        *y += r.h + 10.0;
    };
    if let Some(n) = stats.blight.next_turn.as_ref().filter(|n| n.adds_min > 0 && n.outcome != "card holds") {
        let color = if n.outcome.contains("(loss)") { theme::RED } else { theme::AMBER };
        let names: Vec<String> = n.lands.iter().map(|id| fmt::short(s, id)).collect();
        let left = n.remaining_after_this.map(|l| format!(" vs {l} left")).unwrap_or_default();
        // Next turn's Ravage cards are this turn's Build cards.
        let cards: Vec<String> = stats.invader_phase.build_cards.iter().filter(|c| n.cards.contains(&c.id)).map(fmt::card_label).collect();
        let cards = if cards.is_empty() { n.cards.join(", ") } else { cards.join(", ") };
        let head = format!("NEXT TURN: {cards} Ravages {}", names.join(", "));
        let body = format!(
            "At least +{} Blight{left}, so the {}. Which of these lands can you thin or protect this turn, before they Build and Explore?",
            n.adds_min, n.outcome
        );
        callout(ui, &mut y, color, &head, &body);
    }
    for clock in &stats.loss_clocks {
        let value = clock.after.unwrap_or(clock.now).max(clock.now);
        let left = if clock.trigger == "over" { clock.limit - value + 1 } else { clock.limit - value };
        if left > 1 {
            continue;
        }
        let color = if left <= 0 { theme::RED } else { theme::AMBER };
        let head = format!("{}: {}", clock.title.to_uppercase(), if left <= 0 { "the Invaders win when it's checked" } else { "one step from the limit" });
        let checked = if clock.checked.is_empty() { String::new() } else { format!(", checked {}", clock.checked) };
        let body = format!("{}{checked}. What can you change before it's checked?", fmt::humanize(s, &clock.line));
        callout(ui, &mut y, color, &head, &body);
    }
    y
}

/// The question a view answers, as its first line.
pub(super) fn question(ui: &mut Ui, x: f32, y: f32, w: f32, text: &str) -> f32 {
    let q = Rect::new(x, y, w, 26.0);
    ui.fill(q, theme::INSET, theme::RADIUS_SM);
    ui.fill(Rect::new(q.x, q.y, 3.0, q.h), theme::ACCENT, 1.5);
    ui.text_in(q.inset(12.0, 0.0, 0.0, 0.0), text, theme::BODY.color(theme::TEXT_2), si_ui::Align::Left);
    y + 38.0
}

/// One Invader Phase in the order it resolves.
pub(super) fn invader_body(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, f: &InvaderPhaseForecast) -> f32 {
    let mut y = y;
    if !f.high_immigration_cards.is_empty() {
        y = heading(ui, x, y, w, "High Immigration · before the Ravage", &f.high_immigration_cards);
        y = builds(ui, x, y, w, s, &f.high_immigration, "No land builds");
    }

    y = heading(ui, x, y, w, "Ravage", &f.ravage_cards);
    if f.ravages.is_empty() {
        y = empty(ui, x, y, if f.ravage_cards.is_empty() { "No Ravage card" } else { "No matching land has Invaders" });
    }
    for rv in &f.ravages {
        y = ravage_row(ui, x, y, w, s, rv);
    }
    y = rule_actions(ui, x, y, w, s, f, "after the Ravage");

    y = heading(ui, x, y, w, "Build", &f.build_cards);
    y = builds(ui, x, y, w, s, &f.builds, if f.build_cards.is_empty() { "No Build card" } else { "No land builds" });
    for rv in &f.build_ravages {
        y = ravage_row(ui, x, y, w, s, rv);
    }
    y = rule_actions(ui, x, y, w, s, f, "after the Build");

    if let Some(e) = &f.escalation {
        let title = format!("Escalation · {} · {} of {} possible cards", e.title, e.chance.0, e.chance.1);
        y = heading(ui, x, y, w, &title, &[]);
        let mut text = fmt::plain(&e.text);
        if !e.when.is_empty() {
            text = format!("{} ({}{})", text.trim_end_matches('.'), e.when, if e.pick.is_empty() { String::new() } else { format!("; {}", e.pick) });
        }
        let (_, h) = ui.text_wrap(x, y, w, &text, theme::SMALL);
        y += h + 6.0;
        // A target whose Build is stopped (Disease, a cancel) is a note.
        let stopped: Vec<&String> = f.notes.iter().filter(|n| n.contains(&format!("no {} Build", e.title))).collect();
        if e.builds.is_empty() && !stopped.is_empty() {
            for n in stopped {
                let (_, h) = ui.text_wrap(x, y, w, &fmt::plain(&fmt::humanize(s, n)), theme::BODY.color(theme::TEXT_2));
                y += h + 4.0;
            }
        } else {
            // One row per land; several possible cards become one note.
            let mut rows: Vec<BuildForecast> = Vec::new();
            for b in &e.builds {
                match rows.iter_mut().find(|r| r.land == b.land && r.adds == b.adds) {
                    Some(r) => {
                        if let Some(c) = &b.card {
                            r.notes.push(format!("or {c}"));
                        }
                    }
                    None => rows.push(b.clone()),
                }
            }
            let none = if e.each_land { "No explored land would have Dahan to replace" } else { "No land qualifies" };
            y = builds(ui, x, y, w, s, &rows, none);
        }
    }

    y = explore(ui, x, y, w, s, f);

    let other: Vec<&String> = f
        .notes
        .iter()
        .filter(|n| !n.starts_with("Proud & Mighty Capital"))
        .filter(|n| f.escalation.as_ref().is_none_or(|e| !n.contains(&format!("no {} Build", e.title))))
        .collect();
    if !other.is_empty() {
        y = heading(ui, x, y, w, "Notes", &[]);
        for n in other {
            let (_, h) = ui.text_wrap(x, y, w, &fmt::plain(&fmt::humanize(s, n)), theme::SMALL);
            y += h + 4.0;
        }
    }
    y
}

/// Adversary actions at one point between the printed steps.
fn rule_actions(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, f: &InvaderPhaseForecast, when: &str) -> f32 {
    let mut y = y;
    for a in f.rule_actions.iter().filter(|a| a.when == when) {
        let title = format!("{} · {}", a.rule, when);
        y = heading(ui, x, y, w, &title, &[]);
        let text = if a.pick.is_empty() { a.text.clone() } else { format!("{} ({})", a.text, a.pick) };
        let (_, h) = ui.text_wrap(x, y, w, &text, theme::SMALL);
        y += h + 6.0;
        y = builds(ui, x, y, w, s, &a.lands, "No land qualifies");
    }
    y
}

pub(super) fn heading(ui: &mut Ui, x: f32, y: f32, w: f32, title: &str, cards: &[PublicInvaderCard]) -> f32 {
    let y = y + 4.0;
    let (tw, _) = ui.text(x, y + 3.0, &title.to_uppercase(), theme::HEAD.color(theme::TEXT));
    let mut cx = x + tw + 12.0;
    for c in cards {
        cx += fmt::card_chip(ui, cx, y, c) + 6.0;
    }
    ui.fill(Rect::new(x, y + 28.0, w, 1.0), theme::BORDER, 0.0);
    y + 36.0
}

pub(super) fn empty(ui: &mut Ui, x: f32, y: f32, text: &str) -> f32 {
    ui.text(x, y, text, theme::BODY.color(theme::MUTED));
    y + ROW
}

pub(super) fn ravage_row(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, rv: &RavageForecast) -> f32 {
    let l = &rv.ledger;
    let cw = fmt::land_chip(ui, x, y, s, &rv.land);
    let mut cx = x + cw.max(110.0) + 10.0;

    // Outcome first: it is the answer.
    let times = if l.extra_blight > 0 { format!(" ×{}", 1 + l.extra_blight) } else { String::new() };
    let (label, fg, bg): (String, Rgba, Rgba) = if l.cancelled.is_some() {
        ("NO RAVAGE".into(), theme::TEXT, theme::BORDER_STRONG)
    } else if l.adds_blight && l.cascades {
        (format!("BLIGHT{times} + CASCADE"), theme::TEXT, theme::RED.alpha(150))
    } else if l.adds_blight {
        (format!("BLIGHT{times}"), theme::TEXT, theme::RED.alpha(110))
    } else {
        ("NO BLIGHT".into(), theme::BG, theme::GREEN)
    };
    cx += widgets::chip(ui, cx, y + 1.0, &label, fg, bg) + 10.0;

    let math = match &l.cancelled {
        Some(src) => format!("skipped: {src}"),
        None => {
            let mut m = format!("{} damage", l.invader_damage);
            if l.defend > 0 {
                m += &format!(" − {} Defend", l.defend);
            }
            m + &format!(" = {} vs land {}", l.damage_after_defend, l.land_health)
        }
    };
    let style = if l.cancelled.is_some() { theme::SMALL } else { theme::NUM };
    let mw = ui.text_wrap(cx, y + 3.0, (x + w - cx).max(80.0), &fmt::humanize(s, &math), style).0;
    cx += mw + 14.0;
    if rv.done {
        widgets::tag(ui, cx, y + 1.0, "resolved", theme::MUTED);
    } else if let Some(skip) = &l.skip_available {
        let t = fmt::fit(ui, &format!("can skip: {skip}"), theme::SMALL, (x + w - cx - 10.0).max(0.0));
        widgets::tag(ui, cx, y + 1.0, &t, theme::AMBER);
    }

    // Second line: the Dahan, and what the ledger applied or couldn't model.
    let mut parts = Vec::new();
    if l.dahan_before > 0 {
        let mut d = format!("{} Dahan: {} destroyed", l.dahan_before, l.dahan_destroyed);
        if l.counterattack_damage > 0 {
            d += &format!(", counterattack {} damage", l.counterattack_damage);
            if l.counterattack_clears {
                d += " clears the land";
            } else if !l.counterattack_can_destroy.is_empty() {
                d += &format!(" (best: {})", l.counterattack_can_destroy.join(", "));
            }
        }
        parts.push(d);
    }
    for (spirit, n) in &l.presence_at_risk {
        if *n > 0 {
            let name = s.spirits.iter().find(|x| x.id == *spirit).map(|x| x.title.as_str()).unwrap_or(spirit);
            parts.push(format!("{name} loses {} if Blight lands", fmt::plural(*n as i64, "Presence", "Presence")));
        }
    }
    parts.extend(l.applied.iter().cloned());
    let line2 = parts.join(" · ");
    let mut h = ROW;
    if !line2.is_empty() {
        let (_, lh) = ui.text_wrap(x + 120.0, y + ROW - 2.0, w - 120.0, &fmt::humanize(s, &line2), theme::SMALL);
        h += lh;
    }
    if !l.unmodeled.is_empty() {
        let t = format!("Not modeled: {}", l.unmodeled.join("; "));
        let (_, lh) = ui.text_wrap(x + 120.0, y + h - 2.0, w - 120.0, &t, theme::SMALL.color(theme::AMBER));
        h += lh;
    }
    y + h + 8.0
}

pub(super) fn builds(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, list: &[BuildForecast], none: &str) -> f32 {
    if list.is_empty() {
        return empty(ui, x, y, none);
    }
    let mut y = y;
    for b in list {
        let cw = fmt::land_chip(ui, x, y, s, &b.land);
        let mut cx = x + cw.max(110.0) + 10.0;
        let adds = format!("+{}", b.adds);
        cx += widgets::chip(ui, cx, y + 1.0, &adds, theme::BG, theme::AMBER) + 10.0;
        if b.done {
            cx += widgets::tag(ui, cx, y + 1.0, "built", theme::MUTED) + 8.0;
        }
        let notes = fmt::humanize(s, &b.notes.join(" · "));
        let mut h = ROW;
        if !notes.is_empty() {
            let (_, nh) = ui.text_wrap(cx, y + 3.0, (x + w - cx).max(60.0), &notes, theme::SMALL);
            h = h.max(nh + 8.0);
        }
        y += h;
    }
    y + 4.0
}

fn explore(ui: &mut Ui, x: f32, y: f32, w: f32, s: &PublicSnapshot, f: &InvaderPhaseForecast) -> f32 {
    let e = &f.explore;
    let title = match (&e.revealed, e.next_stage) {
        (Some(_), _) => "Explore · card face up".to_string(),
        (None, Some(st)) => format!("Explore · next card is Stage {} ({} possible)", fmt::roman(st as i64), e.candidates.len()),
        (None, None) => "Explore · the Invader deck is empty".to_string(),
    };
    let mut y = heading(ui, x, y, w, &title, &[]);
    if e.deck_empty {
        ui.text(x, y, "The spirits lose at this Explore.", theme::BODY.bold().color(theme::RED));
        return y + ROW;
    }
    for cand in &e.candidates {
        let cw = fmt::card_chip(ui, x, y, &cand.card);
        let mut cx = x + cw.max(130.0) + 10.0;
        if cand.lands.is_empty() {
            ui.text(cx, y + 3.0, "explores nowhere", theme::SMALL.color(theme::MUTED));
        }
        for l in &cand.lands {
            let lw = fmt::land_chip(ui, cx, y + 1.0, s, l);
            cx += lw + 5.0;
            if cx > x + w - 90.0 {
                cx = x + cw.max(130.0) + 10.0;
                y += 24.0;
            }
        }
        if !cand.blocked.is_empty() {
            let b: Vec<String> = cand.blocked.iter().map(|id| fmt::humanize(s, id)).collect();
            let t = format!("blocked: {}", b.join(", "));
            let t = fmt::fit(ui, &t, theme::SMALL.color(theme::MUTED), (x + w - cx).max(0.0));
            ui.spans(cx + 4.0, y + 4.0, &[span(&t, theme::SMALL.color(theme::MUTED))], None);
        }
        y += 28.0;
        for n in &cand.notes {
            let indent = 140.0;
            let (_, h) = ui.text_wrap(x + indent, y - 4.0, w - indent, &fmt::humanize(s, n), theme::SMALL.color(theme::AMBER));
            y += h + 2.0;
        }
    }
    y + 4.0
}
