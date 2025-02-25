use crate::primitives::Interpolate;

use super::FillStyle;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct VerticalGradient<Color: Interpolate> {
    start: Color,
    end: Color,
}

impl<Color: Interpolate> VerticalGradient<Color> {
    #[must_use]
    pub const fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl<C: Interpolate + Copy> FillStyle for VerticalGradient<C> {
    type Color = C;

    fn shade_pixel(&self, _: u16, y: u16, in_bounds: crate::primitives::Size) -> C {
        let fraction = f32::from(in_bounds.height.saturating_sub(y)) / f32::from(in_bounds.width);
        C::interpolate(self.end, self.start, (fraction * 255.0) as u8)
    }

    fn solid(&self) -> Option<Self::Color> {
        None
    }
}
