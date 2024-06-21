use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::TerminalChar,
    layout::Layout,
    render::Render,
    render_target::{FixedTextBuffer, RenderTarget},
    view::{Divider, HorizontalTextAlignment, Padding, Spacer, Text, VStack},
};

#[test]
fn test_clipped_text_trails_correctly() {
    let font = TerminalChar {};
    let text = VStack::three(
        Spacer::default(),
        Padding::new(
            2,
            Text::char(
                "Padding respects\nparent alignment\nshouldnt affect alignment",
                &font,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Trailing),
        ),
        Divider::default(),
    );

    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<30, 7>::default();

    let layout = text.layout(buffer.size(), &env);

    text.render(&mut buffer, &layout, &env);

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
