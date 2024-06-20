use core::fmt::{Display, Formatter, Result};

use crate::{
    primitives::{Frame, Point, Size},
    render_target::RenderTarget,
};

/// A fixed size text buffer
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
    pub window: Frame,
}

impl<const W: usize, const H: usize> Display for FixedTextBuffer<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in self.text.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Default for FixedTextBuffer<W, H> {
    fn default() -> Self {
        Self {
            text: [[' '; W]; H],
            window: Frame {
                origin: Point::default(),
                size: Size::new(W as u16, H as u16),
            },
        }
    }
}

impl<const W: usize, const H: usize> RenderTarget<char> for FixedTextBuffer<W, H> {
    fn size(&self) -> Size {
        Size::new(W as u16, H as u16)
    }

    fn clear(&mut self) {
        for line in self.text.iter_mut() {
            for c in line.iter_mut() {
                *c = ' ';
            }
        }
    }

    fn draw(&mut self, point: Point, item: char) {
        let absolute_point = point + self.window.origin;
        let x = absolute_point.x as usize;
        let y = absolute_point.y as usize;
        if y < H && x < W {
            self.text[y][x] = item;
        }
    }

    fn set_window(&mut self, frame: Frame) {
        self.window = frame;
    }

    fn window(&self) -> Frame {
        self.window
    }
}
