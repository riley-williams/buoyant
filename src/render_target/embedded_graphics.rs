use crate::color::AlphaColor;
use crate::font::FontRender;
use crate::render_target::surface::OffsetSurface;
use crate::{
    primitives::{
        Interpolate, Point,
        geometry::{Circle, Intersection, Line, PathEl, Rectangle, RoundedRectangle},
        transform::{CoordinateSpaceTransform, LinearTransform},
    },
    render_target::{
        Brush, LayerConfig, LayerHandle, RenderTarget, Shape,
        surface::{AsDrawTarget, ClippedSurface, DrawTargetSurface},
    },
};

use embedded_graphics::prelude::DrawTargetExt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    geometry::Point as EgPoint,
    pixelcolor::PixelColor,
    prelude::Primitive as _,
    primitives::{
        Circle as EgCircle, Line as EgLine, PrimitiveStyle, PrimitiveStyleBuilder,
        Rectangle as EgRectangle, RoundedRectangle as EgRoundedRectangle,
    },
};

use super::{Glyph, ImageBrush, Stroke, Surface};

#[derive(Debug)]
pub struct EmbeddedGraphicsRenderTarget<D: Surface> {
    surface: D,
    active_layer: LayerConfig<D::Color>,
    /// A flag tracking active animation. This is initialized to true to prevent issues displaying
    /// the first frame.
    active_animation: bool,
}

impl<'a, D> EmbeddedGraphicsRenderTarget<DrawTargetSurface<'a, D>>
where
    D: DrawTarget,
    D::Color: PixelColor + Interpolate + AlphaColor,
{
    /// Initialize an `EmbeddedGraphicsRenderTarget` from a `DrawTarget`
    #[must_use]
    pub fn new(display: &'a mut D) -> Self {
        let clip_rect = display.bounding_box();
        Self {
            surface: DrawTargetSurface(display),
            active_layer: LayerConfig::new_clip(clip_rect),
            active_animation: true,
        }
    }

    /// Initialize an `EmbeddedGraphicsRenderTarget` from a `DrawTarget`, using the provided hint for the background color.
    #[must_use]
    pub fn new_hinted(display: &'a mut D, background_hint: D::Color) -> Self {
        let clip_rect = display.bounding_box();
        Self {
            surface: DrawTargetSurface(display),
            active_layer: LayerConfig::new_clip(clip_rect).with_background_hint(background_hint),
            active_animation: true,
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
    C: PixelColor + Interpolate + AlphaColor,
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
        self.active_layer.alpha
    }

    fn report_active_animation(&mut self) {
        self.active_animation = true;
    }

    fn clear_animation_status(&mut self) -> bool {
        let was_active = self.active_animation;
        self.active_animation = false;
        was_active
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

        // Convert the brush to the embedded_graphics color
        if let Some(color) = brush.as_solid().map(Into::into) {
            let color = self
                .active_layer
                .background_hint
                .map_or(color, |background| {
                    if self.active_layer.alpha < 255 {
                        Interpolate::interpolate(background, color, self.active_layer.alpha)
                    } else {
                        color
                    }
                });
            let style = PrimitiveStyleBuilder::new().fill_color(color).build();

            match self.active_layer.clip_rect.intersection_with(&bounding_box) {
                Intersection::Contains => {
                    let mut target = self.surface.draw_target();
                    if let Some(line) = shape.as_line() {
                        draw_line(&mut target, &line, &transform, &style);
                    } else if let Some(rect) = shape.as_rect() {
                        draw_rectangle(&mut target, &rect, &transform, &style);
                    } else if let Some(circle) = shape.as_circle() {
                        draw_circle(&mut target, &circle, &transform, &style);
                    } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                        draw_rounded_rectangle(&mut target, &rounded_rect, &transform, &style);
                    } else {
                        draw_path_shape(&mut target, shape, transform.offset, &style);
                    }
                }
                Intersection::Overlaps => {
                    // For now, we just draw on a clipped surface
                    let mut target = self.surface.draw_target();
                    let mut target = target.clipped(&self.active_layer.clip_rect.clone().into());
                    if let Some(line) = shape.as_line() {
                        draw_line(&mut target, &line, &transform, &style);
                    } else if let Some(rect) = shape.as_rect() {
                        draw_rectangle(&mut target, &rect, &transform, &style);
                    } else if let Some(circle) = shape.as_circle() {
                        draw_circle(&mut target, &circle, &transform, &style);
                    } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                        draw_rounded_rectangle(&mut target, &rounded_rect, &transform, &style);
                    } else {
                        draw_path_shape(&mut target, shape, transform.offset, &style);
                    }
                }
                Intersection::NonIntersecting => (),
            }
        } else if let Some(image) = brush.as_image() {
            // only support rectangles for now
            let Some(rect) = shape.as_rect() else { return };

            match self.active_layer.clip_rect.intersection_with(&bounding_box) {
                Intersection::Contains => {
                    // FIXME: Apply brush transform and clip to shape bounds
                    self.surface
                        .fill_contiguous(&rect, image.color_iter().map(Into::into));
                }
                Intersection::Overlaps => {
                    // There might be a more efficient way to do this
                    let mut surface =
                        ClippedSurface::new(&mut self.surface, self.active_layer.clip_rect.clone());
                    // FIXME: Apply brush transform and clip to shape bounds
                    surface.fill_contiguous(&rect, image.color_iter().map(Into::into));
                }
                Intersection::NonIntersecting => (),
            }
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

        // Convert the brush to the embedded_graphics color.
        // Only solid strokes are implemented
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        let color = self
            .active_layer
            .background_hint
            .map_or(color, |background| {
                if self.active_layer.alpha < 255 {
                    Interpolate::interpolate(background, color, self.active_layer.alpha)
                } else {
                    color
                }
            });

        let scaled_stroke_width = (stroke.width * transform.scale).to_num();
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(scaled_stroke_width)
            .stroke_color(color)
            .build();

        match self.active_layer.clip_rect.intersection_with(&bounding_box) {
            Intersection::Contains => {
                let mut target = self.surface.draw_target();
                if let Some(line) = shape.as_line() {
                    draw_line(&mut target, &line, &transform, &style);
                } else if let Some(rect) = shape.as_rect() {
                    draw_rectangle(&mut target, &rect, &transform, &style);
                } else if let Some(circle) = shape.as_circle() {
                    draw_circle(&mut target, &circle, &transform, &style);
                } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                    draw_rounded_rectangle(&mut target, &rounded_rect, &transform, &style);
                } else {
                    draw_path_shape(&mut target, shape, transform.offset, &style);
                }
            }
            Intersection::Overlaps => {
                // For now, we just draw on a clipped surface
                let mut target = self.surface.draw_target();
                let mut target = target.clipped(&self.active_layer.clip_rect.clone().into());
                if let Some(line) = shape.as_line() {
                    draw_line(&mut target, &line, &transform, &style);
                } else if let Some(rect) = shape.as_rect() {
                    draw_rectangle(&mut target, &rect, &transform, &style);
                } else if let Some(circle) = shape.as_circle() {
                    draw_circle(&mut target, &circle, &transform, &style);
                } else if let Some(rounded_rect) = shape.as_rounded_rect() {
                    draw_rounded_rectangle(&mut target, &rounded_rect, &transform, &style);
                } else {
                    draw_path_shape(&mut target, shape, transform.offset, &style);
                }
            }
            Intersection::NonIntersecting => (),
        }
    }

    fn draw_glyphs<T: Into<Self::ColorFormat>, F: FontRender<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = T>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &F,
        font_attributes: &F::Attributes,
        conservative_bounds: &Rectangle,
    ) {
        let offset = offset.applying(&self.active_layer.transform);
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        let color = self
            .active_layer
            .background_hint
            .map_or(color, |background| {
                if self.active_layer.alpha < 255 {
                    Interpolate::interpolate(background, color, self.active_layer.alpha)
                } else {
                    color
                }
            });

        let text_bounds = conservative_bounds.applying(&self.active_layer.transform);
        let clip_rect = self.active_layer.clip_rect.clone();
        match clip_rect.intersection_with(&text_bounds) {
            Intersection::Contains => {
                glyphs.for_each(|glyph| {
                    font.draw(
                        glyph.character,
                        offset + glyph.offset,
                        color,
                        self.active_layer.background_hint,
                        font_attributes,
                        &mut self.surface,
                    );
                });
            }
            Intersection::Overlaps => {
                let mut surface = ClippedSurface::new(&mut self.surface, clip_rect);
                glyphs.for_each(|glyph| {
                    font.draw(
                        glyph.character,
                        offset + glyph.offset,
                        color,
                        self.active_layer.background_hint,
                        font_attributes,
                        &mut surface,
                    );
                });
            }
            Intersection::NonIntersecting => (),
        }
    }

    fn raw_surface(&mut self) -> impl Surface<Color = Self::ColorFormat> + '_ {
        let offset_surface =
            OffsetSurface::new(&mut self.surface, self.active_layer.transform.offset);
        ClippedSurface::new(offset_surface, self.active_layer.clip_rect.clone())
    }
}

fn draw_line<C: PixelColor>(
    target: &mut impl DrawTarget<Color = C>,
    line: &Line,
    transform: &LinearTransform,
    style: &PrimitiveStyle<C>,
) {
    let line: EgLine = line.applying(transform).into();
    _ = line.into_styled(*style).draw(target);
}

fn draw_rectangle<C: PixelColor>(
    target: &mut impl DrawTarget<Color = C>,
    rect: &Rectangle,
    transform: &LinearTransform,
    style: &PrimitiveStyle<C>,
) {
    let eg_rect: EgRectangle = rect.applying(transform).into();
    let _ = eg_rect.into_styled(*style).draw(target);
}

fn draw_rounded_rectangle<C: PixelColor>(
    target: &mut impl DrawTarget<Color = C>,
    rect: &RoundedRectangle,
    transform: &LinearTransform,
    style: &PrimitiveStyle<C>,
) {
    let eg_rounded_rect: EgRoundedRectangle = rect.applying(transform).into();

    let _ = eg_rounded_rect.into_styled(*style).draw(target);
}

fn draw_circle<C: PixelColor>(
    target: &mut impl DrawTarget<Color = C>,
    circle: &Circle,
    transform: &LinearTransform,
    style: &PrimitiveStyle<C>,
) {
    let circle: EgCircle = circle.applying(transform).into();

    _ = circle.into_styled(*style).draw(target);
}

fn draw_path_shape<C: PixelColor>(
    target: &mut impl DrawTarget<Color = C>,
    shape: &impl Shape,
    offset: Point,
    style: &PrimitiveStyle<C>,
) {
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
                    let _ = eg_line.draw(target);

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
                    let _ = eg_line.draw(target);

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
                    let _ = eg_line.draw(target);

                    last_point = Some(end);
                }
            }
            PathEl::ClosePath => {
                // Close the path by drawing a line back to the starting point
                if let (Some(start), Some(first)) = (last_point, last_point) {
                    let start_eg = EgPoint::new(start.x, start.y);
                    let end_eg = EgPoint::new(first.x, first.y);

                    let eg_line = EgLine::new(start_eg, end_eg).into_styled(*style);
                    let _ = eg_line.draw(target);
                }
            }
        }
    }
}
