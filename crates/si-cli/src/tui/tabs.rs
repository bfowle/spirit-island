//! The folder tab bar: every turn as a tab, the current one open into the screen below.

use super::*;

/// A turn's verdict as a short word and its color: sound green, better yellow, mistake red.
pub(super) fn verdict_short(r: &TurnReview) -> (String, Color) {
    let word = r.reading.split(':').next().unwrap_or("").trim();
    match word {
        "Sound" => ("sound".into(), Color::Green),
        "Unlucky" => ("unlucky".into(), Color::Cyan),
        "Gamble paid" => ("gamble".into(), Color::Green),
        "Better available on what was known" | "Better available" => (format!("better {:.1}", r.decision), Color::Yellow),
        "Mistake on what was known" | "Mistake" => (format!("MISTAKE {:.1}", r.decision), Color::Red),
        _ => ("not judged".into(), Color::DarkGray),
    }
}

/// The turns as folder tabs, the current one open into the screen below, the two
/// screens as tabs at the right end. Box-drawing only, the corners the panels use.
pub(super) fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let width = area.width as usize;
    let full: Vec<(String, Color)> = app
        .turns
        .iter()
        .map(|t| match app.reviews.get(t) {
            Some(r) => {
                let (word, c) = verdict_short(r);
                (format!("T{t} {word}"), c)
            }
            None => (format!("T{t} ..."), Color::DarkGray),
        })
        .collect();
    let screens = [("Turn", Screen::Turn), ("Pacing", Screen::Pacing)];
    let screens_w: usize = screens.iter().map(|(n, _)| n.chars().count() + 4).sum::<usize>() + 1;
    let tabs_w = |labels: &[(String, Color)]| labels.iter().map(|(l, _)| l.chars().count() + 4).sum::<usize>() + 1;
    // narrower labels when the bar would overflow: the number only, then the turn only
    let mut labels = full;
    if tabs_w(&labels) + screens_w + 2 > width {
        labels = app.turns.iter().zip(labels.iter()).map(|(t, (_, c))| match app.reviews.get(t) {
            Some(r) if r.decision > 0.05 => (format!("T{t} {:.1}", r.decision), *c),
            _ => (format!("T{t}"), *c),
        }).collect();
    }
    if tabs_w(&labels) + screens_w + 2 > width {
        labels = app.turns.iter().zip(labels.iter()).map(|(t, (_, c))| (format!("T{t}"), *c)).collect();
    }
    let accent = Style::default().fg(ACCENT);
    let mut top: Vec<Span> = vec![Span::raw(" ")];
    let mut bot: Vec<Span> = vec![Span::styled("\u{2500}", dim())];
    let mut used = 1usize;
    // one tab: its label between rounded corners above, open (blank) or closed (a line) below; returns its width
    let tab = |top: &mut Vec<Span>, bot: &mut Vec<Span>, label: &str, color: Color, open: bool| -> usize {
        let n = label.chars().count() + 2;
        let corner = if open { accent } else { dim() };
        let text = if open { Style::default().fg(color).add_modifier(Modifier::BOLD) } else { Style::default().fg(color) };
        top.push(Span::styled("\u{256d}", corner));
        top.push(Span::styled(format!(" {label} "), text));
        top.push(Span::styled("\u{256e}", corner));
        if open {
            bot.push(Span::styled("\u{256f}", accent));
            bot.push(Span::raw(" ".repeat(n)));
            bot.push(Span::styled("\u{2570}", accent));
        } else {
            bot.push(Span::styled("\u{2534}", dim()));
            bot.push(Span::styled("\u{2500}".repeat(n), dim()));
            bot.push(Span::styled("\u{2534}", dim()));
        }
        n + 2
    };
    for (i, (label, color)) in labels.iter().enumerate() {
        used += tab(&mut top, &mut bot, label, *color, app.screen == Screen::Turn && i == app.cur);
    }
    let fill = width.saturating_sub(used + screens_w);
    top.push(Span::raw(" ".repeat(fill)));
    bot.push(Span::styled("\u{2500}".repeat(fill), dim()));
    for (name, sc) in screens {
        tab(&mut top, &mut bot, name, ACCENT, app.screen == sc);
    }
    bot.push(Span::styled("\u{2500}", dim()));
    f.render_widget(Paragraph::new(vec![TLine::from(top), TLine::from(bot)]), area);
}

