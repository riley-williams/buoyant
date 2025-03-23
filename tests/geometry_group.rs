use buoyant::{
    font::CharacterBufferFont,
    primitives::Point,
    render::{CharacterRender as _, CharacterRenderTarget as _},
    render_target::FixedTextBuffer,
    view::{make_render_tree, Text, VStack, ViewExt as _},
};

#[test]
fn test_geometry_group_retains_text_offset() {
    let font = CharacterBufferFont {};
    let content = VStack::new((
        Text::new("aa aa", &font).foreground_color(' '),
        Text::new("bb", &font).geometry_group(),
        Text::new("ccc", &font),
    ));
    let mut buffer = FixedTextBuffer::<6, 4>::default();
    let tree = make_render_tree(&content, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa aa ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " bb   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " ccc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
}
