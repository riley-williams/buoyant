use crate::{primitives::Point, render_target::CharacterRenderTarget};

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

/// A font that renders individual characters at a time to a character render target
/// Multi-character graphemes are not supported
pub trait CharacterFont<C: Copy>: FontLayout {
    /// Render a sequence of characters with a solid color
    fn render_iter_solid<T, I>(&self, target: &mut T, origin: Point, color: C, characters: I)
    where
        T: CharacterRenderTarget<Color = C>,
        I: IntoIterator<Item = char>,
    {
        self.render_iter(target, origin, characters.into_iter().map(|c| (c, color)));
    }

    /// Render a sequence of characters with individually defined colors
    fn render_iter<T, I>(&self, target: &mut T, origin: Point, characters: I)
    where
        T: CharacterRenderTarget<Color = C>,
        I: IntoIterator<Item = (char, C)>;
}

/// A simple font for rendering non-unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferCharacterFont;

impl FontLayout for BufferCharacterFont {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }

    #[inline]
    fn character_width(&self, _: char) -> u16 {
        1
    }
}

impl<C: Copy> CharacterFont<C> for BufferCharacterFont {
    fn render_iter<T, I>(&self, target: &mut T, origin: Point, characters: I)
    where
        T: CharacterRenderTarget<Color = C>,
        I: IntoIterator<Item = (char, C)>,
    {
        for (i, (character, color)) in characters.into_iter().enumerate() {
            target.draw(origin + Point::new(i as i16, 0), character, color);
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
#[cfg(feature = "embedded-graphics")]
pub trait PixelFont<C>: FontLayout {
    fn render_iter<T, I>(&self, target: &mut T, origin: Point, color: C, characters: I)
    where
        T: DrawTarget<Color = C>,
        I: IntoIterator<Item = char>;
}

#[cfg(feature = "crossterm")]
pub use crossterm_font::TerminalCharFont;

#[cfg(feature = "crossterm")]
mod crossterm_font {
    use crate::{primitives::Point, render_target::CharacterRenderTarget};

    use super::{CharacterFont, FontLayout};

    /// A simple font for rendering non-unicode characters in a text buffer
    /// The width and height of all characters is 1.
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TerminalCharFont;

    impl FontLayout for TerminalCharFont {
        #[inline]
        fn line_height(&self) -> u16 {
            1
        }

        #[inline]
        fn character_width(&self, _: char) -> u16 {
            1
        }
    }

    impl CharacterFont<crossterm::style::Colors> for TerminalCharFont {
        // TODO: Add render_iter_solid impl once character render target draws strings

        fn render_iter<T, I>(&self, target: &mut T, origin: Point, characters: I)
        where
            T: CharacterRenderTarget<Color = crossterm::style::Colors>,
            I: IntoIterator<Item = (char, crossterm::style::Colors)>,
        {
            for (i, (character, color)) in characters.into_iter().enumerate() {
                target.draw(origin + Point::new(i as i16, 0), character, color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_fonts {
    use embedded_graphics::{draw_target::DrawTarget, mono_font::MonoTextStyle, text::Text};
    use embedded_graphics_core::pixelcolor::PixelColor;
    use embedded_graphics_core::Drawable;
    use heapless::String;

    use super::{FontLayout, PixelFont};

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

    impl<C: PixelColor> PixelFont<C> for embedded_graphics::mono_font::MonoFont<'_> {
        fn render_iter<T, I>(
            &self,
            target: &mut T,
            origin: crate::primitives::Point,
            color: C,
            characters: I,
        ) where
            T: DrawTarget<Color = C>,
            I: IntoIterator<Item = char>,
        {
            // embedded graphics Text is drawn at the baseline
            let mut origin: embedded_graphics_core::geometry::Point = origin.into();
            origin.y += self.baseline as i32;
            let style = MonoTextStyle::new(self, color);

            for character in characters {
                let text = String::<1>::from_iter(core::iter::once(character));
                origin = match Text::new(&text, origin, style).draw(target) {
                    Ok(o) => o,
                    Err(_) => break,
                };
            }
        }
    }
}
