use embedded_touch::{Phase, Touch};

use crate::{
    environment::LayoutEnvironment,
    event::{EventResult, TouchResult},
    layout::ResolvedLayout,
    primitives::{Angle, Dimensions, Point, ProposedDimensions, geometry::Shape as _},
    render::shape::AsShapePrimitive as _,
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// An arc along the edge of a circle which takes space greedily on both axes while
/// maintaining a square aspect ratio.
///
/// Angles are measured from the trailing edge (3 o'clock), increasing clockwise. If `end`
/// is less than `start`, the arc is drawn counterclockwise.
///
/// Arcs have no interior, so they are only visible when stroked with
/// [`Shape::stroked`][`super::Shape::stroked`] or
/// [`Shape::stroked_offset`][`super::Shape::stroked_offset`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Arc {
    start: Angle,
    end: Angle,
}

impl Arc {
    /// Creates an arc from the `start` angle to the `end` angle.
    #[must_use]
    pub const fn new(start: Angle, end: Angle) -> Self {
        Self { start, end }
    }
}

impl ViewMarker for Arc {
    type Renderables = crate::render::Arc;
    type Transition = Opacity;
}

impl<Captures: ?Sized> ViewLayout<Captures> for Arc {
    type State = ();
    type Sublayout = crate::primitives::Dimension;
    type FocusTree = ();

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
        let minimum_dimension = offer.width.min(offer.height).resolve_most_flexible(0, 1);

        ResolvedLayout {
            sublayouts: minimum_dimension,
            resolved_size: Dimensions {
                width: minimum_dimension,
                height: minimum_dimension,
            },
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
        crate::render::Arc {
            origin,
            diameter: (*layout).into(),
            start: self.start,
            end: self.end,
        }
    }

    fn handle_event(
        &self,
        _event: &crate::view::Event,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> EventResult {
        EventResult::Deferred
    }

    fn handle_touch(
        &self,
        touch: &Touch,
        _context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> TouchResult<Self::FocusTree> {
        if touch.phase == Phase::Started
            && render_tree
                .as_shape()
                .bounding_box()
                .contains(&touch.location.into())
        {
            TouchResult::Handled
        } else {
            TouchResult::Deferred
        }
    }
}
