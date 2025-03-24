use buoyant::layout::Alignment;
use buoyant::primitives::Point;
use buoyant::render::Render;
use buoyant::view::HStack;
use buoyant::view::Spacer;
use buoyant::view::VStack;
use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::{HorizontalAlignment, Layout, VerticalAlignment},
    primitives::{ProposedDimension, ProposedDimensions, Size},
    render_target::FixedTextBuffer,
    view::{make_render_tree, shape::Rectangle, Text, ViewExt},
};

#[test]
fn test_min() {
    let font = CharacterBufferFont {};
    let content = Text::new("123456", &font).flex_frame().with_min_size(2, 2);

    let env = DefaultEnvironment::non_animated();

    assert_eq!(
        content.layout(&Size::new(1, 1).into(), &env).resolved_size,
        Size::new(2, 2).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(1, 123).into(), &env)
            .resolved_size,
        Size::new(2, 3).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(100, 1).into(), &env)
            .resolved_size,
        Size::new(6, 2).into()
    );

    assert_eq!(
        content.layout(&Size::new(1, 6).into(), &env).resolved_size,
        Size::new(2, 3).into()
    );
}

#[test]
fn test_max() {
    let font = CharacterBufferFont {};
    let content = Text::new("123456", &font).flex_frame().with_max_size(2, 2);

    let env = DefaultEnvironment::non_animated();

    assert_eq!(
        content.layout(&Size::new(2, 1).into(), &env).resolved_size,
        Size::new(2, 1).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(1, 123).into(), &env)
            .resolved_size,
        Size::new(1, 2).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(100, 1).into(), &env)
            .resolved_size,
        Size::new(2, 1).into()
    );

    assert_eq!(
        content.layout(&Size::new(1, 6).into(), &env).resolved_size,
        Size::new(1, 2).into()
    );
}

#[test]
fn test_min_max() {
    let font = CharacterBufferFont {};
    let content = Text::new("xxx|xxx|xxx|xxx|abcdefg", &font)
        .flex_frame()
        .with_min_width(2)
        .with_max_width(4)
        .with_min_height(2)
        .with_max_height(4)
        .foreground_color(' ');

    let env = DefaultEnvironment::non_animated();

    assert_eq!(
        content.layout(&Size::new(2, 1).into(), &env).resolved_size,
        Size::new(2, 2).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(1, 123).into(), &env)
            .resolved_size,
        Size::new(2, 4).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(100, 1).into(), &env)
            .resolved_size,
        Size::new(4, 2).into()
    );

    assert_eq!(
        content.layout(&Size::new(1, 6).into(), &env).resolved_size,
        Size::new(2, 4).into()
    );

    assert_eq!(
        content
            .layout(&Size::new(1000, 1000).into(), &env)
            .resolved_size,
        Size::new(4, 4).into()
    );

    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_top_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_alignment(Alignment::TopLeading)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "cc    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_top_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_vertical_alignment(VerticalAlignment::Top)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_top_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_alignment(Alignment::TopTrailing)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    cc");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_center_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_horizontal_alignment(HorizontalAlignment::Leading)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "cc    ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_center_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_center_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_horizontal_alignment(HorizontalAlignment::Trailing)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "    cc");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_bottom_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_alignment(Alignment::BottomLeading)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "cc    ");
}

#[test]
fn test_render_min_flex_frame_bottom_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_vertical_alignment(VerticalAlignment::Bottom)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  cc  ");
}

#[test]
fn test_render_min_flex_frame_bottom_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_size(6, 5)
        .with_alignment(Alignment::BottomTrailing)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "    cc");
}

#[test]
fn test_render_infinite_width_height_fills_space() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height()
        .with_horizontal_alignment(HorizontalAlignment::Center)
        .with_vertical_alignment(VerticalAlignment::Center)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_oversize_mix() {
    let font = CharacterBufferFont {};
    let content = Text::new("aa\nbb\ncc", &font)
        .flex_frame()
        .with_min_width(8)
        .with_max_width(u16::MAX)
        .with_min_height(8)
        .with_max_height(u16::MAX)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   aa ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   bb ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   cc ");
}

#[test]
fn test_compact() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_ideal_size(8, 4)
        .with_min_size(2, 2);

    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Compact,
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 8.into());
    assert_eq!(layout.resolved_size.height, 4.into());
}

#[test]
fn test_infinite() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_min_width(2).with_min_height(2);

    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    // Rectangle defaults to a 10x10 size if not constrained.
    assert!(layout.resolved_size.width.is_infinite());
    assert!(layout.resolved_size.height.is_infinite());
}

#[test]
fn test_infinite_width_only() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_min_width(4); // no ideal or max
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Exact(6),
        },
        &env,
    );
    assert!(layout.resolved_size.width.is_infinite());
    assert_eq!(layout.resolved_size.height, 6.into());
}

#[test]
fn test_infinite_width_with_min_ideal() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_min_width(2).with_ideal_width(8);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Compact,
        },
        &env,
    );
    // Rectangle's infinite offer leads to child returning infinite, then min ensures at least 2.
    assert!(layout.resolved_size.width.is_infinite());
    // For height with Compact, child's default is 1, so min => 1
    assert_eq!(layout.resolved_size.height, 1.into());

    let layout2 = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Exact(2),
        },
        &env,
    );
    assert!(layout2.resolved_size.width.is_infinite());
    assert_eq!(layout2.resolved_size.height, 2.into());
}

#[test]
fn test_infinite_height_only() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_min_height(5);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(7),
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 7.into());
    assert!(layout.resolved_size.height.is_infinite());

    let layout2 = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    assert_eq!(layout2.resolved_size.width, 5.into());
    assert!(layout2.resolved_size.height.is_infinite());
}

#[test]
fn test_infinite_height_with_min_ideal() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_min_height(2)
        .with_ideal_height(4);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    // For height = infinite, child returns infinite; flex frame ensures at least 2 => u16::MAX
    assert!(layout.resolved_size.height.is_infinite());
    // For width with Compact, child's default is 1, so min => 1
    assert_eq!(layout.resolved_size.width, 1.into());

    let layout2 = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(3),
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    assert_eq!(layout2.resolved_size.width, 3.into());
    assert!(layout2.resolved_size.height.is_infinite());
}

#[test]
fn test_min_greater_than_ideal_height() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_min_height(10)
        .with_ideal_height(5);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(8),
            height: ProposedDimension::Compact,
        },
        &env,
    );
    // min is bigger than ideal, so 10 should be used
    assert_eq!(layout.resolved_size.height, 10.into());
}

#[test]
fn test_max_smaller_than_min_ideal_height() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_min_height(4)
        .with_ideal_height(6)
        .with_max_height(3);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    // max is 3, but min/ideal are 4/6; when max is smaller, the paradox is resolved by using min
    assert_eq!(layout.resolved_size.height, 4.into());
}

#[test]
fn test_min_greater_than_ideal_width() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_min_width(12)
        .with_ideal_width(6);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    // min is bigger than ideal, so 12 should be used
    assert_eq!(layout.resolved_size.width, 12.into());
}

#[test]
fn test_max_smaller_than_min_ideal_width() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_min_width(5)
        .with_ideal_width(8)
        .with_max_width(3);
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Compact,
        },
        &env,
    );
    // max is 3, but min/ideal are 5/8; when max is smaller, the paradox is resolved by using min
    assert_eq!(layout.resolved_size.width, 5.into());
}

#[test]
fn test_infinite_max_width() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_infinite_max_width();

    // With Infinite offer
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    assert!(layout.resolved_size.width.is_infinite());
    assert_eq!(layout.resolved_size.height, 5.into());

    // With Exact offer
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(15),
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 15.into());
    assert_eq!(layout.resolved_size.height, 5.into());

    // With Compact offer and no constraints
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 1.into()); // Default magic value
    assert_eq!(layout.resolved_size.height, 5.into());
}

#[test]
fn test_infinite_max_width_with_min_ideal() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_infinite_max_width()
        .with_min_width(5)
        .with_ideal_width(8);

    // With Compact offer, should use ideal
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 8.into());
    assert_eq!(layout.resolved_size.height, 5.into());

    // With Exact offer below min
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(3),
            height: ProposedDimension::Exact(5),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into()); // Uses min
    assert_eq!(layout.resolved_size.height, 5.into());
}

#[test]
fn test_infinite_max_height() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle.flex_frame().with_infinite_max_height();

    // With Infinite offer
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Infinite,
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into());
    assert!(layout.resolved_size.height.is_infinite());

    // With Exact offer
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Exact(15),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into());
    assert_eq!(layout.resolved_size.height, 15.into());

    // With Compact offer and no constraints
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Compact,
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into());
    assert_eq!(layout.resolved_size.height, 1.into()); // Default magic value
}

#[test]
fn test_infinite_max_height_with_min_ideal() {
    let env = DefaultEnvironment::non_animated();
    let content = Rectangle
        .flex_frame()
        .with_infinite_max_height()
        .with_min_height(5)
        .with_ideal_height(8);

    // With Compact offer, should use ideal
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Compact,
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into());
    assert_eq!(layout.resolved_size.height, 8.into());

    // With Exact offer below min
    let layout = content.layout(
        &ProposedDimensions {
            width: ProposedDimension::Exact(5),
            height: ProposedDimension::Exact(3),
        },
        &env,
    );
    assert_eq!(layout.resolved_size.width, 5.into());
    assert_eq!(layout.resolved_size.height, 5.into()); // Uses min
}

/// Usage of ``flex_infinite_width`` should be equivalent to using ``HStack`` with ``Spacer``
#[test]
fn infinite_max_width_equivalent_to_hstack_spacer() {
    let font = CharacterBufferFont {};
    let stack_view = VStack::new((
        Text::new("aa", &font),
        HStack::new((Spacer::default(), Text::new("bb", &font))),
        HStack::new((Text::new("cc", &font), Spacer::default())),
    ))
    .foreground_color(' ');

    let flex_view = VStack::new((
        Text::new("aa", &font).flex_infinite_width(HorizontalAlignment::Center),
        Text::new("bb", &font).flex_infinite_width(HorizontalAlignment::Trailing),
        Text::new("cc", &font).flex_infinite_width(HorizontalAlignment::Leading),
    ))
    .foreground_color(' ');
    let mut buffer1 = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack_view, buffer1.size());
    tree.render(&mut buffer1, &' ', Point::zero());

    let mut buffer2 = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&flex_view, buffer2.size());
    tree.render(&mut buffer2, &' ', Point::zero());

    assert_eq!(buffer1, buffer2);
}

/// Usage of ``flex_infinite_height`` should be equivalent to using ``VStack`` with ``Spacer``
#[test]
fn infinite_max_height_equivalent_to_vstack_spacer() {
    let font = CharacterBufferFont {};
    let stack_view = HStack::new((
        Text::new("aa", &font),
        VStack::new((Spacer::default(), Text::new("bb", &font))),
        VStack::new((Text::new("cc", &font), Spacer::default())),
    ))
    .foreground_color(' ');

    let flex_view = HStack::new((
        Text::new("aa", &font).flex_infinite_height(VerticalAlignment::Center),
        Text::new("bb", &font).flex_infinite_height(VerticalAlignment::Bottom),
        Text::new("cc", &font).flex_infinite_height(VerticalAlignment::Top),
    ))
    .foreground_color(' ');
    let mut buffer1 = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack_view, buffer1.size());
    tree.render(&mut buffer1, &' ', Point::zero());

    let mut buffer2 = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&flex_view, buffer2.size());
    tree.render(&mut buffer2, &' ', Point::zero());

    assert_eq!(buffer1, buffer2);
}
