use buoyant::font::CharacterBufferFont;
use buoyant::if_view;
use buoyant::layout::Layout;
use buoyant::primitives::{Point, Size};
use buoyant::render::CharacterRenderTarget;
use buoyant::render::{CharacterRender as _, Renderable as _};
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{Text, ViewExt as _};
use common::TestEnv;

mod common;

#[test]
fn test_conditional_view_layout() {
    let font = CharacterBufferFont;
    let make_view = |condition| {
        if_view!((condition) {
            Text::new("true\n!!!", &font)
        } else {
            Text::new("f", &font).foreground_color(' ')
        })
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default();

    let view = make_view(true);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(4, 2).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &env.foreground_color, Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "true ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "!!!  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear();

    let view = make_view(false);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(1, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &env.foreground_color, Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "f    ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}

#[test]
fn one_arm_if() {
    let font = CharacterBufferFont;
    let make_view = |condition| {
        if_view!((condition) {
            Text::new("true\n!!!", &font).foreground_color(' ')
        })
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default();

    let view = make_view(true);
    assert!(!view.is_empty());
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(4, 2).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &env.foreground_color, Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "true ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "!!!  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear();

    let view = make_view(false);
    assert!(view.is_empty());
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(0, 0).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &env.foreground_color, Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}
