use crate::font::{FontMetrics, FontRender};
use crate::primitives::{Interpolate, Pixel};
use crate::surface::{AsDrawTarget, OffsetSurface};
use crate::{
    primitives::{
        geometry::{Circle, Line, PathEl, Rectangle, RoundedRectangle},
        Point,
    },
    render_target::{Brush, RenderTarget, Shape},
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point as EgPoint,
    pixelcolor::PixelColor,
    prelude::Primitive as _,
    primitives::{
        Circle as EgCircle, Line as EgLine, PrimitiveStyle, PrimitiveStyleBuilder,
        Rectangle as EgRectangle, RoundedRectangle as EgRoundedRectangle,
    },
    Drawable,
};

use super::{Glyph, ImageBrush, Stroke, Surface};

#[derive(Debug)]
pub struct DrawTargetSurface<'a, D: DrawTarget>(&'a mut D);

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
}

#[derive(Debug)]
pub struct EmbeddedGraphicsRenderTarget<D> {
    surface: D,
    clip_rect: Rectangle,
}

impl<'a, D> EmbeddedGraphicsRenderTarget<DrawTargetSurface<'a, D>>
where
    D: DrawTarget,
    D::Color: PixelColor + Interpolate,
{
    /// Initialize an `EmbeddedGraphicsRenderTarget` from a `DrawTarget`
    #[must_use]
    pub fn new(display: &'a mut D) -> Self {
        let clip_rect = display.bounding_box().into();
        Self {
            surface: DrawTargetSurface(display),
            clip_rect,
        }
    }

    #[must_use]
    pub fn display(&self) -> &D {
        self.surface.0
    }

    #[must_use]
    pub fn display_mut(&mut self) -> &mut D {
        self.surface.0
    }
}

impl<D, C> RenderTarget for EmbeddedGraphicsRenderTarget<D>
where
    D: Surface<Color = C>,
    C: PixelColor + Interpolate,
{
    type ColorFormat = C;

    fn size(&self) -> crate::primitives::Size {
        self.surface.size()
    }

    fn clear(&mut self, color: Self::ColorFormat) {
        let _ = self.surface.draw_target().clear(color);
    }

    fn set_clip_rect(&mut self, rect: Rectangle) -> Rectangle {
        // TODO: clip to drawable area
        let old_rect = self.clip_rect.clone();
        self.clip_rect = rect;
        old_rect
    }

    fn clip_rect(&self) -> Rectangle {
        self.clip_rect.clone()
    }

    fn fill<T: Into<Self::ColorFormat>>(
        &mut self,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let mut bounding_box = shape.bounding_box();
        bounding_box.origin += transform_offset;
        if !bounding_box.intersects(&self.clip_rect) {
            return;
        }

        // Convert the brush to the embedded_graphics color
        if let Some(color) = brush.as_solid().map(Into::into) {
            let style = PrimitiveStyleBuilder::new().fill_color(color).build();

            // Handle different shape types
            if let Some(line) = shape.as_line() {
                self.draw_line(&line, transform_offset, &style);
            } else if let Some(rect) = shape.as_rect() {
                self.draw_rectangle(&rect, transform_offset, &style);
            } else if let Some(circle) = shape.as_circle() {
                self.draw_circle(&circle, transform_offset, &style);
            } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                self.draw_rounded_rectangle(&rounded_rect, transform_offset, &style);
            } else {
                // For generic shapes, convert the path elements to lines
                self.draw_path_shape(shape, transform_offset, &style);
            }
        } else if let Some(image) = brush.as_image() {
            // only support rectangles for now
            let Some(rect) = shape.as_rect() else { return };
            // FIXME: Apply brush transform and clip to shape bounds
            self.surface
                .fill_contiguous(&rect, image.color_iter().map(Into::into));
        } else {
            // no support for custom brushes yet
        }
    }

    fn stroke<T: Into<Self::ColorFormat>>(
        &mut self,
        stroke: &Stroke,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let mut bounding_box = shape.bounding_box();
        bounding_box.origin += transform_offset;
        if !bounding_box.intersects(&self.clip_rect) {
            return;
        }
        // Convert the brush to the embedded_graphics color.
        // Only solid strokes are implemented
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(stroke.width)
            .stroke_color(color)
            .build();

        if let Some(line) = shape.as_line() {
            self.draw_line(&line, transform_offset, &style);
        } else if let Some(rect) = shape.as_rect() {
            self.draw_rectangle(&rect, transform_offset, &style);
        } else if let Some(circle) = shape.as_circle() {
            self.draw_circle(&circle, transform_offset, &style);
        } else if let Some(rounded_rect) = shape.as_rounded_rect() {
            self.draw_rounded_rectangle(&rounded_rect, transform_offset, &style);
        } else {
            self.draw_path_shape(shape, transform_offset, &style);
        }
    }

    fn draw_glyphs<T: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &impl FontRender<Self::ColorFormat>,
    ) {
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        glyphs.for_each(|glyph| {
            let mut surface = OffsetSurface::new(&mut self.surface, offset + glyph.offset);
            font.draw(glyph.character, color, &mut surface);
        });
    }

    fn raw_surface(&mut self) -> &mut impl Surface<Color = Self::ColorFormat> {
        &mut self.surface
    }
}

impl<D, C> EmbeddedGraphicsRenderTarget<D>
where
    D: Surface<Color = C>,
    C: PixelColor,
{
    fn draw_line(&mut self, line: &Line, offset: Point, style: &PrimitiveStyle<C>) {
        let start = EgPoint::new(line.start.x + offset.x, line.start.y + offset.y);
        let end = EgPoint::new(line.end.x + offset.x, line.end.y + offset.y);

        let eg_line = EgLine::new(start, end).into_styled(*style);
        let _ = eg_line.draw(&mut self.surface.draw_target());
    }

    fn draw_rectangle(&mut self, rect: &Rectangle, offset: Point, style: &PrimitiveStyle<C>) {
        let top_left = EgPoint::new(rect.origin.x + offset.x, rect.origin.y + offset.y);

        let eg_rect = EgRectangle::new(top_left, rect.size.into()).into_styled(*style);
        let _ = eg_rect.draw(&mut self.surface.draw_target());
    }

    fn draw_rounded_rectangle(
        &mut self,
        rect: &RoundedRectangle,
        offset: Point,
        style: &PrimitiveStyle<C>,
    ) {
        let top_left = EgPoint::new(rect.origin.x + offset.x, rect.origin.y + offset.y);
        let eg_rect = EgRectangle::new(top_left, rect.size.into());
        let corner_radius = rect.radius;

        let eg_rounded_rect = EgRoundedRectangle::new(
            eg_rect,
            embedded_graphics::primitives::CornerRadii::new((corner_radius, corner_radius).into()),
        )
        .into_styled(*style);
        let _ = eg_rounded_rect.draw(&mut self.surface.draw_target());
    }

    fn draw_circle(&mut self, circle: &Circle, offset: Point, style: &PrimitiveStyle<C>) {
        let top_left = EgPoint::new(circle.origin.x + offset.x, circle.origin.y + offset.y);

        let eg_circle = EgCircle::new(top_left, circle.diameter).into_styled(*style);
        let _ = eg_circle.draw(&mut self.surface.draw_target());
    }

    fn draw_path_shape(&mut self, shape: &impl Shape, offset: Point, style: &PrimitiveStyle<C>) {
        // Simplistic approach: convert each path segment to a line
        let mut last_point = None;

        for element in shape.path_elements(1) {
            match element {
                PathEl::MoveTo(point) => {
                    last_point = Some(Point::new(point.x + offset.x, point.y + offset.y));
                }
                PathEl::LineTo(point) => {
                    if let Some(start) = last_point {
                        let end = Point::new(point.x + offset.x, point.y + offset.y);

                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(end.x, end.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.surface.draw_target());

                        last_point = Some(end);
                    }
                }
                PathEl::QuadTo(_control, point) => {
                    // FIXME: Simplify quadratic curves to straight lines for now
                    if let Some(start) = last_point {
                        let end = Point::new(point.x + offset.x, point.y + offset.y);

                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(end.x, end.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.surface.draw_target());

                        last_point = Some(end);
                    }
                }
                PathEl::CurveTo(_control1, _control2, point) => {
                    // FIXME: Simplify cubic curves to straight lines for now
                    if let Some(start) = last_point {
                        let end = Point::new(point.x + offset.x, point.y + offset.y);

                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(end.x, end.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.surface.draw_target());

                        last_point = Some(end);
                    }
                }
                PathEl::ClosePath => {
                    // Close the path by drawing a line back to the starting point
                    if let (Some(start), Some(first)) = (last_point, last_point) {
                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(first.x, first.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.surface.draw_target());
                    }
                }
            }
        }
    }
}
