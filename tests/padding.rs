use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::Layout,
    primitives::{Dimensions, Size},
    render::Render,
    render_target::{FixedTextBuffer, RenderTarget, TxtColor},
    view::{
        make_render_tree, shape::Rectangle, Divider, HorizontalTextAlignment, LayoutExtensions,
        RenderExtensions, Spacer, Text, VStack,
    },
};

#[test]
fn test_clipped_text_trails_correctly() {
    let font = CharacterBufferFont {};
    let view = VStack::new((
        Spacer::default(),
        Text::str(
            "Padding respects\nparent alignment\nshouldnt affect alignment",
            &font,
        )
        .multiline_text_alignment(HorizontalTextAlignment::Trailing)
        .padding(2),
        Divider::default().foreground_color(TxtColor::new('-')),
    ));

    let mut buffer = FixedTextBuffer::<30, 7>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &TxtColor::default());

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
    let text = Rectangle.frame(Some(10), Some(10), None, None).padding(2);

    let env = DefaultEnvironment;

    assert_eq!(
        text.layout(&Size::new(1, 1).into(), &env).resolved_size,
        Dimensions::new(14, 14)
    );
}
