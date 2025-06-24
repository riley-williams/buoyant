use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct EraseCaptures<T> {
    inner: T,
}

impl<T> EraseCaptures<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> ViewMarker for EraseCaptures<T>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
}

impl<T: ViewLayout<()>, Captures: ?Sized> ViewLayout<Captures> for EraseCaptures<T> {
    type State = T::State;
    type Sublayout = T::Sublayout;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {
        self.inner.build_state(&mut ())
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, &mut (), state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.inner.render_tree(layout, origin, env, &mut (), state)
    }

    fn handle_event(
        &mut self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        self.inner.handle_event(event, render_tree, &mut (), state)
    }
}

#[derive(Debug, Clone)]
pub struct Lens<InnerView, CaptureFn> {
    inner: InnerView,
    capture_fn: CaptureFn,
}

impl<InnerView, CaptureFn> Lens<InnerView, CaptureFn> {
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
        &mut self,
        event: &super::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        self.inner
            .handle_event(event, render_tree, (self.capture_fn)(captures), state)
    }
}

