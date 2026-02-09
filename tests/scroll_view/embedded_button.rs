use std::time::Duration;

use buoyant::{
    event::EventContext,
    focus::DefaultFocus,
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
            |state| {
                Rectangle
                    .frame()
                    .with_height(4)
                    .foreground_color(if state.is_pressed() { 'A' } else { 'a' })
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

    let ctx = EventContext::new(Duration::from_secs(2));
    view.handle_event(
        &touch_down(2, 3),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );
    assert!(ctx.view_rebuild_requested.get());

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
    let ctx = EventContext::new(Duration::from_secs(3));
    view.handle_event(
        &touch_move(20, 3),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );

    assert!(ctx.view_rebuild_requested.get());

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

    let ctx = EventContext::new(Duration::from_secs(4));
    view.handle_event(
        &touch_move(2, 2),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );

    // Tree manually updated, no view recomputation
    assert!(!ctx.view_rebuild_requested.get());

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

    let ctx = EventContext::new(Duration::from_secs(5));
    view.handle_event(
        &touch_up(3, 1),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );

    assert!(ctx.view_rebuild_requested.get());

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

    let ctx = EventContext::new(Duration::from_secs(2));
    view.handle_event(
        &touch_down(2, 2),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );
    assert!(ctx.view_rebuild_requested.get());

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
    let ctx = EventContext::new(Duration::from_secs(3));
    view.handle_event(
        &touch_move(5, 3),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );
    assert!(!ctx.view_rebuild_requested.get());

    let ctx = EventContext::new(Duration::from_secs(3));
    view.handle_event(
        &touch_up(5, 3),
        &ctx,
        &mut tree,
        &mut captures,
        &mut state,
        &mut DefaultFocus::default_first(),
    );

    assert!(ctx.view_rebuild_requested.get());

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
