use buoyant::{primitives::UnitPoint, view::prelude::*};
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, Size, WebColors},
    primitives::{
        Circle as EgCircle, CornerRadii, PrimitiveStyleBuilder, Rectangle as EgRectangle,
        RoundedRectangle as EgRoundedRectangle,
    },
};

use super::render_to_mock;

#[test]
fn scaled_rectangle() {
    let view = Rectangle
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(8);
    size.width /= 2;
    size.height /= 2;

    EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .resized(size, embedded_graphics::geometry::AnchorPoint::Center)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SPRING_GREEN)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn scaled_circle() {
    let view = Circle
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let diameter = (display.size().width - 8).min(display.size().height - 8);
    let scaled_diameter = diameter / 2;

    EgCircle::new(
        EgPoint::new(
            4 + ((diameter - scaled_diameter) / 2) as i32,
            4 + ((diameter - scaled_diameter) / 2) as i32,
        ),
        scaled_diameter,
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CSS_SPRING_GREEN)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn scaled_rounded_rectangle() {
    let view = RoundedRectangle::new(10)
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(8);
    size.width /= 2;
    size.height /= 2;
    let corner_radius = 10 / 2;

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
            .resized(size, embedded_graphics::geometry::AnchorPoint::Center),
        CornerRadii::new(EgSize::new_equal(corner_radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CSS_SPRING_GREEN)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn scaled_capsule() {
    let view = Capsule
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(8);
    size.width /= 2;
    size.height /= 2;

    // Capsule renders as a rounded rectangle with radius = min(width, height) / 2
    let radius = size.width.min(size.height) / 2;

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
            .resized(size, embedded_graphics::geometry::AnchorPoint::Center),
        CornerRadii::new(EgSize::new_equal(radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CSS_SPRING_GREEN)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}
