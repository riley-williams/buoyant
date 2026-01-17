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
        // TODO: Is this really the correct / expected behavior?
        let mut size = self.surface.size();
        size.width -= self.offset.x as u32;
        size.height -= self.offset.y as u32;
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
