use buoyant::{
    font::CharacterBufferFont, primitives::Point, render::Render as _,
    render_target::FixedTextBuffer, view::prelude::*,
};
mod common;
use common::make_render_tree;

#[test]
fn overlay_inherits_foreground_size() {
    let font = CharacterBufferFont {};
    let view = Text::new("This is\n!visible", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .padding(Edges::All, 1)
        .overlay(Alignment::default(), Rectangle.foreground_color('-'))
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height();

    let mut buffer = FixedTextBuffer::<14, 7>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[6].iter().collect::<String>(), "              ");
}

#[test]
fn overlay_renders_on_top() {
    let font = CharacterBufferFont {};
    // Create a base view with text
    let view = Text::new("BASE", &font)
        .padding(Edges::All, 1)
        .foreground_color('B')
        .overlay(Alignment::default(), Rectangle.foreground_color('O'));

    let mut buffer = FixedTextBuffer::<6, 3>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());

    // The overlay character 'O' should appear on top of the base content 'B'
    assert_eq!(buffer.text[0].iter().collect::<String>(), "OOOOOO");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "OOOOOO");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "OOOOOO");
}

#[test]
fn overlay_alignment_variations() {
    let view_fn = |alignment: Alignment| {
        EmptyView.frame_sized(3, 3).overlay(alignment, {
            Rectangle.foreground_color('X').frame_sized(1, 1)
        })
    };

    let mut buffer = FixedTextBuffer::<3, 3>::default();

    let view = view_fn(Alignment::TopLeading);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Top);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), " X ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::TopTrailing);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "  X");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Leading);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::default());
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " X ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Trailing);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  X");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::BottomLeading);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "X  ");

    let view = view_fn(Alignment::Bottom);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " X ");

    let view = view_fn(Alignment::BottomTrailing);
    let tree = make_render_tree(&view, buffer.size());
    buffer.clear();
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  X");
}

#[test]
fn multiple_overlay_layering() {
    let view = EmptyView
        .frame_sized(3, 3)
        .overlay(Alignment::TopLeading, {
            Rectangle.foreground_color('1').frame_sized(2, 2)
        })
        .overlay(Alignment::BottomTrailing, {
            Rectangle.foreground_color('2').frame_sized(2, 2)
        });

    let mut buffer = FixedTextBuffer::<3, 3>::default();

    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    // The second overlay should be on top where they overlap
    assert_eq!(buffer.text[0].iter().collect::<String>(), "11 ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "122");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " 22");
}

#[test]
fn overlay_offset_order() {
    // Overlay is only relative. The second item in the HStack is drawn last and should
    // be on top of the overlaied view
    let view = HStack::new((
        Rectangle
            .foreground_color('1')
            .overlay(Alignment::Trailing, {
                Rectangle
                    .foreground_color('2')
                    .frame_sized(2, 1)
                    .offset(1, 0)
            }),
        Rectangle.foreground_color('3'),
    ));

    let mut buffer = FixedTextBuffer::<4, 3>::default();

    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    // The second overlay should be on top where they overlap
    assert_eq!(buffer.text[0].iter().collect::<String>(), "1133");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "1233");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "1133");
}
