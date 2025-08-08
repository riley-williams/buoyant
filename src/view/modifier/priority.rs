use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Sets the priority of the view layout.
///
/// Stack subviews will be laid out in groups of equal priority, with higher priority views being
/// laid out first. All the remaining space will be offered, meaning greedy views will leave
/// no space at all for lower priority views. Lower priority views which have a non-zero minimum
/// size in this scenario will cause the stack children to lay out outside the stack frame.
#[derive(Debug, Clone)]
pub struct Priority<T> {
    priority: i8,
    child: T,
}

impl<T> Priority<T> {
    #[allow(missing_docs)]
    pub const fn new(priority: i8, child: T) -> Self {
        Self { priority, child }
    }
}

impl<T> PartialEq for Priority<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<V> ViewMarker for Priority<V>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<Captures: ?Sized, V> ViewLayout<Captures> for Priority<V>
where
    V: ViewLayout<Captures>,
{
    type Sublayout = V::Sublayout;
    type State = V::State;

    fn priority(&self) -> i8 {
        self.priority
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
        layout: &ResolvedLayout<Self::Sublayout>,
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
}
