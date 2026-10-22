//! Text shapes every panel shares: wrapping, hanging indents, shared column widths,
//! the panel type, element spans, the short forms of pieces, steps and terrain.

use super::*;

pub(super) const ACCENT: Color = Color::Cyan;

pub(super) fn header_style() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

/// Word-wrap to a width; words longer than the width are cut.
pub(super) fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if cur.is_empty() {
            cur = word.to_string();
        } else if cur.chars().count() + 1 + word.chars().count() <= width {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        }
        while cur.chars().count() > width {
            let head: String = cur.chars().take(width).collect();
            cur = cur.chars().skip(width).collect();
            lines.push(head);
        }
    }
    if !cur.is_empty() || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

/// with wrapped text or a table whose row heights are summed.
pub(super) struct Panel {
    pub(super) block: Block<'static>,
    pub(super) body: PanelBody,
}

pub(super) enum PanelBody {
    Text(Vec<TLine<'static>>),
    Table { header: Row<'static>, rows: Vec<Row<'static>>, widths: Vec<Constraint>, rows_h: u16, select: Option<usize> },
}

impl Panel {
    pub(super) fn text(title: &str, lines: Vec<TLine<'static>>) -> Panel {
        Panel { block: panel_block(title), body: PanelBody::Text(lines) }
    }

    /// Rows plus header plus the two border rows.
    pub(super) fn height(&self) -> u16 {
        match &self.body {
            PanelBody::Text(l) => l.len() as u16 + 2,
            PanelBody::Table { rows_h, .. } => rows_h + 3,
        }
    }

    pub(super) fn render(self, f: &mut Frame, area: Rect) {
        if area.height == 0 || area.width == 0 {
            return;
        }
        match self.body {
            PanelBody::Text(lines) => f.render_widget(Paragraph::new(lines).block(self.block), area),
            PanelBody::Table { header, rows, widths, select, .. } => {
                let table = Table::new(rows, widths).header(header).block(self.block).column_spacing(1).row_highlight_style(Style::default().add_modifier(Modifier::REVERSED)).highlight_symbol("");
                match select {
                    Some(i) => {
                        let mut state = TableState::default();
                        state.select(Some(i));
                        f.render_stateful_widget(table, area, &mut state);
                    }
                    None => f.render_widget(table, area),
                }
            }
        }
    }
}

pub(super) fn panel_block(title: &str) -> Block<'static> {
    Block::bordered().border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)).title(Span::styled(format!(" {title} "), header_style()))
}

pub(super) fn dim() -> Style {
    Style::default().fg(Color::DarkGray)
}

pub(super) fn header_row(names: &[&str]) -> Row<'static> {
    Row::new(names.iter().map(|h| Cell::new(h.to_string())).collect::<Vec<_>>()).style(header_style()).height(1)
}

/// `ThrivingTrade` -> `Thriving Trade`.
pub(super) fn spaced(id: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for c in id.chars() {
        if c.is_uppercase() && prev_lower {
            out.push(' ');
        }
        prev_lower = c.is_lowercase() || c.is_ascii_digit();
        out.push(c);
    }
    out
}

/// Engine text without its markup: `Defend 1 <nobr>per {Dahan}</nobr>` -> `Defend 1 per Dahan`.
pub(super) fn clean_engine_text(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            '{' | '}' => {}
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

/// An engine target kind in words: `LandWithDahan` -> `land with Dahan`; the game's
/// names keep their capitals.
pub(super) fn target_words(kind: &str) -> String {
    const NAMES: [&str; 22] = ["Dahan", "Invaders", "Invader", "Town", "City", "Blight", "Spirit", "Presence", "Explorer", "Beast", "Beasts", "Wilds", "Disease", "Strife", "Badlands", "Ocean", "Jungle", "Mountain", "Sands", "Wetland", "Sacred", "Site"];
    spaced(kind).split(' ').map(|w| if NAMES.contains(&w) { w.to_string() } else { w.to_lowercase() }).collect::<Vec<_>>().join(" ")
}

pub(super) fn element_abbr(e: Element) -> &'static str {
    match e {
        Element::Sun => "Su",
        Element::Moon => "Mo",
        Element::Fire => "Fi",
        Element::Air => "Ai",
        Element::Water => "Wa",
        Element::Earth => "Ea",
        Element::Plant => "Pl",
        Element::Animal => "An",
    }
}

/// The element's color, the one the printed report uses (`color.rs`).
pub(super) fn element_color(e: Element) -> Color {
    let i = Element::ALL.iter().position(|x| *x == e).unwrap_or(0);
    Color::Indexed(crate::color::ELEMENT_COLORS[i])
}

/// The eight elements in printed order (Sun Moon Fire Air Water Earth Plant Animal), each
/// as its two letters and count, colored; a zero count dimmed. `all`: every element,
/// zeros included; else only the ones present (an innate's threshold).
pub(super) fn element_spans(set: &ElementSet, all: bool) -> Vec<Span<'static>> {
    let mut out: Vec<Span<'static>> = Vec::new();
    for e in Element::ALL {
        let n = set.count(e);
        if n == 0 && !all {
            continue;
        }
        if !out.is_empty() {
            out.push(Span::raw(" "));
        }
        let style = if n == 0 { dim() } else { Style::default().fg(element_color(e)).add_modifier(Modifier::BOLD) };
        out.push(Span::styled(format!("{}{n}", element_abbr(e)), style));
    }
    if out.is_empty() {
        out.push(Span::styled("-", dim()));
    }
    out
}

/// A label and styled spans on one line (a second line under the indent when they overflow).
pub(super) fn hang_spans(label: &str, label_style: Style, spans: Vec<Span<'static>>, width: usize) -> Vec<TLine<'static>> {
    let lw = label.chars().count().max(8) + 1;
    let avail = width.saturating_sub(lw).max(8);
    let mut lines: Vec<TLine<'static>> = Vec::new();
    let mut cur: Vec<Span<'static>> = vec![Span::styled(format!("{label:<w$}", w = lw), label_style)];
    let mut used = 0usize;
    for sp in spans {
        let n = sp.content.chars().count();
        if used + n > avail && used > 0 {
            lines.push(TLine::from(std::mem::take(&mut cur)));
            cur.push(Span::raw(" ".repeat(lw)));
            used = 0;
        }
        used += n;
        cur.push(sp);
    }
    lines.push(TLine::from(cur));
    lines
}

/// A line's `S1 M0 F3 A1 W0 E1 P0 A2` back into a set (by position, so the two A's resolve).
pub(super) fn parse_short_elements(s: &str) -> ElementSet {
    let mut set = ElementSet::new();
    for (tok, e) in s.split_whitespace().zip(Element::ALL.iter()) {
        let n: u8 = tok.trim_start_matches(|c: char| c.is_ascii_alphabetic()).parse().unwrap_or(0);
        set.add(*e, n);
    }
    set
}

pub(super) fn terrain_short(t: Option<Terrain>) -> &'static str {
    match t {
        Some(Terrain::Ocean) => "Oc",
        Some(Terrain::Jungle) => "Ju",
        Some(Terrain::Mountain) => "Mo",
        Some(Terrain::Sands) => "Sa",
        Some(Terrain::Wetland) => "We",
        None => "?",
    }
}

pub(super) fn range_words(r: Option<&si_model::power::Range>) -> String {
    match r {
        None => "-".into(),
        Some(r) => {
            let mut s = format!("r{}", r.distance);
            if r.from_sacred_site {
                s.push_str(" SS");
            }
            if !r.origin_terrains.is_empty() {
                s.push_str(&format!(" from {}", r.origin_terrains.iter().map(|t| terrain_short(Some(*t))).collect::<Vec<_>>().join("/")));
            }
            s
        }
    }
}

/// One growth option in the words the tempo report uses: `+presence r1 with Dahan`.
pub(super) fn growth_option_words(o: &GrowthOptionDef) -> String {
    if o.kind == "Choice" && !o.choices.is_empty() {
        return o.choices.iter().map(growth_option_words).collect::<Vec<_>>().join(" or ");
    }
    let y = o.yield_();
    let mut p: Vec<String> = Vec::new();
    if let Some(r) = y.reclaim {
        p.push(match r {
            Reclaim::All => "Reclaim all".into(),
            Reclaim::Half => "Reclaim half".into(),
            Reclaim::One => "Reclaim one".into(),
        });
    }
    if y.cards > 0 {
        p.push(format!("+{} card{}", y.cards, if y.cards > 1 { "s" } else { "" }));
    }
    if y.energy != 0 {
        p.push(format!("{:+} Energy", y.energy));
    }
    if y.plays_this_turn > 0 {
        p.push(format!("+{} play", y.plays_this_turn));
    }
    if y.presence > 0 {
        let place = match &y.placement {
            Some(Placement::Any { range }) => format!("r{range}"),
            Some(Placement::Terrains { range, terrains }) => format!("r{range} {}", terrains.iter().map(|t| t.name()).collect::<Vec<_>>().join("/")),
            Some(Placement::WithDahan { range }) => format!("r{range} with Dahan"),
            Some(Placement::WithInvaders { range }) => format!("r{range} with Invaders"),
            Some(Placement::WithTownCityOrBlight { range }) => format!("r{range} with Town, City or Blight"),
            Some(Placement::NoBlight { range }) => format!("r{range} no Blight"),
            Some(Placement::Inland { range }) => format!("r{range} inland"),
            Some(Placement::Coastal { range }) => format!("r{range} coastal"),
            Some(Placement::Ocean) => "ocean".into(),
            Some(Placement::Other { range, kind }) => format!("r{range} {}", target_words(kind)),
            None => String::new(),
        };
        p.push(format!("+presence {place}").trim_end().to_string());
    }
    if !y.elements.is_empty() {
        p.push(format!("+{}", y.elements.iter().map(|e| element_abbr(*e)).collect::<Vec<_>>().join(" ")));
    }
    if y.moves_presence > 0 {
        p.push("move presence".into());
    }
    for o in &y.other {
        p.push(target_words(o));
    }
    if p.is_empty() { target_words(&o.kind) } else { p.join(", ") }
}

pub(super) fn piece_letter(k: PieceKind) -> &'static str {
    match k {
        PieceKind::Explorer => "E",
        PieceKind::Town => "T",
        PieceKind::City => "C",
        PieceKind::Dahan => "D",
    }
}

/// What one Invader step did in one land, without the step letter: `Bl1 Dh2 x1 +T`.
pub(super) fn visit_words(v: &si_record::InvaderVisit) -> String {
    let mut p: Vec<String> = Vec::new();
    for k in &v.added {
        p.push(format!("+{}", piece_letter(*k)));
    }
    if v.blight + v.cascade > 0 {
        p.push(format!("Bl{}", v.blight + v.cascade));
    }
    if v.dahan_lost > 0 {
        p.push(format!("Dh{}", v.dahan_lost));
    }
    if v.invaders_destroyed > 0 {
        p.push(format!("x{}", v.invaders_destroyed));
    }
    if v.defend > 0 && v.step == Some(InvaderStep::Ravage) {
        p.push(format!("Df{}", v.defend));
    }
    if p.is_empty() { "-".into() } else { p.join(" ") }
}

pub(super) fn step_letter(s: Option<InvaderStep>) -> &'static str {
    match s {
        Some(InvaderStep::Ravage) => "R",
        Some(InvaderStep::Build) => "B",
        Some(InvaderStep::Explore) => "X",
        None => "?",
    }
}

pub(super) fn pieces_short(c: &si_record::Counts) -> String {
    let v: Vec<String> = [(c.explorers, "E"), (c.towns, "T"), (c.cities, "C")].iter().filter(|(n, _)| *n > 0).map(|(n, k)| format!("{n}{k}")).collect();
    v.join(" ")
}

pub(super) fn tokens_short(c: &si_record::Counts) -> String {
    let v: Vec<String> = [(c.beasts, "Be"), (c.disease, "Di"), (c.strife, "St"), (c.wilds, "Wi"), (c.badlands, "Ba")].iter().filter(|(n, _)| *n > 0).map(|(n, k)| format!("{k}{n}")).collect();
    v.join(" ")
}

/// Cells wrapped to their column widths, as one row of the given style.
pub(super) fn wrapped_row(cells: Vec<String>, widths: &[usize], wrap_cols: &dyn Fn(usize) -> bool, style: Style) -> (Row<'static>, u16) {
    let wrapped: Vec<Vec<String>> = cells.iter().enumerate().map(|(i, t)| if wrap_cols(i) { wrap_text(t, widths.get(i).copied().unwrap_or(8)) } else { vec![t.clone()] }).collect();
    let height = wrapped.iter().map(|w| w.len()).max().unwrap_or(1) as u16;
    let row = Row::new(wrapped.into_iter().map(|w| Cell::new(Text::from(w.join("\n")))).collect::<Vec<_>>()).height(height).style(style);
    (row, height)
}

/// Column widths for a table whose fixed columns are given and whose text columns share
/// the rest by weight, each at least its floor.
pub(super) fn share_widths(inner: usize, fixed: &[Option<usize>], weights: &[usize], floor: usize) -> Vec<usize> {
    let n = fixed.len();
    let fixed_total: usize = fixed.iter().flatten().sum::<usize>() + n.saturating_sub(1);
    let rest = inner.saturating_sub(fixed_total);
    let total_w: usize = fixed.iter().zip(weights).filter(|(f, _)| f.is_none()).map(|(_, w)| *w).sum::<usize>().max(1);
    fixed.iter().zip(weights).map(|(f, w)| match f {
        Some(x) => *x,
        None => (rest * w / total_w).max(floor),
    }).collect()
}
/// A label and a value wrapped to the width with a hanging indent under the label.
pub(super) fn hang(label: &str, label_style: Style, value: &str, value_style: Style, width: usize) -> Vec<TLine<'static>> {
    let lw = label.chars().count().max(8) + 1;
    let body = wrap_text(value, width.saturating_sub(lw).max(8));
    body.into_iter()
        .enumerate()
        .map(|(i, line)| {
            let head = if i == 0 { format!("{label:<w$}", w = lw) } else { " ".repeat(lw) };
            TLine::from(vec![Span::styled(head, label_style), Span::styled(line, value_style)])
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_breaks_on_words_and_cuts_long_ones() {
        assert_eq!(wrap_text("G1 cp5@A2 Rc +IRSL -GP2", 10), vec!["G1 cp5@A2", "Rc +IRSL", "-GP2"]);
        assert_eq!(wrap_text("abcdefghij", 4), vec!["abcd", "efgh", "ij"]);
        assert_eq!(wrap_text("", 4), vec![""]);
    }
}
