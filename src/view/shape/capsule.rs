use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions, Size},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// A capsule primitive, oriented horizontally
///
/// By default, this renders a filled shape with the inherited foreground color.
/// To render with a stroke instead, use [`ShapeExt::stroked`][`super::ShapeExt::stroked`]
/// or [`ShapeExt::stroked_offset`][`super::ShapeExt::stroked_offset`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Capsule;

impl ViewMarker for Capsule {
    type Renderables = crate::render::Capsule;
    type Transition = Opacity;
}

impl<Captures: ?Sized> ViewLayout<Captures> for Capsule {
    type State = ();
    type Sublayout = Dimensions;
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
        let dimensions = offer.resolve_most_flexible(Size::zero(), Size::new(1, 1));
        ResolvedLayout {
            sublayouts: dimensions,
            resolved_size: offer.resolve_most_flexible(Size::zero(), Size::new(1, 1)),
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
        crate::render::Capsule {
            origin,
            size: (*layout).into(),
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
        EventResult::deferred()
    }
}
