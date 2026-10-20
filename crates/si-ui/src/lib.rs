//! `si-ui`: the native app's toolkit (native app spec §4).
//!
//! - [`painter::Ui`]: immediate-mode drawing and input, logical pixels.
//! - [`render::Renderer`]: wgpu rounded rectangles + glyphon text.
//! - [`offscreen::render_png`]: the same frame to a PNG, for screenshots.
//! - [`theme`], [`widgets`], [`geom`]: tokens, panels and chips, layout.

pub mod geom;
pub mod offscreen;
pub mod painter;
pub mod render;
pub mod text;
pub mod theme;
pub mod widgets;

pub use geom::Rect;
pub use painter::{Align, Frame, Input, Key, LAYER_BASE, LAYER_MAP, LAYER_OVERLAY, Pt, Ui, UiState};
pub use render::{IconSource, Renderer};
pub use text::{Span, TextCache, span};
pub use wgpu;
