use core::marker::PhantomData;

use crate::{
    event::{Event, EventResult},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug)]
pub struct MapEvent<V, F, S> {
    inner: V,
    mapping: F,
    _state: PhantomData<S>,
}

impl<V: ViewMarker, F: Fn(&Event, &mut S) -> Option<Event>, S: Default> MapEvent<V, F, S> {
    #[must_use]
    pub const fn new(inner: V, mapping: F) -> Self {
        Self {
            inner,
            mapping,
            _state: PhantomData,
        }
    }
}

impl<V: ViewMarker, F: Fn(&Event, &mut S) -> Option<Event>, S> ViewMarker for MapEvent<V, F, S> {
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<C: ?Sized, V: ViewLayout<C>, F: Fn(&Event, &mut S) -> Option<Event>, S: 'static + Default>
    ViewLayout<C> for MapEvent<V, F, S>
{
    type State = (S, V::State);

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
        (S::default(), self.inner.build_state(captures))
    }

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> crate::layout::ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, captures, &mut state.1)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut C,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.inner
            .render_tree(layout, origin, env, captures, &mut state.1)
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
        let mapped_event = (self.mapping)(event, &mut state.0);
        if let Some(mapped_event) = mapped_event {
            self.inner.handle_event(
                &mapped_event,
                context,
                render_tree,
                captures,
                &mut state.1,
                focus,
            )
        } else {
            EventResult::Deferred
        }
    }
}
