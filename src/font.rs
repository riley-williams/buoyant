use crate::{
    primitives::{Size, geometry::Rectangle},
    surface::Surface,
};

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
    /// The rendered size and offset of a glyph, relative to the top left corner
    fn rendered_size(&self, character: char) -> Option<Rectangle>;

    /// The default spacing between baselines
    fn default_line_height(&self) -> u32;

    /// The horizontal advance produced by a character
    fn advance(&self, character: char) -> u32;

    /// The maximum size of a character in this font
    fn maximum_character_size(&self) -> Size;

    /// The width of a string
    fn str_width(&self, text: &str) -> u32;
}

impl<T: FontMetrics> FontMetrics for &T {
    fn rendered_size(&self, character: char) -> Option<Rectangle> {
        (*self).rendered_size(character)
    }

    fn default_line_height(&self) -> u32 {
        (*self).default_line_height()
    }

    fn advance(&self, character: char) -> u32 {
        (*self).advance(character)
    }

    fn maximum_character_size(&self) -> Size {
        (*self).maximum_character_size()
    }

    fn str_width(&self, text: &str) -> u32 {
        (*self).str_width(text)
    }
}
