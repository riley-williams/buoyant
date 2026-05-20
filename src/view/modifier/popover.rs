use core::time::Duration;

use crate::{
    animation::Animation,
    event::{Event, EventContext, EventResult},
    focus::{BoundaryBehavior, DefaultFocus, FocusAction, FocusDirection},
    layout::{HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::Point,
    render::{Animate, TransitionOption},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct Popover<Inner, Overlay> {
    inner: Inner,
    overlay: Option<Overlay>,
    boundary_behavior: BoundaryBehavior,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusTree<T, U> {
    /// The inner view's focus state (always preserved)
    pub inner: T,
    /// The overlay's focus state (when overlay is active)
    pub overlay: Option<U>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PopoverState<T, U> {
    pub inner_state: T,
    pub overlay_state: Option<U>,
}

impl<T: DefaultFocus, U: DefaultFocus> DefaultFocus for FocusTree<T, U> {
    fn default_first() -> Self {
        Self {
            inner: DefaultFocus::default_first(),
            overlay: None,
        }
    }

    fn default_last() -> Self {
        Self {
            inner: DefaultFocus::default_last(),
            overlay: None,
        }
    }
}

impl<Inner, Overlay> Popover<Inner, Overlay>
where
    Inner: ViewMarker,
    Overlay: ViewMarker,
{
    #[must_use]
    pub fn new<T, ViewFn>(inner: Inner, value: Option<&T>, view_fn: ViewFn) -> Self
    where
        ViewFn: for<'b> FnOnce(&'b T) -> Overlay,
        T: Clone,
    {
        let overlay = value.map(view_fn);
        Self {
            inner,
            overlay,
            boundary_behavior: BoundaryBehavior::Stop,
        }
    }

    /// Modifies the behavior when attempting to move focus beyond the ends of the overlay.
    #[must_use]
    pub fn with_boundary_behavior(mut self, behavior: BoundaryBehavior) -> Self {
        self.boundary_behavior = behavior;
        self
    }
}

impl<Inner: ViewMarker, Overlay: ViewMarker> ViewMarker for Popover<Inner, Overlay> {
    type Renderables = (
        Inner::Renderables,
        Animate<TransitionOption<Overlay::Renderables, Overlay::Transition>, bool>,
    );

    type Transition = Inner::Transition;
}

impl<Captures, Inner, Overlay> ViewLayout<Captures> for Popover<Inner, Overlay>
where
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    Overlay: ViewLayout<Captures>,
{
    type State = PopoverState<Inner::State, Overlay::State>;
    type Sublayout = ResolvedLayout<Inner::Sublayout>;
    type FocusTree = FocusTree<Inner::FocusTree, Overlay::FocusTree>;

    fn transition(&self) -> Self::Transition {
        self.inner.transition()
    }

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        Self::State {
            inner_state: self.inner.build_state(captures),
            overlay_state: self.overlay.as_ref().map(|o| o.build_state(captures)),
        }
    }

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner
            .layout(offer, env, captures, &mut state.inner_state)
            .nested()
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let inner_tree = self.inner.render_tree(
            &layout.sublayouts,
            origin,
            env,
            captures,
            &mut state.inner_state,
        );
        let overlay_tree = if let Some(overlay_view) = &self.overlay {
            let overlay_state = state
                .overlay_state
                .get_or_insert_with(|| overlay_view.build_state(captures));
            let overlay_layout =
                overlay_view.layout(&layout.resolved_size.into(), env, captures, overlay_state);
            let origin = origin
                + Point::new(
                    HorizontalAlignment::Center.align(
                        layout.resolved_size.width.into(),
                        overlay_layout.resolved_size.width.into(),
                    ),
                    VerticalAlignment::Center.align(
                        layout.resolved_size.height.into(),
                        overlay_layout.resolved_size.height.into(),
                    ),
                );
            TransitionOption::new_some(
                overlay_view.render_tree(
                    &overlay_layout.sublayouts,
                    origin,
                    env,
                    captures,
                    overlay_state,
                ),
                layout.resolved_size.into(),
                overlay_view.transition(),
            )
        } else {
            state.overlay_state = None;
            TransitionOption::None
        };
        (
            inner_tree,
            Animate::new(
                overlay_tree,
                Animation::ease_out(Duration::from_millis(300)),
                env.app_time(),
                self.overlay.is_some(),
            ),
        )
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
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
            if let Some(overlay_view) = &self.overlay {
                // Overlay is active - ensure we have overlay focus state

                // FIXME: We don't know when the overlay appears, so we don't get a chance to obtain initial focus
                // Likely requires rethinking focus overall

                let subfocus = focus
                    .overlay
                    .get_or_insert_with(DefaultFocus::default_first);
                let overlay_state = state
                    .overlay_state
                    .get_or_insert_with(|| overlay_view.build_state(captures));

                if let TransitionOption::Some { subtree, .. } = &mut render_tree.1.subtree {
                    let result = overlay_view.handle_event(
                        &Event::Focus {
                            action: *focus_event,
                            group: *group,
                        },
                        context,
                        subtree,
                        captures,
                        overlay_state,
                        subfocus,
                    );

                    if matches!(result, EventResult::Deferred { .. }) {
                        // Determine if we were moving forward or backward
                        let is_forward = matches!(
                            focus_event,
                            FocusAction::Next | FocusAction::Focus(FocusDirection::Forward)
                        );

                        // Wrap to the opposite end based on direction
                        // Reset focus tree to the appropriate end
                        *subfocus = if is_forward {
                            DefaultFocus::default_first()
                        } else {
                            DefaultFocus::default_last()
                        };
                        // Acquire focus at the wrapped position (don't navigate again)
                        let acquire_direction = if is_forward {
                            FocusDirection::Forward
                        } else {
                            FocusDirection::Backward
                        };
                        return overlay_view.handle_event(
                            &Event::Focus {
                                action: FocusAction::Focus(acquire_direction),
                                group: *group,
                            },
                            context,
                            subtree,
                            captures,
                            overlay_state,
                            subfocus,
                        );
                    }

                    return result;
                }
                // TODO: Popover is visible, but we don't have a render tree for it yet
                // Just dump all these events?
                return EventResult::deferred();
            }
            // Overlay is not active - clear overlay focus and use inner focus
            focus.overlay = None;
            state.overlay_state = None;

            return self.inner.handle_event(
                &Event::Focus {
                    action: *focus_event,
                    group: *group,
                },
                context,
                &mut render_tree.0,
                captures,
                &mut state.inner_state,
                &mut focus.inner,
            );
        }

        match (&self.overlay, &mut render_tree.1.subtree) {
            (Some(overlay_view), TransitionOption::Some { subtree, .. }) => {
                let overlay_state = state
                    .overlay_state
                    .get_or_insert_with(|| overlay_view.build_state(captures));
                overlay_view.handle_event(
                    event,
                    context,
                    subtree,
                    captures,
                    overlay_state,
                    focus.overlay.get_or_insert(DefaultFocus::default_first()),
                )
            }
            (Some(_), TransitionOption::None) => {
                // Overlay is present but not rendered yet - defer events until it actually appears
                EventResult::deferred()
            }
            _ => self.inner.handle_event(
                event,
                context,
                &mut render_tree.0,
                captures,
                &mut state.inner_state,
                &mut focus.inner,
            ),
        }
    }
}
