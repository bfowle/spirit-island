//! The Turn screen: what every panel reads and the nine panel builders.

use super::*;


/// What a turn's tab is about: a closed turn carries its review; an open one (the turn
/// being played) the turn as it stands and the plans scored from it.
pub(super) enum TurnKind<'r> {
    Closed(&'r TurnReview),
}

/// What every panel of the Turn screen reads. Built once per frame by `App::turn_ctx`;
/// no builder takes the `App`.
pub(super) struct TurnCtx<'r, 'd> {
    pub rec: &'r GameRecord,
    pub defs: &'d Definitions,
    pub codes: &'r RefCell<si_record::notation::Codes<'d>>,
    pub turn: u32,
    pub spirit: String,
    pub kind: TurnKind<'r>,
    pub t: Option<&'r si_record::TurnRecord>,
    pub s: Option<&'r si_record::SpiritTurn>,
    pub panel: Option<&'r si_model::spirit::SpiritPanel>,
    pub island: si_rules::Island,
    pub rows: Vec<RowView>,
    pub row: usize,
    pub sel: Option<RowView>,
    /// The Blight card's pool at this turn (left, flipped), when a capture kept the save.
    pub pool: Option<(i64, bool)>,
}

impl<'r, 'd> TurnCtx<'r, 'd> {
    pub fn code(&self, id: &str) -> String {
        self.codes.borrow_mut().code(id)
    }
    pub fn review(&self) -> Option<&'r TurnReview> {
        match self.kind {
            TurnKind::Closed(r) => Some(r),
        }
    }
    pub fn hand(&self) -> Vec<String> {
        self.review().map(|r| r.hand.clone()).unwrap_or_default()
    }
    pub fn energy(&self) -> i64 {
        self.review().map(|r| r.energy).unwrap_or(0)
    }
    pub fn plays(&self) -> i64 {
        self.review().map(|r| r.plays).unwrap_or(0)
    }
    pub fn forecast(&self) -> &'r [si_hindsight::deck::TurnForecast] {
        self.review().map(|r| r.forecast.as_slice()).unwrap_or(&[])
    }
    pub fn offered(&self) -> &'r [String] {
        self.review().map(|r| r.offered.as_slice()).unwrap_or(&[])
    }
    pub fn actual(&self) -> Option<&'r si_hindsight::Line> {
        self.review().map(|r| &r.actual)
    }
    pub fn notes(&self) -> &'r [String] {
        self.review().map(|r| r.notes.as_slice()).unwrap_or(&[])
    }
    pub fn choices(&self) -> &'r [si_hindsight::ChoiceReview] {
        self.review().map(|r| r.choices.as_slice()).unwrap_or(&[])
    }
    pub fn terror(&self) -> u8 {
        self.review().map(|r| r.sample.terror).unwrap_or(1)
    }
    pub fn best_growth(&self) -> &'r [String] {
        self.review().map(|r| r.best_growth.as_slice()).unwrap_or(&[])
    }
}

impl<'r, 'd> TurnCtx<'r, 'd> {
    /// The lands the selected row's plays target and its growth places presence in, each
    /// with the codes aimed there (`SLSF`, `GP2@`, `+Pr`).
    fn line_marks(&self, code: &dyn Fn(&str) -> String) -> BTreeMap<LandId, Vec<String>> {
        let mut marks: BTreeMap<LandId, Vec<String>> = BTreeMap::new();
        let Some(rv) = &self.sel else { return marks };
        for g in &rv.line.growth {
            if let Some((_, land)) = g.split_once('@') {
                if let Ok(id) = land.parse::<LandId>() {
                    marks.entry(id).or_default().push("+Pr".into());
                }
            }
        }
        for p in rv.line.plays.iter().chain(rv.line.outcome.slow.iter().filter(|p| p.target.is_some())) {
            if let Some(l) = &p.target {
                let word = if p.places { format!("{}@", code(&p.power)) } else { code(&p.power) };
                let e = marks.entry(l.clone()).or_default();
                if !e.contains(&word) {
                    e.push(word);
                }
            }
        }
        marks
    }

    /// This turn's visits to a land, in order.
    fn visits(&self, land: &LandId) -> Vec<&'r si_record::InvaderVisit> {
        let turn = self.turn;
        self.rec.lands.get(land).map(|ll| ll.visits.iter().filter(|v| v.turn == turn).collect()).unwrap_or_default()
    }

    /// The lands the step's cards act in this turn: where the record shows a visit, else
    /// where the card's terrain matches and Invaders stand.
    fn step_lands(&self, cards: &[String], step: InvaderStep) -> Vec<(LandId, String)> {
        let mut out: Vec<(LandId, String)> = Vec::new();
        for l in self.island.lands.values() {
            let visited: Vec<&si_record::InvaderVisit> = self.visits(&l.id).into_iter().filter(|v| v.step == Some(step) && cards.iter().any(|c| v.card.as_deref() == Some(c) || v.card.is_none())).collect();
            if !visited.is_empty() {
                out.push((l.id.clone(), visited.iter().map(|v| visit_words(v)).collect::<Vec<_>>().join(" ")));
                continue;
            }
            let matches = cards.iter().filter_map(|c| self.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|t| c.matches(t, l.coastal)));
            if matches && l.c.invaders() > 0 && step != InvaderStep::Explore {
                out.push((l.id.clone(), "-".into()));
            }
        }
        out
    }

    /// The selected row's elements this turn.
    fn sel_elements(&self) -> Option<ElementSet> {
        self.sel.as_ref().map(|rv| parse_short_elements(&rv.line.elements))
    }
}

pub(super) fn draw_turn(f: &mut Frame, app: &App, area: Rect) {
    let Some(ctx) = app.turn_ctx() else {
        let msg = if app.running { "reviewing this turn..." } else { "no review for this turn (no Fast-phase state in the record)" };
        f.render_widget(Paragraph::new(msg).style(dim()), area);
        return;
    };
    draw_turn_ctx(f, &ctx, area);
}

/// The nine panels laid out for the width: wide, the decision down the left, the island
/// down the right, the spirit's resources across the bottom; narrow, the same in rows.
fn draw_turn_ctx(f: &mut Frame, ctx: &TurnCtx, area: Rect) {
    let body = area;
    let h = body.height;
    if body.width >= 150 {
        // wide: the decision down the left (lines, reading, the selected row), the island
        // down the right (position, this turn's cards, the deck ahead), the spirit's
        // resources across the bottom at the height they need
        let left_w = body.width * 58 / 100;
        let right_w = body.width - left_w;
        // the hand needs the most width (title, target, the eight element counts)
        let sw = body.width * 30 / 100;
        let gw = body.width * 26 / 100;
        let hw = body.width - sw - gw;
        let spirit = spirit_panel(ctx, sw);
        let growth = growth_panel(ctx, gw);
        let hand = hand_panel(ctx, hw);
        let mid_h = spirit.height().max(growth.height()).max(hand.height()).clamp(5, (h * 40 / 100).max(5));
        let [top, bottom] = Layout::vertical([Constraint::Min(10), Constraint::Length(mid_h)]).areas(body);
        let [left, right] = Layout::horizontal([Constraint::Length(left_w), Constraint::Length(right_w)]).areas(top);
        let reading = reading_panel(ctx, left_w);
        let line = line_panel(ctx, left_w);
        let [lines_a, reading_a, line_a] = Layout::vertical([Constraint::Min(6), Constraint::Length(reading.height().min(top.height / 4)), Constraint::Length(line.height().min(top.height / 3))]).areas(left);
        let position = position_panel(ctx, right_w);
        let invaders = invaders_panel(ctx, right_w);
        let forecast = forecast_panel(ctx, right_w);
        let [pos_a, inv_a, fc_a] = Layout::vertical([Constraint::Length(position.height().min(top.height / 2)), Constraint::Length(invaders.height().min(top.height * 45 / 100)), Constraint::Min(3)]).areas(right);
        let [spirit_a, growth_a, hand_a] = Layout::horizontal([Constraint::Length(sw), Constraint::Length(gw), Constraint::Min(10)]).areas(bottom);
        lines_panel(ctx, left_w).render(f, lines_a);
        reading.render(f, reading_a);
        line.render(f, line_a);
        position.render(f, pos_a);
        invaders.render(f, inv_a);
        forecast.render(f, fc_a);
        spirit.render(f, spirit_a);
        growth.render(f, growth_a);
        hand.render(f, hand_a);
    } else {
        // narrow: the same panels in rows, the lines first
        let half = body.width / 2;
        let reading = reading_panel(ctx, half);
        let line = line_panel(ctx, body.width - half);
        let position = position_panel(ctx, half);
        let invaders = invaders_panel(ctx, body.width - half);
        let third = body.width / 3;
        let spirit = spirit_panel(ctx, third);
        let growth = growth_panel(ctx, third);
        let hand = hand_panel(ctx, body.width - 2 * third);
        let forecast = forecast_panel(ctx, body.width);
        // the lines keep at least a third of the height; the other rows share the rest
        let lines_min = (h * 35 / 100).max(8);
        let mut hs = [reading.height().max(line.height()), position.height().max(invaders.height()), spirit.height().max(growth.height()).max(hand.height()), forecast.height()];
        let total: u16 = hs.iter().sum();
        let avail = h.saturating_sub(lines_min);
        if total > avail && total > 0 {
            for x in hs.iter_mut() {
                *x = (*x as u32 * avail as u32 / total as u32) as u16;
            }
        }
        let [r1, r2, r3, r4, r5] = Layout::vertical([Constraint::Min(6), Constraint::Length(hs[0]), Constraint::Length(hs[1]), Constraint::Length(hs[2]), Constraint::Length(hs[3])]).areas(body);
        let [reading_a, line_a] = Layout::horizontal([Constraint::Length(half), Constraint::Min(10)]).areas(r2);
        let [pos_a, inv_a] = Layout::horizontal([Constraint::Length(half), Constraint::Min(10)]).areas(r3);
        let [spirit_a, growth_a, hand_a] = Layout::horizontal([Constraint::Length(third), Constraint::Length(third), Constraint::Min(10)]).areas(r4);
        lines_panel(ctx, body.width).render(f, r1);
        reading.render(f, reading_a);
        line.render(f, line_a);
        position.render(f, pos_a);
        invaders.render(f, inv_a);
        spirit.render(f, spirit_a);
        growth.render(f, growth_a);
        hand.render(f, hand_a);
        forecast.render(f, r5);
    }
}

/// The lines table: the actual line, the alternatives as a tree, a typed line.
pub(super) fn lines_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let Some(review) = ctx.review() else { return Panel::text("LINES", vec![]) };
    let rows = &ctx.rows;
    let inner = width.saturating_sub(2) as usize;
    let pool = ctx.pool;
    let tiers_w = (review.actual.tiers.iter().map(|(id, _, of)| ctx.code(id).len() + 3 + of.to_string().len()).sum::<usize>() + review.actual.tiers.len().saturating_sub(1)).max(8);
    let show_tiers = inner >= 60 + tiers_w;
    // tag, growth, cards, innates, informed, hindsight, conf, [tiers], produced
    let mut fixed: Vec<Option<usize>> = vec![Some(6), None, None, None, Some(9), Some(9), Some(5)];
    let mut weights: Vec<usize> = vec![0, 24, 24, 14, 0, 0, 0];
    let mut headers = vec!["", "growth", "cards", "innates", "informed", "hindsight", "conf"];
    if show_tiers {
        fixed.push(Some(tiers_w));
        weights.push(0);
        headers.push("tiers");
    }
    fixed.push(None);
    weights.push(38);
    headers.push("produced");
    let widths = share_widths(inner, &fixed, &weights, 10);
    let text_col = |i: usize| fixed.get(i).is_some_and(|f| f.is_none()) || (show_tiers && i == 7);
    let mut table_rows: Vec<Row> = Vec::new();
    let mut rows_h = 0u16;
    for rv in rows {
        let l = &rv.line;
        let tag = rv.tag.as_str();
        let code = |id: &str| ctx.code(id);
        let fold = |n: usize, what: &str| if n > 1 { format!(" [{n} {what}s]") } else { String::new() };
        let more = |open: bool, n: usize| if !open && n > 1 { " >" } else { "" };
        let (growth, cards, innates) = match rv.level {
            0 if tag == "try" => ("(growth as played)".to_string(), l.cards_notation(&code), l.innates_notation(&code)),
            0 if tag == "actual" => (l.growth_notation(&code), l.cards_notation(&code), l.innates_notation(&code)),
            0 => {
                let g = l.growth_notation(&code);
                let g = if l.growth_actual { format!("{g} (as played)") } else { g };
                (format!("{g}{}{}", fold(rv.children, "play set"), more(rv.open, rv.children)), power_names(l, false, &code), power_names(l, true, &code))
            }
            1 => (String::new(), format!("{}{}{}", power_names(l, false, &code), fold(rv.children, "targeting"), more(rv.open, rv.children)), power_names(l, true, &code)),
            _ => (String::new(), l.cards_notation(&code), l.innates_notation(&code)),
        };
        let conf = l.confidence.map(|c| format!("{:.0}%", c * 100.0)).unwrap_or_else(|| "-".into());
        let tiers: Vec<String> = l.tiers.iter().map(|(id, reached, of)| format!("{} {reached}/{of}", code(id))).collect();
        let mark = match pool {
            Some((p, true)) if p > 0 && l.outcome.blight as i64 >= p => " LOSES",
            Some((p, false)) if p > 0 && l.outcome.blight as i64 >= p => " FLIPS",
            _ => "",
        };
        let indent = "  ".repeat(rv.level as usize);
        let mut cells: Vec<String> = vec![format!("{indent}{tag}"), growth, format!("{indent}{cards}"), innates, format!("{:+.2}", l.informed), format!("{:+.2}", l.score), conf];
        if show_tiers {
            cells.push(tiers.join(" "));
        }
        cells.push(format!("{}{mark}", l.outcome.summary()));
        let style = match (tag, rv.level) {
            ("actual", _) => Style::default().add_modifier(Modifier::BOLD),
            ("best", _) => Style::default().fg(Color::Green),
            ("try", _) => Style::default().fg(Color::Yellow),
            (_, 1) => Style::default().fg(Color::Cyan),
            (_, 2) => dim(),
            _ => Style::default(),
        };
        let (row, rh) = wrapped_row(cells, &widths, &text_col, style);
        rows_h += rh;
        table_rows.push(row);
    }
    Panel {
        block: panel_block(&format!("LINES  {} play sets over {} growth position{}, ranked by informed", review.candidates, review.positions, if review.positions == 1 { "" } else { "s" })),
        body: PanelBody::Table { header: header_row(&headers), rows: table_rows, widths: widths.iter().map(|w| Constraint::Length(*w as u16)).collect(), rows_h, select: Some(ctx.row.min(rows.len().saturating_sub(1))) },
    }
}

/// The reading and its numbers.
pub(super) fn reading_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let Some(r) = ctx.review() else { return Panel::text("READING", vec![]) };
    let w = width.saturating_sub(2) as usize;
    let mut lines: Vec<TLine<'static>> = hang("verdict", dim(), &r.reading, Style::default(), w);
    let mut nums = format!("decision {:.1}, luck {:+.1}", r.decision, r.luck);
    if r.gap_plays > 0.05 || r.gap_growth > 0.05 {
        nums.push_str(&format!("; the gap: plays {:.1}, growth {:.1}", r.gap_plays, r.gap_growth));
    }
    if let Some(x) = r.hidden_effect {
        nums.push_str(&format!("; the hidden cards were worth {x:+.2} against {:+.2} expected", r.expected_exogenous));
    }
    lines.extend(hang("numbers", dim(), &nums, Style::default(), w));
    Panel::text("READING  the verdict on what was known", lines)
}

/// The island as the Fast phase began: one row per land with its pieces, the Invader
/// steps that reach it this turn, the selected row's plays aimed there, and what the
/// steps did there as recorded.
pub(super) fn position_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let inner = width.saturating_sub(2) as usize;
    let code = |id: &str| ctx.code(id);
    let marks = ctx.line_marks(&code);
    let (ravage, build, hi): (Vec<String>, Vec<String>, Vec<String>) = ctx.t.map(|t| (t.ravage.clone(), t.build.clone(), t.high_immigration.clone())).unwrap_or_default();
    let fc0 = ctx.forecast().iter().find(|fc| fc.turn == ctx.turn);
    let show_result = inner >= 68;
    // land, terr, inv, Dh, Bl, Pr, tok, steps, line, [result]
    let mut fixed: Vec<Option<usize>> = vec![Some(4), Some(4), Some(8), Some(2), Some(2), Some(2), Some(7), Some(9), None];
    let mut weights: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 40];
    let mut headers = vec!["land", "terr", "inv", "Dh", "Bl", "Pr", "tok", "steps", "line"];
    if show_result {
        fixed.push(None);
        weights.push(60);
        headers.push("result");
    }
    let widths = share_widths(inner, &fixed, &weights, 8);
    let mut rows: Vec<Row> = Vec::new();
    let mut rows_h = 0u16;
    for l in ctx.island.lands.values() {
        if l.terrain == Some(Terrain::Ocean) && l.c.presence + l.c.invaders() + l.c.dahan == 0 {
            continue;
        }
        let hits = |cards: &[String]| cards.iter().filter_map(|c| ctx.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|t| c.matches(t, l.coastal)));
        let mut steps: Vec<String> = Vec::new();
        if hits(&hi) {
            steps.push(if l.c.invaders() > 0 { "HI".into() } else { "hi".into() });
        }
        if hits(&ravage) {
            steps.push(if l.c.invaders() > 0 { "R".into() } else { "r".into() });
        }
        if hits(&build) {
            steps.push(if l.c.invaders() > 0 { "B".into() } else { "b".into() });
        }
        if let Some(p) = fc0.and_then(|fc| fc.lands.iter().find(|x| x.land == l.id)) {
            steps.push(format!("X{:.0}%", p.p * 100.0));
        }
        let line = marks.get(&l.id).map(|v| v.join(" ")).unwrap_or_default();
        let result: Vec<String> = ctx.visits(&l.id).iter().map(|v| format!("{}:{}", step_letter(v.step), visit_words(v))).collect();
        let terr = format!("{}{}", terrain_short(l.terrain), if l.coastal { " c" } else { "" });
        let n = |x: u32| if x > 0 { x.to_string() } else { String::new() };
        let mut cells: Vec<String> = vec![l.id.to_string(), terr, pieces_short(&l.c), n(l.c.dahan), n(l.c.blight), n(l.c.presence), tokens_short(&l.c), steps.join(" "), line];
        if show_result {
            cells.push(result.join(" "));
        }
        let style = if marks.contains_key(&l.id) {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if l.c.invaders() > 0 || l.c.blight > 0 {
            Style::default()
        } else {
            dim()
        };
        let (row, rh) = wrapped_row(cells, &widths, &|i| i >= 7, style);
        rows_h += rh;
        rows.push(row);
    }
    Panel { block: panel_block("POSITION  the island at the Fast phase"), body: PanelBody::Table { header: header_row(&headers), rows, widths: widths.iter().map(|w| Constraint::Length(*w as u16)).collect(), rows_h, select: None } }
}

/// This turn's Invader phase: the face-up cards and where they act, the face-down
/// Explore with its pool and what came, the Event and Fear cards with the choices they
/// asked, the Blight card, the board after.
pub(super) fn invaders_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let title = "INVADER PHASE  this turn's cards";
    let Some(t) = ctx.t else {
        return Panel::text(title, vec![TLine::from(Span::styled("no Invader phase in the record for this turn", dim()))]);
    };
    let mut lines: Vec<TLine<'static>> = Vec::new();
    let lands_words = |v: &[(LandId, String)]| -> String {
        if v.is_empty() { "no land".to_string() } else { v.iter().map(|(l, w)| if w == "-" { l.to_string() } else { format!("{l} {w}") }).collect::<Vec<_>>().join(", ") }
    };
    let clean = clean_engine_text;
    if !t.high_immigration.is_empty() {
        let v = ctx.step_lands(&t.high_immigration, InvaderStep::Build);
        lines.extend(hang("HI build", dim(), &format!("{}: {}", t.high_immigration.join(" "), lands_words(&v)), Style::default(), w));
    }
    if t.ravage.is_empty() {
        lines.extend(hang("Ravage", dim(), "no card in the slot yet", Style::default(), w));
    } else {
        let v = ctx.step_lands(&t.ravage, InvaderStep::Ravage);
        lines.extend(hang("Ravage", dim(), &format!("{}: {}", t.ravage.join(" "), lands_words(&v)), Style::default(), w));
    }
    if t.build.is_empty() {
        lines.extend(hang("Build", dim(), "no card in the slot yet", Style::default(), w));
    } else {
        let v = ctx.step_lands(&t.build, InvaderStep::Build);
        lines.extend(hang("Build", dim(), &format!("{}: {}", t.build.join(" "), lands_words(&v)), Style::default(), w));
    }
    // the Explore: face down at the choice, so its pool first and the card that came after
    let fc0 = ctx.forecast().iter().find(|fc| fc.turn == ctx.turn);
    let pool_words = match fc0 {
        Some(fc) if fc.cards.len() > 1 => format!("face down, one of {} Stage {} cards ({}); ", fc.cards.len(), fc.stage, fc.cards.join(" ")),
        Some(fc) => format!("the last Stage {} card, knowable; ", fc.stage),
        None => String::new(),
    };
    let v = ctx.step_lands(&t.explore, InvaderStep::Explore);
    lines.extend(hang("Explore", dim(), &format!("{pool_words}came {}: {}", if t.explore.is_empty() { "-".to_string() } else { t.explore.join(" ") }, lands_words(&v)), Style::default(), w));
    // Event and Fear: the cards, then every choice they asked with its verdict
    let choice_words = |kind: &str| -> Vec<String> {
        ctx.choices()
            .iter()
            .filter(|c| c.source_kind == kind)
            .map(|c| {
                let verdict = match c.gap {
                    Some(g) if g <= 0.01 => "best option".to_string(),
                    Some(g) if g <= 0.5 => format!("within {g:.2} of best"),
                    Some(g) => format!("better land open by {g:.2}"),
                    None => "not scored".to_string(),
                };
                format!("{} -> {} ({verdict})", spaced(&c.source), clean(&c.selected))
            })
            .collect()
    };
    let mut ev = if t.events.is_empty() { "face down; none resolved this turn".to_string() } else { format!("face down; came {}", t.events.iter().map(|e| spaced(e)).collect::<Vec<_>>().join(", ")) };
    let asked = choice_words("Event");
    if !asked.is_empty() {
        ev.push_str(&format!("; asked {}", asked.join("; ")));
    }
    lines.extend(hang("Event", dim(), &ev, Style::default(), w));
    let fear_titles: Vec<String> = t.fear_cards.iter().map(|id| ctx.defs.fear_cards.get(id).map(|c| c.title.clone()).unwrap_or_else(|| spaced(id))).collect();
    let mut fr = if fear_titles.is_empty() { format!("no card resolved; generated {}", t.fear_generated) } else { format!("face down; resolved {} at TL{}; generated {}", fear_titles.join(", "), ctx.terror(), t.fear_generated) };
    if t.fear_cards_earned > 0 {
        fr.push_str(&format!(", earned {} card{}", t.fear_cards_earned, if t.fear_cards_earned > 1 { "s" } else { "" }));
    }
    fr.push_str(&format!("; TL{} after", t.terror_after));
    let asked = choice_words("Fear");
    if !asked.is_empty() {
        fr.push_str(&format!("; asked {}", asked.join("; ")));
    }
    lines.extend(hang("Fear", dim(), &fr, Style::default(), w));
    let others: Vec<String> = ctx.choices().iter().filter(|c| c.source_kind != "Event" && c.source_kind != "Fear").map(|c| format!("{} {} -> {}", c.source_kind, spaced(&c.source), clean(&c.selected))).collect();
    if !others.is_empty() {
        lines.extend(hang("asked", dim(), &others.join("; "), Style::default(), w));
    }
    let pool = ctx.pool;
    let mut bl = match pool {
        Some((p, flipped)) => format!("{p} left on the card ({}); ", if flipped { "flipped" } else { "healthy" }),
        None => String::new(),
    };
    bl.push_str(&format!("added {} this turn{}", t.blight_added + t.blight_cascaded, if t.blight_cascaded > 0 { format!(" ({} cascaded)", t.blight_cascaded) } else { String::new() }));
    lines.extend(hang("Blight", dim(), &bl, Style::default(), w));
    let a = &t.board_after;
    lines.extend(hang("after", dim(), &format!("Bl{} on the island; {} in {} lands; Dahan {}; Fear {}", a.blight, if pieces_short(a).is_empty() { "no Invaders".to_string() } else { pieces_short(a) }, t.lands_with_invaders_after, a.dahan, t.fear_generated), Style::default(), w));
    Panel::text(title, lines)
}

/// The spirit at the Fast phase: Energy, plays, the track's elements, presence, and
/// each innate's tiers with the ones the selected row reaches lit.
pub(super) fn spirit_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let title = ctx.panel.map(|p| p.title.clone()).unwrap_or_else(|| ctx.spirit.clone());
    let mut lines: Vec<TLine<'static>> = Vec::new();
    let track = ctx.panel.zip(ctx.s).map(|(p, s)| p.track_state(&s.revealed_after));
    let income = track.as_ref().map(|t| format!(" (income {})", t.income)).unwrap_or_default();
    lines.extend(hang("Energy", dim(), &format!("{}{income}; plays {}; hand {} cards", ctx.energy(), ctx.plays(), ctx.hand().len()), Style::default(), w));
    if let Some(t) = &track {
        let mut spans = element_spans(&t.elements, true);
        spans.push(Span::raw(format!("; {} slots revealed", t.revealed)));
        lines.extend(hang_spans("track", dim(), spans, w));
    }
    let presence: Vec<String> = ctx.island.lands.values().filter(|l| l.c.presence > 0).map(|l| if l.c.presence > 1 { format!("{} x{}", l.id, l.c.presence) } else { l.id.to_string() }).collect();
    let total: u32 = ctx.island.lands.values().map(|l| l.c.presence).sum();
    lines.extend(hang("presence", dim(), &format!("{total} on the island: {}", if presence.is_empty() { "-".to_string() } else { presence.join(" ") }), Style::default(), w));
    if let Some(p) = ctx.panel {
        let have = ctx.sel_elements();
        let sel_tiers: Vec<(String, u8, u8)> = ctx.sel.as_ref().map(|rv| rv.line.tiers.clone()).unwrap_or_default();
        for inn in &p.innates {
            let code = ctx.code(&inn.id);
            let speed = match inn.speed {
                Speed::Fast => "Fast",
                Speed::Slow => "Slow",
            };
            let target = if inn.target.is_spirit() { "self".to_string() } else { target_words(&inn.target.0) };
            let reached = sel_tiers.iter().find(|(id, _, _)| *id == inn.id).map(|(_, n, of)| format!(" {n}/{of}")).unwrap_or_default();
            let mut spans: Vec<Span<'static>> = vec![Span::styled(format!("{code:<5} "), header_style()), Span::styled(format!("{speed} {} {target}  ", range_words(inn.range.as_ref())), dim())];
            for tier in inn.tiers.iter().filter(|t| !t.modifier) {
                // a reached tier keeps its element colors; an unreached one is dimmed
                let lit = have.as_ref().is_some_and(|h| h.satisfies(&tier.elements));
                spans.push(Span::styled("[", if lit { Style::default() } else { dim() }));
                for sp in element_spans(&tier.elements, false) {
                    spans.push(if lit { sp } else { Span::styled(sp.content, dim()) });
                }
                spans.push(Span::styled("] ", if lit { Style::default() } else { dim() }));
            }
            spans.push(Span::raw(reached));
            // wrap by hand: the first line keeps the spans, a long tier list continues under the code
            let total: usize = spans.iter().map(|s| s.content.chars().count()).sum();
            if total <= w {
                lines.push(TLine::from(spans));
            } else {
                let mut cur: Vec<Span<'static>> = Vec::new();
                let mut used = 0usize;
                for s in spans {
                    let n = s.content.chars().count();
                    if used + n > w && !cur.is_empty() {
                        lines.push(TLine::from(std::mem::take(&mut cur)));
                        cur.push(Span::raw(" ".repeat(6)));
                        used = 6;
                    }
                    used += n;
                    cur.push(s);
                }
                if !cur.is_empty() {
                    lines.push(TLine::from(cur));
                }
            }
        }
    }
    Panel::text(&format!("SPIRIT  {title}"), lines)
}

/// The growth groups the panel offers, the one taken and the one the best line takes,
/// the draft offered at the gain.
pub(super) fn growth_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let actual_growth: Vec<String> = ctx.actual().map(|a| a.growth.clone()).unwrap_or_default();
    let best_growth = ctx.best_growth();
    let w = width.saturating_sub(2) as usize;
    let code = |id: &str| ctx.code(id);
    let mut lines: Vec<TLine<'static>> = Vec::new();
    let taken: Vec<u8> = ctx.s.map(|s| s.growth_groups.clone()).unwrap_or_default();
    let group_of = |tokens: &[String]| -> Option<u8> { tokens.first().and_then(|g| g.strip_prefix('G')).and_then(|n| n.parse().ok()) };
    let rest_of = |tokens: &[String]| -> String {
        let rest: Vec<String> = tokens.iter().filter(|t| !(t.starts_with('G') && t[1..].chars().all(|c| c.is_ascii_digit()))).cloned().collect();
        si_hindsight::growth_words(&rest, &code)
    };
    let best_group = group_of(best_growth);
    if let Some(p) = ctx.panel {
        if p.growth.pick > 1 {
            lines.push(TLine::from(Span::styled(format!("pick {} of", p.growth.pick), dim())));
        }
        if let Some(a) = &p.growth.always {
            lines.extend(hang("always", dim(), &a.options.iter().map(growth_option_words).collect::<Vec<_>>().join(", "), Style::default(), w));
        }
        for g in &p.growth.groups {
            let words = g.options.iter().map(growth_option_words).collect::<Vec<_>>().join(", ");
            let is_taken = taken.contains(&g.number);
            let is_best = best_group == Some(g.number);
            let mut tail = String::new();
            if is_taken {
                tail.push_str(&format!(" <- taken: {}", rest_of(&actual_growth)));
            }
            if is_best && best_growth != actual_growth.as_slice() {
                tail.push_str(&format!(" <- best: {}", rest_of(best_growth)));
            } else if is_best && !is_taken {
                tail.push_str(" <- best");
            }
            let style = if is_taken { Style::default().add_modifier(Modifier::BOLD) } else if is_best { Style::default().fg(Color::Green) } else { Style::default() };
            lines.extend(hang(&format!("G{}", g.number), if is_best { Style::default().fg(Color::Green) } else { dim() }, &format!("{words}{tail}"), style, w));
        }
    } else {
        lines.extend(hang("growth", dim(), &si_hindsight::growth_words(&actual_growth, &code), Style::default(), w));
    }
    let offered = ctx.offered();
    if !offered.is_empty() {
        let gained: Vec<String> = ctx.s.map(|s| s.cards_gained.iter().map(|c| code(c)).collect()).unwrap_or_default();
        lines.extend(hang("offered", dim(), &format!("{}{}", offered.iter().map(|c| code(c)).collect::<Vec<_>>().join(" "), if gained.is_empty() { String::new() } else { format!("; took {}", gained.join(" ")) }), Style::default(), w));
    }
    if let Some(s) = ctx.s {
        if s.reclaimed {
            lines.extend(hang("reclaim", dim(), &if s.reclaimed_cards.is_empty() { "all".to_string() } else { s.reclaimed_cards.iter().map(|c| code(c)).collect::<Vec<_>>().join(" ") }, Style::default(), w));
        }
        if !s.cards_forgotten.is_empty() {
            lines.extend(hang("forgot", dim(), &s.cards_forgotten.iter().map(|c| code(c)).collect::<Vec<_>>().join(" "), Style::default(), w));
        }
    }
    Panel::text("GROWTH  the panel's groups", lines)
}

/// The hand at the Fast phase: each card with its cost, speed, range, target and
/// elements; `in` says whether the actual line, the selected row or both play it.
pub(super) fn hand_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let inner = width.saturating_sub(2) as usize;
    let hand = ctx.hand();
    let actual: BTreeSet<&str> = ctx.actual().map(|a| a.plays.as_slice()).unwrap_or(&[]).iter().filter(|p| !p.innate).map(|p| p.power.as_str()).collect();
    let sel: BTreeSet<String> = ctx.sel.as_ref().map(|rv| rv.line.plays.iter().filter(|p| !p.innate).map(|p| p.power.clone()).collect()).unwrap_or_default();
    // (name, fixed width, weight, wraps); a narrow panel drops the target, then range and elements
    let mut cols: Vec<(&str, Option<usize>, usize, bool)> = vec![("code", Some(5), 0, false), ("card", None, 55, true), ("cost", Some(4), 0, false), ("spd", Some(3), 0, false), ("range", Some(6), 0, false), ("target", None, 45, true), ("elements", Some(31), 0, false), ("in", Some(6), 0, false)];
    if inner < 95 {
        cols.retain(|c| c.0 != "target");
    }
    if inner < 70 {
        cols.retain(|c| c.0 != "range" && c.0 != "elements");
    }
    let headers: Vec<&str> = cols.iter().map(|c| c.0).collect();
    let fixed: Vec<Option<usize>> = cols.iter().map(|c| c.1).collect();
    let weights: Vec<usize> = cols.iter().map(|c| c.2).collect();
    let widths = share_widths(inner, &fixed, &weights, 8);
    let wraps: Vec<bool> = cols.iter().map(|c| c.3).collect();
    let mut rows: Vec<Row> = Vec::new();
    let mut rows_h = 0u16;
    for id in &hand {
        let code = ctx.code(id);
        let in_actual = actual.contains(id.as_str());
        let in_sel = sel.contains(id);
        let in_word = match (in_actual, in_sel) {
            (true, true) => "both",
            (true, false) => "actual",
            (false, true) => "row",
            _ => "",
        };
        let card = ctx.defs.cards.get(id);
        let cell = |name: &str| -> String {
            match (name, card) {
                ("code", _) => code.clone(),
                ("in", _) => in_word.to_string(),
                ("card", Some(c)) => c.title.clone(),
                ("card", None) => spaced(id),
                ("cost", Some(c)) => c.cost.to_string(),
                ("spd", Some(c)) => if c.speed == Speed::Fast { "F".into() } else { "S".into() },
                ("range", Some(c)) => range_words(c.range.as_ref()),
                ("target", Some(c)) => if c.target.is_spirit() { "a Spirit".to_string() } else { target_words(&c.target.0) },
                _ => String::new(),
            }
        };
        let style = if in_sel { Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD) } else if in_actual { Style::default() } else { dim() };
        // text columns wrap; the elements column is the eight colored counts
        let mut texts: Vec<Text<'static>> = Vec::new();
        let mut rh = 1u16;
        for (i, h) in headers.iter().enumerate() {
            let t = match (*h, card) {
                ("elements", Some(c)) => Text::from(TLine::from(element_spans(&c.elements, true))),
                ("elements", None) => Text::from(""),
                _ => {
                    let v = cell(h);
                    if wraps.get(i).copied().unwrap_or(false) {
                        let lines = wrap_text(&v, widths.get(i).copied().unwrap_or(8));
                        rh = rh.max(lines.len() as u16);
                        Text::from(lines.join("\n"))
                    } else {
                        Text::from(v)
                    }
                }
            };
            texts.push(t);
        }
        rows_h += rh;
        rows.push(Row::new(texts.into_iter().map(Cell::new).collect::<Vec<_>>()).height(rh).style(style));
    }
    Panel { block: panel_block("HAND  in = who plays it (actual, row, both)"), body: PanelBody::Table { header: header_row(&headers), rows, widths: widths.iter().map(|w| Constraint::Length(*w as u16)).collect(), rows_h, select: None } }
}

/// The selected row in words: notation, growth, elements and tiers, what it produced,
/// its scores, the Slow lands, notes.
pub(super) fn line_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let code = |id: &str| ctx.code(id);
    let mut sel: Vec<TLine<'static>> = Vec::new();
    if let Some(rv) = &ctx.sel {
        let l = &rv.line;
        let tag = match (rv.level, rv.children) {
            (0, n) if n > 0 => format!("{} (best of {n} play sets)", rv.tag),
            (1, n) => format!("play set (best of {n} targetings)"),
            (2, _) => "targeting".to_string(),
            _ => rv.tag.clone(),
        };
        sel.extend(hang(&tag, header_style(), &l.notation(&code), Style::default(), w));
        sel.extend(hang("growth", dim(), &l.growth_notation(&code), Style::default(), w));
        let tiers: Vec<String> = l.tiers.iter().map(|(id, reached, of)| format!("{} {reached}/{of}", code(id))).collect();
        let mut spans = element_spans(&parse_short_elements(&l.elements), true);
        if !tiers.is_empty() {
            spans.push(Span::raw(format!("; tiers {}", tiers.join(" "))));
        }
        sel.extend(hang_spans("elements", dim(), spans, w));
        sel.extend(hang("produced", dim(), &l.outcome.words(true), Style::default(), w));
        sel.extend(hang("scores", dim(), &format!("informed {:+.2}, hindsight {:+.2}, luck {:+.2}{}", l.informed, l.score, l.luck, l.confidence.map(|c| format!(", conf {:.0}%", c * 100.0)).unwrap_or_default()), Style::default(), w));
        let slow_words = |v: &[si_hindsight::Play]| v.iter().map(|p| match &p.target { Some(t) => format!("{}>{t}", code(&p.power)), None => code(&p.power) }).collect::<Vec<_>>().join(" ");
        if l.outcome.slow.iter().any(|p| p.target.is_some()) {
            sel.extend(hang("Slow lands", dim(), &format!("{} (chosen after the Invader steps)", slow_words(&l.outcome.slow)), Style::default(), w));
        }
        if l.slow_gap > 0.25 {
            sel.extend(hang("Slow gap", dim(), &format!("{:.1} better after the Explore: {}", l.slow_gap, slow_words(&l.slow_best)), Style::default(), w));
        }
        // a note once, however many lands or effects raised it
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for n in l.outcome.notes.iter().chain(ctx.notes().iter()) {
            if seen.insert(n.as_str()) {
                sel.extend(hang("note", dim(), n, dim(), w));
            }
        }
    }
    Panel::text("LINE  the selected row", sel)
}

/// The Invader deck from this Fast phase: each coming turn's stage, its pool, and every
/// land's chance of an Explore, re-hits apart from fresh lands.
pub(super) fn forecast_panel(ctx: &TurnCtx, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let mut lines: Vec<TLine<'static>> = Vec::new();
    for fc in ctx.forecast() {
        let rehit: Vec<String> = fc.lands.iter().filter(|l| l.rehit).map(|l| format!("{} {:.0}%", l.land, l.p * 100.0)).collect();
        let fresh: Vec<String> = fc.lands.iter().filter(|l| !l.rehit).map(|l| format!("{} {:.0}%", l.land, l.p * 100.0)).collect();
        lines.extend(hang(&format!("T{}", fc.turn), header_style(), &format!("Stage {}, {} card{}: {:.1} re-hits and {:.1} fresh lands expected", fc.stage, fc.cards.len(), if fc.cards.len() == 1 { "" } else { "s" }, fc.rehits, fc.fresh), Style::default(), w));
        lines.extend(hang("  re-hit", dim(), &if rehit.is_empty() { "-".to_string() } else { rehit.join("  ") }, Style::default(), w));
        lines.extend(hang("  fresh", dim(), &if fresh.is_empty() { "-".to_string() } else { fresh.join("  ") }, Style::default(), w));
    }
    if lines.is_empty() {
        lines.push(TLine::from(Span::styled("no deck forecast for this turn", dim())));
    }
    Panel::text("FORECAST  each land's chance of an Explore; re-hit = already holds Invaders or Blight", lines)
}

