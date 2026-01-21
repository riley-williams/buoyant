mod dimension;
pub mod geometry;
mod interpolate;
mod point;
mod size;
pub mod transform;
mod unit_point;

pub use dimension::*;
pub use interpolate::Interpolate;
pub use point::Point;
pub use size::Size;
pub use unit_point::UnitPoint;

#[derive(Debug, Clone)]
pub struct Pixel<C> {
    pub color: C,
    pub point: Point,
}

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics::pixelcolor::PixelColor> From<embedded_graphics::Pixel<C>> for Pixel<C> {
    fn from(value: embedded_graphics::Pixel<C>) -> Self {
        Self {
            color: value.1,
            point: value.0.into(),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics::pixelcolor::PixelColor> From<Pixel<C>> for embedded_graphics::Pixel<C> {
    fn from(value: Pixel<C>) -> Self {
        Self(value.point.into(), value.color)
    }
}
