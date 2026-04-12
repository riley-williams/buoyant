use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventResult},
    focus::{DefaultFocus, FocusAction, FocusDirection},
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

impl<T: ViewMarker, U: ViewMarker> OverlayView<T, U> {
    pub const fn new(foreground: T, overlay: U, alignment: Alignment) -> Self {
        Self {
            foreground,
            overlay,
            alignment,
        }
    }
}

/// Focus tree for overlay view - tracks which child has focus
#[derive(Debug, Clone)]
pub enum OverlayFocus<T, U> {
    /// Focus is on the foreground
    Foreground(T),
    /// Focus is on the overlay
    Overlay(U),
}

impl<T: DefaultFocus, U: DefaultFocus> DefaultFocus for OverlayFocus<T, U> {
    fn default_first() -> Self {
        Self::Overlay(U::default_first())
    }

    fn default_last() -> Self {
        Self::Foreground(T::default_last())
    }
}

impl<T, U> ViewMarker for OverlayView<T, U>
where
    T: ViewMarker,
    U: ViewMarker,
{
    type Renderables = (T::Renderables, U::Renderables);
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T, U> ViewLayout<Captures> for OverlayView<T, U>
where
    T: ViewLayout<Captures>,
    U: ViewLayout<Captures>,
{
    type Sublayout = ResolvedLayout<T::Sublayout>;
    // Tuples are rendered first to last
    type State = (T::State, U::State);
    type FocusTree = OverlayFocus<T::FocusTree, U::FocusTree>;

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
        // Propose the foreground size to the overlay
        // This would benefit from splitting layout into separate functions for the various offers
        let overlay_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let overlay_layout = self
            .overlay
            .layout(&overlay_offer, env, captures, &mut state.1);

        let new_origin = origin
            + Point::new(
                self.alignment.horizontal().align(
                    layout.resolved_size.width.into(),
                    overlay_layout.resolved_size.width.into(),
                ),
                self.alignment.vertical().align(
                    layout.resolved_size.height.into(),
                    overlay_layout.resolved_size.height.into(),
                ),
            );

        (
            self.foreground
                .render_tree(&layout.sublayouts, origin, env, captures, &mut state.0),
            self.overlay.render_tree(
                &overlay_layout.sublayouts,
                new_origin,
                env,
                captures,
                &mut state.1,
            ),
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
            match focus {
                OverlayFocus::Overlay(overlay_focus) => {
                    let result = self.overlay.handle_event(
                        event,
                        context,
                        &mut render_tree.1,
                        captures,
                        &mut state.1,
                        overlay_focus,
                    );

                    if result.is_handled() || *focus_event == FocusAction::Teardown {
                        return result;
                    }

                    // Overlay exhausted - only move to foreground on forward navigation
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
                                &mut render_tree.0,
                                captures,
                                &mut state.0,
                                &mut foreground_focus,
                            );
                            *focus = OverlayFocus::Foreground(foreground_focus);
                            result
                        }
                        FocusAction::Focus(FocusDirection::Backward)
                        | FocusAction::Previous
                        | FocusAction::Blur
                        | FocusAction::Select
                        | FocusAction::Teardown => EventResult::Deferred,
                    }
                }
                OverlayFocus::Foreground(foreground_focus) => {
                    let result = self.foreground.handle_event(
                        event,
                        context,
                        &mut render_tree.0,
                        captures,
                        &mut state.0,
                        foreground_focus,
                    );

                    if result.is_handled() || *focus_event == FocusAction::Teardown {
                        return result;
                    }

                    // Foreground exhausted - only move to overlay on backward navigation
                    match focus_event {
                        FocusAction::Focus(FocusDirection::Backward) | FocusAction::Previous => {
                            let mut overlay_focus = DefaultFocus::default_last();
                            let result = self.overlay.handle_event(
                                &Event::Focus {
                                    action: FocusAction::Focus(FocusDirection::Backward),
                                    group: *group,
                                },
                                context,
                                &mut render_tree.1,
                                captures,
                                &mut state.1,
                                &mut overlay_focus,
                            );
                            *focus = OverlayFocus::Overlay(overlay_focus);
                            result
                        }
                        FocusAction::Focus(FocusDirection::Forward)
                        | FocusAction::Next
                        | FocusAction::Select
                        | FocusAction::Blur
                        | FocusAction::Teardown => EventResult::Deferred,
                    }
                }
            }
        } else {
            // For non-focus events (touch, scroll, etc.), perform DFS
            match focus {
                OverlayFocus::Overlay(_) => {
                    // Start with overlay
                    let overlay_result = self.overlay.handle_event(
                        event,
                        context,
                        &mut render_tree.1,
                        captures,
                        &mut state.1,
                        match focus {
                            OverlayFocus::Overlay(focus) => focus,
                            OverlayFocus::Foreground(_) => unreachable!(),
                        },
                    );
                    if overlay_result.is_handled() {
                        return overlay_result;
                    }
                    // Then foreground
                    *focus = OverlayFocus::Foreground(DefaultFocus::default_first());
                    self.foreground.handle_event(
                        event,
                        context,
                        &mut render_tree.0,
                        captures,
                        &mut state.0,
                        match focus {
                            OverlayFocus::Foreground(focus) => focus,
                            OverlayFocus::Overlay(_) => unreachable!(),
                        },
                    )
                }
                OverlayFocus::Foreground(_) => {
                    // Start with overlay
                    *focus = OverlayFocus::Overlay(DefaultFocus::default_first());
                    let overlay_result = self.overlay.handle_event(
                        event,
                        context,
                        &mut render_tree.1,
                        captures,
                        &mut state.1,
                        match focus {
                            OverlayFocus::Overlay(focus) => focus,
                            OverlayFocus::Foreground(_) => unreachable!(),
                        },
                    );
                    if overlay_result.is_handled() {
                        return overlay_result;
                    }
                    // Then foreground
                    *focus = OverlayFocus::Foreground(DefaultFocus::default_first());
                    self.foreground.handle_event(
                        event,
                        context,
                        &mut render_tree.0,
                        captures,
                        &mut state.0,
                        match focus {
                            OverlayFocus::Foreground(focus) => focus,
                            OverlayFocus::Overlay(_) => unreachable!(),
                        },
                    )
                }
            }
        }
    }
}
