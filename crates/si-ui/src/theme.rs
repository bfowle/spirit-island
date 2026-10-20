//! Theme tokens: colors (sRGB), spacing, radii and the type scale. Dark
//! background, panel cards with headers, one accent (purple) for "yours /
//! now", amber for warnings, red only for loss (native app spec §3).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const fn hex(v: u32) -> Rgba {
        Rgba((v >> 16) as u8, (v >> 8) as u8, v as u8, 255)
    }

    pub const fn alpha(self, a: u8) -> Rgba {
        Rgba(self.0, self.1, self.2, a)
    }

    /// Linear RGBA floats for an sRGB render target (the GPU re-encodes).
    pub fn linear(self) -> [f32; 4] {
        let f = |c: u8| {
            let c = c as f32 / 255.0;
            if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        [f(self.0), f(self.1), f(self.2), self.3 as f32 / 255.0]
    }

    pub fn unorm(self) -> [f32; 4] {
        [self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0, self.3 as f32 / 255.0]
    }
}

pub const TRANSPARENT: Rgba = Rgba(0, 0, 0, 0);

// Surfaces.
pub const BG: Rgba = Rgba::hex(0x0e0f14);
pub const PANEL: Rgba = Rgba::hex(0x15161d);
pub const PANEL_HEAD: Rgba = Rgba::hex(0x1b1c25);
pub const RAISED: Rgba = Rgba::hex(0x20212c);
pub const INSET: Rgba = Rgba::hex(0x101117);
pub const BORDER: Rgba = Rgba::hex(0x262835);
pub const BORDER_STRONG: Rgba = Rgba::hex(0x363948);

// Text.
pub const TEXT: Rgba = Rgba::hex(0xe4e6ef);
pub const TEXT_2: Rgba = Rgba::hex(0xa6aabb);
pub const MUTED: Rgba = Rgba::hex(0x6e7287);
pub const FAINT: Rgba = Rgba::hex(0x4a4d5e);

// Meaning.
pub const ACCENT: Rgba = Rgba::hex(0x8b7cf6);
pub const ACCENT_DIM: Rgba = Rgba::hex(0x3b3566);
pub const AMBER: Rgba = Rgba::hex(0xf2a93b);
pub const RED: Rgba = Rgba::hex(0xef5350);
pub const GREEN: Rgba = Rgba::hex(0x5ccf8a);
pub const BLUE: Rgba = Rgba::hex(0x5aa9e6);

// Spacing and shape.
pub const GAP: f32 = 8.0;
pub const PAD: f32 = 10.0;
pub const RADIUS: f32 = 6.0;
pub const RADIUS_SM: f32 = 4.0;

/// One text style. Sizes in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub size: f32,
    pub line: f32,
    pub mono: bool,
    pub weight: u16,
    pub color: Rgba,
}

impl TextStyle {
    pub const fn color(self, color: Rgba) -> TextStyle {
        TextStyle { color, ..self }
    }

    pub const fn bold(self) -> TextStyle {
        TextStyle { weight: 600, ..self }
    }

    pub const fn mono(self) -> TextStyle {
        TextStyle { mono: true, ..self }
    }

    pub const fn size(self, size: f32) -> TextStyle {
        TextStyle { size, line: size * 1.3, ..self }
    }
}

pub const BODY: TextStyle = TextStyle { size: 13.0, line: 18.0, mono: false, weight: 400, color: TEXT };
pub const SMALL: TextStyle = TextStyle { size: 11.5, line: 15.0, mono: false, weight: 400, color: TEXT_2 };
/// Panel header titles.
pub const HEAD: TextStyle = TextStyle { size: 11.5, line: 15.0, mono: false, weight: 600, color: TEXT_2 };
pub const TITLE: TextStyle = TextStyle { size: 15.0, line: 20.0, mono: false, weight: 600, color: TEXT };
pub const NUM: TextStyle = TextStyle { size: 13.0, line: 18.0, mono: true, weight: 400, color: TEXT };
pub const NUM_BIG: TextStyle = TextStyle { size: 26.0, line: 30.0, mono: true, weight: 600, color: TEXT };
