use crate::{
    primitives::{Pixel, geometry::Rectangle},
    render_target::Surface,
};

/// A surface which reports a restricted drawing region without enforcing it.
///
/// Drawing calls are passed through unchecked, so callers must confine their
/// drawing to the region reported by [`Surface::visibility_of`]. Use
/// [`ClippedSurface`](super::ClippedSurface) when the caller cannot do that.
#[derive(Debug)]
pub struct BoundedSurface<S> {
    surface: S,
    bounds: Rectangle,
}

impl<S: Surface> BoundedSurface<S> {
    pub fn new(surface: S, bounds: Rectangle) -> Self {
        Self { surface, bounds }
    }
}

impl<S: Surface> Surface for BoundedSurface<S> {
    type Color = S::Color;

    fn bounding_box(&self) -> Rectangle {
        self.bounds
            .intersection(&self.surface.bounding_box())
            .unwrap_or_default()
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.surface.draw_iter(pixels);
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.surface.fill_contiguous(area, colors);
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        self.surface.fill_solid(area, color);
    }
}
