//! `si study`: the hindsight review as screens to move through instead of a report to
//! scroll. Same engine and numbers as `si hindsight`; the review runs in a worker
//! thread one turn at a time and each turn lands on the screen as it finishes. Spec:
//! docs/superpowers/specs/2026-10-20-study-tui.md.

mod activity;
mod open;
mod pacing;
mod panels;
mod plans;
mod review;
mod source;
mod tabs;
mod text;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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
use si_model::game::Phase;
use si_record::replay::Seed;
use si_record::GameRecord;

use open::*;
use pacing::*;
use panels::*;
use tabs::*;
use text::*;

/// What the worker sends the screen.
enum Msg {
    Progress(Progress),
    Meta(Box<(Values, Calibration, si_hindsight::deck::Deck)>),
    Turn { generation: u64, review: Box<TurnReview> },
    Finished { generation: u64 },
    Inject(Vec<String>),
    /// The live source: a fresh record from the journal and the newest save's seed.
    Live(Box<source::LiveUpdate>),
    /// The plan worker: the open turn scored.
    Plans(Box<plans::Scored>),
    /// The plan worker: the seed names no open turn.
    NoOpenTurn { generation: u64 },
    /// Live: what the app asks the player now, from its Player.log (None: nothing read).
    Activity(Option<String>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Turn,
    Pacing,
}

/// A tab of the bar: a closed turn (reviewed, or being reviewed) or the turn being played.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Closed(u32),
    Open(u32, Phase),
}

impl Tab {
    fn turn(self) -> u32 {
        match self {
            Tab::Closed(t) | Tab::Open(t, _) => t,
        }
    }
}

/// The turn being played, as the plan worker last scored it. The last scoring stays on
/// screen while a new one runs (`pending`), so an update never blanks the panels.
struct OpenState {
    turn: u32,
    phase: Phase,
    seed: Seed,
    scored: Option<plans::Scored>,
    pending: bool,
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
    /// Per turn, the lines typed and scored with `t` (a closed turn keeps the plans it had while open).
    tries: BTreeMap<u32, Vec<si_hindsight::Line>>,
    popup: Option<(String, Vec<String>)>,
    /// Per turn, the tree rows (growth, play set) that are open.
    expanded: BTreeMap<u32, BTreeSet<String>>,
    codes: RefCell<si_record::notation::Codes<'a>>,
    tx: mpsc::Sender<Msg>,
    /// The record's generation: results from an older record are dropped.
    generation: u64,
    review_tx: mpsc::Sender<review::Job>,
    plan_tx: mpsc::Sender<plans::Job>,
    /// `--live`: the record comes from the Steam folder (or a capture cut) and an open tab exists.
    live: bool,
    open: Option<OpenState>,
    /// Per turn, the plan texts typed on its open tab, kept across re-scores.
    plan_texts: BTreeMap<u32, Vec<String>>,
    /// The footer's words about the live source.
    source_words: String,
    /// Live: the capture the screen is writing (`live-<id>/`), once a save named the game.
    capture: Option<PathBuf>,
    /// Live: the save `i` last injected, which seeds the journal that restarts at the load.
    join_from: source::JoinFrom,
    /// Live: the app's current intent in words, from its Player.log; the open tab's THINK ABOUT leads with it.
    activity: Option<String>,
    /// Live: the game id the newest save names; another id is a new game, a shorter record with the same id a rollback.
    game_id: Option<String>,
    /// Per turn, per plan text: where a plan came from when the screen wrote it (an earlier attempt's line after a rollback).
    plan_notes: BTreeMap<u32, BTreeMap<String, String>>,
    /// Live, `--attempt <save>`: an earlier attempt of this game (a branch rolled back from); its review of the
    /// turn being played seeds the plans, as a rollback inside the session does.
    attempt: Option<Arc<GameRecord>>,
    attempt_asked: BTreeSet<u32>,
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
    /// The bar: the closed turns, then the open one when the game is being played.
    fn tabs(&self) -> Vec<Tab> {
        let mut v: Vec<Tab> = self.turns.iter().map(|t| Tab::Closed(*t)).collect();
        if let Some(o) = &self.open {
            v.push(Tab::Open(o.turn, o.phase));
        }
        v
    }

    fn turn(&self) -> Option<u32> {
        self.tabs().get(self.cur).map(|t| t.turn())
    }

    fn is_open_tab(&self) -> bool {
        matches!(self.tabs().get(self.cur), Some(Tab::Open(..)))
    }

    fn review(&self) -> Option<&TurnReview> {
        if self.is_open_tab() {
            return None;
        }
        self.turn().and_then(|t| self.reviews.get(&t))
    }

    fn scored(&self) -> Option<&plans::Scored> {
        if !self.is_open_tab() {
            return None;
        }
        self.open.as_ref().and_then(|o| o.scored.as_ref())
    }

    /// The open tab's rows: `none`, then each plan in the order typed; a refused plan keeps
    /// its row so `d` can remove it, with the refusal in its notes.
    fn plan_rows(&self) -> Vec<RowView> {
        let Some(sc) = self.scored() else { return vec![] };
        let mut out = vec![RowView { tag: "none".into(), level: 0, key: "none".into(), line: sc.none.clone(), children: 0, open: false }];
        for (i, (text, r)) in sc.rows.iter().enumerate() {
            let tag = ((b'a' + (i % 26) as u8) as char).to_string();
            let note = self.turn().and_then(|t| self.plan_notes.get(&t)).and_then(|m| m.get(text)).cloned();
            match r {
                Ok(l) => {
                    let mut l = l.clone();
                    if let Some(n) = note {
                        l.outcome.notes.insert(0, n);
                    }
                    out.push(RowView { tag, level: 0, key: format!("plan:{text}"), line: l, children: 0, open: false })
                }
                Err(e) => {
                    let mut l = sc.none.clone();
                    l.plays.clear();
                    l.outcome = Default::default();
                    l.outcome.notes = vec![format!("refused: {e}")];
                    l.informed = f64::NAN;
                    l.confidence = None;
                    out.push(RowView { tag, level: 0, key: format!("plan:{text}"), line: l, children: 0, open: false });
                }
            }
        }
        out
    }

    /// Score every plan typed for the open turn again (a new text, a deletion, a new state).
    fn send_plans(&mut self) {
        let Some(o) = self.open.as_mut() else { return };
        o.pending = true;
        let texts = self.plan_texts.get(&o.turn).cloned().unwrap_or_default();
        let _ = self.plan_tx.send(plans::Job { generation: self.generation, rec: Arc::clone(&self.rec), seed: o.seed.clone(), texts });
    }

    /// `d`: the plan or try row under the cursor goes.
    fn delete_row(&mut self) {
        let Some(t) = self.turn() else { return };
        if self.is_open_tab() {
            if self.row == 0 {
                return;
            }
            let i = self.row - 1;
            if let Some(v) = self.plan_texts.get_mut(&t) {
                if i < v.len() {
                    v.remove(i);
                    self.row = self.row.saturating_sub(1);
                    self.send_plans();
                }
            }
            return;
        }
        let rows = self.rows();
        let Some(row) = rows.get(self.row.min(rows.len().saturating_sub(1))) else { return };
        if let Some(i) = row.key.strip_prefix("try:").and_then(|i| i.parse::<usize>().ok()) {
            if let Some(v) = self.tries.get_mut(&t) {
                if i < v.len() {
                    v.remove(i);
                    self.row = self.row.saturating_sub(1);
                }
            }
        }
    }
}

impl<'a> App<'a> {
    /// The Turn screen's context for the current tab, or None while the turn is unreviewed.
    fn turn_ctx(&self) -> Option<TurnCtx<'_, 'a>> {
        let rows = self.rows();
        let row = self.row.min(rows.len().saturating_sub(1));
        let sel = rows.get(row).cloned();
        if self.is_open_tab() {
            let sc = self.scored()?;
            let o = &sc.open;
            let before = if o.phase == Phase::SlowPower && !o.t.before_slow.is_empty() { &o.t.before_slow } else { &o.t.before_fast };
            let island = si_rules::verify::island_for(&o.rec, self.defs, before);
            let pending = self.open.as_ref().is_some_and(|x| x.pending);
            return Some(TurnCtx { rec: &self.rec, defs: self.defs, codes: &self.codes, turn: o.turn, spirit: o.spirit.clone(), kind: TurnKind::Open(sc), t: Some(&o.t), s: Some(&o.s), panel: self.defs.spirits.get(&o.spirit), island, rows, row, sel, pool: o.seed.blight_pool.map(|p| (p, o.seed.blight_flipped)), pending, activity: self.activity.clone() });
        }
        let review = self.review()?;
        let t = self.rec.turn(review.turn);
        let s = self.rec.spirit_turn(&review.spirit, review.turn);
        let panel = self.defs.spirits.get(&review.spirit);
        let island = t.map(|t| si_rules::verify::island_for(&self.rec, self.defs, &t.before_fast)).unwrap_or_default();
        Some(TurnCtx { rec: &self.rec, defs: self.defs, codes: &self.codes, turn: review.turn, spirit: review.spirit.clone(), kind: TurnKind::Closed(review), t, s, panel, island, rows, row, sel, pool: self.pool_at.get(&review.turn).copied(), pending: false, activity: None })
    }
}

impl App<'_> {

    /// The rows the table shows for the current turn: the actual line, then the ranked
    /// alternatives as a tree (growth, play set, targeting) with the open groups
    /// unfolded, then a typed line when there is one.
    fn rows(&self) -> Vec<RowView> {
        if self.is_open_tab() {
            return self.plan_rows();
        }
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
        if let Some(v) = self.turn().and_then(|t| self.tries.get(&t)) {
            for (i, l) in v.iter().enumerate() {
                out.push(RowView { tag: "try".into(), level: 0, key: format!("try:{i}"), line: l.clone(), children: 0, open: false });
            }
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

/// `si study <game> [--defs] [--fixed-growth] [--seed save] [--calibration json]`, or
/// `si study --live [--steam <dir>] [--replay <capture> --at Tn/Phase [--save part]]`.
pub fn run(args: &[String]) -> Result<()> {
    let live = args.iter().any(|a| a == "--live");
    let Some(defs) = crate::defs_from(args)? else { anyhow::bail!("study needs --defs <dir> or SI_DEFS_DIR") };
    let fixed_growth = args.iter().any(|a| a == "--fixed-growth");
    let calibration = crate::calibration_from(args);
    let after = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let steam: PathBuf = after("--steam").map(PathBuf::from).unwrap_or_else(crate::steam_dir);
    let replay: Option<PathBuf> = after("--replay").map(PathBuf::from);
    // an earlier attempt of the game being played: a save of the branch rolled back from
    let attempt: Option<Arc<GameRecord>> = match after("--attempt") {
        Some(p) => Some(Arc::new(crate::load_record(Path::new(&p), Some(&defs), None, false)?.0)),
        None => None,
    };
    let at: Option<(i64, String)> = after("--at").and_then(|s| {
        let (t, ph) = s.split_once('/')?;
        Some((t.trim_start_matches('T').parse().ok()?, ph.to_string()))
    });
    let save_pick: Option<String> = after("--save");
    let defs = Arc::new(defs);
    let (tx, rx) = mpsc::channel::<Msg>();
    let opts = ReviewOptions { growth: !fixed_growth, calibration: calibration.clone(), ..Default::default() };
    let review_tx = review::spawn(Arc::clone(&defs), opts, tx.clone());
    let plan_tx = plans::spawn(Arc::clone(&defs), calibration.clone().unwrap_or_default(), tx.clone());
    let join_from: source::JoinFrom = Arc::new(Mutex::new(None));
    let (rec, path, is_capture, pool_at, turns, source_words) = if live {
        let path = replay.clone().unwrap_or_else(|| steam.clone());
        match &replay {
            Some(capture) => {
                let mut update = source::replay_at(capture, &defs, at, save_pick.as_deref())?;
                update.capture = Some(capture.clone());
                let _ = tx.send(Msg::Live(Box::new(update)));
            }
            None => source::spawn_live(steam.clone(), crate::capture_root(), Arc::clone(&defs), tx.clone(), Arc::clone(&join_from)),
        }
        (Arc::new(GameRecord::default()), path, false, BTreeMap::new(), vec![], "live: waiting for the journal".to_string())
    } else {
        let Some(path) = args.first().filter(|a| !a.starts_with("--")) else { anyhow::bail!("study needs a log file, a capture directory or one of its saves, or --live") };
        let path = PathBuf::from(path);
        let (mut rec, _) = crate::load_record(&path, Some(&defs), crate::seed_from_args(args)?, args.iter().any(|a| a == "--log"))?;
        // a save under a capture's saves/: the game as that save holds it (a branch the
        // capture's newest save no longer stands on); the capture serves i and the Blight pool
        let capture = if path.is_file() { path.parent().filter(|p| p.file_name().is_some_and(|n| n == "saves")).and_then(|p| p.parent()).map(Path::to_path_buf) } else { None };
        let home = capture.clone().unwrap_or_else(|| path.clone());
        crate::apply_save(&mut rec, &home);
        let pool_at = crate::blight_pool_by_turn(&home);
        let rec = Arc::new(rec);
        let turns: Vec<u32> = si_hindsight::closed_turns(&rec);
        let _ = review_tx.send(review::Job { generation: 0, rec: Arc::clone(&rec), turns: turns.clone() });
        let is_capture = home.join("saves").is_dir();
        (rec, home, is_capture, pool_at, turns, String::new())
    };
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
        status: if live { String::new() } else { "reviewing...".into() },
        running: !live,
        prompt: None,
        tries: BTreeMap::new(),
        popup: None,
        expanded: BTreeMap::new(),
        tx,
        generation: 0,
        review_tx,
        plan_tx,
        live,
        open: None,
        plan_texts: BTreeMap::new(),
        source_words,
        capture: None,
        join_from,
        activity: None,
        game_id: None,
        plan_notes: BTreeMap::new(),
        attempt,
        attempt_asked: BTreeSet::new(),
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
                Msg::Turn { generation, review } => {
                    if generation == app.generation {
                        if app.open.as_ref().is_some_and(|o| o.turn == review.turn) {
                            // the earlier attempt's review of the turn being played
                            carry_attempt(app, review.turn, &review);
                            app.send_plans();
                        } else {
                            app.reviews.insert(review.turn, *review);
                        }
                    }
                }
                Msg::Finished { generation } => {
                    if generation == app.generation {
                        app.running = false;
                        app.status = format!("{} turns reviewed", app.reviews.len());
                    }
                }
                Msg::Inject(lines) => app.popup = Some(("inject".into(), lines)),
                Msg::Live(u) => live_update(app, *u),
                Msg::Activity(a) => app.activity = a,
                Msg::Plans(sc) => {
                    if sc.generation == app.generation {
                        if let Some(o) = app.open.as_mut() {
                            if o.turn == sc.turn {
                                o.scored = Some(*sc);
                                o.pending = false;
                            }
                        }
                    }
                }
                Msg::NoOpenTurn { generation } => {
                    if generation == app.generation {
                        if let Some(o) = app.open.as_mut() {
                            o.pending = false;
                        }
                        app.status = "waiting for the next save".into();
                    }
                }
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
                                if app.is_open_tab() {
                                    if !text.trim().is_empty() {
                                        app.plan_texts.entry(t).or_default().push(text);
                                        app.send_plans();
                                    }
                                } else {
                                    match si_hindsight::try_line(&app.rec, app.defs, t, &text) {
                                        Ok(l) => app.tries.entry(t).or_default().push(l),
                                        Err(e) => app.popup = Some(("try".into(), vec![e])),
                                    }
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
                        if app.cur + 1 < app.tabs().len() {
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
                        if i < app.tabs().len() {
                            app.cur = i;
                            app.row = 0;
                        }
                    }
                    KeyCode::Char('t') => app.prompt = Some(String::new()),
                    KeyCode::Char('d') => app.delete_row(),
                    KeyCode::Char('i') => {
                        if app.is_open_tab() {
                            app.popup = Some(("inject".into(), vec!["the game is at this position already".into()]));
                        } else {
                            inject(app);
                        }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => app.toggle(),
                    KeyCode::Char('e') => app.expand_all(true),
                    KeyCode::Char('c') => app.expand_all(false),
                    _ => {}
                }
            }
        }
    }
}

/// A fresh record from the live source: reviews whose turn changed are dropped and queued
/// again, a turn that just closed keeps its plans as try rows, the open tab follows the
/// newest save, and its plans are scored from the new state.
fn live_update(app: &mut App, u: source::LiveUpdate) {
    let old = Arc::clone(&app.rec);
    app.rec = Arc::new(u.rec);
    app.source_words = u.words;
    if u.activity.is_some() {
        app.activity = u.activity;
    }
    if u.capture.is_some() {
        app.capture = u.capture.clone();
        app.is_capture = true;
    }
    app.generation += 1;
    let spirit = app.rec.meta.spirits.first().cloned().unwrap_or_default();
    let closed = si_hindsight::closed_turns(&app.rec);
    // a new game: the save names another game; without a save, a record that restarted from
    // setup. A shorter record of the same game is a rollback (i, or the app's own Undo).
    let game_id = u.seed.as_ref().map(|s| s.game_id.clone());
    let new_game = match (&app.game_id, &game_id) {
        (Some(a), Some(b)) => a != b,
        _ => app.rec.meta.from_setup && app.rec.meta.turns <= 1 && old.meta.turns > 1,
    };
    if game_id.is_some() {
        app.game_id = game_id;
    }
    if new_game {
        // nothing typed for the old one applies
        app.plan_texts.clear();
        app.plan_notes.clear();
        app.tries.clear();
        app.reviews.clear();
        app.open = None;
    }
    // a rollback onto a turn that was closed and reviewed: the earlier attempt's line and
    // its best lines become the open turn's plans, scored against the position as it stands,
    // so what was played and what was better stay in sight while the turn is played again
    let open_turn = u.seed.as_ref().filter(|_| u.open).map(|s| s.turn_index + 1);
    if let Some(t) = open_turn.filter(|t| !closed.contains(t)) {
        if let Some(r) = app.reviews.get(&t).cloned() {
            carry_attempt(app, t, &r);
        } else if let Some(a) = app.attempt.clone() {
            // --attempt: that record's review of this turn, asked for once, lands as Msg::Turn
            if !app.attempt_asked.contains(&t) && si_hindsight::closed_turns(&a).contains(&t) && app.plan_texts.get(&t).is_none_or(|v| v.is_empty()) {
                app.attempt_asked.insert(t);
                let _ = app.review_tx.send(review::Job { generation: app.generation, rec: a, turns: vec![t] });
                app.status = format!("reviewing the earlier attempt's T{t}: its line and its best lines will be the first plans");
            }
        }
    }
    app.reviews.retain(|t, _| closed.contains(t) && old.turn(*t) == app.rec.turn(*t) && old.spirit_turn(&spirit, *t) == app.rec.spirit_turn(&spirit, *t));
    let todo: Vec<u32> = closed.iter().copied().filter(|t| !app.reviews.contains_key(t)).collect();
    app.turns = closed.clone();
    if !todo.is_empty() {
        app.running = true;
        let _ = app.review_tx.send(review::Job { generation: app.generation, rec: Arc::clone(&app.rec), turns: todo });
    }
    // a turn that just closed keeps its plans as try rows
    if let Some(o) = &app.open {
        if closed.contains(&o.turn) && !app.tries.contains_key(&o.turn) {
            let texts = app.plan_texts.get(&o.turn).cloned().unwrap_or_default();
            let lines: Vec<si_hindsight::Line> = texts.iter().filter_map(|x| si_hindsight::try_line(&app.rec, app.defs, o.turn, x.split('|').next_back().unwrap_or(x).trim()).ok()).collect();
            if !lines.is_empty() {
                app.tries.insert(o.turn, lines);
            }
        }
    }
    let was_open = app.open.is_some();
    match u.seed {
        Some(seed) if u.open => {
            let turn = seed.turn_index + 1;
            match app.open.as_mut() {
                // the same turn moved on: keep what is on screen until the new scoring lands
                Some(o) if o.turn == turn => {
                    o.phase = seed.phase;
                    o.seed = seed;
                }
                _ => app.open = Some(OpenState { turn, phase: seed.phase, seed, scored: None, pending: false }),
            }
            app.send_plans();
        }
        _ => app.open = None,
    }
    let n = app.tabs().len();
    if app.open.is_some() && !was_open {
        app.cur = n.saturating_sub(1);
        app.row = 0;
    }
    if app.cur >= n {
        app.cur = n.saturating_sub(1);
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
/// A review line as the plan prompt takes it: the growth tokens before `|` (a drafted
/// card as `+card`, since the live position cannot know the draft; a forget dropped),
/// then the plays by short code with their lands.
/// The earlier attempt's line and its best lines become the first plans of turn `t`, each
/// noted with what it was and scored then; the same text (in any order) is not added twice.
fn carry_attempt(app: &mut App, t: u32, r: &TurnReview) {
    let canonical = |text: &str| -> String { text.split('|').map(|seg| { let mut v: Vec<&str> = seg.split_whitespace().collect(); v.sort_unstable(); v.join(" ") }).collect::<Vec<_>>().join(" | ") };
    let mut carried = vec![(line_as_plan(app, &r.actual), format!("last attempt: played, scored {:.1} then", r.actual.informed))];
    for (i, l) in r.ranked.iter().filter(|l| !l.actual).take(3).enumerate() {
        carried.push((line_as_plan(app, l), format!("last attempt: {} then, scored {:.1}", if i == 0 { "best" } else { "an alternative" }, l.informed)));
    }
    let mut seen: BTreeSet<String> = app.plan_texts.get(&t).map(|v| v.iter().map(|x| canonical(x)).collect()).unwrap_or_default();
    let mut fresh: Vec<(String, String)> = Vec::new();
    for (text, note) in carried {
        if seen.insert(canonical(&text)) {
            fresh.push((text, note));
        }
    }
    if fresh.is_empty() {
        return;
    }
    let texts = app.plan_texts.entry(t).or_default();
    let notes = app.plan_notes.entry(t).or_default();
    for (text, note) in fresh.into_iter().rev() {
        texts.insert(0, text.clone());
        notes.entry(text).or_insert(note);
    }
    app.status = format!("T{t} again: the earlier attempt's line and its best lines are the first plans; d removes one, t adds yours");
}

fn line_as_plan(app: &App, l: &si_hindsight::Line) -> String {
    let code = |id: &str| app.codes.borrow_mut().code(id);
    let growth: Vec<String> = l
        .growth
        .iter()
        .filter(|t| !t.starts_with('-'))
        .map(|t| if t.starts_with('+') && !t.ends_with('E') { "+card".to_string() } else { t.clone() })
        .collect();
    let plays: Vec<String> = l
        .plays
        .iter()
        .map(|p| match &p.target {
            Some(land) if p.places => format!("{}@{land}", code(&p.power)),
            Some(land) if p.fast => format!("{}>{land}", code(&p.power)),
            // a Slow play takes its land after the Invader steps
            _ => code(&p.power),
        })
        .collect();
    if growth.is_empty() { plays.join(" ") } else { format!("{} | {}", growth.join(" "), plays.join(" ")) }
}

fn inject(app: &mut App) {
    let Some(turn) = app.turn() else { return };
    // the line under the cursor (or the best one) follows the reload as the first plan of
    // the turn, so it can be played card by card from the injected position
    if app.live {
        let rows = app.rows();
        let chosen = rows.get(app.row.min(rows.len().saturating_sub(1))).filter(|r| r.tag != "actual").map(|r| r.line.clone()).or_else(|| app.review().and_then(|r| r.ranked.iter().find(|l| !l.actual).cloned()));
        if let Some(l) = chosen {
            let text = line_as_plan(app, &l);
            let v = app.plan_texts.entry(turn).or_default();
            if !v.contains(&text) {
                v.insert(0, text);
            }
        }
    }
    let capture: PathBuf = match (app.live, &app.capture) {
        (true, Some(c)) => c.clone(),
        (true, None) => {
            app.popup = Some(("inject".into(), vec!["no capture yet: the live screen copies every save the app writes from the moment it is up, and none has landed".into()]));
            return;
        }
        (false, _) if app.is_capture => app.path.clone(),
        (false, _) => {
            app.popup = Some(("inject".into(), vec!["this game is a bare EndGame log: no checkpoints to load".into(), "si import <EndGame log> builds a live-<id>/ capture from the saves the app still has; the recorder keeps the full set".into()]));
            return;
        }
    };
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            app.popup = Some(("inject".into(), vec![format!("cannot find the si binary: {e}")]));
            return;
        }
    };
    let path = capture;
    let tx = app.tx.clone();
    let join_from = Arc::clone(&app.join_from);
    let live = app.live;
    app.status = format!("injecting T{turn}'s start into the running game: the game window comes to the front, keep it there until it reloads (up to 90 s). In a game it reloads in place with the app's journal reopened; at the main menu, Continue opens it");
    std::thread::spawn(move || {
        // the save inject will take: the earliest start of that turn on the path the game
        // took; the journal that restarts at the load is seeded from it
        if live {
            let chosen = crate::checkpoints_of(&path).ok().and_then(|all| all.into_iter().find(|c| c.on_final_path && crate::checkpoint_matches(c, Some(turn as i64), Some("SpiritPhase"))).map(|c| c.path));
            match chosen {
                Some(p) => {
                    if let Ok(mut j) = join_from.lock() {
                        *j = Some(p);
                    }
                }
                None => {
                    let _ = tx.send(Msg::Inject(vec![format!("the capture holds no start-of-turn save for T{turn}: the app keeps only the last turn's decision saves, and the screen was not up when T{turn} began")]));
                    return;
                }
            }
        }
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
        let title = if app.is_open_tab() { " plan (G2 e4@C4 | CS>C3 FCD>C4 on the Growth tab, plays only after it; Enter scores it, Esc cancels) " } else { " try a line (SLSF>C8 MI; Enter scores it, Esc cancels) " };
        let p = Paragraph::new(format!("{text}_")).block(Block::default().borders(Borders::ALL).title(title).border_style(Style::default().fg(ACCENT)));
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
        let r = centered(area, 96, 23);
        f.render_widget(Clear, r);
        let text = "h / l, arrows   previous / next turn      1-9  jump to a turn\nj / k           move along the rows\nEnter, Space    open or close the row's group   e / c  open or close every group\nTab             Turn <-> Pacing\nt               type a line in notation and score it (a try row; a plan on the open tab)\nd               delete the plan or try row under the cursor\ni               load this turn's start into the running game (si inject)\n?               this key        q  quit\n\n--live: the Steam folder is polled; the turn being played is the open tab (T4 Fast),\nPLAN scores what you type beside doing nothing, never the engine's own lines.\n\nTurn: LINES is the tree (a growth, its play sets, each set's targetings; a group\nrow carries its best member's numbers) with the READING under it. POSITION is the\nisland at the Fast phase: the steps reaching each land, the selected row's plays\nthere (line), what the steps did (came). INVADER PHASE: this turn's cards and\nwhat came. SPIRIT, GROWTH, HAND: what you had; the row under the cursor lights\nits innate tiers, cards and lands. LINE and FORECAST along the bottom.\nPacing: Blight, Invaders, Fear, the spirit's track and the review, by turn.";
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
        Span::styled(if app.live { " LIVE " } else { " STUDY " }, header_style()),
        Span::raw(format!(" {spirit} | board {} | {adversary} | {result} | deck {deck} | {}", m.boards.join(" "), if app.is_capture { "capture" } else { "bare log" })),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let keys = " h/l turn  j/k row  Enter open/close  e/c all  Tab pacing  t try/plan  d delete  i inject  ? keys  q quit ";
    let mut status = if app.running { format!("{}  ", app.status) } else { app.status.clone() };
    if app.live {
        status = if status.trim().is_empty() { app.source_words.clone() } else { format!("{} | {}", app.source_words, status.trim_end()) };
    }
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
