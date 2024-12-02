use crate::primitives::Size;

pub struct ShapeStyle<C, F: FillStyle<Color = C>> {
    pub fill_style: Option<F>,
    pub stroke_style: Option<StrokeStyle<C>>,
}

pub trait FillStyle {
    type Color: Copy;
    /// Shade a pixel at the given relative coordinates
    fn shade_pixel(&self, x: u16, y: u16, in_bounds: Size) -> Self::Color;

    /// If this style renders as a solid color, it returns that color
    fn solid(&self) -> Option<Self::Color>;
}

pub struct StrokeStyle<C> {
    pub color: C,
    pub width: u16,
}

#[cfg(feature = "embedded-graphics")]
impl<T: embedded_graphics_core::pixelcolor::PixelColor> FillStyle for T {
    type Color = T;

    fn shade_pixel(&self, _: u16, _: u16, _: Size) -> Self::Color {
        *self
    }

    fn solid(&self) -> Option<Self::Color> {
        Some(*self)
    }
}

impl<T: Copy> StrokeStyle<T> {
    pub fn new(color: T, width: u16) -> Self {
        Self { color, width }
    }
}

// TODO: Come back and clean this up to not only be implemented for embedded-graphics
#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics_core::pixelcolor::PixelColor, T: FillStyle<Color = C>> ShapeStyle<C, T> {
    pub fn fill(color: T) -> Self {
        Self {
            fill_style: Some(color),
            stroke_style: None,
        }
    }

    pub fn stroke(color: C, width: u16) -> Self {
        Self {
            fill_style: None,
            stroke_style: Some(StrokeStyle::new(color, width)),
        }
    }

    pub fn with_fill(mut self, fill: T) -> Self {
        self.fill_style = Some(fill);
        self
    }

    pub fn with_stroke(mut self, stroke: C, width: u16) -> Self {
        self.stroke_style = Some(StrokeStyle::new(stroke, width));
        self
    }
}
