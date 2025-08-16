use crate::font::FontRender;
use crate::primitives::transform::CoordinateSpaceTransform;
use crate::primitives::{transform::LinearTransform, Interpolate, Pixel};
use crate::render_target::{LayerConfig, LayerHandle};
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
    active_layer: LayerConfig,
}

impl<'a, D> EmbeddedGraphicsRenderTarget<DrawTargetSurface<'a, D>>
where
    D: DrawTarget,
    D::Color: PixelColor + Interpolate,
{
    /// Initialize an `EmbeddedGraphicsRenderTarget` from a `DrawTarget`
    #[must_use]
    pub fn new(display: &'a mut D) -> Self {
        let clip_rect = display.bounding_box();
        Self {
            surface: DrawTargetSurface(display),
            active_layer: LayerConfig::new_clip(clip_rect),
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

    fn with_layer<LayerFn, DrawFn>(&mut self, layer_fn: LayerFn, draw_fn: DrawFn)
    where
        LayerFn: FnOnce(LayerHandle) -> LayerHandle,
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

    fn fill<T: Into<Self::ColorFormat>>(
        &mut self,
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let transform = transform.into().applying(&self.active_layer.transform);
        let bounding_box = shape.bounding_box().applying(&transform);
        if !bounding_box.intersects(&self.active_layer.clip_rect) {
            return;
        }

        // Convert the brush to the embedded_graphics color
        if let Some(color) = brush.as_solid().map(Into::into) {
            let style = PrimitiveStyleBuilder::new().fill_color(color).build();

            // Handle different shape types
            if let Some(line) = shape.as_line() {
                self.draw_line(&line, &transform, &style);
            } else if let Some(rect) = shape.as_rect() {
                self.draw_rectangle(&rect, &transform, &style);
            } else if let Some(circle) = shape.as_circle() {
                self.draw_circle(&circle, &transform, &style);
            } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                self.draw_rounded_rectangle(&rounded_rect, &transform, &style);
            } else {
                // For generic shapes, convert the path elements to lines
                self.draw_path_shape(shape, transform.offset, &style);
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
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = T>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        let transform = transform.into().applying(&self.active_layer.transform);
        let bounding_box = shape.bounding_box().applying(&transform);
        if !bounding_box.intersects(&self.active_layer.clip_rect) {
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
            self.draw_line(&line, &transform, &style);
        } else if let Some(rect) = shape.as_rect() {
            self.draw_rectangle(&rect, &transform, &style);
        } else if let Some(circle) = shape.as_circle() {
            self.draw_circle(&circle, &transform, &style);
        } else if let Some(rounded_rect) = shape.as_rounded_rect() {
            self.draw_rounded_rectangle(&rounded_rect, &transform, &style);
        } else {
            self.draw_path_shape(shape, transform.offset, &style);
        }
    }

    fn draw_glyphs<T: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &impl FontRender<Self::ColorFormat>,
    ) {
        let offset = offset.applying(&self.active_layer.transform);
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
    fn draw_line(&mut self, line: &Line, transform: &LinearTransform, style: &PrimitiveStyle<C>) {
        let line: EgLine = line.applying(transform).into();
        _ = line
            .into_styled(*style)
            .draw(&mut self.surface.draw_target());
    }

    fn draw_rectangle(
        &mut self,
        rect: &Rectangle,
        transform: &LinearTransform,
        style: &PrimitiveStyle<C>,
    ) {
        let eg_rect: EgRectangle = rect.applying(transform).into();
        let _ = eg_rect
            .into_styled(*style)
            .draw(&mut self.surface.draw_target());
    }

    fn draw_rounded_rectangle(
        &mut self,
        rect: &RoundedRectangle,
        transform: &LinearTransform,
        style: &PrimitiveStyle<C>,
    ) {
        let eg_rounded_rect: EgRoundedRectangle = rect.applying(transform).into();

        let _ = eg_rounded_rect
            .into_styled(*style)
            .draw(&mut self.surface.draw_target());
    }

    fn draw_circle(
        &mut self,
        circle: &Circle,
        transform: &LinearTransform,
        style: &PrimitiveStyle<C>,
    ) {
        let circle: EgCircle = circle.applying(transform).into();

        _ = circle
            .into_styled(*style)
            .draw(&mut self.surface.draw_target());
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
