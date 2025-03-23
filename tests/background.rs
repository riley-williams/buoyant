use buoyant::{
    font::CharacterBufferFont,
    layout::Alignment,
    primitives::Point,
    render::{CharacterRender as _, CharacterRenderTarget as _},
    render_target::FixedTextBuffer,
    view::{
        make_render_tree, padding::Edges, shape::Rectangle, EmptyView, HorizontalTextAlignment,
        Text, ViewExt as _,
    },
};

#[test]
fn background_inherits_foreground_size() {
    let font = CharacterBufferFont {};
    let view = Text::new("This is on\ntop", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .padding(Edges::All, 1)
        .background(Alignment::default(), || Rectangle)
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height()
        .foreground_color('-');

    let mut buffer = FixedTextBuffer::<14, 7>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " ------------ ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " -This is on- ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), " ----top----- ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " ------------ ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "              ");
}

#[test]
fn background_alignment_coverage() {
    let view_fn = |alignment: Alignment| {
        EmptyView.frame_sized(3, 3).background(alignment, || {
            Rectangle.foreground_color('X').frame_sized(1, 1)
        })
    };

    let mut buffer = FixedTextBuffer::<3, 3>::default();

    let view = view_fn(Alignment::TopLeading);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Top);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XX ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::TopTrailing);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Leading);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::default());
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "XX ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Trailing);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::BottomLeading);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "X  ");

    let view = view_fn(Alignment::Bottom);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "XX ");

    let view = view_fn(Alignment::BottomTrailing);
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "XXX");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "XXX");
}
