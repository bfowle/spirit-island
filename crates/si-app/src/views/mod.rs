//! The companion screen, rebuilt from scratch around Lumia's ledgers and
//! Rei's curve (Brett, 2026-10-19: from the Rust data and Lumia's + Rei's
//! approaches, one view at a time; not the old dashboard panels).
//!
//! ```text
//! turn ribbon: Spirit · Fast · Event · Fear · Ravage · Build · Explore · Slow
//! the Ledger: one lane per budget (Blight, Fear, Invader deck, loss clocks),
//!   with the Runway and the Pace beside Blight and Fear
//! the Curve: the Invader step against the Spirits' economy (Tab switches)
//! status line
//! ```
//!
//! What-if: the player tries changes on the current moment (a fix from the
//! Ledger, or the What-If drawer, W) and every view recomputes on a copy of
//! the snapshot, showing what moved. Nothing is sent to the game.

mod chart;
mod curve;
mod fmt;
mod ledger;
mod mock_growth;
mod pace;
mod runway;
mod vs;
mod whatif;

use si_common::companion_stats::whatif::Edit;
use si_common::companion_stats::{self, CompanionStats};
use si_steam::companion::CompanionState;
use si_steam::source::ReplayCommand;
use si_ui::theme::{self, TextStyle};
use si_ui::{Align, Key, Rect, Ui};

/// View state that outlives a frame.
#[derive(Debug, Default)]
pub struct ViewState {
    /// Time to build and submit the last frame.
    pub frame_ms: f32,
    /// GPU adapter, for the status line.
    pub gpu: String,
    pub source_label: String,
    pub page: Page,
    /// A static mockup to draw instead of the live views ("growth").
    pub mock: Option<String>,
    /// What-if changes on the current moment.
    pub edits: Vec<Edit>,
    /// Changes given on the command line, parsed once a snapshot arrives.
    pub pending_edits: Vec<String>,
    /// The What-If drawer is open.
    pub drawer: bool,
    /// The moment the changes were made on (turn, step).
    pub moment: (u32, String),
    /// Stats with the changes applied, for the state and changes they were
    /// computed from.
    pub tried: Option<(u64, Vec<Edit>, Box<CompanionStats>)>,
    /// The Spirit shown on the versus screen's left side.
    pub tab: usize,
    /// The game's art, by file stem ("Sprite_LightningsSwiftStrikeBack"),
    /// as images registered with the renderer.
    pub art: std::collections::HashMap<String, crate::icons::Art>,
}

/// The stats the views draw: the game as it is, and with the what-if
/// changes applied (the same when there are none).
pub struct Both<'a> {
    pub base: &'a CompanionStats,
    pub tried: &'a CompanionStats,
    pub edits: &'a [Edit],
}

impl Both<'_> {
    pub fn trying(&self) -> bool {
        !self.edits.is_empty()
    }
}

/// A change to the what-if, from a click.
pub enum Toggle {
    Add(Edit),
    Remove(Edit),
    Clear,
    CloseDrawer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// The versus screen: the Spirit against the Invaders.
    #[default]
    Vs,
    Ledger,
    Curve,
}

impl Page {
    pub fn named(name: Option<&str>) -> Page {
        match name {
            Some("curve") => Page::Curve,
            Some("ledger") => Page::Ledger,
            _ => Page::Vs,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Page::Vs => "VERSUS",
            Page::Ledger => "THE LEDGER",
            Page::Curve => "THE CURVE",
        }
    }

    fn next(self) -> Page {
        match self {
            Page::Vs => Page::Ledger,
            Page::Ledger => Page::Curve,
            Page::Curve => Page::Vs,
        }
    }
}

pub enum Action {
    Replay(ReplayCommand),
}

/// The steps of a turn, as the ribbon shows them.
pub const STEPS: [&str; 8] = ["Spirit", "Fast", "Event", "Fear", "Ravage", "Build", "Explore", "Slow"];

/// Where a game step sits on the ribbon.
pub fn step_index(step: &str) -> usize {
    match step {
        "FastPower" => 1,
        "BlightedIsland" | "Event" => 2,
        "Fear" => 3,
        "HighImmigration" | "Ravage" => 4,
        "Build" => 5,
        "Explore" | "AdvanceInvaderCards" => 6,
        "SlowPower" | "TimePasses" => 7,
        _ => 0,
    }
}

pub fn draw(ui: &mut Ui, v: &mut ViewState, st: &CompanionState) -> Vec<Action> {
    let mut actions = Vec::new();
    if v.mock.as_deref() == Some("growth") {
        mock_growth::draw(ui);
        return actions;
    }
    if v.mock.as_deref() == Some("alpha") {
        // A blend test: fills, meshes and lines at rising alpha over white.
        ui.fill(Rect::new(40.0, 40.0, 1000.0, 300.0), theme::Rgba::hex(0xffffff), 0.0);
        for (i, a) in [16u8, 64, 128, 192, 236, 255].iter().enumerate() {
            let x = 60.0 + i as f32 * 160.0;
            ui.fill(Rect::new(x, 60.0, 120.0, 80.0), theme::Rgba(14, 15, 20, *a), 6.0);
            ui.quad([(x, 160.0), (x + 120.0, 160.0), (x + 120.0, 240.0), (x, 240.0)], [theme::Rgba(14, 15, 20, *a); 4]);
            ui.line((x, 280.0), (x + 120.0, 280.0), 12.0, theme::Rgba(139, 124, 246, *a), 0.0);
            ui.text(x, 300.0, &format!("a {a}"), theme::NUM.color(theme::Rgba::hex(0x000000)));
        }
        return actions;
    }
    keys(ui, st, &mut actions);
    // P cycles the pages (not Tab: Alt+Tab away from the window arrives as
    // a Tab press and switched pages behind Brett's back).
    if ui.key(Key::Char('p')) || ui.key(Key::Char('P')) {
        v.page = v.page.next();
    }
    if ui.key(Key::Char('w')) || ui.key(Key::Char('W')) {
        v.drawer = !v.drawer;
    }
    if ui.key(Key::Escape) {
        v.edits.clear();
        v.drawer = false;
    }
    let mut r = ui.screen;
    let status = r.cut_bottom(26.0);
    let (Some(s), Some(stats)) = (st.public.as_ref(), st.stats.as_ref()) else {
        status_line(ui, status, v, st);
        waiting(ui, r, st);
        return actions;
    };
    // A what-if belongs to the moment it was made on.
    let moment = (s.turn, s.step.clone());
    if v.moment != moment {
        v.moment = moment;
        v.edits.clear();
    }
    if !v.pending_edits.is_empty() {
        v.edits = v.pending_edits.iter().filter_map(|e| Edit::parse(e, s)).collect();
        v.pending_edits.clear();
    }
    status_line(ui, status, v, st);
    if v.page == Page::Vs {
        vs::draw(ui, r, s, stats, &st.history, v, st, &mut actions);
        return actions;
    }
    let head = r.cut_top(80.0);
    ribbon(ui, head, s, v);
    let body = r.inset(28.0, 6.0, 28.0, 14.0);

    // The what-if's stats, recomputed only when the state or the changes move.
    let cache = v.tried.take().filter(|(seq, e, _)| *seq == st.seq && *e == v.edits);
    let cache = (!v.edits.is_empty()).then(|| {
        cache.unwrap_or_else(|| {
            let tried = companion_stats::whatif::apply(s, &v.edits);
            (st.seq, v.edits.clone(), Box::new(companion_stats::compute(&tried, &st.history)))
        })
    });
    let mut toggles = Vec::new();
    {
        let both = Both { base: stats, tried: cache.as_ref().map(|c| &*c.2).unwrap_or(stats), edits: &v.edits };
        if both.trying() {
            ui.rect(body.inset(-8.0, -4.0, -8.0, -6.0), theme::TRANSPARENT, theme::AMBER.alpha(120), 1.0, 12.0);
        }
        let slot = match v.page {
            Page::Ledger => ledger::draw(ui, body, s, &both, &st.history, &mut toggles),
            Page::Curve | Page::Vs => {
                curve::draw(ui, body, s, &both);
                None
            }
        };
        if v.drawer {
            let h = (body.h * 0.36).clamp(220.0, 280.0);
            let at = slot.unwrap_or(Rect::new(body.x, body.bottom() - h, body.w, h));
            whatif::drawer(ui, at, s, &both, &mut toggles);
        }
    }
    v.tried = cache;
    for t in toggles {
        match t {
            Toggle::Add(e) => v.edits.push(e),
            Toggle::Remove(e) => {
                if let Some(i) = v.edits.iter().position(|x| *x == e) {
                    v.edits.remove(i);
                }
            }
            Toggle::Clear => v.edits.clear(),
            Toggle::CloseDrawer => v.drawer = false,
        }
    }
    actions
}

fn keys(ui: &mut Ui, st: &CompanionState, actions: &mut Vec<Action>) {
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

/// The turn as a line of steps: done steps dim, the current one lit and
/// breathing, the rest ahead. Spends in the ledger name these steps.
fn ribbon(ui: &mut Ui, r: Rect, s: &si_common::public::PublicSnapshot, v: &mut ViewState) {
    let now = step_index(&s.step);
    let turn_style = TextStyle { size: 34.0, line: 38.0, mono: false, weight: 700, color: theme::TEXT };
    ui.text(r.x + 28.0, r.y + 12.0, &format!("Turn {}", s.turn), turn_style);
    // The pages, as tabs (Tab or a click switches).
    let mut x = r.x + 30.0;
    for page in [Page::Ledger, Page::Curve] {
        let label = page.title().trim_start_matches("THE ");
        let on = v.page == page;
        let style = theme::HEAD.color(if on { theme::TEXT } else { theme::MUTED });
        let w = ui.measure(label, style).0;
        let tab = Rect::new(x - 4.0, r.y + 50.0, w + 8.0, 20.0);
        if ui.clicked(tab) {
            v.page = page;
        }
        ui.text(x, r.y + 52.0, label, style);
        if on {
            ui.fill(Rect::new(x, r.y + 69.0, w, 2.0), theme::ACCENT, 1.0);
        }
        x += w + 16.0;
    }
    // The What-If drawer's switch, with how many changes are being tried.
    let n = v.edits.len();
    let label = if n > 0 { format!("WHAT IF · {n}") } else { "WHAT IF".to_string() };
    let lit = v.drawer || n > 0;
    let style = theme::HEAD.color(if lit { theme::AMBER } else { theme::MUTED });
    let w = ui.measure(&label, style).0;
    let tab = Rect::new(x - 6.0, r.y + 48.0, w + 12.0, 22.0);
    if ui.clicked(tab) {
        v.drawer = !v.drawer;
    }
    ui.rect(tab, if v.drawer { theme::AMBER.alpha(30) } else { theme::TRANSPARENT }, if lit { theme::AMBER.alpha(160) } else { theme::BORDER_STRONG }, 1.0, 11.0);
    ui.text(x, r.y + 52.0, &label, style);

    let line = Rect::new(r.x + 240.0, r.y + 32.0, r.w - 300.0, 2.0);
    let n = STEPS.len();
    let gap = line.w / (n - 1) as f32;
    let pt = |i: usize| (line.x + i as f32 * gap, line.y + 1.0);
    // The part of the turn already played, then what's ahead.
    ui.line(pt(0), pt(now), 3.0, theme::ACCENT.alpha(170), 0.0);
    ui.line(pt(now), pt(n - 1), 2.0, theme::BORDER_STRONG, 0.0);
    let breath = ui.pulse(2.4);
    for (i, name) in STEPS.iter().enumerate() {
        let c = pt(i);
        if i == now {
            ui.circle(c, 16.0 + 6.0 * breath, theme::ACCENT.alpha((40.0 + 50.0 * breath) as u8), theme::TRANSPARENT, 0.0);
            ui.circle(c, 9.0, theme::ACCENT, theme::Rgba::hex(0xd8d2ff), 2.0);
        } else if i < now {
            ui.circle(c, 6.0, theme::ACCENT.alpha(170), theme::TRANSPARENT, 0.0);
        } else {
            ui.circle(c, 6.0, theme::BG, theme::BORDER_STRONG, 2.0);
        }
        let style = TextStyle {
            size: if i == now { 14.0 } else { 12.5 },
            line: 18.0,
            mono: false,
            weight: if i == now { 700 } else { 500 },
            color: if i == now { theme::TEXT } else if i < now { theme::MUTED } else { theme::TEXT_2 },
        };
        ui.text_in(Rect::new(c.0 - 60.0, c.1 + 16.0, 120.0, 20.0), name, style, Align::Center);
    }
}

fn status_line(ui: &mut Ui, r: Rect, v: &ViewState, st: &CompanionState) {
    ui.fill(Rect::new(r.x, r.y, r.w, 1.0), theme::BORDER, 0.0);
    let row = r.inset(28.0, 0.0, 28.0, 0.0);
    let style = theme::SMALL.color(theme::FAINT);
    let mut left = Vec::new();
    if let Some(s) = st.public.as_ref() {
        let adv: Vec<String> = s.adversaries.iter().map(|a| format!("{} L{}", a.title, a.level)).collect();
        let spirits: Vec<&str> = s.spirits.iter().map(|x| x.title.as_str()).collect();
        left.push(if adv.is_empty() { "no adversary".to_string() } else { adv.join(" + ") });
        left.push(spirits.join(", "));
        if let Some(sc) = &s.scenario {
            left.push(sc.clone());
        }
    }
    if v.edits.is_empty() {
        ui.text_in(row, &left.join("   ·   "), style, Align::Left);
    } else if let Some(s) = st.public.as_ref() {
        // In a what-if, the changes being tried take the left side.
        let tried: Vec<String> = v.edits.iter().map(|e| e.describe(|l| fmt::short(s, l))).collect();
        ui.text_in(row, &format!("What if: {}   ·   Esc clears", tried.join(" · ")), theme::SMALL.color(theme::AMBER), Align::Left);
    }
    let mut right = vec![v.source_label.clone()];
    if let Some(rep) = st.source.replay.as_ref() {
        right.push(format!("save {} / {}  ·  ← → step, Space play, P page, W what-if", rep.index + 1, rep.total));
    }
    if !v.gpu.is_empty() {
        right.push(v.gpu.clone());
    }
    ui.text_in(row, &right.join("   ·   "), style, Align::Right);
}

fn waiting(ui: &mut Ui, r: Rect, st: &CompanionState) {
    let msg = match st.source.mode.as_str() {
        "live" => "Waiting for a Spirit Island game in Steam.",
        "replay" => "Decoding the capture…",
        _ => "The Steam source is off (engine DLL or save folder not found).",
    };
    let mid = Rect::new(r.x, r.y + r.h / 2.0 - 20.0, r.w, 28.0);
    ui.text_in(mid, msg, theme::TITLE.color(theme::TEXT_2), Align::Center);
}
