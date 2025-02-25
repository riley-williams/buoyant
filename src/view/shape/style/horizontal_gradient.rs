use crate::primitives::Interpolate;

use super::FillStyle;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct HorizontalGradient<Color> {
    start: Color,
    end: Color,
}

impl<Color: Interpolate> HorizontalGradient<Color> {
    #[inline]
    pub const fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl<C: Interpolate + Copy> FillStyle for HorizontalGradient<C> {
    type Color = C;

    #[inline]
    fn shade_pixel(&self, x: u16, _: u16, in_bounds: crate::primitives::Size) -> C {
        let fraction = f32::from(in_bounds.width.saturating_sub(x)) / f32::from(in_bounds.width);
        C::interpolate(self.end, self.start, (fraction * 255.0) as u8)
    }

    #[inline]
    fn solid(&self) -> Option<Self::Color> {
        None
    }
}
