use crate::{
    environment::LayoutEnvironment,
    primitives::{Dimensions, Point, ProposedDimension, ProposedDimensions},
};

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

#[derive(Clone, PartialEq)]
pub struct ResolvedLayout<C: Clone + PartialEq> {
    pub sublayouts: C,
    pub resolved_size: Dimensions,
    pub origin: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    FixedWidth(u16),
    FixedHeight(u16),
}

impl Axis {
    pub fn into_min_proposal(self) -> ProposedDimensions {
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

    pub fn into_max_proposal(self) -> ProposedDimensions {
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

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    );

    // TODO: This should not be part of the trait itself
    fn layout_and_place(
        &self,
        offer: impl Into<ProposedDimensions>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let offer = offer.into();
        let mut layout = self.layout(&offer, env);
        self.place_subviews(&mut layout, origin, env);
        layout
    }

    /// The layout priority of the view. Higher priority views are more likely to be given the size they want
    fn priority(&self) -> i8 {
        0
    }

    /// Returns true if the view should not included in layout. ConditionalView is the primary example of this
    fn is_empty(&self) -> bool {
        false
    }
}
