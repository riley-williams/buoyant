use buoyant::font::CharacterBufferFont;
use buoyant::layout::{Layout, LayoutDirection};
use buoyant::primitives::{Dimension, Dimensions, ProposedDimension, ProposedDimensions, Size};
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget;
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{make_render_tree, HStack, RenderExtensions as _, Spacer, Text, VStack};
use common::{collect_text, TestEnv};

mod common;

#[test]
fn test_horizontal_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = spacer.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 0));
}

#[test]
fn test_vertical_layout() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 10);
    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = spacer.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 10));
}

#[test]
fn test_horizontal_layout_zero() {
    let spacer = Spacer::default();
    let offer = Size::new(0, 10);
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = spacer.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 0));
}

#[test]
fn test_vertical_layout_zero() {
    let spacer = Spacer::default();
    let offer = Size::new(10, 0);
    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = spacer.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 0));
}

#[test]
fn test_horizontal_layout_infinite_width() {
    let spacer = Spacer::default();
    let offer = ProposedDimensions {
        width: ProposedDimension::Infinite,
        height: ProposedDimension::Exact(10),
    };
    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = spacer.layout(&offer, &env);
    assert_eq!(
        layout.resolved_size,
        Dimensions {
            width: Dimension::infinite(),
            height: 0.into()
        }
    );
}

#[test]
fn test_horizontal_layout_compact_width() {
    let spacer = Spacer::default();
    let offer = ProposedDimensions {
        width: ProposedDimension::Compact,
        height: ProposedDimension::Exact(10),
    };

    let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
    let layout = spacer.layout(&offer, &env);
    assert_eq!(
        layout.resolved_size,
        Dimensions {
            width: 0.into(),
            height: 0.into()
        }
    );
}

#[test]
fn test_vertical_layout_infinite_height() {
    let spacer = Spacer::default();
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Infinite,
    };

    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = spacer.layout(&offer, &env);
    assert_eq!(
        layout.resolved_size,
        Dimensions {
            width: 0.into(),
            height: Dimension::infinite()
        }
    );
}

#[test]
fn test_vertical_layout_compact_height() {
    let spacer = Spacer::default();
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Compact,
    };

    let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
    let layout = spacer.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 0));
}

#[test]
fn test_render_fills_hstack() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((Spacer::default(), Text::str("67", &font)))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "       67");
}

#[test]
fn test_render_fills_vstack() {
    let font = CharacterBufferFont {};
    let vstack = VStack::new((Spacer::default(), Text::str("67", &font)))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<1, 9>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(collect_text(&buffer), "       67");
}
