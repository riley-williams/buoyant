mod common;
use crate::common::helpers::tree;
use crate::common::{touch_down, touch_move};

use buoyant::{
    environment::DefaultEnvironment,
    event::EventContext,
    primitives::{Dimensions, Point, ProposedDimensions, Size},
    render::Render,
    render_target::FixedTextBuffer,
    view::{button::ButtonState, prelude::*},
};
use core::time::Duration;

#[expect(clippy::cast_precision_loss)]
fn progress_bar<T>(progress: f32) -> impl View<char, T> {
    GeometryReader::new(move |size| {
        Rectangle
            .frame_sized(size.width, size.height)
            .foreground_color('_')
            .overlay(
                Alignment::Leading,
                Rectangle
                    .foreground_color('=')
                    .frame()
                    .with_width((size.width as f32 * progress) as u32),
            )
    })
    .flex_frame()
    .with_ideal_height(2)
}

#[test]
fn progress_bar_integration_0() {
    let mut buffer = FixedTextBuffer::<18, 3>::default();

    let mut captures = false;
    let view = progress_bar(0.0);
    let mut state = view.build_state(&mut captures);

    tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    )
    .render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "__________________",
            "__________________",
            "__________________",
        ],
        &buffer.text
    );
}

#[test]
fn progress_bar_integration_50() {
    let mut buffer = FixedTextBuffer::<18, 3>::default();

    let mut captures = false;
    let view = progress_bar(0.5);
    let mut state = view.build_state(&mut captures);

    tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    )
    .render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "=========_________",
            "=========_________",
            "=========_________",
        ],
        &buffer.text
    );
}

#[test]
fn progress_bar_integration_100() {
    let mut buffer = FixedTextBuffer::<18, 3>::default();

    let mut captures = false;
    let view = progress_bar(1.0);
    let mut state = view.build_state(&mut captures);

    tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    )
    .render(&mut buffer, &' ');

    assert_str_grid_eq!(
        [
            "==================",
            "==================",
            "==================",
        ],
        &buffer.text
    );
}

#[test]
fn compact_dimensions() {
    let view = progress_bar(1.0);
    let mut state = view.build_state(&mut ());

    let env = DefaultEnvironment::default();
    let layout = view.layout(&ProposedDimensions::compact(), &env, &mut (), &mut state);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 2));
}

/// The inner view exceeds the size of the `GeometryReader`, which should not clip
fn overdraw<T>() -> impl View<char, T> {
    GeometryReader::new(move |_size| Rectangle.frame_sized(3, 3))
}

#[test]
fn overdraw_top_leading_aligned() {
    let mut buffer = FixedTextBuffer::<18, 4>::default();

    let mut captures = false;
    let view = overdraw();
    let mut state = view.build_state(&mut captures);

    tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        ProposedDimensions::compact(),
    )
    .render(&mut buffer, &'+');

    assert_str_grid_eq!(
        [
            "+++               ",
            "+++               ",
            "+++               ",
            "                  ",
        ],
        &buffer.text
    );
}

#[test]
fn undersized_top_leading_aligned() {
    let mut buffer = FixedTextBuffer::<18, 4>::default();

    let mut captures = false;
    let view = overdraw();
    let mut state = view.build_state(&mut captures);

    tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    )
    .render(&mut buffer, &'+');

    assert_str_grid_eq!(
        [
            "+++               ",
            "+++               ",
            "+++               ",
            "                  ",
        ],
        &buffer.text
    );
}

#[test]
fn preserves_inner_state() {
    let button_geometry = || {
        GeometryReader::new(|size: Size| {
            Button::new(
                |(): &mut ()| {},
                move |is_pressed: bool| {
                    Rectangle
                        .foreground_color(if is_pressed { 'x' } else { '-' })
                        .frame_sized(size.width / 2, size.height / 2)
                },
            )
        })
        .frame()
        .with_width(6)
    };

    let mut buffer = FixedTextBuffer::<12, 4>::default();

    let mut captures = ();
    let view = button_geometry();
    let mut state = view.build_state(&mut captures);
    assert_eq!(state, None);

    let mut render_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    );
    render_tree.render(&mut buffer, &' ');

    assert_eq!(state, Some((ButtonState::AtRest, ())));
    assert_str_grid_eq!(
        [
            "---         ",
            "---         ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    view.handle_event(
        &touch_down(Point::new(1, 1)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert_eq!(state, Some((ButtonState::CaptivePressed(0), ())));

    render_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    );
    render_tree.render(&mut buffer, &' ');

    assert_eq!(state, Some((ButtonState::CaptivePressed(0), ())));
    assert_str_grid_eq!(
        [
            "xxx         ",
            "xxx         ",
            "            ",
            "            ",
        ],
        &buffer.text
    );

    view.handle_event(
        &touch_move(Point::new(2, 20)),
        &EventContext::new(Duration::ZERO),
        &mut render_tree,
        &mut captures,
        &mut state,
    );
    assert_eq!(state, Some((ButtonState::Captive(0), ())));

    render_tree = tree(
        &view,
        &mut captures,
        &mut state,
        Duration::default(),
        buffer.size(),
    );
    render_tree.render(&mut buffer, &' ');

    assert_eq!(state, Some((ButtonState::Captive(0), ())));
    assert_str_grid_eq!(
        [
            "---         ",
            "---         ",
            "            ",
            "            ",
        ],
        &buffer.text
    );
}
