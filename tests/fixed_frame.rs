use buoyant::{
    environment::DefaultEnvironment,
    font::BufferCharacterFont,
    layout::{HorizontalAlignment, Layout, VerticalAlignment},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::{CharacterRenderTarget, FixedTextBuffer},
    view::{LayoutExtensions, Text},
};

#[test]
fn test_fixed_width() {
    let font = BufferCharacterFont {};
    let content = Text::str("123456", &font).frame(Some(2), None, None, None);
    let env = DefaultEnvironment::new(());

    assert_eq!(
        content.layout(Size::new(1, 1), &env).resolved_size,
        Size::new(2, 1)
    );

    assert_eq!(
        content.layout(Size::new(20, 123), &env).resolved_size,
        Size::new(2, 3)
    );

    assert_eq!(
        content.layout(Size::new(100, 1), &env).resolved_size,
        Size::new(2, 1)
    );

    assert_eq!(
        content.layout(Size::new(1, 6), &env).resolved_size,
        Size::new(2, 3)
    );
}

#[test]
fn test_fixed_height() {
    let font = BufferCharacterFont {};
    let content = Text::str("123456", &font).frame(None, Some(2), None, None);
    let env = DefaultEnvironment::new(());
    assert_eq!(
        content.layout(Size::new(1, 1), &env).resolved_size,
        Size::new(1, 2)
    );
    assert_eq!(
        content.layout(Size::new(20, 123), &env).resolved_size,
        Size::new(6, 2)
    );
    assert_eq!(
        content.layout(Size::new(100, 1), &env).resolved_size,
        Size::new(6, 2)
    );
    assert_eq!(
        content.layout(Size::new(2, 6), &env).resolved_size,
        Size::new(2, 2)
    );
}

#[test]
fn test_render_frame_top_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Leading),
        Some(VerticalAlignment::Top),
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "cc    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_top_center_alignment() {
    let font = BufferCharacterFont {};
    let content =
        Text::str("aa\nbb\ncc", &font).frame(Some(6), Some(5), None, Some(VerticalAlignment::Top));
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_top_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Trailing),
        Some(VerticalAlignment::Top),
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    cc");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_center_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Leading),
        None,
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "cc    ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_center_center_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(Some(6), Some(5), None, None);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_center_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Trailing),
        None,
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "    cc");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_frame_bottom_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Leading),
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "aa    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "bb    ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "cc    ");
}

#[test]
fn test_render_frame_bottom_center_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        None,
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  cc  ");
}

#[test]
fn test_render_frame_bottom_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::str("aa\nbb\ncc", &font).frame(
        Some(6),
        Some(5),
        Some(HorizontalAlignment::Trailing),
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "    cc");
}
