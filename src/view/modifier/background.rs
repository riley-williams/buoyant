use core::time::Duration;

use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::{Alignment, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A view that uses the layout of the foreground view, but renders the background
/// behind it.
#[derive(Debug, Clone)]
pub struct BackgroundView<T, U> {
    foreground: T,
    background: U,
    alignment: Alignment,
}

impl<T, U> BackgroundView<T, U> {
    pub const fn new(foreground: T, background: U, alignment: Alignment) -> Self {
        Self {
            foreground,
            background,
            alignment,
        }
    }
}

impl<T, U> ViewMarker for BackgroundView<T, U>
where
    T: ViewMarker,
    U: ViewMarker,
{
    type Renderables = (U::Renderables, T::Renderables);
}

impl<Captures: ?Sized, T, U> ViewLayout<Captures> for BackgroundView<T, U>
where
    T: ViewLayout<Captures>,
    U: ViewLayout<Captures>,
{
    type Sublayout = (ResolvedLayout<T::Sublayout>, ResolvedLayout<U::Sublayout>);
    // Tuples are rendered first to last
    type State = (T::State, U::State);

    fn priority(&self) -> i8 {
        self.foreground.priority()
    }

    fn is_empty(&self) -> bool {
        self.foreground.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        (
            self.foreground.build_state(captures),
            self.background.build_state(captures),
        )
    }
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let foreground_layout = self.foreground.layout(offer, env, captures, &mut state.0);
        let foreground_size = foreground_layout.resolved_size;
        // Propose the foreground size to the background
        // This would benefit from splitting layout into separate functions for the various offers
        let background_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let background_layout =
            self.background
                .layout(&background_offer, env, captures, &mut state.1);

        ResolvedLayout {
            sublayouts: (foreground_layout, background_layout),
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
        let new_origin = origin
            + Point::new(
                self.alignment.horizontal().align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.1.resolved_size.width.into(),
                ),
                self.alignment.vertical().align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.1.resolved_size.height.into(),
                ),
            );

        (
            self.background.render_tree(
                &layout.sublayouts.1,
                new_origin,
                env,
                captures,
                &mut state.1,
            ),
            self.foreground
                .render_tree(&layout.sublayouts.0, origin, env, captures, &mut state.0),
        )
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        app_time: Duration,
    ) -> EventResult {
        // Foreground handles events first
        let foreground_result = self.foreground.handle_event(
            event,
            &mut render_tree.1,
            captures,
            &mut state.0,
            app_time,
        );
        if foreground_result.handled {
            return foreground_result;
        }
        self.background
            .handle_event(event, &mut render_tree.0, captures, &mut state.1, app_time)
            .merging(foreground_result)
    }
}
