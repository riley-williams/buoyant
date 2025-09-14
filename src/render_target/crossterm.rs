use crossterm::{
    ExecutableCommand as _, QueueableCommand, cursor, execute,
    style::{self, Colors, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(feature = "std")]
use std::io::{Stdout, Write, stdout};

use crate::{
    primitives::{
        Pixel, Point, Size,
        geometry::Rectangle,
        transform::{CoordinateSpaceTransform, LinearTransform},
    },
    render_target::{LayerConfig, LayerHandle},
    surface::Surface,
};

use super::{Brush, Glyph, RenderTarget, Shape, Stroke};

/// A target that renders views to the terminal using the crossterm library.
///
/// The target will exit the alternate screen when dropped.
///
/// # Examples
///
/// ```
/// # use buoyant::render_target::CrosstermRenderTarget;
/// let mut target = CrosstermRenderTarget::default();
///
/// target.enter_fullscreen();
/// target.clear();
///
/// // Render view...
///
/// ```
#[derive(Debug)]
pub struct CrosstermRenderTarget {
    stdout: Stdout,
    active_layer: LayerConfig<Colors>,
}

impl CrosstermRenderTarget {
    /// Enters the alternate (fullscreen) mode.
    pub fn enter_fullscreen(&mut self) {
        execute!(self.stdout, EnterAlternateScreen).unwrap();
    }

    /// Exits the alternate (fullscreen) mode.
    pub fn exit_fullscreen(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }

    /// Flushes the output buffer.
    ///
    /// Ignores errors produced by executing the command.
    pub fn flush(&mut self) {
        _ = self.stdout.flush();
    }

    /// Returns the clear of this [`CrosstermRenderTarget`].
    ///
    /// Ignores errors produced by executing the command.
    pub fn clear(&mut self) {
        _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All));
    }

    #[must_use]
    pub fn size(&self) -> Size {
        crossterm::terminal::size()
            .map(|(w, h)| Size::new(w.into(), h.into()))
            .unwrap_or_default()
    }

    fn draw_color(&mut self, point: Point, color: Colors) {
        self.draw_character(point, ' ', color);
    }

    #[expect(unused, reason = "This is probably useful later")]
    fn draw_string(&mut self, point: Point, string: &str, color: Colors) {
        let mut styled_string = string.stylize();
        if let Some(foreground) = color.foreground {
            styled_string = styled_string.with(foreground);
        }
        if let Some(background) = color.background {
            styled_string = styled_string.on(background);
        }
        self.stdout
            .queue(cursor::MoveTo(
                point.x.try_into().unwrap_or_default(),
                point.y.try_into().unwrap_or_default(),
            ))
            .unwrap()
            .queue(style::PrintStyledContent(styled_string))
            .unwrap();
    }

    fn draw_character(&mut self, point: Point, character: char, color: Colors) {
        let mut styled_char = character.stylize();
        if let Some(foreground) = color.foreground {
            styled_char = styled_char.with(foreground);
        }
        if let Some(background) = color.background {
            styled_char = styled_char.on(background);
        }
        self.stdout
            .queue(cursor::MoveTo(
                point.x.try_into().unwrap_or_default(),
                point.y.try_into().unwrap_or_default(),
            ))
            .unwrap()
            .queue(style::PrintStyledContent(styled_char))
            .unwrap();
    }
}

impl Default for CrosstermRenderTarget {
    fn default() -> Self {
        let stdout = stdout();
        let size = crossterm::terminal::size()
            .map(|(w, h)| Size::new(w.into(), h.into()))
            .unwrap_or_default();
        let clip_rect = Rectangle::new(Point::zero(), size);
        Self {
            stdout,
            active_layer: LayerConfig::new_clip(clip_rect),
        }
    }
}

impl Drop for CrosstermRenderTarget {
    fn drop(&mut self) {
        self.flush();
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }
}

impl RenderTarget for CrosstermRenderTarget {
    type ColorFormat = Colors;

    fn size(&self) -> Size {
        self.size()
    }

    fn clear(&mut self, _color: Self::ColorFormat) {
        // FIXME: use the color provided
        self.clear();
    }

    fn clip_rect(&self) -> Rectangle {
        self.active_layer
            .clip_rect
            .applying_inverse(&self.active_layer.transform)
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

    fn alpha(&self) -> u8 {
        self.active_layer.alpha
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
            let origin = Point::new(
                rect.origin.x + transform.offset.x,
                rect.origin.y + transform.offset.y,
            );
            let rect = Rectangle::new(origin, rect.size);
            let Some(color) = brush.as_solid() else {
                return;
            };
            let color = color.into();
            let size = self.size();
            for y in 0..rect.size.height {
                for x in 0..rect.size.width {
                    let point = Point::new(rect.origin.x + x as i32, rect.origin.y + y as i32);
                    if point.x >= size.width as i32 || point.y >= size.height as i32 {
                        continue;
                    }
                    if self.active_layer.clip_rect.contains(&point) {
                        self.draw_color(point, color);
                    }
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
        // FIXME: This implementation is untested and only partially correct
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
                if y == 0 || y == rect.size.height as i32 - 1 {
                    for x in 0..rect.size.width as i32 {
                        let point = Point::new(rect.origin.x + x, rect.origin.y + y);
                        if self.active_layer.clip_rect.contains(&point) {
                            self.draw_color(point, color);
                        }
                    }
                } else {
                    let point = Point::new(rect.origin.x, rect.origin.y + y);
                    if self.active_layer.clip_rect.contains(&point) {
                        self.draw_color(point, color);
                    }
                    let point = Point::new(
                        rect.origin.x + rect.size.width as i32 - 1,
                        rect.origin.y + y,
                    );
                    if self.active_layer.clip_rect.contains(&point) {
                        self.draw_color(point, color);
                    }
                }
            }
        }
    }

    fn draw_glyphs<C: Into<Self::ColorFormat>>(
        &mut self,
        offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        _font: &impl crate::font::FontRender<Self::ColorFormat>,
    ) {
        let offset = offset.applying(&self.active_layer.transform);
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        for glyph in glyphs {
            let point = offset + glyph.offset;
            if self.active_layer.clip_rect.contains(&point) {
                self.draw_character(point, glyph.character, color);
            }
        }
    }

    fn raw_surface(&mut self) -> &mut impl Surface<Color = Self::ColorFormat> {
        self
    }
}

impl Surface for CrosstermRenderTarget {
    type Color = Colors;

    fn size(&self) -> Size {
        self.size()
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels
            .into_iter()
            .for_each(|p| self.draw_color(p.point, p.color));
    }
}
