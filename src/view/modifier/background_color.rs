use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::{HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimensions},
    render::{HintBackground, ShadeSubtree},
    view::{ViewLayout, ViewMarker, shape::Shape},
};

/// A view that uses the layout of the foreground view, rendering a background shape
/// in the specified color.
///
/// A specialized version of [`ViewModifier::background()`][`crate::view::modifier::ViewModifier::background()`].
#[derive(Debug, Clone)]
pub struct BackgroundColor<T, C, S> {
    foreground: T,
    color: C,
    background_shape: S,
}

impl<T: ViewMarker, C, S: Shape> BackgroundColor<T, C, S> {
    pub const fn new(foreground: T, background_color: C, background_shape: S) -> Self {
        Self {
            foreground,
            color: background_color,
            background_shape,
        }
    }
}

impl<T, C, S: Shape> ViewMarker for BackgroundColor<T, C, S>
where
    T: ViewMarker,
    C: Clone,
{
    type Renderables = (
        ShadeSubtree<C, S::Renderables>,
        HintBackground<T::Renderables, C>,
    );
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, C, S: Shape> ViewLayout<Captures> for BackgroundColor<T, C, S>
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
        let background_layout =
            self.background_shape
                .layout(&layout.resolved_size.into(), env, &mut (), &mut ());
        let background_origin = origin
            + Point::new(
                HorizontalAlignment::Center.align(
                    layout.resolved_size.width.into(),
                    background_layout.resolved_size.width.into(),
                ),
                VerticalAlignment::Center.align(
                    layout.resolved_size.height.into(),
                    background_layout.resolved_size.height.into(),
                ),
            );
        let background_shape = self.background_shape.render_tree(
            &background_layout,
            background_origin,
            env,
            &mut (),
            &mut (),
        );
        let foreground =
            self.foreground
                .render_tree(&layout.sublayouts, origin, env, captures, state);
        (
            ShadeSubtree::new(self.color.clone(), background_shape),
            HintBackground::new(foreground, self.color.clone()),
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
        self.foreground
            .handle_event(event, context, &mut render_tree.1.subtree, captures, state)
    }
}
