mod fill;
mod image;
mod stroke;
mod text;

use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
};

use super::render_to_mock;

/// Test that nested clipping modifiers result in the intersection of both clip rects.
/// The inner clip rect should be constrained by the outer clip rect.
#[test]
fn nested_clipped_uses_intersection() {
    let view = Rectangle
        .foreground_color(Rgb888::CSS_MEDIUM_PURPLE)
        .frame_sized(20, 20)
        .offset(10, 10)
        .clipped()
        .frame_sized(20, 20)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(10, 10), EgSize::new(10, 10))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_MEDIUM_PURPLE)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}
