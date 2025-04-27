use buoyant::render::Render;
use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::Layout as _,
    primitives::{Dimensions, Point},
    render::Renderable as _,
    render_target::FixedTextBuffer,
    view::{HorizontalTextAlignment, Text, ViewExt as _},
};
mod common;
use common::make_render_tree;

#[test]
fn test_render_wrapping_leading() {
    let font = CharacterBufferFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", &font).foreground_color(' ');
    make_render_tree(&text, buffer.size()).render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}

#[test]
fn test_render_wrapping_center_even() {
    let font = CharacterBufferFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .foreground_color(' ');
    make_render_tree(&text, buffer.size()).render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " here ");
}

#[test]
fn test_render_wrapping_center_odd() {
    let font = CharacterBufferFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text 12345", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .foreground_color(' ');
    make_render_tree(&text, buffer.size()).render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "12345 ");
}

#[test]
fn test_render_wrapping_trailing() {
    let font = CharacterBufferFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Trailing)
        .foreground_color(' ');
    make_render_tree(&text, buffer.size()).render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  This");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  is a");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  here");
}

#[test]
fn test_clipped_text_is_centered_correctly() {
    let font = CharacterBufferFont {};
    let view = Text::new(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Center)
    .foreground_color(' ');

    let env = DefaultEnvironment::non_animated();
    let mut buffer = FixedTextBuffer::<40, 2>::default();

    let layout = view.layout(&buffer.size().into(), &env);

    assert_eq!(layout.resolved_size, Dimensions::new(13, 2));

    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "Several lines                           ",
        "   of text                              ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_clipped_text_trails_correctly() {
    let font = CharacterBufferFont {};
    let view = Text::new(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Trailing)
    .frame()
    .with_height(2) // constrain to 2 pts tall
    .foreground_color(' ');

    let env = DefaultEnvironment::non_animated();
    let mut buffer = FixedTextBuffer::<40, 3>::default();

    let layout = view.layout(&buffer.size().into(), &env);

    assert_eq!(layout.resolved_size, Dimensions::new(13, 2));

    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());

    let lines = [
        "Several lines                           ",
        "      of text                           ",
        "                                        ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}
