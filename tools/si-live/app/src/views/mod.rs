//! The companion screen (native app spec §3): one window, no page scroll.
//!
//! ```text
//! top bar: brand · source · phase tabs (follow the game) · turn · replay
//! clocks (left) | phase view (center) | island + Invader cards (right)
//! play-by-play
//! status bar
//! ```

mod clocks;
mod event;
mod fear;
mod fmt;
mod island;
mod phase;
mod plays;
mod rules;
mod slow;
mod spirit;

use si_steam::companion::CompanionState;
use si_steam::source::ReplayCommand;
use si_ui::theme::{self, TextStyle};
use si_ui::widgets;
use si_ui::{Align, Key, Rect, Ui};

/// View state that outlives a frame.
#[derive(Debug, Default)]
pub struct ViewState {
    /// A phase tab the player clicked; `None` follows the game.
    pub pinned: Option<usize>,
    /// Time to build and submit the last frame.
    pub frame_ms: f32,
    /// GPU adapter, for the status bar.
    pub gpu: String,
    pub source_label: String,
    /// The Rules view replaces the phase view.
    pub rules: bool,
}

pub enum Action {
    Replay(ReplayCommand),
}

pub const PHASES: [&str; 7] = ["Spirit", "Fast", "Event", "Fear", "Invaders", "Slow", "Time Passes"];

/// Which phase tab a game step belongs to.
pub fn phase_of(step: &str) -> usize {
    match step {
        "FastPower" => 1,
        "BlightedIsland" | "Event" => 2,
        "Fear" => 3,
        "HighImmigration" | "Ravage" | "Build" | "Explore" | "AdvanceInvaderCards" => 4,
        "SlowPower" => 5,
        "TimePasses" => 6,
        _ => 0,
    }
}

pub fn draw(ui: &mut Ui, v: &mut ViewState, st: &CompanionState) -> Vec<Action> {
    let mut actions = Vec::new();
    keys(ui, v, st, &mut actions);

    let mut r = ui.screen;
    let top = r.cut_top(46.0);
    top_bar(ui, top, v, st, &mut actions);
    let status = r.cut_bottom(24.0);
    status_bar(ui, status, v, st);

    let mut body = r.inset(theme::GAP, theme::GAP, theme::GAP, theme::GAP);
    let Some(snap) = st.public.as_ref() else {
        waiting(ui, body, st);
        return actions;
    };
    let Some(stats) = st.stats.as_ref() else {
        waiting(ui, body, st);
        return actions;
    };

    let plays_h = (body.h * 0.2).clamp(120.0, 190.0);
    let plays = body.cut_bottom(plays_h);
    body.cut_bottom(theme::GAP);
    let left_w = (body.w * 0.2).clamp(270.0, 330.0);
    let left = body.cut_left(left_w);
    body.cut_left(theme::GAP);
    let right_w = (body.w * 0.36).clamp(420.0, 560.0);
    let right = body.cut_right(right_w);
    body.cut_right(theme::GAP);

    clocks::draw(ui, left, snap, stats);
    if v.rules {
        rules::draw(ui, body, snap, stats);
    } else {
        match v.pinned.unwrap_or_else(|| phase_of(&snap.step)) {
            0 => spirit::draw(ui, body, snap, stats, false),
            1 => spirit::draw(ui, body, snap, stats, true),
            2 => event::draw(ui, body, snap, stats, st),
            3 => fear::draw(ui, body, snap, stats),
            5 | 6 => slow::draw(ui, body, snap, stats),
            _ => phase::draw(ui, body, snap, stats),
        }
    }
    island::draw(ui, right, snap, stats);
    plays::draw(ui, plays, snap, st);
    actions
}

fn keys(ui: &mut Ui, v: &mut ViewState, st: &CompanionState, actions: &mut Vec<Action>) {
    if ui.key(Key::Char('r')) || ui.key(Key::Char('R')) {
        v.rules = !v.rules;
    }
    if ui.key(Key::Escape) {
        v.rules = false;
        v.pinned = None;
    }
    let Some(rep) = st.source.replay.as_ref() else { return };
    let step = if ui.input.shift { 10 } else { 1 };
    if ui.key(Key::Left) {
        actions.push(Action::Replay(ReplayCommand::Step(-step)));
    }
    if ui.key(Key::Right) {
        actions.push(Action::Replay(ReplayCommand::Step(step)));
    }
    if ui.key(Key::Home) {
        actions.push(Action::Replay(ReplayCommand::Seek(0)));
    }
    if ui.key(Key::End) {
        actions.push(Action::Replay(ReplayCommand::Seek(rep.total.saturating_sub(1))));
    }
    if ui.key(Key::Space) {
        actions.push(Action::Replay(if rep.playing { ReplayCommand::Pause } else { ReplayCommand::Play { speed: 8.0 } }));
    }
}

fn top_bar(ui: &mut Ui, r: Rect, v: &mut ViewState, st: &CompanionState, actions: &mut Vec<Action>) {
    ui.fill(r, theme::PANEL, 0.0);
    ui.fill(Rect::new(r.x, r.bottom() - 1.0, r.w, 1.0), theme::BORDER, 0.0);
    let mut row = r.inset(14.0, 0.0, 14.0, 0.0);

    let brand = TextStyle { size: 13.0, line: 18.0, mono: false, weight: 700, color: theme::TEXT };
    let w = ui.text_in(row, "SPIRIT ISLAND", brand, Align::Left);
    row.cut_left(w + 8.0);
    let w = ui.text_in(row, "companion", theme::BODY.color(theme::MUTED), Align::Left);
    row.cut_left(w + 16.0);

    let (mode, color) = match st.source.mode.as_str() {
        "live" => ("LIVE", theme::GREEN),
        "replay" => ("REPLAY", theme::ACCENT),
        _ => ("OFF", theme::MUTED),
    };
    let w = widgets::chip(ui, row.x, row.y + (row.h - 18.0) / 2.0, mode, theme::BG, color);
    row.cut_left(w + 20.0);

    // Phase tabs: the game's current phase is underlined; a click pins one.
    let current = st.public.as_ref().map(|s| phase_of(&s.step));
    for (i, name) in PHASES.iter().enumerate() {
        let style = TextStyle { size: 12.5, line: 16.0, mono: false, weight: 500, color: theme::TEXT };
        let tw = ui.measure(name, style).0 + 20.0;
        let tab = Rect::new(row.x, row.y + 9.0, tw, row.h - 18.0);
        let shown = v.pinned.or(current) == Some(i);
        if widgets::button(ui, tab, name, shown) {
            v.pinned = if v.pinned == Some(i) { None } else { Some(i) };
        }
        if current == Some(i) {
            ui.fill(Rect::new(tab.x + 6.0, r.bottom() - 3.0, tab.w - 12.0, 2.0), theme::ACCENT, 1.0);
        }
        row.cut_left(tw + 2.0);
    }
    if v.pinned.is_some() {
        let w = widgets::tag(ui, row.x + 6.0, row.y + (row.h - 18.0) / 2.0, "pinned · click to follow", theme::AMBER);
        row.cut_left(w + 12.0);
    }
    let rules = Rect::new(row.x + 8.0, row.y + 9.0, 64.0, row.h - 18.0);
    if widgets::button(ui, rules, "Rules", v.rules) {
        v.rules = !v.rules;
    }
    row.cut_left(80.0);

    // Right: replay controls and position.
    if let Some(rep) = st.source.replay.as_ref() {
        let pos = format!("{} / {}  {}", rep.index + 1, rep.total, rep.clock);
        let w = ui.text_in(row, &pos, theme::SMALL.mono(), Align::Right);
        let mut ctl = row;
        ctl.cut_right(w + 12.0);
        let buttons: [(&str, f32, ReplayCommand); 4] = [
            ("«", 30.0, ReplayCommand::Seek(0)),
            ("‹", 30.0, ReplayCommand::Step(-1)),
            (if rep.playing { "Pause" } else { "Play" }, 52.0, if rep.playing { ReplayCommand::Pause } else { ReplayCommand::Play { speed: 8.0 } }),
            ("›", 30.0, ReplayCommand::Step(1)),
        ];
        let mut x = ctl.right() - buttons.iter().map(|b| b.1 + 2.0).sum::<f32>();
        for (label, bw, cmd) in buttons {
            if widgets::button(ui, Rect::new(x, row.y + 9.0, bw, row.h - 18.0), label, false) {
                actions.push(Action::Replay(cmd));
            }
            x += bw + 2.0;
        }
    }
}

fn status_bar(ui: &mut Ui, r: Rect, v: &ViewState, st: &CompanionState) {
    ui.fill(r, theme::PANEL, 0.0);
    ui.fill(Rect::new(r.x, r.y, r.w, 1.0), theme::BORDER, 0.0);
    let row = r.inset(14.0, 0.0, 14.0, 0.0);
    let style = theme::SMALL.color(theme::MUTED);
    let left = match st.public.as_ref() {
        Some(s) => {
            let adv: Vec<String> = s.adversaries.iter().map(|a| format!("{} L{}", a.title, a.level)).collect();
            let spirits: Vec<&str> = s.spirits.iter().map(|x| x.title.as_str()).collect();
            let boards: Vec<&str> = s.boards.iter().map(|b| b.id.as_str()).collect();
            format!(
                "{} · {} · board {} · {}",
                if adv.is_empty() { "no adversary".to_string() } else { adv.join(" + ") },
                spirits.join(", "),
                boards.join("+"),
                s.game_id
            )
        }
        None => st.source.note.clone().unwrap_or_else(|| "no game".into()),
    };
    ui.text_in(row, &left, style, Align::Left);
    let mut right = vec![];
    if let Some(s) = st.public.as_ref() {
        right.push(format!("Turn {} · {}", s.turn, s.step));
        right.push(format!("engine {}", s.engine_build));
    }
    if st.source.decode_errors > 0 {
        right.push(format!("{} decode errors", st.source.decode_errors));
    }
    right.push(v.source_label.clone());
    if !v.gpu.is_empty() {
        right.push(v.gpu.clone());
    }
    right.push(format!("frame {:.1} ms", v.frame_ms));
    ui.text_in(row, &right.join("   ·   "), style, Align::Right);
}

fn waiting(ui: &mut Ui, r: Rect, st: &CompanionState) {
    let msg = match st.source.mode.as_str() {
        "live" => "Waiting for a Spirit Island game. Start or load one in Steam.",
        "replay" => "Decoding the capture…",
        _ => "The Steam source is off (engine DLL or save folder not found).",
    };
    let mid = Rect::new(r.x, r.y + r.h / 2.0 - 30.0, r.w, 24.0);
    ui.text_in(mid, msg, theme::TITLE.color(theme::TEXT_2), Align::Center);
    if let Some(note) = st.source.note.as_ref() {
        ui.text_in(Rect::new(r.x, mid.bottom() + 6.0, r.w, 20.0), note, theme::SMALL, Align::Center);
    }
}
