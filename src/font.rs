use crate::{pixel::PixelColor, primitives::Point, render_target::RenderTarget};

/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait CharacterFontLayout {
    /// The height of a character in points
    fn line_height(&self) -> u16;
    /// The width of a character in points
    fn character_width(&self, character: char) -> u16;
}
///
/// A font that renders individual characters at a time.
/// Multi-character graphemes are not supported, making
/// this primarily useful for embedded devices.
pub trait CharacterFont<C: PixelColor>: CharacterFontLayout {
    fn render_iter<T, I>(&self, target: &mut T, origin: Point, color: C, characters: I)
    where
        T: RenderTarget<Color = C>,
        I: IntoIterator<Item = char>;
}

/// A simple font for rendering non-unicode characters in a text buffer
/// The width and height of all characters is 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferCharacterFont;

impl CharacterFontLayout for BufferCharacterFont {
    #[inline]
    fn line_height(&self) -> u16 {
        1
    }

    #[inline]
    fn character_width(&self, _: char) -> u16 {
        1
    }
}

impl CharacterFont<char> for BufferCharacterFont {
    fn render_iter<T, I>(&self, target: &mut T, origin: Point, _color: char, characters: I)
    where
        T: RenderTarget<Color = char>,
        I: IntoIterator<Item = char>,
    {
        for (i, character) in characters.into_iter().enumerate() {
            target.draw(origin + Point::new(i as i16, 0), character);
        }
    }
}

#[cfg(feature = "crossterm")]
pub use crossterm_font::TerminalCharFont;

#[cfg(feature = "crossterm")]
mod crossterm_font {
    use crate::{pixel::CrosstermColorSymbol, primitives::Point, render_target::RenderTarget};

    use super::{CharacterFont, CharacterFontLayout};

    /// A simple font for rendering non-unicode characters in a text buffer
    /// The width and height of all characters is 1.
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TerminalCharFont;

    impl CharacterFontLayout for TerminalCharFont {
        #[inline]
        fn line_height(&self) -> u16 {
            1
        }

        #[inline]
        fn character_width(&self, _: char) -> u16 {
            1
        }
    }

    impl CharacterFont<CrosstermColorSymbol> for TerminalCharFont {
        fn render_iter<T, I>(
            &self,
            target: &mut T,
            origin: Point,
            color: CrosstermColorSymbol,
            characters: I,
        ) where
            T: RenderTarget<Color = CrosstermColorSymbol>,
            I: IntoIterator<Item = char>,
        {
            for (i, character) in characters.into_iter().enumerate() {
                target.draw(
                    origin + Point::new(i as i16, 0),
                    color.with_character(character),
                );
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_fonts {
    use embedded_graphics::{geometry::OriginDimensions, mono_font::MonoTextStyle, text::Text};
    use embedded_graphics_core::pixelcolor::PixelColor as EmbeddedPixelColor;
    use embedded_graphics_core::Drawable;
    use heapless::String;

    use crate::{pixel::PixelColor, render_target::RenderTarget};

    use super::{CharacterFont, CharacterFontLayout};

    impl CharacterFontLayout for embedded_graphics::mono_font::MonoFont<'_> {
        #[inline]
        fn line_height(&self) -> u16 {
            self.character_size.height as u16
        }

        #[inline]
        fn character_width(&self, _: char) -> u16 {
            self.character_size.width as u16
        }
    }

    impl<C: PixelColor + EmbeddedPixelColor> CharacterFont<C>
        for embedded_graphics::mono_font::MonoFont<'_>
    {
        fn render_iter<T, I>(
            &self,
            target: &mut T,
            mut origin: crate::primitives::Point,
            color: C,
            characters: I,
        ) where
            T: RenderTarget<Color = C>,
            I: IntoIterator<Item = char>,
        {
            let style = MonoTextStyle::new(self, color);
            let mut proxy = ProxyTarget { target };
            for character in characters {
                let text = String::<1>::from_iter(core::iter::once(character));
                _ = Text::new(&text, origin.into(), style).draw(&mut proxy);
                origin.x += self.character_width(character) as i16;
            }
        }
    }

    struct ProxyTarget<'a, T> {
        target: &'a mut T,
    }

    impl<D, C> embedded_graphics_core::draw_target::DrawTarget for ProxyTarget<'_, D>
    where
        D: RenderTarget<Color = C>,
        C: EmbeddedPixelColor + PixelColor,
    {
        type Color = C;
        type Error = ();

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
        {
            self.target.draw_iter(pixels.into_iter().map(Into::into));
            Ok(())
        }
    }

    impl<D> OriginDimensions for ProxyTarget<'_, D>
    where
        D: RenderTarget,
    {
        fn size(&self) -> embedded_graphics::geometry::Size {
            self.target.size().into()
        }
    }
}
