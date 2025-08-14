//! Scroll view and related configuration

use core::time::Duration;

use crate::{
    animation::Animation,
    event::{Event, EventContext, EventResult},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{Animate, Capsule, Offset, ScrollMetadata},
    view::{ViewLayout, ViewMarker},
};

/// The axes along which the scroll view can scroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollDirection {
    /// Constrain scrolling to the vertical axis
    #[default]
    Vertical,
    /// Constrain scrolling to the horizontal axis
    Horizontal,
    /// Allow scrolling in both axes
    Both,
}

/// Configuration for the scroll bars appearance and behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ScrollBarConfig {
    /// When to display scroll bars.
    visibility: ScrollBarVisibility,
    /// Padding applied around all edges of scroll bars.
    padding: u32,
    /// Bar width.
    width: u32,
    /// Whether the scroll bars overlap the content of the scroll view.
    overlaps_content: bool,
    /// The minimum length of the scroll bars.
    minimum_bar_length: u32,
}

impl Default for ScrollBarConfig {
    fn default() -> Self {
        Self {
            visibility: ScrollBarVisibility::default(),
            padding: 5,
            width: 6,
            overlaps_content: false,
            minimum_bar_length: 12,
        }
    }
}

/// When to show the scrollbar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ScrollBarVisibility {
    /// Always show the scrollbar
    #[default]
    Always,
    /// Never show the scrollbar
    Never,
    // /// Only show the scrollbar when scrolling
    // Auto,
}

/// A scroll view that allows scrolling through its inner content.
///
/// The scroll view can be configured to scroll in a specific direction (vertical, horizontal, or both),
/// and to display a scrollbar with various configurations such as visibility, padding, width, and minimum length.
///
/// # Examples
///
/// A vertically scrollable list of text content:
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::{pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
///
/// fn scrollable_content() -> impl View<Rgb565, ()> {
///     ScrollView::new(
///         VStack::new((
///             Text::new("Line 1", &FONT_9X15_BOLD),
///             Text::new("Line 2", &FONT_9X15_BOLD),
///             Text::new("Line 3", &FONT_9X15_BOLD),
///             Text::new("Line 4", &FONT_9X15_BOLD),
///             Text::new("Line 5", &FONT_9X15_BOLD),
///         ))
///     )
/// }
/// ```
///
/// Customizing the scrollbar appearance:
///
/// ```
/// use buoyant::view::{
///     prelude::*,
///     scroll_view::{ScrollBarVisibility, ScrollDirection}
/// };
/// use embedded_graphics::{pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
///
/// fn custom_scrollbar() -> impl View<Rgb565, ()> {
///     ScrollView::new(
///         VStack::new((
///             Text::new("Content 1", &FONT_9X15_BOLD),
///             Text::new("Content 2", &FONT_9X15_BOLD),
///             Text::new("Content 3", &FONT_9X15_BOLD),
///         ))
///     )
///     .with_direction(ScrollDirection::Both)
///     .with_bar_visibility(ScrollBarVisibility::Never)
///     .with_bar_padding(4)
///     .with_bar_width(8)
///     .with_overlapping_bar(false)
///     .with_minimum_bar_length(30)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ScrollView<Inner> {
    inner: Inner,
    bar_config: ScrollBarConfig,
    direction: ScrollDirection,
}

impl<Inner> ScrollView<Inner> {
    /// Constructs a new [`ScrollView`] with the given inner view.
    #[must_use]
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            bar_config: ScrollBarConfig::default(),
            direction: ScrollDirection::default(),
        }
    }

    /// Sets the axes along which the scroll view can scroll.
    #[must_use]
    pub fn with_direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the scrollbar overlaps the content.
    ///
    /// When `true`, the scrollbar is drawn over the content. When `false`,
    /// space is reserved for the scrollbar, reducing the available content area.
    #[must_use]
    pub fn with_overlapping_bar(mut self, overlaps: bool) -> Self {
        self.bar_config.overlaps_content = overlaps;
        self
    }

    /// Sets the minimum length of the scrollbar.
    #[must_use]
    pub fn with_minimum_bar_length(mut self, length: u32) -> Self {
        self.bar_config.minimum_bar_length = length;
        self
    }

    /// Sets the padding which is applied to all edges of the scrollbar.
    #[must_use]
    pub fn with_bar_padding(mut self, padding: u32) -> Self {
        self.bar_config.padding = padding;
        self
    }

    /// Sets the width (thickness) of the scrollbar.
    ///
    /// This applies to both horizontal and vertical scrollbars.
    #[must_use]
    pub fn with_bar_width(mut self, width: u32) -> Self {
        self.bar_config.width = width;
        self
    }

    /// Configures when the scrollbar should be visible.
    #[must_use]
    pub fn with_bar_visibility(mut self, visibility: ScrollBarVisibility) -> Self {
        self.bar_config.visibility = visibility;
        self
    }
}

/// A state machine to track the interaction state
#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum ScrollInteraction {
    /// No interaction
    #[default]
    Idle,
    /// Tracks interaction from touch down to touch up
    Dragging {
        drag_start: Point,
        last_point: Point,
        is_exclusive: bool,
    },
}

/// Persisted state for the scroll view and its inner view.
#[derive(Debug, Clone)]
pub struct ScrollViewState<InnerState> {
    scroll_offset: Point,
    interaction: ScrollInteraction,
    inner_state: InnerState,
}

impl<Inner: ViewMarker> ViewMarker for ScrollView<Inner> {
    type Renderables = ScrollMetadata<Inner::Renderables>;
}

impl<Inner: ViewLayout<Captures>, Captures> ViewLayout<Captures> for ScrollView<Inner> {
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

        let (horizontal_padding, vertical_padding) = if !self.bar_config.overlaps_content
            && self.bar_config.visibility == ScrollBarVisibility::Always
        {
            let bar_space = self.bar_config.padding * 2 + self.bar_config.width;
            match self.direction {
                ScrollDirection::Vertical => (bar_space, 0),
                ScrollDirection::Horizontal => (0, bar_space),
                ScrollDirection::Both => (bar_space, bar_space),
            }
        } else {
            (0, 0)
        };

        let (inner_width, inner_height) = match self.direction {
            ScrollDirection::Vertical => {
                (offer.width - horizontal_padding, ProposedDimension::Compact)
            }
            ScrollDirection::Horizontal => {
                (ProposedDimension::Compact, offer.height - vertical_padding)
            }
            ScrollDirection::Both => (ProposedDimension::Compact, ProposedDimension::Compact),
        };

        let inner_offer = ProposedDimensions::new(inner_width, inner_height);
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
        let scroll_view_width = layout.resolved_size.width.0;
        let scroll_view_height = layout.resolved_size.height.0;
        let inner_view_width = layout.sublayouts.resolved_size.width.0;
        let inner_view_height = layout.sublayouts.resolved_size.height.0;

        let permitted_offset_x = inner_view_width.saturating_sub(scroll_view_width) as i32;
        let permitted_offset_y = inner_view_height.saturating_sub(scroll_view_height) as i32;

        // Make sure the inner view is within the scroll view's bounds, mutate scroll
        // offset to correct only if not dragging
        let is_dragging;
        let subview_offset = match state.interaction {
            ScrollInteraction::Dragging { .. } => {
                // Movement beyond the bounds is reduced by half while dragging
                is_dragging = true;
                let mut offset = state.scroll_offset;

                // The offset should be set correctly based on the scroll direction
                // in the event handler, don't bother checking it here

                if offset.x > 0 {
                    // Overscrolling on the left
                    offset.x /= 2;
                } else if -offset.x > permitted_offset_x {
                    // Overscrolling on the right
                    offset.x = offset.x.midpoint(permitted_offset_x) - permitted_offset_x;
                }

                if offset.y > 0 {
                    // Overscrolling on the top
                    offset.y /= 2;
                } else if -offset.y > permitted_offset_y {
                    // Overscrolling on the bottom
                    offset.y = offset.y.midpoint(permitted_offset_y) - permitted_offset_y;
                }

                offset
            }
            ScrollInteraction::Idle => {
                is_dragging = false;

                state.scroll_offset.x = state.scroll_offset.x.clamp(-permitted_offset_x, 0);
                state.scroll_offset.y = state.scroll_offset.y.clamp(-permitted_offset_y, 0);

                state.scroll_offset
            }
        };

        let (horizontal_bar, vertical_bar) = self.scroll_bars(
            Point::zero(),
            Size::new(scroll_view_width, scroll_view_height),
            Size::new(inner_view_width, inner_view_height),
            subview_offset,
        );

        let inner_origin = subview_offset;
        let inner_render_tree = self.inner.render_tree(
            &layout.sublayouts,
            Point::zero(),
            env,
            captures,
            &mut state.inner_state,
        );
        let offset = Offset::new(inner_origin, inner_render_tree);
        let animation_time = if is_dragging {
            Duration::from_millis(0)
        } else {
            Duration::from_millis(300)
        };

        ScrollMetadata::new(
            Size::new(scroll_view_width, scroll_view_height),
            Size::new(inner_view_width, inner_view_height),
            Offset::new(
                origin,
                Animate::new(
                    (offset, horizontal_bar, vertical_bar),
                    Animation::ease_out_cubic(animation_time),
                    env.app_time(),
                    is_dragging,
                ),
            ),
        )
    }

    #[expect(clippy::too_many_lines)]
    fn handle_event(
        &self,
        event: &crate::event::Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        let (result, delta) = match event {
            Event::Scroll(delta) => (EventResult::new(true, true), *delta),
            Event::TouchDown(point) => {
                state.interaction = ScrollInteraction::Dragging {
                    drag_start: *point,
                    last_point: *point,
                    is_exclusive: false,
                };
                (
                    EventResult::new(true, true).merging(self.inner.handle_event(
                        &event.offset(-render_tree.offset() - render_tree.inner.offset),
                        context,
                        render_tree.inner_mut(),
                        captures,
                        &mut state.inner_state,
                    )),
                    Point::zero(),
                )
            }
            Event::TouchMoved(point) => match &mut state.interaction {
                ScrollInteraction::Dragging {
                    drag_start,
                    last_point,
                    is_exclusive,
                } => {
                    let delta = *point - *last_point;

                    *last_point = *point;
                    let total_scroll = *point - *drag_start;

                    if !*is_exclusive && (total_scroll.x.abs() >= 3 || total_scroll.y.abs() >= 3) {
                        // cancel inner interaction once we're sure the user intended to scroll
                        *is_exclusive = true;
                        (
                            EventResult::new(true, true).merging(self.inner.handle_event(
                                &Event::TouchCancelled,
                                context,
                                render_tree.inner_mut(),
                                captures,
                                &mut state.inner_state,
                            )),
                            delta,
                        )
                    } else {
                        (EventResult::new(true, true), delta)
                    }
                }
                ScrollInteraction::Idle => (EventResult::default(), Point::zero()),
            },
            Event::TouchUp(point) => match state.interaction {
                ScrollInteraction::Dragging {
                    drag_start: _,
                    last_point,
                    is_exclusive,
                } => {
                    state.interaction = ScrollInteraction::Idle;

                    let delta = *point - last_point;

                    if is_exclusive {
                        // If we don't set this, the scroll view will not animate the
                        // snap back because the frame time is old
                        render_tree.inner.subtree.frame_time = context.app_time;
                        render_tree.inner.subtree.value = true;
                        (EventResult::new(true, true), delta)
                    } else {
                        (
                            EventResult::new(true, true).merging(self.inner.handle_event(
                                &Event::TouchUp(
                                    *point - render_tree.offset() - render_tree.inner.offset,
                                ),
                                context,
                                render_tree.inner_mut(),
                                captures,
                                &mut state.inner_state,
                            )),
                            delta,
                        )
                    }
                }
                ScrollInteraction::Idle => (EventResult::default(), Point::zero()),
            },
            Event::TouchCancelled => {
                state.interaction = ScrollInteraction::Idle;
                // FIXME: This might not be right
                (
                    EventResult::new(true, true).merging(self.inner.handle_event(
                        &Event::TouchCancelled,
                        context,
                        render_tree.inner_mut(),
                        captures,
                        &mut state.inner_state,
                    )),
                    Point::zero(),
                )
            }
            _ => (EventResult::default(), Point::zero()),
        };

        match self.direction {
            ScrollDirection::Vertical => {
                state.scroll_offset.y += delta.y;
            }
            ScrollDirection::Horizontal => {
                state.scroll_offset.x += delta.x;
            }
            ScrollDirection::Both => {
                state.scroll_offset.x += delta.x;
                state.scroll_offset.y += delta.y;
            }
        }

        // Recompute scroll bars
        if delta != Point::zero() {
            let subview_offset = {
                let permitted_offset_x = render_tree
                    .inner_size
                    .width
                    .saturating_sub(render_tree.scroll_size.width)
                    as i32;
                let permitted_offset_y = render_tree
                    .inner_size
                    .height
                    .saturating_sub(render_tree.scroll_size.height)
                    as i32;

                // Movement beyond the bounds is reduced by half while dragging
                let mut offset = state.scroll_offset;
                if offset.x > 0 {
                    // Overscrolling on the left
                    offset.x /= 2;
                } else if -offset.x > permitted_offset_x {
                    // Overscrolling on the right
                    offset.x = offset.x.midpoint(permitted_offset_x) - permitted_offset_x;
                }

                if offset.y > 0 {
                    // Overscrolling on the top
                    offset.y /= 2;
                } else if -offset.y > permitted_offset_y {
                    // Overscrolling on the bottom
                    offset.y = offset.y.midpoint(permitted_offset_y) - permitted_offset_y;
                }

                offset
            };
            *render_tree.offset_mut() = subview_offset;
            let (horizontal_bar, vertical_bar) = self.scroll_bars(
                Point::zero(),
                render_tree.scroll_size,
                render_tree.inner_size,
                subview_offset,
            );
            render_tree.set_bars(horizontal_bar, vertical_bar);
        }

        result
    }
}

// TODO: remove generics to prevent excessive monomorphization
impl<V> ScrollView<V> {
    #[must_use]
    fn scroll_bars(
        &self,
        origin: Point,
        scroll_size: Size,
        inner_size: Size,
        subview_offset: Point,
    ) -> (Option<Capsule>, Option<Capsule>) {
        let overlap_padding = match self.direction {
            ScrollDirection::Vertical | ScrollDirection::Horizontal => 0,
            ScrollDirection::Both => self.bar_config.width + self.bar_config.padding,
        };

        let should_show_scrollbar = match self.bar_config.visibility {
            ScrollBarVisibility::Always => true,
            ScrollBarVisibility::Never => false,
        };

        // Create scrollbars based on direction
        let vertical_bar = if should_show_scrollbar
            && matches!(
                self.direction,
                ScrollDirection::Vertical | ScrollDirection::Both
            ) {
            let (bar_y, bar_height) = bar_size(
                scroll_size.height,
                inner_size.height,
                subview_offset.y,
                self.bar_config.minimum_bar_length,
                self.bar_config.padding,
                self.bar_config.padding + overlap_padding,
            );
            let bar_x = scroll_size
                .width
                .saturating_sub(self.bar_config.padding)
                .saturating_sub(self.bar_config.width);

            Some(Capsule::new(
                origin + Point::new(bar_x as i32, bar_y),
                Size::new(self.bar_config.width, bar_height),
            ))
        } else {
            None
        };

        let horizontal_bar = if should_show_scrollbar
            && matches!(
                self.direction,
                ScrollDirection::Horizontal | ScrollDirection::Both
            ) {
            let (bar_x, bar_width) = bar_size(
                scroll_size.width,
                inner_size.width,
                subview_offset.x,
                self.bar_config.minimum_bar_length,
                self.bar_config.padding,
                self.bar_config.padding + overlap_padding,
            );
            let bar_y = scroll_size
                .height
                .saturating_sub(self.bar_config.padding)
                .saturating_sub(self.bar_config.width);

            Some(Capsule::new(
                origin + Point::new(bar_x, bar_y as i32),
                Size::new(bar_width, self.bar_config.width),
            ))
        } else {
            None
        };
        (vertical_bar, horizontal_bar)
    }
}

/// Calculates the position and size of the scrollbar based on the scroll view's size and
/// attributes.
///
/// Returns a tuple containing the position of the scrollbar and its length.
fn bar_size(
    scroll_view_length: u32,
    inner_view_length: u32,
    scroll_offset: i32,
    min_length: u32,
    leading_padding: u32,
    trailing_padding: u32,
) -> (i32, u32) {
    let overscroll_amount = if scroll_offset > 0 {
        scroll_offset as u32
    } else {
        let max_offset = inner_view_length.saturating_sub(scroll_view_length);
        ((-scroll_offset) as u32).saturating_sub(max_offset)
    };

    let available_space = scroll_view_length.saturating_sub(leading_padding + trailing_padding);

    let perceived_scroll_length = scroll_view_length.saturating_sub(overscroll_amount);
    let bar_length = ((available_space * perceived_scroll_length)
        / inner_view_length.max(scroll_view_length))
    .max(min_length);

    let bar_position = if inner_view_length <= scroll_view_length {
        // Inner view is smaller, bar always touches the top or bottom
        if scroll_offset < 0 {
            // Bottom
            (leading_padding + available_space.saturating_sub(bar_length)) as i32
        } else {
            // Top
            leading_padding as i32
        }
    } else {
        // Actual scrollable content - position based on scroll progress
        let max_travel = available_space.saturating_sub(bar_length) as i32;
        let permitted_offset = (inner_view_length - scroll_view_length) as i32;

        let scroll_progress = (-scroll_offset).max(0).min(permitted_offset);
        leading_padding as i32 + (scroll_progress * max_travel) / permitted_offset
    };

    (bar_position, bar_length)
}

#[cfg(test)]
mod tests {
    use super::bar_size;

    #[test]
    fn smaller_inner() {
        // inner view is smaller, at rest
        let scroll_length = 100;
        let inner_length = 50;
        let scroll_offset = 0;
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 5);
        assert_eq!(bar_length, 90);
    }

    #[test]
    fn equal_inner() {
        // inner view is same size, at rest
        let scroll_length = 100;
        let inner_length = 100;
        let scroll_offset = 0;
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 5);
        assert_eq!(bar_length, 90);
    }

    #[test]
    fn double_inner_top() {
        // inner view is 2x scroll view, resting at top
        let scroll_length = 100;
        let inner_length = 200;
        let scroll_offset = 0;
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 5);
        assert_eq!(bar_length, 45); // (100 - 2 * 5) * 100 / 200
    }

    #[test]
    fn double_inner_bottom() {
        // inner view is 2x scroll view, resting at bottom (-y scrolls down)
        let scroll_length = 100;
        let inner_length = 200;
        let scroll_offset = -100;
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 50);
        assert_eq!(bar_length, 45); // (100 - 2 * 5) * 100 / 200
    }

    #[test]
    fn double_inner_slight_overscroll_top() {
        // inner view is 2x scroll view, overscrolled at top
        let scroll_length = 100;
        let inner_length = 200;
        let scroll_offset = 20; // slight overscroll up
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 5);
        assert_eq!(bar_length, 36); // (100 - 2 * 5) * (100 - 20) / 200
    }

    #[test]
    fn double_inner_slight_overscroll_bottom() {
        // inner view is 2x scroll view, overscrolled at bottom
        let scroll_length = 100;
        let inner_length = 200;
        let scroll_offset = -120; // slight overscroll down
        let min_length = 10;
        let leading_padding = 5;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 59);
        assert_eq!(bar_length, 36); // (100 - 2 * 5) * (100 - 20) / 200
    }

    #[test]
    fn min_length_bar_top_rest() {
        // inner view is 1000x scroll view, resting at top
        let scroll_length = 100;
        let inner_length = 100_000;
        let scroll_offset = 0;
        let min_length = 7;
        let leading_padding = 10;
        let trailing_padding = 10;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 10);
        assert_eq!(bar_length, 7);
    }

    #[test]
    fn min_length_bar_bottom_rest() {
        // inner view is 1000x scroll view, resting at bottom
        let scroll_length = 100;
        let inner_length = 100_000;
        let scroll_offset = -99900;
        let min_length = 9;
        let leading_padding = 2;
        let trailing_padding = 5;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 86); // 100 - 5 - 9
        assert_eq!(bar_length, 9);
    }

    #[test]
    fn half_length_bar_top_overscrolled() {
        // inner view overscrolled up to cause half bar length
        let scroll_length = 100;
        let inner_length = 10;
        let scroll_offset = 50; // overscrolled up halfway
        let min_length = 7;
        let leading_padding = 10;
        let trailing_padding = 10;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 10);
        assert_eq!(bar_length, 40); // (100 - (10 + 10)) * 50 / 100
    }

    #[test]
    fn half_length_bar_bottom_overscrolled() {
        // inner view overscrolled down to cause half bar length
        let scroll_length = 100;
        let inner_length = 10;
        let scroll_offset = -50; // overscrolled down halfway
        let min_length = 9;
        let leading_padding = 2;
        let trailing_padding = 2;

        let (bar_y, bar_length) = bar_size(
            scroll_length,
            inner_length,
            scroll_offset,
            min_length,
            leading_padding,
            trailing_padding,
        );

        assert_eq!(bar_y, 50);
        assert_eq!(bar_length, 48); // (100 - (2 + 2)) * 50 / 100
    }
}
