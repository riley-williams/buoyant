use core::marker::PhantomData;

use crate::{
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusAction, FocusDirection, FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    render::TransitionOption,
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct Popover<Inner, ViewFn, Overlay, T> {
    inner: Inner,
    behavior: Behavior,
    view_fn: ViewFn,
    _overlay: PhantomData<Overlay>,
    value: Option<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Behavior {
    /// Wrap focus around to the other side when reaching the end
    #[default]
    Wrap,
    /// Stop movement at the ends
    Terminate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusTree<T, U> {
    /// The inner view's focus state (always preserved)
    pub inner: T,
    /// The overlay's focus state (when overlay is active)
    pub overlay: Option<U>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl<Inner, Overlay, ViewFn, T> Popover<Inner, ViewFn, Overlay, T>
where
    Inner: ViewMarker,
    Overlay: ViewMarker,
    ViewFn: for<'b> Fn(&'b T) -> Overlay,
    T: Clone,
{
    #[must_use]
    pub fn new(inner: Inner, value: Option<T>, view_fn: ViewFn) -> Self {
        Self {
            inner,
            behavior: Behavior::default(),
            view_fn,
            _overlay: PhantomData,
            value,
        }
    }
}

impl<Inner: ViewMarker, ViewFn, Overlay: ViewMarker, T> ViewMarker
    for Popover<Inner, ViewFn, Overlay, T>
{
    type Renderables = (
        Inner::Renderables,
        TransitionOption<Overlay::Renderables, Overlay::Transition>,
    );

    type Transition = Inner::Transition;
}

impl<Captures, Inner, ViewFn, Overlay, T> ViewLayout<Captures>
    for Popover<Inner, ViewFn, Overlay, T>
where
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    Overlay: ViewLayout<Captures>,
    ViewFn: for<'b> Fn(&'b T) -> Overlay,
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
            overlay_state: self.value.as_ref().map(|v| {
                let overlay_view = (self.view_fn)(v);
                overlay_view.build_state(captures)
            }),
        }
    }

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let inner_layout = self
            .inner
            .layout(offer, env, captures, &mut state.inner_state);

        // Take as much space as possible
        let size = offer.resolve_most_flexible(0, 1);
        ResolvedLayout {
            sublayouts: inner_layout,
            resolved_size: size,
        }
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
        let overlay_tree = match &self.value {
            Some(v) => {
                let overlay_view = (self.view_fn)(v);
                let overlay_state = state
                    .overlay_state
                    .get_or_insert_with(|| overlay_view.build_state(captures));
                let overlay_layout = overlay_view
                    .layout(&layout.resolved_size.into(), env, captures, overlay_state)
                    .sublayouts;
                TransitionOption::new_some(
                    overlay_view.render_tree(&overlay_layout, origin, env, captures, overlay_state),
                    layout.resolved_size.into(),
                    overlay_view.transition(),
                )
            }
            _ => TransitionOption::None,
        };
        (inner_tree, overlay_tree)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        // FIXME: State handling?
        match (&self.value, &mut render_tree.1, &mut state.overlay_state) {
            (Some(v), TransitionOption::Some { subtree, .. }, Some(s)) => {
                let overlay_view = (self.view_fn)(v);
                overlay_view.handle_event(event, context, subtree, captures, s)
            }
            _ => self.inner.handle_event(
                event,
                context,
                &mut render_tree.0,
                captures,
                &mut state.inner_state,
            ),
        }
    }

    fn focus(
        &self,
        event: &FocusEvent,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        if let Some(v) = &self.value {
            // Overlay is active - ensure we have overlay focus state
            let subfocus = focus
                .overlay
                .get_or_insert_with(DefaultFocus::default_first);

            let view = (self.view_fn)(v);
            let overlay_state = state
                .overlay_state
                .get_or_insert_with(|| view.build_state(captures));

            if let TransitionOption::Some { subtree, .. } = &mut render_tree.1 {
                let result = view.focus(event, context, subtree, captures, overlay_state, subfocus);

                if matches!(result, FocusStateChange::Exhausted) {
                    match self.behavior {
                        Behavior::Wrap => {
                            *subfocus = DefaultFocus::default_first();
                            return view.focus(
                                event,
                                context,
                                subtree,
                                captures,
                                overlay_state,
                                subfocus,
                            );
                        }
                        Behavior::Terminate => {
                            // Refocus on last element
                            view.focus(
                                &FocusEvent::new(
                                    FocusAction::Focus(FocusDirection::Backward),
                                    event.roles,
                                ),
                                context,
                                subtree,
                                captures,
                                overlay_state,
                                subfocus,
                            );
                        }
                    }
                }

                result
            } else {
                // FIXME: Attempt to recover?
                FocusStateChange::Exhausted
            }
        } else {
            // Overlay is not active - clear overlay focus and use inner focus
            focus.overlay = None;

            self.inner.focus(
                event,
                context,
                &mut render_tree.0,
                captures,
                &mut state.inner_state,
                &mut focus.inner,
            )
        }
    }
}

// Something in here can be reused for the thing I was originally making...
//
//     match focus {
//         FocusTree::Inner(inner) => {
//             let inner_state = self.inner.focus(
//                 event,
//                 context,
//                 &mut render_tree.child,
//                 captures,
//                 state,
//                 inner,
//             );
//             if inner_state == FocusStateChange::Exhausted
//                 && matches!(event.action, FocusAction::Blur)
//             {
//                 *focus = FocusTree::Container;
//                 FocusStateChange::Focused {
//                     shape: ContentShape::Rectangle(render_tree.frame.clone()),
//                     result: EventResult::new(true, true, true),
//                 }
//             } else {
//                 inner_state
//             }
//         }
//         FocusTree::Container => match &event.action {
//             FocusAction::Next | FocusAction::Previous | FocusAction::Blur => {
//                 FocusStateChange::Exhausted
//             }
//             FocusAction::Focus(_) => FocusStateChange::Focused {
//                 shape: ContentShape::Rectangle(render_tree.frame.clone()),
//                 result: EventResult::new(true, true, true),
//             },
//             FocusAction::Select => {
//                 // Update focus to first inner element
//                 // FIXME: Check if role includes containers?
//                 // FIXME: ::Select maybe needs a direction?
//                 *focus = FocusTree::Inner(DefaultFocus::default_first());
//                 if let FocusTree::Inner(inner_focus) = focus {
//                     self.inner.focus(
//                         event,
//                         context,
//                         &mut render_tree.child,
//                         captures,
//                         state,
//                         inner_focus,
//                     )
//                 } else {
//                     unreachable!()
//                 }
//             }
//         },
//     }
// }
//
