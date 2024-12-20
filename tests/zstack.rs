use buoyant::environment::DefaultEnvironment;
use buoyant::font::BufferCharacterFont;
use buoyant::layout::{HorizontalAlignment, Layout, VerticalAlignment};
use buoyant::primitives::{Point, Size};
use buoyant::render::CharacterRender;
use buoyant::render_target::{CharacterRenderTarget as _, FixedTextBuffer};
use buoyant::view::{Divider, LayoutExtensions, Spacer, Text, ZStack};

#[test]
fn test_layout_fills_two() {
    let stack = ZStack::two(Spacer::default(), Divider::default());
    let offer = Size::new(100, 42);
    let env = DefaultEnvironment::new(());
    let layout = stack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(100, 42));
}

#[test]
fn test_oversized_layout_2() {
    let stack = ZStack::two(Divider::default().padding(2), Spacer::default());
    let offer = Size::new(0, 10);
    let env = DefaultEnvironment::new(());
    let layout = stack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(0, 10));
}

#[test]
fn test_render_two_centered_overlap() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(Text::str("aa\nbb\ncc", &font), Text::str("test", &font));
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " aa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "test  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cc   ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_centered() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(Text::str("test", &font), Text::str("aa\nbb\ncc", &font));
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " aa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "tbbt  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cc   ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_center_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Top);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "axxxa ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_leading_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Top)
    .horizontal_alignment(HorizontalAlignment::Leading);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxx a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_trailing_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Top)
    .horizontal_alignment(HorizontalAlignment::Trailing);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a xxx ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_center_leading_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .horizontal_alignment(HorizontalAlignment::Leading);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxx b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_center_trailing_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .horizontal_alignment(HorizontalAlignment::Trailing);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b xxx ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_leading_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Bottom)
    .horizontal_alignment(HorizontalAlignment::Leading);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxx c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_center_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Bottom);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "cxxxc ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_trailing_alignment() {
    let font = BufferCharacterFont {};
    let stack = ZStack::two(
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    )
    .vertical_alignment(VerticalAlignment::Bottom)
    .horizontal_alignment(HorizontalAlignment::Trailing);
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = stack.layout(buffer.size(), &env);
    stack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c xxx ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}
