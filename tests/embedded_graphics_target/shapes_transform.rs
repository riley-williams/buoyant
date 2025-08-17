use buoyant::{
    primitives::{Interpolate, UnitPoint},
    view::prelude::*,
};
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, RgbColor, Size, WebColors},
    primitives::{
        Circle as EgCircle, CornerRadii, PrimitiveStyleBuilder, Rectangle as EgRectangle,
        RoundedRectangle as EgRoundedRectangle, StrokeAlignment,
    },
};

use super::render_to_mock;

#[test]
fn scaled_rectangle() {
    let view = Rectangle
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .opacity(192)
        .hint_background_color(Rgb888::BLACK)
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
                .fill_color(Interpolate::interpolate(
                    Rgb888::BLACK,
                    Rgb888::CSS_SPRING_GREEN,
                    192,
                ))
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
        .opacity(105)
        .hint_background_color(Rgb888::WHITE)
        .foreground_color(Rgb888::CSS_BLUE_VIOLET);

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
            .fill_color(Interpolate::interpolate(
                Rgb888::WHITE,
                Rgb888::CSS_BLUE_VIOLET,
                105,
            ))
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
        .opacity(12)
        .hint_background_color(Rgb888::GREEN)
        .foreground_color(Rgb888::RED);

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
            .fill_color(Interpolate::interpolate(Rgb888::GREEN, Rgb888::RED, 12))
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
        .opacity(250)
        .hint_background_color(Rgb888::CSS_ORANGE_RED)
        .foreground_color(Rgb888::CSS_SKY_BLUE);

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
            .fill_color(Interpolate::interpolate(
                Rgb888::CSS_ORANGE_RED,
                Rgb888::CSS_SKY_BLUE,
                250,
            ))
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn scaled_stroked_rectangle() {
    let view = Rectangle
        .stroked(2)
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .opacity(77)
        .hint_background_color(Rgb888::CSS_PINK)
        .foreground_color(Rgb888::CSS_LAWN_GREEN);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(10);
    size.width /= 2;
    size.height /= 2;

    EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .resized(size, embedded_graphics::geometry::AnchorPoint::Center)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Interpolate::interpolate(
                    Rgb888::CSS_PINK,
                    Rgb888::CSS_LAWN_GREEN,
                    77,
                ))
                .stroke_width(1)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn scaled_stroked_circle() {
    let view = Circle
        .stroked(2)
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .opacity(192)
        .hint_background_color(Rgb888::BLACK)
        .foreground_color(Rgb888::RED);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let diameter = (display.size().width - 10).min(display.size().height - 10);
    let scaled_diameter = diameter / 2;

    EgCircle::new(
        EgPoint::new(
            5 + ((diameter - scaled_diameter) / 2) as i32,
            5 + ((diameter - scaled_diameter) / 2) as i32,
        ),
        scaled_diameter,
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Interpolate::interpolate(Rgb888::BLACK, Rgb888::RED, 192))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn scaled_stroked_rounded_rectangle() {
    let view = RoundedRectangle::new(10)
        .stroked(2)
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .opacity(128)
        .hint_background_color(Rgb888::BLACK)
        .foreground_color(Rgb888::RED);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(10);
    size.width /= 2;
    size.height /= 2;
    let corner_radius = 8 / 2;

    EgRoundedRectangle::new(
        EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
            .resized(size, embedded_graphics::geometry::AnchorPoint::Center),
        CornerRadii::new(EgSize::new_equal(corner_radius)),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Interpolate::interpolate(Rgb888::BLACK, Rgb888::RED, 128))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn scaled_stroked_capsule() {
    let view = Capsule
        .stroked(2)
        .padding(Edges::All, 4)
        .scale_effect(0.5, UnitPoint::center())
        .opacity(25)
        .hint_background_color(Rgb888::BLACK)
        .foreground_color(Rgb888::RED);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let mut size = display.size() - Size::new_equal(10);
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
            .stroke_color(Interpolate::interpolate(Rgb888::BLACK, Rgb888::RED, 25))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build(),
    )
    .draw(&mut display_2)
    .unwrap();

    display.assert_eq(&display_2);
}
