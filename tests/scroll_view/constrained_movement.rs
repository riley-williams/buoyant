use std::time::Duration;

use buoyant::{
    event::{Event, EventContext},
    primitives::{Point, Size},
    render::Render,
    render_target::FixedTextBuffer,
    view::{prelude::*, scroll_view::ScrollDirection},
};

use super::helpers::tree;

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

    let mut tree = tree(&view, &mut captures, &mut state, Duration::default(), size);

    tree.render(&mut buffer, &' ', Point::zero());

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
        &Event::TouchDown(Point::new(2, 3)),
        &EventContext::new(Duration::ZERO),
        &mut tree,
        &mut captures,
        &mut state,
    );

    view.handle_event(
        &Event::TouchMoved(Point::new(20, 3)),
        &EventContext::new(Duration::ZERO),
        &mut tree,
        &mut captures,
        &mut state,
    );

    tree.render(&mut buffer, &' ', Point::zero());
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
        &Event::TouchMoved(Point::new(-20, 2)),
        &EventContext::new(Duration::ZERO),
        &mut tree,
        &mut captures,
        &mut state,
    );

    tree.render(&mut buffer, &' ', Point::zero());
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
        &Event::TouchUp(Point::new(1, 1)),
        &EventContext::new(Duration::ZERO),
        &mut tree,
        &mut captures,
        &mut state,
    );

    tree.render(&mut buffer, &' ', Point::zero());
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
