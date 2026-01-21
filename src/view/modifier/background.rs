use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    focus::{FocusEvent, FocusStateChange},
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

impl<T: ViewMarker, U: ViewMarker> BackgroundView<T, U> {
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
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, U> ViewLayout<Captures> for BackgroundView<T, U>
where
    T: ViewLayout<Captures>,
    U: ViewLayout<Captures>,
{
    type Sublayout = ResolvedLayout<T::Sublayout>;
    // Tuples are rendered first to last
    type State = (T::State, U::State);
    type FocusTree = T::FocusTree;

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
        ResolvedLayout {
            sublayouts: foreground_layout,
            resolved_size: foreground_size,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let foreground_size = layout.resolved_size;
        // Propose the foreground size to the background
        let background_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let background_layout =
            self.background
                .layout(&background_offer, env, captures, &mut state.1);

        let new_origin = origin
            + Point::new(
                self.alignment.horizontal().align(
                    layout.resolved_size.width.into(),
                    background_layout.resolved_size.width.into(),
                ),
                self.alignment.vertical().align(
                    layout.resolved_size.height.into(),
                    background_layout.resolved_size.height.into(),
                ),
            );

        (
            self.background.render_tree(
                &background_layout.sublayouts,
                new_origin,
                env,
                captures,
                &mut state.1,
            ),
            self.foreground
                .render_tree(&layout.sublayouts, origin, env, captures, &mut state.0),
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
        let foreground_result = self.foreground.handle_event(
            event,
            context,
            &mut render_tree.1,
            captures,
            &mut state.0,
        );
        if foreground_result.handled {
            return foreground_result;
        }
        self.background
            .handle_event(event, context, &mut render_tree.0, captures, &mut state.1)
            .merging(foreground_result)
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
        // Foreground handles focus first
        let foreground_result = self.foreground.focus(
            event,
            context,
            &mut render_tree.1,
            captures,
            &mut state.0,
            focus,
        );
        if foreground_result != FocusStateChange::Exhausted {
            return foreground_result;
        }
        foreground_result
    }
}
