//! The open tab's own panels: PLAN in LINES' place, THINK ABOUT in READING's place, the
//! Invader phase as it stands. The other panels take the open turn through `TurnCtx`.
//! Spec: docs/superpowers/specs/2026-10-21-live-study.md section 2.

use super::*;

/// A share as a natural frequency: `about 3 in 4`.
pub(super) fn share_words(c: f64) -> String {
    let q = if c < 0.125 {
        "0 in 4"
    } else if c < 0.375 {
        "1 in 4"
    } else if c < 0.625 {
        "1 in 2"
    } else if c < 0.875 {
        "3 in 4"
    } else {
        "4 in 4"
    };
    format!("about {q}")
}

/// `none`, then each plan in the order typed, never re-sorted. A refused plan shows its
/// refusal in the cards column.
pub(super) fn plan_panel(ctx: &TurnCtx, sc: &plans::Scored, width: u16) -> Panel {
    let rows = &ctx.rows;
    let inner = width.saturating_sub(2) as usize;
    let tiers_w = (sc.none.tiers.iter().map(|(id, _, of)| ctx.code(id).len() + 3 + of.to_string().len()).sum::<usize>() + sc.none.tiers.len().saturating_sub(1)).max(8);
    let show_tiers = inner >= 60 + tiers_w;
    // tag, growth, plays, score, vs none, [tiers], produced
    let mut fixed: Vec<Option<usize>> = vec![Some(5), None, None, Some(7), Some(12)];
    let mut weights: Vec<usize> = vec![0, 22, 38, 0, 0];
    let mut headers = vec!["", "growth", "plays", "score", "vs none"];
    if show_tiers {
        fixed.push(Some(tiers_w));
        weights.push(0);
        headers.push("tiers");
    }
    fixed.push(None);
    weights.push(40);
    headers.push("produced");
    let widths = share_widths(inner, &fixed, &weights, 10);
    let text_col = |i: usize| fixed.get(i).is_some_and(|f| f.is_none()) || (show_tiers && i == 5);
    let m = sc.cands.len().max(1);
    let past_growth = sc.phase != Phase::SpiritPhase;
    let mut table_rows: Vec<Row> = Vec::new();
    let mut rows_h = 0u16;
    for rv in rows {
        let l = &rv.line;
        let code = |id: &str| ctx.code(id);
        let refused = l.outcome.notes.iter().find(|n| n.starts_with("refused: ")).cloned();
        let growth = {
            let g = l.growth_notation(&code);
            if !past_growth && rv.tag == "none" {
                "(nothing yet)".to_string()
            } else if past_growth && l.growth_actual {
                format!("{g} (as taken)")
            } else {
                g
            }
        };
        // every play in the order it acts, innates underlined, Slow plays in blue; a refused plan shows its reason
        let plays = match &refused {
            Some(r) => r.clone(),
            None => l.plays_notation(&code, false, false),
        };
        let score = if l.informed.is_nan() { "-".to_string() } else { format!("{:+.2}", l.informed) };
        let vs = match (rv.tag.as_str(), l.confidence) {
            ("none", _) | (_, None) => String::new(),
            (_, Some(c)) if m > 1 => format!("{} of {m}", (c * m as f64).round() as i64),
            (_, Some(c)) => share_words(c),
        };
        let tiers: Vec<String> = l.tiers.iter().map(|(id, reached, of)| format!("{} {reached}/{of}", code(id))).collect();
        let mark = match ctx.pool {
            Some((p, true)) if p > 0 && l.outcome.blight as i64 >= p && l.outcome.loses.is_none() => " LOSES",
            Some((p, false)) if p > 0 && l.outcome.blight as i64 >= p => " FLIPS",
            _ => "",
        };
        let mut cells: Vec<String> = vec![rv.tag.clone(), growth, plays, score, vs];
        if show_tiers {
            cells.push(tiers.join(" "));
        }
        cells.push(if refused.is_some() { String::new() } else { format!("{}{mark}", l.outcome.summary()) });
        let style = match (rv.tag.as_str(), refused.is_some()) {
            ("none", _) => Style::default().add_modifier(Modifier::BOLD),
            (_, true) => dim(),
            _ => Style::default(),
        };
        let words = play_words(l, &code);
        let (row, rh) = wrapped_row(cells, &widths, &text_col, style, Some(&play_style(&words)));
        rows_h += rh;
        table_rows.push(row);
    }
    let title = format!("PLAN  plans scored: {}{}; in the order typed; none = nothing beyond what the journal shows", sc.rows.len(), if ctx.pending { " (rescoring)" } else { "" });
    Panel { block: panel_block(&title), body: PanelBody::Table { header: header_row(&headers), rows: table_rows, widths: widths.iter().map(|w| Constraint::Length(*w as u16)).collect(), rows_h, select: Some(ctx.row.min(rows.len().saturating_sub(1))) } }
}

/// The lands a step's cards reach where Invaders stand.
fn step_hits<'a>(ctx: &'a TurnCtx, cards: &[String]) -> Vec<&'a si_rules::invaders::Land> {
    ctx.island.lands.values().filter(|l| l.c.invaders() > 0 && cards.iter().filter_map(|c| ctx.defs.invader_cards.get(c)).any(|c| l.terrain.is_some_and(|t| c.matches(t, l.coastal)))).collect()
}

/// Short sentences in game words, computed from the `none` row and the hand; nothing in
/// them is a verdict.
pub(super) fn think_about_panel(ctx: &TurnCtx, sc: &plans::Scored, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let open = &sc.open;
    let t = &open.t;
    let seed = &open.seed;
    let none = &sc.none;
    let mut out: Vec<String> = Vec::new();
    if let Some(a) = &ctx.activity {
        out.push(a.clone());
    }
    // how near each loss condition stands now: the forward view the adversary asks for
    for c in si_rules::invaders::clocks(&ctx.island, &open.rules, si_rules::invaders::Tally::default()) {
        out.push(format!("Loss clock: {c}."));
    }
    // each Ravage land where something is at stake: the simulator's own numbers
    for l in step_hits(ctx, &t.ravage) {
        let mut stake = Vec::new();
        if l.c.dahan > 0 {
            stake.push(format!("{} Dahan", l.c.dahan));
        }
        if l.c.presence > 0 {
            stake.push("your presence".into());
        }
        let th = si_hindsight::live::ravage_thresholds(&ctx.island, ctx.defs, &t.ravage, &open.rules, &l.id);
        let saves = match (th.defend_keeps_land, th.defend_keeps_dahan) {
            (Some(a), Some(b)) if l.c.dahan > 0 && b != a => format!("; Defend {b} keeps the Dahan, Defend {a} keeps the land"),
            (Some(a), _) => format!("; Defend {a} keeps the land"),
            (None, _) => String::new(),
        };
        out.push(format!("{} Ravages for {}{}{}.", l.id, th.damage, if stake.is_empty() { String::new() } else { format!(" with {} at stake", stake.join(" and ")) }, saves));
    }
    // the Builds, from the none row's board after
    let builds: Vec<String> = step_hits(ctx, &t.build)
        .into_iter()
        .map(|l| {
            let a = none.outcome.after.get(&l.id).copied().unwrap_or_default();
            format!("{} {}", l.id, if a.cities > l.c.cities { "a City" } else if a.towns > l.c.towns { "a Town" } else { "nothing" })
        })
        .collect();
    if !builds.is_empty() {
        out.push(format!("Builds: {}.", builds.join(", ")));
    }
    // the adversary's loss condition: what doing nothing reaches, and England's escalation
    // when every card left in the stage carries it (Coastal is the one Stage II card without)
    if let Some(r) = &none.outcome.loses {
        out.push(format!("Doing nothing loses the game this turn: {r}."));
    }
    let all_escalate = !sc.cands.is_empty() && sc.cands.iter().all(|c| ctx.defs.invader_cards.get(c).is_some_and(|x| x.escalation));
    if all_escalate && open.rules.adversary.as_deref() == Some("TheKingdomOfEngland") && open.rules.level >= 1 {
        let mut by_board: std::collections::BTreeMap<String, (u32, si_model::island::LandId)> = std::collections::BTreeMap::new();
        for (id, c) in &none.outcome.after {
            let b = c.towns + c.cities;
            if b > 0 && by_board.get(&id.board).is_none_or(|(n, _)| b > *n) {
                by_board.insert(id.board.clone(), (b, id.clone()));
            }
        }
        for (n, land) in by_board.values() {
            let then = n + 1;
            out.push(format!("Every card left in the stage escalates: England's Building Boom builds again in {land}, {n} Town/City after the Build, {then} after it{}.", if then >= 7 { ": 7 loses the game (Proud & Mighty Capital) unless a Town or City there falls first" } else { "" }));
        }
    }
    if let Some(r) = &none.outcome.next_loses {
        out.push(format!("Unless something stops it, the Invaders win next turn: {r}."));
    }
    if let Some(p) = seed.blight_pool {
        out.push(format!("Blight card: {p} in the pool{}; doing nothing adds {} this turn.", if seed.blight_flipped { " (flipped)" } else { "" }, none.outcome.blight));
    }
    let hand = open.s.hand_after.len() + open.s.cards_played.len();
    let plays = sc.plays;
    out.push(format!("Hand {hand}, plays {plays}{}.", if (hand as i64) < plays { ": a Reclaim next turn unless this turn's growth draws" } else { "" }));
    if open.phase == Phase::SpiritPhase {
        if let Some(panel) = ctx.panel {
            let drawing: Vec<u8> = panel.growth.groups.iter().filter(|g| g.yield_().cards > 0).map(|g| g.number).collect();
            if !drawing.is_empty() {
                let groups = drawing.iter().map(|n| format!("G{n}")).collect::<Vec<_>>().join(" or ");
                for l in step_hits(ctx, &t.ravage).into_iter().take(2) {
                    for kind in [si_model::power::PowerKind::Minor, si_model::power::PowerKind::Major] {
                        let dc = si_hindsight::live::draw_count(ctx.defs, open, kind, &l.id);
                        let word = if kind == si_model::power::PowerKind::Minor { "Minors" } else { "Majors" };
                        out.push(format!("{groups} draws: of the {} unseen {word}, {} play this turn on {}{}.", dc.unseen, dc.playable, l.id, if dc.unreadable > 0 { format!(" ({} not readable)", dc.unreadable) } else { String::new() }));
                    }
                }
            }
        }
    }
    let lines: Vec<TLine<'static>> = out.iter().flat_map(|s| wrap_text(s, w).into_iter().map(|l| TLine::from(Span::raw(l)))).collect();
    Panel::text("THINK ABOUT  from the none row; nothing here is a verdict", lines)
}

/// The Invader phase as it stands: the slots, the Explore's pool, the Blight card, the
/// Fear counts, the Event once drawn, and each recorded step against what the selected
/// plan said.
pub(super) fn invaders_live_panel(ctx: &TurnCtx, sc: &plans::Scored, width: u16) -> Panel {
    let w = width.saturating_sub(2) as usize;
    let title = "INVADER PHASE  as it stands";
    let open = &sc.open;
    let t = &open.t;
    let seed = &open.seed;
    let mut lines: Vec<TLine<'static>> = Vec::new();
    let lands_words = |v: &[(LandId, String)]| -> String {
        if v.is_empty() { "no land".to_string() } else { v.iter().map(|(l, w)| if w == "-" { l.to_string() } else { format!("{l} {w}") }).collect::<Vec<_>>().join(", ") }
    };
    if !t.high_immigration.is_empty() {
        let v = ctx.step_lands(&t.high_immigration, InvaderStep::Build);
        lines.extend(hang("HI build", dim(), &format!("{}: {}", t.high_immigration.join(" "), lands_words(&v)), Style::default(), w));
    }
    for (label, cards, step) in [("Ravage", &t.ravage, InvaderStep::Ravage), ("Build", &t.build, InvaderStep::Build)] {
        if cards.is_empty() {
            lines.extend(hang(label, dim(), "no card in the slot", Style::default(), w));
        } else {
            let v = ctx.step_lands(cards, step);
            lines.extend(hang(label, dim(), &format!("{}: {}", cards.join(" "), lands_words(&v)), Style::default(), w));
        }
    }
    let fc0 = sc.forecast.iter().find(|fc| fc.turn == sc.turn);
    let explore = if t.explore.is_empty() {
        match fc0 {
            Some(fc) if fc.cards.len() > 1 => format!("face down; Stage {}, {} of {} left: {}", fc.stage, fc.cards.len(), open.deck.left_after(sc.turn.saturating_sub(1)).max(fc.cards.len()), fc.cards.join(" ")),
            Some(fc) => format!("the last Stage {} card, knowable: {}", fc.stage, fc.cards.join(" ")),
            None => "face down".to_string(),
        }
    } else {
        let v = ctx.step_lands(&t.explore, InvaderStep::Explore);
        format!("came {}: {}", t.explore.join(" "), lands_words(&v))
    };
    lines.extend(hang("Explore", dim(), &explore, Style::default(), w));
    let plan_blight = ctx.sel.as_ref().map(|rv| rv.line.outcome.blight).unwrap_or(0);
    let plan_tag = ctx.sel.as_ref().map(|rv| rv.tag.clone()).unwrap_or_else(|| "none".into());
    match seed.blight_pool {
        Some(p) => lines.extend(hang("Blight card", dim(), &format!("{}{p} in the pool{}; plan {plan_tag} adds {plan_blight}, leaves {}", seed.blight_card.as_deref().map(|c| format!("{}, ", spaced(c))).unwrap_or_default(), if seed.blight_flipped { " (flipped)" } else { "" }, p - plan_blight as i64), Style::default(), w)),
        None => lines.extend(hang("Blight card", dim(), &format!("pool unknown; plan {plan_tag} adds {plan_blight}"), Style::default(), w)),
    }
    lines.extend(hang("Fear", dim(), &format!("{} card{} earned, Terror {}, {} of {} toward the next", seed.fear_cards_banked, if seed.fear_cards_banked == 1 { "" } else { "s" }, seed.terror, seed.fear_generated, seed.fear_required), Style::default(), w));
    let ev = if t.events.is_empty() {
        "not drawn".to_string()
    } else {
        let asked: Vec<String> = t.choices.iter().filter(|c| c.source_kind == "Event").map(|c| format!("{} -> {}", spaced(&c.source), clean_engine_text(&c.selected))).collect();
        format!("came {}{}", t.events.iter().map(|e| spaced(e)).collect::<Vec<_>>().join(", "), if asked.is_empty() { String::new() } else { format!("; asked {}", asked.join("; ")) })
    };
    lines.extend(hang("Event", dim(), &ev, Style::default(), w));
    if !t.fear_cards.is_empty() {
        let titles: Vec<String> = t.fear_cards.iter().map(|id| ctx.defs.fear_cards.get(id).map(|c| c.title.clone()).unwrap_or_else(|| spaced(id))).collect();
        lines.extend(hang("Fear cards", dim(), &format!("resolved {}", titles.join(", ")), Style::default(), w));
    }
    // each recorded step against what the selected plan said there
    let after = ctx.sel.as_ref().map(|rv| rv.line.outcome.after.clone()).unwrap_or_default();
    for l in ctx.island.lands.values() {
        let visits = ctx.visits(&l.id);
        if visits.is_empty() {
            continue;
        }
        let came: Vec<String> = visits.iter().map(|v| format!("{}:{}", step_letter(v.step), visit_words(v))).collect();
        let said = after.get(&l.id).map(|a| after_words(&l.c, a)).unwrap_or_default();
        lines.extend(hang(&format!("came {}", l.id), dim(), &format!("{}; plan said {}", came.join(" "), if said.is_empty() { "no change".to_string() } else { said }), Style::default(), w));
    }
    Panel::text(title, lines)
}

/// What a land's counts became, in the record's tokens: `2E 1T Bl2 Dh1`; empty when nothing moved.
pub(super) fn after_words(before: &si_record::Counts, after: &si_record::Counts) -> String {
    let mut v: Vec<String> = Vec::new();
    if (after.explorers, after.towns, after.cities) != (before.explorers, before.towns, before.cities) {
        let p = pieces_short(after);
        v.push(if p.is_empty() { "no Invaders".to_string() } else { p });
    }
    if after.blight != before.blight {
        v.push(format!("Bl{}", after.blight));
    }
    if after.dahan != before.dahan {
        v.push(format!("Dh{}", after.dahan));
    }
    if after.presence != before.presence {
        v.push(format!("Pr{}", after.presence));
    }
    v.join(" ")
}
