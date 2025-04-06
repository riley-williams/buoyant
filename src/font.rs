use core::iter::Empty;

use crate::primitives::geometry::PathEl;

/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait FontLayout {
    /// The height of a character in points
    fn line_height(&self) -> u16;

    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;

    /// The distance from the top of the character to the baseline
    fn baseline(&self) -> u16 {
        self.line_height()
    }

    fn str_width(&self, text: &str) -> u16 {
        text.chars().map(|c| self.character_width(c)).sum()
    }
}

pub type GlyphIndex = usize;

pub trait FontRender: FontLayout {
    fn glyph_index(&self, character: char) -> GlyphIndex;
    fn as_path(&self, index: GlyphIndex) -> Option<impl Iterator<Item = PathEl>>;
    fn as_mask(&self, index: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = bool>>>;
    fn as_alpha(&self, index: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = u8>>>;
    fn metrics(&self) -> impl FontMetrics;
}

pub trait FontMetrics {
    fn size(&self, index: GlyphIndex) -> (u16, u16);
    fn advance(&self, index: GlyphIndex) -> u16;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RasterGlyph<I> {
    pub width: u16,
    pub height: u16,
    pub iter: I,
}

/// A simple font for rendering non-Unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterBufferFont;

impl FontLayout for CharacterBufferFont {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }

    #[inline]
    fn character_width(&self, _: char) -> u16 {
        1
    }
}

impl FontRender for CharacterBufferFont {
    #[inline]
    fn glyph_index(&self, character: char) -> GlyphIndex {
        character as usize
    }

    #[inline]
    fn as_path(&self, _: GlyphIndex) -> Option<impl Iterator<Item = PathEl>> {
        Option::<Empty<PathEl>>::None
    }

    #[inline]
    fn as_mask(&self, _: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = bool>>> {
        Option::<RasterGlyph<Empty<bool>>>::None
    }

    #[inline]
    fn as_alpha(&self, _: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = u8>>> {
        Option::<RasterGlyph<Empty<u8>>>::None
    }

    #[inline]
    fn metrics(&self) -> impl FontMetrics {
        CharacterBufferFontMetrics
    }
}

struct CharacterBufferFontMetrics;
impl FontMetrics for CharacterBufferFontMetrics {
    #[inline]
    fn size(&self, _: GlyphIndex) -> (u16, u16) {
        (1, 1)
    }

    #[inline]
    fn advance(&self, _: GlyphIndex) -> u16 {
        1
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_fonts {
    use core::iter::Empty;
    use embedded_graphics::geometry::OriginDimensions;
    use embedded_graphics::image::GetPixel;
    use embedded_graphics::prelude::Point;

    use crate::primitives::geometry::PathEl;

    use super::{FontLayout, FontMetrics, FontRender, GlyphIndex, RasterGlyph};

    impl FontLayout for embedded_graphics::mono_font::MonoFont<'_> {
        #[inline]
        fn line_height(&self) -> u16 {
            self.character_size.height as u16
        }

        #[inline]
        fn character_width(&self, _: char) -> u16 {
            self.character_size.width as u16 + self.character_spacing as u16
        }

        #[inline]
        fn baseline(&self) -> u16 {
            self.baseline as u16
        }
    }

    impl FontRender for embedded_graphics::mono_font::MonoFont<'_> {
        fn glyph_index(&self, character: char) -> GlyphIndex {
            self.glyph_mapping.index(character)
        }

        fn as_path(&self, _: GlyphIndex) -> Option<impl Iterator<Item = PathEl>> {
            Option::<Empty<PathEl>>::None
        }

        fn as_mask(&self, index: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = bool>>> {
            // FIXME: This implementation may rely on private embedded_graphics api, maybe worth
            // opening a PR for a glyph SubImage and SubImage iter?
            if self.character_size.width == 0 || self.image.size().width < self.character_size.width
            {
                return None;
            }

            let glyphs_per_row = self.image.size().width / self.character_size.width;

            let glyph_index = index as u32;
            let row = glyph_index / glyphs_per_row;

            // Top left corner of character, in pixels
            let char_x = (glyph_index - (row * glyphs_per_row)) * self.character_size.width;
            let char_y = row * self.character_size.height;
            let iter = (char_y..char_y + self.character_size.height).flat_map(move |y| {
                (char_x..char_x + self.character_size.width).map(move |x| {
                    self.image
                        .pixel(Point {
                            x: x as i32,
                            y: y as i32,
                        })
                        .is_some_and(embedded_graphics::pixelcolor::BinaryColor::is_on)
                })
            });
            Some(RasterGlyph {
                width: self.character_size.width as u16,
                height: self.character_size.height as u16,
                iter,
            })
        }

        fn as_alpha(&self, _: GlyphIndex) -> Option<RasterGlyph<impl Iterator<Item = u8>>> {
            Option::<RasterGlyph<Empty<u8>>>::None
        }

        fn metrics(&self) -> impl FontMetrics {
            MonoFontMetrics {
                size: (
                    self.character_size.width as u16,
                    self.character_size.height as u16,
                ),
                baseline: self.baseline as u16,
                advance: self.character_spacing as u16 + self.character_size.width as u16,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct MonoFontMetrics {
        size: (u16, u16),
        baseline: u16,
        advance: u16,
    }

    impl FontMetrics for MonoFontMetrics {
        fn size(&self, _: GlyphIndex) -> (u16, u16) {
            self.size
        }

        fn advance(&self, _: GlyphIndex) -> u16 {
            self.advance
        }
    }
}
