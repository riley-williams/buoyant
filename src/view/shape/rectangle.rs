use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

use super::RoundedRectangle;

/// A rectangle which takes space greedily on both axes.
///
/// By default, this renders a filled shape with the inherited foreground color.
/// To render with a stroke instead, use [`ShapeExt::stroked`][`super::ShapeExt::stroked`]
/// or [`ShapeExt::stroked_offset`][`super::ShapeExt::stroked_offset`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Rectangle;

impl Rectangle {
    /// Adds a corner radius to all corners. The radius will be capped to half the shorter
    /// dimension
    #[must_use]
    pub const fn corner_radius(self, radius: u16) -> RoundedRectangle {
        RoundedRectangle::new(radius)
    }
}

impl ViewMarker for Rectangle {
    type Renderables = crate::render::Rect;
    type Transition = Opacity;
}

impl<Captures: ?Sized> ViewLayout<Captures> for Rectangle {
    type State = ();
    type Sublayout = Dimensions;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let dimensions = offer.resolve_most_flexible(0, 1);
        ResolvedLayout {
            sublayouts: dimensions,
            resolved_size: dimensions,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        crate::render::Rect {
            origin,
            size: (*layout).into(),
        }
    }
}
