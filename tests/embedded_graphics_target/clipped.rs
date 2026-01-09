use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
};

use super::render_to_mock;

/// Test that clipping with no offset doesn't change the output.
/// This is a baseline test to ensure clipped views render correctly when
/// the content fits within the clip bounds.
#[test]
fn clipped_no_offset_unchanged() {
    let view_clipped = Rectangle
        .foreground_color(Rgb888::CSS_GOLD)
        .frame_sized(24, 24)
        .clipped();

    let view_unclipped = Rectangle
        .foreground_color(Rgb888::CSS_GOLD)
        .frame_sized(24, 24);

    let display_clipped = render_to_mock(&view_clipped, false);
    let display_unclipped = render_to_mock(&view_unclipped, false);

    display_clipped.assert_eq(&display_unclipped);
}

/// Test that a clipped view correctly renders its content.
/// The clipped modifier should not affect rendering when content is within bounds.
#[test]
fn clipped_rectangle_renders_correctly() {
    let view = Rectangle
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .frame_sized(20, 20)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 20))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SPRING_GREEN)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

/// Test that clipping works correctly with nested frames.
/// A smaller outer frame should result in a smaller clip region.
#[test]
fn clipped_with_smaller_outer_frame() {
    // Inner content is 30x30, but outer frame constrains to 15x15
    // The clipped modifier clips to the child's resolved size
    let view = Rectangle
        .foreground_color(Rgb888::CSS_CORAL)
        .frame_sized(15, 15)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(0, 0), EgSize::new(15, 15))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_CORAL)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

/// Test that clipping a padded rectangle works correctly.
#[test]
fn clipped_padded_rectangle() {
    let view = Rectangle
        .foreground_color(Rgb888::CSS_SKY_BLUE)
        .padding(Edges::All, 4)
        .frame_sized(24, 24)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    // Rectangle is 24x24 total, with 4px padding on all sides
    // So the filled area is 16x16, starting at (4, 4)
    EgRectangle::new(EgPoint::new(4, 4), EgSize::new(16, 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SKY_BLUE)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

/// Test that multiple clipped views in a stack work correctly.
#[test]
fn multiple_clipped_views_in_stack() {
    let view = HStack::new((
        Rectangle
            .foreground_color(Rgb888::CSS_TOMATO)
            .frame_sized(10, 20)
            .clipped(),
        Rectangle
            .foreground_color(Rgb888::CSS_LIME_GREEN)
            .frame_sized(10, 20)
            .clipped(),
    ));

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    // First rectangle at (0, 0)
    EgRectangle::new(EgPoint::new(0, 0), EgSize::new(10, 20))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_TOMATO)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    // Second rectangle at (10, 0)
    EgRectangle::new(EgPoint::new(10, 0), EgSize::new(10, 20))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_LIME_GREEN)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}
