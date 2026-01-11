use crate::{
    primitives::{Interpolate, Point, geometry::Rectangle},
    surface::Surface,
};

mod character_buffer_font;
pub use character_buffer_font::CharacterBufferFont;

#[cfg(feature = "embedded-graphics")]
mod embedded_mono_font;
#[cfg(feature = "rusttype-fonts")]
mod rusttype;
#[cfg(feature = "embedded-graphics")]
mod u8g2;

/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait Font {
    type Attributes: Default + Interpolate + Clone;
    fn metrics(&self, attributes: &Self::Attributes) -> impl FontMetrics;
}

// TODO: This could probably accept a draw target instead of a surface?
// As-is, it limits to basically just embedded-graphics capable fonts.
// For now, I don't think it's worth allowing outside implementations
// until a better solution is determined
pub(crate) trait Sealed {}

#[expect(private_bounds)]
pub trait FontRender<Color>: Font + Sealed {
    /// Render the character by drawing to a surface.
    fn draw(
        &self,
        character: char,
        offset: Point,
        color: Color,
        background_color: Option<Color>,
        attributes: &Self::Attributes,
        surface: &mut impl Surface<Color = Color>,
    );
}

pub trait FontMetrics {
    /// The rendered size and offset of a glyph, relative to the top left corner
    #[must_use]
    fn rendered_size(&self, character: char) -> Option<Rectangle>;

    /// The default spacing between baselines
    #[must_use]
    fn vertical_metrics(&self) -> VMetrics;

    /// The horizontal advance produced by a character
    #[must_use]
    fn advance(&self, character: char) -> u32;
}

impl<T: FontMetrics> FontMetrics for &T {
    fn rendered_size(&self, character: char) -> Option<Rectangle> {
        (*self).rendered_size(character)
    }

    fn vertical_metrics(&self) -> VMetrics {
        (*self).vertical_metrics()
    }

    fn advance(&self, character: char) -> u32 {
        (*self).advance(character)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VMetrics {
    /// The distance from the baseline to the top of the tallest character
    pub ascent: i32,
    /// The distance from the baseline to the bottom of the lowest character, typically negative
    pub descent: i32,
    /// The spacing between lines, i.e. the space between the bottom of one line's descent to the
    /// top of the next line's ascent
    pub line_spacing: i32,
}

impl VMetrics {
    /// The distance between baselines
    #[must_use]
    pub fn line_height(&self) -> u32 {
        (self.ascent - self.descent + self.line_spacing).max(0) as u32
    }
}

/// A Font attribute allowing customization of size.
pub trait CustomSize {
    #[must_use]
    fn with_size(self, size: u32) -> Self;
}
