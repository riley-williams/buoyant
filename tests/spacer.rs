use buoyant::font::CharMonospace;
use buoyant::layout::{Environment, Layout, LayoutDirection};
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{FixedTextBuffer, RenderTarget};
use buoyant::view::{HStack, Spacer, Text};

struct TestEnv {
    direction: LayoutDirection,
}

impl Environment for TestEnv {
    fn layout_direction(&self) -> LayoutDirection {
        self.direction
    }
}

#[test]
fn test_horizontal_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv {
        direction: LayoutDirection::Horizontal,
    };
    let layout = spacer.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 0));
}

#[test]
fn test_vertical_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv {
        direction: LayoutDirection::Vertical,
    };
    let layout = spacer.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(0, 10));
}

#[test]
fn test_render_fills_stack() {
    let font = CharMonospace {};
    let hstack = HStack::two(Spacer::default(), Text::char("67", &font)).spacing(1);
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let env = TestEnv {
        direction: LayoutDirection::Horizontal,
    };
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "       67");
}
