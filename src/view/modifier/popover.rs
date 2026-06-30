use core::time::Duration;

use crate::{
    animation::Animation,
    event::{Event, EventContext, EventResult},
    focus::{BoundaryBehavior, FocusAction, FocusDirection, FocusTree},
    layout::{HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::Point,
    render::{Animate, IntrinsicShape, TransitionOption},
    view::{ViewLayout, ViewMarker},
};

/// The decision to either retain or dismiss the active popover on blur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dismissal {
    /// Dismiss the popover, returning focus to the underlying view.
    Dismiss,
    /// Keep the overlay focused;
    Retain,
}

/// A callback invoked when a popover's overlay receives a `Blur` event.
///
/// The handler may mutate captured state (e.g. clearing the value that drives
/// the overlay's visibility) and then returns a [`Dismissal`] to indicate
/// whether the overlay should actually be dismissed.
pub trait OnBlur<Captures: ?Sized> {
    /// Called when the overlay is blurred. Returns the dismissal decision.
    fn on_blur(&self, captures: &mut Captures) -> Dismissal;
}

impl<C: ?Sized, F> OnBlur<C> for F
where
    F: Fn(&mut C) -> Dismissal,
{
    fn on_blur(&self, captures: &mut C) -> Dismissal {
        (self)(captures)
    }
}

/// An [`OnBlur`] implementation that does not dismiss on blur. This is the
/// default behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoDismiss;

impl<C: ?Sized> OnBlur<C> for NoDismiss {
    fn on_blur(&self, _captures: &mut C) -> Dismissal {
        Dismissal::Retain
    }
}

#[derive(Debug, Clone)]
pub struct Popover<Inner, Overlay, Action = NoDismiss> {
    inner: Inner,
    overlay: Option<Overlay>,
    boundary_behavior: BoundaryBehavior,
    on_blur: Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverFocusTree<T, U> {
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

impl<T: FocusTree, U: FocusTree> FocusTree for PopoverFocusTree<T, U> {
    fn default_first() -> Self {
        Self {
            inner: FocusTree::default_first(),
            overlay: None,
        }
    }

    fn default_last() -> Self {
        Self {
            inner: FocusTree::default_last(),
            overlay: None,
        }
    }

    fn is_focused(&self) -> bool {
        self.inner.is_focused() || self.overlay.as_ref().is_some_and(FocusTree::is_focused)
    }
}

impl<Inner, Overlay> Popover<Inner, Overlay, NoDismiss>
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
            on_blur: NoDismiss,
        }
    }
}

impl<Inner, Overlay, Action> Popover<Inner, Overlay, Action>
where
    Inner: ViewMarker,
    Overlay: ViewMarker,
{
    /// Modifies the behavior when attempting to move focus beyond the ends of the overlay.
    #[must_use]
    pub fn with_boundary_behavior(mut self, behavior: BoundaryBehavior) -> Self {
        self.boundary_behavior = behavior;
        self
    }

    /// Installs a callback invoked when the overlay receives a `Blur` event.
    /// This should be used to update state backing the popover and decide whether
    /// the overlay should be dismissed by the blur.
    ///
    /// Returning [`Dismissal::Dismiss`] drops the overlay's focus subtree and
    /// requests a view rebuild; returning [`Dismissal::Retain`] keeps the
    /// overlay mounted and focused.
    ///
    /// `Teardown` events do not invoke the callback: the overlay's focus tree
    /// is simply dropped along with the rest of the stale focus tree.
    #[must_use]
    pub fn on_blur<F: Fn(&mut C) -> Dismissal, C>(self, on_blur: F) -> Popover<Inner, Overlay, F>
    where
        Self: ViewLayout<C>,
    {
        Popover {
            inner: self.inner,
            overlay: self.overlay,
            boundary_behavior: self.boundary_behavior,
            on_blur,
        }
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    fn handle_overlay_focus_event<Captures: ?Sized>(
        &self,
        overlay_view: &Overlay,
        focus_event: FocusAction,
        group: crate::focus::FocusGroup,
        context: &EventContext,
        render_tree: &mut <Self as ViewMarker>::Renderables,
        captures: &mut Captures,
        state: &mut PopoverState<Inner::State, Overlay::State>,
        focus: &mut PopoverFocusTree<Inner::FocusTree, Overlay::FocusTree>,
    ) -> EventResult
    where
        Inner: ViewLayout<Captures>,
        Overlay: ViewLayout<Captures>,
        Action: OnBlur<Captures>,
        Inner::Renderables: IntrinsicShape,
    {
        // FIXME: We don't know when the overlay appears, so we don't get a chance to obtain initial focus
        // Likely requires rethinking focus overall

        let subfocus = focus.overlay.get_or_insert_with(FocusTree::default_first);
        let overlay_state = state
            .overlay_state
            .get_or_insert_with(|| overlay_view.build_state(captures));

        let TransitionOption::Some { subtree, .. } = &mut render_tree.1.subtree else {
            // TODO: Popover is visible, but we don't have a render tree for it yet
            return EventResult::Deferred;
        };

        let result = overlay_view.handle_event(
            &Event::Focus {
                action: focus_event,
                group,
            },
            context,
            subtree,
            captures,
            overlay_state,
            subfocus,
        );

        if !matches!(result, EventResult::Deferred) {
            return result;
        }

        // The overlay's focused subtree did not handle the event.
        match focus_event {
            FocusAction::Blur => {
                match self.on_blur.on_blur(captures) {
                    Dismissal::Dismiss => {
                        // Drop the overlay focus subtree; focus returns to the
                        // inner view.
                        focus.overlay = None;
                        context.request_view_rebuild();
                        EventResult::handled_focused(render_tree.0.content_shape())
                    }
                    Dismissal::Retain => EventResult::handled_unfocused(),
                }
            }
            // Teardown drops the stale focus subtree without wrapping or
            // invoking on_blur; propagate the child's result so the lost-focus
            // signal survives.
            FocusAction::Teardown => result,
            _ => {
                let is_forward = matches!(
                    focus_event,
                    FocusAction::Next | FocusAction::Focus(FocusDirection::Forward)
                );

                *subfocus = if is_forward {
                    FocusTree::default_first()
                } else {
                    FocusTree::default_last()
                };
                let acquire_direction = if is_forward {
                    FocusDirection::Forward
                } else {
                    FocusDirection::Backward
                };
                overlay_view.handle_event(
                    &Event::Focus {
                        action: FocusAction::Focus(acquire_direction),
                        group,
                    },
                    context,
                    subtree,
                    captures,
                    overlay_state,
                    subfocus,
                )
            }
        }
    }
}

impl<Inner: ViewMarker, Overlay: ViewMarker, Action> ViewMarker
    for Popover<Inner, Overlay, Action>
{
    type Renderables = (
        Inner::Renderables,
        Animate<TransitionOption<Overlay::Renderables, Overlay::Transition>, bool>,
    );

    type Transition = Inner::Transition;
}

impl<Captures, Inner, Overlay, Action> ViewLayout<Captures> for Popover<Inner, Overlay, Action>
where
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    Overlay: ViewLayout<Captures>,
    Action: OnBlur<Captures>,
    Inner::Renderables: IntrinsicShape,
{
    type State = PopoverState<Inner::State, Overlay::State>;
    type Sublayout = ResolvedLayout<Inner::Sublayout>;
    type FocusTree = PopoverFocusTree<Inner::FocusTree, Overlay::FocusTree>;

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
                return self.handle_overlay_focus_event(
                    overlay_view,
                    *focus_event,
                    *group,
                    context,
                    render_tree,
                    captures,
                    state,
                    focus,
                );
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
                    focus.overlay.get_or_insert(FocusTree::default_first()),
                )
            }
            (Some(_), TransitionOption::None) => {
                // Overlay is present but not rendered yet - defer events until it actually appears
                EventResult::Deferred
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
