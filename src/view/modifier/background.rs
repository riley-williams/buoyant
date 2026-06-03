use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventResult},
    focus::{DefaultFocus, FocusAction, FocusDirection},
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

/// Focus tree for background view - tracks which child has focus
#[derive(Debug, Clone)]
pub struct BackgroundFocus<T, U> {
    active_foreground: bool,
    foreground: T,
    background: U,
}

impl<T: DefaultFocus, U: DefaultFocus> DefaultFocus for BackgroundFocus<T, U> {
    fn default_first() -> Self {
        Self {
            active_foreground: false,
            foreground: T::default_first(),
            background: U::default_first(),
        }
    }

    fn default_last() -> Self {
        Self {
            active_foreground: true,
            foreground: T::default_last(),
            background: U::default_last(),
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
    type FocusTree = BackgroundFocus<T::FocusTree, U::FocusTree>;

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
        self.foreground
            .layout(offer, env, captures, &mut state.0)
            .nested()
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

    #[expect(clippy::too_many_lines)]
    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        // Handle focus events specially - they need to route through the focus tree
        if let Event::Focus {
            action: focus_event,
            group,
        } = event
        {
            if focus.active_foreground {
                let result = self.foreground.handle_event(
                    event,
                    context,
                    &mut render_tree.1,
                    captures,
                    &mut state.0,
                    &mut focus.foreground,
                );

                if result.is_handled() || *focus_event == FocusAction::Teardown {
                    return result;
                }

                // Foreground exhausted - only move to background on backward navigation
                match focus_event {
                    FocusAction::Focus(FocusDirection::Backward) | FocusAction::Previous => {
                        let mut background_focus = DefaultFocus::default_last();
                        let result = self.background.handle_event(
                            &Event::Focus {
                                action: FocusAction::Focus(FocusDirection::Backward),
                                group: *group,
                            },
                            context,
                            &mut render_tree.0,
                            captures,
                            &mut state.1,
                            &mut background_focus,
                        );
                        focus.active_foreground = false;
                        focus.background = background_focus;
                        result
                    }
                    FocusAction::Focus(FocusDirection::Forward)
                    | FocusAction::Next
                    | FocusAction::Select
                    | FocusAction::Blur
                    | FocusAction::Teardown => result,
                }
            } else {
                let result = self.background.handle_event(
                    event,
                    context,
                    &mut render_tree.0,
                    captures,
                    &mut state.1,
                    &mut focus.background,
                );

                if result.is_handled() || *focus_event == FocusAction::Teardown {
                    return result;
                }

                // Background exhausted - only move to foreground on forward navigation
                match focus_event {
                    FocusAction::Focus(FocusDirection::Forward) | FocusAction::Next => {
                        // Move to foreground
                        let mut foreground_focus = DefaultFocus::default_first();
                        let result = self.foreground.handle_event(
                            &Event::Focus {
                                action: FocusAction::Focus(FocusDirection::Forward),
                                group: *group,
                            },
                            context,
                            &mut render_tree.1,
                            captures,
                            &mut state.0,
                            &mut foreground_focus,
                        );
                        focus.active_foreground = true;
                        focus.foreground = foreground_focus;
                        result
                    }
                    FocusAction::Select |
                    FocusAction::Focus(FocusDirection::Backward)
                    | FocusAction::Previous
                    | FocusAction::Blur
                    | FocusAction::Teardown => EventResult::deferred(),
                }
            }
        } else {
            // For non-focus events (touch, scroll, etc.), perform DFS back to front
            // The same in main, probably clippy doesn't have a lint for this for `match`.
            #[expect(clippy::if_same_then_else)]
            if focus.active_foreground {
                // Start with background (back)
                let background_result = self.background.handle_event(
                    event,
                    context,
                    &mut render_tree.0,
                    captures,
                    &mut state.1,
                    &mut focus.background,
                );
                if background_result.is_handled() {
                    return background_result;
                }
                // Then foreground (front)
                self.foreground.handle_event(
                    event,
                    context,
                    &mut render_tree.1,
                    captures,
                    &mut state.0,
                    &mut focus.foreground,
                )
            } else {
                // Start with background (back)
                let background_result = self.background.handle_event(
                    event,
                    context,
                    &mut render_tree.0,
                    captures,
                    &mut state.1,
                    &mut focus.background,
                );
                if background_result.is_handled() {
                    return background_result;
                }
                // Then foreground (front)
                self.foreground.handle_event(
                    event,
                    context,
                    &mut render_tree.1,
                    captures,
                    &mut state.0,
                    &mut focus.foreground,
                )
            }
        }
    }
}
