use crossterm::{
    cursor, execute,
    style::{self, Stylize as _},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _, QueueableCommand as _,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write as _};

use crate::{primitives::Size, render::CharacterRenderTarget};

#[derive(Debug)]
pub struct CrosstermRenderTarget {
    stdout: Stdout,
}

impl CrosstermRenderTarget {
    #[inline]
    pub fn enter_fullscreen(&mut self) {
        execute!(self.stdout, EnterAlternateScreen).unwrap();
    }

    #[inline]
    pub fn exit_fullscreen(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }

    #[inline]
    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    #[inline]
    pub fn clear(&mut self) {
        _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }
}

impl Default for CrosstermRenderTarget {
    #[inline]
    fn default() -> Self {
        Self { stdout: stdout() }
    }
}

impl Drop for CrosstermRenderTarget {
    #[inline]
    fn drop(&mut self) {
        self.flush();
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }
}

impl CharacterRenderTarget for CrosstermRenderTarget {
    type Color = crossterm::style::Colors;

    #[inline]
    fn size(&self) -> Size {
        crossterm::terminal::size()
            .map(|(w, h)| Size::new(w, h))
            .unwrap_or_default()
    }

    #[inline]
    fn draw_color(&mut self, point: crate::primitives::Point, color: &Self::Color) {
        self.draw_character(point, ' ', color);
    }

    #[inline]
    fn draw_string(&mut self, point: crate::primitives::Point, string: &str, color: &Self::Color) {
        let mut styled_string = string.stylize();
        if let Some(foreground) = color.foreground {
            styled_string = styled_string.with(foreground);
        }
        if let Some(background) = color.background {
            styled_string = styled_string.on(background);
        }
        self.stdout
            .queue(cursor::MoveTo(point.x as u16, point.y as u16))
            .unwrap()
            .queue(style::PrintStyledContent(styled_string))
            .unwrap();
    }

    #[inline]
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
            .queue(cursor::MoveTo(point.x as u16, point.y as u16))
            .unwrap()
            .queue(style::PrintStyledContent(styled_char))
            .unwrap();
    }
}
