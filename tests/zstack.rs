use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::primitives::{Dimensions, Point, Size};
use buoyant::render::Render;
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::prelude::*;

mod common;
use common::make_render_tree;

#[test]
fn test_layout_fills_two() {
    let stack = ZStack::new((Spacer::default(), Divider::default()));
    let offer = Size::new(100, 42);
    let env = DefaultEnvironment::non_animated();
    let mut state = stack.build_state(&mut ());
    let layout = stack.layout(&offer.into(), &env, &mut (), &mut state);
    assert_eq!(layout.resolved_size, Dimensions::new(100, 42));
}

#[test]
fn test_oversized_layout_2() {
    let stack = ZStack::new((Divider::default().padding(Edges::All, 2), Spacer::default()));
    let offer = Size::new(0, 10);
    let env = DefaultEnvironment::non_animated();
    let mut state = stack.build_state(&mut ());
    let layout = stack.layout(&offer.into(), &env, &mut (), &mut state);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 10));
}

#[test]
fn test_render_two_centered_overlap() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((Text::new("aa\nbb\ncc", &font), Text::new("test", &font)))
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), " aa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "test  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cc   ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn test_render_two_centered() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((Text::new("test", &font), Text::new("aa\nbb\ncc", &font)))
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_vertical_alignment(VerticalAlignment::Top)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_alignment(Alignment::TopLeading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_alignment(Alignment::TopTrailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_horizontal_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_horizontal_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_alignment(Alignment::BottomLeading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_vertical_alignment(VerticalAlignment::Bottom)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
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
        Text::new("a a a\nb b b\nc c c", &font),
        Text::new("xxx", &font),
    ))
    .with_alignment(Alignment::BottomTrailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a a a ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "b b b ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c xxx ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

/// When proposing compact width or height, the [`ZStack`] should first resolve child
/// dimensions with the original offer and then offer the union of the resolved
/// child sizes again.
#[test]
fn compact_proposal_offers_max_child_dimension() {
    let font = CharacterBufferFont {};
    let stack = ZStack::new((
        Rectangle.foreground_color('x'),
        Text::new("|||", &font).frame().with_height(15),
        Text::new("_\n_\n_", &font).frame().with_width(15),
    ))
    .fixed_size(true, true)
    .foreground_color('x');
    // This needs to be bigger than the magic number (10)
    let mut buffer = FixedTextBuffer::<15, 15>::default();
    let tree = make_render_tree(&stack, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(buffer.text[6].iter().collect::<String>(), "xxxxxxx_xxxxxxx");
    assert_eq!(buffer.text[7].iter().collect::<String>(), "xxxxxx|_|xxxxxx");
    assert_eq!(buffer.text[8].iter().collect::<String>(), "xxxxxxx_xxxxxxx");
    assert_eq!(buffer.text[9].iter().collect::<String>(), "xxxxxxxxxxxxxxx");
    assert_eq!(
        buffer.text[10].iter().collect::<String>(),
        "xxxxxxxxxxxxxxx"
    );
    assert_eq!(
        buffer.text[11].iter().collect::<String>(),
        "xxxxxxxxxxxxxxx"
    );
    assert_eq!(
        buffer.text[12].iter().collect::<String>(),
        "xxxxxxxxxxxxxxx"
    );
    assert_eq!(
        buffer.text[13].iter().collect::<String>(),
        "xxxxxxxxxxxxxxx"
    );
    assert_eq!(
        buffer.text[14].iter().collect::<String>(),
        "xxxxxxxxxxxxxxx"
    );
}
