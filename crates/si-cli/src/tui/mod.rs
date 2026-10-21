//! `si study`: the hindsight review as screens to move through instead of a report to
//! scroll. Same engine and numbers as `si hindsight`; the review runs in a worker
//! thread one turn at a time and each turn lands on the screen as it finishes. Spec:
//! docs/superpowers/specs/2026-10-20-study-tui.md.

mod pacing;
mod panels;
mod tabs;
mod text;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;
use ratatui::text::{Line as TLine, Span, Text};
use ratatui::widgets::{Axis, Bar, BarChart, BarGroup, Block, BorderType, Borders, Cell, Chart, Clear, Dataset, GraphType, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;
use si_defs::Definitions;
use si_hindsight::{Calibration, Progress, ReviewOptions, TurnReview, Values};
use si_model::element::{Element, ElementSet};
use si_model::event::InvaderStep;
use si_model::island::{LandId, PieceKind, Terrain};
use si_model::power::Speed;
use si_model::spirit::{GrowthOptionDef, Placement, Reclaim};
use si_record::GameRecord;

use pacing::*;
use panels::*;
use tabs::*;
use text::*;

/// What the worker sends the screen.
enum Msg {
    Progress(Progress),
    Meta(Box<(Values, Calibration, si_hindsight::deck::Deck)>),
    Turn(Box<TurnReview>),
    Finished,
    Inject(Vec<String>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Turn,
    Pacing,
}

struct App<'a> {
    rec: Arc<GameRecord>,
    defs: &'a Definitions,
    path: PathBuf,
    is_capture: bool,
    turns: Vec<u32>,
    reviews: BTreeMap<u32, TurnReview>,
    values: Option<Values>,
    calibration: Option<Calibration>,
    deck: Option<si_hindsight::deck::Deck>,
    pool_at: BTreeMap<u32, (i64, bool)>,
    cur: usize,
    row: usize,
    screen: Screen,
    help: bool,
    status: String,
    running: bool,
    prompt: Option<String>,
    tries: BTreeMap<u32, si_hindsight::Line>,
    popup: Option<(String, Vec<String>)>,
    /// Per turn, the tree rows (growth, play set) that are open.
    expanded: BTreeMap<u32, BTreeSet<String>>,
    codes: RefCell<si_record::notation::Codes<'a>>,
    tx: mpsc::Sender<Msg>,
}

/// One row of the lines table: the actual line, a growth (its best play set's numbers),
/// a play set under it (its best targeting's numbers), a targeting, or a typed line.
#[derive(Clone)]
struct RowView {
    tag: String,
    level: u8,
    key: String,
    line: si_hindsight::Line,
    /// Children a group row holds (0 for a leaf).
    children: usize,
    open: bool,
}

struct Node {
    key: String,
    line: si_hindsight::Line,
    children: Vec<Node>,
}

fn power_names(l: &si_hindsight::Line, innate: bool, code: &dyn Fn(&str) -> String) -> String {
    let v: Vec<String> = l.plays.iter().filter(|p| p.innate == innate).map(|p| code(&p.power)).collect();
    if v.is_empty() && !innate { "(nothing)".into() } else { v.join(" ") }
}

/// The ranked lines as a tree: growth, then the set of cards and innates played, then
/// each targeting; every group keeps its best member (the lines come ranked) in front.
fn tree(ranked: &[si_hindsight::Line]) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    for l in ranked.iter().filter(|l| !l.actual) {
        let gkey = format!("G:{}", l.growth.join(" "));
        let mut powers: Vec<String> = l.plays.iter().map(|p| format!("{}{}", p.power, if p.innate { "*" } else { "" })).collect();
        powers.sort();
        let skey = format!("{gkey}|S:{}", powers.join(" "));
        let g = match out.iter_mut().find(|n| n.key == gkey) {
            Some(g) => g,
            None => {
                out.push(Node { key: gkey.clone(), line: l.clone(), children: vec![] });
                out.last_mut().unwrap()
            }
        };
        let s = match g.children.iter_mut().find(|n| n.key == skey) {
            Some(s) => s,
            None => {
                g.children.push(Node { key: skey.clone(), line: l.clone(), children: vec![] });
                g.children.last_mut().unwrap()
            }
        };
        s.children.push(Node { key: format!("{skey}|{}", s.children.len()), line: l.clone(), children: vec![] });
    }
    out
}

impl App<'_> {
    fn turn(&self) -> Option<u32> {
        self.turns.get(self.cur).copied()
    }

    fn review(&self) -> Option<&TurnReview> {
        self.turn().and_then(|t| self.reviews.get(&t))
    }

}

impl<'a> App<'a> {
    /// The Turn screen's context for the current tab, or None while the turn is unreviewed.
    fn turn_ctx(&self) -> Option<TurnCtx<'_, 'a>> {
        let review = self.review()?;
        let rows = self.rows();
        let row = self.row.min(rows.len().saturating_sub(1));
        let sel = rows.get(row).cloned();
        let t = self.rec.turn(review.turn);
        let s = self.rec.spirit_turn(&review.spirit, review.turn);
        let panel = self.defs.spirits.get(&review.spirit);
        let island = t.map(|t| si_rules::verify::island_for(&self.rec, self.defs, &t.before_fast)).unwrap_or_default();
        Some(TurnCtx { rec: &self.rec, defs: self.defs, codes: &self.codes, turn: review.turn, spirit: review.spirit.clone(), kind: TurnKind::Closed(review), t, s, panel, island, rows, row, sel, pool: self.pool_at.get(&review.turn).copied() })
    }
}

impl App<'_> {

    /// The rows the table shows for the current turn: the actual line, then the ranked
    /// alternatives as a tree (growth, play set, targeting) with the open groups
    /// unfolded, then a typed line when there is one.
    fn rows(&self) -> Vec<RowView> {
        let Some(r) = self.review() else { return vec![] };
        let open = self.turn().map(|t| self.open_set(t)).unwrap_or_default();
        let mut out: Vec<RowView> = vec![RowView { tag: "actual".into(), level: 0, key: "actual".into(), line: r.actual.clone(), children: 0, open: false }];
        for (gi, g) in tree(&r.ranked).iter().enumerate() {
            let g_open = open.contains(&g.key);
            let sets = g.children.len();
            out.push(RowView { tag: if gi == 0 { "best".into() } else { "then".into() }, level: 0, key: g.key.clone(), line: g.line.clone(), children: sets, open: g_open });
            if !g_open {
                continue;
            }
            for s in &g.children {
                let s_open = open.contains(&s.key);
                out.push(RowView { tag: "set".into(), level: 1, key: s.key.clone(), line: s.line.clone(), children: s.children.len(), open: s_open });
                if !s_open {
                    continue;
                }
                for t in &s.children {
                    out.push(RowView { tag: "at".into(), level: 2, key: t.key.clone(), line: t.line.clone(), children: 0, open: false });
                }
            }
        }
        if let Some(t) = self.turn().and_then(|t| self.tries.get(&t)) {
            out.push(RowView { tag: "try".into(), level: 0, key: "try".into(), line: t.clone(), children: 0, open: false });
        }
        out
    }

    /// The groups open for a turn: the first growth (the best) until the tree is touched.
    fn open_set(&self, t: u32) -> BTreeSet<String> {
        match self.expanded.get(&t) {
            Some(s) => s.clone(),
            None => self.reviews.get(&t).and_then(|r| tree(&r.ranked).first().map(|g| g.key.clone())).into_iter().collect(),
        }
    }

    /// Open or close the group under the cursor.
    fn toggle(&mut self) {
        let rows = self.rows();
        let Some(row) = rows.get(self.row.min(rows.len().saturating_sub(1))) else { return };
        if row.children == 0 {
            return;
        }
        let Some(t) = self.turn() else { return };
        let default = self.open_set(t);
        let set = self.expanded.entry(t).or_insert(default);
        if !set.remove(&row.key) {
            set.insert(row.key.clone());
        }
    }

    /// Open every group (`e`) or close them all (`c`).
    fn expand_all(&mut self, open: bool) {
        let Some(t) = self.turn() else { return };
        let keys: Vec<String> = self.rows().iter().filter(|r| r.children > 0).map(|r| r.key.clone()).collect();
        let default = self.open_set(t);
        let set = self.expanded.entry(t).or_insert(default);
        if !open {
            set.clear();
            self.row = 0;
            return;
        }
        for k in keys {
            set.insert(k);
        }
        // the sets inside the groups just opened
        let keys: Vec<String> = self.rows().iter().filter(|r| r.children > 0).map(|r| r.key.clone()).collect();
        let default = self.open_set(t);
        let set = self.expanded.entry(t).or_insert(default);
        for k in keys {
            set.insert(k);
        }
    }
}

/// `si study <game> [--defs] [--fixed-growth] [--seed save] [--calibration json]`.
pub fn run(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { anyhow::bail!("study needs a log file or capture directory") };
    let Some(defs) = crate::defs_from(args)? else { anyhow::bail!("study needs --defs <dir> or SI_DEFS_DIR") };
    let fixed_growth = args.iter().any(|a| a == "--fixed-growth");
    let path = PathBuf::from(path);
    let (lines, undo, source) = si_record::replay::load(&path)?;
    let mut rec = si_record::Replay::new(Some(&defs)).with_seed(crate::seed_from_args(args)?).run(&lines, undo.as_deref(), &source);
    crate::apply_save(&mut rec, &path);
    let pool_at = crate::blight_pool_by_turn(&path);
    let calibration = crate::calibration_from(args);
    let rec = Arc::new(rec);
    let defs = Arc::new(defs);
    let turns: Vec<u32> = si_hindsight::closed_turns(&rec);
    let (tx, rx) = mpsc::channel::<Msg>();
    // the worker: one review call per turn, so turns land as they finish
    {
        let rec = Arc::clone(&rec);
        let defs = Arc::clone(&defs);
        let tx = tx.clone();
        let turns = turns.clone();
        let opts = ReviewOptions { growth: !fixed_growth, calibration: calibration.clone(), ..Default::default() };
        std::thread::spawn(move || {
            let on_progress = |p: Progress| {
                let _ = tx.send(Msg::Progress(p));
            };
            let mut meta_sent = false;
            for t in turns {
                let r = si_hindsight::review_turns(&rec, &defs, &opts, Some(&[t]), &on_progress);
                if !meta_sent {
                    let _ = tx.send(Msg::Meta(Box::new((r.values.clone(), r.calibration.clone(), r.deck.clone()))));
                    meta_sent = true;
                }
                for tr in r.turns {
                    let _ = tx.send(Msg::Turn(Box::new(tr)));
                }
            }
            let _ = tx.send(Msg::Finished);
        });
    }
    let is_capture = path.join("saves").is_dir();
    let defs_ref: &Definitions = &defs;
    let mut app = App {
        codes: RefCell::new(si_record::notation::Codes::new(Some(defs_ref), false)),
        rec,
        defs: defs_ref,
        path,
        is_capture,
        turns,
        reviews: BTreeMap::new(),
        values: None,
        calibration,
        deck: None,
        pool_at,
        cur: 0,
        row: 0,
        screen: Screen::Turn,
        help: false,
        status: "reviewing...".into(),
        running: true,
        prompt: None,
        tries: BTreeMap::new(),
        popup: None,
        expanded: BTreeMap::new(),
        tx,
    };
    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal, &mut app, &rx);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut ratatui::DefaultTerminal, app: &mut App, rx: &mpsc::Receiver<Msg>) -> Result<()> {
    loop {
        while let Ok(m) = rx.try_recv() {
            match m {
                Msg::Progress(p) => app.status = progress_words(&p),
                Msg::Meta(b) => {
                    let (v, c, d) = *b;
                    app.values = Some(v);
                    app.calibration = Some(c);
                    app.deck = Some(d);
                }
                Msg::Turn(t) => {
                    app.reviews.insert(t.turn, *t);
                }
                Msg::Finished => {
                    app.running = false;
                    app.status = format!("{} turns reviewed", app.reviews.len());
                }
                Msg::Inject(lines) => app.popup = Some(("inject".into(), lines)),
            }
        }
        terminal.draw(|f| draw(f, app))?;
        if event::poll(Duration::from_millis(120))? {
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                if let Some(text) = app.prompt.as_mut() {
                    match k.code {
                        KeyCode::Esc => app.prompt = None,
                        KeyCode::Enter => {
                            let text = text.clone();
                            app.prompt = None;
                            if let Some(t) = app.turn() {
                                match si_hindsight::try_line(&app.rec, app.defs, t, &text) {
                                    Ok(l) => {
                                        app.tries.insert(t, l);
                                    }
                                    Err(e) => app.popup = Some(("try".into(), vec![e])),
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            text.pop();
                        }
                        KeyCode::Char(c) => text.push(c),
                        _ => {}
                    }
                    continue;
                }
                if app.popup.is_some() {
                    app.popup = None;
                    continue;
                }
                if app.help {
                    app.help = false;
                    continue;
                }
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Char('?') => app.help = true,
                    KeyCode::Tab => app.screen = if app.screen == Screen::Turn { Screen::Pacing } else { Screen::Turn },
                    KeyCode::Left | KeyCode::Char('h') => {
                        app.cur = app.cur.saturating_sub(1);
                        app.row = 0;
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if app.cur + 1 < app.turns.len() {
                            app.cur += 1;
                            app.row = 0;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let n = app.rows().len();
                        if n > 0 && app.row + 1 < n {
                            app.row += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.row = app.row.saturating_sub(1),
                    KeyCode::Char(c @ '1'..='9') => {
                        let i = c as usize - '1' as usize;
                        if i < app.turns.len() {
                            app.cur = i;
                            app.row = 0;
                        }
                    }
                    KeyCode::Char('t') => app.prompt = Some(String::new()),
                    KeyCode::Char('i') => inject(app),
                    KeyCode::Enter | KeyCode::Char(' ') => app.toggle(),
                    KeyCode::Char('e') => app.expand_all(true),
                    KeyCode::Char('c') => app.expand_all(false),
                    _ => {}
                }
            }
        }
    }
}

fn progress_words(p: &Progress) -> String {
    match p {
        Progress::Start { turn, positions } => format!("T{turn}: {positions} growth positions to try"),
        Progress::Positions { turn, done, total } => format!("T{turn}: {done}/{total} growth positions"),
        Progress::Ranking { turn, play_sets, lines } => format!("T{turn}: ranking {lines} lines from {play_sets} play sets over the Explore cards"),
        Progress::Done { turn, seconds, .. } => format!("T{turn} done in {seconds:.1}s"),
    }
}

/// Load the current turn's start-of-turn checkpoint into the running game: the binary
/// itself as a child (`si inject <capture> --turn n --phase SpiritPhase`), its output
/// shown in a panel. Only a capture has checkpoints.
fn inject(app: &mut App) {
    let Some(turn) = app.turn() else { return };
    if !app.is_capture {
        app.popup = Some(("inject".into(), vec!["this game is a bare EndGame log: no checkpoints to load".into(), "si import <EndGame log> builds a live-<id>/ capture from the saves the app still has; the recorder keeps the full set".into()]));
        return;
    }
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            app.popup = Some(("inject".into(), vec![format!("cannot find the si binary: {e}")]));
            return;
        }
    };
    let path = app.path.clone();
    let tx = app.tx.clone();
    app.status = format!("injecting T{turn} SpiritPhase into the running game...");
    std::thread::spawn(move || {
        let out = std::process::Command::new(exe).arg("inject").arg(&path).args(["--turn", &turn.to_string(), "--phase", "SpiritPhase"]).output();
        let lines: Vec<String> = match out {
            Ok(o) => {
                let mut v: Vec<String> = String::from_utf8_lossy(&o.stdout).lines().map(|l| l.to_string()).collect();
                v.extend(String::from_utf8_lossy(&o.stderr).lines().map(|l| l.to_string()));
                if !o.status.success() {
                    v.push(format!("inject exited with {}", o.status));
                }
                if v.is_empty() {
                    v.push("inject printed nothing".into());
                }
                v
            }
            Err(e) => vec![format!("could not run si inject: {e}")],
        };
        let _ = tx.send(Msg::Inject(lines));
    });
}

fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let [top, tabs, body, foot] = Layout::vertical([Constraint::Length(1), Constraint::Length(2), Constraint::Min(5), Constraint::Length(1)]).areas(area);
    draw_header(f, app, top);
    draw_tabs(f, app, tabs);
    match app.screen {
        Screen::Turn => draw_turn(f, app, body),
        Screen::Pacing => draw_pacing(f, app, body),
    }
    draw_footer(f, app, foot);
    if let Some(text) = &app.prompt {
        let r = centered(area, 70, 3);
        f.render_widget(Clear, r);
        let p = Paragraph::new(format!("{text}_")).block(Block::default().borders(Borders::ALL).title(" try a line (SLSF>C8 MI; Enter scores it, Esc cancels) ").border_style(Style::default().fg(ACCENT)));
        f.render_widget(p, r);
    }
    if let Some((title, lines)) = &app.popup {
        let h = (lines.len() as u16 + 2).clamp(3, area.height.saturating_sub(2).max(3));
        let r = centered(area, 100, h);
        f.render_widget(Clear, r);
        let p = Paragraph::new(lines.join("\n")).wrap(Wrap { trim: false }).block(Block::default().borders(Borders::ALL).title(format!(" {title} (any key closes) ")).border_style(Style::default().fg(ACCENT)));
        f.render_widget(p, r);
    }
    if app.help {
        let r = centered(area, 86, 19);
        f.render_widget(Clear, r);
        let text = "h / l, arrows   previous / next turn      1-9  jump to a turn\nj / k           move along the rows\nEnter, Space    open or close the row's group   e / c  open or close every group\nTab             Turn <-> Pacing\nt               type a line in notation and score it (try row)\ni               load this turn's start into the running game (si inject)\n?               this key        q  quit\n\nTurn: LINES is the tree (a growth, its play sets, each set's targetings; a group\nrow carries its best member's numbers) with the READING under it. POSITION is the\nisland at the Fast phase: the steps reaching each land, the selected row's plays\nthere (line), what the steps did (came). INVADER PHASE: this turn's cards and\nwhat came. SPIRIT, GROWTH, HAND: what you had; the row under the cursor lights\nits innate tiers, cards and lands. LINE and FORECAST along the bottom.\nPacing: Blight, Invaders, Fear, the spirit's track and the review, by turn.";
        let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" keys ").border_style(Style::default().fg(ACCENT)));
        f.render_widget(p, r);
    }
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let [r] = Layout::horizontal([Constraint::Length(w.min(area.width))]).flex(Flex::Center).areas(area);
    let [r] = Layout::vertical([Constraint::Length(h.min(area.height))]).flex(Flex::Center).areas(r);
    r
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let m = &app.rec.meta;
    let spirit = m.spirits.iter().map(|s| app.defs.spirits.get(s).map(|p| p.title.clone()).unwrap_or_else(|| s.clone())).collect::<Vec<_>>().join(" + ");
    let adversary = match (&m.adversary, m.adversary_level) {
        (Some(id), l) => format!("{}{}", app.defs.adversaries.get(id).map(|a| a.title.clone()).unwrap_or_else(|| id.clone()), l.map(|l| format!(" level {l}")).unwrap_or_default()),
        _ => "no adversary".into(),
    };
    let result = m.ending.map(|e| e.words().to_string()).unwrap_or_else(|| format!("in progress, {} turns", m.turns));
    let deck = app.deck.as_ref().map(|d| d.source.rsplit(": ").next().unwrap_or("").to_string()).unwrap_or_default();
    let line = TLine::from(vec![
        Span::styled(" STUDY ", header_style()),
        Span::raw(format!(" {spirit} | board {} | {adversary} | {result} | deck {deck} | {}", m.boards.join(" "), if app.is_capture { "capture" } else { "bare log" })),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let keys = " h/l turn  j/k row  Enter open/close  e/c all  Tab pacing  t try  i inject  ? keys  q quit ";
    let status = if app.running { format!("{}  ", app.status) } else { app.status.clone() };
    let line = TLine::from(vec![Span::styled(keys, Style::default().fg(Color::DarkGray)), Span::raw(" "), Span::styled(status, Style::default().fg(ACCENT))]);
    f.render_widget(Paragraph::new(line), area);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_fits_inside() {
        let r = centered(Rect::new(0, 0, 100, 40), 70, 3);
        assert_eq!((r.width, r.height), (70, 3));
        assert!(r.x >= 10 && r.y >= 15);
        let r = centered(Rect::new(0, 0, 50, 10), 70, 30);
        assert_eq!((r.width, r.height), (50, 10));
    }
}
