use core::marker::PhantomData;

use crate::{
    event::{Event, EventResult},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct MapEvent<V, F, C: ?Sized, I> {
    inner: V,
    mapping: F,
    _state: PhantomData<I>,
    _captures: PhantomData<C>,
}

#[derive(Debug, Clone)]
pub enum Mapping {
    /// Pass through the event unmodified
    Passthrough,
    /// Replace the original event with the provided one
    Replace(Event),
    /// Use the mapped event only if the modified view is unable to handle the original event
    Fallback(Event),
    /// Defer the event without passing it through to the modified view.
    ///
    /// Prefer this over [`Mapping::Handled`] for event types which are unmapped/unhandled
    /// but should not be passed to the modified view.
    Defer,
    /// Report the event as handled without passing through to the modified view.
    ///
    /// Returning this may prevent focus from moving or taps from being handled by subsequent views.
    /// [`Mapping::Defer`] is typically the desired choice.
    Handled,
}

impl<C: ?Sized, V: ViewLayout<C>, F: Fn(&Event, &mut C, &mut I) -> Mapping, I: Default>
    MapEvent<V, F, C, I>
{
    #[must_use]
    pub const fn new(inner: V, mapping: F) -> Self {
        Self {
            inner,
            mapping,
            _state: PhantomData,
            _captures: PhantomData,
        }
    }
}

impl<V: ViewMarker, F, C: ?Sized, I> ViewMarker for MapEvent<V, F, C, I> {
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<C: ?Sized, V: ViewLayout<C>, F: Fn(&Event, &mut C, &mut I) -> Mapping, I: 'static + Default>
    ViewLayout<C> for MapEvent<V, F, C, I>
{
    type State = (I, V::State);

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
        (I::default(), self.inner.build_state(captures))
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
        let mapped_event = (self.mapping)(event, captures, &mut state.0);
        match mapped_event {
            Mapping::Passthrough => {
                self.inner
                    .handle_event(event, context, render_tree, captures, &mut state.1, focus)
            }
            Mapping::Replace(mapped_event) => self.inner.handle_event(
                &mapped_event,
                context,
                render_tree,
                captures,
                &mut state.1,
                focus,
            ),
            Mapping::Fallback(mapped_event) => {
                let result = self.inner.handle_event(
                    event,
                    context,
                    render_tree,
                    captures,
                    &mut state.1,
                    focus,
                );
                if result.is_handled() {
                    result
                } else {
                    self.inner.handle_event(
                        &mapped_event,
                        context,
                        render_tree,
                        captures,
                        &mut state.1,
                        focus,
                    )
                }
            }
            Mapping::Defer => EventResult::deferred(),
            Mapping::Handled => EventResult::handled_unfocused(),
        }
    }
}
