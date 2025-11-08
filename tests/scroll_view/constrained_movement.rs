use std::time::Duration;

use buoyant::{
    event::EventContext,
    primitives::Size,
    render::Render,
    render_target::FixedTextBuffer,
    view::{prelude::*, scroll_view::ScrollDirection},
};

use crate::common::{helpers, touch_move, touch_up};
use crate::{assert_str_grid_eq, common::touch_down};

fn scroll_view<T>() -> impl View<char, T> {
    ScrollView::new(VStack::new((
        Rectangle.frame().with_height(4).foreground_color('a'),
        Rectangle.frame().with_height(4).foreground_color('b'),
        Rectangle.frame().with_height(4).foreground_color('c'),
    )))
    .with_direction(ScrollDirection::Vertical)
    .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
    .padding(Edges::All, 1)
}

#[test]
fn vertical_scroll_does_not_move_horizontally() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = false;
    let view = scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(1),
        size,
    );

    tree.render(&mut buffer, &' ');

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

    view.handle_event(
        &touch_down(2, 3),
        &EventContext::new(Duration::from_secs(2)),
        &mut tree,
        &mut captures,
        &mut state,
    );

    let result = view.handle_event(
        &touch_move(20, 3),
        &EventContext::new(Duration::from_secs(3)),
        &mut tree,
        &mut captures,
        &mut state,
    );

    assert!(!result.recompute_view);

    tree.render(&mut buffer, &' ');
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

    let result = view.handle_event(
        &touch_move(-20, 2),
        &EventContext::new(Duration::from_secs(4)),
        &mut tree,
        &mut captures,
        &mut state,
    );

    // Tree manually updated, no view recomputation
    assert!(!result.recompute_view);

    tree.render(&mut buffer, &' ');
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

    let result = view.handle_event(
        &touch_up(1, 1),
        &EventContext::new(Duration::from_secs(5)),
        &mut tree,
        &mut captures,
        &mut state,
    );

    assert!(result.recompute_view);

    tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(5),
        size,
    );

    tree.render(&mut buffer, &' ');
    assert_str_grid_eq!(
        [
            "            ",
            " aaaaaaaaaa ",
            " aaaaaaaaaa ",
            " bbbbbbbbbb ",
            "            ",
        ],
        &buffer.text
    );
}
