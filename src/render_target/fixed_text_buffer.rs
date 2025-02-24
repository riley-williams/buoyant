use core::fmt::{Display, Formatter, Result};

use crate::{primitives::Size, render::CharacterRenderTarget};

/// A fixed size text buffer
#[derive(Debug, Clone, PartialEq)]
pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
}

impl<const W: usize, const H: usize> FixedTextBuffer<W, H> {
    pub const fn clear(&mut self) {
        self.text = [[' '; W]; H];
    }
}

impl<const W: usize, const H: usize> Display for FixedTextBuffer<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in &self.text {
            for c in line {
                write!(f, "{c}")?;
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

impl<const W: usize, const H: usize> CharacterRenderTarget for FixedTextBuffer<W, H> {
    type Color = char;

    fn draw_character(
        &mut self,
        point: crate::primitives::Point,
        character: char,
        _color: &Self::Color,
    ) {
        #[allow(clippy::cast_sign_loss)]
        if point.x < W as i16 && point.y < H as i16 && point.x >= 0 && point.y >= 0 {
            self.text[point.y as usize][point.x as usize] = character;
        }
    }

    fn draw_color(&mut self, point: crate::primitives::Point, color: &Self::Color) {
        #[allow(clippy::cast_sign_loss)]
        if point.x < W as i16 && point.y < H as i16 && point.x >= 0 && point.y >= 0 {
            self.text[point.y as usize][point.x as usize] = *color;
        }
    }

    fn size(&self) -> Size {
        Size::new(W as u16, H as u16)
    }
}
