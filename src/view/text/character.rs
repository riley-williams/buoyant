use crate::{
    environment::LayoutEnvironment,
    font::FontLayout,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{OwnedText, Renderable},
};
use core::marker::PhantomData;

use super::{wrap::WhitespaceWrap, HorizontalTextAlignment, Text};

impl<'a, T: AsRef<str>, F> Text<'a, T, F> {
    #[must_use]
    pub fn new(text: T, font: &'a F) -> Self {
        Text {
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

impl<'a, T: AsRef<str> + Clone, C, F: FontLayout> Renderable<C> for Text<'a, T, F> {
    type Renderables = OwnedText<'a, T, F>;

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
