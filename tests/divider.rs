use buoyant::environment::LayoutEnvironment;
use buoyant::layout::{Alignment, Layout, LayoutDirection};
use buoyant::primitives::{Point, Size};
use buoyant::render::{CharacterRender, CharacterRenderTarget as _, Renderable as _};
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{Divider, RenderExtensions as _};

pub struct TestEnv {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
}

impl LayoutEnvironment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            alignment: Alignment::default(),
        }
    }
}

impl TestEnv {
    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

#[test]
fn test_horizontal_layout() {
    let divider = Divider::new(2);
    let offer = Size::new(100, 100).into();
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = divider.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Size::new(2, 100).into());
}

#[test]
fn test_vertical_layout() {
    let divider = Divider::new(2);
    let offer = Size::new(100, 100).into();
    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = divider.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Size::new(100, 2).into());
}

#[test]
fn test_horizontal_render() {
    let divider = Divider::new(1).foreground_color('|');
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = divider.layout(&buffer.size().into(), &env);
    let tree = divider.render_tree(&layout, Point::new(0, 0), &env);
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
    let layout = divider.layout(&buffer.size().into(), &env);
    let tree = divider.render_tree(&layout, Point::new(0, 0), &env);
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0][0], '-');
    assert_eq!(buffer.text[0][4], '-');
    assert_eq!(buffer.text[1][0], ' ');
}
