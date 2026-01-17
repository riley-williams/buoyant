use crate::{
    primitives::{Pixel, Size, geometry::Rectangle},
    render_target::Surface,
};

/// A surface which draws with a specified offset.
#[derive(Debug)]
pub struct ClippedSurface<S> {
    surface: S,
    clip_rect: Rectangle,
}

impl<S: Surface> ClippedSurface<S> {
    pub fn new(surface: S, clip_rect: Rectangle) -> Self {
        Self { surface, clip_rect }
    }
}

impl<S: Surface> Surface for ClippedSurface<S> {
    type Color = S::Color;

    fn size(&self) -> Size {
        self.clip_rect.size
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.surface.draw_iter(pixels.into_iter().filter_map(|p| {
            if self.clip_rect.contains(&p.point) {
                Some(p)
            } else {
                None
            }
        }));
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I)
    where
        I: IntoIterator<Item = Self::Color>,
    {
        if let Some(rect) = self.clip_rect.intersection(area) {
            self.surface.fill_contiguous(&rect, colors);
        }
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        if let Some(rect) = self.clip_rect.intersection(area) {
            self.surface.fill_solid(&rect, color);
        }
    }
}
