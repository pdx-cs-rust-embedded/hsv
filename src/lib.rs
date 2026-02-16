//! HSV to sRGB conversion.
//!
//! Thanks to Claude Code Opus 4.6 for this.

#[derive(Clone, Copy)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Hsv {
    /// Convert HSV to sRGB. H is a unit angle in [0..1).
    pub fn to_rgb(self) -> (f32, f32, f32) {
        let c = self.s * self.v;
        let h6 = self.h * 6.0;
        let sector = h6 as u32;
        let frac = h6 - sector as f32;

        // x = c * (1 - |h6 mod 2 - 1|)
        // In even sectors frac goes 0→1, so x = c * (1 - (1 - frac)) = c * frac
        // In odd sectors frac goes 0→1, so x = c * (1 - frac)
        let x = if sector & 1 == 0 { c * frac } else { c * (1.0 - frac) };
        let m = self.v - c;

        let (r1, g1, b1) = match sector {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (c, 0.0, x), // h >= 1.0 wraps
        };

        (r1 + m, g1 + m, b1 + m)
    }
}
