use core::time::Duration;

use buoyant::{
    event::{Event, EventContext},
    primitives::{Point, Size},
    render::{AnimationDomain, Render},
    render_target::FixedTextBuffer,
    view::{prelude::*, scroll_view::ScrollDirection},
};

use crate::assert_str_grid_eq;
use crate::common::helpers::tree;

fn vertical_scroll_view<T>() -> impl View<char, T> {
    ScrollView::new(Rectangle.frame().with_height(2).foreground_color('a'))
        .with_direction(ScrollDirection::Vertical)
        .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
        .padding(Edges::All, 1)
}

#[expect(clippy::too_many_lines)]
#[test]
fn scroll_down_animates_back() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = vertical_scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut render_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(0),
        size,
    );

    render_tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
    // picking times much greater than the scroll animation duration
    let event_result = view.handle_event(
        &Event::TouchDown(Point::new(2, 4)),
        &EventContext::new(Duration::from_millis(500)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    // Pull down just offscreen and release
    let event_result = view.handle_event(
        &Event::TouchMoved(Point::new(2, 10)),
        &EventContext::new(Duration::from_millis(1000)),
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
            "            ",
            "            ",
        ],
        &buffer.text
    );

    let event_result = view.handle_event(
        &Event::TouchUp(Point::new(2, 8)),
        &EventContext::new(Duration::from_millis(1500)),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert!(event_result.handled);

    let new_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(1500),
        size,
    );
    buffer.clear();
    // This time is arbitrary, if the the scroll animation duration changes, this should be updated
    Render::render_animated(
        &mut buffer,
        &render_tree,
        &new_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(1550)),
    );
    assert_str_grid_eq!(
        [
            "            ",
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            "            ",
        ],
        &buffer.text
    );

    buffer.clear();
    Render::render_animated(
        &mut buffer,
        &render_tree,
        &new_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(5000)),
    );
    assert_str_grid_eq!(
        [
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
}
