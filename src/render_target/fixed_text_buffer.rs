use core::fmt::{Display, Formatter, Result};

use crate::primitives::{geometry::Rectangle, Pixel, Point, Size};

use super::{Brush, Glyph, RenderTarget, Shape, Stroke, Surface};

/// A fixed size text buffer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
    pub clip_rect: Rectangle,
}

impl<const W: usize, const H: usize> FixedTextBuffer<W, H> {
    pub const fn clear(&mut self) {
        self.text = [[' '; W]; H];
    }

    const fn draw_character(&mut self, point: Point, character: char) {
        let x_start = self.clip_rect.origin.x;
        let y_start = self.clip_rect.origin.y;
        let x_end = x_start + self.clip_rect.size.width as i32;
        let y_end = y_start + self.clip_rect.size.height as i32;
        #[allow(clippy::cast_sign_loss)]
        if point.x < x_end && point.y < y_end && point.x >= x_start && point.y >= y_start {
            self.text[point.y as usize][point.x as usize] = character;
        }
    }

    #[must_use]
    pub const fn size(&self) -> Size {
        Size::new(W as u32, H as u32)
    }
}

impl<const W: usize, const H: usize> Display for FixedTextBuffer<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in &self.text {
            for c in line {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Default for FixedTextBuffer<W, H> {
    fn default() -> Self {
        Self {
            text: [[' '; W]; H],
            clip_rect: Rectangle::new(Point::zero(), Size::new(W as u32, H as u32)),
        }
    }
}

impl<const W: usize, const H: usize> RenderTarget for FixedTextBuffer<W, H> {
    type ColorFormat = char;

    fn size(&self) -> Size {
        self.size()
    }

    fn clear(&mut self, _color: Self::ColorFormat) {
        self.clear();
    }

    fn set_clip_rect(&mut self, rect: Rectangle) -> Rectangle {
        let old_rect = self.clip_rect.clone();
        self.clip_rect = rect;
        old_rect
    }

    fn clip_rect(&self) -> Rectangle {
        self.clip_rect.clone()
    }

    fn fill<C: Into<Self::ColorFormat>>(
        &mut self,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let mut shape_bounds = shape.bounding_box();
        shape_bounds.origin += transform_offset;
        if !shape_bounds.intersects(&self.clip_rect) {
            return;
        }
        if let Some(rect) = shape.as_rect() {
            let Some(color) = brush.as_solid() else {
                return;
            };
            let color = color.into();
            for y in transform_offset.y..(transform_offset.y + rect.size.height as i32) {
                for x in transform_offset.x..(transform_offset.x + rect.size.width as i32) {
                    let point = Point::new(rect.origin.x + x, rect.origin.y + y);
                    self.draw_character(point, color);
                }
            }
        }
    }

    fn stroke<C: Into<Self::ColorFormat>>(
        &mut self,
        _stroke: &Stroke,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let mut shape_bounds = shape.bounding_box();
        shape_bounds.origin += transform_offset;
        if !shape_bounds.intersects(&self.clip_rect) {
            return;
        }
        if let Some(rect) = shape.as_rect() {
            let origin = Point::new(
                rect.origin.x + transform_offset.x,
                rect.origin.y + transform_offset.y,
            );
            let rect = Rectangle::new(origin, rect.size);
            let Some(color) = brush.as_solid() else {
                return;
            };
            let color = color.into();
            for y in 0..rect.size.height as i32 {
                if y == 0 || y == rect.size.height as i32 {
                    for x in 0..rect.size.width as i32 {
                        let point = Point::new(rect.origin.x + x, rect.origin.y + y);
                        self.draw_character(point, color);
                    }
                } else {
                    let point = Point::new(rect.origin.x, rect.origin.y + y);
                    self.draw_character(point, color);
                    let point =
                        Point::new(rect.origin.x + rect.size.width as i32, rect.origin.y + y);
                    self.draw_character(point, color);
                }
            }
        }
    }

    fn draw_glyphs<C: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        _brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        _font: &impl crate::font::FontRender<Self::ColorFormat>,
    ) {
        for glyph in glyphs {
            self.draw_character(offset + glyph.offset, glyph.character);
        }
    }

    fn raw_surface(&mut self) -> &mut impl Surface<Color = Self::ColorFormat> {
        self
    }
}

impl<const W: usize, const H: usize> Surface for FixedTextBuffer<W, H> {
    type Color = char;

    fn size(&self) -> Size {
        self.size()
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels
            .into_iter()
            .for_each(|p| self.draw_character(p.point, p.color));
    }
}
