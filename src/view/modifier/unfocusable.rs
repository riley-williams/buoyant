use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::FocusTree,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A modifier that prevents a subtree from obtaining focus.
/// Touch events are always passed through.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unfocusable<T> {
    child: T,
}

impl<T: ViewMarker> Unfocusable<T> {
    #[must_use]
    pub fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T: ViewMarker> ViewMarker for Unfocusable<T> {
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T: ViewLayout<Captures>> ViewLayout<Captures> for Unfocusable<T>
where
    T::FocusTree: FocusTree,
{
    type Sublayout = T::Sublayout;
    type State = T::State;
    type FocusTree = T::FocusTree;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.child.transition()
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
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.child.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        // Only allow non-focus events
        let Event::Focus { .. } = event else {
            let mut result =
                self.child
                    .handle_event(event, context, render_tree, captures, state, focus);
            // Prevent `focus_touches` from giving this view focus when handling touch events
            if let EventResult::Handled {
                request_focus: focus_change @ true,
                ..
            } = &mut result
            {
                *focus_change = false;
            }
            return result;
        };

        EventResult::Deferred
    }
}
