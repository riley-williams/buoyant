use rgb::RGB8;

use super::color_style::ColorStyle;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HorizontalGradient {
    start: rgb::RGB8,
    end: rgb::RGB8,
}

impl HorizontalGradient {
    pub fn new(start: rgb::RGB8, end: rgb::RGB8) -> Self {
        Self { start, end }
    }
}

impl ColorStyle for HorizontalGradient {
    fn shade_pixel(&self, x: u16, _: u16, in_bounds: crate::primitives::Size) -> rgb::RGB8 {
        let fraction = in_bounds.width.saturating_sub(x) as f32 / in_bounds.width as f32;
        let inv = 1.0 - fraction;
        RGB8::new(
            (inv * self.start.r as f32 + fraction * self.end.r as f32) as u8,
            (inv * self.start.g as f32 + fraction * self.end.g as f32) as u8,
            (inv * self.start.b as f32 + fraction * self.end.b as f32) as u8,
        )
    }
}
