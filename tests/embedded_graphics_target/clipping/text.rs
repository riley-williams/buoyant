use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    mono_font::{MonoTextStyle, ascii::FONT_7X13},
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable, WebColors},
    primitives::Rectangle as EgRectangle,
    text::Text as EgText,
};

use crate::embedded_graphics_target::render_to_mock;

#[test]
fn clipped_to_exact_bounds() {
    let view = Text::new("Test", &FONT_7X13)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_SPRING_GREEN);
    EgText::new("Test", EgPoint::new(0, 10), style)
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_overlaps_partially() {
    let view = Text::new("Test", &FONT_7X13)
        .foreground_color(Rgb888::CSS_TOMATO)
        .frame_sized(28, 13)
        .offset(-10, 0)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_TOMATO);

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(20, 13));
    EgText::new("Test", EgPoint::new(-10, 10), style)
        .draw(&mut display_2.clipped(&clip_area))
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_rect_inside_view_bounds() {
    // The text is larger than the clip frame
    let view = Text::new("Test", &FONT_7X13)
        .foreground_color(Rgb888::CSS_CORAL)
        .frame_sized(30, 15)
        .with_alignment(Alignment::TopLeading)
        .offset(1, 1) // offset to be inside
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_CORAL);

    // EgText alignment is weird...just trust this position lol
    EgText::new("Test", EgPoint::new(1, 11), style)
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn view_outside_clip_area_not_drawn() {
    let view = Text::new("Test", &FONT_7X13)
        .foreground_color(Rgb888::CSS_MEDIUM_PURPLE)
        .offset(0, -200)
        .clipped();

    let display = render_to_mock(&view, false);

    let display_2 = MockDisplay::<Rgb888>::new();

    display.assert_eq(&display_2);
}
