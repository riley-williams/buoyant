use buoyant::font::BufferCharacterFont;
use buoyant::layout::{Layout, LayoutDirection};
use buoyant::primitives::{Point, Size};
use buoyant::render::CharacterRender;
use buoyant::render_target::{CharacterRenderTarget, FixedTextBuffer};
use buoyant::view::{HStack, Spacer, Text};
use common::TestEnv;

mod common;

#[test]
fn test_horizontal_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv::colorless().with_direction(LayoutDirection::Horizontal);
    let layout = spacer.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 0));
}

#[test]
fn test_vertical_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv::colorless().with_direction(LayoutDirection::Vertical);
    let layout = spacer.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(0, 10));
}

#[test]
fn test_render_fills_stack() {
    let font = BufferCharacterFont {};
    let hstack = HStack::new((Spacer::default(), Text::str("67", &font))).with_spacing(1);
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "       67");
}
