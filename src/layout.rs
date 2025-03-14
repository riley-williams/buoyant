use crate::{
    environment::LayoutEnvironment,
    primitives::{Dimensions, ProposedDimension, ProposedDimensions},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    /// Content is laid out horizontally, from left to right. Typically in a `HStack`
    #[default]
    Horizontal,
    /// Content is laid out vertically, from top to bottom. Typically in a `VStack`
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
            Self::TopLeading | Self::Leading | Self::BottomLeading => {
                HorizontalAlignment::Leading
            }
            Self::Top | Self::Center | Self::Bottom => HorizontalAlignment::Center,
            Self::TopTrailing | Self::Trailing | Self::BottomTrailing => {
                HorizontalAlignment::Trailing
            }
        }
    }

    /// The vertical component of the alignment
    #[must_use]
    pub const fn vertical(&self) -> VerticalAlignment {
        match self {
            Self::TopLeading | Self::Top | Self::TopTrailing => {
                VerticalAlignment::Top
            }
            Self::Leading | Self::Center | Self::Trailing => {
                VerticalAlignment::Center
            }
            Self::BottomLeading | Self::Bottom | Self::BottomTrailing => {
                VerticalAlignment::Bottom
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
            Self::Leading => 0,
            Self::Center => (available - content) / 2,
            Self::Trailing => available - content,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
            Self::Top => 0,
            Self::Center => (available - content) / 2,
            Self::Bottom => available - content,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            Self::FixedWidth(w) => ProposedDimensions {
                width: ProposedDimension::Exact(w),
                height: ProposedDimension::Exact(0),
            },
            Self::FixedHeight(h) => ProposedDimensions {
                width: ProposedDimension::Exact(0),
                height: ProposedDimension::Exact(h),
            },
        }
    }

    #[must_use]
    pub const fn into_max_proposal(self) -> ProposedDimensions {
        match self {
            Self::FixedWidth(w) => ProposedDimensions {
                width: ProposedDimension::Exact(w),
                height: ProposedDimension::Infinite,
            },
            Self::FixedHeight(h) => ProposedDimensions {
                width: ProposedDimension::Infinite,
                height: ProposedDimension::Exact(h),
            },
        }
    }
}

pub trait Layout: Sized {
    type Sublayout: Clone + PartialEq;
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
