use std::time::Duration;

use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::{HorizontalAlignment, Layout as _, VerticalAlignment},
    primitives::Point,
    render::{AnimationDomain, CharacterRender, CharacterRenderTarget, Renderable},
    render_target::FixedTextBuffer,
    view::{
        make_render_tree, shape::Rectangle, ConditionalView, Divider, EmptyView,
        HorizontalTextAlignment, LayoutExtensions, RenderExtensions as _, Text, VStack, ZStack,
    },
    Animation,
};

const FONT: CharacterBufferFont = CharacterBufferFont;

fn x_bar(
    alignment: HorizontalAlignment,
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    Text::str("X", &FONT)
        .flex_frame()
        .with_infinite_max_width()
        .with_horizontal_alignment(alignment)
        .animated(Animation::Linear(Duration::from_secs(1)), alignment)
}

/// Repeatedly render animation of X from left to right without clearing buffer
/// Check the buffer is filled with X.
#[test]
fn sanity_animation_wipe() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size());

    // render 101 steps, 10 ms increments
    for i in 0..101u64 {
        CharacterRender::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXXXXXXX");
}

#[test]
fn sanity_animation_wipe_leading_half() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size());

    for i in 0..50u64 {
        CharacterRender::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXXXX     ");
}

#[test]
fn sanity_animation_wipe_trailing_half() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();

    let mut view = x_bar(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size());

    view = x_bar(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size());

    // The first frame detects the changed value and sets the animation end time in
    // the target tree.
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X         ");
    buffer.clear();

    for i in 60..101u64 {
        CharacterRender::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
            &AnimationDomain::new(255, Duration::from_millis(i * 10)),
        );
    }
    assert_eq!(buffer.text[0].iter().collect::<String>(), "     XXXXX");
}

fn stacked_bars(
    alignment: HorizontalAlignment,
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    VStack::new((
        Text::str("X", &FONT).animated(Animation::Linear(Duration::from_secs(1)), alignment),
        Text::str("Y", &FONT),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

#[test]
fn animation_only_occurs_on_animated_subtrees() {
    let mut buffer = FixedTextBuffer::<10, 3>::default();

    let mut view = stacked_bars(HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size());

    view = stacked_bars(HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size());

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
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
        CharacterRender::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
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
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    VStack::new((
        Text::str("X", &FONT).animated(Animation::Linear(Duration::from_secs(1)), x_value),
        Text::str("Y", &FONT).animated(Animation::Linear(Duration::from_secs(2)), y_value),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

/// Even though the Y text is animated, it will never animate because the value is constant
#[test]
fn no_animation_when_value_doesnt_change() {
    let mut buffer = FixedTextBuffer::<10, 3>::default();

    let mut view = stacked_bars_value(0, 0, HorizontalAlignment::Leading);

    let source_tree = make_render_tree(&view, buffer.size());

    view = stacked_bars_value(1, 0, HorizontalAlignment::Trailing);

    let target_tree = make_render_tree(&view, buffer.size());

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X         ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "         Y");
    buffer.clear();

    for i in 1..101u64 {
        CharacterRender::render_animated(
            &mut buffer,
            &source_tree,
            &target_tree,
            &' ',
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
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    VStack::new((
        Text::str("X", &FONT).animated(Animation::Linear(Duration::from_secs(1)), x_value),
        Text::str("Y", &FONT).animated(Animation::Linear(Duration::from_secs(2)), y_value),
        Text::str("Z", &FONT).animated(Animation::Linear(Duration::from_secs(2)), z_value),
        Divider::new(1), // Ensure the stack spans the offered width
    ))
    .with_alignment(alignment)
}

#[test]
fn partial_animation_join() {
    let mut buffer = FixedTextBuffer::<11, 4>::default();

    let mut view = stacked_bars_3_value(0, 0, 0, HorizontalAlignment::Leading);

    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let layout = view.layout(&buffer.size().into(), &env);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env);

    // change both x and y
    // don't update the env app time, so both frames are generated at the same time
    view = stacked_bars_3_value(1, 1, 1, HorizontalAlignment::Trailing);
    let layout = view.layout(&buffer.size().into(), &env);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env);

    // initial render sets target animation times
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Y          ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Z          ");
    buffer.clear();

    // first real interpolated frame, at .5s
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(500)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "     X     ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  Y        ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  Z        ");
    buffer.clear();

    // Join the views at 1s of animation
    source_tree = CharacterRender::join(
        source_tree,
        target_tree,
        &AnimationDomain::new(255, Duration::from_millis(1000)),
    );

    // The joined view should render to the correct partial animamion state
    source_tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "          X");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     Y     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     Z     ");
    buffer.clear();

    // Create a new view, only changing the y value
    // However, in the new target align leading
    env.app_time = Duration::from_millis(1000);
    view = stacked_bars_3_value(1, 2, 1, HorizontalAlignment::Leading);
    let layout = view.layout(&buffer.size().into(), &env);
    target_tree = view.render_tree(&layout, Point::default(), &env);

    // The previous y animation should continue, but x should jump because the state changed
    // without a change in value

    // No time elapsed since the join, so Y shouldn't have moved, but X jumps
    // Z value didn't change, so it should continue the old animation
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(1000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     Y     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     Z     ");
    buffer.clear();

    // Y changed, so the animation duration is reset and it takes 2s to move from the middle
    // to the left
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::new(255, Duration::from_millis(2000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X          ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  Y        ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Z          ");
    buffer.clear();

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
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
fn toggle_switch(
    is_on: bool,
    subtext: &str,
) -> impl Renderable<char, Renderables: CharacterRender<char>> + use<'_> {
    let alignment = if is_on {
        HorizontalAlignment::Trailing
    } else {
        HorizontalAlignment::Leading
    };

    VStack::new((
        ZStack::new((
            Rectangle.foreground_color('_').frame(
                Some(5),
                Some(1),
                Some(HorizontalAlignment::Center),
                Some(VerticalAlignment::Center),
            ),
            Rectangle.foreground_color('#').frame(
                Some(1),
                Some(1),
                Some(HorizontalAlignment::Center),
                Some(VerticalAlignment::Center),
            ),
        ))
        .horizontal_alignment(alignment)
        .animated(Animation::Linear(Duration::from_secs(1)), is_on),
        ConditionalView::new(
            is_on,
            Text::str(subtext, &FONT).multiline_text_alignment(HorizontalTextAlignment::Trailing),
            EmptyView,
        ),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
}

fn toggle_move(
    is_on: bool,
    alignment: HorizontalAlignment,
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
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

    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let layout = view.layout(&buffer.size().into(), &env);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env);

    // change both x and y
    // don't update the env app time, so both frames are generated at the same time
    view = toggle_move(true, HorizontalAlignment::Trailing);
    let layout = view.layout(&buffer.size().into(), &env);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env);

    // initial render sets target animation times
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      #____");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    // first real interpolated frame, at .5s
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    // Join the views at 1s of animation
    source_tree = CharacterRender::join(
        source_tree,
        target_tree,
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );

    // The joined view should render to the complete animamion state
    source_tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        xxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    env.app_time = Duration::from_millis(500);
    view = toggle_move(true, HorizontalAlignment::Leading);
    let layout = view.layout(&buffer.size().into(), &env);
    target_tree = view.render_tree(&layout, Point::default(), &env);

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );

    // Toggle should continue to animate, but the subtext jumps
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  xxx      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
    buffer.clear();

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(1000)),
    );

    // The toggle completes its animation, catching up to the subtext
    assert_eq!(buffer.text[0].iter().collect::<String>(), "____#      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  xxx      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
}

fn toggle_stack(is_on: bool) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    VStack::new((
        toggle_switch(is_on, "123456\n123"),
        toggle_switch(is_on, "xxx"),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .animated(Animation::Linear(Duration::from_millis(2000)), is_on)
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

    let mut env = DefaultEnvironment::new(Duration::from_millis(0));
    let layout = view.layout(&buffer.size().into(), &env);
    let mut source_tree = view.render_tree(&layout, Point::default(), &env);

    // change both x and y
    // don't update the env app time, so both frames are generated at the same time
    view = toggle_stack(true);
    let layout = view.layout(&buffer.size().into(), &env);
    let mut target_tree = view.render_tree(&layout, Point::default(), &env);

    // initial render sets target animation times
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(0)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      #____");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     1#____"); // toggle 1 subtext is
                                                                          // overwritten by toggle 2
    assert_eq!(buffer.text[2].iter().collect::<String>(), "        123");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx"); // ???
    buffer.clear();

    // first real interpolated frame, at .5s
    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__"); // partially animated
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
    buffer.clear();

    // Join the views at 1s of animation
    source_tree = CharacterRender::join(
        source_tree,
        target_tree,
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );

    // The joined view should render to the partial animamion state
    source_tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
    buffer.clear();

    env.app_time = Duration::from_millis(500);
    view = toggle_stack(true);
    let layout = view.layout(&buffer.size().into(), &env);
    target_tree = view.render_tree(&layout, Point::default(), &env);

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(500)),
    );
    // again, should be the same view
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      __#__");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "           ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");

    buffer.clear();

    CharacterRender::render_animated(
        &mut buffer,
        &source_tree,
        &target_tree,
        &' ',
        &AnimationDomain::top_level(Duration::from_millis(1000)),
    );

    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ____#");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     123456");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "        123");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ____#");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        xxx");
}
