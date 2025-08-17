use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render,
    view::{ViewLayout, ViewMarker},
};

/// A view modifier that adds a background color hint for fast simulated blending
#[derive(Debug, Clone)]
pub struct HintBackground<T, C> {
    inner: T,
    hint_color: C,
}

impl<T, C> HintBackground<T, C> {
    pub const fn new(inner: T, hint_color: C) -> Self {
        Self { inner, hint_color }
    }
}

impl<T, C> ViewMarker for HintBackground<T, C>
where
    T: ViewMarker,
    C: Clone,
{
    type Renderables = render::HintBackground<T::Renderables, C>;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, C> ViewLayout<Captures> for HintBackground<T, C>
where
    T: ViewLayout<Captures>,
    C: Clone,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.inner.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.inner.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        render::HintBackground::new(
            self.inner.render_tree(layout, origin, env, captures, state),
            self.hint_color.clone(),
        )
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.inner
            .handle_event(event, context, &mut render_tree.subtree, captures, state)
    }
}
