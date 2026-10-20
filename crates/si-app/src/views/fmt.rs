//! Shared view helpers: terrain colors, land and card chips, small formats.

use si_common::public::{PublicLand, PublicSnapshot};
use si_ui::theme::{self, Rgba, TextStyle};
use si_ui::{Align, Rect, Ui};

/// Terrain colors matched to the Steam game (playtest U5: Mountain is grey).
pub fn terrain(t: &str) -> Rgba {
    match t {
        "Jungle" => Rgba::hex(0x2f8a4a),
        "Mountain" => Rgba::hex(0x8c8f99),
        "Sands" => Rgba::hex(0xd19a4e),
        "Wetland" => Rgba::hex(0x3a8ea3),
        "Ocean" => Rgba::hex(0x2c5f93),
        "Coastal" => Rgba::hex(0x4f7fb8),
        _ => theme::MUTED,
    }
}

pub fn roman(n: i64) -> &'static str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        _ => "–",
    }
}

pub fn land<'a>(s: &'a PublicSnapshot, id: &str) -> Option<&'a PublicLand> {
    s.lands.iter().find(|l| l.id == id)
}

/// A board's short name: "D", or "NW" for the thematic NorthWest board.
pub fn board_short(b: &str) -> String {
    if b.len() <= 2 { b.to_string() } else { b.chars().filter(|c| c.is_ascii_uppercase()).collect() }
}

/// "D2" for `JungleD2`, "NW4" for `NorthWest4`.
pub fn short(s: &PublicSnapshot, id: &str) -> String {
    land(s, id).map(|l| format!("{}{}", board_short(&l.board), l.number)).unwrap_or_else(|| id.to_string())
}

/// Land ids inside free text ("on JungleD2") shortened to "D2 Jungle".
pub fn humanize(s: &PublicSnapshot, text: &str) -> String {
    let mut out = text.to_string();
    // Longest ids first, so "NorthWest1" doesn't eat "NorthWest10".
    let mut lands: Vec<&PublicLand> = s.lands.iter().collect();
    lands.sort_by_key(|l| std::cmp::Reverse(l.id.len()));
    for l in lands {
        if out.contains(&l.id) {
            out = out.replace(&l.id, &format!("{}{} {}", board_short(&l.board), l.number, l.terrain));
        }
    }
    out
}

/// Card text as plain words: icon tokens ("{Town}/{City}" → "Town/City"),
/// the engine's layout tokens ({BR}, {Bullet}, {EndBullet}) and markup tags
/// (<i>, <b>, <nobr>, <u>) removed.
pub fn plain(text: &str) -> String {
    let t = text.replace("{BR}", " ").replace("{Bullet}", " • ").replace("{EndBullet}", " ");
    let mut out = String::with_capacity(t.len());
    let mut in_tag = false;
    for ch in t.chars() {
        match ch {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            '{' | '}' => {}
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A land chip: terrain swatch, board number, terrain. Returns the width.
pub fn land_chip(ui: &mut Ui, x: f32, y: f32, s: &PublicSnapshot, id: &str) -> f32 {
    let (label, color) = match land(s, id) {
        Some(l) => (format!("{}{} {}", board_short(&l.board), l.number, l.terrain), terrain(&l.terrain)),
        None => (id.to_string(), theme::MUTED),
    };
    let style = TextStyle { size: 12.0, line: 16.0, mono: false, weight: 600, color: theme::TEXT };
    let (w, _) = ui.measure(&label, style);
    let r = Rect::new(x, y, w + 16.0, 20.0);
    ui.fill(r, theme::RAISED, theme::RADIUS_SM);
    ui.fill(Rect::new(r.x, r.y, 4.0, r.h), color, 2.0);
    ui.text_in(r.inset(9.0, 0.0, 0.0, 0.0), &label, style, Align::Left);
    r.w
}

/// One line of text cut to `w` with an ellipsis if it doesn't fit.
pub fn fit(ui: &mut Ui, text: &str, style: TextStyle, w: f32) -> String {
    if ui.measure(text, style).0 <= w {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let cut = |n: usize| chars[..n].iter().collect::<String>().trim_end().to_string() + "…";
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if ui.measure(&cut(mid), style).0 <= w {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    if lo == 0 { String::new() } else { cut(lo) }
}
