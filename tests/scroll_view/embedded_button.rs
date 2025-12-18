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

fn scroll_view() -> impl View<char, u8> {
    ScrollView::new(VStack::new((
        Button::new(
            |i: &mut u8| *i += 1,
            |a| {
                Rectangle
                    .frame()
                    .with_height(4)
                    .foreground_color(if a.is_pressed() { 'A' } else { 'a' })
            },
        ),
        Rectangle.frame().with_height(4).foreground_color('b'),
        Rectangle.frame().with_height(4).foreground_color('c'),
    )))
    .with_direction(ScrollDirection::Vertical)
    .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never)
    .padding(Edges::All, 1)
}

#[expect(clippy::too_many_lines)]
#[test]
fn button_action_cancelled_by_scroll() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = 0;
    let view = scroll_view();
    let input = buoyant::event::input::Input::new();
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

    let result = view.handle_event(
        &touch_down(2, 3),
        &EventContext::new(Duration::from_secs(2), &input),
        &mut tree,
        &mut captures,
        &mut state,
    );
    assert!(result.recompute_view);

    tree = helpers::tree(
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
            " AAAAAAAAAA ",
            " AAAAAAAAAA ",
            " AAAAAAAAAA ",
            "            ",
        ],
        &buffer.text
    );

    // cancel touch by moving touch
    let result = view.handle_event(
        &touch_move(20, 3),
        &EventContext::new(Duration::from_secs(3), &input),
        &mut tree,
        &mut captures,
        &mut state,
    );

    assert!(result.recompute_view);

    tree = helpers::tree(
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

    let result = view.handle_event(
        &touch_move(2, 2),
        &EventContext::new(Duration::from_secs(4), &input),
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
        &touch_up(3, 1),
        &EventContext::new(Duration::from_secs(5), &input),
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

    assert_eq!(captures, 0, "Button action should not have been called");
}

#[test]
fn button_can_be_pressed_with_tiny_wiggle() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();
    let size = Size::new(12, 5);

    let mut captures = 0;
    let view = scroll_view();
    let input = buoyant::event::input::Input::new();
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

    let result = view.handle_event(
        &touch_down(2, 2),
        &EventContext::new(Duration::from_secs(2), &input),
        &mut tree,
        &mut captures,
        &mut state,
    );
    assert!(result.recompute_view);

    tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(2),
        size,
    );

    tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "            ",
            " AAAAAAAAAA ",
            " AAAAAAAAAA ",
            " AAAAAAAAAA ",
            "            ",
        ],
        &buffer.text
    );

    // little wiggle
    let result = view.handle_event(
        &touch_move(5, 3),
        &EventContext::new(Duration::from_secs(3), &input),
        &mut tree,
        &mut captures,
        &mut state,
    );
    assert!(!result.recompute_view);

    let result = view.handle_event(
        &touch_up(5, 3),
        &EventContext::new(Duration::from_secs(3), &input),
        &mut tree,
        &mut captures,
        &mut state,
    );

    assert!(result.recompute_view);

    tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(3),
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

    assert_eq!(captures, 1);
}
