extern crate alloc;
use crate::{
    environment::LayoutEnvironment,
    font::FontLayout,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{
        Renderable, {OwnedText, StaticText},
    },
};
use core::marker::PhantomData;

use super::{wrap::WhitespaceWrap, HorizontalTextAlignment, Text};

impl<'a, F> Text<'a, &'a str, F> {
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
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
impl<'a, F> Text<'a, alloc::string::String, F> {
    #[inline]
    #[must_use]
    pub fn string(text: alloc::string::String, font: &'a F) -> Self {
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
impl Slice for alloc::string::String {
    #[inline]
    fn as_slice(&self) -> &str {
        self.as_str()
    }
}

impl<T, F> Text<'_, T, F> {
    #[inline]
    #[must_use]
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }
}

impl<T: PartialEq, F> PartialEq for Text<'_, T, F> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl<'a, F: FontLayout> Layout for Text<'a, &'a str, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    #[inline]
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
        }
    }
}

impl<'a, C, F: FontLayout> Renderable<C> for Text<'a, &'a str, F> {
    type Renderables = StaticText<'a, F>;

    #[inline]
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        StaticText {
            text: self.text,
            font: self.font,
            origin,
            size: layout.resolved_size.into(),
            alignment: self.alignment,
        }
    }
}

impl<const N: usize, F: FontLayout> Layout for Text<'_, heapless::String<N>, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    #[inline]
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
        }
    }
}

impl<'a, const N: usize, C, F: FontLayout> Renderable<C> for Text<'a, heapless::String<N>, F> {
    type Renderables = OwnedText<'a, N, F>;

    #[inline]
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        OwnedText {
            text: self.text.clone(),
            font: self.font,
            origin,
            size: layout.resolved_size.into(),
            alignment: self.alignment,
        }
    }
}
