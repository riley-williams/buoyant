use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, RgbColor},
    primitives::{
        Circle as EgCircle, CornerRadii, PrimitiveStyleBuilder, Rectangle as EgRectangle,
        RoundedRectangle as EgRoundedRectangle,
    },
};

#[test]
fn inner_inset_rectangle() {
    let mut display = MockDisplay::new();
    Rectangle
        .stroked(4)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(6, 6), display.size() - EgSize::new(12, 12))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn center_inset_rectangle() {
    let mut display = MockDisplay::new();
    Rectangle
        .stroked_offset(4, StrokeOffset::Center)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn outer_inset_rectangle() {
    let mut display = MockDisplay::new();
    Rectangle
        .stroked_offset(4, StrokeOffset::Outer)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(2, 2), display.size() - EgSize::new(4, 4))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn inner_inset_rounded_rectangle() {
    let mut display = MockDisplay::new();
    RoundedRectangle::new(5)
        .stroked(4)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(6, 6), display.size() - EgSize::new(12, 12)),
        CornerRadii::new(EgSize::new(3, 3)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn center_inset_rounded_rectangle() {
    let mut display = MockDisplay::new();
    RoundedRectangle::new(5)
        .stroked_offset(4, StrokeOffset::Center)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8)),
        CornerRadii::new(EgSize::new(5, 5)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn outer_inset_rounded_rectangle() {
    let mut display = MockDisplay::new();
    RoundedRectangle::new(5)
        .stroked_offset(4, StrokeOffset::Outer)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(2, 2), display.size() - EgSize::new(4, 4)),
        CornerRadii::new(EgSize::new(7, 7)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn inner_inset_circle() {
    let mut display = MockDisplay::new();
    Circle
        .stroked(4)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size();
    let min_dimension = size.width.min(size.height) - 12; // Account for padding and stroke
    EgCircle::new(EgPoint::new(6, 6), min_dimension)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn center_inset_circle() {
    let mut display = MockDisplay::new();
    Circle
        .stroked_offset(4, StrokeOffset::Center)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size();
    let min_dimension = size.width.min(size.height) - 8; // Account for padding only
    EgCircle::new(EgPoint::new(4, 4), min_dimension)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn outer_inset_circle() {
    let mut display = MockDisplay::new();
    Circle
        .stroked_offset(4, StrokeOffset::Outer)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size();
    let min_dimension = size.width.min(size.height) - 4; // Account for padding, stroke extends outward
    EgCircle::new(EgPoint::new(2, 2), min_dimension)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::WHITE)
                .stroke_width(4)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn inner_inset_capsule() {
    let mut display = MockDisplay::new();
    Capsule
        .stroked(4)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size() - EgSize::new(12, 12); // Account for padding and inner stroke
    let min_dimension = size.width.min(size.height);
    let radius = min_dimension / 2;
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(6, 6), size),
        CornerRadii::new(EgSize::new(radius, radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn center_inset_capsule() {
    let mut display = MockDisplay::new();
    Capsule
        .stroked_offset(4, StrokeOffset::Center)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size() - EgSize::new(8, 8); // Account for padding only
    let min_dimension = size.width.min(size.height);
    let radius = min_dimension / 2;
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(4, 4), size),
        CornerRadii::new(EgSize::new(radius, radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn outer_inset_capsule() {
    let mut display = MockDisplay::new();
    Capsule
        .stroked_offset(4, StrokeOffset::Outer)
        .padding(Edges::All, 4)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

    let mut display_2 = MockDisplay::new();

    let size = display.size() - EgSize::new(4, 4); // Account for padding, stroke extends outward
    let min_dimension = size.width.min(size.height);
    let radius = min_dimension / 2;
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(2, 2), size),
        CornerRadii::new(EgSize::new(radius, radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::WHITE)
            .stroke_width(4)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();
    display.assert_eq(&display_2);
}
