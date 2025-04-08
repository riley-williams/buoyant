use crossterm::{
    cursor, execute,
    style::{self, Colors, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _, QueueableCommand,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write};

use crate::primitives::{geometry::Rectangle, Point, Size};

use super::{Brush, Glyph, RenderTarget, Shape, Stroke};

/// A target that renders views to the terminal using the crossterm library.
///
/// The target will exit the alternate screen when dropped.
///
/// Example:
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
        Self { stdout: stdout() }
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

    fn clear(&mut self, _color: Self::ColorFormat) {
        // FIXME: use the color provided
        self.clear();
    }

    fn fill<C: Into<Self::ColorFormat>>(
        &mut self,
        _transform_offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        _brush_offset: Option<Point>,
        shape: &impl Shape,
    ) {
        if let Some(rect) = shape.as_rect() {
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
                    self.draw_color(point, color);
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
        // FIXME: This implementation is untested and only partially correct
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
                        self.draw_color(point, color);
                    }
                } else {
                    let point = Point::new(rect.origin.x, rect.origin.y + y);
                    self.draw_color(point, color);
                    let point =
                        Point::new(rect.origin.x + rect.size.width as i32, rect.origin.y + y);
                    self.draw_color(point, color);
                }
            }
        }
    }

    fn draw_glyphs<C: Into<Self::ColorFormat>>(
        &mut self,
        mut offset: Point,
        brush: &impl Brush<ColorFormat = C>,
        glyphs: impl Iterator<Item = Glyph>,
        _font: &impl crate::font::FontRender,
    ) {
        let Some(color) = brush.as_solid().map(Into::into) else {
            return;
        };
        for c in glyphs.map(|g| g.character) {
            let point = Point::new(offset.x, offset.y);
            self.draw_character(point, c, color);
            offset.x += 1;
        }
    }
}
