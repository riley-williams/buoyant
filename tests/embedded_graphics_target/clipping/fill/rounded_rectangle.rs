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
    let view = RoundedRectangle::new(5)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .frame_sized(20, 20)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 20)),
        CornerRadii::new(EgSize::new(5, 5)),
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
    let view = RoundedRectangle::new(5)
        .foreground_color(Rgb888::CSS_TOMATO)
        .frame_sized(20, 20)
        .offset(10, 10)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 20));
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(10, 10), EgSize::new(20, 20)),
        CornerRadii::new(EgSize::new(5, 5)),
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
    let view = RoundedRectangle::new(5)
        .foreground_color(Rgb888::CSS_CORAL)
        .frame_sized(20, 20)
        .frame_sized(15, 15)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(15, 15));
    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(-2, -2), EgSize::new(20, 20)),
        CornerRadii::new(EgSize::new(5, 5)),
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
    let view = RoundedRectangle::new(5)
        .foreground_color(Rgb888::CSS_MEDIUM_PURPLE)
        .frame_sized(10, 10)
        .offset(0, -10)
        .clipped();

    let display = render_to_mock(&view, false);

    let display_2 = MockDisplay::<Rgb888>::new();

    display.assert_eq(&display_2);
}
