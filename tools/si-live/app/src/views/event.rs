//! Event: what does this turn's Event do here, and what will you choose?
//! The Event comes from the game log the moment it's revealed (the save
//! shows it only at the end of the step); each section is marked by whether
//! it applies now.

use super::{fmt, phase};
use si_common::companion_stats::CompanionStats;
use si_common::public::{PublicEventCard, PublicSnapshot};
use si_steam::companion::CompanionState;
use si_ui::theme;
use si_ui::widgets::{self, panel};
use si_ui::{Rect, Ui};

pub fn draw(ui: &mut Ui, r: Rect, s: &PublicSnapshot, stats: &CompanionStats, st: &CompanionState) {
    // This turn's Event from the log, else the last one in the save.
    let logged = st.decisions.iter().rev().find_map(|d| d.event.as_ref().map(|e| (d.turn, e)));
    let (card, when): (Option<&PublicEventCard>, String) = match (logged, s.events.shown.as_ref()) {
        (Some((t, e)), _) if t == s.turn => (Some(e), if s.step == "Event" { "revealed now".into() } else { format!("Turn {t}") }),
        (_, Some(e)) => (Some(e), if e.in_play { "being resolved".into() } else { "the last Event".into() }),
        (Some((t, e)), None) => (Some(e), format!("Turn {t}")),
        (None, None) => (None, String::new()),
    };
    let c = panel(ui, r, &format!("Event · Turn {}", s.turn), Some(&when));
    let off = ui.scroll_begin("event", c);
    let (x, w) = (c.x, c.w - 8.0);
    let mut y = c.y - off;
    y = phase::callouts(ui, x, y, w, s, stats);
    y = phase::question(ui, x, y, w, "What does the Event do here, and what will you choose?");
    let Some(e) = card else {
        ui.text(x, y, "No Event revealed yet this game (the Turn 1 Event is discarded unresolved).", theme::BODY.color(theme::MUTED));
        ui.scroll_end("event", c, y + 30.0 + off - c.y);
        return;
    };
    ui.text(x, y, &e.title, theme::TITLE);
    y += 30.0;
    let flipped = s.blight.flipped;
    let tl = s.fear.terror_level as i64;
    let stage = s.invaders.highest_stage_reached;
    for sec in &e.sections {
        let (label, applies) = applies(&sec.types, flipped, tl, stage);
        let body = sec.body.iter().map(|b| fmt::plain(b)).collect::<Vec<_>>().join(" ");
        let (_, bh) = ui.measure_spans(&[si_ui::span(&body, theme::BODY)], Some(w - 24.0));
        let rr = Rect::new(x, y, w, 34.0 + bh);
        let (border, fg) = if applies { (theme::ACCENT, theme::TEXT) } else { (theme::BORDER, theme::MUTED) };
        ui.rect(rr, if applies { theme::RAISED } else { theme::PANEL }, border, 1.0, theme::RADIUS_SM);
        let tw = ui.text(x + 12.0, y + 7.0, &sec.title, theme::BODY.bold().color(fg)).0;
        let tag = if applies { label } else { format!("{label}: doesn't apply now") };
        widgets::tag(ui, x + 12.0 + tw + 10.0, y + 6.0, &tag, if applies { theme::ACCENT } else { theme::MUTED });
        ui.text_wrap(x + 12.0, y + 28.0, w - 24.0, &body, theme::BODY.color(fg));
        y += rr.h + 8.0;
    }
    let note = "Text from the Steam engine; where the printed card differs, the printed card governs.";
    ui.text(x, y, note, theme::SMALL.color(theme::MUTED));
    y += 24.0;
    ui.scroll_end("event", c, y + off - c.y);
}

/// A section's trigger in words, and whether it applies to this game now.
/// The main section is the one matching the island's side, the Terror Level
/// or the Invader Stage; token and Dahan sections always resolve.
fn applies(types: &[String], flipped: bool, tl: i64, stage: i64) -> (String, bool) {
    let words: Vec<String> = types.iter().map(|t| word(t)).collect();
    let ok = types.iter().any(|t| match t.as_str() {
        "HealthyIsland" => !flipped,
        "BlightedIsland" => flipped,
        "TerrorLevelI" => tl == 1,
        "TerrorLevelII" => tl == 2,
        "TerrorLevelIII" => tl == 3,
        "StageI" => stage <= 1,
        "StageII" => stage == 2,
        "StageIII" => stage >= 3,
        _ => true,
    });
    (words.join(" / "), ok)
}

fn word(t: &str) -> String {
    match t {
        "HealthyIsland" => "Healthy island".into(),
        "BlightedIsland" => "Blighted island".into(),
        "TerrorLevelI" => "Terror Level I".into(),
        "TerrorLevelII" => "Terror Level II".into(),
        "TerrorLevelIII" => "Terror Level III".into(),
        "StageI" => "Stage I".into(),
        "StageII" => "Stage II".into(),
        "StageIII" => "Stage III".into(),
        "Beast" => "Beasts".into(),
        "Choice" => "Choose one".into(),
        other => other.to_string(),
    }
}
