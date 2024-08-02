use crate::primitives::Size;

pub trait FillStyle {
    type Color: Copy;
    /// Shade a pixel at the given relative coordinates
    fn shade_pixel(&self, x: u16, y: u16, in_bounds: Size) -> Self::Color;

    /// If this style renders as a solid color, it returns that color
    fn solid(&self) -> Option<Self::Color>;
}
