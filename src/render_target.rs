#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics;
#[cfg(feature = "embedded-graphics")]
pub use embedded_graphics::EmbeddedGraphicsRenderTarget;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermRenderTarget;

mod fixed_text_buffer;
pub use fixed_text_buffer::FixedTextBuffer;

use crate::{
    font::{self, FontMetrics as _},
    image::EmptyImage,
    primitives::{
        Interpolate, Point, Size,
        geometry::{Rectangle, Shape},
        transform::{CoordinateSpaceTransform as _, LinearTransform, ScaleFactor},
    },
    surface::Surface,
};

pub trait RenderTarget {
    type ColorFormat;

    /// The drawable size of the target
    fn size(&self) -> Size;

    /// Clears the target using the provided color
    fn clear(&mut self, color: Self::ColorFormat);

    /// Returns the current clip area in the local (transformed) coordinate space.
    #[must_use]
    fn clip_rect(&self) -> Rectangle;

    /// Sets the clip area and transform within a scoped function
    ///
    /// The render target may choose to reduce the clip area to fit within its drawable size.
    fn with_layer<LayerFn, DrawFn>(&mut self, layer_fn: LayerFn, draw_fn: DrawFn)
    where
        LayerFn: FnOnce(LayerHandle<Self::ColorFormat>) -> LayerHandle<Self::ColorFormat>,
        DrawFn: FnOnce(&mut Self);

    /// The alpha value for the current layer.
    fn alpha(&self) -> u8;

    /// Reports that an animation is active.
    ///
    /// If a frame passes without this being called, the render target may
    /// choose to reduce its frame rate or enter a low-power state.
    fn report_active_animation(&mut self);

    /// Clears the active animation status, returning true if an animation was
    /// reported as active since the last time the status was cleared.
    fn clear_animation_status(&mut self) -> bool;

    /// Fills a shape using the specified style and brush.
    fn fill<C: Into<Self::ColorFormat>>(
        &mut self,
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = C>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    /// Strokes a shape using the specified style and brush.
    fn stroke<C: Into<Self::ColorFormat>>(
        &mut self,
        stroke: &Stroke,
        transform: impl Into<LinearTransform>,
        brush: &impl Brush<ColorFormat = C>,
        brush_offset: Option<Point>,
        shape: &impl Shape,
    );

    /// Draws a series of glyphs using the specified style and brush.
    fn draw_glyphs<C: Into<Self::ColorFormat>, F: font::FontRender<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        font: &F,
        font_attributes: &F::Attributes,
    );

    /// Draws a string using the specified style and brush.
    ///
    /// This performs the same operation as `draw_glyphs`, but also handles
    /// glyph indexing and positioning.
    fn draw_str<C: Into<Self::ColorFormat>, F: font::FontRender<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        text: &str,
        font: &F,
        font_attributes: &F::Attributes,
    ) {
        let metrics = font.metrics(font_attributes);
        let mut x = 0;
        self.draw_glyphs(
            offset,
            brush,
            text.chars().map(|c| {
                let glyph = Glyph {
                    character: c,
                    offset: Point::new(x, 0),
                };
                x += metrics.advance(glyph.character) as i32;
                glyph
            }),
            font,
            font_attributes,
        );
    }

    /// Obtain a raw surface to directly write pixels.
    ///
    /// This is most often useful for bridging `embedded_graphics` types
    /// that are designed to render to a `DrawTarget`.
    ///
    /// ```no_run
    /// # use buoyant::primitives::Size;
    /// # use buoyant::render_target::RenderTarget;
    /// # use buoyant::render_target::EmbeddedGraphicsRenderTarget;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb888;
    /// # use embedded_graphics::mock_display::MockDisplay;
    /// use tinytga::Tga;
    /// use crate::buoyant::surface::AsDrawTarget;
    ///
    /// let mut display = MockDisplay::<Rgb888>::new();
    /// let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
    ///
    /// let data= [0u8; 100]; // include_bytes!("path/to/image.tga");
    /// let img: Tga<Rgb888> = Tga::from_slice(&data[..]).unwrap();
    ///
    /// img.draw(&mut target.raw_surface().draw_target());
    /// ```
    fn raw_surface(&mut self) -> &mut impl Surface<Color = Self::ColorFormat>;
}

/// Positioned glyph.
#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    /// The character represented by the glyph.
    pub character: char,
    /// Offset in run, relative to transform.
    pub offset: Point,
}

/// Describes the color content of a filled or stroked shape.
pub trait Brush {
    type ColorFormat;

    /// Computes the color at a specific point
    fn color_at(&self, point: Point) -> Option<Self::ColorFormat>;

    /// Solid color brush.
    fn as_solid(&self) -> Option<Self::ColorFormat>;

    /// Image brush.
    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>>;
}

pub trait ImageBrush: Brush {
    /// Dimensions of the image.
    fn size(&self) -> Size;
    /// Iterator over the contiguous pixels of the image.
    fn color_iter(&self) -> impl Iterator<Item = Self::ColorFormat>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolidBrush<C> {
    color: C,
}

impl<C: Copy> SolidBrush<C> {
    #[must_use]
    pub const fn new(color: C) -> Self {
        Self { color }
    }
}

impl<C: Copy> Brush for SolidBrush<C> {
    type ColorFormat = C;

    fn color_at(&self, _point: Point) -> Option<Self::ColorFormat> {
        Some(self.color)
    }

    fn as_solid(&self) -> Option<Self::ColorFormat> {
        Some(self.color)
    }

    fn as_image(&self) -> Option<&impl ImageBrush<ColorFormat = Self::ColorFormat>> {
        Option::<&EmptyImage<Self::ColorFormat>>::None
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Stroke {
    /// Width of the stroke.
    pub width: u32,
}

impl Stroke {
    #[must_use]
    pub const fn new(width: u32) -> Self {
        Self { width }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerConfig<C> {
    /// The alpha value for this layer.
    pub alpha: u8,
    /// A background color hint to simulate alpha blending.
    pub background_hint: Option<C>,
    /// The transform from local to global coordinate space
    pub transform: LinearTransform,
    /// The clip rectangle for this layer, in the global coordinate space.
    pub clip_rect: Rectangle,
}

impl<C> LayerConfig<C> {
    #[must_use]
    pub const fn new(
        alpha: u8,
        background_hint: Option<C>,
        transform: LinearTransform,
        clip_rect: Rectangle,
    ) -> Self {
        Self {
            alpha,
            background_hint,
            transform,
            clip_rect,
        }
    }

    #[must_use]
    pub const fn new_sized(size: Size) -> Self {
        Self {
            alpha: 255,
            background_hint: None,
            transform: LinearTransform::identity(),
            clip_rect: Rectangle::new(Point::zero(), size),
        }
    }

    #[must_use]
    pub fn new_clip(clip_rect: impl Into<Rectangle>) -> Self {
        Self {
            alpha: 255,
            background_hint: None,
            transform: LinearTransform::default(),
            clip_rect: clip_rect.into(),
        }
    }

    #[must_use]
    pub fn with_background_hint(mut self, color: C) -> Self {
        self.background_hint = Some(color);
        self
    }
}

/// Provides a safe abstraction for modifying a layer configuration.
#[derive(Debug)]
pub struct LayerHandle<'a, C> {
    layer: &'a mut LayerConfig<C>,
}

#[allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]
impl<'a, C: Interpolate + Copy> LayerHandle<'a, C> {
    /// Creates a new layer handle for the given layer configuration.
    #[must_use]
    pub fn new(layer: &'a mut LayerConfig<C>) -> Self {
        Self { layer }
    }

    /// Sets the layer opacity.
    ///
    /// Depending on the render target, a background color hint may need to be set
    /// to see the effect of the opacity.
    ///
    /// See: [`hint_background`]
    pub fn opacity(self, opacity: u8) -> Self {
        self.layer.alpha = ((u16::from(self.layer.alpha) * u16::from(opacity)) / 255) as u8;
        self
    }

    /// Sets a background color hint for the layer.
    ///
    /// This is used to simulate alpha blending by interpolating the background color
    /// with the specified color based on the layer's alpha value.
    pub fn hint_background(self, color: C) -> Self {
        let adjusted_color = if self.layer.alpha == 255 {
            color
        } else if let Some(background_hint) = self.layer.background_hint {
            C::interpolate(background_hint, color, self.layer.alpha)
        } else {
            color
        };
        self.layer.background_hint = Some(adjusted_color);
        self
    }

    pub fn transform(self, transform: &LinearTransform) -> Self {
        self.layer.transform = self.layer.transform.applying(transform);
        self
    }

    pub fn offset(self, offset: Point) -> Self {
        self.layer.transform.offset.x +=
            (offset.x * self.layer.transform.scale.cast_signed()).to_num::<i32>();
        self.layer.transform.offset.y +=
            (offset.y * self.layer.transform.scale.cast_signed()).to_num::<i32>();
        self
    }

    pub fn scale(self, scale: ScaleFactor) -> Self {
        self.layer.transform.scale *= scale;
        self
    }

    pub fn clip(self, clip_rect: &Rectangle) -> Self {
        // maintain clip rect in global coordinate space
        // use zero-sized rectangle if the intersection is empty
        self.layer.clip_rect = clip_rect
            .applying_inverse(&self.layer.transform)
            .intersection(&self.layer.clip_rect)
            .unwrap_or_else(|| Rectangle::new(Point::zero(), Size::new(0, 0)));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Point;

    #[test]
    fn layer_config_new_sized() {
        let size = Size::new(640, 480);
        let layer = LayerConfig::<u8>::new_sized(size);

        assert_eq!(layer.alpha, 255);
        assert_eq!(layer.transform, LinearTransform::default());
        assert_eq!(layer.clip_rect, Rectangle::new(Point::zero(), size));
    }

    #[test]
    fn layer_config_new_clip() {
        let clip_rect = Rectangle::new(Point::new(10, 15), Size::new(50, 75));
        let layer = LayerConfig::<u8>::new_clip(clip_rect.clone());

        assert_eq!(layer.alpha, 255);
        assert_eq!(layer.transform, LinearTransform::default());
        assert_eq!(layer.clip_rect, clip_rect);
    }

    #[test]
    fn apply_layer_handle_alpha() {
        let mut layer = LayerConfig::<u8>::new_sized(Size::new(100, 100));
        assert_eq!(layer.alpha, 255);

        LayerHandle::new(&mut layer).opacity(255);
        assert_eq!(layer.alpha, 255);

        LayerHandle::new(&mut layer).opacity(128);
        assert_eq!(layer.alpha, 128);

        LayerHandle::new(&mut layer).opacity(128);
        assert_eq!(layer.alpha, 64);

        LayerHandle::new(&mut layer).opacity(255);
        assert_eq!(layer.alpha, 64);

        LayerHandle::new(&mut layer).opacity(0);
        assert_eq!(layer.alpha, 0);
    }

    #[test]
    fn apply_layer_handle_transform() {
        let mut layer = LayerConfig::<u8>::new_sized(Size::new(100, 100));
        let new_transform = LinearTransform::new(Point::new(10, 20), 2.0);
        let expected_transform = layer.transform.applying(&new_transform);

        LayerHandle::new(&mut layer).transform(&new_transform);

        assert_eq!(layer.transform, expected_transform);
    }

    #[test]
    fn apply_layer_handle_offset() {
        let mut layer = LayerConfig::<u8>::new(
            255,
            None,
            LinearTransform::new(Point::new(0, 0), 2.0),
            Rectangle::new(Point::zero(), Size::new(100, 100)),
        );
        let offset = Point::new(10, 20);

        LayerHandle::new(&mut layer).offset(offset);

        // Offset should be scaled by the layer's transform scale (2.0)
        // So offset (10, 20) * 2.0 = (20, 40)
        assert_eq!(layer.transform.offset, Point::new(20, 40));
    }

    #[test]
    fn apply_layer_handle_scale() {
        let mut layer = LayerConfig::<u8>::new(
            255,
            None,
            LinearTransform::new(Point::new(0, 0), 2.0),
            Rectangle::new(Point::zero(), Size::new(100, 100)),
        );
        let additional_scale = ScaleFactor::from_num(1.5);

        LayerHandle::new(&mut layer).scale(additional_scale);

        // Scale should be multiplied: 2.0 * 1.5 = 3.0
        assert_eq!(layer.transform.scale, ScaleFactor::from_num(3.0));
    }

    #[test]
    fn clip_rect_intersection() {
        let mut layer = LayerConfig::new(
            255,
            Option::<u8>::None,
            LinearTransform::identity(),
            Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
        );
        let new_clip = Rectangle::new(Point::new(25, 25), Size::new(50, 50));

        LayerHandle::new(&mut layer).clip(&new_clip);

        let expected_clip = Rectangle::new(Point::new(25, 25), Size::new(50, 50));
        assert_eq!(layer.clip_rect, expected_clip);
    }

    #[test]
    fn clip_rect_intersection_with_transform() {
        let mut layer = LayerConfig::new(
            255,
            Option::<u8>::None,
            LinearTransform::new(Point::new(4, 8), 2.0),
            Rectangle::new(Point::zero(), Size::new(100, 100)),
        );
        let new_clip = Rectangle::new(Point::new(10, 20), Size::new(50, 60));

        LayerHandle::new(&mut layer).clip(&new_clip);

        // New clip rect should be inverse transformed, then intersected with existing clip rect
        let transformed_clip =
            new_clip.applying_inverse(&LinearTransform::new(Point::new(4, 8), 2.0));
        let expected_clip = transformed_clip
            .intersection(&Rectangle::new(Point::zero(), Size::new(100, 100)))
            .unwrap();
        assert_eq!(layer.clip_rect, expected_clip);
    }

    #[test]
    fn clip_rect_no_intersection() {
        let mut layer = LayerConfig::new(
            255,
            Option::<u8>::None,
            LinearTransform::identity(),
            Rectangle::new(Point::new(0, 0), Size::new(50, 50)),
        );
        let new_clip = Rectangle::new(Point::new(100, 100), Size::new(50, 50));

        LayerHandle::new(&mut layer).clip(&new_clip);

        // Should result in zero-sized rectangle when there's no intersection
        let expected_clip = Rectangle::new(Point::zero(), Size::new(0, 0));
        assert_eq!(layer.clip_rect, expected_clip);
    }

    #[test]
    fn clip_rect_partial_intersection() {
        let mut layer = LayerConfig::new(
            255,
            Option::<u8>::None,
            LinearTransform::identity(),
            Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
        );
        let new_clip = Rectangle::new(Point::new(50, 50), Size::new(100, 100));

        LayerHandle::new(&mut layer).clip(&new_clip);

        let expected_clip = Rectangle::new(Point::new(50, 50), Size::new(50, 50));
        assert_eq!(layer.clip_rect, expected_clip);
    }

    #[test]
    fn chaining() {
        let mut layer = LayerConfig::<u8>::new_sized(Size::new(100, 100));
        let transform = LinearTransform::new(Point::new(5, 10), 1.5);
        let offset = Point::new(20, 30);
        let clip_rect = Rectangle::new(Point::new(10, 15), Size::new(80, 90));

        LayerHandle::new(&mut layer)
            .opacity(200)
            .transform(&transform)
            .offset(offset)
            .scale(ScaleFactor::from_num(2.0))
            .clip(&clip_rect);

        assert_eq!(layer.alpha, 200);
        assert_eq!(layer.transform.scale, ScaleFactor::from_num(3.0));
    }

    #[test]
    fn background_hint_full_opacity() {
        let mut layer = LayerConfig::<u8>::new_sized(Size::new(100, 100));
        assert_eq!(layer.background_hint, None);

        LayerHandle::new(&mut layer).hint_background(128);

        // With full opacity, the color should be used as-is
        assert_eq!(layer.background_hint, Some(128));
    }

    #[test]
    fn background_hint_partial_opacity_no_existing() {
        let mut layer = LayerConfig::new(
            128,
            Option::<u8>::None,
            LinearTransform::identity(),
            Rectangle::new(Point::zero(), Size::new(100, 100)),
        );

        LayerHandle::new(&mut layer).hint_background(200);

        // With partial opacity and no existing hint, should use the color directly
        assert_eq!(layer.background_hint, Some(200));
    }

    #[test]
    fn background_hint_partial_opacity_with_existing() {
        let mut layer = LayerConfig::new(
            12,
            Some(100u8),
            LinearTransform::identity(),
            Rectangle::new(Point::zero(), Size::new(100, 100)),
        );

        LayerHandle::new(&mut layer).hint_background(200);

        // With partial opacity and existing hint, should interpolate
        let expected = u8::interpolate(100, 200, 12);
        assert_eq!(layer.background_hint, Some(expected));
    }
}
