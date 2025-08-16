use buoyant::layout::LayoutDirection;
use buoyant::primitives::{Point, Size};
use buoyant::render::Render;
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::prelude::*;

mod common;
use common::TestEnv;

#[test]
fn test_horizontal_layout() {
    let divider = Divider::new(2);
    let offer = Size::new(100, 100).into();
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = divider.layout(&offer, &env, &mut (), &mut ());
    assert_eq!(layout.resolved_size, Size::new(2, 100).into());
}

#[test]
fn test_vertical_layout() {
    let divider = Divider::new(2);
    let offer = Size::new(100, 100).into();
    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = divider.layout(&offer, &env, &mut (), &mut ());
    assert_eq!(layout.resolved_size, Size::new(100, 2).into());
}

#[test]
fn test_horizontal_render() {
    let divider = Divider::new(1).foreground_color('|');
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = divider.layout(&buffer.size().into(), &env, &mut (), &mut ());
    let tree = divider.render_tree(&layout, Point::new(0, 0), &env, &mut (), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0][0], '|');
    assert_eq!(buffer.text[4][0], '|');
    assert_eq!(buffer.text[0][1], ' ');
}

#[test]
fn test_vertical_render() {
    let divider = Divider::new(1).foreground_color('-');
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = divider.layout(&buffer.size().into(), &env, &mut (), &mut ());
    let tree = divider.render_tree(&layout, Point::new(0, 0), &env, &mut (), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0][0], '-');
    assert_eq!(buffer.text[0][4], '-');
    assert_eq!(buffer.text[1][0], ' ');
}
