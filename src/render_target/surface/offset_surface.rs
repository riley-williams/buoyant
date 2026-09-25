use crate::{
    primitives::{Pixel, Point, geometry::Rectangle},
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

    fn bounding_box(&self) -> Rectangle {
        // The extent of the wrapped surface, expressed in this surface's
        // coordinate space.
        let bounds = self.surface.bounding_box();
        Rectangle::new(bounds.origin - self.offset, bounds.size)
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
    use crate::{primitives::Size, render_target::surface::Visibility};

    #[derive(Debug)]
    struct FixedSurface;

    impl Surface for FixedSurface {
        type Color = u8;

        fn bounding_box(&self) -> Rectangle {
            Rectangle::new(Point::zero(), Size::new(64, 48))
        }

        fn draw_iter<I>(&mut self, _pixels: I)
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
        }
    }

    #[test]
    fn bounding_box_moves_back_with_a_positive_offset() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(10, 5));

        assert_eq!(
            surface.bounding_box(),
            Rectangle::new(Point::new(-10, -5), Size::new(64, 48))
        );
    }

    #[test]
    fn bounding_box_moves_forward_with_a_negative_offset() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(-10, -5));

        assert_eq!(
            surface.bounding_box(),
            Rectangle::new(Point::new(10, 5), Size::new(64, 48))
        );
    }

    #[test]
    fn visibility_is_reported_in_the_offset_coordinate_space() {
        let surface = OffsetSurface::new(FixedSurface, Point::new(10, 5));

        // Fully inside the wrapped surface once the offset is applied.
        assert_eq!(
            surface.visibility_of(&Rectangle::new(Point::zero(), Size::new(10, 10))),
            Visibility::Full
        );

        // Runs off the left edge of the wrapped surface.
        assert_eq!(
            surface.visibility_of(&Rectangle::new(Point::new(-20, 0), Size::new(20, 10))),
            Visibility::Clipped(Rectangle::new(Point::new(-10, 0), Size::new(10, 10)))
        );

        assert_eq!(
            surface.visibility_of(&Rectangle::new(Point::new(-100, -100), Size::new(10, 10))),
            Visibility::Hidden
        );
    }
}
