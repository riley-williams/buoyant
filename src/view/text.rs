use core::marker::PhantomData;

use wrap::WhitespaceWrap;

mod character;
mod wrap;

// W is hardcoded to WhitespaceWrap, leaving generic for future fix

pub struct Text<'a, T, F, W = WhitespaceWrap<'a, F>> {
    pub(crate) text: T,
    pub(crate) font: &'a F,
    pub(crate) alignment: HorizontalTextAlignment,
    pub(crate) _wrap: PhantomData<W>,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HorizontalTextAlignment {
    #[default]
    Leading,
    Center,
    Trailing,
}

impl HorizontalTextAlignment {
    pub(crate) fn align(&self, available: i16, content: i16) -> i16 {
        match self {
            Self::Leading => 0,
            Self::Center => (available - content) / 2,
            Self::Trailing => available - content,
        }
    }
}
