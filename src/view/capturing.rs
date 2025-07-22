use core::time::Duration;

use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Converts the captures of a parent view to a subview's captures.
#[derive(Debug, Clone)]
pub struct Lens<InnerView, CaptureFn> {
    inner: InnerView,
    capture_fn: CaptureFn,
}

impl<InnerView, CaptureFn> Lens<InnerView, CaptureFn> {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new<OuterCapture, InnerCapture>(inner: InnerView, capture_fn: CaptureFn) -> Self
    where
        InnerView: ViewLayout<InnerCapture>,
        CaptureFn: Fn(&mut OuterCapture) -> &mut InnerCapture,
    {
        Self { inner, capture_fn }
    }
}

impl<InnerView, F> ViewMarker for Lens<InnerView, F>
where
    InnerView: ViewMarker,
{
    type Renderables = InnerView::Renderables;
}

impl<
        InnerView: ViewLayout<InnerCaptures>,
        Captures: ?Sized,
        F: for<'a> Fn(&'a mut Captures) -> &'a mut InnerCaptures,
        InnerCaptures,
    > ViewLayout<Captures> for Lens<InnerView, F>
{
    type State = InnerView::State;
    type Sublayout = InnerView::Sublayout;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let inner_captures = (self.capture_fn)(captures);
        self.inner.build_state(inner_captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner
            .layout(offer, env, (self.capture_fn)(captures), state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.inner
            .render_tree(layout, origin, env, (self.capture_fn)(captures), state)
    }

    fn handle_event(
        &self,
        event: &super::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        app_time: Duration,
    ) -> EventResult {
        self.inner.handle_event(
            event,
            render_tree,
            (self.capture_fn)(captures),
            state,
            app_time,
        )
    }
}
