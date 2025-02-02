use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::layout::{HorizontalAlignment, Layout, VerticalAlignment};
use buoyant::primitives::{Dimensions, Size};
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget;
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{
    make_render_tree, Divider, LayoutExtensions, RenderExtensions as _, Spacer, Text, ZStack,
};

#[test]
fn test_layout_fills_two() {
    let stack = ZStack::new((Spacer::default(), Divider::default()));
    let offer = Size::new(100, 42);
    let env = DefaultEnvironment::non_animated();
    let layout = stack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(100, 42));
}

#[test]
fn test_oversized_layout_2() {
    let stack = ZStack::new((Divider::default().padding(2), Spacer::default()));
    let offer = Size::new(0, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = stack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 10));
}

#[test]
fn test_render_two_centered_overlap() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((Text::str("aa\nbb\ncc", &font), Text::str("test", &font)))
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), " aa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "test  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cc   ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_centered() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((Text::str("test", &font), Text::str("aa\nbb\ncc", &font)))
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), " aa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "tbbt  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cc   ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_center_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Top)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "axxxa ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_leading_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Top)
    .horizontal_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxx a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_top_trailing_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Top)
    .horizontal_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a xxx ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_center_leading_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .horizontal_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxx b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_center_trailing_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .horizontal_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b xxx ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c c c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_leading_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Bottom)
    .horizontal_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxx c ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_center_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Bottom)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "cxxxc ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_bottom_trailing_alignment() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Text::str("a a a\nb b b\nc c c", &font),
        Text::str("xxx", &font),
    ))
    .vertical_alignment(VerticalAlignment::Bottom)
    .horizontal_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c xxx ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}
