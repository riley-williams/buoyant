use buoyant::view::padding::Edges;
use buoyant::view::shape::Rectangle;
use buoyant::{
    font::CharacterBufferFont,
    primitives::Point,
    render::{CharacterRender as _, CharacterRenderTarget as _},
    render_target::FixedTextBuffer,
    view::{make_render_tree, HStack, LayoutExtensions as _, RenderExtensions as _, Text},
};

#[test]
fn background_renders_on_hidden_view() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::new("1234", &font).hidden().background(|| {
            Rectangle
                .foreground_color('+')
                .padding(Edges::Horizontal, 1)
        }),
        Rectangle,
    ))
    .foreground_color('-');
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), " ++ -----");
}
