//! The immediate-mode painter. Views call it every frame with logical
//! coordinates; it records rectangles and text into layers (base, then
//! overlay) in physical pixels for the renderer, and answers hover and click
//! from this frame's input.

use crate::geom::Rect;
use crate::text::{Span, TextCache, span};
use crate::theme::{self, Rgba, TextStyle};
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub border: [f32; 4],
    pub params: [f32; 4],
    pub clip: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct TextDraw {
    pub key: u64,
    pub x: f32,
    pub y: f32,
    /// Display scale: text is shaped in logical pixels, rasterized at this.
    pub scale: f32,
    /// Physical pixels: left, top, right, bottom.
    pub clip: [i32; 4],
}

#[derive(Debug, Default)]
pub struct Layer {
    pub rects: Vec<RectInstance>,
    pub texts: Vec<TextDraw>,
}

/// Everything one frame draws, in order.
#[derive(Debug, Default)]
pub struct Frame {
    pub layers: [Layer; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Space,
    Escape,
    Char(char),
}

/// This frame's input, in logical pixels.
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub mouse: Option<(f32, f32)>,
    /// The primary button was released this frame.
    pub clicked: bool,
    /// Wheel lines (positive = up).
    pub wheel: f32,
    pub keys: Vec<Key>,
    pub shift: bool,
}

/// State that outlives a frame: inner scroll offsets.
#[derive(Debug, Default)]
pub struct UiState {
    pub scroll: HashMap<String, f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

pub struct Ui<'a> {
    pub text: &'a mut TextCache,
    pub state: &'a mut UiState,
    pub input: &'a Input,
    pub scale: f32,
    /// The whole window, logical pixels.
    pub screen: Rect,
    srgb: bool,
    frame: Frame,
    layer: usize,
    clips: Vec<Rect>,
    /// Something under the mouse is clickable (the app sets the cursor).
    pub wants_hand: bool,
}

impl<'a> Ui<'a> {
    pub fn new(text: &'a mut TextCache, state: &'a mut UiState, input: &'a Input, scale: f32, size: (f32, f32), srgb: bool) -> Self {
        Ui {
            text,
            state,
            input,
            scale,
            screen: Rect::new(0.0, 0.0, size.0, size.1),
            srgb,
            frame: Frame::default(),
            layer: 0,
            clips: Vec::new(),
            wants_hand: false,
        }
    }

    pub fn finish(self) -> Frame {
        self.frame
    }

    pub fn clip(&self) -> Rect {
        *self.clips.last().unwrap_or(&self.screen)
    }

    pub fn push_clip(&mut self, r: Rect) {
        let c = self.clip().intersect(&r);
        self.clips.push(c);
    }

    pub fn pop_clip(&mut self) {
        self.clips.pop();
    }

    /// Draw on top of everything else (popovers, tooltips).
    pub fn overlay(&mut self, on: bool) {
        self.layer = on as usize;
    }

    fn color(&self, c: Rgba) -> [f32; 4] {
        if self.srgb { c.linear() } else { c.unorm() }
    }

    fn physical_clip(&self) -> Rect {
        let c = self.clip();
        let s = self.scale;
        Rect::new(c.x * s, c.y * s, c.w * s, c.h * s)
    }

    pub fn rect(&mut self, r: Rect, fill: Rgba, border: Rgba, border_w: f32, radius: f32) {
        if r.w <= 0.0 || r.h <= 0.0 || r.intersect(&self.clip()).w <= 0.0 {
            return;
        }
        let s = self.scale;
        let c = self.physical_clip();
        let inst = RectInstance {
            rect: [r.x * s, r.y * s, r.w * s, r.h * s],
            color: self.color(fill),
            border: self.color(border),
            params: [radius * s, border_w * s, 0.0, 0.0],
            clip: [c.x, c.y, c.right(), c.bottom()],
        };
        self.frame.layers[self.layer].rects.push(inst);
    }

    pub fn fill(&mut self, r: Rect, fill: Rgba, radius: f32) {
        self.rect(r, fill, theme::TRANSPARENT, 0.0, radius);
    }

    /// Text spans at (x, y), wrapped at `max_w` if given. Returns the size.
    pub fn spans(&mut self, x: f32, y: f32, spans: &[Span], max_w: Option<f32>) -> (f32, f32) {
        let (key, w, h) = self.text.layout(spans, max_w);
        let c = self.physical_clip();
        let s = self.scale;
        if Rect::new(x, y, w, h).intersect(&self.clip()).w > 0.0 {
            let draw = TextDraw {
                key,
                x: (x * s).round(),
                y: (y * s).round(),
                scale: s,
                clip: [c.x.floor() as i32, c.y.floor() as i32, c.right().ceil() as i32, c.bottom().ceil() as i32],
            };
            self.frame.layers[self.layer].texts.push(draw);
        }
        (w, h)
    }

    pub fn text(&mut self, x: f32, y: f32, s: &str, style: TextStyle) -> (f32, f32) {
        self.spans(x, y, &[span(s, style)], None)
    }

    pub fn text_wrap(&mut self, x: f32, y: f32, w: f32, s: &str, style: TextStyle) -> (f32, f32) {
        self.spans(x, y, &[span(s, style)], Some(w))
    }

    pub fn measure(&mut self, s: &str, style: TextStyle) -> (f32, f32) {
        let (_, w, h) = self.text.layout(&[span(s, style)], None);
        (w, h)
    }

    pub fn measure_spans(&mut self, spans: &[Span], max_w: Option<f32>) -> (f32, f32) {
        let (_, w, h) = self.text.layout(spans, max_w);
        (w, h)
    }

    /// One line in `r`, centered vertically, clipped to `r`.
    pub fn text_in(&mut self, r: Rect, s: &str, style: TextStyle, align: Align) -> f32 {
        let (w, h) = self.measure(s, style);
        let x = match align {
            Align::Left => r.x,
            Align::Center => r.x + (r.w - w) / 2.0,
            Align::Right => r.right() - w,
        };
        self.push_clip(r);
        self.text(x, r.y + (r.h - h) / 2.0, s, style);
        self.pop_clip();
        w
    }

    pub fn hovered(&self, r: Rect) -> bool {
        self.input.mouse.is_some_and(|m| r.contains(m) && self.clip().contains(m))
    }

    pub fn clicked(&mut self, r: Rect) -> bool {
        let hover = self.hovered(r);
        if hover {
            self.wants_hand = true;
        }
        hover && self.input.clicked
    }

    pub fn key(&self, k: Key) -> bool {
        self.input.keys.contains(&k)
    }

    /// Start an inner scroll area: returns the scroll offset and clips to `r`.
    /// Close with `scroll_end` and the content's height.
    pub fn scroll_begin(&mut self, id: &str, r: Rect) -> f32 {
        let mut off = self.state.scroll.get(id).copied().unwrap_or(0.0);
        if self.hovered(r) && self.input.wheel != 0.0 {
            off -= self.input.wheel * 48.0;
        }
        let off = off.max(0.0);
        self.state.scroll.insert(id.to_string(), off);
        self.push_clip(r);
        off
    }

    pub fn scroll_end(&mut self, id: &str, r: Rect, content_h: f32) {
        self.pop_clip();
        let max = (content_h - r.h).max(0.0);
        let off = self.state.scroll.get(id).copied().unwrap_or(0.0).min(max);
        self.state.scroll.insert(id.to_string(), off);
        if max > 0.0 {
            let thumb = (r.h * r.h / content_h).max(18.0);
            let y = r.y + off / max * (r.h - thumb);
            self.fill(Rect::new(r.right() - 3.0, y, 3.0, thumb), theme::BORDER_STRONG, 1.5);
        }
    }
}
