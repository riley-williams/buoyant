use std::iter::zip;

use buoyant::primitives::Point;
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget;
use buoyant::view::padding::Edges;
use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::Layout,
    primitives::{Dimensions, Size},
    render_target::FixedTextBuffer,
    view::{
        make_render_tree, shape::Rectangle, Divider, HorizontalTextAlignment, Spacer, Text, VStack,
        ViewExt,
    },
};

#[test]
fn test_clipped_text_trails_correctly() {
    let font = CharacterBufferFont {};
    let view = VStack::new((
        Spacer::default(),
        Text::new(
            "Padding respects\nparent alignment\nshouldnt affect alignment",
            &font,
        )
        .multiline_text_alignment(HorizontalTextAlignment::Trailing)
        .padding(Edges::All, 2),
        Divider::default().foreground_color('-'),
    ));

    let mut buffer = FixedTextBuffer::<30, 7>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "                              ",
        "                              ",
        "       Padding respects       ",
        "       parent alignment       ",
        "                              ",
        "                              ",
        "------------------------------",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_is_oversized_for_oversized_child() {
    let view = Rectangle.frame_sized(10, 10).padding(Edges::All, 2);

    let env = DefaultEnvironment::non_animated();

    assert_eq!(
        view.layout(&Size::new(1, 1).into(), &env).resolved_size,
        Dimensions::new(14, 14)
    );
}

#[test]
fn test_zero_padding_has_no_effect() {
    let view = Rectangle.foreground_color('X').padding(Edges::All, 0);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_all() {
    let view = Rectangle.foreground_color('X').padding(Edges::All, 2);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "                    ",
        "                    ",
        "  XXXXXXXXXXXXXXXX  ",
        "                    ",
        "                    ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_horizontal() {
    let view = Rectangle
        .foreground_color('X')
        .padding(Edges::Horizontal, 3);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "   XXXXXXXXXXXXXX   ",
        "   XXXXXXXXXXXXXX   ",
        "   XXXXXXXXXXXXXX   ",
        "   XXXXXXXXXXXXXX   ",
        "   XXXXXXXXXXXXXX   ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_vertical() {
    let view = Rectangle.foreground_color('X').padding(Edges::Vertical, 3);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "                    ",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_top() {
    let view = Rectangle.foreground_color('X').padding(Edges::Top, 2);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "                    ",
        "                    ",
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXXXXXXX",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_bottom() {
    let view = Rectangle.foreground_color('X').padding(Edges::Bottom, 4);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "XXXXXXXXXXXXXXXXXXXX",
        "                    ",
        "                    ",
        "                    ",
        "                    ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_leading() {
    let view = Rectangle.foreground_color('X').padding(Edges::Leading, 5);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "     XXXXXXXXXXXXXXX",
        "     XXXXXXXXXXXXXXX",
        "     XXXXXXXXXXXXXXX",
        "     XXXXXXXXXXXXXXX",
        "     XXXXXXXXXXXXXXX",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_padding_trailing() {
    let view = Rectangle.foreground_color('X').padding(Edges::Trailing, 1);

    let mut buffer = FixedTextBuffer::<20, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "XXXXXXXXXXXXXXXXXXX ",
        "XXXXXXXXXXXXXXXXXXX ",
        "XXXXXXXXXXXXXXXXXXX ",
        "XXXXXXXXXXXXXXXXXXX ",
        "XXXXXXXXXXXXXXXXXXX ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}
