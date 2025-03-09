use crate::{
    environment::LayoutEnvironment,
    primitives::{Dimensions, ProposedDimension, ProposedDimensions},
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LayoutDirection {
    /// Content is laid out horizontally, from left to right. Typically in a `HStack`
    #[default]
    Horizontal,
    /// Content is laid out vertically, from top to bottom. Typically in a `VStack`
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Alignment {
    TopLeading,
    Top,
    TopTrailing,
    Leading,
    #[default]
    Center,
    Trailing,
    BottomLeading,
    Bottom,
    BottomTrailing,
}

impl Alignment {
    /// The horizontal component of the alignment
    #[must_use]
    pub const fn horizontal(&self) -> HorizontalAlignment {
        match self {
            Alignment::TopLeading | Alignment::Leading | Alignment::BottomLeading => {
                HorizontalAlignment::Leading
            }
            Alignment::Top | Alignment::Center | Alignment::Bottom => HorizontalAlignment::Center,
            Alignment::TopTrailing | Alignment::Trailing | Alignment::BottomTrailing => {
                HorizontalAlignment::Trailing
            }
        }
    }

    /// The vertical component of the alignment
    #[must_use]
    pub const fn vertical(&self) -> VerticalAlignment {
        match self {
            Alignment::TopLeading | Alignment::Top | Alignment::TopTrailing => {
                VerticalAlignment::Top
            }
            Alignment::Leading | Alignment::Center | Alignment::Trailing => {
                VerticalAlignment::Center
            }
            Alignment::BottomLeading | Alignment::Bottom | Alignment::BottomTrailing => {
                VerticalAlignment::Bottom
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HorizontalAlignment {
    /// Align the content to the leading edge
    Leading,
    /// Align the content to the center
    #[default]
    Center,
    /// Align the content to the trailing edge
    Trailing,
}

impl HorizontalAlignment {
    #[must_use]
    pub const fn align(&self, available: i16, content: i16) -> i16 {
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
    /// Align the content to the top edge
    Top,
    /// Align the content to the center
    #[default]
    Center,
    /// Align the content to the bottom edge
    Bottom,
}

impl VerticalAlignment {
    #[must_use]
    pub const fn align(&self, available: i16, content: i16) -> i16 {
        match self {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => (available - content) / 2,
            VerticalAlignment::Bottom => available - content,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedLayout<C: Clone + PartialEq> {
    pub sublayouts: C,
    pub resolved_size: Dimensions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    FixedWidth(u16),
    FixedHeight(u16),
}

impl Axis {
    #[must_use]
    pub const fn into_min_proposal(self) -> ProposedDimensions {
        match self {
            Axis::FixedWidth(w) => ProposedDimensions {
                width: ProposedDimension::Exact(w),
                height: ProposedDimension::Exact(0),
            },
            Axis::FixedHeight(h) => ProposedDimensions {
                width: ProposedDimension::Exact(0),
                height: ProposedDimension::Exact(h),
            },
        }
    }

    #[must_use]
    pub const fn into_max_proposal(self) -> ProposedDimensions {
        match self {
            Axis::FixedWidth(w) => ProposedDimensions {
                width: ProposedDimension::Exact(w),
                height: ProposedDimension::Infinite,
            },
            Axis::FixedHeight(h) => ProposedDimensions {
                width: ProposedDimension::Infinite,
                height: ProposedDimension::Exact(h),
            },
        }
    }
}

pub trait Layout: Sized {
    type Sublayout: Clone + PartialEq;
    // fn flexibility(&self, axis: Axis, env: &impl LayoutEnvironment) -> (Dimensions, Dimensions);

    /// The size of the view given the offer
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout>;

    /// The layout priority of the view. Higher priority views are more likely to be given the size they want
    fn priority(&self) -> i8 {
        0
    }

    /// Returns true if the view should not included in layout. `ConditionalView` is the primary example of this
    fn is_empty(&self) -> bool {
        false
    }
}
