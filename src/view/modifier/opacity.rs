use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render,
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opacity<V> {
    pub inner: V,
    pub opacity: u8,
}

impl<V> ViewMarker for Opacity<V>
where
    V: ViewMarker,
{
    type Renderables = render::Opacity<V::Renderables>;
    type Transition = V::Transition;
}

impl<Captures: ?Sized, V> ViewLayout<Captures> for Opacity<V>
where
    V: ViewLayout<Captures>,
{
    type Sublayout = V::Sublayout;
    type State = V::State;

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
        render::Opacity::new(
            self.inner.render_tree(layout, origin, env, captures, state),
            self.opacity,
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
