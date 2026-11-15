use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Angle as EgAngle, OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, WebColors},
    primitives::{Arc as EgArc, PrimitiveStyleBuilder, Rectangle as EgRectangle, StrokeAlignment},
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

#[test]
fn unstroked_arc_is_not_drawn() {
    let view =
        Arc::new(Angle::degrees(0), Angle::degrees(270)).foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    display.assert_eq(&MockDisplay::new());
}

#[test]
fn stroked_arc() {
    let view = Arc::new(Angle::degrees(-90), Angle::degrees(180))
        .stroked(3)
        .padding(Edges::All, 4)
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgArc::new(
        EgPoint::new(4, 4),
        display.size().width - 8,
        EgAngle::from_degrees(-90.0),
        EgAngle::from_degrees(270.0),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_SPRING_GREEN)
            .stroke_width(3)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn counterclockwise_arc() {
    let view = Arc::new(Angle::degrees(90), Angle::degrees(-45))
        .stroked(2)
        .foreground_color(Rgb888::CSS_TOMATO);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgArc::new(
        EgPoint::new(0, 0),
        display.size().width,
        EgAngle::from_degrees(90.0),
        EgAngle::from_degrees(-135.0),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_TOMATO)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}
