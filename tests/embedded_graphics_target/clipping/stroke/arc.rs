use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Angle as EgAngle, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable, Primitive, WebColors},
    primitives::{Arc as EgArc, PrimitiveStyleBuilder, Rectangle as EgRectangle, StrokeAlignment},
};

use crate::embedded_graphics_target::render_to_mock;

#[test]
fn clipped_to_exact_bounds() {
    let view = Arc::new(Angle::degrees(-90), Angle::degrees(180))
        .stroked(2)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .frame_sized(20, 20)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgArc::new(
        EgPoint::new(0, 0),
        20,
        EgAngle::from_degrees(-90.0),
        EgAngle::from_degrees(270.0),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_SPRING_GREEN)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_overlaps_partially_diagonal() {
    let view = Arc::new(Angle::degrees(-90), Angle::degrees(180))
        .stroked(2)
        .foreground_color(Rgb888::CSS_TOMATO)
        .frame_sized(20, 20)
        .offset(10, 10)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 20));
    EgArc::new(
        EgPoint::new(10, 10),
        20,
        EgAngle::from_degrees(-90.0),
        EgAngle::from_degrees(270.0),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_TOMATO)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2.clipped(&clip_area))
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_rect_inside_view_bounds() {
    let view = Arc::new(Angle::degrees(-90), Angle::degrees(180))
        .stroked(2)
        .foreground_color(Rgb888::CSS_CORAL)
        .frame_sized(20, 20)
        .frame_sized(15, 15)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(15, 15));
    EgArc::new(
        EgPoint::new(-2, -2),
        20,
        EgAngle::from_degrees(-90.0),
        EgAngle::from_degrees(270.0),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_CORAL)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2.clipped(&clip_area))
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn view_outside_clip_area_not_drawn() {
    let view = Arc::new(Angle::degrees(-90), Angle::degrees(180))
        .stroked(2)
        .foreground_color(Rgb888::CSS_MEDIUM_PURPLE)
        .frame_sized(10, 10)
        .offset(0, -10)
        .clipped();

    let display = render_to_mock(&view, false);

    let display_2 = MockDisplay::<Rgb888>::new();

    display.assert_eq(&display_2);
}
