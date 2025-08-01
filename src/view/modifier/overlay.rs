use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::{Alignment, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A view that uses the layout of the modified view, rendering the overlay
/// on top of it.
#[derive(Debug, Clone)]
pub struct OverlayView<T, U> {
    foreground: T,
    overlay: U,
    alignment: Alignment,
}

impl<T, U> OverlayView<T, U> {
    pub const fn new(foreground: T, overlay: U, alignment: Alignment) -> Self {
        Self {
            foreground,
            overlay,
            alignment,
        }
    }
}

impl<T, U> ViewMarker for OverlayView<T, U>
where
    T: ViewMarker,
    U: ViewMarker,
{
    type Renderables = (T::Renderables, U::Renderables);
}

impl<Captures: ?Sized, T, U> ViewLayout<Captures> for OverlayView<T, U>
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
            self.overlay.build_state(captures),
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
        // Propose the foreground size to the overlay
        // This would benefit from splitting layout into separate functions for the various offers
        let overlay_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let overlay_layout = self
            .overlay
            .layout(&overlay_offer, env, captures, &mut state.1);

        ResolvedLayout {
            sublayouts: (foreground_layout, overlay_layout),
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
            self.foreground
                .render_tree(&layout.sublayouts.0, origin, env, captures, &mut state.0),
            self.overlay.render_tree(
                &layout.sublayouts.1,
                new_origin,
                env,
                captures,
                &mut state.1,
            ),
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
        // Overlay handles events first (it's on top)
        let overlay_result =
            self.overlay
                .handle_event(event, context, &mut render_tree.1, captures, &mut state.1);
        if overlay_result.handled {
            return overlay_result;
        }
        self.foreground
            .handle_event(event, context, &mut render_tree.0, captures, &mut state.0)
            .merging(overlay_result)
    }
}
