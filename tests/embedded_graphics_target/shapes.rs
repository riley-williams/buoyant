use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
};

use super::render_to_mock;

const MOCK_SIZE: u32 = 64;

#[test]
fn sanity_shapes() {
    // This should write to every pixel at most once
    let view = HStack::new((
        VStack::new((
            Circle,
            Rectangle.geometry_group(),
            RoundedRectangle::new(5),
            Rectangle.offset(0, 0),
        )),
        VStack::new((Rectangle, Capsule)).geometry_group(),
    ))
    .frame_sized(MOCK_SIZE - 5, MOCK_SIZE - 6)
    .flex_frame()
    .with_infinite_max_width()
    .with_infinite_max_height()
    .with_alignment(Alignment::BottomTrailing);

    let display = render_to_mock(&view, false);

    assert_eq!(
        display.affected_area(),
        EgRectangle::new(EgPoint::new(5, 6), display.size() - EgSize::new(5, 6))
    );
}

#[test]
fn rectangle() {
    let view = Rectangle
        .padding(Edges::All, 4)
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SPRING_GREEN)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}
