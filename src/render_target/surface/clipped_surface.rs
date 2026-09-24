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
        let Some(rect) = self.clip_rect.intersection(area) else {
            return;
        };

        if rect == *area {
            self.surface.fill_contiguous(area, colors);
        } else {
            let colors = Cropped::new(colors.into_iter(), area, &rect);
            self.surface.fill_contiguous(&rect, colors);
        }
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) {
        if let Some(rect) = self.clip_rect.intersection(area) {
            self.surface.fill_solid(&rect, color);
        }
    }
}

/// Restricts a row-major color iterator spanning `area` to the sub-rectangle `crop`.
///
/// `crop` must be contained within `area`.
struct Cropped<I> {
    colors: I,
    /// Column of the next color, relative to the left edge of `crop`.
    x: u32,
    /// Row of the next color, relative to the top edge of `crop`.
    y: u32,
    size: Size,
    /// Colors discarded between the end of one cropped row and the start of the next.
    row_skip: usize,
}

impl<I: Iterator> Cropped<I> {
    fn new(mut colors: I, area: &Rectangle, crop: &Rectangle) -> Self {
        let stride = area.size.width as usize;
        let initial_skip = (crop.origin.y - area.origin.y) as usize * stride
            + (crop.origin.x - area.origin.x) as usize;

        if initial_skip > 0 {
            colors.nth(initial_skip - 1);
        }

        Self {
            colors,
            x: 0,
            y: 0,
            size: crop.size,
            row_skip: stride - crop.size.width as usize,
        }
    }
}

impl<I: Iterator> Iterator for Cropped<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.size.height || self.size.width == 0 {
            return None;
        }

        if self.x < self.size.width {
            self.x += 1;

            self.colors.next()
        } else {
            self.x = 1;
            self.y += 1;

            if self.y < self.size.height {
                self.colors.nth(self.row_skip)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Point;
    use std::vec::Vec;

    /// A surface which records every pixel it is asked to draw.
    #[derive(Debug, Default)]
    struct RecordingSurface {
        pixels: Vec<(Point, u32)>,
    }

    impl Surface for RecordingSurface {
        type Color = u32;

        fn size(&self) -> Size {
            Size::new(16, 16)
        }

        fn draw_iter<I>(&mut self, pixels: I)
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            self.pixels
                .extend(pixels.into_iter().map(|p| (p.point, p.color)));
        }
    }

    /// Colors are laid out row-major over `area`, so the color at a point is
    /// fully determined by that point.
    fn expected(area: &Rectangle, points: &[(i32, i32)]) -> Vec<(Point, u32)> {
        points
            .iter()
            .map(|&(x, y)| {
                let index =
                    (y - area.origin.y) as u32 * area.size.width + (x - area.origin.x) as u32;
                (Point::new(x, y), index)
            })
            .collect()
    }

    #[test]
    fn fill_contiguous_crops_colors_to_the_clip_rect() {
        let area = Rectangle::new(Point::new(0, 0), Size::new(5, 4));
        let clip = Rectangle::new(Point::new(2, 1), Size::new(3, 2));

        let mut recording = RecordingSurface::default();
        ClippedSurface::new(&mut recording, clip).fill_contiguous(&area, 0..20);

        assert_eq!(
            recording.pixels,
            expected(&area, &[(2, 1), (3, 1), (4, 1), (2, 2), (3, 2), (4, 2)])
        );
    }

    #[test]
    fn fill_contiguous_crops_colors_when_the_area_extends_past_the_clip_rect() {
        let area = Rectangle::new(Point::new(-2, -1), Size::new(6, 5));
        let clip = Rectangle::new(Point::new(0, 0), Size::new(2, 2));

        let mut recording = RecordingSurface::default();
        ClippedSurface::new(&mut recording, clip).fill_contiguous(&area, 0..30);

        assert_eq!(
            recording.pixels,
            expected(&area, &[(0, 0), (1, 0), (0, 1), (1, 1)])
        );
    }

    #[test]
    fn fill_contiguous_passes_through_an_area_inside_the_clip_rect() {
        let area = Rectangle::new(Point::new(1, 1), Size::new(2, 2));
        let clip = Rectangle::new(Point::new(0, 0), Size::new(8, 8));

        let mut recording = RecordingSurface::default();
        ClippedSurface::new(&mut recording, clip).fill_contiguous(&area, 0..4);

        assert_eq!(
            recording.pixels,
            expected(&area, &[(1, 1), (2, 1), (1, 2), (2, 2)])
        );
    }

    #[test]
    fn fill_contiguous_stops_at_the_end_of_a_short_color_iterator() {
        let area = Rectangle::new(Point::new(0, 0), Size::new(5, 4));
        let clip = Rectangle::new(Point::new(2, 1), Size::new(3, 2));

        let mut recording = RecordingSurface::default();
        // Only enough colors to reach the second pixel of the cropped region.
        ClippedSurface::new(&mut recording, clip).fill_contiguous(&area, 0..9);

        assert_eq!(recording.pixels, expected(&area, &[(2, 1), (3, 1)]));
    }

    #[test]
    fn fill_contiguous_skips_an_area_outside_the_clip_rect() {
        let area = Rectangle::new(Point::new(10, 10), Size::new(2, 2));
        let clip = Rectangle::new(Point::new(0, 0), Size::new(4, 4));

        let mut recording = RecordingSurface::default();
        ClippedSurface::new(&mut recording, clip).fill_contiguous(&area, 0..4);

        assert!(recording.pixels.is_empty());
    }
}
