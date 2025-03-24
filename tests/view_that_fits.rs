use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::Layout as _,
    primitives::Point,
    render::{Render as _, Renderable as _},
    render_target::FixedTextBuffer,
    view::{shape::Rectangle, FitAxis, HStack, Text, VStack, View, ViewExt, ViewThatFits},
};

fn single_variant_view() -> impl View<char> {
    ViewThatFits::new(FitAxis::Vertical, {
        Text::new("Single variant", &CharacterBufferFont)
    })
}

fn two_variant_view() -> impl View<char> {
    ViewThatFits::new(FitAxis::Vertical, {
        Text::new("This is the first variant", &CharacterBufferFont)
    })
    .or(Text::new("Second variant", &CharacterBufferFont))
}

fn three_variant_view() -> impl View<char> {
    ViewThatFits::new(FitAxis::Vertical, {
        Text::new("This is the longest variant", &CharacterBufferFont)
    })
    .or(Text::new("Medium length", &CharacterBufferFont))
    .or(Text::new("Short", &CharacterBufferFont))
}

fn four_variant() -> impl View<char> {
    ViewThatFits::new(FitAxis::Vertical, {
        Text::new("12 hours, 16 minutes, and 3 seconds", &CharacterBufferFont)
    })
    .or(Text::new("12h 16m 3s remaining", &CharacterBufferFont))
    .or(Text::new("12h 16m 3s", &CharacterBufferFont))
    .or(Text::new("~12h", &CharacterBufferFont))
}

#[test]
fn single_variant_fits() {
    let mut buffer = FixedTextBuffer::<14, 1>::default();
    let env = DefaultEnvironment::default();

    let view = single_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Single variant");
}

#[test]
fn single_variant_clipped() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    let env = DefaultEnvironment::default();

    let view = single_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Single    ");
}

#[test]
fn two_variant_first_fits() {
    let mut buffer = FixedTextBuffer::<25, 1>::default();
    let env = DefaultEnvironment::default();

    let view = two_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(
        buffer.text[0].iter().collect::<String>(),
        "This is the first variant"
    );
}

#[test]
fn two_variant_first_wraps() {
    let mut buffer = FixedTextBuffer::<13, 2>::default();
    let env = DefaultEnvironment::default();

    let view = two_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "This is the  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "first variant");
}

#[test]
fn two_variant_second_chosen() {
    let mut buffer = FixedTextBuffer::<14, 1>::default();
    let env = DefaultEnvironment::default();

    let view = two_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Second variant");
}

#[test]
fn two_variant_second_clipped() {
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    let env = DefaultEnvironment::default();

    let view = two_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Second    ");
}

// Three variant tests
#[test]
fn three_variant_first_fits() {
    let mut buffer = FixedTextBuffer::<27, 1>::default();
    let env = DefaultEnvironment::default();

    let view = three_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(
        buffer.text[0].iter().collect::<String>(),
        "This is the longest variant"
    );
}

#[test]
fn three_variant_second_chosen() {
    let mut buffer = FixedTextBuffer::<14, 1>::default();
    let env = DefaultEnvironment::default();

    let view = three_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Medium length ");
}

#[test]
fn three_variant_third_chosen() {
    let mut buffer = FixedTextBuffer::<5, 1>::default();
    let env = DefaultEnvironment::default();

    let view = three_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Short");
}

#[test]
fn three_variant_third_clipped() {
    let mut buffer = FixedTextBuffer::<3, 1>::default();
    let env = DefaultEnvironment::default();

    let view = three_variant_view();

    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "Sho");
}

#[test]
fn four_variant_vertical_first_fully_fits() {
    let mut buffer = FixedTextBuffer::<35, 3>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // Full width, first option should fit
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(
        buffer.text[0].iter().collect::<String>(),
        "12 hours, 16 minutes, and 3 seconds"
    );
    assert_eq!(
        buffer.text[1].iter().collect::<String>(),
        "                                   "
    );
    assert_eq!(
        buffer.text[2].iter().collect::<String>(),
        "                                   "
    );
}

#[test]
fn four_variant_vertical_wrapping_first() {
    let mut buffer = FixedTextBuffer::<12, 3>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "12 hours, 16");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "minutes, and");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "3 seconds   ");
}

#[test]
fn four_variant_vertical_second() {
    let mut buffer = FixedTextBuffer::<11, 3>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "12h 16m 3s ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "remaining  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "           ");
}

#[test]
fn four_variant_vertical_third() {
    let mut buffer = FixedTextBuffer::<5, 3>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "12h  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "16m  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "3s   ");
}

#[test]
fn four_variant_vertical_fourth() {
    let mut buffer = FixedTextBuffer::<6, 1>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "~12h  ");
}

#[test]
fn four_variant_vertical_fourth_clipping() {
    let mut buffer = FixedTextBuffer::<3, 1>::default();
    let env = DefaultEnvironment::default();

    let view = four_variant();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "~12");
}

fn fit_rects() -> impl View<char> {
    ViewThatFits::new(FitAxis::Both, {
        VStack::new((
            HStack::new((
                Rectangle
                    .flex_frame()
                    .with_min_width(4)
                    .foreground_color('a'),
                Rectangle
                    .flex_frame()
                    .with_min_width(2)
                    .foreground_color('b'),
            ))
            .flex_frame()
            .with_min_height(2),
            Rectangle
                .flex_frame()
                .with_min_height(2)
                .foreground_color('c'),
        ))
    })
    .or({
        VStack::new((
            Rectangle
                .flex_frame()
                .with_min_size(4, 2)
                .foreground_color('a'),
            Rectangle
                .flex_frame()
                .with_min_height(2)
                .foreground_color('c'),
        ))
    })
    .or({
        HStack::new((
            Rectangle
                .flex_frame()
                .with_min_width(4)
                .foreground_color('a'),
            Rectangle
                .flex_frame()
                .with_min_width(2)
                .foreground_color('b'),
        ))
        .flex_frame()
        .with_min_height(2)
    })
    .or({
        Rectangle
            .flex_frame()
            .with_min_size(4, 2)
            .foreground_color('a')
    })
}

#[test]
fn fit_rects_abc() {
    let mut buffer = FixedTextBuffer::<6, 6>::default();
    let env = DefaultEnvironment::default();

    let view = fit_rects();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaaabb");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaaabb");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aaaabb");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "cccccc");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "cccccc");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "cccccc");
}

#[test]
fn fit_rects_ac() {
    let mut buffer = FixedTextBuffer::<5, 4>::default();
    let env = DefaultEnvironment::default();

    let view = fit_rects();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaaaa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaaaa");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "ccccc");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "ccccc");
}

#[test]
fn fit_rects_ab() {
    let mut buffer = FixedTextBuffer::<6, 3>::default();
    let env = DefaultEnvironment::default();

    let view = fit_rects();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaaabb");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaaabb");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aaaabb");
}

#[test]
fn fit_rects_a() {
    let mut buffer = FixedTextBuffer::<5, 3>::default();
    let env = DefaultEnvironment::default();

    let view = fit_rects();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaaaa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaaaa");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aaaaa");
}

#[test]
fn fit_rects_a_clip() {
    let mut buffer = FixedTextBuffer::<2, 2>::default();
    let env = DefaultEnvironment::default();

    let view = fit_rects();

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa");
}

#[test]
fn oversized_height_ignored_for_horizontal() {
    let mut buffer = FixedTextBuffer::<4, 2>::default();
    let env = DefaultEnvironment::default();

    let view = ViewThatFits::new(FitAxis::Horizontal, {
        Rectangle
            .flex_frame()
            .with_min_size(4, 8)
            .foreground_color('a')
    })
    .or(Rectangle.foreground_color('b'));

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaaa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaaa");
}

#[test]
fn oversized_width_ignored_for_vertical() {
    let mut buffer = FixedTextBuffer::<3, 3>::default();
    let env = DefaultEnvironment::default();

    let view = ViewThatFits::new(FitAxis::Vertical, {
        Rectangle
            .flex_frame()
            .with_min_size(8, 3)
            .foreground_color('a')
    })
    .or(Rectangle.foreground_color('b'));

    // First option should fit, wrapping 3x
    let layout = view.layout(&buffer.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aaa");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aaa");
}
