use core::time::Duration;

use buoyant::{
    if_view,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::FixedTextBuffer,
    transition::Move,
    view::prelude::*,
};

use crate::assert_str_grid_eq;
use crate::common::helpers::tree;

fn rect_slide<T>(is_visible: bool) -> impl View<char, T> {
    Rectangle
        .frame_sized(3, 1)
        .foreground_color('a')
        .overlay(
            Alignment::Top,
            if_view!((is_visible) {
                Rectangle
                    .transition(Move::bottom())
                    .frame_sized(1, 3)
                    .foreground_color('b')
            })
            .animated(Animation::linear(Duration::from_millis(100)), is_visible),
        )
        .flex_infinite_width(if is_visible {
            HorizontalAlignment::Leading
        } else {
            HorizontalAlignment::Trailing
        })
        .animated(Animation::linear(Duration::from_millis(200)), is_visible)
}

#[expect(clippy::too_many_lines)]
#[test]
fn move_out_and_back_in() {
    let mut buffer = FixedTextBuffer::<12, 5>::default();

    let mut captures = false;
    let mut view = rect_slide(true);
    let mut state = view.build_state(&mut captures);

    let mut source_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(0),
        buffer.size(),
    );

    view = rect_slide(true);
    let mut target_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(100),
        buffer.size(),
    );

    let domain = AnimationDomain::top_level(Duration::from_millis(100));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "aba         ",
            " b          ",
            " b          ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();

    target_tree.join_from(&source_tree, &domain);
    source_tree = target_tree;

    view = rect_slide(false);
    target_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(200),
        buffer.size(),
    );

    let domain = AnimationDomain::top_level(Duration::from_millis(200));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "aba         ",
            " b          ",
            " b          ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();

    let domain = AnimationDomain::top_level(Duration::from_millis(250));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "  aaa       ",
            " b          ",
            " b          ",
            " b          ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();

    let domain = AnimationDomain::top_level(Duration::from_millis(299));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "    aaa     ",
            "            ",
            " b          ",
            " b          ",
            " b          ",
        ],
        &buffer.text
    );
    buffer.clear();

    let domain = AnimationDomain::top_level(Duration::from_millis(300));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "    aaa     ",
            "            ",
            "            ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();

    // and back in

    target_tree.join_from(&source_tree, &domain);
    source_tree = target_tree;

    view = rect_slide(true);
    target_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_millis(300),
        buffer.size(),
    );

    let domain = AnimationDomain::top_level(Duration::from_millis(300));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "    aaa     ",
            "            ",
            "            ",
            " b          ",
            " b          ",
        ],
        &buffer.text
    );
    buffer.clear();

    let domain = AnimationDomain::top_level(Duration::from_millis(350));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            "   aaa      ",
            " b          ",
            " b          ",
            " b          ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();

    let domain = AnimationDomain::top_level(Duration::from_millis(400));
    Render::render_animated(&mut buffer, &source_tree, &target_tree, &' ', &domain);

    assert_str_grid_eq!(
        [
            " baaa       ",
            " b          ",
            " b          ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
    buffer.clear();
}
