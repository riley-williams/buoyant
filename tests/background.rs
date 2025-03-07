use buoyant::{
    font::CharacterBufferFont,
    primitives::Point,
    render::{CharacterRender as _, CharacterRenderTarget as _},
    render_target::FixedTextBuffer,
    view::{
        make_render_tree, padding::Edges, shape::Rectangle, HorizontalTextAlignment,
        LayoutExtensions as _, RenderExtensions as _, Text,
    },
};

#[test]
fn background_inherits_foreground_size() {
    let font = CharacterBufferFont {};
    let view = Text::new("This is on\ntop", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .padding(Edges::All, 1)
        .background(|| Rectangle)
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
