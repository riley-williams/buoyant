use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    font::{CharacterFont, FontLayout},
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};
use core::marker::PhantomData;

use super::{wrap::WhitespaceWrap, HorizontalTextAlignment, Text};

impl<'a, F> Text<'a, &'a str, F> {
    pub fn str(text: &'a str, font: &'a F) -> Self {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            _wrap: PhantomData,
        }
    }
}

impl<'a, const N: usize, F> Text<'a, heapless::String<N>, F> {
    pub fn heapless(text: heapless::String<N>, font: &'a F) -> Self {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            _wrap: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<'a, F> Text<'a, String, F> {
    pub fn string(text: String, font: &'a F) -> Self {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            _wrap: PhantomData,
        }
    }
}

trait Slice {
    fn as_slice(&self) -> &str;
}

impl Slice for &str {
    #[inline]
    fn as_slice(&self) -> &str {
        self
    }
}

impl<const N: usize> Slice for heapless::String<N> {
    #[inline]
    fn as_slice(&self) -> &str {
        self
    }
}

#[cfg(feature = "std")]
impl Slice for String {
    #[inline]
    fn as_slice(&self) -> &str {
        self.as_str()
    }
}

impl<'a, T, F> Text<'a, T, F> {
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }
}

impl<'a, T: PartialEq, F> PartialEq for Text<'a, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

// TODO: consolidate the layout implementations...this is getting ridiculous

impl<'a, T: Slice, F: FontLayout> Layout for Text<'a, T, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    fn layout(
        &self,
        offer: Size,
        _env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        if offer.area() == 0 {
            return ResolvedLayout {
                sublayouts: (),
                resolved_size: Size::new(0, 0),
            };
        }
        let line_height = self.font.line_height();
        let wrap = WhitespaceWrap::new(self.text.as_slice(), offer.width, self.font);
        let mut size = Size::zero();
        for line in wrap {
            size.width = core::cmp::max(size.width, self.font.str_width(line));
            size.height += line_height;
            if size.height >= offer.height {
                break;
            }
        }

        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }
}

impl<'a, T: Slice, F: CharacterFont<Color>, Color: Copy> CharacterRender<Color> for Text<'a, T, F> {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Color>,
        layout: &ResolvedLayout<()>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        if layout.resolved_size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text.as_slice(), layout.resolved_size.width, self.font);
        for line in wrap {
            let color = env.foreground_color();
            let width = self.font.str_width(line);

            let x = self
                .alignment
                .align(layout.resolved_size.width as i16, width as i16);
            self.font.render_iter_solid(
                target,
                Point::new(origin.x + x, origin.y + height),
                color,
                line.chars(),
            );

            height += line_height;
            if height >= layout.resolved_size.height as i16 {
                break;
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<
        'a,
        T: Slice,
        F: crate::font::PixelFont<Color>,
        Color: embedded_graphics_core::pixelcolor::PixelColor,
    > crate::render::PixelRender<Color> for Text<'a, T, F>
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Color>,
        layout: &ResolvedLayout<()>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        if layout.resolved_size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text.as_slice(), layout.resolved_size.width, self.font);
        for line in wrap {
            let color = env.foreground_color();
            let width = self.font.str_width(line);

            let x = self
                .alignment
                .align(layout.resolved_size.width as i16, width as i16);
            self.font.render_iter(
                target,
                Point::new(origin.x + x, origin.y + height),
                color,
                line.chars(),
            );

            height += line_height;
            if height >= layout.resolved_size.height as i16 {
                break;
            }
        }
    }
}
