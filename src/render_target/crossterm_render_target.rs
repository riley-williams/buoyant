use crossterm::{
    cursor, execute, style,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

#[cfg(feature = "std")]
use std::io::{stdout, Stdout, Write};

use crate::pixel::CrosstermColorSymbol;
use crate::primitives::{Frame, Point, Size};

use super::RenderTarget;

pub struct CrosstermRenderTarget {
    stdout: Stdout,
    window: Frame,
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
        Self {
            stdout: stdout(),
            window: Frame {
                origin: Point::default(),
                size: crossterm::terminal::size()
                    .map(|(w, h)| Size::new(w, h))
                    .unwrap_or_default(),
            },
        }
    }
}

impl Drop for CrosstermRenderTarget {
    fn drop(&mut self) {
        self.flush();
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }
}

impl RenderTarget<CrosstermColorSymbol> for CrosstermRenderTarget {
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

    fn draw(&mut self, point: crate::primitives::Point, item: CrosstermColorSymbol) {
        let draw_point = point + self.window.origin;
        self.stdout
            .queue(cursor::MoveTo(draw_point.x as u16, draw_point.y as u16))
            .unwrap()
            .queue(style::PrintStyledContent(item.into()))
            .unwrap();
    }

    fn set_window(&mut self, frame: Frame) {
        self.window = frame;
    }

    fn window(&self) -> Frame {
        self.window
    }
}
