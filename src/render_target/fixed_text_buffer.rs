use core::fmt::{Display, Formatter, Result};

use crate::primitives::{Point, Size};
use crate::render::shade::ShadeSolid;

use super::RenderTarget;

/// A fixed size text buffer
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
}

impl<const W: usize, const H: usize> FixedTextBuffer<W, H> {
    pub fn clear(&mut self) {
        self.text = [[' '; W]; H];
    }
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

impl<const W: usize, const H: usize> RenderTarget for FixedTextBuffer<W, H> {
    type Color = TxtColor;

    fn size(&self) -> Size {
        Size::new(W as u16, H as u16)
    }

    fn draw(&mut self, point: Point, color: Self::Color) {
        let x = point.x as usize;
        let y = point.y as usize;
        if y < H && x < W {
            if let Some(color) = color.0 {
                self.text[y][x] = color;
            }
        }
    }

    fn draw_text(
        &mut self,
        text: &str,
        position: Point,
        _shader: &impl crate::render::shade::Shader<Color = Self::Color>,
    ) {
        for (i, c) in text.chars().enumerate() {
            self.draw(position + Point::new(i as i16, 0), TxtColor::new(c));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TxtColor(Option<char>);

impl TxtColor {
    pub fn new(color: char) -> Self {
        Self(Some(color))
    }

    pub fn clear() -> Self {
        Self(None)
    }
}

impl From<char> for TxtColor {
    fn from(color: char) -> Self {
        Self::new(color)
    }
}

impl Default for TxtColor {
    fn default() -> Self {
        Self::clear()
    }
}

impl ShadeSolid for TxtColor {
    fn color(&self) -> TxtColor {
        *self
    }
}
