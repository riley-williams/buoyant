use core::time::Duration;

use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Offsets the rendered position of a child view by a given point.
#[derive(Debug, Clone)]
pub struct Offset<T> {
    child: T,
    offset: Point,
}

impl<T> Offset<T> {
    pub const fn new(child: T, offset: Point) -> Self {
        Self { child, offset }
    }
}

impl<T> ViewMarker for Offset<T>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for Offset<T>
where
    T: ViewLayout<Captures>,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.child.build_state(captures)
    }
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let origin = origin + self.offset;
        self.child.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        app_time: Duration,
    ) -> EventResult {
        // FIXME: Apply offset to event coordinates
        self.child
            .handle_event(event, render_tree, captures, state, app_time)
    }
}
