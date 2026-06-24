use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusGroup},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A modifier that gates focus based on a focus group index.
///
/// This modifier only passes events matching the context's focus group index to its child.
/// A modifier that only allows focus for a specific group
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExclusiveFocus<T> {
    child: T,
    group: FocusGroup,
}

impl<T: ViewMarker> ExclusiveFocus<T> {
    #[must_use]
    pub fn new(child: T, group: FocusGroup) -> Self {
        Self { child, group }
    }
}

impl<T: ViewMarker> ViewMarker for ExclusiveFocus<T> {
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T: ViewLayout<Captures>> ViewLayout<Captures> for ExclusiveFocus<T>
where
    T::FocusTree: DefaultFocus,
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
        // Non-focus events
        let Event::Focus {
            group: event_group, ..
        } = event
        else {
            return self
                .child
                .handle_event(event, context, render_tree, captures, state, focus)
                .with_group(self.group);
        };

        // Only handle focus events if the group matches
        if *event_group == self.group {
            return self
                .child
                .handle_event(event, context, render_tree, captures, state, focus)
                .with_group(self.group);
        }

        EventResult::Deferred
    }
}
