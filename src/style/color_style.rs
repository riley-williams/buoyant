use crate::{pixel::PixelColor, primitives::Size};

pub trait ColorStyle: Copy + PartialEq {
    type Color: Copy + PartialEq;
    /// Shade a pixel at the given relative coordinates
    fn shade_pixel(&self, x: u16, y: u16, in_bounds: Size) -> Self::Color;

    /// If this style renders as a solid color, it returns that color
    fn solid(&self) -> Option<Self::Color>;
}

impl<C: PixelColor> ColorStyle for C {
    type Color = C;

    fn shade_pixel(&self, _: u16, _: u16, _: Size) -> Self::Color {
        *self
    }

    fn solid(&self) -> Option<Self::Color> {
        Some(*self)
    }
}
