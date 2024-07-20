use buoyant::{
    environment::DefaultEnvironment,
    font::BufferCharacterFont,
    layout::{HorizontalAlignment, Layout, VerticalAlignment},
    primitives::{Point, Size},
    render::Render,
    render_target::{FixedTextBuffer, RenderTarget},
    view::{Text, ViewExtensions},
};

#[test]
fn test_min() {
    let font = BufferCharacterFont {};
    let content = Text::char("123456", &font).flex_frame(Some(2), None, Some(2), None, None, None);

    let env = DefaultEnvironment::new(' ');

    assert_eq!(
        content.layout(Size::new(1, 1), &env).resolved_size,
        Size::new(2, 2)
    );

    assert_eq!(
        content.layout(Size::new(1, 123), &env).resolved_size,
        Size::new(2, 3)
    );

    assert_eq!(
        content.layout(Size::new(100, 1), &env).resolved_size,
        Size::new(6, 2)
    );

    assert_eq!(
        content.layout(Size::new(1, 6), &env).resolved_size,
        Size::new(2, 3)
    );
}

#[test]
fn test_max() {
    let font = BufferCharacterFont {};
    let content = Text::char("123456", &font).flex_frame(None, Some(2), None, Some(2), None, None);

    let env = DefaultEnvironment::new(' ');

    assert_eq!(
        content.layout(Size::new(2, 1), &env).resolved_size,
        Size::new(2, 1)
    );

    assert_eq!(
        content.layout(Size::new(1, 123), &env).resolved_size,
        Size::new(1, 2)
    );

    assert_eq!(
        content.layout(Size::new(100, 1), &env).resolved_size,
        Size::new(2, 1)
    );

    assert_eq!(
        content.layout(Size::new(1, 6), &env).resolved_size,
        Size::new(1, 2)
    );
}

#[test]
fn test_min_max() {
    let font = BufferCharacterFont {};
    let content = Text::char("xxx|xxx|xxx|xxx|abcdefg", &font).flex_frame(
        Some(2),
        Some(4),
        Some(2),
        Some(4),
        None,
        None,
    );

    let env = DefaultEnvironment::new(' ');

    assert_eq!(
        content.layout(Size::new(2, 1), &env).resolved_size,
        Size::new(2, 2)
    );

    assert_eq!(
        content.layout(Size::new(1, 123), &env).resolved_size,
        Size::new(2, 4)
    );

    assert_eq!(
        content.layout(Size::new(100, 1), &env).resolved_size,
        Size::new(4, 2)
    );

    assert_eq!(
        content.layout(Size::new(1, 6), &env).resolved_size,
        Size::new(2, 4)
    );

    assert_eq!(
        content.layout(Size::new(1000, 1000), &env).resolved_size,
        Size::new(4, 4)
    );

    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxx|  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_min_flex_frame_top_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Leading),
        Some(VerticalAlignment::Top),
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_top_center_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        None,
        Some(VerticalAlignment::Top),
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_top_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Trailing),
        Some(VerticalAlignment::Top),
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_center_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Leading),
        None,
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_center_center_alignment() {
    let font = BufferCharacterFont {};
    let content =
        Text::char("aa\nbb\ncc", &font).flex_frame(Some(6), None, Some(5), None, None, None);
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_center_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Trailing),
        None,
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_bottom_leading_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Leading),
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_bottom_center_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        None,
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(' ');
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
fn test_render_min_flex_frame_bottom_trailing_alignment() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(6),
        None,
        Some(5),
        None,
        Some(HorizontalAlignment::Trailing),
        Some(VerticalAlignment::Bottom),
    );
    let env = DefaultEnvironment::new(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "    aa");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "    bb");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "    cc");
}

#[test]
fn test_render_infinite_width_height_fills_space() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        None,
        Some(u16::MAX),
        None,
        Some(u16::MAX),
        Some(HorizontalAlignment::Center),
        Some(VerticalAlignment::Center),
    );
    let env = DefaultEnvironment::new(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    assert_eq!(layout.resolved_size, Size::new(6, 5));
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  aa  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  bb  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  cc  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_oversize_mix() {
    let font = BufferCharacterFont {};
    let content = Text::char("aa\nbb\ncc", &font).flex_frame(
        Some(8),
        Some(u16::MAX),
        Some(8),
        Some(u16::MAX),
        Some(HorizontalAlignment::Center),
        Some(VerticalAlignment::Center),
    );
    let env = DefaultEnvironment::new(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = content.layout(buffer.size(), &env);
    assert_eq!(layout.resolved_size, Size::new(8, 8));
    content.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   aa ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   bb ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   cc ");
}
