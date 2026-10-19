//! Slow phase and Time Passes: what sets up next turn? Next turn's Invader
//! Phase played forward from this turn's forecast, in the same layout as the
//! Invaders view.

use super::phase;
use si_common::companion_stats::CompanionStats;
use si_common::public::PublicSnapshot;
use si_ui::theme;
use si_ui::widgets::panel;
use si_ui::{Rect, Ui};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats) {
    // Once this turn's Invader Phase is over, the forecast already is next
    // turn's; before that, it's played forward.
    let after = matches!(s.step.as_str(), "AdvanceInvaderCards" | "SlowPower" | "TimePasses");
    let f = if after { Some(&stats.invader_phase) } else { stats.next_phase.as_ref() };
    let turn = f.map(|f| f.turn).unwrap_or(s.turn + 1);
    let c = panel(ui, r, &format!("Next turn · Turn {turn}"), Some(if after { "from the board now" } else { "played forward" }));
    let off = ui.scroll_begin("slow", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = phase::callouts(ui, x, y, w, s, stats);
    y = phase::question(ui, x, y, w, "What sets up next turn?");
    let basis = if after {
        "Next turn's cards are in place. Defend, Damage and this turn's Fear, Event and Power effects end at Time Passes."
    } else {
        "Played forward from this turn's forecast: no Spirit action assumed, and this turn's Explorers counted only once the Explore card is face up."
    };
    let (_, h) = ui.text_wrap(x, y, w, basis, theme::SMALL.color(theme::MUTED));
    y += h + 8.0;
    match f {
        Some(f) => y = phase::invader_body(ui, x, y, w, s, f),
        None => {
            ui.text(x, y, "No next turn to show.", theme::BODY.color(theme::MUTED));
            y += 26.0;
        }
    }
    ui.scroll_end("slow", c, y + off - c.y);
}
