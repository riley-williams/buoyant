use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::BufferCharacterFont,
    layout::Layout,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::{CharacterRenderTarget, FixedTextBuffer},
    view::{Divider, HorizontalTextAlignment, LayoutExtensions, Rectangle, Spacer, Text, VStack},
};

#[test]
fn test_clipped_text_trails_correctly() {
    let font = BufferCharacterFont {};
    let text = VStack::new((
        Spacer::default(),
        Text::str(
            "Padding respects\nparent alignment\nshouldnt affect alignment",
            &font,
        )
        .multiline_text_alignment(HorizontalTextAlignment::Trailing)
        .padding(2),
        Divider::default(),
    ));

    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<30, 7>::default();

    let layout = text.layout(buffer.size(), &env);

    text.render(&mut buffer, &layout, Point::zero(), &env);

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

    let env = DefaultEnvironment::new(());

    assert_eq!(
        text.layout(Size::new(1, 1), &env).resolved_size,
        Size::new(14, 14)
    );
}
