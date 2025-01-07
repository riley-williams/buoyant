use core::fmt::{Display, Formatter, Result};

use embedded_graphics::pixelcolor::raw::RawU32;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions};

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

impl<const W: usize, const H: usize> DrawTarget for FixedTextBuffer<W, H> {
    type Color = CharColor;

    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> core::result::Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        todo!()
    }
}

impl<const W: usize, const H: usize> OriginDimensions for FixedTextBuffer<W, H> {
    fn size(&self) -> embedded_graphics::geometry::Size {
        embedded_graphics::geometry::Size::new(W as u32, H as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CharColor(Option<char>);

impl CharColor {
    pub fn new(color: char) -> Self {
        Self(Some(color))
    }

    pub fn clear() -> Self {
        Self(None)
    }
}

impl From<char> for CharColor {
    fn from(color: char) -> Self {
        Self::new(color)
    }
}

impl Default for CharColor {
    fn default() -> Self {
        Self::clear()
    }
}

impl embedded_graphics::pixelcolor::PixelColor for CharColor {
    type Raw = RawU32;
}
