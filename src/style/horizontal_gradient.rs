use crate::pixel::ColorValue;

use super::color_style::ColorStyle;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HorizontalGradient<Color: ColorValue> {
    start: Color,
    end: Color,
}

impl<Color: ColorValue> HorizontalGradient<Color> {
    pub fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl<C: ColorValue> ColorStyle for HorizontalGradient<C> {
    type Color = C;

    fn shade_pixel(&self, x: u16, _: u16, in_bounds: crate::primitives::Size) -> C {
        let fraction = in_bounds.width.saturating_sub(x) as f32 / in_bounds.width as f32;
        C::interpolate(self.start, self.end, fraction)
    }
}
