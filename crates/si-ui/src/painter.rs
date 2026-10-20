//! The immediate-mode painter. Views call it every frame with logical
//! coordinates; it records meshes, shapes, text and icons into layers (base
//! panels, then map content, then overlays) in physical pixels for the
//! renderer, and answers hover and click from this frame's input.

use crate::geom::Rect;
use crate::text::{Span, TextCache, span};
use crate::theme::{self, Rgba, TextStyle};
use std::collections::HashMap;

/// One SDF shape. `params`: radius (or half line width), border width,
/// kind (0 rounded rectangle: `rect` is x, y, w, h; 1 line: `rect` is x0,
/// y0, x1, y1), softness (blur width for glows; 0 for a crisp edge).
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub border: [f32; 4],
    pub params: [f32; 4],
    pub clip: [f32; 4],
}

/// A mesh vertex: physical position, premultiplied linear color, clip.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub pos: [f32; 2],
    pub _pad: [f32; 2],
    pub color: [f32; 4],
    pub clip: [f32; 4],
}

/// An icon the renderer rasterizes on demand (see `Renderer::add_icon`).
#[derive(Debug, Clone, Copy)]
pub struct IconDraw {
    pub id: u16,
    /// Physical position, logical size (glyphon scales it).
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub scale: f32,
    pub color: Rgba,
    pub clip: [i32; 4],
}

/// An image registered with the renderer (`Renderer::add_image`), drawn as
/// a textured quad in `rect` (physical pixels), tinted (white × alpha dims).
#[derive(Debug, Clone, Copy)]
pub struct ImageDraw {
    pub id: u16,
    pub rect: [f32; 4],
    /// Premultiplied.
    pub tint: [f32; 4],
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

/// One layer draws its meshes, then images, then shapes, then text and icons.
#[derive(Debug, Default)]
pub struct Layer {
    pub mesh: Vec<MeshVertex>,
    pub indices: Vec<u32>,
    pub images: Vec<ImageDraw>,
    pub rects: Vec<RectInstance>,
    pub texts: Vec<TextDraw>,
    pub icons: Vec<IconDraw>,
}

pub const LAYER_BASE: usize = 0;
/// Map content above its panel.
pub const LAYER_MAP: usize = 1;
pub const LAYER_OVERLAY: usize = 2;

/// Everything one frame draws, in order.
#[derive(Debug, Default)]
pub struct Frame {
    pub layers: [Layer; 3],
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
    Tab,
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

/// A point in logical pixels.
pub type Pt = (f32, f32);

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
    /// Seconds since the app started, for animation.
    pub time: f32,
    /// Something is animating: the app keeps redrawing.
    pub animating: bool,
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
            time: 0.0,
            animating: false,
        }
    }

    /// 0 → 1 → 0 over `period` seconds; marks the frame as animating.
    pub fn pulse(&mut self, period: f32) -> f32 {
        self.animating = true;
        0.5 - 0.5 * (self.time * std::f32::consts::TAU / period).cos()
    }

    /// Draw into a layer: `LAYER_BASE`, `LAYER_MAP` or `LAYER_OVERLAY`.
    pub fn layer(&mut self, n: usize) {
        self.layer = n.min(2);
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
        self.layer = if on { LAYER_OVERLAY } else { LAYER_BASE };
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

    /// A soft glow around `r`: its color fades out over `blur` pixels.
    pub fn glow(&mut self, r: Rect, color: Rgba, radius: f32, blur: f32) {
        let s = self.scale;
        let c = self.physical_clip();
        let g = r.inset(-blur, -blur, -blur, -blur);
        if g.intersect(&self.clip()).w <= 0.0 {
            return;
        }
        let inst = RectInstance {
            rect: [r.x * s, r.y * s, r.w * s, r.h * s],
            color: self.color(color),
            border: [0.0; 4],
            params: [radius * s, 0.0, 0.0, blur * s],
            clip: [c.x, c.y, c.right(), c.bottom()],
        };
        self.frame.layers[self.layer].rects.push(inst);
    }

    /// A line from `a` to `b` with round ends; `blur` > 0 makes it a glow.
    pub fn line(&mut self, a: Pt, b: Pt, width: f32, color: Rgba, blur: f32) {
        let s = self.scale;
        let c = self.physical_clip();
        let inst = RectInstance {
            rect: [a.0 * s, a.1 * s, b.0 * s, b.1 * s],
            color: self.color(color),
            border: [0.0; 4],
            params: [width * s / 2.0, 0.0, 1.0, blur * s],
            clip: [c.x, c.y, c.right(), c.bottom()],
        };
        self.frame.layers[self.layer].rects.push(inst);
    }

    /// A filled circle.
    pub fn circle(&mut self, center: Pt, r: f32, fill: Rgba, border: Rgba, border_w: f32) {
        self.rect(Rect::new(center.0 - r, center.1 - r, 2.0 * r, 2.0 * r), fill, border, border_w, r);
    }

    fn mesh_vertex(&self, p: Pt, color: Rgba) -> MeshVertex {
        let s = self.scale;
        let c = self.physical_clip();
        let col = self.color(color);
        MeshVertex { pos: [p.0 * s, p.1 * s], _pad: [0.0; 2], color: [col[0] * col[3], col[1] * col[3], col[2] * col[3], col[3]], clip: [c.x, c.y, c.right(), c.bottom()] }
    }

    /// A polygon as a fan from `center` (it must see every edge: convex or
    /// star-shaped), colored `center_color` at the center fading to
    /// `edge_color` at the rim.
    pub fn polygon(&mut self, points: &[Pt], center: Pt, center_color: Rgba, edge_color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let base = self.frame.layers[self.layer].mesh.len() as u32;
        let mut verts = vec![self.mesh_vertex(center, center_color)];
        verts.extend(points.iter().map(|p| self.mesh_vertex(*p, edge_color)));
        let n = points.len() as u32;
        let layer = &mut self.frame.layers[self.layer];
        layer.mesh.extend(verts);
        for i in 0..n {
            layer.indices.extend([base, base + 1 + i, base + 1 + (i + 1) % n]);
        }
    }

    /// A quad `a b c d` (in order around it), each corner its own color:
    /// gradients, chart bands.
    pub fn quad(&mut self, pts: [Pt; 4], colors: [Rgba; 4]) {
        let base = self.frame.layers[self.layer].mesh.len() as u32;
        let verts: Vec<MeshVertex> = pts.iter().zip(colors).map(|(p, c)| self.mesh_vertex(*p, c)).collect();
        let layer = &mut self.frame.layers[self.layer];
        layer.mesh.extend(verts);
        layer.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    pub fn triangle(&mut self, a: Pt, b: Pt, c: Pt, color: Rgba) {
        let base = self.frame.layers[self.layer].mesh.len() as u32;
        let verts = [self.mesh_vertex(a, color), self.mesh_vertex(b, color), self.mesh_vertex(c, color)];
        let layer = &mut self.frame.layers[self.layer];
        layer.mesh.extend(verts);
        layer.indices.extend([base, base + 1, base + 2]);
    }

    /// An arrow from `a` to `b`.
    pub fn arrow(&mut self, a: Pt, b: Pt, width: f32, color: Rgba) {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let len = (dx * dx + dy * dy).sqrt().max(1e-3);
        let (ux, uy) = (dx / len, dy / len);
        let head = (width * 3.5).min(len * 0.5);
        let base = (b.0 - ux * head, b.1 - uy * head);
        self.line(a, base, width, color, 0.0);
        let (px, py) = (-uy * head * 0.6, ux * head * 0.6);
        self.triangle(b, (base.0 + px, base.1 + py), (base.0 - px, base.1 - py), color);
    }

    /// An image registered with the renderer, stretched over `r`, at
    /// `alpha` (1 = as is; less dims it toward the background).
    pub fn image(&mut self, r: Rect, id: u16, alpha: f32) {
        if r.w <= 0.0 || r.h <= 0.0 || r.intersect(&self.clip()).w <= 0.0 {
            return;
        }
        let s = self.scale;
        let c = self.physical_clip();
        let a = alpha.clamp(0.0, 1.0);
        let draw = ImageDraw { id, rect: [r.x * s, r.y * s, r.w * s, r.h * s], tint: [a, a, a, a], clip: [c.x, c.y, c.right(), c.bottom()] };
        self.frame.layers[self.layer].images.push(draw);
    }

    /// An icon registered with the renderer, drawn in `r`, tinted `color`
    /// (color icons ignore the tint).
    pub fn icon(&mut self, r: Rect, id: u16, color: Rgba) {
        if r.intersect(&self.clip()).w <= 0.0 {
            return;
        }
        let s = self.scale;
        let c = self.physical_clip();
        let draw = IconDraw {
            id,
            x: (r.x * s).round(),
            y: (r.y * s).round(),
            w: r.w,
            h: r.h,
            scale: s,
            color,
            clip: [c.x.floor() as i32, c.y.floor() as i32, c.right().ceil() as i32, c.bottom().ceil() as i32],
        };
        self.frame.layers[self.layer].icons.push(draw);
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
