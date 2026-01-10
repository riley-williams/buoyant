use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::Offset,
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct GeometryGroup<InnerView> {
    inner: InnerView,
}

impl<InnerView: ViewMarker> GeometryGroup<InnerView> {
    pub const fn new(view: InnerView) -> Self {
        Self { inner: view }
    }
}

impl<InnerView> ViewMarker for GeometryGroup<InnerView>
where
    InnerView: ViewMarker,
{
    type Renderables = Offset<InnerView::Renderables>;
    type Transition = InnerView::Transition;
}

impl<Captures: ?Sized, InnerView> ViewLayout<Captures> for GeometryGroup<InnerView>
where
    InnerView: ViewLayout<Captures>,
{
    type Sublayout = InnerView::Sublayout;
    type State = InnerView::State;

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
        // Store the offset, and render subtrees from zero
        Offset::new(
            origin,
            self.inner
                .render_tree(layout, Point::zero(), env, captures, state),
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
        let event = event.offset(-render_tree.offset);
        self.inner
            .handle_event(&event, context, &mut render_tree.subtree, captures, state)
    }
}
