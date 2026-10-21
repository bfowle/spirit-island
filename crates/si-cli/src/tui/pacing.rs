//! The Pacing screen: six charts over the turns from the record and the review.

use super::*;

pub(super) fn draw_pacing(f: &mut Frame, app: &App, area: Rect) {
    let rec = &app.rec;
    let turns: Vec<&si_record::TurnRecord> = rec.turns.iter().filter(|t| t.turn >= 1).collect();
    if turns.is_empty() {
        f.render_widget(Paragraph::new("no turns in the record"), area);
        return;
    }
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let xmax = turns.last().map(|t| t.turn).unwrap_or(1) as f64;
    let x_labels: Vec<Span> = turns.iter().map(|t| Span::raw(format!("T{}", t.turn))).collect();
    let [row1, row2] = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
    let [a1, a2, a3] = Layout::horizontal([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).areas(row1);
    let [b1, b2, b3] = Layout::horizontal([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).areas(row2);

    // Blight: on the island, added per turn, the pool when known
    let island: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.board_after.blight as f64)).collect();
    let added: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, (t.blight_added + t.blight_cascaded) as f64)).collect();
    let pool: Vec<(f64, f64)> = turns.iter().filter_map(|t| app.pool_at.get(&t.turn).map(|(p, _)| (t.turn as f64, *p as f64))).collect();
    let ymax = island.iter().chain(added.iter()).chain(pool.iter()).map(|p| p.1).fold(1.0, f64::max);
    let mut sets = vec![
        Dataset::default().name("on the island").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Red)).data(&island),
        Dataset::default().name("added this turn").marker(symbols::Marker::Block).graph_type(GraphType::Scatter).style(Style::default().fg(Color::Yellow)).data(&added),
    ];
    if !pool.is_empty() {
        sets.push(Dataset::default().name("left on the card").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::DarkGray)).data(&pool));
    }
    chart(f, a1, " BLIGHT  the loss clock ", sets, xmax, ymax, &x_labels);

    // Invaders: Explorers, Towns, Cities and lands with Invaders after each turn
    let e: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.board_after.explorers as f64)).collect();
    let tw: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.board_after.towns as f64)).collect();
    let c: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.board_after.cities as f64)).collect();
    let lands: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.lands_with_invaders_after as f64)).collect();
    let ymax = e.iter().chain(tw.iter()).chain(c.iter()).chain(lands.iter()).map(|p| p.1).fold(1.0, f64::max);
    chart(
        f,
        a2,
        " INVADERS  on the island after each turn ",
        vec![
            Dataset::default().name("Explorers").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::White)).data(&e),
            Dataset::default().name("Towns").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Yellow)).data(&tw),
            Dataset::default().name("Cities").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Red)).data(&c),
            Dataset::default().name("lands with Invaders").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Magenta)).data(&lands),
        ],
        xmax,
        ymax,
        &x_labels,
    );

    // Fear: cards earned per turn and cumulative, the Terror Level
    let mut cum = 0u32;
    let cards_cum: Vec<(f64, f64)> = turns
        .iter()
        .map(|t| {
            cum += t.fear_cards_earned;
            (t.turn as f64, cum as f64)
        })
        .collect();
    let per: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.fear_cards_earned as f64)).collect();
    let tl: Vec<(f64, f64)> = turns.iter().map(|t| (t.turn as f64, t.terror_after as f64)).collect();
    let ymax = cards_cum.iter().map(|p| p.1).fold(3.0, f64::max);
    chart(
        f,
        a3,
        " FEAR  the win clock ",
        vec![
            Dataset::default().name("cards earned, cumulative").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Magenta)).data(&cards_cum),
            Dataset::default().name("cards this turn").marker(symbols::Marker::Block).graph_type(GraphType::Scatter).style(Style::default().fg(Color::Yellow)).data(&per),
            Dataset::default().name("Terror Level").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::DarkGray)).data(&tl),
        ],
        xmax,
        ymax,
        &x_labels,
    );

    // the spirit: Energy after the turn, plays, cards played, presence on the island
    let sp: Vec<&si_record::SpiritTurn> = rec.spirits.iter().filter(|s| s.spirit == spirit && s.turn >= 1).collect();
    let energy: Vec<(f64, f64)> = sp.iter().map(|s| (s.turn as f64, s.energy_after as f64)).collect();
    let plays: Vec<(f64, f64)> = sp.iter().map(|s| (s.turn as f64, s.plays.unwrap_or(s.cards_played.len() as i64) as f64)).collect();
    let played: Vec<(f64, f64)> = sp.iter().map(|s| (s.turn as f64, s.cards_played.len() as f64)).collect();
    let presence: Vec<(f64, f64)> = sp.iter().map(|s| (s.turn as f64, s.presence_after.values().sum::<u32>() as f64)).collect();
    let ymax = energy.iter().chain(plays.iter()).chain(played.iter()).chain(presence.iter()).map(|p| p.1).fold(1.0, f64::max);
    chart(
        f,
        b1,
        " SPIRIT  the track, by turn ",
        vec![
            Dataset::default().name("Energy after").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Yellow)).data(&energy),
            Dataset::default().name("plays").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Cyan)).data(&plays),
            Dataset::default().name("cards played").marker(symbols::Marker::Block).graph_type(GraphType::Scatter).style(Style::default().fg(Color::White)).data(&played),
            Dataset::default().name("presence on the island").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Green)).data(&presence),
        ],
        xmax,
        ymax,
        &x_labels,
    );

    // the review: the actual line's informed and hindsight per turn
    let reviewed: Vec<&TurnReview> = app.turns.iter().filter_map(|t| app.reviews.get(t)).collect();
    let informed: Vec<(f64, f64)> = reviewed.iter().map(|r| (r.turn as f64, r.actual.informed)).collect();
    let hindsight: Vec<(f64, f64)> = reviewed.iter().map(|r| (r.turn as f64, r.actual.score)).collect();
    let best: Vec<(f64, f64)> = reviewed.iter().map(|r| (r.turn as f64, r.actual.informed + r.decision)).collect();
    let lo = informed.iter().chain(hindsight.iter()).chain(best.iter()).map(|p| p.1).fold(0.0, f64::min).floor();
    let hi = informed.iter().chain(hindsight.iter()).chain(best.iter()).map(|p| p.1).fold(1.0, f64::max).ceil();
    let title = if reviewed.is_empty() { " REVIEW  your line by turn (reviewing...) " } else { " REVIEW  your line by turn " };
    chart_bounds(
        f,
        b2,
        title,
        vec![
            Dataset::default().name("best informed").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Green)).data(&best),
            Dataset::default().name("yours, informed").marker(symbols::Marker::HalfBlock).graph_type(GraphType::Line).style(Style::default().fg(Color::Cyan)).data(&informed),
            Dataset::default().name("yours, hindsight").marker(symbols::Marker::Block).graph_type(GraphType::Scatter).style(Style::default().fg(Color::Yellow)).data(&hindsight),
        ],
        xmax,
        [lo, hi],
        &x_labels,
    );

    // decision gap and luck per turn as bars (luck shown as its size, colored by sign)
    let mut groups: Vec<BarGroup> = Vec::new();
    for r in &reviewed {
        let luck_color = if r.luck < 0.0 { Color::Red } else { Color::Green };
        let bars = vec![
            Bar::default().value((r.decision * 10.0).round() as u64).text_value(format!("{:.1}", r.decision)).style(Style::default().fg(Color::Yellow)),
            Bar::default().value((r.luck.abs() * 10.0).round() as u64).text_value(format!("{:+.1}", r.luck)).style(Style::default().fg(luck_color)),
        ];
        groups.push(BarGroup::default().label(TLine::from(format!("T{}", r.turn))).bars(&bars));
    }
    let bc = BarChart::grouped(groups).bar_width(5).bar_gap(1).group_gap(2).block(Block::default().borders(Borders::ALL).title(Span::styled(" GAP (yellow) and LUCK (green +, red -) by turn ", header_style())));
    f.render_widget(bc, b3);
}

pub(super) fn chart(f: &mut Frame, area: Rect, title: &str, sets: Vec<Dataset>, xmax: f64, ymax: f64, x_labels: &[Span]) {
    chart_bounds(f, area, title, sets, xmax, [0.0, ymax.max(1.0)], x_labels);
}

pub(super) fn chart_bounds(f: &mut Frame, area: Rect, title: &str, sets: Vec<Dataset>, xmax: f64, y: [f64; 2], x_labels: &[Span]) {
    let y_labels: Vec<Span> = vec![Span::raw(format!("{:.0}", y[0])), Span::raw(format!("{:.0}", (y[0] + y[1]) / 2.0)), Span::raw(format!("{:.0}", y[1]))];
    let c = Chart::new(sets)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(title.to_string(), header_style())))
        .x_axis(Axis::default().bounds([1.0, xmax.max(2.0)]).labels(x_labels.to_vec()))
        .y_axis(Axis::default().bounds(y).labels(y_labels))
        .legend_position(Some(ratatui::widgets::LegendPosition::TopLeft))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));
    f.render_widget(c, area);
}

