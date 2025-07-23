use crate::{
    animation::Animation,
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::Animate,
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct Animated<InnerView, Value> {
    inner: InnerView,
    animation: Animation,
    value: Value,
}

impl<InnerView, Value: PartialEq> Animated<InnerView, Value> {
    pub const fn new(inner: InnerView, animation: Animation, value: Value) -> Self {
        Self {
            inner,
            animation,
            value,
        }
    }
}

impl<InnerView: ViewMarker, U> ViewMarker for Animated<InnerView, U> {
    type Renderables = Animate<InnerView::Renderables, U>;
}

impl<Captures: ?Sized, InnerView, U: PartialEq + Clone> ViewLayout<Captures>
    for Animated<InnerView, U>
where
    InnerView: ViewLayout<Captures>,
{
    type State = (U, InnerView::State);
    type Sublayout = InnerView::Sublayout;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        (self.value.clone(), self.inner.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, captures, &mut state.1)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        Animate::new(
            self.inner
                .render_tree(layout, origin, env, captures, &mut state.1),
            self.animation.clone(),
            env.app_time(),
            self.value.clone(),
        )
    }

    fn handle_event(
        &mut self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        self.inner
            .handle_event(event, &mut render_tree.subtree, captures, &mut state.1)
    }
}
