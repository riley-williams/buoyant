use crate::{
    primitives::{Pixel, Point, Size, geometry::Rectangle},
    render_target::Surface,
};

/// A surface which draws with a specified offset.
#[derive(Debug)]
pub struct OffsetSurface<S> {
    surface: S,
    offset: Point,
}

impl<S: Surface> OffsetSurface<S> {
    pub fn new(surface: S, offset: Point) -> Self {
        Self { surface, offset }
    }
}

impl<S: Surface> Surface for OffsetSurface<S> {
    type Color = S::Color;

    fn size(&self) -> Size {
        // The extent of the wrapped surface, expressed in this surface's
        // coordinate space.
        let mut size = self.surface.size();
        size.width = size.width.saturating_add_signed(-self.offset.x);
        size.height = size.height.saturating_add_signed(-self.offset.y);
        size
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.surface.draw_iter(pixels.into_iter().map(|mut p| {
            p.point += self.offset;
            p
        }));
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let origin = area.origin + self.offset;
        let area = Rectangle::new(origin, area.size);
        self.surface.fill_contiguous(&area, colors);
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        let origin = area.origin + self.offset;
        let area = Rectangle::new(origin, area.size);
        self.surface.fill_solid(&area, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FixedSurface;

    impl Surface for FixedSurface {
        type Color = u8;

        fn size(&self) -> Size {
            Size::new(64, 48)
        }

        fn draw_iter<I>(&mut self, _pixels: I)
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
        }
    }

    #[test]
    fn size_shrinks_with_a_positive_offset() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(10, 5));

        assert_eq!(surface.size(), Size::new(54, 43));
    }

    #[test]
    fn size_grows_with_a_negative_offset() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(-10, -5));

        assert_eq!(surface.size(), Size::new(74, 53));
    }

    #[test]
    fn size_saturates_at_zero() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(100, 100));

        assert_eq!(surface.size(), Size::new(0, 0));
    }
}
