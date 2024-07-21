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
    fn render_character<T>(&self, target: &mut T, origin: Point, color: C, character: char)
    where
        T: RenderTarget<Color = C>;
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
    fn render_character<T>(&self, target: &mut T, origin: Point, _: char, character: char)
    where
        T: RenderTarget<Color = char>,
    {
        target.draw(origin, character);
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
        fn render_character<T>(
            &self,
            target: &mut T,
            origin: Point,
            color: CrosstermColorSymbol,
            character: char,
        ) where
            T: RenderTarget<Color = CrosstermColorSymbol>,
        {
            target.draw(origin, color.with_character(character));
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_fonts {
    use embedded_graphics::{geometry::OriginDimensions, mono_font::MonoTextStyle, text::Text};
    use embedded_graphics_core::pixelcolor::PixelColor as EmbeddedPixelColor;
    use embedded_graphics_core::Drawable;

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
        fn render_character<T>(
            &self,
            target: &mut T,
            origin: crate::primitives::Point,
            color: C,
            character: char,
        ) where
            T: RenderTarget<Color = C>,
        {
            let style = MonoTextStyle::new(self, color);
            let mut proxy = ProxyTarget { target };

            _ = Text::new(&character.to_string(), origin.into(), style).draw(&mut proxy);
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
