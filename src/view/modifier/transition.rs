use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    focus::{FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Modifies the transition used to transition this view tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition<V, T> {
    child: V,
    transition: T,
}

impl<V: ViewMarker, T> Transition<V, T> {
    #[allow(missing_docs)]
    pub const fn new(transition: T, child: V) -> Self {
        Self { child, transition }
    }
}

impl<V, T> ViewMarker for Transition<V, T>
where
    V: ViewMarker,
    T: crate::transition::Transition,
{
    type Renderables = V::Renderables;
    type Transition = T;
}

impl<Captures: ?Sized, V, T> ViewLayout<Captures> for Transition<V, T>
where
    V: ViewLayout<Captures>,
    T: crate::transition::Transition,
{
    type Sublayout = V::Sublayout;
    type State = V::State;
    type FocusTree = V::FocusTree;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.transition.clone()
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
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.child
            .handle_event(event, context, render_tree, captures, state)
    }

    fn focus(
        &self,
        event: &FocusEvent,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        self.child
            .focus(event, context, render_tree, captures, state, focus)
    }
}
