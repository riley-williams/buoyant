use crate::{primitives::Size, surface::Surface};

mod character_buffer_font;
pub use character_buffer_font::CharacterBufferFont;

#[cfg(feature = "embedded-graphics")]
mod embedded_mono_font;
#[cfg(feature = "embedded-graphics")]
mod u8g2;

/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait Font {
    fn metrics(&self) -> impl FontMetrics;
}

// TODO: This could probably accept a draw target instead of a surface?
// As-is, it limits to basically just embedded-graphics capable fonts.
// For now, I don't think it's worth allowing outside implementations
// until a better solution is determined
pub(crate) trait Sealed {}

#[expect(private_bounds)]
pub trait FontRender<Color>: Font + Sealed {
    /// Render the character by drawing to a surface.
    fn draw(&self, character: char, color: Color, surface: &mut impl Surface<Color = Color>);
}

pub trait FontMetrics {
    /// The rendered size of a glyph
    fn rendered_size(&self, character: char) -> Size;

    /// The default spacing between baselines
    fn default_line_height(&self) -> u32;

    /// The horizontal advance produced by a character
    fn advance(&self, character: char) -> u32;

    /// The distance from the top of the character to the baseline
    fn baseline(&self) -> u32;

    /// The width of a string
    fn str_width(&self, text: &str) -> u32 {
        text.chars().map(|c| self.advance(c)).sum()
    }
}

impl<T: FontMetrics> FontMetrics for &T {
    fn rendered_size(&self, character: char) -> Size {
        (*self).rendered_size(character)
    }

    fn default_line_height(&self) -> u32 {
        (*self).default_line_height()
    }

    fn advance(&self, character: char) -> u32 {
        (*self).advance(character)
    }

    fn baseline(&self) -> u32 {
        (*self).baseline()
    }

    fn str_width(&self, text: &str) -> u32 {
        (*self).str_width(text)
    }
}
