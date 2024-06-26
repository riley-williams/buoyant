#[cfg(feature = "crossterm")]
use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write};

use crate::primitives::Size;

use super::RenderTarget;

pub struct CrosstermRenderTarget {
    stdout: Stdout,
}

impl CrosstermRenderTarget {
    pub fn enter_fullscreen(&mut self) {
        execute!(self.stdout, EnterAlternateScreen).unwrap();
    }

    pub fn exit_fullscreen(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
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

impl RenderTarget<char> for CrosstermRenderTarget {
    fn size(&self) -> Size {
        crossterm::terminal::size()
            .map(|(w, h)| Size::new(w, h))
            .unwrap_or_default()
    }

    fn clear(&mut self) {
        _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    fn draw(&mut self, point: crate::primitives::Point, item: char) {
        self.stdout
            .queue(cursor::MoveTo(point.x as u16, point.y as u16))
            .unwrap()
            .queue(style::PrintStyledContent(item.to_string().green()))
            .unwrap();
    }
}
