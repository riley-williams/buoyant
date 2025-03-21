use crate::{
    environment::LayoutEnvironment,
    font::FontLayout,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{self, Renderable},
};
use core::marker::PhantomData;

pub use wrap::WhitespaceWrap;

mod wrap;

// W is hardcoded elsewhere to WhitespaceWrap, leaving generic for future fix

#[derive(Debug, Clone)]
pub struct Text<'a, T, F, W = WhitespaceWrap<'a, F>> {
    pub(crate) text: T,
    pub(crate) font: &'a F,
    pub(crate) alignment: HorizontalTextAlignment,
    pub(crate) _wrap: PhantomData<W>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalTextAlignment {
    #[default]
    Leading,
    Center,
    Trailing,
}

impl HorizontalTextAlignment {
    pub(crate) const fn align(self, available: i16, content: i16) -> i16 {
        match self {
            Self::Leading => 0,
            Self::Center => (available - content) / 2,
            Self::Trailing => available - content,
        }
    }
}

impl<'a, T: AsRef<str>, F> Text<'a, T, F> {
    #[must_use]
    pub fn new(text: T, font: &'a F) -> Self {
        Self {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            _wrap: PhantomData,
        }
    }
}

impl<T, F> Text<'_, T, F> {
    #[must_use]
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }
}

impl<T: PartialEq, F> PartialEq for Text<'_, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl<T: AsRef<str>, F: FontLayout> Layout for Text<'_, T, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let line_height = self.font.line_height();
        let wrap = WhitespaceWrap::new(self.text.as_ref(), offer.width, self.font);
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

impl<'a, T: AsRef<str> + Clone, F: FontLayout> Renderable for Text<'a, T, F> {
    type Renderables = render::Text<'a, T, F>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        render::Text {
            text: self.text.clone(),
            font: self.font,
            origin,
            size: layout.resolved_size.into(),
            alignment: self.alignment,
        }
    }
}
