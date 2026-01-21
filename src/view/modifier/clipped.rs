use embedded_touch::Phase;

use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventResult},
    focus::{FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions, geometry::Rectangle},
    render,
    view::{ViewLayout, ViewMarker},
};

/// A modifier that clips to the bounds of its child.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clipped<T> {
    child: T,
}

impl<T: ViewMarker> Clipped<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(child: T) -> Self {
        Self { child }
    }
}

impl<T> ViewMarker for Clipped<T>
where
    T: ViewMarker,
{
    type Renderables = render::Clipped<T::Renderables>;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for Clipped<T>
where
    T: ViewLayout<Captures>,
{
    type Sublayout = ResolvedLayout<T::Sublayout>;
    type State = T::State;
    type FocusTree = T::FocusTree;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.child.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.child.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env, captures, state).nested()
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
            .child
            .render_tree(&layout.sublayouts, origin, env, captures, state);
        let rect = Rectangle::new(origin, layout.resolved_size.into());
        render::Clipped::new(inner, rect)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        // only cull inner handling of start touches. Drag/end may move outside the clip rect
        // but they should sill be tracked
        let should_handle = match event {
            Event::Touch(touch) => {
                touch.phase != Phase::Started
                    || render_tree.clip_rect.contains(&From::from(touch.location))
            }
            Event::Scroll(point) => render_tree.clip_rect.contains(point),
            Event::External | Event::Exit => true,
        };
        if should_handle {
            self.child
                .handle_event(event, context, &mut render_tree.subtree, captures, state)
        } else {
            EventResult::default()
        }
    }

    fn focus(
        &self,
        event: &FocusEvent,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        self.child.focus(
            event,
            context,
            &mut render_tree.subtree,
            captures,
            state,
            focus,
        )
    }
}
