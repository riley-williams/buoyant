use buoyant::layout::Alignment;
use buoyant::view::padding::Edges;
use buoyant::view::shape::Rectangle;
use buoyant::{
    font::CharacterBufferFont,
    primitives::Point,
    render::Render as _,
    render_target::FixedTextBuffer,
    view::{HStack, Text, ViewExt as _},
};
mod common;
use common::make_render_tree;

#[test]
fn background_renders_on_hidden_view() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::new("1234", &font)
            .hidden()
            .background(Alignment::default(), || {
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
