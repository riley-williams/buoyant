use core::fmt::{Display, Formatter, Result};

use crate::{
    primitives::{Point, Size},
    render_target::RenderTarget,
};

/// A fixed size text buffer
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
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
        let y = point.y as usize;
        let x = point.x as usize;
        if y < H && x < W {
            self.text[y][x] = item;
        }
    }
}
