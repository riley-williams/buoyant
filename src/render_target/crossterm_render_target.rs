use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _, QueueableCommand,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write};

use crate::{primitives::Size, render::CharacterRenderTarget};

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

impl CharacterRenderTarget for CrosstermRenderTarget {
    type Color = crossterm::style::Colors;

    fn size(&self) -> Size {
        crossterm::terminal::size()
            .map(|(w, h)| Size::new(w, h))
            .unwrap_or_default()
    }

    fn draw_color(&mut self, point: crate::primitives::Point, color: &Self::Color) {
        self.draw_character(point, ' ', color);
    }

    fn draw_string(&mut self, point: crate::primitives::Point, string: &str, color: &Self::Color) {
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

    fn draw_character(
        &mut self,
        point: crate::primitives::Point,
        character: char,
        color: &Self::Color,
    ) {
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
