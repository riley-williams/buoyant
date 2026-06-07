use buoyant::{
    app::{App, Harness as _},
    font::CharacterBufferFont,
    primitives::{Point, Size},
    render::Render as _,
    render_target::FixedTextBuffer,
    view::{
        prelude::*,
        scroll_view::{ScrollBarVisibility, ScrollDirection},
    },
};

use crate::assert_str_grid_eq;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn nested_scroll(_: &()) -> impl View<char, ()> + use<> {
    ScrollView::new(
        VStack::new((
            Text::new("1\n2", &CharacterBufferFont),
            ScrollView::new(Text::new("sideways scrolling text", &CharacterBufferFont))
                .with_direction(ScrollDirection::Horizontal)
                .with_bar_visibility(ScrollBarVisibility::Never)
                .frame()
                .with_height(2),
            Text::new("3\n4\n5\n6", &CharacterBufferFont),
        ))
        .with_alignment(HorizontalAlignment::Leading),
    )
    .with_direction(ScrollDirection::Vertical)
    .with_bar_visibility(ScrollBarVisibility::Never)
}

#[test]
fn nested_horizontal_view() {
    let state = ();
    let mut harness = App::new(state, Size::new(12, 5), nested_scroll);

    let mut buffer = FixedTextBuffer::<12, 5>::default();

    harness.finalize_view();
    harness.render_trees().target().render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "1           ",
            "2           ",
            "sideways scr",
            "            ",
            "3           ",
        ],
        &buffer.text
    );

    harness.drag(Point::new(1, 2), Point::new(-10, 2));

    harness.finalize_view();
    buffer.clear();
    harness.render_trees().target().render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "1           ",
            "2           ",
            "rolling text",
            "            ",
            "3           ",
        ],
        &buffer.text
    );

    // Diagonal scroll within the "wiggle" bounds interacts with both
    harness.drag(Point::new(1, 2), Point::new(3, 1));

    harness.finalize_view();
    buffer.clear();
    harness.render_trees().target().render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "2           ",
            "scrolling te",
            "            ",
            "3           ",
            "4           ",
        ],
        &buffer.text
    );
}
