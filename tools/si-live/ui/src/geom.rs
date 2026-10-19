//! Rectangles in logical pixels, with cut helpers for fixed layouts: each
//! `cut_*` removes a strip from the rectangle and returns it.

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn contains(&self, p: (f32, f32)) -> bool {
        p.0 >= self.x && p.0 < self.right() && p.1 >= self.y && p.1 < self.bottom()
    }

    pub fn shrink(&self, d: f32) -> Rect {
        self.inset(d, d, d, d)
    }

    pub fn inset(&self, l: f32, t: f32, r: f32, b: f32) -> Rect {
        Rect::new(self.x + l, self.y + t, (self.w - l - r).max(0.0), (self.h - t - b).max(0.0))
    }

    pub fn cut_top(&mut self, h: f32) -> Rect {
        let h = h.min(self.h);
        let out = Rect::new(self.x, self.y, self.w, h);
        self.y += h;
        self.h -= h;
        out
    }

    pub fn cut_bottom(&mut self, h: f32) -> Rect {
        let h = h.min(self.h);
        self.h -= h;
        Rect::new(self.x, self.y + self.h, self.w, h)
    }

    pub fn cut_left(&mut self, w: f32) -> Rect {
        let w = w.min(self.w);
        let out = Rect::new(self.x, self.y, w, self.h);
        self.x += w;
        self.w -= w;
        out
    }

    pub fn cut_right(&mut self, w: f32) -> Rect {
        let w = w.min(self.w);
        self.w -= w;
        Rect::new(self.x + self.w, self.y, w, self.h)
    }

    /// `n` equal columns with `gap` between them.
    pub fn columns(&self, n: usize, gap: f32) -> Vec<Rect> {
        let n = n.max(1);
        let w = ((self.w - gap * (n - 1) as f32) / n as f32).max(0.0);
        (0..n).map(|i| Rect::new(self.x + i as f32 * (w + gap), self.y, w, self.h)).collect()
    }

    /// `n` equal rows with `gap` between them.
    pub fn rows(&self, n: usize, gap: f32) -> Vec<Rect> {
        let n = n.max(1);
        let h = ((self.h - gap * (n - 1) as f32) / n as f32).max(0.0);
        (0..n).map(|i| Rect::new(self.x, self.y + i as f32 * (h + gap), self.w, h)).collect()
    }

    pub fn intersect(&self, o: &Rect) -> Rect {
        let x = self.x.max(o.x);
        let y = self.y.max(o.y);
        let r = self.right().min(o.right());
        let b = self.bottom().min(o.bottom());
        Rect::new(x, y, (r - x).max(0.0), (b - y).max(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuts_remove_strips() {
        let mut r = Rect::new(0.0, 0.0, 100.0, 50.0);
        let top = r.cut_top(10.0);
        let left = r.cut_left(30.0);
        assert_eq!(top, Rect::new(0.0, 0.0, 100.0, 10.0));
        assert_eq!(left, Rect::new(0.0, 10.0, 30.0, 40.0));
        assert_eq!(r, Rect::new(30.0, 10.0, 70.0, 40.0));
        let cols = r.columns(2, 10.0);
        assert_eq!(cols[1].x, 70.0);
        assert_eq!(cols[1].w, 30.0);
    }
}
