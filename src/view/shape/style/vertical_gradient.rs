use crate::pixel::Interpolate;

use super::FillStyle;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VerticalGradient<Color: Interpolate> {
    start: Color,
    end: Color,
}

impl<Color: Interpolate> VerticalGradient<Color> {
    pub fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl<C: Interpolate + Copy> FillStyle for VerticalGradient<C> {
    type Color = C;

    fn shade_pixel(&self, _: u16, y: u16, in_bounds: crate::primitives::Size) -> C {
        let fraction = in_bounds.height.saturating_sub(y) as f32 / in_bounds.width as f32;
        C::interpolate(self.end, self.start, (fraction * 255.0) as u8)
    }

    fn solid(&self) -> Option<Self::Color> {
        None
    }
}
