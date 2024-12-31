use buoyant::font::BufferCharacterFont;
use buoyant::layout::Layout;
use buoyant::primitives::{Point, Size};
use buoyant::render::CharacterRender;
use buoyant::render_target::{CharacterRenderTarget, FixedTextBuffer};
use buoyant::view::{ConditionalView, Text};
use common::TestEnv;

mod common;

#[test]
fn test_conditional_view_layout() {
    let font = BufferCharacterFont {};
    let make_view = |condition| {
        ConditionalView::new(
            condition,
            Text::str("true\n!!!", &font),
            Text::str("f", &font),
        )
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default();

    let view = make_view(true);
    let layout = view.layout_and_place(buffer.size(), Point::zero(), &env);
    assert_eq!(layout.resolved_size, Size::new(4, 2).into());
    view.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "true ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "!!!  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear(None);

    let view = make_view(false);
    let layout = view.layout_and_place(buffer.size(), Point::zero(), &env);
    assert_eq!(layout.resolved_size, Size::new(1, 1).into());
    view.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "f    ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}
