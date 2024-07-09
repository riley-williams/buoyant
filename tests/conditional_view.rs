use buoyant::font::TerminalChar;
use buoyant::layout::Layout;
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{FixedTextBuffer, RenderTarget};
use buoyant::view::{ConditionalView, Text};
use common::TestEnv;

mod common;

#[test]
fn test_conditional_view_layout() {
    let font = TerminalChar {};
    let make_view = |condition| {
        ConditionalView::new(
            condition,
            Text::char("true\n!!!", &font),
            Text::char("f", &font),
        )
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default();

    let view = make_view(true);
    let layout = view.layout(buffer.size(), &env);
    assert_eq!(layout.resolved_size, Size::new(4, 2));
    view.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "true ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "!!!  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear();

    let view = make_view(false);
    let layout = view.layout(buffer.size(), &env);
    assert_eq!(layout.resolved_size, Size::new(1, 1));
    view.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "f    ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}
