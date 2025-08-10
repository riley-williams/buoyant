use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::{shape::Rect, ShadeSubtree},
    view::{ViewLayout, ViewMarker},
};

/// A view that uses the layout of the foreground view, rendering a background in
/// the specified color.
///
/// A specialized version of [`crate::view::modifier::ViewModifier::background()`]
/// that also sets the hinting color for the background.
#[derive(Debug, Clone)]
pub struct BackgroundColor<T, C> {
    foreground: T,
    background_color: C,
}

impl<T, C> BackgroundColor<T, C> {
    pub const fn new(foreground: T, background_color: C) -> Self {
        Self {
            foreground,
            background_color,
        }
    }
}

impl<T, C> ViewMarker for BackgroundColor<T, C>
where
    T: ViewMarker,
    C: Clone,
{
    type Renderables = (ShadeSubtree<Rect, C>, T::Renderables);
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, C> ViewLayout<Captures> for BackgroundColor<T, C>
where
    T: ViewLayout<Captures>,
    C: Clone,
{
    type Sublayout = ResolvedLayout<T::Sublayout>;
    type State = T::State;

    fn priority(&self) -> i8 {
        self.foreground.priority()
    }

    fn is_empty(&self) -> bool {
        self.foreground.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.foreground.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.foreground.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let foreground_layout = self.foreground.layout(offer, env, captures, state);
        let foreground_size = foreground_layout.resolved_size;

        ResolvedLayout {
            sublayouts: foreground_layout,
            resolved_size: foreground_size,
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        (
            ShadeSubtree::new(
                Rect::new(origin, layout.resolved_size.into()),
                self.background_color.clone(),
            ),
            self.foreground
                .render_tree(&layout.sublayouts, origin, env, captures, state),
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
        // Foreground handles events first
        self.foreground
            .handle_event(event, context, &mut render_tree.1, captures, state)
    }
}
