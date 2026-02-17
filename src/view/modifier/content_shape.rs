use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::{self, shape::AsShapePrimitive},
    view::{ViewLayout, ViewMarker, shape::Shape},
};

/// A modifier that overrides the content shape of its child view.
#[derive(Debug, Clone)]
pub struct ContentShape<V, S> {
    inner: V,
    shape: S,
}

impl<V: ViewMarker, S: Shape> ContentShape<V, S> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(inner: V, shape: S) -> Self {
        Self { inner, shape }
    }
}

impl<V, S> ViewMarker for ContentShape<V, S>
where
    V: ViewMarker,
    S: Shape,
{
    type Renderables = render::ContentShapeOverride<V::Renderables>;
    type Transition = V::Transition;
}

impl<Captures, V, S> ViewLayout<Captures> for ContentShape<V, S>
where
    V: ViewLayout<Captures>,
    Captures: ?Sized,
    S: Shape + ViewLayout<()>,
{
    type Sublayout = ResolvedLayout<V::Sublayout>;
    type State = V::State;
    type FocusTree = V::FocusTree;

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
        self.inner.layout(offer, env, captures, state).nested()
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let inner = self
            .inner
            .render_tree(&layout.sublayouts, origin, env, captures, state);
        // There's probably a more efficient way to do this, but it's simple and
        // trivially correct for now.
        // Hopefully the compiler deletes most of it.
        let shape = self
            .shape
            .layout(&layout.resolved_size.into(), env, &mut (), &mut ());
        let render_tree = self
            .shape
            .render_tree(&shape.sublayouts, origin, env, &mut (), &mut ());
        let content_shape = render_tree.as_shape().into();
        render::ContentShapeOverride::new(inner, content_shape)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        self.inner.handle_event(
            event,
            context,
            &mut render_tree.subtree,
            captures,
            state,
            focus,
        )
    }
}
