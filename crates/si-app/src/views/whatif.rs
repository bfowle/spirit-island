//! The What-If drawer (W): try changes on the current moment, the way a
//! player thinks about a turn: lands and their pieces, Defend, Dahan, and
//! the next presence. Each click adds a change; every number on the page
//! recomputes on a copy of the snapshot. Nothing is sent to the game.

use super::{Both, Toggle, fmt};
use crate::icons;
use si_common::companion_stats::whatif::{self, Edit};
use si_common::public::{PublicLand, PublicSnapshot};
use si_ui::theme::{self, Rgba};
use si_ui::{Align, Rect, Ui};

const ROW: f32 = 30.0;

/// The colored edge of a land's row: what the Invaders do there.
const TIERS: [(Rgba, &str); 3] = [(theme::RED, "Ravages now"), (theme::AMBER, "Builds now"), (Rgba::hex(0xb794f6), "Ravages next turn")];

/// The drawer, over `at` (the Fear lane on the Ledger page).
pub fn drawer(ui: &mut Ui, at: Rect, s: &PublicSnapshot, both: &Both, out: &mut Vec<Toggle>) {
    let r = Rect::new(at.x - 6.0, at.y - 4.0, at.w + 12.0, at.h + 8.0);
    ui.overlay(true);
    ui.glow(r, Rgba(0, 0, 0, 170), 12.0, 24.0);
    ui.rect(r, theme::PANEL_HEAD, theme::AMBER.alpha(170), 1.0, 12.0);
    let inner = r.inset(20.0, 14.0, 20.0, 12.0);
    let (w, _) = ui.text(inner.x, inner.y, "WHAT IF", theme::HEAD.bold().color(theme::AMBER));
    let (w2, _) = ui.text(inner.x + w + 12.0, inner.y, "Click a piece to destroy it, add Defend or Dahan; every number recomputes. Nothing is sent to the game.", theme::SMALL.color(theme::MUTED));
    let mut kx = inner.x + w + 12.0 + w2 + 24.0;
    for (c, label) in TIERS {
        ui.fill(Rect::new(kx, inner.y + 3.0, 4.0, 12.0), c, 1.0);
        let (lw, _) = ui.text(kx + 8.0, inner.y, label, theme::SMALL.color(theme::MUTED));
        kx += lw + 22.0;
    }
    // Clear and close, top right.
    let close = Rect::new(inner.right() - 70.0, inner.y - 3.0, 70.0, 20.0);
    if button(ui, close, "close  W", theme::MUTED) {
        out.push(Toggle::CloseDrawer);
    }
    if !both.edits.is_empty() {
        let clear = Rect::new(close.x - 90.0, close.y, 82.0, 20.0);
        if button(ui, clear, "clear all", theme::AMBER) {
            out.push(Toggle::Clear);
        }
    }

    // The board after the changes so far.
    let tried = whatif::apply(s, both.edits);
    let phase = &both.base.invader_phase;
    let next = both.base.next_phase.as_ref();
    let clock_lands: Vec<&str> = both.base.loss_clocks.iter().filter_map(|c| c.at.as_deref()).collect();
    let rank = |l: &PublicLand| -> (u8, i64) {
        let ravage = phase.ravages.iter().chain(&phase.build_ravages).any(|r| !r.done && r.land == l.id);
        let build = phase.builds.iter().any(|b| !b.done && b.land == l.id);
        let later = next.is_some_and(|n| n.ravages.iter().any(|r| r.land == l.id));
        let tier = if ravage || clock_lands.contains(&l.id.as_str()) {
            0
        } else if build {
            1
        } else if later {
            2
        } else {
            3
        };
        (tier, -((l.explorers + 2 * l.towns + 3 * l.cities) as i64))
    };
    let mut lands: Vec<&PublicLand> = s
        .lands
        .iter()
        .filter(|l| !l.is_ocean && (l.explorers + l.towns + l.cities > 0 || both.edits.iter().any(|e| e.land() == Some(l.id.as_str()))))
        .collect();
    lands.sort_by_key(|l| rank(l));

    // Lands on the left, the spirits' growth in a column on the right.
    let side_w = 300.0;
    let grid = Rect::new(inner.x, inner.y + 30.0, inner.w - side_w - 24.0, inner.h - 30.0);
    let cols = 3usize;
    let col_w = (grid.w - 24.0 * (cols as f32 - 1.0)) / cols as f32;
    let rows = (grid.h / ROW).floor().max(1.0) as usize;
    for (k, l) in lands.iter().take(rows * cols).enumerate() {
        let (c, row) = (k / rows, k % rows);
        let rr = Rect::new(grid.x + c as f32 * (col_w + 24.0), grid.y + row as f32 * ROW, col_w, ROW - 4.0);
        let now = tried.lands.iter().find(|x| x.id == l.id).unwrap_or(l);
        land_row(ui, rr, s, now, rank(l).0, both.edits, out);
    }
    if lands.len() > rows * cols {
        ui.text_in(Rect::new(grid.x, grid.bottom() - 2.0, grid.w, 14.0), &format!("+{} more lands with Invaders", lands.len() - rows * cols), theme::SMALL.color(theme::FAINT), Align::Right);
    }

    // Growth: where the next presence comes from (the Curve page shows it).
    let side = Rect::new(inner.right() - side_w, grid.y, side_w, grid.h);
    ui.fill(Rect::new(side.x - 12.0, side.y, 1.0, side.h), theme::BORDER, 0.0);
    ui.text(side.x, side.y, "NEXT PRESENCE FROM", theme::SMALL.color(theme::MUTED));
    let mut y = side.y + 20.0;
    for sp in s.spirits.iter().take(3) {
        let name = fmt::fit(ui, &sp.title, theme::SMALL.bold().color(theme::TEXT_2), side.w);
        ui.text(side.x, y, &name, theme::SMALL.bold().color(theme::TEXT_2));
        let mut x = side.x;
        for (track, label) in [("Energy", "Energy track"), ("CardPlays", "Card Plays track")] {
            let e = Edit::Track { spirit: sp.id.clone(), track: track.into() };
            let on = both.edits.contains(&e);
            let b = Rect::new(x, y + 18.0, 140.0, 22.0);
            if toggle(ui, b, label, on) {
                out.push(if on { Toggle::Remove(e) } else { Toggle::Add(e) });
            }
            x += 150.0;
        }
        y += 48.0;
    }
    ui.overlay(false);
}

/// One land: its pieces as buttons (click destroys one), Defend and Dahan,
/// and an undo for the changes made here.
fn land_row(ui: &mut Ui, r: Rect, s: &PublicSnapshot, l: &PublicLand, tier: u8, edits: &[Edit], out: &mut Vec<Toggle>) {
    let hot = ui.hovered(r.inset(-4.0, 0.0, -4.0, 0.0));
    if hot {
        ui.fill(r.inset(-6.0, -1.0, -6.0, -1.0), theme::RAISED, 6.0);
    }
    if let Some((c, _)) = TIERS.get(tier as usize) {
        ui.fill(Rect::new(r.x - 8.0, r.y + 4.0, 3.0, r.h - 8.0), *c, 1.5);
    }
    let mut x = r.x;
    x += fmt::land_chip(ui, x, r.y + 3.0, s, &l.id).max(112.0) + 8.0;
    for (kind, n, icon) in [("Explorer", l.explorers, icons::EXPLORER), ("Town", l.towns, icons::TOWN), ("City", l.cities, icons::CITY)] {
        let b = Rect::new(x, r.y + 2.0, 42.0, 22.0);
        let live = n > 0;
        let over = live && ui.hovered(b);
        ui.rect(b, if over { theme::RED.alpha(50) } else { theme::INSET }, if over { theme::RED } else { theme::BORDER_STRONG }, 1.0, 6.0);
        ui.icon(Rect::new(b.x + 4.0, b.y + 3.0, 16.0, 16.0), icon, if live { Rgba::hex(0xd9c2a5) } else { theme::FAINT });
        let label = if over { "−1".to_string() } else { n.to_string() };
        ui.text_in(Rect::new(b.x + 20.0, b.y, 20.0, b.h), &label, theme::SMALL.bold().color(if over { theme::RED } else if live { theme::TEXT } else { theme::FAINT }), Align::Center);
        if live && ui.clicked(b) {
            out.push(Toggle::Add(Edit::Destroy { land: l.id.clone(), kind: kind.into() }));
        }
        x += 46.0;
    }
    x += 4.0;
    // Defend: one more each click.
    let defend = edits.iter().find_map(|e| match e {
        Edit::Defend { land, amount } if *land == l.id => Some(*amount),
        _ => None,
    });
    let db = Rect::new(x, r.y + 2.0, 62.0, 22.0);
    let label = defend.map(|d| format!("Def {d} +")).unwrap_or("+Defend".into());
    if button(ui, db, &label, theme::GREEN) {
        if let Some(d) = defend {
            out.push(Toggle::Remove(Edit::Defend { land: l.id.clone(), amount: d }));
        }
        out.push(Toggle::Add(Edit::Defend { land: l.id.clone(), amount: defend.unwrap_or(0) + 1 }));
    }
    x += 66.0;
    let dah = Rect::new(x, r.y + 2.0, 46.0, 22.0);
    ui.rect(dah, theme::INSET, theme::BORDER_STRONG, 1.0, 6.0);
    ui.icon(Rect::new(dah.x + 4.0, dah.y + 3.0, 16.0, 16.0), icons::DAHAN, Rgba::hex(0x9fd3a8));
    ui.text_in(Rect::new(dah.x + 20.0, dah.y, 24.0, dah.h), &format!("{}+", l.dahan), theme::SMALL.bold().color(theme::TEXT_2), Align::Center);
    if ui.clicked(dah) {
        out.push(Toggle::Add(Edit::Dahan { land: l.id.clone() }));
    }
    x += 52.0;
    // What's been tried here, newest last; click to undo the newest.
    let here: Vec<&Edit> = edits.iter().filter(|e| e.land() == Some(l.id.as_str())).collect();
    if let Some(last) = here.last() {
        let ub = Rect::new(r.right() - 24.0, r.y + 2.0, 22.0, 22.0);
        if button(ui, ub, "↺", theme::AMBER) {
            out.push(Toggle::Remove((*last).clone()));
        }
        let n = here.iter().filter(|e| matches!(e, Edit::Destroy { .. })).count();
        if n > 0 {
            ui.text_in(Rect::new(x, r.y + 2.0, ub.x - x - 4.0, 22.0), &format!("−{n}"), theme::SMALL.bold().color(theme::AMBER), Align::Right);
        }
    }
}

/// A small outlined button; true when clicked.
fn button(ui: &mut Ui, r: Rect, label: &str, color: Rgba) -> bool {
    let over = ui.hovered(r);
    ui.rect(r, if over { color.alpha(45) } else { theme::INSET }, color.alpha(if over { 230 } else { 140 }), 1.0, 6.0);
    ui.text_in(r, label, theme::SMALL.bold().color(color), Align::Center);
    ui.clicked(r)
}

fn toggle(ui: &mut Ui, r: Rect, label: &str, on: bool) -> bool {
    let over = ui.hovered(r);
    let c = theme::ACCENT;
    ui.rect(r, if on { c.alpha(110) } else if over { c.alpha(40) } else { theme::INSET }, c.alpha(if on || over { 230 } else { 120 }), 1.0, 11.0);
    ui.text_in(r, label, theme::SMALL.bold().color(if on { theme::TEXT } else { theme::TEXT_2 }), Align::Center);
    ui.clicked(r)
}
