//! A view that manages pagination of a child view using a separate focus group to
//! trigger page changes.

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusAction, FocusGroup},
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageState {
    UnFocused,
    Focused,
    Captive,
}

impl<V: ViewMarker, Action> Paginate<V, Action> {
    #[expect(missing_docs)]
    #[must_use]
    pub fn new<C>(group: FocusGroup, action: Action, view: V) -> Self
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
    inner: T,
}

impl<T: DefaultFocus> DefaultFocus for PaginateFocusTree<T> {
    fn default_first() -> Self {
        Self {
            inner: T::default_first(),
        }
    }

    fn default_last() -> Self {
        Self {
            inner: T::default_last(),
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
    type State = V::State;

    type Sublayout = V::Sublayout;

    type FocusTree = PaginateFocusTree<V::FocusTree>;

    fn transition(&self) -> Self::Transition {
        self.view.transition()
    }

    fn build_state(&self, captures: &mut C) -> Self::State {
        self.view.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.view.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.view.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        if let Event::Focus { action, group } = event
            && *group == self.group
        {
            // Events directed at the pagination group should trigger the action
            match action {
                FocusAction::Next => {
                    (self.action)(captures, PageEvent::Next);
                    context.request_view_rebuild();
                }
                FocusAction::Previous => {
                    (self.action)(captures, PageEvent::Previous);
                    context.request_view_rebuild();
                }
                _ => {}
            }
            return EventResult::handled_focused(render_tree.content_shape());
        }

        // For non-focus events, delegate to inner view.
        self.view.handle_event(
            event,
            context,
            render_tree,
            captures,
            state,
            &mut focus.inner,
        )
    }
}
