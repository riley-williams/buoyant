//! A modifier that bounds focus navigation within its subtree.
//!
//! This modifier prevents focus from escaping its child view, either by wrapping
//! around to the other end (Wrap) or by stopping at the boundary (Stop).

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{BoundaryBehavior, DefaultFocus, FocusAction, FocusDirection},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A modifier that bounds focus navigation within its subtree.
///
/// When focus tries to exit the bounded region (via Next/Previous navigation),
/// this modifier either wraps focus to the other end or stops at the boundary,
/// depending on the configured [`BoundaryBehavior`].
///
/// Focus events that should pass through:
/// - `Blur` - always deferred to parent
/// - `Select` - always deferred to parent if not handled
/// - `Teardown` - cleanup signal; must never trigger boundary re-acquisition
///
/// If the subtree contains no focusable elements, all focus events are deferred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundFocus<T> {
    child: T,
    behavior: BoundaryBehavior,
}

impl<T: ViewMarker> BoundFocus<T> {
    /// Creates a new `BoundFocus` modifier with the specified behavior.
    #[must_use]
    pub const fn new(child: T, behavior: BoundaryBehavior) -> Self {
        Self { child, behavior }
    }
}

impl<T> ViewMarker for BoundFocus<T>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for BoundFocus<T>
where
    T: ViewLayout<Captures>,
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
        // Non-focus events pass through transparently
        // We don't want to wrap searches for touch events infinitely
        let Event::Focus {
            action: focus_event,
            group,
        } = event
        else {
            return self
                .child
                .handle_event(event, context, render_tree, captures, state, focus);
        };

        // Blur and Select always pass through (defer to parent)
        if matches!(
            focus_event,
            FocusAction::Blur | FocusAction::Select | FocusAction::Teardown
        ) {
            return self
                .child
                .handle_event(event, context, render_tree, captures, state, focus);
        }

        // Try to handle the focus event in the child
        let result = self
            .child
            .handle_event(event, context, render_tree, captures, state, focus);

        // If handled, we're done
        if !matches!(result, EventResult::Deferred { .. }) {
            return result;
        }

        // Focus event was deferred, which means we've hit a boundary or there are
        // no focusable elements

        // Determine if we were moving forward or backward
        let is_forward = matches!(
            focus_event,
            FocusAction::Next | FocusAction::Focus(FocusDirection::Forward)
        );

        match self.behavior {
            BoundaryBehavior::Wrap => {
                // Reset focus tree to the opposite end
                *focus = if is_forward {
                    DefaultFocus::default_first()
                } else {
                    DefaultFocus::default_last()
                };

                // Acquire focus at the wrapped position
                let acquire_direction = if is_forward {
                    FocusDirection::Forward
                } else {
                    FocusDirection::Backward
                };

                self.child.handle_event(
                    &Event::Focus {
                        action: FocusAction::Focus(acquire_direction),
                        group: *group,
                    },
                    context,
                    render_tree,
                    captures,
                    state,
                    focus,
                )
            }
            BoundaryBehavior::Stop => {
                // Try to refocus on the element at the boundary we hit
                let refocus_direction = if is_forward {
                    // We went past the end, refocus backward to get the last element
                    FocusDirection::Backward
                } else {
                    // We went past the beginning, refocus forward to get the first element
                    FocusDirection::Forward
                };

                self.child.handle_event(
                    &Event::Focus {
                        action: FocusAction::Focus(refocus_direction),
                        group: *group,
                    },
                    context,
                    render_tree,
                    captures,
                    state,
                    focus,
                )
            }
        }
    }
}
