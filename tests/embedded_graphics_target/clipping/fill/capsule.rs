use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable, Primitive, WebColors},
    primitives::{
        CornerRadii, PrimitiveStyleBuilder, Rectangle as EgRectangle,
        RoundedRectangle as EgRoundedRectangle,
    },
};

use crate::embedded_graphics_target::render_to_mock;

#[test]
fn clipped_to_exact_bounds() {
    let view = Capsule
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .frame_sized(20, 30)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let radius = 20 / 2;
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 30)),
        CornerRadii::new(EgSize::new(radius, radius)),
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
fn clip_overlaps_partially_diagonal() {
    let view = Capsule
        .foreground_color(Rgb888::CSS_TOMATO)
        .frame_sized(20, 30)
        .offset(10, 10)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let radius = 20 / 2;
    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 30));
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(10, 10), EgSize::new(20, 30)),
        CornerRadii::new(EgSize::new(radius, radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CSS_TOMATO)
            .build(),
    )
    .draw(&mut display_2.clipped(&clip_area))
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_rect_inside_view_bounds() {
    let view = Capsule
        .foreground_color(Rgb888::CSS_CORAL)
        .frame_sized(20, 30)
        .frame_sized(15, 25)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let radius = 20 / 2;
    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(15, 25));
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(-2, -2), EgSize::new(20, 30)),
        CornerRadii::new(EgSize::new(radius, radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CSS_CORAL)
            .build(),
    )
    .draw(&mut display_2.clipped(&clip_area))
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn view_outside_clip_area_not_drawn() {
    let view = Capsule
        .foreground_color(Rgb888::CSS_MEDIUM_PURPLE)
        .frame_sized(10, 15)
        .offset(0, -15)
        .clipped();

    let display = render_to_mock(&view, false);

    let display_2 = MockDisplay::<Rgb888>::new();

    display.assert_eq(&display_2);
}
