use buoyant::{
    font::CharacterBufferFont,
    layout::{HorizontalAlignment, Layout as _, VerticalAlignment},
    primitives::{Dimensions, Point, ProposedDimension, ProposedDimensions, Size},
    render::{CharacterRender as _, CharacterRenderTarget as _},
    render_target::FixedTextBuffer,
    view::{make_render_tree, LayoutExtensions as _, RenderExtensions as _, Text},
};
mod common;

#[test]
fn test_fixed_width() {
    let font = CharacterBufferFont {};
    let content = Text::str("123456", &font).frame().with_width(2);
    let env = common::TestEnv::default();

    assert_eq!(
        content.layout(&Size::new(1, 1).into(), &env).resolved_size,
        Dimensions::new(2, 1)
    );

    assert_eq!(
        content
            .layout(&Size::new(20, 123).into(), &env)
            .resolved_size,
        Dimensions::new(2, 3)
    );

    assert_eq!(
        content
            .layout(&Size::new(100, 1).into(), &env)
            .resolved_size,
        Dimensions::new(2, 1)
    );

    assert_eq!(
        content.layout(&Size::new(1, 6).into(), &env).resolved_size,
        Dimensions::new(2, 3)
    );
}

#[test]
fn test_fixed_height() {
    let font = CharacterBufferFont {};
    let content = Text::str("123456", &font).frame().with_height(2);
    let env = common::TestEnv::default();

    assert_eq!(
        content.layout(&Size::new(1, 1).into(), &env).resolved_size,
        Dimensions::new(1, 2)
    );
    assert_eq!(
        content
            .layout(&Size::new(20, 123).into(), &env)
            .resolved_size,
        Dimensions::new(6, 2)
    );
    assert_eq!(
        content
            .layout(&Size::new(100, 1).into(), &env)
            .resolved_size,
        Dimensions::new(6, 2)
    );
    assert_eq!(
        content.layout(&Size::new(2, 6).into(), &env).resolved_size,
        Dimensions::new(2, 2)
    );
}

#[test]
fn test_fixed_frame_compact_width_height() {
    let font = CharacterBufferFont {};
    let content = Text::str("123456", &font)
        .frame()
        .with_width(2)
        .with_height(2);
    let env = common::TestEnv::default();

    assert_eq!(
        content
            .layout(
                &ProposedDimensions {
                    width: ProposedDimension::Compact,
                    height: ProposedDimension::Compact
                },
                &env
            )
            .resolved_size,
        Dimensions::new(2, 2)
    );

    assert_eq!(
        content
            .layout(
                &ProposedDimensions {
                    width: ProposedDimension::Exact(2),
                    height: ProposedDimension::Exact(2)
                },
                &env
            )
            .resolved_size,
        Dimensions::new(2, 2)
    );

    assert_eq!(
        content
            .layout(
                &ProposedDimensions {
                    width: ProposedDimension::Exact(3),
                    height: ProposedDimension::Exact(3)
                },
                &env
            )
            .resolved_size,
        Dimensions::new(2, 2)
    );
}

#[test]
fn test_fixed_frame_infinite_width_height() {
    let font = CharacterBufferFont {};
    let content = Text::str("123456", &font)
        .frame()
        .with_width(25)
        .with_height(25);
    let env = common::TestEnv::default();

    assert_eq!(
        content
            .layout(
                &ProposedDimensions {
                    width: ProposedDimension::Infinite,
                    height: ProposedDimension::Infinite
                },
                &env
            )
            .resolved_size,
        Dimensions::new(25, 25)
    );
}

#[test]
fn test_render_frame_top_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
        .with_horizontal_alignment(HorizontalAlignment::Leading)
        .with_vertical_alignment(VerticalAlignment::Top)
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
fn test_render_frame_top_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
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
fn test_render_frame_top_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
        .with_horizontal_alignment(HorizontalAlignment::Trailing)
        .with_vertical_alignment(VerticalAlignment::Top)
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
fn test_render_frame_center_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
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
fn test_render_frame_center_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
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
fn test_render_frame_center_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
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
fn test_render_frame_bottom_leading_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
        .with_horizontal_alignment(HorizontalAlignment::Leading)
        .with_vertical_alignment(VerticalAlignment::Bottom)
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
fn test_render_frame_bottom_center_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
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
fn test_render_frame_bottom_trailing_alignment() {
    let font = CharacterBufferFont {};
    let content = Text::str("aa\nbb\ncc", &font)
        .frame()
        .with_width(6)
        .with_height(5)
        .with_horizontal_alignment(HorizontalAlignment::Trailing)
        .with_vertical_alignment(VerticalAlignment::Bottom)
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
