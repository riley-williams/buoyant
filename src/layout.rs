use crate::{environment::Environment, primitives::Size};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LayoutDirection {
    /// Content is laid out horizontally, from left to right. Typically in a HStack
    #[default]
    Horizontal,
    /// Content is laid out vertically, from top to bottom. Typically in a VStack
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HorizontalAlignment {
    /// Align the content to the start of the layout direction
    Leading,
    /// Align the content to the center of the layout direction
    #[default]
    Center,
    /// Align the content to the end of the layout direction
    Trailing,
}

impl HorizontalAlignment {
    pub fn align(&self, available: i16, content: i16) -> i16 {
        match self {
            HorizontalAlignment::Leading => 0,
            HorizontalAlignment::Center => (available - content) / 2,
            HorizontalAlignment::Trailing => available - content,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
/// Strategy to align the heights of items that do not fill the available frame height
pub enum VerticalAlignment {
    /// Align the content to the start of the layout direction
    Top,
    /// Align the content to the center of the layout direction
    #[default]
    Center,
    /// Align the content to the end of the layout direction
    Bottom,
}

impl VerticalAlignment {
    pub fn align(&self, available: i16, content: i16) -> i16 {
        match self {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => (available - content) / 2,
            VerticalAlignment::Bottom => available - content,
        }
    }
}

#[derive(Clone)]
pub struct ResolvedLayout<C: Clone> {
    pub sublayouts: C,
    pub resolved_size: Size,
}

pub trait Layout: Sized {
    type Sublayout: Clone;
    /// The size of the view given the offer
    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<Self::Sublayout>;
    /// The layout priority of the view. Higher priority views are more likely to be given the size they want
    fn priority(&self) -> i8 {
        0
    }
}
