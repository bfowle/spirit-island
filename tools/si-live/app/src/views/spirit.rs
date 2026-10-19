//! Spirit and Fast phases: what can still change before this turn's Ravage?
//! Each land taking Blight with how far it is from safe and what in hand can
//! do it; lands one Damage from Blight; then this turn's Builds and Explores
//! in brief (the Invaders tab has them in full).

use super::{fmt, phase};
use si_common::companion_stats::CompanionStats;
use si_common::companion_stats::invaders::RavageForecast;
use si_common::public::PublicSnapshot;
use si_ui::theme;
use si_ui::widgets::{self, panel};
use si_ui::{Rect, Ui};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, fast: bool) {
    let f = &stats.invader_phase;
    let title = if fast { format!("Fast Phase · Turn {}", s.turn) } else { format!("Spirit Phase · Turn {}", s.turn) };
    let c = panel(ui, r, &title, Some("before the Ravage"));
    let off = ui.scroll_begin("spirit", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = phase::callouts(ui, x, y, w, s, stats);
    y = phase::question(ui, x, y, w, "What can you still change before the Ravage?");

    // Lands taking Blight, with the margin to save each.
    let threatened: Vec<&RavageForecast> = f.ravages.iter().chain(&f.build_ravages).filter(|r| !r.done && r.ledger.adds_blight).collect();
    y = phase::heading(ui, x, y, w, &format!("Taking Blight this turn · {}", threatened.len()), &f.ravage_cards);
    if threatened.is_empty() {
        y = phase::empty(ui, x, y, "No land takes Blight from this turn's Ravage as things stand");
    }
    for rv in threatened {
        let l = &rv.ledger;
        let margin = l.damage_after_defend - (l.land_health - 1);
        let cw = fmt::land_chip(ui, x, y, s, &rv.land);
        let mut cx = x + cw.max(110.0) + 10.0;
        let times = if l.extra_blight > 0 { format!(" ×{}", 1 + l.extra_blight) } else { String::new() };
        let label = if l.cascades { format!("BLIGHT{times} + CASCADE") } else { format!("BLIGHT{times}") };
        cx += widgets::chip(ui, cx, y + 1.0, &label, theme::TEXT, theme::RED.alpha(120)) + 10.0;
        let save = format!("Defend {margin}, or remove {margin} Damage of Invaders, and it takes no Blight");
        ui.text_wrap(cx, y + 2.0, (x + w - cx).max(80.0), &save, theme::BODY.bold());
        y += 26.0;
        // What each spirit holds that can do it.
        for need in &stats.needs {
            let Some(sl) = need.save_land.iter().find(|n| n.land == rv.land) else { continue };
            let who = if stats.needs.len() > 1 { format!("{}: ", need.spirit) } else { String::new() };
            let hand: Vec<String> = sl
                .in_hand
                .iter()
                .map(|h| if fast && h.contains("(Slow,") && !h.contains("(Fast,") { format!("{h} — Slow, too late for this Ravage") } else { h.clone() })
                .collect();
            let line = if hand.is_empty() { format!("{who}nothing in hand can") } else { format!("{who}in hand: {}", hand.join(" · ")) };
            let (_, h) = ui.text_wrap(x + 120.0, y, w - 120.0, &line, theme::SMALL.color(if hand.is_empty() { theme::MUTED } else { theme::TEXT }));
            y += h + 2.0;
            if !fast && sl.minor.pool > 0 {
                let draw = format!(
                    "{who}if you gain a Power: {} of {} unseen Minors could ({:.0}% in a draw of 4), {} of {} Majors",
                    sl.minor.effect,
                    sl.minor.pool,
                    sl.minor.p_effect * 100.0,
                    sl.major.effect,
                    sl.major.pool
                );
                let (_, h) = ui.text_wrap(x + 120.0, y, w - 120.0, &draw, theme::SMALL.color(theme::TEXT_2));
                y += h + 2.0;
            }
        }
        y += 8.0;
    }

    // One Damage from Blight: a single extra Explorer (an Event, a Fear Card,
    // an adversary rule) tips these.
    let close: Vec<&RavageForecast> = f
        .ravages
        .iter()
        .filter(|r| !r.done && !r.ledger.adds_blight && r.ledger.cancelled.is_none() && r.ledger.invader_damage > 0 && r.ledger.damage_after_defend + 1 >= r.ledger.land_health)
        .collect();
    if !close.is_empty() {
        y = phase::heading(ui, x, y, w, "One Damage from Blight", &[]);
        for rv in close {
            let cw = fmt::land_chip(ui, x, y, s, &rv.land);
            let t = format!("{} Damage vs land {}: one more Explorer, or 1 less Defend, Blights it", rv.ledger.damage_after_defend, rv.ledger.land_health);
            ui.text(x + cw.max(110.0) + 10.0, y + 3.0, &t, theme::SMALL);
            y += 26.0;
        }
    }

    // This turn's Builds and Explores, in brief.
    y = phase::heading(ui, x, y, w, "Then, this turn", &[]);
    let builds: Vec<String> = f.high_immigration.iter().chain(&f.builds).map(|b| format!("{} +{}", fmt::short(s, &b.land), b.adds)).collect();
    let line = if builds.is_empty() { "Build: none".to_string() } else { format!("Build: {}", builds.join(", ")) };
    let (_, h) = ui.text_wrap(x, y, w, &line, theme::BODY);
    y += h + 4.0;
    let e = &f.explore;
    let explores: Vec<String> = e.per_land.iter().filter(|(_, (n, _))| *n > 0).map(|(id, (n, d))| if e.revealed.is_some() { fmt::short(s, id) } else { format!("{} {n}/{d}", fmt::short(s, id)) }).collect();
    let line = if explores.is_empty() { "Explore: none".to_string() } else { format!("Explore: {}", explores.join(", ")) };
    let (_, h) = ui.text_wrap(x, y, w, &line, theme::BODY);
    y += h + 4.0;
    if let Some(esc) = &f.escalation {
        let t = format!("Escalation possible ({} of {} cards): {}", esc.chance.0, esc.chance.1, esc.title);
        let (_, h) = ui.text_wrap(x, y, w, &t, theme::BODY.color(theme::AMBER));
        y += h + 4.0;
    }
    ui.scroll_end("spirit", c, y + off - c.y);
}
