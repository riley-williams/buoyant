use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    focus::{FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// A rounded rectangle which takes space greedily on both axes.
///
/// By default, this renders a filled shape with the inherited foreground color.
/// To render with a stroke instead, use [`ShapeExt::stroked`][`super::ShapeExt::stroked`]
/// or [`ShapeExt::stroked_offset`][`super::ShapeExt::stroked_offset`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
}

impl RoundedRectangle {
    /// A flexible rounded rectangle with the specified corner radius.
    #[must_use]
    pub const fn new(corner_radius: u16) -> Self {
        Self { corner_radius }
    }
}

impl ViewMarker for RoundedRectangle {
    type Renderables = crate::render::RoundedRect;
    type Transition = Opacity;
}

impl<Captures: ?Sized> ViewLayout<Captures> for RoundedRectangle {
    type Sublayout = Dimensions;
    type State = ();
    type FocusTree = ();

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
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
        crate::render::RoundedRect {
            origin,
            size: (*layout).into(),
            corner_radius: self.corner_radius,
        }
    }

    fn handle_event(
        &self,
        _event: &crate::view::Event,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> EventResult {
        EventResult::default()
    }

    fn focus(
        &self,
        _event: &FocusEvent,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        FocusStateChange::Exhausted
    }
}
