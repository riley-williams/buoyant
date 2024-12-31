use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    font::{CharacterFont, FontLayout},
    layout::{Layout, ResolvedLayout},
    pixel::Interpolate as _,
    primitives::{Dimensions, Point, ProposedDimension, ProposedDimensions, Size},
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

impl<T, F> Text<'_, T, F> {
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }
}

impl<T: PartialEq, F> PartialEq for Text<'_, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

// TODO: consolidate the layout implementations...this is getting ridiculous

impl<T: Slice, F: FontLayout> Layout for Text<'_, T, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let line_height = self.font.line_height();
        let wrap = WhitespaceWrap::new(self.text.as_slice(), offer.width, self.font);
        let mut size = Size::zero();
        for line in wrap {
            size.width = core::cmp::max(size.width, self.font.str_width(line));
            size.height += line_height;
            if ProposedDimension::Exact(size.height) >= offer.height {
                break;
            }
        }

        ResolvedLayout {
            sublayouts: (),
            resolved_size: size.into(),
            origin: Point::zero(),
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
        layout.origin = origin;
    }
}

impl<T: Slice, F: CharacterFont<Color>, Color: Copy> CharacterRender<Color> for Text<'_, T, F> {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Color>,
        layout: &ResolvedLayout<()>,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        if layout.resolved_size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(
            self.text.as_slice(),
            ProposedDimension::Exact(layout.resolved_size.width.into()),
            self.font,
        );
        for line in wrap {
            let color = env.foreground_color();
            let width = self.font.str_width(line);

            let x = self
                .alignment
                .align(layout.resolved_size.width.into(), width as i16);
            self.font.render_iter_solid(
                target,
                Point::new(layout.origin.x + x, layout.origin.y + height),
                color,
                line.chars(),
            );

            height += line_height;
            if height >= layout.resolved_size.height.into() {
                break;
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<
        T: Slice,
        F: crate::font::PixelFont<Color>,
        Color: embedded_graphics_core::pixelcolor::PixelColor,
    > crate::render::PixelRender<Color> for Text<'_, T, F>
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Color>,
        layout: &ResolvedLayout<()>,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        if layout.resolved_size.area() == 0 {
            return;
        }

        let line_height = self.font.line_height() as i16;

        let mut height = 0;
        let wrap = WhitespaceWrap::new(
            self.text.as_slice(),
            ProposedDimension::Exact(layout.resolved_size.width.into()),
            self.font,
        );
        for line in wrap {
            let color = env.foreground_color();
            let width = self.font.str_width(line);

            let x = self
                .alignment
                .align(layout.resolved_size.width.into(), width as i16);
            self.font.render_iter(
                target,
                Point::new(layout.origin.x + x, layout.origin.y + height),
                color,
                line.chars(),
            );

            height += line_height;
            if height >= layout.resolved_size.height.into() {
                break;
            }
        }
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Color>,
        _source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        _source_env: &impl RenderEnvironment<Color = Color>,
        target_env: &impl RenderEnvironment<Color = Color>,
        config: &crate::render::AnimationConfiguration,
    ) {
        let origin = Point::interpolate(source_layout.origin, target_layout.origin, config.factor);
        let size = Dimensions::interpolate(
            source_layout.resolved_size,
            target_layout.resolved_size,
            config.factor,
        );
        let interpolated_layout = ResolvedLayout {
            origin,
            resolved_size: size,
            sublayouts: (),
        };
        target_view.render(target, &interpolated_layout, target_env);
    }
}
