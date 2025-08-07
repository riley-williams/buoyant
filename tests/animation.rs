use std::time::Duration;

use buoyant::{
    animation::Animation,
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    if_view,
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::FixedTextBuffer,
    view::prelude::*,
};
mod common;
use common::make_render_tree;

const FONT: CharacterBufferFont = CharacterBufferFont;

fn x_bar(alignment: HorizontalAlignment) -> impl View<char, ()> {
    Text::new("X", &FONT)
        .flex_infinite_width(alignment)
        .animated(Animation::linear(Duration::from_secs(1)), alignment)
}

/// Repeatedly render animation of X from left to right without clearing buffer
/// Check the buffer is filled with X.
#[test]
fn sanity_animation_wipe() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size(), &mut ());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size(), &mut ());

    // render 101 steps, 10 ms increments
    for i in 0..101u64 {
        Render::<char>::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            Point::zero(),
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXXXXXXX");
}

#[test]
fn sanity_animation_wipe_leading_half() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size(), &mut ());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size(), &mut ());

    for i in 0..50u64 {
        Render::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            Point::zero(),
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXX     ");
}

#[test]
fn sanity_animation_wipe_trailing_half() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size(), &mut ());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size(), &mut ());

    // The first frame detects the changed value and sets the animation end time in
    // the target tree.
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X         ");
    buffer.clear();

    for i in 60..101u64 {
        Render::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            Point::zero(),
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "     XXXXX");
}

fn stacked_bars(alignment: HorizontalAlignment) -> impl View<char, ()> {
    VStack::new((
        Text::new("X", &FONT).animated(Animation::linear(Duration::from_secs(1)), alignment),
        Text::new("Y", &FONT),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

#[test]
fn animation_only_occurs_on_animated_subtrees() {
    let mut buffer = FixedTextBuffer::<10, 3>::default();

    let mut view = stacked_bars(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size(), &mut ());

    view = stacked_bars(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size(), &mut ());

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X         ");
    assert_eq!(
        buffer.text[1].iter().collect::<String>(),
        "         Y",
        "Y text should immediately render at the target location"
    );
    buffer.clear();

    for i in 1..101u64 {
        Render::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            Point::zero(),
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXXXXXXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "         Y");
}

/// Value is used to trigger the top text to animate.
/// Changing alignment should not trigger animation.
fn stacked_bars_value(
    x_value: u8,
    y_value: u8,
    alignment: HorizontalAlignment,
) -> impl View<char, ()> {
    VStack::new((
        Text::new("X", &FONT).animated(Animation::linear(Duration::from_secs(1)), x_value),
        Text::new("Y", &FONT).animated(Animation::linear(Duration::from_secs(2)), y_value),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

/// Even though the Y text is animated, it will never animate because the value is constant
#[test]
fn no_animation_when_value_doesnt_change() {
    let mut buffer = FixedTextBuffer::<10, 3>::default();

    let mut view = stacked_bars_value(0, 0, HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size(), &mut ());

    view = stacked_bars_value(1, 0, HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size(), &mut ());

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X         ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "         Y");
    buffer.clear();

    for i in 1..101u64 {
        Render::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            Point::zero(),
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXXXXXXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "         Y");
}

/// Value is used to trigger the top text to animate.
/// Changing alignment should not trigger animation.
fn stacked_bars_3_value(
    x_value: u8,
    y_value: u8,
    z_value: u8,
    alignment: HorizontalAlignment,
) -> impl View<char, ()> {
    VStack::new((
        Text::new("X", &FONT).animated(Animation::linear(Duration::from_secs(1)), x_value),
        Text::new("Y", &FONT).animated(Animation::linear(Duration::from_secs(2)), y_value),
        Text::new("Z", &FONT).animated(Animation::linear(Duration::from_secs(2)), z_value),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

#[test]
fn partial_animation_join() {
    let mut buffer = FixedTextBuffer::<11, 4>::default();

    let mut view = stacked_bars_3_value(0, 0, 0, HorizontalAlignment::Leading);

    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let mut state = view.build_state(&mut ());
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // change both x and y
    // don't update the env app time, so both frames are generated at the same time
    view = stacked_bars_3_value(1, 1, 1, HorizontalAlignment::Trailing);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // initial render sets target animation times
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Y          ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Z          ");
    buffer.clear();

    // first real interpolated frame, at .5s
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(550)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "     X     ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  Y        ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  Z        ");
    buffer.clear();

    // Join the views at 1s of animation
    target_tree.join_from(
        &source_tree,
        &AnimationDomain::new(255, Duration::from_millis(1050)),
    );
    source_tree = target_tree;

    // The joined view should render to the correct partial animation state
    source_tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "          X");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     Y     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     Z     ");
    buffer.clear();

    // Create a new view, only changing the y value
    // However, in the new target align leading
    env.app_time = Duration::from_millis(1050);
    view = stacked_bars_3_value(1, 2, 1, HorizontalAlignment::Leading);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // The previous y animation should continue, but x should jump because the state changed
    // without a change in value

    // No time elapsed since the join, so Y shouldn't have moved, but X jumps
    // Z value didn't change, so it should continue the old animation
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(1050)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     Y     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     Z     ");
    buffer.clear();

    // Y changed, so the animation duration is reset and it takes 2s to move from the middle
    // to the left
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(2000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  Y        ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Z          ");
    buffer.clear();

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::new(255, Duration::from_millis(3000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Y          ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Z          ");
}

/// Renders a toggle switch that animates between on and off. The subtext is only displayed if the
/// switch is on
///
///   ____#
/// subtext
///
/// and
///
/// #____
///
/// Only the toggle is animated, not the text
fn toggle_switch(is_on: bool, subtext: &str) -> impl View<char, ()> + use<'_> {
    let alignment = if is_on {
        HorizontalAlignment::Trailing
    } else {
        HorizontalAlignment::Leading
    };

    VStack::new((
        ZStack::new((
            Rectangle.foreground_color('_').frame_sized(5, 1),
            Rectangle.foreground_color('#').frame_sized(1, 1),
        ))
        .with_horizontal_alignment(alignment)
        .animated(Animation::linear(Duration::from_secs(1)), is_on),
        if_view!((is_on) {
            Text::new(subtext, &FONT).multiline_text_alignment(HorizontalTextAlignment::Trailing)
        } else {
            EmptyView
        }),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
}

fn toggle_move(is_on: bool, alignment: HorizontalAlignment) -> impl View<char, ()> {
    toggle_switch(is_on, "xxx")
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height()
        .with_vertical_alignment(VerticalAlignment::Top)
        .with_horizontal_alignment(alignment)
}

#[test]
fn jump_toggle_animation() {
    let mut buffer = FixedTextBuffer::<11, 4>::default();

    let mut view = toggle_move(false, HorizontalAlignment::Trailing);
    let mut state = view.build_state(&mut ());
    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    view = toggle_move(true, HorizontalAlignment::Trailing);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // initial render sets target animation times
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      #____");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    // first real interpolated frame, at .5s
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    target_tree.join_from(
        &source_tree,
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );
    source_tree = target_tree;

    source_tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    env.app_time = Duration::from_millis(550);
    view = toggle_move(true, HorizontalAlignment::Leading);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );

    // Toggle should continue to animate, but the subtext jumps
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  xxx      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(1000)),
    );

    // The toggle completes its animation, catching up to the subtext
    assert_eq!(buffer.text[0].iter().collect::<String>(), "____#      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  xxx      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
}

fn toggle_stack(is_on: bool) -> impl View<char, ()> {
    VStack::new((
        toggle_switch(is_on, "123456\n123"),
        toggle_switch(is_on, "xxx"),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    // this animation should do nothing, because the text is transitioned, not moved
    .animated(Animation::linear(Duration::from_millis(2000)), is_on)
    .flex_frame()
    .with_infinite_max_width()
    .with_infinite_max_height()
    .with_vertical_alignment(VerticalAlignment::Top)
    .with_horizontal_alignment(HorizontalAlignment::Trailing)
}

#[test]
fn nested_toggle_animation() {
    let mut buffer = FixedTextBuffer::<11, 5>::default();

    let mut view = toggle_stack(false);

    let mut state = view.build_state(&mut ());
    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // don't update the env app time, so both frames are generated at the same time
    view = toggle_stack(true);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    // initial render sets target animation times
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(0)),
    );
    // subtext should jump
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      #____");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     1#____"); // toggle 1 subtext is
                                                                          // overwritten by toggle 2
    assert_eq!(buffer.text[2].iter().collect::<String>(), "        123");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
    buffer.clear();

    // first real interpolated frame, at .5s
    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__"); // partially animated
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
    buffer.clear();

    // Join the views at 1s of animation
    target_tree.join_from(
        &source_tree,
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );
    source_tree = target_tree;

    // The joined view should render to the partial animation state
    source_tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
    buffer.clear();

    env.app_time = Duration::from_millis(550);
    view = toggle_stack(true);
    let layout = view.layout(&buffer.size().into(), &env, &mut (), &mut state);
    target_tree = view.render_tree(&layout, Point::default(), &env, &mut (), &mut state);

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(550)),
    );
    // again, should be the same view
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");

    buffer.clear();

    Render::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        Point::zero(),
        &AnimationDomain::top_level(Duration::from_millis(1000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ____#");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "        123");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ____#");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
}
