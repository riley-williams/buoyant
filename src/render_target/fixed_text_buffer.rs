use core::fmt::{Display, Formatter, Result};

use crate::{
    primitives::{
        Pixel, Point, Size,
        geometry::Rectangle,
        transform::{CoordinateSpaceTransform as _, LinearTransform},
    },
    render_target::{LayerConfig, LayerHandle},
};

use super::{Brush, Glyph, RenderTarget, Shape, Stroke, Surface};

/// A fixed size text buffer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
    pub active_layer: LayerConfig<char>,
    /// A flag tracking active animation. Initially true to ensure first render occurs.
    active_animation: bool,
}

impl<const W: usize, const H: usize> FixedTextBuffer<W, H> {
    pub const fn clear(&mut self) {
        self.text = [[' '; W]; H];
    }

    const fn draw_character(&mut self, point: Point, character: char) {
        if self.active_layer.clip_rect.contains(&point) {
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
            active_layer: LayerConfig::new_sized(Size::new(W as u32, H as u32)),
            active_animation: true,
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

    fn with_layer<LayerFn, DrawFn>(&mut self, layer_fn: LayerFn, draw_fn: DrawFn)
    where
        LayerFn: FnOnce(LayerHandle<Self::ColorFormat>) -> LayerHandle<Self::ColorFormat>,
        DrawFn: FnOnce(&mut Self),
    {
        let layer = self.active_layer.clone();
        let mut new_layer = self.active_layer.clone();
        layer_fn(LayerHandle::new(&mut new_layer));
        self.active_layer = new_layer;
        draw_fn(self);
        self.active_layer = layer;
    }

    fn clip_rect(&self) -> Rectangle {
        self.active_layer
            .clip_rect
            .applying_inverse(&self.active_layer.transform)
    }

    fn alpha(&self) -> u8 {
        255
    }

    fn report_active_animation(&mut self) {
        self.active_animation = true;
    }

    fn clear_animation_status(&mut self) -> bool {
        let was_active = self.active_animation;
        self.active_animation = false;
        was_active
    }

    fn fill<C: Into<Self::ColorFormat>>(
        &mut self,
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = C>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let transform = transform.into().applying(&self.active_layer.transform);
        let bounding_box = shape.bounding_box().applying(&transform);
        if !bounding_box.intersects(&self.active_layer.clip_rect) {
            return;
        }

        if let Some(rect) = shape.as_rect() {
            let Some(color) = brush.as_solid() else {
                return;
            };
            let color = color.into();
            for y in transform.offset.y..(transform.offset.y + rect.size.height as i32) {
                for x in transform.offset.x..(transform.offset.x + rect.size.width as i32) {
                    let point = Point::new(rect.origin.x + x, rect.origin.y + y);
                    self.draw_character(point, color);
                }
            }
        }
    }

    fn stroke<C: Into<Self::ColorFormat>>(
        &mut self,
        _stroke: &Stroke,
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = C>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let transform = transform.into().applying(&self.active_layer.transform);
        let bounding_box = shape.bounding_box().applying(&transform);
        if !bounding_box.intersects(&self.active_layer.clip_rect) {
            return;
        }

        if let Some(rect) = shape.as_rect() {
            let origin = Point::new(
                rect.origin.x + transform.offset.x,
                rect.origin.y + transform.offset.y,
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

    fn draw_glyphs<C: Into<Self::ColorFormat>, F: crate::font::FontRender<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        _brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        _font: &F,
        _font_attributes: &F::Attributes,
        _conservative_bounds: &Rectangle,
    ) {
        let offset = offset.applying(&self.active_layer.transform);
        for glyph in glyphs {
            self.draw_character(offset + glyph.offset, glyph.character);
        }
    }

    fn raw_surface(&mut self) -> impl Surface<Color = Self::ColorFormat> + '_ {
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
