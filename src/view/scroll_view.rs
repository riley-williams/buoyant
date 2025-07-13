use core::time::Duration;

use crate::{
    animation::Animation,
    event::Event,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render::{Animate, Offset},
    view::{ViewLayout, ViewMarker},
};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ScrollView<Inner> {
    inner: Inner,
}

impl<Inner> ScrollView<Inner> {
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone, Default)]
pub enum ScrollInteraction {
    #[default]
    Idle,
    Dragging {
        drag_start: Point,
        last_point: Point,
        is_exclusive: bool,
    },
}

#[derive(Debug, Clone)]
pub struct ScrollViewState<InnerState> {
    scroll_offset: Point,
    interaction: ScrollInteraction,
    inner_state: InnerState,
}

impl<Inner: ViewMarker<Renderables: Clone>> ViewMarker for ScrollView<Inner> {
    type Renderables = Animate<Offset<Inner::Renderables>, bool>;
}

impl<Inner: ViewLayout<Captures, Renderables: Clone>, Captures> ViewLayout<Captures>
    for ScrollView<Inner>
{
    type State = ScrollViewState<Inner::State>;
    type Sublayout = ResolvedLayout<Inner::Sublayout>;

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        Self::State {
            scroll_offset: Point::zero(),
            interaction: ScrollInteraction::Idle,
            inner_state: self.inner.build_state(captures),
        }
    }

    fn layout(
        &self,
        offer: &crate::primitives::ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let dimensions = offer.resolve_most_flexible(0, 1);
        let inner_offer = ProposedDimensions::new(offer.width, ProposedDimension::Compact);
        let inner_layout = self
            .inner
            .layout(&inner_offer, env, captures, &mut state.inner_state);
        ResolvedLayout {
            sublayouts: inner_layout,
            resolved_size: dimensions,
        }
    }

    fn render_tree(
        &self,
        layout: &crate::layout::ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        // Make sure the inner view is within the scroll view's bounds, adjust scroll offset
        // if not dragging
        let is_dragging;
        let permitted_offset = (layout.sublayouts.resolved_size.height.0 as i32
            - layout.resolved_size.height.0 as i32)
            .max(0);
        let subview_offset = match state.interaction {
            ScrollInteraction::Dragging { .. } => {
                is_dragging = true;
                let mut offset = state.scroll_offset;

                // Apply elastic behavior: If we're scrolling beyond bounds,
                // make the content appear to move only 1/3 as fast
                if offset.y > 0 {
                    // Overscrolling at the top
                    offset.y /= 2;
                } else if offset.y < -permitted_offset {
                    // Overscrolling at the bottom
                    let overscroll = offset.y + permitted_offset;
                    offset.y = -permitted_offset + overscroll / 2;
                }

                offset
            }
            ScrollInteraction::Idle => {
                is_dragging = false;
                let permitted_offset = (layout.sublayouts.resolved_size.height.0 as i32
                    - layout.resolved_size.height.0 as i32)
                    .max(0);
                if state.scroll_offset.y > 0 {
                    state.scroll_offset.y = 0;
                } else if state.scroll_offset.y < -permitted_offset {
                    state.scroll_offset.y = -permitted_offset;
                }
                state.scroll_offset
            }
        };
        let inner_origin = origin + subview_offset;
        let inner_render_tree = self.inner.render_tree(
            &layout.sublayouts,
            Point::zero(),
            env,
            captures,
            &mut state.inner_state,
        );
        let offset = Offset::new(inner_origin, inner_render_tree);
        Animate::new(
            offset,
            Animation::ease_out(Duration::from_millis(200)),
            env.app_time(),
            is_dragging,
        )
    }

    fn handle_event(
        &mut self,
        event: &crate::event::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        match event {
            Event::Scroll(delta) => {
                state.scroll_offset.y += delta.y;
                true
            }
            Event::TouchDown(point) => {
                state.interaction = ScrollInteraction::Dragging {
                    drag_start: *point,
                    last_point: *point,
                    is_exclusive: false,
                };
                self.inner.handle_event(
                    &event.offset(Point::zero() - render_tree.subtree.offset),
                    &mut render_tree.subtree.subtree,
                    captures,
                    &mut state.inner_state,
                );
                true
            }
            Event::TouchMoved(point) => match &mut state.interaction {
                ScrollInteraction::Dragging {
                    drag_start,
                    last_point,
                    is_exclusive,
                } => {
                    let delta = *point - *last_point;
                    state.scroll_offset.y += delta.y;
                    *last_point = *point;
                    let total_scroll = *point - *drag_start;
                    if !*is_exclusive && (total_scroll.x.abs() >= 5 || total_scroll.y.abs() >= 5) {
                        *is_exclusive = true;
                        self.inner.handle_event(
                            &Event::TouchCancelled,
                            &mut render_tree.subtree.subtree,
                            captures,
                            &mut state.inner_state,
                        );
                    }
                    true
                }
                ScrollInteraction::Idle => false,
            },
            Event::TouchUp(point) => {
                let result = match &mut state.interaction {
                    ScrollInteraction::Dragging {
                        drag_start: _,
                        last_point,
                        is_exclusive,
                    } => {
                        let delta = *point - *last_point;
                        state.scroll_offset.y += delta.y;
                        if *is_exclusive {
                            true
                        } else {
                            self.inner.handle_event(
                                &Event::TouchUp(*point - render_tree.subtree.offset),
                                &mut render_tree.subtree.subtree,
                                captures,
                                &mut state.inner_state,
                            )
                        }
                    }
                    ScrollInteraction::Idle => false,
                };
                state.interaction = ScrollInteraction::Idle;
                result
            }
            Event::TouchCancelled => {
                state.interaction = ScrollInteraction::Idle;
                self.inner.handle_event(
                    &Event::TouchCancelled,
                    &mut render_tree.subtree.subtree,
                    captures,
                    &mut state.inner_state,
                );
                true
            }
            _ => false,
        }
    }
}
