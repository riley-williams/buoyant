use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Interpolate, Point, ProposedDimensions},
    render::ShadeSubtree,
    view::{ViewLayout, ViewMarker},
};

/// Sets a foreground style
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundStyle<V, S> {
    inner: V,
    style: S,
}

impl<V, S> ForegroundStyle<V, S> {
    pub const fn new(style: S, inner: V) -> Self {
        Self { inner, style }
    }
}

impl<Inner, Color: Clone> ViewMarker for ForegroundStyle<Inner, Color>
where
    Inner: ViewMarker,
{
    type Renderables = ShadeSubtree<Color, Inner::Renderables>;
}

impl<Color: Interpolate, Captures: ?Sized, Inner: ViewLayout<Captures>> ViewLayout<Captures>
    for ForegroundStyle<Inner, Color>
{
    type Sublayout = Inner::Sublayout;
    type State = Inner::State;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
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
        ShadeSubtree::new(
            self.style,
            self.inner.render_tree(layout, origin, env, captures, state),
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
