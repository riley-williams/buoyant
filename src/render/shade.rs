use crate::primitives::Point;

pub trait Shader {
    type Color;
    fn shade(&self, point: Point) -> Self::Color;
}

pub trait ShadeSolid: Shader<Color = Self> {
    fn color(&self) -> Self;
}

impl<T: ShadeSolid> Shader for T {
    type Color = T;
    #[inline(always)]
    fn shade(&self, _: Point) -> T {
        self.color()
    }
}

#[cfg(feature = "embedded-graphics")]
macro_rules! implement_shade_solid_for_eg_colors {
    ($($color:ty),+) => {
        $(
            impl ShadeSolid for $color {
                fn color(&self) -> $color {
                    *self
                }
            }
        )*
    };
}

#[cfg(feature = "embedded-graphics")]
implement_shade_solid_for_eg_colors!(
    embedded_graphics_core::pixelcolor::BinaryColor,
    embedded_graphics_core::pixelcolor::Gray2,
    embedded_graphics_core::pixelcolor::Gray4,
    embedded_graphics_core::pixelcolor::Gray8,
    embedded_graphics_core::pixelcolor::Rgb555,
    embedded_graphics_core::pixelcolor::Rgb565,
    embedded_graphics_core::pixelcolor::Rgb666,
    embedded_graphics_core::pixelcolor::Rgb888
);

#[cfg(feature = "crossterm")]
impl ShadeSolid for crossterm::style::Colors {
    fn color(&self) -> crossterm::style::Colors {
        *self
    }
}
