use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{FocusAction, FocusDirection, FocusTree},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A modifier that handles touch events by moving focus to a tapped child when
/// it reports that it handled the event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusTouches<T> {
    child: T,
}

impl<T: ViewMarker> FocusTouches<T> {
    #[must_use]
    pub const fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T: ViewMarker> ViewMarker for FocusTouches<T> {
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T: ViewLayout<Captures>> ViewLayout<Captures> for FocusTouches<T>
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
        // Non-touch events are unaffected
        let Event::Touch(_) = event else {
            return self
                .child
                .handle_event(event, context, render_tree, captures, state, focus);
        };

        // TODO: Which occurs more often: a touch event that changes focus, or one that doesn't?
        // We can save some moves if this is known.
        let mut candidate_focus = focus.clone();

        // Handle the event with the cloned candidate focus and only apply focus
        // changes if the event was handled

        let result = self.child.handle_event(
            event,
            context,
            render_tree,
            captures,
            state,
            &mut candidate_focus,
        );

        if let EventResult::Handled {
            request_focus: true,
            group,
            ..
        } = result
        {
            _ = self.child.handle_event(
                &Event::Focus {
                    action: FocusAction::Teardown,
                    group,
                },
                context,
                render_tree,
                captures,
                state,
                focus,
            );
            *focus = candidate_focus;
            // Reassert original focus
            _ = self.child.handle_event(
                &Event::Focus {
                    action: FocusAction::Focus(FocusDirection::Forward),
                    group,
                },
                context,
                render_tree,
                captures,
                state,
                focus,
            );
        }

        result
    }
}
