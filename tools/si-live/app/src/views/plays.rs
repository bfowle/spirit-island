//! Bottom: the play-by-play. Your decisions (from the game log) beside what
//! changed on the board, newest last. Undone actions are already gone: the
//! timeline drops snapshots after an undo, and the decision parser cuts at
//! the undo checkpoint.

use super::fmt;
use si_common::public::PublicSnapshot;
use si_steam::companion::CompanionState;
use si_ui::theme::{self, Rgba};
use si_ui::widgets::panel;
use si_ui::{Rect, Ui, span};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, st: &CompanionState) {
    let cols = r.columns(2, theme::GAP);
    let decisions: Vec<(String, String)> =
        st.decisions.iter().map(|d| (format!("T{} {}", d.turn, step_short(&d.step)), fmt::plain(&fmt::humanize(s, &d.text)))).collect();
    let changes: Vec<(String, String)> =
        st.changes.iter().map(|c| (format!("T{} {}", c.turn, step_short(&c.step)), fmt::humanize(s, &c.text))).collect();
    list(ui, cols[0], "Your decisions", &decisions, theme::ACCENT);
    list(ui, cols[1], "On the board", &changes, theme::AMBER);
}

fn step_short(step: &str) -> &str {
    match step {
        "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower" => "Spirit",
        "FastPower" => "Fast",
        "SlowPower" => "Slow",
        "BlightedIsland" => "Blighted",
        "HighImmigration" => "High Imm.",
        "AdvanceInvaderCards" => "Advance",
        "TimePasses" => "Time",
        "PlayerSetup" | "InvadersStartingAction" => "Setup",
        other => other,
    }
}

/// The newest lines that fit, newest at the bottom.
fn list(ui: &mut Ui, r: Rect, title: &str, rows: &[(String, String)], mark: Rgba) {
    let c = panel(ui, r, title, Some(&rows.len().to_string()));
    ui.push_clip(c);
    let line = 19.0;
    let fit = (c.h / line).floor().max(1.0) as usize;
    let shown = &rows[rows.len().saturating_sub(fit)..];
    let mut y = c.bottom() - shown.len() as f32 * line;
    for (i, (when, text)) in shown.iter().enumerate() {
        let newest = i + 1 == shown.len();
        if newest {
            ui.fill(Rect::new(c.x - 6.0, y + 3.0, 2.0, 12.0), mark, 1.0);
        }
        let when_style = theme::SMALL.mono().color(theme::MUTED);
        ui.text(c.x, y, when, when_style);
        let style = theme::BODY.color(if newest { theme::TEXT } else { theme::TEXT_2 });
        let t = fmt::fit(ui, text, style, c.w - 92.0);
        ui.spans(c.x + 92.0, y, &[span(&t, style)], None);
        y += line;
    }
    if rows.is_empty() {
        ui.text(c.x, c.y, "Nothing yet", theme::SMALL.color(theme::MUTED));
    }
    ui.pop_clip();
}
