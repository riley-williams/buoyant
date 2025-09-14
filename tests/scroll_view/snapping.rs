use crate::common::helpers::tree;
use crate::common::{touch_move, touch_up};
use crate::{assert_str_grid_eq, common::touch_down};
use core::time::Duration;

use buoyant::{
    event::EventContext,
    primitives::{Point, Size},
    render::Render,
    render_target::FixedTextBuffer,
    view::{prelude::*, scroll_view::ScrollDirection},
};

fn vertical_scroll_view<T>() -> impl View<char, T> {
    ScrollView::new(VStack::new((
        Rectangle.frame().with_height(4).foreground_color('a'),
        Rectangle.frame().with_height(4).foreground_color('b'),
        Rectangle.frame().with_height(4).foreground_color('c'),
    )))
    .with_direction(ScrollDirection::Vertical)
    .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
    .padding(Edges::All, 1)
}

fn horizontal_scroll_view<T>() -> impl View<char, T> {
    ScrollView::new(HStack::new((
        Rectangle.frame().with_width(4).foreground_color('a'),
        Rectangle.frame().with_width(4).foreground_color('b'),
        Rectangle.frame().with_width(4).foreground_color('c'),
    )))
    .with_direction(ScrollDirection::Horizontal)
    .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
    .padding(Edges::All, 1)
}

fn both_direction_scroll_view<T>() -> impl View<char, T> {
    ScrollView::new(VStack::new((
        HStack::new((
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('a'),
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('b'),
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('c'),
        )),
        HStack::new((
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('d'),
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('e'),
            Rectangle
                .frame()
                .with_width(4)
                .with_height(2)
                .foreground_color('f'),
        )),
    )))
    .with_direction(ScrollDirection::Both)
    .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
    .padding(Edges::All, 1)
}

#[test]
fn scroll_down_snaps_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = vertical_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    render_tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            "            ",
        ],
        &buffer.text
    );
    let event_result = view.handle_event(
        &touch_down(Point::new(2, 4)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Pull down
    let event_result = view.handle_event(
        &touch_move(Point::new(2, 8)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            "            ",
            "            ",
            " aaaaaaaaaa ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &touch_up(Point::new(2, 8)), // Just at the bottom edge
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            "            ",
            "            ",
            " aaaaaaaaaa ",
            "            ",
        ],
        &buffer.text
    );

    // View should snap back after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            "            ",
        ],
        &buffer.text
    );
}

#[test]
fn scroll_up_past_bottom_snaps_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = vertical_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    // First scroll to the bottom of the content normally
    let event_result = view.handle_event(
        &touch_down(Point::new(2, 2)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Scroll up to just touch bottom content
    let event_result = view.handle_event(
        &touch_move(Point::new(2, -7)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " cccccccccc ",
            " cccccccccc ",
            " cccccccccc ",
            "            ",
        ],
        &buffer.text
    );

    // Now scroll past the bottom limit - additional movement should be reduced by half
    let event_result = view.handle_event(
        &touch_move(Point::new(2, -11)), // 4 past the limit
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " cccccccccc ",
            "            ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    // Release touch
    let event_result = view.handle_event(
        &touch_up(Point::new(2, -11)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // We're modifying the target tree, so it should retain the scroll position
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " cccccccccc ",
            "            ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    // Recomputed view should snap back to proper bottom position after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " cccccccccc ",
            " cccccccccc ",
            " cccccccccc ",
            "            ",
        ],
        &buffer.text
    );
}

#[test]
fn horizontal_scroll_right_snaps_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = horizontal_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    render_tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "            ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            "            ",
        ],
        &buffer.text
    );
    let event_result = view.handle_event(
        &touch_down(Point::new(4, 2)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Pull right
    let event_result = view.handle_event(
        &touch_move(Point::new(8, 2)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &touch_up(Point::new(8, 2)), // Just at the right edge
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "            ",
        ],
        &buffer.text
    );

    // View should snap back after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            "            ",
        ],
        &buffer.text
    );
}

#[test]
fn both_direction_scroll_diagonal_snaps_back_up_left() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = both_direction_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    render_tree.render(&mut buffer, &' ');

    // Initial view shows top-left corner
    assert_str_grid_eq!(
        [
            "            ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            " ddddeeeeff ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &touch_down(Point::new(2, 2)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Pull diagonally down-right past the top-left bounds
    let event_result = view.handle_event(
        &touch_move(Point::new(6, 4)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    // Should show content pulled past bounds with resistance
    assert_str_grid_eq!(
        [
            "            ",
            "            ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &touch_up(Point::new(6, 4)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    // Should still show the over-scrolled position before snapping
    assert_str_grid_eq!(
        [
            "            ",
            "            ",
            "   aaaabbbb ",
            "   aaaabbbb ",
            "            ",
        ],
        &buffer.text
    );

    // View should snap back to normal position after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aaaabbbbcc ",
            " aaaabbbbcc ",
            " ddddeeeeff ",
            "            ",
        ],
        &buffer.text
    );
}

#[test]
fn both_direction_scroll_bottom_right_snaps_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = both_direction_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    // First scroll to the bottom-right corner normally
    let event_result = view.handle_event(
        &touch_down(Point::new(6, 3)),
        &EventContext::new(Duration::from_millis(500)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Scroll to bottom-right corner
    let event_result = view.handle_event(
        &touch_move(Point::new(4, 1)),
        &EventContext::new(Duration::from_millis(600)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    // Should show bottom-right content
    assert_str_grid_eq!(
        [
            "            ",
            " aabbbbcccc ",
            " ddeeeeffff ",
            " ddeeeeffff ",
            "            ",
        ],
        &buffer.text
    );

    // Now scroll past the bottom-right bounds (up-left movement past limits)
    let event_result = view.handle_event(
        &touch_move(Point::new(0, -3)),
        &EventContext::new(Duration::from_millis(700)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    // Should show content pulled past bounds with resistance
    assert_str_grid_eq!(
        [
            "            ",
            " eeeeffff   ",
            "            ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &touch_up(Point::new(0, -3)),
        &EventContext::new(Duration::from_millis(800)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    // Should still show the over-scrolled position before snapping
    assert_str_grid_eq!(
        [
            "            ",
            " eeeeffff   ",
            "            ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    // View should snap back to bottom-right corner after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aabbbbcccc ",
            " ddeeeeffff ",
            " ddeeeeffff ",
            "            ",
        ],
        &buffer.text
    );
}

#[test]
fn horizontal_scroll_left_past_right_edge_snaps_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = horizontal_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    // First scroll to the right edge of the content normally
    let event_result = view.handle_event(
        &touch_down(Point::new(2, 2)),
        &EventContext::new(Duration::from_millis(500)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Scroll left to just touch right edge content
    let event_result = view.handle_event(
        &touch_move(Point::new(0, 2)),
        &EventContext::new(Duration::from_millis(600)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aabbbbcccc ",
            " aabbbbcccc ",
            " aabbbbcccc ",
            "            ",
        ],
        &buffer.text
    );

    // Now scroll past the right limit - additional movement should be reduced by half
    let event_result = view.handle_event(
        &touch_move(Point::new(-8, 2)), // 4 past the limit
        &EventContext::new(Duration::from_millis(700)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " bbcccc     ",
            " bbcccc     ",
            " bbcccc     ",
            "            ",
        ],
        &buffer.text
    );

    // Release touch
    let event_result = view.handle_event(
        &touch_up(Point::new(-8, 2)),
        &EventContext::new(Duration::from_millis(800)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // We're modifying the target tree, so it should retain the scroll position
    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " bbcccc     ",
            " bbcccc     ",
            " bbcccc     ",
            "            ",
        ],
        &buffer.text
    );

    // Recomputed view should snap back to right after releasing
    let render_tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    buffer.clear();
    render_tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aabbbbcccc ",
            " aabbbbcccc ",
            " aabbbbcccc ",
            "            ",
        ],
        &buffer.text
    );
}
