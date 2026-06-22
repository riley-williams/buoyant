//! A view that manages pagination of a child view using a separate focus group to
//! trigger page changes.

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusAction, FocusDirection, FocusGroup},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::IntrinsicShape,
    view::{ViewLayout, ViewMarker},
};

/// This view is experimental and may change or be removed without a major version bump.
///
/// A view that manages pagination of a child view, allowing focus in a separate group
/// to trigger page changes.
#[derive(Clone, Debug)]
pub struct Paginate<V, Action> {
    view: V,
    group: FocusGroup,
    action: Action,
}

/// An event indicating a page should change, triggered by focus events on the pagination
/// group.
#[derive(Clone, Debug)]
pub enum PageEvent {
    #[expect(missing_docs)]
    Next,
    #[expect(missing_docs)]
    Previous,
}

#[expect(missing_docs)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaginateState<T> {
    child: T,
}

impl<V: ViewMarker, Action> Paginate<V, Action> {
    /// Create a new paginate view with the given child view, focus group, and action.
    /// If forceful, focus events on the pagination group will trigger page changes
    /// even if the child view currently holds focus.
    #[must_use]
    pub fn new<C>(group: FocusGroup, _forceful: bool, action: Action, view: V) -> Self
    where
        V: ViewLayout<C>,
        Action: Fn(&mut C, PageEvent),
    {
        Self {
            view,
            group,
            action,
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub struct PaginateFocusTree<T> {
    child: T,
    child_holds_focus: bool,
}

impl<T: DefaultFocus> DefaultFocus for PaginateFocusTree<T> {
    fn default_first() -> Self {
        Self {
            child: T::default_first(),
            child_holds_focus: false,
        }
    }

    fn default_last() -> Self {
        Self {
            child: T::default_last(),
            child_holds_focus: false,
        }
    }
}

impl<V: ViewMarker, Action> ViewMarker for Paginate<V, Action> {
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<C, V, Action> ViewLayout<C> for Paginate<V, Action>
where
    V: ViewLayout<C, Renderables: IntrinsicShape>,
    Action: Fn(&mut C, PageEvent),
{
    type State = PaginateState<V::State>;

    type Sublayout = V::Sublayout;

    type FocusTree = PaginateFocusTree<V::FocusTree>;

    fn transition(&self) -> Self::Transition {
        self.view.transition()
    }

    fn build_state(&self, captures: &mut C) -> Self::State {
        PaginateState {
            child: self.view.build_state(captures),
        }
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.view.layout(offer, env, captures, &mut state.child)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.view
            .render_tree(layout, origin, env, captures, &mut state.child)
    }

    #[expect(clippy::too_many_lines)]
    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        if let Event::Focus { action, group } = event {
            // always forward teardown events
            if *action == FocusAction::Teardown {
                return self.view.handle_event(
                    event,
                    context,
                    render_tree,
                    captures,
                    &mut state.child,
                    &mut focus.child,
                );
            }

            // Events directed at the pagination group should trigger the action
            if self.group == *group {
                match action {
                    FocusAction::Next => {
                        (self.action)(captures, PageEvent::Next);
                        context.request_view_rebuild();
                    }
                    FocusAction::Previous => {
                        (self.action)(captures, PageEvent::Previous);
                        context.request_view_rebuild();
                    }
                    // FIXME: not quite right...?
                    _ => return EventResult::deferred(),
                }
                return EventResult::handled_focused(render_tree.content_shape());
            }

            // group doesn't match, try child if it's focused
            if focus.child_holds_focus {
                match action {
                    FocusAction::Blur => {
                        let result = self.view.handle_event(
                            event,
                            context,
                            render_tree,
                            captures,
                            &mut state.child,
                            &mut focus.child,
                        );
                        if result.is_handled() {
                            return result;
                        }
                        focus.child_holds_focus = false;
                        return EventResult::handled_focused(render_tree.content_shape());
                    }
                    _ => {
                        return self.view.handle_event(
                            event,
                            context,
                            render_tree,
                            captures,
                            &mut state.child,
                            &mut focus.child,
                        );
                    }
                }
            }
            // child doesn't hold focus
            match action {
                FocusAction::Next => {
                    (self.action)(captures, PageEvent::Next);
                    context.request_view_rebuild();
                    return EventResult::handled_focused(render_tree.content_shape());
                }
                FocusAction::Previous => {
                    (self.action)(captures, PageEvent::Previous);
                    context.request_view_rebuild();
                    return EventResult::handled_focused(render_tree.content_shape());
                }
                FocusAction::Focus(_) => {
                    return EventResult::handled_focused(render_tree.content_shape());
                }
                FocusAction::Blur => return EventResult::deferred(),
                FocusAction::Select => {
                    focus.child_holds_focus = true;
                    context.request_view_rebuild();
                    // Notify the child that it has focus
                    return self.view.handle_event(
                        &Event::Focus {
                            action: FocusAction::Focus(FocusDirection::Forward),
                            group: *group,
                        },
                        context,
                        render_tree,
                        captures,
                        &mut state.child,
                        &mut focus.child,
                    );
                }
                FocusAction::Teardown => return EventResult::handled_unfocused(),
            }
        }
        // Ignore key events when child is unfocused
        if matches!(event, Event::KeyUp { .. } | Event::KeyDown { .. }) && !focus.child_holds_focus
        {
            return EventResult::deferred();
        }

        // For all other events, delegate to inner view.
        let result = self.view.handle_event(
            event,
            context,
            render_tree,
            captures,
            &mut state.child,
            &mut focus.child,
        );

        // Allow touches to enter focus
        if let Event::Touch(_) = *event
            && result.requested_focus()
        {
            focus.child_holds_focus = true;
        }
        result
    }
}
