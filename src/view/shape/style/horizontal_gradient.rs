use crate::pixel::Interpolate;

use super::fill_style::FillStyle;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HorizontalGradient<Color> {
    start: Color,
    end: Color,
}

impl<Color: Interpolate> HorizontalGradient<Color> {
    pub fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl<C: Interpolate + Copy> FillStyle for HorizontalGradient<C> {
    type Color = C;

    fn shade_pixel(&self, x: u16, _: u16, in_bounds: crate::primitives::Size) -> C {
        let fraction = in_bounds.width.saturating_sub(x) as f32 / in_bounds.width as f32;
        C::interpolate(self.end, self.start, (fraction * 255.0) as u8)
    }

    fn solid(&self) -> Option<Self::Color> {
        None
    }
}
