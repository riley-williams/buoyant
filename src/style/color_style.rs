use rgb::RGB8;

use crate::primitives::Size;

pub trait ColorStyle: Clone + Copy + PartialEq {
    /// Shade a pixel at the given relative coordinates
    fn shade_pixel(&self, x: u16, y: u16, in_bounds: Size) -> RGB8;
}

impl ColorStyle for RGB8 {
    fn shade_pixel(&self, _: u16, _: u16, _: Size) -> RGB8 {
        *self
    }
}
