//! Rules in play: what is different about this game? Each adversary level,
//! Escalation and loss condition, scenario and spirit rule, in plain words.
//! For an adversary you haven't played (Brett, Habsburg Mining game: "not
//! familiar with Habs Mining at all").

use super::fmt;
use si_common::companion_stats::CompanionStats;
use si_common::public::PublicSnapshot;
use si_ui::theme;
use si_ui::widgets::{self, panel};
use si_ui::{Rect, Ui};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let c = panel(ui, r, "Rules in play", Some("R or the Rules button closes"));
    let off = ui.scroll_begin("rules", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = super::phase::question(ui, x, y, w, "What is different about this game?");

    // Loss conditions first: they end the game on their own clock.
    if !stats.loss_clocks.is_empty() {
        y = super::phase::heading(ui, x, y, w, "Ways to lose besides Blight, Fear and the Invader deck", &[]);
        for c in &stats.loss_clocks {
            let tw = ui.text(x, y, &c.title, theme::BODY.bold()).0;
            widgets::tag(ui, x + tw + 10.0, y, &c.adversary, theme::AMBER);
            y += 22.0;
            let mut text = fmt::plain(&c.note);
            if !c.checked.is_empty() && !text.contains("checked") {
                text += &format!(" Checked {}.", c.checked);
            }
            let (_, h) = ui.text_wrap(x + 12.0, y, w - 12.0, &text, theme::BODY);
            y += h + 4.0;
            let (_, h) = ui.text_wrap(x + 12.0, y, w - 12.0, &format!("Now: {}", fmt::humanize(s, &c.line)), theme::SMALL.color(theme::TEXT_2));
            y += h + 10.0;
        }
    }

    // Each source's rules, once each (the register lists a rule under every
    // part of the game it changes).
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut current = String::new();
    for rule in &stats.rules_in_play {
        if rule.kind == "loss_condition" || seen.contains(&(rule.from.clone(), rule.title.clone())) {
            continue;
        }
        seen.push((rule.from.clone(), rule.title.clone()));
        let group = rule.from.split(" L").next().unwrap_or(&rule.from).to_string();
        if group != current {
            y = super::phase::heading(ui, x, y, w, &group, &[]);
            current = group;
        }
        let level = rule.from.strip_prefix(&current).map(str::trim).unwrap_or("");
        let title = if level.is_empty() { rule.title.clone() } else { format!("{level} · {}", rule.title) };
        ui.text(x, y, &title, theme::BODY.bold());
        y += 21.0;
        let text = fmt::plain(&rule.text);
        let (_, h) = ui.text_wrap(x + 12.0, y, w - 12.0, &text, theme::BODY.color(theme::TEXT_2));
        y += h + 10.0;
    }
    ui.scroll_end("rules", c, y + off - c.y);
}
