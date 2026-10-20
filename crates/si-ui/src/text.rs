//! Shaped text, cached by content and style. Immediate-mode callers ask for
//! the same strings every frame; shaping happens once per distinct string.

use crate::theme::TextStyle;
use glyphon::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Weight, Wrap};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

/// A run of text in one style; several make a line with mixed styles.
#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    pub text: &'a str,
    pub style: TextStyle,
}

pub fn span(text: &str, style: TextStyle) -> Span<'_> {
    Span { text, style }
}

pub(crate) struct Entry {
    pub(crate) buffer: Buffer,
    w: f32,
    h: f32,
    used: u64,
}

pub struct TextCache {
    pub(crate) fonts: FontSystem,
    pub(crate) entries: HashMap<u64, Entry>,
    frame: u64,
    sans: String,
    mono: String,
}

/// The first family in `wanted` that is installed, or "" for the default.
fn installed(fonts: &FontSystem, wanted: &[&str]) -> String {
    let db = fonts.db();
    wanted
        .iter()
        .find(|w| db.faces().any(|f| f.families.iter().any(|(n, _)| n == *w)))
        .map(|w| w.to_string())
        .unwrap_or_default()
}

impl Default for TextCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TextCache {
    /// Loads the system fonts: Segoe UI and Cascadia Mono / Consolas on
    /// Windows, with Linux fallbacks for development.
    pub fn new() -> Self {
        let fonts = FontSystem::new();
        let sans = installed(&fonts, &["Segoe UI", "Inter", "Noto Sans", "DejaVu Sans"]);
        let mono = installed(&fonts, &["Cascadia Mono", "Consolas", "JetBrains Mono", "DejaVu Sans Mono"]);
        TextCache { fonts, entries: HashMap::new(), frame: 0, sans, mono }
    }

    pub fn families(&self) -> (&str, &str) {
        (&self.sans, &self.mono)
    }

    /// Shape `spans` (wrapped at `max_w` if given). Returns the cache key and
    /// the laid-out width and height in logical pixels.
    pub fn layout(&mut self, spans: &[Span], max_w: Option<f32>) -> (u64, f32, f32) {
        let mut h = DefaultHasher::new();
        for s in spans {
            s.text.hash(&mut h);
            s.style.size.to_bits().hash(&mut h);
            s.style.line.to_bits().hash(&mut h);
            s.style.mono.hash(&mut h);
            s.style.weight.hash(&mut h);
            s.style.color.hash(&mut h);
        }
        max_w.map(f32::to_bits).hash(&mut h);
        let key = h.finish();
        if let Some(e) = self.entries.get_mut(&key) {
            e.used = self.frame;
            return (key, e.w, e.h);
        }

        let size = spans.iter().map(|s| s.style.size).fold(1.0, f32::max);
        let line = spans.iter().map(|s| s.style.line).fold(size, f32::max);
        let mut buffer = Buffer::new(&mut self.fonts, Metrics::new(size, line));
        buffer.set_wrap(if max_w.is_some() { Wrap::WordOrGlyph } else { Wrap::None });
        buffer.set_size(max_w, None);
        let (sans, mono) = (self.sans.clone(), self.mono.clone());
        let attrs = |st: &TextStyle| {
            let family = match (st.mono, st.mono && !mono.is_empty(), !sans.is_empty()) {
                (true, true, _) => Family::Name(&mono),
                (true, false, _) => Family::Monospace,
                (false, _, true) => Family::Name(&sans),
                (false, _, false) => Family::SansSerif,
            };
            let c = st.color;
            Attrs::new()
                .family(family)
                .weight(Weight(st.weight))
                .color(Color::rgba(c.0, c.1, c.2, c.3))
                .metrics(Metrics::new(st.size, st.line))
        };
        let default = spans.first().map(|s| attrs(&s.style)).unwrap_or_else(Attrs::new);
        buffer.set_rich_text(spans.iter().map(|s| (s.text, attrs(&s.style))), &default, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.fonts, false);
        let (mut w, mut hgt) = (0.0f32, 0.0f32);
        for run in buffer.layout_runs() {
            w = w.max(run.line_w);
            hgt = hgt.max(run.line_top + run.line_height);
        }
        self.entries.insert(key, Entry { buffer, w, h: hgt, used: self.frame });
        (key, w, hgt)
    }

    /// Drop text not drawn for a while.
    pub fn end_frame(&mut self) {
        self.frame += 1;
        let frame = self.frame;
        self.entries.retain(|_, e| e.used + 600 > frame);
    }
}
