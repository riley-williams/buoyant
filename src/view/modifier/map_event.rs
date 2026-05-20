use core::marker::PhantomData;

use crate::{
    event::{Event, EventResult},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug)]
pub struct MapEvent<V, F, C: ?Sized> {
    inner: V,
    mapping: F,
    _state: PhantomData<C>,
}

impl<V: ViewMarker, F: Fn(Event, &mut C) -> Option<Event>, C: ?Sized> MapEvent<V, F, C> {
    #[must_use]
    pub const fn new(inner: V, mapping: F) -> Self {
        Self {
            inner,
            mapping,
            _state: PhantomData,
        }
    }
}

impl<V: ViewMarker, F: Fn(Event, &mut C) -> Option<Event>, C: ?Sized> ViewMarker
    for MapEvent<V, F, C>
{
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<C: ?Sized, V: ViewLayout<C>, F: Fn(Event, &mut C) -> Option<Event>> ViewLayout<C>
    for MapEvent<V, F, C>
{
    type State = V::State;

    type Sublayout = V::Sublayout;

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

    fn build_state(&self, captures: &mut C) -> Self::State {
        self.inner.build_state(captures)
    }

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> crate::layout::ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.inner.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut C,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> crate::event::EventResult {
        let mapped_event = (self.mapping)(event.clone(), captures);
        mapped_event.map_or_else(EventResult::deferred, |mapped_event| {
            self.inner
                .handle_event(&mapped_event, context, render_tree, captures, state, focus)
        })
    }
}
