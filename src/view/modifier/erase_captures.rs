use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// Converts the captures of a parent view to [`()`]
///
/// When making generic views that do not rely on any captures, it can be more convenient
/// to make the capture generic instead:
///
/// ```
/// # use buoyant::view::prelude::*;
/// # use embedded_graphics::pixelcolor::Rgb888;
/// fn component_view<C: ?Sized>() -> impl View<Rgb888, C> {
///     Rectangle
/// }
/// ```
#[derive(Debug, Clone)]
pub struct EraseCaptures<T> {
    inner: T,
}

impl<T> EraseCaptures<T> {
    #[allow(missing_docs)]
    #[must_use]
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
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.inner
            .handle_event(event, context, render_tree, &mut (), state)
    }
}
