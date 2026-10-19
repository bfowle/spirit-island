//! Fear: what will the banked cards do? Banked cards are face down, so the
//! view says when they resolve and at which Terror Level, shows a card as it
//! resolves, and keeps the resolved ones with the text that applied.

use super::{fmt, phase};
use si_common::companion_stats::CompanionStats;
use si_common::public::{PublicFearCard, PublicSnapshot};
use si_ui::theme;
use si_ui::widgets::{self, panel};
use si_ui::{Rect, Ui};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    let fs = &stats.fear;
    let c = panel(ui, r, &format!("Fear · Terror Level {}", fmt::roman(fs.terror_level as i64)), Some(&format!("{} banked", fs.earned_banked)));
    let off = ui.scroll_begin("fear", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = phase::callouts(ui, x, y, w, s, stats);
    y = phase::question(ui, x, y, w, "What will the banked Fear Cards do?");

    let banked = if fs.earned_banked == 0 {
        format!("No Fear Card banked. {} more Fear earns the next one.", fs.to_next_card)
    } else {
        format!(
            "{} banked, face down: {} in this turn's Fear step, at Terror Level {} (the level when they resolve).",
            fmt::plural(fs.earned_banked as i64, "card", "cards"),
            if fs.earned_banked == 1 { "it resolves" } else { "they resolve" },
            fmt::roman(fs.terror_level as i64)
        )
    };
    let (_, h) = ui.text_wrap(x, y, w, &banked, theme::BODY);
    y += h + 12.0;

    if let Some(card) = &s.fear.resolving {
        y = phase::heading(ui, x, y, w, "Resolving now", &[]);
        y = card_block(ui, x, y, w, card, fs.terror_level as i64, true);
    }
    if !s.fear.discard.is_empty() {
        y = phase::heading(ui, x, y, w, &format!("Resolved this game · {}", s.fear.discard.len()), &[]);
        for card in s.fear.discard.iter().rev() {
            let tl = card.resolved_at_terror_level.unwrap_or(fs.terror_level as i64);
            y = card_block(ui, x, y, w, card, tl, false);
        }
    }
    ui.scroll_end("fear", c, y + off - c.y);
}

/// A Fear Card with the text for Terror Level `tl` (all three when resolving).
fn card_block(ui: &mut Ui, x: f32, y: f32, w: f32, card: &PublicFearCard, tl: i64, all: bool) -> f32 {
    let mut y = y;
    let tw = ui.text(x, y, &card.title, theme::BODY.bold()).0;
    widgets::tag(ui, x + tw + 10.0, y, &format!("Terror Level {}", fmt::roman(tl)), theme::ACCENT);
    y += 24.0;
    for (i, b) in card.body.iter().enumerate() {
        let level = i as i64 + 1;
        if !all && level != tl {
            continue;
        }
        let style = if level == tl { theme::BODY } else { theme::SMALL.color(theme::MUTED) };
        let text = format!("{}: {}", fmt::roman(level), fmt::plain(b));
        let (_, h) = ui.text_wrap(x + 12.0, y, w - 12.0, &text, style);
        y += h + 4.0;
    }
    y + 8.0
}
