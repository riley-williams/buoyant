use crate::{
    primitives::{
        geometry::{Circle, Line, PathEl, Rectangle, RoundedRectangle},
        Point,
    },
    render_target::{Brush, RenderTarget, Shape},
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point as EgPoint, Size},
    pixelcolor::PixelColor,
    prelude::Primitive as _,
    primitives::{
        Circle as EgCircle, Line as EgLine, PrimitiveStyle, PrimitiveStyleBuilder,
        Rectangle as EgRectangle, RoundedRectangle as EgRoundedRectangle,
    },
    Drawable, Pixel,
};

use super::{Glyph, Stroke};

#[derive(Debug)]
pub struct EmbeddedGraphicsRenderTarget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub target: D,
    clear_color: C,
    frame: Rectangle,
}

impl<D, C> EmbeddedGraphicsRenderTarget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    #[must_use]
    pub fn new(target: D, clear_color: C) -> Self {
        let frame = target.bounding_box().into();
        Self {
            target,
            clear_color,
            frame,
        }
    }
}

impl<D, C> RenderTarget for EmbeddedGraphicsRenderTarget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    // Layer is just a Rectangle that defines the clip area
    type Layer = Rectangle;
    type ColorFormat = C;

    fn reset(&mut self) {
        // FIXME: Reset clip area?
        let _ = self.target.clear(self.clear_color);
    }

    fn push_layer(&mut self) -> Self::Layer {
        // Return a layer that represents the entire drawing area
        // This could be extended to actually implement clipping in the future
        self.frame.clone()
    }

    fn pop_layer(&mut self, layer: Self::Layer) {
        self.frame = layer;
    }

    fn fill<T: Into<Self::ColorFormat>>(
        &mut self,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        // Convert the brush to the embedded_graphics color
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };

        let style = PrimitiveStyleBuilder::new().fill_color(color).build();

        // FIXME: Does frame offset replace the need for passing offset everywhere?

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
    }

    fn stroke<T: Into<Self::ColorFormat>>(
        &mut self,
        stroke: &Stroke,
        transform_offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        // Convert the brush to the embedded_graphics color
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(stroke.width)
            .stroke_color(color)
            .build();

        // FIXME: Does layer offset replace the need for passing offset everywhere?

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
    }

    fn draw_glyphs<T: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &impl crate::font::FontRender,
    ) {
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        for glyph in glyphs {
            if let Some(mask) = font.as_mask(glyph.id) {
                let render_offset = offset + Point::new(glyph.x, glyph.y);
                let x_max: i32 = render_offset.x + i32::from(mask.width);
                let mut x = render_offset.x;
                let mut y = render_offset.y;
                _ = self.target.draw_iter(mask.iter.filter_map(|p| {
                    let pixel = if p {
                        Some(Pixel(EgPoint::new(x, y), color))
                    } else {
                        None
                    };
                    x += 1;
                    if x >= x_max {
                        x = render_offset.x;
                        y += 1;
                    }
                    pixel
                }));
            } else {
                //FIXME: This only draws rectangles...support for these fonts is pending...
                let width = font.character_width('x');
                let height = font.line_height();
                let style = PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(color)
                    .build();

                self.draw_rectangle(
                    &Rectangle::new(offset, Size::new(width.into(), height.into()).into()),
                    offset,
                    &style,
                );
            }
        }
    }
}

impl<D, C> EmbeddedGraphicsRenderTarget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn draw_line(&mut self, line: &Line, offset: Point, style: &PrimitiveStyle<C>) {
        let start = EgPoint::new(line.start.x + offset.x, line.start.y + offset.y);
        let end = EgPoint::new(line.end.x + offset.x, line.end.y + offset.y);

        let eg_line = EgLine::new(start, end).into_styled(*style);
        let _ = eg_line.draw(&mut self.target);
    }

    fn draw_rectangle(&mut self, rect: &Rectangle, offset: Point, style: &PrimitiveStyle<C>) {
        let top_left = EgPoint::new(rect.origin.x + offset.x, rect.origin.y + offset.y);

        let eg_rect = EgRectangle::new(top_left, rect.size.into()).into_styled(*style);
        let _ = eg_rect.draw(&mut self.target);
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
        let _ = eg_rounded_rect.draw(&mut self.target);
    }

    fn draw_circle(&mut self, circle: &Circle, offset: Point, style: &PrimitiveStyle<C>) {
        let top_left = EgPoint::new(circle.origin.x + offset.x, circle.origin.y + offset.y);

        let eg_circle = EgCircle::new(top_left, circle.diameter).into_styled(*style);
        let _ = eg_circle.draw(&mut self.target);
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
                        let _ = eg_line.draw(&mut self.target);

                        last_point = Some(end);
                    }
                }
                PathEl::QuadTo(_control, point) => {
                    // Simplify quadratic curves to straight lines for now
                    if let Some(start) = last_point {
                        let end = Point::new(point.x + offset.x, point.y + offset.y);

                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(end.x, end.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.target);

                        last_point = Some(end);
                    }
                }
                PathEl::CurveTo(_control1, _control2, point) => {
                    // Simplify cubic curves to straight lines for now
                    if let Some(start) = last_point {
                        let end = Point::new(point.x + offset.x, point.y + offset.y);

                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(end.x, end.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.target);

                        last_point = Some(end);
                    }
                }
                PathEl::ClosePath => {
                    // Close the path by drawing a line back to the starting point
                    if let (Some(start), Some(first)) = (last_point, last_point) {
                        let start_eg = EgPoint::new(start.x, start.y);
                        let end_eg = EgPoint::new(first.x, first.y);

                        let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                        let _ = eg_line.draw(&mut self.target);
                    }
                }
            }
        }
    }
}
