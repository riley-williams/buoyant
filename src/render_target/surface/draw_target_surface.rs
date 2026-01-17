use embedded_graphics::prelude::DrawTarget;

use crate::{
    primitives::{Pixel, geometry::Rectangle},
    render_target::Surface,
};

#[derive(Debug)]
pub struct DrawTargetSurface<'a, D: DrawTarget>(pub(crate) &'a mut D);

impl<'a, D: DrawTarget> DrawTargetSurface<'a, D> {
    /// Creates a new `DrawTargetSurface` from a `DrawTarget`.
    pub fn new(display: &'a mut D) -> Self {
        Self(display)
    }

    /// Returns a reference to the underlying display.
    #[must_use]
    pub fn inner(&self) -> &D {
        self.0
    }

    /// Returns a mutable reference to the underlying display.
    #[must_use]
    pub fn inner_mut(&mut self) -> &mut D {
        self.0
    }
}

impl<D: DrawTarget> Surface for DrawTargetSurface<'_, D> {
    type Color = D::Color;

    fn size(&self) -> crate::primitives::Size {
        self.0.bounding_box().size.into()
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        _ = self.0.draw_iter(pixels.into_iter().map(Into::into));
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        _ = self.0.fill_contiguous(&area.clone().into(), colors);
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        _ = self.0.fill_solid(&area.clone().into(), color);
    }

    fn clear(&mut self, color: Self::Color) {
        _ = self.0.clear(color);
    }
}
