use crate::{
    environment::LayoutEnvironment,
    event::{EventContext, EventResult},
    focus::DefaultFocus,
    layout::{Alignment, HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

use paste::paste;

/// A stack of heterogeneous views that arranges its children from back to front.
///
/// The parent offer is first offered to each subview. If any offered dimension is
/// [`ProposedDimension::Compact`], [`ZStack`] will offer a new frame that is the
/// union of all the resolved frame sizes from the first pass.
///
/// ```rust
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::view::prelude::*;
///
/// /// A fish at the bottom right corner of an 'o'cean
/// let font = CharacterBufferFont {};
/// let stack = ZStack::new((
///         Rectangle,
///         Text::new("><>", &font),
///     ))
///     .with_alignment(Alignment::BottomTrailing)
///     .foreground_color('o');
/// ```
#[derive(Debug, Clone)]
pub struct ZStack<T> {
    items: T,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T> PartialEq for ZStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<T> ZStack<T> {
    /// Sets the horizontal alignment to use when placing child views of different widths.
    #[must_use]
    pub fn with_horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: alignment,
            ..self
        }
    }

    /// Sets the vertical alignment to use when placing child views of different heights.
    #[must_use]
    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: alignment,
            ..self
        }
    }

    /// Sets the horizontal and vertical alignment to use when placing child views of different
    /// sizes.
    #[must_use]
    pub fn with_alignment(self, alignment: Alignment) -> Self {
        Self {
            horizontal_alignment: alignment.horizontal(),
            vertical_alignment: alignment.vertical(),
            ..self
        }
    }
}

impl<T: ViewMarker> ZStack<T> {
    #[allow(missing_docs)]
    pub fn new(items: T) -> Self {
        Self {
            items,
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }
}

macro_rules! impl_view_for_zstack {
    ($ct:tt, $(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type),+> ViewMarker for ZStack<($($type),+)>
        where
            $($type: ViewMarker),+
        {
            type Renderables = ($($type::Renderables),+);
            type Transition = crate::transition::Opacity;
        }

        impl<Captures: ?Sized, $($type),+> ViewLayout<Captures> for ZStack<($($type),+)>
        where
            $($type: ViewLayout<Captures>),+
        {
            type Sublayout = ResolvedLayout<($(ResolvedLayout<$type::Sublayout>),+)>;
            type State = ($($type::State),+);
            type FocusTree = super::match_view::[<OneOf $ct>]<$($type::FocusTree),+>;

            fn transition(&self) -> Self::Transition {
                crate::transition::Opacity
            }

            fn build_state(&self, captures: &mut Captures) -> Self::State {
                ($(self.items.$n.build_state(captures)),+)
            }

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> ResolvedLayout<Self::Sublayout> {
                $(
                    let mut [<layout$n>] = self.items.$n.layout(offer, env, captures, &mut state.$n);
                )+
                let mut size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;

                // FIXME: Move this logic to render_tree. It doesn't affect the reported size. Also
                    // fixes the other fixme

                if matches!(offer.width, ProposedDimension::Compact) || matches!(offer.height, ProposedDimension::Compact) {
                    // FIXME: The `.into()` here is almost certainly wrong.
                    // While it would be unusual for a view to respond requesting infinite
                    // width or height in response to a compact request, this does not
                    // effectively handle it. This also increases the likelihood of overflow
                    // due to the way Dimension is implemented
                    let offer = ProposedDimensions {
                        width: ProposedDimension::Exact(size.width.into()),
                        height: ProposedDimension::Exact(size.height.into()),
                    };
                    $(
                        [<layout$n>] = self.items.$n.layout(&offer, env, captures, &mut state.$n);
                    )+
                    size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;
                }

                ResolvedLayout {
                    sublayouts: ($(
                        [<layout$n>]
                    ),+),
                    resolved_size: size.intersecting_proposal(offer),
                }.nested()
            }

            fn render_tree(
                &self,
                layout: &Self::Sublayout,
                origin: Point,
                env: &impl LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> Self::Renderables {
                $(
                    let [<offset_$n>] = origin
                        + Point::new(
                            self.horizontal_alignment.align(
                                layout.resolved_size.width.into(),
                                layout.sublayouts.$n.resolved_size.width.into(),
                            ),
                            self.vertical_alignment.align(
                                layout.resolved_size.height.into(),
                                layout.sublayouts.$n.resolved_size.height.into(),
                            ),
                        );
                )+

                (
                    $(
                        self.items.$n.render_tree(&layout.sublayouts.$n.sublayouts, [<offset_$n>], env, captures, &mut state.$n)
                    ),+
                )
            }

            fn handle_event(
                &self,
                event: &crate::view::Event,
                context: &EventContext,
                render_tree: &mut Self::Renderables,
                captures: &mut Captures,
                state: &mut Self::State,
                focus: &mut Self::FocusTree,
            ) -> EventResult {
                use crate::event::Event;
                use crate::focus::{FocusAction, FocusDirection};

                use super::match_view::[<OneOf $ct>];

                // Handle focus events specially - they need to route through the focus tree
                if let Event::Focus { action: focus_event, group } = event {
                    // Track which child index we're currently trying
                    let mut current: usize = match focus {
                        $(
                            [<OneOf $ct>]::[<V $n>](_) => $n,
                        )+
                    };

                    // The event to use - initially the original event, but when entering
                    // a new child during navigation we switch to a Focus event
                    let mut current_event = focus_event.clone();

                    // Track whether any child in this traversal gave up focus, so the
                    // signal isn't lost when a later child defers without losing focus.
                    let mut focus_lost = false;

                    loop {
                        // Try focus on the current child
                        let result = match focus {
                            $(
                                [<OneOf $ct>]::[<V $n>](f) => {
                                    self.items.$n.handle_event(
                                        &Event::Focus { action: current_event.clone(), group: *group },
                                        context,
                                        &mut render_tree.$n,
                                        captures,
                                        &mut state.$n,
                                        f,
                                    )
                                }
                            )+
                        };

                        focus_lost |= result.lost_focus();

                        // If the child handled it (not deferred), return the result
                        if !matches!(result, EventResult::Deferred { .. }) || current_event == FocusAction::Teardown {
                            return result;
                        }

                        // Child is exhausted, try to move based on action
                        match focus_event {
                            FocusAction::Blur | FocusAction::Teardown => {
                                debug_assert!(!matches!(focus_event, FocusAction::Teardown), "Teardown events should not loop");
                                return EventResult::Deferred { focus_lost };
                            }
                            FocusAction::Focus(FocusDirection::Forward) | FocusAction::Select | FocusAction::Next => {
                                // Advance to next child
                                current += 1;
                                match current {
                                    $(
                                        $n => {
                                            *focus = [<OneOf $ct>]::[<V $n>](DefaultFocus::default_first());
                                        }
                                    )+
                                    _ => return EventResult::Deferred { focus_lost },
                                }
                                // When entering a new child, use Focus action (forward)
                                current_event = FocusAction::Focus(FocusDirection::Forward);
                            }
                            FocusAction::Focus(FocusDirection::Backward) | FocusAction::Previous => {
                                // Go to previous child
                                if current == 0 {
                                    return EventResult::Deferred { focus_lost };
                                }
                                current -= 1;
                                match current {
                                    $(
                                        $n => {
                                            *focus = [<OneOf $ct>]::[<V $n>](DefaultFocus::default_last());
                                        }
                                    )+
                                    _ => return EventResult::Deferred { focus_lost },
                                }
                                // When entering a new child, use Focus action (backward)
                                current_event = FocusAction::Focus(FocusDirection::Backward);
                            }
                        }
                    }
                } else if matches!(event, Event::KeyDown { .. } | Event::KeyUp { .. }) {
                    return match focus {
                        $(
                            [<OneOf $ct>]::[<V $n>](f) => self.items.$n.handle_event(
                                event,
                                context,
                                &mut render_tree.$n,
                                captures,
                                &mut state.$n,
                                f,
                            ),
                        )+
                    };
                }

                // For non-focus events (touch, scroll, etc.), use DFS approach
                $(
                    let mut default_focus = DefaultFocus::default_first();
                    let inner_focus = if let Event::Touch(..) = event {
                        focus.[<v $n _or_init_with>](|| DefaultFocus::default_first())
                    } else {
                        focus.[<v $n _mut>]().unwrap_or(&mut default_focus)
                    };
                    let result = self.items.$n.handle_event(
                        event,
                        context,
                        &mut render_tree.$n,
                        captures,
                        &mut state.$n,
                        inner_focus,
                    );
                    if result.is_handled() {
                        return result;
                    }
                )+
                result
            }
        }
    }
    }
}

impl_view_for_zstack!(2, (0, T0), (1, T1));
impl_view_for_zstack!(3, (0, T0), (1, T1), (2, T2));
impl_view_for_zstack!(4, (0, T0), (1, T1), (2, T2), (3, T3));
impl_view_for_zstack!(5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_view_for_zstack!(6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_view_for_zstack!(
    7,
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_view_for_zstack!(
    8,
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_view_for_zstack!(
    9,
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8)
);
impl_view_for_zstack!(
    10,
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8),
    (9, T9)
);

// Implement single-item conformance for convenience, although it does nothing
impl<T> ViewMarker for ZStack<(T,)>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
    type Transition = crate::transition::Opacity;
}

impl<Captures, T> ViewLayout<Captures> for ZStack<(T,)>
where
    T: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type Sublayout = T::Sublayout;
    type State = T::State;
    type FocusTree = T::FocusTree;

    fn transition(&self) -> Self::Transition {
        crate::transition::Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.items.0.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.items.0.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.items
            .0
            .render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::event::Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        self.items
            .0
            .handle_event(event, context, render_tree, captures, state, focus)
    }
}
