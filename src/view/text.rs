mod character;

pub struct Text<'a, F> {
    pub(crate) text: &'a str,
    pub(crate) font: &'a F,
    pub(crate) alignment: HorizontalTextAlignment,
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
