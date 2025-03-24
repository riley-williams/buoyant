use crossterm::{
    cursor, execute,
    style::{self, Colors, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _, QueueableCommand,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write};

use crate::{primitives::Size, render_target::geometry::Point};

use super::{Brush, RenderTarget};

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
            .map(|(w, h)| Size::new(w, h))
            .unwrap_or_default()
    }

    fn draw_color(&mut self, point: Point, color: Colors) {
        self.draw_character(point, ' ', color);
    }

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
    type Layer = ();

    type ColorFormat = Colors;

    fn reset(&mut self) {
        self.clear();
    }

    fn push_layer(&mut self) -> Self::Layer {
        unimplemented!()
    }

    fn pop_layer(&mut self, _layer: Self::Layer) {
        unimplemented!()
    }

    fn fill(
        &mut self,
        _transform_offset: super::geometry::Point,
        brush: super::Brush<'_, impl Into<Self::ColorFormat>>,
        _brush_offset: Option<super::geometry::Point>,
        shape: &impl super::Shape,
    ) {
        if let Some(rect) = shape.as_rect() {
            let Brush::Solid(color) = brush else { return };
            let color = color.into();
            let size = self.size();
            for y in 0..rect.size.1 {
                for x in 0..rect.size.0 {
                    let point = Point::new(rect.origin.x + x as i32, rect.origin.y + y as i32);
                    if point.x >= size.width.into() || point.y >= size.height.into() {
                        continue;
                    }
                    self.draw_color(point, color);
                }
            }
        }
    }

    fn stroke(
        &mut self,
        _stroke: &super::Stroke,
        _transform_offset: super::geometry::Point,
        _brush: super::Brush<'_, impl Into<Self::ColorFormat>>,
        _brush_offset: Option<super::geometry::Point>,
        _shape: &impl super::Shape,
    ) {
        unimplemented!();
    }

    fn draw_glyphs(
        &mut self,
        offset: super::geometry::Point,
        brush: super::Brush<'_, impl Into<Self::ColorFormat>>,
        text: &str,
    ) {
        let Brush::Solid(color) = brush else { return };
        self.draw_string(offset, text, color.into());
    }
}
